//! Linting rules for the CURSED programming language
//!
//! This module contains all the individual linting rules organized by category.
//! Each rule implements the `LintRule` trait and provides specific analysis
//! for different aspects of CURSED code.

pub mod base;
pub mod complexity;
pub mod correctness;
pub mod cursed_specific;
pub mod performance;
pub mod style;

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use crate::lexer::Token;
use crate::linter::{
    config::LinterConfig,
    engine::LintIssue,
    visitor::AnalysisContext,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Severity level for rules
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RuleSeverity {
    Info,
    Warning,
    Error,
}

impl fmt::Display for RuleSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleSeverity::Info => write!(f, "info"),
            RuleSeverity::Warning => write!(f, "warning"),
            RuleSeverity::Error => write!(f, "error"),
        }
    }
}

/// Rule categories for organizing lint rules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum RuleCategory {
    Style,
    Correctness,
    Performance,
    Complexity,
    CursedSpecific,
}

impl fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleCategory::Style => write!(f, "style"),
            RuleCategory::Correctness => write!(f, "correctness"),
            RuleCategory::Performance => write!(f, "performance"),
            RuleCategory::Complexity => write!(f, "complexity"),
            RuleCategory::CursedSpecific => write!(f, "cursed"),
        }
    }
}

/// Trait for implementing lint rules
pub trait LintRule: Send + Sync {
    /// Get the name of this rule
    fn name(&self) -> &'static str;
    
    /// Get the category of this rule
    fn category(&self) -> RuleCategory;
    
    /// Get the default severity of this rule
    fn default_severity(&self) -> RuleSeverity;
    
    /// Get a description of what this rule checks
    fn description(&self) -> &'static str;
    
    /// Check text-based issues (runs before parsing)
    fn check_text(&self, _source: &str, _file_name: Option<&str>) -> Result<Vec<LintIssue>, Error> {
        Ok(Vec::new())
    }
    
    /// Check token-based issues (runs after lexing)
    fn check_tokens(&self, _tokens: &[Token], _context: &mut AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        Ok(Vec::new())
    }
    
    /// Check AST-based issues (runs after parsing)
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        Ok(Vec::new())
    }
    
    /// Check if this rule supports auto-fixing
    fn supports_auto_fix(&self) -> bool {
        false
    }
    
    /// Apply auto-fix for issues found by this rule
    fn auto_fix(&self, _source: &str, _issues: &[LintIssue]) -> Result<String, Error> {
        Err(Error::Configuration("Auto-fix not supported for this rule".to_string()))
    }
    
    /// Configure this rule with custom parameters
    fn configure(&mut self, _params: &HashMap<String, serde_json::Value>) -> Result<(), Error> {
        Ok(())
    }
}

/// A collection of lint rules organized by type
pub struct LintRuleSet {
    text_rules: Vec<Box<dyn LintRule>>,
    token_rules: Vec<Box<dyn LintRule>>,
    ast_rules: Vec<Box<dyn LintRule>>,
}

impl LintRuleSet {
    /// Create a new rule set from configuration
    pub fn from_config(config: &LinterConfig) -> Self {
        let mut rule_set = Self {
            text_rules: Vec::new(),
            token_rules: Vec::new(),
            ast_rules: Vec::new(),
        };
        
        // Add style rules
        if config.rules.style.enabled {
            rule_set.add_style_rules();
        }
        
        // Add correctness rules
        if config.rules.correctness.enabled {
            rule_set.add_correctness_rules();
        }
        
        // Add performance rules
        if config.rules.performance.enabled {
            rule_set.add_performance_rules();
        }
        
        // Add complexity rules
        if config.rules.complexity.enabled {
            rule_set.add_complexity_rules();
        }
        
        // Add CURSED-specific rules
        if config.rules.cursed_specific.enabled {
            rule_set.add_cursed_specific_rules();
        }
        
        rule_set
    }

    /// Add style rules to the rule set
    fn add_style_rules(&mut self) {
        // Text-based style rules
        self.text_rules.push(Box::new(style::LineLengthRule::new()));
        self.text_rules.push(Box::new(style::TrailingWhitespaceRule::new()));
        self.text_rules.push(Box::new(style::MixedIndentationRule::new()));
        self.text_rules.push(Box::new(style::EmptyLineRule::new()));
        
        // Token-based style rules
        self.token_rules.push(Box::new(style::NamingConventionRule::new()));
        self.token_rules.push(Box::new(style::OperatorSpacingRule::new()));
        self.token_rules.push(Box::new(style::CommaSpacingRule::new()));
        
        // AST-based style rules
        self.ast_rules.push(Box::new(style::BraceStyleRule::new()));
        self.ast_rules.push(Box::new(style::FunctionLengthRule::new()));
    }

    /// Add correctness rules to the rule set
    fn add_correctness_rules(&mut self) {
        self.ast_rules.push(Box::new(correctness::UnusedVariableRule::new()));
        self.ast_rules.push(Box::new(correctness::UnusedFunctionRule::new()));
        self.ast_rules.push(Box::new(correctness::UnreachableCodeRule::new()));
        self.ast_rules.push(Box::new(correctness::DeadCodeRule::new()));
        self.ast_rules.push(Box::new(correctness::ShadowingRule::new()));
        self.ast_rules.push(Box::new(correctness::UnusedImportRule::new()));
    }

    /// Add performance rules to the rule set
    fn add_performance_rules(&mut self) {
        self.ast_rules.push(Box::new(performance::UnnecessaryAllocationRule::new()));
        self.ast_rules.push(Box::new(performance::StringConcatenationRule::new()));
        self.ast_rules.push(Box::new(performance::InefficientLoopRule::new()));
        self.ast_rules.push(Box::new(performance::RedundantCloneRule::new()));
    }

    /// Add complexity rules to the rule set
    fn add_complexity_rules(&mut self) {
        self.ast_rules.push(Box::new(complexity::CyclomaticComplexityRule::new()));
        self.ast_rules.push(Box::new(complexity::NestingDepthRule::new()));
        self.ast_rules.push(Box::new(complexity::ParameterCountRule::new()));
        self.ast_rules.push(Box::new(complexity::CognitiveComplexityRule::new()));
    }

    /// Add CURSED-specific rules to the rule set
    fn add_cursed_specific_rules(&mut self) {
        self.ast_rules.push(Box::new(cursed_specific::GenZNamingRule::new()));
        self.ast_rules.push(Box::new(cursed_specific::SlangUsageRule::new()));
        self.ast_rules.push(Box::new(cursed_specific::InterfaceDesignRule::new()));
        self.ast_rules.push(Box::new(cursed_specific::GoroutineBestPracticesRule::new()));
        self.ast_rules.push(Box::new(cursed_specific::ChannelUsageRule::new()));
    }

    /// Get text-based rules
    pub fn text_rules(&self) -> &[Box<dyn LintRule>] {
        &self.text_rules
    }

    /// Get token-based rules
    pub fn token_rules(&self) -> &[Box<dyn LintRule>] {
        &self.token_rules
    }

    /// Get AST-based rules
    pub fn ast_rules(&self) -> &[Box<dyn LintRule>] {
        &self.ast_rules
    }

    /// Get all rules
    pub fn all_rules(&self) -> impl Iterator<Item = &Box<dyn LintRule>> {
        self.text_rules.iter()
            .chain(self.token_rules.iter())
            .chain(self.ast_rules.iter())
    }

    /// Get rules by category
    pub fn rules_by_category(&self, category: RuleCategory) -> Vec<&Box<dyn LintRule>> {
        self.all_rules()
            .filter(|rule| rule.category() == category)
            .collect()
    }

    /// Get rule by name
    pub fn rule_by_name(&self, name: &str) -> Option<&Box<dyn LintRule>> {
        self.all_rules()
            .find(|rule| rule.name() == name)
    }

    /// Get rule count by category
    pub fn rule_count_by_category(&self) -> HashMap<RuleCategory, usize> {
        let mut counts = HashMap::new();
        for rule in self.all_rules() {
            *counts.entry(rule.category()).or_insert(0) += 1;
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::config::LinterConfig;

    #[test]
    fn test_rule_severity_ordering() {
        assert!(RuleSeverity::Error > RuleSeverity::Warning);
        assert!(RuleSeverity::Warning > RuleSeverity::Info);
    }

    #[test]
    fn test_rule_category_display() {
        assert_eq!(format!("{}", RuleCategory::Style), "style");
        assert_eq!(format!("{}", RuleCategory::Correctness), "correctness");
        assert_eq!(format!("{}", RuleCategory::Performance), "performance");
        assert_eq!(format!("{}", RuleCategory::Complexity), "complexity");
        assert_eq!(format!("{}", RuleCategory::CursedSpecific), "cursed");
    }

    #[test]
    fn test_rule_set_creation() {
        let config = LinterConfig::default();
        let rule_set = LintRuleSet::from_config(&config);
        
        assert!(!rule_set.text_rules.is_empty());
        assert!(!rule_set.token_rules.is_empty());
        assert!(!rule_set.ast_rules.is_empty());
    }

    #[test]
    fn test_rule_set_category_counts() {
        let config = LinterConfig::default();
        let rule_set = LintRuleSet::from_config(&config);
        let counts = rule_set.rule_count_by_category();
        
        // Should have rules in all categories
        assert!(counts.contains_key(&RuleCategory::Style));
        assert!(counts.contains_key(&RuleCategory::Correctness));
        assert!(counts.contains_key(&RuleCategory::Performance));
        assert!(counts.contains_key(&RuleCategory::Complexity));
        assert!(counts.contains_key(&RuleCategory::CursedSpecific));
    }

    #[test]
    fn test_rule_lookup() {
        let config = LinterConfig::default();
        let rule_set = LintRuleSet::from_config(&config);
        
        // Should be able to find rules by name
        assert!(rule_set.rule_by_name("line-length").is_some());
        assert!(rule_set.rule_by_name("nonexistent-rule").is_none());
    }

    #[test]
    fn test_rules_by_category() {
        let config = LinterConfig::default();
        let rule_set = LintRuleSet::from_config(&config);
        
        let style_rules = rule_set.rules_by_category(RuleCategory::Style);
        assert!(!style_rules.is_empty());
        
        for rule in style_rules {
            assert_eq!(rule.category(), RuleCategory::Style);
        }
    }
}
