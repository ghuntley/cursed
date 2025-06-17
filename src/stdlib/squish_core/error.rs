/// Error handling for SquishCore compression operations
/// 
/// This module provides comprehensive error types and handling for all compression
/// and decompression operations in the SquishCore module.

use std::fmt;
use std::io;

/// Main error type for SquishCore operations
#[derive(Debug, Clone)]
pub enum SquishError {
    /// IO operation failed
    IoError(String),
    /// Compression operation failed
    CompressionError(String),
    /// Decompression operation failed
    DecompressionError(String),
    /// Invalid input data
    InvalidInput(String),
    /// Unsupported compression format
    UnsupportedFormat(String),
    /// Invalid compression level
    InvalidCompressionLevel(i32),
    /// Corrupted data detected
    CorruptedData(String),
    /// Buffer too small for operation
    BufferTooSmall {
        required: usize,
        available: usize,
    },
    /// Compression ratio too low
    CompressionRatioTooLow {
        expected: f64,
        actual: f64,
    },
    /// Operation timeout
    Timeout(String),
    /// Dictionary not found or invalid
    InvalidDictionary(String),
    /// Memory allocation failed
    OutOfMemory(String),
    /// Generic error with message
    Generic(String),
}

impl fmt::Display for SquishError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SquishError::IoError(msg) => write!(f, "IO error: {}", msg),
            SquishError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            SquishError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
            SquishError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            SquishError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
            SquishError::InvalidCompressionLevel(level) => write!(f, "Invalid compression level: {}", level),
            SquishError::CorruptedData(msg) => write!(f, "Corrupted data: {}", msg),
            SquishError::BufferTooSmall { required, available } => {
                write!(f, "Buffer too small: required {} bytes, available {} bytes", required, available)
            },
            SquishError::CompressionRatioTooLow { expected, actual } => {
                write!(f, "Compression ratio too low: expected {:.2}, actual {:.2}", expected, actual)
            },
            SquishError::Timeout(msg) => write!(f, "Operation timeout: {}", msg),
            SquishError::InvalidDictionary(msg) => write!(f, "Invalid dictionary: {}", msg),
            SquishError::OutOfMemory(msg) => write!(f, "Out of memory: {}", msg),
            SquishError::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for SquishError {}

impl From<io::Error> for SquishError {
    fn from(err: io::Error) -> Self {
        SquishError::IoError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for SquishError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        SquishError::InvalidInput(format!("UTF-8 conversion error: {}", err))
    }
}

/// Result type for SquishCore operations
pub type SquishResult<T> = Result<T, SquishError>;

/// Specific error type for compression operations
#[derive(Debug, Clone)]
pub struct CompressionError {
    pub algorithm: String,
    pub level: Option<i32>,
    pub input_size: usize,
    pub message: String,
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compression error [{}]: {} (input size: {} bytes", 
               self.algorithm, self.message, self.input_size)?;
        if let Some(level) = self.level {
            write!(f, ", level: {}", level)?;
        }
        write!(f, ")")
    }
}

impl std::error::Error for CompressionError {}

/// Specific error type for decompression operations
#[derive(Debug, Clone)]
pub struct DecompressionError {
    pub algorithm: String,
    pub input_size: usize,
    pub bytes_processed: usize,
    pub message: String,
}

impl fmt::Display for DecompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Decompression error [{}]: {} (input size: {} bytes, processed: {} bytes)", 
               self.algorithm, self.message, self.input_size, self.bytes_processed)
    }
}

impl std::error::Error for DecompressionError {}

/// Helper functions for creating specific errors
impl SquishError {
    /// Create an IO error
    pub fn io_error(msg: impl Into<String>) -> Self {
        SquishError::IoError(msg.into())
    }

    /// Create a compression error
    pub fn compression_error(msg: impl Into<String>) -> Self {
        SquishError::CompressionError(msg.into())
    }

    /// Create a decompression error
    pub fn decompression_error(msg: impl Into<String>) -> Self {
        SquishError::DecompressionError(msg.into())
    }

    /// Create an invalid input error
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        SquishError::InvalidInput(msg.into())
    }

    /// Create an unsupported format error
    pub fn unsupported_format(msg: impl Into<String>) -> Self {
        SquishError::UnsupportedFormat(msg.into())
    }

    /// Create an invalid level error
    pub fn invalid_level(level: i32) -> Self {
        SquishError::InvalidCompressionLevel(level)
    }

    /// Create a corrupted data error
    pub fn corrupted_data(msg: impl Into<String>) -> Self {
        SquishError::CorruptedData(msg.into())
    }

    /// Create a buffer too small error
    pub fn buffer_too_small(required: usize, available: usize) -> Self {
        SquishError::BufferTooSmall { required, available }
    }

    /// Create a compression ratio too low error
    pub fn compression_ratio_too_low(expected: f64, actual: f64) -> Self {
        SquishError::CompressionRatioTooLow { expected, actual }
    }

    /// Create a timeout error
    pub fn timeout(msg: impl Into<String>) -> Self {
        SquishError::Timeout(msg.into())
    }

    /// Create an invalid dictionary error
    pub fn invalid_dictionary(msg: impl Into<String>) -> Self {
        SquishError::InvalidDictionary(msg.into())
    }

    /// Create an out of memory error
    pub fn out_of_memory(msg: impl Into<String>) -> Self {
        SquishError::OutOfMemory(msg.into())
    }

    /// Create a generic error
    pub fn generic(msg: impl Into<String>) -> Self {
        SquishError::Generic(msg.into())
    }
}

/// Create a not supported error (convenience function)
pub fn not_supported_error(msg: impl Into<String>) -> SquishError {
    SquishError::UnsupportedFormat(msg.into())
}

/// Create an IO error (convenience function)
pub fn io_error(msg: impl Into<String>) -> SquishError {
    SquishError::IoError(msg.into())
}

/// Create a general error (convenience function)
pub fn general_error(msg: impl Into<String>) -> SquishError {
    SquishError::Generic(msg.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let io_err = SquishError::io_error("test io error");
        assert!(matches!(io_err, SquishError::IoError(_)));

        let comp_err = SquishError::compression_error("test compression error");
        assert!(matches!(comp_err, SquishError::CompressionError(_)));

        let level_err = SquishError::invalid_level(42);
        assert!(matches!(level_err, SquishError::InvalidCompressionLevel(42)));

        let buffer_err = SquishError::buffer_too_small(100, 50);
        assert!(matches!(buffer_err, SquishError::BufferTooSmall { required: 100, available: 50 }));
    }

    #[test]
    fn test_error_display() {
        let error = SquishError::InvalidCompressionLevel(10);
        assert_eq!(error.to_string(), "Invalid compression level: 10");

        let buffer_error = SquishError::buffer_too_small(200, 100);
        assert_eq!(buffer_error.to_string(), "Buffer too small: required 200 bytes, available 100 bytes");
    }

    #[test]
    fn test_error_from_io() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let squish_error: SquishError = io_error.into();
        assert!(matches!(squish_error, SquishError::IoError(_)));
    }

    #[test]
    fn test_compression_error() {
        let comp_err = CompressionError {
            algorithm: "gzip".to_string(),
            level: Some(9),
            input_size: 1024,
            message: "Failed to compress".to_string(),
        };
        
        let display = comp_err.to_string();
        assert!(display.contains("gzip"));
        assert!(display.contains("level: 9"));
        assert!(display.contains("1024"));
    }

    #[test]
    fn test_decompression_error() {
        let decomp_err = DecompressionError {
            algorithm: "zlib".to_string(),
            input_size: 500,
            bytes_processed: 250,
            message: "Unexpected end of stream".to_string(),
        };
        
        let display = decomp_err.to_string();
        assert!(display.contains("zlib"));
        assert!(display.contains("500"));
        assert!(display.contains("250"));
    }
}
