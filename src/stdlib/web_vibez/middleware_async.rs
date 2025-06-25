use crate::web::StatusCode;
/// Middleware system for HTTP request/response processing
/// 
/// Provides a flexible middleware chain system with common middleware
/// implementations for authentication, logging, CORS, rate limiting, and more

// use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext, ContextData};
// use crate::stdlib::web_vibez::handlers::{RequestHandler, HandlerResult};
// use crate::stdlib::web_vibez::{HttpMethod, StatusCode};
use crate::error::CursedError;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn, error, instrument};
use async_trait::async_trait;

use std::future::Future;
use std::pin::Pin;

/// Middleware trait for processing requests and responses
#[async_trait::async_trait]
pub trait Middleware: Send + Sync {
    /// Process request before handler
    async fn before_request(
    ) -> crate::error::Result<()> {
        Ok(())
    /// Process response after handler
    async fn after_response(
    ) -> crate::error::Result<()> {
        Ok(())
    /// Handle middleware error
    async fn on_error(
    ) -> crate::error::Result<()> {
        let error_msg = format!("Middleware error: {}", error);
        response.set_status(StatusCode::InternalServerError);
        response.set_text(&error_msg);
        Ok(())
    /// Get middleware name for debugging
    fn name(&self) -> &'static str {
        "Unknown"
    /// Get middleware priority (lower = earlier in chain)
    fn priority(&self) -> u32 {
        100
    }
}

impl std::fmt::Debug for dyn Middleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Middleware({})", self.name())
    }
}

/// Result type for middleware operations
pub type MiddlewareResult<T> = std::result::Result<T, MiddlewareError>;

/// Authentication middleware
#[derive(Debug)]
pub struct AuthMiddleware {
    /// Required authentication schemes
    /// Optional paths that don't require authentication
    /// Custom authentication validator
#[derive(Debug, Clone)]
pub enum AuthScheme {
#[async_trait::async_trait]
pub trait AuthValidator: Send + Sync {
    async fn validate_credentials(&self, scheme: &AuthScheme, credentials: &str) -> bool;
impl AuthMiddleware {
    pub fn new(schemes: Vec<AuthScheme>) -> Self {
        Self {
        }
    }

    pub fn with_skip_paths(mut self, paths: Vec<String>) -> Self {
        self.skip_paths = paths;
        self
    pub fn with_validator(mut self, validator: Arc<dyn AuthValidator>) -> Self {
        self.validator = Some(validator);
        self
    }
}

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn before_request(
    ) -> MiddlewareResult {
        // Check if path should skip authentication
        if self.skip_paths.iter().any(|skip_path| context.path.starts_with(skip_path)) {
            return Ok(());
        let auth_header = context.header("authorization")
            .ok_or_else(|| MiddlewareError::Authentication("Missing Authorization header".to_string()))?;

        for scheme in &self.schemes {
            if let Some(credentials) = self.extract_credentials(scheme, auth_header) {
                if let Some(validator) = &self.validator {
                    if validator.validate_credentials(scheme, &credentials).await {
                        context.set_data("authenticated", ContextData::Boolean(true));
                        context.set_data("auth_scheme", ContextData::String(format!("{:?}", scheme)));
                        return Ok(());
                    }
                } else {
                    // No validator means accept any credentials for this scheme
                    context.set_data("authenticated", ContextData::Boolean(true));
                    return Ok(());
                }
            }
        Err(MiddlewareError::Authentication("Invalid credentials".to_string()))
    async fn on_error(
    ) -> MiddlewareResult {
        match error {
            MiddlewareError::Authentication(_) => {
                response.set_status(StatusCode::Unauthorized);
                response.set_header("WWW-Authenticate", "Basic");
                response.set_text("Authentication required");
            }
            _ => {
                response.set_status(StatusCode::InternalServerError);
                response.set_text("Internal server error");
            }
        }
        Ok(())
    fn name(&self) -> &'static str {
        "Auth"
    fn priority(&self) -> u32 {
        10 // High priority - run early
    }
}

impl AuthMiddleware {
    fn extract_credentials(&self, scheme: &AuthScheme, auth_header: &str) -> Option<String> {
        match scheme {
            AuthScheme::Basic => {
                if auth_header.starts_with("Basic ") {
                    Some(auth_header[6..].to_string())
                } else {
                    None
                }
            }
            AuthScheme::Bearer => {
                if auth_header.starts_with("Bearer ") {
                    Some(auth_header[7..].to_string())
                } else {
                    None
                }
            }
            AuthScheme::ApiKey { header: _ } => {
                // API key would be extracted from custom header
                Some(auth_header.to_string())
            }
            AuthScheme::Custom { name: _ } => {
                Some(auth_header.to_string())
            }
        }
    }
}

/// Logging middleware for request/response logging
#[derive(Debug)]
pub struct LoggingMiddleware {
    /// Log level for requests
    /// Include request body in logs
    /// Include response body in logs
    /// Maximum body size to log
    /// Skip logging for specific paths
#[derive(Debug, Clone)]
pub enum LogLevel {
impl LoggingMiddleware {
    pub fn new() -> Self {
        Self {
            skip_paths: Vec::from(["/health".to_string(), "/metrics".to_string()]),
        }
    }

    pub fn with_body_logging(mut self, request: bool, response: bool) -> Self {
        self.log_request_body = request;
        self.log_response_body = response;
        self
    pub fn with_skip_paths(mut self, paths: Vec<String>) -> Self {
        self.skip_paths = paths;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn before_request(
    ) -> MiddlewareResult {
        if self.skip_paths.iter().any(|skip_path| context.path.starts_with(skip_path)) {
            return Ok(());
        let body_preview = if self.log_request_body && !context.body.is_empty() {
            let preview_size = std::cmp::min(context.body.len(), self.max_body_log_size);
            match String::from_utf8(context.body[..preview_size].to_vec()) {
            }
        } else {
            None

        match self.log_level {
            LogLevel::Info => {
                info!(
                    "Incoming request"
                );
            }
            LogLevel::Debug => {
                debug!(
                    "Incoming request with details"
                );
            }
            LogLevel::Trace => {
                let full_body = if self.log_request_body {
                    String::from_utf8_lossy(&context.body).to_string()
                } else {
                    format!("<{} bytes>", context.body.len())
                
                tracing::trace!(
                    "Detailed request trace"
                );
            }
        }

        Ok(())
    async fn after_response(
    ) -> MiddlewareResult {
        if self.skip_paths.iter().any(|skip_path| context.path.starts_with(skip_path)) {
            return Ok(());
        let elapsed = context.elapsed();
        let body_preview = if self.log_response_body && !response.body.is_empty() {
            let preview_size = std::cmp::min(response.body.len(), self.max_body_log_size);
            match String::from_utf8(response.body[..preview_size].to_vec()) {
            }
        } else {
            None

        match self.log_level {
            LogLevel::Info => {
                info!(
                    "Request completed"
                );
            }
            LogLevel::Debug => {
                debug!(
                    "Request completed with details"
                );
            }
            LogLevel::Trace => {
                let full_body = if self.log_response_body {
                    String::from_utf8_lossy(&response.body).to_string()
                } else {
                    format!("<{} bytes>", response.body.len())
                
                tracing::trace!(
                    "Detailed response trace"
                );
            }
        }

        Ok(())
    fn name(&self) -> &'static str {
        "Logging"
    fn priority(&self) -> u32 {
        5 // Very high priority - log everything
    }
}

/// CORS middleware for cross-origin resource sharing
#[derive(Debug)]
pub struct CorsMiddleware {
    /// Allowed origins
    /// Allowed methods
    /// Allowed headers
    /// Exposed headers
    /// Allow credentials
    /// Max age for preflight requests
impl CorsMiddleware {
    pub fn new() -> Self {
        Self {
            allowed_methods: vec![
            allowed_headers: vec![
            max_age: Some(Duration::from_secs(3600)), // 1 hour
        }
    }

    pub fn with_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    pub fn with_methods(mut self, methods: Vec<HttpMethod>) -> Self {
        self.allowed_methods = methods;
        self
    pub fn with_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for CorsMiddleware {
    async fn before_request(
    ) -> MiddlewareResult {
        // Handle preflight OPTIONS request
        if context.method == HttpMethod::OPTIONS {
            self.set_cors_headers(context, response);
            response.set_status(StatusCode::NoContent);
            response.mark_sent(); // Skip further processing
        Ok(())
    async fn after_response(
    ) -> MiddlewareResult {
        if context.method != HttpMethod::OPTIONS {
            self.set_cors_headers(context, response);
        }
        Ok(())
    fn name(&self) -> &'static str {
        "CORS"
    fn priority(&self) -> u32 {
        15 // High priority - set headers early
    }
}

impl CorsMiddleware {
    fn set_cors_headers(&self, context: &RequestContext, response: &mut ResponseContext) {
        // Set allowed origins
        if self.allowed_origins.contains(&"*".to_string()) && !self.allow_credentials {
            response.set_header("Access-Control-Allow-Origin", "*");
        } else if let Some(origin) = context.header("origin") {
            if self.allowed_origins.contains(&origin.to_string()) || self.allowed_origins.contains(&"*".to_string()) {
                response.set_header("Access-Control-Allow-Origin", origin);
            }
        }

        // Set allowed methods
        let methods: Vec<String> = self.allowed_methods.iter().map(|m| m.to_string()).collect();
        response.set_header("Access-Control-Allow-Methods", &methods.join(", "));

        // Set allowed headers
        response.set_header("Access-Control-Allow-Headers", &self.allowed_headers.join(", "));

        // Set exposed headers
        if !self.exposed_headers.is_empty() {
            response.set_header("Access-Control-Expose-Headers", &self.exposed_headers.join(", "));
        // Set credentials
        if self.allow_credentials {
            response.set_header("Access-Control-Allow-Credentials", "true");
        // Set max age
        if let Some(max_age) = self.max_age {
            response.set_header("Access-Control-Max-Age", &max_age.as_secs().to_string());
        }
    }
/// Rate limiting middleware
#[derive(Debug)]
pub struct RateLimitMiddleware {
    /// Requests per time window
    /// Time window duration
    /// Rate limiter storage
    /// Key extractor function
    /// Skip rate limiting for specific paths
#[derive(Debug)]
struct RateLimiter {
    /// Request counts per key
    /// Last cleanup time
#[derive(Debug)]
struct RequestCount {
impl RateLimitMiddleware {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            limiter: Arc::new(Mutex::new(RateLimiter {
            key_extractor: |context| {
                context.client_ip.clone().unwrap_or_else(|| "unknown".to_string())
            skip_paths: Vec::from(["/health".to_string()]),
        }
    }

    pub fn with_window(mut self, requests: u32, duration: Duration) -> Self {
        self.requests_per_window = requests;
        self.window_duration = duration;
        self
    pub fn with_key_extractor(mut self, extractor: fn(&RequestContext) -> String) -> Self {
        self.key_extractor = extractor;
        self
    pub fn with_skip_paths(mut self, paths: Vec<String>) -> Self {
        self.skip_paths = paths;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimitMiddleware {
    async fn before_request(
    ) -> MiddlewareResult {
        if self.skip_paths.iter().any(|skip_path| context.path.starts_with(skip_path)) {
            return Ok(());
        let key = (self.key_extractor)(context);
        let now = Instant::now();

        let mut limiter = self.limiter.lock().unwrap();
        
        // Cleanup old entries periodically
        if now.duration_since(limiter.last_cleanup) > Duration::from_secs(300) {
            limiter.requests.retain(|_, count| {
                now.duration_since(count.window_start) < self.window_duration
            });
            limiter.last_cleanup = now;
        // Check rate limit
        let request_count = limiter.requests.entry(key.clone()).or_insert_with(|| {
            RequestCount {
            }
        });

        // Reset window if expired
        if now.duration_since(request_count.window_start) > self.window_duration {
            request_count.count = 0;
            request_count.window_start = now;
        // Check if limit exceeded
        if request_count.count >= self.requests_per_window {
            let reset_time = request_count.window_start + self.window_duration;
            let retry_after = reset_time.duration_since(now).as_secs();
            
            response.set_status(StatusCode::TooManyRequests);
            response.set_header("Retry-After", &retry_after.to_string());
            response.set_header("X-RateLimit-Limit", &self.requests_per_window.to_string());
            response.set_header("X-RateLimit-Remaining", "0");
            response.set_header("X-RateLimit-Reset", &reset_time.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
            response.set_text("Rate limit exceeded");
            
            return Err(MiddlewareError::RateLimit(format!("Rate limit exceeded for key: {}", key)));
        // Increment counter
        request_count.count += 1;

        // Set rate limit headers
        response.set_header("X-RateLimit-Limit", &self.requests_per_window.to_string());
        response.set_header("X-RateLimit-Remaining", &(self.requests_per_window - request_count.count).to_string());
        let reset_time = request_count.window_start + self.window_duration;
        response.set_header("X-RateLimit-Reset", &reset_time.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());

        Ok(())
    fn name(&self) -> &'static str {
        "RateLimit"
    fn priority(&self) -> u32 {
        20 // High priority - check limits early
    }
}

/// Static file serving middleware
#[derive(Debug)]
pub struct StaticFileMiddleware {
    /// Root directory for static files
    /// URL prefix to match
    /// Cache duration for static files
    /// Index files to serve for directories
    /// MIME type mapping
impl StaticFileMiddleware {
    pub fn new(root_dir: PathBuf, url_prefix: &str) -> Self {
        let mut mime_types = HashMap::new();
        mime_types.insert("html".to_string(), "text/html".to_string());
        mime_types.insert("css".to_string(), "text/css".to_string());
        mime_types.insert("js".to_string(), "application/javascript".to_string());
        mime_types.insert("json".to_string(), "application/json".to_string());
        mime_types.insert("png".to_string(), "image/png".to_string());
        mime_types.insert("jpg".to_string(), "image/jpeg".to_string());
        mime_types.insert("jpeg".to_string(), "image/jpeg".to_string());
        mime_types.insert("gif".to_string(), "image/gif".to_string());
        mime_types.insert("svg".to_string(), "image/svg+xml".to_string());
        mime_types.insert("ico".to_string(), "image/x-icon".to_string());
        
        Self {
            cache_duration: Some(Duration::from_secs(3600)), // 1 hour
        }
    }

    pub fn with_cache_duration(mut self, duration: Option<Duration>) -> Self {
        self.cache_duration = duration;
        self
    pub fn with_index_files(mut self, files: Vec<String>) -> Self {
        self.index_files = files;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for StaticFileMiddleware {
    async fn before_request(
    ) -> MiddlewareResult {
        // Only handle GET requests
        if context.method != HttpMethod::GET {
            return Ok(());
        // Check if path matches our prefix
        if !context.path.starts_with(&self.url_prefix) {
            return Ok(());
        // Extract file path
        let relative_path = &context.path[self.url_prefix.len()..];
        let file_path = self.root_dir.join(relative_path.trim_start_matches('/'));

        // Security check: prevent directory traversal
        if !file_path.starts_with(&self.root_dir) {
            return Err(MiddlewareError::Security("Directory traversal attempt".to_string()));
        // Try to serve the file
        if let Ok(metadata) = std::fs::metadata(&file_path) {
            if metadata.is_file() {
                self.serve_file(&file_path, response).await?;
                response.mark_sent();
            } else if metadata.is_dir() {
                // Try to serve index file
                for index_file in &self.index_files {
                    let index_path = file_path.join(index_file);
                    if index_path.exists() && index_path.is_file() {
                        self.serve_file(&index_path, response).await?;
                        response.mark_sent();
                        break;
                    }
                }
            }
        }

        Ok(())
    fn name(&self) -> &'static str {
        "StaticFile"
    fn priority(&self) -> u32 {
        200 // Low priority - handle after other middleware
    }
}

impl StaticFileMiddleware {
    async fn serve_file(&self, file_path: &Path, response: &mut ResponseContext) -> crate::error::Result<()> {
        let content = std::fs::read(file_path)
            .map_err(|e| MiddlewareError::FileSystem(format!("Failed to read file: {}", e)))?;

        // Set content type based on file extension
        if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
            if let Some(mime_type) = self.mime_types.get(extension) {
                response.set_header("Content-Type", mime_type);
            }
        }

        // Set cache headers
        if let Some(cache_duration) = self.cache_duration {
            response.set_cache_control(cache_duration.as_secs() as u32, true);
        response.set_body(content);
        response.set_status(StatusCode::OK);

        debug!(path = ?file_path, size = response.body.len(), "Served static file");
        Ok(())
    }
}

/// Middleware chain for composing multiple middleware
#[derive(Default)]
pub struct MiddlewareChain {
impl std::fmt::Debug for MiddlewareChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiddlewareChain")
            .field("middleware_count", &self.middleware.len())
            .finish()
    }
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self::default()
    pub fn add(&mut self, middleware: Arc<dyn Middleware>) {
        self.middleware.push(middleware);
        // Sort by priority
        self.middleware.sort_by_key(|m| m.priority());
    #[instrument(skip(self, context, response, handler))]
    pub async fn execute(
    ) -> HandlerResult {
        // Execute before_request middleware
        for middleware in &self.middleware {
            if let Err(e) = middleware.before_request(&mut context, &mut response).await {
                error!(middleware = middleware.name(), error = %e, "Middleware error in before_request");
                
                // Try to handle the error
                if let Err(handle_error) = middleware.on_error(&context, &mut response, &e).await {
                    error!(middleware = middleware.name(), error = %handle_error, "CursedError in middleware error handler");
                return Ok(response);
            // If response is marked as sent, skip remaining middleware and handler
            if response.is_sent() {
                debug!(middleware = middleware.name(), "Response sent by middleware, skipping remaining chain");
                return Ok(response);
            }
        }

        // Execute handler if no middleware sent response
        if !response.is_sent() {
            match handler.handle(&context, &mut response).await {
                Ok(_) => {}
                Err(e) => {
                    error!(error = %e, "Handler error");
                    response.set_status(StatusCode::InternalServerError);
                    response.set_text(&format!("Handler error: {}", e));
                }
            }
        // Execute after_response middleware in reverse order
        for middleware in self.middleware.iter().rev() {
            if let Err(e) = middleware.after_response(&context, &mut response).await {
                error!(middleware = middleware.name(), error = %e, "Middleware error in after_response");
                
                // Try to handle the error
                if let Err(handle_error) = middleware.on_error(&context, &mut response, &e).await {
                    error!(middleware = middleware.name(), error = %handle_error, "CursedError in middleware error handler");
                }
            }
        Ok(response)
    }
}

