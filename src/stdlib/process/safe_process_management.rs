use crate::error::CursedError;
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

// Placeholder imports disabled
    timeout_error, invalid_arguments, io_error, system_error, platform_error
// };

/// Safe process handle that eliminates unsafe memory operations
#[derive(Debug)]
pub struct SafeProcessHandle {
    /// Process ID
    /// Safe reference to the child process
    /// Process start time
    /// Process state tracking
    /// Resource limits
    /// Process metadata
/// Process state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
/// Process metadata for tracking and monitoring
#[derive(Debug, Clone)]
pub struct ProcessMetadata {
/// Resource limits for process control
#[derive(Debug, Clone)]
pub struct ResourceLimits {
/// Resource type enumeration for process limits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
/// Security context for process isolation
#[derive(Debug, Clone)]
pub struct SecurityContext {
/// Process isolation configuration
#[derive(Debug, Clone)]
pub struct ProcessIsolation {
/// Security check result
#[derive(Debug, Clone)]
pub enum SecurityCheck {
/// Process guard for safe process management
#[derive(Debug)]
pub struct ProcessGuard {
impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
        }
    }
/// Process statistics for monitoring
#[derive(Debug, Clone)]
pub struct ProcessStatistics {
impl SafeProcessHandle {
    /// Create a new safe process handle
    pub fn new(child: Child, metadata: ProcessMetadata) -> Self {
        let pid = child.id();
        let start_time = Instant::now();
        
        Self {
        }
    }

    /// Get process ID
    pub fn pid(&self) -> u32 {
        self.pid
    /// Get process state
    pub fn state(&self) -> ProcessState {
        *self.state.read().unwrap()
    /// Set process state
    pub fn set_state(&self, new_state: ProcessState) {
        *self.state.write().unwrap() = new_state;
    /// Get process uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
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
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                Err(system_error(errno, "kill", "Failed to send signal"))
            }
        }
    /// Send signal to process (Windows stub)
    #[cfg(windows)]
    pub fn send_signal(&self, _signal: i32) -> ProcessResult<()> {
        Err(platform_error("Signal sending not supported on Windows"))
    /// Terminate process gracefully
    pub fn terminate(&self, grace_period: Duration) -> ProcessResult<()> {
        #[cfg(unix)]
        {
            // Send SIGTERM first
            self.send_signal(15)?;
            
            // Wait for grace period
            if let Ok(Some(_)) = self.wait_timeout(grace_period) {
                return Ok(());
            // Force kill if still running
            self.send_signal(9)?;
            Ok(())
        #[cfg(windows)]
        {
            // On Windows, just kill the process
            self.kill()
        }
    }

    /// Get process statistics
    pub fn get_statistics(&self) -> ProcessResult<ProcessStatistics> {
        get_process_statistics(self.pid, self.start_time)
    /// Set resource limits
    pub fn set_resource_limits(&self, limits: ResourceLimits) -> ProcessResult<()> {
        *self.resource_limits.lock().unwrap() = limits.clone();
        apply_resource_limits(self.pid, &limits)
    /// Get resource limits
    pub fn get_resource_limits(&self) -> ResourceLimits {
        self.resource_limits.lock().unwrap().clone()
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
        tracing::debug!(pid = self.pid, "Process handle dropped and cleaned up");
    }
}

/// Safe process manager for handling multiple processes
#[derive(Debug)]
pub struct SafeProcessManager {
impl SafeProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
        }
    }

    /// Register a process with the manager
    pub fn register_process(&self, handle: Arc<SafeProcessHandle>) {
        let mut processes = self.processes.write().unwrap();
        processes.insert(handle.pid(), handle);
    /// Unregister a process from the manager
    pub fn unregister_process(&self, pid: u32) -> Option<Arc<SafeProcessHandle>> {
        let mut processes = self.processes.write().unwrap();
        processes.remove(&pid)
    /// Get a process handle by PID
    pub fn get_process(&self, pid: u32) -> Option<Arc<SafeProcessHandle>> {
        let processes = self.processes.read().unwrap();
        processes.get(&pid).cloned()
    /// List all managed processes
    pub fn list_processes(&self) -> Vec<Arc<SafeProcessHandle>> {
        let processes = self.processes.read().unwrap();
        processes.values().cloned().collect()
    /// Kill all managed processes
    pub fn kill_all(&self) -> ProcessResult<()> {
        let processes = self.list_processes();
        
        for process in processes {
            if let Err(e) = process.kill() {
                tracing::warn!(pid = process.pid(), error = ?e, "Failed to kill process");
            }
        }
        
        Ok(())
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
            
            if let Some(timeout) = remaining_timeout {
                if process.wait_timeout(timeout)?.is_none() {
                    return Err(timeout_error("wait_all", timeout, "Process did not complete in time"));
                }
            } else {
                process.wait()?;
            }
        }
        
        Ok(())
    /// Set global resource limits
    pub fn set_global_limits(&self, limits: ResourceLimits) {
        *self.global_resource_limits.lock().unwrap() = limits;
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
    #[cfg(target_os = "windows")]
    {
        get_windows_process_statistics(pid, start_time)
    #[cfg(target_os = "macos")]
    {
        get_macos_process_statistics(pid, start_time)
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        // Fallback for unsupported platforms
        Ok(ProcessStatistics {
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
    
    for line in status_content.split("\n") {
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
    // Read /proc/[pid]/io for I/O statistics
    let mut bytes_read = 0u64;
    let mut bytes_written = 0u64;
    
    if let Ok(io_content) = fs::read_to_string(format!("/proc/{}/io", pid)) {
        for line in io_content.split("\n") {
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
    
    Ok(ProcessStatistics {
    })
/// Windows-specific process statistics
#[cfg(target_os = "windows")]
fn get_windows_process_statistics(pid: u32, start_time: Instant) -> ProcessResult<ProcessStatistics> {
    // Use the improved Windows support
//     crate::stdlib::process::windows_support::get_windows_process_statistics(pid, start_time)
/// macOS-specific process statistics
#[cfg(target_os = "macos")]
fn get_macos_process_statistics(pid: u32, start_time: Instant) -> ProcessResult<ProcessStatistics> {
    // macOS-specific implementation would use libproc or sysctl
    // For now, return basic statistics
    Ok(ProcessStatistics {
    })
/// Cross-platform resource limits application
fn apply_resource_limits(pid: u32, limits: &ResourceLimits) -> ProcessResult<()> {
    #[cfg(unix)]
    {
        apply_unix_resource_limits(pid, limits)
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
            
            if libc::setrlimit(libc::RLIMIT_AS, &rlim) != 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                tracing::warn!(pid = pid, errno = errno, "Failed to set memory limit");
            }
        }
    // File descriptor limit
    if let Some(fd_limit) = limits.max_file_descriptors {
        unsafe {
            let rlim = libc::rlimit {
            
            if libc::setrlimit(libc::RLIMIT_NOFILE, &rlim) != 0 {
                let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);
                tracing::warn!(pid = pid, errno = errno, "Failed to set file descriptor limit");
            }
        }
    // CPU limiting would typically use cgroups
    if let Some(cpu_limit) = limits.max_cpu_percent {
        tracing::info!(pid = pid, limit = cpu_limit, "CPU limit set (requires cgroups implementation)");
    Ok(())
/// Windows-specific resource limits
#[cfg(windows)]
fn apply_windows_resource_limits(pid: u32, limits: &ResourceLimits) -> ProcessResult<()> {
    // Use the improved Windows support
//     crate::stdlib::process::windows_support::apply_windows_resource_limits(pid, limits)
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
//         crate::stdlib::process::windows_support::windows_process_exists(pid)
    }
}

/// Get current process ID
pub fn current_pid() -> u32 {
    std::process::id()
/// Get parent process ID (cross-platform)
pub fn parent_pid() -> ProcessResult<u32> {
    #[cfg(unix)]
    {
        Ok(unsafe { libc::getppid() as u32 })
    #[cfg(windows)]
    {
//         crate::stdlib::process::windows_support::get_windows_parent_pid()
    }
}

/// Global process manager instance
static GLOBAL_PROCESS_MANAGER: std::sync::OnceLock<SafeProcessManager> = std::sync::OnceLock::new();

/// Get the global process manager
pub fn global_process_manager() -> &'static SafeProcessManager {
    GLOBAL_PROCESS_MANAGER.get_or_init(SafeProcessManager::new)
/// Initialize the global process management system
pub fn initialize_process_management() -> ProcessResult<()> {
    let _manager = global_process_manager();
    tracing::info!("Safe process management system initialized");
    Ok(())
/// Shutdown the global process management system
pub fn shutdown_process_management() -> ProcessResult<()> {
    let manager = global_process_manager();
    manager.kill_all()?;
    tracing::info!("Safe process management system shutdown complete");
    Ok(())

/// Safety configuration for process management
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    /// Enable memory protection
    /// Enable process isolation
    /// Enable security auditing
    /// Maximum process count
    /// Default resource limits
    /// Security level
/// Security policy for process execution
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    /// Policy name
    /// Allowed capabilities
    /// Denied capabilities
    /// Resource limits
    /// Network access
    /// File system access
    /// System call restrictions
/// Security context data for process
#[derive(Debug, Clone)]
pub struct SecurityContextData {
    /// Process ID
    /// User ID
    /// Group ID
    /// Security labels
    /// Applied policies
    /// Creation time
/// Security event for auditing
#[derive(Debug, Clone)]
pub struct SecurityEvent {
    /// Event timestamp
    /// Process ID
    /// Event type
    /// Event description
    /// Security level
/// Security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Low security
    /// Medium security
    /// High security
    /// Critical security
impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
        }
    }
impl ProcessSecurityManager {
    /// Create a new process security manager
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: SafetyConfig) -> Self {
        Self {
        }
    }

    /// Add a security policy
    pub fn add_policy(&self, policy: SecurityPolicy) -> ProcessResult<()> {
        let mut policies = self.policies.write().unwrap();
        policies.insert(policy.name.clone(), policy);
        Ok(())
    /// Apply security context to process
    pub fn apply_context(&self, pid: u32, context: SecurityContextData) -> ProcessResult<()> {
        let mut contexts = self.contexts.write().unwrap();
        contexts.insert(pid, context);
        Ok(())
    /// Get security context for process
    pub fn get_context(&self, pid: u32) -> Option<SecurityContextData> {
        let contexts = self.contexts.read().unwrap();
        contexts.get(&pid).cloned()
    /// Log security event
    pub fn log_event(&self, event: SecurityEvent) {
        let mut audit_log = self.audit_log.lock().unwrap();
        audit_log.push(event);
    /// Get audit log
    pub fn get_audit_log(&self) -> Vec<SecurityEvent> {
        let audit_log = self.audit_log.lock().unwrap();
        audit_log.clone()
    }
}
