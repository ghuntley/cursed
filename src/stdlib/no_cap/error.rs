use crate::error::CursedError;
/// CursedError handling for NoCap string conversion module
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

// impl fmt::Display for NoCapError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             NoCapError::Syntax(msg) => write!(f, "sus conversion, invalid syntax: {}", msg),
//             NoCapError::Range(msg) => write!(f, "too extra, value out of range: {}", msg),
//             NoCapError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
//             NoCapError::UnsupportedConversion(msg) => write!(f, "unsupported conversion: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for NoCapError {}
// 
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

/// CursedError creation helper functions
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

