/// RegexVibez - Regular expression package for CURSED
/// 
/// This module provides comprehensive regular expression functionality with a vibez-focused interface.
/// Built on top of the regex crate for performance and reliability.

pub mod error;
pub mod pattern;
pub mod groups;
pub mod builder;
pub mod common;
pub mod utils;

// Re-export core types and functions
pub use error::{RegexVibesError, RegexVibesResult};
pub use pattern::VibePattern;
pub use groups::VibeGroups;
pub use builder::PatternBuilder;
pub use common::*;
pub use utils::*;

// Core compilation functions
use crate::error::CursedError;

/// Compile a regular expression pattern into a VibePattern
/// Returns error if the pattern is invalid
pub fn compile(expr: &str) -> RegexVibesResult<VibePattern> {
    VibePattern::compile(expr)
}

/// MustCompile compiles a pattern and panics on error - use with caution!
/// Shooks on tea (panics on error) - only use with patterns you know are valid
pub fn must_compile(expr: &str) -> VibePattern {
    match compile(expr) {
        Ok(pattern) => pattern,
        Err(e) => panic!("RegexVibez MustCompile failed: {}", e),
    }
}

/// Compile a POSIX regular expression pattern
pub fn compile_posix(expr: &str) -> RegexVibesResult<VibePattern> {
    VibePattern::compile_posix(expr)
}

/// MustCompilePOSIX compiles a POSIX pattern and panics on error
/// Shooks on tea (panics on error) - only use with patterns you know are valid
pub fn must_compile_posix(expr: &str) -> VibePattern {
    match compile_posix(expr) {
        Ok(pattern) => pattern,
        Err(e) => panic!("RegexVibez MustCompilePOSIX failed: {}", e),
    }
}

/// Helper function to check if a byte slice matches a pattern
pub fn r#match(pattern: &str, b: &[u8]) -> (bool, Option<CursedError>) {
    match std::str::from_utf8(b) {
        Ok(s) => match_string(pattern, s),
        Err(_) => (false, Some(CursedError::new("Invalid UTF-8 in input".to_string()))),
    }
}

/// Helper function to check if a string matches a pattern
pub fn match_string(pattern: &str, s: &str) -> (bool, Option<CursedError>) {
    match compile(pattern) {
        Ok(p) => (p.match_string(s), None),
        Err(e) => (false, Some(e.into())),
    }
}

/// Quote meta characters in a string to make them literal
pub fn quote_meta(s: &str) -> String {
    regex::escape(s)
}

/// Create a new pattern builder for fluent pattern construction
pub fn new_pattern_builder() -> PatternBuilder {
    PatternBuilder::new()
}


#[derive(Debug, Clone)]
pub struct GroupValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub statistics: GroupStatistics,
}
