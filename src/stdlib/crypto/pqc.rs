//! Post-Quantum Cryptography Module for CURSED
//! 
//! This module provides implementations of post-quantum cryptographic algorithms
//! that are believed to be secure against attacks by quantum computers.
//! 
//! # Algorithms Supported
//! 
//! - **Kyber**: Key Encapsulation Mechanism (KEM) based on Module-LWE
//! - **Dilithium**: Digital signatures based on Module-LWE
//! - **SPHINCS+**: Hash-based signatures (stateless)
//! - **Falcon**: Compact signatures based on NTRU lattices
//! - **NTRU**: Encryption based on NTRU lattices
//! 
//! # Security Considerations
//! 
//! All implementations follow NIST PQC standardization guidelines and provide
//! multiple security levels corresponding to classical cryptographic strength:
//! - Level 1: Equivalent to AES-128
//! - Level 3: Equivalent to AES-192  
//! - Level 5: Equivalent to AES-256
//! 
//! # Usage Example
//! 
//! ```rust
//! use cursed::stdlib::crypto::pqc::{KyberKem, SecurityLevel};
//! 
//! // Generate Kyber-768 key pair (NIST Level 3)
//! let (public_key, secret_key) = KyberKem::keygen(SecurityLevel::Level3)?;
//! 
//! // Encapsulation
//! let (ciphertext, shared_secret) = KyberKem::encaps(&public_key)?;
//! 
//! // Decapsulation
//! let decaps_secret = KyberKem::decaps(&secret_key, &ciphertext)?;
//! assert_eq!(shared_secret, decaps_secret);
//! ```

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use rand::{RngCore, CryptoRng};
use rand::rngs::OsRng;
use sha3::{Sha3_256, Sha3_512, Digest};
use hmac::{Hmac, Mac};

// Post-quantum cryptography imports
use pqcrypto_traits::kem::{PublicKey as KemPublicKey, SecretKey as KemSecretKey, SharedSecret, Ciphertext};
use pqcrypto_traits::sign::{PublicKey as SignPublicKey, SecretKey as SignSecretKey, DetachedSignature};
use pqcrypto_kyber::{kyber512, kyber768, kyber1024};
use pqcrypto_dilithium::{dilithium2, dilithium3, dilithium5};
use pqcrypto_sphincsplus::{sphincssha256128ssimple, sphincssha256192ssimple, sphincssha256256ssimple};
use pqcrypto_falcon::{falcon512, falcon1024};
use pqcrypto_ntru::{ntruhps2048509, ntruhps2048677, ntruhps4096821, ntruhrss701};

// Additional cryptography for hybrid encryption
use aes_gcm::{Aes256Gcm, Key, Nonce, NewAead, Aead};

use crate::error::CursedError;

/// Post-Quantum Cryptography specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum PqcError {
    /// Invalid key size or format
    InvalidKey(String),
    /// Invalid ciphertext or signature
    InvalidCiphertext(String),
    /// Invalid signature or verification failed
    InvalidSignature(String),
    /// Unsupported parameter set or security level
    UnsupportedParameters(String),
    /// Random number generation failed
    RandomGenerationFailed(String),
    /// Key generation failed
    KeyGenerationFailed(String),
    /// Encapsulation failed
    EncapsulationFailed(String),
    /// Decapsulation failed
    DecapsulationFailed(String),
    /// Signing operation failed
    SigningFailed(String),
    /// Verification operation failed
    VerificationFailed(String),
    /// Encryption failed
    EncryptionFailed(String),
    /// Decryption failed
    DecryptionFailed(String),
    /// Parameter validation failed
    ParameterValidation(String),
    /// Internal algorithm error
    InternalError(String),
}

impl fmt::Display for PqcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PqcError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            PqcError::InvalidCiphertext(msg) => write!(f, "Invalid ciphertext: {}", msg),
            PqcError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            PqcError::UnsupportedParameters(msg) => write!(f, "Unsupported parameters: {}", msg),
            PqcError::RandomGenerationFailed(msg) => write!(f, "Random generation failed: {}", msg),
            PqcError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
            PqcError::EncapsulationFailed(msg) => write!(f, "Encapsulation failed: {}", msg),
            PqcError::DecapsulationFailed(msg) => write!(f, "Decapsulation failed: {}", msg),
            PqcError::SigningFailed(msg) => write!(f, "Signing failed: {}", msg),
            PqcError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            PqcError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            PqcError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            PqcError::ParameterValidation(msg) => write!(f, "Parameter validation failed: {}", msg),
            PqcError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for PqcError {}

impl From<PqcError> for CursedError {
    fn from(err: PqcError) -> Self {
        CursedError::Runtime(format!("PQC error: {}", err))
    }
}

/// Result type for PQC operations
pub type PqcResult<T> = Result<T, PqcError>;

/// Security levels corresponding to classical cryptographic strength
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecurityLevel {
    /// NIST Level 1 - Equivalent to AES-128
    Level1,
    /// NIST Level 3 - Equivalent to AES-192
    Level3,
    /// NIST Level 5 - Equivalent to AES-256
    Level5,
}

impl SecurityLevel {
    /// Get the equivalent classical security strength in bits
    pub fn classical_bits(&self) -> u32 {
        match self {
            SecurityLevel::Level1 => 128,
            SecurityLevel::Level3 => 192,
            SecurityLevel::Level5 => 256,
        }
    }

    /// Get a description of the security level
    pub fn description(&self) -> &'static str {
        match self {
            SecurityLevel::Level1 => "NIST Level 1 (AES-128 equivalent)",
            SecurityLevel::Level3 => "NIST Level 3 (AES-192 equivalent)",
            SecurityLevel::Level5 => "NIST Level 5 (AES-256 equivalent)",
        }
    }
}

/// Algorithm type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmType {
    Kyber,
    Dilithium,
    Sphincs,
    Falcon,
    Ntru,
}

impl fmt::Display for AlgorithmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgorithmType::Kyber => write!(f, "Kyber"),
            AlgorithmType::Dilithium => write!(f, "Dilithium"),
            AlgorithmType::Sphincs => write!(f, "SPHINCS+"),
            AlgorithmType::Falcon => write!(f, "Falcon"),
            AlgorithmType::Ntru => write!(f, "NTRU"),
        }
    }
}

/// Performance metrics for PQC operations
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub keygen_time: Duration,
    pub operation_time: Duration,
    pub key_size: usize,
    pub ciphertext_size: usize,
    pub signature_size: Option<usize>,
    pub operations_per_second: f64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            keygen_time: Duration::from_nanos(0),
            operation_time: Duration::from_nanos(0),
            key_size: 0,
            ciphertext_size: 0,
            signature_size: None,
            operations_per_second: 0.0,
        }
    }
}

/// Quantum resistance assessment
#[derive(Debug, Clone)]
pub struct QuantumResistanceAssessment {
    pub algorithm: AlgorithmType,
    pub security_level: SecurityLevel,
    pub quantum_secure: bool,
    pub estimated_quantum_break_time: String,
    pub classical_break_time: String,
    pub key_size_overhead: f64,
    pub performance_overhead: f64,
    pub standardization_status: String,
}

// ============================================================================
// KYBER KEY ENCAPSULATION MECHANISM (KEM)
// ============================================================================

/// Kyber parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KyberParameterSet {
    /// Kyber-512 (NIST Level 1)
    Kyber512,
    /// Kyber-768 (NIST Level 3)
    Kyber768,
    /// Kyber-1024 (NIST Level 5)
    Kyber1024,
}

impl KyberParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
            KyberParameterSet::Kyber512 => SecurityLevel::Level1,
            KyberParameterSet::Kyber768 => SecurityLevel::Level3,
            KyberParameterSet::Kyber1024 => SecurityLevel::Level5,
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
            KyberParameterSet::Kyber512 => 800,
            KyberParameterSet::Kyber768 => 1184,
            KyberParameterSet::Kyber1024 => 1568,
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
            KyberParameterSet::Kyber512 => 1632,
            KyberParameterSet::Kyber768 => 2400,
            KyberParameterSet::Kyber1024 => 3168,
        }
    }

    pub fn ciphertext_size(&self) -> usize {
        match self {
            KyberParameterSet::Kyber512 => 768,
            KyberParameterSet::Kyber768 => 1088,
            KyberParameterSet::Kyber1024 => 1568,
        }
    }

    pub fn shared_secret_size(&self) -> usize {
        32 // All Kyber variants use 32-byte shared secrets
    }
}

/// Kyber public key
#[derive(Debug, Clone)]
pub struct KyberPublicKey {
    pub parameter_set: KyberParameterSet,
    pub key_data: Vec<u8>,
}

/// Kyber secret key
#[derive(Debug, Clone)]
pub struct KyberSecretKey {
    pub parameter_set: KyberParameterSet,
    pub key_data: Vec<u8>,
}

/// Kyber Key Encapsulation Mechanism implementation
pub struct KyberKem;

impl KyberKem {
    /// Generate a Kyber key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(KyberPublicKey, KyberSecretKey)> {
        let parameter_set = match security_level {
            SecurityLevel::Level1 => KyberParameterSet::Kyber512,
            SecurityLevel::Level3 => KyberParameterSet::Kyber768,
            SecurityLevel::Level5 => KyberParameterSet::Kyber1024,
        };

        Self::keygen_with_params(parameter_set)
    }

    /// Generate a Kyber key pair with specific parameter set
    pub fn keygen_with_params(params: KyberParameterSet) -> PqcResult<(KyberPublicKey, KyberSecretKey)> {
        match params {
            KyberParameterSet::Kyber512 => {
                let (pk, sk) = kyber512::keypair();
                let public_key = KyberPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = KyberSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            KyberParameterSet::Kyber768 => {
                let (pk, sk) = kyber768::keypair();
                let public_key = KyberPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = KyberSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            KyberParameterSet::Kyber1024 => {
                let (pk, sk) = kyber1024::keypair();
                let public_key = KyberPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = KyberSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
        }
    }

    /// Encapsulate a shared secret using a public key
    pub fn encaps(public_key: &KyberPublicKey) -> PqcResult<(Vec<u8>, Vec<u8>)> {
        match public_key.parameter_set {
            KyberParameterSet::Kyber512 => {
                let pk = kyber512::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber512 public key".to_string()))?;
                let (ss, ct) = kyber512::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            },
            KyberParameterSet::Kyber768 => {
                let pk = kyber768::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber768 public key".to_string()))?;
                let (ss, ct) = kyber768::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            },
            KyberParameterSet::Kyber1024 => {
                let pk = kyber1024::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber1024 public key".to_string()))?;
                let (ss, ct) = kyber1024::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            },
        }
    }

    /// Decapsulate a shared secret using a secret key and ciphertext
    pub fn decaps(secret_key: &KyberSecretKey, ciphertext: &[u8]) -> PqcResult<Vec<u8>> {
        // Validate ciphertext size
        let expected_size = secret_key.parameter_set.ciphertext_size();
        if ciphertext.len() != expected_size {
            return Err(PqcError::InvalidCiphertext(
                format!("Expected {} bytes, got {}", expected_size, ciphertext.len())
            ));
        }

        match secret_key.parameter_set {
            KyberParameterSet::Kyber512 => {
                let sk = kyber512::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber512 secret key".to_string()))?;
                let ct = kyber512::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid Kyber512 ciphertext".to_string()))?;
                let ss = kyber512::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            },
            KyberParameterSet::Kyber768 => {
                let sk = kyber768::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber768 secret key".to_string()))?;
                let ct = kyber768::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid Kyber768 ciphertext".to_string()))?;
                let ss = kyber768::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            },
            KyberParameterSet::Kyber1024 => {
                let sk = kyber1024::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Kyber1024 secret key".to_string()))?;
                let ct = kyber1024::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid Kyber1024 ciphertext".to_string()))?;
                let ss = kyber1024::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            },
        }
    }


}

// ============================================================================
// DILITHIUM DIGITAL SIGNATURES
// ============================================================================

/// Dilithium parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DilithiumParameterSet {
    /// Dilithium2 (NIST Level 2)
    Dilithium2,
    /// Dilithium3 (NIST Level 3)
    Dilithium3,
    /// Dilithium5 (NIST Level 5)
    Dilithium5,
}

impl DilithiumParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
            DilithiumParameterSet::Dilithium2 => SecurityLevel::Level1,
            DilithiumParameterSet::Dilithium3 => SecurityLevel::Level3,
            DilithiumParameterSet::Dilithium5 => SecurityLevel::Level5,
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
            DilithiumParameterSet::Dilithium2 => 1312,
            DilithiumParameterSet::Dilithium3 => 1952,
            DilithiumParameterSet::Dilithium5 => 2592,
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
            DilithiumParameterSet::Dilithium2 => 2528,
            DilithiumParameterSet::Dilithium3 => 4000,
            DilithiumParameterSet::Dilithium5 => 4864,
        }
    }

    pub fn signature_size(&self) -> usize {
        match self {
            DilithiumParameterSet::Dilithium2 => 2420,
            DilithiumParameterSet::Dilithium3 => 3293,
            DilithiumParameterSet::Dilithium5 => 4595,
        }
    }
}

/// Dilithium public key
#[derive(Debug, Clone)]
pub struct DilithiumPublicKey {
    pub parameter_set: DilithiumParameterSet,
    pub key_data: Vec<u8>,
}

/// Dilithium secret key
#[derive(Debug, Clone)]
pub struct DilithiumSecretKey {
    pub parameter_set: DilithiumParameterSet,
    pub key_data: Vec<u8>,
}

/// Dilithium Digital Signature implementation
pub struct DilithiumSignature;

impl DilithiumSignature {
    /// Generate a Dilithium key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(DilithiumPublicKey, DilithiumSecretKey)> {
        let parameter_set = match security_level {
            SecurityLevel::Level1 => DilithiumParameterSet::Dilithium2,
            SecurityLevel::Level3 => DilithiumParameterSet::Dilithium3,
            SecurityLevel::Level5 => DilithiumParameterSet::Dilithium5,
        };

        Self::keygen_with_params(parameter_set)
    }

    /// Generate a Dilithium key pair with specific parameter set
    pub fn keygen_with_params(params: DilithiumParameterSet) -> PqcResult<(DilithiumPublicKey, DilithiumSecretKey)> {
        match params {
            DilithiumParameterSet::Dilithium2 => {
                let (pk, sk) = dilithium2::keypair();
                let public_key = DilithiumPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = DilithiumSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            DilithiumParameterSet::Dilithium3 => {
                let (pk, sk) = dilithium3::keypair();
                let public_key = DilithiumPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = DilithiumSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            DilithiumParameterSet::Dilithium5 => {
                let (pk, sk) = dilithium5::keypair();
                let public_key = DilithiumPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = DilithiumSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
        }
    }

    /// Sign a message using Dilithium
    pub fn sign(secret_key: &DilithiumSecretKey, message: &[u8]) -> PqcResult<Vec<u8>> {
        match secret_key.parameter_set {
            DilithiumParameterSet::Dilithium2 => {
                let sk = dilithium2::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium2 secret key".to_string()))?;
                let signature = dilithium2::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            },
            DilithiumParameterSet::Dilithium3 => {
                let sk = dilithium3::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium3 secret key".to_string()))?;
                let signature = dilithium3::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            },
            DilithiumParameterSet::Dilithium5 => {
                let sk = dilithium5::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium5 secret key".to_string()))?;
                let signature = dilithium5::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            },
        }
    }

    /// Verify a Dilithium signature
    pub fn verify(public_key: &DilithiumPublicKey, message: &[u8], signature: &[u8]) -> PqcResult<bool> {
        // Validate signature size
        let expected_size = public_key.parameter_set.signature_size();
        if signature.len() != expected_size {
            return Err(PqcError::InvalidSignature(
                format!("Expected {} bytes, got {}", expected_size, signature.len())
            ));
        }

        match public_key.parameter_set {
            DilithiumParameterSet::Dilithium2 => {
                let pk = dilithium2::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium2 public key".to_string()))?;
                let sig = dilithium2::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Dilithium2 signature".to_string()))?;
                match dilithium2::verify_detached_signature(message, &sig, &pk) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            },
            DilithiumParameterSet::Dilithium3 => {
                let pk = dilithium3::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium3 public key".to_string()))?;
                let sig = dilithium3::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Dilithium3 signature".to_string()))?;
                match dilithium3::verify_detached_signature(message, &sig, &pk) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            },
            DilithiumParameterSet::Dilithium5 => {
                let pk = dilithium5::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Dilithium5 public key".to_string()))?;
                let sig = dilithium5::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Dilithium5 signature".to_string()))?;
                match dilithium5::verify_detached_signature(message, &sig, &pk) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            },
        }
    }


}

// ============================================================================
// SPHINCS+ HASH-BASED SIGNATURES
// ============================================================================

/// SPHINCS+ parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SphincsPlusParameterSet {
    /// SPHINCS+-128s (Small signatures, NIST Level 1)
    Sphincs128s,
    /// SPHINCS+-192s (Small signatures, NIST Level 3)
    Sphincs192s,
    /// SPHINCS+-256s (Small signatures, NIST Level 5)
    Sphincs256s,
}

impl SphincsPlusParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
            SphincsPlusParameterSet::Sphincs128s => SecurityLevel::Level1,
            SphincsPlusParameterSet::Sphincs192s => SecurityLevel::Level3,
            SphincsPlusParameterSet::Sphincs256s => SecurityLevel::Level5,
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
            SphincsPlusParameterSet::Sphincs128s => 32,
            SphincsPlusParameterSet::Sphincs192s => 48,
            SphincsPlusParameterSet::Sphincs256s => 64,
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
            SphincsPlusParameterSet::Sphincs128s => 64,
            SphincsPlusParameterSet::Sphincs192s => 96,
            SphincsPlusParameterSet::Sphincs256s => 128,
        }
    }

    pub fn signature_size(&self) -> usize {
        match self {
            SphincsPlusParameterSet::Sphincs128s => 7856,
            SphincsPlusParameterSet::Sphincs192s => 16224,
            SphincsPlusParameterSet::Sphincs256s => 29792,
        }
    }
}

/// SPHINCS+ public key
#[derive(Debug, Clone)]
pub struct SphincsPlusPublicKey {
    pub parameter_set: SphincsPlusParameterSet,
    pub key_data: Vec<u8>,
}

/// SPHINCS+ secret key
#[derive(Debug, Clone)]
pub struct SphincsPlusSecretKey {
    pub parameter_set: SphincsPlusParameterSet,
    pub key_data: Vec<u8>,
}

/// SPHINCS+ Hash-based Signature implementation
pub struct SphincsPlusSignature;

impl SphincsPlusSignature {
    /// Generate a SPHINCS+ key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(SphincsPlusPublicKey, SphincsPlusSecretKey)> {
        let parameter_set = match security_level {
            SecurityLevel::Level1 => SphincsPlusParameterSet::Sphincs128s,
            SecurityLevel::Level3 => SphincsPlusParameterSet::Sphincs192s,
            SecurityLevel::Level5 => SphincsPlusParameterSet::Sphincs256s,
        };

        Self::keygen_with_params(parameter_set)
    }

    /// Generate a SPHINCS+ key pair with specific parameter set
    pub fn keygen_with_params(params: SphincsPlusParameterSet) -> PqcResult<(SphincsPlusPublicKey, SphincsPlusSecretKey)> {
        match params {
            SphincsPlusParameterSet::Sphincs128s => {
                let (pk, sk) = sphincssha256128ssimple::keypair();
                let public_key = SphincsPlusPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = SphincsPlusSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            SphincsPlusParameterSet::Sphincs192s => {
                let (pk, sk) = sphincssha256192ssimple::keypair();
                let public_key = SphincsPlusPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = SphincsPlusSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            SphincsPlusParameterSet::Sphincs256s => {
                let (pk, sk) = sphincssha256256ssimple::keypair();
                let public_key = SphincsPlusPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = SphincsPlusSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
        }
    }

    /// Sign a message using SPHINCS+
    pub fn sign(secret_key: &SphincsPlusSecretKey, message: &[u8]) -> PqcResult<Vec<u8>> {
        match secret_key.parameter_set {
            SphincsPlusParameterSet::Sphincs128s => {
                let sk = sphincssha256128ssimple::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+128s secret key".to_string()))?;
                let signature = sphincssha256128ssimple::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            },
            SphincsPlusParameterSet::Sphincs192s => {
                let sk = sphincssha256192ssimple::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+192s secret key".to_string()))?;
                let signature = sphincssha256192ssimple::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            },
            SphincsPlusParameterSet::Sphincs256s => {
                let sk = sphincssha256256ssimple::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+256s secret key".to_string()))?;
                let signature = sphincssha256256ssimple::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            },
        }
    }

    /// Verify a SPHINCS+ signature
    pub fn verify(public_key: &SphincsPlusPublicKey, message: &[u8], signature: &[u8]) -> PqcResult<bool> {
        let expected_size = public_key.parameter_set.signature_size();
        if signature.len() != expected_size {
            return Err(PqcError::InvalidSignature(
                format!("Expected {} bytes, got {}", expected_size, signature.len())
            ));
        }

        match public_key.parameter_set {
            SphincsPlusParameterSet::Sphincs128s => {
                let pk = sphincssha256128ssimple::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+128s public key".to_string()))?;
                let sig = sphincssha256128ssimple::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid SPHINCS+128s signature".to_string()))?;
                match sphincssha256128ssimple::verify_detached_signature(message, &sig, &pk) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            },
            SphincsPlusParameterSet::Sphincs192s => {
                let pk = sphincssha256192ssimple::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+192s public key".to_string()))?;
                let sig = sphincssha256192ssimple::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid SPHINCS+192s signature".to_string()))?;
                match sphincssha256192ssimple::verify_detached_signature(message, &sig, &pk) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            },
            SphincsPlusParameterSet::Sphincs256s => {
                let pk = sphincssha256256ssimple::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid SPHINCS+256s public key".to_string()))?;
                let sig = sphincssha256256ssimple::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid SPHINCS+256s signature".to_string()))?;
                match sphincssha256256ssimple::verify_detached_signature(message, &sig, &pk) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            },
        }
    }


}

// ============================================================================
// FALCON COMPACT SIGNATURES
// ============================================================================

/// Falcon parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FalconParameterSet {
    /// Falcon-512 (NIST Level 1)
    Falcon512,
    /// Falcon-1024 (NIST Level 5)
    Falcon1024,
}

impl FalconParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
            FalconParameterSet::Falcon512 => SecurityLevel::Level1,
            FalconParameterSet::Falcon1024 => SecurityLevel::Level5,
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
            FalconParameterSet::Falcon512 => 897,
            FalconParameterSet::Falcon1024 => 1793,
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
            FalconParameterSet::Falcon512 => 1281,
            FalconParameterSet::Falcon1024 => 2305,
        }
    }

    pub fn signature_size(&self) -> usize {
        match self {
            FalconParameterSet::Falcon512 => 690,
            FalconParameterSet::Falcon1024 => 1330,
        }
    }
}

/// Falcon public key
#[derive(Debug, Clone)]
pub struct FalconPublicKey {
    pub parameter_set: FalconParameterSet,
    pub key_data: Vec<u8>,
}

/// Falcon secret key
#[derive(Debug, Clone)]
pub struct FalconSecretKey {
    pub parameter_set: FalconParameterSet,
    pub key_data: Vec<u8>,
}

/// Falcon Compact Signature implementation
pub struct FalconSignature;

impl FalconSignature {
    /// Generate a Falcon key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(FalconPublicKey, FalconSecretKey)> {
        let parameter_set = match security_level {
            SecurityLevel::Level1 | SecurityLevel::Level3 => FalconParameterSet::Falcon512,
            SecurityLevel::Level5 => FalconParameterSet::Falcon1024,
        };

        Self::keygen_with_params(parameter_set)
    }

    /// Generate a Falcon key pair with specific parameter set
    pub fn keygen_with_params(params: FalconParameterSet) -> PqcResult<(FalconPublicKey, FalconSecretKey)> {
        match params {
            FalconParameterSet::Falcon512 => {
                let (pk, sk) = falcon512::keypair();
                let public_key = FalconPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = FalconSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            FalconParameterSet::Falcon1024 => {
                let (pk, sk) = falcon1024::keypair();
                let public_key = FalconPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = FalconSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
        }
    }

    /// Sign a message using Falcon
    pub fn sign(secret_key: &FalconSecretKey, message: &[u8]) -> PqcResult<Vec<u8>> {
        match secret_key.parameter_set {
            FalconParameterSet::Falcon512 => {
                let sk = falcon512::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Falcon512 secret key".to_string()))?;
                let signature = falcon512::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            },
            FalconParameterSet::Falcon1024 => {
                let sk = falcon1024::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Falcon1024 secret key".to_string()))?;
                let signature = falcon1024::detached_sign(message, &sk);
                Ok(signature.as_bytes().to_vec())
            },
        }
    }

    /// Verify a Falcon signature
    pub fn verify(public_key: &FalconPublicKey, message: &[u8], signature: &[u8]) -> PqcResult<bool> {
        let expected_size = public_key.parameter_set.signature_size();
        if signature.len() != expected_size {
            return Err(PqcError::InvalidSignature(
                format!("Expected {} bytes, got {}", expected_size, signature.len())
            ));
        }

        match public_key.parameter_set {
            FalconParameterSet::Falcon512 => {
                let pk = falcon512::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Falcon512 public key".to_string()))?;
                let sig = falcon512::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Falcon512 signature".to_string()))?;
                match falcon512::verify_detached_signature(message, &sig, &pk) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            },
            FalconParameterSet::Falcon1024 => {
                let pk = falcon1024::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid Falcon1024 public key".to_string()))?;
                let sig = falcon1024::DetachedSignature::from_bytes(signature)
                    .map_err(|_| PqcError::InvalidSignature("Invalid Falcon1024 signature".to_string()))?;
                match falcon1024::verify_detached_signature(message, &sig, &pk) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            },
        }
    }


}

// ============================================================================
// NTRU ENCRYPTION
// ============================================================================

/// NTRU parameter sets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtruParameterSet {
    /// NTRU-HPS-2048-509 (NIST Level 1)
    NtruHps2048509,
    /// NTRU-HPS-2048-677 (NIST Level 3)
    NtruHps2048677,
    /// NTRU-HPS-4096-821 (NIST Level 5)
    NtruHps4096821,
    /// NTRU-HRSS-701 (NIST Level 1)
    NtruHrss701,
}

impl NtruParameterSet {
    pub fn security_level(&self) -> SecurityLevel {
        match self {
            NtruParameterSet::NtruHps2048509 | NtruParameterSet::NtruHrss701 => SecurityLevel::Level1,
            NtruParameterSet::NtruHps2048677 => SecurityLevel::Level3,
            NtruParameterSet::NtruHps4096821 => SecurityLevel::Level5,
        }
    }

    pub fn public_key_size(&self) -> usize {
        match self {
            NtruParameterSet::NtruHps2048509 => 699,
            NtruParameterSet::NtruHps2048677 => 930,
            NtruParameterSet::NtruHps4096821 => 1230,
            NtruParameterSet::NtruHrss701 => 1138,
        }
    }

    pub fn secret_key_size(&self) -> usize {
        match self {
            NtruParameterSet::NtruHps2048509 => 935,
            NtruParameterSet::NtruHps2048677 => 1234,
            NtruParameterSet::NtruHps4096821 => 1590,
            NtruParameterSet::NtruHrss701 => 1450,
        }
    }

    pub fn ciphertext_size(&self) -> usize {
        match self {
            NtruParameterSet::NtruHps2048509 => 699,
            NtruParameterSet::NtruHps2048677 => 930,
            NtruParameterSet::NtruHps4096821 => 1230,
            NtruParameterSet::NtruHrss701 => 1138,
        }
    }
}

/// NTRU public key
#[derive(Debug, Clone)]
pub struct NtruPublicKey {
    pub parameter_set: NtruParameterSet,
    pub key_data: Vec<u8>,
}

/// NTRU secret key
#[derive(Debug, Clone)]
pub struct NtruSecretKey {
    pub parameter_set: NtruParameterSet,
    pub key_data: Vec<u8>,
}

/// NTRU Encryption implementation
pub struct NtruEncryption;

impl NtruEncryption {
    /// Generate an NTRU key pair
    pub fn keygen(security_level: SecurityLevel) -> PqcResult<(NtruPublicKey, NtruSecretKey)> {
        let parameter_set = match security_level {
            SecurityLevel::Level1 => NtruParameterSet::NtruHps2048509,
            SecurityLevel::Level3 => NtruParameterSet::NtruHps2048677,
            SecurityLevel::Level5 => NtruParameterSet::NtruHps4096821,
        };

        Self::keygen_with_params(parameter_set)
    }

    /// Generate an NTRU key pair with specific parameter set
    pub fn keygen_with_params(params: NtruParameterSet) -> PqcResult<(NtruPublicKey, NtruSecretKey)> {
        match params {
            NtruParameterSet::NtruHps2048509 => {
                let (pk, sk) = ntruhps2048509::keypair();
                let public_key = NtruPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = NtruSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            NtruParameterSet::NtruHps2048677 => {
                let (pk, sk) = ntruhps2048677::keypair();
                let public_key = NtruPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = NtruSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            NtruParameterSet::NtruHps4096821 => {
                let (pk, sk) = ntruhps4096821::keypair();
                let public_key = NtruPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = NtruSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
            NtruParameterSet::NtruHrss701 => {
                let (pk, sk) = ntruhrss701::keypair();
                let public_key = NtruPublicKey {
                    parameter_set: params,
                    key_data: pk.as_bytes().to_vec(),
                };
                let secret_key = NtruSecretKey {
                    parameter_set: params,
                    key_data: sk.as_bytes().to_vec(),
                };
                Ok((public_key, secret_key))
            },
        }
    }

    /// Encapsulate a shared secret using NTRU (KEM operation)
    pub fn encapsulate(public_key: &NtruPublicKey) -> PqcResult<(Vec<u8>, Vec<u8>)> {
        match public_key.parameter_set {
            NtruParameterSet::NtruHps2048509 => {
                let pk = ntruhps2048509::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-2048-509 public key".to_string()))?;
                let (ss, ct) = ntruhps2048509::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            },
            NtruParameterSet::NtruHps2048677 => {
                let pk = ntruhps2048677::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-2048-677 public key".to_string()))?;
                let (ss, ct) = ntruhps2048677::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            },
            NtruParameterSet::NtruHps4096821 => {
                let pk = ntruhps4096821::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-4096-821 public key".to_string()))?;
                let (ss, ct) = ntruhps4096821::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            },
            NtruParameterSet::NtruHrss701 => {
                let pk = ntruhrss701::PublicKey::from_bytes(&public_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HRSS-701 public key".to_string()))?;
                let (ss, ct) = ntruhrss701::encapsulate(&pk);
                Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
            },
        }
    }

    /// Decapsulate a shared secret using NTRU (KEM operation)
    pub fn decapsulate(secret_key: &NtruSecretKey, ciphertext: &[u8]) -> PqcResult<Vec<u8>> {
        let expected_size = secret_key.parameter_set.ciphertext_size();
        if ciphertext.len() != expected_size {
            return Err(PqcError::InvalidCiphertext(
                format!("Expected {} bytes, got {}", expected_size, ciphertext.len())
            ));
        }

        match secret_key.parameter_set {
            NtruParameterSet::NtruHps2048509 => {
                let sk = ntruhps2048509::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-2048-509 secret key".to_string()))?;
                let ct = ntruhps2048509::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid NTRU-HPS-2048-509 ciphertext".to_string()))?;
                let ss = ntruhps2048509::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            },
            NtruParameterSet::NtruHps2048677 => {
                let sk = ntruhps2048677::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-2048-677 secret key".to_string()))?;
                let ct = ntruhps2048677::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid NTRU-HPS-2048-677 ciphertext".to_string()))?;
                let ss = ntruhps2048677::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            },
            NtruParameterSet::NtruHps4096821 => {
                let sk = ntruhps4096821::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HPS-4096-821 secret key".to_string()))?;
                let ct = ntruhps4096821::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid NTRU-HPS-4096-821 ciphertext".to_string()))?;
                let ss = ntruhps4096821::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            },
            NtruParameterSet::NtruHrss701 => {
                let sk = ntruhrss701::SecretKey::from_bytes(&secret_key.key_data)
                    .map_err(|_| PqcError::InvalidKey("Invalid NTRU-HRSS-701 secret key".to_string()))?;
                let ct = ntruhrss701::Ciphertext::from_bytes(ciphertext)
                    .map_err(|_| PqcError::InvalidCiphertext("Invalid NTRU-HRSS-701 ciphertext".to_string()))?;
                let ss = ntruhrss701::decapsulate(&ct, &sk);
                Ok(ss.as_bytes().to_vec())
            },
        }
    }

    /// Legacy encryption method for backward compatibility
    /// (Now uses KEM with AES-GCM for actual message encryption)
    pub fn encrypt(public_key: &NtruPublicKey, plaintext: &[u8]) -> PqcResult<Vec<u8>> {
        // Use KEM for shared secret, then AES-GCM for actual encryption
        let (ciphertext, shared_secret) = Self::encapsulate(public_key)?;
        
        // Use the shared secret with AES-GCM to encrypt the plaintext
        
        let key = Key::from_slice(&shared_secret[..32]);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&shared_secret[..12]); // Use part of shared secret as nonce
        
        let encrypted_data = cipher.encrypt(nonce, plaintext)
            .map_err(|_| PqcError::EncryptionFailed("AES-GCM encryption failed".to_string()))?;
        
        // Prepend the ciphertext with the KEM ciphertext
        let mut result = ciphertext;
        result.extend_from_slice(&encrypted_data);
        Ok(result)
    }

    /// Legacy decryption method for backward compatibility  
    /// (Now uses KEM with AES-GCM for actual message decryption)
    pub fn decrypt(secret_key: &NtruSecretKey, ciphertext: &[u8]) -> PqcResult<Vec<u8>> {
        let kem_ciphertext_size = secret_key.parameter_set.ciphertext_size();
        if ciphertext.len() < kem_ciphertext_size {
            return Err(PqcError::InvalidCiphertext("Ciphertext too short".to_string()));
        }
        
        // Split KEM ciphertext and encrypted data
        let (kem_ciphertext, encrypted_data) = ciphertext.split_at(kem_ciphertext_size);
        
        // Decapsulate shared secret
        let shared_secret = Self::decapsulate(secret_key, kem_ciphertext)?;
        
        // Use the shared secret with AES-GCM to decrypt the data
        
        let key = Key::from_slice(&shared_secret[..32]);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&shared_secret[..12]); // Use part of shared secret as nonce
        
        let decrypted_data = cipher.decrypt(nonce, encrypted_data)
            .map_err(|_| PqcError::DecryptionFailed("AES-GCM decryption failed".to_string()))?;
        
        Ok(decrypted_data)
    }


}

// ============================================================================
// PERFORMANCE BENCHMARKING
// ============================================================================

/// Performance benchmarking utilities for PQC algorithms
pub struct PqcBenchmark;

impl PqcBenchmark {
    /// Benchmark Kyber KEM operations
    pub fn benchmark_kyber(security_level: SecurityLevel, iterations: usize) -> PqcResult<PerformanceMetrics> {
        let mut total_keygen_time = Duration::from_nanos(0);
        let mut total_encaps_time = Duration::from_nanos(0);
        let mut total_decaps_time = Duration::from_nanos(0);

        let parameter_set = match security_level {
            SecurityLevel::Level1 => KyberParameterSet::Kyber512,
            SecurityLevel::Level3 => KyberParameterSet::Kyber768,
            SecurityLevel::Level5 => KyberParameterSet::Kyber1024,
        };

        for _ in 0..iterations {
            // Benchmark key generation
            let keygen_start = Instant::now();
            let (public_key, secret_key) = KyberKem::keygen_with_params(parameter_set)?;
            total_keygen_time += keygen_start.elapsed();

            // Benchmark encapsulation
            let encaps_start = Instant::now();
            let (ciphertext, _shared_secret) = KyberKem::encaps(&public_key)?;
            total_encaps_time += encaps_start.elapsed();

            // Benchmark decapsulation
            let decaps_start = Instant::now();
            let _decaps_secret = KyberKem::decaps(&secret_key, &ciphertext)?;
            total_decaps_time += decaps_start.elapsed();
        }

        let avg_keygen_time = total_keygen_time / iterations as u32;
        let avg_operation_time = (total_encaps_time + total_decaps_time) / (iterations * 2) as u32;
        let operations_per_second = 1.0 / avg_operation_time.as_secs_f64();

        Ok(PerformanceMetrics {
            keygen_time: avg_keygen_time,
            operation_time: avg_operation_time,
            key_size: parameter_set.public_key_size() + parameter_set.secret_key_size(),
            ciphertext_size: parameter_set.ciphertext_size(),
            signature_size: None,
            operations_per_second,
        })
    }

    /// Benchmark Dilithium signature operations
    pub fn benchmark_dilithium(security_level: SecurityLevel, iterations: usize) -> PqcResult<PerformanceMetrics> {
        let mut total_keygen_time = Duration::from_nanos(0);
        let mut total_sign_time = Duration::from_nanos(0);
        let mut total_verify_time = Duration::from_nanos(0);

        let parameter_set = match security_level {
            SecurityLevel::Level1 => DilithiumParameterSet::Dilithium2,
            SecurityLevel::Level3 => DilithiumParameterSet::Dilithium3,
            SecurityLevel::Level5 => DilithiumParameterSet::Dilithium5,
        };

        let test_message = b"This is a test message for benchmarking Dilithium signatures";

        for _ in 0..iterations {
            // Benchmark key generation
            let keygen_start = Instant::now();
            let (public_key, secret_key) = DilithiumSignature::keygen_with_params(parameter_set)?;
            total_keygen_time += keygen_start.elapsed();

            // Benchmark signing
            let sign_start = Instant::now();
            let signature = DilithiumSignature::sign(&secret_key, test_message)?;
            total_sign_time += sign_start.elapsed();

            // Benchmark verification
            let verify_start = Instant::now();
            let _is_valid = DilithiumSignature::verify(&public_key, test_message, &signature)?;
            total_verify_time += verify_start.elapsed();
        }

        let avg_keygen_time = total_keygen_time / iterations as u32;
        let avg_operation_time = (total_sign_time + total_verify_time) / (iterations * 2) as u32;
        let operations_per_second = 1.0 / avg_operation_time.as_secs_f64();

        Ok(PerformanceMetrics {
            keygen_time: avg_keygen_time,
            operation_time: avg_operation_time,
            key_size: parameter_set.public_key_size() + parameter_set.secret_key_size(),
            ciphertext_size: 0,
            signature_size: Some(parameter_set.signature_size()),
            operations_per_second,
        })
    }

    /// Benchmark all PQC algorithms
    pub fn benchmark_all(iterations: usize) -> PqcResult<HashMap<String, PerformanceMetrics>> {
        let mut results = HashMap::new();

        // Benchmark Kyber
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let kyber_metrics = Self::benchmark_kyber(level, iterations)?;
            results.insert(format!("Kyber-{}", level.classical_bits()), kyber_metrics);
        }

        // Benchmark Dilithium
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let dilithium_metrics = Self::benchmark_dilithium(level, iterations)?;
            results.insert(format!("Dilithium-{}", level.classical_bits()), dilithium_metrics);
        }

        Ok(results)
    }
}

// ============================================================================
// QUANTUM RESISTANCE ASSESSMENT
// ============================================================================

/// Quantum resistance assessment utilities
pub struct QuantumResistanceAssessment;

impl QuantumResistanceAssessment {
    /// Assess the quantum resistance of all supported algorithms
    pub fn assess_all_algorithms() -> Vec<crate::stdlib::crypto::pqc::QuantumResistanceAssessment> {
        vec![
            Self::assess_kyber(),
            Self::assess_dilithium(),
            Self::assess_sphincs_plus(),
            Self::assess_falcon(),
            Self::assess_ntru(),
        ]
    }

    /// Assess Kyber quantum resistance
    pub fn assess_kyber() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
        crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            algorithm: AlgorithmType::Kyber,
            security_level: SecurityLevel::Level3,
            quantum_secure: true,
            estimated_quantum_break_time: "2^170 quantum operations".to_string(),
            classical_break_time: "2^192 classical operations".to_string(),
            key_size_overhead: 3.5, // Compared to classical ECDH
            performance_overhead: 2.1, // Compared to classical ECDH
            standardization_status: "NIST PQC Standard (2024)".to_string(),
        }
    }

    /// Assess Dilithium quantum resistance
    pub fn assess_dilithium() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
        crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            algorithm: AlgorithmType::Dilithium,
            security_level: SecurityLevel::Level3,
            quantum_secure: true,
            estimated_quantum_break_time: "2^170 quantum operations".to_string(),
            classical_break_time: "2^192 classical operations".to_string(),
            key_size_overhead: 15.2, // Compared to classical ECDSA
            performance_overhead: 3.8, // Compared to classical ECDSA
            standardization_status: "NIST PQC Standard (2024)".to_string(),
        }
    }

    /// Assess SPHINCS+ quantum resistance
    pub fn assess_sphincs_plus() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
        crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            algorithm: AlgorithmType::Sphincs,
            security_level: SecurityLevel::Level3,
            quantum_secure: true,
            estimated_quantum_break_time: "Secure against quantum attacks".to_string(),
            classical_break_time: "2^192 classical operations".to_string(),
            key_size_overhead: 2.1, // Smaller keys, large signatures
            performance_overhead: 25.7, // Much slower signing
            standardization_status: "NIST PQC Standard (2024)".to_string(),
        }
    }

    /// Assess Falcon quantum resistance
    pub fn assess_falcon() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
        crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            algorithm: AlgorithmType::Falcon,
            security_level: SecurityLevel::Level1,
            quantum_secure: true,
            estimated_quantum_break_time: "2^128 quantum operations".to_string(),
            classical_break_time: "2^128 classical operations".to_string(),
            key_size_overhead: 8.9, // Compared to classical ECDSA
            performance_overhead: 4.2, // Compared to classical ECDSA
            standardization_status: "NIST PQC Standard (2024)".to_string(),
        }
    }

    /// Assess NTRU quantum resistance
    pub fn assess_ntru() -> crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
        crate::stdlib::crypto::pqc::QuantumResistanceAssessment {
            algorithm: AlgorithmType::Ntru,
            security_level: SecurityLevel::Level1,
            quantum_secure: true,
            estimated_quantum_break_time: "2^128 quantum operations".to_string(),
            classical_break_time: "2^128 classical operations".to_string(),
            key_size_overhead: 4.7, // Compared to classical RSA
            performance_overhead: 1.8, // Compared to classical RSA
            standardization_status: "Under NIST evaluation".to_string(),
        }
    }

    /// Generate a comprehensive quantum readiness report
    pub fn generate_readiness_report() -> String {
        let assessments = Self::assess_all_algorithms();
        let mut report = String::new();
        
        report.push_str("# Post-Quantum Cryptography Readiness Report\n\n");
        report.push_str("This report provides an assessment of post-quantum cryptographic algorithms and their readiness for deployment.\n\n");
        
        for assessment in assessments {
            report.push_str(&format!("## {}\n\n", assessment.algorithm));
            report.push_str(&format!("- **Security Level**: {}\n", assessment.security_level.description()));
            report.push_str(&format!("- **Quantum Secure**: {}\n", if assessment.quantum_secure { "Yes" } else { "No" }));
            report.push_str(&format!("- **Estimated Quantum Break Time**: {}\n", assessment.estimated_quantum_break_time));
            report.push_str(&format!("- **Classical Break Time**: {}\n", assessment.classical_break_time));
            report.push_str(&format!("- **Key Size Overhead**: {:.1}x\n", assessment.key_size_overhead));
            report.push_str(&format!("- **Performance Overhead**: {:.1}x\n", assessment.performance_overhead));
            report.push_str(&format!("- **Standardization Status**: {}\n\n", assessment.standardization_status));
        }
        
        report.push_str("## Recommendations\n\n");
        report.push_str("1. **Immediate Deployment**: Kyber and Dilithium are ready for production use\n");
        report.push_str("2. **Hybrid Approach**: Consider combining classical and post-quantum algorithms during transition\n");
        report.push_str("3. **Performance Testing**: Benchmark algorithms in your specific environment\n");
        report.push_str("4. **Key Management**: Update key management systems for larger key sizes\n");
        report.push_str("5. **Regular Updates**: Monitor NIST PQC standardization progress\n\n");
        
        report
    }
}

// ============================================================================
// HELPER FUNCTIONS AND UTILITIES
// ============================================================================

/// Validate input parameters for PQC operations
pub fn validate_security_level(level: SecurityLevel) -> PqcResult<()> {
    // All security levels are valid
    Ok(())
}

/// Get recommended algorithm for specific use case
pub fn get_recommended_algorithm(use_case: &str, security_level: SecurityLevel) -> PqcResult<AlgorithmType> {
    match use_case.to_lowercase().as_str() {
        "kem" | "key_exchange" | "key_encapsulation" => Ok(AlgorithmType::Kyber),
        "signature" | "digital_signature" | "signing" => {
            match security_level {
                SecurityLevel::Level1 | SecurityLevel::Level3 => Ok(AlgorithmType::Dilithium),
                SecurityLevel::Level5 => Ok(AlgorithmType::Falcon),
            }
        },
        "hash_signature" | "stateless_signature" => Ok(AlgorithmType::Sphincs),
        "encryption" | "public_key_encryption" => Ok(AlgorithmType::Ntru),
        _ => Err(PqcError::UnsupportedParameters(format!("Unknown use case: {}", use_case))),
    }
}

/// Convert bytes to hexadecimal string for display
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}

/// Convert hexadecimal string to bytes
pub fn hex_to_bytes(hex: &str) -> PqcResult<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(PqcError::ParameterValidation("Hex string must have even length".to_string()));
    }
    
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for chunk in hex.as_bytes().chunks(2) {
        let hex_byte = std::str::from_utf8(chunk)
            .map_err(|_| PqcError::ParameterValidation("Invalid hex character".to_string()))?;
        let byte = u8::from_str_radix(hex_byte, 16)
            .map_err(|_| PqcError::ParameterValidation("Invalid hex digit".to_string()))?;
        bytes.push(byte);
    }
    
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_levels() {
        assert_eq!(SecurityLevel::Level1.classical_bits(), 128);
        assert_eq!(SecurityLevel::Level3.classical_bits(), 192);
        assert_eq!(SecurityLevel::Level5.classical_bits(), 256);
    }

    #[test]
    fn test_algorithm_display() {
        assert_eq!(format!("{}", AlgorithmType::Kyber), "Kyber");
        assert_eq!(format!("{}", AlgorithmType::Dilithium), "Dilithium");
        assert_eq!(format!("{}", AlgorithmType::Sphincs), "SPHINCS+");
        assert_eq!(format!("{}", AlgorithmType::Falcon), "Falcon");
        assert_eq!(format!("{}", AlgorithmType::Ntru), "NTRU");
    }

    #[test]
    fn test_hex_conversion() {
        let bytes = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let hex = bytes_to_hex(&bytes);
        assert_eq!(hex, "0123456789abcdef");
        
        let converted_bytes = hex_to_bytes(&hex).unwrap();
        assert_eq!(bytes, converted_bytes);
    }

    #[test]
    fn test_recommended_algorithms() {
        assert_eq!(get_recommended_algorithm("kem", SecurityLevel::Level3).unwrap(), AlgorithmType::Kyber);
        assert_eq!(get_recommended_algorithm("signature", SecurityLevel::Level3).unwrap(), AlgorithmType::Dilithium);
        assert_eq!(get_recommended_algorithm("hash_signature", SecurityLevel::Level1).unwrap(), AlgorithmType::Sphincs);
        assert_eq!(get_recommended_algorithm("encryption", SecurityLevel::Level1).unwrap(), AlgorithmType::Ntru);
    }
}
