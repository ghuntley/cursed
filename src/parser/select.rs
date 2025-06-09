//! Parser implementation for select statements
//!
//! This module provides parsing functionality for select statements,
//! which allow non-blocking operations on multiple channels.

use crate::ast::control_flow::select::*;
use crate::ast::expressions::channel::{SendExpression, ReceiveExpression};
use crate::ast::traits::{Expression, Statement};
use crate::error::Error;
use crate::lexer::Token;
use crate::parser::precedence::Precedence;
use crate::parser::Parser;

impl<'a> Parser<'a> {
    /// Parse a select statement
    ///
    /// Syntax: choose { mood <expression>: ... basic: ... }
    pub fn parse_select_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        
        if !matches!(self.current_token, Token::Choose) {
            return Err(Error::new("parse", format!(
                "Expected 'choose', got {:?}",
                self.current_token
            ), None));
        }
        
        self.next_token(); // consume 'choose'
        
        if !matches!(self.current_token, Token::LBrace) {
            return Err(Error::new("parse", format!(
                "Expected '{{' after 'choose', got {:?}",
                self.current_token
            ), None));
        }
        
        self.next_token(); // consume '{'
        
        let mut cases = Vec::new();
        let mut default_case = None;
        
        while !matches!(self.current_token, Token::RBrace) && !matches!(self.current_token, Token::Eof) {
            match &self.current_token {
                Token::Mood => {
                    let case = self.parse_select_case()?;
                    cases.push(case);
                }
                Token::Basic => {
                    if default_case.is_some() {
                        return Err(Error::new("parse", "Multiple default cases in select statement", None));
                    }
                    default_case = Some(self.parse_default_case()?);
                }
                _ => {
                    return Err(Error::new("parse", format!(
                        "Expected 'mood' or 'basic' in select statement, got {:?}",
                        self.current_token
                    ), None));
                }
            }
        }
        
        if !matches!(self.current_token, Token::RBrace) {
            return Err(Error::new("parse", format!(
                "Expected '}}' to close select statement, got {:?}",
                self.current_token
            ), None));
        }
        
        self.next_token(); // consume '}'
        
        if cases.is_empty() && default_case.is_none() {
            return Err(Error::new("parse", "Select statement must have at least one case", None));
        }
        
        Ok(Box::new(SelectStatement {
            token,
            cases,
            default: default_case,
        }))
    }
    
    /// Parse a select case (mood case)
    fn parse_select_case(&mut self) -> Result<SelectCase, Error> {
        let token = self.current_token.clone();
        
        self.next_token(); // consume 'mood'
        
        // Parse the communication expression (send or receive)
        let communication = self.parse_communication_expression()?;
        
        if !matches!(self.current_token, Token::Colon) {
            return Err(Error::new("parse", format!(
                "Expected ':' after select case communication, got {:?}",
                self.current_token
            ), None));
        }
        
        self.next_token(); // consume ':'
        
        // Parse the statements in this case
        let mut statements = Vec::new();
        
        while !matches!(self.current_token, Token::Mood) 
            && !matches!(self.current_token, Token::Basic)
            && !matches!(self.current_token, Token::RBrace)
            && !matches!(self.current_token, Token::Eof) {
            
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        
        Ok(SelectCase {
            token,
            communication,
            statements,
        })
    }
    
    /// Parse a default case in a select statement
    fn parse_default_case(&mut self) -> Result<DefaultCase, Error> {
        let token = self.current_token.clone();
        
        self.next_token(); // consume 'basic'
        
        if !matches!(self.current_token, Token::Colon) {
            return Err(Error::new("parse", format!(
                "Expected ':' after 'basic', got {:?}",
                self.current_token
            ), None));
        }
        
        self.next_token(); // consume ':'
        
        // Parse the statements in the default case
        let mut statements = Vec::new();
        
        while !matches!(self.current_token, Token::Mood) 
            && !matches!(self.current_token, Token::Basic)
            && !matches!(self.current_token, Token::RBrace)
            && !matches!(self.current_token, Token::Eof) {
            
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        
        Ok(DefaultCase {
            token,
            statements,
        })
    }
    
    /// Parse a communication expression (send or receive)
    fn parse_communication_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        // Check if this is a receive expression (<-ch)
        if matches!(self.current_token, Token::Arrow) {
            return self.parse_receive_expression();
        }
        
        // Otherwise, try to parse as a send expression (ch <- value)
        let left = self.parse_expression(Precedence::Lowest)?;
        
        if matches!(self.current_token, Token::Arrow) {
            // This is a send expression
            self.next_token(); // consume '<-'
            let value = self.parse_expression(Precedence::Lowest)?;
            
            let send_expr = SendExpression {
                token: Token::Arrow,
                channel: left,
                value,
            };
            
            return Ok(Box::new(send_expr));
        }
        
        // If it's not a send or receive, it's an error
        Err(Error::new("parse", format!(
            "Expected send or receive expression in select case, got {:?}",
            left.string()
        ), None))
    }
    
    /// Parse a receive expression for select statements
    fn parse_receive_expression(&mut self) -> Result<Box<dyn Expression>, Error> {
        let token = self.current_token.clone();
        
        self.next_token(); // consume '<-'
        
        let channel = self.parse_expression(Precedence::Lowest)?;
        
        let receive_expr = ReceiveExpression {
            token,
            channel,
            element_type: "any".to_string(), // Will be inferred during type checking
        };
        
        Ok(Box::new(receive_expr))
    }
}
