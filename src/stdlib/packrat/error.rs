// Error handling for PackRat archive operations

use std::fmt;

#[derive(Debug, Clone)]
pub enum ArchiveError {
    // I/O related errors
    IoError(String),
    
    // Format-specific errors
    InvalidFormat(String),
    CorruptArchive(String),
    UnsupportedFormat(String),
    
    // Header errors
    InvalidHeader(String),
    HeaderTooLarge,
    
    // Compression errors
    CompressionError(String),
    DecompressionError(String),
    
    // Security errors
    PathTraversal(String),
    NameTooLong(String),
    
    // File operation errors
    FileNotFound(String),
    PermissionDenied(String),
    
    // General errors
    General(String),
}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchiveError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ArchiveError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ArchiveError::CorruptArchive(msg) => write!(f, "Corrupt archive: {}", msg),
            ArchiveError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
            ArchiveError::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            ArchiveError::HeaderTooLarge => write!(f, "Header too large"),
            ArchiveError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            ArchiveError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
            ArchiveError::PathTraversal(msg) => write!(f, "Path traversal detected: {}", msg),
            ArchiveError::NameTooLong(msg) => write!(f, "Name too long: {}", msg),
            ArchiveError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            ArchiveError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            ArchiveError::General(msg) => write!(f, "Archive error: {}", msg),
        }
    }
}

impl std::error::Error for ArchiveError {}

impl From<std::io::Error> for ArchiveError {
    fn from(err: std::io::Error) -> Self {
        ArchiveError::IoError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for ArchiveError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ArchiveError::InvalidFormat(format!("UTF-8 error: {}", err))
    }
}

pub type ArchiveResult<T> = Result<T, ArchiveError>;

// Helper functions for creating specific errors
pub fn io_error(msg: &str) -> ArchiveError {
    ArchiveError::IoError(msg.to_string())
}

pub fn invalid_format(msg: &str) -> ArchiveError {
    ArchiveError::InvalidFormat(msg.to_string())
}

pub fn corrupt_archive(msg: &str) -> ArchiveError {
    ArchiveError::CorruptArchive(msg.to_string())
}

pub fn unsupported_format(msg: &str) -> ArchiveError {
    ArchiveError::UnsupportedFormat(msg.to_string())
}

pub fn invalid_header(msg: &str) -> ArchiveError {
    ArchiveError::InvalidHeader(msg.to_string())
}

pub fn compression_error(msg: &str) -> ArchiveError {
    ArchiveError::CompressionError(msg.to_string())
}

pub fn decompression_error(msg: &str) -> ArchiveError {
    ArchiveError::DecompressionError(msg.to_string())
}

pub fn path_traversal(msg: &str) -> ArchiveError {
    ArchiveError::PathTraversal(msg.to_string())
}

pub fn name_too_long(msg: &str) -> ArchiveError {
    ArchiveError::NameTooLong(msg.to_string())
}

pub fn file_not_found(msg: &str) -> ArchiveError {
    ArchiveError::FileNotFound(msg.to_string())
}

pub fn permission_denied(msg: &str) -> ArchiveError {
    ArchiveError::PermissionDenied(msg.to_string())
}

pub fn general_error(msg: &str) -> ArchiveError {
    ArchiveError::General(msg.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;
    
    #[test]
    fn test_error_creation() {
        info!("Testing PackRat error creation");
        
        let err = io_error("test I/O error");
        assert!(matches!(err, ArchiveError::IoError(_)));
        
        let err = invalid_format("test format error");
        assert!(matches!(err, ArchiveError::InvalidFormat(_)));
        
        let err = corrupt_archive("test corruption");
        assert!(matches!(err, ArchiveError::CorruptArchive(_)));
        
        info!("PackRat error creation tests passed");
    }
    
    #[test]
    fn test_error_display() {
        info!("Testing PackRat error display");
        
        let err = io_error("test message");
        let display = format!("{}", err);
        assert!(display.contains("I/O error"));
        assert!(display.contains("test message"));
        
        info!("PackRat error display tests passed");
    }
    
    #[test]
    fn test_error_conversion() {
        info!("Testing PackRat error conversion");
        
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let archive_err: ArchiveError = io_err.into();
        assert!(matches!(archive_err, ArchiveError::IoError(_)));
        
        info!("PackRat error conversion tests passed");
    }
}
