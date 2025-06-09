/// fr fr Middleware system for web_vibez - request/response processing pipeline
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};

use crate::stdlib::packages::web_vibez::{
    request::HttpRequest,
    response::HttpResponse,
    handler::Handler,
    error::{WebError, WebResult},
    types::Headers,
};

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
    middleware: Vec<Arc<dyn Middleware>>,
}

impl MiddlewareChain {
    /// fr fr Create new middleware chain - empty pipeline
    pub fn new() -> Self {
        Self {
            middleware: Vec::new(),
        }
    }

    /// fr fr Add middleware to chain - extend pipeline
    pub fn add<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middleware.push(Arc::new(middleware));
        self
    }

    /// fr fr Get middleware count - pipeline size
    pub fn len(&self) -> usize {
        self.middleware.len()
    }

    /// fr fr Check if chain is empty - validation
    pub fn is_empty(&self) -> bool {
        self.middleware.is_empty()
    }

    /// fr fr Process request through all middleware - complete preprocessing
    pub async fn process_request(&self, request: &mut HttpRequest) -> WebResult<()> {
        for middleware in &self.middleware {
            middleware.before_request(request).await?;
        }
        Ok(())
    }

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
    handler: Arc<dyn Handler>,
    middleware_chain: MiddlewareChain,
}

impl MiddlewareHandler {
    /// fr fr Create new middleware handler - wrap handler with middleware
    pub fn new<H: Handler + 'static>(handler: H, middleware_chain: MiddlewareChain) -> Self {
        Self {
            handler: Arc::new(handler),
            middleware_chain,
        }
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
    }

    fn name(&self) -> &'static str {
        "MiddlewareHandler"
    }
}

/// fr fr CORS middleware for cross-origin requests - security headers
#[derive(Clone)]
pub struct CorsMiddleware {
    allowed_origins: Vec<String>,
    allowed_methods: Vec<String>,
    allowed_headers: Vec<String>,
    exposed_headers: Vec<String>,
    max_age: Option<Duration>,
    allow_credentials: bool,
}

impl CorsMiddleware {
    /// fr fr Create new CORS middleware - basic setup
    pub fn new() -> Self {
        Self {
            allowed_origins: Vec::from(["*".to_string()]),
            allowed_methods: Vec::from(["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()]),
            allowed_headers: Vec::from(["Content-Type".to_string(), "Authorization".to_string()]),
            exposed_headers: Vec::new(),
            max_age: Some(Duration::from_secs(86400)), // 24 hours
            allow_credentials: false,
        }
    }

    /// fr fr Set allowed origins - who can access
    pub fn allowed_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    }

    /// fr fr Set allowed methods - what methods are OK
    pub fn allowed_methods(mut self, methods: Vec<String>) -> Self {
        self.allowed_methods = methods;
        self
    }

    /// fr fr Set allowed headers - what headers are OK
    pub fn allowed_headers(mut self, headers: Vec<String>) -> Self {
        self.allowed_headers = headers;
        self
    }

    /// fr fr Set exposed headers - what headers client can see
    pub fn exposed_headers(mut self, headers: Vec<String>) -> Self {
        self.exposed_headers = headers;
        self
    }

    /// fr fr Set max age for preflight cache - how long to cache
    pub fn max_age(mut self, duration: Duration) -> Self {
        self.max_age = Some(duration);
        self
    }

    /// fr fr Allow credentials - cookies and auth
    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }

    /// fr fr Create permissive CORS - allow everything (dev mode)
    pub fn permissive() -> Self {
        Self::new()
            .allowed_origins(Vec::from(["*".to_string()]))
            .allow_credentials(false)
    }

    /// fr fr Check if origin is allowed - validation
    fn is_origin_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.contains(&"*".to_string()) || self.allowed_origins.contains(&origin.to_string())
    }
}

impl Middleware for CorsMiddleware {
    fn before_request<'a>(&'a self, _request: &'a mut HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        Box::pin(async { Ok(()) })
    }

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
            }

            // Add allowed methods
            if !allowed_methods.is_empty() {
                response.headers.insert(
                    "access-control-allow-methods".to_string(),
                    allowed_methods.join(", ")
                );
            }

            // Add allowed headers
            if !allowed_headers.is_empty() {
                response.headers.insert(
                    "access-control-allow-headers".to_string(),
                    allowed_headers.join(", ")
                );
            }

            // Add exposed headers
            if !exposed_headers.is_empty() {
                response.headers.insert(
                    "access-control-expose-headers".to_string(),
                    exposed_headers.join(", ")
                );
            }

            // Add max age for preflight
            if let Some(max_age) = max_age {
                response.headers.insert(
                    "access-control-max-age".to_string(),
                    max_age.as_secs().to_string()
                );
            }

            // Add credentials flag
            if allow_credentials {
                response.headers.insert(
                    "access-control-allow-credentials".to_string(),
                    "true".to_string()
                );
            }

            Ok(())
        })
    }

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
    log_requests: bool,
    log_responses: bool,
    log_headers: bool,
    log_body: bool,
}

impl LoggingMiddleware {
    /// fr fr Create new logging middleware - basic setup
    pub fn new() -> Self {
        Self {
            log_requests: true,
            log_responses: true,
            log_headers: false,
            log_body: false,
        }
    }

    /// fr fr Enable/disable request logging - control verbosity
    pub fn log_requests(mut self, enable: bool) -> Self {
        self.log_requests = enable;
        self
    }

    /// fr fr Enable/disable response logging - control verbosity
    pub fn log_responses(mut self, enable: bool) -> Self {
        self.log_responses = enable;
        self
    }

    /// fr fr Enable/disable header logging - detailed info
    pub fn log_headers(mut self, enable: bool) -> Self {
        self.log_headers = enable;
        self
    }

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
                }

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
                        };
                        println!("  Body: {}", preview);
                    }
                }
            }
            Ok(())
        })
    }

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
                };

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
                        };
                        println!("  Body: {}", preview);
                    }
                }
            }
            Ok(())
        })
    }

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
    content_security_policy: Option<String>,
    x_frame_options: Option<String>,
    x_content_type_options: bool,
    x_xss_protection: bool,
    strict_transport_security: Option<String>,
    referrer_policy: Option<String>,
}

impl SecurityHeadersMiddleware {
    /// fr fr Create new security headers middleware - basic security
    pub fn new() -> Self {
        Self {
            content_security_policy: Some("default-src 'self'".to_string()),
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: true,
            x_xss_protection: true,
            strict_transport_security: Some("max-age=31536000; includeSubDomains".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
        }
    }

    /// fr fr Set Content Security Policy - XSS protection
    pub fn content_security_policy(mut self, policy: Option<String>) -> Self {
        self.content_security_policy = policy;
        self
    }

    /// fr fr Set X-Frame-Options - clickjacking protection
    pub fn x_frame_options(mut self, value: Option<String>) -> Self {
        self.x_frame_options = value;
        self
    }

    /// fr fr Enable/disable X-Content-Type-Options - MIME sniffing protection
    pub fn x_content_type_options(mut self, enable: bool) -> Self {
        self.x_content_type_options = enable;
        self
    }

    /// fr fr Set Strict Transport Security - HTTPS enforcement
    pub fn strict_transport_security(mut self, value: Option<String>) -> Self {
        self.strict_transport_security = value;
        self
    }
}

impl Middleware for SecurityHeadersMiddleware {
    fn before_request<'a>(&'a self, _request: &'a mut HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        Box::pin(async { Ok(()) })
    }

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
            }

            if let Some(x_frame) = x_frame {
                response.headers.insert("x-frame-options".to_string(), x_frame);
            }

            if x_content_type {
                response.headers.insert("x-content-type-options".to_string(), "nosniff".to_string());
            }

            if x_xss {
                response.headers.insert("x-xss-protection".to_string(), "1; mode=block".to_string());
            }

            if let Some(hsts) = hsts {
                response.headers.insert("strict-transport-security".to_string(), hsts);
            }

            if let Some(referrer) = referrer {
                response.headers.insert("referrer-policy".to_string(), referrer);
            }

            Ok(())
        })
    }

    fn name(&self) -> &'static str {
        "SecurityHeadersMiddleware"
    }
}

impl Default for SecurityHeadersMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Rate limiting middleware - request throttling
#[derive(Clone)]
pub struct RateLimitMiddleware {
    max_requests: u32,
    window_duration: Duration,
    // In a real implementation, you'd have a store for tracking requests
    // For now, this is a placeholder structure
}

impl RateLimitMiddleware {
    /// fr fr Create new rate limit middleware - request throttling
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            max_requests,
            window_duration,
        }
    }

    /// fr fr Create rate limiter per minute - common setup
    pub fn per_minute(max_requests: u32) -> Self {
        Self::new(max_requests, Duration::from_secs(60))
    }

    /// fr fr Create rate limiter per hour - lenient setup
    pub fn per_hour(max_requests: u32) -> Self {
        Self::new(max_requests, Duration::from_secs(3600))
    }
}

impl Middleware for RateLimitMiddleware {
    fn before_request<'a>(&'a self, request: &'a mut HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        let max_requests = self.max_requests;
        let window_duration = self.window_duration;
        
        Box::pin(async move {
            // In a real implementation, you would:
            // 1. Get client IP or user ID
            // 2. Check request count in the current window
            // 3. Increment counter or return rate limit error
            
            let client_ip = request.client_ip()
                .map(|ip| ip.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            // Placeholder logic - in reality you'd use Redis or in-memory store
            println!("Rate limit check for {}: {}/{} per {:?}", 
                    client_ip, 1, max_requests, window_duration);

            // For demo purposes, always allow (real implementation would track and limit)
            Ok(())
        })
    }

    fn after_response<'a>(&'a self, _request: &'a HttpRequest, response: &'a mut HttpResponse) -> Pin<Box<dyn Future<Output = WebResult<()>> + Send + '_>> {
        let max_requests = self.max_requests;
        let window_duration = self.window_duration;
        
        Box::pin(async move {
            // Add rate limit headers
            response.headers.insert("x-ratelimit-limit".to_string(), max_requests.to_string());
            response.headers.insert("x-ratelimit-remaining".to_string(), (max_requests - 1).to_string());
            response.headers.insert("x-ratelimit-reset".to_string(), window_duration.as_secs().to_string());
            
            Ok(())
        })
    }

    fn name(&self) -> &'static str {
        "RateLimitMiddleware"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::packages::web_vibez::{method::HttpMethod, status::StatusCode};

    #[tokio::test]
    async fn test_middleware_chain() {
        let chain = MiddlewareChain::new()
            .add(LoggingMiddleware::new())
            .add(CorsMiddleware::new());

        assert_eq!(chain.len(), 2);

        let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
        let result = chain.process_request(&mut request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cors_middleware() {
        let cors = CorsMiddleware::new()
            .allowed_origins(Vec::from(["https://example.com".to_string()]));

        let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
        request.headers.insert("origin".to_string(), "https://example.com".to_string());
        
        let mut response = HttpResponse::ok();
        
        cors.after_response(&request, &mut response).await.unwrap();
        
        assert_eq!(
            response.header("access-control-allow-origin"),
            Some(&"https://example.com".to_string())
        );
    }

    #[tokio::test]
    async fn test_security_headers_middleware() {
        let security = SecurityHeadersMiddleware::new();
        let request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
        let mut response = HttpResponse::ok();
        
        security.after_response(&request, &mut response).await.unwrap();
        
        assert!(response.header("content-security-policy").is_some());
        assert!(response.header("x-frame-options").is_some());
        assert_eq!(response.header("x-content-type-options"), Some(&"nosniff".to_string()));
    }

    #[tokio::test]
    async fn test_rate_limit_middleware() {
        let rate_limit = RateLimitMiddleware::per_minute(100);
        let mut request = HttpRequest::new(HttpMethod::Get, "/test".to_string());
        let mut response = HttpResponse::ok();
        
        let result = rate_limit.before_request(&mut request).await;
        assert!(result.is_ok());
        
        rate_limit.after_response(&request, &mut response).await.unwrap();
        assert_eq!(response.header("x-ratelimit-limit"), Some(&"100".to_string()));
    }
}
