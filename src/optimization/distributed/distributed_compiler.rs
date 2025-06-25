// Distributed Compilation Coordinator
//
// Main coordinator for managing distributed compilation across multiple worker nodes.
// Handles job distribution, result collection, and coordination with other subsystems.

use crate::error::{CursedError, Result};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, oneshot, Semaphore};
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

/// Configuration for the distributed compiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    /// Maximum number of concurrent jobs
    pub max_concurrent_jobs: usize,
    /// Timeout for individual jobs
    pub job_timeout: Duration,
    /// Number of retry attempts for failed jobs
    pub retry_attempts: usize,
    /// Size of compilation chunks
    pub chunk_size: usize,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            max_concurrent_jobs: 32,
            job_timeout: Duration::from_secs(300),
            retry_attempts: 3,
            chunk_size: 4,
            enable_monitoring: true,
        }
    }
}

/// A compilation job that can be distributed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationJob {
    pub id: String,
    pub source_files: Vec<String>,
    pub dependencies: Vec<String>,
    pub compiler_flags: Vec<String>,
    pub target_triple: String,
    pub optimization_level: String,
    pub output_type: OutputType,
    pub priority: JobPriority,
    pub estimated_duration: Duration,
    pub created_at: SystemTime,
    pub chunk_id: Option<usize>,
    pub parent_job_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Types of compilation output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputType {
    Object,
    BitcodeIR,
    Assembly,
    Executable,
    StaticLibrary,
    DynamicLibrary,
}

/// Job priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Result of a compilation job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    pub job_id: String,
    pub worker_id: String,
    pub success: bool,
    pub output: Vec<u8>,
    pub error_message: Option<String>,
    pub warnings: Vec<String>,
    pub compilation_time: Duration,
    pub output_files: Vec<String>,
    pub cache_key: Option<String>,
    pub completed_at: SystemTime,
    pub resource_usage: ResourceUsage,
}

/// Resource usage information for a job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_time: Duration,
    pub memory_peak_mb: usize,
    pub disk_io_mb: f64,
    pub network_io_mb: f64,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_time: Duration::ZERO,
            memory_peak_mb: 0,
            disk_io_mb: 0.0,
            network_io_mb: 0.0,
        }
    }
}

/// Job execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
enum JobState {
    Pending,
    Queued,
    Assigned(String), // worker_id
    Running(String),  // worker_id
    Completed(CompilationResult),
    Failed(String),   // error message
    Cancelled,
}

/// Internal job tracking information
#[derive(Debug)]
struct JobTracker {
    job: CompilationJob,
    state: JobState,
    started_at: Option<Instant>,
    retry_count: usize,
    result_sender: Option<oneshot::Sender<CompilationResult>>,
}

/// Distributed compilation coordinator
pub struct DistributedCompiler {
    config: CompilerConfig,
    job_queue: Arc<Mutex<VecDeque<CompilationJob>>>,
    active_jobs: Arc<RwLock<HashMap<String, JobTracker>>>,
    job_semaphore: Arc<Semaphore>,
    command_sender: mpsc::UnboundedSender<CompilerCommand>,
    stats: Arc<Mutex<CompilerStats>>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
    worker_registry: Arc<RwLock<HashMap<String, super::worker_node::WorkerNode>>>,
}

/// Internal commands for the compiler
#[derive(Debug)]
enum CompilerCommand {
    SubmitJob {
        job: CompilationJob,
        result_sender: oneshot::Sender<CompilationResult>,
    },
    JobCompleted {
        result: CompilationResult,
    },
    JobFailed {
        job_id: String,
        worker_id: String,
        error: String,
    },
    WorkerRegistered {
        worker: super::worker_node::WorkerNode,
    },
    WorkerUnregistered {
        worker_id: String,
    },
    UpdateConfig {
        config: CompilerConfig,
    },
    Shutdown,
}

/// Compilation statistics
#[derive(Debug, Clone, Default)]
pub struct CompilerStats {
    pub total_jobs_submitted: usize,
    pub jobs_completed: usize,
    pub jobs_failed: usize,
    pub jobs_retried: usize,
    pub average_job_time: Duration,
    pub total_compilation_time: Duration,
    pub queue_length: usize,
    pub active_jobs_count: usize,
}

impl CompilationJob {
    /// Create a new compilation job
    pub fn new(
        source_files: Vec<String>,
        target_triple: String,
        output_type: OutputType,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_files,
            dependencies: Vec::new(),
            compiler_flags: Vec::new(),
            target_triple,
            optimization_level: "O2".to_string(),
            output_type,
            priority: JobPriority::Normal,
            estimated_duration: Duration::from_secs(30),
            created_at: SystemTime::now(),
            chunk_id: None,
            parent_job_id: None,
            metadata: HashMap::new(),
        }
    }

    /// Generate cache key for this job
    pub fn cache_key(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.source_files.hash(&mut hasher);
        self.dependencies.hash(&mut hasher);
        self.compiler_flags.hash(&mut hasher);
        self.target_triple.hash(&mut hasher);
        self.optimization_level.hash(&mut hasher);
        self.output_type.hash(&mut hasher);

        format!("compile_{:016x}", hasher.finish())
    }

    /// Split job into smaller chunks for parallel processing
    pub fn split_into_chunks(&self, chunk_size: usize) -> Vec<CompilationJob> {
        if self.source_files.len() <= chunk_size {
            return vec![self.clone()];
        }

        let mut chunks = Vec::new();
        let total_chunks = (self.source_files.len() + chunk_size - 1) / chunk_size;

        for (i, chunk_files) in self.source_files.chunks(chunk_size).enumerate() {
            let mut chunk_job = self.clone();
            chunk_job.id = format!("{}_{}", self.id, i);
            chunk_job.source_files = chunk_files.to_vec();
            chunk_job.chunk_id = Some(i);
            chunk_job.parent_job_id = Some(self.id.clone());
            chunk_job.estimated_duration = Duration::from_secs(
                self.estimated_duration.as_secs() / total_chunks as u64
            );
            chunks.push(chunk_job);
        }

        chunks
    }

    /// Estimate compilation duration based on file count and complexity
    pub fn estimate_duration(&mut self) {
        let base_time = Duration::from_secs(5); // Base compilation time
        let per_file_time = Duration::from_secs(2); // Additional time per file
        let complexity_multiplier = match self.optimization_level.as_str() {
            "O0" => 1.0,
            "O1" => 1.5,
            "O2" => 2.0,
            "O3" => 3.0,
            "Os" => 1.8,
            "Oz" => 2.2,
            _ => 2.0,
        };

        let estimated_secs = (base_time.as_secs() 
            + per_file_time.as_secs() * self.source_files.len() as u64) as f64
            * complexity_multiplier;

        self.estimated_duration = Duration::from_secs(estimated_secs as u64);
    }
}

impl DistributedCompiler {
    /// Create a new distributed compiler
    #[instrument]
    pub fn new(config: CompilerConfig) -> Result<Self> {
        let job_semaphore = Arc::new(Semaphore::new(config.max_concurrent_jobs));
        let (command_sender, command_receiver) = mpsc::unbounded_channel();

        let compiler = Self {
            config: config.clone(),
            job_queue: Arc::new(Mutex::new(VecDeque::new())),
            active_jobs: Arc::new(RwLock::new(HashMap::new())),
            job_semaphore,
            command_sender,
            stats: Arc::new(Mutex::new(CompilerStats::default())),
            is_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            worker_registry: Arc::new(RwLock::new(HashMap::new())),
        };

        // Spawn the main coordinator task
        let coordinator = compiler.clone_for_coordinator();
        tokio::spawn(async move {
            coordinator.run_coordinator(command_receiver).await;
        });

        info!("Distributed compiler initialized with {} job slots", config.max_concurrent_jobs);
        Ok(compiler)
    }

    /// Start the distributed compiler
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<()> {
        self.is_running.store(true, std::sync::atomic::Ordering::Relaxed);
        info!("Distributed compiler started");
        Ok(())
    }

    /// Stop the distributed compiler
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<()> {
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
        let _ = self.command_sender.send(CompilerCommand::Shutdown);
        info!("Distributed compiler stopped");
        Ok(())
    }

    /// Submit a compilation job
    #[instrument(skip(self, job))]
    pub async fn submit_job(&self, mut job: CompilationJob) -> Result<CompilationResult> {
        if !self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            return Err(CursedError::system_error("Compiler is not running"));
        }

        // Estimate duration if not set
        if job.estimated_duration == Duration::ZERO {
            job.estimate_duration();
        }

        let (result_sender, result_receiver) = oneshot::channel();

        // Send job to coordinator
        self.command_sender
            .send(CompilerCommand::SubmitJob { job: job.clone(), result_sender })
            .map_err(|_| CursedError::system_error("Failed to submit job to coordinator"))?;

        // Wait for result
        let result = result_receiver
            .await
            .map_err(|_| CursedError::system_error("Failed to receive compilation result"))?;

        debug!(job_id = job.id, success = result.success, "Job completed");
        Ok(result)
    }

    /// Submit multiple jobs as a batch
    #[instrument(skip(self, jobs))]
    pub async fn submit_batch(&self, jobs: Vec<CompilationJob>) -> Result<Vec<CompilationResult>> {
        let mut results = Vec::new();
        let mut handles = Vec::new();

        // Submit all jobs concurrently
        for job in jobs {
            let compiler = self.clone();
            let handle = tokio::spawn(async move {
                compiler.submit_job(job).await
            });
            handles.push(handle);
        }

        // Collect all results
        for handle in handles {
            match handle.await {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(CursedError::system_error(&format!("Task join error: {}", e))),
            }
        }

        Ok(results)
    }

    /// Get current statistics
    pub async fn get_statistics(&self) -> Result<CompilerStats> {
        let stats = self.stats.lock()
            .map_err(|_| CursedError::system_error("Failed to lock stats"))?;
        Ok(stats.clone())
    }

    /// Update configuration
    #[instrument(skip(self, config))]
    pub async fn update_config(&self, config: CompilerConfig) -> Result<()> {
        self.command_sender
            .send(CompilerCommand::UpdateConfig { config })
            .map_err(|_| CursedError::system_error("Failed to update config"))?;
        Ok(())
    }

    /// Register a worker node
    #[instrument(skip(self, worker))]
    pub async fn register_worker(&self, worker: super::worker_node::WorkerNode) -> Result<()> {
        self.command_sender
            .send(CompilerCommand::WorkerRegistered { worker })
            .map_err(|_| CursedError::system_error("Failed to register worker"))?;
        Ok(())
    }

    /// Unregister a worker node
    #[instrument(skip(self))]
    pub async fn unregister_worker(&self, worker_id: String) -> Result<()> {
        self.command_sender
            .send(CompilerCommand::WorkerUnregistered { worker_id })
            .map_err(|_| CursedError::system_error("Failed to unregister worker"))?;
        Ok(())
    }

    /// Clone for coordinator task
    fn clone_for_coordinator(&self) -> CoordinatorHandle {
        CoordinatorHandle {
            config: self.config.clone(),
            job_queue: self.job_queue.clone(),
            active_jobs: self.active_jobs.clone(),
            job_semaphore: self.job_semaphore.clone(),
            stats: self.stats.clone(),
            is_running: self.is_running.clone(),
            worker_registry: self.worker_registry.clone(),
        }
    }

    /// Internal method to handle job completion
    async fn handle_job_completion(&self, result: CompilationResult) -> Result<()> {
        self.command_sender
            .send(CompilerCommand::JobCompleted { result })
            .map_err(|_| CursedError::system_error("Failed to report job completion"))?;
        Ok(())
    }

    /// Internal method to handle job failure
    async fn handle_job_failure(&self, job_id: String, worker_id: String, error: String) -> Result<()> {
        self.command_sender
            .send(CompilerCommand::JobFailed { job_id, worker_id, error })
            .map_err(|_| CursedError::system_error("Failed to report job failure"))?;
        Ok(())
    }
}

impl Clone for DistributedCompiler {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            job_queue: self.job_queue.clone(),
            active_jobs: self.active_jobs.clone(),
            job_semaphore: self.job_semaphore.clone(),
            command_sender: self.command_sender.clone(),
            stats: self.stats.clone(),
            is_running: self.is_running.clone(),
            worker_registry: self.worker_registry.clone(),
        }
    }
}

/// Handle for the coordinator task
#[derive(Debug)]
struct CoordinatorHandle {
    config: CompilerConfig,
    job_queue: Arc<Mutex<VecDeque<CompilationJob>>>,
    active_jobs: Arc<RwLock<HashMap<String, JobTracker>>>,
    job_semaphore: Arc<Semaphore>,
    stats: Arc<Mutex<CompilerStats>>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
    worker_registry: Arc<RwLock<HashMap<String, super::worker_node::WorkerNode>>>,
}

impl CoordinatorHandle {
    /// Main coordinator loop
    async fn run_coordinator(self, mut command_receiver: mpsc::UnboundedReceiver<CompilerCommand>) {
        info!("Starting distributed compiler coordinator");

        // Start job dispatcher
        let dispatcher_handle = {
            let handle = self.clone();
            tokio::spawn(async move {
                handle.run_job_dispatcher().await;
            })
        };

        // Process commands
        while let Some(command) = command_receiver.recv().await {
            match command {
                CompilerCommand::SubmitJob { job, result_sender } => {
                    self.handle_submit_job(job, result_sender).await;
                }
                CompilerCommand::JobCompleted { result } => {
                    self.handle_job_completed(result).await;
                }
                CompilerCommand::JobFailed { job_id, worker_id, error } => {
                    self.handle_job_failed(job_id, worker_id, error).await;
                }
                CompilerCommand::WorkerRegistered { worker } => {
                    self.handle_worker_registered(worker).await;
                }
                CompilerCommand::WorkerUnregistered { worker_id } => {
                    self.handle_worker_unregistered(worker_id).await;
                }
                CompilerCommand::UpdateConfig { config } => {
                    self.handle_update_config(config).await;
                }
                CompilerCommand::Shutdown => {
                    break;
                }
            }
        }

        // Stop job dispatcher
        dispatcher_handle.abort();
        info!("Distributed compiler coordinator stopped");
    }

    /// Clone for spawning tasks
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            job_queue: self.job_queue.clone(),
            active_jobs: self.active_jobs.clone(),
            job_semaphore: self.job_semaphore.clone(),
            stats: self.stats.clone(),
            is_running: self.is_running.clone(),
            worker_registry: self.worker_registry.clone(),
        }
    }

    /// Handle job submission
    async fn handle_submit_job(&self, job: CompilationJob, result_sender: oneshot::Sender<CompilationResult>) {
        let job_id = job.id.clone();
        
        // Split large jobs into chunks if necessary
        let chunks = if job.source_files.len() > self.config.chunk_size {
            job.split_into_chunks(self.config.chunk_size)
        } else {
            vec![job]
        };

        if chunks.len() == 1 {
            // Single job
            self.queue_job(chunks.into_iter().next().unwrap(), Some(result_sender)).await;
        } else {
            // Multi-chunk job - handle separately
            let coordinator = self.clone();
            tokio::spawn(async move {
                coordinator.handle_chunked_job(chunks, result_sender).await;
            });
        }

        debug!(job_id, "Job queued for processing");
    }

    /// Handle chunked job processing
    async fn handle_chunked_job(&self, chunks: Vec<CompilationJob>, result_sender: oneshot::Sender<CompilationResult>) {
        let mut chunk_results = Vec::new();
        let mut handles = Vec::new();

        // Submit all chunks
        for chunk in chunks {
            let (chunk_sender, chunk_receiver) = oneshot::channel();
            self.queue_job(chunk, Some(chunk_sender)).await;
            handles.push(chunk_receiver);
        }

        // Collect chunk results
        for handle in handles {
            match handle.await {
                Ok(result) => chunk_results.push(result),
                Err(_) => {
                    // Create error result for failed chunk
                    let error_result = CompilationResult {
                        job_id: "chunk_error".to_string(),
                        worker_id: "unknown".to_string(),
                        success: false,
                        output: Vec::new(),
                        error_message: Some("Chunk processing failed".to_string()),
                        warnings: Vec::new(),
                        compilation_time: Duration::ZERO,
                        output_files: Vec::new(),
                        cache_key: None,
                        completed_at: SystemTime::now(),
                        resource_usage: ResourceUsage::default(),
                    };
                    chunk_results.push(error_result);
                }
            }
        }

        // Combine chunk results
        let combined_result = self.combine_chunk_results(chunk_results).await;
        let _ = result_sender.send(combined_result);
    }

    /// Combine results from multiple chunks
    async fn combine_chunk_results(&self, chunk_results: Vec<CompilationResult>) -> CompilationResult {
        let success = chunk_results.iter().all(|r| r.success);
        let mut combined_output = Vec::new();
        let mut warnings = Vec::new();
        let mut output_files = Vec::new();
        let mut total_compilation_time = Duration::ZERO;
        let mut resource_usage = ResourceUsage::default();

        for result in &chunk_results {
            combined_output.extend_from_slice(&result.output);
            warnings.extend_from_slice(&result.warnings);
            output_files.extend_from_slice(&result.output_files);
            total_compilation_time += result.compilation_time;
            resource_usage.cpu_time += result.resource_usage.cpu_time;
            resource_usage.memory_peak_mb = resource_usage.memory_peak_mb.max(result.resource_usage.memory_peak_mb);
            resource_usage.disk_io_mb += result.resource_usage.disk_io_mb;
            resource_usage.network_io_mb += result.resource_usage.network_io_mb;
        }

        let error_message = if success {
            None
        } else {
            Some(chunk_results
                .iter()
                .filter_map(|r| r.error_message.as_ref())
                .cloned()
                .collect::<Vec<_>>()
                .join("; "))
        };

        CompilationResult {
            job_id: "combined".to_string(),
            worker_id: "coordinator".to_string(),
            success,
            output: combined_output,
            error_message,
            warnings,
            compilation_time: total_compilation_time,
            output_files,
            cache_key: None,
            completed_at: SystemTime::now(),
            resource_usage,
        }
    }

    /// Queue a job for processing
    async fn queue_job(&self, job: CompilationJob, result_sender: Option<oneshot::Sender<CompilationResult>>) {
        let job_tracker = JobTracker {
            job: job.clone(),
            state: JobState::Pending,
            started_at: None,
            retry_count: 0,
            result_sender,
        };

        // Add to active jobs
        {
            let mut active_jobs = self.active_jobs.write().unwrap();
            active_jobs.insert(job.id.clone(), job_tracker);
        }

        // Add to queue
        {
            let mut queue = self.job_queue.lock().unwrap();
            queue.push_back(job);
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_jobs_submitted += 1;
            stats.queue_length = queue.len();
        }
    }

    /// Job dispatcher loop
    async fn run_job_dispatcher(&self) {
        while self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            self.dispatch_pending_jobs().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    /// Dispatch pending jobs to available workers
    async fn dispatch_pending_jobs(&self) {
        let job = {
            let mut queue = self.job_queue.lock().unwrap();
            queue.pop_front()
        };

        if let Some(job) = job {
            // Acquire semaphore permit
            if let Ok(permit) = self.job_semaphore.try_acquire() {
                let coordinator = self.clone();
                tokio::spawn(async move {
                    coordinator.execute_job(job, permit).await;
                });
            } else {
                // No permits available, put job back
                let mut queue = self.job_queue.lock().unwrap();
                queue.push_front(job);
            }
        }
    }

    /// Execute a job on an available worker
    async fn execute_job(&self, job: CompilationJob, _permit: tokio::sync::SemaphorePermit<'_>) {
        let job_id = job.id.clone();
        let start_time = Instant::now();

        // Update job state to running
        {
            let mut active_jobs = self.active_jobs.write().unwrap();
            if let Some(tracker) = active_jobs.get_mut(&job_id) {
                tracker.state = JobState::Running("mock_worker".to_string());
                tracker.started_at = Some(start_time);
            }
        }

        // Mock compilation (replace with actual worker communication)
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Create mock result
        let result = CompilationResult {
            job_id: job_id.clone(),
            worker_id: "mock_worker".to_string(),
            success: true,
            output: b"mock compiled output".to_vec(),
            error_message: None,
            warnings: Vec::new(),
            compilation_time: start_time.elapsed(),
            output_files: vec!["output.o".to_string()],
            cache_key: Some(job.cache_key()),
            completed_at: SystemTime::now(),
            resource_usage: ResourceUsage {
                cpu_time: start_time.elapsed(),
                memory_peak_mb: 128,
                disk_io_mb: 1.0,
                network_io_mb: 0.1,
            },
        };

        self.handle_job_completed(result).await;
    }

    /// Handle job completion
    async fn handle_job_completed(&self, result: CompilationResult) {
        let job_id = result.job_id.clone();

        // Update job state and send result
        {
            let mut active_jobs = self.active_jobs.write().unwrap();
            if let Some(mut tracker) = active_jobs.remove(&job_id) {
                tracker.state = JobState::Completed(result.clone());
                
                if let Some(sender) = tracker.result_sender.take() {
                    let _ = sender.send(result.clone());
                }
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            if result.success {
                stats.jobs_completed += 1;
            } else {
                stats.jobs_failed += 1;
            }
            
            // Update average job time
            let new_time = result.compilation_time;
            let total_jobs = stats.jobs_completed + stats.jobs_failed;
            if total_jobs > 1 {
                let weight = 1.0 / total_jobs as f64;
                let old_avg = stats.average_job_time.as_secs_f64();
                let new_avg = old_avg * (1.0 - weight) + new_time.as_secs_f64() * weight;
                stats.average_job_time = Duration::from_secs_f64(new_avg);
            } else {
                stats.average_job_time = new_time;
            }
            
            stats.total_compilation_time += new_time;
            stats.active_jobs_count = active_jobs.len();
        }

        info!(job_id, success = result.success, duration = ?result.compilation_time, "Job completed");
    }

    /// Handle job failure
    async fn handle_job_failed(&self, job_id: String, worker_id: String, error: String) {
        // Check if job should be retried
        let should_retry = {
            let active_jobs = self.active_jobs.read().unwrap();
            if let Some(tracker) = active_jobs.get(&job_id) {
                tracker.retry_count < self.config.retry_attempts
            } else {
                false
            }
        };

        if should_retry {
            // Retry the job
            {
                let mut active_jobs = self.active_jobs.write().unwrap();
                if let Some(tracker) = active_jobs.get_mut(&job_id) {
                    tracker.retry_count += 1;
                    tracker.state = JobState::Pending;
                    
                    // Re-queue the job
                    let mut queue = self.job_queue.lock().unwrap();
                    queue.push_back(tracker.job.clone());
                }
            }

            {
                let mut stats = self.stats.lock().unwrap();
                stats.jobs_retried += 1;
            }

            warn!(job_id, worker_id, error, "Job failed, retrying");
        } else {
            // Job failed permanently
            let result = CompilationResult {
                job_id: job_id.clone(),
                worker_id,
                success: false,
                output: Vec::new(),
                error_message: Some(error.clone()),
                warnings: Vec::new(),
                compilation_time: Duration::ZERO,
                output_files: Vec::new(),
                cache_key: None,
                completed_at: SystemTime::now(),
                resource_usage: ResourceUsage::default(),
            };

            self.handle_job_completed(result).await;
            error!(job_id, error, "Job failed permanently");
        }
    }

    /// Handle worker registration
    async fn handle_worker_registered(&self, worker: super::worker_node::WorkerNode) {
        let worker_id = worker.id.clone();
        {
            let mut registry = self.worker_registry.write().unwrap();
            registry.insert(worker_id.clone(), worker);
        }
        info!(worker_id, "Worker registered");
    }

    /// Handle worker unregistration
    async fn handle_worker_unregistered(&self, worker_id: String) {
        {
            let mut registry = self.worker_registry.write().unwrap();
            registry.remove(&worker_id);
        }
        info!(worker_id, "Worker unregistered");
    }

    /// Handle configuration update
    async fn handle_update_config(&self, _config: CompilerConfig) {
        // Implementation would update the coordinator configuration
        info!("Configuration updated");
    }
}

