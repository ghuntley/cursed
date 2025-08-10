use crate::error::CursedError;
// ExecSlay - Process execution module for CURSED
// 
// Provides utilities for running external commands with style and efficiency.
// Inspired by Go's os/exec package but with enhanced features for process management,
// input/output control, and resource monitoring.

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::process::{Child, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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
pub type Slaycrate::error::Result<T> = std::result::Result<T>;

/// Configuration options for command execution
#[derive(Debug, Clone)]
pub struct SlayOptions {
    /// Working directory for the command
    /// Environment variables
    /// Timeout for command execution
    /// Wait delay before killing
    /// Signal to use for killing
    /// Use shell for command execution
    /// Shell path to use
    /// Buffer size for I/O operations
    /// Collect output in memory
    /// Capture environment statistics
    /// Working set memory limit in bytes
    /// CPU usage limit as percentage
impl Default for SlayOptions {
    fn default() -> Self {
        Self {
            buffer_size: 8192, // 8KB default buffer
        }
    }
/// Signal handling options
#[derive(Debug, Clone)]
pub struct SignalOptions {
    /// Grace period before force killing
    /// Force kill immediately
    /// Signal to send
    /// Kill process tree recursively
impl Default for SignalOptions {
    fn default() -> Self {
        Self {
            signal: 15, // SIGTERM
        }
    }
/// Process statistics for monitoring
#[derive(Debug, Clone)]
pub struct ProcessStats {
    /// CPU usage percentage
    /// Memory usage in bytes
    /// Resident memory in bytes
    /// Virtual memory in bytes
    /// Swap memory in bytes
    /// Bytes read from disk
    /// Bytes written to disk
    /// Read operations count
    /// Write operations count
    /// Process uptime
    /// Number of threads
    /// Number of open files
    /// Number of network connections
impl Default for ProcessStats {
    fn default() -> Self {
        Self {
        }
    }
/// Internal shared state for process management
#[derive(Debug)]
pub struct SharedProcessState {
    /// Child process handle
    /// Process start time
    /// Exit status when completed
    /// Collected stdout
    /// Collected stderr
    /// Whether the process is running
    /// Last error encountered
impl SharedProcessState {
    pub fn new() -> Self {
        Self {
        }
    }
/// Convert std::io::Error to CursedError
pub(crate) fn io_error_to_cursed(err: std::io::Error) -> CursedError {
    CursedError::RuntimeError(format!("I/O error: {}", err))
// Public constructor functions for easier API usage

/// Create a new SlayCommand with the given name and arguments
pub fn new_slay_command(name: &str, args: &[&str]) -> SlayCommand {
    SlayCommand::new(name, args)
/// Create a new command builder
pub fn new_slay_command_builder(command: &str) -> SlayCommandBuilder {
    SlayCommandBuilder::new(command)
/// Create a new pipeline with commands
pub fn new_slay_pipeline(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::new(commands)
/// Create a pipeline from commands (convenience function)
pub fn pipe(commands: Vec<SlayCommand>) -> SlayPipeline {
    SlayPipeline::pipe(commands)
/// Run a command in the background
pub fn run_background(command: SlayCommand) -> SlayTask {
    SlayTask::run_background(command)
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
    let shell = shell_path.unwrap_or(get_default_shell());
    
    if cfg!(target_os = "windows") {
        vec![shell.to_string(), "/C".to_string()]
    } else {
        vec![shell.to_string(), "-c".to_string()]
    }
}

