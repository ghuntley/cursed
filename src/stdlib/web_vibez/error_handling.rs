use crate::web::StatusCode;
use crate::error::Error;
/// Error handling for the web_vibez framework
/// 
/// Provides comprehensive error types and handling for routing,
/// middleware, and handler operations

use std::fmt;
use std::error::Error;

/// Router-specific errors
#[derive(Debug, Clone)]
pub enum RouterError {
    /// Invalid route pattern
    InvalidPattern(String, String),
    /// Route already exists
    DuplicateRoute(String),
    /// Route not found
    RouteNotFound(String),
    /// Too many routes registered
    TooManyRoutes(usize),
    /// Route priority conflict
    PriorityConflict(String, String),
    /// Invalid route configuration
    InvalidConfiguration(String),
}

impl fmt::Display for RouterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouterError::InvalidPattern(pattern, reason) => {
                write!(f, "Invalid route pattern '{}': {}", pattern, reason)
            }
            RouterError::DuplicateRoute(route) => {
                write!(f, "Route '{}' already exists", route)
            }
            RouterError::RouteNotFound(route) => {
                write!(f, "Route '{}' not found", route)
            }
            RouterError::TooManyRoutes(count) => {
                write!(f, "Too many routes registered: {}", count)
            }
            RouterError::PriorityConflict(route1, route2) => {
                write!(f, "Priority conflict between routes '{}' and '{}'", route1, route2)
            }
            RouterError::InvalidConfiguration(reason) => {
                write!(f, "Invalid router configuration: {}", reason)
            }
        }
    }
}

impl Error for RouterError {}

impl From<String> for RouterError {
    fn from(error: String) -> Self {
        RouterError::InvalidConfiguration(error)
    }
}

/// Middleware-specific errors
#[derive(Debug, Clone)]
pub enum MiddlewareError {
    /// Authentication failed
    Authentication(String),
    /// Authorization failed
    Authorization(String),
    /// Rate limit exceeded
    RateLimit(String),
    /// File system error
    FileSystem(String),
    /// Network error
    Network(String),
    /// Timeout error
    Timeout(String),
    /// Configuration error
    Configuration(String),
    /// Security violation
    Security(String),
    /// Validation error
    Validation(String),
    /// External service error
    ExternalService(String),
    /// Custom middleware error
    Custom(String),
}

impl fmt::Display for MiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MiddlewareError::Authentication(msg) => write!(f, "Authentication error: {}", msg),
            MiddlewareError::Authorization(msg) => write!(f, "Authorization error: {}", msg),
            MiddlewareError::RateLimit(msg) => write!(f, "Rate limit error: {}", msg),
            MiddlewareError::FileSystem(msg) => write!(f, "File system error: {}", msg),
            MiddlewareError::Network(msg) => write!(f, "Network error: {}", msg),
            MiddlewareError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            MiddlewareError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            MiddlewareError::Security(msg) => write!(f, "Security error: {}", msg),
            MiddlewareError::Validation(msg) => write!(f, "Validation error: {}", msg),
            MiddlewareError::ExternalService(msg) => write!(f, "External service error: {}", msg),
            MiddlewareError::Custom(msg) => write!(f, "Middleware error: {}", msg),
        }
    }
}

impl Error for MiddlewareError {}

/// Handler-specific errors
#[derive(Debug, Clone)]
pub enum HandlerError {
    /// Request processing error
    RequestProcessing(String),
    /// Response generation error
    ResponseGeneration(String),
    /// Data serialization error
    Serialization(String),
    /// Data deserialization error
    Deserialization(String),
    /// File system error
    FileSystem(String),
    /// Database error
    Database(String),
    /// External API error
    ExternalApi(String),
    /// Business logic error
    BusinessLogic(String),
    /// Validation error
    Validation(String),
    /// Network communication error
    Network(String),
    /// Configuration error
    Configuration(String),
    /// Not implemented
    NotImplemented(String),
    /// Internal error
    Internal(String),
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandlerError::RequestProcessing(msg) => write!(f, "Request processing error: {}", msg),
            HandlerError::ResponseGeneration(msg) => write!(f, "Response generation error: {}", msg),
            HandlerError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            HandlerError::Deserialization(msg) => write!(f, "Deserialization error: {}", msg),
            HandlerError::FileSystem(msg) => write!(f, "File system error: {}", msg),
            HandlerError::Database(msg) => write!(f, "Database error: {}", msg),
            HandlerError::ExternalApi(msg) => write!(f, "External API error: {}", msg),
            HandlerError::BusinessLogic(msg) => write!(f, "Business logic error: {}", msg),
            HandlerError::Validation(msg) => write!(f, "Validation error: {}", msg),
            HandlerError::Network(msg) => write!(f, "Network error: {}", msg),
            HandlerError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            HandlerError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            HandlerError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl Error for HandlerError {}

/// Comprehensive web framework error
#[derive(Debug)]
pub enum WebVibezError {
    /// Router error
    Router(RouterError),
    /// Middleware error
    Middleware(MiddlewareError),
    /// Handler error
    Handler(HandlerError),
    /// Configuration error
    Configuration(String),
    /// System error
    System(String),
}

impl fmt::Display for WebVibezError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebVibezError::Router(err) => write!(f, "Router: {}", err),
            WebVibezError::Middleware(err) => write!(f, "Middleware: {}", err),
            WebVibezError::Handler(err) => write!(f, "Handler: {}", err),
            WebVibezError::Configuration(msg) => write!(f, "Configuration: {}", msg),
            WebVibezError::System(msg) => write!(f, "System: {}", msg),
        }
    }
}

impl Error for WebVibezError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WebVibezError::Router(err) => Some(err),
            WebVibezError::Middleware(err) => Some(err),
            WebVibezError::Handler(err) => Some(err),
            _ => None,
        }
    }
}

impl From<RouterError> for WebVibezError {
    fn from(err: RouterError) -> Self {
        WebVibezError::Router(err)
    }
}

impl From<MiddlewareError> for WebVibezError {
    fn from(err: MiddlewareError) -> Self {
        WebVibezError::Middleware(err)
    }
}

impl From<HandlerError> for WebVibezError {
    fn from(err: HandlerError) -> Self {
        WebVibezError::Handler(err)
    }
}

/// Error context for providing additional information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Error location (file, line, function)
    pub location: Option<String>,
    /// Request ID for tracing
    pub request_id: Option<String>,
    /// User ID if available
    pub user_id: Option<String>,
    /// Additional context data
    pub data: std::collections::HashMap<String, String>,
    /// Timestamp when error occurred
    pub timestamp: std::time::SystemTime,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            location: None,
            request_id: None,
            user_id: None,
            data: std::collections::HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        }
    }

    pub fn with_location(mut self, location: &str) -> Self {
        self.location = Some(location.to_string());
        self
    }

    pub fn with_request_id(mut self, request_id: &str) -> Self {
        self.request_id = Some(request_id.to_string());
        self
    }

    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.data.insert(key.to_string(), value.to_string());
        self
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced error with context
#[derive(Debug)]
pub struct ContextualError {
    /// The underlying error
    pub error: WebVibezError,
    /// Error context
    pub context: ErrorContext,
}

impl ContextualError {
    pub fn new(error: WebVibezError) -> Self {
        Self {
            error,
            context: ErrorContext::new(),
        }
    }

    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    pub fn add_context(mut self, key: &str, value: &str) -> Self {
        self.context.data.insert(key.to_string(), value.to_string());
        self
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;
        
        if let Some(request_id) = &self.context.request_id {
            write!(f, " (request: {})", request_id)?;
        }
        
        if let Some(location) = &self.context.location {
            write!(f, " at {}", location)?;
        }
        
        Ok(())
    }
}

impl Error for ContextualError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// Error response builder for HTTP responses
#[derive(Debug)]
pub struct ErrorResponse {
    /// HTTP status code
    pub status: u16,
    /// Error code for API clients
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Detailed error information
    pub details: Option<serde_json::Value>,
    /// Request ID for tracking
    pub request_id: Option<String>,
    /// Timestamp
    pub timestamp: String,
}

impl ErrorResponse {
    pub fn from_error(error: &WebVibezError) -> Self {
        let (status, code, message) = match error {
            WebVibezError::Router(RouterError::RouteNotFound(_)) => {
                (404, "ROUTE_NOT_FOUND", "Route not found")
            }
            WebVibezError::Router(RouterError::InvalidPattern(_, _)) => {
                (400, "INVALID_ROUTE_PATTERN", "Invalid route pattern")
            }
            WebVibezError::Middleware(MiddlewareError::Authentication(_)) => {
                (401, "AUTHENTICATION_FAILED", "Authentication required")
            }
            WebVibezError::Middleware(MiddlewareError::Authorization(_)) => {
                (403, "AUTHORIZATION_FAILED", "Access denied")
            }
            WebVibezError::Middleware(MiddlewareError::RateLimit(_)) => {
                (429, "RATE_LIMIT_EXCEEDED", "Rate limit exceeded")
            }
            WebVibezError::Middleware(MiddlewareError::Validation(_)) => {
                (400, "VALIDATION_ERROR", "Validation failed")
            }
            WebVibezError::Handler(HandlerError::NotImplemented(_)) => {
                (501, "NOT_IMPLEMENTED", "Feature not implemented")
            }
            WebVibezError::Handler(HandlerError::Validation(_)) => {
                (400, "VALIDATION_ERROR", "Request validation failed")
            }
            _ => (500, "INTERNAL_ERROR", "Internal server error"),
        };

        Self {
            status,
            code: code.to_string(),
            message: message.to_string(),
            details: Some(serde_json::json!({
                "error": error.to_string()
            })),
            request_id: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_request_id(mut self, request_id: &str) -> Self {
        self.request_id = Some(request_id.to_string());
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut json = serde_json::json!({
            "error": {
                "code": self.code,
                "message": self.message,
                "timestamp": self.timestamp
            }
        });

        if let Some(request_id) = &self.request_id {
            json["error"]["request_id"] = serde_json::json!(request_id);
        }

        if let Some(details) = &self.details {
            json["error"]["details"] = details.clone();
        }

        json
    }
}

/// Error handler trait for custom error processing
pub trait ErrorHandler: Send + Sync {
    /// Handle an error and generate appropriate response
    fn handle_error(
        &self,
        error: &WebVibezError,
        context: &crate::stdlib::web_vibez::context::RequestContext,
        response: &mut crate::stdlib::web_vibez::context::ResponseContext,
    );

    /// Check if this handler can process the given error
    fn can_handle(&self, error: &WebVibezError) -> bool;

    /// Get handler priority (lower = higher priority)
    fn priority(&self) -> u32 {
        100
    }
}

/// Default error handler implementation
#[derive(Debug)]
pub struct DefaultErrorHandler {
    /// Include detailed error information in responses
    include_details: bool,
    /// Include stack traces in development mode
    include_stack_trace: bool,
}

impl DefaultErrorHandler {
    pub fn new() -> Self {
        Self {
            include_details: false,
            include_stack_trace: false,
        }
    }

    pub fn with_details(mut self, include: bool) -> Self {
        self.include_details = include;
        self
    }

    pub fn with_stack_trace(mut self, include: bool) -> Self {
        self.include_stack_trace = include;
        self
    }
}

impl Default for DefaultErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(
        &self,
        error: &WebVibezError,
        context: &crate::stdlib::web_vibez::context::RequestContext,
        response: &mut crate::stdlib::web_vibez::context::ResponseContext,
    ) {
        use crate::stdlib::web_vibez::StatusCode;

        let mut error_response = ErrorResponse::from_error(error)
            .with_request_id(&context.request_id);

        if !self.include_details {
            error_response.details = None;
        }

        response.set_status(StatusCode(error_response.status));
        response.set_header("Content-Type", "application/json");
        
        if let Ok(json_data) = serde_json::to_vec(&error_response.to_json()) {
            response.set_body(json_data);
        } else {
            // Fallback if JSON serialization fails
            response.set_body_string(&format!(
                r#"{{"error":{{"code":"{}","message":"{}"}}}}"#,
                error_response.code, error_response.message
            ));
        }
    }

    fn can_handle(&self, _error: &WebVibezError) -> bool {
        true // Default handler can handle any error
    }

    fn priority(&self) -> u32 {
        1000 // Lowest priority - fallback handler
    }
}

/// Convenience macros for error creation
#[macro_export]
macro_rules! router_error {
    ($variant:ident, $($arg:expr),*) => {
        $crate::stdlib::web_vibez::error_handling::RouterError::$variant($($arg),*)
    };
}

#[macro_export]
macro_rules! middleware_error {
    ($variant:ident, $($arg:expr),*) => {
        $crate::stdlib::web_vibez::error_handling::MiddlewareError::$variant($($arg),*)
    };
}

#[macro_export]
macro_rules! handler_error {
    ($variant:ident, $($arg:expr),*) => {
        $crate::stdlib::web_vibez::error_handling::HandlerError::$variant($($arg),*)
    };
}

/// Add chrono dependency (would be in Cargo.toml)
mod chrono {
    use std::time::SystemTime;
    
    pub struct Utc;
    
    impl Utc {
        pub fn now() -> DateTime {
            DateTime(SystemTime::now())
        }
    }
    
    pub struct DateTime(SystemTime);
    
    impl DateTime {
        pub fn to_rfc3339(&self) -> String {
            // Simplified RFC3339 formatting
            format!("{:?}", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_error_display() {
        let error = RouterError::InvalidPattern("/invalid[".to_string(), "unclosed bracket".to_string());
        assert_eq!(error.to_string(), "Invalid route pattern '/invalid[': unclosed bracket");
    }

    #[test]
    fn test_middleware_error_display() {
        let error = MiddlewareError::Authentication("Invalid token".to_string());
        assert_eq!(error.to_string(), "Authentication error: Invalid token");
    }

    #[test]
    fn test_handler_error_display() {
        let error = HandlerError::NotImplemented("Feature X".to_string());
        assert_eq!(error.to_string(), "Not implemented: Feature X");
    }

    #[test]
    fn test_error_response() {
        let error = WebVibezError::Middleware(MiddlewareError::Authentication("Bad token".to_string()));
        let response = ErrorResponse::from_error(&error);
        
        assert_eq!(response.status, 401);
        assert_eq!(response.code, "AUTHENTICATION_FAILED");
        assert_eq!(response.message, "Authentication required");
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new()
            .with_request_id("req_123")
            .with_location("handler.rs:42")
            .with_data("user_id", "user_456");
            
        assert_eq!(context.request_id, Some("req_123".to_string()));
        assert_eq!(context.location, Some("handler.rs:42".to_string()));
        assert_eq!(context.data.get("user_id"), Some(&"user_456".to_string()));
    }

    #[test]
    fn test_contextual_error() {
        let error = WebVibezError::Handler(HandlerError::Internal("Database connection failed".to_string()));
        let context = ErrorContext::new().with_request_id("req_789");
        let contextual = ContextualError::new(error).with_context(context);
        
        assert!(contextual.to_string().contains("req_789"));
        assert!(contextual.to_string().contains("Database connection failed"));
    }
}
