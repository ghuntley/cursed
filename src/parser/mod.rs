/// CURSED Language Parser
/// 
/// This module implements a recursive descent parser for the CURSED programming language.
/// The parser handles all language constructs with proper operator precedence and
/// comprehensive error recovery.

mod mod_parser_expressions;
mod mod_parser_statements;

use crate::ast::*;
use crate::error::{Error, SourceLocation};
use crate::lexer::{Token, TokenType, Lexer};
use std::collections::VecDeque;

/// Operator precedence levels for expression parsing
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 1,
    Channel = 2,     // <-
    Equals = 3,      // ==, !=
    LessGreater = 4, // > or <
    Sum = 5,         // +, -
    Product = 6,     // *, /, %
    Prefix = 7,      // -x, !x
    Call = 8,        // myFunction(x)
    Index = 9,       // array[index]
}

impl From<&TokenType> for Precedence {
    fn from(token_type: &TokenType) -> Self {
        match token_type {
            TokenType::LeftArrow => Precedence::Channel,
            TokenType::Equal | TokenType::NotEqual => Precedence::Equals,
            TokenType::LessThan | TokenType::LessThanEqual |
            TokenType::GreaterThan | TokenType::GreaterThanEqual => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Multiply | TokenType::Divide | TokenType::Modulo => Precedence::Product,
            TokenType::LeftParen => Precedence::Call,
            TokenType::LeftBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}

/// Main parser structure with error recovery
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    sync_tokens: Vec<TokenType>,
}

impl Parser {
    /// Create a new parser instance
    pub fn new(mut lexer: Lexer) -> Result<Self, Error> {
        let current_token = lexer.next_token()?;
        let peek_token = lexer.next_token()?;
        
        Ok(Self {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
            sync_tokens: vec![
                TokenType::Slay,      // function
                TokenType::Sus,       // variable
                TokenType::Facts,     // constant
                TokenType::Squad,     // struct
                TokenType::Collab,    // interface
                TokenType::VibeCheck, // switch
                TokenType::Lowkey,    // if
                TokenType::Bestie,    // for
                TokenType::Periodt,   // while
                TokenType::Yolo,      // return
            ],
        })
    }
    
    /// Parse a complete CURSED program
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program::new();
        
        // Skip initial newlines
        self.skip_newlines();
        
        // Parse package declaration if present
        if self.current_token_is(&TokenType::Vibe) {
            if let Some(package_name) = self.parse_package_declaration()? {
                program.package_name = Some(package_name);
            }
            self.skip_newlines();
        }
        
        // Parse imports
        while self.current_token_is(&TokenType::Yeet) {
            let import = self.parse_import_statement()?;
            program.add_import(import);
            self.skip_newlines();
        }
        
        // Parse top-level declarations and statements
        while !self.current_token_is(&TokenType::Eof) {
            self.skip_newlines();
            if self.current_token_is(&TokenType::Eof) {
                break;
            }
            
            match self.parse_statement() {
                Ok(stmt) => program.add_statement(stmt),
                Err(e) => {
                    self.errors.push(e.to_string());
                    self.synchronize();
                }
            }
        }
        
        if !self.errors.is_empty() {
            return Err(Error::Parse(format!("Parse errors: {}", self.errors.join(", "))));
        }
        
        Ok(program)
    }
    
    /// Get parser errors
    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }
    
    // Token manipulation helpers
    fn advance_token(&mut self) -> Result<(), Error> {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;
        Ok(())
    }
    
    fn current_token_is(&self, token_type: &TokenType) -> bool {
        self.current_token.token_type == *token_type
    }
    
    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.token_type == *token_type
    }
    
    fn expect_token(&mut self, token_type: TokenType) -> Result<Token, Error> {
        if self.current_token.token_type == token_type {
            let token = self.current_token.clone();
            self.advance_token()?;
            Ok(token)
        } else {
            Err(Error::Parse(format!(
                "Expected {:?}, got {:?} at line {} column {}",
                token_type,
                self.current_token.token_type,
                self.current_token.location.line,
                self.current_token.location.column
            )))
        }
    }
    
    fn skip_newlines(&mut self) {
        while self.current_token_is(&TokenType::Newline) {
            if let Err(_) = self.advance_token() {
                break;
            }
        }
    }
    
    fn current_precedence(&self) -> Precedence {
        Precedence::from(&self.current_token.token_type)
    }
    
    fn peek_precedence(&self) -> Precedence {
        Precedence::from(&self.peek_token.token_type)
    }
    
    // Error recovery
    fn synchronize(&mut self) {
        while !self.current_token_is(&TokenType::Eof) {
            if self.sync_tokens.contains(&self.current_token.token_type) {
                return;
            }
            if let Err(_) = self.advance_token() {
                return;
            }
        }
    }
    
    fn parse_package_declaration(&mut self) -> Result<Option<String>, Error> {
        self.expect_token(TokenType::Vibe)?;
        
        if !self.current_token_is(&TokenType::Identifier) {
            return Err(Error::Parse("Expected package name after 'vibe'".to_string()));
        }
        
        let package_name = self.current_token.literal.clone();
        self.advance_token()?;
        
        Ok(Some(package_name))
    }
    
    fn parse_import_statement(&mut self) -> Result<ImportStatement, Error> {
        let token = self.expect_token(TokenType::Yeet)?;
        
        // Handle aliased imports: yeet alias "path" 
        let (alias, path) = if self.current_token_is(&TokenType::Identifier) && self.peek_token_is(&TokenType::String) {
            let alias = self.current_token.literal.clone();
            self.advance_token()?;
            let path = self.current_token.literal.clone();
            self.advance_token()?;
            (Some(alias), path)
        } else if self.current_token_is(&TokenType::String) {
            let path = self.current_token.literal.clone();
            self.advance_token()?;
            (None, path)
        } else {
            return Err(Error::Parse("Expected import path".to_string()));
        };
        
        Ok(if let Some(alias) = alias {
            ImportStatement::with_alias(token, path, alias)
        } else {
            ImportStatement::new(token, path)
        })
    }
    

}
