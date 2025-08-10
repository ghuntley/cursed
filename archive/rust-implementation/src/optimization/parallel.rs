//! Parallel compilation module for CURSED

use crate::error::CursedError;
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

/// Priority levels for compilation jobs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobPriority {
    Critical,
    High,
    Normal,
    Medium,
    Low,
}

/// Compilation job definition
#[derive(Debug, Clone)]
pub struct CompilationJob {
    pub id: String,
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub priority: JobPriority,
    pub dependencies: Vec<PathBuf>,
    pub compile_flags: Vec<String>,
    pub created_at: Instant,
}

impl CompilationJob {
    pub fn new(id: String, source_path: PathBuf, priority: JobPriority) -> Self {
        Self {
            id,
            source_path: source_path.clone(),
            output_path: source_path.with_extension("o"),
            priority,
            dependencies: Vec::new(),
            compile_flags: Vec::new(),
            created_at: Instant::now(),
        }
    }
}

/// Result of a compilation job
#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub job_id: String,
    pub success: bool,
    pub duration: Duration,
    pub memory_used: usize,
    pub output_path: PathBuf,
    pub error_message: Option<String>,
}

/// Parallel compiler for CURSED
pub struct ParallelCompiler {
    max_workers: usize,
    memory_limit_mb: Option<usize>,
    cpu_threshold: Option<f64>,
    job_queue: Arc<Mutex<VecDeque<CompilationJob>>>,
    results: Arc<Mutex<Vec<CompilationResult>>>,
    worker_handles: Vec<thread::JoinHandle<()>>,
    shutdown_tx: Option<mpsc::Sender<()>>,
    completed_jobs: Arc<Mutex<HashMap<String, CompilationResult>>>,
    progress_enabled: bool,
}

impl ParallelCompiler {
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers,
            memory_limit_mb: None,
            cpu_threshold: None,
            job_queue: Arc::new(Mutex::new(VecDeque::new())),
            results: Arc::new(Mutex::new(Vec::new())),
            worker_handles: Vec::new(),
            shutdown_tx: None,
            completed_jobs: Arc::new(Mutex::new(HashMap::new())),
            progress_enabled: false,
        }
    }

    pub fn with_limits(max_workers: usize, memory_limit_mb: usize, cpu_threshold: f64) -> Self {
        Self {
            max_workers,
            memory_limit_mb: Some(memory_limit_mb),
            cpu_threshold: Some(cpu_threshold),
            job_queue: Arc::new(Mutex::new(VecDeque::new())),
            results: Arc::new(Mutex::new(Vec::new())),
            worker_handles: Vec::new(),
            shutdown_tx: None,
            completed_jobs: Arc::new(Mutex::new(HashMap::new())),
            progress_enabled: false,
        }
    }

    pub fn start(&mut self) -> Result<(), CursedError> {
        self.start_with_progress(false)
    }

    pub fn start_with_progress(&mut self, enable_progress: bool) -> Result<(), CursedError> {
        self.progress_enabled = enable_progress;
        
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        self.shutdown_tx = Some(shutdown_tx);
        let shutdown_rx = Arc::new(Mutex::new(shutdown_rx));

        for worker_id in 0..self.max_workers {
            let job_queue = Arc::clone(&self.job_queue);
            let results = Arc::clone(&self.results);
            let completed_jobs = Arc::clone(&self.completed_jobs);
            let shutdown_rx = Arc::clone(&shutdown_rx);
            let memory_limit = self.memory_limit_mb;
            let cpu_threshold = self.cpu_threshold;
            let progress_enabled = self.progress_enabled;

            let handle = thread::spawn(move || {
                Self::worker_loop(
                    worker_id,
                    job_queue,
                    results,
                    completed_jobs,
                    shutdown_rx,
                    memory_limit,
                    cpu_threshold,
                    progress_enabled,
                );
            });

            self.worker_handles.push(handle);
        }

        if self.progress_enabled {
            println!("✓ Started {} worker threads with progress reporting", self.max_workers);
        }

        Ok(())
    }

    pub fn add_job(&mut self, job: CompilationJob) {
        let mut queue = self.job_queue.lock().unwrap();
        queue.push_back(job);
    }

    pub fn add_jobs_with_dependencies(&mut self, jobs: Vec<CompilationJob>) -> Result<(), CursedError> {
        // Sort jobs by priority and dependencies
        let mut sorted_jobs = jobs;
        sorted_jobs.sort_by(|a, b| {
            // First by priority (Critical first)
            match a.priority.cmp(&b.priority) {
                std::cmp::Ordering::Equal => {
                    // Then by dependency count (fewer dependencies first)
                    a.dependencies.len().cmp(&b.dependencies.len())
                }
                other => other,
            }
        });

        for job in sorted_jobs {
            self.add_job(job);
        }

        Ok(())
    }

    pub fn wait_for_completion(&mut self, timeout: Option<Duration>) -> Result<Vec<CompilationResult>, CursedError> {
        let start_time = Instant::now();
        let total_jobs = {
            let queue = self.job_queue.lock().unwrap();
            queue.len()
        };

        if total_jobs == 0 {
            return Ok(Vec::new());
        }

        let mut last_progress = 0;
        
        loop {
            let (completed_count, queue_empty) = {
                let queue = self.job_queue.lock().unwrap();
                let completed = self.completed_jobs.lock().unwrap();
                (completed.len(), queue.is_empty())
            };

            if self.progress_enabled && completed_count > last_progress {
                println!("  Progress: {}/{} jobs completed", completed_count, total_jobs);
                last_progress = completed_count;
            }

            if queue_empty && completed_count >= total_jobs {
                break;
            }

            if let Some(timeout) = timeout {
                if start_time.elapsed() > timeout {
                    return Err(CursedError::RuntimeError("Compilation timeout".to_string()));
                }
            }

            thread::sleep(Duration::from_millis(100));
        }

        let results = self.results.lock().unwrap();
        Ok(results.clone())
    }

    pub fn stop(&mut self) -> Result<(), CursedError> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            // Ignore SendError - workers may have already shut down
            let _ = shutdown_tx.send(());
        }

        for handle in self.worker_handles.drain(..) {
            let _ = handle.join(); // Workers should shut down gracefully
        }

        Ok(())
    }

    pub fn get_compilation_report(&self) -> String {
        let results = self.results.lock().unwrap();
        let successful = results.iter().filter(|r| r.success).count();
        let total = results.len();
        let avg_duration = if total > 0 {
            results.iter().map(|r| r.duration.as_millis()).sum::<u128>() / total as u128
        } else {
            0
        };

        format!(
            "Compilation Report: {}/{} successful, avg duration: {}ms",
            successful, total, avg_duration
        )
    }

    fn worker_loop(
        worker_id: usize,
        job_queue: Arc<Mutex<VecDeque<CompilationJob>>>,
        results: Arc<Mutex<Vec<CompilationResult>>>,
        completed_jobs: Arc<Mutex<HashMap<String, CompilationResult>>>,
        shutdown_rx: Arc<Mutex<mpsc::Receiver<()>>>,
        memory_limit_mb: Option<usize>,
        cpu_threshold: Option<f64>,
        progress_enabled: bool,
    ) {
        loop {
            // Check for shutdown signal
            if let Ok(rx) = shutdown_rx.lock() {
                match rx.try_recv() {
                    Ok(()) => break, // Shutdown requested
                    Err(mpsc::TryRecvError::Disconnected) => break, // Channel closed
                    Err(mpsc::TryRecvError::Empty) => {} // Continue working
                }
            }

            // Get next job from queue
            let job = {
                let mut queue = job_queue.lock().unwrap();
                queue.pop_front()
            };

            if let Some(job) = job {
                if progress_enabled {
                    println!("  Worker {} starting job: {}", worker_id, job.id);
                }

                let start_time = Instant::now();
                let result = Self::compile_job(&job, memory_limit_mb, cpu_threshold);
                let duration = start_time.elapsed();
                let success = result.is_ok();
                let error_message = result.err().map(|e| e.to_string());

                let compilation_result = CompilationResult {
                    job_id: job.id.clone(),
                    success,
                    duration,
                    memory_used: Self::estimate_memory_usage(&job),
                    output_path: job.output_path,
                    error_message,
                };

                // Store result
                {
                    let mut results = results.lock().unwrap();
                    results.push(compilation_result.clone());
                }

                {
                    let mut completed = completed_jobs.lock().unwrap();
                    completed.insert(job.id.clone(), compilation_result);
                }

                if progress_enabled {
                    let status = if success { "✓" } else { "✗" };
                    println!("  Worker {} {} job: {} ({:?})", 
                            worker_id, status, job.id, duration);
                }
            } else {
                // No jobs available, wait a bit
                thread::sleep(Duration::from_millis(50));
            }
        }
    }

    fn compile_job(
        job: &CompilationJob,
        memory_limit_mb: Option<usize>,
        cpu_threshold: Option<f64>,
    ) -> Result<(), CursedError> {
        // Check resource constraints
        if let Some(memory_limit) = memory_limit_mb {
            let estimated_memory = Self::estimate_memory_usage(job);
            if estimated_memory > memory_limit * 1024 * 1024 {
                return Err(CursedError::RuntimeError(
                    format!("Job {} exceeds memory limit", job.id)
                ));
            }
        }

        if let Some(cpu_threshold) = cpu_threshold {
            let cpu_usage = Self::get_cpu_usage();
            if cpu_usage > cpu_threshold {
                thread::sleep(Duration::from_millis(100)); // Wait for CPU to cool down
            }
        }

        // Simulate compilation work
        let compile_time = match job.priority {
            JobPriority::Critical => Duration::from_millis(50),
            JobPriority::High => Duration::from_millis(100),
            JobPriority::Normal => Duration::from_millis(200),
            JobPriority::Medium => Duration::from_millis(300),
            JobPriority::Low => Duration::from_millis(500),
        };

        thread::sleep(compile_time);

        // Simulate occasional failures
        if job.id.contains("complex") && rand::random::<f64>() < 0.1 {
            return Err(CursedError::RuntimeError("Simulated compilation error".to_string()));
        }

        Ok(())
    }

    fn estimate_memory_usage(job: &CompilationJob) -> usize {
        // Estimate based on source file size and complexity
        let base_usage = 1024 * 1024; // 1MB base
        let complexity_factor = if job.id.contains("complex") { 4 } else { 1 };
        let flag_factor = job.compile_flags.len() + 1;
        
        base_usage * complexity_factor * flag_factor
    }

    fn get_cpu_usage() -> f64 {
        // Simplified CPU usage simulation
        rand::random::<f64>() * 100.0
    }

    pub fn compile_all(&mut self) -> Result<(), CursedError> {
        println!("Starting parallel compilation with {} workers", self.max_workers);
        for job in &self.job_queue.lock().unwrap().iter().collect::<Vec<_>>() {
            println!("Compiling job: {} (priority: {:?})", job.id, job.priority);
        }
        Ok(())
    }
}

impl Drop for ParallelCompiler {
    fn drop(&mut self) {
        // Ensure workers are properly shut down
        let _ = self.stop();
    }
}

// Add a simple random number generator for simulation
mod rand {
    use std::cell::RefCell;
    use std::time::{SystemTime, UNIX_EPOCH};

    thread_local! {
        static RNG: RefCell<SimpleRng> = RefCell::new(SimpleRng::new());
    }

    pub fn random<T>() -> T 
    where 
        T: RandomValue,
    {
        RNG.with(|rng| T::random_value(&mut *rng.borrow_mut()))
    }

    pub trait RandomValue {
        fn random_value(rng: &mut SimpleRng) -> Self;
    }

    impl RandomValue for f64 {
        fn random_value(rng: &mut SimpleRng) -> Self {
            rng.next_f64()
        }
    }

    impl RandomValue for bool {
        fn random_value(rng: &mut SimpleRng) -> Self {
            rng.next_u32() % 2 == 0
        }
    }

    pub struct SimpleRng {
        state: u64,
    }

    impl SimpleRng {
        fn new() -> Self {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
            Self { state: seed }
        }

        fn next_u32(&mut self) -> u32 {
            self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
            (self.state >> 32) as u32
        }

        fn next_f64(&mut self) -> f64 {
            let val = self.next_u32() as f64 / u32::MAX as f64;
            val
        }
    }
}
