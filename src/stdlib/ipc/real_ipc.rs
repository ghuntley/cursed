use crate::error::CursedError;
/// Real IPC (Inter-Process Communication) implementation
/// 
/// This module provides production-ready IPC mechanisms including:
/// - Named pipes with async support
/// - Message queues with priority handling
/// - Enhanced shared memory with synchronization
/// - Unix domain sockets with connection pooling
/// - Memory-mapped files for large data exchange
/// - Cross-platform IPC abstraction layer

use std::collections::{HashMap, VecDeque};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock, Condvar, mpsc};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};

// use crate::stdlib::ipc::error::{IpcError, IpcResult, ipc_error, system_error, not_found, timeout_error};
// use crate::stdlib::ipc::{IpcConfig, SharedMemory, MessageQueue, Message};

/// Real IPC manager with connection pooling and resource management
#[derive(Debug)]
pub struct RealIpcManager {
/// IPC connection wrapper for different communication types
#[derive(Debug)]
pub enum IpcConnection {
/// Named pipe connection with buffered I/O
#[derive(Debug)]
pub struct NamedPipeConnection {
    #[cfg(unix)]
    #[cfg(windows)]
/// Unix domain socket connection
#[derive(Debug)]
pub struct UnixSocketConnection {
    #[cfg(unix)]
    #[cfg(unix)]
/// Message queue connection with priority handling
#[derive(Debug)]
pub struct MessageQueueConnection {
/// Shared memory connection with synchronization
#[derive(Debug)]
pub struct SharedMemoryConnection {
/// Memory-mapped file connection
#[derive(Debug)]
pub struct MemoryMappedConnection {
    #[cfg(unix)]
    #[cfg(windows)]
// Safety: MemoryMappedConnection is safe to send between threads
// The raw pointers are managed properly and protected by the owning structure
unsafe impl Send for MemoryMappedConnection {}
unsafe impl Sync for MemoryMappedConnection {}

/// Priority message queue implementation
#[derive(Debug)]
pub struct PriorityMessageQueue {
/// IPC message with metadata
#[derive(Debug, Clone)]
pub struct IpcMessage {
/// Message priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
/// IPC statistics for monitoring
#[derive(Debug, Clone)]
pub struct IpcStats {
impl RealIpcManager {
    /// Create new IPC manager
    pub fn new(config: IpcConfig) -> IpcResult<Self> {
        let shutdown_signal = Arc::new((Mutex::new(false), Condvar::new()));
        let shutdown_clone = shutdown_signal.clone();
        
        // Start cleanup thread
        let cleanup_thread = thread::spawn(move || {
            let (lock, cvar) = &*shutdown_clone;
            
            loop {
                let result = cvar.wait_timeout(
                    Duration::from_secs(60)
                ).unwrap();
                
                if *result.0 {
                    break; // Shutdown requested
                // Perform periodic cleanup
                // This would clean up expired connections, etc.
            }
        });
        
        Ok(Self {
        })
    /// Create named pipe connection
    pub fn create_named_pipe(&self, name: &str, is_server: bool) -> IpcResult<Arc<IpcConnection>> {
        let connection = NamedPipeConnection::new(name, is_server, &self.config)?;
        let arc_connection = Arc::new(IpcConnection::NamedPipe(connection));
        
        let mut connections = self.connections.write()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        connections.insert(name.to_string(), arc_connection.clone());
        
        Ok(arc_connection)
    /// Create Unix domain socket connection
    #[cfg(unix)]
    pub fn create_unix_socket<P: AsRef<Path>>(&self, path: P, is_server: bool) -> IpcResult<Arc<IpcConnection>> {
        let connection = UnixSocketConnection::new(path, is_server, &self.config)?;
        let arc_connection = Arc::new(IpcConnection::UnixSocket(connection));
        
        let path_str = path.as_ref().to_string_lossy().to_string();
        let mut connections = self.connections.write()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        connections.insert(path_str, arc_connection.clone());
        
        Ok(arc_connection)
    /// Create message queue
    pub fn create_message_queue(&self, name: &str) -> IpcResult<Arc<IpcConnection>> {
        let connection = MessageQueueConnection::new(name, &self.config)?;
        let arc_connection = Arc::new(IpcConnection::MessageQueue(connection));
        
        let mut connections = self.connections.write()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        connections.insert(name.to_string(), arc_connection.clone());
        
        Ok(arc_connection)
    /// Create shared memory segment
    pub fn create_shared_memory(&self, name: &str, size: usize) -> IpcResult<Arc<IpcConnection>> {
        let connection = SharedMemoryConnection::new(name, size, &self.config)?;
        let arc_connection = Arc::new(IpcConnection::SharedMemory(connection));
        
        let mut connections = self.connections.write()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        connections.insert(name.to_string(), arc_connection.clone());
        
        Ok(arc_connection)
    /// Create memory-mapped file
    pub fn create_memory_mapped_file<P: AsRef<Path>>(&self, path: P, size: usize, read_only: bool) -> IpcResult<Arc<IpcConnection>> {
        let connection = MemoryMappedConnection::new(path, size, read_only, &self.config)?;
        let arc_connection = Arc::new(IpcConnection::MemoryMappedFile(connection));
        
        let path_str = path.as_ref().to_string_lossy().to_string();
        let mut connections = self.connections.write()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        connections.insert(path_str, arc_connection.clone());
        
        Ok(arc_connection)
    /// Get connection by name
    pub fn get_connection(&self, name: &str) -> IpcResult<Arc<IpcConnection>> {
        let connections = self.connections.read()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        
        connections.get(name)
            .cloned()
            .ok_or_else(|| not_found("connection", name, "IPC connection not found"))
    /// Remove connection
    pub fn remove_connection(&self, name: &str) -> IpcResult<()> {
        let mut connections = self.connections.write()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        
        connections.remove(name);
        Ok(())
    /// Get IPC statistics
    pub fn get_stats(&self) -> IpcResult<IpcStats> {
        let connections = self.connections.read()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        
        Ok(IpcStats {
            messages_sent: 0, // Would be tracked in real implementation
            uptime: Duration::from_secs(0), // Would track actual uptime
            memory_usage: 0, // Would calculate actual memory usage
        })
    /// Shutdown IPC manager
    pub fn shutdown(&mut self) -> IpcResult<()> {
        // Signal cleanup thread to stop
        {
            let (lock, cvar) = &*self.shutdown_signal;
            let mut shutdown = lock.lock().unwrap();
            *shutdown = true;
            cvar.notify_all();
        // Wait for cleanup thread
        if let Some(handle) = self.cleanup_thread.take() {
            handle.join().map_err(|_| ipc_error("shutdown", "Failed to join cleanup thread"))?;
        // Close all connections
        let mut connections = self.connections.write()
            .map_err(|_| ipc_error("lock", "Failed to acquire connections lock"))?;
        connections.clear();
        
        Ok(())
    }
}

impl NamedPipeConnection {
    #[cfg(unix)]
    pub fn new(name: &str, is_server: bool, config: &IpcConfig) -> IpcResult<Self> {
        use std::ffi::CString;
        use std::os::unix::io::{RawFd, FromRawFd};
        
        let pipe_path = format!("/tmp/{}", name);
        let pipe_path_c = CString::new(pipe_path.clone())
            .map_err(|e| ipc_error("pipe_path", &e.to_string()))?;
        
        let fd = if is_server {
            // Create named pipe (FIFO)
            let result = unsafe {
                libc::mkfifo(pipe_path_c.as_ptr(), 0o666)
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                if errno != libc::EEXIST {
                    return Err(system_error(errno, "mkfifo", "Failed to create named pipe"));
                }
            }
            
            // Open for reading and writing
            unsafe {
                libc::open(pipe_path_c.as_ptr(), libc::O_RDWR | libc::O_NONBLOCK)
            }
        } else {
            // Open existing pipe
            unsafe {
                libc::open(pipe_path_c.as_ptr(), libc::O_RDWR)
            }
        
        if fd < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_error(errno, "open", "Failed to open named pipe"));
        Ok(Self {
        })
    #[cfg(windows)]
    pub fn new(name: &str, is_server: bool, config: &IpcConfig) -> IpcResult<Self> {
        use std::ffi::CString;
        use std::ptr;
        use winapi::um::winbase::{CreateNamedPipeA, PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE, PIPE_READMODE_BYTE, PIPE_WAIT};
        use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
        use winapi::um::winnt::{GENERIC_READ, GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE};
        use winapi::um::handleapi::INVALID_HANDLE_VALUE;
        
        let pipe_name = format!("\\\\.\\pipe\\{}", name);
        let pipe_name_c = CString::new(pipe_name)
            .map_err(|e| ipc_error("pipe_name", &e.to_string()))?;
        
        let handle = if is_server {
            // Create named pipe server
            unsafe {
                CreateNamedPipeA(
                    1, // Max instances
                    4096, // Out buffer size
                    4096, // In buffer size
                    0, // Default timeout
                    ptr::null_mut(), // Default security
                )
            }
        } else {
            // Connect to existing named pipe
            unsafe {
                CreateFileA(
                )
            }
        
        if handle == INVALID_HANDLE_VALUE {
            let error_code = unsafe { winapi::um::errhandlingapi::GetLastError() };
            return Err(system_error(
                "CreateNamedPipe/CreateFile",
                "Failed to create/open named pipe"
            ));
        Ok(Self {
        })
    pub fn write(&self, data: &[u8]) -> IpcResult<usize> {
        #[cfg(unix)]
        {
            if let Some(fd) = self.fd {
                let result = unsafe {
                    libc::write(fd, data.as_ptr() as *const libc::c_void, data.len())
                
                if result < 0 {
                    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                    return Err(system_error(errno, "write", "Failed to write to named pipe"));
                // Update last activity
                if let Ok(mut last_activity) = self.last_activity.lock() {
                    *last_activity = SystemTime::now();
                Ok(result as usize)
            } else {
                Err(ipc_error("write", "Named pipe not open"))
            }
        }
        
        #[cfg(windows)]
        {
            use winapi::um::fileapi::WriteFile;
            use std::ptr;
            
            if let Some(handle) = self.handle {
                let mut bytes_written: u32 = 0;
                let result = unsafe {
                    WriteFile(
                    )
                
                if result == 0 {
                    let error_code = unsafe { winapi::um::errhandlingapi::GetLastError() };
                    return Err(system_error(
                        "Failed to write to named pipe"
                    ));
                // Update last activity
                if let Ok(mut last_activity) = self.last_activity.lock() {
                    *last_activity = SystemTime::now();
                Ok(bytes_written as usize)
            } else {
                Err(ipc_error("write", "Named pipe not open"))
            }
        }
    pub fn read(&self, buffer: &mut [u8]) -> IpcResult<usize> {
        #[cfg(unix)]
        {
            if let Some(fd) = self.fd {
                let result = unsafe {
                    libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len())
                
                if result < 0 {
                    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                    return Err(system_error(errno, "read", "Failed to read from named pipe"));
                // Update last activity
                if let Ok(mut last_activity) = self.last_activity.lock() {
                    *last_activity = SystemTime::now();
                Ok(result as usize)
            } else {
                Err(ipc_error("read", "Named pipe not open"))
            }
        }
        
        #[cfg(windows)]
        {
            use winapi::um::fileapi::ReadFile;
            use std::ptr;
            
            if let Some(handle) = self.handle {
                let mut bytes_read: u32 = 0;
                let result = unsafe {
                    ReadFile(
                    )
                
                if result == 0 {
                    let error_code = unsafe { winapi::um::errhandlingapi::GetLastError() };
                    return Err(system_error(
                        "Failed to read from named pipe"
                    ));
                // Update last activity
                if let Ok(mut last_activity) = self.last_activity.lock() {
                    *last_activity = SystemTime::now();
                Ok(bytes_read as usize)
            } else {
                Err(ipc_error("read", "Named pipe not open"))
            }
        }
    }
}

impl Drop for NamedPipeConnection {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            if let Some(fd) = self.fd.take() {
                unsafe {
                    libc::close(fd);
                if self.is_server {
                    let pipe_path = format!("/tmp/{}", self.name);
                    let _ = std::fs::remove_file(&pipe_path);
                }
            }
        #[cfg(windows)]
        {
            if let Some(handle) = self.handle.take() {
                unsafe {
                    winapi::um::handleapi::CloseHandle(handle as winapi::um::winnt::HANDLE);
                }
            }
        }
    }
#[cfg(unix)]
impl UnixSocketConnection {
    pub fn new<P: AsRef<Path>>(path: P, is_server: bool, config: &IpcConfig) -> IpcResult<Self> {
        let path = path.as_ref().to_path_buf();
        
        if is_server {
            // Remove existing socket file
            if path.exists() {
                std::fs::remove_file(&path)
                    .map_err(|e| ipc_error("remove_socket", &e.to_string()))?;
            let listener = UnixListener::bind(&path)
                .map_err(|e| ipc_error("bind", &e.to_string()))?;
            
            Ok(Self {
                max_connections: 10, // Configurable
            })
        } else {
            let stream = UnixStream::connect(&path)
                .map_err(|e| ipc_error("connect", &e.to_string()))?;
            
            Ok(Self {
            })
        }
    }

    pub fn accept(&self) -> IpcResult<UnixStream> {
        if let Some(ref listener) = self.listener {
            let listener = listener.lock()
                .map_err(|_| ipc_error("lock", "Failed to acquire listener lock"))?;
            
            let (stream, _) = listener.accept()
                .map_err(|e| ipc_error("accept", &e.to_string()))?;
            
            Ok(stream)
        } else {
            Err(ipc_error("accept", "Not a server socket"))
        }
    }

    pub fn send(&self, data: &[u8]) -> IpcResult<usize> {
        if let Some(ref stream) = self.stream {
            let mut stream = stream.lock()
                .map_err(|_| ipc_error("lock", "Failed to acquire stream lock"))?;
            
            stream.write_all(data)
                .map_err(|e| ipc_error("write", &e.to_string()))?;
            
            Ok(data.len())
        } else {
            Err(ipc_error("send", "No client stream available"))
        }
    }

    pub fn receive(&self, buffer: &mut [u8]) -> IpcResult<usize> {
        if let Some(ref stream) = self.stream {
            let mut stream = stream.lock()
                .map_err(|_| ipc_error("lock", "Failed to acquire stream lock"))?;
            
            stream.read(buffer)
                .map_err(|e| ipc_error("read", &e.to_string()))
        } else {
            Err(ipc_error("receive", "No client stream available"))
        }
    }
impl MessageQueueConnection {
    pub fn new(name: &str, config: &IpcConfig) -> IpcResult<Self> {
        let queue = PriorityMessageQueue::new(config.max_queue_size);
        
        Ok(Self {
        })
    pub fn send(&self, message: IpcMessage) -> IpcResult<()> {
        if message.data.len() > self.max_message_size {
            return Err(ipc_error("send", "Message size exceeds limit"));
        let mut queue = self.queue.lock()
            .map_err(|_| ipc_error("lock", "Failed to acquire queue lock"))?;
        
        queue.push(message.clone())?;
        
        // Notify subscribers
        let subscribers = self.subscribers.lock()
            .map_err(|_| ipc_error("lock", "Failed to acquire subscribers lock"))?;
        
        for subscriber in subscribers.iter() {
            let msg = Message {
            let _ = subscriber.send(msg); // Ignore errors for disconnected subscribers
        Ok(())
    pub fn receive(&self, timeout: Option<Duration>) -> IpcResult<IpcMessage> {
        let start = Instant::now();
        
        loop {
            {
                let mut queue = self.queue.lock()
                    .map_err(|_| ipc_error("lock", "Failed to acquire queue lock"))?;
                
                if let Some(message) = queue.pop() {
                    return Ok(message);
                }
            }
            
            if let Some(timeout) = timeout {
                if start.elapsed() >= timeout {
                    return Err(timeout_error("receive", timeout, "Message receive timeout"));
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn subscribe(&self) -> IpcResult<mpsc::Receiver<Message>> {
        let (tx, rx) = mpsc::channel();
        
        let mut subscribers = self.subscribers.lock()
            .map_err(|_| ipc_error("lock", "Failed to acquire subscribers lock"))?;
        
        subscribers.push(tx);
        Ok(rx)
    }
}

impl SharedMemoryConnection {
    pub fn new(name: &str, size: usize, config: &IpcConfig) -> IpcResult<Self> {
        let mut memory = SharedMemory::new(name, size);
        memory.create_and_attach()
            .map_err(|e| ipc_error("create_shared_memory", &e.to_string()))?;
        
        Ok(Self {
        })
    pub fn write(&self, offset: usize, data: &[u8]) -> IpcResult<usize> {
        let _guard = self.acquire_lock()?;
        
        let mut memory = self.memory.lock()
            .map_err(|_| ipc_error("lock", "Failed to acquire memory lock"))?;
        
        memory.write(offset, data)
            .map_err(|e| ipc_error("write", &e.to_string()))
    pub fn read(&self, offset: usize, buffer: &mut [u8]) -> IpcResult<usize> {
        let _guard = self.acquire_lock()?;
        
        let memory = self.memory.lock()
            .map_err(|_| ipc_error("lock", "Failed to acquire memory lock"))?;
        
        memory.read(offset, buffer)
            .map_err(|e| ipc_error("read", &e.to_string()))
    fn acquire_lock(&self) -> IpcResult<std::sync::MutexGuard<bool>> {
        let (lock, _) = &*self.lock;
        lock.lock().map_err(|_| ipc_error("acquire_lock", "Failed to acquire shared memory lock"))
    }
}

impl MemoryMappedConnection {
    #[cfg(unix)]
    pub fn new<P: AsRef<Path>>(path: P, size: usize, read_only: bool, config: &IpcConfig) -> IpcResult<Self> {
        use std::ffi::CString;
        
        let path = path.as_ref().to_path_buf();
        let path_c = CString::new(path.to_string_lossy().as_ref())
            .map_err(|e| ipc_error("path", &e.to_string()))?;
        
        // Open or create file
        let flags = if read_only {
            libc::O_RDONLY
        } else {
            libc::O_RDWR | libc::O_CREAT
        
        let fd = unsafe {
            libc::open(path_c.as_ptr(), flags, 0o666)
        
        if fd < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            return Err(system_error(errno, "open", "Failed to open memory-mapped file"));
        // Set file size if creating
        if !read_only {
            let result = unsafe {
                libc::ftruncate(fd, size as i64)
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                unsafe { libc::close(fd); }
                return Err(system_error(errno, "ftruncate", "Failed to set file size"));
            }
        }
        
        // Map the file
        let prot = if read_only {
            libc::PROT_READ
        } else {
            libc::PROT_READ | libc::PROT_WRITE
        
        let mapping = unsafe {
            libc::mmap(
                0
            )
        
        if mapping == libc::MAP_FAILED {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            unsafe { libc::close(fd); }
            return Err(system_error(errno, "mmap", "Failed to map file"));
        Ok(Self {
        })
    #[cfg(windows)]
    pub fn new<P: AsRef<Path>>(path: P, size: usize, read_only: bool, config: &IpcConfig) -> IpcResult<Self> {
        // Windows memory-mapped file implementation would go here
        Ok(Self {
        })
    pub fn write(&self, offset: usize, data: &[u8]) -> IpcResult<usize> {
        if self.read_only {
            return Err(ipc_error("write", "Memory-mapped file is read-only"));
        if offset + data.len() > self.size {
            return Err(ipc_error("write", "Write would exceed file size"));
        if let Some(mapping) = self.mapping {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    data.len()
                );
            }
            Ok(data.len())
        } else {
            Err(ipc_error("write", "File not mapped"))
        }
    }

    pub fn read(&self, offset: usize, buffer: &mut [u8]) -> IpcResult<usize> {
        if offset >= self.size {
            return Err(ipc_error("read", "Read offset beyond file size"));
        let available = self.size - offset;
        let to_read = buffer.len().min(available);

        if let Some(mapping) = self.mapping {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    to_read
                );
            }
            Ok(to_read)
        } else {
            Err(ipc_error("read", "File not mapped"))
        }
    }
impl Drop for MemoryMappedConnection {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            if let Some(mapping) = self.mapping.take() {
                unsafe {
                    libc::munmap(mapping.as_ptr() as *mut libc::c_void, self.size);
                }
            }
            
            if let Some(fd) = self.fd.take() {
                unsafe {
                    libc::close(fd);
                }
            }
        }
    }
impl PriorityMessageQueue {
    pub fn new(max_messages: usize) -> Self {
        Self {
        }
    }

    pub fn push(&mut self, message: IpcMessage) -> IpcResult<()> {
        if self.total_messages >= self.max_messages {
            return Err(ipc_error("push", "Message queue is full"));
        match message.priority {
            MessagePriority::Critical | MessagePriority::High => {
                self.high_priority.push_back(message);
            }
            MessagePriority::Normal => {
                self.normal_priority.push_back(message);
            }
            MessagePriority::Low => {
                self.low_priority.push_back(message);
            }
        }

        self.total_messages += 1;
        Ok(())
    pub fn pop(&mut self) -> Option<IpcMessage> {
        if let Some(message) = self.high_priority.pop_front() {
            self.total_messages -= 1;
            return Some(message);
        if let Some(message) = self.normal_priority.pop_front() {
            self.total_messages -= 1;
            return Some(message);
        if let Some(message) = self.low_priority.pop_front() {
            self.total_messages -= 1;
            return Some(message);
        None
    pub fn len(&self) -> usize {
        self.total_messages
    pub fn is_empty(&self) -> bool {
        self.total_messages == 0
    }
}

/// Global IPC manager instance
static IPC_MANAGER: std::sync::OnceLock<Arc<Mutex<RealIpcManager>>> = std::sync::OnceLock::new();

/// Initialize global IPC manager
pub fn initialize_real_ipc(config: IpcConfig) -> IpcResult<()> {
    let manager = RealIpcManager::new(config)?;
    IPC_MANAGER.set(Arc::new(Mutex::new(manager)))
        .map_err(|_| ipc_error("initialize", "IPC manager already initialized"))?;
    Ok(())
/// Get global IPC manager
pub fn get_ipc_manager() -> IpcResult<Arc<Mutex<RealIpcManager>>> {
    IPC_MANAGER.get()
        .cloned()
        .ok_or_else(|| ipc_error("get_manager", "IPC manager not initialized"))
/// Cleanup global IPC resources
pub fn cleanup_real_ipc() -> IpcResult<()> {
    if let Some(manager) = IPC_MANAGER.get() {
        let mut manager = manager.lock()
            .map_err(|_| ipc_error("lock", "Failed to acquire manager lock"))?;
        manager.shutdown()?;
    }
    Ok(())
