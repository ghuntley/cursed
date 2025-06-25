use crate::web::StatusCode;
/// HTTP response types for CURSED networking

// use crate::stdlib::net::http::{Status, HttpHeaders, HttpVersion};

/// HTTP response representation
#[derive(Debug, Clone)]
pub struct HttpResponse {
impl HttpResponse {
    pub fn new(status: Status) -> Self {
        Self {
        }
    }
// Re-export Status as StatusCode for compatibility
pub use super::Status as StatusCode;

/// Response body wrapper
#[derive(Debug, Clone)]
pub struct ResponseBody {
impl ResponseBody {
    pub fn new(content: String) -> Self {
        Self { content }
    }
    
    pub fn text(&self) -> &str {
        &self.content
    pub fn bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }
}
