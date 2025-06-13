/// fr fr Advanced hash functions module with real implementations
pub mod sha3;
pub mod blake3;
pub mod hmac;

pub use sha3::*;
pub use blake3::*;
pub use hmac::*;

use crate::error::CursedError;

/// fr fr Initialize advanced hash package
pub fn init_crypto_hash_advanced() -> Result<(), CursedError> {
    // Test SHA-3 implementation
    let _sha3_hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Sha3_256);
    
    // Test BLAKE3 implementation
    let _blake3_hasher = blake3::Blake3Hasher::new();
    
    // Test HMAC implementation
    let _hmac_engine = hmac::HmacEngine::new(hmac::HmacAlgorithm::Sha256, b"test_key");
    
    println!("🔒 Advanced hash package initialized with SHA-3, BLAKE3, HMAC support!");
    Ok(())
}
