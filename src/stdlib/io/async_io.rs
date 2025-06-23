/// Async I/O operations for CURSED standard library
use std::io::{self, Error as IoError, ErrorKind};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::runtime::r#async::{Future as RuntimeFuture, FutureResult, FutureError};
use std::future::Future;
use crate::stdlib::io::IoResult;

/// Async file operations
pub struct AsyncFile {
    inner: tokio::fs::File,
}

impl AsyncFile {
    /// Open a file asynchronously
    pub async fn open<P: AsRef<Path>>(path: P) -> IoResult<Self> {
        let file = tokio::fs::File::open(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(AsyncFile { inner: file })
    }

    /// Create a new file asynchronously
    pub async fn create<P: AsRef<Path>>(path: P) -> IoResult<Self> {
        let file = tokio::fs::File::create(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(AsyncFile { inner: file })
    }

    /// Read entire file content asynchronously
    pub async fn read_to_string(&mut self) -> IoResult<String> {
        let mut content = String::new();
        tokio::io::AsyncReadExt::read_to_string(&mut self.inner, &mut content).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(content)
    }

    /// Read file content into bytes asynchronously
    pub async fn read_to_end(&mut self) -> IoResult<Vec<u8>> {
        let mut content = Vec::new();
        tokio::io::AsyncReadExt::read_to_end(&mut self.inner, &mut content).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(content)
    }

    /// Write string content to file asynchronously
    pub async fn write_all_string(&mut self, content: &str) -> IoResult<()> {
        tokio::io::AsyncWriteExt::write_all(&mut self.inner, content.as_bytes()).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Write bytes to file asynchronously
    pub async fn write_all_bytes(&mut self, content: &[u8]) -> IoResult<()> {
        tokio::io::AsyncWriteExt::write_all(&mut self.inner, content).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Flush the file asynchronously
    pub async fn flush(&mut self) -> IoResult<()> {
        tokio::io::AsyncWriteExt::flush(&mut self.inner).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Sync all data to disk asynchronously
    pub async fn sync_all(&self) -> IoResult<()> {
        self.inner.sync_all().await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }
}

/// Async network I/O operations
pub struct AsyncTcpStream {
    inner: tokio::net::TcpStream,
}

impl AsyncTcpStream {
    /// Connect to a TCP server asynchronously
    pub async fn connect(addr: &str) -> IoResult<Self> {
        let stream = tokio::net::TcpStream::connect(addr).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(AsyncTcpStream { inner: stream })
    }

    /// Read data from the stream asynchronously
    pub async fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        tokio::io::AsyncReadExt::read(&mut self.inner, buf).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))
    }

    /// Write data to the stream asynchronously
    pub async fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        tokio::io::AsyncWriteExt::write(&mut self.inner, buf).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))
    }

    /// Write all data to the stream asynchronously
    pub async fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        tokio::io::AsyncWriteExt::write_all(&mut self.inner, buf).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Flush the stream asynchronously
    pub async fn flush(&mut self) -> IoResult<()> {
        tokio::io::AsyncWriteExt::flush(&mut self.inner).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Shutdown the stream asynchronously
    pub async fn shutdown(&mut self) -> IoResult<()> {
        tokio::io::AsyncWriteExt::shutdown(&mut self.inner).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }
}

/// Async TCP listener for server applications
pub struct AsyncTcpListener {
    inner: tokio::net::TcpListener,
}

impl AsyncTcpListener {
    /// Bind to an address asynchronously
    pub async fn bind(addr: &str) -> IoResult<Self> {
        let listener = tokio::net::TcpListener::bind(addr).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(AsyncTcpListener { inner: listener })
    }

    /// Accept a new connection asynchronously
    pub async fn accept(&self) -> IoResult<(AsyncTcpStream, std::net::SocketAddr)> {
        let (stream, addr) = self.inner.accept().await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok((AsyncTcpStream { inner: stream }, addr))
    }

    /// Get the local address
    pub fn local_addr(&self) -> IoResult<std::net::SocketAddr> {
        self.inner.local_addr()
            .map_err(|e| crate::stdlib::io::IoError::from(e))
    }
}

/// Async sleep function
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await
}

/// Async timeout wrapper
pub async fn timeout<F>(duration: Duration, future: F) -> Result<(), Error>
where
    F: Future + Send,
{
    match tokio::time::timeout(duration, future).await {
        Ok(result) => Ok(result),
        Err(_) => Err(FutureError::Timeout),
    }
}

/// Async spawn function that runs a future on the async runtime
pub fn spawn<F>(future: F) -> crate::runtime::r#async::TaskHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    crate::runtime::r#async::spawn(future)
}

/// Async spawn local function for non-Send futures
pub fn spawn_local<F>(future: F) -> LocalTaskHandle<F::Output>
where
    F: Future + 'static,
{
    LocalTaskHandle::new(future)
}

/// Handle for local (non-Send) tasks
pub struct LocalTaskHandle<T> {
    inner: Pin<Box<dyn Future<Output = T>>>,
}

impl<T> LocalTaskHandle<T> {
    fn new<F>(future: F) -> Self
    where
        F: Future<Output = T> + 'static,
    {
        Self {
            inner: Box::pin(future),
        }
    }
}

impl<T> Future for LocalTaskHandle<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}

/// Async utilities for file system operations
pub mod fs {
    use super::*;

    /// Read entire file content asynchronously
    pub async fn read_to_string<P: AsRef<Path>>(path: P) -> IoResult<String> {
        tokio::fs::read_to_string(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))
    }

    /// Read file content as bytes asynchronously
    pub async fn read<P: AsRef<Path>>(path: P) -> IoResult<Vec<u8>> {
        tokio::fs::read(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))
    }

    /// Write string content to file asynchronously
    pub async fn write<P: AsRef<Path>>(path: P, content: &str) -> IoResult<()> {
        tokio::fs::write(path, content.as_bytes()).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Write bytes to file asynchronously
    pub async fn write_bytes<P: AsRef<Path>>(path: P, content: &[u8]) -> IoResult<()> {
        tokio::fs::write(path, content).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Create a directory asynchronously
    pub async fn create_dir<P: AsRef<Path>>(path: P) -> IoResult<()> {
        tokio::fs::create_dir(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Create a directory and all parent directories asynchronously
    pub async fn create_dir_all<P: AsRef<Path>>(path: P) -> IoResult<()> {
        tokio::fs::create_dir_all(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Remove a file asynchronously
    pub async fn remove_file<P: AsRef<Path>>(path: P) -> IoResult<()> {
        tokio::fs::remove_file(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Remove an empty directory asynchronously
    pub async fn remove_dir<P: AsRef<Path>>(path: P) -> IoResult<()> {
        tokio::fs::remove_dir(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Remove a directory and all its contents asynchronously
    pub async fn remove_dir_all<P: AsRef<Path>>(path: P) -> IoResult<()> {
        tokio::fs::remove_dir_all(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Copy a file asynchronously
    pub async fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> IoResult<u64> {
        tokio::fs::copy(from, to).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))
    }

    /// Rename a file asynchronously
    pub async fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> IoResult<()> {
        tokio::fs::rename(from, to).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))?;
        Ok(())
    }

    /// Check if a path exists asynchronously
    pub async fn exists<P: AsRef<Path>>(path: P) -> bool {
        tokio::fs::try_exists(path).await.unwrap_or(false)
    }

    /// Get file metadata asynchronously
    pub async fn metadata<P: AsRef<Path>>(path: P) -> IoResult<std::fs::Metadata> {
        tokio::fs::metadata(path).await
            .map_err(|e| crate::stdlib::io::IoError::from(e))
    }
}

/// Async utilities for process operations
pub mod process {
    use super::*;
    use std::process::Stdio;

    /// Async process builder and executor
    pub struct AsyncCommand {
        command: tokio::process::Command,
    }

    impl AsyncCommand {
        /// Create a new async command
        pub fn new(program: &str) -> Self {
            Self {
                command: tokio::process::Command::new(program),
            }
        }

        /// Add an argument to the command
        pub fn arg(&mut self, arg: &str) -> &mut Self {
            self.command.arg(arg);
            self
        }

        /// Add multiple arguments to the command
        pub fn args(&mut self, args: &[&str]) -> &mut Self {
            self.command.args(args);
            self
        }

        /// Set environment variable
        pub fn env(&mut self, key: &str, value: &str) -> &mut Self {
            self.command.env(key, value);
            self
        }

        /// Set working directory
        pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
            self.command.current_dir(dir);
            self
        }

        /// Set stdin handling
        pub fn stdin(&mut self, cfg: Stdio) -> &mut Self {
            self.command.stdin(cfg);
            self
        }

        /// Set stdout handling
        pub fn stdout(&mut self, cfg: Stdio) -> &mut Self {
            self.command.stdout(cfg);
            self
        }

        /// Set stderr handling
        pub fn stderr(&mut self, cfg: Stdio) -> &mut Self {
            self.command.stderr(cfg);
            self
        }

        /// Execute the command and wait for completion
        pub async fn output(mut self) -> IoResult<std::process::Output> {
            self.command.output().await
                .map_err(|e| crate::stdlib::io::IoError::from(e))
        }

        /// Execute the command and get status
        pub async fn status(mut self) -> IoResult<std::process::ExitStatus> {
            self.command.status().await
                .map_err(|e| crate::stdlib::io::IoError::from(e))
        }

        /// Spawn the command and return a handle
        pub fn spawn(mut self) -> IoResult<AsyncChild> {
            let child = self.command.spawn()
                .map_err(|e| crate::stdlib::io::IoError::from(e))?;
            Ok(AsyncChild { inner: child })
        }
    }

    /// Handle for spawned async process
    pub struct AsyncChild {
        inner: tokio::process::Child,
    }

    impl AsyncChild {
        /// Wait for the process to complete
        pub async fn wait(mut self) -> IoResult<std::process::ExitStatus> {
            self.inner.wait().await
                .map_err(|e| crate::stdlib::io::IoError::from(e))
        }

        /// Wait for the process and capture output
        pub async fn wait_with_output(self) -> IoResult<std::process::Output> {
            self.inner.wait_with_output().await
                .map_err(|e| crate::stdlib::io::IoError::from(e))
        }

        /// Kill the process
        pub async fn kill(&mut self) -> IoResult<()> {
            self.inner.kill().await
                .map_err(|e| crate::stdlib::io::IoError::from(e))
        }

        /// Get the process ID
        pub fn id(&self) -> Option<u32> {
            self.inner.id()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::r#async::block_on;

    #[test]
    fn test_async_sleep() {
        block_on(async {
            let start = std::time::Instant::now();
            sleep(Duration::from_millis(100)).await;
            let elapsed = start.elapsed();
            assert!(elapsed >= Duration::from_millis(90));
        });
    }

    #[test]
    fn test_async_timeout() {
        block_on(async {
            // Test successful completion within timeout
            let result = timeout(Duration::from_millis(100), async {
                sleep(Duration::from_millis(50)).await;
                42
            }).await;
            assert_eq!(result.unwrap(), 42);

            // Test timeout
            let result = timeout(Duration::from_millis(50), async {
                sleep(Duration::from_millis(100)).await;
                42
            }).await;
            assert!(matches!(result, Err(FutureError::Timeout)));
        });
    }

    #[test]
    fn test_local_task_handle() {
        block_on(async {
            let handle = spawn_local(async { 42 });
            let result = handle.await;
            assert_eq!(result, 42);
        });
    }
}
