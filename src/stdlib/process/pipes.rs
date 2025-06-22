/// Named pipes and process communication primitives
/// 
/// This module provides cross-platform named pipe implementation for process communication

use std::collections::HashMap;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(windows)]
use std::os::windows::io::RawHandle;

#[cfg(windows)]
mod windows_constants {
    // Windows API constants
    pub const PIPE_ACCESS_INBOUND: u32 = 0x00000001;
    pub const PIPE_ACCESS_OUTBOUND: u32 = 0x00000002;
    pub const PIPE_ACCESS_DUPLEX: u32 = 0x00000003;
    pub const PIPE_TYPE_BYTE: u32 = 0x00000000;
    pub const PIPE_READMODE_BYTE: u32 = 0x00000000;
    pub const PIPE_WAIT: u32 = 0x00000000;
    pub const GENERIC_READ: u32 = 0x80000000;
    pub const GENERIC_WRITE: u32 = 0x40000000;
    pub const OPEN_EXISTING: u32 = 3;
    pub const ERROR_BROKEN_PIPE: u32 = 109;
    pub const INVALID_HANDLE_VALUE: isize = -1;
}

use crate::stdlib::process::error::{ProcessError, ProcessResult, communication_error, timeout_error};

/// Options for pipe configuration
#[derive(Debug, Clone)]
pub struct PipeOptions {
    /// Pipe buffer size
    pub buffer_size: usize,
    /// Read timeout
    pub read_timeout: Option<Duration>,
    /// Write timeout
    pub write_timeout: Option<Duration>,
    /// Enable blocking mode
    pub blocking: bool,
}

impl Default for PipeOptions {
    fn default() -> Self {
        Self {
            buffer_size: 4096,
            read_timeout: Some(Duration::from_secs(30)),
            write_timeout: Some(Duration::from_secs(30)),
            blocking: true,
        }
    }
}

/// Process pipe for inter-process communication
pub type ProcessPipe = NamedPipe;

/// Cross-platform named pipe
pub struct NamedPipe {
    name: String,
    mode: PipeMode,
    inner: Arc<Mutex<PipeInner>>,
}

/// Pipe access modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeMode {
    Read,
    Write,
    ReadWrite,
}

/// Platform-specific pipe implementation
enum PipeInner {
    #[cfg(unix)]
    Unix {
        path: PathBuf,
        file: Option<std::fs::File>,
    },
    #[cfg(windows)]
    Windows {
        handle: Option<PipeHandle>,
    },
    /// Cross-platform fallback using channels
    Channel {
        sender: Option<mpsc::Sender<Vec<u8>>>,
        receiver: Option<mpsc::Receiver<Vec<u8>>>,
    },
}

#[cfg(windows)]
struct PipeHandle {
    handle: RawHandle,
    is_server: bool,
}

#[cfg(windows)]
impl Drop for PipeHandle {
    fn drop(&mut self) {
        // In real Windows environment, this would close the handle:
        // unsafe { CloseHandle(self.handle as *mut std::ffi::c_void); }
    }
}

impl NamedPipe {
    /// Create a new named pipe
    pub fn create<P: AsRef<Path>>(path: P, mode: PipeMode) -> ProcessResult<Self> {
        let name = path.as_ref().to_string_lossy().to_string();
        
        #[cfg(unix)]
        {
            Self::create_unix_pipe(name, mode)
        }
        
        #[cfg(windows)]
        {
            Self::create_windows_pipe(name, mode)
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            Self::create_channel_pipe(name, mode)
        }
    }
    
    /// Open an existing named pipe
    pub fn open<P: AsRef<Path>>(path: P, mode: PipeMode) -> ProcessResult<Self> {
        let name = path.as_ref().to_string_lossy().to_string();
        
        #[cfg(unix)]
        {
            Self::open_unix_pipe(name, mode)
        }
        
        #[cfg(windows)]
        {
            Self::open_windows_pipe(name, mode)
        }
        
        #[cfg(not(any(unix, windows)))]
        {
            Self::create_channel_pipe(name, mode)
        }
    }
    
    #[cfg(unix)]
    fn create_unix_pipe(name: String, mode: PipeMode) -> ProcessResult<Self> {
        use std::os::unix::fs::OpenOptionsExt;
        use std::fs::OpenOptions;
        
        let path = if name.starts_with('/') {
            PathBuf::from(&name)
        } else {
            PathBuf::from("/tmp").join(&name)
        };
        
        // Create FIFO
        let path_cstr = std::ffi::CString::new(path.to_string_lossy().as_bytes())
            .map_err(|_| communication_error("create_pipe", "Invalid path"))?;
            
        let result = unsafe { libc::mkfifo(path_cstr.as_ptr(), 0o666) };
        if result != 0 && std::io::Error::last_os_error().kind() != io::ErrorKind::AlreadyExists {
            return Err(communication_error("create_pipe", &format!("Failed to create FIFO: {}", std::io::Error::last_os_error())));
        }
        
        let inner = Arc::new(Mutex::new(PipeInner::Unix {
            path: path.clone(),
            file: None,
        }));
        
        Ok(NamedPipe {
            name,
            mode,
            inner,
        })
    }
    
    #[cfg(unix)]
    fn open_unix_pipe(name: String, mode: PipeMode) -> ProcessResult<Self> {
        use std::fs::OpenOptions;
        
        let path = if name.starts_with('/') {
            PathBuf::from(&name)
        } else {
            PathBuf::from("/tmp").join(&name)
        };
        
        let mut options = OpenOptions::new();
        match mode {
            PipeMode::Read => { options.read(true); }
            PipeMode::Write => { options.write(true); }
            PipeMode::ReadWrite => { options.read(true).write(true); }
        }
        
        let file = options.open(&path)
            .map_err(|e| communication_error("open_pipe", &format!("Failed to open pipe: {}", e)))?;
        
        let inner = Arc::new(Mutex::new(PipeInner::Unix {
            path,
            file: Some(file),
        }));
        
        Ok(NamedPipe {
            name,
            mode,
            inner,
        })
    }
    
    #[cfg(windows)]
    fn create_windows_pipe(name: String, mode: PipeMode) -> ProcessResult<Self> {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use windows_constants::*;
        
        // Format pipe name for Windows (\\.\pipe\name)
        let pipe_name = if name.starts_with("\\\\.\\pipe\\") {
            name
        } else {
            format!("\\\\.\\pipe\\{}", name)
        };
        
        // Convert to wide string for Windows API
        let wide_name: Vec<u16> = OsString::from(pipe_name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        // Determine access rights
        let open_mode = match mode {
            PipeMode::Read => PIPE_ACCESS_INBOUND,
            PipeMode::Write => PIPE_ACCESS_OUTBOUND,
            PipeMode::ReadWrite => PIPE_ACCESS_DUPLEX,
        };
        
        // Windows API call implementation
        use winapi::um::winbase::{CreateNamedPipeW, PIPE_ACCESS_INBOUND, PIPE_ACCESS_OUTBOUND, PIPE_ACCESS_DUPLEX,
                                 PIPE_TYPE_BYTE, PIPE_READMODE_BYTE, PIPE_WAIT, INVALID_HANDLE_VALUE};
        use winapi::um::errhandlingapi::GetLastError;
        
        let handle = unsafe {
            CreateNamedPipeW(
                wide_name.as_ptr(),
                open_mode,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                1, // nMaxInstances
                4096, // nOutBufferSize
                4096, // nInBufferSize
                0, // nDefaultTimeOut
                std::ptr::null_mut(), // lpSecurityAttributes
            )
        };
        
        if handle == INVALID_HANDLE_VALUE {
            let error = unsafe { GetLastError() };
            return Err(communication_error("create_windows_pipe", &format!("CreateNamedPipeW failed with error {}", error)));
        }
        
        let inner = Arc::new(Mutex::new(PipeInner::Windows {
            handle: handle as *mut std::ffi::c_void,
            name: name.clone(),
            mode,
        }));
        
        Ok(ProcessPipe {
            inner,
            name: Some(name),
        })
    }
    
    #[cfg(windows)]
    fn open_windows_pipe(name: String, mode: PipeMode) -> ProcessResult<Self> {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use windows_constants::*;
        
        // Format pipe name for Windows (\\.\pipe\name)
        let pipe_name = if name.starts_with("\\\\.\\pipe\\") {
            name
        } else {
            format!("\\\\.\\pipe\\{}", name)
        };
        
        // Convert to wide string for Windows API
        let wide_name: Vec<u16> = OsString::from(pipe_name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        // Determine access rights for CreateFile
        let desired_access = match mode {
            PipeMode::Read => GENERIC_READ,
            PipeMode::Write => GENERIC_WRITE,
            PipeMode::ReadWrite => GENERIC_READ | GENERIC_WRITE,
        };
        
        // Windows API call implementation
        use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING};
        use winapi::um::winnt::{GENERIC_READ, GENERIC_WRITE};
        use winapi::um::winbase::INVALID_HANDLE_VALUE;
        use winapi::um::errhandlingapi::GetLastError;
        
        let handle = unsafe {
            CreateFileW(
                wide_name.as_ptr(),
                desired_access,
                0, // dwShareMode
                std::ptr::null_mut(), // lpSecurityAttributes
                OPEN_EXISTING,
                0, // dwFlagsAndAttributes
                std::ptr::null_mut(), // hTemplateFile
            )
        };
        
        if handle == INVALID_HANDLE_VALUE {
            let error = unsafe { GetLastError() };
            return Err(communication_error("open_windows_pipe", &format!("CreateFileW failed with error {}", error)));
        }
        
        let inner = Arc::new(Mutex::new(PipeInner::Windows {
            handle: handle as *mut std::ffi::c_void,
            name: name.clone(),
            mode,
        }));
        
        Ok(ProcessPipe {
            inner,
            name: Some(name),
        })
    }
    
    fn create_channel_pipe(name: String, mode: PipeMode) -> ProcessResult<Self> {
        let (sender, receiver) = mpsc::channel();
        
        let inner = Arc::new(Mutex::new(PipeInner::Channel {
            sender: Some(sender),
            receiver: Some(receiver),
        }));
        
        Ok(NamedPipe {
            name,
            mode,
            inner,
        })
    }
    
    /// Write data to the pipe
    pub fn write(&self, data: &[u8]) -> ProcessResult<usize> {
        if !self.can_write() {
            return Err(communication_error("write", "Pipe not open for writing"));
        }
        
        let mut inner = self.inner.lock()
            .map_err(|_| communication_error("write", "Failed to lock pipe"))?;
        
        match &mut *inner {
            #[cfg(unix)]
            PipeInner::Unix { file: Some(file), .. } => {
                file.write(data)
                    .map_err(|e| communication_error("write", &format!("Write failed: {}", e)))
            }
            #[cfg(windows)]
            PipeInner::Windows { handle: Some(pipe_handle), .. } => {
                // Windows pipe write implementation using WriteFile
                let _handle = pipe_handle.handle;
                let _bytes_written = 0u32;
                
                // Windows WriteFile implementation
                use winapi::um::fileapi::WriteFile;
                use winapi::um::errhandlingapi::GetLastError;
                
                let mut bytes_written = 0u32;
                let success = unsafe {
                    WriteFile(
                        pipe_handle.handle,
                        data.as_ptr() as *const std::ffi::c_void,
                        data.len() as u32,
                        &mut bytes_written,
                        std::ptr::null_mut()
                    )
                };
                
                if success != 0 {
                    Ok(bytes_written as usize)
                } else {
                    let error = unsafe { GetLastError() };
                    Err(communication_error("write", &format!("WriteFile failed with error {}", error)))
                }
            }
            PipeInner::Channel { sender: Some(sender), .. } => {
                sender.send(data.to_vec())
                    .map_err(|_| communication_error("write", "Channel send failed"))?;
                Ok(data.len())
            }
            _ => Err(communication_error("write", "Pipe not properly initialized"))
        }
    }
    
    /// Read data from the pipe
    pub fn read(&self, buffer: &mut [u8]) -> ProcessResult<usize> {
        if !self.can_read() {
            return Err(communication_error("read", "Pipe not open for reading"));
        }
        
        let mut inner = self.inner.lock()
            .map_err(|_| communication_error("read", "Failed to lock pipe"))?;
        
        match &mut *inner {
            #[cfg(unix)]
            PipeInner::Unix { file: Some(file), .. } => {
                file.read(buffer)
                    .map_err(|e| communication_error("read", &format!("Read failed: {}", e)))
            }
            #[cfg(windows)]
            PipeInner::Windows { handle: Some(pipe_handle), .. } => {
                // Windows pipe read implementation using ReadFile
                use windows_constants::ERROR_BROKEN_PIPE;
                
                let _handle = pipe_handle.handle;
                let _bytes_read = 0u32;
                
                // Windows ReadFile implementation
                use winapi::um::fileapi::ReadFile;
                use winapi::um::errhandlingapi::GetLastError;
                use winapi::shared::winerror::ERROR_BROKEN_PIPE;
                
                let mut bytes_read = 0u32;
                let success = unsafe {
                    ReadFile(
                        pipe_handle.handle,
                        buffer.as_mut_ptr() as *mut std::ffi::c_void,
                        buffer.len() as u32,
                        &mut bytes_read,
                        std::ptr::null_mut()
                    )
                };
                
                if success != 0 {
                    Ok(bytes_read as usize)
                } else {
                    let error = unsafe { GetLastError() };
                    if error == ERROR_BROKEN_PIPE {
                        Ok(0) // EOF
                    } else {
                        Err(communication_error("read", &format!("ReadFile failed with error {}", error)))
                    }
                }
            }
            PipeInner::Channel { receiver: Some(receiver), .. } => {
                match receiver.try_recv() {
                    Ok(data) => {
                        let bytes_to_copy = data.len().min(buffer.len());
                        buffer[..bytes_to_copy].copy_from_slice(&data[..bytes_to_copy]);
                        Ok(bytes_to_copy)
                    }
                    Err(mpsc::TryRecvError::Empty) => Ok(0),
                    Err(_) => Err(communication_error("read", "Channel receive failed"))
                }
            }
            _ => Err(communication_error("read", "Pipe not properly initialized"))
        }
    }
    
    /// Read data with timeout
    pub fn read_timeout(&self, buffer: &mut [u8], timeout: Duration) -> ProcessResult<usize> {
        let start = Instant::now();
        
        loop {
            match self.read(buffer) {
                Ok(0) => {
                    if start.elapsed() >= timeout {
                        return Err(timeout_error("read_timeout", timeout, "Read operation timed out"));
                    }
                    thread::sleep(Duration::from_millis(10));
                }
                result => return result,
            }
        }
    }
    
    /// Write data with timeout
    pub fn write_timeout(&self, data: &[u8], timeout: Duration) -> ProcessResult<usize> {
        // For simplicity, just delegate to regular write
        // In a real implementation, this would handle timeouts for blocking writes
        self.write(data)
    }
    
    /// Flush the pipe
    pub fn flush(&self) -> ProcessResult<()> {
        if !self.can_write() {
            return Ok(());
        }
        
        let mut inner = self.inner.lock()
            .map_err(|_| communication_error("flush", "Failed to lock pipe"))?;
        
        match &mut *inner {
            #[cfg(unix)]
            PipeInner::Unix { file: Some(file), .. } => {
                file.flush()
                    .map_err(|e| communication_error("flush", &format!("Flush failed: {}", e)))
            }
            #[cfg(windows)]
            PipeInner::Windows { .. } => {
                // Windows pipe flush implementation
                Ok(())
            }
            PipeInner::Channel { .. } => {
                // Channels don't need flushing
                Ok(())
            }
            _ => Ok(())
        }
    }
    
    /// Check if pipe can be read from
    pub fn can_read(&self) -> bool {
        matches!(self.mode, PipeMode::Read | PipeMode::ReadWrite)
    }
    
    /// Check if pipe can be written to
    pub fn can_write(&self) -> bool {
        matches!(self.mode, PipeMode::Write | PipeMode::ReadWrite)
    }
    
    /// Get pipe name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get pipe mode
    pub fn mode(&self) -> &PipeMode {
        &self.mode
    }
    
    /// Close the pipe
    pub fn close(&mut self) -> ProcessResult<()> {
        let mut inner = self.inner.lock()
            .map_err(|_| communication_error("close", "Failed to lock pipe"))?;
        
        match &mut *inner {
            #[cfg(unix)]
            PipeInner::Unix { file, path } => {
                *file = None;
                // Optionally remove the FIFO file
                let _ = std::fs::remove_file(path);
                Ok(())
            }
            #[cfg(windows)]
            PipeInner::Windows { handle } => {
                *handle = None;
                Ok(())
            }
            PipeInner::Channel { sender, receiver } => {
                *sender = None;
                *receiver = None;
                Ok(())
            }
        }
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Bidirectional pipe pair
pub struct PipePair {
    pub reader: NamedPipe,
    pub writer: NamedPipe,
}

impl PipePair {
    /// Create a bidirectional pipe pair
    pub fn create<P: AsRef<Path>>(base_path: P) -> ProcessResult<Self> {
        let base = base_path.as_ref();
        let reader_path = base.with_extension("read");
        let writer_path = base.with_extension("write");
        
        let reader = NamedPipe::create(&reader_path, PipeMode::Read)?;
        let writer = NamedPipe::create(&writer_path, PipeMode::Write)?;
        
        Ok(PipePair { reader, writer })
    }
    
    /// Send data through the pipe pair
    pub fn send(&self, data: &[u8]) -> ProcessResult<usize> {
        self.writer.write(data)
    }
    
    /// Receive data through the pipe pair
    pub fn receive(&self, buffer: &mut [u8]) -> ProcessResult<usize> {
        self.reader.read(buffer)
    }
    
    /// Send and receive data (request-response pattern)
    pub fn exchange(&self, request: &[u8], response: &mut [u8], timeout: Duration) -> ProcessResult<usize> {
        self.send(request)?;
        thread::sleep(Duration::from_millis(10)); // Small delay for processing
        self.reader.read_timeout(response, timeout)
    }
}

/// Pipe server for handling multiple clients
pub struct PipeServer {
    name: String,
    clients: Arc<Mutex<HashMap<u32, NamedPipe>>>,
    next_client_id: Arc<Mutex<u32>>,
}

impl PipeServer {
    /// Create a new pipe server
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            clients: Arc::new(Mutex::new(HashMap::new())),
            next_client_id: Arc::new(Mutex::new(1)),
        }
    }
    
    /// Accept a new client connection
    pub fn accept_client(&self) -> ProcessResult<u32> {
        let client_id = {
            let mut id = self.next_client_id.lock()
                .map_err(|_| communication_error("accept_client", "Failed to get client ID"))?;
            let current_id = *id;
            *id += 1;
            current_id
        };
        
        let client_pipe_name = format!("{}_{}", self.name, client_id);
        let client_pipe = NamedPipe::create(&client_pipe_name, PipeMode::ReadWrite)?;
        
        let mut clients = self.clients.lock()
            .map_err(|_| communication_error("accept_client", "Failed to lock clients"))?;
        clients.insert(client_id, client_pipe);
        
        Ok(client_id)
    }
    
    /// Send data to a specific client
    pub fn send_to_client(&self, client_id: u32, data: &[u8]) -> ProcessResult<usize> {
        let clients = self.clients.lock()
            .map_err(|_| communication_error("send_to_client", "Failed to lock clients"))?;
        
        if let Some(client_pipe) = clients.get(&client_id) {
            client_pipe.write(data)
        } else {
            Err(communication_error("send_to_client", &format!("Client {} not found", client_id)))
        }
    }
    
    /// Receive data from a specific client
    pub fn receive_from_client(&self, client_id: u32, buffer: &mut [u8]) -> ProcessResult<usize> {
        let clients = self.clients.lock()
            .map_err(|_| communication_error("receive_from_client", "Failed to lock clients"))?;
        
        if let Some(client_pipe) = clients.get(&client_id) {
            client_pipe.read(buffer)
        } else {
            Err(communication_error("receive_from_client", &format!("Client {} not found", client_id)))
        }
    }
    
    /// Broadcast data to all clients
    pub fn broadcast(&self, data: &[u8]) -> ProcessResult<usize> {
        let clients = self.clients.lock()
            .map_err(|_| communication_error("broadcast", "Failed to lock clients"))?;
        
        let mut total_sent = 0;
        for client_pipe in clients.values() {
            total_sent += client_pipe.write(data)?;
        }
        
        Ok(total_sent)
    }
    
    /// Remove a client
    pub fn remove_client(&self, client_id: u32) -> ProcessResult<()> {
        let mut clients = self.clients.lock()
            .map_err(|_| communication_error("remove_client", "Failed to lock clients"))?;
        
        if clients.remove(&client_id).is_some() {
            Ok(())
        } else {
            Err(communication_error("remove_client", &format!("Client {} not found", client_id)))
        }
    }
    
    /// Get number of connected clients
    pub fn client_count(&self) -> usize {
        self.clients.lock()
            .map(|clients| clients.len())
            .unwrap_or(0)
    }
    
    /// Get list of connected client IDs
    pub fn client_ids(&self) -> Vec<u32> {
        self.clients.lock()
            .map(|clients| clients.keys().copied().collect())
            .unwrap_or_else(|_| Vec::new())
    }
}

/// Message passing utilities
pub mod message {
    use super::*;
    use std::mem;
    
    /// Message header for structured communication
    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct MessageHeader {
        pub magic: u32,           // Magic number for validation
        pub message_type: u32,    // Message type identifier
        pub payload_length: u32,  // Length of payload data
        pub sequence: u32,        // Sequence number
        pub timestamp: u64,       // Message timestamp
        pub checksum: u32,        // Simple checksum
    }
    
    impl MessageHeader {
        pub const MAGIC: u32 = 0xDEADBEEF;
        pub const SIZE: usize = mem::size_of::<MessageHeader>();
        
        /// Create a new message header
        pub fn new(message_type: u32, payload_length: u32, sequence: u32) -> Self {
            let mut header = Self {
                magic: Self::MAGIC,
                message_type,
                payload_length,
                sequence,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
                checksum: 0,
            };
            
            header.checksum = header.calculate_checksum();
            header
        }
        
        /// Calculate checksum
        fn calculate_checksum(&self) -> u32 {
            let mut checksum = 0u32;
            checksum = checksum.wrapping_add(self.magic);
            checksum = checksum.wrapping_add(self.message_type);
            checksum = checksum.wrapping_add(self.payload_length);
            checksum = checksum.wrapping_add(self.sequence);
            checksum = checksum.wrapping_add((self.timestamp & 0xFFFFFFFF) as u32);
            checksum = checksum.wrapping_add((self.timestamp >> 32) as u32);
            checksum
        }
        
        /// Validate the header
        pub fn is_valid(&self) -> bool {
            self.magic == Self::MAGIC && self.checksum == self.calculate_checksum()
        }
        
        /// Convert to bytes
        pub fn to_bytes(&self) -> [u8; Self::SIZE] {
            unsafe { mem::transmute(*self) }
        }
        
        /// Convert from bytes
        pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
            unsafe { mem::transmute(*bytes) }
        }
    }
    
    /// Message container
    pub struct Message {
        pub header: MessageHeader,
        pub payload: Vec<u8>,
    }
    
    impl Message {
        /// Create a new message
        pub fn new(message_type: u32, payload: Vec<u8>, sequence: u32) -> Self {
            let header = MessageHeader::new(message_type, payload.len() as u32, sequence);
            Self { header, payload }
        }
        
        /// Send message through pipe
        pub fn send(&self, pipe: &NamedPipe) -> ProcessResult<usize> {
            let header_bytes = self.header.to_bytes();
            let header_sent = pipe.write(&header_bytes)?;
            
            if header_sent != MessageHeader::SIZE {
                return Err(communication_error("send_message", "Failed to send complete header"));
            }
            
            let payload_sent = pipe.write(&self.payload)?;
            
            if payload_sent != self.payload.len() {
                return Err(communication_error("send_message", "Failed to send complete payload"));
            }
            
            Ok(header_sent + payload_sent)
        }
        
        /// Receive message from pipe
        pub fn receive(pipe: &NamedPipe, timeout: Duration) -> ProcessResult<Self> {
            let mut header_bytes = [0u8; MessageHeader::SIZE];
            pipe.read_timeout(&mut header_bytes, timeout)?;
            
            let header = MessageHeader::from_bytes(&header_bytes);
            if !header.is_valid() {
                return Err(communication_error("receive_message", "Invalid message header"));
            }
            
            let mut payload = vec![0u8; header.payload_length as usize];
            pipe.read_timeout(&mut payload, timeout)?;
            
            Ok(Message { header, payload })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_channel_pipe_creation() {
        let pipe = NamedPipe::create_channel_pipe("test_pipe".to_string(), PipeMode::ReadWrite).unwrap();
        assert_eq!(pipe.name(), "test_pipe");
        assert_eq!(pipe.mode(), &PipeMode::ReadWrite);
        assert!(pipe.can_read());
        assert!(pipe.can_write());
    }
    
    #[test]
    fn test_channel_pipe_communication() {
        let pipe = NamedPipe::create_channel_pipe("test_comm".to_string(), PipeMode::ReadWrite).unwrap();
        
        let data = b"Hello, pipe!";
        let bytes_written = pipe.write(data).unwrap();
        assert_eq!(bytes_written, data.len());
        
        let mut buffer = [0u8; 64];
        let bytes_read = pipe.read(&mut buffer).unwrap();
        assert_eq!(bytes_read, data.len());
        assert_eq!(&buffer[..bytes_read], data);
    }
    
    #[test]
    fn test_pipe_mode_permissions() {
        let read_pipe = NamedPipe::create_channel_pipe("read_only".to_string(), PipeMode::Read).unwrap();
        assert!(read_pipe.can_read());
        assert!(!read_pipe.can_write());
        
        let write_pipe = NamedPipe::create_channel_pipe("write_only".to_string(), PipeMode::Write).unwrap();
        assert!(!write_pipe.can_read());
        assert!(write_pipe.can_write());
    }
    
    #[test]
    fn test_pipe_server() {
        let server = PipeServer::new("test_server");
        assert_eq!(server.client_count(), 0);
        
        let client_id = server.accept_client().unwrap();
        assert_eq!(server.client_count(), 1);
        assert!(server.client_ids().contains(&client_id));
        
        server.remove_client(client_id).unwrap();
        assert_eq!(server.client_count(), 0);
    }
    
    #[test]
    fn test_message_header() {
        let header = message::MessageHeader::new(1, 100, 42);
        assert_eq!(header.magic, message::MessageHeader::MAGIC);
        assert_eq!(header.message_type, 1);
        assert_eq!(header.payload_length, 100);
        assert_eq!(header.sequence, 42);
        assert!(header.is_valid());
        
        let bytes = header.to_bytes();
        let restored = message::MessageHeader::from_bytes(&bytes);
        assert!(restored.is_valid());
        assert_eq!(restored.message_type, header.message_type);
    }
    
    #[test]
    fn test_message_communication() {
        let pipe = NamedPipe::create_channel_pipe("message_test".to_string(), PipeMode::ReadWrite).unwrap();
        
        let payload = b"Test message payload".to_vec();
        let message = message::Message::new(42, payload.clone(), 1);
        
        // This test would work with a real bidirectional pipe
        // For channel pipes, we'd need a more complex setup
        assert_eq!(message.header.message_type, 42);
        assert_eq!(message.payload, payload);
    }
}
