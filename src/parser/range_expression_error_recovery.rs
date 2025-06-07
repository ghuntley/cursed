//! Enhanced range expression parsing with error recovery
//!
//! This module provides improved error recovery mechanisms for parsing range expressions,
//! allowing the parser to continue even when it encounters invalid range parameters,
//! type errors, or other issues with range syntax.

use crate::ast::Expression;
use crate::ast::expressions::range_expression::RangeExpression;
use crate::ast::expressions::{IntegerLiteral, StringLiteral};
use crate::ast::Node;
use crate::error::Error;
use crate::lexer::{Token, TokenType};
use super::precedence::Precedence;
use std::any::Any;
use tracing::{debug, error, info, instrument, warn};

use super::parser::Parser;

/// Extension trait for Parser to handle range expression error recovery
pub trait RangeExpressionErrorRecovery<'a> {
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
    fn parse_range_expression_with_recovery(&mut self) -> Result<Box<dyn Expression>, Error>;
    
    /// Creates a fallback range expression when parsing fails
    ///
    /// # Returns
    ///
    /// A default range expression (typically range 0..0)
    fn create_fallback_range_expression(&self) -> Box<dyn Expression>;
    
    /// Recovers from errors in the range end parameter
    ///
    /// # Parameters
    ///
    /// * `error` - The original parsing error
    ///
    /// # Returns
    ///
    /// A valid expression to use as the range end (defaults to literal 0)
    fn recover_range_end_parameter(&self, error: Error) -> Box<dyn Expression>;
    
    /// Recovers from errors in the range start parameter
    ///
    /// # Parameters
    ///
    /// * `error` - The original parsing error
    ///
    /// # Returns
    ///
    /// A valid expression to use as the range start (defaults to literal 0)
    fn recover_range_start_parameter(&self, error: Error) -> Box<dyn Expression>;
    
    /// Recovers from errors in the range step parameter
    ///
    /// # Parameters
    ///
    /// * `error` - The original parsing error
    ///
    /// # Returns
    ///
    /// A valid expression to use as the range step (defaults to literal 1)
    fn recover_range_step_parameter(&self, error: Error) -> Box<dyn Expression>;
    
    /// Creates a diagnostic message for a range expression parsing error
    ///
    /// # Parameters
    ///
    /// * `error` - The original parsing error
    /// * `context` - Additional context about where the error occurred
    ///
    /// # Returns
    ///
    /// A descriptive diagnostic message for the error
    fn create_range_error_diagnostic(&self, error: &Error, context: &str) -> String;
}

impl<'a> RangeExpressionErrorRecovery<'a> for Parser<'a> {
    #[instrument(skip(self), level = "debug")]
    fn parse_range_expression_with_recovery(&mut self) -> Result<Box<dyn Expression>, Error> {
        debug!("Parsing range expression with recovery mechanisms");
        
        // Save current token to diagnose potential errors
        let start_token = self.current_token.clone();
        
        // Expect the 'flex' token (which is our 'range' keyword)
        if !self.current_token_is(Token::Flex) {
            let err = Error::Parsing(format!(
                "Expected 'flex' keyword for range expression, got '{}'", 
                self.current_token.token_literal()
            ));
            
            warn!(error = %err, "Error parsing range expression, using fallback");
            
            // Attempt to create a useful diagnostic
            let diagnostic = self.create_range_error_diagnostic(&err, "range expression");
            error!(diagnostic = %diagnostic, "Range expression parsing error");
            
            return Ok(self.create_fallback_range_expression());
        }
        
        // Advance past the 'flex' token
        self.next_token()?;
        
        // Now we need to parse the range parameters based on how many there are
        // We could have:
        // - Just end:  flex 10
        // - Start and end: flex 1, 10
        // - Start, end, and step: flex 1, 10, 2
        
        // First, try to parse the end (which could be the only parameter)
        let end_expr = match self.parse_expression(Precedence::Lowest) {
            Ok(expr) => expr,
            Err(err) => {
                warn!(error = %err, "Error parsing range end parameter, using fallback");
                let diagnostic = self.create_range_error_diagnostic(&err, "range end parameter");
                error!(diagnostic = %diagnostic, "Range end parameter parsing error");
                
                self.recover_range_end_parameter(err)
            }
        };
        
        // If there's no comma after this, it's just a single parameter range
        if !self.peek_token_is(Token::Comma) {
            // Just end parameter, return Range { end }
            return Ok(Box::new(RangeExpression::Range { end: end_expr }));
        }
        
        // We have a comma, so we're parsing start and end (and maybe step)
        // The first expression we parsed is actually the start
        let start_expr = end_expr;
        
        // Advance past the comma
        self.next_token()?; // Move to the comma
        self.next_token()?; // Move past the comma
        
        // Now parse the end expression
        let end_expr = match self.parse_expression(Precedence::Lowest) {
            Ok(expr) => expr,
            Err(err) => {
                warn!(error = %err, "Error parsing range end parameter (after start), using fallback");
                let diagnostic = self.create_range_error_diagnostic(&err, "range end parameter");
                error!(diagnostic = %diagnostic, "Range end parameter parsing error");
                
                self.recover_range_end_parameter(err)
            }
        };
        
        // If there's no comma after this, it's a two-parameter range (start, end)
        if !self.peek_token_is(Token::Comma) {
            // Start and end parameters, return RangeFromTo { start, end }
            return Ok(Box::new(RangeExpression::RangeFromTo { 
                start: start_expr, 
                end: end_expr 
            }));
        }
        
        // We have another comma, so we're parsing all three parameters
        // Advance past the comma
        self.next_token()?; // Move to the comma
        self.next_token()?; // Move past the comma
        
        // Now parse the step expression
        let step_expr = match self.parse_expression(Precedence::Lowest) {
            Ok(expr) => expr,
            Err(err) => {
                warn!(error = %err, "Error parsing range step parameter, using fallback");
                let diagnostic = self.create_range_error_diagnostic(&err, "range step parameter");
                error!(diagnostic = %diagnostic, "Range step parameter parsing error");
                
                self.recover_range_step_parameter(err)
            }
        };
        
        // Return the full range expression with all three parameters
        Ok(Box::new(RangeExpression::RangeFromToStep { 
            start: start_expr, 
            end: end_expr, 
            step: step_expr 
        }))
    }
    
    fn create_fallback_range_expression(&self) -> Box<dyn Expression> {
        // Create a simple range from 0 to 0 (which won't iterate)
        let zero_expr = Box::new(IntegerLiteral {
            token: "0".to_string(),
            value: 0,
        });
        
        Box::new(RangeExpression::Range { end: zero_expr })
    }
    
    fn recover_range_end_parameter(&self, error: Error) -> Box<dyn Expression> {
        // Default to a safe value for end parameter
        debug!(error = %error, "Recovering from range end parameter error");
        
        // Default to 0 as a safe end
        Box::new(IntegerLiteral {
            token: "0".to_string(),
            value: 0,
        })
    }
    
    fn recover_range_start_parameter(&self, error: Error) -> Box<dyn Expression> {
        // Default to a safe value for start parameter
        debug!(error = %error, "Recovering from range start parameter error");
        
        // Default to 0 as a safe start
        Box::new(IntegerLiteral {
            token: "0".to_string(),
            value: 0,
        })
    }
    
    fn recover_range_step_parameter(&self, error: Error) -> Box<dyn Expression> {
        // Default to a safe value for step parameter
        debug!(error = %error, "Recovering from range step parameter error");
        
        // Default to 1 as a safe step
        Box::new(IntegerLiteral {
            token: "1".to_string(),
            value: 1,
        })
    }
    
    fn create_range_error_diagnostic(&self, error: &Error, context: &str) -> String {
        match error {
            Error::Parsing(msg) => {
                format!("Range parsing error in {}: {}", context, msg)
            },
            Error::Runtime(msg) => {
                format!("Range runtime error in {}: {}", context, msg)
            },
            _ => {
                format!("Range error in {}: {:?}", context, error)
            }
        }
    }
}

/// Extension trait adding improved range expression parsing to the Parser
pub trait EnhancedRangeExpressionParsing<'a>: RangeExpressionErrorRecovery<'a> {
    /// Parse a range expression with full error recovery and detailed diagnostics
    ///
    /// This method integrates with the main parser to provide a robust implementation
    /// for parsing range expressions, with fallbacks and recovery for all error cases.
    fn parse_range_expression_enhanced(&mut self) -> Result<Box<dyn Expression>, Error>;
    
    /// Validates a range expression for potential runtime issues
    ///
    /// This is an additional check that can detect potential issues like:
    /// - Zero step size
    /// - Negative range lengths
    /// - Invalid types for range parameters
    ///
    /// # Parameters
    ///
    /// * `range_expr` - The range expression to validate
    ///
    /// # Returns
    ///
    /// The original expression, or a corrected version if there were issues
    fn validate_range_expression(&self, range_expr: Box<dyn Expression>) -> Box<dyn Expression>;
}

impl<'a> EnhancedRangeExpressionParsing<'a> for Parser<'a> {
    #[instrument(skip(self), level = "debug")]
    fn parse_range_expression_enhanced(&mut self) -> Result<Box<dyn Expression>, Error> {
        debug!("Parsing enhanced range expression");
        
        // Use the recovery-enabled parsing
        let range_expr = self.parse_range_expression_with_recovery()?;
        
        // Further validate for potential runtime issues
        let validated_expr = self.validate_range_expression(range_expr);
        
        Ok(validated_expr)
    }
    
    fn validate_range_expression(&self, range_expr: Box<dyn Expression>) -> Box<dyn Expression> {
        // Here we would check for potential runtime issues in the range
        // For now, we'll just return the original expression
        // In a more complete implementation, you would:
        // 1. Check for zero step values
        // 2. Check for negative range lengths
        // 3. Ensure compatible types for range parameters
        
        // This is placeholder for now
        range_expr
    }
}

// Utility types to improve error recovery

/// Represents a range expression that can be recovered from errors
///
/// This struct wraps the standard RangeExpression with additional information
/// about any errors encountered during parsing, and fallback values used.
pub struct RecoverableRangeExpression {
    /// The underlying range expression
    pub range: RangeExpression,
    /// Whether this expression was created through error recovery
    pub is_recovered: bool,
    /// Diagnostic messages about any errors encountered
    pub diagnostics: Vec<String>,
}

impl Node for RecoverableRangeExpression {
    fn token_literal(&self) -> String {
        self.range.token_literal()
    }

    fn string(&self) -> String {
        self.range.string()
    }
}

impl Expression for RecoverableRangeExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn node_type(&self) -> &str {
        "RecoverableRangeExpression"
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(RecoverableRangeExpression {
            range: match &self.range {
                RangeExpression::Range { end } => RangeExpression::Range {
                    end: end.clone_box(),
                },
                RangeExpression::RangeFromTo { start, end } => RangeExpression::RangeFromTo {
                    start: start.clone_box(),
                    end: end.clone_box(),
                },
                RangeExpression::RangeFromToStep { start, end, step } => RangeExpression::RangeFromToStep {
                    start: start.clone_box(),
                    end: end.clone_box(),
                    step: step.clone_box(),
                },
            },
            is_recovered: self.is_recovered,
            diagnostics: self.diagnostics.clone(),
        })
    }
}