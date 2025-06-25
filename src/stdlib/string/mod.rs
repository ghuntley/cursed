use crate::error::CursedError;
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
pub type StringResult<T> = std::result::Result<T, StringError>;

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

// impl std::fmt::Display for StringError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             StringError::IndexOutOfBounds { index, length } => {
//                 write!(f, "Index {} out of bounds for string of length {}", index, length)
//             }
//             StringError::InvalidRange { start, end, length } => {
//                 write!(f, "Invalid range {}..{} for string of length {}", start, end, length)
//             }
//             StringError::InvalidUtf8 { position } => {
//                 write!(f, "Invalid UTF-8 sequence at position {}", position)
//             }
//             StringError::EmptyInput => write!(f, "Empty input string"),
//             StringError::InvalidParameter { param, value } => {
//                 write!(f, "Invalid parameter '{}': {}", param, value)
//             }
//             StringError::RegexError { message, pattern } => {
//                 write!(f, "Regex error in pattern '{}': {}", pattern, message)
//             }
//         }
//     }
// }

// impl std::error::CursedError for StringError {}
// 
