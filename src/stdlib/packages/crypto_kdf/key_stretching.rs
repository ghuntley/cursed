/// fr fr Key Stretching algorithms for CURSED crypto
/// 
/// This module provides advanced key stretching algorithms for strengthening
/// weak keys and expanding short keys to required lengths.

use crate::error::CursedError;
use crate::stdlib::value::Value;
use crate::stdlib::packages::crypto_kdf::{KdfResult, KdfError};
use sha2::{Sha256, Sha512, Digest};
use sha3::{Sha3_256, Sha3_512};

/// fr fr Key stretching algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StretchingAlgorithm {
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
    Blake2b,
    Iterative,
}

impl StretchingAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            StretchingAlgorithm::Sha256 => "SHA-256",
            StretchingAlgorithm::Sha512 => "SHA-512",
            StretchingAlgorithm::Sha3_256 => "SHA3-256",
            StretchingAlgorithm::Sha3_512 => "SHA3-512",
            StretchingAlgorithm::Blake2b => "BLAKE2b",
            StretchingAlgorithm::Iterative => "Iterative",
        }
    }
    
    pub fn base_output_len(&self) -> usize {
        match self {
            StretchingAlgorithm::Sha256 | StretchingAlgorithm::Sha3_256 => 32,
            StretchingAlgorithm::Sha512 | StretchingAlgorithm::Sha3_512 => 64,
            StretchingAlgorithm::Blake2b => 64,
            StretchingAlgorithm::Iterative => 32, // Configurable
        }
    }
}

/// fr fr Key stretching configuration
#[derive(Debug, Clone)]
pub struct StretchingConfig {
    pub algorithm: StretchingAlgorithm,
    pub iterations: u32,
    pub salt_len: usize,
    pub expansion_factor: usize,
    pub use_personalization: bool,
}

impl StretchingConfig {
    /// slay Create key stretching config with defaults
    pub fn new() -> Self {
        Self {
            algorithm: StretchingAlgorithm::Sha256,
            iterations: 1000,
            salt_len: 16,
            expansion_factor: 2,
            use_personalization: true,
        }
    }
    
    /// bestie Create config for high-security stretching
    pub fn high_security() -> Self {
        Self {
            algorithm: StretchingAlgorithm::Sha3_512,
            iterations: 10000,
            salt_len: 32,
            expansion_factor: 4,
            use_personalization: true,
        }
    }
    
    /// vibes Create config for fast stretching
    pub fn fast() -> Self {
        Self {
            algorithm: StretchingAlgorithm::Sha256,
            iterations: 100,
            salt_len: 8,
            expansion_factor: 2,
            use_personalization: false,
        }
    }
    
    /// periodt Validate stretching configuration
    pub fn validate(&self) -> KdfResult<()> {
        if self.iterations == 0 {
            return Err(KdfError::InvalidConfig("Iterations must be greater than 0".to_string()));
        }
        
        if self.salt_len < 4 {
            return Err(KdfError::InvalidConfig("Salt length must be at least 4 bytes".to_string()));
        }
        
        if self.expansion_factor == 0 || self.expansion_factor > 1024 {
            return Err(KdfError::InvalidConfig("Expansion factor must be between 1 and 1024".to_string()));
        }
        
        Ok(())
    }
}

impl Default for StretchingConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Key stretching engine
pub struct KeyStretchingEngine {
    config: StretchingConfig,
}

impl KeyStretchingEngine {
    /// slay Create new key stretching engine
    pub fn new() -> Self {
        Self {
            config: StretchingConfig::new(),
        }
    }
    
    /// bestie Create engine with custom config
    pub fn with_config(config: StretchingConfig) -> KdfResult<Self> {
        config.validate()?;
        Ok(Self { config })
    }
    
    /// vibes Stretch a key to the specified length
    pub fn stretch(&self, key: &[u8], target_length: usize) -> KdfResult<Vec<u8>> {
        if key.is_empty() {
            return Err(KdfError::InvalidInput("Key cannot be empty".to_string()));
        }
        
        if target_length == 0 || target_length > 1024 * 1024 {
            return Err(KdfError::InvalidInput("Target length must be between 1 and 1MB".to_string()));
        }
        
        match self.config.algorithm {
            StretchingAlgorithm::Sha256 => self.stretch_sha256(key, target_length),
            StretchingAlgorithm::Sha512 => self.stretch_sha512(key, target_length),
            StretchingAlgorithm::Sha3_256 => self.stretch_sha3_256(key, target_length),
            StretchingAlgorithm::Sha3_512 => self.stretch_sha3_512(key, target_length),
            StretchingAlgorithm::Blake2b => self.stretch_blake2b(key, target_length),
            StretchingAlgorithm::Iterative => self.stretch_iterative(key, target_length),
        }
    }
    
    /// facts Stretch key with salt
    pub fn stretch_with_salt(&self, key: &[u8], salt: &[u8], target_length: usize) -> KdfResult<Vec<u8>> {
        if salt.len() < 4 {
            return Err(KdfError::InvalidInput("Salt must be at least 4 bytes".to_string()));
        }
        
        // Combine key and salt
        let mut combined = Vec::new();
        combined.extend_from_slice(key);
        combined.extend_from_slice(salt);
        
        self.stretch(&combined, target_length)
    }
    
    /// bestie Strengthen weak key by iterative hashing
    pub fn strengthen_key(&self, weak_key: &[u8]) -> KdfResult<Vec<u8>> {
        if weak_key.is_empty() {
            return Err(KdfError::InvalidInput("Weak key cannot be empty".to_string()));
        }
        
        let mut strengthened = weak_key.to_vec();
        
        // Apply iterative strengthening
        for i in 0..self.config.iterations {
            strengthened = match self.config.algorithm {
                StretchingAlgorithm::Sha256 => {
                    let mut hasher = Sha256::new();
                    hasher.update(&strengthened);
                    hasher.update(&(i as u32).to_le_bytes());
                    if self.config.use_personalization {
                        hasher.update(b"strengthen");
                    }
                    hasher.finalize().to_vec()
                }
                StretchingAlgorithm::Sha512 => {
                    let mut hasher = Sha512::new();
                    hasher.update(&strengthened);
                    hasher.update(&(i as u32).to_le_bytes());
                    if self.config.use_personalization {
                        hasher.update(b"strengthen");
                    }
                    hasher.finalize().to_vec()
                }
                StretchingAlgorithm::Sha3_256 => {
                    let mut hasher = Sha3_256::new();
                    hasher.update(&strengthened);
                    hasher.update(&(i as u32).to_le_bytes());
                    if self.config.use_personalization {
                        hasher.update(b"strengthen");
                    }
                    hasher.finalize().to_vec()
                }
                StretchingAlgorithm::Sha3_512 => {
                    let mut hasher = Sha3_512::new();
                    hasher.update(&strengthened);
                    hasher.update(&(i as u32).to_le_bytes());
                    if self.config.use_personalization {
                        hasher.update(b"strengthen");
                    }
                    hasher.finalize().to_vec()
                }
                StretchingAlgorithm::Blake2b => self.blake2b_hash(&strengthened, Some(&(i as u32).to_le_bytes()))?,
                StretchingAlgorithm::Iterative => {
                    let mut hasher = Sha256::new();
                    hasher.update(&strengthened);
                    hasher.update(&(i as u64).to_le_bytes());
                    hasher.finalize().to_vec()
                }
            };
        }
        
        Ok(strengthened)
    }
    
    /// periodt Expand key to multiple derived keys
    pub fn expand_to_multiple(&self, master_key: &[u8], key_count: usize, key_length: usize) -> KdfResult<Vec<Vec<u8>>> {
        if master_key.is_empty() {
            return Err(KdfError::InvalidInput("Master key cannot be empty".to_string()));
        }
        
        if key_count == 0 || key_count > 1024 {
            return Err(KdfError::InvalidInput("Key count must be between 1 and 1024".to_string()));
        }
        
        let mut derived_keys = Vec::new();
        
        for i in 0..key_count {
            let mut context = Vec::new();
            context.extend_from_slice(master_key);
            context.extend_from_slice(&(i as u32).to_le_bytes());
            context.extend_from_slice(b"derive_key");
            
            let derived_key = self.stretch(&context, key_length)?;
            derived_keys.push(derived_key);
        }
        
        Ok(derived_keys)
    }
    
    // Helper methods for different algorithms
    
    fn stretch_sha256(&self, key: &[u8], target_length: usize) -> KdfResult<Vec<u8>> {
        let mut output = Vec::new();
        let mut counter = 0u32;
        
        while output.len() < target_length {
            let mut hasher = Sha256::new();
            hasher.update(key);
            hasher.update(&counter.to_le_bytes());
            if self.config.use_personalization {
                hasher.update(b"stretch_sha256");
            }
            let hash = hasher.finalize();
            output.extend_from_slice(&hash);
            counter += 1;
        }
        
        output.truncate(target_length);
        Ok(output)
    }
    
    fn stretch_sha512(&self, key: &[u8], target_length: usize) -> KdfResult<Vec<u8>> {
        let mut output = Vec::new();
        let mut counter = 0u32;
        
        while output.len() < target_length {
            let mut hasher = Sha512::new();
            hasher.update(key);
            hasher.update(&counter.to_le_bytes());
            if self.config.use_personalization {
                hasher.update(b"stretch_sha512");
            }
            let hash = hasher.finalize();
            output.extend_from_slice(&hash);
            counter += 1;
        }
        
        output.truncate(target_length);
        Ok(output)
    }
    
    fn stretch_sha3_256(&self, key: &[u8], target_length: usize) -> KdfResult<Vec<u8>> {
        let mut output = Vec::new();
        let mut counter = 0u32;
        
        while output.len() < target_length {
            let mut hasher = Sha3_256::new();
            hasher.update(key);
            hasher.update(&counter.to_le_bytes());
            if self.config.use_personalization {
                hasher.update(b"stretch_sha3_256");
            }
            let hash = hasher.finalize();
            output.extend_from_slice(&hash);
            counter += 1;
        }
        
        output.truncate(target_length);
        Ok(output)
    }
    
    fn stretch_sha3_512(&self, key: &[u8], target_length: usize) -> KdfResult<Vec<u8>> {
        let mut output = Vec::new();
        let mut counter = 0u32;
        
        while output.len() < target_length {
            let mut hasher = Sha3_512::new();
            hasher.update(key);
            hasher.update(&counter.to_le_bytes());
            if self.config.use_personalization {
                hasher.update(b"stretch_sha3_512");
            }
            let hash = hasher.finalize();
            output.extend_from_slice(&hash);
            counter += 1;
        }
        
        output.truncate(target_length);
        Ok(output)
    }
    
    fn stretch_blake2b(&self, key: &[u8], target_length: usize) -> KdfResult<Vec<u8>> {
        let mut output = Vec::new();
        let mut counter = 0u32;
        
        while output.len() < target_length {
            let context = format!("stretch_blake2b_{}", counter);
            let hash = self.blake2b_hash(key, Some(context.as_bytes()))?;
            output.extend_from_slice(&hash);
            counter += 1;
        }
        
        output.truncate(target_length);
        Ok(output)
    }
    
    fn stretch_iterative(&self, key: &[u8], target_length: usize) -> KdfResult<Vec<u8>> {
        let mut current = key.to_vec();
        
        // First, strengthen the key through iterations
        for i in 0..self.config.iterations {
            let mut hasher = Sha256::new();
            hasher.update(&current);
            hasher.update(&(i as u32).to_le_bytes());
            hasher.update(b"iterative_stretch");
            current = hasher.finalize().to_vec();
        }
        
        // Then expand to target length
        self.stretch_sha256(&current, target_length)
    }
    
    fn blake2b_hash(&self, input: &[u8], salt: Option<&[u8]>) -> KdfResult<Vec<u8>> {
        // Simplified BLAKE2b using SHA-512 as fallback
        let mut hasher = Sha512::new();
        hasher.update(input);
        if let Some(s) = salt {
            hasher.update(s);
        }
        hasher.update(b"blake2b_fallback");
        Ok(hasher.finalize().to_vec())
    }
}

impl Default for KeyStretchingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Key stretching utilities
pub struct StretchingUtils;

impl StretchingUtils {
    /// bestie Calculate optimal stretching parameters for target security level
    pub fn params_for_security_level(security_level: u32) -> StretchingConfig {
        match security_level {
            1..=80 => StretchingConfig::fast(),
            81..=112 => StretchingConfig::new(),
            113..=128 => StretchingConfig::high_security(),
            _ => StretchingConfig {
                algorithm: StretchingAlgorithm::Sha3_512,
                iterations: 50000,
                salt_len: 64,
                expansion_factor: 8,
                use_personalization: true,
            }
        }
    }
    
    /// vibes Generate random salt for stretching
    pub fn generate_salt(length: usize) -> KdfResult<Vec<u8>> {
        use rand::RngCore;
        
        if length == 0 || length > 1024 {
            return Err(KdfError::InvalidInput("Salt length must be between 1 and 1024 bytes".to_string()));
        }
        
        let mut salt = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut salt);
        Ok(salt)
    }
    
    /// facts Estimate stretching time
    pub fn estimate_stretching_time(config: &StretchingConfig, key_length: usize, target_length: usize) -> f64 {
        // Rough estimate in milliseconds
        let base_time = match config.algorithm {
            StretchingAlgorithm::Sha256 => 0.001,
            StretchingAlgorithm::Sha512 => 0.002,
            StretchingAlgorithm::Sha3_256 => 0.003,
            StretchingAlgorithm::Sha3_512 => 0.005,
            StretchingAlgorithm::Blake2b => 0.002,
            StretchingAlgorithm::Iterative => 0.001,
        };
        
        let rounds = if target_length <= config.algorithm.base_output_len() {
            1
        } else {
            (target_length + config.algorithm.base_output_len() - 1) / config.algorithm.base_output_len()
        };
        
        base_time * (config.iterations as f64) * (rounds as f64) * (1.0 + key_length as f64 / 1000.0)
    }
}

/// fr fr Public API functions for CURSED integration

/// slay Key stretching function
pub fn stretch_key(args: Vec<Value>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(CursedError::Runtime("stretch_key requires key and target_length arguments".to_string()));
    }
    
    let key = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Key must be a string".to_string())),
    };
    
    let target_length = match &args[1] {
        Value::Number(n) => *n as usize,
        _ => return Err(CursedError::Runtime("Target length must be a number".to_string())),
    };
    
    let config = if args.len() > 2 {
        // TODO: Parse config from args[2]
        StretchingConfig::new()
    } else {
        StretchingConfig::new()
    };
    
    let engine = KeyStretchingEngine::with_config(config)
        .map_err(|e| CursedError::Runtime(format!("Key stretching engine creation failed: {}", e)))?;
    
    let stretched_key = engine.stretch(key, target_length)
        .map_err(|e| CursedError::Runtime(format!("Key stretching failed: {}", e)))?;
    
    Ok(Value::String(hex::encode(stretched_key)))
}

/// slay Strengthen weak key
pub fn strengthen_key(args: Vec<Value>) -> Result<(), Error> {
    if args.is_empty() {
        return Err(CursedError::Runtime("strengthen_key requires key argument".to_string()));
    }
    
    let key = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(CursedError::Runtime("Key must be a string".to_string())),
    };
    
    let config = if args.len() > 1 {
        // TODO: Parse config from args[1]
        StretchingConfig::new()
    } else {
        StretchingConfig::new()
    };
    
    let engine = KeyStretchingEngine::with_config(config)
        .map_err(|e| CursedError::Runtime(format!("Key stretching engine creation failed: {}", e)))?;
    
    let strengthened_key = engine.strengthen_key(key)
        .map_err(|e| CursedError::Runtime(format!("Key strengthening failed: {}", e)))?;
    
    Ok(Value::String(hex::encode(strengthened_key)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stretching_config() {
        let config = StretchingConfig::new();
        assert!(config.validate().is_ok());
        
        let fast_config = StretchingConfig::fast();
        assert!(fast_config.validate().is_ok());
        assert!(fast_config.iterations < config.iterations);
        
        let high_sec_config = StretchingConfig::high_security();
        assert!(high_sec_config.validate().is_ok());
        assert!(high_sec_config.iterations > config.iterations);
    }
    
    #[test]
    fn test_key_stretching() {
        let engine = KeyStretchingEngine::new();
        let key = b"short_key";
        let target_length = 64;
        
        let stretched = engine.stretch(key, target_length).unwrap();
        assert_eq!(stretched.len(), target_length);
        
        // Same key should produce same result
        let stretched2 = engine.stretch(key, target_length).unwrap();
        assert_eq!(stretched, stretched2);
        
        // Different key should produce different result
        let different_key = b"other_key";
        let stretched3 = engine.stretch(different_key, target_length).unwrap();
        assert_ne!(stretched, stretched3);
    }
    
    #[test]
    fn test_key_strengthening() {
        let engine = KeyStretchingEngine::new();
        let weak_key = b"weak";
        
        let strengthened = engine.strengthen_key(weak_key).unwrap();
        assert!(!strengthened.is_empty());
        
        // Strengthening should be deterministic
        let strengthened2 = engine.strengthen_key(weak_key).unwrap();
        assert_eq!(strengthened, strengthened2);
        
        // Different weak key should produce different result
        let weak_key2 = b"other_weak";
        let strengthened3 = engine.strengthen_key(weak_key2).unwrap();
        assert_ne!(strengthened, strengthened3);
    }
    
    #[test]
    fn test_multiple_key_expansion() {
        let engine = KeyStretchingEngine::new();
        let master_key = b"master_key";
        let key_count = 5;
        let key_length = 32;
        
        let derived_keys = engine.expand_to_multiple(master_key, key_count, key_length).unwrap();
        assert_eq!(derived_keys.len(), key_count);
        
        // All keys should be different
        for i in 0..key_count {
            assert_eq!(derived_keys[i].len(), key_length);
            for j in (i+1)..key_count {
                assert_ne!(derived_keys[i], derived_keys[j]);
            }
        }
    }
    
    #[test]
    fn test_stretch_with_salt() {
        let engine = KeyStretchingEngine::new();
        let key = b"test_key";
        let salt = b"test_salt";
        let target_length = 48;
        
        let stretched = engine.stretch_with_salt(key, salt, target_length).unwrap();
        assert_eq!(stretched.len(), target_length);
        
        // Different salt should produce different result
        let salt2 = b"other_salt";
        let stretched2 = engine.stretch_with_salt(key, salt2, target_length).unwrap();
        assert_ne!(stretched, stretched2);
    }
    
    #[test]
    fn test_algorithm_variants() {
        let algorithms = vec![
            StretchingAlgorithm::Sha256,
            StretchingAlgorithm::Sha512,
            StretchingAlgorithm::Sha3_256,
            StretchingAlgorithm::Sha3_512,
            StretchingAlgorithm::Blake2b,
            StretchingAlgorithm::Iterative,
        ];
        
        let key = b"test_key";
        let target_length = 32;
        
        let mut results = Vec::new();
        
        for algorithm in algorithms {
            let config = StretchingConfig {
                algorithm,
                iterations: 100,
                salt_len: 16,
                expansion_factor: 2,
                use_personalization: true,
            };
            
            let engine = KeyStretchingEngine::with_config(config).unwrap();
            let result = engine.stretch(key, target_length).unwrap();
            results.push(result);
        }
        
        // All algorithms should produce different results
        for i in 0..results.len() {
            for j in (i+1)..results.len() {
                assert_ne!(results[i], results[j], "Algorithms {} and {} produced same result", i, j);
            }
        }
    }
    
    #[test]
    fn test_stretching_utils() {
        let config = StretchingUtils::params_for_security_level(128);
        assert!(config.validate().is_ok());
        
        let salt = StretchingUtils::generate_salt(16).unwrap();
        assert_eq!(salt.len(), 16);
        
        let salt2 = StretchingUtils::generate_salt(16).unwrap();
        assert_ne!(salt, salt2);
        
        let estimate = StretchingUtils::estimate_stretching_time(&config, 16, 64);
        assert!(estimate > 0.0);
    }
    
    #[test]
    fn test_validation() {
        let engine = KeyStretchingEngine::new();
        
        // Test empty key
        assert!(engine.stretch(&[], 32).is_err());
        
        // Test zero target length
        assert!(engine.stretch(b"key", 0).is_err());
        
        // Test too large target length
        assert!(engine.stretch(b"key", 2 * 1024 * 1024).is_err());
        
        // Test invalid config
        let invalid_config = StretchingConfig {
            algorithm: StretchingAlgorithm::Sha256,
            iterations: 0, // Invalid
            salt_len: 16,
            expansion_factor: 2,
            use_personalization: true,
        };
        assert!(invalid_config.validate().is_err());
    }
}
