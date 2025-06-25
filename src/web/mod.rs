use crate::error::CursedError;
// Web module for CURSED HTTP server functionality
use std::collections::HashMap;
use std::fmt;

/// Template system for CURSED web applications
pub mod template;

pub use template::{
    TemplateRenderer, TemplateCache, TemplateLoader
// };

/// HTTP status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusCode {
    // 1xx Informational
    
    // 2xx Success
    
    // 3xx Redirection
    
    // 4xx Client CursedError
    
    // 5xx Server CursedError
impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    /// Create StatusCode from u16 value
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
        }
    }

    /// Alias for reason_phrase for compatibility
    pub fn canonical_reason(&self) -> &'static str {
        self.reason_phrase()
    pub fn reason_phrase(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn is_success(&self) -> bool {
        matches!(self.as_u16(), 200..=299)
    pub fn is_redirection(&self) -> bool {
        matches!(self.as_u16(), 300..=399)
    pub fn is_client_error(&self) -> bool {
        matches!(self.as_u16(), 400..=499)
    pub fn is_server_error(&self) -> bool {
        matches!(self.as_u16(), 500..=599)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.as_u16(), self.reason_phrase())
    }
}

/// HTTP methods
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// HTTP headers
#[derive(Debug, Clone)]
pub struct Headers {
impl Headers {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn insert(&mut self, key: String, value: String) {
        self.inner.insert(key.to_lowercase(), value);
    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(&key.to_lowercase())
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.inner.remove(&key.to_lowercase())
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.inner.iter()
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP request
#[derive(Debug, Clone)]
pub struct Request {
impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
        }
    }
    
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    pub fn with_query_param(mut self, key: String, value: String) -> Self {
        self.query_params.insert(key, value);
        self
    pub fn body_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }
}

/// HTTP response
#[derive(Debug, Clone)]
pub struct Response {
impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
        }
    }
    
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    pub fn with_text(mut self, text: String) -> Self {
        self.body = text.into_bytes();
        self.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        self
    pub fn with_json(mut self, json: String) -> Self {
        self.body = json.into_bytes();
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self
    pub fn with_html(mut self, html: String) -> Self {
        self.body = html.into_bytes();
        self.headers.insert("Content-Type".to_string(), "text/html".to_string());
        self
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    pub fn body_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new(StatusCode::OK)
    }
}

/// Session management
#[derive(Debug, Clone)]
pub enum SameSitePolicy {
#[derive(Debug, Clone)]
pub enum SessionStoreType {
#[derive(Debug, Clone)]
pub struct SessionConfig {
impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_age: Some(3600), // 1 hour
        }
    }
/// Session data
#[derive(Debug, Clone)]
pub struct Session {
impl Session {
    pub fn new(id: String) -> Self {
        Self {
        }
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            std::time::SystemTime::now() > expires
        } else {
            false
        }
    }
/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Timeout configuration
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Timeout errors
#[derive(Debug, Clone)]
pub enum TimeoutError {
// impl fmt::Display for TimeoutError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             TimeoutError::RequestTimeout { elapsed, timeout } => {
//                 write!(f, "Request timeout: elapsed {:?}, timeout {:?}", elapsed, timeout)
//             }
//             TimeoutError::ResponseTimeout { elapsed, timeout } => {
//                 write!(f, "Response timeout: elapsed {:?}, timeout {:?}", elapsed, timeout)
//             }
//             TimeoutError::DatabaseTimeout { elapsed, timeout, operation } => {
//                 write!(f, "Database timeout for {}: elapsed {:?}, timeout {:?}", operation, elapsed, timeout)
//             }
//             TimeoutError::ConnectionTimeout { elapsed, timeout } => {
//                 write!(f, "Connection timeout: elapsed {:?}, timeout {:?}", elapsed, timeout)
//             }
//         }
//     }
// }

// impl std::error::CursedError for TimeoutError {}
// 
/// Middleware trait
pub trait Middleware: Send + Sync {
    fn handle(&self, request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::CursedError>>;
/// Middleware chain
#[derive(Debug)]
pub struct MiddlewareChain {
impl MiddlewareChain {
    pub fn new(middlewares: Vec<Box<dyn Middleware>>) -> Self {
        Self { middlewares }
    }
    
    pub fn add_middleware(&mut self, middleware: Box<dyn Middleware>) {
        self.middlewares.push(middleware);
    pub fn handle(&self, request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::CursedError>> {
        for middleware in &self.middlewares {
            middleware.handle(request, response)?;
        }
        Ok(())
    }
}

/// Basic middleware implementations
#[derive(Debug)]
pub struct LoggingMiddleware {
impl LoggingMiddleware {
    pub fn new() -> Self {
        Self { enabled: true }
    }
impl Middleware for LoggingMiddleware {
    fn handle(&self, request: &mut Request, _response: &mut Response) -> Result<(), Box<dyn std::error::CursedError>> {
        if self.enabled {
                request.path
            );
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CorsMiddleware {
impl CorsMiddleware {
    pub fn new() -> Self {
        Self {
            allow_headers: vec![
        }
    }
impl Middleware for CorsMiddleware {
    fn handle(&self, _request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::CursedError>> {
        response.headers.insert("Access-Control-Allow-Origin".to_string(), self.allow_origins.join(", "));
            self.allow_methods.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", "));
        response.headers.insert("Access-Control-Allow-Headers".to_string(), self.allow_headers.join(", "));
        Ok(())
    }
}

#[derive(Debug)]
pub struct RateLimitMiddleware {
impl RateLimitMiddleware {
    pub fn new(requests_per_minute: usize) -> Self {
        Self {
        }
    }
impl Middleware for RateLimitMiddleware {
    fn handle(&self, _request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::CursedError>> {
        // Simple rate limiting - in production this would use a proper store
        response.headers.insert("X-RateLimit-Limit".to_string(), self.requests_per_minute.to_string());
        response.headers.insert("X-RateLimit-Remaining".to_string(), self.requests_per_minute.to_string());
        Ok(())
    }
}

#[derive(Debug)]
pub struct TimeoutMiddleware {
impl TimeoutMiddleware {
    pub fn new(_server_config: ServerConfig, _session_config: SessionConfig) -> Self {
        Self {
        }
    }
impl Middleware for TimeoutMiddleware {
    fn handle(&self, _request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::CursedError>> {
                self.config.response_timeout));
        Ok(())
    }
}

/// Session manager
#[derive(Debug)]
pub struct TimeoutSessionManager {
impl TimeoutSessionManager {
    pub fn new(config: SessionConfig) -> Result<Self, Box<dyn std::error::CursedError>> {
        Ok(Self {
        })
    pub fn create_session(&mut self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let session = Session::new(id.clone());
        self.sessions.insert(id.clone(), session);
        id
    pub fn get_session(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    pub fn get_session_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    pub fn remove_session(&mut self, id: &str) -> Option<Session> {
        self.sessions.remove(id)
    pub fn cleanup_expired(&mut self) {
        let expired_ids: Vec<String> = self.sessions
            .iter()
            .filter(|(_, session)| session.is_expired())
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in expired_ids {
            self.sessions.remove(&id);
        }
    }
/// Router
#[derive(Debug)]
pub struct Router {
impl Router {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn add_route<F>(&mut self, method: Method, path: String, handler: F)
    where
    {
        self.routes.insert((method, path), Box::new(handler));
    pub fn handle(&self, request: &Request) -> Option<Response> {
        if let Some(handler) = self.routes.get(&(request.method.clone(), request.path.clone())) {
            Some(handler(request))
        } else {
            None
        }
    }
impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

