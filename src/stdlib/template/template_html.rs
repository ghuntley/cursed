use crate::error::CursedError;
/// HTML Template Features - HTML-specific templating functionality
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, instrument, warn, info};

use crate::object::Object as CursedObject;

/// Template-specific error types
#[derive(Debug, Clone)]
pub enum TemplateError {
    /// CursedError during template rendering
    /// Invalid template parameter
    /// Template not found
    /// General template error
// impl std::fmt::Display for TemplateError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TemplateError::RenderError(msg) => write!(f, "Template render error: {}", msg),
//             TemplateError::ParameterError(msg) => write!(f, "Template parameter error: {}", msg),
//             TemplateError::NotFoundError(msg) => write!(f, "Template not found: {}", msg),
//             TemplateError::GeneralError(msg) => write!(f, "Template error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for TemplateError {}
// 
// impl From<CursedError> for TemplateError {
//     fn from(err: CursedError) -> Self {
//         TemplateError::GeneralError(err.to_string())
//     }
// }

/// HTML template context with auto-escaping and safe content handling
#[derive(Debug, Clone)]
pub struct HtmlTemplateContext {
    /// Whether to auto-escape HTML by default
    /// Content Security Policy settings
    /// Safe content markers
    /// Layout configuration
    /// Request context for web integration
    /// Component cache
/// Content Security Policy settings
#[derive(Debug, Clone)]
pub struct CspSettings {
    /// Whether to generate CSP nonces
    /// CSP nonce for scripts
    /// CSP nonce for styles
    /// Allowed inline script patterns
    /// Allowed inline style patterns
/// Types of safe content that bypass escaping
#[derive(Debug, Clone)]
pub enum SafeContentType {
/// Layout configuration for template composition
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    /// Default layout template
    /// Content blocks for yielding
    /// Asset configuration
    /// Meta tags configuration
/// Asset configuration for CSS/JS inclusion
#[derive(Debug, Clone)]
pub struct AssetConfig {
    /// Base URL for assets
    /// CSS file paths
    /// JavaScript file paths
    /// Asset versioning
/// Meta tags configuration
#[derive(Debug, Clone)]
pub struct MetaConfig {
    /// Page title
    /// Meta description
    /// Meta keywords
    /// Custom meta tags
/// Request context for web framework integration
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// Current request path
    /// Request method
    /// Session data
    /// Flash messages
    /// CSRF token
    /// Current user context
/// HTML Component template
#[derive(Debug, Clone)]
pub struct ComponentTemplate {
    /// Component name
    /// Component template content
    /// Component parameters
    /// Whether component is cached
/// Component parameter definition
#[derive(Debug, Clone)]
pub struct ComponentParameter {
    /// Parameter name
    /// Parameter type
    /// Whether parameter is required
    /// Default value
/// Component parameter types
#[derive(Debug, Clone)]
pub enum ComponentParameterType {
impl Default for CspSettings {
    fn default() -> Self {
        Self {
        }
    }
impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for AssetConfig {
    fn default() -> Self {
        Self {
            base_url: "/assets".to_string(),
        }
    }
impl Default for MetaConfig {
    fn default() -> Self {
        Self {
        }
    }
impl HtmlTemplateContext {
    /// Create a new HTML template context
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create context with auto-escaping disabled
    pub fn with_auto_escape(auto_escape: bool) -> Self {
        Self {
        }
    }

    /// Create context with CSP settings
    pub fn with_csp(csp_settings: CspSettings) -> Self {
        Self {
        }
    }

    /// Create context with layout configuration
    pub fn with_layout(layout_config: LayoutConfig) -> Self {
        Self {
        }
    }

    /// Create context with request context for web integration
    pub fn with_request_context(request_context: RequestContext) -> Self {
        Self {
        }
    }

    /// Mark content as safe HTML (bypasses escaping)
    pub fn mark_safe_html(&mut self, key: String) {
        self.safe_content.insert(key, SafeContentType::Html);
    /// Mark content as safe URL
    pub fn mark_safe_url(&mut self, key: String) {
        self.safe_content.insert(key, SafeContentType::Url);
    /// Mark content as safe JavaScript
    pub fn mark_safe_js(&mut self, key: String) {
        self.safe_content.insert(key, SafeContentType::JavaScript);
    /// Mark content as safe CSS
    pub fn mark_safe_css(&mut self, key: String) {
        self.safe_content.insert(key, SafeContentType::Css);
    /// Check if content is marked as safe
    pub fn is_safe_content(&self, key: &str, content_type: &SafeContentType) -> bool {
        self.safe_content.get(key)
            .map(|t| std::mem::discriminant(t) == std::mem::discriminant(content_type))
            .unwrap_or(false)
    /// Get CSP nonce for scripts
    pub fn script_nonce(&self) -> Option<&str> {
        self.csp_settings.script_nonce.as_deref()
    /// Get CSP nonce for styles
    pub fn style_nonce(&self) -> Option<&str> {
        self.csp_settings.style_nonce.as_deref()
    /// Set content for a layout block
    pub fn set_content_block(&mut self, name: String, content: String) {
        self.layout_config.content_blocks.insert(name, content);
    /// Get content for a layout block
    pub fn get_content_block(&self, name: &str) -> Option<&str> {
        self.layout_config.content_blocks.get(name).map(|s| s.as_str())
    /// Add stylesheet to asset configuration
    pub fn add_stylesheet(&mut self, path: String) {
        self.layout_config.asset_config.stylesheets.push(path);
    /// Add JavaScript to asset configuration
    pub fn add_script(&mut self, path: String) {
        self.layout_config.asset_config.scripts.push(path);
    /// Set page title
    pub fn set_title(&mut self, title: String) {
        self.layout_config.meta_config.title = Some(title);
    /// Get page title
    pub fn title(&self) -> Option<&str> {
        self.layout_config.meta_config.title.as_deref()
    /// Set meta description
    pub fn set_description(&mut self, description: String) {
        self.layout_config.meta_config.description = Some(description);
    /// Add meta keyword
    pub fn add_keyword(&mut self, keyword: String) {
        self.layout_config.meta_config.keywords.push(keyword);
    /// Set custom meta tag
    pub fn set_meta(&mut self, name: String, content: String) {
        self.layout_config.meta_config.custom_meta.insert(name, content);
    /// Get request context
    pub fn request_context(&self) -> Option<&RequestContext> {
        self.request_context.as_ref()
    /// Get session data
    pub fn session_get(&self, key: &str) -> Option<&CursedObject> {
        self.request_context.as_ref()
            .and_then(|ctx| ctx.session.get(key))
    /// Get flash message
    pub fn flash_get(&self, key: &str) -> Option<&str> {
        self.request_context.as_ref()
            .and_then(|ctx| ctx.flash.get(key).map(|s| s.as_str()))
    /// Get CSRF token
    pub fn csrf_token(&self) -> Option<&str> {
        self.request_context.as_ref()
            .and_then(|ctx| ctx.csrf_token.as_deref())
    /// Register a component template
    pub fn register_component(&self, component: ComponentTemplate) -> crate::error::Result<()> {
        let mut cache = self.component_cache.lock()
            .map_err(|e| CursedError::Runtime(format!("Component cache lock error: {}", e)))?;
        cache.insert(component.name.clone(), component);
        Ok(())
    /// Get a component template
    pub fn get_component(&self, name: &str) -> crate::error::Result<()> {
        let cache = self.component_cache.lock()
            .map_err(|e| CursedError::Runtime(format!("Component cache lock error: {}", e)))?;
        Ok(cache.get(name).cloned())
    }
}

impl Default for HtmlTemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

/// HTML content escaper with context-aware escaping
pub struct HtmlEscaper {
impl HtmlEscaper {
    /// Create a new HTML escaper
    pub fn new(context: HtmlTemplateContext) -> Self {
        Self { context }
    }

    /// Escape content based on context
    #[instrument(skip(self, content))]
    pub fn escape(&self, content: &str, escape_context: EscapeContext) -> crate::error::Result<()> {
        debug!(context = ?escape_context, content_length = content.len(), "Escaping HTML content");

        if !self.context.auto_escape {
            return Ok(content.to_string());
        match escape_context {
        }
    }

    /// Escape HTML content
    fn escape_html(&self, content: &str) -> String {
        content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    /// Escape HTML attribute content
    fn escape_html_attribute(&self, content: &str) -> String {
        content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('\n', "&#10;")
            .replace('\r', "&#13;")
            .replace('\t', "&#9;")
    /// Escape JavaScript content
    fn escape_javascript(&self, content: &str) -> String {
        content
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "\\'")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
            .replace('\u{2028}', "\\u2028") // Line separator
            .replace('\u{2029}', "\\u2029") // Paragraph separator
            .replace('<', "\\u003c")         // Prevent </script> injection
            .replace('>', "\\u003e")
    /// Escape CSS content
    fn escape_css(&self, content: &str) -> String {
        content.chars()
            .map(|c| match c {
            })
            .collect()
    /// Escape URL content
    fn escape_url(&self, content: &str) -> String {
        urlencoding::encode(content).to_string()
    /// Generate HTML with CSP nonces
    #[instrument(skip(self, tag_name, attributes, content))]
    pub fn generate_html_with_csp(
    ) -> crate::error::Result<()> {
        let mut html = format!("<{}", tag_name);

        // Add attributes
        for (name, value) in attributes {
            let escaped_value = self.escape_html_attribute(value);
            html.push_str(&format!(" {}=\"{}\"", name, escaped_value));
        // Add CSP nonces if applicable
        if self.context.csp_settings.generate_nonces {
            match tag_name.to_lowercase().as_str() {
                "script" => {
                    if let Some(nonce) = &self.context.csp_settings.script_nonce {
                        html.push_str(&format!(" nonce=\"{}\"", nonce));
                    }
                }
                "style" => {
                    if let Some(nonce) = &self.context.csp_settings.style_nonce {
                        html.push_str(&format!(" nonce=\"{}\"", nonce));
                    }
                }
                _ => {}
            }
        html.push('>');

        // Add content if provided
        if let Some(content) = content {
            let escaped_content = match tag_name.to_lowercase().as_str() {
            html.push_str(&escaped_content);
            html.push_str(&format!("</{}>", tag_name));
        Ok(html)
    /// Validate and sanitize HTML content
    #[instrument(skip(self, html))]
    pub fn sanitize_html(&self, html: &str) -> crate::error::Result<()> {
        debug!(html_length = html.len(), "Sanitizing HTML content");

        // This is a simplified HTML sanitizer
        // In a production environment, you'd want to use a proper HTML sanitizer library
        let mut sanitized = html.to_string();

        // Remove dangerous script tags
        sanitized = regex::Regex::new(r"(?i)<script[^>]*>.*?</script>")
            .unwrap()
            .replace_all(&sanitized, "")
            .to_string();

        // Remove dangerous event handlers
        sanitized = regex::Regex::new(r#"(?i)\s*on\w+\s*=\s*['"][^'"]*['"]"#)
            .unwrap()
            .replace_all(&sanitized, "")
            .to_string();

        // Remove javascript: URLs
        sanitized = regex::Regex::new(r#"(?i)javascript:[^'"]*"#)
            .unwrap()
            .replace_all(&sanitized, "")
            .to_string();

        // Remove data: URLs (potentially dangerous)
        sanitized = regex::Regex::new(r#"(?i)data:[^'"]*"#)
            .unwrap()
            .replace_all(&sanitized, "")
            .to_string();

        Ok(sanitized)
    }
}

/// Context for escaping content
#[derive(Debug, Clone, Copy)]
pub enum EscapeContext {
    /// HTML body content
    /// HTML attribute value
    /// JavaScript content
    /// CSS content
    /// URL content
    /// No escaping
/// Enhanced HTML template helpers with comprehensive web framework integration
pub struct HtmlTemplateHelpers;

/// Layout and composition helpers
pub struct LayoutHelpers;

/// Asset management helpers  
pub struct AssetHelpers;

/// Form generation helpers
pub struct FormHelpers;

/// Component system
pub struct ComponentSystem;

impl HtmlTemplateHelpers {
    /// Generate an HTML tag with attributes
    pub fn tag(
    ) -> crate::error::Result<()> {
        let mut attr_map = HashMap::new();
        for (key, value) in attributes {
            let value_str = match value {
            attr_map.insert(key.clone(), value_str);
        let html = escaper.generate_html_with_csp(tag_name, &attr_map, content)?;
        Ok(CursedObject::String(html))
    /// Generate a link tag
    pub fn link(
    ) -> crate::error::Result<()> {
        let mut attr_map = HashMap::new();
        attr_map.insert("href".to_string(), href.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "href" {
                    let value_str = match value {
                    attr_map.insert(key.clone(), value_str);
                }
            }
        let html = escaper.generate_html_with_csp("a", &attr_map, Some(text))?;
        Ok(html)
    /// Generate an image tag
    pub fn img(
    ) -> crate::error::Result<()> {
        let mut attr_map = HashMap::new();
        attr_map.insert("src".to_string(), src.to_string());
        attr_map.insert("alt".to_string(), alt.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "src" && key != "alt" {
                    let value_str = match value {
                    attr_map.insert(key.clone(), value_str);
                }
            }
        let html = escaper.generate_html_with_csp("img", &attr_map, None)?;
        Ok(CursedObject::String(html))
    /// Generate a form tag
    pub fn form(
    ) -> crate::error::Result<()> {
        let mut attr_map = HashMap::new();
        attr_map.insert("action".to_string(), action.to_string());
        attr_map.insert("method".to_string(), method.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "action" && key != "method" {
                    let value_str = match value {
                    attr_map.insert(key.clone(), value_str);
                }
            }
        let html = escaper.generate_html_with_csp("form", &attr_map, Some(content))?;
        Ok(CursedObject::String(html))
    /// Generate an input tag
    pub fn input(
    ) -> crate::error::Result<()> {
        let mut attr_map = HashMap::new();
        attr_map.insert("type".to_string(), input_type.to_string());
        attr_map.insert("name".to_string(), name.to_string());

        if let Some(val) = value {
            attr_map.insert("value".to_string(), val.to_string());
        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "type" && key != "name" && key != "value" {
                    let value_str = match value {
                    attr_map.insert(key.clone(), value_str);
                }
            }
        let html = escaper.generate_html_with_csp("input", &attr_map, None)?;
        Ok(CursedObject::String(html))
    /// Generate CSRF protection token
    pub fn csrf_token(secret: &str) -> crate::error::Result<()> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Simple CSRF token generation (in production, use proper crypto)
        let token = format!("{:x}", md5::compute(format!("{}{}", secret, timestamp)));
        Ok(CursedObject::String(token))
    /// Generate CSP nonce
    pub fn csp_nonce() -> crate::error::Result<()> {
        use rand::RngCore;
        
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 16];
        rng.fill_bytes(&mut bytes);
        
        let nonce = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
        Ok(CursedObject::String(nonce))
    /// Generate a select dropdown
    pub fn select(
        options: &[(String, String)], // (value, display_text) pairs
    ) -> crate::error::Result<()> {
        let mut attr_map = HashMap::new();
        attr_map.insert("name".to_string(), name.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "name" {
                    let value_str = match value {
                    attr_map.insert(key.clone(), value_str);
                }
            }
        let mut content = String::new();
        for (value, text) in options {
            let selected_attr = if Some(value.as_str()) == selected {
                " selected"
            } else {
                ""
            let escaped_value = escaper.escape_html_attribute(value);
            let escaped_text = escaper.escape_html(text);
            content.push_str(&format!(
                "<option value=\"{}\"{}>{}</option>",
                escaped_value, selected_attr, escaped_text
            ));
        let html = escaper.generate_html_with_csp("select", &attr_map, Some(&content))?;
        Ok(html)
    /// Generate a textarea
    pub fn textarea(
    ) -> crate::error::Result<()> {
        let mut attr_map = HashMap::new();
        attr_map.insert("name".to_string(), name.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "name" {
                    let value_str = match value {
                    attr_map.insert(key.clone(), value_str);
                }
            }
        let html = escaper.generate_html_with_csp("textarea", &attr_map, Some(content))?;
        Ok(html)
    /// Generate radio button group
    pub fn radio_group(
        options: &[(String, String)], // (value, label) pairs
    ) -> crate::error::Result<()> {
        let mut html = String::new();
        
        for (value, label) in options {
            let checked_attr = if Some(value.as_str()) == selected {
                " checked"
            } else {
                ""
            
            let input_id = format!("{}_{}", name, value);
            let escaped_value = escaper.escape_html_attribute(value);
            let escaped_label = escaper.escape_html(label);
            let escaped_id = escaper.escape_html_attribute(&input_id);
            
            html.push_str(&format!(
                "<input type=\"radio\" name=\"{}\" value=\"{}\" id=\"{}\"{}><label for=\"{}\">{}</label>",
                name, escaped_value, escaped_id, checked_attr, escaped_id, escaped_label
            ));
        Ok(html)
    /// Generate checkbox
    pub fn checkbox(
    ) -> crate::error::Result<()> {
        let mut attr_map = HashMap::new();
        attr_map.insert("type".to_string(), "checkbox".to_string());
        attr_map.insert("name".to_string(), name.to_string());
        attr_map.insert("value".to_string(), value.to_string());
        
        if checked {
            attr_map.insert("checked".to_string(), "checked".to_string());
        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if !["type", "name", "value", "checked"].contains(&key.as_str()) {
                    let value_str = match value {
                    attr_map.insert(key.clone(), value_str);
                }
            }
        let input_html = escaper.generate_html_with_csp("input", &attr_map, None)?;
        
        if let Some(label_text) = label {
            let input_id = format!("{}_{}", name, value);
            let escaped_id = escaper.escape_html_attribute(&input_id);
            let escaped_label = escaper.escape_html(label_text);
            
            // Add id to input
            let input_with_id = input_html.replace(">", &format!(" id=\"{}\">", escaped_id));
            let full_html = format!("{}<label for=\"{}\">{}</label>", input_with_id, escaped_id, escaped_label);
            Ok(full_html)
        } else {
            Ok(input_html)
        }
    }
impl LayoutHelpers {
    /// Render a layout with content blocks
    #[instrument(skip(context, content_blocks))]
    pub fn render_layout(
    ) -> crate::error::Result<()> {
        info!(layout_template_length = layout_template.len(), blocks_count = content_blocks.len(), "Rendering layout");
        
        let mut rendered = layout_template.to_string();
        
        // Replace content blocks
        for (block_name, content) in content_blocks {
            let placeholder = format!("{{{{ yield '{}' }}}}", block_name);
            rendered = rendered.replace(&placeholder, content);
        // Replace main content block
        if let Some(main_content) = content_blocks.get("main") {
            rendered = rendered.replace("{{ yield }}", main_content);
        Ok(rendered)
    /// Generate meta tags for the page head
    pub fn render_meta_tags(context: &HtmlTemplateContext) -> crate::error::Result<()> {
        let meta_config = &context.layout_config.meta_config;
        let mut meta_html = String::new();
        
        // Title tag
        if let Some(title) = &meta_config.title {
            let escaped_title = HtmlEscaper::new(context.clone()).escape_html(title);
            meta_html.push_str(&format!("<title>{}</title>\n", escaped_title));
        // Meta description
        if let Some(description) = &meta_config.description {
            let escaped_desc = HtmlEscaper::new(context.clone()).escape_html_attribute(description);
            meta_html.push_str(&format!("<meta name=\"description\" content=\"{}\">\n", escaped_desc));
        // Meta keywords
        if !meta_config.keywords.is_empty() {
            let keywords = meta_config.keywords.join(", ");
            let escaped_keywords = HtmlEscaper::new(context.clone()).escape_html_attribute(&keywords);
            meta_html.push_str(&format!("<meta name=\"keywords\" content=\"{}\">\n", escaped_keywords));
        // Custom meta tags
        for (name, content) in &meta_config.custom_meta {
            let escaped_name = HtmlEscaper::new(context.clone()).escape_html_attribute(name);
            let escaped_content = HtmlEscaper::new(context.clone()).escape_html_attribute(content);
            meta_html.push_str(&format!("<meta name=\"{}\" content=\"{}\">\n", escaped_name, escaped_content));
        // CSRF meta tag
        if let Some(csrf_token) = context.csrf_token() {
            let escaped_token = HtmlEscaper::new(context.clone()).escape_html_attribute(csrf_token);
            meta_html.push_str(&format!("<meta name=\"csrf-token\" content=\"{}\">\n", escaped_token));
        Ok(meta_html)
    /// Render partial template
    pub fn render_partial(
    ) -> crate::error::Result<()> {
        use super::template_core::{FileSystemLoader, TemplateLoader, TemplateEngine};
        use super::template_render::RenderContext;
        use std::path::PathBuf;
        
        // Default partials directory
        let partials_dir = PathBuf::from("templates/partials");
        
        // Create filesystem loader for partials
        let loader = FileSystemLoader::new(partials_dir);
        
        // Try different partial file patterns
        let possible_names = vec![
            format!("_{}.html", partial_name),    // _header.html
            format!("{}.html", partial_name),      // header.html
            format!("_{}.txt", partial_name),      // _header.txt
            format!("{}.txt", partial_name),       // header.txt
        ];
        
        let mut template_content = None;
        let mut found_name = None;
        
        for name in &possible_names {
            if loader.exists(name) {
                match loader.load(name) {
                    Ok(content) => {
                        template_content = Some(content);
                        found_name = Some(name.clone());
                        debug!("Loaded partial template: {}", name);
                        break;
                    }
                    Err(e) => {
                        warn!("Failed to load partial '{}': {}", name, e);
                        continue;
                    }
                }
            }
        }
        
        let content = template_content.ok_or_else(|| {
            CursedError::TemplateError {
            }
        })?;
        
        // Parse and render the partial template
        use super::template_syntax::{TemplateLexer, TemplateParser};
        
        let mut lexer = TemplateLexer::new(&content);
        let tokens = lexer.tokenize().map_err(|e| {
            CursedError::TemplateError {
            }
        })?;
        
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().map_err(|e| {
            CursedError::TemplateError {
            }
        })?;
        
        // Create render context with locals merged in
        let mut render_context = RenderContext::new();
        
        // Add locals to render context
        for (key, value) in locals {
            render_context.set_variable(key.clone(), value.clone());
        // Create a basic template engine for rendering
        let engine = TemplateEngine::new(Box::new(loader));
        let mut renderer = super::template_render::TemplateRenderer::new(&engine);
        
        // Render the partial
        let rendered = renderer.render_ast(&ast, &render_context).map_err(|e| {
            CursedError::TemplateError {
            }
        })?;
        
        debug!("Successfully rendered partial '{}' ({} chars)", found_name.unwrap_or_default(), rendered.len());
        Ok(rendered)
    }
}

impl AssetHelpers {
    /// Generate stylesheet link tags
    pub fn stylesheet_links(context: &HtmlTemplateContext) -> crate::error::Result<()> {
        let asset_config = &context.layout_config.asset_config;
        let mut html = String::new();
        
        for stylesheet in &asset_config.stylesheets {
            let href = Self::asset_url(stylesheet, &asset_config.base_url, &asset_config.version_suffix);
            let escaped_href = HtmlEscaper::new(context.clone()).escape_html_attribute(&href);
            
            let mut link_html = format!("<link rel=\"stylesheet\" href=\"{}\"", escaped_href);
            
            // Add nonce if CSP is enabled
            if let Some(nonce) = context.style_nonce() {
                link_html.push_str(&format!(" nonce=\"{}\"", nonce));
            link_html.push_str(">\n");
            html.push_str(&link_html);
        Ok(html)
    /// Generate JavaScript script tags
    pub fn javascript_includes(context: &HtmlTemplateContext) -> crate::error::Result<()> {
        let asset_config = &context.layout_config.asset_config;
        let mut html = String::new();
        
        for script in &asset_config.scripts {
            let src = Self::asset_url(script, &asset_config.base_url, &asset_config.version_suffix);
            let escaped_src = HtmlEscaper::new(context.clone()).escape_html_attribute(&src);
            
            let mut script_html = format!("<script src=\"{}\"", escaped_src);
            
            // Add nonce if CSP is enabled
            if let Some(nonce) = context.script_nonce() {
                script_html.push_str(&format!(" nonce=\"{}\"", nonce));
            script_html.push_str("></script>\n");
            html.push_str(&script_html);
        Ok(html)
    /// Generate asset URL with versioning
    pub fn asset_url(path: &str, base_url: &str, version_suffix: &Option<String>) -> String {
        let mut url = format!("{}/{}", base_url.trim_end_matches('/'), path.trim_start_matches('/'));
        
        if let Some(version) = version_suffix {
            url.push_str(&format!("?v={}", version));
        url
    /// Generate image tag with responsive attributes
    pub fn responsive_image(
        sizes: &[(u32, String)], // (width, src) pairs
    ) -> crate::error::Result<()> {
        let escaper = HtmlEscaper::new(context.clone());
        let escaped_src = escaper.escape_html_attribute(src);
        let escaped_alt = escaper.escape_html_attribute(alt);
        
        let mut img_html = format!("<img src=\"{}\" alt=\"{}\"", escaped_src, escaped_alt);
        
        // Add srcset for responsive images
        if !sizes.is_empty() {
            let srcset: Vec<String> = sizes.iter()
                .map(|(width, src)| format!("{} {}w", escaper.escape_html_attribute(src), width))
                .collect();
            img_html.push_str(&format!(" srcset=\"{}\"", srcset.join(", ")));
        img_html.push('>');
        Ok(img_html)
    }
}

impl FormHelpers {
    /// Generate a complete form with CSRF protection
    pub fn form_with_csrf(
    ) -> crate::error::Result<()> {
        let mut form_content = String::new();
        
        // Add CSRF token as hidden field
        if let Some(csrf_token) = context.csrf_token() {
            let escaped_token = escaper.escape_html_attribute(csrf_token);
            form_content.push_str(&format!(
                escaped_token
            ));
        form_content.push_str(content);
        
        let result = HtmlTemplateHelpers::form(action, method, attributes, &form_content, escaper)?;
        if let CursedObject::String(html) = result {
            Ok(html)
        } else {
            Err(CursedError::Runtime("Expected String result from form helper".to_string()))
        }
    }

    /// Generate form field with label and validation
    pub fn form_field(
    ) -> crate::error::Result<()> {
        let mut field_html = String::new();
        
        // Add label if provided
        if let Some(label_text) = label {
            let escaped_label = escaper.escape_html(label_text);
            let escaped_name = escaper.escape_html_attribute(name);
            field_html.push_str(&format!("<label for=\"{}\">{}</label>\n", escaped_name, escaped_label));
        // Add the input field
        let input_result = HtmlTemplateHelpers::input(field_type, name, value, attributes, escaper)?;
        if let CursedObject::String(input_html) = input_result {
            // Add id attribute for label association
            let input_with_id = input_html.replace(">", &format!(" id=\"{}\">", escaper.escape_html_attribute(name)));
            field_html.push_str(&input_with_id);
        // Add validation errors
        if !errors.is_empty() {
            field_html.push_str("<div class=\"errors\">\n");
            for error in errors {
                let escaped_error = escaper.escape_html(error);
                field_html.push_str(&format!("<span class=\"error\">{}</span>\n", escaped_error));
            }
            field_html.push_str("</div>\n");
        Ok(field_html)
    }
}

impl ComponentSystem {
    /// Render a component with parameters
    #[instrument(skip(context, parameters))]
    pub fn render_component(
    ) -> crate::error::Result<()> {
        info!(component_name = component_name, param_count = parameters.len(), "Rendering component");
        
        // Get component from cache
        let component = context.get_component(component_name)?
            .ok_or_else(|| CursedError::Runtime(format!("Component '{}' not found", component_name)))?;
        
        // Validate parameters
        Self::validate_parameters(&component, parameters)?;
        
        // Render component template with parameters
        let mut rendered = component.template.clone();
        
        // Simple parameter substitution (in real implementation, use proper templating)
        for (param_name, param_value) in parameters {
            let value_str = match param_value {
            
            let placeholder = format!("{{{{ {} }}}}", param_name);
            rendered = rendered.replace(&placeholder, &value_str);
        Ok(rendered)
    /// Validate component parameters
    fn validate_parameters(
    ) -> crate::error::Result<()> {
        for param_def in &component.parameters {
            if param_def.required && !parameters.contains_key(&param_def.name) {
                return Err(CursedError::Runtime(format!(
                    param_def.name, component.name
                )));
            // Type validation (simplified)
            if let Some(value) = parameters.get(&param_def.name) {
                let valid = match (&param_def.param_type, value) {
                    (ComponentParameterType::Object, _) => true, // Accept any object
                    (ComponentParameterType::Array, _) => true,  // Accept any array-like
                
                if !valid {
                    return Err(CursedError::Runtime(format!(
                        param_def.name, component.name
                    )));
                }
            }
        Ok(())
    /// Create a new component template
    pub fn create_component(
    ) -> ComponentTemplate {
        ComponentTemplate {
        }
    }

    /// Create a component parameter definition
    pub fn create_parameter(
    ) -> ComponentParameter {
        ComponentParameter {
        }
    }
