//! Error propagation expression for the CURSED language
//!
//! This module defines the error propagation expression (`expr?`), which
//! automatically propagates errors following Go-style error handling patterns.
//! When applied to an expression that may return an error, it automatically
//! returns the error if present, or unwraps the successful value.

use crate::ast::traits::{Expression, Node};
use std::any::Any;
use std::fmt;

/// Error propagation expression using the `?` operator
///
/// Syntax: `expression?`
///
/// This operator is used for early return from functions when encountering errors.
/// If the expression evaluates to an error, the error is immediately returned from
/// the containing function. If the expression evaluates to a success value,
/// that value is unwrapped and used.
///
/// Example:
/// ```cursed
/// slay parseAndAdd(s tea) (normie, error) {
///     n := parseInt(s)?  // If parseInt returns error, function returns that error
///     return n + 1, nil  // Otherwise, use the parsed value
/// }
/// ```
#[derive(Clone)]
pub struct ErrorPropagation {
    /// The token literal for error reporting
    pub token: String,
    /// The expression that may return an error
    pub expression: Box<dyn Expression>,
}

impl ErrorPropagation {
    /// Create a new error propagation expression
    pub fn new(token: String, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
    
    /// Get the inner expression
    pub fn get_expression(&self) -> &dyn Expression {
        self.expression.as_ref()
    }
}

impl fmt::Debug for ErrorPropagation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorPropagation")
            .field("token", &self.token)
            .field("expression", &format!("{}", self.expression.string()))
            .finish()
    }
}

impl Node for ErrorPropagation {
    fn token_literal(&self) -> String {
        self.token.clone()
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

    fn string(&self) -> String {
        format!("{}?", self.expression.string())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for ErrorPropagation {
    fn expression_node(&self) {}

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for ErrorPropagation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}?", self.expression.string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::Identifier;

    #[test]
    fn test_error_propagation_creation() {
        let identifier = Box::new(Identifier {
            token: "x".to_string(),
            value: "x".to_string(),
        });
        
        let error_prop = ErrorPropagation::new("?".to_string(), identifier);
        
        assert_eq!(error_prop.token_literal(), "?");
        assert_eq!(error_prop.string(), "x?");
    }

    #[test]
    fn test_error_propagation_display() {
        let identifier = Box::new(Identifier {
            token: "result".to_string(),
            value: "result".to_string(),
        });
        
        let error_prop = ErrorPropagation::new("?".to_string(), identifier);
        
        assert_eq!(format!("{}", error_prop), "result?");
    }

    #[test]
    fn test_error_propagation_nested() {
        let inner_identifier = Box::new(Identifier {
            token: "func_call".to_string(),
            value: "func_call".to_string(),
        });
        
        let inner_error_prop = Box::new(ErrorPropagation::new("?".to_string(), inner_identifier));
        let outer_error_prop = ErrorPropagation::new("?".to_string(), inner_error_prop);
        
        assert_eq!(outer_error_prop.string(), "func_call??");
    }
}
