// Advanced Parallel Compilation System
// 
// Provides intelligent parallel compilation with dependency-aware scheduling,
// CPU core optimization, memory-conscious task distribution, and compilation
// pipeline optimization for maximum developer productivity.

use crate::build_system::{BuildConfig, BuildTarget, BuildProfile, BuildError, BuildResult, BuildStatistics, TargetType};
use crate::common_types::optimization_level::OptimizationLevel;
use crate::build_system::dependency_resolver::{DependencyGraph, DependencyResolver};
use crate::error::CursedError;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc, Condvar};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn, instrument};
use num_cpus;
use sysinfo;
use sha2;

/// Parallel compilation coordinator
#[derive(Debug)]
pub struct ParallelCompiler {
/// Parallel compilation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCompilationConfig {
    /// Maximum number of worker threads
    
    /// Memory limit per worker (in MB)
    
    /// CPU affinity optimization
    
    /// Enable compilation pipeline overlapping
    
    /// Task scheduling strategy
    
    /// Resource monitoring interval
    
    /// Maximum queue depth per worker
    
    /// Enable compilation result streaming
    
    /// Adaptive worker scaling
    
    /// Enable cross-module optimization
/// Task scheduling strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulingStrategy {
    /// First In, First Out
    /// Shortest Job First
    /// Critical Path First
    /// Dependency-Aware Round Robin
    /// Work-Stealing Queue
    /// Adaptive (switches strategies based on workload)
/// Compilation task definition
#[derive(Debug, Clone)]
pub struct CompilationTask {
/// Task priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
/// Individual compilation unit
#[derive(Debug, Clone)]
pub struct CompilationUnit {
/// Worker pool management
#[derive(Debug)]
pub struct WorkerPool {
/// Individual compilation worker
#[derive(Debug)]
pub struct CompilationWorker {
/// Task distributor for load balancing
#[derive(Debug)]
pub struct TaskDistributor {
/// Worker load tracking
#[derive(Debug, Clone)]
pub struct WorkerLoad {
/// Task scheduler with dependency awareness
#[derive(Debug)]
pub struct TaskScheduler {
/// Resource monitor for system optimization
#[derive(Debug)]
pub struct ResourceMonitor {
/// Resource usage thresholds
#[derive(Debug, Clone)]
pub struct ResourceThresholds {
    pub max_memory_usage: f64,  // Percentage
    pub max_cpu_usage: f64,     // Percentage
    pub max_disk_io: usize,     // MB/s
/// Resource alert types
#[derive(Debug, Clone)]
pub enum ResourceAlert {
/// Compilation cache for incremental builds
#[derive(Debug)]
pub struct CompilationCache {
/// Cached compilation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCompilationResult {
/// Worker execution result
#[derive(Debug)]
pub struct WorkerResult {
/// Parallel compilation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCompilationResult {
/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
/// Bottleneck analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckAnalysis {
/// Types of bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
/// Worker performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStatistics {
/// Scheduler performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerStatistics {
/// Cache performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
/// Resource monitoring statistics
#[derive(Debug, Clone)]
pub struct ResourceStatistics {
/// Parallel compilation efficiency analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelEfficiencyAnalysis {
    /// Overall parallel efficiency (0.0 to 1.0)
    
    /// Load balance score across workers (0.0 to 1.0)
    
    /// Average worker utilization (0.0 to 1.0)
    
    /// Theoretical maximum speedup possible
    
    /// Recommendations for improving scalability
/// Task-specific memory monitoring
#[derive(Debug)]
pub struct TaskMemoryMonitor {
impl Default for ParallelCompilationConfig {
    fn default() -> Self {
        Self {
        }
    }
impl ParallelCompiler {
    /// Create new parallel compiler
    pub fn new(config: ParallelCompilationConfig) -> crate::error::Result<()> {
        let worker_pool = WorkerPool::new(&config)?;
        let task_scheduler = TaskScheduler::new();
        let resource_monitor = ResourceMonitor::new(&config)?;
        let compilation_cache = Arc::new(Mutex::new(CompilationCache::new()));
        
        Ok(ParallelCompiler {
        })
    /// Compile targets in parallel with dependency awareness
    #[instrument(skip(self, targets))]
    pub async fn compile_parallel(
    ) -> crate::error::Result<()> {
        info!("Starting parallel compilation of {} targets", targets.len());
        let start_time = Instant::now();
        
        // Analyze dependencies and create optimal schedule
        let scheduled_tasks = self.task_scheduler.schedule_tasks(targets)?;
        
        // Start resource monitoring
        self.resource_monitor.start_monitoring()?;
        
        // Execute compilation tasks
        let worker_results = self.execute_tasks_parallel(scheduled_tasks).await?;
        
        // Stop monitoring and collect statistics
        let resource_stats = self.resource_monitor.stop_monitoring()?;
        
        // Analyze results and generate comprehensive report
        let result = self.analyze_compilation_results(
        )?;
        
        info!(
            result.parallel_efficiency * 100.0
        );
        
        Ok(result)
    /// Execute tasks with intelligent load balancing
    async fn execute_tasks_parallel(
    ) -> crate::error::Result<()> {
        let (result_sender, result_receiver) = mpsc::channel();
        let task_distributor = Arc::new(Mutex::new(TaskDistributor::new(tasks, self.config.scheduling_strategy.clone())));
        
        // Start worker threads
        let mut worker_handles = Vec::new();
        for worker_id in 0..self.config.max_workers {
            let distributor = Arc::clone(&task_distributor);
            let sender = result_sender.clone();
            let config = self.config.clone();
            let cache = Arc::clone(&self.compilation_cache);
            
            let handle = thread::spawn(move || {
                Self::worker_thread(worker_id, distributor, sender, config, cache);
            });
            worker_handles.push(handle);
        // Collect results
        drop(result_sender); // Close sender in main thread
        let mut results = Vec::new();
        while let Ok(result) = result_receiver.recv() {
            results.push(result);
        // Wait for all workers to complete
        for handle in worker_handles {
            if let Err(e) = handle.join() {
                error!("Worker thread panicked: {:?}", e);
            }
        }
        
        Ok(results)
    /// Enhanced worker thread implementation with detailed metrics
    fn worker_thread(
    ) {
        debug!("Worker {} started", worker_id);
        let worker_start_time = Instant::now();
        let mut stats = WorkerStatistics {
        
        let mut idle_start: Option<Instant> = None;
        let mut total_idle_time = Duration::default();
        
        loop {
            let task_request_start = Instant::now();
            
            // Get next task from distributor
            let task = {
                let mut distributor = task_distributor.lock().unwrap();
                distributor.get_next_task(worker_id)
            
            match task {
                Some(compilation_task) => {
                    // Track idle time
                    if let Some(idle_start_time) = idle_start.take() {
                        total_idle_time += idle_start_time.elapsed();
                    let task_start_time = Instant::now();
                    
                    // Check cache first
                    let cache_result = {
                        let cache = cache.lock().unwrap();
                        cache.get_cached_result(&compilation_task.id)
                    
                    let result = if let Some(cached) = cache_result {
                        // Cache hit
                        stats.cache_hits += 1;
                        debug!("Worker {} using cached result for task {}", worker_id, compilation_task.id);
                        
                        WorkerResult {
                            duration: Duration::from_millis(1), // Minimal cache lookup time
                        }
                    } else {
                        // Perform actual compilation with enhanced monitoring
                        Self::compile_task_enhanced(worker_id, &compilation_task, &config)
                    
                    let task_duration = task_start_time.elapsed();
                    stats.tasks_completed += 1;
                    stats.total_compilation_time += task_duration;
                    stats.average_task_time = stats.total_compilation_time / stats.tasks_completed as u32;
                    stats.memory_peak = stats.memory_peak.max(result.memory_peak);
                    
                    // Cache successful results
                    if result.success {
                        let mut cache = cache.lock().unwrap();
                        cache.cache_result(&compilation_task, &result);
                    if let Err(e) = result_sender.send(result) {
                        error!("Failed to send worker result: {}", e);
                        break;
                    }
                }
                None => {
                    // No more tasks available - start idle tracking
                    if idle_start.is_none() {
                        idle_start = Some(Instant::now());
                    // Check if we should continue waiting or exit
                    thread::sleep(Duration::from_millis(10));
                    
                    // Exit if idle too long (indicating no more work)
                    if let Some(idle_start_time) = idle_start {
                        if idle_start_time.elapsed() > Duration::from_millis(100) {
                            total_idle_time += idle_start_time.elapsed();
                            debug!("Worker {} finished - no more tasks after idle period", worker_id);
                            break;
                        }
                    }
                }
            }
        // Calculate final efficiency score
        let total_worker_time = worker_start_time.elapsed();
        let active_time = total_worker_time.saturating_sub(total_idle_time);
        stats.idle_time = total_idle_time;
        stats.efficiency_score = if total_worker_time.as_millis() > 0 {
            active_time.as_millis() as f64 / total_worker_time.as_millis() as f64
        } else {
            0.0
        
        debug!(
            total_idle_time
        );
    /// Compile individual task with basic monitoring
    fn compile_task(
    ) -> WorkerResult {
        debug!("Worker {} compiling task: {}", worker_id, task.id);
        let start_time = Instant::now();
        
        // Simulate compilation process
        // In real implementation, this would call the CURSED compiler
        let success = true; // Placeholder
        let duration = start_time.elapsed();
        
        WorkerResult {
            memory_peak: 128 * 1024 * 1024, // 128MB placeholder
        }
    }
    
    /// Enhanced compilation task with detailed monitoring
    fn compile_task_enhanced(
    ) -> WorkerResult {
        debug!("Worker {} compiling task with enhanced monitoring: {}", worker_id, task.id);
        let start_time = Instant::now();
        
        // Start memory monitoring for this specific task
        let memory_monitor = Self::start_task_memory_monitoring();
        
        // Simulate enhanced compilation process
        // In real implementation, this would:
        // 1. Set up compilation environment
        // 2. Execute CURSED compiler with detailed logging
        // 3. Monitor resource usage throughout compilation
        // 4. Collect compilation metrics and warnings
        
        let compilation_success = Self::simulate_enhanced_compilation(task, config);
        let compilation_duration = start_time.elapsed();
        
        // Stop memory monitoring and get peak usage
        let peak_memory = Self::stop_task_memory_monitoring(memory_monitor);
        
        // Collect compilation warnings and errors (simulated)
        let (warnings, errors) = Self::collect_compilation_messages(task, compilation_success);
        
        // Determine output files based on target type
        let output_files = Self::determine_output_files(task);
        
        WorkerResult {
        }
    }
    
    /// Simulate enhanced compilation with realistic behavior
    fn simulate_enhanced_compilation(task: &CompilationTask, config: &ParallelCompilationConfig) -> bool {
        // Simulate compilation time based on task complexity
        let base_time = match task.target.target_type {
        
        // Add variability based on estimated duration
        let actual_time = base_time + task.estimated_duration / 10;
        thread::sleep(actual_time);
        
        // Simulate occasional compilation failures (5% failure rate)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        task.id.hash(&mut hasher);
        let hash_value = hasher.finish();
        
        (hash_value % 20) != 0 // 95% success rate
    /// Start memory monitoring for a specific task
    fn start_task_memory_monitoring() -> TaskMemoryMonitor {
        TaskMemoryMonitor {
        }
    }
    
    /// Stop memory monitoring and return peak usage
    fn stop_task_memory_monitoring(monitor: TaskMemoryMonitor) -> usize {
        let current_memory = Self::get_current_memory_usage();
        let memory_delta = current_memory.saturating_sub(monitor.initial_memory);
        
        // Add some realistic variation
        let base_usage = 64 * 1024 * 1024; // 64MB base
        base_usage + memory_delta
    /// Get current process memory usage
    fn get_current_memory_usage() -> usize {
        use sysinfo::{System, Process, Pid};
        
        let mut sys = System::new();
        sys.refresh_processes();
        
        let current_pid = Pid::from(std::process::id() as usize);
        if let Some(process) = sys.process(current_pid) {
            process.memory() as usize
        } else {
            0
        }
    }
    
    /// Collect compilation messages (warnings and errors)
    fn collect_compilation_messages(task: &CompilationTask, success: bool) -> (Vec<String>, Vec<String>) {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        // Simulate common warnings
        if task.compilation_units.len() > 5 {
            warnings.push("Large number of compilation units may slow build".to_string());
        if matches!(task.priority, TaskPriority::Low) {
            warnings.push("Low priority task - consider increasing priority for critical path".to_string());
        // Simulate errors for failed compilations
        if !success {
            errors.push(format!("Compilation failed for target {}", task.id));
            errors.push("Syntax error or missing dependency".to_string());
        (warnings, errors)
    /// Determine output files based on task configuration
    fn determine_output_files(task: &CompilationTask) -> Vec<PathBuf> {
        let mut output_files = Vec::new();
        
        // Primary output file
        let primary_output = match task.target.target_type {
            crate::build_system::TargetType::Bin => {
                task.target.path.with_extension("exe")
            }
            crate::build_system::TargetType::Lib |
            crate::build_system::TargetType::StaticLib => {
                task.target.path.with_extension("a")
            }
            crate::build_system::TargetType::DynLib |
            crate::build_system::TargetType::CDynLib => {
                task.target.path.with_extension("so")
            }
        output_files.push(primary_output);
        
        // Debug information file
        if task.profile.debug {
            output_files.push(task.target.path.with_extension("pdb"));
        // Object files for each compilation unit
        for unit in &task.compilation_units {
            output_files.push(unit.output_file.clone());
        output_files
    /// Analyze compilation results and generate performance report
    fn analyze_compilation_results(
    ) -> crate::error::Result<()> {
        let tasks_completed = results.iter().filter(|r| r.success).count();
        let tasks_failed = results.iter().filter(|r| !r.success).count();
        
        // Calculate cached tasks from worker statistics
        let tasks_cached = self.calculate_cached_tasks(&results)?;
        
        // Calculate parallel efficiency
        let total_work_time: Duration = results.iter().map(|r| r.duration).sum();
        let parallel_efficiency = if total_duration.as_millis() > 0 {
            (total_work_time.as_millis() as f64) / 
            (total_duration.as_millis() as f64 * self.config.max_workers as f64)
        } else {
            0.0
        
        // Analyze bottlenecks
        let bottlenecks = self.analyze_bottlenecks(&results, &resource_stats);
        
        // Generate worker statistics
        let worker_statistics = self.generate_worker_statistics(&results);
        
        Ok(ParallelCompilationResult {
            resource_utilization: ResourceUtilization {
            scheduler_statistics: SchedulerStatistics {
            cache_statistics: CacheStatistics {
        })
    /// Analyze performance bottlenecks
    fn analyze_bottlenecks(
    ) -> Vec<BottleneckAnalysis> {
        let mut bottlenecks = Vec::new();
        
        // CPU bottleneck analysis
        if resource_stats.cpu_usage > 95.0 {
            bottlenecks.push(BottleneckAnalysis {
                recommendations: vec![
            });
        // Memory bottleneck analysis
        if resource_stats.memory_usage > (resource_stats.available_memory * 90 / 100) {
            bottlenecks.push(BottleneckAnalysis {
                recommendations: vec![
            });
        // Load imbalance analysis
        let task_times: Vec<Duration> = results.iter().map(|r| r.duration).collect();
        if let (Some(min), Some(max)) = (task_times.iter().min(), task_times.iter().max()) {
            let imbalance_ratio = max.as_millis() as f64 / min.as_millis() as f64;
            if imbalance_ratio > 2.0 {
                bottlenecks.push(BottleneckAnalysis {
                    recommendations: vec![
                });
            }
        }
        
        bottlenecks
    /// Generate per-worker statistics
    fn generate_worker_statistics(&self, results: &[WorkerResult]) -> Vec<WorkerStatistics> {
        let mut worker_stats: HashMap<usize, WorkerStatistics> = HashMap::new();
        
        for result in results {
            let stats = worker_stats.entry(result.worker_id).or_insert(WorkerStatistics {
            });
            
            stats.tasks_completed += 1;
            stats.total_compilation_time += result.duration;
            stats.average_task_time = stats.total_compilation_time / stats.tasks_completed as u32;
            stats.memory_peak = stats.memory_peak.max(result.memory_peak);
        // Calculate efficiency scores
        for stats in worker_stats.values_mut() {
            stats.efficiency_score = if stats.tasks_completed > 0 {
                1.0 / (stats.average_task_time.as_millis() as f64 / 1000.0)
            } else {
                0.0
        worker_stats.into_values().collect()
    /// Calculate number of cached tasks from worker results
    fn calculate_cached_tasks(&self, results: &[WorkerResult]) -> crate::error::Result<()> {
        let mut cached_count = 0;
        
        for result in results {
            // Tasks with very short duration (< 10ms) are likely cache hits
            if result.duration < Duration::from_millis(10) && result.success {
                cached_count += 1;
            }
        }
        
        Ok(cached_count)
    /// Calculate actual queue wait time from worker results
    fn calculate_actual_queue_wait_time(&self, results: &[WorkerResult]) -> Duration {
        // Calculate based on task distribution and worker utilization
        let total_tasks = results.len();
        let worker_count = self.config.max_workers;
        
        if total_tasks == 0 || worker_count == 0 {
            return Duration::default();
        // Estimate queue wait time based on load imbalance
        let tasks_per_worker = total_tasks as f64 / worker_count as f64;
        let ideal_completion_time = results.iter()
            .map(|r| r.duration)
            .sum::<Duration>()
            .as_millis() as f64 / worker_count as f64;
        
        // Calculate variance in task completion times
        let completion_times: Vec<u128> = results.iter()
            .map(|r| r.duration.as_millis())
            .collect();
        
        let mean_time = completion_times.iter().sum::<u128>() as f64 / completion_times.len() as f64;
        let variance = completion_times.iter()
            .map(|&time| {
                let diff = time as f64 - mean_time;
                diff * diff
            })
            .sum::<f64>() / completion_times.len() as f64;
        
        let std_deviation = variance.sqrt();
        
        // Higher variance indicates more queue waiting
        let wait_factor = (std_deviation / mean_time).min(1.0);
        let estimated_wait_ms = (wait_factor * mean_time * 0.1) as u64; // 10% of mean time as wait
        
        Duration::from_millis(estimated_wait_ms)
    /// Optimize task distribution for better load balancing
    fn optimize_task_distribution(&self, tasks: &[CompilationTask]) -> Vec<CompilationTask> {
        let mut optimized_tasks = tasks.to_vec();
        
        // Sort by estimated duration (shortest first) for better load balancing
        optimized_tasks.sort_by_key(|task| task.estimated_duration);
        
        // Group small tasks together to reduce context switching overhead
        let mut small_tasks = Vec::new();
        let mut large_tasks = Vec::new();
        
        let threshold = Duration::from_secs(5); // 5 seconds threshold
        
        for task in optimized_tasks {
            if task.estimated_duration < threshold {
                small_tasks.push(task);
            } else {
                large_tasks.push(task);
            }
        }
        
        // Interleave small and large tasks for optimal scheduling
        let mut result = Vec::new();
        let mut small_iter = small_tasks.into_iter();
        let mut large_iter = large_tasks.into_iter();
        
        loop {
            match (large_iter.next(), small_iter.next()) {
                (Some(large), Some(small)) => {
                    result.push(large);
                    result.push(small);
                }
            }
        }
        
        result
    /// Analyze parallel compilation efficiency
    fn analyze_parallel_efficiency(&self, results: &[WorkerResult], total_duration: Duration) -> ParallelEfficiencyAnalysis {
        let total_work_time: Duration = results.iter().map(|r| r.duration).sum();
        let ideal_parallel_time = total_work_time.as_millis() as f64 / self.config.max_workers as f64;
        
        let efficiency = if total_duration.as_millis() > 0 {
            ideal_parallel_time / total_duration.as_millis() as f64
        } else {
            0.0
        
        // Calculate load balance score
        let completion_times: Vec<u128> = results.iter().map(|r| r.duration.as_millis()).collect();
        let max_time = completion_times.iter().max().copied().unwrap_or(0) as f64;
        let min_time = completion_times.iter().min().copied().unwrap_or(0) as f64;
        
        let load_balance_score = if max_time > 0.0 {
            min_time / max_time
        } else {
            1.0
        
        // Calculate worker utilization
        let worker_utilizations = self.calculate_worker_utilizations(&results);
        let avg_utilization = worker_utilizations.iter().sum::<f64>() / worker_utilizations.len() as f64;
        
        ParallelEfficiencyAnalysis {
        }
    }
    
    /// Calculate worker utilizations
    fn calculate_worker_utilizations(&self, results: &[WorkerResult]) -> Vec<f64> {
        let mut worker_times: HashMap<usize, Duration> = HashMap::new();
        
        for result in results {
            *worker_times.entry(result.worker_id).or_insert(Duration::default()) += result.duration;
        let max_time = worker_times.values().max().copied().unwrap_or(Duration::default());
        
        worker_times.values()
            .map(|&time| {
                if max_time.as_millis() > 0 {
                    time.as_millis() as f64 / max_time.as_millis() as f64
                } else {
                    0.0
                }
            })
            .collect()
    /// Calculate potential speedup with better parallelization
    fn calculate_potential_speedup(&self, results: &[WorkerResult]) -> f64 {
        let total_work = results.iter().map(|r| r.duration.as_millis()).sum::<u128>() as f64;
        let critical_path = self.calculate_critical_path_duration(results);
        
        if critical_path > 0.0 {
            total_work / critical_path
        } else {
            1.0
        }
    }
    
    /// Calculate critical path duration
    fn calculate_critical_path_duration(&self, results: &[WorkerResult]) -> f64 {
        // For simplicity, assume critical path is the longest single task duration
        // In reality, this would involve dependency analysis
        results.iter()
            .map(|r| r.duration.as_millis())
            .max()
            .unwrap_or(0) as f64
    /// Generate scalability recommendations
    fn generate_scalability_recommendations(&self, efficiency: f64, load_balance: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if efficiency < 0.7 {
            recommendations.push("Parallel efficiency is low - consider reducing worker count or improving task granularity".to_string());
        if load_balance < 0.8 {
            recommendations.push("Load imbalance detected - consider work-stealing scheduler or better task distribution".to_string());
        if efficiency > 0.9 && self.config.max_workers < num_cpus::get() {
            recommendations.push("High efficiency achieved - consider increasing worker count for better performance".to_string());
        recommendations
    }
}

impl WorkerPool {
    fn new(config: &ParallelCompilationConfig) -> crate::error::Result<()> {
        let (completion_sender, completion_receiver) = mpsc::channel();
        let task_distributor = Arc::new(Mutex::new(TaskDistributor::new(Vec::new(), config.scheduling_strategy.clone())));
        let active_workers = Arc::new(Mutex::new(0));
        
        Ok(WorkerPool {
        })
    }
}

impl TaskScheduler {
    fn new() -> Self {
        TaskScheduler {
            scheduler_stats: Arc::new(Mutex::new(SchedulerStatistics {
        }
    }
    
    fn schedule_tasks(&mut self, tasks: Vec<CompilationTask>) -> crate::error::Result<()> {
        // Build dependency graph
        for task in &tasks {
            self.dependency_graph.add_node(&task.id, task.dependencies.clone());
        // Topological sort for dependency-aware scheduling
        let scheduled_order = self.dependency_graph.topological_sort()
            .map_err(|e| BuildError::DependencyError(e.to_string()))?;
        
        // Return tasks in dependency-respecting order
        let mut scheduled_tasks = Vec::new();
        for task_id in scheduled_order {
            if let Some(task) = tasks.iter().find(|t| t.id == task_id) {
                scheduled_tasks.push(task.clone());
            }
        }
        
        Ok(scheduled_tasks)
    }
}

impl TaskDistributor {
    fn new(tasks: Vec<CompilationTask>, strategy: SchedulingStrategy) -> Self {
        TaskDistributor {
        }
    }
    
    fn get_next_task(&mut self, worker_id: usize) -> Option<CompilationTask> {
        match self.strategy {
            SchedulingStrategy::ShortestFirst => {
                // Find task with shortest estimated duration
                let (min_index, _) = self.pending_tasks
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, task)| task.estimated_duration)?;
                self.pending_tasks.remove(min_index)
            }
            SchedulingStrategy::CriticalPath => {
                // Find task on critical path using dependency analysis
                self.get_critical_path_task(worker_id)
            }
        }
    }
    
    /// Get task on critical path for optimization
    fn get_critical_path_task(&mut self, worker_id: usize) -> Option<CompilationTask> {
        if self.pending_tasks.is_empty() {
            return None;
        // Calculate critical path for each pending task
        let mut task_priorities: Vec<(usize, Duration)> = Vec::new();
        
        for (index, task) in self.pending_tasks.iter().enumerate() {
            let critical_path_duration = self.calculate_critical_path_duration(task);
            task_priorities.push((index, critical_path_duration));
        // Sort by critical path duration (longest first)
        task_priorities.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Return task with longest critical path
        if let Some((index, _)) = task_priorities.first() {
            self.pending_tasks.remove(*index)
        } else {
            self.pending_tasks.pop_front()
        }
    }
    
    /// Calculate critical path duration for a task
    fn calculate_critical_path_duration(&self, task: &CompilationTask) -> Duration {
        let mut total_duration = task.estimated_duration;
        
        // Add estimated duration of dependencies
        for dep_id in &task.dependencies {
            if let Some(dep_task) = self.pending_tasks.iter().find(|t| t.id == *dep_id) {
                total_duration += dep_task.estimated_duration;
            }
        }
        
        total_duration
    }
}

impl ResourceMonitor {
    fn new(config: &ParallelCompilationConfig) -> crate::error::Result<()> {
        let (alert_sender, _alert_receiver) = mpsc::channel();
        
        Ok(ResourceMonitor {
            resource_stats: Arc::new(Mutex::new(ResourceStatistics {
                available_memory: 8 * 1024 * 1024 * 1024, // 8GB placeholder
            thresholds: ResourceThresholds {
                max_disk_io: 1000, // MB/s
        })
    fn start_monitoring(&mut self) -> crate::error::Result<()> {
        if self.monitor_thread.is_some() {
            return Ok(()); // Already monitoring
        let stats = Arc::clone(&self.resource_stats);
        let handle = thread::spawn(move || {
            use sysinfo::{System, Process, Pid};
            let mut sys = System::new_all();
            let current_pid = Pid::from(std::process::id() as usize);
            
            loop {
                sys.refresh_all();
                
                let mut current_stats = match stats.lock() {
                
                // Update CPU usage
                current_stats.cpu_usage = sys.global_processor_info().cpu_usage() as f64;
                current_stats.load_average = sys.load_average().one;
                
                // Update memory usage
                if let Some(process) = sys.process(current_pid) {
                    current_stats.memory_usage = process.memory() as usize;
                }
                current_stats.available_memory = sys.available_memory() as usize;
                
                // Update disk I/O (simplified)
                current_stats.disk_io_read = 0; // Would need platform-specific implementation
                current_stats.disk_io_write = 0;
                
                drop(current_stats);
                thread::sleep(Duration::from_millis(100));
            }
        });
        
        self.monitor_thread = Some(handle);
        Ok(())
    fn stop_monitoring(&mut self) -> crate::error::Result<()> {
        if let Some(handle) = self.monitor_thread.take() {
            // In real implementation, signal the thread to stop
            // For now, just take the current stats
        let stats = self.resource_stats.lock().unwrap().clone();
        Ok(stats)
    }
}

impl CompilationCache {
    fn new() -> Self {
        CompilationCache {
            cache_stats: CacheStatistics {
        }
    }
    
    fn get_cached_result(&self, task_id: &str) -> Option<&CachedCompilationResult> {
        self.compilation_results.get(task_id)
    fn cache_result(&mut self, task: &CompilationTask, result: &WorkerResult) {
        let checksum = self.calculate_result_checksum(task, result);
        
        let cached_result = CachedCompilationResult {
        
        self.compilation_results.insert(task.id.clone(), cached_result);
    /// Calculate checksum for compilation result
    fn calculate_result_checksum(&self, task: &CompilationTask, result: &WorkerResult) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        
        // Hash task properties
        hasher.update(task.id.as_bytes());
        hasher.update(task.target.path.to_string_lossy().as_bytes());
        
        // Hash compilation flags
        for flag in &task.compilation_units {
            hasher.update(flag.source_file.to_string_lossy().as_bytes());
            hasher.update(flag.optimization_level.as_bytes());
        // Hash output files
        for output_file in &result.output_files {
            if let Ok(content) = std::fs::read(output_file) {
                hasher.update(&content);
            }
        }
        
        // Hash compilation duration (for cache invalidation on performance changes)
        hasher.update(&result.duration.as_millis().to_le_bytes());
        
        format!("{:x}", hasher.finalize())
    }
}

