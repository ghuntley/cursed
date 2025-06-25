use crate::error::CursedError;
/// Named pipes (FIFOs) implementation for CURSED IPC
/// 
/// Provides cross-platform named pipe functionality for inter-process communication

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::thread;

// use crate::stdlib::ipc::error::{IpcError, IpcResult, named_pipe_error, system_error, timeout_error};

/// Named pipe registry for cleanup
static PIPE_REGISTRY: std::sync::OnceLock<Arc<RwLock<HashMap<String, Arc<NamedPipeInfo>>>>> = std::sync::OnceLock::new();

#[derive(Debug)]
struct NamedPipeInfo {
fn get_pipe_registry() -> &'static Arc<RwLock<HashMap<String, Arc<NamedPipeInfo>>>> {
    PIPE_REGISTRY.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
/// Named pipe configuration
#[derive(Debug, Clone)]
pub struct NamedPipeConfig {
    /// Buffer size for reading/writing
    /// File permissions (Unix only)
    /// Whether to create the pipe if it doesn't exist
    /// Timeout for operations
    /// Maximum number of pending connections (server mode)
impl Default for NamedPipeConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Cross-platform named pipe
#[derive(Debug)]
pub struct NamedPipe {
    #[cfg(unix)]
    #[cfg(windows)]
impl NamedPipe {
    /// Create a new named pipe
    pub fn new<P: AsRef<Path>>(name: &str, path: P) -> Self {
        Self {
            #[cfg(unix)]
            #[cfg(windows)]
        }
    }

    /// Create with configuration
    pub fn with_config<P: AsRef<Path>>(name: &str, path: P, config: NamedPipeConfig) -> Self {
        Self {
            #[cfg(unix)]
            #[cfg(windows)]
        }
    }

    /// Create and open a named pipe (for compatibility)
    pub fn create(name: &str) -> IpcResult<Self> {
        let path = format!("/tmp/{}", name);
        let mut pipe = Self::new(name, &path);
        pipe.create_pipe()?;
        Ok(pipe)
    /// Open pipe for reading
    pub fn open_read(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            self.open_unix_read()
        #[cfg(windows)]
        {
            self.open_windows_read()
        }
    }

    /// Open pipe for writing
    pub fn open_write(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            self.open_unix_write()
        #[cfg(windows)]
        {
            self.open_windows_write()
        }
    }

    /// Read data from pipe
    pub fn read(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        #[cfg(unix)]
        {
            if let Some(ref mut file) = self.file {
                file.read(buffer)
                    .map_err(|e| named_pipe_error(&self.name, "read", &e.to_string()))
            } else {
                Err(named_pipe_error(&self.name, "read", "Pipe not open"))
            }
        }
        
        #[cfg(windows)]
        {
            self.read_windows(buffer)
        }
    }

    /// Write data to pipe
    pub fn write(&mut self, data: &[u8]) -> IpcResult<usize> {
        #[cfg(unix)]
        {
            if let Some(ref mut file) = self.file {
                file.write(data)
                    .map_err(|e| named_pipe_error(&self.name, "write", &e.to_string()))
            } else {
                Err(named_pipe_error(&self.name, "write", "Pipe not open"))
            }
        }
        
        #[cfg(windows)]
        {
            self.write_windows(data)
        }
    }

    /// Read line from pipe
    pub fn read_line(&mut self) -> IpcResult<String> {
        #[cfg(unix)]
        {
            if let Some(ref file) = self.file {
                let mut reader = BufReader::new(file);
                let mut line = String::new();
                reader.read_line(&mut line)
                    .map_err(|e| named_pipe_error(&self.name, "read_line", &e.to_string()))?;
                Ok(line)
            } else {
                Err(named_pipe_error(&self.name, "read_line", "Pipe not open"))
            }
        }
        
        #[cfg(windows)]
        {
            // Windows implementation would use ReadFile with line parsing
            let mut buffer = vec![0u8; self.config.buffer_size];
            let bytes_read = self.read_windows(&mut buffer)?;
            let data = String::from_utf8_lossy(&buffer[..bytes_read]);
            
            // Find first newline
            if let Some(pos) = data.find('\n') {
                Ok(data[..=pos].to_string())
            } else {
                Ok(data.to_string())
            }
        }
    /// Write line to pipe
    pub fn write_line(&mut self, line: &str) -> IpcResult<()> {
        let mut data = line.to_string();
        if !data.ends_with('\n') {
            data.push('\n');
        }
        self.write(data.as_bytes())?;
        self.flush()?;
        Ok(())
    /// Flush pipe
    pub fn flush(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            if let Some(ref mut file) = self.file {
                file.flush()
                    .map_err(|e| named_pipe_error(&self.name, "flush", &e.to_string()))
            } else {
                Ok(())
            }
        }
        
        #[cfg(windows)]
        {
            // Windows pipes are typically automatically flushed
            Ok(())
        }
    }

    /// Close pipe
    pub fn close(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            if self.file.is_some() {
                self.file = None;
                self.is_connected = false;
            }
        }
        
        #[cfg(windows)]
        {
            if let Some(handle) = self.handle {
                unsafe {
                    CloseHandle(handle as *mut std::ffi::c_void);
                }
                self.handle = None;
                self.is_connected = false;
            }
        }
        
        self.unregister_pipe();
        Ok(())
    /// Check if pipe is connected
    pub fn is_connected(&self) -> bool {
        self.is_connected
    /// Get pipe name
    pub fn name(&self) -> &str {
        &self.name
    /// Get pipe path
    pub fn path(&self) -> &Path {
        &self.path
    #[cfg(unix)]
    fn open_unix_read(&mut self) -> IpcResult<()> {
        self.create_fifo_if_needed()?;
        
        let file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| named_pipe_error(&self.name, "open_read", &e.to_string()))?;
        
        self.file = Some(file);
        self.is_connected = true;
        self.register_pipe();
        Ok(())
    #[cfg(unix)]
    fn open_unix_write(&mut self) -> IpcResult<()> {
        self.create_fifo_if_needed()?;
        
        let file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .map_err(|e| named_pipe_error(&self.name, "open_write", &e.to_string()))?;
        
        self.file = Some(file);
        self.is_connected = true;
        self.register_pipe();
        Ok(())
    #[cfg(unix)]
    fn create_fifo_if_needed(&self) -> IpcResult<()> {
        if self.path.exists() {
            return Ok(());
        if !self.config.create_if_missing {
            return Err(named_pipe_error(&self.name, "create", "Pipe does not exist and create_if_missing is false"));
        // Create FIFO using mkfifo system call
        unsafe {
            let path_cstr = std::ffi::CString::new(self.path.to_string_lossy().as_bytes())
                .map_err(|e| named_pipe_error(&self.name, "create", &e.to_string()))?;
            
            let result = libc::mkfifo(path_cstr.as_ptr(), self.config.permissions);
            if result != 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                if errno != libc::EEXIST {
                    return Err(system_error(errno, "mkfifo", "Failed to create FIFO"));
                }
            }
        Ok(())
    #[cfg(windows)]
    fn open_windows_read(&mut self) -> IpcResult<()> {
        self.create_named_pipe_if_needed()?;
        
        let pipe_name = format!(r"\\.\pipe\{}", self.name);
        let pipe_name_wide: Vec<u16> = pipe_name.encode_utf16().chain(std::iter::once(0)).collect();
        
        let handle = unsafe {
            CreateFileW(
            )
        
        if handle == INVALID_HANDLE_VALUE {
            return Err(named_pipe_error(&self.name, "open_read", "Failed to open named pipe"));
        self.handle = Some(handle as std::os::windows::io::RawHandle);
        self.is_connected = true;
        self.register_pipe();
        Ok(())
    #[cfg(windows)]
    fn open_windows_write(&mut self) -> IpcResult<()> {
        self.create_named_pipe_if_needed()?;
        
        let pipe_name = format!(r"\\.\pipe\{}", self.name);
        let pipe_name_wide: Vec<u16> = pipe_name.encode_utf16().chain(std::iter::once(0)).collect();
        
        let handle = unsafe {
            CreateFileW(
            )
        
        if handle == INVALID_HANDLE_VALUE {
            return Err(named_pipe_error(&self.name, "open_write", "Failed to open named pipe"));
        self.handle = Some(handle as std::os::windows::io::RawHandle);
        self.is_connected = true;
        self.register_pipe();
        Ok(())
    #[cfg(windows)]
    fn create_named_pipe_if_needed(&mut self) -> IpcResult<()> {
        if !self.config.create_if_missing {
            return Ok(());
        let pipe_name = format!(r"\\.\pipe\{}", self.name);
        let pipe_name_wide: Vec<u16> = pipe_name.encode_utf16().chain(std::iter::once(0)).collect();
        
        let handle = unsafe {
            CreateNamedPipeW(
            )
        
        if handle == INVALID_HANDLE_VALUE {
            let error = unsafe { GetLastError() };
            if error != ERROR_ALREADY_EXISTS {
                return Err(system_error(error as i32, "CreateNamedPipe", "Failed to create named pipe"));
            }
        } else {
            // We created the pipe, so we're the server
            self.is_server = true;
            self.handle = Some(handle as std::os::windows::io::RawHandle);
        Ok(())
    #[cfg(windows)]
    fn read_windows(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        if let Some(handle) = self.handle {
            let mut bytes_read: u32 = 0;
            let result = unsafe {
                ReadFile(
                )
            
            if result == 0 {
                let error = unsafe { GetLastError() };
                return Err(system_error(error as i32, "ReadFile", "Failed to read from pipe"));
            Ok(bytes_read as usize)
        } else {
            Err(named_pipe_error(&self.name, "read", "Pipe not open"))
        }
    }

    #[cfg(windows)]
    fn write_windows(&mut self, data: &[u8]) -> IpcResult<usize> {
        if let Some(handle) = self.handle {
            let mut bytes_written: u32 = 0;
            let result = unsafe {
                WriteFile(
                )
            
            if result == 0 {
                let error = unsafe { GetLastError() };
                return Err(system_error(error as i32, "WriteFile", "Failed to write to pipe"));
            Ok(bytes_written as usize)
        } else {
            Err(named_pipe_error(&self.name, "write", "Pipe not open"))
        }
    }

    fn register_pipe(&self) {
        let registry = get_pipe_registry();
        if let Ok(mut pipes) = registry.write() {
            let info = Arc::new(NamedPipeInfo {
            });
            pipes.insert(self.name.clone(), info);
        }
    }

    fn unregister_pipe(&self) {
        let registry = get_pipe_registry();
        if let Ok(mut pipes) = registry.write() {
            if let Some(info) = pipes.get(&self.name) {
                let mut ref_count = info.ref_count.lock().unwrap();
                *ref_count -= 1;
                if *ref_count == 0 {
                    pipes.remove(&self.name);
                    
                    // Clean up the FIFO file if we created it
                    #[cfg(unix)]
                    if info.created_by_us && info.path.exists() {
                        let _ = std::fs::remove_file(&info.path);
                    }
                }
            }
        }
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Named pipe server for handling multiple connections
#[derive(Debug)]
pub struct NamedPipeServer {
    #[cfg(unix)]
    #[cfg(windows)]
impl NamedPipeServer {
    /// Create a new named pipe server
    pub fn new(name: &str, config: NamedPipeConfig) -> Self {
        #[cfg(unix)]
        let path = PathBuf::from(format!("/tmp/{}", name));
        
        Self {
            #[cfg(unix)]
            #[cfg(windows)]
        }
    }

    /// Start listening for connections
    pub fn listen(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            self.listen_unix()
        #[cfg(windows)]
        {
            self.listen_windows()
        }
    }

    /// Accept a connection
    pub fn accept(&mut self) -> IpcResult<NamedPipe> {
        #[cfg(unix)]
        {
            self.accept_unix()
        #[cfg(windows)]
        {
            self.accept_windows()
        }
    }

    /// Stop listening
    pub fn stop(&mut self) -> IpcResult<()> {
        self.is_listening = false;
        
        #[cfg(windows)]
        {
            for handle in &self.handles {
                unsafe {
                    CloseHandle(*handle as *mut std::ffi::c_void);
                }
            }
            self.handles.clear();
        Ok(())
    #[cfg(unix)]
    fn listen_unix(&mut self) -> IpcResult<()> {
        // Create FIFO
        if self.path.exists() {
            std::fs::remove_file(&self.path)
                .map_err(|e| named_pipe_error(&self.name, "listen", &e.to_string()))?;
        unsafe {
            let path_cstr = std::ffi::CString::new(self.path.to_string_lossy().as_bytes())
                .map_err(|e| named_pipe_error(&self.name, "listen", &e.to_string()))?;
            
            let result = libc::mkfifo(path_cstr.as_ptr(), self.config.permissions);
            if result != 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "mkfifo", "Failed to create server FIFO"));
            }
        }
        
        self.is_listening = true;
        Ok(())
    #[cfg(unix)]
    fn accept_unix(&mut self) -> IpcResult<NamedPipe> {
        if !self.is_listening {
            return Err(named_pipe_error(&self.name, "accept", "Server not listening"));
        let mut pipe = NamedPipe::with_config(&self.name, &self.path, self.config.clone());
        pipe.open_read()?;
        Ok(pipe)
    #[cfg(windows)]
    fn listen_windows(&mut self) -> IpcResult<()> {
        let pipe_name = format!(r"\\.\pipe\{}", self.name);
        let pipe_name_wide: Vec<u16> = pipe_name.encode_utf16().chain(std::iter::once(0)).collect();
        
        for _ in 0..self.config.max_connections {
            let handle = unsafe {
                CreateNamedPipeW(
                )
            
            if handle == INVALID_HANDLE_VALUE {
                return Err(named_pipe_error(&self.name, "listen", "Failed to create named pipe"));
            self.handles.push(handle as std::os::windows::io::RawHandle);
        self.is_listening = true;
        Ok(())
    #[cfg(windows)]
    fn accept_windows(&mut self) -> IpcResult<NamedPipe> {
        if !self.is_listening || self.handles.is_empty() {
            return Err(named_pipe_error(&self.name, "accept", "Server not listening or no available handles"));
        let handle = self.handles.remove(0);
        
        // Wait for client connection
        let result = unsafe {
            ConnectNamedPipe(handle as *mut std::ffi::c_void, std::ptr::null_mut())
        
        if result == 0 {
            let error = unsafe { GetLastError() };
            if error != ERROR_PIPE_CONNECTED {
                return Err(system_error(error as i32, "ConnectNamedPipe", "Failed to connect to client"));
            }
        }
        
        let mut pipe = NamedPipe::with_config(&self.name, "", self.config.clone());
        pipe.handle = Some(handle);
        pipe.is_connected = true;
        Ok(pipe)
    }
}

/// Named pipe client for connecting to servers
#[derive(Debug)]
pub struct NamedPipeClient {
impl NamedPipeClient {
    /// Create a new named pipe client
    pub fn new(name: &str, config: NamedPipeConfig) -> Self {
        Self {
        }
    }

    /// Connect to a named pipe server
    pub fn connect(&self) -> IpcResult<NamedPipe> {
        #[cfg(unix)]
        {
            let path = PathBuf::from(format!("/tmp/{}", self.name));
            let mut pipe = NamedPipe::with_config(&self.name, &path, self.config.clone());
            pipe.open_write()?;
            Ok(pipe)
        #[cfg(windows)]
        {
            let mut pipe = NamedPipe::with_config(&self.name, "", self.config.clone());
            pipe.open_windows_write()?;
            Ok(pipe)
        }
    }

    /// Connect with timeout
    pub fn connect_timeout(&self, timeout: Duration) -> IpcResult<NamedPipe> {
        let start = Instant::now();
        
        loop {
            match self.connect() {
                Err(_) if start.elapsed() < timeout => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
            }
        }
    }
}

/// Cleanup all registered pipes
pub fn cleanup_pipes() -> IpcResult<()> {
    let registry = get_pipe_registry();
    if let Ok(mut pipes) = registry.write() {
        for (name, info) in pipes.drain() {
            #[cfg(unix)]
            if info.created_by_us && info.path.exists() {
                let _ = std::fs::remove_file(&info.path);
                tracing::debug!(pipe_name = name, "Cleaned up pipe file");
            }
        }
    }
    Ok(())
// Windows API definitions
#[cfg(windows)]
extern "system" {
    fn CreateNamedPipeW(
    ) -> *mut std::ffi::c_void;
    
    fn ConnectNamedPipe(
    ) -> i32;
    
    fn CreateFileW(
    ) -> *mut std::ffi::c_void;
    
    fn ReadFile(
    ) -> i32;
    
    fn WriteFile(
    ) -> i32;
    
    fn CloseHandle(handle: *mut std::ffi::c_void) -> i32;
    fn GetLastError() -> u32;
#[cfg(windows)]
const GENERIC_READ: u32 = 0x80000000;
#[cfg(windows)]
const GENERIC_WRITE: u32 = 0x40000000;
#[cfg(windows)]
const OPEN_EXISTING: u32 = 3;
#[cfg(windows)]
const PIPE_ACCESS_DUPLEX: u32 = 0x00000003;
#[cfg(windows)]
const PIPE_TYPE_BYTE: u32 = 0x00000000;
#[cfg(windows)]
const PIPE_READMODE_BYTE: u32 = 0x00000000;
#[cfg(windows)]
const PIPE_WAIT: u32 = 0x00000000;
#[cfg(windows)]
const INVALID_HANDLE_VALUE: *mut std::ffi::c_void = (-1isize) as *mut std::ffi::c_void;
#[cfg(windows)]
const ERROR_ALREADY_EXISTS: u32 = 183;
#[cfg(windows)]
const ERROR_PIPE_CONNECTED: u32 = 535;

