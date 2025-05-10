//! # Asynchronous Constraint Checker
//!
//! This module provides asynchronous constraint checking for improved parallelism
//! in the interface registry system. It allows concurrent execution of multiple constraint
//! checks for better performance in code that uses many generic types with constraints.
//!
//! Features:
//! - Parallel execution of multiple constraint checks
//! - Dynamic worker sizing based on system resources and workload
//! - Performance statistics tracking
//! - Configurable worker thread limits

use crate::core::interface_registry::InterfaceRegistry;
use crate::core::type_checker::Type;
use crate::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use tracing::{debug, info, trace, instrument, warn};

/// The absolute minimum number of worker threads to use for constraint checking
const MIN_WORKERS: usize = 1;

/// The absolute maximum number of worker threads to use for constraint checking
const MAX_WORKERS: usize = 32;

/// The default scaling factor for worker threads (percentage of available cores to use)
const DEFAULT_SCALING_FACTOR: f32 = 0.75;

/// A task for the async constraint checker
#[derive(Debug)]
struct ConstraintCheckTask {
    /// The type to check
    type_: Type,
    /// The interface name to check against
    interface_name: String,
}

/// The result of a constraint check task
#[derive(Debug)]
struct ConstraintCheckResult {
    /// The type that was checked
    type_: Type,
    /// The interface name that was checked
    interface_name: String,
    /// The result of the check
    result: Result<bool, Error>,
}

/// Asynchronous constraint checker that allows parallel constraint checking
#[derive(Debug)]
pub struct AsyncConstraintChecker {
    /// The interface registry to use for constraint checking
    registry: Arc<InterfaceRegistry>,
    /// Statistics for the async checker
    stats: Arc<Mutex<AsyncCheckerStats>>,
    /// Minimum number of worker threads to use
    min_workers: usize,
    /// Maximum number of worker threads to use
    max_workers: usize,
    /// Scaling factor for calculating worker threads (0.0-1.0)
    scaling_factor: f32,
}

/// Statistics for the async constraint checker
#[derive(Debug, Default)]
struct AsyncCheckerStats {
    /// Number of tasks processed
    tasks_processed: usize,
    /// Number of tasks executed in parallel
    parallel_tasks: usize,
    /// Number of tasks executed sequentially
    sequential_tasks: usize,
    /// Maximum concurrency achieved
    max_concurrency: usize,
    /// Average worker utilization (0.0-1.0)
    avg_worker_utilization: f32,
    /// Number of times worker count was adjusted
    worker_count_adjustments: usize,
    /// Average task processing time in milliseconds
    avg_task_time_ms: f32,
    /// Total number of available CPU cores detected
    available_cores: usize,
}

impl AsyncConstraintChecker {
    /// Create a new async constraint checker with the given registry and default settings
    pub fn new(registry: Arc<InterfaceRegistry>) -> Self {
        let available_cores = num_cpus::get();
        let mut stats = AsyncCheckerStats::default();
        stats.available_cores = available_cores;
        
        Self {
            registry,
            stats: Arc::new(Mutex::new(stats)),
            min_workers: MIN_WORKERS,
            max_workers: std::cmp::min(MAX_WORKERS, available_cores),
            scaling_factor: DEFAULT_SCALING_FACTOR,
        }
    }
    
    /// Create a new async constraint checker with custom worker configuration
    pub fn with_worker_config(
        registry: Arc<InterfaceRegistry>,
        min_workers: usize,
        max_workers: usize,
        scaling_factor: f32
    ) -> Self {
        let available_cores = num_cpus::get();
        let mut stats = AsyncCheckerStats::default();
        stats.available_cores = available_cores;
        
        // Ensure constraints are respected
        let min_workers = std::cmp::max(MIN_WORKERS, min_workers);
        let max_workers = std::cmp::min(std::cmp::max(min_workers, max_workers), MAX_WORKERS);
        let scaling_factor = scaling_factor.max(0.1).min(1.0);
        
        Self {
            registry,
            stats: Arc::new(Mutex::new(stats)),
            min_workers,
            max_workers,
            scaling_factor,
        }
    }
    
    /// Calculate the optimal number of worker threads based on current system resources
    #[instrument(skip(self), level = "debug")]
    fn calculate_optimal_workers(&self, num_constraints: usize) -> usize {
        // Get system information
        let available_cores = num_cpus::get();
        let system_load = self.get_system_load();
        
        // Base calculation on available resources and scaling factor
        let theoretical_max = (available_cores as f32 * self.scaling_factor * (1.0 - system_load)) as usize;
        let workload_based = std::cmp::min(num_constraints, theoretical_max);
        
        // Respect configured limits
        let optimal = workload_based.max(self.min_workers).min(self.max_workers);
        
        debug!("Calculated optimal workers: {}, available cores: {}, system load: {:.2}, constraints: {}", 
              optimal, available_cores, system_load, num_constraints);
        
        optimal
    }
    
    /// Get the current system load as a factor between 0.0 and 1.0
    fn get_system_load(&self) -> f32 {
        // This is a simplified implementation that could be enhanced with actual system metrics
        // For now, we use a simple heuristic based on average task processing time
        let stats = self.stats.lock().unwrap();
        
        if stats.tasks_processed == 0 || stats.avg_task_time_ms == 0.0 {
            // If we don't have any history, assume moderate load (0.5)
            return 0.5;
        }
        
        // Use a simple heuristic based on average task time
        // Higher task times indicate higher system load
        // This is a very basic approximation that could be improved
        let load_factor = (stats.avg_task_time_ms / 100.0).min(1.0);
        
        load_factor
    }

    /// Check a set of constraints in parallel
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
        
        // If there are too few constraints, just check them sequentially
        if num_constraints <= 1 {
            self.update_stats(num_constraints, 0, num_constraints, 0.0);
            return constraints
                .into_iter()
                .map(|(type_, interface_name)| {
                    self.registry.check_implementation(&type_, &interface_name)
                })
                .collect();
        }
        
        // Determine optimal number of worker threads based on system resources
        let num_workers = self.calculate_optimal_workers(num_constraints);
        
        info!("Checking {} constraints with {} worker threads (scaling factor: {:.2}, max: {})", 
              num_constraints, num_workers, self.scaling_factor, self.max_workers);
        
        // Convert constraints into tasks
        let tasks: Vec<ConstraintCheckTask> = constraints.iter()
            .map(|(type_, interface_name)| {
                ConstraintCheckTask {
                    type_: type_.clone(),
                    interface_name: interface_name.clone(),
                }
            })
            .collect();
        
        // Create a shared task queue and results collection
        let tasks = Arc::new(Mutex::new(tasks));
        let results = Arc::new(Mutex::new(Vec::with_capacity(num_constraints)));
        
        // Spawn worker threads
        let mut handles = Vec::with_capacity(num_workers);
        for i in 0..num_workers {
            let tasks = Arc::clone(&tasks);
            let results = Arc::clone(&results);
            let registry = Arc::clone(&self.registry);
            
            let handle = thread::spawn(move || {
                trace!("Worker {} started", i);
                loop {
                    // Get a task from the queue
                    let task_opt = {
                        let mut tasks = tasks.lock().unwrap();
                        if tasks.is_empty() {
                            None
                        } else {
                            Some(tasks.remove(0))
                        }
                    };
                    
                    // Process the task if there is one
                    if let Some(task) = task_opt {
                        let type_clone = task.type_.clone();
                        let interface_name_clone = task.interface_name.clone();
                        
                        trace!("Worker {} processing task: {:?} implements {}", i, task.type_, task.interface_name);
                        
                        // Check the constraint
                        let result = registry.check_implementation(&task.type_, &task.interface_name);
                        
                        // Store the result
                        let check_result = ConstraintCheckResult {
                            type_: task.type_,
                            interface_name: task.interface_name,
                            result,
                        };
                        
                        let mut results = results.lock().unwrap();
                        results.push(check_result);
                        
                        trace!("Worker {} completed task: {:?} implements {}", i, type_clone, interface_name_clone);
                    } else {
                        // No more tasks
                        break;
                    }
                }
                trace!("Worker {} finished", i);
            });
            
            handles.push(handle);
        }
        
        // Track the start time for performance measurement
        let start_time = std::time::Instant::now();
        
        // Wait for all workers to finish
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Calculate the average task time
        let elapsed = start_time.elapsed();
        let elapsed_ms = elapsed.as_secs() as f32 * 1000.0 + elapsed.subsec_millis() as f32;
        let avg_task_time_ms = if num_constraints > 0 {
            elapsed_ms / num_constraints as f32
        } else {
            0.0
        };
        
        // Calculate stats for this run
        self.update_stats(num_constraints, num_workers, 0, avg_task_time_ms);
        
        debug!("Completed {} constraint checks in {:.2}ms (avg {:.2}ms per task)", 
               num_constraints, elapsed_ms, avg_task_time_ms);
        
        // Collect and return results in the order of the original constraints
        let mut result_map = std::collections::HashMap::new();
        let results = results.lock().unwrap();
        
        for result in results.iter() {
            let key = (result.type_.clone(), result.interface_name.clone());
            result_map.insert(key, result.result.clone());
        }
        
        // Return results in the original order
        let mut ordered_results = Vec::with_capacity(num_constraints);
        for (type_, interface_name) in constraints {
            let key = (type_.clone(), interface_name.clone());
            if let Some(result) = result_map.get(&key) {
                ordered_results.push(result.clone());
            } else {
                // This should never happen if all tasks were processed
                warn!("Missing result for {:?} implements {}", type_, interface_name);
                ordered_results.push(Err(Error::new("CNST01", "Missing constraint check result", None)));
            }
        }
        
        ordered_results
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
    
    /// Update statistics for the async constraint checker
    fn update_stats(&self, total: usize, parallel: usize, sequential: usize, avg_task_time_ms: f32) {
        let mut stats = self.stats.lock().unwrap();
        
        // If we've processed tasks before, calculate a weighted average of task times
        if stats.tasks_processed > 0 && avg_task_time_ms > 0.0 {
            stats.avg_task_time_ms = (stats.avg_task_time_ms * stats.tasks_processed as f32 + 
                                      avg_task_time_ms * total as f32) / 
                                      (stats.tasks_processed + total) as f32;
        } else if avg_task_time_ms > 0.0 {
            stats.avg_task_time_ms = avg_task_time_ms;
        }
        
        // Update worker utilization if we used parallel workers
        if parallel > 0 {
            let utilization = total as f32 / (parallel as f32 * avg_task_time_ms.max(0.1));
            // Update with a moving average
            if stats.avg_worker_utilization > 0.0 {
                stats.avg_worker_utilization = 0.7 * stats.avg_worker_utilization + 0.3 * utilization;
            } else {
                stats.avg_worker_utilization = utilization;
            }
        }
        
        // Update basic stats
        stats.tasks_processed += total;
        stats.parallel_tasks += parallel;
        stats.sequential_tasks += sequential;
        stats.max_concurrency = std::cmp::max(stats.max_concurrency, parallel);
        
        // Ensure available cores is updated
        if stats.available_cores == 0 {
            stats.available_cores = num_cpus::get();
        }
    }
    
    /// Get statistics for the async constraint checker
    pub fn get_stats(&self) -> (usize, usize, usize, usize, f32, f32, usize) {
        let stats = self.stats.lock().unwrap();
        (
            stats.tasks_processed,
            stats.parallel_tasks,
            stats.sequential_tasks,
            stats.max_concurrency,
            stats.avg_worker_utilization,
            stats.avg_task_time_ms,
            stats.available_cores,
        )
    }
    
    /// Get a detailed statistics report as a structured object
    pub fn get_detailed_stats(&self) -> AsyncConstraintCheckerStats {
        let stats = self.stats.lock().unwrap();
        AsyncConstraintCheckerStats {
            tasks_processed: stats.tasks_processed,
            parallel_tasks: stats.parallel_tasks,
            sequential_tasks: stats.sequential_tasks,
            max_concurrency: stats.max_concurrency,
            avg_worker_utilization: stats.avg_worker_utilization,
            avg_task_time_ms: stats.avg_task_time_ms,
            available_cores: stats.available_cores,
            worker_count_adjustments: stats.worker_count_adjustments,
            min_workers: self.min_workers,
            max_workers: self.max_workers,
            scaling_factor: self.scaling_factor,
        }
    }
}

/// Public statistics for the async constraint checker
#[derive(Debug, Clone)]
pub struct AsyncConstraintCheckerStats {
    /// Number of tasks processed
    pub tasks_processed: usize,
    /// Number of tasks executed in parallel
    pub parallel_tasks: usize,
    /// Number of tasks executed sequentially
    pub sequential_tasks: usize,
    /// Maximum concurrency achieved
    pub max_concurrency: usize,
    /// Average worker utilization (0.0-1.0)
    pub avg_worker_utilization: f32,
    /// Average task processing time in milliseconds
    pub avg_task_time_ms: f32,
    /// Total number of available CPU cores detected
    pub available_cores: usize,
    /// Number of times worker count was adjusted
    pub worker_count_adjustments: usize,
    /// Minimum number of worker threads to use
    pub min_workers: usize,
    /// Maximum number of worker threads to use
    pub max_workers: usize,
    /// Scaling factor for calculating worker threads (0.0-1.0)
    pub scaling_factor: f32,
}

/// Extension trait for the InterfaceRegistry to add async constraint checking methods
pub trait AsyncConstraintChecking {
    /// Check a set of constraints in parallel
    fn check_constraints_parallel(
        &self,
        constraints: Vec<(Type, String)>,
    ) -> Vec<Result<bool, Error>>;
    
    /// Check a single set of generic constraints in parallel
    fn check_generic_constraints_parallel(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)],
    ) -> Result<bool, Error>;
    
    /// Create a new constraint checker with custom worker configuration
    fn with_worker_config(
        &self,
        min_workers: usize,
        max_workers: usize,
        scaling_factor: f32,
    ) -> Arc<AsyncConstraintChecker>;
    
    /// Check constraints in parallel with custom worker configuration
    fn check_constraints_with_config(
        &self,
        constraints: Vec<(Type, String)>,
        min_workers: usize,
        max_workers: usize,
        scaling_factor: f32,
    ) -> Vec<Result<bool, Error>>;
}

impl AsyncConstraintChecking for InterfaceRegistry {
    fn check_constraints_parallel(
        &self,
        constraints: Vec<(Type, String)>,
    ) -> Vec<Result<bool, Error>> {
        let checker = AsyncConstraintChecker::new(Arc::new(self.clone()));
        checker.check_constraints_parallel(constraints)
    }
    
    fn check_generic_constraints_parallel(
        &self,
        type_args: &[Type],
        type_params: &[String],
        constraints: &[(String, String)],
    ) -> Result<bool, Error> {
        let checker = AsyncConstraintChecker::new(Arc::new(self.clone()));
        checker.check_generic_constraints_parallel(type_args, type_params, constraints)
    }
    
    /// Create a new constraint checker with custom worker configuration
    fn with_worker_config(
        &self,
        min_workers: usize,
        max_workers: usize,
        scaling_factor: f32,
    ) -> Arc<AsyncConstraintChecker> {
        let checker = AsyncConstraintChecker::with_worker_config(
            Arc::new(self.clone()),
            min_workers,
            max_workers,
            scaling_factor
        );
        Arc::new(checker)
    }
    
    /// Check constraints in parallel with custom worker configuration
    fn check_constraints_with_config(
        &self,
        constraints: Vec<(Type, String)>,
        min_workers: usize,
        max_workers: usize,
        scaling_factor: f32,
    ) -> Vec<Result<bool, Error>> {
        let checker = self.with_worker_config(min_workers, max_workers, scaling_factor);
        checker.check_constraints_parallel(constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::type_checker::Type;
    
    #[path = "../../tests/common.rs"]
    mod common;
    
    #[test]
    fn test_async_constraint_checker_single_constraint() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let checker = AsyncConstraintChecker::new(registry_arc);
        
        // Check a single constraint
        let constraints = vec![(Type::Normie, "Numeric".to_string())];
        let results = checker.check_constraints_parallel(constraints);
        
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], Ok(true));
    }
    
    #[test]
    fn test_dynamic_worker_sizing() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        
        // Create a checker with custom worker configuration
        let checker = AsyncConstraintChecker::with_worker_config(
            registry_arc,
            2, // min_workers
            6, // max_workers
            0.5, // scaling_factor
        );
        
        // Verify the configuration was applied correctly
        assert_eq!(checker.min_workers, 2);
        assert_eq!(checker.max_workers, 6);
        assert_eq!(checker.scaling_factor, 0.5);
        
        // Create constraints to check
        let constraints = vec![
            (Type::Normie, "Numeric".to_string()),
            (Type::Tea, "Comparable".to_string()),
            (Type::Thicc, "Numeric".to_string()),
            (Type::Lit, "Comparable".to_string()),
        ];
        
        // Check constraints and verify results
        let results = checker.check_constraints_parallel(constraints);
        assert_eq!(results.len(), 4);
        
        // All of these should be true
        for result in &results {
            assert_eq!(*result, Ok(true));
        }
        
        // Check that stats were updated
        let stats = checker.get_detailed_stats();
        assert_eq!(stats.tasks_processed, 4);
        assert!(stats.avg_task_time_ms > 0.0);
        assert_eq!(stats.min_workers, 2);
        assert_eq!(stats.max_workers, 6);
        assert_eq!(stats.scaling_factor, 0.5);
    }
    
    #[test]
    fn test_async_constraint_checker_multiple_constraints() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let checker = AsyncConstraintChecker::new(registry_arc);
        
        // Check multiple constraints
        let constraints = vec![
            (Type::Normie, "Numeric".to_string()),
            (Type::Tea, "Comparable".to_string()),
            (Type::Lit, "Numeric".to_string()), // Should be false
        ];
        
        let results = checker.check_constraints_parallel(constraints);
        
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], Ok(true));   // Normie implements Numeric
        assert_eq!(results[1], Ok(true));   // Tea implements Comparable
        assert_eq!(results[2], Ok(false));  // Lit does not implement Numeric
    }
    
    #[test]
    fn test_async_checker_generic_constraints() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        let registry_arc = Arc::new(registry);
        let checker = AsyncConstraintChecker::new(registry_arc);
        
        // Type arguments and parameters
        let type_args = vec![Type::Tea, Type::Normie];
        let type_params = vec!["K".to_string(), "V".to_string()];
        
        // Constraints: K must implement Comparable
        let constraints = vec![("K".to_string(), "Comparable".to_string())];
        
        // Check constraints in parallel - should succeed because Tea implements Comparable
        let result = checker.check_generic_constraints_parallel(&type_args, &type_params, &constraints);
        assert_eq!(result, Ok(true));
        
        // Now try with a type that doesn't implement Comparable
        let non_comparable = Type::Struct("NonComparable".to_string(), vec![]);
        let type_args = vec![non_comparable, Type::Normie];
        
        let result = checker.check_generic_constraints_parallel(&type_args, &type_params, &constraints);
        assert_eq!(result, Ok(false));
    }
    
    #[test]
    fn test_async_constraint_extension_trait() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Check constraints through the extension trait
        let constraints = vec![
            (Type::Normie, "Numeric".to_string()),
            (Type::Tea, "Comparable".to_string()),
        ];
        
        let results = registry.check_constraints_parallel(constraints);
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], Ok(true));  // Normie implements Numeric
        assert_eq!(results[1], Ok(true));  // Tea implements Comparable
        
        // Test the generic constraints method
        let type_args = vec![Type::Tea, Type::Normie];
        let type_params = vec!["K".to_string(), "V".to_string()];
        let constraints = vec![("K".to_string(), "Comparable".to_string())];
        
        let result = registry.check_generic_constraints_parallel(&type_args, &type_params, &constraints);
        assert_eq!(result, Ok(true));
    }
    
    #[test]
    fn test_extension_trait_with_worker_config() {
        common::tracing::setup();
        
        let mut registry = InterfaceRegistry::new();
        registry.populate_with_defaults();
        
        // Use the extension trait to create a custom worker configuration
        let constraints = vec![
            (Type::Normie, "Numeric".to_string()),
            (Type::Tea, "Comparable".to_string()),
            (Type::Thicc, "Numeric".to_string()),
            (Type::Lit, "Comparable".to_string()),
        ];
        
        // Test the check_constraints_with_config method
        let results = registry.check_constraints_with_config(
            constraints.clone(),
            1,  // min_workers
            4,  // max_workers
            0.8 // scaling_factor
        );
        
        assert_eq!(results.len(), 4);
        for result in &results {
            assert_eq!(*result, Ok(true));
        }
        
        // Test the with_worker_config method
        let checker = registry.with_worker_config(2, 8, 0.7);
        let results = checker.check_constraints_parallel(constraints);
        
        assert_eq!(results.len(), 4);
        for result in &results {
            assert_eq!(*result, Ok(true));
        }
        
        // Get and verify stats
        let stats = checker.get_detailed_stats();
        assert_eq!(stats.min_workers, 2);
        assert_eq!(stats.max_workers, 8);
        assert_eq!(stats.scaling_factor, 0.7);
    }
}