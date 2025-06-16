//! GlowUpHTTP - HTTP client and server implementations with modern features
//!
//! This package provides HTTP client and server implementations with modern features
//! and an aesthetically pleasing (glowed up) API. It's inspired by Go's net/http package
//! but designed for the next generation of web development.

pub mod server;
pub mod client;
pub mod handler;
pub mod request;
pub mod response;
pub mod router;
pub mod middleware;
pub mod websocket;
pub mod error;

// Re-export main types for convenience
pub use server::{VibeServer, Serve, ServeTLS};
pub use client::{VibeClient, VibeResponse};
pub use handler::{Handler, HandlerFunc};
pub use request::VibeRequest;
pub use response::ResponderVibe;
pub use router::VibeRouter;
pub use middleware::{MiddlewareFunc, LoggingMiddleware, UnbotheredMiddleware, CORSMiddleware};
pub use websocket::{WebSocketUpgrader, WebSocketConn};
pub use error::{GlowUpError, GlowUpResult};

/// HTTP processing version and capabilities
pub const VERSION: &str = "1.0.0";
pub const SUPPORTED_HTTP_VERSIONS: &[&str] = &["HTTP/1.0", "HTTP/1.1", "HTTP/2.0"];
pub const MAX_HEADER_SIZE: usize = 8192;
pub const MAX_REQUEST_SIZE: usize = 100 * 1024 * 1024; // 100MB
pub const DEFAULT_READ_TIMEOUT: u64 = 30; // seconds
pub const DEFAULT_WRITE_TIMEOUT: u64 = 30; // seconds
pub const DEFAULT_IDLE_TIMEOUT: u64 = 120; // seconds
