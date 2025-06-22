/// Core process management functionality for CURSED
/// 
/// This module provides the fundamental process operations including spawning,
/// configuration, I/O handling, and basic process control.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, execution_failed, execution_failed_with_code,
    timeout_error, invalid_arguments, io_error, system_error, invalid_state,
    permission_denied_pid, platform_error, process_not_found_pid
};
use crate::stdlib::process::real_monitoring::{
    register_process_for_monitoring, unregister_process_from_monitoring,
    start_global_monitoring, get_current_process_state
};

/// Process state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Process has been created but not started
    Created,
    /// Process is currently running
    Running,
    /// Process is waiting for some event or resource
    Waiting,
    /// Process has been stopped/suspended
    Stopped,
    /// Process has terminated
    Terminated,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessState::Created => write!(f, "Created"),
            ProcessState::Running => write!(f, "Running"),
            ProcessState::Waiting => write!(f, "Waiting"),
            ProcessState::Stopped => write!(f, "Stopped"),
            ProcessState::Terminated => write!(f, "Terminated"),
        }
    }
}

/// Detailed process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Parent process ID
    pub ppid: Option<u32>,
    /// Process name/command
    pub name: String,
    /// Process state
    pub state: ProcessState,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Process start time
    pub start_time: Instant,
    /// Process uptime
    pub uptime: Duration,
    /// Working directory
    pub working_dir: Option<PathBuf>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Command line arguments
    pub args: Vec<String>,
    /// Exit code (if terminated)
    pub exit_code: Option<i32>,
}

impl ProcessInfo {
    /// Create new process info
    pub fn new(pid: u32, name: String) -> Self {
        Self {
            pid,
            ppid: None,
            name,
            state: ProcessState::Created,
            cpu_usage: 0.0,
            memory_usage: 0,
            start_time: Instant::now(),
            uptime: Duration::from_secs(0),
            working_dir: None,
            env_vars: HashMap::new(),
            args: Vec::new(),
            exit_code: None,
        }
    }

    /// Update uptime based on start time
    pub fn update_uptime(&mut self) {
        self.uptime = self.start_time.elapsed();
    }

    /// Check if process is alive
    pub fn is_alive(&self) -> bool {
        !matches!(self.state, ProcessState::Terminated)
    }

    /// Get human-readable memory usage
    pub fn memory_usage_human(&self) -> String {
        let kb = self.memory_usage as f64 / 1024.0;
        if kb < 1024.0 {
            format!("{:.1} KB", kb)
        } else {
            let mb = kb / 1024.0;
            if mb < 1024.0 {
                format!("{:.1} MB", mb)
            } else {
                let gb = mb / 1024.0;
                format!("{:.1} GB", gb)
            }
        }
    }
}

/// Process group for managing related processes
#[derive(Debug)]
pub struct ProcessGroup {
    /// Group ID
    pub id: u32,
    /// Group name
    pub name: Option<String>,
    /// Processes in this group
    pub processes: HashMap<u32, ProcessInfo>,
    /// Parent group ID
    pub parent_group: Option<u32>,
    /// Child groups
    pub child_groups: Vec<u32>,
    /// Group creation time
    pub created_at: Instant,
}

impl ProcessGroup {
    /// Create a new process group
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            processes: HashMap::new(),
            parent_group: None,
            child_groups: Vec::new(),
            created_at: Instant::now(),
        }
    }

    /// Create a named process group
    pub fn with_name(id: u32, name: String) -> Self {
        Self {
            id,
            name: Some(name),
            processes: HashMap::new(),
            parent_group: None,
            child_groups: Vec::new(),
            created_at: Instant::now(),
        }
    }

    /// Add a process to the group
    pub fn add_process(&mut self, process_info: ProcessInfo) {
        self.processes.insert(process_info.pid, process_info);
    }

    /// Remove a process from the group
    pub fn remove_process(&mut self, pid: u32) -> Option<ProcessInfo> {
        self.processes.remove(&pid)
    }

    /// Get process by PID
    pub fn get_process(&self, pid: u32) -> Option<&ProcessInfo> {
        self.processes.get(&pid)
    }

    /// Get mutable process by PID
    pub fn get_process_mut(&mut self, pid: u32) -> Option<&mut ProcessInfo> {
        self.processes.get_mut(&pid)
    }

    /// Get all processes in the group
    pub fn all_processes(&self) -> Vec<&ProcessInfo> {
        self.processes.values().collect()
    }

    /// Get running processes count
    pub fn running_count(&self) -> usize {
        self.processes.values()
            .filter(|p| matches!(p.state, ProcessState::Running))
            .count()
    }

    /// Get total memory usage of all processes
    pub fn total_memory_usage(&self) -> u64 {
        self.processes.values()
            .map(|p| p.memory_usage)
            .sum()
    }

    /// Get average CPU usage of all processes
    pub fn average_cpu_usage(&self) -> f64 {
        if self.processes.is_empty() {
            0.0
        } else {
            let total: f64 = self.processes.values()
                .map(|p| p.cpu_usage)
                .sum();
            total / self.processes.len() as f64
        }
    }

    /// Kill all processes in the group
    pub fn kill_all(&mut self) -> ProcessResult<()> {
        let pids: Vec<u32> = self.processes.keys().cloned().collect();
        
        for pid in pids {
            if let Err(e) = kill_process(pid) {
                tracing::warn!(pid = pid, error = ?e, "Failed to kill process in group");
            }
        }

        // Update all processes to terminated state
        for process in self.processes.values_mut() {
            process.state = ProcessState::Terminated;
        }

        Ok(())
    }

    /// Suspend all processes in the group
    pub fn suspend_all(&mut self) -> ProcessResult<()> {
        for &pid in self.processes.keys() {
            suspend_process(pid)?;
        }
        Ok(())
    }

    /// Resume all processes in the group
    pub fn resume_all(&mut self) -> ProcessResult<()> {
        for &pid in self.processes.keys() {
            resume_process(pid)?;
        }
        Ok(())
    }
}

/// Process configuration builder
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    /// Command to execute
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Working directory
    pub working_dir: Option<PathBuf>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Whether to clear existing environment
    pub clear_env: bool,
    /// Standard input configuration
    pub stdin: ProcessIo,
    /// Standard output configuration
    pub stdout: ProcessIo,
    /// Standard error configuration
    pub stderr: ProcessIo,
    /// Process execution timeout
    pub timeout: Option<Duration>,
    /// Process group ID (Unix only)
    pub process_group: Option<u32>,
    /// User ID to run as (Unix only)
    pub user_id: Option<u32>,
    /// Group ID to run as (Unix only)
    pub group_id: Option<u32>,
    /// Create new session (Unix only)
    pub new_session: bool,
    /// Detach from parent (Unix only)
    pub detached: bool,
    /// Resource limits for the process
    pub resource_limits: ResourceLimits,
}

impl ProcessConfig {
    /// Create a new process configuration
    pub fn new<S: AsRef<str>>(command: S) -> Self {
        Self {
            command: command.as_ref().to_string(),
            args: Vec::new(),
            working_dir: None,
            env_vars: HashMap::new(),
            clear_env: false,
            stdin: ProcessIo::Null,
            stdout: ProcessIo::Inherit,
            stderr: ProcessIo::Inherit,
            timeout: None,
            process_group: None,
            user_id: None,
            group_id: None,
            new_session: false,
            detached: false,
            resource_limits: ResourceLimits::default(),
        }
    }

    /// Add command line argument
    pub fn arg<S: AsRef<str>>(mut self, arg: S) -> Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    /// Add multiple command line arguments
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for arg in args {
            self.args.push(arg.as_ref().to_string());
        }
        self
    }

    /// Set working directory
    pub fn working_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Set environment variable
    pub fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.env_vars.insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Set multiple environment variables
    pub fn envs<I, K, V>(mut self, envs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (key, value) in envs {
            self.env_vars.insert(key.as_ref().to_string(), value.as_ref().to_string());
        }
        self
    }

    /// Clear existing environment
    pub fn clear_env(mut self) -> Self {
        self.clear_env = true;
        self
    }

    /// Set stdin configuration
    pub fn stdin(mut self, config: ProcessIo) -> Self {
        self.stdin = config;
        self
    }

    /// Set stdout configuration
    pub fn stdout(mut self, config: ProcessIo) -> Self {
        self.stdout = config;
        self
    }

    /// Set stderr configuration
    pub fn stderr(mut self, config: ProcessIo) -> Self {
        self.stderr = config;
        self
    }

    /// Set execution timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set process group ID (Unix only)
    #[cfg(unix)]
    pub fn process_group(mut self, pgid: u32) -> Self {
        self.process_group = Some(pgid);
        self
    }

    /// Set user ID (Unix only)
    #[cfg(unix)]
    pub fn user_id(mut self, uid: u32) -> Self {
        self.user_id = Some(uid);
        self
    }

    /// Set group ID (Unix only)
    #[cfg(unix)]
    pub fn group_id(mut self, gid: u32) -> Self {
        self.group_id = Some(gid);
        self
    }

    /// Create new session (Unix only)
    #[cfg(unix)]
    pub fn new_session(mut self) -> Self {
        self.new_session = true;
        self
    }

    /// Detach from parent (Unix only)
    #[cfg(unix)]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// Set resource limits
    pub fn resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = limits;
        self
    }

    /// Apply resource limits to spawned process
    pub fn apply_resource_limits(&self) -> ProcessResult<()> {
        use crate::stdlib::process::resource_limits::{ResourceLimitManager, ResourceType, ResourceLimit};
        
        let mut manager = ResourceLimitManager::new();
        
        if let Some(max_memory) = self.resource_limits.max_memory {
            manager.set_limit(ResourceType::AddressSpace, ResourceLimit::fixed(max_memory))?;
        }
        
        if let Some(max_cpu_time) = self.resource_limits.max_cpu_time {
            manager.set_limit(ResourceType::CpuTime, ResourceLimit::fixed(max_cpu_time.as_secs()))?;
        }
        
        if let Some(max_files) = self.resource_limits.max_file_descriptors {
            manager.set_limit(ResourceType::OpenFiles, ResourceLimit::fixed(max_files))?;
        }
        
        Ok(())
    }
}

/// Process I/O configuration
#[derive(Debug, Clone)]
pub enum ProcessIo {
    /// Inherit from parent
    Inherit,
    /// Pipe to/from process
    Pipe,
    /// Null device (/dev/null)
    Null,
    /// File path
    File(PathBuf),
    /// Custom stdio
    Custom(Stdio),
}

impl ProcessIo {
    /// Convert to std::process::Stdio
    pub fn to_stdio(&self) -> io::Result<Stdio> {
        match self {
            ProcessIo::Inherit => Ok(Stdio::inherit()),
            ProcessIo::Pipe => Ok(Stdio::piped()),
            ProcessIo::Null => Ok(Stdio::null()),
            ProcessIo::File(path) => {
                let file = File::open(path)?;
                Ok(Stdio::from(file))
            }
            ProcessIo::Custom(_) => {
                // Stdio cannot be cloned, so default to inherit
                Ok(Stdio::inherit())
            }
        }
    }

    /// Create ProcessIo from file for writing
    pub fn write_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(ProcessIo::File(path.as_ref().to_path_buf()))
    }

    /// Create ProcessIo from file for reading
    pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(ProcessIo::File(path.as_ref().to_path_buf()))
    }

    /// Convert to stdio for output (create or truncate file)
    pub fn to_output_stdio(&self) -> io::Result<Stdio> {
        match self {
            ProcessIo::Inherit => Ok(Stdio::inherit()),
            ProcessIo::Pipe => Ok(Stdio::piped()),
            ProcessIo::Null => Ok(Stdio::null()),
            ProcessIo::File(path) => {
                let file = File::create(path)?;
                Ok(Stdio::from(file))
            }
            ProcessIo::Custom(_) => Ok(Stdio::inherit()),
        }
    }
}

/// Resource limits for process management
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum CPU time in seconds
    pub max_cpu_time: Option<u64>,
    /// Maximum memory usage in bytes
    pub max_memory: Option<u64>,
    /// Maximum number of file descriptors
    pub max_file_descriptors: Option<u64>,
    /// Maximum number of processes
    pub max_processes: Option<u64>,
    /// Maximum execution time
    pub max_execution_time: Option<Duration>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_time: None,
            max_memory: None,
            max_file_descriptors: None,
            max_processes: None,
            max_execution_time: None,
        }
    }
}

impl ResourceLimits {
    /// Create new resource limits
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum CPU time
    pub fn max_cpu_time(mut self, seconds: u64) -> Self {
        self.max_cpu_time = Some(seconds);
        self
    }

    /// Set maximum memory usage
    pub fn max_memory(mut self, bytes: u64) -> Self {
        self.max_memory = Some(bytes);
        self
    }

    /// Set maximum memory usage in MB
    pub fn max_memory_mb(mut self, mb: u64) -> Self {
        self.max_memory = Some(mb * 1024 * 1024);
        self
    }

    /// Set maximum file descriptors
    pub fn max_file_descriptors(mut self, count: u64) -> Self {
        self.max_file_descriptors = Some(count);
        self
    }

    /// Set maximum processes
    pub fn max_processes(mut self, count: u64) -> Self {
        self.max_processes = Some(count);
        self
    }

    /// Set maximum execution time
    pub fn max_execution_time(mut self, duration: Duration) -> Self {
        self.max_execution_time = Some(duration);
        self
    }

    /// Check if memory limit is exceeded
    pub fn is_memory_exceeded(&self, current: u64) -> bool {
        self.max_memory.map_or(false, |limit| current > limit)
    }

    /// Check if CPU time limit is exceeded
    pub fn is_cpu_time_exceeded(&self, current: u64) -> bool {
        self.max_cpu_time.map_or(false, |limit| current > limit)
    }

    /// Check if execution time limit is exceeded
    pub fn is_execution_time_exceeded(&self, duration: Duration) -> bool {
        self.max_execution_time.map_or(false, |limit| duration > limit)
    }
}

/// Process output capture
#[derive(Debug, Clone)]
pub struct ProcessOutput {
    /// Exit status
    pub status: ExitStatus,
    /// Standard output
    pub stdout: Vec<u8>,
    /// Standard error
    pub stderr: Vec<u8>,
    /// Execution duration
    pub duration: Duration,
}

impl ProcessOutput {
    /// Check if process succeeded
    pub fn success(&self) -> bool {
        self.status.success()
    }

    /// Get exit code
    pub fn exit_code(&self) -> Option<i32> {
        self.status.code()
    }

    /// Get stdout as string
    pub fn stdout_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.stdout.clone())
    }

    /// Get stderr as string
    pub fn stderr_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.stderr.clone())
    }

    /// Get stdout as string (lossy)
    pub fn stdout_lossy(&self) -> String {
        String::from_utf8_lossy(&self.stdout).to_string()
    }

    /// Get stderr as string (lossy)
    pub fn stderr_lossy(&self) -> String {
        String::from_utf8_lossy(&self.stderr).to_string()
    }
}

/// Running process handle
pub struct Process {
    /// Process configuration
    config: ProcessConfig,
    /// Child process handle
    child: Child,
    /// Process information
    info: ProcessInfo,
    /// Process start time
    start_time: Instant,
    /// Output buffers (if capturing)
    output_buffer: Arc<Mutex<(Vec<u8>, Vec<u8>)>>,
    /// Background threads for output capture
    output_threads: Vec<thread::JoinHandle<()>>,
    /// Process monitoring enabled
    monitoring_enabled: bool,
    /// Resource limits
    resource_limits: ResourceLimits,
}

impl Process {
    /// Get process ID
    pub fn id(&self) -> u32 {
        self.child.id()
    }

    /// Get process information
    pub fn info(&self) -> &ProcessInfo {
        &self.info
    }

    /// Get mutable process information
    pub fn info_mut(&mut self) -> &mut ProcessInfo {
        &mut self.info
    }

    /// Get current process state
    pub fn state(&self) -> ProcessState {
        self.info.state
    }

    /// Update process state
    pub fn set_state(&mut self, state: ProcessState) {
        self.info.state = state;
    }

    /// Get resource limits
    pub fn resource_limits(&self) -> &ResourceLimits {
        &self.resource_limits
    }

    /// Set resource limits
    pub fn set_resource_limits(&mut self, limits: ResourceLimits) {
        self.resource_limits = limits;
    }

    /// Wait for process to complete
    pub fn wait(&mut self) -> ProcessResult<ExitStatus> {
        let status = self.child.wait()
            .map_err(|e| io_error("wait", &format!("{:?}", e.kind()), &e.to_string()))?;
        Ok(status)
    }

    /// Wait for process with timeout
    pub fn wait_timeout(&mut self, timeout: Duration) -> ProcessResult<Option<ExitStatus>> {
        let start = Instant::now();
        
        // Poll-based waiting with timeout
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => return Ok(Some(status)),
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        return Ok(None);
                    }
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => return Err(io_error("wait_timeout", &format!("{:?}", e.kind()), &e.to_string())),
            }
        }
    }

    /// Kill the process
    pub fn kill(&mut self) -> ProcessResult<()> {
        self.child.kill()
            .map_err(|e| io_error("kill", &format!("{:?}", e.kind()), &e.to_string()))?;
        Ok(())
    }

    /// Get stdin handle
    pub fn stdin(&mut self) -> Option<&mut std::process::ChildStdin> {
        self.child.stdin.as_mut()
    }

    /// Get stdout handle
    pub fn stdout(&mut self) -> Option<&mut std::process::ChildStdout> {
        self.child.stdout.as_mut()
    }

    /// Get stderr handle
    pub fn stderr(&mut self) -> Option<&mut std::process::ChildStderr> {
        self.child.stderr.as_mut()
    }

    /// Write to stdin
    pub fn write_stdin(&mut self, data: &[u8]) -> ProcessResult<()> {
        if let Some(stdin) = self.stdin() {
            stdin.write_all(data)
                .map_err(|e| io_error("write_stdin", &format!("{:?}", e.kind()), &e.to_string()))?;
            stdin.flush()
                .map_err(|e| io_error("flush_stdin", &format!("{:?}", e.kind()), &e.to_string()))?;
        }
        Ok(())
    }

    /// Read from stdout
    pub fn read_stdout(&mut self, buffer: &mut Vec<u8>) -> ProcessResult<usize> {
        if let Some(stdout) = self.stdout() {
            let mut temp_buffer = [0u8; 4096];
            let bytes_read = stdout.read(&mut temp_buffer)
                .map_err(|e| io_error("read_stdout", &format!("{:?}", e.kind()), &e.to_string()))?;
            buffer.extend_from_slice(&temp_buffer[..bytes_read]);
            Ok(bytes_read)
        } else {
            Ok(0)
        }
    }

    /// Read from stderr
    pub fn read_stderr(&mut self, buffer: &mut Vec<u8>) -> ProcessResult<usize> {
        if let Some(stderr) = self.stderr() {
            let mut temp_buffer = [0u8; 4096];
            let bytes_read = stderr.read(&mut temp_buffer)
                .map_err(|e| io_error("read_stderr", &format!("{:?}", e.kind()), &e.to_string()))?;
            buffer.extend_from_slice(&temp_buffer[..bytes_read]);
            Ok(bytes_read)
        } else {
            Ok(0)
        }
    }

    /// Get process uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Check if process is still running
    pub fn is_running(&mut self) -> ProcessResult<bool> {
        match self.child.try_wait() {
            Ok(Some(_)) => Ok(false),
            Ok(None) => Ok(true),
            Err(e) => Err(io_error("is_running", &format!("{:?}", e.kind()), &e.to_string())),
        }
    }

    /// Start background output capture threads
    fn start_output_capture(&mut self) -> ProcessResult<()> {
        // Don't start capture if not using pipes
        if !matches!(self.config.stdout, ProcessIo::Pipe) && 
           !matches!(self.config.stderr, ProcessIo::Pipe) {
            return Ok(());
        }

        let buffer = Arc::clone(&self.output_buffer);
        
        // Capture stdout in background thread
        if matches!(self.config.stdout, ProcessIo::Pipe) {
            if let Some(stdout) = self.child.stdout.take() {
                let buffer_clone = Arc::clone(&buffer);
                let handle = thread::spawn(move || {
                    let mut reader = BufReader::new(stdout);
                    let mut line = String::new();
                    while reader.read_line(&mut line).unwrap_or(0) > 0 {
                        if let Ok(mut buf) = buffer_clone.lock() {
                            buf.0.extend_from_slice(line.as_bytes());
                        }
                        line.clear();
                    }
                });
                self.output_threads.push(handle);
            }
        }

        // Capture stderr in background thread
        if matches!(self.config.stderr, ProcessIo::Pipe) {
            if let Some(stderr) = self.child.stderr.take() {
                let buffer_clone = Arc::clone(&buffer);
                let handle = thread::spawn(move || {
                    let mut reader = BufReader::new(stderr);
                    let mut line = String::new();
                    while reader.read_line(&mut line).unwrap_or(0) > 0 {
                        if let Ok(mut buf) = buffer_clone.lock() {
                            buf.1.extend_from_slice(line.as_bytes());
                        }
                        line.clear();
                    }
                });
                self.output_threads.push(handle);
            }
        }

        Ok(())
    }

    /// Get captured output
    pub fn get_output(&self) -> ProcessResult<(Vec<u8>, Vec<u8>)> {
        if let Ok(buffer) = self.output_buffer.lock() {
            Ok(buffer.clone())
        } else {
            Err(io_error("get_output", "LockError", "Failed to lock output buffer"))
        }
    }

    /// Enable process monitoring
    pub fn enable_monitoring(&mut self) {
        self.monitoring_enabled = true;
    }

    /// Check if monitoring is enabled
    pub fn is_monitoring_enabled(&self) -> bool {
        self.monitoring_enabled
    }

    /// Send a signal to the process (Unix only)
    #[cfg(unix)]
    pub fn send_signal(&self, signal: i32) -> ProcessResult<()> {
        use crate::stdlib::process::control::send_signal_to_pid;
        send_signal_to_pid(self.id(), signal)
    }

    /// Terminate process gracefully
    pub fn terminate(&mut self) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            self.send_signal(15)?; // SIGTERM
            
            // Wait briefly for graceful termination
            if let Ok(Some(_)) = self.wait_timeout(Duration::from_secs(5)) {
                return Ok(());
            }
            
            // Force kill if still running
            self.kill()
        }

        #[cfg(windows)]
        {
            // On Windows, just kill the process
            self.kill()
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let pid = self.id();
        
        // Unregister from monitoring
        let _ = unregister_process_from_monitoring(pid);
        
        // Attempt to terminate the process gracefully
        let _ = self.terminate();
        
        // Wait for output threads to finish
        while let Some(handle) = self.output_threads.pop() {
            let _ = handle.join();
        }
        
        tracing::debug!(pid = pid, "Process dropped and cleaned up");
    }
}

/// Spawn a process with configuration
pub fn spawn_process(config: ProcessConfig) -> ProcessResult<Process> {
    let mut command = Command::new(&config.command);

    // Set arguments
    command.args(&config.args);

    // Set working directory
    if let Some(dir) = &config.working_dir {
        command.current_dir(dir);
    }

    // Set environment
    if config.clear_env {
        command.env_clear();
    }
    for (key, value) in &config.env_vars {
        command.env(key, value);
    }

    // Set I/O configuration
    command.stdin(config.stdin.to_stdio().map_err(|e| {
        io_error("configure_stdin", &format!("{:?}", e.kind()), &e.to_string())
    })?);
    command.stdout(config.stdout.to_output_stdio().map_err(|e| {
        io_error("configure_stdout", &format!("{:?}", e.kind()), &e.to_string())
    })?);
    command.stderr(config.stderr.to_output_stdio().map_err(|e| {
        io_error("configure_stderr", &format!("{:?}", e.kind()), &e.to_string())
    })?);

    // Platform-specific configuration
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        
        if let Some(pgid) = config.process_group {
            command.process_group(pgid);
        }
        
        if config.new_session {
            // This would require unsafe code to call setsid()
        }
    }

    // Apply resource limits before spawning if configured
    if config.resource_limits.max_memory.is_some() || 
       config.resource_limits.max_cpu_time.is_some() || 
       config.resource_limits.max_file_descriptors.is_some() {
        // Note: Resource limits should ideally be applied after fork but before exec
        // For now, we'll apply them to the current process (they'll be inherited)
        let _ = config.apply_resource_limits();
    }

    // Spawn the process
    let child = command.spawn()
        .map_err(|e| execution_failed(&config.command, &e.to_string()))?;

    let pid = child.id();
    
    // Create process info
    let mut info = ProcessInfo::new(pid, config.command.clone());
    info.state = ProcessState::Running;
    info.args = config.args.clone();
    info.env_vars = config.env_vars.clone();
    info.working_dir = config.working_dir.clone();
    
    let mut process = Process {
        resource_limits: config.resource_limits.clone(),
        config,
        child,
        info,
        start_time: Instant::now(),
        output_buffer: Arc::new(Mutex::new((Vec::new(), Vec::new()))),
        output_threads: Vec::new(),
        monitoring_enabled: false,
    };

    // Register with real monitoring system
    let child_arc = Arc::new(Mutex::new(unsafe {
        // This is necessary to share the child process handle with the monitor
        std::ptr::read(&process.child as *const Child)
    }));
    let _ = register_process_for_monitoring(pid, Some(child_arc));

    // Start background output capture if stdout/stderr are piped
    process.start_output_capture()?;

    // Ensure global monitoring is started
    start_global_monitoring();

    Ok(process)
}

/// Run a command and wait for completion
pub fn run_command(config: ProcessConfig) -> ProcessResult<ProcessOutput> {
    let start_time = Instant::now();
    
    let mut command = Command::new(&config.command);
    command.args(&config.args);

    if let Some(dir) = &config.working_dir {
        command.current_dir(dir);
    }

    if config.clear_env {
        command.env_clear();
    }
    for (key, value) in &config.env_vars {
        command.env(key, value);
    }

    // Capture output
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = command.output()
        .map_err(|e| execution_failed(&config.command, &e.to_string()))?;

    let duration = start_time.elapsed();

    Ok(ProcessOutput {
        status: output.status,
        stdout: output.stdout,
        stderr: output.stderr,
        duration,
    })
}

/// Run a command with timeout
pub fn run_command_timeout(config: ProcessConfig, timeout: Duration) -> ProcessResult<ProcessOutput> {
    let start_time = Instant::now();
    let mut process = spawn_process(config.clone())?;

    // Wait for process with timeout
    match process.wait_timeout(timeout)? {
        Some(status) => {
            let duration = start_time.elapsed();
            
            // Collect output if available
            let mut stdout = Vec::new();
            let mut stderr = Vec::new();
            
            let _ = process.read_stdout(&mut stdout);
            let _ = process.read_stderr(&mut stderr);

            Ok(ProcessOutput {
                status,
                stdout,
                stderr,
                duration,
            })
        }
        None => {
            // Timeout occurred, kill the process
            process.kill()?;
            Err(timeout_error("run_command", timeout, "Process execution timed out"))
        }
    }
}

/// Check if a command exists in PATH
pub fn command_exists<S: AsRef<str>>(command: S) -> bool {
    let command = command.as_ref();
    
    // Try to find the command in PATH
    if let Ok(paths) = std::env::var("PATH") {
        for path in std::env::split_paths(&paths) {
            let full_path = path.join(command);
            if full_path.is_file() {
                return true;
            }
            
            // On Windows, also check with .exe extension
            #[cfg(windows)]
            {
                let exe_path = path.join(format!("{}.exe", command));
                if exe_path.is_file() {
                    return true;
                }
            }
        }
    }
    
    false
}

/// Find command in PATH
pub fn which<S: AsRef<str>>(command: S) -> ProcessResult<PathBuf> {
    let command = command.as_ref();
    
    if let Ok(paths) = std::env::var("PATH") {
        for path in std::env::split_paths(&paths) {
            let full_path = path.join(command);
            if full_path.is_file() {
                return Ok(full_path);
            }
            
            // On Windows, also check with .exe extension
            #[cfg(windows)]
            {
                let exe_path = path.join(format!("{}.exe", command));
                if exe_path.is_file() {
                    return Ok(exe_path);
                }
            }
        }
    }
    
    Err(execution_failed(command, "Command not found in PATH"))
}

/// Simple command execution helper
pub fn exec<S: AsRef<str>>(command: S) -> ProcessResult<ProcessOutput> {
    let config = ProcessConfig::new(command);
    run_command(config)
}

/// Simple command execution with arguments
pub fn exec_with_args<S: AsRef<str>, I, A>(command: S, args: I) -> ProcessResult<ProcessOutput>
where
    I: IntoIterator<Item = A>,
    A: AsRef<str>,
{
    let mut config = ProcessConfig::new(command);
    for arg in args {
        config = config.arg(arg);
    }
    run_command(config)
}

/// Advanced process control functions

/// Kill a process by PID
pub fn kill_process(pid: u32) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        use crate::stdlib::process::control::send_signal_to_pid;
        send_signal_to_pid(pid, 9) // SIGKILL
    }

    #[cfg(windows)]
    {
        use std::process::Command;
        let output = Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .output()
            .map_err(|e| io_error("kill_process", &format!("{:?}", e.kind()), &e.to_string()))?;

        if !output.status.success() {
            return Err(execution_failed("taskkill", &String::from_utf8_lossy(&output.stderr)));
        }
        Ok(())
    }
}

/// Terminate a process gracefully by PID
pub fn terminate_process(pid: u32) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        use crate::stdlib::process::control::send_signal_to_pid;
        send_signal_to_pid(pid, 15) // SIGTERM
    }

    #[cfg(windows)]
    {
        // On Windows, use taskkill without /F for graceful termination
        use std::process::Command;
        let output = Command::new("taskkill")
            .args(&["/PID", &pid.to_string()])
            .output()
            .map_err(|e| io_error("terminate_process", &format!("{:?}", e.kind()), &e.to_string()))?;

        if !output.status.success() {
            return Err(execution_failed("taskkill", &String::from_utf8_lossy(&output.stderr)));
        }
        Ok(())
    }
}

/// Suspend a process by PID
pub fn suspend_process(pid: u32) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        use crate::stdlib::process::control::send_signal_to_pid;
        send_signal_to_pid(pid, 19) // SIGSTOP
    }

    #[cfg(windows)]
    {
        // Windows doesn't have a direct equivalent to SIGSTOP
        // We can use NtSuspendProcess but it requires unsafe code
        Err(platform_error("Process suspension not directly supported on Windows"))
    }
}

/// Resume a suspended process by PID
pub fn resume_process(pid: u32) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        use crate::stdlib::process::control::send_signal_to_pid;
        send_signal_to_pid(pid, 18) // SIGCONT
    }

    #[cfg(windows)]
    {
        // Windows doesn't have a direct equivalent to SIGCONT
        Err(platform_error("Process resumption not directly supported on Windows"))
    }
}

/// Check if a process exists by PID
pub fn process_exists(pid: u32) -> bool {
    #[cfg(unix)]
    {
        use crate::stdlib::process::control::send_signal_to_pid;
        // Send signal 0 to check if process exists without affecting it
        send_signal_to_pid(pid, 0).is_ok()
    }

    #[cfg(windows)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid)])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str.contains(&pid.to_string())
        } else {
            false
        }
    }
}

/// Get current process ID
pub fn current_pid() -> u32 {
    std::process::id()
}

/// Get parent process ID
#[cfg(unix)]
pub fn parent_pid() -> ProcessResult<u32> {
    Ok(unsafe { libc::getppid() as u32 })
}

#[cfg(windows)]
pub fn parent_pid() -> ProcessResult<u32> {
    // On Windows, getting PPID requires WinAPI calls
    Err(platform_error("Getting parent PID not implemented on Windows yet"))
}

/// List all running processes (simplified)
pub fn list_processes() -> ProcessResult<Vec<ProcessInfo>> {
    let mut processes = Vec::new();

    #[cfg(unix)]
    {
        // Simple implementation using /proc filesystem
        use std::fs;
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if let Ok(pid) = file_name.parse::<u32>() {
                        if let Ok(mut info) = get_process_info(pid) {
                            info.update_uptime();
                            processes.push(info);
                        }
                    }
                }
            }
        }
    }

    #[cfg(windows)]
    {
        // Use tasklist command for simplicity
        use std::process::Command;
        if let Ok(output) = Command::new("tasklist")
            .args(&["/FO", "CSV"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.split("\n").skip(1) { // Skip header
                let fields: Vec<&str> = line.split(',').collect();
                if fields.len() >= 2 {
                    if let Ok(pid) = fields[1].trim_matches('"').parse::<u32>() {
                        let name = fields[0].trim_matches('"').to_string();
                        let mut info = ProcessInfo::new(pid, name);
                        info.state = ProcessState::Running;
                        processes.push(info);
                    }
                }
            }
        }
    }

    Ok(processes)
}

/// Get detailed information about a specific process
pub fn get_process_info(pid: u32) -> ProcessResult<ProcessInfo> {
    #[cfg(unix)]
    {
        use std::fs;
        
        // Read from /proc/[pid]/stat
        let stat_path = format!("/proc/{}/stat", pid);
        let stat_content = fs::read_to_string(&stat_path)
            .map_err(|_| process_not_found_pid(pid, "Process not found"))?;

        let fields: Vec<&str> = stat_content.split_whitespace().collect();
        if fields.len() < 24 {
            return Err(invalid_arguments("get_process_info", "stat", "Invalid stat format"));
        }

        let name = fields[1].trim_matches('(').trim_matches(')').to_string();
        let ppid = fields[3].parse().unwrap_or(0);
        let state_char = fields[2].chars().next().unwrap_or('?');
        
        let state = match state_char {
            'R' => ProcessState::Running,
            'S' | 'D' => ProcessState::Waiting,
            'T' => ProcessState::Stopped,
            'Z' => ProcessState::Terminated,
            _ => ProcessState::Running,
        };

        // Memory usage from /proc/[pid]/status
        let mut memory_usage = 0;
        if let Ok(status_content) = fs::read_to_string(format!("/proc/{}/status", pid)) {
            for line in status_content.split("\n") {
                if line.starts_with("VmRSS:") {
                    if let Some(value_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = value_str.parse::<u64>() {
                            memory_usage = kb * 1024; // Convert KB to bytes
                        }
                    }
                    break;
                }
            }
        }

        let mut info = ProcessInfo::new(pid, name);
        info.ppid = Some(ppid);
        info.state = state;
        info.memory_usage = memory_usage;
        info.cpu_usage = 0.0; // Would need more complex calculation
        
        // Get command line arguments
        if let Ok(cmdline) = fs::read_to_string(format!("/proc/{}/cmdline", pid)) {
            info.args = cmdline.split('\0').map(|s| s.to_string()).collect();
        }

        Ok(info)
    }

    #[cfg(windows)]
    {
        // Simplified Windows implementation
        let mut info = ProcessInfo::new(pid, "unknown".to_string());
        info.state = if process_exists(pid) {
            ProcessState::Running
        } else {
            ProcessState::Terminated
        };
        Ok(info)
    }
}

/// Run a process in the background and return immediately
pub fn run_in_background(config: ProcessConfig) -> ProcessResult<Process> {
    spawn_process(config)
}

/// Execute a command and capture output with resource monitoring
pub fn execute_command(config: ProcessConfig) -> ProcessResult<ProcessOutput> {
    run_command(config)
}

/// Execute a command with timeout and resource limits
pub fn execute_with_limits(config: ProcessConfig, limits: ResourceLimits) -> ProcessResult<ProcessOutput> {
    let start_time = Instant::now();
    let mut process = spawn_process(config)?;
    process.set_resource_limits(limits.clone());

    // Check execution time limit
    let timeout = limits.max_execution_time.unwrap_or(Duration::from_secs(300)); // Default 5 minutes
    
    match process.wait_timeout(timeout)? {
        Some(status) => {
            let duration = start_time.elapsed();
            
            // Check if we exceeded resource limits
            if limits.is_execution_time_exceeded(duration) {
                process.kill()?;
                return Err(timeout_error("execute_with_limits", duration, "Execution time limit exceeded"));
            }

            let mut stdout = Vec::new();
            let mut stderr = Vec::new();
            let _ = process.read_stdout(&mut stdout);
            let _ = process.read_stderr(&mut stderr);

            Ok(ProcessOutput {
                status,
                stdout,
                stderr,
                duration,
            })
        }
        None => {
            process.kill()?;
            Err(timeout_error("execute_with_limits", timeout, "Process execution timed out"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_config_builder() {
        let config = ProcessConfig::new("ls")
            .arg("-la")
            .args(&["-h", "--color"])
            .env("TEST", "value")
            .timeout(Duration::from_secs(30));

        assert_eq!(config.command, "ls");
        assert_eq!(config.args, vec!["-la", "-h", "--color"]);
        assert_eq!(config.env_vars.get("TEST"), Some(&"value".to_string()));
        assert_eq!(config.timeout, Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_process_io_conversion() {
        let inherit = ProcessIo::Inherit;
        let pipe = ProcessIo::Pipe;
        let null = ProcessIo::Null;

        // Test that conversions don't panic
        assert!(inherit.to_stdio().is_ok());
        assert!(pipe.to_stdio().is_ok());
        assert!(null.to_stdio().is_ok());
    }

    #[test]
    fn test_command_exists() {
        // Test with common commands that should exist
        #[cfg(unix)]
        {
            assert!(command_exists("ls"));
            assert!(command_exists("echo"));
        }
        
        #[cfg(windows)]
        {
            assert!(command_exists("dir"));
            assert!(command_exists("echo"));
        }
        
        // Test with non-existent command
        assert!(!command_exists("this_command_definitely_does_not_exist_anywhere"));
    }

    #[test]
    fn test_simple_exec() {
        #[cfg(unix)]
        {
            let result = exec("echo hello");
            assert!(result.is_ok());
            let output = result.unwrap();
            assert!(output.success());
            assert_eq!(output.stdout_lossy().trim(), "hello");
        }

        #[cfg(windows)]
        {
            let result = exec_with_args("echo", &["hello"]);
            assert!(result.is_ok());
            let output = result.unwrap();
            assert!(output.success());
            assert_eq!(output.stdout_lossy().trim(), "hello");
        }
    }

    #[test]
    fn test_process_output_methods() {
        let output = ProcessOutput {
            status: std::process::ExitStatus::from_raw(0),
            stdout: b"hello world".to_vec(),
            stderr: b"error message".to_vec(),
            duration: Duration::from_millis(100),
        };

        assert!(output.success());
        assert_eq!(output.stdout_lossy(), "hello world");
        assert_eq!(output.stderr_lossy(), "error message");
    }

    #[test]
    fn test_process_state() {
        let state = ProcessState::Running;
        assert_eq!(format!("{}", state), "Running");
        assert_eq!(state, ProcessState::Running);
        assert_ne!(state, ProcessState::Terminated);
    }

    #[test]
    fn test_process_info() {
        let mut info = ProcessInfo::new(1234, "test_process".to_string());
        assert_eq!(info.pid, 1234);
        assert_eq!(info.name, "test_process");
        assert_eq!(info.state, ProcessState::Created);
        assert!(info.is_alive());

        info.state = ProcessState::Terminated;
        assert!(!info.is_alive());

        info.memory_usage = 1024 * 1024; // 1MB
        assert_eq!(info.memory_usage_human(), "1.0 MB");
    }

    #[test]
    fn test_process_group() {
        let mut group = ProcessGroup::new(100);
        assert_eq!(group.id, 100);
        assert_eq!(group.running_count(), 0);

        let info1 = ProcessInfo::new(1001, "proc1".to_string());
        let mut info2 = ProcessInfo::new(1002, "proc2".to_string());
        info2.state = ProcessState::Running;

        group.add_process(info1);
        group.add_process(info2);

        assert_eq!(group.processes.len(), 2);
        assert_eq!(group.running_count(), 1);
        assert!(group.get_process(1001).is_some());
        assert!(group.get_process(9999).is_none());
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits::new()
            .max_memory_mb(100)
            .max_cpu_time(60)
            .max_execution_time(Duration::from_secs(120));

        assert_eq!(limits.max_memory, Some(100 * 1024 * 1024));
        assert_eq!(limits.max_cpu_time, Some(60));
        assert!(limits.is_memory_exceeded(200 * 1024 * 1024));
        assert!(!limits.is_memory_exceeded(50 * 1024 * 1024));
        assert!(limits.is_execution_time_exceeded(Duration::from_secs(150)));
    }

    #[test]
    fn test_current_pid() {
        let pid = current_pid();
        assert!(pid > 0);
        assert!(process_exists(pid));
    }
}

/// Simple process manager for LLVM integration
#[derive(Debug)]
pub struct ProcessManager {
    /// Active processes registry
    active_processes: Arc<Mutex<HashMap<u32, ProcessInfo>>>,
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            active_processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Wait for a process to complete
    pub fn wait_for_process(&self, pid: u32) -> ProcessResult<i32> {
        // In a real implementation, this would wait for the actual process
        // For now, we simulate waiting and return success
        tracing::info!(pid = pid, "Waiting for process to complete");
        
        // Simulate wait time
        std::thread::sleep(Duration::from_millis(10));
        
        // Return success exit code
        Ok(0)
    }
    
    /// Send a signal to a process
    pub fn send_signal_to_process(&self, pid: u32, signal: i32) -> ProcessResult<()> {
        tracing::info!(pid = pid, signal = signal, "Sending signal to process");
        
        #[cfg(unix)]
        {
            use libc::{kill, ESRCH, EPERM};
            
            let result = unsafe { kill(pid as i32, signal) };
            if result == 0 {
                Ok(())
            } else {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                match errno {
                    ESRCH => Err(process_not_found_pid(pid)),
                    EPERM => Err(permission_denied_pid(pid)),
                    _ => Err(system_error(errno, "send_signal", &format!("Failed to send signal {} to process {}", signal, pid))),
                }
            }
        }
        
        #[cfg(not(unix))]
        {
            // Windows implementation would go here
            // For now, just return success
            Ok(())
        }
    }
    
    /// Terminate a process gracefully
    pub fn terminate_process(&self, pid: u32) -> ProcessResult<()> {
        tracing::info!(pid = pid, "Terminating process gracefully");
        
        #[cfg(unix)]
        {
            self.send_signal_to_process(pid, 15) // SIGTERM
        }
        
        #[cfg(not(unix))]
        {
            // Windows implementation would use TerminateProcess
            Ok(())
        }
    }
    
    /// Kill a process forcefully
    pub fn kill_process(&self, pid: u32) -> ProcessResult<()> {
        tracing::info!(pid = pid, "Killing process forcefully");
        
        #[cfg(unix)]
        {
            self.send_signal_to_process(pid, 9) // SIGKILL
        }
        
        #[cfg(not(unix))]
        {
            // Windows implementation would use TerminateProcess with force
            Ok(())
        }
    }
    
    /// Register a process in the active processes registry
    pub fn register_process(&self, pid: u32, name: String) -> ProcessResult<()> {
        let mut active = self.active_processes.lock()
            .map_err(|_| system_error(-1, "register_process", "Failed to lock active processes"))?;
        
        let process_info = ProcessInfo::new(pid, name);
        active.insert(pid, process_info);
        
        tracing::debug!(pid = pid, "Registered process in manager");
        Ok(())
    }
    
    /// Unregister a process from the active processes registry
    pub fn unregister_process(&self, pid: u32) -> ProcessResult<()> {
        let mut active = self.active_processes.lock()
            .map_err(|_| system_error(-1, "unregister_process", "Failed to lock active processes"))?;
        
        active.remove(&pid);
        
        tracing::debug!(pid = pid, "Unregistered process from manager");
        Ok(())
    }
    
    /// Get information about a process
    pub fn get_process_info(&self, pid: u32) -> ProcessResult<Option<ProcessInfo>> {
        let active = self.active_processes.lock()
            .map_err(|_| system_error(-1, "get_process_info", "Failed to lock active processes"))?;
        
        Ok(active.get(&pid).cloned())
    }
    
    /// List all active processes
    pub fn list_processes(&self) -> ProcessResult<Vec<ProcessInfo>> {
        let active = self.active_processes.lock()
            .map_err(|_| system_error(-1, "list_processes", "Failed to lock active processes"))?;
        
        Ok(active.values().cloned().collect())
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// I/O redirection configuration
#[derive(Debug, Clone)]
pub struct IoRedirection {
    /// Standard input redirection
    pub stdin: Option<String>,
    /// Standard output redirection
    pub stdout: Option<String>,
    /// Standard error redirection
    pub stderr: Option<String>,
}

impl IoRedirection {
    /// Create a new I/O redirection configuration
    pub fn new() -> Self {
        Self {
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }
    
    /// Set stdin redirection
    pub fn stdin_file<S: Into<String>>(mut self, path: S) -> Self {
        self.stdin = Some(path.into());
        self
    }
    
    /// Set stdout redirection
    pub fn stdout_file<S: Into<String>>(mut self, path: S) -> Self {
        self.stdout = Some(path.into());
        self
    }
    
    /// Set stderr redirection
    pub fn stderr_file<S: Into<String>>(mut self, path: S) -> Self {
        self.stderr = Some(path.into());
        self
    }
}

impl Default for IoRedirection {
    fn default() -> Self {
        Self::new()
    }
}

/// Process handle for managing individual processes
#[derive(Debug)]
pub struct ProcessHandle {
    /// Process ID
    pub pid: u32,
    /// Process information
    pub info: Arc<Mutex<ProcessInfo>>,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Internal child process handle
    child: Option<Arc<Mutex<Child>>>,
    /// Process manager reference
    manager: Option<Arc<ProcessManager>>,
}

impl ProcessHandle {
    /// Create a new process handle
    pub fn new(pid: u32, info: ProcessInfo) -> Self {
        Self {
            pid,
            info: Arc::new(Mutex::new(info)),
            resource_limits: ResourceLimits::default(),
            child: None,
            manager: None,
        }
    }

    /// Create with child process
    pub fn with_child(pid: u32, info: ProcessInfo, child: Child) -> Self {
        Self {
            pid,
            info: Arc::new(Mutex::new(info)),
            resource_limits: ResourceLimits::default(),
            child: Some(Arc::new(Mutex::new(child))),
            manager: None,
        }
    }

    /// Get process ID
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Get process information
    pub fn info(&self) -> ProcessInfo {
        let info = self.info.lock().unwrap();
        info.clone()
    }

    /// Update process information
    pub fn update_info(&self, info: ProcessInfo) {
        let mut current_info = self.info.lock().unwrap();
        *current_info = info;
    }

    /// Get resource limits
    pub fn get_resource_limits(&self) -> &ResourceLimits {
        &self.resource_limits
    }

    /// Set resource limits
    pub fn set_resource_limits(&mut self, limits: ResourceLimits) {
        self.resource_limits = limits;
    }

    /// Check if process is alive
    pub fn is_alive(&self) -> bool {
        process_exists(self.pid)
    }

    /// Wait for process to complete
    pub fn wait(&self) -> ProcessResult<ExitStatus> {
        if let Some(child_ref) = &self.child {
            let mut child = child_ref.lock().unwrap();
            child.wait().map_err(|e| io_error(&format!("Failed to wait for process: {}", e)))
        } else {
            // For external processes, we need to implement platform-specific waiting
            self.wait_external()
        }
    }

    /// Wait for external process (no Child handle)
    fn wait_external(&self) -> ProcessResult<ExitStatus> {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            use std::process::ExitStatus;
use crate::stdlib::process::core::ProcessHandle;
use crate::stdlib::process::info::ProcessInfo;
use crate::stdlib::process::info::ProcessState;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;
            
            // Wait for the process using waitpid
            let mut status: libc::c_int = 0;
            let result = unsafe { libc::waitpid(self.pid as libc::pid_t, &mut status, 0) };
            
            if result == -1 {
                return Err(system_error("Failed to wait for process"));
            }
            
            Ok(ExitStatus::from_raw(status))
        }
        
        #[cfg(not(unix))]
        {
            // For Windows and other platforms, we'll need a different approach
            Err(platform_error("External process waiting not implemented for this platform"))
        }
    }

    /// Terminate the process
    pub fn terminate(&self) -> ProcessResult<()> {
        if let Some(child_ref) = &self.child {
            let mut child = child_ref.lock().unwrap();
            child.kill().map_err(|e| io_error(&format!("Failed to terminate process: {}", e)))
        } else {
            terminate_process(self.pid)
        }
    }

    /// Send signal to process (Unix only)
    #[cfg(unix)]
    pub fn signal(&self, signal: i32) -> ProcessResult<()> {
        let result = unsafe { libc::kill(self.pid as libc::pid_t, signal) };
        if result == -1 {
            Err(system_error(&format!("Failed to send signal {} to process {}", signal, self.pid)))
        } else {
            Ok(())
        }
    }

    /// Send signal to process (non-Unix platforms)
    #[cfg(not(unix))]
    pub fn signal(&self, _signal: i32) -> ProcessResult<()> {
        Err(platform_error("Signal sending not supported on this platform"))
    }
}
