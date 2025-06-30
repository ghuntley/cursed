use std::fmt;

/// Result type for rate limit operations
pub type RateLimitResult<T> = Result<T, RateLimitError>;

/// Rate limit specific error types
#[derive(Debug, Clone)]
pub enum RateLimitError {
    StoreError(String),
    ConfigurationError(String),
    AlgorithmError(String),
    NetworkError(String),
    TimeoutError,
    InternalError(String),
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitError::StoreError(msg) => write!(f, "Store error: {}", msg),
            RateLimitError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            RateLimitError::AlgorithmError(msg) => write!(f, "Algorithm error: {}", msg),
            RateLimitError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            RateLimitError::TimeoutError => write!(f, "Timeout error"),
            RateLimitError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for RateLimitError {}

/// Error category classification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorCategory {
    Configuration,
    Network,
    Storage,
    Algorithm,
    Timeout,
    Internal,
}

impl From<RateLimitError> for ErrorCategory {
    fn from(error: RateLimitError) -> Self {
        match error {
            RateLimitError::StoreError(_) => ErrorCategory::Storage,
            RateLimitError::ConfigurationError(_) => ErrorCategory::Configuration,
            RateLimitError::AlgorithmError(_) => ErrorCategory::Algorithm,
            RateLimitError::NetworkError(_) => ErrorCategory::Network,
            RateLimitError::TimeoutError => ErrorCategory::Timeout,
            RateLimitError::InternalError(_) => ErrorCategory::Internal,
        }
    }
}
