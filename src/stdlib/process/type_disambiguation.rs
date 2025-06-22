//! Type disambiguation for process management
//! 
//! This module provides type aliases and clarifications to resolve
//! naming conflicts between different process management modules.

/// Runtime process information (from runtime::process)
pub type RuntimeProcessInfo = crate::runtime::process::ProcessInfo;

/// Standard library process information (from stdlib::process::info)
pub type StdProcessInfo = crate::stdlib::process::info::ProcessInfo;

/// Runtime process status (from runtime::process)
pub type RuntimeProcessStatus = crate::runtime::process::ProcessStatus;

/// Standard library process state (from stdlib::process::info)  
pub type StdProcessState = crate::stdlib::process::info::ProcessState;

/// Runtime process output (from runtime::process)
pub type RuntimeProcessOutput = crate::runtime::process::ProcessOutput;

/// Exec slay process (from stdlib::exec_slay)
pub type SlayProcess = crate::stdlib::exec_slay::SlayProcess;

/// Signal type disambiguation
pub type RuntimeSignal = crate::runtime::process::Signal;
pub type ProcessSignal = crate::stdlib::process::signals::SignalType;

/// Resource limits disambiguation  
pub type RuntimeResourceLimits = crate::runtime::process::ResourceLimits;
pub type ProcessResourceLimits = crate::stdlib::process::resource_limits::ResourceLimits;

/// Security context disambiguation
pub type RuntimeSecurityContext = crate::runtime::process::SecurityContext;
pub type ProcessSecurityContext = crate::stdlib::process::privileges::SecurityContext;

/// Process group disambiguation
pub type RuntimeProcessGroup = crate::runtime::process::ProcessGroup;
pub type ProcessGroup = crate::stdlib::process::core::ProcessGroup;

/// Enhanced process disambiguation
pub type RuntimeEnhancedProcess = crate::runtime::process::EnhancedProcess;
pub type ProcessEnhancedProcess = crate::stdlib::process::enhanced_control::EnhancedProcess;
