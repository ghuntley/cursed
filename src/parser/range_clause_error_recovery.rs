//! Extended implementation for range clauses with error recovery
//!
//! This module enhances the standard range clause parsing with robust
//! error handling and recovery strategies.

use crate::ast::control_flow::range::{RangeClause, RangeForStatement};
use crate::ast::Expression;
use crate::ast::expressions::{IntegerLiteral, Identifier};
use crate::error::Error;
use crate::lexer::Token;
use tracing::{debug, error, info, instrument, warn};

use super::parser::Parser;
use super::range_expression_error_recovery::{RangeExpressionErrorRecovery, EnhancedRangeExpressionParsing};

/// Extension trait for Parser implementing enhanced range clause parsing with error recovery
pub trait RangeClauseErrorRecovery<'a> {
    /// Parse a range clause with robust error recovery
    ///
    /// This method enhances standard range clause parsing with the ability to recover
    /// from various error conditions, ensuring parsing can continue even when
    /// range expressions contain errors.
    ///
    /// # Returns
    ///
    /// A RangeClause AST node, potentially with fallback values for invalid expressions
    fn parse_range_clause_with_recovery(&mut self) -> Result<RangeClause, Error>;
    
    /// Create a fallback RangeClause when parsing fails
    ///
    /// # Returns
    ///
    /// A default RangeClause that will produce an empty range
    fn create_fallback_range_clause(&self) -> RangeClause;
    
    /// Create a diagnostic message for a range clause error
    ///
    /// # Parameters
    ///
    /// * `error` - The original error
    /// * `context` - Additional context information
    ///
    /// # Returns
    ///
    /// A formatted diagnostic message
    fn create_range_clause_diagnostic(&self, error: &Error, context: &str) -> String;
    
    /// Parse a range-based for loop with error recovery
    ///
    /// This method enhances the standard range for loop parsing with
    /// robust error handling for variable declarations, range clauses,
    /// and loop bodies.
    ///
    /// # Returns
    ///
    /// A RangeForStatement AST node
    fn parse_range_for_statement_with_recovery(&mut self) -> Result<RangeForStatement, Error>;
}

impl<'a> RangeClauseErrorRecovery<'a> for Parser<'a> {
    #[instrument(skip(self), level = "debug")]
    fn parse_range_clause_with_recovery(&mut self) -> Result<RangeClause, Error> {
        debug!("Parsing range clause with recovery");
        
        // Save current token to diagnose potential errors
        let token = self.current_token.clone();
        
        // Expect 'flex' token
        if !self.current_token_is(Token::Flex) {
            let err = Error::Parsing(format!(
                "Expected 'flex' token for range clause, got '{}'", 
                self.current_token.token_literal()
            ));
            
            warn!(error = %err, "Error parsing range clause, using fallback");
            
            // Log a diagnostic message
            let diagnostic = self.create_range_clause_diagnostic(&err, "range clause");
            error!(diagnostic = %diagnostic, "Range clause parsing error");
            
            return Ok(self.create_fallback_range_clause());
        }
        
        self.next_token()?; // Advance past 'flex'
        
        // Parse the first expression (could be start or container)
        let first_expr = match self.parse_expression(super::precedence::Precedence::Lowest) {
            Ok(expr) => expr,
            Err(err) => {
                // Log the error
                warn!(error = %err, "Error parsing first range expression, using fallback");
                
                // Create a diagnostic message
                let diagnostic = self.create_range_clause_diagnostic(&err, "range start/container parameter");
                error!(diagnostic = %diagnostic, "Range parameter parsing error");
                
                // Use a default expression (0)
                Box::new(IntegerLiteral {
                    token: "0".to_string(),
                    value: 0,
                })
            }
        };
        
        // Check if there are more expressions (indicating a numeric range)
        if self.peek_token_is(Token::Comma) {
            // This is a numeric range with start, end, and optional step
            // We just saw the start, now parse end
            self.next_token()?; // Advance to comma
            self.next_token()?; // Advance past comma
            
            // Parse the end expression
            let end_expr = match self.parse_expression(super::precedence::Precedence::Lowest) {
                Ok(expr) => expr,
                Err(err) => {
                    // Log the error
                    warn!(error = %err, "Error parsing range end expression, using fallback");
                    
                    // Create a diagnostic message
                    let diagnostic = self.create_range_clause_diagnostic(&err, "range end parameter");
                    error!(diagnostic = %diagnostic, "Range end parameter parsing error");
                    
                    // Use a default expression (10)
                    Box::new(IntegerLiteral {
                        token: "10".to_string(),
                        value: 10,
                    })
                }
            };
            
            // Check if there's a step value
            if self.peek_token_is(Token::Comma) {
                self.next_token()?; // Advance to comma
                self.next_token()?; // Advance past comma
                
                // Parse the step expression
                let step_expr = match self.parse_expression(super::precedence::Precedence::Lowest) {
                    Ok(expr) => expr,
                    Err(err) => {
                        // Log the error
                        warn!(error = %err, "Error parsing range step expression, using fallback");
                        
                        // Create a diagnostic message
                        let diagnostic = self.create_range_clause_diagnostic(&err, "range step parameter");
                        error!(diagnostic = %diagnostic, "Range step parameter parsing error");
                        
                        // Use a default expression (1)
                        Box::new(IntegerLiteral {
                            token: "1".to_string(),
                            value: 1,
                        })
                    }
                };
                
                // Create range clause with start, end, and step
                return Ok(RangeClause {
                    token: token.token_literal(),
                    start: Some(first_expr),
                    end: end_expr,
                    step: Some(step_expr),
                    is_container: false,
                });
            }
            
            // Create range clause with start and end (no step)
            return Ok(RangeClause {
                token: token.token_literal(),
                start: Some(first_expr),
                end: end_expr,
                step: None,
                is_container: false,
            });
        }
        
        // If we get here, it's either a simple numeric range (flex 10)
        // or a container iteration (flex container)
        
        // Check if it looks like a container (identifier or expression)
        let is_container = match first_expr.as_any().downcast_ref::<Identifier>() {
            Some(ident) => {
                // If it's an identifier that's not a number, assume it's a container
                !ident.value.parse::<i64>().is_ok()
            },
            None => {
                // If it's not an identifier at all, it's probably a complex expression
                // for a container (e.g., a function call)
                true
            }
        };
        
        Ok(RangeClause {
            token: token.token_literal(),
            start: None,
            end: first_expr,
            step: None,
            is_container,
        })
    }
    
    fn create_fallback_range_clause(&self) -> RangeClause {
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
    
    fn create_range_clause_diagnostic(&self, error: &Error, context: &str) -> String {
        match error {
            Error::Parsing(msg) => {
                format!("Range clause parsing error in {}: {}", context, msg)
            },
            Error::Runtime(msg) => {
                format!("Range clause runtime error in {}: {}", context, msg)
            },
            _ => {
                format!("Range clause error in {}: {:?}", context, error)
            }
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn parse_range_for_statement_with_recovery(&mut self) -> Result<RangeForStatement, Error> {
        debug!("Parsing range for statement with recovery");
        
        let token = self.current_token.clone();
        
        // Expect 'bestie' token
        if !self.current_token_is(Token::Bestie) {
            let err = Error::Parsing(format!(
                "Expected 'bestie' token for range for loop, got '{}'", 
                self.current_token.token_literal()
            ));
            
            warn!(error = %err, "Error parsing range for statement, using fallback");
            
            // Create a fallback for loop with empty range and body
            return Err(err);
        }
        
        self.next_token()?; // Advance past 'bestie'
        
        // Parse the variable(s) for the loop
        let mut key_var = None;
        let mut value_var = String::new();
        
        // Parse first variable
        if !matches!(self.current_token, Token::Identifier(_)) {
            let err = self.error(&format!(
                "Expected identifier for range variable, got {:?}",
                self.current_token
            ));
            
            warn!(error = %err, "Missing variable identifier in range for loop");
            
            // Use a fallback variable name
            value_var = "_i".to_string();
            
            // Skip to the expected token location
            while !self.current_token_is(Token::DeclAssign) && 
                  !self.current_token_is(Token::Flex) && 
                  !self.current_token_is(Token::LBrace) && 
                  !self.current_token_is(Token::Eof) {
                self.next_token()?;
            }
        } else {
            // Get the variable name
            let first_var = match &self.current_token {
                Token::Identifier(ident) => ident.clone(),
                _ => unreachable!(),
            };
            
            self.next_token()?; // Advance past first variable
            
            // Check if we have a second variable (key, value := ...)
            if self.current_token_is(Token::Comma) {
                self.next_token()?; // Advance past comma
                
                // Parse second variable
                if !matches!(self.current_token, Token::Identifier(_)) {
                    let err = self.error(&format!(
                        "Expected identifier for second range variable, got {:?}",
                        self.current_token
                    ));
                    
                    warn!(error = %err, "Missing second variable identifier in range for loop");
                    
                    // Use fallback variable names
                    key_var = Some(first_var);
                    value_var = "_v".to_string();
                    
                    // Skip to the expected token location
                    while !self.current_token_is(Token::DeclAssign) && 
                          !self.current_token_is(Token::Flex) && 
                          !self.current_token_is(Token::LBrace) && 
                          !self.current_token_is(Token::Eof) {
                        self.next_token()?;
                    }
                } else {
                    // Get the second variable name
                    let second_var = match &self.current_token {
                        Token::Identifier(ident) => ident.clone(),
                        _ => unreachable!(),
                    };
                    
                    // Set key_var and value_var
                    key_var = Some(first_var);
                    value_var = second_var;
                    
                    self.next_token()?; // Advance past second variable
                }
            } else {
                // Only one variable
                value_var = first_var;
            }
        }
        
        // Expect ':='
        if !self.current_token_is(Token::DeclAssign) {
            let err = self.error(&format!(
                "Expected ':=' in range for loop, got {:?}",
                self.current_token
            ));
            
            warn!(error = %err, "Missing declaration assignment in range for loop");
            
            // Skip to the next meaningful token
            while !self.current_token_is(Token::Flex) && 
                  !self.current_token_is(Token::LBrace) && 
                  !self.current_token_is(Token::Eof) {
                self.next_token()?;
            }
        } else {
            self.next_token()?; // Advance past ':='
        }
        
        // Expect 'flex' keyword
        if !self.current_token_is(Token::Flex) {
            let err = self.error(&format!(
                "Expected 'flex' in range for loop, got {:?}",
                self.current_token
            ));
            
            warn!(error = %err, "Missing 'flex' keyword in range for loop");
            
            // Create a fallback range clause
            let range_clause = self.create_fallback_range_clause();
            
            // Skip to the expected '{'  
            while !self.current_token_is(Token::LBrace) && 
                  !self.current_token_is(Token::Eof) {
                self.next_token()?;
            }
            
            // If we found '{', parse the body
            if self.current_token_is(Token::LBrace) {
                let body = self.parse_block_statement()?;
                
                // Create the range for statement with the fallback range
                return Ok(RangeForStatement {
                    token: token.token_literal(),
                    value_var,
                    key_var,
                    range: Box::new(range_clause),
                    body: Box::new(body),
                });
            } else {
                // Couldn't find the body
                return Err(err);
            }
        }
        
        // Parse the range clause with recovery
        let range_clause = self.parse_range_clause_with_recovery()?;
        
        // Expect '{' for loop body
        if !self.current_token_is(Token::LBrace) {
            let err = self.error(&format!(
                "Expected '{{' for range for loop body, got {:?}",
                self.current_token
            ));
            
            warn!(error = %err, "Missing body in range for loop");
            
            // Create an empty body as a fallback
            return Err(err);
        }
        
        // Parse the loop body
        let body = self.parse_block_statement()?;
        
        // Create the range for statement
        Ok(RangeForStatement {
            token: token.token_literal(),
            value_var,
            key_var,
            range: Box::new(range_clause),
            body: Box::new(body),
        })
    }
}

// Extended trait combining range expression and clause error recovery
pub trait EnhancedRangeErrorRecovery<'a>: RangeClauseErrorRecovery<'a> + RangeExpressionErrorRecovery<'a> {
    /// Parse a range clause or expression with full error recovery
    ///
    /// This method combines both expression and clause error recovery
    /// to provide a robust implementation for all range parsing needs.
    ///
    /// # Returns
    ///
    /// Either a RangeClause or RangeExpression depending on context
    fn parse_range_with_full_recovery(&mut self) -> Result<Box<dyn Expression>, Error>;
    
    /// Automatically choose the right recovery method based on context
    ///
    /// # Returns
    ///
    /// The most appropriate recovery strategy for the current parsing context
    fn auto_recover_range_error(&mut self, error: Error) -> Result<Box<dyn Expression>, Error>;
}

impl<'a> EnhancedRangeErrorRecovery<'a> for Parser<'a> {
    fn parse_range_with_full_recovery(&mut self) -> Result<Box<dyn Expression>, Error> {
        // Determine if we're in a for loop context or a regular expression context
        if self.peek_token_is(Token::Identifier(String::new())) || 
           self.peek_token_is(Token::LBrace) {
            // This looks like a for loop context
            // Parse a range clause and box it as an expression
            let range_clause = self.parse_range_clause_with_recovery()?;
            Ok(Box::new(range_clause))
        } else {
            // This looks like a regular expression context
            // Use the expression recovery
            self.parse_range_expression_enhanced()
        }
    }
    
    fn auto_recover_range_error(&mut self, error: Error) -> Result<Box<dyn Expression>, Error> {
        // Log the error
        warn!(error = %error, "Automatically recovering from range error");
        
        // Create a diagnostic message
        let diagnostic = self.create_range_error_diagnostic(&error, "range parsing");
        error!(diagnostic = %diagnostic, "Range error recovery");
        
        // Return a fallback expression
        Ok(self.create_fallback_range_expression())
    }
}