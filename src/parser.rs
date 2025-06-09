/// Parser for CURSED language
use crate::error::Error;
use crate::lexer::{Lexer, Token};
use crate::ast::Program;

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, Error> {
        let current_token = Some(lexer.next_token()?);
        Ok(Self {
            lexer,
            current_token,
        })
    }
    
    pub fn parse_program(&mut self) -> Result<Program, Error> {
        Ok(Program {
            statements: Vec::new(),
        })
    }
}
