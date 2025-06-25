use crate::error::CursedError;
// SlayProcess implementation for process management

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use super::{SlayResult, SharedProcessState, ProcessStats, SignalOptions, io_error_to_cursed};
// use crate::stdlib::process::real_monitoring::get_real_cpu_times;

/// Represents a process created by a SlayCommand
#[derive(Debug)]
pub struct SlayProcess {
    /// Shared process state
impl SlayProcess {
    /// Create a new SlayProcess with shared state
    pub(crate) fn new(state: Arc<Mutex<SharedProcessState>>) -> Self {
        Self { state }
    }

    /// Kill the process immediately
    pub fn kill(&self) -> SlayResult<()> {
        let mut state = self.state.lock().unwrap();
        
        if let Some(ref mut child) = state.child {
            child.kill().map_err(io_error_to_cursed)?;
            state.is_running = false;
        Ok(())
    /// Send a signal to the process (Unix only)
    #[cfg(unix)]
    pub fn signal(&self, sig: i32) -> SlayResult<()> {
        use std::process::Command;
        
        let pid = self.pid().ok_or_else(|| {
            CursedError::RuntimeError("Process not running or PID unavailable".to_string())
        })?;

        let output = Command::new("kill")
            .arg("-s")
            .arg(sig.to_string())
            .arg(pid.to_string())
            .output()
            .map_err(io_error_to_cursed)?;

        if !output.status.success() {
            return Err(CursedError::RuntimeError(
                format!("Failed to send signal {}: {}", sig, String::from_utf8_lossy(&output.stderr))
            ));
        Ok(())
    /// Send a signal to the process (Windows - limited support)
    #[cfg(windows)]
    pub fn signal(&self, _sig: i32) -> SlayResult<()> {
        // On Windows, we can only terminate
        self.kill()
    /// Get the process ID
    pub fn pid(&self) -> Option<u32> {
        let state = self.state.lock().unwrap();
        state.child.as_ref().map(|child| child.id())
    /// Wait for the process to complete
    pub fn wait(&self) -> SlayResult<SlayProcessState> {
        let mut state = self.state.lock().unwrap();
        
        if let Some(ref mut child) = state.child {
            let pid = child.id();
            let exit_status = child.wait().map_err(io_error_to_cursed)?;
            state.exit_status = Some(exit_status);
            state.is_running = false;
            
            // Get CPU timing information
            let (user_time, system_time) = get_real_cpu_times(pid)
                .unwrap_or((Duration::from_millis(0), Duration::from_millis(0)));
            
            Ok(SlayProcessState::with_timing(
            ))
        } else {
            Err(CursedError::RuntimeError("No process to wait for".to_string()))
        }
    }

    /// Release the process handle
    pub fn release(&self) -> SlayResult<()> {
        let mut state = self.state.lock().unwrap();
        state.child = None;
        state.is_running = false;
        Ok(())
    /// Terminate the process gracefully with options
    pub fn terminate(&self, opts: SignalOptions) -> SlayResult<()> {
        if opts.force {
            return self.kill();
        // Try graceful termination first
        #[cfg(unix)]
        {
            if let Err(_) = self.signal(opts.signal) {
                // If signal fails, fall back to kill
                return self.kill();
            }
        }

        #[cfg(windows)]
        {
            // On Windows, we can only kill
            return self.kill();
        // Wait for grace period
        let start = Instant::now();
        while start.elapsed() < opts.grace_period {
            if !self.is_running() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        // Grace period expired, force kill
        self.kill()
    /// Kill the entire process tree (Unix only)
    #[cfg(unix)]
    pub fn kill_tree(&self) -> SlayResult<()> {
        use std::process::Command;
        
        let pid = self.pid().ok_or_else(|| {
            CursedError::RuntimeError("Process not running or PID unavailable".to_string())
        })?;

        // Use pkill to kill the process group
        let output = Command::new("pkill")
            .arg("-TERM")
            .arg("-P")
            .arg(pid.to_string())
            .output();

        match output {
            Ok(result) => {
                if !result.status.success() {
                    // Try individual kill if pkill fails
                    return self.kill();
                }
            }
            Err(_) => {
                // Fall back to individual kill
                return self.kill();
            }
        }

        // Wait a bit then kill the parent
        std::thread::sleep(Duration::from_millis(500));
        self.kill()
    /// Kill the entire process tree (Windows)
    #[cfg(windows)]
    pub fn kill_tree(&self) -> SlayResult<()> {
        use std::process::Command;
        
        let pid = self.pid().ok_or_else(|| {
            CursedError::RuntimeError("Process not running or PID unavailable".to_string())
        })?;

        let output = Command::new("taskkill")
            .arg("/F")
            .arg("/T")
            .arg("/PID")
            .arg(pid.to_string())
            .output()
            .map_err(io_error_to_cursed)?;

        if !output.status.success() {
            return Err(CursedError::RuntimeError(
                format!("Failed to kill process tree: {}", String::from_utf8_lossy(&output.stderr))
            ));
        Ok(())
    /// Check if the process is still running
    pub fn is_running(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        
        if !state.is_running {
            return false;
        // Check if process is actually still alive
        if let Some(ref mut child) = state.child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    state.exit_status = Some(status);
                    state.is_running = false;
                    false
                }
                Err(_) => {
                    state.is_running = false;
                    false
                }
            }
        } else {
            false
        }
    }

    /// Get basic process statistics
    pub fn stats(&self) -> SlayResult<ProcessStats> {
        let pid = self.pid().ok_or_else(|| {
            CursedError::RuntimeError("Process not running or PID unavailable".to_string())
        })?;

        let state = self.state.lock().unwrap();
        let up_time = state.start_time.elapsed();

        // Create basic stats - platform-specific implementations would get real data
        Ok(ProcessStats {
            cpu: 0.0, // Would need platform-specific implementation
            memory: 0, // Would need platform-specific implementation
            thread_count: 1, // Minimum assumption
        })
    /// Monitor the process with periodic stats updates
    pub fn monitor<F>(&self, interval: Duration, callback: F) -> SlayResult<()>
    where
    {
        let state_clone = self.state.clone();
        
        std::thread::spawn(move || {
            while {
                let state = state_clone.lock().unwrap();
                state.is_running
            } {
                // Get stats and call callback
                let stats = ProcessStats {
                    cpu: 0.0, // Platform-specific implementation needed
                    up_time: {
                        let state = state_clone.lock().unwrap();
                        state.start_time.elapsed()
                
                callback(&stats);
                std::thread::sleep(interval);
            }
        });

        Ok(())
    /// Set resource limits for the process
    pub fn set_limits(&self, memory_mb: i32, cpu_percent: f64) -> SlayResult<()> {
        // This would require platform-specific implementations
        // For now, just validate the inputs
        if memory_mb <= 0 {
            return Err(CursedError::RuntimeError("Memory limit must be positive".to_string()));
        if cpu_percent <= 0.0 || cpu_percent > 100.0 {
            return Err(CursedError::RuntimeError("CPU limit must be between 0 and 100".to_string()));
        // Platform-specific implementation would go here
        Ok(())
    /// Get the elapsed time since process start
    pub fn elapsed_time(&self) -> Duration {
        let state = self.state.lock().unwrap();
        state.start_time.elapsed()
    }
}

/// Contains information about a process that has finished
#[derive(Debug, Clone)]
pub struct SlayProcessState {
    /// Exit status
    /// Process run time
    /// Captured stdout
    /// Captured stderr
    /// User CPU time
    /// System CPU time  
    /// Process ID (if available)
impl SlayProcessState {
    /// Create a new SlayProcessState
    pub(crate) fn new(
    ) -> Self {
        Self {
        }
    }

    /// Create a new SlayProcessState with timing information
    pub(crate) fn with_timing(
    ) -> Self {
        Self {
        }
    }

    /// Check if the process exited normally
    pub fn exited(&self) -> bool {
        self.exit_status.code().is_some()
    /// Check if the process was successful
    pub fn success(&self) -> bool {
        self.exit_status.success()
    /// Get the exit code
    pub fn exit_code(&self) -> i32 {
        self.exit_status.code().unwrap_or(-1)
    /// Get system-specific information
    pub fn sys(&self) -> &std::process::ExitStatus {
        &self.exit_status
    /// Get system usage information (placeholder)
    pub fn sys_usage(&self) -> Option<()> {
        None // Would need platform-specific implementation
    /// Get user time
    pub fn user_time(&self) -> Duration {
        self.user_time
    /// Get system time
    pub fn system_time(&self) -> Duration {
        self.system_time
    /// Get total run time
    pub fn run_time(&self) -> Duration {
        self.run_time
    /// Get captured stdout
    pub fn stdout(&self) -> &[u8] {
        &self.stdout_data
    /// Get captured stderr
    pub fn stderr(&self) -> &[u8] {
        &self.stderr_data
    /// Get string representation
    pub fn to_string(&self) -> String {
        format!(
            self.run_time
        )
    }
}

