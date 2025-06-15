/// Async I/O operations for CURSED stdlib
use std::pin::Pin;
use std::task::{Context, Poll};
use std::io::{self, Read, Write, Seek, BufRead, BufReader, BufWriter};
use std::path::Path;

use crate::runtime::r#async::{Future, Promise, PromiseResolver};
use crate::stdlib::r#async::{AsyncError, AsyncResult};

/// Async read trait
pub trait AsyncReader {
    /// Read data into a buffer asynchronously
    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = AsyncResult<usize>>;

    /// Read exact number of bytes
    fn read_exact(&mut self, buf: &mut [u8]) -> impl Future<Output = AsyncResult<()>>;

    /// Read to end of stream
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> impl Future<Output = AsyncResult<usize>>;

    /// Read to string
    fn read_to_string(&mut self, buf: &mut String) -> impl Future<Output = AsyncResult<usize>>;
}

/// Async write trait
pub trait AsyncWriter {
    /// Write data from a buffer asynchronously
    fn write(&mut self, buf: &[u8]) -> impl Future<Output = AsyncResult<usize>>;

    /// Write all data
    fn write_all(&mut self, buf: &[u8]) -> impl Future<Output = AsyncResult<()>>;

    /// Flush the writer
    fn flush(&mut self) -> impl Future<Output = AsyncResult<()>>;
}

/// Async seek trait
pub trait AsyncSeeker {
    /// Seek to a position in the stream
    fn seek(&mut self, pos: io::SeekFrom) -> impl Future<Output = AsyncResult<u64>>;
}

/// Async buffered reader
pub struct AsyncBufReader<R> {
    inner: BufReader<R>,
}

impl<R> AsyncBufReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            inner: BufReader::new(reader),
        }
    }

    pub fn with_capacity(cap: usize, reader: R) -> Self {
        Self {
            inner: BufReader::with_capacity(cap, reader),
        }
    }

    pub fn into_inner(self) -> R {
        self.inner.into_inner()
    }

    /// Read a line asynchronously
    pub async fn read_line(&mut self, buf: &mut String) -> AsyncResult<usize>
    where
        R: Read + Send,
    {
        spawn_blocking_io(move || {
            self.inner.read_line(buf).map_err(AsyncError::from)
        }).await
    }

    /// Read until delimiter
    pub async fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> AsyncResult<usize>
    where
        R: Read + Send,
    {
        spawn_blocking_io(move || {
            self.inner.read_until(byte, buf).map_err(AsyncError::from)
        }).await
    }
}

impl<R: Read + Send> AsyncReader for AsyncBufReader<R> {
    async fn read(&mut self, buf: &mut [u8]) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.read(buf).map_err(AsyncError::from)
        }).await
    }

    async fn read_exact(&mut self, buf: &mut [u8]) -> AsyncResult<()> {
        spawn_blocking_io(move || {
            self.inner.read_exact(buf).map_err(AsyncError::from)
        }).await
    }

    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.read_to_end(buf).map_err(AsyncError::from)
        }).await
    }

    async fn read_to_string(&mut self, buf: &mut String) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.read_to_string(buf).map_err(AsyncError::from)
        }).await
    }
}

/// Async buffered writer
pub struct AsyncBufWriter<W> {
    inner: BufWriter<W>,
}

impl<W> AsyncBufWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            inner: BufWriter::new(writer),
        }
    }

    pub fn with_capacity(cap: usize, writer: W) -> Self {
        Self {
            inner: BufWriter::with_capacity(cap, writer),
        }
    }

    pub fn into_inner(self) -> Result<W, BufWriter<W>> {
        self.inner.into_inner()
    }
}

impl<W: Write + Send> AsyncWriter for AsyncBufWriter<W> {
    async fn write(&mut self, buf: &[u8]) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.write(buf).map_err(AsyncError::from)
        }).await
    }

    async fn write_all(&mut self, buf: &[u8]) -> AsyncResult<()> {
        spawn_blocking_io(move || {
            self.inner.write_all(buf).map_err(AsyncError::from)
        }).await
    }

    async fn flush(&mut self) -> AsyncResult<()> {
        spawn_blocking_io(move || {
            self.inner.flush().map_err(AsyncError::from)
        }).await
    }
}

/// Async standard input
pub struct AsyncStdin {
    inner: std::io::Stdin,
}

impl AsyncStdin {
    pub fn new() -> Self {
        Self {
            inner: std::io::stdin(),
        }
    }

    /// Read a line from stdin
    pub async fn read_line(&self, buf: &mut String) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.read_line(buf).map_err(AsyncError::from)
        }).await
    }
}

impl AsyncReader for AsyncStdin {
    async fn read(&mut self, buf: &mut [u8]) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.read(buf).map_err(AsyncError::from)
        }).await
    }

    async fn read_exact(&mut self, buf: &mut [u8]) -> AsyncResult<()> {
        spawn_blocking_io(move || {
            self.inner.read_exact(buf).map_err(AsyncError::from)
        }).await
    }

    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.read_to_end(buf).map_err(AsyncError::from)
        }).await
    }

    async fn read_to_string(&mut self, buf: &mut String) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.read_to_string(buf).map_err(AsyncError::from)
        }).await
    }
}

/// Async standard output
pub struct AsyncStdout {
    inner: std::io::Stdout,
}

impl AsyncStdout {
    pub fn new() -> Self {
        Self {
            inner: std::io::stdout(),
        }
    }
}

impl AsyncWriter for AsyncStdout {
    async fn write(&mut self, buf: &[u8]) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.write(buf).map_err(AsyncError::from)
        }).await
    }

    async fn write_all(&mut self, buf: &[u8]) -> AsyncResult<()> {
        spawn_blocking_io(move || {
            self.inner.write_all(buf).map_err(AsyncError::from)
        }).await
    }

    async fn flush(&mut self) -> AsyncResult<()> {
        spawn_blocking_io(move || {
            self.inner.flush().map_err(AsyncError::from)
        }).await
    }
}

/// Async standard error
pub struct AsyncStderr {
    inner: std::io::Stderr,
}

impl AsyncStderr {
    pub fn new() -> Self {
        Self {
            inner: std::io::stderr(),
        }
    }
}

impl AsyncWriter for AsyncStderr {
    async fn write(&mut self, buf: &[u8]) -> AsyncResult<usize> {
        spawn_blocking_io(move || {
            self.inner.write(buf).map_err(AsyncError::from)
        }).await
    }

    async fn write_all(&mut self, buf: &[u8]) -> AsyncResult<()> {
        spawn_blocking_io(move || {
            self.inner.write_all(buf).map_err(AsyncError::from)
        }).await
    }

    async fn flush(&mut self) -> AsyncResult<()> {
        spawn_blocking_io(move || {
            self.inner.flush().map_err(AsyncError::from)
        }).await
    }
}

/// Get async stdin
pub fn stdin_async() -> AsyncStdin {
    AsyncStdin::new()
}

/// Get async stdout
pub fn stdout_async() -> AsyncStdout {
    AsyncStdout::new()
}

/// Get async stderr
pub fn stderr_async() -> AsyncStderr {
    AsyncStderr::new()
}

/// Read entire contents to string
pub async fn read_to_string<P: AsRef<Path>>(path: P) -> AsyncResult<String> {
    spawn_blocking_io(move || {
        std::fs::read_to_string(path).map_err(AsyncError::from)
    }).await
}

/// Read entire contents to vec
pub async fn read_to_vec<P: AsRef<Path>>(path: P) -> AsyncResult<Vec<u8>> {
    spawn_blocking_io(move || {
        std::fs::read(path).map_err(AsyncError::from)
    }).await
}

/// Write all data to file
pub async fn write_all<P: AsRef<Path>>(path: P, contents: &[u8]) -> AsyncResult<()> {
    let contents = contents.to_vec();
    spawn_blocking_io(move || {
        std::fs::write(path, contents).map_err(AsyncError::from)
    }).await
}

/// Copy from reader to writer
pub async fn copy<R, W>(reader: &mut R, writer: &mut W) -> AsyncResult<u64>
where
    R: AsyncReader,
    W: AsyncWriter,
{
    let mut buf = vec![0; 8192]; // 8KB buffer
    let mut total = 0;

    loop {
        let bytes_read = reader.read(&mut buf).await?;
        if bytes_read == 0 {
            break;
        }

        writer.write_all(&buf[..bytes_read]).await?;
        total += bytes_read as u64;
    }

    writer.flush().await?;
    Ok(total)
}

/// Helper function to spawn blocking I/O operation
async fn spawn_blocking_io<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let (promise, resolver, _rejecter) = Promise::new();

    std::thread::spawn(move || {
        let result = f();
        let _ = resolver.resolve(result);
    });

    promise.await.unwrap_or_else(|_| panic!("IO operation failed"))
}

/// Public helper function for spawning blocking operations
pub async fn spawn_blocking_io_public<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    spawn_blocking_io(f).await
}

/// Async file operations module
pub mod file {
    use super::*;
    use std::fs::{File, OpenOptions};
    use std::path::Path;

    /// Async file wrapper
    pub struct AsyncFile {
        file: File,
    }

    impl AsyncFile {
        /// Open file for reading
        pub async fn open<P: AsRef<Path>>(path: P) -> AsyncResult<Self> {
            spawn_blocking_io(move || {
                let file = File::open(path)?;
                Ok(AsyncFile { file })
            }).await
        }

        /// Create new file for writing
        pub async fn create<P: AsRef<Path>>(path: P) -> AsyncResult<Self> {
            spawn_blocking_io(move || {
                let file = File::create(path)?;
                Ok(AsyncFile { file })
            }).await
        }

        /// Open file with options
        pub async fn open_with_options<P: AsRef<Path>>(
            path: P,
            options: &OpenOptions,
        ) -> AsyncResult<Self> {
            let options = options.clone();
            spawn_blocking_io(move || {
                let file = options.open(path)?;
                Ok(AsyncFile { file })
            }).await
        }

        /// Get file metadata
        pub async fn metadata(&self) -> AsyncResult<std::fs::Metadata> {
            spawn_blocking_io(move || {
                self.file.metadata().map_err(AsyncError::from)
            }).await
        }

        /// Sync all data to disk
        pub async fn sync_all(&self) -> AsyncResult<()> {
            spawn_blocking_io(move || {
                self.file.sync_all().map_err(AsyncError::from)
            }).await
        }

        /// Sync data (not metadata) to disk
        pub async fn sync_data(&self) -> AsyncResult<()> {
            spawn_blocking_io(move || {
                self.file.sync_data().map_err(AsyncError::from)
            }).await
        }
    }

    impl AsyncReader for AsyncFile {
        async fn read(&mut self, buf: &mut [u8]) -> AsyncResult<usize> {
            spawn_blocking_io(move || {
                self.file.read(buf).map_err(AsyncError::from)
            }).await
        }

        async fn read_exact(&mut self, buf: &mut [u8]) -> AsyncResult<()> {
            spawn_blocking_io(move || {
                self.file.read_exact(buf).map_err(AsyncError::from)
            }).await
        }

        async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> AsyncResult<usize> {
            spawn_blocking_io(move || {
                self.file.read_to_end(buf).map_err(AsyncError::from)
            }).await
        }

        async fn read_to_string(&mut self, buf: &mut String) -> AsyncResult<usize> {
            spawn_blocking_io(move || {
                self.file.read_to_string(buf).map_err(AsyncError::from)
            }).await
        }
    }

    impl AsyncWriter for AsyncFile {
        async fn write(&mut self, buf: &[u8]) -> AsyncResult<usize> {
            spawn_blocking_io(move || {
                self.file.write(buf).map_err(AsyncError::from)
            }).await
        }

        async fn write_all(&mut self, buf: &[u8]) -> AsyncResult<()> {
            spawn_blocking_io(move || {
                self.file.write_all(buf).map_err(AsyncError::from)
            }).await
        }

        async fn flush(&mut self) -> AsyncResult<()> {
            spawn_blocking_io(move || {
                self.file.flush().map_err(AsyncError::from)
            }).await
        }
    }

    impl AsyncSeeker for AsyncFile {
        async fn seek(&mut self, pos: io::SeekFrom) -> AsyncResult<u64> {
            spawn_blocking_io(move || {
                self.file.seek(pos).map_err(AsyncError::from)
            }).await
        }
    }
}

pub use file::AsyncFile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_stdin_creation() {
        let stdin = stdin_async();
        // Would need proper async test framework for polling
    }

    #[test]
    fn test_async_stdout_creation() {
        let stdout = stdout_async();
        // Would need proper async test framework for polling
    }

    #[test]
    fn test_async_buf_reader_creation() {
        let data = b"hello world";
        let cursor = std::io::Cursor::new(data);
        let reader = AsyncBufReader::new(cursor);
        // Would need proper async test framework for testing read operations
    }
}
