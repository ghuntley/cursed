/// Process communication and IPC mechanisms
use std::collections::HashMap;
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, ChildStderr, Stdio, ExitStatus};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use super::error::{ProcessError, ProcessResult};

/// Process communication channels
#[derive(Debug)]
pub struct ProcessChannels {
    /// Standard input pipe to child process
    pub stdin: Option<ChildStdin>,
    /// Standard output pipe from child process
    pub stdout: Option<ChildStdout>,
    /// Standard error pipe from child process  
    pub stderr: Option<ChildStderr>,
}

/// Bidirectional process communication
#[derive(Debug)]
pub struct ProcessCommunication {
    /// Child process handle
    pub child: Child,
    /// Communication channels
    pub channels: ProcessChannels,
    /// Background reader threads
    stdout_thread: Option<thread::JoinHandle<ProcessResult<Vec<u8>>>>,
    stderr_thread: Option<thread::JoinHandle<ProcessResult<Vec<u8>>>>,
    /// Output receivers
    stdout_receiver: Option<mpsc::Receiver<String>>,
    stderr_receiver: Option<mpsc::Receiver<String>>,
}

/// Named pipe for inter-process communication
#[derive(Debug)]
pub struct NamedPipe {
    /// Pipe name/path
    pub name: String,
    /// Pipe file path
    pub path: PathBuf,
    /// Read handle
    read_handle: Option<std::fs::File>,
    /// Write handle
    write_handle: Option<std::fs::File>,
}

/// Shared memory segment
#[derive(Debug)]
pub struct SharedMemory {
    /// Memory segment name
    pub name: String,
    /// Size in bytes
    pub size: usize,
    /// Memory region
    memory: Arc<Mutex<Vec<u8>>>,
}

/// Message queue for process communication
#[derive(Debug)]
pub struct MessageQueue {
    /// Queue name
    pub name: String,
    /// Maximum message size
    pub max_message_size: usize,
    /// Maximum queue size
    pub max_queue_size: usize,
    /// Internal message storage
    messages: Arc<Mutex<std::collections::VecDeque<Vec<u8>>>>,
}

/// IPC communication types
#[derive(Debug, Clone)]
pub enum IpcType {
    /// Anonymous pipes
    Pipe,
    /// Named pipes (FIFOs)
    NamedPipe(String),
    /// Unix domain sockets
    UnixSocket(String),
    /// TCP sockets
    TcpSocket(String, u16),
    /// Shared memory
    SharedMemory(String, usize),
    /// Message queues
    MessageQueue(String),
}

/// Process communication configuration
#[derive(Debug, Clone)]
pub struct CommunicationConfig {
    /// Buffer size for I/O operations
    pub buffer_size: usize,
    /// Timeout for communication operations
    pub timeout: Option<Duration>,
    /// Enable real-time communication
    pub realtime: bool,
    /// IPC mechanisms to use
    pub ipc_types: Vec<IpcType>,
}

impl Default for CommunicationConfig {
    fn default() -> Self {
        Self {
            buffer_size: 8192,
            timeout: Some(Duration::from_secs(30)),
            realtime: false,
            ipc_types: vec![IpcType::Pipe],
        }
    }
}

impl ProcessCommunication {
    /// Create new process communication with captured I/O
    pub fn new(mut child: Child) -> ProcessResult<Self> {
        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        
        let channels = ProcessChannels {
            stdin,
            stdout,
            stderr,
        };
        
        Ok(ProcessCommunication {
            child,
            channels,
            stdout_thread: None,
            stderr_thread: None,
            stdout_receiver: None,
            stderr_receiver: None,
        })
    }
    
    /// Start background readers for stdout and stderr
    pub fn start_readers(&mut self) -> ProcessResult<()> {
        // Start stdout reader
        if let Some(stdout) = self.channels.stdout.take() {
            let (tx, rx) = mpsc::channel();
            self.stdout_receiver = Some(rx);
            
            let handle = thread::spawn(move || {
                Self::read_stream_lines(stdout, tx)
            });
            self.stdout_thread = Some(handle);
        }
        
        // Start stderr reader
        if let Some(stderr) = self.channels.stderr.take() {
            let (tx, rx) = mpsc::channel();
            self.stderr_receiver = Some(rx);
            
            let handle = thread::spawn(move || {
                Self::read_stream_lines(stderr, tx)
            });
            self.stderr_thread = Some(handle);
        }
        
        Ok(())
    }
    
    /// Write data to stdin
    pub fn write_stdin(&mut self, data: &[u8]) -> ProcessResult<()> {
        if let Some(ref mut stdin) = self.channels.stdin {
            stdin.write_all(data)
                .map_err(|e| ProcessError::CommunicationError(format!("Stdin write error: {}", e)))?;
            stdin.flush()
                .map_err(|e| ProcessError::CommunicationError(format!("Stdin flush error: {}", e)))?;
            Ok(())
        } else {
            Err(ProcessError::CommunicationError("Stdin not available".to_string()))
        }
    }
    
    /// Write line to stdin
    pub fn write_line(&mut self, line: &str) -> ProcessResult<()> {
        let data = format!("{}\n", line);
        self.write_stdin(data.as_bytes())
    }
    
    /// Read stdout line (non-blocking)
    pub fn read_stdout_line(&self) -> ProcessResult<Option<String>> {
        if let Some(ref receiver) = self.stdout_receiver {
            match receiver.try_recv() {
                Ok(line) => Ok(Some(line)),
                Err(mpsc::TryRecvError::Empty) => Ok(None),
                Err(mpsc::TryRecvError::Disconnected) => Ok(None),
            }
        } else {
            Err(ProcessError::CommunicationError("Stdout reader not started".to_string()))
        }
    }
    
    /// Read stderr line (non-blocking)
    pub fn read_stderr_line(&self) -> ProcessResult<Option<String>> {
        if let Some(ref receiver) = self.stderr_receiver {
            match receiver.try_recv() {
                Ok(line) => Ok(Some(line)),
                Err(mpsc::TryRecvError::Empty) => Ok(None),
                Err(mpsc::TryRecvError::Disconnected) => Ok(None),
            }
        } else {
            Err(ProcessError::CommunicationError("Stderr reader not started".to_string()))
        }
    }
    
    /// Read stdout line with timeout
    pub fn read_stdout_line_timeout(&self, timeout: Duration) -> ProcessResult<Option<String>> {
        if let Some(ref receiver) = self.stdout_receiver {
            match receiver.recv_timeout(timeout) {
                Ok(line) => Ok(Some(line)),
                Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
                Err(mpsc::RecvTimeoutError::Disconnected) => Ok(None),
            }
        } else {
            Err(ProcessError::CommunicationError("Stdout reader not started".to_string()))
        }
    }
    
    /// Read stderr line with timeout
    pub fn read_stderr_line_timeout(&self, timeout: Duration) -> ProcessResult<Option<String>> {
        if let Some(ref receiver) = self.stderr_receiver {
            match receiver.recv_timeout(timeout) {
                Ok(line) => Ok(Some(line)),
                Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
                Err(mpsc::RecvTimeoutError::Disconnected) => Ok(None),
            }
        } else {
            Err(ProcessError::CommunicationError("Stderr reader not started".to_string()))
        }
    }
    
    /// Get process ID
    pub fn id(&self) -> u32 {
        self.child.id()
    }
    
    /// Check if process is running
    pub fn is_running(&mut self) -> ProcessResult<bool> {
        match self.child.try_wait() {
            Ok(Some(_)) => Ok(false),
            Ok(None) => Ok(true),
            Err(e) => Err(ProcessError::from(e)),
        }
    }
    
    /// Wait for process completion
    pub fn wait(&mut self) -> ProcessResult<ExitStatus> {
        // Wait for child process
        let status = self.child.wait()
            .map_err(|e| ProcessError::from(e))?;
        
        // Wait for reader threads to complete
        if let Some(handle) = self.stdout_thread.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.stderr_thread.take() {
            let _ = handle.join();
        }
        
        Ok(status)
    }
    
    /// Kill the process
    pub fn kill(&mut self) -> ProcessResult<()> {
        self.child.kill().map_err(|e| ProcessError::from(e))
    }
    
    /// Read stream lines in background thread
    fn read_stream_lines<R: Read + Send + 'static>(
        stream: R, 
        sender: mpsc::Sender<String>
    ) -> ProcessResult<Vec<u8>> {
        let reader = BufReader::new(stream);
        let mut all_output = Vec::new();
        
        for line in reader.lines() {
            match line {
                Ok(line_str) => {
                    all_output.extend_from_slice(line_str.as_bytes());
                    all_output.push(b'\n');
                    
                    // Send line to receiver (ignore errors if receiver is dropped)
                    let _ = sender.send(line_str);
                }
                Err(e) => {
                    return Err(ProcessError::CommunicationError(format!("Read error: {}", e)));
                }
            }
        }
        
        Ok(all_output)
    }
}

impl NamedPipe {
    /// Create a new named pipe
    pub fn create<S: Into<String>>(name: S) -> ProcessResult<Self> {
        let name = name.into();
        
        #[cfg(unix)]
        let path = PathBuf::from(format!("/tmp/{}", name));
        
        #[cfg(windows)]
        let path = PathBuf::from(format!("\\\\.\\pipe\\{}", name));
        
        #[cfg(unix)]
        {
            // Create FIFO on Unix
            use std::ffi::CString;
            let c_path = CString::new(path.to_string_lossy().as_ref())
                .map_err(|_| ProcessError::InvalidArguments("Invalid path".to_string()))?;
            
            let result = unsafe { libc::mkfifo(c_path.as_ptr(), 0o666) };
            if result != 0 && std::io::Error::last_os_error().kind() != std::io::ErrorKind::AlreadyExists {
                return Err(ProcessError::SystemError(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "Failed to create FIFO".to_string()
                ));
            }
        }
        
        Ok(NamedPipe {
            name,
            path,
            read_handle: None,
            write_handle: None,
        })
    }
    
    /// Open pipe for reading
    pub fn open_read(&mut self) -> ProcessResult<()> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| ProcessError::CommunicationError(format!("Failed to open pipe for reading: {}", e)))?;
        
        self.read_handle = Some(file);
        Ok(())
    }
    
    /// Open pipe for writing
    pub fn open_write(&mut self) -> ProcessResult<()> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open(&self.path)
            .map_err(|e| ProcessError::CommunicationError(format!("Failed to open pipe for writing: {}", e)))?;
        
        self.write_handle = Some(file);
        Ok(())
    }
    
    /// Write data to pipe
    pub fn write(&mut self, data: &[u8]) -> ProcessResult<usize> {
        if let Some(ref mut handle) = self.write_handle {
            handle.write(data)
                .map_err(|e| ProcessError::CommunicationError(format!("Pipe write error: {}", e)))
        } else {
            Err(ProcessError::CommunicationError("Pipe not open for writing".to_string()))
        }
    }
    
    /// Read data from pipe
    pub fn read(&mut self, buffer: &mut [u8]) -> ProcessResult<usize> {
        if let Some(ref mut handle) = self.read_handle {
            handle.read(buffer)
                .map_err(|e| ProcessError::CommunicationError(format!("Pipe read error: {}", e)))
        } else {
            Err(ProcessError::CommunicationError("Pipe not open for reading".to_string()))
        }
    }
    
    /// Close and remove the pipe
    pub fn close(self) -> ProcessResult<()> {
        drop(self.read_handle);
        drop(self.write_handle);
        
        #[cfg(unix)]
        {
            if self.path.exists() {
                std::fs::remove_file(&self.path)
                    .map_err(|e| ProcessError::CommunicationError(format!("Failed to remove pipe: {}", e)))?;
            }
        }
        
        Ok(())
    }
}

impl SharedMemory {
    /// Create new shared memory segment
    pub fn create<S: Into<String>>(name: S, size: usize) -> ProcessResult<Self> {
        let name = name.into();
        
        // This is a simplified implementation using Arc<Mutex<Vec<u8>>>
        // In a real implementation, you'd use platform-specific shared memory APIs
        let memory = Arc::new(Mutex::new(vec![0u8; size]));
        
        Ok(SharedMemory {
            name,
            size,
            memory,
        })
    }
    
    /// Get a reference to the shared memory for reading/writing
    pub fn get_memory(&self) -> Arc<Mutex<Vec<u8>>> {
        self.memory.clone()
    }
    
    /// Write data to shared memory at offset
    pub fn write_at(&self, offset: usize, data: &[u8]) -> ProcessResult<()> {
        let mut memory = self.memory.lock()
            .map_err(|_| ProcessError::CommunicationError("Failed to lock shared memory".to_string()))?;
        
        if offset + data.len() > self.size {
            return Err(ProcessError::InvalidArguments("Write would exceed memory bounds".to_string()));
        }
        
        memory[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
    
    /// Read data from shared memory at offset
    pub fn read_at(&self, offset: usize, length: usize) -> ProcessResult<Vec<u8>> {
        let memory = self.memory.lock()
            .map_err(|_| ProcessError::CommunicationError("Failed to lock shared memory".to_string()))?;
        
        if offset + length > self.size {
            return Err(ProcessError::InvalidArguments("Read would exceed memory bounds".to_string()));
        }
        
        Ok(memory[offset..offset + length].to_vec())
    }
}

impl MessageQueue {
    /// Create new message queue
    pub fn create<S: Into<String>>(
        name: S, 
        max_message_size: usize, 
        max_queue_size: usize
    ) -> ProcessResult<Self> {
        let name = name.into();
        let messages = Arc::new(Mutex::new(std::collections::VecDeque::new()));
        
        Ok(MessageQueue {
            name,
            max_message_size,
            max_queue_size,
            messages,
        })
    }
    
    /// Send message to queue
    pub fn send(&self, message: &[u8]) -> ProcessResult<()> {
        if message.len() > self.max_message_size {
            return Err(ProcessError::InvalidArguments("Message too large".to_string()));
        }
        
        let mut messages = self.messages.lock()
            .map_err(|_| ProcessError::CommunicationError("Failed to lock message queue".to_string()))?;
        
        if messages.len() >= self.max_queue_size {
            return Err(ProcessError::ResourceLimitExceeded("Message queue full".to_string()));
        }
        
        messages.push_back(message.to_vec());
        Ok(())
    }
    
    /// Receive message from queue (blocking)
    pub fn receive(&self) -> ProcessResult<Vec<u8>> {
        // This is a simplified implementation
        // In practice, you'd use proper blocking mechanisms
        loop {
            {
                let mut messages = self.messages.lock()
                    .map_err(|_| ProcessError::CommunicationError("Failed to lock message queue".to_string()))?;
                
                if let Some(message) = messages.pop_front() {
                    return Ok(message);
                }
            }
            
            // Brief sleep to avoid busy waiting
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    
    /// Try to receive message from queue (non-blocking)
    pub fn try_receive(&self) -> ProcessResult<Option<Vec<u8>>> {
        let mut messages = self.messages.lock()
            .map_err(|_| ProcessError::CommunicationError("Failed to lock message queue".to_string()))?;
        
        Ok(messages.pop_front())
    }
    
    /// Get queue length
    pub fn len(&self) -> ProcessResult<usize> {
        let messages = self.messages.lock()
            .map_err(|_| ProcessError::CommunicationError("Failed to lock message queue".to_string()))?;
        
        Ok(messages.len())
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> ProcessResult<bool> {
        Ok(self.len()? == 0)
    }
}

/// Create process communication with pipes
pub fn create_process_communication(mut child: Child) -> ProcessResult<ProcessCommunication> {
    ProcessCommunication::new(child)
}

/// Create anonymous pipe pair
pub fn create_pipe() -> ProcessResult<(std::process::ChildStdin, std::process::ChildStdout)> {
    // This is a simplified approach - in practice you'd create actual pipes
    use std::process::{Command, Stdio};
    
    #[cfg(unix)]
    let mut cmd = Command::new("cat");
    
    #[cfg(windows)]
    let mut cmd = Command::new("type");
    
    cmd.stdin(Stdio::piped())
       .stdout(Stdio::piped());
    
    let mut child = cmd.spawn()
        .map_err(|e| ProcessError::CommunicationError(format!("Failed to create pipe: {}", e)))?;
    
    let stdin = child.stdin.take()
        .ok_or_else(|| ProcessError::CommunicationError("Failed to get stdin pipe".to_string()))?;
    let stdout = child.stdout.take()
        .ok_or_else(|| ProcessError::CommunicationError("Failed to get stdout pipe".to_string()))?;
    
    // We need to handle the child process cleanup elsewhere
    // This is a limitation of this simplified implementation
    
    Ok((stdin, stdout))
}

/// Execute command with bidirectional communication
pub fn execute_with_communication<S: AsRef<str>>(
    command: S,
    config: CommunicationConfig,
) -> ProcessResult<ProcessCommunication> {
    let command_str = command.as_ref();
    
    #[cfg(windows)]
    let mut cmd = std::process::Command::new("cmd");
    #[cfg(windows)]
    cmd.args(&["/C", command_str]);
    
    #[cfg(not(windows))]
    let mut cmd = std::process::Command::new("sh");
    #[cfg(not(windows))]
    cmd.args(&["-c", command_str]);
    
    cmd.stdin(Stdio::piped())
       .stdout(Stdio::piped())
       .stderr(Stdio::piped());
    
    let child = cmd.spawn()
        .map_err(|e| ProcessError::ExecutionFailed(format!("Failed to spawn command: {}", e)))?;
    
    ProcessCommunication::new(child)
}

/// Send data to process via stdin and read response
pub fn send_and_receive<S: AsRef<str>>(
    command: S,
    input: &[u8],
    timeout: Duration,
) -> ProcessResult<(Vec<u8>, Vec<u8>)> {
    let mut comm = execute_with_communication(command, CommunicationConfig::default())?;
    
    // Start readers
    comm.start_readers()?;
    
    // Send input
    comm.write_stdin(input)?;
    
    // Close stdin to signal end of input
    comm.channels.stdin = None;
    
    // Collect output
    let mut stdout_lines = Vec::new();
    let mut stderr_lines = Vec::new();
    
    let start_time = std::time::Instant::now();
    
    while start_time.elapsed() < timeout {
        if let Ok(Some(line)) = comm.read_stdout_line() {
            stdout_lines.push(line);
        }
        if let Ok(Some(line)) = comm.read_stderr_line() {
            stderr_lines.push(line);
        }
        
        // Check if process finished
        if let Ok(false) = comm.is_running() {
            break;
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
    
    let stdout = stdout_lines.join("\n").into_bytes();
    let stderr = stderr_lines.join("\n").into_bytes();
    
    Ok((stdout, stderr))
}

/// Create daemon process (Unix only)
#[cfg(unix)]
pub fn create_daemon<F>(daemon_fn: F) -> ProcessResult<u32>
where
    F: FnOnce() -> ProcessResult<()> + Send + 'static,
{
    use std::os::unix::process::CommandExt;
    
    // Fork the process
    let pid = unsafe { libc::fork() };
    
    match pid {
        -1 => {
            return Err(ProcessError::SystemError(
                std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                "Failed to fork process".to_string()
            ));
        }
        0 => {
            // Child process - become daemon
            
            // Create new session
            unsafe { libc::setsid() };
            
            // Change working directory to root
            unsafe { libc::chdir(b"/\0".as_ptr() as *const i8) };
            
            // Close standard file descriptors
            unsafe {
                libc::close(0); // stdin
                libc::close(1); // stdout
                libc::close(2); // stderr
            }
            
            // Run daemon function
            if let Err(e) = daemon_fn() {
                eprintln!("Daemon error: {}", e);
                std::process::exit(1);
            }
            
            std::process::exit(0);
        }
        child_pid => {
            // Parent process - return child PID
            Ok(child_pid as u32)
        }
    }
}

/// Monitor process output in real-time
pub fn monitor_process_output<F>(
    mut communication: ProcessCommunication,
    mut output_handler: F,
) -> ProcessResult<ExitStatus>
where
    F: FnMut(&str, bool) -> bool, // (line, is_stderr) -> continue
{
    communication.start_readers()?;
    
    loop {
        // Check for stdout
        if let Ok(Some(line)) = communication.read_stdout_line() {
            if !output_handler(&line, false) {
                break;
            }
        }
        
        // Check for stderr
        if let Ok(Some(line)) = communication.read_stderr_line() {
            if !output_handler(&line, true) {
                break;
            }
        }
        
        // Check if process finished
        if let Ok(false) = communication.is_running() {
            break;
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
    
    communication.wait()
}
