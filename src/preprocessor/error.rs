/// Preprocessor CursedError Types
/// 
/// This module defines error types specific to the generic syntax preprocessor,
/// providing detailed error messages for malformed generic syntax.

use crate::error::{CursedError, SourceLocation};

use std::fmt;

/// Result type for preprocessor operations
pub type PreprocessorResult<T> = std::result::Result<T, PreprocessorError>;

/// Preprocessor-specific error types
#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessorError {
    /// Unclosed type parameter brackets
    UnclosedTypeParameters {
    /// Unexpected token in generic declaration
    UnexpectedToken {
    /// Missing required token after type parameters
    MissingRequiredToken {
    /// Invalid generic syntax pattern
    InvalidGenericSyntax {
    /// Nested generic type too deep
    NestedGenericTooDeep {
    /// Lexer error during preprocessing
    LexerError {
    /// General preprocessing error
    General {
impl PreprocessorError {
    /// Create an unclosed type parameters error
    pub fn unclosed_type_parameters(location: SourceLocation, message: String) -> Self {
        Self::UnclosedTypeParameters { location, message }
    }

    /// Create an unexpected token error
    pub fn unexpected_token(location: SourceLocation, expected: String, found: String) -> Self {
        Self::UnexpectedToken { location, expected, found }
    }

    /// Create a missing required token error
    pub fn missing_required_token(location: SourceLocation, expected: String, context: String) -> Self {
        Self::MissingRequiredToken { location, expected, context }
    }

    /// Create an invalid generic syntax error
    pub fn invalid_generic_syntax(location: SourceLocation, pattern: String, reason: String) -> Self {
        Self::InvalidGenericSyntax { location, pattern, reason }
    }

    /// Create a nested generic too deep error
    pub fn nested_generic_too_deep(location: SourceLocation, depth: usize, max_depth: usize) -> Self {
        Self::NestedGenericTooDeep { location, depth, max_depth }
    }

    /// Create a lexer error
    pub fn lexer_error(location: SourceLocation, message: String) -> Self {
        Self::LexerError { location, message }
    }

    /// Create a general preprocessor error
    pub fn general(location: SourceLocation, message: String) -> Self {
        Self::General { location, message }
    }

    /// Get the source location of the error
    pub fn location(&self) -> &SourceLocation {
        match self {
        }
    }

    /// Get a detailed error message
    pub fn detailed_message(&self) -> String {
        match self {
            Self::UnclosedTypeParameters { message, .. } => {
                format!("Unclosed type parameter brackets: {}", message)
            }
            Self::UnexpectedToken { expected, found, .. } => {
                format!("Expected '{}', but found '{}'", expected, found)
            }
            Self::MissingRequiredToken { expected, context, .. } => {
                format!("Missing required token '{}' after {}", expected, context)
            }
            Self::InvalidGenericSyntax { pattern, reason, .. } => {
                format!("Invalid generic syntax pattern '{}': {}", pattern, reason)
            }
            Self::NestedGenericTooDeep { depth, max_depth, .. } => {
                format!("Nested generic type too deep: {} levels (max: {})", depth, max_depth)
            }
            Self::LexerError { message, .. } => {
                format!("Lexer error during preprocessing: {}", message)
            }
        }
    }
// impl fmt::Display for PreprocessorError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} at {}", self.detailed_message(), self.location())
//     }
// }

// impl std::error::CursedError for PreprocessorError {}
// 
// Convert PreprocessorError to the main CursedError type
// impl From<PreprocessorError> for CursedError {
//     fn from(err: PreprocessorError) -> Self {
//         match err {
//             PreprocessorError::LexerError { message, location } => {
//                 CursedError::Lexical { message, location }
//             }
//             _ => CursedError::Parse(err.to_string()),
//         }
//     }
// }

// Convert main CursedError to PreprocessorError
// impl From<CursedError> for PreprocessorError {
//     fn from(err: CursedError) -> Self {
//         match err {
//             CursedError::Lexical { message, location } => {
//                 PreprocessorError::lexer_error(location, message)
//             }
//             CursedError::Parse(message) => {
//                 PreprocessorError::general(
//                     SourceLocation::new(0, 0, 0, None),
//                     message
//                 )
//             }
//             _ => {
//                 PreprocessorError::general(
//                     SourceLocation::new(0, 0, 0, None),
//                     err.to_string()
//                 )
//             }
//         }
//     }
// }

