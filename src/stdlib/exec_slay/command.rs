//! SlayCommand implementation for process execution

use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{self, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use crate::error::CursedError;
use super::{SlayOptions, SlayResult, SharedProcessState, io_error_to_cursed, get_shell_args};
use super::process::SlayProcess;

/// Represents an external command to be executed
#[derive(Debug)]
pub struct SlayCommand {
    /// Command name/path
    pub name: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Execution options
    pub options: SlayOptions,
    /// Shared process state
    pub(crate) state: Arc<Mutex<SharedProcessState>>,
}

impl SlayCommand {
    /// Create a new SlayCommand with the given name and arguments
    pub fn new(name: &str, args: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            options: SlayOptions::default(),
            state: Arc::new(Mutex::new(SharedProcessState::new())),
        }
    }

    /// Run the command and wait for completion
    pub fn run(&mut self) -> SlayResult<()> {
        self.start()?;
        self.wait()
    }

    /// Start the command without waiting for completion
    pub fn start(&mut self) -> SlayResult<()> {
        let mut cmd = self.build_command()?;
        
        // Configure stdio
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());

        // Spawn the process
        let child = cmd.spawn().map_err(io_error_to_cursed)?;
        
        // Store the child process
        {
            let mut state = self.state.lock().unwrap();
            state.child = Some(child);
            state.is_running = true;
            state.start_time = Instant::now();
        }

        Ok(())
    }

    /// Wait for the command to complete
    pub fn wait(&mut self) -> SlayResult<()> {
        let timeout = self.options.timeout;
        
        if let Some(timeout_duration) = timeout {
            self.wait_with_timeout(timeout_duration)
        } else {
            self.wait_indefinitely()
        }
    }

    /// Get the command output
    pub fn output(&mut self) -> SlayResult<Vec<u8>> {
        self.run()?;
        
        let state = self.state.lock().unwrap();
        Ok(state.stdout_data.clone())
    }

    /// Get combined stdout and stderr output
    pub fn combined_output(&mut self) -> SlayResult<Vec<u8>> {
        self.run()?;
        
        let state = self.state.lock().unwrap();
        let mut combined = state.stdout_data.clone();
        combined.extend_from_slice(&state.stderr_data);
        Ok(combined)
    }

    /// Configure the command with options
    pub fn with_options(mut self, options: SlayOptions) -> Self {
        self.options = options;
        self
    }

    /// Set the working directory
    pub fn set_dir(&mut self, dir: &str) -> &mut Self {
        self.options.dir = Some(dir.to_string());
        self
    }

    /// Set environment variables
    pub fn set_env(&mut self, env: Vec<String>) -> &mut Self {
        self.options.env = env;
        self
    }

    /// Add an environment variable
    pub fn add_env(&mut self, key: &str, value: &str) -> &mut Self {
        self.options.env.push(format!("{}={}", key, value));
        self
    }

    /// Set timeout for execution
    pub fn set_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.options.timeout = Some(timeout);
        self
    }

    /// Enable shell execution
    pub fn use_shell(&mut self, use_shell: bool) -> &mut Self {
        self.options.use_shell = use_shell;
        self
    }

    /// Set shell path
    pub fn set_shell_path(&mut self, path: &str) -> &mut Self {
        self.options.shell_path = Some(path.to_string());
        self
    }

    /// Get the process handle
    pub fn process(&self) -> Option<SlayProcess> {
        let state = self.state.lock().unwrap();
        if state.is_running {
            Some(SlayProcess::new(self.state.clone()))
        } else {
            None
        }
    }

    /// Check if the command is currently running
    pub fn is_running(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.is_running
    }

    /// Get the exit code if the process has completed
    pub fn exit_code(&self) -> Option<i32> {
        let state = self.state.lock().unwrap();
        state.exit_status.as_ref().and_then(|status| status.code())
    }

    /// Get string representation of the command
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
    }

    /// Build the underlying Command object
    fn build_command(&self) -> SlayResult<Command> {
        let mut cmd = if self.options.use_shell {
            let shell_args = get_shell_args(true, self.options.shell_path.as_deref());
            let mut shell_cmd = Command::new(&shell_args[0]);
            
            if shell_args.len() > 1 {
                shell_cmd.args(&shell_args[1..]);
            }
            
            // Build the full command string
            let full_cmd = format!("{} {}", self.name, self.args.join(" "));
            shell_cmd.arg(full_cmd);
            shell_cmd
        } else {
            let mut direct_cmd = Command::new(&self.name);
            direct_cmd.args(&self.args);
            direct_cmd
        };

        // Set working directory
        if let Some(ref dir) = self.options.dir {
            cmd.current_dir(dir);
        }

        // Set environment variables
        for env_var in &self.options.env {
            if let Some(eq_pos) = env_var.find('=') {
                let key = &env_var[..eq_pos];
                let value = &env_var[eq_pos + 1..];
                cmd.env(key, value);
            }
        }

        Ok(cmd)
    }

    /// Wait for completion with timeout
    fn wait_with_timeout(&mut self, timeout: Duration) -> SlayResult<()> {
        let start_time = Instant::now();
        
        loop {
            // Check if process has completed
            {
                let mut state = self.state.lock().unwrap();
                if let Some(ref mut child) = state.child {
                    match child.try_wait() {
                        Ok(Some(status)) => {
                            state.exit_status = Some(status);
                            state.is_running = false;
                            
                            // Collect output if configured
                            if self.options.collect_output {
                                self.collect_output(&mut state, child)?;
                            }
                            
                            return Ok(());
                        }
                        Ok(None) => {
                            // Process still running, check timeout
                            if start_time.elapsed() >= timeout {
                                // Kill the process
                                let _ = child.kill();
                                state.is_running = false;
                                return Err(CursedError::RuntimeError(
                                    format!("Command timed out after {:?}", timeout)
                                ));
                            }
                        }
                        Err(e) => {
                            state.is_running = false;
                            return Err(io_error_to_cursed(e));
                        }
                    }
                }
            }
            
            // Sleep briefly before checking again
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Wait for completion indefinitely
    fn wait_indefinitely(&mut self) -> SlayResult<()> {
        let mut state = self.state.lock().unwrap();
        
        if let Some(ref mut child) = state.child {
            let status = child.wait().map_err(io_error_to_cursed)?;
            state.exit_status = Some(status);
            state.is_running = false;
            
            // Collect output if configured
            if self.options.collect_output {
                self.collect_output(&mut state, child)?;
            }
        }
        
        Ok(())
    }

    /// Collect output from the process
    fn collect_output(&self, state: &mut SharedProcessState, child: &mut Child) -> SlayResult<()> {
        // Read stdout
        if let Some(ref mut stdout) = child.stdout {
            let mut stdout_data = Vec::new();
            stdout.read_to_end(&mut stdout_data).map_err(io_error_to_cursed)?;
            state.stdout_data = stdout_data;
        }

        // Read stderr
        if let Some(ref mut stderr) = child.stderr {
            let mut stderr_data = Vec::new();
            stderr.read_to_end(&mut stderr_data).map_err(io_error_to_cursed)?;
            state.stderr_data = stderr_data;
        }

        Ok(())
    }
}

impl Clone for SlayCommand {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            args: self.args.clone(),
            options: self.options.clone(),
            state: Arc::new(Mutex::new(SharedProcessState::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
use crate::stdlib::process::info::ProcessState;

    #[test]
    fn test_new_slay_command() {
        let cmd = SlayCommand::new("echo", &["hello", "world"]);
        assert_eq!(cmd.name, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_slay_command_to_string() {
        let cmd = SlayCommand::new("ls", &["-la", "/tmp"]);
        let cmd_str = cmd.to_string();
        assert_eq!(cmd_str, "ls -la /tmp");
    }

    #[test]
    fn test_slay_command_with_options() {
        let mut options = SlayOptions::default();
        options.timeout = Some(Duration::from_secs(10));
        options.use_shell = true;

        let cmd = SlayCommand::new("echo", &["test"])
            .with_options(options);
        
        assert_eq!(cmd.options.timeout, Some(Duration::from_secs(10)));
        assert!(cmd.options.use_shell);
    }

    #[test]
    fn test_slay_command_set_dir() {
        let mut cmd = SlayCommand::new("pwd", &[]);
        cmd.set_dir("/tmp");
        assert_eq!(cmd.options.dir, Some("/tmp".to_string()));
    }

    #[test]
    fn test_slay_command_add_env() {
        let mut cmd = SlayCommand::new("env", &[]);
        cmd.add_env("TEST_VAR", "test_value");
        assert!(cmd.options.env.contains(&"TEST_VAR=test_value".to_string()));
    }

    #[test]
    fn test_slay_command_clone() {
        let cmd1 = SlayCommand::new("echo", &["test"]);
        let cmd2 = cmd1.clone();
        
        assert_eq!(cmd1.name, cmd2.name);
        assert_eq!(cmd1.args, cmd2.args);
    }

    #[cfg(unix)]
    #[test]
    fn test_simple_command_execution() {
        let mut cmd = SlayCommand::new("echo", &["hello"]);
        let result = cmd.run();
        assert!(result.is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn test_command_output() {
        let mut cmd = SlayCommand::new("echo", &["hello"]);
        let output = cmd.output().unwrap();
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("hello"));
    }

    #[cfg(unix)]
    #[test]
    fn test_command_with_timeout() {
        let mut cmd = SlayCommand::new("sleep", &["0.1"]);
        cmd.set_timeout(Duration::from_secs(1));
        let result = cmd.run();
        assert!(result.is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn test_command_timeout_exceeded() {
        let mut cmd = SlayCommand::new("sleep", &["2"]);
        cmd.set_timeout(Duration::from_millis(100));
        let result = cmd.run();
        assert!(result.is_err());
    }
}
