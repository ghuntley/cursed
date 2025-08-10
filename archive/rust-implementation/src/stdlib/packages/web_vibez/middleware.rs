use super::{HttpRequest, HttpResponse, WebResult};

/// Middleware trait
pub trait Middleware: Send + Sync {
    fn process(&self, request: &HttpRequest, next: &dyn Fn(&HttpRequest) -> WebResult<HttpResponse>) -> WebResult<HttpResponse>;
}

/// Middleware chain
pub struct MiddlewareChain;

/// CORS middleware
pub struct CorsMiddleware {
    allowed_origins: Vec<String>,
    allow_credentials: bool,
}

impl CorsMiddleware {
    pub fn new() -> Self {
        Self {
            allowed_origins: Vec::new(),
            allow_credentials: false,
        }
    }
    
    pub fn permissive() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allow_credentials: false,
        }
    }
    
    pub fn allowed_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    }
    
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }
}

impl Default for CorsMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// Logging middleware
pub struct LoggingMiddleware;

/// Security headers middleware
pub struct SecurityHeadersMiddleware;

/// Rate limit middleware
pub struct RateLimitMiddleware;

/// Middleware handler
pub struct MiddlewareHandler;
