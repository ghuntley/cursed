//! Subset validation logic for bootstrap compilation
//!
//! This module provides functionality to validate that a CURSED program
//! uses only the features available in the bootstrap subset.

use std::collections::HashSet;
use crate::ast::base::Program;
use crate::ast::traits::{Expression, Node, Statement};
use crate::bootstrap_minimal::subset::BootstrapSubset;
use crate::error::{Error, SourceLocation};
use tracing::{debug, info, warn};

/// Validation result containing information about subset compliance
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the program is valid for bootstrap compilation
    pub is_valid: bool,
    /// List of validation errors found
    pub errors: Vec<ValidationError>,
    /// List of warnings (features that work but are not optimal)
    pub warnings: Vec<ValidationWarning>,
    /// Statistics about the validation
    pub stats: ValidationStats,
}

/// A validation error indicating use of non-bootstrap features
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Description of the error
    pub message: String,
    /// Source location where the error occurred
    pub location: Option<SourceLocation>,
    /// Type of feature that caused the error
    pub feature_type: String,
    /// Suggested alternative if available
    pub suggestion: Option<String>,
}

/// A validation warning for suboptimal but allowed features
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// Description of the warning
    pub message: String,
    /// Source location where the warning occurred
    pub location: Option<SourceLocation>,
    /// Suggested improvement
    pub suggestion: Option<String>,
}

/// Statistics about the validation process
#[derive(Debug, Clone, Default)]
pub struct ValidationStats {
    /// Total number of statements validated
    pub statements_checked: usize,
    /// Total number of expressions validated
    pub expressions_checked: usize,
    /// Number of function definitions found
    pub functions_found: usize,
    /// Number of import statements found
    pub imports_found: usize,
    /// Number of different expression types used
    pub expression_types: HashSet<String>,
    /// Number of different statement types used
    pub statement_types: HashSet<String>,
}

/// Validator for checking bootstrap subset compliance
pub struct SubsetValidator {
    /// The bootstrap subset definition to validate against
    subset: BootstrapSubset,
    /// Accumulated validation errors
    errors: Vec<ValidationError>,
    /// Accumulated validation warnings
    warnings: Vec<ValidationWarning>,
    /// Statistics being gathered
    stats: ValidationStats,
}

impl SubsetValidator {
    /// Creates a new subset validator
    pub fn new() -> Self {
        SubsetValidator {
            subset: BootstrapSubset::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        }
    }
    
    /// Creates a validator with a custom subset definition
    pub fn with_subset(subset: BootstrapSubset) -> Self {
        SubsetValidator {
            subset,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        }
    }
    
    /// Validates a complete program against the bootstrap subset
    pub fn validate_program(&mut self, program: &Program) -> ValidationResult {
        info!("Starting bootstrap subset validation");
        
        // Reset state
        self.errors.clear();
        self.warnings.clear();
        self.stats = ValidationStats::default();
        
        // Validate the overall subset definition first
        if let Err(msg) = self.subset.validate_subset() {
            self.errors.push(ValidationError {
                message: format!("Invalid subset definition: {}", msg),
                location: None,
                feature_type: "SubsetDefinition".to_string(),
                suggestion: None,
            });
        }
        
        // Validate each statement in the program
        for statement in &program.statements {
            self.validate_statement(statement);
        }
        
        // Check for required elements
        self.check_required_elements(program);
        
        let is_valid = self.errors.is_empty();
        
        info!(
            is_valid = is_valid,
            errors = self.errors.len(),
            warnings = self.warnings.len(),
            statements = self.stats.statements_checked,
            expressions = self.stats.expressions_checked,
            "Bootstrap subset validation completed"
        );
        
        ValidationResult {
            is_valid,
            errors: self.errors.clone(),
            warnings: self.warnings.clone(),
            stats: self.stats.clone(),
        }
    }
    
    /// Validates a single statement
    fn validate_statement(&mut self, statement: &dyn Statement) {
        self.stats.statements_checked += 1;
        
        let stmt_type = statement.node_type();
        self.stats.statement_types.insert(stmt_type.clone());
        
        debug!("Validating statement type: {}", stmt_type);
        
        if !self.subset.is_statement_allowed(&stmt_type) {
            self.errors.push(ValidationError {
                message: format!("Statement type '{}' is not allowed in bootstrap subset", stmt_type),
                location: self.get_statement_location(statement),
                feature_type: "Statement".to_string(),
                suggestion: self.suggest_statement_alternative(&stmt_type),
            });
            return;
        }
        
        // Validate specific statement types
        match stmt_type.as_str() {
            "FunctionStatement" => self.validate_function_statement(statement),
            "IfStatement" => self.validate_if_statement(statement),
            "ForStatement" => self.validate_for_statement(statement),
            "ExpressionStatement" => self.validate_expression_statement(statement),
            "VarStatement" => self.validate_var_statement(statement),
            "AssignmentStatement" => self.validate_assignment_statement(statement),
            _ => {} // Other allowed statements don't need special validation
        }
    }
    
    /// Validates an expression and its sub-expressions
    fn validate_expression(&mut self, expression: &dyn Expression) {
        self.stats.expressions_checked += 1;
        
        let expr_type = expression.node_type();
        self.stats.expression_types.insert(expr_type.clone());
        
        debug!("Validating expression type: {}", expr_type);
        
        if !self.subset.is_expression_allowed(&expr_type) {
            self.errors.push(ValidationError {
                message: format!("Expression type '{}' is not allowed in bootstrap subset", expr_type),
                location: self.get_expression_location(expression),
                feature_type: "Expression".to_string(),
                suggestion: self.suggest_expression_alternative(&expr_type),
            });
            return;
        }
        
        // Validate specific expression types that may have restrictions
        match expr_type.as_str() {
            "CallExpression" => self.validate_call_expression(expression),
            "DotExpression" => self.validate_dot_expression(expression),
            "ArrayLiteral" => self.validate_array_literal(expression),
            _ => {} // Other expressions are validated recursively
        }
    }
    
    /// Validates a function statement
    fn validate_function_statement(&mut self, _statement: &dyn Statement) {
        self.stats.functions_found += 1;
        
        // For now, all function statements are allowed
        // In the future, we might restrict certain function features
        // like variadic parameters, generics, etc.
    }
    
    /// Validates an if statement
    fn validate_if_statement(&mut self, _statement: &dyn Statement) {
        // Basic if statements are allowed
        // Complex features like type switching would be rejected elsewhere
    }
    
    /// Validates a for statement
    fn validate_for_statement(&mut self, _statement: &dyn Statement) {
        // Basic for loops are allowed
        // Range expressions might need validation
    }
    
    /// Validates an expression statement
    fn validate_expression_statement(&mut self, statement: &dyn Statement) {
        // Extract and validate the contained expression
        // This is a simplified implementation - in practice we'd need
        // to access the actual expression field
        debug!("Validating expression statement: {}", statement.string());
    }
    
    /// Validates a variable declaration statement
    fn validate_var_statement(&mut self, _statement: &dyn Statement) {
        // Basic variable declarations are allowed
        // Complex types would be caught at the type level
    }
    
    /// Validates an assignment statement
    fn validate_assignment_statement(&mut self, _statement: &dyn Statement) {
        // Basic assignments are allowed
    }
    
    /// Validates a function call expression
    fn validate_call_expression(&mut self, _expression: &dyn Expression) {
        // Basic function calls are allowed
        // We might want to restrict certain built-in functions
    }
    
    /// Validates a dot expression (property access)
    fn validate_dot_expression(&mut self, expression: &dyn Expression) {
        // Dot expressions are essential for stdlib access
        // Validate that it's being used for allowed purposes
        let expr_str = expression.string();
        
        // Check if it's accessing allowed standard library modules
        let allowed_stdlib_modules = vec!["vibez", "mathz", "stringz", "timez"];
        
        let mut is_stdlib_access = false;
        for module in &allowed_stdlib_modules {
            if expr_str.starts_with(module) {
                is_stdlib_access = true;
                break;
            }
        }
        
        if !is_stdlib_access {
            self.warnings.push(ValidationWarning {
                message: format!("Dot expression '{}' may not be accessing standard library", expr_str),
                location: self.get_expression_location(expression),
                suggestion: Some("Use vibez.spill(), mathz.add(), etc. for standard library access".to_string()),
            });
        }
    }
    
    /// Validates an array literal
    fn validate_array_literal(&mut self, _expression: &dyn Expression) {
        // Basic array literals are allowed for bootstrap
        // Complex array operations might be restricted
    }
    
    /// Checks that the program contains required elements for compilation
    fn check_required_elements(&mut self, program: &Program) {
        let program_str = program.string();
        
        // Check for package declaration
        if !program_str.contains("vibe ") && !program_str.contains("package ") {
            self.errors.push(ValidationError {
                message: "Bootstrap programs must have a package declaration (vibe statement)".to_string(),
                location: None,
                feature_type: "RequiredElement".to_string(),
                suggestion: Some("Add 'vibe main' at the top of your program".to_string()),
            });
        }
        
        // Check for main function
        if self.stats.functions_found == 0 {
            self.warnings.push(ValidationWarning {
                message: "No functions found - bootstrap programs typically need a main function".to_string(),
                location: None,
                suggestion: Some("Add 'slay main() { ... }' to define the program entry point".to_string()),
            });
        }
        
        // Validate minimum complexity for self-hosting
        if self.stats.statements_checked < 10 {
            self.warnings.push(ValidationWarning {
                message: "Program may be too simple for self-hosting compilation".to_string(),
                location: None,
                suggestion: Some("Bootstrap compilers typically need more complexity to be self-hosting".to_string()),
            });
        }
    }
    
    /// Gets the source location of a statement (placeholder implementation)
    fn get_statement_location(&self, _statement: &dyn Statement) -> Option<SourceLocation> {
        // In a real implementation, we would extract location information from the AST
        None
    }
    
    /// Gets the source location of an expression (placeholder implementation) 
    fn get_expression_location(&self, _expression: &dyn Expression) -> Option<SourceLocation> {
        // In a real implementation, we would extract location information from the AST
        None
    }
    
    /// Suggests an alternative for a disallowed statement type
    fn suggest_statement_alternative(&self, stmt_type: &str) -> Option<String> {
        match stmt_type {
            "SwitchStatement" => Some("Use if/else chains instead of switch statements".to_string()),
            "SelectStatement" => Some("Channel select is not available in bootstrap subset".to_string()),
            "StructStatement" => Some("Use simple variables instead of structs".to_string()),
            "InterfaceStatement" => Some("Interfaces are not available in bootstrap subset".to_string()),
            "MethodStatement" => Some("Use standalone functions instead of methods".to_string()),
            _ => None,
        }
    }
    
    /// Suggests an alternative for a disallowed expression type
    fn suggest_expression_alternative(&self, expr_type: &str) -> Option<String> {
        match expr_type {
            "StructLiteral" => Some("Use individual variables instead of struct literals".to_string()),
            "ChannelExpression" => Some("Channels are not available in bootstrap subset".to_string()),
            "TypeAssertion" => Some("Type assertions are not available in bootstrap subset".to_string()),
            "MethodCall" => Some("Use function calls instead of method calls".to_string()),
            _ => None,
        }
    }
}

impl Default for SubsetValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::base::Program;
    
    #[test]
    fn test_validator_creation() {
        let validator = SubsetValidator::new();
        assert_eq!(validator.errors.len(), 0);
        assert_eq!(validator.warnings.len(), 0);
    }
    
    #[test]
    fn test_empty_program_validation() {
        let mut validator = SubsetValidator::new();
        let program = Program::new(Vec::new());
        
        let result = validator.validate_program(&program);
        
        // Empty program should have warnings but may not be invalid
        assert!(result.warnings.len() > 0);
        assert_eq!(result.stats.statements_checked, 0);
    }
    
    #[test]
    fn test_validation_stats() {
        let mut validator = SubsetValidator::new();
        let program = Program::new(Vec::new());
        
        let result = validator.validate_program(&program);
        
        assert_eq!(result.stats.statements_checked, 0);
        assert_eq!(result.stats.expressions_checked, 0);
        assert_eq!(result.stats.functions_found, 0);
    }
}
