use crate::ast::{self, Statement, Expression};
use crate::ast::base::Program;
use crate::error::{Error, SourceLocation, ErrorReporter};
use crate::lexer::{Lexer, Token};

use super::precedence::Precedence;

/// Parser for the CURSED language
pub struct Parser<'a> {
    pub(super) lexer: &'a mut Lexer<'a>,
    pub(super) current_token: Token,
    pub(super) peek_token: Token,
    pub(super) errors: Vec<Error>,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given lexer
    pub fn new(lexer: &'a mut Lexer<'a>) -> Result<Self, Error> {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
            errors: Vec::new(),
        };

        // Read two tokens, so current_token and peek_token are both set
        parser.next_token()?;
        parser.next_token()?;

        Ok(parser)
    }

    /// Helper method to provide debug information about current parser state
    pub(super) fn parser_state_debug(&self) -> String {
        let mut info = String::new();
        info.push_str(&format!("Parser state:\n"));
        info.push_str(&format!("  Position: line {}, column {}\n", self.lexer.line, self.lexer.column));
        info.push_str(&format!("  Current token: {:?}\n", self.current_token));
        info.push_str(&format!("  Next token: {:?}\n", self.peek_token));
        
        info
    }

    /// Move to the next token
    pub(super) fn next_token(&mut self) -> Result<(), Error> {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;
        Ok(())
    }

    /// Parse the program
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while !self.current_token_is(Token::Eof) {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(e) => self.errors.push(e),
            }
            
            // Advance to the next statement
            self.next_token()?;
        }

        Ok(program)
    }

    /// Check if the current token is of the expected type
    pub(super) fn current_token_is(&self, token: Token) -> bool {
        matches!(&self.current_token, t if std::mem::discriminant(t) == std::mem::discriminant(&token))
    }

    /// Check if the peek token is of the expected type
    pub(super) fn peek_token_is(&self, token: Token) -> bool {
        matches!(&self.peek_token, t if std::mem::discriminant(t) == std::mem::discriminant(&token))
    }

    /// Expect the peek token to be of the expected type
    pub(super) fn expect_peek(&mut self, token: Token) -> Result<(), Error> {
        if self.peek_token_is(token.clone()) {
            self.next_token()?;
            Ok(())
        } else {
            let msg = format!("expected next token to be {:?}, got {:?} instead",
                           token, self.peek_token);
            let location = SourceLocation {
                line: self.lexer.line,
                column: self.lexer.column,
                file: None,
                source_line: String::new(),
            };
            let error = Error::new("Parser", &msg, Some(location));
            self.errors.push(error.clone());
            Err(error)
        }
    }

    /// Get the precedence of the peek token
    pub(super) fn peek_precedence(&self) -> Precedence {
        match &self.peek_token {
            Token::Eq => Precedence::Equals,
            Token::NotEq => Precedence::Equals,
            Token::Lt => Precedence::LessGreater,
            Token::Gt => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Slash => Precedence::Product,
            Token::Asterisk => Precedence::Product,
            Token::Percent => Precedence::Product,
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Index,
            Token::Dot => Precedence::Dot,
            _ => Precedence::Lowest,
        }
    }

    /// Get the precedence of the current token
    pub(super) fn current_precedence(&self) -> Precedence {
        match &self.current_token {
            Token::Eq => Precedence::Equals,
            Token::NotEq => Precedence::Equals,
            Token::Lt => Precedence::LessGreater,
            Token::Gt => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Slash => Precedence::Product,
            Token::Asterisk => Precedence::Product,
            Token::Percent => Precedence::Product,
            Token::LParen => Precedence::Call,
            Token::LBracket => Precedence::Index,
            Token::Dot => Precedence::Dot,
            _ => Precedence::Lowest,
        }
    }

    /// Get the errors that occurred during parsing
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    // Helper to create an error with current location
    pub(super) fn error(&self, message: &str) -> Error {
        Error::new("Parser", message, Some(self.lexer.location()))
    }
}