//! Core preprocessor functionality

use crate::error::CursedError;
use crate::lexer::Lexer;
use crate::preprocessor::{TokenStream, PreprocessorResult};

#[derive(Debug)]
pub struct Preprocessor {
    lexer: Lexer,
    macros: Vec<Macro>,
    includes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub definition: String,
    pub parameters: Vec<String>,
}

impl Preprocessor {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            macros: Vec::new(),
            includes: Vec::new(),
        }
    }

    pub fn add_macro(&mut self, name: String, definition: String, parameters: Vec<String>) {
        self.macros.push(Macro { name, definition, parameters });
    }

    pub fn process(&mut self) -> PreprocessorResult<TokenStream> {
        // Create a basic token stream from lexer
        let tokens = self.lexer.tokenize().map_err(|e| crate::preprocessor::PreprocessorError::LexError(e.to_string()))?;
        Ok(TokenStream::new(tokens))
    }
}

impl Default for Preprocessor {
    fn default() -> Self {
        // Create a default lexer with empty input
        let lexer = Lexer::new(String::new());
        Self::new(lexer)
    }
}
