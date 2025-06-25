/// HTTP request types for CURSED networking

// use crate::stdlib::net::http::{Method, HttpHeaders};

/// HTTP request representation
#[derive(Debug, Clone)]
pub struct HttpRequest {
impl HttpRequest {
    pub fn new(method: Method, url: String) -> Self {
        Self {
        }
    }
// Re-export Method as HttpMethod for compatibility
pub use super::Method as HttpMethod;

/// Request builder (re-exported from client)
pub use super::client::RequestBuilder;
