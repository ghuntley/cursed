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
    }

    /// fr fr Insert a single value (replaces any existing values)
    pub fn insert_single(&mut self, key: String, value: String) {
        self.insert(key, vec![value]);
    }

    /// fr fr Add a value to existing values (or create new entry)
    pub fn add_value(&mut self, key: String, value: String) {
        self.entry(key).or_insert_with(Vec::new).push(value);
    }

    /// fr fr Get first value for a parameter (for backward compatibility)
    pub fn get_first(&self, key: &str) -> Option<&String> {
        self.get(key).and_then(|values| values.first())
    }

    /// fr fr Get all values for a parameter
    pub fn get_all(&self, key: &str) -> Vec<&String> {
        self.get(key).map(|values| values.iter().collect()).unwrap_or_default()
    }
}

/// fr fr JSON data wrapper for type safety - keeping it real
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Json<T> {
    pub data: T,
}

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
    Html,
    /// fr fr JSON API responses
    Json,
    /// fr fr Plain text responses
    Text,
    /// fr fr XML content
    Xml,
    /// fr fr CSS stylesheets
    Css,
    /// fr fr JavaScript files
    JavaScript,
    /// fr fr Binary data
    Binary,
    /// fr fr Custom content type
    Custom(String),
}

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
            ContentType::Custom(mime) => mime,
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
            _ => ContentType::Custom(s.to_string()),
        }
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
    Empty,
    /// fr fr Raw text content
    Text(String),
    /// fr fr JSON data
    Json(serde_json::Value),
    /// fr fr Form-encoded data
    Form(FormData),
    /// fr fr Binary data
    Binary(Vec<u8>),
}

impl RequestBody {
    /// fr fr Check if body is empty - quick validation
    pub fn is_empty(&self) -> bool {
        matches!(self, RequestBody::Empty)
    }

    /// fr fr Get content length - useful for headers
    pub fn content_length(&self) -> usize {
        match self {
            RequestBody::Empty => 0,
            RequestBody::Text(s) => s.len(),
            RequestBody::Json(v) => serde_json::to_string(v).unwrap_or_default().len(),
            RequestBody::Form(f) => {
                f.iter()
                    .map(|(k, v)| k.len() + v.len() + 2) // +2 for = and &
                    .sum::<usize>()
                    .saturating_sub(1) // Remove last &
            }
            RequestBody::Binary(b) => b.len(),
        }
    }

    /// fr fr Convert to string representation
    pub fn to_string(&self) -> Result<(), Error>> {
        match self {
            RequestBody::Empty => Ok(String::new()),
            RequestBody::Text(s) => Ok(s.clone()),
            RequestBody::Json(v) => Ok(serde_json::to_string(v)?),
            RequestBody::Form(f) => {
                let encoded = f
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                Ok(encoded)
            }
            RequestBody::Binary(b) => Ok(String::from_utf8_lossy(b).to_string()),
        }
    }
}

/// fr fr Cookie representation for HTTP requests/responses
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<String>,
    pub max_age: Option<i64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

/// fr fr SameSite cookie attribute values
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Cookie {
    /// fr fr Create a basic cookie - minimal setup
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            domain: None,
            path: None,
            expires: None,
            max_age: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    /// fr fr Convert cookie to Set-Cookie header value
    pub fn to_header_value(&self) -> String {
        let mut header = format!("{}={}", self.name, self.value);

        if let Some(domain) = &self.domain {
            header.push_str(&format!("; Domain={}", domain));
        }

        if let Some(path) = &self.path {
            header.push_str(&format!("; Path={}", path));
        }

        if let Some(expires) = &self.expires {
            header.push_str(&format!("; Expires={}", expires));
        }

        if let Some(max_age) = self.max_age {
            header.push_str(&format!("; Max-Age={}", max_age));
        }

        if self.secure {
            header.push_str("; Secure");
        }

        if self.http_only {
            header.push_str("; HttpOnly");
        }

        if let Some(same_site) = &self.same_site {
            let value = match same_site {
                SameSite::Strict => "Strict",
                SameSite::Lax => "Lax",
                SameSite::None => "None",
            };
            header.push_str(&format!("; SameSite={}", value));
        }

        header
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_header_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_mime_types() {
        assert_eq!(ContentType::Html.mime_type(), "text/html");
        assert_eq!(ContentType::Json.mime_type(), "application/json");
        assert_eq!(ContentType::Text.mime_type(), "text/plain");
    }

    #[test]
    fn test_content_type_from_str() {
        assert_eq!(ContentType::from_str("json"), ContentType::Json);
        assert_eq!(ContentType::from_str("application/json"), ContentType::Json);
        assert_eq!(ContentType::from_str("text/html"), ContentType::Html);
    }

    #[test]
    fn test_request_body_content_length() {
        assert_eq!(RequestBody::Empty.content_length(), 0);
        assert_eq!(RequestBody::Text("hello".to_string()).content_length(), 5);
        
        let mut form = FormData::new();
        form.insert("key".to_string(), "value".to_string());
        assert_eq!(RequestBody::Form(form).content_length(), 9); // key=value
    }

    #[test]
    fn test_cookie_header_value() {
        let cookie = Cookie::new("session".to_string(), "abc123".to_string());
        assert_eq!(cookie.to_header_value(), "session=abc123");
        
        let mut cookie = Cookie::new("user".to_string(), "john".to_string());
        cookie.secure = true;
        cookie.http_only = true;
        assert_eq!(cookie.to_header_value(), "user=john; Secure; HttpOnly");
    }
}
