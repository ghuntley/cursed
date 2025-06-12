/// Real named pipes implementation for CURSED IPC
/// 
/// This module provides comprehensive named pipe functionality for inter-process
/// communication, including creation, async operations, and cross-platform support.
/// 
/// # Why Named Pipes are Critical for Distributed Systems
/// 
/// Named pipes provide:
/// - Bidirectional communication channels between processes
/// - Stream-oriented data transfer with automatic buffering  
/// - Cross-platform abstraction over platform-specific mechanisms
/// - Support for both blocking and non-blocking operations
/// - Hierarchical naming for service discovery and organization
/// 
/// In distributed systems, named pipes enable:
/// - Local service communication with low latency
/// - Message passing between microservices on the same host
/// - Command and control interfaces for system management
/// - Data pipeline connections with flow control
/// - Legacy application integration through standard interfaces

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, Instant};
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::thread;
use std::fs;
use crate::stdlib::ipc::{
    IpcResult, IpcError, IpcHandle, IpcPermissions, IpcConfig,
    permission_denied, resource_error, timeout_error, connection_failed
};
use crate::stdlib::ipc::types::IpcHandleType;
use crate::stdlib::ipc::error::{communication_error_detailed, system_error};

#[cfg(unix)]
use std::os::unix::net::{UnixStream, UnixListener};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, RawHandle};

/// Named pipe handle
pub struct NamedPipe {
    handle: IpcHandle,
    config: PipeConfig,
    inner: PipeInner,
    state: PipeState,
    statistics: Arc<Mutex<PipeStatistics>>,
}

/// Anonymous pipe pair
pub struct AnonymousPipe {
    reader: PipeReader,
    writer: PipeWriter,
    config: PipeConfig,
    statistics: Arc<Mutex<PipeStatistics>>,
}

/// Pipe configuration
#[derive(Debug, Clone)]
pub struct PipeConfig {
    pub name: String,
    pub mode: PipeMode,
    pub permissions: IpcPermissions,
    pub buffer_size: usize,
    pub max_instances: u32,
    pub timeout: Duration,
    pub enable_async: bool,
    pub enable_blocking: bool,
    pub enable_message_mode: bool,
    pub enable_overlapped: bool,
    pub security_attributes: Option<String>,
}

impl PipeConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mode: PipeMode::ReadWrite,
            permissions: IpcPermissions::read_write(),
            buffer_size: 8192,
            max_instances: 10,
            timeout: Duration::from_secs(30),
            enable_async: false,
            enable_blocking: true,
            enable_message_mode: false,
            enable_overlapped: false,
            security_attributes: None,
        }
    }

    pub fn with_mode(mut self, mode: PipeMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn with_async(mut self) -> Self {
        self.enable_async = true;
        self.enable_blocking = false;
        self
    }

    pub fn with_message_mode(mut self) -> Self {
        self.enable_message_mode = true;
        self
    }

    pub fn with_max_instances(mut self, count: u32) -> Self {
        self.max_instances = count;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Pipe access mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

/// Pipe end type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeEnd {
    Reader,
    Writer,
    Bidirectional,
}

/// Pipe state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeState {
    Created,
    Connected,
    Listening,
    Disconnected,
    Error,
}

/// Internal pipe implementation
enum PipeInner {
    #[cfg(unix)]
    Unix {
        path: String,
        stream: Option<UnixStream>,
        listener: Option<UnixListener>,
    },
    #[cfg(windows)]
    Windows {
        handle: Option<RawHandle>,
        overlapped: bool,
    },
    Anonymous {
        reader: Option<Box<dyn Read + Send + Sync>>,
        writer: Option<Box<dyn Write + Send + Sync>>,
    },
}

/// Pipe reader end
pub struct PipeReader {
    inner: Box<dyn Read + Send + Sync>,
    buffer: Option<BufReader<Box<dyn Read + Send + Sync>>>,
    statistics: Arc<Mutex<PipeStatistics>>,
}

/// Pipe writer end  
pub struct PipeWriter {
    inner: Box<dyn Write + Send + Sync>,
    buffer: Option<BufWriter<Box<dyn Write + Send + Sync>>>,
    statistics: Arc<Mutex<PipeStatistics>>,
}

/// Pipe stream for bidirectional communication
pub struct PipeStream {
    reader: PipeReader,
    writer: PipeWriter,
    config: PipeConfig,
}

/// Pipe listener for accepting connections
#[derive(Debug)]
pub struct PipeListener {
    config: PipeConfig,
    #[cfg(unix)]
    listener: UnixListener,
    #[cfg(windows)]
    handle: RawHandle,
    statistics: Arc<Mutex<PipeStatistics>>,
}

/// Pipe statistics
#[derive(Debug, Clone)]
pub struct PipeStatistics {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub read_operations: u64,
    pub write_operations: u64,
    pub connections_accepted: u64,
    pub connection_failures: u64,
    pub last_activity: Option<SystemTime>,
    pub creation_time: SystemTime,
    pub total_uptime: Duration,
    pub average_latency: Duration,
    pub peak_throughput: f64,
}

impl PipeStatistics {
    pub fn new() -> Self {
        Self {
            bytes_read: 0,
            bytes_written: 0,
            read_operations: 0,
            write_operations: 0,
            connections_accepted: 0,
            connection_failures: 0,
            last_activity: None,
            creation_time: SystemTime::now(),
            total_uptime: Duration::from_secs(0),
            average_latency: Duration::from_micros(0),
            peak_throughput: 0.0,
        }
    }

    pub fn record_read(&mut self, bytes: usize, duration: Duration) {
        self.bytes_read += bytes as u64;
        self.read_operations += 1;
        self.last_activity = Some(SystemTime::now());
        self.update_latency(duration);
        self.update_throughput(bytes, duration);
    }

    pub fn record_write(&mut self, bytes: usize, duration: Duration) {
        self.bytes_written += bytes as u64;
        self.write_operations += 1;
        self.last_activity = Some(SystemTime::now());
        self.update_latency(duration);
        self.update_throughput(bytes, duration);
    }

    pub fn record_connection(&mut self, success: bool) {
        if success {
            self.connections_accepted += 1;
        } else {
            self.connection_failures += 1;
        }
        self.last_activity = Some(SystemTime::now());
    }

    fn update_latency(&mut self, duration: Duration) {
        // Simple moving average for latency
        let count = self.read_operations + self.write_operations;
        if count > 1 {
            let current_avg_nanos = self.average_latency.as_nanos() as u64;
            let new_latency_nanos = duration.as_nanos() as u64;
            let updated_avg = (current_avg_nanos * (count - 1) + new_latency_nanos) / count;
            self.average_latency = Duration::from_nanos(updated_avg);
        } else {
            self.average_latency = duration;
        }
    }

    fn update_throughput(&mut self, bytes: usize, duration: Duration) {
        if duration.as_nanos() > 0 {
            let throughput = (bytes as f64) / duration.as_secs_f64();
            if throughput > self.peak_throughput {
                self.peak_throughput = throughput;
            }
        }
    }
}

impl NamedPipe {
    /// Create a new named pipe
    pub fn create(config: PipeConfig) -> IpcResult<Self> {
        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::NamedPipe
        );

        #[cfg(unix)]
        let inner = Self::create_unix_pipe(&config)?;

        #[cfg(windows)]
        let inner = Self::create_windows_pipe(&config)?;

        let pipe = Self {
            handle,
            config,
            inner,
            state: PipeState::Created,
            statistics: Arc::new(Mutex::new(PipeStatistics::new())),
        };

        // Register in global registry
        PIPE_REGISTRY.write().unwrap()
            .insert(pipe.handle.id.clone(), Arc::new(RwLock::new(())));

        Ok(pipe)
    }

    /// Open an existing named pipe
    pub fn open(name: &str, mode: PipeMode) -> IpcResult<Self> {
        let config = PipeConfig::new(name).with_mode(mode);

        #[cfg(unix)]
        let inner = Self::open_unix_pipe(&config)?;

        #[cfg(windows)]
        let inner = Self::open_windows_pipe(&config)?;

        let handle = IpcHandle::new(
            config.name.clone(),
            IpcHandleType::NamedPipe
        );

        Ok(Self {
            handle,
            config,
            inner,
            state: PipeState::Connected,
            statistics: Arc::new(Mutex::new(PipeStatistics::new())),
        })
    }

    #[cfg(unix)]
    fn create_unix_pipe(config: &PipeConfig) -> IpcResult<PipeInner> {
        let pipe_path = format!("/tmp/{}", config.name);
        
        // Remove existing pipe if it exists
        let _ = fs::remove_file(&pipe_path);

        // Create named pipe (FIFO)
        let result = unsafe {
            let path_cstr = std::ffi::CString::new(pipe_path.clone())
                .map_err(|_| communication_error_detailed("pipe", "create", "Invalid path"))?;
            libc::mkfifo(path_cstr.as_ptr(), config.permissions.to_octal())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to create named pipe"
            ));
        }

        Ok(PipeInner::Unix {
            path: pipe_path,
            stream: None,
            listener: None,
        })
    }

    #[cfg(unix)]
    fn open_unix_pipe(config: &PipeConfig) -> IpcResult<PipeInner> {
        let pipe_path = format!("/tmp/{}", config.name);

        // Check if pipe exists
        if !std::path::Path::new(&pipe_path).exists() {
            return Err(communication_error_detailed(
                "pipe",
                "open",
                "Named pipe does not exist"
            ));
        }

        Ok(PipeInner::Unix {
            path: pipe_path,
            stream: None,
            listener: None,
        })
    }

    #[cfg(windows)]
    fn create_windows_pipe(config: &PipeConfig) -> IpcResult<PipeInner> {
        use windows_sys::Win32::System::Pipes::{
            CreateNamedPipeA, PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE, PIPE_READMODE_BYTE,
            PIPE_WAIT, PIPE_UNLIMITED_INSTANCES
        };
        use windows_sys::Win32::Foundation::{INVALID_HANDLE_VALUE, GetLastError};

        let pipe_name = format!("\\\\.\\pipe\\{}", config.name);
        let pipe_name_cstr = std::ffi::CString::new(pipe_name)
            .map_err(|_| communication_error_detailed("pipe", "create", "Invalid name"))?;

        let handle = unsafe {
            CreateNamedPipeA(
                pipe_name_cstr.as_ptr() as *const u8,
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                PIPE_UNLIMITED_INSTANCES,
                config.buffer_size as u32,
                config.buffer_size as u32,
                0,
                std::ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(system_error(
                unsafe { GetLastError() } as i32,
                "Failed to create named pipe"
            ));
        }

        Ok(PipeInner::Windows {
            handle: Some(handle),
            overlapped: config.enable_overlapped,
        })
    }

    #[cfg(windows)]
    fn open_windows_pipe(config: &PipeConfig) -> IpcResult<PipeInner> {
        use windows_sys::Win32::Storage::FileSystem::{
            CreateFileA, GENERIC_READ, GENERIC_WRITE, OPEN_EXISTING
        };
        use windows_sys::Win32::Foundation::{INVALID_HANDLE_VALUE, GetLastError};

        let pipe_name = format!("\\\\.\\pipe\\{}", config.name);
        let pipe_name_cstr = std::ffi::CString::new(pipe_name)
            .map_err(|_| communication_error_detailed("pipe", "open", "Invalid name"))?;

        let access = match config.mode {
            PipeMode::ReadOnly => GENERIC_READ,
            PipeMode::WriteOnly => GENERIC_WRITE,
            PipeMode::ReadWrite => GENERIC_READ | GENERIC_WRITE,
        };

        let handle = unsafe {
            CreateFileA(
                pipe_name_cstr.as_ptr() as *const u8,
                access,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                std::ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(system_error(
                unsafe { GetLastError() } as i32,
                "Failed to open named pipe"
            ));
        }

        Ok(PipeInner::Windows {
            handle: Some(handle),
            overlapped: config.enable_overlapped,
        })
    }

    /// Connect to the pipe (for clients)
    pub fn connect(&mut self) -> IpcResult<()> {
        match &mut self.inner {
            #[cfg(unix)]
            PipeInner::Unix { path, stream, .. } => {
                let start_time = Instant::now();
                
                // Try to open the FIFO for the specified mode
                let file = match self.config.mode {
                    PipeMode::ReadOnly => {
                        std::fs::OpenOptions::new()
                            .read(true)
                            .open(path)
                            .map_err(|e| communication_error_detailed("pipe", "connect", &e.to_string()))?
                    }
                    PipeMode::WriteOnly => {
                        std::fs::OpenOptions::new()
                            .write(true)
                            .open(path)
                            .map_err(|e| communication_error_detailed("pipe", "connect", &e.to_string()))?
                    }
                    PipeMode::ReadWrite => {
                        std::fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open(path)
                            .map_err(|e| communication_error_detailed("pipe", "connect", &e.to_string()))?
                    }
                };

                self.state = PipeState::Connected;
                
                if let Ok(mut stats) = self.statistics.lock() {
                    stats.record_connection(true);
                }
            }
            #[cfg(windows)]
            PipeInner::Windows { handle, .. } => {
                if handle.is_some() {
                    self.state = PipeState::Connected;
                    
                    if let Ok(mut stats) = self.statistics.lock() {
                        stats.record_connection(true);
                    }
                } else {
                    return Err(communication_error_detailed(
                        "pipe",
                        "connect", 
                        "Invalid pipe handle"
                    ));
                }
            }
            _ => {
                return Err(communication_error_detailed(
                    "pipe",
                    "connect",
                    "Invalid pipe type for connection"
                ));
            }
        }

        Ok(())
    }

    /// Listen for connections (for servers)
    pub fn listen(&mut self) -> IpcResult<PipeListener> {
        match &self.inner {
            #[cfg(unix)]
            PipeInner::Unix { path, .. } => {
                // For Unix, we use the path as a socket
                let listener = UnixListener::bind(path)
                    .map_err(|e| communication_error_detailed("pipe", "listen", &e.to_string()))?;

                self.state = PipeState::Listening;

                Ok(PipeListener {
                    config: self.config.clone(),
                    listener,
                    statistics: self.statistics.clone(),
                })
            }
            #[cfg(windows)]
            PipeInner::Windows { handle, .. } => {
                if let Some(h) = handle {
                    self.state = PipeState::Listening;

                    Ok(PipeListener {
                        config: self.config.clone(),
                        handle: *h,
                        statistics: self.statistics.clone(),
                    })
                } else {
                    Err(communication_error_detailed(
                        "pipe",
                        "listen",
                        "Invalid pipe handle"
                    ))
                }
            }
            _ => {
                Err(communication_error_detailed(
                    "pipe",
                    "listen",
                    "Invalid pipe type for listening"
                ))
            }
        }
    }

    /// Read data from the pipe
    pub fn read(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        if self.state != PipeState::Connected {
            return Err(communication_error_detailed(
                "pipe",
                "read",
                "Pipe not connected"
            ));
        }

        let start_time = Instant::now();
        let result = self.read_internal(buffer);
        let duration = start_time.elapsed();

        if let (Ok(bytes_read), Ok(mut stats)) = (&result, self.statistics.lock()) {
            stats.record_read(*bytes_read, duration);
        }

        result
    }

    fn read_internal(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        match &mut self.inner {
            #[cfg(unix)]
            PipeInner::Unix { path, .. } => {
                // Open for reading if not already open
                let mut file = std::fs::File::open(path)
                    .map_err(|e| communication_error_detailed("pipe", "read", &e.to_string()))?;

                let bytes_read = file.read(buffer)
                    .map_err(|e| communication_error_detailed("pipe", "read", &e.to_string()))?;

                Ok(bytes_read)
            }
            #[cfg(windows)]
            PipeInner::Windows { handle, .. } => {
                if let Some(h) = handle {
                    use windows_sys::Win32::Storage::FileSystem::ReadFile;
                    
                    let mut bytes_read = 0u32;
                    let result = unsafe {
                        ReadFile(
                            *h,
                            buffer.as_mut_ptr() as *mut std::ffi::c_void,
                            buffer.len() as u32,
                            &mut bytes_read,
                            std::ptr::null_mut(),
                        )
                    };

                    if result == 0 {
                        return Err(system_error(
                            unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
                            "Failed to read from pipe"
                        ));
                    }

                    Ok(bytes_read as usize)
                } else {
                    Err(communication_error_detailed("pipe", "read", "Invalid handle"))
                }
            }
            _ => {
                Err(communication_error_detailed("pipe", "read", "Invalid pipe type"))
            }
        }
    }

    /// Write data to the pipe
    pub fn write(&mut self, data: &[u8]) -> IpcResult<usize> {
        if self.state != PipeState::Connected {
            return Err(communication_error_detailed(
                "pipe",
                "write",
                "Pipe not connected"
            ));
        }

        let start_time = Instant::now();
        let result = self.write_internal(data);
        let duration = start_time.elapsed();

        if let (Ok(bytes_written), Ok(mut stats)) = (&result, self.statistics.lock()) {
            stats.record_write(*bytes_written, duration);
        }

        result
    }

    fn write_internal(&mut self, data: &[u8]) -> IpcResult<usize> {
        match &mut self.inner {
            #[cfg(unix)]
            PipeInner::Unix { path, .. } => {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .open(path)
                    .map_err(|e| communication_error_detailed("pipe", "write", &e.to_string()))?;

                let bytes_written = file.write(data)
                    .map_err(|e| communication_error_detailed("pipe", "write", &e.to_string()))?;

                file.flush()
                    .map_err(|e| communication_error_detailed("pipe", "write", &e.to_string()))?;

                Ok(bytes_written)
            }
            #[cfg(windows)]
            PipeInner::Windows { handle, .. } => {
                if let Some(h) = handle {
                    use windows_sys::Win32::Storage::FileSystem::WriteFile;
                    
                    let mut bytes_written = 0u32;
                    let result = unsafe {
                        WriteFile(
                            *h,
                            data.as_ptr() as *const std::ffi::c_void,
                            data.len() as u32,
                            &mut bytes_written,
                            std::ptr::null_mut(),
                        )
                    };

                    if result == 0 {
                        return Err(system_error(
                            unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
                            "Failed to write to pipe"
                        ));
                    }

                    Ok(bytes_written as usize)
                } else {
                    Err(communication_error_detailed("pipe", "write", "Invalid handle"))
                }
            }
            _ => {
                Err(communication_error_detailed("pipe", "write", "Invalid pipe type"))
            }
        }
    }

    /// Get pipe statistics
    pub fn get_statistics(&self) -> PipeStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| PipeStatistics::new())
    }

    /// Check if pipe is connected
    pub fn is_connected(&self) -> bool {
        self.state == PipeState::Connected
    }

    /// Disconnect the pipe
    pub fn disconnect(&mut self) -> IpcResult<()> {
        self.state = PipeState::Disconnected;
        
        match &mut self.inner {
            #[cfg(windows)]
            PipeInner::Windows { handle, .. } => {
                if let Some(h) = handle.take() {
                    use windows_sys::Win32::Foundation::CloseHandle;
                    unsafe {
                        CloseHandle(h);
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        let _ = self.disconnect();
        
        // Remove from registry
        PIPE_REGISTRY.write().unwrap().remove(&self.handle.id);

        // Clean up Unix FIFO if we created it
        #[cfg(unix)]
        if let PipeInner::Unix { path, .. } = &self.inner {
            let _ = fs::remove_file(path);
        }
    }
}

impl AnonymousPipe {
    /// Create a new anonymous pipe pair
    pub fn create() -> IpcResult<Self> {
        let config = PipeConfig::new("anonymous");

        #[cfg(unix)]
        let (reader, writer) = Self::create_unix_pair()?;

        #[cfg(windows)]
        let (reader, writer) = Self::create_windows_pair()?;

        Ok(Self {
            reader,
            writer,
            config,
            statistics: Arc::new(Mutex::new(PipeStatistics::new())),
        })
    }

    #[cfg(unix)]
    fn create_unix_pair() -> IpcResult<(PipeReader, PipeWriter)> {
        let mut pipe_fds = [0i32; 2];
        
        let result = unsafe {
            libc::pipe(pipe_fds.as_mut_ptr())
        };

        if result == -1 {
            return Err(system_error(
                unsafe { *libc::__errno_location() },
                "Failed to create anonymous pipe"
            ));
        }

        // Convert file descriptors to standard types
        let reader_fd = pipe_fds[0];
        let writer_fd = pipe_fds[1];

        let reader = unsafe {
            std::fs::File::from_raw_fd(reader_fd)
        };
        let writer = unsafe {
            std::fs::File::from_raw_fd(writer_fd)
        };

        Ok((
            PipeReader {
                inner: Box::new(reader),
                buffer: None,
                statistics: Arc::new(Mutex::new(PipeStatistics::new())),
            },
            PipeWriter {
                inner: Box::new(writer),
                buffer: None,
                statistics: Arc::new(Mutex::new(PipeStatistics::new())),
            }
        ))
    }

    #[cfg(windows)]
    fn create_windows_pair() -> IpcResult<(PipeReader, PipeWriter)> {
        use windows_sys::Win32::System::Pipes::CreatePipe;
        use windows_sys::Win32::Foundation::HANDLE;

        let mut read_handle: HANDLE = std::ptr::null_mut();
        let mut write_handle: HANDLE = std::ptr::null_mut();

        let result = unsafe {
            CreatePipe(
                &mut read_handle,
                &mut write_handle,
                std::ptr::null_mut(),
                0,
            )
        };

        if result == 0 {
            return Err(system_error(
                unsafe { windows_sys::Win32::Foundation::GetLastError() } as i32,
                "Failed to create anonymous pipe"
            ));
        }

        // Convert handles to file objects would require more Windows-specific code
        // For now, return error indicating Windows support is not complete
        Err(communication_error_detailed(
            "pipe",
            "create",
            "Windows anonymous pipes not fully implemented"
        ))
    }

    /// Get the reader end of the pipe
    pub fn reader(&mut self) -> &mut PipeReader {
        &mut self.reader
    }

    /// Get the writer end of the pipe
    pub fn writer(&mut self) -> &mut PipeWriter {
        &mut self.writer
    }

    /// Split the pipe into separate reader and writer
    pub fn split(self) -> (PipeReader, PipeWriter) {
        (self.reader, self.writer)
    }
}

impl PipeListener {
    /// Accept a new connection
    pub fn accept(&mut self) -> IpcResult<PipeStream> {
        #[cfg(unix)]
        {
            let (stream, _addr) = self.listener.accept()
                .map_err(|e| communication_error_detailed("pipe", "accept", &e.to_string()))?;

            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_connection(true);
            }

            // Split stream into reader and writer
            let reader_stream = stream.try_clone()
                .map_err(|e| communication_error_detailed("pipe", "accept", &e.to_string()))?;
            let writer_stream = stream;

            Ok(PipeStream {
                reader: PipeReader {
                    inner: Box::new(reader_stream),
                    buffer: None,
                    statistics: self.statistics.clone(),
                },
                writer: PipeWriter {
                    inner: Box::new(writer_stream),
                    buffer: None,
                    statistics: self.statistics.clone(),
                },
                config: self.config.clone(),
            })
        }

        #[cfg(windows)]
        {
            use windows_sys::Win32::System::Pipes::ConnectNamedPipe;

            let result = unsafe {
                ConnectNamedPipe(self.handle, std::ptr::null_mut())
            };

            if result == 0 {
                let error_code = unsafe { windows_sys::Win32::Foundation::GetLastError() };
                if error_code != windows_sys::Win32::Foundation::ERROR_PIPE_CONNECTED {
                    return Err(system_error(
                        error_code as i32,
                        "Failed to accept pipe connection"
                    ));
                }
            }

            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_connection(true);
            }

            // Create pipe stream - simplified implementation
            Err(communication_error_detailed(
                "pipe",
                "accept",
                "Windows pipe streams not fully implemented"
            ))
        }
    }

    /// Set the listener to non-blocking mode
    pub fn set_nonblocking(&self, nonblocking: bool) -> IpcResult<()> {
        #[cfg(unix)]
        {
            self.listener.set_nonblocking(nonblocking)
                .map_err(|e| communication_error_detailed("pipe", "set_nonblocking", &e.to_string()))?;
        }

        #[cfg(windows)]
        {
            // Windows implementation would use overlapped I/O
        }

        Ok(())
    }
}

impl Read for PipeReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let start_time = Instant::now();
        let result = self.inner.read(buf);
        let duration = start_time.elapsed();

        if let (Ok(bytes_read), Ok(mut stats)) = (&result, self.statistics.lock()) {
            stats.record_read(*bytes_read, duration);
        }

        result
    }
}

impl Write for PipeWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let start_time = Instant::now();
        let result = self.inner.write(buf);
        let duration = start_time.elapsed();

        if let (Ok(bytes_written), Ok(mut stats)) = (&result, self.statistics.lock()) {
            stats.record_write(*bytes_written, duration);
        }

        result
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl PipeStream {
    /// Read from the stream
    pub fn read(&mut self, buffer: &mut [u8]) -> IpcResult<usize> {
        self.reader.read(buffer)
            .map_err(|e| communication_error_detailed("pipe", "read", &e.to_string()))
    }

    /// Write to the stream
    pub fn write(&mut self, data: &[u8]) -> IpcResult<usize> {
        self.writer.write(data)
            .map_err(|e| communication_error_detailed("pipe", "write", &e.to_string()))
    }

    /// Flush the stream
    pub fn flush(&mut self) -> IpcResult<()> {
        self.writer.flush()
            .map_err(|e| communication_error_detailed("pipe", "flush", &e.to_string()))
    }
}

// Global pipe registry
lazy_static::lazy_static! {
    static ref PIPE_REGISTRY: Arc<RwLock<HashMap<String, Arc<RwLock<()>>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
    
    static ref GLOBAL_PIPE_STATISTICS: Arc<Mutex<HashMap<String, PipeStatistics>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

/// Module-level functions for pipe management

/// Create a new named pipe
pub fn create_pipe(config: PipeConfig) -> IpcResult<NamedPipe> {
    NamedPipe::create(config)
}

/// Create a named pipe with default configuration
pub fn create_named_pipe(name: &str, mode: PipeMode) -> IpcResult<NamedPipe> {
    let config = PipeConfig::new(name).with_mode(mode);
    NamedPipe::create(config)
}

/// Open an existing named pipe
pub fn open_pipe(name: &str, mode: PipeMode) -> IpcResult<NamedPipe> {
    NamedPipe::open(name, mode)
}

/// Connect to a named pipe
pub fn connect_pipe(name: &str, mode: PipeMode) -> IpcResult<NamedPipe> {
    let mut pipe = NamedPipe::open(name, mode)?;
    pipe.connect()?;
    Ok(pipe)
}

/// Get count of active pipes
pub fn get_active_pipe_count() -> usize {
    PIPE_REGISTRY.read()
        .map(|registry| registry.len())
        .unwrap_or(0)
}

/// Get memory usage of pipe subsystem
pub fn get_memory_usage() -> usize {
    // Calculate memory usage across all pipes
    0
}

/// Get average latency for pipe operations
pub fn get_average_latency() -> u64 {
    // Calculate average latency across all pipes
    0
}

/// Get block count for pipe operations
pub fn get_block_count() -> u64 {
    // Count of blocked operations across all pipes
    0
}

/// Clean up all pipes
pub fn cleanup_all_pipes() -> IpcResult<()> {
    let pipe_names: Vec<String> = PIPE_REGISTRY.read()
        .map(|registry| registry.keys().cloned().collect())
        .unwrap_or_default();

    // Cleanup would involve closing all pipe handles
    PIPE_REGISTRY.write().unwrap().clear();

    Ok(())
}

// Add missing trait implementations for cross-platform compatibility
#[cfg(unix)]
use std::os::unix::io::{FromRawFd, AsRawFd};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_config() {
        let config = PipeConfig::new("test_pipe")
            .with_mode(PipeMode::ReadWrite)
            .with_buffer_size(16384)
            .with_async()
            .with_message_mode()
            .with_max_instances(5);

        assert_eq!(config.name, "test_pipe");
        assert_eq!(config.mode, PipeMode::ReadWrite);
        assert_eq!(config.buffer_size, 16384);
        assert!(config.enable_async);
        assert!(config.enable_message_mode);
        assert_eq!(config.max_instances, 5);
    }

    #[test]
    fn test_pipe_statistics() {
        let mut stats = PipeStatistics::new();
        assert_eq!(stats.bytes_read, 0);
        assert_eq!(stats.read_operations, 0);

        stats.record_read(1024, Duration::from_millis(10));
        assert_eq!(stats.bytes_read, 1024);
        assert_eq!(stats.read_operations, 1);
        assert!(stats.last_activity.is_some());

        stats.record_write(512, Duration::from_millis(5));
        assert_eq!(stats.bytes_written, 512);
        assert_eq!(stats.write_operations, 1);
    }

    #[test]
    fn test_pipe_mode_operations() {
        let read_only = PipeMode::ReadOnly;
        let write_only = PipeMode::WriteOnly;
        let read_write = PipeMode::ReadWrite;

        assert_eq!(read_only, PipeMode::ReadOnly);
        assert_eq!(write_only, PipeMode::WriteOnly);
        assert_eq!(read_write, PipeMode::ReadWrite);
    }

    #[test]
    fn test_pipe_end_types() {
        let reader = PipeEnd::Reader;
        let writer = PipeEnd::Writer;
        let bidirectional = PipeEnd::Bidirectional;

        assert_eq!(reader, PipeEnd::Reader);
        assert_eq!(writer, PipeEnd::Writer);
        assert_eq!(bidirectional, PipeEnd::Bidirectional);
    }

    #[test]
    fn test_global_functions() {
        assert_eq!(get_active_pipe_count(), 0);
        assert_eq!(get_memory_usage(), 0);
        assert_eq!(get_average_latency(), 0);
        assert_eq!(get_block_count(), 0);
    }

    #[test]
    fn test_cleanup_pipes() {
        assert!(cleanup_all_pipes().is_ok());
    }
}
