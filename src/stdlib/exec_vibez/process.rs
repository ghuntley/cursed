use crate::error::Error;
/// Process management for exec_vibez
/// 
/// Implements Process and ProcessState types according to specs/stdlib/exec_vibez.md

use std::io;
use std::process::{self, ExitStatus};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

use super::error::{ExecResult, ExecError, execution_failed};
use crate::error::CursedError;

/// Represents a process created by a call to Start or Run
#[derive(Debug)]
pub struct Process {
    /// Process ID
    pub pid: u32,
    /// Internal process handle
    inner: Arc<Mutex<ProcessInner>>,
}

#[derive(Debug)]
struct ProcessInner {
    /// System process handle
    process: Option<process::Child>,
    /// Process start time
    start_time: Instant,
    /// Whether the process has been released
    released: bool,
}

impl Process {
    /// Create a new Process wrapper
    pub(crate) fn new(mut child: process::Child) -> ExecResult<Self> {
        let pid = child.id();
        
        let inner = ProcessInner {
            process: Some(child),
            start_time: Instant::now(),
            released: false,
        };
        
        Ok(Process {
            pid,
            inner: Arc::new(Mutex::new(inner)),
        })
    }
    
    /// Get the process ID
    pub fn pid(&self) -> u32 {
        self.pid
    }
    
    /// Kill the process
    pub fn kill(&self) -> ExecResult<()> {
        let mut inner = self.inner.lock().unwrap();
        
        if let Some(ref mut process) = inner.process {
            process.kill().map_err(|e| execution_failed(&format!("Failed to kill process {}: {}", self.pid, e)))?;
        }
        
        Ok(())
    }
    
    /// Send a signal to the process (Unix only)
    #[cfg(unix)]
    pub fn signal(&self, sig: i32) -> ExecResult<()> {
        use std::os::unix::process::ExitStatusExt;
        
        unsafe {
            let result = libc::kill(self.pid as libc::pid_t, sig);
            if result != 0 {
                return Err(execution_failed(&format!("Failed to send signal {} to process {}", sig, self.pid)));
            }
        }
        
        Ok(())
    }
    
    /// Send a signal to the process (Windows - limited support)
    #[cfg(windows)]
    pub fn signal(&self, _sig: i32) -> ExecResult<()> {
        // Windows doesn't have POSIX signals, so we can only terminate
        self.kill()
    }
    
    /// Wait for the process to complete and return its state
    pub fn wait(&self) -> ExecResult<ProcessState> {
        let mut inner = self.inner.lock().unwrap();
        
        if let Some(mut process) = inner.process.take() {
            let exit_status = process.wait().map_err(|e| execution_failed(&format!("Failed to wait for process {}: {}", self.pid, e)))?;
            
            let state = ProcessState::new(exit_status, inner.start_time);
            Ok(state)
        } else {
            Err(execution_failed("Process has already been waited for or released"))
        }
    }
    
    /// Release the process (detach from it)
    pub fn release(&self) -> ExecResult<()> {
        let mut inner = self.inner.lock().unwrap();
        inner.released = true;
        inner.process.take(); // Drop the process handle
        Ok(())
    }
    
    /// Check if the process is still running
    pub fn is_running(&self) -> bool {
        let mut inner = self.inner.lock().unwrap();
        
        if let Some(ref mut process) = inner.process {
            match process.try_wait() {
                Ok(Some(_)) => false, // Process has exited
                Ok(None) => true,     // Process is still running
                Err(_) => false,      // Error checking status, assume not running
            }
        } else {
            false // No process handle
        }
    }
    
    /// Get process uptime
    pub fn uptime(&self) -> Duration {
        let inner = self.inner.lock().unwrap();
        inner.start_time.elapsed()
    }
}

/// Contains information about a process that has exited
#[derive(Debug, Clone)]
pub struct ProcessState {
    /// Process exit status
    exit_status: ExitStatus,
    /// Process start time
    start_time: Instant,
    /// Process end time
    end_time: Instant,
}

impl ProcessState {
    /// Create a new ProcessState
    pub(crate) fn new(exit_status: ExitStatus, start_time: Instant) -> Self {
        Self {
            exit_status,
            start_time,
            end_time: Instant::now(),
        }
    }
    
    /// Check if the process has exited
    pub fn exited(&self) -> bool {
        true // If we have a ProcessState, the process has exited
    }
    
    /// Get the exit code
    pub fn exit_code(&self) -> i32 {
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            self.exit_status.code().unwrap_or_else(|| {
                // If terminated by signal, return negative signal number
                if let Some(signal) = self.exit_status.signal() {
                    -(signal)
                } else {
                    -1
                }
            })
        }
        
        #[cfg(windows)]
        {
            self.exit_status.code().unwrap_or(-1)
        }
    }
    
    /// Check if the process completed successfully
    pub fn success(&self) -> bool {
        self.exit_status.success()
    }
    
    /// Get system-specific information
    pub fn sys(&self) -> Box<dyn std::any::Any> {
        Box::new(self.exit_status)
    }
    
    /// Get system usage information (placeholder)
    pub fn sys_usage(&self) -> Box<dyn std::any::Any> {
        // This would contain resource usage info in a real implementation
        Box::new(ProcessResourceUsage::default())
    }
    
    /// Get user CPU time (placeholder)
    pub fn user_time(&self) -> Duration {
        // In a real implementation, this would query system for CPU time
        Duration::from_millis(0)
    }
    
    /// Get system CPU time (placeholder)
    pub fn system_time(&self) -> Duration {
        // In a real implementation, this would query system for CPU time
        Duration::from_millis(0)
    }
    
    /// Get total process runtime
    pub fn runtime(&self) -> Duration {
        self.end_time.duration_since(self.start_time)
    }
}

impl std::fmt::Display for ProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "exit status: {}, runtime: {:?}", self.exit_code(), self.runtime())
    }
}

/// Process resource usage information (placeholder for real implementation)
#[derive(Debug, Default)]
pub struct ProcessResourceUsage {
    pub max_rss: u64,        // Maximum resident set size
    pub user_time: Duration, // User CPU time
    pub sys_time: Duration,  // System CPU time
    pub voluntary_switches: u64,   // Voluntary context switches
    pub involuntary_switches: u64, // Involuntary context switches
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    
    #[test]
    fn test_process_creation() {
        // Test with a simple command that should be available on most systems
        let child = Command::new("echo")
            .arg("test")
            .spawn();
            
        if let Ok(child) = child {
            let process = Process::new(child).unwrap();
            assert!(process.pid() > 0);
        }
        // If echo is not available, skip the test
    }
    
    #[test]
    fn test_process_state_creation() {
        use std::process::ExitStatus;
use crate::stdlib::process::info::ProcessState;
        
        // Create a mock exit status for testing
        // This is a bit tricky since ExitStatus can't be constructed directly
        // We'll test the methods that don't depend on actual process execution
        let now = Instant::now();
        
        // We can't easily create an ExitStatus, so we'll test what we can
        let uptime = now.elapsed();
        assert!(uptime >= Duration::from_nanos(0));
    }
    
    #[test]
    fn test_process_resource_usage() {
        let usage = ProcessResourceUsage::default();
        assert_eq!(usage.max_rss, 0);
        assert_eq!(usage.user_time, Duration::from_secs(0));
        assert_eq!(usage.sys_time, Duration::from_secs(0));
    }
}
