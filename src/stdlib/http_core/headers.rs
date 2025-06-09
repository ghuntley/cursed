//! HTTP Header Management for CURSED web_vibez
//!
//! Case-insensitive header storage, parsing, and manipulation.

use std::collections::HashMap;
use std::fmt;

use crate::stdlib::http_core::{HttpError, HttpResult};

/// Case-insensitive header name wrapper
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderName(String);

impl HeaderName {
    /// Create a new header name
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self(name.into().to_lowercase())
    }

    /// Get the original case header name
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate header name according to RFC 7230
    pub fn validate(&self) -> HttpResult<()> {
        if self.0.is_empty() {
            return Err(HttpError::InvalidHeader("Empty header name".to_string()));
        }

        // Header names must be tokens (RFC 7230)
        for ch in self.0.chars() {
            if !ch.is_ascii() || ch.is_ascii_control() || "()<>@,;:\\\"/[]?={} \t".contains(ch) {
                return Err(HttpError::InvalidHeader(
                    format!("Invalid character '{}' in header name", ch)
                ));
            }
        }

        Ok(())
    }
}

impl From<&str> for HeaderName {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for HeaderName {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// HTTP header value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderValue(String);

impl HeaderValue {
    /// Create a new header value
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into().trim().to_string())
    }

    /// Get the header value as string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validate header value according to RFC 7230
    pub fn validate(&self) -> HttpResult<()> {
        // Header values must be visible VCHAR, WSP, or obs-text
        for ch in self.0.chars() {
            if ch.is_ascii_control() && ch != '\t' {
                return Err(HttpError::InvalidHeader(
                    format!("Invalid control character in header value")
                ));
            }
        }

        Ok(())
    }

    /// Check if value contains a specific substring (case-insensitive)
    pub fn contains(&self, needle: &str) -> bool {
        self.0.to_lowercase().contains(&needle.to_lowercase())
    }

    /// Split value by delimiter and return trimmed parts
    pub fn split(&self, delimiter: char) -> Vec<String> {
        self.0
            .split(delimiter)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Parse quality values (q=0.8 format)
    pub fn parse_quality(&self) -> f32 {
        if let Some(q_pos) = self.0.find("q=") {
            if let Ok(quality) = self.0[q_pos + 2..].split(',').next()
                .unwrap_or("1.0")
                .trim()
                .parse::<f32>() {
                return quality.clamp(0.0, 1.0);
            }
        }
        1.0
    }
}

impl From<&str> for HeaderValue {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for HeaderValue {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Case-insensitive header map
#[derive(Debug, Clone)]
pub struct HeaderMap {
    headers: HashMap<HeaderName, Vec<HeaderValue>>,
    original_names: HashMap<HeaderName, String>, // Preserve original case
}

impl HeaderMap {
    /// Create a new empty header map
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
            original_names: HashMap::new(),
        }
    }

    /// Insert a header (replaces existing)
    pub fn insert<K, V>(&mut self, name: K, value: V) -> Option<Vec<HeaderValue>>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let name_str = name.into();
        let header_name = HeaderName::new(name_str.to_lowercase());
        let header_value = HeaderValue::new(value.into());

        // Preserve original case
        self.original_names.insert(header_name.clone(), name_str);

        self.headers.insert(header_name, vec![header_value])
    }

    /// Append a header value (adds to existing)
    pub fn append<K, V>(&mut self, name: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        let name_str = name.into();
        let header_name = HeaderName::new(name_str.to_lowercase());
        let header_value = HeaderValue::new(value.into());

        // Preserve original case
        self.original_names.insert(header_name.clone(), name_str);

        self.headers
            .entry(header_name)
            .or_insert_with(Vec::new)
            .push(header_value);
    }

    /// Get the first header value
    pub fn get<K>(&self, name: K) -> Option<&String>
    where
        K: Into<String>,
    {
        let header_name = HeaderName::new(name.into());
        self.headers
            .get(&header_name)
            .and_then(|values| values.first())
            .map(|v| &v.0)
    }

    /// Get all header values
    pub fn get_all<K>(&self, name: K) -> Vec<&str>
    where
        K: Into<String>,
    {
        let header_name = HeaderName::new(name.into());
        self.headers
            .get(&header_name)
            .map(|values| values.iter().map(|v| v.as_str()).collect())
            .unwrap_or_default()
    }

    /// Check if header exists
    pub fn contains_key<K>(&self, name: K) -> bool
    where
        K: Into<String>,
    {
        let header_name = HeaderName::new(name.into());
        self.headers.contains_key(&header_name)
    }

    /// Remove a header
    pub fn remove<K>(&mut self, name: K) -> Option<Vec<HeaderValue>>
    where
        K: Into<String>,
    {
        let header_name = HeaderName::new(name.into());
        self.original_names.remove(&header_name);
        self.headers.remove(&header_name)
    }

    /// Get header names (with original case preserved)
    pub fn keys(&self) -> Vec<&String> {
        self.original_names.values().collect()
    }

    /// Get all header entries
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<HeaderValue>)> {
        self.headers.iter().map(|(name, values)| {
            let original_name = self.original_names.get(name).unwrap();
            (original_name, values)
        })
    }

    /// Get number of headers
    pub fn len(&self) -> usize {
        self.headers.len()
    }

    /// Check if header map is empty
    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }

    /// Clear all headers
    pub fn clear(&mut self) {
        self.headers.clear();
        self.original_names.clear();
    }

    /// Merge another header map into this one
    pub fn merge(&mut self, other: HeaderMap) {
        for (name, values) in other.headers {
            let original_name = other.original_names.get(&name).unwrap();
            self.original_names.insert(name.clone(), original_name.clone());
            
            for value in values {
                self.headers
                    .entry(name.clone())
                    .or_insert_with(Vec::new)
                    .push(value);
            }
        }
    }

    /// Get content type
    pub fn content_type(&self) -> Option<String> {
        self.get("Content-Type").cloned()
    }

    /// Get content length
    pub fn content_length(&self) -> Option<usize> {
        self.get("Content-Length")?.parse().ok()
    }

    /// Get authorization header
    pub fn authorization(&self) -> Option<String> {
        self.get("Authorization").cloned()
    }

    /// Get user agent
    pub fn user_agent(&self) -> Option<String> {
        self.get("User-Agent").cloned()
    }

    /// Get host
    pub fn host(&self) -> Option<String> {
        self.get("Host").cloned()
    }

    /// Check if connection should be kept alive
    pub fn keep_alive(&self) -> bool {
        if let Some(connection) = self.get("Connection") {
            connection.to_lowercase() == "keep-alive"
        } else {
            false
        }
    }

    /// Get accept header values with quality parsing
    pub fn accept(&self) -> Vec<(String, f32)> {
        if let Some(accept) = self.get("Accept") {
            accept
                .split(',')
                .map(|part| {
                    let trimmed = part.trim();
                    if let Some(semicolon_pos) = trimmed.find(';') {
                        let media_type = trimmed[..semicolon_pos].trim().to_string();
                        let params = &trimmed[semicolon_pos + 1..];
                        let quality = HeaderValue::new(params).parse_quality();
                        (media_type, quality)
                    } else {
                        (trimmed.to_string(), 1.0)
                    }
                })
                .collect()
        } else {
            vec![("*/*".to_string(), 1.0)]
        }
    }

    /// Get accept encoding header values
    pub fn accept_encoding(&self) -> Vec<String> {
        if let Some(encoding) = self.get("Accept-Encoding") {
            encoding
                .split(',')
                .map(|s| s.trim().split(';').next().unwrap_or("").to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get accept language header values
    pub fn accept_language(&self) -> Vec<(String, f32)> {
        if let Some(language) = self.get("Accept-Language") {
            language
                .split(',')
                .map(|part| {
                    let trimmed = part.trim();
                    if let Some(semicolon_pos) = trimmed.find(';') {
                        let lang = trimmed[..semicolon_pos].trim().to_string();
                        let params = &trimmed[semicolon_pos + 1..];
                        let quality = HeaderValue::new(params).parse_quality();
                        (lang, quality)
                    } else {
                        (trimmed.to_string(), 1.0)
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Validate all headers
    pub fn validate(&self) -> HttpResult<()> {
        for (name, values) in &self.headers {
            name.validate()?;
            for value in values {
                value.validate()?;
            }
        }
        Ok(())
    }
}

impl Default for HeaderMap {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for HeaderMap {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut items = Vec::new();
        for (name, values) in self.headers {
            let original_name = self.original_names.get(&name).unwrap();
            for value in values {
                items.push((original_name.clone(), value.0));
            }
        }
        items.into_iter()
    }
}

/// Trait for types that can provide headers
pub trait Headers {
    fn headers(&self) -> &HeaderMap;
    fn headers_mut(&mut self) -> &mut HeaderMap;

    fn header<K>(&self, name: K) -> Option<&String>
    where
        K: Into<String>,
    {
        self.headers().get(name)
    }

    fn set_header<K, V>(&mut self, name: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers_mut().insert(name, value);
    }

    fn add_header<K, V>(&mut self, name: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers_mut().append(name, value);
    }

    fn remove_header<K>(&mut self, name: K) -> Option<Vec<HeaderValue>>
    where
        K: Into<String>,
    {
        self.headers_mut().remove(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_name_case_insensitive() {
        let name1 = HeaderName::new("Content-Type");
        let name2 = HeaderName::new("content-type");
        let name3 = HeaderName::new("CONTENT-TYPE");

        assert_eq!(name1, name2);
        assert_eq!(name2, name3);
    }

    #[test]
    fn test_header_map_case_insensitive() {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json");
        
        assert_eq!(headers.get("content-type"), Some(&"application/json".to_string()));
        assert_eq!(headers.get("CONTENT-TYPE"), Some(&"application/json".to_string()));
        assert_eq!(headers.get("Content-Type"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_header_map_multiple_values() {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "text/html");
        headers.append("Accept", "application/json");
        
        let values = headers.get_all("Accept");
        assert_eq!(values.len(), 2);
        assert!(values.contains(&"text/html"));
        assert!(values.contains(&"application/json"));
    }

    #[test]
    fn test_accept_header_parsing() {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "text/html,application/xml;q=0.9,*/*;q=0.8");
        
        let accept_values = headers.accept();
        assert_eq!(accept_values.len(), 3);
        assert_eq!(accept_values[0], ("text/html".to_string(), 1.0));
        assert_eq!(accept_values[1], ("application/xml".to_string(), 0.9));
        assert_eq!(accept_values[2], ("*/*".to_string(), 0.8));
    }

    #[test]
    fn test_header_validation() {
        let mut headers = HeaderMap::new();
        headers.insert("Valid-Header", "valid value");
        assert!(headers.validate().is_ok());

        // Test will be extended when validation is more strict
    }

    #[test]
    fn test_quality_value_parsing() {
        let value = HeaderValue::new("application/json;q=0.8");
        assert_eq!(value.parse_quality(), 0.8);

        let value = HeaderValue::new("text/html");
        assert_eq!(value.parse_quality(), 1.0);

        let value = HeaderValue::new("*/*;q=0.1");
        assert_eq!(value.parse_quality(), 0.1);
    }
}
