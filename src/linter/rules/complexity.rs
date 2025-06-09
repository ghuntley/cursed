//! Complexity-related linting rules for CURSED

use crate::ast::*;
use crate::error::Error;
use crate::linter::{
    engine::LintIssue,
    rules::{base::IssueBuilder, LintRule, RuleCategory, RuleSeverity},
    visitor::AnalysisContext,
};

pub struct CyclomaticComplexityRule { max_complexity: usize }
impl CyclomaticComplexityRule { pub fn new() -> Self { Self { max_complexity: 10 } } }
impl IssueBuilder for CyclomaticComplexityRule {}
impl LintRule for CyclomaticComplexityRule {
    fn name(&self) -> &'static str { "cyclomatic-complexity" }
    fn category(&self) -> RuleCategory { RuleCategory::Complexity }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Check cyclomatic complexity of functions" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct NestingDepthRule { max_depth: usize }
impl NestingDepthRule { pub fn new() -> Self { Self { max_depth: 4 } } }
impl IssueBuilder for NestingDepthRule {}
impl LintRule for NestingDepthRule {
    fn name(&self) -> &'static str { "nesting-depth" }
    fn category(&self) -> RuleCategory { RuleCategory::Complexity }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Limit maximum nesting depth" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct ParameterCountRule { max_params: usize }
impl ParameterCountRule { pub fn new() -> Self { Self { max_params: 5 } } }
impl IssueBuilder for ParameterCountRule {}
impl LintRule for ParameterCountRule {
    fn name(&self) -> &'static str { "parameter-count" }
    fn category(&self) -> RuleCategory { RuleCategory::Complexity }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Info }
    fn description(&self) -> &'static str { "Limit number of function parameters" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct CognitiveComplexityRule { max_complexity: usize }
impl CognitiveComplexityRule { pub fn new() -> Self { Self { max_complexity: 15 } } }
impl IssueBuilder for CognitiveComplexityRule {}
impl LintRule for CognitiveComplexityRule {
    fn name(&self) -> &'static str { "cognitive-complexity" }
    fn category(&self) -> RuleCategory { RuleCategory::Complexity }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Check cognitive complexity of functions" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}
