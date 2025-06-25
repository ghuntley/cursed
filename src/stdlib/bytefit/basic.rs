/// Basic byte slice operations
use super::{ByteFitResult, invalid_utf8};
use std::cmp::Ordering;

/// Compare returns an integer comparing two byte slices lexicographically.
/// The result will be 0 if a==b, -1 if a < b, and +1 if a > b.
pub fn compare(a: &[u8], b: &[u8]) -> i32 {
    match a.cmp(b) {
    }
}

/// Equal reports whether a and b are the same length and contain the same bytes.
pub fn equal(a: &[u8], b: &[u8]) -> bool {
    a == b
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
    let mut result = Vec::with_capacity(b.len() * count);
    for _ in 0..count {
        result.extend_from_slice(b);
    }
    result
/// Runes converts a slice of bytes to a slice of runes (Unicode code points).
pub fn runes(s: &[u8]) -> ByteFitResult<Vec<char>> {
    match std::str::from_utf8(s) {
    }
}

