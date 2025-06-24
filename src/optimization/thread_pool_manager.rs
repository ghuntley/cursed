/// Thread Pool Manager for Parallel Optimization
/// 
/// Manages worker threads for parallel compilation and optimization tasks

use crate::error::{Error, Result};

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, Condvar, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn, error};

/// Configuration for thread pool manager
#[derive(Debug, Clone)]
pub struct ThreadPoolConfig {
    /// Number of worker threads
    pub num_threads: usize,
    /// Maximum queue size per worker
    pub max_queue_size: usize,
    /// Thread idle timeout before termination
    pub idle_timeout: Duration,
    /// Enable work stealing between workers
    pub enable_work_stealing: bool,
    /// Priority queue enabled
    pub enable_priority_queue: bool,
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        Self {
            num_threads: num_cpus::get(),
            max_queue_size: 1000,
            idle_timeout: Duration::from_secs(60),
            enable_work_stealing: true,
            enable_priority_queue: false,
        }
    }
}

/// Task to be executed by the thread pool
pub trait Task: Send {
    /// Execute the task
    fn execute(self: Box<Self>) -> Result<()>;
    /// Get task priority (higher = more priority)
    fn priority(&self) -> i32 { 0 }
    /// Get task name for debugging
    fn name(&self) -> &str { "unnamed_task" }
}

/// Work-stealing task queue
#[derive(Debug)]
struct TaskQueue {
    tasks: VecDeque<Box<dyn Task>>,
    priority_tasks: VecDeque<Box<dyn Task>>,
    config: ThreadPoolConfig,
}

impl TaskQueue {
    fn new(config: ThreadPoolConfig) -> Self {
        Self {
            tasks: VecDeque::new(),
            priority_tasks: VecDeque::new(),
            config,
        }
    }

    fn push_task(&mut self, task: Box<dyn Task>) -> Result<()> {
        if self.config.enable_priority_queue && task.priority() > 0 {
            if self.priority_tasks.len() >= self.config.max_queue_size {
                return Err(Error::General("Priority task queue full".to_string()));
            }
            self.priority_tasks.push_back(task);
        } else {
            if self.tasks.len() >= self.config.max_queue_size {
                return Err(Error::General("Task queue full".to_string()));
            }
            self.tasks.push_back(task);
        }
        Ok(())
    }

    fn pop_task(&mut self) -> Option<Box<dyn Task>> {
        // Priority tasks first
        if let Some(task) = self.priority_tasks.pop_front() {
            return Some(task);
        }
        // Then regular tasks
        self.tasks.pop_front()
    }

    fn steal_task(&mut self) -> Option<Box<dyn Task>> {
        // Steal from the back for better cache locality
        self.tasks.pop_back().or_else(|| self.priority_tasks.pop_back())
    }

    fn len(&self) -> usize {
        self.tasks.len() + self.priority_tasks.len()
    }
}

/// Worker thread state
#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
    active: Arc<AtomicBool>,
}

/// Thread pool manager for parallel optimization tasks
#[derive(Debug)]
pub struct ThreadPoolManager {
    config: ThreadPoolConfig,
    workers: Vec<Worker>,
    task_queues: Arc<Mutex<Vec<Arc<Mutex<TaskQueue>>>>>,
    shutdown: Arc<AtomicBool>,
    pending_tasks: Arc<AtomicUsize>,
    task_condvar: Arc<Condvar>,
    statistics: Arc<Mutex<ThreadPoolStatistics>>,
}

/// Statistics for thread pool performance
#[derive(Debug, Default)]
pub struct ThreadPoolStatistics {
    /// Total tasks executed
    pub tasks_executed: u64,
    /// Total tasks failed
    pub tasks_failed: u64,
    /// Average task execution time
    pub avg_execution_time: Duration,
    /// Work stealing operations
    pub work_steals: u64,
    /// Queue utilization (0.0 to 1.0)
    pub queue_utilization: f64,
    /// Active worker count
    pub active_workers: usize,
}

impl ThreadPoolManager {
    /// Create a new thread pool manager
    #[instrument]
    pub fn new(config: ThreadPoolConfig) -> Result<Self> {
        info!("Creating thread pool with {} workers", config.num_threads);
        
        let task_queues = Arc::new(Mutex::new(Vec::new()));
        let shutdown = Arc::new(AtomicBool::new(false));
        let pending_tasks = Arc::new(AtomicUsize::new(0));
        let task_condvar = Arc::new(Condvar::new());
        let statistics = Arc::new(Mutex::new(ThreadPoolStatistics::default()));

        // Initialize task queues
        {
            let mut queues = task_queues.lock().unwrap();
            for _ in 0..config.num_threads {
                queues.push(Arc::new(Mutex::new(TaskQueue::new(config.clone()))));
            }
        }

        let mut workers = Vec::new();
        
        // Spawn worker threads
        for worker_id in 0..config.num_threads {
            let worker_shutdown = shutdown.clone();
            let worker_queues = task_queues.clone();
            let worker_pending = pending_tasks.clone();
            let worker_condvar = task_condvar.clone();
            let worker_config = config.clone();
            let worker_stats = statistics.clone();
            let active = Arc::new(AtomicBool::new(true));
            let worker_active = active.clone();

            let thread = thread::Builder::new()
                .name(format!("optimization-worker-{}", worker_id))
                .spawn(move || {
                    Self::worker_loop(
                        worker_id,
                        worker_shutdown,
                        worker_queues,
                        worker_pending,
                        worker_condvar,
                        worker_config,
                        worker_stats,
                        worker_active,
                    );
                })?;

            workers.push(Worker {
                id: worker_id,
                thread: Some(thread),
                active,
            });
        }

        Ok(Self {
            config,
            workers,
            task_queues,
            shutdown,
            pending_tasks,
            task_condvar,
            statistics,
        })
    }

    /// Submit a task for execution
    #[instrument(skip(self, task))]
    pub fn submit_task(&self, task: Box<dyn Task>) -> Result<()> {
        if self.shutdown.load(Ordering::Relaxed) {
            return Err(Error::General("Thread pool is shutting down".to_string()));
        }

        // Find the least loaded queue
        let queues = self.task_queues.lock().unwrap();
        let mut min_queue_idx = 0;
        let mut min_queue_size = usize::MAX;

        for (idx, queue) in queues.iter().enumerate() {
            let queue_size = queue.lock().unwrap().len();
            if queue_size < min_queue_size {
                min_queue_size = queue_size;
                min_queue_idx = idx;
            }
        }

        // Submit to the least loaded queue
        let queue = &queues[min_queue_idx];
        queue.lock().unwrap().push_task(task)?;
        
        self.pending_tasks.fetch_add(1, Ordering::Relaxed);
        self.task_condvar.notify_one();

        debug!("Task submitted to worker {}", min_queue_idx);
        Ok(())
    }

    /// Wait for all pending tasks to complete
    #[instrument]
    pub fn wait_for_completion(&self) -> Result<()> {
        while self.pending_tasks.load(Ordering::Relaxed) > 0 {
            thread::sleep(Duration::from_millis(10));
        }
        info!("All tasks completed");
        Ok(())
    }

    /// Get current statistics
    pub fn get_statistics(&self) -> ThreadPoolStatistics {
        let stats = self.statistics.lock().unwrap();
        let mut result = stats.clone();
        
        // Update active worker count
        result.active_workers = self.workers.iter()
            .filter(|w| w.active.load(Ordering::Relaxed))
            .count();

        // Update queue utilization
        let queues = self.task_queues.lock().unwrap();
        let total_capacity = queues.len() * self.config.max_queue_size;
        let total_tasks: usize = queues.iter()
            .map(|q| q.lock().unwrap().len())
            .sum();
        result.queue_utilization = total_tasks as f64 / total_capacity as f64;

        result
    }

    /// Shutdown the thread pool
    #[instrument]
    pub fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down thread pool");
        
        self.shutdown.store(true, Ordering::Relaxed);
        self.task_condvar.notify_all();

        // Wait for all workers to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().map_err(|_| Error::General("Failed to join worker thread".to_string()))?;
            }
        }

        info!("Thread pool shutdown complete");
        Ok(())
    }

    /// Worker thread main loop
    fn worker_loop(
        worker_id: usize,
        shutdown: Arc<AtomicBool>,
        task_queues: Arc<Mutex<Vec<Arc<Mutex<TaskQueue>>>>>,
        pending_tasks: Arc<AtomicUsize>,
        task_condvar: Arc<Condvar>,
        config: ThreadPoolConfig,
        statistics: Arc<Mutex<ThreadPoolStatistics>>,
        active: Arc<AtomicBool>,
    ) {
        debug!("Worker {} started", worker_id);
        
        let mut last_activity = Instant::now();
        
        while !shutdown.load(Ordering::Relaxed) {
            let task = {
                let queues = task_queues.lock().unwrap();
                let mut own_queue = queues[worker_id].lock().unwrap();
                
                // Try to get task from own queue first
                if let Some(task) = own_queue.pop_task() {
                    Some(task)
                } else if config.enable_work_stealing {
                    // Try to steal from other queues
                    drop(own_queue);
                    Self::try_steal_task(&queues, worker_id, &statistics)
                } else {
                    None
                }
            };

            if let Some(task) = task {
                active.store(true, Ordering::Relaxed);
                last_activity = Instant::now();
                
                let task_name = task.name().to_string();
                let start_time = Instant::now();
                
                debug!("Worker {} executing task: {}", worker_id, task_name);
                
                // Execute the task
                let result = task.execute();
                let execution_time = start_time.elapsed();
                
                // Update statistics
                let mut stats = statistics.lock().unwrap();
                stats.tasks_executed += 1;
                if result.is_err() {
                    stats.tasks_failed += 1;
                    error!("Task {} failed: {:?}", task_name, result);
                } else {
                    debug!("Task {} completed in {:?}", task_name, execution_time);
                }
                
                // Update average execution time
                stats.avg_execution_time = Duration::from_nanos(
                    ((stats.avg_execution_time.as_nanos() as u64 * (stats.tasks_executed - 1)) +
                     execution_time.as_nanos() as u64) / stats.tasks_executed
                );
                
                pending_tasks.fetch_sub(1, Ordering::Relaxed);
            } else {
                active.store(false, Ordering::Relaxed);
                
                // No task available, wait for notification or timeout
                let _unused = task_condvar.wait_timeout(
                    task_queues.lock().unwrap(),
                    Duration::from_millis(100)
                );
                
                // Check for idle timeout
                if last_activity.elapsed() > config.idle_timeout {
                    debug!("Worker {} idle timeout", worker_id);
                    break;
                }
            }
        }
        
        debug!("Worker {} stopped", worker_id);
    }

    /// Try to steal a task from other workers
    fn try_steal_task(
        queues: &[Arc<Mutex<TaskQueue>>],
        worker_id: usize,
        statistics: &Arc<Mutex<ThreadPoolStatistics>>,
    ) -> Option<Box<dyn Task>> {
        // Try to steal from each other queue
        for (i, queue) in queues.iter().enumerate() {
            if i != worker_id {
                if let Ok(mut queue) = queue.try_lock() {
                    if let Some(task) = queue.steal_task() {
                        // Update steal statistics
                        statistics.lock().unwrap().work_steals += 1;
                        debug!("Worker {} stole task from worker {}", worker_id, i);
                        return Some(task);
                    }
                }
            }
        }
        None
    }
}

impl Drop for ThreadPoolManager {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

// Simple task implementation for testing
pub struct SimpleTask {
    name: String,
    function: Box<dyn FnOnce() -> Result<()> + Send>,
    priority: i32,
}

impl SimpleTask {
    pub fn new<F>(name: String, function: F) -> Self 
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        Self {
            name,
            function: Box::new(function),
            priority: 0,
        }
    }

    pub fn with_priority<F>(name: String, function: F, priority: i32) -> Self 
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        Self {
            name,
            function: Box::new(function),
            priority,
        }
    }
}

impl Task for SimpleTask {
    fn execute(self: Box<Self>) -> Result<()> {
        (self.function)()
    }

    fn priority(&self) -> i32 {
        self.priority
    }

    fn name(&self) -> &str {
        &self.name
    }
}
