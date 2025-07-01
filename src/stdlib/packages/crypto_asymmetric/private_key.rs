//! Cryptographic functionality for private_key

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_private_key() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (private_key) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_private_key() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}



// Private Key specific types
#[derive(Debug, Clone)]
pub struct PrivateKeyEngine {
    pub key_type: PrivateKeyType,
}

#[derive(Debug, Clone)]
pub enum PrivateKeyType {
    RSA,
    ECC,
    Ed25519,
    X25519,
}

#[derive(Debug, Clone)]
pub struct PrivateKeyInfo {
    pub algorithm: String,
    pub key_size: usize,
    pub key_data: Vec<u8>,
    pub is_encrypted: bool,
}

#[derive(Debug, Clone)]
pub enum PrivateKeyError {
    InvalidFormat,
    DecryptionFailed,
    UnsupportedAlgorithm,
}
