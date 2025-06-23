/// Process management error types for CURSED
/// 
/// This module provides comprehensive error handling for process management operations,
/// including spawning, communication, monitoring, and control.

use std::fmt;
use std::io;
use std::num;
use std::time::Duration;

/// Result type for process operations
pub type ProcessResult<(), Error>;

/// Comprehensive error types for process management
#[derive(Debug, Clone)]
pub enum ProcessError {
    /// Process not found error
    ProcessNotFound {
        pid: Option<u32>,
        name: Option<String>,
        message: String,
    },
    
    /// Permission denied for process operation
    PermissionDenied {
        operation: String,
        pid: Option<u32>,
        message: String,
    },
    
    /// Invalid process state for operation
    InvalidState {
        expected: String,
        actual: String,
        pid: u32,
    },
    
    /// Process execution failed
    ExecutionFailed {
        command: String,
        exit_code: Option<i32>,
        stderr: Option<String>,
        message: String,
    },
    
    /// Operation timed out
    Timeout {
        operation: String,
        duration: Duration,
        message: String,
    },
    
    /// Invalid arguments provided
    InvalidArguments {
        operation: String,
        argument: String,
        message: String,
    },
    
    /// Environment variable error
    EnvironmentError {
        variable: Option<String>,
        operation: String,
        message: String,
    },
    
    /// Communication error with process
    CommunicationError {
        operation: String,
        error_type: String,
        message: String,
    },
    
    /// System-level error
    SystemError {
        code: i32,
        operation: String,
        message: String,
    },
    
    /// I/O error during process operations
    IoError {
        operation: String,
        kind: String,
        message: String,
    },
    
    /// Signal handling error
    SignalError {
        signal: String,
        operation: String,
        message: String,
    },
    
    /// Resource limit exceeded
    ResourceLimitExceeded {
        resource: String,
        limit: u64,
        current: u64,
        message: String,
    },
    
    /// Platform-specific error
    PlatformError {
        platform: String,
        feature: Option<String>,
        message: String,
    },
    
    /// Generic process error
    General {
        message: String,
    },
}

impl ProcessError {
    /// Get error message
    pub fn message(&self) -> &str {
        match self {
            ProcessError::ProcessNotFound { message, .. } => message,
            ProcessError::PermissionDenied { message, .. } => message,
            ProcessError::InvalidState { .. } => "Invalid process state",
            ProcessError::ExecutionFailed { message, .. } => message,
            ProcessError::Timeout { message, .. } => message,
            ProcessError::InvalidArguments { message, .. } => message,
            ProcessError::EnvironmentError { message, .. } => message,
            ProcessError::CommunicationError { message, .. } => message,
            ProcessError::SystemError { message, .. } => message,
            ProcessError::IoError { message, .. } => message,
            ProcessError::SignalError { message, .. } => message,
            ProcessError::ResourceLimitExceeded { message, .. } => message,
            ProcessError::PlatformError { message, .. } => message,
            ProcessError::General { message } => message,
        }
    }
    
    /// Get error category
    pub fn category(&self) -> &'static str {
        match self {
            ProcessError::ProcessNotFound { .. } => "ProcessNotFound",
            ProcessError::PermissionDenied { .. } => "PermissionDenied",
            ProcessError::InvalidState { .. } => "InvalidState",
            ProcessError::ExecutionFailed { .. } => "ExecutionFailed",
            ProcessError::Timeout { .. } => "Timeout",
            ProcessError::InvalidArguments { .. } => "InvalidArguments",
            ProcessError::EnvironmentError { .. } => "EnvironmentError",
            ProcessError::CommunicationError { .. } => "CommunicationError",
            ProcessError::SystemError { .. } => "SystemError",
            ProcessError::IoError { .. } => "IoError",
            ProcessError::SignalError { .. } => "SignalError",
            ProcessError::ResourceLimitExceeded { .. } => "ResourceLimitExceeded",
            ProcessError::PlatformError { .. } => "PlatformError",
            ProcessError::General { .. } => "General",
        }
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            ProcessError::ProcessNotFound { .. } => false,
            ProcessError::PermissionDenied { .. } => false,
            ProcessError::InvalidState { .. } => true,
            ProcessError::ExecutionFailed { .. } => false,
            ProcessError::Timeout { .. } => true,
            ProcessError::InvalidArguments { .. } => false,
            ProcessError::EnvironmentError { .. } => true,
            ProcessError::CommunicationError { .. } => true,
            ProcessError::SystemError { .. } => false,
            ProcessError::IoError { .. } => true,
            ProcessError::SignalError { .. } => true,
            ProcessError::ResourceLimitExceeded { .. } => true,
            ProcessError::PlatformError { .. } => false,
            ProcessError::General { .. } => false,
        }
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::ProcessNotFound { pid, name, message } => {
                write!(f, "Process not found")?;
                if let Some(pid) = pid {
                    write!(f, " (PID: {})", pid)?;
                }
                if let Some(name) = name {
                    write!(f, " (name: {})", name)?;
                }
                write!(f, ": {}", message)
            }
            ProcessError::PermissionDenied { operation, pid, message } => {
                write!(f, "Permission denied for {}", operation)?;
                if let Some(pid) = pid {
                    write!(f, " (PID: {})", pid)?;
                }
                write!(f, ": {}", message)
            }
            ProcessError::InvalidState { expected, actual, pid } => {
                write!(f, "Invalid process state for PID {}: expected {}, got {}", pid, expected, actual)
            }
            ProcessError::ExecutionFailed { command, exit_code, stderr, message } => {
                write!(f, "Execution failed for '{}': {}", command, message)?;
                if let Some(code) = exit_code {
                    write!(f, " (exit code: {})", code)?;
                }
                if let Some(err) = stderr {
                    write!(f, " (stderr: {})", err)?;
                }
                Ok(())
            }
            ProcessError::Timeout { operation, duration, message } => {
                write!(f, "Timeout in {} after {:?}: {}", operation, duration, message)
            }
            ProcessError::InvalidArguments { operation, argument, message } => {
                write!(f, "Invalid argument '{}' for {}: {}", argument, operation, message)
            }
            ProcessError::EnvironmentError { variable, operation, message } => {
                write!(f, "Environment error in {}", operation)?;
                if let Some(var) = variable {
                    write!(f, " (variable: {})", var)?;
                }
                write!(f, ": {}", message)
            }
            ProcessError::CommunicationError { operation, error_type, message } => {
                write!(f, "Communication error in {} ({}): {}", operation, error_type, message)
            }
            ProcessError::SystemError { code, operation, message } => {
                write!(f, "System error in {} (code {}): {}", operation, code, message)
            }
            ProcessError::IoError { operation, kind, message } => {
                write!(f, "I/O error in {} ({}): {}", operation, kind, message)
            }
            ProcessError::SignalError { signal, operation, message } => {
                write!(f, "Signal error for {} in {}: {}", signal, operation, message)
            }
            ProcessError::ResourceLimitExceeded { resource, limit, current, message } => {
                write!(f, "Resource limit exceeded for {}: {} / {} - {}", resource, current, limit, message)
            }
            ProcessError::PlatformError { platform, feature, message } => {
                write!(f, "Platform error on {}", platform)?;
                if let Some(feat) = feature {
                    write!(f, " (feature: {})", feat)?;
                }
                write!(f, ": {}", message)
            }
            ProcessError::General { message } => {
                write!(f, "Process error: {}", message)
            }
        }
    }
}

impl std::error::Error for ProcessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for ProcessError {
    fn from(error: io::Error) -> Self {
        ProcessError::IoError {
            operation: "unknown".to_string(),
            kind: format!("{:?}", error.kind()),
            message: error.to_string(),
        }
    }
}

impl From<num::ParseIntError> for ProcessError {
    fn from(error: num::ParseIntError) -> Self {
        ProcessError::InvalidArguments {
            operation: "parse_integer".to_string(),
            argument: "number".to_string(),
            message: error.to_string(),
        }
    }
}

/// Error creation helper functions

/// Create a process not found error
pub fn process_not_found(message: &str) -> ProcessError {
    ProcessError::ProcessNotFound {
        pid: None,
        name: None,
        message: message.to_string(),
    }
}

/// Create a process not found error with PID
pub fn process_not_found_pid(pid: u32, message: &str) -> ProcessError {
    ProcessError::ProcessNotFound {
        pid: Some(pid),
        name: None,
        message: message.to_string(),
    }
}

/// Create a process not found error with name
pub fn process_not_found_name(name: &str, message: &str) -> ProcessError {
    ProcessError::ProcessNotFound {
        pid: None,
        name: Some(name.to_string()),
        message: message.to_string(),
    }
}

/// Create a permission denied error
pub fn permission_denied(operation: &str, message: &str) -> ProcessError {
    ProcessError::PermissionDenied {
        operation: operation.to_string(),
        pid: None,
        message: message.to_string(),
    }
}

/// Create a permission denied error with PID
pub fn permission_denied_pid(operation: &str, pid: u32, message: &str) -> ProcessError {
    ProcessError::PermissionDenied {
        operation: operation.to_string(),
        pid: Some(pid),
        message: message.to_string(),
    }
}

/// Create an invalid state error
pub fn invalid_state(expected: &str, actual: &str, pid: u32) -> ProcessError {
    ProcessError::InvalidState {
        expected: expected.to_string(),
        actual: actual.to_string(),
        pid,
    }
}

/// Create an execution failed error
pub fn execution_failed(command: &str, message: &str) -> ProcessError {
    ProcessError::ExecutionFailed {
        command: command.to_string(),
        exit_code: None,
        stderr: None,
        message: message.to_string(),
    }
}

/// Create an execution failed error with exit code
pub fn execution_failed_with_code(command: &str, exit_code: i32, message: &str) -> ProcessError {
    ProcessError::ExecutionFailed {
        command: command.to_string(),
        exit_code: Some(exit_code),
        stderr: None,
        message: message.to_string(),
    }
}

/// Create an execution failed error with stderr
pub fn execution_failed_with_stderr(command: &str, stderr: &str, message: &str) -> ProcessError {
    ProcessError::ExecutionFailed {
        command: command.to_string(),
        exit_code: None,
        stderr: Some(stderr.to_string()),
        message: message.to_string(),
    }
}

/// Create a timeout error
pub fn timeout_error(operation: &str, duration: Duration, message: &str) -> ProcessError {
    ProcessError::Timeout {
        operation: operation.to_string(),
        duration,
        message: message.to_string(),
    }
}

/// Create an invalid arguments error
pub fn invalid_arguments(operation: &str, argument: &str, message: &str) -> ProcessError {
    ProcessError::InvalidArguments {
        operation: operation.to_string(),
        argument: argument.to_string(),
        message: message.to_string(),
    }
}

/// Create an environment error
pub fn environment_error(operation: &str, message: &str) -> ProcessError {
    ProcessError::EnvironmentError {
        variable: None,
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create an environment error with variable name
pub fn environment_error_var(variable: &str, operation: &str, message: &str) -> ProcessError {
    ProcessError::EnvironmentError {
        variable: Some(variable.to_string()),
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a communication error
pub fn communication_error(operation: &str, error_type: &str, message: &str) -> ProcessError {
    ProcessError::CommunicationError {
        operation: operation.to_string(),
        error_type: error_type.to_string(),
        message: message.to_string(),
    }
}

/// Create a system error
pub fn system_error(code: i32, operation: &str, message: &str) -> ProcessError {
    ProcessError::SystemError {
        code,
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create an I/O error
pub fn io_error(operation: &str, kind: &str, message: &str) -> ProcessError {
    ProcessError::IoError {
        operation: operation.to_string(),
        kind: kind.to_string(),
        message: message.to_string(),
    }
}

/// Create a signal error
pub fn signal_error(signal: &str, operation: &str, message: &str) -> ProcessError {
    ProcessError::SignalError {
        signal: signal.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a resource limit exceeded error
pub fn resource_limit_exceeded(resource: &str, limit: u64, current: u64, message: &str) -> ProcessError {
    ProcessError::ResourceLimitExceeded {
        resource: resource.to_string(),
        limit,
        current,
        message: message.to_string(),
    }
}

/// Create a platform error
pub fn platform_error(message: &str) -> ProcessError {
    ProcessError::PlatformError {
        platform: std::env::consts::OS.to_string(),
        feature: None,
        message: message.to_string(),
    }
}

/// Create a platform error with feature
pub fn platform_error_feature(feature: &str, message: &str) -> ProcessError {
    ProcessError::PlatformError {
        platform: std::env::consts::OS.to_string(),
        feature: Some(feature.to_string()),
        message: message.to_string(),
    }
}

/// Create a general error
pub fn general_error(message: &str) -> ProcessError {
    ProcessError::General {
        message: message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::stdlib::process::error::ProcessResult;
use crate::stdlib::process::error::ProcessError;

    #[test]
    fn test_error_creation() {
        let err = process_not_found("Process does not exist");
        assert_eq!(err.category(), "ProcessNotFound");
        assert!(!err.is_recoverable());

        let err = permission_denied("kill", "Access denied");
        assert_eq!(err.category(), "PermissionDenied");
        assert!(!err.is_recoverable());

        let err = timeout_error("wait", Duration::from_secs(30), "Process did not respond");
        assert_eq!(err.category(), "Timeout");
        assert!(err.is_recoverable());
    }

    #[test]
    fn test_error_display() {
        let err = process_not_found_pid(1234, "No such process");
        let display = format!("{}", err);
        assert!(display.contains("Process not found"));
        assert!(display.contains("PID: 1234"));
        assert!(display.contains("No such process"));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let proc_err: ProcessError = io_err.into();
        assert_eq!(proc_err.category(), "IoError");
    }

    #[test]
    fn test_execution_failed_variants() {
        let err1 = execution_failed("ls", "Command not found");
        let err2 = execution_failed_with_code("ls", 127, "Command not found");
        let err3 = execution_failed_with_stderr("ls", "No such file", "Failed");

        assert_eq!(err1.category(), "ExecutionFailed");
        assert_eq!(err2.category(), "ExecutionFailed");
        assert_eq!(err3.category(), "ExecutionFailed");
    }

    #[test]
    fn test_resource_limit_error() {
        let err = resource_limit_exceeded("memory", 1024*1024, 2*1024*1024, "Memory limit exceeded");
        assert_eq!(err.category(), "ResourceLimitExceeded");
        assert!(err.is_recoverable());
        
        let display = format!("{}", err);
        assert!(display.contains("memory"));
        assert!(display.contains("1048576"));
        assert!(display.contains("2097152"));
    }
}
