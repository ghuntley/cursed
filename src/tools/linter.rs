/// Linter for CURSED Programming Language
/// 
/// This linter enforces CURSED language style guidelines, Gen Z slang usage,
/// and detects common programming anti-patterns while respecting the language's
/// unique characteristics.

use crate::error::Error;
use crate::ast::{Node, Program, Statement, Expression};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use std::collections::{HashMap, HashSet};

/// Lint severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum LintSeverity {
    Error,      // Code that will not compile or is fundamentally broken
    Warning,    // Code that compiles but violates style or best practices  
    Suggestion, // Minor style improvements
    Info,       // Informational messages
}

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
}

/// A single lint result with detailed information
#[derive(Debug, Clone)]
pub struct LintResult {
    pub rule_id: String,
    pub severity: LintSeverity,
    pub category: LintCategory,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub end_line: Option<usize>,
    pub end_column: Option<usize>,
    pub suggestion: Option<String>,
    pub help_text: Option<String>,
}

impl LintResult {
    pub fn new(
        rule_id: String,
        severity: LintSeverity,
        category: LintCategory,
        message: String,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            rule_id,
            severity,
            category,
            message,
            line,
            column,
            end_line: None,
            end_column: None,
            suggestion: None,
            help_text: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    pub fn with_help(mut self, help_text: String) -> Self {
        self.help_text = Some(help_text);
        self
    }

    pub fn with_range(mut self, end_line: usize, end_column: usize) -> Self {
        self.end_line = Some(end_line);
        self.end_column = Some(end_column);
        self
    }
}

/// Configuration for the CURSED linter
#[derive(Debug, Clone)]
pub struct LinterConfig {
    pub strict_mode: bool,
    pub max_line_length: usize,
    pub max_function_length: usize,
    pub max_function_parameters: usize,
    pub max_cognitive_complexity: usize,
    pub require_documentation: bool,
    pub enforce_naming_conventions: bool,
    pub check_unused_variables: bool,
    pub check_unused_imports: bool,
    pub disabled_rules: HashSet<String>,
    pub custom_rules: HashMap<String, bool>,
}

impl Default for LinterConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            max_line_length: 100,
            max_function_length: 50,
            max_function_parameters: 6,
            max_cognitive_complexity: 15,
            require_documentation: false,
            enforce_naming_conventions: true,
            check_unused_variables: true,
            check_unused_imports: true,
            disabled_rules: HashSet::new(),
            custom_rules: HashMap::new(),
        }
    }
}

/// Main CURSED linter implementation
pub struct CursedLinter {
    config: LinterConfig,
    results: Vec<LintResult>,
    source_lines: Vec<String>,
    declared_variables: HashSet<String>,
    used_variables: HashSet<String>,
    declared_functions: HashSet<String>,
    imported_modules: HashSet<String>,
    used_imports: HashSet<String>,
}

impl CursedLinter {
    pub fn new(config: LinterConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
            source_lines: Vec::new(),
            declared_variables: HashSet::new(),
            used_variables: HashSet::new(),
            declared_functions: HashSet::new(),
            imported_modules: HashSet::new(),
            used_imports: HashSet::new(),
        }
    }
    
    /// Main linting function - analyzes CURSED source code
    pub fn lint(&mut self, source: &str) -> Result<Vec<LintResult>, Error> {
        self.results.clear();
        self.source_lines = source.lines().map(|s| s.to_string()).collect();
        
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
            Ok(_) => {},
            Err(e) => {
                // Add parse error as lint result
                self.add_result(LintResult::new(
                    "parse_error".to_string(),
                    LintSeverity::Error,
                    LintCategory::Correctness,
                    format!("Parse error: {}", e),
                    1,
                    1,
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
    }

    /// Analyze lexical issues (comments, string literals, line length, etc.)
    fn analyze_lexical_issues(&mut self, source: &str) -> Result<(), Error> {
        let mut lexer = Lexer::new(source.to_string());
        let mut line_number = 1;
        let mut column = 1;
        
        // Check line length issues
        for (line_idx, line) in self.source_lines.iter().enumerate() {
            if line.len() > self.config.max_line_length {
                self.add_result(LintResult::new(
                    "line_too_long".to_string(),
                    LintSeverity::Warning,
                    LintCategory::Style,
                    format!("Line length {} exceeds maximum of {}", line.len(), self.config.max_line_length),
                    line_idx + 1,
                    self.config.max_line_length + 1,
                ).with_suggestion("Consider breaking this line into multiple lines".to_string()));
            }

            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                self.add_result(LintResult::new(
                    "trailing_whitespace".to_string(),
                    LintSeverity::Suggestion,
                    LintCategory::Style,
                    "Line has trailing whitespace".to_string(),
                    line_idx + 1,
                    line.len(),
                ).with_suggestion("Remove trailing whitespace".to_string()));
            }

            // Check for mixed tabs and spaces
            if line.contains('\t') && line.contains("  ") {
                self.add_result(LintResult::new(
                    "mixed_indentation".to_string(),
                    LintSeverity::Warning,
                    LintCategory::Style,
                    "Mixed tabs and spaces for indentation".to_string(),
                    line_idx + 1,
                    1,
                ).with_suggestion("Use consistent indentation (either tabs or spaces)".to_string()));
            }
        }

        // Analyze tokens for keyword usage and style
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == TokenType::Eof {
                        break;
                    }
                    
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
                        "lexer_error".to_string(),
                        LintSeverity::Error,
                        LintCategory::Correctness,
                        format!("Lexer error: {}", e),
                        line_number,
                        column,
                    ));
                    break;
                }
            }
        }

        Ok(())
    }

    /// Analyze individual tokens for style and correctness
    fn analyze_token(&mut self, token: &Token, line: usize, column: usize) {
        match &token.token_type {
            // Check for proper Gen Z slang usage
            TokenType::Identifier => {
                self.check_identifier_naming(&token.literal, line, column);
            }
            
            // Check comment styles
            TokenType::Comment => {
                self.check_comment_style(&token.literal, line, column);
            }
            
            // Check string literal styles
            TokenType::String => {
                self.check_string_literal_style(&token.literal, line, column);
            }
            
            // Check for deprecated patterns
            _ => {
                self.check_deprecated_usage(token, line, column);
            }
        }
    }

    /// Check identifier naming conventions
    fn check_identifier_naming(&mut self, name: &str, line: usize, column: usize) {
        if !self.config.enforce_naming_conventions {
            return;
        }

        // Check for non-ASCII characters (discouraged but allowed)
        if !name.chars().all(|c| c.is_ascii() || c.is_alphanumeric() || c == '_') {
            self.add_result(LintResult::new(
                "non_ascii_identifier".to_string(),
                LintSeverity::Suggestion,
                LintCategory::Style,
                format!("Identifier '{}' contains non-ASCII characters", name),
                line,
                column,
            ).with_help("Consider using ASCII-only identifiers for better compatibility".to_string()));
        }

        // Check for Go-style naming instead of CURSED style
        self.check_for_go_style_naming(name, line, column);
        
        // Check for overly long identifiers
        if name.len() > 50 {
            self.add_result(LintResult::new(
                "identifier_too_long".to_string(),
                LintSeverity::Warning,
                LintCategory::Style,
                format!("Identifier '{}' is very long ({})", name, name.len()),
                line,
                column,
            ).with_suggestion("Consider using a shorter, more concise name".to_string()));
        }

        // Check for single-letter variables (except common ones like i, j, k)
        if name.len() == 1 && !["i", "j", "k", "x", "y", "z"].contains(&name) {
            self.add_result(LintResult::new(
                "single_letter_variable".to_string(),
                LintSeverity::Suggestion,
                LintCategory::Naming,
                format!("Single letter variable '{}' is not descriptive", name),
                line,
                column,
            ).with_suggestion("Consider using a more descriptive variable name".to_string()));
        }
    }

    /// Check for Go-style naming that should use CURSED equivalents
    fn check_for_go_style_naming(&mut self, name: &str, line: usize, column: usize) {
        let go_to_cursed = [
            ("func", "slay"),
            ("var", "sus"),
            ("const", "facts"),
            ("if", "lowkey"),
            ("else", "highkey"),
            ("for", "bestie"),
            ("while", "periodt"),
            ("switch", "vibe_check"),
            ("case", "mood"),
            ("default", "basic"),
            ("return", "yolo"),
            ("break", "ghosted"),
            ("continue", "simp"),
            ("true", "based"),
            ("false", "sus"),
            ("nil", "cap"),
            ("struct", "squad"),
            ("interface", "collab"),
            ("package", "vibe"),
            ("import", "yeet"),
        ];

        for (go_word, cursed_word) in &go_to_cursed {
            if name == *go_word {
                self.add_result(LintResult::new(
                    "go_style_keyword".to_string(),
                    LintSeverity::Error,
                    LintCategory::GenZSlang,
                    format!("Use CURSED keyword '{}' instead of Go keyword '{}'", cursed_word, go_word),
                    line,
                    column,
                ).with_suggestion(format!("Replace '{}' with '{}'", go_word, cursed_word)));
            }
        }
    }

    /// Check comment style and content
    fn check_comment_style(&mut self, comment: &str, line: usize, column: usize) {
        // Check for proper CURSED comment syntax
        if comment.starts_with("//") {
            self.add_result(LintResult::new(
                "go_style_comment".to_string(),
                LintSeverity::Warning,
                LintCategory::GenZSlang,
                "Use CURSED comment syntax 'fr fr' instead of '//'".to_string(),
                line,
                column,
            ).with_suggestion("Replace '//' with 'fr fr'".to_string()));
        }

        if comment.starts_with("/*") && comment.ends_with("*/") {
            self.add_result(LintResult::new(
                "go_style_block_comment".to_string(),
                LintSeverity::Warning,
                LintCategory::GenZSlang,
                "Use CURSED block comment syntax 'no cap' ... 'on god' instead of /* ... */".to_string(),
                line,
                column,
            ).with_suggestion("Replace '/* */' with 'no cap' ... 'on god'".to_string()));
        }

        // Check comment length and content quality
        if comment.len() > 120 {
            self.add_result(LintResult::new(
                "long_comment".to_string(),
                LintSeverity::Suggestion,
                LintCategory::Style,
                format!("Comment is quite long ({}). Consider breaking into multiple lines", comment.len()),
                line,
                column,
            ));
        }
    }

    /// Check string literal style
    fn check_string_literal_style(&mut self, literal: &str, line: usize, column: usize) {
        // Check for unnecessarily escaped quotes
        if literal.contains("\\\"") && !literal.contains("\"") {
            self.add_result(LintResult::new(
                "unnecessary_escape".to_string(),
                LintSeverity::Suggestion,
                LintCategory::Style,
                "Unnecessary escaped quotes in string literal".to_string(),
                line,
                column,
            ).with_suggestion("Remove unnecessary escape sequences".to_string()));
        }

        // Check for very long string literals
        if literal.len() > 200 {
            self.add_result(LintResult::new(
                "long_string_literal".to_string(),
                LintSeverity::Suggestion,
                LintCategory::Style,
                format!("String literal is very long ({}). Consider using a multi-line string or constant", literal.len()),
                line,
                column,
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
                "go_style_channel".to_string(),
                LintSeverity::Warning,
                LintCategory::GenZSlang,
                "Use CURSED channel keyword 'dm' instead of 'chan'".to_string(),
                line,
                column,
            ).with_suggestion("Replace 'chan' with 'dm'".to_string()));
        }
    }

    /// Parse source code and analyze AST structure
    fn parse_and_analyze(&mut self, source: &str) -> Result<(), Error> {
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        // Analyze the parsed program
        self.analyze_program(&program);

        Ok(())
    }

    /// Analyze the complete program structure
    fn analyze_program(&mut self, program: &Program) {
        // Check package declaration
        if let Some(package_name) = &program.package_name {
            self.check_package_naming(package_name);
        }

        // Analyze imports
        for import in &program.imports {
            self.analyze_import(import);
        }

        // Analyze statements
        for statement in &program.statements {
            self.analyze_statement(statement.as_ref());
        }
    }

    /// Check package naming conventions
    fn check_package_naming(&mut self, package_name: &str) {
        if package_name.is_empty() {
            self.add_result(LintResult::new(
                "empty_package_name".to_string(),
                LintSeverity::Error,
                LintCategory::Naming,
                "Package name cannot be empty".to_string(),
                1,
                1,
            ));
        }

        if !package_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            self.add_result(LintResult::new(
                "invalid_package_name".to_string(),
                LintSeverity::Error,
                LintCategory::Naming,
                format!("Package name '{}' contains invalid characters", package_name),
                1,
                1,
            ).with_help("Package names should only contain letters, numbers, and underscores".to_string()));
        }

        if package_name.starts_with(char::is_numeric) {
            self.add_result(LintResult::new(
                "package_name_starts_with_number".to_string(),
                LintSeverity::Error,
                LintCategory::Naming,
                format!("Package name '{}' cannot start with a number", package_name),
                1,
                1,
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
                "empty_import_path".to_string(),
                LintSeverity::Error,
                LintCategory::Correctness,
                "Import path cannot be empty".to_string(),
                1, // Would need actual line number from AST
                1,
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
        }
        
        // Check for variable declarations
        if statement_str.starts_with("sus ") || statement_str.starts_with("facts ") {
            self.analyze_variable_declaration(&statement_str);
        }
        
        // Check for complexity issues
        self.check_statement_complexity(&statement_str);
    }

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
                        "too_many_parameters".to_string(),
                        LintSeverity::Warning,
                        LintCategory::Complexity,
                        format!("Function '{}' has {} parameters, which exceeds the maximum of {}", 
                               function_name, param_count, self.config.max_function_parameters),
                        1, // Would need actual line number
                        1,
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
                "mixed_naming_style".to_string(),
                LintSeverity::Warning,
                LintCategory::Naming,
                format!("Function '{}' mixes camelCase and snake_case", function_name),
                1,
                1,
            ).with_suggestion("Choose either camelCase or snake_case consistently".to_string()));
        }

        // Check for overly generic names
        let generic_names = ["doSomething", "handle", "process", "execute", "run", "func"];
        if generic_names.contains(&function_name) {
            self.add_result(LintResult::new(
                "generic_function_name".to_string(),
                LintSeverity::Suggestion,
                LintCategory::Naming,
                format!("Function name '{}' is too generic", function_name),
                1,
                1,
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
                    "single_letter_variable".to_string(),
                    LintSeverity::Suggestion,
                    LintCategory::Naming,
                    format!("Variable '{}' has a non-descriptive single-letter name", var_name),
                    1,
                    1,
                ).with_suggestion("Use a more descriptive variable name".to_string()));
            }
        }
    }

    /// Check statement complexity
    fn check_statement_complexity(&mut self, statement: &str) {
        // Count nesting levels
        let nesting_level = statement.matches('{').count();
        if nesting_level > 4 {
            self.add_result(LintResult::new(
                "deep_nesting".to_string(),
                LintSeverity::Warning,
                LintCategory::Complexity,
                format!("Statement has deep nesting (level {})", nesting_level),
                1,
                1,
            ).with_suggestion("Consider extracting nested logic into separate functions".to_string()));
        }
    }

    /// Analyze usage patterns for unused variables, imports, etc.
    fn analyze_usage_patterns(&mut self) {
        if self.config.check_unused_variables {
            for var_name in &self.declared_variables {
                if !self.used_variables.contains(var_name) {
                    self.add_result(LintResult::new(
                        "unused_variable".to_string(),
                        LintSeverity::Warning,
                        LintCategory::Correctness,
                        format!("Variable '{}' is declared but never used", var_name),
                        1,
                        1,
                    ).with_suggestion("Remove unused variable or prefix with '_' if intentionally unused".to_string()));
                }
            }
        }

        if self.config.check_unused_imports {
            for import_path in &self.imported_modules {
                if !self.used_imports.contains(import_path) {
                    self.add_result(LintResult::new(
                        "unused_import".to_string(),
                        LintSeverity::Warning,
                        LintCategory::Correctness,
                        format!("Import '{}' is imported but never used", import_path),
                        1,
                        1,
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
    }

    /// Get results filtered by severity
    pub fn get_results_by_severity(&self, severity: LintSeverity) -> Vec<&LintResult> {
        self.results.iter().filter(|r| r.severity == severity).collect()
    }

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
            strict_mode: true,
            max_line_length: 80,
            max_function_length: 30,
            max_function_parameters: 4,
            max_cognitive_complexity: 10,
            require_documentation: true,
            enforce_naming_conventions: true,
            check_unused_variables: true,
            check_unused_imports: true,
            disabled_rules: HashSet::new(),
            custom_rules: HashMap::new(),
        }
    }

    /// Create a relaxed configuration for less strict linting
    pub fn relaxed() -> Self {
        Self {
            strict_mode: false,
            max_line_length: 120,
            max_function_length: 100,
            max_function_parameters: 10,
            max_cognitive_complexity: 25,
            require_documentation: false,
            enforce_naming_conventions: false,
            check_unused_variables: false,
            check_unused_imports: false,
            disabled_rules: HashSet::new(),
            custom_rules: HashMap::new(),
        }
    }

    /// Disable a specific rule
    pub fn disable_rule(&mut self, rule_id: &str) {
        self.disabled_rules.insert(rule_id.to_string());
    }

    /// Enable a custom rule
    pub fn enable_custom_rule(&mut self, rule_id: &str) {
        self.custom_rules.insert(rule_id.to_string(), true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_basic_functionality() {
        let mut linter = CursedLinter::default();
        let source = r#"
vibe test

slay main() {
    sus x = 42
}
"#;
        
        let results = linter.lint(source).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_line_length_check() {
        let mut linter = CursedLinter::new(LinterConfig {
            max_line_length: 10,
            ..LinterConfig::default()
        });
        
        let source = "sus very_long_variable_name = 42";
        let results = linter.lint(source).unwrap();
        
        let long_line_errors: Vec<_> = results.iter()
            .filter(|r| r.rule_id == "line_too_long")
            .collect();
        assert!(!long_line_errors.is_empty());
    }

    #[test]
    fn test_go_style_detection() {
        let mut linter = CursedLinter::default();
        let source = "func main() { return 42; }";
        
        let results = linter.lint(source).unwrap();
        let go_style_errors: Vec<_> = results.iter()
            .filter(|r| r.category == LintCategory::GenZSlang)
            .collect();
        assert!(!go_style_errors.is_empty());
    }

    #[test]
    fn test_disabled_rules() {
        let mut config = LinterConfig::default();
        config.disable_rule("line_too_long");
        
        let mut linter = CursedLinter::new(config);
        let source = "sus very_long_variable_name_that_exceeds_the_maximum_line_length_setting = 42";
        
        let results = linter.lint(source).unwrap();
        let long_line_errors: Vec<_> = results.iter()
            .filter(|r| r.rule_id == "line_too_long")
            .collect();
        assert!(long_line_errors.is_empty());
    }

    #[test]
    fn test_severity_filtering() {
        let mut linter = CursedLinter::default();
        let source = r#"
slay test() {
    sus x = 42 
    sus y = "test"   
}
"#;
        
        let results = linter.lint(source).unwrap();
        linter.results = results;
        
        let errors = linter.get_results_by_severity(LintSeverity::Error);
        let warnings = linter.get_results_by_severity(LintSeverity::Warning);
        let suggestions = linter.get_results_by_severity(LintSeverity::Suggestion);
        
        // Should have some results in each category
        assert!(errors.len() + warnings.len() + suggestions.len() > 0);
    }
}
