use std::fmt;
use crate::error::Error;
use std::io;

/// File system specific error types
#[derive(Debug, Clone)]
pub enum FsError {
    /// File or directory not found
    NotFound(String),
    /// Permission denied
    PermissionDenied(String),
    /// File or directory already exists
    AlreadyExists(String),
    /// Invalid path
    InvalidPath(String),
    /// Directory not empty
    DirectoryNotEmpty(String),
    /// I/O error occurred
    IoError(String),
    /// Invalid operation
    InvalidOperation(String),
    /// Path is not a file
    NotAFile(String),
    /// Path is not a directory
    NotADirectory(String),
    /// Unsupported operation
    Unsupported(String),
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FsError::NotFound(path) => write!(f, "File or directory not found: {}", path),
            FsError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            FsError::AlreadyExists(path) => write!(f, "File or directory already exists: {}", path),
            FsError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            FsError::DirectoryNotEmpty(path) => write!(f, "Directory not empty: {}", path),
            FsError::IoError(msg) => write!(f, "I/O error: {}", msg),
            FsError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            FsError::NotAFile(path) => write!(f, "Path is not a file: {}", path),
            FsError::NotADirectory(path) => write!(f, "Path is not a directory: {}", path),
            FsError::Unsupported(msg) => write!(f, "Unsupported operation: {}", msg),
        }
    }
}

impl std::error::Error for FsError {}

impl From<io::Error> for FsError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => FsError::NotFound(error.to_string()),
            io::ErrorKind::PermissionDenied => FsError::PermissionDenied(error.to_string()),
            io::ErrorKind::AlreadyExists => FsError::AlreadyExists(error.to_string()),
            io::ErrorKind::InvalidInput => FsError::InvalidPath(error.to_string()),
            _ => FsError::IoError(error.to_string()),
        }
    }
}

/// Result type for file system operations
pub type FsResult<T> = std::result::Result<T, FsError>;
