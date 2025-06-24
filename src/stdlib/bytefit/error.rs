/// Error handling for ByteFit operations
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ByteFitError {
    InvalidUtf8(String),
    InvalidBase64(String),
    InvalidHex(String),
    InvalidPattern(String),
    IndexOutOfBounds(String),
    InvalidInput(String),
    BufferOverflow(String),
    EncodingError(String),
    RegexError(String),
}

impl fmt::Display for ByteFitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByteFitError::InvalidUtf8(msg) => write!(f, "Invalid UTF-8: {}", msg),
            ByteFitError::InvalidBase64(msg) => write!(f, "Invalid Base64: {}", msg),
            ByteFitError::InvalidHex(msg) => write!(f, "Invalid hex: {}", msg),
            ByteFitError::InvalidPattern(msg) => write!(f, "Invalid pattern: {}", msg),
            ByteFitError::IndexOutOfBounds(msg) => write!(f, "Index out of bounds: {}", msg),
            ByteFitError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ByteFitError::BufferOverflow(msg) => write!(f, "Buffer overflow: {}", msg),
            ByteFitError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            ByteFitError::RegexError(msg) => write!(f, "Regex error: {}", msg),
        }
    }
}

impl std::error::Error for ByteFitError {}

/// Result type for ByteFit operations
pub type ByteFitResult<T> = std::result::Result<T, ByteFitError>;

/// Helper functions for creating errors
pub fn invalid_utf8(msg: &str) -> ByteFitError {
    ByteFitError::InvalidUtf8(msg.to_string())
}

pub fn invalid_base64(msg: &str) -> ByteFitError {
    ByteFitError::InvalidBase64(msg.to_string())
}

pub fn invalid_hex(msg: &str) -> ByteFitError {
    ByteFitError::InvalidHex(msg.to_string())
}

pub fn invalid_pattern(msg: &str) -> ByteFitError {
    ByteFitError::InvalidPattern(msg.to_string())
}

pub fn index_out_of_bounds(msg: &str) -> ByteFitError {
    ByteFitError::IndexOutOfBounds(msg.to_string())
}

pub fn invalid_input(msg: &str) -> ByteFitError {
    ByteFitError::InvalidInput(msg.to_string())
}

pub fn buffer_overflow(msg: &str) -> ByteFitError {
    ByteFitError::BufferOverflow(msg.to_string())
}

pub fn encoding_error(msg: &str) -> ByteFitError {
    ByteFitError::EncodingError(msg.to_string())
}

pub fn regex_error(msg: &str) -> ByteFitError {
    ByteFitError::RegexError(msg.to_string())
}
