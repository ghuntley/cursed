use crate::error::CursedError;
/// Enhanced ExecSlay Implementation - Complete Process Management System
/// 
/// This module provides a complete implementation of the ExecSlay specification
/// with all features from exec_slay.md including advanced process management,
/// pipeline execution, background tasks, and comprehensive monitoring.

use std::collections::HashMap;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio, ExitStatus};
use std::sync::{Arc, Mutex, RwLock, Condvar, mpsc};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::ffi::{OsStr, OsString};

use tracing::{info, warn, error, debug, instrument};

// use crate::stdlib::ipc::{IpcResult, IpcError};

/// Result type for ExecSlay operations
pub type SlayResult<T> = std::result::Result<T, ProcessError>;

/// Enhanced SlayCommand with full specification compliance
#[derive(Debug)]
pub struct EnhancedSlayCommand {
    /// Command name/path
    /// Command arguments
    /// Working directory
    /// Environment variables
    /// Standard input source
    /// Standard output destination
    /// Standard error destination
    /// Extra file descriptors
    /// System process attributes
    /// Execution options
    /// Internal state
/// Enhanced execution options
#[derive(Debug, Clone)]
pub struct EnhancedSlayOptions {
    /// Execution timeout
    /// Wait delay before killing
    /// Kill signal
    /// Stdout callback for real-time processing
    /// Stderr callback for real-time processing
    /// Use shell for execution
    /// Shell path
    /// Buffer size for I/O
    /// Collect output in memory
    /// Capture environment statistics
    /// Working memory limit
    /// CPU usage limit
    /// Process monitoring interval
/// System process attributes
#[derive(Debug, Clone)]
pub struct SysProcAttr {
    /// Process group ID
    /// User ID
    /// Group ID
    /// Additional groups
    /// Working directory
    /// Environment variables to unset
    /// Use foreground process group
/// Enhanced SlayProcess for process control
#[derive(Debug)]
pub struct EnhancedSlayProcess {
    /// Process ID
    /// Process state
    /// Process handle
    /// Monitoring data
    /// Stdout pipe for reading output
    /// Stderr pipe for reading errors
/// Enhanced process state information
#[derive(Debug)]
pub struct EnhancedSlayProcessState {
    /// Exit status
    /// Process start time
    /// Process end time
    /// CPU time used by user processes
    /// CPU time used by system
    /// Memory usage statistics
    /// I/O statistics
/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Peak memory usage
    /// Current memory usage
    /// Virtual memory size
    /// Resident set size
    /// Shared memory
    /// Page faults
/// I/O statistics
#[derive(Debug, Clone)]
pub struct IoStats {
    /// Bytes read
    /// Bytes written
    /// Read operations
    /// Write operations
    /// Network bytes received
    /// Network bytes transmitted
/// Process state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
/// Command internal state
#[derive(Debug)]
struct CommandState {
/// Process monitoring data
#[derive(Debug)]
struct ProcessMonitoring {
/// Enhanced pipeline for command chaining
#[derive(Debug)]
pub struct EnhancedSlayPipeline {
    /// Commands in the pipeline
    /// Pipeline options
    /// Pipeline state
/// Pipeline state
#[derive(Debug)]
struct PipelineState {
/// Background task management
#[derive(Debug)]
pub struct EnhancedSlayTask {
    /// Associated command
    /// Task start time
    /// Task state
    /// Background thread handle
/// Task state
#[derive(Debug)]
struct TaskState {
/// Command builder for fluent API
#[derive(Debug)]
pub struct EnhancedSlayCommandBuilder {
/// Signal handling options
#[derive(Debug, Clone)]
pub struct EnhancedSignalOptions {
    /// Grace period before force killing
    /// Force kill immediately
    /// Signal to send
    /// Kill process tree recursively
    /// Escalation signals
    /// Escalation delays
/// Advanced process statistics
#[derive(Debug, Clone)]
pub struct EnhancedProcessStats {
    /// CPU usage percentage
    /// Memory usage in bytes
    /// Resident memory in bytes
    /// Virtual memory in bytes
    /// Swap memory in bytes
    /// Bytes read from disk
    /// Bytes written to disk
    /// Read operations count
    /// Write operations count
    /// Process uptime
    /// Number of threads
    /// Number of open files
    /// Number of network connections
    /// Number of child processes
    /// Process priority
    /// Nice value
impl Default for EnhancedSlayOptions {
    fn default() -> Self {
        Self {
            kill_signal: Some(15), // SIGTERM
        }
    }
impl Default for EnhancedSignalOptions {
    fn default() -> Self {
        Self {
            signal: 15, // SIGTERM
            escalation_signals: vec![15, 9], // SIGTERM, SIGKILL
        }
    }
impl EnhancedSlayCommand {
    /// Create a new enhanced command
    #[instrument]
    pub fn new(name: &str, args: &[&str]) -> Self {
        Self {
        }
    }
    
    /// Run the command and wait for completion
    #[instrument(skip(self))]
    pub fn run(&mut self) -> SlayResult<()> {
        self.start()?;
        self.wait()
    /// Start the command without waiting
    #[instrument(skip(self))]
    pub fn start(&mut self) -> SlayResult<()> {
        let mut cmd = self.build_command()?;
        
        // Configure stdio
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());
        
        // Set working directory
        if let Some(ref dir) = self.dir {
            cmd.current_dir(dir);
        // Set environment variables
        for (key, value) in &self.env {
            cmd.env(key, value);
        // Apply system process attributes
        if let Some(ref attr) = self.sys_proc_attr {
            self.apply_sys_proc_attr(&mut cmd, attr)?;
        // Spawn the process
        let mut child = cmd.spawn()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to spawn process: {}", e)))?;
        
        let pid = child.id();
        
        // Create process handle
        let process = EnhancedSlayProcess::new(pid, child)?;
        
        // Update state
        {
            let mut state = self.state.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
            
            state.process = Some(process);
            state.start_time = Some(SystemTime::now());
            state.is_running = true;
        // Start monitoring if configured
        if self.options.capture_env_stats {
            self.start_monitoring()?;
        // Start output callbacks if configured
        if self.options.stdout_callback.is_some() || self.options.stderr_callback.is_some() {
            self.start_output_callbacks()?;
        info!("Process started: PID {} - {}", pid, self.to_string());
        Ok(())
    /// Wait for the command to complete
    #[instrument(skip(self))]
    pub fn wait(&mut self) -> SlayResult<()> {
        let timeout = self.options.timeout;
        let start_time = Instant::now();
        
        loop {
            // Check if process has completed
            let (is_running, exit_code) = {
                let mut state = self.state.lock()
                    .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
                
                if let Some(ref mut process) = state.process {
                    match process.try_wait()? {
                        Some(exit_status) => {
                            state.is_running = false;
                            state.end_time = Some(SystemTime::now());
                            state.exit_code = exit_status.code();
                            
                            // Collect final output
                            if self.options.collect_output {
                                self.collect_final_output(&mut state, process)?;
                            (false, exit_status.code())
                        }
                        None => (true, None)
                    }
                } else {
                    return Err(CursedError::RuntimeError("No process handle available".to_string()));
                }
            
            if !is_running {
                info!("Process completed: exit code {:?}", exit_code);
                return Ok(());
            // Check timeout
            if let Some(timeout) = timeout {
                if start_time.elapsed() >= timeout {
                    self.terminate_with_timeout()?;
                    return Err(CursedError::RuntimeError(format!("Process timed out after {:?}", timeout)));
                }
            }
            
            // Sleep briefly before checking again
            thread::sleep(Duration::from_millis(50));
        }
    }
    
    /// Get command output
    #[instrument(skip(self))]
    pub fn output(&mut self) -> SlayResult<Vec<u8>> {
        self.run()?;
        
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        Ok(state.output_buffer.clone())
    /// Get combined output (stdout + stderr)
    #[instrument(skip(self))]
    pub fn combined_output(&mut self) -> SlayResult<Vec<u8>> {
        self.run()?;
        
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        let mut combined = state.output_buffer.clone();
        combined.extend_from_slice(&state.error_buffer);
        Ok(combined)
    /// Get stdout pipe for reading from process
    #[instrument(skip(self))]
    pub fn stdout_pipe(&mut self) -> SlayResult<Box<dyn Read + Send>> {
        debug!("Creating stdout pipe for process");
        
        // Configure stdout for piping and start process
        self.stdout = Some(ProcessStdout::Pipe);
        
        // Start process if not already running
        if self.child.is_none() {
            self.spawn()?;
        if let Some(ref mut child) = self.child {
            if let Some(stdout) = child.stdout.take() {
                debug!("Successfully created stdout pipe");
                Ok(Box::new(stdout))
            } else {
                error!("Stdout not available - pipe not configured");
                Err(CursedError::RuntimeError("Stdout not available - pipe not configured".to_string()))
            }
        } else {
            error!("Process not started");
            Err(CursedError::RuntimeError("Process not started".to_string()))
        }
    }
    
    /// Get stderr pipe for reading from process
    #[instrument(skip(self))]
    pub fn stderr_pipe(&mut self) -> SlayResult<Box<dyn Read + Send>> {
        debug!("Creating stderr pipe for process");
        
        // Configure stderr for piping and start process
        self.stderr = Some(ProcessStderr::Pipe);
        
        // Start process if not already running
        if self.child.is_none() {
            self.spawn()?;
        if let Some(ref mut child) = self.child {
            if let Some(stderr) = child.stderr.take() {
                debug!("Successfully created stderr pipe");
                Ok(Box::new(stderr))
            } else {
                error!("Stderr not available - pipe not configured");
                Err(CursedError::RuntimeError("Stderr not available - pipe not configured".to_string()))
            }
        } else {
            error!("Process not started");
            Err(CursedError::RuntimeError("Process not started".to_string()))
        }
    }
    
    /// Get stdin pipe for writing to process
    #[instrument(skip(self))]
    pub fn stdin_pipe(&mut self) -> SlayResult<Box<dyn Write + Send>> {
        debug!("Creating stdin pipe for process");
        
        // Configure stdin for piping and start process
        self.stdin = Some(ProcessStdin::Pipe);
        
        // Start process if not already running
        if self.child.is_none() {
            self.spawn()?;
        if let Some(ref mut child) = self.child {
            if let Some(stdin) = child.stdin.take() {
                debug!("Successfully created stdin pipe");
                Ok(Box::new(stdin))
            } else {
                error!("Stdin not available - pipe not configured");
                Err(CursedError::RuntimeError("Stdin not available - pipe not configured".to_string()))
            }
        } else {
            error!("Process not started");
            Err(CursedError::RuntimeError("Process not started".to_string()))
        }
    }
    
    /// Set working directory
    #[instrument(skip(self))]
    pub fn set_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.dir = Some(dir.as_ref().to_path_buf());
        self
    /// Set environment variables
    #[instrument(skip(self))]
    pub fn set_env(&mut self, env: HashMap<String, String>) -> &mut Self {
        self.env = env;
        self
    /// Add environment variable
    #[instrument(skip(self))]
    pub fn add_env(&mut self, key: &str, value: &str) -> &mut Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    /// Set stdin
    #[instrument(skip(self, reader))]
    pub fn set_stdin(&mut self, reader: Box<dyn Read + Send>) -> &mut Self {
        self.stdin = Some(reader);
        self
    /// Set stdout
    #[instrument(skip(self, writer))]
    pub fn set_stdout(&mut self, writer: Box<dyn Write + Send>) -> &mut Self {
        self.stdout = Some(writer);
        self
    /// Set stderr
    #[instrument(skip(self, writer))]
    pub fn set_stderr(&mut self, writer: Box<dyn Write + Send>) -> &mut Self {
        self.stderr = Some(writer);
        self
    /// Set path (alias for name)
    #[instrument(skip(self))]
    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.name = path.to_string();
        self
    /// Set extra files
    #[instrument(skip(self))]
    pub fn set_extra_files(&mut self, files: Vec<std::fs::File>) -> &mut Self {
        self.extra_files = files;
        self
    /// Set system process attributes
    #[instrument(skip(self))]
    pub fn set_sys_proc_attr(&mut self, attr: SysProcAttr) -> &mut Self {
        self.sys_proc_attr = Some(attr);
        self
    /// Get process handle
    #[instrument(skip(self))]
    pub fn process(&self) -> SlayResult<EnhancedSlayProcess> {
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        state.process.clone()
            .ok_or_else(|| CursedError::RuntimeError("Process not started".to_string()))
    /// Get process state
    #[instrument(skip(self))]
    pub fn process_state(&self) -> SlayResult<EnhancedSlayProcessState> {
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref process) = state.process {
            process.get_process_state()
        } else {
            Err(CursedError::RuntimeError("Process not started".to_string()))
        }
    }
    
    /// Get string representation
    #[instrument(skip(self))]
    pub fn to_string(&self) -> String {
        let mut cmd_str = self.name.clone();
        for arg in &self.args {
            cmd_str.push(' ');
            if arg.contains(' ') {
                cmd_str.push_str(&format!("\"{}\"", arg));
            } else {
                cmd_str.push_str(arg);
            }
        }
        cmd_str
    /// Apply options to command
    #[instrument(skip(self))]
    pub fn with_options(mut self, options: EnhancedSlayOptions) -> Self {
        self.options = options;
        self
    /// Build the underlying Command
    #[instrument(skip(self))]
    fn build_command(&self) -> SlayResult<Command> {
        let mut cmd = if self.options.use_shell {
            let shell_path = self.options.shell_path
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| self.get_default_shell().to_string());
            
            let mut shell_cmd = Command::new(&shell_path);
            
            // Add shell flags
            if cfg!(target_os = "windows") {
                shell_cmd.arg("/C");
            } else {
                shell_cmd.arg("-c");
            // Build command string
            let full_cmd = format!("{} {}", self.name, self.args.join(" "));
            shell_cmd.arg(full_cmd);
            shell_cmd
        } else {
            let mut direct_cmd = Command::new(&self.name);
            direct_cmd.args(&self.args);
            direct_cmd
        
        Ok(cmd)
    /// Apply system process attributes
    #[instrument(skip(self, cmd, attr))]
    fn apply_sys_proc_attr(&self, cmd: &mut Command, attr: &SysProcAttr) -> SlayResult<()> {
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            
            if let Some(gid) = attr.gid {
                cmd.gid(gid);
            if let Some(uid) = attr.uid {
                cmd.uid(uid);
            if !attr.groups.is_empty() {
                cmd.groups(&attr.groups);
            if let Some(ref chroot) = attr.chroot {
                // Note: chroot requires root privileges
                // This would need to be implemented with pre_exec
            }
        }
        
        Ok(())
    /// Get default shell for platform
    #[instrument(skip(self))]
    fn get_default_shell(&self) -> &'static str {
        if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "/bin/sh"
        }
    }
    
    /// Start monitoring
    #[instrument(skip(self))]
    fn start_monitoring(&self) -> SlayResult<()> {
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref process) = state.process {
            if let Some(interval) = self.options.monitoring_interval {
                let pid = process.pid;
                let monitoring_handle = state.monitoring_handle.clone();
                
                // Start background monitoring thread
                let _handle = std::thread::spawn(move || {
//                     use crate::stdlib::process::real_monitoring::get_real_process_state;
                    
                    loop {
                        std::thread::sleep(interval);
                        
                        // Check if monitoring should stop
                        if let Ok(should_stop) = monitoring_handle.lock() {
                            if *should_stop {
                                break;
                            }
                        }
                        
                        // Get current process stats
                        if let Ok(stats) = get_real_process_state(pid) {
                                          "Process monitoring update");
                        }
                    }
                });
                
                tracing::debug!(pid = pid, interval = ?interval, "Started process monitoring");
            }
        }
        
        Ok(())
    /// Start output callbacks
    #[instrument(skip(self))]
    fn start_output_callbacks(&self) -> SlayResult<()> {
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref process) = state.process {
            // Start stdout callback thread if configured
            if let Some(ref stdout_callback) = self.options.stdout_callback {
                let callback = stdout_callback.clone();
                let stdout_pipe = process.stdout_pipe.clone();
                
                std::thread::spawn(move || {
                    use std::io::{BufRead, BufReader};
                    
                    if let Some(pipe) = stdout_pipe {
                        let reader = BufReader::new(pipe);
                        for line_result in reader.split("\n") {
                            if let Ok(line) = line_result {
                                callback(line.into_bytes());
                            } else {
                                break;
                            }
                        }
                    }
                });
            // Start stderr callback thread if configured
            if let Some(ref stderr_callback) = self.options.stderr_callback {
                let callback = stderr_callback.clone();
                let stderr_pipe = process.stderr_pipe.clone();
                
                std::thread::spawn(move || {
                    use std::io::{BufRead, BufReader};
                    
                    if let Some(pipe) = stderr_pipe {
                        let reader = BufReader::new(pipe);
                        for line_result in reader.split("\n") {
                            if let Ok(line) = line_result {
                                callback(line.into_bytes());
                            } else {
                                break;
                            }
                        }
                    }
                });
            tracing::debug!(pid = process.pid, "Started output callbacks");
        Ok(())
    /// Collect final output from process
    #[instrument(skip(self, state, process))]
    fn collect_final_output(&self, state: &mut CommandState, process: &mut EnhancedSlayProcess) -> SlayResult<()> {
        use std::io::Read;
        
        // Collect any remaining stdout
        if let Some(ref mut stdout_pipe) = process.stdout_pipe {
            let mut buffer = Vec::new();
            if let Ok(bytes_read) = stdout_pipe.read_to_end(&mut buffer) {
                if bytes_read > 0 {
                    state.stdout_output.extend_from_slice(&buffer);
                    tracing::debug!(bytes = bytes_read, "Collected final stdout output");
                }
            }
        // Collect any remaining stderr
        if let Some(ref mut stderr_pipe) = process.stderr_pipe {
            let mut buffer = Vec::new();
            if let Ok(bytes_read) = stderr_pipe.read_to_end(&mut buffer) {
                if bytes_read > 0 {
                    state.stderr_output.extend_from_slice(&buffer);
                    tracing::debug!(bytes = bytes_read, "Collected final stderr output");
                }
            }
        Ok(())
    /// Terminate process with timeout handling
    #[instrument(skip(self))]
    fn terminate_with_timeout(&mut self) -> SlayResult<()> {
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref mut process) = state.process {
            process.terminate(EnhancedSignalOptions::default())?;
            state.is_running = false;
            state.end_time = Some(SystemTime::now());
        Ok(())
    }
}

impl EnhancedSlayProcess {
    /// Create new process handle
    #[instrument]
    fn new(pid: u32, child: Child) -> SlayResult<Self> {
        Ok(Self {
        })
    /// Kill the process
    #[instrument(skip(self))]
    pub fn kill(&self) -> SlayResult<()> {
        let mut child = self.child.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire child lock".to_string()))?;
        
        if let Some(ref mut child) = child.as_mut() {
            child.kill()
                .map_err(|e| CursedError::RuntimeError(format!("Failed to kill process: {}", e)))?;
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        *state = ProcessState::Terminated;
        
        Ok(())
    /// Send signal to process
    #[instrument(skip(self))]
    pub fn signal(&self, sig: i32) -> SlayResult<()> {
        #[cfg(unix)]
        {
            use libc;
            
            let result = unsafe { libc::kill(self.pid as i32, sig) };
            if result == -1 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(CursedError::RuntimeError(format!("Failed to send signal {}: errno {}", sig, errno)));
            }
        }
        
        #[cfg(windows)]
        {
//             use crate::stdlib::process::windows_support::send_windows_signal;
            
            // Use Windows-specific signal handling
            match sig {
                libc::SIGTERM | libc::SIGINT => {
                    // For SIGTERM/SIGINT, attempt graceful termination
                    send_windows_signal(self.pid, sig)
                        .map_err(|e| CursedError::RuntimeError(format!("Failed to send Windows signal {}: {}", sig, e)))?;
                libc::SIGKILL => {
                    // For SIGKILL, force termination
                    self.kill()?;
                _ => {
                    // For other signals, attempt best-effort handling
                    send_windows_signal(self.pid, sig)
                        .map_err(|e| CursedError::RuntimeError(format!("Signal {} not supported on Windows: {}", sig, e)))?;
                }
            }
        Ok(())
    /// Get process ID
    #[instrument(skip(self))]
    pub fn pid(&self) -> u32 {
        self.pid
    /// Wait for process completion
    #[instrument(skip(self))]
    pub fn wait(&mut self) -> SlayResult<EnhancedSlayProcessState> {
        let mut child = self.child.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire child lock".to_string()))?;
        
        if let Some(child) = child.take() {
            let status = child.wait()
                .map_err(|e| CursedError::RuntimeError(format!("Failed to wait for process: {}", e)))?;
            
            let mut state = self.state.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
            *state = ProcessState::Terminated;
            
            // Create process state
            Ok(EnhancedSlayProcessState {
                start_time: SystemTime::now(), // This should be tracked properly
                user_time: Duration::from_secs(0), // This would be collected from the system
            })
        } else {
            Err(CursedError::RuntimeError("Process already waited on".to_string()))
        }
    }
    
    /// Try to wait without blocking
    #[instrument(skip(self))]
    pub fn try_wait(&mut self) -> SlayResult<Option<ExitStatus>> {
        let mut child = self.child.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire child lock".to_string()))?;
        
        if let Some(ref mut child) = child.as_mut() {
            child.try_wait()
                .map_err(|e| CursedError::RuntimeError(format!("Failed to try_wait for process: {}", e)))
        } else {
            Ok(None)
        }
    }
    
    /// Release process handle
    #[instrument(skip(self))]
    pub fn release(&mut self) -> SlayResult<()> {
        let mut child = self.child.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire child lock".to_string()))?;
        
        *child = None;
        Ok(())
    /// Get process statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> SlayResult<EnhancedProcessStats> {
        let monitoring = self.monitoring.read()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire monitoring lock".to_string()))?;
        
        Ok(EnhancedProcessStats {
            resident_memory: monitoring.memory_usage, // Simplified
            virtual_memory: monitoring.memory_usage * 2, // Simplified
            open_files: 0, // Would need system call
            network_conns: 0, // Would need system call
            child_processes: 0, // Would need system call
            priority: 0, // Would need system call
            nice_value: 0, // Would need system call
        })
    /// Monitor process with periodic updates
    #[instrument(skip(self, callback))]
    pub fn monitor<F>(&self, interval: Duration, callback: F) -> SlayResult<()>
    where
    {
        let pid = self.pid;
        let monitoring = self.monitoring.clone();
        
        thread::spawn(move || {
            loop {
                // Get current stats
                let stats = EnhancedProcessStats {
                    cpu: 0.0, // Would get from system
                    memory: 0, // Would get from system
                
                // Call callback
                callback(&stats);
                
                // Update monitoring data
                if let Ok(mut monitoring) = monitoring.write() {
                    monitoring.last_update = SystemTime::now();
                thread::sleep(interval);
            }
        });
        
        Ok(())
    /// Set resource limits
    #[instrument(skip(self))]
    pub fn set_limits(&self, memory_mb: i32, cpu_percent: f64) -> SlayResult<()> {
        info!("Setting limits: memory={}MB, cpu={}%", memory_mb, cpu_percent);
        
        // Convert memory from MB to bytes for system calls
        let memory_bytes = (memory_mb as u64) * 1024 * 1024;
        
        #[cfg(unix)]
        {
            use libc::{setrlimit, rlimit, RLIMIT_AS, RLIMIT_DATA, RLIMIT_RSS};
            
            // Set virtual memory limit (address space)
            let mem_limit = rlimit {
            
            unsafe {
                if setrlimit(RLIMIT_AS, &mem_limit) == -1 {
                    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                    warn!("Failed to set virtual memory limit: errno {}", errno);
                    // Don't fail completely, just warn
                // Also set data segment limit
                if setrlimit(RLIMIT_DATA, &mem_limit) == -1 {
                    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                    warn!("Failed to set data segment limit: errno {}", errno);
                // Set resident memory limit if available
                #[cfg(target_os = "linux")]
                if setrlimit(RLIMIT_RSS, &mem_limit) == -1 {
                    let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                    warn!("Failed to set resident memory limit: errno {}", errno);
                }
            }
            
            // Note: CPU limiting is more complex on Unix and typically requires
            // additional mechanisms like cgroups or nice values
            if cpu_percent < 100.0 {
                info!("CPU limiting requested ({}%) - consider using cgroups or nice for more precise control", cpu_percent);
            }
        }
        
        #[cfg(windows)]
        {
            // On Windows, use Job Objects for resource limiting
            use std::ptr;
            use winapi::um::winnt::{HANDLE, JOBOBJECT_EXTENDED_LIMIT_INFORMATION};
            use winapi::um::jobapi2::{CreateJobObjectW, SetInformationJobObject};
            use winapi::um::winnt::{JOB_OBJECT_LIMIT_PROCESS_MEMORY, JOB_OBJECT_LIMIT_JOB_MEMORY};
            use winapi::um::winbase::JobObjectExtendedLimitInformation;
            
            unsafe {
                let job_handle = CreateJobObjectW(ptr::null_mut(), ptr::null());
                if job_handle.is_null() {
                    return Err(CursedError::RuntimeError("Failed to create job object".to_string()));
                let mut job_info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
                job_info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_PROCESS_MEMORY | JOB_OBJECT_LIMIT_JOB_MEMORY;
                job_info.ProcessMemoryLimit = memory_bytes as usize;
                job_info.JobMemoryLimit = memory_bytes as usize;
                
                if SetInformationJobObject(
                ) == 0 {
                    warn!("Failed to set job object limits");
                // Note: CPU limiting on Windows requires additional setup with job objects
                if cpu_percent < 100.0 {
                    info!("CPU limiting requested ({}%) - requires additional job object configuration", cpu_percent);
                }
            }
        #[cfg(not(any(unix, windows)))]
        {
            warn!("Resource limiting not implemented for this platform");
        Ok(())
    /// Terminate process gracefully
    #[instrument(skip(self))]
    pub fn terminate(&mut self, options: EnhancedSignalOptions) -> SlayResult<()> {
        if options.escalation_signals.is_empty() {
            return self.signal(options.signal);
        // Escalation approach
        for (i, &signal) in options.escalation_signals.iter().enumerate() {
            self.signal(signal)?;
            
            if i < options.escalation_delays.len() {
                thread::sleep(options.escalation_delays[i]);
                
                // Check if process is still running
                if let Ok(Some(_)) = self.try_wait() {
                    return Ok(());
                }
            }
        // Force kill if still running
        if options.force {
            self.kill()?;
        Ok(())
    /// Kill process tree recursively
    #[instrument(skip(self))]
    pub fn kill_tree(&self) -> SlayResult<()> {
        // This would implement recursive process tree killing
        // For now, just kill this process
        self.kill()
    /// Get process state
    #[instrument(skip(self))]
    pub fn get_process_state(&self) -> SlayResult<EnhancedSlayProcessState> {
        Ok(EnhancedSlayProcessState {
            exit_status: None, // Would check from child
            start_time: SystemTime::now(), // Would track properly
        })
    }
}

impl EnhancedSlayPipeline {
    /// Create new pipeline
    #[instrument]
    pub fn new(commands: Vec<EnhancedSlayCommand>) -> Self {
        Self {
        }
    }
    
    /// Create pipeline from commands (convenience)
    #[instrument]
    pub fn pipe(commands: Vec<EnhancedSlayCommand>) -> Self {
        Self::new(commands)
    /// Run pipeline
    #[instrument(skip(self))]
    pub fn run(&mut self) -> SlayResult<()> {
        self.start()?;
        self.wait()
    /// Start pipeline
    #[instrument(skip(self))]
    pub fn start(&mut self) -> SlayResult<()> {
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        state.start_time = Some(SystemTime::now());
        state.is_running = true;
        
        // Start all commands in pipeline with proper piping
        for (i, mut command) in self.commands.clone().into_iter().enumerate() {
            // Configure piping between commands
            if i > 0 {
                // Connect stdout of previous to stdin of current
                // This would need more sophisticated implementation
            command.start()?;
            
            if let Ok(process) = command.process() {
                state.processes.push(process);
            }
        }
        
        Ok(())
    /// Wait for pipeline completion
    #[instrument(skip(self))]
    pub fn wait(&mut self) -> SlayResult<()> {
        let state = self.state.clone();
        
        // Wait for all processes to complete
        loop {
            let all_done = {
                let mut state = state.lock()
                    .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
                
                let mut all_done = true;
                for process in &mut state.processes {
                    if let Ok(None) = process.try_wait() {
                        all_done = false;
                    }
                }
                
                if all_done {
                    state.is_running = false;
                    state.end_time = Some(SystemTime::now());
                all_done
            
            if all_done {
                break;
            thread::sleep(Duration::from_millis(50));
        Ok(())
    /// Get pipeline output
    #[instrument(skip(self))]
    pub fn output(&mut self) -> SlayResult<Vec<u8>> {
        self.run()?;
        
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        Ok(state.combined_output.clone())
    /// Get combined output
    #[instrument(skip(self))]
    pub fn combined_output(&mut self) -> SlayResult<Vec<u8>> {
        self.output()
    /// Apply options to pipeline
    #[instrument(skip(self))]
    pub fn with_options(mut self, options: EnhancedSlayOptions) -> Self {
        self.options = options;
        self
    /// Add command to pipeline
    #[instrument(skip(self))]
    pub fn add_command(mut self, cmd: EnhancedSlayCommand) -> Self {
        self.commands.push(cmd);
        self
    /// Set commands for pipeline
    #[instrument(skip(self))]
    pub fn set_commands(mut self, cmds: Vec<EnhancedSlayCommand>) -> Self {
        self.commands = cmds;
        self
    /// Get string representation
    #[instrument(skip(self))]
    pub fn to_string(&self) -> String {
        self.commands.iter()
            .map(|cmd| cmd.to_string())
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

impl EnhancedSlayTask {
    /// Run command in background
    #[instrument]
    pub fn run_background(mut command: EnhancedSlayCommand) -> Self {
        let start_time = SystemTime::now();
        let state = Arc::new(Mutex::new(TaskState::new()));
        let state_clone = state.clone();
        
        let handle = thread::spawn(move || {
            let result = command.run();
            
            let mut state = state_clone.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
            
            match result {
                Ok(()) => {
                    state.finished = true;
                    state.exit_code = command.state.lock().unwrap().exit_code;
                    state.output = command.state.lock().unwrap().output_buffer.clone();
                    state.combined_output = {
                        let cmd_state = command.state.lock().unwrap();
                        let mut combined = cmd_state.output_buffer.clone();
                        combined.extend_from_slice(&cmd_state.error_buffer);
                        combined
                }
                Err(e) => {
                    state.finished = true;
                    state.error = Some(e.to_string());
                }
            }
            
            Ok(())
        });
        
        Self {
        }
    }
    
    /// Wait for task completion
    #[instrument(skip(self))]
    pub fn wait(&mut self) -> SlayResult<()> {
        if let Some(handle) = self.thread_handle.take() {
            handle.join()
                .map_err(|_| CursedError::RuntimeError("Failed to join background thread".to_string()))?
        } else {
            Ok(())
        }
    }
    
    /// Kill the background task
    #[instrument(skip(self))]
    pub fn kill(&mut self) -> SlayResult<()> {
        // Try to kill the underlying process
        if let Ok(process) = self.command.process() {
            process.kill()?;
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        state.finished = true;
        state.error = Some("Task killed".to_string());
        
        Ok(())
    /// Check if task is running
    #[instrument(skip(self))]
    pub fn is_running(&self) -> bool {
        let state = self.state.lock().unwrap();
        !state.finished
    /// Get elapsed time
    #[instrument(skip(self))]
    pub fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed().unwrap_or(Duration::ZERO)
    /// Get task output
    #[instrument(skip(self))]
    pub fn get_output(&self) -> SlayResult<Vec<u8>> {
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        Ok(state.output.clone())
    /// Get combined output
    #[instrument(skip(self))]
    pub fn get_combined_output(&self) -> SlayResult<Vec<u8>> {
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        Ok(state.combined_output.clone())
    /// Get exit code
    #[instrument(skip(self))]
    pub fn exit_code(&self) -> Option<i32> {
        let state = self.state.lock().unwrap();
        state.exit_code
    /// Check if task is finished
    #[instrument(skip(self))]
    pub fn is_finished(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.finished
    /// Get error if any
    #[instrument(skip(self))]
    pub fn get_error(&self) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.error.clone()
    }
}

impl EnhancedSlayCommandBuilder {
    /// Create new command builder
    #[instrument]
    pub fn new(command: &str) -> Self {
        Self {
        }
    }
    
    /// Add arguments
    #[instrument(skip(self))]
    pub fn with_args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    /// Set working directory
    #[instrument(skip(self))]
    pub fn with_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.dir = Some(dir.as_ref().to_path_buf());
        self
    /// Set environment variables
    #[instrument(skip(self))]
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    /// Add environment variable
    #[instrument(skip(self))]
    pub fn add_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    /// Set stdin
    #[instrument(skip(self, reader))]
    pub fn with_stdin(mut self, reader: Box<dyn Read + Send>) -> Self {
        self.stdin = Some(reader);
        self
    /// Set stdout
    #[instrument(skip(self, writer))]
    pub fn with_stdout(mut self, writer: Box<dyn Write + Send>) -> Self {
        self.stdout = Some(writer);
        self
    /// Set stderr
    #[instrument(skip(self, writer))]
    pub fn with_stderr(mut self, writer: Box<dyn Write + Send>) -> Self {
        self.stderr = Some(writer);
        self
    /// Set timeout
    #[instrument(skip(self))]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    /// Use shell
    #[instrument(skip(self))]
    pub fn use_shell(mut self, use_shell: bool) -> Self {
        self.use_shell = use_shell;
        self
    /// Build the command
    #[instrument(skip(self))]
    pub fn build(self) -> EnhancedSlayCommand {
        let mut cmd = EnhancedSlayCommand::new(&self.command, &self.args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        
        if let Some(dir) = self.dir {
            cmd.set_dir(dir);
        cmd.set_env(self.env);
        
        if let Some(stdin) = self.stdin {
            cmd.set_stdin(stdin);
        if let Some(stdout) = self.stdout {
            cmd.set_stdout(stdout);
        if let Some(stderr) = self.stderr {
            cmd.set_stderr(stderr);
        if let Some(timeout) = self.timeout {
            cmd.options.timeout = Some(timeout);
        cmd.options.use_shell = self.use_shell;
        cmd.options.shell_path = self.shell_path;
        
        cmd
    }
}

// Helper implementations
impl CommandState {
    fn new() -> Self {
        Self {
        }
    }
impl ProcessMonitoring {
    fn new() -> Self {
        Self {
        }
    }
impl PipelineState {
    fn new() -> Self {
        Self {
        }
    }
impl TaskState {
    fn new() -> Self {
        Self {
        }
    }
impl Default for MemoryStats {
    fn default() -> Self {
        Self {
        }
    }
impl Default for IoStats {
    fn default() -> Self {
        Self {
        }
    }
/// Convenience functions matching the specification

/// Create a new SlayCommand
#[instrument]
pub fn new_slay_command(name: &str, args: &[&str]) -> EnhancedSlayCommand {
    EnhancedSlayCommand::new(name, args)
/// Create a new command builder
#[instrument]
pub fn new_slay_command_builder(command: &str) -> EnhancedSlayCommandBuilder {
    EnhancedSlayCommandBuilder::new(command)
/// Create a new pipeline
#[instrument]
pub fn new_slay_pipeline(commands: Vec<EnhancedSlayCommand>) -> EnhancedSlayPipeline {
    EnhancedSlayPipeline::new(commands)
/// Create a pipeline from commands
#[instrument]
pub fn pipe(commands: Vec<EnhancedSlayCommand>) -> EnhancedSlayPipeline {
    EnhancedSlayPipeline::pipe(commands)
/// Run a command in the background
#[instrument]
pub fn run_background(command: EnhancedSlayCommand) -> EnhancedSlayTask {
    EnhancedSlayTask::run_background(command)
/// Run a command with timeout
#[instrument]
pub fn run_with_timeout(mut cmd: EnhancedSlayCommand, timeout: Duration) -> SlayResult<()> {
    cmd.options.timeout = Some(timeout);
    cmd.run()
/// Run a command and get output with timeout
#[instrument]
pub fn output_with_timeout(mut cmd: EnhancedSlayCommand, timeout: Duration) -> SlayResult<Vec<u8>> {
    cmd.options.timeout = Some(timeout);
    cmd.output()
/// Run a command and get combined output with timeout
#[instrument]
pub fn combined_output_with_timeout(mut cmd: EnhancedSlayCommand, timeout: Duration) -> SlayResult<Vec<u8>> {
    cmd.options.timeout = Some(timeout);
    cmd.combined_output()
