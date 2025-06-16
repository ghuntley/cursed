/// Basic byte slice operations
use super::{ByteFitResult, invalid_utf8};
use std::cmp::Ordering;

/// Compare returns an integer comparing two byte slices lexicographically.
/// The result will be 0 if a==b, -1 if a < b, and +1 if a > b.
pub fn compare(a: &[u8], b: &[u8]) -> i32 {
    match a.cmp(b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Equal reports whether a and b are the same length and contain the same bytes.
pub fn equal(a: &[u8], b: &[u8]) -> bool {
    a == b
}

/// EqualFold reports whether a and b are equal under Unicode case-folding.
pub fn equal_fold(a: &[u8], b: &[u8]) -> bool {
    // Convert to strings for case-insensitive comparison
    if let (Ok(a_str), Ok(b_str)) = (std::str::from_utf8(a), std::str::from_utf8(b)) {
        a_str.to_lowercase() == b_str.to_lowercase()
    } else {
        // Fall back to byte comparison if not valid UTF-8
        equal(a, b)
    }
}

/// Repeat returns a new byte slice consisting of count copies of b.
pub fn repeat(b: &[u8], count: usize) -> Vec<u8> {
    if count == 0 || b.is_empty() {
        return Vec::new();
    }
    
    let mut result = Vec::with_capacity(b.len() * count);
    for _ in 0..count {
        result.extend_from_slice(b);
    }
    result
}

/// Runes converts a slice of bytes to a slice of runes (Unicode code points).
pub fn runes(s: &[u8]) -> ByteFitResult<Vec<char>> {
    match std::str::from_utf8(s) {
        Ok(string) => Ok(string.chars().collect()),
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        assert_eq!(compare(b"abc", b"abc"), 0);
        assert_eq!(compare(b"abc", b"def"), -1);
        assert_eq!(compare(b"def", b"abc"), 1);
        assert_eq!(compare(b"", b""), 0);
        assert_eq!(compare(b"a", b""), 1);
        assert_eq!(compare(b"", b"a"), -1);
    }

    #[test]
    fn test_equal() {
        assert!(equal(b"hello", b"hello"));
        assert!(!equal(b"hello", b"world"));
        assert!(!equal(b"hello", b"hello!"));
        assert!(equal(b"", b""));
    }

    #[test]
    fn test_equal_fold() {
        assert!(equal_fold(b"Hello", b"HELLO"));
        assert!(equal_fold(b"World", b"world"));
        assert!(!equal_fold(b"hello", b"world"));
        assert!(equal_fold(b"", b""));
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat(b"abc", 3), b"abcabcabc");
        assert_eq!(repeat(b"x", 5), b"xxxxx");
        assert_eq!(repeat(b"hello", 0), b"");
        assert_eq!(repeat(b"", 10), b"");
    }

    #[test]
    fn test_runes() {
        assert_eq!(runes(b"hello").unwrap(), vec!['h', 'e', 'l', 'l', 'o']);
        assert_eq!(runes(b"").unwrap(), vec![]);
        assert_eq!(runes("🦀".as_bytes()).unwrap(), vec!['🦀']);
        
        // Test invalid UTF-8
        let invalid_utf8 = vec![0xFF, 0xFE];
        assert!(runes(&invalid_utf8).is_err());
    }
}
