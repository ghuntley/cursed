//! Parser implementation for switch statements (vibe_check) in CURSED.
//!
//! This module provides functions to parse switch statements with proper handling
//! for string literals in case values, as required for the string switch feature.

use crate::ast;
use crate::ast::control_flow::switch::{SwitchCase, SwitchStatement};
use crate::ast::expressions::literals::StringLiteral;
use crate::ast::expressions::Identifier;
use crate::ast::statements::block::BlockStatement;
use crate::ast::{Expression, Node, Statement};
use crate::error::Error;
use crate::lexer::Token;

use super::context::{ContextAwareParsing, ParsingContext};
use super::parser::Parser;
use super::precedence::Precedence;

impl<'a> Parser<'a> {

    /// Parse a switch statement (vibe_check in CURSED)
    ///
    /// This parses a switch statement with the following form:
    /// ```
    /// vibe_check <expression> {
    ///     mood <expressions>: <statements>
    ///     mood <expressions>: <statements>
    ///     basic: <statements>
    /// }
    /// ```
    /// 
    /// Where <expressions> can be any expression, including string literals for string switches.
    /// Multiple expressions can be provided for a single case, separated by commas.
    ///
    /// # Returns
    ///
    /// A Result containing a boxed SwitchStatement if successful, or an Error if parsing fails.
    pub(super) fn parse_switch_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Push switch statement context
        self.push_context(ParsingContext::SwitchStatement);
        
        // Remember the current token for use in AST
        let token = self.current_token.clone();
        
        // Skip past 'vibe_check' token
        self.next_token()?;
        
        // Parse the switch value expression (what we're switching on)
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Handle LBrace if present, consume it
        if !self.current_token_is(Token::LBrace) {
            // Pop context before returning error
            self.pop_context();
            return Err(self.error(&format!(
                "Expected '{{' after switch expression, got {:?}",
                self.current_token
            )));
        }
        
        // Move past opening brace
        self.next_token()?; // Skip past '{'
        
        // Push switch body context
        self.push_context(ParsingContext::SwitchBody);
        
        // Parse case statements
        let mut cases = Vec::new();
        let mut default = None;

        while !self.current_token_is(Token::RBrace) && !self.current_token_is(Token::Eof) {

            // For each case, we'll directly parse the token
            if self.current_token_is(Token::Mood) {
                // Push case clause context
                self.push_context(ParsingContext::CaseClause);
                
                // Start a new case
                let token = self.current_token.clone();
                self.next_token()?; // Advance past 'mood'
                
                // Parse the first expression
                let expr = self.parse_expression(Precedence::Lowest)?;
                let mut expressions = vec![expr];
                
                // Handle multiple comma-separated expressions for this case
                while self.current_token_is(Token::Comma) || self.peek_token_is(Token::Comma) {
                    // Get to the comma
                    if !self.current_token_is(Token::Comma) {
                        self.next_token()?; // Move to comma
                    }
                    
                    // Move past the comma
                    self.next_token()?;
                    
                    // Parse the next expression
                    let next_expr = self.parse_expression(Precedence::Lowest)?;
                    expressions.push(next_expr);
                }
                
                // Expect colon after expressions
                if !self.current_token_is(Token::Colon) && !self.peek_token_is(Token::Colon) {

                    // Pop contexts before returning error
                    self.pop_context(); // Pop case clause
                    self.pop_context(); // Pop switch body
                    self.pop_context(); // Pop switch statement
                    return Err(self.error(&format!(
                        "Expected ':' after case expression, got {:?}",
                        self.current_token
                    )));
                }
                
                // Skip colon if present
                if self.current_token_is(Token::Colon) {
                    self.next_token()?; // Skip colon
                } else if self.peek_token_is(Token::Colon) {
                    self.next_token()?; // Move to colon
                    self.next_token()?; // Skip past colon
                }
                
                // Parse the statements in this case
                let mut statements = Vec::new();
                while !self.current_token_is(Token::Mood) && 
                      !self.current_token_is(Token::Basic) &&
                      !self.current_token_is(Token::RBrace) &&
                      !self.current_token_is(Token::Eof) {
                    statements.push(self.parse_statement()?);
                    self.next_token()?;
                }
                
                // Create the case statement
                if let Some(case_expr) = expressions.first() {
                    // We need to clone the case expression properly
                    let cloned_expr = if let Some(ident) = case_expr.as_any().downcast_ref::<Identifier>() {
                        Box::new(Identifier {
                            token: ident.token.clone(),
                            value: ident.value.clone(),
                        }) as Box<dyn Expression>
                    } else if let Some(str_lit) = case_expr.as_any().downcast_ref::<StringLiteral>() {
                        Box::new(StringLiteral {
                            token: str_lit.token.clone(),
                            value: str_lit.value.clone(),
                        }) as Box<dyn Expression>
                    } else {
                        // For other types, create a string representation
                        Box::new(StringLiteral {
                            token: "case".to_string(),
                            value: case_expr.string(),
                        }) as Box<dyn Expression>
                    };
                    
                    cases.push(SwitchCase {
                        value: cloned_expr,
                        statements,
                    });
                }
                
                // Pop case clause context
                self.pop_context();
            } else if self.current_token_is(Token::Basic) {
                // Push default clause context
                self.push_context(ParsingContext::DefaultClause);
                
                // Parse default case
                let token = self.current_token.clone();
                self.next_token()?; // Skip 'basic'
                
                // Expect colon after 'basic'
                if !self.current_token_is(Token::Colon) && !self.peek_token_is(Token::Colon) {
                    // Pop contexts before returning error
                    self.pop_context(); // Pop default clause
                    self.pop_context(); // Pop switch body
                    self.pop_context(); // Pop switch statement
                    return Err(self.error(&format!(
                        "Expected ':' after 'basic', got {:?}",
                        self.current_token
                    )));
                }
                
                // Skip colon if present
                if self.current_token_is(Token::Colon) {
                    self.next_token()?; // Skip colon
                } else if self.peek_token_is(Token::Colon) {
                    self.next_token()?; // Move to colon
                    self.next_token()?; // Skip past colon
                }
                
                // Parse default case statements
                let mut statements = Vec::new();
                while !self.current_token_is(Token::Mood) && 
                      !self.current_token_is(Token::Basic) &&
                      !self.current_token_is(Token::RBrace) &&
                      !self.current_token_is(Token::Eof) {
                    statements.push(self.parse_statement()?);
                    self.next_token()?;
                }
                
                default = Some(SwitchCase {
                    value: Box::new(StringLiteral {
                        token: "basic".to_string(),
                        value: "default".to_string(),
                    }),
                    statements,
                });
                
                // Pop default clause context
                self.pop_context();
            } else {
                // Pop contexts before returning error
                self.pop_context(); // Pop switch body
                self.pop_context(); // Pop switch statement
                return Err(self.error(&format!(
                    "Expected 'mood' or 'basic' in switch statement, got {:?}",
                    self.current_token
                )));
            }
        }

        // Expect closing brace
        if !self.current_token_is(Token::RBrace) {
            // Pop contexts before returning error
            self.pop_context(); // Pop switch body
            self.pop_context(); // Pop switch statement
            return Err(self.error(&format!(
                "Expected '}}' at end of switch statement, got {:?}",
                self.current_token
            )));
        }
        
        self.next_token()?; // Advance past '}'
        
        // Pop switch body context
        self.pop_context();
        
        // Pop switch statement context
        self.pop_context();

        Ok(Box::new(SwitchStatement {
            token: token.token_literal(),
            value,
            cases,
            default,
        }))
    }

    /// Parse a case statement (mood in CURSED)
    ///
    /// This parses a case statement with the following form:
    /// ```
    /// mood <expression>, <expression>, ...: <statements>
    /// ```
    ///
    /// The expressions can be of any type, including strings, which is essential
    /// for the string switch feature.
    ///
    /// # Returns
    ///
    /// A Result containing a CaseStatement if successful, or an Error if parsing fails.
    fn parse_case_statement(&mut self) -> Result<(Vec<Box<dyn Expression>>, Vec<Box<dyn Statement>>), Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'mood'

        // Parse comma-separated case values
        let mut expressions = Vec::new();

        // Parse the first expression
        expressions.push(self.parse_expression(Precedence::Lowest)?);

        // Parse additional expressions if separated by commas
        while self.peek_token_is(Token::Comma) {
            self.next_token()?; // Advance past comma
            self.next_token()?; // Advance to the next expression
            expressions.push(self.parse_expression(Precedence::Lowest)?);
        }

        // Expect colon after case values
        if self.peek_token_is(Token::Colon) {
            self.next_token()?; // Advance to ':'
        } else {
            return Err(self.error(&format!(
                "Expected ':' after case expression, got {:?}",
                self.peek_token
            )));
        }

        self.next_token()?; // Advance past ':'

        // Parse case body statements
        let mut statements = Vec::new();

        while !self.current_token_is(Token::Mood)
            && !self.current_token_is(Token::Basic)
            && !self.current_token_is(Token::RBrace)
            && !self.current_token_is(Token::Eof)
        {
            statements.push(self.parse_statement()?);
            self.next_token()?;
        }

        // Return the expressions and statements directly instead of creating a CaseStatement
        Ok((expressions, statements))
    }

    /// Parse a default case (basic in CURSED) for switch statements
    ///
    /// This parses a default case with the following form:
    /// ```
    /// basic: <statements>
    /// ```
    ///
    /// # Returns
    ///
    /// A Result containing a BlockStatement for the default case if successful,
    /// or an Error if parsing fails.
    fn parse_default_case_for_switch(&mut self) -> Result<BlockStatement, Error> {
        self.next_token()?; // Advance past 'basic'

        // Expect colon after 'basic'
        if self.peek_token_is(Token::Colon) {
            self.next_token()?; // Advance to ':'
        } else {
            return Err(self.error(&format!(
                "Expected ':' after 'basic', got {:?}",
                self.peek_token
            )));
        }

        self.next_token()?; // Advance past ':'

        // Parse default case body statements
        let mut statements = Vec::new();

        while !self.current_token_is(Token::Mood)
            && !self.current_token_is(Token::Basic)
            && !self.current_token_is(Token::RBrace)
            && !self.current_token_is(Token::Eof)
        {
            statements.push(self.parse_statement()?);
            self.next_token()?;
        }

        Ok(BlockStatement {
            token: Token::LBrace,
            statements,
        })
    }
}