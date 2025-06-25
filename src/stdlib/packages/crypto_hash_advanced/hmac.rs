/// fr fr HMAC implementation with support for multiple hash functions
/// 
/// This module provides production-ready HMAC (Hash-based Message Authentication Code)
/// implementation following RFC 2104 with support for SHA-256, SHA-512, and BLAKE3.

use crate::error::CursedError;
// use crate::stdlib::value::Value;
use std::collections::HashMap;

// Import hash functions
// use crate::stdlib::crypto::hash::{Sha256, Sha512, HashFunction};
use super::blake3::Blake3Hasher;

/// fr fr Supported hash algorithms for HMAC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HmacAlgorithm {
impl HmacAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
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
        }
    }
    
    pub fn recommended_key_size(&self) -> usize {
        self.output_size() // Recommended key size equals output size
    }
}

/// fr fr HMAC implementation with generic hash function support
#[derive(Debug, Clone)]
pub struct HmacEngine {
impl HmacEngine {
    /// slay Create new HMAC engine with specified algorithm and key
    pub fn new(algorithm: HmacAlgorithm, key: &[u8]) -> crate::error::Result<()> {
        if key.is_empty() {
            return Err(HmacError::InvalidKey("HMAC key cannot be empty".to_string()));
        let block_size = algorithm.block_size();
        
        // Process key according to RFC 2104
        let processed_key = if key.len() > block_size {
            // If key is longer than block size, hash it
            match algorithm {
                HmacAlgorithm::Sha256 => {
                    let hash = Sha256::hash(key);
                    hash.to_vec()
                HmacAlgorithm::Sha512 => {
                    let hash = Sha512::hash(key);
                    hash.to_vec()
                HmacAlgorithm::Blake3 => {
                    let hash = Blake3Hasher::hash(key);
                    hash.to_vec()
            }
        } else {
            key.to_vec()
        
        // Pad key to block size
        let mut padded_key = processed_key;
        padded_key.resize(block_size, 0);
        
        // Create ipad and opad
        let mut ipad = vec![0x36u8; block_size];
        let mut opad = vec![0x5cu8; block_size];
        
        for i in 0..block_size {
            ipad[i] ^= padded_key[i];
            opad[i] ^= padded_key[i];
        Ok(Self {
        })
    /// bestie Compute HMAC for given message
    pub fn compute(&self, message: &[u8]) -> Vec<u8> {
        match self.algorithm {
        }
    }
    
    /// vibes Verify HMAC against expected value
    pub fn verify(&self, message: &[u8], expected_mac: &[u8]) -> bool {
        let computed_mac = self.compute(message);
        self.constant_time_compare(&computed_mac, expected_mac)
    /// periodt Create streaming HMAC computer
    pub fn create_stream(&self) -> HmacStream {
        HmacStream::new(self.algorithm, &self.ipad, &self.opad)
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
    /// facts Constant-time comparison to prevent timing attacks
    fn constant_time_compare(&self, a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
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
impl HmacStream {
    fn new(algorithm: HmacAlgorithm, ipad: &[u8], opad: &[u8]) -> Self {
        let inner_state: Box<dyn HashState> = match algorithm {
            HmacAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(ipad);
                Box::new(Sha256State { hasher })
            HmacAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(ipad);
                Box::new(Sha512State { hasher })
            HmacAlgorithm::Blake3 => {
                let mut hasher = Blake3Hasher::new();
                hasher.update(ipad);
                Box::new(Blake3State { hasher })
        
        Self {
        }
    }
    
    /// yolo Update HMAC with more message data
    pub fn update(&mut self, data: &[u8]) {
        self.inner_state.update(data);
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
            HmacAlgorithm::Sha512 => {
                let mut outer_hasher = Sha512::new();
                outer_hasher.update(&self.opad);
                outer_hasher.update(&inner_hash);
                outer_hasher.finalize().to_vec()
            HmacAlgorithm::Blake3 => {
                let mut outer_hasher = Blake3Hasher::new();
                outer_hasher.update(&self.opad);
                outer_hasher.update(&inner_hash);
                outer_hasher.finalize_fixed().to_vec()
        }
    }
/// fr fr Trait for hash state abstraction
trait HashState {
    fn update(&mut self, data: &[u8]);
    fn finalize(self: Box<Self>) -> Vec<u8>;
struct Sha256State {
impl HashState for Sha256State {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.hasher.finalize().to_vec()
    }
}

struct Sha512State {
impl HashState for Sha512State {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.hasher.finalize().to_vec()
    }
}

struct Blake3State {
impl HashState for Blake3State {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.hasher.finalize_fixed().to_vec()
    }
}

/// fr fr HMAC error types
#[derive(Debug, Clone, PartialEq)]
pub enum HmacError {
// impl std::fmt::Display for HmacError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             HmacError::InvalidKey(msg) => write!(f, "Invalid HMAC key: {}", msg),
//             HmacError::InvalidAlgorithm(msg) => write!(f, "Invalid HMAC algorithm: {}", msg),
//             HmacError::VerificationFailed => write!(f, "HMAC verification failed"),
//             HmacError::Internal(msg) => write!(f, "Internal HMAC error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for HmacError {}
// 
/// fr fr HMAC utilities
pub struct HmacUtils;

impl HmacUtils {
    /// bestie Convert MAC bytes to hex string
    pub fn to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    /// vibes Generate cryptographically secure HMAC key
    pub fn generate_key(algorithm: HmacAlgorithm) -> Vec<u8> {
        use rand::RngCore;
        let key_size = algorithm.recommended_key_size();
        let mut key = vec![0u8; key_size];
        rand::thread_rng().fill_bytes(&mut key);
        key
    /// periodt HMAC-SHA256 in one shot
    pub fn hmac_sha256(key: &[u8], message: &[u8]) -> crate::error::Result<()> {
        let engine = HmacEngine::new(HmacAlgorithm::Sha256, key)?;
        Ok(engine.compute(message))
    /// facts HMAC-SHA512 in one shot
    pub fn hmac_sha512(key: &[u8], message: &[u8]) -> crate::error::Result<()> {
        let engine = HmacEngine::new(HmacAlgorithm::Sha512, key)?;
        Ok(engine.compute(message))
    /// yolo HMAC-BLAKE3 in one shot
    pub fn hmac_blake3(key: &[u8], message: &[u8]) -> crate::error::Result<()> {
        let engine = HmacEngine::new(HmacAlgorithm::Blake3, key)?;
        Ok(engine.compute(message))
    /// slay HMAC string with SHA-256
    pub fn hmac_sha256_string(key: &str, message: &str) -> crate::error::Result<()> {
        let mac = Self::hmac_sha256(key.as_bytes(), message.as_bytes())?;
        Ok(Self::to_hex(&mac))
    /// bestie HMAC string with SHA-512
    pub fn hmac_sha512_string(key: &str, message: &str) -> crate::error::Result<()> {
        let mac = Self::hmac_sha512(key.as_bytes(), message.as_bytes())?;
        Ok(Self::to_hex(&mac))
    /// vibes HMAC string with BLAKE3
    pub fn hmac_blake3_string(key: &str, message: &str) -> crate::error::Result<()> {
        let mac = Self::hmac_blake3(key.as_bytes(), message.as_bytes())?;
        Ok(Self::to_hex(&mac))
    }
}

/// fr fr Public API functions for CURSED integration

/// slay HMAC-SHA256 function
pub fn hmac_sha256(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("HMAC-SHA256 requires key and message".to_string()));
    let key = match &args[0] {
    
    let message = match &args[1] {
    
    match HmacUtils::hmac_sha256(key, message) {
    }
}

/// slay HMAC-SHA512 function
pub fn hmac_sha512(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("HMAC-SHA512 requires key and message".to_string()));
    let key = match &args[0] {
    
    let message = match &args[1] {
    
    match HmacUtils::hmac_sha512(key, message) {
    }
}

/// slay HMAC-BLAKE3 function
pub fn hmac_blake3(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("HMAC-BLAKE3 requires key and message".to_string()));
    let key = match &args[0] {
    
    let message = match &args[1] {
    
    match HmacUtils::hmac_blake3(key, message) {
    }
}

/// slay Verify HMAC
pub fn hmac_verify(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 4 {
        return Err(CursedError::Runtime("HMAC verify requires algorithm, key, message, and expected MAC".to_string()));
    let algorithm_str = match &args[0] {
    
    let algorithm = match algorithm_str {
    
    let key = match &args[1] {
    
    let message = match &args[2] {
    
    let expected_mac_hex = match &args[3] {
    
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
    match HmacEngine::new(algorithm, key) {
    }
}

/// slay Generate HMAC key
pub fn hmac_generate_key(args: Vec<Value>) -> crate::error::Result<()> {
    let algorithm_str = if args.is_empty() {
        "SHA256"
    } else {
        match &args[0] {
        }
    
    let algorithm = match algorithm_str {
    
    let key = HmacUtils::generate_key(algorithm);
    Ok(Value::String(HmacUtils::to_hex(&key)))
/// slay Create streaming HMAC computer
pub fn create_hmac_stream(args: Vec<Value>) -> crate::error::Result<()> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("HMAC stream requires algorithm and key".to_string()));
    let algorithm_str = match &args[0] {
    
    let algorithm = match algorithm_str {
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(algorithm.name().to_string()));
            std::ptr::addr_of!(*self) as usize)));
    result.insert("output_size".to_string(), Value::Number(algorithm.output_size() as f64));
    
    Ok(Value::Object(result))
