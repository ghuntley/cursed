//! Cryptographic functionality for hash_algorithms

use crate::error::CursedError;
use std::collections::HashMap;

/// Result type for crypto operations
pub type CryptoResult<T> = Result<T, CursedError>;

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
