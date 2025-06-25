/// Trimming functions for byte slices
use super::{ByteFitResult, invalid_utf8};

/// Trim returns a subslice of s by removing all leading and trailing UTF-8-encoded code points contained in cutset.
pub fn trim(s: &[u8], cutset: &str) -> ByteFitResult<Vec<u8>> {
    match std::str::from_utf8(s) {
        Ok(string) => {
            let trimmed = string.trim_matches(|c: char| cutset.contains(c));
            Ok(trimmed.as_bytes().to_vec())
        }
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

/// TrimLeft returns a subslice of s by removing all leading UTF-8-encoded code points contained in cutset.
pub fn trim_left(s: &[u8], cutset: &str) -> ByteFitResult<Vec<u8>> {
    match std::str::from_utf8(s) {
        Ok(string) => {
            let trimmed = string.trim_start_matches(|c: char| cutset.contains(c));
            Ok(trimmed.as_bytes().to_vec())
        }
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

/// TrimRight returns a subslice of s by removing all trailing UTF-8-encoded code points contained in cutset.
pub fn trim_right(s: &[u8], cutset: &str) -> ByteFitResult<Vec<u8>> {
    match std::str::from_utf8(s) {
        Ok(string) => {
            let trimmed = string.trim_end_matches(|c: char| cutset.contains(c));
            Ok(trimmed.as_bytes().to_vec())
        }
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

/// TrimSpace returns a subslice of s by removing all leading and trailing white space.
pub fn trim_space(s: &[u8]) -> ByteFitResult<Vec<u8>> {
    match std::str::from_utf8(s) {
        Ok(string) => {
            let trimmed = string.trim();
            Ok(trimmed.as_bytes().to_vec())
        }
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

/// TrimPrefix returns s without the provided leading prefix string.
pub fn trim_prefix(s: &[u8], prefix: &[u8]) -> Vec<u8> {
    if s.starts_with(prefix) {
        s[prefix.len()..].to_vec()
    } else {
        s.to_vec()
    }
}

/// TrimSuffix returns s without the provided trailing suffix string.
pub fn trim_suffix(s: &[u8], suffix: &[u8]) -> Vec<u8> {
    if s.ends_with(suffix) {
        s[..s.len() - suffix.len()].to_vec()
    } else {
        s.to_vec()
    }
}

/// TrimFunc returns a subslice of s by removing all leading and trailing Unicode code points c that satisfy f(c).
pub fn trim_func<F>(s: &[u8], f: F) -> ByteFitResult<Vec<u8>>
where
    F: Fn(char) -> bool,
{
    match std::str::from_utf8(s) {
        Ok(string) => {
            let trimmed = string.trim_matches(f);
            Ok(trimmed.as_bytes().to_vec())
        }
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

