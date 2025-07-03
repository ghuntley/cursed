//! Cryptographic hash functions

use crate::error::CursedError;

/// Result type for hash operations
pub type HashResult<T> = Result<T, CursedError>;

/// Supported hash algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
    Blake3,
    Md5,
}

/// Cryptographic hash function implementation
pub struct Hasher {
    algorithm: HashAlgorithm,
}

impl Hasher {
    /// Create a new hasher with the specified algorithm
    pub fn new(algorithm: HashAlgorithm) -> Self {
        Self { algorithm }
    }
    
    /// Hash the input data and return the digest as bytes
    pub fn hash(&self, data: &[u8]) -> HashResult<Vec<u8>> {
        match self.algorithm {
            HashAlgorithm::Sha256 => {
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha512 => {
                use sha2::{Sha512, Digest};
                let mut hasher = Sha512::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha3_256 => {
                use sha3::{Sha3_256, Digest};
                let mut hasher = Sha3_256::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha3_512 => {
                use sha3::{Sha3_512, Digest};
                let mut hasher = Sha3_512::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Blake3 => {
                let hash = blake3::hash(data);
                Ok(hash.as_bytes().to_vec())
            }
            HashAlgorithm::Md5 => {
                use md5::{Md5, Digest};
                let mut hasher = Md5::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
        }
    }
    
    /// Hash the input data and return the digest as a hex string
    pub fn hash_hex(&self, data: &[u8]) -> HashResult<String> {
        let hash_bytes = self.hash(data)?;
        Ok(hex::encode(hash_bytes))
    }
}

/// Convenience functions for common hash algorithms

/// SHA-256 hash
pub fn sha256(data: &[u8]) -> Vec<u8> {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// SHA-256 hash as hex string
pub fn sha256_hex(data: &[u8]) -> String {
    hex::encode(sha256(data))
}

/// SHA-512 hash
pub fn sha512(data: &[u8]) -> Vec<u8> {
    use sha2::{Sha512, Digest};
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// SHA-512 hash as hex string
pub fn sha512_hex(data: &[u8]) -> String {
    hex::encode(sha512(data))
}

/// BLAKE3 hash
pub fn blake3_hash(data: &[u8]) -> Vec<u8> {
    blake3::hash(data).as_bytes().to_vec()
}

/// BLAKE3 hash as hex string
pub fn blake3_hex(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}

/// MD5 hash (for legacy compatibility)
pub fn md5_hash(data: &[u8]) -> Vec<u8> {
    use md5::{Md5, Digest};
    let mut hasher = Md5::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// MD5 hash as hex string
pub fn md5_hex(data: &[u8]) -> String {
    hex::encode(md5_hash(data))
}

/// HMAC implementation
pub struct Hmac {
    algorithm: HashAlgorithm,
}

impl Hmac {
    /// Create new HMAC instance
    pub fn new(algorithm: HashAlgorithm) -> Self {
        Self { algorithm }
    }
    
    /// Compute HMAC
    pub fn compute(&self, key: &[u8], data: &[u8]) -> HashResult<Vec<u8>> {
        match self.algorithm {
            HashAlgorithm::Sha256 => {
                use hmac::{Hmac, Mac};
                use sha2::Sha256;
                type HmacSha256 = Hmac<Sha256>;
                
                let mut mac = HmacSha256::new_from_slice(key)
                    .map_err(|e| CryptoError::Other(format!("HMAC key error: {}")))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            HashAlgorithm::Sha512 => {
                use hmac::{Hmac, Mac};
                use sha2::Sha512;
use crate::stdlib::packages::CryptoError;
                type HmacSha512 = Hmac<Sha512>;
                
                let mut mac = HmacSha512::new_from_slice(key)
                    .map_err(|e| CryptoError::Other(format!("HMAC key error: {}")))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            _ => Err(CryptoError::Other("HMAC not supported for this hash algorithm")),
        }
    }
}

/// Test hash functionality
pub fn test_hash_functions() -> HashResult<()> {
    let test_data = b"Hello, CURSED!";
    
    // Test SHA-256
    let sha256_result = sha256(test_data);
    if sha256_result.is_empty() {
        return Err(CryptoError::Other("SHA-256 hash failed"));
    }
    
    // Test BLAKE3
    let blake3_result = blake3_hash(test_data);
    if blake3_result.is_empty() {
        return Err(CryptoError::Other("BLAKE3 hash failed"));
    }
    
    // Test HMAC
    let hmac = Hmac::new(HashAlgorithm::Sha256);
    let hmac_result = hmac.compute(b"secret_key", test_data)?;
    if hmac_result.is_empty() {
        return Err(CryptoError::Other("HMAC failed"));
    }
    
    Ok(())
}

/// Initialize hash subsystem
pub fn init_hash_functions() -> HashResult<()> {
    test_hash_functions()?;
    println!("🔒 Cryptographic hash functions initialized");
    Ok(())
}
