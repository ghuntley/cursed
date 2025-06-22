/// Process management integration and high-level APIs
/// 
/// This module provides unified access to all process management capabilities,
/// integrating monitoring, control, lifecycle management, and execution APIs.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::core::{ProcessConfig};
use crate::stdlib::process::lifecycle::{ProcessLifecycleManager};
use crate::stdlib::process::monitoring::{ProcessMonitor, HealthCheckConfig, HealthStatus, PerformanceMetrics, ProcessStats};
use crate::stdlib::process::control::{ProcessControl};
use crate::stdlib::process::signals::{SignalType as Signal};
use crate::stdlib::process::real_monitoring::{start_global_monitoring, stop_global_monitoring, get_current_process_state};
use crate::runtime::process::{ProcessOutput as RuntimeProcessOutput};

// Type aliases
type ProcessOutput = RuntimeProcessOutput;

/// Unified process manager that integrates all process management capabilities
#[derive(Debug)]
pub struct UnifiedProcessManager {
    /// Lifecycle manager for process tracking
    lifecycle_manager: ProcessLifecycleManager,
    /// Process monitor for health checks
    process_monitor: ProcessMonitor,
    /// Active processes registry
    active_processes: Arc<Mutex<HashMap<u32, ProcessInfo>>>,
}

/// Comprehensive process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Process command line
    pub command: String,
    /// Process start time
    pub start_time: std::time::Instant,
    /// Process status
    pub status: ProcessStatus,
    /// Health status
    pub health: HealthStatus,
    /// Resource usage
    pub resources: Option<ProcessStats>,
}

/// Process status enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    Starting,
    Running,
    Paused,
    Stopping,
    Stopped,
    Failed(String),
}

impl UnifiedProcessManager {
    /// Create a new unified process manager
    pub fn new() -> ProcessResult<Self> {
        let lifecycle_manager = ProcessLifecycleManager::new();
        let process_monitor = ProcessMonitor::new(HealthCheckConfig::default());
        
        // Start global monitoring
        start_global_monitoring();
        
        Ok(Self {
            lifecycle_manager,
            process_monitor,
            active_processes: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Spawn a process with comprehensive tracking
    #[tracing::instrument(skip(self))]
    pub fn spawn_tracked<S: AsRef<str>>(&mut self, command: S, args: &[&str]) -> ProcessResult<u32> {
        let command_str = command.as_ref();
        tracing::info!(command = command_str, args = ?args, "Spawning tracked process");
        
        // Create process configuration
        let config = ProcessConfig::new(command_str)
            .args(args.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        
        // Spawn through lifecycle manager
        let pid = self.lifecycle_manager.spawn(config)?;
        
        // Add to monitoring
        self.process_monitor.add_process(pid)?;
        
        // Register in our active processes
        let process_info = ProcessInfo {
            pid,
            command: format!("{} {}", command_str, args.join(" ")),
            start_time: std::time::Instant::now(),
            status: ProcessStatus::Starting,
            health: HealthStatus::Unknown,
            resources: None,
        };
        
        {
            let mut active = self.active_processes.lock()
                .map_err(|_| ProcessError::SystemError(-1, "Failed to lock active processes".to_string()))?;
            active.insert(pid, process_info);
        }
        
        tracing::info!(pid = pid, "Process spawned and tracked successfully");
        Ok(pid)
    }

    /// Get comprehensive process information
    pub fn get_process_info(&self, pid: u32) -> ProcessResult<ProcessInfo> {
        let active = self.active_processes.lock()
            .map_err(|_| ProcessError::SystemError(-1, "Failed to lock active processes".to_string()))?;
        
        if let Some(mut info) = active.get(&pid).cloned() {
            // Update with latest monitoring data
            if let Ok(health) = self.process_monitor.get_health_status(pid) {
                info.health = health;
            }
            
            // Get real-time process state
            if let Ok(real_state) = get_current_process_state(pid) {
                info.status = if real_state.is_running {
                    ProcessStatus::Running
                } else {
                    ProcessStatus::Stopped
                };
            }
            
            Ok(info)
        } else {
            Err(ProcessError::ProcessNotFound(pid))
        }
    }

    /// List all active processes
    pub fn list_active(&self) -> ProcessResult<Vec<ProcessInfo>> {
        let active = self.active_processes.lock()
            .map_err(|_| ProcessError::SystemError(-1, "Failed to lock active processes".to_string()))?;
        
        Ok(active.values().cloned().collect())
    }

    /// Terminate a process gracefully
    #[tracing::instrument(skip(self))]
    pub fn terminate_process(&mut self, pid: u32) -> ProcessResult<()> {
        tracing::info!(pid = pid, "Terminating process");
        
        // Update status
        self.update_process_status(pid, ProcessStatus::Stopping)?;
        
        // Terminate through lifecycle manager
        self.lifecycle_manager.terminate(pid, Some(Duration::from_secs(10)))?;
        
        // Remove from monitoring
        self.process_monitor.remove_process(pid)?;
        
        // Update status
        self.update_process_status(pid, ProcessStatus::Stopped)?;
        
        tracing::info!(pid = pid, "Process terminated successfully");
        Ok(())
    }

    /// Kill a process immediately
    #[tracing::instrument(skip(self))]
    pub fn kill_process(&mut self, pid: u32) -> ProcessResult<()> {
        tracing::warn!(pid = pid, "Killing process immediately");
        
        // Update status
        self.update_process_status(pid, ProcessStatus::Stopping)?;
        
        // Kill through lifecycle manager
        self.lifecycle_manager.kill(pid)?;
        
        // Remove from monitoring
        self.process_monitor.remove_process(pid)?;
        
        // Update status
        self.update_process_status(pid, ProcessStatus::Stopped)?;
        
        tracing::info!(pid = pid, "Process killed successfully");
        Ok(())
    }

    /// Get health summary for all processes
    pub fn get_health_summary(&self) -> ProcessResult<HashMap<u32, HealthStatus>> {
        self.process_monitor.get_health_summary()
    }

    /// Get performance metrics for a process
    pub fn get_performance_metrics(&self, pid: u32) -> ProcessResult<Vec<PerformanceMetrics>> {
        self.process_monitor.get_performance_history(pid)
    }

    /// Send signal to process
    pub fn send_signal(&self, pid: u32, signal: Signal) -> ProcessResult<()> {
        ProcessControl::send_signal(pid, signal)
    }

    /// Set process priority
    pub fn set_priority(&self, pid: u32, priority: Priority) -> ProcessResult<()> {
        ProcessControl::set_priority(pid, priority)
    }

    /// Start monitoring all processes
    pub fn start_monitoring(&mut self) -> ProcessResult<()> {
        self.process_monitor.start()
    }

    /// Stop monitoring
    pub fn stop_monitoring(&mut self) -> ProcessResult<()> {
        self.process_monitor.stop()
    }

    /// Cleanup finished processes
    pub fn cleanup_finished(&mut self) -> ProcessResult<usize> {
        let mut cleaned_count = 0;
        let pids_to_check: Vec<u32> = {
            let active = self.active_processes.lock()
                .map_err(|_| ProcessError::SystemError(-1, "Failed to lock active processes".to_string()))?;
            active.keys().copied().collect()
        };

        for pid in pids_to_check {
            if let Ok(real_state) = get_current_process_state(pid) {
                if !real_state.is_running {
                    // Process has finished, remove from tracking
                    {
                        let mut active = self.active_processes.lock()
                            .map_err(|_| ProcessError::SystemError(-1, "Failed to lock active processes".to_string()))?;
                        active.remove(&pid);
                    }
                    
                    // Remove from monitoring
                    let _ = self.process_monitor.remove_process(pid);
                    
                    cleaned_count += 1;
                    tracing::debug!(pid = pid, "Cleaned up finished process");
                }
            }
        }

        if cleaned_count > 0 {
            tracing::info!(count = cleaned_count, "Cleaned up finished processes");
        }
        
        Ok(cleaned_count)
    }

    /// Update process status
    fn update_process_status(&self, pid: u32, status: ProcessStatus) -> ProcessResult<()> {
        let mut active = self.active_processes.lock()
            .map_err(|_| ProcessError::SystemError(-1, "Failed to lock active processes".to_string()))?;
        
        if let Some(process_info) = active.get_mut(&pid) {
            process_info.status = status;
        }
        
        Ok(())
    }

    /// Shutdown the manager
    pub fn shutdown(&mut self) -> ProcessResult<()> {
        tracing::info!("Shutting down unified process manager");
        
        // Stop monitoring
        let _ = self.stop_monitoring();
        
        // Terminate all active processes
        let pids: Vec<u32> = {
            let active = self.active_processes.lock()
                .map_err(|_| ProcessError::SystemError(-1, "Failed to lock active processes".to_string()))?;
            active.keys().copied().collect()
        };
        
        for pid in pids {
            let _ = self.terminate_process(pid);
        }
        
        // Shutdown lifecycle manager
        self.lifecycle_manager.shutdown()?;
        
        // Stop global monitoring
        stop_global_monitoring();
        
        tracing::info!("Unified process manager shutdown complete");
        Ok(())
    }
}

impl Drop for UnifiedProcessManager {
    fn drop(&mut self) {
        if let Err(e) = self.shutdown() {
            tracing::error!(error = ?e, "Error during UnifiedProcessManager shutdown");
        }
    }
}

/// High-level process execution functions
pub mod quick_exec {
    use super::*;
    
    /// Execute a command quickly and return output
    #[tracing::instrument]
    pub fn exec<S: AsRef<str>>(command: S) -> ProcessResult<ProcessOutput> {
        let config = ProcessConfig::new(command);
        crate::stdlib::process::core::run_command(config)
    }
    
    /// Execute a command with arguments
    #[tracing::instrument]
    pub fn exec_with_args<S: AsRef<str>>(command: S, args: &[&str]) -> ProcessResult<ProcessOutput> {
        let mut config = ProcessConfig::new(command);
        for arg in args {
            config = config.arg(arg);
        }
        crate::stdlib::process::core::run_command(config)
    }
    
    /// Execute a command with timeout
    #[tracing::instrument]
    pub fn exec_timeout<S: AsRef<str>>(command: S, timeout: Duration) -> ProcessResult<ProcessOutput> {
        let config = ProcessConfig::new(command).timeout(timeout);
        crate::stdlib::process::core::run_command_timeout(config, timeout)
    }
    
    /// Execute a shell command
    #[tracing::instrument]
    pub fn shell<S: AsRef<str>>(command: S) -> ProcessResult<ProcessOutput> {
        let shell = if cfg!(windows) { "cmd" } else { "sh" };
        let flag = if cfg!(windows) { "/C" } else { "-c" };
        
        exec_with_args(shell, &[flag, command.as_ref()])
    }
}

/// Process group management
pub mod process_groups {
    use super::*;
    
    /// Process group manager
    #[derive(Debug)]
    pub struct ProcessGroupManager {
        groups: Arc<Mutex<HashMap<String, Vec<u32>>>>,
        manager: Arc<Mutex<UnifiedProcessManager>>,
    }
    
    impl ProcessGroupManager {
        /// Create a new process group manager
        pub fn new() -> ProcessResult<Self> {
            Ok(Self {
                groups: Arc::new(Mutex::new(HashMap::new())),
                manager: Arc::new(Mutex::new(UnifiedProcessManager::new()?)),
            })
        }
        
        /// Create a new process group
        pub fn create_group<S: AsRef<str>>(&self, group_name: S) -> ProcessResult<()> {
            let mut groups = self.groups.lock()
                .map_err(|_| ProcessError::SystemError(-1, "Failed to lock groups".to_string()))?;
            
            groups.insert(group_name.as_ref().to_string(), Vec::new());
            Ok(())
        }
        
        /// Add process to group
        pub fn add_to_group<S: AsRef<str>>(&self, group_name: S, pid: u32) -> ProcessResult<()> {
            let mut groups = self.groups.lock()
                .map_err(|_| ProcessError::SystemError(-1, "Failed to lock groups".to_string()))?;
            
            let group = groups.entry(group_name.as_ref().to_string()).or_insert_with(Vec::new);
            group.push(pid);
            Ok(())
        }
        
        /// Terminate entire group
        pub fn terminate_group<S: AsRef<str>>(&self, group_name: S) -> ProcessResult<()> {
            let pids = {
                let groups = self.groups.lock()
                    .map_err(|_| ProcessError::SystemError(-1, "Failed to lock groups".to_string()))?;
                
                groups.get(group_name.as_ref()).cloned().unwrap_or_default()
            };
            
            let mut manager = self.manager.lock()
                .map_err(|_| ProcessError::SystemError(-1, "Failed to lock manager".to_string()))?;
            
            for pid in pids {
                let _ = manager.terminate_process(pid);
            }
            
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unified_manager_creation() {
        let manager = UnifiedProcessManager::new();
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_quick_exec() {
        #[cfg(unix)]
        {
            let result = quick_exec::exec("echo test");
            assert!(result.is_ok());
            let output = result.unwrap();
            assert_eq!(output.stdout_lossy().trim(), "test");
        }
        
        #[cfg(windows)]
        {
            let result = quick_exec::exec_with_args("echo", &["test"]);
            assert!(result.is_ok());
        }
    }
    
    #[test]
    fn test_process_group_manager() {
        let manager = process_groups::ProcessGroupManager::new();
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        assert!(manager.create_group("test_group").is_ok());
    }
}
