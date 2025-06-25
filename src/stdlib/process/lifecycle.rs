use crate::error::CursedError;
/// RuntimeProcessInfo lifecycle management for CURSED
/// 
/// This module provides comprehensive process lifecycle management including
/// spawning, waiting, termination, and cleanup operations with robust error handling.

use std::collections::HashMap;
use std::process::{Child, ExitStatus};
use std::sync::{Arc, Mutex, Weak};
use std::thread;
use std::time::{Duration, Instant};

// Placeholder imports disabled
    ProcessError, ProcessResult, timeout_error, execution_failed, invalid_state, system_error
// };

// use crate::stdlib::process::core::{ProcessConfig};
// use crate::stdlib::process::info::{ProcessInfo as StdProcessInfo, ProcessState as StdProcessState};
use crate::runtime::process::{ProcessInfo as RuntimeProcessInfo, ProcessStatus as RuntimeProcessStatus};

/// Process lifecycle events
#[derive(Debug, Clone)]
pub enum LifecycleEvent {
    /// Process spawned successfully
    /// Process started execution
    /// Process is running normally
    /// Process terminated normally
    /// Process was killed
    /// Process failed to start
    /// Process timed out
    /// Process is being cleaned up
impl LifecycleEvent {
    /// Get the process ID associated with this event
    pub fn pid(&self) -> u32 {
        match self {
        }
    }
    
    /// Get the timestamp when this event occurred
    pub fn timestamp(&self) -> Instant {
        match self {
        }
    }
/// RuntimeProcessInfo lifecycle manager
#[derive(Debug)]
pub struct ProcessLifecycleManager {
    /// Active processes being managed
    /// Maximum number of concurrent processes
    /// Default timeout for process operations
    /// Cleanup thread handle
    /// Shutdown flag
/// Managed process wrapper
#[derive(Debug)]
struct ManagedProcess {
    /// RuntimeProcessInfo instance
    /// Spawn time
    /// Expected termination time (if timeout set)
    /// Lifecycle state
    /// Parent manager reference (weak to avoid cycles)
/// RuntimeProcessInfo lifecycle states
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessLifecycleState {
    /// RuntimeProcessInfo is starting up
    /// RuntimeProcessInfo is running normally
    /// RuntimeProcessInfo is being terminated
    /// RuntimeProcessInfo has completed successfully
    /// RuntimeProcessInfo failed during execution
    /// RuntimeProcessInfo was terminated due to timeout
    /// RuntimeProcessInfo was forcibly killed
impl ProcessLifecycleManager {
    /// Create a new process lifecycle manager
    pub fn new() -> Self {
        Self::with_config(100, Duration::from_secs(300))
    /// Create a new process lifecycle manager with custom configuration
    pub fn with_config(max_concurrent: usize, default_timeout: Duration) -> Self {
        let active_processes = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_flag = Arc::new(Mutex::new(false));
        
        let manager = Self {
        
        // Start cleanup thread
        let cleanup_active = active_processes.clone();
        let cleanup_shutdown = shutdown_flag.clone();
        let cleanup_handle = thread::spawn(move || {
            Self::cleanup_loop(cleanup_active, cleanup_shutdown);
        });
        
        Self {
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

        // Add to active processes
        {
            let mut active = self.active_processes.lock().unwrap();
            active.insert(pid, managed);
        Ok(pid)
    /// Wait for a process to complete
    pub fn wait(&self, pid: u32) -> ProcessResult<ExitStatus> {
        self.wait_with_timeout(pid, None)
    /// Wait for a process to complete with timeout
    pub fn wait_with_timeout(&self, pid: u32, timeout: Option<Duration>) -> ProcessResult<ExitStatus> {
        let start_time = Instant::now();
        let deadline = timeout.map(|t| start_time + t);

        loop {
            // Check if process exists and get its state
            let state = {
                let active = self.active_processes.lock().unwrap();
                match active.get(&pid) {
                }

            // Check final states
            match state {
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
            }

        // Try graceful termination first
        if let Err(_) = process.terminate() {
            // Graceful termination failed, try kill
            process.kill()?;
        // Wait for termination with grace period
        let start = Instant::now();
        while start.elapsed() < grace_period {
            if !process.is_running()? {
                self.mark_process_killed(pid);
                return Ok(());
            }
            thread::sleep(Duration::from_millis(100));
        // Force kill if still running
        process.kill()?;
        self.mark_process_killed(pid);
        Ok(())
    /// Kill a process immediately
    pub fn kill(&self, pid: u32) -> ProcessResult<()> {
        let mut process = {
            let mut active = self.active_processes.lock().unwrap();
            match active.get_mut(&pid) {
                Some(managed) => {
                    managed.state = ProcessLifecycleState::Terminating;
                    managed.process.clone()
                }
            }

        process.kill()?;
        self.mark_process_killed(pid);
        Ok(())
    /// Get process information
    pub fn get_process_info(&self, pid: u32) -> ProcessResult<ProcessInfo> {
        let active = self.active_processes.lock().unwrap();
        match active.get(&pid) {
        }
    }

    /// Get process lifecycle state
    pub fn get_process_state(&self, pid: u32) -> ProcessResult<ProcessLifecycleState> {
        let active = self.active_processes.lock().unwrap();
        match active.get(&pid) {
        }
    }

    /// List all active processes
    pub fn list_active_processes(&self) -> Vec<u32> {
        let active = self.active_processes.lock().unwrap();
        active.keys().copied().collect()
    /// Get the number of active processes
    pub fn active_process_count(&self) -> usize {
        let active = self.active_processes.lock().unwrap();
        active.len()
    /// Terminate all active processes
    pub fn terminate_all(&self, grace_period: Option<Duration>) -> ProcessResult<()> {
        let pids: Vec<u32> = {
            let active = self.active_processes.lock().unwrap();
            active.keys().copied().collect()

        for pid in pids {
            if let Err(e) = self.terminate(pid, grace_period) {
                eprintln!("Failed to terminate process {}: {}", pid, e);
            }
        }

        Ok(())
    /// Shutdown the lifecycle manager
    pub fn shutdown(&mut self) -> ProcessResult<()> {
        // Set shutdown flag
        {
            let mut shutdown = self.shutdown_flag.lock().unwrap();
            *shutdown = true;
        // Terminate all processes
        self.terminate_all(Some(Duration::from_secs(5)))?;

        // Wait for cleanup thread to finish
        if let Some(handle) = self.cleanup_thread.take() {
            handle.join().map_err(|_| {
                system_error("Failed to join cleanup thread", None)
            })?;
        Ok(())
    /// Mark a process as killed
    fn mark_process_killed(&self, pid: u32) {
        let mut active = self.active_processes.lock().unwrap();
        if let Some(managed) = active.get_mut(&pid) {
            managed.state = ProcessLifecycleState::Killed;
        }
    }

    /// Cleanup loop for background thread
    fn cleanup_loop(
    ) {
        while !*shutdown_flag.lock().unwrap() {
            let pids_to_check: Vec<u32> = {
                let active = active_processes.lock().unwrap();
                active.keys().copied().collect()

            for pid in pids_to_check {
                Self::check_process_status(&active_processes, pid);
            thread::sleep(Duration::from_secs(1));
        }
    }

    /// Check and update process status
    fn check_process_status(
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
                            // CursedError checking process status
                            managed.state = ProcessLifecycleState::Failed(
                                system_error(&format!("Failed to check process status: {}", e), None)
                            );
                            true // Remove from active list
                        }
                    }
                }
                None => false, // RuntimeProcessInfo not found, nothing to do
            }

        if should_remove {
            let mut active = active_processes.lock().unwrap();
            active.remove(&pid);
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
            eprintln!("CursedError during ProcessLifecycleManager shutdown: {}", e);
        }
    }
/// RuntimeProcessInfo lifecycle statistics
#[derive(Debug, Clone)]
pub struct ProcessLifecycleStats {
    /// Total processes spawned
    /// Currently active processes
    /// Processes completed successfully
    /// Processes that failed
    /// Processes that timed out
    /// Processes that were killed
    /// Average process runtime
    /// Maximum concurrent processes reached
impl ProcessLifecycleManager {
    /// Get lifecycle statistics
    pub fn get_statistics(&self) -> ProcessLifecycleStats {
        let active = self.active_processes.lock().unwrap();
        
        // For a complete implementation, we would need to track these statistics
        // Here we provide a basic implementation
        ProcessLifecycleStats {
            total_spawned: 0, // Would be tracked in a production implementation
        }
    }
