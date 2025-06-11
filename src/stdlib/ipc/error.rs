/// Error handling for IPC operations
use std::fmt;
use std::error::Error as StdError;
use std::time::Duration;
use crate::error::CursedError;

/// Result type for IPC operations
pub type IpcResult<T> = Result<T, IpcError>;

/// Comprehensive error types for Inter-Process Communication operations
#[derive(Debug, Clone, PartialEq)]
pub enum IpcError {
    /// Communication channel error (pipes, sockets, etc.)
    CommunicationError { 
        channel_type: String, 
        operation: String, 
        message: String,
        error_code: Option<i32>
    },
    
    /// Security or permission related error
    SecurityError { 
        operation: String, 
        required_permission: String, 
        current_user: Option<String>,
        resource: String
    },
    
    /// Resource allocation or management error
    ResourceError { 
        resource_type: String, 
        operation: String, 
        message: String,
        available: Option<usize>,
        requested: Option<usize>
    },
    
    /// Operation timed out
    TimeoutError { 
        operation: String, 
        duration: Duration,
        resource: String
    },
    
    /// Invalid operation for current state
    InvalidOperation { 
        operation: String, 
        current_state: String, 
        expected_state: String,
        resource: String
    },
    
    /// Permission denied for operation
    PermissionDenied { 
        operation: String, 
        resource: String,
        required_permission: String,
        current_user: Option<String>
    },
    
    /// Resource limit exceeded
    ResourceExhausted { 
        resource_type: String, 
        limit: usize, 
        current_usage: usize,
        operation: String
    },
    
    /// Connection establishment failed
    ConnectionFailed { 
        target: String, 
        reason: String,
        retry_count: Option<u32>,
        last_error_code: Option<i32>
    },
    
    /// Shared memory specific errors
    SharedMemoryError { 
        operation: String, 
        memory_id: String, 
        message: String,
        size: Option<usize>
    },
    
    /// Message queue specific errors
    MessageQueueError { 
        operation: String, 
        queue_id: String, 
        message: String,
        queue_size: Option<usize>,
        message_count: Option<usize>
    },
    
    /// Semaphore specific errors
    SemaphoreError { 
        operation: String, 
        semaphore_id: String, 
        message: String,
        current_value: Option<i32>,
        max_value: Option<i32>
    },
    
    /// Signal handling errors
    SignalError { 
        signal: String, 
        operation: String, 
        message: String,
        process_id: Option<u32>
    },
    
    /// Serialization/Deserialization errors
    SerializationError { 
        operation: String, 
        data_type: String, 
        message: String,
        data_size: Option<usize>
    },
    
    /// Protocol or format errors
    ProtocolError { 
        protocol: String, 
        operation: String, 
        message: String,
        version_expected: Option<String>,
        version_received: Option<String>
    },
    
    /// Deadlock detected in IPC operations
    DeadlockError { 
        resources: Vec<String>, 
        processes: Vec<u32>,
        detection_method: String
    },
    
    /// Data corruption detected
    DataCorruption { 
        resource: String, 
        operation: String, 
        checksum_expected: Option<String>,
        checksum_actual: Option<String>
    },
    
    /// System-level error
    SystemError { 
        code: i32, 
        message: String,
        system_call: Option<String>
    },
    
    /// General IPC error
    General { 
        message: String,
        context: Option<String>
    },
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcError::CommunicationError { channel_type, operation, message, error_code } => {
                if let Some(code) = error_code {
                    write!(f, "{} {} failed (code {}): {}", channel_type, operation, code, message)
                } else {
                    write!(f, "{} {} failed: {}", channel_type, operation, message)
                }
            }
            IpcError::SecurityError { operation, required_permission, current_user, resource } => {
                if let Some(user) = current_user {
                    write!(f, "Security error in {}: user '{}' lacks '{}' permission for {}", 
                           operation, user, required_permission, resource)
                } else {
                    write!(f, "Security error in {}: '{}' permission required for {}", 
                           operation, required_permission, resource)
                }
            }
            IpcError::ResourceError { resource_type, operation, message, available, requested } => {
                if let (Some(avail), Some(req)) = (available, requested) {
                    write!(f, "{} {} failed: {} (available: {}, requested: {})", 
                           resource_type, operation, message, avail, req)
                } else {
                    write!(f, "{} {} failed: {}", resource_type, operation, message)
                }
            }
            IpcError::TimeoutError { operation, duration, resource } => {
                write!(f, "Operation '{}' on {} timed out after {:?}", operation, resource, duration)
            }
            IpcError::InvalidOperation { operation, current_state, expected_state, resource } => {
                write!(f, "Invalid operation '{}' on {}: state is '{}', expected '{}'", 
                       operation, resource, current_state, expected_state)
            }
            IpcError::PermissionDenied { operation, resource, required_permission, current_user } => {
                if let Some(user) = current_user {
                    write!(f, "Permission denied for {} on {}: user '{}' needs '{}'", 
                           operation, resource, user, required_permission)
                } else {
                    write!(f, "Permission denied for {} on {}: '{}' required", 
                           operation, resource, required_permission)
                }
            }
            IpcError::ResourceExhausted { resource_type, limit, current_usage, operation } => {
                write!(f, "{} limit exceeded in {}: {}/{} used", 
                       resource_type, operation, current_usage, limit)
            }
            IpcError::ConnectionFailed { target, reason, retry_count, last_error_code } => {
                if let (Some(retries), Some(code)) = (retry_count, last_error_code) {
                    write!(f, "Connection to {} failed after {} retries (code {}): {}", 
                           target, retries, code, reason)
                } else {
                    write!(f, "Connection to {} failed: {}", target, reason)
                }
            }
            IpcError::SharedMemoryError { operation, memory_id, message, size } => {
                if let Some(s) = size {
                    write!(f, "Shared memory {} '{}' failed (size {}): {}", 
                           operation, memory_id, s, message)
                } else {
                    write!(f, "Shared memory {} '{}' failed: {}", operation, memory_id, message)
                }
            }
            IpcError::MessageQueueError { operation, queue_id, message, queue_size, message_count } => {
                if let (Some(size), Some(count)) = (queue_size, message_count) {
                    write!(f, "Message queue {} '{}' failed ({}/{} messages): {}", 
                           operation, queue_id, count, size, message)
                } else {
                    write!(f, "Message queue {} '{}' failed: {}", operation, queue_id, message)
                }
            }
            IpcError::SemaphoreError { operation, semaphore_id, message, current_value, max_value } => {
                if let (Some(current), Some(max)) = (current_value, max_value) {
                    write!(f, "Semaphore {} '{}' failed (value {}/{}): {}", 
                           operation, semaphore_id, current, max, message)
                } else {
                    write!(f, "Semaphore {} '{}' failed: {}", operation, semaphore_id, message)
                }
            }
            IpcError::SignalError { signal, operation, message, process_id } => {
                if let Some(pid) = process_id {
                    write!(f, "Signal {} {} failed for process {}: {}", 
                           signal, operation, pid, message)
                } else {
                    write!(f, "Signal {} {} failed: {}", signal, operation, message)
                }
            }
            IpcError::SerializationError { operation, data_type, message, data_size } => {
                if let Some(size) = data_size {
                    write!(f, "{} {} failed ({} bytes): {}", 
                           data_type, operation, size, message)
                } else {
                    write!(f, "{} {} failed: {}", data_type, operation, message)
                }
            }
            IpcError::ProtocolError { protocol, operation, message, version_expected, version_received } => {
                if let (Some(expected), Some(received)) = (version_expected, version_received) {
                    write!(f, "{} {} failed: {} (expected version {}, got {})", 
                           protocol, operation, message, expected, received)
                } else {
                    write!(f, "{} {} failed: {}", protocol, operation, message)
                }
            }
            IpcError::DeadlockError { resources, processes, detection_method } => {
                write!(f, "Deadlock detected by {}: resources {:?}, processes {:?}", 
                       detection_method, resources, processes)
            }
            IpcError::DataCorruption { resource, operation, checksum_expected, checksum_actual } => {
                if let (Some(expected), Some(actual)) = (checksum_expected, checksum_actual) {
                    write!(f, "Data corruption in {} during {}: checksum mismatch (expected {}, got {})", 
                           resource, operation, expected, actual)
                } else {
                    write!(f, "Data corruption detected in {} during {}", resource, operation)
                }
            }
            IpcError::SystemError { code, message, system_call } => {
                if let Some(call) = system_call {
                    write!(f, "System error in {} (code {}): {}", call, code, message)
                } else {
                    write!(f, "System error (code {}): {}", code, message)
                }
            }
            IpcError::General { message, context } => {
                if let Some(ctx) = context {
                    write!(f, "IPC error in {}: {}", ctx, message)
                } else {
                    write!(f, "IPC error: {}", message)
                }
            }
        }
    }
}

impl StdError for IpcError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// Helper functions for creating specific error types
pub fn communication_error(message: &str) -> IpcError {
    IpcError::CommunicationError {
        channel_type: "unknown".to_string(),
        operation: "unknown".to_string(),
        message: message.to_string(),
        error_code: None,
    }
}

pub fn communication_error_detailed(channel_type: &str, operation: &str, message: &str) -> IpcError {
    IpcError::CommunicationError {
        channel_type: channel_type.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
        error_code: None,
    }
}

pub fn communication_error_with_code(channel_type: &str, operation: &str, message: &str, code: i32) -> IpcError {
    IpcError::CommunicationError {
        channel_type: channel_type.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
        error_code: Some(code),
    }
}

pub fn security_error(message: &str) -> IpcError {
    IpcError::SecurityError {
        operation: "unknown".to_string(),
        required_permission: "unknown".to_string(),
        current_user: None,
        resource: "unknown".to_string(),
    }
}

pub fn security_error_detailed(operation: &str, required_permission: &str, resource: &str) -> IpcError {
    IpcError::SecurityError {
        operation: operation.to_string(),
        required_permission: required_permission.to_string(),
        current_user: None,
        resource: resource.to_string(),
    }
}

pub fn resource_error(message: &str) -> IpcError {
    IpcError::ResourceError {
        resource_type: "unknown".to_string(),
        operation: "unknown".to_string(),
        message: message.to_string(),
        available: None,
        requested: None,
    }
}

pub fn resource_error_detailed(resource_type: &str, operation: &str, message: &str) -> IpcError {
    IpcError::ResourceError {
        resource_type: resource_type.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
        available: None,
        requested: None,
    }
}

pub fn timeout_error(operation: &str, duration: Duration, resource: &str) -> IpcError {
    IpcError::TimeoutError {
        operation: operation.to_string(),
        duration,
        resource: resource.to_string(),
    }
}

pub fn invalid_operation(operation: &str, current_state: &str, expected_state: &str, resource: &str) -> IpcError {
    IpcError::InvalidOperation {
        operation: operation.to_string(),
        current_state: current_state.to_string(),
        expected_state: expected_state.to_string(),
        resource: resource.to_string(),
    }
}

pub fn permission_denied(operation: &str, resource: &str) -> IpcError {
    IpcError::PermissionDenied {
        operation: operation.to_string(),
        resource: resource.to_string(),
        required_permission: "unknown".to_string(),
        current_user: None,
    }
}

pub fn permission_denied_detailed(operation: &str, resource: &str, required_permission: &str) -> IpcError {
    IpcError::PermissionDenied {
        operation: operation.to_string(),
        resource: resource.to_string(),
        required_permission: required_permission.to_string(),
        current_user: None,
    }
}

pub fn resource_exhausted(resource_type: &str, limit: usize, current_usage: usize, operation: &str) -> IpcError {
    IpcError::ResourceExhausted {
        resource_type: resource_type.to_string(),
        limit,
        current_usage,
        operation: operation.to_string(),
    }
}

pub fn connection_failed(target: &str, reason: &str) -> IpcError {
    IpcError::ConnectionFailed {
        target: target.to_string(),
        reason: reason.to_string(),
        retry_count: None,
        last_error_code: None,
    }
}

pub fn shared_memory_error(operation: &str, memory_id: &str, message: &str) -> IpcError {
    IpcError::SharedMemoryError {
        operation: operation.to_string(),
        memory_id: memory_id.to_string(),
        message: message.to_string(),
        size: None,
    }
}

pub fn message_queue_error(operation: &str, queue_id: &str, message: &str) -> IpcError {
    IpcError::MessageQueueError {
        operation: operation.to_string(),
        queue_id: queue_id.to_string(),
        message: message.to_string(),
        queue_size: None,
        message_count: None,
    }
}

pub fn semaphore_error(operation: &str, semaphore_id: &str, message: &str) -> IpcError {
    IpcError::SemaphoreError {
        operation: operation.to_string(),
        semaphore_id: semaphore_id.to_string(),
        message: message.to_string(),
        current_value: None,
        max_value: None,
    }
}

pub fn signal_error(signal: &str, operation: &str, message: &str) -> IpcError {
    IpcError::SignalError {
        signal: signal.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
        process_id: None,
    }
}

pub fn serialization_error(operation: &str, data_type: &str, message: &str) -> IpcError {
    IpcError::SerializationError {
        operation: operation.to_string(),
        data_type: data_type.to_string(),
        message: message.to_string(),
        data_size: None,
    }
}

pub fn protocol_error(protocol: &str, operation: &str, message: &str) -> IpcError {
    IpcError::ProtocolError {
        protocol: protocol.to_string(),
        operation: operation.to_string(),
        message: message.to_string(),
        version_expected: None,
        version_received: None,
    }
}

pub fn deadlock_error(resources: Vec<String>, processes: Vec<u32>, detection_method: &str) -> IpcError {
    IpcError::DeadlockError {
        resources,
        processes,
        detection_method: detection_method.to_string(),
    }
}

pub fn data_corruption_error(resource: &str, operation: &str) -> IpcError {
    IpcError::DataCorruption {
        resource: resource.to_string(),
        operation: operation.to_string(),
        checksum_expected: None,
        checksum_actual: None,
    }
}

pub fn system_error(code: i32, message: &str) -> IpcError {
    IpcError::SystemError {
        code,
        message: message.to_string(),
        system_call: None,
    }
}

pub fn general_error(message: &str) -> IpcError {
    IpcError::General {
        message: message.to_string(),
        context: None,
    }
}

// Conversion from standard library errors
impl From<std::io::Error> for IpcError {
    fn from(err: std::io::Error) -> Self {
        system_error(
            err.raw_os_error().unwrap_or(-1), 
            &err.to_string()
        )
    }
}

impl From<std::string::FromUtf8Error> for IpcError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        serialization_error("deserialization", "UTF-8", &err.to_string())
    }
}

impl From<std::str::Utf8Error> for IpcError {
    fn from(err: std::str::Utf8Error) -> Self {
        serialization_error("deserialization", "UTF-8", &err.to_string())
    }
}

// Integration with CURSED error system
impl From<IpcError> for CursedError {
    fn from(err: IpcError) -> Self {
        CursedError::Runtime(err.to_string())
    }
}

impl From<CursedError> for IpcError {
    fn from(err: CursedError) -> Self {
        general_error(&err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = communication_error("test error");
        assert!(matches!(err, IpcError::CommunicationError { .. }));
        assert_eq!(err.to_string(), "unknown unknown failed: test error");
    }

    #[test]
    fn test_communication_error_detailed() {
        let err = communication_error_detailed("pipe", "write", "broken pipe");
        assert!(matches!(err, IpcError::CommunicationError { .. }));
        assert!(err.to_string().contains("pipe write failed"));
    }

    #[test]
    fn test_security_error() {
        let err = security_error_detailed("read", "read_permission", "/shared/data");
        assert!(matches!(err, IpcError::SecurityError { .. }));
        assert!(err.to_string().contains("Security error"));
    }

    #[test]
    fn test_timeout_error() {
        let timeout = Duration::from_millis(1000);
        let err = timeout_error("acquire", timeout, "semaphore1");
        assert!(matches!(err, IpcError::TimeoutError { .. }));
        assert!(err.to_string().contains("timed out"));
    }

    #[test]
    fn test_resource_exhausted_error() {
        let err = resource_exhausted("memory", 1024, 2048, "allocate");
        assert!(matches!(err, IpcError::ResourceExhausted { .. }));
        assert!(err.to_string().contains("limit exceeded"));
    }

    #[test]
    fn test_error_conversions() {
        // Test conversion from std::io::Error
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let ipc_err: IpcError = io_err.into();
        assert!(matches!(ipc_err, IpcError::SystemError { .. }));

        // Test conversion to CursedError
        let ipc_err = communication_error("test");
        let cursed_err: CursedError = ipc_err.into();
        assert!(matches!(cursed_err, CursedError::Runtime(_)));
    }

    #[test]
    fn test_deadlock_error() {
        let resources = vec!["resource1".to_string(), "resource2".to_string()];
        let processes = vec![1234, 5678];
        let err = deadlock_error(resources, processes, "banker's algorithm");
        assert!(matches!(err, IpcError::DeadlockError { .. }));
        assert!(err.to_string().contains("Deadlock detected"));
    }

    #[test]
    fn test_data_corruption_error() {
        let err = data_corruption_error("shared_memory_1", "read");
        assert!(matches!(err, IpcError::DataCorruption { .. }));
        assert!(err.to_string().contains("Data corruption"));
    }
}
