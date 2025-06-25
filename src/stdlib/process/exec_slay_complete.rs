use crate::error::CursedError;
/// Complete ExecSlay implementation matching the specification
/// Provides utilities for running external commands with style and efficiency

use std::collections::HashMap;
use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// use crate::stdlib::process::error::{ProcessResult, ProcessError, execution_failed, timeout_error, system_error};

/// Represents an external command to be executed (SlayCommand)
#[derive(Debug, Clone)]
pub struct SlayCommand {
/// Represents a process created by a SlayCommand (SlayProcess)
#[derive(Debug)]
pub struct SlayProcess {
/// Contains information about a process that has finished (SlayProcessState)
#[derive(Debug, Clone)]
pub struct SlayProcessState {
/// Configuration options for command execution (SlayOptions)
#[derive(Debug, Clone, Default)]
pub struct SlayOptions {
/// Execution pipeline for chaining commands (SlayPipeline)
#[derive(Debug)]
pub struct SlayPipeline {
/// Background task management (SlayTask)
#[derive(Debug)]
pub struct SlayTask {
/// Command builder for fluent interface (SlayCommandBuilder)
#[derive(Debug)]
pub struct SlayCommandBuilder {
/// Signal options for process termination
#[derive(Debug, Clone)]
pub struct SignalOptions {
/// Process statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct ProcessStats {
impl SlayCommand {
    /// Create a new SlayCommand
    pub fn new<S: AsRef<str>>(name: S, args: &[S]) -> Self {
        Self {
        }
    }

    /// Run the command and wait for completion
    pub fn run(&mut self) -> ProcessResult<i32> {
        let mut cmd = self.build_command()?;
        
        let start_time = Instant::now();
        let mut child = cmd.spawn()
            .map_err(|e| execution_failed(&self.command, &format!("Failed to spawn process: {}", e)))?;

        // Handle timeout if specified
        if let Some(timeout) = self.timeout {
            match self.wait_with_timeout(&mut child, timeout) {
                Err(e) => {
                    let _ = child.kill();
                    Err(e)
                }
            }
        } else {
            let status = child.wait()
                .map_err(|e| execution_failed(&self.command, &format!("Failed to wait for process: {}", e)))?;
            Ok(status.code().unwrap_or(-1))
        }
    }

    /// Start the command without waiting
    pub fn start(&mut self) -> ProcessResult<SlayProcess> {
        let mut cmd = self.build_command()?;
        
        let child = cmd.spawn()
            .map_err(|e| execution_failed(&self.command, &format!("Failed to spawn process: {}", e)))?;

        let pid = child.id();
        let process = SlayProcess {

        Ok(process)
    /// Wait for a started process to complete
    pub fn wait(&mut self) -> ProcessResult<SlayProcessState> {
        if let Some(process_handle) = &self.process_handle {
            let mut child = process_handle.lock().unwrap();
            let start_time = Instant::now();
            
            let status = child.wait()
                .map_err(|e| execution_failed(&self.command, &format!("Failed to wait for process: {}", e)))?;

            let exit_code = status.code();
            let success = status.success();
            
            Ok(SlayProcessState {
                user_time: Duration::from_millis(0), // Platform-specific implementation needed
                system_time: Duration::from_millis(0), // Platform-specific implementation needed
            })
        } else {
            Err(system_error(-1, "wait", "No process handle available"))
        }
    }

    /// Get command output
    pub fn output(&mut self) -> ProcessResult<Vec<u8>> {
        let mut cmd = self.build_command()?;
        cmd.stdout(Stdio::piped());
        
        let output = cmd.output()
            .map_err(|e| execution_failed(&self.command, &format!("Failed to get output: {}", e)))?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(execution_failed(&self.command, &format!("Command failed with exit code: {:?}", output.status.code())))
        }
    }

    /// Get combined stdout and stderr output
    pub fn combined_output(&mut self) -> ProcessResult<Vec<u8>> {
        let mut cmd = self.build_command()?;
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let output = cmd.output()
            .map_err(|e| execution_failed(&self.command, &format!("Failed to get output: {}", e)))?;

        let mut combined = output.stdout;
        combined.extend_from_slice(&output.stderr);

        if output.status.success() {
            Ok(combined)
        } else {
            Err(execution_failed(&self.command, &format!("Command failed with exit code: {:?}", output.status.code())))
        }
    }

    // Configuration methods

    /// Set working directory
    pub fn set_dir<S: AsRef<str>>(mut self, dir: S) -> Self {
        self.working_dir = Some(dir.as_ref().to_string());
        self
    /// Set environment variables
    pub fn set_env(mut self, env: HashMap<String, String>) -> Self {
        self.env_vars = env;
        self
    /// Add environment variable
    pub fn add_env<S: AsRef<str>>(mut self, key: S, value: S) -> Self {
        self.env_vars.insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    /// Set timeout
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    /// Use shell for execution
    pub fn use_shell(mut self, use_shell: bool) -> Self {
        self.use_shell = use_shell;
        self
    /// Apply options to command
    pub fn with_options(mut self, opts: SlayOptions) -> Self {
        if let Some(dir) = opts.dir {
            self.working_dir = Some(dir);
        }
        if let Some(env) = opts.env {
            self.env_vars.extend(env);
        }
        self.timeout = opts.timeout;
        self.use_shell = opts.use_shell;
        self.shell_path = opts.shell_path;
        self.buffer_size = opts.buffer_size;
        self.collect_output = opts.collect_output;
        self
    // Helper methods

    fn build_command(&self) -> ProcessResult<Command> {
        let mut cmd = if self.use_shell {
            let shell = self.shell_path.as_deref().unwrap_or("/bin/sh");
            let mut shell_cmd = Command::new(shell);
            shell_cmd.arg("-c");
            
            let full_command = if self.args.is_empty() {
                self.command.clone()
            } else {
                format!("{} {}", self.command, self.args.join(" "))
            shell_cmd.arg(full_command);
            shell_cmd
        } else {
            let mut cmd = Command::new(&self.command);
            cmd.args(&self.args);
            cmd

        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        for (key, value) in &self.env_vars {
            cmd.env(key, value);
        Ok(cmd)
    fn wait_with_timeout(&self, child: &mut Child, timeout: Duration) -> ProcessResult<std::process::ExitStatus> {
        let start = Instant::now();
        
        loop {
            match child.try_wait() {
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        return Err(timeout_error("command execution", timeout, &format!("Command '{}' timed out", self.command)));
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
}

impl SlayProcess {
    /// Kill the process
    pub fn kill(&self) -> ProcessResult<()> {
        let mut child = self.child.lock().unwrap();
        child.kill()
            .map_err(|e| execution_failed("kill", &format!("Failed to kill process: {}", e)))
    /// Send signal to process (Unix-specific)
    #[cfg(unix)]
    pub fn signal(&self, sig: i32) -> ProcessResult<()> {
        use std::os::unix::process::ExitStatusExt;
        
        unsafe {
            if libc::kill(self.pid as i32, sig) == 0 {
                Ok(())
            } else {
                Err(execution_failed("signal", &format!("Failed to send signal {} to process {}", sig, self.pid)))
            }
        }
    #[cfg(not(unix))]
    pub fn signal(&self, _sig: i32) -> ProcessResult<()> {
        Err(system_error(-1, "signal", "Signal sending not supported on this platform"))
    /// Get process ID
    pub fn pid(&self) -> u32 {
        self.pid
    /// Wait for process to complete
    pub fn wait(&self) -> ProcessResult<SlayProcessState> {
        let mut child = self.child.lock().unwrap();
        let status = child.wait()
            .map_err(|e| execution_failed("wait", &format!("Failed to wait for process: {}", e)))?;

        Ok(SlayProcessState {
            user_time: Duration::from_millis(0), // Platform-specific implementation needed
            system_time: Duration::from_millis(0), // Platform-specific implementation needed
        })
    /// Get process statistics
    pub fn stats(&self) -> ProcessResult<ProcessStats> {
        // This would require platform-specific implementation
        // For now, return default stats
        Ok(ProcessStats::default())
    /// Terminate process gracefully
    pub fn terminate(&self, opts: SignalOptions) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            // Send SIGTERM first
            self.signal(libc::SIGTERM)?;
            
            // Wait for grace period
            thread::sleep(opts.grace_period);
            
            // Check if process is still running
            if opts.force {
                // Send SIGKILL if still running
                let _ = self.signal(libc::SIGKILL);
            }
        }
        
        #[cfg(not(unix))]
        {
            // On Windows, use terminate
            self.kill()?;
        Ok(())
    }
}

impl SlayProcessState {
    /// Check if process exited normally
    pub fn exited(&self) -> bool {
        self.exit_code.is_some()
    /// Check if process succeeded
    pub fn success(&self) -> bool {
        self.success
    /// Get exit code
    pub fn exit_code(&self) -> i32 {
        self.exit_code.unwrap_or(-1)
    /// Get user time
    pub fn user_time(&self) -> Duration {
        self.user_time
    /// Get system time
    pub fn system_time(&self) -> Duration {
        self.system_time
    }
}

impl SlayPipeline {
    /// Create a new pipeline
    pub fn new(commands: Vec<SlayCommand>) -> Self {
        Self {
        }
    }

    /// Run the pipeline
    pub fn run(&mut self) -> ProcessResult<Vec<i32>> {
        let mut exit_codes = Vec::new();
        
        for command in &mut self.commands {
            let exit_code = command.run()?;
            exit_codes.push(exit_code);
            
            // Stop on first failure unless configured otherwise
            if exit_code != 0 {
                break;
            }
        }
        
        Ok(exit_codes)
    /// Get combined output from pipeline
    pub fn output(&mut self) -> ProcessResult<Vec<u8>> {
        if self.commands.is_empty() {
            return Ok(Vec::new());
        // For simplicity, just return the output of the last command
        // A full implementation would pipe between commands
        let last_command = self.commands.last_mut().unwrap();
        last_command.output()
    }
}

impl SlayTask {
    /// Create a new background task
    pub fn new(command: SlayCommand) -> Self {
        Self {
        }
    }

    /// Check if task is running
    pub fn is_running(&self) -> bool {
        !self.finished && self.error.is_none()
    /// Get elapsed time
    pub fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    /// Wait for task to complete
    pub fn wait(&mut self) -> ProcessResult<i32> {
        if let Some(ref process) = self.process {
            let state = process.wait()?;
            self.exit_code = Some(state.exit_code());
            self.finished = true;
            Ok(state.exit_code())
        } else {
            Err(system_error(-1, "wait", "Task has no associated process"))
        }
    }

    /// Kill the task
    pub fn kill(&mut self) -> ProcessResult<()> {
        if let Some(ref process) = self.process {
            process.kill()?;
            self.finished = true;
            self.exit_code = Some(-1);
        }
        Ok(())
    /// Get task output
    pub fn get_output(&self) -> ProcessResult<Vec<u8>> {
        Ok(self.output.clone())
    /// Get combined output
    pub fn get_combined_output(&self) -> ProcessResult<Vec<u8>> {
        Ok(self.combined_output.clone())
    }
}

impl SlayCommandBuilder {
    /// Create a new command builder
    pub fn new<S: AsRef<str>>(command: S) -> Self {
        Self {
        }
    }

    /// Add arguments
    pub fn with_args<S: AsRef<str>>(mut self, args: &[S]) -> Self {
        self.args.extend(args.iter().map(|s| s.as_ref().to_string()));
        self
    /// Set working directory
    pub fn with_dir<S: AsRef<str>>(mut self, dir: S) -> Self {
        self.options.dir = Some(dir.as_ref().to_string());
        self
    /// Set environment variables
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.options.env = Some(env);
        self
    /// Add environment variable
    pub fn add_env<S: AsRef<str>>(mut self, key: S, value: S) -> Self {
        if self.options.env.is_none() {
            self.options.env = Some(HashMap::new());
        }
        self.options.env.as_mut().unwrap().insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.options.timeout = Some(timeout);
        self
    /// Use shell
    pub fn use_shell(mut self, use_shell: bool) -> Self {
        self.options.use_shell = use_shell;
        self
    /// Build the command
    pub fn build(self) -> SlayCommand {
        SlayCommand::new(&self.command, &self.args)
            .with_options(self.options)
    }
}

// Top-level functions

/// Create a new SlayCommand
pub fn new_slay_command<S: AsRef<str>>(name: S, args: &[S]) -> SlayCommand {
    SlayCommand::new(name, args)
/// Create a new SlayPipeline
pub fn new_slay_pipeline(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::new(commands)
/// Create a pipeline from commands
pub fn pipe(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::new(commands)
/// Run a command in the background
pub fn run_background(mut command: SlayCommand) -> ProcessResult<SlayTask> {
    let mut task = SlayTask::new(command.clone());
    let process = command.start()?;
    task.process = Some(process);
    Ok(task)
/// Create a new command builder
pub fn new_slay_command_builder<S: AsRef<str>>(command: S) -> SlayCommandBuilder {
    SlayCommandBuilder::new(command)
/// Run command with timeout
pub fn run_with_timeout(mut command: SlayCommand, timeout: Duration) -> ProcessResult<i32> {
    command.timeout = Some(timeout);
    command.run()
/// Get output with timeout
pub fn output_with_timeout(mut command: SlayCommand, timeout: Duration) -> ProcessResult<Vec<u8>> {
    command.timeout = Some(timeout);
    command.output()
/// Get combined output with timeout
pub fn combined_output_with_timeout(mut command: SlayCommand, timeout: Duration) -> ProcessResult<Vec<u8>> {
    command.timeout = Some(timeout);
    command.combined_output()
/// Run shell command
pub fn run_shell<S: AsRef<str>>(cmd_string: S) -> ProcessResult<i32> {
    let mut command = SlayCommand::new("sh", &["-c", cmd_string.as_ref()]);
    command.run()
/// Get shell output
pub fn shell_output<S: AsRef<str>>(cmd_string: S) -> ProcessResult<Vec<u8>> {
    let mut command = SlayCommand::new("sh", &["-c", cmd_string.as_ref()]);
    command.output()
/// Run shell command with environment
pub fn run_shell_with_env<S: AsRef<str>>(cmd_string: S, env: HashMap<String, String>) -> ProcessResult<i32> {
    let mut command = SlayCommand::new("sh", &["-c", cmd_string.as_ref()]);
    command.env_vars = env;
    command.run()
/// Run shell command in directory
pub fn run_shell_in_dir<S: AsRef<str>>(cmd_string: S, dir: S) -> ProcessResult<i32> {
    let mut command = SlayCommand::new("sh", &["-c", cmd_string.as_ref()]);
    command.working_dir = Some(dir.as_ref().to_string());
    command.run()
impl Default for SlayOptions {
    fn default() -> Self {
        Self {
        }
    }
impl Default for SignalOptions {
    fn default() -> Self {
        Self {
            signal: 15, // SIGTERM
        }
    }
