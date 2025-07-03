//! Cryptographic functionality for ocsp_client

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_ocsp_client() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    println!("🔐 Crypto processing (ocsp_client) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_ocsp_client() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CryptoError::Other("Crypto hash test failed".to_string().into()).into());
    }
    Ok(())
}

// OCSP Client specific types
#[derive(Debug, Clone)]
pub struct OcspClient {
    pub endpoint: String,
    pub timeout: u32,
}

impl OcspClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            timeout: 30,
        }
    }
}
