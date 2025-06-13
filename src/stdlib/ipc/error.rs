/// IPC specific error types for CURSED
use std::fmt;
use crate::error::{CursedError, SourceLocation};

/// Errors that can occur during IPC operations
#[derive(Debug, Clone, PartialEq)]
pub enum IpcError {
    /// Permission denied for IPC operation
    PermissionDenied(String),
    /// Resource not found (pipe, socket, shared memory, etc.)
    NotFound(String),
    /// Resource already exists
    AlreadyExists(String),
    /// Invalid operation or state
    InvalidOperation(String),
    /// I/O error during IPC operation
    IoError(String),
    /// Timeout occurred during operation
    Timeout(String),
    /// Connection failed or lost
    ConnectionFailed(String),
    /// Buffer overflow or size mismatch
    BufferError(String),
    /// Security or authentication error
    SecurityError(String),
    /// Resource exhausted (memory, file descriptors, etc.)
    ResourceExhausted(String),
    /// Invalid input or parameters
    InvalidInput(String),
    /// Internal error
    Internal(String),
    /// System-level error
    System(i32, String),
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            IpcError::NotFound(msg) => write!(f, "Resource not found: {}", msg),
            IpcError::AlreadyExists(msg) => write!(f, "Resource already exists: {}", msg),
            IpcError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            IpcError::IoError(msg) => write!(f, "I/O error: {}", msg),
            IpcError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            IpcError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            IpcError::BufferError(msg) => write!(f, "Buffer error: {}", msg),
            IpcError::SecurityError(msg) => write!(f, "Security error: {}", msg),
            IpcError::ResourceExhausted(msg) => write!(f, "Resource exhausted: {}", msg),
            IpcError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            IpcError::Internal(msg) => write!(f, "Internal error: {}", msg),
            IpcError::System(code, msg) => write!(f, "System error {}: {}", code, msg),
        }
    }
}

impl std::error::Error for IpcError {}

impl From<IpcError> for CursedError {
    fn from(err: IpcError) -> Self {
        CursedError::RuntimeError {
            message: err.to_string(),
            location: SourceLocation::unknown(),
        }
    }
}

impl From<std::io::Error> for IpcError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match err.kind() {
            ErrorKind::PermissionDenied => IpcError::PermissionDenied(err.to_string()),
            ErrorKind::NotFound => IpcError::NotFound(err.to_string()),
            ErrorKind::AlreadyExists => IpcError::AlreadyExists(err.to_string()),
            ErrorKind::InvalidInput => IpcError::InvalidInput(err.to_string()),
            ErrorKind::TimedOut => IpcError::Timeout(err.to_string()),
            ErrorKind::ConnectionRefused | ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset => {
                IpcError::ConnectionFailed(err.to_string())
            }
            ErrorKind::WriteZero | ErrorKind::UnexpectedEof => {
                IpcError::BufferError(err.to_string())
            }
            _ => IpcError::IoError(err.to_string()),
        }
    }
}

impl From<std::ffi::NulError> for IpcError {
    fn from(err: std::ffi::NulError) -> Self {
        IpcError::InvalidInput(format!("Null byte in string: {}", err))
    }
}

impl From<std::str::Utf8Error> for IpcError {
    fn from(err: std::str::Utf8Error) -> Self {
        IpcError::InvalidInput(format!("Invalid UTF-8: {}", err))
    }
}

/// Result type for IPC operations
pub type IpcResult<T> = Result<T, IpcError>;

/// Helper functions for creating common errors
pub fn permission_denied(msg: &str) -> IpcError {
    IpcError::PermissionDenied(msg.to_string())
}

pub fn not_found(msg: &str) -> IpcError {
    IpcError::NotFound(msg.to_string())
}

pub fn already_exists(msg: &str) -> IpcError {
    IpcError::AlreadyExists(msg.to_string())
}

pub fn invalid_operation(msg: &str) -> IpcError {
    IpcError::InvalidOperation(msg.to_string())
}

pub fn io_error(msg: &str) -> IpcError {
    IpcError::IoError(msg.to_string())
}

pub fn timeout(msg: &str) -> IpcError {
    IpcError::Timeout(msg.to_string())
}

pub fn connection_failed(msg: &str) -> IpcError {
    IpcError::ConnectionFailed(msg.to_string())
}

pub fn buffer_error(msg: &str) -> IpcError {
    IpcError::BufferError(msg.to_string())
}

pub fn security_error(msg: &str) -> IpcError {
    IpcError::SecurityError(msg.to_string())
}

pub fn resource_exhausted(msg: &str) -> IpcError {
    IpcError::ResourceExhausted(msg.to_string())
}

pub fn invalid_input(msg: &str) -> IpcError {
    IpcError::InvalidInput(msg.to_string())
}

pub fn internal_error(msg: &str) -> IpcError {
    IpcError::Internal(msg.to_string())
}

pub fn system_error(code: i32, msg: &str) -> IpcError {
    IpcError::System(code, msg.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = permission_denied("test access");
        assert!(matches!(err, IpcError::PermissionDenied(_)));
        assert!(err.to_string().contains("Permission denied"));

        let err = not_found("missing resource");
        assert!(matches!(err, IpcError::NotFound(_)));
        assert!(err.to_string().contains("Resource not found"));

        let err = timeout("operation took too long");
        assert!(matches!(err, IpcError::Timeout(_)));
        assert!(err.to_string().contains("Timeout"));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test");
        let ipc_err: IpcError = io_err.into();
        assert!(matches!(ipc_err, IpcError::PermissionDenied(_)));

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let ipc_err: IpcError = io_err.into();
        assert!(matches!(ipc_err, IpcError::NotFound(_)));
    }

    #[test]
    fn test_cursed_error_conversion() {
        let ipc_err = invalid_operation("test operation");
        let cursed_err: CursedError = ipc_err.into();
        assert!(matches!(cursed_err, CursedError::RuntimeError { .. }));
    }
}
