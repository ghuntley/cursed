//! Cryptographic functionality for ocsp

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_ocsp() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    println!("🔐 Crypto processing (ocsp) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_ocsp() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CryptoError::Other("Crypto hash test failed".to_string().into()).into());
    }
    Ok(())
}

// OCSP specific types
#[derive(Debug, Clone)]
pub enum OcspError {
    RequestFailed,
    InvalidResponse,
    Expired,
}

pub type OcspResult<T> = Result<T, CursedError>;

#[derive(Debug, Clone)]
pub struct OcspCache {
    pub entries: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OcspValidator {
    pub cache: OcspCache,
}
