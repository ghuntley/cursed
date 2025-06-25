/// HTTP request types for CURSED networking

// use crate::stdlib::net::http::{Method, HttpHeaders};

/// HTTP request representation
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: Method,
    pub url: String,
    pub headers: HttpHeaders,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(method: Method, url: String) -> Self {
        Self {
            method,
            url,
            headers: HttpHeaders::new(),
            body: None,
        }
    }
}

// Re-export Method as HttpMethod for compatibility
pub use super::Method as HttpMethod;

/// Request builder (re-exported from client)
pub use super::client::RequestBuilder;
