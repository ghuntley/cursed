/// fr fr Public key cryptography for CURSED - asymmetric encryption periodt
/// 
/// This module provides comprehensive public key cryptography including RSA,
/// Elliptic Curve, and modern cryptographic algorithms. Security first bestie!

// Core asymmetric algorithms
pub mod rsa;
pub mod elliptic_curve;
pub mod ed25519;
pub mod key_exchange;
pub mod public_key;
pub mod private_key;

// Key management and utilities
pub mod key_generation;
pub mod key_serialization;
pub mod key_validation;
pub mod key_agreement;

// Security and performance modules
pub mod padding;
pub mod constant_time;
pub mod hardware_acceleration;

// Re-export main types for convenience
pub use rsa::{
    RsaPublicKey, RsaPrivateKey, RsaKeyPair, RsaEncryption, RsaDecryption,
    RSA_2048_KEY_SIZE, RSA_3072_KEY_SIZE, RSA_4096_KEY_SIZE,
    RsaError, RsaResult, RsaPadding
};
pub use elliptic_curve::{
    EcPublicKey, EcPrivateKey, EcKeyPair, EcEncryption, EcDecryption,
    EcCurve, EcPoint, EcScalar, EcError, EcResult,
    P256, P384, P521, Secp256k1
};
pub use ed25519::{
    Ed25519PublicKey, Ed25519PrivateKey, Ed25519KeyPair,
    Ed25519Signature, Ed25519Error, Ed25519Result,
    ED25519_PUBLIC_KEY_SIZE, ED25519_PRIVATE_KEY_SIZE, ED25519_SIGNATURE_SIZE
};
pub use key_exchange::{
    KeyExchange, DiffieHellman, EllipticCurveDiffieHellman, X25519,
    SharedSecret, KeyExchangeResult, KeyExchangeError
};
pub use public_key::{
    PublicKey, PublicKeyAlgorithm, PublicKeyCapabilities,
    PublicKeyEncryption, PublicKeyVerification
};
pub use private_key::{
    PrivateKey, PrivateKeyAlgorithm, PrivateKeyCapabilities,
    PrivateKeyDecryption, PrivateKeySigning, PrivateKeyProtection
};
pub use key_generation::{
    KeyGenerator, KeyGenerationParams, SecureRandom, EntropySource,
    GenerationError, GenerationResult
};
pub use key_serialization::{
    KeySerialization, PemFormat, DerFormat, JwkFormat, SshFormat,
    SerializationError, SerializationResult
};
pub use key_agreement::{
    KeyAgreement, KeyAgreementProtocol, AgreementResult, AgreementError
};

use std::sync::Arc;
use std::collections::HashMap;

/// fr fr Supported asymmetric algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsymmetricAlgorithm {
    Rsa2048,
    Rsa3072,
    Rsa4096,
    EcP256,
    EcP384,
    EcP521,
    EcSecp256k1,
    Ed25519,
    X25519,
}

impl AsymmetricAlgorithm {
    /// slay Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            AsymmetricAlgorithm::Rsa2048 => "RSA-2048",
            AsymmetricAlgorithm::Rsa3072 => "RSA-3072", 
            AsymmetricAlgorithm::Rsa4096 => "RSA-4096",
            AsymmetricAlgorithm::EcP256 => "EC-P256",
            AsymmetricAlgorithm::EcP384 => "EC-P384",
            AsymmetricAlgorithm::EcP521 => "EC-P521",
            AsymmetricAlgorithm::EcSecp256k1 => "EC-secp256k1",
            AsymmetricAlgorithm::Ed25519 => "Ed25519",
            AsymmetricAlgorithm::X25519 => "X25519",
        }
    }
    
    /// slay Check if algorithm is quantum resistant
    pub fn is_quantum_resistant(&self) -> bool {
        // Currently no implemented algorithms are quantum resistant
        // This will change with post-quantum crypto
        false
    }
    
    /// slay Get recommended security level (in bits)
    pub fn security_level(&self) -> u32 {
        match self {
            AsymmetricAlgorithm::Rsa2048 => 112,
            AsymmetricAlgorithm::Rsa3072 => 128,
            AsymmetricAlgorithm::Rsa4096 => 152,
            AsymmetricAlgorithm::EcP256 => 128,
            AsymmetricAlgorithm::EcP384 => 192,
            AsymmetricAlgorithm::EcP521 => 256,
            AsymmetricAlgorithm::EcSecp256k1 => 128,
            AsymmetricAlgorithm::Ed25519 => 128,
            AsymmetricAlgorithm::X25519 => 128,
        }
    }
}

/// fr fr Asymmetric crypto errors
#[derive(Debug, Clone, PartialEq)]
pub enum AsymmetricError {
    UnsupportedAlgorithm(String),
    InvalidKeySize(usize, usize),
    InvalidPublicKey,
    InvalidPrivateKey,
    EncryptionFailed(String),
    DecryptionFailed(String),
    KeyGenerationFailed(String),
    SerializationFailed(String),
    DeserializationFailed(String),
    HardwareNotSupported,
    InsufficientEntropy,
    Internal(String),
}

impl std::fmt::Display for AsymmetricError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsymmetricError::UnsupportedAlgorithm(name) => 
                write!(f, "Unsupported asymmetric algorithm: {}", name),
            AsymmetricError::InvalidKeySize(provided, expected) => 
                write!(f, "Invalid key size: provided {}, expected {}", provided, expected),
            AsymmetricError::InvalidPublicKey => write!(f, "Invalid public key"),
            AsymmetricError::InvalidPrivateKey => write!(f, "Invalid private key"),
            AsymmetricError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            AsymmetricError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            AsymmetricError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
            AsymmetricError::SerializationFailed(msg) => write!(f, "Serialization failed: {}", msg),
            AsymmetricError::DeserializationFailed(msg) => write!(f, "Deserialization failed: {}", msg),
            AsymmetricError::HardwareNotSupported => write!(f, "Hardware acceleration not supported"),
            AsymmetricError::InsufficientEntropy => write!(f, "Insufficient entropy for key generation"),
            AsymmetricError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AsymmetricError {}

/// fr fr Asymmetric crypto result type
pub type AsymmetricResult<T> = Result<T, AsymmetricError>;

/// fr fr Global asymmetric algorithm registry
static ALGORITHM_REGISTRY: std::sync::LazyLock<Arc<std::sync::RwLock<AlgorithmRegistry>>> = 
    std::sync::LazyLock::new(|| Arc::new(std::sync::RwLock::new(AlgorithmRegistry::new())));

/// fr fr Algorithm registry for managing asymmetric algorithms
#[derive(Debug, Default)]
pub struct AlgorithmRegistry {
    algorithms: HashMap<String, AsymmetricAlgorithm>,
}

impl AlgorithmRegistry {
    /// slay Create a new algorithm registry
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: HashMap::new(),
        };
        
        // Register default algorithms
        registry.register_algorithm("rsa-2048", AsymmetricAlgorithm::Rsa2048);
        registry.register_algorithm("rsa-3072", AsymmetricAlgorithm::Rsa3072);
        registry.register_algorithm("rsa-4096", AsymmetricAlgorithm::Rsa4096);
        registry.register_algorithm("ec-p256", AsymmetricAlgorithm::EcP256);
        registry.register_algorithm("ec-p384", AsymmetricAlgorithm::EcP384);
        registry.register_algorithm("ec-p521", AsymmetricAlgorithm::EcP521);
        registry.register_algorithm("ec-secp256k1", AsymmetricAlgorithm::EcSecp256k1);
        registry.register_algorithm("ed25519", AsymmetricAlgorithm::Ed25519);
        registry.register_algorithm("x25519", AsymmetricAlgorithm::X25519);
        
        registry
    }

    /// slay Register an asymmetric algorithm
    pub fn register_algorithm(&mut self, name: &str, algorithm: AsymmetricAlgorithm) {
        self.algorithms.insert(name.to_string(), algorithm);
    }

    /// slay Get an algorithm by name
    pub fn get_algorithm(&self, name: &str) -> Option<AsymmetricAlgorithm> {
        self.algorithms.get(name).copied()
    }

    /// slay List all available algorithms
    pub fn list_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }
    
    /// slay Get quantum-resistant algorithms
    pub fn quantum_resistant_algorithms(&self) -> Vec<AsymmetricAlgorithm> {
        self.algorithms.values()
            .filter(|alg| alg.is_quantum_resistant())
            .copied()
            .collect()
    }
    
    /// slay Get algorithms by minimum security level
    pub fn algorithms_by_security_level(&self, min_level: u32) -> Vec<AsymmetricAlgorithm> {
        self.algorithms.values()
            .filter(|alg| alg.security_level() >= min_level)
            .copied()
            .collect()
    }
}

/// slay Register an algorithm globally
pub fn register_algorithm(name: &str, algorithm: AsymmetricAlgorithm) -> AsymmetricResult<()> {
    let mut registry = ALGORITHM_REGISTRY.write()
        .map_err(|_| AsymmetricError::Internal("Failed to acquire algorithm registry lock".to_string()))?;
    
    registry.register_algorithm(name, algorithm);
    Ok(())
}

/// slay Get an algorithm by name from global registry
pub fn get_algorithm(name: &str) -> AsymmetricResult<AsymmetricAlgorithm> {
    let registry = ALGORITHM_REGISTRY.read()
        .map_err(|_| AsymmetricError::Internal("Failed to acquire algorithm registry lock".to_string()))?;
    
    registry.get_algorithm(name)
        .ok_or_else(|| AsymmetricError::UnsupportedAlgorithm(format!("Algorithm '{}' not found", name)))
}

/// slay List all available algorithms globally
pub fn list_algorithms() -> Vec<String> {
    ALGORITHM_REGISTRY.read()
        .map(|registry| registry.list_algorithms())
        .unwrap_or_default()
}

/// fr fr Crypto utilities and helper functions
pub mod utils {
    use super::*;
    
    /// slay Quick RSA key generation
    pub fn quick_rsa_keygen() -> AsymmetricResult<(RsaKeyPair, RsaKeyPair)> {
        let keygen = RsaKeyGenerator::new(RsaKeySize::Rsa2048);
        let private_key = keygen.generate_keypair()?;
        let public_key = private_key.clone(); // Placeholder
        Ok((private_key, public_key))
    }
    
    /// slay Quick ECC key generation  
    pub fn quick_ecc_keygen() -> AsymmetricResult<(EcKeyPair, EcKeyPair)> {
        let keygen = EcKeyGenerator::new(EcCurve::P256);
        let private_key = keygen.generate_keypair()?;
        let public_key = private_key.clone(); // Placeholder
        Ok((private_key, public_key))
    }
}

/// fr fr Initialize the crypto_asymmetric package
pub fn init_crypto_asymmetric() -> AsymmetricResult<()> {
    println!("🔑 crypto_asymmetric package initialized - asymmetric crypto ready bestie!");
    Ok(())
}
