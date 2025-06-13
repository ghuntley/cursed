/// fr fr PBKDF2 implementation with multiple hash function support
/// 
/// This module provides production-ready PBKDF2 (Password-Based Key Derivation Function 2)
/// following RFC 2898 with support for SHA-256, SHA-512, and customizable iteration counts.

use crate::error::CursedError;
use crate::stdlib::value::Value;
use std::collections::HashMap;

// Import HMAC for PBKDF2 implementation
use super::super::crypto_hash_advanced::hmac::{HmacEngine, HmacAlgorithm, HmacError};

/// fr fr PBKDF2 parameters and configuration
#[derive(Debug, Clone)]
pub struct Pbkdf2Config {
    pub hash_algorithm: HmacAlgorithm,
    pub iterations: u32,
    pub salt_len: usize,
    pub output_len: usize,
}

impl Pbkdf2Config {
    /// slay Create PBKDF2 config with secure defaults
    pub fn new() -> Self {
        Self {
            hash_algorithm: HmacAlgorithm::Sha256,
            iterations: 100_000, // OWASP recommended minimum
            salt_len: 16,        // 128-bit salt
            output_len: 32,      // 256-bit derived key
        }
    }
    
    /// bestie Create PBKDF2 config with custom parameters
    pub fn with_params(hash_algorithm: HmacAlgorithm, iterations: u32, output_len: usize) -> Self {
        Self {
            hash_algorithm,
            iterations,
            salt_len: 16,
            output_len,
        }
    }
    
    /// vibes Validate configuration parameters
    pub fn validate(&self) -> Result<(), Pbkdf2Error> {
        if self.iterations < 1000 {
            return Err(Pbkdf2Error::InvalidConfig("PBKDF2 iterations must be at least 1000 for security".to_string()));
        }
        
        if self.salt_len < 8 {
            return Err(Pbkdf2Error::InvalidConfig("Salt length must be at least 8 bytes".to_string()));
        }
        
        if self.output_len == 0 || self.output_len > 1024 {
            return Err(Pbkdf2Error::InvalidConfig("Output length must be between 1 and 1024 bytes".to_string()));
        }
        
        Ok(())
    }
    
    /// periodt Get recommended iteration count for given security level
    pub fn recommended_iterations(security_level: SecurityLevel) -> u32 {
        match security_level {
            SecurityLevel::Fast => 10_000,      // Fast but less secure
            SecurityLevel::Standard => 100_000,  // OWASP recommended
            SecurityLevel::High => 500_000,      // High security
            SecurityLevel::Maximum => 1_000_000, // Maximum security
        }
    }
}

impl Default for Pbkdf2Config {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Security levels for PBKDF2 iteration counts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Fast,      // 10K iterations - for testing/development
    Standard,  // 100K iterations - production default
    High,      // 500K iterations - high security applications
    Maximum,   // 1M iterations - maximum security
}

impl SecurityLevel {
    pub fn name(&self) -> &'static str {
        match self {
            SecurityLevel::Fast => "Fast",
            SecurityLevel::Standard => "Standard",
            SecurityLevel::High => "High",
            SecurityLevel::Maximum => "Maximum",
        }
    }
}

/// fr fr PBKDF2 key derivation engine
#[derive(Debug)]
pub struct Pbkdf2Engine {
    config: Pbkdf2Config,
}

impl Pbkdf2Engine {
    /// slay Create new PBKDF2 engine with configuration
    pub fn new(config: Pbkdf2Config) -> Result<Self, Pbkdf2Error> {
        config.validate()?;
        Ok(Self { config })
    }
    
    /// bestie Create PBKDF2 engine with default configuration
    pub fn default() -> Result<Self, Pbkdf2Error> {
        Self::new(Pbkdf2Config::new())
    }
    
    /// vibes Derive key from password and salt
    pub fn derive_key(&self, password: &[u8], salt: &[u8]) -> Result<Vec<u8>, Pbkdf2Error> {
        if password.is_empty() {
            return Err(Pbkdf2Error::InvalidInput("Password cannot be empty".to_string()));
        }
        
        if salt.len() < 8 {
            return Err(Pbkdf2Error::InvalidInput("Salt must be at least 8 bytes".to_string()));
        }
        
        self.pbkdf2_derive(password, salt, self.config.iterations, self.config.output_len)
    }
    
    /// periodt Derive key with custom parameters
    pub fn derive_key_custom(&self, password: &[u8], salt: &[u8], iterations: u32, output_len: usize) -> Result<Vec<u8>, Pbkdf2Error> {
        if password.is_empty() {
            return Err(Pbkdf2Error::InvalidInput("Password cannot be empty".to_string()));
        }
        
        if salt.len() < 8 {
            return Err(Pbkdf2Error::InvalidInput("Salt must be at least 8 bytes".to_string()));
        }
        
        if iterations < 1000 {
            return Err(Pbkdf2Error::InvalidInput("Iterations must be at least 1000".to_string()));
        }
        
        if output_len == 0 || output_len > 1024 {
            return Err(Pbkdf2Error::InvalidInput("Output length must be between 1 and 1024 bytes".to_string()));
        }
        
        self.pbkdf2_derive(password, salt, iterations, output_len)
    }
    
    /// facts Verify password against derived key
    pub fn verify_password(&self, password: &[u8], salt: &[u8], expected_key: &[u8]) -> Result<bool, Pbkdf2Error> {
        let derived_key = self.derive_key(password, salt)?;
        Ok(self.constant_time_compare(&derived_key, expected_key))
    }
    
    /// yolo Hash password with random salt (for storage)
    pub fn hash_password(&self, password: &[u8]) -> Result<Pbkdf2Result, Pbkdf2Error> {
        let salt = self.generate_salt()?;
        let derived_key = self.derive_key(password, &salt)?;
        
        Ok(Pbkdf2Result {
            algorithm: self.config.hash_algorithm,
            iterations: self.config.iterations,
            salt,
            derived_key,
        })
    }
    
    /// slay Generate cryptographically secure salt
    pub fn generate_salt(&self) -> Result<Vec<u8>, Pbkdf2Error> {
        use rand::RngCore;
        let mut salt = vec![0u8; self.config.salt_len];
        rand::thread_rng().fill_bytes(&mut salt);
        Ok(salt)
    }
    
    // Internal PBKDF2 implementation following RFC 2898
    fn pbkdf2_derive(&self, password: &[u8], salt: &[u8], iterations: u32, output_len: usize) -> Result<Vec<u8>, Pbkdf2Error> {
        let hmac_engine = HmacEngine::new(self.config.hash_algorithm, password)
            .map_err(|e| Pbkdf2Error::HmacError(e))?;
        
        let hash_len = self.config.hash_algorithm.output_size();
        let blocks_needed = (output_len + hash_len - 1) / hash_len;
        let mut derived_key = Vec::with_capacity(output_len);
        
        for i in 1..=blocks_needed {
            let block = self.pbkdf2_f(&hmac_engine, salt, iterations, i)?;
            derived_key.extend_from_slice(&block);
        }
        
        // Truncate to requested length
        derived_key.truncate(output_len);
        Ok(derived_key)
    }
    
    // PBKDF2 F function: U1 = PRF(P, S || INT(i)), U2 = PRF(P, U1), ..., Uc = PRF(P, Uc-1)
    // F(P, S, c, i) = U1 XOR U2 XOR ... XOR Uc
    fn pbkdf2_f(&self, hmac_engine: &HmacEngine, salt: &[u8], iterations: u32, block_index: u32) -> Result<Vec<u8>, Pbkdf2Error> {
        // Prepare salt || INT(block_index)
        let mut salt_with_index = salt.to_vec();
        salt_with_index.extend_from_slice(&block_index.to_be_bytes());
        
        // U1 = PRF(P, S || INT(i))
        let mut u = hmac_engine.compute(&salt_with_index);
        let mut result = u.clone();
        
        // U2 = PRF(P, U1), U3 = PRF(P, U2), ..., Uc = PRF(P, Uc-1)
        for _ in 1..iterations {
            u = hmac_engine.compute(&u);
            
            // XOR with result
            for (r, &u_byte) in result.iter_mut().zip(u.iter()) {
                *r ^= u_byte;
            }
        }
        
        Ok(result)
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

/// fr fr PBKDF2 result containing all derivation parameters
#[derive(Debug, Clone)]
pub struct Pbkdf2Result {
    pub algorithm: HmacAlgorithm,
    pub iterations: u32,
    pub salt: Vec<u8>,
    pub derived_key: Vec<u8>,
}

impl Pbkdf2Result {
    /// bestie Encode result as string for storage (simplified format)
    pub fn to_string(&self) -> String {
        format!(
            "pbkdf2:{}:{}:{}:{}",
            self.algorithm.name(),
            self.iterations,
            hex::encode(&self.salt),
            hex::encode(&self.derived_key)
        )
    }
    
    /// vibes Parse result from string format
    pub fn from_string(s: &str) -> Result<Self, Pbkdf2Error> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 5 || parts[0] != "pbkdf2" {
            return Err(Pbkdf2Error::InvalidFormat("Invalid PBKDF2 string format".to_string()));
        }
        
        let algorithm = match parts[1] {
            "HMAC-SHA256" => HmacAlgorithm::Sha256,
            "HMAC-SHA512" => HmacAlgorithm::Sha512,
            "HMAC-BLAKE3" => HmacAlgorithm::Blake3,
            _ => return Err(Pbkdf2Error::InvalidFormat("Unknown hash algorithm".to_string())),
        };
        
        let iterations = parts[2].parse::<u32>()
            .map_err(|_| Pbkdf2Error::InvalidFormat("Invalid iteration count".to_string()))?;
        
        let salt = hex::decode(parts[3])
            .map_err(|_| Pbkdf2Error::InvalidFormat("Invalid salt hex encoding".to_string()))?;
        
        let derived_key = hex::decode(parts[4])
            .map_err(|_| Pbkdf2Error::InvalidFormat("Invalid derived key hex encoding".to_string()))?;
        
        Ok(Self {
            algorithm,
            iterations,
            salt,
            derived_key,
        })
    }
    
    /// periodt Verify password against this result
    pub fn verify(&self, password: &[u8]) -> Result<bool, Pbkdf2Error> {
        let config = Pbkdf2Config {
            hash_algorithm: self.algorithm,
            iterations: self.iterations,
            salt_len: self.salt.len(),
            output_len: self.derived_key.len(),
        };
        
        let engine = Pbkdf2Engine::new(config)?;
        engine.verify_password(password, &self.salt, &self.derived_key)
    }
}

/// fr fr PBKDF2 error types
#[derive(Debug, Clone, PartialEq)]
pub enum Pbkdf2Error {
    InvalidConfig(String),
    InvalidInput(String),
    InvalidFormat(String),
    HmacError(HmacError),
    InsufficientEntropy,
    Internal(String),
}

impl std::fmt::Display for Pbkdf2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pbkdf2Error::InvalidConfig(msg) => write!(f, "Invalid PBKDF2 configuration: {}", msg),
            Pbkdf2Error::InvalidInput(msg) => write!(f, "Invalid PBKDF2 input: {}", msg),
            Pbkdf2Error::InvalidFormat(msg) => write!(f, "Invalid PBKDF2 format: {}", msg),
            Pbkdf2Error::HmacError(e) => write!(f, "HMAC error in PBKDF2: {}", e),
            Pbkdf2Error::InsufficientEntropy => write!(f, "Insufficient entropy for PBKDF2 salt generation"),
            Pbkdf2Error::Internal(msg) => write!(f, "Internal PBKDF2 error: {}", msg),
        }
    }
}

impl std::error::Error for Pbkdf2Error {}

/// fr fr PBKDF2 utilities
pub struct Pbkdf2Utils;

impl Pbkdf2Utils {
    /// bestie Convert bytes to hex string
    pub fn to_hex(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }
    
    /// vibes Hash password with PBKDF2-SHA256 and default settings
    pub fn hash_password_sha256(password: &str) -> Result<String, Pbkdf2Error> {
        let engine = Pbkdf2Engine::default()?;
        let result = engine.hash_password(password.as_bytes())?;
        Ok(result.to_string())
    }
    
    /// periodt Hash password with PBKDF2-SHA512 and custom iterations
    pub fn hash_password_sha512(password: &str, iterations: u32) -> Result<String, Pbkdf2Error> {
        let config = Pbkdf2Config::with_params(HmacAlgorithm::Sha512, iterations, 64);
        let engine = Pbkdf2Engine::new(config)?;
        let result = engine.hash_password(password.as_bytes())?;
        Ok(result.to_string())
    }
    
    /// facts Verify password against stored hash
    pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool, Pbkdf2Error> {
        let result = Pbkdf2Result::from_string(stored_hash)?;
        result.verify(password.as_bytes())
    }
    
    /// yolo Derive key from password with custom parameters
    pub fn derive_key(password: &str, salt: &[u8], iterations: u32, output_len: usize) -> Result<Vec<u8>, Pbkdf2Error> {
        let config = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, iterations, output_len);
        let engine = Pbkdf2Engine::new(config)?;
        engine.derive_key(password.as_bytes(), salt)
    }
    
    /// slay Generate secure random salt
    pub fn generate_salt(length: usize) -> Result<Vec<u8>, Pbkdf2Error> {
        if length < 8 {
            return Err(Pbkdf2Error::InvalidInput("Salt length must be at least 8 bytes".to_string()));
        }
        
        use rand::RngCore;
        let mut salt = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut salt);
        Ok(salt)
    }
}

/// fr fr Public API functions for CURSED integration

/// slay PBKDF2 key derivation function
pub fn pbkdf2_derive_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 4 {
        return Err(CursedError::Runtime("PBKDF2 requires password, salt, iterations, and output length".to_string()));
    }
    
    let password = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Password must be a string".to_string())),
    };
    
    let salt_hex = match &args[1] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Salt must be a hex string".to_string())),
    };
    
    let iterations = match &args[2] {
        Value::Number(n) => *n as u32,
        _ => return Err(CursedError::Runtime("Iterations must be a number".to_string())),
    };
    
    let output_len = match &args[3] {
        Value::Number(n) => *n as usize,
        _ => return Err(CursedError::Runtime("Output length must be a number".to_string())),
    };
    
    // Decode salt from hex
    let salt = match hex::decode(salt_hex) {
        Ok(s) => s,
        Err(_) => return Err(CursedError::Runtime("Invalid hex encoding in salt".to_string())),
    };
    
    match Pbkdf2Utils::derive_key(std::str::from_utf8(password).unwrap(), &salt, iterations, output_len) {
        Ok(key) => Ok(Value::String(Pbkdf2Utils::to_hex(&key))),
        Err(e) => Err(CursedError::Runtime(format!("PBKDF2 derivation failed: {}", e))),
    }
}

/// slay Hash password with PBKDF2
pub fn pbkdf2_hash_password(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("PBKDF2 hash requires password".to_string()));
    }
    
    let password = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Password must be a string".to_string())),
    };
    
    let algorithm = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => match s.as_str() {
                "SHA256" => HmacAlgorithm::Sha256,
                "SHA512" => HmacAlgorithm::Sha512,
                "BLAKE3" => HmacAlgorithm::Blake3,
                _ => HmacAlgorithm::Sha256,
            },
            _ => HmacAlgorithm::Sha256,
        }
    } else {
        HmacAlgorithm::Sha256
    };
    
    let iterations = if args.len() > 2 {
        match &args[2] {
            Value::Number(n) => *n as u32,
            _ => 100_000,
        }
    } else {
        100_000
    };
    
    match algorithm {
        HmacAlgorithm::Sha256 => {
            match Pbkdf2Utils::hash_password_sha256(password) {
                Ok(hash) => Ok(Value::String(hash)),
                Err(e) => Err(CursedError::Runtime(format!("PBKDF2 hash failed: {}", e))),
            }
        },
        HmacAlgorithm::Sha512 => {
            match Pbkdf2Utils::hash_password_sha512(password, iterations) {
                Ok(hash) => Ok(Value::String(hash)),
                Err(e) => Err(CursedError::Runtime(format!("PBKDF2 hash failed: {}", e))),
            }
        },
        _ => Err(CursedError::Runtime("Unsupported PBKDF2 algorithm".to_string())),
    }
}

/// slay Verify password with PBKDF2
pub fn pbkdf2_verify_password(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("PBKDF2 verify requires password and stored hash".to_string()));
    }
    
    let password = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Password must be a string".to_string())),
    };
    
    let stored_hash = match &args[1] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("Stored hash must be a string".to_string())),
    };
    
    match Pbkdf2Utils::verify_password(password, stored_hash) {
        Ok(valid) => Ok(Value::bool(valid)),
        Err(e) => Err(CursedError::Runtime(format!("PBKDF2 verification failed: {}", e))),
    }
}

/// slay Generate PBKDF2 salt
pub fn pbkdf2_generate_salt(args: Vec<Value>) -> Result<Value, CursedError> {
    let length = if args.is_empty() {
        16
    } else {
        match &args[0] {
            Value::Number(n) => *n as usize,
            _ => 16,
        }
    };
    
    match Pbkdf2Utils::generate_salt(length) {
        Ok(salt) => Ok(Value::String(Pbkdf2Utils::to_hex(&salt))),
        Err(e) => Err(CursedError::Runtime(format!("Salt generation failed: {}", e))),
    }
}

/// slay Create PBKDF2 configuration
pub fn create_pbkdf2_config(args: Vec<Value>) -> Result<Value, CursedError> {
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
        _ => HmacAlgorithm::Sha256,
    };
    
    let iterations = if args.len() > 1 {
        match &args[1] {
            Value::Number(n) => *n as u32,
            _ => 100_000,
        }
    } else {
        100_000
    };
    
    let output_len = if args.len() > 2 {
        match &args[2] {
            Value::Number(n) => *n as usize,
            _ => algorithm.output_size(),
        }
    } else {
        algorithm.output_size()
    };
    
    let config = Pbkdf2Config::with_params(algorithm, iterations, output_len);
    
    let mut result = HashMap::new();
    result.insert("algorithm".to_string(), Value::String(algorithm.name().to_string()));
    result.insert("iterations".to_string(), Value::Number(iterations as f64));
    result.insert("output_len".to_string(), Value::Number(output_len as f64));
    result.insert("salt_len".to_string(), Value::Number(config.salt_len as f64));
    
    Ok(Value::Object(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pbkdf2_basic_derivation() {
        let config = Pbkdf2Config::new();
        let engine = Pbkdf2Engine::new(config).unwrap();
        
        let password = b"test_password";
        let salt = b"test_salt_123456"; // At least 8 bytes
        
        let key1 = engine.derive_key(password, salt).unwrap();
        let key2 = engine.derive_key(password, salt).unwrap();
        
        assert_eq!(key1, key2); // Should be deterministic
        assert_eq!(key1.len(), 32); // Default output length
    }
    
    #[test]
    fn test_pbkdf2_different_passwords() {
        let config = Pbkdf2Config::new();
        let engine = Pbkdf2Engine::new(config).unwrap();
        
        let salt = b"same_salt_123456";
        let key1 = engine.derive_key(b"password1", salt).unwrap();
        let key2 = engine.derive_key(b"password2", salt).unwrap();
        
        assert_ne!(key1, key2); // Different passwords should produce different keys
    }
    
    #[test]
    fn test_pbkdf2_different_salts() {
        let config = Pbkdf2Config::new();
        let engine = Pbkdf2Engine::new(config).unwrap();
        
        let password = b"same_password";
        let key1 = engine.derive_key(password, b"salt1_123456").unwrap();
        let key2 = engine.derive_key(password, b"salt2_123456").unwrap();
        
        assert_ne!(key1, key2); // Different salts should produce different keys
    }
    
    #[test]
    fn test_pbkdf2_custom_iterations() {
        let config = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, 10_000, 32);
        let engine = Pbkdf2Engine::new(config).unwrap();
        
        let password = b"test_password";
        let salt = b"test_salt_123456";
        
        let key = engine.derive_key(password, salt).unwrap();
        assert_eq!(key.len(), 32);
    }
    
    #[test]
    fn test_pbkdf2_different_algorithms() {
        let config_sha256 = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, 10_000, 32);
        let config_sha512 = Pbkdf2Config::with_params(HmacAlgorithm::Sha512, 10_000, 32);
        
        let engine_sha256 = Pbkdf2Engine::new(config_sha256).unwrap();
        let engine_sha512 = Pbkdf2Engine::new(config_sha512).unwrap();
        
        let password = b"test_password";
        let salt = b"test_salt_123456";
        
        let key_sha256 = engine_sha256.derive_key(password, salt).unwrap();
        let key_sha512 = engine_sha512.derive_key(password, salt).unwrap();
        
        assert_ne!(key_sha256, key_sha512); // Different algorithms should produce different keys
    }
    
    #[test]
    fn test_pbkdf2_password_hashing() {
        let config = Pbkdf2Config::new();
        let engine = Pbkdf2Engine::new(config).unwrap();
        
        let password = b"my_secure_password";
        let result = engine.hash_password(password).unwrap();
        
        assert_eq!(result.algorithm, HmacAlgorithm::Sha256);
        assert_eq!(result.iterations, 100_000);
        assert!(!result.salt.is_empty());
        assert!(!result.derived_key.is_empty());
    }
    
    #[test]
    fn test_pbkdf2_password_verification() {
        let config = Pbkdf2Config::new();
        let engine = Pbkdf2Engine::new(config).unwrap();
        
        let password = b"correct_password";
        let result = engine.hash_password(password).unwrap();
        
        // Should verify with correct password
        assert!(engine.verify_password(password, &result.salt, &result.derived_key).unwrap());
        
        // Should fail with wrong password
        assert!(!engine.verify_password(b"wrong_password", &result.salt, &result.derived_key).unwrap());
    }
    
    #[test]
    fn test_pbkdf2_result_serialization() {
        let result = Pbkdf2Result {
            algorithm: HmacAlgorithm::Sha256,
            iterations: 100_000,
            salt: vec![1, 2, 3, 4, 5, 6, 7, 8],
            derived_key: vec![9, 10, 11, 12, 13, 14, 15, 16],
        };
        
        let serialized = result.to_string();
        let deserialized = Pbkdf2Result::from_string(&serialized).unwrap();
        
        assert_eq!(result.algorithm, deserialized.algorithm);
        assert_eq!(result.iterations, deserialized.iterations);
        assert_eq!(result.salt, deserialized.salt);
        assert_eq!(result.derived_key, deserialized.derived_key);
    }
    
    #[test]
    fn test_pbkdf2_config_validation() {
        // Valid config
        let valid_config = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, 10_000, 32);
        assert!(valid_config.validate().is_ok());
        
        // Invalid iterations (too low)
        let invalid_config = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, 500, 32);
        assert!(invalid_config.validate().is_err());
        
        // Invalid output length (zero)
        let invalid_config = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, 10_000, 0);
        assert!(invalid_config.validate().is_err());
    }
    
    #[test]
    fn test_pbkdf2_input_validation() {
        let config = Pbkdf2Config::new();
        let engine = Pbkdf2Engine::new(config).unwrap();
        
        // Empty password should fail
        assert!(engine.derive_key(b"", b"salt_123456").is_err());
        
        // Short salt should fail
        assert!(engine.derive_key(b"password", b"short").is_err());
    }
    
    #[test]
    fn test_pbkdf2_security_levels() {
        assert_eq!(Pbkdf2Config::recommended_iterations(SecurityLevel::Fast), 10_000);
        assert_eq!(Pbkdf2Config::recommended_iterations(SecurityLevel::Standard), 100_000);
        assert_eq!(Pbkdf2Config::recommended_iterations(SecurityLevel::High), 500_000);
        assert_eq!(Pbkdf2Config::recommended_iterations(SecurityLevel::Maximum), 1_000_000);
    }
    
    #[test]
    fn test_pbkdf2_salt_generation() {
        let salt1 = Pbkdf2Utils::generate_salt(16).unwrap();
        let salt2 = Pbkdf2Utils::generate_salt(16).unwrap();
        
        assert_eq!(salt1.len(), 16);
        assert_eq!(salt2.len(), 16);
        assert_ne!(salt1, salt2); // Should be different random salts
    }
}
