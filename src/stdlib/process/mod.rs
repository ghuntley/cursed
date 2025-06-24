use crate::stdlib::web_vibez::SecurityContext;
use crate::stdlib::process::EnhancedProcess;
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

// Re-export enhanced functionality
pub use enhanced_exec_slay::*;
// // pub use enhanced_exec_slay_complete::*; // Removed to avoid circular dependency // Removed to avoid E0659 conflicts
// // pub use enhanced_exec_vibez_complete::*; // Removed to avoid circular dependency // Removed to avoid E0659 conflicts
// // pub use comprehensive_integration::*; // Removed to avoid circular dependency // Removed to avoid E0659 conflicts
pub use llvm_integration::*;
pub use shell_commands::*;
// Use explicit imports from safe_process_management to avoid conflicts
pub use safe_process_management::{
    SafeProcessManager, ProcessSecurityManager, SafetyConfig, SecurityPolicy, 
    SecurityContextData, SecurityEvent, SecurityLevel,
    ResourceLimits as SafeResourceLimits, ResourceType as SafeResourceType,
    SecurityContext, ProcessIsolation, SecurityCheck, ProcessGuard
};
pub use safe_exec_slay::*;

#[cfg(test)]
pub mod tests;
pub mod type_disambiguation;

// Re-export public API
// Core types - explicit exports to avoid conflicts
pub use error::{ProcessError, ProcessResult};
pub use core::{ProcessManager, ProcessHandle, ProcessConfig, ProcessGroup, ProcessState, ProcessInfo};
pub use info::{ProcessInfo as StdProcessInfo, SystemInfo, ProcessState as StdProcessState};
pub use control::{ProcessController, ControlOptions};
pub use enhanced_control::{EnhancedProcessController, EnhancedControlOptions};
pub use communication::{ProcessCommunication, CommunicationChannel};
pub use monitoring::{ProcessMonitor, MonitoringOptions, ProcessMetrics};
pub use platform::{PlatformHandler, PlatformCapabilities};
pub use pipes::{ProcessPipe, PipeOptions};
pub use signals::{SignalHandler, SignalType};
pub use daemon::{DaemonManager, DaemonOptions};
pub use environment::{EnvironmentManager, EnvVar};
pub use lifecycle::{ProcessLifecycleManager, ManagedProcess, LifecycleEvent};

// Exec modules - prefixed to avoid conflicts
pub use exec_slay::{
    SlayProcess as ProcessSlayProcess,
    SlayCommand as ProcessSlayCommand,
    SlayResult as ProcessSlayResult
};
pub use exec_vibez::{
    VibezProcess as ProcessVibezProcess,
    VibezCommand as ProcessVibezCommand,
    VibezResult as ProcessVibezResult
};
pub use exec_vibez_crate::types::{
    VibezResult, ExecutionContext, EnhancedCmd, ResourceLimits as ExecResourceLimits,
    SecurityContext as ExecSecurityContext, ExecutionMode, Priority
};

// Enhanced modules with prefixes
pub use enhanced_exec_slay::{
    EnhancedSlayProcess,
    EnhancedSlayCommand,
    EnhancedSlayOptions
};

// Monitoring exports
pub use real_monitoring::{
    get_real_cpu_times, get_real_memory_usage, get_real_process_stats,
    RealProcessStats, CpuTimes, MemoryUsage
};

// Integration modules
pub use integration::{ProcessIntegration, IntegrationOptions};
pub use ipc_integration::{IpcIntegration, IpcOptions};
pub use comprehensive_integration::{ComprehensiveProcessManager};

// Pipeline and task management
pub use pipeline::{ProcessPipeline, PipelineStage};
pub use background_tasks::{BackgroundTaskManager, TaskHandle};
pub use shell_commands::{ShellCommandManager, ShellOptions};

// IPC and communication
pub use real_ipc::{RealIpcManager, IpcChannel, IpcMessage};

// System-level operations
pub use fork::{ForkManager, ForkOptions};
pub use resource_limits::{ResourceLimitManager, ResourceLimit as ResourceLimits};
pub use namespaces::{NamespaceManager, NamespaceOptions};
pub use privileges::{PrivilegeManager, PrivilegeOptions};
pub use mmap::{MmapManager, MmapOptions};

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

// Type disambiguation exports
pub use type_disambiguation::*;
