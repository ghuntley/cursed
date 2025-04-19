//! Parser implementation for range clauses and range-based for loops
//!
//! This module provides the parser functionality for range-based iterations,
//! including numeric ranges and container iteration.

use crate::ast::Expression;
use crate::ast::control_flow::range::{RangeClause, RangeForStatement};
use crate::ast::expressions::Identifier;
use crate::error::Error;
use crate::lexer::Token;

use super::parser::Parser;
use super::precedence::Precedence;

impl<'a> Parser<'a> {
    /// Parse a range clause (flex <start> <end> <step>)
    ///
    /// A range clause can appear in the following forms:
    /// 1. flex 10 - Range from 0 to n-1
    /// 2. flex 5, 10 - Range from start to end-1
    /// 3. flex 0, 10, 2 - Range from start to end-1 with step
    /// 4. flex container - Iterate over a container
    ///
    /// This function parses these variants and constructs the appropriate
    /// RangeClause AST node.
    pub(super) fn parse_range_clause(&mut self) -> Result<RangeClause, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'flex'
        
        // Parse the range expressions
        // First expression could be start value or container
        let first_expr = self.parse_expression(Precedence::Lowest)?;
        
        // Check if there are more expressions (indicating a numeric range)
        if self.peek_token_is(Token::Comma) {
            // This is a numeric range with start, end, and optional step
            // We just saw the start, now parse end
            self.next_token()?; // Advance to comma
            self.next_token()?; // Advance past comma
            
            // Parse the end expression
            let end_expr = self.parse_expression(Precedence::Lowest)?;
            
            // Check if there's a step value
            if self.peek_token_is(Token::Comma) {
                self.next_token()?; // Advance to comma
                self.next_token()?; // Advance past comma
                
                // Parse the step expression
                let step_expr = self.parse_expression(Precedence::Lowest)?;
                
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
            is_container: is_container,
        })
    }
    
    /// Parse a range-based for loop (bestie var := flex ...)
    ///
    /// A range-based for loop can have two forms:
    /// 1. bestie var := flex ... - Single variable iteration
    /// 2. bestie key, var := flex ... - Key-value (or index-value) iteration
    ///
    /// This function parses these forms and constructs a RangeForStatement AST node.
    pub(super) fn parse_range_for_statement(&mut self) -> Result<RangeForStatement, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'bestie'
        
        // Parse the variable(s) for the loop
        let mut key_var = None;
        let mut value_var = String::new();
        
        // Parse first variable
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!(
                "Expected identifier for range variable, got {:?}",
                self.current_token
            )));
        }
        
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
                return Err(self.error(&format!(
                    "Expected identifier for second range variable, got {:?}",
                    self.current_token
                )));
            }
            
            // Get the second variable name
            let second_var = match &self.current_token {
                Token::Identifier(ident) => ident.clone(),
                _ => unreachable!(),
            };
            
            // Set key_var and value_var
            key_var = Some(first_var);
            value_var = second_var;
            
            self.next_token()?; // Advance past second variable
        } else {
            // Only one variable
            value_var = first_var;
        }
        
        // Expect ':='
        if !self.current_token_is(Token::DeclAssign) {
            return Err(self.error(&format!(
                "Expected ':=' in range for loop, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past ':='
        
        // Expect 'flex' keyword
        if !self.current_token_is(Token::Flex) {
            return Err(self.error(&format!(
                "Expected 'flex' in range for loop, got {:?}",
                self.current_token
            )));
        }
        
        // Parse the range clause
        let range_clause = self.parse_range_clause()?;
        
        // Expect '{' for loop body
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!(
                "Expected '{{' for range for loop body, got {:?}",
                self.current_token
            )));
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