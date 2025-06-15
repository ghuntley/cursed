/// Async/await runtime system for CURSED
/// 
/// Provides Future/Promise types, async executors, and integration with
/// the existing goroutine scheduler for comprehensive concurrency support.

pub mod future;
pub mod promise;
pub mod executor;
pub mod runtime;
pub mod task;
pub mod scheduler;
pub mod timer;
pub mod event_loop;

// Re-export core types
pub use future::{Future, FutureState, FutureResult, FutureError, BoxFuture, LocalFuture};
pub use promise::{Promise, PromiseResolver, PromiseRejecter, PromiseState};
pub use executor::{AsyncExecutor, ExecutorConfig, ExecutorStatistics, TaskQueue};
pub use runtime::{AsyncRuntime, AsyncRuntimeConfig, RuntimeStatistics, RuntimeCoordinator};
pub use task::{Task, TaskId, TaskState, TaskHandle, TaskContext, TaskWaker};
pub use scheduler::{AsyncScheduler, SchedulerConfig as AsyncSchedulerConfig, WorkStealingQueue};
pub use timer::{Timer, Delay, Timeout, Interval, TimerWheel, TimerHandle};
pub use event_loop::{EventLoop, EventLoopConfig, EventLoopStats};

// FFI functions for LLVM integration
extern "C" {
    /// Spawn an async task from compiled code
    pub fn cursed_spawn_async_task(task_fn: extern "C" fn(), context: *mut std::ffi::c_void) -> u64;
    
    /// Await a future from compiled code
    pub fn cursed_await_future(future_id: u64) -> *mut std::ffi::c_void;
    
    /// Check if a future is ready
    pub fn cursed_future_is_ready(future_id: u64) -> bool;
    
    /// Get the result of a completed future
    pub fn cursed_future_get_result(future_id: u64) -> *mut std::ffi::c_void;
    
    /// Create a delay timer
    pub fn cursed_create_delay(duration_ms: u64) -> u64;
    
    /// Create a timeout wrapper
    pub fn cursed_create_timeout(future_id: u64, timeout_ms: u64) -> u64;
}

/// Initialize the async runtime system
pub fn initialize_async_runtime() -> Result<(), crate::error::Error> {
    runtime::initialize_global_async_runtime()
}

/// Get the global async runtime
pub fn get_async_runtime() -> Option<std::sync::Arc<AsyncRuntime>> {
    runtime::get_global_async_runtime()
}

/// Shutdown the async runtime system
pub fn shutdown_async_runtime() {
    runtime::shutdown_global_async_runtime()
}

/// Spawn a future on the async runtime
pub fn spawn<F>(future: F) -> TaskHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    if let Some(runtime) = get_async_runtime() {
        runtime.spawn(future)
    } else {
        // Create a placeholder handle for when runtime is not available
        TaskHandle::placeholder()
    }
}

/// Block on a future until completion
pub fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    if let Some(runtime) = get_async_runtime() {
        runtime.block_on(future)
    } else {
        // Fallback: create minimal executor for this future
        let executor = AsyncExecutor::new(ExecutorConfig::default());
        executor.block_on(future)
    }
}

/// Yield control back to the async executor
pub async fn yield_now() {
    // Implementation will yield to the async scheduler
    future::yield_now().await
}

/// Create a delay future
pub fn delay(duration: std::time::Duration) -> Delay {
    Timer::delay(duration)
}

/// Create a timeout wrapper for a future
pub fn timeout<F>(duration: std::time::Duration, future: F) -> Timeout<F>
where
    F: Future,
{
    Timer::timeout(duration, future)
}
