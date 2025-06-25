use crate::error::CursedError;
/// Timeout and cancellation support for exec_vibez
/// 
/// Implements timeout functionality according to specs/stdlib/exec_vibez.md

use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use super::cmd::Cmd;
use super::process::{Process, ProcessState};
use super::error::{ExecResult, ExecError, execution_failed};

/// Timeout configuration for command execution
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Execution timeout
    pub timeout: Duration,
    /// Grace period before force killing
    pub grace_period: Duration,
    /// Whether to use SIGTERM before SIGKILL (Unix only)
    pub graceful_shutdown: bool,
    /// Whether to kill the entire process group
    pub kill_group: bool,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            grace_period: Duration::from_secs(5),
            graceful_shutdown: true,
            kill_group: false,
        }
    }
}

impl TimeoutConfig {
    /// Create a new timeout configuration
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            ..Default::default()
        }
    }
    
    /// Set the grace period
    pub fn with_grace_period(mut self, grace_period: Duration) -> Self {
        self.grace_period = grace_period;
        self
    }
    
    /// Set graceful shutdown behavior
    pub fn with_graceful_shutdown(mut self, graceful: bool) -> Self {
        self.graceful_shutdown = graceful;
        self
    }
    
    /// Set whether to kill the process group
    pub fn with_kill_group(mut self, kill_group: bool) -> Self {
        self.kill_group = kill_group;
        self
    }
}

/// Run a command with a timeout
pub fn run_with_timeout(name: &str, args: &[&str], timeout: Duration) -> ExecResult<Vec<u8>> {
    let config = TimeoutConfig::new(timeout);
    run_with_timeout_config(name, args, config)
}

/// Run a command with timeout configuration
pub fn run_with_timeout_config(name: &str, args: &[&str], config: TimeoutConfig) -> ExecResult<Vec<u8>> {
    let mut cmd = Cmd::new(name, args);
    cmd.set_timeout(config.timeout);
    
    let start_time = Instant::now();
    let process = cmd.start()?;
    
    // Create a timeout handler
    let timeout_handler = TimeoutHandler::new(process, config);
    let result = timeout_handler.wait_with_timeout()?;
    
    match result {
        TimeoutResult::Completed(state) => {
            if state.success() {
                cmd.output()
            } else {
                Err(execution_failed(&format!("Command failed with exit code: {}", state.exit_code())))
            }
        }
        TimeoutResult::TimedOut => {
            Err(timeout_exceeded(&format!("Command timed out after {:?}", config.timeout)))
        }
        TimeoutResult::Killed => {
            Err(execution_failed("Command was killed"))
        }
    }
}

/// Output command with timeout
pub fn output_with_timeout(name: &str, args: &[&str], timeout: Duration) -> ExecResult<Vec<u8>> {
    run_with_timeout(name, args, timeout)
}

/// Combined output command with timeout
pub fn combined_output_with_timeout(name: &str, args: &[&str], timeout: Duration) -> ExecResult<Vec<u8>> {
    let mut cmd = Cmd::new(name, args);
    cmd.set_timeout(timeout);
    cmd.combined_output()
}

/// Timeout handler for process management
#[derive(Debug)]
struct TimeoutHandler {
    process: Process,
    config: TimeoutConfig,
    cancelled: Arc<AtomicBool>,
}

/// Result of a timeout operation
#[derive(Debug, Clone)]
enum TimeoutResult {
    Completed(ProcessState),
    TimedOut,
    Killed,
}

impl TimeoutHandler {
    /// Create a new timeout handler
    fn new(process: Process, config: TimeoutConfig) -> Self {
        Self {
            process,
            config,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
    
    /// Wait for the process with timeout
    fn wait_with_timeout(self) -> ExecResult<TimeoutResult> {
        let start_time = Instant::now();
        let cancelled = Arc::clone(&self.cancelled);
        let timeout = self.config.timeout;
        
        // Spawn a timeout thread
        let timeout_process = self.process;
        let timeout_config = self.config;
        
        let timeout_thread = thread::spawn(move || {
            thread::sleep(timeout);
            
            if !cancelled.load(Ordering::Relaxed) {
                // Timeout occurred, try to kill the process
                if timeout_config.graceful_shutdown {
                    Self::graceful_kill(&timeout_process, &timeout_config);
                } else {
                    let _ = timeout_process.kill();
                }
                
                TimeoutResult::TimedOut
            } else {
                // Process completed before timeout
                TimeoutResult::Killed
            }
        });
        
        // Wait for the process in the main thread
        let process_result = loop {
            if self.process.is_running() {
                thread::sleep(Duration::from_millis(10));
                
                // Check if we've exceeded the timeout
                if start_time.elapsed() >= timeout {
                    break None;
                }
            } else {
                // Process has completed
                break Some(self.process.wait());
            }
        };
        
        // Cancel the timeout thread
        self.cancelled.store(true, Ordering::Relaxed);
        
        match process_result {
            Some(Ok(state)) => Ok(TimeoutResult::Completed(state)),
            Some(Err(e)) => Err(e),
            None => {
                // Wait for timeout thread to finish cleanup
                match timeout_thread.join() {
                    Ok(result) => Ok(result),
                    Err(_) => Ok(TimeoutResult::TimedOut),
                }
            }
        }
    }
    
    /// Gracefully kill a process
    fn graceful_kill(process: &Process, config: &TimeoutConfig) {
        #[cfg(unix)]
        {
            // Try SIGTERM first
            if let Err(e) = process.signal(libc::SIGTERM) {
                tracing::warn!("Failed to send SIGTERM to process {}: {}", process.pid(), e);
            } else {
                // Wait for grace period
                let grace_start = Instant::now();
                while grace_start.elapsed() < config.grace_period {
                    if !process.is_running() {
                        return; // Process terminated gracefully
                    }
                    thread::sleep(Duration::from_millis(10));
                }
            }
            
            // If still running after grace period, use SIGKILL
            if process.is_running() {
                if let Err(e) = process.signal(libc::SIGKILL) {
                    tracing::warn!("Failed to send SIGKILL to process {}: {}", process.pid(), e);
                }
            }
        }
        
        #[cfg(windows)]
        {
            // Windows doesn't have graceful signals, just terminate
            if let Err(e) = process.kill() {
                tracing::warn!("Failed to terminate process {}: {}", process.pid(), e);
            }
        }
    }
}

/// Timeout manager for multiple processes
#[derive(Debug)]
pub struct TimeoutManager {
    /// Active timeouts
    timeouts: Arc<Mutex<Vec<ActiveTimeout>>>,
}

#[derive(Debug)]
struct ActiveTimeout {
    process_id: u32,
    deadline: Instant,
    config: TimeoutConfig,
}

impl TimeoutManager {
    /// Create a new timeout manager
    pub fn new() -> Self {
        Self {
            timeouts: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Add a process to timeout management
    pub fn add_timeout(&self, process_id: u32, config: TimeoutConfig) {
        let deadline = Instant::now() + config.timeout;
        let timeout = ActiveTimeout {
            process_id,
            deadline,
            config,
        };
        
        self.timeouts.lock().unwrap().push(timeout);
    }
    
    /// Remove a process from timeout management
    pub fn remove_timeout(&self, process_id: u32) {
        let mut timeouts = self.timeouts.lock().unwrap();
        timeouts.retain(|t| t.process_id != process_id);
    }
    
    /// Check for expired timeouts
    pub fn check_timeouts(&self) -> Vec<u32> {
        let now = Instant::now();
        let timeouts = self.timeouts.lock().unwrap();
        
        timeouts.iter()
            .filter(|t| now >= t.deadline)
            .map(|t| t.process_id)
            .collect()
    }
    
    /// Get the number of active timeouts
    pub fn active_count(&self) -> usize {
        self.timeouts.lock().unwrap().len()
    }
}

impl Default for TimeoutManager {
    fn default() -> Self {
        Self::new()
    }
}



pub trait RunWithTimeout {
    fn run_with_timeout(&self, timeout: std::time::Duration) -> ExecResult<()>;
}
