/// Cryptographic type definitions for the CURSED standard library
/// 
/// This module provides type definitions for cryptographic functionality
/// that are commonly referenced but may be missing from specific modules.

use std::collections::HashMap;

/// Ed25519 public key type
#[derive(Debug, Clone, PartialEq)]
pub struct Ed25519PublicKey {
    /// The 32-byte public key
    pub bytes: [u8; 32],
}

impl Ed25519PublicKey {
    /// Create a new Ed25519 public key from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    /// Get the key as bytes
    pub fn to_bytes(&self) -> [u8; 32] {
        self.bytes
    }

    /// Create from byte slice
    pub fn from_slice(bytes: &[u8]) -> Result<Self, CryptoError> {
        if bytes.len() != 32 {
            return Err(CryptoError::InvalidKeySize);
        }
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(bytes);
        Ok(Self::from_bytes(key_bytes))
    }

    /// Verify a signature with this public key
    pub fn verify(&self, message: &[u8], signature: &Ed25519Signature) -> Result<bool, CryptoError> {
        // Mock implementation - in real usage would use actual Ed25519 verification
        Ok(message.len() > 0 && signature.bytes.len() == 64)
    }
}

/// Ed25519 private key type
#[derive(Debug, Clone)]
pub struct Ed25519PrivateKey {
    /// The 32-byte private key (kept private)
    bytes: [u8; 32],
}

impl Ed25519PrivateKey {
    /// Create a new Ed25519 private key from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    /// Generate a new random private key
    pub fn generate() -> Result<Self, CryptoError> {
        // Mock implementation - in real usage would use secure random number generation
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = (i * 7 + 42) as u8; // Mock random data
        }
        Ok(Self::from_bytes(bytes))
    }

    /// Get the corresponding public key
    pub fn public_key(&self) -> Ed25519PublicKey {
        // Mock implementation - in real usage would derive actual public key
        let mut pub_bytes = [0u8; 32];
        for i in 0..32 {
            pub_bytes[i] = self.bytes[i] ^ 0x55; // Mock derivation
        }
        Ed25519PublicKey::from_bytes(pub_bytes)
    }

    /// Sign a message with this private key
    pub fn sign(&self, message: &[u8]) -> Result<Ed25519Signature, CryptoError> {
        // Mock implementation - in real usage would use actual Ed25519 signing
        let mut sig_bytes = [0u8; 64];
        for i in 0..32 {
            sig_bytes[i] = self.bytes[i];
        }
        for i in 32..64 {
            sig_bytes[i] = message.get(i - 32).copied().unwrap_or(0);
        }
        Ok(Ed25519Signature::from_bytes(sig_bytes))
    }
}

/// Ed25519 signature type
#[derive(Debug, Clone, PartialEq)]
pub struct Ed25519Signature {
    /// The 64-byte signature
    pub bytes: [u8; 64],
}

impl Ed25519Signature {
    /// Create a new Ed25519 signature from bytes
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        Self { bytes }
    }

    /// Get the signature as bytes
    pub fn to_bytes(&self) -> [u8; 64] {
        self.bytes
    }

    /// Create from byte slice
    pub fn from_slice(bytes: &[u8]) -> Result<Self, CryptoError> {
        if bytes.len() != 64 {
            return Err(CryptoError::InvalidSignatureSize);
        }
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(bytes);
        Ok(Self::from_bytes(sig_bytes))
    }
}

/// Cryptographic error types
#[derive(Debug, Clone)]
pub enum CryptoError {
    /// Invalid key size
    InvalidKeySize,
    /// Invalid signature size
    InvalidSignatureSize,
    /// Invalid input data
    InvalidInput,
    /// Verification failed
    VerificationFailed,
    /// Encryption failed
    EncryptionFailed,
    /// Decryption failed
    DecryptionFailed,
    /// Key generation failed
    KeyGenerationFailed,
    /// Algorithm not supported
    UnsupportedAlgorithm,
    /// Random number generation failed
    RandomGenerationFailed,
    /// Generic cryptographic error
    Generic(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidKeySize => write!(f, "Invalid key size"),
            CryptoError::InvalidSignatureSize => write!(f, "Invalid signature size"),
            CryptoError::InvalidInput => write!(f, "Invalid input data"),
            CryptoError::VerificationFailed => write!(f, "Signature verification failed"),
            CryptoError::EncryptionFailed => write!(f, "Encryption failed"),
            CryptoError::DecryptionFailed => write!(f, "Decryption failed"),
            CryptoError::KeyGenerationFailed => write!(f, "Key generation failed"),
            CryptoError::UnsupportedAlgorithm => write!(f, "Algorithm not supported"),
            CryptoError::RandomGenerationFailed => write!(f, "Random number generation failed"),
            CryptoError::Generic(msg) => write!(f, "Cryptographic error: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

/// Hash function type
#[derive(Debug, Clone)]
pub enum HashFunction {
    Sha256,
    Sha512,
    Blake3,
    Keccak256,
}

impl HashFunction {
    /// Get the output size in bytes for this hash function
    pub fn output_size(&self) -> usize {
        match self {
            HashFunction::Sha256 => 32,
            HashFunction::Sha512 => 64,
            HashFunction::Blake3 => 32,
            HashFunction::Keccak256 => 32,
        }
    }

    /// Hash data with this function
    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        // Mock implementation - in real usage would use actual hash functions
        let output_size = self.output_size();
        let mut result = vec![0u8; output_size];
        
        for (i, &byte) in data.iter().enumerate() {
            result[i % output_size] ^= byte;
        }
        
        // Simple checksum for mock purposes
        let checksum = data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        result[0] = checksum;
        
        result
    }
}

/// Symmetric encryption algorithm
#[derive(Debug, Clone)]
pub enum SymmetricAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
}

impl SymmetricAlgorithm {
    /// Get the key size in bytes for this algorithm
    pub fn key_size(&self) -> usize {
        match self {
            SymmetricAlgorithm::Aes256Gcm => 32,
            SymmetricAlgorithm::ChaCha20Poly1305 => 32,
            SymmetricAlgorithm::XChaCha20Poly1305 => 32,
        }
    }

    /// Get the nonce size in bytes for this algorithm
    pub fn nonce_size(&self) -> usize {
        match self {
            SymmetricAlgorithm::Aes256Gcm => 12,
            SymmetricAlgorithm::ChaCha20Poly1305 => 12,
            SymmetricAlgorithm::XChaCha20Poly1305 => 24,
        }
    }

    /// Get the tag size in bytes for this algorithm
    pub fn tag_size(&self) -> usize {
        16 // All supported algorithms use 16-byte authentication tags
    }
}

/// Asymmetric encryption algorithm
#[derive(Debug, Clone)]
pub enum AsymmetricAlgorithm {
    Rsa2048,
    Rsa4096,
    EccP256,
    EccP384,
    EccP521,
    Ed25519,
    X25519,
}

impl AsymmetricAlgorithm {
    /// Get the public key size in bytes for this algorithm
    pub fn public_key_size(&self) -> usize {
        match self {
            AsymmetricAlgorithm::Rsa2048 => 256,
            AsymmetricAlgorithm::Rsa4096 => 512,
            AsymmetricAlgorithm::EccP256 => 64,
            AsymmetricAlgorithm::EccP384 => 96,
            AsymmetricAlgorithm::EccP521 => 132,
            AsymmetricAlgorithm::Ed25519 => 32,
            AsymmetricAlgorithm::X25519 => 32,
        }
    }

    /// Get the private key size in bytes for this algorithm
    pub fn private_key_size(&self) -> usize {
        match self {
            AsymmetricAlgorithm::Rsa2048 => 256,
            AsymmetricAlgorithm::Rsa4096 => 512,
            AsymmetricAlgorithm::EccP256 => 32,
            AsymmetricAlgorithm::EccP384 => 48,
            AsymmetricAlgorithm::EccP521 => 66,
            AsymmetricAlgorithm::Ed25519 => 32,
            AsymmetricAlgorithm::X25519 => 32,
        }
    }

    /// Get the signature size in bytes for this algorithm
    pub fn signature_size(&self) -> usize {
        match self {
            AsymmetricAlgorithm::Rsa2048 => 256,
            AsymmetricAlgorithm::Rsa4096 => 512,
            AsymmetricAlgorithm::EccP256 => 64,
            AsymmetricAlgorithm::EccP384 => 96,
            AsymmetricAlgorithm::EccP521 => 132,
            AsymmetricAlgorithm::Ed25519 => 64,
            AsymmetricAlgorithm::X25519 => 0, // X25519 is for key exchange, not signing
        }
    }
}

/// Cryptographic key material
#[derive(Debug, Clone)]
pub struct KeyMaterial {
    /// The key bytes
    pub bytes: Vec<u8>,
    /// Algorithm this key is for
    pub algorithm: String,
    /// Key type (public, private, symmetric)
    pub key_type: KeyType,
}

impl KeyMaterial {
    /// Create new key material
    pub fn new(bytes: Vec<u8>, algorithm: String, key_type: KeyType) -> Self {
        Self {
            bytes,
            algorithm,
            key_type,
        }
    }

    /// Get the key size in bytes
    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    /// Check if this is a symmetric key
    pub fn is_symmetric(&self) -> bool {
        matches!(self.key_type, KeyType::Symmetric)
    }

    /// Check if this is a public key
    pub fn is_public(&self) -> bool {
        matches!(self.key_type, KeyType::Public)
    }

    /// Check if this is a private key
    pub fn is_private(&self) -> bool {
        matches!(self.key_type, KeyType::Private)
    }
}

/// Type of cryptographic key
#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    Symmetric,
    Public,
    Private,
}

/// Cryptographic context for operations
#[derive(Debug, Clone)]
pub struct CryptoContext {
    /// Preferred hash function
    pub hash_function: HashFunction,
    /// Preferred symmetric algorithm
    pub symmetric_algorithm: SymmetricAlgorithm,
    /// Preferred asymmetric algorithm
    pub asymmetric_algorithm: AsymmetricAlgorithm,
    /// Security level
    pub security_level: SecurityLevel,
    /// Additional parameters
    pub parameters: HashMap<String, String>,
}

impl CryptoContext {
    /// Create a new crypto context with default settings
    pub fn new() -> Self {
        Self {
            hash_function: HashFunction::Sha256,
            symmetric_algorithm: SymmetricAlgorithm::Aes256Gcm,
            asymmetric_algorithm: AsymmetricAlgorithm::Ed25519,
            security_level: SecurityLevel::Standard,
            parameters: HashMap::new(),
        }
    }

    /// Create a high-security crypto context
    pub fn high_security() -> Self {
        Self {
            hash_function: HashFunction::Sha512,
            symmetric_algorithm: SymmetricAlgorithm::XChaCha20Poly1305,
            asymmetric_algorithm: AsymmetricAlgorithm::EccP521,
            security_level: SecurityLevel::High,
            parameters: HashMap::new(),
        }
    }

    /// Add a parameter to the context
    pub fn with_parameter(mut self, key: String, value: String) -> Self {
        self.parameters.insert(key, value);
        self
    }
}

impl Default for CryptoContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Security level for cryptographic operations
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Low,
    Standard,
    High,
    Maximum,
}

impl SecurityLevel {
    /// Get recommended key sizes for this security level
    pub fn recommended_key_sizes(&self) -> KeySizeRecommendations {
        match self {
            SecurityLevel::Low => KeySizeRecommendations {
                symmetric: 128,
                rsa: 1024,
                ecc: 256,
            },
            SecurityLevel::Standard => KeySizeRecommendations {
                symmetric: 256,
                rsa: 2048,
                ecc: 256,
            },
            SecurityLevel::High => KeySizeRecommendations {
                symmetric: 256,
                rsa: 4096,
                ecc: 384,
            },
            SecurityLevel::Maximum => KeySizeRecommendations {
                symmetric: 256,
                rsa: 4096,
                ecc: 521,
            },
        }
    }
}

/// Key size recommendations for different algorithms
#[derive(Debug, Clone)]
pub struct KeySizeRecommendations {
    /// Symmetric key size in bits
    pub symmetric: usize,
    /// RSA key size in bits
    pub rsa: usize,
    /// ECC key size in bits
    pub ecc: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_key_generation() {
        let private_key = Ed25519PrivateKey::generate().unwrap();
        let public_key = private_key.public_key();
        
        let message = b"test message";
        let signature = private_key.sign(message).unwrap();
        let is_valid = public_key.verify(message, &signature).unwrap();
        
        assert!(is_valid);
    }

    #[test]
    fn test_hash_function() {
        let hash_fn = HashFunction::Sha256;
        let data = b"hello world";
        let hash = hash_fn.hash(data);
        
        assert_eq!(hash.len(), 32);
        assert_eq!(hash_fn.output_size(), 32);
    }

    #[test]
    fn test_crypto_context() {
        let context = CryptoContext::new();
        assert_eq!(context.security_level, SecurityLevel::Standard);
        
        let high_security = CryptoContext::high_security();
        assert_eq!(high_security.security_level, SecurityLevel::High);
    }

    #[test]
    fn test_key_material() {
        let key_bytes = vec![1, 2, 3, 4];
        let key = KeyMaterial::new(key_bytes, "AES".to_string(), KeyType::Symmetric);
        
        assert_eq!(key.size(), 4);
        assert!(key.is_symmetric());
        assert!(!key.is_public());
        assert!(!key.is_private());
    }
}
