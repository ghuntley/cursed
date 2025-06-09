//! Base utilities for implementing lint rules
//!
//! This module provides common functionality and helper utilities
//! for implementing lint rules efficiently and consistently.

use crate::ast::*;
use crate::error::SourceLocation;
use crate::linter::{
    engine::{LintIssue, LintSeverity},
    rules::{RuleCategory, RuleSeverity},
    visitor::AnalysisContext,
    fix::FixSuggestion,
};
use std::collections::HashMap;

/// Base trait for creating lint issues with consistent formatting
pub trait IssueBuilder {
    /// Create a new lint issue with the given parameters
    fn create_issue(
        &self,
        severity: RuleSeverity,
        rule_name: &str,
        category: RuleCategory,
        message: String,
        location: SourceLocation,
    ) -> LintIssue {
        LintIssue::new(
            severity.into(),
            rule_name.to_string(),
            category,
            message,
            location,
        )
    }

    /// Create an issue with a fix suggestion
    fn create_issue_with_fix(
        &self,
        severity: RuleSeverity,
        rule_name: &str,
        category: RuleCategory,
        message: String,
        location: SourceLocation,
        fix: FixSuggestion,
    ) -> LintIssue {
        self.create_issue(severity, rule_name, category, message, location)
            .with_suggestion(fix)
    }

    /// Create an issue with context information
    fn create_issue_with_context(
        &self,
        severity: RuleSeverity,
        rule_name: &str,
        category: RuleCategory,
        message: String,
        location: SourceLocation,
        context: HashMap<String, String>,
    ) -> LintIssue {
        let mut issue = self.create_issue(severity, rule_name, category, message, location);
        for (key, value) in context {
            issue = issue.with_context(key, value);
        }
        issue
    }
}

/// Helper for location tracking in source code
pub struct LocationHelper<'a> {
    source_lines: &'a [String],
    file_name: Option<String>,
}

impl<'a> LocationHelper<'a> {
    pub fn new(source_lines: &'a [String], file_name: Option<String>) -> Self {
        Self {
            source_lines,
            file_name,
        }
    }

    /// Create a source location for a given line and column
    pub fn location(&self, line: usize, column: usize) -> SourceLocation {
        let source_line = self.source_lines
            .get(line)
            .cloned()
            .unwrap_or_else(|| String::new());

        SourceLocation {
            line,
            column,
            file: self.file_name.clone(),
            source_line,
        }
    }

    /// Create a source location for a token position
    pub fn location_from_token(&self, token: &crate::lexer::Token) -> SourceLocation {
        // For now, use a default location. In a full implementation,
        // we would need position tracking in the lexer
        self.location(1, 1)
    }

    /// Get the source line at a given line number
    pub fn get_line(&self, line: usize) -> Option<&String> {
        self.source_lines.get(line)
    }

    /// Check if a line exists
    pub fn has_line(&self, line: usize) -> bool {
        line < self.source_lines.len()
    }
}

/// Utility for analyzing identifier names
pub struct IdentifierAnalyzer;

impl IdentifierAnalyzer {
    /// Check if an identifier follows camelCase convention
    pub fn is_camel_case(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_lowercase() {
            return false;
        }

        // Should not contain underscores
        if name.contains('_') {
            return false;
        }

        // Should contain at least one uppercase letter after the first
        name.chars().skip(1).any(|c| c.is_ascii_uppercase())
    }

    /// Check if an identifier follows snake_case convention
    pub fn is_snake_case(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        // Should be all lowercase with underscores
        name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            && !name.starts_with('_')
            && !name.ends_with('_')
            && !name.contains("__")
    }

    /// Check if an identifier follows PascalCase convention
    pub fn is_pascal_case(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_uppercase() {
            return false;
        }

        // Should not contain underscores
        !name.contains('_')
    }

    /// Check if an identifier follows SCREAMING_SNAKE_CASE convention
    pub fn is_screaming_snake_case(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        // Should be all uppercase with underscores
        name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            && !name.starts_with('_')
            && !name.ends_with('_')
            && !name.contains("__")
    }

    /// Get suggested naming convention for an identifier
    pub fn suggest_naming_convention(name: &str, target_convention: NamingConvention) -> String {
        match target_convention {
            NamingConvention::CamelCase => Self::to_camel_case(name),
            NamingConvention::SnakeCase => Self::to_snake_case(name),
            NamingConvention::PascalCase => Self::to_pascal_case(name),
            NamingConvention::ScreamingSnakeCase => Self::to_screaming_snake_case(name),
        }
    }

    /// Convert identifier to camelCase
    pub fn to_camel_case(name: &str) -> String {
        let words = Self::split_identifier(name);
        if words.is_empty() {
            return String::new();
        }

        let mut result = words[0].to_lowercase();
        for word in &words[1..] {
            result.push_str(&Self::capitalize_first(word));
        }
        result
    }

    /// Convert identifier to snake_case
    pub fn to_snake_case(name: &str) -> String {
        let words = Self::split_identifier(name);
        words.iter()
            .map(|w| w.to_lowercase())
            .collect::<Vec<_>>()
            .join("_")
    }

    /// Convert identifier to PascalCase
    pub fn to_pascal_case(name: &str) -> String {
        let words = Self::split_identifier(name);
        words.iter()
            .map(|w| Self::capitalize_first(w))
            .collect::<Vec<_>>()
            .join("")
    }

    /// Convert identifier to SCREAMING_SNAKE_CASE
    pub fn to_screaming_snake_case(name: &str) -> String {
        let words = Self::split_identifier(name);
        words.iter()
            .map(|w| w.to_uppercase())
            .collect::<Vec<_>>()
            .join("_")
    }

    /// Split an identifier into words
    fn split_identifier(name: &str) -> Vec<String> {
        let mut words = Vec::new();
        let mut current_word = String::new();

        for (i, c) in name.chars().enumerate() {
            if c == '_' {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                    current_word.clear();
                }
            } else if c.is_ascii_uppercase() && i > 0 && !current_word.is_empty() {
                words.push(current_word.clone());
                current_word.clear();
                current_word.push(c);
            } else {
                current_word.push(c);
            }
        }

        if !current_word.is_empty() {
            words.push(current_word);
        }

        words
    }

    /// Capitalize the first letter of a word
    fn capitalize_first(word: &str) -> String {
        if word.is_empty() {
            return String::new();
        }

        let mut chars: Vec<char> = word.chars().collect();
        chars[0] = chars[0].to_ascii_uppercase();
        chars.iter().collect()
    }
}

/// Naming convention types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamingConvention {
    CamelCase,
    SnakeCase,
    PascalCase,
    ScreamingSnakeCase,
}

/// Utility for analyzing code complexity
pub struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    /// Calculate cyclomatic complexity for a function
    pub fn cyclomatic_complexity(statements: &[Box<dyn Statement>]) -> usize {
        let mut complexity = 1; // Base complexity

        for statement in statements {
            complexity += Self::statement_complexity(statement.as_ref());
        }

        complexity
    }

    /// Calculate complexity contribution of a single statement
    fn statement_complexity(statement: &dyn Statement) -> usize {
        // This is a simplified implementation
        // In a real implementation, you'd need to check the actual statement type
        // and recurse into nested structures
        
        // For now, assume any control flow statement adds 1 to complexity
        if Self::is_control_flow_statement(statement) {
            1
        } else {
            0
        }
    }

    /// Check if a statement is a control flow statement
    fn is_control_flow_statement(statement: &dyn Statement) -> bool {
        // This would need actual type checking based on your AST structure
        // For now, return false as a placeholder
        false
    }

    /// Calculate nesting depth of statements
    pub fn nesting_depth(statements: &[Box<dyn Statement>]) -> usize {
        statements.iter()
            .map(|stmt| Self::statement_nesting_depth(stmt.as_ref(), 0))
            .max()
            .unwrap_or(0)
    }

    /// Calculate nesting depth of a single statement
    fn statement_nesting_depth(statement: &dyn Statement, current_depth: usize) -> usize {
        // This would need to be implemented based on your actual AST structure
        current_depth
    }
}

/// Utility for analyzing performance issues
pub struct PerformanceAnalyzer;

impl PerformanceAnalyzer {
    /// Check if an expression involves string concatenation in a loop
    pub fn is_string_concatenation_in_loop(expression: &dyn Expression) -> bool {
        // This would need to be implemented based on your actual AST structure
        false
    }

    /// Check if an allocation could be avoided
    pub fn is_unnecessary_allocation(expression: &dyn Expression) -> bool {
        // This would need to be implemented based on your actual AST structure
        false
    }

    /// Suggest performance improvements
    pub fn suggest_performance_improvement(issue_type: &str) -> String {
        match issue_type {
            "string_concat_loop" => "Consider using a string builder or collecting into a vector".to_string(),
            "unnecessary_clone" => "Remove unnecessary clone() call".to_string(),
            "inefficient_loop" => "Consider using iterator methods instead of manual loops".to_string(),
            _ => "Consider optimizing this code".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_analyzer_camel_case() {
        assert!(IdentifierAnalyzer::is_camel_case("myVariable"));
        assert!(IdentifierAnalyzer::is_camel_case("getData"));
        assert!(!IdentifierAnalyzer::is_camel_case("MyVariable"));
        assert!(!IdentifierAnalyzer::is_camel_case("my_variable"));
        assert!(!IdentifierAnalyzer::is_camel_case("myvariable"));
    }

    #[test]
    fn test_identifier_analyzer_snake_case() {
        assert!(IdentifierAnalyzer::is_snake_case("my_variable"));
        assert!(IdentifierAnalyzer::is_snake_case("get_data"));
        assert!(!IdentifierAnalyzer::is_snake_case("myVariable"));
        assert!(!IdentifierAnalyzer::is_snake_case("MyVariable"));
        assert!(!IdentifierAnalyzer::is_snake_case("_private"));
        assert!(!IdentifierAnalyzer::is_snake_case("trailing_"));
    }

    #[test]
    fn test_identifier_analyzer_pascal_case() {
        assert!(IdentifierAnalyzer::is_pascal_case("MyClass"));
        assert!(IdentifierAnalyzer::is_pascal_case("DataProcessor"));
        assert!(!IdentifierAnalyzer::is_pascal_case("myClass"));
        assert!(!IdentifierAnalyzer::is_pascal_case("my_class"));
    }

    #[test]
    fn test_identifier_analyzer_screaming_snake_case() {
        assert!(IdentifierAnalyzer::is_screaming_snake_case("MY_CONSTANT"));
        assert!(IdentifierAnalyzer::is_screaming_snake_case("MAX_SIZE"));
        assert!(!IdentifierAnalyzer::is_screaming_snake_case("My_Constant"));
        assert!(!IdentifierAnalyzer::is_screaming_snake_case("my_constant"));
    }

    #[test]
    fn test_identifier_conversion() {
        assert_eq!(IdentifierAnalyzer::to_camel_case("my_variable"), "myVariable");
        assert_eq!(IdentifierAnalyzer::to_snake_case("myVariable"), "my_variable");
        assert_eq!(IdentifierAnalyzer::to_pascal_case("my_variable"), "MyVariable");
        assert_eq!(IdentifierAnalyzer::to_screaming_snake_case("myVariable"), "MY_VARIABLE");
    }

    #[test]
    fn test_identifier_splitting() {
        let words = IdentifierAnalyzer::split_identifier("myVariableName");
        assert_eq!(words, vec!["my", "Variable", "Name"]);

        let words = IdentifierAnalyzer::split_identifier("my_variable_name");
        assert_eq!(words, vec!["my", "variable", "name"]);
    }

    #[test]
    fn test_location_helper() {
        let source_lines = vec![
            "line 1".to_string(),
            "line 2".to_string(),
            "line 3".to_string(),
        ];
        let helper = LocationHelper::new(&source_lines, Some("test.csd".to_string()));

        let location = helper.location(1, 5);
        assert_eq!(location.line, 1);
        assert_eq!(location.column, 5);
        assert_eq!(location.source_line, "line 2");
        assert_eq!(location.file, Some("test.csd".to_string()));
    }

    #[test]
    fn test_complexity_analyzer() {
        let complexity = ComplexityAnalyzer::cyclomatic_complexity(&[]);
        assert_eq!(complexity, 1); // Base complexity
    }
}
