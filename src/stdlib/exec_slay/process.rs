use crate::error::Error;
// SlayProcess implementation for process management

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::error::CursedError;
use super::{SlayResult, SharedProcessState, ProcessStats, SignalOptions, io_error_to_cursed};
use crate::stdlib::process::real_monitoring::get_real_cpu_times;

/// Represents a process created by a SlayCommand
#[derive(Debug)]
pub struct SlayProcess {
    /// Shared process state
    pub(crate) state: Arc<Mutex<SharedProcessState>>,
}

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
        }
        
        Ok(())
    }

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
        }

        Ok(())
    }

    /// Send a signal to the process (Windows - limited support)
    #[cfg(windows)]
    pub fn signal(&self, _sig: i32) -> SlayResult<()> {
        // On Windows, we can only terminate
        self.kill()
    }

    /// Get the process ID
    pub fn pid(&self) -> Option<u32> {
        let state = self.state.lock().unwrap();
        state.child.as_ref().map(|child| child.id())
    }

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
                exit_status,
                state.start_time.elapsed(),
                state.stdout_data.clone(),
                state.stderr_data.clone(),
                user_time,
                system_time,
                Some(pid),
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
    }

    /// Terminate the process gracefully with options
    pub fn terminate(&self, opts: SignalOptions) -> SlayResult<()> {
        if opts.force {
            return self.kill();
        }

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
        }

        // Wait for grace period
        let start = Instant::now();
        while start.elapsed() < opts.grace_period {
            if !self.is_running() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        // Grace period expired, force kill
        self.kill()
    }

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
    }

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
        }

        Ok(())
    }

    /// Check if the process is still running
    pub fn is_running(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        
        if !state.is_running {
            return false;
        }

        // Check if process is actually still alive
        if let Some(ref mut child) = state.child {
            match child.try_wait() {
                Ok(Some(status)) => {
                    state.exit_status = Some(status);
                    state.is_running = false;
                    false
                }
                Ok(None) => true,
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
            resident_memory: 0,
            virtual_memory: 0,
            swap_memory: 0,
            read_bytes: 0,
            write_bytes: 0,
            read_ops: 0,
            write_ops: 0,
            up_time,
            thread_count: 1, // Minimum assumption
            open_files: 0,
            network_conns: 0,
        })
    }

    /// Monitor the process with periodic stats updates
    pub fn monitor<F>(&self, interval: Duration, callback: F) -> SlayResult<()>
    where
        F: Fn(&ProcessStats) + Send + 'static,
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
                    memory: 0,
                    resident_memory: 0,
                    virtual_memory: 0,
                    swap_memory: 0,
                    read_bytes: 0,
                    write_bytes: 0,
                    read_ops: 0,
                    write_ops: 0,
                    up_time: {
                        let state = state_clone.lock().unwrap();
                        state.start_time.elapsed()
                    },
                    thread_count: 1,
                    open_files: 0,
                    network_conns: 0,
                };
                
                callback(&stats);
                std::thread::sleep(interval);
            }
        });

        Ok(())
    }

    /// Set resource limits for the process
    pub fn set_limits(&self, memory_mb: i32, cpu_percent: f64) -> SlayResult<()> {
        // This would require platform-specific implementations
        // For now, just validate the inputs
        if memory_mb <= 0 {
            return Err(CursedError::RuntimeError("Memory limit must be positive".to_string()));
        }
        
        if cpu_percent <= 0.0 || cpu_percent > 100.0 {
            return Err(CursedError::RuntimeError("CPU limit must be between 0 and 100".to_string()));
        }

        // Platform-specific implementation would go here
        Ok(())
    }

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
    exit_status: std::process::ExitStatus,
    /// Process run time
    run_time: Duration,
    /// Captured stdout
    stdout_data: Vec<u8>,
    /// Captured stderr
    stderr_data: Vec<u8>,
    /// User CPU time
    user_time: Duration,
    /// System CPU time  
    system_time: Duration,
    /// Process ID (if available)
    pid: Option<u32>,
}

impl SlayProcessState {
    /// Create a new SlayProcessState
    pub(crate) fn new(
        exit_status: std::process::ExitStatus,
        run_time: Duration,
        stdout_data: Vec<u8>,
        stderr_data: Vec<u8>,
    ) -> Self {
        Self {
            exit_status,
            run_time,
            stdout_data,
            stderr_data,
            user_time: Duration::from_millis(0),
            system_time: Duration::from_millis(0),
            pid: None,
        }
    }

    /// Create a new SlayProcessState with timing information
    pub(crate) fn with_timing(
        exit_status: std::process::ExitStatus,
        run_time: Duration,
        stdout_data: Vec<u8>,
        stderr_data: Vec<u8>,
        user_time: Duration,
        system_time: Duration,
        pid: Option<u32>,
    ) -> Self {
        Self {
            exit_status,
            run_time,
            stdout_data,
            stderr_data,
            user_time,
            system_time,
            pid,
        }
    }

    /// Check if the process exited normally
    pub fn exited(&self) -> bool {
        self.exit_status.code().is_some()
    }

    /// Check if the process was successful
    pub fn success(&self) -> bool {
        self.exit_status.success()
    }

    /// Get the exit code
    pub fn exit_code(&self) -> i32 {
        self.exit_status.code().unwrap_or(-1)
    }

    /// Get system-specific information
    pub fn sys(&self) -> &std::process::ExitStatus {
        &self.exit_status
    }

    /// Get system usage information (placeholder)
    pub fn sys_usage(&self) -> Option<()> {
        None // Would need platform-specific implementation
    }

    /// Get user time
    pub fn user_time(&self) -> Duration {
        self.user_time
    }

    /// Get system time
    pub fn system_time(&self) -> Duration {
        self.system_time
    }

    /// Get total run time
    pub fn run_time(&self) -> Duration {
        self.run_time
    }

    /// Get captured stdout
    pub fn stdout(&self) -> &[u8] {
        &self.stdout_data
    }

    /// Get captured stderr
    pub fn stderr(&self) -> &[u8] {
        &self.stderr_data
    }

    /// Get string representation
    pub fn to_string(&self) -> String {
        format!(
            "exit status: {}, success: {}, run time: {:?}",
            self.exit_code(),
            self.success(),
            self.run_time
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn create_test_process() -> SlayProcess {
        SlayProcess::new(Arc::new(Mutex::new(SharedProcessState::new())))
    }

    #[test]
    fn test_slay_process_creation() {
        let process = create_test_process();
        assert!(!process.is_running());
    }

    #[test]
    fn test_slay_process_elapsed_time() {
        let process = create_test_process();
        let elapsed = process.elapsed_time();
        assert!(elapsed >= Duration::from_secs(0));
    }

    #[test]
    fn test_slay_process_set_limits() {
        let process = create_test_process();
        
        // Valid limits
        assert!(process.set_limits(100, 50.0).is_ok());
        
        // Invalid limits
        assert!(process.set_limits(-1, 50.0).is_err());
        assert!(process.set_limits(100, -1.0).is_err());
        assert!(process.set_limits(100, 150.0).is_err());
    }

    #[test]
    fn test_signal_options_default() {
        let opts = SignalOptions::default();
        assert_eq!(opts.grace_period, Duration::from_secs(5));
        assert!(!opts.force);
        assert_eq!(opts.signal, 15);
        assert!(!opts.recursive);
    }

    #[test]
    fn test_slay_process_state() {
        use std::process::ExitStatus;
        use std::os::unix::process::ExitStatusExt;
use crate::stdlib::process::info::ProcessState;
        
        let exit_status = ExitStatus::from_raw(0);
        let run_time = Duration::from_secs(1);
        let stdout = b"hello".to_vec();
        let stderr = b"error".to_vec();
        
        let state = SlayProcessState::new(exit_status, run_time, stdout.clone(), stderr.clone());
        
        assert!(state.success());
        assert_eq!(state.exit_code(), 0);
        assert_eq!(state.run_time(), run_time);
        assert_eq!(state.stdout(), stdout.as_slice());
        assert_eq!(state.stderr(), stderr.as_slice());
    }
}
