use crate::error::CursedError;
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
    // Search in PATH
    if let Ok(path_var) = env::var("PATH") {
        let path_separator = if cfg!(windows) { ';' } else { ':' };
        
        for path_dir in path_var.split(path_separator) {
            if path_dir.is_empty() {
                continue;
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
                    let mut cmd_candidate = candidate.clone();
                    cmd_candidate.set_extension("cmd");
                    if cmd_candidate.exists() {
                        return Ok(cmd_candidate.to_string_lossy().to_string());
                    let mut bat_candidate = candidate.clone();
                    bat_candidate.set_extension("bat");
                    if bat_candidate.exists() {
                        return Ok(bat_candidate.to_string_lossy().to_string());
                    }
                }
            if candidate.exists() {
                return Ok(candidate.to_string_lossy().to_string());
            }
        }
    Err(system_error(&format!("Executable not found in PATH: {}", file)))
/// Resource limits for process execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    /// Maximum CPU time in seconds
    /// Maximum wall clock time
    /// Maximum number of open files
    /// Maximum number of processes/threads
impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
        }
    }
impl ResourceLimits {
    /// Create new resource limits
    pub fn new() -> Self {
        Self::default()
    /// Set maximum memory
    pub fn with_max_memory(mut self, max_memory: u64) -> Self {
        self.max_memory = Some(max_memory);
        self
    /// Set maximum CPU time
    pub fn with_max_cpu_time(mut self, max_cpu_time: Duration) -> Self {
        self.max_cpu_time = Some(max_cpu_time);
        self
    /// Set maximum wall time
    pub fn with_max_wall_time(mut self, max_wall_time: Duration) -> Self {
        self.max_wall_time = Some(max_wall_time);
        self
    /// Set maximum open files
    pub fn with_max_open_files(mut self, max_open_files: u64) -> Self {
        self.max_open_files = Some(max_open_files);
        self
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
    /// Drop privileges to this group (Unix only)
    /// Chroot to this directory (Unix only)
    /// Use a sandbox environment
    /// Disable network access
    /// Read-only filesystem
    /// Allowed file paths for access
impl Default for SecurityOptions {
    fn default() -> Self {
        Self {
        }
    }
/// Process monitor for tracking resource usage
#[derive(Debug)]
pub struct ProcessMonitor {
    /// Process being monitored
    /// Resource limits
    /// Monitoring interval
    /// Whether monitoring is active
    /// Collected statistics
#[derive(Debug, Clone)]
pub struct ProcessResourceSnapshot {
impl ProcessMonitor {
    /// Create a new process monitor
    pub fn new(process: Process, limits: ResourceLimits) -> Self {
        Self {
        }
    }
    
    /// Set monitoring interval
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    /// Start monitoring
    pub fn start(&self) -> ExecResult<()> {
        {
            let mut active = self.active.lock().unwrap();
            *active = true;
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
    /// Stop monitoring
    pub fn stop(&self) {
        let mut active = self.active.lock().unwrap();
        *active = false;
    /// Get collected statistics
    pub fn get_stats(&self) -> Vec<ProcessResourceSnapshot> {
        self.stats.lock().unwrap().clone()
    fn collect_snapshot(pid: u32) -> ExecResult<ProcessResourceSnapshot> {
        // This is a simplified implementation
        // In practice, this would read from /proc/PID/stat on Linux,
        // or use platform-specific APIs
        Ok(ProcessResourceSnapshot {
            memory_usage: 0, // Would read actual memory usage
            cpu_percentage: 0.0, // Would calculate CPU percentage
            open_files: 0, // Would count open file descriptors
            threads: 1, // Would count threads
        })
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
    /// Currently running processes
    /// Queue of pending commands
    /// Pool statistics
#[derive(Debug, Default)]
struct PoolStats {
impl ProcessPool {
    /// Create a new process pool
    pub fn new(max_processes: usize) -> Self {
        Self {
        }
    }
    
    /// Submit a command for execution
    pub fn submit(&self, cmd: Cmd) -> ExecResult<()> {
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_submitted += 1;
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(cmd);
        
        // Try to start queued processes
        self.process_queue();
        
        Ok(())
    /// Get the number of running processes
    pub fn running_count(&self) -> usize {
        self.running.lock().unwrap().len()
    /// Get the number of queued processes
    pub fn queue_size(&self) -> usize {
        self.queue.lock().unwrap().len()
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
    /// Execution mode
    /// Maximum concurrent processes
#[derive(Debug, Clone)]
pub enum BatchMode {
    /// Execute all commands in parallel
    /// Execute commands sequentially
    /// Execute in batches of specified size
impl BatchRunner {
    /// Create a new batch runner
    pub fn new(commands: Vec<Cmd>) -> Self {
        Self {
        }
    }
    
    /// Set execution mode
    pub fn with_mode(mut self, mode: BatchMode) -> Self {
        self.mode = mode;
        self
    /// Set maximum concurrent processes
    pub fn with_max_concurrent(mut self, max_concurrent: usize) -> Self {
        self.max_concurrent = Some(max_concurrent);
        self
    /// Execute all commands
    pub fn run(&mut self) -> ExecResult<Vec<ProcessState>> {
        match self.mode {
        }
    }
    
    fn run_sequential(&mut self) -> ExecResult<Vec<ProcessState>> {
        let mut results = Vec::new();
        
        for cmd in &mut self.commands {
            let state = cmd.run()?;
            results.push(state);
        Ok(results)
    fn run_parallel(&mut self) -> ExecResult<Vec<ProcessState>> {
        let mut group = ProcessGroup::new();
        
        for cmd in std::mem::take(&mut self.commands) {
            group.add_command(cmd);
        group.run()
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
    /// Whether the platform supports signals
    /// Whether the platform supports resource limits
    /// Whether the platform supports chroot
    /// Whether the platform supports namespaces
impl PlatformFeatures {
    /// Detect platform features
    pub fn detect() -> Self {
        Self {
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

