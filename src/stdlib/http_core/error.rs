// CursedError Handling for CURSED web_vibez
//
// Comprehensive error types and handling for HTTP processing.

use std::fmt;
use crate::error::CursedError;

/// HTTP-specific error types
#[derive(Debug, Clone)]
pub enum HttpError {
    /// Invalid HTTP request format
    InvalidRequest(String),
    /// Invalid HTTP method
    InvalidMethod(String),
    /// Invalid URL format
    InvalidUrl(String),
    /// Invalid header format or value
    InvalidHeader(String),
    /// Invalid cookie format or value
    InvalidCookie(String),
    /// Invalid content type
    InvalidContentType(String),
    /// Form data parsing error
    FormDataError(String),
    /// JSON serialization/deserialization error
    SerializationError(String),
    /// Encoding/decoding error
    EncodingError(String),
    /// Wrong body type for operation
    WrongBodyType(String),
    /// Stream not available
    StreamNotAvailable,
    /// Unsupported HTTP version
    UnsupportedVersion(String),
    /// I/O error
    IoError(String),
    /// Format error
    FormatError(String),
    /// Timeout error
    Timeout(String),
    /// Too large request
    RequestTooLarge(String),
    /// Missing required field
    MissingField(String),
    /// Validation error
    ValidationError(String),
    /// Authentication error
    AuthenticationError(String),
    /// Authorization error
    AuthorizationError(String),
    /// Rate limiting error
    RateLimitError(String),
    /// Custom application error
    Custom(String),
}

impl HttpError {
    /// Create a custom error
    pub fn custom<S: Into<String>>(message: S) -> Self {
        HttpError::Custom(message.into())
    }

    /// Create an invalid request error
    pub fn invalid_request<S: Into<String>>(message: S) -> Self {
        HttpError::InvalidRequest(message.into())
    }

    /// Create a validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        HttpError::ValidationError(message.into())
    }

    /// Create an authentication error
    pub fn authentication<S: Into<String>>(message: S) -> Self {
        HttpError::AuthenticationError(message.into())
    }

    /// Create an authorization error
    pub fn authorization<S: Into<String>>(message: S) -> Self {
        HttpError::AuthorizationError(message.into())
    }

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
            HttpError::ValidationError(_) => ErrorCategory::ClientError,

            HttpError::SerializationError(_) |
            HttpError::EncodingError(_) |
            HttpError::StreamNotAvailable |
            HttpError::UnsupportedVersion(_) |
            HttpError::IoError(_) |
            HttpError::FormatError(_) |
            HttpError::Custom(_) => ErrorCategory::ServerError,

            HttpError::Timeout(_) |
            HttpError::RequestTooLarge(_) => ErrorCategory::RequestError,

            HttpError::AuthenticationError(_) => ErrorCategory::AuthenticationError,
            HttpError::AuthorizationError(_) => ErrorCategory::AuthorizationError,
            HttpError::RateLimitError(_) => ErrorCategory::RateLimitError,
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
    }

    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        matches!(self.category(), ErrorCategory::ServerError)
    }

    /// Get error code for programmatic handling
    pub fn error_code(&self) -> &'static str {
        match self {
            HttpError::InvalidRequest(_) => "INVALID_REQUEST",
            HttpError::InvalidMethod(_) => "INVALID_METHOD",
            HttpError::InvalidUrl(_) => "INVALID_URL",
            HttpError::InvalidHeader(_) => "INVALID_HEADER",
            HttpError::InvalidCookie(_) => "INVALID_COOKIE",
            HttpError::InvalidContentType(_) => "INVALID_CONTENT_TYPE",
            HttpError::FormDataError(_) => "FORM_DATA_ERROR",
            HttpError::SerializationError(_) => "SERIALIZATION_ERROR",
            HttpError::EncodingError(_) => "ENCODING_ERROR",
            HttpError::WrongBodyType(_) => "WRONG_BODY_TYPE",
            HttpError::StreamNotAvailable => "STREAM_NOT_AVAILABLE",
            HttpError::UnsupportedVersion(_) => "UNSUPPORTED_VERSION",
            HttpError::IoError(_) => "IO_ERROR",
            HttpError::FormatError(_) => "FORMAT_ERROR",
            HttpError::Timeout(_) => "TIMEOUT",
            HttpError::RequestTooLarge(_) => "REQUEST_TOO_LARGE",
            HttpError::MissingField(_) => "MISSING_FIELD",
            HttpError::ValidationError(_) => "VALIDATION_ERROR",
            HttpError::AuthenticationError(_) => "AUTHENTICATION_ERROR",
            HttpError::AuthorizationError(_) => "AUTHORIZATION_ERROR",
            HttpError::RateLimitError(_) => "RATE_LIMIT_ERROR",
            HttpError::Custom(_) => "CUSTOM_ERROR",
        }
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            HttpError::InvalidRequest(msg) => format!("Invalid request: {}", msg),
            HttpError::InvalidMethod(method) => format!("Unsupported HTTP method: {}", method),
            HttpError::InvalidUrl(msg) => format!("Invalid URL: {}", msg),
            HttpError::InvalidHeader(msg) => format!("Invalid header: {}", msg),
            HttpError::InvalidCookie(msg) => format!("Invalid cookie: {}", msg),
            HttpError::InvalidContentType(msg) => format!("Invalid content type: {}", msg),
            HttpError::FormDataError(msg) => format!("Form data error: {}", msg),
            HttpError::SerializationError(msg) => format!("Data format error: {}", msg),
            HttpError::EncodingError(msg) => format!("Encoding error: {}", msg),
            HttpError::WrongBodyType(msg) => format!("Incorrect data format: {}", msg),
            HttpError::StreamNotAvailable => "Data stream not available".to_string(),
            HttpError::UnsupportedVersion(version) => format!("Unsupported HTTP version: {}", version),
            HttpError::IoError(_) => "Input/output error occurred".to_string(),
            HttpError::FormatError(_) => "Data formatting error".to_string(),
            HttpError::Timeout(msg) => format!("Request timeout: {}", msg),
            HttpError::RequestTooLarge(msg) => format!("Request too large: {}", msg),
            HttpError::MissingField(field) => format!("Missing required field: {}", field),
            HttpError::ValidationError(msg) => format!("Validation failed: {}", msg),
            HttpError::AuthenticationError(msg) => format!("Authentication required: {}", msg),
            HttpError::AuthorizationError(msg) => format!("Access denied: {}", msg),
            HttpError::RateLimitError(msg) => format!("Rate limit exceeded: {}", msg),
            HttpError::Custom(msg) => msg.clone(),
        }
    }

    /// Convert to JSON object for API responses
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "error": {
                "code": self.error_code(),
                "message": self.user_message(),
                "status": self.status_code(),
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
    ClientError,
    /// Server-side errors (processing failures, bugs)
    ServerError,
    /// Request-related errors (timeout, too large)
    RequestError,
    /// Authentication errors
    AuthenticationError,
    /// Authorization errors
    AuthorizationError,
    /// Rate limiting errors
    RateLimitError,
}

impl ErrorCategory {
    /// Get HTTP status code range for this category
    pub fn status_range(&self) -> (u16, u16) {
        match self {
            ErrorCategory::ClientError => (400, 499),
            ErrorCategory::ServerError => (500, 599),
            ErrorCategory::RequestError => (400, 499),
            ErrorCategory::AuthenticationError => (401, 401),
            ErrorCategory::AuthorizationError => (403, 403),
            ErrorCategory::RateLimitError => (429, 429),
        }
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
    pub error: HttpError,
    /// Additional context information
    pub context: std::collections::HashMap<String, String>,
    /// Stack trace information
    pub trace: Vec<String>,
}

impl ErrorContext {
    /// Create new error context
    pub fn new(error: HttpError) -> Self {
        Self {
            error,
            context: std::collections::HashMap::new(),
            trace: Vec::new(),
        }
    }

    /// Add context information
    pub fn with_context<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Add trace information
    pub fn with_trace<T>(mut self, trace: T) -> Self
    where
        T: Into<String>,
    {
        self.trace.push(trace.into());
        self
    }

    /// Get context value
    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    }

    /// Get all context
    pub fn context(&self) -> &std::collections::HashMap<String, String> {
        &self.context
    }

    /// Get trace
    pub fn trace(&self) -> &Vec<String> {
        &self.trace
    }

    /// Convert to detailed JSON
    pub fn to_detailed_json(&self) -> serde_json::Value {
        let mut json = self.error.to_json();
        
        if !self.context.is_empty() {
            json["error"]["context"] = serde_json::to_value(&self.context).unwrap();
        }
        
        if !self.trace.is_empty() {
            json["error"]["trace"] = serde_json::to_value(&self.trace).unwrap();
        }
        
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
        K: Into<String>,
        V: Into<String>;

    /// Add trace to error
    fn with_trace<Tr>(self, trace: Tr) -> Result<T, ErrorContext>
    where
        Tr: Into<String>;
}

impl<T> ErrorContextExt<T> for HttpResult<T> {
    fn with_context<K, V>(self, key: K, value: V) -> Result<T, ErrorContext>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.map_err(|err| ErrorContext::new(err).with_context(key, value))
    }

    fn with_trace<Tr>(self, trace: Tr) -> Result<T, ErrorContext>
    where
        Tr: Into<String>,
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
    }

    /// Unauthorized (401)
    pub fn unauthorized<S: Into<String>>(message: S) -> HttpError {
        HttpError::authentication(message)
    }

    /// Forbidden (403)
    pub fn forbidden<S: Into<String>>(message: S) -> HttpError {
        HttpError::authorization(message)
    }

    /// Not Found (404)
    pub fn not_found<S: Into<String>>(message: S) -> HttpError {
        HttpError::custom(format!("Not found: {}", message.into()))
    }

    /// Method Not Allowed (405)
    pub fn method_not_allowed<S: Into<String>>(method: S) -> HttpError {
        HttpError::InvalidMethod(method.into())
    }

    /// Request Timeout (408)
    pub fn request_timeout<S: Into<String>>(message: S) -> HttpError {
        HttpError::Timeout(message.into())
    }

    /// Payload Too Large (413)
    pub fn payload_too_large<S: Into<String>>(message: S) -> HttpError {
        HttpError::RequestTooLarge(message.into())
    }

    /// Unprocessable Entity (422)
    pub fn unprocessable_entity<S: Into<String>>(message: S) -> HttpError {
        HttpError::validation(message)
    }

    /// Too Many Requests (429)
    pub fn too_many_requests<S: Into<String>>(message: S) -> HttpError {
        HttpError::RateLimitError(message.into())
    }

    /// Internal Server CursedError (500)
    pub fn internal_server_error<S: Into<String>>(message: S) -> HttpError {
        HttpError::custom(message)
    }
}

