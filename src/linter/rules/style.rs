//! Style-related linting rules for the CURSED programming language
//!
//! These rules enforce consistent code formatting, naming conventions,
//! and stylistic preferences to improve code readability and maintainability.

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use crate::lexer::{Token, TokenType};
use crate::linter::{
    engine::LintIssue,
    rules::{
        base::{IssueBuilder, LocationHelper, IdentifierAnalyzer, NamingConvention},
        LintRule, RuleCategory, RuleSeverity,
    },
    visitor::AnalysisContext,
    fix::FixSuggestion,
};
use std::collections::HashMap;
use tracing::debug;

/// Rule to check line length limits
pub struct LineLengthRule {
    max_length: usize,
}

impl LineLengthRule {
    pub fn new() -> Self {
        Self { max_length: 100 }
    }

    pub fn with_max_length(max_length: usize) -> Self {
        Self { max_length }
    }
}

impl IssueBuilder for LineLengthRule {}

impl LintRule for LineLengthRule {
    fn name(&self) -> &'static str {
        "line-length"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Lines should not exceed the maximum length limit"
    }

    fn check_text(&self, source: &str, file_name: Option<&str>) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        let location_helper = LocationHelper::new(&source_lines, file_name.map(|s| s.to_string()));

        for (line_num, line) in source_lines.iter().enumerate() {
            if line.len() > self.max_length {
                let location = location_helper.location(line_num, line.len());
                let issue = self.create_issue(
                    self.default_severity(),
                    self.name(),
                    self.category(),
                    format!("Line exceeds maximum length of {} characters ({})", self.max_length, line.len()),
                    location,
                );
                issues.push(issue);
            }
        }

        Ok(issues)
    }

    fn configure(&mut self, params: &HashMap<String, serde_json::Value>) -> Result<(), Error> {
        if let Some(max_length) = params.get("max_length") {
            if let Some(length) = max_length.as_u64() {
                self.max_length = length as usize;
            }
        }
        Ok(())
    }
}

/// Rule to check for trailing whitespace
pub struct TrailingWhitespaceRule;

impl TrailingWhitespaceRule {
    pub fn new() -> Self {
        Self
    }
}

impl IssueBuilder for TrailingWhitespaceRule {}

impl LintRule for TrailingWhitespaceRule {
    fn name(&self) -> &'static str {
        "trailing-whitespace"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Info
    }

    fn description(&self) -> &'static str {
        "Lines should not have trailing whitespace"
    }

    fn check_text(&self, source: &str, file_name: Option<&str>) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        let location_helper = LocationHelper::new(&source_lines, file_name.map(|s| s.to_string()));

        for (line_num, line) in source_lines.iter().enumerate() {
            if line.ends_with(' ') || line.ends_with('\t') {
                let location = location_helper.location(line_num, line.trim_end().len());
                let fix = FixSuggestion::simple_replacement(
                    "Remove trailing whitespace".to_string(),
                    line.trim_end().to_string(),
                );
                let issue = self.create_issue_with_fix(
                    self.default_severity(),
                    self.name(),
                    self.category(),
                    "Line has trailing whitespace".to_string(),
                    location,
                    fix,
                );
                issues.push(issue);
            }
        }

        Ok(issues)
    }

    fn supports_auto_fix(&self) -> bool {
        true
    }
}

/// Rule to check for mixed indentation (tabs and spaces)
pub struct MixedIndentationRule;

impl MixedIndentationRule {
    pub fn new() -> Self {
        Self
    }
}

impl IssueBuilder for MixedIndentationRule {}

impl LintRule for MixedIndentationRule {
    fn name(&self) -> &'static str {
        "mixed-indentation"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Use consistent indentation (either tabs or spaces, not both)"
    }

    fn check_text(&self, source: &str, file_name: Option<&str>) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        let location_helper = LocationHelper::new(&source_lines, file_name.map(|s| s.to_string()));

        for (line_num, line) in source_lines.iter().enumerate() {
            let leading_whitespace = line.chars().take_while(|&c| c == ' ' || c == '\t').collect::<String>();
            if leading_whitespace.contains('\t') && leading_whitespace.contains(' ') {
                let location = location_helper.location(line_num, 0);
                let issue = self.create_issue(
                    self.default_severity(),
                    self.name(),
                    self.category(),
                    "Mixed tabs and spaces for indentation".to_string(),
                    location,
                );
                issues.push(issue);
            }
        }

        Ok(issues)
    }
}

/// Rule to check for excessive empty lines
pub struct EmptyLineRule {
    max_consecutive: usize,
}

impl EmptyLineRule {
    pub fn new() -> Self {
        Self { max_consecutive: 2 }
    }
}

impl IssueBuilder for EmptyLineRule {}

impl LintRule for EmptyLineRule {
    fn name(&self) -> &'static str {
        "empty-lines"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Info
    }

    fn description(&self) -> &'static str {
        "Limit consecutive empty lines"
    }

    fn check_text(&self, source: &str, file_name: Option<&str>) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        let location_helper = LocationHelper::new(&source_lines, file_name.map(|s| s.to_string()));

        let mut consecutive_empty = 0;
        for (line_num, line) in source_lines.iter().enumerate() {
            if line.trim().is_empty() {
                consecutive_empty += 1;
                if consecutive_empty > self.max_consecutive {
                    let location = location_helper.location(line_num, 0);
                    let issue = self.create_issue(
                        self.default_severity(),
                        self.name(),
                        self.category(),
                        format!("Too many consecutive empty lines ({})", consecutive_empty),
                        location,
                    );
                    issues.push(issue);
                }
            } else {
                consecutive_empty = 0;
            }
        }

        Ok(issues)
    }

    fn configure(&mut self, params: &HashMap<String, serde_json::Value>) -> Result<(), Error> {
        if let Some(max_consecutive) = params.get("max_consecutive") {
            if let Some(count) = max_consecutive.as_u64() {
                self.max_consecutive = count as usize;
            }
        }
        Ok(())
    }
}

/// Rule to check naming conventions
pub struct NamingConventionRule {
    variable_convention: NamingConvention,
    function_convention: NamingConvention,
    struct_convention: NamingConvention,
    constant_convention: NamingConvention,
}

impl NamingConventionRule {
    pub fn new() -> Self {
        Self {
            variable_convention: NamingConvention::SnakeCase,
            function_convention: NamingConvention::SnakeCase,
            struct_convention: NamingConvention::PascalCase,
            constant_convention: NamingConvention::ScreamingSnakeCase,
        }
    }
}

impl IssueBuilder for NamingConventionRule {}

impl LintRule for NamingConventionRule {
    fn name(&self) -> &'static str {
        "naming-convention"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Enforce consistent naming conventions"
    }

    fn check_tokens(&self, tokens: &[Token], context: &mut AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        for (i, token) in tokens.iter().enumerate() {
            if let Token::Identifier(name) = token {
                
                // Determine expected convention based on context
                let expected_convention = self.determine_convention(tokens, i);
                
                if let Some(convention) = expected_convention {
                    if !self.matches_convention(name, convention) {
                        let location = location_helper.location_from_token(token);
                        let suggestion = IdentifierAnalyzer::suggest_naming_convention(name, convention);
                        let fix = FixSuggestion::simple_replacement(
                            format!("Rename to '{}'", suggestion),
                            suggestion,
                        );
                        
                        let issue = self.create_issue_with_fix(
                            self.default_severity(),
                            self.name(),
                            self.category(),
                            format!("Identifier '{}' does not follow {:?} convention", name, convention),
                            location,
                            fix,
                        );
                        issues.push(issue);
                    }
                }
            }
        }

        Ok(issues)
    }

    fn supports_auto_fix(&self) -> bool {
        true
    }
}

impl NamingConventionRule {
    /// Determine the expected naming convention based on token context
    fn determine_convention(&self, tokens: &[Token], index: usize) -> Option<NamingConvention> {
        // Look at preceding tokens to determine context
        if index == 0 {
            return None;
        }

        let prev_token = &tokens[index - 1];
        match prev_token {
            Token::Sus | Token::Facts => Some(self.variable_convention),
            Token::Slay => Some(self.function_convention),
            Token::Squad => Some(self.struct_convention),
            Token::Periodt => Some(self.constant_convention),
            _ => None,
        }
    }

    /// Check if a name matches the expected convention
    fn matches_convention(&self, name: &str, convention: NamingConvention) -> bool {
        match convention {
            NamingConvention::CamelCase => IdentifierAnalyzer::is_camel_case(name),
            NamingConvention::SnakeCase => IdentifierAnalyzer::is_snake_case(name),
            NamingConvention::PascalCase => IdentifierAnalyzer::is_pascal_case(name),
            NamingConvention::ScreamingSnakeCase => IdentifierAnalyzer::is_screaming_snake_case(name),
        }
    }
}

/// Rule to check operator spacing
pub struct OperatorSpacingRule {
    require_spaces: bool,
}

impl OperatorSpacingRule {
    pub fn new() -> Self {
        Self { require_spaces: true }
    }
}

impl IssueBuilder for OperatorSpacingRule {}

impl LintRule for OperatorSpacingRule {
    fn name(&self) -> &'static str {
        "operator-spacing"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Info
    }

    fn description(&self) -> &'static str {
        "Ensure consistent spacing around operators"
    }

    fn check_tokens(&self, tokens: &[Token], context: &mut AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        for (i, token) in tokens.iter().enumerate() {
            if self.is_binary_operator_token(token) {
                let location = location_helper.location_from_token(token);
                
                // Check spacing before operator
                if i > 0 {
                    let prev_token = &tokens[i - 1];
                    let has_space_before = self.has_space_between_tokens(prev_token, token, &context.source_lines);
                    
                    if self.require_spaces && !has_space_before {
                        let issue = self.create_issue(
                            self.default_severity(),
                            self.name(),
                            self.category(),
                            format!("Missing space before operator '{}'", self.token_symbol(token)),
                            location.clone(),
                        );
                        issues.push(issue);
                    }
                }
                
                // Check spacing after operator
                if i + 1 < tokens.len() {
                    let next_token = &tokens[i + 1];
                    let has_space_after = self.has_space_between_tokens(token, next_token, &context.source_lines);
                    
                    if self.require_spaces && !has_space_after {
                        let issue = self.create_issue(
                            self.default_severity(),
                            self.name(),
                            self.category(),
                            format!("Missing space after operator '{}'", self.token_symbol(token)),
                            location,
                        );
                        issues.push(issue);
                    }
                }
            }
        }

        Ok(issues)
    }
}

impl OperatorSpacingRule {
    /// Check if a token is a binary operator
    fn is_binary_operator_token(&self, token: &Token) -> bool {
        matches!(token, 
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash |
            Token::Eq | Token::NotEq | Token::Lt | Token::Gt |
            Token::LtEq | Token::GtEq | Token::Assign
        )
    }

    /// Get the string representation of a token
    fn token_symbol(&self, token: &Token) -> &str {
        match token {
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Eq => "==",
            Token::NotEq => "!=",
            Token::Lt => "<",
            Token::Gt => ">",
            Token::LtEq => "<=",
            Token::GtEq => ">=",
            Token::Assign => "=",
            _ => "?",
        }
    }

    /// Check if there's whitespace between two tokens
    fn has_space_between_tokens(&self, first: &Token, second: &Token, source_lines: &[String]) -> bool {
        // For now, return true since we don't have position tracking in tokens
        // In a full implementation, we would track positions in the lexer
        true
    }
}

/// Rule to check comma spacing
pub struct CommaSpacingRule {
    space_after: bool,
    space_before: bool,
}

impl CommaSpacingRule {
    pub fn new() -> Self {
        Self {
            space_after: true,
            space_before: false,
        }
    }
}

impl IssueBuilder for CommaSpacingRule {}

impl LintRule for CommaSpacingRule {
    fn name(&self) -> &'static str {
        "comma-spacing"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Info
    }

    fn description(&self) -> &'static str {
        "Ensure consistent spacing around commas"
    }

    fn check_tokens(&self, tokens: &[Token], context: &mut AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        for (i, token) in tokens.iter().enumerate() {
            if matches!(token, Token::Comma) {
                let location = location_helper.location_from_token(token);
                
                // Check spacing before comma
                if i > 0 && self.space_before {
                    let prev_token = &tokens[i - 1];
                    let has_space_before = self.has_space_between_tokens(prev_token, token, &context.source_lines);
                    
                    if !has_space_before {
                        let issue = self.create_issue(
                            self.default_severity(),
                            self.name(),
                            self.category(),
                            "Missing space before comma".to_string(),
                            location.clone(),
                        );
                        issues.push(issue);
                    }
                }
                
                // Check spacing after comma
                if i + 1 < tokens.len() && self.space_after {
                    let next_token = &tokens[i + 1];
                    let has_space_after = self.has_space_between_tokens(token, next_token, &context.source_lines);
                    
                    if !has_space_after {
                        let issue = self.create_issue(
                            self.default_severity(),
                            self.name(),
                            self.category(),
                            "Missing space after comma".to_string(),
                            location,
                        );
                        issues.push(issue);
                    }
                }
            }
        }

        Ok(issues)
    }
}

impl CommaSpacingRule {
    /// Check if there's whitespace between two tokens (reused from OperatorSpacingRule)
    fn has_space_between_tokens(&self, first: &Token, second: &Token, source_lines: &[String]) -> bool {
        // For now, return true since we don't have position tracking in tokens
        // In a full implementation, we would track positions in the lexer
        true
    }
}

/// Rule to check brace style consistency
pub struct BraceStyleRule {
    style: BraceStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BraceStyle {
    SameLine,
    NextLine,
    NextLineUnindented,
}

impl BraceStyleRule {
    pub fn new() -> Self {
        Self {
            style: BraceStyle::SameLine,
        }
    }
}

impl IssueBuilder for BraceStyleRule {}

impl LintRule for BraceStyleRule {
    fn name(&self) -> &'static str {
        "brace-style"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Info
    }

    fn description(&self) -> &'static str {
        "Enforce consistent brace style"
    }

    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        // This would need to traverse the AST looking for block statements
        // and check their brace positioning
        Ok(Vec::new())
    }
}

/// Rule to check function length
pub struct FunctionLengthRule {
    max_lines: usize,
}

impl FunctionLengthRule {
    pub fn new() -> Self {
        Self { max_lines: 50 }
    }
}

impl IssueBuilder for FunctionLengthRule {}

impl LintRule for FunctionLengthRule {
    fn name(&self) -> &'static str {
        "function-length"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Style
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Functions should not exceed the maximum line limit"
    }

    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        // This would need to traverse function declarations and count lines
        Ok(Vec::new())
    }

    fn configure(&mut self, params: &HashMap<String, serde_json::Value>) -> Result<(), Error> {
        if let Some(max_lines) = params.get("max_lines") {
            if let Some(lines) = max_lines.as_u64() {
                self.max_lines = lines as usize;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_length_rule() {
        let rule = LineLengthRule::with_max_length(10);
        let source = "short\nthis line is definitely too long";
        let issues = rule.check_text(source, None).unwrap();
        
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule_name, "line-length");
        assert_eq!(issues[0].location.line, 1);
    }

    #[test]
    fn test_trailing_whitespace_rule() {
        let rule = TrailingWhitespaceRule::new();
        let source = "clean line\nline with space \nclean again";
        let issues = rule.check_text(source, None).unwrap();
        
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule_name, "trailing-whitespace");
        assert_eq!(issues[0].location.line, 1);
        assert!(issues[0].suggestion.is_some());
    }

    #[test]
    fn test_mixed_indentation_rule() {
        let rule = MixedIndentationRule::new();
        let source = "\tif true {\n    \treturn false\n}";
        let issues = rule.check_text(source, None).unwrap();
        
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule_name, "mixed-indentation");
    }

    #[test]
    fn test_empty_line_rule() {
        let rule = EmptyLineRule::new();
        let source = "line 1\n\n\n\nline 2";
        let issues = rule.check_text(source, None).unwrap();
        
        assert_eq!(issues.len(), 2); // Lines 3 and 4 exceed the limit
        assert_eq!(issues[0].rule_name, "empty-lines");
    }

    #[test]
    fn test_naming_convention_matches() {
        let rule = NamingConventionRule::new();
        
        assert!(rule.matches_convention("my_variable", NamingConvention::SnakeCase));
        assert!(rule.matches_convention("myVariable", NamingConvention::CamelCase));
        assert!(rule.matches_convention("MyClass", NamingConvention::PascalCase));
        assert!(rule.matches_convention("MY_CONSTANT", NamingConvention::ScreamingSnakeCase));
        
        assert!(!rule.matches_convention("MyVariable", NamingConvention::SnakeCase));
        assert!(!rule.matches_convention("my_variable", NamingConvention::CamelCase));
    }

    #[test]
    fn test_operator_spacing_rule() {
        let rule = OperatorSpacingRule::new();
        assert!(rule.is_binary_operator(&TokenType::Plus));
        assert!(rule.is_binary_operator(&TokenType::Equal));
        assert!(!rule.is_binary_operator(&TokenType::Identifier));
    }
}
