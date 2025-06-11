/// Error handling for system operations
use std::fmt;
use std::io;

/// Result type for process operations
pub type ProcessResult<T> = Result<T, ProcessError>;

/// Error types for process operations
#[derive(Debug, Clone)]
pub enum ProcessError {
    /// Process spawning failed
    SpawnError {
        command: String,
        error: String,
    },
    /// Process waiting/monitoring failed
    WaitError {
        pid: u32,
        error: String,
    },
    /// Signal sending/handling failed
    SignalError {
        signal: String,
        target: Option<u32>,
        error: String,
    },
    /// Permission denied
    PermissionError {
        operation: String,
        error: String,
    },
    /// Process not found
    ProcessNotFound {
        pid: u32,
    },
    /// Invalid arguments
    InvalidArguments {
        operation: String,
        details: String,
    },
    /// System operation failed
    SystemError {
        operation: String,
        error: String,
    },
    /// I/O operation failed
    IoError {
        operation: String,
        error: String,
    },
    /// Timeout occurred
    TimeoutError {
        operation: String,
        timeout_ms: u64,
    },
    /// Process terminated abnormally
    ProcessTerminated {
        pid: u32,
        exit_code: Option<i32>,
        signal: Option<String>,
    },
    /// General error with message
    General(String),
}

/// Specific error kinds for different types of process errors
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessErrorKind {
    SpawnFailure,
    WaitFailure, 
    SignalFailure,
    PermissionDenied,
    ProcessNotFound,
    InvalidArguments,
    SystemFailure,
    IoFailure,
    Timeout,
    ProcessTerminated,
    General,
}

impl ProcessError {
    pub fn kind(&self) -> ProcessErrorKind {
        match self {
            ProcessError::SpawnError { .. } => ProcessErrorKind::SpawnFailure,
            ProcessError::WaitError { .. } => ProcessErrorKind::WaitFailure,
            ProcessError::SignalError { .. } => ProcessErrorKind::SignalFailure,
            ProcessError::PermissionError { .. } => ProcessErrorKind::PermissionDenied,
            ProcessError::ProcessNotFound { .. } => ProcessErrorKind::ProcessNotFound,
            ProcessError::InvalidArguments { .. } => ProcessErrorKind::InvalidArguments,
            ProcessError::SystemError { .. } => ProcessErrorKind::SystemFailure,
            ProcessError::IoError { .. } => ProcessErrorKind::IoFailure,
            ProcessError::TimeoutError { .. } => ProcessErrorKind::Timeout,
            ProcessError::ProcessTerminated { .. } => ProcessErrorKind::ProcessTerminated,
            ProcessError::General(_) => ProcessErrorKind::General,
        }
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::SpawnError { command, error } => {
                write!(f, "Failed to spawn process '{}': {}", command, error)
            }
            ProcessError::WaitError { pid, error } => {
                write!(f, "Failed to wait for process {}: {}", pid, error)
            }
            ProcessError::SignalError { signal, target, error } => {
                if let Some(pid) = target {
                    write!(f, "Failed to send signal {} to process {}: {}", signal, pid, error)
                } else {
                    write!(f, "Failed to handle signal {}: {}", signal, error)
                }
            }
            ProcessError::PermissionError { operation, error } => {
                write!(f, "Permission denied for operation '{}': {}", operation, error)
            }
            ProcessError::ProcessNotFound { pid } => {
                write!(f, "Process with PID {} not found", pid)
            }
            ProcessError::InvalidArguments { operation, details } => {
                write!(f, "Invalid arguments for operation '{}': {}", operation, details)
            }
            ProcessError::SystemError { operation, error } => {
                write!(f, "System error during operation '{}': {}", operation, error)
            }
            ProcessError::IoError { operation, error } => {
                write!(f, "I/O error during operation '{}': {}", operation, error)
            }
            ProcessError::TimeoutError { operation, timeout_ms } => {
                write!(f, "Operation '{}' timed out after {}ms", operation, timeout_ms)
            }
            ProcessError::ProcessTerminated { pid, exit_code, signal } => {
                match (exit_code, signal) {
                    (Some(code), None) => write!(f, "Process {} terminated with exit code {}", pid, code),
                    (None, Some(sig)) => write!(f, "Process {} terminated by signal {}", pid, sig),
                    (Some(code), Some(sig)) => write!(f, "Process {} terminated with exit code {} by signal {}", pid, code, sig),
                    (None, None) => write!(f, "Process {} terminated abnormally", pid),
                }
            }
            ProcessError::General(msg) => write!(f, "Process error: {}", msg),
        }
    }
}

impl std::error::Error for ProcessError {}

impl From<io::Error> for ProcessError {
    fn from(error: io::Error) -> Self {
        ProcessError::IoError {
            operation: "I/O operation".to_string(),
            error: error.to_string(),
        }
    }
}

// Helper functions for creating specific error types
pub fn spawn_error(command: &str, error: &str) -> ProcessError {
    ProcessError::SpawnError {
        command: command.to_string(),
        error: error.to_string(),
    }
}

pub fn wait_error(pid: u32, error: &str) -> ProcessError {
    ProcessError::WaitError {
        pid,
        error: error.to_string(),
    }
}

pub fn signal_error(signal: &str, target: Option<u32>, error: &str) -> ProcessError {
    ProcessError::SignalError {
        signal: signal.to_string(),
        target,
        error: error.to_string(),
    }
}

pub fn permission_error(operation: &str, error: &str) -> ProcessError {
    ProcessError::PermissionError {
        operation: operation.to_string(),
        error: error.to_string(),
    }
}

pub fn process_not_found(pid: u32) -> ProcessError {
    ProcessError::ProcessNotFound { pid }
}

pub fn invalid_arguments(operation: &str, details: &str) -> ProcessError {
    ProcessError::InvalidArguments {
        operation: operation.to_string(),
        details: details.to_string(),
    }
}

pub fn system_error(operation: &str, error: &str) -> ProcessError {
    ProcessError::SystemError {
        operation: operation.to_string(),
        error: error.to_string(),
    }
}

pub fn io_error(operation: &str, error: &str) -> ProcessError {
    ProcessError::IoError {
        operation: operation.to_string(),
        error: error.to_string(),
    }
}

pub fn timeout_error(operation: &str, timeout_ms: u64) -> ProcessError {
    ProcessError::TimeoutError {
        operation: operation.to_string(),
        timeout_ms,
    }
}

pub fn process_terminated(pid: u32, exit_code: Option<i32>, signal: Option<&str>) -> ProcessError {
    ProcessError::ProcessTerminated {
        pid,
        exit_code,
        signal: signal.map(|s| s.to_string()),
    }
}

pub fn general_error(message: &str) -> ProcessError {
    ProcessError::General(message.to_string())
}
