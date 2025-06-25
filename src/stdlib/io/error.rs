
/// I/O specific error types for CURSED console operations
use std::fmt;
use crate::error::{CursedError, SourceLocation};

/// Errors that can occur during I/O operations
#[derive(Debug, Clone, PartialEq)]
pub enum IoError {
    /// End of file or stream reached unexpectedly
    /// Invalid UTF-8 sequence encountered
    /// Operation was interrupted
    /// Permission denied for the operation
    /// Resource temporarily unavailable
    /// Invalid input provided
    /// Buffer overflow or capacity exceeded
    /// Stream or handle is closed
    /// Timeout occurred during operation
    /// General I/O error with message
    /// System-level error
// impl fmt::Display for IoError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             IoError::UnexpectedEof => write!(f, "Unexpected end of input"),
//             IoError::InvalidUtf8 => write!(f, "Invalid UTF-8 sequence"),
//             IoError::Interrupted => write!(f, "Operation interrupted"),
//             IoError::PermissionDenied => write!(f, "Permission denied"),
//             IoError::WouldBlock => write!(f, "Resource temporarily unavailable"),
//             IoError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
//             IoError::BufferOverflow => write!(f, "Buffer overflow"),
//             IoError::StreamClosed => write!(f, "Stream is closed"),
//             IoError::Timeout => write!(f, "Operation timed out"),
//             IoError::General(msg) => write!(f, "I/O error: {}", msg),
//             IoError::System(code, msg) => write!(f, "System error {}: {}", code, msg),
//         }
//     }
// }

// impl std::error::CursedError for IoError {}
// 
// impl From<std::io::Error> for IoError {
//     fn from(error: std::io::Error) -> Self {
//         match error.kind() {
//             std::io::ErrorKind::UnexpectedEof => IoError::UnexpectedEof,
//             std::io::ErrorKind::InvalidInput => IoError::InvalidInput(error.to_string()),
//             std::io::ErrorKind::InvalidData => IoError::InvalidUtf8,
//             std::io::ErrorKind::PermissionDenied => IoError::PermissionDenied,
//             std::io::ErrorKind::WouldBlock => IoError::WouldBlock,
//             std::io::ErrorKind::Interrupted => IoError::Interrupted,
//             std::io::ErrorKind::TimedOut => IoError::Timeout,
//             _ => IoError::General(error.to_string()),
//         }
//     }
// }

impl From<std::string::FromUtf8Error> for IoError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        IoError::InvalidUtf8
    }
}

impl From<std::str::Utf8Error> for IoError {
    fn from(_: std::str::Utf8Error) -> Self {
        IoError::InvalidUtf8
    }
}

// impl From<IoError> for CursedError {
//     fn from(io_error: IoError) -> Self {
//         CursedError::Runtime(format!("I/O operation failed: {}", io_error))
//     }
// }

/// Result type for I/O operations
pub type IoResult<T> = std::result::Result<T, IoError>;

/// Helper function to create IoError from system error
pub fn system_error(code: i32, message: impl Into<String>) -> IoError {
    IoError::System(code, message.into())
/// Helper function to create general I/O error
pub fn io_error(message: impl Into<String>) -> IoError {
    IoError::General(message.into())
/// Helper function to create invalid input error
pub fn invalid_input(message: impl Into<String>) -> IoError {
    IoError::InvalidInput(message.into())
}
