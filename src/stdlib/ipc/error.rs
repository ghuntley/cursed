use crate::error::CursedError;
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
    
    /// Message queue error
    MessageQueueError {
    
    /// Shared memory error
    SharedMemoryError {
    
    /// Semaphore error
    SemaphoreError {
    
    /// Unix socket error
    UnixSocketError {
    
    /// Permission denied
    PermissionDenied {
    
    /// Resource already exists
    AlreadyExists {
    
    /// Resource not found
    NotFound {
    
    /// Operation timed out
    Timeout {
    
    /// Invalid configuration
    InvalidConfig {
    
    /// Buffer size error
    BufferSize {
    
    /// Connection error
    ConnectionError {
    
    /// Protocol error
    ProtocolError {
    
    /// System-level error
    SystemError {
    
    /// I/O error during IPC operations
    IoError {
    
    /// Platform-specific error
    PlatformError {
    
    /// General IPC error
    General {
impl IpcError {
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
// impl fmt::Display for IpcError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             IpcError::NamedPipeError { pipe_name, operation, message } => {
//                 write!(f, "Named pipe error in {} for '{}': {}", operation, pipe_name, message)
//             }
//             IpcError::MessageQueueError { queue_name, operation, message } => {
//                 write!(f, "Message queue error in {}", operation)?;
//                 if let Some(name) = queue_name {
//                     write!(f, " for '{}'", name)?;
//                 }
//                 write!(f, ": {}", message)
//             }
//             IpcError::SharedMemoryError { segment_name, operation, message } => {
//                 write!(f, "Shared memory error in {}", operation)?;
//                 if let Some(name) = segment_name {
//                     write!(f, " for '{}'", name)?;
//                 }
//                 write!(f, ": {}", message)
//             }
//             IpcError::SemaphoreError { semaphore_name, operation, message } => {
//                 write!(f, "Semaphore error in {}", operation)?;
//                 if let Some(name) = semaphore_name {
//                     write!(f, " for '{}'", name)?;
//                 }
//                 write!(f, ": {}", message)
//             }
//             IpcError::UnixSocketError { path, operation, message } => {
//                 write!(f, "Unix socket error in {}", operation)?;
//                 if let Some(p) = path {
//                     write!(f, " for '{}'", p)?;
//                 }
//                 write!(f, ": {}", message)
//             }
//             IpcError::PermissionDenied { resource, operation, message } => {
//                 write!(f, "Permission denied for {} in {}: {}", resource, operation, message)
//             }
//             IpcError::AlreadyExists { resource_type, name, message } => {
//                 write!(f, "{} '{}' already exists: {}", resource_type, name, message)
//             }
//             IpcError::NotFound { resource_type, name, message } => {
//                 write!(f, "{} '{}' not found: {}", resource_type, name, message)
//             }
//             IpcError::Timeout { operation, duration, message } => {
//                 write!(f, "Timeout in {} after {:?}: {}", operation, duration, message)
//             }
//             IpcError::InvalidConfig { parameter, value, message } => {
//                 write!(f, "Invalid configuration '{}': '{}' - {}", parameter, value, message)
//             }
//             IpcError::BufferSize { requested, available, message } => {
//                 write!(f, "Buffer size error: requested {}, available {} - {}", requested, available, message)
//             }
//             IpcError::ConnectionError { endpoint, message } => {
//                 write!(f, "Connection error to '{}': {}", endpoint, message)
//             }
//             IpcError::ProtocolError { expected, received, message } => {
//                 write!(f, "Protocol error: expected {}, received {} - {}", expected, received, message)
//             }
//             IpcError::SystemError { code, operation, message } => {
//                 write!(f, "System error in {} (code {}): {}", operation, code, message)
//             }
//             IpcError::IoError { operation, kind, message } => {
//                 write!(f, "I/O error in {} ({}): {}", operation, kind, message)
//             }
//             IpcError::PlatformError { platform, feature, message } => {
//                 write!(f, "Platform error on {} ({}): {}", platform, feature, message)
//             }
//             IpcError::General { message } => {
//                 write!(f, "IPC error: {}", message)
//             }
//         }
//     }
// }

// impl std::error::CursedError for IpcError {
//     fn source(&self) -> Option<&(dyn std::error::CursedError + 'static)> {
//         None
//     }
// }

// impl From<std::io::Error> for IpcError {
//     fn from(error: std::io::Error) -> Self {
//         IpcError::IoError {
//             operation: "unknown".to_string(),
//             kind: format!("{:?}", error.kind()),
//             message: error.to_string(),
//         }
//     }
// }

/// CursedError creation helper functions

/// Create a named pipe error
pub fn named_pipe_error(pipe_name: &str, operation: &str, message: &str) -> IpcError {
    IpcError::NamedPipeError {
    }
}

/// Create a message queue error
pub fn message_queue_error(queue_name: Option<&str>, operation: &str, message: &str) -> IpcError {
    IpcError::MessageQueueError {
    }
}

/// Create a shared memory error
pub fn shared_memory_error(segment_name: Option<&str>, operation: &str, message: &str) -> IpcError {
    IpcError::SharedMemoryError {
    }
}

/// Create a semaphore error
pub fn semaphore_error(semaphore_name: Option<&str>, operation: &str, message: &str) -> IpcError {
    IpcError::SemaphoreError {
    }
}

/// Create a unix socket error
pub fn unix_socket_error(path: Option<&str>, operation: &str, message: &str) -> IpcError {
    IpcError::UnixSocketError {
    }
}

/// Create a permission denied error
pub fn permission_denied(resource: &str, operation: &str, message: &str) -> IpcError {
    IpcError::PermissionDenied {
    }
}

/// Create an already exists error
pub fn already_exists(resource_type: &str, name: &str, message: &str) -> IpcError {
    IpcError::AlreadyExists {
    }
}

/// Create a not found error
pub fn not_found(resource_type: &str, name: &str, message: &str) -> IpcError {
    IpcError::NotFound {
    }
}

/// Create a timeout error
pub fn timeout_error(operation: &str, duration: Duration, message: &str) -> IpcError {
    IpcError::Timeout {
    }
}

/// Create a system error
pub fn system_error(code: i32, operation: &str, message: &str) -> IpcError {
    IpcError::SystemError {
    }
}

/// Create a platform error
pub fn platform_error(feature: &str, message: &str) -> IpcError {
    IpcError::PlatformError {
    }
}

/// Create a connection error
pub fn connection_error(endpoint: &str, message: &str) -> IpcError {
    IpcError::ConnectionError {
    }
}

/// Create an invalid operation error
pub fn invalid_operation(operation: &str, message: &str) -> IpcError {
    IpcError::General {
    }
}

/// Create a general IPC error
pub fn ipc_error(message: &str) -> IpcError {
    IpcError::General {
    }
}

/// Create an out of resources error
pub fn out_of_resources(resource: &str, message: &str) -> IpcError {
    IpcError::General {
    }
}

