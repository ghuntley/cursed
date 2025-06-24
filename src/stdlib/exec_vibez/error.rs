use crate::error::Error as CursedError;
/// Error handling for exec_vibez
use std::fmt;
use std::io;

/// Result type for exec_vibez operations
pub type ExecResult<T> = std::result::Result<T, ExecError>;

/// Error types for exec_vibez operations
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

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecError::ExecutionFailed { command, message, exit_code } => {
                if let Some(code) = exit_code {
                    write!(f, "Command '{}' failed with exit code {}: {}", command, code, message)
                } else {
                    write!(f, "Command '{}' failed: {}", command, message)
                }
            }
            ExecError::InvalidArguments { function, parameter, message } => {
                write!(f, "Invalid argument '{}' in function '{}': {}", parameter, function, message)
            }
            ExecError::IoError { operation, kind, message } => {
                write!(f, "I/O error during '{}' ({}): {}", operation, kind, message)
            }
            ExecError::Timeout { command, timeout } => {
                write!(f, "Command '{}' timed out after {:?}", command, timeout)
            }
            ExecError::PermissionDenied { operation, resource } => {
                write!(f, "Permission denied for '{}' on resource '{}'", operation, resource)
            }
            ExecError::CommandNotFound { command, search_paths } => {
                write!(f, "Command '{}' not found in paths: {}", command, search_paths.join(":"))
            }
            ExecError::ProcessNotFound { pid } => {
                write!(f, "Process with PID {} not found", pid)
            }
            ExecError::EnvironmentError { variable, message } => {
                write!(f, "Environment variable '{}' error: {}", variable, message)
            }
            ExecError::PlatformError { platform, message } => {
                write!(f, "Platform-specific error on '{}': {}", platform, message)
            }
            ExecError::ResourceLimitExceeded { resource, limit, current } => {
                write!(f, "Resource limit exceeded for '{}': {} > {}", resource, current, limit)
            }
            ExecError::SecurityViolation { operation, message } => {
                write!(f, "Security violation during '{}': {}", operation, message)
            }
            ExecError::SystemError { code, operation, message } => {
                write!(f, "System error {} during '{}': {}", code, operation, message)
            }
        }
    }
}

impl std::error::Error for ExecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for ExecError {
    fn from(error: io::Error) -> Self {
        ExecError::IoError {
            operation: "unknown".to_string(),
            kind: format!("{:?}", error.kind()),
            message: error.to_string(),
        }
    }
}

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

/// Error trait implementation for integration with CURSED error system
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_execution_failed_error() {
        let err = execution_failed("test_cmd", "Test failure");
        
        match err {
            ExecError::ExecutionFailed { command, message, exit_code } => {
                assert_eq!(command, "test_cmd");
                assert_eq!(message, "Test failure");
                assert_eq!(exit_code, None);
            }
            _ => panic!("Wrong error type"),
        }
        
        assert_eq!(err.category(), "execution");
        assert!(err.is_recoverable());
    }
    
    #[test]
    fn test_execution_failed_with_code_error() {
        let err = execution_failed_with_code("test_cmd", 1, "Test failure");
        
        match err {
            ExecError::ExecutionFailed { command, message, exit_code } => {
                assert_eq!(command, "test_cmd");
                assert_eq!(message, "Test failure");
                assert_eq!(exit_code, Some(1));
            }
            _ => panic!("Wrong error type"),
        }
        
        assert_eq!(err.exit_code(), Some(1));
    }
    
    #[test]
    fn test_timeout_error() {
        let timeout = Duration::from_secs(30);
        let err = timeout_error("long_cmd", timeout);
        
        match err {
            ExecError::Timeout { command, timeout: t } => {
                assert_eq!(command, "long_cmd");
                assert_eq!(t, timeout);
            }
            _ => panic!("Wrong error type"),
        }
        
        assert_eq!(err.category(), "timeout");
        assert!(err.is_recoverable());
    }
    
    #[test]
    fn test_invalid_arguments_error() {
        let err = invalid_arguments("test_function", "test_param", "Invalid value");
        
        assert_eq!(err.category(), "arguments");
        assert!(err.is_recoverable());
    }
    
    #[test]
    fn test_permission_denied_error() {
        let err = permission_denied("execute", "/usr/bin/sudo");
        
        assert_eq!(err.category(), "permission");
        assert!(!err.is_recoverable());
    }
    
    #[test]
    fn test_command_not_found_error() {
        let paths = vec!["/usr/bin".to_string(), "/bin".to_string()];
        let err = command_not_found("nonexistent", paths);
        
        assert_eq!(err.category(), "not_found");
        assert!(!err.is_recoverable());
    }
    
    #[test]
    fn test_error_display() {
        let err = execution_failed_with_code("test", 1, "failed");
        let display = format!("{}", err);
        assert!(display.contains("test"));
        assert!(display.contains("exit code 1"));
        assert!(display.contains("failed"));
    }
    
    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let exec_err: ExecError = io_err.into();
        
        match exec_err {
            ExecError::IoError { operation, kind, message } => {
                assert_eq!(operation, "unknown");
                assert_eq!(kind, "NotFound");
                assert!(message.contains("File not found"));
            }
            _ => panic!("Wrong error type"),
        }
    }
}


pub type Error = Box<dyn std::error::Error + Send + Sync>;


#[derive(Debug, Clone)]
pub enum ContextError {
    InvalidContext(String),
    MissingContext,
    ContextSetup(String),
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::InvalidContext(msg) => write!(f, "Invalid context: {}", msg),
            ContextError::MissingContext => write!(f, "Missing context"),
            ContextError::ContextSetup(msg) => write!(f, "Context setup error: {}", msg),
        }
    }
}

impl std::error::Error for ContextError {}
