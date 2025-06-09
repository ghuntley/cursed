/// HTML Template Features - HTML-specific templating functionality
use std::collections::HashMap;
use tracing::{debug, instrument, warn};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;

/// HTML template context with auto-escaping and safe content handling
#[derive(Debug, Clone)]
pub struct HtmlTemplateContext {
    /// Whether to auto-escape HTML by default
    auto_escape: bool,
    /// Content Security Policy settings
    csp_settings: CspSettings,
    /// Safe content markers
    safe_content: HashMap<String, SafeContentType>,
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

impl HtmlTemplateContext {
    /// Create a new HTML template context
    pub fn new() -> Self {
        Self {
            auto_escape: true,
            csp_settings: CspSettings::default(),
            safe_content: HashMap::new(),
        }
    }

    /// Create context with auto-escaping disabled
    pub fn with_auto_escape(auto_escape: bool) -> Self {
        Self {
            auto_escape,
            csp_settings: CspSettings::default(),
            safe_content: HashMap::new(),
        }
    }

    /// Create context with CSP settings
    pub fn with_csp(csp_settings: CspSettings) -> Self {
        Self {
            auto_escape: true,
            csp_settings,
            safe_content: HashMap::new(),
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
    pub fn escape(&self, content: &str, escape_context: EscapeContext) -> Result<String, CursedError> {
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
    ) -> Result<String, CursedError> {
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
    pub fn sanitize_html(&self, html: &str) -> Result<String, CursedError> {
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

/// HTML template helper functions
pub struct HtmlTemplateHelpers;

impl HtmlTemplateHelpers {
    /// Generate an HTML tag with attributes
    pub fn tag(
        tag_name: &str,
        attributes: &HashMap<String, CursedObject>,
        content: Option<&str>,
        escaper: &HtmlEscaper,
    ) -> Result<CursedObject, CursedError> {
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
    ) -> Result<CursedObject, CursedError> {
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
        Ok(CursedObject::String(html))
    }

    /// Generate an image tag
    pub fn img(
        src: &str,
        alt: &str,
        attributes: Option<&HashMap<String, CursedObject>>,
        escaper: &HtmlEscaper,
    ) -> Result<CursedObject, CursedError> {
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
    ) -> Result<CursedObject, CursedError> {
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
    ) -> Result<CursedObject, CursedError> {
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
    pub fn csrf_token(secret: &str) -> Result<CursedObject, CursedError> {
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
    pub fn csp_nonce() -> Result<CursedObject, CursedError> {
        use rand::RngCore;
        
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 16];
        rng.fill_bytes(&mut bytes);
        
        let nonce = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
        Ok(CursedObject::String(nonce))
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
}
