// Parser module for CURSED language
use crate::ast::{Program, Ast};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::error_types::{Error, Result};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self> {
        let current_token = match lexer.next_token() {
            Ok(token) => Some(token),
            Err(_) => None,
        };
        Ok(Parser {
            lexer,
            current_token,
        })
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        // Create a basic lexer from tokens - simplified implementation
        let mut lexer = Lexer::new(String::new());
        let current_token = tokens.first().cloned();
        Parser {
            lexer,
            current_token,
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        // Basic implementation - returns empty program for now
        Ok(Program::new())
    }

    pub fn parse(&mut self) -> Result<Ast> {
        // Basic implementation
        Ok(Ast::Program(self.parse_program()?))
    }

    pub fn errors(&self) -> Vec<Error> {
        // Return empty errors for now
        vec![]
    }

    fn next_token(&mut self) -> Result<()> {
        self.current_token = match self.lexer.next_token() {
            Ok(token) => Some(token),
            Err(_) => None,
        };
        Ok(())
    }

    fn peek_token(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }
}

// Factory function for creating new parser
pub fn new_parser(source: &str) -> Result<Parser> {
    let lexer = Lexer::new(source.to_string());
    Parser::new(lexer)
}
