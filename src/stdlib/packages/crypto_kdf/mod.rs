/// fr fr Key derivation functions for CURSED - secure key generation periodt
/// 
/// This module provides comprehensive key derivation functions including
/// PBKDF2, scrypt, Argon2, and HKDF for secure password-based and key-based
/// key derivation. Secure keys all day bestie!

// Core KDF algorithms
pub mod pbkdf2;
pub mod scrypt;
pub mod argon2;
pub mod hkdf;
pub mod kdf_traits;

// Password and salt handling
pub mod password_policy;
pub mod salt_generation;
pub mod timing_attacks;

// Advanced features
pub mod key_stretching;
pub mod memory_hard_functions;
pub mod parallel_processing;

// Re-export main types for convenience
pub use pbkdf2::{
    Pbkdf2, Pbkdf2Params, Pbkdf2Error, Pbkdf2Result,
    PBKDF2_MIN_ITERATIONS, PBKDF2_RECOMMENDED_ITERATIONS
};
pub use scrypt::{
    Scrypt, ScryptParams, ScryptError, ScryptResult,
    SCRYPT_MIN_N, SCRYPT_MIN_R, SCRYPT_MIN_P
};
pub use argon2::{
    Argon2, Argon2Params, Argon2Variant, Argon2Error, Argon2Result,
    ARGON2_MIN_MEMORY, ARGON2_MIN_ITERATIONS, ARGON2_MIN_PARALLELISM
};
pub use hkdf::{
    Hkdf, HkdfExtract, HkdfExpand, HkdfParams, HkdfError, HkdfResult,
    HKDF_MAX_OUTPUT_LENGTH
};
pub use kdf_traits::{
    KeyDerivationFunction, KdfParams, KdfResult, KdfError,
    KdfCapabilities, KdfSecurityLevel
};
pub use password_policy::{
    PasswordPolicy, PasswordStrength, PasswordRequirements,
    PolicyError, PolicyResult, validate_password
};
pub use salt_generation::{
    SaltGenerator, SecureSalt, SaltParams, SaltError, SaltResult,
    RECOMMENDED_SALT_SIZE, MIN_SALT_SIZE
};
pub use key_stretching::{
    KeyStretching, StretchingParams, StretchingResult, StretchingError,
    adaptive_iterations, benchmark_iterations
};

use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;

/// fr fr Supported KDF algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KdfAlgorithm {
    Pbkdf2Sha256,
    Pbkdf2Sha384,
    Pbkdf2Sha512,
    ScryptDefault,
    ScryptInteractive,
    ScryptSensitive,
    Argon2i,
    Argon2d,
    Argon2id,
    HkdfSha256,
    HkdfSha384,
    HkdfSha512,
}

impl KdfAlgorithm {
    /// slay Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            KdfAlgorithm::Pbkdf2Sha256 => "PBKDF2-SHA256",
            KdfAlgorithm::Pbkdf2Sha384 => "PBKDF2-SHA384",
            KdfAlgorithm::Pbkdf2Sha512 => "PBKDF2-SHA512",
            KdfAlgorithm::ScryptDefault => "scrypt-default",
            KdfAlgorithm::ScryptInteractive => "scrypt-interactive",
            KdfAlgorithm::ScryptSensitive => "scrypt-sensitive",
            KdfAlgorithm::Argon2i => "Argon2i",
            KdfAlgorithm::Argon2d => "Argon2d",
            KdfAlgorithm::Argon2id => "Argon2id",
            KdfAlgorithm::HkdfSha256 => "HKDF-SHA256",
            KdfAlgorithm::HkdfSha384 => "HKDF-SHA384",
            KdfAlgorithm::HkdfSha512 => "HKDF-SHA512",
        }
    }
    
    /// slay Get recommended security level (in bits)
    pub fn security_level(&self) -> u32 {
        match self {
            KdfAlgorithm::Pbkdf2Sha256 => 128,
            KdfAlgorithm::Pbkdf2Sha384 => 192,
            KdfAlgorithm::Pbkdf2Sha512 => 256,
            KdfAlgorithm::ScryptDefault => 128,
            KdfAlgorithm::ScryptInteractive => 128,
            KdfAlgorithm::ScryptSensitive => 256,
            KdfAlgorithm::Argon2i => 128,
            KdfAlgorithm::Argon2d => 128,
            KdfAlgorithm::Argon2id => 128, // Recommended
            KdfAlgorithm::HkdfSha256 => 128,
            KdfAlgorithm::HkdfSha384 => 192,
            KdfAlgorithm::HkdfSha512 => 256,
        }
    }
    
    /// slay Check if algorithm is memory-hard
    pub fn is_memory_hard(&self) -> bool {
        match self {
            KdfAlgorithm::Pbkdf2Sha256 |
            KdfAlgorithm::Pbkdf2Sha384 |
            KdfAlgorithm::Pbkdf2Sha512 |
            KdfAlgorithm::HkdfSha256 |
            KdfAlgorithm::HkdfSha384 |
            KdfAlgorithm::HkdfSha512 => false,
            KdfAlgorithm::ScryptDefault |
            KdfAlgorithm::ScryptInteractive |
            KdfAlgorithm::ScryptSensitive |
            KdfAlgorithm::Argon2i |
            KdfAlgorithm::Argon2d |
            KdfAlgorithm::Argon2id => true,
        }
    }
    
    /// slay Check if algorithm is suitable for password hashing
    pub fn is_password_suitable(&self) -> bool {
        match self {
            KdfAlgorithm::HkdfSha256 |
            KdfAlgorithm::HkdfSha384 |
            KdfAlgorithm::HkdfSha512 => false, // HKDF is for key derivation, not password hashing
            _ => true,
        }
    }
    
    /// slay Get recommended parameters for this algorithm
    pub fn recommended_params(&self) -> KdfRecommendedParams {
        match self {
            KdfAlgorithm::Pbkdf2Sha256 => KdfRecommendedParams {
                iterations: 600_000,
                memory_cost: 0,
                parallelism: 1,
                salt_size: 16,
                output_size: 32,
            },
            KdfAlgorithm::Pbkdf2Sha384 => KdfRecommendedParams {
                iterations: 600_000,
                memory_cost: 0,
                parallelism: 1,
                salt_size: 16,
                output_size: 48,
            },
            KdfAlgorithm::Pbkdf2Sha512 => KdfRecommendedParams {
                iterations: 600_000,
                memory_cost: 0,
                parallelism: 1,
                salt_size: 16,
                output_size: 64,
            },
            KdfAlgorithm::ScryptDefault => KdfRecommendedParams {
                iterations: 32768, // N
                memory_cost: 8,    // r
                parallelism: 1,    // p
                salt_size: 16,
                output_size: 32,
            },
            KdfAlgorithm::ScryptInteractive => KdfRecommendedParams {
                iterations: 32768,
                memory_cost: 8,
                parallelism: 1,
                salt_size: 16,
                output_size: 32,
            },
            KdfAlgorithm::ScryptSensitive => KdfRecommendedParams {
                iterations: 1048576,
                memory_cost: 8,
                parallelism: 1,
                salt_size: 16,
                output_size: 32,
            },
            KdfAlgorithm::Argon2i => KdfRecommendedParams {
                iterations: 3,
                memory_cost: 65536, // 64 MB
                parallelism: 4,
                salt_size: 16,
                output_size: 32,
            },
            KdfAlgorithm::Argon2d => KdfRecommendedParams {
                iterations: 3,
                memory_cost: 65536,
                parallelism: 4,
                salt_size: 16,
                output_size: 32,
            },
            KdfAlgorithm::Argon2id => KdfRecommendedParams {
                iterations: 3,
                memory_cost: 65536,
                parallelism: 4,
                salt_size: 16,
                output_size: 32,
            },
            KdfAlgorithm::HkdfSha256 => KdfRecommendedParams {
                iterations: 1, // HKDF doesn't use iterations
                memory_cost: 0,
                parallelism: 1,
                salt_size: 16,
                output_size: 32,
            },
            KdfAlgorithm::HkdfSha384 => KdfRecommendedParams {
                iterations: 1,
                memory_cost: 0,
                parallelism: 1,
                salt_size: 16,
                output_size: 48,
            },
            KdfAlgorithm::HkdfSha512 => KdfRecommendedParams {
                iterations: 1,
                memory_cost: 0,
                parallelism: 1,
                salt_size: 16,
                output_size: 64,
            },
        }
    }
}

/// fr fr Recommended parameters for KDF algorithms
#[derive(Debug, Clone)]
pub struct KdfRecommendedParams {
    pub iterations: u32,
    pub memory_cost: u32,
    pub parallelism: u32,
    pub salt_size: usize,
    pub output_size: usize,
}

/// fr fr KDF errors
#[derive(Debug, Clone, PartialEq)]
pub enum KdfError {
    UnsupportedAlgorithm(String),
    InvalidPassword,
    InvalidSalt,
    InvalidParameters(String),
    InsufficientMemory,
    ComputationFailed(String),
    WeakParameters(String),
    TimingAttackDetected,
    Internal(String),
}

impl std::fmt::Display for KdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KdfError::UnsupportedAlgorithm(name) => 
                write!(f, "Unsupported KDF algorithm: {}", name),
            KdfError::InvalidPassword => write!(f, "Invalid password"),
            KdfError::InvalidSalt => write!(f, "Invalid salt"),
            KdfError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            KdfError::InsufficientMemory => write!(f, "Insufficient memory for computation"),
            KdfError::ComputationFailed(msg) => write!(f, "KDF computation failed: {}", msg),
            KdfError::WeakParameters(msg) => write!(f, "Weak parameters detected: {}", msg),
            KdfError::TimingAttackDetected => write!(f, "Potential timing attack detected"),
            KdfError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for KdfError {}

/// fr fr KDF result type
pub type KdfResult<T> = Result<T, KdfError>;

/// fr fr Key derivation parameters
#[derive(Debug, Clone)]
pub struct KeyDerivationParams {
    pub algorithm: KdfAlgorithm,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub memory_cost: u32,
    pub parallelism: u32,
    pub output_length: usize,
    pub additional_data: Option<Vec<u8>>,
}

impl KeyDerivationParams {
    /// slay Create new KDF parameters
    pub fn new(algorithm: KdfAlgorithm, password: Vec<u8>, salt: Vec<u8>, output_length: usize) -> Self {
        let recommended = algorithm.recommended_params();
        
        Self {
            algorithm,
            password,
            salt,
            iterations: recommended.iterations,
            memory_cost: recommended.memory_cost,
            parallelism: recommended.parallelism,
            output_length,
            additional_data: None,
        }
    }
    
    /// slay Validate parameters for security
    pub fn validate(&self) -> KdfResult<()> {
        // Validate salt size
        if self.salt.len() < MIN_SALT_SIZE {
            return Err(KdfError::InvalidSalt);
        }
        
        // Validate password
        if self.password.is_empty() {
            return Err(KdfError::InvalidPassword);
        }
        
        // Algorithm-specific validation
        match self.algorithm {
            KdfAlgorithm::Pbkdf2Sha256 | KdfAlgorithm::Pbkdf2Sha384 | KdfAlgorithm::Pbkdf2Sha512 => {
                if self.iterations < PBKDF2_MIN_ITERATIONS {
                    return Err(KdfError::WeakParameters(
                        format!("PBKDF2 iterations too low: {} < {}", self.iterations, PBKDF2_MIN_ITERATIONS)
                    ));
                }
            },
            KdfAlgorithm::ScryptDefault | KdfAlgorithm::ScryptInteractive | KdfAlgorithm::ScryptSensitive => {
                if self.iterations < SCRYPT_MIN_N {
                    return Err(KdfError::WeakParameters(
                        format!("scrypt N too low: {} < {}", self.iterations, SCRYPT_MIN_N)
                    ));
                }
                if self.memory_cost < SCRYPT_MIN_R {
                    return Err(KdfError::WeakParameters(
                        format!("scrypt r too low: {} < {}", self.memory_cost, SCRYPT_MIN_R)
                    ));
                }
                if self.parallelism < SCRYPT_MIN_P {
                    return Err(KdfError::WeakParameters(
                        format!("scrypt p too low: {} < {}", self.parallelism, SCRYPT_MIN_P)
                    ));
                }
            },
            KdfAlgorithm::Argon2i | KdfAlgorithm::Argon2d | KdfAlgorithm::Argon2id => {
                if self.memory_cost < ARGON2_MIN_MEMORY {
                    return Err(KdfError::WeakParameters(
                        format!("Argon2 memory too low: {} < {}", self.memory_cost, ARGON2_MIN_MEMORY)
                    ));
                }
                if self.iterations < ARGON2_MIN_ITERATIONS {
                    return Err(KdfError::WeakParameters(
                        format!("Argon2 iterations too low: {} < {}", self.iterations, ARGON2_MIN_ITERATIONS)
                    ));
                }
                if self.parallelism < ARGON2_MIN_PARALLELISM {
                    return Err(KdfError::WeakParameters(
                        format!("Argon2 parallelism too low: {} < {}", self.parallelism, ARGON2_MIN_PARALLELISM)
                    ));
                }
            },
            _ => {} // HKDF has no iteration requirements
        }
        
        Ok(())
    }
}

/// fr fr Derived key with metadata
#[derive(Debug, Clone)]
pub struct DerivedKey {
    pub key: Vec<u8>,
    pub algorithm: KdfAlgorithm,
    pub parameters: KeyDerivationParams,
    pub derivation_time: Duration,
}

impl DerivedKey {
    /// slay Create new derived key
    pub fn new(key: Vec<u8>, algorithm: KdfAlgorithm, parameters: KeyDerivationParams, derivation_time: Duration) -> Self {
        Self {
            key,
            algorithm,
            parameters,
            derivation_time,
        }
    }
    
    /// slay Get key bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }
    
    /// slay Check if derivation was fast enough (potential weak parameters)
    pub fn is_derivation_too_fast(&self, min_time: Duration) -> bool {
        self.derivation_time < min_time
    }
}

/// fr fr Global KDF algorithm registry
static KDF_REGISTRY: std::sync::LazyLock<Arc<std::sync::RwLock<KdfRegistry>>> = 
    std::sync::LazyLock::new(|| Arc::new(std::sync::RwLock::new(KdfRegistry::new())));

/// fr fr KDF algorithm registry
#[derive(Debug, Default)]
pub struct KdfRegistry {
    algorithms: HashMap<String, KdfAlgorithm>,
}

impl KdfRegistry {
    /// slay Create a new KDF registry
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: HashMap::new(),
        };
        
        // Register default algorithms
        registry.register_algorithm("pbkdf2-sha256", KdfAlgorithm::Pbkdf2Sha256);
        registry.register_algorithm("pbkdf2-sha384", KdfAlgorithm::Pbkdf2Sha384);
        registry.register_algorithm("pbkdf2-sha512", KdfAlgorithm::Pbkdf2Sha512);
        registry.register_algorithm("scrypt", KdfAlgorithm::ScryptDefault);
        registry.register_algorithm("scrypt-interactive", KdfAlgorithm::ScryptInteractive);
        registry.register_algorithm("scrypt-sensitive", KdfAlgorithm::ScryptSensitive);
        registry.register_algorithm("argon2i", KdfAlgorithm::Argon2i);
        registry.register_algorithm("argon2d", KdfAlgorithm::Argon2d);
        registry.register_algorithm("argon2id", KdfAlgorithm::Argon2id);
        registry.register_algorithm("hkdf-sha256", KdfAlgorithm::HkdfSha256);
        registry.register_algorithm("hkdf-sha384", KdfAlgorithm::HkdfSha384);
        registry.register_algorithm("hkdf-sha512", KdfAlgorithm::HkdfSha512);
        
        registry
    }

    /// slay Register a KDF algorithm
    pub fn register_algorithm(&mut self, name: &str, algorithm: KdfAlgorithm) {
        self.algorithms.insert(name.to_string(), algorithm);
    }

    /// slay Get an algorithm by name
    pub fn get_algorithm(&self, name: &str) -> Option<KdfAlgorithm> {
        self.algorithms.get(name).copied()
    }

    /// slay List all available algorithms
    pub fn list_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }
    
    /// slay Get memory-hard algorithms
    pub fn memory_hard_algorithms(&self) -> Vec<KdfAlgorithm> {
        self.algorithms.values()
            .filter(|alg| alg.is_memory_hard())
            .copied()
            .collect()
    }
    
    /// slay Get password-suitable algorithms
    pub fn password_suitable_algorithms(&self) -> Vec<KdfAlgorithm> {
        self.algorithms.values()
            .filter(|alg| alg.is_password_suitable())
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

/// fr fr Initialize the crypto_kdf package
pub fn init_crypto_kdf() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 crypto_kdf package initialized - ready bestie!");
    Ok(())
}
