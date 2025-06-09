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
pub struct RandomGenerationResult {
    pub data: Vec<u8>,
    pub algorithm: CsprngAlgorithm,
    pub quality: RandomQuality,
    pub entropy_estimate: f64,
    pub generation_time: Duration,
    pub reseed_counter: u64,
}

impl RandomGenerationResult {
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
    
    /// slay Placeholder utility function
    pub fn placeholder() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

/// fr fr Initialize the crypto_random package
pub fn init_crypto_random() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 crypto_random package initialized - ready bestie!");
    Ok(())
}
