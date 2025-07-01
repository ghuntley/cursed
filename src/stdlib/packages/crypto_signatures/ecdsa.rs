//! Cryptographic functionality for ecdsa

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_ecdsa() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (ecdsa) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_ecdsa() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}

// ECDSA constants  
pub const ECDSA_SIGNATURE_SIZE: usize = 64; // For P-256
pub const ECDSA_PRIVATE_KEY_SIZE: usize = 32; // For P-256
pub const ECDSA_PUBLIC_KEY_SIZE: usize = 64; // For P-256 uncompressed
