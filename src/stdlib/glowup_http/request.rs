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
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl FromStr for Method {
    type Err = GlowUpError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "PATCH" => Ok(Method::PATCH),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "CONNECT" => Ok(Method::CONNECT),
            _ => Err(GlowUpError::parse_error(format!("Unknown HTTP method: {}", s))),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::TRACE => "TRACE",
            Method::CONNECT => "CONNECT",
        };
        write!(f, "{}", method_str)
    }
}

/// HTTP version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    Http1_0,
    Http1_1,
    Http2_0,
}

impl FromStr for HttpVersion {
    type Err = GlowUpError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(HttpVersion::Http1_0),
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2.0" | "HTTP/2" => Ok(HttpVersion::Http2_0),
            _ => Err(GlowUpError::parse_error(format!("Unsupported HTTP version: {}", s))),
        }
    }
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version_str = match self {
            HttpVersion::Http1_0 => "HTTP/1.0",
            HttpVersion::Http1_1 => "HTTP/1.1",
            HttpVersion::Http2_0 => "HTTP/2.0",
        };
        write!(f, "{}", version_str)
    }
}

/// Header map type
pub type HeaderMap = HashMap<String, String>;

/// Cookie representation
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<std::time::SystemTime>,
    pub max_age: Option<u64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

#[derive(Debug, Clone)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Cookie {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            domain: None,
            path: None,
            expires: None,
            max_age: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }
    
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
    
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }
    
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }
}

/// Multipart form data
#[derive(Debug, Clone)]
pub struct MultipartForm {
    pub fields: HashMap<String, Vec<String>>,
    pub files: HashMap<String, Vec<FormFile>>,
}

#[derive(Debug, Clone)]
pub struct FormFile {
    pub filename: String,
    pub content_type: String,
    pub size: usize,
    pub data: Vec<u8>,
}

/// Context type for request processing
pub type VibeContext = Arc<dyn std::any::Any + Send + Sync>;

/// Represents an HTTP request received by a server or to be sent by a client
/// This follows the CURSED spec's `VibeRequest` naming
#[derive(Debug, Clone)]
pub struct VibeRequest {
    /// HTTP method
    pub method: Method,
    /// Request URL/path
    pub url: String,
    /// HTTP protocol version
    pub proto: HttpVersion,
    /// Request headers
    pub header: HeaderMap,
    /// Request body
    pub body: Vec<u8>,
    /// Content length
    pub content_length: i64,
    /// Transfer encoding
    pub transfer_encoding: Vec<String>,
    /// Host header value
    pub host: String,
    /// Parsed form data
    pub form: HashMap<String, Vec<String>>,
    /// Parsed POST form data
    pub post_form: HashMap<String, Vec<String>>,
    /// Multipart form data
    pub multipart_form: Option<MultipartForm>,
    /// Trailer headers
    pub trailer: HeaderMap,
    /// Remote address
    pub remote_addr: String,
    /// Request URI (raw)
    pub request_uri: String,
    /// TLS connection state
    pub tls: Option<TlsConnectionState>,
    /// Request context
    pub context: Option<VibeContext>,
    /// Path parameters (for routing)
    pub path_params: HashMap<String, String>,
}

/// TLS connection state information
#[derive(Debug, Clone)]
pub struct TlsConnectionState {
    pub version: String,
    pub cipher_suite: String,
    pub server_name: Option<String>,
    pub peer_certificates: Vec<Vec<u8>>,
}

impl VibeRequest {
    /// Create a new HTTP request
    pub fn new(method: Method, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            proto: HttpVersion::Http1_1,
            header: HashMap::new(),
            body: Vec::new(),
            content_length: 0,
            transfer_encoding: Vec::new(),
            host: String::new(),
            form: HashMap::new(),
            post_form: HashMap::new(),
            multipart_form: None,
            trailer: HashMap::new(),
            remote_addr: String::new(),
            request_uri: String::new(),
            tls: None,
            context: None,
            path_params: HashMap::new(),
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
        }
        
        cookies
    }
    
    /// Get a specific cookie by name
    #[instrument(skip(self))]
    pub fn cookie(&self, name: &str) -> GlowUpResult<Option<Cookie>> {
        for cookie in self.cookies() {
            if cookie.name == name {
                return Ok(Some(cookie));
            }
        }
        Ok(None)
    }
    
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
        }
        
        Ok(())
    }
    
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
                    fields: HashMap::new(),
                    files: HashMap::new(),
                });
            }
        }
        Ok(())
    }
    
    /// Get form value by key
    pub fn form_value(&self, key: &str) -> String {
        self.form.get(key)
            .and_then(|values| values.first())
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get POST form value by key
    pub fn post_form_value(&self, key: &str) -> String {
        self.post_form.get(key)
            .and_then(|values| values.first())
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get form file
    pub fn form_file(&self, key: &str) -> GlowUpResult<Option<FormFile>> {
        if let Some(multipart) = &self.multipart_form {
            if let Some(files) = multipart.files.get(key) {
                return Ok(files.first().cloned());
            }
        }
        Ok(None)
    }
    
    /// Create a new request with context
    pub fn with_context(&self, ctx: VibeContext) -> Self {
        let mut new_req = self.clone();
        new_req.context = Some(ctx);
        new_req
    }
    
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
    }
    
    /// Parse JSON from request body
    #[instrument(skip(self))]
    pub fn get_json<T: serde::de::DeserializeOwned>(&self) -> GlowUpResult<T> {
        let json_str = String::from_utf8(self.body.clone())
            .map_err(|e| GlowUpError::parse_error(format!("Invalid UTF-8 in JSON body: {}", e)))?;
        
        serde_json::from_str(&json_str)
            .map_err(|e| GlowUpError::json(e.to_string()))
    }
    
    /// Get path parameter by name (for routing)
    pub fn path_param(&self, name: &str) -> String {
        self.path_params.get(name).cloned().unwrap_or_default()
    }
    
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
