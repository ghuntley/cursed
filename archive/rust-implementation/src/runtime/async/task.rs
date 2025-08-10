use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::error::CursedError;
use crate::runtime::goroutine::{GoroutineId, GoroutineScheduler};

use super::executor::{TaskId, TaskPriority, TaskState};

/// Task metadata and context
#[derive(Debug, Clone)]
pub struct TaskMetadata {
    pub name: Option<String>,
    pub created_at: Instant,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
    pub priority: TaskPriority,
    pub parent_task_id: Option<TaskId>,
    pub goroutine_id: Option<GoroutineId>,
    pub poll_count: u64,
    pub total_runtime: Duration,
    pub last_yield: Option<Instant>,
}

impl TaskMetadata {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name,
            created_at: Instant::now(),
            started_at: None,
            completed_at: None,
            priority: TaskPriority::Normal,
            parent_task_id: None,
            goroutine_id: None,
            poll_count: 0,
            total_runtime: Duration::default(),
            last_yield: None,
        }
    }

    pub fn set_started(&mut self) {
        self.started_at = Some(Instant::now());
    }

    pub fn set_completed(&mut self) {
        self.completed_at = Some(Instant::now());
        if let Some(started) = self.started_at {
            self.total_runtime = self.completed_at.unwrap() - started;
        }
    }

    pub fn increment_poll_count(&mut self) {
        self.poll_count += 1;
    }

    pub fn record_yield(&mut self) {
        self.last_yield = Some(Instant::now());
    }
}

/// Task context for execution environment
pub struct TaskContext {
    pub task_id: TaskId,
    pub metadata: Arc<Mutex<TaskMetadata>>,
    pub waker: Option<Waker>,
    pub local_storage: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    pub cancellation_token: Arc<CancellationToken>,
}

impl TaskContext {
    pub fn new(task_id: TaskId, name: Option<String>) -> Self {
        Self {
            task_id,
            metadata: Arc::new(Mutex::new(TaskMetadata::new(name))),
            waker: None,
            local_storage: HashMap::new(),
            cancellation_token: Arc::new(CancellationToken::new()),
        }
    }

    pub fn set_waker(&mut self, waker: Waker) {
        self.waker = Some(waker);
    }

    pub fn wake(&self) {
        if let Some(waker) = &self.waker {
            waker.wake_by_ref();
        }
    }

    pub fn store_local<T>(&mut self, key: String, value: T)
    where
        T: std::any::Any + Send + Sync,
    {
        self.local_storage.insert(key, Box::new(value));
    }

    pub fn get_local<T>(&self, key: &str) -> Option<&T>
    where
        T: std::any::Any + Send + Sync,
    {
        self.local_storage
            .get(key)?
            .downcast_ref::<T>()
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancellation_token.is_cancelled()
    }

    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }
}

/// Cancellation token for cooperative task cancellation
pub struct CancellationToken {
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    callbacks: Arc<Mutex<Vec<Box<dyn FnOnce() + Send + 'static>>>>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub fn cancel(&self) {
        if !self.cancelled.swap(true, Ordering::SeqCst) {
            // Execute cancellation callbacks
            let callbacks = {
                let mut callbacks_guard = self.callbacks.lock().unwrap();
                std::mem::take(&mut *callbacks_guard)
            };

            for callback in callbacks {
                callback();
            }
        }
    }

    pub fn on_cancelled<F>(&self, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if self.is_cancelled() {
            callback();
        } else {
            self.callbacks.lock().unwrap().push(Box::new(callback));
        }
    }

    pub fn child_token(&self) -> CancellationToken {
        let child = CancellationToken::new();
        let child_clone = child.clone();
        
        self.on_cancelled(move || {
            child_clone.cancel();
        });
        
        child
    }
}

impl Clone for CancellationToken {
    fn clone(&self) -> Self {
        Self {
            cancelled: self.cancelled.clone(),
            callbacks: self.callbacks.clone(),
        }
    }
}

/// Async task wrapper with full lifecycle management
pub struct AsyncTask<T> {
    pub id: TaskId,
    pub context: TaskContext,
    pub future: Pin<Box<dyn Future<Output = T> + Send + 'static>>,
    pub state: TaskState,
    pub result_sender: Option<tokio::sync::oneshot::Sender<T>>,
}

impl<T> AsyncTask<T>
where
    T: Send + 'static,
{
    pub fn new<F>(id: TaskId, future: F, name: Option<String>) -> (Self, tokio::sync::oneshot::Receiver<T>)
    where
        F: Future<Output = T> + Send + 'static,
    {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let context = TaskContext::new(id, name);
        
        let task = Self {
            id,
            context,
            future: Box::pin(future),
            state: TaskState::Pending,
            result_sender: Some(sender),
        };

        (task, receiver)
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        // Update metadata
        {
            let mut metadata = self.context.metadata.lock().unwrap();
            if metadata.started_at.is_none() {
                metadata.set_started();
            }
            metadata.increment_poll_count();
        }

        // Check for cancellation
        if self.context.is_cancelled() {
            self.state = TaskState::Cancelled;
            return Poll::Ready(());
        }

        // Update waker
        self.context.set_waker(cx.waker().clone());
        self.state = TaskState::Running;

        // Poll the future
        match self.future.as_mut().poll(cx) {
            Poll::Ready(result) => {
                self.state = TaskState::Completed;
                
                // Send result
                if let Some(sender) = self.result_sender.take() {
                    let _ = sender.send(result);
                }

                // Update metadata
                {
                    let mut metadata = self.context.metadata.lock().unwrap();
                    metadata.set_completed();
                }

                Poll::Ready(())
            }
            Poll::Pending => {
                self.state = TaskState::Pending;
                Poll::Pending
            }
        }
    }

    pub fn cancel(&mut self) {
        self.context.cancel();
        self.state = TaskState::Cancelled;
    }

    pub fn get_metadata(&self) -> TaskMetadata {
        self.context.metadata.lock().unwrap().clone()
    }
}

/// Task builder for creating tasks with specific configurations
pub struct TaskBuilder {
    name: Option<String>,
    priority: TaskPriority,
    parent_task_id: Option<TaskId>,
    goroutine_id: Option<GoroutineId>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            priority: TaskPriority::Normal,
            parent_task_id: None,
            goroutine_id: None,
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn parent_task(mut self, parent_id: TaskId) -> Self {
        self.parent_task_id = Some(parent_id);
        self
    }

    pub fn goroutine(mut self, goroutine_id: GoroutineId) -> Self {
        self.goroutine_id = Some(goroutine_id);
        self
    }

    pub fn spawn<F, T>(self, future: F) -> (AsyncTask<T>, tokio::sync::oneshot::Receiver<T>)
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let task_id = NEXT_TASK_ID.fetch_add(1, Ordering::SeqCst);
        let (mut task, receiver) = AsyncTask::new(task_id, future, self.name);
        
        // Apply builder configuration
        {
            let mut metadata = task.context.metadata.lock().unwrap();
            metadata.priority = self.priority;
            metadata.parent_task_id = self.parent_task_id;
            metadata.goroutine_id = self.goroutine_id;
        }

        (task, receiver)
    }
}

impl Default for TaskBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Global task ID counter
static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);

/// Task registry for tracking active tasks
pub struct TaskRegistry {
    tasks: Arc<Mutex<HashMap<TaskId, TaskMetadata>>>,
    stats: Arc<Mutex<TaskRegistryStats>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(TaskRegistryStats::default())),
        }
    }

    pub fn register_task(&self, task_id: TaskId, metadata: TaskMetadata) {
        self.tasks.lock().unwrap().insert(task_id, metadata);
        
        let mut stats = self.stats.lock().unwrap();
        stats.total_tasks += 1;
        stats.active_tasks += 1;
    }

    pub fn update_task(&self, task_id: TaskId, metadata: TaskMetadata) {
        self.tasks.lock().unwrap().insert(task_id, metadata);
    }

    pub fn unregister_task(&self, task_id: TaskId) -> Option<TaskMetadata> {
        let metadata = self.tasks.lock().unwrap().remove(&task_id);
        
        if metadata.is_some() {
            let mut stats = self.stats.lock().unwrap();
            stats.active_tasks = stats.active_tasks.saturating_sub(1);
            stats.completed_tasks += 1;
        }
        
        metadata
    }

    pub fn get_task(&self, task_id: TaskId) -> Option<TaskMetadata> {
        self.tasks.lock().unwrap().get(&task_id).cloned()
    }

    pub fn list_tasks(&self) -> Vec<(TaskId, TaskMetadata)> {
        self.tasks
            .lock()
            .unwrap()
            .iter()
            .map(|(&id, metadata)| (id, metadata.clone()))
            .collect()
    }

    pub fn get_stats(&self) -> TaskRegistryStats {
        self.stats.lock().unwrap().clone()
    }
}

impl Default for TaskRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Task registry statistics
#[derive(Debug, Default, Clone)]
pub struct TaskRegistryStats {
    pub total_tasks: u64,
    pub active_tasks: usize,
    pub completed_tasks: u64,
    pub cancelled_tasks: u64,
    pub failed_tasks: u64,
}

/// Global task registry
static GLOBAL_TASK_REGISTRY: once_cell::sync::OnceCell<Arc<TaskRegistry>> = once_cell::sync::OnceCell::new();

/// Initialize the global task registry
pub fn initialize_global_task_registry() -> Result<(), CursedError> {
    let registry = Arc::new(TaskRegistry::new());
    
    GLOBAL_TASK_REGISTRY
        .set(registry)
        .map_err(|_| CursedError::runtime_error("Global task registry already initialized"))?;

    Ok(())
}

/// Get the global task registry
pub fn get_global_task_registry() -> Option<Arc<TaskRegistry>> {
    GLOBAL_TASK_REGISTRY.get().cloned()
}

/// Register a task with the global registry
pub fn register_task(task_id: TaskId, metadata: TaskMetadata) -> Result<(), CursedError> {
    get_global_task_registry()
        .ok_or_else(|| CursedError::runtime_error("Global task registry not initialized"))?
        .register_task(task_id, metadata);
    Ok(())
}

/// Unregister a task from the global registry
pub fn unregister_task(task_id: TaskId) -> Result<Option<TaskMetadata>, CursedError> {
    Ok(get_global_task_registry()
        .ok_or_else(|| CursedError::runtime_error("Global task registry not initialized"))?
        .unregister_task(task_id))
}

/// Get task information from the global registry
pub fn get_task_info(task_id: TaskId) -> Result<Option<TaskMetadata>, CursedError> {
    Ok(get_global_task_registry()
        .ok_or_else(|| CursedError::runtime_error("Global task registry not initialized"))?
        .get_task(task_id))
}

/// List all active tasks
pub fn list_active_tasks() -> Result<Vec<(TaskId, TaskMetadata)>, CursedError> {
    Ok(get_global_task_registry()
        .ok_or_else(|| CursedError::runtime_error("Global task registry not initialized"))?
        .list_tasks())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_metadata_creation() {
        let metadata = TaskMetadata::new(Some("test_task".to_string()));
        assert_eq!(metadata.name, Some("test_task".to_string()));
        assert_eq!(metadata.priority, TaskPriority::Normal);
        assert_eq!(metadata.poll_count, 0);
    }

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());
        
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn test_task_builder() {
        let builder = TaskBuilder::new()
            .name("test_task")
            .priority(TaskPriority::High);
        
        let (task, _receiver) = builder.spawn(async { 42 });
        let metadata = task.get_metadata();
        
        assert_eq!(metadata.name, Some("test_task".to_string()));
        assert_eq!(metadata.priority, TaskPriority::High);
    }

    #[test]
    fn test_task_registry() {
        let registry = TaskRegistry::new();
        let metadata = TaskMetadata::new(Some("test".to_string()));
        
        registry.register_task(1, metadata.clone());
        assert_eq!(registry.get_task(1).unwrap().name, Some("test".to_string()));
        
        let removed = registry.unregister_task(1);
        assert!(removed.is_some());
        assert!(registry.get_task(1).is_none());
    }
}
