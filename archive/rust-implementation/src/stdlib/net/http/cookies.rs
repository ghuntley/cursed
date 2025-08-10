//! HTTP cookie functionality

use std::collections::HashMap;

/// HTTP cookie
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: bool,
    pub http_only: bool,
    pub max_age: Option<u64>,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            max_age: None,
        }
    }
    
    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }
    
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
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

/// Cookie jar for managing cookies
#[derive(Debug, Clone, Default)]
pub struct CookieJar {
    cookies: HashMap<String, Cookie>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }
    
    pub fn get(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }
    
    pub fn remove(&mut self, name: &str) -> Option<Cookie> {
        self.cookies.remove(name)
    }
    
    pub fn clear(&mut self) {
        self.cookies.clear();
    }
}

/// Cookie store interface
pub trait CookieStore {
    fn store_cookie(&mut self, cookie: Cookie);
    fn get_cookies_for_url(&self, url: &str) -> Vec<Cookie>;
    fn clear_cookies(&mut self);
}

impl CookieStore for CookieJar {
    fn store_cookie(&mut self, cookie: Cookie) {
        self.add(cookie);
    }
    
    fn get_cookies_for_url(&self, _url: &str) -> Vec<Cookie> {
        // Stub implementation - would filter by domain/path
        self.cookies.values().cloned().collect()
    }
    
    fn clear_cookies(&mut self) {
        self.clear();
    }
}
