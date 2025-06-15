/// Async runtime system for CURSED that integrates with goroutines
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use std::thread;

use crate::runtime::r#async::{
    AsyncExecutor, ExecutorConfig, ExecutorStatistics,
    Future, TaskHandle, TaskPriority,
    Promise, PromiseResolver, PromiseRejecter, FutureError, FutureResult
};
use crate::runtime::goroutine::{GoroutineScheduler, SchedulerConfig};

/// Global async runtime instance
static mut GLOBAL_ASYNC_RUNTIME: Option<Arc<AsyncRuntime>> = None;
static RUNTIME_INIT: Once = Once::new();

/// Configuration for the async runtime
#[derive(Debug, Clone)]
pub struct AsyncRuntimeConfig {
    /// Executor configuration
    pub executor_config: ExecutorConfig,
    /// Enable integration with goroutine scheduler
    pub integrate_with_goroutines: bool,
    /// Number of async worker threads
    pub async_worker_threads: usize,
    /// Runtime statistics collection interval
    pub stats_collection_interval: Duration,
}

impl Default for AsyncRuntimeConfig {
    fn default() -> Self {
        Self {
            executor_config: ExecutorConfig::default(),
            integrate_with_goroutines: true,
            async_worker_threads: num_cpus::get().max(2),
            stats_collection_interval: Duration::from_secs(5),
        }
    }
}

/// Runtime statistics for monitoring
#[derive(Debug, Clone)]
pub struct RuntimeStatistics {
    pub executor_stats: ExecutorStatistics,
    pub active_futures: usize,
    pub completed_futures: usize,
    pub failed_futures: usize,
    pub runtime_uptime: Duration,
    pub memory_usage: usize,
    pub goroutine_integration_active: bool,
}

impl Default for RuntimeStatistics {
    fn default() -> Self {
        Self {
            executor_stats: ExecutorStatistics::default(),
            active_futures: 0,
            completed_futures: 0,
            failed_futures: 0,
            runtime_uptime: Duration::ZERO,
            memory_usage: 0,
            goroutine_integration_active: false,
        }
    }
}

/// Runtime coordinator for managing async/goroutine integration
pub struct RuntimeCoordinator {
    goroutine_scheduler: Option<Arc<GoroutineScheduler>>,
    integration_active: bool,
}

impl RuntimeCoordinator {
    pub fn new() -> Self {
        Self {
            goroutine_scheduler: None,
            integration_active: false,
        }
    }

    /// Set the goroutine scheduler for integration
    pub fn set_goroutine_scheduler(&mut self, scheduler: Arc<GoroutineScheduler>) {
        self.goroutine_scheduler = Some(scheduler);
        self.integration_active = true;
    }

    /// Check if goroutine integration is active
    pub fn is_integration_active(&self) -> bool {
        self.integration_active
    }

    /// Coordinate with goroutine scheduler for GC safe points
    pub fn coordinate_gc_safe_point(&self) -> Result<(), crate::error::Error> {
        if let Some(scheduler) = &self.goroutine_scheduler {
            scheduler.coordinate_gc(Duration::from_millis(100))
        } else {
            Ok(())
        }
    }
}

/// Main async runtime for CURSED
pub struct AsyncRuntime {
    config: AsyncRuntimeConfig,
    executor: Mutex<Option<AsyncExecutor>>,
    coordinator: Mutex<RuntimeCoordinator>,
    statistics: Mutex<RuntimeStatistics>,
    start_time: std::time::Instant,
    shutdown: Mutex<bool>,
}

impl AsyncRuntime {
    /// Create a new async runtime with the given configuration
    pub fn new(config: AsyncRuntimeConfig) -> Self {
        Self {
            config,
            executor: Mutex::new(None),
            coordinator: Mutex::new(RuntimeCoordinator::new()),
            statistics: Mutex::new(RuntimeStatistics::default()),
            start_time: std::time::Instant::now(),
            shutdown: Mutex::new(false),
        }
    }

    /// Initialize the runtime with an executor
    pub fn initialize(&self) -> Result<(), crate::error::Error> {
        let mut executor_opt = self.executor.lock().unwrap();
        
        if executor_opt.is_none() {
            let mut executor = AsyncExecutor::new(self.config.executor_config.clone());
            executor.start();
            *executor_opt = Some(executor);
        }

        // Initialize goroutine integration if enabled
        if self.config.integrate_with_goroutines {
            self.initialize_goroutine_integration()?;
        }

        Ok(())
    }

    /// Initialize goroutine integration
    fn initialize_goroutine_integration(&self) -> Result<(), crate::error::Error> {
        // Try to get the global goroutine scheduler
        if let Some(scheduler) = crate::runtime::goroutine::get_global_scheduler() {
            let mut coordinator = self.coordinator.lock().unwrap();
            coordinator.set_goroutine_scheduler(scheduler);
        }
        Ok(())
    }

    /// Spawn a future on the runtime
    pub fn spawn<F>(&self, future: F) -> TaskHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let executor = self.executor.lock().unwrap();
        if let Some(ref executor) = *executor {
            executor.spawn(future)
        } else {
            // Runtime not initialized, return placeholder
            TaskHandle::placeholder()
        }
    }

    /// Spawn a future with specific priority
    pub fn spawn_with_priority<F>(&self, future: F, priority: TaskPriority) -> TaskHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let executor = self.executor.lock().unwrap();
        if let Some(ref executor) = *executor {
            executor.spawn_with_priority(future, priority)
        } else {
            TaskHandle::placeholder()
        }
    }

    /// Block on a future until completion
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        let executor = self.executor.lock().unwrap();
        if let Some(ref executor) = *executor {
            executor.block_on(future)
        } else {
            // Create temporary executor for blocking
            let temp_executor = AsyncExecutor::new(ExecutorConfig::default());
            temp_executor.block_on(future)
        }
    }

    /// Spawn a future with a resolver/rejecter pair (for Promise integration)
    pub fn spawn_with_resolver<F, T>(
        &self,
        future: F,
        resolver: PromiseResolver<T>,
        rejecter: PromiseRejecter<T>,
    ) where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = self.spawn(async move {
            future.await
        });

        // Spawn a task to handle the result
        self.spawn(async move {
            match handle.await {
                Ok(result) => {
                    let _ = resolver.resolve(result);
                }
                Err(error) => {
                    let _ = rejecter.reject(error);
                }
            }
        });
    }

    /// Spawn a promise chain operation
    pub fn spawn_chain<F, T, U>(
        &self,
        promise: Promise<T>,
        chain_fn: F,
        resolver: PromiseResolver<U>,
        rejecter: PromiseRejecter<U>,
    ) where
        F: FnOnce(T) -> U + Send + 'static,
        T: Clone + Send + 'static,
        U: Send + 'static,
    {
        self.spawn(async move {
            match promise.await {
                Ok(value) => {
                    let result = chain_fn(value);
                    let _ = resolver.resolve(result);
                }
                Err(error) => {
                    let _ = rejecter.reject(error);
                }
            }
        });
    }

    /// Spawn a promise catch operation
    pub fn spawn_catch<F, T>(
        &self,
        promise: Promise<T>,
        catch_fn: F,
        resolver: PromiseResolver<T>,
        rejecter: PromiseRejecter<T>,
    ) where
        F: FnOnce(FutureError) -> T + Send + 'static,
        T: Clone + Send + 'static,
    {
        self.spawn(async move {
            match promise.await {
                Ok(value) => {
                    let _ = resolver.resolve(value);
                }
                Err(error) => {
                    let result = catch_fn(error);
                    let _ = resolver.resolve(result);
                }
            }
        });
    }

    /// Spawn a promise map operation
    pub fn spawn_map<F, T, U>(
        &self,
        promise: Promise<T>,
        map_fn: F,
        resolver: PromiseResolver<U>,
        rejecter: PromiseRejecter<U>,
    ) where
        F: FnOnce(T) -> FutureResult<U> + Send + 'static,
        T: Clone + Send + 'static,
        U: Send + 'static,
    {
        self.spawn(async move {
            match promise.await {
                Ok(value) => {
                    match map_fn(value) {
                        Ok(result) => {
                            let _ = resolver.resolve(result);
                        }
                        Err(error) => {
                            let _ = rejecter.reject(error);
                        }
                    }
                }
                Err(error) => {
                    let _ = rejecter.reject(error);
                }
            }
        });
    }

    /// Get runtime statistics
    pub fn statistics(&self) -> RuntimeStatistics {
        let mut stats = self.statistics.lock().unwrap();
        stats.runtime_uptime = self.start_time.elapsed();
        
        // Update executor statistics
        let executor = self.executor.lock().unwrap();
        if let Some(ref executor) = *executor {
            stats.executor_stats = executor.statistics();
        }

        // Update coordinator status
        let coordinator = self.coordinator.lock().unwrap();
        stats.goroutine_integration_active = coordinator.is_integration_active();

        stats.clone()
    }

    /// Shutdown the runtime
    pub fn shutdown(&self) {
        let mut shutdown = self.shutdown.lock().unwrap();
        if !*shutdown {
            *shutdown = true;

            // Shutdown executor
            let mut executor = self.executor.lock().unwrap();
            if let Some(ref mut executor) = *executor {
                executor.shutdown();
            }
        }
    }

    /// Check if the runtime is shut down
    pub fn is_shutdown(&self) -> bool {
        *self.shutdown.lock().unwrap()
    }

    /// Coordinate with goroutine scheduler for GC
    pub fn coordinate_gc(&self) -> Result<(), crate::error::Error> {
        let coordinator = self.coordinator.lock().unwrap();
        coordinator.coordinate_gc_safe_point()
    }
}

impl Drop for AsyncRuntime {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Initialize the global async runtime
pub fn initialize_global_async_runtime() -> Result<(), crate::error::Error> {
    RUNTIME_INIT.call_once(|| {
        let config = AsyncRuntimeConfig::default();
        let runtime = Arc::new(AsyncRuntime::new(config));
        
        if let Err(e) = runtime.initialize() {
            eprintln!("Failed to initialize async runtime: {}", e);
            return;
        }

        unsafe {
            GLOBAL_ASYNC_RUNTIME = Some(runtime);
        }
    });
    Ok(())
}

/// Get the global async runtime
pub fn get_global_async_runtime() -> Option<Arc<AsyncRuntime>> {
    unsafe { GLOBAL_ASYNC_RUNTIME.clone() }
}

/// Shutdown the global async runtime
pub fn shutdown_global_async_runtime() {
    unsafe {
        if let Some(runtime) = GLOBAL_ASYNC_RUNTIME.take() {
            runtime.shutdown();
        }
    }
}

/// Spawn a future on the global runtime
pub fn spawn_global<F>(future: F) -> TaskHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    if let Some(runtime) = get_global_async_runtime() {
        runtime.spawn(future)
    } else {
        TaskHandle::placeholder()
    }
}

/// Block on a future using the global runtime
pub fn block_on_global<F>(future: F) -> F::Output
where
    F: Future,
{
    if let Some(runtime) = get_global_async_runtime() {
        runtime.block_on(future)
    } else {
        // Fallback to local executor
        let executor = AsyncExecutor::new(ExecutorConfig::default());
        executor.block_on(future)
    }
}

/// Get global runtime statistics
pub fn get_global_runtime_statistics() -> Option<RuntimeStatistics> {
    get_global_async_runtime().map(|runtime| runtime.statistics())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::r#async::future::ready;

    #[test]
    fn test_runtime_creation() {
        let config = AsyncRuntimeConfig::default();
        let runtime = AsyncRuntime::new(config);
        
        let stats = runtime.statistics();
        assert_eq!(stats.active_futures, 0);
    }

    #[test]
    fn test_runtime_initialization() {
        let config = AsyncRuntimeConfig::default();
        let runtime = AsyncRuntime::new(config);
        
        assert!(runtime.initialize().is_ok());
    }

    #[test]
    fn test_block_on() {
        let config = AsyncRuntimeConfig::default();
        let runtime = AsyncRuntime::new(config);
        let _ = runtime.initialize();
        
        let result = runtime.block_on(ready(42));
        assert_eq!(result, 42);
    }

    #[test]
    fn test_global_runtime() {
        assert!(initialize_global_async_runtime().is_ok());
        
        let runtime = get_global_async_runtime();
        assert!(runtime.is_some());
        
        shutdown_global_async_runtime();
    }
}
