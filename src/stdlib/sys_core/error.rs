/// CursedError types for system core operations
use std::fmt;
use crate::error::CursedError;

/// System core operation errors
#[derive(Debug, Clone)]
pub enum SysCoreError {
    /// System call failed
    SystemCall(String, i32),
    /// Permission denied
    PermissionDenied(String),
    /// Resource not found
    NotFound(String),
    /// Resource already exists
    AlreadyExists(String),
    /// Invalid argument
    InvalidArgument(String),
    /// Operation not supported
    NotSupported(String),
    /// Out of resources
    OutOfResources(String),
    /// I/O error
    IoError(String),
    /// Memory allocation failed
    MemoryError(String),
    /// Network error
    NetworkError(String),
    /// Platform-specific error
    PlatformError(String, Option<i32>),
    /// General error with message
    General(String),
}

// impl fmt::Display for SysCoreError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::SystemCall(msg, errno) => write!(f, "System call failed: {} (errno: {})", msg, errno),
//             Self::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
//             Self::NotFound(msg) => write!(f, "Not found: {}", msg),
//             Self::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
//             Self::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
//             Self::NotSupported(msg) => write!(f, "Operation not supported: {}", msg),
//             Self::OutOfResources(msg) => write!(f, "Out of resources: {}", msg),
//             Self::IoError(msg) => write!(f, "I/O error: {}", msg),
//             Self::MemoryError(msg) => write!(f, "Memory error: {}", msg),
//             Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
//             Self::PlatformError(msg, Some(code)) => write!(f, "Platform error: {} (code: {})", msg, code),
//             Self::PlatformError(msg, None) => write!(f, "Platform error: {}", msg),
//             Self::General(msg) => write!(f, "{}", msg),
//         }
//     }
// }

// impl CursedError for SysCoreError {}
// 
// impl From<std::io::Error> for SysCoreError {
//     fn from(err: std::io::Error) -> Self {
//         Self::IoError(err.to_string())
//     }
// }

/// Result type for system core operations
pub type SysCoreResult<T> = std::result::Result<T, SysCoreError>;

/// Helper functions for creating errors
pub fn system_call_error(msg: &str, errno: i32) -> SysCoreError {
    SysCoreError::SystemCall(msg.to_string(), errno)
}

pub fn permission_denied(msg: &str) -> SysCoreError {
    SysCoreError::PermissionDenied(msg.to_string())
}

pub fn not_found(msg: &str) -> SysCoreError {
    SysCoreError::NotFound(msg.to_string())
}

pub fn already_exists(msg: &str) -> SysCoreError {
    SysCoreError::AlreadyExists(msg.to_string())
}

pub fn invalid_argument(msg: &str) -> SysCoreError {
    SysCoreError::InvalidArgument(msg.to_string())
}

pub fn not_supported(msg: &str) -> SysCoreError {
    SysCoreError::NotSupported(msg.to_string())
}

pub fn out_of_resources(msg: &str) -> SysCoreError {
    SysCoreError::OutOfResources(msg.to_string())
}

pub fn memory_error(msg: &str) -> SysCoreError {
    SysCoreError::MemoryError(msg.to_string())
}

pub fn network_error(msg: &str) -> SysCoreError {
    SysCoreError::NetworkError(msg.to_string())
}

pub fn platform_error(msg: &str, code: Option<i32>) -> SysCoreError {
    SysCoreError::PlatformError(msg.to_string(), code)
}

pub fn general_error(msg: &str) -> SysCoreError {
    SysCoreError::General(msg.to_string())
}
