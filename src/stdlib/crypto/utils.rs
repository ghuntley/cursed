/// fr fr Cryptographic Utilities for CURSED - Essential crypto helpers bestie
/// 
/// This module provides comprehensive utility functions for cryptographic operations:
/// - Secure random number generation
/// - Key and IV/nonce generation
/// - Padding schemes (PKCS#7)
/// - Secure memory operations
/// - Validation utilities

use crate::stdlib::packages::crypto_random::{CryptographicRng, CsprngAlgorithm, fill_random};
use crate::stdlib::packages::crypto_advanced::{constant_time_compare, SecureMemory, clear_sensitive_data};
use super::symmetric::{CryptoError, CryptoResult, EncryptionKey};
use std::fmt;

/// fr fr Secure random number generator
pub struct SecureRandom {
    rng: CryptographicRng,
}

impl SecureRandom {
    /// slay Create new secure random generator
    pub fn new() -> CryptoResult<Self> {
        let rng = CryptographicRng::new(CsprngAlgorithm::ChaCha20)
            .map_err(|e| CryptoError::RandomGenerationFailed(format!("Failed to create RNG: {:?}", e)))?;
        
        Ok(Self { rng })
    }
    
    /// slay Initialize system entropy pool
    pub fn init_system_entropy() -> CryptoResult<()> {
        // Placeholder for system entropy initialization
        // In a real implementation, this would seed the system RNG
        Ok(())
    }
    
    /// slay Generate secure random bytes
    pub fn generate_bytes(&self, size: usize) -> CryptoResult<Vec<u8>> {
        let mut bytes = vec![0u8; size];
        fill_random(&mut bytes)
            .map_err(|e| CryptoError::RandomGenerationFailed(format!("Random generation failed: {:?}", e)))?;
        Ok(bytes)
    }
    
    /// slay Generate random u32
    pub fn generate_u32(&self) -> CryptoResult<u32> {
        let bytes = self.generate_bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    
    /// slay Generate random u64
    pub fn generate_u64(&self) -> CryptoResult<u64> {
        let bytes = self.generate_bytes(8)?;
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    }
}

/// slay Generate secure random bytes (convenience function)
pub fn secure_random_bytes(size: usize) -> CryptoResult<Vec<u8>> {
    let rng = SecureRandom::new()?;
    rng.generate_bytes(size)
}

/// slay Generate encryption key of specified size
pub fn generate_key(algorithm: &str, size: usize) -> CryptoResult<EncryptionKey> {
    EncryptionKey::generate(algorithm, size)
}

/// slay Generate IV for block ciphers
pub fn generate_iv(size: usize) -> CryptoResult<Vec<u8>> {
    secure_random_bytes(size)
}

/// slay Generate nonce for stream ciphers
pub fn generate_nonce(size: usize) -> CryptoResult<Vec<u8>> {
    secure_random_bytes(size)
}

/// fr fr Padding schemes for block ciphers
#[derive(Debug, Clone, PartialEq)]
pub enum PaddingScheme {
    Pkcs7,
    AnsiX923,
    Zero,
    None,
}

impl fmt::Display for PaddingScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaddingScheme::Pkcs7 => write!(f, "PKCS#7"),
            PaddingScheme::AnsiX923 => write!(f, "ANSI X9.23"),
            PaddingScheme::Zero => write!(f, "Zero padding"),
            PaddingScheme::None => write!(f, "No padding"),
        }
    }
}

/// fr fr PKCS#7 padding implementation
pub struct Pkcs7Padding;

impl Pkcs7Padding {
    /// slay Apply PKCS#7 padding to data
    pub fn apply(data: &[u8], block_size: usize) -> CryptoResult<Vec<u8>> {
        if block_size == 0 || block_size > 255 {
            return Err(CryptoError::PaddingError(format!("Invalid block size: {}", block_size)));
        }
        
        let padding_len = block_size - (data.len() % block_size);
        let mut padded = data.to_vec();
        padded.extend(vec![padding_len as u8; padding_len]);
        Ok(padded)
    }
    
    /// slay Remove PKCS#7 padding from data
    pub fn remove(data: &[u8]) -> CryptoResult<Vec<u8>> {
        if data.is_empty() {
            return Err(CryptoError::PaddingError("Cannot remove padding from empty data".to_string()));
        }
        
        let padding_len = *data.last().unwrap() as usize;
        
        if padding_len == 0 || padding_len > data.len() {
            return Err(CryptoError::PaddingError(format!("Invalid padding length: {}", padding_len)));
        }
        
        // Verify all padding bytes are correct
        let padding_start = data.len() - padding_len;
        for byte in &data[padding_start..] {
            if *byte != padding_len as u8 {
                return Err(CryptoError::PaddingError("Invalid PKCS#7 padding".to_string()));
            }
        }
        
        Ok(data[..padding_start].to_vec())
    }
    
    /// slay Validate PKCS#7 padding
    pub fn validate(data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }
        
        let padding_len = *data.last().unwrap() as usize;
        
        if padding_len == 0 || padding_len > data.len() {
            return false;
        }
        
        let padding_start = data.len() - padding_len;
        data[padding_start..].iter().all(|&b| b == padding_len as u8)
    }
}

/// slay Apply padding to data
pub fn apply_padding(data: &[u8], block_size: usize) -> CryptoResult<Vec<u8>> {
    Pkcs7Padding::apply(data, block_size)
}

/// slay Remove padding from data
pub fn remove_padding(data: &[u8]) -> CryptoResult<Vec<u8>> {
    Pkcs7Padding::remove(data)
}

/// fr fr IV/Nonce generator with uniqueness guarantees
pub struct IvGenerator {
    counter: u64,
    random_bytes: Vec<u8>,
}

impl IvGenerator {
    /// slay Create new IV generator
    pub fn new() -> CryptoResult<Self> {
        let random_bytes = secure_random_bytes(8)?;
        Ok(Self {
            counter: 0,
            random_bytes,
        })
    }
    
    /// slay Generate IV with counter-based uniqueness
    pub fn generate_iv(&mut self, size: usize) -> CryptoResult<Vec<u8>> {
        if size < 8 {
            return Err(CryptoError::InvalidIvSize(format!("IV size must be at least 8 bytes, got {}", size)));
        }
        
        let mut iv = vec![0u8; size];
        
        // Use counter for uniqueness
        let counter_bytes = self.counter.to_le_bytes();
        iv[..8].copy_from_slice(&counter_bytes);
        
        // Add random bytes for security
        let random_bytes = secure_random_bytes(size - 8)?;
        iv[8..].copy_from_slice(&random_bytes);
        
        self.counter = self.counter.wrapping_add(1);
        Ok(iv)
    }
    
    /// slay Generate nonce with timestamp and randomness
    pub fn generate_nonce(&mut self, size: usize) -> CryptoResult<Vec<u8>> {
        if size < 12 {
            return Err(CryptoError::InvalidNonceSize(format!("Nonce size must be at least 12 bytes, got {}", size)));
        }
        
        let mut nonce = vec![0u8; size];
        
        // Use timestamp for uniqueness
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        let timestamp_bytes = timestamp.to_le_bytes();
        nonce[..8].copy_from_slice(&timestamp_bytes);
        
        // Add counter
        let counter_bytes = self.counter.to_le_bytes();
        nonce[8..12].copy_from_slice(&counter_bytes[..4]);
        
        // Fill remaining with random data
        if size > 12 {
            let random_bytes = secure_random_bytes(size - 12)?;
            nonce[12..].copy_from_slice(&random_bytes);
        }
        
        self.counter = self.counter.wrapping_add(1);
        Ok(nonce)
    }
}

/// fr fr Nonce manager for AEAD ciphers
pub struct NonceManager {
    algorithm: String,
    generator: IvGenerator,
    used_nonces: std::collections::HashSet<Vec<u8>>,
}

impl NonceManager {
    /// slay Create new nonce manager
    pub fn new(algorithm: &str) -> CryptoResult<Self> {
        Ok(Self {
            algorithm: algorithm.to_string(),
            generator: IvGenerator::new()?,
            used_nonces: std::collections::HashSet::new(),
        })
    }
    
    /// slay Generate unique nonce
    pub fn generate_unique_nonce(&mut self, size: usize) -> CryptoResult<Vec<u8>> {
        let max_attempts = 1000;
        
        for _ in 0..max_attempts {
            let nonce = self.generator.generate_nonce(size)?;
            
            if !self.used_nonces.contains(&nonce) {
                self.used_nonces.insert(nonce.clone());
                return Ok(nonce);
            }
        }
        
        Err(CryptoError::RandomGenerationFailed("Failed to generate unique nonce after 1000 attempts".to_string()))
    }
    
    /// slay Check if nonce was already used
    pub fn is_nonce_used(&self, nonce: &[u8]) -> bool {
        self.used_nonces.contains(nonce)
    }
    
    /// slay Clear used nonces (for testing or when restarting)
    pub fn clear_used_nonces(&mut self) {
        self.used_nonces.clear();
    }
}

/// fr fr Secure memory operations
pub struct SecureOps;

impl SecureOps {
    /// slay Securely clear memory
    pub fn secure_clear(data: &mut [u8]) {
        clear_sensitive_data(data);
    }
    
    /// slay Constant-time memory comparison
    pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
        constant_time_compare(a, b)
    }
    
    /// slay Timing-safe memory comparison (alias for secure_compare)
    pub fn timing_safe_memcmp(a: &[u8], b: &[u8]) -> bool {
        Self::secure_compare(a, b)
    }
    
    /// slay Copy memory with secure overwrite of source
    pub fn secure_copy(src: &mut [u8], dst: &mut [u8]) -> CryptoResult<()> {
        if src.len() != dst.len() {
            return Err(CryptoError::InvalidDataSize(format!("Source and destination lengths don't match: {} vs {}", src.len(), dst.len())));
        }
        
        dst.copy_from_slice(src);
        Self::secure_clear(src);
        Ok(())
    }
}

/// slay Securely clear memory (convenience function)
pub fn secure_clear(data: &mut [u8]) {
    SecureOps::secure_clear(data);
}

/// slay Constant-time memory comparison (convenience function)
pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    SecureOps::secure_compare(a, b)
}

/// slay Timing-safe memory comparison (convenience function)
pub fn timing_safe_memcmp(a: &[u8], b: &[u8]) -> bool {
    SecureOps::timing_safe_memcmp(a, b)
}

/// fr fr Validation utilities
pub struct CryptoValidator;

impl CryptoValidator {
    /// slay Validate key size for algorithm
    pub fn validate_key_size(algorithm: &str, key_size: usize) -> CryptoResult<()> {
        let expected = match algorithm {
            "AES-128-CBC" | "AES-128-GCM" => 16,
            "AES-192-CBC" | "AES-192-GCM" => 24,
            "AES-256-CBC" | "AES-256-GCM" => 32,
            "ChaCha20" | "ChaCha20-Poly1305" => 32,
            _ => return Err(CryptoError::UnsupportedCipher(algorithm.to_string())),
        };
        
        if key_size != expected {
            return Err(CryptoError::InvalidKeySize(format!(
                "{} requires {}-byte key, got {}", algorithm, expected, key_size
            )));
        }
        
        Ok(())
    }
    
    /// slay Validate IV size for algorithm
    pub fn validate_iv_size(algorithm: &str, iv_size: usize) -> CryptoResult<()> {
        let expected = match algorithm {
            "AES-128-CBC" | "AES-192-CBC" | "AES-256-CBC" => 16,
            "AES-128-GCM" | "AES-192-GCM" | "AES-256-GCM" => 12,
            "ChaCha20" | "ChaCha20-Poly1305" => 12,
            _ => return Err(CryptoError::UnsupportedCipher(algorithm.to_string())),
        };
        
        if iv_size != expected {
            return Err(CryptoError::InvalidIvSize(format!(
                "{} requires {}-byte IV/nonce, got {}", algorithm, expected, iv_size
            )));
        }
        
        Ok(())
    }
    
    /// slay Validate data size constraints
    pub fn validate_data_size(data_size: usize, min_size: usize, max_size: Option<usize>) -> CryptoResult<()> {
        if data_size < min_size {
            return Err(CryptoError::InvalidDataSize(format!("Data size {} is below minimum {}", data_size, min_size)));
        }
        
        if let Some(max) = max_size {
            if data_size > max {
                return Err(CryptoError::InvalidDataSize(format!("Data size {} exceeds maximum {}", data_size, max)));
            }
        }
        
        Ok(())
    }
    
    /// slay Validate authentication tag size
    pub fn validate_tag_size(algorithm: &str, tag_size: usize) -> CryptoResult<()> {
        let expected = match algorithm {
            "AES-128-GCM" | "AES-192-GCM" | "AES-256-GCM" => 16,
            "ChaCha20-Poly1305" => 16,
            _ => return Err(CryptoError::UnsupportedCipher(algorithm.to_string())),
        };
        
        if tag_size != expected {
            return Err(CryptoError::InvalidDataSize(format!(
                "{} requires {}-byte authentication tag, got {}", algorithm, expected, tag_size
            )));
        }
        
        Ok(())
    }
}

/// slay Validate key size (convenience function)
pub fn validate_key_size(algorithm: &str, key_size: usize) -> CryptoResult<()> {
    CryptoValidator::validate_key_size(algorithm, key_size)
}

/// slay Validate IV size (convenience function)
pub fn validate_iv_size(algorithm: &str, iv_size: usize) -> CryptoResult<()> {
    CryptoValidator::validate_iv_size(algorithm, iv_size)
}

/// slay Validate data size (convenience function)
pub fn validate_data_size(data_size: usize, min_size: usize, max_size: Option<usize>) -> CryptoResult<()> {
    CryptoValidator::validate_data_size(data_size, min_size, max_size)
}

/// fr fr Crypto utilities collection
pub struct CryptoUtils;

impl CryptoUtils {
    /// slay Generate secure salt for key derivation
    pub fn generate_salt(size: usize) -> CryptoResult<Vec<u8>> {
        secure_random_bytes(size)
    }
    
    /// slay Convert bytes to hex string
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
    
    /// slay Convert hex string to bytes
    pub fn hex_to_bytes(hex: &str) -> CryptoResult<Vec<u8>> {
        if hex.len() % 2 != 0 {
            return Err(CryptoError::InvalidDataSize("Hex string must have even length".to_string()));
        }
        
        let mut bytes = Vec::new();
        for chunk in hex.as_bytes().chunks(2) {
            let hex_byte = std::str::from_utf8(chunk)
                .map_err(|_| CryptoError::InvalidDataSize("Invalid hex character".to_string()))?;
            let byte = u8::from_str_radix(hex_byte, 16)
                .map_err(|_| CryptoError::InvalidDataSize(format!("Invalid hex byte: {}", hex_byte)))?;
            bytes.push(byte);
        }
        
        Ok(bytes)
    }
    
    /// slay Encode bytes as base64
    pub fn bytes_to_base64(bytes: &[u8]) -> String {
        base64_encode(bytes)
    }
    
    /// slay Decode base64 to bytes
    pub fn base64_to_bytes(base64: &str) -> CryptoResult<Vec<u8>> {
        base64_decode(base64)
    }
    
    /// slay XOR two byte arrays
    pub fn xor_bytes(a: &[u8], b: &[u8]) -> CryptoResult<Vec<u8>> {
        if a.len() != b.len() {
            return Err(CryptoError::InvalidDataSize(format!("XOR operands must have same length: {} vs {}", a.len(), b.len())));
        }
        
        Ok(a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect())
    }
}

/// Simple base64 encoding (placeholder implementation)
fn base64_encode(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b = (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | (buf[2] as u32);
        
        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[((b >> 6) & 63) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(b & 63) as usize] as char } else { '=' });
    }
    
    result
}

/// Simple base64 decoding (placeholder implementation)
fn base64_decode(s: &str) -> CryptoResult<Vec<u8>> {
    // Simplified base64 decoder - in production use a proper library
    let mut result = Vec::new();
    let chars: Vec<char> = s.chars().filter(|&c| c != '=').collect();
    
    for chunk in chars.chunks(4) {
        if chunk.len() < 2 {
            break;
        }
        
        let mut values = [0u8; 4];
        for (i, &c) in chunk.iter().enumerate() {
            values[i] = match c {
                'A'..='Z' => (c as u8) - b'A',
                'a'..='z' => (c as u8) - b'a' + 26,
                '0'..='9' => (c as u8) - b'0' + 52,
                '+' => 62,
                '/' => 63,
                _ => return Err(CryptoError::InvalidDataSize(format!("Invalid base64 character: {}", c))),
            };
        }
        
        let combined = (values[0] as u32) << 18 | (values[1] as u32) << 12 | (values[2] as u32) << 6 | (values[3] as u32);
        
        result.push((combined >> 16) as u8);
        if chunk.len() > 2 {
            result.push((combined >> 8) as u8);
        }
        if chunk.len() > 3 {
            result.push(combined as u8);
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secure_random() {
        let rng = SecureRandom::new();
        assert!(rng.is_ok());
        
        let rng = rng.unwrap();
        let bytes = rng.generate_bytes(32);
        assert!(bytes.is_ok());
        assert_eq!(bytes.unwrap().len(), 32);
    }
    
    #[test]
    fn test_pkcs7_padding() {
        let data = b"Hello World";
        let padded = Pkcs7Padding::apply(data, 16).unwrap();
        assert_eq!(padded.len(), 16);
        
        let unpadded = Pkcs7Padding::remove(&padded).unwrap();
        assert_eq!(unpadded, data);
        
        assert!(Pkcs7Padding::validate(&padded));
    }
    
    #[test]
    fn test_iv_generator() {
        let mut generator = IvGenerator::new().unwrap();
        
        let iv1 = generator.generate_iv(16).unwrap();
        let iv2 = generator.generate_iv(16).unwrap();
        assert_ne!(iv1, iv2);
        assert_eq!(iv1.len(), 16);
        assert_eq!(iv2.len(), 16);
    }
    
    #[test]
    fn test_nonce_manager() {
        let mut manager = NonceManager::new("ChaCha20-Poly1305").unwrap();
        
        let nonce1 = manager.generate_unique_nonce(12).unwrap();
        let nonce2 = manager.generate_unique_nonce(12).unwrap();
        assert_ne!(nonce1, nonce2);
        
        assert!(manager.is_nonce_used(&nonce1));
        assert!(manager.is_nonce_used(&nonce2));
    }
    
    #[test]
    fn test_secure_operations() {
        let mut data1 = vec![1, 2, 3, 4];
        let data2 = vec![1, 2, 3, 4];
        
        assert!(SecureOps::secure_compare(&data1, &data2));
        
        SecureOps::secure_clear(&mut data1);
        assert_eq!(data1, vec![0, 0, 0, 0]);
    }
    
    #[test]
    fn test_validation() {
        assert!(CryptoValidator::validate_key_size("AES-256-CBC", 32).is_ok());
        assert!(CryptoValidator::validate_key_size("AES-256-CBC", 16).is_err());
        
        assert!(CryptoValidator::validate_iv_size("AES-256-CBC", 16).is_ok());
        assert!(CryptoValidator::validate_iv_size("AES-256-GCM", 12).is_ok());
        assert!(CryptoValidator::validate_iv_size("AES-256-CBC", 12).is_err());
    }
    
    #[test]
    fn test_crypto_utils() {
        let bytes = vec![0xde, 0xad, 0xbe, 0xef];
        let hex = CryptoUtils::bytes_to_hex(&bytes);
        assert_eq!(hex, "deadbeef");
        
        let decoded = CryptoUtils::hex_to_bytes(&hex).unwrap();
        assert_eq!(decoded, bytes);
        
        let base64 = CryptoUtils::bytes_to_base64(&bytes);
        let decoded_b64 = CryptoUtils::base64_to_bytes(&base64).unwrap();
        assert_eq!(decoded_b64, bytes);
    }
    
    #[test]
    fn test_convenience_functions() {
        let bytes = secure_random_bytes(16);
        assert!(bytes.is_ok());
        assert_eq!(bytes.unwrap().len(), 16);
        
        let key = generate_key("AES-256", 32);
        assert!(key.is_ok());
        assert_eq!(key.unwrap().size(), 32);
    }
}
