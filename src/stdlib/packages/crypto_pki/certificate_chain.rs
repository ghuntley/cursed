//! Cryptographic functionality for certificate_chain

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_certificate_chain() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
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
        return Err(CryptoError::Other("Crypto hash test failed".to_string().into()).into());
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
