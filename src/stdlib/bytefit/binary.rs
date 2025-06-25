use crate::error::CursedError;
/// Binary data manipulation functions
use super::{ByteFitError, ByteFitResult, invalid_hex, invalid_base64, invalid_input};

/// FromHex converts a hex-encoded byte slice to its binary representation
pub fn from_hex(s: &[u8]) -> ByteFitResult<Vec<u8>> {
    let hex_str = match std::str::from_utf8(s) {
    
    if hex_str.len() % 2 != 0 {
        return Err(invalid_hex("Hex string must have even length"));
    let mut result = Vec::with_capacity(hex_str.len() / 2);
    let chars: Vec<char> = hex_str.chars().collect();
    
    for chunk in chars.chunks(2) {
        let hex_byte = format!("{}{}", chunk[0], chunk[1]);
        match u8::from_str_radix(&hex_byte, 16) {
        }
    }
    
    Ok(result)
/// ToHex converts a byte slice to its hex-encoded representation
pub fn to_hex(s: &[u8]) -> Vec<u8> {
    let hex_string: String = s.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    hex_string.into_bytes()
/// FromBase64 decodes a base64-encoded byte slice
pub fn from_base64(s: &[u8]) -> ByteFitResult<Vec<u8>> {
    let base64_str = match std::str::from_utf8(s) {
    
    // Simple base64 decoder (for demonstration - in production, use a proper library)
    let cleaned = base64_str.chars().filter(|&c| c != '\n' && c != '\r' && c != ' ').collect::<String>();
    
    // Basic base64 character validation
    for c in cleaned.chars() {
        if !c.is_ascii_alphanumeric() && c != '+' && c != '/' && c != '=' {
            return Err(invalid_base64(&format!("Invalid base64 character: {}", c)));
        }
    }
    
    // Simplified base64 decoding - in production, use base64 crate
    match base64_decode_simple(&cleaned) {
    }
}

/// ToBase64 encodes a byte slice to base64
pub fn to_base64(s: &[u8]) -> Vec<u8> {
    let base64_string = base64_encode_simple(s);
    base64_string.into_bytes()
/// Bitwise AND operation on two byte slices
pub fn and(a: &[u8], b: &[u8]) -> Vec<u8> {
    let len = std::cmp::min(a.len(), b.len());
    let mut result = Vec::with_capacity(len);
    
    for i in 0..len {
        result.push(a[i] & b[i]);
    result
/// Bitwise OR operation on two byte slices
pub fn or(a: &[u8], b: &[u8]) -> Vec<u8> {
    let len = std::cmp::min(a.len(), b.len());
    let mut result = Vec::with_capacity(len);
    
    for i in 0..len {
        result.push(a[i] | b[i]);
    result
/// Bitwise XOR operation on two byte slices
pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let len = std::cmp::min(a.len(), b.len());
    let mut result = Vec::with_capacity(len);
    
    for i in 0..len {
        result.push(a[i] ^ b[i]);
    result
/// Bitwise NOT operation on a byte slice
pub fn not(a: &[u8]) -> Vec<u8> {
    a.iter().map(|&byte| !byte).collect()
/// Bitwise left shift operation on a byte slice
pub fn shift_left(a: &[u8], bits: usize) -> Vec<u8> {
    if bits == 0 {
        return a.to_vec();
    if bits >= 8 {
        let byte_shift = bits / 8;
        let bit_shift = bits % 8;
        let mut result = vec![0u8; byte_shift];
        
        if bit_shift == 0 {
            result.extend_from_slice(a);
        } else {
            let mut carry = 0u8;
            for &byte in a {
                let shifted = (byte << bit_shift) | carry;
                result.push(shifted);
                carry = byte >> (8 - bit_shift);
            }
            if carry != 0 {
                result.push(carry);
            }
        }
        
        result
    } else {
        let mut result = Vec::with_capacity(a.len() + 1);
        let mut carry = 0u8;
        
        for &byte in a {
            let shifted = (byte << bits) | carry;
            result.push(shifted);
            carry = byte >> (8 - bits);
        if carry != 0 {
            result.push(carry);
        result
    }
}

/// Bitwise right shift operation on a byte slice
pub fn shift_right(a: &[u8], bits: usize) -> Vec<u8> {
    if bits == 0 {
        return a.to_vec();
    if bits >= 8 {
        let byte_shift = bits / 8;
        let bit_shift = bits % 8;
        
        if byte_shift >= a.len() {
            return vec![0];
        let slice = &a[byte_shift..];
        if bit_shift == 0 {
            return slice.to_vec();
        let mut result = Vec::with_capacity(slice.len());
        let mut carry = 0u8;
        
        for &byte in slice.iter().rev() {
            let shifted = (byte >> bit_shift) | carry;
            result.insert(0, shifted);
            carry = byte << (8 - bit_shift);
        result
    } else {
        let mut result = Vec::with_capacity(a.len());
        let mut carry = 0u8;
        
        for &byte in a.iter().rev() {
            let shifted = (byte >> bits) | carry;
            result.insert(0, shifted);
            carry = byte << (8 - bits);
        // Remove leading zeros
        while result.len() > 1 && result[0] == 0 {
            result.remove(0);
        if result.is_empty() {
            result.push(0);
        result
    }
}

/// Simple base64 encoding (for demonstration)
fn base64_encode_simple(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in input.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        let b0 = buf[0] as usize;
        let b1 = buf[1] as usize;
        let b2 = buf[2] as usize;
        
        result.push(CHARS[b0 >> 2] as char);
        result.push(CHARS[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
        
        if chunk.len() > 1 {
            result.push(CHARS[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        if chunk.len() > 2 {
            result.push(CHARS[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
    }
    
    result
/// Simple base64 decoding (for demonstration)
fn base64_decode_simple(input: &str) -> Result<Vec<u8>, String> {
    let mut chars_to_values = [0u8; 256];
    
    // Initialize lookup table
    for (i, &c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
        chars_to_values[c as usize] = i as u8;
    let input = input.trim_end_matches('=');
    let mut result = Vec::new();
    let mut buf = 0u32;
    let mut bits = 0;
    
    for c in input.chars() {
        if c == '=' {
            break;
        let val = chars_to_values[c as usize] as u32;
        buf = (buf << 6) | val;
        bits += 6;
        
        if bits >= 8 {
            bits -= 8;
            result.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    
    Ok(result)
