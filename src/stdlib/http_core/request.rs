use crate::error::CursedError;
/// HTTP Request Processing for CURSED web_vibez
///
/// Comprehensive request handling with proper parsing, validation, and security.

use std::collections::HashMap;
use std::io::{self, Read, BufRead, BufReader};
use std::str::FromStr;
use std::time::{Duration, Instant};

use uuid::Uuid;

// Placeholder imports disabled
    Cookie, CookieJar, HttpError, HttpResult, HttpValidator
// };

/// HTTP request methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
impl Method {
    /// Check if method allows a request body
    pub fn allows_body(&self) -> bool {
        matches!(self, Method::POST | Method::PUT | Method::PATCH)
    /// Check if method is safe (read-only)
    pub fn is_safe(&self) -> bool {
        matches!(self, Method::GET | Method::HEAD | Method::OPTIONS | Method::TRACE)
    /// Check if method is idempotent
    pub fn is_idempotent(&self) -> bool {
        matches!(self, Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS | Method::TRACE)
    }
}

impl FromStr for Method {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
        }
    }
impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
        write!(f, "{}", s)
    }
}

/// HTTP request body types
pub enum RequestBody {
impl std::fmt::Debug for RequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
impl Clone for RequestBody {
    fn clone(&self) -> Self {
        match self {
        }
    }
impl RequestBody {
    /// Get the body as bytes
    pub fn as_bytes(&self) -> HttpResult<Vec<u8>> {
        match self {
            RequestBody::Json(j) => {
                serde_json::to_vec(j).map_err(|e| HttpError::SerializationError(e.to_string()))
            }
        }
    }

    /// Get the body as a string
    pub fn as_string(&self) -> HttpResult<String> {
        match self {
            RequestBody::Binary(b) => {
                String::from_utf8(b.clone())
                    .map_err(|e| HttpError::EncodingError(e.to_string()))
            }
            RequestBody::Json(j) => {
                serde_json::to_string(j)
                    .map_err(|e| HttpError::SerializationError(e.to_string()))
            }
        }
    }

    /// Get the content length
    pub fn content_length(&self) -> usize {
        match self {
            RequestBody::Json(j) => {
                serde_json::to_string(j).map(|s| s.len()).unwrap_or(0)
            }
            RequestBody::Stream(_) => 0, // Unknown length for streams
        }
    }
/// Comprehensive HTTP Request structure
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    /// Request URL with path and query parameters
    /// HTTP version (e.g., "HTTP/1.1")
    /// Request headers
    /// Request body
    /// Parsed cookies
    /// Client IP address
    /// Request timestamp
    /// Request ID for tracing
impl Request {
    /// Create a new request
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
        }
    }

    /// Parse an HTTP request from a raw string
    pub fn parse(raw_request: &str) -> HttpResult<Self> {
        let lines: Vec<&str> = raw_request.split("\n").collect();
        if lines.is_empty() {
            return Err(HttpError::InvalidRequest("Empty request".to_string()));
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
        Ok(request)
    /// Parse request line (METHOD URL VERSION)
    fn parse_request_line(line: &str) -> HttpResult<(Method, Url, String)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(HttpError::InvalidRequest(
                "Invalid request line format".to_string()
            ));
        let method = Method::from_str(parts[0])?;
        let url = Url::parse(parts[1])?;
        let version = parts[2].to_string();

        // Validate HTTP version
        if !version.starts_with("HTTP/") {
            return Err(HttpError::UnsupportedVersion(version));
        Ok((method, url, version))
    /// Parse a header line
    fn parse_header_line(&mut self, line: &str) -> HttpResult<()> {
        if let Some(colon_pos) = line.find(':') {
            let name = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            
            // Validate header name
            if name.is_empty() || name.contains(char::is_whitespace) {
                return Err(HttpError::InvalidHeader(format!("Invalid header name: {}", name)));
            self.headers.insert(name, value);
        } else {
            return Err(HttpError::InvalidHeader(format!("Invalid header format: {}", line)));
        }
        Ok(())
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
    /// Parse request body based on content type
    fn parse_body(&mut self, body_content: String) -> HttpResult<()> {
        if body_content.is_empty() {
            return Ok(());
        let content_type = self.get_content_type();
        
        match content_type.as_str() {
            "application/json" => {
                match serde_json::from_str(&body_content) {
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
//                         let mut parser = crate::stdlib::http_core::form_data::MultipartData::new(boundary);
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
        Ok(())
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
    /// Get a header value
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    /// Get all values for a header (handles multiple values)
    pub fn header_values(&self, name: &str) -> Vec<&str> {
        self.headers.get_all(name)
    /// Get query parameter
    pub fn query(&self, name: &str) -> Option<&str> {
        self.url.query_params().get(name)
    /// Get all query parameters
    pub fn query_params(&self) -> &QueryParams {
        self.url.query_params()
    /// Get form field value
    pub fn form(&self, name: &str) -> Option<String> {
        match &self.body {
        }
    }

    /// Get JSON field value
    pub fn json<T>(&self) -> HttpResult<T>
    where
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
        }
    }

    /// Get cookie value
    pub fn cookie(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    /// Get the request path
    pub fn path(&self) -> &str {
        self.url.path()
    /// Get path parameters (for route matching)
    pub fn path_param(&self, name: &str) -> Option<&str> {
        self.url.path_params().get(name)
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
    /// Get authorization header
    pub fn authorization(&self) -> Option<&String> {
        self.header("Authorization")
    /// Get host
    pub fn host(&self) -> Option<&String> {
        self.header("Host")
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
    /// Get request duration since creation
    pub fn duration(&self) -> Duration {
        self.timestamp.elapsed()
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
impl RequestBuilder {
    /// Create new request builder
    pub fn new(method: Method, url: &str) -> HttpResult<Self> {
        let url = Url::parse(url)?;
        Ok(Self {
        })
    /// Add header
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
    {
        self.request.headers.insert(key.into(), value.into());
        self
    /// Add multiple headers
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        for (key, value) in headers {
            self.request.headers.insert(key, value);
        }
        self
    /// Set request body
    pub fn body(mut self, body: RequestBody) -> Self {
        self.request.body = body;
        self
    /// Set JSON body
    pub fn json<T>(mut self, data: &T) -> HttpResult<Self>
    where
    {
        let json = serde_json::to_value(data)
            .map_err(|e| HttpError::SerializationError(e.to_string()))?;
        self.request.body = RequestBody::Json(json);
        self.request.headers.insert("Content-Type".to_string(), "application/json".to_string());
        Ok(self)
    /// Set form body
    pub fn form(mut self, form_data: FormData) -> Self {
        self.request.body = RequestBody::Form(form_data);
        self.request.headers.insert(
            "application/x-www-form-urlencoded".to_string(),
        );
        self
    /// Add cookie
    pub fn cookie(mut self, cookie: Cookie) -> Self {
        self.request.cookies.add(cookie);
        self
    /// Set remote address
    pub fn remote_addr<A>(mut self, addr: A) -> Self
    where
    {
        self.request.remote_addr = Some(addr.into());
        self
    /// Build the request
    pub fn build(self) -> Request {
        self.request
    }
}

