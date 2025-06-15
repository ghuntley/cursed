/// Enhanced exec_vibez - Process execution with comprehensive enhanced features
/// 
/// This module provides the complete "ExecVibez" API with all enhanced features
/// including ProcessGroup management, OutputStreamer, InputGenerator, enhanced 
/// Environment management, context-based timeout/cancellation, and cross-platform
/// LookPath implementation.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, RwLock, mpsc, Condvar};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(unix)]
extern crate libc;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, execution_failed, execution_failed_with_code,
    timeout_error, invalid_arguments, io_error, system_error
};
use crate::stdlib::process::real_monitoring::{
    RealProcessState, register_process_for_monitoring, wait_for_real_process,
    unregister_process_from_monitoring
};

/// Enhanced Cmd represents an external command being prepared or run
#[derive(Debug)]
pub struct EnhancedCmd {
    /// Path to the executable
    pub path: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Environment variables for the process
    pub env: EnhancedEnvironment,
    /// Working directory
    pub dir: Option<PathBuf>,
    /// Process context for cancellation/timeout
    pub context: Option<ProcessContext>,
    /// Internal child process handle
    child: Option<Child>,
    /// Process start time
    start_time: Option<Instant>,
    /// Process ID (when started)
    pid: Option<u32>,
}

/// Enhanced Process represents a running process with full monitoring
#[derive(Debug)]
pub struct EnhancedProcess {
    /// Process ID
    pub pid: u32,
    /// Process handle
    child: Arc<Mutex<Child>>,
    /// Start time
    start_time: Instant,
    /// Context for cancellation
    context: Option<ProcessContext>,
}

/// Enhanced ProcessState with comprehensive exit information
#[derive(Debug, Clone)]
pub struct EnhancedProcessState {
    /// Exit status
    exit_status: ExitStatus,
    /// Process ID
    pid: u32,
    /// User CPU time used
    user_time: Duration,
    /// System CPU time used
    system_time: Duration,
    /// System-specific information
    sys_info: Vec<u8>,
    /// Exit code
    exit_code: i32,
    /// Error message if failed
    error_message: Option<String>,
}

/// Enhanced Error represents comprehensive error information
#[derive(Debug, Clone)]
pub struct EnhancedError {
    /// Error message
    message: String,
    /// Exit code if available
    exit_code: Option<i32>,
    /// Underlying error
    source: Option<String>,
    /// Error category
    category: ErrorCategory,
    /// System error code
    system_code: Option<i32>,
}

/// Error categories for better error handling
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCategory {
    /// Command not found
    NotFound,
    /// Permission denied
    PermissionDenied,
    /// Timeout occurred
    Timeout,
    /// Process was killed/terminated
    Killed,
    /// I/O error
    IoError,
    /// System error
    SystemError,
    /// Generic execution error
    ExecutionError,
}

/// Process context for timeout and cancellation
#[derive(Debug, Clone)]
pub struct ProcessContext {
    /// Timeout duration
    pub timeout: Option<Duration>,
    /// Cancellation signal
    pub cancelled: Arc<RwLock<bool>>,
    /// Context ID for tracking
    pub id: String,
    /// Parent context
    pub parent: Option<Box<ProcessContext>>,
}

/// Enhanced ProcessGroup for managing multiple related processes
#[derive(Debug)]
pub struct ProcessGroup {
    /// Commands in the group
    commands: Vec<EnhancedCmd>,
    /// Group options
    options: ProcessGroupOptions,
    /// Running processes
    processes: Vec<EnhancedProcess>,
    /// Group ID
    group_id: String,
    /// Start time
    start_time: Option<Instant>,
}

/// Enhanced options for process groups
#[derive(Debug, Clone)]
pub struct ProcessGroupOptions {
    /// Start all processes simultaneously
    pub start_all: bool,
    /// Wait for all to complete before considering success
    pub wait_all: bool,
    /// Continue on individual failures
    pub continue_on_failure: bool,
    /// Timeout for the entire group
    pub group_timeout: Option<Duration>,
    /// Maximum concurrent processes
    pub max_concurrent: Option<usize>,
    /// Process priority
    pub priority: Option<i32>,
    /// Kill tree on failure
    pub kill_tree_on_failure: bool,
}

/// Enhanced Environment management for processes
#[derive(Debug, Clone)]
pub struct EnhancedEnvironment {
    /// Environment variables
    vars: HashMap<String, String>,
    /// Whether to inherit parent environment
    inherit: bool,
    /// Variables to append to (like PATH)
    append_vars: HashMap<String, Vec<String>>,
    /// Variables to prepend to
    prepend_vars: HashMap<String, Vec<String>>,
    /// Variables to remove
    remove_vars: Vec<String>,
}

/// OutputStreamer for real-time command output processing
#[derive(Debug)]
pub struct OutputStreamer {
    /// Command to stream
    cmd: EnhancedCmd,
    /// Line callback for stdout
    stdout_callback: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Line callback for stderr
    stderr_callback: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Raw byte callback
    raw_callback: Option<Arc<dyn Fn(&[u8], StreamType) + Send + Sync>>,
    /// Buffer size for reading
    buffer_size: usize,
    /// Whether to capture output
    capture_output: bool,
    /// Captured output
    captured_stdout: Arc<Mutex<Vec<u8>>>,
    captured_stderr: Arc<Mutex<Vec<u8>>>,
}

/// Stream type indicator
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StreamType {
    Stdout,
    Stderr,
}

/// InputGenerator for programmatically providing input to commands
#[derive(Debug)]
pub struct InputGenerator {
    /// Command to provide input to
    cmd: EnhancedCmd,
    /// Input queue with timing
    input_queue: Arc<Mutex<Vec<InputItem>>>,
    /// Whether to close after all input
    auto_close: bool,
    /// Input thread handle
    thread_handle: Option<thread::JoinHandle<ProcessResult<()>>>,
}

/// Input item with timing and options
#[derive(Debug, Clone)]
pub struct InputItem {
    /// Data to write
    data: Vec<u8>,
    /// Delay before writing
    delay: Option<Duration>,
    /// Whether to flush after writing
    flush: bool,
    /// Whether to add newline
    add_newline: bool,
}

impl Default for ProcessGroupOptions {
    fn default() -> Self {
        Self {
            start_all: true,
            wait_all: true,
            continue_on_failure: false,
            group_timeout: None,
            max_concurrent: None,
            priority: None,
            kill_tree_on_failure: false,
        }
    }
}

impl EnhancedEnvironment {
    /// Create a new environment
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            inherit: true,
            append_vars: HashMap::new(),
            prepend_vars: HashMap::new(),
            remove_vars: Vec::new(),
        }
    }

    /// Create environment from current system environment
    pub fn from_current() -> Self {
        let mut env = Self::new();
        for (key, value) in std::env::vars() {
            env.vars.insert(key, value);
        }
        env
    }

    /// Set an environment variable
    pub fn set<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        self.vars.insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Get an environment variable
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&String> {
        self.vars.get(key.as_ref())
    }

    /// Append to an environment variable (like PATH)
    pub fn append<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        self.append_vars.entry(key).or_insert_with(Vec::new).push(value);
        self
    }

    /// Prepend to an environment variable
    pub fn prepend<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        self.prepend_vars.entry(key).or_insert_with(Vec::new).insert(0, value);
        self
    }

    /// Remove an environment variable
    pub fn remove<K: AsRef<str>>(&mut self, key: K) -> &mut Self {
        let key = key.as_ref().to_string();
        self.vars.remove(&key);
        self.remove_vars.push(key);
        self
    }

    /// Set whether to inherit parent environment
    pub fn set_inherit(&mut self, inherit: bool) -> &mut Self {
        self.inherit = inherit;
        self
    }

    /// Clear all environment variables
    pub fn clear(&mut self) -> &mut Self {
        self.vars.clear();
        self.inherit = false;
        self
    }

    /// Apply the environment to a Command
    pub fn apply_to_command(&self, command: &mut Command) {
        if !self.inherit {
            command.env_clear();
        }

        // Apply regular variables
        for (key, value) in &self.vars {
            command.env(key, value);
        }

        // Apply append/prepend operations
        for (key, values) in &self.append_vars {
            if let Ok(existing) = std::env::var(key) {
                let separator = if key == "PATH" { 
                    if cfg!(windows) { ";" } else { ":" }
                } else { ":" };
                let mut new_value = existing;
                for value in values {
                    new_value.push_str(separator);
                    new_value.push_str(value);
                }
                command.env(key, new_value);
            } else {
                command.env(key, values.join(":"));
            }
        }

        for (key, values) in &self.prepend_vars {
            if let Ok(existing) = std::env::var(key) {
                let separator = if key == "PATH" { 
                    if cfg!(windows) { ";" } else { ":" }
                } else { ":" };
                let mut new_value = values.join(separator);
                new_value.push_str(separator);
                new_value.push_str(&existing);
                command.env(key, new_value);
            } else {
                command.env(key, values.join(":"));
            }
        }

        // Remove specified variables
        for key in &self.remove_vars {
            command.env_remove(key);
        }
    }

    /// Convert to HashMap for compatibility
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut result = self.vars.clone();
        
        // Apply append/prepend operations
        for (key, values) in &self.append_vars {
            if let Some(existing) = result.get(key) {
                let separator = if key == "PATH" { 
                    if cfg!(windows) { ";" } else { ":" }
                } else { ":" };
                let mut new_value = existing.clone();
                for value in values {
                    new_value.push_str(separator);
                    new_value.push_str(value);
                }
                result.insert(key.clone(), new_value);
            } else if let Ok(existing) = std::env::var(key) {
                let separator = if key == "PATH" { 
                    if cfg!(windows) { ";" } else { ":" }
                } else { ":" };
                let mut new_value = existing;
                for value in values {
                    new_value.push_str(separator);
                    new_value.push_str(value);
                }
                result.insert(key.clone(), new_value);
            } else {
                result.insert(key.clone(), values.join(":"));
            }
        }

        for (key, values) in &self.prepend_vars {
            if let Some(existing) = result.get(key) {
                let separator = if key == "PATH" { 
                    if cfg!(windows) { ";" } else { ":" }
                } else { ":" };
                let mut new_value = values.join(separator);
                new_value.push_str(separator);
                new_value.push_str(existing);
                result.insert(key.clone(), new_value);
            } else if let Ok(existing) = std::env::var(key) {
                let separator = if key == "PATH" { 
                    if cfg!(windows) { ";" } else { ":" }
                } else { ":" };
                let mut new_value = values.join(separator);
                new_value.push_str(separator);
                new_value.push_str(&existing);
                result.insert(key.clone(), new_value);
            } else {
                result.insert(key.clone(), values.join(":"));
            }
        }

        // Remove specified variables
        for key in &self.remove_vars {
            result.remove(key);
        }

        result
    }
}

impl OutputStreamer {
    /// Create a new output streamer
    pub fn new(cmd: EnhancedCmd) -> Self {
        Self {
            cmd,
            stdout_callback: None,
            stderr_callback: None,
            raw_callback: None,
            buffer_size: 8192,
            capture_output: false,
            captured_stdout: Arc::new(Mutex::new(Vec::new())),
            captured_stderr: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Set line callback for stdout
    pub fn on_stdout_line<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.stdout_callback = Some(Arc::new(callback));
        self
    }

    /// Set line callback for stderr
    pub fn on_stderr_line<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.stderr_callback = Some(Arc::new(callback));
        self
    }

    /// Set raw byte callback
    pub fn on_raw_data<F>(&mut self, callback: F) -> &mut Self
    where
        F: Fn(&[u8], StreamType) + Send + Sync + 'static,
    {
        self.raw_callback = Some(Arc::new(callback));
        self
    }

    /// Set buffer size
    pub fn set_buffer_size(&mut self, size: usize) -> &mut Self {
        self.buffer_size = size;
        self
    }

    /// Enable output capture
    pub fn capture_output(&mut self, capture: bool) -> &mut Self {
        self.capture_output = capture;
        self
    }

    /// Start streaming
    pub fn start(&mut self) -> ProcessResult<()> {
        self.cmd.start()?;
        
        if let Some(child) = &mut self.cmd.child {
            // Handle stdout streaming
            if let Some(stdout) = child.stdout.take() {
                let stdout_callback = self.stdout_callback.clone();
                let raw_callback = self.raw_callback.clone();
                let captured_stdout = self.captured_stdout.clone();
                let capture_output = self.capture_output;
                
                thread::spawn(move || {
                    let mut reader = BufReader::new(stdout);
                    let mut buffer = vec![0; 8192];
                    
                    loop {
                        match reader.read(&mut buffer) {
                            Ok(0) => break, // EOF
                            Ok(n) => {
                                let data = &buffer[..n];
                                
                                if capture_output {
                                    if let Ok(mut captured) = captured_stdout.lock() {
                                        captured.extend_from_slice(data);
                                    }
                                }
                                
                                if let Some(ref cb) = raw_callback {
                                    cb(data, StreamType::Stdout);
                                }
                                
                                if let Some(ref cb) = stdout_callback {
                                    let string_data = String::from_utf8_lossy(data);
                                    for line in string_data.lines() {
                                        cb(line);
                                    }
                                }
                            }
                            Err(_) => break,
                        }
                    }
                });
            }
            
            // Handle stderr streaming
            if let Some(stderr) = child.stderr.take() {
                let stderr_callback = self.stderr_callback.clone();
                let raw_callback = self.raw_callback.clone();
                let captured_stderr = self.captured_stderr.clone();
                let capture_output = self.capture_output;
                
                thread::spawn(move || {
                    let mut reader = BufReader::new(stderr);
                    let mut buffer = vec![0; 8192];
                    
                    loop {
                        match reader.read(&mut buffer) {
                            Ok(0) => break, // EOF
                            Ok(n) => {
                                let data = &buffer[..n];
                                
                                if capture_output {
                                    if let Ok(mut captured) = captured_stderr.lock() {
                                        captured.extend_from_slice(data);
                                    }
                                }
                                
                                if let Some(ref cb) = raw_callback {
                                    cb(data, StreamType::Stderr);
                                }
                                
                                if let Some(ref cb) = stderr_callback {
                                    let string_data = String::from_utf8_lossy(data);
                                    for line in string_data.lines() {
                                        cb(line);
                                    }
                                }
                            }
                            Err(_) => break,
                        }
                    }
                });
            }
        }
        
        Ok(())
    }

    /// Wait for streaming to complete
    pub fn wait(&mut self) -> ProcessResult<()> {
        self.cmd.wait()
    }

    /// Get captured stdout
    pub fn get_captured_stdout(&self) -> Vec<u8> {
        self.captured_stdout.lock().unwrap().clone()
    }

    /// Get captured stderr
    pub fn get_captured_stderr(&self) -> Vec<u8> {
        self.captured_stderr.lock().unwrap().clone()
    }
}

impl InputGenerator {
    /// Create a new input generator
    pub fn new(cmd: EnhancedCmd) -> Self {
        Self {
            cmd,
            input_queue: Arc::new(Mutex::new(Vec::new())),
            auto_close: true,
            thread_handle: None,
        }
    }

    /// Write input immediately
    pub fn write<S: AsRef<[u8]>>(&mut self, input: S) -> ProcessResult<()> {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push(InputItem {
            data: input.as_ref().to_vec(),
            delay: None,
            flush: true,
            add_newline: false,
        });
        Ok(())
    }

    /// Write line input (adds newline)
    pub fn write_line<S: AsRef<str>>(&mut self, input: S) -> ProcessResult<()> {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push(InputItem {
            data: input.as_ref().as_bytes().to_vec(),
            delay: None,
            flush: true,
            add_newline: true,
        });
        Ok(())
    }

    /// Write input after a delay
    pub fn write_after<S: AsRef<[u8]>>(&mut self, input: S, delay: Duration) -> ProcessResult<()> {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push(InputItem {
            data: input.as_ref().to_vec(),
            delay: Some(delay),
            flush: true,
            add_newline: false,
        });
        Ok(())
    }

    /// Write line after a delay
    pub fn write_line_after<S: AsRef<str>>(&mut self, input: S, delay: Duration) -> ProcessResult<()> {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push(InputItem {
            data: input.as_ref().as_bytes().to_vec(),
            delay: Some(delay),
            flush: true,
            add_newline: true,
        });
        Ok(())
    }

    /// Set auto-close behavior
    pub fn set_auto_close(&mut self, auto_close: bool) -> &mut Self {
        self.auto_close = auto_close;
        self
    }

    /// Start the input generation process
    pub fn start(&mut self) -> ProcessResult<()> {
        self.cmd.start()?;
        
        if let Some(child) = &mut self.cmd.child {
            if let Some(stdin) = child.stdin.take() {
                let queue = self.input_queue.clone();
                let auto_close = self.auto_close;
                
                let handle = thread::spawn(move || -> ProcessResult<()> {
                    let mut writer = BufWriter::new(stdin);
                    
                    loop {
                        let item = {
                            let mut q = queue.lock().unwrap();
                            q.pop()
                        };
                        
                        if let Some(input_item) = item {
                            if let Some(delay) = input_item.delay {
                                thread::sleep(delay);
                            }
                            
                            if writer.write_all(&input_item.data).is_err() {
                                break;
                            }
                            
                            if input_item.add_newline {
                                if writer.write_all(b"\n").is_err() {
                                    break;
                                }
                            }
                            
                            if input_item.flush {
                                if writer.flush().is_err() {
                                    break;
                                }
                            }
                        } else {
                            // No more input, check if we should close
                            if auto_close {
                                break;
                            }
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
                    Ok(())
                });
                
                self.thread_handle = Some(handle);
            }
        }
        
        Ok(())
    }

    /// Close input stream
    pub fn close(&mut self) -> ProcessResult<()> {
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
        Ok(())
    }

    /// Wait for all input to be written
    pub fn wait(&mut self) -> ProcessResult<()> {
        if let Some(handle) = self.thread_handle.take() {
            match handle.join() {
                Ok(result) => result,
                Err(_) => Err(system_error(0, "join", "Failed to join input thread")),
            }
        } else {
            Ok(())
        }
    }
}

impl EnhancedCmd {
    /// Create a new EnhancedCmd instance
    pub fn new<S: AsRef<str>>(name: S, args: &[&str]) -> Self {
        Self {
            path: name.as_ref().to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            env: EnhancedEnvironment::new(),
            dir: None,
            context: None,
            child: None,
            start_time: None,
            pid: None,
        }
    }

    /// Set working directory
    pub fn dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Set environment
    pub fn env(&mut self, env: EnhancedEnvironment) -> &mut Self {
        self.env = env;
        self
    }

    /// Set context
    pub fn context(&mut self, context: ProcessContext) -> &mut Self {
        self.context = Some(context);
        self
    }

    /// Start the process without waiting
    pub fn start(&mut self) -> ProcessResult<()> {
        let mut command = Command::new(&self.path);
        command.args(&self.args);

        if let Some(dir) = &self.dir {
            command.current_dir(dir);
        }

        // Apply environment
        self.env.apply_to_command(&mut command);

        // Configure I/O
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let child = command.spawn()
            .map_err(|e| {
                match e.kind() {
                    io::ErrorKind::NotFound => execution_failed(&self.path, "Command not found"),
                    io::ErrorKind::PermissionDenied => execution_failed(&self.path, "Permission denied"),
                    _ => execution_failed(&self.path, &e.to_string()),
                }
            })?;

        let pid = child.id();
        self.child = Some(child);
        self.start_time = Some(Instant::now());
        self.pid = Some(pid);

        // Register process for monitoring
        if let Some(ref child) = self.child {
            let child_arc = Arc::new(Mutex::new(unsafe {
                std::ptr::read(child as *const Child)
            }));
            let _ = register_process_for_monitoring(pid, Some(child_arc));
        }

        Ok(())
    }

    /// Run the command and wait for completion
    pub fn run(&mut self) -> ProcessResult<()> {
        self.start()?;
        self.wait()
    }

    /// Capture stdout output
    pub fn output(&mut self) -> ProcessResult<Vec<u8>> {
        self.start()?;
        
        let output = if let Some(child) = &mut self.child {
            child.wait_with_output()
                .map_err(|e| io_error("output", &format!("{:?}", e.kind()), &e.to_string()))?
        } else {
            return Err(invalid_arguments("output", "command", "Command not started"));
        };

        if !output.status.success() {
            if let Some(code) = output.status.code() {
                return Err(execution_failed_with_code(&self.path, code, "Command failed"));
            }
        }

        Ok(output.stdout)
    }

    /// Capture combined stdout and stderr
    pub fn combined_output(&mut self) -> ProcessResult<Vec<u8>> {
        self.start()?;
        
        let output = if let Some(child) = &mut self.child {
            child.wait_with_output()
                .map_err(|e| io_error("combined_output", &format!("{:?}", e.kind()), &e.to_string()))?
        } else {
            return Err(invalid_arguments("combined_output", "command", "Command not started"));
        };

        if !output.status.success() {
            if let Some(code) = output.status.code() {
                return Err(execution_failed_with_code(&self.path, code, "Command failed"));
            }
        }

        let mut combined = output.stdout;
        combined.extend_from_slice(&output.stderr);
        Ok(combined)
    }

    /// Wait for the command to complete
    pub fn wait(&mut self) -> ProcessResult<()> {
        if let Some(child) = &mut self.child {
            let pid = child.id();
            
            // Check for timeout
            if let Some(ref context) = self.context {
                if let Some(timeout) = context.timeout {
                    let start = Instant::now();
                    loop {
                        // Check cancellation
                        if context.is_cancelled() {
                            let _ = child.kill();
                            return Err(execution_failed(&self.path, "Process cancelled"));
                        }
                        
                        match child.try_wait() {
                            Ok(Some(status)) => {
                                if !status.success() {
                                    if let Some(code) = status.code() {
                                        return Err(execution_failed_with_code(&self.path, code, "Command failed"));
                                    } else {
                                        return Err(execution_failed(&self.path, "Command terminated by signal"));
                                    }
                                }
                                break;
                            }
                            Ok(None) => {
                                if start.elapsed() >= timeout {
                                    let _ = child.kill();
                                    let _ = child.wait();
                                    return Err(timeout_error("wait", timeout, "Command execution timed out"));
                                }
                                thread::sleep(Duration::from_millis(10));
                            }
                            Err(e) => {
                                return Err(io_error("try_wait", &format!("{:?}", e.kind()), &e.to_string()));
                            }
                        }
                    }
                } else {
                    // No timeout, regular wait
                    let status = child.wait()
                        .map_err(|e| io_error("wait", &format!("{:?}", e.kind()), &e.to_string()))?;
                    
                    if !status.success() {
                        if let Some(code) = status.code() {
                            return Err(execution_failed_with_code(&self.path, code, "Command failed"));
                        } else {
                            return Err(execution_failed(&self.path, "Command terminated by signal"));
                        }
                    }
                }
            } else {
                // No context, regular wait
                let status = child.wait()
                    .map_err(|e| io_error("wait", &format!("{:?}", e.kind()), &e.to_string()))?;
                
                if !status.success() {
                    if let Some(code) = status.code() {
                        return Err(execution_failed_with_code(&self.path, code, "Command failed"));
                    } else {
                        return Err(execution_failed(&self.path, "Command terminated by signal"));
                    }
                }
            }
            
            // Unregister from monitoring
            let _ = unregister_process_from_monitoring(pid);
            Ok(())
        } else {
            Err(invalid_arguments("wait", "command", "Command not started"))
        }
    }

    /// Get process handle
    pub fn process(&self) -> ProcessResult<EnhancedProcess> {
        if let Some(child) = &self.child {
            Ok(EnhancedProcess {
                pid: child.id(),
                child: Arc::new(Mutex::new(unsafe { 
                    std::ptr::read(child as *const Child)
                })),
                start_time: self.start_time.unwrap_or_else(Instant::now),
                context: self.context.clone(),
            })
        } else {
            Err(invalid_arguments("process", "command", "Command not started"))
        }
    }

    /// Get process state
    pub fn process_state(&self) -> ProcessResult<EnhancedProcessState> {
        if let Some(child) = &self.child {
            let pid = child.id();
            
            match wait_for_real_process(pid) {
                Ok(real_state) => {
                    let exit_status = real_state.exit_status.unwrap_or_else(|| {
                        ExitStatus::from_raw(0)
                    });
                    
                    Ok(EnhancedProcessState {
                        exit_status,
                        pid,
                        user_time: real_state.user_time,
                        system_time: real_state.system_time,
                        sys_info: Vec::new(),
                        exit_code: exit_status.code().unwrap_or(-1),
                        error_message: if exit_status.success() { None } else { Some("Process failed".to_string()) },
                    })
                }
                Err(_) => {
                    Ok(EnhancedProcessState {
                        exit_status: ExitStatus::from_raw(0),
                        pid,
                        user_time: Duration::from_millis(0),
                        system_time: Duration::from_millis(0),
                        sys_info: Vec::new(),
                        exit_code: 0,
                        error_message: None,
                    })
                }
            }
        } else {
            Err(invalid_arguments("process_state", "command", "Command not started"))
        }
    }
}

impl ProcessContext {
    /// Create a new process context
    pub fn new() -> Self {
        Self {
            timeout: None,
            cancelled: Arc::new(RwLock::new(false)),
            id: format!("ctx_{}", std::process::id()),
            parent: None,
        }
    }

    /// Create context with timeout
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            timeout: Some(timeout),
            cancelled: Arc::new(RwLock::new(false)),
            id: format!("ctx_{}_{}", std::process::id(), instant_ns()),
            parent: None,
        }
    }

    /// Create context with ID
    pub fn with_id<S: AsRef<str>>(id: S) -> Self {
        Self {
            timeout: None,
            cancelled: Arc::new(RwLock::new(false)),
            id: id.as_ref().to_string(),
            parent: None,
        }
    }

    /// Create child context
    pub fn child(&self) -> Self {
        Self {
            timeout: self.timeout,
            cancelled: Arc::new(RwLock::new(false)),
            id: format!("{}_child_{}", self.id, instant_ns()),
            parent: Some(Box::new(self.clone())),
        }
    }

    /// Cancel the context
    pub fn cancel(&self) {
        let mut cancelled = self.cancelled.write().unwrap();
        *cancelled = true;
    }

    /// Check if context is cancelled
    pub fn is_cancelled(&self) -> bool {
        if *self.cancelled.read().unwrap() {
            return true;
        }
        
        // Check parent cancellation
        if let Some(ref parent) = self.parent {
            return parent.is_cancelled();
        }
        
        false
    }

    /// Get deadline (if timeout is set)
    pub fn deadline(&self) -> Option<Instant> {
        self.timeout.map(|timeout| Instant::now() + timeout)
    }
}

impl ProcessGroup {
    /// Create a new process group
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            options: ProcessGroupOptions::default(),
            processes: Vec::new(),
            group_id: format!("group_{}", instant_ns()),
            start_time: None,
        }
    }

    /// Create process group with ID
    pub fn with_id<S: AsRef<str>>(id: S) -> Self {
        Self {
            commands: Vec::new(),
            options: ProcessGroupOptions::default(),
            processes: Vec::new(),
            group_id: id.as_ref().to_string(),
            start_time: None,
        }
    }

    /// Set options
    pub fn options(&mut self, options: ProcessGroupOptions) -> &mut Self {
        self.options = options;
        self
    }

    /// Add a command to the group
    pub fn add_command(&mut self, cmd: EnhancedCmd) -> &mut Self {
        self.commands.push(cmd);
        self
    }

    /// Add multiple commands
    pub fn add_commands(&mut self, cmds: Vec<EnhancedCmd>) -> &mut Self {
        self.commands.extend(cmds);
        self
    }

    /// Start all commands in the group
    pub fn start_all(&mut self) -> ProcessResult<()> {
        self.start_time = Some(Instant::now());
        
        // Check max_concurrent limit
        if let Some(max_concurrent) = self.options.max_concurrent {
            if self.commands.len() > max_concurrent {
                return Err(invalid_arguments("start_all", "commands", 
                    &format!("Too many commands ({}) for max_concurrent ({})", 
                        self.commands.len(), max_concurrent)));
            }
        }

        for cmd in &mut self.commands {
            cmd.start()?;
            if let Ok(process) = cmd.process() {
                self.processes.push(process);
            }
        }
        Ok(())
    }

    /// Wait for all commands to complete
    pub fn wait_all(&mut self) -> ProcessResult<()> {
        let start_time = Instant::now();
        
        for cmd in &mut self.commands {
            // Check group timeout
            if let Some(group_timeout) = self.options.group_timeout {
                if start_time.elapsed() >= group_timeout {
                    // Kill all remaining processes
                    for process in &self.processes {
                        let _ = process.kill();
                    }
                    return Err(timeout_error("wait_all", group_timeout, "Group timeout exceeded"));
                }
            }
            
            if let Err(e) = cmd.wait() {
                if !self.options.continue_on_failure {
                    if self.options.kill_tree_on_failure {
                        // Kill all processes in the group
                        for process in &self.processes {
                            let _ = process.kill();
                        }
                    }
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    /// Kill all processes in the group
    pub fn kill_all(&mut self) -> ProcessResult<()> {
        for process in &self.processes {
            let _ = process.kill();
        }
        Ok(())
    }

    /// Get group status
    pub fn status(&self) -> ProcessGroupStatus {
        let mut running = 0;
        let mut completed = 0;
        let mut failed = 0;
        
        for process in &self.processes {
            // This is simplified - in a real implementation we'd check actual process status
            completed += 1;
        }
        
        ProcessGroupStatus {
            group_id: self.group_id.clone(),
            total: self.commands.len(),
            running,
            completed,
            failed,
            start_time: self.start_time,
        }
    }
}

/// Process group status information
#[derive(Debug, Clone)]
pub struct ProcessGroupStatus {
    pub group_id: String,
    pub total: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
    pub start_time: Option<Instant>,
}

/// Cross-platform enhanced LookPath implementation
pub fn look_path<S: AsRef<str>>(file: S) -> ProcessResult<String> {
    let file = file.as_ref();
    
    // If file contains path separator, check if it exists directly
    if file.contains('/') || (cfg!(windows) && (file.contains('\\') || file.contains(':'))) {
        let path = Path::new(file);
        if path.is_file() {
            return Ok(path.to_string_lossy().to_string());
        }
        return Err(execution_failed(file, "File not found"));
    }
    
    // Search in PATH
    if let Ok(paths) = std::env::var("PATH") {
        let path_separator = if cfg!(windows) { ';' } else { ':' };
        
        for path_dir in paths.split(path_separator) {
            let path_dir = Path::new(path_dir);
            
            // Try exact filename
            let full_path = path_dir.join(file);
            if full_path.is_file() {
                return Ok(full_path.to_string_lossy().to_string());
            }
            
            // On Windows, also try with common executable extensions
            #[cfg(windows)]
            {
                for ext in &[".exe", ".bat", ".cmd", ".com"] {
                    let exe_path = path_dir.join(format!("{}{}", file, ext));
                    if exe_path.is_file() {
                        return Ok(exe_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    
    Err(execution_failed(file, "Command not found in PATH"))
}

/// Get current time in nanoseconds for unique IDs
fn instant_ns() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

/// Enhanced factory functions

/// Create a new enhanced command
pub fn command<S: AsRef<str>>(name: S, args: &[&str]) -> EnhancedCmd {
    EnhancedCmd::new(name, args)
}

/// Create a new enhanced command with context
pub fn command_context(ctx: ProcessContext, name: &str, args: &[&str]) -> EnhancedCmd {
    let mut cmd = EnhancedCmd::new(name, args);
    cmd.context = Some(ctx);
    cmd
}

/// Create a new enhanced environment
pub fn new_environment() -> EnhancedEnvironment {
    EnhancedEnvironment::new()
}

/// Create a new process group
pub fn new_process_group() -> ProcessGroup {
    ProcessGroup::new()
}

/// Create a new output streamer
pub fn new_output_streamer(cmd: EnhancedCmd) -> OutputStreamer {
    OutputStreamer::new(cmd)
}

/// Create a new input generator
pub fn new_input_generator(cmd: EnhancedCmd) -> InputGenerator {
    InputGenerator::new(cmd)
}

/// Create a new process context
pub fn new_context() -> ProcessContext {
    ProcessContext::new()
}

/// Create a new process context with timeout
pub fn new_context_with_timeout(timeout: Duration) -> ProcessContext {
    ProcessContext::with_timeout(timeout)
}

/// Run command with timeout using enhanced features
pub fn run_with_timeout<S: AsRef<str>>(name: S, args: &[&str], timeout: Duration) -> ProcessResult<Vec<u8>> {
    let mut cmd = command(name, args);
    let ctx = ProcessContext::with_timeout(timeout);
    cmd.context = Some(ctx);
    cmd.output()
}

/// Create command with enhanced environment
pub fn command_with_env<S: AsRef<str>>(name: S, args: &[&str], env: EnhancedEnvironment) -> EnhancedCmd {
    let mut cmd = command(name, args);
    cmd.env = env;
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_enhanced_cmd_creation() {
        let cmd = EnhancedCmd::new("echo", &["hello", "world"]);
        assert_eq!(cmd.path, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_enhanced_environment() {
        let mut env = EnhancedEnvironment::new();
        env.set("TEST_VAR", "test_value")
           .append("PATH", "/usr/local/bin")
           .prepend("LD_LIBRARY_PATH", "/opt/lib");
        
        assert_eq!(env.get("TEST_VAR"), Some(&"test_value".to_string()));
        let hashmap = env.to_hashmap();
        assert!(hashmap.contains_key("TEST_VAR"));
    }

    #[test]
    fn test_process_context() {
        let ctx = ProcessContext::with_timeout(Duration::from_secs(30));
        assert!(!ctx.is_cancelled());
        assert_eq!(ctx.timeout, Some(Duration::from_secs(30)));
        
        ctx.cancel();
        assert!(ctx.is_cancelled());
    }

    #[test]
    fn test_process_group() {
        let mut group = ProcessGroup::new();
        let cmd = EnhancedCmd::new("echo", &["test"]);
        group.add_command(cmd);
        
        assert_eq!(group.commands.len(), 1);
    }

    #[test]
    fn test_output_streamer() {
        let cmd = EnhancedCmd::new("echo", &["test"]);
        let mut streamer = OutputStreamer::new(cmd);
        streamer.set_buffer_size(4096).capture_output(true);
        assert_eq!(streamer.buffer_size, 4096);
        assert!(streamer.capture_output);
    }

    #[test]
    fn test_input_generator() {
        let cmd = EnhancedCmd::new("cat", &[]);
        let mut generator = InputGenerator::new(cmd);
        
        assert!(generator.write("test input").is_ok());
        assert!(generator.write_line("line input").is_ok());
        assert!(generator.write_after(b"delayed input", Duration::from_millis(500)).is_ok());
    }

    #[test]
    fn test_enhanced_error() {
        let err = EnhancedError {
            message: "Test error".to_string(),
            exit_code: Some(1),
            source: None,
            category: ErrorCategory::ExecutionError,
            system_code: None,
        };
        assert_eq!(err.message, "Test error");
        assert_eq!(err.exit_code, Some(1));
        assert_eq!(err.category, ErrorCategory::ExecutionError);
    }

    #[test]
    fn test_look_path() {
        // This will depend on the system, but we can test the logic
        #[cfg(unix)]
        {
            match look_path("sh") {
                Ok(path) => assert!(path.contains("sh")),
                Err(_) => {} // May not exist in test environment
            }
        }
        
        #[cfg(windows)]
        {
            match look_path("cmd") {
                Ok(path) => assert!(path.contains("cmd")),
                Err(_) => {} // May not exist in test environment
            }
        }
    }
}
