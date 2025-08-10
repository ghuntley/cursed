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
    match_with_regex, capture_groups, extract_patterns, RegexPattern, RegexMatch
};

/// CURSED string type with enhanced functionality
#[derive(Debug, Clone)]
pub struct CursedString {
    inner: String,
}

impl CursedString {
    /// Create new CursedString from regular string
    pub fn new(s: String) -> Self {
        Self { inner: s }
    }

    /// Create new CursedString from string slice
    pub fn from_str(s: &str) -> Self {
        Self { inner: s.to_string() }
    }

    /// Get inner string
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Convert to regular String
    pub fn into_string(self) -> String {
        self.inner
    }

    /// Get length of string
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if string is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Default for CursedString {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl From<String> for CursedString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for CursedString {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl std::fmt::Display for CursedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

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
