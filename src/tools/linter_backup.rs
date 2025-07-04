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
    /// Rule category
    pub category: LintCategory,
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

    /// Create a strict linter with all rules enabled and strict settings
    pub fn strict_linter() -> Self {
        let mut severity_levels = HashMap::new();
        severity_levels.insert("slang_validation".to_string(), LintSeverity::Error);
        severity_levels.insert("function_naming".to_string(), LintSeverity::Error);
        severity_levels.insert("variable_naming".to_string(), LintSeverity::Warning);
        severity_levels.insert("complexity".to_string(), LintSeverity::Error);
        severity_levels.insert("line_length".to_string(), LintSeverity::Warning);
        severity_levels.insert("unused_variables".to_string(), LintSeverity::Error);
        
        let config = LinterConfig {
            validate_slang: true,
            enforce_function_naming: true,
            check_variable_naming: true,
            max_function_complexity: 5,
            max_line_length: 80,
            require_explicit_types: true,
            check_unused_variables: true,
            enforce_error_handling: true,
            custom_rules: vec![],
            severity_levels,
        };
        Self::with_config(config)
    }

    /// Create a minimal linter with only essential rules enabled
    pub fn minimal_linter() -> Self {
        let mut severity_levels = HashMap::new();
        severity_levels.insert("slang_validation".to_string(), LintSeverity::Info);
        severity_levels.insert("function_naming".to_string(), LintSeverity::Info);
        severity_levels.insert("variable_naming".to_string(), LintSeverity::Info);
        severity_levels.insert("complexity".to_string(), LintSeverity::Warning);
        severity_levels.insert("line_length".to_string(), LintSeverity::Info);
        severity_levels.insert("unused_variables".to_string(), LintSeverity::Info);
        
        let config = LinterConfig {
            validate_slang: false,
            enforce_function_naming: false,
            check_variable_naming: false,
            max_function_complexity: 20,
            max_line_length: 120,
            require_explicit_types: false,
            check_unused_variables: false,
            enforce_error_handling: false,
            custom_rules: vec![],
            severity_levels,
        };
        Self::with_config(config)
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

    /// Get category for a rule, defaulting to Style if rule not found
    fn get_rule_category(&self, rule_id: &str) -> LintCategory {
        self.get_rule(rule_id)
            .map(|rule| rule.category.clone())
            .unwrap_or(LintCategory::Style)
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

    /// Analyze AST for structural issues using the AstVisitor pattern
    fn analyze_ast(&mut self, program: &Program) -> Result<Vec<LintIssue>, CursedError> {
        // Use the AstVisitor implementation for comprehensive analysis
        let issues = self.visit_program(program);
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
                    let rule_id = "function_complexity".to_string();
                    issues.push(LintIssue {
                        rule_id: rule_id.clone(),
                        category: self.get_rule_category(&rule_id),
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
                let rule_id = "line_length".to_string();
                issues.push(LintIssue {
                    rule_id: rule_id.clone(),
                    category: self.get_rule_category(&rule_id),
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
                    let rule_id = "hardcoded_secrets".to_string();
                    return Ok(Some(LintIssue {
                        rule_id: rule_id.clone(),
                        category: self.get_rule_category(&rule_id),
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
            let rule_id = "slang_function_naming".to_string();
            return Ok(Some(LintIssue {
                rule_id: rule_id.clone(),
                category: self.get_rule_category(&rule_id),
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
            let rule_id = "slang_variable_naming".to_string();
            return Ok(Some(LintIssue {
                rule_id: rule_id.clone(),
                category: self.get_rule_category(&rule_id),
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
                let rule_id = "unused_variables".to_string();
                issues.push(LintIssue {
                    rule_id: rule_id.clone(),
                    category: self.get_rule_category(&rule_id),
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
    
    // Helper methods for AstVisitor implementation
    
    /// Check if a function has proper Gen Z slang naming
    fn has_proper_slang_naming(&self, name: &str) -> bool {
        name == "main" || name.starts_with("slay_") || self.is_slang_keyword(name)
    }
    
    /// Check package naming conventions
    fn check_package_naming(&self, package: &str) -> Option<LintIssue> {
        if package.is_empty() {
            return Some(LintIssue {
                rule_id: "package_naming".to_string(),
                category: LintCategory::Naming,
                severity: LintSeverity::Error,
                message: "Package name cannot be empty".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 0,
                suggestion: Some("Provide a valid package name".to_string()),
                context: HashMap::new(),
            });
        }
        
        if !package.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Some(LintIssue {
                rule_id: "package_naming".to_string(),
                category: LintCategory::Naming,
                severity: LintSeverity::Warning,
                message: "Package name should only contain alphanumeric characters and underscores".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: package.len(),
                suggestion: None,
                context: HashMap::new(),
            });
        }
        
        None
    }
    
    /// Perform global analysis after visiting all statements
    fn perform_global_analysis(&self) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        // Check for unused variables
        if self.rules.get("unused_variables").map_or(false, |r| r.enabled) {
            issues.extend(self.check_unused_variables().unwrap_or_default());
        }
        
        // Check for unused imports
        issues.extend(self.check_unused_imports());
        
        // Check for function naming consistency
        issues.extend(self.check_function_naming_consistency());
        
        // Check for slang usage patterns
        issues.extend(self.check_slang_usage_patterns());
        
        issues
    }
    
    /// Check parameter naming conventions
    fn check_parameter_naming(&self, name: &str, line: usize) -> Option<LintIssue> {
        if name.len() > 25 {
            return Some(LintIssue {
                rule_id: "parameter_naming".to_string(),
                category: LintCategory::Naming,
                severity: LintSeverity::Info,
                message: format!("Parameter name '{}' is very long", name),
                file_path: self.context.current_file.clone(),
                line,
                column: 1,
                length: name.len(),
                suggestion: Some("Consider using a shorter parameter name".to_string()),
                context: HashMap::new(),
            });
        }
        
        None
    }
    
    /// Create a complexity issue
    fn create_complexity_issue(&self, func_name: &str, complexity: usize) -> LintIssue {
        LintIssue {
            rule_id: "function_complexity".to_string(),
            category: LintCategory::Complexity,
            severity: LintSeverity::Warning,
            message: format!("Function '{}' has complexity {} which exceeds maximum {}", 
                           func_name, complexity, self.config.max_function_complexity),
            file_path: self.context.current_file.clone(),
            line: 1,
            column: 1,
            length: func_name.len(),
            suggestion: Some("Consider breaking this function into smaller functions".to_string()),
            context: HashMap::new(),
        }
    }
    
    /// Check return type patterns
    fn check_return_type(&self, return_type: &Type, func_name: &str) -> Option<LintIssue> {
        if self.config.enforce_error_handling {
            if let Type::String = return_type {
                if !func_name.contains("error") && !func_name.contains("message") {
                    return Some(LintIssue {
                        rule_id: "return_type_pattern".to_string(),
                        category: LintCategory::Correctness,
                        severity: LintSeverity::Hint,
                        message: "Consider using Result type for error handling".to_string(),
                        file_path: self.context.current_file.clone(),
                        line: 1,
                        column: 1,
                        length: func_name.len(),
                        suggestion: Some("Use Result<T, E> for better error handling".to_string()),
                        context: HashMap::new(),
                    });
                }
            }
        }
        None
    }
    
    /// Check error handling patterns
    fn check_error_handling(&self, statements: &[Statement], func_name: &str) -> Option<LintIssue> {
        let has_error_handling = statements.iter().any(|stmt| {
            matches!(stmt, Statement::Try(_) | Statement::Throw(_))
        });
        
        if !has_error_handling && statements.len() > 5 {
            return Some(LintIssue {
                rule_id: "error_handling".to_string(),
                category: LintCategory::Correctness,
                severity: LintSeverity::Warning,
                message: format!("Function '{}' should handle errors properly", func_name),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: func_name.len(),
                suggestion: Some("Add try-catch blocks or use Result types".to_string()),
                context: HashMap::new(),
            });
        }
        
        None
    }
    
    /// Create type annotation issue
    fn create_type_annotation_issue(&self, var_name: &str) -> LintIssue {
        LintIssue {
            rule_id: "type_annotation".to_string(),
            category: LintCategory::Style,
            severity: LintSeverity::Info,
            message: format!("Variable '{}' should have explicit type annotation", var_name),
            file_path: self.context.current_file.clone(),
            line: 1,
            column: 1,
            length: var_name.len(),
            suggestion: Some("Add explicit type annotation".to_string()),
            context: HashMap::new(),
        }
    }
    
    /// Check for variable shadowing
    fn check_variable_shadowing(&self, var_name: &str) -> Option<LintIssue> {
        // Simplified shadowing check
        if self.context.scope_depth > 0 && self.context.variable_usage.contains_key(var_name) {
            return Some(LintIssue {
                rule_id: "variable_shadowing".to_string(),
                category: LintCategory::Correctness,
                severity: LintSeverity::Warning,
                message: format!("Variable '{}' shadows another variable", var_name),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: var_name.len(),
                suggestion: Some("Use a different variable name".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Check conditional patterns
    fn check_conditional_patterns(&self, condition: &Expression) -> Option<LintIssue> {
        // Check for always true/false conditions
        if let Expression::Literal(Literal::Boolean(value)) = condition {
            return Some(LintIssue {
                rule_id: "constant_condition".to_string(),
                category: LintCategory::Correctness,
                severity: LintSeverity::Warning,
                message: format!("Condition is always {}", value),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 5,
                suggestion: Some("Remove redundant condition".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Create empty branch issue
    fn create_empty_branch_issue(&self, branch_type: &str) -> LintIssue {
        LintIssue {
            rule_id: "empty_branch".to_string(),
            category: LintCategory::Style,
            severity: LintSeverity::Info,
            message: format!("Empty {} branch", branch_type),
            file_path: self.context.current_file.clone(),
            line: 1,
            column: 1,
            length: 0,
            suggestion: Some("Add statements or remove empty branch".to_string()),
            context: HashMap::new(),
        }
    }
    
    /// Check for infinite loop patterns
    fn check_infinite_loop_patterns(&self, condition: &Expression) -> Option<LintIssue> {
        if let Expression::Literal(Literal::Boolean(true)) = condition {
            return Some(LintIssue {
                rule_id: "infinite_loop".to_string(),
                category: LintCategory::Correctness,
                severity: LintSeverity::Warning,
                message: "Potential infinite loop detected".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 4,
                suggestion: Some("Ensure loop has proper exit condition".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Create empty loop issue
    fn create_empty_loop_issue(&self, loop_type: &str) -> LintIssue {
        LintIssue {
            rule_id: "empty_loop".to_string(),
            category: LintCategory::Style,
            severity: LintSeverity::Info,
            message: format!("Empty {} loop", loop_type),
            file_path: self.context.current_file.clone(),
            line: 1,
            column: 1,
            length: 0,
            suggestion: Some("Add statements to loop body".to_string()),
            context: HashMap::new(),
        }
    }
    
    /// Check expression statements
    fn check_expression_statement(&self, expr: &Expression) -> Option<LintIssue> {
        // Check for expressions that might be missing assignments
        if let Expression::Call(_) = expr {
            // Function calls in statements are usually okay
            return None;
        }
        
        if let Expression::Binary(binary) = expr {
            if matches!(binary.operator, BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide) {
                return Some(LintIssue {
                    rule_id: "unused_expression".to_string(),
                    category: LintCategory::Correctness,
                    severity: LintSeverity::Warning,
                    message: "Expression result is unused".to_string(),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: 0,
                    suggestion: Some("Assign result to a variable or remove expression".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        
        None
    }
    
    /// Check if in loop context
    fn is_in_loop_context(&self) -> bool {
        // Simplified check - in a real implementation, we'd track loop context
        true
    }
    
    /// Create misplaced control flow issue
    fn create_misplaced_control_flow_issue(&self, statement: &Statement) -> LintIssue {
        let keyword = match statement {
            Statement::Break => "break",
            Statement::Continue => "continue",
            _ => "control flow",
        };
        
        LintIssue {
            rule_id: "misplaced_control_flow".to_string(),
            category: LintCategory::Correctness,
            severity: LintSeverity::Error,
            message: format!("'{}' statement outside of loop", keyword),
            file_path: self.context.current_file.clone(),
            line: 1,
            column: 1,
            length: keyword.len(),
            suggestion: Some("Move statement inside a loop".to_string()),
            context: HashMap::new(),
        }
    }
    
    /// Check import patterns
    fn check_import_patterns(&self, module: &str) -> Option<LintIssue> {
        if module.contains("..") {
            return Some(LintIssue {
                rule_id: "import_pattern".to_string(),
                category: LintCategory::Security,
                severity: LintSeverity::Warning,
                message: "Relative imports with '..' can be risky".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: module.len(),
                suggestion: Some("Use absolute imports when possible".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Check export patterns
    fn check_export_patterns(&self, name: &str) -> Option<LintIssue> {
        if name.starts_with('_') {
            return Some(LintIssue {
                rule_id: "export_pattern".to_string(),
                category: LintCategory::Style,
                severity: LintSeverity::Warning,
                message: "Exporting private-style identifier".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: name.len(),
                suggestion: Some("Consider using public naming convention".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Check throw patterns
    fn check_throw_patterns(&self, expr: &Expression) -> Option<LintIssue> {
        if let Expression::Literal(Literal::String(msg)) = expr {
            if msg.len() < 5 {
                return Some(LintIssue {
                    rule_id: "throw_pattern".to_string(),
                    category: LintCategory::Correctness,
                    severity: LintSeverity::Info,
                    message: "Error message is very short".to_string(),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: msg.len(),
                    suggestion: Some("Provide more descriptive error message".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        None
    }
    
    /// Check try-catch patterns
    fn check_try_catch_patterns(&self, try_stmt: &crate::ast::TryStatement) -> Option<LintIssue> {
        if try_stmt.body.is_empty() {
            return Some(LintIssue {
                rule_id: "try_catch_pattern".to_string(),
                category: LintCategory::Style,
                severity: LintSeverity::Warning,
                message: "Empty try block".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 0,
                suggestion: Some("Add statements to try block or remove it".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Check if identifier is builtin
    fn is_builtin_identifier(&self, name: &str) -> bool {
        matches!(name, 
            "true" | "false" | "null" | "undefined" | "console" | "print" | "println" |
            "len" | "push" | "pop" | "map" | "filter" | "reduce" | "forEach" |
            "String" | "Number" | "Boolean" | "Array" | "Object" | "Function" |
            "slay" | "yeet" | "facts" | "sus" | "periodt" | "bestie" | "vibez"
        )
    }
    
    /// Create undefined variable issue
    fn create_undefined_variable_issue(&self, name: &str) -> LintIssue {
        LintIssue {
            rule_id: "undefined_variable".to_string(),
            category: LintCategory::Correctness,
            severity: LintSeverity::Error,
            message: format!("Undefined variable '{}'", name),
            file_path: self.context.current_file.clone(),
            line: 1,
            column: 1,
            length: name.len(),
            suggestion: Some("Declare variable before use".to_string()),
            context: HashMap::new(),
        }
    }
    
    /// Check literal patterns
    fn check_literal_patterns(&self, literal: &Literal) -> Option<LintIssue> {
        match literal {
            Literal::String(s) => {
                if s.len() > 1000 {
                    return Some(LintIssue {
                        rule_id: "large_literal".to_string(),
                        category: LintCategory::Performance,
                        severity: LintSeverity::Hint,
                        message: "Very large string literal".to_string(),
                        file_path: self.context.current_file.clone(),
                        line: 1,
                        column: 1,
                        length: s.len(),
                        suggestion: Some("Consider loading from file or splitting".to_string()),
                        context: HashMap::new(),
                    });
                }
            },
            Literal::Number(n) => {
                if n > &1000000.0 {
                    return Some(LintIssue {
                        rule_id: "large_number".to_string(),
                        category: LintCategory::Style,
                        severity: LintSeverity::Hint,
                        message: "Very large number literal".to_string(),
                        file_path: self.context.current_file.clone(),
                        line: 1,
                        column: 1,
                        length: n.to_string().len(),
                        suggestion: Some("Consider using scientific notation or constants".to_string()),
                        context: HashMap::new(),
                    });
                }
            },
            _ => {}
        }
        None
    }
    
    /// Check binary operator patterns
    fn check_binary_operator_patterns(&self, op: &BinaryOperator, left: &Expression, right: &Expression) -> Option<LintIssue> {
        // Check for division by zero
        if matches!(op, BinaryOperator::Divide) {
            if let Expression::Literal(Literal::Number(n)) = right {
                if *n == 0.0 {
                    return Some(LintIssue {
                        rule_id: "division_by_zero".to_string(),
                        category: LintCategory::Correctness,
                        severity: LintSeverity::Error,
                        message: "Division by zero".to_string(),
                        file_path: self.context.current_file.clone(),
                        line: 1,
                        column: 1,
                        length: 1,
                        suggestion: Some("Check for zero before division".to_string()),
                        context: HashMap::new(),
                    });
                }
            }
        }
        
        // Check for redundant comparisons
        if matches!(op, BinaryOperator::Equal | BinaryOperator::NotEqual) {
            if let (Expression::Literal(lit1), Expression::Literal(lit2)) = (left, right) {
                if lit1 == lit2 {
                    return Some(LintIssue {
                        rule_id: "redundant_comparison".to_string(),
                        category: LintCategory::Correctness,
                        severity: LintSeverity::Warning,
                        message: "Redundant comparison with same values".to_string(),
                        file_path: self.context.current_file.clone(),
                        line: 1,
                        column: 1,
                        length: 0,
                        suggestion: Some("Remove redundant comparison".to_string()),
                        context: HashMap::new(),
                    });
                }
            }
        }
        
        None
    }
    
    /// Check unary operator patterns
    fn check_unary_operator_patterns(&self, op: &UnaryOperator, operand: &Expression) -> Option<LintIssue> {
        if matches!(op, UnaryOperator::Not) {
            if let Expression::Literal(Literal::Boolean(_)) = operand {
                return Some(LintIssue {
                    rule_id: "unnecessary_negation".to_string(),
                    category: LintCategory::Style,
                    severity: LintSeverity::Hint,
                    message: "Unnecessary negation of boolean literal".to_string(),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: 0,
                    suggestion: Some("Use the opposite boolean value directly".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        None
    }
    
    /// Check function call patterns
    fn check_function_call_patterns(&self, function: &Expression, args: &[Expression]) -> Option<LintIssue> {
        if let Expression::Identifier(name) = function {
            // Check for deprecated function calls
            if name == "eval" {
                return Some(LintIssue {
                    rule_id: "deprecated_function".to_string(),
                    category: LintCategory::Security,
                    severity: LintSeverity::Error,
                    message: "Use of 'eval' function is dangerous".to_string(),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: name.len(),
                    suggestion: Some("Use safer alternatives".to_string()),
                    context: HashMap::new(),
                });
            }
            
            // Check argument count for known functions
            if name == "print" && args.len() > 5 {
                return Some(LintIssue {
                    rule_id: "too_many_arguments".to_string(),
                    category: LintCategory::Style,
                    severity: LintSeverity::Hint,
                    message: "Too many arguments to print function".to_string(),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: name.len(),
                    suggestion: Some("Consider formatting arguments".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        None
    }
    
    /// Check array patterns
    fn check_array_patterns(&self, elements: &[Expression]) -> Option<LintIssue> {
        if elements.len() > 100 {
            return Some(LintIssue {
                rule_id: "large_array".to_string(),
                category: LintCategory::Performance,
                severity: LintSeverity::Hint,
                message: "Very large array literal".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 0,
                suggestion: Some("Consider loading from file or using iteration".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Check map patterns
    fn check_map_patterns(&self, pairs: &[(Expression, Expression)]) -> Option<LintIssue> {
        if pairs.len() > 50 {
            return Some(LintIssue {
                rule_id: "large_map".to_string(),
                category: LintCategory::Performance,
                severity: LintSeverity::Hint,
                message: "Very large map literal".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 0,
                suggestion: Some("Consider loading from configuration file".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Check index patterns
    fn check_index_patterns(&self, object: &Expression, index: &Expression) -> Option<LintIssue> {
        if let Expression::Literal(Literal::Number(n)) = index {
            if *n < 0.0 {
                return Some(LintIssue {
                    rule_id: "negative_index".to_string(),
                    category: LintCategory::Correctness,
                    severity: LintSeverity::Warning,
                    message: "Negative array index".to_string(),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: 0,
                    suggestion: Some("Use positive index or length-based indexing".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        None
    }
    
    /// Check member access patterns
    fn check_member_access_patterns(&self, object: &Expression, property: &str) -> Option<LintIssue> {
        if property.starts_with('_') {
            return Some(LintIssue {
                rule_id: "private_access".to_string(),
                category: LintCategory::Style,
                severity: LintSeverity::Warning,
                message: "Accessing private-style property".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: property.len(),
                suggestion: Some("Use public API instead".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Check assignment patterns
    fn check_assignment_patterns(&self, left: &Expression, right: &Expression) -> Option<LintIssue> {
        // Check for self-assignment
        if let (Expression::Identifier(left_name), Expression::Identifier(right_name)) = (left, right) {
            if left_name == right_name {
                return Some(LintIssue {
                    rule_id: "self_assignment".to_string(),
                    category: LintCategory::Correctness,
                    severity: LintSeverity::Warning,
                    message: "Self-assignment detected".to_string(),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: left_name.len(),
                    suggestion: Some("Remove redundant assignment".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        None
    }
    
    /// Check lambda patterns
    fn check_lambda_patterns(&self, lambda: &crate::ast::LambdaExpression) -> Option<LintIssue> {
        if lambda.parameters.len() > 10 {
            return Some(LintIssue {
                rule_id: "lambda_complexity".to_string(),
                category: LintCategory::Complexity,
                severity: LintSeverity::Warning,
                message: "Lambda has too many parameters".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 0,
                suggestion: Some("Consider using a regular function".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Check conditional expression patterns
    fn check_conditional_expr_patterns(&self, condition: &Expression, then_expr: &Expression, else_expr: &Expression) -> Option<LintIssue> {
        // Check for redundant conditional
        if let (Expression::Literal(then_lit), Expression::Literal(else_lit)) = (then_expr, else_expr) {
            if then_lit == else_lit {
                return Some(LintIssue {
                    rule_id: "redundant_conditional".to_string(),
                    category: LintCategory::Correctness,
                    severity: LintSeverity::Warning,
                    message: "Conditional expression always returns the same value".to_string(),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: 0,
                    suggestion: Some("Use the constant value directly".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        None
    }
    
    /// Check for unused imports
    fn check_unused_imports(&self) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        // Simplified check - would need more sophisticated analysis
        for import in &self.context.imports {
            if import.starts_with("test_") {
                issues.push(LintIssue {
                    rule_id: "unused_import".to_string(),
                    category: LintCategory::Style,
                    severity: LintSeverity::Warning,
                    message: format!("Import '{}' may be unused", import),
                    file_path: self.context.current_file.clone(),
                    line: 1,
                    column: 1,
                    length: import.len(),
                    suggestion: Some("Remove unused import".to_string()),
                    context: HashMap::new(),
                });
            }
        }
        
        issues
    }
    
    /// Check function naming consistency
    fn check_function_naming_consistency(&self) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        let slang_functions = self.context.functions.values()
            .filter(|f| f.proper_slang_naming)
            .count();
        let total_functions = self.context.functions.len();
        
        if total_functions > 3 && slang_functions < total_functions / 2 {
            issues.push(LintIssue {
                rule_id: "naming_consistency".to_string(),
                category: LintCategory::Style,
                severity: LintSeverity::Info,
                message: "Inconsistent use of Gen Z naming conventions".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 0,
                suggestion: Some("Consider using consistent naming throughout".to_string()),
                context: HashMap::new(),
            });
        }
        
        issues
    }
    
    /// Check slang usage patterns
    fn check_slang_usage_patterns(&self) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        if self.context.slang_usage.improper_patterns > self.context.slang_usage.proper_patterns {
            issues.push(LintIssue {
                rule_id: "slang_usage_pattern".to_string(),
                category: LintCategory::SlangSyntax,
                severity: LintSeverity::Warning,
                message: "More improper slang usage than proper usage detected".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 0,
                suggestion: Some("Review Gen Z slang usage patterns".to_string()),
                context: HashMap::new(),
            });
        }
        
        issues
    }
}

impl Default for CursedLinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of AstVisitor trait for comprehensive AST analysis
impl AstVisitor<Vec<LintIssue>> for CursedLinter {
    fn visit_program(&mut self, program: &Program) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        // Reset context for new program analysis
        self.context.scope_depth = 0;
        self.context.variable_usage.clear();
        self.context.functions.clear();
        self.context.imports.clear();
        self.context.slang_usage = SlangUsageStats::default();
        
        // Collect import statements
        for import in &program.imports {
            self.context.imports.push(import.clone());
        }
        
        // Validate package structure if present
        if let Some(package) = &program.package {
            if let Some(issue) = self.check_package_naming(package) {
                issues.push(issue);
            }
        }
        
        // Visit all statements in the program
        for statement in &program.statements {
            issues.extend(self.visit_statement(statement));
        }
        
        // Perform global analysis after visiting all statements
        issues.extend(self.perform_global_analysis());
        
        issues
    }
    
    fn visit_statement(&mut self, statement: &Statement) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        match statement {
            Statement::Function(func_stmt) => {
                // Check function naming conventions
                if let Some(issue) = self.check_function_naming(&func_stmt.name, 1).unwrap_or(None) {
                    issues.push(issue);
                }
                
                // Check function parameters
                for param in &func_stmt.parameters {
                    if let Some(issue) = self.check_parameter_naming(&param.name, 1) {
                        issues.push(issue);
                    }
                }
                
                // Track function info
                let complexity = self.calculate_function_complexity(&func_stmt.body);
                self.context.functions.insert(func_stmt.name.clone(), FunctionInfo {
                    name: func_stmt.name.clone(),
                    declared_line: 1,
                    param_count: func_stmt.parameters.len(),
                    complexity,
                    proper_slang_naming: self.has_proper_slang_naming(&func_stmt.name),
                });
                
                // Check function complexity
                if complexity > self.config.max_function_complexity {
                    issues.push(self.create_complexity_issue(&func_stmt.name, complexity));
                }
                
                // Check return type if present
                if let Some(return_type) = &func_stmt.return_type {
                    if let Some(issue) = self.check_return_type(return_type, &func_stmt.name) {
                        issues.push(issue);
                    }
                }
                
                // Enter function scope
                self.context.scope_depth += 1;
                
                // Visit function body statements
                for stmt in &func_stmt.body {
                    issues.extend(self.visit_statement(stmt));
                }
                
                // Exit function scope
                self.context.scope_depth -= 1;
                
                // Check for proper error handling
                if self.config.enforce_error_handling {
                    if let Some(issue) = self.check_error_handling(&func_stmt.body, &func_stmt.name) {
                        issues.push(issue);
                    }
                }
            },
            
            Statement::Let(let_stmt) => {
                // Check variable naming
                if let Some(issue) = self.check_variable_naming(&let_stmt.name, 1).unwrap_or(None) {
                    issues.push(issue);
                }
                
                // Track variable declaration
                self.context.variable_usage.insert(let_stmt.name.clone(), VariableInfo {
                    name: let_stmt.name.clone(),
                    declared_line: 1,
                    usage_count: 0,
                    is_mutable: let_stmt.mutable,
                    var_type: let_stmt.type_annotation.clone(),
                });
                
                // Check type annotations
                if self.config.require_explicit_types && let_stmt.type_annotation.is_none() {
                    issues.push(self.create_type_annotation_issue(&let_stmt.name));
                }
                
                // Visit the value expression
                issues.extend(self.visit_expression(&let_stmt.value));
                
                // Check for variable shadowing
                if let Some(issue) = self.check_variable_shadowing(&let_stmt.name) {
                    issues.push(issue);
                }
            },
            
            Statement::If(if_stmt) => {
                // Visit condition expression
                issues.extend(self.visit_expression(&if_stmt.condition));
                
                // Check for proper conditional patterns
                if let Some(issue) = self.check_conditional_patterns(&if_stmt.condition) {
                    issues.push(issue);
                }
                
                // Visit then branch
                self.context.scope_depth += 1;
                for stmt in &if_stmt.then_branch {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
                
                // Visit else branch if present
                if let Some(else_stmts) = &if_stmt.else_branch {
                    self.context.scope_depth += 1;
                    for stmt in else_stmts {
                        issues.extend(self.visit_statement(stmt));
                    }
                    self.context.scope_depth -= 1;
                }
                
                // Check for empty branches
                if if_stmt.then_branch.is_empty() {
                    issues.push(self.create_empty_branch_issue("then"));
                }
                if let Some(else_stmts) = &if_stmt.else_branch {
                    if else_stmts.is_empty() {
                        issues.push(self.create_empty_branch_issue("else"));
                    }
                }
            },
            
            Statement::While(while_stmt) => {
                // Visit condition
                issues.extend(self.visit_expression(&while_stmt.condition));
                
                // Check for infinite loop patterns
                if let Some(issue) = self.check_infinite_loop_patterns(&while_stmt.condition) {
                    issues.push(issue);
                }
                
                // Visit body
                self.context.scope_depth += 1;
                for stmt in &while_stmt.body {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
                
                // Check for empty loop body
                if while_stmt.body.is_empty() {
                    issues.push(self.create_empty_loop_issue("while"));
                }
            },
            
            Statement::For(for_stmt) => {
                // Visit iterator expression
                issues.extend(self.visit_expression(&for_stmt.iterable));
                
                // Track loop variable
                self.context.variable_usage.insert(for_stmt.variable.clone(), VariableInfo {
                    name: for_stmt.variable.clone(),
                    declared_line: 1,
                    usage_count: 1, // Loop variable is implicitly used
                    is_mutable: false,
                    var_type: None,
                });
                
                // Visit body
                self.context.scope_depth += 1;
                for stmt in &for_stmt.body {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
                
                // Check for empty loop body
                if for_stmt.body.is_empty() {
                    issues.push(self.create_empty_loop_issue("for"));
                }
            },
            
            Statement::Return(return_stmt) => {
                // Visit return value if present
                if let Some(expr) = &return_stmt.value {
                    issues.extend(self.visit_expression(expr));
                }
                
                // Check for unreachable code after return
                // This would require more sophisticated analysis
            },
            
            Statement::Expression(expr_stmt) => {
                // Visit the expression
                issues.extend(self.visit_expression(&expr_stmt.expression));
                
                // Check for statement-like expressions that might be missing side effects
                if let Some(issue) = self.check_expression_statement(&expr_stmt.expression) {
                    issues.push(issue);
                }
            },
            
            Statement::Break | Statement::Continue => {
                // Check if break/continue is within a loop
                if !self.is_in_loop_context() {
                    issues.push(self.create_misplaced_control_flow_issue(statement));
                }
            },
            
            Statement::Import(import_stmt) => {
                // Check import naming and patterns
                if let Some(issue) = self.check_import_patterns(&import_stmt.module) {
                    issues.push(issue);
                }
                
                // Track import for dependency analysis
                self.context.imports.push(import_stmt.module.clone());
            },
            
            Statement::Export(export_stmt) => {
                // Check export patterns
                if let Some(issue) = self.check_export_patterns(&export_stmt.name) {
                    issues.push(issue);
                }
            },
            
            Statement::Throw(throw_stmt) => {
                // Visit the thrown expression
                issues.extend(self.visit_expression(&throw_stmt.value));
                
                // Check for proper error throwing patterns
                if let Some(issue) = self.check_throw_patterns(&throw_stmt.value) {
                    issues.push(issue);
                }
            },
            
            Statement::Try(try_stmt) => {
                // Visit try block
                self.context.scope_depth += 1;
                for stmt in &try_stmt.body {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
                
                // Visit catch block if present
                if let Some(catch_clause) = &try_stmt.catch_clause {
                    self.context.scope_depth += 1;
                    for stmt in &catch_clause.body {
                        issues.extend(self.visit_statement(stmt));
                    }
                    self.context.scope_depth -= 1;
                }
                
                // Visit finally block if present
                if let Some(finally_stmts) = &try_stmt.finally_clause {
                    self.context.scope_depth += 1;
                    for stmt in finally_stmts {
                        issues.extend(self.visit_statement(stmt));
                    }
                    self.context.scope_depth -= 1;
                }
                
                // Check for proper error handling patterns
                if let Some(issue) = self.check_try_catch_patterns(try_stmt) {
                    issues.push(issue);
                }
            },
            
            _ => {
                // Handle other statement types that may be added in the future
            }
        }
        
        issues
    }
    
    fn visit_expression(&mut self, expression: &Expression) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        match expression {
            Expression::Identifier(name) => {
                // Track variable usage
                if let Some(var_info) = self.context.variable_usage.get_mut(name) {
                    var_info.usage_count += 1;
                }
                
                // Check for undefined variables
                if !self.context.variable_usage.contains_key(name) && !self.is_builtin_identifier(name) {
                    issues.push(self.create_undefined_variable_issue(name));
                }
            },
            
            Expression::Literal(literal) => {
                // Check literal patterns
                if let Some(issue) = self.check_literal_patterns(literal) {
                    issues.push(issue);
                }
            },
            
            Expression::Binary(binary_expr) => {
                // Visit both operands
                issues.extend(self.visit_expression(&binary_expr.left));
                issues.extend(self.visit_expression(&binary_expr.right));
                
                // Check for operator-specific issues
                if let Some(issue) = self.check_binary_operator_patterns(&binary_expr.operator, &binary_expr.left, &binary_expr.right) {
                    issues.push(issue);
                }
            },
            
            Expression::Unary(unary_expr) => {
                // Visit operand
                issues.extend(self.visit_expression(&unary_expr.operand));
                
                // Check unary operator patterns
                if let Some(issue) = self.check_unary_operator_patterns(&unary_expr.operator, &unary_expr.operand) {
                    issues.push(issue);
                }
            },
            
            Expression::Call(call_expr) => {
                // Visit function expression
                issues.extend(self.visit_expression(&call_expr.function));
                
                // Visit all arguments
                for arg in &call_expr.arguments {
                    issues.extend(self.visit_expression(arg));
                }
                
                // Check function call patterns
                if let Some(issue) = self.check_function_call_patterns(&call_expr.function, &call_expr.arguments) {
                    issues.push(issue);
                }
            },
            
            Expression::Array(elements) => {
                // Visit all elements
                for element in elements {
                    issues.extend(self.visit_expression(element));
                }
                
                // Check array patterns
                if let Some(issue) = self.check_array_patterns(elements) {
                    issues.push(issue);
                }
            },
            
            Expression::Map(pairs) => {
                // Visit all key-value pairs
                for (key, value) in pairs {
                    issues.extend(self.visit_expression(key));
                    issues.extend(self.visit_expression(value));
                }
                
                // Check map patterns
                if let Some(issue) = self.check_map_patterns(pairs) {
                    issues.push(issue);
                }
            },
            
            Expression::Index(index_expr) => {
                // Visit object and index expressions
                issues.extend(self.visit_expression(&index_expr.object));
                issues.extend(self.visit_expression(&index_expr.index));
                
                // Check indexing patterns
                if let Some(issue) = self.check_index_patterns(&index_expr.object, &index_expr.index) {
                    issues.push(issue);
                }
            },
            
            Expression::Member(member_expr) => {
                // Visit object expression
                issues.extend(self.visit_expression(&member_expr.object));
                
                // Check member access patterns
                if let Some(issue) = self.check_member_access_patterns(&member_expr.object, &member_expr.property) {
                    issues.push(issue);
                }
            },
            
            Expression::Assign(assign_expr) => {
                // Visit left and right sides
                issues.extend(self.visit_expression(&assign_expr.left));
                issues.extend(self.visit_expression(&assign_expr.right));
                
                // Check assignment patterns
                if let Some(issue) = self.check_assignment_patterns(&assign_expr.left, &assign_expr.right) {
                    issues.push(issue);
                }
            },
            
            Expression::Lambda(lambda_expr) => {
                // Track lambda parameters
                self.context.scope_depth += 1;
                for param in &lambda_expr.parameters {
                    self.context.variable_usage.insert(param.name.clone(), VariableInfo {
                        name: param.name.clone(),
                        declared_line: 1,
                        usage_count: 0,
                        is_mutable: false,
                        var_type: param.type_annotation.clone(),
                    });
                }
                
                // Visit lambda body
                issues.extend(self.visit_expression(&lambda_expr.body));
                
                self.context.scope_depth -= 1;
                
                // Check lambda patterns
                if let Some(issue) = self.check_lambda_patterns(lambda_expr) {
                    issues.push(issue);
                }
            },
            
            Expression::Conditional(cond_expr) => {
                // Visit all parts
                issues.extend(self.visit_expression(&cond_expr.condition));
                issues.extend(self.visit_expression(&cond_expr.then_expr));
                issues.extend(self.visit_expression(&cond_expr.else_expr));
                
                // Check conditional expression patterns
                if let Some(issue) = self.check_conditional_expr_patterns(&cond_expr.condition, &cond_expr.then_expr, &cond_expr.else_expr) {
                    issues.push(issue);
                }
            },
            
            _ => {
                // Handle other expression types
            }
        }
        
        issues
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
