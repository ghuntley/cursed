//! Parser implementation for channel range clauses and range-based for loops over channels
//!
//! This module provides the parser functionality for channel range-based iterations,
//! allowing for-range loops to iterate over values received from channels.

use crate::ast::control_flow::channel_range::{ChannelRangeClause, ChannelRangeForStatement, ChannelClosureDetection};
use crate::ast::expressions::Identifier;
use crate::error::Error;
use crate::lexer::Token;
use super::parser::Parser;
use super::precedence::Precedence;

impl<'a> Parser<'a> {
    /// Parse a channel range clause (flex <-channel)
    ///
    /// A channel range clause appears in the following form:
    /// - flex <-channel - Iterate over values received from the channel
    ///
    /// This function parses the channel receive expression and constructs
    /// the appropriate ChannelRangeClause AST node.
    pub(super) fn parse_channel_range_clause(&mut self) -> Result<ChannelRangeClause, Error> {
        let token = self.current_token.clone();
        
        // We should already be at 'flex'
        if !self.current_token_is(Token::Flex) {
            return Err(self.error(&format!(
                "Expected 'flex' for channel range, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past 'flex'
        
        // Expect '<-' for channel receive
        if !self.current_token_is(Token::Arrow) {
            return Err(self.error(&format!(
                "Expected '<-' for channel range, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past '<-'
        
        // Parse the channel expression
        let channel_expr = self.parse_expression(Precedence::Lowest)?;
        
        Ok(ChannelRangeClause {
            token,
            channel: channel_expr,
            with_ok: false, // Default to false, will be set based on variable count
        })
    }
    
    /// Parse a channel range for statement (bestie var := flex <-channel ...)
    ///
    /// A channel range for loop can have two forms:
    /// 1. bestie var := flex <-channel - Single variable iteration (value only)
    /// 2. bestie value, ok := flex <-channel - Two variable iteration (value and closure status)
    ///
    /// This function parses these forms and constructs a ChannelRangeForStatement AST node.
    pub(super) fn parse_channel_range_for_statement(&mut self) -> Result<ChannelRangeForStatement, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'bestie'
        
        // Parse the variable(s) for the loop
        let mut ok_var = None;
        let mut value_var = String::new();
        
        // Parse first variable
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!(
                "Expected identifier for channel range variable, got {:?}",
                self.current_token
            )));
        }
        
        // Get the variable name
        let first_var = match &self.current_token {
            Token::Identifier(ident) => ident.clone(),
            _ => unreachable!(),
        };
        
        self.next_token()?; // Advance past first variable
        
        // Check if we have a second variable (value, ok := ...)
        if self.current_token_is(Token::Comma) {
            self.next_token()?; // Advance past comma
            
            // Parse second variable
            if !matches!(self.current_token, Token::Identifier(_)) {
                return Err(self.error(&format!(
                    "Expected identifier for second channel range variable, got {:?}",
                    self.current_token
                )));
            }
            
            // Get the second variable name
            let second_var = match &self.current_token {
                Token::Identifier(ident) => ident.clone(),
                _ => unreachable!(),
            };
            
            // Set value_var and ok_var
            value_var = first_var;
            ok_var = Some(second_var);
            
            self.next_token()?; // Advance past second variable
        } else {
            // Only one variable
            value_var = first_var;
        }
        
        // Expect ':='
        if !self.current_token_is(Token::DeclAssign) {
            return Err(self.error(&format!(
                "Expected ':=' in channel range for loop, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past ':='
        
        // Parse the channel range clause
        let mut channel_range = self.parse_channel_range_clause()?;
        
        // Set the with_ok flag based on whether we have two variables
        channel_range.with_ok = ok_var.is_some();
        
        // Expect '{' for loop body
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!(
                "Expected '{{' for channel range for loop body, got {:?}",
                self.current_token
            )));
        }
        
        // Parse the loop body
        let body = self.parse_block_statement()?;
        
        // Create the channel range for statement
        Ok(ChannelRangeForStatement {
            token: token,
            value_var,
            ok_var,
            channel_range: Box::new(channel_range),
            body: Box::new(body),
        })
    }
    
    /// Parse a channel closure detection expression (closed(channel))
    ///
    /// This function parses expressions that check if a channel is closed.
    /// Used for manually checking channel status outside of range loops.
    pub(super) fn parse_channel_closure_detection(&mut self) -> Result<ChannelClosureDetection, Error> {
        let token = self.current_token.clone();
        
        // Expect 'closed' keyword
        if !matches!(self.current_token, Token::Identifier(ref name) if name == "closed") {
            return Err(self.error(&format!(
                "Expected 'closed' for channel closure detection, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past 'closed'
        
        // Expect '('
        if !self.current_token_is(Token::LParen) {
            return Err(self.error(&format!(
                "Expected '(' after 'closed', got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past '('
        
        // Parse the channel expression
        let channel_expr = self.parse_expression(Precedence::Lowest)?;
        
        // Expect ')'
        if !self.current_token_is(Token::RParen) {
            return Err(self.error(&format!(
                "Expected ')' after channel expression, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past ')'
        
        Ok(ChannelClosureDetection {
            token,
            channel: channel_expr,
        })
    }
    
    /// Check if the current context is a channel range (flex <-...)
    ///
    /// This helper function determines if we're parsing a channel range
    /// as opposed to a regular range clause.
    pub(super) fn is_channel_range(&mut self) -> bool {
        // Check if we have 'flex' followed by '<-'
        if self.current_token_is(Token::Flex) {
            // Look ahead to see if the next token is '<-'
            self.peek_token_is(Token::Arrow)
        } else {
            false
        }
    }
    
    /// Check if the current context is a channel range for statement
    ///
    /// This helper function determines if we're parsing a channel range for statement
    /// as opposed to a regular range-based for loop.
    pub(super) fn is_channel_range_for_statement(&mut self) -> bool {
        // We need to look ahead to see the pattern:
        // bestie var := flex <-...
        // or
        // bestie var1, var2 := flex <-...
        
        if !self.current_token_is(Token::Bestie) {
            return false;
        }
        
        // Save current position
        let saved_current = self.current_token.clone();
        let saved_peek = self.peek_token.clone();
        
        // Look ahead to find the pattern
        let mut is_channel_range = false;
        
        // This is a simplified check - in a real implementation, you might need
        // more sophisticated lookahead logic
        if let (Token::Identifier(_), Token::DeclAssign) = (&self.peek_token, &self.peek_token) {
            // Simple case: bestie var := ...
            // Would need to look further ahead to see 'flex <-'
            // For now, we'll do a simple heuristic
        }
        
        // For now, return false and let the normal parsing logic handle it
        // The actual detection will happen during parsing
        false
    }
}
