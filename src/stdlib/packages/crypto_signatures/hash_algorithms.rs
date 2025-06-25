// Production-ready Hash Algorithms for Digital Signatures
// 
// Comprehensive hash algorithm implementations optimized for signature operations,
// with support for multiple algorithms, streaming, and security validation.

// use crate::stdlib::packages::crypto_signatures::errors::{SignatureError, SignatureResult};
use crate::error::CursedError;
use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use blake3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Supported hash algorithms for signatures
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-2 family
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    
    /// SHA-3 family
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    
    /// BLAKE3 (modern, fast)
    Blake3,
    
    /// Legacy algorithms (discouraged)
    Sha1,
    Md5,
}

/// Hash algorithm properties
#[derive(Debug, Clone)]
pub struct HashProperties {
    pub algorithm: HashAlgorithm,
    pub digest_size: usize,
    pub block_size: usize,
    pub security_level: u32,
    pub is_quantum_resistant: bool,
    pub recommended: bool,
}

/// Hash computation context for streaming
pub enum HashContext {
    Sha224(Sha224),
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
    Sha3_224(Sha3_224),
    Sha3_256(Sha3_256),
    Sha3_384(Sha3_384),
    Sha3_512(Sha3_512),
    Blake3(blake3::Hasher),
    Sha1(sha1::Sha1),
    Md5(md5::Digest),
}

/// Hash result with metadata
#[derive(Debug, Clone)]
pub struct HashResult {
    pub digest: Vec<u8>,
    pub algorithm: HashAlgorithm,
    pub input_size: usize,
}

/// Production-ready hash algorithm manager
pub struct HashAlgorithmManager {
    properties: HashMap<HashAlgorithm, HashProperties>,
    default_algorithm: HashAlgorithm,
}

impl HashAlgorithmManager {
    /// Create a new hash algorithm manager
    pub fn new() -> Self {
        let mut manager = Self {
            properties: HashMap::new(),
            default_algorithm: HashAlgorithm::Sha256,
        };
        
        manager.initialize_properties();
        manager
    }

    /// Set default hash algorithm
    pub fn set_default(&mut self, algorithm: HashAlgorithm) {
        self.default_algorithm = algorithm;
    }

    /// Get default hash algorithm
    pub fn get_default(&self) -> &HashAlgorithm {
        &self.default_algorithm
    }

    /// Compute hash using default algorithm
    pub fn hash(&self, data: &[u8]) -> SignatureResult<HashResult> {
        self.hash_with_algorithm(data, &self.default_algorithm)
    }

    /// Compute hash using specified algorithm
    pub fn hash_with_algorithm(&self, data: &[u8], algorithm: &HashAlgorithm) -> SignatureResult<HashResult> {
        let digest = match algorithm {
            HashAlgorithm::Sha224 => {
                let mut hasher = Sha224::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha384 => {
                let mut hasher = Sha384::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha3_224 => {
                let mut hasher = Sha3_224::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha3_256 => {
                let mut hasher = Sha3_256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha3_384 => {
                let mut hasher = Sha3_384::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha3_512 => {
                let mut hasher = Sha3_512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Blake3 => {
                blake3::hash(data).as_bytes().to_vec()
            }
            HashAlgorithm::Sha1 => {
                use sha1::Digest;
                let mut hasher = sha1::Sha1::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Md5 => {
                use md5::Digest;
                let mut hasher = md5::Digest::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
        };

        Ok(HashResult {
            digest,
            algorithm: algorithm.clone(),
            input_size: data.len(),
        })
    }

    /// Create streaming hash context
    pub fn create_context(&self, algorithm: &HashAlgorithm) -> SignatureResult<HashContext> {
        let context = match algorithm {
            HashAlgorithm::Sha224 => HashContext::Sha224(Sha224::new()),
            HashAlgorithm::Sha256 => HashContext::Sha256(Sha256::new()),
            HashAlgorithm::Sha384 => HashContext::Sha384(Sha384::new()),
            HashAlgorithm::Sha512 => HashContext::Sha512(Sha512::new()),
            HashAlgorithm::Sha3_224 => HashContext::Sha3_224(Sha3_224::new()),
            HashAlgorithm::Sha3_256 => HashContext::Sha3_256(Sha3_256::new()),
            HashAlgorithm::Sha3_384 => HashContext::Sha3_384(Sha3_384::new()),
            HashAlgorithm::Sha3_512 => HashContext::Sha3_512(Sha3_512::new()),
            HashAlgorithm::Blake3 => HashContext::Blake3(blake3::Hasher::new()),
            HashAlgorithm::Sha1 => HashContext::Sha1(sha1::Sha1::new()),
            HashAlgorithm::Md5 => HashContext::Md5(md5::Digest::new()),
        };

        Ok(context)
    }

    /// Update streaming hash context
    pub fn update_context(&self, context: &mut HashContext, data: &[u8]) -> SignatureResult<()> {
        match context {
            HashContext::Sha224(ref mut hasher) => hasher.update(data),
            HashContext::Sha256(ref mut hasher) => hasher.update(data),
            HashContext::Sha384(ref mut hasher) => hasher.update(data),
            HashContext::Sha512(ref mut hasher) => hasher.update(data),
            HashContext::Sha3_224(ref mut hasher) => hasher.update(data),
            HashContext::Sha3_256(ref mut hasher) => hasher.update(data),
            HashContext::Sha3_384(ref mut hasher) => hasher.update(data),
            HashContext::Sha3_512(ref mut hasher) => hasher.update(data),
            HashContext::Blake3(ref mut hasher) => {
                hasher.update(data);
            }
            HashContext::Sha1(ref mut hasher) => {
                use sha1::Digest;
                hasher.update(data);
            }
            HashContext::Md5(ref mut hasher) => {
                use md5::Digest;
                hasher.update(data);
            }
        }

        Ok(())
    }

    /// Finalize streaming hash context
    pub fn finalize_context(&self, context: HashContext, algorithm: HashAlgorithm, input_size: usize) -> SignatureResult<HashResult> {
        let digest = match context {
            HashContext::Sha224(hasher) => hasher.finalize().to_vec(),
            HashContext::Sha256(hasher) => hasher.finalize().to_vec(),
            HashContext::Sha384(hasher) => hasher.finalize().to_vec(),
            HashContext::Sha512(hasher) => hasher.finalize().to_vec(),
            HashContext::Sha3_224(hasher) => hasher.finalize().to_vec(),
            HashContext::Sha3_256(hasher) => hasher.finalize().to_vec(),
            HashContext::Sha3_384(hasher) => hasher.finalize().to_vec(),
            HashContext::Sha3_512(hasher) => hasher.finalize().to_vec(),
            HashContext::Blake3(hasher) => hasher.finalize().as_bytes().to_vec(),
            HashContext::Sha1(hasher) => {
                use sha1::Digest;
                hasher.finalize().to_vec()
            }
            HashContext::Md5(hasher) => {
                use md5::Digest;
                hasher.finalize().to_vec()
            }
        };

        Ok(HashResult {
            digest,
            algorithm,
            input_size,
        })
    }

    /// Get algorithm properties
    pub fn get_properties(&self, algorithm: &HashAlgorithm) -> Option<&HashProperties> {
        self.properties.get(algorithm)
    }

    /// Get all supported algorithms
    pub fn get_supported_algorithms(&self) -> Vec<HashAlgorithm> {
        self.properties.keys().cloned().collect()
    }

    /// Get recommended algorithms
    pub fn get_recommended_algorithms(&self) -> Vec<HashAlgorithm> {
        self.properties
            .values()
            .filter(|props| props.recommended)
            .map(|props| props.algorithm.clone())
            .collect()
    }

    /// Check if algorithm is quantum resistant
    pub fn is_quantum_resistant(&self, algorithm: &HashAlgorithm) -> bool {
        self.properties
            .get(algorithm)
            .map(|props| props.is_quantum_resistant)
            .unwrap_or(false)
    }

    /// Get digest size for algorithm
    pub fn get_digest_size(&self, algorithm: &HashAlgorithm) -> Option<usize> {
        self.properties
            .get(algorithm)
            .map(|props| props.digest_size)
    }

    /// Validate hash result
    pub fn validate_hash(&self, result: &HashResult) -> SignatureResult<bool> {
        let expected_size = self.get_digest_size(&result.algorithm)
            .ok_or_else(|| SignatureError::UnsupportedAlgorithm(format!("{:?}", result.algorithm)))?;

        Ok(result.digest.len() == expected_size)
    }

    /// Compare hash results
    pub fn compare_hashes(&self, hash1: &HashResult, hash2: &HashResult) -> bool {
        hash1.algorithm == hash2.algorithm && hash1.digest == hash2.digest
    }

    /// Hash multiple chunks and combine
    pub fn hash_chunks(&self, chunks: &[&[u8]], algorithm: &HashAlgorithm) -> SignatureResult<HashResult> {
        let mut context = self.create_context(algorithm)?;
        let mut total_size = 0;

        for chunk in chunks {
            self.update_context(&mut context, chunk)?;
            total_size += chunk.len();
        }

        self.finalize_context(context, algorithm.clone(), total_size)
    }

    /// Compute HMAC-style keyed hash
    pub fn keyed_hash(&self, key: &[u8], data: &[u8], algorithm: &HashAlgorithm) -> SignatureResult<HashResult> {
        // Simple HMAC implementation
        let properties = self.get_properties(algorithm)
            .ok_or_else(|| SignatureError::UnsupportedAlgorithm(format!("{:?}", algorithm)))?;

        let block_size = properties.block_size;
        let mut normalized_key = if key.len() > block_size {
            self.hash_with_algorithm(key, algorithm)?.digest
        } else {
            key.to_vec()
        };

        // Pad key to block size
        normalized_key.resize(block_size, 0);

        // Create inner and outer padding
        let mut ipad = vec![0x36; block_size];
        let mut opad = vec![0x5c; block_size];

        for i in 0..block_size {
            ipad[i] ^= normalized_key[i];
            opad[i] ^= normalized_key[i];
        }

        // Compute inner hash
        let mut inner_data = ipad;
        inner_data.extend(data);
        let inner_hash = self.hash_with_algorithm(&inner_data, algorithm)?;

        // Compute outer hash
        let mut outer_data = opad;
        outer_data.extend(&inner_hash.digest);
        let result = self.hash_with_algorithm(&outer_data, algorithm)?;

        Ok(HashResult {
            digest: result.digest,
            algorithm: algorithm.clone(),
            input_size: data.len(),
        })
    }

    /// Initialize algorithm properties
    fn initialize_properties(&mut self) {
        // SHA-2 family
        self.properties.insert(HashAlgorithm::Sha224, HashProperties {
            algorithm: HashAlgorithm::Sha224,
            digest_size: 28,
            block_size: 64,
            security_level: 112,
            is_quantum_resistant: false,
            recommended: false,
        });

        self.properties.insert(HashAlgorithm::Sha256, HashProperties {
            algorithm: HashAlgorithm::Sha256,
            digest_size: 32,
            block_size: 64,
            security_level: 128,
            is_quantum_resistant: false,
            recommended: true,
        });

        self.properties.insert(HashAlgorithm::Sha384, HashProperties {
            algorithm: HashAlgorithm::Sha384,
            digest_size: 48,
            block_size: 128,
            security_level: 192,
            is_quantum_resistant: false,
            recommended: true,
        });

        self.properties.insert(HashAlgorithm::Sha512, HashProperties {
            algorithm: HashAlgorithm::Sha512,
            digest_size: 64,
            block_size: 128,
            security_level: 256,
            is_quantum_resistant: false,
            recommended: true,
        });

        // SHA-3 family
        self.properties.insert(HashAlgorithm::Sha3_224, HashProperties {
            algorithm: HashAlgorithm::Sha3_224,
            digest_size: 28,
            block_size: 144,
            security_level: 112,
            is_quantum_resistant: true,
            recommended: false,
        });

        self.properties.insert(HashAlgorithm::Sha3_256, HashProperties {
            algorithm: HashAlgorithm::Sha3_256,
            digest_size: 32,
            block_size: 136,
            security_level: 128,
            is_quantum_resistant: true,
            recommended: true,
        });

        self.properties.insert(HashAlgorithm::Sha3_384, HashProperties {
            algorithm: HashAlgorithm::Sha3_384,
            digest_size: 48,
            block_size: 104,
            security_level: 192,
            is_quantum_resistant: true,
            recommended: true,
        });

        self.properties.insert(HashAlgorithm::Sha3_512, HashProperties {
            algorithm: HashAlgorithm::Sha3_512,
            digest_size: 64,
            block_size: 72,
            security_level: 256,
            is_quantum_resistant: true,
            recommended: true,
        });

        // BLAKE3
        self.properties.insert(HashAlgorithm::Blake3, HashProperties {
            algorithm: HashAlgorithm::Blake3,
            digest_size: 32,
            block_size: 64,
            security_level: 128,
            is_quantum_resistant: true,
            recommended: true,
        });

        // Legacy algorithms (not recommended)
        self.properties.insert(HashAlgorithm::Sha1, HashProperties {
            algorithm: HashAlgorithm::Sha1,
            digest_size: 20,
            block_size: 64,
            security_level: 80,
            is_quantum_resistant: false,
            recommended: false,
        });

        self.properties.insert(HashAlgorithm::Md5, HashProperties {
            algorithm: HashAlgorithm::Md5,
            digest_size: 16,
            block_size: 64,
            security_level: 64,
            is_quantum_resistant: false,
            recommended: false,
        });
    }
}

impl Default for HashAlgorithmManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::Sha224 => write!(f, "SHA-224"),
            HashAlgorithm::Sha256 => write!(f, "SHA-256"),
            HashAlgorithm::Sha384 => write!(f, "SHA-384"),
            HashAlgorithm::Sha512 => write!(f, "SHA-512"),
            HashAlgorithm::Sha3_224 => write!(f, "SHA3-224"),
            HashAlgorithm::Sha3_256 => write!(f, "SHA3-256"),
            HashAlgorithm::Sha3_384 => write!(f, "SHA3-384"),
            HashAlgorithm::Sha3_512 => write!(f, "SHA3-512"),
            HashAlgorithm::Blake3 => write!(f, "BLAKE3"),
            HashAlgorithm::Sha1 => write!(f, "SHA-1"),
            HashAlgorithm::Md5 => write!(f, "MD5"),
        }
    }
}

/// Convenience functions for common hash operations
pub mod utils {
    use super::*;

    /// Quick SHA-256 hash
    pub fn sha256(data: &[u8]) -> Vec<u8> {
        let manager = HashAlgorithmManager::new();
        manager.hash_with_algorithm(data, &HashAlgorithm::Sha256)
            .map(|result| result.digest)
            .unwrap_or_default()
    }

    /// Quick SHA-512 hash
    pub fn sha512(data: &[u8]) -> Vec<u8> {
        let manager = HashAlgorithmManager::new();
        manager.hash_with_algorithm(data, &HashAlgorithm::Sha512)
            .map(|result| result.digest)
            .unwrap_or_default()
    }

    /// Quick BLAKE3 hash
    pub fn blake3(data: &[u8]) -> Vec<u8> {
        blake3::hash(data).as_bytes().to_vec()
    }

    /// Quick SHA3-256 hash
    pub fn sha3_256(data: &[u8]) -> Vec<u8> {
        let manager = HashAlgorithmManager::new();
        manager.hash_with_algorithm(data, &HashAlgorithm::Sha3_256)
            .map(|result| result.digest)
            .unwrap_or_default()
    }

    /// Hash with automatic algorithm selection based on security requirements
    pub fn hash_secure(data: &[u8], min_security_level: u32) -> SignatureResult<HashResult> {
        let manager = HashAlgorithmManager::new();
        
        // Find the fastest algorithm that meets security requirements
        let algorithm = manager.get_supported_algorithms()
            .into_iter()
            .filter(|alg| {
                manager.get_properties(alg)
                    .map(|props| props.security_level >= min_security_level && props.recommended)
                    .unwrap_or(false)
            })
            .min_by_key(|alg| {
                // Prefer faster algorithms (smaller digest size as proxy for speed)
                manager.get_digest_size(alg).unwrap_or(999)
            })
            .unwrap_or(HashAlgorithm::Sha256);

        manager.hash_with_algorithm(data, &algorithm)
    }

    /// Multi-algorithm hash for extra security
    pub fn hash_multi(data: &[u8]) -> Vec<HashResult> {
        let manager = HashAlgorithmManager::new();
        let algorithms = [
            HashAlgorithm::Sha256,
            HashAlgorithm::Sha3_256,
            HashAlgorithm::Blake3,
        ];

        algorithms
            .iter()
            .filter_map(|alg| manager.hash_with_algorithm(data, alg).ok())
            .collect()
    }
}

