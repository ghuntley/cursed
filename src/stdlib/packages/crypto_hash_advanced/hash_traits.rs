/// Production-ready hash traits and interfaces for advanced cryptographic hashing
use std::io::{Read, Write};
use std::fmt::{Debug, Display};
use crate::error::CursedError;

/// Result type for hash operations
pub type HashResult<T> = std::result::Result<T, CryptoError>;

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
    /// Hash from reader
    fn hash_reader<R: Read>(&mut self, mut reader: R) -> HashResult<Vec<u8>> {
        self.reset();
        let mut buffer = [0u8; 8192];
        loop {
            match reader.read(&mut buffer) {
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
/// Security level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// No security (broken)
    /// Weak security (deprecated)
    /// Acceptable security
    /// Strong security 
    /// Very strong security
    /// Quantum-resistant
impl SecurityLevel {
    pub fn bits(&self) -> usize {
        *self as usize
    }
}

impl Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// Hash algorithm metadata
#[derive(Debug, Clone)]
pub struct HashAlgorithmInfo {
/// Registry of supported hash algorithms
pub struct HashRegistry {
impl HashRegistry {
    pub fn new() -> Self {
        let algorithms = vec![
            HashAlgorithmInfo {
            HashAlgorithmInfo {
            HashAlgorithmInfo {
            HashAlgorithmInfo {
            HashAlgorithmInfo {
            HashAlgorithmInfo {
        ];
        
        Self { algorithms }
    }
    
    pub fn get_algorithm(&self, name: &str) -> Option<&HashAlgorithmInfo> {
        self.algorithms.iter().find(|alg| alg.name.eq_ignore_ascii_case(name))
    pub fn list_algorithms(&self) -> &[HashAlgorithmInfo] {
        &self.algorithms
    pub fn cryptographic_algorithms(&self) -> Vec<&HashAlgorithmInfo> {
        self.algorithms.iter().filter(|alg| alg.is_cryptographic).collect()
    pub fn fast_algorithms(&self) -> Vec<&HashAlgorithmInfo> {
        self.algorithms.iter().filter(|alg| !alg.is_cryptographic).collect()
    }
}

/// Constant-time equality comparison for security
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    let mut result = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        result |= byte_a ^ byte_b;
    }
    result == 0
/// Secure memory clearing
pub fn secure_zero(data: &mut [u8]) {
    for byte in data.iter_mut() {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
/// Hash builder pattern for easy configuration
pub struct HashBuilder {
impl HashBuilder {
    pub fn new(algorithm: &str) -> Self {
        Self {
        }
    }
    
    pub fn with_key(mut self, key: &[u8]) -> Self {
        self.key = Some(key.to_vec());
        self
    pub fn with_salt(mut self, salt: &[u8]) -> Self {
        self.salt = Some(salt.to_vec());
        self
    }
}

/// Standard test vectors for validation
pub struct TestVector {
/// Common test vectors from standards
pub const STANDARD_TEST_VECTORS: &[TestVector] = &[
    TestVector {
        expected: &[
    TestVector {
        expected: &[
];

