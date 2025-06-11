/// Process-specific error types and error handling utilities
use std::fmt;
use std::io;
use crate::stdlib::errors_simple::CursedError;

/// Process-specific error types
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessError {
    /// Process not found or doesn't exist
    ProcessNotFound(u32),
    /// Permission denied when accessing or controlling process
    PermissionDenied(String),
    /// Process already running or in invalid state
    InvalidState(String),
    /// Process execution failed
    ExecutionFailed(String),
    /// Process timeout exceeded
    Timeout(String),
    /// Invalid process arguments or configuration
    InvalidArguments(String),
    /// Working directory doesn't exist or is inaccessible
    InvalidWorkingDirectory(String),
    /// Environment variable error
    EnvironmentError(String),
    /// I/O redirection error
    IoRedirectionError(String),
    /// Signal handling error
    SignalError(String),
    /// Process communication error (pipes, IPC)
    CommunicationError(String),
    /// System resource limit exceeded
    ResourceLimitExceeded(String),
    /// Platform-specific error
    PlatformError(String),
    /// General system error with code
    SystemError(i32, String),
    /// Child process management error
    ChildProcessError(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::ProcessNotFound(pid) => {
                write!(f, "Process with PID {} not found", pid)
            }
            ProcessError::PermissionDenied(msg) => {
                write!(f, "Permission denied: {}", msg)
            }
            ProcessError::InvalidState(msg) => {
                write!(f, "Invalid process state: {}", msg)
            }
            ProcessError::ExecutionFailed(msg) => {
                write!(f, "Process execution failed: {}", msg)
            }
            ProcessError::Timeout(msg) => {
                write!(f, "Process operation timed out: {}", msg)
            }
            ProcessError::InvalidArguments(msg) => {
                write!(f, "Invalid process arguments: {}", msg)
            }
            ProcessError::InvalidWorkingDirectory(msg) => {
                write!(f, "Invalid working directory: {}", msg)
            }
            ProcessError::EnvironmentError(msg) => {
                write!(f, "Environment variable error: {}", msg)
            }
            ProcessError::IoRedirectionError(msg) => {
                write!(f, "I/O redirection error: {}", msg)
            }
            ProcessError::SignalError(msg) => {
                write!(f, "Signal handling error: {}", msg)
            }
            ProcessError::CommunicationError(msg) => {
                write!(f, "Process communication error: {}", msg)
            }
            ProcessError::ResourceLimitExceeded(msg) => {
                write!(f, "Resource limit exceeded: {}", msg)
            }
            ProcessError::PlatformError(msg) => {
                write!(f, "Platform-specific error: {}", msg)
            }
            ProcessError::SystemError(code, msg) => {
                write!(f, "System error {}: {}", code, msg)
            }
            ProcessError::ChildProcessError(msg) => {
                write!(f, "Child process error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ProcessError {}

impl From<io::Error> for ProcessError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => {
                ProcessError::ProcessNotFound(0) // Generic not found
            }
            io::ErrorKind::PermissionDenied => {
                ProcessError::PermissionDenied(err.to_string())
            }
            io::ErrorKind::TimedOut => {
                ProcessError::Timeout(err.to_string())
            }
            io::ErrorKind::InvalidInput => {
                ProcessError::InvalidArguments(err.to_string())
            }
            _ => ProcessError::SystemError(
                err.raw_os_error().unwrap_or(-1),
                err.to_string()
            )
        }
    }
}

impl From<ProcessError> for CursedError {
    fn from(err: ProcessError) -> Self {
        CursedError::ProcessError(err.to_string())
    }
}

/// Type alias for Result with ProcessError
pub type ProcessResult<T> = Result<T, ProcessError>;

/// Helper functions for creating common process errors
pub fn process_not_found(pid: u32) -> ProcessError {
    ProcessError::ProcessNotFound(pid)
}

pub fn permission_denied(msg: &str) -> ProcessError {
    ProcessError::PermissionDenied(msg.to_string())
}

pub fn invalid_state(msg: &str) -> ProcessError {
    ProcessError::InvalidState(msg.to_string())
}

pub fn execution_failed(msg: &str) -> ProcessError {
    ProcessError::ExecutionFailed(msg.to_string())
}

pub fn timeout_error(msg: &str) -> ProcessError {
    ProcessError::Timeout(msg.to_string())
}

pub fn invalid_arguments(msg: &str) -> ProcessError {
    ProcessError::InvalidArguments(msg.to_string())
}

pub fn environment_error(msg: &str) -> ProcessError {
    ProcessError::EnvironmentError(msg.to_string())
}

pub fn communication_error(msg: &str) -> ProcessError {
    ProcessError::CommunicationError(msg.to_string())
}

pub fn system_error(code: i32, msg: &str) -> ProcessError {
    ProcessError::SystemError(code, msg.to_string())
}
