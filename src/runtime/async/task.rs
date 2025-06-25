use crate::error::CursedError;
/// Task management for async runtime
use std::sync::{Arc, Mutex, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::task::{Context, Poll, Waker};
use std::pin::Pin;
use std::time::{Instant, Duration};
use crate::runtime::r#async::{Future, FutureError};

/// Global task ID counter
static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a unique task ID
fn next_task_id() -> TaskId {
    TaskId(TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst))
/// Unique identifier for a task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

impl TaskId {
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task({})", self.0)
    }
}

/// Task state enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    /// Task is created but not yet started
    /// Task is ready to run
    /// Task is currently running
    /// Task is waiting for something (I/O, timer, etc.)
    /// Task completed successfully
    /// Task failed with an error
    /// Task was cancelled
/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Normal
    }
}

/// Task execution context
pub struct TaskContext {
impl TaskContext {
    pub fn new(task_id: TaskId, priority: TaskPriority) -> Self {
        Self {
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(Instant::now());
    pub fn running_time(&self) -> Option<Duration> {
        self.started_at.map(|start| start.elapsed())
    pub fn total_time(&self) -> Duration {
        self.created_at.elapsed()
    }
}

/// Task waker for notifying the scheduler when a task is ready
pub struct TaskWaker {
impl TaskWaker {
    /// Create a new task waker with scheduler integration
    pub fn new(task_id: TaskId, scheduler_waker: Arc<dyn Fn(TaskId) + Send + Sync>) -> Self {
        Self {
        }
    }

    /// Create a new task waker with event loop integration
    pub fn new_with_event_loop(
        event_loop_data: Arc<crate::runtime::r#async::event_loop::EventLoopWakerData>
    ) -> Self {
        let task_id = TaskId(0); // Will be set by event loop data
        Self {
            scheduler_waker: Arc::new(|_| {}), // No-op scheduler waker
        }
    }

    /// Convert to std::task::Waker
    pub fn into_waker(self) -> std::task::Waker {
        std::task::Waker::from(Arc::new(self))
    }
}

impl std::task::Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        if let Some(ref event_loop_data) = self.event_loop_data {
            event_loop_data.wake();
        } else {
            (self.scheduler_waker)(self.task_id);
        }
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(ref event_loop_data) = self.event_loop_data {
            event_loop_data.wake();
        } else {
            (self.scheduler_waker)(self.task_id);
        }
    }
/// A spawned task that can be awaited
pub struct Task<T> {
impl<T> Task<T> {
    pub fn new(
    ) -> Self {
        let id = next_task_id();
        let context = TaskContext::new(id, priority);

        Self {
        }
    }

    pub fn id(&self) -> TaskId {
        self.id
    pub fn state(&self) -> &TaskState {
        &self.state
    pub fn priority(&self) -> TaskPriority {
        self.context.priority
    pub fn context(&self) -> &TaskContext {
        &self.context
    pub fn context_mut(&mut self) -> &mut TaskContext {
        &mut self.context
    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    /// Poll the task's future
    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<T> {
        if self.state == TaskState::Created {
            self.state = TaskState::Running;
            self.context.start();
        } else if self.state != TaskState::Running && self.state != TaskState::Waiting {
            return Poll::Pending;
        self.state = TaskState::Running;
        self.context.waker = Some(cx.waker().clone());

        match self.future.as_mut().poll(cx) {
            Poll::Ready(result) => {
                self.state = TaskState::Completed;
                Poll::Ready(result)
            }
            Poll::Pending => {
                self.state = TaskState::Waiting;
                Poll::Pending
            }
        }
    /// Cancel the task
    pub fn cancel(&mut self) {
        if !matches!(self.state, TaskState::Completed | TaskState::Failed | TaskState::Cancelled) {
            self.state = TaskState::Cancelled;
        }
    }

    /// Check if the task is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.state == TaskState::Cancelled
    /// Check if the task is completed
    pub fn is_completed(&self) -> bool {
        matches!(self.state, TaskState::Completed | TaskState::Failed | TaskState::Cancelled)
    }
}

/// Handle to a spawned task that allows waiting for completion
pub struct TaskHandle<T> {
struct TaskHandleState<T> {
impl<T> TaskHandle<T> {
    pub fn new(task_id: TaskId) -> (Self, TaskHandleNotifier<T>) {
        let shared_state = Arc::new(Mutex::new(TaskHandleState {
        }));

        let cancelled = Arc::new(AtomicBool::new(false));

        let handle = TaskHandle {

        let notifier = TaskHandleNotifier {

        (handle, notifier)
    /// Create a placeholder handle for when runtime is not available
    pub fn placeholder() -> Self {
        let (handle, _notifier) = Self::new(TaskId(0));
        handle
    pub fn task_id(&self) -> TaskId {
        self.task_id
    /// Cancel the task
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        // Wake any waiting futures
        let mut state = self.shared_state.lock().unwrap();
        for waker in state.wakers.drain(..) {
            waker.wake();
        }
    }

    /// Check if the task is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    /// Try to get the result without waiting
    pub fn try_result(&self) -> Option<crate::error::Result<()>>
    where
    {
        let state = self.shared_state.lock().unwrap();
        if state.completed {
            state.result.clone()
        } else {
            None
        }
    }

    /// Check if the task is completed
    pub fn is_completed(&self) -> bool {
        let state = self.shared_state.lock().unwrap();
        state.completed
    }
}

impl<T> Clone for TaskHandle<T> {
    fn clone(&self) -> Self {
        Self {
        }
    }
impl<T: Clone> Future for TaskHandle<T> {
    type Output = crate::error::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.shared_state.lock().unwrap();

        if self.is_cancelled() {
            return Poll::Ready(Err(FutureError::Cancelled));
        if state.completed {
            if let Some(result) = &state.result {
                Poll::Ready(result.clone())
            } else {
                Poll::Ready(Err(FutureError::InvalidState))
            }
        } else {
            // Add waker to be notified when completed
            state.wakers.push(cx.waker().clone());
            Poll::Pending
        }
    }
/// Notifier for task completion
pub struct TaskHandleNotifier<T> {
impl<T> TaskHandleNotifier<T> {
    /// Notify that the task completed successfully
    pub fn complete(&self, result: T) {
        let mut state = self.shared_state.lock().unwrap();
        if !state.completed && !self.cancelled.load(Ordering::SeqCst) {
            state.result = Some(Ok(result));
            state.completed = true;
            
            // Wake all waiting futures
            for waker in state.wakers.drain(..) {
                waker.wake();
            }
        }
    /// Notify that the task failed
    pub fn fail(&self, error: FutureError) {
        let mut state = self.shared_state.lock().unwrap();
        if !state.completed {
            state.result = Some(Err(error));
            state.completed = true;
            
            // Wake all waiting futures
            for waker in state.wakers.drain(..) {
                waker.wake();
            }
        }
    /// Check if cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

/// Task statistics for monitoring
#[derive(Debug, Clone)]
pub struct TaskStatistics {
impl Default for TaskStatistics {
    fn default() -> Self {
        Self {
        }
    }
impl TaskStatistics {
    pub fn update_average_execution_time(&mut self) {
        if self.completed_tasks > 0 {
            self.average_execution_time = self.total_execution_time / self.completed_tasks as u32;
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_tasks == 0 {
            0.0
        } else {
            self.completed_tasks as f64 / self.total_tasks as f64
        }
    }

    pub fn failure_rate(&self) -> f64 {
        if self.total_tasks == 0 {
            0.0
        } else {
            self.failed_tasks as f64 / self.total_tasks as f64
        }
    }
/// Task manager for tracking and managing tasks
pub struct TaskManager {
impl TaskManager {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn register_task<T: Send + 'static>(&self, task: Task<T>) -> TaskId {
        let task_id = task.id();
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(task_id, Arc::new(Mutex::new(task)));

        // Update statistics
        let mut stats = self.statistics.lock().unwrap();
        stats.total_tasks += 1;

        task_id
    pub fn remove_task(&self, task_id: TaskId) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(&task_id);
    pub fn task_count(&self) -> usize {
        let tasks = self.tasks.lock().unwrap();
        tasks.len()
    pub fn get_statistics(&self) -> TaskStatistics {
        let stats = self.statistics.lock().unwrap();
        stats.clone()
    pub fn update_task_completion(&self, execution_time: Duration) {
        let mut stats = self.statistics.lock().unwrap();
        stats.completed_tasks += 1;
        stats.total_execution_time += execution_time;
        stats.update_average_execution_time();
    pub fn update_task_failure(&self) {
        let mut stats = self.statistics.lock().unwrap();
        stats.failed_tasks += 1;
    pub fn update_task_cancellation(&self) {
        let mut stats = self.statistics.lock().unwrap();
        stats.cancelled_tasks += 1;
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
