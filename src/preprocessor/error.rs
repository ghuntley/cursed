/// Preprocessor Error Types
/// 
/// This module defines error types specific to the generic syntax preprocessor,
/// providing detailed error messages for malformed generic syntax.

use crate::error::{Error, SourceLocation};

use std::fmt;

/// Result type for preprocessor operations
pub type PreprocessorResult<T> = std::result::Result<T, PreprocessorError>;

/// Preprocessor-specific error types
#[derive(Debug, Clone, PartialEq)]
pub enum PreprocessorError {
    /// Unclosed type parameter brackets
    UnclosedTypeParameters {
        location: SourceLocation,
        message: String,
    },
    /// Unexpected token in generic declaration
    UnexpectedToken {
        location: SourceLocation,
        expected: String,
        found: String,
    },
    /// Missing required token after type parameters
    MissingRequiredToken {
        location: SourceLocation,
        expected: String,
        context: String,
    },
    /// Invalid generic syntax pattern
    InvalidGenericSyntax {
        location: SourceLocation,
        pattern: String,
        reason: String,
    },
    /// Nested generic type too deep
    NestedGenericTooDeep {
        location: SourceLocation,
        depth: usize,
        max_depth: usize,
    },
    /// Lexer error during preprocessing
    LexerError {
        location: SourceLocation,
        message: String,
    },
    /// General preprocessing error
    General {
        location: SourceLocation,
        message: String,
    },
}

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
            Self::UnclosedTypeParameters { location, .. } => location,
            Self::UnexpectedToken { location, .. } => location,
            Self::MissingRequiredToken { location, .. } => location,
            Self::InvalidGenericSyntax { location, .. } => location,
            Self::NestedGenericTooDeep { location, .. } => location,
            Self::LexerError { location, .. } => location,
            Self::General { location, .. } => location,
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
            Self::General { message, .. } => message.clone(),
        }
    }
}

impl fmt::Display for PreprocessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.detailed_message(), self.location())
    }
}

impl std::error::Error for PreprocessorError {}

/// Convert PreprocessorError to the main Error type
impl From<PreprocessorError> for Error {
    fn from(err: PreprocessorError) -> Self {
        match err {
            PreprocessorError::LexerError { message, location } => {
                Error::Lexical { message, location }
            }
            _ => Error::Parse(err.to_string()),
        }
    }
}

/// Convert main Error to PreprocessorError
impl From<Error> for PreprocessorError {
    fn from(err: Error) -> Self {
        match err {
            Error::Lexical { message, location } => {
                PreprocessorError::lexer_error(location, message)
            }
            Error::Parse(message) => {
                PreprocessorError::general(
                    SourceLocation::new(0, 0, 0, None),
                    message
                )
            }
            _ => {
                PreprocessorError::general(
                    SourceLocation::new(0, 0, 0, None),
                    err.to_string()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocessor_error_creation() {
        let location = SourceLocation::new(1, 10, 15, Some("test.csd".to_string()));
        let error = PreprocessorError::unclosed_type_parameters(
            location.clone(),
            "Missing closing bracket".to_string()
        );
        
        assert_eq!(error.location(), &location);
        assert!(error.detailed_message().contains("Unclosed type parameter brackets"));
    }

    #[test]
    fn test_error_conversion() {
        let location = SourceLocation::new(1, 5, 10, Some("test.csd".to_string()));
        let preprocessor_error = PreprocessorError::unexpected_token(
            location,
            "T".to_string(),
            "normie".to_string()
        );
        
        let main_error: Error = preprocessor_error.into();
        assert!(main_error.to_string().contains("Expected 'T'"));
    }
}
