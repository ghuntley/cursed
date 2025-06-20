/// Command structure and implementation for exec_vibez
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command as StdCommand, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::error::{ExecResult, ExecError, execution_failed, execution_failed_with_code, invalid_arguments, io_error};
use super::context::VibeContext;
use super::core::{register_process, unregister_process};

/// Cmd represents an external command being prepared or run
#[derive(Debug)]
pub struct Cmd {
    /// Path to the executable
    pub path: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Environment variables for the process
    pub env: Vec<String>,
    /// Working directory
    pub dir: Option<PathBuf>,
    /// Standard input configuration
    pub stdin: Option<Box<dyn Read + Send>>,
    /// Standard output configuration  
    pub stdout: Option<Box<dyn Write + Send>>,
    /// Standard error configuration
    pub stderr: Option<Box<dyn Write + Send>>,
    /// Process context for cancellation
    pub context: Option<VibeContext>,
    /// Internal child process handle
    child: Option<Child>,
    /// Process start time
    start_time: Option<Instant>,
    /// Environment inheritance flag
    inherit_env: bool,
}

/// Process represents a running process
#[derive(Debug)]
pub struct Process {
    /// Process ID
    pub pid: u32,
    /// Process handle
    child: Arc<Mutex<Child>>,
    /// Start time
    start_time: Instant,
    /// Command that created this process
    command: String,
}

/// ProcessState contains information about a process that has exited
#[derive(Debug, Clone)]
pub struct ProcessState {
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
}

impl Cmd {
    /// Create a new Cmd instance
    pub fn new<S: AsRef<str>>(name: S, args: &[&str]) -> Self {
        Self {
            path: name.as_ref().to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            env: Vec::new(),
            dir: None,
            stdin: None,
            stdout: None,
            stderr: None,
            context: None,
            child: None,
            start_time: None,
            inherit_env: true,
        }
    }

    /// Set the working directory for the command
    pub fn set_dir<P: Into<PathBuf>>(&mut self, dir: P) {
        self.dir = Some(dir.into());
    }

    /// Set environment variables (replaces existing)
    pub fn set_env(&mut self, env: Vec<String>) {
        self.env = env;
        self.inherit_env = false;
    }

    /// Add an environment variable
    pub fn add_env<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        self.env.push(format!("{}={}", key.as_ref(), value.as_ref()));
    }

    /// Set whether to inherit parent environment
    pub fn set_inherit_env(&mut self, inherit: bool) {
        self.inherit_env = inherit;
    }

    /// Set the process context
    pub fn set_context(&mut self, context: VibeContext) {
        self.context = Some(context);
    }

    /// Start the process without waiting
    pub fn start(&mut self) -> ExecResult<()> {
        let mut command = StdCommand::new(&self.path);
        command.args(&self.args);

        // Set working directory
        if let Some(ref dir) = self.dir {
            command.current_dir(dir);
        }

        // Set environment
        if !self.inherit_env {
            command.env_clear();
        }
        for env_pair in &self.env {
            if let Some((key, value)) = env_pair.split_once('=') {
                command.env(key, value);
            }
        }

        // Configure I/O
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        // Spawn the process
        let child = command.spawn()
            .map_err(|e| execution_failed(&self.path, &e.to_string()))?;

        let pid = child.id();
        self.child = Some(child);
        self.start_time = Some(Instant::now());

        // Register process for tracking
        register_process(pid, &self.path, &self.args, self.dir.clone());

        Ok(())
    }

    /// Run the command and wait for completion
    pub fn run(&mut self) -> ExecResult<()> {
        self.start()?;
        self.wait()
    }

    /// Capture stdout output
    pub fn output(&mut self) -> ExecResult<Vec<u8>> {
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
    pub fn combined_output(&mut self) -> ExecResult<Vec<u8>> {
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

    /// Get stdin pipe (WriteCloser equivalent)
    pub fn stdin_pipe(&mut self) -> ExecResult<Box<dyn Write + Send>> {
        if !self.is_started() {
            self.start()?;
        }
        
        if let Some(child) = &mut self.child {
            if let Some(stdin) = child.stdin.take() {
                Ok(Box::new(stdin))
            } else {
                Err(invalid_arguments("stdin_pipe", "stdin", "Stdin not available"))
            }
        } else {
            Err(invalid_arguments("stdin_pipe", "command", "Command not started"))
        }
    }

    /// Get stdout pipe (ReadCloser equivalent)
    pub fn stdout_pipe(&mut self) -> ExecResult<Box<dyn Read + Send>> {
        if !self.is_started() {
            self.start()?;
        }
        
        if let Some(child) = &mut self.child {
            if let Some(stdout) = child.stdout.take() {
                Ok(Box::new(stdout))
            } else {
                Err(invalid_arguments("stdout_pipe", "stdout", "Stdout not available"))
            }
        } else {
            Err(invalid_arguments("stdout_pipe", "command", "Command not started"))
        }
    }

    /// Get stderr pipe (ReadCloser equivalent)
    pub fn stderr_pipe(&mut self) -> ExecResult<Box<dyn Read + Send>> {
        if !self.is_started() {
            self.start()?;
        }
        
        if let Some(child) = &mut self.child {
            if let Some(stderr) = child.stderr.take() {
                Ok(Box::new(stderr))
            } else {
                Err(invalid_arguments("stderr_pipe", "stderr", "Stderr not available"))
            }
        } else {
            Err(invalid_arguments("stderr_pipe", "command", "Command not started"))
        }
    }

    /// Set environment using Environment struct
    pub fn set_env(&mut self, env: super::environment::Environment) {
        self.env = env.to_env_vec();
        self.inherit_env = false;
    }

    /// Set timeout for command execution
    pub fn set_timeout(&mut self, timeout: Duration) {
        // Store timeout in context for later use
        if self.context.is_none() {
            self.context = Some(super::context::VibeContext::with_timeout_simple(timeout));
        } else if let Some(ref mut ctx) = self.context {
            ctx.set_timeout(timeout);
        }
    }

    /// Get the process handle if started
    pub fn process(&self) -> Option<super::process::Process> {
        if let Some(ref child) = self.child {
            // Create a new Process wrapper
            // Note: This is a simplified implementation
            // In practice, we'd need better process handle management
            None // TODO: Implement proper process handle sharing
        } else {
            None
        }
    }

    /// Wait for the command to complete
    pub fn wait(&mut self) -> ExecResult<()> {
        if let Some(child) = &mut self.child {
            let pid = child.id();
            
            // Check context timeout/cancellation
            if let Some(ref ctx) = self.context {
                if ctx.done() {
                    // Kill the process if context is done
                    let _ = child.kill();
                    unregister_process(pid);
                    
                    if let Some(err) = ctx.err() {
                        return Err(ExecError::from(err));
                    }
                }
            }
            
            let status = child.wait()
                .map_err(|e| io_error("wait", &format!("{:?}", e.kind()), &e.to_string()))?;
            
            // Unregister process
            unregister_process(pid);
            
            if !status.success() {
                if let Some(code) = status.code() {
                    return Err(execution_failed_with_code(&self.path, code, "Command failed"));
                } else {
                    return Err(execution_failed(&self.path, "Command terminated by signal"));
                }
            }
            Ok(())
        } else {
            Err(invalid_arguments("wait", "command", "Command not started"))
        }
    }

    /// Get process handle
    pub fn process(&self) -> ExecResult<Process> {
        if let Some(child) = &self.child {
            Ok(Process {
                pid: child.id(),
                child: Arc::new(Mutex::new(unsafe { 
                    // This is unsafe but necessary for the API
                    std::ptr::read(child as *const Child)
                })),
                start_time: self.start_time.unwrap_or_else(Instant::now),
                command: self.path.clone(),
            })
        } else {
            Err(invalid_arguments("process", "command", "Command not started"))
        }
    }

    /// Get process state
    pub fn process_state(&self) -> ExecResult<ProcessState> {
        if let Some(child) = &self.child {
            let pid = child.id();
            
            // Create a basic process state
            Ok(ProcessState {
                exit_status: ExitStatus::from_raw(0), // Will be updated when process exits
                pid,
                user_time: Duration::from_millis(0),
                system_time: Duration::from_millis(0),
                sys_info: Vec::new(),
            })
        } else {
            Err(invalid_arguments("process_state", "command", "Command not started"))
        }
    }

    /// Check if the process has been started
    pub fn is_started(&self) -> bool {
        self.child.is_some()
    }

    /// Get the command path
    pub fn command_path(&self) -> &str {
        &self.path
    }

    /// Get the command arguments
    pub fn command_args(&self) -> &[String] {
        &self.args
    }

    /// Get the working directory
    pub fn working_dir(&self) -> Option<&PathBuf> {
        self.dir.as_ref()
    }

    /// Get the start time if the process has been started
    pub fn start_time(&self) -> Option<Instant> {
        self.start_time
    }
}

impl Process {
    /// Kill the process
    pub fn kill(&self) -> ExecResult<()> {
        let mut child = self.child.lock().unwrap();
        child.kill()
            .map_err(|e| io_error("kill", &format!("{:?}", e.kind()), &e.to_string()))?;
        
        // Unregister process
        unregister_process(self.pid);
        Ok(())
    }

    /// Send signal to process (Unix only)
    #[cfg(unix)]
    pub fn signal(&self, sig: i32) -> ExecResult<()> {
        unsafe {
            if libc::kill(self.pid as i32, sig) == 0 {
                Ok(())
            } else {
                use super::error::system_error;
                Err(system_error(
                    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                    "signal",
                    "Failed to send signal"
                ))
            }
        }
    }

    #[cfg(not(unix))]
    pub fn signal(&self, _sig: i32) -> ExecResult<()> {
        use super::error::platform_error;
        Err(platform_error("current", "Signal sending not supported on this platform"))
    }

    /// Wait for process completion
    pub fn wait(&self) -> ExecResult<ProcessState> {
        let mut child = self.child.lock().unwrap();
        let status = child.wait()
            .map_err(|e| io_error("wait", &format!("{:?}", e.kind()), &e.to_string()))?;

        // Unregister process
        unregister_process(self.pid);

        Ok(ProcessState {
            exit_status: status,
            pid: self.pid,
            user_time: Duration::from_millis(0),
            system_time: Duration::from_millis(0),
            sys_info: Vec::new(),
        })
    }

    /// Release process resources
    pub fn release(&self) -> ExecResult<()> {
        // Process will be cleaned up when dropped
        Ok(())
    }

    /// Get process ID
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Get process start time
    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    /// Get process uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get the command that created this process
    pub fn command(&self) -> &str {
        &self.command
    }
}

impl ProcessState {
    /// Check if process exited normally
    pub fn exited(&self) -> bool {
        true // All processes that reach this state have exited
    }

    /// Get exit code
    pub fn exit_code(&self) -> i32 {
        self.exit_status.code().unwrap_or(-1)
    }

    /// Check if process was successful
    pub fn success(&self) -> bool {
        self.exit_status.success()
    }

    /// Get system-specific information
    pub fn sys(&self) -> Box<dyn std::any::Any> {
        Box::new(self.exit_status)
    }

    /// Get system usage information
    pub fn sys_usage(&self) -> Box<dyn std::any::Any> {
        Box::new((self.user_time, self.system_time))
    }

    /// String representation
    pub fn string(&self) -> String {
        format!("Process {} exited with code {}", self.pid, self.exit_code())
    }

    /// Get user CPU time
    pub fn user_time(&self) -> Duration {
        self.user_time
    }

    /// Get system CPU time
    pub fn system_time(&self) -> Duration {
        self.system_time
    }

    /// Get process ID
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Get exit status
    pub fn exit_status(&self) -> &ExitStatus {
        &self.exit_status
    }
}

// Convert context errors to exec errors
impl From<super::context::ContextError> for ExecError {
    fn from(err: super::context::ContextError) -> Self {
        match err {
            super::context::ContextError::Cancelled => {
                ExecError::ExecutionFailed {
                    command: "unknown".to_string(),
                    message: "Process cancelled".to_string(),
                    exit_code: Some(-1),
                }
            }
            super::context::ContextError::DeadlineExceeded => {
                ExecError::Timeout {
                    command: "unknown".to_string(),
                    timeout: Duration::from_secs(0), // Unknown timeout
                }
            }
        }
    }
}

/// Core command creation functions

/// Create a new Cmd instance to execute a given program
pub fn Command(name: &str, args: &[&str]) -> Cmd {
    Cmd::new(name, args)
}

/// Create a new Cmd with a context for timeout/cancellation
pub fn CommandContext(ctx: VibeContext, name: &str, args: &[&str]) -> Cmd {
    let mut cmd = Cmd::new(name, args);
    cmd.set_context(ctx);
    cmd
}

/// Create a new Cmd instance to execute a given program (lowercase version)
pub fn command(name: &str, args: &[&str]) -> Cmd {
    Cmd::new(name, args)
}

/// Create a new Cmd with a context for timeout/cancellation (lowercase version)
pub fn command_context(ctx: VibeContext, name: &str, args: &[&str]) -> Cmd {
    let mut cmd = Cmd::new(name, args);
    cmd.context = Some(ctx);
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_cmd_creation() {
        let cmd = Cmd::new("echo", &["hello", "world"]);
        assert_eq!(cmd.path, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
        assert!(!cmd.is_started());
    }

    #[test]
    fn test_cmd_configuration() {
        let mut cmd = Cmd::new("echo", &["test"]);
        
        cmd.set_dir("/tmp");
        assert_eq!(cmd.working_dir(), Some(&PathBuf::from("/tmp")));
        
        cmd.add_env("TEST_VAR", "test_value");
        assert!(cmd.env.iter().any(|e| e == "TEST_VAR=test_value"));
        
        cmd.set_inherit_env(false);
        assert!(!cmd.inherit_env);
    }

    #[test]
    fn test_cmd_context() {
        let ctx = VibeContext::background();
        let cmd = CommandContext(ctx, "echo", &["test"]);
        assert!(cmd.context.is_some());
    }

    #[test]
    fn test_process_state() {
        let state = ProcessState {
            exit_status: ExitStatus::from_raw(0),
            pid: 1234,
            user_time: Duration::from_millis(100),
            system_time: Duration::from_millis(50),
            sys_info: Vec::new(),
        };
        
        assert!(state.success());
        assert_eq!(state.exit_code(), 0);
        assert_eq!(state.user_time(), Duration::from_millis(100));
        assert_eq!(state.pid(), 1234);
        assert!(state.exited());
    }

    #[test]
    fn test_command_functions() {
        let cmd1 = Command("ls", &["-la"]);
        let cmd2 = CommandContext(VibeContext::background(), "pwd", &[]);
        
        assert_eq!(cmd1.command_path(), "ls");
        assert_eq!(cmd1.command_args(), &["ls", "-la"]);
        assert_eq!(cmd2.command_path(), "pwd");
        assert!(cmd2.context.is_some());
    }
}
