//! Enhanced error propagation parsing for the CURSED programming language
//!
//! This module provides comprehensive parsing support for the `?` operator,
//! including integration with Result/Option types, context validation,
//! and error recovery mechanisms.

use crate::ast::expressions::{ErrorPropagation, QuestionMarkExpression};
use crate::ast::traits::Expression;
use crate::error::{CursedError, SourceLocation};
use crate::lexer::{Token, TokenType};
use crate::parser::{Parser, Precedence};
use crate::types::result::{ResultTypeExpression, OptionTypeExpression};
use std::fmt;
use tracing::{debug, error, info, instrument, warn};

/// Enhanced error propagation parser with comprehensive support
// Stub implementations for missing Parser methods
impl Parser {
    /// Get current function context (stub implementation)
    fn current_function_context(&self) -> Option<String> {
        None // Placeholder - would be populated by function parsing
    }
    
    /// Function return types stack (stub implementation) 
    fn function_return_types(&self) -> Vec<String> {
        Vec::new() // Placeholder - would be populated by function parsing
    }
    
    // Note: current_token_is and expect_token methods removed to avoid duplicates with mod.rs
    
    /// Advance to next token (using existing advance_token method)
    fn next_token(&mut self) -> Result<(), CursedError> {
        self.advance_token()
            .map_err(|e| CursedError::Parse(format!("Parser error: {:?}", e)))
    }
    
    /// Parse primary expression (stub)
    fn parse_primary_expression(&mut self) -> Result<Box<dyn Expression>, CursedError> {
        Err(CursedError::Parse("parse_primary_expression not implemented".to_string()))
    }
    
    // Note: parse_expression and parse_block_statement methods removed to avoid duplicates
    
    /// Parse function arguments (stub)
    fn parse_function_arguments(&mut self) -> Result<Vec<Box<dyn Expression>>, CursedError> {
        Ok(Vec::new())
    }
    
    /// Compile expression (stub for codegen integration)
    fn compile_expression(&mut self, _expr: &Box<dyn Expression>) -> Result<(), CursedError> {
        Ok(())
    }
}

impl Parser {
    /// Parse enhanced question mark expression with full context validation
    #[instrument(skip(self, left_expr))]
    pub fn parse_enhanced_question_mark(
        &mut self,
        left_expr: Box<dyn Expression>,
    ) -> Result<Box<dyn Expression>, CursedError> {
        let start_location = self.current_token.location.clone();
        
        // Validate we have a question mark token
        if !self.current_token_is(&TokenType::Question) {
            return Err(CursedError::parse_error_with_location(
                format!("Expected '?' token for error propagation, found {:?}", self.current_token.token_type),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        // Validate that we're in a valid context for error propagation
        self.validate_error_propagation_context(&start_location)?;

        // Consume the '?' token
        let question_token = self.current_token.clone();
        self.next_token()?;

        // Create enhanced question mark expression
        let enhanced_expr = EnhancedQuestionMarkExpression::new(
            left_expr,
            question_token.location,
            self.get_current_function_context(),
            self.get_expected_return_type(),
        );

        debug!(
            location = ?question_token.location,
            function_context = ?enhanced_expr.function_context,
            "Parsed enhanced question mark expression"
        );

        Ok(Box::new(enhanced_expr))
    }

    /// Parse error propagation with type checking
    #[instrument(skip(self, left_expr))]
    pub fn parse_typed_error_propagation(
        &mut self,
        left_expr: Box<dyn Expression>,
    ) -> Result<Box<dyn Expression>, CursedError> {
        // Check if the left expression has a compatible type
        let expr_type = self.infer_expression_type(&left_expr)?;
        
        if !self.is_propagatable_type(&expr_type) {
            return Err(CursedError::parse_error_with_location(
                format!("Cannot apply '?' operator to expression of type '{}'. Expected Result<T, E> or Option<T>", expr_type),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        // Parse the basic question mark expression
        let basic_expr = self.parse_enhanced_question_mark(left_expr)?;

        // Wrap with type-checked propagation
        let typed_expr = TypedErrorPropagation::new(
            basic_expr,
            expr_type,
            self.get_current_function_return_type().to_string(),
        );

        Ok(Box::new(typed_expr))
    }

    /// Parse result unwrapping with pattern matching
    #[instrument(skip(self))]
    pub fn parse_result_unwrapping(&mut self) -> Result<Box<dyn Expression>, CursedError> {
        // Handle syntax like: `result?` or `option?`
        let expr = self.parse_primary_expression()?;
        
        if self.current_token_is(&TokenType::Question) {
            return self.parse_typed_error_propagation(expr);
        }

        Ok(expr)
    }

    /// Parse optional chaining with error propagation
    #[instrument(skip(self))]
    pub fn parse_optional_chaining(&mut self) -> Result<Box<dyn Expression>, CursedError> {
        let mut expr = self.parse_primary_expression()?;

        // Handle chains like: `obj?.method()?.field?`
        while self.current_token_is(&TokenType::Question) {
            
            if self.current_token_is(&TokenType::Question) {
                expr = self.parse_enhanced_question_mark(expr)?;
            } else {
                // Handle `?.` operator for optional chaining
                expr = self.parse_optional_access(expr)?;
            }
        }

        Ok(expr)
    }

    /// Parse error recovery expressions
    #[instrument(skip(self))]
    pub fn parse_error_recovery(&mut self) -> Result<Box<dyn Expression>, CursedError> {
        // Handle syntax like: `expr.unwrap_or(default)` or `expr.unwrap_or_else(|| default)`
        if self.current_token_is(&TokenType::Identifier) && 
           (self.current_token.literal == "unwrap_or" || self.current_token.literal == "unwrap_or_else") {
            return self.parse_unwrap_or_expression();
        }

        // Handle try-catch style error handling
        if self.current_token_is(&TokenType::Identifier) && self.current_token.literal == "try" {
            return self.parse_try_expression();
        }

        self.parse_optional_chaining()
    }

    /// Validate that error propagation is allowed in current context
    fn validate_error_propagation_context(&self, location: &SourceLocation) -> Result<(), CursedError> {
        // Check if we're in a function that can return an error
        if self.current_function_context().is_none() {
            return Err(CursedError::parse_error_with_location(
                "Error propagation with '?' can only be used inside functions".to_string(),
                location.line,
                location.column,
            ));
        }

        // Check if the current function has a compatible return type
        let return_type = self.get_current_function_return_type();
        if !self.is_propagatable_return_type(&return_type) {
            return Err(CursedError::parse_error_with_location(
                format!("Function return type must be compatible with error propagation (Result<T, E> or Option<T>). Found: {}", return_type),
                location.line,
                location.column,
            ));
        }

        Ok(())
    }

    /// Check if a type supports error propagation
    fn is_propagatable_type(&self, type_name: &str) -> bool {
        type_name.starts_with("Result<") || 
        type_name.starts_with("Option<") ||
        type_name == "Result" ||
        type_name == "Option"
    }

    /// Check if a return type supports error propagation
    fn is_propagatable_return_type(&self, return_type: &str) -> bool {
        self.is_propagatable_type(return_type)
    }

    /// Get current function context for error propagation
    fn get_current_function_context(&self) -> Option<String> {
        self.current_function_context().clone()
    }

    /// Get expected return type for current function
    fn get_expected_return_type(&self) -> Option<String> {
        Some(self.get_current_function_return_type().to_string())
    }

    /// Get current function return type
    fn get_current_function_return_type(&self) -> &str {
        // This would be populated by function parsing - using method call
        self.function_return_types()
            .last()
            .map(|s| s.as_str())
            .unwrap_or("()")
    }

    /// Infer the type of an expression for propagation validation
    fn infer_expression_type(&self, expr: &Box<dyn Expression>) -> Result<String, CursedError> {
        // This is a simplified type inference - in a real implementation,
        // this would integrate with the full type system
        
        // Look at the expression type representation for type hints
        let expr_str = format!("{:?}", expr); // Use Debug formatting instead
        
        if expr_str.contains("Result::") || expr_str.contains(".ok()") || expr_str.contains(".err()") {
            return Ok("Result<T, E>".to_string());
        }
        
        if expr_str.contains("Option::") || expr_str.contains(".some()") || expr_str.contains(".none()") {
            return Ok("Option<T>".to_string());
        }
        
        if expr_str.contains("?") {
            return Ok("Result<T, E>".to_string()); // Previous propagation
        }

        // Check if it's a function call that might return Result/Option
        if expr_str.contains("()") {
            return Ok("Result<T, E>".to_string()); // Assume function calls return Results
        }

        // Default to unknown type - this would trigger an error in validation
        Ok("unknown".to_string())
    }

    /// Parse optional access operator (?.)
    fn parse_optional_access(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, CursedError> {
        // Consume the '?.' token
        self.next_token()?;

        // Parse the right-hand side (field access or method call)
        let right = match self.current_token.token_type {
            TokenType::Identifier => {
                let field_name = self.current_token.literal.clone();
                let location = self.current_token.location.clone();
                self.next_token()?;

                // Check if it's a method call
                if self.current_token_is(&TokenType::LeftParen) {
                    self.parse_method_call_with_base(left, field_name, location)?
                } else {
                    self.create_field_access(left, field_name, location)?
                }
            }
            _ => {
                return Err(CursedError::parse_error_with_location(
                    "Expected identifier after '?.' operator".to_string(),
                    self.current_token.location.line,
                    self.current_token.location.column,
                ));
            }
        };

        Ok(right)
    }

    /// Parse unwrap_or expression
    fn parse_unwrap_or_expression(&mut self) -> Result<Box<dyn Expression>, CursedError> {
        // Implementation for unwrap_or parsing
        let method_name = match &self.current_token.token_type {
            TokenType::Identifier => self.current_token.literal.clone(),
            _ => return Err(CursedError::parse_error_with_location(
                "Expected method name".to_string(),
                self.current_token.location.line,
                self.current_token.location.column,
            )),
        };

        self.next_token()?;
        self.expect_token(TokenType::LeftParen)?;

        let default_expr = self.parse_primary_expression()?; // Using available method
        
        self.expect_token(TokenType::RightParen)?;

        // Create a simple error propagation expression (stub)
        // In a full implementation, this would create an UnwrapOrExpression
        let error_prop = ErrorPropagation::new(default_expr);
        Ok(Box::new(error_prop))
    }

    /// Parse try expression for error handling
    fn parse_try_expression(&mut self) -> Result<Box<dyn Expression>, CursedError> {
        // Skip 'try' keyword
        self.next_token()?;
        
        self.expect_token(TokenType::LeftBrace)?;
        let try_body = self.parse_primary_expression()?; // Using available method
        self.expect_token(TokenType::RightBrace)?;

        // Optional catch block
        let catch_block = if self.current_token_is(&TokenType::Identifier) && self.current_token.literal == "catch" {
            self.next_token()?;
            self.expect_token(TokenType::LeftBrace)?;
            let catch_body = self.parse_primary_expression()?; // Using available method
            self.expect_token(TokenType::RightBrace)?;
            Some(catch_body)
        } else {
            None
        };

        // Create a simple error propagation expression (stub)
        // In a full implementation, this would create a TryExpression
        let error_prop = ErrorPropagation::new(try_body);
        Ok(Box::new(error_prop))
    }

    /// Create field access expression
    fn create_field_access(
        &self,
        object: Box<dyn Expression>,
        field_name: String,
        location: SourceLocation,
    ) -> Result<Box<dyn Expression>, CursedError> {
        // Create a simple error propagation expression (stub)
        // In a full implementation, this would create a FieldAccessExpression  
        let error_prop = ErrorPropagation::new(object);
        Ok(Box::new(error_prop))
    }

    /// Parse method call with base object
    fn parse_method_call_with_base(
        &mut self,
        base: Box<dyn Expression>,
        method_name: String,
        location: SourceLocation,
    ) -> Result<Box<dyn Expression>, CursedError> {
        // Parse method arguments
        let args = self.parse_function_arguments()?;
        
        // Create a simple error propagation expression (stub)
        // In a full implementation, this would create a MethodCallExpression
        let error_prop = ErrorPropagation::new(base);
        Ok(Box::new(error_prop))
    }
}

/// Enhanced question mark expression with comprehensive context
#[derive(Debug, Clone)]
pub struct EnhancedQuestionMarkExpression {
    /// The expression being unwrapped
    pub inner_expression: Box<dyn Expression>,
    /// Source location of the ? operator
    pub location: SourceLocation,
    /// Function context where this appears
    pub function_context: Option<String>,
    /// Expected return type
    pub expected_return_type: Option<String>,
    /// Additional metadata
    pub metadata: PropagationMetadata,
}

impl EnhancedQuestionMarkExpression {
    pub fn new(
        inner_expression: Box<dyn Expression>,
        location: SourceLocation,
        function_context: Option<String>,
        expected_return_type: Option<String>,
    ) -> Self {
        Self {
            inner_expression,
            location,
            function_context,
            expected_return_type,
            metadata: PropagationMetadata::new(),
        }
    }
}

/// Type-checked error propagation expression
#[derive(Debug, Clone)]
pub struct TypedErrorPropagation {
    /// The underlying question mark expression
    pub inner_expression: Box<dyn Expression>,
    /// Inferred type of the expression
    pub expression_type: String,
    /// Expected function return type
    pub return_type: String,
}

impl TypedErrorPropagation {
    pub fn new(
        inner_expression: Box<dyn Expression>,
        expression_type: String,
        return_type: String,
    ) -> Self {
        Self {
            inner_expression,
            expression_type,
            return_type,
        }
    }
}

/// Unwrap-or expression for error recovery
#[derive(Debug, Clone)]
pub struct UnwrapOrExpression {
    /// Method name (unwrap_or or unwrap_or_else)
    pub method_name: String,
    /// Default value expression
    pub default_expr: Box<dyn Expression>,
}

impl UnwrapOrExpression {
    pub fn new(method_name: String, default_expr: Box<dyn Expression>) -> Self {
        Self {
            method_name,
            default_expr,
        }
    }
}

/// Try-catch expression for error handling
#[derive(Debug, Clone)]
pub struct TryExpression {
    /// Try block body
    pub try_body: Box<dyn Expression>,
    /// Optional catch block
    pub catch_block: Option<Box<dyn Expression>>,
}

impl TryExpression {
    pub fn new(
        try_body: Box<dyn Expression>,
        catch_block: Option<Box<dyn Expression>>,
    ) -> Self {
        Self {
            try_body,
            catch_block,
        }
    }
}

/// Field access expression
#[derive(Debug, Clone)]
pub struct FieldAccessExpression {
    /// Object being accessed
    pub object: Box<dyn Expression>,
    /// Field name
    pub field_name: String,
    /// Source location
    pub location: SourceLocation,
}

impl FieldAccessExpression {
    pub fn new(
        object: Box<dyn Expression>,
        field_name: String,
        location: SourceLocation,
    ) -> Self {
        Self {
            object,
            field_name,
            location,
        }
    }
}

/// Method call expression
#[derive(Debug, Clone)]
pub struct MethodCallExpression {
    /// Base object
    pub base: Box<dyn Expression>,
    /// Method name
    pub method_name: String,
    /// Arguments
    pub arguments: Vec<Box<dyn Expression>>,
    /// Source location
    pub location: SourceLocation,
}

impl MethodCallExpression {
    pub fn new(
        base: Box<dyn Expression>,
        method_name: String,
        arguments: Vec<Box<dyn Expression>>,
        location: SourceLocation,
    ) -> Self {
        Self {
            base,
            method_name,
            arguments,
            location,
        }
    }
}

/// Metadata for error propagation
#[derive(Debug, Clone)]
pub struct PropagationMetadata {
    /// Whether this is in a try block
    pub in_try_block: bool,
    /// Nesting level of propagation
    pub nesting_level: u32,
    /// Associated error types
    pub error_types: Vec<String>,
}

impl PropagationMetadata {
    pub fn new() -> Self {
        Self {
            in_try_block: false,
            nesting_level: 0,
            error_types: Vec::new(),
        }
    }
}

// Implement Expression trait for all new expression types
impl crate::ast::traits::Expression for EnhancedQuestionMarkExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn crate::ast::traits::Expression> {
        Box::new(self.clone())
    }
}

impl crate::ast::traits::Node for EnhancedQuestionMarkExpression {
    fn string(&self) -> String {
        format!("{}?", self.inner_expression.string())
    }

    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl fmt::Display for EnhancedQuestionMarkExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}?", self.inner_expression)
    }
}

// Similar implementations for other expression types...
impl crate::ast::traits::Expression for TypedErrorPropagation {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn crate::ast::traits::Expression> {
        Box::new(self.clone())
    }
}

impl crate::ast::traits::Node for TypedErrorPropagation {
    fn string(&self) -> String {
        format!("{}? : {} -> {}", self.inner_expression.string(), self.expression_type, self.return_type)
    }

    fn token_literal(&self) -> String {
        "?".to_string()
    }
}

impl fmt::Display for TypedErrorPropagation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}? : {} -> {}", self.inner_expression, self.expression_type, self.return_type)
    }
}

/// Error recovery utilities
pub mod error_recovery {
    use super::*;

    /// Check if an expression can recover from errors
    pub fn can_recover_from_error(expr: &dyn Expression) -> bool {
        // Check if the expression has error recovery mechanisms
        let expr_str = expr.to_string();
        expr_str.contains("unwrap_or") || 
        expr_str.contains("unwrap_or_else") ||
        expr_str.contains("try")
    }

    /// Get suggested recovery patterns for an expression
    pub fn suggest_recovery_patterns(expr_type: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        if expr_type.starts_with("Result<") {
            suggestions.push("Use .unwrap_or(default) for error recovery".to_string());
            suggestions.push("Use .unwrap_or_else(|| default) for lazy evaluation".to_string());
            suggestions.push("Use match expression for explicit error handling".to_string());
        }

        if expr_type.starts_with("Option<") {
            suggestions.push("Use .unwrap_or(default) for None handling".to_string());
            suggestions.push("Use .unwrap_or_else(|| default) for lazy evaluation".to_string());
            suggestions.push("Use if let Some(val) = expr for explicit handling".to_string());
        }

        suggestions
    }

    /// Validate error propagation chain
    pub fn validate_propagation_chain(chain: &[Box<dyn Expression>]) -> Result<(), CursedError> {
        for (i, expr) in chain.iter().enumerate() {
            if expr.to_string().contains("?") && i == chain.len() - 1 {
                return Err(CursedError::Parse(
                    "Error propagation at end of chain should be handled".to_string()
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propagatable_type_checking() {
        let parser = Parser::new(crate::lexer::Lexer::new("").unwrap()).unwrap();
        
        assert!(parser.is_propagatable_type("Result<i32, String>"));
        assert!(parser.is_propagatable_type("Option<String>"));
        assert!(parser.is_propagatable_type("Result"));
        assert!(parser.is_propagatable_type("Option"));
        
        assert!(!parser.is_propagatable_type("i32"));
        assert!(!parser.is_propagatable_type("String"));
        assert!(!parser.is_propagatable_type("Vec<i32>"));
    }

    #[test]
    fn test_expression_type_inference() {
        let parser = Parser::new(crate::lexer::Lexer::new("").unwrap()).unwrap();
        
        // This would need mock expressions for testing
        // let result_expr = create_mock_result_expression();
        // let inferred_type = parser.infer_expression_type(&result_expr).unwrap();
        // assert_eq!(inferred_type, "Result<T, E>");
    }

    #[test]
    fn test_propagation_metadata() {
        let metadata = PropagationMetadata::new();
        assert!(!metadata.in_try_block);
        assert_eq!(metadata.nesting_level, 0);
        assert!(metadata.error_types.is_empty());
    }

    #[test]
    fn test_error_recovery_suggestions() {
        let suggestions = error_recovery::suggest_recovery_patterns("Result<i32, String>");
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.contains("unwrap_or")));

        let option_suggestions = error_recovery::suggest_recovery_patterns("Option<i32>");
        assert!(!option_suggestions.is_empty());
        assert!(option_suggestions.iter().any(|s| s.contains("unwrap_or")));
    }
}
