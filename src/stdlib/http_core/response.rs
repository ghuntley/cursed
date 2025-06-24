use crate::web::StatusCode;
// HTTP Response Processing for CURSED web_vibez
//
// Comprehensive response building, formatting, and writing capabilities.

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::stdlib::http_core::{
use crate::error::Error;
    Headers, HeaderMap, ContentType, Cookie, CookieJar, HttpError, HttpResult
};

/// HTTP status codes with their meanings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusCode {
    // 1xx Informational
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,

    // 2xx Success
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    // 3xx Redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // 4xx Client Error
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    UnprocessableEntity = 422,
    TooManyRequests = 429,

    // 5xx Server Error
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
}

impl StatusCode {
    /// Get status code from u16
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            100 => Some(StatusCode::Continue),
            101 => Some(StatusCode::SwitchingProtocols),
            102 => Some(StatusCode::Processing),
            200 => Some(StatusCode::Ok),
            201 => Some(StatusCode::Created),
            202 => Some(StatusCode::Accepted),
            203 => Some(StatusCode::NonAuthoritativeInformation),
            204 => Some(StatusCode::NoContent),
            205 => Some(StatusCode::ResetContent),
            206 => Some(StatusCode::PartialContent),
            300 => Some(StatusCode::MultipleChoices),
            301 => Some(StatusCode::MovedPermanently),
            302 => Some(StatusCode::Found),
            303 => Some(StatusCode::SeeOther),
            304 => Some(StatusCode::NotModified),
            305 => Some(StatusCode::UseProxy),
            307 => Some(StatusCode::TemporaryRedirect),
            308 => Some(StatusCode::PermanentRedirect),
            400 => Some(StatusCode::BadRequest),
            401 => Some(StatusCode::Unauthorized),
            402 => Some(StatusCode::PaymentRequired),
            403 => Some(StatusCode::Forbidden),
            404 => Some(StatusCode::NotFound),
            405 => Some(StatusCode::MethodNotAllowed),
            406 => Some(StatusCode::NotAcceptable),
            407 => Some(StatusCode::ProxyAuthenticationRequired),
            408 => Some(StatusCode::RequestTimeout),
            409 => Some(StatusCode::Conflict),
            410 => Some(StatusCode::Gone),
            411 => Some(StatusCode::LengthRequired),
            412 => Some(StatusCode::PreconditionFailed),
            413 => Some(StatusCode::PayloadTooLarge),
            414 => Some(StatusCode::UriTooLong),
            415 => Some(StatusCode::UnsupportedMediaType),
            416 => Some(StatusCode::RangeNotSatisfiable),
            417 => Some(StatusCode::ExpectationFailed),
            418 => Some(StatusCode::ImATeapot),
            422 => Some(StatusCode::UnprocessableEntity),
            429 => Some(StatusCode::TooManyRequests),
            500 => Some(StatusCode::InternalServerError),
            501 => Some(StatusCode::NotImplemented),
            502 => Some(StatusCode::BadGateway),
            503 => Some(StatusCode::ServiceUnavailable),
            504 => Some(StatusCode::GatewayTimeout),
            505 => Some(StatusCode::HttpVersionNotSupported),
            _ => None,
        }
    }

    /// Get the reason phrase for the status code
    pub fn reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Continue => "Continue",
            StatusCode::SwitchingProtocols => "Switching Protocols",
            StatusCode::Processing => "Processing",
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NonAuthoritativeInformation => "Non-Authoritative Information",
            StatusCode::NoContent => "No Content",
            StatusCode::ResetContent => "Reset Content",
            StatusCode::PartialContent => "Partial Content",
            StatusCode::MultipleChoices => "Multiple Choices",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::SeeOther => "See Other",
            StatusCode::NotModified => "Not Modified",
            StatusCode::UseProxy => "Use Proxy",
            StatusCode::TemporaryRedirect => "Temporary Redirect",
            StatusCode::PermanentRedirect => "Permanent Redirect",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::PaymentRequired => "Payment Required",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotAcceptable => "Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::PreconditionFailed => "Precondition Failed",
            StatusCode::PayloadTooLarge => "Payload Too Large",
            StatusCode::UriTooLong => "URI Too Long",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::RangeNotSatisfiable => "Range Not Satisfiable",
            StatusCode::ExpectationFailed => "Expectation Failed",
            StatusCode::ImATeapot => "I'm a teapot",
            StatusCode::UnprocessableEntity => "Unprocessable Entity",
            StatusCode::TooManyRequests => "Too Many Requests",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
        }
    }

    /// Check if status code indicates success (2xx)
    pub fn is_success(&self) -> bool {
        let code = *self as u16;
        code >= 200 && code < 300
    }

    /// Check if status code indicates redirection (3xx)
    pub fn is_redirection(&self) -> bool {
        let code = *self as u16;
        code >= 300 && code < 400
    }

    /// Check if status code indicates client error (4xx)
    pub fn is_client_error(&self) -> bool {
        let code = *self as u16;
        code >= 400 && code < 500
    }

    /// Check if status code indicates server error (5xx)
    pub fn is_server_error(&self) -> bool {
        let code = *self as u16;
        code >= 500 && code < 600
    }

    /// Check if status code indicates an error (4xx or 5xx)
    pub fn is_error(&self) -> bool {
        self.is_client_error() || self.is_server_error()
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", *self as u16, self.reason_phrase())
    }
}

/// HTTP response body types
pub enum ResponseBody {
    Empty,
    Text(String),
    Binary(Vec<u8>),
    Json(serde_json::Value),
    Html(String),
    Stream(Box<dyn std::io::Read + Send + Sync>),
}

impl std::fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseBody::Empty => write!(f, "Empty"),
            ResponseBody::Text(s) => f.debug_tuple("Text").field(s).finish(),
            ResponseBody::Binary(v) => f.debug_tuple("Binary").field(&format!("{} bytes", v.len())).finish(),
            ResponseBody::Json(j) => f.debug_tuple("Json").field(j).finish(),
            ResponseBody::Html(h) => f.debug_tuple("Html").field(h).finish(),
            ResponseBody::Stream(_) => f.debug_tuple("Stream").field(&"<stream>").finish(),
        }
    }
}

impl Clone for ResponseBody {
    fn clone(&self) -> Self {
        match self {
            ResponseBody::Empty => ResponseBody::Empty,
            ResponseBody::Text(s) => ResponseBody::Text(s.clone()),
            ResponseBody::Binary(v) => ResponseBody::Binary(v.clone()),
            ResponseBody::Json(j) => ResponseBody::Json(j.clone()),
            ResponseBody::Html(h) => ResponseBody::Html(h.clone()),
            ResponseBody::Stream(_) => panic!("Cannot clone stream body"),
        }
    }
}

impl ResponseBody {
    /// Convert body to bytes
    pub fn as_bytes(&self) -> HttpResult<Vec<u8>> {
        match self {
            ResponseBody::Empty => Ok(Vec::new()),
            ResponseBody::Text(s) => Ok(s.as_bytes().to_vec()),
            ResponseBody::Binary(b) => Ok(b.clone()),
            ResponseBody::Json(j) => {
                serde_json::to_vec(j).map_err(|e| HttpError::SerializationError(e.to_string()))
            }
            ResponseBody::Html(h) => Ok(h.as_bytes().to_vec()),
            ResponseBody::Stream(_) => Err(HttpError::StreamNotAvailable),
        }
    }

    /// Get content type for the body
    pub fn content_type(&self) -> &'static str {
        match self {
            ResponseBody::Empty => "text/plain",
            ResponseBody::Text(_) => "text/plain; charset=utf-8",
            ResponseBody::Binary(_) => "application/octet-stream",
            ResponseBody::Json(_) => "application/json; charset=utf-8",
            ResponseBody::Html(_) => "text/html; charset=utf-8",
            ResponseBody::Stream(_) => "application/octet-stream",
        }
    }

    /// Get content length
    pub fn content_length(&self) -> usize {
        match self {
            ResponseBody::Empty => 0,
            ResponseBody::Text(s) => s.len(),
            ResponseBody::Binary(b) => b.len(),
            ResponseBody::Json(j) => {
                serde_json::to_string(j).map(|s| s.len()).unwrap_or(0)
            }
            ResponseBody::Html(h) => h.len(),
            ResponseBody::Stream(_) => 0, // Unknown for streams
        }
    }
}

/// Comprehensive HTTP Response structure
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP version
    pub version: String,
    /// Response status code
    pub status: StatusCode,
    /// Response headers
    pub headers: HeaderMap,
    /// Response body
    pub body: ResponseBody,
    /// Cookies to set
    pub cookies: Vec<Cookie>,
    /// Response timestamp
    pub timestamp: SystemTime,
}

impl Response {
    /// Create a new response
    pub fn new(status: StatusCode) -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status,
            headers: HeaderMap::new(),
            body: ResponseBody::Empty,
            cookies: Vec::new(),
            timestamp: SystemTime::now(),
        }
    }

    /// Create a successful response
    pub fn ok() -> Self {
        Self::new(StatusCode::Ok)
    }

    /// Create a not found response
    pub fn not_found() -> Self {
        Self::new(StatusCode::NotFound)
    }

    /// Create an internal server error response
    pub fn internal_error() -> Self {
        Self::new(StatusCode::InternalServerError)
    }

    /// Create a bad request response
    pub fn bad_request() -> Self {
        Self::new(StatusCode::BadRequest)
    }

    /// Create an unauthorized response
    pub fn unauthorized() -> Self {
        Self::new(StatusCode::Unauthorized)
    }

    /// Create a forbidden response
    pub fn forbidden() -> Self {
        Self::new(StatusCode::Forbidden)
    }

    /// Set response body
    pub fn body(mut self, body: ResponseBody) -> Self {
        self.body = body;
        self.set_content_type_if_not_set();
        self.set_content_length();
        self
    }

    /// Set text body
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.body = ResponseBody::Text(text.into());
        self.set_content_type_if_not_set();
        self.set_content_length();
        self
    }

    /// Set JSON body
    pub fn json<T: serde::Serialize>(mut self, data: &T) -> HttpResult<Self> {
        let json = serde_json::to_value(data)
            .map_err(|e| HttpError::SerializationError(e.to_string()))?;
        self.body = ResponseBody::Json(json);
        self.set_content_type_if_not_set();
        self.set_content_length();
        Ok(self)
    }

    /// Set HTML body
    pub fn html<S: Into<String>>(mut self, html: S) -> Self {
        self.body = ResponseBody::Html(html.into());
        self.set_content_type_if_not_set();
        self.set_content_length();
        self
    }

    /// Set binary body
    pub fn binary(mut self, data: Vec<u8>) -> Self {
        self.body = ResponseBody::Binary(data);
        self.set_content_type_if_not_set();
        self.set_content_length();
        self
    }

    /// Add header
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Add multiple headers
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        for (key, value) in headers {
            self.headers.insert(key, value);
        }
        self
    }

    /// Add cookie
    pub fn cookie(mut self, cookie: Cookie) -> Self {
        self.cookies.push(cookie);
        self
    }

    /// Set cache control
    pub fn cache_control<S: Into<String>>(self, value: S) -> Self {
        self.header("Cache-Control", value.into())
    }

    /// Set no cache headers
    pub fn no_cache(self) -> Self {
        self.header("Cache-Control", "no-cache, no-store, must-revalidate")
            .header("Pragma", "no-cache")
            .header("Expires", "0")
    }

    /// Set CORS headers
    pub fn cors(self, origin: &str) -> Self {
        self.header("Access-Control-Allow-Origin", origin)
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
            .header("Access-Control-Allow-Credentials", "true")
    }

    /// Set location header for redirects
    pub fn location<S: Into<String>>(self, url: S) -> Self {
        self.header("Location", url.into())
    }

    /// Create redirect response
    pub fn redirect<S: Into<String>>(url: S, permanent: bool) -> Self {
        let status = if permanent {
            StatusCode::MovedPermanently
        } else {
            StatusCode::Found
        };
        Self::new(status).location(url)
    }

    /// Set content type if not already set
    fn set_content_type_if_not_set(&mut self) {
        if !self.headers.contains_key("Content-Type") {
            let content_type = self.body.content_type();
            self.headers.insert("Content-Type".to_string(), content_type.to_string());
        }
    }

    /// Set content length based on body
    fn set_content_length(&mut self) {
        let length = self.body.content_length();
        if length > 0 {
            self.headers.insert("Content-Length".to_string(), length.to_string());
        }
    }

    /// Format response as HTTP string
    pub fn to_http_string(&self) -> HttpResult<String> {
        let mut response = String::new();

        // Status line
        writeln!(response, "{} {}", self.version, self.status)
            .map_err(|e| HttpError::FormatError(e.to_string()))?;

        // Headers
        for (key, value) in &self.headers {
            writeln!(response, "{}: {}", key, value)
                .map_err(|e| HttpError::FormatError(e.to_string()))?;
        }

        // Cookies
        for cookie in &self.cookies {
            writeln!(response, "Set-Cookie: {}", cookie.to_string())
                .map_err(|e| HttpError::FormatError(e.to_string()))?;
        }

        // Date header if not present
        if !self.headers.contains_key("Date") {
            let date = httpdate::fmt_http_date(self.timestamp);
            writeln!(response, "Date: {}", date)
                .map_err(|e| HttpError::FormatError(e.to_string()))?;
        }

        // Server header if not present
        if !self.headers.contains_key("Server") {
            writeln!(response, "Server: CURSED/1.0")
                .map_err(|e| HttpError::FormatError(e.to_string()))?;
        }

        // Empty line before body
        writeln!(response)
            .map_err(|e| HttpError::FormatError(e.to_string()))?;

        // Body
        match &self.body {
            ResponseBody::Empty => {}
            ResponseBody::Text(text) => {
                response.push_str(text);
            }
            ResponseBody::Html(html) => {
                response.push_str(html);
            }
            ResponseBody::Json(json) => {
                let json_str = serde_json::to_string(json)
                    .map_err(|e| HttpError::SerializationError(e.to_string()))?;
                response.push_str(&json_str);
            }
            ResponseBody::Binary(data) => {
                // For string representation, we'll use a placeholder
                response.push_str(&format!("[Binary data: {} bytes]", data.len()));
            }
            ResponseBody::Stream(_) => {
                response.push_str("[Stream data]");
            }
        }

        Ok(response)
    }

    /// Write response to a writer
    pub fn write_to<W: Write>(&self, writer: &mut W) -> HttpResult<()> {
        // Status line
        writeln!(writer, "{} {}", self.version, self.status)
            .map_err(|e| HttpError::IoError(e.to_string()))?;

        // Headers
        for (key, value) in &self.headers {
            writeln!(writer, "{}: {}", key, value)
                .map_err(|e| HttpError::IoError(e.to_string()))?;
        }

        // Cookies
        for cookie in &self.cookies {
            writeln!(writer, "Set-Cookie: {}", cookie.to_string())
                .map_err(|e| HttpError::IoError(e.to_string()))?;
        }

        // Date header if not present
        if !self.headers.contains_key("Date") {
            let date = httpdate::fmt_http_date(self.timestamp);
            writeln!(writer, "Date: {}", date)
                .map_err(|e| HttpError::IoError(e.to_string()))?;
        }

        // Server header if not present
        if !self.headers.contains_key("Server") {
            writeln!(writer, "Server: CURSED/1.0")
                .map_err(|e| HttpError::IoError(e.to_string()))?;
        }

        // Empty line before body
        writeln!(writer)
            .map_err(|e| HttpError::IoError(e.to_string()))?;

        // Body
        let body_bytes = self.body.as_bytes()?;
        writer.write_all(&body_bytes)
            .map_err(|e| HttpError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Get a header value
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }

    /// Check if response has body
    pub fn has_body(&self) -> bool {
        !matches!(self.body, ResponseBody::Empty)
    }

    /// Get content length
    pub fn content_length(&self) -> usize {
        if let Some(length_str) = self.get_header("Content-Length") {
            length_str.parse().unwrap_or_else(|_| self.body.content_length())
        } else {
            self.body.content_length()
        }
    }

    /// Get content type
    pub fn content_type(&self) -> String {
        self.get_header("Content-Type")
            .cloned()
            .unwrap_or_else(|| self.body.content_type().to_string())
    }
}

/// Response builder for fluent response construction
#[derive(Debug)]
pub struct ResponseBuilder {
    response: Response,
}

impl ResponseBuilder {
    /// Create new response builder
    pub fn new(status: StatusCode) -> Self {
        Self {
            response: Response::new(status),
        }
    }

    /// Create OK response builder
    pub fn ok() -> Self {
        Self::new(StatusCode::Ok)
    }

    /// Create error response builder
    pub fn error(status: StatusCode) -> Self {
        Self::new(status)
    }

    /// Set status code
    pub fn status(mut self, status: StatusCode) -> Self {
        self.response.status = status;
        self
    }

    /// Add header
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.response.headers.insert(key.into(), value.into());
        self
    }

    /// Set body
    pub fn body(mut self, body: ResponseBody) -> Self {
        self.response.body = body;
        self.response.set_content_type_if_not_set();
        self.response.set_content_length();
        self
    }

    /// Set text body
    pub fn text<S: Into<String>>(self, text: S) -> Self {
        self.body(ResponseBody::Text(text.into()))
    }

    /// Set JSON body
    pub fn json<T: serde::Serialize>(mut self, data: &T) -> HttpResult<Self> {
        let json = serde_json::to_value(data)
            .map_err(|e| HttpError::SerializationError(e.to_string()))?;
        self.response.body = ResponseBody::Json(json);
        self.response.set_content_type_if_not_set();
        self.response.set_content_length();
        Ok(self)
    }

    /// Set HTML body
    pub fn html<S: Into<String>>(self, html: S) -> Self {
        self.body(ResponseBody::Html(html.into()))
    }

    /// Add cookie
    pub fn cookie(mut self, cookie: Cookie) -> Self {
        self.response.cookies.push(cookie);
        self
    }

    /// Build the response
    pub fn build(self) -> Response {
        self.response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_properties() {
        assert!(StatusCode::Ok.is_success());
        assert!(!StatusCode::Ok.is_error());
        
        assert!(StatusCode::NotFound.is_client_error());
        assert!(StatusCode::NotFound.is_error());
        
        assert!(StatusCode::InternalServerError.is_server_error());
        assert!(StatusCode::InternalServerError.is_error());
        
        assert!(StatusCode::MovedPermanently.is_redirection());
    }

    #[test]
    fn test_response_creation() {
        let response = Response::ok()
            .text("Hello, World!")
            .header("X-Custom", "value");

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.get_header("X-Custom"), Some(&"value".to_string()));
        assert!(response.has_body());
    }

    #[test]
    fn test_response_builder() {
        let response = ResponseBuilder::ok()
            .header("Content-Type", "application/json")
            .text("{\"message\": \"success\"}")
            .build();

        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(response.get_header("Content-Type"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_redirect_response() {
        let response = Response::redirect("https://example.com", true);
        
        assert_eq!(response.status, StatusCode::MovedPermanently);
        assert_eq!(response.get_header("Location"), Some(&"https://example.com".to_string()));
    }

    #[test]
    fn test_response_formatting() {
        let response = Response::ok()
            .text("Hello")
            .header("Content-Type", "text/plain");

        let http_string = response.to_http_string().unwrap();
        assert!(http_string.contains("HTTP/1.1 200 OK"));
        assert!(http_string.contains("Content-Type: text/plain"));
        assert!(http_string.contains("Hello"));
    }
}
