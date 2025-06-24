use crate::error::Error;
/// Pattern matching functions for byte slices
use super::{ByteFitError, ByteFitResult, invalid_pattern, regex_error};

/// WildcardMatch performs wildcard pattern matching on byte data
/// Supports '*' for any sequence of bytes and '?' for any single byte
pub fn wildcard_match(pattern: &[u8], data: &[u8]) -> bool {
    wildcard_match_impl(pattern, data, 0, 0)
}

/// RegexMatch performs regular expression matching on byte data
pub fn regex_match(pattern: &str, data: &[u8]) -> ByteFitResult<bool> {
    match std::str::from_utf8(data) {
        Ok(text) => {
            // Simple regex implementation for demonstration
            // In production, would use the regex crate
            simple_regex_match(pattern, text)
        }
        Err(_) => Err(regex_error("Data contains invalid UTF-8")),
    }
}

/// RegexFindAll finds all matches of a regular expression pattern in byte data
pub fn regex_find_all(pattern: &str, data: &[u8], n: i32) -> ByteFitResult<Vec<Vec<u8>>> {
    match std::str::from_utf8(data) {
        Ok(text) => {
            let matches = simple_regex_find_all(pattern, text, n)?;
            Ok(matches.into_iter().map(|s| s.into_bytes()).collect())
        }
        Err(_) => Err(regex_error("Data contains invalid UTF-8")),
    }
}

/// RegexReplace replaces matches of a regular expression pattern in byte data
pub fn regex_replace(pattern: &str, data: &[u8], repl: &[u8]) -> ByteFitResult<Vec<u8>> {
    match (std::str::from_utf8(data), std::str::from_utf8(repl)) {
        (Ok(text), Ok(replacement)) => {
            let result = simple_regex_replace(pattern, text, replacement)?;
            Ok(result.into_bytes())
        }
        _ => Err(regex_error("Data or replacement contains invalid UTF-8")),
    }
}

/// Recursive implementation of wildcard matching
fn wildcard_match_impl(pattern: &[u8], data: &[u8], p_idx: usize, d_idx: usize) -> bool {
    // Base cases
    if p_idx == pattern.len() {
        return d_idx == data.len();
    }
    
    if d_idx == data.len() {
        // Check if remaining pattern is all '*'
        return pattern[p_idx..].iter().all(|&c| c == b'*');
    }
    
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
        ".*" => Ok(true),
        "" => Ok(text.is_empty()),
        _ => {
            // Convert some common patterns
            if pattern.starts_with('^') && pattern.ends_with('$') {
                let inner = &pattern[1..pattern.len()-1];
                return Ok(text == inner);
            }
            
            if pattern.starts_with('^') {
                let prefix = &pattern[1..];
                return Ok(text.starts_with(prefix));
            }
            
            if pattern.ends_with('$') {
                let suffix = &pattern[..pattern.len()-1];
                return Ok(text.ends_with(suffix));
            }
            
            // Handle digit pattern
            if pattern == r"\d+" {
                return Ok(text.chars().all(|c| c.is_ascii_digit()) && !text.is_empty());
            }
            
            // Handle word pattern
            if pattern == r"\w+" {
                return Ok(text.chars().all(|c| c.is_alphanumeric()) && !text.is_empty());
            }
            
            // Handle whitespace pattern
            if pattern == r"\s+" {
                return Ok(text.chars().all(|c| c.is_whitespace()) && !text.is_empty());
            }
            
            // Simple literal match
            Ok(text.contains(pattern))
        }
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
}

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
            }
            
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
            }
            
            Ok(result)
        }
        _ => {
            // Simple literal replacement
            Ok(text.replace(pattern, replacement))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wildcard_match() {
        assert!(wildcard_match(b"hello", b"hello"));
        assert!(wildcard_match(b"h*o", b"hello"));
        assert!(wildcard_match(b"h*", b"hello"));
        assert!(wildcard_match(b"*o", b"hello"));
        assert!(wildcard_match(b"*", b"hello"));
        assert!(wildcard_match(b"h?llo", b"hello"));
        assert!(!wildcard_match(b"h?llo", b"hllo"));
        assert!(!wildcard_match(b"hello", b"world"));
    }

    #[test]
    fn test_wildcard_edge_cases() {
        assert!(wildcard_match(b"", b""));
        assert!(wildcard_match(b"*", b""));
        assert!(wildcard_match(b"**", b""));
        assert!(!wildcard_match(b"a", b""));
        assert!(!wildcard_match(b"", b"a"));
        assert!(wildcard_match(b"*a*", b"abc"));
        assert!(wildcard_match(b"a*b*c", b"aXbYc"));
    }

    #[test]
    fn test_regex_match() {
        assert!(regex_match("hello", b"hello world").unwrap());
        assert!(!regex_match("foo", b"hello world").unwrap());
        assert!(regex_match("^hello", b"hello world").unwrap());
        assert!(!regex_match("^world", b"hello world").unwrap());
        assert!(regex_match("world$", b"hello world").unwrap());
        assert!(!regex_match("hello$", b"hello world").unwrap());
        assert!(regex_match(r"\d+", b"123").unwrap());
        assert!(!regex_match(r"\d+", b"abc").unwrap());
    }

    #[test]
    fn test_regex_find_all() {
        let result = regex_find_all(r"\d+", b"abc123def456ghi", -1).unwrap();
        assert_eq!(result, vec![b"123".to_vec(), b"456".to_vec()]);
        
        let result = regex_find_all(r"\d+", b"abc123def456ghi", 1).unwrap();
        assert_eq!(result, vec![b"123".to_vec()]);
        
        let result = regex_find_all("hello", b"hello world hello", -1).unwrap();
        assert_eq!(result, vec![b"hello".to_vec(), b"hello".to_vec()]);
    }

    #[test]
    fn test_regex_replace() {
        let result = regex_replace(r"\d+", b"abc123def456", b"XXX").unwrap();
        assert_eq!(result, b"abcXXXdefXXX");
        
        let result = regex_replace("hello", b"hello world hello", b"hi").unwrap();
        assert_eq!(result, b"hi world hi");
        
        let result = regex_replace(r"\w+", b"hello world", b"X").unwrap();
        assert_eq!(result, b"X X");
    }

    #[test]
    fn test_complex_wildcard_patterns() {
        assert!(wildcard_match(b"a*b*c", b"aXXbYYc"));
        assert!(wildcard_match(b"a?b?c", b"axbyc"));
        assert!(!wildcard_match(b"a?b?c", b"axxbyc"));
        assert!(wildcard_match(b"*.txt", b"file.txt"));
        assert!(!wildcard_match(b"*.txt", b"file.doc"));
    }
}
