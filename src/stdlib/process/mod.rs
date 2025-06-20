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

// Re-export enhanced functionality
pub use enhanced_exec_slay::*;
pub use enhanced_exec_slay_complete::*;
pub use enhanced_exec_vibez_complete::*;
pub use comprehensive_integration::*;
pub use llvm_integration::*;
pub use shell_commands::*;
pub use safe_process_management::*;
pub use safe_exec_slay::*;

#[cfg(test)]
pub mod tests;

// Re-export public API
pub use error::*;
pub use core::*;
pub use info::*;
pub use control::*;
pub use enhanced_control::*;
pub use communication::*;
pub use monitoring::*;
pub use platform::*;
pub use pipes::*;
pub use signals::*;
pub use daemon::*;
pub use environment::*;
pub use lifecycle::*;
pub use exec_slay::*;
// Primary exec_vibez exports (basic module)
pub use exec_vibez::*;
// Enhanced modules for specific features
// pub use exec_vibez_advanced::*;
// pub use exec_vibez_enhanced::*;
pub use real_monitoring::*;
pub use integration::*;
pub use ipc_integration::*;
pub use enhanced_exec_slay::*;
pub use enhanced_exec_slay_complete::*;
pub use enhanced_exec_vibez_complete::*;
pub use comprehensive_integration::*;
pub use pipeline::*;
pub use background_tasks::*;
pub use shell_commands::*;
pub use real_ipc::*;
pub use fork::*;
pub use resource_limits::*;
pub use namespaces::*;
pub use privileges::*;
pub use mmap::*;

// Unified process-IPC system exports
pub use unified_process_ipc::{
    UnifiedProcessIpcManager, UnifiedConfig, IpcConnectionRequest, IpcType,
    ProcessConfig, SecuritySettings, IsolationLevel, PlatformSettings,
    ProcessWithIpc, UnifiedStatus, SecurityStatus, IpcHandle,
    initialize_unified_system, get_unified_manager
};

// Platform-specific exports
#[cfg(unix)]
pub use unix_platform::UnixPlatformHandler;
#[cfg(windows)]
pub use windows_platform::WindowsPlatformHandler;
