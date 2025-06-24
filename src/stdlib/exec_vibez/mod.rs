use crate::error::Error;
/// exec_vibez - Enhanced external command execution for CURSED
/// 
/// This module provides comprehensive functionality for executing external commands
/// and managing external processes, inspired by Go's os/exec but with enhanced features
/// including process groups, timeouts, environment control, output streaming, and input generation.

pub mod core;
pub mod cmd;
pub mod process;
pub mod groups;
pub mod environment;
pub mod streaming;
pub mod timeout;
pub mod error;
pub mod context;
pub mod enhanced;

// Re-export all public APIs
pub use core::*;
pub use cmd::{Cmd, Command, CommandContext};
pub use process::{Process, ProcessState as ExecProcessState}; // Rename to avoid conflict
pub use groups::{ProcessGroup, ProcessGroupOptions, NewProcessGroup};
pub use environment::{Environment, NewEnvironment, CommandWithEnv};
pub use streaming::{OutputStreamer, NewOutputStreamer, InputGenerator, NewInputGenerator};
pub use timeout::{RunWithTimeout, TimeoutConfig};
pub use error::{Error as ExecError, ExecResult};
pub use context::{ProcessContext, VibeContext};
pub use enhanced::{
    // Enhanced features
    LookPath, ProcessMonitor, ResourceLimits, SecurityOptions,
    // Process coordination
    ProcessPool, ProcessQueue, BatchRunner,
    // Platform utilities
    PlatformFeatures, CrossPlatformUtils
};

// Module initialization
use std::sync::Once;
use crate::stdlib::process::info::ProcessState;

static INIT: Once = Once::new();

/// Initialize the exec_vibez module
pub fn initialize() -> ExecResult<()> {
    INIT.call_once(|| {
        // Initialize platform-specific process handling
        #[cfg(unix)]
        {
            // Set up signal handlers for child process management
            unsafe {
                libc::signal(libc::SIGCHLD, libc::SIG_DFL);
            }
        }
        
        tracing::info!("exec_vibez module initialized");
    });
    
    Ok(())
}

/// Get module statistics and status
pub fn get_statistics() -> ModuleStatistics {
    ModuleStatistics {
        active_processes: core::get_active_process_count(),
        total_spawned: core::get_total_spawned_count(),
        process_groups: groups::get_active_group_count(),
        streamers_active: streaming::get_active_streamer_count(),
    }
}

/// Module statistics
#[derive(Debug, Clone)]
pub struct ModuleStatistics {
    pub active_processes: usize,
    pub total_spawned: u64,
    pub process_groups: usize,
    pub streamers_active: usize,
}

#[derive(Debug, Clone)]
pub enum ContextError {
    InvalidContext(String),
    MissingContext,
    ContextSetup(String),
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::InvalidContext(msg) => write!(f, "Invalid context: {}", msg),
            ContextError::MissingContext => write!(f, "Missing context"),
            ContextError::ContextSetup(msg) => write!(f, "Context setup error: {}", msg),
        }
    }
}

impl std::error::Error for ContextError {}
