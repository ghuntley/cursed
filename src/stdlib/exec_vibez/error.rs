use crate::error::CursedError;
/// CursedError handling for exec_vibez
use std::fmt;
use std::io;

/// Result type for exec_vibez operations
pub type ExecResult<T> = std::result::Result<T, ExecError>;

/// CursedError types for exec_vibez operations
#[derive(Debug, Clone)]
pub enum ExecError {
    /// Command execution failed
    ExecutionFailed {
    /// Invalid arguments provided
    InvalidArguments {
    /// I/O error occurred
    IoError {
    /// Timeout occurred
    Timeout {
    /// Permission denied
    PermissionDenied {
    /// Command not found
    CommandNotFound {
    /// Process not found
    ProcessNotFound {
    /// Environment error
    EnvironmentError {
    /// Platform-specific error
    PlatformError {
    /// Resource limit exceeded
    ResourceLimitExceeded {
    /// Security violation
    SecurityViolation {
    /// General system error
    SystemError {
// impl fmt::Display for ExecError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ExecError::ExecutionFailed { command, message, exit_code } => {
//                 if let Some(code) = exit_code {
//                     write!(f, "Command '{}' failed with exit code {}: {}", command, code, message)
//                 } else {
//                     write!(f, "Command '{}' failed: {}", command, message)
//                 }
//             }
//             ExecError::InvalidArguments { function, parameter, message } => {
//                 write!(f, "Invalid argument '{}' in function '{}': {}", parameter, function, message)
//             }
//             ExecError::IoError { operation, kind, message } => {
//                 write!(f, "I/O error during '{}' ({}): {}", operation, kind, message)
//             }
//             ExecError::Timeout { command, timeout } => {
//                 write!(f, "Command '{}' timed out after {:?}", command, timeout)
//             }
//             ExecError::PermissionDenied { operation, resource } => {
//                 write!(f, "Permission denied for '{}' on resource '{}'", operation, resource)
//             }
//             ExecError::CommandNotFound { command, search_paths } => {
//                 write!(f, "Command '{}' not found in paths: {}", command, search_paths.join(":"))
//             }
//             ExecError::ProcessNotFound { pid } => {
//                 write!(f, "Process with PID {} not found", pid)
//             }
//             ExecError::EnvironmentError { variable, message } => {
//                 write!(f, "Environment variable '{}' error: {}", variable, message)
//             }
//             ExecError::PlatformError { platform, message } => {
//                 write!(f, "Platform-specific error on '{}': {}", platform, message)
//             }
//             ExecError::ResourceLimitExceeded { resource, limit, current } => {
//                 write!(f, "Resource limit exceeded for '{}': {} > {}", resource, current, limit)
//             }
//             ExecError::SecurityViolation { operation, message } => {
//                 write!(f, "Security violation during '{}': {}", operation, message)
//             }
//             ExecError::SystemError { code, operation, message } => {
//                 write!(f, "System error {} during '{}': {}", code, operation, message)
//             }
//         }
//     }
// }

// impl std::error::CursedError for ExecError {
//     fn source(&self) -> Option<&(dyn std::error::CursedError + 'static)> {
//         None
//     }
// }

// impl From<std::io::Error> for ExecError {
//     fn from(error: std::io::Error) -> Self {
//         ExecError::IoError {
//             operation: "unknown".to_string(),
//             kind: format!("{:?}", error.kind()),
//             message: error.to_string(),
//         }
//     }
// }

/// Helper functions for creating specific error types

/// Create an execution failed error
pub fn execution_failed(command: &str, message: &str) -> ExecError {
    ExecError::ExecutionFailed {
    }
}

/// Create an execution failed error with exit code
pub fn execution_failed_with_code(command: &str, exit_code: i32, message: &str) -> ExecError {
    ExecError::ExecutionFailed {
    }
}

/// Create an invalid arguments error
pub fn invalid_arguments(function: &str, parameter: &str, message: &str) -> ExecError {
    ExecError::InvalidArguments {
    }
}

/// Create an I/O error
pub fn io_error(operation: &str, kind: &str, message: &str) -> ExecError {
    ExecError::IoError {
    }
}

/// Create a timeout error
pub fn timeout_error(command: &str, timeout: std::time::Duration) -> ExecError {
    ExecError::Timeout {
    }
}

/// Create a permission denied error
pub fn permission_denied(operation: &str, resource: &str) -> ExecError {
    ExecError::PermissionDenied {
    }
}

/// Create a command not found error
pub fn command_not_found(command: &str, search_paths: Vec<String>) -> ExecError {
    ExecError::CommandNotFound {
    }
}

/// Create a process not found error
pub fn process_not_found(pid: u32) -> ExecError {
    ExecError::ProcessNotFound { pid }
}

/// Create an environment error
pub fn environment_error(variable: &str, message: &str) -> ExecError {
    ExecError::EnvironmentError {
    }
}

/// Create a platform error
pub fn platform_error(platform: &str, message: &str) -> ExecError {
    ExecError::PlatformError {
    }
}

/// Create a resource limit exceeded error
pub fn resource_limit_exceeded(resource: &str, limit: u64, current: u64) -> ExecError {
    ExecError::ResourceLimitExceeded {
    }
}

/// Create a security violation error
pub fn security_violation(operation: &str, message: &str) -> ExecError {
    ExecError::SecurityViolation {
    }
}

/// Create a system error
pub fn system_error(code: i32, operation: &str, message: &str) -> ExecError {
    ExecError::SystemError {
    }
}

/// CursedError trait implementation for integration with CURSED error system
impl ExecError {
    /// Get error code if available
    pub fn exit_code(&self) -> Option<i32> {
        match self {
        }
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            ExecError::ExecutionFailed { exit_code, .. } => {
                // Non-zero exit codes might be recoverable depending on the application
                exit_code.map_or(false, |code| code != -1)
            }
        }
    }
    
    /// Get error category for logging and handling
    pub fn category(&self) -> &'static str {
        match self {
        }
    }

// impl std::fmt::Display for ContextError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ContextError::InvalidContext(msg) => write!(f, "Invalid context: {}", msg),
//             ContextError::MissingContext => write!(f, "Missing context"),
//             ContextError::ContextSetup(msg) => write!(f, "Context setup error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for ContextError {}
// 