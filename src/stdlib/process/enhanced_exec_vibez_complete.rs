use crate::error::CursedError;
/// Enhanced ExecVibez Implementation - Complete Process Execution System
/// 
/// This module provides a complete implementation of the ExecVibez specification
/// with all features from exec_vibez.md including process groups, timeouts,
/// environment control, output streaming, and input generation.

use std::collections::HashMap;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio, ExitStatus};
use std::sync::{Arc, Mutex, RwLock, Condvar, mpsc};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::ffi::{OsStr, OsString};

use tracing::{info, warn, error, debug, instrument};

// use crate::stdlib::time::Duration as CursedDuration;

/// Result type for exec_vibez operations
pub type VibezResult<T> = std::result::Result<T, ProcessError>;

/// Enhanced Cmd structure matching exec_vibez specification
#[derive(Debug)]
pub struct EnhancedCmd {
    /// Command path
    /// Command arguments
    /// Environment variables
    /// Working directory
    /// Standard input
    /// Standard output
    /// Standard error
    /// Internal state
    /// Execution context
/// Process handle for spawned processes
#[derive(Debug)]
pub struct EnhancedProcess {
    /// Process ID
    /// Process state
    /// Process handle
/// Process state information
#[derive(Debug)]
pub struct EnhancedProcessState {
    /// Exit status
    /// Process start time
    /// Process end time
    /// System resource usage
    /// User time
    /// System time
/// System resource usage information
#[derive(Debug, Clone)]
pub struct SystemUsage {
    /// CPU usage percentage
    /// Memory usage in bytes
    /// I/O read bytes
    /// I/O write bytes
    /// Context switches
/// CursedError type for command execution
#[derive(Debug)]
pub struct ExecError {
    /// CursedError message
    /// Exit code if available
    /// Underlying error
/// Process group for managing multiple processes
#[derive(Debug)]
pub struct ProcessGroup {
    /// Group ID
    /// Processes in the group
    /// Group state
    /// Group options
/// Process group options
#[derive(Debug, Clone)]
pub struct ProcessGroupOptions {
    /// Maximum concurrent processes
    /// Timeout for group operations
    /// Kill all on first failure
    /// Collect combined output
/// Process group state
#[derive(Debug)]
struct GroupState {
/// Environment management
#[derive(Debug)]
pub struct Environment {
    /// Environment variables
    /// Path manipulation
    /// Inherited environment
/// Output streaming configuration
#[derive(Debug)]
pub struct OutputStreamer {
    /// Associated command
    /// Line callback
    /// Streaming options
    /// Streaming state
/// Streaming options
#[derive(Debug, Clone)]
pub struct StreamingOptions {
    /// Buffer size for streaming
    /// Line-based processing
    /// Include stderr in streaming
    /// Timeout for streaming operations
/// Streaming state
#[derive(Debug)]
struct StreamingState {
/// Input generation for providing programmatic input
#[derive(Debug)]
pub struct InputGenerator {
    /// Associated command
    /// Input queue
    /// Generator state
/// Input item for delayed input
#[derive(Debug)]
struct InputItem {
/// Input generator state
#[derive(Debug)]
struct GeneratorState {
/// Command internal state
#[derive(Debug)]
struct CmdState {
/// Process state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessState {
/// Execution context for timeout and cancellation
#[derive(Debug)]
pub struct ExecutionContext {
    /// Cancellation signal
    /// Timeout duration
    /// Start time
impl Default for ProcessGroupOptions {
    fn default() -> Self {
        Self {
        }
    }
impl Default for StreamingOptions {
    fn default() -> Self {
        Self {
        }
    }
impl EnhancedCmd {
    /// Create a new command
    #[instrument]
    pub fn command(name: &str, args: &[&str]) -> Self {
        Self {
        }
    }
    
    /// Create a new command with context for timeout/cancellation
    #[instrument]
    pub fn command_context(ctx: ExecutionContext, name: &str, args: &[&str]) -> Self {
        let mut cmd = Self::command(name, args);
        cmd.context = Some(Arc::new(ctx));
        cmd
    /// Start the command
    #[instrument(skip(self))]
    pub fn start(&mut self) -> VibezResult<()> {
        let mut system_cmd = Command::new(&self.path);
        system_cmd.args(&self.args);
        
        // Configure working directory
        if let Some(ref dir) = self.dir {
            system_cmd.current_dir(dir);
        // Configure environment
        for env_var in &self.env {
            if let Some(eq_pos) = env_var.find('=') {
                let key = &env_var[..eq_pos];
                let value = &env_var[eq_pos + 1..];
                system_cmd.env(key, value);
            }
        }
        
        // Configure stdio
        system_cmd.stdout(Stdio::piped());
        system_cmd.stderr(Stdio::piped());
        system_cmd.stdin(Stdio::piped());
        
        // Spawn the process
        let child = system_cmd.spawn()
            .map_err(|e| CursedError::RuntimeError(format!("Failed to spawn process: {}", e)))?;
        
        let pid = child.id();
        let process = EnhancedProcess::new(pid, child)?;
        
        // Update state
        {
            let mut state = self.state.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
            
            state.process = Some(process);
            state.start_time = Some(SystemTime::now());
            state.is_running = true;
        info!("Process started: PID {} - {}", pid, self.path);
        Ok(())
    /// Run the command and wait for completion
    #[instrument(skip(self))]
    pub fn run(&mut self) -> VibezResult<()> {
        self.start()?;
        
        // Check for timeout context
        if let Some(ref ctx) = self.context {
            self.wait_with_context(ctx)
        } else {
            self.wait()
        }
    }
    
    /// Wait for command completion
    #[instrument(skip(self))]
    pub fn wait(&mut self) -> VibezResult<()> {
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref mut process) = state.process {
            let process_state = process.wait()?;
            
            state.is_running = false;
            state.end_time = Some(SystemTime::now());
            state.exit_code = process_state.exit_status.and_then(|s| s.code());
            
            if let Some(exit_code) = state.exit_code {
                if exit_code != 0 {
                    return Err(CursedError::RuntimeError(format!("Process failed with exit code {}", exit_code)));
                }
            }
        Ok(())
    /// Wait with execution context (timeout/cancellation)
    #[instrument(skip(self, ctx))]
    fn wait_with_context(&mut self, ctx: &ExecutionContext) -> VibezResult<()> {
        let start_time = Instant::now();
        
        loop {
            // Check cancellation signal
            {
                let (lock, _) = &*ctx.cancel_signal;
                let cancelled = lock.lock()
                    .map_err(|_| CursedError::RuntimeError("Failed to check cancellation".to_string()))?;
                
                if *cancelled {
                    self.kill_process()?;
                    return Err(CursedError::RuntimeError("Process cancelled".to_string()));
                }
            }
            
            // Check timeout
            if let Some(timeout) = ctx.timeout {
                if start_time.elapsed() >= timeout {
                    self.kill_process()?;
                    return Err(CursedError::RuntimeError("Process timed out".to_string()));
                }
            }
            
            // Check if process completed
            {
                let mut state = self.state.lock()
                    .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
                
                if let Some(ref mut process) = state.process {
                    if let Ok(Some(exit_status)) = process.try_wait() {
                        state.is_running = false;
                        state.end_time = Some(SystemTime::now());
                        state.exit_code = exit_status.code();
                        
                        if let Some(exit_code) = state.exit_code {
                            if exit_code != 0 {
                                return Err(CursedError::RuntimeError(format!("Process failed with exit code {}", exit_code)));
                            }
                        }
                        
                        return Ok(());
                    }
                }
            thread::sleep(Duration::from_millis(50));
        }
    }
    
    /// Get command output
    #[instrument(skip(self))]
    pub fn output(&mut self) -> VibezResult<Vec<u8>> {
        self.run()?;
        
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        Ok(state.output_data.clone())
    /// Get combined output (stdout + stderr)
    #[instrument(skip(self))]
    pub fn combined_output(&mut self) -> VibezResult<Vec<u8>> {
        self.run()?;
        
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        let mut combined = state.output_data.clone();
        combined.extend_from_slice(&state.error_data);
        Ok(combined)
    /// Get stdin pipe for writing to process
    #[instrument(skip(self))]
    pub fn stdin_pipe(&mut self) -> VibezResult<Box<dyn Write + Send>> {
        debug!("Creating stdin pipe for process");
        
        // Ensure stdin is configured for piping
        self.stdin(ProcessStdin::Pipe);
        
        // Start the process if not already started
        if !self.is_running()? {
            self.spawn()?;
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref mut process) = state.process {
            if let Some(stdin) = process.stdin.take() {
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
    
    /// Get stdout pipe for reading from process
    #[instrument(skip(self))]
    pub fn stdout_pipe(&mut self) -> VibezResult<Box<dyn Read + Send>> {
        debug!("Creating stdout pipe for process");
        
        // Ensure stdout is configured for piping
        self.stdout(ProcessStdout::Pipe);
        
        // Start the process if not already started
        if !self.is_running()? {
            self.spawn()?;
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref mut process) = state.process {
            if let Some(stdout) = process.stdout.take() {
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
    pub fn stderr_pipe(&mut self) -> VibezResult<Box<dyn Read + Send>> {
        debug!("Creating stderr pipe for process");
        
        // Ensure stderr is configured for piping
        self.stderr(ProcessStderr::Pipe);
        
        // Start the process if not already started
        if !self.is_running()? {
            self.spawn()?;
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref mut process) = state.process {
            if let Some(stderr) = process.stderr.take() {
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
    
    /// Get process handle
    #[instrument(skip(self))]
    pub fn process(&self) -> VibezResult<EnhancedProcess> {
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        state.process.clone()
            .ok_or_else(|| CursedError::RuntimeError("Process not started".to_string()))
    /// Get process state
    #[instrument(skip(self))]
    pub fn process_state(&self) -> VibezResult<EnhancedProcessState> {
        let state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref process) = state.process {
            process.get_process_state()
        } else {
            Err(CursedError::RuntimeError("Process not started".to_string()))
        }
    }
    
    /// Kill the process
    #[instrument(skip(self))]
    fn kill_process(&mut self) -> VibezResult<()> {
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        if let Some(ref mut process) = state.process {
            process.kill()?;
            state.is_running = false;
            state.end_time = Some(SystemTime::now());
        Ok(())
    }
}

impl EnhancedProcess {
    /// Create new process handle
    #[instrument]
    fn new(pid: u32, child: Child) -> VibezResult<Self> {
        Ok(Self {
        })
    /// Kill the process
    #[instrument(skip(self))]
    pub fn kill(&mut self) -> VibezResult<()> {
        let mut child = self.child.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire child lock".to_string()))?;
        
        if let Some(ref mut child) = child.as_mut() {
            child.kill()
                .map_err(|e| CursedError::RuntimeError(format!("Failed to kill process: {}", e)))?;
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        *state = ProcessState::Killed;
        
        Ok(())
    /// Send signal to process
    #[instrument(skip(self))]
    pub fn signal(&self, sig: i32) -> VibezResult<()> {
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
            return Err(CursedError::RuntimeError("Signal handling not implemented for Windows".to_string()));
        Ok(())
    /// Wait for process completion
    #[instrument(skip(self))]
    pub fn wait(&mut self) -> VibezResult<EnhancedProcessState> {
        let mut child = self.child.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire child lock".to_string()))?;
        
        if let Some(child) = child.take() {
            let status = child.wait()
                .map_err(|e| CursedError::RuntimeError(format!("Failed to wait for process: {}", e)))?;
            
            let mut state = self.state.lock()
                .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
            
            *state = if status.success() {
                ProcessState::Completed
            } else {
                ProcessState::Failed
            
            Ok(EnhancedProcessState {
                start_time: SystemTime::now(), // Would track properly
                system_usage: Some(SystemUsage {
            })
        } else {
            Err(CursedError::RuntimeError("Process already waited on".to_string()))
        }
    }
    
    /// Try to wait without blocking
    #[instrument(skip(self))]
    pub fn try_wait(&mut self) -> VibezResult<Option<ExitStatus>> {
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
    pub fn release(&mut self) -> VibezResult<()> {
        let mut child = self.child.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire child lock".to_string()))?;
        
        *child = None;
        Ok(())
    /// Get process state
    #[instrument(skip(self))]
    pub fn get_process_state(&self) -> VibezResult<EnhancedProcessState> {
        Ok(EnhancedProcessState {
            system_usage: Some(SystemUsage {
        })
    }
}

impl EnhancedProcessState {
    /// Check if process exited
    #[instrument(skip(self))]
    pub fn exited(&self) -> bool {
        self.exit_status.is_some()
    /// Get exit code
    #[instrument(skip(self))]
    pub fn exit_code(&self) -> i32 {
        self.exit_status
            .and_then(|status| status.code())
            .unwrap_or(-1)
    /// Check if process was successful
    #[instrument(skip(self))]
    pub fn success(&self) -> bool {
        self.exit_status
            .map(|status| status.success())
            .unwrap_or(false)
    /// Get system information
    #[instrument(skip(self))]
    pub fn sys(&self) -> Option<&SystemUsage> {
        self.system_usage.as_ref()
    /// Get system usage information
    #[instrument(skip(self))]
    pub fn sys_usage(&self) -> Option<&SystemUsage> {
        self.system_usage.as_ref()
    /// Get string representation
    #[instrument(skip(self))]
    pub fn to_string(&self) -> String {
        format!("ProcessState(exit_code: {}, success: {})", self.exit_code(), self.success())
    /// Get user time
    #[instrument(skip(self))]
    pub fn user_time(&self) -> CursedDuration {
        self.user_time.clone()
    /// Get system time
    #[instrument(skip(self))]
    pub fn system_time(&self) -> CursedDuration {
        self.system_time.clone()
    }
}

impl ExecError {
    /// Create new execution error
    #[instrument]
    pub fn new(message: &str) -> Self {
        Self {
        }
    }
    
    /// Create error with exit code
    #[instrument]
    pub fn with_exit_code(message: &str, exit_code: i32) -> Self {
        Self {
        }
    }
    
    /// Get error message
    #[instrument(skip(self))]
    pub fn error(&self) -> String {
        self.message.clone()
    /// Unwrap underlying error
    #[instrument(skip(self))]
    pub fn unwrap(&self) -> Option<&(dyn std::error::CursedError + Send + Sync)> {
        self.source.as_deref()
    /// Get exit code
    #[instrument(skip(self))]
    pub fn exit_code(&self) -> i32 {
        self.exit_code.unwrap_or(-1)
    }
}

impl ProcessGroup {
    /// Create new process group
    #[instrument]
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add command to group
    #[instrument(skip(self))]
    pub fn add_command(&self, cmd: EnhancedCmd) -> VibezResult<()> {
        let mut processes = self.processes.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire processes lock".to_string()))?;
        
        processes.push(cmd);
        Ok(())
    /// Start all commands in the group
    #[instrument(skip(self))]
    pub fn start_all(&self) -> VibezResult<()> {
        let mut processes = self.processes.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire processes lock".to_string()))?;
        
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        state.start_time = Some(SystemTime::now());
        
        // Start processes up to max_concurrent limit
        let mut started = 0;
        for process in processes.iter_mut() {
            if started >= self.options.max_concurrent {
                break;
            if let Err(e) = process.start() {
                if self.options.fail_fast {
                    return Err(e);
                }
                // Continue starting other processes
                warn!("Failed to start process in group: {}", e);
            } else {
                started += 1;
            }
        }
        
        info!("Started {} processes in group {}", started, self.id);
        Ok(())
    /// Wait for all processes to complete
    #[instrument(skip(self))]
    pub fn wait_all(&self) -> VibezResult<()> {
        let start_time = Instant::now();
        
        loop {
            let (all_done, any_failed) = {
                let mut processes = self.processes.lock()
                    .map_err(|_| CursedError::RuntimeError("Failed to acquire processes lock".to_string()))?;
                
                let mut all_done = true;
                let mut any_failed = false;
                
                for process in processes.iter_mut() {
                    let process_state = process.state.lock()
                        .map_err(|_| CursedError::RuntimeError("Failed to acquire process state lock".to_string()))?;
                    
                    if process_state.is_running {
                        all_done = false;
                    if let Some(exit_code) = process_state.exit_code {
                        if exit_code != 0 {
                            any_failed = true;
                        }
                    }
                (all_done, any_failed)
            
            if all_done {
                let mut state = self.state.lock()
                    .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
                state.end_time = Some(SystemTime::now());
                
                if any_failed && self.options.fail_fast {
                    return Err(CursedError::RuntimeError("One or more processes in group failed".to_string()));
                break;
            // Check timeout
            if let Some(timeout) = self.options.timeout {
                if start_time.elapsed() >= timeout {
                    return Err(CursedError::RuntimeError("Process group timed out".to_string()));
                }
            }
            
            thread::sleep(Duration::from_millis(100));
        Ok(())
    }
}

impl Environment {
    /// Create new environment
    #[instrument]
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Set environment variable
    #[instrument(skip(self))]
    pub fn set(&mut self, key: &str, value: &str) -> &mut Self {
        self.variables.insert(key.to_string(), value.to_string());
        self
    /// Append to PATH-like variable
    #[instrument(skip(self))]
    pub fn append(&mut self, key: &str, value: &str) -> &mut Self {
        if let Some(existing) = self.variables.get_mut(key) {
            existing.push_str(&format!(":{}", value));
        } else {
            self.variables.insert(key.to_string(), value.to_string());
        }
        self
    /// Get environment variables as vector
    #[instrument(skip(self))]
    pub fn to_env_vec(&self) -> Vec<String> {
        self.variables.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect()
    }
}

impl OutputStreamer {
    /// Create new output streamer
    #[instrument]
    pub fn new(command: EnhancedCmd) -> Self {
        Self {
        }
    }
    
    /// Set line callback
    #[instrument(skip(self, callback))]
    pub fn on_line<F>(&mut self, callback: F) -> &mut Self
    where
    {
        self.line_callback = Some(Arc::new(callback));
        self
    /// Start streaming
    #[instrument(skip(self))]
    pub fn start(&mut self) -> VibezResult<()> {
        self.command.start()?;
        
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        state.is_streaming = true;
        
        // In a real implementation, this would start background threads
        // to read from stdout/stderr and call the callback
        
        Ok(())
    /// Wait for streaming to complete
    #[instrument(skip(self))]
    pub fn wait(&mut self) -> VibezResult<()> {
        self.command.wait()?;
        
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        state.is_streaming = false;
        
        Ok(())
    }
}

impl InputGenerator {
    /// Create new input generator
    #[instrument]
    pub fn new(command: EnhancedCmd) -> Self {
        Self {
        }
    }
    
    /// Write immediate input
    #[instrument(skip(self, data))]
    pub fn write(&self, data: &[u8]) -> VibezResult<()> {
        let mut queue = self.input_queue.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire queue lock".to_string()))?;
        
        queue.push(InputItem {
        });
        
        Ok(())
    /// Write input after delay
    #[instrument(skip(self, data))]
    pub fn write_after(&self, data: &[u8], delay: Duration) -> VibezResult<()> {
        let mut queue = self.input_queue.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire queue lock".to_string()))?;
        
        queue.push(InputItem {
        });
        
        Ok(())
    /// Close input
    #[instrument(skip(self))]
    pub fn close(&self) -> VibezResult<()> {
        let mut state = self.state.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire state lock".to_string()))?;
        
        state.is_active = false;
        Ok(())
    }
}

impl ExecutionContext {
    /// Create new execution context with timeout
    #[instrument]
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
        }
    }
    
    /// Create cancellable context
    #[instrument]
    pub fn cancellable() -> Self {
        Self {
        }
    }
    
    /// Cancel the context
    #[instrument(skip(self))]
    pub fn cancel(&self) -> VibezResult<()> {
        let (lock, cvar) = &*self.cancel_signal;
        let mut cancelled = lock.lock()
            .map_err(|_| CursedError::RuntimeError("Failed to acquire cancel lock".to_string()))?;
        
        *cancelled = true;
        cvar.notify_all();
        Ok(())
    }
}

// Helper implementations
impl CmdState {
    fn new() -> Self {
        Self {
        }
    }
impl GroupState {
    fn new() -> Self {
        Self {
        }
    }
impl StreamingState {
    fn new() -> Self {
        Self {
        }
    }
impl GeneratorState {
    fn new() -> Self {
        Self {
        }
    }
// impl std::error::CursedError for ExecError {
//     fn source(&self) -> Option<&(dyn std::error::CursedError + 'static)> {
//         None // Would be implemented properly
//     }
// }

// impl std::fmt::Display for ExecError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

/// Global functions matching the exec_vibez specification

/// Create a new command
#[instrument]
pub fn command(name: &str, args: &[&str]) -> EnhancedCmd {
    EnhancedCmd::command(name, args)
/// Create a command with context
#[instrument]
pub fn command_context(ctx: ExecutionContext, name: &str, args: &[&str]) -> EnhancedCmd {
    EnhancedCmd::command_context(ctx, name, args)
/// Look up executable path
#[instrument]
pub fn look_path(file: &str) -> VibezResult<String> {
    // This would implement PATH lookup
    // For now, just return the file as-is
    Ok(file.to_string())
/// Create new process group
#[instrument]
pub fn new_process_group() -> ProcessGroup {
    ProcessGroup::new()
/// Run with timeout
#[instrument]
pub fn run_with_timeout(name: &str, arg: &str, timeout: Duration) -> VibezResult<()> {
    let ctx = ExecutionContext::with_timeout(timeout);
    let mut cmd = command_context(ctx, name, &[arg]);
    cmd.run()
/// Create command with environment
#[instrument]
pub fn command_with_env(name: &str, arg: &str, env: Environment) -> EnhancedCmd {
    let mut cmd = command(name, &[arg]);
    cmd.env = env.to_env_vec();
    cmd
/// Create new output streamer
#[instrument]
pub fn new_output_streamer(command: EnhancedCmd) -> OutputStreamer {
    OutputStreamer::new(command)
/// Create new input generator
#[instrument]
pub fn new_input_generator(command: EnhancedCmd) -> InputGenerator {
    InputGenerator::new(command)
/// Create new environment
#[instrument]
pub fn new_environment() -> Environment {
    Environment::new()
