use crate::web::StatusCode;
/// Middleware system for HTTP request/response processing
/// 
/// Provides a flexible middleware chain system with common middleware
/// implementations for authentication, logging, CORS, rate limiting, and more

use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext, ContextData};
use crate::stdlib::web_vibez::handlers::{RequestHandler, HandlerResult};
use crate::stdlib::web_vibez::{HttpMethod, StatusCode};
use crate::stdlib::web_vibez::error_handling::MiddlewareError;

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
        &self,
        context: &mut RequestContext,
        response: &mut ResponseContext,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// Process response after handler
    async fn after_response(
        &self,
        context: &RequestContext,
        response: &mut ResponseContext,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// Handle middleware error
    async fn on_error(
        &self,
        context: &RequestContext,
        response: &mut ResponseContext,
        error: &MiddlewareError,
    ) -> Result<(), Error> {
        let error_msg = format!("Middleware error: {}", error);
        response.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        response.set_text(&error_msg);
        Ok(())
    }

    /// Get middleware name for debugging
    fn name(&self) -> &'static str {
        "Unknown"
    }

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
    schemes: Vec<AuthScheme>,
    /// Optional paths that don't require authentication
    skip_paths: Vec<String>,
    /// Custom authentication validator
    validator: Option<Arc<dyn AuthValidator>>,
}

#[derive(Debug, Clone)]
pub enum AuthScheme {
    Basic,
    Bearer,
    ApiKey { header: String },
    Custom { name: String },
}

#[async_trait::async_trait]
pub trait AuthValidator: Send + Sync {
    async fn validate_credentials(&self, scheme: &AuthScheme, credentials: &str) -> bool;
}

impl AuthMiddleware {
    pub fn new(schemes: Vec<AuthScheme>) -> Self {
        Self {
            schemes,
            skip_paths: Vec::new(),
            validator: None,
        }
    }

    pub fn with_skip_paths(mut self, paths: Vec<String>) -> Self {
        self.skip_paths = paths;
        self
    }

    pub fn with_validator(mut self, validator: Arc<dyn AuthValidator>) -> Self {
        self.validator = Some(validator);
        self
    }
}

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn before_request(
        &self,
        context: &mut RequestContext,
        response: &mut ResponseContext,
    ) -> MiddlewareResult {
        // Check if path should skip authentication
        if self.skip_paths.iter().any(|skip_path| context.path.starts_with(skip_path)) {
            return Ok(());
        }

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
        }

        Err(MiddlewareError::Authentication("Invalid credentials".to_string()))
    }

    async fn on_error(
        &self,
        _context: &RequestContext,
        response: &mut ResponseContext,
        error: &MiddlewareError,
    ) -> MiddlewareResult {
        match error {
            MiddlewareError::Authentication(_) => {
                response.set_status(StatusCode::UNAUTHORIZED);
                response.set_header("WWW-Authenticate", "Basic");
                response.set_text("Authentication required");
            }
            _ => {
                response.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                response.set_text("Internal server error");
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Auth"
    }

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
    log_level: LogLevel,
    /// Include request body in logs
    log_request_body: bool,
    /// Include response body in logs
    log_response_body: bool,
    /// Maximum body size to log
    max_body_log_size: usize,
    /// Skip logging for specific paths
    skip_paths: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Debug,
    Trace,
}

impl LoggingMiddleware {
    pub fn new() -> Self {
        Self {
            log_level: LogLevel::Info,
            log_request_body: false,
            log_response_body: false,
            max_body_log_size: 1024,
            skip_paths: Vec::from(["/health".to_string(), "/metrics".to_string()]),
        }
    }

    pub fn with_body_logging(mut self, request: bool, response: bool) -> Self {
        self.log_request_body = request;
        self.log_response_body = response;
        self
    }

    pub fn with_skip_paths(mut self, paths: Vec<String>) -> Self {
        self.skip_paths = paths;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn before_request(
        &self,
        context: &mut RequestContext,
        _response: &mut ResponseContext,
    ) -> MiddlewareResult {
        if self.skip_paths.iter().any(|skip_path| context.path.starts_with(skip_path)) {
            return Ok(());
        }

        let body_preview = if self.log_request_body && !context.body.is_empty() {
            let preview_size = std::cmp::min(context.body.len(), self.max_body_log_size);
            match String::from_utf8(context.body[..preview_size].to_vec()) {
                Ok(body_str) => Some(body_str),
                Err(_) => Some(format!("<binary data {} bytes>", context.body.len())),
            }
        } else {
            None
        };

        match self.log_level {
            LogLevel::Info => {
                info!(
                    request_id = %context.request_id,
                    method = %context.method,
                    path = %context.path,
                    user_agent = ?context.user_agent,
                    client_ip = ?context.client_ip,
                    "Incoming request"
                );
            }
            LogLevel::Debug => {
                debug!(
                    request_id = %context.request_id,
                    method = %context.method,
                    path = %context.path,
                    headers = ?context.headers,
                    query_params = ?context.query_params,
                    body_preview = ?body_preview,
                    "Incoming request with details"
                );
            }
            LogLevel::Trace => {
                let full_body = if self.log_request_body {
                    String::from_utf8_lossy(&context.body).to_string()
                } else {
                    format!("<{} bytes>", context.body.len())
                };
                
                tracing::trace!(
                    request_id = %context.request_id,
                    method = %context.method,
                    path = %context.path,
                    headers = ?context.headers,
                    query_params = ?context.query_params,
                    route_params = ?context.route_params,
                    body = %full_body,
                    "Detailed request trace"
                );
            }
        }

        Ok(())
    }

    async fn after_response(
        &self,
        context: &RequestContext,
        response: &mut ResponseContext,
    ) -> MiddlewareResult {
        if self.skip_paths.iter().any(|skip_path| context.path.starts_with(skip_path)) {
            return Ok(());
        }

        let elapsed = context.elapsed();
        let body_preview = if self.log_response_body && !response.body.is_empty() {
            let preview_size = std::cmp::min(response.body.len(), self.max_body_log_size);
            match String::from_utf8(response.body[..preview_size].to_vec()) {
                Ok(body_str) => Some(body_str),
                Err(_) => Some(format!("<binary data {} bytes>", response.body.len())),
            }
        } else {
            None
        };

        match self.log_level {
            LogLevel::Info => {
                info!(
                    request_id = %context.request_id,
                    method = %context.method,
                    path = %context.path,
                    status = %response.status.0,
                    elapsed_ms = elapsed.as_millis(),
                    response_size = response.body.len(),
                    "Request completed"
                );
            }
            LogLevel::Debug => {
                debug!(
                    request_id = %context.request_id,
                    method = %context.method,
                    path = %context.path,
                    status = %response.status.0,
                    elapsed_ms = elapsed.as_millis(),
                    response_headers = ?response.headers,
                    response_size = response.body.len(),
                    body_preview = ?body_preview,
                    "Request completed with details"
                );
            }
            LogLevel::Trace => {
                let full_body = if self.log_response_body {
                    String::from_utf8_lossy(&response.body).to_string()
                } else {
                    format!("<{} bytes>", response.body.len())
                };
                
                tracing::trace!(
                    request_id = %context.request_id,
                    method = %context.method,
                    path = %context.path,
                    status = %response.status.0,
                    elapsed_ms = elapsed.as_millis(),
                    response_headers = ?response.headers,
                    response_body = %full_body,
                    "Detailed response trace"
                );
            }
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Logging"
    }

    fn priority(&self) -> u32 {
        5 // Very high priority - log everything
    }
}

/// CORS middleware for cross-origin resource sharing
#[derive(Debug)]
pub struct CorsMiddleware {
    /// Allowed origins
    allowed_origins: Vec<String>,
    /// Allowed methods
    allowed_methods: Vec<HttpMethod>,
    /// Allowed headers
    allowed_headers: Vec<String>,
    /// Exposed headers
    exposed_headers: Vec<String>,
    /// Allow credentials
    allow_credentials: bool,
    /// Max age for preflight requests
    max_age: Option<Duration>,
}

impl CorsMiddleware {
    pub fn new() -> Self {
        Self {
            allowed_origins: Vec::from(["*".to_string()]),
            allowed_methods: vec![
                HttpMethod::GET,
                HttpMethod::POST,
                HttpMethod::PUT,
                HttpMethod::DELETE,
                HttpMethod::OPTIONS,
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
            ],
            exposed_headers: Vec::new(),
            allow_credentials: false,
            max_age: Some(Duration::from_secs(3600)), // 1 hour
        }
    }

    pub fn with_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    }

    pub fn with_methods(mut self, methods: Vec<HttpMethod>) -> Self {
        self.allowed_methods = methods;
        self
    }

    pub fn with_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for CorsMiddleware {
    async fn before_request(
        &self,
        context: &mut RequestContext,
        response: &mut ResponseContext,
    ) -> MiddlewareResult {
        // Handle preflight OPTIONS request
        if context.method == HttpMethod::OPTIONS {
            self.set_cors_headers(context, response);
            response.set_status(StatusCode::NO_CONTENT);
            response.mark_sent(); // Skip further processing
        }

        Ok(())
    }

    async fn after_response(
        &self,
        context: &RequestContext,
        response: &mut ResponseContext,
    ) -> MiddlewareResult {
        if context.method != HttpMethod::OPTIONS {
            self.set_cors_headers(context, response);
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "CORS"
    }

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
        }

        // Set credentials
        if self.allow_credentials {
            response.set_header("Access-Control-Allow-Credentials", "true");
        }

        // Set max age
        if let Some(max_age) = self.max_age {
            response.set_header("Access-Control-Max-Age", &max_age.as_secs().to_string());
        }
    }
}

/// Rate limiting middleware
#[derive(Debug)]
pub struct RateLimitMiddleware {
    /// Requests per time window
    requests_per_window: u32,
    /// Time window duration
    window_duration: Duration,
    /// Rate limiter storage
    limiter: Arc<Mutex<RateLimiter>>,
    /// Key extractor function
    key_extractor: fn(&RequestContext) -> String,
    /// Skip rate limiting for specific paths
    skip_paths: Vec<String>,
}

#[derive(Debug)]
struct RateLimiter {
    /// Request counts per key
    requests: HashMap<String, RequestCount>,
    /// Last cleanup time
    last_cleanup: Instant,
}

#[derive(Debug)]
struct RequestCount {
    count: u32,
    window_start: Instant,
}

impl RateLimitMiddleware {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_window: requests_per_minute,
            window_duration: Duration::from_secs(60),
            limiter: Arc::new(Mutex::new(RateLimiter {
                requests: HashMap::new(),
                last_cleanup: Instant::now(),
            })),
            key_extractor: |context| {
                context.client_ip.clone().unwrap_or_else(|| "unknown".to_string())
            },
            skip_paths: Vec::from(["/health".to_string()]),
        }
    }

    pub fn with_window(mut self, requests: u32, duration: Duration) -> Self {
        self.requests_per_window = requests;
        self.window_duration = duration;
        self
    }

    pub fn with_key_extractor(mut self, extractor: fn(&RequestContext) -> String) -> Self {
        self.key_extractor = extractor;
        self
    }

    pub fn with_skip_paths(mut self, paths: Vec<String>) -> Self {
        self.skip_paths = paths;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimitMiddleware {
    async fn before_request(
        &self,
        context: &mut RequestContext,
        response: &mut ResponseContext,
    ) -> MiddlewareResult {
        if self.skip_paths.iter().any(|skip_path| context.path.starts_with(skip_path)) {
            return Ok(());
        }

        let key = (self.key_extractor)(context);
        let now = Instant::now();

        let mut limiter = self.limiter.lock().unwrap();
        
        // Cleanup old entries periodically
        if now.duration_since(limiter.last_cleanup) > Duration::from_secs(300) {
            limiter.requests.retain(|_, count| {
                now.duration_since(count.window_start) < self.window_duration
            });
            limiter.last_cleanup = now;
        }

        // Check rate limit
        let request_count = limiter.requests.entry(key.clone()).or_insert_with(|| {
            RequestCount {
                count: 0,
                window_start: now,
            }
        });

        // Reset window if expired
        if now.duration_since(request_count.window_start) > self.window_duration {
            request_count.count = 0;
            request_count.window_start = now;
        }

        // Check if limit exceeded
        if request_count.count >= self.requests_per_window {
            let reset_time = request_count.window_start + self.window_duration;
            let retry_after = reset_time.duration_since(now).as_secs();
            
            response.set_status(StatusCode::TOO_MANY_REQUESTS);
            response.set_header("Retry-After", &retry_after.to_string());
            response.set_header("X-RateLimit-Limit", &self.requests_per_window.to_string());
            response.set_header("X-RateLimit-Remaining", "0");
            response.set_header("X-RateLimit-Reset", &reset_time.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());
            response.set_text("Rate limit exceeded");
            
            return Err(MiddlewareError::RateLimit(format!("Rate limit exceeded for key: {}", key)));
        }

        // Increment counter
        request_count.count += 1;

        // Set rate limit headers
        response.set_header("X-RateLimit-Limit", &self.requests_per_window.to_string());
        response.set_header("X-RateLimit-Remaining", &(self.requests_per_window - request_count.count).to_string());
        let reset_time = request_count.window_start + self.window_duration;
        response.set_header("X-RateLimit-Reset", &reset_time.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string());

        Ok(())
    }

    fn name(&self) -> &'static str {
        "RateLimit"
    }

    fn priority(&self) -> u32 {
        20 // High priority - check limits early
    }
}

/// Static file serving middleware
#[derive(Debug)]
pub struct StaticFileMiddleware {
    /// Root directory for static files
    root_dir: PathBuf,
    /// URL prefix to match
    url_prefix: String,
    /// Cache duration for static files
    cache_duration: Option<Duration>,
    /// Index files to serve for directories
    index_files: Vec<String>,
    /// MIME type mapping
    mime_types: HashMap<String, String>,
}

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
            root_dir,
            url_prefix: url_prefix.to_string(),
            cache_duration: Some(Duration::from_secs(3600)), // 1 hour
            index_files: Vec::from(["index.html".to_string(), "index.htm".to_string()]),
            mime_types,
        }
    }

    pub fn with_cache_duration(mut self, duration: Option<Duration>) -> Self {
        self.cache_duration = duration;
        self
    }

    pub fn with_index_files(mut self, files: Vec<String>) -> Self {
        self.index_files = files;
        self
    }
}

#[async_trait::async_trait]
impl Middleware for StaticFileMiddleware {
    async fn before_request(
        &self,
        context: &mut RequestContext,
        response: &mut ResponseContext,
    ) -> MiddlewareResult {
        // Only handle GET requests
        if context.method != HttpMethod::GET {
            return Ok(());
        }

        // Check if path matches our prefix
        if !context.path.starts_with(&self.url_prefix) {
            return Ok(());
        }

        // Extract file path
        let relative_path = &context.path[self.url_prefix.len()..];
        let file_path = self.root_dir.join(relative_path.trim_start_matches('/'));

        // Security check: prevent directory traversal
        if !file_path.starts_with(&self.root_dir) {
            return Err(MiddlewareError::Security("Directory traversal attempt".to_string()));
        }

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
    }

    fn name(&self) -> &'static str {
        "StaticFile"
    }

    fn priority(&self) -> u32 {
        200 // Low priority - handle after other middleware
    }
}

impl StaticFileMiddleware {
    async fn serve_file(&self, file_path: &Path, response: &mut ResponseContext) -> Result<(), Error> {
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
        }

        response.set_body(content);
        response.set_status(StatusCode::OK);

        debug!(path = ?file_path, size = response.body.len(), "Served static file");
        Ok(())
    }
}

/// Middleware chain for composing multiple middleware
#[derive(Default)]
pub struct MiddlewareChain {
    middleware: Vec<Arc<dyn Middleware>>,
}

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
    }

    pub fn add(&mut self, middleware: Arc<dyn Middleware>) {
        self.middleware.push(middleware);
        // Sort by priority
        self.middleware.sort_by_key(|m| m.priority());
    }

    #[instrument(skip(self, context, response, handler))]
    pub async fn execute(
        &self,
        mut context: RequestContext,
        mut response: ResponseContext,
        handler: Arc<dyn RequestHandler>,
    ) -> HandlerResult {
        // Execute before_request middleware
        for middleware in &self.middleware {
            if let Err(e) = middleware.before_request(&mut context, &mut response).await {
                error!(middleware = middleware.name(), error = %e, "Middleware error in before_request");
                
                // Try to handle the error
                if let Err(handle_error) = middleware.on_error(&context, &mut response, &e).await {
                    error!(middleware = middleware.name(), error = %handle_error, "Error in middleware error handler");
                }
                
                return Ok(response);
            }

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
                    response.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                    response.set_text(&format!("Handler error: {}", e));
                }
            }
        }

        // Execute after_response middleware in reverse order
        for middleware in self.middleware.iter().rev() {
            if let Err(e) = middleware.after_response(&context, &mut response).await {
                error!(middleware = middleware.name(), error = %e, "Middleware error in after_response");
                
                // Try to handle the error
                if let Err(handle_error) = middleware.on_error(&context, &mut response, &e).await {
                    error!(middleware = middleware.name(), error = %handle_error, "Error in middleware error handler");
                }
            }
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::web_vibez::handlers::StaticHandler;

    #[tokio::test]
    async fn test_auth_middleware() {
        let middleware = AuthMiddleware::new(Vec::from([AuthScheme::Bearer]));
        let mut context = RequestContext::new("GET".to_string(), "/protected".to_string());
        let mut response = ResponseContext::new();

        // Test missing auth header
        let result = middleware.before_request(&mut context, &mut response).await;
        assert!(result.is_err());

        // Test valid auth header
        context.add_header("Authorization", "Bearer token123");
        let result = middleware.before_request(&mut context, &mut response).await;
        assert!(result.is_ok());
        assert_eq!(context.get_data("authenticated").unwrap().as_boolean(), Some(true));
    }

    #[tokio::test]
    async fn test_cors_middleware() {
        let middleware = CorsMiddleware::new();
        let mut context = RequestContext::new("OPTIONS".to_string(), "/api/test".to_string());
        context.add_header("Origin", "https://example.com");
        let mut response = ResponseContext::new();

        let result = middleware.before_request(&mut context, &mut response).await;
        assert!(result.is_ok());
        assert!(response.is_sent()); // Preflight should be handled
        assert_eq!(response.status, StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_rate_limit_middleware() {
        let middleware = RateLimitMiddleware::new(2); // 2 requests per minute
        let mut context = RequestContext::new("GET".to_string(), "/api/test".to_string());
        context.set_client_ip("127.0.0.1".to_string());
        
        // First request should pass
        let mut response = ResponseContext::new();
        let result = middleware.before_request(&mut context, &mut response).await;
        assert!(result.is_ok());

        // Second request should pass
        let mut response = ResponseContext::new();
        let result = middleware.before_request(&mut context, &mut response).await;
        assert!(result.is_ok());

        // Third request should be rate limited
        let mut response = ResponseContext::new();
        let result = middleware.before_request(&mut context, &mut response).await;
        assert!(result.is_err());
        assert_eq!(response.status, StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn test_middleware_chain() {
        let logging = Arc::new(LoggingMiddleware::new());
        let cors = Arc::new(CorsMiddleware::new());
        let handler = Arc::new(StaticHandler::new("Test response"));

        let mut chain = MiddlewareChain::new();
        chain.add(logging);
        chain.add(cors);
        
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        let response = ResponseContext::new();

        let result = chain.execute(context, response, handler).await;
        assert!(result.is_ok());
    }
}
