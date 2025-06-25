/// Linter for CURSED Programming Language
/// 
/// This linter enforces CURSED language style guidelines, Gen Z slang usage,
/// and detects common programming anti-patterns while respecting the language's
/// unique characteristics.

use crate::error::CursedError;
use crate::ast::{Node, Program, Statement, Expression};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use std::collections::{HashMap, HashSet};

/// Lint severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum LintSeverity {
    CursedError,      // Code that will not compile or is fundamentally broken
    Warning,    // Code that compiles but violates style or best practices  
    Suggestion, // Minor style improvements
    Info,       // Informational messages
/// Lint categories for organizing different types of rules
#[derive(Debug, Clone, PartialEq)]
pub enum LintCategory {
    Style,          // Code style and formatting
    Naming,         // Identifier naming conventions
    GenZSlang,      // Proper Gen Z slang keyword usage
    Performance,    // Performance-related issues
    Complexity,     // Code complexity and maintainability
    Deprecated,     // Usage of deprecated features
    Security,       // Security-related issues
    Correctness,    // Logical correctness issues
/// A single lint result with detailed information
#[derive(Debug, Clone)]
pub struct LintResult {
impl LintResult {
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    pub fn with_help(mut self, help_text: String) -> Self {
        self.help_text = Some(help_text);
        self
    pub fn with_range(mut self, end_line: usize, end_column: usize) -> Self {
        self.end_line = Some(end_line);
        self.end_column = Some(end_column);
        self
    }
}

/// Configuration for the CURSED linter
#[derive(Debug, Clone)]
pub struct LinterConfig {
impl Default for LinterConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Main CURSED linter implementation
pub struct CursedLinter {
impl CursedLinter {
    pub fn new(config: LinterConfig) -> Self {
        Self {
        }
    }
    
    /// Main linting function - analyzes CURSED source code
    pub fn lint(&mut self, source: &str) -> crate::error::Result<()> {
        self.results.clear();
        self.source_lines = source.split("\n").map(|s| s.to_string()).collect();
        
        // Clear analysis state
        self.declared_variables.clear();
        self.used_variables.clear();
        self.declared_functions.clear();
        self.imported_modules.clear();
        self.used_imports.clear();

        // First pass: Lexical analysis and basic style checks
        self.analyze_lexical_issues(source)?;
        
        // Second pass: Parse and analyze AST
        match self.parse_and_analyze(source) {
            Err(e) => {
                // Add parse error as lint result
                self.add_result(LintResult::new(
                ));
            }
        }

        // Third pass: Cross-reference analysis
        self.analyze_usage_patterns();

        // Sort results by line number for consistent output
        self.results.sort_by(|a, b| {
            a.line.cmp(&b.line).then_with(|| a.column.cmp(&b.column))
        });

        Ok(self.results.clone())
    /// Analyze lexical issues (comments, string literals, line length, etc.)
    fn analyze_lexical_issues(&mut self, source: &str) -> crate::error::Result<()> {
        let mut lexer = Lexer::new(source.to_string());
        let mut line_number = 1;
        let mut column = 1;
        
        // Check line length issues
        for (line_idx, line) in self.source_lines.iter().enumerate() {
            if line.len() > self.config.max_line_length {
                self.add_result(LintResult::new(
                ).with_suggestion("Consider breaking this line into multiple lines".to_string()));
            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                self.add_result(LintResult::new(
                ).with_suggestion("Remove trailing whitespace".to_string()));
            // Check for mixed tabs and spaces
            if line.contains('\t') && line.contains("  ") {
                self.add_result(LintResult::new(
                ).with_suggestion("Use consistent indentation (either tabs or spaces)".to_string()));
            }
        }

        // Analyze tokens for keyword usage and style
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == TokenType::Eof {
                        break;
                    self.analyze_token(&token, line_number, column);
                    
                    // Update position tracking (simplified)
                    if token.literal.contains('\n') {
                        line_number += token.literal.matches('\n').count();
                        column = 1;
                    } else {
                        column += token.literal.len();
                    }
                }
                Err(e) => {
                    self.add_result(LintResult::new(
                    ));
                    break;
                }
            }
        Ok(())
    /// Analyze individual tokens for style and correctness
    fn analyze_token(&mut self, token: &Token, line: usize, column: usize) {
        match &token.token_type {
            // Check for proper Gen Z slang usage
            TokenType::Identifier => {
                self.check_identifier_naming(&token.literal, line, column);
            // Check comment styles
            TokenType::Comment => {
                self.check_comment_style(&token.literal, line, column);
            // Check string literal styles
            TokenType::String => {
                self.check_string_literal_style(&token.literal, line, column);
            // Check for deprecated patterns
            _ => {
                self.check_deprecated_usage(token, line, column);
            }
        }
    /// Check identifier naming conventions
    fn check_identifier_naming(&mut self, name: &str, line: usize, column: usize) {
        if !self.config.enforce_naming_conventions {
            return;
        // Check for non-ASCII characters (discouraged but allowed)
        if !name.chars().all(|c| c.is_ascii() || c.is_alphanumeric() || c == '_') {
            self.add_result(LintResult::new(
            ).with_help("Consider using ASCII-only identifiers for better compatibility".to_string()));
        // Check for Go-style naming instead of CURSED style
        self.check_for_go_style_naming(name, line, column);
        
        // Check for overly long identifiers
        if name.len() > 50 {
            self.add_result(LintResult::new(
            ).with_suggestion("Consider using a shorter, more concise name".to_string()));
        // Check for single-letter variables (except common ones like i, j, k)
        if name.len() == 1 && !["i", "j", "k", "x", "y", "z"].contains(&name) {
            self.add_result(LintResult::new(
            ).with_suggestion("Consider using a more descriptive variable name".to_string()));
        }
    }

    /// Check for Go-style naming that should use CURSED equivalents
    fn check_for_go_style_naming(&mut self, name: &str, line: usize, column: usize) {
        let go_to_cursed = [
        ];

        for (go_word, cursed_word) in &go_to_cursed {
            if name == *go_word {
                self.add_result(LintResult::new(
                ).with_suggestion(format!("Replace '{}' with '{}'", go_word, cursed_word)));
            }
        }
    /// Check comment style and content
    fn check_comment_style(&mut self, comment: &str, line: usize, column: usize) {
        // Check for proper CURSED comment syntax
        if comment.starts_with("//") {
            self.add_result(LintResult::new(
                "Use CURSED comment syntax 'fr fr' instead of '//'".to_string(),
            ).with_suggestion("Replace '//' with 'fr fr'".to_string()));
        if comment.starts_with("/*") && comment.ends_with("*/") {
            self.add_result(LintResult::new(
                "Use CURSED block comment syntax 'no cap' ... 'on god' instead of /* ... */".to_string(),
            ).with_suggestion("Replace '/* */' with 'no cap' ... 'on god'".to_string()));
        // Check comment length and content quality
        if comment.len() > 120 {
            self.add_result(LintResult::new(
            ));
        }
    }

    /// Check string literal style
    fn check_string_literal_style(&mut self, literal: &str, line: usize, column: usize) {
        // Check for unnecessarily escaped quotes
        if literal.contains("\\\"") && !literal.contains("\"") {
            self.add_result(LintResult::new(
            ).with_suggestion("Remove unnecessary escape sequences".to_string()));
        // Check for very long string literals
        if literal.len() > 200 {
            self.add_result(LintResult::new(
            ));
        }
    }

    /// Check for deprecated usage patterns
    fn check_deprecated_usage(&mut self, token: &Token, line: usize, column: usize) {
        // This can be extended to check for specific deprecated patterns
        // For now, it's a placeholder for future deprecated features
        
        // Example: Check for old-style syntax that might be deprecated
        if token.literal == "chan" {
            self.add_result(LintResult::new(
            ).with_suggestion("Replace 'chan' with 'dm'".to_string()));
        }
    }

    /// Parse source code and analyze AST structure
    fn parse_and_analyze(&mut self, source: &str) -> crate::error::Result<()> {
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        // Analyze the parsed program
        self.analyze_program(&program);

        Ok(())
    /// Analyze the complete program structure
    fn analyze_program(&mut self, program: &Program) {
        // Check package declaration
        if let Some(package_name) = &program.package_name {
            self.check_package_naming(package_name);
        // Analyze imports
        for import in &program.imports {
            self.analyze_import(import);
        // Analyze statements
        for statement in &program.statements {
            self.analyze_statement(statement.as_ref());
        }
    }

    /// Check package naming conventions
    fn check_package_naming(&mut self, package_name: &str) {
        if package_name.is_empty() {
            self.add_result(LintResult::new(
            ));
        if !package_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            self.add_result(LintResult::new(
            ).with_help("Package names should only contain letters, numbers, and underscores".to_string()));
        if package_name.starts_with(char::is_numeric) {
            self.add_result(LintResult::new(
            ));
        }
    }

    /// Analyze import statements
    fn analyze_import(&mut self, import: &crate::ast::ImportStatement) {
        self.imported_modules.insert(import.path.clone());
        
        // Check for duplicate imports (simplified check)
        // In a real implementation, this would track actual duplicate imports
        
        // Check import path format
        if import.path.is_empty() {
            self.add_result(LintResult::new(
                1, // Would need actual line number from AST
            ));
        }
    }

    /// Analyze individual statements
    fn analyze_statement(&mut self, statement: &dyn Statement) {
        // This is a simplified implementation. In a real linter, you would
        // use visitor pattern or match on specific statement types
        
        let statement_str = statement.string();
        
        // Check for function declarations
        if statement_str.starts_with("slay ") {
            self.analyze_function_declaration(&statement_str);
        // Check for variable declarations
        if statement_str.starts_with("sus ") || statement_str.starts_with("facts ") {
            self.analyze_variable_declaration(&statement_str);
        // Check for complexity issues
        self.check_statement_complexity(&statement_str);
    /// Analyze function declarations for various issues
    fn analyze_function_declaration(&mut self, function_str: &str) {
        // Extract function name (simplified parsing)
        if let Some(name_start) = function_str.find("slay ") {
            let after_slay = &function_str[name_start + 5..];
            if let Some(paren_pos) = after_slay.find('(') {
                let function_name = after_slay[..paren_pos].trim();
                self.declared_functions.insert(function_name.to_string());
                
                // Check function naming
                self.check_function_naming(function_name);
                
                // Check parameter count (simplified)
                let params_section = &after_slay[paren_pos..];
                let param_count = self.count_parameters(params_section);
                if param_count > self.config.max_function_parameters {
                    self.add_result(LintResult::new(
                        1, // Would need actual line number
                    ).with_suggestion("Consider grouping related parameters into a struct".to_string()));
                }
            }
        }
    }

    /// Check function naming conventions
    fn check_function_naming(&mut self, function_name: &str) {
        // Check for camelCase vs snake_case
        if function_name.contains('_') && function_name.chars().any(|c| c.is_uppercase()) {
            self.add_result(LintResult::new(
            ).with_suggestion("Choose either camelCase or snake_case consistently".to_string()));
        // Check for overly generic names
        let generic_names = ["doSomething", "handle", "process", "execute", "run", "func"];
        if generic_names.contains(&function_name) {
            self.add_result(LintResult::new(
            ).with_suggestion("Choose a more descriptive function name".to_string()));
        }
    }

    /// Count function parameters (simplified implementation)
    fn count_parameters(&self, params_section: &str) -> usize {
        if let Some(close_paren) = params_section.find(')') {
            let params = &params_section[1..close_paren];
            if params.trim().is_empty() {
                0
            } else {
                params.split(',').count()
            }
        } else {
            0
        }
    }

    /// Analyze variable declarations
    fn analyze_variable_declaration(&mut self, var_str: &str) {
        // Extract variable name (simplified)
        let parts: Vec<&str> = var_str.split_whitespace().collect();
        if parts.len() >= 2 {
            let var_name = parts[1];
            self.declared_variables.insert(var_name.to_string());
            
            // Check for variable naming issues
            if var_name.len() == 1 && !["i", "j", "k"].contains(&var_name) {
                self.add_result(LintResult::new(
                ).with_suggestion("Use a more descriptive variable name".to_string()));
            }
        }
    /// Check statement complexity
    fn check_statement_complexity(&mut self, statement: &str) {
        // Count nesting levels
        let nesting_level = statement.matches('{').count();
        if nesting_level > 4 {
            self.add_result(LintResult::new(
            ).with_suggestion("Consider extracting nested logic into separate functions".to_string()));
        }
    }

    /// Analyze usage patterns for unused variables, imports, etc.
    fn analyze_usage_patterns(&mut self) {
        if self.config.check_unused_variables {
            for var_name in &self.declared_variables {
                if !self.used_variables.contains(var_name) {
                    self.add_result(LintResult::new(
                    ).with_suggestion("Remove unused variable or prefix with '_' if intentionally unused".to_string()));
                }
            }
        if self.config.check_unused_imports {
            for import_path in &self.imported_modules {
                if !self.used_imports.contains(import_path) {
                    self.add_result(LintResult::new(
                    ).with_suggestion("Remove unused import".to_string()));
                }
            }
        }
    }

    /// Add a lint result if the rule is not disabled
    fn add_result(&mut self, result: LintResult) {
        if !self.config.disabled_rules.contains(&result.rule_id) {
            self.results.push(result);
        }
    }

    /// Get all lint results
    pub fn get_results(&self) -> &[LintResult] {
        &self.results
    /// Get results filtered by severity
    pub fn get_results_by_severity(&self, severity: LintSeverity) -> Vec<&LintResult> {
        self.results.iter().filter(|r| r.severity == severity).collect()
    /// Get results filtered by category
    pub fn get_results_by_category(&self, category: LintCategory) -> Vec<&LintResult> {
        self.results.iter().filter(|r| r.category == category).collect()
    }
}

impl Default for CursedLinter {
    fn default() -> Self {
        Self::new(LinterConfig::default())
    }
}

/// Utility functions for linter configuration
impl LinterConfig {
    /// Create a strict configuration with more aggressive linting
    pub fn strict() -> Self {
        Self {
        }
    }

    /// Create a relaxed configuration for less strict linting
    pub fn relaxed() -> Self {
        Self {
        }
    }

    /// Disable a specific rule
    pub fn disable_rule(&mut self, rule_id: &str) {
        self.disabled_rules.insert(rule_id.to_string());
    /// Enable a custom rule
    pub fn enable_custom_rule(&mut self, rule_id: &str) {
        self.custom_rules.insert(rule_id.to_string(), true);
    }
}
