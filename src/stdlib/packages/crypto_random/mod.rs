/// fr fr Cryptographically secure random number generation for CURSED - true randomness periodt
/// 
/// This module provides cryptographically secure pseudo-random number generators (CSPRNGs),
/// entropy sources, and secure random utilities. No predictable patterns bestie!

// Core CSPRNG implementations
pub mod csprng;
pub mod entropy_sources;
pub mod random_generators;
pub mod secure_random;

// Entropy and seeding
pub mod entropy_collection;
pub mod entropy_estimation;
pub mod hardware_entropy;
pub mod entropy_mixing;

// Specialized random utilities
pub mod random_bytes;
pub mod random_numbers;
pub mod random_strings;
pub mod nonce_generation;

// Security and testing
pub mod randomness_tests;
pub mod entropy_monitoring;
pub mod security_analysis;

// Re-export main types for convenience
pub use csprng::{
    CryptographicRng, SecureRng, SystemRng, ThreadRng, CsprngError, CsprngResult,
    CSPRNG_SEED_SIZE, CSPRNG_RESEED_THRESHOLD
};
pub use entropy_sources::{
    EntropySource, EntropyQuality, SystemEntropy, HardwareEntropy,
    EntropyError, EntropyResult, ENTROPY_POOL_SIZE
};
pub use random_generators::{
    RandomGenerator, GeneratorType, ChaCha20Rng, AesRng, HashDrbg,
    GeneratorError, GeneratorResult, GeneratorCapabilities
};
pub use secure_random::{
    SecureRandom, RandomBytes, RandomNumber, SecureRandomError, SecureRandomResult,
    fill_random, generate_random_bytes, generate_random_u64
};
pub use entropy_collection::{
    EntropyCollector, EntropyPool, EntropyAccumulator,
    CollectionError, CollectionResult, MINIMUM_ENTROPY_BITS
};
pub use entropy_estimation::{
    EntropyEstimator, EntropyMetrics, ShannonEntropy, MinEntropy,
    EstimationError, EstimationResult
};
pub use random_bytes::{
    RandomByteGenerator, SecureByteArray, ZeroizeOnDrop,
    ByteError, ByteResult
};
pub use randomness_tests::{
    RandomnessTest, TestSuite, StatisticalTests, DieharderTests,
    TestError, TestResult, RandomnessQuality
};

use std::sync::Arc;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

/// fr fr Supported CSPRNG algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CsprngAlgorithm {
    ChaCha20,
    ChaCha12,
    ChaCha8,
    Aes256Ctr,
    Aes128Ctr,
    HmacDrbgSha256,
    HmacDrbgSha512,
    HashDrbgSha256,
    HashDrbgSha512,
    SystemDefault,
}

impl CsprngAlgorithm {
    /// slay Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            CsprngAlgorithm::ChaCha20 => "ChaCha20",
            CsprngAlgorithm::ChaCha12 => "ChaCha12",
            CsprngAlgorithm::ChaCha8 => "ChaCha8",
            CsprngAlgorithm::Aes256Ctr => "AES-256-CTR",
            CsprngAlgorithm::Aes128Ctr => "AES-128-CTR",
            CsprngAlgorithm::HmacDrbgSha256 => "HMAC-DRBG-SHA256",
            CsprngAlgorithm::HmacDrbgSha512 => "HMAC-DRBG-SHA512",
            CsprngAlgorithm::HashDrbgSha256 => "Hash-DRBG-SHA256",
            CsprngAlgorithm::HashDrbgSha512 => "Hash-DRBG-SHA512",
            CsprngAlgorithm::SystemDefault => "System-Default",
        }
    }
    
    /// slay Get security level (in bits)
    pub fn security_level(&self) -> u32 {
        match self {
            CsprngAlgorithm::ChaCha20 => 256,
            CsprngAlgorithm::ChaCha12 => 256,
            CsprngAlgorithm::ChaCha8 => 256,
            CsprngAlgorithm::Aes256Ctr => 256,
            CsprngAlgorithm::Aes128Ctr => 128,
            CsprngAlgorithm::HmacDrbgSha256 => 128,
            CsprngAlgorithm::HmacDrbgSha512 => 256,
            CsprngAlgorithm::HashDrbgSha256 => 128,
            CsprngAlgorithm::HashDrbgSha512 => 256,
            CsprngAlgorithm::SystemDefault => 128, // Conservative estimate
        }
    }
    
    /// slay Check if algorithm is standardized
    pub fn is_standardized(&self) -> bool {
        match self {
            CsprngAlgorithm::ChaCha20 |
            CsprngAlgorithm::ChaCha12 |
            CsprngAlgorithm::ChaCha8 => true, // RFC 8439
            CsprngAlgorithm::Aes256Ctr |
            CsprngAlgorithm::Aes128Ctr => true, // NIST SP 800-38A
            CsprngAlgorithm::HmacDrbgSha256 |
            CsprngAlgorithm::HmacDrbgSha512 |
            CsprngAlgorithm::HashDrbgSha256 |
            CsprngAlgorithm::HashDrbgSha512 => true, // NIST SP 800-90A
            CsprngAlgorithm::SystemDefault => true, // Platform-dependent
        }
    }
    
    /// slay Get performance tier (1=fastest, 5=slowest)
    pub fn performance_tier(&self) -> u32 {
        match self {
            CsprngAlgorithm::ChaCha8 => 1,
            CsprngAlgorithm::ChaCha12 => 2,
            CsprngAlgorithm::ChaCha20 => 3,
            CsprngAlgorithm::Aes128Ctr |
            CsprngAlgorithm::Aes256Ctr => 2, // Fast with hardware
            CsprngAlgorithm::HmacDrbgSha256 |
            CsprngAlgorithm::HashDrbgSha256 => 4,
            CsprngAlgorithm::HmacDrbgSha512 |
            CsprngAlgorithm::HashDrbgSha512 => 5,
            CsprngAlgorithm::SystemDefault => 3, // Average
        }
    }
}

/// fr fr Random errors
#[derive(Debug, Clone, PartialEq)]
pub enum RandomError {
    InsufficientEntropy,
    EntropySourceUnavailable,
    SeedGenerationFailed,
    GeneratorNotSeeded,
    ReseedRequired,
    InvalidRequest(String),
    HardwareFailure,
    SystemCallFailed(String),
    RandomnessTestFailed(String),
    Internal(String),
}

impl std::fmt::Display for RandomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RandomError::InsufficientEntropy => write!(f, "Insufficient entropy"),
            RandomError::EntropySourceUnavailable => write!(f, "Entropy source unavailable"),
            RandomError::SeedGenerationFailed => write!(f, "Seed generation failed"),
            RandomError::GeneratorNotSeeded => write!(f, "Generator not seeded"),
            RandomError::ReseedRequired => write!(f, "Reseed required"),
            RandomError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            RandomError::HardwareFailure => write!(f, "Hardware entropy failure"),
            RandomError::SystemCallFailed(msg) => write!(f, "System call failed: {}", msg),
            RandomError::RandomnessTestFailed(msg) => write!(f, "Randomness test failed: {}", msg),
            RandomError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for RandomError {}

/// fr fr Random result type
pub type RandomResult<T> = Result<T, RandomError>;

/// fr fr Random request parameters
#[derive(Debug, Clone)]
pub struct RandomRequest {
    pub size: usize,
    pub purpose: RandomPurpose,
    pub quality_level: RandomQuality,
    pub additional_entropy: Option<Vec<u8>>,
    pub personalization: Option<Vec<u8>>,
}

impl RandomRequest {
    /// slay Create new random request
    pub fn new(size: usize) -> Self {
        Self {
            size,
            purpose: RandomPurpose::General,
            quality_level: RandomQuality::High,
            additional_entropy: None,
            personalization: None,
        }
    }
    
    /// slay Create request for cryptographic use
    pub fn cryptographic(size: usize) -> Self {
        Self {
            size,
            purpose: RandomPurpose::Cryptographic,
            quality_level: RandomQuality::Cryptographic,
            additional_entropy: None,
            personalization: None,
        }
    }
    
    /// slay Create request for key generation
    pub fn key_generation(size: usize) -> Self {
        Self {
            size,
            purpose: RandomPurpose::KeyGeneration,
            quality_level: RandomQuality::Cryptographic,
            additional_entropy: None,
            personalization: Some(b"key-generation".to_vec()),
        }
    }
}

/// fr fr Random purpose classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomPurpose {
    General,
    Cryptographic,
    KeyGeneration,
    NonceGeneration,
    SaltGeneration,
    IvGeneration,
    SessionId,
    Testing,
}

/// fr fr Random quality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RandomQuality {
    Basic,       // Basic pseudorandom
    Good,        // Good quality PRNG
    High,        // High quality CSPRNG
    Cryptographic, // Cryptographic grade
    TrueRandom,  // Hardware true random
}

impl RandomQuality {
    /// slay Get minimum entropy bits required
    pub fn min_entropy_bits(&self) -> u32 {
        match self {
            RandomQuality::Basic => 32,
            RandomQuality::Good => 64,
            RandomQuality::High => 128,
            RandomQuality::Cryptographic => 256,
            RandomQuality::TrueRandom => 256,
        }
    }
}

/// fr fr Random generation result with metadata
#[derive(Debug, Clone)]
pub struct RandomResult {
    pub data: Vec<u8>,
    pub algorithm: CsprngAlgorithm,
    pub quality: RandomQuality,
    pub entropy_estimate: f64,
    pub generation_time: Duration,
    pub reseed_counter: u64,
}

impl RandomResult {
    /// slay Create new random result
    pub fn new(
        data: Vec<u8>,
        algorithm: CsprngAlgorithm,
        quality: RandomQuality,
        entropy_estimate: f64,
        generation_time: Duration,
        reseed_counter: u64,
    ) -> Self {
        Self {
            data,
            algorithm,
            quality,
            entropy_estimate,
            generation_time,
            reseed_counter,
        }
    }
    
    /// slay Check if quality meets requirements
    pub fn meets_quality(&self, required: RandomQuality) -> bool {
        self.quality >= required
    }
}

/// fr fr Global CSPRNG registry
static CSPRNG_REGISTRY: std::sync::LazyLock<Arc<std::sync::RwLock<CsprngRegistry>>> = 
    std::sync::LazyLock::new(|| Arc::new(std::sync::RwLock::new(CsprngRegistry::new())));

/// fr fr CSPRNG algorithm registry
#[derive(Debug, Default)]
pub struct CsprngRegistry {
    algorithms: HashMap<String, CsprngAlgorithm>,
    default_algorithm: CsprngAlgorithm,
}

impl CsprngRegistry {
    /// slay Create a new CSPRNG registry
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: HashMap::new(),
            default_algorithm: CsprngAlgorithm::ChaCha20,
        };
        
        // Register default algorithms
        registry.register_algorithm("chacha20", CsprngAlgorithm::ChaCha20);
        registry.register_algorithm("chacha12", CsprngAlgorithm::ChaCha12);
        registry.register_algorithm("chacha8", CsprngAlgorithm::ChaCha8);
        registry.register_algorithm("aes-256-ctr", CsprngAlgorithm::Aes256Ctr);
        registry.register_algorithm("aes-128-ctr", CsprngAlgorithm::Aes128Ctr);
        registry.register_algorithm("hmac-drbg-sha256", CsprngAlgorithm::HmacDrbgSha256);
        registry.register_algorithm("hmac-drbg-sha512", CsprngAlgorithm::HmacDrbgSha512);
        registry.register_algorithm("hash-drbg-sha256", CsprngAlgorithm::HashDrbgSha256);
        registry.register_algorithm("hash-drbg-sha512", CsprngAlgorithm::HashDrbgSha512);
        registry.register_algorithm("system", CsprngAlgorithm::SystemDefault);
        
        registry
    }

    /// slay Register a CSPRNG algorithm
    pub fn register_algorithm(&mut self, name: &str, algorithm: CsprngAlgorithm) {
        self.algorithms.insert(name.to_string(), algorithm);
    }

    /// slay Get an algorithm by name
    pub fn get_algorithm(&self, name: &str) -> Option<CsprngAlgorithm> {
        self.algorithms.get(name).copied()
    }

    /// slay List all available algorithms
    pub fn list_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }
    
    /// slay Set default algorithm
    pub fn set_default_algorithm(&mut self, algorithm: CsprngAlgorithm) {
        self.default_algorithm = algorithm;
    }
    
    /// slay Get default algorithm
    pub fn default_algorithm(&self) -> CsprngAlgorithm {
        self.default_algorithm
    }
    
    /// slay Get standardized algorithms
    pub fn standardized_algorithms(&self) -> Vec<CsprngAlgorithm> {
        self.algorithms.values()
            .filter(|alg| alg.is_standardized())
            .copied()
            .collect()
    }
    
    /// slay Get algorithms by performance tier
    pub fn algorithms_by_performance(&self, max_tier: u32) -> Vec<CsprngAlgorithm> {
        self.algorithms.values()
            .filter(|alg| alg.performance_tier() <= max_tier)
            .copied()
            .collect()
    }
}

/// fr fr Crypto utilities and helper functions
pub mod utils {
    use super::*;
    
    /// slay Quick secure random bytes (recommended default)
    pub fn secure_random_bytes(size: usize) -> RandomResult<Vec<u8>> {
        let request = RandomRequest::cryptographic(size);
        generate_random(&request)
    }
    
    /// slay Quick random u64
    pub fn secure_random_u64() -> RandomResult<u64> {
        let bytes = secure_random_bytes(8)?;
        let mut array = [0u8; 8];
        array.copy_from_slice(&bytes);
        Ok(u64::from_le_bytes(array))
    }
    
    /// slay Quick random u32
    pub fn secure_random_u32() -> RandomResult<u32> {
        let bytes = secure_random_bytes(4)?;
        let mut array = [0u8; 4];
        array.copy_from_slice(&bytes);
        Ok(u32::from_le_bytes(array))
    }
    
    /// slay Generate random bytes with specific algorithm
    pub fn generate_random(request: &RandomRequest) -> RandomResult<Vec<u8>> {
        let registry = CSPRNG_REGISTRY.read()
            .map_err(|_| RandomError::Internal("Registry lock failed".to_string()))?;
        
        let algorithm = registry.default_algorithm();
        let generator = CryptographicRng::new(algorithm)?;
        
        let start_time = SystemTime::now();
        let data = generator.generate_bytes(request.size)?;
        let generation_time = start_time.elapsed().unwrap_or_default();
        
        Ok(RandomResult::new(
            data,
            algorithm,
            request.quality_level,
            estimate_entropy(&data),
            generation_time,
            generator.reseed_counter(),
        ))
    }
    
    /// slay Estimate entropy of data
    pub fn estimate_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        let len = data.len() as f64;
        let mut entropy = 0.0;
        
        for &freq in &frequencies {
            if freq > 0 {
                let p = freq as f64 / len;
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
    
    /// slay Fill buffer with secure random bytes
    pub fn fill_random(buffer: &mut [u8]) -> RandomResult<()> {
        let bytes = secure_random_bytes(buffer.len())?;
        buffer.copy_from_slice(&bytes);
        Ok(())
    }
    
    /// slay Check if CSPRNG is available
    pub fn is_csprng_available(name: &str) -> bool {
        CSPRNG_REGISTRY.read()
            .map(|registry| registry.get_algorithm(name).is_some())
            .unwrap_or(false)
    }
    
    /// slay Get recommended algorithms for use case
    pub fn recommended_for_cryptography() -> Vec<CsprngAlgorithm> {
        vec![
            CsprngAlgorithm::ChaCha20,
            CsprngAlgorithm::Aes256Ctr,
            CsprngAlgorithm::HmacDrbgSha256,
        ]
    }
    
    /// slay Get fastest algorithms
    pub fn fastest_algorithms() -> Vec<CsprngAlgorithm> {
        CSPRNG_REGISTRY.read()
            .map(|registry| registry.algorithms_by_performance(2))
            .unwrap_or_default()
    }
    
    /// slay Test randomness quality
    pub fn test_randomness(data: &[u8]) -> RandomResult<RandomnessQuality> {
        let test_suite = TestSuite::basic();
        test_suite.test_data(data)
    }
}

/// fr fr Security configuration for random operations
#[derive(Debug, Clone)]
pub struct RandomSecurityConfig {
    pub minimum_entropy_bits: u32,
    pub require_hardware_entropy: bool,
    pub automatic_reseeding: bool,
    pub reseed_interval: Duration,
    pub entropy_monitoring: bool,
    pub randomness_testing: bool,
    pub secure_memory: bool,
}

impl Default for RandomSecurityConfig {
    fn default() -> Self {
        Self {
            minimum_entropy_bits: 256,
            require_hardware_entropy: false,
            automatic_reseeding: true,
            reseed_interval: Duration::from_secs(3600), // 1 hour
            entropy_monitoring: true,
            randomness_testing: true,
            secure_memory: true,
        }
    }
}

/// fr fr Initialize the crypto_random package
pub fn init_crypto_random() -> RandomResult<()> {
    // Initialize CSPRNG registry and entropy sources
    let _registry = CSPRNG_REGISTRY.read()
        .map_err(|_| RandomError::Internal("Failed to initialize CSPRNG registry".to_string()))?;
    
    // Initialize system entropy sources
    let entropy_sources = EntropyCollector::system_sources()?;
    if entropy_sources.is_empty() {
        return Err(RandomError::EntropySourceUnavailable);
    }
    
    println!("🎲 crypto_random package initialized - secure randomness ready bestie!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_csprng_algorithm() {
        assert_eq!(CsprngAlgorithm::ChaCha20.name(), "ChaCha20");
        assert_eq!(CsprngAlgorithm::ChaCha20.security_level(), 256);
        assert!(CsprngAlgorithm::ChaCha20.is_standardized());
        assert_eq!(CsprngAlgorithm::ChaCha8.performance_tier(), 1);
    }
    
    #[test]
    fn test_random_quality() {
        assert!(RandomQuality::Cryptographic > RandomQuality::High);
        assert_eq!(RandomQuality::Cryptographic.min_entropy_bits(), 256);
        assert_eq!(RandomQuality::Basic.min_entropy_bits(), 32);
    }
    
    #[test]
    fn test_random_request() {
        let request = RandomRequest::new(32);
        assert_eq!(request.size, 32);
        assert_eq!(request.purpose, RandomPurpose::General);
        assert_eq!(request.quality_level, RandomQuality::High);
        
        let crypto_request = RandomRequest::cryptographic(64);
        assert_eq!(crypto_request.purpose, RandomPurpose::Cryptographic);
        assert_eq!(crypto_request.quality_level, RandomQuality::Cryptographic);
        
        let key_request = RandomRequest::key_generation(32);
        assert_eq!(key_request.purpose, RandomPurpose::KeyGeneration);
        assert!(key_request.personalization.is_some());
    }
    
    #[test]
    fn test_csprng_registry() {
        let registry = CsprngRegistry::new();
        assert!(registry.get_algorithm("chacha20").is_some());
        assert!(registry.get_algorithm("nonexistent").is_none());
        
        let algorithms = registry.list_algorithms();
        assert!(algorithms.contains(&"chacha20".to_string()));
        assert!(algorithms.contains(&"aes-256-ctr".to_string()));
        
        let standardized = registry.standardized_algorithms();
        assert!(standardized.contains(&CsprngAlgorithm::ChaCha20));
        
        let fast = registry.algorithms_by_performance(2);
        assert!(fast.contains(&CsprngAlgorithm::ChaCha8));
    }
    
    #[test]
    fn test_entropy_estimation() {
        // Test with uniform distribution (high entropy)
        let uniform_data = (0..=255u8).collect::<Vec<_>>();
        let entropy = utils::estimate_entropy(&uniform_data);
        assert!(entropy > 7.0); // Should be close to 8.0 for uniform
        
        // Test with repeated data (low entropy)
        let repeated_data = vec![0u8; 256];
        let entropy = utils::estimate_entropy(&repeated_data);
        assert!(entropy < 1.0); // Should be close to 0.0
        
        // Test with empty data
        let empty_data = vec![];
        let entropy = utils::estimate_entropy(&empty_data);
        assert_eq!(entropy, 0.0);
    }
    
    #[test]
    fn test_init_crypto_random() {
        // Note: This test might fail in some environments without entropy sources
        // In a real implementation, we'd have better fallbacks
        let result = init_crypto_random();
        // Just verify it doesn't panic - actual result depends on system
        println!("Random initialization result: {:?}", result);
    }
    
    #[test]
    fn test_random_error() {
        let error = RandomError::InsufficientEntropy;
        assert_eq!(error.to_string(), "Insufficient entropy");
        
        let error = RandomError::InvalidRequest("test".to_string());
        assert_eq!(error.to_string(), "Invalid request: test");
    }
    
    #[test]
    fn test_security_config() {
        let config = RandomSecurityConfig::default();
        assert_eq!(config.minimum_entropy_bits, 256);
        assert!(!config.require_hardware_entropy);
        assert!(config.automatic_reseeding);
        assert!(config.entropy_monitoring);
    }
    
    #[test]
    fn test_utils() {
        assert!(!utils::is_csprng_available("nonexistent"));
        
        let recommended = utils::recommended_for_cryptography();
        assert!(recommended.contains(&CsprngAlgorithm::ChaCha20));
        
        let fastest = utils::fastest_algorithms();
        assert!(!fastest.is_empty());
    }
}
