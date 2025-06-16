/// fr fr Rate limiting error types - comprehensive error handling
use std::fmt;

/// fr fr Rate limit error - all possible error types
#[derive(Debug, Clone)]
pub enum RateLimitError {
    /// fr fr Store operation failed - storage issues
    StoreError(String),
    
    /// fr fr Algorithm error - computation issues
    AlgorithmError(String),
    
    /// fr fr Configuration error - invalid settings
    ConfigError(String),
    
    /// fr fr Client identification error - ID extraction issues
    ClientIdentificationError(String),
    
    /// fr fr Time calculation error - timestamp issues
    TimeError(String),
    
    /// fr fr Serialization error - data conversion issues
    SerializationError(String),
    
    /// fr fr Network error - communication issues
    NetworkError(String),
    
    /// fr fr Concurrent access error - threading issues
    ConcurrencyError(String),
    
    /// fr fr Limit exceeded - rate limit violation
    LimitExceeded {
        limit: u64,
        current: u64,
        retry_after: u64,
    },
    
    /// fr fr Invalid client - malformed client ID
    InvalidClient(String),
    
    /// fr fr System overload - too much load
    SystemOverload(String),
    
    /// fr fr General error - catch-all
    General(String),
}

impl RateLimitError {
    /// fr fr Create store error
    pub fn store_error(message: impl Into<String>) -> Self {
        Self::StoreError(message.into())
    }

    /// fr fr Create algorithm error
    pub fn algorithm_error(message: impl Into<String>) -> Self {
        Self::AlgorithmError(message.into())
    }

    /// fr fr Create config error
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError(message.into())
    }

    /// fr fr Create client identification error
    pub fn client_id_error(message: impl Into<String>) -> Self {
        Self::ClientIdentificationError(message.into())
    }

    /// fr fr Create time error
    pub fn time_error(message: impl Into<String>) -> Self {
        Self::TimeError(message.into())
    }

    /// fr fr Create serialization error
    pub fn serialization_error(message: impl Into<String>) -> Self {
        Self::SerializationError(message.into())
    }

    /// fr fr Create network error
    pub fn network_error(message: impl Into<String>) -> Self {
        Self::NetworkError(message.into())
    }

    /// fr fr Create concurrency error
    pub fn concurrency_error(message: impl Into<String>) -> Self {
        Self::ConcurrencyError(message.into())
    }

    /// fr fr Create limit exceeded error
    pub fn limit_exceeded(limit: u64, current: u64, retry_after: u64) -> Self {
        Self::LimitExceeded {
            limit,
            current,
            retry_after,
        }
    }

    /// fr fr Create invalid client error
    pub fn invalid_client(message: impl Into<String>) -> Self {
        Self::InvalidClient(message.into())
    }

    /// fr fr Create system overload error
    pub fn system_overload(message: impl Into<String>) -> Self {
        Self::SystemOverload(message.into())
    }

    /// fr fr Create general error
    pub fn general(message: impl Into<String>) -> Self {
        Self::General(message.into())
    }

    /// fr fr Check if error is recoverable - can retry
    pub fn is_recoverable(&self) -> bool {
        match self {
            RateLimitError::LimitExceeded { .. } => true,
            RateLimitError::SystemOverload(_) => true,
            RateLimitError::NetworkError(_) => true,
            RateLimitError::ConcurrencyError(_) => true,
            _ => false,
        }
    }

    /// fr fr Get error category - classification
    pub fn category(&self) -> ErrorCategory {
        match self {
            RateLimitError::StoreError(_) => ErrorCategory::Storage,
            RateLimitError::AlgorithmError(_) => ErrorCategory::Algorithm,
            RateLimitError::ConfigError(_) => ErrorCategory::Configuration,
            RateLimitError::ClientIdentificationError(_) => ErrorCategory::Client,
            RateLimitError::TimeError(_) => ErrorCategory::Time,
            RateLimitError::SerializationError(_) => ErrorCategory::Serialization,
            RateLimitError::NetworkError(_) => ErrorCategory::Network,
            RateLimitError::ConcurrencyError(_) => ErrorCategory::Concurrency,
            RateLimitError::LimitExceeded { .. } => ErrorCategory::RateLimit,
            RateLimitError::InvalidClient(_) => ErrorCategory::Client,
            RateLimitError::SystemOverload(_) => ErrorCategory::System,
            RateLimitError::General(_) => ErrorCategory::General,
        }
    }

    /// fr fr Get HTTP status code - web response
    pub fn http_status_code(&self) -> u16 {
        match self {
            RateLimitError::LimitExceeded { .. } => 429,
            RateLimitError::InvalidClient(_) => 400,
            RateLimitError::ClientIdentificationError(_) => 400,
            RateLimitError::SystemOverload(_) => 503,
            RateLimitError::ConfigError(_) => 500,
            RateLimitError::StoreError(_) => 500,
            RateLimitError::AlgorithmError(_) => 500,
            RateLimitError::ConcurrencyError(_) => 503,
            RateLimitError::NetworkError(_) => 502,
            _ => 500,
        }
    }

    /// fr fr Get retry after seconds - when to retry
    pub fn retry_after_seconds(&self) -> Option<u64> {
        match self {
            RateLimitError::LimitExceeded { retry_after, .. } => Some(*retry_after),
            RateLimitError::SystemOverload(_) => Some(60), // Retry after 1 minute
            RateLimitError::ConcurrencyError(_) => Some(1), // Retry after 1 second
            RateLimitError::NetworkError(_) => Some(30), // Retry after 30 seconds
            _ => None,
        }
    }
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimitError::StoreError(msg) => write!(f, "Store error: {}", msg),
            RateLimitError::AlgorithmError(msg) => write!(f, "Algorithm error: {}", msg),
            RateLimitError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            RateLimitError::ClientIdentificationError(msg) => write!(f, "Client identification error: {}", msg),
            RateLimitError::TimeError(msg) => write!(f, "Time error: {}", msg),
            RateLimitError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            RateLimitError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            RateLimitError::ConcurrencyError(msg) => write!(f, "Concurrency error: {}", msg),
            RateLimitError::LimitExceeded { limit, current, retry_after } => {
                write!(f, "Rate limit exceeded: {}/{} requests, retry after {} seconds", current, limit, retry_after)
            }
            RateLimitError::InvalidClient(msg) => write!(f, "Invalid client: {}", msg),
            RateLimitError::SystemOverload(msg) => write!(f, "System overload: {}", msg),
            RateLimitError::General(msg) => write!(f, "Rate limit error: {}", msg),
        }
    }
}

impl std::error::Error for RateLimitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// fr fr Error category - classification of errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// fr fr Storage related errors
    Storage,
    
    /// fr fr Algorithm computation errors
    Algorithm,
    
    /// fr fr Configuration errors
    Configuration,
    
    /// fr fr Client identification errors
    Client,
    
    /// fr fr Time calculation errors
    Time,
    
    /// fr fr Data serialization errors
    Serialization,
    
    /// fr fr Network communication errors
    Network,
    
    /// fr fr Concurrency/threading errors
    Concurrency,
    
    /// fr fr Rate limit violations
    RateLimit,
    
    /// fr fr System overload errors
    System,
    
    /// fr fr General uncategorized errors
    General,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ErrorCategory::Storage => "Storage",
            ErrorCategory::Algorithm => "Algorithm",
            ErrorCategory::Configuration => "Configuration",
            ErrorCategory::Client => "Client",
            ErrorCategory::Time => "Time",
            ErrorCategory::Serialization => "Serialization",
            ErrorCategory::Network => "Network",
            ErrorCategory::Concurrency => "Concurrency",
            ErrorCategory::RateLimit => "RateLimit",
            ErrorCategory::System => "System",
            ErrorCategory::General => "General",
        };
        write!(f, "{}", name)
    }
}

/// fr fr Rate limit result type - convenience alias
pub type RateLimitResult<T> = Result<T, RateLimitError>;

/// fr fr Error context - additional error information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// fr fr Operation that failed
    pub operation: String,
    
    /// fr fr Client ID involved
    pub client_id: Option<String>,
    
    /// fr fr Timestamp of error
    pub timestamp: u64,
    
    /// fr fr Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// fr fr Create new error context
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            client_id: None,
            timestamp: super::current_timestamp(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// fr fr Set client ID
    pub fn with_client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// fr fr Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// fr fr Contextual error - error with additional context
#[derive(Debug, Clone)]
pub struct ContextualError {
    /// fr fr Base error
    pub error: RateLimitError,
    
    /// fr fr Error context
    pub context: ErrorContext,
}

impl ContextualError {
    /// fr fr Create new contextual error
    pub fn new(error: RateLimitError, context: ErrorContext) -> Self {
        Self { error, context }
    }

    /// fr fr Create with operation context
    pub fn with_operation(error: RateLimitError, operation: impl Into<String>) -> Self {
        Self::new(error, ErrorContext::new(operation))
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (operation: {}", self.error, self.context.operation)?;
        
        if let Some(client_id) = &self.context.client_id {
            write!(f, ", client: {}", client_id)?;
        }
        
        write!(f, ", timestamp: {})", self.context.timestamp)
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// fr fr Error aggregator - collect multiple errors
#[derive(Debug, Clone)]
pub struct ErrorAggregator {
    errors: Vec<ContextualError>,
}

impl ErrorAggregator {
    /// fr fr Create new error aggregator
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
        }
    }

    /// fr fr Add error to aggregator
    pub fn add(&mut self, error: ContextualError) {
        self.errors.push(error);
    }

    /// fr fr Add simple error with operation
    pub fn add_error(&mut self, error: RateLimitError, operation: impl Into<String>) {
        self.add(ContextualError::with_operation(error, operation));
    }

    /// fr fr Check if has errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// fr fr Get error count
    pub fn count(&self) -> usize {
        self.errors.len()
    }

    /// fr fr Get errors by category
    pub fn errors_by_category(&self, category: ErrorCategory) -> Vec<&ContextualError> {
        self.errors
            .iter()
            .filter(|e| e.error.category() == category)
            .collect()
    }

    /// fr fr Get all errors
    pub fn errors(&self) -> &[ContextualError] {
        &self.errors
    }

    /// fr fr Clear all errors
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}

impl Default for ErrorAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ErrorAggregator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            write!(f, "No errors")
        } else {
            write!(f, "{} errors:", self.errors.len())?;
            for (i, error) in self.errors.iter().enumerate() {
                write!(f, "\n  {}: {}", i + 1, error)?;
            }
            Ok(())
        }
    }
}

/// fr fr Convert from std::io::Error
impl From<std::io::Error> for RateLimitError {
    fn from(error: std::io::Error) -> Self {
        RateLimitError::StoreError(error.to_string())
    }
}

/// fr fr Convert from serde_json::Error
impl From<serde_json::Error> for RateLimitError {
    fn from(error: serde_json::Error) -> Self {
        RateLimitError::SerializationError(error.to_string())
    }
}

/// fr fr Convert from std::time::SystemTimeError
impl From<std::time::SystemTimeError> for RateLimitError {
    fn from(error: std::time::SystemTimeError) -> Self {
        RateLimitError::TimeError(error.to_string())
    }
}

/// fr fr Convert from config error
impl From<super::config::ConfigError> for RateLimitError {
    fn from(error: super::config::ConfigError) -> Self {
        RateLimitError::ConfigError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let store_error = RateLimitError::store_error("Connection failed");
        assert!(matches!(store_error, RateLimitError::StoreError(_)));
        
        let limit_error = RateLimitError::limit_exceeded(100, 150, 60);
        assert!(matches!(limit_error, RateLimitError::LimitExceeded { .. }));
    }

    #[test]
    fn test_error_properties() {
        let recoverable_error = RateLimitError::limit_exceeded(10, 15, 30);
        assert!(recoverable_error.is_recoverable());
        assert_eq!(recoverable_error.http_status_code(), 429);
        assert_eq!(recoverable_error.retry_after_seconds(), Some(30));
        assert_eq!(recoverable_error.category(), ErrorCategory::RateLimit);
        
        let config_error = RateLimitError::config_error("Invalid setting");
        assert!(!config_error.is_recoverable());
        assert_eq!(config_error.http_status_code(), 500);
        assert_eq!(config_error.retry_after_seconds(), None);
        assert_eq!(config_error.category(), ErrorCategory::Configuration);
    }

    #[test]
    fn test_error_display() {
        let error = RateLimitError::limit_exceeded(100, 150, 60);
        let display = format!("{}", error);
        assert!(display.contains("Rate limit exceeded"));
        assert!(display.contains("150/100"));
        assert!(display.contains("60 seconds"));
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new("check_request")
            .with_client_id("192.168.1.1")
            .with_metadata("algorithm", "fixed_window");
        
        assert_eq!(context.operation, "check_request");
        assert_eq!(context.client_id, Some("192.168.1.1".to_string()));
        assert_eq!(context.metadata.get("algorithm"), Some(&"fixed_window".to_string()));
    }

    #[test]
    fn test_contextual_error() {
        let error = RateLimitError::limit_exceeded(10, 15, 30);
        let context = ErrorContext::new("check_request").with_client_id("test_client");
        let contextual_error = ContextualError::new(error, context);
        
        let display = format!("{}", contextual_error);
        assert!(display.contains("Rate limit exceeded"));
        assert!(display.contains("check_request"));
        assert!(display.contains("test_client"));
    }

    #[test]
    fn test_error_aggregator() {
        let mut aggregator = ErrorAggregator::new();
        assert!(!aggregator.has_errors());
        assert_eq!(aggregator.count(), 0);
        
        aggregator.add_error(RateLimitError::store_error("Error 1"), "operation_1");
        aggregator.add_error(RateLimitError::config_error("Error 2"), "operation_2");
        
        assert!(aggregator.has_errors());
        assert_eq!(aggregator.count(), 2);
        
        let storage_errors = aggregator.errors_by_category(ErrorCategory::Storage);
        assert_eq!(storage_errors.len(), 1);
        
        let config_errors = aggregator.errors_by_category(ErrorCategory::Configuration);
        assert_eq!(config_errors.len(), 1);
        
        aggregator.clear();
        assert!(!aggregator.has_errors());
    }

    #[test]
    fn test_error_categories() {
        let categories = [
            ErrorCategory::Storage,
            ErrorCategory::Algorithm,
            ErrorCategory::Configuration,
            ErrorCategory::Client,
            ErrorCategory::Time,
            ErrorCategory::Serialization,
            ErrorCategory::Network,
            ErrorCategory::Concurrency,
            ErrorCategory::RateLimit,
            ErrorCategory::System,
            ErrorCategory::General,
        ];
        
        for category in &categories {
            let display = format!("{}", category);
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn test_error_conversions() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let rate_limit_error: RateLimitError = io_error.into();
        assert!(matches!(rate_limit_error, RateLimitError::StoreError(_)));
        
        let json_error = serde_json::from_str::<u32>("invalid json");
        if let Err(json_err) = json_error {
            let rate_limit_error: RateLimitError = json_err.into();
            assert!(matches!(rate_limit_error, RateLimitError::SerializationError(_)));
        }
    }
}
