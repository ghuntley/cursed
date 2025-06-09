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
pub type HandlerResult = Result<ResponseContext, HandlerError>;

use std::future::Future;
use std::pin::Pin;

/// Trait for handling HTTP requests
pub trait RequestHandler: Send + Sync {
    /// Handle an HTTP request
    fn handle<'a>(
        &'a self,
        context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>>;

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
    ) -> Result<(), HandlerError> {
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
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>> {
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
    handlers: HashMap<String, Box<dyn Fn(&RequestContext) -> Result<serde_json::Value, HandlerError> + Send + Sync>>,
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
        F: Fn(&RequestContext) -> Result<serde_json::Value, HandlerError> + Send + Sync + 'static,
    {
        self.handlers.insert("GET".to_string(), Box::new(handler));
        self
    }

    pub fn on_post<F>(mut self, handler: F) -> Self
    where
        F: Fn(&RequestContext) -> Result<serde_json::Value, HandlerError> + Send + Sync + 'static,
    {
        self.handlers.insert("POST".to_string(), Box::new(handler));
        self
    }

    pub fn on_put<F>(mut self, handler: F) -> Self
    where
        F: Fn(&RequestContext) -> Result<serde_json::Value, HandlerError> + Send + Sync + 'static,
    {
        self.handlers.insert("PUT".to_string(), Box::new(handler));
        self
    }

    pub fn on_delete<F>(mut self, handler: F) -> Self
    where
        F: Fn(&RequestContext) -> Result<serde_json::Value, HandlerError> + Send + Sync + 'static,
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
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>> {
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
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>> {
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
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>> {
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
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>> {
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
}

impl ProxyHandler {
    pub fn new(target_base_url: &str) -> Self {
        Self {
            target_base_url: target_base_url.to_string(),
            additional_headers: HashMap::new(),
            preserve_host: false,
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
}

impl RequestHandler for ProxyHandler {
    fn handle<'a>(
        &'a self,
        context: &'a RequestContext,
        response: &'a mut ResponseContext,
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>> {
        Box::pin(async move {
        // This is a simplified proxy implementation
        // In a real implementation, you would use an HTTP client library
        
        let target_url = format!("{}{}", self.target_base_url.trim_end_matches('/'), &context.path);
        
        // For now, just return a placeholder response
        let proxy_response = format!(
            "PROXY: {} {} -> {}",
            context.method,
            context.path,
            target_url
        );
        
        response.set_status(StatusCode::OK);
        response.set_header("Content-Type", "text/plain");
        response.set_body_string(&proxy_response);
        
        debug!(
            source_path = %context.path,
            target_url = %target_url,
            "Proxied request"
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
    ) -> Pin<Box<dyn Future<Output = Result<(), HandlerError>> + Send + '_>> {
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
