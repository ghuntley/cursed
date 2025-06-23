/// Comprehensive string manipulation utilities for CURSED
/// 
/// This module provides a complete set of string operations including:
/// - Core operations (length, concatenation, reversal)
/// - Search and replace functionality
/// - String transformations (case conversion, trimming)
/// - Splitting and joining utilities
/// - Character-level operations
/// - String validation and formatting

pub mod core;
pub mod search;
pub mod transform;
pub mod split_join;
pub mod validation;
pub mod format;
pub mod regex;

// Re-export all public functions for easy access
pub use core::*;
pub use search::*;
pub use transform::*;
pub use split_join::*;
pub use validation::*;
pub use format::*;
// Use explicit imports from regex to avoid conflicts
pub use regex::{
    find_with_regex, replace_with_regex, replace_all_with_regex, split_with_regex,
    match_with_regex, capture_groups, extract_patterns, RegexPattern, RegexMatch
};

// String manipulation result type
pub type StringResult<(), Error>;

/// Errors that can occur during string operations
#[derive(Debug, Clone, PartialEq)]
pub enum StringError {
    IndexOutOfBounds { index: usize, length: usize },
    InvalidRange { start: usize, end: usize, length: usize },
    InvalidUtf8 { position: usize },
    EmptyInput,
    InvalidParameter { param: String, value: String },
    RegexError { message: String, pattern: String },
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringError::IndexOutOfBounds { index, length } => {
                write!(f, "Index {} out of bounds for string of length {}", index, length)
            }
            StringError::InvalidRange { start, end, length } => {
                write!(f, "Invalid range {}..{} for string of length {}", start, end, length)
            }
            StringError::InvalidUtf8 { position } => {
                write!(f, "Invalid UTF-8 sequence at position {}", position)
            }
            StringError::EmptyInput => write!(f, "Empty input string"),
            StringError::InvalidParameter { param, value } => {
                write!(f, "Invalid parameter '{}': {}", param, value)
            }
            StringError::RegexError { message, pattern } => {
                write!(f, "Regex error in pattern '{}': {}", pattern, message)
            }
        }
    }
}

impl std::error::Error for StringError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_error_display() {
        let error = StringError::IndexOutOfBounds { index: 5, length: 3 };
        assert_eq!(error.to_string(), "Index 5 out of bounds for string of length 3");

        let error = StringError::InvalidRange { start: 2, end: 1, length: 5 };
        assert_eq!(error.to_string(), "Invalid range 2..1 for string of length 5");

        let error = StringError::InvalidUtf8 { position: 10 };
        assert_eq!(error.to_string(), "Invalid UTF-8 sequence at position 10");

        let error = StringError::EmptyInput;
        assert_eq!(error.to_string(), "Empty input string");

        let error = StringError::InvalidParameter { 
            param: "width".to_string(), 
            value: "negative".to_string() 
        };
        assert_eq!(error.to_string(), "Invalid parameter 'width': negative");
    }
}
