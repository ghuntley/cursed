//! Cryptographic functionality for key_exchange

use crate::error::CursedError;

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CursedError>;

/// Cryptographic operations handler
pub struct CryptoHandler {
    key_size: usize,
}

impl CryptoHandler {
    /// Create a new crypto handler
    pub fn new() -> Self {
        Self {
            key_size: 32,
        }
    }
    
    /// Set key size
    pub fn key_size(mut self, size: usize) -> Self {
        self.key_size = size;
        self
    }
    
    /// Generate random bytes
    pub fn random_bytes(&self, size: usize) -> CryptoResult<Vec<u8>> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; size];
        rng.fill_bytes(&mut bytes);
        Ok(bytes)
    }
    
    /// Hash data using SHA-256
    pub fn hash_sha256(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
    
    /// Generate a key
    pub fn generate_key(&self) -> CryptoResult<Vec<u8>> {
        self.random_bytes(self.key_size)
    }
    
    /// Encode to hex
    pub fn to_hex(&self, data: &[u8]) -> String {
        hex::encode(data)
    }
    
    /// Decode from hex
    pub fn from_hex(&self, hex_str: &str) -> CryptoResult<Vec<u8>> {
        hex::decode(hex_str).map_err(|e| CursedError::runtime_error(&format!("Hex decode error: {}", e)))
    }
}

impl Default for CryptoHandler {
    fn default() -> Self {
        Self::new()
    }
}

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
