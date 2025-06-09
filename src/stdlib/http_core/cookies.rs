//! HTTP Cookie Management for CURSED web_vibez
//!
//! Comprehensive cookie parsing, validation, and management.

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::stdlib::http_core::{HttpError, HttpResult};

/// Cookie SameSite attribute
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl fmt::Display for SameSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SameSite::Strict => write!(f, "Strict"),
            SameSite::Lax => write!(f, "Lax"),
            SameSite::None => write!(f, "None"),
        }
    }
}

impl FromStr for SameSite {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "strict" => Ok(SameSite::Strict),
            "lax" => Ok(SameSite::Lax),
            "none" => Ok(SameSite::None),
            _ => Err(HttpError::InvalidCookie(format!("Invalid SameSite value: {}", s))),
        }
    }
}

/// HTTP Cookie structure
#[derive(Debug, Clone)]
pub struct Cookie {
    /// Cookie name
    pub name: String,
    /// Cookie value
    pub value: String,
    /// Domain attribute
    pub domain: Option<String>,
    /// Path attribute
    pub path: Option<String>,
    /// Expires attribute (timestamp)
    pub expires: Option<SystemTime>,
    /// Max-Age attribute (duration in seconds)
    pub max_age: Option<Duration>,
    /// Secure flag
    pub secure: bool,
    /// HttpOnly flag
    pub http_only: bool,
    /// SameSite attribute
    pub same_site: Option<SameSite>,
}

impl Cookie {
    /// Create a new cookie
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
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

    /// Set domain
    pub fn domain<D>(mut self, domain: D) -> Self
    where
        D: Into<String>,
    {
        self.domain = Some(domain.into());
        self
    }

    /// Set path
    pub fn path<P>(mut self, path: P) -> Self
    where
        P: Into<String>,
    {
        self.path = Some(path.into());
        self
    }

    /// Set expires time
    pub fn expires(mut self, expires: SystemTime) -> Self {
        self.expires = Some(expires);
        self
    }

    /// Set max age duration
    pub fn max_age(mut self, max_age: Duration) -> Self {
        self.max_age = Some(max_age);
        self
    }

    /// Set secure flag
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Set http only flag
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    /// Set same site attribute
    pub fn same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = Some(same_site);
        self
    }

    /// Create session cookie (expires when browser closes)
    pub fn session<N, V>(name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        Self::new(name, value)
    }

    /// Create persistent cookie with expiration
    pub fn persistent<N, V>(name: N, value: V, max_age: Duration) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        Self::new(name, value).max_age(max_age)
    }

    /// Create secure cookie for HTTPS
    pub fn secure_cookie<N, V>(name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        Self::new(name, value)
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
    }

    /// Create deletion cookie (expires immediately)
    pub fn delete<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        Self::new(name, "")
            .expires(UNIX_EPOCH)
            .max_age(Duration::from_secs(0))
    }

    /// Parse cookie from string (Cookie header format)
    pub fn parse(cookie_str: &str) -> HttpResult<Self> {
        let parts: Vec<&str> = cookie_str.split(';').map(|s| s.trim()).collect();
        
        if parts.is_empty() {
            return Err(HttpError::InvalidCookie("Empty cookie string".to_string()));
        }

        // Parse name=value pair
        let name_value = parts[0];
        let (name, value) = if let Some(eq_pos) = name_value.find('=') {
            (name_value[..eq_pos].trim(), name_value[eq_pos + 1..].trim())
        } else {
            return Err(HttpError::InvalidCookie("No '=' found in cookie".to_string()));
        };

        if name.is_empty() {
            return Err(HttpError::InvalidCookie("Empty cookie name".to_string()));
        }

        let mut cookie = Self::new(name, value);

        // Parse attributes
        for part in &parts[1..] {
            if let Some(eq_pos) = part.find('=') {
                let attr_name = part[..eq_pos].trim().to_lowercase();
                let attr_value = part[eq_pos + 1..].trim();

                match attr_name.as_str() {
                    "domain" => cookie.domain = Some(attr_value.to_string()),
                    "path" => cookie.path = Some(attr_value.to_string()),
                    "expires" => {
                        if let Ok(expires) = httpdate::parse_http_date(attr_value) {
                            cookie.expires = Some(expires);
                        }
                    }
                    "max-age" => {
                        if let Ok(seconds) = attr_value.parse::<u64>() {
                            cookie.max_age = Some(Duration::from_secs(seconds));
                        }
                    }
                    "samesite" => {
                        if let Ok(same_site) = SameSite::from_str(attr_value) {
                            cookie.same_site = Some(same_site);
                        }
                    }
                    _ => {} // Ignore unknown attributes
                }
            } else {
                // Boolean attributes
                match part.to_lowercase().as_str() {
                    "secure" => cookie.secure = true,
                    "httponly" => cookie.http_only = true,
                    _ => {} // Ignore unknown flags
                }
            }
        }

        cookie.validate()?;
        Ok(cookie)
    }

    /// Parse cookie from Set-Cookie header format
    pub fn parse_set_cookie(set_cookie_str: &str) -> HttpResult<Self> {
        Self::parse(set_cookie_str)
    }

    /// Convert to Cookie header format (name=value only)
    pub fn to_cookie_header(&self) -> String {
        format!("{}={}", self.name, self.value)
    }

    /// Convert to Set-Cookie header format (with all attributes)
    pub fn to_set_cookie_header(&self) -> String {
        let mut result = format!("{}={}", self.name, self.value);

        if let Some(domain) = &self.domain {
            result.push_str(&format!("; Domain={}", domain));
        }

        if let Some(path) = &self.path {
            result.push_str(&format!("; Path={}", path));
        }

        if let Some(expires) = self.expires {
            let expires_str = httpdate::fmt_http_date(expires);
            result.push_str(&format!("; Expires={}", expires_str));
        }

        if let Some(max_age) = self.max_age {
            result.push_str(&format!("; Max-Age={}", max_age.as_secs()));
        }

        if self.secure {
            result.push_str("; Secure");
        }

        if self.http_only {
            result.push_str("; HttpOnly");
        }

        if let Some(same_site) = self.same_site {
            result.push_str(&format!("; SameSite={}", same_site));
        }

        result
    }

    /// Check if cookie is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            return SystemTime::now() > expires;
        }

        if let Some(max_age) = self.max_age {
            // For max_age, we'd need to track when the cookie was created
            // For now, we'll assume it's not expired unless expires is set
            return max_age.is_zero();
        }

        false
    }

    /// Check if cookie is secure
    pub fn is_secure(&self) -> bool {
        self.secure
    }

    /// Check if cookie is http only
    pub fn is_http_only(&self) -> bool {
        self.http_only
    }

    /// Check if cookie matches domain
    pub fn matches_domain(&self, domain: &str) -> bool {
        if let Some(cookie_domain) = &self.domain {
            // Cookie domain matching rules (simplified)
            domain == cookie_domain || domain.ends_with(&format!(".{}", cookie_domain))
        } else {
            true // No domain restriction
        }
    }

    /// Check if cookie matches path
    pub fn matches_path(&self, path: &str) -> bool {
        if let Some(cookie_path) = &self.path {
            path.starts_with(cookie_path)
        } else {
            true // No path restriction
        }
    }

    /// Validate cookie according to RFC 6265
    pub fn validate(&self) -> HttpResult<()> {
        // Validate name
        if self.name.is_empty() {
            return Err(HttpError::InvalidCookie("Cookie name cannot be empty".to_string()));
        }

        // Cookie name must not contain special characters
        for ch in self.name.chars() {
            if ch.is_ascii_control() || "(),/:;<=>?@[\\]{}\" \t".contains(ch) {
                return Err(HttpError::InvalidCookie(
                    format!("Invalid character '{}' in cookie name", ch)
                ));
            }
        }

        // Validate value (simplified)
        for ch in self.value.chars() {
            if ch.is_ascii_control() && ch != '\t' {
                return Err(HttpError::InvalidCookie(
                    "Invalid control character in cookie value".to_string()
                ));
            }
        }

        // Validate domain (simplified)
        if let Some(domain) = &self.domain {
            if domain.is_empty() || domain.starts_with('.') && domain.len() == 1 {
                return Err(HttpError::InvalidCookie("Invalid domain".to_string()));
            }
        }

        // Validate path
        if let Some(path) = &self.path {
            if !path.starts_with('/') {
                return Err(HttpError::InvalidCookie("Path must start with '/'".to_string()));
            }
        }

        Ok(())
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_set_cookie_header())
    }
}

/// Cookie jar for managing multiple cookies
#[derive(Debug, Clone)]
pub struct CookieJar {
    cookies: HashMap<String, Cookie>,
}

impl CookieJar {
    /// Create a new empty cookie jar
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    /// Add a cookie to the jar
    pub fn add(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }

    /// Get a cookie by name
    pub fn get(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }

    /// Remove a cookie by name
    pub fn remove(&mut self, name: &str) -> Option<Cookie> {
        self.cookies.remove(name)
    }

    /// Get all cookies
    pub fn cookies(&self) -> impl Iterator<Item = &Cookie> {
        self.cookies.values()
    }

    /// Get all cookie names
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.cookies.keys()
    }

    /// Clear all cookies
    pub fn clear(&mut self) {
        self.cookies.clear();
    }

    /// Get number of cookies
    pub fn len(&self) -> usize {
        self.cookies.len()
    }

    /// Check if jar is empty
    pub fn is_empty(&self) -> bool {
        self.cookies.is_empty()
    }

    /// Parse cookies from Cookie header
    pub fn parse_cookie_header(&mut self, cookie_header: &str) -> HttpResult<()> {
        for cookie_str in cookie_header.split(';') {
            let trimmed = cookie_str.trim();
            if !trimmed.is_empty() {
                if let Some(eq_pos) = trimmed.find('=') {
                    let name = trimmed[..eq_pos].trim();
                    let value = trimmed[eq_pos + 1..].trim();
                    self.add(Cookie::new(name, value));
                }
            }
        }
        Ok(())
    }

    /// Convert to Cookie header format
    pub fn to_cookie_header(&self) -> String {
        self.cookies
            .values()
            .map(|cookie| cookie.to_cookie_header())
            .collect::<Vec<_>>()
            .join("; ")
    }

    /// Get cookies matching domain and path
    pub fn matching_cookies(&self, domain: &str, path: &str) -> Vec<&Cookie> {
        self.cookies
            .values()
            .filter(|cookie| {
                !cookie.is_expired() && 
                cookie.matches_domain(domain) && 
                cookie.matches_path(path)
            })
            .collect()
    }

    /// Remove expired cookies
    pub fn remove_expired(&mut self) {
        self.cookies.retain(|_, cookie| !cookie.is_expired());
    }

    /// Merge another cookie jar into this one
    pub fn merge(&mut self, other: CookieJar) {
        for (name, cookie) in other.cookies {
            self.cookies.insert(name, cookie);
        }
    }
}

impl Default for CookieJar {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for CookieJar {
    type Item = (String, Cookie);
    type IntoIter = std::collections::hash_map::IntoIter<String, Cookie>;

    fn into_iter(self) -> Self::IntoIter {
        self.cookies.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cookie_creation() {
        let cookie = Cookie::new("session", "abc123")
            .domain("example.com")
            .path("/")
            .secure(true)
            .http_only(true);

        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "abc123");
        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert_eq!(cookie.path, Some("/".to_string()));
        assert!(cookie.secure);
        assert!(cookie.http_only);
    }

    #[test]
    fn test_cookie_parsing() {
        let cookie_str = "session=abc123; Domain=example.com; Path=/; Secure; HttpOnly";
        let cookie = Cookie::parse(cookie_str).unwrap();

        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "abc123");
        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert_eq!(cookie.path, Some("/".to_string()));
        assert!(cookie.secure);
        assert!(cookie.http_only);
    }

    #[test]
    fn test_cookie_formatting() {
        let cookie = Cookie::new("test", "value")
            .domain("example.com")
            .path("/app")
            .secure(true)
            .same_site(SameSite::Strict);

        let set_cookie = cookie.to_set_cookie_header();
        assert!(set_cookie.contains("test=value"));
        assert!(set_cookie.contains("Domain=example.com"));
        assert!(set_cookie.contains("Path=/app"));
        assert!(set_cookie.contains("Secure"));
        assert!(set_cookie.contains("SameSite=Strict"));
    }

    #[test]
    fn test_cookie_jar() {
        let mut jar = CookieJar::new();
        jar.add(Cookie::new("session", "abc123"));
        jar.add(Cookie::new("prefs", "dark_mode"));

        assert_eq!(jar.len(), 2);
        assert!(jar.get("session").is_some());
        assert!(jar.get("prefs").is_some());
        assert!(jar.get("nonexistent").is_none());

        let cookie_header = jar.to_cookie_header();
        assert!(cookie_header.contains("session=abc123"));
        assert!(cookie_header.contains("prefs=dark_mode"));
    }

    #[test]
    fn test_cookie_domain_matching() {
        let cookie = Cookie::new("test", "value").domain("example.com");
        
        assert!(cookie.matches_domain("example.com"));
        assert!(cookie.matches_domain("sub.example.com"));
        assert!(!cookie.matches_domain("other.com"));
    }

    #[test]
    fn test_cookie_path_matching() {
        let cookie = Cookie::new("test", "value").path("/api");
        
        assert!(cookie.matches_path("/api"));
        assert!(cookie.matches_path("/api/users"));
        assert!(!cookie.matches_path("/"));
        assert!(!cookie.matches_path("/other"));
    }

    #[test]
    fn test_same_site_parsing() {
        assert_eq!(SameSite::from_str("Strict").unwrap(), SameSite::Strict);
        assert_eq!(SameSite::from_str("lax").unwrap(), SameSite::Lax);
        assert_eq!(SameSite::from_str("None").unwrap(), SameSite::None);
        assert!(SameSite::from_str("Invalid").is_err());
    }
}
