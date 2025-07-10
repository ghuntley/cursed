//! Linting rules implementation

use super::*;
use crate::ast::*;
use std::collections::HashSet;

/// Style-related linting rules
pub struct StyleRule;

impl StyleRule {
    pub fn new() -> Self {
        Self
    }
}

impl LintRule for StyleRule {
    fn name(&self) -> &'static str {
        "style"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn default_severity(&self) -> Severity {
        Severity::Info
    }

    fn description(&self) -> &'static str {
        "Checks for code style violations and formatting issues"
    }

    fn check(&self, context: &mut LintContext) -> Result<(), CursedError> {
        // Check for style violations
        for statement in &context.ast.statements {
            self.check_statement(statement, context)?;
        }
        Ok(())
    }
}

impl StyleRule {
    fn check_statement(&self, statement: &Statement, context: &mut LintContext) -> Result<(), CursedError> {
        match statement {
            Statement::Let(let_stmt) => {
                let name = let_stmt.target.primary_name();
                self.check_variable_naming(&name, context);
            }
            Statement::Function(func_stmt) => {
                self.check_function_naming(&func_stmt.name, context);
                self.check_function_length(&func_stmt.body, context);
            }
            Statement::Expression(expr) => {
                self.check_expression(expr, context)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn check_variable_naming(&self, name: &str, context: &mut LintContext) {
        // Check for snake_case convention
        if name.contains(char::is_uppercase) && !name.contains('_') {
            context.add_issue(LintIssue {
                rule: "variable_naming".to_string(),
                severity: Severity::Info,
                category: Category::Style,
                message: format!("Variable '{}' should use snake_case naming", name),
                line: 1, // TODO: Get actual line number from AST
                column: 1,
                fix_suggestion: Some(format!("Consider renaming to '{}'", to_snake_case(name))),
            });
        }

        // Check for meaningful names
        if name.len() < 3 && !["i", "j", "k", "x", "y", "z"].contains(&name) {
            context.add_issue(LintIssue {
                rule: "variable_naming".to_string(),
                severity: Severity::Info,
                category: Category::Style,
                message: format!("Variable '{}' has a very short name", name),
                line: 1,
                column: 1,
                fix_suggestion: Some("Consider using a more descriptive name".to_string()),
            });
        }
    }

    fn check_function_naming(&self, name: &str, context: &mut LintContext) {
        // Check for camelCase convention for functions
        if name.contains('_') {
            context.add_issue(LintIssue {
                rule: "function_naming".to_string(),
                severity: Severity::Info,
                category: Category::Style,
                message: format!("Function '{}' should use camelCase naming", name),
                line: 1,
                column: 1,
                fix_suggestion: Some(format!("Consider renaming to '{}'", to_camel_case(name))),
            });
        }
    }

    fn check_function_length(&self, body: &[Statement], context: &mut LintContext) {
        if body.len() > 50 {
            context.add_issue(LintIssue {
                rule: "function_length".to_string(),
                severity: Severity::Warning,
                category: Category::Style,
                message: format!("Function has {} statements, consider breaking it down", body.len()),
                line: 1,
                column: 1,
                fix_suggestion: Some("Consider splitting into smaller functions".to_string()),
            });
        }
    }

    fn check_expression(&self, expr: &Expression, context: &mut LintContext) -> Result<(), CursedError> {
        match expr {
            Expression::Binary(bin_expr) => {
                // Check for complex expressions
                if self.is_complex_expression(&bin_expr.left) && self.is_complex_expression(&bin_expr.right) {
                    context.add_issue(LintIssue {
                        rule: "complex_expression".to_string(),
                        severity: Severity::Info,
                        category: Category::Style,
                        message: "Complex expression, consider using intermediate variables".to_string(),
                        line: 1,
                        column: 1,
                        fix_suggestion: Some("Break down into multiple simpler expressions".to_string()),
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn is_complex_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Binary(_) => true,
            Expression::Call(_) => true,
            _ => false,
        }
    }
}

/// Performance-related linting rules
pub struct PerformanceRule;

impl PerformanceRule {
    pub fn new() -> Self {
        Self
    }
}

impl LintRule for PerformanceRule {
    fn name(&self) -> &'static str {
        "performance"
    }

    fn category(&self) -> Category {
        Category::Performance
    }

    fn default_severity(&self) -> Severity {
        Severity::Warning
    }

    fn description(&self) -> &'static str {
        "Checks for potential performance issues"
    }

    fn check(&self, context: &mut LintContext) -> Result<(), CursedError> {
        for statement in &context.ast.statements {
            self.check_statement(statement, context)?;
        }
        Ok(())
    }
}

impl PerformanceRule {
    fn check_statement(&self, statement: &Statement, context: &mut LintContext) -> Result<(), CursedError> {
        match statement {
            Statement::For(for_stmt) => {
                self.check_loop_performance(for_stmt, context);
            }
            Statement::While(while_stmt) => {
                self.check_while_performance(while_stmt, context);
            }
            Statement::Expression(expr) => {
                self.check_expression_performance(expr, context)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn check_loop_performance(&self, for_stmt: &ForStatement, context: &mut LintContext) {
        // Check for inefficient loop patterns
        if self.has_string_concatenation_in_loop(&for_stmt.body) {
            context.add_issue(LintIssue {
                rule: "string_concatenation_in_loop".to_string(),
                severity: Severity::Warning,
                category: Category::Performance,
                message: "String concatenation in loop can be inefficient".to_string(),
                line: 1,
                column: 1,
                fix_suggestion: Some("Consider using a string builder or collecting into a vector".to_string()),
            });
        }
    }

    fn check_while_performance(&self, while_stmt: &WhileStatement, context: &mut LintContext) {
        // Check for potential infinite loops
        if let Expression::Boolean(true) = &while_stmt.condition {
            context.add_issue(LintIssue {
                rule: "infinite_loop".to_string(),
                severity: Severity::Info,
                category: Category::Performance,
                message: "Infinite loop detected, ensure there's a break condition".to_string(),
                line: 1,
                column: 1,
                fix_suggestion: Some("Add a break condition or use a different loop pattern".to_string()),
            });
        }
    }

    fn check_expression_performance(&self, expr: &Expression, context: &mut LintContext) -> Result<(), CursedError> {
        match expr {
            Expression::Call(call_expr) => {
                // Check for expensive operations in hot paths
                if let Expression::Identifier(name) = call_expr.function.as_ref() {
                    if name == "sleep" || name == "wait" {
                        context.add_issue(LintIssue {
                            rule: "blocking_call".to_string(),
                            severity: Severity::Info,
                            category: Category::Performance,
                            message: "Blocking call detected, consider async alternatives".to_string(),
                            line: 1,
                            column: 1,
                            fix_suggestion: Some("Use async/await or non-blocking alternatives".to_string()),
                        });
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn has_string_concatenation_in_loop(&self, body: &[Statement]) -> bool {
        for statement in body {
            if let Statement::Expression(Expression::Binary(bin_expr)) = statement {
                if bin_expr.operator == "+" {
                    return true;
                }
            }
        }
        false
    }
}

/// Security-related linting rules
pub struct SecurityRule;

impl SecurityRule {
    pub fn new() -> Self {
        Self
    }
}

impl LintRule for SecurityRule {
    fn name(&self) -> &'static str {
        "security"
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn default_severity(&self) -> Severity {
        Severity::Error
    }

    fn description(&self) -> &'static str {
        "Checks for potential security vulnerabilities"
    }

    fn check(&self, context: &mut LintContext) -> Result<(), CursedError> {
        for statement in &context.ast.statements {
            self.check_statement(statement, context)?;
        }
        Ok(())
    }
}

impl SecurityRule {
    fn check_statement(&self, statement: &Statement, context: &mut LintContext) -> Result<(), CursedError> {
        match statement {
            Statement::Expression(expr) => {
                self.check_expression_security(expr, context)?;
            }
            Statement::Let(let_stmt) => {
                let name = let_stmt.target.primary_name();
                self.check_variable_security(&name, context);
            }
            _ => {}
        }
        Ok(())
    }

    fn check_expression_security(&self, expr: &Expression, context: &mut LintContext) -> Result<(), CursedError> {
        match expr {
            Expression::Call(call_expr) => {
                if let Expression::Identifier(name) = call_expr.function.as_ref() {
                    // Check for dangerous functions
                    if ["eval", "exec", "system", "shell"].contains(&name.as_str()) {
                        context.add_issue(LintIssue {
                            rule: "dangerous_function".to_string(),
                            severity: Severity::Error,
                            category: Category::Security,
                            message: format!("Dangerous function '{}' can lead to code injection", name),
                            line: 1,
                            column: 1,
                            fix_suggestion: Some("Use safer alternatives or validate input thoroughly".to_string()),
                        });
                    }

                    // Check for crypto functions
                    if ["md5", "sha1"].contains(&name.as_str()) {
                        context.add_issue(LintIssue {
                            rule: "weak_crypto".to_string(),
                            severity: Severity::Warning,
                            category: Category::Security,
                            message: format!("Weak cryptographic function '{}' is not recommended", name),
                            line: 1,
                            column: 1,
                            fix_suggestion: Some("Use SHA-256 or stronger cryptographic functions".to_string()),
                        });
                    }
                }
            }
            Expression::String(s) => {
                // Check for hardcoded credentials
                if self.contains_potential_credential(s) {
                    context.add_issue(LintIssue {
                        rule: "hardcoded_credential".to_string(),
                        severity: Severity::Error,
                        category: Category::Security,
                        message: "Potential hardcoded credential detected".to_string(),
                        line: 1,
                        column: 1,
                        fix_suggestion: Some("Use environment variables or secure configuration".to_string()),
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn check_variable_security(&self, name: &str, context: &mut LintContext) {
        // Check for variables that might contain sensitive data
        let sensitive_patterns = ["password", "secret", "key", "token", "credential"];
        if sensitive_patterns.iter().any(|&pattern| name.to_lowercase().contains(pattern)) {
            context.add_issue(LintIssue {
                rule: "sensitive_variable".to_string(),
                severity: Severity::Warning,
                category: Category::Security,
                message: format!("Variable '{}' may contain sensitive data", name),
                line: 1,
                column: 1,
                fix_suggestion: Some("Ensure sensitive data is properly protected".to_string()),
            });
        }
    }

    fn contains_potential_credential(&self, s: &str) -> bool {
        let patterns = ["password=", "secret=", "key=", "token=", "api_key="];
        patterns.iter().any(|&pattern| s.to_lowercase().contains(pattern))
    }
}

/// Correctness-related linting rules
pub struct CorrectnessRule;

impl CorrectnessRule {
    pub fn new() -> Self {
        Self
    }
}

impl LintRule for CorrectnessRule {
    fn name(&self) -> &'static str {
        "correctness"
    }

    fn category(&self) -> Category {
        Category::Correctness
    }

    fn default_severity(&self) -> Severity {
        Severity::Error
    }

    fn description(&self) -> &'static str {
        "Checks for potential correctness issues and bugs"
    }

    fn check(&self, context: &mut LintContext) -> Result<(), CursedError> {
        let mut used_vars = HashSet::new();
        let mut defined_vars = HashSet::new();

        for statement in &context.ast.statements {
            self.check_statement(statement, context, &mut used_vars, &mut defined_vars)?;
        }

        // Check for unused variables
        for var in &defined_vars {
            if !used_vars.contains(var) {
                context.add_issue(LintIssue {
                    rule: "unused_variable".to_string(),
                    severity: Severity::Warning,
                    category: Category::Correctness,
                    message: format!("Variable '{}' is defined but never used", var),
                    line: 1,
                    column: 1,
                    fix_suggestion: Some("Remove unused variable or use it".to_string()),
                });
            }
        }

        Ok(())
    }
}

impl CorrectnessRule {
    fn check_statement(
        &self,
        statement: &Statement,
        context: &mut LintContext,
        used_vars: &mut HashSet<String>,
        defined_vars: &mut HashSet<String>,
    ) -> Result<(), CursedError> {
        match statement {
            Statement::Let(let_stmt) => {
                let name = let_stmt.target.primary_name();
                defined_vars.insert(name);
                self.collect_used_variables(&let_stmt.value, used_vars);
            }
            Statement::Assignment(assign_stmt) => {
                if let AssignmentTarget::Single(name) = &assign_stmt.target {
                    used_vars.insert(name.clone());
                }
                self.collect_used_variables(&assign_stmt.value, used_vars);
            }
            Statement::Expression(expr) => {
                self.collect_used_variables(expr, used_vars);
            }
            Statement::Return(ret_stmt) => {
                if let Some(value) = &ret_stmt.value {
                    self.collect_used_variables(value, used_vars);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn collect_used_variables(&self, expr: &Expression, used_vars: &mut HashSet<String>) {
        match expr {
            Expression::Identifier(name) => {
                used_vars.insert(name.clone());
            }
            Expression::Binary(bin_expr) => {
                self.collect_used_variables(&bin_expr.left, used_vars);
                self.collect_used_variables(&bin_expr.right, used_vars);
            }
            Expression::Call(call_expr) => {
                self.collect_used_variables(&call_expr.function, used_vars);
                for arg in &call_expr.arguments {
                    self.collect_used_variables(arg, used_vars);
                }
            }
            _ => {}
        }
    }
}

/// Best practice linting rules
pub struct BestPracticeRule;

impl BestPracticeRule {
    pub fn new() -> Self {
        Self
    }
}

impl LintRule for BestPracticeRule {
    fn name(&self) -> &'static str {
        "best_practice"
    }

    fn category(&self) -> Category {
        Category::BestPractice
    }

    fn default_severity(&self) -> Severity {
        Severity::Info
    }

    fn description(&self) -> &'static str {
        "Checks for best practice violations"
    }

    fn check(&self, context: &mut LintContext) -> Result<(), CursedError> {
        for statement in &context.ast.statements {
            self.check_statement(statement, context)?;
        }
        Ok(())
    }
}

impl BestPracticeRule {
    fn check_statement(&self, statement: &Statement, context: &mut LintContext) -> Result<(), CursedError> {
        match statement {
            Statement::Function(func_stmt) => {
                self.check_function_best_practices(func_stmt, context);
            }
            Statement::Expression(expr) => {
                self.check_expression_best_practices(expr, context)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn check_function_best_practices(&self, func_stmt: &FunctionStatement, context: &mut LintContext) {
        // Check for missing documentation
        if func_stmt.name != "main" && !self.has_documentation(&func_stmt.body) {
            context.add_issue(LintIssue {
                rule: "missing_documentation".to_string(),
                severity: Severity::Info,
                category: Category::BestPractice,
                message: format!("Function '{}' should have documentation", func_stmt.name),
                line: 1,
                column: 1,
                fix_suggestion: Some("Add a comment explaining what the function does".to_string()),
            });
        }

        // Check for single responsibility
        if func_stmt.body.len() > 20 {
            context.add_issue(LintIssue {
                rule: "single_responsibility".to_string(),
                severity: Severity::Info,
                category: Category::BestPractice,
                message: "Function is doing too many things, consider splitting".to_string(),
                line: 1,
                column: 1,
                fix_suggestion: Some("Break down into smaller, focused functions".to_string()),
            });
        }
    }

    fn check_expression_best_practices(&self, expr: &Expression, context: &mut LintContext) -> Result<(), CursedError> {
        match expr {
            Expression::Integer(n) => {
                // Check for magic numbers
                if *n != 0 && *n != 1 && *n != -1 {
                    context.add_issue(LintIssue {
                        rule: "magic_number".to_string(),
                        severity: Severity::Info,
                        category: Category::BestPractice,
                        message: format!("Magic number {} should be a named constant", n),
                        line: 1,
                        column: 1,
                        fix_suggestion: Some("Define as a named constant".to_string()),
                    });
                }
            }
            Expression::Float(n) => {
                // Check for magic numbers
                if *n != 0.0 && *n != 1.0 && *n != -1.0 {
                    context.add_issue(LintIssue {
                        rule: "magic_number".to_string(),
                        severity: Severity::Info,
                        category: Category::BestPractice,
                        message: format!("Magic number {} should be a named constant", n),
                        line: 1,
                        column: 1,
                        fix_suggestion: Some("Define as a named constant".to_string()),
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn has_documentation(&self, body: &[Statement]) -> bool {
        // Simple check for comments - in a real implementation,
        // this would check for actual documentation comments
        body.len() > 0 // Placeholder
    }
}

// Helper functions for string case conversion
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    
    result
}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case_conversion() {
        assert_eq!(to_snake_case("CamelCase"), "camel_case");
        assert_eq!(to_snake_case("HTMLParser"), "h_t_m_l_parser");
        assert_eq!(to_snake_case("simple"), "simple");
    }

    #[test]
    fn test_camel_case_conversion() {
        assert_eq!(to_camel_case("snake_case"), "snakeCase");
        assert_eq!(to_camel_case("simple"), "simple");
        assert_eq!(to_camel_case("multiple_words_here"), "multipleWordsHere");
    }

    #[test]
    fn test_rule_names() {
        let style = StyleRule::new();
        let perf = PerformanceRule::new();
        let security = SecurityRule::new();
        let correctness = CorrectnessRule::new();
        let best_practice = BestPracticeRule::new();

        assert_eq!(style.name(), "style");
        assert_eq!(perf.name(), "performance");
        assert_eq!(security.name(), "security");
        assert_eq!(correctness.name(), "correctness");
        assert_eq!(best_practice.name(), "best_practice");
    }
}
