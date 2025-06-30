// use crate::stdlib::web_vibez::SecurityContext;
// use crate::stdlib::process::EnhancedProcess;
use crate::error::CursedError;
// Remove duplicate imports - these are re-exported below
/// Process management and system interaction for CURSED
/// 
/// This module provides comprehensive process management functionality including:
/// - Process spawning and lifecycle management
/// - Environment variable handling and inheritance
/// - Process synchronization and waiting
/// - Working directory management
/// - Process termination and cleanup
/// - Cross-platform compatibility (Linux, macOS, Windows)
/// - Inter-process communication primitives
/// - Process monitoring and health checks
/// - Signal handling and process control
/// - Daemon and background process management

pub mod error;
pub mod core;
pub mod info;
pub mod control;
pub mod enhanced_control;
pub mod communication;
pub mod monitoring;
pub mod platform;
pub mod pipes;
pub mod signals;
pub mod daemon;
pub mod environment;
pub mod lifecycle;
pub mod exec_slay;
pub mod exec_vibez;
pub mod exec_vibez_types;
pub mod exec_vibez_advanced;
pub mod exec_vibez_enhanced;
pub mod real_monitoring;
pub mod integration;
pub mod ipc_integration;
pub mod enhanced_exec_slay;
pub mod pipeline;
pub mod background_tasks;
pub mod shell_commands;
pub mod real_ipc;
pub mod safe_process_management;
pub mod safe_exec_slay;
pub mod enhanced_exec_slay_complete;
pub mod exec_slay_complete;
pub mod enhanced_exec_vibez_complete;
pub mod comprehensive_integration;
pub mod llvm_integration;
pub mod fork;
pub mod resource_limits;
pub mod namespaces;
pub mod privileges;
pub mod unified_process_ipc;

// Platform-specific modules
#[cfg(unix)]
pub mod unix_platform;
#[cfg(windows)]
pub mod windows_platform;
#[cfg(target_os = "macos")]
pub mod macos_platform;
pub mod mmap;

// Re-export core functionality  
pub use core::*;

// Re-export enhanced functionality
pub use enhanced_exec_slay::*;
// // pub use enhanced_exec_slay_complete::*; // Removed to avoid circular dependency // Removed to avoid E0659 conflicts
// // pub use enhanced_exec_vibez_complete::*; // Removed to avoid circular dependency // Removed to avoid E0659 conflicts
// // pub use comprehensive_integration::*; // Removed to avoid circular dependency // Removed to avoid E0659 conflicts
pub use llvm_integration::*;
pub use shell_commands::*;
// Use explicit imports from safe_process_management to avoid conflicts
pub use safe_process_management::{
    SecurityContext, ProcessIsolation, SecurityCheck, ProcessGuard
};
pub use safe_exec_slay::*;

