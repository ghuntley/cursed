use crate::web::StatusCode;
use crate::error::CursedError;
/// CursedError handling for the web_vibez framework
/// 
/// Provides comprehensive error types and handling for routing,
/// middleware, and handler operations

use std::fmt;

/// Router-specific errors
#[derive(Debug, Clone)]
pub enum RouterError {
    /// Invalid route pattern
    /// Route already exists
    /// Route not found
    /// Too many routes registered
    /// Route priority conflict
    /// Invalid route configuration
// impl fmt::Display for RouterError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             RouterError::InvalidPattern(pattern, reason) => {
//                 write!(f, "Invalid route pattern '{}': {}", pattern, reason)
//             }
//             RouterError::DuplicateRoute(route) => {
//                 write!(f, "Route '{}' already exists", route)
//             }
//             RouterError::RouteNotFound(route) => {
//                 write!(f, "Route '{}' not found", route)
//             }
//             RouterError::TooManyRoutes(count) => {
//                 write!(f, "Too many routes registered: {}", count)
//             }
//             RouterError::PriorityConflict(route1, route2) => {
//                 write!(f, "Priority conflict between routes '{}' and '{}'", route1, route2)
//             }
//             RouterError::InvalidConfiguration(reason) => {
//                 write!(f, "Invalid router configuration: {}", reason)
//             }
//         }
//     }
// }

// impl CursedError for RouterError {}
// 
impl From<String> for RouterError {
    fn from(error: String) -> Self {
        RouterError::InvalidConfiguration(error)
    }
}

/// Middleware-specific errors
#[derive(Debug, Clone)]
pub enum MiddlewareError {
    /// Authentication failed
    /// Authorization failed
    /// Rate limit exceeded
    /// File system error
    /// Network error
    /// Timeout error
    /// Configuration error
    /// Security violation
    /// Validation error
    /// External service error
    /// Custom middleware error
// impl fmt::Display for MiddlewareError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             MiddlewareError::Authentication(msg) => write!(f, "Authentication error: {}", msg),
//             MiddlewareError::Authorization(msg) => write!(f, "Authorization error: {}", msg),
//             MiddlewareError::RateLimit(msg) => write!(f, "Rate limit error: {}", msg),
//             MiddlewareError::FileSystem(msg) => write!(f, "File system error: {}", msg),
//             MiddlewareError::Network(msg) => write!(f, "Network error: {}", msg),
//             MiddlewareError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
//             MiddlewareError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
//             MiddlewareError::Security(msg) => write!(f, "Security error: {}", msg),
//             MiddlewareError::Validation(msg) => write!(f, "Validation error: {}", msg),
//             MiddlewareError::ExternalService(msg) => write!(f, "External service error: {}", msg),
//             MiddlewareError::Custom(msg) => write!(f, "Middleware error: {}", msg),
//         }
//     }
// }

// impl CursedError for MiddlewareError {}
// 
/// Handler-specific errors
#[derive(Debug, Clone)]
pub enum HandlerError {
    /// Request processing error
    /// Response generation error
    /// Data serialization error
    /// Data deserialization error
    /// File system error
    /// Database error
    /// External API error
    /// Business logic error
    /// Validation error
    /// Network communication error
    /// Configuration error
    /// Not implemented
    /// Internal error
// impl fmt::Display for HandlerError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             HandlerError::RequestProcessing(msg) => write!(f, "Request processing error: {}", msg),
//             HandlerError::ResponseGeneration(msg) => write!(f, "Response generation error: {}", msg),
//             HandlerError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
//             HandlerError::Deserialization(msg) => write!(f, "Deserialization error: {}", msg),
//             HandlerError::FileSystem(msg) => write!(f, "File system error: {}", msg),
//             HandlerError::Database(msg) => write!(f, "Database error: {}", msg),
//             HandlerError::ExternalApi(msg) => write!(f, "External API error: {}", msg),
//             HandlerError::BusinessLogic(msg) => write!(f, "Business logic error: {}", msg),
//             HandlerError::Validation(msg) => write!(f, "Validation error: {}", msg),
//             HandlerError::Network(msg) => write!(f, "Network error: {}", msg),
//             HandlerError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
//             HandlerError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
//             HandlerError::Internal(msg) => write!(f, "Internal error: {}", msg),
//         }
//     }
// }

// impl CursedError for HandlerError {}
// 
/// Comprehensive web framework error
#[derive(Debug)]
pub enum WebVibezError {
    /// Router error
    /// Middleware error
    /// Handler error
    /// Configuration error
    /// System error
// impl fmt::Display for WebVibezError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             WebVibezError::Router(err) => write!(f, "Router: {}", err),
//             WebVibezError::Middleware(err) => write!(f, "Middleware: {}", err),
//             WebVibezError::Handler(err) => write!(f, "Handler: {}", err),
//             WebVibezError::Configuration(msg) => write!(f, "Configuration: {}", msg),
//             WebVibezError::System(msg) => write!(f, "System: {}", msg),
//         }
//     }
// }

// impl CursedError for WebVibezError {
//     fn source(&self) -> Option<&(dyn CursedError + 'static)> {
//         match self {
//             WebVibezError::Router(err) => Some(err),
//             WebVibezError::Middleware(err) => Some(err),
//             WebVibezError::Handler(err) => Some(err),
//             _ => None,
//         }
//     }
// }

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

/// CursedError context for providing additional information
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// CursedError location (file, line, function)
    /// Request ID for tracing
    /// User ID if available
    /// Additional context data
    /// Timestamp when error occurred
impl ErrorContext {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn with_location(mut self, location: &str) -> Self {
        self.location = Some(location.to_string());
        self
    pub fn with_request_id(mut self, request_id: &str) -> Self {
        self.request_id = Some(request_id.to_string());
        self
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
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
    /// CursedError context
impl ContextualError {
    pub fn new(error: WebVibezError) -> Self {
        Self {
        }
    }

    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    pub fn add_context(mut self, key: &str, value: &str) -> Self {
        self.context.data.insert(key.to_string(), value.to_string());
        self
    }
}

// impl fmt::Display for ContextualError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.error)?;
//         
//         if let Some(request_id) = &self.context.request_id {
//             write!(f, " (request: {})", request_id)?;
//         }
//         
//         if let Some(location) = &self.context.location {
//             write!(f, " at {}", location)?;
//         }
//         
//         Ok(())
//     }
// }

// impl CursedError for ContextualError {
//     fn source(&self) -> Option<&(dyn CursedError + 'static)> {
//         Some(&self.error)
//     }
// }

/// CursedError response builder for HTTP responses
#[derive(Debug)]
pub struct ErrorResponse {
    /// HTTP status code
    /// CursedError code for API clients
    /// Human-readable error message
    /// Detailed error information
    /// Request ID for tracking
    /// Timestamp
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

        Self {
            details: Some(serde_json::json!({
                "error": error.to_string()
        }
    }

    pub fn with_request_id(mut self, request_id: &str) -> Self {
        self.request_id = Some(request_id.to_string());
        self
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    pub fn to_json(&self) -> serde_json::Value {
        let mut json = serde_json::json!({
            "error": {
                "timestamp": self.timestamp
            }
        });

        if let Some(request_id) = &self.request_id {
            json["error"]["request_id"] = serde_json::json!(request_id);
        if let Some(details) = &self.details {
            json["error"]["details"] = details.clone();
        json
    }
}

/// CursedError handler trait for custom error processing
pub trait ErrorHandler: Send + Sync {
    /// Handle an error and generate appropriate response
    fn handle_error(
//         context: &crate::stdlib::web_vibez::context::RequestContext,
//         response: &mut crate::stdlib::web_vibez::context::ResponseContext,
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
    /// Include stack traces in development mode
impl DefaultErrorHandler {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn with_details(mut self, include: bool) -> Self {
        self.include_details = include;
        self
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
//         context: &crate::stdlib::web_vibez::context::RequestContext,
//         response: &mut crate::stdlib::web_vibez::context::ResponseContext,
    ) {
//         use crate::stdlib::web_vibez::StatusCode;

        let mut error_response = ErrorResponse::from_error(error)
            .with_request_id(&context.request_id);

        if !self.include_details {
            error_response.details = None;
        response.set_status(StatusCode(error_response.status));
        response.set_header("Content-Type", "application/json");
        
        if let Ok(json_data) = serde_json::to_vec(&error_response.to_json()) {
            response.set_body(json_data);
        } else {
            // Fallback if JSON serialization fails
            response.set_body_string(&format!(
                error_response.code, error_response.message
            ));
        }
    }

    fn can_handle(&self, _error: &WebVibezError) -> bool {
        true // Default handler can handle any error
    fn priority(&self) -> u32 {
        1000 // Lowest priority - fallback handler
    }
}

/// Convenience macros for error creation
#[macro_export]
macro_rules! router_error {
    ($variant:ident, $($arg:expr),*) => {
//         $crate::stdlib::web_vibez::error_handling::RouterError::$variant($($arg),*)
#[macro_export]
macro_rules! middleware_error {
    ($variant:ident, $($arg:expr),*) => {
//         $crate::stdlib::web_vibez::error_handling::MiddlewareError::$variant($($arg),*)
#[macro_export]
macro_rules! handler_error {
    ($variant:ident, $($arg:expr),*) => {
//         $crate::stdlib::web_vibez::error_handling::HandlerError::$variant($($arg),*)
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
