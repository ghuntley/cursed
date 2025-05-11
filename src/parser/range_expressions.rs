//! Additional parsing functions for range expressions
//!
//! This module defines parsing functions to treat range clauses as expressions,
//! which allows them to be used in the prefix parse function lookup in the Pratt parser.

use crate::ast::Expression;
use crate::error::Error;
use crate::ast::expressions::StringLiteral;

use super::parser::Parser;
use super::range_expression_error_recovery_simple::RangeExpressionErrorRecoverySimple;

impl<'a> Parser<'a> {
    /// Parse a range clause as an expression
    ///
    /// This is a wrapper around the parse_range_clause method that returns a Box<dyn Expression>
    /// for use in the prefix parse function lookup.
    ///
    /// # Returns
    ///
    /// A boxed Range expression
    pub(super) fn parse_range_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        // Use our simplified recovery version for better error handling
        self.parse_range_expression_with_recovery_simple()
    }
    
    /// Parse an ellipsis expression (for the '...' token)
    ///
    /// This provides a handler for the '...' token in expressions
    ///
    /// # Returns
    ///
    /// A placeholder expression for the ellipsis
    pub(super) fn parse_ellipsis_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past '...'
        
        // For simplicity, just return a string literal with "..." value
        // In a complete implementation, you would create a proper AST node
        Ok(Box::new(StringLiteral {
            token: token.token_literal(),
            value: "...".to_string(),
        }))
    }
}