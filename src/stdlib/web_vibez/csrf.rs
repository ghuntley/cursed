/// CSRF token generation and validation utilities
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// CSRF token structure
#[derive(Debug, Clone)]
pub struct CsrfToken {
    pub token: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub session_id: Option<String>,
}

impl CsrfToken {
    /// Create new CSRF token
    pub fn new(secret: &str, session_id: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let expires_at = now + 3600; // 1 hour expiry
        let token_data = format!("{}:{}:{}", 
            session_id.as_deref().unwrap_or(""), 
            now, 
            secret
        );
        
        let token = Self::hash_token(&token_data);

        Self {
            token,
            created_at: now,
            expires_at,
            session_id,
        }
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now > self.expires_at
    }

    /// Get token as string
    pub fn as_string(&self) -> &str {
        &self.token
    }

    /// Create token from string (for validation)
    pub fn from_string(token: &str, secret: &str, session_id: Option<String>) -> Option<Self> {
        // Extract timestamp from token if possible
        // For now, create a token for validation purposes
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Some(Self {
            token: token.to_string(),
            created_at: now,
            expires_at: now + 3600,
            session_id,
        })
    }

    /// Simple hash function for token generation
    fn hash_token(input: &str) -> String {
        // Simple hash implementation (in production, use proper crypto)
        let mut hash: u64 = 5381;
        for byte in input.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
    }

    /// Generate secure random-like token
    pub fn generate_secure_token() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        // Mix current time with some pseudo-random elements
        let random_data = format!("{}:{}", now, now.wrapping_mul(9973));
        Self::hash_token(&random_data)
    }
}

/// CSRF validator for managing and validating tokens
pub struct CsrfValidator {
    secret: String,
    tokens: HashMap<String, CsrfToken>,
    max_tokens: usize,
    token_lifetime: u64,
}

impl CsrfValidator {
    /// Create new CSRF validator
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            tokens: HashMap::new(),
            max_tokens: 1000,
            token_lifetime: 3600, // 1 hour
        }
    }

    /// Set maximum number of stored tokens
    pub fn with_max_tokens(mut self, max: usize) -> Self {
        self.max_tokens = max;
        self
    }

    /// Set token lifetime in seconds
    pub fn with_lifetime(mut self, lifetime: u64) -> Self {
        self.token_lifetime = lifetime;
        self
    }

    /// Generate new CSRF token
    pub fn generate_token(&mut self, session_id: Option<String>) -> CsrfToken {
        self.cleanup_expired_tokens();

        let token = CsrfToken::new(&self.secret, session_id.clone());
        
        // Store token for validation
        let key = self.create_token_key(&token.token, &session_id);
        self.tokens.insert(key, token.clone());

        // Cleanup if we have too many tokens
        if self.tokens.len() > self.max_tokens {
            self.cleanup_oldest_tokens();
        }

        token
    }

    /// Validate CSRF token
    pub fn validate_token(&mut self, token: &str, session_id: Option<String>) -> bool {
        self.cleanup_expired_tokens();

        let key = self.create_token_key(token, &session_id);
        
        if let Some(stored_token) = self.tokens.get(&key) {
            if stored_token.is_expired() {
                self.tokens.remove(&key);
                return false;
            }

            // Validate token matches
            if stored_token.token == token {
                // One-time use: remove after validation
                self.tokens.remove(&key);
                return true;
            }
        }

        false
    }

    /// Check if token exists (without consuming it)
    pub fn check_token(&self, token: &str, session_id: Option<String>) -> bool {
        let key = self.create_token_key(token, &session_id);
        
        if let Some(stored_token) = self.tokens.get(&key) {
            !stored_token.is_expired() && stored_token.token == token
        } else {
            false
        }
    }

    /// Generate token for HTML form
    pub fn html_token_input(&mut self, session_id: Option<String>) -> String {
        let token = self.generate_token(session_id);
        format!(r#"<input type="hidden" name="_csrf_token" value="{}" />"#, token.as_string())
    }

    /// Generate token for AJAX requests
    pub fn meta_token_tag(&mut self, session_id: Option<String>) -> String {
        let token = self.generate_token(session_id);
        format!(r#"<meta name="csrf-token" content="{}" />"#, token.as_string())
    }

    /// Extract token from request headers
    pub fn extract_token_from_headers(&self, headers: &HashMap<String, String>) -> Option<String> {
        // Check various header names
        if let Some(token) = headers.get("X-CSRF-Token") {
            return Some(token.clone());
        }
        if let Some(token) = headers.get("X-XSRF-Token") {
            return Some(token.clone());
        }
        if let Some(token) = headers.get("X-Requested-With") {
            if token == "XMLHttpRequest" {
                // Check for token in other headers
                return headers.get("X-CSRFToken").cloned();
            }
        }
        None
    }

    /// Extract token from form data
    pub fn extract_token_from_form(&self, form_data: &HashMap<String, String>) -> Option<String> {
        form_data.get("_csrf_token").cloned()
            .or_else(|| form_data.get("csrf_token").cloned())
            .or_else(|| form_data.get("_token").cloned())
    }

    /// Validate request with CSRF protection
    pub fn validate_request(
        &mut self,
        headers: &HashMap<String, String>,
        form_data: &HashMap<String, String>,
        session_id: Option<String>,
    ) -> bool {
        // Extract token from headers or form data
        let token = self.extract_token_from_headers(headers)
            .or_else(|| self.extract_token_from_form(form_data));

        if let Some(token) = token {
            self.validate_token(&token, session_id)
        } else {
            false
        }
    }

    /// Check if request method requires CSRF protection
    pub fn requires_csrf_protection(method: &str) -> bool {
        matches!(method.to_uppercase().as_str(), "POST" | "PUT" | "DELETE" | "PATCH")
    }

    /// Get CSRF error response
    pub fn csrf_error_response() -> CsrfErrorResponse {
        CsrfErrorResponse {
            status: 403,
            message: "CSRF token validation failed".to_string(),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
            ],
        }
    }

    /// Cleanup expired tokens
    fn cleanup_expired_tokens(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.tokens.retain(|_, token| now <= token.expires_at);
    }

    /// Cleanup oldest tokens when at capacity
    fn cleanup_oldest_tokens(&mut self) {
        let target_size = self.max_tokens * 3 / 4; // Remove 25% of tokens
        
        let mut tokens_vec: Vec<_> = self.tokens.iter().collect();
        tokens_vec.sort_by_key(|(_, token)| token.created_at);

        let tokens_to_remove = tokens_vec.len().saturating_sub(target_size);
        for (key, _) in tokens_vec.into_iter().take(tokens_to_remove) {
            self.tokens.remove(key);
        }
    }

    /// Create unique key for token storage
    fn create_token_key(&self, token: &str, session_id: &Option<String>) -> String {
        match session_id {
            Some(id) => format!("{}:{}", id, token),
            None => token.to_string(),
        }
    }

    /// Get statistics about stored tokens
    pub fn get_stats(&self) -> CsrfStats {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let (total, expired) = self.tokens.values().fold((0, 0), |(total, expired), token| {
            if now > token.expires_at {
                (total + 1, expired + 1)
            } else {
                (total + 1, expired)
            }
        });

        CsrfStats {
            total_tokens: total,
            expired_tokens: expired,
            active_tokens: total - expired,
            max_tokens: self.max_tokens,
            token_lifetime: self.token_lifetime,
        }
    }
}

/// CSRF error response
pub struct CsrfErrorResponse {
    pub status: u16,
    pub message: String,
    pub headers: Vec<(String, String)>,
}

impl CsrfErrorResponse {
    /// Convert to JSON response body
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"error":"{}","status":{},"code":"CSRF_VALIDATION_FAILED"}}"#,
            self.message, self.status
        )
    }

    /// Convert to HTML response body
    pub fn to_html(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>CSRF Validation Failed</title>
</head>
<body>
    <h1>Access Denied</h1>
    <p>{}</p>
    <p>Please refresh the page and try again.</p>
</body>
</html>"#,
            self.message
        )
    }
}

/// CSRF statistics
#[derive(Debug)]
pub struct CsrfStats {
    pub total_tokens: usize,
    pub expired_tokens: usize,
    pub active_tokens: usize,
    pub max_tokens: usize,
    pub token_lifetime: u64,
}

/// CSRF middleware configuration
#[derive(Debug, Clone)]
pub struct CsrfConfig {
    pub enabled: bool,
    pub secret: String,
    pub token_lifetime: u64,
    pub max_tokens: usize,
    pub require_session: bool,
    pub safe_methods: Vec<String>,
    pub excluded_paths: Vec<String>,
}

impl Default for CsrfConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            secret: "changeme".to_string(),
            token_lifetime: 3600,
            max_tokens: 1000,
            require_session: true,
            safe_methods: vec![
                "GET".to_string(),
                "HEAD".to_string(),
                "OPTIONS".to_string(),
                "TRACE".to_string(),
            ],
            excluded_paths: vec![
                "/api/health".to_string(),
                "/metrics".to_string(),
            ],
        }
    }
}

impl CsrfConfig {
    /// Check if path is excluded from CSRF protection
    pub fn is_path_excluded(&self, path: &str) -> bool {
        self.excluded_paths.iter().any(|excluded| {
            path.starts_with(excluded) || path == excluded
        })
    }

    /// Check if method is safe (doesn't require CSRF protection)
    pub fn is_method_safe(&self, method: &str) -> bool {
        self.safe_methods.iter().any(|safe_method| {
            safe_method.eq_ignore_ascii_case(method)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csrf_token_creation() {
        let token = CsrfToken::new("secret", Some("session123".to_string()));
        assert!(!token.token.is_empty());
        assert_eq!(token.session_id, Some("session123".to_string()));
        assert!(!token.is_expired());
    }

    #[test]
    fn test_csrf_token_expiry() {
        let mut token = CsrfToken::new("secret", None);
        token.expires_at = 0; // Set to already expired
        assert!(token.is_expired());
    }

    #[test]
    fn test_csrf_validator() {
        let mut validator = CsrfValidator::new("secret".to_string());
        
        // Generate token
        let token = validator.generate_token(Some("session123".to_string()));
        
        // Validate token
        assert!(validator.validate_token(&token.token, Some("session123".to_string())));
        
        // Token should be consumed after validation
        assert!(!validator.validate_token(&token.token, Some("session123".to_string())));
    }

    #[test]
    fn test_csrf_token_extraction() {
        let validator = CsrfValidator::new("secret".to_string());
        
        // Test header extraction
        let mut headers = HashMap::new();
        headers.insert("X-CSRF-Token".to_string(), "token123".to_string());
        assert_eq!(validator.extract_token_from_headers(&headers), Some("token123".to_string()));
        
        // Test form extraction
        let mut form_data = HashMap::new();
        form_data.insert("_csrf_token".to_string(), "token456".to_string());
        assert_eq!(validator.extract_token_from_form(&form_data), Some("token456".to_string()));
    }

    #[test]
    fn test_csrf_html_generation() {
        let mut validator = CsrfValidator::new("secret".to_string());
        
        let html_input = validator.html_token_input(None);
        assert!(html_input.contains("_csrf_token"));
        assert!(html_input.contains("hidden"));
        
        let meta_tag = validator.meta_token_tag(None);
        assert!(meta_tag.contains("csrf-token"));
        assert!(meta_tag.contains("meta"));
    }

    #[test]
    fn test_csrf_config() {
        let config = CsrfConfig::default();
        
        assert!(config.is_method_safe("GET"));
        assert!(config.is_method_safe("HEAD"));
        assert!(!config.is_method_safe("POST"));
        
        assert!(config.is_path_excluded("/api/health"));
        assert!(!config.is_path_excluded("/api/users"));
    }

    #[test]
    fn test_csrf_cleanup() {
        let mut validator = CsrfValidator::new("secret".to_string());
        
        // Generate many tokens to trigger cleanup
        for i in 0..1100 {
            validator.generate_token(Some(format!("session{}", i)));
        }
        
        let stats = validator.get_stats();
        assert!(stats.total_tokens <= validator.max_tokens);
    }
}
