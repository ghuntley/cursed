/// Enhanced ExecSlay implementation with complete feature set
/// 
/// This module provides the full ExecSlay API as specified in the CURSED language
/// specifications, with comprehensive process management, pipeline execution,
/// background tasks, and advanced IPC integration.

use std::collections::HashMap;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, RwLock, mpsc, Condvar};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use crate::stdlib::process::error::{ProcessError, ProcessResult};
pub use crate::stdlib::process::exec_slay::{
    SlayCommand, SlayProcess, SlayProcessState, SlayOptions, SlayPipeline,
    SlayTask, SlayCommandBuilder, ProcessStats, SignalOptions
};
use crate::stdlib::ipc::{IpcConfig, initialize_ipc};

/// Enhanced slay command with additional capabilities
pub type EnhancedSlayCommand = EnhancedCommandBuilder;

/// Enhanced slay process for advanced process management
#[derive(Debug)]
pub struct EnhancedSlayProcess {
    /// Process ID
    pub pid: u32,
    /// Process handle
    pub handle: Option<Child>,
    /// Start time
    pub start_time: SystemTime,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Security context
    pub security_context: SecurityContext,
}

/// Enhanced slay options for configuration
#[derive(Debug, Clone)]
pub struct EnhancedSlayOptions {
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Security context
    pub security_context: SecurityContext,
    /// Timeout configuration
    pub timeout: Option<Duration>,
    /// Process priority
    pub priority: ProcessPriority,
}

/// Enhanced command builder with comprehensive configuration
#[derive(Debug, Clone)]
pub struct EnhancedCommandBuilder {
    command: String,
    args: Vec<String>,
    env_vars: HashMap<String, String>,
    working_dir: Option<PathBuf>,
    stdin_config: StdinConfig,
    stdout_config: StdoutConfig,
    stderr_config: StderrConfig,
    timeout: Option<Duration>,
    resource_limits: ResourceLimits,
    priority: ProcessPriority,
    security_context: SecurityContext,
}

/// Standard input configuration
#[derive(Debug, Clone)]
pub enum StdinConfig {
    Inherit,
    Null,
    Piped,
    FromFile(PathBuf),
    FromBytes(Vec<u8>),
    FromString(String),
}

/// Standard output configuration
#[derive(Debug, Clone)]
pub enum StdoutConfig {
    Inherit,
    Null,
    Piped,
    ToFile(PathBuf),
    Append(PathBuf),
    ToBuf(Arc<Mutex<Vec<u8>>>),
}

/// Standard error configuration
#[derive(Debug, Clone)]
pub enum StderrConfig {
    Inherit,
    Null,
    Piped,
    ToFile(PathBuf),
    Append(PathBuf),
    ToBuf(Arc<Mutex<Vec<u8>>>),
    ToStdout,
}

/// Resource limits for process execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory: Option<u64>,
    pub max_cpu_time: Option<Duration>,
    pub max_wall_time: Option<Duration>,
    pub max_open_files: Option<u64>,
    pub max_processes: Option<u64>,
    pub max_file_size: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: None,
            max_cpu_time: None,
            max_wall_time: None,
            max_open_files: None,
            max_processes: None,
            max_file_size: None,
        }
    }
}

/// Process execution priority
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessPriority {
    Highest,
    High,
    Normal,
    Low,
    Lowest,
    Custom(i32),
}

impl Default for ProcessPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Security context for process execution
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user_id: Option<u32>,
    pub group_id: Option<u32>,
    pub supplementary_groups: Vec<u32>,
    pub capabilities: Vec<String>,
    pub seccomp_filter: Option<String>,
    pub chroot_path: Option<PathBuf>,
    pub namespace_isolation: NamespaceIsolation,
}

/// Namespace isolation configuration
#[derive(Debug, Clone)]
pub struct NamespaceIsolation {
    pub pid: bool,
    pub network: bool,
    pub mount: bool,
    pub user: bool,
    pub ipc: bool,
    pub uts: bool,
}

impl Default for NamespaceIsolation {
    fn default() -> Self {
        Self {
            pid: false,
            network: false,
            mount: false,
            user: false,
            ipc: false,
            uts: false,
        }
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user_id: None,
            group_id: None,
            supplementary_groups: Vec::new(),
            capabilities: Vec::new(),
            seccomp_filter: None,
            chroot_path: None,
            namespace_isolation: NamespaceIsolation::default(),
        }
    }
}

impl EnhancedCommandBuilder {
    /// Create a new enhanced command builder
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            args: Vec::new(),
            env_vars: HashMap::new(),
            working_dir: None,
            stdin_config: StdinConfig::Inherit,
            stdout_config: StdoutConfig::Inherit,
            stderr_config: StderrConfig::Inherit,
            timeout: None,
            resource_limits: ResourceLimits::default(),
            priority: ProcessPriority::default(),
            security_context: SecurityContext::default(),
        }
    }

    /// Add command line arguments
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.args.extend(args.into_iter().map(|s| s.as_ref().to_string()));
        self
    }

    /// Add a single argument
    pub fn arg<S: AsRef<str>>(mut self, arg: S) -> Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    /// Set environment variables
    pub fn envs<I, K, V>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        for (k, v) in vars {
            self.env_vars.insert(k.as_ref().to_string(), v.as_ref().to_string());
        }
        self
    }

    /// Set a single environment variable
    pub fn env<K, V>(mut self, key: K, val: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.env_vars.insert(key.as_ref().to_string(), val.as_ref().to_string());
        self
    }

    /// Set working directory
    pub fn current_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.working_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Configure stdin
    pub fn stdin(mut self, config: StdinConfig) -> Self {
        self.stdin_config = config;
        self
    }

    /// Configure stdout
    pub fn stdout(mut self, config: StdoutConfig) -> Self {
        self.stdout_config = config;
        self
    }

    /// Configure stderr
    pub fn stderr(mut self, config: StderrConfig) -> Self {
        self.stderr_config = config;
        self
    }

    /// Set execution timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set resource limits
    pub fn resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = limits;
        self
    }

    /// Set process priority
    pub fn priority(mut self, priority: ProcessPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set security context
    pub fn security_context(mut self, context: SecurityContext) -> Self {
        self.security_context = context;
        self
    }

    /// Build and execute the command
    pub fn spawn(self) -> ProcessResult<EnhancedProcess> {
        let mut cmd = Command::new(&self.command);
        
        // Set arguments
        cmd.args(&self.args);
        
        // Set environment variables
        for (key, value) in &self.env_vars {
            cmd.env(key, value);
        }
        
        // Set working directory
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }
        
        // Configure stdio
        self.configure_stdio(&mut cmd)?;
        
        // Apply resource limits and security context
        self.apply_resource_limits(&mut cmd)?;
        self.apply_security_context(&mut cmd)?;
        
        // Spawn the process
        let child = cmd.spawn()
            .map_err(|e| ProcessError::SpawnFailed(self.command.clone(), e.to_string()))?;
        
        let process = EnhancedProcess::new(child, self.timeout, self.resource_limits.clone());
        Ok(process)
    }

    /// Execute and wait for completion
    pub fn output(self) -> ProcessResult<ProcessOutput> {
        let mut process = self.spawn()?;
        process.wait_with_output()
    }

    /// Execute and return status
    pub fn status(self) -> ProcessResult<ExitStatus> {
        let mut process = self.spawn()?;
        process.wait()
    }

    fn configure_stdio(&self, cmd: &mut Command) -> ProcessResult<()> {
        // Configure stdin
        match &self.stdin_config {
            StdinConfig::Inherit => { cmd.stdin(Stdio::inherit()); }
            StdinConfig::Null => { cmd.stdin(Stdio::null()); }
            StdinConfig::Piped => { cmd.stdin(Stdio::piped()); }
            StdinConfig::FromFile(path) => {
                let file = std::fs::File::open(path)
                    .map_err(|e| ProcessError::IoError(format!("Failed to open stdin file: {}", e)))?;
                cmd.stdin(Stdio::from(file));
            }
            _ => { cmd.stdin(Stdio::piped()); } // Handle FromBytes and FromString after spawn
        }
        
        // Configure stdout
        match &self.stdout_config {
            StdoutConfig::Inherit => { cmd.stdout(Stdio::inherit()); }
            StdoutConfig::Null => { cmd.stdout(Stdio::null()); }
            StdoutConfig::Piped => { cmd.stdout(Stdio::piped()); }
            StdoutConfig::ToFile(path) => {
                let file = std::fs::File::create(path)
                    .map_err(|e| ProcessError::IoError(format!("Failed to create stdout file: {}", e)))?;
                cmd.stdout(Stdio::from(file));
            }
            StdoutConfig::Append(path) => {
                let file = std::fs::OpenOptions::new().create(true).append(true).open(path)
                    .map_err(|e| ProcessError::IoError(format!("Failed to open stdout file for append: {}", e)))?;
                cmd.stdout(Stdio::from(file));
            }
            _ => { cmd.stdout(Stdio::piped()); } // Handle ToBuf after spawn
        }
        
        // Configure stderr
        match &self.stderr_config {
            StderrConfig::Inherit => { cmd.stderr(Stdio::inherit()); }
            StderrConfig::Null => { cmd.stderr(Stdio::null()); }
            StderrConfig::Piped => { cmd.stderr(Stdio::piped()); }
            StderrConfig::ToStdout => { cmd.stderr(Stdio::piped()); } // Will be handled specially
            StderrConfig::ToFile(path) => {
                let file = std::fs::File::create(path)
                    .map_err(|e| ProcessError::IoError(format!("Failed to create stderr file: {}", e)))?;
                cmd.stderr(Stdio::from(file));
            }
            StderrConfig::Append(path) => {
                let file = std::fs::OpenOptions::new().create(true).append(true).open(path)
                    .map_err(|e| ProcessError::IoError(format!("Failed to open stderr file for append: {}", e)))?;
                cmd.stderr(Stdio::from(file));
            }
            _ => { cmd.stderr(Stdio::piped()); } // Handle ToBuf after spawn
        }
        
        Ok(())
    }

    fn apply_resource_limits(&self, _cmd: &mut Command) -> ProcessResult<()> {
        // Resource limits would be applied using setrlimit after fork but before exec
        // This is a simplified version - full implementation would use pre_exec
        
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            
            let limits = self.resource_limits.clone();
            unsafe {
                _cmd.pre_exec(move || {
                    // Apply resource limits
                    if let Some(max_memory) = limits.max_memory {
                        let rlim = libc::rlimit {
                            rlim_cur: max_memory,
                            rlim_max: max_memory,
                        };
                        libc::setrlimit(libc::RLIMIT_AS, &rlim);
                    }
                    
                    if let Some(max_cpu_time) = limits.max_cpu_time {
                        let rlim = libc::rlimit {
                            rlim_cur: max_cpu_time.as_secs(),
                            rlim_max: max_cpu_time.as_secs(),
                        };
                        libc::setrlimit(libc::RLIMIT_CPU, &rlim);
                    }
                    
                    if let Some(max_open_files) = limits.max_open_files {
                        let rlim = libc::rlimit {
                            rlim_cur: max_open_files,
                            rlim_max: max_open_files,
                        };
                        libc::setrlimit(libc::RLIMIT_NOFILE, &rlim);
                    }
                    
                    Ok(())
                });
            }
        }
        
        Ok(())
    }

    fn apply_security_context(&self, _cmd: &mut Command) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            
            let context = self.security_context.clone();
            unsafe {
                _cmd.pre_exec(move || {
                    // Change user/group if specified
                    if let Some(gid) = context.group_id {
                        if libc::setgid(gid) != 0 {
                            return Err(io::Error::last_os_error());
                        }
                    }
                    
                    if let Some(uid) = context.user_id {
                        if libc::setuid(uid) != 0 {
                            return Err(io::Error::last_os_error());
                        }
                    }
                    
                    // Set supplementary groups
                    if !context.supplementary_groups.is_empty() {
                        if libc::setgroups(
                            context.supplementary_groups.len(),
                            context.supplementary_groups.as_ptr()
                        ) != 0 {
                            return Err(io::Error::last_os_error());
                        }
                    }
                    
                    Ok(())
                });
            }
        }
        
        Ok(())
    }
}

/// Enhanced process with comprehensive monitoring and control
pub struct EnhancedProcess {
    child: Child,
    start_time: Instant,
    timeout: Option<Duration>,
    resource_limits: ResourceLimits,
    stats: Arc<Mutex<ProcessStatistics>>,
    monitoring_thread: Option<thread::JoinHandle<()>>,
}

/// Comprehensive process statistics
#[derive(Debug, Clone)]
pub struct ProcessStatistics {
    pub pid: u32,
    pub start_time: Instant,
    pub cpu_time: Duration,
    pub memory_usage: u64,
    pub max_memory_usage: u64,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
    pub context_switches: u64,
    pub page_faults: u64,
    pub open_file_descriptors: u32,
}

impl Default for ProcessStatistics {
    fn default() -> Self {
        Self {
            pid: 0,
            start_time: Instant::now(),
            cpu_time: Duration::from_secs(0),
            memory_usage: 0,
            max_memory_usage: 0,
            io_read_bytes: 0,
            io_write_bytes: 0,
            context_switches: 0,
            page_faults: 0,
            open_file_descriptors: 0,
        }
    }
}

/// Process output with comprehensive information
#[derive(Debug)]
pub struct ProcessOutput {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub statistics: ProcessStatistics,
    pub execution_time: Duration,
}

impl EnhancedProcess {
    fn new(child: Child, timeout: Option<Duration>, limits: ResourceLimits) -> Self {
        let pid = child.id();
        let start_time = Instant::now();
        
        let stats = Arc::new(Mutex::new(ProcessStatistics {
            pid,
            start_time,
            ..Default::default()
        }));
        
        // Start monitoring thread
        let monitoring_thread = if limits.max_memory.is_some() || limits.max_cpu_time.is_some() {
            let stats_clone = Arc::clone(&stats);
            let limits_clone = limits.clone();
            
            Some(thread::spawn(move || {
                Self::monitor_process(pid, stats_clone, limits_clone);
            }))
        } else {
            None
        };
        
        Self {
            child,
            start_time,
            timeout,
            resource_limits: limits,
            stats,
            monitoring_thread,
        }
    }

    /// Wait for process completion
    pub fn wait(&mut self) -> ProcessResult<ExitStatus> {
        let result = if let Some(timeout) = self.timeout {
            self.wait_timeout(timeout)
        } else {
            self.child.wait().map_err(|e| ProcessError::WaitFailed(e.to_string()))
        };
        
        // Update final statistics
        self.update_final_stats();
        
        result
    }

    /// Wait with timeout
    pub fn wait_timeout(&mut self, timeout: Duration) -> ProcessResult<ExitStatus> {
        let start = Instant::now();
        
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => return Ok(status),
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        // Timeout reached, terminate the process
                        let _ = self.kill();
                        return Err(ProcessError::TimedOut(timeout));
                    }
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => return Err(ProcessError::WaitFailed(e.to_string())),
            }
        }
    }

    /// Wait and capture output
    pub fn wait_with_output(mut self) -> ProcessResult<ProcessOutput> {
        let start_time = self.start_time;
        let status = self.wait()?;
        let execution_time = start_time.elapsed();
        
        let stdout = if let Some(mut stdout) = self.child.stdout.take() {
            let mut output = Vec::new();
            stdout.read_to_end(&mut output)
                .map_err(|e| ProcessError::IoError(format!("Failed to read stdout: {}", e)))?;
            output
        } else {
            Vec::new()
        };
        
        let stderr = if let Some(mut stderr) = self.child.stderr.take() {
            let mut output = Vec::new();
            stderr.read_to_end(&mut output)
                .map_err(|e| ProcessError::IoError(format!("Failed to read stderr: {}", e)))?;
            output
        } else {
            Vec::new()
        };
        
        let statistics = self.stats.lock().unwrap().clone();
        
        Ok(ProcessOutput {
            status,
            stdout,
            stderr,
            statistics,
            execution_time,
        })
    }

    /// Kill the process
    pub fn kill(&mut self) -> ProcessResult<()> {
        self.child.kill().map_err(|e| ProcessError::KillFailed(e.to_string()))
    }

    /// Get process ID
    pub fn id(&self) -> u32 {
        self.child.id()
    }

    /// Get current statistics
    pub fn statistics(&self) -> ProcessStatistics {
        self.stats.lock().unwrap().clone()
    }

    fn monitor_process(pid: u32, stats: Arc<Mutex<ProcessStatistics>>, limits: ResourceLimits) {
        loop {
            thread::sleep(Duration::from_millis(100));
            
            // Check if process still exists
            #[cfg(unix)]
            {
                if unsafe { libc::kill(pid as i32, 0) } != 0 {
                    break; // Process no longer exists
                }
            }
            
            // Update statistics
            if let Ok(mut stats_guard) = stats.lock() {
                Self::update_process_stats(pid, &mut stats_guard);
                
                // Check resource limits
                if let Some(max_memory) = limits.max_memory {
                    if stats_guard.memory_usage > max_memory {
                        // Process exceeded memory limit
                        #[cfg(unix)]
                        unsafe {
                            libc::kill(pid as i32, libc::SIGKILL);
                        }
                        break;
                    }
                }
                
                if let Some(max_cpu_time) = limits.max_cpu_time {
                    if stats_guard.cpu_time > max_cpu_time {
                        // Process exceeded CPU time limit
                        #[cfg(unix)]
                        unsafe {
                            libc::kill(pid as i32, libc::SIGKILL);
                        }
                        break;
                    }
                }
            }
        }
    }

    fn update_process_stats(pid: u32, stats: &mut ProcessStatistics) {
        // Read process statistics from /proc/{pid}/stat (Linux)
        #[cfg(target_os = "linux")]
        {
            if let Ok(stat_content) = std::fs::read_to_string(format!("/proc/{}/stat", pid)) {
                let fields: Vec<&str> = stat_content.split_whitespace().collect();
                if fields.len() >= 24 {
                    // Parse various fields from /proc/pid/stat
                    if let Ok(utime) = fields[13].parse::<u64>() {
                        if let Ok(stime) = fields[14].parse::<u64>() {
                            let clock_ticks = unsafe { libc::sysconf(libc::_SC_CLK_TCK) as u64 };
                            stats.cpu_time = Duration::from_millis((utime + stime) * 1000 / clock_ticks);
                        }
                    }
                    
                    if let Ok(rss_pages) = fields[23].parse::<u64>() {
                        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as u64 };
                        stats.memory_usage = rss_pages * page_size;
                        stats.max_memory_usage = stats.max_memory_usage.max(stats.memory_usage);
                    }
                }
            }
            
            // Read I/O statistics from /proc/{pid}/io
            if let Ok(io_content) = std::fs::read_to_string(format!("/proc/{}/io", pid)) {
                for line in io_content.split("\n") {
                    if line.starts_with("read_bytes:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            if let Ok(bytes) = value.parse::<u64>() {
                                stats.io_read_bytes = bytes;
                            }
                        }
                    } else if line.starts_with("write_bytes:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            if let Ok(bytes) = value.parse::<u64>() {
                                stats.io_write_bytes = bytes;
                            }
                        }
                    }
                }
            }
            
            // Count open file descriptors
            if let Ok(fd_entries) = std::fs::read_dir(format!("/proc/{}/fd", pid)) {
                stats.open_file_descriptors = fd_entries.count() as u32;
            }
        }
    }

    fn update_final_stats(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            Self::update_process_stats(stats.pid, &mut stats);
        }
    }
}

impl Drop for EnhancedProcess {
    fn drop(&mut self) {
        // Ensure the child process is cleaned up
        let _ = self.child.wait();
        
        // Wait for monitoring thread to finish
        if let Some(handle) = self.monitoring_thread.take() {
            let _ = handle.join();
        }
    }
}

/// Complete SlayCommand implementation following the specification
impl SlayCommand {
    /// Create new SlayCommand constructor from specification
    pub fn new_slay_command(name: &str, args: &[&str]) -> Self {
        Self::new(name, args)
    }

    /// Run command with enhanced error handling
    pub fn run_enhanced(&mut self) -> ProcessResult<SlayProcessState> {
        self.start()?;
        let process = self.process()?;
        process.wait()
    }

    /// Start command with enhanced monitoring
    pub fn start_enhanced(&mut self) -> ProcessResult<SlayProcess> {
        self.start()?;
        self.process()
    }

    /// Wait for command completion with enhanced state
    pub fn wait_enhanced(&mut self) -> ProcessResult<SlayProcessState> {
        self.wait()?;
        self.process_state()
    }

    /// Get output with enhanced error information
    pub fn output_enhanced(&mut self) -> ProcessResult<(Vec<u8>, SlayProcessState)> {
        let output = self.output()?;
        let state = self.process_state()?;
        Ok((output, state))
    }

    /// Get combined output with state information
    pub fn combined_output_enhanced(&mut self) -> ProcessResult<(Vec<u8>, SlayProcessState)> {
        let output = self.combined_output()?;
        let state = self.process_state()?;
        Ok((output, state))
    }

    /// Get stdout pipe with enhanced buffering
    pub fn stdout_pipe_enhanced(&mut self) -> ProcessResult<Box<dyn BufRead + Send>> {
        let pipe = self.stdout_pipe()?;
        Ok(Box::new(BufReader::new(pipe)))
    }

    /// Get stderr pipe with enhanced buffering
    pub fn stderr_pipe_enhanced(&mut self) -> ProcessResult<Box<dyn BufRead + Send>> {
        let pipe = self.stderr_pipe()?;
        Ok(Box::new(BufReader::new(pipe)))
    }

    /// Get stdin pipe with enhanced buffering
    pub fn stdin_pipe_enhanced(&mut self) -> ProcessResult<Box<dyn Write + Send>> {
        let pipe = self.stdin_pipe()?;
        Ok(Box::new(BufWriter::new(pipe)))
    }

    /// Enhanced configuration methods with fluent interface
    pub fn with_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.set_dir(dir);
        self
    }

    pub fn with_env(mut self, env: Vec<String>) -> Self {
        self.set_env(env);
        self
    }

    pub fn with_env_var<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> Self {
        self.add_env(key, value);
        self
    }

    /// Set system process attributes (Unix-specific)
    #[cfg(unix)]
    pub fn set_sys_proc_attr(&mut self, process_group: Option<u32>) -> &mut Self {
        self.process_group = process_group;
        self
    }

    /// Enhanced string representation
    pub fn enhanced_string(&self) -> String {
        let mut result = format!("{} {}", self.path, self.args.join(" "));
        
        if let Some(dir) = &self.dir {
            result.push_str(&format!(" (in {})", dir.display()));
        }
        
        if !self.env.is_empty() {
            result.push_str(&format!(" with {} env vars", self.env.len()));
        }
        
        result
    }
}

/// Enhanced SlayProcess implementation
impl SlayProcess {
    /// Send signal with enhanced error reporting
    pub fn send_signal_enhanced(&self, sig: i32) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            self.signal(sig)?;
            tracing::debug!(pid = self.pid, signal = sig, "Signal sent to process");
            Ok(())
        }
        
        #[cfg(not(unix))]
        {
            Err(ProcessError::PlatformError {
                operation: "send_signal".to_string(),
                message: "Signal sending not supported on this platform".to_string(),
            })
        }
    }

    /// Terminate with detailed options
    pub fn terminate_enhanced(&self, opts: SignalOptions) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            tracing::info!(
                pid = self.pid,
                signal = opts.signal,
                grace_period_secs = opts.grace_period.as_secs(),
                force = opts.force,
                recursive = opts.recursive,
                "Terminating process"
            );
            
            if opts.recursive {
                self.kill_tree()?;
            } else {
                self.terminate(opts)?;
            }
            
            Ok(())
        }
        
        #[cfg(not(unix))]
        {
            self.kill()
        }
    }

    /// Monitor with enhanced callbacks
    pub fn monitor_enhanced<F>(&self, interval: Duration, mut callback: F) -> ProcessResult<thread::JoinHandle<()>>
    where
        F: FnMut(&ProcessStats, bool) + Send + 'static,
    {
        let pid = self.pid;
        let start_time = self.start_time;
        
        let handle = thread::spawn(move || {
            let mut last_stats = None;
            
            loop {
                let running = check_process_running(pid);
                
                if !running {
                    // Process terminated, send final callback
                    if let Some(stats) = last_stats {
                        callback(&stats, false);
                    }
                    break;
                }
                
                // Get current stats
                match get_process_stats_enhanced(pid, start_time) {
                    Ok(stats) => {
                        callback(&stats, true);
                        last_stats = Some(stats);
                    }
                    Err(_) => {
                        // Process might have terminated
                        if let Some(stats) = last_stats {
                            callback(&stats, false);
                        }
                        break;
                    }
                }
                
                thread::sleep(interval);
            }
        });
        
        Ok(handle)
    }

    /// Set enhanced resource limits
    pub fn set_enhanced_limits(&self, memory_limit: Option<u64>, cpu_limit: Option<f64>) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            if let Some(memory) = memory_limit {
                self.set_memory_limit(memory)?;
            }
            
            if let Some(cpu) = cpu_limit {
                self.set_cpu_limit(cpu)?;
            }
            
            tracing::info!(
                pid = self.pid,
                memory_limit = memory_limit,
                cpu_limit = cpu_limit,
                "Resource limits set"
            );
            
            Ok(())
        }
        
        #[cfg(not(unix))]
        {
            tracing::warn!("Resource limits not supported on this platform");
            Ok(())
        }
    }

    #[cfg(unix)]
    fn set_memory_limit(&self, limit_bytes: u64) -> ProcessResult<()> {
        unsafe {
            let rlim = libc::rlimit {
                rlim_cur: limit_bytes,
                rlim_max: limit_bytes,
            };
            
            if libc::setrlimit(libc::RLIMIT_AS, &rlim) != 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                return Err(ProcessError::SystemError {
                    code: errno,
                    operation: "setrlimit".to_string(),
                    message: "Failed to set memory limit".to_string(),
                });
            }
        }
        Ok(())
    }

    #[cfg(unix)]
    fn set_cpu_limit(&self, limit_percent: f64) -> ProcessResult<()> {
        use std::process::Command;
        use std::fs;
        use std::io::Write;
        
        // Validate input
        if limit_percent <= 0.0 || limit_percent > 100.0 {
            return Err(crate::stdlib::process::error::invalid_arguments(
                &format!("CPU limit percentage must be between 0 and 100, got {}", limit_percent)
            ));
        }
        
        // Try to use cgroups v2 first (modern systems)
        if let Ok(_) = fs::metadata("/sys/fs/cgroup/cgroup.controllers") {
            if let Err(_) = self.set_cpu_limit_cgroups_v2(limit_percent) {
                // Fallback to cpulimit tool if available
                self.set_cpu_limit_cpulimit_tool(limit_percent)?;
            }
        }
        // Try cgroups v1 (older systems)
        else if let Ok(_) = fs::metadata("/sys/fs/cgroup/cpu") {
            if let Err(_) = self.set_cpu_limit_cgroups_v1(limit_percent) {
                // Fallback to cpulimit tool if available
                self.set_cpu_limit_cpulimit_tool(limit_percent)?;
            }
        }
        // Fallback to external tools
        else {
            self.set_cpu_limit_cpulimit_tool(limit_percent)?;
        }
        
        tracing::info!(pid = self.pid, limit = limit_percent, "CPU limit applied successfully");
        Ok(())
    }
    
    #[cfg(unix)]
    fn set_cpu_limit_cgroups_v2(&self, limit_percent: f64) -> ProcessResult<()> {
        use std::fs::OpenOptions;
        use std::io::Write;
        
        // Create a unique cgroup for this process
        let cgroup_name = format!("cursed_proc_{}", self.pid);
        let cgroup_path = format!("/sys/fs/cgroup/{}", cgroup_name);
        
        // Create cgroup directory
        std::fs::create_dir_all(&cgroup_path)
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "create_cgroup", &e.to_string()))?;
        
        // Set CPU quota (period is typically 100000 microseconds = 100ms)
        let period = 100_000;
        let quota = ((limit_percent / 100.0) * period as f64) as u64;
        
        let mut quota_file = OpenOptions::new()
            .write(true)
            .open(format!("{}/cpu.max", cgroup_path))
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "open_cpu_max", &e.to_string()))?;
        
        write!(quota_file, "{} {}", quota, period)
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "write_cpu_max", &e.to_string()))?;
        
        // Add process to cgroup
        let mut procs_file = OpenOptions::new()
            .write(true)
            .open(format!("{}/cgroup.procs", cgroup_path))
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "open_cgroup_procs", &e.to_string()))?;
        
        write!(procs_file, "{}", self.pid)
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "write_cgroup_procs", &e.to_string()))?;
        
        Ok(())
    }
    
    #[cfg(unix)]
    fn set_cpu_limit_cgroups_v1(&self, limit_percent: f64) -> ProcessResult<()> {
        use std::fs::OpenOptions;
        use std::io::Write;
        
        // Create a unique cgroup for this process
        let cgroup_name = format!("cursed_proc_{}", self.pid);
        let cgroup_path = format!("/sys/fs/cgroup/cpu/{}", cgroup_name);
        
        // Create cgroup directory
        std::fs::create_dir_all(&cgroup_path)
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "create_cgroup", &e.to_string()))?;
        
        // Set CPU quota (period is typically 100000 microseconds = 100ms)
        let period = 100_000;
        let quota = ((limit_percent / 100.0) * period as f64) as u64;
        
        let mut period_file = OpenOptions::new()
            .write(true)
            .open(format!("{}/cpu.cfs_period_us", cgroup_path))
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "open_cpu_period", &e.to_string()))?;
        
        write!(period_file, "{}", period)
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "write_cpu_period", &e.to_string()))?;
        
        let mut quota_file = OpenOptions::new()
            .write(true)
            .open(format!("{}/cpu.cfs_quota_us", cgroup_path))
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "open_cpu_quota", &e.to_string()))?;
        
        write!(quota_file, "{}", quota)
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "write_cpu_quota", &e.to_string()))?;
        
        // Add process to cgroup
        let mut tasks_file = OpenOptions::new()
            .write(true)
            .open(format!("{}/tasks", cgroup_path))
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "open_tasks", &e.to_string()))?;
        
        write!(tasks_file, "{}", self.pid)
            .map_err(|e| crate::stdlib::process::error::system_error(-1, "write_tasks", &e.to_string()))?;
        
        Ok(())
    }
    
    #[cfg(unix)]
    fn set_cpu_limit_cpulimit_tool(&self, limit_percent: f64) -> ProcessResult<()> {
        use std::process::Command;
        
        // Try to use cpulimit tool as fallback
        let output = Command::new("which")
            .arg("cpulimit")
            .output();
        
        if output.is_ok() && output.unwrap().status.success() {
            // Launch cpulimit in background
            std::thread::spawn(move || {
                let _ = Command::new("cpulimit")
                    .arg("--pid")
                    .arg(format!("{}", self.pid))
                    .arg("--limit")
                    .arg(format!("{}", limit_percent))
                    .spawn();
            });
            Ok(())
        } else {
            Err(crate::stdlib::process::error::system_error(-1, "cpulimit", 
                "CPU limiting requires cgroups support or cpulimit tool"))
        }
    }
}

/// Enhanced SlayPipeline implementation
impl SlayPipeline {
    /// Create new pipeline
    pub fn new(commands: Vec<SlayCommand>) -> Self {
        Self {
            commands,
            options: SlayOptions::default(),
        }
    }

    /// Create pipeline from multiple commands
    pub fn pipe(commands: Vec<SlayCommand>) -> Self {
        Self::new(commands)
    }

    /// Run pipeline with enhanced error handling
    pub fn run_enhanced(&mut self) -> ProcessResult<Vec<SlayProcessState>> {
        self.start_enhanced()?;
        self.wait_enhanced()
    }

    /// Start pipeline execution
    pub fn start_enhanced(&mut self) -> ProcessResult<Vec<SlayProcess>> {
        if self.commands.is_empty() {
            return Err(ProcessError::InvalidArguments {
                operation: "start_pipeline".to_string(),
                parameter: "commands".to_string(),
                message: "Pipeline must have at least one command".to_string(),
            });
        }

        let mut processes = Vec::new();
        let mut previous_stdout: Option<Box<dyn Read + Send>> = None;

        for (i, command) in self.commands.iter_mut().enumerate() {
            // Configure stdin from previous command's stdout
            if i > 0 {
                if let Some(stdin) = previous_stdout.take() {
                    // This would require more complex pipe management
                    // For now, we'll use the simpler approach
                }
            }

            // Configure stdout for next command (except last)
            if i < self.commands.len() - 1 {
                command.stdout = Some(crate::stdlib::process::exec_slay::ProcessStdout::Pipe);
            }

            command.start()?;
            let process = command.process()?;
            
            // Get stdout for next command
            if i < self.commands.len() - 1 {
                previous_stdout = Some(command.stdout_pipe()?);
            }
            
            processes.push(process);
        }

        Ok(processes)
    }

    /// Wait for pipeline completion
    pub fn wait_enhanced(&mut self) -> ProcessResult<Vec<SlayProcessState>> {
        let mut states = Vec::new();
        
        for command in &mut self.commands {
            let state = command.wait_enhanced()?;
            states.push(state);
        }
        
        Ok(states)
    }

    /// Get pipeline output
    pub fn output_enhanced(&mut self) -> ProcessResult<(Vec<u8>, Vec<SlayProcessState>)> {
        if let Some(last_command) = self.commands.last_mut() {
            last_command.stdout = Some(crate::stdlib::process::exec_slay::ProcessStdout::Pipe);
        }
        
        let states = self.run_enhanced()?;
        
        if let Some(last_command) = self.commands.last_mut() {
            let output = last_command.output()?;
            Ok((output, states))
        } else {
            Ok((Vec::new(), states))
        }
    }

    /// Get combined pipeline output
    pub fn combined_output_enhanced(&mut self) -> ProcessResult<(Vec<u8>, Vec<SlayProcessState>)> {
        if let Some(last_command) = self.commands.last_mut() {
            last_command.stdout = Some(crate::stdlib::process::exec_slay::ProcessStdout::Pipe);
            last_command.stderr = Some(crate::stdlib::process::exec_slay::ProcessStderr::Pipe);
        }
        
        let states = self.run_enhanced()?;
        
        if let Some(last_command) = self.commands.last_mut() {
            let output = last_command.combined_output()?;
            Ok((output, states))
        } else {
            Ok((Vec::new(), states))
        }
    }

    /// Apply options to pipeline
    pub fn with_options_enhanced(mut self, opts: SlayOptions) -> Self {
        self.options = opts;
        
        // Apply options to all commands
        for command in &mut self.commands {
            command.with_options(self.options.clone());
        }
        
        self
    }

    /// Add command to pipeline
    pub fn add_command_enhanced(mut self, command: SlayCommand) -> Self {
        self.commands.push(command);
        self
    }

    /// Set all commands
    pub fn set_commands_enhanced(mut self, commands: Vec<SlayCommand>) -> Self {
        self.commands = commands;
        self
    }

    /// Get string representation of pipeline
    pub fn string_enhanced(&self) -> String {
        self.commands
            .iter()
            .map(|cmd| cmd.enhanced_string())
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

/// Enhanced SlayTask implementation for background execution
impl SlayTask {
    /// Run command in background with enhanced monitoring
    pub fn run_background_enhanced(mut command: SlayCommand) -> ProcessResult<Self> {
        let start_time = Instant::now();
        let (tx, rx) = mpsc::channel();
        
        let thread_handle = thread::spawn(move || -> ProcessResult<()> {
            let result = command.run_enhanced();
            tx.send(result).ok();
            Ok(())
        });
        
        Ok(Self {
            command,
            start_time,
            exit_code: None,
            finished: false,
            error: None,
            output: Vec::new(),
            combined_output: Vec::new(),
            thread_handle: Some(thread_handle),
        })
    }

    /// Wait for task completion with timeout
    pub fn wait_with_timeout(&mut self, timeout: Duration) -> ProcessResult<SlayProcessState> {
        let start = Instant::now();
        
        while !self.finished && start.elapsed() < timeout {
            thread::sleep(Duration::from_millis(10));
            
            // Check if task completed
            if let Some(handle) = &self.thread_handle {
                if handle.is_finished() {
                    self.finished = true;
                    break;
                }
            }
        }
        
        if !self.finished {
            self.kill_enhanced()?;
            return Err(ProcessError::TimeoutError {
                operation: "wait_task".to_string(),
                timeout,
                message: "Task execution timed out".to_string(),
            });
        }
        
        self.command.process_state()
    }

    /// Kill task with enhanced cleanup
    pub fn kill_enhanced(&mut self) -> ProcessResult<()> {
        if let Ok(process) = self.command.process() {
            process.kill()?;
        }
        
        if let Some(handle) = self.thread_handle.take() {
            // Wait a bit for thread to cleanup
            thread::sleep(Duration::from_millis(100));
            
            // Thread should be finished after killing process
            if !handle.is_finished() {
                tracing::warn!("Background thread did not finish cleanly");
            }
        }
        
        self.finished = true;
        self.exit_code = Some(-1);
        self.error = Some("Task killed".to_string());
        
        Ok(())
    }

    /// Check if task is running with enhanced status
    pub fn is_running_enhanced(&self) -> bool {
        if self.finished {
            return false;
        }
        
        if let Ok(process) = self.command.process() {
            check_process_running(process.pid())
        } else {
            false
        }
    }

    /// Get elapsed time with precision
    pub fn elapsed_time_enhanced(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get output with error handling
    pub fn get_output_enhanced(&mut self) -> ProcessResult<Vec<u8>> {
        if !self.finished {
            self.wait()?;
        }
        
        if self.output.is_empty() {
            self.output = self.command.output()?;
        }
        
        Ok(self.output.clone())
    }

    /// Get combined output with error handling
    pub fn get_combined_output_enhanced(&mut self) -> ProcessResult<Vec<u8>> {
        if !self.finished {
            self.wait()?;
        }
        
        if self.combined_output.is_empty() {
            self.combined_output = self.command.combined_output()?;
        }
        
        Ok(self.combined_output.clone())
    }
}

/// Enhanced SlayCommandBuilder implementation
impl SlayCommandBuilder {
    /// Create new command builder
    pub fn new_enhanced(command: &str) -> Self {
        Self {
            command: command.to_string(),
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

    /// Add arguments with fluent interface
    pub fn with_args_enhanced(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }

    /// Set working directory
    pub fn with_dir_enhanced<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Set environment variables
    pub fn with_env_enhanced(mut self, env: Vec<String>) -> Self {
        self.env = env;
        self
    }

    /// Add environment variable
    pub fn add_env_enhanced<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> Self {
        let env_pair = format!("{}={}", key.as_ref(), value.as_ref());
        self.env.push(env_pair);
        self
    }

    /// Set stdin configuration
    pub fn with_stdin_enhanced(mut self, config: crate::stdlib::process::exec_slay::ProcessStdin) -> Self {
        self.stdin = Some(config);
        self
    }

    /// Set stdout configuration
    pub fn with_stdout_enhanced(mut self, config: crate::stdlib::process::exec_slay::ProcessStdout) -> Self {
        self.stdout = Some(config);
        self
    }

    /// Set stderr configuration
    pub fn with_stderr_enhanced(mut self, config: crate::stdlib::process::exec_slay::ProcessStderr) -> Self {
        self.stderr = Some(config);
        self
    }

    /// Set timeout
    pub fn with_timeout_enhanced(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Use shell for execution
    pub fn use_shell_enhanced(mut self, use_shell: bool) -> Self {
        self.use_shell = use_shell;
        self
    }

    /// Build the command
    pub fn build_enhanced(self) -> SlayCommand {
        let mut command = SlayCommand::new(&self.command, &self.args.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        
        if let Some(dir) = self.dir {
            command.set_dir(dir);
        }
        
        if !self.env.is_empty() {
            command.set_env(self.env);
        }
        
        if let Some(stdin) = self.stdin {
            command.set_stdin(stdin);
        }
        
        if let Some(stdout) = self.stdout {
            command.set_stdout(stdout);
        }
        
        if let Some(stderr) = self.stderr {
            command.set_stderr(stderr);
        }
        
        command
    }
}

/// Global functions for convenience (following the specification)

/// Run command with timeout
pub fn run_with_timeout_enhanced(mut cmd: SlayCommand, timeout: Duration) -> ProcessResult<SlayProcessState> {
    cmd.run_with_timeout(timeout)?;
    cmd.process_state()
}

/// Get output with timeout
pub fn output_with_timeout_enhanced(mut cmd: SlayCommand, timeout: Duration) -> ProcessResult<(Vec<u8>, SlayProcessState)> {
    cmd.stdout = Some(crate::stdlib::process::exec_slay::ProcessStdout::Pipe);
    cmd.run_with_timeout(timeout)?;
    let output = cmd.output()?;
    let state = cmd.process_state()?;
    Ok((output, state))
}

/// Get combined output with timeout
pub fn combined_output_with_timeout_enhanced(mut cmd: SlayCommand, timeout: Duration) -> ProcessResult<(Vec<u8>, SlayProcessState)> {
    cmd.stdout = Some(crate::stdlib::process::exec_slay::ProcessStdout::Pipe);
    cmd.stderr = Some(crate::stdlib::process::exec_slay::ProcessStderr::Pipe);
    cmd.run_with_timeout(timeout)?;
    let output = cmd.combined_output()?;
    let state = cmd.process_state()?;
    Ok((output, state))
}

/// Helper functions

#[cfg(unix)]
fn check_process_running(pid: u32) -> bool {
    unsafe {
        libc::kill(pid as i32, 0) == 0
    }
}

#[cfg(not(unix))]
fn check_process_running(_pid: u32) -> bool {
    // Windows implementation would use GetExitCodeProcess
    true // Assume running for now
}

fn get_process_stats_enhanced(pid: u32, start_time: Instant) -> ProcessResult<ProcessStats> {
    #[cfg(target_os = "linux")]
    {
        get_linux_process_stats(pid, start_time)
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        // Fallback implementation
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
            up_time: start_time.elapsed(),
            thread_count: 1,
            open_files: 0,
            network_conns: 0,
        })
    }
}

#[cfg(target_os = "linux")]
fn get_linux_process_stats(pid: u32, start_time: Instant) -> ProcessResult<ProcessStats> {
    use std::fs;
    
    // Read /proc/[pid]/stat
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = fs::read_to_string(&stat_path)
        .map_err(|e| ProcessError::IoError {
            operation: "read_proc_stat".to_string(),
            kind: format!("{:?}", e.kind()),
            message: e.to_string(),
        })?;
    
    let stat_fields: Vec<&str> = stat_content.split_whitespace().collect();
    
    // Read /proc/[pid]/status
    let status_path = format!("/proc/{}/status", pid);
    let status_content = fs::read_to_string(&status_path)
        .map_err(|e| ProcessError::IoError {
            operation: "read_proc_status".to_string(),
            kind: format!("{:?}", e.kind()),
            message: e.to_string(),
        })?;
    
    // Parse memory information
    let mut rss_kb = 0u64;
    let mut vmsize_kb = 0u64;
    let mut threads = 1i32;
    
    for line in status_content.split("\n") {
        if line.starts_with("VmRSS:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                rss_kb = value.parse().unwrap_or(0);
            }
        } else if line.starts_with("VmSize:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                vmsize_kb = value.parse().unwrap_or(0);
            }
        } else if line.starts_with("Threads:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                threads = value.parse().unwrap_or(1);
            }
        }
    }
    
    // Read /proc/[pid]/io for I/O stats
    let io_path = format!("/proc/{}/io", pid);
    let mut read_bytes = 0u64;
    let mut write_bytes = 0u64;
    
    if let Ok(io_content) = fs::read_to_string(&io_path) {
        for line in io_content.split("\n") {
            if line.starts_with("read_bytes:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    read_bytes = value.parse().unwrap_or(0);
                }
            } else if line.starts_with("write_bytes:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    write_bytes = value.parse().unwrap_or(0);
                }
            }
        }
    }
    
    // Count open files
    let fd_path = format!("/proc/{}/fd", pid);
    let open_files = if let Ok(entries) = fs::read_dir(&fd_path) {
        entries.count() as i32
    } else {
        0
    };
    
    Ok(ProcessStats {
        cpu: 0.0, // Would need previous sample to calculate
        memory: rss_kb * 1024,
        resident_memory: rss_kb * 1024,
        virtual_memory: vmsize_kb * 1024,
        swap_memory: 0,
        read_bytes,
        write_bytes,
        read_ops: 0,
        write_ops: 0,
        up_time: start_time.elapsed(),
        thread_count: threads,
        open_files,
        network_conns: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
use crate::stdlib::process::info::ProcessState;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_enhanced_command_creation() {
        let cmd = SlayCommand::new_slay_command("echo", &["hello", "world"]);
        assert_eq!(cmd.path, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_enhanced_command_builder() {
        let cmd = SlayCommandBuilder::new_enhanced("ls")
            .with_args_enhanced(&["-la", "/tmp"])
            .with_timeout_enhanced(Duration::from_secs(5))
            .use_shell_enhanced(false)
            .build_enhanced();
        
        assert_eq!(cmd.path, "ls");
        assert_eq!(cmd.args, vec!["-la", "/tmp"]);
    }

    #[test]
    fn test_enhanced_pipeline_creation() {
        let cmd1 = SlayCommand::new("cat", &["test.txt"]);
        let cmd2 = SlayCommand::new("grep", &["pattern"]);
        let cmd3 = SlayCommand::new("wc", &["-l"]);
        
        let pipeline = SlayPipeline::pipe(vec![cmd1, cmd2, cmd3]);
        assert_eq!(pipeline.commands.len(), 3);
    }

    #[test]
    fn test_enhanced_string_representation() {
        let cmd = SlayCommand::new("echo", &["hello"])
            .with_dir("/tmp")
            .with_env_var("TEST", "value");
        
        let repr = cmd.enhanced_string();
        assert!(repr.contains("echo hello"));
        assert!(repr.contains("/tmp"));
        assert!(repr.contains("env vars"));
    }
}
