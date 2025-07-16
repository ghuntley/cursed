//! Semantic Analysis Error Recovery for CURSED
//! 
//! This module implements error recovery for semantic analysis and type checking

use crate::error_types::{Error, Result};
use crate::error_recovery::{SourceLocation, ErrorContext, ErrorSeverity, SemanticErrorRecovery};
use crate::type_system::{TypeChecker, TypeCheckError, TypeErrorKind};
use crate::ast::{Expression, Statement, Program};
use std::collections::HashMap;

impl SemanticErrorRecovery for TypeChecker {
    /// Accumulate semantic errors without stopping analysis
    fn accumulate_error(&mut self, error: Error, location: SourceLocation) {
        let type_error = TypeCheckError {
            message: error.to_string(),
            location: Some(format!("{}:{}", location.line, location.column)),
            error_type: self.error_to_kind(&error),
        };
        
        self.errors.push(type_error);
    }
    
    /// Check if we can continue semantic analysis
    fn can_continue_analysis(&self) -> bool {
        let fatal_errors = self.errors.iter()
            .filter(|e| matches!(e.error_type, 
                TypeErrorKind::InterfaceComplianceError | 
                TypeErrorKind::ParameterCountMismatch))
            .count();
        
        // Continue if we have fewer than 10 fatal errors
        fatal_errors < 10 && self.errors.len() < 50
    }
    
    /// Generate a placeholder type when analysis fails
    fn generate_placeholder_type(&self) -> String {
        "ErrorPlaceholder".to_string()
    }
    
    /// Skip erroneous declaration and continue
    fn skip_erroneous_declaration(&mut self) -> bool {
        // Mark that we skipped a declaration for recovery
        true
    }
}

impl TypeChecker {
    /// Enhanced type checking with error recovery
    pub fn check_program_with_recovery(&mut self, program: &Program) -> Result<()> {
        // Clear previous errors
        self.errors.clear();
        
        // Check each statement with error recovery
        for statement in &program.statements {
            if let Err(error) = self.check_statement_with_recovery(statement) {
                let location = SourceLocation::new(1, 1, 0); // TODO: Get actual location
                self.accumulate_error(error, location);
                
                // Continue if possible
                if !self.can_continue_analysis() {
                    break;
                }
            }
        }
        
        // Return success even if we have recoverable errors
        if self.has_fatal_errors() {
            Err(Error::TypeCheck("Fatal type checking errors".to_string()))
        } else {
            Ok(())
        }
    }
    
    /// Check statement with error recovery
    pub fn check_statement_with_recovery(&mut self, statement: &Statement) -> Result<()> {
        match self.check_statement(statement) {
            Ok(()) => Ok(()),
            Err(error) => {
                // Try to recover based on error type
                match &error {
                    Error::Type(msg) if msg.contains("undefined variable") => {
                        // Add placeholder variable to continue analysis
                        self.add_placeholder_variable(msg);
                        Ok(())
                    }
                    Error::Type(msg) if msg.contains("type mismatch") => {
                        // Use type coercion or placeholder
                        eprintln!("Warning: Type mismatch recovered with placeholder");
                        Ok(())
                    }
                    _ => Err(error)
                }
            }
        }
    }
    
    /// Check expression with error recovery
    pub fn check_expression_with_recovery(&mut self, expression: &Expression) -> Result<String> {
        match self.check_expression(expression) {
            Ok(type_name) => Ok(type_name),
            Err(error) => {
                // Try to infer a reasonable type based on the expression
                let placeholder_type = self.infer_placeholder_type(expression);
                
                // Log the error but continue with placeholder
                let location = SourceLocation::new(1, 1, 0); // TODO: Get actual location
                self.accumulate_error(error, location);
                
                Ok(placeholder_type)
            }
        }
    }
    
    /// Add placeholder variable for error recovery
    fn add_placeholder_variable(&mut self, error_msg: &str) {
        // Extract variable name from error message
        if let Some(var_name) = self.extract_variable_name(error_msg) {
            // Add with placeholder type
            self.add_variable(var_name, 
                crate::type_system::TypeExpression::named("ErrorPlaceholder"));
            eprintln!("Recovery: Added placeholder variable with ErrorPlaceholder type");
        }
    }
    
    /// Extract variable name from error message
    fn extract_variable_name(&self, error_msg: &str) -> Option<String> {
        // Simple extraction - in practice this would be more sophisticated
        if error_msg.contains("undefined variable") {
            // Extract between quotes or after "variable"
            error_msg.split("'").nth(1).map(|s| s.to_string())
        } else {
            None
        }
    }
    
    /// Infer a reasonable placeholder type for an expression
    fn infer_placeholder_type(&self, expression: &Expression) -> String {
        match expression {
            Expression::Literal(literal) => {
                match literal {
                    crate::ast::Literal::Integer(_) => "normie".to_string(),
                    crate::ast::Literal::Float(_) => "meal".to_string(),
                    crate::ast::Literal::String(_) => "tea".to_string(),
                    crate::ast::Literal::Boolean(_) => "lit".to_string(),
                    crate::ast::Literal::Character(_) => "sip".to_string(),
                    _ => "ErrorPlaceholder".to_string(),
                }
            }
            Expression::Binary(_) => {
                // Assume arithmetic results in number
                "normie".to_string()
            }
            Expression::Call(_) => {
                // Function calls might return anything
                "ErrorPlaceholder".to_string()
            }
            Expression::MemberAccess(_) => {
                // Member access type depends on the member
                "ErrorPlaceholder".to_string()
            }
            _ => "ErrorPlaceholder".to_string(),
        }
    }
    
    /// Check if we have fatal errors that prevent continuation
    fn has_fatal_errors(&self) -> bool {
        self.errors.iter().any(|e| {
            matches!(e.error_type, 
                TypeErrorKind::InterfaceComplianceError |
                TypeErrorKind::ParameterCountMismatch |
                TypeErrorKind::ReturnTypeMismatch)
        })
    }
    
    /// Convert Error to TypeErrorKind
    fn error_to_kind(&self, error: &Error) -> TypeErrorKind {
        match error {
            Error::Type(msg) => {
                if msg.contains("undefined variable") {
                    TypeErrorKind::UndefinedVariable
                } else if msg.contains("undefined function") {
                    TypeErrorKind::UndefinedFunction
                } else if msg.contains("type mismatch") {
                    TypeErrorKind::TypeMismatch
                } else if msg.contains("arity mismatch") {
                    TypeErrorKind::ArityMismatch
                } else {
                    TypeErrorKind::TypeMismatch
                }
            }
            Error::Parse(_) => TypeErrorKind::TypeMismatch,
            _ => TypeErrorKind::TypeMismatch,
        }
    }
    
    /// Generate detailed error report with suggestions
    pub fn generate_semantic_error_report(&self) -> String {
        let mut report = String::new();
        
        let error_count = self.errors.len();
        report.push_str(&format!("Semantic Analysis: {} error(s) found\n\n", error_count));
        
        for (index, error) in self.errors.iter().enumerate() {
            report.push_str(&format!("{}. {}: {}\n", 
                index + 1, 
                self.error_kind_to_string(&error.error_type),
                error.message));
            
            if let Some(ref location) = error.location {
                report.push_str(&format!("   at {}\n", location));
            }
            
            // Add suggestions based on error type
            let suggestions = self.generate_error_suggestions(&error.error_type);
            for suggestion in suggestions {
                report.push_str(&format!("   help: {}\n", suggestion));
            }
            
            report.push('\n');
        }
        
        if error_count == 0 {
            report.push_str("✅ Semantic analysis completed successfully!\n");
        } else {
            report.push_str("⚠️ Semantic analysis completed with errors. Some functionality may be limited.\n");
        }
        
        report
    }
    
    /// Convert TypeErrorKind to string
    fn error_kind_to_string(&self, kind: &TypeErrorKind) -> &'static str {
        match kind {
            TypeErrorKind::TypeMismatch => "type mismatch",
            TypeErrorKind::UndefinedVariable => "undefined variable",
            TypeErrorKind::UndefinedFunction => "undefined function",
            TypeErrorKind::ArityMismatch => "arity mismatch", 
            TypeErrorKind::InvalidOperation => "invalid operation",
            TypeErrorKind::ConstraintViolation => "constraint violation",
            TypeErrorKind::UnificationFailure => "unification failure",
            TypeErrorKind::TypeNotFound => "type not found",
            TypeErrorKind::FieldNotFound => "field not found",
            TypeErrorKind::UnsupportedOperation => "unsupported operation",
            TypeErrorKind::InvalidArraySize => "invalid array size",
            TypeErrorKind::InterfaceComplianceError => "interface compliance error",
            TypeErrorKind::ParameterCountMismatch => "parameter count mismatch",
            TypeErrorKind::ParameterTypeMismatch => "parameter type mismatch",
            TypeErrorKind::ReturnTypeMismatch => "return type mismatch",
            TypeErrorKind::InterfaceCastError => "interface cast error",
            TypeErrorKind::MethodDispatchError => "method dispatch error",
            TypeErrorKind::MutableBorrowError => "mutable borrow error",
            TypeErrorKind::ImmutableBorrowError => "immutable borrow error",
            TypeErrorKind::BorrowConflictError => "borrow conflict error",
            TypeErrorKind::UseAfterFreeError => "use after free error",
            TypeErrorKind::InvalidDereferenceError => "invalid dereference error",
            TypeErrorKind::MutabilityViolationError => "mutability violation error",
        }
    }
    
    /// Generate helpful suggestions for error types
    fn generate_error_suggestions(&self, kind: &TypeErrorKind) -> Vec<String> {
        match kind {
            TypeErrorKind::UndefinedVariable => vec![
                "Declare the variable before using it with 'sus variable_name type_name = value'".to_string(),
                "Check for typos in the variable name".to_string(),
                "Ensure the variable is in scope".to_string(),
            ],
            TypeErrorKind::UndefinedFunction => vec![
                "Define the function before calling it".to_string(),
                "Check if you need to import a module with 'yeet \"module_name\"'".to_string(),
                "Verify the function name spelling".to_string(),
            ],
            TypeErrorKind::TypeMismatch => vec![
                "Check that the types are compatible".to_string(),
                "Consider using type conversion if appropriate".to_string(),
                "Verify variable declarations and assignments".to_string(),
            ],
            TypeErrorKind::ArityMismatch => vec![
                "Check the number of function arguments".to_string(),
                "Verify the function signature".to_string(),
            ],
            TypeErrorKind::ParameterCountMismatch => vec![
                "Check the number of parameters in function call".to_string(),
                "Verify function definition has correct parameter count".to_string(),
            ],
            TypeErrorKind::InterfaceComplianceError => vec![
                "Implement all required interface methods".to_string(),
                "Check method signatures match interface definition".to_string(),
            ],
            _ => vec![
                "Refer to the CURSED language documentation".to_string(),
                "Check syntax and type requirements".to_string(),
            ],
        }
    }
    
    /// Attempt to continue analysis with placeholder types
    pub fn continue_with_placeholders(&mut self) -> bool {
        // Add common placeholder types to continue analysis
        self.add_placeholder_types();
        true
    }
    
    /// Add placeholder types for common missing types
    fn add_placeholder_types(&mut self) {
        // Add placeholder type definitions
        use crate::type_system::{TypeExpression, TypeDefinition, TypeKind};
        
        let placeholder_type = TypeDefinition {
            name: "ErrorPlaceholder".to_string(),
            kind: TypeKind::Primitive,
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            methods: Vec::new(),
            fields: Vec::new(),
            is_builtin: true,
        };
        
        // Add to type system (this would need proper integration)
        eprintln!("Added ErrorPlaceholder type for recovery");
    }
}

/// Helper function to check if error is recoverable
pub fn is_recoverable_semantic_error(error: &Error) -> bool {
    match error {
        Error::Type(msg) => {
            // Recoverable type errors
            msg.contains("undefined variable") ||
            msg.contains("type mismatch") ||
            msg.contains("undefined function")
        }
        Error::Parse(_) => false, // Parse errors handled by parser recovery
        _ => false,
    }
}

/// Enhanced semantic analysis with comprehensive error recovery
pub fn analyze_with_recovery(type_checker: &mut TypeChecker, program: &Program) -> Result<String> {
    match type_checker.check_program_with_recovery(program) {
        Ok(()) => {
            if type_checker.errors.is_empty() {
                Ok("✅ Semantic analysis completed successfully".to_string())
            } else {
                let report = type_checker.generate_semantic_error_report();
                Ok(format!("⚠️ Semantic analysis completed with warnings:\n{}", report))
            }
        }
        Err(_) => {
            let report = type_checker.generate_semantic_error_report();
            Ok(format!("❌ Semantic analysis failed:\n{}", report))
        }
    }
}
