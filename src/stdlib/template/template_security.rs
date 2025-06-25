use crate::error::CursedError;
/// Template Security Validator - Advanced security features for CURSED templates
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use regex::Regex;
use tracing::{debug, error, info, instrument, warn};

use crate::object::Object as CursedObject;
use super::template_core::{TemplateContext, TemplateConfig};
use super::template_syntax::{TemplateAst, TemplateNode, TemplateExpression};

/// Template security validation levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityValidationLevel {
    /// Minimal security checks
    /// Standard security validation
    /// Strict security with comprehensive checks
    /// Maximum security for production environments
/// Security policy configuration
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    /// Validation level
    /// Allowed template directories
    /// Blocked template directories
    /// Allowed file extensions
    /// Maximum template nesting depth
    /// Maximum template size in bytes
    /// Enable path traversal protection
    /// Enable content injection protection
    /// Enable XSS protection
    /// Enable CSRF protection
    /// Content Security Policy settings
    /// Trusted domains for external includes
    /// Maximum render time (for DoS protection)
    /// Maximum memory usage (for DoS protection)
impl Default for SecurityPolicy {
    fn default() -> Self {
        let mut csp_directives = HashMap::new();
        csp_directives.insert("default-src".to_string(), vec!["'self'".to_string()]);
        csp_directives.insert("script-src".to_string(), vec!["'self'".to_string(), "'unsafe-inline'".to_string()]);
        csp_directives.insert("style-src".to_string(), vec!["'self'".to_string(), "'unsafe-inline'".to_string()]);
        csp_directives.insert("img-src".to_string(), vec!["'self'".to_string(), "data:".to_string()]);
        
        Self {
            max_template_size: 1024 * 1024, // 1MB
            max_render_time_ms: 5000, // 5 seconds
            max_memory_usage_mb: 100, // 100MB
        }
    }
/// Security validation result
#[derive(Debug, Clone)]
pub struct SecurityValidationResult {
    /// Whether validation passed
    /// Security issues found
    /// Security warnings
    /// Security recommendations
    /// Validation time
/// Security issue types
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    /// Issue severity
    /// Issue type
    /// Issue description
    /// Template location (if applicable)
    /// Suggested fix
/// Security issue severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
/// Types of security issues
#[derive(Debug, Clone)]
pub enum SecurityIssueType {
/// Template security validator
pub struct TemplateSecurityValidator {
    /// Security policy
    /// XSS pattern detection
    /// Path traversal patterns
    /// Content injection patterns
    /// Performance monitoring
    /// Security metrics
/// Security metrics tracking
#[derive(Debug, Clone)]
pub struct SecurityMetrics {
    /// Total validations performed
    /// Security issues found by type
    /// Average validation time
    /// Blocked attempts
    /// False positives reported
impl TemplateSecurityValidator {
    /// Create a new security validator with default policy
    pub fn new() -> Self {
        Self::with_policy(SecurityPolicy::default())
    /// Create a security validator with custom policy
    pub fn with_policy(policy: SecurityPolicy) -> Self {
        let xss_patterns = vec![
            Regex::new(r"data:text/html").unwrap(),
        ];
        
        let path_traversal_patterns = vec![
            Regex::new(r"\.\./").unwrap(),
            Regex::new(r"/etc/passwd").unwrap(),
            Regex::new(r"%2e%2e%2f").unwrap(), // URL encoded ../
        ];
        
        let injection_patterns = vec![
        ];
        
        Self {
            security_metrics: Arc::new(std::sync::Mutex::new(SecurityMetrics {
        }
    }
    
    /// Validate a template for security issues
    #[instrument(skip(self, template_ast, template_path))]
    pub fn validate_template(
    ) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        info!("Starting security validation");
        
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        // Update validation count
        if let Ok(mut count) = self.validation_count.lock() {
            *count += 1;
        // Validate template path
        if let Some(path) = template_path {
            self.validate_template_path(path, &mut issues, &mut warnings)?;
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
            "Security validation completed"
        );
        
        Ok(SecurityValidationResult {
        })
    /// Validate template file path
    fn validate_template_path(
    ) -> crate::error::Result<()> {
        // Check path traversal
        if self.policy.enable_path_traversal_protection {
            let path_str = path.to_string_lossy();
            for pattern in &self.path_traversal_patterns {
                if pattern.is_match(&path_str) {
                    issues.push(SecurityIssue {
                    });
                }
            }
        // Check allowed directories
        if !self.policy.allowed_directories.is_empty() {
            let is_allowed = self.policy.allowed_directories.iter()
                .any(|allowed_dir| path.starts_with(allowed_dir));
            
            if !is_allowed {
                issues.push(SecurityIssue {
                });
            }
        }
        
        // Check blocked directories
        for blocked_dir in &self.policy.blocked_directories {
            if path.starts_with(blocked_dir) {
                issues.push(SecurityIssue {
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
    /// Validate template size
    fn validate_template_size(
    ) -> crate::error::Result<()> {
        let size = template_source.len();
        
        if size > self.policy.max_template_size {
            issues.push(SecurityIssue {
            });
        if size > self.policy.max_template_size / 2 {
            warnings.push(format!("Template size ({} bytes) is approaching maximum limit", size));
        Ok(())
    /// Validate template content for security issues
    fn validate_template_content(
    ) -> crate::error::Result<()> {
        // Check for XSS patterns
        if self.policy.enable_xss_protection {
            for pattern in &self.xss_patterns {
                if pattern.is_match(template_source) {
                    issues.push(SecurityIssue {
                    });
                }
            }
        // Check for content injection patterns
        if self.policy.enable_content_injection_protection {
            for pattern in &self.injection_patterns {
                if pattern.is_match(template_source) {
                    issues.push(SecurityIssue {
                    });
                }
            }
        // Check for unescaped output patterns
        let unescaped_pattern = Regex::new(r"\{\{\{\s*[^}]+\s*\}\}\}").unwrap();
        if unescaped_pattern.is_match(template_source) {
            warnings.push("Unescaped output detected - ensure content is trusted".to_string());
        Ok(())
    /// Validate template AST for security issues
    fn validate_template_ast(
    ) -> crate::error::Result<()> {
        let mut nesting_depth = 0;
        self.validate_ast_nodes(&ast.nodes, &mut nesting_depth, issues, warnings, recommendations)?;
        Ok(())
    /// Recursively validate AST nodes
    fn validate_ast_nodes(
    ) -> crate::error::Result<()> {
        *nesting_depth += 1;
        
        // Check nesting depth
        if *nesting_depth > self.policy.max_nesting_depth {
            issues.push(SecurityIssue {
            });
        for node in nodes {
            self.validate_ast_node(node, nesting_depth, issues, warnings, recommendations)?;
        *nesting_depth -= 1;
        Ok(())
    /// Validate individual AST node
    fn validate_ast_node(
    ) -> crate::error::Result<()> {
        match node {
            TemplateNode::Text(_) => {
                // Text nodes are generally safe
            }
            TemplateNode::Variable { name, filters } => {
                // Check for suspicious variable names
                if name.contains("password") || name.contains("secret") || name.contains("token") {
                    warnings.push(format!("Potentially sensitive variable in template: {}", name));
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
                        });
                    }
                    _ => {}
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
    /// Generate security recommendations based on issues found
    fn generate_security_recommendations(
    ) {
        let high_severity_count = issues.iter().filter(|i| matches!(i.severity, SecuritySeverity::High | SecuritySeverity::Critical)).count();
        
        if high_severity_count > 0 {
            recommendations.push("High severity security issues detected - review template security immediately".to_string());
        let xss_issues = issues.iter().filter(|i| matches!(i.issue_type, SecurityIssueType::XssVulnerability)).count();
        if xss_issues > 0 {
            recommendations.push("Enable auto-escaping and use security filters for all user content".to_string());
        recommendations.push("Regularly audit templates for security vulnerabilities".to_string());
        recommendations.push("Use Content Security Policy headers to prevent XSS attacks".to_string());
        recommendations.push("Validate all template inputs and sanitize user content".to_string());
    /// Update security metrics
    fn update_security_metrics(&self, issues: &[SecurityIssue], validation_time_ms: u64) {
        if let Ok(mut metrics) = self.security_metrics.lock() {
            metrics.total_validations += 1;
            
            for issue in issues {
                let issue_type_key = format!("{:?}", issue.issue_type);
                *metrics.issues_by_type.entry(issue_type_key).or_insert(0) += 1;
            // Update average validation time
            let total_time = (metrics.average_validation_time_ms * (metrics.total_validations - 1) as f64) + validation_time_ms as f64;
            metrics.average_validation_time_ms = total_time / metrics.total_validations as f64;
        }
    }
    
    /// Get security metrics
    pub fn get_security_metrics(&self) -> Option<SecurityMetrics> {
        self.security_metrics.lock().ok().map(|m| m.clone())
    /// Generate Content Security Policy header value
    pub fn generate_csp_header(&self) -> String {
        self.policy.csp_directives
            .iter()
            .map(|(directive, values)| format!("{} {}", directive, values.join(" ")))
            .collect::<Vec<_>>()
            .join("; ")
    /// Check if domain is trusted for external includes
    pub fn is_trusted_domain(&self, domain: &str) -> bool {
        self.policy.trusted_domains.contains(domain)
    /// Update security policy
    pub fn update_policy(&mut self, policy: SecurityPolicy) {
        self.policy = policy;
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
    /// Request origin
    /// CSRF token
    /// User ID
    /// Security level override
impl SecurityContext {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    pub fn with_csrf_token(mut self, token: String) -> Self {
        self.csrf_token = Some(token);
        self
    pub fn with_origin(mut self, origin: String) -> Self {
        self.request_origin = Some(origin);
        self
    pub fn add_permission(&mut self, permission: String) {
        self.user_permissions.insert(permission);
    pub fn has_permission(&self, permission: &str) -> bool {
        self.user_permissions.contains(permission)
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

