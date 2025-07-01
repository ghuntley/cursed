//! Comprehensive input validation and sanitization for CURSED

use crate::error::CursedError;
use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

pub type ValidationResult<T> = Result<T, CursedError>;

/// Input validation rules and sanitization
pub struct InputValidator {
    max_string_length: usize,
    max_array_length: usize,
    allowed_chars: Option<Regex>,
    deny_patterns: Vec<Regex>,
}

impl InputValidator {
    pub fn new() -> Self {
        Self {
            max_string_length: 10_000,
            max_array_length: 1_000,
            allowed_chars: None,
            deny_patterns: Vec::new(),
        }
    }

    /// Set maximum string length
    pub fn max_string_length(mut self, length: usize) -> Self {
        self.max_string_length = length;
        self
    }

    /// Set allowed character pattern
    pub fn allowed_chars(mut self, pattern: &str) -> ValidationResult<Self> {
        let regex = Regex::new(pattern)
            .map_err(|e| CursedError::runtime_error(&format!("Invalid regex: {}", e)))?;
        self.allowed_chars = Some(regex);
        Ok(self)
    }

    /// Add denied pattern
    pub fn deny_pattern(mut self, pattern: &str) -> ValidationResult<Self> {
        let regex = Regex::new(pattern)
            .map_err(|e| CursedError::runtime_error(&format!("Invalid regex: {}", e)))?;
        self.deny_patterns.push(regex);
        Ok(self)
    }

    /// Validate and sanitize string input
    pub fn validate_string(&self, input: &str) -> ValidationResult<String> {
        // Length check
        if input.len() > self.max_string_length {
            return Err(CursedError::runtime_error(&format!(
                "String too long: {} > {}", input.len(), self.max_string_length
            )));
        }

        // Check denied patterns
        for pattern in &self.deny_patterns {
            if pattern.is_match(input) {
                return Err(CursedError::runtime_error("Input contains forbidden pattern"));
            }
        }

        // Check allowed characters
        if let Some(ref allowed) = self.allowed_chars {
            if !allowed.is_match(input) {
                return Err(CursedError::runtime_error("Input contains forbidden characters"));
            }
        }

        // Basic sanitization
        let sanitized = self.sanitize_string(input);
        Ok(sanitized)
    }

    /// Sanitize string by removing/escaping dangerous content
    fn sanitize_string(&self, input: &str) -> String {
        input
            .replace('\0', "") // Remove null bytes
            .replace('\x08', "") // Remove backspace
            .replace('\x7F', "") // Remove DEL
            .chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
            .collect()
    }

    /// Validate numeric input with range
    pub fn validate_number<T>(&self, value: T, min: T, max: T) -> ValidationResult<T>
    where
        T: PartialOrd + Copy,
    {
        if value < min || value > max {
            return Err(CursedError::runtime_error("Number out of allowed range"));
        }
        Ok(value)
    }

    /// Validate array length
    pub fn validate_array_length<T>(&self, arr: &[T]) -> ValidationResult<()> {
        if arr.len() > self.max_array_length {
            return Err(CursedError::runtime_error(&format!(
                "Array too long: {} > {}", arr.len(), self.max_array_length
            )));
        }
        Ok(())
    }
}

/// HTML/XML sanitization
pub struct HtmlSanitizer {
    allowed_tags: Vec<String>,
    allowed_attributes: HashMap<String, Vec<String>>,
}

impl HtmlSanitizer {
    pub fn new() -> Self {
        Self {
            allowed_tags: vec![
                "p".to_string(), "br".to_string(), "strong".to_string(), 
                "em".to_string(), "u".to_string(), "ol".to_string(), 
                "ul".to_string(), "li".to_string()
            ],
            allowed_attributes: HashMap::new(),
        }
    }

    /// Set allowed HTML tags
    pub fn allowed_tags(mut self, tags: Vec<String>) -> Self {
        self.allowed_tags = tags;
        self
    }

    /// Sanitize HTML content
    pub fn sanitize_html(&self, input: &str) -> ValidationResult<String> {
        // Basic HTML entity encoding
        let sanitized = input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('/', "&#x2F;");

        // Additional XSS prevention
        let xss_patterns = get_xss_patterns();
        for pattern in xss_patterns {
            if pattern.is_match(&sanitized.to_lowercase()) {
                return Err(CursedError::runtime_error("Potential XSS attack detected"));
            }
        }

        Ok(sanitized)
    }

    /// Sanitize for URL usage
    pub fn sanitize_url(&self, input: &str) -> ValidationResult<String> {
        // URL encode special characters
        let encoded = urlencoding::encode(input);
        
        // Validate URL scheme
        if input.starts_with("javascript:") || input.starts_with("data:") || input.starts_with("vbscript:") {
            return Err(CursedError::runtime_error("Dangerous URL scheme"));
        }

        Ok(encoded.to_string())
    }
}

/// SQL injection prevention patterns
pub struct SqlSanitizer;

impl SqlSanitizer {
    /// Escape SQL string literals
    pub fn escape_sql_string(input: &str) -> String {
        input.replace('\'', "''").replace('\0', "")
    }

    /// Validate SQL identifier (table/column name)
    pub fn validate_sql_identifier(name: &str) -> ValidationResult<String> {
        if name.is_empty() || name.len() > 64 {
            return Err(CursedError::runtime_error("Invalid identifier length"));
        }

        // Must start with letter or underscore
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return Err(CursedError::runtime_error("Identifier must start with letter or underscore"));
        }

        // Must contain only safe characters
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(CursedError::runtime_error("Identifier contains forbidden characters"));
        }

        // Check for SQL keywords
        if is_sql_keyword(name) {
            return Err(CursedError::runtime_error("Identifier is a reserved SQL keyword"));
        }

        Ok(name.to_string())
    }

    /// Check for SQL injection patterns
    pub fn check_sql_injection(input: &str) -> ValidationResult<()> {
        let sql_patterns = get_sql_injection_patterns();
        let input_lower = input.to_lowercase();

        for pattern in sql_patterns {
            if pattern.is_match(&input_lower) {
                return Err(CursedError::runtime_error("Potential SQL injection detected"));
            }
        }

        Ok(())
    }
}

/// Path traversal prevention
pub struct PathSanitizer;

impl PathSanitizer {
    /// Sanitize file path to prevent directory traversal
    pub fn sanitize_path(path: &str) -> ValidationResult<String> {
        // Normalize path separators
        let normalized = path.replace('\\', "/");

        // Check for path traversal attempts
        if normalized.contains("..") || normalized.contains("~") {
            return Err(CursedError::runtime_error("Path traversal attempt detected"));
        }

        // Remove leading slashes (force relative paths)
        let relative = normalized.trim_start_matches('/');

        // Validate path components
        for component in relative.split('/') {
            if component.is_empty() || component == "." || component == ".." {
                return Err(CursedError::runtime_error("Invalid path component"));
            }

            // Check for special characters
            if component.chars().any(|c| matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\0')) {
                return Err(CursedError::runtime_error("Invalid characters in path"));
            }
        }

        Ok(relative.to_string())
    }

    /// Validate file extension
    pub fn validate_file_extension(filename: &str, allowed_extensions: &[&str]) -> ValidationResult<()> {
        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        if !allowed_extensions.contains(&extension) {
            return Err(CursedError::runtime_error(&format!(
                "File extension '{}' not allowed", extension
            )));
        }

        Ok(())
    }
}

/// Email validation
pub struct EmailValidator;

impl EmailValidator {
    /// Validate email address format
    pub fn validate_email(email: &str) -> ValidationResult<String> {
        let email_regex = get_email_regex();
        
        if !email_regex.is_match(email) {
            return Err(CursedError::runtime_error("Invalid email format"));
        }

        if email.len() > 254 {
            return Err(CursedError::runtime_error("Email address too long"));
        }

        // Basic sanitization
        let sanitized = email.trim().to_lowercase();
        Ok(sanitized)
    }
}

/// Command injection prevention
pub struct CommandSanitizer;

impl CommandSanitizer {
    /// Validate command line arguments
    pub fn validate_command_arg(arg: &str) -> ValidationResult<String> {
        // Check for command injection patterns
        let dangerous_chars = ['&', '|', ';', '$', '`', '(', ')', '{', '}', '[', ']', '<', '>', '\n', '\r'];
        
        if arg.chars().any(|c| dangerous_chars.contains(&c)) {
            return Err(CursedError::runtime_error("Dangerous characters in command argument"));
        }

        // Check for command substitution
        if arg.contains("$(") || arg.contains("`") {
            return Err(CursedError::runtime_error("Command substitution detected"));
        }

        Ok(arg.to_string())
    }
}

// Regex pattern getters with lazy initialization
fn get_xss_patterns() -> &'static [Regex] {
    static XSS_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    XSS_PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>").unwrap(),
            Regex::new(r"javascript:").unwrap(),
            Regex::new(r"on\w+\s*=").unwrap(),
            Regex::new(r"<iframe\b").unwrap(),
            Regex::new(r"<object\b").unwrap(),
            Regex::new(r"<embed\b").unwrap(),
            Regex::new(r"<link\b").unwrap(),
            Regex::new(r"<meta\b").unwrap(),
        ]
    })
}

fn get_sql_injection_patterns() -> &'static [Regex] {
    static SQL_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    SQL_PATTERNS.get_or_init(|| {
        vec![
            Regex::new(r"union\s+select").unwrap(),
            Regex::new(r";\s*drop\s+table").unwrap(),
            Regex::new(r";\s*delete\s+from").unwrap(),
            Regex::new(r";\s*truncate").unwrap(),
            Regex::new(r"'\s*or\s*'1'\s*=\s*'1").unwrap(),
            Regex::new(r"\"\s*or\s*\"1\"\s*=\s*\"1").unwrap(),
            Regex::new(r"'\s*or\s*1\s*=\s*1").unwrap(),
            Regex::new(r"--").unwrap(),
            Regex::new(r"/\*.*\*/").unwrap(),
            Regex::new(r"xp_cmdshell").unwrap(),
            Regex::new(r"sp_executesql").unwrap(),
        ]
    })
}

fn get_email_regex() -> &'static Regex {
    static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    })
}

fn is_sql_keyword(word: &str) -> bool {
    const SQL_KEYWORDS: &[&str] = &[
        "SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "CREATE", "ALTER", "INDEX", "TABLE",
        "DATABASE", "TRIGGER", "VIEW", "PROCEDURE", "FUNCTION", "GRANT", "REVOKE", "COMMIT",
        "ROLLBACK", "TRANSACTION", "UNION", "WHERE", "ORDER", "GROUP", "HAVING", "FROM",
        "INTO", "VALUES", "SET", "JOIN", "INNER", "LEFT", "RIGHT", "OUTER", "ON", "AS",
    ];
    
    SQL_KEYWORDS.contains(&word.to_uppercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_validation() {
        let validator = InputValidator::new()
            .max_string_length(100)
            .allowed_chars(r"^[a-zA-Z0-9\s]+$").unwrap();

        assert!(validator.validate_string("Hello World 123").is_ok());
        assert!(validator.validate_string("Hello<script>").is_err());
        assert!(validator.validate_string(&"x".repeat(200)).is_err());
    }

    #[test]
    fn test_html_sanitization() {
        let sanitizer = HtmlSanitizer::new();
        
        let result = sanitizer.sanitize_html("<script>alert('xss')</script>").unwrap();
        assert!(!result.contains("<script>"));
        
        let result = sanitizer.sanitize_html("Hello & goodbye").unwrap();
        assert!(result.contains("&amp;"));
    }

    #[test]
    fn test_sql_injection_detection() {
        assert!(SqlSanitizer::check_sql_injection("normal input").is_ok());
        assert!(SqlSanitizer::check_sql_injection("'; DROP TABLE users; --").is_err());
        assert!(SqlSanitizer::check_sql_injection("1' OR '1'='1").is_err());
    }

    #[test]
    fn test_path_sanitization() {
        assert!(PathSanitizer::sanitize_path("documents/file.txt").is_ok());
        assert!(PathSanitizer::sanitize_path("../../../etc/passwd").is_err());
        assert!(PathSanitizer::sanitize_path("docs/file<script>.txt").is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(EmailValidator::validate_email("user@example.com").is_ok());
        assert!(EmailValidator::validate_email("invalid-email").is_err());
        assert!(EmailValidator::validate_email("user@").is_err());
    }

    #[test]
    fn test_command_injection_prevention() {
        assert!(CommandSanitizer::validate_command_arg("normal_arg").is_ok());
        assert!(CommandSanitizer::validate_command_arg("arg; rm -rf /").is_err());
        assert!(CommandSanitizer::validate_command_arg("arg && evil_command").is_err());
        assert!(CommandSanitizer::validate_command_arg("$(evil_command)").is_err());
    }
}
