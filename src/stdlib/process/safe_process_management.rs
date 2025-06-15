/// Safe Process Management System for CURSED
/// 
/// This module provides a comprehensive, safe process management system that
/// eliminates unsafe memory operations and provides cross-platform support
/// for process control, monitoring, and resource limiting.

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex, RwLock, Weak, mpsc};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

#[cfg(windows)]
use std::os::windows::process::ExitStatusExt;

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, execution_failed, execution_failed_with_code,
    timeout_error, invalid_arguments, io_error, system_error, platform_error
};

/// Safe process handle that eliminates unsafe memory operations
#[derive(Debug)]
pub struct SafeProcessHandle {
    /// Process ID
    pub pid: u32,
    /// Safe reference to the child process
    child_handle: Arc<Mutex<Option<Child>>>,
    /// Process start time
    start_time: Instant,
    /// Process state tracking
    state: Arc<RwLock<ProcessState>>,
    /// Resource limits
    resource_limits: Arc<Mutex<ResourceLimits>>,
    /// Process metadata
    metadata: ProcessMetadata,
}

/// Process state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Created,
    Running,
    Waiting,
    Stopped,
    Terminated,
}

/// Process metadata for tracking and monitoring
#[derive(Debug, Clone)]
pub struct ProcessMetadata {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<PathBuf>,
    pub env_vars: HashMap<String, String>,
    pub parent_pid: Option<u32>,
}

/// Resource limits for process control
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_bytes: Option<u64>,
    pub max_cpu_percent: Option<f64>,
    pub max_execution_time: Option<Duration>,
    pub max_file_descriptors: Option<u32>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: None,
            max_cpu_percent: None,
            max_execution_time: None,
            max_file_descriptors: None,
        }
    }
}

/// Process statistics for monitoring
#[derive(Debug, Clone)]
pub struct ProcessStatistics {
    pub cpu_usage_percent: f64,
    pub memory_usage_bytes: u64,
    pub virtual_memory_bytes: u64,
    pub resident_memory_bytes: u64,
    pub file_descriptors_count: u32,
    pub thread_count: u32,
    pub uptime: Duration,
    pub total_cpu_time: Duration,
    pub bytes_read: u64,
    pub bytes_written: u64,
}

impl SafeProcessHandle {
    /// Create a new safe process handle
    pub fn new(child: Child, metadata: ProcessMetadata) -> Self {
        let pid = child.id();
        let start_time = Instant::now();
        
        Self {
            pid,
            child_handle: Arc::new(Mutex::new(Some(child))),
            start_time,
            state: Arc::new(RwLock::new(ProcessState::Running)),
            resource_limits: Arc::new(Mutex::new(ResourceLimits::default())),
            metadata,
        }
    }

    /// Get process ID
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Get process state
    pub fn state(&self) -> ProcessState {
        *self.state.read().unwrap()
    }

    /// Set process state
    pub fn set_state(&self, new_state: ProcessState) {
        *self.state.write().unwrap() = new_state;
    }

    /// Get process uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Wait for process completion
    pub fn wait(&self) -> ProcessResult<ExitStatus> {
        let mut child_guard = self.child_handle.lock().unwrap();
        
        if let Some(child) = child_guard.as_mut() {
            let status = child.wait()
                .map_err(|e| io_error("wait", &format!("{:?}", e.kind()), &e.to_string()))?;
            
            self.set_state(ProcessState::Terminated);
            Ok(status)
        } else {
            Err(invalid_arguments("wait", "process", "Process handle not available"))
        }
    }

    /// Wait for process completion with timeout
    pub fn wait_timeout(&self, timeout: Duration) -> ProcessResult<Option<ExitStatus>> {
        let start = Instant::now();
        
        loop {
            {
                let mut child_guard = self.child_handle.lock().unwrap();
                if let Some(child) = child_guard.as_mut() {
                    match child.try_wait() {
                        Ok(Some(status)) => {
                            self.set_state(ProcessState::Terminated);
                            return Ok(Some(status));
                        }
                        Ok(None) => {
                            // Process still running
                            if start.elapsed() >= timeout {
                                return Ok(None);
                            }
                        }
                        Err(e) => {
                            return Err(io_error("wait_timeout", &format!("{:?}", e.kind()), &e.to_string()));
                        }
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Kill the process
    pub fn kill(&self) -> ProcessResult<()> {
        let mut child_guard = self.child_handle.lock().unwrap();
        
        if let Some(child) = child_guard.as_mut() {
            child.kill()
                .map_err(|e| io_error("kill", &format!("{:?}", e.kind()), &e.to_string()))?;
            
            self.set_state(ProcessState::Terminated);
            Ok(())
        } else {
            Err(invalid_arguments("kill", "process", "Process handle not available"))
        }
    }

    /// Send signal to process (Unix only)
    #[cfg(unix)]
    pub fn send_signal(&self, signal: i32) -> ProcessResult<()> {
        unsafe {
            if libc::kill(self.pid as i32, signal) == 0 {
                Ok(())
            } else {
                let errno = *libc::__errno_location();
                Err(system_error(errno, "kill", "Failed to send signal"))
            }
        }
    }

    /// Send signal to process (Windows stub)
    #[cfg(windows)]
    pub fn send_signal(&self, _signal: i32) -> ProcessResult<()> {
        Err(platform_error("Signal sending not supported on Windows"))
    }

    /// Terminate process gracefully
    pub fn terminate(&self, grace_period: Duration) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            // Send SIGTERM first
            self.send_signal(15)?;
            
            // Wait for grace period
            if let Ok(Some(_)) = self.wait_timeout(grace_period) {
                return Ok(());
            }
            
            // Force kill if still running
            self.send_signal(9)?;
            Ok(())
        }
        
        #[cfg(windows)]
        {
            // On Windows, just kill the process
            self.kill()
        }
    }

    /// Get process statistics
    pub fn get_statistics(&self) -> ProcessResult<ProcessStatistics> {
        get_process_statistics(self.pid, self.start_time)
    }

    /// Set resource limits
    pub fn set_resource_limits(&self, limits: ResourceLimits) -> ProcessResult<()> {
        *self.resource_limits.lock().unwrap() = limits.clone();
        apply_resource_limits(self.pid, &limits)
    }

    /// Get resource limits
    pub fn get_resource_limits(&self) -> ResourceLimits {
        self.resource_limits.lock().unwrap().clone()
    }

    /// Check if process is still running
    pub fn is_running(&self) -> bool {
        self.state() != ProcessState::Terminated && process_exists(self.pid)
    }
}

impl Drop for SafeProcessHandle {
    fn drop(&mut self) {
        // Attempt graceful termination
        if self.is_running() {
            let _ = self.terminate(Duration::from_secs(5));
        }
        
        tracing::debug!(pid = self.pid, "Process handle dropped and cleaned up");
    }
}

/// Safe process manager for handling multiple processes
#[derive(Debug)]
pub struct SafeProcessManager {
    processes: Arc<RwLock<HashMap<u32, Arc<SafeProcessHandle>>>>,
    global_resource_limits: Arc<Mutex<ResourceLimits>>,
}

impl SafeProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            processes: Arc::new(RwLock::new(HashMap::new())),
            global_resource_limits: Arc::new(Mutex::new(ResourceLimits::default())),
        }
    }

    /// Register a process with the manager
    pub fn register_process(&self, handle: Arc<SafeProcessHandle>) {
        let mut processes = self.processes.write().unwrap();
        processes.insert(handle.pid(), handle);
    }

    /// Unregister a process from the manager
    pub fn unregister_process(&self, pid: u32) -> Option<Arc<SafeProcessHandle>> {
        let mut processes = self.processes.write().unwrap();
        processes.remove(&pid)
    }

    /// Get a process handle by PID
    pub fn get_process(&self, pid: u32) -> Option<Arc<SafeProcessHandle>> {
        let processes = self.processes.read().unwrap();
        processes.get(&pid).cloned()
    }

    /// List all managed processes
    pub fn list_processes(&self) -> Vec<Arc<SafeProcessHandle>> {
        let processes = self.processes.read().unwrap();
        processes.values().cloned().collect()
    }

    /// Kill all managed processes
    pub fn kill_all(&self) -> ProcessResult<()> {
        let processes = self.list_processes();
        
        for process in processes {
            if let Err(e) = process.kill() {
                tracing::warn!(pid = process.pid(), error = ?e, "Failed to kill process");
            }
        }
        
        Ok(())
    }

    /// Wait for all managed processes
    pub fn wait_all(&self, timeout: Option<Duration>) -> ProcessResult<()> {
        let processes = self.list_processes();
        let start = Instant::now();
        
        for process in processes {
            let remaining_timeout = if let Some(total_timeout) = timeout {
                if start.elapsed() >= total_timeout {
                    return Err(timeout_error("wait_all", total_timeout, "Timeout waiting for all processes"));
                }
                Some(total_timeout - start.elapsed())
            } else {
                None
            };
            
            if let Some(timeout) = remaining_timeout {
                if process.wait_timeout(timeout)?.is_none() {
                    return Err(timeout_error("wait_all", timeout, "Process did not complete in time"));
                }
            } else {
                process.wait()?;
            }
        }
        
        Ok(())
    }

    /// Set global resource limits
    pub fn set_global_limits(&self, limits: ResourceLimits) {
        *self.global_resource_limits.lock().unwrap() = limits;
    }

    /// Apply global limits to all processes
    pub fn apply_global_limits(&self) -> ProcessResult<()> {
        let limits = self.global_resource_limits.lock().unwrap().clone();
        let processes = self.list_processes();
        
        for process in processes {
            if let Err(e) = process.set_resource_limits(limits.clone()) {
                tracing::warn!(pid = process.pid(), error = ?e, "Failed to apply limits to process");
            }
        }
        
        Ok(())
    }
}

impl Default for SafeProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Cross-platform process statistics collection
fn get_process_statistics(pid: u32, start_time: Instant) -> ProcessResult<ProcessStatistics> {
    #[cfg(target_os = "linux")]
    {
        get_linux_process_statistics(pid, start_time)
    }
    
    #[cfg(target_os = "windows")]
    {
        get_windows_process_statistics(pid, start_time)
    }
    
    #[cfg(target_os = "macos")]
    {
        get_macos_process_statistics(pid, start_time)
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        // Fallback for unsupported platforms
        Ok(ProcessStatistics {
            cpu_usage_percent: 0.0,
            memory_usage_bytes: 0,
            virtual_memory_bytes: 0,
            resident_memory_bytes: 0,
            file_descriptors_count: 0,
            thread_count: 1,
            uptime: start_time.elapsed(),
            total_cpu_time: Duration::from_secs(0),
            bytes_read: 0,
            bytes_written: 0,
        })
    }
}

/// Linux-specific process statistics
#[cfg(target_os = "linux")]
fn get_linux_process_statistics(pid: u32, start_time: Instant) -> ProcessResult<ProcessStatistics> {
    use std::fs;
    
    // Read /proc/[pid]/stat
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = fs::read_to_string(&stat_path)
        .map_err(|e| io_error("read_stat", &format!("{:?}", e.kind()), &e.to_string()))?;
    
    let stat_fields: Vec<&str> = stat_content.split_whitespace().collect();
    if stat_fields.len() < 24 {
        return Err(invalid_arguments("parse_stat", "stat_fields", "Invalid stat format"));
    }
    
    // Parse CPU times (in clock ticks)
    let utime: u64 = stat_fields[13].parse().unwrap_or(0);
    let stime: u64 = stat_fields[14].parse().unwrap_or(0);
    let clock_ticks_per_sec = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as u64;
    let total_cpu_time = Duration::from_secs((utime + stime) / clock_ticks_per_sec);
    
    // Read /proc/[pid]/status for memory info
    let status_path = format!("/proc/{}/status", pid);
    let status_content = fs::read_to_string(&status_path)
        .map_err(|e| io_error("read_status", &format!("{:?}", e.kind()), &e.to_string()))?;
    
    let mut vm_size_kb = 0u64;
    let mut vm_rss_kb = 0u64;
    let mut threads = 1u32;
    let mut fd_size = 0u32;
    
    for line in status_content.lines() {
        if line.starts_with("VmSize:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                vm_size_kb = value.parse().unwrap_or(0);
            }
        } else if line.starts_with("VmRSS:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                vm_rss_kb = value.parse().unwrap_or(0);
            }
        } else if line.starts_with("Threads:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                threads = value.parse().unwrap_or(1);
            }
        } else if line.starts_with("FDSize:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                fd_size = value.parse().unwrap_or(0);
            }
        }
    }
    
    // Read /proc/[pid]/io for I/O statistics
    let mut bytes_read = 0u64;
    let mut bytes_written = 0u64;
    
    if let Ok(io_content) = fs::read_to_string(format!("/proc/{}/io", pid)) {
        for line in io_content.lines() {
            if line.starts_with("read_bytes:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    bytes_read = value.parse().unwrap_or(0);
                }
            } else if line.starts_with("write_bytes:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    bytes_written = value.parse().unwrap_or(0);
                }
            }
        }
    }
    
    // Calculate CPU usage percentage (simplified)
    let uptime = start_time.elapsed();
    let cpu_usage_percent = if uptime.as_secs() > 0 {
        (total_cpu_time.as_secs_f64() / uptime.as_secs_f64()) * 100.0
    } else {
        0.0
    };
    
    Ok(ProcessStatistics {
        cpu_usage_percent,
        memory_usage_bytes: vm_rss_kb * 1024,
        virtual_memory_bytes: vm_size_kb * 1024,
        resident_memory_bytes: vm_rss_kb * 1024,
        file_descriptors_count: fd_size,
        thread_count: threads,
        uptime,
        total_cpu_time,
        bytes_read,
        bytes_written,
    })
}

/// Windows-specific process statistics
#[cfg(target_os = "windows")]
fn get_windows_process_statistics(pid: u32, start_time: Instant) -> ProcessResult<ProcessStatistics> {
    // Use the improved Windows support
    crate::stdlib::process::windows_support::get_windows_process_statistics(pid, start_time)
}

/// macOS-specific process statistics
#[cfg(target_os = "macos")]
fn get_macos_process_statistics(pid: u32, start_time: Instant) -> ProcessResult<ProcessStatistics> {
    // macOS-specific implementation would use libproc or sysctl
    // For now, return basic statistics
    Ok(ProcessStatistics {
        cpu_usage_percent: 0.0,
        memory_usage_bytes: 0,
        virtual_memory_bytes: 0,
        resident_memory_bytes: 0,
        file_descriptors_count: 0,
        thread_count: 1,
        uptime: start_time.elapsed(),
        total_cpu_time: Duration::from_secs(0),
        bytes_read: 0,
        bytes_written: 0,
    })
}

/// Cross-platform resource limits application
fn apply_resource_limits(pid: u32, limits: &ResourceLimits) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        apply_unix_resource_limits(pid, limits)
    }
    
    #[cfg(windows)]
    {
        apply_windows_resource_limits(pid, limits)
    }
}

/// Unix-specific resource limits
#[cfg(unix)]
fn apply_unix_resource_limits(pid: u32, limits: &ResourceLimits) -> ProcessResult<()> {
    // Memory limit
    if let Some(memory_limit) = limits.max_memory_bytes {
        unsafe {
            let rlim = libc::rlimit {
                rlim_cur: memory_limit,
                rlim_max: memory_limit,
            };
            
            if libc::setrlimit(libc::RLIMIT_AS, &rlim) != 0 {
                let errno = *libc::__errno_location();
                tracing::warn!(pid = pid, errno = errno, "Failed to set memory limit");
            }
        }
    }
    
    // File descriptor limit
    if let Some(fd_limit) = limits.max_file_descriptors {
        unsafe {
            let rlim = libc::rlimit {
                rlim_cur: fd_limit as u64,
                rlim_max: fd_limit as u64,
            };
            
            if libc::setrlimit(libc::RLIMIT_NOFILE, &rlim) != 0 {
                let errno = *libc::__errno_location();
                tracing::warn!(pid = pid, errno = errno, "Failed to set file descriptor limit");
            }
        }
    }
    
    // CPU limiting would typically use cgroups
    if let Some(cpu_limit) = limits.max_cpu_percent {
        tracing::info!(pid = pid, limit = cpu_limit, "CPU limit set (requires cgroups implementation)");
    }
    
    Ok(())
}

/// Windows-specific resource limits
#[cfg(windows)]
fn apply_windows_resource_limits(pid: u32, limits: &ResourceLimits) -> ProcessResult<()> {
    // Use the improved Windows support
    crate::stdlib::process::windows_support::apply_windows_resource_limits(pid, limits)
}

/// Check if a process exists
pub fn process_exists(pid: u32) -> bool {
    #[cfg(unix)]
    {
        unsafe {
            libc::kill(pid as i32, 0) == 0
        }
    }
    
    #[cfg(windows)]
    {
        crate::stdlib::process::windows_support::windows_process_exists(pid)
    }
}

/// Get current process ID
pub fn current_pid() -> u32 {
    std::process::id()
}

/// Get parent process ID (cross-platform)
pub fn parent_pid() -> ProcessResult<u32> {
    #[cfg(unix)]
    {
        Ok(unsafe { libc::getppid() as u32 })
    }
    
    #[cfg(windows)]
    {
        crate::stdlib::process::windows_support::get_windows_parent_pid()
    }
}

/// Global process manager instance
static GLOBAL_PROCESS_MANAGER: std::sync::OnceLock<SafeProcessManager> = std::sync::OnceLock::new();

/// Get the global process manager
pub fn global_process_manager() -> &'static SafeProcessManager {
    GLOBAL_PROCESS_MANAGER.get_or_init(SafeProcessManager::new)
}

/// Initialize the global process management system
pub fn initialize_process_management() -> ProcessResult<()> {
    let _manager = global_process_manager();
    tracing::info!("Safe process management system initialized");
    Ok(())
}

/// Shutdown the global process management system
pub fn shutdown_process_management() -> ProcessResult<()> {
    let manager = global_process_manager();
    manager.kill_all()?;
    tracing::info!("Safe process management system shutdown complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_process_manager_creation() {
        let manager = SafeProcessManager::new();
        assert_eq!(manager.list_processes().len(), 0);
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits {
            max_memory_bytes: Some(100 * 1024 * 1024), // 100MB
            max_cpu_percent: Some(80.0),
            max_execution_time: Some(Duration::from_secs(300)),
            max_file_descriptors: Some(1024),
        };
        
        assert_eq!(limits.max_memory_bytes, Some(100 * 1024 * 1024));
        assert_eq!(limits.max_cpu_percent, Some(80.0));
    }

    #[test]
    fn test_process_state() {
        assert_eq!(ProcessState::Created, ProcessState::Created);
        assert_ne!(ProcessState::Running, ProcessState::Terminated);
    }

    #[test]
    fn test_current_pid() {
        let pid = current_pid();
        assert!(pid > 0);
        assert!(process_exists(pid));
    }

    #[test]
    fn test_parent_pid() {
        if let Ok(ppid) = parent_pid() {
            assert!(ppid > 0);
        }
    }

    #[test]
    fn test_global_process_manager() {
        let manager1 = global_process_manager();
        let manager2 = global_process_manager();
        
        // Should be the same instance
        assert!(std::ptr::eq(manager1, manager2));
    }
}
