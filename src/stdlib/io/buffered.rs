use crate::error::Error;
/// Buffered I/O implementations for efficient CURSED console operations
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::sync::{Arc, Mutex};
use crate::stdlib::io::error::{IoError, IoResult};

/// Default buffer size for I/O operations
const DEFAULT_BUFFER_SIZE: usize = 8192;

/// Buffered reader for efficient input operations
pub struct BufferedReader<R: Read> {
    inner: BufReader<R>,
    line_number: usize,
}

impl<R: Read> BufferedReader<R> {
    /// Create a new buffered reader with default buffer size
    pub fn new(reader: R) -> Self {
        Self {
            inner: BufReader::new(reader),
            line_number: 0,
        }
    }

    /// Create a new buffered reader with specified buffer size
    pub fn with_capacity(capacity: usize, reader: R) -> Self {
        Self {
            inner: BufReader::with_capacity(capacity, reader),
            line_number: 0,
        }
    }

    /// Read a line from the buffered reader
    pub fn read_line(&mut self) -> IoResult<Option<String>> {
        let mut line = String::new();
        match self.inner.read_line(&mut line)? {
            0 => Ok(None), // EOF
            _ => {
                self.line_number += 1;
                // Remove trailing newline if present
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                Ok(Some(line))
            }
        }
    }

    /// Read all lines into a vector
    pub fn read_lines(&mut self) -> IoResult<Vec<String>> {
        let mut lines = Vec::new();
        while let Some(line) = self.read_line()? {
            lines.push(line);
        }
        Ok(lines)
    }

    /// Read until a delimiter
    pub fn read_until(&mut self, delimiter: u8) -> IoResult<Vec<u8>> {
        let mut buffer = Vec::new();
        self.inner.read_until(delimiter, &mut buffer)?;
        Ok(buffer)
    }

    /// Read all remaining content
    pub fn read_to_end(&mut self) -> IoResult<Vec<u8>> {
        let mut buffer = Vec::new();
        self.inner.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Read all remaining content as string
    pub fn read_to_string(&mut self) -> IoResult<String> {
        let mut string = String::new();
        self.inner.read_to_string(&mut string)?;
        Ok(string)
    }

    /// Get the current line number
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    /// Check if the reader has data available
    pub fn has_data_in_buffer(&self) -> bool {
        self.inner.buffer().len() > 0
    }

    /// Get the number of bytes in the buffer
    pub fn buffer_len(&self) -> usize {
        self.inner.buffer().len()
    }
}

/// Buffered writer for efficient output operations
pub struct BufferedWriter<W: Write> {
    inner: BufWriter<W>,
    bytes_written: usize,
}

impl<W: Write> BufferedWriter<W> {
    /// Create a new buffered writer with default buffer size
    pub fn new(writer: W) -> Self {
        Self {
            inner: BufWriter::new(writer),
            bytes_written: 0,
        }
    }

    /// Create a new buffered writer with specified buffer size
    pub fn with_capacity(capacity: usize, writer: W) -> Self {
        Self {
            inner: BufWriter::with_capacity(capacity, writer),
            bytes_written: 0,
        }
    }

    /// Write bytes to the buffered writer
    pub fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        let bytes = self.inner.write(buf)?;
        self.bytes_written += bytes;
        Ok(bytes)
    }

    /// Write all bytes to the buffered writer
    pub fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        self.inner.write_all(buf)?;
        self.bytes_written += buf.len();
        Ok(())
    }

    /// Write a string to the buffered writer
    pub fn write_str(&mut self, s: &str) -> IoResult<()> {
        self.write_all(s.as_bytes())
    }

    /// Write a string with newline to the buffered writer
    pub fn write_line(&mut self, s: &str) -> IoResult<()> {
        self.write_str(s)?;
        self.write_all(b"\n")
    }

    /// Flush the buffer
    pub fn flush(&mut self) -> IoResult<()> {
        self.inner.flush()?;
        Ok(())
    }

    /// Get the number of bytes written
    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }

    /// Get the buffer capacity
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }
}

/// Thread-safe buffered reader
pub struct SharedBufferedReader<R: Read + Send> {
    inner: Arc<Mutex<BufferedReader<R>>>,
}

impl<R: Read + Send> SharedBufferedReader<R> {
    /// Create a new shared buffered reader
    pub fn new(reader: R) -> Self {
        Self {
            inner: Arc::new(Mutex::new(BufferedReader::new(reader))),
        }
    }

    /// Create a new shared buffered reader with specified buffer size
    pub fn with_capacity(capacity: usize, reader: R) -> Self {
        Self {
            inner: Arc::new(Mutex::new(BufferedReader::with_capacity(capacity, reader))),
        }
    }

    /// Read a line from the shared buffered reader
    pub fn read_line(&self) -> IoResult<Option<String>> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock reader".to_string()))?;
        guard.read_line()
    }

    /// Read all lines from the shared buffered reader
    pub fn read_lines(&self) -> IoResult<Vec<String>> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock reader".to_string()))?;
        guard.read_lines()
    }

    /// Get the current line number
    pub fn line_number(&self) -> IoResult<usize> {
        let guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock reader".to_string()))?;
        Ok(guard.line_number())
    }
}

impl<R: Read + Send> Clone for SharedBufferedReader<R> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Thread-safe buffered writer
pub struct SharedBufferedWriter<W: Write + Send> {
    inner: Arc<Mutex<BufferedWriter<W>>>,
}

impl<W: Write + Send> SharedBufferedWriter<W> {
    /// Create a new shared buffered writer
    pub fn new(writer: W) -> Self {
        Self {
            inner: Arc::new(Mutex::new(BufferedWriter::new(writer))),
        }
    }

    /// Create a new shared buffered writer with specified buffer size
    pub fn with_capacity(capacity: usize, writer: W) -> Self {
        Self {
            inner: Arc::new(Mutex::new(BufferedWriter::with_capacity(capacity, writer))),
        }
    }

    /// Write bytes to the shared buffered writer
    pub fn write(&self, buf: &[u8]) -> IoResult<usize> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock writer".to_string()))?;
        guard.write(buf)
    }

    /// Write all bytes to the shared buffered writer
    pub fn write_all(&self, buf: &[u8]) -> IoResult<()> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock writer".to_string()))?;
        guard.write_all(buf)
    }

    /// Write a string to the shared buffered writer
    pub fn write_str(&self, s: &str) -> IoResult<()> {
        self.write_all(s.as_bytes())
    }

    /// Write a string with newline to the shared buffered writer
    pub fn write_line(&self, s: &str) -> IoResult<()> {
        self.write_str(s)?;
        self.write_all(b"\n")
    }

    /// Flush the buffer
    pub fn flush(&self) -> IoResult<()> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock writer".to_string()))?;
        guard.flush()
    }

    /// Get the number of bytes written
    pub fn bytes_written(&self) -> IoResult<usize> {
        let guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock writer".to_string()))?;
        Ok(guard.bytes_written())
    }
}

impl<W: Write + Send> Clone for SharedBufferedWriter<W> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Create a buffered stdin reader
pub fn buffered_stdin() -> BufferedReader<std::io::Stdin> {
    BufferedReader::new(std::io::stdin())
}

/// Create a buffered stdout writer
pub fn buffered_stdout() -> BufferedWriter<std::io::Stdout> {
    BufferedWriter::new(std::io::stdout())
}

/// Create a buffered stderr writer
pub fn buffered_stderr() -> BufferedWriter<std::io::Stderr> {
    BufferedWriter::new(std::io::stderr())
}

/// Create a shared buffered stdin reader
pub fn shared_buffered_stdin() -> SharedBufferedReader<std::io::Stdin> {
    SharedBufferedReader::new(std::io::stdin())
}

/// Create a shared buffered stdout writer
pub fn shared_buffered_stdout() -> SharedBufferedWriter<std::io::Stdout> {
    SharedBufferedWriter::new(std::io::stdout())
}

/// Create a shared buffered stderr writer
pub fn shared_buffered_stderr() -> SharedBufferedWriter<std::io::Stderr> {
    SharedBufferedWriter::new(std::io::stderr())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_buffered_reader_line_reading() {
        let data = "line1\nline2\nline3";
        let cursor = Cursor::new(data.as_bytes());
        let mut reader = BufferedReader::new(cursor);

        assert_eq!(reader.read_line().unwrap(), Some("line1".to_string()));
        assert_eq!(reader.line_number(), 1);
        assert_eq!(reader.read_line().unwrap(), Some("line2".to_string()));
        assert_eq!(reader.line_number(), 2);
        assert_eq!(reader.read_line().unwrap(), Some("line3".to_string()));
        assert_eq!(reader.line_number(), 3);
        assert_eq!(reader.read_line().unwrap(), None);
    }

    #[test]
    fn test_buffered_reader_read_all_lines() {
        let data = "line1\nline2\nline3";
        let cursor = Cursor::new(data.as_bytes());
        let mut reader = BufferedReader::new(cursor);

        let lines = reader.read_lines().unwrap();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
        assert_eq!(reader.line_number(), 3);
    }

    #[test]
    fn test_buffered_writer_writing() {
        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufferedWriter::new(cursor);

            writer.write_line("Hello").unwrap();
            writer.write_line("World").unwrap();
            writer.flush().unwrap();
        }

        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Hello\nWorld\n");
    }

    #[test]
    fn test_buffered_writer_bytes_written() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let mut writer = BufferedWriter::new(cursor);

        writer.write_str("Hello").unwrap();
        assert_eq!(writer.bytes_written(), 5);

        writer.write_str(" World").unwrap();
        assert_eq!(writer.bytes_written(), 11);
    }

    #[test]
    fn test_buffered_reader_with_capacity() {
        let data = "test data";
        let cursor = Cursor::new(data.as_bytes());
        let reader = BufferedReader::with_capacity(1024, cursor);
        
        // Just test that it was created successfully
        assert_eq!(reader.line_number(), 0);
    }

    #[test]
    fn test_buffered_writer_with_capacity() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let writer = BufferedWriter::with_capacity(1024, cursor);
        
        // Test capacity
        assert_eq!(writer.capacity(), 1024);
        assert_eq!(writer.bytes_written(), 0);
    }
}
