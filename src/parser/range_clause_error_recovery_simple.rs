//! Simplified range clause error recovery
//!
//! This module provides a simplified error recovery implementation for range clauses
//! in the CURSED programming language's parser.

use crate::ast::control_flow::range::RangeClause;
use crate::ast::expressions::IntegerLiteral;
use crate::error::Error;

use super::parser::Parser;

/// Extension trait for Parser to handle range clause error recovery
pub trait RangeClauseErrorRecoverySimple<'a> {
    /// Parse a range clause with robust error recovery
    ///
    /// This method enhances standard range clause parsing with the ability to recover
    /// from various error conditions.
    ///
    /// # Returns
    ///
    /// A RangeClause AST node, potentially with fallback values for invalid expressions
    fn parse_range_clause_with_recovery_simple(&mut self) -> Result<RangeClause, Error>;
    
    /// Create a fallback RangeClause when parsing fails
    ///
    /// # Returns
    ///
    /// A default RangeClause that will produce an empty range
    fn create_fallback_range_clause_simple(&self) -> RangeClause;
}

impl<'a> RangeClauseErrorRecoverySimple<'a> for Parser<'a> {
    fn parse_range_clause_with_recovery_simple(&mut self) -> Result<RangeClause, Error> {
        // Try to parse normally first
        match self.parse_range_clause() {
            Ok(range_clause) => Ok(range_clause),
            Err(err) => {
                // Log the error and use a fallback
                tracing::warn!(error = ?err, "Range clause parsing failed, using fallback");
                Ok(self.create_fallback_range_clause_simple())
            }
        }
    }
    
    fn create_fallback_range_clause_simple(&self) -> RangeClause {
        // Create a default range clause (0 to 0, which is an empty range)
        let zero_expr = Box::new(IntegerLiteral {
            token: "0".to_string(),
            value: 0,
        });
        
        RangeClause {
            token: "flex".to_string(),
            start: Some(zero_expr.clone()),
            end: zero_expr,
            step: None,
            is_container: false,
        }
    }
}