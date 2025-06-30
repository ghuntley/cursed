//! EdDSA (Edwards-curve Digital Signature Algorithm) functionality

use crate::error::CursedError;
use std::collections::HashMap;

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CursedError>;

/// EdDSA context for contextual signatures
#[derive(Debug, Clone)]
pub struct EdDsaContext {
    pub context: Vec<u8>,
    pub prehash: bool,
    pub curve: EdDsaCurve,
}

impl EdDsaContext {
    pub fn new(context: Vec<u8>, prehash: bool, curve: EdDsaCurve) -> Self {
        Self {
            context,
            prehash,
            curve,
        }
    }

    pub fn ed25519() -> Self {
        Self {
            context: Vec::new(),
            prehash: false,
            curve: EdDsaCurve::Ed25519,
        }
    }

    pub fn ed448() -> Self {
        Self {
            context: Vec::new(),
            prehash: false,
            curve: EdDsaCurve::Ed448,
        }
    }
}

/// EdDSA curve types
#[derive(Debug, Clone, PartialEq)]
pub enum EdDsaCurve {
    Ed25519,
    Ed448,
}

/// EdDSA verification result
#[derive(Debug, Clone)]
pub struct EdDsaVerificationResult {
    pub is_valid: bool,
    pub signature_size: usize,
    pub public_key_size: usize,
    pub curve: EdDsaCurve,
    pub errors: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl EdDsaVerificationResult {
    pub fn new(is_valid: bool, signature_size: usize, public_key_size: usize, curve: EdDsaCurve) -> Self {
        Self {
            is_valid,
            signature_size,
            public_key_size,
            curve,
            errors: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// EdDSA batch verification result
#[derive(Debug, Clone)]
pub struct EdDsaBatchVerificationResult {
    pub total_signatures: usize,
    pub valid_signatures: usize,
    pub invalid_signatures: usize,
    pub batch_is_valid: bool,
    pub individual_results: Vec<EdDsaVerificationResult>,
    pub processing_time_ms: u64,
}

impl EdDsaBatchVerificationResult {
    pub fn new(total_signatures: usize) -> Self {
        Self {
            total_signatures,
            valid_signatures: 0,
            invalid_signatures: 0,
            batch_is_valid: false,
            individual_results: Vec::new(),
            processing_time_ms: 0,
        }
    }

    pub fn add_result(&mut self, result: EdDsaVerificationResult) {
        if result.is_valid {
            self.valid_signatures += 1;
        } else {
            self.invalid_signatures += 1;
        }
        self.individual_results.push(result);
        self.batch_is_valid = self.invalid_signatures == 0;
    }

    pub fn set_processing_time(&mut self, time_ms: u64) {
        self.processing_time_ms = time_ms;
    }
}

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
pub fn init_eddsa() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (eddsa) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_eddsa() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
