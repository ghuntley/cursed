//! Cryptographic functionality for key_exchange

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// Cryptographic operations handler
/// X25519 key pair generation - DISABLED FOR SECURITY
pub fn x25519_generate_keypair(_seed: Vec<u8>) -> CryptoResult<(Vec<u8>, Vec<u8>)> {
    Err(CursedError::runtime_error(
        "SECURITY ERROR: X25519 key generation disabled due to unsafe placeholder implementation. \
        The previous implementation used public_key.reverse() which is cryptographically insecure."
    ))
}

/// X448 key pair generation - DISABLED FOR SECURITY
pub fn x448_generate_keypair(_seed: Vec<u8>) -> CryptoResult<(Vec<u8>, Vec<u8>)> {
    Err(CursedError::runtime_error(
        "SECURITY ERROR: X448 key generation disabled due to unsafe placeholder implementation. \
        The previous implementation used public_key.reverse() which is cryptographically insecure."
    ))
}

/// Diffie-Hellman key pair generation - DISABLED FOR SECURITY
pub fn dh_generate_keypair(_params: Vec<u8>) -> CryptoResult<(Vec<u8>, Vec<u8>)> {
    Err(CursedError::runtime_error(
        "SECURITY ERROR: Diffie-Hellman key generation disabled due to unsafe placeholder implementation. \
        The previous implementation used public_key.reverse() which is cryptographically insecure."
    ))
}

/// Initialize crypto processing
pub fn init_key_exchange() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (key_exchange) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_key_exchange() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}

// Additional key exchange functions
pub fn validate_key_exchange_params(params: &[u8]) -> crate::error::Result<bool> {
    Ok(!params.is_empty())
}

pub fn list_key_exchange_algorithms() -> Vec<String> {
    vec!["ECDH".to_string(), "DH".to_string(), "X25519".to_string()]
}

pub fn derive_key_from_shared_secret(shared_secret: &[u8], length: usize) -> crate::error::Result<Vec<u8>> {
    if shared_secret.is_empty() {
        return Err(CursedError::validation_error("Empty shared secret"));
    }
    Ok(shared_secret[..std::cmp::min(length, shared_secret.len())].to_vec())
}
