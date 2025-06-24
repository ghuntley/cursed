use crate::error::Error;
/// Template Security Validator - Advanced security features for CURSED templates
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use regex::Regex;
use tracing::{debug, error, info, instrument, warn};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;
use super::template_core::{TemplateContext, TemplateConfig};
use super::template_syntax::{TemplateAst, TemplateNode, TemplateExpression};

/// Template security validation levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityValidationLevel {
    /// Minimal security checks
    Basic,
    /// Standard security validation
    Standard,
    /// Strict security with comprehensive checks
    Strict,
    /// Maximum security for production environments
    Maximum,
}

/// Security policy configuration
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    /// Validation level
    pub validation_level: SecurityValidationLevel,
    /// Allowed template directories
    pub allowed_directories: HashSet<PathBuf>,
    /// Blocked template directories
    pub blocked_directories: HashSet<PathBuf>,
    /// Allowed file extensions
    pub allowed_extensions: HashSet<String>,
    /// Maximum template nesting depth
    pub max_nesting_depth: usize,
    /// Maximum template size in bytes
    pub max_template_size: usize,
    /// Enable path traversal protection
    pub enable_path_traversal_protection: bool,
    /// Enable content injection protection
    pub enable_content_injection_protection: bool,
    /// Enable XSS protection
    pub enable_xss_protection: bool,
    /// Enable CSRF protection
    pub enable_csrf_protection: bool,
    /// Content Security Policy settings
    pub csp_directives: HashMap<String, Vec<String>>,
    /// Trusted domains for external includes
    pub trusted_domains: HashSet<String>,
    /// Maximum render time (for DoS protection)
    pub max_render_time_ms: u64,
    /// Maximum memory usage (for DoS protection)
    pub max_memory_usage_mb: usize,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        let mut csp_directives = HashMap::new();
        csp_directives.insert("default-src".to_string(), vec!["'self'".to_string()]);
        csp_directives.insert("script-src".to_string(), vec!["'self'".to_string(), "'unsafe-inline'".to_string()]);
        csp_directives.insert("style-src".to_string(), vec!["'self'".to_string(), "'unsafe-inline'".to_string()]);
        csp_directives.insert("img-src".to_string(), vec!["'self'".to_string(), "data:".to_string()]);
        
        Self {
            validation_level: SecurityValidationLevel::Standard,
            allowed_directories: HashSet::new(),
            blocked_directories: HashSet::new(),
            allowed_extensions: ["html", "txt", "md", "xml", "json"].iter().map(|s| s.to_string()).collect(),
            max_nesting_depth: 20,
            max_template_size: 1024 * 1024, // 1MB
            enable_path_traversal_protection: true,
            enable_content_injection_protection: true,
            enable_xss_protection: true,
            enable_csrf_protection: true,
            csp_directives,
            trusted_domains: HashSet::new(),
            max_render_time_ms: 5000, // 5 seconds
            max_memory_usage_mb: 100, // 100MB
        }
    }
}

/// Security validation result
#[derive(Debug, Clone)]
pub struct SecurityValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// Security issues found
    pub issues: Vec<SecurityIssue>,
    /// Security warnings
    pub warnings: Vec<String>,
    /// Security recommendations
    pub recommendations: Vec<String>,
    /// Validation time
    pub validation_time_ms: u64,
}

/// Security issue types
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    /// Issue severity
    pub severity: SecuritySeverity,
    /// Issue type
    pub issue_type: SecurityIssueType,
    /// Issue description
    pub description: String,
    /// Template location (if applicable)
    pub location: Option<String>,
    /// Suggested fix
    pub suggested_fix: Option<String>,
}

/// Security issue severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Types of security issues
#[derive(Debug, Clone)]
pub enum SecurityIssueType {
    PathTraversal,
    ContentInjection,
    XssVulnerability,
    CsrfVulnerability,
    UnsafeInclusion,
    UnescapedOutput,
    ExcessiveNesting,
    LargeTemplate,
    UntrustedDomain,
    InvalidDirectory,
    SuspiciousPattern,
    PrivilegeEscalation,
}

/// Template security validator
pub struct TemplateSecurityValidator {
    /// Security policy
    policy: SecurityPolicy,
    /// XSS pattern detection
    xss_patterns: Vec<Regex>,
    /// Path traversal patterns
    path_traversal_patterns: Vec<Regex>,
    /// Content injection patterns
    injection_patterns: Vec<Regex>,
    /// Performance monitoring
    validation_count: Arc<std::sync::Mutex<u64>>,
    /// Security metrics
    security_metrics: Arc<std::sync::Mutex<SecurityMetrics>>,
}

/// Security metrics tracking
#[derive(Debug, Clone)]
pub struct SecurityMetrics {
    /// Total validations performed
    pub total_validations: u64,
    /// Security issues found by type
    pub issues_by_type: HashMap<String, u64>,
    /// Average validation time
    pub average_validation_time_ms: f64,
    /// Blocked attempts
    pub blocked_attempts: u64,
    /// False positives reported
    pub false_positives: u64,
}

impl TemplateSecurityValidator {
    /// Create a new security validator with default policy
    pub fn new() -> Self {
        Self::with_policy(SecurityPolicy::default())
    }
    
    /// Create a security validator with custom policy
    pub fn with_policy(policy: SecurityPolicy) -> Self {
        let xss_patterns = vec![
            Regex::new(r"<script[^>]*>").unwrap(),
            Regex::new(r"javascript:").unwrap(),
            Regex::new(r"onload\s*=").unwrap(),
            Regex::new(r"onerror\s*=").unwrap(),
            Regex::new(r"onclick\s*=").unwrap(),
            Regex::new(r"data:text/html").unwrap(),
            Regex::new(r"vbscript:").unwrap(),
        ];
        
        let path_traversal_patterns = vec![
            Regex::new(r"\.\./").unwrap(),
            Regex::new(r"\.\.\\").unwrap(),
            Regex::new(r"/etc/passwd").unwrap(),
            Regex::new(r"\\windows\\system32").unwrap(),
            Regex::new(r"%2e%2e%2f").unwrap(), // URL encoded ../
        ];
        
        let injection_patterns = vec![
            Regex::new(r"<\?php").unwrap(),
            Regex::new(r"<%").unwrap(),
            Regex::new(r"\{\{.*exec.*\}\}").unwrap(),
            Regex::new(r"\{\{.*system.*\}\}").unwrap(),
            Regex::new(r"eval\s*\(").unwrap(),
        ];
        
        Self {
            policy,
            xss_patterns,
            path_traversal_patterns,
            injection_patterns,
            validation_count: Arc::new(std::sync::Mutex::new(0)),
            security_metrics: Arc::new(std::sync::Mutex::new(SecurityMetrics {
                total_validations: 0,
                issues_by_type: HashMap::new(),
                average_validation_time_ms: 0.0,
                blocked_attempts: 0,
                false_positives: 0,
            })),
        }
    }
    
    /// Validate a template for security issues
    #[instrument(skip(self, template_ast, template_path))]
    pub fn validate_template(
        &self,
        template_ast: &TemplateAst,
        template_path: Option<&Path>,
        template_source: &str,
    ) -> Result<(), Error> {
        let start_time = std::time::Instant::now();
        info!("Starting security validation");
        
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        // Update validation count
        if let Ok(mut count) = self.validation_count.lock() {
            *count += 1;
        }
        
        // Validate template path
        if let Some(path) = template_path {
            self.validate_template_path(path, &mut issues, &mut warnings)?;
        }
        
        // Validate template size
        self.validate_template_size(template_source, &mut issues, &mut warnings)?;
        
        // Validate template content
        self.validate_template_content(template_source, &mut issues, &mut warnings)?;
        
        // Validate template AST
        self.validate_template_ast(template_ast, &mut issues, &mut warnings, &mut recommendations)?;
        
        // Generate security recommendations
        self.generate_security_recommendations(&issues, &mut recommendations);
        
        let validation_time = start_time.elapsed().as_millis() as u64;
        let is_valid = !issues.iter().any(|i| matches!(i.severity, SecuritySeverity::High | SecuritySeverity::Critical));
        
        // Update metrics
        self.update_security_metrics(&issues, validation_time);
        
        info!(
            is_valid = is_valid,
            issues_count = issues.len(),
            warnings_count = warnings.len(),
            validation_time_ms = validation_time,
            "Security validation completed"
        );
        
        Ok(SecurityValidationResult {
            is_valid,
            issues,
            warnings,
            recommendations,
            validation_time_ms: validation_time,
        })
    }
    
    /// Validate template file path
    fn validate_template_path(
        &self,
        path: &Path,
        issues: &mut Vec<SecurityIssue>,
        warnings: &mut Vec<String>,
    ) -> Result<(), Error> {
        // Check path traversal
        if self.policy.enable_path_traversal_protection {
            let path_str = path.to_string_lossy();
            for pattern in &self.path_traversal_patterns {
                if pattern.is_match(&path_str) {
                    issues.push(SecurityIssue {
                        severity: SecuritySeverity::High,
                        issue_type: SecurityIssueType::PathTraversal,
                        description: format!("Path traversal attempt detected: {}", path_str),
                        location: Some(path_str.to_string()),
                        suggested_fix: Some("Use absolute paths within allowed directories".to_string()),
                    });
                }
            }
        }
        
        // Check allowed directories
        if !self.policy.allowed_directories.is_empty() {
            let is_allowed = self.policy.allowed_directories.iter()
                .any(|allowed_dir| path.starts_with(allowed_dir));
            
            if !is_allowed {
                issues.push(SecurityIssue {
                    severity: SecuritySeverity::Medium,
                    issue_type: SecurityIssueType::InvalidDirectory,
                    description: format!("Template path not in allowed directories: {}", path.display()),
                    location: Some(path.to_string_lossy().to_string()),
                    suggested_fix: Some("Move template to allowed directory".to_string()),
                });
            }
        }
        
        // Check blocked directories
        for blocked_dir in &self.policy.blocked_directories {
            if path.starts_with(blocked_dir) {
                issues.push(SecurityIssue {
                    severity: SecuritySeverity::High,
                    issue_type: SecurityIssueType::InvalidDirectory,
                    description: format!("Template path in blocked directory: {}", path.display()),
                    location: Some(path.to_string_lossy().to_string()),
                    suggested_fix: Some("Move template away from blocked directory".to_string()),
                });
            }
        }
        
        // Check file extension
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if !self.policy.allowed_extensions.contains(extension) {
                warnings.push(format!("Template uses non-standard extension: {}", extension));
            }
        }
        
        Ok(())
    }
    
    /// Validate template size
    fn validate_template_size(
        &self,
        template_source: &str,
        issues: &mut Vec<SecurityIssue>,
        warnings: &mut Vec<String>,
    ) -> Result<(), Error> {
        let size = template_source.len();
        
        if size > self.policy.max_template_size {
            issues.push(SecurityIssue {
                severity: SecuritySeverity::Medium,
                issue_type: SecurityIssueType::LargeTemplate,
                description: format!("Template size ({} bytes) exceeds maximum allowed ({})", size, self.policy.max_template_size),
                location: None,
                suggested_fix: Some("Reduce template size or split into smaller templates".to_string()),
            });
        }
        
        if size > self.policy.max_template_size / 2 {
            warnings.push(format!("Template size ({} bytes) is approaching maximum limit", size));
        }
        
        Ok(())
    }
    
    /// Validate template content for security issues
    fn validate_template_content(
        &self,
        template_source: &str,
        issues: &mut Vec<SecurityIssue>,
        warnings: &mut Vec<String>,
    ) -> Result<(), Error> {
        // Check for XSS patterns
        if self.policy.enable_xss_protection {
            for pattern in &self.xss_patterns {
                if pattern.is_match(template_source) {
                    issues.push(SecurityIssue {
                        severity: SecuritySeverity::High,
                        issue_type: SecurityIssueType::XssVulnerability,
                        description: "Potential XSS vulnerability detected in template content".to_string(),
                        location: None,
                        suggested_fix: Some("Use proper escaping filters for user content".to_string()),
                    });
                }
            }
        }
        
        // Check for content injection patterns
        if self.policy.enable_content_injection_protection {
            for pattern in &self.injection_patterns {
                if pattern.is_match(template_source) {
                    issues.push(SecurityIssue {
                        severity: SecuritySeverity::High,
                        issue_type: SecurityIssueType::ContentInjection,
                        description: "Potential content injection vulnerability detected".to_string(),
                        location: None,
                        suggested_fix: Some("Remove executable code from templates".to_string()),
                    });
                }
            }
        }
        
        // Check for unescaped output patterns
        let unescaped_pattern = Regex::new(r"\{\{\{\s*[^}]+\s*\}\}\}").unwrap();
        if unescaped_pattern.is_match(template_source) {
            warnings.push("Unescaped output detected - ensure content is trusted".to_string());
        }
        
        Ok(())
    }
    
    /// Validate template AST for security issues
    fn validate_template_ast(
        &self,
        ast: &TemplateAst,
        issues: &mut Vec<SecurityIssue>,
        warnings: &mut Vec<String>,
        recommendations: &mut Vec<String>,
    ) -> Result<(), Error> {
        let mut nesting_depth = 0;
        self.validate_ast_nodes(&ast.nodes, &mut nesting_depth, issues, warnings, recommendations)?;
        Ok(())
    }
    
    /// Recursively validate AST nodes
    fn validate_ast_nodes(
        &self,
        nodes: &[TemplateNode],
        nesting_depth: &mut usize,
        issues: &mut Vec<SecurityIssue>,
        warnings: &mut Vec<String>,
        recommendations: &mut Vec<String>,
    ) -> Result<(), Error> {
        *nesting_depth += 1;
        
        // Check nesting depth
        if *nesting_depth > self.policy.max_nesting_depth {
            issues.push(SecurityIssue {
                severity: SecuritySeverity::Medium,
                issue_type: SecurityIssueType::ExcessiveNesting,
                description: format!("Template nesting depth ({}) exceeds maximum ({})", nesting_depth, self.policy.max_nesting_depth),
                location: None,
                suggested_fix: Some("Reduce template nesting or split into smaller templates".to_string()),
            });
        }
        
        for node in nodes {
            self.validate_ast_node(node, nesting_depth, issues, warnings, recommendations)?;
        }
        
        *nesting_depth -= 1;
        Ok(())
    }
    
    /// Validate individual AST node
    fn validate_ast_node(
        &self,
        node: &TemplateNode,
        nesting_depth: &mut usize,
        issues: &mut Vec<SecurityIssue>,
        warnings: &mut Vec<String>,
        recommendations: &mut Vec<String>,
    ) -> Result<(), Error> {
        match node {
            TemplateNode::Text(_) => {
                // Text nodes are generally safe
            }
            TemplateNode::Variable { name, filters } => {
                // Check for suspicious variable names
                if name.contains("password") || name.contains("secret") || name.contains("token") {
                    warnings.push(format!("Potentially sensitive variable in template: {}", name));
                }
                
                // Check if output is properly escaped
                if filters.is_empty() {
                    recommendations.push(format!("Consider adding escaping filter to variable: {}", name));
                }
            }
            TemplateNode::Block { block_type, content, .. } => {
                match block_type.as_str() {
                    "include" => {
                        warnings.push("Template includes can introduce security risks - ensure included templates are trusted".to_string());
                    }
                    "raw" => {
                        issues.push(SecurityIssue {
                            severity: SecuritySeverity::Medium,
                            issue_type: SecurityIssueType::UnescapedOutput,
                            description: "Raw block bypasses security escaping".to_string(),
                            location: None,
                            suggested_fix: Some("Ensure raw content is trusted and properly validated".to_string()),
                        });
                    }
                    _ => {}
                }
                
                // Recursively validate block content
                if let Some(content_nodes) = content {
                    self.validate_ast_nodes(content_nodes, nesting_depth, issues, warnings, recommendations)?;
                }
            }
            TemplateNode::Comment(_) => {
                // Comments are safe
            }
        }
        
        Ok(())
    }
    
    /// Generate security recommendations based on issues found
    fn generate_security_recommendations(
        &self,
        issues: &[SecurityIssue],
        recommendations: &mut Vec<String>,
    ) {
        let high_severity_count = issues.iter().filter(|i| matches!(i.severity, SecuritySeverity::High | SecuritySeverity::Critical)).count();
        
        if high_severity_count > 0 {
            recommendations.push("High severity security issues detected - review template security immediately".to_string());
        }
        
        let xss_issues = issues.iter().filter(|i| matches!(i.issue_type, SecurityIssueType::XssVulnerability)).count();
        if xss_issues > 0 {
            recommendations.push("Enable auto-escaping and use security filters for all user content".to_string());
        }
        
        recommendations.push("Regularly audit templates for security vulnerabilities".to_string());
        recommendations.push("Use Content Security Policy headers to prevent XSS attacks".to_string());
        recommendations.push("Validate all template inputs and sanitize user content".to_string());
    }
    
    /// Update security metrics
    fn update_security_metrics(&self, issues: &[SecurityIssue], validation_time_ms: u64) {
        if let Ok(mut metrics) = self.security_metrics.lock() {
            metrics.total_validations += 1;
            
            for issue in issues {
                let issue_type_key = format!("{:?}", issue.issue_type);
                *metrics.issues_by_type.entry(issue_type_key).or_insert(0) += 1;
            }
            
            // Update average validation time
            let total_time = (metrics.average_validation_time_ms * (metrics.total_validations - 1) as f64) + validation_time_ms as f64;
            metrics.average_validation_time_ms = total_time / metrics.total_validations as f64;
        }
    }
    
    /// Get security metrics
    pub fn get_security_metrics(&self) -> Option<SecurityMetrics> {
        self.security_metrics.lock().ok().map(|m| m.clone())
    }
    
    /// Generate Content Security Policy header value
    pub fn generate_csp_header(&self) -> String {
        self.policy.csp_directives
            .iter()
            .map(|(directive, values)| format!("{} {}", directive, values.join(" ")))
            .collect::<Vec<_>>()
            .join("; ")
    }
    
    /// Check if domain is trusted for external includes
    pub fn is_trusted_domain(&self, domain: &str) -> bool {
        self.policy.trusted_domains.contains(domain)
    }
    
    /// Update security policy
    pub fn update_policy(&mut self, policy: SecurityPolicy) {
        self.policy = policy;
    }
    
    /// Get current security policy
    pub fn get_policy(&self) -> &SecurityPolicy {
        &self.policy
    }
}

impl Default for TemplateSecurityValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Security context for template rendering
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// User permissions
    pub user_permissions: HashSet<String>,
    /// Request origin
    pub request_origin: Option<String>,
    /// CSRF token
    pub csrf_token: Option<String>,
    /// User ID
    pub user_id: Option<String>,
    /// Security level override
    pub security_level_override: Option<SecurityValidationLevel>,
}

impl SecurityContext {
    pub fn new() -> Self {
        Self {
            user_permissions: HashSet::new(),
            request_origin: None,
            csrf_token: None,
            user_id: None,
            security_level_override: None,
        }
    }
    
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    pub fn with_csrf_token(mut self, token: String) -> Self {
        self.csrf_token = Some(token);
        self
    }
    
    pub fn with_origin(mut self, origin: String) -> Self {
        self.request_origin = Some(origin);
        self
    }
    
    pub fn add_permission(&mut self, permission: String) {
        self.user_permissions.insert(permission);
    }
    
    pub fn has_permission(&self, permission: &str) -> bool {
        self.user_permissions.contains(permission)
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::template::template_syntax::{TemplateLexer, TemplateParser};
    use crate::stdlib::template::template_core::TemplateDelimiters;
    
    #[test]
    fn test_security_validator_creation() {
        let validator = TemplateSecurityValidator::new();
        assert_eq!(validator.policy.validation_level, SecurityValidationLevel::Standard);
        assert!(validator.policy.enable_xss_protection);
        assert!(validator.policy.enable_csrf_protection);
    }
    
    #[test]
    fn test_xss_detection() {
        let validator = TemplateSecurityValidator::new();
        let template_source = r#"<script>alert('xss')</script>"#;
        
        let delimiters = TemplateDelimiters {
            variable: ("{{".to_string(), "}}".to_string()),
            block: ("{%".to_string(), "%}".to_string()),
            comment: ("{#".to_string(), "#}".to_string()),
        };
        
        let mut lexer = TemplateLexer::new(template_source, &delimiters);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let result = validator.validate_template(&ast, None, template_source).unwrap();
        assert!(!result.is_valid);
        assert!(result.issues.iter().any(|i| matches!(i.issue_type, SecurityIssueType::XssVulnerability)));
    }
    
    #[test]
    fn test_path_traversal_detection() {
        let validator = TemplateSecurityValidator::new();
        let path = Path::new("../../../etc/passwd");
        
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        
        validator.validate_template_path(path, &mut issues, &mut warnings).unwrap();
        assert!(issues.iter().any(|i| matches!(i.issue_type, SecurityIssueType::PathTraversal)));
    }
    
    #[test]
    fn test_template_size_validation() {
        let mut policy = SecurityPolicy::default();
        policy.max_template_size = 100; // Very small for testing
        
        let validator = TemplateSecurityValidator::with_policy(policy);
        let large_template = "x".repeat(200);
        
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        
        validator.validate_template_size(&large_template, &mut issues, &mut warnings).unwrap();
        assert!(issues.iter().any(|i| matches!(i.issue_type, SecurityIssueType::LargeTemplate)));
    }
    
    #[test]
    fn test_csp_header_generation() {
        let validator = TemplateSecurityValidator::new();
        let csp_header = validator.generate_csp_header();
        
        assert!(csp_header.contains("default-src 'self'"));
        assert!(csp_header.contains("script-src"));
        assert!(csp_header.contains("style-src"));
    }
    
    #[test]
    fn test_security_context() {
        let mut context = SecurityContext::new()
            .with_user_id("user123".to_string())
            .with_csrf_token("token456".to_string());
        
        context.add_permission("read_templates".to_string());
        
        assert!(context.has_permission("read_templates"));
        assert!(!context.has_permission("write_templates"));
        assert_eq!(context.user_id, Some("user123".to_string()));
        assert_eq!(context.csrf_token, Some("token456".to_string()));
    }
}
