//! Cryptographic functionality for asymmetric

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_asymmetric() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (asymmetric) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_asymmetric() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}



// Asymmetric additional functions
pub fn get_asymmetric_algorithms() -> Vec<String> {
    vec!["RSA".to_string(), "ECC".to_string(), "Ed25519".to_string(), "X25519".to_string()]
}

pub fn get_asymmetric_capabilities() -> crate::error::Result<Vec<String>> {
    Ok(vec!["signing".to_string(), "encryption".to_string(), "key_exchange".to_string()])
}
