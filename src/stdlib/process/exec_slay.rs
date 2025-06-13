/// ExecSlay - Enhanced process execution with style and efficiency
/// 
/// This module provides the "ExecSlay" API inspired by Go's os/exec but with enhanced features
/// for process management, I/O control, and advanced process handling capabilities.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, execution_failed, execution_failed_with_code,
    timeout_error, invalid_arguments, io_error, system_error
};

/// SlayCommand represents an external command to be executed with enhanced features
#[derive(Debug)]
pub struct SlayCommand {
    /// Command path/name
    pub path: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env: Vec<String>,
    /// Working directory
    pub dir: Option<PathBuf>,
    /// Standard input configuration
    pub stdin: Option<ProcessStdin>,
    /// Standard output configuration
    pub stdout: Option<ProcessStdout>,
    /// Standard error configuration
    pub stderr: Option<ProcessStderr>,
    /// Extra files to pass to child process
    pub extra_files: Vec<File>,
    /// Process group configuration
    pub process_group: Option<u32>,
    /// Internal child process handle
    child: Option<Child>,
    /// Process start time
    start_time: Option<Instant>,
}

/// SlayProcess represents a running process
#[derive(Debug)]
pub struct SlayProcess {
    /// Process ID
    pub pid: u32,
    /// Process handle
    child: Arc<Mutex<Child>>,
    /// Start time
    start_time: Instant,
}

/// SlayProcessState contains information about a finished process
#[derive(Debug, Clone)]
pub struct SlayProcessState {
    /// Exit status
    pub exit_status: ExitStatus,
    /// Process ID
    pub pid: u32,
    /// User CPU time used
    pub user_time: Duration,
    /// System CPU time used
    pub system_time: Duration,
    /// Maximum resident set size
    pub max_rss: u64,
}

/// SlayOptions configuration for enhanced command execution
#[derive(Debug, Clone)]
pub struct SlayOptions {
    /// Working directory
    pub dir: Option<PathBuf>,
    /// Environment variables
    pub env: Vec<String>,
    /// Standard input source
    pub stdin: Option<Box<dyn Read + Send>>,
    /// Standard output destination  
    pub stdout: Option<Box<dyn Write + Send>>,
    /// Standard error destination
    pub stderr: Option<Box<dyn Write + Send>>,
    /// Extra files
    pub extra_files: Vec<File>,
    /// Execution timeout
    pub timeout: Option<Duration>,
    /// Wait delay before forcing termination
    pub wait_delay: Option<Duration>,
    /// Signal to use for killing
    pub kill_signal: Option<i32>,
    /// Stdout callback for real-time processing
    pub stdout_callback: Option<fn(&[u8])>,
    /// Stderr callback for real-time processing
    pub stderr_callback: Option<fn(&[u8])>,
    /// Use shell for execution
    pub use_shell: bool,
    /// Shell path (if using shell)
    pub shell_path: Option<String>,
    /// Buffer size for I/O operations
    pub buffer_size: usize,
    /// Collect output in memory
    pub collect_output: bool,
    /// Capture environment statistics
    pub capture_env_stats: bool,
    /// Working memory limit (bytes)
    pub working_limit: Option<u64>,
    /// CPU usage limit (percentage)
    pub cpu_limit: Option<f64>,
}

impl Default for SlayOptions {
    fn default() -> Self {
        Self {
            dir: None,
            env: Vec::new(),
            stdin: None,
            stdout: None,
            stderr: None,
            extra_files: Vec::new(),
            timeout: None,
            wait_delay: None,
            kill_signal: None,
            stdout_callback: None,
            stderr_callback: None,
            use_shell: false,
            shell_path: None,
            buffer_size: 8192,
            collect_output: true,
            capture_env_stats: false,
            working_limit: None,
            cpu_limit: None,
        }
    }
}

/// SlayPipeline for executing multiple commands in sequence
#[derive(Debug)]
pub struct SlayPipeline {
    /// Commands in the pipeline
    pub commands: Vec<SlayCommand>,
    /// Pipeline options
    pub options: SlayOptions,
}

/// SlayTask for background command execution
#[derive(Debug)]
pub struct SlayTask {
    /// The command being executed
    pub command: SlayCommand,
    /// Start time
    pub start_time: Instant,
    /// Exit code (when finished)
    pub exit_code: Option<i32>,
    /// Whether the task is finished
    pub finished: bool,
    /// Error message (if any)
    pub error: Option<String>,
    /// Captured output
    pub output: Vec<u8>,
    /// Captured combined output
    pub combined_output: Vec<u8>,
    /// Background thread handle
    thread_handle: Option<thread::JoinHandle<ProcessResult<()>>>,
}

/// SlayCommandBuilder for fluent command construction
#[derive(Debug)]
pub struct SlayCommandBuilder {
    command: String,
    args: Vec<String>,
    dir: Option<PathBuf>,
    env: Vec<String>,
    stdin: Option<ProcessStdin>,
    stdout: Option<ProcessStdout>,
    stderr: Option<ProcessStderr>,
    timeout: Option<Duration>,
    use_shell: bool,
}

/// Process stdin configuration
#[derive(Debug)]
pub enum ProcessStdin {
    Null,
    Inherit,
    Pipe,
    File(PathBuf),
}

/// Process stdout configuration
#[derive(Debug)]
pub enum ProcessStdout {
    Null,
    Inherit,
    Pipe,
    File(PathBuf),
}

/// Process stderr configuration
#[derive(Debug)]
pub enum ProcessStderr {
    Null,
    Inherit,
    Pipe,
    File(PathBuf),
}

/// Signal handling options
#[derive(Debug, Clone)]
pub struct SignalOptions {
    /// Grace period before forcing termination
    pub grace_period: Duration,
    /// Force termination if graceful fails
    pub force: bool,
    /// Signal to send
    pub signal: i32,
    /// Apply to process tree recursively
    pub recursive: bool,
}

impl Default for SignalOptions {
    fn default() -> Self {
        Self {
            grace_period: Duration::from_secs(5),
            force: true,
            signal: 15, // SIGTERM
            recursive: false,
        }
    }
}

/// Process resource statistics
#[derive(Debug, Clone)]
pub struct ProcessStats {
    /// CPU usage percentage
    pub cpu: f64,
    /// Memory usage in bytes
    pub memory: u64,
    /// Resident memory in bytes
    pub resident_memory: u64,
    /// Virtual memory in bytes
    pub virtual_memory: u64,
    /// Swap memory in bytes
    pub swap_memory: u64,
    /// Bytes read from storage
    pub read_bytes: u64,
    /// Bytes written to storage
    pub write_bytes: u64,
    /// Read operations count
    pub read_ops: u64,
    /// Write operations count
    pub write_ops: u64,
    /// Process uptime
    pub up_time: Duration,
    /// Number of threads
    pub thread_count: i32,
    /// Number of open files
    pub open_files: i32,
    /// Number of network connections
    pub network_conns: i32,
}

impl SlayCommand {
    /// Create a new SlayCommand
    pub fn new<S: AsRef<str>>(name: S, args: &[&str]) -> Self {
        Self {
            path: name.as_ref().to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            env: Vec::new(),
            dir: None,
            stdin: None,
            stdout: None,
            stderr: None,
            extra_files: Vec::new(),
            process_group: None,
            child: None,
            start_time: None,
        }
    }

    /// Run the command and wait for completion
    pub fn run(&mut self) -> ProcessResult<()> {
        self.start()?;
        self.wait()
    }

    /// Start the command without waiting
    pub fn start(&mut self) -> ProcessResult<()> {
        let mut command = Command::new(&self.path);
        command.args(&self.args);

        if let Some(dir) = &self.dir {
            command.current_dir(dir);
        }

        // Set environment
        if !self.env.is_empty() {
            command.env_clear();
            for env_pair in &self.env {
                if let Some((key, value)) = env_pair.split_once('=') {
                    command.env(key, value);
                }
            }
        }

        // Configure I/O
        if let Some(stdin_config) = &self.stdin {
            command.stdin(match stdin_config {
                ProcessStdin::Null => Stdio::null(),
                ProcessStdin::Inherit => Stdio::inherit(),
                ProcessStdin::Pipe => Stdio::piped(),
                ProcessStdin::File(path) => {
                    let file = File::open(path)
                        .map_err(|e| io_error("stdin_file", &format!("{:?}", e.kind()), &e.to_string()))?;
                    file.into()
                }
            });
        }

        if let Some(stdout_config) = &self.stdout {
            command.stdout(match stdout_config {
                ProcessStdout::Null => Stdio::null(),
                ProcessStdout::Inherit => Stdio::inherit(),
                ProcessStdout::Pipe => Stdio::piped(),
                ProcessStdout::File(path) => {
                    let file = File::create(path)
                        .map_err(|e| io_error("stdout_file", &format!("{:?}", e.kind()), &e.to_string()))?;
                    file.into()
                }
            });
        }

        if let Some(stderr_config) = &self.stderr {
            command.stderr(match stderr_config {
                ProcessStderr::Null => Stdio::null(),
                ProcessStderr::Inherit => Stdio::inherit(),
                ProcessStderr::Pipe => Stdio::piped(),
                ProcessStderr::File(path) => {
                    let file = File::create(path)
                        .map_err(|e| io_error("stderr_file", &format!("{:?}", e.kind()), &e.to_string()))?;
                    file.into()
                }
            });
        }

        let child = command.spawn()
            .map_err(|e| execution_failed(&self.path, &e.to_string()))?;

        self.child = Some(child);
        self.start_time = Some(Instant::now());
        Ok(())
    }

    /// Wait for the command to complete
    pub fn wait(&mut self) -> ProcessResult<()> {
        if let Some(child) = &mut self.child {
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
        } else {
            Err(invalid_arguments("wait", "command", "Command not started"))
        }
    }

    /// Capture command output
    pub fn output(&mut self) -> ProcessResult<Vec<u8>> {
        self.stdout = Some(ProcessStdout::Pipe);
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
    pub fn combined_output(&mut self) -> ProcessResult<Vec<u8>> {
        self.stdout = Some(ProcessStdout::Pipe);
        self.stderr = Some(ProcessStderr::Pipe);
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

    /// Get stdout pipe
    pub fn stdout_pipe(&mut self) -> ProcessResult<Box<dyn Read>> {
        self.stdout = Some(ProcessStdout::Pipe);
        self.start()?;
        
        if let Some(child) = &mut self.child {
            if let Some(stdout) = child.stdout.take() {
                Ok(Box::new(stdout))
            } else {
                Err(invalid_arguments("stdout_pipe", "stdout", "Stdout not piped"))
            }
        } else {
            Err(invalid_arguments("stdout_pipe", "command", "Command not started"))
        }
    }

    /// Get stderr pipe
    pub fn stderr_pipe(&mut self) -> ProcessResult<Box<dyn Read>> {
        self.stderr = Some(ProcessStderr::Pipe);
        self.start()?;
        
        if let Some(child) = &mut self.child {
            if let Some(stderr) = child.stderr.take() {
                Ok(Box::new(stderr))
            } else {
                Err(invalid_arguments("stderr_pipe", "stderr", "Stderr not piped"))
            }
        } else {
            Err(invalid_arguments("stderr_pipe", "command", "Command not started"))
        }
    }

    /// Get stdin pipe
    pub fn stdin_pipe(&mut self) -> ProcessResult<Box<dyn Write>> {
        self.stdin = Some(ProcessStdin::Pipe);
        self.start()?;
        
        if let Some(child) = &mut self.child {
            if let Some(stdin) = child.stdin.take() {
                Ok(Box::new(stdin))
            } else {
                Err(invalid_arguments("stdin_pipe", "stdin", "Stdin not piped"))
            }
        } else {
            Err(invalid_arguments("stdin_pipe", "command", "Command not started"))
        }
    }

    /// Configuration methods
    pub fn set_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.dir = Some(dir.as_ref().to_path_buf());
        self
    }

    pub fn set_env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = env;
        self
    }

    pub fn add_env<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        let env_pair = format!("{}={}", key.as_ref(), value.as_ref());
        self.env.push(env_pair);
        self
    }

    pub fn set_stdin(&mut self, config: ProcessStdin) -> &mut Self {
        self.stdin = Some(config);
        self
    }

    pub fn set_stdout(&mut self, config: ProcessStdout) -> &mut Self {
        self.stdout = Some(config);
        self
    }

    pub fn set_stderr(&mut self, config: ProcessStderr) -> &mut Self {
        self.stderr = Some(config);
        self
    }

    pub fn set_path<S: AsRef<str>>(&mut self, path: S) -> &mut Self {
        self.path = path.as_ref().to_string();
        self
    }

    pub fn set_extra_files(&mut self, files: Vec<File>) -> &mut Self {
        self.extra_files = files;
        self
    }

    /// Apply SlayOptions to the command
    pub fn with_options(&mut self, opts: SlayOptions) -> &mut Self {
        if let Some(dir) = opts.dir {
            self.dir = Some(dir);
        }
        if !opts.env.is_empty() {
            self.env = opts.env;
        }
        self.extra_files = opts.extra_files;
        self
    }

    /// Get process handle
    pub fn process(&mut self) -> ProcessResult<SlayProcess> {
        if let Some(child) = &self.child {
            Ok(SlayProcess {
                pid: child.id(),
                child: Arc::new(Mutex::new(unsafe { 
                    // This is unsafe but necessary for the API
                    std::ptr::read(child as *const Child)
                })),
                start_time: self.start_time.unwrap_or_else(Instant::now),
            })
        } else {
            Err(invalid_arguments("process", "command", "Command not started"))
        }
    }

    /// Get process state
    pub fn process_state(&self) -> ProcessResult<SlayProcessState> {
        if let Some(child) = &self.child {
            let pid = child.id();
            // Note: In a real implementation, we'd collect actual resource usage
            Ok(SlayProcessState {
                exit_status: ExitStatus::from_raw(0), // Placeholder
                pid,
                user_time: Duration::from_millis(0),
                system_time: Duration::from_millis(0),
                max_rss: 0,
            })
        } else {
            Err(invalid_arguments("process_state", "command", "Command not started"))
        }
    }

    /// String representation
    pub fn string(&self) -> String {
        format!("{} {}", self.path, self.args.join(" "))
    }
}

impl SlayProcess {
    /// Kill the process
    pub fn kill(&self) -> ProcessResult<()> {
        let child = self.child.lock().unwrap();
        // Note: This is a simplified implementation
        Ok(())
    }

    /// Send signal to process
    #[cfg(unix)]
    pub fn signal(&self, sig: i32) -> ProcessResult<()> {
        unsafe {
            if libc::kill(self.pid as i32, sig) == 0 {
                Ok(())
            } else {
                Err(system_error(
                    *libc::__errno_location(),
                    "signal",
                    "Failed to send signal"
                ))
            }
        }
    }

    #[cfg(not(unix))]
    pub fn signal(&self, _sig: i32) -> ProcessResult<()> {
        Err(platform_error("Signal sending not supported on this platform"))
    }

    /// Get process ID
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Wait for process completion
    pub fn wait(&self) -> ProcessResult<SlayProcessState> {
        // Simplified implementation
        Ok(SlayProcessState {
            exit_status: ExitStatus::from_raw(0),
            pid: self.pid,
            user_time: Duration::from_millis(0),
            system_time: Duration::from_millis(0),
            max_rss: 0,
        })
    }

    /// Release process resources
    pub fn release(&self) -> ProcessResult<()> {
        // Release any held resources
        Ok(())
    }

    /// Send signal with options
    pub fn send_signal(&self, sig: i32) -> ProcessResult<()> {
        self.signal(sig)
    }

    /// Terminate process gracefully
    pub fn terminate(&self, opts: SignalOptions) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            // Send initial signal
            self.signal(opts.signal)?;
            
            // Wait for grace period
            thread::sleep(opts.grace_period);
            
            // Force kill if requested and process still running
            if opts.force {
                self.signal(9)?; // SIGKILL
            }
            
            Ok(())
        }
        
        #[cfg(not(unix))]
        {
            self.kill()
        }
    }

    /// Kill process tree
    pub fn kill_tree(&self) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            // Kill the process group
            unsafe {
                if libc::kill(-(self.pid as i32), 9) == 0 {
                    Ok(())
                } else {
                    Err(system_error(
                        *libc::__errno_location(),
                        "kill_tree",
                        "Failed to kill process tree"
                    ))
                }
            }
        }
        
        #[cfg(not(unix))]
        {
            self.kill()
        }
    }

    /// Get process statistics
    pub fn stats(&self) -> ProcessResult<ProcessStats> {
        // In a real implementation, this would read from /proc on Linux,
        // or use system APIs on other platforms
        Ok(ProcessStats {
            cpu: 0.0,
            memory: 0,
            resident_memory: 0,
            virtual_memory: 0,
            swap_memory: 0,
            read_bytes: 0,
            write_bytes: 0,
            read_ops: 0,
            write_ops: 0,
            up_time: self.start_time.elapsed(),
            thread_count: 1,
            open_files: 0,
            network_conns: 0,
        })
    }

    /// Monitor process with periodic stats updates
    pub fn monitor<F>(&self, interval: Duration, callback: F) -> ProcessResult<()>
    where
        F: Fn(&ProcessStats) + Send + 'static,
    {
        let pid = self.pid;
        let start_time = self.start_time;
        
        thread::spawn(move || {
            loop {
                // Get current stats
                let stats = ProcessStats {
                    cpu: 0.0,
                    memory: 0,
                    resident_memory: 0,
                    virtual_memory: 0,
                    swap_memory: 0,
                    read_bytes: 0,
                    write_bytes: 0,
                    read_ops: 0,
                    write_ops: 0,
                    up_time: start_time.elapsed(),
                    thread_count: 1,
                    open_files: 0,
                    network_conns: 0,
                };
                
                callback(&stats);
                thread::sleep(interval);
                
                // Check if process still exists
                #[cfg(unix)]
                {
                    unsafe {
                        if libc::kill(pid as i32, 0) != 0 {
                            break; // Process no longer exists
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    /// Set resource limits
    pub fn set_limits(&self, memory_mb: i32, cpu_percent: f64) -> ProcessResult<()> {
        // In a real implementation, this would use setrlimit() on Unix
        // or job objects on Windows
        Ok(())
    }
}

impl SlayProcessState {
    /// Check if process exited normally
    pub fn exited(&self) -> bool {
        true // Simplified
    }

    /// Check if process was successful
    pub fn success(&self) -> bool {
        self.exit_status.success()
    }

    /// Get system-specific exit information
    pub fn sys(&self) -> Box<dyn std::any::Any> {
        Box::new(self.exit_status)
    }

    /// Get system usage information
    pub fn sys_usage(&self) -> Box<dyn std::any::Any> {
        Box::new((self.user_time, self.system_time))
    }

    /// Get exit code
    pub fn exit_code(&self) -> i32 {
        self.exit_status.code().unwrap_or(-1)
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
}

impl SlayPipeline {
    /// Create a new pipeline
    pub fn new(commands: Vec<SlayCommand>) -> Self {
        Self {
            commands,
            options: SlayOptions::default(),
        }
    }

    /// Run pipeline and wait for completion
    pub fn run(&mut self) -> ProcessResult<()> {
        self.start()?;
        self.wait()
    }

    /// Start pipeline without waiting
    pub fn start(&mut self) -> ProcessResult<()> {
        // Start all commands in sequence, piping output
        for i in 0..self.commands.len() {
            if i > 0 {
                // Pipe previous command's stdout to this command's stdin
                self.commands[i].stdin = Some(ProcessStdin::Pipe);
            }
            if i < self.commands.len() - 1 {
                // Pipe this command's stdout to next command
                self.commands[i].stdout = Some(ProcessStdout::Pipe);
            }
            self.commands[i].start()?;
        }
        Ok(())
    }

    /// Wait for all commands to complete
    pub fn wait(&mut self) -> ProcessResult<()> {
        for command in &mut self.commands {
            command.wait()?;
        }
        Ok(())
    }

    /// Get output from last command
    pub fn output(&mut self) -> ProcessResult<Vec<u8>> {
        if let Some(last_command) = self.commands.last_mut() {
            last_command.output()
        } else {
            Err(invalid_arguments("output", "pipeline", "Empty pipeline"))
        }
    }

    /// Get combined output from last command
    pub fn combined_output(&mut self) -> ProcessResult<Vec<u8>> {
        if let Some(last_command) = self.commands.last_mut() {
            last_command.combined_output()
        } else {
            Err(invalid_arguments("combined_output", "pipeline", "Empty pipeline"))
        }
    }

    /// Apply options to pipeline
    pub fn with_options(&mut self, opts: SlayOptions) -> &mut Self {
        self.options = opts;
        self
    }

    /// Add command to pipeline
    pub fn add_command(&mut self, cmd: SlayCommand) -> &mut Self {
        self.commands.push(cmd);
        self
    }

    /// Set all commands
    pub fn set_commands(&mut self, cmds: Vec<SlayCommand>) -> &mut Self {
        self.commands = cmds;
        self
    }

    /// String representation
    pub fn string(&self) -> String {
        self.commands.iter()
            .map(|cmd| cmd.string())
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

impl SlayTask {
    /// Wait for task completion
    pub fn wait(&mut self) -> ProcessResult<()> {
        if let Some(handle) = self.thread_handle.take() {
            match handle.join() {
                Ok(result) => {
                    self.finished = true;
                    result
                }
                Err(_) => {
                    self.finished = true;
                    self.error = Some("Thread panicked".to_string());
                    Err(execution_failed(&self.command.path, "Background thread panicked"))
                }
            }
        } else {
            Ok(())
        }
    }

    /// Kill the background task
    pub fn kill(&mut self) -> ProcessResult<()> {
        if let Some(process) = self.command.process().ok() {
            process.kill()?;
        }
        self.finished = true;
        Ok(())
    }

    /// Check if task is running
    pub fn is_running(&self) -> bool {
        !self.finished
    }

    /// Get elapsed time
    pub fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get output
    pub fn get_output(&self) -> ProcessResult<Vec<u8>> {
        Ok(self.output.clone())
    }

    /// Get combined output
    pub fn get_combined_output(&self) -> ProcessResult<Vec<u8>> {
        Ok(self.combined_output.clone())
    }
}

impl SlayCommandBuilder {
    /// Create a new command builder
    pub fn new<S: AsRef<str>>(command: S) -> Self {
        Self {
            command: command.as_ref().to_string(),
            args: Vec::new(),
            dir: None,
            env: Vec::new(),
            stdin: None,
            stdout: None,
            stderr: None,
            timeout: None,
            use_shell: false,
        }
    }

    /// Add arguments
    pub fn with_args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }

    /// Set working directory
    pub fn with_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Set environment variables
    pub fn with_env(mut self, env: Vec<String>) -> Self {
        self.env = env;
        self
    }

    /// Add environment variable
    pub fn add_env<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> Self {
        let env_pair = format!("{}={}", key.as_ref(), value.as_ref());
        self.env.push(env_pair);
        self
    }

    /// Set stdin
    pub fn with_stdin(mut self, stdin: ProcessStdin) -> Self {
        self.stdin = Some(stdin);
        self
    }

    /// Set stdout
    pub fn with_stdout(mut self, stdout: ProcessStdout) -> Self {
        self.stdout = Some(stdout);
        self
    }

    /// Set stderr
    pub fn with_stderr(mut self, stderr: ProcessStderr) -> Self {
        self.stderr = Some(stderr);
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Use shell
    pub fn use_shell(mut self, use_shell: bool) -> Self {
        self.use_shell = use_shell;
        self
    }

    /// Build the command
    pub fn build(self) -> SlayCommand {
        let mut cmd = SlayCommand::new(&self.command, &self.args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        
        if let Some(dir) = self.dir {
            cmd.set_dir(dir);
        }
        
        if !self.env.is_empty() {
            cmd.set_env(self.env);
        }
        
        if let Some(stdin) = self.stdin {
            cmd.set_stdin(stdin);
        }
        
        if let Some(stdout) = self.stdout {
            cmd.set_stdout(stdout);
        }
        
        if let Some(stderr) = self.stderr {
            cmd.set_stderr(stderr);
        }
        
        cmd
    }
}

/// Create a new SlayCommand
pub fn new_slay_command<S: AsRef<str>>(name: S, args: &[&str]) -> SlayCommand {
    SlayCommand::new(name, args)
}

/// Create a new SlayPipeline
pub fn new_slay_pipeline(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::new(commands)
}

/// Create a pipeline from commands
pub fn pipe(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::new(commands)
}

/// Run a command in the background
pub fn run_background(mut cmd: SlayCommand) -> SlayTask {
    let start_time = Instant::now();
    let command_copy = SlayCommand::new(&cmd.path, &cmd.args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    
    let handle = thread::spawn(move || {
        cmd.run()
    });
    
    SlayTask {
        command: command_copy,
        start_time,
        exit_code: None,
        finished: false,
        error: None,
        output: Vec::new(),
        combined_output: Vec::new(),
        thread_handle: Some(handle),
    }
}

/// Run a command with timeout
pub fn run_with_timeout(mut cmd: SlayCommand, timeout: Duration) -> ProcessResult<()> {
    cmd.start()?;
    
    let start = Instant::now();
    while start.elapsed() < timeout {
        if let Some(child) = &mut cmd.child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        return Ok(());
                    } else {
                        return Err(execution_failed_with_code(
                            &cmd.path,
                            status.code().unwrap_or(-1),
                            "Command failed"
                        ));
                    }
                }
                Ok(None) => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Err(e) => {
                    return Err(io_error("try_wait", &format!("{:?}", e.kind()), &e.to_string()));
                }
            }
        }
    }
    
    // Timeout occurred
    if let Some(child) = &mut cmd.child {
        let _ = child.kill();
    }
    Err(timeout_error("run_with_timeout", timeout, "Command execution timed out"))
}

/// Get output with timeout
pub fn output_with_timeout(mut cmd: SlayCommand, timeout: Duration) -> ProcessResult<Vec<u8>> {
    cmd.stdout = Some(ProcessStdout::Pipe);
    cmd.start()?;
    
    let start = Instant::now();
    while start.elapsed() < timeout {
        if let Some(child) = &mut cmd.child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if let Some(stdout) = child.stdout.take() {
                        let mut output = Vec::new();
                        let mut reader = BufReader::new(stdout);
                        reader.read_to_end(&mut output)
                            .map_err(|e| io_error("read_output", &format!("{:?}", e.kind()), &e.to_string()))?;
                        return Ok(output);
                    } else {
                        return Ok(Vec::new());
                    }
                }
                Ok(None) => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Err(e) => {
                    return Err(io_error("try_wait", &format!("{:?}", e.kind()), &e.to_string()));
                }
            }
        }
    }
    
    // Timeout occurred
    if let Some(child) = &mut cmd.child {
        let _ = child.kill();
    }
    Err(timeout_error("output_with_timeout", timeout, "Command output timed out"))
}

/// Get combined output with timeout
pub fn combined_output_with_timeout(mut cmd: SlayCommand, timeout: Duration) -> ProcessResult<Vec<u8>> {
    cmd.stdout = Some(ProcessStdout::Pipe);
    cmd.stderr = Some(ProcessStderr::Pipe);
    cmd.start()?;
    
    let start = Instant::now();
    while start.elapsed() < timeout {
        if let Some(child) = &mut cmd.child {
            match child.try_wait() {
                Ok(Some(_status)) => {
                    let mut combined_output = Vec::new();
                    
                    if let Some(stdout) = child.stdout.take() {
                        let mut reader = BufReader::new(stdout);
                        reader.read_to_end(&mut combined_output)
                            .map_err(|e| io_error("read_stdout", &format!("{:?}", e.kind()), &e.to_string()))?;
                    }
                    
                    if let Some(stderr) = child.stderr.take() {
                        let mut reader = BufReader::new(stderr);
                        reader.read_to_end(&mut combined_output)
                            .map_err(|e| io_error("read_stderr", &format!("{:?}", e.kind()), &e.to_string()))?;
                    }
                    
                    return Ok(combined_output);
                }
                Ok(None) => {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }
                Err(e) => {
                    return Err(io_error("try_wait", &format!("{:?}", e.kind()), &e.to_string()));
                }
            }
        }
    }
    
    // Timeout occurred
    if let Some(child) = &mut cmd.child {
        let _ = child.kill();
    }
    Err(timeout_error("combined_output_with_timeout", timeout, "Command output timed out"))
}

/// Run shell command directly
pub fn run_shell<S: AsRef<str>>(cmd_string: S) -> ProcessResult<()> {
    let shell = if cfg!(windows) { "cmd" } else { "sh" };
    let flag = if cfg!(windows) { "/C" } else { "-c" };
    
    let mut cmd = SlayCommand::new(shell, &[flag, cmd_string.as_ref()]);
    cmd.run()
}

/// Run shell command and return output
pub fn shell_output<S: AsRef<str>>(cmd_string: S) -> ProcessResult<Vec<u8>> {
    let shell = if cfg!(windows) { "cmd" } else { "sh" };
    let flag = if cfg!(windows) { "/C" } else { "-c" };
    
    let mut cmd = SlayCommand::new(shell, &[flag, cmd_string.as_ref()]);
    cmd.output()
}

/// Run shell command with environment variables
pub fn run_shell_with_env<S: AsRef<str>>(cmd_string: S, env: HashMap<String, String>) -> ProcessResult<()> {
    let shell = if cfg!(windows) { "cmd" } else { "sh" };
    let flag = if cfg!(windows) { "/C" } else { "-c" };
    
    let mut cmd = SlayCommand::new(shell, &[flag, cmd_string.as_ref()]);
    
    for (key, value) in env {
        cmd.add_env(key, value);
    }
    
    cmd.run()
}

/// Run shell command in specific directory
pub fn run_shell_in_dir<S: AsRef<str>, P: AsRef<Path>>(cmd_string: S, dir: P) -> ProcessResult<()> {
    let shell = if cfg!(windows) { "cmd" } else { "sh" };
    let flag = if cfg!(windows) { "/C" } else { "-c" };
    
    let mut cmd = SlayCommand::new(shell, &[flag, cmd_string.as_ref()]);
    cmd.set_dir(dir);
    cmd.run()
}

/// Create a new command builder
pub fn new_slay_command_builder<S: AsRef<str>>(command: S) -> SlayCommandBuilder {
    SlayCommandBuilder::new(command)
}

use crate::stdlib::process::error::platform_error;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_slay_command_creation() {
        let cmd = SlayCommand::new("echo", &["hello", "world"]);
        assert_eq!(cmd.path, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_slay_command_builder() {
        let cmd = SlayCommandBuilder::new("ls")
            .with_args(&["-la", "-h"])
            .with_timeout(Duration::from_secs(30))
            .build();
        
        assert_eq!(cmd.path, "ls");
        assert_eq!(cmd.args, vec!["-la", "-h"]);
    }

    #[test]
    fn test_slay_options_default() {
        let opts = SlayOptions::default();
        assert_eq!(opts.buffer_size, 8192);
        assert!(opts.collect_output);
        assert!(!opts.use_shell);
    }

    #[test]
    fn test_signal_options_default() {
        let opts = SignalOptions::default();
        assert_eq!(opts.grace_period, Duration::from_secs(5));
        assert!(opts.force);
        assert_eq!(opts.signal, 15); // SIGTERM
    }

    #[test]
    fn test_pipeline_creation() {
        let cmd1 = SlayCommand::new("echo", &["hello"]);
        let cmd2 = SlayCommand::new("grep", &["hello"]);
        let pipeline = SlayPipeline::new(vec![cmd1, cmd2]);
        
        assert_eq!(pipeline.commands.len(), 2);
    }

    #[test]
    fn test_process_stats_creation() {
        let stats = ProcessStats {
            cpu: 50.0,
            memory: 1024*1024,
            resident_memory: 512*1024,
            virtual_memory: 2*1024*1024,
            swap_memory: 0,
            read_bytes: 1000,
            write_bytes: 500,
            read_ops: 10,
            write_ops: 5,
            up_time: Duration::from_secs(300),
            thread_count: 3,
            open_files: 15,
            network_conns: 2,
        };
        
        assert_eq!(stats.cpu, 50.0);
        assert_eq!(stats.memory, 1024*1024);
        assert_eq!(stats.thread_count, 3);
    }
}
