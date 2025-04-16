//! Core parser implementation for the CURSED language
//!
//! This module contains the main Parser struct and its implementation,
//! responsible for transforming token streams into Abstract Syntax Trees.
//! It implements a recursive descent parser with Pratt parsing for expressions.

use crate::ast::base::Program;
use crate::ast::traits::{Expression, Statement};
use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::lexer::{Lexer, Token};

use super::precedence::Precedence;
use super::context::{ContextAwareParsing, ParsingContext};

/// Parser for the CURSED language
///
/// The Parser takes a token stream from the lexer and builds an Abstract
/// Syntax Tree (AST) representation of the program. It uses recursive descent
/// parsing for statements and Pratt parsing for expressions to handle
/// operator precedence correctly.
pub struct Parser<'a> {
    /// Reference to the lexer that provides tokens
    pub(super) lexer: &'a mut Lexer<'a>,
    /// Current token being processed
    pub(super) current_token: Token,
    /// Next token in the stream (lookahead)
    pub(super) peek_token: Token,
    /// Collection of errors encountered during parsing
    pub(super) errors: Vec<Error>,
    /// Stack of parsing contexts
    pub(super) context_stack: Vec<ParsingContext>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser with the given lexer
    ///
    /// Initializes the parser and reads the first two tokens to set up
    /// the current_token and peek_token fields.
    ///
    /// # Arguments
    ///
    /// * `lexer` - A mutable reference to a Lexer that will provide tokens
    ///
    /// # Returns
    ///
    /// A Result containing the new Parser instance or an Error if token reading fails
    #[tracing::instrument(skip(lexer), level = "debug")]
pub fn new(lexer: &'a mut Lexer<'a>) -> Result<Self, Error> {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
            errors: Vec::new(),
            context_stack: vec![ParsingContext::Statement], // Start in statement context by default
        };

        // Read two tokens, so current_token and peek_token are both set
        parser.next_token()?;
        parser.next_token()?;

        Ok(parser)
    }
    
    /// Creates a new parser with the given lexer and preprocesses tokens
    ///
    /// This version runs the token preprocessor to handle complex syntax patterns like generics
    /// before normal parsing begins.
    ///
    /// # Arguments
    ///
    /// * `lexer` - A mutable reference to a Lexer that will provide tokens
    ///
    /// # Returns
    ///
    /// A Result containing the new Parser instance with preprocessed tokens or an Error if preprocessing fails
    pub fn new_with_preprocessor(lexer: &'a mut Lexer<'a>) -> Result<Self, Error> {
        // This version doesn't fully utilize the preprocessor yet - just demonstrates the concept
        // In the future, the tokens would be stored in TokenStream and we'd need a new parser method
        // that accepts TokenStream instead of Lexer
        
        // For now, let's just create a normal parser
        Self::new(lexer)
    }

    /// Provides debug information about current parser state
    ///
    /// # Returns
    ///
    /// A string containing information about the current parser state,
    /// including position and tokens
    pub(super) fn parser_state_debug(&self) -> String {
        let mut info = String::new();
        info.push_str(&format!("Parser state:\n"));
        info.push_str(&format!(
            "  Position: line {}, column {}\n",
            self.lexer.line, self.lexer.column
        ));
        info.push_str(&format!("  Current token: {:?}\n", self.current_token));
        info.push_str(&format!("  Next token: {:?}\n", self.peek_token));

        info
    }

    /// Advances to the next token in the stream
    ///
    /// Shifts the peek_token to current_token and reads a new token
    /// from the lexer into peek_token.
    ///
    /// # Returns
    ///
    /// Result<(), Error> - Ok if successful, Error if lexer fails
    pub(super) fn next_token(&mut self) -> Result<(), Error> {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;
        Ok(())
    }

    /// Parses a complete program from the token stream
    ///
    /// Processes the entire token stream and builds an AST representation
    /// of the program, handling any parsing errors encountered.
    ///
    /// # Returns
    ///
    /// Result<Program, Error> - The parsed program AST or an error
    #[tracing::instrument(skip(self), fields(token = ?self.current_token), level = "debug")]
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while !self.current_token_is(Token::Eof) {
            match self.parse_statement() {
                Ok(stmt) => program.statements.push(stmt),
                Err(e) => self.log_error(e),
            }

            // Advance to the next statement
            self.next_token()?;
        }

        Ok(program)
    }

    /// Checks if the current token matches the expected type
    ///
    /// # Arguments
    ///
    /// * `token` - The token type to check against
    ///
    /// # Returns
    ///
    /// `true` if the current token is of the expected type, `false` otherwise
    pub(super) fn current_token_is(&self, token: Token) -> bool {
        matches!(&self.current_token, t if std::mem::discriminant(t) == std::mem::discriminant(&token))
    }

    /// Checks if the peek token matches the expected type
    ///
    /// # Arguments
    ///
    /// * `token` - The token type to check against
    ///
    /// # Returns
    ///
    /// `true` if the peek token is of the expected type, `false` otherwise
    pub(super) fn peek_token_is(&self, token: Token) -> bool {
        matches!(&self.peek_token, t if std::mem::discriminant(t) == std::mem::discriminant(&token))
    }

    /// Ensures the peek token is of the expected type and advances if it is
    ///
    /// # Arguments
    ///
    /// * `token` - The expected token type
    ///
    /// # Returns
    ///
    /// Result<(), Error> - Ok if the token matches and advances, Error otherwise
    pub(super) fn expect_peek(&mut self, token: Token) -> Result<(), Error> {
        if self.peek_token_is(token.clone()) {
            self.next_token()?;
            Ok(())
        } else {
            let msg = format!(
                "expected next token to be {:?}, got {:?} instead",
                token, self.peek_token
            );
            let location = SourceLocation {
                line: self.lexer.line,
                column: self.lexer.column,
                file: None,
                source_line: String::new(),
            };
            let error = Error::new("Parser", &msg, Some(location));
            self.log_error(error.clone());
            Err(error)
        }
    }

    /// Gets the precedence level of the peek token
    ///
    /// Used in expression parsing to determine operator precedence.
    ///
    /// # Returns
    ///
    /// The precedence level of the peek token
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

    /// Gets the precedence level of the current token
    ///
    /// Used in expression parsing to determine operator precedence.
    ///
    /// # Returns
    ///
    /// The precedence level of the current token
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

    /// Returns a slice containing all parsing errors
    ///
    /// # Returns
    ///
    /// A slice of Error objects representing parsing errors
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
    
    #[tracing::instrument(skip(self), level = "debug")]
    fn log_error(&mut self, error: Error) {
        tracing::error!(error = ?error, "Parser error encountered");
        self.errors.push(error);
    }

    /// Creates an error with the current source location
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    ///
    /// # Returns
    ///
    /// A new Error instance with the current parser location
    pub(super) fn error(&self, message: &str) -> Error {
        Error::new("Parser", message, Some(self.lexer.location()))
    }

    /// Gets a reference to the peek token without advancing
    ///
    /// # Returns
    ///
    /// A reference to the peek token
    pub(super) fn peek_token(&self) -> &Token {
        &self.peek_token
    }
}

// Implement the ContextAwareParsing trait for Parser
impl<'a> ContextAwareParsing for Parser<'a> {
    /// Push a new context onto the context stack
    fn push_context(&mut self, context: ParsingContext) {
        self.context_stack.push(context);
    }
    
    /// Pop the most recent context from the context stack
    fn pop_context(&mut self) -> Option<ParsingContext> {
        self.context_stack.pop()
    }
    
    /// Get the current parsing context
    fn current_context(&self) -> Option<&ParsingContext> {
        self.context_stack.last()
    }
    
    /// Check if we're currently in a specific context
    fn in_context(&self, context: ParsingContext) -> bool {
        self.context_stack.contains(&context)
    }
    
    /// Check if we're in any of the specified contexts
    fn in_any_context(&self, contexts: &[ParsingContext]) -> bool {
        for ctx in self.context_stack.iter() {
            if contexts.contains(ctx) {
                return true;
            }
        }
        false
    }
    
    /// Check if the current token is in a particular context
    fn current_token_is_in_context(&self, token_predicate: fn(&Self) -> bool, context: ParsingContext) -> bool {
        self.in_context(context) && token_predicate(self)
    }
}
