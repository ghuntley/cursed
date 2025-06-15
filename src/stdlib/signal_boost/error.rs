/// Error handling for SignalBoost module
use std::fmt;
use crate::error::CursedError;

/// Result type for SignalBoost operations
pub type SignalBoostResult<T> = Result<T, SignalBoostError>;

/// Comprehensive error type for signal handling operations
#[derive(Debug, Clone)]
pub enum SignalBoostError {
    /// Invalid signal number or type
    InvalidSignal(String),
    /// System error during signal operation
    SystemError(String),
    /// Permission denied for signal operation
    PermissionDenied(String),
    /// Signal operation not supported on this platform
    NotSupported(String),
    /// Timeout during signal operation
    Timeout(String),
    /// Handler already registered for signal
    HandlerExists(String),
    /// No handler registered for signal
    NoHandler(String),
    /// Signal processing error
    ProcessingError(String),
    /// Configuration error
    ConfigError(String),
    /// I/O error during signal communication
    IoError(String),
    /// General signal boost error
    General(String),
}

impl fmt::Display for SignalBoostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignalBoostError::InvalidSignal(msg) => write!(f, "Invalid signal: {}", msg),
            SignalBoostError::SystemError(msg) => write!(f, "System error: {}", msg),
            SignalBoostError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            SignalBoostError::NotSupported(msg) => write!(f, "Not supported: {}", msg),
            SignalBoostError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            SignalBoostError::HandlerExists(msg) => write!(f, "Handler exists: {}", msg),
            SignalBoostError::NoHandler(msg) => write!(f, "No handler: {}", msg),
            SignalBoostError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            SignalBoostError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            SignalBoostError::IoError(msg) => write!(f, "I/O error: {}", msg),
            SignalBoostError::General(msg) => write!(f, "SignalBoost error: {}", msg),
        }
    }
}

impl std::error::Error for SignalBoostError {}

impl From<SignalBoostError> for CursedError {
    fn from(err: SignalBoostError) -> Self {
        CursedError::Runtime(err.to_string())
    }
}

impl From<std::io::Error> for SignalBoostError {
    fn from(err: std::io::Error) -> Self {
        SignalBoostError::IoError(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for SignalBoostError {
    fn from(err: std::time::SystemTimeError) -> Self {
        SignalBoostError::SystemError(err.to_string())
    }
}

// Helper functions for creating specific errors
pub fn invalid_signal(msg: &str) -> SignalBoostError {
    SignalBoostError::InvalidSignal(msg.to_string())
}

pub fn system_error(msg: &str) -> SignalBoostError {
    SignalBoostError::SystemError(msg.to_string())
}

pub fn permission_denied(msg: &str) -> SignalBoostError {
    SignalBoostError::PermissionDenied(msg.to_string())
}

pub fn not_supported(msg: &str) -> SignalBoostError {
    SignalBoostError::NotSupported(msg.to_string())
}

pub fn timeout_error(msg: &str) -> SignalBoostError {
    SignalBoostError::Timeout(msg.to_string())
}

pub fn handler_exists(msg: &str) -> SignalBoostError {
    SignalBoostError::HandlerExists(msg.to_string())
}

pub fn no_handler(msg: &str) -> SignalBoostError {
    SignalBoostError::NoHandler(msg.to_string())
}

pub fn processing_error(msg: &str) -> SignalBoostError {
    SignalBoostError::ProcessingError(msg.to_string())
}

pub fn config_error(msg: &str) -> SignalBoostError {
    SignalBoostError::ConfigError(msg.to_string())
}

pub fn general_error(msg: &str) -> SignalBoostError {
    SignalBoostError::General(msg.to_string())
}
