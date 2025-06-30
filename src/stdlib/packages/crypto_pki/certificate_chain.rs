//! Cryptographic functionality for certificate_chain

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

/// Initialize crypto processing
pub fn init_certificate_chain() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (certificate_chain) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_certificate_chain() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}

// Certificate chain specific types
#[derive(Debug, Clone)]
pub enum ChainError {
    InvalidChain,
    TrustAnchorNotFound,
    ValidationFailed,
    ExpiredCertificate,
}

pub type ChainResult<T> = Result<T, CursedError>;

#[derive(Debug, Clone)]
pub struct ChainValidationPolicy {
    pub max_depth: usize,
    pub allow_self_signed: bool,
    pub check_revocation: bool,
    pub require_basic_constraints: bool,
}

impl Default for ChainValidationPolicy {
    fn default() -> Self {
        Self {
            max_depth: 10,
            allow_self_signed: false,
            check_revocation: true,
            require_basic_constraints: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChainConstraints {
    pub max_path_length: Option<usize>,
    pub permitted_subtrees: Vec<String>,
    pub excluded_subtrees: Vec<String>,
}

impl Default for ChainConstraints {
    fn default() -> Self {
        Self {
            max_path_length: None,
            permitted_subtrees: vec![],
            excluded_subtrees: vec![],
        }
    }
}
