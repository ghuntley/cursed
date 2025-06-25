use crate::ast::expressions::{ErrorPropagation, QuestionMarkExpression};
use crate::ast::traits::Expression;
use crate::lexer::{Token, TokenType};
use crate::parser::{Parser, Precedence};
use crate::error::CursedError;

use std::fmt;

/// Parser support for question mark operator (`?`) error propagation
/// 
/// The question mark operator in CURSED provides automatic error propagation
/// similar to Rust's `?` operator. It evaluates an expression and:
/// - If the result contains an error, returns early with that error
/// - If the result is successful, unwraps and continues with the inner value
/// 
/// Syntax: `expression?`

impl Parser {
    /// Parse a question mark expression (expr?)
    /// 
    /// This is typically called as a postfix operator during expression parsing.
    /// The precedence should be high (similar to function calls and array indexing).
    pub fn parse_question_mark_expression(&mut self, left: Box<dyn Expression>) -> crate::error::Result<()> {
        // We should be sitting on the '?' token
        if !self.current_token_is(&TokenType::Question) {
            return Err(CursedError::parse_error_with_location(
                format!("Expected '?' token, found {:?}", self.current_token.token_type),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        let question_token = self.current_token.clone();
        
        // Move past the '?' token
        self.lexer.next_token();
        
        // Create the question mark expression
        let question_expr = QuestionMarkExpression::new(
            left,
            question_token.location.line,
            question_token.location.column,
        );
        
        Ok(Box::new(question_expr))
    }

    /// Parse error propagation expression (for backward compatibility)
    /// 
    /// This maintains compatibility with the existing ErrorPropagation type
    /// while we transition to the new QuestionMarkExpression.
    pub fn parse_error_propagation(&mut self, left: Box<dyn Expression>) -> crate::error::Result<()> {
        if !self.current_token_is(&TokenType::Question) {
            return Err(CursedError::parse_error_with_location(
                format!("Expected '?' token, found {:?}", self.current_token.token_type),
                self.current_token.location.line,
                self.current_token.location.column,
            ));
        }

        let question_token = self.current_token.clone();
        
        // Move past the '?' token using parser's advance method
        self.advance_token()?;
        
        // Create the error propagation expression with location info
        let error_prop = ErrorPropagation::new(left);
        
        Ok(Box::new(error_prop))
    }

    /// Check if the current token is a question mark
    pub fn is_question_mark(&self) -> bool {
        self.current_token_is(&TokenType::Question)
    }
    
    /// Get the precedence for the question mark operator
    /// 
    /// Question mark should have high precedence, similar to function calls
    /// and array indexing, since it's a postfix operator.
    pub fn question_mark_precedence() -> Precedence {
        Precedence::Call // High precedence for postfix operators
    }
}

/// Enhanced parse error variants for question mark operator
#[derive(Debug, Clone, PartialEq)]
pub enum QuestionMarkParseError {
    /// Missing expression before question mark
    MissingExpression {
        line: usize,
        column: usize,
    },
    
    /// Invalid expression type for question mark
    InvalidExpressionType {
        expression_type: String,
        line: usize,
        column: usize,
    },
    
    /// Question mark in invalid context
    InvalidContext {
        context: String,
        line: usize,
        column: usize,
    },
}

// impl fmt::Display for QuestionMarkParseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             QuestionMarkParseError::MissingExpression { line, column } => {
//                 write!(f, "Missing expression before '?' operator at line {}, column {}", line, column)
//             },
//             QuestionMarkParseError::InvalidExpressionType { expression_type, line, column } => {
//                 write!(f, "Cannot apply '?' operator to expression of type '{}' at line {}, column {}", 
//                        expression_type, line, column)
//             },
//             QuestionMarkParseError::InvalidContext { context, line, column } => {
//                 write!(f, "Question mark operator '?' not allowed in {} context at line {}, column {}", 
//                        context, line, column)
//             },
//         }
//     }
// }

// impl std::error::CursedError for QuestionMarkParseError {}
// 
/// Helper functions for parsing question mark expressions
impl Parser {
    /// Validate that the expression is suitable for question mark operator
    pub fn validate_question_mark_expression(&self, expr: &dyn Expression) -> crate::error::Result<()> {
        // In CURSED, the question mark operator can be applied to any expression
        // that returns a Result-like type. For now, we'll allow it on all expressions
        // and let the type checker handle the validation.
        Ok(())
    }
    
    /// Check if question mark is allowed in the current parsing context
    pub fn is_question_mark_allowed(&self) -> bool {
        // Question mark is not allowed in certain contexts like:
        // - Function parameter types
        // - Struct field types
        // - Return type declarations
        // For now, we'll allow it everywhere and refine later
        true
    }
    
    /// Parse chained question marks (expr??.foo?)
    pub fn parse_chained_question_marks(&mut self, mut expr: Box<dyn Expression>) -> crate::error::Result<()> {
        while self.is_question_mark() {
            expr = self.parse_question_mark_expression(expr)?;
        }
        Ok(expr)
    }
}

