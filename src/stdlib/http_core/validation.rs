use crate::error::CursedError;
/// HTTP Validation and Security for CURSED web_vibez
///
/// Comprehensive validation rules and security checks for HTTP processing.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::time::{Duration, Instant};

// use crate::stdlib::http_core::{
    Request, Response, HeaderMap, HttpError, HttpResult, ContentType, MimeType
};

/// Validation rule types
#[derive(Debug, Clone)]
pub enum ValidationRule {
    /// Required field validation
    Required,
    /// Minimum length validation
    MinLength(usize),
    /// Maximum length validation
    MaxLength(usize),
    /// Pattern validation (regex)
    Pattern(String),
    /// Email format validation
    Email,
    /// URL format validation
    Url,
    /// IP address validation
    IpAddress,
    /// Numeric validation
    Numeric,
    /// Range validation for numbers
    Range(f64, f64),
    /// Custom validation function
    Custom(fn(&str) -> bool),
}

impl ValidationRule {
    /// Validate a value against this rule
    pub fn validate(&self, value: &str) -> HttpResult<()> {
        match self {
            ValidationRule::Required => {
                if value.trim().is_empty() {
                    Err(HttpError::ValidationError("Field is required".to_string()))
                } else {
                    Ok(())
                }
            }
            ValidationRule::MinLength(min) => {
                if value.len() < *min {
                    Err(HttpError::ValidationError(
                        format!("Minimum length is {} characters", min)
                    ))
                } else {
                    Ok(())
                }
            }
            ValidationRule::MaxLength(max) => {
                if value.len() > *max {
                    Err(HttpError::ValidationError(
                        format!("Maximum length is {} characters", max)
                    ))
                } else {
                    Ok(())
                }
            }
            ValidationRule::Pattern(pattern) => {
                // Simple pattern matching (in a real implementation, use regex crate)
                if Self::matches_pattern(value, pattern) {
                    Ok(())
                } else {
                    Err(HttpError::ValidationError(
                        format!("Value does not match required pattern")
                    ))
                }
            }
            ValidationRule::Email => {
                if Self::is_valid_email(value) {
                    Ok(())
                } else {
                    Err(HttpError::ValidationError("Invalid email format".to_string()))
                }
            }
            ValidationRule::Url => {
                if Self::is_valid_url(value) {
                    Ok(())
                } else {
                    Err(HttpError::ValidationError("Invalid URL format".to_string()))
                }
            }
            ValidationRule::IpAddress => {
                if Self::is_valid_ip(value) {
                    Ok(())
                } else {
                    Err(HttpError::ValidationError("Invalid IP address format".to_string()))
                }
            }
            ValidationRule::Numeric => {
                if value.parse::<f64>().is_ok() {
                    Ok(())
                } else {
                    Err(HttpError::ValidationError("Value must be numeric".to_string()))
                }
            }
            ValidationRule::Range(min, max) => {
                if let Ok(num) = value.parse::<f64>() {
                    if num >= *min && num <= *max {
                        Ok(())
                    } else {
                        Err(HttpError::ValidationError(
                            format!("Value must be between {} and {}", min, max)
                        ))
                    }
                } else {
                    Err(HttpError::ValidationError("Value must be numeric".to_string()))
                }
            }
            ValidationRule::Custom(validator) => {
                if validator(value) {
                    Ok(())
                } else {
                    Err(HttpError::ValidationError("Custom validation failed".to_string()))
                }
            }
        }
    }

    /// Simple pattern matching (placeholder for regex)
    fn matches_pattern(value: &str, pattern: &str) -> bool {
        // Simplified pattern matching - in production, use regex crate
        match pattern {
            "alphanumeric" => value.chars().all(|c| c.is_alphanumeric()),
            "alpha" => value.chars().all(|c| c.is_alphabetic()),
            "digits" => value.chars().all(|c| c.is_ascii_digit()),
            "phone" => Self::is_valid_phone(value),
            "password" => Self::is_valid_password(value),
            _ => true, // Unknown patterns pass by default
        }
    }

    /// Validate email format (simplified)
    fn is_valid_email(email: &str) -> bool {
        email.contains('@') && 
        email.chars().all(|c| c.is_ascii()) &&
        email.len() >= 5 && 
        email.len() <= 254 &&
        !email.starts_with('@') &&
        !email.ends_with('@') &&
        email.split('@').count() == 2
    }

    /// Validate URL format (simplified)
    fn is_valid_url(url: &str) -> bool {
        url.starts_with("http://") || 
        url.starts_with("https://") ||
        url.starts_with("ftp://") ||
        url.starts_with("//") ||
        url.starts_with("/")
    }

    /// Validate IP address format
    fn is_valid_ip(ip: &str) -> bool {
        IpAddr::from_str(ip).is_ok()
    }

    /// Validate phone number format (simplified)
    fn is_valid_phone(phone: &str) -> bool {
        let cleaned = phone.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        cleaned.len() >= 10 && cleaned.len() <= 15
    }

    /// Validate password strength (simplified)
    fn is_valid_password(password: &str) -> bool {
        password.len() >= 8 &&
        password.chars().any(|c| c.is_ascii_lowercase()) &&
        password.chars().any(|c| c.is_ascii_uppercase()) &&
        password.chars().any(|c| c.is_ascii_digit())
    }
}

/// Validation rules collection for a field
#[derive(Debug, Clone)]
pub struct FieldRules {
    pub field_name: String,
    pub rules: Vec<ValidationRule>,
    pub optional: bool,
}

impl FieldRules {
    /// Create new field rules
    pub fn new<S: Into<String>>(field_name: S) -> Self {
        Self {
            field_name: field_name.into(),
            rules: Vec::new(),
            optional: false,
        }
    }

    /// Make field optional
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Add validation rule
    pub fn rule(mut self, rule: ValidationRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Add required rule
    pub fn required(self) -> Self {
        self.rule(ValidationRule::Required)
    }

    /// Add min length rule
    pub fn min_length(self, min: usize) -> Self {
        self.rule(ValidationRule::MinLength(min))
    }

    /// Add max length rule
    pub fn max_length(self, max: usize) -> Self {
        self.rule(ValidationRule::MaxLength(max))
    }

    /// Add pattern rule
    pub fn pattern<S: Into<String>>(self, pattern: S) -> Self {
        self.rule(ValidationRule::Pattern(pattern.into()))
    }

    /// Add email rule
    pub fn email(self) -> Self {
        self.rule(ValidationRule::Email)
    }

    /// Add URL rule
    pub fn url(self) -> Self {
        self.rule(ValidationRule::Url)
    }

    /// Add IP address rule
    pub fn ip_address(self) -> Self {
        self.rule(ValidationRule::IpAddress)
    }

    /// Add numeric rule
    pub fn numeric(self) -> Self {
        self.rule(ValidationRule::Numeric)
    }

    /// Add range rule
    pub fn range(self, min: f64, max: f64) -> Self {
        self.rule(ValidationRule::Range(min, max))
    }

    /// Add custom rule
    pub fn custom(self, validator: fn(&str) -> bool) -> Self {
        self.rule(ValidationRule::Custom(validator))
    }

    /// Validate field value
    pub fn validate(&self, value: Option<&str>) -> HttpResult<()> {
        match value {
            Some(val) => {
                for rule in &self.rules {
                    rule.validate(val).map_err(|err| {
                        HttpError::ValidationError(
                            format!("Field '{}': {}", self.field_name, err)
                        )
                    })?;
                }
                Ok(())
            }
            None => {
                if self.optional {
                    Ok(())
                } else {
                    Err(HttpError::ValidationError(
                        format!("Field '{}' is required", self.field_name)
                    ))
                }
            }
        }
    }
}

/// Complete validation rules set
#[derive(Debug, Clone)]
pub struct ValidationRules {
    field_rules: HashMap<String, FieldRules>,
    max_request_size: Option<usize>,
    max_header_count: Option<usize>,
    max_header_size: Option<usize>,
    allowed_methods: Option<Vec<String>>,
    allowed_content_types: Option<Vec<String>>,
    rate_limit: Option<RateLimit>,
}

impl ValidationRules {
    /// Create new validation rules
    pub fn new() -> Self {
        Self {
            field_rules: HashMap::new(),
            max_request_size: None,
            max_header_count: None,
            max_header_size: None,
            allowed_methods: None,
            allowed_content_types: None,
            rate_limit: None,
        }
    }

    /// Add field rules
    pub fn field(mut self, rules: FieldRules) -> Self {
        self.field_rules.insert(rules.field_name.clone(), rules);
        self
    }

    /// Set maximum request size
    pub fn max_request_size(mut self, size: usize) -> Self {
        self.max_request_size = Some(size);
        self
    }

    /// Set maximum header count
    pub fn max_header_count(mut self, count: usize) -> Self {
        self.max_header_count = Some(count);
        self
    }

    /// Set maximum header size
    pub fn max_header_size(mut self, size: usize) -> Self {
        self.max_header_size = Some(size);
        self
    }

    /// Set allowed HTTP methods
    pub fn allowed_methods(mut self, methods: Vec<String>) -> Self {
        self.allowed_methods = Some(methods);
        self
    }

    /// Set allowed content types
    pub fn allowed_content_types(mut self, types: Vec<String>) -> Self {
        self.allowed_content_types = Some(types);
        self
    }

    /// Set rate limiting
    pub fn rate_limit(mut self, limit: RateLimit) -> Self {
        self.rate_limit = Some(limit);
        self
    }

    /// Get field rules
    pub fn get_field_rules(&self, field_name: &str) -> Option<&FieldRules> {
        self.field_rules.get(field_name)
    }
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum requests per window
    pub max_requests: usize,
    /// Time window duration
    pub window: Duration,
    /// Rate limiting by IP, user, or other identifier
    pub by: RateLimitBy,
}

impl RateLimit {
    /// Create new rate limit
    pub fn new(max_requests: usize, window: Duration, by: RateLimitBy) -> Self {
        Self {
            max_requests,
            window,
            by,
        }
    }

    /// Create rate limit by IP
    pub fn by_ip(max_requests: usize, window: Duration) -> Self {
        Self::new(max_requests, window, RateLimitBy::Ip)
    }

    /// Create rate limit by user
    pub fn by_user(max_requests: usize, window: Duration) -> Self {
        Self::new(max_requests, window, RateLimitBy::User)
    }
}

/// Rate limiting identifier
#[derive(Debug, Clone)]
pub enum RateLimitBy {
    /// Rate limit by IP address
    Ip,
    /// Rate limit by user ID
    User,
    /// Rate limit by API key
    ApiKey,
    /// Rate limit by custom header
    Header(String),
    /// Rate limit by custom function
    Custom(fn(&Request) -> Option<String>),
}

/// Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Enable HTTPS enforcement
    pub enforce_https: bool,
    /// Enable CSRF protection
    pub csrf_protection: bool,
    /// Enable XSS protection headers
    pub xss_protection: bool,
    /// Enable content type sniffing protection
    pub content_type_nosniff: bool,
    /// Enable frame options protection
    pub frame_options: Option<String>,
    /// Enable HSTS (HTTP Strict Transport Security)
    pub hsts: Option<Duration>,
    /// Maximum file upload size
    pub max_file_size: Option<usize>,
    /// Allowed file types for uploads
    pub allowed_file_types: Option<Vec<String>>,
    /// Enable request logging
    pub request_logging: bool,
}

impl SecurityConfig {
    /// Create default security configuration
    pub fn default() -> Self {
        Self {
            enforce_https: false,
            csrf_protection: true,
            xss_protection: true,
            content_type_nosniff: true,
            frame_options: Some("DENY".to_string()),
            hsts: None,
            max_file_size: Some(10 * 1024 * 1024), // 10MB
            allowed_file_types: None,
            request_logging: true,
        }
    }

    /// Enable strict security (production settings)
    pub fn strict() -> Self {
        Self {
            enforce_https: true,
            csrf_protection: true,
            xss_protection: true,
            content_type_nosniff: true,
            frame_options: Some("DENY".to_string()),
            hsts: Some(Duration::from_secs(31536000)), // 1 year
            max_file_size: Some(5 * 1024 * 1024), // 5MB
            allowed_file_types: Some(vec![
                "image/jpeg".to_string(),
                "image/png".to_string(),
                "image/gif".to_string(),
                "application/pdf".to_string(),
                "text/plain".to_string(),
            ]),
            request_logging: true,
        }
    }

    /// Apply security headers to response
    pub fn apply_security_headers(&self, response: &mut Response) {
        if self.xss_protection {
            response.headers.insert(
                "X-XSS-Protection".to_string(),
                "1; mode=block".to_string()
            );
        }

        if self.content_type_nosniff {
            response.headers.insert(
                "X-Content-Type-Options".to_string(),
                "nosniff".to_string()
            );
        }

        if let Some(frame_options) = &self.frame_options {
            response.headers.insert(
                "X-Frame-Options".to_string(),
                frame_options.clone()
            );
        }

        if let Some(hsts_duration) = self.hsts {
            response.headers.insert(
                "Strict-Transport-Security".to_string(),
                format!("max-age={}", hsts_duration.as_secs())
            );
        }

        // Add other security headers
        response.headers.insert(
            "X-Permitted-Cross-Domain-Policies".to_string(),
            "none".to_string()
        );

        response.headers.insert(
            "Referrer-Policy".to_string(),
            "strict-origin-when-cross-origin".to_string()
        );
    }
}

/// HTTP validator with comprehensive validation and security checks
#[derive(Debug, Clone)]
pub struct HttpValidator {
    validation_rules: ValidationRules,
    security_config: SecurityConfig,
    rate_limiter: Option<RateLimiter>,
}

impl HttpValidator {
    /// Create new HTTP validator
    pub fn new() -> Self {
        Self {
            validation_rules: ValidationRules::new(),
            security_config: SecurityConfig::default(),
            rate_limiter: None,
        }
    }

    /// Set validation rules
    pub fn with_rules(mut self, rules: ValidationRules) -> Self {
        self.validation_rules = rules;
        self
    }

    /// Set security configuration
    pub fn with_security(mut self, config: SecurityConfig) -> Self {
        self.security_config = config;
        self
    }

    /// Set rate limiter
    pub fn with_rate_limiter(mut self, limiter: RateLimiter) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    /// Validate HTTP request
    pub fn validate_request(&self, request: &Request) -> HttpResult<()> {
        // Check rate limiting
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.check_rate_limit(request)?;
        }

        // Check HTTPS enforcement
        if self.security_config.enforce_https && !request.url.is_secure() {
            return Err(HttpError::ValidationError(
                "HTTPS is required".to_string()
            ));
        }

        // Check allowed methods
        if let Some(allowed_methods) = &self.validation_rules.allowed_methods {
            if !allowed_methods.contains(&request.method.to_string()) {
                return Err(HttpError::InvalidMethod(request.method.to_string()));
            }
        }

        // Check request size
        if let Some(max_size) = self.validation_rules.max_request_size {
            if request.content_length() > max_size {
                return Err(HttpError::RequestTooLarge(
                    format!("Request size exceeds {} bytes", max_size)
                ));
            }
        }

        // Check header count and size
        if let Some(max_count) = self.validation_rules.max_header_count {
            if request.headers.len() > max_count {
                return Err(HttpError::ValidationError(
                    format!("Too many headers (max: {})", max_count)
                ));
            }
        }

        if let Some(max_size) = self.validation_rules.max_header_size {
            for (name, value) in request.headers.iter() {
                if name.len() + value.len() > max_size {
                    return Err(HttpError::ValidationError(
                        format!("Header too large (max: {} bytes)", max_size)
                    ));
                }
            }
        }

        // Check content type
        if let Some(allowed_types) = &self.validation_rules.allowed_content_types {
            let content_type = request.get_content_type();
            if !allowed_types.iter().any(|t| content_type.contains(t)) {
                return Err(HttpError::ValidationError(
                    format!("Content type '{}' not allowed", content_type)
                ));
            }
        }

        // Validate headers
        request.headers.validate()?;

        Ok(())
    }

    /// Validate form data against field rules
    pub fn validate_form_data(&self, data: &HashMap<String, String>) -> HttpResult<()> {
        for (field_name, field_rules) in &self.validation_rules.field_rules {
            let value = data.get(field_name).map(|s| s.as_str());
            field_rules.validate(value)?;
        }

        Ok(())
    }

    /// Validate file upload
    pub fn validate_file_upload(&self, filename: &str, content: &[u8], content_type: &str) -> HttpResult<()> {
        // Check file size
        if let Some(max_size) = self.security_config.max_file_size {
            if content.len() > max_size {
                return Err(HttpError::ValidationError(
                    format!("File too large (max: {} bytes)", max_size)
                ));
            }
        }

        // Check file type
        if let Some(allowed_types) = &self.security_config.allowed_file_types {
            if !allowed_types.contains(&content_type.to_string()) {
                return Err(HttpError::ValidationError(
                    format!("File type '{}' not allowed", content_type)
                ));
            }
        }

        // Check filename for security issues
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            return Err(HttpError::ValidationError(
                "Invalid filename".to_string()
            ));
        }

        Ok(())
    }

    /// Apply security headers to response
    pub fn apply_security_headers(&self, response: &mut Response) {
        self.security_config.apply_security_headers(response);
    }
}

impl Default for HttpValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple rate limiter implementation
#[derive(Debug, Clone)]
pub struct RateLimiter {
    config: RateLimit,
    // In a real implementation, this would use a proper storage backend
    requests: HashMap<String, Vec<Instant>>,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(config: RateLimit) -> Self {
        Self {
            config,
            requests: HashMap::new(),
        }
    }

    /// Check rate limit for request
    pub fn check_rate_limit(&self, request: &Request) -> HttpResult<()> {
        let identifier = self.get_identifier(request)?;
        let now = Instant::now();

        // In a real implementation, this would be atomic and persistent
        // For now, this is a simplified version for demonstration
        
        Ok(()) // Always pass for demonstration
    }

    /// Get rate limit identifier for request
    fn get_identifier(&self, request: &Request) -> HttpResult<String> {
        match &self.config.by {
            RateLimitBy::Ip => {
                request.remote_addr.clone()
                    .ok_or_else(|| HttpError::ValidationError("No IP address available".to_string()))
            }
            RateLimitBy::User => {
                // Would extract user ID from authentication
                Ok("anonymous".to_string())
            }
            RateLimitBy::ApiKey => {
                request.header("X-API-Key")
                    .cloned()
                    .ok_or_else(|| HttpError::ValidationError("No API key provided".to_string()))
            }
            RateLimitBy::Header(header_name) => {
                request.header(header_name)
                    .cloned()
                    .ok_or_else(|| HttpError::ValidationError(
                        format!("Header '{}' not found", header_name)
                    ))
            }
            RateLimitBy::Custom(func) => {
                func(request)
                    .ok_or_else(|| HttpError::ValidationError("Custom identifier not available".to_string()))
            }
        }
    }
}

