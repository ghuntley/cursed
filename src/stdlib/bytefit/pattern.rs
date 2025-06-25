use crate::error::CursedError;
/// Pattern matching functions for byte slices
use super::{ByteFitError, ByteFitResult, invalid_pattern, regex_error};

/// WildcardMatch performs wildcard pattern matching on byte data
/// Supports '*' for any sequence of bytes and '?' for any single byte
pub fn wildcard_match(pattern: &[u8], data: &[u8]) -> bool {
    wildcard_match_impl(pattern, data, 0, 0)
/// RegexMatch performs regular expression matching on byte data
pub fn regex_match(pattern: &str, data: &[u8]) -> ByteFitResult<bool> {
    match std::str::from_utf8(data) {
        Ok(text) => {
            // Simple regex implementation for demonstration
            // In production, would use the regex crate
            simple_regex_match(pattern, text)
        }
    }
}

/// RegexFindAll finds all matches of a regular expression pattern in byte data
pub fn regex_find_all(pattern: &str, data: &[u8], n: i32) -> ByteFitResult<Vec<Vec<u8>>> {
    match std::str::from_utf8(data) {
        Ok(text) => {
            let matches = simple_regex_find_all(pattern, text, n)?;
            Ok(matches.into_iter().map(|s| s.into_bytes()).collect())
        }
    }
}

/// RegexReplace replaces matches of a regular expression pattern in byte data
pub fn regex_replace(pattern: &str, data: &[u8], repl: &[u8]) -> ByteFitResult<Vec<u8>> {
    match (std::str::from_utf8(data), std::str::from_utf8(repl)) {
        (Ok(text), Ok(replacement)) => {
            let result = simple_regex_replace(pattern, text, replacement)?;
            Ok(result.into_bytes())
        }
    }
}

/// Recursive implementation of wildcard matching
fn wildcard_match_impl(pattern: &[u8], data: &[u8], p_idx: usize, d_idx: usize) -> bool {
    // Base cases
    if p_idx == pattern.len() {
        return d_idx == data.len();
    if d_idx == data.len() {
        // Check if remaining pattern is all '*'
        return pattern[p_idx..].iter().all(|&c| c == b'*');
    match pattern[p_idx] {
        b'*' => {
            // Try matching zero or more characters
            // First try zero characters (move pattern index)
            if wildcard_match_impl(pattern, data, p_idx + 1, d_idx) {
                return true;
            }
            // Then try one or more characters (move data index)
            wildcard_match_impl(pattern, data, p_idx, d_idx + 1)
        }
        b'?' => {
            // Match exactly one character
            wildcard_match_impl(pattern, data, p_idx + 1, d_idx + 1)
        }
        c => {
            // Match literal character
            if data[d_idx] == c {
                wildcard_match_impl(pattern, data, p_idx + 1, d_idx + 1)
            } else {
                false
            }
        }
    }
}

/// Simple regex matching implementation (for demonstration)
/// In production, would use the regex crate
fn simple_regex_match(pattern: &str, text: &str) -> ByteFitResult<bool> {
    // Handle some basic regex patterns
    match pattern {
        _ => {
            // Convert some common patterns
            if pattern.starts_with('^') && pattern.ends_with('$') {
                let inner = &pattern[1..pattern.len()-1];
                return Ok(text == inner);
            if pattern.starts_with('^') {
                let prefix = &pattern[1..];
                return Ok(text.starts_with(prefix));
            if pattern.ends_with('$') {
                let suffix = &pattern[..pattern.len()-1];
                return Ok(text.ends_with(suffix));
            // Handle digit pattern
            if pattern == r"\d+" {
                return Ok(text.chars().all(|c| c.is_ascii_digit()) && !text.is_empty());
            // Handle word pattern
            if pattern == r"\w+" {
                return Ok(text.chars().all(|c| c.is_alphanumeric()) && !text.is_empty());
            // Handle whitespace pattern
            if pattern == r"\s+" {
                return Ok(text.chars().all(|c| c.is_whitespace()) && !text.is_empty());
            // Simple literal match
            Ok(text.contains(pattern))
        }
    }
/// Simple regex find all implementation
fn simple_regex_find_all(pattern: &str, text: &str, n: i32) -> ByteFitResult<Vec<String>> {
    let mut matches = Vec::new();
    let limit = if n < 0 { usize::MAX } else { n as usize };
    
    match pattern {
        r"\d+" => {
            let mut current = String::new();
            for c in text.chars() {
                if c.is_ascii_digit() {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        matches.push(current.clone());
                        current.clear();
                        if matches.len() >= limit {
                            break;
                        }
                    }
                }
            }
            if !current.is_empty() && matches.len() < limit {
                matches.push(current);
            }
        }
        r"\w+" => {
            let mut current = String::new();
            for c in text.chars() {
                if c.is_alphanumeric() {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        matches.push(current.clone());
                        current.clear();
                        if matches.len() >= limit {
                            break;
                        }
                    }
                }
            }
            if !current.is_empty() && matches.len() < limit {
                matches.push(current);
            }
        }
        _ => {
            // Simple literal search
            let mut start = 0;
            while let Some(pos) = text[start..].find(pattern) {
                matches.push(pattern.to_string());
                start += pos + pattern.len();
                if matches.len() >= limit {
                    break;
                }
            }
        }
    }
    
    Ok(matches)
/// Simple regex replace implementation
fn simple_regex_replace(pattern: &str, text: &str, replacement: &str) -> ByteFitResult<String> {
    match pattern {
        r"\d+" => {
            let mut result = String::new();
            let mut current = String::new();
            
            for c in text.chars() {
                if c.is_ascii_digit() {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        result.push_str(replacement);
                        current.clear();
                    }
                    result.push(c);
                }
            }
            
            if !current.is_empty() {
                result.push_str(replacement);
            Ok(result)
        }
        r"\w+" => {
            let mut result = String::new();
            let mut current = String::new();
            
            for c in text.chars() {
                if c.is_alphanumeric() {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        result.push_str(replacement);
                        current.clear();
                    }
                    result.push(c);
                }
            }
            
            if !current.is_empty() {
                result.push_str(replacement);
            Ok(result)
        }
        _ => {
            // Simple literal replacement
            Ok(text.replace(pattern, replacement))
        }
    }
