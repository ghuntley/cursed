//! Performance-related linting rules for CURSED

use crate::ast::*;
use crate::error::Error;
use crate::linter::{
    engine::LintIssue,
    rules::{base::IssueBuilder, LintRule, RuleCategory, RuleSeverity},
    visitor::AnalysisContext,
};

pub struct UnnecessaryAllocationRule;
impl UnnecessaryAllocationRule { pub fn new() -> Self { Self } }
impl IssueBuilder for UnnecessaryAllocationRule {}
impl LintRule for UnnecessaryAllocationRule {
    fn name(&self) -> &'static str { "unnecessary-allocation" }
    fn category(&self) -> RuleCategory { RuleCategory::Performance }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Detect unnecessary memory allocations" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct StringConcatenationRule;
impl StringConcatenationRule { pub fn new() -> Self { Self } }
impl IssueBuilder for StringConcatenationRule {}
impl LintRule for StringConcatenationRule {
    fn name(&self) -> &'static str { "string-concatenation" }
    fn category(&self) -> RuleCategory { RuleCategory::Performance }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Info }
    fn description(&self) -> &'static str { "Suggest efficient string concatenation" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct InefficientLoopRule;
impl InefficientLoopRule { pub fn new() -> Self { Self } }
impl IssueBuilder for InefficientLoopRule {}
impl LintRule for InefficientLoopRule {
    fn name(&self) -> &'static str { "inefficient-loop" }
    fn category(&self) -> RuleCategory { RuleCategory::Performance }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Info }
    fn description(&self) -> &'static str { "Suggest more efficient loop patterns" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct RedundantCloneRule;
impl RedundantCloneRule { pub fn new() -> Self { Self } }
impl IssueBuilder for RedundantCloneRule {}
impl LintRule for RedundantCloneRule {
    fn name(&self) -> &'static str { "redundant-clone" }
    fn category(&self) -> RuleCategory { RuleCategory::Performance }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Detect unnecessary clone operations" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}
