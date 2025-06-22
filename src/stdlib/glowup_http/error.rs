//! GlowUpHTTP error types and error handling utilities

use std::fmt;
use std::io;
use std::net::AddrParseError;
use std::sync::PoisonError;

/// Result type for GlowUpHTTP operations
pub type GlowUpResult<T> = Result<T, GlowUpError>;

/// Main error type for GlowUpHTTP operations
#[derive(Debug, Clone)]
pub enum GlowUpError {
    /// I/O related errors
    Io(String),
    /// HTTP parsing errors
    Parse(String),
    /// Connection errors
    Connection(String),
    /// Server binding errors
    Bind(String),
    /// TLS/SSL errors
    Tls(String),
    /// Timeout errors
    Timeout(String),
    /// Authentication errors
    Auth(String),
    /// Configuration errors
    Config(String),
    /// Middleware errors
    Middleware(String),
    /// Router errors
    Router(String),
    /// WebSocket errors
    WebSocket(String),
    /// JSON parsing/serialization errors
    Json(String),
    /// General HTTP errors with status code
    Http(u16, String),
    /// Other generic errors
    Other(String),
    /// Invalid request format
    InvalidRequest(String),
    /// Server already running
    ServerAlreadyRunning,
    /// Server not running
    ServerNotRunning,
    /// Internal server error
    Internal(String),
}

impl fmt::Display for GlowUpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlowUpError::Io(msg) => write!(f, "I/O error: {}", msg),
            GlowUpError::Parse(msg) => write!(f, "Parse error: {}", msg),
            GlowUpError::Connection(msg) => write!(f, "Connection error: {}", msg),
            GlowUpError::Bind(msg) => write!(f, "Bind error: {}", msg),
            GlowUpError::Tls(msg) => write!(f, "TLS error: {}", msg),
            GlowUpError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            GlowUpError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            GlowUpError::Config(msg) => write!(f, "Configuration error: {}", msg),
            GlowUpError::Middleware(msg) => write!(f, "Middleware error: {}", msg),
            GlowUpError::Router(msg) => write!(f, "Router error: {}", msg),
            GlowUpError::WebSocket(msg) => write!(f, "WebSocket error: {}", msg),
            GlowUpError::Json(msg) => write!(f, "JSON error: {}", msg),
            GlowUpError::Http(code, msg) => write!(f, "HTTP error {}: {}", code, msg),
            GlowUpError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            GlowUpError::ServerAlreadyRunning => write!(f, "Server is already running"),
            GlowUpError::ServerNotRunning => write!(f, "Server is not running"),
            GlowUpError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for GlowUpError {}

impl From<io::Error> for GlowUpError {
    fn from(err: io::Error) -> Self {
        GlowUpError::Io(err.to_string())
    }
}

impl From<AddrParseError> for GlowUpError {
    fn from(err: AddrParseError) -> Self {
        GlowUpError::Parse(err.to_string())
    }
}

impl<T> From<PoisonError<T>> for GlowUpError {
    fn from(err: PoisonError<T>) -> Self {
        GlowUpError::Internal(format!("Mutex poison error: {}", err))
    }
}

impl From<serde_json::Error> for GlowUpError {
    fn from(err: serde_json::Error) -> Self {
        GlowUpError::Json(err.to_string())
    }
}

// Helper functions for creating specific error types
impl GlowUpError {
    pub fn io_error(msg: impl Into<String>) -> Self {
        GlowUpError::Io(msg.into())
    }
    
    pub fn parse_error(msg: impl Into<String>) -> Self {
        GlowUpError::Parse(msg.into())
    }
    
    pub fn connection_error(msg: impl Into<String>) -> Self {
        GlowUpError::Connection(msg.into())
    }
    
    pub fn timeout_error(msg: impl Into<String>) -> Self {
        GlowUpError::Timeout(msg.into())
    }
    
    pub fn http_error(code: u16, msg: impl Into<String>) -> Self {
        GlowUpError::Http(code, msg.into())
    }
    
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        GlowUpError::InvalidRequest(msg.into())
    }
    
    pub fn internal_error(msg: impl Into<String>) -> Self {
        GlowUpError::Internal(msg.into())
    }
    
    pub fn timeout(msg: impl Into<String>) -> Self {
        GlowUpError::Timeout(msg.into())
    }
    
    pub fn bad_request(msg: impl Into<String>) -> Self {
        GlowUpError::Http(400, msg.into())
    }
    
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        GlowUpError::Http(401, msg.into())
    }
    
    pub fn forbidden(msg: impl Into<String>) -> Self {
        GlowUpError::Http(403, msg.into())
    }
    
    pub fn not_found(msg: impl Into<String>) -> Self {
        GlowUpError::Http(404, msg.into())
    }
    
    pub fn rate_limited(msg: impl Into<String>) -> Self {
        GlowUpError::Http(429, msg.into())
    }
    
    pub fn server_error(msg: impl Into<String>) -> Self {
        GlowUpError::Http(500, msg.into())
    }
    
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        GlowUpError::InvalidRequest(msg.into())
    }
}
