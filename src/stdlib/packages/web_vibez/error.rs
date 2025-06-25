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
        status: StatusCode,
        message: String,
        source: Option<String>,
    },
    
    /// fr fr Network connectivity issues
    Network {
        message: String,
        kind: NetworkErrorKind,
    },
    
    /// fr fr Request parsing errors
    RequestParsing {
        message: String,
        field: Option<String>,
    },
    
    /// fr fr Response building errors
    ResponseBuilding {
        message: String,
        component: String,
    },
    
    /// fr fr Route matching errors
    Routing {
        path: String,
        method: String,
        message: String,
    },
    
    /// fr fr Middleware execution errors
    Middleware {
        name: String,
        message: String,
        inner: Option<Box<WebError>>,
    },
    
    /// fr fr Server configuration errors
    Configuration {
        setting: String,
        value: String,
        message: String,
    },
    
    /// fr fr Authentication/authorization errors
    Auth {
        message: String,
        kind: AuthErrorKind,
    },
    
    /// fr fr JSON serialization/deserialization errors
    Json {
        message: String,
        path: Option<String>,
    },
    
    /// fr fr I/O errors from file system or network
    Io {
        message: String,
        kind: String,
    },
    
    /// fr fr Timeout errors
    Timeout {
        operation: String,
        duration_ms: u64,
    },
    
    /// fr fr Rate limiting errors
    RateLimit {
        status_code: u16,
        message: String,
        retry_after: Option<u64>,
    },
    
    /// fr fr Custom application errors
    Custom {
        code: String,
        message: String,
        data: Option<serde_json::Value>,
    },
}

/// fr fr Network error subcategories - specific networking issues
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkErrorKind {
    /// fr fr Connection refused by server
    ConnectionRefused,
    /// fr fr Connection timeout
    ConnectionTimeout,
    /// fr fr DNS resolution failed
    DnsResolution,
    /// fr fr SSL/TLS errors
    SslError,
    /// fr fr Network unreachable
    NetworkUnreachable,
    /// fr fr Other network error
    Other,
}

/// fr fr Authentication error subcategories - auth-specific issues
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthErrorKind {
    /// fr fr Missing credentials
    MissingCredentials,
    /// fr fr Invalid credentials
    InvalidCredentials,
    /// fr fr Expired token/session
    Expired,
    /// fr fr Insufficient permissions
    InsufficientPermissions,
    /// fr fr Rate limit exceeded
    RateLimited,
    /// fr fr Other auth error
    Other,
}

impl WebError {
    /// fr fr Create HTTP error with status code - standard HTTP errors
    pub fn http(status: StatusCode, message: impl Into<String>) -> Self {
        Self::Http {
            status,
            message: message.into(),
            source: None,
        }
    }

    /// fr fr Create HTTP error with source information - detailed context
    pub fn http_with_source(
        status: StatusCode,
        message: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self::Http {
            status,
            message: message.into(),
            source: Some(source.into()),
        }
    }

    /// fr fr Create bad request error - 400 with message
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::http(StatusCode::BadRequest, message)
    }

    /// fr fr Create unauthorized error - 401 with message
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::http(StatusCode::Unauthorized, message)
    }

    /// fr fr Create forbidden error - 403 with message
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::http(StatusCode::Forbidden, message)
    }

    /// fr fr Create not found error - 404 with message
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::http(StatusCode::NotFound, message)
    }

    /// fr fr Create method not allowed error - 405 with message
    pub fn method_not_allowed(message: impl Into<String>) -> Self {
        Self::http(StatusCode::MethodNotAllowed, message)
    }

    /// fr fr Create internal server error - 500 with message
    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self::http(StatusCode::InternalServerError, message)
    }

    /// fr fr Create network error - connection issues
    pub fn network(kind: NetworkErrorKind, message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            kind,
        }
    }

    /// fr fr Create request parsing error - malformed requests
    pub fn request_parsing(message: impl Into<String>, field: Option<String>) -> Self {
        Self::RequestParsing {
            message: message.into(),
            field,
        }
    }

    /// fr fr Create routing error - path/method matching issues
    pub fn routing(
        path: impl Into<String>,
        method: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Routing {
            path: path.into(),
            method: method.into(),
            message: message.into(),
        }
    }

    /// fr fr Create middleware error - middleware chain issues
    pub fn middleware(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Middleware {
            name: name.into(),
            message: message.into(),
            inner: None,
        }
    }

    /// fr fr Create middleware error with inner error - error chain
    pub fn middleware_with_inner(
        name: impl Into<String>,
        message: impl Into<String>,
        inner: WebError,
    ) -> Self {
        Self::Middleware {
            name: name.into(),
            message: message.into(),
            inner: Some(Box::new(inner)),
        }
    }

    /// fr fr Create timeout error - operation took too long
    pub fn timeout(operation: impl Into<String>, duration_ms: u64) -> Self {
        Self::Timeout {
            operation: operation.into(),
            duration_ms,
        }
    }

    /// fr fr Create rate limit error - too many requests
    pub fn rate_limit(
        status_code: u16,
        message: impl Into<String>,
        retry_after: Option<u64>,
    ) -> Self {
        Self::RateLimit {
            status_code,
            message: message.into(),
            retry_after,
        }
    }

    /// fr fr Create custom error - application-specific errors
    pub fn custom(
        code: impl Into<String>,
        message: impl Into<String>,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self::Custom {
            code: code.into(),
            message: message.into(),
            data,
        }
    }

    /// fr fr Get HTTP status code for this error - what to return to client
    pub fn status_code(&self) -> StatusCode {
        match self {
            WebError::Http { status, .. } => *status,
            WebError::Network { .. } => StatusCode::BadGateway,
            WebError::RequestParsing { .. } => StatusCode::BadRequest,
            WebError::ResponseBuilding { .. } => StatusCode::InternalServerError,
            WebError::Routing { .. } => StatusCode::NotFound,
            WebError::Middleware { .. } => StatusCode::InternalServerError,
            WebError::Configuration { .. } => StatusCode::InternalServerError,
            WebError::Auth { kind, .. } => match kind {
                AuthErrorKind::MissingCredentials | AuthErrorKind::InvalidCredentials => {
                    StatusCode::Unauthorized
                }
                AuthErrorKind::Expired => StatusCode::Unauthorized,
                AuthErrorKind::InsufficientPermissions => StatusCode::Forbidden,
                AuthErrorKind::RateLimited => StatusCode::TooManyRequests,
                AuthErrorKind::Other => StatusCode::Unauthorized,
            },
            WebError::Json { .. } => StatusCode::BadRequest,
            WebError::Io { .. } => StatusCode::InternalServerError,
            WebError::Timeout { .. } => StatusCode::RequestTimeout,
            WebError::RateLimit { status_code, .. } => {
                StatusCode::from_u16(*status_code).unwrap_or(StatusCode::TooManyRequests)
            }
            WebError::Custom { .. } => StatusCode::InternalServerError,
        }
    }

    /// fr fr Get error message - human-readable description
    pub fn message(&self) -> String {
        match self {
            WebError::Http { message, .. } => message.clone(),
            WebError::Network { message, .. } => message.clone(),
            WebError::RequestParsing { message, .. } => message.clone(),
            WebError::ResponseBuilding { message, .. } => message.clone(),
            WebError::Routing { message, .. } => message.clone(),
            WebError::Middleware { message, .. } => message.clone(),
            WebError::Configuration { message, .. } => message.clone(),
            WebError::Auth { message, .. } => message.clone(),
            WebError::Json { message, .. } => message.clone(),
            WebError::Io { message, .. } => message.clone(),
            WebError::Timeout { operation, duration_ms } => {
                format!("Operation '{}' timed out after {}ms", operation, duration_ms)
            }
            WebError::RateLimit { message, .. } => message.clone(),
            WebError::Custom { message, .. } => message.clone(),
        }
    }

    /// fr fr Check if error is retryable - can we try again
    pub fn is_retryable(&self) -> bool {
        match self {
            WebError::Network { kind, .. } => matches!(
                kind,
                NetworkErrorKind::ConnectionTimeout
                    | NetworkErrorKind::NetworkUnreachable
                    | NetworkErrorKind::Other
            ),
            WebError::Timeout { .. } => true,
            WebError::RateLimit { .. } => true,
            WebError::Http { status, .. } => matches!(
                status,
                StatusCode::TooManyRequests
                    | StatusCode::InternalServerError
                    | StatusCode::BadGateway
                    | StatusCode::ServiceUnavailable
                    | StatusCode::GatewayTimeout
            ),
            _ => false,
        }
    }

    /// fr fr Get error category for logging/metrics - error classification
    pub fn category(&self) -> &'static str {
        match self {
            WebError::Http { .. } => "http",
            WebError::Network { .. } => "network",
            WebError::RequestParsing { .. } => "request_parsing",
            WebError::ResponseBuilding { .. } => "response_building",
            WebError::Routing { .. } => "routing",
            WebError::Middleware { .. } => "middleware",
            WebError::Configuration { .. } => "configuration",
            WebError::Auth { .. } => "auth",
            WebError::Json { .. } => "json",
            WebError::Io { .. } => "io",
            WebError::Timeout { .. } => "timeout",
            WebError::RateLimit { .. } => "rate_limit",
            WebError::Custom { .. } => "custom",
        }
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
            WebError::Middleware { inner: Some(inner), .. } => Some(inner.as_ref()),
            _ => None,
        }
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
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: Into<WebError>,
{
    fn with_context(self, context: impl Into<String>) -> WebResult<T> {
        self.map_err(|e| {
            let web_error = e.into();
            WebError::custom(
                "context",
                format!("{}: {}", context.into(), web_error.message()),
                None,
            )
        })
    }

    fn with_context_fn<F>(self, f: F) -> WebResult<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let web_error = e.into();
            WebError::custom(
                "context",
                format!("{}: {}", f(), web_error.message()),
                None,
            )
        })
    }
}

