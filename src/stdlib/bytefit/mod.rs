// ByteFit - Binary data manipulation library for CURSED
// Provides utilities for working with binary data, bit manipulation, and byte operations

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

/// Regex find all matches (simple implementation)
pub fn regex_find_all(pattern: &str, text: &str) -> Result<Vec<String>, String> {
    let mut matches = Vec::new();
    
    match pattern {
        r"\d+" => {
            // Find all digit sequences
            let mut current_match = String::new();
            for ch in text.chars() {
                if ch.is_ascii_digit() {
                    current_match.push(ch);
                } else if !current_match.is_empty() {
                    matches.push(current_match.clone());
                    current_match.clear();
                }
            }
            if !current_match.is_empty() {
                matches.push(current_match);
            }
        }
        r"\w+" => {
            // Find all word sequences
            let mut current_match = String::new();
            for ch in text.chars() {
                if ch.is_alphanumeric() || ch == '_' {
                    current_match.push(ch);
                } else if !current_match.is_empty() {
                    matches.push(current_match.clone());
                    current_match.clear();
                }
            }
            if !current_match.is_empty() {
                matches.push(current_match);
            }
        }
        _ => {
            // Simple substring search
            if text.contains(pattern) {
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
