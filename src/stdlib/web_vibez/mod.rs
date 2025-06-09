/// CURSED Web Framework - web_vibez
/// 
/// A high-performance HTTP framework with flexible routing and middleware
/// 
/// Features:
/// - Fast radix tree-based routing with pattern matching
/// - Composable middleware system with request/response lifecycle
/// - HTTP method-specific routes with parameter extraction
/// - Route groups and nested routing patterns
/// - Built-in middleware for common tasks (auth, logging, CORS, etc.)
/// - Request context passing through middleware chain
/// - Route priority and conflict resolution
/// - Performance-optimized route matching algorithms

pub mod router;
pub mod middleware;
pub mod context;
pub mod handlers;
pub mod route_matcher;
pub mod middleware_chain;
pub mod error_handling;

// Re-export main types for easy access
pub use router::{Router, Route, RouteGroup, RoutePriority};
pub use middleware::{
    Middleware, MiddlewareChain, MiddlewareResult,
    AuthMiddleware, LoggingMiddleware, CorsMiddleware, 
    RateLimitMiddleware, StaticFileMiddleware
};
pub use context::{RequestContext, ResponseContext, ContextData};
pub use handlers::{RequestHandler, RouteHandler, HandlerResult};
pub use route_matcher::{RouteMatcher, RoutePattern, PathSegment, WildcardType};
pub use middleware_chain::{ChainBuilder, MiddlewareOrdering, ChainExecution};
pub use error_handling::{RouterError, MiddlewareError, HandlerError};

/// HTTP methods supported by the router
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::TRACE => write!(f, "TRACE"),
            HttpMethod::CONNECT => write!(f, "CONNECT"),
        }
    }
}

impl std::str::FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "TRACE" => Ok(HttpMethod::TRACE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            _ => Err(format!("Unknown HTTP method: {}", s)),
        }
    }
}

/// Status codes for HTTP responses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(pub u16);

impl StatusCode {
    pub const OK: StatusCode = StatusCode(200);
    pub const CREATED: StatusCode = StatusCode(201);
    pub const NO_CONTENT: StatusCode = StatusCode(204);
    pub const BAD_REQUEST: StatusCode = StatusCode(400);
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);
    pub const FORBIDDEN: StatusCode = StatusCode(403);
    pub const NOT_FOUND: StatusCode = StatusCode(404);
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(405);
    pub const CONFLICT: StatusCode = StatusCode(409);
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode(429);
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
    pub const NOT_IMPLEMENTED: StatusCode = StatusCode(501);
    pub const BAD_GATEWAY: StatusCode = StatusCode(502);
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);
    pub const GATEWAY_TIMEOUT: StatusCode = StatusCode(504);
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Configuration for the web_vibez framework
#[derive(Debug, Clone)]
pub struct WebVibezConfig {
    /// Maximum number of routes to cache in route matcher
    pub max_route_cache: usize,
    /// Enable route debugging information
    pub debug_routes: bool,
    /// Maximum middleware chain depth
    pub max_middleware_depth: usize,
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    /// Maximum request body size in bytes
    pub max_request_body_size: usize,
    /// Enable request/response logging
    pub enable_logging: bool,
    /// Rate limiting configuration
    pub rate_limit_requests_per_minute: Option<u32>,
    /// CORS configuration
    pub cors_allowed_origins: Vec<String>,
    /// Static file serving configuration
    pub static_file_root: Option<String>,
}

impl Default for WebVibezConfig {
    fn default() -> Self {
        Self {
            max_route_cache: 1000,
            debug_routes: false,
            max_middleware_depth: 20,
            request_timeout_ms: 30000,
            max_request_body_size: 10 * 1024 * 1024, // 10MB
            enable_logging: true,
            rate_limit_requests_per_minute: Some(1000),
            cors_allowed_origins: vec!["*".to_string()],
            static_file_root: Some("./static".to_string()),
        }
    }
}
