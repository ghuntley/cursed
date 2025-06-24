/// Error handling for ChaosMode runtime system
use std::fmt;

#[derive(Debug, Clone)]
pub enum ChaosError {
    /// Runtime system error
    RuntimeError(String),
    /// Goroutine management error
    GoroutineError(String),
    /// Memory management error
    MemoryError(String),
    /// Profiling error
    ProfilingError(String),
    /// Configuration error
    ConfigError(String),
    /// System call error
    SystemError(String),
    /// Invalid parameter error
    InvalidParameter(String),
    /// Operation not supported on this platform
    NotSupported(String),
    /// Permission denied
    PermissionDenied(String),
    /// Resource not available
    ResourceUnavailable(String),
    /// Timeout error
    Timeout(String),
}

impl fmt::Display for ChaosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChaosError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            ChaosError::GoroutineError(msg) => write!(f, "Goroutine error: {}", msg),
            ChaosError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            ChaosError::ProfilingError(msg) => write!(f, "Profiling error: {}", msg),
            ChaosError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            ChaosError::SystemError(msg) => write!(f, "System error: {}", msg),
            ChaosError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            ChaosError::NotSupported(msg) => write!(f, "Not supported: {}", msg),
            ChaosError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            ChaosError::ResourceUnavailable(msg) => write!(f, "Resource unavailable: {}", msg),
            ChaosError::Timeout(msg) => write!(f, "Timeout: {}", msg),
        }
    }
}

impl std::error::Error for ChaosError {}

pub type ChaosResult<T> = std::result::Result<T, ChaosError>;

// Helper functions for creating errors
pub fn runtime_error(msg: &str) -> ChaosError {
    ChaosError::RuntimeError(msg.to_string())
}

pub fn goroutine_error(msg: &str) -> ChaosError {
    ChaosError::GoroutineError(msg.to_string())
}

pub fn memory_error(msg: &str) -> ChaosError {
    ChaosError::MemoryError(msg.to_string())
}

pub fn profiling_error(msg: &str) -> ChaosError {
    ChaosError::ProfilingError(msg.to_string())
}

pub fn config_error(msg: &str) -> ChaosError {
    ChaosError::ConfigError(msg.to_string())
}

pub fn system_error(msg: &str) -> ChaosError {
    ChaosError::SystemError(msg.to_string())
}

pub fn invalid_parameter(msg: &str) -> ChaosError {
    ChaosError::InvalidParameter(msg.to_string())
}

pub fn not_supported(msg: &str) -> ChaosError {
    ChaosError::NotSupported(msg.to_string())
}

pub fn permission_denied(msg: &str) -> ChaosError {
    ChaosError::PermissionDenied(msg.to_string())
}

pub fn resource_unavailable(msg: &str) -> ChaosError {
    ChaosError::ResourceUnavailable(msg.to_string())
}

pub fn timeout_error(msg: &str) -> ChaosError {
    ChaosError::Timeout(msg.to_string())
}

// Conversion functions from other error types
impl From<std::io::Error> for ChaosError {
    fn from(err: std::io::Error) -> Self {
        ChaosError::SystemError(err.to_string())
    }
}

impl From<crate::error::crate::types::RuntimeError> for ChaosError {
    fn from(err: crate::error::crate::types::RuntimeError) -> Self {
        ChaosError::RuntimeError(err.to_string())
    }
}

impl From<serde_json::Error> for ChaosError {
    fn from(err: serde_json::Error) -> Self {
        ChaosError::RuntimeError(format!("JSON error: {}", err))
    }
}
