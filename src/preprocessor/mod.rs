/// Generic Syntax Preprocessor Module
/// 
/// This module provides preprocessing capabilities for handling generic syntax
/// in the CURSED programming language, sitting between the lexer and parser
/// to enhance tokens with contextual information.

pub mod core;
pub mod token_stream;
pub mod error;

// Re-export core types for public API
pub use core::Preprocessor;
pub use token_stream::{TokenStream, TokenWithContext, TokenMetadata};
pub use error::{PreprocessorError, PreprocessorResult};

use crate::lexer::Lexer;
use crate::error::SourceLocation;
use crate::error::CursedError;

/// Initialize a preprocessor with the given lexer
pub fn new_preprocessor(lexer: Lexer) -> Preprocessor {
    Preprocessor::new(lexer)
}

/// Process source code through the preprocessor pipeline
pub fn process_source(source: &str) -> PreprocessorResult<TokenStream> {
    let lexer = Lexer::new(source.to_string());
    let mut preprocessor = Preprocessor::new(lexer);
    preprocessor.process()
}

