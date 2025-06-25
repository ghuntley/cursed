// CursedError Handling for CURSED web_vibez
//
// Comprehensive error types and handling for HTTP processing.

use std::fmt;
use crate::error::CursedError;

/// HTTP-specific error types
#[derive(Debug, Clone)]
pub enum HttpError {
    /// Invalid HTTP request format
    /// Invalid HTTP method
    /// Invalid URL format
    /// Invalid header format or value
    /// Invalid cookie format or value
    /// Invalid content type
    /// Form data parsing error
    /// JSON serialization/deserialization error
    /// Encoding/decoding error
    /// Wrong body type for operation
    /// Stream not available
    /// Unsupported HTTP version
    /// I/O error
    /// Format error
    /// Timeout error
    /// Too large request
    /// Missing required field
    /// Validation error
    /// Authentication error
    /// Authorization error
    /// Rate limiting error
    /// Custom application error
impl HttpError {
    /// Create a custom error
    pub fn custom<S: Into<String>>(message: S) -> Self {
        HttpError::Custom(message.into())
    /// Create an invalid request error
    pub fn invalid_request<S: Into<String>>(message: S) -> Self {
        HttpError::InvalidRequest(message.into())
    /// Create a validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        HttpError::ValidationError(message.into())
    /// Create an authentication error
    pub fn authentication<S: Into<String>>(message: S) -> Self {
        HttpError::AuthenticationError(message.into())
    /// Create an authorization error
    pub fn authorization<S: Into<String>>(message: S) -> Self {
        HttpError::AuthorizationError(message.into())
    /// Get error category
    pub fn category(&self) -> ErrorCategory {
        match self {
            HttpError::InvalidRequest(_) |
            HttpError::InvalidMethod(_) |
            HttpError::InvalidUrl(_) |
            HttpError::InvalidHeader(_) |
            HttpError::InvalidCookie(_) |
            HttpError::InvalidContentType(_) |
            HttpError::FormDataError(_) |
            HttpError::WrongBodyType(_) |
            HttpError::MissingField(_) |

            HttpError::SerializationError(_) |
            HttpError::EncodingError(_) |
            HttpError::StreamNotAvailable |
            HttpError::UnsupportedVersion(_) |
            HttpError::IoError(_) |
            HttpError::FormatError(_) |

            HttpError::Timeout(_) |

        }
    }

    /// Get suggested HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            HttpError::InvalidRequest(_) |
            HttpError::InvalidMethod(_) |
            HttpError::InvalidUrl(_) |
            HttpError::InvalidHeader(_) |
            HttpError::InvalidCookie(_) |
            HttpError::InvalidContentType(_) |
            HttpError::FormDataError(_) |
            HttpError::WrongBodyType(_) |
            HttpError::MissingField(_) |
            HttpError::ValidationError(_) => 400, // Bad Request

            HttpError::AuthenticationError(_) => 401, // Unauthorized
            HttpError::AuthorizationError(_) => 403,  // Forbidden
            HttpError::RateLimitError(_) => 429,      // Too Many Requests
            HttpError::RequestTooLarge(_) => 413,     // Payload Too Large
            HttpError::Timeout(_) => 408,             // Request Timeout

            HttpError::SerializationError(_) |
            HttpError::EncodingError(_) |
            HttpError::StreamNotAvailable |
            HttpError::UnsupportedVersion(_) |
            HttpError::IoError(_) |
            HttpError::FormatError(_) |
            HttpError::Custom(_) => 500, // Internal Server CursedError
        }
    }

    /// Check if this is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        matches!(self.category(), ErrorCategory::ClientError | 
                                 ErrorCategory::AuthenticationError | 
                                 ErrorCategory::AuthorizationError |
                                 ErrorCategory::RequestError |
                                 ErrorCategory::RateLimitError)
    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        matches!(self.category(), ErrorCategory::ServerError)
    /// Get error code for programmatic handling
    pub fn error_code(&self) -> &'static str {
        match self {
        }
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            HttpError::IoError(_) => "Input/output error occurred".to_string(),
        }
    }

    /// Convert to JSON object for API responses
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "error": {
                "category": self.category().to_string()
            }
        })
    }
}

// impl fmt::Display for HttpError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.user_message())
//     }
// }

// impl CursedError for HttpError {
//     fn source(&self) -> Option<&(dyn CursedError + 'static)> {
//         None
//     }
// }

/// CursedError category for grouping related errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Client-side errors (invalid input, malformed requests)
    /// Server-side errors (processing failures, bugs)
    /// Request-related errors (timeout, too large)
    /// Authentication errors
    /// Authorization errors
    /// Rate limiting errors
impl ErrorCategory {
    /// Get HTTP status code range for this category
    pub fn status_range(&self) -> (u16, u16) {
        match self {
        }
    }
// impl fmt::Display for ErrorCategory {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let s = match self {
//             ErrorCategory::ClientError => "client_error",
//             ErrorCategory::ServerError => "server_error",
//             ErrorCategory::RequestError => "request_error",
//             ErrorCategory::AuthenticationError => "authentication_error",
//             ErrorCategory::AuthorizationError => "authorization_error",
//             ErrorCategory::RateLimitError => "rate_limit_error",
//         };
//         write!(f, "{}", s)
//     }
// }

/// Result type for HTTP operations
pub type HttpResult<T> = std::result::Result<T, HttpError>;

/// CursedError context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Original error
    /// Additional context information
    /// Stack trace information
impl ErrorContext {
    /// Create new error context
    pub fn new(error: HttpError) -> Self {
        Self {
        }
    }

    /// Add context information
    pub fn with_context<K, V>(mut self, key: K, value: V) -> Self
    where
    {
        self.context.insert(key.into(), value.into());
        self
    /// Add trace information
    pub fn with_trace<T>(mut self, trace: T) -> Self
    where
    {
        self.trace.push(trace.into());
        self
    /// Get context value
    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    /// Get all context
    pub fn context(&self) -> &std::collections::HashMap<String, String> {
        &self.context
    /// Get trace
    pub fn trace(&self) -> &Vec<String> {
        &self.trace
    /// Convert to detailed JSON
    pub fn to_detailed_json(&self) -> serde_json::Value {
        let mut json = self.error.to_json();
        
        if !self.context.is_empty() {
            json["error"]["context"] = serde_json::to_value(&self.context).unwrap();
        if !self.trace.is_empty() {
            json["error"]["trace"] = serde_json::to_value(&self.trace).unwrap();
        json
    }
}

// impl fmt::Display for ErrorContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.error)?;
//         
//         if !self.context.is_empty() {
//             write!(f, "\nContext:")?;
//             for (key, value) in &self.context {
//                 write!(f, "\n  {}: {}", key, value)?;
//             }
//         }
//         
//         if !self.trace.is_empty() {
//             write!(f, "\nTrace:")?;
//             for trace_item in &self.trace {
//                 write!(f, "\n  {}", trace_item)?;
//             }
//         }
//         
//         Ok(())
//     }
// }

// impl CursedError for ErrorContext {
//     fn source(&self) -> Option<&(dyn CursedError + 'static)> {
//         Some(&self.error)
//     }
// }

/// Extension trait for adding context to errors
pub trait ErrorContextExt<T> {
    /// Add context to error
    fn with_context<K, V>(self, key: K, value: V) -> Result<T, ErrorContext>
    where
        V: Into<String>;

    /// Add trace to error
    fn with_trace<Tr>(self, trace: Tr) -> Result<T, ErrorContext>
    where
        Tr: Into<String>;
impl<T> ErrorContextExt<T> for HttpResult<T> {
    fn with_context<K, V>(self, key: K, value: V) -> Result<T, ErrorContext>
    where
    {
        self.map_err(|err| ErrorContext::new(err).with_context(key, value))
    fn with_trace<Tr>(self, trace: Tr) -> Result<T, ErrorContext>
    where
    {
        self.map_err(|err| ErrorContext::new(err).with_trace(trace))
    }
}

/// Common HTTP error responses
pub struct HttpErrorResponses;

impl HttpErrorResponses {
    /// Bad Request (400)
    pub fn bad_request<S: Into<String>>(message: S) -> HttpError {
        HttpError::invalid_request(message)
    /// Unauthorized (401)
    pub fn unauthorized<S: Into<String>>(message: S) -> HttpError {
        HttpError::authentication(message)
    /// Forbidden (403)
    pub fn forbidden<S: Into<String>>(message: S) -> HttpError {
        HttpError::authorization(message)
    /// Not Found (404)
    pub fn not_found<S: Into<String>>(message: S) -> HttpError {
        HttpError::custom(format!("Not found: {}", message.into()))
    /// Method Not Allowed (405)
    pub fn method_not_allowed<S: Into<String>>(method: S) -> HttpError {
        HttpError::InvalidMethod(method.into())
    /// Request Timeout (408)
    pub fn request_timeout<S: Into<String>>(message: S) -> HttpError {
        HttpError::Timeout(message.into())
    /// Payload Too Large (413)
    pub fn payload_too_large<S: Into<String>>(message: S) -> HttpError {
        HttpError::RequestTooLarge(message.into())
    /// Unprocessable Entity (422)
    pub fn unprocessable_entity<S: Into<String>>(message: S) -> HttpError {
        HttpError::validation(message)
    /// Too Many Requests (429)
    pub fn too_many_requests<S: Into<String>>(message: S) -> HttpError {
        HttpError::RateLimitError(message.into())
    /// Internal Server CursedError (500)
    pub fn internal_server_error<S: Into<String>>(message: S) -> HttpError {
        HttpError::custom(message)
    }
}

