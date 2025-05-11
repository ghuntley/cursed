//! # Asynchronous Constraint Checker with Work Stealing
//!
//! This module extends the base AsyncConstraintChecker with a work stealing algorithm
//! to improve task distribution and throughput for constraint checking operations.
//!
//! Work stealing allows idle worker threads to "steal" pending tasks from busy workers,
//! which leads to better load balancing and overall system throughput.
//!
//! ## Features
//!
//! - Adaptive work distribution using a work stealing deque
//! - Reduced contention on the central task queue
//! - Improved worker utilization
//! - Better performance for unbalanced workloads
//! - Fine-grained performance metrics

use crate::core::async_constraint_checker::{AsyncConstraintChecker, AsyncConstraintCheckerStats};
use crate::core::interface_registry::InterfaceRegistry;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self, JoinHandle};
use tracing::{debug, info, trace, instrument, warn};

/// A task for the async constraint checker
#[derive(Debug, Clone)]
struct ConstraintCheckTask {
    /// The type to check
    type_: Type,
    /// The interface name to check against
    interface_name: String,
    /// Task identifier for tracking
    task_id: usize,
}

/// The result of a constraint check task
#[derive(Debug)]
struct ConstraintCheckResult {
    /// The task ID that was checked
    task_id: usize,
    /// The type that was checked
    type_: Type,
    /// The interface name that was checked
    interface_name: String,
    /// The result of the check
    result: Result<bool, Error>,
    /// Worker ID that processed this task
    worker_id: usize,
    /// Time taken to process this task in milliseconds
    processing_time_ms: f32,
}

/// WorkStealingQueue structure for task distribution
struct WorkStealingQueue<T> {
    /// Per-worker task queues
    worker_queues: Vec<Mutex<VecDeque<T>>>,
    /// Condition variable for worker notification
    condvar: Condvar,
    /// Flag to indicate when all work is done
    done: Mutex<bool>,
    /// Steal count statistics
    steal_count: Mutex<Vec<usize>>,
}

impl<T: Clone + Send + 'static> WorkStealingQueue<T> {
    /// Create a new work stealing queue with a given number of workers
    fn new(num_workers: usize) -> Self {
        let mut worker_queues = Vec::with_capacity(num_workers);
        let mut steal_count = Vec::with_capacity(num_workers);
        
        for _ in 0..num_workers {
            worker_queues.push(Mutex::new(VecDeque::new()));
            steal_count.push(0);
        }
        
        Self {
            worker_queues,
            condvar: Condvar::new(),
            done: Mutex::new(false),
            steal_count: Mutex::new(steal_count),
        }
    }
    
    /// Add a task to a specific worker's queue
    fn push(&self, worker_id: usize, task: T) {
        let mut queue = self.worker_queues[worker_id].lock().unwrap();
        queue.push_back(task);
        self.condvar.notify_all();
    }
    
    /// Distribute tasks among all workers
    fn distribute_tasks(&self, tasks: Vec<T>) {
        let num_workers = self.worker_queues.len();
        let tasks_per_worker = (tasks.len() + num_workers - 1) / num_workers; // Ceiling division
        
        // Reset the done flag
        let mut done = self.done.lock().unwrap();
        *done = false;
        drop(done);
        
        // Distribute tasks in a round-robin fashion with larger chunks
        for (i, chunk) in tasks.chunks(tasks_per_worker).enumerate() {
            let worker_id = i % num_workers;
            let mut queue = self.worker_queues[worker_id].lock().unwrap();
            for task in chunk {
                queue.push_back(task.clone());
            }
        }
        
        self.condvar.notify_all();
    }
    
    /// Pop a task from the current worker's queue
    fn pop(&self, worker_id: usize) -> Option<T> {
        let mut queue = self.worker_queues[worker_id].lock().unwrap();
        queue.pop_front()
    }
    
    /// Try to steal a task from another worker
    fn steal(&self, current_worker: usize) -> Option<T> {
        let num_workers = self.worker_queues.len();
        
        // Try to steal from other workers in sequence
        for i in 0..num_workers {
            let victim = (current_worker + i + 1) % num_workers;
            if victim != current_worker {
                let mut victim_queue = self.worker_queues[victim].lock().unwrap();
                if let Some(task) = victim_queue.pop_back() {
                    // Record this steal
                    let mut steal_count = self.steal_count.lock().unwrap();
                    steal_count[current_worker] += 1;
                    
                    return Some(task);
                }
            }
        }
        
        None
    }
    
    /// Wait for a task, either from own queue or by stealing
    fn wait_for_task(&self, worker_id: usize) -> Option<T> {
        // First try the worker's own queue
        if let Some(task) = self.pop(worker_id) {
            return Some(task);
        }
        
        // Try to steal from other workers
        if let Some(task) = self.steal(worker_id) {
            return Some(task);
        }
        
        // If no tasks are available, wait on the condition variable
        let mut done = self.done.lock().unwrap();
        if *done {
            return None;
        }
        
        // Wait for notification or timeout
        let wait_result = self.condvar.wait_timeout(done, std::time::Duration::from_millis(50)).unwrap();
        done = wait_result.0;
        
        // After waking up, check if we're done or try again
        if *done {
            None
        } else {
            // Try one more time
            self.pop(worker_id).or_else(|| self.steal(worker_id))
        }
    }
    
    /// Signal that all work is done
    fn signal_done(&self) {
        let mut done = self.done.lock().unwrap();
        *done = true;
        self.condvar.notify_all();
    }
    
    /// Get the number of tasks in all queues
    fn total_remaining_tasks(&self) -> usize {
        let mut total = 0;
        for queue in &self.worker_queues {
            total += queue.lock().unwrap().len();
        }
        total
    }
    
    /// Get steal count statistics
    fn get_steal_count(&self) -> Vec<usize> {
        self.steal_count.lock().unwrap().clone()
    }
}

/// Extended statistics for the work stealing async constraint checker
#[derive(Debug, Default, Clone)]
pub struct WorkStealingStats {
    /// Base statistics from AsyncConstraintCheckerStats
    pub base: AsyncConstraintCheckerStats,
    /// Number of tasks stolen between workers
    pub total_steals: usize,
    /// Maximum number of steals by any single worker
    pub max_steals: usize,
    /// Average number of steals per worker
    pub avg_steals_per_worker: f32,
    /// Imbalance factor (0.0 = perfectly balanced, 1.0 = completely imbalanced)
    pub workload_imbalance: f32,
    /// Number of tasks completed by each worker
    pub tasks_per_worker: Vec<usize>,
    /// Cache hit rate for constraint checks (0.0-1.0)
    pub cache_hit_rate: f32,
}

/// Asynchronous constraint checker with work stealing
#[derive(Debug)]
pub struct WorkStealingConstraintChecker {
    /// The base AsyncConstraintChecker
    base_checker: AsyncConstraintChecker,
    /// The interface registry to use for constraint checking
    registry: Arc<InterfaceRegistry>,
    /// Additional statistics specific to work stealing
    work_stealing_stats: Arc<Mutex<WorkStealingStats>>,
    /// Enable caching of constraint check results
    enable_caching: bool,
}

impl WorkStealingConstraintChecker {
    /// Create a new work stealing constraint checker with the given registry
    pub fn new(registry: Arc<InterfaceRegistry>) -> Self {
        let base_checker = AsyncConstraintChecker::new(Arc::clone(&registry));
        
        Self {
            base_checker,
            registry,
            work_stealing_stats: Arc::new(Mutex::new(WorkStealingStats::default())),
            enable_caching: true,
        }
    }
    
    /// Create a new work stealing constraint checker with custom worker configuration
    pub fn with_worker_config(
        registry: Arc<InterfaceRegistry>,
        min_workers: usize,
        max_workers: usize,
        scaling_factor: f32
    ) -> Self {
        let base_checker = AsyncConstraintChecker::with_worker_config(
            Arc::clone(&registry),
            min_workers,
            max_workers,
            scaling_factor
        );
        
        Self {
            base_checker,
            registry,
            work_stealing_stats: Arc::new(Mutex::new(WorkStealingStats::default())),
            enable_caching: true,
        }
    }
    
    /// Enable or disable caching of constraint check results
    pub fn with_caching(mut self, enable: bool) -> Self {
        self.enable_caching = enable;
        self
    }
    
    /// Check a set of constraints in parallel using work stealing algorithm
    ///
    /// # Arguments
    ///
    /// * `constraints` - A vector of (type, interface_name) pairs to check
    ///
    /// # Returns
    ///
    /// A vector of results corresponding to each constraint
    #[instrument(skip(self, constraints), level = "debug")]
    pub fn check_constraints_parallel(
        &self,
        constraints: Vec<(Type, String)>,
    ) -> Vec<Result<bool, Error>> {
        let num_constraints = constraints.len();
        
        // If there are too few constraints, just use the base implementation
        if num_constraints <= 1 {
            return self.base_checker.check_constraints_parallel(constraints);
        }
        
        // Calculate optimal number of workers
        let num_workers = self.calculate_optimal_workers(num_constraints);
        
        info!("Checking {} constraints with {} worker threads using work stealing algorithm", 
              num_constraints, num_workers);
        
        // Convert constraints into tasks
        let mut tasks = Vec::with_capacity(num_constraints);
        for (i, (type_, interface_name)) in constraints.iter().enumerate() {
            tasks.push(ConstraintCheckTask {
                type_: type_.clone(),
                interface_name: interface_name.clone(),
                task_id: i,
            });
        }
        
        // Create work stealing queue
        let queue = Arc::new(WorkStealingQueue::new(num_workers));
        
        // Create cache for results if enabled
        let result_cache = if self.enable_caching {
            Some(Arc::new(Mutex::new(std::collections::HashMap::<(Type, String), Result<bool, Error>>::new())))
        } else {
            None
        };
        
        // Create a shared results collection
        let results = Arc::new(Mutex::new(Vec::<ConstraintCheckResult>::with_capacity(num_constraints)));
        
        // Distribute tasks
        queue.distribute_tasks(tasks);
        
        // Track statistics for each worker
        let worker_stats = Arc::new(Mutex::new(vec![0usize; num_workers])); // Tasks completed by each worker
        let cache_stats = Arc::new(Mutex::new((0usize, 0usize))); // (hits, lookups)
        
        // Spawn worker threads
        let mut handles = Vec::with_capacity(num_workers);
        for worker_id in 0..num_workers {
            let queue = Arc::clone(&queue);
            let results = Arc::clone(&results);
            let registry = Arc::clone(&self.registry);
            let worker_stats = Arc::clone(&worker_stats);
            let cache = result_cache.clone();
            let cache_stats = Arc::clone(&cache_stats);
            
            let handle = thread::spawn(move || {
                trace!("Worker {} started", worker_id);
                let mut tasks_processed = 0;
                
                while let Some(task) = queue.wait_for_task(worker_id) {
                    let type_clone = task.type_.clone();
                    let interface_name_clone = task.interface_name.clone();
                    let task_id = task.task_id;
                    
                    trace!("Worker {} processing task {}: {:?} implements {}", 
                          worker_id, task_id, type_clone, interface_name_clone);
                    
                    let start_time = std::time::Instant::now();
                    
                    // Check if result is already in cache
                    let mut cache_hit = false;
                    let result = if let Some(cache) = &cache {
                        let key = (type_clone.clone(), interface_name_clone.clone());
                        let mut cache = cache.lock().unwrap();
                        
                        // Update cache stats
                        let mut stats = cache_stats.lock().unwrap();
                        stats.1 += 1; // Increment lookups
                        
                        if let Some(cached_result) = cache.get(&key) {
                            // Cache hit
                            stats.0 += 1; // Increment hits
                            cache_hit = true;
                            cached_result.clone()
                        } else {
                            // Cache miss - do the check and cache the result
                            let result = registry.check_implementation(&task.type_, &task.interface_name);
                            cache.insert(key, result.clone());
                            result
                        }
                    } else {
                        // No caching - always do the check
                        registry.check_implementation(&task.type_, &task.interface_name)
                    };
                    
                    let elapsed = start_time.elapsed();
                    let elapsed_ms = elapsed.as_secs() as f32 * 1000.0 + elapsed.subsec_millis() as f32;
                    
                    // Store the result
                    let check_result = ConstraintCheckResult {
                        task_id,
                        type_: task.type_,
                        interface_name: task.interface_name,
                        result,
                        worker_id,
                        processing_time_ms: elapsed_ms,
                    };
                    
                    let mut results = results.lock().unwrap();
                    results.push(check_result);
                    
                    // Update worker statistics
                    let mut stats = worker_stats.lock().unwrap();
                    stats[worker_id] += 1;
                    
                    tasks_processed += 1;
                    
                    if cache_hit {
                        trace!("Worker {} completed task {} in {:.2}ms (cache hit): {:?} implements {}", 
                              worker_id, task_id, elapsed_ms, type_clone, interface_name_clone);
                    } else {
                        trace!("Worker {} completed task {} in {:.2}ms: {:?} implements {}", 
                              worker_id, task_id, elapsed_ms, type_clone, interface_name_clone);
                    }
                }
                
                trace!("Worker {} finished, processed {} tasks", worker_id, tasks_processed);
            });
            
            handles.push(handle);
        }
        
        // Track the start time for performance measurement
        let start_time = std::time::Instant::now();
        
        // Monitor remaining tasks and signal when all are done
        loop {
            let remaining = queue.total_remaining_tasks();
            let results_count = results.lock().unwrap().len();
            
            if results_count >= num_constraints || (remaining == 0 && results_count > 0) {
                // All tasks are done
                queue.signal_done();
                break;
            }
            
            // Short sleep to avoid busy waiting
            thread::sleep(std::time::Duration::from_millis(10));
        }
        
        // Wait for all workers to finish
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Calculate the elapsed time
        let elapsed = start_time.elapsed();
        let elapsed_ms = elapsed.as_secs() as f32 * 1000.0 + elapsed.subsec_millis() as f32;
        let avg_task_time_ms = if num_constraints > 0 {
            elapsed_ms / num_constraints as f32
        } else {
            0.0
        };
        
        // Collect steal statistics
        let steal_counts = queue.get_steal_count();
        let total_steals: usize = steal_counts.iter().sum();
        let max_steals = steal_counts.iter().max().cloned().unwrap_or(0);
        let avg_steals = if num_workers > 0 {
            total_steals as f32 / num_workers as f32
        } else {
            0.0
        };
        
        // Get worker task distribution
        let tasks_per_worker = worker_stats.lock().unwrap().clone();
        
        // Calculate workload imbalance (0.0 = perfectly balanced, 1.0 = completely imbalanced)
        let workload_imbalance = if num_constraints > 0 && num_workers > 0 {
            let avg_tasks_per_worker = num_constraints as f32 / num_workers as f32;
            let max_deviation = tasks_per_worker.iter()
                .map(|&tasks| (tasks as f32 - avg_tasks_per_worker).abs())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0);
            
            max_deviation / avg_tasks_per_worker
        } else {
            0.0
        };
        
        // Calculate cache hit rate if caching is enabled
        let cache_hit_rate = if self.enable_caching {
            let stats = cache_stats.lock().unwrap();
            let (hits, lookups) = *stats;
            if lookups > 0 {
                hits as f32 / lookups as f32
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Update work stealing statistics
        let base_stats = self.base_checker.get_detailed_stats();
        let mut ws_stats = self.work_stealing_stats.lock().unwrap();
        ws_stats.base = base_stats;
        ws_stats.total_steals = total_steals;
        ws_stats.max_steals = max_steals;
        ws_stats.avg_steals_per_worker = avg_steals;
        ws_stats.workload_imbalance = workload_imbalance;
        ws_stats.tasks_per_worker = tasks_per_worker;
        ws_stats.cache_hit_rate = cache_hit_rate;
        
        debug!("Completed {} constraint checks in {:.2}ms with {} steals (imbalance: {:.2}, cache hit rate: {:.2})", 
               num_constraints, elapsed_ms, total_steals, workload_imbalance, cache_hit_rate);
        
        // Collect and return results in the order of the original constraints
        let mut result_map = std::collections::HashMap::new();
        let result_list = results.lock().unwrap();
        
        for result in result_list.iter() {
            result_map.insert(result.task_id, result.result.clone());
        }
        
        // Return results in the original order
        let mut ordered_results = Vec::with_capacity(num_constraints);
        for i in 0..num_constraints {
            if let Some(result) = result_map.get(&i) {
                ordered_results.push(result.clone());
            } else {
                // This should never happen if all tasks were processed
                warn!("Missing result for task {}", i);
                ordered_results.push(Err(Error::new("CNST01", "Missing constraint check result", None)));
            }
        }
        
        ordered_results
    }
    
    /// Calculate the optimal number of worker threads to use
    fn calculate_optimal_workers(&self, num_constraints: usize) -> usize {
        self.base_checker.calculate_optimal_workers(num_constraints)
    }
    
    /// Check a single set of generic constraints in parallel
    ///
    /// This is especially useful for checking all the constraints of a generic type
    /// at once.
    ///
    /// # Arguments
    ///
    /// * `type_args` - The concrete type arguments
    /// * `type_params` - The type parameters of the generic type
    /// * `constraints` - The constraints on the type parameters
    ///
    /// # Returns
    ///
    /// `Ok(true)` if all constraints are satisfied, `Ok(false)` otherwise,
    /// or `Err` if there was an error during the check.
    #[instrument(skip(self, type_args, type_params, constraints), level = "debug")]
    pub fn check_generic_constraints_parallel(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)],
    ) -> Result<bool, Error> {
        // Check if we have the right number of type arguments
        if type_args.len() != type_params.len() {
            debug!(
                "Wrong number of type arguments: expected {}, got {}",
                type_params.len(),
                type_args.len()
            );
            return Ok(false);
        }
        
        // Create a mapping from type parameter names to concrete types
        let mut type_map = std::collections::HashMap::new();
        for (i, param_name) in type_params.iter().enumerate() {
            type_map.insert(param_name.clone(), type_args[i].clone());
        }
        
        // If there are no constraints, return true
        if constraints.is_empty() {
            return Ok(true);
        }
        
        // Convert constraints into (type, interface) pairs for parallel checking
        let mut constraint_pairs = Vec::with_capacity(constraints.len());
        for (param_name, interface_name) in constraints {
            if let Some(concrete_type) = type_map.get(param_name) {
                constraint_pairs.push((concrete_type.clone(), interface_name.clone()));
            } else {
                // This should never happen if the type parameter lists match
                debug!("Type parameter {} not found in mapping", param_name);
                return Ok(false);
            }
        }
        
        // Check all constraints in parallel
        let results = self.check_constraints_parallel(constraint_pairs);
        
        // All constraints must be satisfied
        for result in results {
            match result {
                Ok(satisfied) => {
                    if !satisfied {
                        debug!("One of the constraints is not satisfied");
                        return Ok(false);
                    }
                }
                Err(err) => {
                    // Propagate any errors
                    return Err(err);
                }
            }
        }
        
        // All constraints satisfied
        debug!("All generic constraints satisfied");
        Ok(true)
    }
    
    /// Get detailed statistics for the work stealing constraint checker
    pub fn get_stats(&self) -> WorkStealingStats {
        self.work_stealing_stats.lock().unwrap().clone()
    }
}

/// Extension trait for the InterfaceRegistry to add work stealing constraint checking methods
pub trait WorkStealingConstraintChecking {
    /// Check a set of constraints in parallel using work stealing
    fn check_constraints_work_stealing(
        &self,
        constraints: Vec<(Type, String)>,
    ) -> Vec<Result<bool, Error>>;
    
    /// Check a single set of generic constraints in parallel using work stealing
    fn check_generic_constraints_work_stealing(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)],
    ) -> Result<bool, Error>;
    
    /// Create a new work stealing constraint checker
    fn create_work_stealing_checker(&self) -> Arc<WorkStealingConstraintChecker>;
    
    /// Create a new work stealing constraint checker with custom worker configuration
    fn create_work_stealing_checker_with_config(
        &self,
        min_workers: usize,
        max_workers: usize,
        scaling_factor: f32,
    ) -> Arc<WorkStealingConstraintChecker>;
}

impl WorkStealingConstraintChecking for InterfaceRegistry {
    fn check_constraints_work_stealing(
        &self,
        constraints: Vec<(Type, String)>,
    ) -> Vec<Result<bool, Error>> {
        let checker = WorkStealingConstraintChecker::new(Arc::new(self.clone()));
        checker.check_constraints_parallel(constraints)
    }
    
    fn check_generic_constraints_work_stealing(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)],
    ) -> Result<bool, Error> {
        let checker = WorkStealingConstraintChecker::new(Arc::new(self.clone()));
        checker.check_generic_constraints_parallel(type_args, type_params, constraints)
    }
    
    fn create_work_stealing_checker(&self) -> Arc<WorkStealingConstraintChecker> {
        Arc::new(WorkStealingConstraintChecker::new(Arc::new(self.clone())))
    }
    
    fn create_work_stealing_checker_with_config(
        &self,
        min_workers: usize,
        max_workers: usize,
        scaling_factor: f32,
    ) -> Arc<WorkStealingConstraintChecker> {
        Arc::new(WorkStealingConstraintChecker::with_worker_config(
            Arc::new(self.clone()),
            min_workers,
            max_workers,
            scaling_factor
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::type_checker::Type;
    
    use crate::tests::common;
    
    #[test]
    fn test_work_stealing_basic() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let checker = WorkStealingConstraintChecker::new(registry_arc);
        
        // Check a single constraint
        let constraints = vec![(Type::Normie, "Numeric".to_string())];
        let results = checker.check_constraints_parallel(constraints);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], Ok(true));
    }
    
    #[test]
    fn test_work_stealing_vs_regular() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let regular_checker = AsyncConstraintChecker::new(Arc::clone(&registry_arc));
        let stealing_checker = WorkStealingConstraintChecker::new(Arc::clone(&registry_arc));
        
        // Create many constraints to check
        let mut constraints = vec![];
        
        // Add repeated constraints to simulate a heavy load
        for _ in 0..10 {
            constraints.push((Type::Normie, "Numeric".to_string()));
            constraints.push((Type::Tea, "Comparable".to_string()));
            constraints.push((Type::Lit, "Comparable".to_string()));
            constraints.push((Type::Thicc, "Numeric".to_string()));
        }
        
        // Check with regular checker
        let start_regular = std::time::Instant::now();
        let regular_results = regular_checker.check_constraints_parallel(constraints.clone());
        let regular_time = start_regular.elapsed();
        
        // Check with work stealing checker
        let start_stealing = std::time::Instant::now();
        let stealing_results = stealing_checker.check_constraints_parallel(constraints);
        let stealing_time = start_stealing.elapsed();
        
        // Verify both give same results
        assert_eq!(regular_results.len(), stealing_results.len());
        for i in 0..regular_results.len() {
            assert_eq!(regular_results[i], stealing_results[i]);
        }
        
        // Get statistics
        let stats = stealing_checker.get_stats();
        
        // The actual time comparison is informational rather than a strict test
        // as performance can vary based on system load
        info!(
            "Regular: {:?}, Work Stealing: {:?}, Steals: {}, Imbalance: {:.2}",
            regular_time, stealing_time, stats.total_steals, stats.workload_imbalance
        );
    }
    
    #[test]
    fn test_caching_vs_no_caching() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let with_cache = WorkStealingConstraintChecker::new(Arc::clone(&registry_arc))
            .with_caching(true);
        let without_cache = WorkStealingConstraintChecker::new(Arc::clone(&registry_arc))
            .with_caching(false);
        
        // Create constraints with some duplicates to test caching
        let mut constraints = vec![];
        
        // Add duplicate constraints
        for _ in 0..5 {
            constraints.push((Type::Normie, "Numeric".to_string()));
            constraints.push((Type::Tea, "Comparable".to_string()));
            constraints.push((Type::Normie, "Numeric".to_string())); // Duplicate
            constraints.push((Type::Tea, "Comparable".to_string())); // Duplicate
            constraints.push((Type::Lit, "Comparable".to_string()));
            constraints.push((Type::Lit, "Comparable".to_string())); // Duplicate
        }
        
        // Check with caching
        let results_with_cache = with_cache.check_constraints_parallel(constraints.clone());
        
        // Check without caching
        let results_without_cache = without_cache.check_constraints_parallel(constraints);
        
        // Verify both give same results
        assert_eq!(results_with_cache.len(), results_without_cache.len());
        for i in 0..results_with_cache.len() {
            assert_eq!(results_with_cache[i], results_without_cache[i]);
        }
        
        // Get statistics
        let stats_with_cache = with_cache.get_stats();
        let stats_without_cache = without_cache.get_stats();
        
        // Cache hit rate should be > 0 with caching
        assert!(stats_with_cache.cache_hit_rate > 0.0);
        
        // Cache hit rate should be 0 without caching
        assert_eq!(stats_without_cache.cache_hit_rate, 0.0);
        
        info!(
            "Cache hit rate: {:.2}, Total steals with cache: {}, without cache: {}",
            stats_with_cache.cache_hit_rate, 
            stats_with_cache.total_steals,
            stats_without_cache.total_steals
        );
    }
    
    #[test]
    fn test_work_stealing_extension_trait() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Check constraints through the extension trait
        let constraints = vec![
            (Type::Normie, "Numeric".to_string()),
            (Type::Tea, "Comparable".to_string()),
            (Type::Lit, "Comparable".to_string()),
            (Type::Thicc, "Numeric".to_string()),
        ];
        
        let results = registry.check_constraints_work_stealing(constraints);
        
        assert_eq!(results.len(), 4);
        for result in &results {
            assert_eq!(*result, Ok(true));
        }
        
        // Test the generic constraints method
        let type_args = vec![Type::Tea, Type::Normie];
        let type_params = vec!["K".to_string(), "V".to_string()];
        let constraints = vec![("K".to_string(), "Comparable".to_string())];
        
        let result = registry.check_generic_constraints_work_stealing(&type_args, &type_params, &constraints);
        assert_eq!(result, Ok(true));
    }
    
    #[test]
    fn test_work_stealing_unbalanced_workload() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let checker = WorkStealingConstraintChecker::new(registry_arc);
        
        // Create constraints with an unbalanced workload
        // (some checks take longer than others)
        let mut constraints = vec![];
        
        // Add lots of quick checks
        for _ in 0..20 {
            constraints.push((Type::Normie, "Numeric".to_string()));
            constraints.push((Type::Tea, "Comparable".to_string()));
        }
        
        // Add a few more complex checks (in practice these would take longer)
        for _ in 0..5 {
            constraints.push((Type::Struct("ComplexStruct".to_string(), vec![]), "Container".to_string()));
            constraints.push((Type::Struct("DeepNested".to_string(), vec![]), "Comparable".to_string()));
        }
        
        // Check with work stealing
        let results = checker.check_constraints_parallel(constraints);
        
        // Verify results count
        assert_eq!(results.len(), 50);
        
        // Get statistics
        let stats = checker.get_stats();
        
        // In an unbalanced workload, we should see some steals
        info!(
            "Work stealing stats - Steals: {}, Workers: {}, Imbalance: {:.2}, Cache hit rate: {:.2}",
            stats.total_steals,
            stats.tasks_per_worker.len(),
            stats.workload_imbalance,
            stats.cache_hit_rate
        );
        
        // Output per-worker stats
        for (i, tasks) in stats.tasks_per_worker.iter().enumerate() {
            info!("Worker {} processed {} tasks", i, tasks);
        }
    }
}