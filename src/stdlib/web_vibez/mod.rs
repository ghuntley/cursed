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
pub mod middleware_async;
pub mod middleware_chain;
pub mod timeout_middleware;
pub mod session;
pub mod session_enhanced;
pub mod session_timeout;
pub mod context;
pub mod handlers;
pub mod route_matcher;
pub mod error_handling;
pub mod config;
pub mod health;
pub mod client;
pub mod server;
pub mod compression;
pub mod csrf;
pub mod debug;
pub mod json;
pub mod monitoring;
pub mod multipart;
pub mod security;
pub mod static_files;
pub mod templates;
pub mod utils;
pub mod runtime;

// Re-export main types for easy access
pub use router::{Router, Route, RouteGroup, RoutePriority};
pub use middleware::{
    Middleware, MiddlewareChain, MiddlewareResult,
    AuthMiddleware, LoggingMiddleware, CorsMiddleware, 
    RateLimitMiddleware, StaticFileMiddleware,
    TimeoutMiddleware, TimeoutConfig, TimeoutError, TimeoutResult, TimeoutStatistics
};
pub use context::{RequestContext, ResponseContext, ContextData};
pub use handlers::{RequestHandler, RouteHandler, HandlerResult};
pub use route_matcher::{RouteMatcher, RoutePattern, PathSegment, WildcardType};
pub use middleware_chain::{ChainBuilder, MiddlewareOrdering, ChainExecution};
pub use error_handling::{RouterError, MiddlewareError, HandlerError};
pub use config::WebVibezConfig;
pub use health::{HealthChecker, HealthResult, HealthStatus, HealthCheck};
pub use client::{HttpClient, HttpError, HttpResponse, RequestBuilder, Cookie, ConnectionPool};
pub use server::{
    HttpServer, ServerState, Connection, ConnectionPool as ServerConnectionPool,
    HttpRequest, HttpResponse as ServerHttpResponse, HttpVersion, TlsConfig, TlsProtocol,
    ServerStats, ServerError, Signal, SignalHandler
};

// Additional module re-exports
pub use compression::{CompressionType, CompressionConfig, CompressionEngine, CompressionStats};
pub use csrf::{CsrfToken, CsrfMiddleware, CsrfConfig, CsrfError};
pub use debug::{RequestDebugger, LiveReload, DebugMode, DebugConfig};
pub use json::{JsonHandler, JsonValue, JsonError, JsonResponse};
pub use monitoring::{MetricsCollector, RequestMetrics, GlobalMetrics, MonitoringDashboard};
pub use multipart::{MultipartProcessor, FileUpload, MultipartError, MultipartField};
pub use security::{InputSanitizer, XssProtection, SecurityHeaders, ContentSecurityPolicy};
pub use static_files::{StaticFileServer, StaticFileCache, StaticFileResponse, StaticFileError};
pub use templates::{TemplateEngine, Template, TemplateContext, TemplateValue, TemplateError};
pub use utils::{ConnectionPool as UtilsConnectionPool, UrlEncoder, HttpHeaders, MimeTypes};
pub use session_enhanced::{EnhancedSessionManager, SessionOptions, SessionSecurity};

// Runtime functions for LLVM integration
pub use runtime::{
    web_vibez_listen_and_serve, web_vibez_listen_and_serve_tls, web_vibez_handle_func,
    web_vibez_get, web_vibez_post, web_vibez_head, web_vibez_delete, web_vibez_client_timeout,
    web_vibez_request_url, web_vibez_request_method, web_vibez_request_body,
    web_vibez_response_write, web_vibez_response_write_header, web_vibez_free_string
};

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


