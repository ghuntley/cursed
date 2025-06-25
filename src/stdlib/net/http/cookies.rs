use crate::error::CursedError;
/// HTTP cookie management for CURSED networking

use std::collections::HashMap;
use std::time::SystemTime;
// use crate::stdlib::net::error::{NetError, NetResult};

/// HTTP cookie representation
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<SystemTime>,
    pub max_age: Option<u64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

/// SameSite attribute values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Cookie {
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
    
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            SystemTime::now() > expires
        } else {
            false
        }
    }
    
    pub fn matches_domain(&self, domain: &str) -> bool {
        if let Some(cookie_domain) = &self.domain {
            domain == cookie_domain || domain.ends_with(&format!(".{}", cookie_domain))
        } else {
            true
        }
    }
    
    pub fn matches_path(&self, path: &str) -> bool {
        if let Some(cookie_path) = &self.path {
            path.starts_with(cookie_path)
        } else {
            true
        }
    }
}

/// Cookie jar for managing cookies
#[derive(Debug)]
pub struct CookieJar {
    cookies: HashMap<String, Cookie>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }
    
    pub fn add_cookie(&mut self, cookie: Cookie) {
        let key = format!("{}:{}", cookie.domain.as_deref().unwrap_or(""), cookie.name);
        self.cookies.insert(key, cookie);
    }
    
    pub fn add_cookie_from_header(&mut self, header_value: &str) -> NetResult<()> {
        // Parse Set-Cookie header
        let cookie = self.parse_set_cookie_header(header_value)?;
        self.add_cookie(cookie);
        Ok(())
    }
    
    pub fn get_cookies_for_request(&self, domain: &str, path: &str) -> String {
        let mut cookie_values = Vec::new();
        
        for cookie in self.cookies.values() {
            if !cookie.is_expired() && cookie.matches_domain(domain) && cookie.matches_path(path) {
                cookie_values.push(format!("{}={}", cookie.name, cookie.value));
            }
        }
        
        cookie_values.join("; ")
    }
    
    fn parse_set_cookie_header(&self, header_value: &str) -> NetResult<Cookie> {
        let parts: Vec<&str> = header_value.split(';').collect();
        if parts.is_empty() {
            return Err(NetError::Http {
                status_code: None,
                message: "Invalid Set-Cookie header".to_string(),
                url: None,
            });
        }
        
        // Parse name=value
        let name_value = parts[0].trim();
        let (name, value) = if let Some(eq_pos) = name_value.find('=') {
            (name_value[..eq_pos].trim(), name_value[eq_pos + 1..].trim())
        } else {
            return Err(NetError::Http {
                status_code: None,
                message: "Invalid cookie name=value".to_string(),
                url: None,
            });
        };
        
        let mut cookie = Cookie::new(name.to_string(), value.to_string());
        
        // Parse attributes
        for part in parts.iter().skip(1) {
            let part = part.trim();
            if let Some(eq_pos) = part.find('=') {
                let attr_name = part[..eq_pos].trim().to_lowercase();
                let attr_value = part[eq_pos + 1..].trim();
                
                match attr_name.as_str() {
                    "domain" => cookie.domain = Some(attr_value.to_string()),
                    "path" => cookie.path = Some(attr_value.to_string()),
                    "max-age" => {
                        if let Ok(max_age) = attr_value.parse::<u64>() {
                            cookie.max_age = Some(max_age);
                        }
                    },
                    _ => {} // Ignore unknown attributes
                }
            } else {
                match part.to_lowercase().as_str() {
                    "secure" => cookie.secure = true,
                    "httponly" => cookie.http_only = true,
                    _ => {} // Ignore unknown flags
                }
            }
        }
        
        Ok(cookie)
    }
    
    pub fn clear(&mut self) {
        self.cookies.clear();
    }
    
    pub fn remove_expired(&mut self) {
        self.cookies.retain(|_, cookie| !cookie.is_expired());
    }
}

impl Default for CookieJar {
    fn default() -> Self {
        Self::new()
    }
}

/// Cookie store trait for pluggable storage
pub trait CookieStore {
    fn save_cookie(&mut self, cookie: &Cookie) -> NetResult<()>;
    fn load_cookies(&self, domain: &str, path: &str) -> NetResult<Vec<Cookie>>;
    fn clear_cookies(&mut self) -> NetResult<()>;
}
