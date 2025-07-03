//! EdDSA (Edwards-curve Digital Signature Algorithm) functionality

use crate::error::CursedError;
use std::collections::HashMap;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
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
/// Initialize crypto processing
pub fn init_eddsa() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
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
        return Err(CursedError::runtime_error(&"Crypto hash test failed".to_string()));
    }
    Ok(())
}
