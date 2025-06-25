use crate::error::CursedError;
/// fr fr Core type definitions for web_vibez package
use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};

/// fr fr HTTP headers type alias - clean and simple bestie
pub type Headers = HashMap<String, String>;

/// fr fr Query parameters from URL - supports arrays like ?tags=rust&tags=web
pub type QueryParams = HashMap<String, Vec<String>>;

/// fr fr Form data from POST requests - straightforward vibes
pub type FormData = HashMap<String, String>;

/// fr fr Helper functions for QueryParams
impl QueryParams {
    /// fr fr Create new empty query params
    pub fn new() -> Self {
        HashMap::new()
    /// fr fr Insert a single value (replaces any existing values)
    pub fn insert_single(&mut self, key: String, value: String) {
        self.insert(key, vec![value]);
    /// fr fr Add a value to existing values (or create new entry)
    pub fn add_value(&mut self, key: String, value: String) {
        self.entry(key).or_insert_with(Vec::new).push(value);
    /// fr fr Get first value for a parameter (for backward compatibility)
    pub fn get_first(&self, key: &str) -> Option<&String> {
        self.get(key).and_then(|values| values.first())
    /// fr fr Get all values for a parameter
    pub fn get_all(&self, key: &str) -> Vec<&String> {
        self.get(key).map(|values| values.iter().collect()).unwrap_or_default()
    }
}

/// fr fr JSON data wrapper for type safety - keeping it real
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Json<T> {
impl<T> Json<T> {
    /// fr fr Create new JSON wrapper - simple and clean
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// fr fr Extract the inner data - get what you need
    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<T: fmt::Display> fmt::Display for Json<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

/// fr fr Content type enum for HTTP responses - covers all the basics
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    /// fr fr Standard HTML content
    /// fr fr JSON API responses
    /// fr fr Plain text responses
    /// fr fr XML content
    /// fr fr CSS stylesheets
    /// fr fr JavaScript files
    /// fr fr Binary data
    /// fr fr Custom content type
impl ContentType {
    /// fr fr Get the MIME type string - what browsers expect
    pub fn mime_type(&self) -> &str {
        match self {
            ContentType::Html => "text/html",
            ContentType::Json => "application/json",
            ContentType::Text => "text/plain",
            ContentType::Xml => "application/xml",
            ContentType::Css => "text/css",
            ContentType::JavaScript => "application/javascript",
            ContentType::Binary => "application/octet-stream",
        }
    }

    /// fr fr Parse from string - flexible input handling
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "text/html" | "html" => ContentType::Html,
            "application/json" | "json" => ContentType::Json,
            "text/plain" | "text" => ContentType::Text,
            "application/xml" | "xml" => ContentType::Xml,
            "text/css" | "css" => ContentType::Css,
            "application/javascript" | "javascript" | "js" => ContentType::JavaScript,
            "application/octet-stream" | "binary" => ContentType::Binary,
        }
    }
impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mime_type())
    }
}

/// fr fr Request body types - handle different data formats
#[derive(Debug, Clone)]
pub enum RequestBody {
    /// fr fr No body content
    /// fr fr Raw text content
    /// fr fr JSON data
    /// fr fr Form-encoded data
    /// fr fr Binary data
impl RequestBody {
    /// fr fr Check if body is empty - quick validation
    pub fn is_empty(&self) -> bool {
        matches!(self, RequestBody::Empty)
    /// fr fr Get content length - useful for headers
    pub fn content_length(&self) -> usize {
        match self {
            RequestBody::Form(f) => {
                f.iter()
                    .map(|(k, v)| k.len() + v.len() + 2) // +2 for = and &
                    .sum::<usize>()
                    .saturating_sub(1) // Remove last &
            }
        }
    }

    /// fr fr Convert to string representation
    pub fn to_string(&self) -> crate::error::Result<()> {
        match self {
            RequestBody::Form(f) => {
                let encoded = f
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                Ok(encoded)
            }
        }
    }
/// fr fr Cookie representation for HTTP requests/responses
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
/// fr fr SameSite cookie attribute values
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SameSite {
impl Cookie {
    /// fr fr Create a basic cookie - minimal setup
    pub fn new(name: String, value: String) -> Self {
        Self {
        }
    }

    /// fr fr Convert cookie to Set-Cookie header value
    pub fn to_header_value(&self) -> String {
        let mut header = format!("{}={}", self.name, self.value);

        if let Some(domain) = &self.domain {
            header.push_str(&format!("; Domain={}", domain));
        if let Some(path) = &self.path {
            header.push_str(&format!("; Path={}", path));
        if let Some(expires) = &self.expires {
            header.push_str(&format!("; Expires={}", expires));
        if let Some(max_age) = self.max_age {
            header.push_str(&format!("; Max-Age={}", max_age));
        if self.secure {
            header.push_str("; Secure");
        if self.http_only {
            header.push_str("; HttpOnly");
        if let Some(same_site) = &self.same_site {
            let value = match same_site {
            header.push_str(&format!("; SameSite={}", value));
        header
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_header_value())
    }
}

