/// Core process management functionality for CURSED
/// 
/// This module provides the fundamental process operations including spawning,
/// configuration, I/O handling, and basic process control.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, execution_failed, execution_failed_with_code,
    timeout_error, invalid_arguments, io_error, system_error
};

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
                Ok(file.into())
            }
            ProcessIo::Custom(stdio) => {
                // Note: Can't clone Stdio, so this is a simplified approach
                Ok(Stdio::inherit())
            }
        }
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
    /// Process start time
    start_time: Instant,
    /// Output buffers (if capturing)
    output_buffer: Arc<Mutex<(Vec<u8>, Vec<u8>)>>,
}

impl Process {
    /// Get process ID
    pub fn id(&self) -> u32 {
        self.child.id()
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
}

impl Drop for Process {
    fn drop(&mut self) {
        // Attempt to kill the process if it's still running
        let _ = self.child.kill();
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
    command.stdout(config.stdout.to_stdio().map_err(|e| {
        io_error("configure_stdout", &format!("{:?}", e.kind()), &e.to_string())
    })?);
    command.stderr(config.stderr.to_stdio().map_err(|e| {
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

    // Spawn the process
    let child = command.spawn()
        .map_err(|e| execution_failed(&config.command, &e.to_string()))?;

    Ok(Process {
        config,
        child,
        start_time: Instant::now(),
        output_buffer: Arc::new(Mutex::new((Vec::new(), Vec::new()))),
    })
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
}
