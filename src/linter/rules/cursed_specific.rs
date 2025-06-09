//! CURSED-specific linting rules

use crate::ast::*;
use crate::error::Error;
use crate::linter::{
    engine::LintIssue,
    rules::{base::IssueBuilder, LintRule, RuleCategory, RuleSeverity},
    visitor::AnalysisContext,
};

pub struct GenZNamingRule;
impl GenZNamingRule { pub fn new() -> Self { Self } }
impl IssueBuilder for GenZNamingRule {}
impl LintRule for GenZNamingRule {
    fn name(&self) -> &'static str { "genz-naming" }
    fn category(&self) -> RuleCategory { RuleCategory::CursedSpecific }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Info }
    fn description(&self) -> &'static str { "Encourage Gen Z naming conventions" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct SlangUsageRule;
impl SlangUsageRule { pub fn new() -> Self { Self } }
impl IssueBuilder for SlangUsageRule {}
impl LintRule for SlangUsageRule {
    fn name(&self) -> &'static str { "slang-usage" }
    fn category(&self) -> RuleCategory { RuleCategory::CursedSpecific }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Info }
    fn description(&self) -> &'static str { "Suggest proper Gen Z slang usage" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct InterfaceDesignRule;
impl InterfaceDesignRule { pub fn new() -> Self { Self } }
impl IssueBuilder for InterfaceDesignRule {}
impl LintRule for InterfaceDesignRule {
    fn name(&self) -> &'static str { "interface-design" }
    fn category(&self) -> RuleCategory { RuleCategory::CursedSpecific }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Check interface design patterns" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct GoroutineBestPracticesRule;
impl GoroutineBestPracticesRule { pub fn new() -> Self { Self } }
impl IssueBuilder for GoroutineBestPracticesRule {}
impl LintRule for GoroutineBestPracticesRule {
    fn name(&self) -> &'static str { "goroutine-best-practices" }
    fn category(&self) -> RuleCategory { RuleCategory::CursedSpecific }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Enforce goroutine best practices" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}

pub struct ChannelUsageRule;
impl ChannelUsageRule { pub fn new() -> Self { Self } }
impl IssueBuilder for ChannelUsageRule {}
impl LintRule for ChannelUsageRule {
    fn name(&self) -> &'static str { "channel-usage" }
    fn category(&self) -> RuleCategory { RuleCategory::CursedSpecific }
    fn default_severity(&self) -> RuleSeverity { RuleSeverity::Warning }
    fn description(&self) -> &'static str { "Check proper channel usage patterns" }
    fn check_ast(&self, _program: &Program, _context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> { Ok(Vec::new()) }
}
