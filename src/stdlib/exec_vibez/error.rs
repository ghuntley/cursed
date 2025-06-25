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
        command: String,
        message: String,
        exit_code: Option<i32>,
    },
    /// Invalid arguments provided
    InvalidArguments {
        function: String,
        parameter: String,
        message: String,
    },
    /// I/O error occurred
    IoError {
        operation: String,
        kind: String,
        message: String,
    },
    /// Timeout occurred
    Timeout {
        command: String,
        timeout: std::time::Duration,
    },
    /// Permission denied
    PermissionDenied {
        operation: String,
        resource: String,
    },
    /// Command not found
    CommandNotFound {
        command: String,
        search_paths: Vec<String>,
    },
    /// Process not found
    ProcessNotFound {
        pid: u32,
    },
    /// Environment error
    EnvironmentError {
        variable: String,
        message: String,
    },
    /// Platform-specific error
    PlatformError {
        platform: String,
        message: String,
    },
    /// Resource limit exceeded
    ResourceLimitExceeded {
        resource: String,
        limit: u64,
        current: u64,
    },
    /// Security violation
    SecurityViolation {
        operation: String,
        message: String,
    },
    /// General system error
    SystemError {
        code: i32,
        operation: String,
        message: String,
    },
}

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
        command: command.to_string(),
        message: message.to_string(),
        exit_code: None,
    }
}

/// Create an execution failed error with exit code
pub fn execution_failed_with_code(command: &str, exit_code: i32, message: &str) -> ExecError {
    ExecError::ExecutionFailed {
        command: command.to_string(),
        message: message.to_string(),
        exit_code: Some(exit_code),
    }
}

/// Create an invalid arguments error
pub fn invalid_arguments(function: &str, parameter: &str, message: &str) -> ExecError {
    ExecError::InvalidArguments {
        function: function.to_string(),
        parameter: parameter.to_string(),
        message: message.to_string(),
    }
}

/// Create an I/O error
pub fn io_error(operation: &str, kind: &str, message: &str) -> ExecError {
    ExecError::IoError {
        operation: operation.to_string(),
        kind: kind.to_string(),
        message: message.to_string(),
    }
}

/// Create a timeout error
pub fn timeout_error(command: &str, timeout: std::time::Duration) -> ExecError {
    ExecError::Timeout {
        command: command.to_string(),
        timeout,
    }
}

/// Create a permission denied error
pub fn permission_denied(operation: &str, resource: &str) -> ExecError {
    ExecError::PermissionDenied {
        operation: operation.to_string(),
        resource: resource.to_string(),
    }
}

/// Create a command not found error
pub fn command_not_found(command: &str, search_paths: Vec<String>) -> ExecError {
    ExecError::CommandNotFound {
        command: command.to_string(),
        search_paths,
    }
}

/// Create a process not found error
pub fn process_not_found(pid: u32) -> ExecError {
    ExecError::ProcessNotFound { pid }
}

/// Create an environment error
pub fn environment_error(variable: &str, message: &str) -> ExecError {
    ExecError::EnvironmentError {
        variable: variable.to_string(),
        message: message.to_string(),
    }
}

/// Create a platform error
pub fn platform_error(platform: &str, message: &str) -> ExecError {
    ExecError::PlatformError {
        platform: platform.to_string(),
        message: message.to_string(),
    }
}

/// Create a resource limit exceeded error
pub fn resource_limit_exceeded(resource: &str, limit: u64, current: u64) -> ExecError {
    ExecError::ResourceLimitExceeded {
        resource: resource.to_string(),
        limit,
        current,
    }
}

/// Create a security violation error
pub fn security_violation(operation: &str, message: &str) -> ExecError {
    ExecError::SecurityViolation {
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a system error
pub fn system_error(code: i32, operation: &str, message: &str) -> ExecError {
    ExecError::SystemError {
        code,
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// CursedError trait implementation for integration with CURSED error system
impl ExecError {
    /// Get error code if available
    pub fn exit_code(&self) -> Option<i32> {
        match self {
            ExecError::ExecutionFailed { exit_code, .. } => *exit_code,
            ExecError::SystemError { code, .. } => Some(*code),
            _ => None,
        }
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            ExecError::Timeout { .. } => true,
            ExecError::IoError { .. } => true,
            ExecError::ExecutionFailed { exit_code, .. } => {
                // Non-zero exit codes might be recoverable depending on the application
                exit_code.map_or(false, |code| code != -1)
            }
            ExecError::PermissionDenied { .. } => false,
            ExecError::CommandNotFound { .. } => false,
            ExecError::SecurityViolation { .. } => false,
            _ => true,
        }
    }
    
    /// Get error category for logging and handling
    pub fn category(&self) -> &'static str {
        match self {
            ExecError::ExecutionFailed { .. } => "execution",
            ExecError::InvalidArguments { .. } => "arguments",
            ExecError::IoError { .. } => "io",
            ExecError::Timeout { .. } => "timeout",
            ExecError::PermissionDenied { .. } => "permission",
            ExecError::CommandNotFound { .. } => "not_found",
            ExecError::ProcessNotFound { .. } => "not_found",
            ExecError::EnvironmentError { .. } => "environment",
            ExecError::PlatformError { .. } => "platform",
            ExecError::ResourceLimitExceeded { .. } => "resource",
            ExecError::SecurityViolation { .. } => "security",
            ExecError::SystemError { .. } => "system",
        }
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