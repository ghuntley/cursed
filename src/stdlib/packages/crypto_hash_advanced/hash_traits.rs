//! Cryptographic functionality for hash_traits

use crate::error::CursedError;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// Cryptographic operations handler
/// Hash algorithm information
#[derive(Debug, Clone)]
pub struct HashAlgorithmInfo {
    pub name: String,
    pub block_size: usize,
    pub output_size: usize,
    pub is_secure: bool,
}

/// Hash registry for managing hash algorithms
pub struct HashRegistry {
    algorithms: Vec<HashAlgorithmInfo>,
}

impl HashRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: Vec::new(),
        };
        registry.register_defaults();
        registry
    }
    
    fn register_defaults(&mut self) {
        self.algorithms.push(HashAlgorithmInfo {
            name: "SHA-256".to_string(),
            block_size: 64,
            output_size: 32,
            is_secure: true,
        });
        self.algorithms.push(HashAlgorithmInfo {
            name: "BLAKE3".to_string(),
            block_size: 64,
            output_size: 32,
            is_secure: true,
        });
        self.algorithms.push(HashAlgorithmInfo {
            name: "SHA-3".to_string(),
            block_size: 144,
            output_size: 32,
            is_secure: true,
        });
    }
    
    pub fn get_algorithms(&self) -> &[HashAlgorithmInfo] {
        &self.algorithms
    }
}

/// Initialize crypto processing
pub fn init_hash_traits() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (hash_traits) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_hash_traits() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
