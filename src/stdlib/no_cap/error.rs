use crate::error::Error;
/// Error handling for NoCap string conversion module
use std::fmt;

/// Main error type for no_cap operations
#[derive(Debug, Clone, PartialEq)]
pub enum NoCapError {
    /// Syntax error during parsing - invalid format
    Syntax(String),
    /// Range error - value out of valid range
    Range(String),
    /// Invalid input provided
    InvalidInput(String),
    /// Unsupported conversion
    UnsupportedConversion(String),
}

impl fmt::Display for NoCapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoCapError::Syntax(msg) => write!(f, "sus conversion, invalid syntax: {}", msg),
            NoCapError::Range(msg) => write!(f, "too extra, value out of range: {}", msg),
            NoCapError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
            NoCapError::UnsupportedConversion(msg) => write!(f, "unsupported conversion: {}", msg),
        }
    }
}

impl std::error::Error for NoCapError {}

/// Result type for no_cap operations
pub type NoCapResult<T> = std::result::Result<T, NoCapError>;

/// Standard error instances as specified in the API
pub struct ErrorConstants;

impl ErrorConstants {
    /// Syntax error constant
    pub fn err_syntax() -> NoCapError {
        NoCapError::Syntax("sus conversion, invalid syntax".to_string())
    }
    
    /// Range error constant
    pub fn err_range() -> NoCapError {
        NoCapError::Range("too extra, value out of range".to_string())
    }
}

/// Error creation helper functions
pub fn syntax_error(msg: &str) -> NoCapError {
    NoCapError::Syntax(msg.to_string())
}

pub fn range_error(msg: &str) -> NoCapError {
    NoCapError::Range(msg.to_string())
}

pub fn invalid_input_error(msg: &str) -> NoCapError {
    NoCapError::InvalidInput(msg.to_string())
}

pub fn unsupported_conversion_error(msg: &str) -> NoCapError {
    NoCapError::UnsupportedConversion(msg.to_string())
}

/// Public error constants for API compatibility
pub const ErrSyntax: fn() -> NoCapError = ErrorConstants::err_syntax;
pub const ErrRange: fn() -> NoCapError = ErrorConstants::err_range;

/// Module information type (defined in parent module)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let syntax_err = syntax_error("test message");
        assert_eq!(syntax_err.to_string(), "sus conversion, invalid syntax: test message");
        
        let range_err = range_error("value too big");
        assert_eq!(range_err.to_string(), "too extra, value out of range: value too big");
    }

    #[test]
    fn test_error_constants() {
        let err1 = ErrSyntax();
        let err2 = ErrorConstants::err_syntax();
        assert_eq!(err1, err2);

        let err3 = ErrRange();
        let err4 = ErrorConstants::err_range();
        assert_eq!(err3, err4);
    }

    #[test]
    fn test_error_creation_helpers() {
        let syntax = syntax_error("invalid format");
        match syntax {
            NoCapError::Syntax(msg) => assert_eq!(msg, "invalid format"),
            _ => panic!("Expected Syntax error"),
        }

        let range = range_error("overflow");
        match range {
            NoCapError::Range(msg) => assert_eq!(msg, "overflow"),
            _ => panic!("Expected Range error"),
        }
    }
}
