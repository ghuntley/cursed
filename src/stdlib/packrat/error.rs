use crate::error::CursedError;
// CursedError handling for PackRat archive operations

use std::fmt;

#[derive(Debug, Clone)]
pub enum ArchiveError {
    // I/O related errors
    
    // Format-specific errors
    
    // Header errors
    
    // Compression errors
    
    // Security errors
    
    // File operation errors
    
    // General errors
// impl fmt::Display for ArchiveError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ArchiveError::IoError(msg) => write!(f, "I/O error: {}", msg),
//             ArchiveError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
//             ArchiveError::CorruptArchive(msg) => write!(f, "Corrupt archive: {}", msg),
//             ArchiveError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
//             ArchiveError::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
//             ArchiveError::HeaderTooLarge => write!(f, "Header too large"),
//             ArchiveError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
//             ArchiveError::DecompressionError(msg) => write!(f, "Decompression error: {}", msg),
//             ArchiveError::PathTraversal(msg) => write!(f, "Path traversal detected: {}", msg),
//             ArchiveError::NameTooLong(msg) => write!(f, "Name too long: {}", msg),
//             ArchiveError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
//             ArchiveError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
//             ArchiveError::General(msg) => write!(f, "Archive error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for ArchiveError {}
// 
// impl From<std::io::Error> for ArchiveError {
//     fn from(err: std::io::Error) -> Self {
//         ArchiveError::IoError(err.to_string())
//     }
// }

impl From<std::string::FromUtf8Error> for ArchiveError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ArchiveError::InvalidFormat(format!("UTF-8 error: {}", err))
    }
}

pub type ArchiveResult<T> = std::result::Result<T, ArchiveError>;

// Helper functions for creating specific errors
pub fn io_error(msg: &str) -> ArchiveError {
    ArchiveError::IoError(msg.to_string())
pub fn invalid_format(msg: &str) -> ArchiveError {
    ArchiveError::InvalidFormat(msg.to_string())
pub fn corrupt_archive(msg: &str) -> ArchiveError {
    ArchiveError::CorruptArchive(msg.to_string())
pub fn unsupported_format(msg: &str) -> ArchiveError {
    ArchiveError::UnsupportedFormat(msg.to_string())
pub fn invalid_header(msg: &str) -> ArchiveError {
    ArchiveError::InvalidHeader(msg.to_string())
pub fn compression_error(msg: &str) -> ArchiveError {
    ArchiveError::CompressionError(msg.to_string())
pub fn decompression_error(msg: &str) -> ArchiveError {
    ArchiveError::DecompressionError(msg.to_string())
pub fn path_traversal(msg: &str) -> ArchiveError {
    ArchiveError::PathTraversal(msg.to_string())
pub fn name_too_long(msg: &str) -> ArchiveError {
    ArchiveError::NameTooLong(msg.to_string())
pub fn file_not_found(msg: &str) -> ArchiveError {
    ArchiveError::FileNotFound(msg.to_string())
pub fn permission_denied(msg: &str) -> ArchiveError {
    ArchiveError::PermissionDenied(msg.to_string())
pub fn general_error(msg: &str) -> ArchiveError {
    ArchiveError::General(msg.to_string())
