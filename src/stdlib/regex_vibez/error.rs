/// Error handling for RegexVibez module
use crate::error::CursedError;
use std::fmt;

/// Result type for RegexVibez operations
pub type RegexVibesResult<T> = Result<T, RegexVibesError>;

/// Comprehensive error types for regex operations
#[derive(Debug, Clone)]
pub enum RegexVibesError {
    /// Invalid regex pattern compilation error
    CompilationError(String),
    /// Invalid input data error
    InvalidInput(String),
    /// Replacement template error
    TemplateError(String),
    /// Index out of bounds error
    IndexError(String),
    /// IO error during operations
    IoError(String),
    /// UTF-8 encoding error
    EncodingError(String),
    /// General regex operation error
    GeneralError(String),
}

impl fmt::Display for RegexVibesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegexVibesError::CompilationError(msg) => write!(f, "Regex compilation error: {}", msg),
            RegexVibesError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            RegexVibesError::TemplateError(msg) => write!(f, "Template error: {}", msg),
            RegexVibesError::IndexError(msg) => write!(f, "Index error: {}", msg),
            RegexVibesError::IoError(msg) => write!(f, "IO error: {}", msg),
            RegexVibesError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            RegexVibesError::GeneralError(msg) => write!(f, "Regex error: {}", msg),
        }
    }
}

impl std::error::Error for RegexVibesError {}

impl From<regex::Error> for RegexVibesError {
    fn from(err: regex::Error) -> Self {
        RegexVibesError::CompilationError(err.to_string())
    }
}

impl From<std::io::Error> for RegexVibesError {
    fn from(err: std::io::Error) -> Self {
        RegexVibesError::IoError(err.to_string())
    }
}

impl From<std::str::Utf8Error> for RegexVibesError {
    fn from(err: std::str::Utf8Error) -> Self {
        RegexVibesError::EncodingError(err.to_string())
    }
}

impl From<RegexVibesError> for CursedError {
    fn from(err: RegexVibesError) -> Self {
        CursedError::new(err.to_string())
    }
}

/// Create a compilation error
pub fn compilation_error(msg: &str) -> RegexVibesError {
    RegexVibesError::CompilationError(msg.to_string())
}

/// Create an invalid input error
pub fn invalid_input_error(msg: &str) -> RegexVibesError {
    RegexVibesError::InvalidInput(msg.to_string())
}

/// Create a template error
pub fn template_error(msg: &str) -> RegexVibesError {
    RegexVibesError::TemplateError(msg.to_string())
}

/// Create an index error
pub fn index_error(msg: &str) -> RegexVibesError {
    RegexVibesError::IndexError(msg.to_string())
}

/// Create an encoding error
pub fn encoding_error(msg: &str) -> RegexVibesError {
    RegexVibesError::EncodingError(msg.to_string())
}

/// Create a general error
pub fn general_error(msg: &str) -> RegexVibesError {
    RegexVibesError::GeneralError(msg.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = compilation_error("invalid pattern");
        assert_eq!(err.to_string(), "Regex compilation error: invalid pattern");

        let err = invalid_input_error("bad input");
        assert_eq!(err.to_string(), "Invalid input: bad input");
    }

    #[test]
    fn test_error_conversion() {
        let regex_err = regex::Regex::new("[invalid").unwrap_err();
        let vibe_err = RegexVibesError::from(regex_err);
        assert!(matches!(vibe_err, RegexVibesError::CompilationError(_)));

        let cursed_err = CursedError::from(vibe_err);
        assert!(cursed_err.message().contains("compilation error"));
    }
}
