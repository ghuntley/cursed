use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use std::thread;

use crate::error::CursedError;
use crate::runtime::goroutine::GoroutineId;

/// Task identifier
pub type TaskId = u64;

/// Boxed future type
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Cancelled,
    Failed,
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Normal
    }
}

/// Async task wrapper
pub struct AsyncTask {
    pub id: TaskId,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub future: BoxFuture<'static, ()>,
    pub waker: Option<Waker>,
    pub created_at: Instant,
    pub last_poll: Option<Instant>,
    pub poll_count: u64,
    pub goroutine_id: Option<GoroutineId>,
}

impl AsyncTask {
    pub fn new<F>(id: TaskId, future: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Self {
            id,
            state: TaskState::Pending,
            priority: TaskPriority::default(),
            future: Box::pin(future),
            waker: None,
            created_at: Instant::now(),
            last_poll: None,
            poll_count: 0,
            goroutine_id: None,
        }
    }

    pub fn with_priority<F>(id: TaskId, future: F, priority: TaskPriority) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let mut task = Self::new(id, future);
        task.priority = priority;
        task
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.state = TaskState::Running;
        self.last_poll = Some(Instant::now());
        self.poll_count += 1;
        
        match self.future.as_mut().poll(cx) {
            Poll::Ready(result) => {
                self.state = TaskState::Completed;
                Poll::Ready(result)
            }
            Poll::Pending => {
                self.state = TaskState::Pending;
                self.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }

    pub fn cancel(&mut self) {
        self.state = TaskState::Cancelled;
    }

    pub fn fail(&mut self) {
        self.state = TaskState::Failed;
    }
}

/// Task handle for external control
pub struct TaskHandle<T> {
    pub task_id: TaskId,
    pub receiver: tokio::sync::oneshot::Receiver<T>,
}

impl<T> TaskHandle<T> {
    pub fn new(task_id: TaskId, receiver: tokio::sync::oneshot::Receiver<T>) -> Self {
        Self { task_id, receiver }
    }

    pub fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub async fn join(self) -> Result<T, CursedError> {
        self.receiver.await
            .map_err(|_| CursedError::runtime_error("Task was cancelled or failed"))
    }

    pub fn cancel(&self) {
        // Signal cancellation to executor
        if let Some(executor) = GLOBAL_EXECUTOR.get() {
            executor.cancel_task(self.task_id);
        }
    }
}

impl<T> Future for TaskHandle<T> {
    type Output = Result<T, CursedError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.receiver).poll(cx) {
            Poll::Ready(Ok(result)) => Poll::Ready(Ok(result)),
            Poll::Ready(Err(_)) => Poll::Ready(Err(CursedError::runtime_error("Task failed"))),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Executor configuration
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    pub max_threads: usize,
    pub max_blocking_threads: usize,
    pub thread_stack_size: usize,
    pub keep_alive_duration: Duration,
    pub enable_work_stealing: bool,
    pub enable_io_integration: bool,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            max_threads: num_cpus::get(),
            max_blocking_threads: 512,
            thread_stack_size: 2 * 1024 * 1024, // 2MB
            keep_alive_duration: Duration::from_secs(60),
            enable_work_stealing: true,
            enable_io_integration: true,
        }
    }
}

/// Executor statistics
#[derive(Debug, Default, Clone)]
pub struct ExecutorStats {
    pub tasks_spawned: u64,
    pub tasks_completed: u64,
    pub tasks_cancelled: u64,
    pub tasks_failed: u64,
    pub total_poll_count: u64,
    pub active_tasks: usize,
    pub queued_tasks: usize,
    pub worker_threads: usize,
    pub blocking_threads: usize,
    pub work_steals: u64,
    pub uptime: Duration,
    pub started_at: Option<Instant>,
}

/// Work-stealing async executor
pub struct AsyncExecutor {
    config: ExecutorConfig,
    next_task_id: AtomicU64,
    tasks: Arc<Mutex<HashMap<TaskId, AsyncTask>>>,
    ready_queue: Arc<Mutex<VecDeque<TaskId>>>,
    high_priority_queue: Arc<Mutex<VecDeque<TaskId>>>,
    tokio_handle: Option<tokio::runtime::Handle>,
    stats: Arc<Mutex<ExecutorStats>>,
    shutdown: Arc<std::sync::atomic::AtomicBool>,
}

impl AsyncExecutor {
    pub fn new() -> Result<Self, CursedError> {
        Self::with_config(ExecutorConfig::default())
    }

    pub fn with_config(config: ExecutorConfig) -> Result<Self, CursedError> {
        // Try to use current runtime handle instead of creating new runtime
        let tokio_handle = match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                // We're already in a tokio runtime, use it
                Some(handle)
            }
            Err(_) => {
                // No current runtime, we'll need to create one when needed
                // This should not happen in the CURSED executor since main uses #[tokio::main]
                None
            }
        };

        let mut stats = ExecutorStats::default();
        stats.started_at = Some(Instant::now());

        Ok(Self {
            config,
            next_task_id: AtomicU64::new(1),
            tasks: Arc::new(Mutex::new(HashMap::new())),
            ready_queue: Arc::new(Mutex::new(VecDeque::new())),
            high_priority_queue: Arc::new(Mutex::new(VecDeque::new())),
            tokio_handle,
            stats: Arc::new(Mutex::new(stats)),
            shutdown: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    /// Spawn a future on the executor
    pub fn spawn<F, T>(&self, future: F) -> TaskHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.spawn_with_priority(future, TaskPriority::Normal)
    }

    /// Spawn a future with specific priority
    pub fn spawn_with_priority<F, T>(&self, future: F, priority: TaskPriority) -> TaskHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let task_id = self.next_task_id.fetch_add(1, Ordering::SeqCst);
        let (sender, receiver) = tokio::sync::oneshot::channel();

        let wrapped_future = async move {
            let result = future.await;
            let _ = sender.send(result);
        };

        let task = AsyncTask::with_priority(task_id, wrapped_future, priority);

        // Add to task storage
        self.tasks.lock().unwrap().insert(task_id, task);

        // Queue for execution
        match priority {
            TaskPriority::High | TaskPriority::Critical => {
                self.high_priority_queue.lock().unwrap().push_back(task_id);
            }
            _ => {
                self.ready_queue.lock().unwrap().push_back(task_id);
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.tasks_spawned += 1;
            stats.active_tasks += 1;
        }

        // Wake up executor if needed
        self.wake_executor();

        TaskHandle::new(task_id, receiver)
    }

    /// Spawn a blocking task
    pub fn spawn_blocking<F, T>(&self, f: F) -> TaskHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let task_id = self.next_task_id.fetch_add(1, Ordering::SeqCst);
        let (sender, receiver) = tokio::sync::oneshot::channel();

        let future = async move {
            let result = tokio::task::spawn_blocking(f).await;
            match result {
                Ok(value) => {
                    let _ = sender.send(value);
                }
                Err(_) => {
                    // Task was cancelled or panicked
                }
            }
        };

        let task = AsyncTask::new(task_id, future);
        self.tasks.lock().unwrap().insert(task_id, task);
        self.ready_queue.lock().unwrap().push_back(task_id);

        {
            let mut stats = self.stats.lock().unwrap();
            stats.tasks_spawned += 1;
            stats.active_tasks += 1;
        }

        self.wake_executor();
        TaskHandle::new(task_id, receiver)
    }

    /// Block on a future until completion
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        if let Some(handle) = &self.tokio_handle {
            // Use current runtime - cannot call block_on from within a runtime
            // This should not be called from within an async context
            eprintln!("CURSED Runtime Error: Cannot call block_on from within an async runtime context");
            eprintln!("This indicates a design issue in the async code.");
            std::process::exit(1);
        } else {
            // Create a temporary runtime if no current handle
            let rt = tokio::runtime::Runtime::new()
                .expect("Failed to create temporary runtime");
            rt.block_on(future)
        }
    }

    /// Enter the runtime context
    pub fn enter(&self) -> Option<tokio::runtime::EnterGuard<'_>> {
        // Cannot create EnterGuard from Handle, and we're already in the runtime context
        // This method is no longer needed with the new implementation
        None
    }

    /// Run the executor's main loop
    pub fn run(&self) -> Result<(), CursedError> {
        // For now, we'll use a simpler approach that doesn't require complex async task management
        // The full implementation would use a more sophisticated task polling mechanism
        Ok(())
    }

    fn execute_task(
        task_id: TaskId,
        tasks: &Arc<Mutex<HashMap<TaskId, AsyncTask>>>,
        stats: &Arc<Mutex<ExecutorStats>>,
        waker_cache: &mut HashMap<TaskId, Waker>,
    ) {
        let waker = waker_cache.entry(task_id)
            .or_insert_with(|| futures::task::noop_waker());

        let mut context = Context::from_waker(waker);

        // Get and poll the task
        let poll_result = {
            let mut tasks_guard = tasks.lock().unwrap();
            if let Some(task) = tasks_guard.get_mut(&task_id) {
                task.poll(&mut context)
            } else {
                return; // Task was removed
            }
        };

        // Update statistics
        {
            let mut stats_guard = stats.lock().unwrap();
            stats_guard.total_poll_count += 1;
        }

        // Handle poll result
        match poll_result {
            Poll::Ready(()) => {
                // Task completed, remove it
                tasks.lock().unwrap().remove(&task_id);
                waker_cache.remove(&task_id);
                
                let mut stats_guard = stats.lock().unwrap();
                stats_guard.tasks_completed += 1;
                stats_guard.active_tasks = stats_guard.active_tasks.saturating_sub(1);
            }
            Poll::Pending => {
                // Task is still pending, it will be woken up when ready
            }
        }
    }

    /// Cancel a task
    pub fn cancel_task(&self, task_id: TaskId) {
        if let Some(task) = self.tasks.lock().unwrap().get_mut(&task_id) {
            task.cancel();
        }
        
        let mut stats = self.stats.lock().unwrap();
        stats.tasks_cancelled += 1;
        stats.active_tasks = stats.active_tasks.saturating_sub(1);
    }

    /// Get executor statistics
    pub fn get_stats(&self) -> ExecutorStats {
        let mut stats = self.stats.lock().unwrap().clone();
        stats.queued_tasks = self.ready_queue.lock().unwrap().len() + 
                           self.high_priority_queue.lock().unwrap().len();
        stats.active_tasks = self.tasks.lock().unwrap().len();
        
        if let Some(started_at) = stats.started_at {
            stats.uptime = started_at.elapsed();
        }
        
        stats
    }

    /// Shutdown the executor
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        
        // Cancel all remaining tasks
        let task_ids: Vec<TaskId> = self.tasks.lock().unwrap().keys().cloned().collect();
        for task_id in task_ids {
            self.cancel_task(task_id);
        }
    }

    /// Wake up the executor (internal use)
    fn wake_executor(&self) {
        // The tokio runtime handles task waking internally
        // This method is here for potential future use
    }
}

/// Global executor instance
static GLOBAL_EXECUTOR: once_cell::sync::OnceCell<Arc<AsyncExecutor>> = once_cell::sync::OnceCell::new();

/// Initialize the global async executor
pub fn initialize_global_executor() -> Result<(), CursedError> {
    initialize_global_executor_with_config(ExecutorConfig::default())
}

/// Initialize the global executor with custom configuration
pub fn initialize_global_executor_with_config(config: ExecutorConfig) -> Result<(), CursedError> {
    let executor = Arc::new(AsyncExecutor::with_config(config)?);
    
    GLOBAL_EXECUTOR
        .set(executor.clone())
        .map_err(|_| CursedError::runtime_error("Global executor already initialized"))?;

    executor.run()?;
    Ok(())
}

/// Get the global async executor
pub fn get_global_executor() -> Option<Arc<AsyncExecutor>> {
    GLOBAL_EXECUTOR.get().cloned()
}

/// Shutdown the global executor
pub fn shutdown_global_executor() {
    if let Some(executor) = get_global_executor() {
        executor.shutdown();
    }
}

/// Spawn a future on the global executor
pub fn spawn<F, T>(future: F) -> Result<TaskHandle<T>, CursedError>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    get_global_executor()
        .map(|executor| executor.spawn(future))
        .ok_or_else(|| CursedError::runtime_error("Global executor not initialized"))
}

/// Spawn a blocking task on the global executor
pub fn spawn_blocking<F, T>(f: F) -> Result<TaskHandle<T>, CursedError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    get_global_executor()
        .map(|executor| executor.spawn_blocking(f))
        .ok_or_else(|| CursedError::runtime_error("Global executor not initialized"))
}

/// Block on a future using the global executor
pub fn block_on<F>(future: F) -> Result<F::Output, CursedError>
where
    F: Future,
{
    get_global_executor()
        .map(|executor| executor.block_on(future))
        .ok_or_else(|| CursedError::runtime_error("Global executor not initialized"))
}
