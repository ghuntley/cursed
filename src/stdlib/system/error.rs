/// CursedError handling for system operations
use std::fmt;
use crate::error::CursedError;
use std::io;

/// Result type for process operations
pub type ProcessResult<T> = std::result::Result<T, ProcessError>;

/// CursedError types for process operations
#[derive(Debug, Clone)]
pub enum ProcessError {
    /// Process spawning failed
    SpawnError {
    /// Process waiting/monitoring failed
    WaitError {
    /// Signal sending/handling failed
    SignalError {
    /// Permission denied
    PermissionError {
    /// Process not found
    ProcessNotFound {
    /// Invalid arguments
    InvalidArguments {
    /// System operation failed
    SystemError {
    /// I/O operation failed
    IoError {
    /// Timeout occurred
    TimeoutError {
    /// Process terminated abnormally
    ProcessTerminated {
    /// General error with message
/// Specific error kinds for different types of process errors
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessErrorKind {
impl ProcessError {
    pub fn kind(&self) -> ProcessErrorKind {
        match self {
        }
    }
// impl fmt::Display for ProcessError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ProcessError::SpawnError { command, error } => {
//                 write!(f, "Failed to spawn process '{}': {}", command, error)
//             }
//             ProcessError::WaitError { pid, error } => {
//                 write!(f, "Failed to wait for process {}: {}", pid, error)
//             }
//             ProcessError::SignalError { signal, target, error } => {
//                 if let Some(pid) = target {
//                     write!(f, "Failed to send signal {} to process {}: {}", signal, pid, error)
//                 } else {
//                     write!(f, "Failed to handle signal {}: {}", signal, error)
//                 }
//             }
//             ProcessError::PermissionError { operation, error } => {
//                 write!(f, "Permission denied for operation '{}': {}", operation, error)
//             }
//             ProcessError::ProcessNotFound { pid } => {
//                 write!(f, "Process with PID {} not found", pid)
//             }
//             ProcessError::InvalidArguments { operation, details } => {
//                 write!(f, "Invalid arguments for operation '{}': {}", operation, details)
//             }
//             ProcessError::SystemError { operation, error } => {
//                 write!(f, "System error during operation '{}': {}", operation, error)
//             }
//             ProcessError::IoError { operation, error } => {
//                 write!(f, "I/O error during operation '{}': {}", operation, error)
//             }
//             ProcessError::TimeoutError { operation, timeout_ms } => {
//                 write!(f, "Operation '{}' timed out after {}ms", operation, timeout_ms)
//             }
//             ProcessError::ProcessTerminated { pid, exit_code, signal } => {
//                 match (exit_code, signal) {
//                     (Some(code), None) => write!(f, "Process {} terminated with exit code {}", pid, code),
//                     (None, Some(sig)) => write!(f, "Process {} terminated by signal {}", pid, sig),
//                     (Some(code), Some(sig)) => write!(f, "Process {} terminated with exit code {} by signal {}", pid, code, sig),
//                     (None, None) => write!(f, "Process {} terminated abnormally", pid),
//                 }
//             }
//             ProcessError::General(msg) => write!(f, "Process error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for ProcessError {}
// 
// impl From<std::io::Error> for ProcessError {
//     fn from(error: std::io::Error) -> Self {
//         ProcessError::IoError {
//             operation: "I/O operation".to_string(),
//             error: error.to_string(),
//         }
//     }
// }

// Helper functions for creating specific error types
pub fn spawn_error(command: &str, error: &str) -> ProcessError {
    ProcessError::SpawnError {
    }
}

pub fn wait_error(pid: u32, error: &str) -> ProcessError {
    ProcessError::WaitError {
    }
}

pub fn signal_error(signal: &str, target: Option<u32>, error: &str) -> ProcessError {
    ProcessError::SignalError {
    }
}

pub fn permission_error(operation: &str, error: &str) -> ProcessError {
    ProcessError::PermissionError {
    }
}

pub fn process_not_found(pid: u32) -> ProcessError {
    ProcessError::ProcessNotFound { pid }
}

pub fn invalid_arguments(operation: &str, details: &str) -> ProcessError {
    ProcessError::InvalidArguments {
    }
}

pub fn system_error(operation: &str, error: &str) -> ProcessError {
    ProcessError::SystemError {
    }
}

pub fn io_error(operation: &str, error: &str) -> ProcessError {
    ProcessError::IoError {
    }
}

pub fn timeout_error(operation: &str, timeout_ms: u64) -> ProcessError {
    ProcessError::TimeoutError {
    }
}

pub fn process_terminated(pid: u32, exit_code: Option<i32>, signal: Option<&str>) -> ProcessError {
    ProcessError::ProcessTerminated {
    }
}

pub fn general_error(message: &str) -> ProcessError {
    ProcessError::General(message.to_string())
}
