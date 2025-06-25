use crate::web::StatusCode;
use crate::error::CursedError;
/// fr fr Request handler definitions for web_vibez - processing incoming requests
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

// Placeholder imports disabled
// };

/// fr fr Handler trait for processing HTTP requests - the core interface
pub trait Handler: Send + Sync {
    /// fr fr Handle an HTTP request and return a response - main processing
    fn handle(&self, request: HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send + '_>>;
    
    /// fr fr Get handler name for debugging/logging - identification
    fn name(&self) -> &'static str {
        "Handler"
    }
}

/// fr fr Function handler type - closure-based handlers
pub type HandlerFunc = Arc<dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send>> + Send + Sync>;

/// fr fr Simple function wrapper that implements Handler trait
#[derive(Clone)]
pub struct FunctionHandler {
impl FunctionHandler {
    /// fr fr Create new function handler - wrap a closure
    pub fn new<F, Fut>(name: String, func: F) -> Self
    where
    {
        let handler_func = Arc::new(move |req: HttpRequest| -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send>> {
            Box::pin(func(req))
        });

        Self {
        }
    }

    /// fr fr Create simple sync handler - for basic responses
    pub fn sync<F>(name: String, func: F) -> Self
    where
    {
        Self::new(name, move |req| {
            let result = func(req);
            async move { result }
        })
    }
}

impl Handler for FunctionHandler {
    fn handle(&self, request: HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send + '_>> {
        (self.func)(request)
    fn name(&self) -> &'static str {
        // Note: This is a limitation - we can't return a &str from String
        // In practice, you'd want to use a different approach for dynamic names
        "FunctionHandler"
    }
}

/// fr fr Handler chain for middleware pattern - composable processing
#[derive(Clone)]
pub struct HandlerChain {
impl HandlerChain {
    /// fr fr Create new empty handler chain - start building
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Add handler to chain - extend processing
    pub fn add<H: Handler + 'static>(mut self, handler: H) -> Self {
        self.handlers.push(Arc::new(handler));
        self
    /// fr fr Add function handler to chain - quick closure addition
    pub fn add_fn<F, Fut>(self, name: String, func: F) -> Self
    where
    {
        self.add(FunctionHandler::new(name, func))
    /// fr fr Add sync function handler - simple closures
    pub fn add_sync<F>(self, name: String, func: F) -> Self
    where
    {
        self.add(FunctionHandler::sync(name, func))
    /// fr fr Get number of handlers in chain - size info
    pub fn len(&self) -> usize {
        self.handlers.len()
    /// fr fr Check if chain is empty - validation
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
}

impl Handler for HandlerChain {
    fn handle(&self, mut request: HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send + '_>> {
        Box::pin(async move {
            for (i, handler) in self.handlers.iter().enumerate() {
                // Each handler processes the request and potentially modifies it
                // The last handler should return the final response
                match handler.handle(request.clone()).await {
                    Ok(response) => {
                        // Check if this is a final response or if we should continue
                        // For now, we'll return the first successful response
                        return Ok(response);
                    }
                    Err(e) => {
                        // Continue to next handler on error, or return error if it's the last one
                        if i == self.handlers.len() - 1 {
                            return Err(e);
                        }
                        // For middleware, we might want to log the error and continue
                        continue;
                    }
                }
            // If no handlers or all failed, return a default error
            Err(WebError::internal_server_error("No handlers available"))
        })
    fn name(&self) -> &'static str {
        "HandlerChain"
    }
}

/// fr fr Static file handler - serve files from filesystem
pub struct StaticFileHandler {
impl StaticFileHandler {
    /// fr fr Create new static file handler - serve from directory
    pub fn new(root_dir: String) -> Self {
        Self {
        }
    }

    /// fr fr Set index files - default files for directories
    pub fn with_index_files(mut self, index_files: Vec<String>) -> Self {
        self.index_files = index_files;
        self
    /// fr fr Get MIME type from file extension - content type detection
    fn get_mime_type(&self, path: &str) -> &'static str {
        match path.split('.').last().unwrap_or("").to_lowercase().as_str() {
            "html" | "htm" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "json" => "application/json",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "ico" => "image/x-icon",
            "txt" => "text/plain",
            "pdf" => "application/pdf",
            _ => "application/octet-stream",
        }
    }
impl Handler for StaticFileHandler {
    fn handle(&self, request: HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send + '_>> {
        let root_dir = self.root_dir.clone();
        let index_files = self.index_files.clone();
        
        Box::pin(async move {
            // Security: Prevent directory traversal attacks
            if request.path.contains("..") || request.path.contains("\\") {
                return Ok(HttpResponse::forbidden().with_text("Access denied"));
            // Build file path
            let mut file_path = format!("{}{}", root_dir, request.path);
            
            // Check if path is a directory and try index files
            if std::path::Path::new(&file_path).is_dir() {
                let mut found_index = false;
                for index_file in index_files {
                    let index_path = format!("{}/{}", file_path, index_file);
                    if std::path::Path::new(&index_path).exists() {
                        file_path = index_path;
                        found_index = true;
                        break;
                    }
                }
                
                if !found_index {
                    return Ok(HttpResponse::not_found().with_text("Directory listing not allowed"));
                }
            }

            // Try to read the file
            match std::fs::read(&file_path) {
                Ok(contents) => {
                    let mime_type = StaticFileHandler::get_mime_type(&StaticFileHandler::new(root_dir), &file_path);
                    Ok(HttpResponse::ok()
                        .with_header("content-type", mime_type)
                        .with_body(contents))
                }
                Err(_) => {
                    Ok(HttpResponse::not_found().with_text("File not found"))
                }
            }
        })
    fn name(&self) -> &'static str {
        "StaticFileHandler"
    }
}

/// fr fr JSON API handler - structured response helpers
pub struct JsonApiHandler<T> {
impl<T> JsonApiHandler<T>
where
{
    /// fr fr Create JSON API handler - auto-serialize responses
    pub fn new<F, Fut>(handler: F) -> Self
    where
    {
        let handler_fn = Arc::new(move |req: HttpRequest| -> Pin<Box<dyn Future<Output = WebResult<T>> + Send>> {
            Box::pin(handler(req))
        });

        Self {
        }
    }
impl<T> Handler for JsonApiHandler<T>
where
{
    fn handle(&self, request: HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send + '_>> {
        let handler = self.handler.clone();
        
        Box::pin(async move {
            match handler(request).await {
                Ok(data) => {
                    HttpResponse::ok().with_json(&data)
                }
            }
        })
    fn name(&self) -> &'static str {
        "JsonApiHandler"
    }
}

/// fr fr Redirect handler - send users elsewhere
pub struct RedirectHandler {
impl RedirectHandler {
    /// fr fr Create permanent redirect handler - 301 response
    pub fn permanent(location: String) -> Self {
        Self {
        }
    }

    /// fr fr Create temporary redirect handler - 302 response
    pub fn temporary(location: String) -> Self {
        Self {
        }
    }
impl Handler for RedirectHandler {
    fn handle(&self, _request: HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send + '_>> {
        let location = self.location.clone();
        let permanent = self.permanent;
        
        Box::pin(async move {
            if permanent {
                Ok(HttpResponse::permanent_redirect(location))
            } else {
                Ok(HttpResponse::temporary_redirect(location))
            }
        })
    fn name(&self) -> &'static str {
        if self.permanent {
            "PermanentRedirectHandler"
        } else {
            "TemporaryRedirectHandler"
        }
    }
/// fr fr Health check handler - service monitoring
pub struct HealthCheckHandler {
impl HealthCheckHandler {
    /// fr fr Create new health check handler - monitoring endpoint
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Add health check function - custom validation
    pub fn add_check<F, Fut>(mut self, check: F) -> Self
    where
    {
        let check_fn = Arc::new(move || -> Pin<Box<dyn Future<Output = bool> + Send>> {
            Box::pin(check())
        });
        self.checks.push(check_fn);
        self
    }
}

impl Handler for HealthCheckHandler {
    fn handle(&self, _request: HttpRequest) -> Pin<Box<dyn Future<Output = WebResult<HttpResponse>> + Send + '_>> {
        let checks = self.checks.clone();
        
        Box::pin(async move {
            let mut all_healthy = true;
            
            for check in checks {
                if !check().await {
                    all_healthy = false;
                    break;
                }
            }

            let status = if all_healthy { "healthy" } else { "unhealthy" };
            let response_data = serde_json::json!({
            });

            if all_healthy {
                HttpResponse::ok().with_json(&response_data)
            } else {
//                 HttpResponse::new(crate::stdlib::packages::web_vibez::status::StatusCode::ServiceUnavailable)
                    .with_json(&response_data)
            }
        })
    fn name(&self) -> &'static str {
        "HealthCheckHandler"
    }
}

impl Default for HandlerChain {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for HealthCheckHandler {
    fn default() -> Self {
        Self::new()
    }
}

