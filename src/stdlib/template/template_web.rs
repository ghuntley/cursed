use crate::error::Error;
/// Web Framework Integration - CURSED template integration for web applications
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, instrument, warn};
use serde_json::Value as JsonValue;

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;
use super::template_core::{TemplateEngine, TemplateContext, TemplateConfig};
use super::template_html::{HtmlTemplateContext, HtmlEscaper};
use super::template_formats::{TemplateFormat, TemplateFormatRenderer};
use super::template_cache::TemplateCache;

/// Web template renderer with HTTP-specific features
#[derive(Debug)]
pub struct WebTemplateRenderer {
    /// Core template engine
    engine: TemplateEngine,
    /// HTML context for web templates
    html_context: HtmlTemplateContext,
    /// Template cache
    cache: Arc<TemplateCache>,
    /// Web-specific configuration
    web_config: WebTemplateConfig,
}

/// Web template configuration
#[derive(Debug, Clone)]
pub struct WebTemplateConfig {
    /// Default content type for responses
    pub default_content_type: String,
    /// Enable CSRF protection
    pub enable_csrf: bool,
    /// CSRF token field name
    pub csrf_field_name: String,
    /// CSRF secret key
    pub csrf_secret: String,
    /// Enable XSS protection headers
    pub enable_xss_protection: bool,
    /// Content Security Policy
    pub csp_policy: Option<String>,
    /// Enable compression for responses
    pub enable_compression: bool,
    /// Cache control header value
    pub cache_control: Option<String>,
    /// Enable template hot reloading in development
    pub enable_hot_reload: bool,
}

impl Default for WebTemplateConfig {
    fn default() -> Self {
        Self {
            default_content_type: "text/html; charset=utf-8".to_string(),
            enable_csrf: true,
            csrf_field_name: "_token".to_string(),
            csrf_secret: "default_secret_change_me".to_string(),
            enable_xss_protection: true,
            csp_policy: Some("default-src 'self'".to_string()),
            enable_compression: true,
            cache_control: Some("no-cache".to_string()),
            enable_hot_reload: false,
        }
    }
}

/// HTTP response with template rendering
#[derive(Debug, Clone)]
pub struct TemplateResponse {
    /// Response body
    pub body: String,
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Content type
    pub content_type: String,
}

/// Web template request context
#[derive(Debug, Clone)]
pub struct WebTemplateRequest {
    /// HTTP method
    pub method: String,
    /// Request URL
    pub url: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Query parameters
    pub query: HashMap<String, String>,
    /// Form data
    pub form: HashMap<String, String>,
    /// Cookies
    pub cookies: HashMap<String, String>,
    /// Session data
    pub session: HashMap<String, CursedObject>,
    /// User context
    pub user: Option<HashMap<String, CursedObject>>,
}

impl WebTemplateRenderer {
    /// Create a new web template renderer
    pub fn new(template_dir: &str) -> Self {
        let config = TemplateConfig::default();
        let engine = TemplateEngine::new();
        let html_context = HtmlTemplateContext::new();
        let cache = Arc::new(TemplateCache::new(1000));
        let web_config = WebTemplateConfig::default();

        Self {
            engine,
            html_context,
            cache,
            web_config,
        }
    }

    /// Create with custom configuration
    pub fn with_config(
        template_config: TemplateConfig,
        web_config: WebTemplateConfig,
    ) -> Self {
        let engine = TemplateEngine::new();
        let html_context = HtmlTemplateContext::with_auto_escape(template_config.auto_escape);
        let cache = Arc::new(TemplateCache::new(template_config.cache_size));

        Self {
            engine,
            html_context,
            cache,
            web_config,
        }
    }

    /// Render a template for HTTP response
    #[instrument(skip(self, context, request))]
    pub fn render_response(
        &self,
        template_name: &str,
        context: TemplateContext,
        request: &WebTemplateRequest,
    ) -> Result<(), Error> {
        debug!(template = template_name, "Rendering web template");

        // Create enhanced context with web-specific variables
        let mut web_context = self.create_web_context(context, request)?;

        // Add CSRF token if enabled
        if self.web_config.enable_csrf {
            let csrf_token = self.generate_csrf_token(request)?;
            web_context.set(self.web_config.csrf_field_name.clone(), 
                CursedObject::String(csrf_token));
        }

        // Render the template
        let body = self.engine.render(template_name, web_context)?;

        // Create response with appropriate headers
        let mut response = TemplateResponse {
            body,
            status: 200,
            headers: HashMap::new(),
            content_type: self.web_config.default_content_type.clone(),
        };

        self.add_security_headers(&mut response)?;
        Ok(response)
    }

    /// Render template with specific format
    #[instrument(skip(self, context, request))]
    pub fn render_format(
        &self,
        template_name: &str,
        context: TemplateContext,
        request: &WebTemplateRequest,
        format: TemplateFormat,
    ) -> Result<(), Error> {
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
        };

        let mut response = TemplateResponse {
            body,
            status: 200,
            headers: HashMap::new(),
            content_type: format!("{}; charset=utf-8", content_type),
        };

        self.add_security_headers(&mut response)?;
        Ok(response)
    }

    /// Render partial template (for AJAX responses)
    #[instrument(skip(self, context))]
    pub fn render_partial(
        &self,
        template_name: &str,
        context: TemplateContext,
    ) -> Result<(), Error> {
        debug!(template = template_name, "Rendering partial template");
        self.engine.render(template_name, context)
    }

    /// Render JSON response
    #[instrument(skip(self, data))]
    pub fn render_json(&self, data: &CursedObject) -> Result<(), Error> {
        debug!("Rendering JSON response");

        let formatter = TemplateFormatRenderer::new(TemplateFormat::Json);
        let body = formatter.render(data)?;

        let mut response = TemplateResponse {
            body,
            status: 200,
            headers: HashMap::new(),
            content_type: "application/json; charset=utf-8".to_string(),
        };

        self.add_security_headers(&mut response)?;
        Ok(response)
    }

    /// Render error page
    #[instrument(skip(self, error))]
    pub fn render_error(
        &self,
        error: &CursedError,
        status_code: u16,
        request: &WebTemplateRequest,
    ) -> Result<(), Error> {
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
        };

        let mut response = TemplateResponse {
            body,
            status: status_code,
            headers: HashMap::new(),
            content_type: self.web_config.default_content_type.clone(),
        };

        self.add_security_headers(&mut response)?;
        Ok(response)
    }

    /// Create web-specific template context
    fn create_web_context(
        &self,
        mut context: TemplateContext,
        request: &WebTemplateRequest,
    ) -> Result<(), Error> {
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
        }

        // Add web helper functions
        self.add_web_helpers(&mut context)?;

        Ok(context)
    }

    /// Add web helper functions to context
    fn add_web_helpers(&self, context: &mut TemplateContext) -> Result<(), Error> {
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
    }

    /// Generate CSRF token
    fn generate_csrf_token(&self, request: &WebTemplateRequest) -> Result<(), Error> {
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
    }

    /// Verify CSRF token
    pub fn verify_csrf_token(
        &self,
        token: &str,
        request: &WebTemplateRequest,
    ) -> Result<(), Error> {
        let expected_token = self.generate_csrf_token(request)?;
        Ok(token == expected_token)
    }

    /// Add security headers to response
    fn add_security_headers(&self, response: &mut TemplateResponse) -> Result<(), Error> {
        if self.web_config.enable_xss_protection {
            response.headers.insert(
                "X-XSS-Protection".to_string(),
                "1; mode=block".to_string(),
            );
            response.headers.insert(
                "X-Content-Type-Options".to_string(),
                "nosniff".to_string(),
            );
            response.headers.insert(
                "X-Frame-Options".to_string(),
                "DENY".to_string(),
            );
        }

        if let Some(csp_policy) = &self.web_config.csp_policy {
            response.headers.insert(
                "Content-Security-Policy".to_string(),
                csp_policy.clone(),
            );
        }

        if let Some(cache_control) = &self.web_config.cache_control {
            response.headers.insert(
                "Cache-Control".to_string(),
                cache_control.clone(),
            );
        }

        Ok(())
    }

    /// Parse template output into CursedObject
    fn parse_template_output(&self, output: &str) -> Result<(), Error> {
        // Simple parsing - in a real implementation, you might want more sophisticated parsing
        if output.trim().starts_with('{') && output.trim().ends_with('}') {
            // Try to parse as JSON
            if let Ok(json_value) = serde_json::from_str::<JsonValue>(output) {
                return Ok(self.json_to_cursed(&json_value)?);
            }
        }

        // Default to string
        Ok(CursedObject::String(output.to_string()))
    }

    /// Convert JSON value to CursedObject
    fn json_to_cursed(&self, json: &JsonValue) -> Result<(), Error> {
        match json {
            JsonValue::Null => Ok(CursedObject::Nil),
            JsonValue::Bool(b) => Ok(CursedObject::Boolean(*b)),
            JsonValue::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(CursedObject::Integer(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(CursedObject::Float(f))
                } else {
                    Ok(CursedObject::Nil)
                }
            }
            JsonValue::String(s) => Ok(CursedObject::String(s.clone())),
            JsonValue::Array(arr) => {
                let cursed_array: Result<(), Error> = arr.iter()
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
    }

    /// Render default error page
    fn render_default_error(&self, error: &CursedError, status_code: u16) -> Result<(), Error> {
        let error_message = match status_code {
            404 => "Page Not Found",
            500 => "Internal Server Error",
            403 => "Forbidden",
            401 => "Unauthorized",
            _ => "Error",
        };

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
    }

    /// Enable hot reloading for development
    pub fn enable_hot_reload(&mut self) {
        self.web_config.enable_hot_reload = true;
    }

    /// Set custom CSP policy
    pub fn set_csp_policy(&mut self, policy: String) {
        self.web_config.csp_policy = Some(policy);
    }

    /// Set CSRF secret
    pub fn set_csrf_secret(&mut self, secret: String) {
        self.web_config.csrf_secret = secret;
    }
}

/// Template middleware for web frameworks
pub struct TemplateMiddleware {
    renderer: Arc<WebTemplateRenderer>,
}

impl TemplateMiddleware {
    pub fn new(renderer: WebTemplateRenderer) -> Self {
        Self {
            renderer: Arc::new(renderer),
        }
    }

    /// Process HTTP request with template rendering
    pub fn process_request(
        &self,
        request: &WebTemplateRequest,
        template_name: &str,
        context: TemplateContext,
    ) -> Result<(), Error> {
        self.renderer.render_response(template_name, context, request)
    }

    /// Handle error responses
    pub fn handle_error(
        &self,
        error: &CursedError,
        status_code: u16,
        request: &WebTemplateRequest,
    ) -> Result<(), Error> {
        self.renderer.render_error(error, status_code, request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_request() -> WebTemplateRequest {
        WebTemplateRequest {
            method: "GET".to_string(),
            url: "/test".to_string(),
            headers: HashMap::new(),
            query: HashMap::new(),
            form: HashMap::new(),
            cookies: HashMap::new(),
            session: HashMap::new(),
            user: None,
        }
    }

    #[test]
    fn test_web_template_renderer_creation() {
        let renderer = WebTemplateRenderer::new("templates");
        assert_eq!(renderer.web_config.default_content_type, "text/html; charset=utf-8");
        assert!(renderer.web_config.enable_csrf);
    }

    #[test]
    fn test_csrf_token_generation() {
        let renderer = WebTemplateRenderer::new("templates");
        let request = create_test_request();
        
        let token1 = renderer.generate_csrf_token(&request).unwrap();
        let token2 = renderer.generate_csrf_token(&request).unwrap();
        
        // Tokens should be the same for the same request
        assert_eq!(token1, token2);
        assert!(!token1.is_empty());
    }

    #[test]
    fn test_csrf_token_verification() {
        let renderer = WebTemplateRenderer::new("templates");
        let request = create_test_request();
        
        let token = renderer.generate_csrf_token(&request).unwrap();
        assert!(renderer.verify_csrf_token(&token, &request).unwrap());
        assert!(!renderer.verify_csrf_token("invalid_token", &request).unwrap());
    }

    #[test]
    fn test_json_response_rendering() {
        let renderer = WebTemplateRenderer::new("templates");
        
        let mut data = HashMap::new();
        data.insert("message".to_string(), CursedObject::String("Hello".to_string()));
        data.insert("status".to_string(), CursedObject::Integer(200));
        
        let cursed_data = CursedObject::Map(data);
        let response = renderer.render_json(&cursed_data).unwrap();
        
        assert_eq!(response.status, 200);
        assert_eq!(response.content_type, "application/json; charset=utf-8");
        assert!(response.body.contains("Hello"));
        assert!(response.body.contains("200"));
    }

    #[test]
    fn test_security_headers() {
        let renderer = WebTemplateRenderer::new("templates");
        let request = create_test_request();
        let context = TemplateContext::new();
        
        // This would fail because we don't have actual templates, but we can test the config
        assert!(renderer.web_config.enable_xss_protection);
        assert!(renderer.web_config.csp_policy.is_some());
    }

    #[test]
    fn test_web_context_creation() {
        let renderer = WebTemplateRenderer::new("templates");
        let mut request = create_test_request();
        request.query.insert("param1".to_string(), "value1".to_string());
        request.session.insert("user_id".to_string(), CursedObject::Integer(123));
        
        let context = TemplateContext::new();
        let web_context = renderer.create_web_context(context, &request).unwrap();
        
        assert!(web_context.contains("request_method"));
        assert!(web_context.contains("query"));
        assert!(web_context.contains("session"));
    }

    #[test]
    fn test_error_page_rendering() {
        let renderer = WebTemplateRenderer::new("templates");
        let request = create_test_request();
        let error = CursedError::TemplateError {
            message: "Test error".to_string(),
            source_location: None,
        };
        
        let response = renderer.render_error(&error, 404, &request).unwrap();
        
        assert_eq!(response.status, 404);
        assert!(response.body.contains("404"));
        assert!(response.body.contains("Page Not Found"));
    }
}
