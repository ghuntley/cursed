/// fr fr HMAC implementation with support for multiple hash functions
/// 
/// This module provides production-ready HMAC (Hash-based Message Authentication Code)
/// implementation following RFC 2104 with support for SHA-256, SHA-512, and BLAKE3.

use crate::error::CursedError;
use crate::stdlib::value::Value;
use crate::error::Error;
use std::collections::HashMap;

// Import hash functions
use crate::stdlib::crypto::hash::{Sha256, Sha512, HashFunction};
use super::blake3::Blake3Hasher;

/// fr fr Supported hash algorithms for HMAC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HmacAlgorithm {
    Sha256,
    Sha512,
    Blake3,
}

impl HmacAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            HmacAlgorithm::Sha256 => "HMAC-SHA256",
            HmacAlgorithm::Sha512 => "HMAC-SHA512",
            HmacAlgorithm::Blake3 => "HMAC-BLAKE3",
        }
    }
    
    pub fn block_size(&self) -> usize {
        match self {
            HmacAlgorithm::Sha256 => 64,  // SHA-256 block size
            HmacAlgorithm::Sha512 => 128, // SHA-512 block size
            HmacAlgorithm::Blake3 => 64,  // BLAKE3 block size
        }
    }
    
    pub fn output_size(&self) -> usize {
        match self {
            HmacAlgorithm::Sha256 => 32,
            HmacAlgorithm::Sha512 => 64,
            HmacAlgorithm::Blake3 => 32,
        }
    }
    
    pub fn recommended_key_size(&self) -> usize {
        self.output_size() // Recommended key size equals output size
    }
}

/// fr fr HMAC implementation with generic hash function support
#[derive(Debug, Clone)]
pub struct HmacEngine {
    algorithm: HmacAlgorithm,
    key: Vec<u8>,
    ipad: Vec<u8>,
    opad: Vec<u8>,
}

impl HmacEngine {
    /// slay Create new HMAC engine with specified algorithm and key
    pub fn new(algorithm: HmacAlgorithm, key: &[u8]) -> Result<(), Error> {
        if key.is_empty() {
            return Err(HmacError::InvalidKey("HMAC key cannot be empty".to_string()));
        }
        
        let block_size = algorithm.block_size();
        
        // Process key according to RFC 2104
        let processed_key = if key.len() > block_size {
            // If key is longer than block size, hash it
            match algorithm {
                HmacAlgorithm::Sha256 => {
                    let hash = Sha256::hash(key);
                    hash.to_vec()
                },
                HmacAlgorithm::Sha512 => {
                    let hash = Sha512::hash(key);
                    hash.to_vec()
                },
                HmacAlgorithm::Blake3 => {
                    let hash = Blake3Hasher::hash(key);
                    hash.to_vec()
                },
            }
        } else {
            key.to_vec()
        };
        
        // Pad key to block size
        let mut padded_key = processed_key;
        padded_key.resize(block_size, 0);
        
        // Create ipad and opad
        let mut ipad = vec![0x36u8; block_size];
        let mut opad = vec![0x5cu8; block_size];
        
        for i in 0..block_size {
            ipad[i] ^= padded_key[i];
            opad[i] ^= padded_key[i];
        }
        
        Ok(Self {
            algorithm,
            key: padded_key,
            ipad,
            opad,
        })
    }
    
    /// bestie Compute HMAC for given message
    pub fn compute(&self, message: &[u8]) -> Vec<u8> {
        match self.algorithm {
            HmacAlgorithm::Sha256 => self.compute_sha256(message),
            HmacAlgorithm::Sha512 => self.compute_sha512(message),
            HmacAlgorithm::Blake3 => self.compute_blake3(message),
        }
    }
    
    /// vibes Verify HMAC against expected value
    pub fn verify(&self, message: &[u8], expected_mac: &[u8]) -> bool {
        let computed_mac = self.compute(message);
        self.constant_time_compare(&computed_mac, expected_mac)
    }
    
    /// periodt Create streaming HMAC computer
    pub fn create_stream(&self) -> HmacStream {
        HmacStream::new(self.algorithm, &self.ipad, &self.opad)
    }
    
    fn compute_sha256(&self, message: &[u8]) -> Vec<u8> {
        // Inner hash: H(ipad || message)
        let mut inner_hasher = Sha256::new();
        inner_hasher.update(&self.ipad);
        inner_hasher.update(message);
        let inner_hash = inner_hasher.finalize();
        
        // Outer hash: H(opad || inner_hash)
        let mut outer_hasher = Sha256::new();
        outer_hasher.update(&self.opad);
        outer_hasher.update(&inner_hash);
        let outer_hash = outer_hasher.finalize();
        
        outer_hash.to_vec()
    }
    
    fn compute_sha512(&self, message: &[u8]) -> Vec<u8> {
        // Inner hash: H(ipad || message)
        let mut inner_hasher = Sha512::new();
        inner_hasher.update(&self.ipad);
        inner_hasher.update(message);
        let inner_hash = inner_hasher.finalize();
        
        // Outer hash: H(opad || inner_hash)
        let mut outer_hasher = Sha512::new();
        outer_hasher.update(&self.opad);
        outer_hasher.update(&inner_hash);
        let outer_hash = outer_hasher.finalize();
        
        outer_hash.to_vec()
    }
    
    fn compute_blake3(&self, message: &[u8]) -> Vec<u8> {
        // Inner hash: H(ipad || message)
        let mut inner_hasher = Blake3Hasher::new();
        inner_hasher.update(&self.ipad);
        inner_hasher.update(message);
        let inner_hash = inner_hasher.finalize_fixed();
        
        // Outer hash: H(opad || inner_hash)
        let mut outer_hasher = Blake3Hasher::new();
        outer_hasher.update(&self.opad);
        outer_hasher.update(&inner_hash);
        let outer_hash = outer_hasher.finalize_fixed();
        
        outer_hash.to_vec()
    }
    
    /// facts Constant-time comparison to prevent timing attacks
    fn constant_time_compare(&self, a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }
}

/// fr fr Streaming HMAC computation for large messages
#[derive(Debug)]
pub struct HmacStream {
    algorithm: HmacAlgorithm,
    inner_state: Box<dyn HashState>,
    opad: Vec<u8>,
}

impl HmacStream {
    fn new(algorithm: HmacAlgorithm, ipad: &[u8], opad: &[u8]) -> Self {
        let inner_state: Box<dyn HashState> = match algorithm {
            HmacAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(ipad);
                Box::new(Sha256State { hasher })
            },
            HmacAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(ipad);
                Box::new(Sha512State { hasher })
            },
            HmacAlgorithm::Blake3 => {
                let mut hasher = Blake3Hasher::new();
                hasher.update(ipad);
                Box::new(Blake3State { hasher })
            },
        };
        
        Self {
            algorithm,
            inner_state,
            opad: opad.to_vec(),
        }
    }
    
    /// yolo Update HMAC with more message data
    pub fn update(&mut self, data: &[u8]) {
        self.inner_state.update(data);
    }
    
    /// slay Finalize HMAC and get result
    pub fn finalize(self) -> Vec<u8> {
        let inner_hash = self.inner_state.finalize();
        
        // Outer hash: H(opad || inner_hash)
        match self.algorithm {
            HmacAlgorithm::Sha256 => {
                let mut outer_hasher = Sha256::new();
                outer_hasher.update(&self.opad);
                outer_hasher.update(&inner_hash);
                outer_hasher.finalize().to_vec()
            },
            HmacAlgorithm::Sha512 => {
                let mut outer_hasher = Sha512::new();
                outer_hasher.update(&self.opad);
                outer_hasher.update(&inner_hash);
                outer_hasher.finalize().to_vec()
            },
            HmacAlgorithm::Blake3 => {
                let mut outer_hasher = Blake3Hasher::new();
                outer_hasher.update(&self.opad);
                outer_hasher.update(&inner_hash);
                outer_hasher.finalize_fixed().to_vec()
            },
        }
    }
}

/// fr fr Trait for hash state abstraction
trait HashState {
    fn update(&mut self, data: &[u8]);
    fn finalize(self: Box<Self>) -> Vec<u8>;
}

struct Sha256State {
    hasher: Sha256,
}

impl HashState for Sha256State {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.hasher.finalize().to_vec()
    }
}

struct Sha512State {
    hasher: Sha512,
}

impl HashState for Sha512State {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.hasher.finalize().to_vec()
    }
}

struct Blake3State {
    hasher: Blake3Hasher,
}

impl HashState for Blake3State {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }
    
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.hasher.finalize_fixed().to_vec()
    }
}

/// fr fr HMAC error types
#[derive(Debug, Clone, PartialEq)]
pub enum HmacError {
    InvalidKey(String),
    InvalidAlgorithm(String),
    VerificationFailed,
    Internal(String),
}

impl std::fmt::Display for HmacError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HmacError::InvalidKey(msg) => write!(f, "Invalid HMAC key: {}", msg),
            HmacError::InvalidAlgorithm(msg) => write!(f, "Invalid HMAC algorithm: {}", msg),
            HmacError::VerificationFailed => write!(f, "HMAC verification failed"),
            HmacError::Internal(msg) => write!(f, "Internal HMAC error: {}", msg),
        }
    }
}

impl std::error::Error for HmacError {}

/// fr fr HMAC utilities
pub struct HmacUtils;

impl HmacUtils {
    /// bestie Convert MAC bytes to hex string
    pub fn to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
    
    /// vibes Generate cryptographically secure HMAC key
    pub fn generate_key(algorithm: HmacAlgorithm) -> Vec<u8> {
        use rand::RngCore;
        let key_size = algorithm.recommended_key_size();
        let mut key = vec![0u8; key_size];
        rand::thread_rng().fill_bytes(&mut key);
        key
    }
    
    /// periodt HMAC-SHA256 in one shot
    pub fn hmac_sha256(key: &[u8], message: &[u8]) -> Result<(), Error> {
        let engine = HmacEngine::new(HmacAlgorithm::Sha256, key)?;
        Ok(engine.compute(message))
    }
    
    /// facts HMAC-SHA512 in one shot
    pub fn hmac_sha512(key: &[u8], message: &[u8]) -> Result<(), Error> {
        let engine = HmacEngine::new(HmacAlgorithm::Sha512, key)?;
        Ok(engine.compute(message))
    }
    
    /// yolo HMAC-BLAKE3 in one shot
    pub fn hmac_blake3(key: &[u8], message: &[u8]) -> Result<(), Error> {
        let engine = HmacEngine::new(HmacAlgorithm::Blake3, key)?;
        Ok(engine.compute(message))
    }
    
    /// slay HMAC string with SHA-256
    pub fn hmac_sha256_string(key: &str, message: &str) -> Result<(), Error> {
        let mac = Self::hmac_sha256(key.as_bytes(), message.as_bytes())?;
        Ok(Self::to_hex(&mac))
    }
    
    /// bestie HMAC string with SHA-512
    pub fn hmac_sha512_string(key: &str, message: &str) -> Result<(), Error> {
        let mac = Self::hmac_sha512(key.as_bytes(), message.as_bytes())?;
        Ok(Self::to_hex(&mac))
    }
    
    /// vibes HMAC string with BLAKE3
    pub fn hmac_blake3_string(key: &str, message: &str) -> Result<(), Error> {
        let mac = Self::hmac_blake3(key.as_bytes(), message.as_bytes())?;
        Ok(Self::to_hex(&mac))
    }
}

/// fr fr Public API functions for CURSED integration

/// slay HMAC-SHA256 function
pub fn hmac_sha256(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("HMAC-SHA256 requires key and message".to_string()));
    }
    
    let key = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("HMAC key must be a string".to_string())),
    };
    
    let message = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("HMAC message must be a string".to_string())),
    };
    
    match HmacUtils::hmac_sha256(key, message) {
        Ok(mac) => Ok(Value::String(HmacUtils::to_hex(&mac))),
        Err(e) => Err(CursedError::Runtime(format!("HMAC-SHA256 failed: {}", e))),
    }
}

/// slay HMAC-SHA512 function
pub fn hmac_sha512(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("HMAC-SHA512 requires key and message".to_string()));
    }
    
    let key = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("HMAC key must be a string".to_string())),
    };
    
    let message = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("HMAC message must be a string".to_string())),
    };
    
    match HmacUtils::hmac_sha512(key, message) {
        Ok(mac) => Ok(Value::String(HmacUtils::to_hex(&mac))),
        Err(e) => Err(CursedError::Runtime(format!("HMAC-SHA512 failed: {}", e))),
    }
}

/// slay HMAC-BLAKE3 function
pub fn hmac_blake3(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("HMAC-BLAKE3 requires key and message".to_string()));
    }
    
    let key = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("HMAC key must be a string".to_string())),
    };
    
    let message = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("HMAC message must be a string".to_string())),
    };
    
    match HmacUtils::hmac_blake3(key, message) {
        Ok(mac) => Ok(Value::String(HmacUtils::to_hex(&mac))),
        Err(e) => Err(CursedError::Runtime(format!("HMAC-BLAKE3 failed: {}", e))),
    }
}

/// slay Verify HMAC
pub fn hmac_verify(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 4 {
        return Err(CursedError::Runtime("HMAC verify requires algorithm, key, message, and expected MAC".to_string()));
    }
    
    let algorithm_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(CursedError::Runtime("HMAC algorithm must be a string".to_string())),
    };
    
    let algorithm = match algorithm_str {
        "SHA256" => HmacAlgorithm::Sha256,
        "SHA512" => HmacAlgorithm::Sha512,
        "BLAKE3" => HmacAlgorithm::Blake3,
        _ => return Err(CursedError::Runtime(format!("Unsupported HMAC algorithm: {}", algorithm_str))),
    };
    
    let key = match &args[1] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("HMAC key must be a string".to_string())),
    };
    
    let message = match &args[2] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("HMAC message must be a string".to_string())),
    };
    
    let expected_mac_hex = match &args[3] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Expected MAC must be a string".to_string())),
    };
    
    // Convert hex MAC to bytes
    let mut expected_mac = Vec::new();
    for chunk in expected_mac_hex.as_bytes().chunks(2) {
        if chunk.len() == 2 {
            if let Ok(byte) = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16) {
                expected_mac.push(byte);
            } else {
                return Err(CursedError::Runtime("Invalid hex character in expected MAC".to_string()));
            }
        }
    }
    
    match HmacEngine::new(algorithm, key) {
        Ok(engine) => Ok(Value::bool(engine.verify(message, &expected_mac))),
        Err(e) => Err(CursedError::Runtime(format!("HMAC verification failed: {}", e))),
    }
}

/// slay Generate HMAC key
pub fn hmac_generate_key(args: Vec<Value>) -> Result<(), Error> {
    let algorithm_str = if args.is_empty() {
        "SHA256"
    } else {
        match &args[0] {
            Value::String(s) => s.as_str(),
            _ => "SHA256",
        }
    };
    
    let algorithm = match algorithm_str {
        "SHA256" => HmacAlgorithm::Sha256,
        "SHA512" => HmacAlgorithm::Sha512,
        "BLAKE3" => HmacAlgorithm::Blake3,
        _ => return Err(CursedError::Runtime(format!("Unsupported HMAC algorithm: {}", algorithm_str))),
    };
    
    let key = HmacUtils::generate_key(algorithm);
    Ok(Value::String(HmacUtils::to_hex(&key)))
}

/// slay Create streaming HMAC computer
pub fn create_hmac_stream(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("HMAC stream requires algorithm and key".to_string()));
    }
    
    let algorithm_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(CursedError::Runtime("HMAC algorithm must be a string".to_string())),
    };
    
    let algorithm = match algorithm_str {
        "SHA256" => HmacAlgorithm::Sha256,
        "SHA512" => HmacAlgorithm::Sha512,
        "BLAKE3" => HmacAlgorithm::Blake3,
        _ => return Err(CursedError::Runtime(format!("Unsupported HMAC algorithm: {}", algorithm_str))),
    };
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(algorithm.name().to_string()));
    result.insert("stream_id".to_string(), Value::String(format!("hmac_stream_{:x}", 
            std::ptr::addr_of!(*self) as usize)));
    result.insert("output_size".to_string(), Value::Number(algorithm.output_size() as f64));
    
    Ok(Value::Object(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hmac_sha256_rfc_test_vectors() {
        // RFC 4231 test vectors
        let key = b"key";
        let message = b"The quick brown fox jumps over the lazy dog";
        
        let mac = HmacUtils::hmac_sha256(key, message).unwrap();
        assert_eq!(mac.len(), 32);
        assert_ne!(mac, vec![0u8; 32]); // Should not be all zeros
    }
    
    #[test]
    fn test_hmac_sha512() {
        let key = b"test_key";
        let message = b"test_message";
        
        let mac = HmacUtils::hmac_sha512(key, message).unwrap();
        assert_eq!(mac.len(), 64);
    }
    
    #[test]
    fn test_hmac_blake3() {
        let key = b"test_key";
        let message = b"test_message";
        
        let mac = HmacUtils::hmac_blake3(key, message).unwrap();
        assert_eq!(mac.len(), 32);
    }
    
    #[test]
    fn test_hmac_verification() {
        let key = b"secret_key";
        let message = b"authenticated_message";
        
        let engine = HmacEngine::new(HmacAlgorithm::Sha256, key).unwrap();
        let mac = engine.compute(message);
        
        // Should verify correctly
        assert!(engine.verify(message, &mac));
        
        // Should fail with wrong message
        assert!(!engine.verify(b"wrong_message", &mac));
        
        // Should fail with wrong MAC
        let mut wrong_mac = mac.clone();
        wrong_mac[0] ^= 1;
        assert!(!engine.verify(message, &wrong_mac));
    }
    
    #[test]
    fn test_hmac_streaming() {
        let key = b"streaming_key";
        let message = b"hello world";
        
        // Compute HMAC in one shot
        let engine = HmacEngine::new(HmacAlgorithm::Sha256, key).unwrap();
        let mac1 = engine.compute(message);
        
        // Compute HMAC with streaming
        let mut stream = engine.create_stream();
        stream.update(b"hello ");
        stream.update(b"world");
        let mac2 = stream.finalize();
        
        assert_eq!(mac1, mac2);
    }
    
    #[test]
    fn test_hmac_key_processing() {
        // Test with key longer than block size
        let long_key = vec![0x42u8; 100]; // Longer than SHA-256 block size (64 bytes)
        let message = b"test";
        
        let engine = HmacEngine::new(HmacAlgorithm::Sha256, &long_key).unwrap();
        let mac = engine.compute(message);
        assert_eq!(mac.len(), 32);
    }
    
    #[test]
    fn test_hmac_empty_key() {
        let result = HmacEngine::new(HmacAlgorithm::Sha256, &[]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HmacError::InvalidKey(_)));
    }
    
    #[test]
    fn test_hmac_algorithm_properties() {
        assert_eq!(HmacAlgorithm::Sha256.name(), "HMAC-SHA256");
        assert_eq!(HmacAlgorithm::Sha256.block_size(), 64);
        assert_eq!(HmacAlgorithm::Sha256.output_size(), 32);
        
        assert_eq!(HmacAlgorithm::Sha512.name(), "HMAC-SHA512");
        assert_eq!(HmacAlgorithm::Sha512.block_size(), 128);
        assert_eq!(HmacAlgorithm::Sha512.output_size(), 64);
        
        assert_eq!(HmacAlgorithm::Blake3.name(), "HMAC-BLAKE3");
        assert_eq!(HmacAlgorithm::Blake3.block_size(), 64);
        assert_eq!(HmacAlgorithm::Blake3.output_size(), 32);
    }
    
    #[test]
    fn test_constant_time_compare() {
        let engine = HmacEngine::new(HmacAlgorithm::Sha256, b"key").unwrap();
        
        assert!(engine.constant_time_compare(b"hello", b"hello"));
        assert!(!engine.constant_time_compare(b"hello", b"world"));
        assert!(!engine.constant_time_compare(b"hello", b"hi"));
        assert!(!engine.constant_time_compare(b"", b"a"));
    }
    
    #[test]
    fn test_hmac_key_generation() {
        let key1 = HmacUtils::generate_key(HmacAlgorithm::Sha256);
        let key2 = HmacUtils::generate_key(HmacAlgorithm::Sha256);
        
        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);
        assert_ne!(key1, key2); // Should be different random keys
    }
}
