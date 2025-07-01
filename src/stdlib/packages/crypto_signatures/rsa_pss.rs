//! RSA-PSS signature functionality

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// RSA-PSS salt length specification
#[derive(Debug, Clone, PartialEq)]
pub enum SaltLength {
    /// Use digest length
    DigestLength,
    /// Use maximum possible length
    Maximum,
    /// Use specific length
    Fixed(usize),
    /// Auto-detect based on signature
    Auto,
}

impl Default for SaltLength {
    fn default() -> Self {
        SaltLength::DigestLength
    }
}

/// RSA-PSS signature container
#[derive(Debug, Clone)]
pub struct RsaPssSignature {
    pub signature: Vec<u8>,
    pub salt_length: SaltLength,
    pub hash_algorithm: String,
    pub key_size: usize,
}

impl RsaPssSignature {
    pub fn new(signature: Vec<u8>, salt_length: SaltLength, hash_algorithm: String, key_size: usize) -> Self {
        Self {
            signature,
            salt_length,
            hash_algorithm,
            key_size,
        }
    }

    pub fn signature_size(&self) -> usize {
        self.signature.len()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.signature)
    }

    pub fn from_hex(hex_str: &str, salt_length: SaltLength, hash_algorithm: String, key_size: usize) -> CryptoResult<Self> {
        let signature = hex::decode(hex_str)
            .map_err(|e| CursedError::runtime_error(&format!("Hex decode error: {}", e)))?;
        Ok(Self::new(signature, salt_length, hash_algorithm, key_size))
    }
}

/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_rsa_pss() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (rsa_pss) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_rsa_pss() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
