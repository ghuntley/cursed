use crate::web::StatusCode;
use crate::error::CursedError;
// HTTP Response Processing for CURSED web_vibez
//
// Comprehensive response building, formatting, and writing capabilities.

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// use crate::stdlib::http_core::{
    Headers, HeaderMap, ContentType, Cookie, CookieJar, HttpError, HttpResult
};

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

