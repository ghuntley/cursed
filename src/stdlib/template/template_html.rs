/// HTML Template Features - HTML-specific templating functionality
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, instrument, warn, info};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;

/// Template-specific error types
#[derive(Debug, Clone)]
pub enum TemplateError {
    /// Error during template rendering
    RenderError(String),
    /// Invalid template parameter
    ParameterError(String),
    /// Template not found
    NotFoundError(String),
    /// General template error
    GeneralError(String),
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::RenderError(msg) => write!(f, "Template render error: {}", msg),
            TemplateError::ParameterError(msg) => write!(f, "Template parameter error: {}", msg),
            TemplateError::NotFoundError(msg) => write!(f, "Template not found: {}", msg),
            TemplateError::GeneralError(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

impl std::error::Error for TemplateError {}

impl From<CursedError> for TemplateError {
    fn from(err: CursedError) -> Self {
        TemplateError::GeneralError(err.to_string())
    }
}

/// HTML template context with auto-escaping and safe content handling
#[derive(Debug, Clone)]
pub struct HtmlTemplateContext {
    /// Whether to auto-escape HTML by default
    auto_escape: bool,
    /// Content Security Policy settings
    csp_settings: CspSettings,
    /// Safe content markers
    safe_content: HashMap<String, SafeContentType>,
    /// Layout configuration
    layout_config: LayoutConfig,
    /// Request context for web integration
    request_context: Option<RequestContext>,
    /// Component cache
    component_cache: Arc<Mutex<HashMap<String, ComponentTemplate>>>,
}

/// Content Security Policy settings
#[derive(Debug, Clone)]
pub struct CspSettings {
    /// Whether to generate CSP nonces
    pub generate_nonces: bool,
    /// CSP nonce for scripts
    pub script_nonce: Option<String>,
    /// CSP nonce for styles
    pub style_nonce: Option<String>,
    /// Allowed inline script patterns
    pub allowed_inline_scripts: Vec<String>,
    /// Allowed inline style patterns
    pub allowed_inline_styles: Vec<String>,
}

/// Types of safe content that bypass escaping
#[derive(Debug, Clone)]
pub enum SafeContentType {
    Html,
    Url,
    JavaScript,
    Css,
    Attribute,
}

/// Layout configuration for template composition
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    /// Default layout template
    pub default_layout: Option<String>,
    /// Content blocks for yielding
    pub content_blocks: HashMap<String, String>,
    /// Asset configuration
    pub asset_config: AssetConfig,
    /// Meta tags configuration
    pub meta_config: MetaConfig,
}

/// Asset configuration for CSS/JS inclusion
#[derive(Debug, Clone)]
pub struct AssetConfig {
    /// Base URL for assets
    pub base_url: String,
    /// CSS file paths
    pub stylesheets: Vec<String>,
    /// JavaScript file paths
    pub scripts: Vec<String>,
    /// Asset versioning
    pub version_suffix: Option<String>,
}

/// Meta tags configuration
#[derive(Debug, Clone)]
pub struct MetaConfig {
    /// Page title
    pub title: Option<String>,
    /// Meta description
    pub description: Option<String>,
    /// Meta keywords
    pub keywords: Vec<String>,
    /// Custom meta tags
    pub custom_meta: HashMap<String, String>,
}

/// Request context for web framework integration
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// Current request path
    pub path: String,
    /// Request method
    pub method: String,
    /// Session data
    pub session: HashMap<String, CursedObject>,
    /// Flash messages
    pub flash: HashMap<String, String>,
    /// CSRF token
    pub csrf_token: Option<String>,
    /// Current user context
    pub user: Option<HashMap<String, CursedObject>>,
}

/// HTML Component template
#[derive(Debug, Clone)]
pub struct ComponentTemplate {
    /// Component name
    pub name: String,
    /// Component template content
    pub template: String,
    /// Component parameters
    pub parameters: Vec<ComponentParameter>,
    /// Whether component is cached
    pub cacheable: bool,
}

/// Component parameter definition
#[derive(Debug, Clone)]
pub struct ComponentParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: ComponentParameterType,
    /// Whether parameter is required
    pub required: bool,
    /// Default value
    pub default_value: Option<CursedObject>,
}

/// Component parameter types
#[derive(Debug, Clone)]
pub enum ComponentParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Object,
    Array,
}

impl Default for CspSettings {
    fn default() -> Self {
        Self {
            generate_nonces: false,
            script_nonce: None,
            style_nonce: None,
            allowed_inline_scripts: vec![],
            allowed_inline_styles: vec![],
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            default_layout: None,
            content_blocks: HashMap::new(),
            asset_config: AssetConfig::default(),
            meta_config: MetaConfig::default(),
        }
    }
}

impl Default for AssetConfig {
    fn default() -> Self {
        Self {
            base_url: "/assets".to_string(),
            stylesheets: vec![],
            scripts: vec![],
            version_suffix: None,
        }
    }
}

impl Default for MetaConfig {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            keywords: vec![],
            custom_meta: HashMap::new(),
        }
    }
}

impl HtmlTemplateContext {
    /// Create a new HTML template context
    pub fn new() -> Self {
        Self {
            auto_escape: true,
            csp_settings: CspSettings::default(),
            safe_content: HashMap::new(),
            layout_config: LayoutConfig::default(),
            request_context: None,
            component_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create context with auto-escaping disabled
    pub fn with_auto_escape(auto_escape: bool) -> Self {
        Self {
            auto_escape,
            csp_settings: CspSettings::default(),
            safe_content: HashMap::new(),
            layout_config: LayoutConfig::default(),
            request_context: None,
            component_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create context with CSP settings
    pub fn with_csp(csp_settings: CspSettings) -> Self {
        Self {
            auto_escape: true,
            csp_settings,
            safe_content: HashMap::new(),
            layout_config: LayoutConfig::default(),
            request_context: None,
            component_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create context with layout configuration
    pub fn with_layout(layout_config: LayoutConfig) -> Self {
        Self {
            auto_escape: true,
            csp_settings: CspSettings::default(),
            safe_content: HashMap::new(),
            layout_config,
            request_context: None,
            component_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create context with request context for web integration
    pub fn with_request_context(request_context: RequestContext) -> Self {
        Self {
            auto_escape: true,
            csp_settings: CspSettings::default(),
            safe_content: HashMap::new(),
            layout_config: LayoutConfig::default(),
            request_context: Some(request_context),
            component_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Mark content as safe HTML (bypasses escaping)
    pub fn mark_safe_html(&mut self, key: String) {
        self.safe_content.insert(key, SafeContentType::Html);
    }

    /// Mark content as safe URL
    pub fn mark_safe_url(&mut self, key: String) {
        self.safe_content.insert(key, SafeContentType::Url);
    }

    /// Mark content as safe JavaScript
    pub fn mark_safe_js(&mut self, key: String) {
        self.safe_content.insert(key, SafeContentType::JavaScript);
    }

    /// Mark content as safe CSS
    pub fn mark_safe_css(&mut self, key: String) {
        self.safe_content.insert(key, SafeContentType::Css);
    }

    /// Check if content is marked as safe
    pub fn is_safe_content(&self, key: &str, content_type: &SafeContentType) -> bool {
        self.safe_content.get(key)
            .map(|t| std::mem::discriminant(t) == std::mem::discriminant(content_type))
            .unwrap_or(false)
    }

    /// Get CSP nonce for scripts
    pub fn script_nonce(&self) -> Option<&str> {
        self.csp_settings.script_nonce.as_deref()
    }

    /// Get CSP nonce for styles
    pub fn style_nonce(&self) -> Option<&str> {
        self.csp_settings.style_nonce.as_deref()
    }

    /// Set content for a layout block
    pub fn set_content_block(&mut self, name: String, content: String) {
        self.layout_config.content_blocks.insert(name, content);
    }

    /// Get content for a layout block
    pub fn get_content_block(&self, name: &str) -> Option<&str> {
        self.layout_config.content_blocks.get(name).map(|s| s.as_str())
    }

    /// Add stylesheet to asset configuration
    pub fn add_stylesheet(&mut self, path: String) {
        self.layout_config.asset_config.stylesheets.push(path);
    }

    /// Add JavaScript to asset configuration
    pub fn add_script(&mut self, path: String) {
        self.layout_config.asset_config.scripts.push(path);
    }

    /// Set page title
    pub fn set_title(&mut self, title: String) {
        self.layout_config.meta_config.title = Some(title);
    }

    /// Get page title
    pub fn title(&self) -> Option<&str> {
        self.layout_config.meta_config.title.as_deref()
    }

    /// Set meta description
    pub fn set_description(&mut self, description: String) {
        self.layout_config.meta_config.description = Some(description);
    }

    /// Add meta keyword
    pub fn add_keyword(&mut self, keyword: String) {
        self.layout_config.meta_config.keywords.push(keyword);
    }

    /// Set custom meta tag
    pub fn set_meta(&mut self, name: String, content: String) {
        self.layout_config.meta_config.custom_meta.insert(name, content);
    }

    /// Get request context
    pub fn request_context(&self) -> Option<&RequestContext> {
        self.request_context.as_ref()
    }

    /// Get session data
    pub fn session_get(&self, key: &str) -> Option<&CursedObject> {
        self.request_context.as_ref()
            .and_then(|ctx| ctx.session.get(key))
    }

    /// Get flash message
    pub fn flash_get(&self, key: &str) -> Option<&str> {
        self.request_context.as_ref()
            .and_then(|ctx| ctx.flash.get(key).map(|s| s.as_str()))
    }

    /// Get CSRF token
    pub fn csrf_token(&self) -> Option<&str> {
        self.request_context.as_ref()
            .and_then(|ctx| ctx.csrf_token.as_deref())
    }

    /// Register a component template
    pub fn register_component(&self, component: ComponentTemplate) -> Result<(), Error> {
        let mut cache = self.component_cache.lock()
            .map_err(|e| CursedError::Runtime(format!("Component cache lock error: {}", e)))?;
        cache.insert(component.name.clone(), component);
        Ok(())
    }

    /// Get a component template
    pub fn get_component(&self, name: &str) -> Result<(), Error> {
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
    context: HtmlTemplateContext,
}

impl HtmlEscaper {
    /// Create a new HTML escaper
    pub fn new(context: HtmlTemplateContext) -> Self {
        Self { context }
    }

    /// Escape content based on context
    #[instrument(skip(self, content))]
    pub fn escape(&self, content: &str, escape_context: EscapeContext) -> Result<(), Error> {
        debug!(context = ?escape_context, content_length = content.len(), "Escaping HTML content");

        if !self.context.auto_escape {
            return Ok(content.to_string());
        }

        match escape_context {
            EscapeContext::Html => Ok(self.escape_html(content)),
            EscapeContext::HtmlAttribute => Ok(self.escape_html_attribute(content)),
            EscapeContext::JavaScript => Ok(self.escape_javascript(content)),
            EscapeContext::Css => Ok(self.escape_css(content)),
            EscapeContext::Url => Ok(self.escape_url(content)),
            EscapeContext::None => Ok(content.to_string()),
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
    }

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
    }

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
    }

    /// Escape CSS content
    fn escape_css(&self, content: &str) -> String {
        content.chars()
            .map(|c| match c {
                '"' => "\\\"".to_string(),
                '\'' => "\\'".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\A".to_string(),
                '\r' => "\\D".to_string(),
                '\t' => "\\9".to_string(),
                c if c.is_control() => format!("\\{:X}", c as u32),
                c => c.to_string(),
            })
            .collect()
    }

    /// Escape URL content
    fn escape_url(&self, content: &str) -> String {
        urlencoding::encode(content).to_string()
    }

    /// Generate HTML with CSP nonces
    #[instrument(skip(self, tag_name, attributes, content))]
    pub fn generate_html_with_csp(
        &self,
        tag_name: &str,
        attributes: &HashMap<String, String>,
        content: Option<&str>,
    ) -> Result<(), Error> {
        let mut html = format!("<{}", tag_name);

        // Add attributes
        for (name, value) in attributes {
            let escaped_value = self.escape_html_attribute(value);
            html.push_str(&format!(" {}=\"{}\"", name, escaped_value));
        }

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
        }

        html.push('>');

        // Add content if provided
        if let Some(content) = content {
            let escaped_content = match tag_name.to_lowercase().as_str() {
                "script" => self.escape_javascript(content),
                "style" => self.escape_css(content),
                _ => self.escape_html(content),
            };
            html.push_str(&escaped_content);
            html.push_str(&format!("</{}>", tag_name));
        }

        Ok(html)
    }

    /// Validate and sanitize HTML content
    #[instrument(skip(self, html))]
    pub fn sanitize_html(&self, html: &str) -> Result<(), Error> {
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
    Html,
    /// HTML attribute value
    HtmlAttribute,
    /// JavaScript content
    JavaScript,
    /// CSS content
    Css,
    /// URL content
    Url,
    /// No escaping
    None,
}

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
        tag_name: &str,
        attributes: &HashMap<String, CursedObject>,
        content: Option<&str>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut attr_map = HashMap::new();
        for (key, value) in attributes {
            let value_str = match value {
                CursedObject::String(s) => s.clone(),
                CursedObject::Integer(n) => n.to_string(),
                CursedObject::Float(n) => n.to_string(),
                CursedObject::Boolean(b) => b.to_string(),
                _ => continue,
            };
            attr_map.insert(key.clone(), value_str);
        }

        let html = escaper.generate_html_with_csp(tag_name, &attr_map, content)?;
        Ok(CursedObject::String(html))
    }

    /// Generate a link tag
    pub fn link(
        href: &str,
        text: &str,
        attributes: Option<&HashMap<String, CursedObject>>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut attr_map = HashMap::new();
        attr_map.insert("href".to_string(), href.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "href" {
                    let value_str = match value {
                        CursedObject::String(s) => s.clone(),
                        CursedObject::Integer(n) => n.to_string(),
                        CursedObject::Float(n) => n.to_string(),
                        CursedObject::Boolean(b) => b.to_string(),
                        _ => continue,
                    };
                    attr_map.insert(key.clone(), value_str);
                }
            }
        }

        let html = escaper.generate_html_with_csp("a", &attr_map, Some(text))?;
        Ok(html)
    }

    /// Generate an image tag
    pub fn img(
        src: &str,
        alt: &str,
        attributes: Option<&HashMap<String, CursedObject>>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut attr_map = HashMap::new();
        attr_map.insert("src".to_string(), src.to_string());
        attr_map.insert("alt".to_string(), alt.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "src" && key != "alt" {
                    let value_str = match value {
                        CursedObject::String(s) => s.clone(),
                        CursedObject::Integer(n) => n.to_string(),
                        CursedObject::Float(n) => n.to_string(),
                        CursedObject::Boolean(b) => b.to_string(),
                        _ => continue,
                    };
                    attr_map.insert(key.clone(), value_str);
                }
            }
        }

        let html = escaper.generate_html_with_csp("img", &attr_map, None)?;
        Ok(CursedObject::String(html))
    }

    /// Generate a form tag
    pub fn form(
        action: &str,
        method: &str,
        attributes: Option<&HashMap<String, CursedObject>>,
        content: &str,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut attr_map = HashMap::new();
        attr_map.insert("action".to_string(), action.to_string());
        attr_map.insert("method".to_string(), method.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "action" && key != "method" {
                    let value_str = match value {
                        CursedObject::String(s) => s.clone(),
                        CursedObject::Integer(n) => n.to_string(),
                        CursedObject::Float(n) => n.to_string(),
                        CursedObject::Boolean(b) => b.to_string(),
                        _ => continue,
                    };
                    attr_map.insert(key.clone(), value_str);
                }
            }
        }

        let html = escaper.generate_html_with_csp("form", &attr_map, Some(content))?;
        Ok(CursedObject::String(html))
    }

    /// Generate an input tag
    pub fn input(
        input_type: &str,
        name: &str,
        value: Option<&str>,
        attributes: Option<&HashMap<String, CursedObject>>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut attr_map = HashMap::new();
        attr_map.insert("type".to_string(), input_type.to_string());
        attr_map.insert("name".to_string(), name.to_string());

        if let Some(val) = value {
            attr_map.insert("value".to_string(), val.to_string());
        }

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "type" && key != "name" && key != "value" {
                    let value_str = match value {
                        CursedObject::String(s) => s.clone(),
                        CursedObject::Integer(n) => n.to_string(),
                        CursedObject::Float(n) => n.to_string(),
                        CursedObject::Boolean(b) => b.to_string(),
                        _ => continue,
                    };
                    attr_map.insert(key.clone(), value_str);
                }
            }
        }

        let html = escaper.generate_html_with_csp("input", &attr_map, None)?;
        Ok(CursedObject::String(html))
    }

    /// Generate CSRF protection token
    pub fn csrf_token(secret: &str) -> Result<(), Error> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Simple CSRF token generation (in production, use proper crypto)
        let token = format!("{:x}", md5::compute(format!("{}{}", secret, timestamp)));
        Ok(CursedObject::String(token))
    }

    /// Generate CSP nonce
    pub fn csp_nonce() -> Result<(), Error> {
        use rand::RngCore;
        
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 16];
        rng.fill_bytes(&mut bytes);
        
        let nonce = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
        Ok(CursedObject::String(nonce))
    }

    /// Generate a select dropdown
    pub fn select(
        name: &str,
        options: &[(String, String)], // (value, display_text) pairs
        selected: Option<&str>,
        attributes: Option<&HashMap<String, CursedObject>>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut attr_map = HashMap::new();
        attr_map.insert("name".to_string(), name.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "name" {
                    let value_str = match value {
                        CursedObject::String(s) => s.clone(),
                        CursedObject::Integer(n) => n.to_string(),
                        CursedObject::Float(n) => n.to_string(),
                        CursedObject::Boolean(b) => b.to_string(),
                        _ => continue,
                    };
                    attr_map.insert(key.clone(), value_str);
                }
            }
        }

        let mut content = String::new();
        for (value, text) in options {
            let selected_attr = if Some(value.as_str()) == selected {
                " selected"
            } else {
                ""
            };
            let escaped_value = escaper.escape_html_attribute(value);
            let escaped_text = escaper.escape_html(text);
            content.push_str(&format!(
                "<option value=\"{}\"{}>{}</option>",
                escaped_value, selected_attr, escaped_text
            ));
        }

        let html = escaper.generate_html_with_csp("select", &attr_map, Some(&content))?;
        Ok(html)
    }

    /// Generate a textarea
    pub fn textarea(
        name: &str,
        content: &str,
        attributes: Option<&HashMap<String, CursedObject>>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut attr_map = HashMap::new();
        attr_map.insert("name".to_string(), name.to_string());

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if key != "name" {
                    let value_str = match value {
                        CursedObject::String(s) => s.clone(),
                        CursedObject::Integer(n) => n.to_string(),
                        CursedObject::Float(n) => n.to_string(),
                        CursedObject::Boolean(b) => b.to_string(),
                        _ => continue,
                    };
                    attr_map.insert(key.clone(), value_str);
                }
            }
        }

        let html = escaper.generate_html_with_csp("textarea", &attr_map, Some(content))?;
        Ok(html)
    }

    /// Generate radio button group
    pub fn radio_group(
        name: &str,
        options: &[(String, String)], // (value, label) pairs
        selected: Option<&str>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut html = String::new();
        
        for (value, label) in options {
            let checked_attr = if Some(value.as_str()) == selected {
                " checked"
            } else {
                ""
            };
            
            let input_id = format!("{}_{}", name, value);
            let escaped_value = escaper.escape_html_attribute(value);
            let escaped_label = escaper.escape_html(label);
            let escaped_id = escaper.escape_html_attribute(&input_id);
            
            html.push_str(&format!(
                "<input type=\"radio\" name=\"{}\" value=\"{}\" id=\"{}\"{}><label for=\"{}\">{}</label>",
                name, escaped_value, escaped_id, checked_attr, escaped_id, escaped_label
            ));
        }
        
        Ok(html)
    }

    /// Generate checkbox
    pub fn checkbox(
        name: &str,
        value: &str,
        checked: bool,
        label: Option<&str>,
        attributes: Option<&HashMap<String, CursedObject>>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut attr_map = HashMap::new();
        attr_map.insert("type".to_string(), "checkbox".to_string());
        attr_map.insert("name".to_string(), name.to_string());
        attr_map.insert("value".to_string(), value.to_string());
        
        if checked {
            attr_map.insert("checked".to_string(), "checked".to_string());
        }

        if let Some(attrs) = attributes {
            for (key, value) in attrs {
                if !["type", "name", "value", "checked"].contains(&key.as_str()) {
                    let value_str = match value {
                        CursedObject::String(s) => s.clone(),
                        CursedObject::Integer(n) => n.to_string(),
                        CursedObject::Float(n) => n.to_string(),
                        CursedObject::Boolean(b) => b.to_string(),
                        _ => continue,
                    };
                    attr_map.insert(key.clone(), value_str);
                }
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
}

impl LayoutHelpers {
    /// Render a layout with content blocks
    #[instrument(skip(context, content_blocks))]
    pub fn render_layout(
        layout_template: &str,
        content_blocks: &HashMap<String, String>,
        context: &HtmlTemplateContext,
    ) -> Result<(), Error> {
        info!(layout_template_length = layout_template.len(), blocks_count = content_blocks.len(), "Rendering layout");
        
        let mut rendered = layout_template.to_string();
        
        // Replace content blocks
        for (block_name, content) in content_blocks {
            let placeholder = format!("{{{{ yield '{}' }}}}", block_name);
            rendered = rendered.replace(&placeholder, content);
        }
        
        // Replace main content block
        if let Some(main_content) = content_blocks.get("main") {
            rendered = rendered.replace("{{ yield }}", main_content);
        }
        
        Ok(rendered)
    }

    /// Generate meta tags for the page head
    pub fn render_meta_tags(context: &HtmlTemplateContext) -> Result<(), Error> {
        let meta_config = &context.layout_config.meta_config;
        let mut meta_html = String::new();
        
        // Title tag
        if let Some(title) = &meta_config.title {
            let escaped_title = HtmlEscaper::new(context.clone()).escape_html(title);
            meta_html.push_str(&format!("<title>{}</title>\n", escaped_title));
        }
        
        // Meta description
        if let Some(description) = &meta_config.description {
            let escaped_desc = HtmlEscaper::new(context.clone()).escape_html_attribute(description);
            meta_html.push_str(&format!("<meta name=\"description\" content=\"{}\">\n", escaped_desc));
        }
        
        // Meta keywords
        if !meta_config.keywords.is_empty() {
            let keywords = meta_config.keywords.join(", ");
            let escaped_keywords = HtmlEscaper::new(context.clone()).escape_html_attribute(&keywords);
            meta_html.push_str(&format!("<meta name=\"keywords\" content=\"{}\">\n", escaped_keywords));
        }
        
        // Custom meta tags
        for (name, content) in &meta_config.custom_meta {
            let escaped_name = HtmlEscaper::new(context.clone()).escape_html_attribute(name);
            let escaped_content = HtmlEscaper::new(context.clone()).escape_html_attribute(content);
            meta_html.push_str(&format!("<meta name=\"{}\" content=\"{}\">\n", escaped_name, escaped_content));
        }
        
        // CSRF meta tag
        if let Some(csrf_token) = context.csrf_token() {
            let escaped_token = HtmlEscaper::new(context.clone()).escape_html_attribute(csrf_token);
            meta_html.push_str(&format!("<meta name=\"csrf-token\" content=\"{}\">\n", escaped_token));
        }
        
        Ok(meta_html)
    }

    /// Render partial template
    pub fn render_partial(
        partial_name: &str,
        locals: &HashMap<String, CursedObject>,
        context: &HtmlTemplateContext,
    ) -> Result<(), Error> {
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
                message: format!("Partial template '{}' not found. Tried: {:?}", partial_name, possible_names),
                source_location: None,
            }
        })?;
        
        // Parse and render the partial template
        use super::template_syntax::{TemplateLexer, TemplateParser};
        
        let mut lexer = TemplateLexer::new(&content);
        let tokens = lexer.tokenize().map_err(|e| {
            CursedError::TemplateError {
                message: format!("Failed to tokenize partial '{}': {}", partial_name, e),
                source_location: None,
            }
        })?;
        
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().map_err(|e| {
            CursedError::TemplateError {
                message: format!("Failed to parse partial '{}': {}", partial_name, e),
                source_location: None,
            }
        })?;
        
        // Create render context with locals merged in
        let mut render_context = RenderContext::new();
        
        // Add locals to render context
        for (key, value) in locals {
            render_context.set_variable(key.clone(), value.clone());
        }
        
        // Create a basic template engine for rendering
        let engine = TemplateEngine::new(Box::new(loader));
        let mut renderer = super::template_render::TemplateRenderer::new(&engine);
        
        // Render the partial
        let rendered = renderer.render_ast(&ast, &render_context).map_err(|e| {
            CursedError::TemplateError {
                message: format!("Failed to render partial '{}': {}", partial_name, e),
                source_location: None,
            }
        })?;
        
        debug!("Successfully rendered partial '{}' ({} chars)", found_name.unwrap_or_default(), rendered.len());
        Ok(rendered)
    }
}

impl AssetHelpers {
    /// Generate stylesheet link tags
    pub fn stylesheet_links(context: &HtmlTemplateContext) -> Result<(), Error> {
        let asset_config = &context.layout_config.asset_config;
        let mut html = String::new();
        
        for stylesheet in &asset_config.stylesheets {
            let href = Self::asset_url(stylesheet, &asset_config.base_url, &asset_config.version_suffix);
            let escaped_href = HtmlEscaper::new(context.clone()).escape_html_attribute(&href);
            
            let mut link_html = format!("<link rel=\"stylesheet\" href=\"{}\"", escaped_href);
            
            // Add nonce if CSP is enabled
            if let Some(nonce) = context.style_nonce() {
                link_html.push_str(&format!(" nonce=\"{}\"", nonce));
            }
            
            link_html.push_str(">\n");
            html.push_str(&link_html);
        }
        
        Ok(html)
    }

    /// Generate JavaScript script tags
    pub fn javascript_includes(context: &HtmlTemplateContext) -> Result<(), Error> {
        let asset_config = &context.layout_config.asset_config;
        let mut html = String::new();
        
        for script in &asset_config.scripts {
            let src = Self::asset_url(script, &asset_config.base_url, &asset_config.version_suffix);
            let escaped_src = HtmlEscaper::new(context.clone()).escape_html_attribute(&src);
            
            let mut script_html = format!("<script src=\"{}\"", escaped_src);
            
            // Add nonce if CSP is enabled
            if let Some(nonce) = context.script_nonce() {
                script_html.push_str(&format!(" nonce=\"{}\"", nonce));
            }
            
            script_html.push_str("></script>\n");
            html.push_str(&script_html);
        }
        
        Ok(html)
    }

    /// Generate asset URL with versioning
    pub fn asset_url(path: &str, base_url: &str, version_suffix: &Option<String>) -> String {
        let mut url = format!("{}/{}", base_url.trim_end_matches('/'), path.trim_start_matches('/'));
        
        if let Some(version) = version_suffix {
            url.push_str(&format!("?v={}", version));
        }
        
        url
    }

    /// Generate image tag with responsive attributes
    pub fn responsive_image(
        src: &str,
        alt: &str,
        sizes: &[(u32, String)], // (width, src) pairs
        context: &HtmlTemplateContext,
    ) -> Result<(), Error> {
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
        }
        
        img_html.push('>');
        Ok(img_html)
    }
}

impl FormHelpers {
    /// Generate a complete form with CSRF protection
    pub fn form_with_csrf(
        action: &str,
        method: &str,
        attributes: Option<&HashMap<String, CursedObject>>,
        content: &str,
        context: &HtmlTemplateContext,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut form_content = String::new();
        
        // Add CSRF token as hidden field
        if let Some(csrf_token) = context.csrf_token() {
            let escaped_token = escaper.escape_html_attribute(csrf_token);
            form_content.push_str(&format!(
                "<input type=\"hidden\" name=\"_token\" value=\"{}\">\n",
                escaped_token
            ));
        }
        
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
        field_type: &str,
        name: &str,
        value: Option<&str>,
        label: Option<&str>,
        errors: &[String],
        attributes: Option<&HashMap<String, CursedObject>>,
        escaper: &HtmlEscaper,
    ) -> Result<(), Error> {
        let mut field_html = String::new();
        
        // Add label if provided
        if let Some(label_text) = label {
            let escaped_label = escaper.escape_html(label_text);
            let escaped_name = escaper.escape_html_attribute(name);
            field_html.push_str(&format!("<label for=\"{}\">{}</label>\n", escaped_name, escaped_label));
        }
        
        // Add the input field
        let input_result = HtmlTemplateHelpers::input(field_type, name, value, attributes, escaper)?;
        if let CursedObject::String(input_html) = input_result {
            // Add id attribute for label association
            let input_with_id = input_html.replace(">", &format!(" id=\"{}\">", escaper.escape_html_attribute(name)));
            field_html.push_str(&input_with_id);
        }
        
        // Add validation errors
        if !errors.is_empty() {
            field_html.push_str("<div class=\"errors\">\n");
            for error in errors {
                let escaped_error = escaper.escape_html(error);
                field_html.push_str(&format!("<span class=\"error\">{}</span>\n", escaped_error));
            }
            field_html.push_str("</div>\n");
        }
        
        Ok(field_html)
    }
}

impl ComponentSystem {
    /// Render a component with parameters
    #[instrument(skip(context, parameters))]
    pub fn render_component(
        component_name: &str,
        parameters: &HashMap<String, CursedObject>,
        context: &HtmlTemplateContext,
    ) -> Result<(), Error> {
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
                CursedObject::String(s) => s.clone(),
                CursedObject::Integer(n) => n.to_string(),
                CursedObject::Float(f) => f.to_string(),
                CursedObject::Boolean(b) => b.to_string(),
                _ => format!("{:?}", param_value),
            };
            
            let placeholder = format!("{{{{ {} }}}}", param_name);
            rendered = rendered.replace(&placeholder, &value_str);
        }
        
        Ok(rendered)
    }

    /// Validate component parameters
    fn validate_parameters(
        component: &ComponentTemplate,
        parameters: &HashMap<String, CursedObject>,
    ) -> Result<(), Error> {
        for param_def in &component.parameters {
            if param_def.required && !parameters.contains_key(&param_def.name) {
                return Err(CursedError::Runtime(format!(
                    "Required parameter '{}' missing for component '{}'",
                    param_def.name, component.name
                )));
            }
            
            // Type validation (simplified)
            if let Some(value) = parameters.get(&param_def.name) {
                let valid = match (&param_def.param_type, value) {
                    (ComponentParameterType::String, CursedObject::String(_)) => true,
                    (ComponentParameterType::Integer, CursedObject::Integer(_)) => true,
                    (ComponentParameterType::Float, CursedObject::Float(_)) => true,
                    (ComponentParameterType::Boolean, CursedObject::Boolean(_)) => true,
                    (ComponentParameterType::Object, _) => true, // Accept any object
                    (ComponentParameterType::Array, _) => true,  // Accept any array-like
                    _ => false,
                };
                
                if !valid {
                    return Err(CursedError::Runtime(format!(
                        "Parameter '{}' has invalid type for component '{}'",
                        param_def.name, component.name
                    )));
                }
            }
        }
        
        Ok(())
    }

    /// Create a new component template
    pub fn create_component(
        name: String,
        template: String,
        parameters: Vec<ComponentParameter>,
        cacheable: bool,
    ) -> ComponentTemplate {
        ComponentTemplate {
            name,
            template,
            parameters,
            cacheable,
        }
    }

    /// Create a component parameter definition
    pub fn create_parameter(
        name: String,
        param_type: ComponentParameterType,
        required: bool,
        default_value: Option<CursedObject>,
    ) -> ComponentParameter {
        ComponentParameter {
            name,
            param_type,
            required,
            default_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escaping() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let content = "<script>alert('xss')</script>";
        let escaped = escaper.escape(content, EscapeContext::Html).unwrap();
        
        assert!(escaped.contains("&lt;script&gt;"));
        assert!(escaped.contains("&#x27;"));
    }

    #[test]
    fn test_javascript_escaping() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let content = "alert('hello\nworld')";
        let escaped = escaper.escape(content, EscapeContext::JavaScript).unwrap();
        
        assert!(escaped.contains("\\'"));
        assert!(escaped.contains("\\n"));
    }

    #[test]
    fn test_url_escaping() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let content = "hello world & more";
        let escaped = escaper.escape(content, EscapeContext::Url).unwrap();
        
        assert!(escaped.contains("%20")); // space
        assert!(escaped.contains("%26")); // ampersand
    }

    #[test]
    fn test_html_tag_generation() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let mut attributes = HashMap::new();
        attributes.insert("class".to_string(), "btn btn-primary".to_string());
        attributes.insert("id".to_string(), "submit-btn".to_string());

        let html = escaper.generate_html_with_csp("button", &attributes, Some("Click me")).unwrap();
        
        assert!(html.contains("<button"));
        assert!(html.contains("class=\"btn btn-primary\""));
        assert!(html.contains("id=\"submit-btn\""));
        assert!(html.contains("Click me"));
        assert!(html.contains("</button>"));
    }

    #[test]
    fn test_csp_nonce_generation() {
        let mut csp = CspSettings::default();
        csp.generate_nonces = true;
        csp.script_nonce = Some("abc123".to_string());

        let context = HtmlTemplateContext::with_csp(csp);
        let escaper = HtmlEscaper::new(context);

        let html = escaper.generate_html_with_csp("script", &HashMap::new(), Some("console.log('test')")).unwrap();
        
        assert!(html.contains("nonce=\"abc123\""));
    }

    #[test]
    fn test_html_sanitization() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let dangerous_html = "<script>alert('xss')</script><p onclick=\"alert('click')\">Hello</p>";
        let sanitized = escaper.sanitize_html(dangerous_html).unwrap();
        
        assert!(!sanitized.contains("<script>"));
        assert!(!sanitized.contains("onclick"));
        assert!(sanitized.contains("<p>"));
        assert!(sanitized.contains("Hello"));
    }

    #[test]
    fn test_safe_content_marking() {
        let mut context = HtmlTemplateContext::new();
        context.mark_safe_html("trusted_content".to_string());

        assert!(context.is_safe_content("trusted_content", &SafeContentType::Html));
        assert!(!context.is_safe_content("trusted_content", &SafeContentType::JavaScript));
        assert!(!context.is_safe_content("untrusted_content", &SafeContentType::Html));
    }

    #[test]
    fn test_select_generation() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let options = vec![
            ("value1".to_string(), "Option 1".to_string()),
            ("value2".to_string(), "Option 2".to_string()),
        ];

        let html = HtmlTemplateHelpers::select("test_select", &options, Some("value1"), None, &escaper)
            .expect("Failed to generate select HTML - expected String result but got different type");
        assert!(html.contains("<select"));
        assert!(html.contains("name=\"test_select\""));
        assert!(html.contains("<option value=\"value1\" selected>Option 1</option>"));
        assert!(html.contains("<option value=\"value2\">Option 2</option>"));
    }

    #[test]
    fn test_textarea_generation() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let html = HtmlTemplateHelpers::textarea("message", "Hello world", None, &escaper)
            .expect("Failed to generate textarea HTML - expected String result but got different type");
        assert!(html.contains("<textarea"));
        assert!(html.contains("name=\"message\""));
        assert!(html.contains("Hello world"));
        assert!(html.contains("</textarea>"));
    }

    #[test]
    fn test_radio_group_generation() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let options = vec![
            ("yes".to_string(), "Yes".to_string()),
            ("no".to_string(), "No".to_string()),
        ];

        let html = HtmlTemplateHelpers::radio_group("choice", &options, Some("yes"), &escaper)
            .expect("Failed to generate radio group HTML - expected String result but got different type");
        assert!(html.contains("type=\"radio\""));
        assert!(html.contains("name=\"choice\""));
        assert!(html.contains("value=\"yes\" id=\"choice_yes\" checked"));
        assert!(html.contains("value=\"no\" id=\"choice_no\">"));
        assert!(html.contains("<label for=\"choice_yes\">Yes</label>"));
        assert!(html.contains("<label for=\"choice_no\">No</label>"));
    }

    #[test]
    fn test_checkbox_generation() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);

        let html = HtmlTemplateHelpers::checkbox("agree", "1", true, Some("I agree"), None, &escaper)
            .expect("Failed to generate checkbox HTML - expected String result but got different type");
        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("name=\"agree\""));
        assert!(html.contains("value=\"1\""));
        assert!(html.contains("checked"));
        assert!(html.contains("<label for=\"agree_1\">I agree</label>"));
    }

    #[test]
    fn test_layout_content_blocks() {
        let mut context = HtmlTemplateContext::new();
        context.set_content_block("sidebar".to_string(), "<div>Sidebar content</div>".to_string());

        assert_eq!(context.get_content_block("sidebar"), Some("<div>Sidebar content</div>"));
        assert_eq!(context.get_content_block("nonexistent"), None);
    }

    #[test]
    fn test_meta_configuration() {
        let mut context = HtmlTemplateContext::new();
        context.set_title("Test Page".to_string());
        context.set_description("A test page description".to_string());
        context.add_keyword("test".to_string());
        context.add_keyword("page".to_string());
        context.set_meta("author".to_string(), "Test Author".to_string());

        assert_eq!(context.title(), Some("Test Page"));
        assert_eq!(context.layout_config.meta_config.description, Some("A test page description".to_string()));
        assert_eq!(context.layout_config.meta_config.keywords, vec!["test", "page"]);
        assert_eq!(context.layout_config.meta_config.custom_meta.get("author"), Some(&"Test Author".to_string()));
    }

    #[test]
    fn test_asset_configuration() {
        let mut context = HtmlTemplateContext::new();
        context.add_stylesheet("styles/main.css".to_string());
        context.add_script("js/app.js".to_string());

        assert_eq!(context.layout_config.asset_config.stylesheets, vec!["styles/main.css"]);
        assert_eq!(context.layout_config.asset_config.scripts, vec!["js/app.js"]);
    }

    #[test]
    fn test_layout_rendering() {
        let context = HtmlTemplateContext::new();
        let mut content_blocks = HashMap::new();
        content_blocks.insert("main".to_string(), "<h1>Main Content</h1>".to_string());
        content_blocks.insert("sidebar".to_string(), "<div>Sidebar</div>".to_string());

        let layout_template = r#"
        <html>
        <body>
            <main>{{ yield }}</main>
            <aside>{{ yield 'sidebar' }}</aside>
        </body>
        </html>
        "#;

        let result = LayoutHelpers::render_layout(layout_template, &content_blocks, &context).unwrap();
        assert!(result.contains("<main><h1>Main Content</h1></main>"));
        assert!(result.contains("<aside><div>Sidebar</div></aside>"));
    }

    #[test]
    fn test_meta_tags_rendering() {
        let mut context = HtmlTemplateContext::new();
        context.set_title("Test Page".to_string());
        context.set_description("Test description".to_string());
        context.add_keyword("test".to_string());
        context.set_meta("author".to_string(), "Test Author".to_string());

        let result = LayoutHelpers::render_meta_tags(&context).unwrap();
        assert!(result.contains("<title>Test Page</title>"));
        assert!(result.contains("<meta name=\"description\" content=\"Test description\">"));
        assert!(result.contains("<meta name=\"keywords\" content=\"test\">"));
        assert!(result.contains("<meta name=\"author\" content=\"Test Author\">"));
    }

    #[test]
    fn test_asset_url_generation() {
        let base_url = "/assets";
        let version_suffix = Some("v1.2.3".to_string());

        let url = AssetHelpers::asset_url("css/style.css", base_url, &version_suffix);
        assert_eq!(url, "/assets/css/style.css?v=v1.2.3");

        let url_no_version = AssetHelpers::asset_url("js/app.js", base_url, &None);
        assert_eq!(url_no_version, "/assets/js/app.js");
    }

    #[test]
    fn test_stylesheet_links_generation() {
        let mut context = HtmlTemplateContext::new();
        context.add_stylesheet("css/main.css".to_string());
        context.add_stylesheet("css/theme.css".to_string());

        let result = AssetHelpers::stylesheet_links(&context).unwrap();
        assert!(result.contains("<link rel=\"stylesheet\" href=\"/assets/css/main.css\">"));
        assert!(result.contains("<link rel=\"stylesheet\" href=\"/assets/css/theme.css\">"));
    }

    #[test]
    fn test_javascript_includes_generation() {
        let mut context = HtmlTemplateContext::new();
        context.add_script("js/app.js".to_string());
        context.add_script("js/utils.js".to_string());

        let result = AssetHelpers::javascript_includes(&context).unwrap();
        assert!(result.contains("<script src=\"/assets/js/app.js\"></script>"));
        assert!(result.contains("<script src=\"/assets/js/utils.js\"></script>"));
    }

    #[test]
    fn test_responsive_image_generation() {
        let context = HtmlTemplateContext::new();
        let sizes = vec![
            (320, "images/small.jpg".to_string()),
            (768, "images/medium.jpg".to_string()),
            (1024, "images/large.jpg".to_string()),
        ];

        let result = AssetHelpers::responsive_image("images/default.jpg", "Test image", &sizes, &context).unwrap();
        assert!(result.contains("src=\"images/default.jpg\""));
        assert!(result.contains("alt=\"Test image\""));
        assert!(result.contains("srcset=\"images/small.jpg 320w, images/medium.jpg 768w, images/large.jpg 1024w\""));
    }

    #[test]
    fn test_form_with_csrf() {
        let mut request_context = RequestContext {
            path: "/test".to_string(),
            method: "POST".to_string(),
            session: HashMap::new(),
            flash: HashMap::new(),
            csrf_token: Some("csrf-token-123".to_string()),
            user: None,
        };

        let context = HtmlTemplateContext::with_request_context(request_context);
        let escaper = HtmlEscaper::new(context.clone());

        let html = FormHelpers::form_with_csrf("/submit", "POST", None, "<input type=\"text\" name=\"data\">", &context, &escaper).unwrap();
        assert!(html.contains("<form"));
        assert!(html.contains("action=\"/submit\""));
        assert!(html.contains("method=\"POST\""));
        assert!(html.contains("<input type=\"hidden\" name=\"_token\" value=\"csrf-token-123\">"));
        assert!(html.contains("<input type=\"text\" name=\"data\">"));
    }

    #[test]
    fn test_form_field_with_validation() {
        let context = HtmlTemplateContext::new();
        let escaper = HtmlEscaper::new(context);
        let errors = vec!["Field is required".to_string(), "Field must be valid".to_string()];

        let result = FormHelpers::form_field("text", "username", Some("testuser"), Some("Username"), &errors, None, &escaper).unwrap();
        assert!(result.contains("<label for=\"username\">Username</label>"));
        assert!(result.contains("type=\"text\""));
        assert!(result.contains("name=\"username\""));
        assert!(result.contains("value=\"testuser\""));
        assert!(result.contains("id=\"username\""));
        assert!(result.contains("<div class=\"errors\">"));
        assert!(result.contains("<span class=\"error\">Field is required</span>"));
        assert!(result.contains("<span class=\"error\">Field must be valid</span>"));
    }

    #[test]
    fn test_component_creation_and_registration() {
        let context = HtmlTemplateContext::new();
        
        let parameters = vec![
            ComponentSystem::create_parameter("title".to_string(), ComponentParameterType::String, true, None),
            ComponentSystem::create_parameter("count".to_string(), ComponentParameterType::Integer, false, Some(CursedObject::Integer(0))),
        ];

        let component = ComponentSystem::create_component(
            "test_component".to_string(),
            "<h1>{{ title }}</h1><p>Count: {{ count }}</p>".to_string(),
            parameters,
            true,
        );

        context.register_component(component).unwrap();
        let retrieved = context.get_component("test_component").unwrap();
        assert!(retrieved.is_some());
        
        let component = retrieved.unwrap();
        assert_eq!(component.name, "test_component");
        assert_eq!(component.parameters.len(), 2);
        assert!(component.cacheable);
    }

    #[test]
    fn test_component_rendering() {
        let context = HtmlTemplateContext::new();
        
        let parameters = vec![
            ComponentSystem::create_parameter("title".to_string(), ComponentParameterType::String, true, None),
        ];

        let component = ComponentSystem::create_component(
            "simple_component".to_string(),
            "<h1>{{ title }}</h1>".to_string(),
            parameters,
            true,
        );

        context.register_component(component).unwrap();

        let mut render_params = HashMap::new();
        render_params.insert("title".to_string(), CursedObject::String("Hello World".to_string()));

        let result = ComponentSystem::render_component("simple_component", &render_params, &context).unwrap();
        assert_eq!(result, "<h1>Hello World</h1>");
    }

    #[test]
    fn test_component_parameter_validation() {
        let context = HtmlTemplateContext::new();
        
        let parameters = vec![
            ComponentSystem::create_parameter("required_param".to_string(), ComponentParameterType::String, true, None),
        ];

        let component = ComponentSystem::create_component(
            "validation_component".to_string(),
            "{{ required_param }}".to_string(),
            parameters,
            false,
        );

        context.register_component(component).unwrap();

        // Test missing required parameter
        let empty_params = HashMap::new();
        let result = ComponentSystem::render_component("validation_component", &empty_params, &context);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Required parameter 'required_param' missing"));

        // Test valid parameters
        let mut valid_params = HashMap::new();
        valid_params.insert("required_param".to_string(), CursedObject::String("test".to_string()));
        let result = ComponentSystem::render_component("validation_component", &valid_params, &context);
        assert!(result.is_ok());
    }
}
