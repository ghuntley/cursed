// HTTP request types and utilities for GlowUpHTTP

// use crate::stdlib::glowup_http::error::{GlowUpError, GlowUpResult};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, instrument};
use crate::error::CursedError;

/// HTTP methods
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
impl FromStr for Method {
    type Err = GlowUpError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
        }
    }
impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
        write!(f, "{}", method_str)
    }
}

/// HTTP version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
impl FromStr for HttpVersion {
    type Err = GlowUpError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(HttpVersion::Http1_0),
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2.0" | "HTTP/2" => Ok(HttpVersion::Http2_0),
        }
    }
impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version_str = match self {
            HttpVersion::Http1_0 => "HTTP/1.0",
            HttpVersion::Http1_1 => "HTTP/1.1",
            HttpVersion::Http2_0 => "HTTP/2.0",
        write!(f, "{}", version_str)
    }
}

/// Header map type
pub type HeaderMap = HashMap<String, String>;

/// Cookie representation
#[derive(Debug, Clone)]
pub struct Cookie {
#[derive(Debug, Clone)]
pub enum SameSite {
impl Cookie {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
        }
    }
    
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }
}

/// Multipart form data
#[derive(Debug, Clone)]
pub struct MultipartForm {
#[derive(Debug, Clone)]
pub struct FormFile {
/// Context type for request processing
pub type VibeContext = Arc<dyn std::any::Any + Send + Sync>;

/// Represents an HTTP request received by a server or to be sent by a client
/// This follows the CURSED spec's `VibeRequest` naming
#[derive(Debug, Clone)]
pub struct VibeRequest {
    /// HTTP method
    /// Request URL/path
    /// HTTP protocol version
    /// Request headers
    /// Request body
    /// Content length
    /// Transfer encoding
    /// Host header value
    /// Parsed form data
    /// Parsed POST form data
    /// Multipart form data
    /// Trailer headers
    /// Remote address
    /// Request URI (raw)
    /// TLS connection state
    /// Request context
    /// Path parameters (for routing)
/// TLS connection state information
#[derive(Debug, Clone)]
pub struct TlsConnectionState {
impl VibeRequest {
    /// Create a new HTTP request
    pub fn new(method: Method, url: impl Into<String>) -> Self {
        Self {
        }
    }
    
    /// Add a cookie to the request
    #[instrument(skip(self))]
    pub fn add_cookie(&mut self, cookie: Cookie) {
        let cookie_value = format!("{}={}", cookie.name, cookie.value);
        match self.header.get_mut("cookie") {
            Some(existing) => {
                existing.push_str(&format!("; {}", cookie_value));
            }
            None => {
                self.header.insert("cookie".to_string(), cookie_value);
            }
        }
    /// Get all cookies from the request
    #[instrument(skip(self))]
    pub fn cookies(&self) -> Vec<Cookie> {
        let mut cookies = Vec::new();
        
        if let Some(cookie_header) = self.header.get("cookie") {
            for cookie_pair in cookie_header.split(';') {
                let cookie_pair = cookie_pair.trim();
                if let Some(equals_pos) = cookie_pair.find('=') {
                    let name = cookie_pair[..equals_pos].trim().to_string();
                    let value = cookie_pair[equals_pos + 1..].trim().to_string();
                    cookies.push(Cookie::new(name, value));
                }
            }
        cookies
    /// Get a specific cookie by name
    #[instrument(skip(self))]
    pub fn cookie(&self, name: &str) -> GlowUpResult<Option<Cookie>> {
        for cookie in self.cookies() {
            if cookie.name == name {
                return Ok(Some(cookie));
            }
        }
        Ok(None)
    /// Parse form data from the request body
    #[instrument(skip(self))]
    pub fn parse_form(&mut self) -> GlowUpResult<()> {
        if let Some(content_type) = self.header.get("content-type") {
            if content_type.starts_with("application/x-www-form-urlencoded") {
                let body_str = String::from_utf8(self.body.clone())
                    .map_err(|e| GlowUpError::parse_error(format!("Invalid UTF-8 in form data: {}", e)))?;
                
                self.post_form = self.parse_form_data(&body_str);
            }
        }
        
        // Also parse query parameters into form
        if let Some(query_start) = self.url.find('?') {
            let query = &self.url[query_start + 1..];
            self.form = self.parse_form_data(query);
        Ok(())
    /// Parse multipart form data
    #[instrument(skip(self))]
    pub fn parse_multipart_form(&mut self, max_memory: i64) -> GlowUpResult<()> {
        // This is a simplified implementation
        // In a real implementation, you'd use a proper multipart parser
        if let Some(content_type) = self.header.get("content-type") {
            if content_type.starts_with("multipart/form-data") {
                debug!("Parsing multipart form data (simplified implementation)");
                // For now, return empty multipart form
                self.multipart_form = Some(MultipartForm {
                });
            }
        }
        Ok(())
    /// Get form value by key
    pub fn form_value(&self, key: &str) -> String {
        self.form.get(key)
            .and_then(|values| values.first())
            .cloned()
            .unwrap_or_default()
    /// Get POST form value by key
    pub fn post_form_value(&self, key: &str) -> String {
        self.post_form.get(key)
            .and_then(|values| values.first())
            .cloned()
            .unwrap_or_default()
    /// Get form file
    pub fn form_file(&self, key: &str) -> GlowUpResult<Option<FormFile>> {
        if let Some(multipart) = &self.multipart_form {
            if let Some(files) = multipart.files.get(key) {
                return Ok(files.first().cloned());
            }
        }
        Ok(None)
    /// Create a new request with context
    pub fn with_context(&self, ctx: VibeContext) -> Self {
        let mut new_req = self.clone();
        new_req.context = Some(ctx);
        new_req
    /// Get basic authentication credentials
    #[instrument(skip(self))]
    pub fn basic_auth(&self) -> (Option<String>, Option<String>, bool) {
        if let Some(auth_header) = self.header.get("authorization") {
            if let Some(basic_part) = auth_header.strip_prefix("Basic ") {
                if let Ok(decoded) = base64::decode(basic_part) {
                    if let Ok(auth_str) = String::from_utf8(decoded) {
                        if let Some(colon_pos) = auth_str.find(':') {
                            let username = auth_str[..colon_pos].to_string();
                            let password = auth_str[colon_pos + 1..].to_string();
                            return (Some(username), Some(password), true);
                        }
                    }
                }
            }
        }
        (None, None, false)
    /// Parse JSON from request body
    #[instrument(skip(self))]
    pub fn get_json<T: serde::de::DeserializeOwned>(&self) -> GlowUpResult<T> {
        let json_str = String::from_utf8(self.body.clone())
            .map_err(|e| GlowUpError::parse_error(format!("Invalid UTF-8 in JSON body: {}", e)))?;
        
        serde_json::from_str(&json_str)
            .map_err(|e| GlowUpError::json(e.to_string()))
    /// Get path parameter by name (for routing)
    pub fn path_param(&self, name: &str) -> String {
        self.path_params.get(name).cloned().unwrap_or_default()
    /// Helper function to parse form data
    fn parse_form_data(&self, data: &str) -> HashMap<String, Vec<String>> {
        let mut form_data = HashMap::new();
        
        for pair in data.split('&') {
            if let Some(equals_pos) = pair.find('=') {
                let key = urlencoding::decode(&pair[..equals_pos])
                    .unwrap_or_default()
                    .to_string();
                let value = urlencoding::decode(&pair[equals_pos + 1..])
                    .unwrap_or_default()
                    .to_string();
                
                form_data.entry(key).or_insert_with(Vec::new).push(value);
            } else {
                let key = urlencoding::decode(pair)
                    .unwrap_or_default()
                    .to_string();
                form_data.entry(key).or_insert_with(Vec::new).push(String::new());
            }
        }
        
        form_data
    }
}

// Add some external dependencies that would be needed
mod base64 {
    pub fn decode(input: &str) -> crate::error::Result<()> {
        // Simplified base64 decode - in real implementation use base64 crate
        Ok(input.as_bytes().to_vec())
    }
}

mod urlencoding {
    pub fn decode(input: &str) -> crate::error::Result<()> {
        // Simplified URL decode - in real implementation use percent-encoding crate
        Ok(std::borrow::Cow::Borrowed(input))
    }
}
