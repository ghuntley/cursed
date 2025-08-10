// ByteFit - Binary data manipulation library for CURSED
// Provides utilities for working with binary data, bit manipulation, and byte operations

use std::cmp::Ordering;

pub mod fitbuffer;
pub use fitbuffer::*;

/// Byte manipulation utilities
pub struct ByteFit;

impl ByteFit {
    /// Convert bytes to hex string
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    /// Convert hex string to bytes
    pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
        if hex.len() % 2 != 0 {
            return Err("Hex string must have even length".to_string());
        }
        let mut bytes = Vec::new();
        for i in (0..hex.len()).step_by(2) {
            let byte_str = &hex[i..i+2];
            match u8::from_str_radix(byte_str, 16) {
                Ok(byte) => bytes.push(byte),
                Err(_) => return Err(format!("Invalid hex byte: {}", byte_str)),
            }
        }
        Ok(bytes)
    }
    
    /// Get bit at position
    pub fn get_bit(byte: u8, position: u8) -> bool {
        if position >= 8 {
            return false;
        }
        (byte & (1 << position)) != 0
    }
    
    /// Set bit at position
    pub fn set_bit(byte: u8, position: u8, value: bool) -> u8 {
        if position >= 8 {
            return byte;
        }
        if value {
            byte | (1 << position)
        } else {
            byte & !(1 << position)
        }
    }

    /// Count set bits
    pub fn count_ones(byte: u8) -> u32 {
        byte.count_ones()
    }
    
    /// Reverse bits in byte
    pub fn reverse_bits(byte: u8) -> u8 {
        byte.reverse_bits()
    }
    
    /// Logical NOT operation
    pub fn not(byte: u8) -> u8 {
        !byte
    }
    
    /// Shift left operation
    pub fn shift_left(byte: u8, positions: u8) -> u8 {
        byte << positions
    }
    
    /// Shift right operation
    pub fn shift_right(byte: u8, positions: u8) -> u8 {
        byte >> positions
    }
}

/// Shift left operation on byte arrays
pub fn shift_left(bytes: &[u8], positions: u8) -> Vec<u8> {
    bytes.iter().map(|&b| b << positions).collect()
}

/// Shift right operation on byte arrays
pub fn shift_right(bytes: &[u8], positions: u8) -> Vec<u8> {
    bytes.iter().map(|&b| b >> positions).collect()
}

/// Logical NOT operation on byte arrays
pub fn not(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|&b| !b).collect()
}

/// Wildcard pattern matching
pub fn wildcard_match(pattern: &[u8], text: &[u8]) -> bool {
    wildcard_match_str(
        &String::from_utf8_lossy(pattern),
        &String::from_utf8_lossy(text)
    )
}

/// Wildcard pattern matching for strings
fn wildcard_match_str(pattern: &str, text: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    
    wildcard_match_recursive(&pattern_chars, &text_chars, 0, 0)
}

/// Recursive wildcard matching implementation
fn wildcard_match_recursive(pattern: &[char], text: &[char], p_idx: usize, t_idx: usize) -> bool {
    if p_idx >= pattern.len() {
        return t_idx >= text.len();
    }
    
    match pattern[p_idx] {
        '*' => {
            // Try matching zero or more characters
            for i in t_idx..=text.len() {
                if wildcard_match_recursive(pattern, text, p_idx + 1, i) {
                    return true;
                }
            }
            false
        }
        '?' => {
            // Match exactly one character
            if t_idx >= text.len() {
                false
            } else {
                wildcard_match_recursive(pattern, text, p_idx + 1, t_idx + 1)
            }
        }
        c => {
            // Match exact character
            if t_idx >= text.len() || text[t_idx] != c {
                false
            } else {
                wildcard_match_recursive(pattern, text, p_idx + 1, t_idx + 1)
            }
        }
    }
}

/// Regex matching (simple implementation)
pub fn regex_match(pattern: &str, text: &str) -> Result<bool, String> {
    // Simple regex patterns - in production would use proper regex engine
    match pattern {
        r"\d+" => {
            // Match one or more digits
            Ok(text.chars().any(|c| c.is_ascii_digit()))
        }
        r"\w+" => {
            // Match one or more word characters
            Ok(text.chars().any(|c| c.is_alphanumeric() || c == '_'))
        }
        r"\s+" => {
            // Match one or more whitespace characters
            Ok(text.chars().any(|c| c.is_whitespace()))
        }
        _ => {
            // Exact match fallback
            Ok(text.contains(pattern))
        }
    }
}

/// Regex find all matches with limit (simple implementation)
pub fn regex_find_all(pattern: &str, text: &str, limit: i32) -> Result<Vec<String>, String> {
    let mut matches = Vec::new();
    let max_matches = if limit < 0 { usize::MAX } else { limit as usize };
    
    match pattern {
        r"\d+" => {
            // Find all digit sequences
            let mut current_match = String::new();
            for ch in text.chars() {
                if matches.len() >= max_matches {
                    break;
                }
                if ch.is_ascii_digit() {
                    current_match.push(ch);
                } else if !current_match.is_empty() {
                    matches.push(current_match.clone());
                    current_match.clear();
                }
            }
            if !current_match.is_empty() && matches.len() < max_matches {
                matches.push(current_match);
            }
        }
        r"\w+" => {
            // Find all word sequences
            let mut current_match = String::new();
            for ch in text.chars() {
                if matches.len() >= max_matches {
                    break;
                }
                if ch.is_alphanumeric() || ch == '_' {
                    current_match.push(ch);
                } else if !current_match.is_empty() {
                    matches.push(current_match.clone());
                    current_match.clear();
                }
            }
            if !current_match.is_empty() && matches.len() < max_matches {
                matches.push(current_match);
            }
        }
        _ => {
            // Simple substring search
            if text.contains(pattern) && matches.len() < max_matches {
                matches.push(pattern.to_string());
            }
        }
    }
    
    Ok(matches)
}

/// Regex replace (simple implementation)
pub fn regex_replace(pattern: &str, text: &str, replacement: &str) -> Result<String, String> {
    match pattern {
        r"\d+" => {
            // Replace all digit sequences
            let mut result = String::new();
            let mut current_match = String::new();
            for ch in text.chars() {
                if ch.is_ascii_digit() {
                    current_match.push(ch);
                } else {
                    if !current_match.is_empty() {
                        result.push_str(replacement);
                        current_match.clear();
                    }
                    result.push(ch);
                }
            }
            if !current_match.is_empty() {
                result.push_str(replacement);
            }
            Ok(result)
        }
        r"\w+" => {
            // Replace all word sequences
            let mut result = String::new();
            let mut current_match = String::new();
            for ch in text.chars() {
                if ch.is_alphanumeric() || ch == '_' {
                    current_match.push(ch);
                } else {
                    if !current_match.is_empty() {
                        result.push_str(replacement);
                        current_match.clear();
                    }
                    result.push(ch);
                }
            }
            if !current_match.is_empty() {
                result.push_str(replacement);
            }
            Ok(result)
        }
        _ => {
            // Simple substring replacement
            Ok(text.replace(pattern, replacement))
        }
    }
}

/// Bit manipulation trait
pub trait BitOps {
    fn get_bit(&self, position: u8) -> bool;
    fn set_bit(&mut self, position: u8, value: bool);
    fn count_ones(&self) -> u32;
    fn reverse_bits(&self) -> Self;
}

impl BitOps for u8 {
    fn get_bit(&self, position: u8) -> bool {
        ByteFit::get_bit(*self, position)
    }
    
    fn set_bit(&mut self, position: u8, value: bool) {
        *self = ByteFit::set_bit(*self, position, value);
    }
    
    fn count_ones(&self) -> u32 {
        ByteFit::count_ones(*self)
    }
    
    fn reverse_bits(&self) -> Self {
        ByteFit::reverse_bits(*self)
    }
}

/// Byte array utilities
pub struct ByteArray;

impl ByteArray {
    /// XOR two byte arrays
    pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| x ^ y)
            .collect()
    }
    
    /// Rotate left
    pub fn rotate_left(bytes: &[u8], positions: usize) -> Vec<u8> {
        if bytes.is_empty() {
            return Vec::new();
        }
        let len = bytes.len();
        let pos = positions % len;
        let mut result = Vec::with_capacity(len);
        result.extend_from_slice(&bytes[pos..]);
        result.extend_from_slice(&bytes[..pos]);
        result
    }
    
    /// Rotate right
    pub fn rotate_right(bytes: &[u8], positions: usize) -> Vec<u8> {
        if bytes.is_empty() {
            return Vec::new();
        }
        let len = bytes.len();
        let pos = positions % len;
        let mut result = Vec::with_capacity(len);
        result.extend_from_slice(&bytes[len - pos..]);
        result.extend_from_slice(&bytes[..len - pos]);
        result
    }
}

/// Compare two byte arrays
pub fn compare(a: &[u8], b: &[u8]) -> Ordering {
    a.cmp(b)
}

/// Check if two byte arrays are equal
pub fn equal(a: &[u8], b: &[u8]) -> bool {
    a == b
}

/// Case-insensitive comparison of byte arrays (treating as UTF-8)
pub fn equal_fold(a: &[u8], b: &[u8]) -> bool {
    let a_str = String::from_utf8_lossy(a);
    let b_str = String::from_utf8_lossy(b);
    a_str.to_lowercase() == b_str.to_lowercase()
}

/// Convert bytes to runes (characters)
pub fn runes(bytes: &[u8]) -> Vec<char> {
    String::from_utf8_lossy(bytes).chars().collect()
}

/// Repeat bytes N times
pub fn repeat(bytes: &[u8], n: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(bytes.len() * n);
    for _ in 0..n {
        result.extend_from_slice(bytes);
    }
    result
}

/// Join byte arrays with separator
pub fn join(parts: &[Vec<u8>], separator: &[u8]) -> Vec<u8> {
    if parts.is_empty() {
        return Vec::new();
    }
    
    let mut result = parts[0].clone();
    for part in &parts[1..] {
        result.extend_from_slice(separator);
        result.extend_from_slice(part);
    }
    result
}

/// Replace occurrences with limit
pub fn replace(bytes: &[u8], old: &[u8], new: &[u8], limit: usize) -> Vec<u8> {
    if old.is_empty() || limit == 0 {
        return bytes.to_vec();
    }
    
    let mut result = Vec::new();
    let mut pos = 0;
    let mut replacements = 0;
    
    while pos < bytes.len() && replacements < limit {
        if pos + old.len() <= bytes.len() && &bytes[pos..pos + old.len()] == old {
            result.extend_from_slice(new);
            pos += old.len();
            replacements += 1;
        } else {
            result.push(bytes[pos]);
            pos += 1;
        }
    }
    
    // Add remaining bytes
    if pos < bytes.len() {
        result.extend_from_slice(&bytes[pos..]);
    }
    
    result
}

/// Check if bytes contain a subsequence
pub fn contains(bytes: &[u8], subseq: &[u8]) -> bool {
    bytes.windows(subseq.len()).any(|window| window == subseq)
}

/// Check if bytes contain any of the specified characters
pub fn contains_any(bytes: &[u8], chars: &[u8]) -> bool {
    bytes.iter().any(|&b| chars.contains(&b))
}

/// Check if bytes contain a specific character
pub fn contains_rune(bytes: &[u8], rune: char) -> bool {
    String::from_utf8_lossy(bytes).contains(rune)
}

/// Count occurrences of a subsequence in bytes
pub fn count(bytes: &[u8], subseq: &[u8]) -> usize {
    if subseq.is_empty() {
        return bytes.len() + 1;
    }
    
    let mut count = 0;
    let mut pos = 0;
    
    while pos <= bytes.len() - subseq.len() {
        if &bytes[pos..pos + subseq.len()] == subseq {
            count += 1;
            pos += subseq.len();
        } else {
            pos += 1;
        }
    }
    count
}

/// Find index of first occurrence of a subsequence
pub fn index(bytes: &[u8], subseq: &[u8]) -> Option<usize> {
    if subseq.is_empty() {
        return Some(0);
    }
    
    bytes.windows(subseq.len())
        .position(|window| window == subseq)
}

/// Find last occurrence of a byte
pub fn last_index_byte(bytes: &[u8], byte: u8) -> Option<usize> {
    bytes.iter().rposition(|&b| b == byte)
}

/// Check if bytes start with a prefix
pub fn has_prefix(bytes: &[u8], prefix: &[u8]) -> bool {
    bytes.starts_with(prefix)
}

/// Check if bytes end with a suffix
pub fn has_suffix(bytes: &[u8], suffix: &[u8]) -> bool {
    bytes.ends_with(suffix)
}

/// Replace all occurrences of a subsequence
pub fn replace_all(bytes: &[u8], old: &[u8], new: &[u8]) -> Vec<u8> {
    if old.is_empty() {
        return bytes.to_vec();
    }
    
    let mut result = Vec::new();
    let mut pos = 0;
    
    while pos < bytes.len() {
        if pos + old.len() <= bytes.len() && &bytes[pos..pos + old.len()] == old {
            result.extend_from_slice(new);
            pos += old.len();
        } else {
            result.push(bytes[pos]);
            pos += 1;
        }
    }
    
    result
}

/// Convert bytes to uppercase (treating as UTF-8)
pub fn to_upper(bytes: &[u8]) -> Vec<u8> {
    String::from_utf8_lossy(bytes).to_uppercase().into_bytes()
}

/// Convert bytes to lowercase (treating as UTF-8)
pub fn to_lower(bytes: &[u8]) -> Vec<u8> {
    String::from_utf8_lossy(bytes).to_lowercase().into_bytes()
}

/// Convert bytes to title case (treating as UTF-8)
pub fn to_title(bytes: &[u8]) -> Vec<u8> {
    let s = String::from_utf8_lossy(bytes);
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for ch in s.chars() {
        if ch.is_alphabetic() {
            if capitalize_next {
                result.push_str(&ch.to_uppercase().collect::<String>());
                capitalize_next = false;
            } else {
                result.push_str(&ch.to_lowercase().collect::<String>());
            }
        } else {
            result.push(ch);
            capitalize_next = true;
        }
    }
    
    result.into_bytes()
}

/// Create a new fit buffer with specified capacity
pub fn new_fit_buffer(capacity: usize) -> FitBuffer {
    FitBuffer::new(capacity)
}

/// Convert bytes to hex string (public function)
pub fn to_hex(bytes: &[u8]) -> String {
    ByteFit::bytes_to_hex(bytes)
}

/// Convert hex string to bytes (public function)
pub fn from_hex(hex: &str) -> Result<Vec<u8>, String> {
    ByteFit::hex_to_bytes(hex)
}

/// Convert bytes to base64 string
pub fn to_base64(bytes: &[u8]) -> String {
    // Simple base64 encoding without padding for simplicity
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::new();
    let mut i = 0;
    
    while i < bytes.len() {
        let b1 = bytes[i] as usize;
        let b2 = if i + 1 < bytes.len() { bytes[i + 1] as usize } else { 0 };
        let b3 = if i + 2 < bytes.len() { bytes[i + 2] as usize } else { 0 };
        
        let bitmap = (b1 << 16) | (b2 << 8) | b3;
        
        result.push(CHARS[(bitmap >> 18) & 0x3F] as char);
        result.push(CHARS[(bitmap >> 12) & 0x3F] as char);
        
        if i + 1 < bytes.len() {
            result.push(CHARS[(bitmap >> 6) & 0x3F] as char);
        }
        if i + 2 < bytes.len() {
            result.push(CHARS[bitmap & 0x3F] as char);
        }
        
        i += 3;
    }
    
    result
}

/// Convert base64 string to bytes
pub fn from_base64(base64: &str) -> Result<Vec<u8>, String> {
    let chars = base64.chars().collect::<Vec<_>>();
    let mut result = Vec::new();
    let mut i = 0;
    
    while i < chars.len() {
        let mut bitmap = 0u32;
        let mut padding = 0;
        
        for j in 0..4 {
            if i + j < chars.len() {
                let ch = chars[i + j];
                let val = match ch {
                    'A'..='Z' => (ch as u8 - b'A') as u32,
                    'a'..='z' => (ch as u8 - b'a' + 26) as u32,
                    '0'..='9' => (ch as u8 - b'0' + 52) as u32,
                    '+' => 62,
                    '/' => 63,
                    '=' => {
                        padding += 1;
                        0
                    }
                    _ => return Err(format!("Invalid base64 character: {}", ch)),
                };
                bitmap |= val << (18 - j * 6);
            }
        }
        
        result.push((bitmap >> 16) as u8);
        if padding < 2 {
            result.push((bitmap >> 8) as u8);
        }
        if padding < 1 {
            result.push(bitmap as u8);
        }
        
        i += 4;
    }
    
    Ok(result)
}

/// Bitwise AND operation on byte arrays
pub fn and(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x & y)
        .collect()
}

/// Bitwise OR operation on byte arrays
pub fn or(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x | y)
        .collect()
}

/// Bitwise XOR operation on byte arrays (public function)
pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    ByteArray::xor(a, b)
}

/// Split bytes by separator
pub fn split(bytes: &[u8], separator: &[u8]) -> Vec<Vec<u8>> {
    if separator.is_empty() {
        return bytes.iter().map(|&b| vec![b]).collect();
    }
    
    let mut result = Vec::new();
    let mut start = 0;
    
    while start < bytes.len() {
        match bytes[start..].windows(separator.len()).position(|w| w == separator) {
            Some(pos) => {
                result.push(bytes[start..start + pos].to_vec());
                start += pos + separator.len();
            }
            None => {
                result.push(bytes[start..].to_vec());
                break;
            }
        }
    }
    
    result
}

/// Split bytes by separator with limit
pub fn split_n(bytes: &[u8], separator: &[u8], n: usize) -> Vec<Vec<u8>> {
    if n == 0 {
        return vec![bytes.to_vec()];
    }
    
    let mut result = Vec::new();
    let mut start = 0;
    let mut splits = 0;
    
    while start < bytes.len() && splits < n - 1 {
        match bytes[start..].windows(separator.len()).position(|w| w == separator) {
            Some(pos) => {
                result.push(bytes[start..start + pos].to_vec());
                start += pos + separator.len();
                splits += 1;
            }
            None => break,
        }
    }
    
    if start < bytes.len() {
        result.push(bytes[start..].to_vec());
    }
    
    result
}

/// Split bytes after separator (include separator in result)
pub fn split_after(bytes: &[u8], separator: &[u8]) -> Vec<Vec<u8>> {
    if separator.is_empty() {
        return vec![bytes.to_vec()];
    }
    
    let mut result = Vec::new();
    let mut start = 0;
    
    while start < bytes.len() {
        match bytes[start..].windows(separator.len()).position(|w| w == separator) {
            Some(pos) => {
                result.push(bytes[start..start + pos + separator.len()].to_vec());
                start += pos + separator.len();
            }
            None => {
                result.push(bytes[start..].to_vec());
                break;
            }
        }
    }
    
    result
}

/// Split bytes into fields (whitespace-separated)
pub fn fields(bytes: &[u8]) -> Vec<Vec<u8>> {
    let s = String::from_utf8_lossy(bytes);
    s.split_whitespace()
        .map(|field| field.as_bytes().to_vec())
        .collect()
}

/// Trim whitespace from bytes
pub fn trim_space(bytes: &[u8]) -> Vec<u8> {
    String::from_utf8_lossy(bytes).trim().as_bytes().to_vec()
}

/// Trim characters from left
pub fn trim_left(bytes: &[u8], chars: &str) -> Vec<u8> {
    let s = String::from_utf8_lossy(bytes);
    s.trim_start_matches(chars).as_bytes().to_vec()
}

/// Trim characters from right
pub fn trim_right(bytes: &[u8], chars: &str) -> Vec<u8> {
    let s = String::from_utf8_lossy(bytes);
    s.trim_end_matches(chars).as_bytes().to_vec()
}

/// Trim prefix from bytes
pub fn trim_prefix(bytes: &[u8], prefix: &[u8]) -> Vec<u8> {
    if bytes.starts_with(prefix) {
        bytes[prefix.len()..].to_vec()
    } else {
        bytes.to_vec()
    }
}

/// Trim suffix from bytes
pub fn trim_suffix(bytes: &[u8], suffix: &[u8]) -> Vec<u8> {
    if bytes.ends_with(suffix) {
        bytes[..bytes.len() - suffix.len()].to_vec()
    } else {
        bytes.to_vec()
    }
}

/// Regex match wrapper for bytes
pub fn regex_match_bytes(pattern: &str, bytes: &[u8]) -> Result<bool, String> {
    let text = String::from_utf8_lossy(bytes);
    regex_match(pattern, &text)
}

/// Regex find all wrapper for bytes
pub fn regex_find_all_bytes(pattern: &str, bytes: &[u8], limit: i32) -> Result<Vec<Vec<u8>>, String> {
    let text = String::from_utf8_lossy(bytes);
    let matches = regex_find_all(pattern, &text, limit)?;
    Ok(matches.into_iter().map(|s| s.into_bytes()).collect())
}

/// Regex replace wrapper for bytes
pub fn regex_replace_bytes(pattern: &str, bytes: &[u8], replacement: &[u8]) -> Result<Vec<u8>, String> {
    let text = String::from_utf8_lossy(bytes);
    let replacement_str = String::from_utf8_lossy(replacement);
    let result = regex_replace(pattern, &text, &replacement_str)?;
    Ok(result.into_bytes())
}
