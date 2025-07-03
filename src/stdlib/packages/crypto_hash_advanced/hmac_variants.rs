//! Cryptographic functionality for hmac_variants

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_hmac_variants() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    println!("🔐 Crypto processing (hmac_variants) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_hmac_variants() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    Ok(())
}
