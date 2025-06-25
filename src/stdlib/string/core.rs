use crate::error::CursedError;
/// Core string operations
use super::{StringError, StringResult};

/// Get the length of a string in characters (not bytes)
pub fn length(s: &str) -> usize {
    s.chars().count()
}

/// Check if a string is empty
pub fn is_empty(s: &str) -> bool {
    s.is_empty()
}

/// Concatenate multiple strings into one
pub fn concat(strings: &[&str]) -> String {
    strings.concat()
}

/// Concatenate a vector of owned strings
pub fn concat_owned(strings: Vec<String>) -> String {
    strings.concat()
}

/// Repeat a string N times
pub fn repeat(s: &str, count: usize) -> String {
    s.repeat(count)
}

/// Reverse a string while preserving Unicode grapheme clusters
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Get character at specific index (0-based)
pub fn char_at(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

/// Convert string to character vector
pub fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

/// Convert string to byte vector
pub fn bytes(s: &str) -> Vec<u8> {
    s.bytes().collect()
}

/// Check if string contains only ASCII characters
pub fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}

/// Convert from bytes to string safely
pub fn from_utf8(bytes: &[u8]) -> StringResult<String> {
    match std::str::from_utf8(bytes) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(StringError::InvalidUtf8 { 
            position: e.valid_up_to() 
        })
    }
}

/// Convert from bytes to string lossy (replaces invalid UTF-8 with replacement chars)
pub fn from_utf8_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

