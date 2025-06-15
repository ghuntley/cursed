/// Parallel Compilation System
/// 
/// Provides parallel compilation capabilities for CURSED source files using
/// worker threads, job scheduling, and dependency-aware compilation.

use crate::error::{Error, Result};
use crate::optimization::OptimizationResult;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc, Condvar};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use crossbeam_channel::{Receiver, Sender, bounded, unbounded};

/// Compilation job information
#[derive(Debug, Clone)]
pub struct CompilationJob {
    pub id: String,
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub dependencies: Vec<PathBuf>,
    pub priority: JobPriority,
    pub compile_flags: Vec<String>,
    pub created_at: Instant,
}

/// Job priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Compilation result for a single job
#[derive(Debug, Clone)]
pub struct JobResult {
    pub job_id: String,
    pub success: bool,
    pub duration: Duration,
    pub memory_used: usize,
    pub output_size: usize,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

/// Worker thread state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    Idle,
    Working,
    Finished,
    Error,
}

/// Worker thread information
#[derive(Debug, Clone)]
pub struct WorkerInfo {
    pub id: usize,
    pub state: WorkerState,
    pub current_job: Option<String>,
    pub jobs_completed: usize,
    pub total_time: Duration,
    pub created_at: Instant,
}

/// Parallel compiler implementation
pub struct ParallelCompiler {
    worker_count: usize,
    job_queue: Arc<Mutex<VecDeque<CompilationJob>>>,
    completed_jobs: Arc<Mutex<Vec<JobResult>>>,
    workers: Vec<WorkerInfo>,
    job_sender: Option<Sender<CompilationJob>>,
    result_receiver: Option<Receiver<JobResult>>,
    shutdown_signal: Arc<Mutex<bool>>,
    stats: Arc<Mutex<ParallelStats>>,
}

/// Compilation statistics
#[derive(Debug, Clone)]
pub struct ParallelStats {
    pub jobs_queued: usize,
    pub jobs_completed: usize,
    pub jobs_failed: usize,
    pub total_compilation_time: Duration,
    pub wall_clock_time: Duration,
    pub average_job_time: Duration,
    pub worker_utilization: f64,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

impl Default for ParallelStats {
    fn default() -> Self {
        Self {
            jobs_queued: 0,
            jobs_completed: 0,
            jobs_failed: 0,
            total_compilation_time: Duration::from_secs(0),
            wall_clock_time: Duration::from_secs(0),
            average_job_time: Duration::from_secs(0),
            worker_utilization: 0.0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}

impl ParallelCompiler {
    /// Create new parallel compiler
    pub fn new(worker_count: usize) -> Self {
        let effective_workers = worker_count.max(1);
        
        Self {
            worker_count: effective_workers,
            job_queue: Arc::new(Mutex::new(VecDeque::new())),
            completed_jobs: Arc::new(Mutex::new(Vec::new())),
            workers: Vec::new(),
            job_sender: None,
            result_receiver: None,
            shutdown_signal: Arc::new(Mutex::new(false)),
            stats: Arc::new(Mutex::new(ParallelStats::default())),
        }
    }
    
    /// Start worker threads
    pub fn start(&mut self) -> Result<()> {
        let (job_tx, job_rx) = bounded(self.worker_count * 2);
        let (result_tx, result_rx) = unbounded();
        
        self.job_sender = Some(job_tx);
        self.result_receiver = Some(result_rx);
        
        // Start worker threads
        for worker_id in 0..self.worker_count {
            let job_receiver = job_rx.clone();
            let result_sender = result_tx.clone();
            let shutdown_signal = self.shutdown_signal.clone();
            let stats = self.stats.clone();
            
            thread::spawn(move || {
                Self::worker_thread(worker_id, job_receiver, result_sender, shutdown_signal, stats);
            });
            
            self.workers.push(WorkerInfo {
                id: worker_id,
                state: WorkerState::Idle,
                current_job: None,
                jobs_completed: 0,
                total_time: Duration::from_secs(0),
                created_at: Instant::now(),
            });
        }
        
        Ok(())
    }
    
    /// Stop all worker threads
    pub fn stop(&mut self) -> Result<()> {
        // Signal shutdown
        {
            let mut shutdown = self.shutdown_signal.lock().unwrap();
            *shutdown = true;
        }
        
        // Clear channels
        if let Some(sender) = self.job_sender.take() {
            drop(sender);
        }
        
        // Update worker states
        for worker in &mut self.workers {
            if worker.state != WorkerState::Finished {
                worker.state = WorkerState::Finished;
            }
        }
        
        Ok(())
    }
    
    /// Add compilation job to queue
    pub fn add_job(&self, job: CompilationJob) -> Result<()> {
        if let Some(ref sender) = self.job_sender {
            sender.send(job.clone())
                .map_err(|e| Error::Other(format!("Failed to queue job: {}", e)))?;
            
            // Update stats
            let mut stats = self.stats.lock().unwrap();
            stats.jobs_queued += 1;
            
            Ok(())
        } else {
            Err(Error::Other("Parallel compiler not started".to_string()))
        }
    }
    
    /// Add multiple jobs with dependency resolution
    pub fn add_jobs_with_dependencies(&self, jobs: Vec<CompilationJob>) -> Result<()> {
        let ordered_jobs = self.resolve_dependencies(jobs)?;
        
        for job in ordered_jobs {
            self.add_job(job)?;
        }
        
        Ok(())
    }
    
    /// Wait for all jobs to complete
    pub fn wait_for_completion(&self, timeout: Option<Duration>) -> Result<Vec<JobResult>> {
        let start_time = Instant::now();
        let mut results = Vec::new();
        
        if let Some(ref receiver) = self.result_receiver {
            loop {
                // Check timeout
                if let Some(timeout_duration) = timeout {
                    if start_time.elapsed() > timeout_duration {
                        return Err(Error::Other("Compilation timeout".to_string()));
                    }
                }
                
                // Check if we have any pending jobs
                let stats = self.stats.lock().unwrap();
                let pending_jobs = stats.jobs_queued - stats.jobs_completed - stats.jobs_failed;
                drop(stats);
                
                if pending_jobs == 0 {
                    break;
                }
                
                // Receive result with timeout
                match receiver.recv_timeout(Duration::from_millis(100)) {
                    Ok(result) => {
                        // Update worker info
                        self.update_worker_on_completion(&result);
                        
                        // Update stats
                        let mut stats = self.stats.lock().unwrap();
                        if result.success {
                            stats.jobs_completed += 1;
                        } else {
                            stats.jobs_failed += 1;
                        }
                        stats.total_compilation_time += result.duration;
                        drop(stats);
                        
                        results.push(result);
                    }
                    Err(_) => {
                        // Timeout, continue checking
                        continue;
                    }
                }
            }
        }
        
        // Update wall clock time
        let mut stats = self.stats.lock().unwrap();
        stats.wall_clock_time = start_time.elapsed();
        if stats.jobs_completed > 0 {
            stats.average_job_time = stats.total_compilation_time / stats.jobs_completed as u32;
        }
        stats.worker_utilization = if stats.wall_clock_time.as_secs() > 0 {
            stats.total_compilation_time.as_secs_f64() / 
            (stats.wall_clock_time.as_secs_f64() * self.worker_count as f64)
        } else {
            0.0
        };
        
        Ok(results)
    }
    
    /// Get current compilation statistics
    pub fn get_stats(&self) -> ParallelStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Get worker information
    pub fn get_workers(&self) -> Vec<WorkerInfo> {
        self.workers.clone()
    }
    
    /// Get active worker count
    pub fn active_workers(&self) -> usize {
        self.workers.iter()
            .filter(|w| matches!(w.state, WorkerState::Working))
            .count()
    }
    
    /// Resolve job dependencies using topological sort
    fn resolve_dependencies(&self, jobs: Vec<CompilationJob>) -> Result<Vec<CompilationJob>> {
        let mut job_map: HashMap<PathBuf, CompilationJob> = HashMap::new();
        let mut dependency_graph: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
        let mut in_degree: HashMap<PathBuf, usize> = HashMap::new();
        
        // Build job map and dependency graph
        for job in jobs {
            let source_path = job.source_path.clone();
            dependency_graph.insert(source_path.clone(), job.dependencies.clone());
            in_degree.insert(source_path.clone(), job.dependencies.len());
            job_map.insert(source_path, job);
        }
        
        // Topological sort using Kahn's algorithm
        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        let mut result: Vec<CompilationJob> = Vec::new();
        
        // Find nodes with no incoming edges
        for (path, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(path.clone());
            }
        }
        
        while let Some(current) = queue.pop_front() {
            if let Some(job) = job_map.remove(&current) {
                result.push(job);
            }
            
            // Reduce in-degree of dependent nodes
            for (path, deps) in &dependency_graph {
                if deps.contains(&current) {
                    if let Some(degree) = in_degree.get_mut(path) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(path.clone());
                        }
                    }
                }
            }
        }
        
        // Check for circular dependencies
        if result.len() != job_map.len() + result.len() {
            return Err(Error::Other("Circular dependencies detected".to_string()));
        }
        
        Ok(result)
    }
    
    /// Worker thread implementation
    fn worker_thread(
        worker_id: usize,
        job_receiver: Receiver<CompilationJob>,
        result_sender: Sender<JobResult>,
        shutdown_signal: Arc<Mutex<bool>>,
        stats: Arc<Mutex<ParallelStats>>,
    ) {
        loop {
            // Check shutdown signal
            {
                let shutdown = shutdown_signal.lock().unwrap();
                if *shutdown {
                    break;
                }
            }
            
            // Wait for job
            match job_receiver.recv_timeout(Duration::from_millis(100)) {
                Ok(job) => {
                    let start_time = Instant::now();
                    
                    // Compile the job (placeholder implementation)
                    let result = Self::compile_job(worker_id, job.clone());
                    
                    let duration = start_time.elapsed();
                    let job_result = JobResult {
                        job_id: job.id,
                        success: result.is_ok(),
                        duration,
                        memory_used: 0, // Would be measured in real implementation
                        output_size: 0, // Would be measured in real implementation
                        warnings: Vec::new(),
                        error: result.err().map(|e| e.to_string()),
                    };
                    
                    // Send result
                    if let Err(_) = result_sender.send(job_result) {
                        break; // Channel closed
                    }
                }
                Err(_) => {
                    // Timeout, continue checking shutdown signal
                    continue;
                }
            }
        }
    }
    
    /// Compile a single job (placeholder implementation)
    fn compile_job(worker_id: usize, job: CompilationJob) -> Result<()> {
        // This is a placeholder implementation
        // In a real implementation, this would call the actual compiler
        
        println!("Worker {} compiling: {}", worker_id, job.source_path.display());
        
        // Simulate compilation time
        thread::sleep(Duration::from_millis(100));
        
        // Simulate occasional failures
        if job.source_path.to_string_lossy().contains("fail") {
            return Err(Error::Other("Simulated compilation error".to_string()));
        }
        
        Ok(())
    }
    
    /// Update worker information when job completes
    fn update_worker_on_completion(&self, result: &JobResult) {
        // This would update worker statistics in a real implementation
        // For now, we'll just log the completion
        println!("Job {} completed in {:?}", result.job_id, result.duration);
    }
}

impl Drop for ParallelCompiler {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Utility functions for parallel compilation
pub mod utils {
    use super::*;
    
    /// Create compilation jobs from source files
    pub fn create_jobs_from_files(
        source_files: &[PathBuf],
        output_dir: &Path,
        compile_flags: &[String],
    ) -> Vec<CompilationJob> {
        source_files.iter()
            .enumerate()
            .map(|(i, source_path)| {
                let output_path = output_dir.join(
                    source_path.file_stem().unwrap_or_default()
                ).with_extension("o");
                
                CompilationJob {
                    id: format!("job_{}", i),
                    source_path: source_path.clone(),
                    output_path,
                    dependencies: Vec::new(),
                    priority: JobPriority::Normal,
                    compile_flags: compile_flags.to_vec(),
                    created_at: Instant::now(),
                }
            })
            .collect()
    }
    
    /// Analyze job results and create optimization result
    pub fn analyze_job_results(
        results: &[JobResult],
        stats: &ParallelStats,
    ) -> OptimizationResult {
        let mut opt_result = OptimizationResult::default();
        
        opt_result.success = results.iter().all(|r| r.success);
        opt_result.duration = stats.wall_clock_time;
        opt_result.files_processed = results.len();
        
        // Calculate improvements based on parallelization
        let sequential_time: Duration = results.iter().map(|r| r.duration).sum();
        let parallel_time = stats.wall_clock_time;
        
        if sequential_time.as_secs() > 0 && parallel_time.as_secs() > 0 {
            opt_result.compilation_speed_improvement = 
                100.0 * (1.0 - parallel_time.as_secs_f64() / sequential_time.as_secs_f64());
        }
        
        // Collect errors and warnings
        for result in results {
            if let Some(ref error) = result.error {
                opt_result.add_error(format!("Job {}: {}", result.job_id, error));
            }
            for warning in &result.warnings {
                opt_result.add_warning(format!("Job {}: {}", result.job_id, warning));
            }
        }
        
        opt_result.set_cache_stats(stats.cache_hits, stats.cache_misses);
        
        opt_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_parallel_compiler_creation() {
        let compiler = ParallelCompiler::new(4);
        assert_eq!(compiler.worker_count, 4);
        assert_eq!(compiler.active_workers(), 0);
    }
    
    #[test]
    fn test_job_priority_ordering() {
        assert!(JobPriority::Critical > JobPriority::High);
        assert!(JobPriority::High > JobPriority::Normal);
        assert!(JobPriority::Normal > JobPriority::Low);
    }
    
    #[test]
    fn test_dependency_resolution() {
        let temp_dir = TempDir::new().unwrap();
        let compiler = ParallelCompiler::new(2);
        
        let job1 = CompilationJob {
            id: "job1".to_string(),
            source_path: temp_dir.path().join("file1.csd"),
            output_path: temp_dir.path().join("file1.o"),
            dependencies: vec![temp_dir.path().join("file2.csd")],
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: Instant::now(),
        };
        
        let job2 = CompilationJob {
            id: "job2".to_string(),
            source_path: temp_dir.path().join("file2.csd"),
            output_path: temp_dir.path().join("file2.o"),
            dependencies: Vec::new(),
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: Instant::now(),
        };
        
        let jobs = vec![job1, job2];
        let ordered = compiler.resolve_dependencies(jobs).unwrap();
        
        // job2 should come before job1 due to dependency
        assert_eq!(ordered[0].id, "job2");
        assert_eq!(ordered[1].id, "job1");
    }
    
    #[test]
    fn test_circular_dependency_detection() {
        let temp_dir = TempDir::new().unwrap();
        let compiler = ParallelCompiler::new(2);
        
        let job1 = CompilationJob {
            id: "job1".to_string(),
            source_path: temp_dir.path().join("file1.csd"),
            output_path: temp_dir.path().join("file1.o"),
            dependencies: vec![temp_dir.path().join("file2.csd")],
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: Instant::now(),
        };
        
        let job2 = CompilationJob {
            id: "job2".to_string(),
            source_path: temp_dir.path().join("file2.csd"),
            output_path: temp_dir.path().join("file2.o"),
            dependencies: vec![temp_dir.path().join("file1.csd")],
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: Instant::now(),
        };
        
        let jobs = vec![job1, job2];
        let result = compiler.resolve_dependencies(jobs);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Circular dependencies"));
    }
    
    #[test]
    fn test_job_creation_from_files() {
        let temp_dir = TempDir::new().unwrap();
        let source_files = vec![
            temp_dir.path().join("file1.csd"),
            temp_dir.path().join("file2.csd"),
        ];
        let output_dir = temp_dir.path().join("output");
        let compile_flags = vec!["--optimize".to_string()];
        
        let jobs = utils::create_jobs_from_files(&source_files, &output_dir, &compile_flags);
        
        assert_eq!(jobs.len(), 2);
        assert_eq!(jobs[0].source_path, source_files[0]);
        assert_eq!(jobs[1].source_path, source_files[1]);
        assert!(jobs[0].output_path.ends_with("file1.o"));
        assert!(jobs[1].output_path.ends_with("file2.o"));
        assert_eq!(jobs[0].compile_flags, compile_flags);
    }
    
    #[test]
    fn test_parallel_stats_default() {
        let stats = ParallelStats::default();
        assert_eq!(stats.jobs_queued, 0);
        assert_eq!(stats.jobs_completed, 0);
        assert_eq!(stats.jobs_failed, 0);
        assert_eq!(stats.worker_utilization, 0.0);
    }
}
