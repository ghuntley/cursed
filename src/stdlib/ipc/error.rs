/// Comprehensive error handling for IPC operations in CURSED
/// 
/// This module provides detailed error types for all inter-process communication
/// operations including shared memory, pipes, message queues, and synchronization.

use std::fmt;
use std::io;
use std::time::Duration;

/// Result type for IPC operations
pub type IpcResult<T> = Result<T, IpcError>;

/// Comprehensive error types for IPC operations
#[derive(Debug, Clone)]
pub enum IpcError {
    /// Communication error during IPC operation
    CommunicationError {
        operation: String,
        error_type: String,
        message: String,
        resource_id: Option<String>,
    },
    
    /// Security-related error
    SecurityError {
        operation: String,
        security_context: String,
        message: String,
        permission_required: Option<String>,
    },
    
    /// Resource-related error (memory, handles, etc.)
    ResourceError {
        resource_type: String,
        operation: String,
        message: String,
        resource_id: Option<String>,
        limit_exceeded: Option<u64>,
    },
    
    /// Operation timeout
    Timeout {
        operation: String,
        duration: Duration,
        message: String,
        resource_id: Option<String>,
    },
    
    /// Invalid operation for current state
    InvalidOperation {
        operation: String,
        current_state: String,
        expected_state: String,
        resource_id: Option<String>,
    },
    
    /// Permission denied for IPC operation
    PermissionDenied {
        operation: String,
        resource_id: String,
        required_permission: String,
        message: String,
    },
    
    /// Resource exhausted (e.g., no more shared memory)
    ResourceExhausted {
        resource_type: String,
        operation: String,
        current_usage: u64,
        max_limit: u64,
        message: String,
    },
    
    /// Connection failed
    ConnectionFailed {
        operation: String,
        target: String,
        error_type: String,
        message: String,
        retry_count: Option<u32>,
    },
    
    /// Serialization/Deserialization error
    SerializationError {
        operation: String,
        data_type: String,
        message: String,
        position: Option<usize>,
    },
    
    /// Protocol error
    ProtocolError {
        protocol: String,
        operation: String,
        message: String,
        error_code: Option<i32>,
    },
    
    /// System-level error
    SystemError {
        operation: String,
        error_code: i32,
        message: String,
        system_call: Option<String>,
    },
    
    /// Data corruption detected
    DataCorruption {
        resource_type: String,
        resource_id: String,
        operation: String,
        message: String,
        checksum_expected: Option<String>,
        checksum_actual: Option<String>,
    },
    
    /// Deadlock detected
    Deadlock {
        operation: String,
        resources_involved: Vec<String>,
        message: String,
        detection_method: String,
    },
    
    /// Configuration error
    ConfigurationError {
        parameter: String,
        value: String,
        message: String,
        valid_range: Option<String>,
    },
    
    /// Generic IPC error
    General {
        message: String,
        error_code: Option<i32>,
    },
}

impl IpcError {
    /// Get error message
    pub fn message(&self) -> &str {
        match self {
            IpcError::CommunicationError { message, .. } => message,
            IpcError::SecurityError { message, .. } => message,
            IpcError::ResourceError { message, .. } => message,
            IpcError::Timeout { message, .. } => message,
            IpcError::InvalidOperation { .. } => "Invalid operation for current state",
            IpcError::PermissionDenied { message, .. } => message,
            IpcError::ResourceExhausted { message, .. } => message,
            IpcError::ConnectionFailed { message, .. } => message,
            IpcError::SerializationError { message, .. } => message,
            IpcError::ProtocolError { message, .. } => message,
            IpcError::SystemError { message, .. } => message,
            IpcError::DataCorruption { message, .. } => message,
            IpcError::Deadlock { message, .. } => message,
            IpcError::ConfigurationError { message, .. } => message,
            IpcError::General { message, .. } => message,
        }
    }
    
    /// Get error category
    pub fn category(&self) -> &'static str {
        match self {
            IpcError::CommunicationError { .. } => "CommunicationError",
            IpcError::SecurityError { .. } => "SecurityError",
            IpcError::ResourceError { .. } => "ResourceError",
            IpcError::Timeout { .. } => "Timeout",
            IpcError::InvalidOperation { .. } => "InvalidOperation",
            IpcError::PermissionDenied { .. } => "PermissionDenied",
            IpcError::ResourceExhausted { .. } => "ResourceExhausted",
            IpcError::ConnectionFailed { .. } => "ConnectionFailed",
            IpcError::SerializationError { .. } => "SerializationError",
            IpcError::ProtocolError { .. } => "ProtocolError",
            IpcError::SystemError { .. } => "SystemError",
            IpcError::DataCorruption { .. } => "DataCorruption",
            IpcError::Deadlock { .. } => "Deadlock",
            IpcError::ConfigurationError { .. } => "ConfigurationError",
            IpcError::General { .. } => "General",
        }
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            IpcError::CommunicationError { .. } => true,
            IpcError::SecurityError { .. } => false,
            IpcError::ResourceError { .. } => true,
            IpcError::Timeout { .. } => true,
            IpcError::InvalidOperation { .. } => true,
            IpcError::PermissionDenied { .. } => false,
            IpcError::ResourceExhausted { .. } => true,
            IpcError::ConnectionFailed { .. } => true,
            IpcError::SerializationError { .. } => false,
            IpcError::ProtocolError { .. } => false,
            IpcError::SystemError { .. } => false,
            IpcError::DataCorruption { .. } => false,
            IpcError::Deadlock { .. } => true,
            IpcError::ConfigurationError { .. } => false,
            IpcError::General { .. } => false,
        }
    }
    
    /// Check if error suggests retry
    pub fn should_retry(&self) -> bool {
        match self {
            IpcError::CommunicationError { .. } => true,
            IpcError::ResourceError { .. } => true,
            IpcError::Timeout { .. } => true,
            IpcError::ResourceExhausted { .. } => true,
            IpcError::ConnectionFailed { .. } => true,
            IpcError::Deadlock { .. } => true,
            _ => false,
        }
    }
    
    /// Get retry delay suggestion
    pub fn suggested_retry_delay(&self) -> Option<Duration> {
        match self {
            IpcError::Timeout { duration, .. } => Some(*duration / 2),
            IpcError::ResourceExhausted { .. } => Some(Duration::from_millis(100)),
            IpcError::ConnectionFailed { retry_count, .. } => {
                let base_delay = Duration::from_millis(50);
                let count = retry_count.unwrap_or(0);
                Some(base_delay * (1 << count.min(6))) // Exponential backoff, max 3.2s
            }
            IpcError::Deadlock { .. } => Some(Duration::from_millis(10)),
            _ => None,
        }
    }
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcError::CommunicationError { operation, error_type, message, resource_id } => {
                write!(f, "Communication error in {} ({}): {}", operation, error_type, message)?;
                if let Some(id) = resource_id {
                    write!(f, " [resource: {}]", id)?;
                }
                Ok(())
            }
            IpcError::SecurityError { operation, security_context, message, permission_required } => {
                write!(f, "Security error in {} ({}): {}", operation, security_context, message)?;
                if let Some(perm) = permission_required {
                    write!(f, " [permission required: {}]", perm)?;
                }
                Ok(())
            }
            IpcError::ResourceError { resource_type, operation, message, resource_id, limit_exceeded } => {
                write!(f, "Resource error with {} in {}: {}", resource_type, operation, message)?;
                if let Some(id) = resource_id {
                    write!(f, " [resource: {}]", id)?;
                }
                if let Some(limit) = limit_exceeded {
                    write!(f, " [limit: {}]", limit)?;
                }
                Ok(())
            }
            IpcError::Timeout { operation, duration, message, resource_id } => {
                write!(f, "Timeout in {} after {:?}: {}", operation, duration, message)?;
                if let Some(id) = resource_id {
                    write!(f, " [resource: {}]", id)?;
                }
                Ok(())
            }
            IpcError::InvalidOperation { operation, current_state, expected_state, resource_id } => {
                write!(f, "Invalid operation '{}': expected state '{}', got '{}'", 
                       operation, expected_state, current_state)?;
                if let Some(id) = resource_id {
                    write!(f, " [resource: {}]", id)?;
                }
                Ok(())
            }
            IpcError::PermissionDenied { operation, resource_id, required_permission, message } => {
                write!(f, "Permission denied for {} on {}: {} (requires {})", 
                       operation, resource_id, message, required_permission)
            }
            IpcError::ResourceExhausted { resource_type, operation, current_usage, max_limit, message } => {
                write!(f, "Resource exhausted: {} in {} ({}/{}) - {}", 
                       resource_type, operation, current_usage, max_limit, message)
            }
            IpcError::ConnectionFailed { operation, target, error_type, message, retry_count } => {
                write!(f, "Connection failed in {} to {} ({}): {}", operation, target, error_type, message)?;
                if let Some(count) = retry_count {
                    write!(f, " [retry: {}]", count)?;
                }
                Ok(())
            }
            IpcError::SerializationError { operation, data_type, message, position } => {
                write!(f, "Serialization error in {} for {}: {}", operation, data_type, message)?;
                if let Some(pos) = position {
                    write!(f, " [position: {}]", pos)?;
                }
                Ok(())
            }
            IpcError::ProtocolError { protocol, operation, message, error_code } => {
                write!(f, "Protocol error in {} ({}): {}", protocol, operation, message)?;
                if let Some(code) = error_code {
                    write!(f, " [code: {}]", code)?;
                }
                Ok(())
            }
            IpcError::SystemError { operation, error_code, message, system_call } => {
                write!(f, "System error in {} (code {}): {}", operation, error_code, message)?;
                if let Some(call) = system_call {
                    write!(f, " [syscall: {}]", call)?;
                }
                Ok(())
            }
            IpcError::DataCorruption { resource_type, resource_id, operation, message, checksum_expected, checksum_actual } => {
                write!(f, "Data corruption in {} {} during {}: {}", resource_type, resource_id, operation, message)?;
                if let (Some(expected), Some(actual)) = (checksum_expected, checksum_actual) {
                    write!(f, " [checksum: expected {}, got {}]", expected, actual)?;
                }
                Ok(())
            }
            IpcError::Deadlock { operation, resources_involved, message, detection_method } => {
                write!(f, "Deadlock detected in {} ({}): {} [resources: {}]", 
                       operation, detection_method, message, resources_involved.join(", "))
            }
            IpcError::ConfigurationError { parameter, value, message, valid_range } => {
                write!(f, "Configuration error: {} = '{}' - {}", parameter, value, message)?;
                if let Some(range) = valid_range {
                    write!(f, " [valid range: {}]", range)?;
                }
                Ok(())
            }
            IpcError::General { message, error_code } => {
                write!(f, "IPC error: {}", message)?;
                if let Some(code) = error_code {
                    write!(f, " [code: {}]", code)?;
                }
                Ok(())
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
        IpcError::SystemError {
            operation: "io_operation".to_string(),
            error_code: error.raw_os_error().unwrap_or(-1),
            message: error.to_string(),
            system_call: None,
        }
    }
}

/// Error creation helper functions

/// Create a communication error
pub fn communication_error(message: &str) -> IpcError {
    IpcError::CommunicationError {
        operation: "unknown".to_string(),
        error_type: "general".to_string(),
        message: message.to_string(),
        resource_id: None,
    }
}

/// Create a detailed communication error
pub fn communication_error_detailed(operation: &str, error_type: &str, message: &str) -> IpcError {
    IpcError::CommunicationError {
        operation: operation.to_string(),
        error_type: error_type.to_string(),
        message: message.to_string(),
        resource_id: None,
    }
}

/// Create a communication error with resource ID
pub fn communication_error_with_resource(operation: &str, error_type: &str, message: &str, resource_id: &str) -> IpcError {
    IpcError::CommunicationError {
        operation: operation.to_string(),
        error_type: error_type.to_string(),
        message: message.to_string(),
        resource_id: Some(resource_id.to_string()),
    }
}

/// Create a security error
pub fn security_error(message: &str) -> IpcError {
    IpcError::SecurityError {
        operation: "unknown".to_string(),
        security_context: "general".to_string(),
        message: message.to_string(),
        permission_required: None,
    }
}

/// Create a detailed security error
pub fn security_error_detailed(operation: &str, context: &str, message: &str) -> IpcError {
    IpcError::SecurityError {
        operation: operation.to_string(),
        security_context: context.to_string(),
        message: message.to_string(),
        permission_required: None,
    }
}

/// Create a resource error
pub fn resource_error(message: &str) -> IpcError {
    IpcError::ResourceError {
        resource_type: "unknown".to_string(),
        operation: "unknown".to_string(),
        message: message.to_string(),
        resource_id: None,
        limit_exceeded: None,
    }
}

/// Create a detailed resource error
pub fn resource_error_detailed(resource_type: &str, operation: &str, message: &str) -> IpcError {
    IpcError::ResourceError {
        resource_type: resource_type.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
        resource_id: None,
        limit_exceeded: None,
    }
}

/// Create a timeout error
pub fn timeout_error(operation: &str, duration: Duration, message: &str) -> IpcError {
    IpcError::Timeout {
        operation: operation.to_string(),
        duration,
        message: message.to_string(),
        resource_id: None,
    }
}

/// Create an invalid operation error
pub fn invalid_operation(operation: &str, current_state: &str, expected_state: &str) -> IpcError {
    IpcError::InvalidOperation {
        operation: operation.to_string(),
        current_state: current_state.to_string(),
        expected_state: expected_state.to_string(),
        resource_id: None,
    }
}

/// Create a permission denied error
pub fn permission_denied(operation: &str, resource_id: &str) -> IpcError {
    IpcError::PermissionDenied {
        operation: operation.to_string(),
        resource_id: resource_id.to_string(),
        required_permission: "read_write".to_string(),
        message: "Access denied".to_string(),
    }
}

/// Create a detailed permission denied error
pub fn permission_denied_detailed(operation: &str, resource_id: &str, required_permission: &str, message: &str) -> IpcError {
    IpcError::PermissionDenied {
        operation: operation.to_string(),
        resource_id: resource_id.to_string(),
        required_permission: required_permission.to_string(),
        message: message.to_string(),
    }
}

/// Create a resource exhausted error
pub fn resource_exhausted(resource_type: &str, operation: &str, current: u64, limit: u64) -> IpcError {
    IpcError::ResourceExhausted {
        resource_type: resource_type.to_string(),
        operation: operation.to_string(),
        current_usage: current,
        max_limit: limit,
        message: format!("Resource limit exceeded: {}/{}", current, limit),
    }
}

/// Create a connection failed error
pub fn connection_failed(operation: &str, target: &str, message: &str) -> IpcError {
    IpcError::ConnectionFailed {
        operation: operation.to_string(),
        target: target.to_string(),
        error_type: "connection".to_string(),
        message: message.to_string(),
        retry_count: None,
    }
}

/// Create a system error
pub fn system_error(code: i32, message: &str) -> IpcError {
    IpcError::SystemError {
        operation: "system".to_string(),
        error_code: code,
        message: message.to_string(),
        system_call: None,
    }
}

/// Create a detailed system error
pub fn system_error_detailed(operation: &str, code: i32, message: &str, system_call: &str) -> IpcError {
    IpcError::SystemError {
        operation: operation.to_string(),
        error_code: code,
        message: message.to_string(),
        system_call: Some(system_call.to_string()),
    }
}

/// Create a shared memory error
pub fn shared_memory_error(operation: &str, resource_id: &str, message: &str) -> IpcError {
    IpcError::ResourceError {
        resource_type: "shared_memory".to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
        resource_id: Some(resource_id.to_string()),
        limit_exceeded: None,
    }
}

/// Create a pipe error
pub fn pipe_error(operation: &str, resource_id: &str, message: &str) -> IpcError {
    IpcError::CommunicationError {
        operation: operation.to_string(),
        error_type: "pipe".to_string(),
        message: message.to_string(),
        resource_id: Some(resource_id.to_string()),
    }
}

/// Create a message queue error
pub fn message_queue_error(operation: &str, resource_id: &str, message: &str) -> IpcError {
    IpcError::CommunicationError {
        operation: operation.to_string(),
        error_type: "message_queue".to_string(),
        message: message.to_string(),
        resource_id: Some(resource_id.to_string()),
    }
}

/// Create a serialization error
pub fn serialization_error(operation: &str, data_type: &str, message: &str) -> IpcError {
    IpcError::SerializationError {
        operation: operation.to_string(),
        data_type: data_type.to_string(),
        message: message.to_string(),
        position: None,
    }
}

/// Create a deadlock error
pub fn deadlock_error(operation: &str, resources: Vec<String>, message: &str) -> IpcError {
    IpcError::Deadlock {
        operation: operation.to_string(),
        resources_involved: resources,
        message: message.to_string(),
        detection_method: "timeout".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = communication_error("Test communication error");
        assert_eq!(err.category(), "CommunicationError");
        assert!(err.is_recoverable());
        assert!(err.should_retry());

        let err = security_error("Access denied");
        assert_eq!(err.category(), "SecurityError");
        assert!(!err.is_recoverable());
        assert!(!err.should_retry());
    }

    #[test]
    fn test_error_display() {
        let err = communication_error_detailed("read", "pipe", "Connection lost");
        let display = format!("{}", err);
        assert!(display.contains("Communication error"));
        assert!(display.contains("read"));
        assert!(display.contains("pipe"));
        assert!(display.contains("Connection lost"));
    }

    #[test]
    fn test_timeout_error() {
        let duration = Duration::from_secs(30);
        let err = timeout_error("wait", duration, "Operation timed out");
        
        assert_eq!(err.category(), "Timeout");
        assert!(err.is_recoverable());
        assert!(err.should_retry());
        
        let suggested_delay = err.suggested_retry_delay();
        assert!(suggested_delay.is_some());
        assert_eq!(suggested_delay.unwrap(), Duration::from_secs(15));
    }

    #[test]
    fn test_resource_exhausted_error() {
        let err = resource_exhausted("memory", "allocate", 1024, 512);
        assert_eq!(err.category(), "ResourceExhausted");
        assert!(err.is_recoverable());
        assert!(err.should_retry());
        
        let display = format!("{}", err);
        assert!(display.contains("1024/512"));
    }

    #[test]
    fn test_permission_denied_error() {
        let err = permission_denied_detailed("write", "/tmp/shared", "write", "Insufficient privileges");
        assert_eq!(err.category(), "PermissionDenied");
        assert!(!err.is_recoverable());
        assert!(!err.should_retry());
        
        let display = format!("{}", err);
        assert!(display.contains("write"));
        assert!(display.contains("/tmp/shared"));
        assert!(display.contains("Insufficient privileges"));
    }

    #[test]
    fn test_deadlock_error() {
        let resources = vec!["resource1".to_string(), "resource2".to_string()];
        let err = deadlock_error("acquire", resources, "Circular dependency detected");
        
        assert_eq!(err.category(), "Deadlock");
        assert!(err.is_recoverable());
        assert!(err.should_retry());
        
        let display = format!("{}", err);
        assert!(display.contains("resource1, resource2"));
    }

    #[test]
    fn test_error_conversion_from_io() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let ipc_err: IpcError = io_err.into();
        
        assert_eq!(ipc_err.category(), "SystemError");
        assert!(ipc_err.message().contains("Access denied"));
    }

    #[test]
    fn test_retry_delay_suggestions() {
        let err1 = timeout_error("wait", Duration::from_secs(10), "Timeout");
        assert_eq!(err1.suggested_retry_delay(), Some(Duration::from_secs(5)));
        
        let err2 = resource_exhausted("memory", "alloc", 100, 50);
        assert_eq!(err2.suggested_retry_delay(), Some(Duration::from_millis(100)));
        
        let err3 = security_error("Access denied");
        assert_eq!(err3.suggested_retry_delay(), None);
    }
}
