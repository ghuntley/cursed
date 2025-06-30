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
