//! Preprocessor error types

use crate::error::CursedError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum PreprocessorError {
    MacroExpansionError(String),
    IncludeError(String),
    DirectiveError(String),
    TokenizationError(String),
    LexError(String),
}

pub type PreprocessorResult<T> = Result<T, PreprocessorError>;

impl fmt::Display for PreprocessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PreprocessorError::MacroExpansionError(msg) => write!(f, "Macro expansion error: {}", msg),
            PreprocessorError::IncludeError(msg) => write!(f, "Include error: {}", msg),
            PreprocessorError::DirectiveError(msg) => write!(f, "Directive error: {}", msg),
            PreprocessorError::TokenizationError(msg) => write!(f, "Tokenization error: {}", msg),
            PreprocessorError::LexError(msg) => write!(f, "Lex error: {}", msg),
        }
    }
}

impl std::error::Error for PreprocessorError {}

impl From<PreprocessorError> for CursedError {
    fn from(err: PreprocessorError) -> Self {
        CursedError::runtime_error(&err.to_string())
    }
}
