//! Cryptographic functionality for key_agreement

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_key_agreement() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (key_agreement) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_key_agreement() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}



// Key Agreement additional functions
pub fn derive_key_from_shared_secret(shared_secret: &[u8], info: &[u8], length: usize) -> crate::error::Result<Vec<u8>> {
    if shared_secret.is_empty() {
        return Err(CursedError::validation_error("Empty shared secret"));
    }
    Ok(shared_secret[..std::cmp::min(length, shared_secret.len())].to_vec())
}
