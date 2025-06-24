use crate::web::StatusCode;
/// Request handlers for HTTP routes
/// 
/// Provides trait definitions and common handler implementations
/// for processing HTTP requests and generating responses

use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext};
use crate::stdlib::web_vibez::{StatusCode};
use crate::stdlib::web_vibez::error_handling::HandlerError;

use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, instrument};

/// Result type for request handlers
pub type HandlerResult<T> = std::result::Result<T, HandlerError>;

use std::future::Future;
use std::pin::Pin;

/// Trait for handling HTTP requests
pub trait RequestHandler: Send + Sync {
    /// Handle an HTTP request
    fn handle<'a>(
        &'a self,
        context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>>;

    /// Get handler name for debugging
    fn name(&self) -> &'static str {
        "Unknown"
    }

    /// Get handler description
    fn description(&self) -> String {
        format!("Handler: {}", self.name())
    }
}

/// Debug implementation for RequestHandler
impl std::fmt::Debug for dyn RequestHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestHandler({})", self.name())
    }
}

/// Route handler wrapper that manages the full request/response lifecycle
#[derive(Debug)]
pub struct RouteHandler {
    /// The actual request handler
    handler: Arc<dyn RequestHandler>,
    /// Handler metadata
    metadata: HashMap<String, String>,
}

impl RouteHandler {
    pub fn new(handler: Arc<dyn RequestHandler>) -> Self {
        Self {
            handler,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    #[instrument(skip(self, context, response))]
    pub async fn execute(
        &self,
        context: &RequestContext,
        response: &mut ResponseContext,
    ) -> Result<(), Error> {
        debug!(
            handler = self.handler.name(),
            request_id = %context.request_id,
            method = %context.method,
            path = %context.path,
            "Executing handler"
        );

        let result = self.handler.handle(context, response).await;

        if let Err(ref e) = result {
            debug!(
                handler = self.handler.name(),
                request_id = %context.request_id,
                error = %e,
                "Handler execution failed"
            );
        } else {
            debug!(
                handler = self.handler.name(),
                request_id = %context.request_id,
                status = %response.status,
                response_size = response.body.len(),
                "Handler execution completed"
            );
        }

        result
    }

    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}

/// Static response handler - returns fixed content
#[derive(Debug)]
pub struct StaticHandler {
    content: String,
    content_type: String,
    status: StatusCode,
}

impl StaticHandler {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            content_type: "text/plain; charset=utf-8".to_string(),
            status: StatusCode::OK,
        }
    }

    pub fn with_content_type(mut self, content_type: &str) -> Self {
        self.content_type = content_type.to_string();
        self
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn json(content: &str) -> Self {
        Self::new(content).with_content_type("application/json")
    }

    pub fn html(content: &str) -> Self {
        Self::new(content).with_content_type("text/html; charset=utf-8")
    }

    pub fn not_found() -> Self {
        Self::new("Not Found")
            .with_status(StatusCode::NOT_FOUND)
            .with_content_type("text/plain")
    }
}

impl RequestHandler for StaticHandler {
    fn handle<'a>(
        &'a self,
        _context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
        let status = self.status;
        let content_type = self.content_type.clone();
        let content = self.content.clone();
        
        Box::pin(async move {
            response.set_status(status);
            response.set_header("Content-Type", &content_type);
            response.set_body_string(&content);
            Ok(())
        })
    }

    fn name(&self) -> &'static str {
        "Static"
    }

    fn description(&self) -> String {
        format!("Static handler: {} bytes, {}", self.content.len(), self.content_type)
    }
}

/// JSON API handler for REST endpoints
pub struct JsonApiHandler {
    /// Handler function for different HTTP methods
    handlers: HashMap<String, Box<dyn Fn(&RequestContext) -> Result<(), Error> + Send + Sync>>,
}

impl std::fmt::Debug for JsonApiHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonApiHandler")
            .field("handler_count", &self.handlers.len())
            .finish()
    }
}

impl JsonApiHandler {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn on_get<F>(mut self, handler: F) -> Self
    where
        F: Fn(&RequestContext) -> Result<(), Error> + Send + Sync + 'static,
    {
        self.handlers.insert("GET".to_string(), Box::new(handler));
        self
    }

    pub fn on_post<F>(mut self, handler: F) -> Self
    where
        F: Fn(&RequestContext) -> Result<(), Error> + Send + Sync + 'static,
    {
        self.handlers.insert("POST".to_string(), Box::new(handler));
        self
    }

    pub fn on_put<F>(mut self, handler: F) -> Self
    where
        F: Fn(&RequestContext) -> Result<(), Error> + Send + Sync + 'static,
    {
        self.handlers.insert("PUT".to_string(), Box::new(handler));
        self
    }

    pub fn on_delete<F>(mut self, handler: F) -> Self
    where
        F: Fn(&RequestContext) -> Result<(), Error> + Send + Sync + 'static,
    {
        self.handlers.insert("DELETE".to_string(), Box::new(handler));
        self
    }
}

impl RequestHandler for JsonApiHandler {
    fn handle<'a>(
        &'a self,
        context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
        let method_str = context.method.to_string();
        
        if let Some(handler) = self.handlers.get(&method_str) {
            match handler(context) {
                Ok(json_data) => {
                    response.set_json(&json_data)
                        .map_err(|e| HandlerError::Serialization(format!("JSON serialization error: {}", e)))?;
                    response.set_status(StatusCode::OK);
                }
                Err(e) => {
                    // Convert handler error to JSON error response
                    let error_response = serde_json::json!({
                        "error": e.to_string(),
                        "status": "error"
                    });
                    response.set_json(&error_response)
                        .map_err(|e| HandlerError::Serialization(format!("Error JSON serialization error: {}", e)))?;
                    response.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        } else {
            // Method not supported
            let error_response = serde_json::json!({
                "error": format!("Method {} not supported", method_str),
                "status": "error"
            });
            response.set_json(&error_response)
                .map_err(|e| HandlerError::Serialization(format!("Error JSON serialization error: {}", e)))?;
            response.set_status(StatusCode::METHOD_NOT_ALLOWED);
        }

        Ok(())
        })
    }

    fn name(&self) -> &'static str {
        "JsonApi"
    }

    fn description(&self) -> String {
        let methods: Vec<String> = self.handlers.keys().cloned().collect();
        format!("JSON API handler supporting methods: {}", methods.join(", "))
    }
}

/// Template handler for rendering dynamic content
pub struct TemplateHandler {
    /// Template content with placeholders
    template: String,
    /// Content type for response
    content_type: String,
    /// Data provider function
    data_provider: Option<Arc<dyn Fn(&RequestContext) -> HashMap<String, String> + Send + Sync>>,
}

impl std::fmt::Debug for TemplateHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateHandler")
            .field("template_length", &self.template.len())
            .field("content_type", &self.content_type)
            .field("has_data_provider", &self.data_provider.is_some())
            .finish()
    }
}

impl TemplateHandler {
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
            content_type: "text/html; charset=utf-8".to_string(),
            data_provider: None,
        }
    }

    pub fn with_content_type(mut self, content_type: &str) -> Self {
        self.content_type = content_type.to_string();
        self
    }

    pub fn with_data_provider<F>(mut self, provider: F) -> Self
    where
        F: Fn(&RequestContext) -> HashMap<String, String> + Send + Sync + 'static,
    {
        self.data_provider = Some(Arc::new(provider));
        self
    }

    /// Simple template substitution
    fn render_template(&self, template: &str, data: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        
        for (key, value) in data {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }
}

impl RequestHandler for TemplateHandler {
    fn handle<'a>(
        &'a self,
        context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
        let mut data = HashMap::new();
        
        // Add route parameters to template data
        for (key, value) in &context.route_params {
            data.insert(key.clone(), value.clone());
        }
        
        // Add query parameters to template data
        for (key, value) in &context.query_params {
            data.insert(format!("query_{}", key), value.clone());
        }
        
        // Add request metadata
        data.insert("method".to_string(), context.method.to_string());
        data.insert("path".to_string(), context.path.clone());
        data.insert("request_id".to_string(), context.request_id.clone());
        
        // Call data provider if available
        if let Some(provider) = &self.data_provider {
            let provided_data = provider(context);
            data.extend(provided_data);
        }
        
        let rendered = self.render_template(&self.template, &data);
        
        response.set_status(StatusCode::OK);
        response.set_header("Content-Type", &self.content_type);
        response.set_body_string(&rendered);
        
        Ok(())
        })
    }

    fn name(&self) -> &'static str {
        "Template"
    }

    fn description(&self) -> String {
        format!("Template handler: {} characters, {}", self.template.len(), self.content_type)
    }
}

/// File handler for serving files
#[derive(Debug)]
pub struct FileHandler {
    /// File path to serve
    file_path: PathBuf,
    /// Content type (auto-detected if None)
    content_type: Option<String>,
    /// Cache duration
    cache_duration: Option<std::time::Duration>,
}

impl FileHandler {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            content_type: None,
            cache_duration: Some(std::time::Duration::from_secs(3600)), // 1 hour default
        }
    }

    pub fn with_content_type(mut self, content_type: &str) -> Self {
        self.content_type = Some(content_type.to_string());
        self
    }

    pub fn with_cache_duration(mut self, duration: Option<std::time::Duration>) -> Self {
        self.cache_duration = duration;
        self
    }

    /// Detect content type from file extension
    fn detect_content_type(&self) -> String {
        if let Some(content_type) = &self.content_type {
            return content_type.clone();
        }

        if let Some(extension) = self.file_path.extension().and_then(|ext| ext.to_str()) {
            match extension.to_lowercase().as_str() {
                "html" | "htm" => "text/html; charset=utf-8".to_string(),
                "css" => "text/css".to_string(),
                "js" => "application/javascript".to_string(),
                "json" => "application/json".to_string(),
                "png" => "image/png".to_string(),
                "jpg" | "jpeg" => "image/jpeg".to_string(),
                "gif" => "image/gif".to_string(),
                "svg" => "image/svg+xml".to_string(),
                "pdf" => "application/pdf".to_string(),
                "txt" => "text/plain; charset=utf-8".to_string(),
                _ => "application/octet-stream".to_string(),
            }
        } else {
            "application/octet-stream".to_string()
        }
    }
}

impl RequestHandler for FileHandler {
    fn handle<'a>(
        &'a self,
        _context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
        let content = std::fs::read(&self.file_path)
            .map_err(|e| HandlerError::FileSystem(format!("Failed to read file {:?}: {}", self.file_path, e)))?;

        let content_type = self.detect_content_type();
        
        response.set_status(StatusCode::OK);
        response.set_header("Content-Type", &content_type);
        response.set_body(content);

        // Set cache headers if specified
        if let Some(cache_duration) = self.cache_duration {
            response.set_cache_control(cache_duration.as_secs() as u32, true);
        }

        Ok(())
        })
    }

    fn name(&self) -> &'static str {
        "File"
    }

    fn description(&self) -> String {
        format!("File handler: {:?}", self.file_path)
    }
}

/// Redirect handler for HTTP redirects
#[derive(Debug)]
pub struct RedirectHandler {
    /// Target URL for redirect
    target_url: String,
    /// Whether redirect is permanent (301) or temporary (302)
    permanent: bool,
}

impl RedirectHandler {
    pub fn temporary(target_url: &str) -> Self {
        Self {
            target_url: target_url.to_string(),
            permanent: false,
        }
    }

    pub fn permanent(target_url: &str) -> Self {
        Self {
            target_url: target_url.to_string(),
            permanent: true,
        }
    }

    /// Create redirect with parameter substitution
    pub fn with_params(mut self, substitute_params: bool) -> Self {
        // This would be enhanced to support parameter substitution
        self
    }
}

impl RequestHandler for RedirectHandler {
    fn handle<'a>(
        &'a self,
        context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
        let mut target_url = self.target_url.clone();

        // Substitute route parameters in target URL
        for (key, value) in &context.route_params {
            let placeholder = format!("{{{}}}", key);
            target_url = target_url.replace(&placeholder, value);
        }

        response.set_redirect(&target_url, self.permanent);
        
        Ok(())
        })
    }

    fn name(&self) -> &'static str {
        "Redirect"
    }

    fn description(&self) -> String {
        let redirect_type = if self.permanent { "permanent" } else { "temporary" };
        format!("Redirect handler: {} to {}", redirect_type, self.target_url)
    }
}

/// Proxy handler for forwarding requests
#[derive(Debug)]
pub struct ProxyHandler {
    /// Target base URL for proxying
    target_base_url: String,
    /// Headers to add to proxied requests
    additional_headers: HashMap<String, String>,
    /// Whether to preserve host header
    preserve_host: bool,
    /// Request timeout for proxied requests
    timeout: std::time::Duration,
    /// HTTP client for making requests
    client: reqwest::Client,
    /// Headers to remove from proxied responses
    filtered_response_headers: std::collections::HashSet<String>,
}

impl ProxyHandler {
    pub fn new(target_base_url: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none()) // Handle redirects manually
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        let mut filtered_headers = std::collections::HashSet::new();
        // Headers that should not be forwarded from the target response
        filtered_headers.insert("connection".to_lowercase());
        filtered_headers.insert("transfer-encoding".to_lowercase());
        filtered_headers.insert("upgrade".to_lowercase());
        filtered_headers.insert("proxy-authenticate".to_lowercase());
        filtered_headers.insert("proxy-authorization".to_lowercase());

        Self {
            target_base_url: target_base_url.to_string(),
            additional_headers: HashMap::new(),
            preserve_host: false,
            timeout: std::time::Duration::from_secs(30),
            client,
            filtered_response_headers: filtered_headers,
        }
    }

    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        self.additional_headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn preserve_host(mut self, preserve: bool) -> Self {
        self.preserve_host = preserve;
        self
    }

    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    /// Build the target URL from base URL and request path
    fn build_target_url(&self, path: &str, query: &str) -> Result<(), Error> {
        let base = self.target_base_url.trim_end_matches('/');
        let path = if path.starts_with('/') { path } else { &format!("/{}", path) };
        
        let url = if query.is_empty() {
            format!("{}{}", base, path)
        } else {
            format!("{}{}?{}", base, path, query)
        };

        // Validate the URL
        reqwest::Url::parse(&url)
            .map_err(|e| HandlerError::Configuration(format!("Invalid proxy target URL: {}", e)))?;

        Ok(url)
    }

    /// Build headers for the proxied request
    fn build_request_headers(&self, context: &RequestContext) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        // Copy headers from original request (except filtered ones)
        for (name, value) in &context.headers {
            let header_name = name.to_lowercase();
            
            // Skip connection-related headers
            if !self.should_filter_request_header(&header_name) {
                if let (Ok(name), Ok(value)) = (
                    reqwest::header::HeaderName::from_bytes(name.as_bytes()),
                    reqwest::header::HeaderValue::from_str(value)
                ) {
                    headers.insert(name, value);
                }
            }
        }

        // Handle host header preservation
        if !self.preserve_host {
            if let Ok(target_url) = reqwest::Url::parse(&self.target_base_url) {
                if let Some(host) = target_url.host_str() {
                    if let Ok(host_value) = reqwest::header::HeaderValue::from_str(host) {
                        headers.insert(reqwest::header::HOST, host_value);
                    }
                }
            }
        }

        // Add additional headers
        for (name, value) in &self.additional_headers {
            if let (Ok(name), Ok(value)) = (
                reqwest::header::HeaderName::from_str(name),
                reqwest::header::HeaderValue::from_str(value)
            ) {
                headers.insert(name, value);
            }
        }

        // Add X-Forwarded headers for proxy transparency
        if let Ok(forwarded_for) = reqwest::header::HeaderValue::from_str(&context.client_ip.as_ref().unwrap_or(&"unknown".to_string())) {
            headers.insert("x-forwarded-for", forwarded_for);
        }
        
        if let Ok(forwarded_proto) = reqwest::header::HeaderValue::from_str("http") {
            headers.insert("x-forwarded-proto", forwarded_proto);
        }

        headers
    }

    /// Check if a request header should be filtered out
    fn should_filter_request_header(&self, header_name: &str) -> bool {
        matches!(header_name.to_lowercase().as_str(),
            "connection" | "upgrade" | "proxy-connection" | "proxy-authenticate" | "proxy-authorization"
        )
    }

    /// Copy response headers from target to client response
    fn copy_response_headers(&self, target_response: &reqwest::Response, response: &mut ResponseContext) {
        for (name, value) in target_response.headers() {
            let header_name = name.as_str().to_lowercase();
            
            // Skip filtered headers
            if !self.filtered_response_headers.contains(&header_name) {
                if let Ok(value_str) = value.to_str() {
                    response.set_header(name.as_str(), value_str);
                }
            }
        }
    }
}

impl RequestHandler for ProxyHandler {
    fn handle<'a>(
        &'a self,
        context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
        // Build target URL with query parameters
        let query_string = context.query_params.iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        let target_url = self.build_target_url(&context.path, &query_string)?;
        
        debug!(
            source_path = %context.path,
            target_url = %target_url,
            method = %context.method,
            "Starting proxy request"
        );

        // Build the proxied request
        let mut request_builder = self.client
            .request(self.convert_method(&context.method)?, &target_url)
            .timeout(self.timeout)
            .headers(self.build_request_headers(context));

        // Add request body if present
        if !context.body.is_empty() {
            request_builder = request_builder.body(context.body.clone());
        }

        // Execute the proxied request
        let target_response = match request_builder.send().await {
            Ok(resp) => resp,
            Err(e) => {
                let error_msg = if e.is_timeout() {
                    format!("Proxy request timeout after {:?}", self.timeout)
                } else if e.is_connect() {
                    format!("Failed to connect to target server: {}", e)
                } else {
                    format!("Proxy request failed: {}", e)
                };
                
                debug!(
                    target_url = %target_url,
                    error = %e,
                    "Proxy request failed"
                );
                
                return Err(HandlerError::Network(error_msg));
            }
        };

        // Convert response status
        let status_code = target_response.status().as_u16();
        let status = StatusCode(status_code);

        // Copy response headers
        self.copy_response_headers(&target_response, response);

        // Get response body
        let response_body = match target_response.bytes().await {
            Ok(bytes) => bytes.to_vec(),
            Err(e) => {
                debug!(
                    target_url = %target_url,
                    error = %e,
                    "Failed to read response body"
                );
                return Err(HandlerError::Network(format!("Failed to read proxy response body: {}", e)));
            }
        };

        // Set final response
        response.set_status(status);
        response.set_body(response_body);

        debug!(
            source_path = %context.path,
            target_url = %target_url,
            status = status_code,
            response_size = response.body.len(),
            "Proxy request completed"
        );
        
        Ok(())
        })
    }

    fn name(&self) -> &'static str {
        "Proxy"
    }

    fn description(&self) -> String {
        format!("Proxy handler to: {}", self.target_base_url)
    }
}

impl ProxyHandler {
    /// Convert CURSED HttpMethod to reqwest Method
    fn convert_method(&self, method: &crate::stdlib::web_vibez::HttpMethod) -> Result<(), Error> {
        use crate::stdlib::web_vibez::HttpMethod;
        
        let method_str = match method {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::CONNECT => "CONNECT",
        };

        reqwest::Method::from_bytes(method_str.as_bytes())
            .map_err(|e| HandlerError::Configuration(format!("Invalid HTTP method: {}", e)))
    }
}

/// Composite handler that can delegate to different handlers based on conditions
pub struct CompositeHandler {
    /// Default handler
    default_handler: Arc<dyn RequestHandler>,
    /// Conditional handlers
    conditional_handlers: Vec<(Box<dyn Fn(&RequestContext) -> bool + Send + Sync>, Arc<dyn RequestHandler>)>,
}

impl std::fmt::Debug for CompositeHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompositeHandler")
            .field("default_handler", &self.default_handler.name())
            .field("condition_count", &self.conditional_handlers.len())
            .finish()
    }
}

impl CompositeHandler {
    pub fn new(default_handler: Arc<dyn RequestHandler>) -> Self {
        Self {
            default_handler,
            conditional_handlers: Vec::new(),
        }
    }

    pub fn add_condition<F>(mut self, condition: F, handler: Arc<dyn RequestHandler>) -> Self
    where
        F: Fn(&RequestContext) -> bool + Send + Sync + 'static,
    {
        self.conditional_handlers.push((Box::new(condition), handler));
        self
    }

    pub fn on_method(self, method: &str, handler: Arc<dyn RequestHandler>) -> Self {
        let method = method.to_string();
        self.add_condition(move |ctx| ctx.method.to_string() == method, handler)
    }

    pub fn on_header(self, header_name: &str, header_value: &str, handler: Arc<dyn RequestHandler>) -> Self {
        let header_name = header_name.to_lowercase();
        let header_value = header_value.to_string();
        self.add_condition(
            move |ctx| ctx.header(&header_name).map_or(false, |v| v == header_value),
            handler,
        )
    }
}

impl RequestHandler for CompositeHandler {
    fn handle<'a>(
        &'a self,
        context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
        // Check conditional handlers first
        for (condition, handler) in &self.conditional_handlers {
            if condition(context) {
                debug!(handler = handler.name(), "Using conditional handler");
                return handler.handle(context, response).await;
            }
        }

        // Fall back to default handler
        debug!(handler = self.default_handler.name(), "Using default handler");
        self.default_handler.handle(context, response).await
        })
    }

    fn name(&self) -> &'static str {
        "Composite"
    }

    fn description(&self) -> String {
        format!(
            "Composite handler with {} conditions, default: {}",
            self.conditional_handlers.len(),
            self.default_handler.name()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::web_vibez::HttpMethod;

    #[tokio::test]
    async fn test_static_handler() {
        let handler = StaticHandler::new("Hello, World!");
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        let mut response = ResponseContext::new();

        let result = handler.handle(&context, &mut response).await;
        assert!(result.is_ok());
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(String::from_utf8(response.body).unwrap(), "Hello, World!");
    }

    #[tokio::test]
    async fn test_json_api_handler() {
        let handler = JsonApiHandler::new()
            .on_get(|_ctx| Ok(serde_json::json!({"message": "Hello"})));

        let context = RequestContext::new("GET".to_string(), "/api/test".to_string());
        let mut response = ResponseContext::new();

        let result = handler.handle(&context, &mut response).await;
        assert!(result.is_ok());
        assert_eq!(response.status, StatusCode::OK);
        assert!(response.header("Content-Type").unwrap().contains("application/json"));
    }

    #[tokio::test]
    async fn test_template_handler() {
        let handler = TemplateHandler::new("Hello, {{name}}!")
            .with_data_provider(|_ctx| {
                let mut data = HashMap::new();
                data.insert("name".to_string(), "World".to_string());
                data
            });

        let context = RequestContext::new("GET".to_string(), "/template".to_string());
        let mut response = ResponseContext::new();

        let result = handler.handle(&context, &mut response).await;
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(response.body).unwrap(), "Hello, World!");
    }

    #[tokio::test]
    async fn test_redirect_handler() {
        let handler = RedirectHandler::temporary("https://example.com");
        let context = RequestContext::new("GET".to_string(), "/redirect".to_string());
        let mut response = ResponseContext::new();

        let result = handler.handle(&context, &mut response).await;
        assert!(result.is_ok());
        assert_eq!(response.status.0, 302);
        assert_eq!(response.header("Location"), Some("https://example.com"));
    }

    #[tokio::test]
    async fn test_composite_handler() {
        let default_handler = Arc::new(StaticHandler::new("Default"));
        let get_handler = Arc::new(StaticHandler::new("GET Response"));
        
        let handler = CompositeHandler::new(default_handler)
            .on_method("GET", get_handler);

        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        let mut response = ResponseContext::new();

        let result = handler.handle(&context, &mut response).await;
        assert!(result.is_ok());
        assert_eq!(String::from_utf8(response.body).unwrap(), "GET Response");
    }
}
