use crate::error::CursedError;
/// Stream handle management for CURSED console I/O operations
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::sync::{Arc, Mutex};
// use crate::stdlib::io::error::{IoError, IoResult};

/// Handle to standard input stream
#[derive(Debug, Clone)]
pub struct Stdin {
impl Stdin {
    /// Create new stdin handle
    pub fn new() -> Self {
        Self {
        }
    }

    /// Read a line from stdin
    pub fn read_line(&self) -> IoResult<String> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stdin".to_string()))?;
        
        let mut line = String::new();
        guard.read_line(&mut line)?;
        
        // Remove trailing newline if present
        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }
        
        Ok(line)
    /// Read all input until EOF
    pub fn read_all(&self) -> IoResult<String> {
        use std::io::Read;
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stdin".to_string()))?;
        
        let mut buffer = String::new();
        guard.read_to_string(&mut buffer)?;
        Ok(buffer)
    /// Read until a specific delimiter
    pub fn read_until(&self, delimiter: u8) -> IoResult<String> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stdin".to_string()))?;
        
        let mut buffer = Vec::new();
        guard.read_until(delimiter, &mut buffer)?;
        
        // Remove delimiter if present
        if buffer.last() == Some(&delimiter) {
            buffer.pop();
        String::from_utf8(buffer).map_err(|_| IoError::InvalidUtf8)
    }
}

impl Default for Stdin {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle to standard output stream
#[derive(Debug, Clone)]
pub struct Stdout {
impl Stdout {
    /// Create new stdout handle
    pub fn new() -> Self {
        Self {
        }
    }

    /// Write string to stdout without newline
    pub fn print(&self, s: &str) -> IoResult<()> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stdout".to_string()))?;
        
        guard.write_all(s.as_bytes())?;
        Ok(())
    /// Write string to stdout with newline
    pub fn println(&self, s: &str) -> IoResult<()> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stdout".to_string()))?;
        
        guard.write_all(s.as_bytes())?;
        guard.write_all(b"\n")?;
        Ok(())
    /// Flush the output buffer
    pub fn flush(&self) -> IoResult<()> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stdout".to_string()))?;
        
        guard.flush()?;
        Ok(())
    }
}

impl Default for Stdout {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle to standard error stream
#[derive(Debug, Clone)]
pub struct Stderr {
impl Stderr {
    /// Create new stderr handle
    pub fn new() -> Self {
        Self {
        }
    }

    /// Write string to stderr without newline
    pub fn eprint(&self, s: &str) -> IoResult<()> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stderr".to_string()))?;
        
        guard.write_all(s.as_bytes())?;
        Ok(())
    /// Write string to stderr with newline
    pub fn eprintln(&self, s: &str) -> IoResult<()> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stderr".to_string()))?;
        
        guard.write_all(s.as_bytes())?;
        guard.write_all(b"\n")?;
        Ok(())
    /// Flush the error buffer
    pub fn flush(&self) -> IoResult<()> {
        let mut guard = self.inner.lock()
            .map_err(|_| IoError::General("Failed to lock stderr".to_string()))?;
        
        guard.flush()?;
        Ok(())
    }
}

impl Default for Stderr {
    fn default() -> Self {
        Self::new()
    }
}

/// Global stream handles
static mut STDIN_HANDLE: Option<Stdin> = None;
static mut STDOUT_HANDLE: Option<Stdout> = None;
static mut STDERR_HANDLE: Option<Stderr> = None;

/// Get global stdin handle
pub fn stdin() -> Stdin {
    unsafe {
        STDIN_HANDLE.get_or_insert_with(Stdin::new).clone()
    }
}

/// Get global stdout handle
pub fn stdout() -> Stdout {
    unsafe {
        STDOUT_HANDLE.get_or_insert_with(Stdout::new).clone()
    }
}

/// Get global stderr handle
pub fn stderr() -> Stderr {
    unsafe {
        STDERR_HANDLE.get_or_insert_with(Stderr::new).clone()
    }
}

/// Flush all output streams
pub fn flush_all() -> IoResult<()> {
    stdout().flush()?;
    stderr().flush()?;
    Ok(())
}
