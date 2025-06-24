use crate::error::Error;
/// HTTP Request Processing for CURSED web_vibez
///
/// Comprehensive request handling with proper parsing, validation, and security.

use std::collections::HashMap;
use std::io::{self, Read, BufRead, BufReader};
use std::str::FromStr;
use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::stdlib::http_core::{
    Headers, HeaderMap, Url, QueryParams, FormData, ContentType,
    Cookie, CookieJar, HttpError, HttpResult, HttpValidator
};

/// HTTP request methods
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

impl Method {
    /// Check if method allows a request body
    pub fn allows_body(&self) -> bool {
        matches!(self, Method::POST | Method::PUT | Method::PATCH)
    }

    /// Check if method is safe (read-only)
    pub fn is_safe(&self) -> bool {
        matches!(self, Method::GET | Method::HEAD | Method::OPTIONS | Method::TRACE)
    }

    /// Check if method is idempotent
    pub fn is_idempotent(&self) -> bool {
        matches!(self, Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS | Method::TRACE)
    }
}

impl FromStr for Method {
    type Err = HttpError;

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
            _ => Err(HttpError::InvalidMethod(s.to_string())),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::PATCH => "PATCH",
            Method::TRACE => "TRACE",
            Method::CONNECT => "CONNECT",
        };
        write!(f, "{}", s)
    }
}

/// HTTP request body types
pub enum RequestBody {
    Empty,
    Text(String),
    Binary(Vec<u8>),
    Json(serde_json::Value),
    Form(FormData),
    Stream(Box<dyn Read + Send + Sync>),
}

impl std::fmt::Debug for RequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestBody::Empty => write!(f, "Empty"),
            RequestBody::Text(s) => f.debug_tuple("Text").field(s).finish(),
            RequestBody::Binary(v) => f.debug_tuple("Binary").field(&format!("{} bytes", v.len())).finish(),
            RequestBody::Json(j) => f.debug_tuple("Json").field(j).finish(),
            RequestBody::Form(form) => f.debug_tuple("Form").field(form).finish(),
            RequestBody::Stream(_) => f.debug_tuple("Stream").field(&"<stream>").finish(),
        }
    }
}

impl Clone for RequestBody {
    fn clone(&self) -> Self {
        match self {
            RequestBody::Empty => RequestBody::Empty,
            RequestBody::Text(s) => RequestBody::Text(s.clone()),
            RequestBody::Binary(v) => RequestBody::Binary(v.clone()),
            RequestBody::Json(j) => RequestBody::Json(j.clone()),
            RequestBody::Form(form) => RequestBody::Form(form.clone()),
            RequestBody::Stream(_) => panic!("Cannot clone stream body"),
        }
    }
}

impl RequestBody {
    /// Get the body as bytes
    pub fn as_bytes(&self) -> HttpResult<Vec<u8>> {
        match self {
            RequestBody::Empty => Ok(Vec::new()),
            RequestBody::Text(s) => Ok(s.as_bytes().to_vec()),
            RequestBody::Binary(b) => Ok(b.clone()),
            RequestBody::Json(j) => {
                serde_json::to_vec(j).map_err(|e| HttpError::SerializationError(e.to_string()))
            }
            RequestBody::Form(f) => Ok(f.to_url_encoded().as_bytes().to_vec()),
            RequestBody::Stream(_) => Err(HttpError::StreamNotAvailable),
        }
    }

    /// Get the body as a string
    pub fn as_string(&self) -> HttpResult<String> {
        match self {
            RequestBody::Empty => Ok(String::new()),
            RequestBody::Text(s) => Ok(s.clone()),
            RequestBody::Binary(b) => {
                String::from_utf8(b.clone())
                    .map_err(|e| HttpError::EncodingError(e.to_string()))
            }
            RequestBody::Json(j) => {
                serde_json::to_string(j)
                    .map_err(|e| HttpError::SerializationError(e.to_string()))
            }
            RequestBody::Form(f) => Ok(f.to_url_encoded()),
            RequestBody::Stream(_) => Err(HttpError::StreamNotAvailable),
        }
    }

    /// Get the content length
    pub fn content_length(&self) -> usize {
        match self {
            RequestBody::Empty => 0,
            RequestBody::Text(s) => s.len(),
            RequestBody::Binary(b) => b.len(),
            RequestBody::Json(j) => {
                serde_json::to_string(j).map(|s| s.len()).unwrap_or(0)
            }
            RequestBody::Form(f) => f.to_url_encoded().len(),
            RequestBody::Stream(_) => 0, // Unknown length for streams
        }
    }
}

/// Comprehensive HTTP Request structure
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    pub method: Method,
    /// Request URL with path and query parameters
    pub url: Url,
    /// HTTP version (e.g., "HTTP/1.1")
    pub version: String,
    /// Request headers
    pub headers: HeaderMap,
    /// Request body
    pub body: RequestBody,
    /// Parsed cookies
    pub cookies: CookieJar,
    /// Client IP address
    pub remote_addr: Option<String>,
    /// Request timestamp
    pub timestamp: Instant,
    /// Request ID for tracing
    pub id: String,
}

impl Request {
    /// Create a new request
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            version: "HTTP/1.1".to_string(),
            headers: HeaderMap::new(),
            body: RequestBody::Empty,
            cookies: CookieJar::new(),
            remote_addr: None,
            timestamp: Instant::now(),
            id: Uuid::new_v4().to_string(),
        }
    }

    /// Parse an HTTP request from a raw string
    pub fn parse(raw_request: &str) -> HttpResult<Self> {
        let lines: Vec<&str> = raw_request.split("\n").collect();
        if lines.is_empty() {
            return Err(HttpError::InvalidRequest("Empty request".to_string()));
        }

        // Parse request line
        let request_line = lines[0];
        let (method, url, version) = Self::parse_request_line(request_line)?;

        let mut request = Self::new(method, url);
        request.version = version;

        // Find the separator between headers and body
        let mut body_start = lines.len();
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.is_empty() {
                body_start = i + 1;
                break;
            }
        }

        // Parse headers
        for line in &lines[1..body_start.min(lines.len())] {
            if !line.is_empty() {
                request.parse_header_line(line)?;
            }
        }

        // Parse cookies from headers
        request.parse_cookies()?;

        // Parse body if present
        if body_start < lines.len() {
            let body_content = lines[body_start..].join("\n");
            request.parse_body(body_content)?;
        }

        Ok(request)
    }

    /// Parse request line (METHOD URL VERSION)
    fn parse_request_line(line: &str) -> HttpResult<(Method, Url, String)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(HttpError::InvalidRequest(
                "Invalid request line format".to_string()
            ));
        }

        let method = Method::from_str(parts[0])?;
        let url = Url::parse(parts[1])?;
        let version = parts[2].to_string();

        // Validate HTTP version
        if !version.starts_with("HTTP/") {
            return Err(HttpError::UnsupportedVersion(version));
        }

        Ok((method, url, version))
    }

    /// Parse a header line
    fn parse_header_line(&mut self, line: &str) -> HttpResult<()> {
        if let Some(colon_pos) = line.find(':') {
            let name = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            
            // Validate header name
            if name.is_empty() || name.contains(char::is_whitespace) {
                return Err(HttpError::InvalidHeader(format!("Invalid header name: {}", name)));
            }

            self.headers.insert(name, value);
        } else {
            return Err(HttpError::InvalidHeader(format!("Invalid header format: {}", line)));
        }
        Ok(())
    }

    /// Parse cookies from headers
    fn parse_cookies(&mut self) -> HttpResult<()> {
        if let Some(cookie_header) = self.headers.get("Cookie") {
            for cookie_str in cookie_header.split(';') {
                if let Ok(cookie) = Cookie::parse(cookie_str.trim()) {
                    self.cookies.add(cookie);
                }
            }
        }
        Ok(())
    }

    /// Parse request body based on content type
    fn parse_body(&mut self, body_content: String) -> HttpResult<()> {
        if body_content.is_empty() {
            return Ok(());
        }

        let content_type = self.get_content_type();
        
        match content_type.as_str() {
            "application/json" => {
                match serde_json::from_str(&body_content) {
                    Ok(json) => self.body = RequestBody::Json(json),
                    Err(e) => return Err(HttpError::SerializationError(e.to_string())),
                }
            }
            "application/x-www-form-urlencoded" => {
                let form_data = FormData::from_url_encoded(&body_content)?;
                self.body = RequestBody::Form(form_data);
            }
            "multipart/form-data" => {
                // Extract boundary from Content-Type header
                if let Some(content_type_header) = self.headers.get("Content-Type") {
                    if let Some(boundary) = Self::extract_boundary(content_type_header) {
                        // Use the form_data MultipartData for parsing
                        let mut parser = crate::stdlib::http_core::form_data::MultipartData::new(boundary);
                        match parser.parse(body_content.as_bytes()) {
                            Ok(_) => {
                                self.body = RequestBody::Form(parser.into_form_data());
                            }
                            Err(e) => {
                                return Err(HttpError::FormDataError(format!("Multipart parse error: {}", e)));
                            }
                        }
                    } else {
                        return Err(HttpError::FormDataError("Missing boundary in Content-Type".to_string()));
                    }
                } else {
                    return Err(HttpError::FormDataError("Missing Content-Type header".to_string()));
                }
            }
            _ => {
                // Try to detect if it's binary or text
                if body_content.chars().all(|c| c.is_ascii() || c.is_ascii_graphic() || c.is_whitespace()) {
                    self.body = RequestBody::Text(body_content);
                } else {
                    self.body = RequestBody::Binary(body_content.into_bytes());
                }
            }
        }

        Ok(())
    }

    /// Get the content type from headers
    pub fn get_content_type(&self) -> String {
        self.headers
            .get("Content-Type")
            .unwrap_or(&"text/plain".to_string())
            .split(';')
            .next()
            .unwrap_or("text/plain")
            .trim()
            .to_lowercase()
    }

    /// Get a header value
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }

    /// Get all values for a header (handles multiple values)
    pub fn header_values(&self, name: &str) -> Vec<&str> {
        self.headers.get_all(name)
    }

    /// Get query parameter
    pub fn query(&self, name: &str) -> Option<&str> {
        self.url.query_params().get(name)
    }

    /// Get all query parameters
    pub fn query_params(&self) -> &QueryParams {
        self.url.query_params()
    }

    /// Get form field value
    pub fn form(&self, name: &str) -> Option<String> {
        match &self.body {
            RequestBody::Form(form_data) => form_data.get(name),
            _ => None,
        }
    }

    /// Get JSON field value
    pub fn json<T>(&self) -> HttpResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match &self.body {
            RequestBody::Json(json) => {
                serde_json::from_value(json.clone())
                    .map_err(|e| HttpError::SerializationError(e.to_string()))
            }
            RequestBody::Text(text) => {
                serde_json::from_str(text)
                    .map_err(|e| HttpError::SerializationError(e.to_string()))
            }
            _ => Err(HttpError::WrongBodyType("Expected JSON body".to_string())),
        }
    }

    /// Get cookie value
    pub fn cookie(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }

    /// Get the request path
    pub fn path(&self) -> &str {
        self.url.path()
    }

    /// Get path parameters (for route matching)
    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.url.path_params().get(name)
    }

    /// Check if request accepts content type
    pub fn accepts(&self, content_type: &str) -> bool {
        if let Some(accept_header) = self.header("Accept") {
            accept_header.contains(content_type) || accept_header.contains("*/*")
        } else {
            false
        }
    }

    /// Get user agent
    pub fn user_agent(&self) -> Option<&String> {
        self.header("User-Agent")
    }

    /// Get authorization header
    pub fn authorization(&self) -> Option<&String> {
        self.header("Authorization")
    }

    /// Get host
    pub fn host(&self) -> Option<&String> {
        self.header("Host")
    }

    /// Get content length
    pub fn content_length(&self) -> usize {
        if let Some(length_str) = self.header("Content-Length") {
            length_str.parse().unwrap_or(0)
        } else {
            self.body.content_length()
        }
    }

    /// Check if connection should be kept alive
    pub fn keep_alive(&self) -> bool {
        if let Some(connection) = self.header("Connection") {
            connection.to_lowercase() == "keep-alive"
        } else {
            self.version == "HTTP/1.1" // Default for HTTP/1.1
        }
    }

    /// Validate the request
    pub fn validate(&self) -> HttpResult<()> {
        let validator = HttpValidator::new();
        validator.validate_request(self)
    }

    /// Get request duration since creation
    pub fn duration(&self) -> Duration {
        self.timestamp.elapsed()
    }

    /// Extract boundary from Content-Type header
    fn extract_boundary(content_type: &str) -> Option<String> {
        // Look for boundary parameter in Content-Type header
        // Example: multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW
        for part in content_type.split(';') {
            let part = part.trim();
            if part.starts_with("boundary=") {
                let boundary = &part[9..]; // Skip "boundary="
                // Remove quotes if present
                if boundary.starts_with('"') && boundary.ends_with('"') {
                    return Some(boundary[1..boundary.len() - 1].to_string());
                } else {
                    return Some(boundary.to_string());
                }
            }
        }
        None
    }
}

/// Request builder for fluent request construction
#[derive(Debug)]
pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    /// Create new request builder
    pub fn new(method: Method, url: &str) -> HttpResult<Self> {
        let url = Url::parse(url)?;
        Ok(Self {
            request: Request::new(method, url),
        })
    }

    /// Add header
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.request.headers.insert(key.into(), value.into());
        self
    }

    /// Add multiple headers
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        for (key, value) in headers {
            self.request.headers.insert(key, value);
        }
        self
    }

    /// Set request body
    pub fn body(mut self, body: RequestBody) -> Self {
        self.request.body = body;
        self
    }

    /// Set JSON body
    pub fn json<T>(mut self, data: &T) -> HttpResult<Self>
    where
        T: serde::Serialize,
    {
        let json = serde_json::to_value(data)
            .map_err(|e| HttpError::SerializationError(e.to_string()))?;
        self.request.body = RequestBody::Json(json);
        self.request.headers.insert("Content-Type".to_string(), "application/json".to_string());
        Ok(self)
    }

    /// Set form body
    pub fn form(mut self, form_data: FormData) -> Self {
        self.request.body = RequestBody::Form(form_data);
        self.request.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );
        self
    }

    /// Add cookie
    pub fn cookie(mut self, cookie: Cookie) -> Self {
        self.request.cookies.add(cookie);
        self
    }

    /// Set remote address
    pub fn remote_addr<A>(mut self, addr: A) -> Self
    where
        A: Into<String>,
    {
        self.request.remote_addr = Some(addr.into());
        self
    }

    /// Build the request
    pub fn build(self) -> Request {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_parsing() {
        assert_eq!(Method::from_str("GET").unwrap(), Method::GET);
        assert_eq!(Method::from_str("post").unwrap(), Method::POST);
        assert!(Method::from_str("INVALID").is_err());
    }

    #[test]
    fn test_method_properties() {
        assert!(Method::GET.is_safe());
        assert!(Method::GET.is_idempotent());
        assert!(!Method::GET.allows_body());
        
        assert!(!Method::POST.is_safe());
        assert!(!Method::POST.is_idempotent());
        assert!(Method::POST.allows_body());
    }

    #[test]
    fn test_request_parsing() {
        let raw = "GET /path?q=value HTTP/1.1\r\nHost: example.com\r\nUser-Agent: test\r\n\r\n";
        let request = Request::parse(raw).unwrap();
        
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.path(), "/path");
        assert_eq!(request.query("q"), Some("value"));
        assert_eq!(request.header("Host"), Some(&"example.com".to_string()));
    }

    #[test]
    fn test_request_builder() {
        let request = RequestBuilder::new(Method::POST, "http://example.com/api")
            .unwrap()
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer token")
            .build();

        assert_eq!(request.method, Method::POST);
        assert_eq!(request.header("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(request.header("Authorization"), Some(&"Bearer token".to_string()));
    }
}
