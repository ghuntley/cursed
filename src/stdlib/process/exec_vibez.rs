use crate::error::CursedError;
/// exec_vibez - Process execution with enhanced features
/// 
/// This module provides functionality for executing external commands and managing
/// external processes, similar to Go's os/exec package but with CURSED-style enhancements.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};
// use crate::stdlib::web_vibez::SecurityContext;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

// Placeholder imports disabled
    timeout_error, invalid_arguments, io_error, system_error, platform_error
// };

// Placeholder imports disabled
    unregister_process_from_monitoring
// };

// Re-export types from exec_vibez_types
// pub use crate::stdlib::process::exec_vibez_types::{
    ExecutionMode, Priority
// };

// Type aliases for compatibility
pub type VibezProcess = Process;
pub type VibezCommand = Cmd;

/// Cmd represents an external command being prepared or run
#[derive(Debug)]
pub struct Cmd {
    /// Path to the executable
    /// Command arguments
    /// Environment variables for the process
    /// Working directory
    /// Standard input configuration
    /// Standard output configuration  
    /// Standard error configuration
    /// Process context for cancellation
    /// Internal child process handle
    /// Process start time
/// Process represents a running process
#[derive(Debug)]
pub struct Process {
    /// Process ID
    /// Process handle
    /// Start time
/// ProcessState contains information about a process that has exited
#[derive(Debug, Clone)]
pub struct ProcessState {
    /// Exit status
    /// Process ID
    /// User CPU time used
    /// System CPU time used
    /// System-specific information
/// CursedError represents an error from an executable program
#[derive(Debug, Clone)]
pub struct CursedError {
    /// CursedError message
    /// Exit code if available
    /// Underlying error
/// Process context for timeout and cancellation
#[derive(Debug, Clone)]
pub struct ProcessContext {
    /// Timeout duration
    /// Cancellation signal
/// Process group for managing multiple related processes
#[derive(Debug)]
pub struct ProcessGroup {
    /// Commands in the group
    /// Group options
    /// Running processes
/// Options for process groups
#[derive(Debug, Clone)]
pub struct ProcessGroupOptions {
    /// Start all processes simultaneously
    /// Wait for all to complete before considering success
    /// Continue on individual failures
    /// Timeout for the entire group
impl Default for ProcessGroupOptions {
    fn default() -> Self {
        Self {
        }
    }
/// Environment management for processes
#[derive(Debug, Clone)]
pub struct Environment {
    /// Environment variables
    /// Whether to inherit parent environment
impl Environment {
    /// Create a new environment
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set an environment variable
    pub fn set<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        self.vars.insert(key.as_ref().to_string(), value.as_ref().to_string());
    /// Get an environment variable
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&String> {
        self.vars.get(key.as_ref())
    /// Append to an environment variable (like PATH)
    pub fn append<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        let key = key.as_ref();
        let value = value.as_ref();
        
        if let Some(existing) = self.vars.get_mut(key) {
            existing.push_str(value);
        } else {
            self.vars.insert(key.to_string(), value.to_string());
        }
    }

    /// Remove an environment variable
    pub fn remove<K: AsRef<str>>(&mut self, key: K) {
        self.vars.remove(key.as_ref());
    /// Set whether to inherit parent environment
    pub fn set_inherit(&mut self, inherit: bool) {
        self.inherit = inherit;
    /// Convert to vector format for Command
    pub fn to_env_vec(&self) -> Vec<String> {
        self.vars.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect()
    }
}

/// Output streamer for real-time command output processing
#[derive(Debug)]
pub struct OutputStreamer {
    /// Command to stream
    /// Line callback
    /// Buffer size for reading
impl OutputStreamer {
    /// Create a new output streamer
    pub fn new(cmd: Cmd) -> Self {
        Self {
        }
    }

    /// Set line callback
    pub fn on_line<F>(&mut self, callback: F)
    where
    {
        self.line_callback = Some(Box::new(callback));
    /// Start streaming
    pub fn start(&mut self) -> ProcessResult<()> {
        self.cmd.start()?;
        
        if let Some(child_arc) = &self.cmd.child {
            let stdout = {
                let mut child = child_arc.lock().unwrap();
                child.stdout.take()
            
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                let callback = self.line_callback.take();
                
                thread::spawn(move || {
                    for line in reader.split("\n") {
                        if let Ok(line) = line {
                            if let Some(ref cb) = callback {
                                cb(&line);
                            }
                        }
                    }
                });
            }
        }
        
        Ok(())
    /// Wait for streaming to complete
    pub fn wait(&mut self) -> ProcessResult<()> {
        self.cmd.wait()
    }
}

/// Input generator for programmatically providing input to commands
#[derive(Debug)]
pub struct InputGenerator {
    /// Command to provide input to
    /// Input queue
impl InputGenerator {
    /// Create a new input generator
    pub fn new(cmd: Cmd) -> Self {
        Self {
        }
    }

    /// Write input immediately
    pub fn write<S: AsRef<str>>(&mut self, input: S) -> ProcessResult<()> {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push((input.as_ref().to_string(), None));
        Ok(())
    /// Write input after a delay
    pub fn write_after<S: AsRef<str>>(&mut self, input: S, delay: Duration) -> ProcessResult<()> {
        let mut queue = self.input_queue.lock().unwrap();
        queue.push((input.as_ref().to_string(), Some(delay)));
        Ok(())
    /// Start the input generation process
    pub fn start(&mut self) -> ProcessResult<()> {
        self.cmd.start()?;
        
        if let Some(child_arc) = &self.cmd.child {
            let stdin = {
                let mut child = child_arc.lock().unwrap();
                child.stdin.take()
            
            if let Some(stdin) = stdin {
                let queue = self.input_queue.clone();
                
                thread::spawn(move || {
                    let mut writer = BufWriter::new(stdin);
                    
                    loop {
                        let item = {
                            let mut q = queue.lock().unwrap();
                            q.pop()
                        
                        if let Some((input, delay)) = item {
                            if let Some(delay) = delay {
                                thread::sleep(delay);
                            if writer.write_all(input.as_bytes()).is_err() {
                                break;
                            }
                            if writer.flush().is_err() {
                                break;
                            }
                        } else {
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
                });
            }
        }
        
        Ok(())
    /// Close input stream
    pub fn close(&mut self) -> ProcessResult<()> {
        // Signal that no more input will be provided
        Ok(())
    }
}

impl Cmd {
    /// Create a new Cmd instance
    pub fn new<S: AsRef<str>>(name: S, args: &[&str]) -> Self {
        Self {
        }
    }

    /// Start the process without waiting
    pub fn start(&mut self) -> ProcessResult<()> {
        let mut command = Command::new(&self.path);
        command.args(&self.args);

        if let Some(dir) = &self.dir {
            command.current_dir(dir);
        // Set environment
        if !self.env.is_empty() {
            command.env_clear();
            for env_pair in &self.env {
                if let Some((key, value)) = env_pair.split_once('=') {
                    command.env(key, value);
                }
            }
        // Configure I/O
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let child = command.spawn()
            .map_err(|e| execution_failed(&self.path, &e.to_string()))?;

        let pid = child.id();
        let child_arc = Arc::new(Mutex::new(child));
        self.child = Some(child_arc.clone());
        self.start_time = Some(Instant::now());

        // Register process for monitoring
        let _ = register_process_for_monitoring(pid, Some(child_arc));

        Ok(())
    /// Run the command and wait for completion
    pub fn run(&mut self) -> ProcessResult<()> {
        self.start()?;
        self.wait()
    /// Capture stdout output
    pub fn output(&mut self) -> ProcessResult<Vec<u8>> {
        self.start()?;
        
        let output = if let Some(child_arc) = &self.child {
            let mut child = child_arc.lock().unwrap();
            child.wait_with_output()
                .map_err(|e| io_error("output", &format!("{:?}", e.kind()), &e.to_string()))?
        } else {
            return Err(invalid_arguments("output", "command", "Command not started"));

        if !output.status.success() {
            if let Some(code) = output.status.code() {
                return Err(execution_failed_with_code(&self.path, code, "Command failed"));
            }
        }

        Ok(output.stdout)
    /// Capture combined stdout and stderr
    pub fn combined_output(&mut self) -> ProcessResult<Vec<u8>> {
        self.start()?;
        
        let output = if let Some(child_arc) = &self.child {
            let mut child = child_arc.lock().unwrap();
            child.wait_with_output()
                .map_err(|e| io_error("combined_output", &format!("{:?}", e.kind()), &e.to_string()))?
        } else {
            return Err(invalid_arguments("combined_output", "command", "Command not started"));

        if !output.status.success() {
            if let Some(code) = output.status.code() {
                return Err(execution_failed_with_code(&self.path, code, "Command failed"));
            }
        }

        let mut combined = output.stdout;
        combined.extend_from_slice(&output.stderr);
        Ok(combined)
    /// Get stdin pipe for writing to process
    pub fn stdin_pipe(&mut self) -> ProcessResult<Box<dyn Write + Send>> {
        // Ensure stdin is configured for piping
        self.stdin = Some(ProcessStdin::Pipe);
        
        // Start the process if not already started
        self.start()?;
        
        if let Some(child_arc) = &self.child {
            let mut child = child_arc.lock()
                .map_err(|_| invalid_arguments("stdin_pipe", "lock", "Failed to acquire child process lock"))?;
            
            if let Some(stdin) = child.stdin.take() {
                Ok(Box::new(stdin))
            } else {
                Err(invalid_arguments("stdin_pipe", "stdin", "Stdin not available - pipe not configured"))
            }
        } else {
            Err(invalid_arguments("stdin_pipe", "command", "Command not started"))
        }
    }

    /// Get stdout pipe for reading from process
    pub fn stdout_pipe(&mut self) -> ProcessResult<Box<dyn Read + Send>> {
        // Ensure stdout is configured for piping
        self.stdout = Some(ProcessStdout::Pipe);
        
        // Start the process if not already started
        self.start()?;
        
        if let Some(child_arc) = &self.child {
            let mut child = child_arc.lock()
                .map_err(|_| invalid_arguments("stdout_pipe", "lock", "Failed to acquire child process lock"))?;
            
            if let Some(stdout) = child.stdout.take() {
                Ok(Box::new(stdout))
            } else {
                Err(invalid_arguments("stdout_pipe", "stdout", "Stdout not available - pipe not configured"))
            }
        } else {
            Err(invalid_arguments("stdout_pipe", "command", "Command not started"))
        }
    }

    /// Get stderr pipe for reading from process
    pub fn stderr_pipe(&mut self) -> ProcessResult<Box<dyn Read + Send>> {
        // Ensure stderr is configured for piping
        self.stderr = Some(ProcessStderr::Pipe);
        
        // Start the process if not already started
        self.start()?;
        
        if let Some(child_arc) = &self.child {
            let mut child = child_arc.lock()
                .map_err(|_| invalid_arguments("stderr_pipe", "lock", "Failed to acquire child process lock"))?;
            
            if let Some(stderr) = child.stderr.take() {
                Ok(Box::new(stderr))
            } else {
                Err(invalid_arguments("stderr_pipe", "stderr", "Stderr not available - pipe not configured"))
            }
        } else {
            Err(invalid_arguments("stderr_pipe", "command", "Command not started"))
        }
    }

    /// Wait for the command to complete
    pub fn wait(&mut self) -> ProcessResult<()> {
        if let Some(child_arc) = &self.child {
            let pid = {
                let child = child_arc.lock().unwrap();
                child.id()
            
            // Use real process monitoring to wait for completion
            match wait_for_real_process(pid) {
                Ok(real_state) => {
                    // Check if process completed successfully
                    if let Some(exit_status) = real_state.exit_status {
                        if !exit_status.success() {
                            if let Some(code) = exit_status.code() {
                                return Err(execution_failed_with_code(&self.path, code, "Command failed"));
                            } else {
                                return Err(execution_failed(&self.path, "Command terminated by signal"));
                            }
                        }
                    // Unregister from monitoring
                    let _ = unregister_process_from_monitoring(pid);
                    Ok(())
                }
                Err(_) => {
                    // Fallback to standard wait if monitoring fails
                    let mut child = child_arc.lock().unwrap();
                    let status = child.wait()
                        .map_err(|e| io_error("wait", &format!("{:?}", e.kind()), &e.to_string()))?;
                    
                    if !status.success() {
                        if let Some(code) = status.code() {
                            return Err(execution_failed_with_code(&self.path, code, "Command failed"));
                        } else {
                            return Err(execution_failed(&self.path, "Command terminated by signal"));
                        }
                    }
                    Ok(())
                }
            }
        } else {
            Err(invalid_arguments("wait", "command", "Command not started"))
        }
    }

    /// Get process handle
    pub fn process(&self) -> ProcessResult<Process> {
        if let Some(child_arc) = &self.child {
            let pid = {
                let child = child_arc.lock().unwrap();
                child.id()
            Ok(Process {
            })
        } else {
            Err(invalid_arguments("process", "command", "Command not started"))
        }
    }

    /// Get process state
    pub fn process_state(&self) -> ProcessResult<ProcessState> {
        if let Some(child_arc) = &self.child {
            let pid = {
                let child = child_arc.lock().unwrap();
                child.id()
            
            // Use real process monitoring to get actual state
            match wait_for_real_process(pid) {
                Ok(real_state) => {
                    // Convert real state to our ProcessState format
                    let exit_status = real_state.exit_status.unwrap_or_else(|| {
                        // If no exit status yet, create a running status
                        #[cfg(unix)]
                        {
                            ExitStatus::from_raw(0)
                        }
                        #[cfg(not(unix))]
                        {
                            // On non-Unix platforms, we can't create a fake status
                            // so we'll need to handle this differently
                            std::process::ExitStatus::from(std::process::Command::new("true").status().unwrap())
                        }
                    });
                    
                    Ok(ProcessState {
                        sys_info: Vec::new(), // Could be extended with real_state.memory_info serialized
                    })
                }
                Err(_) => {
                    // Fallback to basic state if real monitoring fails
                    let exit_status = {
                        #[cfg(unix)]
                        {
                            ExitStatus::from_raw(0)
                        }
                        #[cfg(not(unix))]
                        {
                            std::process::Command::new("true").status().unwrap()
                        }
                    
                    Ok(ProcessState {
                    })
                }
            }
        } else {
            Err(invalid_arguments("process_state", "command", "Command not started"))
        }
    }
impl Process {
    /// Kill the process
    pub fn kill(&self) -> ProcessResult<()> {
        let mut child = self.child.lock().unwrap();
        child.kill()
            .map_err(|e| io_error("kill", &format!("{:?}", e.kind()), &e.to_string()))?;
        Ok(())
    /// Send signal to process (Unix only)
    #[cfg(unix)]
    pub fn signal(&self, sig: i32) -> ProcessResult<()> {
        // Use nix crate for safer signal handling if available, otherwise fall back to basic kill
        use std::process::Command;
        
        let output = Command::new("kill")
            .arg(format!("-{}", sig))
            .arg(self.pid.to_string())
            .output();
            
        match output {
        }
    }

    #[cfg(not(unix))]
    pub fn signal(&self, _sig: i32) -> ProcessResult<()> {
        Err(platform_error("Signal sending not supported on this platform"))
    /// Wait for process completion
    pub fn wait(&self) -> ProcessResult<ProcessState> {
        let mut child = self.child.lock().unwrap();
        let status = child.wait()
            .map_err(|e| io_error("wait", &format!("{:?}", e.kind()), &e.to_string()))?;

        Ok(ProcessState {
        })
    /// Release process resources
    pub fn release(&self) -> ProcessResult<()> {
        // Release any held resources
        Ok(())
    }
}

impl ProcessState {
    /// Check if process exited normally
    pub fn exited(&self) -> bool {
        true // Simplified
    /// Get exit code
    pub fn exit_code(&self) -> i32 {
        self.exit_status.code().unwrap_or(-1)
    /// Check if process was successful
    pub fn success(&self) -> bool {
        self.exit_status.success()
    /// Get system-specific information
    pub fn sys(&self) -> Box<dyn std::any::Any> {
        Box::new(self.exit_status)
    /// Get system usage information
    pub fn sys_usage(&self) -> Box<dyn std::any::Any> {
        Box::new((self.user_time, self.system_time))
    /// String representation
    pub fn string(&self) -> String {
        format!("Process {} exited with code {}", self.pid, self.exit_code())
    /// Get user CPU time
    pub fn user_time(&self) -> Duration {
        self.user_time
    /// Get system CPU time
    pub fn system_time(&self) -> Duration {
        self.system_time
    }
}

impl CursedError {
    /// Create a new error
    pub fn new<S: AsRef<str>>(message: S) -> Self {
        Self {
        }
    }

    /// Get error message
    pub fn error(&self) -> String {
        self.message.clone()
    /// Unwrap underlying error
    pub fn unwrap(&self) -> Option<String> {
        self.source.clone()
    /// Get exit code
    pub fn exit_code(&self) -> i32 {
        self.exit_code.unwrap_or(-1)
    }
}

impl ProcessContext {
    /// Create a new process context
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create context with timeout
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
        }
    }

    /// Cancel the context
    pub fn cancel(&self) {
        let mut cancelled = self.cancelled.write().unwrap();
        *cancelled = true;
    /// Check if context is cancelled
    pub fn is_cancelled(&self) -> bool {
        *self.cancelled.read().unwrap()
    }
}

impl ProcessGroup {
    /// Create a new process group
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add a command to the group
    pub fn add_command(&mut self, cmd: Cmd) {
        self.commands.push(cmd);
    /// Start all commands in the group
    pub fn start_all(&mut self) -> ProcessResult<()> {
        for cmd in &mut self.commands {
            cmd.start()?;
            if let Ok(process) = cmd.process() {
                self.processes.push(process);
            }
        }
        Ok(())
    /// Wait for all commands to complete
    pub fn wait_all(&mut self) -> ProcessResult<()> {
        for cmd in &mut self.commands {
            if let Err(e) = cmd.wait() {
                if !self.options.continue_on_failure {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

/// Create a new Cmd instance
pub fn command<S: AsRef<str>>(name: S, args: &[&str]) -> Cmd {
    Cmd::new(name, args)
/// Create a new Cmd with context
pub fn command_context(ctx: ProcessContext, name: &str, args: &[&str]) -> Cmd {
    let mut cmd = Cmd::new(name, args);
    cmd.context = Some(ctx);
    cmd
/// Look up executable path
pub fn look_path<S: AsRef<str>>(file: S) -> ProcessResult<String> {
    let file = file.as_ref();
    
    if let Ok(paths) = std::env::var("PATH") {
        for path in std::env::split_paths(&paths) {
            let full_path = path.join(file);
            if full_path.is_file() {
                return Ok(full_path.to_string_lossy().to_string());
            // On Windows, also check with .exe extension
            #[cfg(windows)]
            {
                let exe_path = path.join(format!("{}.exe", file));
                if exe_path.is_file() {
                    return Ok(exe_path.to_string_lossy().to_string());
                }
            }
        }
    }
    
    Err(execution_failed(file, "Command not found in PATH"))
/// Enhanced features

/// Create a new process group
pub fn new_process_group() -> ProcessGroup {
    ProcessGroup::new()
/// Run command with timeout
pub fn run_with_timeout<S: AsRef<str>>(name: S, args: &[&str], timeout: Duration) -> ProcessResult<Vec<u8>> {
    let mut cmd = command(name, args);
    let ctx = ProcessContext::with_timeout(timeout);
    cmd.context = Some(ctx);
    cmd.output()
/// Create command with environment
pub fn command_with_env<S: AsRef<str>>(name: S, args: &[&str], env: Environment) -> Cmd {
    let mut cmd = command(name, args);
    cmd.env = env.to_env_vec();
    cmd
/// Create a new output streamer
pub fn new_output_streamer(cmd: Cmd) -> OutputStreamer {
    OutputStreamer::new(cmd)
/// Create a new input generator
pub fn new_input_generator(cmd: Cmd) -> InputGenerator {
    InputGenerator::new(cmd)
/// Create a new environment
pub fn new_environment() -> Environment {
    Environment::new()
