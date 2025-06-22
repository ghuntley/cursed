/// Enhanced features for exec_vibez
/// 
/// Implements advanced functionality including LookPath, ProcessMonitor, 
/// ResourceLimits, SecurityOptions, ProcessPool, ProcessQueue, BatchRunner,
/// and cross-platform utilities.

use std::collections::{HashMap, VecDeque};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::thread;

use super::cmd::Cmd;
use super::process::{Process, ProcessState};
use super::groups::{ProcessGroup, ProcessGroupOptions};
use super::error::{ExecResult, ExecError, execution_failed, invalid_arguments, system_error};

/// Look up the executable path for a named program
pub fn look_path(file: &str) -> ExecResult<String> {
    // Check if the file is already an absolute path
    if Path::new(file).is_absolute() {
        if Path::new(file).exists() {
            return Ok(file.to_string());
        } else {
            return Err(system_error(&format!("Executable not found: {}", file)));
        }
    }
    
    // Check if it's a relative path with directory separators
    if file.contains('/') || (cfg!(windows) && file.contains('\\')) {
        let path = Path::new(file);
        if path.exists() {
            if let Ok(canonical) = path.canonicalize() {
                return Ok(canonical.to_string_lossy().to_string());
            }
        }
        return Err(system_error(&format!("Executable not found: {}", file)));
    }
    
    // Search in PATH
    if let Ok(path_var) = env::var("PATH") {
        let path_separator = if cfg!(windows) { ';' } else { ':' };
        
        for path_dir in path_var.split(path_separator) {
            if path_dir.is_empty() {
                continue;
            }
            
            let mut candidate = PathBuf::from(path_dir);
            candidate.push(file);
            
            // On Windows, try with .exe extension if not already present
            #[cfg(windows)]
            {
                if !file.ends_with(".exe") && !file.ends_with(".cmd") && !file.ends_with(".bat") {
                    let mut exe_candidate = candidate.clone();
                    exe_candidate.set_extension("exe");
                    if exe_candidate.exists() {
                        return Ok(exe_candidate.to_string_lossy().to_string());
                    }
                    
                    let mut cmd_candidate = candidate.clone();
                    cmd_candidate.set_extension("cmd");
                    if cmd_candidate.exists() {
                        return Ok(cmd_candidate.to_string_lossy().to_string());
                    }
                    
                    let mut bat_candidate = candidate.clone();
                    bat_candidate.set_extension("bat");
                    if bat_candidate.exists() {
                        return Ok(bat_candidate.to_string_lossy().to_string());
                    }
                }
            }
            
            if candidate.exists() {
                return Ok(candidate.to_string_lossy().to_string());
            }
        }
    }
    
    Err(system_error(&format!("Executable not found in PATH: {}", file)))
}

/// Resource limits for process execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    pub max_memory: Option<u64>,
    /// Maximum CPU time in seconds
    pub max_cpu_time: Option<Duration>,
    /// Maximum wall clock time
    pub max_wall_time: Option<Duration>,
    /// Maximum number of open files
    pub max_open_files: Option<u64>,
    /// Maximum number of processes/threads
    pub max_processes: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: None,
            max_cpu_time: None,
            max_wall_time: None,
            max_open_files: None,
            max_processes: None,
        }
    }
}

impl ResourceLimits {
    /// Create new resource limits
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set maximum memory
    pub fn with_max_memory(mut self, max_memory: u64) -> Self {
        self.max_memory = Some(max_memory);
        self
    }
    
    /// Set maximum CPU time
    pub fn with_max_cpu_time(mut self, max_cpu_time: Duration) -> Self {
        self.max_cpu_time = Some(max_cpu_time);
        self
    }
    
    /// Set maximum wall time
    pub fn with_max_wall_time(mut self, max_wall_time: Duration) -> Self {
        self.max_wall_time = Some(max_wall_time);
        self
    }
    
    /// Set maximum open files
    pub fn with_max_open_files(mut self, max_open_files: u64) -> Self {
        self.max_open_files = Some(max_open_files);
        self
    }
    
    /// Set maximum processes
    pub fn with_max_processes(mut self, max_processes: u64) -> Self {
        self.max_processes = Some(max_processes);
        self
    }
}

/// Security options for process execution
#[derive(Debug, Clone)]
pub struct SecurityOptions {
    /// Drop privileges to this user (Unix only)
    pub user: Option<String>,
    /// Drop privileges to this group (Unix only)
    pub group: Option<String>,
    /// Chroot to this directory (Unix only)
    pub chroot: Option<PathBuf>,
    /// Use a sandbox environment
    pub sandbox: bool,
    /// Disable network access
    pub no_network: bool,
    /// Read-only filesystem
    pub readonly_fs: bool,
    /// Allowed file paths for access
    pub allowed_paths: Vec<PathBuf>,
}

impl Default for SecurityOptions {
    fn default() -> Self {
        Self {
            user: None,
            group: None,
            chroot: None,
            sandbox: false,
            no_network: false,
            readonly_fs: false,
            allowed_paths: Vec::new(),
        }
    }
}

/// Process monitor for tracking resource usage
#[derive(Debug)]
pub struct ProcessMonitor {
    /// Process being monitored
    process: Process,
    /// Resource limits
    limits: ResourceLimits,
    /// Monitoring interval
    interval: Duration,
    /// Whether monitoring is active
    active: Arc<Mutex<bool>>,
    /// Collected statistics
    stats: Arc<Mutex<Vec<ProcessResourceSnapshot>>>,
}

#[derive(Debug, Clone)]
pub struct ProcessResourceSnapshot {
    pub timestamp: Instant,
    pub memory_usage: u64,
    pub cpu_percentage: f64,
    pub open_files: u32,
    pub threads: u32,
}

impl ProcessMonitor {
    /// Create a new process monitor
    pub fn new(process: Process, limits: ResourceLimits) -> Self {
        Self {
            process,
            limits,
            interval: Duration::from_millis(100),
            active: Arc::new(Mutex::new(false)),
            stats: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Set monitoring interval
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }
    
    /// Start monitoring
    pub fn start(&self) -> ExecResult<()> {
        {
            let mut active = self.active.lock().unwrap();
            *active = true;
        }
        
        let process_pid = self.process.pid();
        let limits = self.limits.clone();
        let interval = self.interval;
        let active = Arc::clone(&self.active);
        let stats = Arc::clone(&self.stats);
        
        thread::spawn(move || {
            while *active.lock().unwrap() {
                // Collect resource usage stats
                if let Ok(snapshot) = Self::collect_snapshot(process_pid) {
                    stats.lock().unwrap().push(snapshot.clone());
                    
                    // Check limits
                    if Self::check_limits(&snapshot, &limits) {
                        tracing::warn!("Process {} exceeded resource limits", process_pid);
                        // In a real implementation, we might kill the process here
                    }
                }
                
                thread::sleep(interval);
            }
        });
        
        Ok(())
    }
    
    /// Stop monitoring
    pub fn stop(&self) {
        let mut active = self.active.lock().unwrap();
        *active = false;
    }
    
    /// Get collected statistics
    pub fn get_stats(&self) -> Vec<ProcessResourceSnapshot> {
        self.stats.lock().unwrap().clone()
    }
    
    fn collect_snapshot(pid: u32) -> ExecResult<ProcessResourceSnapshot> {
        // This is a simplified implementation
        // In practice, this would read from /proc/PID/stat on Linux,
        // or use platform-specific APIs
        Ok(ProcessResourceSnapshot {
            timestamp: Instant::now(),
            memory_usage: 0, // Would read actual memory usage
            cpu_percentage: 0.0, // Would calculate CPU percentage
            open_files: 0, // Would count open file descriptors
            threads: 1, // Would count threads
        })
    }
    
    fn check_limits(snapshot: &ProcessResourceSnapshot, limits: &ResourceLimits) -> bool {
        if let Some(max_memory) = limits.max_memory {
            if snapshot.memory_usage > max_memory {
                return true;
            }
        }
        
        // Check other limits...
        false
    }
}

/// Process pool for managing concurrent execution
#[derive(Debug)]
pub struct ProcessPool {
    /// Maximum number of concurrent processes
    max_processes: usize,
    /// Currently running processes
    running: Arc<Mutex<HashMap<u32, Process>>>,
    /// Queue of pending commands
    queue: Arc<Mutex<VecDeque<Cmd>>>,
    /// Pool statistics
    stats: Arc<Mutex<PoolStats>>,
}

#[derive(Debug, Default)]
struct PoolStats {
    total_submitted: usize,
    total_completed: usize,
    total_failed: usize,
}

impl ProcessPool {
    /// Create a new process pool
    pub fn new(max_processes: usize) -> Self {
        Self {
            max_processes,
            running: Arc::new(Mutex::new(HashMap::new())),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(Mutex::new(PoolStats::default())),
        }
    }
    
    /// Submit a command for execution
    pub fn submit(&self, cmd: Cmd) -> ExecResult<()> {
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_submitted += 1;
        }
        
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(cmd);
        
        // Try to start queued processes
        self.process_queue();
        
        Ok(())
    }
    
    /// Get the number of running processes
    pub fn running_count(&self) -> usize {
        self.running.lock().unwrap().len()
    }
    
    /// Get the number of queued processes
    pub fn queue_size(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
    
    fn process_queue(&self) {
        let mut running = self.running.lock().unwrap();
        let mut queue = self.queue.lock().unwrap();
        
        // Clean up completed processes
        let mut completed = Vec::new();
        for (&pid, process) in running.iter() {
            if !process.is_running() {
                completed.push(pid);
            }
        }
        
        for pid in completed {
            running.remove(&pid);
            let mut stats = self.stats.lock().unwrap();
            stats.total_completed += 1;
        }
        
        // Start new processes if we have capacity
        while running.len() < self.max_processes && !queue.is_empty() {
            if let Some(mut cmd) = queue.pop_front() {
                match cmd.start() {
                    Ok(process) => {
                        let pid = process.pid();
                        running.insert(pid, process);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to start process: {}", e);
                        let mut stats = self.stats.lock().unwrap();
                        stats.total_failed += 1;
                    }
                }
            }
        }
    }
}

/// Process queue for ordered execution
pub type ProcessQueue = VecDeque<Cmd>;

/// Batch runner for executing multiple commands
#[derive(Debug)]
pub struct BatchRunner {
    /// Commands to execute
    commands: Vec<Cmd>,
    /// Execution mode
    mode: BatchMode,
    /// Maximum concurrent processes
    max_concurrent: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum BatchMode {
    /// Execute all commands in parallel
    Parallel,
    /// Execute commands sequentially
    Sequential,
    /// Execute in batches of specified size
    Batched(usize),
}

impl BatchRunner {
    /// Create a new batch runner
    pub fn new(commands: Vec<Cmd>) -> Self {
        Self {
            commands,
            mode: BatchMode::Sequential,
            max_concurrent: None,
        }
    }
    
    /// Set execution mode
    pub fn with_mode(mut self, mode: BatchMode) -> Self {
        self.mode = mode;
        self
    }
    
    /// Set maximum concurrent processes
    pub fn with_max_concurrent(mut self, max_concurrent: usize) -> Self {
        self.max_concurrent = Some(max_concurrent);
        self
    }
    
    /// Execute all commands
    pub fn run(&mut self) -> ExecResult<Vec<ProcessState>> {
        match self.mode {
            BatchMode::Sequential => self.run_sequential(),
            BatchMode::Parallel => self.run_parallel(),
            BatchMode::Batched(batch_size) => self.run_batched(batch_size),
        }
    }
    
    fn run_sequential(&mut self) -> ExecResult<Vec<ProcessState>> {
        let mut results = Vec::new();
        
        for cmd in &mut self.commands {
            let state = cmd.run()?;
            results.push(state);
        }
        
        Ok(results)
    }
    
    fn run_parallel(&mut self) -> ExecResult<Vec<ProcessState>> {
        let mut group = ProcessGroup::new();
        
        for cmd in std::mem::take(&mut self.commands) {
            group.add_command(cmd);
        }
        
        group.run()
    }
    
    fn run_batched(&mut self, batch_size: usize) -> ExecResult<Vec<ProcessState>> {
        let mut all_results = Vec::new();
        
        for chunk in self.commands.chunks_mut(batch_size) {
            let mut group = ProcessGroup::new();
            
            // We need to clone commands since we can't move out of a slice
            for cmd in chunk {
                // This is a limitation - we'd need to restructure to avoid cloning
                // For now, we'll run sequentially within batches
                let state = cmd.run()?;
                all_results.push(state);
            }
        }
        
        Ok(all_results)
    }
}

/// Platform-specific features detection
#[derive(Debug)]
pub struct PlatformFeatures {
    /// Whether the platform supports process groups
    pub process_groups: bool,
    /// Whether the platform supports signals
    pub signals: bool,
    /// Whether the platform supports resource limits
    pub resource_limits: bool,
    /// Whether the platform supports chroot
    pub chroot: bool,
    /// Whether the platform supports namespaces
    pub namespaces: bool,
}

impl PlatformFeatures {
    /// Detect platform features
    pub fn detect() -> Self {
        Self {
            process_groups: cfg!(unix),
            signals: cfg!(unix),
            resource_limits: cfg!(unix),
            chroot: cfg!(unix),
            namespaces: cfg!(target_os = "linux"),
        }
    }
}

/// Cross-platform utilities
pub struct CrossPlatformUtils;

impl CrossPlatformUtils {
    /// Get the platform-specific shell
    pub fn get_shell() -> &'static str {
        if cfg!(windows) {
            "cmd.exe"
        } else {
            "/bin/sh"
        }
    }
    
    /// Get shell arguments for command execution
    pub fn get_shell_args() -> Vec<&'static str> {
        if cfg!(windows) {
            vec!["/C"]
        } else {
            vec!["-c"]
        }
    }
    
    /// Get the path separator
    pub fn path_separator() -> char {
        if cfg!(windows) { ';' } else { ':' }
    }
    
    /// Get the directory separator
    pub fn dir_separator() -> char {
        if cfg!(windows) { '\\' } else { '/' }
    }
    
    /// Check if a file is executable
    pub fn is_executable(path: &Path) -> bool {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = path.metadata() {
                let permissions = metadata.permissions();
                permissions.mode() & 0o111 != 0
            } else {
                false
            }
        }
        
        #[cfg(windows)]
        {
            // On Windows, check file extension
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                matches!(ext.as_str(), "exe" | "bat" | "cmd" | "com")
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::info::ProcessState;
    
    #[test]
    fn test_look_path_absolute() {
        // Test with an absolute path that should exist
        if cfg!(unix) {
            let result = look_path("/bin/sh");
            if Path::new("/bin/sh").exists() {
                assert!(result.is_ok());
            }
        }
    }
    
    #[test]
    fn test_look_path_nonexistent() {
        let result = look_path("nonexistent_program_12345");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_resource_limits_builder() {
        let limits = ResourceLimits::new()
            .with_max_memory(1024 * 1024)
            .with_max_cpu_time(Duration::from_secs(30))
            .with_max_wall_time(Duration::from_secs(60));
        
        assert_eq!(limits.max_memory, Some(1024 * 1024));
        assert_eq!(limits.max_cpu_time, Some(Duration::from_secs(30)));
        assert_eq!(limits.max_wall_time, Some(Duration::from_secs(60)));
    }
    
    #[test]
    fn test_security_options_default() {
        let options = SecurityOptions::default();
        assert!(!options.sandbox);
        assert!(!options.no_network);
        assert!(!options.readonly_fs);
        assert!(options.allowed_paths.is_empty());
    }
    
    #[test]
    fn test_process_pool_creation() {
        let pool = ProcessPool::new(4);
        assert_eq!(pool.max_processes, 4);
        assert_eq!(pool.running_count(), 0);
        assert_eq!(pool.queue_size(), 0);
    }
    
    #[test]
    fn test_batch_runner_creation() {
        let commands = vec![
            Cmd::new("echo", &["hello"]),
            Cmd::new("echo", &["world"]),
        ];
        
        let runner = BatchRunner::new(commands);
        assert_eq!(runner.commands.len(), 2);
    }
    
    #[test]
    fn test_platform_features_detection() {
        let features = PlatformFeatures::detect();
        
        if cfg!(unix) {
            assert!(features.process_groups);
            assert!(features.signals);
            assert!(features.resource_limits);
            assert!(features.chroot);
        }
        
        if cfg!(target_os = "linux") {
            assert!(features.namespaces);
        }
    }
    
    #[test]
    fn test_cross_platform_utils() {
        let shell = CrossPlatformUtils::get_shell();
        assert!(!shell.is_empty());
        
        let args = CrossPlatformUtils::get_shell_args();
        assert!(!args.is_empty());
        
        let path_sep = CrossPlatformUtils::path_separator();
        assert!(path_sep == ':' || path_sep == ';');
        
        let dir_sep = CrossPlatformUtils::dir_separator();
        assert!(dir_sep == '/' || dir_sep == '\\');
    }
}


pub trait LookPath {
    fn lookup_path(&self) -> Option<std::path::PathBuf>;
}
