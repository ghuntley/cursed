use crate::error::CursedError;
use std::future::Future;
use std::time::Duration;

/// Async standard library for CURSED
/// 
/// Provides high-level async operations for I/O, networking, and other
/// asynchronous operations that integrate with the async runtime.

pub mod io;
pub mod net;
pub mod fs;
pub mod timer;
pub mod sync;

// Re-export core types from runtime
pub use crate::runtime::r#async::{
    spawn, spawn_blocking, block_on, yield_now, sleep, timeout,
    TaskHandle, Promise, ready, pending, join_all, select_all,
    BoxFuture, FutureError
};

// Re-export async stdlib components
pub use io::{
    stdin_async, stdout_async, stderr_async, AsyncFile
};

pub use net::{
    connect, bind, listen, accept
};

pub use fs::{
    copy_async, remove_async, metadata_async, read_dir_async
};

pub use timer::{
    sleep as async_sleep, sleep_until as async_sleep_until, interval as async_interval, after, Interval
};

pub use sync::{
    mpsc, oneshot, broadcast, select, join, race
};

pub use utils::{spawn_blocking_io, retry, race_all, join_all_async};

/// Common async utilities
pub mod utils {
    use super::*;
    use crate::runtime::r#async::{spawn_blocking as runtime_spawn_blocking, Promise, BoxFuture};

    /// Convert a synchronous function to an async one by running it in a thread pool
    pub async fn spawn_blocking<F, R>(f: F) -> Result<R, AsyncError>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        match runtime_spawn_blocking(f) {
            Ok(handle) => handle.join().await.map_err(|e| AsyncError::Runtime(e.to_string())),
            Err(e) => Err(AsyncError::Runtime(e.to_string())),
        }
    }

    /// Helper for async operations that return AsyncResult
    pub async fn spawn_blocking_result<F, R>(f: F) -> AsyncResult<R>
    where
        F: FnOnce() -> Result<R, AsyncError> + Send + 'static,
        R: Send + 'static,
    {
        match spawn_blocking(f).await {
            Ok(result) => result,
            Err(e) => Err(e),
        }
    }

    /// Race multiple futures and return the first to complete
    pub async fn race_all<T>(futures: Vec<BoxFuture<'static, T>>) -> T
    where
        T: Send + 'static,
    {
        let (result, _index) = crate::runtime::r#async::select_all(futures).await;
        result
    }

    /// Join multiple futures and wait for all to complete
    pub async fn join_all_async<T>(futures: Vec<BoxFuture<'static, T>>) -> Vec<T>
    where
        T: Send + 'static,
    {
        crate::runtime::r#async::join_all(futures).await
    }

    /// Convert a Result to a Future
    pub async fn flatten_result<T, E>(result: Result<T, E>) -> Result<T, E> {
        result
    }

    /// Retry a future up to n times with a delay between attempts
    pub async fn retry<F, Fut, T, E>(
        mut f: F,
        max_attempts: usize,
        delay: Duration,
    ) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        let mut attempts = 0;
        loop {
            match f().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(error);
                    }
                    crate::runtime::r#async::sleep(delay).await.unwrap_or(());
                }
            }
        }
    }

    /// Create a future that never completes
    pub fn never<T>() -> impl Future<Output = T> {
        crate::runtime::r#async::pending()
    }

    /// Create a future that completes immediately with a value
    pub fn ready<T>(value: T) -> impl Future<Output = T> {
        crate::runtime::r#async::ready(value)
    }

    /// Yield execution to allow other tasks to run
    pub async fn yield_to_scheduler() {
        let _ = yield_now().await;
    }

    /// Spawn a blocking I/O operation
    pub async fn spawn_blocking_io<F, R>(f: F) -> AsyncResult<R>
    where
        F: FnOnce() -> Result<R, AsyncError> + Send + 'static,
        R: Send + 'static,
    {
        spawn_blocking_result(f).await
    }
}

/// Async error types
#[derive(Debug, Clone)]
pub enum AsyncError {
    /// I/O error occurred
    Io(String),
    /// Network error occurred
    Network(String),
    /// Timeout error
    Timeout,
    /// Channel error (closed, full, etc.)
    Channel(String),
    /// Runtime error
    Runtime(String),
    /// Generic async error
    Other(String),
}

impl std::fmt::Display for AsyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsyncError::Io(msg) => write!(f, "I/O error: {}", msg),
            AsyncError::Network(msg) => write!(f, "Network error: {}", msg),
            AsyncError::Timeout => write!(f, "Operation timed out"),
            AsyncError::Channel(msg) => write!(f, "Channel error: {}", msg),
            AsyncError::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            AsyncError::Other(msg) => write!(f, "Async error: {}", msg),
        }
    }
}

impl std::error::Error for AsyncError {}

impl From<std::io::Error> for AsyncError {
    fn from(err: std::io::Error) -> Self {
        AsyncError::Io(err.to_string())
    }
}

impl From<FutureError> for AsyncError {
    fn from(err: FutureError) -> Self {
        match err {
            FutureError::Cancelled => AsyncError::Runtime("Future was cancelled".to_string()),
            FutureError::Timeout => AsyncError::Timeout,
            FutureError::Runtime(msg) => AsyncError::Runtime(msg),
        }
    }
}

impl From<CursedError> for AsyncError {
    fn from(err: CursedError) -> Self {
        AsyncError::Runtime(err.to_string())
    }
}

/// Result type for async operations
pub type AsyncResult<T> = std::result::Result<T, AsyncError>;

/// Helper functions for error conversion
pub fn io_error(msg: &str) -> AsyncError {
    AsyncError::Io(msg.to_string())
}

pub fn network_error(msg: &str) -> AsyncError {
    AsyncError::Network(msg.to_string())
}

pub fn channel_error(msg: &str) -> AsyncError {
    AsyncError::Channel(msg.to_string())
}

pub fn runtime_error(msg: &str) -> AsyncError {
    AsyncError::Runtime(msg.to_string())
}

pub fn timeout_error() -> AsyncError {
    AsyncError::Timeout
}

/// Initialize the async stdlib module
pub fn initialize() -> crate::error::Result<()> {
    // Initialize the async runtime
    crate::runtime::r#async::initialize_async_runtime()
}

/// Shutdown the async stdlib module
pub fn shutdown() {
    crate::runtime::r#async::shutdown_async_runtime();
}
}
