//! Minimal parser for CURSED - just enough to parse basic programs
use crate::error::Error;
use crate::minimal_ast::*;
use crate::minimal_lexer::{Lexer, Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, Error> {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        
        Ok(Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        })
    }
    
    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }
    
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token.token_type == token_type {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }
    
    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
    
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program {
            statements: Vec::new(),
        };
        
        while self.current_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        
        Ok(program)
    }
    
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Facts => self.parse_facts_statement(),
            TokenType::Slay => self.parse_slay_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    
    fn parse_facts_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        if !self.expect_peek(TokenType::Assign) {
            return None;
        }
        
        self.next_token();
        let value = self.parse_expression()?;
        
        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }
        
        Some(Statement::Facts(name, value))
    }
    
    fn parse_slay_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let name = self.current_token.literal.clone();
        
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }
        
        let parameters = self.parse_function_parameters();
        
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        
        let body = self.parse_block_statement();
        
        Some(Statement::Slay(name, parameters, body))
    }
    
    fn parse_function_parameters(&mut self) -> Vec<String> {
        let mut parameters = Vec::new();
        
        if self.peek_token.token_type == TokenType::RightParen {
            self.next_token();
            return parameters;
        }
        
        self.next_token();
        
        if self.current_token.token_type == TokenType::Identifier {
            parameters.push(self.current_token.literal.clone());
        }
        
        while self.peek_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            if self.current_token.token_type == TokenType::Identifier {
                parameters.push(self.current_token.literal.clone());
            }
        }
        
        if !self.expect_peek(TokenType::RightParen) {
            return Vec::new();
        }
        
        parameters
    }
    
    fn parse_block_statement(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        
        self.next_token();
        
        while self.current_token.token_type != TokenType::RightBrace && 
              self.current_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        
        statements
    }
    
    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression()?;
        
        if self.peek_token.token_type == TokenType::Semicolon {
            self.next_token();
        }
        
        Some(Statement::Expression(expr))
    }
    
    fn parse_expression(&mut self) -> Option<Expression> {
        match self.current_token.token_type {
            TokenType::Identifier => {
                let ident = self.current_token.literal.clone();
                
                // Check if it's a function call
                if self.peek_token.token_type == TokenType::LeftParen {
                    self.next_token();
                    let args = self.parse_expression_list(TokenType::RightParen);
                    Some(Expression::FunctionCall(ident, args))
                } else {
                    Some(Expression::Identifier(ident))
                }
            },
            TokenType::Integer => {
                if let Ok(value) = self.current_token.literal.parse::<i64>() {
                    Some(Expression::Integer(value))
                } else {
                    None
                }
            },
            TokenType::Boolean => {
                let value = self.current_token.literal == "true";
                Some(Expression::Boolean(value))
            },
            TokenType::String => {
                Some(Expression::String(self.current_token.literal.clone()))
            },
            _ => None,
        }
    }
    
    fn parse_expression_list(&mut self, end: TokenType) -> Vec<Expression> {
        let mut args = Vec::new();
        
        if self.peek_token.token_type == end {
            self.next_token();
            return args;
        }
        
        self.next_token();
        if let Some(expr) = self.parse_expression() {
            args.push(expr);
        }
        
        while self.peek_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            if let Some(expr) = self.parse_expression() {
                args.push(expr);
            }
        }
        
        if !self.expect_peek(end) {
            return Vec::new();
        }
        
        args
    }
}
