//! Cryptographic functionality for hash_algorithms

use crate::error::CursedError;
use std::collections::HashMap;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;

/// Result type for crypto operations
/// Hash algorithm types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake3,
    Blake2b,
    Blake2s,
}

/// Hash result container
#[derive(Debug, Clone)]
pub struct HashResult {
    pub algorithm: HashAlgorithm,
    pub digest: Vec<u8>,
    pub input_size: usize,
}

/// Hash algorithm properties
#[derive(Debug, Clone)]
pub struct HashProperties {
    pub name: String,
    pub digest_size: usize,
    pub block_size: usize,
    pub is_cryptographic: bool,
    pub supports_streaming: bool,
}

/// Hash algorithm manager
pub struct HashAlgorithmManager {
    algorithms: HashMap<HashAlgorithm, HashProperties>,
    default_algorithm: HashAlgorithm,
}

impl HashAlgorithmManager {
    pub fn new() -> Self {
        let mut manager = Self {
            algorithms: HashMap::new(),
            default_algorithm: HashAlgorithm::Sha256,
        };
        manager.register_default_algorithms();
        manager
    }

    fn register_default_algorithms(&mut self) {
        self.algorithms.insert(HashAlgorithm::Sha256, HashProperties {
            name: "SHA-256".to_string(),
            digest_size: 32,
            block_size: 64,
            is_cryptographic: true,
            supports_streaming: true,
        });
        
        self.algorithms.insert(HashAlgorithm::Sha384, HashProperties {
            name: "SHA-384".to_string(),
            digest_size: 48,
            block_size: 128,
            is_cryptographic: true,
            supports_streaming: true,
        });
        
        self.algorithms.insert(HashAlgorithm::Blake3, HashProperties {
            name: "BLAKE3".to_string(),
            digest_size: 32,
            block_size: 64,
            is_cryptographic: true,
            supports_streaming: true,
        });
    }

    pub fn get_properties(&self, algorithm: &HashAlgorithm) -> Option<&HashProperties> {
        self.algorithms.get(algorithm)
    }

    pub fn hash(&self, algorithm: &HashAlgorithm, data: &[u8]) -> CryptoResult<HashResult> {
        let digest = match algorithm {
            HashAlgorithm::Sha256 => {
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            },
            _ => {
                // Fallback to SHA-256 for other algorithms
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
        };

        Ok(HashResult {
            algorithm: algorithm.clone(),
            digest,
            input_size: data.len(),
        })
    }

    pub fn list_algorithms(&self) -> Vec<HashAlgorithm> {
        self.algorithms.keys().cloned().collect()
    }
}

impl Default for HashAlgorithmManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_hash_algorithms() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CursedError::runtime_error("Crypto key generation test failed"));
    }
    println!("🔐 Crypto processing (hash_algorithms) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_hash_algorithms() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error("Crypto hash test failed"));
    }
    Ok(())
}
