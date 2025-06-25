use crate::error::CursedError;
/// Named pipes (FIFO) implementation for CURSED IPC
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::os::unix::fs::OpenOptionsExt;
use std::time::Duration;
// use crate::stdlib::ipc::error::{IpcError, IpcResult};

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};

/// Configuration for named pipe creation
#[derive(Debug, Clone)]
pub struct PipeConfig {
impl PipeConfig {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            permissions: 0o600, // Owner read/write only by default
            buffer_size: 8192,  // 8KB buffer
        }
    }

    pub fn with_mode(mut self, mode: PipeMode) -> Self {
        self.mode = mode;
        self
    pub fn with_permissions(mut self, permissions: u32) -> Self {
        self.permissions = permissions;
        self
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

/// Pipe access mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PipeMode {
/// Handle to a named pipe
#[derive(Debug, Clone)]
pub struct PipeHandle {
impl PipeHandle {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

/// Named pipe for inter-process communication
pub struct NamedPipe {
    #[cfg(unix)]
impl NamedPipe {
    /// Create a new named pipe
    pub fn create<P: AsRef<Path>>(path: P) -> IpcResult<Self> {
        let config = PipeConfig::new(path);
        Self::create_with_config(config)
    /// Create a named pipe with custom configuration
    pub fn create_with_config(config: PipeConfig) -> IpcResult<Self> {
        #[cfg(unix)]
        {
            // Create FIFO using mkfifo
            let path_cstr = std::ffi::CString::new(config.path.to_string_lossy().as_bytes())?;
            let result = unsafe { libc::mkfifo(path_cstr.as_ptr(), config.permissions) };
            
            if result != 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                if errno != libc::EEXIST {
                    return Err(IpcError::System(errno, format!("Failed to create FIFO: {}", config.path.display())));
                }
            }
        #[cfg(not(unix))]
        {
            // On non-Unix systems, create a regular file as fallback
            // This is not a true FIFO but provides basic file-based communication
            if !config.path.exists() {
                File::create(&config.path)?;
            }
        }

        let mut pipe = Self {
            #[cfg(unix)]

        pipe.open()?;
        
        // Register with IPC registry
        let handle = PipeHandle::new(pipe.config.path.to_string_lossy().to_string());
//         crate::stdlib::ipc::register_pipe(pipe.config.path.to_string_lossy().to_string(), handle)?;
        
        Ok(pipe)
    /// Open an existing named pipe
    pub fn open<P: AsRef<Path>>(path: P) -> IpcResult<Self> {
        let config = PipeConfig::new(path);
        
        if !config.path.exists() {
            return Err(IpcError::NotFound(format!("Pipe does not exist: {}", config.path.display())));
        let mut pipe = Self {
            #[cfg(unix)]

        pipe.open()?;
        Ok(pipe)
    /// Open the pipe for reading/writing based on configuration
    fn open(&mut self) -> IpcResult<()> {
        match self.config.mode {
            PipeMode::ReadOnly => {
                let file = OpenOptions::new()
                    .read(true)
                    .open(&self.config.path)?;
                
                #[cfg(unix)]
                {
                    self.fd = Some(file.as_raw_fd());
                self.reader = Some(BufReader::with_capacity(self.config.buffer_size, file));
            }
            PipeMode::WriteOnly => {
                let file = OpenOptions::new()
                    .write(true)
                    .open(&self.config.path)?;
                
                #[cfg(unix)]
                {
                    self.fd = Some(file.as_raw_fd());
                self.writer = Some(BufWriter::with_capacity(self.config.buffer_size, file));
            }
            PipeMode::ReadWrite => {
                // Open separate handles for reading and writing
                let read_file = OpenOptions::new()
                    .read(true)
                    .open(&self.config.path)?;
                
                let write_file = OpenOptions::new()
                    .write(true)
                    .open(&self.config.path)?;
                
                #[cfg(unix)]
                {
                    self.fd = Some(read_file.as_raw_fd());
                self.reader = Some(BufReader::with_capacity(self.config.buffer_size, read_file));
                self.writer = Some(BufWriter::with_capacity(self.config.buffer_size, write_file));
            }
        }
        
        Ok(())
    /// Write data to the pipe
    pub fn write(&mut self, data: &[u8]) -> IpcResult<usize> {
        match &mut self.writer {
            Some(writer) => {
//                 crate::stdlib::ipc::increment_operations();
                writer.write(data).map_err(|e| {
//                     crate::stdlib::ipc::increment_failed_operations();
                    IpcError::from(e)
                })
            }
            None => {
//                 crate::stdlib::ipc::increment_failed_operations();
                Err(IpcError::InvalidOperation("Pipe not open for writing".to_string()))
            }
        }
    /// Write a string to the pipe
    pub fn write_string(&mut self, s: &str) -> IpcResult<usize> {
        self.write(s.as_bytes())
    /// Write a line to the pipe (adds newline)
    pub fn write_line(&mut self, s: &str) -> IpcResult<usize> {
        let mut total = self.write(s.as_bytes())?;
        total += self.write(b"\n")?;
        Ok(total)
    /// Read data from the pipe
    pub fn read(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        match &mut self.reader {
            Some(reader) => {
//                 crate::stdlib::ipc::increment_operations();
                reader.read(buffer).map_err(|e| {
//                     crate::stdlib::ipc::increment_failed_operations();
                    IpcError::from(e)
                })
            }
            None => {
//                 crate::stdlib::ipc::increment_failed_operations();
                Err(IpcError::InvalidOperation("Pipe not open for reading".to_string()))
            }
        }
    /// Read a line from the pipe
    pub fn read_line(&mut self) -> IpcResult<String> {
        match &mut self.reader {
            Some(reader) => {
//                 crate::stdlib::ipc::increment_operations();
                let mut line = String::new();
                match std::io::BufRead::read_line(reader, &mut line) {
                    Ok(0) => {
//                         crate::stdlib::ipc::increment_failed_operations();
                        Err(IpcError::IoError("End of pipe".to_string()))
                    }
                    Ok(_) => {
                        // Remove trailing newline
                        if line.ends_with('\n') {
                            line.pop();
                            if line.ends_with('\r') {
                                line.pop();
                            }
                        }
                        Ok(line)
                    }
                    Err(e) => {
//                         crate::stdlib::ipc::increment_failed_operations();
                        Err(IpcError::from(e))
                    }
                }
            }
            None => {
//                 crate::stdlib::ipc::increment_failed_operations();
                Err(IpcError::InvalidOperation("Pipe not open for reading".to_string()))
            }
        }
    /// Read all available data as a string
    pub fn read_string(&mut self) -> IpcResult<String> {
        match &mut self.reader {
            Some(reader) => {
//                 crate::stdlib::ipc::increment_operations();
                let mut content = String::new();
                match std::io::Read::read_to_string(reader, &mut content) {
                    Err(e) => {
//                         crate::stdlib::ipc::increment_failed_operations();
                        Err(IpcError::from(e))
                    }
                }
            }
            None => {
//                 crate::stdlib::ipc::increment_failed_operations();
                Err(IpcError::InvalidOperation("Pipe not open for reading".to_string()))
            }
        }
    /// Flush any pending writes
    pub fn flush(&mut self) -> IpcResult<()> {
        if let Some(writer) = &mut self.writer {
            writer.flush().map_err(IpcError::from)
        } else {
            Ok(())
        }
    }

    /// Get the pipe path
    pub fn path(&self) -> &Path {
        &self.config.path
    /// Check if pipe is open for reading
    pub fn can_read(&self) -> bool {
        self.reader.is_some()
    /// Check if pipe is open for writing
    pub fn can_write(&self) -> bool {
        self.writer.is_some()
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        let _ = self.flush();
//         let _ = crate::stdlib::ipc::unregister_pipe(&self.config.path.to_string_lossy());
    }
}

/// Create a named pipe
pub fn create_named_pipe<P: AsRef<Path>>(path: P) -> IpcResult<NamedPipe> {
    NamedPipe::create(path)
/// Open an existing named pipe
pub fn open_named_pipe<P: AsRef<Path>>(path: P) -> IpcResult<NamedPipe> {
    NamedPipe::open(path)
/// Remove a named pipe
pub fn remove_named_pipe<P: AsRef<Path>>(path: P) -> IpcResult<()> {
    let path = path.as_ref();
    if path.exists() {
        std::fs::remove_file(path).map_err(IpcError::from)?;
    }
    Ok(())
