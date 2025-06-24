use crate::error::Error;
/// IPC-specific error types for CURSED
/// 
/// This module provides comprehensive error handling for IPC operations

use std::fmt;
use std::io;
use std::time::Duration;

/// Result type for IPC operations
pub type IpcResult<T> = std::result::Result<T, IpcError>;

/// Comprehensive error types for IPC operations
#[derive(Debug, Clone)]
pub enum IpcError {
    /// Named pipe error
    NamedPipeError {
        pipe_name: String,
        operation: String,
        message: String,
    },
    
    /// Message queue error
    MessageQueueError {
        queue_name: Option<String>,
        operation: String,
        message: String,
    },
    
    /// Shared memory error
    SharedMemoryError {
        segment_name: Option<String>,
        operation: String,
        message: String,
    },
    
    /// Semaphore error
    SemaphoreError {
        semaphore_name: Option<String>,
        operation: String,
        message: String,
    },
    
    /// Unix socket error
    UnixSocketError {
        path: Option<String>,
        operation: String,
        message: String,
    },
    
    /// Permission denied
    PermissionDenied {
        resource: String,
        operation: String,
        message: String,
    },
    
    /// Resource already exists
    AlreadyExists {
        resource_type: String,
        name: String,
        message: String,
    },
    
    /// Resource not found
    NotFound {
        resource_type: String,
        name: String,
        message: String,
    },
    
    /// Operation timed out
    Timeout {
        operation: String,
        duration: Duration,
        message: String,
    },
    
    /// Invalid configuration
    InvalidConfig {
        parameter: String,
        value: String,
        message: String,
    },
    
    /// Buffer size error
    BufferSize {
        requested: usize,
        available: usize,
        message: String,
    },
    
    /// Connection error
    ConnectionError {
        endpoint: String,
        message: String,
    },
    
    /// Protocol error
    ProtocolError {
        expected: String,
        received: String,
        message: String,
    },
    
    /// System-level error
    SystemError {
        code: i32,
        operation: String,
        message: String,
    },
    
    /// I/O error during IPC operations
    IoError {
        operation: String,
        kind: String,
        message: String,
    },
    
    /// Platform-specific error
    PlatformError {
        platform: String,
        feature: String,
        message: String,
    },
    
    /// General IPC error
    General {
        message: String,
    },
}

impl IpcError {
    /// Get error message
    pub fn message(&self) -> &str {
        match self {
            IpcError::NamedPipeError { message, .. } => message,
            IpcError::MessageQueueError { message, .. } => message,
            IpcError::SharedMemoryError { message, .. } => message,
            IpcError::SemaphoreError { message, .. } => message,
            IpcError::UnixSocketError { message, .. } => message,
            IpcError::PermissionDenied { message, .. } => message,
            IpcError::AlreadyExists { message, .. } => message,
            IpcError::NotFound { message, .. } => message,
            IpcError::Timeout { message, .. } => message,
            IpcError::InvalidConfig { message, .. } => message,
            IpcError::BufferSize { message, .. } => message,
            IpcError::ConnectionError { message, .. } => message,
            IpcError::ProtocolError { message, .. } => message,
            IpcError::SystemError { message, .. } => message,
            IpcError::IoError { message, .. } => message,
            IpcError::PlatformError { message, .. } => message,
            IpcError::General { message } => message,
        }
    }
    
    /// Get error category
    pub fn category(&self) -> &'static str {
        match self {
            IpcError::NamedPipeError { .. } => "NamedPipeError",
            IpcError::MessageQueueError { .. } => "MessageQueueError",
            IpcError::SharedMemoryError { .. } => "SharedMemoryError",
            IpcError::SemaphoreError { .. } => "SemaphoreError",
            IpcError::UnixSocketError { .. } => "UnixSocketError",
            IpcError::PermissionDenied { .. } => "PermissionDenied",
            IpcError::AlreadyExists { .. } => "AlreadyExists",
            IpcError::NotFound { .. } => "NotFound",
            IpcError::Timeout { .. } => "Timeout",
            IpcError::InvalidConfig { .. } => "InvalidConfig",
            IpcError::BufferSize { .. } => "BufferSize",
            IpcError::ConnectionError { .. } => "ConnectionError",
            IpcError::ProtocolError { .. } => "ProtocolError",
            IpcError::SystemError { .. } => "SystemError",
            IpcError::IoError { .. } => "IoError",
            IpcError::PlatformError { .. } => "PlatformError",
            IpcError::General { .. } => "General",
        }
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            IpcError::NamedPipeError { .. } => true,
            IpcError::MessageQueueError { .. } => true,
            IpcError::SharedMemoryError { .. } => true,
            IpcError::SemaphoreError { .. } => true,
            IpcError::UnixSocketError { .. } => true,
            IpcError::PermissionDenied { .. } => false,
            IpcError::AlreadyExists { .. } => false,
            IpcError::NotFound { .. } => false,
            IpcError::Timeout { .. } => true,
            IpcError::InvalidConfig { .. } => false,
            IpcError::BufferSize { .. } => true,
            IpcError::ConnectionError { .. } => true,
            IpcError::ProtocolError { .. } => true,
            IpcError::SystemError { .. } => false,
            IpcError::IoError { .. } => true,
            IpcError::PlatformError { .. } => false,
            IpcError::General { .. } => false,
        }
    }
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcError::NamedPipeError { pipe_name, operation, message } => {
                write!(f, "Named pipe error in {} for '{}': {}", operation, pipe_name, message)
            }
            IpcError::MessageQueueError { queue_name, operation, message } => {
                write!(f, "Message queue error in {}", operation)?;
                if let Some(name) = queue_name {
                    write!(f, " for '{}'", name)?;
                }
                write!(f, ": {}", message)
            }
            IpcError::SharedMemoryError { segment_name, operation, message } => {
                write!(f, "Shared memory error in {}", operation)?;
                if let Some(name) = segment_name {
                    write!(f, " for '{}'", name)?;
                }
                write!(f, ": {}", message)
            }
            IpcError::SemaphoreError { semaphore_name, operation, message } => {
                write!(f, "Semaphore error in {}", operation)?;
                if let Some(name) = semaphore_name {
                    write!(f, " for '{}'", name)?;
                }
                write!(f, ": {}", message)
            }
            IpcError::UnixSocketError { path, operation, message } => {
                write!(f, "Unix socket error in {}", operation)?;
                if let Some(p) = path {
                    write!(f, " for '{}'", p)?;
                }
                write!(f, ": {}", message)
            }
            IpcError::PermissionDenied { resource, operation, message } => {
                write!(f, "Permission denied for {} in {}: {}", resource, operation, message)
            }
            IpcError::AlreadyExists { resource_type, name, message } => {
                write!(f, "{} '{}' already exists: {}", resource_type, name, message)
            }
            IpcError::NotFound { resource_type, name, message } => {
                write!(f, "{} '{}' not found: {}", resource_type, name, message)
            }
            IpcError::Timeout { operation, duration, message } => {
                write!(f, "Timeout in {} after {:?}: {}", operation, duration, message)
            }
            IpcError::InvalidConfig { parameter, value, message } => {
                write!(f, "Invalid configuration '{}': '{}' - {}", parameter, value, message)
            }
            IpcError::BufferSize { requested, available, message } => {
                write!(f, "Buffer size error: requested {}, available {} - {}", requested, available, message)
            }
            IpcError::ConnectionError { endpoint, message } => {
                write!(f, "Connection error to '{}': {}", endpoint, message)
            }
            IpcError::ProtocolError { expected, received, message } => {
                write!(f, "Protocol error: expected {}, received {} - {}", expected, received, message)
            }
            IpcError::SystemError { code, operation, message } => {
                write!(f, "System error in {} (code {}): {}", operation, code, message)
            }
            IpcError::IoError { operation, kind, message } => {
                write!(f, "I/O error in {} ({}): {}", operation, kind, message)
            }
            IpcError::PlatformError { platform, feature, message } => {
                write!(f, "Platform error on {} ({}): {}", platform, feature, message)
            }
            IpcError::General { message } => {
                write!(f, "IPC error: {}", message)
            }
        }
    }
}

impl std::error::Error for IpcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for IpcError {
    fn from(error: io::Error) -> Self {
        IpcError::IoError {
            operation: "unknown".to_string(),
            kind: format!("{:?}", error.kind()),
            message: error.to_string(),
        }
    }
}

/// Error creation helper functions

/// Create a named pipe error
pub fn named_pipe_error(pipe_name: &str, operation: &str, message: &str) -> IpcError {
    IpcError::NamedPipeError {
        pipe_name: pipe_name.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a message queue error
pub fn message_queue_error(queue_name: Option<&str>, operation: &str, message: &str) -> IpcError {
    IpcError::MessageQueueError {
        queue_name: queue_name.map(|s| s.to_string()),
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a shared memory error
pub fn shared_memory_error(segment_name: Option<&str>, operation: &str, message: &str) -> IpcError {
    IpcError::SharedMemoryError {
        segment_name: segment_name.map(|s| s.to_string()),
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a semaphore error
pub fn semaphore_error(semaphore_name: Option<&str>, operation: &str, message: &str) -> IpcError {
    IpcError::SemaphoreError {
        semaphore_name: semaphore_name.map(|s| s.to_string()),
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a unix socket error
pub fn unix_socket_error(path: Option<&str>, operation: &str, message: &str) -> IpcError {
    IpcError::UnixSocketError {
        path: path.map(|s| s.to_string()),
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a permission denied error
pub fn permission_denied(resource: &str, operation: &str, message: &str) -> IpcError {
    IpcError::PermissionDenied {
        resource: resource.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create an already exists error
pub fn already_exists(resource_type: &str, name: &str, message: &str) -> IpcError {
    IpcError::AlreadyExists {
        resource_type: resource_type.to_string(),
        name: name.to_string(),
        message: message.to_string(),
    }
}

/// Create a not found error
pub fn not_found(resource_type: &str, name: &str, message: &str) -> IpcError {
    IpcError::NotFound {
        resource_type: resource_type.to_string(),
        name: name.to_string(),
        message: message.to_string(),
    }
}

/// Create a timeout error
pub fn timeout_error(operation: &str, duration: Duration, message: &str) -> IpcError {
    IpcError::Timeout {
        operation: operation.to_string(),
        duration,
        message: message.to_string(),
    }
}

/// Create a system error
pub fn system_error(code: i32, operation: &str, message: &str) -> IpcError {
    IpcError::SystemError {
        code,
        operation: operation.to_string(),
        message: message.to_string(),
    }
}

/// Create a platform error
pub fn platform_error(feature: &str, message: &str) -> IpcError {
    IpcError::PlatformError {
        platform: std::env::consts::OS.to_string(),
        feature: feature.to_string(),
        message: message.to_string(),
    }
}

/// Create a connection error
pub fn connection_error(endpoint: &str, message: &str) -> IpcError {
    IpcError::ConnectionError {
        endpoint: endpoint.to_string(),
        message: message.to_string(),
    }
}

/// Create an invalid operation error
pub fn invalid_operation(operation: &str, message: &str) -> IpcError {
    IpcError::General {
        message: format!("Invalid operation '{}': {}", operation, message),
    }
}

/// Create a general IPC error
pub fn ipc_error(message: &str) -> IpcError {
    IpcError::General {
        message: message.to_string(),
    }
}

/// Create an out of resources error
pub fn out_of_resources(resource: &str, message: &str) -> IpcError {
    IpcError::General {
        message: format!("Out of resources '{}': {}", resource, message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = named_pipe_error("test_pipe", "open", "Failed to open pipe");
        assert_eq!(err.category(), "NamedPipeError");
        assert!(err.is_recoverable());

        let err = permission_denied("semaphore", "create", "Access denied");
        assert_eq!(err.category(), "PermissionDenied");
        assert!(!err.is_recoverable());

        let err = timeout_error("read", Duration::from_secs(5), "Operation timed out");
        assert_eq!(err.category(), "Timeout");
        assert!(err.is_recoverable());
    }

    #[test]
    fn test_error_display() {
        let err = message_queue_error(Some("test_queue"), "send", "Queue is full");
        let display = format!("{}", err);
        assert!(display.contains("Message queue error"));
        assert!(display.contains("test_queue"));
        assert!(display.contains("Queue is full"));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let ipc_err: IpcError = io_err.into();
        assert_eq!(ipc_err.category(), "IoError");
    }
}
