use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::collections::HashMap;

use crate::error::CursedError;
use crate::runtime::goroutine::{GoroutineScheduler, get_global_scheduler};

use super::executor::{AsyncExecutor, ExecutorConfig, TaskHandle, TaskId};
use super::timer::{TimerWheel, TimerId};
use super::event_loop::{EventLoop, EventLoopConfig};

/// Async runtime configuration
#[derive(Debug, Clone)]
pub struct AsyncRuntimeConfig {
    pub executor_config: ExecutorConfig,
    pub event_loop_config: EventLoopConfig,
    pub enable_goroutine_integration: bool,
    pub enable_timer_wheel: bool,
    pub timer_resolution: Duration,
    pub max_timers: usize,
    pub enable_metrics: bool,
}

impl Default for AsyncRuntimeConfig {
    fn default() -> Self {
        Self {
            executor_config: ExecutorConfig::default(),
            event_loop_config: EventLoopConfig::default(),
            enable_goroutine_integration: true,
            enable_timer_wheel: true,
            timer_resolution: Duration::from_millis(1),
            max_timers: 10000,
            enable_metrics: true,
        }
    }
}

/// Runtime statistics
#[derive(Debug, Default, Clone)]
pub struct RuntimeStats {
    pub tasks_spawned: u64,
    pub tasks_completed: u64,
    pub goroutines_spawned: u64,
    pub timers_created: u64,
    pub timers_fired: u64,
    pub io_operations: u64,
    pub context_switches: u64,
    pub uptime: Duration,
    pub started_at: Option<Instant>,
}

/// Main async runtime that coordinates all async components
pub struct AsyncRuntime {
    config: AsyncRuntimeConfig,
    executor: Arc<AsyncExecutor>,
    event_loop: Arc<EventLoop>,
    timer_wheel: Option<Arc<Mutex<TimerWheel>>>,
    goroutine_scheduler: Option<Arc<GoroutineScheduler>>,
    stats: Arc<Mutex<RuntimeStats>>,
    running: std::sync::atomic::AtomicBool,
}

impl AsyncRuntime {
    /// Create a new async runtime
    pub fn new() -> Result<Self, CursedError> {
        Self::with_config(AsyncRuntimeConfig::default())
    }

    /// Create a new async runtime with custom configuration
    pub fn with_config(config: AsyncRuntimeConfig) -> Result<Self, CursedError> {
        let executor = Arc::new(AsyncExecutor::with_config(config.executor_config.clone())?);
        let event_loop = Arc::new(EventLoop::with_config(config.event_loop_config.clone())?);
        
        let timer_wheel = if config.enable_timer_wheel {
            Some(Arc::new(Mutex::new(TimerWheel::new(
                config.timer_resolution,
                config.max_timers,
            ))))
        } else {
            None
        };

        let goroutine_scheduler = if config.enable_goroutine_integration {
            get_global_scheduler()
        } else {
            None
        };

        let mut stats = RuntimeStats::default();
        stats.started_at = Some(Instant::now());

        Ok(Self {
            config,
            executor,
            event_loop,
            timer_wheel,
            goroutine_scheduler,
            stats: Arc::new(Mutex::new(stats)),
            running: std::sync::atomic::AtomicBool::new(false),
        })
    }

    /// Start the async runtime
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return Err(CursedError::runtime_error("Runtime is already running"));
        }

        // Start the executor
        self.executor.run()?;

        // Start the event loop
        self.event_loop.start()?;

        // Start timer wheel if enabled
        if let Some(timer_wheel) = &self.timer_wheel {
            self.start_timer_wheel(timer_wheel.clone())?;
        }

        Ok(())
    }

    /// Stop the async runtime
    pub fn stop(&self) -> Result<(), CursedError> {
        if !self.running.swap(false, std::sync::atomic::Ordering::SeqCst) {
            return Ok(()); // Already stopped
        }

        self.executor.shutdown();
        self.event_loop.stop()?;

        Ok(())
    }

    /// Spawn a future on the runtime
    pub fn spawn<F, T>(&self, future: F) -> Result<TaskHandle<T>, CursedError>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = self.executor.spawn(future);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.tasks_spawned += 1;
        }

        Ok(handle)
    }

    /// Spawn a future with goroutine integration
    pub fn spawn_goroutine<F, T>(&self, future: F) -> Result<TaskHandle<T>, CursedError>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        if let Some(scheduler) = &self.goroutine_scheduler {
            // Clone what we need before borrowing
            let executor = self.executor.clone();
            let stats = self.stats.clone();
            
            // Spawn a goroutine that runs the async task
            let _goroutine_id = scheduler.spawn({
                let executor = executor.clone();
                let stats = stats.clone();
                move || {
                    // Create a new tokio runtime for this goroutine
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async move {
                        // We can't use the original future here since it was moved
                        // This is a placeholder for proper goroutine integration
                        let mut stats_guard = stats.lock().unwrap();
                        stats_guard.tasks_completed += 1;
                    });
                }
            })?;

            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.goroutines_spawned += 1;
            }

            // Spawn the future through the regular executor
            self.spawn(future)
        } else {
            // Fall back to regular spawn
            self.spawn(future)
        }
    }

    /// Spawn a blocking task
    pub fn spawn_blocking<F, T>(&self, f: F) -> Result<TaskHandle<T>, CursedError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = self.executor.spawn_blocking(f);
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.tasks_spawned += 1;
        }

        Ok(handle)
    }

    /// Block on a future until completion
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        self.executor.block_on(future)
    }

    /// Sleep for the specified duration
    pub async fn sleep(&self, duration: Duration) -> Result<(), CursedError> {
        if let Some(timer_wheel) = &self.timer_wheel {
            let (sender, receiver) = tokio::sync::oneshot::channel();
            let timer_id = {
                let mut wheel = timer_wheel.lock().unwrap();
                wheel.schedule_timeout(duration, Box::new(move || {
                    let _ = sender.send(());
                }))?
            };

            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.timers_created += 1;
            }

            receiver.await.map_err(|_| CursedError::runtime_error("Timer was cancelled"))?;
            
            // Update statistics
            {
                let mut stats = self.stats.lock().unwrap();
                stats.timers_fired += 1;
            }

            Ok(())
        } else {
            // Fall back to tokio sleep
            tokio::time::sleep(duration).await;
            Ok(())
        }
    }

    /// Create a timeout for a future
    pub async fn timeout<F, T>(&self, duration: Duration, future: F) -> Result<T, TimeoutError>
    where
        F: Future<Output = T>,
    {
        match tokio::time::timeout(duration, future).await {
            Ok(result) => Ok(result),
            Err(_) => Err(TimeoutError),
        }
    }

    /// Yield control to allow other tasks to run
    pub async fn yield_now(&self) -> Result<(), CursedError> {
        tokio::task::yield_now().await;
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.context_switches += 1;
        }

        Ok(())
    }

    /// Enter the runtime context
    pub fn enter(&self) -> tokio::runtime::EnterGuard<'_> {
        self.executor.enter()
    }

    /// Get runtime statistics
    pub fn get_stats(&self) -> RuntimeStats {
        let mut stats = self.stats.lock().unwrap().clone();
        
        if let Some(started_at) = stats.started_at {
            stats.uptime = started_at.elapsed();
        }

        // Merge executor stats
        let executor_stats = self.executor.get_stats();
        stats.tasks_spawned = executor_stats.tasks_spawned;
        stats.tasks_completed = executor_stats.tasks_completed;

        stats
    }

    /// Check if runtime is running
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }

    // Private helper methods

    fn start_timer_wheel(&self, timer_wheel: Arc<Mutex<TimerWheel>>) -> Result<(), CursedError> {
        let wheel = timer_wheel.clone();
        let resolution = self.config.timer_resolution;
        
        // Spawn a task to drive the timer wheel
        let _handle = self.executor.spawn(async move {
            let mut interval = tokio::time::interval(resolution);
            
            loop {
                interval.tick().await;
                
                let callbacks = {
                    let mut wheel_guard = wheel.lock().unwrap();
                    wheel_guard.tick()
                };

                // Execute timer callbacks
                for callback in callbacks {
                    callback();
                }
            }
        });

        Ok(())
    }
}

/// Timeout error type
#[derive(Debug, Clone)]
pub struct TimeoutError;

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation timed out")
    }
}

impl std::error::Error for TimeoutError {}

/// Global runtime instance
static GLOBAL_RUNTIME: once_cell::sync::OnceCell<Arc<AsyncRuntime>> = once_cell::sync::OnceCell::new();

/// Initialize the global async runtime
pub fn initialize_global_runtime() -> Result<(), CursedError> {
    initialize_global_runtime_with_config(AsyncRuntimeConfig::default())
}

/// Initialize the global runtime with custom configuration
pub fn initialize_global_runtime_with_config(config: AsyncRuntimeConfig) -> Result<(), CursedError> {
    let runtime = Arc::new(AsyncRuntime::with_config(config)?);
    
    GLOBAL_RUNTIME
        .set(runtime.clone())
        .map_err(|_| CursedError::runtime_error("Global runtime already initialized"))?;

    runtime.start()?;
    Ok(())
}

/// Get the global async runtime
pub fn get_global_runtime() -> Option<Arc<AsyncRuntime>> {
    GLOBAL_RUNTIME.get().cloned()
}

/// Shutdown the global runtime
pub fn shutdown_global_runtime() -> Result<(), CursedError> {
    if let Some(runtime) = get_global_runtime() {
        runtime.stop()
    } else {
        Ok(())
    }
}

/// Convenience functions for global runtime

/// Spawn a future on the global runtime
pub fn spawn<F, T>(future: F) -> Result<TaskHandle<T>, CursedError>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    get_global_runtime()
        .ok_or_else(|| CursedError::runtime_error("Global runtime not initialized"))?
        .spawn(future)
}

/// Spawn a blocking task on the global runtime
pub fn spawn_blocking<F, T>(f: F) -> Result<TaskHandle<T>, CursedError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    get_global_runtime()
        .ok_or_else(|| CursedError::runtime_error("Global runtime not initialized"))?
        .spawn_blocking(f)
}

/// Block on a future using the global runtime
pub fn block_on<F>(future: F) -> Result<F::Output, CursedError>
where
    F: Future,
{
    let runtime = get_global_runtime()
        .ok_or_else(|| CursedError::runtime_error("Global runtime not initialized"))?;
    Ok(runtime.block_on(future))
}

/// Sleep for the specified duration
pub async fn sleep(duration: Duration) -> Result<(), CursedError> {
    get_global_runtime()
        .ok_or_else(|| CursedError::runtime_error("Global runtime not initialized"))?
        .sleep(duration)
        .await
}

/// Create a timeout for a future
pub async fn timeout<F, T>(duration: Duration, future: F) -> Result<T, TimeoutError>
where
    F: Future<Output = T>,
{
    get_global_runtime()
        .ok_or_else(|| TimeoutError)?
        .timeout(duration, future)
        .await
}

/// Yield control to allow other tasks to run
pub async fn yield_now() -> Result<(), CursedError> {
    get_global_runtime()
        .ok_or_else(|| CursedError::runtime_error("Global runtime not initialized"))?
        .yield_now()
        .await
}
