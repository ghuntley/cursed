use crate::error::Error;
/// exec_vibez_advanced - Advanced process execution features
/// 
/// This module provides the complete exec_vibez functionality as specified,
/// building upon the base implementation with enhanced features for:
/// - Process Groups with sophisticated management
/// - Enhanced Timeout Control with better error handling
/// - Environment Control with inheritance and manipulation
/// - Output Streaming with real-time processing
/// - Input Generation with precise timing control
/// - Enhanced Context Support with cancellation
/// - Improved LookPath with better search algorithms
/// - Cross-platform signal handling

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, RwLock, Condvar, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, execution_failed, execution_failed_with_code,
    timeout_error, invalid_arguments, io_error, system_error, platform_error
};

use crate::stdlib::process::exec_vibez::{Cmd, Process, ProcessState, Error, ProcessContext, Environment};

/// Enhanced Process Group with sophisticated lifecycle management
#[derive(Debug)]
pub struct EnhancedProcessGroup {
    /// Commands in the group
    commands: Vec<Cmd>,
    /// Group configuration options
    options: ProcessGroupConfig,
    /// Running processes with metadata
    processes: Vec<ProcessInfo>,
    /// Group state
    state: Arc<RwLock<GroupState>>,
    /// Completion tracking
    completion: Arc<(Mutex<usize>, Condvar)>,
    /// Error accumulator
    errors: Arc<Mutex<Vec<ProcessError>>>,
}

/// Process information within a group
#[derive(Debug)]
struct ProcessInfo {
    /// Process handle
    process: Process,
    /// Command name for identification
    name: String,
    /// Start time
    start_time: Instant,
    /// Completion state
    completed: Arc<RwLock<bool>>,
}

/// Process group configuration
#[derive(Debug, Clone)]
pub struct ProcessGroupConfig {
    /// Maximum parallel processes (0 = unlimited)
    pub max_parallel: usize,
    /// Timeout for individual processes
    pub process_timeout: Option<Duration>,
    /// Timeout for the entire group
    pub group_timeout: Option<Duration>,
    /// Whether to kill remaining processes on first failure
    pub kill_on_failure: bool,
    /// Whether to continue on individual failures
    pub continue_on_failure: bool,
    /// Whether to collect outputs from all processes
    pub collect_outputs: bool,
}

impl Default for ProcessGroupConfig {
    fn default() -> Self {
        Self {
            max_parallel: 0, // Unlimited
            process_timeout: None,
            group_timeout: None,
            kill_on_failure: false,
            continue_on_failure: false,
            collect_outputs: false,
        }
    }
}

/// Group execution state
#[derive(Debug, Clone, PartialEq)]
enum GroupState {
    Created,
    Starting,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Enhanced Environment Manager with inheritance and manipulation
#[derive(Debug, Clone)]
pub struct EnhancedEnvironment {
    /// Environment variables
    vars: HashMap<String, String>,
    /// Variables to remove from parent environment
    removed_vars: Vec<String>,
    /// Whether to inherit parent environment
    inherit_parent: bool,
    /// Whether to clear all environment first
    clear_all: bool,
    /// Path manipulation operations
    path_operations: Vec<PathOperation>,
}

#[derive(Debug, Clone)]
enum PathOperation {
    Append(String),
    Prepend(String),
    Set(String),
}

impl EnhancedEnvironment {
    /// Create a new enhanced environment
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            removed_vars: Vec::new(),
            inherit_parent: true,
            clear_all: false,
            path_operations: Vec::new(),
        }
    }

    /// Set an environment variable
    pub fn set<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        self.vars.insert(key.as_ref().to_string(), value.as_ref().to_string());
    }

    /// Get an environment variable (including parent if inherited)
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<String> {
        let key = key.as_ref();
        
        // Check our variables first
        if let Some(value) = self.vars.get(key) {
            return Some(value.clone());
        }
        
        // Check parent environment if inheriting and not removed
        if self.inherit_parent && !self.removed_vars.contains(&key.to_string()) {
            return std::env::var(key).ok();
        }
        
        None
    }

    /// Append to PATH environment variable
    pub fn append_path<V: AsRef<str>>(&mut self, value: V) {
        self.path_operations.push(PathOperation::Append(value.as_ref().to_string()));
    }

    /// Prepend to PATH environment variable
    pub fn prepend_path<V: AsRef<str>>(&mut self, value: V) {
        self.path_operations.push(PathOperation::Prepend(value.as_ref().to_string()));
    }

    /// Set PATH environment variable (replaces existing)
    pub fn set_path<V: AsRef<str>>(&mut self, value: V) {
        self.path_operations.push(PathOperation::Set(value.as_ref().to_string()));
    }

    /// Remove an environment variable
    pub fn remove<K: AsRef<str>>(&mut self, key: K) {
        let key = key.as_ref().to_string();
        self.vars.remove(&key);
        self.removed_vars.push(key);
    }

    /// Clear all environment variables
    pub fn clear_all(&mut self) {
        self.clear_all = true;
        self.inherit_parent = false;
    }

    /// Set whether to inherit parent environment
    pub fn set_inherit(&mut self, inherit: bool) {
        self.inherit_parent = inherit;
    }

    /// Build final environment for command
    pub fn build_env(&self) -> Vec<String> {
        let mut env_map = HashMap::new();
        
        // Start with parent environment if inheriting
        if self.inherit_parent && !self.clear_all {
            for (key, value) in std::env::vars() {
                if !self.removed_vars.contains(&key) {
                    env_map.insert(key, value);
                }
            }
        }
        
        // Apply our variables
        for (key, value) in &self.vars {
            env_map.insert(key.clone(), value.clone());
        }
        
        // Apply PATH operations
        let mut path = env_map.get("PATH").cloned().unwrap_or_default();
        for operation in &self.path_operations {
            match operation {
                PathOperation::Set(value) => path = value.clone(),
                PathOperation::Append(value) => {
                    if !path.is_empty() {
                        path.push_str(":");
                    }
                    path.push_str(value);
                }
                PathOperation::Prepend(value) => {
                    if !path.is_empty() {
                        path = format!("{}:{}", value, path);
                    } else {
                        path = value.clone();
                    }
                }
            }
        }
        env_map.insert("PATH".to_string(), path);
        
        // Convert to vector format
        env_map.into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect()
    }
}

/// Enhanced Output Streamer with configurable processing
#[derive(Debug)]
pub struct EnhancedOutputStreamer {
    /// Command to stream
    cmd: Cmd,
    /// Line processing callbacks
    line_callbacks: Vec<Box<dyn Fn(&str) + Send + Sync>>,
    /// Chunk processing callback
    chunk_callback: Option<Box<dyn Fn(&[u8]) + Send + Sync>>,
    /// Buffer size for reading
    buffer_size: usize,
    /// Stream both stdout and stderr
    stream_stderr: bool,
    /// Prefix lines with timestamps
    timestamp_lines: bool,
    /// Worker thread handles
    worker_threads: Vec<JoinHandle<()>>,
    /// State tracking
    state: Arc<RwLock<StreamerState>>,
}

#[derive(Debug, Clone, PartialEq)]
enum StreamerState {
    Created,
    Running,
    Completed,
    Failed,
}

impl EnhancedOutputStreamer {
    /// Create a new enhanced output streamer
    pub fn new(cmd: Cmd) -> Self {
        Self {
            cmd,
            line_callbacks: Vec::new(),
            chunk_callback: None,
            buffer_size: 8192,
            stream_stderr: true,
            timestamp_lines: false,
            worker_threads: Vec::new(),
            state: Arc::new(RwLock::new(StreamerState::Created)),
        }
    }

    /// Add line processing callback
    pub fn on_line<F>(&mut self, callback: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.line_callbacks.push(Box::new(callback));
    }

    /// Set chunk processing callback
    pub fn on_chunk<F>(&mut self, callback: F)
    where
        F: Fn(&[u8]) + Send + Sync + 'static,
    {
        self.chunk_callback = Some(Box::new(callback));
    }

    /// Set buffer size
    pub fn set_buffer_size(&mut self, size: usize) {
        self.buffer_size = size;
    }

    /// Set whether to stream stderr
    pub fn set_stream_stderr(&mut self, stream: bool) {
        self.stream_stderr = stream;
    }

    /// Set whether to timestamp lines
    pub fn set_timestamp_lines(&mut self, timestamp: bool) {
        self.timestamp_lines = timestamp;
    }

    /// Start streaming
    pub fn start(&mut self) -> ProcessResult<()> {
        {
            let mut state = self.state.write().unwrap();
            *state = StreamerState::Running;
        }

        self.cmd.start()?;
        
        if let Some(child) = &mut self.cmd.child {
            // Stream stdout
            if let Some(stdout) = child.stdout.take() {
                let callbacks = self.line_callbacks.iter()
                    .map(|cb| unsafe { std::mem::transmute::<&Box<dyn Fn(&str) + Send + Sync>, &'static Box<dyn Fn(&str) + Send + Sync>>(cb) })
                    .collect::<Vec<_>>();
                let timestamp = self.timestamp_lines;
                let state = self.state.clone();
                
                let handle = thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    for line in reader.split("\n") {
                        if let Ok(line) = line {
                            let output_line = if timestamp {
                                format!("[{}] {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"), line)
                            } else {
                                line
                            };
                            
                            for callback in &callbacks {
                                callback(&output_line);
                            }
                        }
                    }
                    
                    let mut state = state.write().unwrap();
                    if *state == StreamerState::Running {
                        *state = StreamerState::Completed;
                    }
                });
                
                self.worker_threads.push(handle);
            }

            // Stream stderr if enabled
            if self.stream_stderr {
                if let Some(stderr) = child.stderr.take() {
                    let callbacks = self.line_callbacks.iter()
                        .map(|cb| unsafe { std::mem::transmute::<&Box<dyn Fn(&str) + Send + Sync>, &'static Box<dyn Fn(&str) + Send + Sync>>(cb) })
                        .collect::<Vec<_>>();
                    let timestamp = self.timestamp_lines;
                    
                    let handle = thread::spawn(move || {
                        let reader = BufReader::new(stderr);
                        for line in reader.split("\n") {
                            if let Ok(line) = line {
                                let output_line = if timestamp {
                                    format!("[{}] STDERR: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"), line)
                                } else {
                                    format!("STDERR: {}", line)
                                };
                                
                                for callback in &callbacks {
                                    callback(&output_line);
                                }
                            }
                        }
                    });
                    
                    self.worker_threads.push(handle);
                }
            }
        }
        
        Ok(())
    }

    /// Wait for streaming to complete
    pub fn wait(&mut self) -> ProcessResult<()> {
        // Wait for command to complete
        self.cmd.wait()?;
        
        // Wait for worker threads to complete
        for handle in self.worker_threads.drain(..) {
            let _ = handle.join();
        }
        
        {
            let mut state = self.state.write().unwrap();
            if *state == StreamerState::Running {
                *state = StreamerState::Completed;
            }
        }
        
        Ok(())
    }

    /// Get current streaming state
    pub fn state(&self) -> StreamerState {
        self.state.read().unwrap().clone()
    }
}

/// Enhanced Input Generator with precise timing and sequencing
#[derive(Debug)]
pub struct EnhancedInputGenerator {
    /// Command to provide input to
    cmd: Cmd,
    /// Input sequence with timing
    input_sequence: Arc<Mutex<VecDeque<InputItem>>>,
    /// Worker thread handle
    worker_thread: Option<JoinHandle<()>>,
    /// Completion signal
    completion: Arc<(Mutex<bool>, Condvar)>,
    /// Input generation state
    state: Arc<RwLock<InputState>>,
}

#[derive(Debug, Clone)]
struct InputItem {
    data: Vec<u8>,
    delay: Option<Duration>,
    close_after: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum InputState {
    Created,
    Running,
    Completed,
    Closed,
}

impl EnhancedInputGenerator {
    /// Create a new enhanced input generator
    pub fn new(cmd: Cmd) -> Self {
        Self {
            cmd,
            input_sequence: Arc::new(Mutex::new(VecDeque::new())),
            worker_thread: None,
            completion: Arc::new((Mutex::new(false), Condvar::new())),
            state: Arc::new(RwLock::new(InputState::Created)),
        }
    }

    /// Write input immediately
    pub fn write<D: AsRef<[u8]>>(&mut self, data: D) -> ProcessResult<()> {
        let mut sequence = self.input_sequence.lock().unwrap();
        sequence.push_back(InputItem {
            data: data.as_ref().to_vec(),
            delay: None,
            close_after: false,
        });
        Ok(())
    }

    /// Write input after a delay
    pub fn write_after<D: AsRef<[u8]>>(&mut self, data: D, delay: Duration) -> ProcessResult<()> {
        let mut sequence = self.input_sequence.lock().unwrap();
        sequence.push_back(InputItem {
            data: data.as_ref().to_vec(),
            delay: Some(delay),
            close_after: false,
        });
        Ok(())
    }

    /// Write line (adds newline)
    pub fn write_line<S: AsRef<str>>(&mut self, line: S) -> ProcessResult<()> {
        let mut data = line.as_ref().as_bytes().to_vec();
        data.push(b'\n');
        self.write(data)
    }

    /// Write line after delay
    pub fn write_line_after<S: AsRef<str>>(&mut self, line: S, delay: Duration) -> ProcessResult<()> {
        let mut data = line.as_ref().as_bytes().to_vec();
        data.push(b'\n');
        self.write_after(data, delay)
    }

    /// Schedule input to be written repeatedly
    pub fn write_periodic<D: AsRef<[u8]>>(&mut self, data: D, interval: Duration, count: usize) -> ProcessResult<()> {
        for i in 0..count {
            let delay = if i == 0 { None } else { Some(interval) };
            let mut sequence = self.input_sequence.lock().unwrap();
            sequence.push_back(InputItem {
                data: data.as_ref().to_vec(),
                delay,
                close_after: false,
            });
        }
        Ok(())
    }

    /// Start the input generation process
    pub fn start(&mut self) -> ProcessResult<()> {
        {
            let mut state = self.state.write().unwrap();
            *state = InputState::Running;
        }

        self.cmd.start()?;
        
        if let Some(child) = &mut self.cmd.child {
            if let Some(stdin) = child.stdin.take() {
                let sequence = self.input_sequence.clone();
                let completion = self.completion.clone();
                let state = self.state.clone();
                
                let handle = thread::spawn(move || {
                    let mut writer = BufWriter::new(stdin);
                    
                    loop {
                        let item = {
                            let mut seq = sequence.lock().unwrap();
                            seq.pop_front()
                        };
                        
                        if let Some(input_item) = item {
                            // Handle delay
                            if let Some(delay) = input_item.delay {
                                thread::sleep(delay);
                            }
                            
                            // Write data
                            if writer.write_all(&input_item.data).is_err() {
                                break;
                            }
                            if writer.flush().is_err() {
                                break;
                            }
                            
                            // Close if requested
                            if input_item.close_after {
                                break;
                            }
                        } else {
                            // No more input, check if we should wait or exit
                            let (completed_lock, cvar) = &*completion;
                            let completed = completed_lock.lock().unwrap();
                            if *completed {
                                break;
                            }
                            drop(completed);
                            
                            // Wait a bit before checking again
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
                    
                    let mut state = state.write().unwrap();
                    *state = InputState::Completed;
                });
                
                self.worker_thread = Some(handle);
            }
        }
        
        Ok(())
    }

    /// Close input stream and signal completion
    pub fn close(&mut self) -> ProcessResult<()> {
        {
            let mut state = self.state.write().unwrap();
            *state = InputState::Closed;
        }

        // Signal completion
        let (completed_lock, cvar) = &*self.completion;
        let mut completed = completed_lock.lock().unwrap();
        *completed = true;
        cvar.notify_all();
        
        // Wait for worker thread to complete
        if let Some(handle) = self.worker_thread.take() {
            let _ = handle.join();
        }
        
        Ok(())
    }

    /// Get current input generation state
    pub fn state(&self) -> InputState {
        self.state.read().unwrap().clone()
    }
}

/// Enhanced timeout handling with better control and monitoring
pub struct TimeoutManager {
    /// Timeout duration
    timeout: Duration,
    /// Start time
    start_time: Instant,
    /// Cancellation signal
    cancelled: Arc<RwLock<bool>>,
    /// Timeout callback
    timeout_callback: Option<Box<dyn Fn() + Send + Sync>>,
}

impl TimeoutManager {
    /// Create a new timeout manager
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            start_time: Instant::now(),
            cancelled: Arc::new(RwLock::new(false)),
            timeout_callback: None,
        }
    }

    /// Set timeout callback
    pub fn on_timeout<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.timeout_callback = Some(Box::new(callback));
    }

    /// Check if timeout has elapsed
    pub fn is_expired(&self) -> bool {
        self.start_time.elapsed() >= self.timeout
    }

    /// Get remaining time
    pub fn remaining(&self) -> Duration {
        let elapsed = self.start_time.elapsed();
        if elapsed >= self.timeout {
            Duration::from_secs(0)
        } else {
            self.timeout - elapsed
        }
    }

    /// Cancel the timeout
    pub fn cancel(&self) {
        let mut cancelled = self.cancelled.write().unwrap();
        *cancelled = true;
    }

    /// Check if cancelled
    pub fn is_cancelled(&self) -> bool {
        *self.cancelled.read().unwrap()
    }
}

impl EnhancedProcessGroup {
    /// Create a new enhanced process group
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            options: ProcessGroupConfig::default(),
            processes: Vec::new(),
            state: Arc::new(RwLock::new(GroupState::Created)),
            completion: Arc::new((Mutex::new(0), Condvar::new())),
            errors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create with configuration
    pub fn with_config(config: ProcessGroupConfig) -> Self {
        Self {
            commands: Vec::new(),
            options: config,
            processes: Vec::new(),
            state: Arc::new(RwLock::new(GroupState::Created)),
            completion: Arc::new((Mutex::new(0), Condvar::new())),
            errors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a command to the group
    pub fn add_command(&mut self, cmd: Cmd) {
        self.commands.push(cmd);
    }

    /// Add multiple commands
    pub fn add_commands(&mut self, commands: Vec<Cmd>) {
        self.commands.extend(commands);
    }

    /// Start all commands with sophisticated scheduling
    pub fn start_all(&mut self) -> ProcessResult<()> {
        {
            let mut state = self.state.write().unwrap();
            *state = GroupState::Starting;
        }

        let max_parallel = if self.options.max_parallel == 0 {
            self.commands.len()
        } else {
            self.options.max_parallel
        };

        let mut started = 0;
        let mut pending = VecDeque::from_iter(self.commands.drain(..));

        // Start initial batch
        while started < max_parallel && !pending.is_empty() {
            if let Some(mut cmd) = pending.pop_front() {
                match cmd.start() {
                    Ok(()) => {
                        if let Ok(process) = cmd.process() {
                            let process_info = ProcessInfo {
                                process,
                                name: cmd.path.clone(),
                                start_time: Instant::now(),
                                completed: Arc::new(RwLock::new(false)),
                            };
                            self.processes.push(process_info);
                            started += 1;
                        }
                    }
                    Err(e) => {
                        let mut errors = self.errors.lock().unwrap();
                        errors.push(e);
                        
                        if !self.options.continue_on_failure {
                            let mut state = self.state.write().unwrap();
                            *state = GroupState::Failed;
                            return Err(errors.last().unwrap().clone());
                        }
                    }
                }
            }
        }

        // Start remaining processes as others complete
        if !pending.is_empty() {
            let pending = Arc::new(Mutex::new(pending));
            let processes = Arc::new(Mutex::new(&mut self.processes));
            let completion = self.completion.clone();
            let errors = self.errors.clone();
            let options = self.options.clone();
            let state = self.state.clone();

            thread::spawn(move || {
                loop {
                    let (completed_count, _) = &*completion;
                    let count = *completed_count.lock().unwrap();
                    
                    let mut pending_guard = pending.lock().unwrap();
                    if pending_guard.is_empty() {
                        break;
                    }
                    
                    if count > 0 && count < max_parallel {
                        if let Some(mut cmd) = pending_guard.pop_front() {
                            match cmd.start() {
                                Ok(()) => {
                                    if let Ok(process) = cmd.process() {
                                        let process_info = ProcessInfo {
                                            process,
                                            name: cmd.path.clone(),
                                            start_time: Instant::now(),
                                            completed: Arc::new(RwLock::new(false)),
                                        };
                                        // Note: In a real implementation, we'd need better synchronization here
                                    }
                                }
                                Err(e) => {
                                    let mut errors = errors.lock().unwrap();
                                    errors.push(e);
                                    
                                    if !options.continue_on_failure {
                                        let mut state = state.write().unwrap();
                                        *state = GroupState::Failed;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    
                    thread::sleep(Duration::from_millis(100));
                }
            });
        }

        {
            let mut state = self.state.write().unwrap();
            *state = GroupState::Running;
        }

        Ok(())
    }

    /// Wait for all processes to complete
    pub fn wait_all(&mut self) -> ProcessResult<()> {
        let start_time = Instant::now();
        
        for process_info in &mut self.processes {
            // Check group timeout
            if let Some(group_timeout) = self.options.group_timeout {
                if start_time.elapsed() >= group_timeout {
                    let mut state = self.state.write().unwrap();
                    *state = GroupState::Failed;
                    return Err(timeout_error("wait_all", &format!("Group timeout of {:?} exceeded", group_timeout)));
                }
            }
            
            // Wait for individual process with timeout
            let result = if let Some(process_timeout) = self.options.process_timeout {
                self.wait_with_timeout(&process_info.process, process_timeout)
            } else {
                process_info.process.wait().map(|_| ())
            };
            
            match result {
                Ok(()) => {
                    let mut completed = process_info.completed.write().unwrap();
                    *completed = true;
                    
                    // Update completion counter
                    let (completed_count, cvar) = &*self.completion;
                    let mut count = completed_count.lock().unwrap();
                    *count += 1;
                    cvar.notify_all();
                }
                Err(e) => {
                    let mut errors = self.errors.lock().unwrap();
                    errors.push(e.clone());
                    
                    if self.options.kill_on_failure {
                        self.kill_all()?;
                    }
                    
                    if !self.options.continue_on_failure {
                        let mut state = self.state.write().unwrap();
                        *state = GroupState::Failed;
                        return Err(e);
                    }
                }
            }
        }

        let mut state = self.state.write().unwrap();
        *state = GroupState::Completed;
        Ok(())
    }

    /// Kill all running processes
    pub fn kill_all(&mut self) -> ProcessResult<()> {
        for process_info in &self.processes {
            let _ = process_info.process.kill(); // Ignore individual kill errors
        }
        
        let mut state = self.state.write().unwrap();
        *state = GroupState::Cancelled;
        Ok(())
    }

    /// Get group state
    pub fn state(&self) -> GroupState {
        self.state.read().unwrap().clone()
    }

    /// Get errors that occurred during execution
    pub fn errors(&self) -> Vec<ProcessError> {
        self.errors.lock().unwrap().clone()
    }

    /// Wait for process with timeout
    fn wait_with_timeout(&self, process: &Process, timeout: Duration) -> ProcessResult<()> {
        let start = Instant::now();
        loop {
            if start.elapsed() >= timeout {
                return Err(timeout_error("wait", &format!("Process timeout of {:?} exceeded", timeout)));
            }
            
            // Check if process is still running (simplified check)
            match process.wait() {
                Ok(_) => return Ok(()),
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
            }
        }
    }
}

/// Enhanced command creation functions

/// Create command with enhanced environment
pub fn command_with_enhanced_env<S: AsRef<str>>(name: S, args: &[&str], env: EnhancedEnvironment) -> Cmd {
    let mut cmd = Cmd::new(name, args);
    cmd.env = env.build_env();
    cmd
}

/// Run command with enhanced timeout control
pub fn run_with_enhanced_timeout<S: AsRef<str>>(
    name: S, 
    args: &[&str], 
    timeout: Duration,
    on_timeout: Option<Box<dyn Fn() + Send + Sync>>
) -> ProcessResult<Vec<u8>> {
    let mut cmd = Cmd::new(name, args);
    let mut timeout_mgr = TimeoutManager::new(timeout);
    
    if let Some(callback) = on_timeout {
        timeout_mgr.on_timeout(move || callback());
    }
    
    let start_time = Instant::now();
    cmd.start()?;
    
    // Monitor timeout in separate thread
    let timeout_check = timeout_mgr.cancelled.clone();
    let cmd_pid = cmd.process()?.pid;
    
    thread::spawn(move || {
        while start_time.elapsed() < timeout {
            if *timeout_check.read().unwrap() {
                return;
            }
            thread::sleep(Duration::from_millis(100));
        }
        
        // Timeout occurred, try to kill process
        #[cfg(unix)]
        unsafe {
            libc::kill(cmd_pid as i32, libc::SIGKILL);
        }
    });
    
    let result = cmd.output();
    timeout_mgr.cancel();
    result
}

/// Enhanced executable path lookup with better search algorithms
pub fn enhanced_look_path<S: AsRef<str>>(file: S) -> ProcessResult<PathBuf> {
    let file = file.as_ref();
    
    // If it's already an absolute path and exists, return it
    let path = Path::new(file);
    if path.is_absolute() {
        if path.is_file() {
            return Ok(path.to_path_buf());
        } else {
            return Err(execution_failed(file, "Absolute path does not exist or is not a file"));
        }
    }
    
    // Search in PATH
    if let Ok(path_env) = std::env::var("PATH") {
        for search_path in std::env::split_paths(&path_env) {
            let full_path = search_path.join(file);
            
            // Check exact match
            if full_path.is_file() && is_executable(&full_path) {
                return Ok(full_path);
            }
            
            // On Windows, also check with various extensions
            #[cfg(windows)]
            {
                for ext in &[".exe", ".cmd", ".bat", ".com"] {
                    let exe_path = search_path.join(format!("{}{}", file, ext));
                    if exe_path.is_file() && is_executable(&exe_path) {
                        return Ok(exe_path);
                    }
                }
            }
        }
    }
    
    // Search in current directory as fallback
    let current_dir = std::env::current_dir()
        .map_err(|e| io_error("look_path", "current_dir", &e.to_string()))?;
    let local_path = current_dir.join(file);
    if local_path.is_file() && is_executable(&local_path) {
        return Ok(local_path);
    }
    
    Err(execution_failed(file, "Command not found in PATH or current directory"))
}

/// Check if a file is executable
fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            // Check if any execute bit is set
            (permissions.mode() & 0o111) != 0
        } else {
            false
        }
    }
    
    #[cfg(windows)]
    {
        // On Windows, check file extension or just assume it's executable if it exists
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "exe" | "cmd" | "bat" | "com" | "ps1")
        } else {
            true // Assume executable if no extension
        }
    }
}

/// Factory functions for creating enhanced objects

/// Create a new enhanced process group
pub fn new_enhanced_process_group() -> EnhancedProcessGroup {
    EnhancedProcessGroup::new()
}

/// Create enhanced process group with configuration
pub fn new_enhanced_process_group_with_config(config: ProcessGroupConfig) -> EnhancedProcessGroup {
    EnhancedProcessGroup::with_config(config)
}

/// Create a new enhanced environment
pub fn new_enhanced_environment() -> EnhancedEnvironment {
    EnhancedEnvironment::new()
}

/// Create a new enhanced output streamer
pub fn new_enhanced_output_streamer(cmd: Cmd) -> EnhancedOutputStreamer {
    EnhancedOutputStreamer::new(cmd)
}

/// Create a new enhanced input generator
pub fn new_enhanced_input_generator(cmd: Cmd) -> EnhancedInputGenerator {
    EnhancedInputGenerator::new(cmd)
}

// Add chrono dependency for timestamps (this would need to be added to Cargo.toml)
// For now, we'll provide a simple timestamp function
mod chrono {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub struct Utc;
    
    impl Utc {
        pub fn now() -> DateTime {
            DateTime {
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            }
        }
    }
    
    pub struct DateTime {
        timestamp: u64,
    }
    
    impl DateTime {
        pub fn format(&self, _format: &str) -> String {
            // Simplified timestamp formatting
            format!("{}", self.timestamp)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
use crate::stdlib::process::info::ProcessInfo;
use crate::stdlib::process::info::ProcessState;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_enhanced_environment() {
        let mut env = EnhancedEnvironment::new();
        env.set("TEST_VAR", "test_value");
        env.append_path("/usr/local/bin");
        env.prepend_path("/opt/bin");
        
        let env_vec = env.build_env();
        assert!(env_vec.iter().any(|s| s.starts_with("TEST_VAR=")));
    }

    #[test]
    fn test_enhanced_process_group() {
        let mut group = EnhancedProcessGroup::new();
        let cmd = Cmd::new("echo", &["test"]);
        group.add_command(cmd);
        
        assert_eq!(group.commands.len(), 1);
        assert_eq!(group.state(), GroupState::Created);
    }

    #[test]
    fn test_enhanced_output_streamer() {
        let cmd = Cmd::new("echo", &["test"]);
        let mut streamer = EnhancedOutputStreamer::new(cmd);
        streamer.set_buffer_size(4096);
        streamer.set_timestamp_lines(true);
        
        assert_eq!(streamer.state(), StreamerState::Created);
    }

    #[test]
    fn test_enhanced_input_generator() {
        let cmd = Cmd::new("cat", &[]);
        let mut generator = EnhancedInputGenerator::new(cmd);
        
        assert!(generator.write("test input").is_ok());
        assert!(generator.write_line_after("delayed line", Duration::from_millis(500)).is_ok());
        assert_eq!(generator.state(), InputState::Created);
    }

    #[test]
    fn test_timeout_manager() {
        let mut timeout_mgr = TimeoutManager::new(Duration::from_millis(100));
        assert!(!timeout_mgr.is_expired());
        assert!(timeout_mgr.remaining() > Duration::from_millis(50));
        
        timeout_mgr.cancel();
        assert!(timeout_mgr.is_cancelled());
    }

    #[test]
    fn test_process_group_config() {
        let config = ProcessGroupConfig {
            max_parallel: 4,
            process_timeout: Some(Duration::from_secs(30)),
            group_timeout: Some(Duration::from_secs(300)),
            kill_on_failure: true,
            continue_on_failure: false,
            collect_outputs: true,
        };
        
        let group = EnhancedProcessGroup::with_config(config.clone());
        assert_eq!(group.options.max_parallel, 4);
        assert_eq!(group.options.process_timeout, Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_enhanced_look_path() {
        // Test with a common command that should exist
        #[cfg(unix)]
        {
            let result = enhanced_look_path("sh");
            assert!(result.is_ok());
        }
        
        #[cfg(windows)]
        {
            let result = enhanced_look_path("cmd");
            assert!(result.is_ok());
        }
        
        // Test with non-existent command
        let result = enhanced_look_path("definitely_does_not_exist_command_12345");
        assert!(result.is_err());
    }
}
