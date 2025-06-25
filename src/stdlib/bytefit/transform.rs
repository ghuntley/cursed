/// Transformation functions for byte slices
use super::{ByteFitResult, invalid_utf8, invalid_input};

/// Join concatenates the elements of s to create a new byte slice with sep between each element.
pub fn join(s: &[&[u8]], sep: &[u8]) -> Vec<u8> {
    if s.is_empty() {
        return Vec::new();
    }
    
    if s.len() == 1 {
        return s[0].to_vec();
    }
    
    let total_len = s.iter().map(|slice| slice.len()).sum::<usize>() + sep.len() * (s.len() - 1);
    let mut result = Vec::with_capacity(total_len);
    
    for (i, slice) in s.iter().enumerate() {
        if i > 0 {
            result.extend_from_slice(sep);
        }
        result.extend_from_slice(slice);
    }
    
    result
}

/// Replace returns a copy of the slice s with the first n non-overlapping instances of old replaced by new.
pub fn replace(s: &[u8], old: &[u8], new: &[u8], n: usize) -> Vec<u8> {
    if n == 0 || old.is_empty() || s.is_empty() {
        return s.to_vec();
    }
    
    let mut result = Vec::new();
    let mut remaining = s;
    let mut replacements = 0;
    
    while replacements < n && !remaining.is_empty() {
        if let Some(pos) = find_pattern(remaining, old) {
            // Add everything before the match
            result.extend_from_slice(&remaining[..pos]);
            // Add the replacement
            result.extend_from_slice(new);
            // Move past the matched pattern
            remaining = &remaining[pos + old.len()..];
            replacements += 1;
        } else {
            // No more matches, add the rest
            result.extend_from_slice(remaining);
            break;
        }
    }
    
    // Add any remaining bytes if we've hit the replacement limit
    if replacements == n {
        result.extend_from_slice(remaining);
    }
    
    result
}

/// ReplaceAll returns a copy of the slice s with all non-overlapping instances of old replaced by new.
pub fn replace_all(s: &[u8], old: &[u8], new: &[u8]) -> Vec<u8> {
    if old.is_empty() || s.is_empty() {
        return s.to_vec();
    }
    
    let mut result = Vec::new();
    let mut start = 0;
    
    while start < s.len() {
        if let Some(pos) = find_pattern(&s[start..], old) {
            let actual_pos = start + pos;
            // Add everything before the match
            result.extend_from_slice(&s[start..actual_pos]);
            // Add the replacement
            result.extend_from_slice(new);
            // Move past the matched pattern
            start = actual_pos + old.len();
        } else {
            // No more matches, add the rest
            result.extend_from_slice(&s[start..]);
            break;
        }
    }
    
    result
}

/// Map returns a copy of the byte slice s with all its characters modified per mapping function.
pub fn map<F>(mapping: F, s: &[u8]) -> ByteFitResult<Vec<u8>>
where
    F: Fn(char) -> char,
{
    match std::str::from_utf8(s) {
        Ok(string) => {
            let mapped: String = string.chars().map(mapping).collect();
            Ok(mapped.into_bytes())
        }
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

/// ToUpper returns a copy of the byte slice s with all Unicode letters mapped to their upper case.
pub fn to_upper(s: &[u8]) -> ByteFitResult<Vec<u8>> {
    map(|c| c.to_uppercase().next().unwrap_or(c), s)
}

/// ToLower returns a copy of the byte slice s with all Unicode letters mapped to their lower case.
pub fn to_lower(s: &[u8]) -> ByteFitResult<Vec<u8>> {
    map(|c| c.to_lowercase().next().unwrap_or(c), s)
}

/// ToTitle returns a copy of the byte slice s with all Unicode letters mapped to their title case.
pub fn to_title(s: &[u8]) -> ByteFitResult<Vec<u8>> {
    match std::str::from_utf8(s) {
        Ok(string) => {
            let mut result = String::new();
            let mut capitalize_next = true;
            
            for c in string.chars() {
                if c.is_alphabetic() {
                    if capitalize_next {
                        result.extend(c.to_uppercase());
                        capitalize_next = false;
                    } else {
                        result.extend(c.to_lowercase());
                    }
                } else {
                    result.push(c);
                    capitalize_next = c.is_whitespace();
                }
            }
            
            Ok(result.into_bytes())
        }
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

/// Helper function to find a pattern in a byte slice
fn find_pattern(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    
    haystack.windows(needle.len()).position(|window| window == needle)
}

