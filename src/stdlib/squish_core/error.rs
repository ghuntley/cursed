//! Error types for SquishCore compression library

use crate::error_types::CursedError;
use std::fmt;

/// Compression/decompression error types
#[derive(Debug, Clone)]
pub enum SquishError {
    /// Invalid input data
    InvalidData(String),
    /// Compression failed
    CompressionError(String),
    /// Decompression failed
    DecompressionError(String),
    /// Unsupported compression format
    UnsupportedFormat(String),
    /// I/O error during compression/decompression
    IoError(String),
    /// Invalid compression level
    InvalidLevel(String),
    /// Generic error
    Generic(String),
}

impl fmt::Display for SquishError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SquishError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            SquishError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            SquishError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
            SquishError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
            SquishError::IoError(msg) => write!(f, "I/O error: {}", msg),
            SquishError::InvalidLevel(msg) => write!(f, "Invalid level: {}", msg),
            SquishError::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for SquishError {}

impl From<std::io::Error> for SquishError {
    fn from(err: std::io::Error) -> Self {
        SquishError::IoError(err.to_string())
    }
}

impl From<CursedError> for SquishError {
    fn from(err: CursedError) -> Self {
        SquishError::Generic(err.to_string())
    }
}

impl SquishError {
    pub fn generic(msg: &str) -> Self {
        SquishError::Generic(msg.to_string())
    }
}

/// Result type for compression operations
pub type SquishResult<T> = Result<T, SquishError>;

/// Alias for compatibility
pub type CompressionError = SquishError;
pub type DecompressionError = SquishError;
