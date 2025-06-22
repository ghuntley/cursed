//! ExecSlay - Process execution module for CURSED
//! 
//! Provides utilities for running external commands with style and efficiency.
//! Inspired by Go's os/exec package but with enhanced features for process management,
//! input/output control, and resource monitoring.

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::process::{Child, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::error::CursedError;

// Re-export all public types
pub use command::*;
pub use process::*;
pub use pipeline::*;
pub use task::*;
pub use builder::*;
pub use shell::*;
pub use monitor::*;
pub use enhanced_command::*;

// Export timeout functions
pub use timeout::*;

mod command;
mod process;
mod pipeline;
mod task;
mod builder;
pub mod shell;
mod monitor;
mod timeout;
mod enhanced_command;

/// Result type for exec_slay operations
pub type SlayResult<T> = Result<T, CursedError>;

/// Configuration options for command execution
#[derive(Debug, Clone)]
pub struct SlayOptions {
    /// Working directory for the command
    pub dir: Option<String>,
    /// Environment variables
    pub env: Vec<String>,
    /// Timeout for command execution
    pub timeout: Option<Duration>,
    /// Wait delay before killing
    pub wait_delay: Option<Duration>,
    /// Signal to use for killing
    pub kill_signal: Option<i32>,
    /// Use shell for command execution
    pub use_shell: bool,
    /// Shell path to use
    pub shell_path: Option<String>,
    /// Buffer size for I/O operations
    pub buffer_size: usize,
    /// Collect output in memory
    pub collect_output: bool,
    /// Capture environment statistics
    pub capture_env_stats: bool,
    /// Working set memory limit in bytes
    pub working_limit: Option<i64>,
    /// CPU usage limit as percentage
    pub cpu_limit: Option<f64>,
}

impl Default for SlayOptions {
    fn default() -> Self {
        Self {
            dir: None,
            env: Vec::new(),
            timeout: None,
            wait_delay: None,
            kill_signal: None,
            use_shell: false,
            shell_path: None,
            buffer_size: 8192, // 8KB default buffer
            collect_output: true,
            capture_env_stats: false,
            working_limit: None,
            cpu_limit: None,
        }
    }
}

/// Signal handling options
#[derive(Debug, Clone)]
pub struct SignalOptions {
    /// Grace period before force killing
    pub grace_period: Duration,
    /// Force kill immediately
    pub force: bool,
    /// Signal to send
    pub signal: i32,
    /// Kill process tree recursively
    pub recursive: bool,
}

impl Default for SignalOptions {
    fn default() -> Self {
        Self {
            grace_period: Duration::from_secs(5),
            force: false,
            signal: 15, // SIGTERM
            recursive: false,
        }
    }
}

/// Process statistics for monitoring
#[derive(Debug, Clone)]
pub struct ProcessStats {
    /// CPU usage percentage
    pub cpu: f64,
    /// Memory usage in bytes
    pub memory: u64,
    /// Resident memory in bytes
    pub resident_memory: u64,
    /// Virtual memory in bytes
    pub virtual_memory: u64,
    /// Swap memory in bytes
    pub swap_memory: u64,
    /// Bytes read from disk
    pub read_bytes: u64,
    /// Bytes written to disk
    pub write_bytes: u64,
    /// Read operations count
    pub read_ops: u64,
    /// Write operations count
    pub write_ops: u64,
    /// Process uptime
    pub up_time: Duration,
    /// Number of threads
    pub thread_count: i32,
    /// Number of open files
    pub open_files: i32,
    /// Number of network connections
    pub network_conns: i32,
}

impl Default for ProcessStats {
    fn default() -> Self {
        Self {
            cpu: 0.0,
            memory: 0,
            resident_memory: 0,
            virtual_memory: 0,
            swap_memory: 0,
            read_bytes: 0,
            write_bytes: 0,
            read_ops: 0,
            write_ops: 0,
            up_time: Duration::from_secs(0),
            thread_count: 0,
            open_files: 0,
            network_conns: 0,
        }
    }
}

/// Internal shared state for process management
#[derive(Debug)]
pub(crate) struct SharedProcessState {
    /// Child process handle
    pub child: Option<Child>,
    /// Process start time
    pub start_time: Instant,
    /// Exit status when completed
    pub exit_status: Option<ExitStatus>,
    /// Collected stdout
    pub stdout_data: Vec<u8>,
    /// Collected stderr
    pub stderr_data: Vec<u8>,
    /// Whether the process is running
    pub is_running: bool,
    /// Last error encountered
    pub last_error: Option<String>,
}

impl SharedProcessState {
    pub fn new() -> Self {
        Self {
            child: None,
            start_time: Instant::now(),
            exit_status: None,
            stdout_data: Vec::new(),
            stderr_data: Vec::new(),
            is_running: false,
            last_error: None,
        }
    }
}

/// Convert std::io::Error to CursedError
pub(crate) fn io_error_to_cursed(err: io::Error) -> CursedError {
    CursedError::RuntimeError(format!("I/O error: {}", err))
}

// Public constructor functions for easier API usage

/// Create a new SlayCommand with the given name and arguments
pub fn new_slay_command(name: &str, args: &[&str]) -> SlayCommand {
    SlayCommand::new(name, args)
}

/// Create a new command builder
pub fn new_slay_command_builder(command: &str) -> SlayCommandBuilder {
    SlayCommandBuilder::new(command)
}

/// Create a new pipeline with commands
pub fn new_slay_pipeline(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::new(commands)
}

/// Create a pipeline from commands (convenience function)
pub fn pipe(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::pipe(commands)
}

/// Run a command in the background
pub fn run_background(command: SlayCommand) -> SlayTask {
    SlayTask::run_background(command)
}

/// Get the default shell for the current platform
pub(crate) fn get_default_shell() -> &'static str {
    if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "/bin/sh"
    }
}

/// Get shell arguments for command execution
pub(crate) fn get_shell_args(use_shell: bool, shell_path: Option<&str>) -> Vec<String> {
    if !use_shell {
        return Vec::new();
    }

    let shell = shell_path.unwrap_or(get_default_shell());
    
    if cfg!(target_os = "windows") {
        vec![shell.to_string(), "/C".to_string()]
    } else {
        vec![shell.to_string(), "-c".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::info::ProcessState;

    #[test]
    fn test_slay_options_default() {
        let opts = SlayOptions::default();
        assert_eq!(opts.buffer_size, 8192);
        assert!(opts.collect_output);
        assert!(!opts.use_shell);
    }

    #[test]
    fn test_signal_options_default() {
        let opts = SignalOptions::default();
        assert_eq!(opts.grace_period, Duration::from_secs(5));
        assert!(!opts.force);
        assert_eq!(opts.signal, 15);
    }

    #[test]
    fn test_process_stats_default() {
        let stats = ProcessStats::default();
        assert_eq!(stats.cpu, 0.0);
        assert_eq!(stats.memory, 0);
        assert_eq!(stats.thread_count, 0);
    }

    #[test]
    fn test_get_default_shell() {
        let shell = get_default_shell();
        if cfg!(target_os = "windows") {
            assert_eq!(shell, "cmd");
        } else {
            assert_eq!(shell, "/bin/sh");
        }
    }

    #[test]
    fn test_get_shell_args() {
        let args = get_shell_args(false, None);
        assert!(args.is_empty());

        let args = get_shell_args(true, None);
        assert!(!args.is_empty());
        
        if cfg!(target_os = "windows") {
            assert!(args.contains(&"/C".to_string()));
        } else {
            assert!(args.contains(&"-c".to_string()));
        }
    }
}
