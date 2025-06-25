// Parallel compilation support for improved build performance

use crate::error::{CursedError, Result};
use crate::optimization::metrics::CompilationUnit;
use crate::optimization::dependency_analyzer::{DependencyGraph, CompilationPlan};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{info, debug, warn, error, instrument};
use serde::{Deserialize, Serialize};

/// Configuration for parallel compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCompilationConfig {
    pub enable_parallel: bool,
    pub max_parallel_jobs: Option<usize>,
    pub job_scheduling_strategy: SchedulingStrategy,
    pub load_balancing_enabled: bool,
    pub thread_affinity_enabled: bool,
    pub compilation_timeout_secs: Option<u64>,
}

impl Default for ParallelCompilationConfig {
    fn default() -> Self {
        Self {
            enable_parallel: true,
            max_parallel_jobs: None, // Use all available cores
            job_scheduling_strategy: SchedulingStrategy::WorkStealing,
            load_balancing_enabled: true,
            thread_affinity_enabled: false,
            compilation_timeout_secs: Some(300), // 5 minutes per unit
        }
    }
}

/// Job scheduling strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SchedulingStrategy {
    /// Simple round-robin assignment
    RoundRobin,
    /// Work-stealing queue based scheduling
    WorkStealing,
    /// Longest job first
    LongestJobFirst,
    /// Critical path first
    CriticalPathFirst,
}

/// Parallel compilation coordinator
#[derive(Debug)]
pub struct ParallelCompiler {
    config: ParallelCompilationConfig,
    worker_threads: Vec<WorkerThread>,
    job_queue: Arc<Mutex<JobQueue>>,
    statistics: ParallelCompilationStatistics,
}

/// Worker thread for parallel compilation
#[derive(Debug)]
struct WorkerThread {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
    status: WorkerStatus,
}

/// Status of a worker thread
#[derive(Debug, Clone, PartialEq, Eq)]
enum WorkerStatus {
    Idle,
    Working,
    Finished,
    CursedError,
}

/// Job queue for managing compilation tasks
#[derive(Debug)]
struct JobQueue {
    pending_jobs: Vec<CompilationJob>,
    in_progress_jobs: HashMap<usize, CompilationJob>,
    completed_jobs: Vec<CompilationResult>,
    failed_jobs: Vec<CompilationResult>,
}

/// A compilation job for a single unit
#[derive(Debug, Clone)]
struct CompilationJob {
    id: usize,
    unit: CompilationUnit,
    priority: JobPriority,
    estimated_time: Duration,
    dependencies: Vec<usize>,
}

/// Priority levels for compilation jobs
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum JobPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Result of a compilation job
#[derive(Debug, Clone)]
struct CompilationResult {
    job_id: usize,
    unit_name: String,
    success: bool,
    compilation_time: Duration,
    worker_id: usize,
    error_message: Option<String>,
    output_size: usize,
}

/// Statistics for parallel compilation
#[derive(Debug, Default, Clone)]
pub struct ParallelCompilationStatistics {
    pub total_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub total_compilation_time: Duration,
    pub parallel_efficiency: f64,
    pub worker_utilization: HashMap<usize, f64>,
    pub average_job_time: Duration,
    pub peak_memory_usage_mb: f64,
}

/// Result of parallel compilation
#[derive(Debug)]
pub struct ParallelCompilationResult {
    pub successful_units: Vec<String>,
    pub failed_units: Vec<String>,
    pub total_time: Duration,
    pub parallel_efficiency: f64,
    pub jobs_per_second: f64,
    pub statistics: ParallelCompilationStatistics,
}

impl ParallelCompiler {
    /// Create a new parallel compiler
    #[instrument]
    pub fn new(config: ParallelCompilationConfig) -> Result<Self> {
        info!("Creating parallel compiler with {} max jobs", 
            config.max_parallel_jobs.unwrap_or_else(num_cpus::get));

        let job_queue = Arc::new(Mutex::new(JobQueue::new()));
        let worker_count = config.effective_worker_count();
        let mut worker_threads = Vec::with_capacity(worker_count);

        // Create worker threads
        for id in 0..worker_count {
            worker_threads.push(WorkerThread {
                id,
                handle: None,
                status: WorkerStatus::Idle,
            });
        }

        Ok(Self {
            config,
            worker_threads,
            job_queue,
            statistics: ParallelCompilationStatistics::default(),
        })
    }

    /// Compile units in parallel
    #[instrument(skip(self, units, dependency_graph))]
    pub fn compile_parallel(
        &mut self,
        units: &mut [CompilationUnit],
        dependency_graph: Option<&DependencyGraph>,
    ) -> Result<ParallelCompilationResult> {
        if !self.config.enable_parallel || units.len() <= 1 {
            return self.compile_sequential(units);
        }

        info!("Starting parallel compilation of {} units", units.len());
        let start_time = Instant::now();

        // Create compilation plan
        let compilation_plan = self.create_compilation_plan(units, dependency_graph)?;
        
        // Prepare job queue
        self.prepare_job_queue(&compilation_plan)?;

        // Start worker threads
        self.start_worker_threads()?;

        // Wait for completion
        let result = self.wait_for_completion(start_time)?;

        // Apply results back to units
        self.apply_results_to_units(units)?;

        info!(
            "Parallel compilation completed: {} successful, {} failed in {:.2?}",
            result.successful_units.len(),
            result.failed_units.len(),
            result.total_time
        );

        Ok(result)
    }

    /// Create compilation plan considering dependencies
    #[instrument(skip(self, units, dependency_graph))]
    fn create_compilation_plan(
        &self,
        units: &[CompilationUnit],
        dependency_graph: Option<&DependencyGraph>,
    ) -> Result<CompilationPlan> {
        if let Some(graph) = dependency_graph {
            // Use dependency graph to create optimal plan
            graph.find_optimal_compilation_order(self.config.effective_worker_count())
        } else {
            // Create simple parallel plan without dependencies
            let batches = self.create_simple_batches(units);
            Ok(CompilationPlan {
                batches,
                critical_path: Vec::new(),
                estimated_total_time: Duration::from_secs(units.len() as u64),
                max_parallelism: self.config.effective_worker_count(),
            })
        }
    }

    /// Create simple batches for units without dependency information
    fn create_simple_batches(&self, units: &[CompilationUnit]) -> Vec<Vec<String>> {
        let worker_count = self.config.effective_worker_count();
        let mut batches = vec![Vec::new(); worker_count];
        
        // Distribute units across workers using round-robin
        for (i, unit) in units.iter().enumerate() {
            batches[i % worker_count].push(unit.name.clone());
        }

        // Remove empty batches
        batches.into_iter().filter(|batch| !batch.is_empty()).collect()
    }

    /// Prepare the job queue with compilation jobs
    #[instrument(skip(self, plan))]
    fn prepare_job_queue(&mut self, plan: &CompilationPlan) -> Result<()> {
        let mut queue = self.job_queue.lock().map_err(|_| {
            CursedError::optimization_error("Failed to acquire job queue lock")
        })?;

        let mut job_id = 0;
        
        for batch in &plan.batches {
            for unit_name in batch {
                // Find the actual unit (simplified lookup)
                let unit = CompilationUnit::new(unit_name.clone());
                
                let priority = if plan.critical_path.contains(unit_name) {
                    JobPriority::Critical
                } else {
                    JobPriority::Medium
                };

                let estimated_time = self.estimate_compilation_time(&unit);

                queue.pending_jobs.push(CompilationJob {
                    id: job_id,
                    unit,
                    priority,
                    estimated_time,
                    dependencies: Vec::new(), // Simplified
                });
                
                job_id += 1;
            }
        }

        // Sort jobs by priority and estimated time
        queue.pending_jobs.sort_by(|a, b| {
            b.priority.cmp(&a.priority)
                .then_with(|| match self.config.job_scheduling_strategy {
                    SchedulingStrategy::LongestJobFirst => b.estimated_time.cmp(&a.estimated_time),
                    _ => a.estimated_time.cmp(&b.estimated_time),
                })
        });

        self.statistics.total_jobs = queue.pending_jobs.len();
        debug!("Prepared {} compilation jobs", queue.pending_jobs.len());

        Ok(())
    }

    /// Start worker threads
    #[instrument(skip(self))]
    fn start_worker_threads(&mut self) -> Result<()> {
        debug!("Starting {} worker threads", self.worker_threads.len());

        for worker in &mut self.worker_threads {
            let queue = Arc::clone(&self.job_queue);
            let worker_id = worker.id;
            let timeout = self.config.compilation_timeout_secs.map(Duration::from_secs);

            let handle = thread::spawn(move || {
                Self::worker_thread_main(worker_id, queue, timeout);
            });

            worker.handle = Some(handle);
            worker.status = WorkerStatus::Working;
        }

        Ok(())
    }

    /// Main function for worker threads
    fn worker_thread_main(
        worker_id: usize,
        job_queue: Arc<Mutex<JobQueue>>,
        timeout: Option<Duration>,
    ) {
        debug!("Worker {} started", worker_id);

        loop {
            // Try to get a job from the queue
            let job = {
                let mut queue = match job_queue.lock() {
                    Ok(queue) => queue,
                    Err(_) => {
                        error!("Worker {} failed to acquire queue lock", worker_id);
                        break;
                    }
                };

                if let Some(job) = queue.get_next_job() {
                    queue.in_progress_jobs.insert(job.id, job.clone());
                    Some(job)
                } else {
                    None
                }
            };

            match job {
                Some(job) => {
                    // Process the job
                    let result = Self::compile_job(worker_id, job, timeout);
                    
                    // Store result
                    let mut queue = match job_queue.lock() {
                        Ok(queue) => queue,
                        Err(_) => {
                            error!("Worker {} failed to acquire queue lock for result", worker_id);
                            break;
                        }
                    };

                    queue.in_progress_jobs.remove(&result.job_id);
                    
                    if result.success {
                        queue.completed_jobs.push(result);
                    } else {
                        queue.failed_jobs.push(result);
                    }
                }
                None => {
                    // No more jobs, check if we should exit
                    let queue = match job_queue.lock() {
                        Ok(queue) => queue,
                        Err(_) => break,
                    };

                    if queue.pending_jobs.is_empty() && queue.in_progress_jobs.is_empty() {
                        break; // All jobs completed
                    }

                    // Wait a bit before checking again
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }

        debug!("Worker {} finished", worker_id);
    }

    /// Compile a single job
    fn compile_job(
        worker_id: usize,
        job: CompilationJob,
        timeout: Option<Duration>,
    ) -> CompilationResult {
        let start_time = Instant::now();
        
        debug!("Worker {} compiling unit: {}", worker_id, job.unit.name);

        // Check timeout before starting
        if let Some(timeout_duration) = timeout {
            if job.estimated_time > timeout_duration {
                warn!("Job {} estimated time exceeds timeout, skipping", job.id);
                return CompilationResult {
                    job_id: job.id,
                    unit_name: job.unit.name,
                    success: false,
                    compilation_time: start_time.elapsed(),
                    worker_id,
                    error_message: Some("Compilation timeout (pre-check)".to_string()),
                    output_size: 0,
                };
            }
        }

        // Perform actual compilation work (simulated with realistic steps)
        let mut success = true;
        let mut error_message = None;
        let mut output_size = job.unit.estimated_size_bytes;
        
        // Step 1: Parse and validate source files
        let parse_time = job.estimated_time / 4;
        thread::sleep(parse_time.min(Duration::from_millis(25)));
        
        if start_time.elapsed() > timeout.unwrap_or(Duration::from_secs(u64::MAX)) {
            success = false;
            error_message = Some("Timeout during parsing".to_string());
        }
        
        // Step 2: Type checking and semantic analysis
        if success {
            let typecheck_time = job.estimated_time / 4;
            thread::sleep(typecheck_time.min(Duration::from_millis(25)));
            
            if start_time.elapsed() > timeout.unwrap_or(Duration::from_secs(u64::MAX)) {
                success = false;
                error_message = Some("Timeout during type checking".to_string());
            }
        }
        
        // Step 3: LLVM IR generation
        if success {
            let codegen_time = job.estimated_time / 4;
            thread::sleep(codegen_time.min(Duration::from_millis(25)));
            
            if start_time.elapsed() > timeout.unwrap_or(Duration::from_secs(u64::MAX)) {
                success = false;
                error_message = Some("Timeout during code generation".to_string());
            }
        }
        
        // Step 4: Optimization
        if success {
            let optimization_time = job.estimated_time / 4;
            thread::sleep(optimization_time.min(Duration::from_millis(25)));
            
            if start_time.elapsed() > timeout.unwrap_or(Duration::from_secs(u64::MAX)) {
                success = false;
                error_message = Some("Timeout during optimization".to_string());
            } else {
                // Simulate optimization reducing output size
                output_size = (output_size as f64 * 0.85) as usize;
            }
        }
        
        // Simulate occasional compilation failures based on complexity
        if success {
            let failure_probability = match job.priority {
                JobPriority::Critical => 0.01, // 1% failure rate for critical jobs
                JobPriority::High => 0.02,     // 2% failure rate for high priority
                JobPriority::Medium => 0.03,   // 3% failure rate for medium priority
                JobPriority::Low => 0.05,      // 5% failure rate for low priority
            };
            
            if rand::random::<f64>() < failure_probability {
                success = false;
                error_message = Some(format!("Simulated compilation error (priority: {:?})", job.priority));
            }
        }

        let actual_time = start_time.elapsed();

        CompilationResult {
            job_id: job.id,
            unit_name: job.unit.name,
            success,
            compilation_time: actual_time,
            worker_id,
            error_message,
            output_size,
        }
    }

    /// Wait for all compilation to complete
    #[instrument(skip(self))]
    fn wait_for_completion(&mut self, start_time: Instant) -> Result<ParallelCompilationResult> {
        // Wait for all worker threads to finish
        for worker in &mut self.worker_threads {
            if let Some(handle) = worker.handle.take() {
                if let Err(_) = handle.join() {
                    warn!("Worker {} panicked", worker.id);
                    worker.status = WorkerStatus::CursedError;
                } else {
                    worker.status = WorkerStatus::Finished;
                }
            }
        }

        let total_time = start_time.elapsed();

        // Collect results
        let queue = self.job_queue.lock().map_err(|_| {
            CursedError::optimization_error("Failed to acquire job queue lock for results")
        })?;

        let successful_units: Vec<String> = queue.completed_jobs.iter()
            .map(|result| result.unit_name.clone())
            .collect();

        let failed_units: Vec<String> = queue.failed_jobs.iter()
            .map(|result| result.unit_name.clone())
            .collect();

        // Calculate statistics
        self.statistics.completed_jobs = queue.completed_jobs.len();
        self.statistics.failed_jobs = queue.failed_jobs.len();
        self.statistics.total_compilation_time = total_time;

        let sequential_time: Duration = queue.completed_jobs.iter()
            .map(|result| result.compilation_time)
            .sum();

        self.statistics.parallel_efficiency = if total_time.as_secs_f64() > 0.0 {
            sequential_time.as_secs_f64() / (total_time.as_secs_f64() * self.worker_threads.len() as f64)
        } else {
            0.0
        };

        if !queue.completed_jobs.is_empty() {
            self.statistics.average_job_time = sequential_time / queue.completed_jobs.len() as u32;
        }

        let jobs_per_second = if total_time.as_secs_f64() > 0.0 {
            queue.completed_jobs.len() as f64 / total_time.as_secs_f64()
        } else {
            0.0
        };

        Ok(ParallelCompilationResult {
            successful_units,
            failed_units,
            total_time,
            parallel_efficiency: self.statistics.parallel_efficiency,
            jobs_per_second,
            statistics: self.statistics.clone(),
        })
    }

    /// Apply compilation results back to units
    fn apply_results_to_units(&self, _units: &mut [CompilationUnit]) -> Result<()> {
        // In a real implementation, would apply compilation artifacts back to units
        Ok(())
    }

    /// Compile units sequentially (fallback)
    fn compile_sequential(&self, units: &mut [CompilationUnit]) -> Result<ParallelCompilationResult> {
        info!("Compiling {} units sequentially", units.len());
        let start_time = Instant::now();

        let mut successful_units = Vec::new();
        let mut failed_units = Vec::new();

        for unit in units.iter_mut() {
            let unit_start = Instant::now();
            
            // Simulate compilation
            let estimated_time = self.estimate_compilation_time(unit);
            thread::sleep(estimated_time.min(Duration::from_millis(50)));

            let success = true; // Simplified
            if success {
                successful_units.push(unit.name.clone());
            } else {
                failed_units.push(unit.name.clone());
            }
        }

        let total_time = start_time.elapsed();

        Ok(ParallelCompilationResult {
            successful_units,
            failed_units,
            total_time,
            parallel_efficiency: 1.0, // Sequential is 100% efficient by definition
            jobs_per_second: units.len() as f64 / total_time.as_secs_f64(),
            statistics: ParallelCompilationStatistics {
                total_jobs: units.len(),
                completed_jobs: successful_units.len(),
                failed_jobs: failed_units.len(),
                total_compilation_time: total_time,
                parallel_efficiency: 1.0,
                ..Default::default()
            },
        })
    }

    /// Estimate compilation time for a unit
    fn estimate_compilation_time(&self, unit: &CompilationUnit) -> Duration {
        // Base time + time per source file + time per dependency
        let base_time = Duration::from_millis(100);
        let file_time = Duration::from_millis(unit.source_files.len() as u64 * 50);
        let dep_time = Duration::from_millis(unit.dependencies.len() as u64 * 10);
        let size_time = Duration::from_millis(unit.estimated_size_bytes as u64 / 1000);

        base_time + file_time + dep_time + size_time
    }

    /// Get current statistics
    pub fn get_statistics(&self) -> &ParallelCompilationStatistics {
        &self.statistics
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: ParallelCompilationConfig) -> Result<()> {
        info!("Updating parallel compilation configuration");
        self.config = new_config;
        Ok(())
    }
}

impl ParallelCompilationConfig {
    /// Get effective worker count
    pub fn effective_worker_count(&self) -> usize {
        if !self.enable_parallel {
            return 1;
        }

        self.max_parallel_jobs.unwrap_or_else(num_cpus::get).max(1)
    }
}

impl JobQueue {
    fn new() -> Self {
        Self {
            pending_jobs: Vec::new(),
            in_progress_jobs: HashMap::new(),
            completed_jobs: Vec::new(),
            failed_jobs: Vec::new(),
        }
    }

    fn get_next_job(&mut self) -> Option<CompilationJob> {
        // Get highest priority job that has no pending dependencies
        for i in 0..self.pending_jobs.len() {
            let job = &self.pending_jobs[i];
            if self.dependencies_satisfied(job) {
                return Some(self.pending_jobs.remove(i));
            }
        }
        None
    }

    fn dependencies_satisfied(&self, job: &CompilationJob) -> bool {
        // Check if all dependencies are completed
        job.dependencies.iter().all(|&dep_id| {
            self.completed_jobs.iter().any(|result| result.job_id == dep_id)
        })
    }
}

