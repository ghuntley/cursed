//! Simplified CURSED Linter with AstVisitor implementation

use crate::error::CursedError;
use crate::ast::{Program, Statement, Expression, Literal, BinaryOperator, UnaryOperator, Type, AstVisitor};
use crate::lexer::{Lexer, Token, TokenKind};
use std::collections::HashMap;
use std::path::Path;

use super::linter::*;

/// Simplified implementation of AstVisitor trait for CursedLinter
impl AstVisitor<Vec<LintIssue>> for CursedLinter {
    fn visit_program(&mut self, program: &Program) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        // Reset context for new program analysis
        self.context.scope_depth = 0;
        self.context.variable_usage.clear();
        self.context.functions.clear();
        self.context.imports.clear();
        self.context.slang_usage = SlangUsageStats::default();
        
        // Collect import statements
        for import in &program.imports {
            self.context.imports.push(import.clone());
        }
        
        // Validate package structure if present
        if let Some(package) = &program.package {
            if let Some(issue) = self.check_package_naming_simplified(&package.name) {
                issues.push(issue);
            }
        }
        
        // Visit all statements in the program
        for statement in &program.statements {
            issues.extend(self.visit_statement(statement));
        }
        
        // Perform global analysis after visiting all statements
        issues.extend(self.perform_global_analysis_simplified());
        
        issues
    }
    
    fn visit_statement(&mut self, statement: &Statement) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        match statement {
            Statement::Function(func_stmt) => {
                // Check function naming conventions
                if let Some(issue) = self.check_function_naming(&func_stmt.name, 1).unwrap_or(None) {
                    issues.push(issue);
                }
                
                // Track function info
                let complexity = self.calculate_function_complexity(&func_stmt.body);
                self.context.functions.insert(func_stmt.name.clone(), FunctionInfo {
                    name: func_stmt.name.clone(),
                    declared_line: 1,
                    param_count: func_stmt.parameters.len(),
                    complexity,
                    proper_slang_naming: self.has_proper_slang_naming(&func_stmt.name),
                });
                
                // Check function complexity
                if complexity > self.config.max_function_complexity {
                    issues.push(self.create_complexity_issue(&func_stmt.name, complexity));
                }
                
                // Enter function scope and visit body
                self.context.scope_depth += 1;
                for stmt in &func_stmt.body {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
            },
            
            Statement::Let(let_stmt) => {
                // Check variable naming
                if let Some(issue) = self.check_variable_naming(&let_stmt.name, 1).unwrap_or(None) {
                    issues.push(issue);
                }
                
                // Track variable declaration
                self.context.variable_usage.insert(let_stmt.name.clone(), VariableInfo {
                    name: let_stmt.name.clone(),
                    declared_line: 1,
                    usage_count: 0,
                    is_mutable: false, // CURSED doesn't have explicit mutability in AST
                    var_type: None, // Simplified - would map from var_type field
                });
                
                // Visit the value expression
                issues.extend(self.visit_expression(&let_stmt.value));
            },
            
            Statement::If(if_stmt) => {
                // Visit condition expression
                issues.extend(self.visit_expression(&if_stmt.condition));
                
                // Visit then branch
                self.context.scope_depth += 1;
                for stmt in &if_stmt.then_branch {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
                
                // Visit else branch if present
                if let Some(else_stmts) = &if_stmt.else_branch {
                    self.context.scope_depth += 1;
                    for stmt in else_stmts {
                        issues.extend(self.visit_statement(stmt));
                    }
                    self.context.scope_depth -= 1;
                }
            },
            
            Statement::While(while_stmt) => {
                // Visit condition and body
                issues.extend(self.visit_expression(&while_stmt.condition));
                
                self.context.scope_depth += 1;
                for stmt in &while_stmt.body {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
            },
            
            Statement::For(for_stmt) => {
                // Visit condition if present
                if let Some(condition) = &for_stmt.condition {
                    issues.extend(self.visit_expression(condition));
                }
                
                // Visit init if present
                if let Some(init) = &for_stmt.init {
                    issues.extend(self.visit_statement(init));
                }
                
                // Visit update if present
                if let Some(update) = &for_stmt.update {
                    issues.extend(self.visit_expression(update));
                }
                
                // Visit body
                self.context.scope_depth += 1;
                for stmt in &for_stmt.body {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
            },
            
            Statement::Return(return_stmt) => {
                // Visit return value if present
                if let Some(expr) = &return_stmt.value {
                    issues.extend(self.visit_expression(expr));
                }
            },
            
            Statement::Expression(expr) => {
                // Visit the expression
                issues.extend(self.visit_expression(expr));
            },
            
            Statement::Panic(panic_stmt) => {
                // Visit the panic message expression
                issues.extend(self.visit_expression(&panic_stmt.message));
            },
            
            Statement::Catch(catch_stmt) => {
                // Visit protected block
                self.context.scope_depth += 1;
                for stmt in &catch_stmt.protected_block {
                    issues.extend(self.visit_statement(stmt));
                }
                self.context.scope_depth -= 1;
                
                // Visit recovery block if present
                if let Some(recovery_stmts) = &catch_stmt.recovery_block {
                    self.context.scope_depth += 1;
                    for stmt in recovery_stmts {
                        issues.extend(self.visit_statement(stmt));
                    }
                    self.context.scope_depth -= 1;
                }
            },
            
            _ => {
                // Handle other statement types
            }
        }
        
        issues
    }
    
    fn visit_expression(&mut self, expression: &Expression) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        match expression {
            Expression::Identifier(name) => {
                // Track variable usage
                if let Some(var_info) = self.context.variable_usage.get_mut(name) {
                    var_info.usage_count += 1;
                }
                
                // Check for undefined variables
                if !self.context.variable_usage.contains_key(name) && !self.is_builtin_identifier(name) {
                    issues.push(self.create_undefined_variable_issue(name));
                }
            },
            
            Expression::Literal(literal) => {
                // Check literal patterns
                if let Some(issue) = self.check_literal_patterns(literal) {
                    issues.push(issue);
                }
            },
            
            Expression::Binary(binary_expr) => {
                // Visit both operands
                issues.extend(self.visit_expression(&binary_expr.left));
                issues.extend(self.visit_expression(&binary_expr.right));
                
                // Check for operator-specific issues (simplified)
                if let Some(issue) = self.check_binary_operator_string_patterns(&binary_expr.operator, &binary_expr.left, &binary_expr.right) {
                    issues.push(issue);
                }
            },
            
            Expression::Unary(unary_expr) => {
                // Visit operand
                issues.extend(self.visit_expression(&unary_expr.operand));
            },
            
            Expression::Call(call_expr) => {
                // Visit function expression
                issues.extend(self.visit_expression(&call_expr.function));
                
                // Visit all arguments
                for arg in &call_expr.arguments {
                    issues.extend(self.visit_expression(arg));
                }
            },
            
            Expression::Array(elements) => {
                // Visit all elements
                for element in elements {
                    issues.extend(self.visit_expression(element));
                }
            },
            
            Expression::Map(pairs) => {
                // Visit all key-value pairs
                for (key, value) in pairs {
                    issues.extend(self.visit_expression(key));
                    issues.extend(self.visit_expression(value));
                }
            },
            
            Expression::MemberAccess(member_expr) => {
                // Visit object expression
                issues.extend(self.visit_expression(&member_expr.object));
            },
            
            _ => {
                // Handle other expression types
            }
        }
        
        issues
    }
}

// Additional simplified helper methods for the AstVisitor implementation
impl CursedLinter {
    /// Simplified package naming check
    fn check_package_naming_simplified(&self, package_name: &str) -> Option<LintIssue> {
        if package_name.is_empty() {
            return Some(LintIssue {
                rule_id: "package_naming".to_string(),
                category: LintCategory::Naming,
                severity: LintSeverity::Error,
                message: "Package name cannot be empty".to_string(),
                file_path: self.context.current_file.clone(),
                line: 1,
                column: 1,
                length: 0,
                suggestion: Some("Provide a valid package name".to_string()),
                context: HashMap::new(),
            });
        }
        None
    }
    
    /// Simplified global analysis
    fn perform_global_analysis_simplified(&self) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        // Check for unused variables
        if self.rules.get("unused_variables").map_or(false, |r| r.enabled) {
            issues.extend(self.check_unused_variables().unwrap_or_default());
        }
        
        issues
    }
    
    /// Check binary operator patterns (string version for CURSED AST)
    fn check_binary_operator_string_patterns(&self, op: &str, left: &Expression, right: &Expression) -> Option<LintIssue> {
        // Check for division by zero
        if op == "/" {
            if let Expression::Integer(n) = right {
                if *n == 0 {
                    return Some(LintIssue {
                        rule_id: "division_by_zero".to_string(),
                        category: LintCategory::Correctness,
                        severity: LintSeverity::Error,
                        message: "Division by zero".to_string(),
                        file_path: self.context.current_file.clone(),
                        line: 1,
                        column: 1,
                        length: 1,
                        suggestion: Some("Check for zero before division".to_string()),
                        context: HashMap::new(),
                    });
                }
            }
        }
        
        None
    }
}
