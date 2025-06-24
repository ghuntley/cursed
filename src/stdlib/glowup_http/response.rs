use crate::web::StatusCode;
// HTTP response types and utilities for GlowUpHTTP

use crate::stdlib::glowup_http::error::{GlowUpError, GlowUpResult};
use crate::stdlib::glowup_http::request::{HeaderMap, Cookie};
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::{debug, instrument};
use crate::error::Error;

/// HTTP status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(pub u16);

impl StatusCode {
    // 1xx Informational
    pub const CONTINUE: StatusCode = StatusCode(100);
    pub const SWITCHING_PROTOCOLS: StatusCode = StatusCode(101);
    
    // 2xx Success
    pub const OK: StatusCode = StatusCode(200);
    pub const CREATED: StatusCode = StatusCode(201);
    pub const ACCEPTED: StatusCode = StatusCode(202);
    pub const NO_CONTENT: StatusCode = StatusCode(204);
    pub const PARTIAL_CONTENT: StatusCode = StatusCode(206);
    
    // 3xx Redirection
    pub const MULTIPLE_CHOICES: StatusCode = StatusCode(300);
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(301);
    pub const FOUND: StatusCode = StatusCode(302);
    pub const SEE_OTHER: StatusCode = StatusCode(303);
    pub const NOT_MODIFIED: StatusCode = StatusCode(304);
    pub const TEMPORARY_REDIRECT: StatusCode = StatusCode(307);
    pub const PERMANENT_REDIRECT: StatusCode = StatusCode(308);
    
    // 4xx Client Errors
    pub const BAD_REQUEST: StatusCode = StatusCode(400);
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);
    pub const FORBIDDEN: StatusCode = StatusCode(403);
    pub const NOT_FOUND: StatusCode = StatusCode(404);
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(405);
    pub const NOT_ACCEPTABLE: StatusCode = StatusCode(406);
    pub const CONFLICT: StatusCode = StatusCode(409);
    pub const GONE: StatusCode = StatusCode(410);
    pub const UNPROCESSABLE_ENTITY: StatusCode = StatusCode(422);
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode(429);
    
    // 5xx Server Errors
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
    pub const NOT_IMPLEMENTED: StatusCode = StatusCode(501);
    pub const BAD_GATEWAY: StatusCode = StatusCode(502);
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);
    pub const GATEWAY_TIMEOUT: StatusCode = StatusCode(504);
    
    pub fn as_u16(&self) -> u16 {
        self.0
    }
    
    pub fn canonical_reason(&self) -> &'static str {
        match self.0 {
            100 => "Continue",
            101 => "Switching Protocols",
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            204 => "No Content",
            206 => "Partial Content",
            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            409 => "Conflict",
            410 => "Gone",
            422 => "Unprocessable Entity",
            429 => "Too Many Requests",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            _ => "Unknown",
        }
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.canonical_reason())
    }
}

/// Response writer that implements the ResponderVibe interface
/// This follows the CURSED spec's `ResponderVibe` naming
pub struct ResponderVibe {
    headers: Arc<Mutex<HeaderMap>>,
    status: Arc<Mutex<Option<StatusCode>>>,
    body: Arc<Mutex<Vec<u8>>>,
    writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    written: Arc<Mutex<bool>>,
}

impl ResponderVibe {
    /// Create a new response writer
    pub fn new() -> Self {
        Self {
            headers: Arc::new(Mutex::new(HeaderMap::new())),
            status: Arc::new(Mutex::new(None)),
            body: Arc::new(Mutex::new(Vec::new())),
            writer: Arc::new(Mutex::new(None)),
            written: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Create a new response writer with a specific writer
    pub fn with_writer(writer: Box<dyn Write + Send>) -> Self {
        Self {
            headers: Arc::new(Mutex::new(HeaderMap::new())),
            status: Arc::new(Mutex::new(None)),
            body: Arc::new(Mutex::new(Vec::new())),
            writer: Arc::new(Mutex::new(Some(writer))),
            written: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Get headers (mutable access)
    pub fn header(&self) -> Arc<Mutex<HeaderMap>> {
        self.headers.clone()
    }
    
    /// Write status code
    #[instrument(skip(self))]
    pub fn write_header(&self, status_code: StatusCode) {
        let mut status = self.status.lock().unwrap();
        if status.is_none() {
            *status = Some(status_code);
            debug!("Set response status: {}", status_code);
        }
    }
    
    /// Write bytes to response body
    #[instrument(skip(self, data))]
    pub fn write(&self, data: &[u8]) -> GlowUpResult<usize> {
        // Set default status if not set
        {
            let mut status = self.status.lock().unwrap();
            if status.is_none() {
                *status = Some(StatusCode::OK);
            }
        }
        
        // Write to body buffer
        let mut body = self.body.lock().unwrap();
        body.extend_from_slice(data);
        
        // Mark as written
        *self.written.lock().unwrap() = true;
        
        Ok(data.len())
    }
    
    /// Write JSON response
    #[instrument(skip(self, value))]
    pub fn write_json<T: serde::Serialize>(&self, value: &T) -> GlowUpResult<()> {
        let json_data = serde_json::to_vec(value)?;
        
        // Set content type
        {
            let mut headers = self.headers.lock().unwrap();
            headers.insert("content-type".to_string(), "application/json".to_string());
        }
        
        self.write(&json_data)?;
        Ok(())
    }
    
    /// Write template response (placeholder implementation)
    #[instrument(skip(self, data))]
    pub fn write_template(&self, name: &str, data: &dyn std::any::Any) -> GlowUpResult<()> {
        // This would integrate with a template engine in a real implementation
        let template_content = format!("Template: {} with data", name);
        
        {
            let mut headers = self.headers.lock().unwrap();
            headers.insert("content-type".to_string(), "text/html".to_string());
        }
        
        self.write(template_content.as_bytes())?;
        Ok(())
    }
    
    /// Send redirect response
    #[instrument(skip(self))]
    pub fn redirect(&self, url: &str, code: StatusCode) -> GlowUpResult<()> {
        {
            let mut headers = self.headers.lock().unwrap();
            headers.insert("location".to_string(), url.to_string());
        }
        
        self.write_header(code);
        Ok(())
    }
    
    /// Set cookie
    #[instrument(skip(self))]
    pub fn set_cookie(&self, cookie: &Cookie) {
        let mut cookie_value = format!("{}={}", cookie.name, cookie.value);
        
        if let Some(domain) = &cookie.domain {
            cookie_value.push_str(&format!("; Domain={}", domain));
        }
        
        if let Some(path) = &cookie.path {
            cookie_value.push_str(&format!("; Path={}", path));
        }
        
        if cookie.secure {
            cookie_value.push_str("; Secure");
        }
        
        if cookie.http_only {
            cookie_value.push_str("; HttpOnly");
        }
        
        if let Some(max_age) = cookie.max_age {
            cookie_value.push_str(&format!("; Max-Age={}", max_age));
        }
        
        let mut headers = self.headers.lock().unwrap();
        // Multiple Set-Cookie headers are allowed
        if let Some(existing) = headers.get("set-cookie") {
            headers.insert("set-cookie".to_string(), format!("{}\r\nSet-Cookie: {}", existing, cookie_value));
        } else {
            headers.insert("set-cookie".to_string(), cookie_value);
        }
    }
    
    /// Fluent interface: set status
    #[instrument(skip(self))]
    pub fn status(mut self, code: StatusCode) -> Self {
        self.write_header(code);
        self
    }
    
    /// Fluent interface: write JSON
    #[instrument(skip(self, value))]
    pub fn json<T: serde::Serialize>(self, value: &T) -> GlowUpResult<Self> {
        self.write_json(value)?;
        Ok(self)
    }
    
    /// Fluent interface: write text
    #[instrument(skip(self))]
    pub fn text(self, text: &str) -> GlowUpResult<Self> {
        {
            let mut headers = self.headers.lock().unwrap();
            headers.insert("content-type".to_string(), "text/plain".to_string());
        }
        
        self.write(text.as_bytes())?;
        Ok(self)
    }
    
    /// Fluent interface: write HTML
    #[instrument(skip(self))]
    pub fn html(self, html: &str) -> GlowUpResult<Self> {
        {
            let mut headers = self.headers.lock().unwrap();
            headers.insert("content-type".to_string(), "text/html".to_string());
        }
        
        self.write(html.as_bytes())?;
        Ok(self)
    }
    
    /// Fluent interface: send file
    #[instrument(skip(self))]
    pub fn file(self, filepath: &str) -> GlowUpResult<Self> {
        use std::fs;
        
        let file_content = fs::read(filepath)
            .map_err(|e| GlowUpError::io_error(format!("Failed to read file {}: {}", filepath, e)))?;
        
        // Try to determine content type from extension
        let content_type = match std::path::Path::new(filepath).extension().and_then(|s| s.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css", 
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("pdf") => "application/pdf",
            _ => "application/octet-stream",
        };
        
        {
            let mut headers = self.headers.lock().unwrap();
            headers.insert("content-type".to_string(), content_type.to_string());
        }
        
        self.write(&file_content)?;
        Ok(self)
    }
    
    /// Get the current status code
    pub fn get_status(&self) -> Option<StatusCode> {
        *self.status.lock().unwrap()
    }
    
    /// Get the response headers (read-only)
    pub fn get_headers(&self) -> HeaderMap {
        self.headers.lock().unwrap().clone()
    }
    
    /// Get the response body
    pub fn get_body(&self) -> Vec<u8> {
        self.body.lock().unwrap().clone()
    }
    
    /// Check if response has been written to
    pub fn is_written(&self) -> bool {
        *self.written.lock().unwrap()
    }
    
    /// Flush the response to the underlying writer
    #[instrument(skip(self))]
    pub fn flush(&self) -> GlowUpResult<()> {
        if let Some(writer) = &mut *self.writer.lock().unwrap() {
            // Write status line
            let status = self.status.lock().unwrap().unwrap_or(StatusCode::OK);
            writeln!(writer, "HTTP/1.1 {}", status)?;
            
            // Write headers
            let headers = self.headers.lock().unwrap();
            for (name, value) in headers.iter() {
                writeln!(writer, "{}: {}", name, value)?;
            }
            
            // Write content-length if not present
            if !headers.contains_key("content-length") {
                let body = self.body.lock().unwrap();
                writeln!(writer, "Content-Length: {}", body.len())?;
            }
            
            // End headers
            writeln!(writer)?;
            
            // Write body
            let body = self.body.lock().unwrap();
            writer.write_all(&body)?;
            
            writer.flush()?;
        }
        
        Ok(())
    }
}

impl Default for ResponderVibe {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ResponderVibe {
    fn clone(&self) -> Self {
        Self {
            headers: self.headers.clone(),
            status: self.status.clone(),
            body: self.body.clone(),
            writer: Arc::new(Mutex::new(None)), // Don't clone the writer
            written: self.written.clone(),
        }
    }
}

/// Client response type (for HTTP clients)
#[derive(Debug, Clone)]
pub struct VibeResponse {
    /// Status code (u16 for compatibility with reqwest)
    pub status: u16,
    /// Response headers
    pub headers: HeaderMap,
    /// Response body
    pub body: Vec<u8>,
    /// Status line (computed from status)
    pub status_line: String,
    /// Status code object
    pub status_code: StatusCode,
    /// HTTP protocol version
    pub proto: String,
    /// Content length
    pub content_length: i64,
    /// Transfer encoding
    pub transfer_encoding: Vec<String>,
    /// Whether connection should be closed
    pub close: bool,
    /// Whether response was uncompressed
    pub uncompressed: bool,
    /// Trailer headers
    pub trailer: HeaderMap,
}

impl VibeResponse {
    /// Create a new response
    pub fn new(status_code: StatusCode) -> Self {
        Self {
            status: status_code.as_u16(),
            headers: HeaderMap::new(),
            body: Vec::new(),
            status_line: status_code.to_string(),
            status_code,
            proto: "HTTP/1.1".to_string(),
            content_length: 0,
            transfer_encoding: Vec::new(),
            close: false,
            uncompressed: false,
            trailer: HeaderMap::new(),
        }
    }
    
    /// Create a new response from status code (u16)
    pub fn from_status(status: u16) -> Self {
        let status_code = StatusCode(status);
        Self {
            status,
            headers: HeaderMap::new(),
            body: Vec::new(),
            status_line: status_code.to_string(),
            status_code,
            proto: "HTTP/1.1".to_string(),
            content_length: 0,
            transfer_encoding: Vec::new(),
            close: false,
            uncompressed: false,
            trailer: HeaderMap::new(),
        }
    }
    
    /// Get cookies from response
    pub fn cookies(&self) -> Vec<Cookie> {
        let mut cookies = Vec::new();
        
        for (name, value) in &self.headers {
            if name.to_lowercase() == "set-cookie" {
                // Parse set-cookie header - simplified implementation
                if let Some(equals_pos) = value.find('=') {
                    let cookie_name = value[..equals_pos].trim().to_string();
                    let rest = &value[equals_pos + 1..];
                    let cookie_value = if let Some(semicolon_pos) = rest.find(';') {
                        rest[..semicolon_pos].trim().to_string()
                    } else {
                        rest.trim().to_string()
                    };
                    
                    cookies.push(Cookie::new(cookie_name, cookie_value));
                }
            }
        }
        
        cookies
    }
    
    /// Parse JSON from response body
    #[instrument(skip(self))]
    pub fn parse_json<T: serde::de::DeserializeOwned>(&self) -> GlowUpResult<T> {
        let json_str = String::from_utf8(self.body.clone())
            .map_err(|e| GlowUpError::parse_error(format!("Invalid UTF-8 in JSON response: {}", e)))?;
        
        serde_json::from_str(&json_str)
            .map_err(|e| GlowUpError::json(e.to_string()))
    }
    
    /// Get response body as string
    #[instrument(skip(self))]
    pub fn string(&self) -> GlowUpResult<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| GlowUpError::parse_error(format!("Invalid UTF-8 in response body: {}", e)))
    }
    
    /// Get response body as bytes
    pub fn bytes(&self) -> Vec<u8> {
        self.body.clone()
    }
}
