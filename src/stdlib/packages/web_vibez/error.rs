use crate::web::StatusCode;
use crate::error::CursedError;
/// fr fr CursedError handling for web_vibez package - comprehensive error management
use std::fmt;
// use crate::stdlib::packages::web_vibez::status::StatusCode as WebStatusCode;

/// fr fr Result type alias for web operations - clean error handling
pub type WebResult<T> = std::result::Result<T, WebError>;

/// fr fr Comprehensive error types for web operations - covers all the things
#[derive(Debug, Clone)]
pub enum WebError {
    /// fr fr HTTP-related errors with status codes
    Http {
    
    /// fr fr Network connectivity issues
    Network {
    
    /// fr fr Request parsing errors
    RequestParsing {
    
    /// fr fr Response building errors
    ResponseBuilding {
    
    /// fr fr Route matching errors
    Routing {
    
    /// fr fr Middleware execution errors
    Middleware {
    
    /// fr fr Server configuration errors
    Configuration {
    
    /// fr fr Authentication/authorization errors
    Auth {
    
    /// fr fr JSON serialization/deserialization errors
    Json {
    
    /// fr fr I/O errors from file system or network
    Io {
    
    /// fr fr Timeout errors
    Timeout {
    
    /// fr fr Rate limiting errors
    RateLimit {
    
    /// fr fr Custom application errors
    Custom {
/// fr fr Network error subcategories - specific networking issues
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkErrorKind {
    /// fr fr Connection refused by server
    /// fr fr Connection timeout
    /// fr fr DNS resolution failed
    /// fr fr SSL/TLS errors
    /// fr fr Network unreachable
    /// fr fr Other network error
/// fr fr Authentication error subcategories - auth-specific issues
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthErrorKind {
    /// fr fr Missing credentials
    /// fr fr Invalid credentials
    /// fr fr Expired token/session
    /// fr fr Insufficient permissions
    /// fr fr Rate limit exceeded
    /// fr fr Other auth error
impl WebError {
    /// fr fr Create HTTP error with status code - standard HTTP errors
    pub fn http(status: StatusCode, message: impl Into<String>) -> Self {
        Self::Http {
        }
    }

    /// fr fr Create HTTP error with source information - detailed context
    pub fn http_with_source(
    ) -> Self {
        Self::Http {
        }
    }

    /// fr fr Create bad request error - 400 with message
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::http(StatusCode::BadRequest, message)
    /// fr fr Create unauthorized error - 401 with message
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::http(StatusCode::Unauthorized, message)
    /// fr fr Create forbidden error - 403 with message
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::http(StatusCode::Forbidden, message)
    /// fr fr Create not found error - 404 with message
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::http(StatusCode::NotFound, message)
    /// fr fr Create method not allowed error - 405 with message
    pub fn method_not_allowed(message: impl Into<String>) -> Self {
        Self::http(StatusCode::MethodNotAllowed, message)
    /// fr fr Create internal server error - 500 with message
    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self::http(StatusCode::InternalServerError, message)
    /// fr fr Create network error - connection issues
    pub fn network(kind: NetworkErrorKind, message: impl Into<String>) -> Self {
        Self::Network {
        }
    }

    /// fr fr Create request parsing error - malformed requests
    pub fn request_parsing(message: impl Into<String>, field: Option<String>) -> Self {
        Self::RequestParsing {
        }
    }

    /// fr fr Create routing error - path/method matching issues
    pub fn routing(
    ) -> Self {
        Self::Routing {
        }
    }

    /// fr fr Create middleware error - middleware chain issues
    pub fn middleware(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Middleware {
        }
    }

    /// fr fr Create middleware error with inner error - error chain
    pub fn middleware_with_inner(
    ) -> Self {
        Self::Middleware {
        }
    }

    /// fr fr Create timeout error - operation took too long
    pub fn timeout(operation: impl Into<String>, duration_ms: u64) -> Self {
        Self::Timeout {
        }
    }

    /// fr fr Create rate limit error - too many requests
    pub fn rate_limit(
    ) -> Self {
        Self::RateLimit {
        }
    }

    /// fr fr Create custom error - application-specific errors
    pub fn custom(
    ) -> Self {
        Self::Custom {
        }
    }

    /// fr fr Get HTTP status code for this error - what to return to client
    pub fn status_code(&self) -> StatusCode {
        match self {
            WebError::Auth { kind, .. } => match kind {
                AuthErrorKind::MissingCredentials | AuthErrorKind::InvalidCredentials => {
                    StatusCode::Unauthorized
                }
            WebError::RateLimit { status_code, .. } => {
                StatusCode::from_u16(*status_code).unwrap_or(StatusCode::TooManyRequests)
            }
        }
    }

    /// fr fr Get error message - human-readable description
    pub fn message(&self) -> String {
        match self {
            WebError::Timeout { operation, duration_ms } => {
                format!("Operation '{}' timed out after {}ms", operation, duration_ms)
            }
        }
    }

    /// fr fr Check if error is retryable - can we try again
    pub fn is_retryable(&self) -> bool {
        match self {
            WebError::Network { kind, .. } => matches!(
                NetworkErrorKind::ConnectionTimeout
                    | NetworkErrorKind::NetworkUnreachable
                    | NetworkErrorKind::Other
            WebError::Http { status, .. } => matches!(
                StatusCode::TooManyRequests
                    | StatusCode::InternalServerError
                    | StatusCode::BadGateway
                    | StatusCode::ServiceUnavailable
                    | StatusCode::GatewayTimeout
        }
    }

    /// fr fr Get error category for logging/metrics - error classification
    pub fn category(&self) -> &'static str {
        match self {
        }
    }
// impl fmt::Display for WebError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             WebError::Http { status, message, source } => {
//                 write!(f, "HTTP {} - {}", status.as_u16(), message)?;
//                 if let Some(src) = source {
//                     write!(f, " (source: {})", src)?;
//                 }
//                 Ok(())
//             }
//             WebError::Network { message, kind } => {
//                 write!(f, "Network error ({:?}): {}", kind, message)
//             }
//             WebError::RequestParsing { message, field } => {
//                 write!(f, "Request parsing error: {}", message)?;
//                 if let Some(field) = field {
//                     write!(f, " (field: {})", field)?;
//                 }
//                 Ok(())
//             }
//             WebError::ResponseBuilding { message, component } => {
//                 write!(f, "Response building error in {}: {}", component, message)
//             }
//             WebError::Routing { path, method, message } => {
//                 write!(f, "Routing error for {} {}: {}", method, path, message)
//             }
//             WebError::Middleware { name, message, .. } => {
//                 write!(f, "Middleware '{}' error: {}", name, message)
//             }
//             WebError::Configuration { setting, value, message } => {
//                 write!(f, "Configuration error for {} = '{}': {}", setting, value, message)
//             }
//             WebError::Auth { message, kind } => {
//                 write!(f, "Auth error ({:?}): {}", kind, message)
//             }
//             WebError::Json { message, path } => {
//                 write!(f, "JSON error: {}", message)?;
//                 if let Some(path) = path {
//                     write!(f, " (path: {})", path)?;
//                 }
//                 Ok(())
//             }
//             WebError::Io { message, kind } => {
//                 write!(f, "I/O error ({}): {}", kind, message)
//             }
//             WebError::Timeout { operation, duration_ms } => {
//                 write!(f, "Timeout: operation '{}' took longer than {}ms", operation, duration_ms)
//             }
//             WebError::RateLimit { status_code, message, retry_after } => {
//                 write!(f, "Rate limit error ({}): {}", status_code, message)?;
//                 if let Some(retry_after) = retry_after {
//                     write!(f, " (retry after {}s)", retry_after)?;
//                 }
//                 Ok(())
//             }
//             WebError::Custom { code, message, .. } => {
//                 write!(f, "Custom error [{}]: {}", code, message)
//             }
//         }
//     }
// }

impl StdError for WebError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
        }
    }
/// fr fr CursedError context trait for adding context to errors - error enrichment
pub trait ErrorContext<T> {
    /// fr fr Add context to error - more information
    fn with_context(self, context: impl Into<String>) -> WebResult<T>;
    
    /// fr fr Add context with function - lazy evaluation
    fn with_context_fn<F>(self, f: F) -> WebResult<T>
    where
        F: FnOnce() -> String;
impl<T, E> ErrorContext<T> for Result<T, E>
where
{
    fn with_context(self, context: impl Into<String>) -> WebResult<T> {
        self.map_err(|e| {
            let web_error = e.into();
            WebError::custom(
            )
        })
    fn with_context_fn<F>(self, f: F) -> WebResult<T>
    where
    {
        self.map_err(|e| {
            let web_error = e.into();
            WebError::custom(
            )
        })
    }
}

