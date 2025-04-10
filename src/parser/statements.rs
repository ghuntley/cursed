use crate::ast::{self, Statement, Expression, Node};
use crate::ast::statements::*;
use crate::ast::control_flow::*;
use crate::ast::declarations::*;
use crate::ast::expressions::Identifier;
use crate::error::Error;
use crate::lexer::Token;
use std::any::Any;

use super::precedence::Precedence;
use super::parser::Parser;

impl<'a> Parser<'a> {
    /// Parse a statement based on the current token
    pub(super) fn parse_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        match &self.current_token {
            Token::Sus => self.parse_var_statement(),
            Token::Facts => self.parse_const_statement(),
            Token::Yolo => self.parse_return_statement(),
            Token::LBrace => {
                let block = self.parse_block_statement()?;
                Ok(Box::new(block))
            },
            Token::Lowkey => self.parse_if_statement(),
            Token::Bestie => self.parse_for_statement(),
            Token::Periodt => self.parse_while_statement(),
            Token::Ghosted => self.parse_break_statement(),
            Token::Simp => self.parse_continue_statement(),
            Token::Normie | Token::Tea | Token::Thicc | Token::Smol | Token::Mid | Token::Lit | Token::Snack | Token::Meal => {
                // Check if this is a type followed by identifier and :=
                if self.is_decl_assign() {
                    self.parse_decl_assign_statement()
                } else {
                    self.parse_expression_statement()
                }
            },
            _ => self.parse_expression_statement()
        }
    }
    
    /// Check if the current token is part of a declaration-assignment
    fn is_decl_assign(&mut self) -> bool {
        // Ensure we have enough tokens to check: type identifier :=
        if !matches!(self.peek_token, Token::Identifier(_)) {
            return false;
        }
        
        // Look ahead to find :=
        let current_token = self.current_token.clone();
        let peek_token = self.peek_token.clone();
        let mut result = false;
        
        // Try to peek ahead
        if let Ok(_) = self.next_token() { // Move to identifier
            if self.peek_token_is(Token::DeclAssign) {
                result = true;
            }
        }
        
        // Restore state
        self.current_token = current_token;
        self.peek_token = peek_token;
        
        result
    }
    
    /// Parse a declaration-assignment statement (tea x := 5)
    pub(super) fn parse_decl_assign_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let type_token = self.current_token.clone();
        let type_name = type_token.token_literal();
        
        self.next_token()?; // Advance past type token
        
        // Parse identifier
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!("Expected identifier, got {:?}", self.current_token)));
        }
        
        // Get variable name
        let name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };
        
        self.next_token()?; // Advance past identifier
        
        // Expect ':=' token
        if !self.current_token_is(Token::DeclAssign) {
            return Err(self.error(&format!("Expected ':=', got {:?}", self.current_token)));
        }
        
        self.next_token()?; // Advance past ':='
        
        // Parse value expression
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Expect semicolon
        self.expect_semicolon()?;
        
        // Create a simple expression statement instead since we can't access DeclAssignStatement
        use crate::ast::statements::expressions::ExpressionStatement;
        Ok(Box::new(ExpressionStatement {
            token: type_token.token_literal(),
            expression: Some(value)
        }))
    }
    
    /// Parse a variable declaration statement (sus x = 5)
    pub(super) fn parse_var_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'sus'
        
        // Parse identifier
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!("Expected identifier, got {:?}", self.current_token)));
        }
        
        // Get variable name
        let name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };
        
        self.next_token()?; // Advance past identifier
        
        // Parse type annotation if present
        let mut type_annotation = None;
        if !self.current_token_is(Token::Assign) {
            // This should be a type name
            let type_token = self.current_token.clone();
            self.next_token()?; // Advance past type name
            type_annotation = Some(type_token);
        }
        
        // Expect '=' token
        if !self.current_token_is(Token::Assign) {
            return Err(self.error(&format!("Expected '=', got {:?}", self.current_token)));
        }
        
        self.next_token()?; // Advance past '='
        
        // Parse value expression
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Expect semicolon
        self.expect_semicolon()?;
        
        Ok(Box::new(ast::statements::declarations::LetStatement {
            token: token.token_literal(),
            name,
            type_annotation,
            value: Some(value),
        }))
    }
    
    /// Parse a constant declaration statement (facts PI = 3.14)
    pub(super) fn parse_const_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'facts'
        
        // Parse identifier
        if !matches!(self.current_token, Token::Identifier(_)) {
            return Err(self.error(&format!("Expected identifier, got {:?}", self.current_token)));
        }
        
        // Get constant name
        let name = match &self.current_token {
            Token::Identifier(ident) => ast::Identifier {
                token: self.current_token.token_literal(),
                value: ident.clone(),
            },
            _ => unreachable!(),
        };
        
        self.next_token()?; // Advance past identifier
        
        // Expect '=' token
        if !self.current_token_is(Token::Assign) {
            return Err(self.error(&format!("Expected '=', got {:?}", self.current_token)));
        }
        
        self.next_token()?; // Advance past '='
        
        // Parse value expression
        let value = self.parse_expression(Precedence::Lowest)?;
        
        // Expect semicolon
        self.expect_semicolon()?;
        
        // Create a proper FactsStatement instance
        Ok(Box::new(ast::statements::declarations::FactsStatement {
            token: token.token_literal(),
            name: Box::new(name) as Box<dyn ast::Expression>,
            value,
        }))
    }
    
    /// Parse an expression statement
    pub(super) fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        
        // Parse the expression
        let expression = self.parse_expression(Precedence::Lowest)?;
        
        // Semicolon is optional for expression statements
        if self.peek_token_is(Token::Semicolon) {
            self.next_token()?; // Advance past semicolon
        }
        
        Ok(Box::new(ast::statements::expressions::ExpressionStatement {
            token: token.token_literal(),
            expression: Some(expression),
        }))
    }
    
    /// Parse a return statement (yolo <expression>)
    pub(super) fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'yolo'
        
        // Parse the return value expression (if any)
        let return_value = if self.current_token_is(Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression(Precedence::Lowest)?)
        };
        
        self.expect_semicolon()?;
        
        Ok(Box::new(ast::statements::declarations::ReturnStatement {
            token: token.token_literal(),
            return_value,
        }))
    }
    
    /// Parse a block statement ({ ... })
    pub(super) fn parse_block_statement(&mut self) -> Result<ast::statements::block::BlockStatement, Error> {
        let token = self.current_token.clone();
        let mut statements = Vec::new();
        
        self.next_token()?; // Advance past '{'
        
        while !self.current_token_is(Token::RBrace) && !self.current_token_is(Token::Eof) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token()?;
        }
        
        // Check if we have a closing brace
        if !self.current_token_is(Token::RBrace) {
            return Err(self.error(&format!("Expected '}}', got {:?}", self.current_token)));
        }
        
        Ok(ast::statements::block::BlockStatement {
            token: token.token_literal(),
            statements,
        })
    }
    
    /// Parse a break statement (ghosted)
    pub(super) fn parse_break_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'ghosted'
        
        self.expect_semicolon()?;
        
        Ok(Box::new(ast::control_flow::loops::BreakStatement {
            token: token.token_literal(),
        }))
    }
    
    /// Parse a continue statement (simp)
    pub(super) fn parse_continue_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        // Create a custom ContinueStatement if not defined in AST
        struct ContinueStatement {
            token: String,
        }
        
        impl Node for ContinueStatement {
            fn token_literal(&self) -> String {
                self.token.clone()
            }
            
            fn string(&self) -> String {
                format!("{};", self.token)
            }
        }
        
        impl Statement for ContinueStatement {
            fn statement_node(&self) {}
            fn as_any(&self) -> &dyn Any { self }
        }
        
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'simp'
        
        self.expect_semicolon()?;
        
        Ok(Box::new(ContinueStatement {
            token: token.token_literal(),
        }))
    }
    
    /// Parse an if statement (lowkey <condition> { ... } highkey { ... })
    pub(super) fn parse_if_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'lowkey'
        
        // Handle optional parentheses around condition
        let has_parens = self.current_token_is(Token::LParen);
        if has_parens {
            self.next_token()?; // Advance past '('
        }
        
        // Parse condition
        let condition = self.parse_expression(Precedence::Lowest)?;
        
        // Handle closing parenthesis if they were used
        if has_parens {
            if !self.current_token_is(Token::RParen) {
                return Err(self.error(&format!("Expected ')' after condition, got {:?}", self.current_token)));
            }
            self.next_token()?; // Advance past ')'
        }
        
        // Expect '{' for consequence block
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!("Expected '{{' after condition, got {:?}", self.current_token)));
        }
        
        // Parse consequence block
        let consequence = self.parse_block_statement()?;
        
        // Check for 'highkey' (else block)
        let mut alternative = None;
        if self.peek_token_is(Token::Highkey) {
            self.next_token()?; // Advance to 'highkey'
            self.next_token()?; // Advance past 'highkey'
            
            // Check if this is an 'else if' (highkey lowkey ...)
            if self.current_token_is(Token::Lowkey) {
                // We need to wrap the if statement in a block statement
                let if_stmt = self.parse_if_statement()?;
                let mut stmts = Vec::new();
                stmts.push(if_stmt);
                alternative = Some(ast::statements::block::BlockStatement {
                    token: "{".to_string(),
                    statements: stmts,
                });
            } else {
                // Regular else block
                if !self.current_token_is(Token::LBrace) {
                    return Err(self.error(&format!("Expected '{{' after highkey, got {:?}", self.current_token)));
                }
                alternative = Some(self.parse_block_statement()?);
            }
        }
        
        Ok(Box::new(ast::control_flow::conditionals::IfStatement {
            token: token.token_literal(),
            condition,
            consequence: Box::new(consequence),
            alternative: alternative.map(Box::new),
        }))
    }
    
    /// Parse a for statement (bestie <init>; <condition>; <post> { ... })
    pub(super) fn parse_for_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'bestie'
        
        // Handle different forms of for loops
        let mut init = None;
        let mut condition = None;
        let mut post = None;
        
        // Check if this is a C-style for loop: bestie i := 0; i < 10; i++ { ... }
        if !self.current_token_is(Token::LBrace) {
            // This is not a simple infinite loop
            
            // Parse initialization if not immediately followed by semicolon
            if !self.current_token_is(Token::Semicolon) {
                init = Some(self.parse_statement()?);
                // Don't advance past semicolon, as parse_statement will have done that
            } else {
                self.next_token()?; // Advance past semicolon
            }
            
            // Parse condition if not immediately followed by semicolon
            if !self.current_token_is(Token::Semicolon) {
                let expr = self.parse_expression(Precedence::Lowest)?;
                condition = Some(expr);
                self.next_token()?; // Advance past condition to semicolon
            } else {
                self.next_token()?; // Advance past semicolon
            }
            
            // Parse post-iteration statement if not immediately followed by opening brace
            if !self.current_token_is(Token::LBrace) {
                let expr = self.parse_expression(Precedence::Lowest)?;
                post = Some(expr);
                self.next_token()?; // Advance past post-expression
            }
        }
        
        // Expect '{' for the loop body
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!("Expected '{{' for for loop body, got {:?}", self.current_token)));
        }
        
        let body = self.parse_block_statement()?;
        
        Ok(Box::new(ast::control_flow::loops::ForStatement {
            token: token.token_literal(),
            init,
            condition,
            post: None, // The post field might have a different type in the actual AST
            body: Box::new(body),
        }))
    }
    
    /// Parse a while statement (periodt <condition> { ... })
    pub(super) fn parse_while_statement(&mut self) -> Result<Box<dyn Statement>, Error> {
        let token = self.current_token.clone();
        self.next_token()?; // Advance past 'periodt'
        
        // Handle optional parentheses around condition
        let has_parens = self.current_token_is(Token::LParen);
        if has_parens {
            self.next_token()?; // Advance past '('
        }
        
        // Parse condition
        let condition = self.parse_expression(Precedence::Lowest)?;
        
        // Handle closing parenthesis if they were used
        if has_parens {
            if !self.current_token_is(Token::RParen) {
                return Err(self.error(&format!("Expected ')' after while condition, got {:?}", self.current_token)));
            }
            self.next_token()?; // Advance past ')'
        }
        
        // Expect '{' for the loop body
        if !self.current_token_is(Token::LBrace) {
            return Err(self.error(&format!("Expected '{{' for while loop body, got {:?}", self.current_token)));
        }
        
        let body = self.parse_block_statement()?;
        
        Ok(Box::new(ast::control_flow::loops::WhileStatement {
            token: token.token_literal(),
            condition,
            body: Box::new(body),
        }))
    }
    
    /// Expect semicolon and advance past it if found
    fn expect_semicolon(&mut self) -> Result<(), Error> {
        if self.peek_token_is(Token::Semicolon) {
            self.next_token()?; // Advance past semicolon
        }
        Ok(())
    }
}