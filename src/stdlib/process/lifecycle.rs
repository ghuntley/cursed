/// RuntimeProcessInfo lifecycle management for CURSED
/// 
/// This module provides comprehensive process lifecycle management including
/// spawning, waiting, termination, and cleanup operations with robust error handling.

use std::collections::HashMap;
use std::process::{Child, ExitStatus};
use std::sync::{Arc, Mutex, Weak};
use std::thread;
use std::time::{Duration, Instant};

use crate::stdlib::process::error::{
    ProcessError, ProcessResult, timeout_error, execution_failed, invalid_state, system_error
};
use crate::stdlib::process::core::{ProcessConfig};
use crate::stdlib::process::info::{ProcessInfo as StdProcessInfo, ProcessState as StdProcessState};
use crate::runtime::process::{ProcessInfo as RuntimeProcessInfo, ProcessStatus as RuntimeProcessStatus};


/// RuntimeProcessInfo lifecycle manager
#[derive(Debug)]
pub struct ProcessLifecycleManager {
    /// Active processes being managed
    active_processes: Arc<Mutex<HashMap<u32, ManagedProcess>>>,
    /// Maximum number of concurrent processes
    max_concurrent: usize,
    /// Default timeout for process operations
    default_timeout: Duration,
    /// Cleanup thread handle
    cleanup_thread: Option<thread::JoinHandle<()>>,
    /// Shutdown flag
    shutdown_flag: Arc<Mutex<bool>>,
}

/// Managed process wrapper
#[derive(Debug)]
struct ManagedProcess {
    /// RuntimeProcessInfo instance
    process: Process,
    /// Spawn time
    spawn_time: Instant,
    /// Expected termination time (if timeout set)
    timeout_at: Option<Instant>,
    /// Lifecycle state
    state: ProcessLifecycleState,
    /// Parent manager reference (weak to avoid cycles)
    manager: Weak<Mutex<HashMap<u32, ManagedProcess>>>,
}

/// RuntimeProcessInfo lifecycle states
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessLifecycleState {
    /// RuntimeProcessInfo is starting up
    Starting,
    /// RuntimeProcessInfo is running normally
    Running,
    /// RuntimeProcessInfo is being terminated
    Terminating,
    /// RuntimeProcessInfo has completed successfully
    Completed(ExitStatus),
    /// RuntimeProcessInfo failed during execution
    Failed(ProcessError),
    /// RuntimeProcessInfo was terminated due to timeout
    TimedOut,
    /// RuntimeProcessInfo was forcibly killed
    Killed,
}

impl ProcessLifecycleManager {
    /// Create a new process lifecycle manager
    pub fn new() -> Self {
        Self::with_config(100, Duration::from_secs(300))
    }

    /// Create a new process lifecycle manager with custom configuration
    pub fn with_config(max_concurrent: usize, default_timeout: Duration) -> Self {
        let active_processes = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_flag = Arc::new(Mutex::new(false));
        
        let manager = Self {
            active_processes: active_processes.clone(),
            max_concurrent,
            default_timeout,
            cleanup_thread: None,
            shutdown_flag: shutdown_flag.clone(),
        };
        
        // Start cleanup thread
        let cleanup_active = active_processes.clone();
        let cleanup_shutdown = shutdown_flag.clone();
        let cleanup_handle = thread::spawn(move || {
            Self::cleanup_loop(cleanup_active, cleanup_shutdown);
        });
        
        Self {
            cleanup_thread: Some(cleanup_handle),
            ..manager
        }
    }

    /// Spawn a new managed process
    pub fn spawn(&self, config: ProcessConfig) -> ProcessResult<u32> {
        // Check concurrent process limit
        {
            let active = self.active_processes.lock().unwrap();
            if active.len() >= self.max_concurrent {
                return Err(system_error(
                    "Maximum concurrent processes reached",
                    Some("Process limit exceeded".to_string())
                ));
            }
        }

        // Spawn the process
        let mut process = RuntimeProcessInfo::spawn(config)?;
        let pid = process.id();
        
        // Calculate timeout
        let timeout_at = process.config().timeout
            .or(Some(self.default_timeout))
            .map(|timeout| Instant::now() + timeout);

        // Create managed process
        let managed = ManagedProcess {
            process,
            spawn_time: Instant::now(),
            timeout_at,
            state: ProcessLifecycleState::Starting,
            manager: Arc::downgrade(&self.active_processes),
        };

        // Add to active processes
        {
            let mut active = self.active_processes.lock().unwrap();
            active.insert(pid, managed);
        }

        Ok(pid)
    }

    /// Wait for a process to complete
    pub fn wait(&self, pid: u32) -> ProcessResult<ExitStatus> {
        self.wait_with_timeout(pid, None)
    }

    /// Wait for a process to complete with timeout
    pub fn wait_with_timeout(&self, pid: u32, timeout: Option<Duration>) -> ProcessResult<ExitStatus> {
        let start_time = Instant::now();
        let deadline = timeout.map(|t| start_time + t);

        loop {
            // Check if process exists and get its state
            let state = {
                let active = self.active_processes.lock().unwrap();
                match active.get(&pid) {
                    Some(managed) => managed.state.clone(),
                    None => return Err(invalid_state(&format!("Process {} not found", pid))),
                }
            };

            // Check final states
            match state {
                ProcessLifecycleState::Completed(exit_status) => return Ok(exit_status),
                ProcessLifecycleState::Failed(error) => return Err(error),
                ProcessLifecycleState::TimedOut => {
                    return Err(timeout_error(&format!("Process {} timed out", pid)));
                }
                ProcessLifecycleState::Killed => {
                    return Err(execution_failed(&format!("Process {} was killed", pid)));
                }
                _ => {
                    // RuntimeProcessInfo still running, check our timeout
                    if let Some(deadline) = deadline {
                        if Instant::now() >= deadline {
                            // Terminate the process
                            self.terminate(pid, Some(Duration::from_secs(5)))?;
                            return Err(timeout_error(&format!("Wait timeout for process {}", pid)));
                        }
                    }
                    
                    // Sleep briefly before checking again
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }

    /// Terminate a process gracefully
    pub fn terminate(&self, pid: u32, grace_period: Option<Duration>) -> ProcessResult<()> {
        let grace_period = grace_period.unwrap_or(Duration::from_secs(10));
        
        // Mark process as terminating and get the process
        let mut process = {
            let mut active = self.active_processes.lock().unwrap();
            match active.get_mut(&pid) {
                Some(managed) => {
                    managed.state = ProcessLifecycleState::Terminating;
                    // Clone process for termination (we can't move it out of the HashMap)
                    managed.process.clone()
                }
                None => return Err(invalid_state(&format!("Process {} not found", pid))),
            }
        };

        // Try graceful termination first
        if let Err(_) = process.terminate() {
            // Graceful termination failed, try kill
            process.kill()?;
        }

        // Wait for termination with grace period
        let start = Instant::now();
        while start.elapsed() < grace_period {
            if !process.is_running()? {
                self.mark_process_killed(pid);
                return Ok(());
            }
            thread::sleep(Duration::from_millis(100));
        }

        // Force kill if still running
        process.kill()?;
        self.mark_process_killed(pid);
        Ok(())
    }

    /// Kill a process immediately
    pub fn kill(&self, pid: u32) -> ProcessResult<()> {
        let mut process = {
            let mut active = self.active_processes.lock().unwrap();
            match active.get_mut(&pid) {
                Some(managed) => {
                    managed.state = ProcessLifecycleState::Terminating;
                    managed.process.clone()
                }
                None => return Err(invalid_state(&format!("Process {} not found", pid))),
            }
        };

        process.kill()?;
        self.mark_process_killed(pid);
        Ok(())
    }

    /// Get process information
    pub fn get_process_info(&self, pid: u32) -> ProcessResult<ProcessInfo> {
        let active = self.active_processes.lock().unwrap();
        match active.get(&pid) {
            Some(managed) => managed.process.get_info(),
            None => Err(invalid_state(&format!("Process {} not found", pid))),
        }
    }

    /// Get process lifecycle state
    pub fn get_process_state(&self, pid: u32) -> ProcessResult<ProcessLifecycleState> {
        let active = self.active_processes.lock().unwrap();
        match active.get(&pid) {
            Some(managed) => Ok(managed.state.clone()),
            None => Err(invalid_state(&format!("Process {} not found", pid))),
        }
    }

    /// List all active processes
    pub fn list_active_processes(&self) -> Vec<u32> {
        let active = self.active_processes.lock().unwrap();
        active.keys().copied().collect()
    }

    /// Get the number of active processes
    pub fn active_process_count(&self) -> usize {
        let active = self.active_processes.lock().unwrap();
        active.len()
    }

    /// Terminate all active processes
    pub fn terminate_all(&self, grace_period: Option<Duration>) -> ProcessResult<()> {
        let pids: Vec<u32> = {
            let active = self.active_processes.lock().unwrap();
            active.keys().copied().collect()
        };

        for pid in pids {
            if let Err(e) = self.terminate(pid, grace_period) {
                eprintln!("Failed to terminate process {}: {}", pid, e);
            }
        }

        Ok(())
    }

    /// Shutdown the lifecycle manager
    pub fn shutdown(&mut self) -> ProcessResult<()> {
        // Set shutdown flag
        {
            let mut shutdown = self.shutdown_flag.lock().unwrap();
            *shutdown = true;
        }

        // Terminate all processes
        self.terminate_all(Some(Duration::from_secs(5)))?;

        // Wait for cleanup thread to finish
        if let Some(handle) = self.cleanup_thread.take() {
            handle.join().map_err(|_| {
                system_error("Failed to join cleanup thread", None)
            })?;
        }

        Ok(())
    }

    /// Mark a process as killed
    fn mark_process_killed(&self, pid: u32) {
        let mut active = self.active_processes.lock().unwrap();
        if let Some(managed) = active.get_mut(&pid) {
            managed.state = ProcessLifecycleState::Killed;
        }
    }

    /// Cleanup loop for background thread
    fn cleanup_loop(
        active_processes: Arc<Mutex<HashMap<u32, ManagedProcess>>>,
        shutdown_flag: Arc<Mutex<bool>>,
    ) {
        while !*shutdown_flag.lock().unwrap() {
            let pids_to_check: Vec<u32> = {
                let active = active_processes.lock().unwrap();
                active.keys().copied().collect()
            };

            for pid in pids_to_check {
                Self::check_process_status(&active_processes, pid);
            }

            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Check and update process status
    fn check_process_status(
        active_processes: &Arc<Mutex<HashMap<u32, ManagedProcess>>>,
        pid: u32,
    ) {
        let should_remove = {
            let mut active = active_processes.lock().unwrap();
            match active.get_mut(&pid) {
                Some(managed) => {
                    // Check if process is still running
                    match managed.process.try_wait() {
                        Ok(Some(exit_status)) => {
                            // RuntimeProcessInfo completed
                            managed.state = ProcessLifecycleState::Completed(exit_status);
                            true // Remove from active list
                        }
                        Ok(None) => {
                            // RuntimeProcessInfo still running, check timeout
                            if let Some(timeout_at) = managed.timeout_at {
                                if Instant::now() >= timeout_at {
                                    // RuntimeProcessInfo timed out
                                    managed.state = ProcessLifecycleState::TimedOut;
                                    // Try to kill the process
                                    if let Err(_) = managed.process.kill() {
                                        // Kill failed, but mark as timed out anyway
                                    }
                                    true // Remove from active list
                                } else {
                                    // Update state to running if it was starting
                                    if managed.state == ProcessLifecycleState::Starting {
                                        managed.state = ProcessLifecycleState::Running;
                                    }
                                    false // Keep in active list
                                }
                            } else {
                                // Update state to running if it was starting
                                if managed.state == ProcessLifecycleState::Starting {
                                    managed.state = ProcessLifecycleState::Running;
                                }
                                false // Keep in active list
                            }
                        }
                        Err(e) => {
                            // Error checking process status
                            managed.state = ProcessLifecycleState::Failed(
                                system_error(&format!("Failed to check process status: {}", e), None)
                            );
                            true // Remove from active list
                        }
                    }
                }
                None => false, // RuntimeProcessInfo not found, nothing to do
            }
        };

        if should_remove {
            let mut active = active_processes.lock().unwrap();
            active.remove(&pid);
        }
    }
}

impl Default for ProcessLifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ProcessLifecycleManager {
    fn drop(&mut self) {
        if let Err(e) = self.shutdown() {
            eprintln!("Error during ProcessLifecycleManager shutdown: {}", e);
        }
    }
}

/// RuntimeProcessInfo lifecycle statistics
#[derive(Debug, Clone)]
pub struct ProcessLifecycleStats {
    /// Total processes spawned
    pub total_spawned: u64,
    /// Currently active processes
    pub active_count: usize,
    /// Processes completed successfully
    pub completed_count: u64,
    /// Processes that failed
    pub failed_count: u64,
    /// Processes that timed out
    pub timeout_count: u64,
    /// Processes that were killed
    pub killed_count: u64,
    /// Average process runtime
    pub average_runtime: Duration,
    /// Maximum concurrent processes reached
    pub max_concurrent_reached: usize,
}

impl ProcessLifecycleManager {
    /// Get lifecycle statistics
    pub fn get_statistics(&self) -> ProcessLifecycleStats {
        let active = self.active_processes.lock().unwrap();
        
        // For a complete implementation, we would need to track these statistics
        // Here we provide a basic implementation
        ProcessLifecycleStats {
            total_spawned: 0, // Would be tracked in a production implementation
            active_count: active.len(),
            completed_count: 0,
            failed_count: 0,
            timeout_count: 0,
            killed_count: 0,
            average_runtime: Duration::from_secs(0),
            max_concurrent_reached: active.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::process::core::{ProcessConfig};
    use crate::stdlib::process::communication::{ProcessIo};
    use std::time::Duration;

    #[test]
    fn test_lifecycle_manager_creation() {
        let manager = ProcessLifecycleManager::new();
        assert_eq!(manager.active_process_count(), 0);
    }

    #[test]
    fn test_lifecycle_manager_with_config() {
        let manager = ProcessLifecycleManager::with_config(50, Duration::from_secs(60));
        assert_eq!(manager.max_concurrent, 50);
        assert_eq!(manager.default_timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_process_spawning() {
        let manager = ProcessLifecycleManager::new();
        
        // Create a simple echo command
        let config = ProcessConfig::new("echo")
            .args(&["test"])
            .timeout(Duration::from_secs(5));
        
        let pid = manager.spawn(config).unwrap();
        assert!(pid > 0);
        assert_eq!(manager.active_process_count(), 1);
        
        // Wait for process to complete
        let exit_status = manager.wait(pid).unwrap();
        assert!(exit_status.success());
    }

    #[test]
    fn test_process_timeout() {
        let manager = ProcessLifecycleManager::new();
        
        // Create a long-running command that should timeout
        #[cfg(unix)]
        let config = ProcessConfig::new("sleep")
            .args(&["10"])
            .timeout(Duration::from_millis(100));
        
        #[cfg(windows)]
        let config = ProcessConfig::new("timeout")
            .args(&["10"])
            .timeout(Duration::from_millis(100));
        
        let pid = manager.spawn(config).unwrap();
        
        // Wait with a short timeout
        let result = manager.wait_with_timeout(pid, Some(Duration::from_millis(200)));
        assert!(result.is_err());
    }

    #[test]
    fn test_process_termination() {
        let manager = ProcessLifecycleManager::new();
        
        // Create a long-running command
        #[cfg(unix)]
        let config = ProcessConfig::new("sleep").args(&["60"]);
        
        #[cfg(windows)]
        let config = ProcessConfig::new("timeout").args(&["60"]);
        
        let pid = manager.spawn(config).unwrap();
        
        // Terminate the process
        assert!(manager.terminate(pid, Some(Duration::from_secs(1))).is_ok());
        
        // Check that process state is updated
        thread::sleep(Duration::from_millis(500));
        let state = manager.get_process_state(pid);
        assert!(state.is_err() || matches!(state.unwrap(), ProcessLifecycleState::Killed));
    }

    #[test]
    fn test_process_listing() {
        let manager = ProcessLifecycleManager::new();
        
        // Spawn multiple processes
        let config1 = ProcessConfig::new("echo").args(&["test1"]);
        let config2 = ProcessConfig::new("echo").args(&["test2"]);
        
        let pid1 = manager.spawn(config1).unwrap();
        let pid2 = manager.spawn(config2).unwrap();
        
        let active_pids = manager.list_active_processes();
        assert!(active_pids.contains(&pid1));
        assert!(active_pids.contains(&pid2));
        assert_eq!(manager.active_process_count(), 2);
    }

    #[test]
    fn test_lifecycle_states() {
        let manager = ProcessLifecycleManager::new();
        
        let config = ProcessConfig::new("echo").args(&["test"]);
        let pid = manager.spawn(config).unwrap();
        
        // Initially should be Starting or Running
        let initial_state = manager.get_process_state(pid).unwrap();
        assert!(matches!(initial_state, 
            ProcessLifecycleState::Starting | ProcessLifecycleState::Running
        ));
        
        // Wait for completion
        let _ = manager.wait(pid);
        
        // Should eventually be completed (or removed from active list)
        let final_state = manager.get_process_state(pid);
        assert!(final_state.is_err() || matches!(final_state.unwrap(), ProcessLifecycleState::Completed(_)));
    }

    #[test]
    fn test_statistics() {
        let manager = ProcessLifecycleManager::new();
        let stats = manager.get_statistics();
        
        assert_eq!(stats.active_count, 0);
        assert!(stats.max_concurrent_reached >= 0);
    }

    #[test]
    fn test_concurrent_limit() {
        let manager = ProcessLifecycleManager::with_config(1, Duration::from_secs(60));
        
        // First process should succeed
        let config1 = ProcessConfig::new("echo").args(&["test1"]);
        let _pid1 = manager.spawn(config1).unwrap();
        
        // Second process should fail due to limit
        let config2 = ProcessConfig::new("echo").args(&["test2"]);
        let result = manager.spawn(config2);
        assert!(result.is_err());
    }
}
