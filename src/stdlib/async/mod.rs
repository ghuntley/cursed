use crate::error::CursedError;
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
    spawn, block_on, yield_now, delay, timeout
// };

// Re-export async stdlib components
pub use io::{
    stdin_async, stdout_async, stderr_async, AsyncFile as IoAsyncFile
// };

pub use net::{
    connect, bind, listen, accept
// };

pub use fs::{
    copy_async, remove_async, metadata_async, read_dir_async
// };

pub use timer::{
    sleep, sleep_until, interval, after, Interval as AsyncInterval
// };

pub use sync::{
    mpsc, oneshot, broadcast, select, join, race
// };

pub use utils::{spawn_blocking_io};

/// Common async utilities
pub mod utils {
    use super::*;
    use std::time::Duration;

    /// Convert a synchronous function to an async one by running it in a thread pool
    pub async fn spawn_blocking<F, R>(f: F) -> crate::error::Result<()>
    where
    {
        let (promise, resolver, _rejecter) = Promise::new();

        std::thread::spawn(move || {
            let result = f();
            let _ = resolver.resolve(result);
        });

        promise.await
    /// Helper for async operations that return AsyncResult
    pub async fn spawn_blocking_result<F, R>(f: F) -> AsyncResult<R>
    where
    {
        let (promise, resolver, _rejecter) = Promise::new();

        std::thread::spawn(move || {
            let result = f();
            let _ = resolver.resolve(result);
        });

        promise.clone().await.unwrap_or_else(|_| Err(AsyncError::Runtime("Operation failed".to_string())))
    /// Race multiple futures and return the first to complete
    pub async fn race_all<T>(futures: Vec<BoxFuture<'static, T>>) -> T
    where
    {
        use crate::runtime::r#async::future::SelectFuture;
        let select = SelectFuture::new(futures);
        select.await.0
    /// Join multiple futures and wait for all to complete
    pub async fn join_all<T>(futures: Vec<BoxFuture<'static, T>>) -> Vec<T>
    where
    {
        use crate::runtime::r#async::future::JoinFuture;
        let join = JoinFuture::new(futures);
        join.await
    /// Convert a crate::error::Result<()> to a Future<crate::error::Result<()>>
    pub async fn flatten_result<T, E, F>(result: Result<F, E>) -> Result<T, E>
    where
    {
        match result {
        }
    }

    /// Retry a future up to n times with a delay between attempts
    pub async fn retry<F, Fut, T, E>(
    ) -> Result<T, E>
    where
    {
        let mut attempts = 0;
        loop {
            match f().await {
                Err(error) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(error);
                    }
                    sleep(delay).await;
                }
            }
        }
    }

    /// Create a future that never completes
    pub fn never<T>() -> impl Future<Output = T> {
        use crate::runtime::r#async::future::PendingFuture;
        PendingFuture::new()
    /// Create a future that completes immediately with a value
    pub fn ready<T>(value: T) -> impl Future<Output = T> {
        use crate::runtime::r#async::future::ReadyFuture;
        ReadyFuture::new(value)
    /// Yield execution to allow other tasks to run
    pub async fn yield_to_scheduler() {
        // TODO: implement
    }
        yield_now().await
    /// Spawn a blocking I/O operation
    pub async fn spawn_blocking_io<F, R>(f: F) -> AsyncResult<R>
    where
    {
        spawn_blocking_result(f).await
    }
}

/// Async error types
#[derive(Debug, Clone)]
pub enum AsyncError {
    /// I/O error occurred
    /// Network error occurred
    /// Timeout error
    /// Channel error (closed, full, etc.)
    /// Runtime error
    /// Generic async error
// impl std::fmt::Display for AsyncError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             AsyncError::Io(msg) => write!(f, "I/O error: {}", msg),
//             AsyncError::Network(msg) => write!(f, "Network error: {}", msg),
//             AsyncError::Timeout => write!(f, "Operation timed out"),
//             AsyncError::Channel(msg) => write!(f, "Channel error: {}", msg),
//             AsyncError::Runtime(msg) => write!(f, "Runtime error: {}", msg),
//             AsyncError::Other(msg) => write!(f, "Async error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for AsyncError {}
// 
// impl From<std::io::Error> for AsyncError {
//     fn from(err: std::io::Error) -> Self {
//         AsyncError::Io(err.to_string())
//     }
// }

impl From<FutureError> for AsyncError {
    fn from(err: FutureError) -> Self {
        match err {
        }
    }
/// Result type for async operations
pub type AsyncResult<T> = std::result::Result<T, AsyncError>;

/// Helper functions for error conversion
pub fn io_error(msg: &str) -> AsyncError {
    AsyncError::Io(msg.to_string())
pub fn network_error(msg: &str) -> AsyncError {
    AsyncError::Network(msg.to_string())
pub fn channel_error(msg: &str) -> AsyncError {
    AsyncError::Channel(msg.to_string())
pub fn runtime_error(msg: &str) -> AsyncError {
    AsyncError::Runtime(msg.to_string())
pub fn timeout_error() -> AsyncError {
    AsyncError::Timeout
/// Initialize the async stdlib module
pub fn initialize() -> crate::error::Result<()> {
    // Initialize any global async state
    crate::runtime::r#async::initialize_async_runtime()
/// Shutdown the async stdlib module
pub fn shutdown() {
        // TODO: implement
    }
    crate::runtime::r#async::shutdown_async_runtime();
}
