// Async Runtime Module for CURSED
//
// This module provides asynchronous runtime support including:
// - Task spawning and scheduling with work-stealing
// - Future/promise handling with JavaScript-like API
// - High-resolution timer wheel for timeouts and delays
// - Event loop for I/O and callback handling
// - Integration with LLVM generated code
// - FFI bindings for compiled async operations
// - Goroutine interoperability

// Core async components
pub mod executor;
pub mod runtime;
pub mod scheduler;
pub mod task;
pub mod event_loop;
pub mod timer;
pub mod future;
pub mod promise;

// Re-export main types
pub use executor::{
    AsyncExecutor, ExecutorConfig, ExecutorStats, TaskHandle, TaskId, TaskPriority, TaskState
};
pub use runtime::{
    AsyncRuntime, AsyncRuntimeConfig, RuntimeStats, TimeoutError,
    initialize_global_runtime, get_global_runtime, shutdown_global_runtime
};
pub use scheduler::{
    AsyncScheduler, SchedulerConfig, SchedulerStats, SchedulingPolicy,
    initialize_global_async_scheduler, get_global_async_scheduler, shutdown_global_async_scheduler
};
pub use task::{
    TaskBuilder, TaskMetadata, TaskContext, CancellationToken, TaskRegistry, TaskRegistryStats,
    initialize_global_task_registry, get_global_task_registry
};
pub use event_loop::{
    EventLoop, EventLoopConfig, EventLoopStats, Event, EventId, EventPriority,
    initialize_global_event_loop, get_global_event_loop, shutdown_global_event_loop
};
pub use timer::{
    TimerWheel, TimerId, TimerEntry, TimerWheelStats, Interval, 
    timeout as timer_timeout, sleep as timer_sleep, sleep_until as timer_sleep_until, 
    delay, delay_until, interval, after
};
pub use future::{
    ReadyFuture, PendingFuture, DelayFuture, TimeoutFuture,
    JoinFuture, SelectFuture, AndThenFuture, LazyFuture
};
pub use promise::{
    Promise, PromiseState, PromiseResult, PromiseResolver, PromiseRejector
};

use std::future::Future;
use std::time::Duration;
use crate::error::CursedError;

// Global runtime functions

/// Initialize the complete async runtime system
pub fn initialize_async_runtime() -> crate::error::Result<()> {
    // Initialize task registry first
    task::initialize_global_task_registry()?;
    
    // Initialize event loop
    event_loop::initialize_global_event_loop()?;
    
    // Initialize scheduler
    scheduler::initialize_global_async_scheduler()?;
    
    // Initialize main runtime
    runtime::initialize_global_runtime()?;
    
    Ok(())
}

/// Initialize async runtime with custom configuration
pub fn initialize_async_runtime_with_config(config: AsyncRuntimeConfig) -> crate::error::Result<()> {
    // Initialize task registry first
    task::initialize_global_task_registry()?;
    
    // Initialize event loop with config
    event_loop::initialize_global_event_loop_with_config(config.event_loop_config.clone())?;
    
    // Initialize scheduler with config
    scheduler::initialize_global_async_scheduler_with_config(SchedulerConfig {
        num_workers: config.executor_config.max_threads,
        scheduling_policy: SchedulingPolicy::WorkStealing,
        work_stealing_enabled: config.executor_config.enable_work_stealing,
        preemption_enabled: false,
        time_slice_duration: Duration::from_millis(10),
        max_tasks_per_worker: 1000,
        enable_load_balancing: true,
        load_balance_interval: Duration::from_millis(100),
        enable_metrics: config.enable_metrics,
    })?;
    
    // Initialize main runtime with config
    runtime::initialize_global_runtime_with_config(config)?;
    
    Ok(())
}

/// Get the global async runtime
pub fn get_async_runtime() -> Option<std::sync::Arc<AsyncRuntime>> {
    runtime::get_global_runtime()
}

/// Shutdown the async runtime system
pub fn shutdown_async_runtime() {
    let _ = runtime::shutdown_global_runtime();
    let _ = scheduler::shutdown_global_async_scheduler();
    let _ = event_loop::shutdown_global_event_loop();
}

/// Spawn a future on the global async runtime
pub fn spawn<F, T>(future: F) -> Result<TaskHandle<T>, CursedError>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    runtime::spawn(future)
}

/// Spawn a blocking task on the global async runtime
pub fn spawn_blocking<F, T>(f: F) -> Result<TaskHandle<T>, CursedError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    runtime::spawn_blocking(f)
}

/// Block on a future using the global runtime
pub fn block_on<F>(future: F) -> Result<F::Output, CursedError>
where
    F: Future,
{
    runtime::block_on(future)
}

/// Sleep for the specified duration
pub async fn sleep(duration: Duration) -> Result<(), CursedError> {
    runtime::sleep(duration).await
}

/// Yield control to allow other tasks to run
pub async fn yield_now() -> Result<(), CursedError> {
    runtime::yield_now().await
}

/// Create a timeout for a future
pub async fn timeout<F, T>(duration: Duration, future: F) -> Result<T, TimeoutError>
where
    F: Future<Output = T>,
{
    runtime::timeout(duration, future).await
}

// Legacy compatibility types

/// Legacy async task wrapper for backward compatibility
#[deprecated(note = "Use TaskHandle from executor module instead")]
pub struct AsyncTask {
    task_id: u64,
    status: TaskStatus,
}

#[allow(deprecated)]
impl AsyncTask {
    pub fn new(task_id: u64) -> Self {
        Self {
            task_id,
            status: TaskStatus::Pending,
        }
    }

    pub fn task_id(&self) -> u64 {
        self.task_id
    }

    pub fn status(&self) -> TaskStatus {
        self.status
    }
}

/// Legacy status of an async task
#[deprecated(note = "Use TaskState from executor module instead")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
    Failed,
}

// FFI functions for LLVM integration

/// Spawn an async task from compiled code
#[no_mangle]
pub extern "C" fn cursed_spawn_async_task(
    task_fn: extern "C" fn(*mut std::ffi::c_void),
    context: *mut std::ffi::c_void,
) -> u64 {
    let context_addr = context as usize;
    
    let future = async move {
        let context = context_addr as *mut std::ffi::c_void;
        task_fn(context);
    };
    
    match spawn(future) {
        Ok(handle) => handle.task_id(),
        Err(_) => 0, // Error indicator
    }
}

/// Await a future from compiled code (simplified implementation)
#[no_mangle]
pub extern "C" fn cursed_await_future(future_id: u64) -> *mut std::ffi::c_void {
    // In a real implementation, this would look up the future by ID
    // and block until completion
    std::ptr::null_mut()
}

/// Check if a future is ready
#[no_mangle]
pub extern "C" fn cursed_future_is_ready(future_id: u64) -> bool {
    // In a real implementation, this would check the future status
    false
}

/// Get the result of a completed future
#[no_mangle]
pub extern "C" fn cursed_future_get_result(future_id: u64) -> *mut std::ffi::c_void {
    // In a real implementation, this would return the future result
    std::ptr::null_mut()
}

/// Create a delay timer from compiled code
#[no_mangle]
pub extern "C" fn cursed_create_delay(duration_ms: u64) -> u64 {
    let duration = Duration::from_millis(duration_ms);
    let future = delay(duration);
    
    match spawn(async move {
        future.await;
    }) {
        Ok(handle) => handle.task_id(),
        Err(_) => 0,
    }
}

/// Create a timeout wrapper from compiled code
#[no_mangle]
pub extern "C" fn cursed_create_timeout(future_id: u64, timeout_ms: u64) -> u64 {
    // In a real implementation, this would wrap an existing future with a timeout
    let timeout_duration = Duration::from_millis(timeout_ms);
    let timeout_future = delay(timeout_duration);
    
    match spawn(async move {
        timeout_future.await;
    }) {
        Ok(handle) => handle.task_id(),
        Err(_) => 0,
    }
}

/// Initialize async runtime from compiled code
#[no_mangle]
pub extern "C" fn cursed_init_async_runtime() -> bool {
    initialize_async_runtime().is_ok()
}

/// Shutdown async runtime from compiled code
#[no_mangle]
pub extern "C" fn cursed_shutdown_async_runtime() {
    shutdown_async_runtime();
}

/// Spawn a goroutine-integrated async task from compiled code
#[no_mangle]
pub extern "C" fn cursed_spawn_goroutine_async_task(
    task_fn: extern "C" fn(*mut std::ffi::c_void),
    context: *mut std::ffi::c_void,
) -> u64 {
    let context_addr = context as usize;
    
    // Try to use goroutine integration if available
    if let Some(runtime) = get_async_runtime() {
        let future = async move {
            let context = context_addr as *mut std::ffi::c_void;
            task_fn(context);
        };
        
        match runtime.spawn_goroutine(future) {
            Ok(handle) => handle.task_id(),
            Err(_) => 0,
        }
    } else {
        cursed_spawn_async_task(task_fn, context)
    }
}

/// Get async runtime statistics from compiled code
#[no_mangle]
pub extern "C" fn cursed_get_async_runtime_stats() -> *mut std::ffi::c_void {
    if let Some(runtime) = get_async_runtime() {
        let stats = runtime.get_stats();
        Box::into_raw(Box::new(stats)) as *mut std::ffi::c_void
    } else {
        std::ptr::null_mut()
    }
}

/// Free async runtime statistics memory
#[no_mangle]
pub extern "C" fn cursed_free_async_runtime_stats(stats_ptr: *mut std::ffi::c_void) {
    if !stats_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(stats_ptr as *mut RuntimeStats);
        }
    }
}
