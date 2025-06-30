use crate::error::CursedError;
use std::fmt;

/// Result type for web operations
pub type WebResult<T> = Result<T, WebError>;

/// Web-specific error types
#[derive(Debug, Clone)]
pub enum WebError {
    Network(NetworkErrorKind),
    Auth(AuthErrorKind),
    Generic(String),
    Runtime(CursedError),
}

#[derive(Debug, Clone)]
pub enum NetworkErrorKind {
    ConnectionFailed,
    Timeout,
    InvalidUrl,
    HttpError(u16),
}

#[derive(Debug, Clone)]
pub enum AuthErrorKind {
    Unauthorized,
    Forbidden,
    InvalidToken,
    ExpiredToken,
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub message: String,
    pub source: Option<String>,
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebError::Network(kind) => write!(f, "Network error: {:?}", kind),
            WebError::Auth(kind) => write!(f, "Auth error: {:?}", kind),
            WebError::Generic(msg) => write!(f, "Web error: {}", msg),
            WebError::Runtime(err) => write!(f, "Runtime error: {}", err),
        }
    }
}

impl std::error::Error for WebError {}

impl From<CursedError> for WebError {
    fn from(err: CursedError) -> Self {
        WebError::Runtime(err)
    }
}
