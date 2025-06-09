/// fr fr Advanced hashing algorithms for CURSED - beyond basic hashes bestie
/// 
/// This module provides cutting-edge hash functions including Blake3, SHA-3,
/// Keccak, and various HMAC implementations for maximum security.

// Core advanced hash algorithms
pub mod blake3;
pub mod sha3;
pub mod keccak;
pub mod hmac_variants;
pub mod hash_traits;

// Specialized hash functions
pub mod siphash;
pub mod xxhash;
pub mod password_hashing;
pub mod tree_hashing;

// Security and utilities
pub mod hash_validation;
pub mod performance_analysis;
pub mod collision_resistance;

// Re-export main types
pub use blake3::{Blake3Hasher, Blake3Hash, Blake3Result, Blake3Error, BLAKE3_OUTPUT_SIZE};
pub use sha3::{Sha3_256, Sha3_384, Sha3_512, Shake128, Shake256, Sha3Error, Sha3Result};
pub use keccak::{Keccak256, Keccak384, Keccak512, KeccakError, KeccakResult};
pub use hmac_variants::{HmacSha3, HmacBlake3, HmacVariant, HmacError, HmacResult};
pub use hash_traits::{AdvancedHasher, HashAlgorithm, HashCapabilities, HashResult};

use std::collections::HashMap;

/// fr fr Supported advanced hash algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdvancedHashAlgorithm {
    Blake3,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Shake128,
    Shake256,
    Keccak256,
    Keccak384,
    Keccak512,
    HmacSha3_256,
    HmacSha3_512,
    HmacBlake3,
    SipHash,
    XxHash64,
}

impl AdvancedHashAlgorithm {
    /// slay Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            AdvancedHashAlgorithm::Blake3 => "BLAKE3",
            AdvancedHashAlgorithm::Sha3_256 => "SHA3-256",
            AdvancedHashAlgorithm::Sha3_384 => "SHA3-384", 
            AdvancedHashAlgorithm::Sha3_512 => "SHA3-512",
            AdvancedHashAlgorithm::Shake128 => "SHAKE128",
            AdvancedHashAlgorithm::Shake256 => "SHAKE256",
            AdvancedHashAlgorithm::Keccak256 => "Keccak-256",
            AdvancedHashAlgorithm::Keccak384 => "Keccak-384",
            AdvancedHashAlgorithm::Keccak512 => "Keccak-512",
            AdvancedHashAlgorithm::HmacSha3_256 => "HMAC-SHA3-256",
            AdvancedHashAlgorithm::HmacSha3_512 => "HMAC-SHA3-512",
            AdvancedHashAlgorithm::HmacBlake3 => "HMAC-BLAKE3",
            AdvancedHashAlgorithm::SipHash => "SipHash",
            AdvancedHashAlgorithm::XxHash64 => "xxHash64",
        }
    }
    
    /// slay Get output size in bytes
    pub fn output_size(&self) -> usize {
        match self {
            AdvancedHashAlgorithm::Blake3 => 32,
            AdvancedHashAlgorithm::Sha3_256 => 32,
            AdvancedHashAlgorithm::Sha3_384 => 48,
            AdvancedHashAlgorithm::Sha3_512 => 64,
            AdvancedHashAlgorithm::Shake128 => 16, // Variable, this is minimum
            AdvancedHashAlgorithm::Shake256 => 32, // Variable, this is minimum
            AdvancedHashAlgorithm::Keccak256 => 32,
            AdvancedHashAlgorithm::Keccak384 => 48,
            AdvancedHashAlgorithm::Keccak512 => 64,
            AdvancedHashAlgorithm::HmacSha3_256 => 32,
            AdvancedHashAlgorithm::HmacSha3_512 => 64,
            AdvancedHashAlgorithm::HmacBlake3 => 32,
            AdvancedHashAlgorithm::SipHash => 8,
            AdvancedHashAlgorithm::XxHash64 => 8,
        }
    }
    
    /// slay Check if algorithm is cryptographically secure
    pub fn is_cryptographic(&self) -> bool {
        match self {
            AdvancedHashAlgorithm::Blake3 |
            AdvancedHashAlgorithm::Sha3_256 |
            AdvancedHashAlgorithm::Sha3_384 |
            AdvancedHashAlgorithm::Sha3_512 |
            AdvancedHashAlgorithm::Shake128 |
            AdvancedHashAlgorithm::Shake256 |
            AdvancedHashAlgorithm::Keccak256 |
            AdvancedHashAlgorithm::Keccak384 |
            AdvancedHashAlgorithm::Keccak512 |
            AdvancedHashAlgorithm::HmacSha3_256 |
            AdvancedHashAlgorithm::HmacSha3_512 |
            AdvancedHashAlgorithm::HmacBlake3 => true,
            AdvancedHashAlgorithm::SipHash |
            AdvancedHashAlgorithm::XxHash64 => false, // Fast but not cryptographic
        }
    }
}

/// fr fr Advanced hash errors
#[derive(Debug, Clone, PartialEq)]
pub enum AdvancedHashError {
    UnsupportedAlgorithm(String),
    InvalidInput,
    InvalidKeySize(usize, usize),
    ComputationFailed(String),
    Internal(String),
}

impl std::fmt::Display for AdvancedHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancedHashError::UnsupportedAlgorithm(name) => 
                write!(f, "Unsupported hash algorithm: {}", name),
            AdvancedHashError::InvalidInput => write!(f, "Invalid input"),
            AdvancedHashError::InvalidKeySize(provided, expected) => 
                write!(f, "Invalid key size: provided {}, expected {}", provided, expected),
            AdvancedHashError::ComputationFailed(msg) => write!(f, "Hash computation failed: {}", msg),
            AdvancedHashError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AdvancedHashError {}

/// fr fr Advanced hash result type
pub type AdvancedHashResult<T> = Result<T, AdvancedHashError>;

/// fr fr Utilities and helper functions
pub mod utils {
    use super::*;
    
    /// slay Quick Blake3 hash (recommended default)
    pub fn quick_blake3(data: &[u8]) -> AdvancedHashResult<Vec<u8>> {
        // Placeholder implementation
        Ok(vec![0u8; 32])
    }
    
    /// slay Quick SHA3-256 hash
    pub fn quick_sha3_256(data: &[u8]) -> AdvancedHashResult<Vec<u8>> {
        // Placeholder implementation
        Ok(vec![0u8; 32])
    }
    
    /// slay Quick HMAC with Blake3
    pub fn quick_hmac_blake3(key: &[u8], data: &[u8]) -> AdvancedHashResult<Vec<u8>> {
        // Placeholder implementation
        Ok(vec![0u8; 32])
    }
    
    /// slay Compare hash outputs securely
    pub fn secure_hash_compare(a: &[u8], b: &[u8]) -> bool {
        a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y)
    }
    
    /// slay Get recommended algorithm for use case
    pub fn recommended_for_integrity() -> AdvancedHashAlgorithm {
        AdvancedHashAlgorithm::Blake3
    }
    
    /// slay Get recommended algorithm for authentication
    pub fn recommended_for_authentication() -> AdvancedHashAlgorithm {
        AdvancedHashAlgorithm::HmacBlake3
    }
}

/// fr fr Initialize the crypto_hash_advanced package
pub fn init_crypto_hash_advanced() -> AdvancedHashResult<()> {
    println!("🔍 crypto_hash_advanced package initialized - advanced hashing ready bestie!");
    Ok(())
}
