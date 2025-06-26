//! CURSED Linter - Comprehensive code analysis and quality enforcement
//! 
//! This module provides advanced linting capabilities for CURSED source code including:
//! - Gen Z slang syntax validation
//! - Code quality analysis
//! - AST-based analysis integration
//! - Customizable rule system
//! - Integration with existing parser and lexer modules

use crate::error::CursedError;
use crate::ast::{Program, Statement, Expression, Literal, BinaryOperator, UnaryOperator, Type, AstVisitor};
use crate::lexer::{Lexer, Token, TokenKind};
use std::collections::HashMap;
use std::path::Path;

/// Configuration for the CURSED linter
#[derive(Debug, Clone)]
pub struct LinterConfig {
    /// Enable Gen Z slang validation
    pub validate_slang: bool,
    /// Enforce proper function naming (slay prefix)
    pub enforce_function_naming: bool,
    /// Check variable naming patterns (sus/facts)
    pub check_variable_naming: bool,
    /// Maximum function complexity
    pub max_function_complexity: usize,
    /// Maximum line length
    pub max_line_length: usize,
    /// Require explicit types
    pub require_explicit_types: bool,
    /// Check for unused variables
    pub check_unused_variables: bool,
    /// Enforce proper error handling
    pub enforce_error_handling: bool,
    /// Custom rules to apply
    pub custom_rules: Vec<String>,
    /// Severity level for different rule types
    pub severity_levels: HashMap<String, LintSeverity>,
}

impl Default for LinterConfig {
    fn default() -> Self {
        let mut severity_levels = HashMap::new();
        severity_levels.insert("slang_validation".to_string(), LintSeverity::Error);
        severity_levels.insert("function_naming".to_string(), LintSeverity::Warning);
        severity_levels.insert("variable_naming".to_string(), LintSeverity::Info);
        severity_levels.insert("complexity".to_string(), LintSeverity::Warning);
        severity_levels.insert("line_length".to_string(), LintSeverity::Info);
        severity_levels.insert("unused_variables".to_string(), LintSeverity::Warning);
        
        Self {
            validate_slang: true,
            enforce_function_naming: true,
            check_variable_naming: true,
            max_function_complexity: 10,
            max_line_length: 120,
            require_explicit_types: false,
            check_unused_variables: true,
            enforce_error_handling: true,
            custom_rules: Vec::new(),
            severity_levels,
        }
    }
}

/// Severity levels for lint warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

impl LintSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            LintSeverity::Error => "error",
            LintSeverity::Warning => "warning", 
            LintSeverity::Info => "info",
            LintSeverity::Hint => "hint",
        }
    }
}

/// Individual lint rule for code analysis
#[derive(Debug, Clone)]
pub struct LintRule {
    /// Rule identifier
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Rule category
    pub category: LintCategory,
    /// Severity level
    pub severity: LintSeverity,
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Custom configuration for this rule
    pub config: HashMap<String, String>,
}

/// Categories of lint rules
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LintCategory {
    /// Gen Z slang syntax validation
    SlangSyntax,
    /// Code style and formatting
    Style,
    /// Potential bugs and errors
    Correctness,
    /// Performance optimization suggestions
    Performance,
    /// Security vulnerabilities
    Security,
    /// Code complexity and maintainability
    Complexity,
    /// Naming conventions
    Naming,
    /// Documentation requirements
    Documentation,
}

/// Result of running a lint rule
#[derive(Debug, Clone)]
pub struct LintResult {
    /// Issues found during linting
    pub issues: Vec<LintIssue>,
    /// Summary statistics
    pub summary: LintSummary,
    /// Performance metrics
    pub metrics: LintMetrics,
}

/// Individual lint issue
#[derive(Debug, Clone)]
pub struct LintIssue {
    /// Rule that generated this issue
    pub rule_id: String,
    /// Issue severity
    pub severity: LintSeverity,
    /// Issue message
    pub message: String,
    /// File path where issue was found
    pub file_path: Option<String>,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based) 
    pub column: usize,
    /// Length of the problematic text
    pub length: usize,
    /// Suggested fix (if available)
    pub suggestion: Option<String>,
    /// Additional context information
    pub context: HashMap<String, String>,
}

/// Summary of linting results
#[derive(Debug, Clone, Default)]
pub struct LintSummary {
    /// Total number of issues
    pub total_issues: usize,
    /// Issues by severity level
    pub by_severity: HashMap<LintSeverity, usize>,
    /// Issues by category
    pub by_category: HashMap<LintCategory, usize>,
    /// Issues by rule
    pub by_rule: HashMap<String, usize>,
    /// Files analyzed
    pub files_analyzed: usize,
    /// Lines of code analyzed
    pub lines_analyzed: usize,
}

/// Performance metrics for linting
#[derive(Debug, Clone, Default)]
pub struct LintMetrics {
    /// Time taken for analysis (in milliseconds)
    pub analysis_time_ms: u64,
    /// Memory usage (in bytes)
    pub memory_usage_bytes: usize,
    /// Number of AST nodes processed
    pub ast_nodes_processed: usize,
    /// Number of tokens processed
    pub tokens_processed: usize,
}

/// Main CURSED linter that analyzes source code
pub struct CursedLinter {
    /// Linter configuration
    config: LinterConfig,
    /// Available lint rules
    rules: HashMap<String, LintRule>,
    /// Current analysis context
    context: LintContext,
}

/// Context information during linting
#[derive(Debug, Default)]
struct LintContext {
    /// Current file being analyzed
    current_file: Option<String>,
    /// Variable usage tracking
    variable_usage: HashMap<String, VariableInfo>,
    /// Function definitions
    functions: HashMap<String, FunctionInfo>,
    /// Import statements
    imports: Vec<String>,
    /// Current scope depth
    scope_depth: usize,
    /// Gen Z slang usage statistics
    slang_usage: SlangUsageStats,
}

/// Information about variable usage
#[derive(Debug, Clone)]
struct VariableInfo {
    /// Variable name
    name: String,
    /// Line where declared
    declared_line: usize,
    /// Number of times used
    usage_count: usize,
    /// Is mutable (sus) or immutable (facts)
    is_mutable: bool,
    /// Variable type if known
    var_type: Option<Type>,
}

/// Information about function definitions
#[derive(Debug, Clone)]
struct FunctionInfo {
    /// Function name
    name: String,
    /// Line where declared
    declared_line: usize,
    /// Number of parameters
    param_count: usize,
    /// Estimated complexity
    complexity: usize,
    /// Whether it uses proper Gen Z naming
    proper_slang_naming: bool,
}

/// Statistics about Gen Z slang usage
#[derive(Debug, Default)]
struct SlangUsageStats {
    /// Count of each slang keyword used
    keyword_counts: HashMap<String, usize>,
    /// Total slang usage
    total_slang_usage: usize,
    /// Proper slang pattern usage
    proper_patterns: usize,
    /// Improper slang pattern usage
    improper_patterns: usize,
}

impl CursedLinter {
    /// Create a new CURSED linter with default configuration
    pub fn new() -> Self {
        Self::with_config(LinterConfig::default())
    }

    /// Create a new CURSED linter with custom configuration
    pub fn with_config(config: LinterConfig) -> Self {
        let mut linter = Self {
            config,
            rules: HashMap::new(),
            context: LintContext::default(),
        };
        
        linter.initialize_rules();
        linter
    }

    /// Initialize built-in lint rules
    fn initialize_rules(&mut self) {
        // Gen Z slang validation rules
        self.add_rule(LintRule {
            id: "slang_function_naming".to_string(),
            description: "Functions should use 'slay' prefix for proper Gen Z naming".to_string(),
            category: LintCategory::SlangSyntax,
            severity: LintSeverity::Warning,
            enabled: self.config.enforce_function_naming,
            config: HashMap::new(),
        });

        self.add_rule(LintRule {
            id: "slang_variable_naming".to_string(),
            description: "Variables should use 'sus' (mutable) or 'facts' (immutable) keywords".to_string(),
            category: LintCategory::SlangSyntax,
            severity: LintSeverity::Info,
            enabled: self.config.check_variable_naming,
            config: HashMap::new(),
        });

        self.add_rule(LintRule {
            id: "slang_keyword_usage".to_string(),
            description: "Validate proper usage of Gen Z slang keywords".to_string(),
            category: LintCategory::SlangSyntax,
            severity: LintSeverity::Error,
            enabled: self.config.validate_slang,
            config: HashMap::new(),
        });

        // Code quality rules
        self.add_rule(LintRule {
            id: "function_complexity".to_string(),
            description: format!("Function complexity should not exceed {}", self.config.max_function_complexity),
            category: LintCategory::Complexity,
            severity: LintSeverity::Warning,
            enabled: true,
            config: HashMap::new(),
        });

        self.add_rule(LintRule {
            id: "line_length".to_string(),
            description: format!("Lines should not exceed {} characters", self.config.max_line_length),
            category: LintCategory::Style,
            severity: LintSeverity::Info,
            enabled: true,
            config: HashMap::new(),
        });

        self.add_rule(LintRule {
            id: "unused_variables".to_string(),
            description: "Variables should be used after declaration".to_string(),
            category: LintCategory::Correctness,
            severity: LintSeverity::Warning,
            enabled: self.config.check_unused_variables,
            config: HashMap::new(),
        });

        // Error handling rules
        self.add_rule(LintRule {
            id: "error_handling".to_string(),
            description: "Functions should handle errors properly with Result types".to_string(),
            category: LintCategory::Correctness,
            severity: LintSeverity::Warning,
            enabled: self.config.enforce_error_handling,
            config: HashMap::new(),
        });

        // Performance rules
        self.add_rule(LintRule {
            id: "unnecessary_allocation".to_string(),
            description: "Avoid unnecessary memory allocations".to_string(),
            category: LintCategory::Performance,
            severity: LintSeverity::Hint,
            enabled: true,
            config: HashMap::new(),
        });

        // Security rules
        self.add_rule(LintRule {
            id: "hardcoded_secrets".to_string(),
            description: "Avoid hardcoded secrets and credentials".to_string(),
            category: LintCategory::Security,
            severity: LintSeverity::Error,
            enabled: true,
            config: HashMap::new(),
        });
    }

    /// Add a new lint rule
    pub fn add_rule(&mut self, rule: LintRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    /// Get a lint rule by ID
    pub fn get_rule(&self, rule_id: &str) -> Option<&LintRule> {
        self.rules.get(rule_id)
    }

    /// Enable or disable a lint rule
    pub fn set_rule_enabled(&mut self, rule_id: &str, enabled: bool) -> Result<(), CursedError> {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.enabled = enabled;
            Ok(())
        } else {
            Err(CursedError::syntax_error(&format!("Unknown lint rule: {}", rule_id)))
        }
    }

    /// Lint a CURSED source file
    pub fn lint_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<LintResult, CursedError> {
        let path = file_path.as_ref();
        let source = std::fs::read_to_string(path)
            .map_err(|e| CursedError::syntax_error(&format!("Failed to read file: {}", e)))?;
        
        self.context.current_file = Some(path.to_string_lossy().to_string());
        self.lint_source(&source)
    }

    /// Lint CURSED source code
    pub fn lint_source(&mut self, source: &str) -> Result<LintResult, CursedError> {
        let start_time = std::time::Instant::now();
        let mut issues = Vec::new();
        
        // Reset context for new analysis
        self.context = LintContext::default();
        if let Some(file) = &self.context.current_file {
            self.context.current_file = Some(file.clone());
        }

        // Tokenize source for lexical analysis
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize()?;
        
        // Perform token-level analysis
        issues.extend(self.analyze_tokens(&tokens, source)?);
        
        // Parse source for AST analysis
        let program = crate::ast::parse_program(source)?;
        
        // Perform AST-level analysis
        issues.extend(self.analyze_ast(&program)?);
        
        // Perform line-level analysis
        issues.extend(self.analyze_lines(source)?);
        
        // Build summary
        let summary = self.build_summary(&issues, source);
        
        // Calculate metrics
        let metrics = LintMetrics {
            analysis_time_ms: start_time.elapsed().as_millis() as u64,
            memory_usage_bytes: self.estimate_memory_usage(),
            ast_nodes_processed: self.count_ast_nodes(&program),
            tokens_processed: tokens.len(),
        };

        Ok(LintResult {
            issues,
            summary,
            metrics,
        })
    }

    /// Analyze tokens for lexical issues
    fn analyze_tokens(&mut self, tokens: &[Token], source: &str) -> Result<Vec<LintIssue>, CursedError> {
        let mut issues = Vec::new();
        
        for token in tokens {
            // Track Gen Z slang usage
            if self.is_slang_keyword(&token.lexeme) {
                *self.context.slang_usage.keyword_counts
                    .entry(token.lexeme.clone())
                    .or_insert(0) += 1;
                self.context.slang_usage.total_slang_usage += 1;
            }
            
            // Check for proper slang patterns
            if let Some(issue) = self.check_slang_patterns(token)? {
                issues.push(issue);
            }
            
            // Check for hardcoded secrets
            if let Some(issue) = self.check_hardcoded_secrets(token)? {
                issues.push(issue);
            }
        }
        
        Ok(issues)
    }

    /// Analyze AST for structural issues
    fn analyze_ast(&mut self, program: &Program) -> Result<Vec<LintIssue>, CursedError> {
        let mut issues = Vec::new();
        
        // Analyze statements
        for statement in &program.statements {
            issues.extend(self.analyze_statement(statement)?);
        }
        
        // Check for unused variables
        if self.rules.get("unused_variables").map_or(false, |r| r.enabled) {
            issues.extend(self.check_unused_variables()?);
        }
        
        Ok(issues)
    }

    /// Analyze individual statement
    fn analyze_statement(&mut self, statement: &Statement) -> Result<Vec<LintIssue>, CursedError> {
        let mut issues = Vec::new();
        
        match statement {
            Statement::Function(func_stmt) => {
                // Check function naming
                if let Some(issue) = self.check_function_naming(&func_stmt.name, 1)? {
                    issues.push(issue);
                }
                
                // Track function info
                self.context.functions.insert(func_stmt.name.clone(), FunctionInfo {
                    name: func_stmt.name.clone(),
                    declared_line: 1, // TODO: Get actual line number
                    param_count: func_stmt.parameters.len(),
                    complexity: self.calculate_function_complexity(&func_stmt.body),
                    proper_slang_naming: func_stmt.name.starts_with("slay_") || func_stmt.name == "main",
                });
                
                // Check function complexity
                let complexity = self.calculate_function_complexity(&func_stmt.body);
                if complexity > self.config.max_function_complexity {
                    issues.push(LintIssue {
                        rule_id: "function_complexity".to_string(),
                        severity: LintSeverity::Warning,
                        message: format!("Function '{}' has complexity {} which exceeds maximum {}", 
                                       func_stmt.name, complexity, self.config.max_function_complexity),
                        file_path: self.context.current_file.clone(),
                        line: 1,
                        column: 1,
                        length: func_stmt.name.len(),
                        suggestion: Some("Consider breaking this function into smaller functions".to_string()),
                        context: HashMap::new(),
                    });
                }
                
                // Analyze function body
                for stmt in &func_stmt.body {
                    issues.extend(self.analyze_statement(stmt)?);
                }
            },
            Statement::Let(let_stmt) => {
                // Track variable declaration
                self.context.variable_usage.insert(let_stmt.name.clone(), VariableInfo {
                    name: let_stmt.name.clone(),
                    declared_line: 1, // TODO: Get actual line number
                    usage_count: 0,
                    is_mutable: true, // TODO: Determine from context
                    var_type: None,
                });
                
                // Check variable naming
                if let Some(issue) = self.check_variable_naming(&let_stmt.name, 1)? {
                    issues.push(issue);
                }
                
                // Analyze value expression
                issues.extend(self.analyze_expression(&let_stmt.value)?);
            },
            Statement::If(if_stmt) => {
                issues.extend(self.analyze_expression(&if_stmt.condition)?);
                for stmt in &if_stmt.then_branch {
                    issues.extend(self.analyze_statement(stmt)?);
                }
                if let Some(else_stmts) = &if_stmt.else_branch {
                    for stmt in else_stmts {
                        issues.extend(self.analyze_statement(stmt)?);
                    }
                }
            },
            Statement::While(while_stmt) => {
                issues.extend(self.analyze_expression(&while_stmt.condition)?);
                for stmt in &while_stmt.body {
                    issues.extend(self.analyze_statement(stmt)?);
                }
            },
            Statement::Return(return_stmt) => {
                if let Some(expr) = &return_stmt.value {
                    issues.extend(self.analyze_expression(expr)?);
                }
            },
            _ => {
                // Handle other statement types
            }
        }
        
        Ok(issues)
    }

    /// Analyze individual expression
    fn analyze_expression(&mut self, expression: &Expression) -> Result<Vec<LintIssue>, CursedError> {
        let mut issues = Vec::new();
        
        match expression {
            Expression::Identifier(name) => {
                // Track variable usage
                if let Some(var_info) = self.context.variable_usage.get_mut(name) {
                    var_info.usage_count += 1;
                }
            },
            Expression::Binary(binary_expr) => {
                issues.extend(self.analyze_expression(&binary_expr.left)?);
                issues.extend(self.analyze_expression(&binary_expr.right)?);
            },
            Expression::Unary(unary_expr) => {
                issues.extend(self.analyze_expression(&unary_expr.operand)?);
            },
            Expression::Call(call_expr) => {
                issues.extend(self.analyze_expression(&call_expr.function)?);
                for arg in &call_expr.arguments {
                    issues.extend(self.analyze_expression(arg)?);
                }
            },
            Expression::Array(elements) => {
                for element in elements {
                    issues.extend(self.analyze_expression(element)?);
                }
            },
            Expression::Map(pairs) => {
                for (key, value) in pairs {
                    issues.extend(self.analyze_expression(key)?);
                    issues.extend(self.analyze_expression(value)?);
                }
            },
            _ => {
                // Handle other expression types
            }
        }
        
        Ok(issues)
    }

    /// Analyze source code line by line
    fn analyze_lines(&self, source: &str) -> Result<Vec<LintIssue>, CursedError> {
        let mut issues = Vec::new();
        
        for (line_num, line) in source.lines().enumerate() {
            let line_num = line_num + 1; // 1-based line numbers
            
            // Check line length
            if line.len() > self.config.max_line_length {
                issues.push(LintIssue {
                    rule_id: "line_length".to_string(),
                    severity: LintSeverity::Info,
                    message: format!("Line exceeds maximum length of {} characters", self.config.max_line_length),
                    file_path: self.context.current_file.clone(),
                    line: line_num,
                    column: self.config.max_line_length + 1,
                    length: line.len() - self.config.max_line_length,
                    suggestion: Some("Consider breaking this line into multiple lines".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        
        Ok(issues)
    }

    /// Check if a token represents a Gen Z slang keyword
    fn is_slang_keyword(&self, token: &str) -> bool {
        matches!(token, 
            "slay" | "yeet" | "facts" | "sus" | "lowkey" | "highkey" | 
            "periodt" | "bestie" | "cap" | "no_cap" | "vibez" | "spill" | 
            "tea" | "normie" | "stan" | "yolo" | "based" | "cringe" |
            "rizz" | "bussin" | "sheesh" | "deadass"
        )
    }

    /// Check for proper slang patterns in tokens
    fn check_slang_patterns(&mut self, token: &Token) -> Result<Option<LintIssue>, CursedError> {
        if !self.rules.get("slang_keyword_usage").map_or(false, |r| r.enabled) {
            return Ok(None);
        }

        // Check for common slang misusages
        match token.lexeme.as_str() {
            "slay" => {
                // Should be followed by function name
                self.context.slang_usage.proper_patterns += 1;
            },
            "yeet" => {
                // Should be used for imports or throwing
                self.context.slang_usage.proper_patterns += 1;
            },
            "sus" | "facts" => {
                // Should be used for variable declarations
                self.context.slang_usage.proper_patterns += 1;
            },
            "lowkey" | "highkey" => {
                // Should be used for conditionals
                self.context.slang_usage.proper_patterns += 1;
            },
            "periodt" => {
                // Should be used for loops or emphasis
                self.context.slang_usage.proper_patterns += 1;
            },
            _ => {}
        }

        Ok(None)
    }

    /// Check for hardcoded secrets in tokens
    fn check_hardcoded_secrets(&self, token: &Token) -> Result<Option<LintIssue>, CursedError> {
        if !self.rules.get("hardcoded_secrets").map_or(false, |r| r.enabled) {
            return Ok(None);
        }

        if matches!(token.kind, TokenKind::String) {
            let content = token.lexeme.to_lowercase();
            let suspicious_patterns = [
                "password", "secret", "api_key", "token", "private_key",
                "access_key", "auth", "credential", "passwd"
            ];

            for pattern in &suspicious_patterns {
                if content.contains(pattern) && content.len() > 10 {
                    return Ok(Some(LintIssue {
                        rule_id: "hardcoded_secrets".to_string(),
                        severity: LintSeverity::Error,
                        message: "Potential hardcoded secret detected".to_string(),
                        file_path: self.context.current_file.clone(),
                        line: token.line,
                        column: token.column,
                        length: token.lexeme.len(),
                        suggestion: Some("Use environment variables or secure configuration".to_string()),
                        context: HashMap::new(),
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Check function naming conventions
    fn check_function_naming(&self, name: &str, line: usize) -> Result<Option<LintIssue>, CursedError> {
        if !self.rules.get("slang_function_naming").map_or(false, |r| r.enabled) {
            return Ok(None);
        }

        if name != "main" && !name.starts_with("slay_") {
            return Ok(Some(LintIssue {
                rule_id: "slang_function_naming".to_string(),
                severity: LintSeverity::Warning,
                message: format!("Function '{}' should use 'slay_' prefix for proper Gen Z naming", name),
                file_path: self.context.current_file.clone(),
                line,
                column: 1,
                length: name.len(),
                suggestion: Some(format!("Rename to 'slay_{}'", name)),
                context: HashMap::new(),
            }));
        }

        Ok(None)
    }

    /// Check variable naming conventions
    fn check_variable_naming(&self, name: &str, line: usize) -> Result<Option<LintIssue>, CursedError> {
        if !self.rules.get("slang_variable_naming").map_or(false, |r| r.enabled) {
            return Ok(None);
        }

        // For now, just provide informational message about CURSED variable naming
        if name.len() > 20 {
            return Ok(Some(LintIssue {
                rule_id: "slang_variable_naming".to_string(),
                severity: LintSeverity::Info,
                message: format!("Consider shorter variable name for '{}'", name),
                file_path: self.context.current_file.clone(),
                line,
                column: 1,
                length: name.len(),
                suggestion: None,
                context: HashMap::new(),
            }));
        }

        Ok(None)
    }

    /// Check for unused variables
    fn check_unused_variables(&self) -> Result<Vec<LintIssue>, CursedError> {
        let mut issues = Vec::new();

        for (name, info) in &self.context.variable_usage {
            if info.usage_count == 0 {
                issues.push(LintIssue {
                    rule_id: "unused_variables".to_string(),
                    severity: LintSeverity::Warning,
                    message: format!("Variable '{}' is declared but never used", name),
                    file_path: self.context.current_file.clone(),
                    line: info.declared_line,
                    column: 1,
                    length: name.len(),
                    suggestion: Some(format!("Remove unused variable '{}'", name)),
                    context: HashMap::new(),
                });
            }
        }

        Ok(issues)
    }

    /// Calculate function complexity (simplified cyclomatic complexity)
    fn calculate_function_complexity(&self, statements: &[Statement]) -> usize {
        let mut complexity = 1; // Base complexity

        for statement in statements {
            complexity += match statement {
                Statement::If(_) => 1,
                Statement::While(_) => 1,
                Statement::For(_) => 1,
                _ => 0,
            };
        }

        complexity
    }

    /// Build summary of linting results
    fn build_summary(&self, issues: &[LintIssue], source: &str) -> LintSummary {
        let mut summary = LintSummary::default();
        
        summary.total_issues = issues.len();
        summary.files_analyzed = 1;
        summary.lines_analyzed = source.lines().count();
        
        // Count by severity
        for issue in issues {
            *summary.by_severity.entry(issue.severity).or_insert(0) += 1;
        }
        
        // Count by rule
        for issue in issues {
            *summary.by_rule.entry(issue.rule_id.clone()).or_insert(0) += 1;
        }
        
        summary
    }

    /// Estimate memory usage for metrics
    fn estimate_memory_usage(&self) -> usize {
        // Simplified memory usage estimation
        let context_size = std::mem::size_of_val(&self.context);
        let rules_size = self.rules.len() * std::mem::size_of::<LintRule>();
        context_size + rules_size
    }

    /// Count AST nodes for metrics
    fn count_ast_nodes(&self, program: &Program) -> usize {
        // Simplified node counting
        program.statements.len()
    }

    /// Get configuration
    pub fn config(&self) -> &LinterConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: LinterConfig) {
        self.config = config;
        self.initialize_rules(); // Reinitialize rules with new config
    }

    /// Get all available rules
    pub fn rules(&self) -> &HashMap<String, LintRule> {
        &self.rules
    }

    /// Get enabled rules
    pub fn enabled_rules(&self) -> Vec<&LintRule> {
        self.rules.values().filter(|rule| rule.enabled).collect()
    }
}

impl Default for CursedLinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for linting
pub mod utils {
    use super::*;

    /// Quick lint check for source code
    pub fn quick_lint(source: &str) -> Result<LintResult, CursedError> {
        let mut linter = CursedLinter::new();
        linter.lint_source(source)
    }

    /// Create linter with minimal configuration
    pub fn minimal_linter() -> CursedLinter {
        let mut config = LinterConfig::default();
        config.validate_slang = true;
        config.enforce_function_naming = false;
        config.check_variable_naming = false;
        config.check_unused_variables = false;
        
        CursedLinter::with_config(config)
    }

    /// Create linter with strict configuration
    pub fn strict_linter() -> CursedLinter {
        let mut config = LinterConfig::default();
        config.validate_slang = true;
        config.enforce_function_naming = true;
        config.check_variable_naming = true;
        config.max_function_complexity = 5;
        config.max_line_length = 80;
        config.require_explicit_types = true;
        config.check_unused_variables = true;
        config.enforce_error_handling = true;
        
        CursedLinter::with_config(config)
    }

    /// Format lint results as human-readable text
    pub fn format_results(result: &LintResult) -> String {
        let mut output = String::new();
        
        if result.issues.is_empty() {
            output.push_str("✅ No issues found! Your CURSED code is clean periodt\n");
        } else {
            output.push_str(&format!("Found {} issues:\n\n", result.issues.len()));
            
            for issue in &result.issues {
                let severity_icon = match issue.severity {
                    LintSeverity::Error => "❌",
                    LintSeverity::Warning => "⚠️",
                    LintSeverity::Info => "ℹ️",
                    LintSeverity::Hint => "💡",
                };
                
                output.push_str(&format!(
                    "{} {} ({}:{}:{}) - {}\n",
                    severity_icon,
                    issue.severity.as_str().to_uppercase(),
                    issue.file_path.as_deref().unwrap_or("<source>"),
                    issue.line,
                    issue.column,
                    issue.message
                ));
                
                if let Some(suggestion) = &issue.suggestion {
                    output.push_str(&format!("   💡 Suggestion: {}\n", suggestion));
                }
                
                output.push('\n');
            }
        }
        
        // Add summary
        output.push_str(&format!(
            "Summary: {} issues, {} files analyzed, {} lines analyzed\n",
            result.summary.total_issues,
            result.summary.files_analyzed,
            result.summary.lines_analyzed
        ));
        
        output.push_str(&format!(
            "Analysis completed in {}ms\n",
            result.metrics.analysis_time_ms
        ));
        
        output
    }
}

/// Legacy support for existing minimal implementation
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED linting system enabled with Gen Z slang validation periodt".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_creation() {
        let linter = CursedLinter::new();
        assert!(!linter.rules.is_empty());
        assert!(linter.config.validate_slang);
    }

    #[test]
    fn test_slang_keyword_detection() {
        let linter = CursedLinter::new();
        assert!(linter.is_slang_keyword("slay"));
        assert!(linter.is_slang_keyword("sus"));
        assert!(linter.is_slang_keyword("facts"));
        assert!(!linter.is_slang_keyword("function"));
    }

    #[test]
    fn test_quick_lint_simple() {
        let source = r#"
            slay main() {
                sus x = 42;
                facts message = "Hello, CURSED!";
            }
        "#;
        
        let result = utils::quick_lint(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_naming_rule() {
        let mut linter = CursedLinter::new();
        let result = linter.check_function_naming("bad_function", 1);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_minimal_linter() {
        let linter = utils::minimal_linter();
        assert!(linter.config.validate_slang);
        assert!(!linter.config.enforce_function_naming);
    }

    #[test]
    fn test_strict_linter() {
        let linter = utils::strict_linter();
        assert!(linter.config.validate_slang);
        assert!(linter.config.enforce_function_naming);
        assert_eq!(linter.config.max_function_complexity, 5);
    }
}
