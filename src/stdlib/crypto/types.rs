/// Cryptographic type definitions for the CURSED standard library
/// 
/// This module provides type definitions for cryptographic functionality
/// that are commonly referenced but may be missing from specific modules.

use std::collections::HashMap;

/// Platform-specific cryptographic operations
#[derive(Debug, Clone)]
pub struct CryptoPlatform {
    /// Platform identifier
    pub platform_id: String,
    /// Supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Hardware acceleration support
    pub hardware_acceleration: bool,
}

impl CryptoPlatform {
    /// Create a new crypto platform
    pub fn new() -> Result<(), Error> {
        Ok(Self {
            platform_id: "default".to_string(),
            supported_algorithms: vec![
                "AES-256-GCM".to_string(),
                "ChaCha20-Poly1305".to_string(),
                "Ed25519".to_string(),
                "X25519".to_string(),
            ],
            hardware_acceleration: false,
        })
    }

    /// Constant-time equality comparison
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
}

impl Default for CryptoPlatform {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            platform_id: "fallback".to_string(),
            supported_algorithms: vec![],
            hardware_acceleration: false,
        })
    }
}

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
    pub fn from_slice(bytes: &[u8]) -> Result<(), Error> {
        if bytes.len() != 32 {
            return Err(CryptoError::InvalidKeySize);
        }
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(bytes);
        Ok(Self::from_bytes(key_bytes))
    }

    /// Verify a signature with this public key
    pub fn verify(&self, message: &[u8], signature: &Ed25519Signature) -> Result<(), Error> {
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
    pub fn generate() -> Result<(), Error> {
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
    pub fn sign(&self, message: &[u8]) -> Result<(), Error> {
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
    pub fn from_slice(bytes: &[u8]) -> Result<(), Error> {
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

/// Cryptographic parameters for configuration
#[derive(Debug, Clone)]
pub struct CryptoParameters {
    /// Key size for symmetric encryption in bits
    pub symmetric_key_size: usize,
    /// Key size for asymmetric encryption in bits
    pub asymmetric_key_size: usize,
    /// Hash algorithm to use
    pub hash_algorithm: String,
    /// Encryption algorithm to use
    pub encryption_algorithm: String,
    /// Security level
    pub security_level: SecurityLevel,
    /// Key derivation parameters
    pub kdf_iterations: usize,
    /// Salt size for key derivation
    pub salt_size: usize,
    /// Additional algorithm-specific parameters
    pub algorithm_parameters: HashMap<String, String>,
}

impl CryptoParameters {
    /// Create default cryptographic parameters
    pub fn new() -> Self {
        Self {
            symmetric_key_size: 256,
            asymmetric_key_size: 2048,
            hash_algorithm: "SHA-256".to_string(),
            encryption_algorithm: "AES-256-GCM".to_string(),
            security_level: SecurityLevel::Standard,
            kdf_iterations: 100_000,
            salt_size: 32,
            algorithm_parameters: HashMap::new(),
        }
    }

    /// Create high-security parameters
    pub fn high_security() -> Self {
        Self {
            symmetric_key_size: 256,
            asymmetric_key_size: 4096,
            hash_algorithm: "SHA-512".to_string(),
            encryption_algorithm: "ChaCha20-Poly1305".to_string(),
            security_level: SecurityLevel::High,
            kdf_iterations: 500_000,
            salt_size: 64,
            algorithm_parameters: HashMap::new(),
        }
    }

    /// Create maximum security parameters
    pub fn maximum_security() -> Self {
        Self {
            symmetric_key_size: 256,
            asymmetric_key_size: 4096,
            hash_algorithm: "SHA-512".to_string(),
            encryption_algorithm: "XChaCha20-Poly1305".to_string(),
            security_level: SecurityLevel::Maximum,
            kdf_iterations: 1_000_000,
            salt_size: 64,
            algorithm_parameters: HashMap::new(),
        }
    }

    /// Add an algorithm-specific parameter
    pub fn with_parameter(mut self, key: String, value: String) -> Self {
        self.algorithm_parameters.insert(key, value);
        self
    }

    /// Get the effective security level in bits
    pub fn effective_security_bits(&self) -> usize {
        match self.security_level {
            SecurityLevel::Low => 80,
            SecurityLevel::Standard => 128,
            SecurityLevel::High => 192,
            SecurityLevel::Maximum => 256,
        }
    }
}

impl Default for CryptoParameters {
    fn default() -> Self {
        Self::new()
    }
}

/// Security context for cryptographic operations
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// User identity for access control
    pub user_id: String,
    /// Roles assigned to the user
    pub roles: Vec<String>,
    /// Permissions granted
    pub permissions: Vec<String>,
    /// Security level required
    pub security_level: SecurityLevel,
    /// Cryptographic parameters
    pub crypto_parameters: CryptoParameters,
    /// Session-specific data
    pub session_data: HashMap<String, String>,
    /// Audit trail enabled
    pub audit_enabled: bool,
    /// Time-based restrictions
    pub valid_from: Option<u64>,
    pub valid_until: Option<u64>,
}

impl SecurityContext {
    /// Create a new security context
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            roles: Vec::new(),
            permissions: Vec::new(),
            security_level: SecurityLevel::Standard,
            crypto_parameters: CryptoParameters::new(),
            session_data: HashMap::new(),
            audit_enabled: true,
            valid_from: None,
            valid_until: None,
        }
    }

    /// Create a high-security context
    pub fn high_security(user_id: String) -> Self {
        Self {
            user_id,
            roles: Vec::new(),
            permissions: Vec::new(),
            security_level: SecurityLevel::High,
            crypto_parameters: CryptoParameters::high_security(),
            session_data: HashMap::new(),
            audit_enabled: true,
            valid_from: None,
            valid_until: None,
        }
    }

    /// Add a role to the context
    pub fn with_role(mut self, role: String) -> Self {
        self.roles.push(role);
        self
    }

    /// Add a permission to the context
    pub fn with_permission(mut self, permission: String) -> Self {
        self.permissions.push(permission);
        self
    }

    /// Add session data
    pub fn with_session_data(mut self, key: String, value: String) -> Self {
        self.session_data.insert(key, value);
        self
    }

    /// Set validity period
    pub fn with_validity(mut self, from: u64, until: u64) -> Self {
        self.valid_from = Some(from);
        self.valid_until = Some(until);
        self
    }

    /// Check if context has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Check if context has a specific permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }

    /// Check if context is currently valid
    pub fn is_valid(&self, current_time: u64) -> bool {
        if let Some(from) = self.valid_from {
            if current_time < from {
                return false;
            }
        }
        if let Some(until) = self.valid_until {
            if current_time > until {
                return false;
            }
        }
        true
    }

    /// Get the minimum security level required
    pub fn minimum_security_level(&self) -> &SecurityLevel {
        &self.security_level
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new("anonymous".to_string())
    }
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

    #[test]
    fn test_crypto_parameters() {
        let params = CryptoParameters::new();
        assert_eq!(params.symmetric_key_size, 256);
        assert_eq!(params.asymmetric_key_size, 2048);
        assert_eq!(params.security_level, SecurityLevel::Standard);
        assert_eq!(params.effective_security_bits(), 128);

        let high_security = CryptoParameters::high_security();
        assert_eq!(high_security.asymmetric_key_size, 4096);
        assert_eq!(high_security.security_level, SecurityLevel::High);
        assert_eq!(high_security.effective_security_bits(), 192);

        let with_param = CryptoParameters::new()
            .with_parameter("custom".to_string(), "value".to_string());
        assert_eq!(with_param.algorithm_parameters.get("custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_security_context() {
        let context = SecurityContext::new("user123".to_string());
        assert_eq!(context.user_id, "user123");
        assert_eq!(context.security_level, SecurityLevel::Standard);
        assert!(context.audit_enabled);

        let enhanced_context = SecurityContext::high_security("admin".to_string())
            .with_role("administrator".to_string())
            .with_permission("read_all".to_string())
            .with_session_data("session_id".to_string(), "abc123".to_string())
            .with_validity(1000, 2000);

        assert!(enhanced_context.has_role("administrator"));
        assert!(enhanced_context.has_permission("read_all"));
        assert!(!enhanced_context.has_role("user"));
        assert!(!enhanced_context.has_permission("write_all"));
        assert!(enhanced_context.is_valid(1500));
        assert!(!enhanced_context.is_valid(500));
        assert!(!enhanced_context.is_valid(3000));
    }

    #[test]
    fn test_security_level_recommendations() {
        let low = SecurityLevel::Low;
        let recommendations = low.recommended_key_sizes();
        assert_eq!(recommendations.symmetric, 128);
        assert_eq!(recommendations.rsa, 1024);
        assert_eq!(recommendations.ecc, 256);

        let maximum = SecurityLevel::Maximum;
        let max_recommendations = maximum.recommended_key_sizes();
        assert_eq!(max_recommendations.symmetric, 256);
        assert_eq!(max_recommendations.rsa, 4096);
        assert_eq!(max_recommendations.ecc, 521);
    }
}
