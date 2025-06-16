/// Search functions for byte slices
use super::{ByteFitResult, invalid_utf8};

/// Contains reports whether subslice is within b.
pub fn contains(b: &[u8], subslice: &[u8]) -> bool {
    if subslice.is_empty() {
        return true;
    }
    b.windows(subslice.len()).any(|window| window == subslice)
}

/// ContainsAny reports whether any of the UTF-8-encoded code points in chars are within b.
pub fn contains_any(b: &[u8], chars: &str) -> bool {
    if let Ok(s) = std::str::from_utf8(b) {
        s.chars().any(|c| chars.contains(c))
    } else {
        false
    }
}

/// ContainsRune reports whether the rune is contained in the UTF-8-encoded byte slice.
pub fn contains_rune(b: &[u8], r: char) -> bool {
    if let Ok(s) = std::str::from_utf8(b) {
        s.contains(r)
    } else {
        false
    }
}

/// Count counts the number of non-overlapping instances of sep in s.
pub fn count(s: &[u8], sep: &[u8]) -> usize {
    if sep.is_empty() {
        return s.len() + 1;
    }
    
    let mut count = 0;
    let mut start = 0;
    
    while start <= s.len() {
        if let Some(pos) = find_at(s, sep, start) {
            count += 1;
            start = pos + sep.len();
        } else {
            break;
        }
    }
    
    count
}

/// HasPrefix tests whether the byte slice s begins with prefix.
pub fn has_prefix(s: &[u8], prefix: &[u8]) -> bool {
    s.starts_with(prefix)
}

/// HasSuffix tests whether the byte slice s ends with suffix.
pub fn has_suffix(s: &[u8], suffix: &[u8]) -> bool {
    s.ends_with(suffix)
}

/// Index returns the index of the first instance of sep in s, or -1 if sep is not present.
pub fn index(s: &[u8], sep: &[u8]) -> i32 {
    if sep.is_empty() {
        return 0;
    }
    
    match find_at(s, sep, 0) {
        Some(pos) => pos as i32,
        None => -1,
    }
}

/// IndexAny returns the index of the first instance of any Unicode code point from chars in s.
pub fn index_any(s: &[u8], chars: &str) -> i32 {
    if let Ok(string) = std::str::from_utf8(s) {
        for (i, c) in string.char_indices() {
            if chars.contains(c) {
                return i as i32;
            }
        }
    }
    -1
}

/// IndexByte returns the index of the first instance of c in s, or -1 if c is not present.
pub fn index_byte(s: &[u8], c: u8) -> i32 {
    match s.iter().position(|&x| x == c) {
        Some(pos) => pos as i32,
        None => -1,
    }
}

/// IndexRune returns the index of the first instance of the Unicode code point r in s.
pub fn index_rune(s: &[u8], r: char) -> i32 {
    if let Ok(string) = std::str::from_utf8(s) {
        if let Some(pos) = string.find(r) {
            return pos as i32;
        }
    }
    -1
}

/// LastIndex returns the index of the last instance of sep in s, or -1 if sep is not present.
pub fn last_index(s: &[u8], sep: &[u8]) -> i32 {
    if sep.is_empty() {
        return s.len() as i32;
    }
    
    let mut last_pos = None;
    let mut start = 0;
    
    while start <= s.len() {
        if let Some(pos) = find_at(s, sep, start) {
            last_pos = Some(pos);
            start = pos + 1;
        } else {
            break;
        }
    }
    
    match last_pos {
        Some(pos) => pos as i32,
        None => -1,
    }
}

/// LastIndexAny returns the index of the last instance of any Unicode code point from chars in s.
pub fn last_index_any(s: &[u8], chars: &str) -> i32 {
    if let Ok(string) = std::str::from_utf8(s) {
        let mut last_pos = None;
        for (i, c) in string.char_indices() {
            if chars.contains(c) {
                last_pos = Some(i);
            }
        }
        if let Some(pos) = last_pos {
            return pos as i32;
        }
    }
    -1
}

/// LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present.
pub fn last_index_byte(s: &[u8], c: u8) -> i32 {
    match s.iter().rposition(|&x| x == c) {
        Some(pos) => pos as i32,
        None => -1,
    }
}

/// Helper function to find a pattern starting at a specific position
fn find_at(s: &[u8], pattern: &[u8], start: usize) -> Option<usize> {
    if start > s.len() || pattern.len() > s.len() - start {
        return None;
    }
    
    s[start..].windows(pattern.len())
        .position(|window| window == pattern)
        .map(|pos| start + pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(contains(b"hello world", b"world"));
        assert!(contains(b"hello world", b"hello"));
        assert!(!contains(b"hello world", b"foo"));
        assert!(contains(b"hello world", b""));
        assert!(!contains(b"", b"hello"));
    }

    #[test]
    fn test_contains_any() {
        assert!(contains_any(b"hello", "aeiou"));
        assert!(!contains_any(b"bcdfg", "aeiou"));
        assert!(contains_any("café".as_bytes(), "é"));
    }

    #[test]
    fn test_contains_rune() {
        assert!(contains_rune(b"hello", 'e'));
        assert!(!contains_rune(b"hello", 'x'));
        assert!(contains_rune("café".as_bytes(), 'é'));
    }

    #[test]
    fn test_count() {
        assert_eq!(count(b"aaaa", b"aa"), 2);
        assert_eq!(count(b"hello world hello", b"hello"), 2);
        assert_eq!(count(b"hello", b"x"), 0);
        assert_eq!(count(b"hello", b""), 6);
    }

    #[test]
    fn test_has_prefix() {
        assert!(has_prefix(b"hello world", b"hello"));
        assert!(!has_prefix(b"hello world", b"world"));
        assert!(has_prefix(b"hello", b""));
        assert!(!has_prefix(b"", b"hello"));
    }

    #[test]
    fn test_has_suffix() {
        assert!(has_suffix(b"hello world", b"world"));
        assert!(!has_suffix(b"hello world", b"hello"));
        assert!(has_suffix(b"hello", b""));
        assert!(!has_suffix(b"", b"hello"));
    }

    #[test]
    fn test_index() {
        assert_eq!(index(b"hello world", b"world"), 6);
        assert_eq!(index(b"hello world", b"hello"), 0);
        assert_eq!(index(b"hello world", b"foo"), -1);
        assert_eq!(index(b"hello", b""), 0);
    }

    #[test]
    fn test_index_byte() {
        assert_eq!(index_byte(b"hello", b'e'), 1);
        assert_eq!(index_byte(b"hello", b'x'), -1);
        assert_eq!(index_byte(b"hello", b'h'), 0);
    }

    #[test]
    fn test_last_index() {
        assert_eq!(last_index(b"hello world hello", b"hello"), 12);
        assert_eq!(last_index(b"hello world", b"foo"), -1);
        assert_eq!(last_index(b"hello", b""), 5);
    }

    #[test]
    fn test_last_index_byte() {
        assert_eq!(last_index_byte(b"hello", b'l'), 3);
        assert_eq!(last_index_byte(b"hello", b'x'), -1);
        assert_eq!(last_index_byte(b"hello", b'h'), 0);
    }
}
