//! URL Processing and Query Parameter Handling for CURSED web_vibez
//!
//! Comprehensive URL parsing, validation, and manipulation.

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::stdlib::http_core::{HttpError, HttpResult};

/// URL query parameters
#[derive(Debug, Clone, PartialEq)]
pub struct QueryParams {
    params: HashMap<String, Vec<String>>,
}

impl QueryParams {
    /// Create new empty query parameters
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Parse query parameters from string
    pub fn parse(query_string: &str) -> Self {
        let mut params = HashMap::new();

        if query_string.is_empty() {
            return Self { params };
        }

        for pair in query_string.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = urlencoding::decode(&pair[..eq_pos])
                    .unwrap_or_else(|_| pair[..eq_pos].into())
                    .to_string();
                let value = urlencoding::decode(&pair[eq_pos + 1..])
                    .unwrap_or_else(|_| pair[eq_pos + 1..].into())
                    .to_string();

                params.entry(key).or_insert_with(Vec::new).push(value);
            } else if !pair.is_empty() {
                let key = urlencoding::decode(pair)
                    .unwrap_or_else(|_| pair.into())
                    .to_string();
                params.entry(key).or_insert_with(Vec::new).push(String::new());
            }
        }

        Self { params }
    }

    /// Get first value for parameter
    pub fn get(&self, name: &str) -> Option<&str> {
        self.params.get(name)?.first().map(|s| s.as_str())
    }

    /// Get all values for parameter
    pub fn get_all(&self, name: &str) -> Option<&Vec<String>> {
        self.params.get(name)
    }

    /// Insert parameter (replaces existing)
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.params.insert(key.into(), vec![value.into()]);
    }

    /// Add parameter value (appends to existing)
    pub fn add<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.params
            .entry(key.into())
            .or_insert_with(Vec::new)
            .push(value.into());
    }

    /// Remove parameter
    pub fn remove(&mut self, key: &str) -> Option<Vec<String>> {
        self.params.remove(key)
    }

    /// Check if parameter exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Get all parameter names
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.params.keys()
    }

    /// Get number of parameters
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Clear all parameters
    pub fn clear(&mut self) {
        self.params.clear();
    }

    /// Convert to query string
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();

        for (key, values) in &self.params {
            let encoded_key = urlencoding::encode(key);
            for value in values {
                if value.is_empty() {
                    parts.push(encoded_key.to_string());
                } else {
                    let encoded_value = urlencoding::encode(value);
                    parts.push(format!("{}={}", encoded_key, encoded_value));
                }
            }
        }

        parts.join("&")
    }

    /// Merge another QueryParams into this one
    pub fn merge(&mut self, other: QueryParams) {
        for (key, values) in other.params {
            for value in values {
                self.add(key.clone(), value);
            }
        }
    }
}

impl Default for QueryParams {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for QueryParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl IntoIterator for QueryParams {
    type Item = (String, Vec<String>);
    type IntoIter = std::collections::hash_map::IntoIter<String, Vec<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.params.into_iter()
    }
}

/// URL path parameters (for route matching)
#[derive(Debug, Clone, PartialEq)]
pub struct PathParams {
    params: HashMap<String, String>,
}

impl PathParams {
    /// Create new empty path parameters
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Get path parameter value
    pub fn get(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(|s| s.as_str())
    }

    /// Insert path parameter
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.params.insert(key.into(), value.into());
    }

    /// Remove path parameter
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.params.remove(key)
    }

    /// Check if parameter exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Get all parameter names
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.params.keys()
    }

    /// Get number of parameters
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Clear all parameters
    pub fn clear(&mut self) {
        self.params.clear();
    }
}

impl Default for PathParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive URL structure
#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    /// URL scheme (http, https, etc.)
    pub scheme: Option<String>,
    /// Host (domain or IP)
    pub host: Option<String>,
    /// Port number
    pub port: Option<u16>,
    /// URL path
    pub path: String,
    /// Query parameters
    pub query_params: QueryParams,
    /// URL fragment
    pub fragment: Option<String>,
    /// Path parameters (for route matching)
    pub path_params: PathParams,
}

impl Url {
    /// Create a new URL
    pub fn new<P: Into<String>>(path: P) -> Self {
        Self {
            scheme: None,
            host: None,
            port: None,
            path: path.into(),
            query_params: QueryParams::new(),
            fragment: None,
            path_params: PathParams::new(),
        }
    }

    /// Parse URL from string
    pub fn parse(url_str: &str) -> HttpResult<Self> {
        if url_str.is_empty() {
            return Err(HttpError::InvalidUrl("Empty URL".to_string()));
        }

        let mut url = Self::new("");
        let mut remaining = url_str;

        // Parse scheme
        if let Some(scheme_end) = remaining.find("://") {
            url.scheme = Some(remaining[..scheme_end].to_string());
            remaining = &remaining[scheme_end + 3..];
        }

        // Parse fragment first (if present)
        if let Some(fragment_start) = remaining.find('#') {
            url.fragment = Some(remaining[fragment_start + 1..].to_string());
            remaining = &remaining[..fragment_start];
        }

        // Parse query parameters
        if let Some(query_start) = remaining.find('?') {
            url.query_params = QueryParams::parse(&remaining[query_start + 1..]);
            remaining = &remaining[..query_start];
        }

        // Parse host and port (if scheme is present)
        if url.scheme.is_some() {
            if let Some(path_start) = remaining.find('/') {
                let host_port = &remaining[..path_start];
                url.parse_host_port(host_port)?;
                url.path = remaining[path_start..].to_string();
            } else {
                url.parse_host_port(remaining)?;
                url.path = "/".to_string();
            }
        } else {
            // No scheme, treat as path only
            url.path = remaining.to_string();
        }

        // Ensure path starts with '/' if not empty
        if !url.path.is_empty() && !url.path.starts_with('/') {
            url.path = format!("/{}", url.path);
        }

        url.validate()?;
        Ok(url)
    }

    /// Parse host and port
    fn parse_host_port(&mut self, host_port: &str) -> HttpResult<()> {
        if host_port.is_empty() {
            return Ok(());
        }

        if host_port.starts_with('[') && host_port.contains(']') {
            // IPv6 address
            if let Some(bracket_end) = host_port.find(']') {
                self.host = Some(host_port[1..bracket_end].to_string());
                if host_port.len() > bracket_end + 1 && host_port.chars().nth(bracket_end + 1) == Some(':') {
                    let port_str = &host_port[bracket_end + 2..];
                    self.port = Some(port_str.parse()
                        .map_err(|_| HttpError::InvalidUrl(format!("Invalid port: {}", port_str)))?);
                }
            }
        } else if let Some(colon_pos) = host_port.rfind(':') {
            // IPv4 with port or hostname with port
            self.host = Some(host_port[..colon_pos].to_string());
            let port_str = &host_port[colon_pos + 1..];
            self.port = Some(port_str.parse()
                .map_err(|_| HttpError::InvalidUrl(format!("Invalid port: {}", port_str)))?);
        } else {
            // Host only
            self.host = Some(host_port.to_string());
        }

        Ok(())
    }

    /// Set scheme
    pub fn scheme<S: Into<String>>(mut self, scheme: S) -> Self {
        self.scheme = Some(scheme.into());
        self
    }

    /// Set host
    pub fn host<H: Into<String>>(mut self, host: H) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Set port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Set path
    pub fn with_path<P: Into<String>>(mut self, path: P) -> Self {
        self.path = path.into();
        self
    }

    /// Add query parameter
    pub fn query<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.query_params.add(key, value);
        self
    }

    /// Set fragment
    pub fn fragment<F: Into<String>>(mut self, fragment: F) -> Self {
        self.fragment = Some(fragment.into());
        self
    }

    /// Get full URL as string
    pub fn to_string(&self) -> String {
        let mut url = String::new();

        if let Some(scheme) = &self.scheme {
            url.push_str(scheme);
            url.push_str("://");
        }

        if let Some(host) = &self.host {
            if host.contains(':') && !host.starts_with('[') {
                // IPv6 without brackets
                url.push('[');
                url.push_str(host);
                url.push(']');
            } else {
                url.push_str(host);
            }

            if let Some(port) = self.port {
                // Only include port if it's not the default for the scheme
                let include_port = match (self.scheme.as_deref(), port) {
                    (Some("http"), 80) => false,
                    (Some("https"), 443) => false,
                    _ => true,
                };

                if include_port {
                    url.push(':');
                    url.push_str(&port.to_string());
                }
            }
        }

        url.push_str(&self.path);

        if !self.query_params.is_empty() {
            url.push('?');
            url.push_str(&self.query_params.to_string());
        }

        if let Some(fragment) = &self.fragment {
            url.push('#');
            url.push_str(fragment);
        }

        url
    }

    /// Get path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get query parameters
    pub fn query_params(&self) -> &QueryParams {
        &self.query_params
    }

    /// Get mutable query parameters
    pub fn query_params_mut(&mut self) -> &mut QueryParams {
        &mut self.query_params
    }

    /// Get path parameters
    pub fn path_params(&self) -> &PathParams {
        &self.path_params
    }

    /// Get mutable path parameters
    pub fn path_params_mut(&mut self) -> &mut PathParams {
        &mut self.path_params
    }

    /// Check if URL is absolute (has scheme)
    pub fn is_absolute(&self) -> bool {
        self.scheme.is_some()
    }

    /// Check if URL is secure (HTTPS)
    pub fn is_secure(&self) -> bool {
        matches!(self.scheme.as_deref(), Some("https"))
    }

    /// Get authority (host:port)
    pub fn authority(&self) -> Option<String> {
        if let Some(host) = &self.host {
            if let Some(port) = self.port {
                Some(format!("{}:{}", host, port))
            } else {
                Some(host.clone())
            }
        } else {
            None
        }
    }

    /// Get default port for scheme
    pub fn default_port(&self) -> Option<u16> {
        match self.scheme.as_deref() {
            Some("http") => Some(80),
            Some("https") => Some(443),
            Some("ftp") => Some(21),
            Some("ssh") => Some(22),
            _ => None,
        }
    }

    /// Get effective port (explicit or default)
    pub fn effective_port(&self) -> Option<u16> {
        self.port.or_else(|| self.default_port())
    }

    /// Join with another path
    pub fn join(&self, path: &str) -> Self {
        let mut new_url = self.clone();
        
        if path.starts_with('/') {
            new_url.path = path.to_string();
        } else {
            let mut base_path = self.path.clone();
            if !base_path.ends_with('/') {
                if let Some(last_slash) = base_path.rfind('/') {
                    base_path = base_path[..last_slash + 1].to_string();
                } else {
                    base_path = "/".to_string();
                }
            }
            new_url.path = format!("{}{}", base_path, path);
        }

        // Clear query and fragment for joined path
        new_url.query_params = QueryParams::new();
        new_url.fragment = None;

        new_url
    }

    /// Normalize the URL path
    pub fn normalize(&mut self) {
        let mut parts = Vec::new();
        
        for segment in self.path.split('/') {
            match segment {
                "" | "." => continue,
                ".." => {
                    parts.pop();
                }
                _ => parts.push(segment),
            }
        }

        if self.path.starts_with('/') {
            self.path = format!("/{}", parts.join("/"));
        } else {
            self.path = parts.join("/");
        }

        if self.path.is_empty() {
            self.path = "/".to_string();
        }
    }

    /// Validate URL components
    pub fn validate(&self) -> HttpResult<()> {
        // Validate scheme
        if let Some(scheme) = &self.scheme {
            if scheme.is_empty() || !scheme.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.') {
                return Err(HttpError::InvalidUrl(format!("Invalid scheme: {}", scheme)));
            }
        }

        // Validate host
        if let Some(host) = &self.host {
            if host.is_empty() {
                return Err(HttpError::InvalidUrl("Empty host".to_string()));
            }
        }

        // Validate port
        if let Some(port) = self.port {
            if port == 0 || port > 65535 {
                return Err(HttpError::InvalidUrl(format!("Invalid port: {}", port)));
            }
        }

        // Validate path
        if self.path.contains('\0') {
            return Err(HttpError::InvalidUrl("Path contains null byte".to_string()));
        }

        Ok(())
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl FromStr for Url {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_params_parsing() {
        let params = QueryParams::parse("key1=value1&key2=value2&key3");
        
        assert_eq!(params.get("key1"), Some("value1"));
        assert_eq!(params.get("key2"), Some("value2"));
        assert_eq!(params.get("key3"), Some(""));
        assert_eq!(params.get("nonexistent"), None);
    }

    #[test]
    fn test_query_params_encoding() {
        let mut params = QueryParams::new();
        params.insert("key with spaces", "value with spaces");
        params.insert("special", "chars&=?");

        let encoded = params.to_string();
        assert!(encoded.contains("key%20with%20spaces"));
        assert!(encoded.contains("value%20with%20spaces"));
    }

    #[test]
    fn test_url_parsing_full() {
        let url = Url::parse("https://example.com:8080/path/to/resource?q=search&limit=10#section").unwrap();
        
        assert_eq!(url.scheme, Some("https".to_string()));
        assert_eq!(url.host, Some("example.com".to_string()));
        assert_eq!(url.port, Some(8080));
        assert_eq!(url.path, "/path/to/resource");
        assert_eq!(url.query_params.get("q"), Some("search"));
        assert_eq!(url.query_params.get("limit"), Some("10"));
        assert_eq!(url.fragment, Some("section".to_string()));
    }

    #[test]
    fn test_url_parsing_path_only() {
        let url = Url::parse("/api/users?id=123").unwrap();
        
        assert_eq!(url.scheme, None);
        assert_eq!(url.host, None);
        assert_eq!(url.port, None);
        assert_eq!(url.path, "/api/users");
        assert_eq!(url.query_params.get("id"), Some("123"));
    }

    #[test]
    fn test_url_building() {
        let url = Url::new("/api")
            .scheme("https")
            .host("example.com")
            .port(443)
            .query("version", "v1")
            .query("format", "json")
            .fragment("results");

        assert_eq!(url.to_string(), "https://example.com/api?version=v1&format=json#results");
    }

    #[test]
    fn test_url_joining() {
        let base = Url::parse("https://example.com/api/v1/").unwrap();
        let joined = base.join("users/123");
        
        assert_eq!(joined.path, "/api/v1/users/123");
        assert_eq!(joined.scheme, Some("https".to_string()));
        assert_eq!(joined.host, Some("example.com".to_string()));
    }

    #[test]
    fn test_url_normalization() {
        let mut url = Url::parse("/api/../users/./profile/../settings").unwrap();
        url.normalize();
        
        assert_eq!(url.path, "/users/settings");
    }

    #[test]
    fn test_ipv6_url_parsing() {
        let url = Url::parse("http://[::1]:8080/path").unwrap();
        
        assert_eq!(url.host, Some("::1".to_string()));
        assert_eq!(url.port, Some(8080));
        assert_eq!(url.path, "/path");
    }

    #[test]
    fn test_url_properties() {
        let https_url = Url::parse("https://example.com/secure").unwrap();
        assert!(https_url.is_absolute());
        assert!(https_url.is_secure());

        let relative_url = Url::parse("/relative/path").unwrap();
        assert!(!relative_url.is_absolute());
        assert!(!relative_url.is_secure());
    }
}
