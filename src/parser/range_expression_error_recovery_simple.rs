//! Simplified range expression parsing with error recovery
//!
//! This module provides improved error recovery mechanisms for parsing range expressions,
//! allowing the parser to continue even when it encounters invalid range parameters,
//! type errors, or other issues with range syntax.

use crate::ast::Expression;
use crate::ast::expressions::range_expression::RangeExpression;
use crate::ast::expressions::IntegerLiteral;
use crate::error::Error;

use super::parser::Parser;

/// Extension trait for Parser to handle range expression error recovery
pub trait RangeExpressionErrorRecoverySimple<'a> {
    /// Attempts to parse a range expression with robust error recovery
    ///
    /// This method will try to recover from various errors:
    /// - Missing range components (start, end, step)
    /// - Type errors in range parameters
    /// - Invalid range bounds (negative length, etc.)
    /// - Syntax errors in range expression
    ///
    /// # Returns
    ///
    /// A BoxedExpression containing either the valid range expression or a fallback
    fn parse_range_expression_with_recovery_simple(&mut self) -> Result<Box<dyn Expression>, Error>;
    
    /// Creates a fallback range expression when parsing fails
    ///
    /// # Returns
    ///
    /// A default range expression (typically range 0..0)
    fn create_fallback_range_expression_simple(&self) -> Box<dyn Expression>;
}

impl<'a> RangeExpressionErrorRecoverySimple<'a> for Parser<'a> {
    fn parse_range_expression_with_recovery_simple(&mut self) -> Result<Box<dyn Expression>, Error> {
        // Try to parse normally first
        match self.parse_range_expression() {
            Ok(expr) => Ok(expr),
            Err(err) => {
                // Log the error and recover
                tracing::warn!(error = ?err, "Range expression parsing failed, using fallback");
                Ok(self.create_fallback_range_expression_simple())
            }
        }
    }
    
    fn create_fallback_range_expression_simple(&self) -> Box<dyn Expression> {
        // Create a simple range from 0 to 0 (which won't iterate)
        let zero_expr = Box::new(IntegerLiteral {
            token: "0".to_string(),
            value: 0,
        });
        
        Box::new(RangeExpression::Range { end: zero_expr })
    }
}