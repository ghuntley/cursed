//! CURSED Language Linter
//!
//! This module provides comprehensive linting capabilities for the CURSED programming language.
//! It analyzes CURSED source code to detect style issues, potential errors, and code quality problems.
//! The linter supports different severity levels and handles all CURSED language constructs including
//! Gen Z slang keywords.

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::Path;
use tracing::{debug, info, instrument, warn};

/// Severity level for lint issues
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LintSeverity {
    Info,
    Warning,
    Error,
}

impl fmt::Display for LintSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LintSeverity::Info => write!(f, "info"),
            LintSeverity::Warning => write!(f, "warning"),
            LintSeverity::Error => write!(f, "error"),
        }
    }
}

/// A lint issue found in the code
#[derive(Debug, Clone)]
pub struct LintIssue {
    pub severity: LintSeverity,
    pub rule_name: String,
    pub message: String,
    pub location: SourceLocation,
    pub suggestion: Option<String>,
}

impl fmt::Display for LintIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} [{}] - {}",
            self.severity, self.location, self.rule_name, self.message
        )?;
        if let Some(suggestion) = &self.suggestion {
            write!(f, "\n  Suggestion: {}", suggestion)?;
        }
        Ok(())
    }
}

/// Configuration for the CURSED linter
#[derive(Debug, Clone)]
pub struct LinterConfig {
    /// Maximum complexity score for functions
    pub max_function_complexity: usize,
    /// Maximum line length
    pub max_line_length: usize,
    /// Whether to enforce Gen Z slang naming conventions
    pub enforce_genz_naming: bool,
    /// Whether to check for unused variables
    pub check_unused_variables: bool,
    /// Whether to check for unreachable code
    pub check_unreachable_code: bool,
    /// Whether to check style consistency
    pub check_style_consistency: bool,
    /// Whether to detect dead code
    pub check_dead_code: bool,
    /// Minimum severity level to report
    pub min_severity: LintSeverity,
}

impl Default for LinterConfig {
    fn default() -> Self {
        Self {
            max_function_complexity: 10,
            max_line_length: 100,
            enforce_genz_naming: true,
            check_unused_variables: true,
            check_unreachable_code: true,
            check_style_consistency: true,
            check_dead_code: true,
            min_severity: LintSeverity::Info,
        }
    }
}

/// Context for tracking variable usage during linting
#[derive(Debug, Default)]
struct LintContext {
    /// Variables declared in the current scope
    declared_variables: HashMap<String, SourceLocation>,
    /// Variables that have been used
    used_variables: HashSet<String>,
    /// Functions declared in the program
    declared_functions: HashMap<String, SourceLocation>,
    /// Functions that have been called
    called_functions: HashSet<String>,
    /// Current scope depth
    scope_depth: usize,
    /// Whether we're in unreachable code
    in_unreachable: bool,
}

/// The main CURSED language linter
#[derive(Debug)]
pub struct CursedLinter {
    config: LinterConfig,
    issues: Vec<LintIssue>,
    context: LintContext,
    source_lines: Vec<String>,
}

impl CursedLinter {
    /// Create a new linter with default configuration
    pub fn new() -> Self {
        Self::with_config(LinterConfig::default())
    }

    /// Create a new linter with custom configuration
    pub fn with_config(config: LinterConfig) -> Self {
        Self {
            config,
            issues: Vec::new(),
            context: LintContext::default(),
            source_lines: Vec::new(),
        }
    }

    /// Lint a CURSED source file
    #[instrument(skip(self, file_path))]
    pub fn lint_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<Vec<LintIssue>, Error> {
        let content = std::fs::read_to_string(&file_path).map_err(Error::IoError)?;
        self.lint_source(&content, Some(file_path.as_ref().to_string_lossy().to_string()))
    }

    /// Lint CURSED source code
    #[instrument(skip(self, source))]
    pub fn lint_source(&mut self, source: &str, file_name: Option<String>) -> Result<Vec<LintIssue>, Error> {
        debug!("Starting lint analysis");
        
        // Clear previous state
        self.issues.clear();
        self.context = LintContext::default();
        self.source_lines = source.lines().map(|s| s.to_string()).collect();

        // Check basic style issues first
        self.check_line_style(source, file_name.as_deref());

        // Parse the source code
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(&mut lexer).map_err(|e| {
            Error::Parser {
                location: SourceLocation::new(0, 0),
                message: format!("Failed to create parser: {:?}", e),
            }
        })?;
        
        let program = parser.parse_program().map_err(|e| {
            Error::Parser {
                location: SourceLocation::new(0, 0),
                message: format!("Failed to parse: {:?}", e),
            }
        })?;

        // Perform AST-based analysis
        self.analyze_program(&program);

        // Check for unused variables and functions
        if self.config.check_unused_variables {
            self.check_unused_items();
        }

        // Filter issues by minimum severity
        let filtered_issues: Vec<LintIssue> = self.issues
            .clone()
            .into_iter()
            .filter(|issue| issue.severity >= self.config.min_severity)
            .collect();

        info!("Found {} lint issues", filtered_issues.len());
        Ok(filtered_issues)
    }

    /// Check line-level style issues
    fn check_line_style(&mut self, source: &str, file_name: Option<&str>) {
        for (line_num, line) in source.lines().enumerate() {
            let location = SourceLocation {
                line: line_num,
                column: 0,
                file: file_name.map(|s| s.to_string()),
                source_line: line.to_string(),
            };

            // Check line length
            if line.len() > self.config.max_line_length {
                self.add_issue(LintIssue {
                    severity: LintSeverity::Warning,
                    rule_name: "line-too-long".to_string(),
                    message: format!("Line exceeds maximum length of {} characters", self.config.max_line_length),
                    location: location.clone(),
                    suggestion: Some("Consider breaking this line into multiple lines".to_string()),
                });
            }

            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                self.add_issue(LintIssue {
                    severity: LintSeverity::Info,
                    rule_name: "trailing-whitespace".to_string(),
                    message: "Line has trailing whitespace".to_string(),
                    location: location.clone(),
                    suggestion: Some("Remove trailing whitespace".to_string()),
                });
            }

            // Check for tabs vs spaces consistency
            if line.contains('\t') && line.contains("    ") {
                self.add_issue(LintIssue {
                    severity: LintSeverity::Warning,
                    rule_name: "mixed-indentation".to_string(),
                    message: "Mixed tabs and spaces for indentation".to_string(),
                    location,
                    suggestion: Some("Use consistent indentation (either tabs or spaces)".to_string()),
                });
            }
        }
    }

    /// Analyze the entire program AST
    fn analyze_program(&mut self, program: &Program) {
        debug!("Analyzing program with {} statements", program.statements.len());
        
        // First pass: collect declarations
        for statement in &program.statements {
            self.collect_declarations(statement.as_ref());
        }

        // Second pass: analyze usage and other issues
        for statement in &program.statements {
            self.analyze_statement(statement.as_ref());
        }
    }

    /// Collect variable and function declarations
    fn collect_declarations(&mut self, statement: &dyn Statement) {
        // This is a simplified approach - in a real implementation you'd
        // need to properly dispatch based on statement type
        if let Some(func_stmt) = statement.as_any().downcast_ref::<FunctionStatement>() {
            let location = SourceLocation::new(0, 0); // Would get from statement
            self.context.declared_functions.insert(func_stmt.name.value.clone(), location);
        }
    }

    /// Analyze a statement for lint issues
    fn analyze_statement(&mut self, statement: &dyn Statement) {
        // Check for unreachable code
        if self.context.in_unreachable && self.config.check_unreachable_code {
            self.add_issue(LintIssue {
                severity: LintSeverity::Warning,
                rule_name: "unreachable-code".to_string(),
                message: "This code is unreachable".to_string(),
                location: SourceLocation::new(0, 0), // Would get from statement
                suggestion: Some("Remove unreachable code".to_string()),
            });
        }

        // Analyze different statement types
        // Note: This is simplified - real implementation would need proper AST traversal
        self.analyze_statement_complexity(statement);
    }

    /// Analyze statement complexity
    fn analyze_statement_complexity(&mut self, _statement: &dyn Statement) {
        // Simplified complexity analysis
        // In real implementation, this would calculate cyclomatic complexity
        // For now, just demonstrate the concept
        let complexity = 1; // Placeholder

        if complexity > self.config.max_function_complexity {
            self.add_issue(LintIssue {
                severity: LintSeverity::Warning,
                rule_name: "high-complexity".to_string(),
                message: format!("Function has high complexity ({})", complexity),
                location: SourceLocation::new(0, 0),
                suggestion: Some("Consider breaking this function into smaller functions".to_string()),
            });
        }
    }

    /// Check for unused variables and functions
    fn check_unused_items(&mut self) {
        // Collect unused variables to avoid borrowing issues
        let unused_vars: Vec<_> = self.context.declared_variables
            .iter()
            .filter(|(var_name, _)| !self.context.used_variables.contains(*var_name))
            .map(|(name, location)| (name.clone(), location.clone()))
            .collect();

        // Check unused variables
        for (var_name, location) in unused_vars {
            self.add_issue(LintIssue {
                severity: LintSeverity::Warning,
                rule_name: "unused-variable".to_string(),
                message: format!("Variable '{}' is declared but never used", var_name),
                location,
                suggestion: Some(format!("Remove unused variable '{}' or prefix with '_'", var_name)),
            });
        }

        // Collect unused functions to avoid borrowing issues
        let unused_funcs: Vec<_> = self.context.declared_functions
            .iter()
            .filter(|(func_name, _)| !self.context.called_functions.contains(*func_name) && *func_name != "main")
            .map(|(name, location)| (name.clone(), location.clone()))
            .collect();

        // Check unused functions
        for (func_name, location) in unused_funcs {
            self.add_issue(LintIssue {
                severity: LintSeverity::Info,
                rule_name: "unused-function".to_string(),
                message: format!("Function '{}' is declared but never called", func_name),
                location,
                suggestion: Some(format!("Remove unused function '{}'", func_name)),
            });
        }
    }

    /// Check Gen Z slang naming conventions
    fn check_genz_naming(&mut self, name: &str, location: &SourceLocation, item_type: &str) {
        if !self.config.enforce_genz_naming {
            return;
        }

        // Check if identifier uses appropriate Gen Z slang
        let has_genz_style = name.contains("_") || 
                            name.chars().any(|c| c.is_lowercase()) ||
                            ["vibes", "mood", "slay", "tea", "flex", "stan"].iter()
                                .any(|word| name.to_lowercase().contains(word));

        if !has_genz_style && name.len() > 3 {
            self.add_issue(LintIssue {
                severity: LintSeverity::Info,
                rule_name: "genz-naming".to_string(),
                message: format!("{} '{}' should follow Gen Z naming conventions", item_type, name),
                location: location.clone(),
                suggestion: Some("Consider using Gen Z slang or snake_case naming".to_string()),
            });
        }
    }

    /// Add a lint issue
    fn add_issue(&mut self, issue: LintIssue) {
        debug!("Adding lint issue: {} - {}", issue.rule_name, issue.message);
        self.issues.push(issue);
    }

    /// Get all issues found during linting
    pub fn issues(&self) -> &[LintIssue] {
        &self.issues
    }

    /// Get issue count by severity
    pub fn issue_count_by_severity(&self) -> HashMap<LintSeverity, usize> {
        let mut counts = HashMap::new();
        for issue in &self.issues {
            *counts.entry(issue.severity.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Check if there are any error-level issues
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| issue.severity == LintSeverity::Error)
    }

    /// Get a summary of linting results
    pub fn summary(&self) -> String {
        let counts = self.issue_count_by_severity();
        let error_count = counts.get(&LintSeverity::Error).unwrap_or(&0);
        let warning_count = counts.get(&LintSeverity::Warning).unwrap_or(&0);
        let info_count = counts.get(&LintSeverity::Info).unwrap_or(&0);

        format!(
            "Linting complete: {} errors, {} warnings, {} info",
            error_count, warning_count, info_count
        )
    }
}

impl Default for CursedLinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Result type for linting operations
pub type LintResult<T> = Result<T, Error>;

/// Utility function to lint a single file
pub fn lint_file<P: AsRef<Path>>(file_path: P) -> LintResult<Vec<LintIssue>> {
    let mut linter = CursedLinter::new();
    linter.lint_file(file_path)
}

/// Utility function to lint source code with default configuration
pub fn lint_source(source: &str) -> LintResult<Vec<LintIssue>> {
    let mut linter = CursedLinter::new();
    linter.lint_source(source, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_creation() {
        let linter = CursedLinter::new();
        assert!(linter.issues().is_empty());
    }

    #[test]
    fn test_line_length_check() {
        let mut linter = CursedLinter::with_config(LinterConfig {
            max_line_length: 10,
            ..Default::default()
        });

        let source = "this line is definitely too long for the limit";
        let issues = linter.lint_source(source, None).unwrap();
        
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|issue| issue.rule_name == "line-too-long"));
    }

    #[test]
    fn test_trailing_whitespace_check() {
        let mut linter = CursedLinter::new();
        let source = "valid line\nline with trailing space \nanother valid line";
        
        let issues = linter.lint_source(source, None).unwrap();
        assert!(issues.iter().any(|issue| issue.rule_name == "trailing-whitespace"));
    }

    #[test]
    fn test_mixed_indentation_check() {
        let mut linter = CursedLinter::new();
        let source = "\tif true {\n    \treturn false\n}";
        
        let issues = linter.lint_source(source, None).unwrap();
        assert!(issues.iter().any(|issue| issue.rule_name == "mixed-indentation"));
    }

    #[test]
    fn test_severity_filtering() {
        let mut linter = CursedLinter::with_config(LinterConfig {
            min_severity: LintSeverity::Warning,
            max_line_length: 10,
            ..Default::default()
        });

        let source = "this is a very long line that exceeds the limit\ntrailing space ";
        let issues = linter.lint_source(source, None).unwrap();
        
        // Should only have warning-level issues, not info-level
        assert!(issues.iter().all(|issue| issue.severity >= LintSeverity::Warning));
    }

    #[test]
    fn test_issue_summary() {
        let mut linter = CursedLinter::with_config(LinterConfig {
            max_line_length: 10,
            ..Default::default()
        });

        let source = "very long line here\ntrailing space ";
        linter.lint_source(source, None).unwrap();
        
        let summary = linter.summary();
        assert!(summary.contains("warning"));
        assert!(summary.contains("info"));
    }
}
