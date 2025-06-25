use crate::web::StatusCode;
use crate::error::CursedError;
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
    TimeoutMiddleware, TimeoutConfig, TimeoutError, TimeoutResult, TimeoutStatistics
// };
pub use context::{RequestContext, ResponseContext, ContextData};
pub use handlers::{RequestHandler, RouteHandler, HandlerResult};
pub use route_matcher::{RouteMatcher, RoutePattern, PathSegment, WildcardType};
pub use middleware_chain::{ChainBuilder, MiddlewareOrdering, ChainExecution};
pub use error_handling::{RouterError, MiddlewareError, HandlerError};
pub use config::WebVibezConfig;
pub use health::{HealthChecker, HealthResult, HealthStatus, HealthCheck};
pub use client::{HttpClient, HttpError, HttpResponse, RequestBuilder, Cookie, ConnectionPool};
pub use server::{
    ServerStats, ServerError, Signal, SignalHandler
// };

// Additional module re-exports
pub use compression::{CompressionType, CompressionConfig, CompressionEngine, CompressionManager, CompressionStats};
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
    web_vibez_response_write, web_vibez_response_write_header, web_vibez_free_string
// };

/// HTTP methods supported by the router
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
impl std::str::FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
        }
    }
// Use the main StatusCode type from crate::web

// Display implementation is provided by the main StatusCode enum


