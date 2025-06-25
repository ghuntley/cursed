use crate::web::StatusCode;
use crate::error::CursedError;
/// fr fr Middleware system for web_vibez - request/response processing pipeline
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};
use std::net::IpAddr;

// Placeholder imports disabled
// };

/// fr fr Middleware trait for request/response processing - pipeline component
pub trait Middleware: Send + Sync {
    /// fr fr Process request before handler - preprocessing
    fn before_request<'a>(&'a self, request: &'a mut HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>>;
    
    /// fr fr Process response after handler - postprocessing
    fn after_response<'a>(&'a self, request: &'a HttpRequest, response: &'a mut HttpResponse) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>>;
    
    /// fr fr Get middleware name for debugging - identification
    fn name(&self) -> &'static str {
        "Middleware"
    }
}

/// fr fr Middleware chain for composing multiple middleware - processing pipeline
#[derive(Clone)]
pub struct MiddlewareChain {
impl MiddlewareChain {
    /// fr fr Create new middleware chain - empty pipeline
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Add middleware to chain - extend pipeline
    pub fn add<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middleware.push(Arc::new(middleware));
        self
    /// fr fr Get middleware count - pipeline size
    pub fn len(&self) -> usize {
        self.middleware.len()
    /// fr fr Check if chain is empty - validation
    pub fn is_empty(&self) -> bool {
        self.middleware.is_empty()
    /// fr fr Process request through all middleware - complete preprocessing
    pub async fn process_request(&self, request: &mut HttpRequest) -> WebResult<()> {
        for middleware in &self.middleware {
            middleware.before_request(request).await?;
        }
        Ok(())
    /// fr fr Process response through all middleware - complete postprocessing  
    pub async fn process_response(&self, request: &HttpRequest, response: &mut HttpResponse) -> WebResult<()> {
        // Process in reverse order for response
        for middleware in self.middleware.iter().rev() {
            middleware.after_response(request, response).await?;
        }
        Ok(())
    }
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Middleware-aware handler wrapper - combines handler with middleware
pub struct MiddlewareHandler {
impl MiddlewareHandler {
    /// fr fr Create new middleware handler - wrap handler with middleware
    pub fn new<H: Handler + 'static>(handler: H, middleware_chain: MiddlewareChain) -> Self {
        Self {
        }
    }
impl Handler for MiddlewareHandler {
    fn handle(&self, mut request: HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send + '_>> {
        let handler = self.handler.clone();
        let middleware_chain = self.middleware_chain.clone();
        
        Box::pin(async move {
            // Process request through middleware
            middleware_chain.process_request(&mut request).await?;
            
            // Handle the request
            let mut response = handler.handle(request.clone()).await?;
            
            // Process response through middleware
            middleware_chain.process_response(&request, &mut response).await?;
            
            Ok(response)
        })
    fn name(&self) -> &'static str {
        "MiddlewareHandler"
    }
}

/// fr fr CORS middleware for cross-origin requests - security headers
#[derive(Clone)]
pub struct CorsMiddleware {
impl CorsMiddleware {
    /// fr fr Create new CORS middleware - basic setup
    pub fn new() -> Self {
        Self {
            max_age: Some(Duration::from_secs(86400)), // 24 hours
        }
    }

    /// fr fr Set allowed origins - who can access
    pub fn allowed_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    /// fr fr Set allowed methods - what methods are OK
    pub fn allowed_methods(mut self, methods: Vec<String>) -> Self {
        self.allowed_methods = methods;
        self
    /// fr fr Set allowed headers - what headers are OK
    pub fn allowed_headers(mut self, headers: Vec<String>) -> Self {
        self.allowed_headers = headers;
        self
    /// fr fr Set exposed headers - what headers client can see
    pub fn exposed_headers(mut self, headers: Vec<String>) -> Self {
        self.exposed_headers = headers;
        self
    /// fr fr Set max age for preflight cache - how long to cache
    pub fn max_age(mut self, duration: Duration) -> Self {
        self.max_age = Some(duration);
        self
    /// fr fr Allow credentials - cookies and auth
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    /// fr fr Create permissive CORS - allow everything (dev mode)
    pub fn permissive() -> Self {
        Self::new()
            .allowed_origins(Vec::from(["*".to_string()]))
            .allow_credentials(false)
    /// fr fr Check if origin is allowed - validation
    fn is_origin_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.contains(&"*".to_string()) || self.allowed_origins.contains(&origin.to_string())
    }
}

impl Middleware for CorsMiddleware {
    fn before_request<'a>(&'a self, _request: &'a mut HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        Box::pin(async { Ok(()) })
    fn after_response<'a>(&'a self, request: &'a HttpRequest, response: &'a mut HttpResponse) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        let allowed_origins = self.allowed_origins.clone();
        let allowed_methods = self.allowed_methods.clone();
        let allowed_headers = self.allowed_headers.clone();
        let exposed_headers = self.exposed_headers.clone();
        let max_age = self.max_age;
        let allow_credentials = self.allow_credentials;
        
        Box::pin(async move {
            // Get origin from request
            if let Some(origin) = request.header("origin") {
                if CorsMiddleware::is_origin_allowed(&CorsMiddleware::new().allowed_origins(allowed_origins.clone()), origin) {
                    if allowed_origins.contains(&"*".to_string()) && !allow_credentials {
                        response.headers.insert("access-control-allow-origin".to_string(), "*".to_string());
                    } else {
                        response.headers.insert("access-control-allow-origin".to_string(), origin.clone());
                    }
                }
            // Add allowed methods
            if !allowed_methods.is_empty() {
                response.headers.insert(
                    allowed_methods.join(", ")
                );
            // Add allowed headers
            if !allowed_headers.is_empty() {
                response.headers.insert(
                    allowed_headers.join(", ")
                );
            // Add exposed headers
            if !exposed_headers.is_empty() {
                response.headers.insert(
                    exposed_headers.join(", ")
                );
            // Add max age for preflight
            if let Some(max_age) = max_age {
                response.headers.insert(
                    max_age.as_secs().to_string()
                );
            // Add credentials flag
            if allow_credentials {
                response.headers.insert(
                    "true".to_string()
                );
            Ok(())
        })
    fn name(&self) -> &'static str {
        "CorsMiddleware"
    }
}

impl Default for CorsMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Logging middleware for request/response tracking - observability
#[derive(Clone)]
pub struct LoggingMiddleware {
impl LoggingMiddleware {
    /// fr fr Create new logging middleware - basic setup
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Enable/disable request logging - control verbosity
    pub fn log_requests(mut self, enable: bool) -> Self {
        self.log_requests = enable;
        self
    /// fr fr Enable/disable response logging - control verbosity
    pub fn log_responses(mut self, enable: bool) -> Self {
        self.log_responses = enable;
        self
    /// fr fr Enable/disable header logging - detailed info
    pub fn log_headers(mut self, enable: bool) -> Self {
        self.log_headers = enable;
        self
    /// fr fr Enable/disable body logging - sensitive data
    pub fn log_body(mut self, enable: bool) -> Self {
        self.log_body = enable;
        self
    }
}

impl Middleware for LoggingMiddleware {
    fn before_request<'a>(&'a self, request: &'a mut HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        let log_requests = self.log_requests;
        let log_headers = self.log_headers;
        let log_body = self.log_body;
        
        Box::pin(async move {
            if log_requests {
                let start_time = Instant::now();
                request.set_extension("start_time".to_string(), serde_json::json!(start_time.elapsed().as_millis()));

                println!("→ {} {} {}", request.method, request.path, request.version);
                
                if let Some(client_ip) = request.client_ip() {
                    println!("  Client: {}", client_ip);
                if log_headers && !request.headers.is_empty() {
                    println!("  Headers:");
                    for (name, value) in &request.headers {
                        println!("    {}: {}", name, value);
                    }
                }

                if log_body && request.has_body() {
                    if let Ok(body_text) = request.text() {
                        let preview = if body_text.len() > 200 {
                            format!("{}...", &body_text[..200])
                        } else {
                            body_text
                        println!("  Body: {}", preview);
                    }
                }
            }
            Ok(())
        })
    fn after_response<'a>(&'a self, request: &'a HttpRequest, response: &'a mut HttpResponse) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        let log_responses = self.log_responses;
        let log_headers = self.log_headers;
        let log_body = self.log_body;
        
        Box::pin(async move {
            if log_responses {
                let duration = if let Some(start_time) = request.extension("start_time") {
                    format!(" ({}ms)", start_time.as_u64().unwrap_or(0))
                } else {
                    String::new()

                println!("← {} {}{}", response.status, response.content_length(), duration);

                if log_headers && !response.headers.is_empty() {
                    println!("  Headers:");
                    for (name, value) in &response.headers {
                        println!("    {}: {}", name, value);
                    }
                }

                if log_body && !response.body.is_empty() {
                    if let Ok(body_text) = response.body_text() {
                        let preview = if body_text.len() > 200 {
                            format!("{}...", &body_text[..200])
                        } else {
                            body_text
                        println!("  Body: {}", preview);
                    }
                }
            }
            Ok(())
        })
    fn name(&self) -> &'static str {
        "LoggingMiddleware"
    }
}

impl Default for LoggingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Security headers middleware - common security headers
#[derive(Clone)]
pub struct SecurityHeadersMiddleware {
impl SecurityHeadersMiddleware {
    /// fr fr Create new security headers middleware - basic security
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Set Content Security Policy - XSS protection
    pub fn content_security_policy(mut self, policy: Option<String>) -> Self {
        self.content_security_policy = policy;
        self
    /// fr fr Set X-Frame-Options - clickjacking protection
    pub fn x_frame_options(mut self, value: Option<String>) -> Self {
        self.x_frame_options = value;
        self
    /// fr fr Enable/disable X-Content-Type-Options - MIME sniffing protection
    pub fn x_content_type_options(mut self, enable: bool) -> Self {
        self.x_content_type_options = enable;
        self
    /// fr fr Set Strict Transport Security - HTTPS enforcement
    pub fn strict_transport_security(mut self, value: Option<String>) -> Self {
        self.strict_transport_security = value;
        self
    }
}

impl Middleware for SecurityHeadersMiddleware {
    fn before_request<'a>(&'a self, _request: &'a mut HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        Box::pin(async { Ok(()) })
    fn after_response<'a>(&'a self, _request: &'a HttpRequest, response: &'a mut HttpResponse) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        let csp = self.content_security_policy.clone();
        let x_frame = self.x_frame_options.clone();
        let x_content_type = self.x_content_type_options;
        let x_xss = self.x_xss_protection;
        let hsts = self.strict_transport_security.clone();
        let referrer = self.referrer_policy.clone();
        
        Box::pin(async move {
            if let Some(csp) = csp {
                response.headers.insert("content-security-policy".to_string(), csp);
            if let Some(x_frame) = x_frame {
                response.headers.insert("x-frame-options".to_string(), x_frame);
            if x_content_type {
                response.headers.insert("x-content-type-options".to_string(), "nosniff".to_string());
            if x_xss {
                response.headers.insert("x-xss-protection".to_string(), "1; mode=block".to_string());
            if let Some(hsts) = hsts {
                response.headers.insert("strict-transport-security".to_string(), hsts);
            if let Some(referrer) = referrer {
                response.headers.insert("referrer-policy".to_string(), referrer);
            Ok(())
        })
    fn name(&self) -> &'static str {
        "SecurityHeadersMiddleware"
    }
}

impl Default for SecurityHeadersMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Rate limiting middleware - production-ready request throttling
// Placeholder imports disabled
// };

#[derive(Clone)]
pub struct RateLimitMiddleware {
impl RateLimitMiddleware {
    /// fr fr Create new rate limit middleware - comprehensive setup
    pub fn new(config: RateLimitConfig) -> Self {
        let store = Arc::new(InMemoryStore::new());
        let algorithm = Arc::new(FixedWindow::new());
        let limiter = Arc::new(RateLimiter::new(store, algorithm, config.clone()));
        
        Self {
        }
    }

    /// fr fr Create with custom limiter - advanced setup
    pub fn with_limiter(limiter: Arc<RateLimiter>, config: RateLimitConfig) -> Self {
        Self {
        }
    }

    /// fr fr Create rate limiter per minute - common setup
    pub fn per_minute(max_requests: u64) -> Self {
        let config = RateLimitConfig::per_minute(max_requests);
        Self::new(config)
    /// fr fr Create rate limiter per hour - lenient setup
    pub fn per_hour(max_requests: u64) -> Self {
        let config = RateLimitConfig::per_hour(max_requests);
        Self::new(config)
    /// fr fr Create rate limiter per second - strict setup
    pub fn per_second(max_requests: u64) -> Self {
        let config = RateLimitConfig::per_second(max_requests);
        Self::new(config)
    /// fr fr Create sliding window rate limiter - smooth limiting
    pub fn sliding_window(max_requests: u64, duration: Duration) -> Self {
        let config = RateLimitConfig::new(max_requests, duration)
            .with_sliding_window(duration);
        Self::new(config)
    /// fr fr Create token bucket rate limiter - burst-friendly
    pub fn token_bucket(max_requests: u64, window: Duration, capacity: f64, refill_rate: f64) -> Self {
        let config = RateLimitConfig::new(max_requests, window)
            .with_token_bucket(capacity, refill_rate);
        Self::new(config)
    /// fr fr Extract client identifier from request - configurable identification
    fn extract_client_id(&self, request: &HttpRequest) -> String {
        match &self.client_identification {
            ClientIdentification::IpAddress => {
                extract_client_id(request.client_ip())
            }
            ClientIdentification::Header { name } => {
                request.header(name)
                    .map(|h| h.clone())
                    .unwrap_or_else(|| extract_client_id(request.client_ip()))
            }
            ClientIdentification::Composite { factors } => {
                let mut parts = Vec::new();
                for factor in factors {
                    match factor {
//                         crate::stdlib::packages::web_vibez::ratelimit::IdentificationFactor::IpAddress => {
                            parts.push(extract_client_id(request.client_ip()));
                        }
//                         crate::stdlib::packages::web_vibez::ratelimit::IdentificationFactor::Header { name } => {
                            if let Some(value) = request.header(name) {
                                parts.push(value.clone());
                            }
                        }
//                         crate::stdlib::packages::web_vibez::ratelimit::IdentificationFactor::UserAgent => {
                            if let Some(ua) = request.header("user-agent") {
                                parts.push(ua.clone());
                            }
                        }
//                         crate::stdlib::packages::web_vibez::ratelimit::IdentificationFactor::Custom { name: _, extractor: _ } => {
                            // Custom extractors would be implemented here
                            parts.push("custom".to_string());
                        }
                    }
                }
                if parts.is_empty() {
                    extract_client_id(request.client_ip())
                } else {
                    parts.join(":")
                }
            }
            ClientIdentification::Custom { identifier } => {
                // Custom identification logic would be implemented here
                format!("custom:{}", identifier)
            }
        }
    /// fr fr Add rate limit headers to response
    fn add_rate_limit_headers(&self, response: &mut HttpResponse, decision: &RateLimitDecision, client_id: &str) {
        if !self.config.error_config.include_headers {
            return;
        match decision {
            RateLimitDecision::Allow { remaining, reset_time, .. } => {
                response.headers.insert("x-ratelimit-limit".to_string(), self.config.max_requests.to_string());
                response.headers.insert("x-ratelimit-remaining".to_string(), remaining.to_string());
                response.headers.insert("x-ratelimit-reset".to_string(), reset_time.to_string());
                response.headers.insert("x-ratelimit-policy".to_string(), format!("{};w={}", self.config.max_requests, match &self.config.window_config {
                }));
            }
            RateLimitDecision::Deny { retry_after, reset_time } => {
                response.headers.insert("x-ratelimit-limit".to_string(), self.config.max_requests.to_string());
                response.headers.insert("x-ratelimit-remaining".to_string(), "0".to_string());
                response.headers.insert("x-ratelimit-reset".to_string(), reset_time.to_string());
                
                if self.config.error_config.include_retry_after {
                    response.headers.insert("retry-after".to_string(), retry_after.to_string());
                }
            }
        // Add client identifier for debugging (if not sensitive)
        if matches!(self.client_identification, ClientIdentification::IpAddress) {
            response.headers.insert("x-ratelimit-scope".to_string(), client_id.to_string());
        }
    }

    /// fr fr Get rate limiter metrics - monitoring
//     pub async fn get_metrics(&self) -> crate::stdlib::packages::web_vibez::ratelimit::RateLimitMetrics {
        self.limiter.get_metrics().await
    /// fr fr Reset client rate limit - administrative function
//     pub async fn reset_client(&self, client_id: &str) -> crate::stdlib::packages::web_vibez::ratelimit::RateLimitResult<()> {
        self.limiter.reset_client(client_id).await
    }
}

impl Middleware for RateLimitMiddleware {
    fn before_request<'a>(&'a self, request: &'a mut HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        let client_id = self.extract_client_id(request);
        let limiter = self.limiter.clone();
        let error_config = self.config.error_config.clone();
        
        Box::pin(async move {
            match limiter.check_request(&client_id).await {
                Ok(RateLimitDecision::Allow { .. }) => {
                    // Request allowed, continue processing
                    Ok(())
                }
                Ok(RateLimitDecision::Deny { retry_after, .. }) => {
                    // Request denied, return rate limit error
                    let error_response = if let Some(custom_response) = &error_config.custom_response {
                        custom_response.clone()
                    } else {
                        serde_json::json!({
                            "client_id": client_id
                        }).to_string()
                    
                    Err(WebError::RateLimit {
                    })
                }
                Err(rate_limit_error) => {
                    // Internal error in rate limiting system
                    eprintln!("Rate limiting error for client {}: {}", client_id, rate_limit_error);
                    
                    // Fail open - allow request but log error
                    Ok(())
                }
            }
        })
    fn after_response<'a>(&'a self, request: &'a HttpRequest, response: &'a mut HttpResponse) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        let client_id = self.extract_client_id(request);
        let limiter = self.limiter.clone();
        let config = self.config.clone();
        
        Box::pin(async move {
            // Get current rate limit status and add headers
            if let Ok(context) = limiter.get_context(&client_id).await {
                let decision = if context.remaining > 0 {
                    RateLimitDecision::Allow {
                    }
                } else {
                    RateLimitDecision::Deny {
                    }
                
                if config.error_config.include_headers {
                    self.add_rate_limit_headers(response, &decision, &client_id);
                }
            }
            
            Ok(())
        })
    fn name(&self) -> &'static str {
        "RateLimitMiddleware"
    }
}

