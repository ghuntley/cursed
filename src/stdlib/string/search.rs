/// Search and replace operations for strings
use super::{StringError, StringResult};

/// Check if haystack contains needle
pub fn contains(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

/// Check if string starts with given prefix
pub fn starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

/// Check if string ends with given suffix
pub fn ends_with(s: &str, suffix: &str) -> bool {
    s.ends_with(suffix)
}

/// Find the first occurrence of needle in haystack
pub fn find(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}

/// Find the last occurrence of needle in haystack
pub fn find_last(haystack: &str, needle: &str) -> Option<usize> {
    haystack.rfind(needle)
}

/// Find all occurrences of needle in haystack
pub fn find_all(haystack: &str, needle: &str) -> Vec<usize> {
    if needle.is_empty() {
        return Vec::new();
    }
    
    let mut positions = Vec::new();
    let mut start = 0;
    
    while let Some(pos) = haystack[start..].find(needle) {
        let absolute_pos = start + pos;
        positions.push(absolute_pos);
        start = absolute_pos + needle.len();
    }
    
    positions
}

/// Replace all occurrences of 'from' with 'to'
pub fn replace(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

/// Replace only the first occurrence of 'from' with 'to'
pub fn replace_first(s: &str, from: &str, to: &str) -> String {
    if let Some(pos) = s.find(from) {
        let mut result = String::with_capacity(s.len() + to.len());
        result.push_str(&s[..pos]);
        result.push_str(to);
        result.push_str(&s[pos + from.len()..]);
        result
    } else {
        s.to_string()
    }
}

/// Replace only the last occurrence of 'from' with 'to'
pub fn replace_last(s: &str, from: &str, to: &str) -> String {
    if let Some(pos) = s.rfind(from) {
        let mut result = String::with_capacity(s.len() + to.len());
        result.push_str(&s[..pos]);
        result.push_str(to);
        result.push_str(&s[pos + from.len()..]);
        result
    } else {
        s.to_string()
    }
}

/// Replace N occurrences of 'from' with 'to'
pub fn replace_n(s: &str, from: &str, to: &str, count: usize) -> String {
    if count == 0 || from.is_empty() {
        return s.to_string();
    }
    
    let mut result = s.to_string();
    let mut replacements = 0;
    let mut start = 0;
    
    while replacements < count {
        if let Some(pos) = result[start..].find(from) {
            let absolute_pos = start + pos;
            result.replace_range(absolute_pos..absolute_pos + from.len(), to);
            start = absolute_pos + to.len();
            replacements += 1;
        } else {
            break;
        }
    }
    
    result
}

/// Count occurrences of needle in haystack
pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    
    haystack.matches(needle).count()
}

/// Case-insensitive contains check
pub fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

/// Case-insensitive find
pub fn find_ignore_case(haystack: &str, needle: &str) -> Option<usize> {
    let haystack_lower = haystack.to_lowercase();
    let needle_lower = needle.to_lowercase();
    haystack_lower.find(&needle_lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(contains("hello world", "world"));
        assert!(contains("hello world", "hello"));
        assert!(contains("hello world", ""));
        assert!(!contains("hello world", "foo"));
        assert!(!contains("", "foo"));
    }

    #[test]
    fn test_starts_with() {
        assert!(starts_with("hello world", "hello"));
        assert!(starts_with("hello world", ""));
        assert!(!starts_with("hello world", "world"));
        assert!(!starts_with("", "hello"));
    }

    #[test]
    fn test_ends_with() {
        assert!(ends_with("hello world", "world"));
        assert!(ends_with("hello world", ""));
        assert!(!ends_with("hello world", "hello"));
        assert!(!ends_with("", "world"));
    }

    #[test]
    fn test_find() {
        assert_eq!(find("hello world", "world"), Some(6));
        assert_eq!(find("hello world", "hello"), Some(0));
        assert_eq!(find("hello world", ""), Some(0));
        assert_eq!(find("hello world", "foo"), None);
    }

    #[test]
    fn test_find_last() {
        assert_eq!(find_last("hello hello", "hello"), Some(6));
        assert_eq!(find_last("hello world", "world"), Some(6));
        assert_eq!(find_last("hello world", "foo"), None);
    }

    #[test]
    fn test_find_all() {
        assert_eq!(find_all("hello hello hello", "hello"), vec![0, 6, 12]);
        assert_eq!(find_all("abcabc", "abc"), vec![0, 3]);
        assert_eq!(find_all("hello", "foo"), Vec::<usize>::new());
        assert_eq!(find_all("hello", ""), Vec::<usize>::new());
    }

    #[test]
    fn test_replace() {
        assert_eq!(replace("hello world", "world", "universe"), "hello universe");
        assert_eq!(replace("hello hello", "hello", "hi"), "hi hi");
        assert_eq!(replace("hello world", "foo", "bar"), "hello world");
    }

    #[test]
    fn test_replace_first() {
        assert_eq!(replace_first("hello hello", "hello", "hi"), "hi hello");
        assert_eq!(replace_first("hello world", "foo", "bar"), "hello world");
    }

    #[test]
    fn test_replace_last() {
        assert_eq!(replace_last("hello hello", "hello", "hi"), "hello hi");
        assert_eq!(replace_last("hello world", "foo", "bar"), "hello world");
    }

    #[test]
    fn test_replace_n() {
        assert_eq!(replace_n("hello hello hello", "hello", "hi", 2), "hi hi hello");
        assert_eq!(replace_n("hello hello", "hello", "hi", 0), "hello hello");
        assert_eq!(replace_n("hello world", "foo", "bar", 1), "hello world");
    }

    #[test]
    fn test_count_occurrences() {
        assert_eq!(count_occurrences("hello hello hello", "hello"), 3);
        assert_eq!(count_occurrences("abcabc", "abc"), 2);
        assert_eq!(count_occurrences("hello", "foo"), 0);
        assert_eq!(count_occurrences("hello", ""), 0);
    }

    #[test]
    fn test_contains_ignore_case() {
        assert!(contains_ignore_case("Hello World", "WORLD"));
        assert!(contains_ignore_case("HELLO WORLD", "world"));
        assert!(!contains_ignore_case("hello", "foo"));
    }

    #[test]
    fn test_find_ignore_case() {
        assert_eq!(find_ignore_case("Hello World", "WORLD"), Some(6));
        assert_eq!(find_ignore_case("HELLO WORLD", "world"), Some(6));
        assert_eq!(find_ignore_case("hello", "foo"), None);
    }
}
