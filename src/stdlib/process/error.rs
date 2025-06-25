use crate::error::CursedError;
/// Process management error types for CURSED
/// 
/// This module provides comprehensive error handling for process management operations,
/// including spawning, communication, monitoring, and control.

use std::fmt;
use std::io;
use std::num;
use std::time::Duration;

/// Result type for process operations
pub type ProcessResult<T> = std::result::Result<T, ProcessError>;

/// Comprehensive error types for process management
#[derive(Debug, Clone)]
pub enum ProcessError {
    /// Process not found error
    ProcessNotFound {
    
    /// Permission denied for process operation
    PermissionDenied {
    
    /// Invalid process state for operation
    InvalidState {
    
    /// Process execution failed
    ExecutionFailed {
    
    /// Operation timed out
    Timeout {
    
    /// Invalid arguments provided
    InvalidArguments {
    
    /// Environment variable error
    EnvironmentError {
    
    /// Communication error with process
    CommunicationError {
    
    /// System-level error
    SystemError {
    
    /// I/O error during process operations
    IoError {
    
    /// Signal handling error
    SignalError {
    
    /// Resource limit exceeded
    ResourceLimitExceeded {
    
    /// Platform-specific error
    PlatformError {
    
    /// Generic process error
    General {
impl ProcessError {
    /// Get error message
    pub fn message(&self) -> &str {
        match self {
        }
    }
    
    /// Get error category
    pub fn category(&self) -> &'static str {
        match self {
        }
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
        }
    }
// impl fmt::Display for ProcessError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ProcessError::ProcessNotFound { pid, name, message } => {
//                 write!(f, "Process not found")?;
//                 if let Some(pid) = pid {
//                     write!(f, " (PID: {})", pid)?;
//                 }
//                 if let Some(name) = name {
//                     write!(f, " (name: {})", name)?;
//                 }
//                 write!(f, ": {}", message)
//             }
//             ProcessError::PermissionDenied { operation, pid, message } => {
//                 write!(f, "Permission denied for {}", operation)?;
//                 if let Some(pid) = pid {
//                     write!(f, " (PID: {})", pid)?;
//                 }
//                 write!(f, ": {}", message)
//             }
//             ProcessError::InvalidState { expected, actual, pid } => {
//                 write!(f, "Invalid process state for PID {}: expected {}, got {}", pid, expected, actual)
//             }
//             ProcessError::ExecutionFailed { command, exit_code, stderr, message } => {
//                 write!(f, "Execution failed for '{}': {}", command, message)?;
//                 if let Some(code) = exit_code {
//                     write!(f, " (exit code: {})", code)?;
//                 }
//                 if let Some(err) = stderr {
//                     write!(f, " (stderr: {})", err)?;
//                 }
//                 Ok(())
//             }
//             ProcessError::Timeout { operation, duration, message } => {
//                 write!(f, "Timeout in {} after {:?}: {}", operation, duration, message)
//             }
//             ProcessError::InvalidArguments { operation, argument, message } => {
//                 write!(f, "Invalid argument '{}' for {}: {}", argument, operation, message)
//             }
//             ProcessError::EnvironmentError { variable, operation, message } => {
//                 write!(f, "Environment error in {}", operation)?;
//                 if let Some(var) = variable {
//                     write!(f, " (variable: {})", var)?;
//                 }
//                 write!(f, ": {}", message)
//             }
//             ProcessError::CommunicationError { operation, error_type, message } => {
//                 write!(f, "Communication error in {} ({}): {}", operation, error_type, message)
//             }
//             ProcessError::SystemError { code, operation, message } => {
//                 write!(f, "System error in {} (code {}): {}", operation, code, message)
//             }
//             ProcessError::IoError { operation, kind, message } => {
//                 write!(f, "I/O error in {} ({}): {}", operation, kind, message)
//             }
//             ProcessError::SignalError { signal, operation, message } => {
//                 write!(f, "Signal error for {} in {}: {}", signal, operation, message)
//             }
//             ProcessError::ResourceLimitExceeded { resource, limit, current, message } => {
//                 write!(f, "Resource limit exceeded for {}: {} / {} - {}", resource, current, limit, message)
//             }
//             ProcessError::PlatformError { platform, feature, message } => {
//                 write!(f, "Platform error on {}", platform)?;
//                 if let Some(feat) = feature {
//                     write!(f, " (feature: {})", feat)?;
//                 }
//                 write!(f, ": {}", message)
//             }
//             ProcessError::General { message } => {
//                 write!(f, "Process error: {}", message)
//             }
//         }
//     }
// }

// impl std::error::CursedError for ProcessError {
//     fn source(&self) -> Option<&(dyn std::error::CursedError + 'static)> {
//         None
//     }
// }

// impl From<std::io::Error> for ProcessError {
//     fn from(error: std::io::Error) -> Self {
//         ProcessError::IoError {
//             operation: "unknown".to_string(),
//             kind: format!("{:?}", error.kind()),
//             message: error.to_string(),
//         }
//     }
// }

impl From<num::ParseIntError> for ProcessError {
    fn from(error: num::ParseIntError) -> Self {
        ProcessError::InvalidArguments {
        }
    }
/// CursedError creation helper functions

/// Create a process not found error
pub fn process_not_found(message: &str) -> ProcessError {
    ProcessError::ProcessNotFound {
    }
}

/// Create a process not found error with PID
pub fn process_not_found_pid(pid: u32, message: &str) -> ProcessError {
    ProcessError::ProcessNotFound {
    }
}

/// Create a process not found error with name
pub fn process_not_found_name(name: &str, message: &str) -> ProcessError {
    ProcessError::ProcessNotFound {
    }
}

/// Create a permission denied error
pub fn permission_denied(operation: &str, message: &str) -> ProcessError {
    ProcessError::PermissionDenied {
    }
}

/// Create a permission denied error with PID
pub fn permission_denied_pid(operation: &str, pid: u32, message: &str) -> ProcessError {
    ProcessError::PermissionDenied {
    }
}

/// Create an invalid state error
pub fn invalid_state(expected: &str, actual: &str, pid: u32) -> ProcessError {
    ProcessError::InvalidState {
    }
}

/// Create an execution failed error
pub fn execution_failed(command: &str, message: &str) -> ProcessError {
    ProcessError::ExecutionFailed {
    }
}

/// Create an execution failed error with exit code
pub fn execution_failed_with_code(command: &str, exit_code: i32, message: &str) -> ProcessError {
    ProcessError::ExecutionFailed {
    }
}

/// Create an execution failed error with stderr
pub fn execution_failed_with_stderr(command: &str, stderr: &str, message: &str) -> ProcessError {
    ProcessError::ExecutionFailed {
    }
}

/// Create a timeout error
pub fn timeout_error(operation: &str, duration: Duration, message: &str) -> ProcessError {
    ProcessError::Timeout {
    }
}

/// Create an invalid arguments error
pub fn invalid_arguments(operation: &str, argument: &str, message: &str) -> ProcessError {
    ProcessError::InvalidArguments {
    }
}

/// Create an environment error
pub fn environment_error(operation: &str, message: &str) -> ProcessError {
    ProcessError::EnvironmentError {
    }
}

/// Create an environment error with variable name
pub fn environment_error_var(variable: &str, operation: &str, message: &str) -> ProcessError {
    ProcessError::EnvironmentError {
    }
}

/// Create a communication error
pub fn communication_error(operation: &str, error_type: &str, message: &str) -> ProcessError {
    ProcessError::CommunicationError {
    }
}

/// Create a system error
pub fn system_error(code: i32, operation: &str, message: &str) -> ProcessError {
    ProcessError::SystemError {
    }
}

/// Create an I/O error
pub fn io_error(operation: &str, kind: &str, message: &str) -> ProcessError {
    ProcessError::IoError {
    }
}

/// Create a signal error
pub fn signal_error(signal: &str, operation: &str, message: &str) -> ProcessError {
    ProcessError::SignalError {
    }
}

/// Create a resource limit exceeded error
pub fn resource_limit_exceeded(resource: &str, limit: u64, current: u64, message: &str) -> ProcessError {
    ProcessError::ResourceLimitExceeded {
    }
}

/// Create a platform error
pub fn platform_error(message: &str) -> ProcessError {
    ProcessError::PlatformError {
    }
}

/// Create a platform error with feature
pub fn platform_error_feature(feature: &str, message: &str) -> ProcessError {
    ProcessError::PlatformError {
    }
}

/// Create a general error
pub fn general_error(message: &str) -> ProcessError {
    ProcessError::General {
    }
}

