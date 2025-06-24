use crate::error::Error;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(length("hello"), 5);
        assert_eq!(length(""), 0);
        assert_eq!(length("café"), 4); // Unicode
        assert_eq!(length("🦀🚀"), 2); // Emojis
    }

    #[test]
    fn test_is_empty() {
        assert!(is_empty(""));
        assert!(!is_empty("hello"));
        assert!(!is_empty(" ")); // Space is not empty
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(&["hello", " ", "world"]), "hello world");
        assert_eq!(concat(&[]), "");
        assert_eq!(concat(&["single"]), "single");
    }

    #[test]
    fn test_concat_owned() {
        let strings = vec!["hello".to_string(), " ".to_string(), "world".to_string()];
        assert_eq!(concat_owned(strings), "hello world");
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat("abc", 3), "abcabcabc");
        assert_eq!(repeat("hello", 0), "");
        assert_eq!(repeat("", 5), "");
        assert_eq!(repeat("🦀", 3), "🦀🦀🦀");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("café"), "éfac");
        assert_eq!(reverse("🦀🚀"), "🚀🦀");
    }

    #[test]
    fn test_char_at() {
        assert_eq!(char_at("hello", 0), Some('h'));
        assert_eq!(char_at("hello", 4), Some('o'));
        assert_eq!(char_at("hello", 5), None);
        assert_eq!(char_at("café", 3), Some('é'));
        assert_eq!(char_at("🦀🚀", 1), Some('🚀'));
    }

    #[test]
    fn test_chars() {
        assert_eq!(chars("hello"), vec!['h', 'e', 'l', 'l', 'o']);
        assert_eq!(chars(""), Vec::<char>::new());
        assert_eq!(chars("café"), vec!['c', 'a', 'f', 'é']);
    }

    #[test]
    fn test_bytes() {
        assert_eq!(bytes("hello"), b"hello".to_vec());
        assert_eq!(bytes(""), Vec::<u8>::new());
    }

    #[test]
    fn test_is_ascii() {
        assert!(is_ascii("hello"));
        assert!(is_ascii("123"));
        assert!(is_ascii(""));
        assert!(!is_ascii("café"));
        assert!(!is_ascii("🦀"));
    }

    #[test]
    fn test_from_utf8() {
        assert_eq!(from_utf8(b"hello").unwrap(), "hello");
        assert_eq!(from_utf8(b"").unwrap(), "");
        
        // Test invalid UTF-8
        let invalid = &[0xff, 0xfe];
        assert!(from_utf8(invalid).is_err());
    }

    #[test]
    fn test_from_utf8_lossy() {
        assert_eq!(from_utf8_lossy(b"hello"), "hello");
        assert_eq!(from_utf8_lossy(&[0xff, 0xfe]), "��");
    }
}
