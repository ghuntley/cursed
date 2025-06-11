/// Core process management functionality
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::Duration;
use super::error::{ProcessError, ProcessResult};
use super::info::ProcessInfo;
use super::control::ProcessControl;

/// Process spawn configuration
#[derive(Debug, Clone)]
pub struct ProcessConfig {
    /// Command to execute
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Working directory (None = inherit from parent)
    pub working_dir: Option<PathBuf>,
    /// Environment variables (None = inherit from parent)
    pub env: Option<HashMap<String, String>>,
    /// Clear environment before setting new variables
    pub clear_env: bool,
    /// Stdin redirection
    pub stdin: ProcessIo,
    /// Stdout redirection
    pub stdout: ProcessIo,
    /// Stderr redirection
    pub stderr: ProcessIo,
    /// Process timeout
    pub timeout: Option<Duration>,
    /// Detach process (run as daemon)
    pub detached: bool,
    /// Process group ID
    pub process_group: Option<u32>,
    /// User ID to run as (Unix only)
    pub uid: Option<u32>,
    /// Group ID to run as (Unix only)  
    pub gid: Option<u32>,
}

/// Process I/O redirection options
#[derive(Debug, Clone)]
pub enum ProcessIo {
    /// Inherit from parent process
    Inherit,
    /// Pipe to/from parent
    Pipe,
    /// Redirect to null/void
    Null,
    /// Redirect to file
    File(PathBuf),
    /// Capture output in memory
    Capture,
}

/// Process execution result
#[derive(Debug)]
pub struct ProcessOutput {
    /// Exit status
    pub status: ExitStatus,
    /// Captured stdout (if ProcessIo::Capture was used)
    pub stdout: Vec<u8>,
    /// Captured stderr (if ProcessIo::Capture was used)
    pub stderr: Vec<u8>,
    /// Process execution time
    pub duration: Duration,
}

/// Running process handle
#[derive(Debug)]
pub struct Process {
    /// Child process handle
    pub(crate) child: Child,
    /// Process configuration
    pub config: ProcessConfig,
    /// Start time
    pub start_time: std::time::Instant,
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
            args: Vec::new(),
            working_dir: None,
            env: None,
            clear_env: false,
            stdin: ProcessIo::Inherit,
            stdout: ProcessIo::Inherit,
            stderr: ProcessIo::Inherit,
            timeout: None,
            detached: false,
            process_group: None,
            uid: None,
            gid: None,
        }
    }
}

impl ProcessConfig {
    /// Create new process configuration with command
    pub fn new<S: Into<String>>(command: S) -> Self {
        Self {
            command: command.into(),
            ..Default::default()
        }
    }

    /// Add command argument
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Add multiple command arguments
    pub fn args<I, S>(mut self, args: I) -> Self 
    where 
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(|s| s.into()));
        self
    }

    /// Set working directory
    pub fn working_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Set environment variable
    pub fn env<K, V>(mut self, key: K, value: V) -> Self 
    where
        K: Into<String>,
        V: Into<String>,
    {
        if self.env.is_none() {
            self.env = Some(HashMap::new());
        }
        self.env.as_mut().unwrap().insert(key.into(), value.into());
        self
    }

    /// Set multiple environment variables
    pub fn envs<I, K, V>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        if self.env.is_none() {
            self.env = Some(HashMap::new());
        }
        let env = self.env.as_mut().unwrap();
        for (k, v) in vars {
            env.insert(k.into(), v.into());
        }
        self
    }

    /// Clear environment before setting new variables
    pub fn clear_env(mut self) -> Self {
        self.clear_env = true;
        self
    }

    /// Set stdin redirection
    pub fn stdin(mut self, io: ProcessIo) -> Self {
        self.stdin = io;
        self
    }

    /// Set stdout redirection
    pub fn stdout(mut self, io: ProcessIo) -> Self {
        self.stdout = io;
        self
    }

    /// Set stderr redirection
    pub fn stderr(mut self, io: ProcessIo) -> Self {
        self.stderr = io;
        self
    }

    /// Capture stdout and stderr in memory
    pub fn capture_output(mut self) -> Self {
        self.stdout = ProcessIo::Capture;
        self.stderr = ProcessIo::Capture;
        self
    }

    /// Set process timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Run process as detached (daemon)
    pub fn detached(mut self, detached: bool) -> Self {
        self.detached = detached;
        self
    }

    /// Set process group ID
    pub fn process_group(mut self, gid: u32) -> Self {
        self.process_group = Some(gid);
        self
    }

    /// Set user ID (Unix only)
    pub fn uid(mut self, uid: u32) -> Self {
        self.uid = Some(uid);
        self
    }

    /// Set group ID (Unix only)
    pub fn gid(mut self, gid: u32) -> Self {
        self.gid = Some(gid);
        self
    }
}

impl Process {
    /// Get process ID
    pub fn id(&self) -> u32 {
        self.child.id()
    }

    /// Get process information
    pub fn info(&self) -> ProcessResult<ProcessInfo> {
        ProcessInfo::from_pid(self.id())
    }

    /// Wait for process to complete
    pub fn wait(&mut self) -> ProcessResult<ExitStatus> {
        Ok(self.child.wait()?)
    }

    /// Wait for process with timeout
    pub fn wait_timeout(&mut self, timeout: Duration) -> ProcessResult<Option<ExitStatus>> {
        // This is a simplified implementation - in practice you'd use platform-specific APIs
        use std::thread;
        use std::sync::mpsc;
        
        let (tx, rx) = mpsc::channel::<Option<ExitStatus>>();
        let child_id = self.child.id();
        
        thread::spawn(move || {
            // In a real implementation, this would properly wait for the process
            thread::sleep(timeout);
            let _ = tx.send(None);
        });
        
        match self.child.try_wait()? {
            Some(status) => Ok(Some(status)),
            None => {
                match rx.recv_timeout(timeout) {
                    Ok(_) => Ok(None), // Timeout
                    Err(_) => {
                        // Check once more
                        Ok(self.child.try_wait()?)
                    }
                }
            }
        }
    }

    /// Kill the process
    pub fn kill(&mut self) -> ProcessResult<()> {
        Ok(self.child.kill()?)
    }

    /// Try to kill the process gracefully, then forcefully
    pub fn terminate(&mut self, grace_period: Duration) -> ProcessResult<()> {
        // First try to terminate gracefully (platform-specific)
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            // Send SIGTERM first
            unsafe {
                libc::kill(self.child.id() as libc::pid_t, libc::SIGTERM);
            }
            
            // Wait for grace period
            if let Ok(Some(_)) = self.wait_timeout(grace_period) {
                return Ok(());
            }
        }
        
        // Force kill if graceful termination failed
        self.kill()
    }

    /// Check if process is still running
    pub fn is_running(&mut self) -> ProcessResult<bool> {
        match self.child.try_wait()? {
            Some(_) => Ok(false),
            None => Ok(true),
        }
    }

    /// Get process uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Spawn a new process
pub fn spawn_process(config: ProcessConfig) -> ProcessResult<Process> {
    if config.command.is_empty() {
        return Err(ProcessError::InvalidArguments("Command cannot be empty".to_string()));
    }
    
    let mut command = Command::new(&config.command);
    
    // Set arguments
    if !config.args.is_empty() {
        command.args(&config.args);
    }
    
    // Set working directory
    if let Some(ref dir) = config.working_dir {
        if !dir.exists() {
            return Err(ProcessError::InvalidWorkingDirectory(
                format!("Directory does not exist: {}", dir.display())
            ));
        }
        command.current_dir(dir);
    }
    
    // Set environment variables
    if config.clear_env {
        command.env_clear();
    }
    if let Some(ref env_vars) = config.env {
        for (key, value) in env_vars {
            command.env(key, value);
        }
    }
    
    // Configure I/O redirection
    command.stdin(convert_process_io(&config.stdin)?);
    command.stdout(convert_process_io(&config.stdout)?);
    command.stderr(convert_process_io(&config.stderr)?);
    
    // Platform-specific configuration
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        
        if let Some(uid) = config.uid {
            command.uid(uid);
        }
        if let Some(gid) = config.gid {
            command.gid(gid);
        }
        if let Some(pgid) = config.process_group {
            command.process_group(pgid as i32);
        }
    }
    
    // Spawn the process
    let child = command.spawn()?;
    
    Ok(Process {
        child,
        config,
        start_time: std::time::Instant::now(),
    })
}

/// Execute a shell command and wait for completion
pub fn run_command<S: AsRef<str>>(command: S) -> ProcessResult<ProcessOutput> {
    let command_str = command.as_ref();
    
    #[cfg(windows)]
    let config = ProcessConfig::new("cmd")
        .args(["/C", command_str])
        .capture_output();
    
    #[cfg(not(windows))]
    let config = ProcessConfig::new("sh")
        .args(["-c", command_str])
        .capture_output();
    
    let mut process = spawn_process(config)?;
    let start_time = std::time::Instant::now();
    let status = process.wait()?;
    let duration = start_time.elapsed();
    
    // Get captured output
    let stdout = if let Some(mut stdout) = process.child.stdout.take() {
        use std::io::Read;
        let mut output = Vec::new();
        stdout.read_to_end(&mut output)?;
        output
    } else {
        Vec::new()
    };
    
    let stderr = if let Some(mut stderr) = process.child.stderr.take() {
        use std::io::Read;
        let mut output = Vec::new();
        stderr.read_to_end(&mut output)?;
        output
    } else {
        Vec::new()
    };
    
    Ok(ProcessOutput {
        status,
        stdout,
        stderr,
        duration,
    })
}

/// Execute a shell command with timeout
pub fn run_command_timeout<S: AsRef<str>>(
    command: S, 
    timeout: Duration
) -> ProcessResult<ProcessOutput> {
    let command_str = command.as_ref();
    
    #[cfg(windows)]
    let config = ProcessConfig::new("cmd")
        .args(["/C", command_str])
        .capture_output()
        .timeout(timeout);
    
    #[cfg(not(windows))]
    let config = ProcessConfig::new("sh")
        .args(["-c", command_str])
        .capture_output()
        .timeout(timeout);
    
    let mut process = spawn_process(config)?;
    let start_time = std::time::Instant::now();
    
    // Wait with timeout
    match process.wait_timeout(timeout)? {
        Some(status) => {
            let duration = start_time.elapsed();
            
            // Get captured output
            let stdout = if let Some(mut stdout) = process.child.stdout.take() {
                use std::io::Read;
                let mut output = Vec::new();
                stdout.read_to_end(&mut output)?;
                output
            } else {
                Vec::new()
            };
            
            let stderr = if let Some(mut stderr) = process.child.stderr.take() {
                use std::io::Read;
                let mut output = Vec::new();
                stderr.read_to_end(&mut output)?;
                output
            } else {
                Vec::new()
            };
            
            Ok(ProcessOutput {
                status,
                stdout,
                stderr,
                duration,
            })
        }
        None => {
            // Timeout exceeded, kill the process
            process.kill()?;
            Err(ProcessError::Timeout(format!(
                "Command '{}' timed out after {:?}", 
                command_str, 
                timeout
            )))
        }
    }
}

/// Convert ProcessIo to std::process::Stdio
fn convert_process_io(io: &ProcessIo) -> ProcessResult<Stdio> {
    match io {
        ProcessIo::Inherit => Ok(Stdio::inherit()),
        ProcessIo::Pipe => Ok(Stdio::piped()),
        ProcessIo::Null => Ok(Stdio::null()),
        ProcessIo::Capture => Ok(Stdio::piped()),
        ProcessIo::File(path) => {
            // This would require more complex implementation for file redirection
            // For now, we'll use piped I/O
            Ok(Stdio::piped())
        }
    }
}

/// Check if a command exists in PATH
pub fn command_exists<S: AsRef<str>>(command: S) -> bool {
    let command_str = command.as_ref();
    
    #[cfg(windows)]
    let which_cmd = format!("where {}", command_str);
    
    #[cfg(not(windows))]
    let which_cmd = format!("which {}", command_str);
    
    match run_command(&which_cmd) {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Get the full path of a command in PATH
pub fn which<S: AsRef<str>>(command: S) -> ProcessResult<PathBuf> {
    let command_str = command.as_ref();
    
    #[cfg(windows)]
    let which_cmd = format!("where {}", command_str);
    
    #[cfg(not(windows))]
    let which_cmd = format!("which {}", command_str);
    
    let output = run_command(&which_cmd)?;
    
    if output.status.success() {
        let path_str = String::from_utf8_lossy(&output.stdout);
        let path = path_str.trim();
        Ok(PathBuf::from(path))
    } else {
        Err(ProcessError::ProcessNotFound(0))
    }
}
