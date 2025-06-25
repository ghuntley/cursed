use crate::error::CursedError;
/// Security utilities for input validation, XSS protection, and more
use std::collections::HashMap;
use std::fmt;
use regex::Regex;

/// Input sanitizer for cleaning user input
pub struct InputSanitizer {
#[derive(Debug, Clone)]
pub struct SanitizerConfig {
impl Default for SanitizerConfig {
    fn default() -> Self {
        Self {
        }
    }
impl InputSanitizer {
    /// Create new input sanitizer with default config
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create sanitizer with custom config
    pub fn with_config(config: SanitizerConfig) -> Self {
        Self { config }
    }

    /// Sanitize input string
    pub fn sanitize(&self, input: &str) -> String {
        let mut result = input.to_string();

        // Apply length limit
        if let Some(max_len) = self.config.max_length {
            if result.len() > max_len {
                result.truncate(max_len);
            }
        }

        // Strip whitespace
        if self.config.strip_whitespace {
            result = result.trim().to_string();
        // Handle HTML
        if !self.config.allow_html {
            result = self.escape_html(&result);
        } else {
            result = self.filter_html(&result);
        // Escape quotes
        if self.config.escape_quotes {
            result = result.replace('"', "&quot;").replace('\'', "&#x27;");
        result
    /// Sanitize for SQL to prevent injection
    pub fn sanitize_sql(&self, input: &str) -> String {
        input
            .replace('\'', "''")
            .replace(';', "")
            .replace("--", "")
            .replace("/*", "")
            .replace("*/", "")
            .replace("xp_", "")
            .replace("sp_", "")
    /// Sanitize filename to prevent path traversal
    pub fn sanitize_filename(&self, filename: &str) -> String {
        filename
            .replace("..", "")
            .replace('/', "")
            .replace('\\', "")
            .replace(':', "")
            .replace('*', "")
            .replace('?', "")
            .replace('"', "")
            .replace('<', "")
            .replace('>', "")
            .replace('|', "")
            .chars()
            .filter(|c| !c.is_control())
            .collect()
    /// Escape HTML entities
    fn escape_html(&self, input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    /// Filter HTML to allowed tags only
    fn filter_html(&self, input: &str) -> String {
        // Simple HTML filtering - remove disallowed tags
        let mut result = input.to_string();
        
        // Remove script tags and their content
        while let Some(start) = result.find("<script") {
            if let Some(end) = result[start..].find("</script>") {
                result.replace_range(start..start + end + 9, "");
            } else {
                result.replace_range(start.., "");
                break;
            }
        }

        // Remove other dangerous tags
        let dangerous_tags = ["iframe", "object", "embed", "form", "input"];
        for tag in &dangerous_tags {
            let start_tag = format!("<{}", tag);
            let end_tag = format!("</{}>", tag);
            
            while let Some(start) = result.find(&start_tag) {
                if let Some(end) = result[start..].find(&end_tag) {
                    result.replace_range(start..start + end + end_tag.len(), "");
                } else if let Some(end) = result[start..].find('>') {
                    result.replace_range(start..start + end + 1, "");
                } else {
                    result.replace_range(start.., "");
                    break;
                }
            }
        result
    /// Validate email format
    pub fn validate_email(&self, email: &str) -> bool {
        let email = email.trim();
        if email.is_empty() || email.len() > 254 {
            return false;
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return false;
        let local = parts[0];
        let domain = parts[1];

        // Basic validation
        !local.is_empty() 
            && !domain.is_empty() 
            && local.len() <= 64 
            && domain.contains('.') 
            && !domain.starts_with('.') 
            && !domain.ends_with('.')
    /// Validate URL format
    pub fn validate_url(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}

impl Default for InputSanitizer {
    fn default() -> Self {
        Self::new()
    }
}

/// XSS protection utilities
pub struct XssProtector {
impl XssProtector {
    /// Create new XSS protector
    pub fn new() -> Self {
        Self {
        }
    }

    /// Enable strict mode (more aggressive protection)
    pub fn strict(mut self) -> Self {
        self.strict_mode = true;
        self
    /// Set Content Security Policy
    pub fn with_csp(mut self, csp: String) -> Self {
        self.content_security_policy = Some(csp);
        self
    /// Get security headers for XSS protection
    pub fn get_security_headers(&self) -> Vec<(String, String)> {
        let mut headers = vec![
        ];

        if let Some(csp) = &self.content_security_policy {
            headers.push(("Content-Security-Policy".to_string(), csp.clone()));
        if self.strict_mode {
                         "max-age=31536000; includeSubDomains".to_string()));
        headers
    /// Check if content contains potential XSS
    pub fn detect_xss(&self, content: &str) -> bool {
        let dangerous_patterns = [
        ];

        let content_lower = content.to_lowercase();
        dangerous_patterns.iter().any(|pattern| content_lower.contains(pattern))
    /// Sanitize content to prevent XSS
    pub fn sanitize_content(&self, content: &str) -> String {
        let sanitizer = InputSanitizer::new();
        sanitizer.sanitize(content)
    /// Encode output for safe HTML insertion
    pub fn encode_output(&self, output: &str) -> String {
        output
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('/', "&#x2F;")
    /// Encode for JavaScript string context
    pub fn encode_js_string(&self, input: &str) -> String {
        input
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\'', "\\'")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
            .replace('\u{08}', "\\b")
            .replace('\u{0C}', "\\f")
    /// Encode for CSS context
    pub fn encode_css(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c.to_string()
                } else {
                    format!("\\{:X}", c as u32)
                }
            })
            .collect()
    /// Encode for URL context
    pub fn encode_url(&self, input: &str) -> String {
        input
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || "-_.~".contains(c) {
                    c.to_string()
                } else {
                    format!("%{:02X}", c as u8)
                }
            })
            .collect()
    }
}

impl Default for XssProtector {
    fn default() -> Self {
        Self::new()
    }
}

/// Input validation utilities
pub struct InputValidator {
#[derive(Debug, Clone)]
pub struct ValidationRule {
#[derive(Debug)]
pub struct ValidationError {
// impl fmt::Display for ValidationError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}: {}", self.field, self.message)
//     }
// }

// impl std::error::CursedError for ValidationError {}
// 
impl InputValidator {
    /// Create new input validator
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add validation rule for field
    pub fn add_rule(&mut self, field: String, rule: ValidationRule) {
        self.rules.insert(field, rule);
    /// Add required field rule
    pub fn required(&mut self, field: &str) -> &mut Self {
        self.rules.insert(
            ValidationRule {
        );
        self
    /// Add length validation rule
    pub fn length(&mut self, field: &str, min: Option<usize>, max: Option<usize>) -> &mut Self {
        // Get or create the validation rule for this field
        if !self.rules.contains_key(field) {
            self.rules.insert(field.to_string(), ValidationRule {
            });
        // Safe to unwrap because we just ensured the key exists
        let rule = self.rules.get_mut(field).unwrap();
        rule.min_length = min;
        rule.max_length = max;
        self
    /// Add pattern validation rule
    pub fn pattern(&mut self, field: &str, pattern: &str) -> &mut Self {
        // Get or create the validation rule for this field
        if !self.rules.contains_key(field) {
            self.rules.insert(field.to_string(), ValidationRule {
            });
        // Safe to unwrap because we just ensured the key exists
        let rule = self.rules.get_mut(field).unwrap();
        rule.pattern = Some(pattern.to_string());

        // Pre-compile regex patterns for better performance and early error detection
        if let Err(e) = self.compile_pattern(pattern) {
            // Log warning but don't fail - will be handled during validation
            eprintln!("Warning: Failed to compile regex pattern '{}' for field '{}': {}", pattern, field, e);
        self
    /// Compile and cache a regex pattern
    fn compile_pattern(&mut self, pattern: &str) -> Result<&Regex, String> {
        if let Some(compiled) = self.compiled_patterns.get(pattern) {
            return Ok(compiled);
        // Handle built-in patterns with improved regex patterns
        let regex_pattern = match pattern {
            "url" => r"^https?://[a-zA-Z0-9.-]+(?:\.[a-zA-Z]{2,})+(?:/[^\s]*)?$",
            _ => pattern, // Use as-is for custom regex patterns

        // Validate and compile the regex pattern
        match Regex::new(regex_pattern) {
            Ok(compiled_regex) => {
                self.compiled_patterns.insert(pattern.to_string(), compiled_regex);
                // Safe to unwrap because we just inserted it
                Ok(self.compiled_patterns.get(pattern).unwrap())
            }
        }
    }

    /// Validate input data
    pub fn validate(&self, data: &HashMap<String, String>) -> crate::error::Result<()> {
        let mut errors = Vec::new();

        for (field, rule) in &self.rules {
            if let Some(value) = data.get(field) {
                // Check length constraints
                if let Some(min_len) = rule.min_length {
                    if value.len() < min_len {
                        errors.push(ValidationError {
                        });
                    }
                }

                if let Some(max_len) = rule.max_length {
                    if value.len() > max_len {
                        errors.push(ValidationError {
                        });
                    }
                }

                // Check pattern
                if let Some(pattern) = &rule.pattern {
                    if !self.matches_pattern(value, pattern) {
                        errors.push(ValidationError {
                        });
                    }
                }
            } else if rule.required {
                errors.push(ValidationError {
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Pattern matching with regex support
    fn matches_pattern(&self, value: &str, pattern: &str) -> bool {
        // Try to get compiled regex pattern first
        if let Some(regex) = self.compiled_patterns.get(pattern) {
            return regex.is_match(value);
        // Fallback to basic pattern matching for built-in patterns
        match pattern {
            "email" => {
                let sanitizer = InputSanitizer::new();
                sanitizer.validate_email(value)
            }
            "url" => {
                let sanitizer = InputSanitizer::new();
                sanitizer.validate_url(value)
            }
            "phone" => {
                // Basic phone validation - at least 10 digits with optional formatting
                let digit_count = value.chars().filter(|c| c.is_ascii_digit()).count();
                digit_count >= 10 && value.len() >= 10
            }
            "password_strong" => {
                // Strong password: 8+ chars, mixed case, number, special char
                value.len() >= 8 
                    && value.chars().any(|c| c.is_lowercase())
                    && value.chars().any(|c| c.is_uppercase())
                    && value.chars().any(|c| c.is_ascii_digit())
                    && value.chars().any(|c| "@$!%*?&".contains(c))
            }
            "ipv4" => {
                // Basic IPv4 validation
                let parts: Vec<&str> = value.split('.').collect();
                parts.len() == 4 && parts.iter().all(|part| {
                    part.parse::<u8>().is_ok()
                })
            }
            "hex_color" => {
                // Hex color validation #RRGGBB
                value.len() == 7 
                    && value.starts_with('#')
                    && value[1..].chars().all(|c| c.is_ascii_hexdigit())
            }
            "username" => {
                // Username: 3-20 chars, letters, numbers, underscore
                value.len() >= 3 && value.len() <= 20
                    && value.chars().all(|c| c.is_alphanumeric() || c == '_')
            }
            _ => {
                // Try to compile pattern on-demand for custom regex
                match Regex::new(pattern) {
                    Err(e) => {
                        // Invalid regex pattern - log warning and fail validation
                        eprintln!("Warning: Invalid regex pattern '{}' used in validation: {}", pattern, e);
                        false
                    }
                }
            }
        }
    /// Validate a single value against a pattern with detailed error information
    pub fn validate_pattern(&mut self, value: &str, pattern: &str) -> Result<bool, String> {
        // Compile pattern first to catch regex errors
        match self.compile_pattern(pattern) {
        }
    }

    /// Get list of available built-in patterns
    pub fn get_builtin_patterns() -> Vec<(&'static str, &'static str)> {
        vec![
            ("url", "Valid HTTP/HTTPS URL"),
            ("postal_code", "Postal/ZIP code"),
        ]
    /// Test a pattern against sample values for validation
    pub fn test_pattern(&mut self, pattern: &str, test_values: &[(&str, bool)]) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        
        // Compile the pattern first
        self.compile_pattern(pattern)?;
        
        for (value, expected) in test_values {
            let actual = self.matches_pattern(value, pattern);
            if actual != *expected {
                results.push(format!(
                    pattern, value, expected, actual
                ));
            }
        }
        
        Ok(results)
    }
}

impl Default for InputValidator {
    fn default() -> Self {
        Self::new()
    }
}


impl SecurityHeaders {
    /// Create new security headers with safe defaults
    pub fn new() -> Self {
        Self {
        }
    }

    /// Convert to HTTP headers vector
    pub fn to_headers(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        
        if let Some(val) = &self.x_frame_options {
            headers.push(("X-Frame-Options".to_string(), val.clone()));
        }
        if let Some(val) = &self.x_content_type_options {
            headers.push(("X-Content-Type-Options".to_string(), val.clone()));
        }
        if let Some(val) = &self.x_xss_protection {
            headers.push(("X-XSS-Protection".to_string(), val.clone()));
        }
        if let Some(val) = &self.strict_transport_security {
            headers.push(("Strict-Transport-Security".to_string(), val.clone()));
        }
        if let Some(val) = &self.content_security_policy {
            headers.push(("Content-Security-Policy".to_string(), val.clone()));
        }
        if let Some(val) = &self.referrer_policy {
            headers.push(("Referrer-Policy".to_string(), val.clone()));
        }
        if let Some(val) = &self.permissions_policy {
            headers.push(("Permissions-Policy".to_string(), val.clone()));
        for (key, value) in &self.custom_headers {
            headers.push((key.clone(), value.clone()));
        headers
    /// Set custom header
    pub fn set_custom_header(&mut self, name: String, value: String) {
        self.custom_headers.insert(name, value);
    }
}

impl Default for SecurityHeaders {
    fn default() -> Self {
        Self::new()
    }
}

/// Content Security Policy configuration and management
#[derive(Debug, Clone)]
pub struct ContentSecurityPolicy {
impl ContentSecurityPolicy {
    /// Create new CSP with safe defaults
    pub fn new() -> Self {
        Self {
        }
    }

    /// Convert to CSP header string
    pub fn to_header_string(&self) -> String {
        let mut directives = Vec::new();
        
        if !self.default_src.is_empty() {
            directives.push(format!("default-src {}", self.default_src.join(" ")));
        }
        if !self.script_src.is_empty() {
            directives.push(format!("script-src {}", self.script_src.join(" ")));
        }
        if !self.style_src.is_empty() {
            directives.push(format!("style-src {}", self.style_src.join(" ")));
        }
        if !self.img_src.is_empty() {
            directives.push(format!("img-src {}", self.img_src.join(" ")));
        }
        if !self.connect_src.is_empty() {
            directives.push(format!("connect-src {}", self.connect_src.join(" ")));
        }
        if !self.font_src.is_empty() {
            directives.push(format!("font-src {}", self.font_src.join(" ")));
        }
        if !self.object_src.is_empty() {
            directives.push(format!("object-src {}", self.object_src.join(" ")));
        }
        if !self.media_src.is_empty() {
            directives.push(format!("media-src {}", self.media_src.join(" ")));
        }
        if !self.frame_src.is_empty() {
            directives.push(format!("frame-src {}", self.frame_src.join(" ")));
        }
        if !self.sandbox.is_empty() {
            directives.push(format!("sandbox {}", self.sandbox.join(" ")));
        }
        if let Some(uri) = &self.report_uri {
            directives.push(format!("report-uri {}", uri));
        }
        if let Some(report_to) = &self.report_to {
            directives.push(format!("report-to {}", report_to));
        }
        if !self.require_trusted_types_for.is_empty() {
            directives.push(format!("require-trusted-types-for {}", self.require_trusted_types_for.join(" ")));
        }
        if !self.trusted_types.is_empty() {
            directives.push(format!("trusted-types {}", self.trusted_types.join(" ")));
        }
        if self.upgrade_insecure_requests {
            directives.push("upgrade-insecure-requests".to_string());
        }
        if self.block_all_mixed_content {
            directives.push("block-all-mixed-content".to_string());
        directives.join("; ")
    /// Allow unsafe inline scripts (use cautiously)
    pub fn allow_unsafe_inline_scripts(&mut self) {
        if !self.script_src.contains(&"'unsafe-inline'".to_string()) {
            self.script_src.push("'unsafe-inline'".to_string());
        }
    }

    /// Allow unsafe eval (use very cautiously)
    pub fn allow_unsafe_eval(&mut self) {
        if !self.script_src.contains(&"'unsafe-eval'".to_string()) {
            self.script_src.push("'unsafe-eval'".to_string());
        }
    }

    /// Add allowed domain to script sources
    pub fn add_script_domain(&mut self, domain: String) {
        if !self.script_src.contains(&domain) {
            self.script_src.push(domain);
        }
    }

    /// Add allowed domain to style sources
    pub fn add_style_domain(&mut self, domain: String) {
        if !self.style_src.contains(&domain) {
            self.style_src.push(domain);
        }
    }
impl Default for ContentSecurityPolicy {
    fn default() -> Self {
        Self::new()
    }
}
