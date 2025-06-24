use crate::error::Error;
/// Async executor implementation for CURSED runtime
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, Condvar};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant};
use std::pin::Pin;

use crate::runtime::r#async::{
    Future, Task, TaskId, TaskHandle, TaskHandleNotifier, TaskState, TaskPriority,
    TaskStatistics, TaskManager, TaskWaker
};

/// Configuration for the async executor
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Number of worker threads
    pub worker_threads: usize,
    /// Maximum number of tasks in queue before blocking
    pub max_queue_size: usize,
    /// Enable work stealing between threads
    pub enable_work_stealing: bool,
    /// Task poll timeout
    pub task_timeout: Duration,
    /// Statistics collection interval
    pub stats_interval: Duration,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get().max(2),
            max_queue_size: 10000,
            enable_work_stealing: true,
            task_timeout: Duration::from_millis(100),
            stats_interval: Duration::from_secs(1),
        }
    }
}

/// Statistics for executor performance monitoring
#[derive(Debug, Clone)]
pub struct ExecutorStatistics {
    pub tasks_executed: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub tasks_cancelled: u64,
    pub average_task_time: Duration,
    pub total_execution_time: Duration,
    pub queue_size: usize,
    pub active_workers: usize,
    pub idle_workers: usize,
}

impl Default for ExecutorStatistics {
    fn default() -> Self {
        Self {
            tasks_executed: 0,
            tasks_completed: 0,
            tasks_failed: 0,
            tasks_cancelled: 0,
            average_task_time: Duration::ZERO,
            total_execution_time: Duration::ZERO,
            queue_size: 0,
            active_workers: 0,
            idle_workers: 0,
        }
    }
}

/// Task queue with priority support
pub struct TaskQueue {
    high_priority: VecDeque<TaskEntry>,
    normal_priority: VecDeque<TaskEntry>,
    low_priority: VecDeque<TaskEntry>,
    total_size: usize,
}

struct TaskEntry {
    task_id: TaskId,
    task: Box<dyn FnOnce() + Send>,
    priority: TaskPriority,
    created_at: Instant,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            high_priority: VecDeque::new(),
            normal_priority: VecDeque::new(),
            low_priority: VecDeque::new(),
            total_size: 0,
        }
    }

    pub fn push(&mut self, entry: TaskEntry) {
        match entry.priority {
            TaskPriority::Critical | TaskPriority::High => {
                self.high_priority.push_back(entry);
            }
            TaskPriority::Normal => {
                self.normal_priority.push_back(entry);
            }
            TaskPriority::Low => {
                self.low_priority.push_back(entry);
            }
        }
        self.total_size += 1;
    }

    pub fn pop(&mut self) -> Option<TaskEntry> {
        // Check high priority first
        if let Some(entry) = self.high_priority.pop_front() {
            self.total_size -= 1;
            return Some(entry);
        }

        // Then normal priority
        if let Some(entry) = self.normal_priority.pop_front() {
            self.total_size -= 1;
            return Some(entry);
        }

        // Finally low priority
        if let Some(entry) = self.low_priority.pop_front() {
            self.total_size -= 1;
            return Some(entry);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.total_size
    }

    pub fn is_empty(&self) -> bool {
        self.total_size == 0
    }
}

/// Work-stealing task queue for load balancing
pub struct WorkStealingQueue {
    local_queue: VecDeque<TaskEntry>,
    steal_queue: VecDeque<TaskEntry>,
}

impl WorkStealingQueue {
    pub fn new() -> Self {
        Self {
            local_queue: VecDeque::new(),
            steal_queue: VecDeque::new(),
        }
    }

    pub fn push_local(&mut self, entry: TaskEntry) {
        self.local_queue.push_back(entry);
    }

    pub fn pop_local(&mut self) -> Option<TaskEntry> {
        self.local_queue.pop_front()
    }

    pub fn steal(&mut self) -> Option<TaskEntry> {
        // Try to steal from the back of local queue
        if let Some(entry) = self.local_queue.pop_back() {
            return Some(entry);
        }
        
        // Then try steal queue
        self.steal_queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.local_queue.len() + self.steal_queue.len()
    }
}

/// Async executor for running futures
pub struct AsyncExecutor {
    config: ExecutorConfig,
    task_queue: Arc<Mutex<TaskQueue>>,
    work_stealing_queues: Arc<Mutex<HashMap<ThreadId, WorkStealingQueue>>>,
    task_manager: TaskManager,
    statistics: Arc<Mutex<ExecutorStatistics>>,
    worker_handles: Vec<thread::JoinHandle<()>>,
    shutdown_signal: Arc<Mutex<bool>>,
    task_ready_condvar: Arc<Condvar>,
    running_tasks: Arc<Mutex<HashMap<TaskId, Box<dyn std::any::Any + Send>>>>,
}

impl AsyncExecutor {
    /// Create a new async executor with the given configuration
    pub fn new(config: ExecutorConfig) -> Self {
        Self {
            config,
            task_queue: Arc::new(Mutex::new(TaskQueue::new())),
            work_stealing_queues: Arc::new(Mutex::new(HashMap::new())),
            task_manager: TaskManager::new(),
            statistics: Arc::new(Mutex::new(ExecutorStatistics::default())),
            worker_handles: Vec::new(),
            shutdown_signal: Arc::new(Mutex::new(false)),
            task_ready_condvar: Arc::new(Condvar::new()),
            running_tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Start the executor with worker threads
    pub fn start(&mut self) {
        for worker_id in 0..self.config.worker_threads {
            let task_queue = self.task_queue.clone();
            let work_stealing_queues = self.work_stealing_queues.clone();
            let shutdown_signal = self.shutdown_signal.clone();
            let task_ready_condvar = self.task_ready_condvar.clone();
            let statistics = self.statistics.clone();
            let running_tasks = self.running_tasks.clone();
            let config = self.config.clone();

            let handle = thread::Builder::new()
                .name(format!("async-worker-{}", worker_id))
                .spawn(move || {
                    Self::worker_thread(
                        worker_id,
                        task_queue,
                        work_stealing_queues,
                        shutdown_signal,
                        task_ready_condvar,
                        statistics,
                        running_tasks,
                        config,
                    );
                })
                .expect("Failed to spawn worker thread");

            self.worker_handles.push(handle);
        }
    }

    /// Spawn a future on the executor
    pub fn spawn<F>(&self, future: F) -> TaskHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let priority = TaskPriority::Normal;
        let task = Task::new(Box::pin(future), priority);
        let task_id = task.id();

        let (handle, notifier) = TaskHandle::new(task_id);

        // Create task entry
        let task_entry = TaskEntry {
            task_id,
            task: Box::new(move || {
                Self::execute_task(task, notifier);
            }),
            priority,
            created_at: Instant::now(),
        };

        // Add to queue
        let mut queue = self.task_queue.lock().unwrap();
        queue.push(task_entry);
        
        // Notify workers
        self.task_ready_condvar.notify_one();

        handle
    }

    /// Spawn a future with specific priority
    pub fn spawn_with_priority<F>(&self, future: F, priority: TaskPriority) -> TaskHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let task = Task::new(Box::pin(future), priority);
        let task_id = task.id();

        let (handle, notifier) = TaskHandle::new(task_id);

        let task_entry = TaskEntry {
            task_id,
            task: Box::new(move || {
                Self::execute_task(task, notifier);
            }),
            priority,
            created_at: Instant::now(),
        };

        let mut queue = self.task_queue.lock().unwrap();
        queue.push(task_entry);
        
        self.task_ready_condvar.notify_one();

        handle
    }

    /// Block on a future until completion
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        let mut task = Task::new(Box::pin(future), TaskPriority::High);
        
        // Create a simple waker that does nothing (blocking execution)
        let waker = self.create_noop_waker();
        let mut context = Context::from_waker(&waker);

        loop {
            match task.poll(&mut context) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    // In a real implementation, we might yield to other tasks here
                    thread::yield_now();
                }
            }
        }
    }

    /// Get current executor statistics
    pub fn statistics(&self) -> ExecutorStatistics {
        let stats = self.statistics.lock().unwrap();
        let mut result = stats.clone();
        
        // Update queue size
        let queue = self.task_queue.lock().unwrap();
        result.queue_size = queue.len();
        
        result
    }

    /// Shutdown the executor
    pub fn shutdown(&mut self) {
        // Signal shutdown
        {
            let mut shutdown = self.shutdown_signal.lock().unwrap();
            *shutdown = true;
        }
        
        // Notify all workers
        self.task_ready_condvar.notify_all();

        // Wait for workers to finish
        for handle in self.worker_handles.drain(..) {
            let _ = handle.join();
        }
    }

    /// Worker thread main loop
    fn worker_thread(
        worker_id: usize,
        task_queue: Arc<Mutex<TaskQueue>>,
        work_stealing_queues: Arc<Mutex<HashMap<ThreadId, WorkStealingQueue>>>,
        shutdown_signal: Arc<Mutex<bool>>,
        task_ready_condvar: Arc<Condvar>,
        statistics: Arc<Mutex<ExecutorStatistics>>,
        running_tasks: Arc<Mutex<HashMap<TaskId, Box<dyn std::any::Any + Send>>>>,
        config: ExecutorConfig,
    ) {
        let thread_id = thread::current().id();
        
        // Initialize work stealing queue for this thread
        {
            let mut queues = work_stealing_queues.lock().unwrap();
            queues.insert(thread_id, WorkStealingQueue::new());
        }

        loop {
            // Check for shutdown
            {
                let shutdown = shutdown_signal.lock().unwrap();
                if *shutdown {
                    break;
                }
            }

            // Try to get a task
            let task_entry = {
                let mut queue = task_queue.lock().unwrap();
                
                if queue.is_empty() {
                    // Wait for tasks if none available
                    let _queue = task_ready_condvar
                        .wait_timeout(queue, config.task_timeout)
                        .unwrap().0;
                    
                    // Check again after wait
                    queue.pop()
                } else {
                    queue.pop()
                }
            };

            if let Some(entry) = task_entry {
                let start_time = Instant::now();
                
                // Execute the task
                (entry.task)();
                
                let execution_time = start_time.elapsed();
                
                // Update statistics
                {
                    let mut stats = statistics.lock().unwrap();
                    stats.tasks_executed += 1;
                    stats.total_execution_time += execution_time;
                    
                    if stats.tasks_executed > 0 {
                        stats.average_task_time = stats.total_execution_time / stats.tasks_executed as u32;
                    }
                }
            } else if config.enable_work_stealing {
                // Try work stealing
                Self::try_work_stealing(&work_stealing_queues, thread_id);
            }
        }
    }

    /// Execute a single task
    fn execute_task<T>(mut task: Task<T>, notifier: TaskHandleNotifier<T>)
    where
        T: Send + 'static,
    {
        // Check if cancelled before starting
        if notifier.is_cancelled() {
            notifier.fail(crate::runtime::r#async::FutureError::Cancelled);
            return;
        }

        // Create a simple waker for this task
        let waker = create_simple_waker();
        let mut context = Context::from_waker(&waker);

        // Poll the task
        match task.poll(&mut context) {
            Poll::Ready(result) => {
                notifier.complete(result);
            }
            Poll::Pending => {
                // Task is not ready, it should be rescheduled by the waker
                // For now, we'll just fail it since we don't have a scheduler integration
                notifier.fail(crate::runtime::r#async::FutureError::Failed(
                    "Task yielded but scheduler not integrated".to_string()
                ));
            }
        }
    }

    /// Try to steal work from other threads
    fn try_work_stealing(
        work_stealing_queues: &Arc<Mutex<HashMap<ThreadId, WorkStealingQueue>>>,
        current_thread: ThreadId,
    ) {
        let mut queues = work_stealing_queues.lock().unwrap();
        
        // Try to steal from other threads
        for (thread_id, queue) in queues.iter_mut() {
            if *thread_id != current_thread {
                if let Some(_entry) = queue.steal() {
                    // Execute stolen task
                    // For now, we'll just drop it since we need more integration
                    break;
                }
            }
        }
    }

    /// Create a no-op waker for blocking execution
    fn create_noop_waker(&self) -> Waker {
        create_simple_waker()
    }
}

/// Create a simple waker implementation
fn create_simple_waker() -> Waker {
    fn raw_waker_clone(_data: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &SIMPLE_WAKER_VTABLE)
    }

    fn raw_waker_wake(_data: *const ()) {
        // No-op for simple waker
    }

    fn raw_waker_wake_by_ref(_data: *const ()) {
        // No-op for simple waker
    }

    fn raw_waker_drop(_data: *const ()) {
        // No-op for simple waker
    }

    static SIMPLE_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        raw_waker_clone,
        raw_waker_wake,
        raw_waker_wake_by_ref,
        raw_waker_drop,
    );

    let raw_waker = RawWaker::new(std::ptr::null(), &SIMPLE_WAKER_VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

impl Drop for AsyncExecutor {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Utility function to run a future to completion on a new executor
pub fn run<F>(future: F) -> F::Output
where
    F: Future,
{
    let executor = AsyncExecutor::new(ExecutorConfig::default());
    executor.block_on(future)
}

/// Utility function to spawn a future on a new executor
pub fn spawn<F>(future: F) -> TaskHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let executor = AsyncExecutor::new(ExecutorConfig::default());
    executor.spawn(future)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::r#async::future::ready;

    #[test]
    fn test_executor_creation() {
        let config = ExecutorConfig::default();
        let executor = AsyncExecutor::new(config);
        
        let stats = executor.statistics();
        assert_eq!(stats.tasks_executed, 0);
    }

    #[test]
    fn test_block_on_simple_future() {
        let executor = AsyncExecutor::new(ExecutorConfig::default());
        let result = executor.block_on(ready(42));
        assert_eq!(result, 42);
    }

    #[test]
    fn test_task_queue_priority() {
        let mut queue = TaskQueue::new();
        
        let high_task = TaskEntry {
            task_id: TaskId(1),
            task: Box::new(|| {}),
            priority: TaskPriority::High,
            created_at: Instant::now(),
        };
        
        let low_task = TaskEntry {
            task_id: TaskId(2),
            task: Box::new(|| {}),
            priority: TaskPriority::Low,
            created_at: Instant::now(),
        };
        
        queue.push(low_task);
        queue.push(high_task);
        
        // High priority should come out first
        let first = queue.pop().unwrap();
        assert_eq!(first.task_id.as_u64(), 1);
    }
}
