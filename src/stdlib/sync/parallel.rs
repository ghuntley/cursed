/// Parallel processing utilities for CURSED
/// 
/// This module provides thread pools, parallel iterators, and work-stealing
/// scheduling for efficient parallel computation.

use crate::stdlib::sync::error::{SyncError, SyncResult, thread_pool_error, timeout_error};
use crate::stdlib::sync::primitives::{spawn, AtomicUsize, AtomicBool, Ordering, Mutex, CondVar};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, AtomicUsize as StdAtomicUsize, AtomicBool as StdAtomicBool, Ordering as StdOrdering};
use num_cpus;

// Global thread pool management
static GLOBAL_POOL_INITIALIZED: StdAtomicBool = StdAtomicBool::new(false);
static GLOBAL_POOL_UTILIZATION: AtomicU64 = AtomicU64::new(0);
static mut GLOBAL_POOL: Option<ThreadPool> = None;
static GLOBAL_POOL_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

//==============================================================================
// Thread Pool
//==============================================================================

/// Configuration for thread pool behavior
#[derive(Debug, Clone)]
pub struct ThreadPoolConfig {
    /// Number of worker threads
    pub num_threads: usize,
    /// Maximum queue size (None for unlimited)
    pub max_queue_size: Option<usize>,
    /// Thread stack size
    pub stack_size: Option<usize>,
    /// Thread name prefix
    pub thread_name_prefix: String,
    /// Keep alive time for idle threads
    pub keep_alive: Duration,
    /// Whether to allow core threads to timeout
    pub allow_core_thread_timeout: bool,
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        Self {
            num_threads: num_cpus::get(),
            max_queue_size: None,
            stack_size: None,
            thread_name_prefix: "cursed-pool".to_string(),
            keep_alive: Duration::from_secs(60),
            allow_core_thread_timeout: false,
        }
    }
}

/// Builder for configuring thread pools
pub struct ThreadPoolBuilder {
    config: ThreadPoolConfig,
}

impl ThreadPoolBuilder {
    /// Create a new thread pool builder
    pub fn new() -> Self {
        Self {
            config: ThreadPoolConfig::default(),
        }
    }

    /// Set the number of threads
    pub fn num_threads(mut self, num_threads: usize) -> Self {
        self.config.num_threads = num_threads.max(1);
        self
    }

    /// Set the maximum queue size
    pub fn max_queue_size(mut self, max_queue_size: usize) -> Self {
        self.config.max_queue_size = Some(max_queue_size);
        self
    }

    /// Set the thread stack size
    pub fn stack_size(mut self, stack_size: usize) -> Self {
        self.config.stack_size = Some(stack_size);
        self
    }

    /// Set the thread name prefix
    pub fn thread_name_prefix(mut self, prefix: String) -> Self {
        self.config.thread_name_prefix = prefix;
        self
    }

    /// Set the keep alive time
    pub fn keep_alive(mut self, keep_alive: Duration) -> Self {
        self.config.keep_alive = keep_alive;
        self
    }

    /// Build the thread pool
    pub fn build(self) -> SyncResult<ThreadPool> {
        ThreadPool::with_config(self.config)
    }
}

impl Default for ThreadPoolBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A task to be executed by the thread pool
pub type Task = Box<dyn FnOnce() + Send + 'static>;

/// Result of a task execution
pub type TaskResult<T> = Result<T, SyncError>;

/// A thread pool for executing tasks
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    config: ThreadPoolConfig,
    active_tasks: Arc<StdAtomicUsize>,
    total_tasks: Arc<AtomicU64>,
    completed_tasks: Arc<AtomicU64>,
    shutdown: Arc<StdAtomicBool>,
}

enum Message {
    NewTask(Task),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new thread pool with default configuration
    pub fn new(num_threads: usize) -> SyncResult<Self> {
        Self::with_config(ThreadPoolConfig {
            num_threads,
            ..Default::default()
        })
    }

    /// Create a thread pool with custom configuration
    pub fn with_config(config: ThreadPoolConfig) -> SyncResult<Self> {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let active_tasks = Arc::new(StdAtomicUsize::new(0));
        let total_tasks = Arc::new(AtomicU64::new(0));
        let completed_tasks = Arc::new(AtomicU64::new(0));
        let shutdown = Arc::new(StdAtomicBool::new(false));

        let mut workers = Vec::with_capacity(config.num_threads);

        for id in 0..config.num_threads {
            let receiver = Arc::clone(&receiver);
            let active_tasks = Arc::clone(&active_tasks);
            let completed_tasks = Arc::clone(&completed_tasks);
            let shutdown = Arc::clone(&shutdown);
            let thread_name = format!("{}-{}", config.thread_name_prefix, id);

            let mut builder = thread::Builder::new().name(thread_name);
            
            if let Some(stack_size) = config.stack_size {
                builder = builder.stack_size(stack_size);
            }

            let thread = builder.spawn(move || {
                Worker::run(id, receiver, active_tasks, completed_tasks, shutdown);
            }).map_err(|e| thread_pool_error("worker", &format!("Failed to spawn worker thread: {}", e)))?;

            workers.push(Worker {
                id,
                thread: Some(thread),
            });
        }

        Ok(ThreadPool {
            workers,
            sender,
            config,
            active_tasks,
            total_tasks,
            completed_tasks,
            shutdown,
        })
    }

    /// Execute a task asynchronously
    pub fn execute<F>(&self, f: F) -> SyncResult<()>
    where
        F: FnOnce() + Send + 'static,
    {
        if self.shutdown.load(StdOrdering::Acquire) {
            return Err(thread_pool_error("execute", "Thread pool is shut down"));
        }

        let task = Box::new(f);
        
        self.sender
            .send(Message::NewTask(task))
            .map_err(|_| thread_pool_error("execute", "Failed to send task to thread pool"))?;
        
        self.total_tasks.fetch_add(1, StdOrdering::Relaxed);
        Ok(())
    }

    /// Execute a task and return a handle to wait for the result
    pub fn spawn<F, T>(&self, f: F) -> SyncResult<TaskHandle<T>>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (sender, receiver) = mpsc::channel();
        
        self.execute(move || {
            let result = f();
            let _ = sender.send(result);
        })?;

        Ok(TaskHandle { receiver })
    }

    /// Get the number of worker threads
    pub fn thread_count(&self) -> usize {
        self.config.num_threads
    }

    /// Get the number of active tasks
    pub fn active_tasks(&self) -> usize {
        self.active_tasks.load(StdOrdering::Acquire)
    }

    /// Get the total number of tasks submitted
    pub fn total_tasks(&self) -> u64 {
        self.total_tasks.load(StdOrdering::Acquire)
    }

    /// Get the total number of completed tasks
    pub fn completed_tasks(&self) -> u64 {
        self.completed_tasks.load(StdOrdering::Acquire)
    }

    /// Get the current utilization (0.0 to 1.0)
    pub fn utilization(&self) -> f64 {
        let active = self.active_tasks() as f64;
        let total = self.thread_count() as f64;
        if total > 0.0 {
            active / total
        } else {
            0.0
        }
    }

    /// Wait for all tasks to complete
    pub fn join(&self) -> SyncResult<()> {
        self.wait_for_completion(None)
    }

    /// Wait for all tasks to complete with a timeout
    pub fn join_timeout(&self, timeout: Duration) -> SyncResult<bool> {
        match self.wait_for_completion(Some(timeout)) {
            Ok(()) => Ok(true),
            Err(SyncError::TimeoutError { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    fn wait_for_completion(&self, timeout: Option<Duration>) -> SyncResult<()> {
        let start = Instant::now();
        
        loop {
            if self.active_tasks() == 0 {
                return Ok(());
            }
            
            if let Some(timeout) = timeout {
                if start.elapsed() >= timeout {
                    return Err(timeout_error("thread pool join", timeout));
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Shutdown the thread pool gracefully
    pub fn shutdown(&self) -> SyncResult<()> {
        self.shutdown.store(true, StdOrdering::Release);
        
        for _ in &self.workers {
            let _ = self.sender.send(Message::Terminate);
        }
        
        Ok(())
    }

    /// Get the configuration
    pub fn config(&self) -> &ThreadPoolConfig {
        &self.config
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let _ = self.shutdown();
        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

impl Worker {
    fn run(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        active_tasks: Arc<StdAtomicUsize>,
        completed_tasks: Arc<AtomicU64>,
        shutdown: Arc<StdAtomicBool>,
    ) {
        loop {
            let message = {
                let receiver = receiver.lock().unwrap();
                receiver.recv()
            };

            match message {
                Ok(Message::NewTask(task)) => {
                    active_tasks.fetch_add(1, StdOrdering::Relaxed);
                    task();
                    active_tasks.fetch_sub(1, StdOrdering::Relaxed);
                    completed_tasks.fetch_add(1, StdOrdering::Relaxed);
                }
                Ok(Message::Terminate) | Err(_) => {
                    break;
                }
            }

            if shutdown.load(StdOrdering::Acquire) {
                break;
            }
        }
    }
}

/// Handle for waiting on task completion
pub struct TaskHandle<T> {
    receiver: mpsc::Receiver<T>,
}

impl<T> TaskHandle<T> {
    /// Wait for the task to complete and get the result
    pub fn join(self) -> SyncResult<T> {
        self.receiver
            .recv()
            .map_err(|_| thread_pool_error("task", "Task execution failed"))
    }

    /// Try to get the result without blocking
    pub fn try_join(self) -> SyncResult<Option<T>> {
        match self.receiver.try_recv() {
            Ok(result) => Ok(Some(result)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => {
                Err(thread_pool_error("task", "Task execution failed"))
            }
        }
    }
}

//==============================================================================
// Work-Stealing Pool
//==============================================================================

/// A work-stealing thread pool implementation
pub struct WorkStealingPool {
    workers: Vec<WorkStealingWorker>,
    task_queues: Vec<Arc<Mutex<VecDeque<Task>>>>,
    shutdown: Arc<AtomicBool>,
    active_workers: Arc<AtomicUsize>,
}

struct WorkStealingWorker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    local_queue: Arc<Mutex<VecDeque<Task>>>,
}

impl WorkStealingPool {
    /// Create a new work-stealing pool
    pub fn new(num_threads: usize) -> SyncResult<Self> {
        let mut workers = Vec::with_capacity(num_threads);
        let mut task_queues = Vec::with_capacity(num_threads);
        let shutdown = Arc::new(AtomicBool::new(false));
        let active_workers = Arc::new(AtomicUsize::new(0));

        // Create task queues
        for _ in 0..num_threads {
            task_queues.push(Arc::new(Mutex::new(VecDeque::new())));
        }

        // Create workers
        for id in 0..num_threads {
            let local_queue = Arc::clone(&task_queues[id]);
            let all_queues = task_queues.clone();
            let shutdown_flag = Arc::clone(&shutdown);
            let active_workers_counter = Arc::clone(&active_workers);

            let local_queue_for_worker = Arc::clone(&local_queue);
            let thread = thread::Builder::new()
                .name(format!("work-stealing-{}", id))
                .spawn(move || {
                    WorkStealingWorker::run(
                        id,
                        local_queue_for_worker,
                        all_queues,
                        shutdown_flag,
                        active_workers_counter,
                    );
                })
                .map_err(|e| thread_pool_error("work-stealing", &format!("Failed to spawn worker: {}", e)))?;

            workers.push(WorkStealingWorker {
                id,
                thread: Some(thread),
                local_queue,
            });
        }

        Ok(Self {
            workers,
            task_queues,
            shutdown,
            active_workers,
        })
    }

    /// Submit a task to the pool
    pub fn submit<F>(&self, f: F) -> SyncResult<()>
    where
        F: FnOnce() + Send + 'static,
    {
        if self.shutdown.load(StdOrdering::Acquire) {
            return Err(thread_pool_error("work-stealing", "Pool is shut down"));
        }

        let task = Box::new(f);
        
        // Try to find the least loaded queue
        let mut min_len = usize::MAX;
        let mut target_queue = 0;
        
        for (i, queue) in self.task_queues.iter().enumerate() {
            if let Ok(q) = queue.try_lock() {
                if q.len() < min_len {
                    min_len = q.len();
                    target_queue = i;
                }
            }
        }

        let queue = &self.task_queues[target_queue];
        let mut q = queue.lock().map_err(|_| thread_pool_error("work-stealing", "Failed to lock queue"))?;
        q.push_back(task);
        
        Ok(())
    }

    /// Shutdown the pool
    pub fn shutdown(&self) -> SyncResult<()> {
        self.shutdown.store(true, StdOrdering::Release);
        Ok(())
    }

    /// Get the number of active workers
    pub fn active_workers(&self) -> usize {
        self.active_workers.load(StdOrdering::Acquire)
    }
}

impl Drop for WorkStealingPool {
    fn drop(&mut self) {
        let _ = self.shutdown();
        
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

impl WorkStealingWorker {
    fn run(
        id: usize,
        local_queue: Arc<Mutex<VecDeque<Task>>>,
        all_queues: Vec<Arc<Mutex<VecDeque<Task>>>>,
        shutdown: Arc<AtomicBool>,
        active_workers: Arc<AtomicUsize>,
    ) {
        while !shutdown.load(StdOrdering::Acquire) {
            // Try to get a task from local queue first
            if let Some(task) = Self::pop_local(&local_queue) {
                active_workers.fetch_add(1, StdOrdering::Relaxed);
                task();
                active_workers.fetch_sub(1, StdOrdering::Relaxed);
                continue;
            }

            // Try to steal work from other queues
            if let Some(task) = Self::steal_work(id, &all_queues) {
                active_workers.fetch_add(1, StdOrdering::Relaxed);
                task();
                active_workers.fetch_sub(1, StdOrdering::Relaxed);
                continue;
            }

            // No work available, sleep briefly
            thread::sleep(Duration::from_millis(1));
        }
    }

    fn pop_local(queue: &Arc<Mutex<VecDeque<Task>>>) -> Option<Task> {
        queue.lock().ok()?.pop_front()
    }

    fn steal_work(worker_id: usize, all_queues: &[Arc<Mutex<VecDeque<Task>>>]) -> Option<Task> {
        for (i, queue) in all_queues.iter().enumerate() {
            if i != worker_id {
                if let Ok(mut q) = queue.try_lock() {
                    if let Some(task) = q.pop_back() {
                        return Some(task);
                    }
                }
            }
        }
        None
    }
}

//==============================================================================
// Task Queue
//==============================================================================

/// A queue for managing tasks
pub struct TaskQueue {
    tasks: Mutex<VecDeque<Task>>,
    not_empty: CondVar,
    max_size: Option<usize>,
}

impl TaskQueue {
    /// Create a new task queue
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(VecDeque::new()),
            not_empty: CondVar::new(),
            max_size: None,
        }
    }

    /// Create a bounded task queue
    pub fn bounded(max_size: usize) -> Self {
        Self {
            tasks: Mutex::new(VecDeque::new()),
            not_empty: CondVar::new(),
            max_size: Some(max_size),
        }
    }

    /// Add a task to the queue
    pub fn push(&self, task: Task) -> SyncResult<()> {
        let mut tasks = self.tasks.lock()?;
        
        if let Some(max_size) = self.max_size {
            if tasks.len() >= max_size {
                return Err(thread_pool_error("task_queue", "Queue is full"));
            }
        }
        
        tasks.push_back(task);
        self.not_empty.notify_one();
        Ok(())
    }

    /// Get a task from the queue (blocking)
    pub fn pop(&self) -> SyncResult<Task> {
        let mut tasks = self.tasks.lock()?;
        
        while tasks.is_empty() {
            tasks = self.not_empty.wait(tasks)?;
        }
        
        Ok(tasks.pop_front().unwrap())
    }

    /// Try to get a task without blocking
    pub fn try_pop(&self) -> SyncResult<Option<Task>> {
        let mut tasks = self.tasks.lock()?;
        Ok(tasks.pop_front())
    }

    /// Get the queue length
    pub fn len(&self) -> SyncResult<usize> {
        let tasks = self.tasks.lock()?;
        Ok(tasks.len())
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> SyncResult<bool> {
        let tasks = self.tasks.lock()?;
        Ok(tasks.is_empty())
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

//==============================================================================
// Parallel Iterators
//==============================================================================

/// Trait for parallel iteration
pub trait ParallelIterator<T> {
    /// Execute a function on each element in parallel
    fn par_for_each<F>(self, f: F) -> SyncResult<()>
    where
        F: Fn(&T) + Send + Sync + 'static,
        T: Send + Sync;

    /// Map each element in parallel
    fn par_map<F, U>(self, f: F) -> SyncResult<Vec<U>>
    where
        F: Fn(&T) -> U + Send + Sync + 'static,
        T: Send + Sync,
        U: Send + 'static;

    /// Filter elements in parallel
    fn par_filter<F>(self, f: F) -> SyncResult<Vec<T>>
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
        T: Send + Sync + Clone;

    /// Reduce elements in parallel
    fn par_reduce<F>(self, identity: T, f: F) -> SyncResult<T>
    where
        F: Fn(T, T) -> T + Send + Sync + 'static,
        T: Send + Sync + Clone;
}

impl<T: 'static> ParallelIterator<T> for Vec<T> {
    fn par_for_each<F>(self, f: F) -> SyncResult<()>
    where
        F: Fn(&T) + Send + Sync + 'static,
        T: Send + Sync,
    {
        par_for_each(self, f)
    }

    fn par_map<F, U>(self, f: F) -> SyncResult<Vec<U>>
    where
        F: Fn(&T) -> U + Send + Sync + 'static,
        T: Send + Sync,
        U: Send + 'static,
    {
        par_map(self, f)
    }

    fn par_filter<F>(self, f: F) -> SyncResult<Vec<T>>
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
        T: Send + Sync + Clone,
    {
        par_filter(self, f)
    }

    fn par_reduce<F>(self, identity: T, f: F) -> SyncResult<T>
    where
        F: Fn(T, T) -> T + Send + Sync + 'static,
        T: Send + Sync + Clone,
    {
        par_reduce(self, identity, f)
    }
}

/// Execute a function on each element in parallel
pub fn par_for_each<T, F>(data: Vec<T>, f: F) -> SyncResult<()>
where
    F: Fn(&T) + Send + Sync + 'static,
    T: Send + Sync + 'static,
{
    let pool = get_global_pool()?;
    let f = Arc::new(f);
    let mut handles = Vec::new();

    for item in data {
        let f_clone = Arc::clone(&f);
        let handle = pool.spawn(move || {
            f_clone(&item);
        })?;
        handles.push(handle);
    }

    for handle in handles {
        handle.join()?;
    }

    Ok(())
}

/// Map each element in parallel
pub fn par_map<T, U, F>(data: Vec<T>, f: F) -> SyncResult<Vec<U>>
where
    F: Fn(&T) -> U + Send + Sync + 'static,
    T: Send + Sync + 'static,
    U: Send + 'static,
{
    let pool = get_global_pool()?;
    let f = Arc::new(f);
    let mut handles = Vec::new();

    for item in data {
        let f_clone = Arc::clone(&f);
        let handle = pool.spawn(move || f_clone(&item))?;
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join()?);
    }

    Ok(results)
}

/// Filter elements in parallel
pub fn par_filter<T, F>(data: Vec<T>, f: F) -> SyncResult<Vec<T>>
where
    F: Fn(&T) -> bool + Send + Sync + 'static,
    T: Send + Sync + Clone + 'static,
{
    let pool = get_global_pool()?;
    let f = Arc::new(f);
    let mut handles = Vec::new();

    for item in data {
        let f_clone = Arc::clone(&f);
        let item_clone = item.clone();
        let handle = pool.spawn(move || {
            if f_clone(&item) {
                Some(item)
            } else {
                None
            }
        })?;
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Some(item) = handle.join()? {
            results.push(item);
        }
    }

    Ok(results)
}

/// Reduce elements in parallel
pub fn par_reduce<T, F>(data: Vec<T>, identity: T, f: F) -> SyncResult<T>
where
    F: Fn(T, T) -> T + Send + Sync + 'static,
    T: Send + Sync + Clone + 'static,
{
    if data.is_empty() {
        return Ok(identity);
    }

    let pool = get_global_pool()?;
    let f = Arc::new(f);
    
    // Divide into chunks for parallel processing
    let chunk_size = (data.len() + pool.thread_count() - 1) / pool.thread_count();
    let chunks: Vec<Vec<T>> = data.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();
    
    let mut handles = Vec::new();

    for chunk in chunks {
        let f_clone = Arc::clone(&f);
        let identity_clone = identity.clone();
        let handle = pool.spawn(move || {
            chunk.into_iter().fold(identity_clone, |acc, item| f_clone(acc, item))
        })?;
        handles.push(handle);
    }

    let mut result = identity;
    for handle in handles {
        result = f(result, handle.join()?);
    }

    Ok(result)
}

/// Sort a vector in parallel
pub fn parallel_sort<T>(mut data: Vec<T>) -> SyncResult<Vec<T>>
where
    T: Ord + Send + Clone + 'static,
{
    // Simple parallel merge sort implementation
    if data.len() <= 1000 {
        data.sort();
        return Ok(data);
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    
    let pool = get_global_pool()?;
    let left_handle = pool.spawn({
        let left = left.to_vec();
        move || parallel_sort(left)
    })?;
    
    let right_sorted = parallel_sort(right.to_vec())?;
    let left_sorted = left_handle.join()??;
    
    // Merge the sorted halves
    Ok(merge_sorted(left_sorted, right_sorted))
}

fn merge_sorted<T: Ord>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();
    let mut left_item = left_iter.next();
    let mut right_item = right_iter.next();

    loop {
        match (left_item.take(), right_item.take()) {
            (Some(l), Some(r)) => {
                if l <= r {
                    result.push(l);
                    left_item = left_iter.next();
                    right_item = Some(r);
                } else {
                    result.push(r);
                    right_item = right_iter.next();
                    left_item = Some(l);
                }
            }
            (Some(l), None) => {
                result.push(l);
                result.extend(left_iter);
                break;
            }
            (None, Some(r)) => {
                result.push(r);
                result.extend(right_iter);
                break;
            }
            (None, None) => break,
        }
    }

    result
}

/// Search for an element in parallel
pub fn parallel_search<T, F>(data: Vec<T>, predicate: F) -> SyncResult<Option<T>>
where
    F: Fn(&T) -> bool + Send + Sync + 'static,
    T: Send + Sync + Clone + 'static,
{
    let pool = get_global_pool()?;
    let predicate = Arc::new(predicate);
    let found = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::new();

    let chunk_size = (data.len() + pool.thread_count() - 1) / pool.thread_count();
    
    for chunk in data.chunks(chunk_size) {
        let predicate_clone = Arc::clone(&predicate);
        let found_clone = Arc::clone(&found);
        let chunk = chunk.to_vec();
        
        let handle = pool.spawn(move || {
            for item in chunk {
                if found_clone.load(StdOrdering::Acquire) {
                    return None;
                }
                if predicate_clone(&item) {
                    found_clone.store(true, StdOrdering::Release);
                    return Some(item);
                }
            }
            None
        })?;
        handles.push(handle);
    }

    for handle in handles {
        if let Some(result) = handle.join()? {
            return Ok(Some(result));
        }
    }

    Ok(None)
}

//==============================================================================
// Scheduler Policies and Load Balancing
//==============================================================================

/// Scheduling policy for thread pools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerPolicy {
    /// First In, First Out
    Fifo,
    /// Last In, First Out
    Lifo,
    /// Work stealing
    WorkStealing,
    /// Round robin assignment
    RoundRobin,
}

/// Load balancer for distributing tasks
pub struct LoadBalancer {
    policy: SchedulerPolicy,
    next_worker: AtomicUsize,
}

impl LoadBalancer {
    /// Create a new load balancer
    pub fn new(policy: SchedulerPolicy) -> Self {
        Self {
            policy,
            next_worker: AtomicUsize::new(0),
        }
    }

    /// Select the next worker based on the policy
    pub fn select_worker(&self, num_workers: usize, _task_size_hint: Option<usize>) -> usize {
        match self.policy {
            SchedulerPolicy::RoundRobin => {
                let next = self.next_worker.fetch_add(1, StdOrdering::Relaxed);
                next % num_workers
            }
            _ => {
                // Default to round robin for now
                let next = self.next_worker.fetch_add(1, StdOrdering::Relaxed);
                next % num_workers
            }
        }
    }

    /// Get the current policy
    pub fn policy(&self) -> SchedulerPolicy {
        self.policy
    }
}

//==============================================================================
// Rayon Compatibility Layer
//==============================================================================

/// Compatibility layer for Rayon-style parallel operations
pub struct RayonCompat;

impl RayonCompat {
    /// Install global thread pool with Rayon-style interface
    pub fn install<F, R>(pool: ThreadPool, f: F) -> SyncResult<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        // This is a simplified implementation
        // In a real implementation, this would set up thread-local state
        let handle = pool.spawn(f)?;
        handle.join()
    }

    /// Join two computations
    pub fn join<A, B, RA, RB>(pool: &ThreadPool, a: A, b: B) -> SyncResult<(RA, RB)>
    where
        A: FnOnce() -> RA + Send + 'static,
        B: FnOnce() -> RB + Send + 'static,
        RA: Send + 'static,
        RB: Send + 'static,
    {
        let handle_a = pool.spawn(a)?;
        let result_b = b();
        let result_a = handle_a.join()?;
        Ok((result_a, result_b))
    }
}

//==============================================================================
// Global Thread Pool Management
//==============================================================================

/// Initialize the global thread pool
pub fn init_global_thread_pool() -> SyncResult<()> {
    let _guard = GLOBAL_POOL_MUTEX.lock().unwrap();
    
    if !GLOBAL_POOL_INITIALIZED.load(StdOrdering::Acquire) {
        unsafe {
            GLOBAL_POOL = Some(ThreadPool::new(num_cpus::get())?);
        }
        GLOBAL_POOL_INITIALIZED.store(true, StdOrdering::Release);
    }
    
    Ok(())
}

/// Get the global thread pool
pub fn get_global_pool() -> SyncResult<&'static ThreadPool> {
    if !GLOBAL_POOL_INITIALIZED.load(StdOrdering::Acquire) {
        init_global_thread_pool()?;
    }
    
    unsafe {
        GLOBAL_POOL.as_ref().ok_or_else(|| thread_pool_error("global", "Global thread pool not initialized"))
    }
}

/// Shutdown the global thread pool
pub fn shutdown_global_thread_pool() -> SyncResult<()> {
    let _guard = GLOBAL_POOL_MUTEX.lock().unwrap();
    
    unsafe {
        if let Some(pool) = GLOBAL_POOL.take() {
            pool.shutdown()?;
        }
    }
    
    GLOBAL_POOL_INITIALIZED.store(false, StdOrdering::Release);
    Ok(())
}

/// Get global thread pool utilization
pub fn get_thread_pool_utilization() -> f64 {
    if let Ok(pool) = get_global_pool() {
        pool.utilization()
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4).unwrap();
        assert_eq!(pool.thread_count(), 4);
        assert_eq!(pool.active_tasks(), 0);
    }

    #[test]
    fn test_thread_pool_execution() {
        let pool = ThreadPool::new(2).unwrap();
        let counter = Arc::new(AtomicI32::new(0));
        
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                counter_clone.fetch_add(1, StdOrdering::Relaxed);
            }).unwrap();
        }
        
        pool.join().unwrap();
        assert_eq!(counter.load(StdOrdering::Relaxed), 10);
    }

    #[test]
    fn test_thread_pool_spawn() {
        let pool = ThreadPool::new(2).unwrap();
        
        let handle = pool.spawn(|| 42).unwrap();
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_thread_pool_builder() {
        let pool = ThreadPoolBuilder::new()
            .num_threads(3)
            .thread_name_prefix("test".to_string())
            .build()
            .unwrap();
        
        assert_eq!(pool.thread_count(), 3);
        assert_eq!(pool.config().thread_name_prefix, "test");
    }

    #[test]
    fn test_work_stealing_pool() {
        let pool = WorkStealingPool::new(2).unwrap();
        let counter = Arc::new(AtomicI32::new(0));
        
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.submit(move || {
                counter_clone.fetch_add(1, StdOrdering::Relaxed);
            }).unwrap();
        }
        
        // Wait a bit for tasks to complete
        thread::sleep(Duration::from_millis(100));
        assert_eq!(counter.load(StdOrdering::Relaxed), 10);
    }

    #[test]
    fn test_task_queue() {
        let queue = TaskQueue::new();
        
        queue.push(Box::new(|| {})).unwrap();
        assert_eq!(queue.len().unwrap(), 1);
        assert!(!queue.is_empty().unwrap());
        
        let _task = queue.try_pop().unwrap();
        assert_eq!(queue.len().unwrap(), 0);
        assert!(queue.is_empty().unwrap());
    }

    #[test]
    fn test_bounded_task_queue() {
        let queue = TaskQueue::bounded(1);
        
        queue.push(Box::new(|| {})).unwrap();
        assert!(queue.push(Box::new(|| {})).is_err()); // Should fail, queue is full
    }

    #[test]
    fn test_parallel_for_each() {
        let data = vec![1, 2, 3, 4, 5];
        let counter = Arc::new(AtomicI32::new(0));
        
        {
            let counter_clone = Arc::clone(&counter);
            par_for_each(data, move |_| {
                counter_clone.fetch_add(1, StdOrdering::Relaxed);
            }).unwrap();
        }
        
        assert_eq!(counter.load(StdOrdering::Relaxed), 5);
    }

    #[test]
    fn test_parallel_map() {
        let data = vec![1, 2, 3, 4, 5];
        let results = par_map(data, |x| x * 2).unwrap();
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_parallel_filter() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let results = par_filter(data, |x| *x % 2 == 0).unwrap();
        assert_eq!(results, vec![2, 4, 6]);
    }

    #[test]
    fn test_parallel_reduce() {
        let data = vec![1, 2, 3, 4, 5];
        let sum = par_reduce(data, 0, |acc, x| acc + x).unwrap();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_parallel_sort() {
        let data = vec![5, 2, 8, 1, 9, 3];
        let sorted = parallel_sort(data).unwrap();
        assert_eq!(sorted, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_parallel_search() {
        let data = vec![1, 2, 3, 4, 5];
        let result = parallel_search(data, |x| *x == 3).unwrap();
        assert_eq!(result, Some(3));
        
        let data = vec![1, 2, 4, 5];
        let result = parallel_search(data, |x| *x == 3).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_load_balancer() {
        let balancer = LoadBalancer::new(SchedulerPolicy::RoundRobin);
        
        assert_eq!(balancer.select_worker(4, None), 0);
        assert_eq!(balancer.select_worker(4, None), 1);
        assert_eq!(balancer.select_worker(4, None), 2);
        assert_eq!(balancer.select_worker(4, None), 3);
        assert_eq!(balancer.select_worker(4, None), 0); // Wraps around
    }

    #[test]
    fn test_global_thread_pool() {
        let _ = init_global_thread_pool();
        let pool = get_global_pool().unwrap();
        assert!(pool.thread_count() > 0);
    }
}
