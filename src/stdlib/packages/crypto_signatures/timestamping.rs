//! Timestamping functionality for cryptographic signatures

use crate::error::CursedError;
use std::collections::HashMap;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
/// Timestamp validation policy
#[derive(Debug, Clone)]
pub struct TimestampValidationPolicy {
    pub require_rfc3161: bool,
    pub max_age_seconds: u64,
    pub allowed_tsa_sources: Vec<String>,
    pub verify_chain: bool,
    pub check_revocation: bool,
}

impl Default for TimestampValidationPolicy {
    fn default() -> Self {
        Self {
            require_rfc3161: true,
            max_age_seconds: 86400 * 30, // 30 days
            allowed_tsa_sources: Vec::new(),
            verify_chain: true,
            check_revocation: true,
        }
    }
}

/// Timestamp verification result
#[derive(Debug, Clone)]
pub struct TimestampVerificationResult {
    pub is_valid: bool,
    pub timestamp: String,
    pub tsa_source: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl TimestampVerificationResult {
    pub fn new(is_valid: bool, timestamp: String, tsa_source: String) -> Self {
        Self {
            is_valid,
            timestamp,
            tsa_source,
            errors: Vec::new(),
            warnings: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_timestamping() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    println!("🔐 Crypto processing (timestamping) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_timestamping() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error(&"Crypto hash test failed".to_string()));
    }
    Ok(())
}
