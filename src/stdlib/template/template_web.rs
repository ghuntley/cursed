use crate::error::CursedError;
/// Web Framework Integration - CURSED template integration for web applications
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, instrument, warn};
use serde_json::Value as JsonValue;

use crate::object::Object as CursedObject;
use super::template_core::{TemplateEngine, TemplateContext, TemplateConfig};
use super::template_html::{HtmlTemplateContext, HtmlEscaper};
use super::template_formats::{TemplateFormat, TemplateFormatRenderer};
use super::template_cache::TemplateCache;

/// Web template renderer with HTTP-specific features
#[derive(Debug)]
pub struct WebTemplateRenderer {
    /// Core template engine
    /// HTML context for web templates
    /// Template cache
    /// Web-specific configuration
/// Web template configuration
#[derive(Debug, Clone)]
pub struct WebTemplateConfig {
    /// Default content type for responses
    /// Enable CSRF protection
    /// CSRF token field name
    /// CSRF secret key
    /// Enable XSS protection headers
    /// Content Security Policy
    /// Enable compression for responses
    /// Cache control header value
    /// Enable template hot reloading in development
impl Default for WebTemplateConfig {
    fn default() -> Self {
        Self {
            default_content_type: "text/html; charset=utf-8".to_string(),
        }
    }
/// HTTP response with template rendering
#[derive(Debug, Clone)]
pub struct TemplateResponse {
    /// Response body
    /// HTTP status code
    /// Response headers
    /// Content type
/// Web template request context
#[derive(Debug, Clone)]
pub struct WebTemplateRequest {
    /// HTTP method
    /// Request URL
    /// Request headers
    /// Query parameters
    /// Form data
    /// Cookies
    /// Session data
    /// User context
impl WebTemplateRenderer {
    /// Create a new web template renderer
    pub fn new(template_dir: &str) -> Self {
        let config = TemplateConfig::default();
        let engine = TemplateEngine::new();
        let html_context = HtmlTemplateContext::new();
        let cache = Arc::new(TemplateCache::new(1000));
        let web_config = WebTemplateConfig::default();

        Self {
        }
    }

    /// Create with custom configuration
    pub fn with_config(
    ) -> Self {
        let engine = TemplateEngine::new();
        let html_context = HtmlTemplateContext::with_auto_escape(template_config.auto_escape);
        let cache = Arc::new(TemplateCache::new(template_config.cache_size));

        Self {
        }
    }

    /// Render a template for HTTP response
    #[instrument(skip(self, context, request))]
    pub fn render_response(
    ) -> crate::error::Result<()> {
        debug!(template = template_name, "Rendering web template");

        // Create enhanced context with web-specific variables
        let mut web_context = self.create_web_context(context, request)?;

        // Add CSRF token if enabled
        if self.web_config.enable_csrf {
            let csrf_token = self.generate_csrf_token(request)?;
                CursedObject::String(csrf_token));
        // Render the template
        let body = self.engine.render(template_name, web_context)?;

        // Create response with appropriate headers
        let mut response = TemplateResponse {

        self.add_security_headers(&mut response)?;
        Ok(response)
    /// Render template with specific format
    #[instrument(skip(self, context, request))]
    pub fn render_format(
    ) -> crate::error::Result<()> {
        debug!(template = template_name, format = ?format, "Rendering template with format");

        let web_context = self.create_web_context(context, request)?;
        let template_output = self.engine.render(template_name, web_context)?;

        // Convert to CursedObject for format rendering
        let data = self.parse_template_output(&template_output)?;
        
        let formatter = TemplateFormatRenderer::new(format.clone());
        let body = formatter.render(&data)?;

        let content_type = match format {
            TemplateFormat::Json => "application/json",
            TemplateFormat::Xml => "application/xml",
            TemplateFormat::Yaml => "application/x-yaml",
            TemplateFormat::Csv => "text/csv",
            TemplateFormat::Text => "text/plain",
            _ => "text/html",

        let mut response = TemplateResponse {

        self.add_security_headers(&mut response)?;
        Ok(response)
    /// Render partial template (for AJAX responses)
    #[instrument(skip(self, context))]
    pub fn render_partial(
    ) -> crate::error::Result<()> {
        debug!(template = template_name, "Rendering partial template");
        self.engine.render(template_name, context)
    /// Render JSON response
    #[instrument(skip(self, data))]
    pub fn render_json(&self, data: &CursedObject) -> crate::error::Result<()> {
        debug!("Rendering JSON response");

        let formatter = TemplateFormatRenderer::new(TemplateFormat::Json);
        let body = formatter.render(data)?;

        let mut response = TemplateResponse {
            content_type: "application/json; charset=utf-8".to_string(),

        self.add_security_headers(&mut response)?;
        Ok(response)
    /// Render error page
    #[instrument(skip(self, error))]
    pub fn render_error(
    ) -> crate::error::Result<()> {
        debug!(status = status_code, "Rendering error page");

        let mut context = TemplateContext::new();
        context.set("error", CursedObject::String(error.to_string()));
        context.set("status", CursedObject::Integer(status_code as i64));

        let web_context = self.create_web_context(context, request)?;

        // Try to render custom error template, fall back to default
        let template_name = format!("errors/{}.html", status_code);
        let body = if self.engine.template_exists(&template_name) {
            self.engine.render(&template_name, web_context)?
        } else {
            self.render_default_error(error, status_code)?

        let mut response = TemplateResponse {

        self.add_security_headers(&mut response)?;
        Ok(response)
    /// Create web-specific template context
    fn create_web_context(
    ) -> crate::error::Result<()> {
        // Add request information
        context.set("request_method", CursedObject::String(request.method.clone()));
        context.set("request_url", CursedObject::String(request.url.clone()));

        // Add query parameters
        let query_map: HashMap<String, CursedObject> = request.query.iter()
            .map(|(k, v)| (k.clone(), CursedObject::String(v.clone())))
            .collect();
        context.set("query", CursedObject::Map(query_map));

        // Add form data
        let form_map: HashMap<String, CursedObject> = request.form.iter()
            .map(|(k, v)| (k.clone(), CursedObject::String(v.clone())))
            .collect();
        context.set("form", CursedObject::Map(form_map));

        // Add session data
        context.set("session", CursedObject::Map(request.session.clone()));

        // Add user context if available
        if let Some(user) = &request.user {
            context.set("user", CursedObject::Map(user.clone()));
        // Add web helper functions
        self.add_web_helpers(&mut context)?;

        Ok(context)
    /// Add web helper functions to context
    fn add_web_helpers(&self, context: &mut TemplateContext) -> crate::error::Result<()> {
        // Add current timestamp
        context.set("now", CursedObject::Integer(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
        ));

        // Add environment helpers
        context.set("env", CursedObject::String(
            std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
        ));

        Ok(())
    /// Generate CSRF token
    fn generate_csrf_token(&self, request: &WebTemplateRequest) -> crate::error::Result<()> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(self.web_config.csrf_secret.as_bytes());
        hasher.update(request.url.as_bytes());
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        hasher.update(timestamp.to_string().as_bytes());
        
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    /// Verify CSRF token
    pub fn verify_csrf_token(
    ) -> crate::error::Result<()> {
        let expected_token = self.generate_csrf_token(request)?;
        Ok(token == expected_token)
    /// Add security headers to response
    fn add_security_headers(&self, response: &mut TemplateResponse) -> crate::error::Result<()> {
        if self.web_config.enable_xss_protection {
            response.headers.insert(
            );
            response.headers.insert(
            );
            response.headers.insert(
            );
        if let Some(csp_policy) = &self.web_config.csp_policy {
            response.headers.insert(
            );
        if let Some(cache_control) = &self.web_config.cache_control {
            response.headers.insert(
            );
        Ok(())
    /// Parse template output into CursedObject
    fn parse_template_output(&self, output: &str) -> crate::error::Result<()> {
        // Simple parsing - in a real implementation, you might want more sophisticated parsing
        if output.trim().starts_with('{') && output.trim().ends_with('}') {
            // Try to parse as JSON
            if let Ok(json_value) = serde_json::from_str::<JsonValue>(output) {
                return Ok(self.json_to_cursed(&json_value)?);
            }
        }

        // Default to string
        Ok(CursedObject::String(output.to_string()))
    /// Convert JSON value to CursedObject
    fn json_to_cursed(&self, json: &JsonValue) -> crate::error::Result<()> {
        match json {
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(CursedObject::Integer(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(CursedObject::Float(f))
                } else {
                    Ok(CursedObject::Nil)
                }
            }
            JsonValue::Array(arr) => {
                let cursed_array: crate::error::Result<()> = arr.iter()
                    .map(|item| self.json_to_cursed(item))
                    .collect();
                Ok(CursedObject::Array(cursed_array?))
            }
            JsonValue::Object(obj) => {
                let mut cursed_map = HashMap::new();
                for (key, value) in obj {
                    cursed_map.insert(key.clone(), self.json_to_cursed(value)?);
                }
                Ok(CursedObject::Map(cursed_map))
            }
        }
    /// Render default error page
    fn render_default_error(&self, error: &CursedError, status_code: u16) -> crate::error::Result<()> {
        let error_message = match status_code {

        Ok(format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>{} - {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; text-align: center; margin-top: 50px; }}
        .error-container {{ max-width: 600px; margin: 0 auto; }}
        .error-code {{ font-size: 4em; color: #e74c3c; margin: 0; }}
        .error-message {{ font-size: 1.5em; color: #333; margin: 20px 0; }}
        .error-details {{ color: #666; font-size: 0.9em; }}
    </style>
</head>
<body>
    <div class="error-container">
        <h1 class="error-code">{}</h1>
        <p class="error-message">{}</p>
        <p class="error-details">{}</p>
    </div>
</body>
</html>"#,
            status_code, error_message, status_code, error_message, error
        ))
    /// Enable hot reloading for development
    pub fn enable_hot_reload(&mut self) {
        self.web_config.enable_hot_reload = true;
    /// Set custom CSP policy
    pub fn set_csp_policy(&mut self, policy: String) {
        self.web_config.csp_policy = Some(policy);
    /// Set CSRF secret
    pub fn set_csrf_secret(&mut self, secret: String) {
        self.web_config.csrf_secret = secret;
    }
}

/// Template middleware for web frameworks
pub struct TemplateMiddleware {
impl TemplateMiddleware {
    pub fn new(renderer: WebTemplateRenderer) -> Self {
        Self {
        }
    }

    /// Process HTTP request with template rendering
    pub fn process_request(
    ) -> crate::error::Result<()> {
        self.renderer.render_response(template_name, context, request)
    /// Handle error responses
    pub fn handle_error(
    ) -> crate::error::Result<()> {
        self.renderer.render_error(error, status_code, request)
    }
}

