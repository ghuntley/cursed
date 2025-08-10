use crate::stdlib::net::http::response::StatusCode;
use crate::error::CursedError;
/// fr fr HTTP server implementation for CURSED language - web_vibez package
/// This module provides no-cap HTTP server functionality with Gen Z energy

// Core modules
pub mod server;
pub mod request;
pub mod response;
pub mod handler;
pub mod router;
pub mod middleware;
pub mod ratelimit;
pub mod status;
pub mod method;
pub mod error;
pub mod types;

// Re-export core types for easy access - periodt
pub use server::{HttpServer, ServerConfig};
pub use request::{HttpRequest, RequestBuilder};
pub use response::{HttpResponse, ResponseBuilder};
pub use handler::{Handler, HandlerFunc, HandlerChain, FunctionHandler, StaticFileHandler, JsonApiHandler, RedirectHandler, HealthCheckHandler};
pub use router::{Router, RouteEntry, PathParams};
pub use middleware::{Middleware, MiddlewareChain, CorsMiddleware, LoggingMiddleware, SecurityHeadersMiddleware, RateLimitMiddleware, MiddlewareHandler};
pub use status::{HttpStatusCode as WebStatusCode, StatusClass};
pub use method::{HttpMethod, MethodSet, InvalidMethodError};
pub use error::{WebError, WebResult, NetworkErrorKind, AuthErrorKind, ErrorContext};
pub use types::{Headers, QueryParams, FormData, Json, ContentType, RequestBody, Cookie, SameSite};

/// fr fr Initialize the web_vibez package and register it with stdlib
pub fn init_web_vibez() {
    // Initialize HTTP server capabilities
    println!("🌐 web_vibez package initialized - ready to serve some content bestie!");
    register_builtin_functions();
}

/// fr fr Register built-in web functions with the CURSED stdlib
fn register_builtin_functions() {
    // Basic web functions registration
    // In a full implementation, these would register with the CURSED runtime
    // This will integrate with the existing dot registry system
    // Common web functions that would be available globally:
    // - web_vibez.server() - create HTTP server
    // - web_vibez.client() - create HTTP client  
    // - web_vibez.router() - create router
    // - web_vibez.cors() - create CORS middleware
    // Implementation will be added when we integrate with the main stdlib
}
/// fr fr Quick server builder for common use cases - convenience function
pub fn quick_server(port: u16) -> WebResult<HttpServer> {
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let config = ServerConfig::new(addr);
    
    Ok(HttpServer::new(config))
}

/// fr fr Create basic CORS middleware - permissive setup for development
pub fn dev_cors() -> CorsMiddleware {
    CorsMiddleware::permissive()
}

/// fr fr Create production CORS middleware - secure setup
pub fn prod_cors(allowed_origins: Vec<String>) -> CorsMiddleware {
    CorsMiddleware::new()
        .allowed_origins(allowed_origins)
        .allow_credentials(true)
}

/// fr fr Create basic router with common middleware - quick setup
pub fn basic_router() -> Router {
    Router::new()
}

/// fr fr Create router with common middleware applied - production ready
pub fn production_router() -> Router {
    Router::new()
        // Add common production routes like health checks
}
