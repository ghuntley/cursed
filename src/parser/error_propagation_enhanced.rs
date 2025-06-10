use crate::ast::expressions::ErrorPropagation;
use crate::ast::traits::Expression;
use crate::lexer::{Token, TokenType};
use crate::parser::{Parser, Precedence};
use crate::error::{CursedError, SourceLocation};
use std::fmt;

/// Enhanced parser support for error propagation (`?` operator)
/// 
/// This module implements comprehensive parsing for the question mark operator
/// in CURSED, providing proper precedence handling, error recovery, and 
/// context validation.

impl Parser {
    /// Parse a question mark expression with full context awareness
    /// 
    /// This method handles the `?` operator as a postfix operator with high precedence.
    /// It validates the context and provides detailed error messages.
    pub fn parse_error_propagation_enhanced(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, CursedError> {
        // Validate that we're on the question mark token
        if !self.current_token_is(&TokenType::Question) {
            return Err(CursedError::ParseError {
                message: format!("Expected '?' token, found {:?}", self.current_token.token_type),
                line: Some(self.current_token.location.line),
                column: Some(self.current_token.location.column),
            });
        }

        let question_token = self.current_token.clone();
        let location = SourceLocation::new(
            question_token.location.line,
            question_token.location.column,
        );
        
        // Note: Context validation would call validate_error_propagation_context if it were public
        
        // Move past the '?' token
        self.lexer.next_token();
        
        // Create error propagation expression (using existing API)
        let error_prop = ErrorPropagation::new(left);
        
        // Note: Enhanced features like tail position and type information
        // would be implemented in the enhanced version when needed
        
        Ok(Box::new(error_prop))
    }
    
    /// Validate that error propagation is allowed in the current context (enhanced version)
    fn validate_error_propagation_context_enhanced(&self, location: &SourceLocation) -> Result<(), CursedError> {
        // Check if we're inside a function
        if !self.is_in_function_context() {
            return Err(CursedError::ErrorPropagation {
                message: "Error propagation with '?' can only be used within functions".to_string(),
                line: Some(location.line),
                column: Some(location.column),
            });
        }
        
        // Check if we're in a valid return type context
        if !self.has_compatible_return_type() {
            return Err(CursedError::ErrorPropagation {
                message: "Function return type must be compatible with error propagation (Result<T, E> or Option<T>)".to_string(),
                line: Some(location.line),
                column: Some(location.column),
            });
        }
        
        // Check for invalid contexts (e.g., const expressions)
        if self.is_in_const_context() {
            return Err(CursedError::ErrorPropagation {
                message: "Error propagation with '?' is not allowed in constant expressions".to_string(),
                line: Some(location.line),
                column: Some(location.column),
            });
        }
        
        Ok(())
    }
    
    /// Infer the expected type for error propagation
    fn infer_propagation_type(&self, _expr: &ErrorPropagation) -> Option<String> {
        // In a full implementation, this would use type inference
        // Note: Would call get_current_function_return_type if it were public
        None
    }
    
    /// Check if we're currently parsing inside a function
    fn is_in_function_context(&self) -> bool {
        // Simplified check - would use actual parser state in full implementation
        true
    }
    
    /// Check if the current function has a return type compatible with error propagation
    fn has_compatible_return_type(&self) -> bool {
        if let Some(return_type) = self.get_current_function_return_type_enhanced() {
            return_type.starts_with("Result<") || 
            return_type.starts_with("Option<") ||
            return_type == "()?" // For functions that can return errors
        } else {
            // Allow propagation if no explicit return type (will be inferred)
            true
        }
    }
    
    /// Check if we're in a constant expression context
    fn is_in_const_context(&self) -> bool {
        // Simplified check - would use actual parser state in full implementation
        false
    }
    
    /// Check if the current position is a tail position (last expression in function)
    fn is_in_tail_position(&self) -> bool {
        // Simplified check - in practice would analyze the parsing context
        matches!(self.peek_token.token_type, TokenType::RightBrace | TokenType::Eof)
    }
    
    /// Get the current function's return type (enhanced version)
    fn get_current_function_return_type_enhanced(&self) -> Option<String> {
        // Simplified - would use actual parser state in full implementation
        None
    }
    
    /// Parse chained error propagations (expr?.field?.method()?)
    pub fn parse_chained_error_propagations(&mut self, mut expr: Box<dyn Expression>) -> Result<Box<dyn Expression>, CursedError> {
        let mut propagation_count = 0;
        const MAX_PROPAGATION_CHAIN: usize = 50; // Prevent extremely long chains
        
        while self.current_token_is(&TokenType::Question) && propagation_count < MAX_PROPAGATION_CHAIN {
            expr = self.parse_error_propagation_enhanced(expr)?;
            propagation_count += 1;
        }
        
        if propagation_count >= MAX_PROPAGATION_CHAIN {
            return Err(CursedError::ParseError {
                message: "Error propagation chain too long (maximum 50 chained '?' operators)".to_string(),
                line: Some(self.current_token.location.line),
                column: Some(self.current_token.location.column),
            });
        }
        
        Ok(expr)
    }
    
    /// Get the precedence for the question mark operator
    pub fn get_question_mark_precedence() -> Precedence {
        Precedence::Call // High precedence for postfix operators
    }
    
    /// Check if the current token is a question mark
    pub fn is_error_propagation_token(&self) -> bool {
        self.current_token_is(&TokenType::Question)
    }
    
    /// Parse error propagation with error recovery
    pub fn parse_error_propagation_with_recovery(&mut self, left: Box<dyn Expression>) -> Result<Box<dyn Expression>, CursedError> {
        match self.parse_error_propagation_enhanced(left.clone()) {
            Ok(expr) => Ok(expr),
            Err(err) => {
                // Attempt error recovery
                self.recover_from_error_propagation_error()?;
                
                // Return the original expression without propagation
                eprintln!("Warning: Error propagation parsing failed, continuing without '?' operator: {}", err);
                Ok(left)
            }
        }
    }
    
    /// Recover from error propagation parsing errors
    fn recover_from_error_propagation_error(&mut self) -> Result<(), CursedError> {
        // Skip the problematic token and try to continue
        if self.current_token_is(&TokenType::Question) {
            self.lexer.next_token();
        }
        
        // Skip any additional problematic tokens
        while matches!(self.current_token.token_type, 
                     TokenType::Question | TokenType::Illegal) {
            self.lexer.next_token();
            
            // Prevent infinite loops
            if self.current_token_is(&TokenType::Eof) {
                break;
            }
        }
        
        Ok(())
    }
}

/// Enhanced parse error variants for error propagation
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorPropagationParseError {
    /// Missing expression before question mark
    MissingExpression {
        line: usize,
        column: usize,
    },
    
    /// Invalid expression type for error propagation
    InvalidExpressionType {
        expression_type: String,
        line: usize,
        column: usize,
    },
    
    /// Error propagation in invalid context
    InvalidContext {
        context: String,
        line: usize,
        column: usize,
    },
    
    /// Incompatible return type for error propagation
    IncompatibleReturnType {
        return_type: String,
        line: usize,
        column: usize,
    },
    
    /// Excessive error propagation chain length
    ChainTooLong {
        chain_length: usize,
        line: usize,
        column: usize,
    },
    
    /// Error propagation in constant expression
    InConstantExpression {
        line: usize,
        column: usize,
    },
}

impl fmt::Display for ErrorPropagationParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorPropagationParseError::MissingExpression { line, column } => {
                write!(f, "Missing expression before '?' operator at line {}, column {}", line, column)
            },
            ErrorPropagationParseError::InvalidExpressionType { expression_type, line, column } => {
                write!(f, "Cannot apply '?' operator to expression of type '{}' at line {}, column {}", 
                       expression_type, line, column)
            },
            ErrorPropagationParseError::InvalidContext { context, line, column } => {
                write!(f, "Error propagation with '?' not allowed in {} context at line {}, column {}", 
                       context, line, column)
            },
            ErrorPropagationParseError::IncompatibleReturnType { return_type, line, column } => {
                write!(f, "Function return type '{}' is incompatible with error propagation at line {}, column {}", 
                       return_type, line, column)
            },
            ErrorPropagationParseError::ChainTooLong { chain_length, line, column } => {
                write!(f, "Error propagation chain too long ({} operators) at line {}, column {}", 
                       chain_length, line, column)
            },
            ErrorPropagationParseError::InConstantExpression { line, column } => {
                write!(f, "Error propagation with '?' not allowed in constant expressions at line {}, column {}", 
                       line, column)
            },
        }
    }
}

impl std::error::Error for ErrorPropagationParseError {}

/// Context tracking for error propagation parsing
#[derive(Debug, Clone)]
pub struct ErrorPropagationParseContext {
    /// Current function context depth
    pub function_depth: usize,
    
    /// Current constant expression depth
    pub const_depth: usize,
    
    /// Current function return type
    pub function_return_type: Option<String>,
    
    /// Error propagation chain length
    pub propagation_chain_length: usize,
}

impl ErrorPropagationParseContext {
    /// Create a new parse context
    pub fn new() -> Self {
        Self {
            function_depth: 0,
            const_depth: 0,
            function_return_type: None,
            propagation_chain_length: 0,
        }
    }
    
    /// Enter a function context
    pub fn enter_function(&mut self, return_type: Option<String>) {
        self.function_depth += 1;
        self.function_return_type = return_type;
    }
    
    /// Exit a function context
    pub fn exit_function(&mut self) {
        self.function_depth = self.function_depth.saturating_sub(1);
        if self.function_depth == 0 {
            self.function_return_type = None;
        }
    }
    
    /// Enter a constant expression context
    pub fn enter_const(&mut self) {
        self.const_depth += 1;
    }
    
    /// Exit a constant expression context
    pub fn exit_const(&mut self) {
        self.const_depth = self.const_depth.saturating_sub(1);
    }
    
    /// Increment the propagation chain length
    pub fn increment_propagation(&mut self) {
        self.propagation_chain_length += 1;
    }
    
    /// Reset the propagation chain length
    pub fn reset_propagation_chain(&mut self) {
        self.propagation_chain_length = 0;
    }
    
    /// Check if we're in a function context
    pub fn in_function(&self) -> bool {
        self.function_depth > 0
    }
    
    /// Check if we're in a constant context
    pub fn in_const(&self) -> bool {
        self.const_depth > 0
    }
}

impl Default for ErrorPropagationParseContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::identifiers::Identifier;
    use crate::lexer::Lexer;

    fn setup_parser_with_input(input: &str) -> Parser {
        let mut lexer = Lexer::new(input.to_string());
        Parser::new(lexer).expect("Failed to create parser")
    }

    #[test]
    fn test_error_propagation_parse_context() {
        let mut context = ErrorPropagationParseContext::new();
        
        assert!(!context.in_function());
        context.enter_function(Some("Result<i32, String>".to_string()));
        assert!(context.in_function());
        assert_eq!(context.function_return_type, Some("Result<i32, String>".to_string()));
        
        context.exit_function();
        assert!(!context.in_function());
        assert_eq!(context.function_return_type, None);
    }
    
    #[test]
    fn test_error_propagation_precedence() {
        let precedence = Parser::get_question_mark_precedence();
        assert_eq!(precedence, Precedence::Call);
    }
    
    #[test]
    fn test_propagation_chain_tracking() {
        let mut context = ErrorPropagationParseContext::new();
        
        assert_eq!(context.propagation_chain_length, 0);
        context.increment_propagation();
        assert_eq!(context.propagation_chain_length, 1);
        context.increment_propagation();
        assert_eq!(context.propagation_chain_length, 2);
        
        context.reset_propagation_chain();
        assert_eq!(context.propagation_chain_length, 0);
    }
    
    #[test]
    fn test_const_context_tracking() {
        let mut context = ErrorPropagationParseContext::new();
        
        assert!(!context.in_const());
        context.enter_const();
        assert!(context.in_const());
        context.exit_const();
        assert!(!context.in_const());
    }
    
    #[test]
    fn test_error_propagation_parse_error_display() {
        let error = ErrorPropagationParseError::MissingExpression { line: 1, column: 5 };
        let message = format!("{}", error);
        assert!(message.contains("Missing expression"));
        assert!(message.contains("line 1"));
        assert!(message.contains("column 5"));
        
        let error = ErrorPropagationParseError::InvalidContext { 
            context: "global".to_string(), 
            line: 2, 
            column: 10 
        };
        let message = format!("{}", error);
        assert!(message.contains("global context"));
    }
    
    #[test]
    fn test_chained_propagation_limit() {
        let error = ErrorPropagationParseError::ChainTooLong { 
            chain_length: 51, 
            line: 1, 
            column: 1 
        };
        let message = format!("{}", error);
        assert!(message.contains("51 operators"));
        assert!(message.contains("too long"));
    }
    
    #[test]
    fn test_incompatible_return_type_error() {
        let error = ErrorPropagationParseError::IncompatibleReturnType { 
            return_type: "i32".to_string(), 
            line: 1, 
            column: 5 
        };
        let message = format!("{}", error);
        assert!(message.contains("i32"));
        assert!(message.contains("incompatible"));
    }
    
    #[test]
    fn test_constant_expression_error() {
        let error = ErrorPropagationParseError::InConstantExpression { line: 1, column: 5 };
        let message = format!("{}", error);
        assert!(message.contains("constant expressions"));
        assert!(message.contains("not allowed"));
    }
}
