//! Parser module for CURSED language

use crate::error::CursedError;
use crate::lexer::Lexer;
use crate::ast::Program;

pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Result<Self, CursedError> {
        Ok(Self {
            lexer,
            errors: Vec::new(),
        })
    }

    pub fn parse_program(&mut self) -> Result<Program, CursedError> {
        // TODO: Implement full parsing
        Ok(Program::default())
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

/// Create a new parser from source code
pub fn new_parser(source: &str) -> Result<Parser, CursedError> {
    let lexer = Lexer::new(source.to_string());
    Parser::new(lexer)
}

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED advanced features enabled".to_string())
}
