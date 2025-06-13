/// Parallel Compilation Infrastructure
/// 
/// Provides advanced parallel compilation features including module-level
/// parallelization, pipeline parallelism, and intelligent work distribution.

use crate::error::{Error, Result};
use crate::optimization::optimization_levels::LevelConfig;
use crate::optimization::compilation_speed::{CompilationUnit, DependencyGraph, CompilationStatus};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock, mpsc, Condvar};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn, error};
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use rayon::prelude::*;

/// Parallel compilation configuration
#[derive(Debug, Clone)]
pub struct ParallelCompilationConfig {
    /// Number of worker threads
    pub worker_threads: usize,
    /// Enable pipeline parallelism
    pub enable_pipeline_parallelism: bool,
    /// Enable module-level parallelism
    pub enable_module_parallelism: bool,
    /// Maximum queue size per worker
    pub max_queue_size: usize,
    /// Compilation timeout per unit
    pub compilation_timeout: Duration,
    /// Enable work stealing
    pub enable_work_stealing: bool,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
}

/// Load balancing strategies
#[derive(Debug, Clone, Copy)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Least loaded worker
    LeastLoaded,
    /// Priority-based distribution
    Priority,
    /// Work stealing
    WorkStealing,
}

impl Default for ParallelCompilationConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get().max(1),
            enable_pipeline_parallelism: true,
            enable_module_parallelism: true,
            max_queue_size: 100,
            compilation_timeout: Duration::from_secs(60),
            enable_work_stealing: true,
            load_balancing: LoadBalancingStrategy::WorkStealing,
        }
    }
}

/// Compilation work item
#[derive(Debug, Clone)]
pub struct WorkItem {
    /// Compilation unit
    pub unit: CompilationUnit,
    /// Priority (higher = more urgent)
    pub priority: u32,
    /// Dependencies that must complete first
    pub dependencies: Vec<String>,
    /// Estimated compilation time
    pub estimated_time: Option<Duration>,
}

/// Worker thread state
#[derive(Debug)]
pub struct WorkerState {
    /// Worker ID
    pub id: usize,
    /// Current work item
    pub current_work: Option<String>,
    /// Number of completed items
    pub completed_items: usize,
    /// Total compilation time
    pub total_time: Duration,
    /// Worker load (0.0 to 1.0)
    pub load: f64,
    /// Last activity timestamp
    pub last_activity: Instant,
}

/// Parallel compiler with advanced scheduling
pub struct ParallelCompiler {
    config: ParallelCompilationConfig,
    workers: Vec<Worker>,
    work_queue: Arc<RwLock<VecDeque<WorkItem>>>,
    completed_work: Arc<RwLock<HashMap<String, CompilationResult>>>,
    dependency_graph: Arc<Mutex<DependencyGraph>>,
    statistics: Arc<Mutex<ParallelCompilationStatistics>>,
    scheduler: Arc<Mutex<WorkScheduler>>,
    shutdown_signal: Arc<(Mutex<bool>, Condvar)>,
}

/// Compilation result
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// Unit ID
    pub unit_id: String,
    /// Compilation status
    pub status: CompilationStatus,
    /// Compilation time
    pub compilation_time: Duration,
    /// Worker that completed the work
    pub worker_id: usize,
    /// Any error message
    pub error_message: Option<String>,
}

/// Parallel compilation statistics
#[derive(Debug, Clone, Default)]
pub struct ParallelCompilationStatistics {
    /// Total compilation units processed
    pub units_processed: usize,
    /// Total compilation time across all workers
    pub total_compilation_time: Duration,
    /// Wall clock time for entire compilation
    pub wall_clock_time: Duration,
    /// Number of cache hits
    pub cache_hits: usize,
    /// Number of cache misses
    pub cache_misses: usize,
    /// Average worker utilization
    pub average_utilization: f64,
    /// Peak memory usage
    pub peak_memory_usage: usize,
    /// Work stealing events
    pub work_stealing_events: usize,
}

impl ParallelCompilationStatistics {
    /// Calculate parallelization efficiency
    pub fn efficiency(&self) -> f64 {
        if self.wall_clock_time.as_millis() == 0 {
            return 0.0;
        }
        
        let parallel_time = self.total_compilation_time.as_millis() as f64;
        let wall_time = self.wall_clock_time.as_millis() as f64;
        
        parallel_time / wall_time
    }

    /// Calculate cache hit ratio
    pub fn cache_hit_ratio(&self) -> f64 {
        let total_accesses = self.cache_hits + self.cache_misses;
        if total_accesses == 0 {
            return 0.0;
        }
        
        self.cache_hits as f64 / total_accesses as f64
    }
}

/// Work scheduler for intelligent work distribution
pub struct WorkScheduler {
    config: ParallelCompilationConfig,
    worker_states: Vec<WorkerState>,
    pending_work: VecDeque<WorkItem>,
    ready_work: VecDeque<WorkItem>,
    next_worker: usize, // For round-robin scheduling
}

impl WorkScheduler {
    /// Create a new work scheduler
    pub fn new(config: ParallelCompilationConfig) -> Self {
        let worker_states = (0..config.worker_threads)
            .map(|id| WorkerState {
                id,
                current_work: None,
                completed_items: 0,
                total_time: Duration::default(),
                load: 0.0,
                last_activity: Instant::now(),
            })
            .collect();

        Self {
            config,
            worker_states,
            pending_work: VecDeque::new(),
            ready_work: VecDeque::new(),
            next_worker: 0,
        }
    }

    /// Add work items to the scheduler
    #[instrument(skip(self, items))]
    pub fn add_work(&mut self, items: Vec<WorkItem>) {
        debug!("Adding {} work items to scheduler", items.len());
        
        for item in items {
            if item.dependencies.is_empty() {
                self.ready_work.push_back(item);
            } else {
                self.pending_work.push_back(item);
            }
        }
        
        // Sort ready work by priority
        let mut ready_items: Vec<_> = self.ready_work.drain(..).collect();
        ready_items.sort_by(|a, b| b.priority.cmp(&a.priority));
        self.ready_work.extend(ready_items);
    }

    /// Get next work item for a worker
    #[instrument(skip(self))]
    pub fn get_work(&mut self, worker_id: usize) -> Option<WorkItem> {
        match self.config.load_balancing {
            LoadBalancingStrategy::RoundRobin => self.get_work_round_robin(),
            LoadBalancingStrategy::LeastLoaded => self.get_work_least_loaded(worker_id),
            LoadBalancingStrategy::Priority => self.get_work_priority(),
            LoadBalancingStrategy::WorkStealing => self.get_work_stealing(worker_id),
        }
    }

    /// Round-robin work distribution
    fn get_work_round_robin(&mut self) -> Option<WorkItem> {
        self.ready_work.pop_front()
    }

    /// Least loaded worker strategy
    fn get_work_least_loaded(&mut self, worker_id: usize) -> Option<WorkItem> {
        // Find the worker with the least load
        let least_loaded_worker = self.worker_states
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.load.partial_cmp(&b.load).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, _)| id)
            .unwrap_or(0);

        if least_loaded_worker == worker_id {
            self.ready_work.pop_front()
        } else {
            None
        }
    }

    /// Priority-based work distribution
    fn get_work_priority(&mut self) -> Option<WorkItem> {
        // Work is already sorted by priority
        self.ready_work.pop_front()
    }

    /// Work stealing strategy
    fn get_work_stealing(&mut self, worker_id: usize) -> Option<WorkItem> {
        // Try to get work from own queue first
        if let Some(work) = self.ready_work.pop_front() {
            return Some(work);
        }

        // Try to steal work from other workers
        // This is a simplified implementation
        None
    }

    /// Mark work as completed and check for newly ready work
    #[instrument(skip(self))]
    pub fn complete_work(&mut self, unit_id: &str, worker_id: usize, duration: Duration) {
        debug!("Marking work {} as completed by worker {}", unit_id, worker_id);

        // Update worker state
        if let Some(worker) = self.worker_states.get_mut(worker_id) {
            worker.completed_items += 1;
            worker.total_time += duration;
            worker.current_work = None;
            worker.last_activity = Instant::now();
            worker.load = self.calculate_worker_load(worker_id);
        }

        // Check if any pending work is now ready
        let mut newly_ready = Vec::new();
        self.pending_work.retain(|item| {
            if item.dependencies.iter().any(|dep| dep == unit_id) {
                // Remove this dependency
                let mut updated_item = item.clone();
                updated_item.dependencies.retain(|dep| dep != unit_id);
                
                if updated_item.dependencies.is_empty() {
                    newly_ready.push(updated_item);
                    false
                } else {
                    *item = updated_item;
                    true
                }
            } else {
                true
            }
        });

        // Add newly ready work to ready queue
        for item in newly_ready {
            self.ready_work.push_back(item);
        }

        // Resort ready work by priority
        let mut ready_items: Vec<_> = self.ready_work.drain(..).collect();
        ready_items.sort_by(|a, b| b.priority.cmp(&a.priority));
        self.ready_work.extend(ready_items);
    }

    /// Calculate current load for a worker
    fn calculate_worker_load(&self, worker_id: usize) -> f64 {
        if let Some(worker) = self.worker_states.get(worker_id) {
            // Simple load calculation based on recent activity
            let time_since_activity = worker.last_activity.elapsed();
            if time_since_activity > Duration::from_secs(5) {
                0.0 // Idle
            } else if worker.current_work.is_some() {
                1.0 // Busy
            } else {
                0.5 // Available
            }
        } else {
            0.0
        }
    }

    /// Check if there's any work available
    pub fn has_work(&self) -> bool {
        !self.ready_work.is_empty()
    }

    /// Get scheduler statistics
    pub fn get_statistics(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        
        let total_completed: usize = self.worker_states.iter().map(|w| w.completed_items).sum();
        let average_load: f64 = self.worker_states.iter().map(|w| w.load).sum::<f64>() / self.worker_states.len() as f64;
        
        stats.insert("total_completed".to_string(), total_completed as f64);
        stats.insert("average_load".to_string(), average_load);
        stats.insert("pending_work".to_string(), self.pending_work.len() as f64);
        stats.insert("ready_work".to_string(), self.ready_work.len() as f64);
        
        stats
    }
}

/// Worker thread for parallel compilation
pub struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
    work_sender: Sender<WorkItem>,
    result_receiver: Receiver<CompilationResult>,
}

impl Worker {
    /// Create a new worker
    pub fn new(
        id: usize,
        config: ParallelCompilationConfig,
        scheduler: Arc<Mutex<WorkScheduler>>,
        statistics: Arc<Mutex<ParallelCompilationStatistics>>,
        shutdown_signal: Arc<(Mutex<bool>, Condvar)>,
    ) -> Self {
        let (work_sender, work_receiver) = bounded(config.max_queue_size);
        let (result_sender, result_receiver) = unbounded();

        let handle = thread::spawn(move || {
            Self::worker_loop(
                id,
                work_receiver,
                result_sender,
                scheduler,
                statistics,
                shutdown_signal,
                config,
            );
        });

        Self {
            id,
            handle: Some(handle),
            work_sender,
            result_receiver,
        }
    }

    /// Worker main loop
    fn worker_loop(
        worker_id: usize,
        work_receiver: Receiver<WorkItem>,
        result_sender: Sender<CompilationResult>,
        scheduler: Arc<Mutex<WorkScheduler>>,
        statistics: Arc<Mutex<ParallelCompilationStatistics>>,
        shutdown_signal: Arc<(Mutex<bool>, Condvar)>,
        config: ParallelCompilationConfig,
    ) {
        debug!("Worker {} starting", worker_id);

        loop {
            // Check for shutdown signal
            {
                let (lock, _) = &*shutdown_signal;
                if *lock.lock().unwrap() {
                    debug!("Worker {} received shutdown signal", worker_id);
                    break;
                }
            }

            // Try to get work from scheduler
            let work_item = {
                let mut scheduler = scheduler.lock().unwrap();
                scheduler.get_work(worker_id)
            };

            if let Some(work) = work_item {
                // Process the work item
                let result = Self::process_work_item(worker_id, work, &config);
                
                // Update statistics
                {
                    let mut stats = statistics.lock().unwrap();
                    stats.units_processed += 1;
                    stats.total_compilation_time += result.compilation_time;
                }

                // Notify scheduler of completion
                {
                    let mut scheduler = scheduler.lock().unwrap();
                    scheduler.complete_work(&result.unit_id, worker_id, result.compilation_time);
                }

                // Send result
                if let Err(e) = result_sender.send(result) {
                    error!("Worker {} failed to send result: {}", worker_id, e);
                    break;
                }
            } else {
                // No work available, sleep briefly
                thread::sleep(Duration::from_millis(10));
            }
        }

        debug!("Worker {} shutting down", worker_id);
    }

    /// Process a single work item
    fn process_work_item(
        worker_id: usize,
        work_item: WorkItem,
        config: &ParallelCompilationConfig,
    ) -> CompilationResult {
        let start_time = Instant::now();
        let unit_id = work_item.unit.id.clone();

        debug!("Worker {} processing unit {}", worker_id, unit_id);

        // Simulate compilation work
        // In a real implementation, this would call the actual compiler
        let compilation_time = Duration::from_millis(100); // Simulate work
        thread::sleep(compilation_time);

        let result = CompilationResult {
            unit_id,
            status: CompilationStatus::Completed,
            compilation_time: start_time.elapsed(),
            worker_id,
            error_message: None,
        };

        debug!("Worker {} completed unit {} in {:?}", 
               worker_id, result.unit_id, result.compilation_time);

        result
    }

    /// Send work to this worker
    pub fn send_work(&self, work: WorkItem) -> Result<()> {
        self.work_sender.send(work)
            .map_err(|_| Error::Internal("Failed to send work to worker".to_string()))
    }

    /// Try to receive a result from this worker
    pub fn try_receive_result(&self) -> Option<CompilationResult> {
        self.result_receiver.try_recv().ok()
    }

    /// Shutdown the worker
    pub fn shutdown(mut self) -> Result<()> {
        if let Some(handle) = self.handle.take() {
            handle.join()
                .map_err(|_| Error::Internal("Failed to join worker thread".to_string()))?;
        }
        Ok(())
    }
}

impl ParallelCompiler {
    /// Create a new parallel compiler
    #[instrument(skip(config))]
    pub fn new(config: &super::OptimizationConfig) -> Result<Self> {
        let parallel_config = ParallelCompilationConfig {
            worker_threads: config.max_parallel_threads,
            ..Default::default()
        };

        info!("Creating parallel compiler with {} workers", parallel_config.worker_threads);

        let work_queue = Arc::new(RwLock::new(VecDeque::new()));
        let completed_work = Arc::new(RwLock::new(HashMap::new()));
        let dependency_graph = Arc::new(Mutex::new(DependencyGraph::new()));
        let statistics = Arc::new(Mutex::new(ParallelCompilationStatistics::default()));
        let scheduler = Arc::new(Mutex::new(WorkScheduler::new(parallel_config.clone())));
        let shutdown_signal = Arc::new((Mutex::new(false), Condvar::new()));

        // Create worker threads
        let mut workers = Vec::new();
        for i in 0..parallel_config.worker_threads {
            let worker = Worker::new(
                i,
                parallel_config.clone(),
                scheduler.clone(),
                statistics.clone(),
                shutdown_signal.clone(),
            );
            workers.push(worker);
        }

        Ok(Self {
            config: parallel_config,
            workers,
            work_queue,
            completed_work,
            dependency_graph,
            statistics,
            scheduler,
            shutdown_signal,
        })
    }

    /// Compile multiple units in parallel
    #[instrument(skip(self, units))]
    pub fn compile_parallel(&self, units: Vec<CompilationUnit>) -> Result<HashMap<String, CompilationResult>> {
        let start_time = Instant::now();
        
        info!("Starting parallel compilation of {} units", units.len());

        // Convert units to work items
        let work_items: Vec<WorkItem> = units.into_iter().map(|unit| {
            WorkItem {
                priority: unit.priority,
                dependencies: unit.dependencies.clone(),
                estimated_time: None,
                unit,
            }
        }).collect();

        // Add work to scheduler
        {
            let mut scheduler = self.scheduler.lock().unwrap();
            scheduler.add_work(work_items);
        }

        // Distribute work to workers and collect results
        let mut results = HashMap::new();
        let total_units = {
            let scheduler = self.scheduler.lock().unwrap();
            scheduler.ready_work.len() + scheduler.pending_work.len()
        };

        while results.len() < total_units {
            // Collect results from workers
            for worker in &self.workers {
                if let Some(result) = worker.try_receive_result() {
                    results.insert(result.unit_id.clone(), result);
                }
            }

            // Brief sleep to prevent busy waiting
            thread::sleep(Duration::from_millis(1));
        }

        // Update final statistics
        {
            let mut stats = self.statistics.lock().unwrap();
            stats.wall_clock_time = start_time.elapsed();
        }

        info!("Parallel compilation completed in {:?}", start_time.elapsed());
        
        Ok(results)
    }

    /// Get compilation statistics
    pub fn get_statistics(&self) -> ParallelCompilationStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// Print compilation summary
    pub fn print_summary(&self) {
        let stats = self.get_statistics();
        
        println!("⚡ Parallel Compilation Summary:");
        println!("   Worker threads: {}", self.config.worker_threads);
        println!("   Units processed: {}", stats.units_processed);
        println!("   Wall clock time: {:?}", stats.wall_clock_time);
        println!("   Total compilation time: {:?}", stats.total_compilation_time);
        println!("   Parallelization efficiency: {:.2}x", stats.efficiency());
        println!("   Average worker utilization: {:.1}%", stats.average_utilization * 100.0);
        println!("   Cache hit ratio: {:.1}%", stats.cache_hit_ratio() * 100.0);
        println!("   Work stealing events: {}", stats.work_stealing_events);
    }
}

impl Drop for ParallelCompiler {
    fn drop(&mut self) {
        // Signal shutdown
        {
            let (lock, cvar) = &*self.shutdown_signal;
            let mut shutdown = lock.lock().unwrap();
            *shutdown = true;
            cvar.notify_all();
        }

        // Shutdown all workers
        let workers = std::mem::take(&mut self.workers);
        for worker in workers {
            if let Err(e) = worker.shutdown() {
                error!("Failed to shutdown worker: {}", e);
            }
        }
    }
}

/// Module compiler for individual compilation units
pub struct ModuleCompiler {
    config: ParallelCompilationConfig,
}

impl ModuleCompiler {
    /// Create a new module compiler
    pub fn new(config: ParallelCompilationConfig) -> Self {
        Self { config }
    }

    /// Compile a single module
    #[instrument(skip(self, unit))]
    pub fn compile_module(&self, unit: &CompilationUnit) -> Result<CompilationResult> {
        let start_time = Instant::now();
        
        debug!("Compiling module: {}", unit.module_name);

        // Simulate module compilation
        // In a real implementation, this would call the lexer, parser, type checker, and code generator
        thread::sleep(Duration::from_millis(50));

        Ok(CompilationResult {
            unit_id: unit.id.clone(),
            status: CompilationStatus::Completed,
            compilation_time: start_time.elapsed(),
            worker_id: 0,
            error_message: None,
        })
    }
}
