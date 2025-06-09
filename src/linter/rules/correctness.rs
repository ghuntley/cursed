//! Correctness-related linting rules for the CURSED programming language
//!
//! These rules detect potential bugs, logical errors, and code that
//! may not behave as intended. They help ensure program correctness.

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use crate::lexer::Token;
use crate::linter::{
    engine::LintIssue,
    rules::{
        base::{IssueBuilder, LocationHelper},
        LintRule, RuleCategory, RuleSeverity,
    },
    visitor::AnalysisContext,
    fix::FixSuggestion,
};
use std::collections::{HashMap, HashSet};
use tracing::debug;

/// Rule to detect unused variables
pub struct UnusedVariableRule;

impl UnusedVariableRule {
    pub fn new() -> Self {
        Self
    }
}

impl IssueBuilder for UnusedVariableRule {}

impl LintRule for UnusedVariableRule {
    fn name(&self) -> &'static str {
        "unused-variable"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Correctness
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Detect variables that are declared but never used"
    }

    fn check_ast(&self, program: &Program, context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        // Collect declared and used variables
        let (declared_vars, used_vars) = self.collect_variable_usage(program);

        for (var_name, declaration_info) in declared_vars {
            if !used_vars.contains(&var_name) && !self.is_special_variable(&var_name) {
                let location = declaration_info.location;
                let fix = FixSuggestion::simple_replacement(
                    format!("Prefix with underscore to indicate intentionally unused: _{}", var_name),
                    format!("_{}", var_name),
                );
                
                let issue = self.create_issue_with_fix(
                    self.default_severity(),
                    self.name(),
                    self.category(),
                    format!("Variable '{}' is declared but never used", var_name),
                    location,
                    fix,
                );
                issues.push(issue);
            }
        }

        debug!("Found {} unused variables", issues.len());
        Ok(issues)
    }

    fn supports_auto_fix(&self) -> bool {
        true
    }
}

impl UnusedVariableRule {
    /// Collect declared and used variables from the AST
    fn collect_variable_usage(&self, program: &Program) -> (HashMap<String, VariableInfo>, HashSet<String>) {
        let mut declared = HashMap::new();
        let mut used = HashSet::new();

        // This is a simplified implementation
        // In a real implementation, you'd traverse the entire AST
        // looking for variable declarations and usage
        
        for statement in &program.statements {
            self.collect_from_statement(statement.as_ref(), &mut declared, &mut used);
        }

        (declared, used)
    }

    /// Collect variable info from a single statement
    fn collect_from_statement(
        &self,
        statement: &dyn Statement,
        declared: &mut HashMap<String, VariableInfo>,
        used: &mut HashSet<String>,
    ) {
        // This would need to be implemented based on your actual AST structure
        // For now, this is a placeholder
    }

    /// Check if a variable name is special (like _ or error variables)
    fn is_special_variable(&self, name: &str) -> bool {
        name.starts_with('_') || name == "err" || name == "ok"
    }
}

#[derive(Debug, Clone)]
struct VariableInfo {
    location: SourceLocation,
    var_type: String,
}

/// Rule to detect unused functions
pub struct UnusedFunctionRule;

impl UnusedFunctionRule {
    pub fn new() -> Self {
        Self
    }
}

impl IssueBuilder for UnusedFunctionRule {}

impl LintRule for UnusedFunctionRule {
    fn name(&self) -> &'static str {
        "unused-function"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Correctness
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Info
    }

    fn description(&self) -> &'static str {
        "Detect functions that are declared but never called"
    }

    fn check_ast(&self, program: &Program, context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        let (declared_funcs, called_funcs) = self.collect_function_usage(program);

        for (func_name, func_info) in declared_funcs {
            if !called_funcs.contains(&func_name) && !self.is_special_function(&func_name) {
                let location = func_info.location;
                let issue = self.create_issue(
                    self.default_severity(),
                    self.name(),
                    self.category(),
                    format!("Function '{}' is declared but never called", func_name),
                    location,
                );
                issues.push(issue);
            }
        }

        debug!("Found {} unused functions", issues.len());
        Ok(issues)
    }
}

impl UnusedFunctionRule {
    /// Collect declared and called functions from the AST
    fn collect_function_usage(&self, program: &Program) -> (HashMap<String, FunctionInfo>, HashSet<String>) {
        let mut declared = HashMap::new();
        let mut called = HashSet::new();

        // This would need actual AST traversal implementation
        (declared, called)
    }

    /// Check if a function name is special (like main, init, test functions)
    fn is_special_function(&self, name: &str) -> bool {
        name == "main" || name.starts_with("test_") || name.starts_with("init_")
    }
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    location: SourceLocation,
    parameters: Vec<String>,
}

/// Rule to detect unreachable code
pub struct UnreachableCodeRule;

impl UnreachableCodeRule {
    pub fn new() -> Self {
        Self
    }
}

impl IssueBuilder for UnreachableCodeRule {}

impl LintRule for UnreachableCodeRule {
    fn name(&self) -> &'static str {
        "unreachable-code"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Correctness
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Detect code that can never be executed"
    }

    fn check_ast(&self, program: &Program, context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        // Analyze control flow to find unreachable code
        for statement in &program.statements {
            let unreachable_statements = self.find_unreachable_in_statement(statement.as_ref());
            for unreachable in unreachable_statements {
                let location = unreachable.location;
                let fix = FixSuggestion::deletion("Remove unreachable code".to_string());
                let issue = self.create_issue_with_fix(
                    self.default_severity(),
                    self.name(),
                    self.category(),
                    "This code is unreachable".to_string(),
                    location,
                    fix,
                );
                issues.push(issue);
            }
        }

        debug!("Found {} unreachable code blocks", issues.len());
        Ok(issues)
    }

    fn supports_auto_fix(&self) -> bool {
        true
    }
}

impl UnreachableCodeRule {
    /// Find unreachable statements within a given statement
    fn find_unreachable_in_statement(&self, statement: &dyn Statement) -> Vec<UnreachableInfo> {
        // This would need to analyze control flow based on your AST structure
        Vec::new()
    }
}

#[derive(Debug, Clone)]
struct UnreachableInfo {
    location: SourceLocation,
    reason: String,
}

/// Rule to detect dead code (code with no effect)
pub struct DeadCodeRule;

impl DeadCodeRule {
    pub fn new() -> Self {
        Self
    }
}

impl IssueBuilder for DeadCodeRule {}

impl LintRule for DeadCodeRule {
    fn name(&self) -> &'static str {
        "dead-code"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Correctness
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Info
    }

    fn description(&self) -> &'static str {
        "Detect code that has no effect on program behavior"
    }

    fn check_ast(&self, program: &Program, context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        // Look for expressions with no side effects
        for statement in &program.statements {
            let dead_expressions = self.find_dead_expressions_in_statement(statement.as_ref());
            for dead_expr in dead_expressions {
                let location = dead_expr.location;
                let issue = self.create_issue(
                    self.default_severity(),
                    self.name(),
                    self.category(),
                    "Expression has no effect".to_string(),
                    location,
                );
                issues.push(issue);
            }
        }

        debug!("Found {} dead code expressions", issues.len());
        Ok(issues)
    }
}

impl DeadCodeRule {
    /// Find expressions with no side effects
    fn find_dead_expressions_in_statement(&self, statement: &dyn Statement) -> Vec<DeadExpressionInfo> {
        // This would analyze expressions to find those with no side effects
        Vec::new()
    }
}

#[derive(Debug, Clone)]
struct DeadExpressionInfo {
    location: SourceLocation,
    expression_type: String,
}

/// Rule to detect variable shadowing
pub struct ShadowingRule;

impl ShadowingRule {
    pub fn new() -> Self {
        Self
    }
}

impl IssueBuilder for ShadowingRule {}

impl LintRule for ShadowingRule {
    fn name(&self) -> &'static str {
        "shadowing"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Correctness
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Detect variables that shadow variables from outer scopes"
    }

    fn check_ast(&self, program: &Program, context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        let shadowing_cases = self.find_shadowing(program);
        for shadow_case in shadowing_cases {
            let location = shadow_case.inner_location;
            let issue = self.create_issue(
                self.default_severity(),
                self.name(),
                self.category(),
                format!(
                    "Variable '{}' shadows variable from outer scope (line {})",
                    shadow_case.variable_name,
                    shadow_case.outer_location.line + 1
                ),
                location,
            );
            issues.push(issue);
        }

        debug!("Found {} shadowing cases", issues.len());
        Ok(issues)
    }
}

impl ShadowingRule {
    /// Find variable shadowing cases
    fn find_shadowing(&self, program: &Program) -> Vec<ShadowingCase> {
        // This would need to track variable scopes and detect shadowing
        Vec::new()
    }
}

#[derive(Debug, Clone)]
struct ShadowingCase {
    variable_name: String,
    outer_location: SourceLocation,
    inner_location: SourceLocation,
}

/// Rule to detect unused imports
pub struct UnusedImportRule;

impl UnusedImportRule {
    pub fn new() -> Self {
        Self
    }
}

impl IssueBuilder for UnusedImportRule {}

impl LintRule for UnusedImportRule {
    fn name(&self) -> &'static str {
        "unused-import"
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Correctness
    }

    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Detect imports that are never used"
    }

    fn check_ast(&self, program: &Program, context: &AnalysisContext) -> Result<Vec<LintIssue>, Error> {
        let mut issues = Vec::new();
        let location_helper = LocationHelper::new(&context.source_lines, context.file_name.clone());

        let (imported_items, used_items) = self.collect_import_usage(program);
        
        for (import_name, import_info) in imported_items {
            if !used_items.contains(&import_name) {
                let location = import_info.location;
                let fix = FixSuggestion::deletion("Remove unused import".to_string());
                let issue = self.create_issue_with_fix(
                    self.default_severity(),
                    self.name(),
                    self.category(),
                    format!("Import '{}' is never used", import_name),
                    location,
                    fix,
                );
                issues.push(issue);
            }
        }

        debug!("Found {} unused imports", issues.len());
        Ok(issues)
    }

    fn supports_auto_fix(&self) -> bool {
        true
    }
}

impl UnusedImportRule {
    /// Collect imported and used items
    fn collect_import_usage(&self, program: &Program) -> (HashMap<String, ImportInfo>, HashSet<String>) {
        let mut imported = HashMap::new();
        let mut used = HashSet::new();

        // This would analyze import statements and their usage
        (imported, used)
    }
}

#[derive(Debug, Clone)]
struct ImportInfo {
    location: SourceLocation,
    module: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SourceLocation;

    #[test]
    fn test_unused_variable_rule() {
        let rule = UnusedVariableRule::new();
        assert_eq!(rule.name(), "unused-variable");
        assert_eq!(rule.category(), RuleCategory::Correctness);
        assert_eq!(rule.default_severity(), RuleSeverity::Warning);
        assert!(rule.supports_auto_fix());
    }

    #[test]
    fn test_special_variable_detection() {
        let rule = UnusedVariableRule::new();
        assert!(rule.is_special_variable("_temp"));
        assert!(rule.is_special_variable("err"));
        assert!(rule.is_special_variable("ok"));
        assert!(!rule.is_special_variable("myVar"));
    }

    #[test]
    fn test_unused_function_rule() {
        let rule = UnusedFunctionRule::new();
        assert_eq!(rule.name(), "unused-function");
        assert_eq!(rule.category(), RuleCategory::Correctness);
        assert_eq!(rule.default_severity(), RuleSeverity::Info);
    }

    #[test]
    fn test_special_function_detection() {
        let rule = UnusedFunctionRule::new();
        assert!(rule.is_special_function("main"));
        assert!(rule.is_special_function("test_something"));
        assert!(rule.is_special_function("init_module"));
        assert!(!rule.is_special_function("my_function"));
    }

    #[test]
    fn test_unreachable_code_rule() {
        let rule = UnreachableCodeRule::new();
        assert_eq!(rule.name(), "unreachable-code");
        assert_eq!(rule.category(), RuleCategory::Correctness);
        assert_eq!(rule.default_severity(), RuleSeverity::Warning);
        assert!(rule.supports_auto_fix());
    }

    #[test]
    fn test_dead_code_rule() {
        let rule = DeadCodeRule::new();
        assert_eq!(rule.name(), "dead-code");
        assert_eq!(rule.category(), RuleCategory::Correctness);
        assert_eq!(rule.default_severity(), RuleSeverity::Info);
    }

    #[test]
    fn test_shadowing_rule() {
        let rule = ShadowingRule::new();
        assert_eq!(rule.name(), "shadowing");
        assert_eq!(rule.category(), RuleCategory::Correctness);
        assert_eq!(rule.default_severity(), RuleSeverity::Warning);
    }

    #[test]
    fn test_unused_import_rule() {
        let rule = UnusedImportRule::new();
        assert_eq!(rule.name(), "unused-import");
        assert_eq!(rule.category(), RuleCategory::Correctness);
        assert_eq!(rule.default_severity(), RuleSeverity::Warning);
        assert!(rule.supports_auto_fix());
    }
}
