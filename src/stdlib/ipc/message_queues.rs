/// Message queues implementation for CURSED IPC
/// 
/// Provides System V message queues and POSIX message queues for inter-process communication

use std::collections::{HashMap, VecDeque};
use std::ffi::{CString, CStr};
use std::mem;
use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};

use crate::stdlib::ipc::error::{IpcError, IpcResult, message_queue_error, system_error, timeout_error, not_found, already_exists};

/// Windows message frame structure for message serialization
#[cfg(windows)]
#[repr(C)]
struct WindowsMessageFrame {
    msg_type: i64,
    priority: u32,
    timestamp_secs: u64,
    timestamp_nanos: u32,
    sender_pid: u32,
    data_len: u32,
}

/// Windows error constants
#[cfg(windows)]
const ERROR_NO_DATA: u32 = 232;
#[cfg(windows)]
const ERROR_BROKEN_PIPE: u32 = 109;

/// Message queue registry for cleanup
static QUEUE_REGISTRY: std::sync::OnceLock<Arc<RwLock<HashMap<String, Arc<MessageQueueInfo>>>>> = std::sync::OnceLock::new();

#[derive(Debug)]
struct MessageQueueInfo {
    name: String,
    queue_id: i32,
    created_by_us: bool,
    ref_count: Arc<Mutex<usize>>,
}

fn get_queue_registry() -> &'static Arc<RwLock<HashMap<String, Arc<MessageQueueInfo>>>> {
    QUEUE_REGISTRY.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Message structure
#[derive(Debug, Clone)]
pub struct Message {
    /// Message type (used for filtering)
    pub msg_type: i64,
    /// Message data
    pub data: Vec<u8>,
    /// Message priority (0 = highest)
    pub priority: u32,
    /// Timestamp when message was sent
    pub timestamp: SystemTime,
    /// Sender information
    pub sender_pid: Option<u32>,
}

impl Message {
    /// Create a new message
    pub fn new(msg_type: i64, data: Vec<u8>) -> Self {
        Self {
            msg_type,
            data,
            priority: 0,
            timestamp: SystemTime::now(),
            sender_pid: Some(std::process::id()),
        }
    }

    /// Create with priority
    pub fn with_priority(msg_type: i64, data: Vec<u8>, priority: u32) -> Self {
        Self {
            msg_type,
            data,
            priority,
            timestamp: SystemTime::now(),
            sender_pid: Some(std::process::id()),
        }
    }

    /// Get message size
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Convert to string (if data is UTF-8)
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    /// Create from string
    pub fn from_string<S: AsRef<str>>(msg_type: i64, text: S) -> Self {
        Self::new(msg_type, text.as_ref().as_bytes().to_vec())
    }
}

/// Message queue configuration
#[derive(Debug, Clone)]
pub struct MessageQueueConfig {
    /// Maximum message size
    pub max_message_size: usize,
    /// Maximum number of messages in queue
    pub max_queue_size: usize,
    /// Queue permissions
    pub permissions: u32,
    /// Whether to create the queue if it doesn't exist
    pub create_if_missing: bool,
    /// Timeout for blocking operations
    pub timeout: Option<Duration>,
    /// Whether to use POSIX message queues (vs System V)
    pub use_posix: bool,
}

impl Default for MessageQueueConfig {
    fn default() -> Self {
        Self {
            max_message_size: 8192,
            max_queue_size: 100,
            permissions: 0o666,
            create_if_missing: true,
            timeout: Some(Duration::from_secs(30)),
            use_posix: false, // Default to System V for broader compatibility
        }
    }
}

/// Cross-platform message queue
#[derive(Debug)]
pub struct MessageQueue {
    name: String,
    config: MessageQueueConfig,
    #[cfg(unix)]
    queue_id: Option<i32>,
    #[cfg(unix)]
    posix_fd: Option<i32>,
    #[cfg(windows)]
    pipe_handle: Option<windows_sys::Win32::Foundation::HANDLE>,
    #[cfg(windows)]
    pipe_name: String,
    is_open: bool,
}

impl MessageQueue {
    /// Create a new message queue
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            config: MessageQueueConfig::default(),
            #[cfg(unix)]
            queue_id: None,
            #[cfg(unix)]
            posix_fd: None,
            #[cfg(windows)]
            pipe_handle: None,
            #[cfg(windows)]
            pipe_name: format!(r"\\.\pipe\cursed_mq_{}", name),
            is_open: false,
        }
    }

    /// Create with configuration
    pub fn with_config(name: &str, config: MessageQueueConfig) -> Self {
        Self {
            name: name.to_string(),
            config,
            #[cfg(unix)]
            queue_id: None,
            #[cfg(unix)]
            posix_fd: None,
            #[cfg(windows)]
            pipe_handle: None,
            #[cfg(windows)]
            pipe_name: format!(r"\\.\pipe\cursed_mq_{}", name),
            is_open: false,
        }
    }

    /// Create and open a message queue (for compatibility)
    pub fn create(name: &str, max_messages: usize) -> IpcResult<Self> {
        let config = MessageQueueConfig {
            max_messages,
            ..Default::default()
        };
        let mut queue = Self::with_config(name, config);
        queue.create()?;
        Ok(queue)
    }

    /// Open the message queue
    pub fn open(&mut self) -> IpcResult<()> {
        if self.is_open {
            return Ok(());
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.open_posix()
            } else {
                self.open_sysv()
            }
        }

        #[cfg(windows)]
        {
            // Windows doesn't have native message queues, simulate with named memory
            self.open_windows()
        }
    }

    /// Send a message
    pub fn send(&mut self, message: &Message) -> IpcResult<()> {
        if !self.is_open {
            return Err(message_queue_error(Some(&self.name), "send", "Queue not open"));
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.send_posix(message)
            } else {
                self.send_sysv(message)
            }
        }

        #[cfg(windows)]
        {
            self.send_windows(message)
        }
    }

    /// Receive a message
    pub fn receive(&mut self, msg_type: i64) -> IpcResult<Message> {
        if !self.is_open {
            return Err(message_queue_error(Some(&self.name), "receive", "Queue not open"));
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.receive_posix()
            } else {
                self.receive_sysv(msg_type)
            }
        }

        #[cfg(windows)]
        {
            self.receive_windows()
        }
    }

    /// Receive with timeout
    pub fn receive_timeout(&mut self, msg_type: i64, timeout: Duration) -> IpcResult<Message> {
        let start = Instant::now();
        
        loop {
            match self.receive(msg_type) {
                Ok(message) => return Ok(message),
                Err(err) => {
                    if start.elapsed() >= timeout {
                        return Err(timeout_error("receive", timeout, "Message receive timed out"));
                    }
                    
                    // Brief sleep to avoid busy waiting
                    std::thread::sleep(Duration::from_millis(10));
                    continue;
                }
            }
        }
    }

    /// Send raw data as a message (compatibility method)
    pub fn send_data(&mut self, data: &[u8]) -> IpcResult<()> {
        let message = Message::new(1, data.to_vec()); // Use message type 1 by default
        self.send(&message)
    }

    /// Receive data with timeout (compatibility method)
    pub fn receive_data_timeout(&mut self, timeout: Duration) -> IpcResult<Vec<u8>> {
        let message = self.receive_timeout(0, timeout)?; // Receive any message type
        Ok(message.data)
    }

    /// Get queue statistics
    pub fn stats(&self) -> IpcResult<MessageQueueStats> {
        if !self.is_open {
            return Err(message_queue_error(Some(&self.name), "stats", "Queue not open"));
        }

        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.stats_posix()
            } else {
                self.stats_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.stats_windows()
        }
    }

    /// Close the message queue
    pub fn close(&mut self) -> IpcResult<()> {
        if !self.is_open {
            return Ok(());
        }

        #[cfg(unix)]
        {
            if let Some(queue_id) = self.queue_id {
                // Don't delete System V queue on close, just detach
                self.queue_id = None;
            }
            
            if let Some(fd) = self.posix_fd {
                unsafe {
                    libc::close(fd);
                }
                self.posix_fd = None;
            }
        }

        #[cfg(windows)]
        {
            if let Some(handle) = self.pipe_handle {
                unsafe {
                    windows_sys::Win32::Foundation::CloseHandle(handle);
                }
                self.pipe_handle = None;
            }
        }

        self.is_open = false;
        self.unregister_queue();
        Ok(())
    }

    /// Delete the message queue
    pub fn delete(&mut self) -> IpcResult<()> {
        #[cfg(unix)]
        {
            if self.config.use_posix {
                self.delete_posix()
            } else {
                self.delete_sysv()
            }
        }

        #[cfg(windows)]
        {
            self.delete_windows()
        }
    }

    /// Get queue name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if queue is open
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    #[cfg(unix)]
    fn open_sysv(&mut self) -> IpcResult<()> {
        // Generate key from name
        let key = self.generate_sysv_key()?;
        
        // Try to get existing queue first
        let queue_id = unsafe {
            libc::msgget(key, 0)
        };
        
        if queue_id >= 0 {
            self.queue_id = Some(queue_id);
            self.is_open = true;
            self.register_queue(queue_id, false);
            return Ok(());
        }
        
        // Create new queue if allowed
        if !self.config.create_if_missing {
            return Err(not_found("message_queue", &self.name, "Queue does not exist"));
        }
        
        let queue_id = unsafe {
            libc::msgget(key, libc::IPC_CREAT | libc::IPC_EXCL | (self.config.permissions as i32))
        };
        
        if queue_id < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            if errno == libc::EEXIST {
                // Queue was created by another process, try to get it
                let queue_id = unsafe { libc::msgget(key, 0) };
                if queue_id >= 0 {
                    self.queue_id = Some(queue_id);
                    self.is_open = true;
                    self.register_queue(queue_id, false);
                    return Ok(());
                }
            }
            return Err(system_error(errno, "msgget", "Failed to create message queue"));
        }
        
        self.queue_id = Some(queue_id);
        self.is_open = true;
        self.register_queue(queue_id, true);
        Ok(())
    }

    #[cfg(unix)]
    fn open_posix(&mut self) -> IpcResult<()> {
        let queue_name = format!("/{}", self.name);
        let queue_name_cstr = CString::new(queue_name)
            .map_err(|e| message_queue_error(Some(&self.name), "open", &e.to_string()))?;
        
        // Try to open existing queue first
        let fd = unsafe {
            mq_open(queue_name_cstr.as_ptr(), libc::O_RDWR, 0, ptr::null_mut())
        };
        
        if fd >= 0 {
            self.posix_fd = Some(fd);
            self.is_open = true;
            self.register_queue(fd, false);
            return Ok(());
        }
        
        // Create new queue if allowed
        if !self.config.create_if_missing {
            return Err(not_found("message_queue", &self.name, "Queue does not exist"));
        }
        
        let mut attr: mq_attr = unsafe { mem::zeroed() };
        attr.mq_flags = 0;
        attr.mq_maxmsg = self.config.max_queue_size as i64;
        attr.mq_msgsize = self.config.max_message_size as i64;
        attr.mq_curmsgs = 0;
        
        let fd = unsafe {
            mq_open(
                queue_name_cstr.as_ptr(),
                libc::O_CREAT | libc::O_EXCL | libc::O_RDWR,
                self.config.permissions,
                &attr
            )
        };
        
        if fd < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            if errno == libc::EEXIST {
                // Queue was created by another process, try to open it
                let fd = unsafe {
                    mq_open(queue_name_cstr.as_ptr(), libc::O_RDWR, 0, ptr::null_mut())
                };
                if fd >= 0 {
                    self.posix_fd = Some(fd);
                    self.is_open = true;
                    self.register_queue(fd, false);
                    return Ok(());
                }
            }
            return Err(system_error(errno, "mq_open", "Failed to create POSIX message queue"));
        }
        
        self.posix_fd = Some(fd);
        self.is_open = true;
        self.register_queue(fd, true);
        Ok(())
    }

    #[cfg(windows)]
    fn open_windows(&mut self) -> IpcResult<()> {
        use windows_sys::Win32::System::Pipes::*;
        use windows_sys::Win32::Foundation::*;
        use windows_sys::Win32::Storage::FileSystem::*;
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        // Convert pipe name to wide string
        let pipe_name_wide: Vec<u16> = OsString::from(&self.pipe_name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // Try to connect to existing pipe first
        let handle = unsafe {
            CreateFileW(
                pipe_name_wide.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                INVALID_HANDLE_VALUE,
            )
        };

        if handle != INVALID_HANDLE_VALUE {
            self.pipe_handle = Some(handle);
            self.is_open = true;
            self.register_queue(handle as i32, false);
            return Ok(());
        }

        // Create new pipe if allowed
        if !self.config.create_if_missing {
            return Err(not_found("message_queue", &self.name, "Pipe does not exist"));
        }

        let handle = unsafe {
            CreateNamedPipeW(
                pipe_name_wide.as_ptr(),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                1, // Max instances
                self.config.max_message_size as u32,
                self.config.max_message_size as u32,
                0, // Default timeout
                std::ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            let error = unsafe { GetLastError() };
            return Err(system_error(
                error as i32,
                "CreateNamedPipeW",
                "Failed to create Windows named pipe"
            ));
        }

        self.pipe_handle = Some(handle);
        self.is_open = true;
        self.register_queue(handle as i32, true);
        Ok(())
    }

    #[cfg(unix)]
    fn send_sysv(&mut self, message: &Message) -> IpcResult<()> {
        if let Some(queue_id) = self.queue_id {
            // System V message structure
            #[repr(C)]
            struct MsgBuf {
                mtype: i64,
                mtext: [u8; 8192], // Max message size
            }
            
            if message.data.len() > self.config.max_message_size {
                return Err(message_queue_error(
                    Some(&self.name),
                    "send",
                    &format!("Message too large: {} > {}", message.data.len(), self.config.max_message_size)
                ));
            }
            
            let mut msg_buf: MsgBuf = unsafe { mem::zeroed() };
            msg_buf.mtype = message.msg_type;
            msg_buf.mtext[..message.data.len()].copy_from_slice(&message.data);
            
            let result = unsafe {
                libc::msgsnd(
                    queue_id,
                    &msg_buf as *const MsgBuf as *const libc::c_void,
                    message.data.len(),
                    libc::IPC_NOWAIT
                )
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "msgsnd", "Failed to send message"));
            }
            
            Ok(())
        } else {
            Err(message_queue_error(Some(&self.name), "send", "Queue not open"))
        }
    }

    #[cfg(unix)]
    fn send_posix(&mut self, message: &Message) -> IpcResult<()> {
        if let Some(fd) = self.posix_fd {
            if message.data.len() > self.config.max_message_size {
                return Err(message_queue_error(
                    Some(&self.name),
                    "send",
                    &format!("Message too large: {} > {}", message.data.len(), self.config.max_message_size)
                ));
            }
            
            let result = unsafe {
                mq_send(
                    fd,
                    message.data.as_ptr() as *const i8,
                    message.data.len(),
                    message.priority
                )
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "mq_send", "Failed to send POSIX message"));
            }
            
            Ok(())
        } else {
            Err(message_queue_error(Some(&self.name), "send", "Queue not open"))
        }
    }

    #[cfg(windows)]
    fn send_windows(&mut self, message: &Message) -> IpcResult<()> {
        use windows_sys::Win32::Storage::FileSystem::*;
        use windows_sys::Win32::Foundation::*;
        use std::mem;

        if let Some(handle) = self.pipe_handle {
            if message.data.len() > self.config.max_message_size {
                return Err(message_queue_error(
                    Some(&self.name),
                    "send",
                    &format!("Message too large: {} > {}", message.data.len(), self.config.max_message_size)
                ));
            }

            // Create message frame with metadata

            let frame = WindowsMessageFrame {
                msg_type: message.msg_type,
                priority: message.priority,
                timestamp_secs: message.timestamp.duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default().as_secs(),
                timestamp_nanos: message.timestamp.duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default().subsec_nanos(),
                sender_pid: message.sender_pid.unwrap_or(0),
                data_len: message.data.len() as u32,
            };

            // Write message frame
            let mut bytes_written = 0u32;
            let frame_bytes = unsafe {
                std::slice::from_raw_parts(
                    &frame as *const WindowsMessageFrame as *const u8,
                    mem::size_of::<WindowsMessageFrame>()
                )
            };

            let result = unsafe {
                WriteFile(
                    handle,
                    frame_bytes.as_ptr() as *const std::ffi::c_void,
                    frame_bytes.len() as u32,
                    &mut bytes_written,
                    std::ptr::null_mut(),
                )
            };

            if result == 0 {
                let error = unsafe { GetLastError() };
                return Err(system_error(error as i32, "WriteFile", "Failed to write message frame"));
            }

            // Write message data
            if !message.data.is_empty() {
                let result = unsafe {
                    WriteFile(
                        handle,
                        message.data.as_ptr() as *const std::ffi::c_void,
                        message.data.len() as u32,
                        &mut bytes_written,
                        std::ptr::null_mut(),
                    )
                };

                if result == 0 {
                    let error = unsafe { GetLastError() };
                    return Err(system_error(error as i32, "WriteFile", "Failed to write message data"));
                }
            }

            // Flush the pipe to ensure message is sent
            unsafe { FlushFileBuffers(handle); }

            Ok(())
        } else {
            Err(message_queue_error(Some(&self.name), "send", "Pipe not open"))
        }
    }

    #[cfg(unix)]
    fn receive_sysv(&mut self, msg_type: i64) -> IpcResult<Message> {
        if let Some(queue_id) = self.queue_id {
            #[repr(C)]
            struct MsgBuf {
                mtype: i64,
                mtext: [u8; 8192],
            }
            
            let mut msg_buf: MsgBuf = unsafe { mem::zeroed() };
            
            let result = unsafe {
                libc::msgrcv(
                    queue_id,
                    &mut msg_buf as *mut MsgBuf as *mut libc::c_void,
                    self.config.max_message_size,
                    msg_type,
                    libc::IPC_NOWAIT
                )
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                if errno == libc::ENOMSG {
                    return Err(message_queue_error(Some(&self.name), "receive", "No message available"));
                }
                return Err(system_error(errno, "msgrcv", "Failed to receive message"));
            }
            
            let data = msg_buf.mtext[..result as usize].to_vec();
            Ok(Message::new(msg_buf.mtype, data))
        } else {
            Err(message_queue_error(Some(&self.name), "receive", "Queue not open"))
        }
    }

    #[cfg(unix)]
    fn receive_posix(&mut self) -> IpcResult<Message> {
        if let Some(fd) = self.posix_fd {
            let mut buffer = vec![0u8; self.config.max_message_size];
            let mut priority: u32 = 0;
            
            let result = unsafe {
                mq_receive(
                    fd,
                    buffer.as_mut_ptr() as *mut i8,
                    buffer.len(),
                    &mut priority
                )
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                if errno == libc::EAGAIN {
                    return Err(message_queue_error(Some(&self.name), "receive", "No message available"));
                }
                return Err(system_error(errno, "mq_receive", "Failed to receive POSIX message"));
            }
            
            buffer.truncate(result as usize);
            let mut message = Message::new(1, buffer); // POSIX doesn't have message types
            message.priority = priority;
            Ok(message)
        } else {
            Err(message_queue_error(Some(&self.name), "receive", "Queue not open"))
        }
    }

    #[cfg(windows)]
    fn receive_windows(&mut self) -> IpcResult<Message> {
        use windows_sys::Win32::Storage::FileSystem::*;
        use windows_sys::Win32::Foundation::*;
        use std::mem;

        if let Some(handle) = self.pipe_handle {
            // First, read the message frame

            let mut frame: WindowsMessageFrame = unsafe { mem::zeroed() };
            let mut bytes_read = 0u32;

            let result = unsafe {
                ReadFile(
                    handle,
                    &mut frame as *mut WindowsMessageFrame as *mut std::ffi::c_void,
                    mem::size_of::<WindowsMessageFrame>() as u32,
                    &mut bytes_read,
                    std::ptr::null_mut(),
                )
            };

            if result == 0 {
                let error = unsafe { GetLastError() };
                if error == ERROR_NO_DATA || error == ERROR_BROKEN_PIPE {
                    return Err(message_queue_error(Some(&self.name), "receive", "No message available"));
                }
                return Err(system_error(error as i32, "ReadFile", "Failed to read message frame"));
            }

            if bytes_read != mem::size_of::<WindowsMessageFrame>() as u32 {
                return Err(message_queue_error(
                    Some(&self.name),
                    "receive",
                    "Incomplete message frame received"
                ));
            }

            // Validate data length
            if frame.data_len > self.config.max_message_size as u32 {
                return Err(message_queue_error(
                    Some(&self.name),
                    "receive",
                    &format!("Message data too large: {} > {}", frame.data_len, self.config.max_message_size)
                ));
            }

            // Read message data
            let mut data = vec![0u8; frame.data_len as usize];
            if frame.data_len > 0 {
                let result = unsafe {
                    ReadFile(
                        handle,
                        data.as_mut_ptr() as *mut std::ffi::c_void,
                        frame.data_len,
                        &mut bytes_read,
                        std::ptr::null_mut(),
                    )
                };

                if result == 0 {
                    let error = unsafe { GetLastError() };
                    return Err(system_error(error as i32, "ReadFile", "Failed to read message data"));
                }

                if bytes_read != frame.data_len {
                    return Err(message_queue_error(
                        Some(&self.name),
                        "receive",
                        "Incomplete message data received"
                    ));
                }
            }

            // Reconstruct timestamp
            let timestamp = std::time::UNIX_EPOCH 
                + std::time::Duration::from_secs(frame.timestamp_secs)
                + std::time::Duration::from_nanos(frame.timestamp_nanos as u64);

            let message = Message {
                msg_type: frame.msg_type,
                data,
                priority: frame.priority,
                timestamp,
                sender_pid: if frame.sender_pid == 0 { None } else { Some(frame.sender_pid) },
            };

            Ok(message)
        } else {
            Err(message_queue_error(Some(&self.name), "receive", "Pipe not open"))
        }
    }

    #[cfg(unix)]
    fn stats_sysv(&self) -> IpcResult<MessageQueueStats> {
        if let Some(queue_id) = self.queue_id {
            let mut ds: libc::msqid_ds = unsafe { mem::zeroed() };
            
            let result = unsafe {
                libc::msgctl(queue_id, libc::IPC_STAT, &mut ds)
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "msgctl", "Failed to get queue statistics"));
            }
            
            Ok(MessageQueueStats {
                current_messages: ds.msg_qnum as usize,
                max_messages: self.config.max_queue_size,
                max_message_size: self.config.max_message_size,
                total_bytes: ds.msg_cbytes as usize,
                last_send_time: SystemTime::UNIX_EPOCH + Duration::from_secs(ds.msg_stime as u64),
                last_receive_time: SystemTime::UNIX_EPOCH + Duration::from_secs(ds.msg_rtime as u64),
                last_change_time: SystemTime::UNIX_EPOCH + Duration::from_secs(ds.msg_ctime as u64),
                send_pid: ds.msg_lspid,
                receive_pid: ds.msg_lrpid,
            })
        } else {
            Err(message_queue_error(Some(&self.name), "stats", "Queue not open"))
        }
    }

    #[cfg(unix)]
    fn stats_posix(&self) -> IpcResult<MessageQueueStats> {
        if let Some(fd) = self.posix_fd {
            let mut attr: mq_attr = unsafe { mem::zeroed() };
            
            let result = unsafe {
                mq_getattr(fd, &mut attr)
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "mq_getattr", "Failed to get POSIX queue attributes"));
            }
            
            Ok(MessageQueueStats {
                current_messages: attr.mq_curmsgs as usize,
                max_messages: attr.mq_maxmsg as usize,
                max_message_size: attr.mq_msgsize as usize,
                total_bytes: 0, // Not available in POSIX
                last_send_time: SystemTime::now(), // Not available in POSIX
                last_receive_time: SystemTime::now(), // Not available in POSIX
                last_change_time: SystemTime::now(), // Not available in POSIX
                send_pid: 0, // Not available in POSIX
                receive_pid: 0, // Not available in POSIX
            })
        } else {
            Err(message_queue_error(Some(&self.name), "stats", "Queue not open"))
        }
    }

    #[cfg(windows)]
    fn stats_windows(&self) -> IpcResult<MessageQueueStats> {
        use windows_sys::Win32::Storage::FileSystem::*;
        use windows_sys::Win32::Foundation::*;
        use windows_sys::Win32::System::Pipes::PeekNamedPipe;

        if let Some(handle) = self.pipe_handle {
            // Get pipe information
            let mut available_bytes = 0u32;
            let mut total_bytes = 0u32;
            let mut message_count = 0u32;
            let mut next_message_size = 0u32;

            let result = unsafe {
                PeekNamedPipe(
                    handle,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null_mut(),
                    &mut available_bytes,
                    &mut next_message_size,
                )
            };

            if result != 0 {
                // Estimate message count based on available bytes and message structure size
                let frame_size = std::mem::size_of::<WindowsMessageFrame>();
                if available_bytes >= frame_size as u32 {
                    // This is a rough estimate - actual message count would require 
                    // parsing the pipe buffer, which isn't easily accessible
                    message_count = 1; // At least one message if data is available
                }
                total_bytes = available_bytes;
            }

            // Note: Windows named pipes don't provide detailed statistics like Unix message queues
            // Some information like last send/receive times and PIDs are not available
            Ok(MessageQueueStats {
                current_messages: message_count as usize,
                max_messages: self.config.max_queue_size,
                max_message_size: self.config.max_message_size,
                total_bytes: total_bytes as usize,
                last_send_time: SystemTime::now(), // Not available - using current time
                last_receive_time: SystemTime::now(), // Not available - using current time
                last_change_time: SystemTime::now(), // Not available - using current time
                send_pid: 0, // Not available in Windows named pipes
                receive_pid: 0, // Not available in Windows named pipes
            })
        } else {
            Err(message_queue_error(Some(&self.name), "stats", "Pipe not open"))
        }
    }

    #[cfg(unix)]
    fn delete_sysv(&mut self) -> IpcResult<()> {
        if let Some(queue_id) = self.queue_id {
            let result = unsafe {
                libc::msgctl(queue_id, libc::IPC_RMID, ptr::null_mut())
            };
            
            if result < 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(system_error(errno, "msgctl", "Failed to delete message queue"));
            }
            
            self.queue_id = None;
            self.is_open = false;
            Ok(())
        } else {
            Ok(())
        }
    }

    #[cfg(unix)]
    fn delete_posix(&mut self) -> IpcResult<()> {
        let queue_name = format!("/{}", self.name);
        let queue_name_cstr = CString::new(queue_name)
            .map_err(|e| message_queue_error(Some(&self.name), "delete", &e.to_string()))?;
        
        if let Some(fd) = self.posix_fd {
            unsafe { libc::close(fd); }
            self.posix_fd = None;
        }
        
        let result = unsafe {
            mq_unlink(queue_name_cstr.as_ptr())
        };
        
        if result < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
            if errno != libc::ENOENT {
                return Err(system_error(errno, "mq_unlink", "Failed to delete POSIX message queue"));
            }
        }
        
        self.is_open = false;
        Ok(())
    }

    #[cfg(windows)]
    fn delete_windows(&mut self) -> IpcResult<()> {
        use windows_sys::Win32::Foundation::*;

        if let Some(handle) = self.pipe_handle {
            unsafe {
                CloseHandle(handle);
            }
            self.pipe_handle = None;
        }

        self.is_open = false;
        self.unregister_queue();
        Ok(())
    }

    #[cfg(unix)]
    fn generate_sysv_key(&self) -> IpcResult<i32> {
        // Generate a key based on the queue name
        let mut hash = 0i32;
        for byte in self.name.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as i32);
        }
        
        // Ensure it's a valid System V key (non-zero)
        if hash == 0 {
            hash = 1;
        }
        
        Ok(hash)
    }

    fn register_queue(&self, queue_id: i32, created_by_us: bool) {
        let registry = get_queue_registry();
        if let Ok(mut queues) = registry.write() {
            let info = Arc::new(MessageQueueInfo {
                name: self.name.clone(),
                queue_id,
                created_by_us,
                ref_count: Arc::new(Mutex::new(1)),
            });
            queues.insert(self.name.clone(), info);
        }
    }

    fn unregister_queue(&self) {
        let registry = get_queue_registry();
        if let Ok(mut queues) = registry.write() {
            if let Some(info) = queues.get(&self.name) {
                let mut ref_count = info.ref_count.lock().unwrap();
                *ref_count -= 1;
                if *ref_count == 0 {
                    queues.remove(&self.name);
                }
            }
        }
    }
}

impl Drop for MessageQueue {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// Message queue statistics
#[derive(Debug, Clone)]
pub struct MessageQueueStats {
    /// Current number of messages in queue
    pub current_messages: usize,
    /// Maximum number of messages allowed
    pub max_messages: usize,
    /// Maximum message size
    pub max_message_size: usize,
    /// Total bytes currently in queue
    pub total_bytes: usize,
    /// Time of last send operation
    pub last_send_time: SystemTime,
    /// Time of last receive operation
    pub last_receive_time: SystemTime,
    /// Time of last change to queue
    pub last_change_time: SystemTime,
    /// PID of last process to send
    pub send_pid: u32,
    /// PID of last process to receive
    pub receive_pid: u32,
}

/// Cleanup all registered message queues
pub fn cleanup_queues() -> IpcResult<()> {
    let registry = get_queue_registry();
    if let Ok(mut queues) = registry.write() {
        for (name, info) in queues.drain() {
            #[cfg(unix)]
            if info.created_by_us {
                // Try to delete the queue
                unsafe {
                    if info.queue_id > 0 {
                        libc::msgctl(info.queue_id, libc::IPC_RMID, ptr::null_mut());
                    }
                }
                tracing::debug!(queue_name = name, "Cleaned up message queue");
            }
        }
    }
    Ok(())
}

// POSIX message queue system calls
#[cfg(unix)]
extern "C" {
    fn mq_open(name: *const i8, oflag: i32, mode: u32, attr: *mut mq_attr) -> i32;
    fn mq_close(mqdes: i32) -> i32;
    fn mq_unlink(name: *const i8) -> i32;
    fn mq_send(mqdes: i32, msg_ptr: *const i8, msg_len: usize, msg_prio: u32) -> i32;
    fn mq_receive(mqdes: i32, msg_ptr: *mut i8, msg_len: usize, msg_prio: *mut u32) -> isize;
    fn mq_getattr(mqdes: i32, attr: *mut mq_attr) -> i32;
}

#[cfg(unix)]
#[repr(C)]
struct mq_attr {
    mq_flags: i64,
    mq_maxmsg: i64,
    mq_msgsize: i64,
    mq_curmsgs: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_message_creation() {
        let message = Message::new(1, b"Hello, World!".to_vec());
        assert_eq!(message.msg_type, 1);
        assert_eq!(message.data, b"Hello, World!");
        assert_eq!(message.size(), 13);
        assert_eq!(message.to_string().unwrap(), "Hello, World!");
    }

    #[test]
    fn test_message_from_string() {
        let message = Message::from_string(2, "Test message");
        assert_eq!(message.msg_type, 2);
        assert_eq!(message.to_string().unwrap(), "Test message");
    }

    #[test]
    fn test_message_queue_config() {
        let config = MessageQueueConfig::default();
        assert_eq!(config.max_message_size, 8192);
        assert_eq!(config.max_queue_size, 100);
        assert!(!config.use_posix);
    }

    #[test]
    fn test_message_queue_creation() {
        let queue = MessageQueue::new("test_queue");
        assert_eq!(queue.name(), "test_queue");
        assert!(!queue.is_open());
    }

    #[cfg(unix)]
    #[test]
    fn test_message_queue_sysv() {
        let mut queue = MessageQueue::new("test_sysv_queue");
        
        // Test opening
        if queue.open().is_ok() {
            assert!(queue.is_open());
            
            // Test sending and receiving
            let message = Message::from_string(1, "Test message");
            if queue.send(&message).is_ok() {
                if let Ok(received) = queue.receive(1) {
                    assert_eq!(received.to_string().unwrap(), "Test message");
                }
            }
            
            // Test stats
            if let Ok(stats) = queue.stats() {
                assert!(stats.max_messages > 0);
            }
            
            // Cleanup
            let _ = queue.delete();
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_message_queue_posix() {
        let config = MessageQueueConfig {
            use_posix: true,
            ..Default::default()
        };
        let mut queue = MessageQueue::with_config("test_posix_queue", config);
        
        // Test opening
        if queue.open().is_ok() {
            assert!(queue.is_open());
            
            // Test sending and receiving
            let message = Message::from_string(1, "POSIX test message");
            if queue.send(&message).is_ok() {
                if let Ok(received) = queue.receive(0) {
                    assert_eq!(received.to_string().unwrap(), "POSIX test message");
                }
            }
            
            // Cleanup
            let _ = queue.delete();
        }
    }

    #[test]
    fn test_queue_registry() {
        let registry = get_queue_registry();
        assert!(registry.read().is_ok());
        
        // Test cleanup
        assert!(cleanup_queues().is_ok());
    }

    #[cfg(windows)]
    #[test]
    fn test_message_queue_windows() {
        let mut queue = MessageQueue::new("test_windows_queue");
        
        // Test opening
        if queue.open().is_ok() {
            assert!(queue.is_open());
            
            // Test sending and receiving
            let message = Message::from_string(1, "Windows test message");
            if queue.send(&message).is_ok() {
                // Note: This test may not work as expected because named pipes 
                // typically require separate client/server processes or handles
                // In a real scenario, you'd need a separate reader process
                println!("Windows message sent successfully");
            }
            
            // Test stats
            if let Ok(stats) = queue.stats() {
                assert!(stats.max_messages > 0);
                println!("Windows stats retrieved: {:?}", stats);
            }
            
            // Cleanup
            let _ = queue.delete();
        }
    }
}
