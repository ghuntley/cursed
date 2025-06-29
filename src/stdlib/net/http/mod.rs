use crate::web::StatusCode;
use crate::error::CursedError;
/// HTTP client library for the CURSED networking module
/// 
/// This module provides a comprehensive HTTP client implementation supporting
/// HTTP/1.1 and HTTP/2, with features including authentication, cookie management,
/// connection pooling, and comprehensive error handling.

pub mod client;
pub mod request;
pub mod response;
pub mod headers;
pub mod auth;
pub mod cookies;
pub mod pool;
pub mod config;

// Re-export main types for easy access
pub use client::{HttpClient, HttpClientBuilder};
pub use request::{HttpRequest, RequestBuilder, HttpMethod};
pub use response::{HttpResponse, StatusCode, ResponseBody};
pub use headers::{HttpHeaders, HeaderMap, HeaderValue};
pub use auth::{HttpAuth, BasicAuth, BearerAuth, OAuth2Auth};
pub use cookies::{Cookie, CookieJar, CookieStore};
pub use pool::{ConnectionPool, PoolConfig, PoolStats};
pub use config::{HttpConfig, TimeoutConfig, RetryConfig, CompressionConfig};

/// HTTP version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http2,
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http10 => write!(f, "HTTP/1.0"),
            HttpVersion::Http11 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2 => write!(f, "HTTP/2"),
        }
    }
}

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::HEAD => write!(f, "HEAD"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::PATCH => write!(f, "PATCH"),
            Method::TRACE => write!(f, "TRACE"),
            Method::CONNECT => write!(f, "CONNECT"),
        }
    }
}

impl std::str::FromStr for Method {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "PATCH" => Ok(Method::PATCH),
            "TRACE" => Ok(Method::TRACE),
            "CONNECT" => Ok(Method::CONNECT),
            _ => Err(format!("Unknown HTTP method: {}", s)),
        }
    }
}
/// HTTP status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Status(pub u16);

impl Status {
    // 1xx Informational
    pub const CONTINUE: Status = Status(100);
    pub const SWITCHING_PROTOCOLS: Status = Status(101);
    pub const PROCESSING: Status = Status(102);
    
    // 2xx Success
    pub const OK: Status = Status(200);
    pub const CREATED: Status = Status(201);
    pub const ACCEPTED: Status = Status(202);
    pub const NON_AUTHORITATIVE_INFORMATION: Status = Status(203);
    pub const NO_CONTENT: Status = Status(204);
    pub const RESET_CONTENT: Status = Status(205);
    pub const PARTIAL_CONTENT: Status = Status(206);
    
    // 3xx Redirection
    pub const MULTIPLE_CHOICES: Status = Status(300);
    pub const MOVED_PERMANENTLY: Status = Status(301);
    pub const FOUND: Status = Status(302);
    pub const SEE_OTHER: Status = Status(303);
    pub const NOT_MODIFIED: Status = Status(304);
    pub const USE_PROXY: Status = Status(305);
    pub const TEMPORARY_REDIRECT: Status = Status(307);
    pub const PERMANENT_REDIRECT: Status = Status(308);
    
    // 4xx Client CursedError
    pub const BAD_REQUEST: Status = Status(400);
    pub const UNAUTHORIZED: Status = Status(401);
    pub const PAYMENT_REQUIRED: Status = Status(402);
    pub const FORBIDDEN: Status = Status(403);
    pub const NOT_FOUND: Status = Status(404);
    pub const METHOD_NOT_ALLOWED: Status = Status(405);
    pub const NOT_ACCEPTABLE: Status = Status(406);
    pub const PROXY_AUTHENTICATION_REQUIRED: Status = Status(407);
    pub const REQUEST_TIMEOUT: Status = Status(408);
    pub const CONFLICT: Status = Status(409);
    pub const GONE: Status = Status(410);
    pub const LENGTH_REQUIRED: Status = Status(411);
    pub const PRECONDITION_FAILED: Status = Status(412);
    pub const PAYLOAD_TOO_LARGE: Status = Status(413);
    pub const URI_TOO_LONG: Status = Status(414);
    pub const UNSUPPORTED_MEDIA_TYPE: Status = Status(415);
    pub const RANGE_NOT_SATISFIABLE: Status = Status(416);
    pub const EXPECTATION_FAILED: Status = Status(417);
    pub const IM_A_TEAPOT: Status = Status(418);
    pub const UNPROCESSABLE_ENTITY: Status = Status(422);
    pub const TOO_MANY_REQUESTS: Status = Status(429);
    
    // 5xx Server CursedError
    pub const INTERNAL_SERVER_ERROR: Status = Status(500);
    pub const NOT_IMPLEMENTED: Status = Status(501);
    pub const BAD_GATEWAY: Status = Status(502);
    pub const SERVICE_UNAVAILABLE: Status = Status(503);
    pub const GATEWAY_TIMEOUT: Status = Status(504);
    pub const HTTP_VERSION_NOT_SUPPORTED: Status = Status(505);
    
    /// Get status code as number
    pub fn as_u16(&self) -> u16 {
        self.0
    }
    
    /// Check if status indicates success (2xx)
    pub fn is_success(&self) -> bool {
        self.0 >= 200 && self.0 < 300
    }
    
    /// Check if status indicates redirection (3xx)
    pub fn is_redirection(&self) -> bool {
        self.0 >= 300 && self.0 < 400
    }
    
    /// Check if status indicates client error (4xx)
    pub fn is_client_error(&self) -> bool {
        self.0 >= 400 && self.0 < 500
    }
    
    /// Check if status indicates server error (5xx)
    pub fn is_server_error(&self) -> bool {
        self.0 >= 500 && self.0 < 600
    }
    
    /// Check if status indicates error (4xx or 5xx)
    pub fn is_error(&self) -> bool {
        self.is_client_error() || self.is_server_error()
    }
    
    /// Get canonical reason phrase for status code
    pub fn canonical_reason(&self) -> &'static str {
        match self.0 {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            304 => "Not Modified",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "Unknown",
        }
    }
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.canonical_reason())
    }
}

impl From<u16> for Status {
    fn from(code: u16) -> Self {
        Status(code)
    }
}

impl From<Status> for u16 {
    fn from(status: Status) -> Self {
        status.0
    }
}

/// Content encoding types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentEncoding {
impl std::fmt::Display for ContentEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
impl std::str::FromStr for ContentEncoding {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
        }
    }
/// MIME types for common content types
pub mod mime {
    pub const TEXT_PLAIN: &str = "text/plain";
    pub const TEXT_HTML: &str = "text/html";
    pub const TEXT_CSS: &str = "text/css";
    pub const TEXT_JAVASCRIPT: &str = "text/javascript";
    pub const TEXT_XML: &str = "text/xml";
    
    pub const APPLICATION_JSON: &str = "application/json";
    pub const APPLICATION_XML: &str = "application/xml";
    pub const APPLICATION_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";
    pub const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";
    pub const APPLICATION_PDF: &str = "application/pdf";
    pub const APPLICATION_ZIP: &str = "application/zip";
    
    pub const MULTIPART_FORM_DATA: &str = "multipart/form-data";
    pub const MULTIPART_MIXED: &str = "multipart/mixed";
    
    pub const IMAGE_JPEG: &str = "image/jpeg";
    pub const IMAGE_PNG: &str = "image/png";
    pub const IMAGE_GIF: &str = "image/gif";
    pub const IMAGE_SVG: &str = "image/svg+xml";
    pub const IMAGE_WEBP: &str = "image/webp";
    
    pub const AUDIO_MPEG: &str = "audio/mpeg";
    pub const AUDIO_WAV: &str = "audio/wav";
    pub const AUDIO_OGG: &str = "audio/ogg";
    
    pub const VIDEO_MP4: &str = "video/mp4";
    pub const VIDEO_WEBM: &str = "video/webm";
    pub const VIDEO_OGG: &str = "video/ogg";
    
    /// Get MIME type from file extension
    pub fn from_extension(ext: &str) -> &'static str {
        match ext.to_lowercase().as_str() {
        }
    }
