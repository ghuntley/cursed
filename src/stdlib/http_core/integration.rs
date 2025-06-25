use crate::error::CursedError;
use crate::web::StatusCode;
// Integration utilities for CURSED web_vibez package
//
// Higher-level utilities that combine multiple components for common use cases.

use std::collections::HashMap;
use std::io::Write;

// use crate::stdlib::http_core::{
    Request, Response, Method, ContentType, FormData,
    HttpError, HttpResult, HttpValidator, ValidationRules
};

// use crate::stdlib::http_core::validation::SecurityConfig;

/// HTTP server context with request/response processing
#[derive(Debug)]
pub struct HttpContext {
    pub request: Request,
    validator: HttpValidator,
}

impl HttpContext {
    /// Create new HTTP context
    pub fn new(request: Request) -> Self {
        Self {
            request,
            validator: HttpValidator::new(),
        }
    }

    /// Create with custom validator
    pub fn with_validator(request: Request, validator: HttpValidator) -> Self {
        Self {
            request,
            validator,
        }
    }

    /// Validate the request
    pub fn validate(&self) -> HttpResult<()> {
        self.validator.validate_request(&self.request)
    }

    /// Extract and validate JSON data
    pub fn json<T>(&self) -> HttpResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.request.json()
    }

    /// Extract and validate form data
    pub fn form_data(&self) -> HttpResult<HashMap<String, String>> {
        match &self.request.body {
//             crate::stdlib::http_core::request::RequestBody::Form(form) => {
                let mut data = HashMap::new();
                for (key, values) in form.fields() {
                    if let Some(first_value) = values.first() {
                        if let Some(text) = first_value.as_text() {
                            data.insert(key.clone(), text.to_string());
                        }
                    }
                }
                
                // Validate form data
                self.validator.validate_form_data(&data)?;
                Ok(data)
            }
            _ => Err(HttpError::WrongBodyType("Expected form data".to_string())),
        }
    }

    /// Create response builder with security headers applied
    pub fn response(&self, status: StatusCode) -> SecureResponseBuilder {
        SecureResponseBuilder::new(status, &self.validator)
    }

    /// Create OK response
    pub fn ok(&self) -> SecureResponseBuilder {
        self.response(StatusCode::OK)
    }

    /// Create error response
    pub fn error(&self, status: StatusCode) -> SecureResponseBuilder {
        self.response(status)
    }

    /// Create JSON response
    pub fn json_response<T>(&self, data: &T) -> HttpResult<Response>
    where
        T: serde::Serialize,
    {
        self.ok().json(data).map(|builder| builder.build())
    }

    /// Create HTML response
    pub fn html_response<S>(&self, html: S) -> Response
    where
        S: Into<String>,
    {
        self.ok().html(html).build()
    }

    /// Create redirect response
    pub fn redirect<S>(&self, url: S, permanent: bool) -> Response
    where
        S: Into<String>,
    {
        let status = if permanent {
            StatusCode::MovedPermanently
        } else {
            StatusCode::Found
        };
        
        let mut response = Response::new(status).location(url);
        self.validator.apply_security_headers(&mut response);
        response
    }

    /// Handle file upload with validation
    pub fn handle_file_upload(&self, field_name: &str) -> HttpResult<(String, Vec<u8>, String)> {
        match &self.request.body {
//             crate::stdlib::http_core::request::RequestBody::Form(form) => {
                if let Some(values) = form.get_all(field_name) {
                    for value in values {
                        if let Some((filename, content, content_type)) = value.as_file() {
                            let ct = content_type.unwrap_or("application/octet-stream");
                            
                            // Validate file upload
                            self.validator.validate_file_upload(filename, content, ct)?;
                            
                            return Ok((filename.to_string(), content.to_vec(), ct.to_string()));
                        }
                    }
                }
                Err(HttpError::MissingField(format!("File field '{}' not found", field_name)))
            }
            _ => Err(HttpError::WrongBodyType("Expected multipart form data".to_string())),
        }
    }

    /// Get client IP address with proxy support
    pub fn client_ip(&self) -> Option<String> {
        // Check for forwarded headers first
        if let Some(forwarded) = self.request.header("X-Forwarded-For") {
            if let Some(first_ip) = forwarded.split(',').next() {
                return Some(first_ip.trim().to_string());
            }
        }

        if let Some(real_ip) = self.request.header("X-Real-IP") {
            return Some(real_ip.clone());
        }

        self.request.remote_addr.clone()
    }

    /// Check if request is from mobile device
    pub fn is_mobile(&self) -> bool {
        if let Some(user_agent) = self.request.user_agent() {
            let ua = user_agent.to_lowercase();
            ua.contains("mobile") || 
            ua.contains("android") || 
            ua.contains("iphone") || 
            ua.contains("ipod") ||
            ua.contains("blackberry") ||
            ua.contains("windows phone")
        } else {
            false
        }
    }

    /// Check if request accepts JSON
    pub fn accepts_json(&self) -> bool {
        self.request.accepts("application/json")
    }

    /// Check if request accepts HTML
    pub fn accepts_html(&self) -> bool {
        self.request.accepts("text/html")
    }
}

/// Response builder with automatic security headers
#[derive(Debug)]
pub struct SecureResponseBuilder {
    response: Response,
    validator: Option<HttpValidator>,
}

impl SecureResponseBuilder {
    /// Create new secure response builder
    pub fn new(status: StatusCode, validator: &HttpValidator) -> Self {
        Self {
            response: Response::new(status),
            validator: Some(validator.clone()),
        }
    }

    /// Set response body
//     pub fn body(mut self, body: crate::stdlib::http_core::response::ResponseBody) -> Self {
        self.response = self.response.body(body);
        self
    }

    /// Set text body
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.response = self.response.text(text);
        self
    }

    /// Set JSON body
    pub fn json<T: serde::Serialize>(mut self, data: &T) -> HttpResult<Self> {
        self.response = self.response.json(data)?;
        Ok(self)
    }

    /// Set HTML body
    pub fn html<S: Into<String>>(mut self, html: S) -> Self {
        self.response = self.response.html(html);
        self
    }

    /// Add header
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.response = self.response.header(key, value);
        self
    }

    /// Add cookie
//     pub fn cookie(mut self, cookie: crate::stdlib::http_core::Cookie) -> Self {
        self.response = self.response.cookie(cookie);
        self
    }

    /// Set cache control
    pub fn cache_control<S: Into<String>>(mut self, value: S) -> Self {
        self.response = self.response.cache_control(value);
        self
    }

    /// Set no cache
    pub fn no_cache(mut self) -> Self {
        self.response = self.response.no_cache();
        self
    }

    /// Set CORS headers
    pub fn cors(mut self, origin: &str) -> Self {
        self.response = self.response.cors(origin);
        self
    }

    /// Build the response with security headers applied
    pub fn build(mut self) -> Response {
        if let Some(validator) = &self.validator {
            validator.apply_security_headers(&mut self.response);
        }
        self.response
    }
}

/// HTTP request parser with comprehensive error handling
pub struct RequestParser {
    validator: HttpValidator,
    max_request_size: usize,
}

impl RequestParser {
    /// Create new request parser
    pub fn new() -> Self {
        Self {
            validator: HttpValidator::new(),
            max_request_size: 100 * 1024 * 1024, // 100MB default
        }
    }

    /// Create with custom validator
    pub fn with_validator(validator: HttpValidator) -> Self {
        Self {
            validator,
            max_request_size: 100 * 1024 * 1024,
        }
    }

    /// Set maximum request size
    pub fn max_request_size(mut self, size: usize) -> Self {
        self.max_request_size = size;
        self
    }

    /// Parse HTTP request from raw data
    pub fn parse(&self, raw_data: &str) -> HttpResult<HttpContext> {
        // Check size limit
        if raw_data.len() > self.max_request_size {
            return Err(HttpError::RequestTooLarge(
                format!("Request exceeds {} bytes", self.max_request_size)
            ));
        }

        // Parse request
        let request = Request::parse(raw_data)?;
        
        // Create context and validate
        let context = HttpContext::with_validator(request, self.validator.clone());
        context.validate()?;
        
        Ok(context)
    }

    /// Parse HTTP request with streaming support
    pub fn parse_stream<R: std::io::BufRead>(&self, reader: &mut R) -> HttpResult<HttpContext> {
        let mut raw_data = String::new();
        let mut total_size = 0;

        // Read request line by line with size checking
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)
                .map_err(|e| HttpError::IoError(e.to_string()))?;
            
            if bytes_read == 0 {
                break; // EOF
            }

            total_size += bytes_read;
            if total_size > self.max_request_size {
                return Err(HttpError::RequestTooLarge(
                    format!("Request exceeds {} bytes", self.max_request_size)
                ));
            }

            raw_data.push_str(&line);

            // Check for end of headers
            if line.trim().is_empty() {
                break;
            }
        }

        self.parse(&raw_data)
    }
}

impl Default for RequestParser {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP response writer with streaming support
pub struct ResponseWriter<W: Write> {
    writer: W,
    headers_sent: bool,
}

impl<W: Write> ResponseWriter<W> {
    /// Create new response writer
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            headers_sent: false,
        }
    }

    /// Write complete response
    pub fn write_response(&mut self, response: &Response) -> HttpResult<()> {
        response.write_to(&mut self.writer)
    }

    /// Write response headers only
    pub fn write_headers(&mut self, response: &Response) -> HttpResult<()> {
        if self.headers_sent {
            return Err(HttpError::custom("Headers already sent"));
        }

        // Write status line
        writeln!(self.writer, "{} {}", response.version, response.status)
            .map_err(|e| HttpError::IoError(e.to_string()))?;

        // Write headers
        for (key, value) in &response.headers {
            writeln!(self.writer, "{}: {}", key, value)
                .map_err(|e| HttpError::IoError(e.to_string()))?;
        }

        // Write cookies
        for cookie in &response.cookies {
            writeln!(self.writer, "Set-Cookie: {}", cookie.to_string())
                .map_err(|e| HttpError::IoError(e.to_string()))?;
        }

        // Empty line to separate headers from body
        writeln!(self.writer)
            .map_err(|e| HttpError::IoError(e.to_string()))?;

        self.headers_sent = true;
        Ok(())
    }

    /// Write response body data
    pub fn write_body(&mut self, data: &[u8]) -> HttpResult<()> {
        if !self.headers_sent {
            return Err(HttpError::custom("Headers must be sent before body"));
        }

        self.writer.write_all(data)
            .map_err(|e| HttpError::IoError(e.to_string()))?;
        
        Ok(())
    }

    /// Flush the writer
    pub fn flush(&mut self) -> HttpResult<()> {
        self.writer.flush()
            .map_err(|e| HttpError::IoError(e.to_string()))
    }
}

/// Utility functions for common HTTP operations
pub mod utils {
    use super::*;

    /// Create a simple JSON error response
    pub fn json_error(status: StatusCode, message: &str) -> Response {
        let error_data = serde_json::json!({
            "error": {
                "status": status as u16,
                "message": message
            }
        });

        Response::new(status)
            .json(&error_data)
            .unwrap_or_else(|_| Response::new(status).text(message))
    }

    /// Create a simple HTML error page
    pub fn html_error(status: StatusCode, message: &str) -> Response {
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>CursedError {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .error {{ color: #d32f2f; }}
        .code {{ font-size: 2em; font-weight: bold; }}
    </style>
</head>
<body>
    <div class="error">
        <div class="code">{}</div>
        <h1>{}</h1>
        <p>{}</p>
    </div>
</body>
</html>"#,
            status as u16, status as u16, status.reason_phrase(), message
        );

        Response::new(status).html(html)
    }

    /// Parse multipart boundary from content type
    pub fn parse_multipart_boundary(content_type: &str) -> Option<String> {
        ContentType::parse(content_type)
            .ok()?
            .mime_type()
            .boundary()
            .cloned()
    }

    /// Generate CSRF token
    pub fn generate_csrf_token() -> String {
        // In a real implementation, use a cryptographically secure random generator
        format!("csrf_token_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis())
    }

    /// Validate CSRF token
    pub fn validate_csrf_token(request: &Request, expected_token: &str) -> bool {
        // Check in form data first
        if let Some(token) = request.form("csrf_token") {
            return token == expected_token;
        }

        // Check in headers
        if let Some(token) = request.header("X-CSRF-Token") {
            return token == expected_token;
        }

        false
    }

    /// Extract file extension from filename
    pub fn file_extension(filename: &str) -> Option<&str> {
        std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
    }

    /// Generate ETag for response content
    pub fn generate_etag(content: &[u8]) -> String {
        // Simple hash-based ETag (in production, use a proper hash function)
        let hash = content.iter().fold(0u32, |acc, &b| {
            acc.wrapping_mul(31).wrapping_add(b as u32)
        });
        format!("\"{}\"", hash)
    }

    /// Check if request has matching ETag
    pub fn check_etag(request: &Request, etag: &str) -> bool {
        if let Some(if_none_match) = request.header("If-None-Match") {
            if_none_match.split(',').any(|tag| tag.trim() == etag)
        } else {
            false
        }
    }
}

