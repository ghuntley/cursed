/// Production-ready Advanced Cryptographic Hash Functions Module
pub mod algorithms;
pub mod blake3;
pub mod collision_resistance;
pub mod hash_functions;
pub mod hash_traits;
pub mod hash_validation;
pub mod hmac;
pub mod hmac_variants;
pub mod keccak;
pub mod password_hashing;
pub mod performance_analysis;
pub mod sha3;
pub mod siphash;
pub mod tree_hashing;
pub mod xxhash;

// Re-export core types and traits
pub use hash_traits::*;
pub use collision_resistance::*;
pub use hash_validation::*;
pub use tree_hashing::*;
pub use password_hashing::*;
pub use performance_analysis::*;

// Re-export algorithm implementations
pub use blake3::*;
pub use keccak::*;
pub use sha3::*;
pub use xxhash::*;
pub use siphash::*;
pub use hmac::*;
pub use hmac_variants::*;

use crate::error::CursedError;

/// Initialize advanced cryptographic hash package with comprehensive functionality
pub fn init_crypto_hash_advanced() -> crate::error::Result<()> {
    println!("🔒 Initializing CURSED Advanced Cryptographic Hash Package...");
    
    // Test core hash implementations
    let _sha3_hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Sha3_256);
    let _blake3_hasher = blake3::Blake3Hasher::new();
    let _keccak_hasher = keccak::KeccakHasher::keccak256();
    let _xxhash_hasher = xxhash::XxHash64::new();
    let _siphash_hasher = siphash::SipHash::new(&[0u8; 16]);
    
    // Test HMAC implementation
    let _hmac_engine = hmac::HmacEngine::new(hmac::HmacAlgorithm::Sha256, b"test_key");
    
    // Test password hashing
    let _password_hasher = password_hashing::PasswordHasher::with_defaults();
    
    // Initialize hash registry
    let _registry = hash_traits::HashRegistry::new();
    
    // Test collision resistance analyzer
    let _collision_analyzer = collision_resistance::CollisionAnalyzer::new();
    
    // Test hash validator
    let _hash_validator = hash_validation::MultiHashValidator::new();
    
    // Test Merkle tree
    let _merkle_tree = tree_hashing::MerkleTree::new(xxhash::XxHash64::new());
    
    println!("✅ Advanced hash package successfully initialized!");
    println!("   📊 Available algorithms: SHA-3, BLAKE3, Keccak, xxHash, SipHash");
    println!("   🔐 Password hashing: Argon2, scrypt, PBKDF2");
    println!("   🌳 Tree hashing: Merkle trees, binary hash trees");
    println!("   🛡️  Security analysis: collision detection, validation");
    println!("   ⚡ Performance: benchmarking and optimization");
    
    Ok(())
}

/// Get comprehensive hash algorithm information
pub fn get_supported_algorithms() -> Vec<hash_traits::HashAlgorithmInfo> {
    let registry = hash_traits::HashRegistry::new();
    registry.list_algorithms().to_vec()
}

/// Quick hash function for common use cases
pub fn quick_hash(algorithm: &str, data: &[u8]) -> crate::error::Result<()> {
    match algorithm.to_lowercase().as_str() {
        "blake3" => {
            let mut hasher = blake3::Blake3Hasher::new();
            Ok(hasher.hash(data))
        },
        "sha3-256" => {
            let mut hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Sha3_256);
            Ok(hasher.hash(data))
        },
        "keccak256" => {
            Ok(keccak::keccak256(data))
        },
        "xxhash64" => {
            let mut hasher = xxhash::XxHash64::new();
            Ok(hasher.hash(data))
        },
        "siphash" => {
            let mut hasher = siphash::SipHash::new(&[0u8; 16]);
            Ok(hasher.hash(data))
        },
        _ => Err(CursedError::InvalidArgument(format!("Unknown hash algorithm: {}", algorithm))),
    }
}

/// Hash a password with secure defaults
pub fn hash_password(password: &str) -> crate::error::Result<()> {
    let hasher = password_hashing::PasswordHasher::with_defaults();
    hasher.hash_password(password)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &password_hashing::PasswordHash) -> crate::error::Result<()> {
    let hasher = password_hashing::PasswordHasher::new(hash.config.clone());
    hasher.verify_password(password, hash)
}
