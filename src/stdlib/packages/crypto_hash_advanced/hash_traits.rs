/// Production-ready hash traits and interfaces for advanced cryptographic hashing
use crate::error::CursedError;
use std::io::{Read, Write};
use std::fmt::{Debug, Display};

/// Result type for hash operations
pub type HashResult<T> = Result<T, CursedError>;

/// Standard digest size constants
pub const MD5_DIGEST_SIZE: usize = 16;
pub const SHA1_DIGEST_SIZE: usize = 20;
pub const SHA256_DIGEST_SIZE: usize = 32;
pub const SHA512_DIGEST_SIZE: usize = 64;
pub const SHA3_256_DIGEST_SIZE: usize = 32;
pub const SHA3_512_DIGEST_SIZE: usize = 64;
pub const BLAKE3_DIGEST_SIZE: usize = 32;
pub const XXHASH64_DIGEST_SIZE: usize = 8;
pub const SIPHASH_DIGEST_SIZE: usize = 8;

/// Core trait for all hash functions
pub trait Hasher: Clone + Debug + Send + Sync {
    /// Hash algorithm identifier
    fn algorithm(&self) -> &'static str;
    
    /// Expected digest size in bytes
    fn digest_size(&self) -> usize;
    
    /// Update hash with new data
    fn update(&mut self, data: &[u8]);
    
    /// Finalize hash and return digest
    fn finalize(self) -> Vec<u8>;
    
    /// Reset hasher to initial state
    fn reset(&mut self);
    
    /// Hash data in one shot
    fn hash(&mut self, data: &[u8]) -> Vec<u8> {
        self.reset();
        self.update(data);
        self.clone().finalize()
    }
    
    /// Hash from reader
    fn hash_reader<R: Read>(&mut self, mut reader: R) -> HashResult<Vec<u8>> {
        self.reset();
        let mut buffer = [0u8; 8192];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => self.update(&buffer[..n]),
                Err(e) => return Err(CursedError::IoError(format!("Read error: {}", e))),
            }
        }
        Ok(self.clone().finalize())
    }
}

/// Trait for cryptographically secure hash functions
pub trait CryptographicHasher: Hasher {
    /// Security level in bits
    fn security_level(&self) -> usize;
    
    /// Whether the hash function is quantum-resistant
    fn is_quantum_resistant(&self) -> bool;
    
    /// Collision resistance level
    fn collision_resistance(&self) -> SecurityLevel;
    
    /// Pre-image resistance level
    fn preimage_resistance(&self) -> SecurityLevel;
}

/// Trait for keyed hash functions (MACs)
pub trait KeyedHasher: Hasher {
    /// Set the key for the hasher
    fn set_key(&mut self, key: &[u8]) -> HashResult<()>;
    
    /// Get current key length
    fn key_length(&self) -> usize;
    
    /// Verify MAC against expected value
    fn verify(&mut self, data: &[u8], expected: &[u8]) -> bool {
        let computed = self.hash(data);
        constant_time_eq(&computed, expected)
    }
}

/// Trait for streaming hash functions
pub trait StreamingHasher: Hasher {
    /// Process data chunk by chunk
    fn process_chunk(&mut self, chunk: &[u8]);
    
    /// Get intermediate hash state
    fn intermediate_state(&self) -> Vec<u8>;
    
    /// Restore from intermediate state
    fn restore_state(&mut self, state: &[u8]) -> HashResult<()>;
}

/// Security level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// No security (broken)
    None = 0,
    /// Weak security (deprecated)
    Weak = 64,
    /// Acceptable security
    Acceptable = 112,
    /// Strong security 
    Strong = 128,
    /// Very strong security
    VeryStrong = 192,
    /// Quantum-resistant
    QuantumResistant = 256,
}

impl SecurityLevel {
    pub fn bits(&self) -> usize {
        *self as usize
    }
}

impl Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityLevel::None => write!(f, "None (0 bits)"),
            SecurityLevel::Weak => write!(f, "Weak (64 bits)"),
            SecurityLevel::Acceptable => write!(f, "Acceptable (112 bits)"),
            SecurityLevel::Strong => write!(f, "Strong (128 bits)"),
            SecurityLevel::VeryStrong => write!(f, "Very Strong (192 bits)"),
            SecurityLevel::QuantumResistant => write!(f, "Quantum Resistant (256+ bits)"),
        }
    }
}

/// Hash algorithm metadata
#[derive(Debug, Clone)]
pub struct HashAlgorithmInfo {
    pub name: &'static str,
    pub digest_size: usize,
    pub block_size: usize,
    pub security_level: SecurityLevel,
    pub is_cryptographic: bool,
    pub is_quantum_resistant: bool,
    pub standardized: bool,
    pub description: &'static str,
}

/// Registry of supported hash algorithms
pub struct HashRegistry {
    algorithms: Vec<HashAlgorithmInfo>,
}

impl HashRegistry {
    pub fn new() -> Self {
        let algorithms = vec![
            HashAlgorithmInfo {
                name: "SHA-256",
                digest_size: 32,
                block_size: 64,
                security_level: SecurityLevel::Strong,
                is_cryptographic: true,
                is_quantum_resistant: false,
                standardized: true,
                description: "NIST SHA-2 family, 256-bit digest",
            },
            HashAlgorithmInfo {
                name: "SHA-512",
                digest_size: 64,
                block_size: 128,
                security_level: SecurityLevel::VeryStrong,
                is_cryptographic: true,
                is_quantum_resistant: false,
                standardized: true,
                description: "NIST SHA-2 family, 512-bit digest",
            },
            HashAlgorithmInfo {
                name: "SHA3-256",
                digest_size: 32,
                block_size: 136,
                security_level: SecurityLevel::Strong,
                is_cryptographic: true,
                is_quantum_resistant: false,
                standardized: true,
                description: "NIST SHA-3 family, 256-bit digest",
            },
            HashAlgorithmInfo {
                name: "BLAKE3",
                digest_size: 32,
                block_size: 64,
                security_level: SecurityLevel::Strong,
                is_cryptographic: true,
                is_quantum_resistant: false,
                standardized: false,
                description: "Modern cryptographic hash, very fast",
            },
            HashAlgorithmInfo {
                name: "xxHash64",
                digest_size: 8,
                block_size: 32,
                security_level: SecurityLevel::None,
                is_cryptographic: false,
                is_quantum_resistant: false,
                standardized: false,
                description: "High-speed non-cryptographic hash",
            },
            HashAlgorithmInfo {
                name: "SipHash",
                digest_size: 8,
                block_size: 8,
                security_level: SecurityLevel::Weak,
                is_cryptographic: false,
                is_quantum_resistant: false,
                standardized: false,
                description: "Keyed hash for hash tables",
            },
        ];
        
        Self { algorithms }
    }
    
    pub fn get_algorithm(&self, name: &str) -> Option<&HashAlgorithmInfo> {
        self.algorithms.iter().find(|alg| alg.name.eq_ignore_ascii_case(name))
    }
    
    pub fn list_algorithms(&self) -> &[HashAlgorithmInfo] {
        &self.algorithms
    }
    
    pub fn cryptographic_algorithms(&self) -> Vec<&HashAlgorithmInfo> {
        self.algorithms.iter().filter(|alg| alg.is_cryptographic).collect()
    }
    
    pub fn fast_algorithms(&self) -> Vec<&HashAlgorithmInfo> {
        self.algorithms.iter().filter(|alg| !alg.is_cryptographic).collect()
    }
}

/// Constant-time equality comparison for security
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        result |= byte_a ^ byte_b;
    }
    result == 0
}

/// Secure memory clearing
pub fn secure_zero(data: &mut [u8]) {
    for byte in data.iter_mut() {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
}

/// Hash builder pattern for easy configuration
pub struct HashBuilder {
    algorithm: String,
    key: Option<Vec<u8>>,
    salt: Option<Vec<u8>>,
}

impl HashBuilder {
    pub fn new(algorithm: &str) -> Self {
        Self {
            algorithm: algorithm.to_string(),
            key: None,
            salt: None,
        }
    }
    
    pub fn with_key(mut self, key: &[u8]) -> Self {
        self.key = Some(key.to_vec());
        self
    }
    
    pub fn with_salt(mut self, salt: &[u8]) -> Self {
        self.salt = Some(salt.to_vec());
        self
    }
}

/// Standard test vectors for validation
pub struct TestVector {
    pub algorithm: &'static str,
    pub input: &'static [u8],
    pub expected: &'static [u8],
    pub description: &'static str,
}

/// Common test vectors from standards
pub const STANDARD_TEST_VECTORS: &[TestVector] = &[
    TestVector {
        algorithm: "SHA-256",
        input: b"",
        expected: &[
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8,
            0x99, 0x6f, 0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c,
            0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
        ],
        description: "SHA-256 empty string",
    },
    TestVector {
        algorithm: "SHA-256",
        input: b"abc",
        expected: &[
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde,
            0x5d, 0xae, 0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c,
            0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad,
        ],
        description: "SHA-256 'abc'",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_ordering() {
        assert!(SecurityLevel::None < SecurityLevel::Weak);
        assert!(SecurityLevel::Weak < SecurityLevel::Strong);
        assert!(SecurityLevel::Strong < SecurityLevel::VeryStrong);
        assert!(SecurityLevel::VeryStrong < SecurityLevel::QuantumResistant);
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
    }

    #[test]
    fn test_hash_registry() {
        let registry = HashRegistry::new();
        
        let sha256 = registry.get_algorithm("SHA-256").unwrap();
        assert_eq!(sha256.name, "SHA-256");
        assert_eq!(sha256.digest_size, 32);
        assert!(sha256.is_cryptographic);
        
        let crypto_algs = registry.cryptographic_algorithms();
        assert!(crypto_algs.len() > 0);
        
        let fast_algs = registry.fast_algorithms();
        assert!(fast_algs.len() > 0);
    }

    #[test]
    fn test_secure_zero() {
        let mut data = vec![1, 2, 3, 4, 5];
        secure_zero(&mut data);
        assert_eq!(data, vec![0, 0, 0, 0, 0]);
    }
}
