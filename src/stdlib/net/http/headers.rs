/// HTTP headers implementation for CURSED networking
/// 
/// This module provides HTTP header management including header maps,
/// header values, and common header utilities.

use std::collections::HashMap;
use std::fmt;

/// HTTP header map
#[derive(Debug, Clone)]
pub struct HttpHeaders {
impl HttpHeaders {
    /// Create new empty headers
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Set a header value
    pub fn set(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_lowercase(), value.to_string());
    /// Get a header value
    pub fn get(&self, name: &str) -> Option<&String> {
        self.headers.get(&name.to_lowercase())
    /// Remove a header
    pub fn remove(&mut self, name: &str) -> Option<String> {
        self.headers.remove(&name.to_lowercase())
    /// Check if header exists
    pub fn contains(&self, name: &str) -> bool {
        self.headers.contains_key(&name.to_lowercase())
    /// Get all header names
    pub fn names(&self) -> Vec<&String> {
        self.headers.keys().collect()
    /// Get all header values
    pub fn values(&self) -> Vec<&String> {
        self.headers.values().collect()
    /// Clear all headers
    pub fn clear(&mut self) {
        self.headers.clear();
    /// Get number of headers
    pub fn len(&self) -> usize {
        self.headers.len()
    /// Check if headers is empty
    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    /// Merge headers from another HttpHeaders
    pub fn extend(&mut self, other: &HttpHeaders) {
        for (name, value) in &other.headers {
            self.headers.insert(name.clone(), value.clone());
        }
    }
    
    /// Get iterator over headers
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.headers.iter()
    }
}

impl Default for HttpHeaders {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for HttpHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, value) in &self.headers {
            writeln!(f, "{}: {}", name, value)?;
        }
        Ok(())
    }
}

/// HTTP header value wrapper
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderValue {
impl HeaderValue {
    /// Create new header value
    pub fn from_str(value: &str) -> Self {
        Self {
        }
    }
    
    /// Get value as string
    pub fn as_str(&self) -> &str {
        &self.value
    /// Get value as bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    /// Check if value is empty
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    /// Get length of value
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for HeaderValue {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<String> for HeaderValue {
    fn from(value: String) -> Self {
        Self { value }
    }
impl From<HeaderValue> for String {
    fn from(header: HeaderValue) -> Self {
        header.value
    }
}

/// Type alias for header map using HeaderValue
pub type HeaderMap = HashMap<String, HeaderValue>;

/// Common HTTP headers
pub mod header {
    pub const ACCEPT: &str = "accept";
    pub const ACCEPT_CHARSET: &str = "accept-charset";
    pub const ACCEPT_ENCODING: &str = "accept-encoding";
    pub const ACCEPT_LANGUAGE: &str = "accept-language";
    pub const AUTHORIZATION: &str = "authorization";
    pub const CACHE_CONTROL: &str = "cache-control";
    pub const CONNECTION: &str = "connection";
    pub const CONTENT_ENCODING: &str = "content-encoding";
    pub const CONTENT_LENGTH: &str = "content-length";
    pub const CONTENT_TYPE: &str = "content-type";
    pub const COOKIE: &str = "cookie";
    pub const DATE: &str = "date";
    pub const ETAG: &str = "etag";
    pub const EXPIRES: &str = "expires";
    pub const HOST: &str = "host";
    pub const IF_MODIFIED_SINCE: &str = "if-modified-since";
    pub const IF_NONE_MATCH: &str = "if-none-match";
    pub const LAST_MODIFIED: &str = "last-modified";
    pub const LOCATION: &str = "location";
    pub const REFERER: &str = "referer";
    pub const SERVER: &str = "server";
    pub const SET_COOKIE: &str = "set-cookie";
    pub const TRANSFER_ENCODING: &str = "transfer-encoding";
    pub const USER_AGENT: &str = "user-agent";
    pub const WWW_AUTHENTICATE: &str = "www-authenticate";
/// HTTP header utilities
pub mod utils {
    use super::*;
    
    /// Parse Content-Type header
    pub fn parse_content_type(value: &str) -> (String, HashMap<String, String>) {
        let mut parts = value.split(';');
        let media_type = parts.next().unwrap_or("").trim().to_lowercase();
        
        let mut params = HashMap::new();
        for part in parts {
            if let Some(eq_pos) = part.find('=') {
                let key = part[..eq_pos].trim().to_lowercase();
                let value = part[eq_pos + 1..].trim().trim_matches('"').to_string();
                params.insert(key, value);
            }
        }
        
        (media_type, params)
    /// Format Content-Type header
    pub fn format_content_type(media_type: &str, params: &HashMap<String, String>) -> String {
        let mut result = media_type.to_string();
        for (key, value) in params {
            result.push_str(&format!("; {}={}", key, value));
        }
        result
    /// Parse Cache-Control header
    pub fn parse_cache_control(value: &str) -> HashMap<String, Option<String>> {
        let mut directives = HashMap::new();
        
        for directive in value.split(',') {
            let directive = directive.trim();
            if let Some(eq_pos) = directive.find('=') {
                let key = directive[..eq_pos].trim().to_lowercase();
                let value = directive[eq_pos + 1..].trim().trim_matches('"').to_string();
                directives.insert(key, Some(value));
            } else {
                directives.insert(directive.to_lowercase(), None);
            }
        }
        
        directives
    /// Check if header value contains a token
    pub fn contains_token(value: &str, token: &str) -> bool {
        value.split(',')
            .any(|part| part.trim().eq_ignore_ascii_case(token))
    /// Parse quality values (q-values)
    pub fn parse_quality_values(value: &str) -> Vec<(String, f32)> {
        let mut items = Vec::new();
        
        for item in value.split(',') {
            let item = item.trim();
            if let Some(semicolon_pos) = item.find(';') {
                let media_type = item[..semicolon_pos].trim().to_string();
                let params = &item[semicolon_pos + 1..];
                
                let quality = if let Some(q_pos) = params.find("q=") {
                    params[q_pos + 2..].trim().parse().unwrap_or(1.0)
                } else {
                    1.0
                
                items.push((media_type, quality));
            } else {
                items.push((item.to_string(), 1.0));
            }
        }
        
        // Sort by quality value (highest first)
        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        items
    /// Validate header name according to HTTP specification
    pub fn is_valid_header_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        name.chars().all(|c| {
                ' ' | '\t' | '(' | ')' | '<' | '>' | '@' | ',' | ';' | ':' | 
                '\\' | '"' | '/' | '[' | ']' | '?' | '=' | '{' | '}'
            )
        })
    /// Validate header value according to HTTP specification
    pub fn is_valid_header_value(value: &str) -> bool {
        value.chars().all(|c| {
            c.is_ascii() && (c == '\t' || (c >= ' ' && c != '\u{7f}'))
        })
    /// Normalize header name (convert to lowercase)
    pub fn normalize_header_name(name: &str) -> String {
        name.to_lowercase()
    /// Get standard header capitalization
    pub fn canonical_header_name(name: &str) -> String {
        name.split('-')
            .map(|part| {
                let mut chars: Vec<char> = part.chars().collect();
                if !chars.is_empty() {
                    chars[0] = chars[0].to_ascii_uppercase();
                    for c in chars.iter_mut().skip(1) {
                        *c = c.to_ascii_lowercase();
                    }
                }
                chars.into_iter().collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("-")
    }
}

