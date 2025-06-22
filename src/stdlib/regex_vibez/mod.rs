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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compilation() {
        let pattern = compile(r"[a-z]+").expect("Should compile basic pattern");
        assert!(pattern.match_string("hello"));
        assert!(!pattern.match_string("HELLO"));
    }

    #[test]
    fn test_must_compile() {
        let pattern = must_compile(r"\d+");
        assert!(pattern.match_string("123"));
        assert!(!pattern.match_string("abc"));
    }

    #[test]
    #[should_panic(expected = "RegexVibez MustCompile failed")]
    fn test_must_compile_panic() {
        must_compile("[invalid");
    }

    #[test]
    fn test_match_string() {
        let (matched, err) = match_string(r"f[a-z]+", "frfr");
        assert!(matched);
        assert!(err.is_none());

        let (matched, err) = match_string("[invalid", "test");
        assert!(!matched);
        assert!(err.is_some());
    }

    #[test]
    fn test_quote_meta() {
        assert_eq!(quote_meta("a.b*c+d?e"), r"a\.b\*c\+d\?e");
        assert_eq!(quote_meta("normal"), "normal");
    }
}


#[derive(Debug, Clone)]
pub struct GroupStatistics {
    pub total_groups: usize,
    pub named_groups: usize,
    pub unnamed_groups: usize,
    pub nested_groups: usize,
}

#[derive(Debug, Clone)]
pub struct GroupValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub statistics: GroupStatistics,
}
