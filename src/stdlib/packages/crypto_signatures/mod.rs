/// fr fr Digital signature algorithms for CURSED - authentic and non-repudiable periodt
/// 
/// This module provides comprehensive digital signature capabilities including
/// RSA-PSS, ECDSA, EdDSA, and advanced signature schemes. Trust but verify bestie!

// Core signature algorithms
pub mod rsa_pss;
pub mod ecdsa;
pub mod eddsa;
pub mod signature_scheme;
pub mod verification;

// Message and hash handling
pub mod message_digest;
pub mod hash_algorithms;
pub mod signature_format;

// Advanced features
pub mod blind_signatures;
pub mod threshold_signatures;
pub mod ring_signatures;
pub mod aggregate_signatures;

// Security and utilities
pub mod signature_validation;
pub mod timestamping;
pub mod certificate_validation;

// Re-export main types for convenience
pub use rsa_pss::{
    RsaPssSignature, RsaPssPrivateKey, RsaPssPublicKey, RsaPssSigner,
    RsaPssVerifier, RsaPssParams, RSA_PSS_SALT_LENGTH_AUTO,
    RsaPssError, RsaPssResult
};
pub use ecdsa::{
    EcdsaSignature, EcdsaPrivateKey, EcdsaPublicKey, EcdsaSigner,
    EcdsaVerifier, EcdsaCurve, EcdsaParams, EcdsaError, EcdsaResult
};
pub use eddsa::{
    EddsaSignature, EddsaPrivateKey, EddsaPublicKey, EddsaSigner,
    EddsaVerifier, Ed25519Signer, Ed448Signer, EddsaError, EddsaResult
};
pub use signature_scheme::{
    SignatureScheme, DigitalSignature, SignatureAlgorithm,
    SignatureParams, SignatureCapabilities, SignatureContext
};
pub use verification::{
    SignatureVerification, VerificationResult, VerificationError,
    VerificationContext, BatchVerification, VerificationPolicy
};
pub use message_digest::{
    MessageDigest, DigestAlgorithm, HashContext, DigestError, DigestResult,
    SHA256_DIGEST_SIZE, SHA384_DIGEST_SIZE, SHA512_DIGEST_SIZE
};
pub use signature_format::{
    SignatureFormat, DerSignature, JoseSignature, RawSignature,
    SignatureEncoding, FormatError, FormatResult
};
pub use blind_signatures::{
    BlindSignature, BlindSigner, BlindVerifier, BlindingFactor,
    UnblindedSignature, BlindSignatureScheme, BlindError, BlindResult
};
pub use threshold_signatures::{
    ThresholdSignature, ThresholdSigner, ThresholdScheme, ShareHolder,
    SignatureShare, ThresholdParams, ThresholdError, ThresholdResult
};

use std::sync::Arc;
use std::collections::HashMap;
use std::time::SystemTime;

/// fr fr Supported signature algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignatureAlgorithmType {
    RsaPssSha256,
    RsaPssSha384,
    RsaPssSha512,
    EcdsaP256Sha256,
    EcdsaP384Sha384,
    EcdsaP521Sha512,
    EcdsaSecp256k1Sha256,
    Ed25519,
    Ed448,
}

impl SignatureAlgorithmType {
    /// slay Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            SignatureAlgorithmType::RsaPssSha256 => "RSA-PSS-SHA256",
            SignatureAlgorithmType::RsaPssSha384 => "RSA-PSS-SHA384",
            SignatureAlgorithmType::RsaPssSha512 => "RSA-PSS-SHA512",
            SignatureAlgorithmType::EcdsaP256Sha256 => "ECDSA-P256-SHA256",
            SignatureAlgorithmType::EcdsaP384Sha384 => "ECDSA-P384-SHA384",
            SignatureAlgorithmType::EcdsaP521Sha512 => "ECDSA-P521-SHA512",
            SignatureAlgorithmType::EcdsaSecp256k1Sha256 => "ECDSA-secp256k1-SHA256",
            SignatureAlgorithmType::Ed25519 => "Ed25519",
            SignatureAlgorithmType::Ed448 => "Ed448",
        }
    }
    
    /// slay Get recommended security level (in bits)
    pub fn security_level(&self) -> u32 {
        match self {
            SignatureAlgorithmType::RsaPssSha256 => 128,
            SignatureAlgorithmType::RsaPssSha384 => 192,
            SignatureAlgorithmType::RsaPssSha512 => 256,
            SignatureAlgorithmType::EcdsaP256Sha256 => 128,
            SignatureAlgorithmType::EcdsaP384Sha384 => 192,
            SignatureAlgorithmType::EcdsaP521Sha512 => 256,
            SignatureAlgorithmType::EcdsaSecp256k1Sha256 => 128,
            SignatureAlgorithmType::Ed25519 => 128,
            SignatureAlgorithmType::Ed448 => 224,
        }
    }
    
    /// slay Check if algorithm is deterministic
    pub fn is_deterministic(&self) -> bool {
        match self {
            SignatureAlgorithmType::RsaPssSha256 |
            SignatureAlgorithmType::RsaPssSha384 |
            SignatureAlgorithmType::RsaPssSha512 => false, // PSS uses random salt
            SignatureAlgorithmType::EcdsaP256Sha256 |
            SignatureAlgorithmType::EcdsaP384Sha384 |
            SignatureAlgorithmType::EcdsaP521Sha512 |
            SignatureAlgorithmType::EcdsaSecp256k1Sha256 => false, // ECDSA uses random k
            SignatureAlgorithmType::Ed25519 |
            SignatureAlgorithmType::Ed448 => true, // EdDSA is deterministic
        }
    }
    
    /// slay Check if algorithm is quantum resistant
    pub fn is_quantum_resistant(&self) -> bool {
        // Currently implemented algorithms are not quantum resistant
        false
    }
}

/// fr fr Signature errors
#[derive(Debug, Clone, PartialEq)]
pub enum SignatureError {
    UnsupportedAlgorithm(String),
    InvalidSignature,
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidMessage,
    SigningFailed(String),
    VerificationFailed(String),
    InvalidFormat(String),
    WeakKey(String),
    TimestampExpired,
    CertificateInvalid,
    Internal(String),
}

impl std::fmt::Display for SignatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignatureError::UnsupportedAlgorithm(name) => 
                write!(f, "Unsupported signature algorithm: {}", name),
            SignatureError::InvalidSignature => write!(f, "Invalid signature"),
            SignatureError::InvalidPublicKey => write!(f, "Invalid public key"),
            SignatureError::InvalidPrivateKey => write!(f, "Invalid private key"),
            SignatureError::InvalidMessage => write!(f, "Invalid message"),
            SignatureError::SigningFailed(msg) => write!(f, "Signing failed: {}", msg),
            SignatureError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            SignatureError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            SignatureError::WeakKey(msg) => write!(f, "Weak key detected: {}", msg),
            SignatureError::TimestampExpired => write!(f, "Timestamp expired"),
            SignatureError::CertificateInvalid => write!(f, "Certificate invalid"),
            SignatureError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SignatureError {}

/// fr fr Signature result type
pub type SignatureResult<T> = Result<T, SignatureError>;

/// fr fr Signature metadata for enhanced verification
#[derive(Debug, Clone)]
pub struct SignatureMetadata {
    pub algorithm: SignatureAlgorithmType,
    pub timestamp: Option<SystemTime>,
    pub signer_id: Option<String>,
    pub message_hash: Vec<u8>,
    pub signature_id: Option<String>,
    pub certificate_chain: Option<Vec<Vec<u8>>>,
    pub custom_attributes: HashMap<String, String>,
}

impl Default for SignatureMetadata {
    fn default() -> Self {
        Self {
            algorithm: SignatureAlgorithmType::Ed25519,
            timestamp: Some(SystemTime::now()),
            signer_id: None,
            message_hash: Vec::new(),
            signature_id: None,
            certificate_chain: None,
            custom_attributes: HashMap::new(),
        }
    }
}

/// fr fr Complete signature with metadata
#[derive(Debug, Clone)]
pub struct SignatureWithMetadata {
    pub signature: Vec<u8>,
    pub metadata: SignatureMetadata,
}

impl SignatureWithMetadata {
    /// slay Create new signature with metadata
    pub fn new(signature: Vec<u8>, metadata: SignatureMetadata) -> Self {
        Self { signature, metadata }
    }
    
    /// slay Verify this signature
    pub fn verify(&self, message: &[u8], public_key: &dyn SignatureScheme) -> SignatureResult<bool> {
        public_key.verify(message, &self.signature)
    }
    
    /// slay Check if signature is expired
    pub fn is_expired(&self, max_age: std::time::Duration) -> bool {
        if let Some(timestamp) = self.metadata.timestamp {
            if let Ok(elapsed) = timestamp.elapsed() {
                elapsed > max_age
            } else {
                true // Timestamp in future, consider expired
            }
        } else {
            false // No timestamp, not expired
        }
    }
}

/// fr fr Global signature algorithm registry
static SIGNATURE_REGISTRY: std::sync::LazyLock<Arc<std::sync::RwLock<SignatureRegistry>>> = 
    std::sync::LazyLock::new(|| Arc::new(std::sync::RwLock::new(SignatureRegistry::new())));

/// fr fr Signature algorithm registry
#[derive(Debug, Default)]
pub struct SignatureRegistry {
    algorithms: HashMap<String, SignatureAlgorithmType>,
}

impl SignatureRegistry {
    /// slay Create a new signature registry
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: HashMap::new(),
        };
        
        // Register default algorithms
        registry.register_algorithm("rsa-pss-sha256", SignatureAlgorithmType::RsaPssSha256);
        registry.register_algorithm("rsa-pss-sha384", SignatureAlgorithmType::RsaPssSha384);
        registry.register_algorithm("rsa-pss-sha512", SignatureAlgorithmType::RsaPssSha512);
        registry.register_algorithm("ecdsa-p256-sha256", SignatureAlgorithmType::EcdsaP256Sha256);
        registry.register_algorithm("ecdsa-p384-sha384", SignatureAlgorithmType::EcdsaP384Sha384);
        registry.register_algorithm("ecdsa-p521-sha512", SignatureAlgorithmType::EcdsaP521Sha512);
        registry.register_algorithm("ecdsa-secp256k1-sha256", SignatureAlgorithmType::EcdsaSecp256k1Sha256);
        registry.register_algorithm("ed25519", SignatureAlgorithmType::Ed25519);
        registry.register_algorithm("ed448", SignatureAlgorithmType::Ed448);
        
        registry
    }

    /// slay Register a signature algorithm
    pub fn register_algorithm(&mut self, name: &str, algorithm: SignatureAlgorithmType) {
        self.algorithms.insert(name.to_string(), algorithm);
    }

    /// slay Get an algorithm by name
    pub fn get_algorithm(&self, name: &str) -> Option<SignatureAlgorithmType> {
        self.algorithms.get(name).copied()
    }

    /// slay List all available algorithms
    pub fn list_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }
    
    /// slay Get deterministic algorithms
    pub fn deterministic_algorithms(&self) -> Vec<SignatureAlgorithmType> {
        self.algorithms.values()
            .filter(|alg| alg.is_deterministic())
            .copied()
            .collect()
    }
    
    /// slay Get algorithms by minimum security level
    pub fn algorithms_by_security_level(&self, min_level: u32) -> Vec<SignatureAlgorithmType> {
        self.algorithms.values()
            .filter(|alg| alg.security_level() >= min_level)
            .copied()
            .collect()
    }
}

/// slay Register an algorithm globally
pub fn register_signature_algorithm(name: &str, algorithm: SignatureAlgorithmType) -> SignatureResult<()> {
    let mut registry = SIGNATURE_REGISTRY.write()
        .map_err(|_| SignatureError::Internal("Failed to acquire signature registry lock".to_string()))?;
    
    registry.register_algorithm(name, algorithm);
    Ok(())
}

/// slay Get an algorithm by name from global registry
pub fn get_signature_algorithm(name: &str) -> SignatureResult<SignatureAlgorithmType> {
    let registry = SIGNATURE_REGISTRY.read()
        .map_err(|_| SignatureError::Internal("Failed to acquire signature registry lock".to_string()))?;
    
    registry.get_algorithm(name)
        .ok_or_else(|| SignatureError::UnsupportedAlgorithm(format!("Algorithm '{}' not found", name)))
}

/// slay List all available algorithms globally
pub fn list_signature_algorithms() -> Vec<String> {
    SIGNATURE_REGISTRY.read()
        .map(|registry| registry.list_algorithms())
        .unwrap_or_default()
}

/// fr fr Crypto utilities and helper functions
pub mod utils {
    use super::*;
    
    /// slay Quick Ed25519 signing (recommended default)
    pub fn quick_sign(private_key: &dyn SignatureScheme, message: &[u8]) -> SignatureResult<Vec<u8>> {
        private_key.sign(message)
    }
    
    /// slay Quick Ed25519 verification
    pub fn quick_verify(public_key: &dyn SignatureScheme, message: &[u8], signature: &[u8]) -> SignatureResult<bool> {
        public_key.verify(message, signature)
    }
    
    /// slay Generate signature with metadata
    pub fn sign_with_metadata(
        private_key: &dyn SignatureScheme,
        message: &[u8],
        metadata: SignatureMetadata
    ) -> SignatureResult<SignatureWithMetadata> {
        let signature = private_key.sign(message)?;
        Ok(SignatureWithMetadata::new(signature, metadata))
    }
    
    /// slay Batch verify multiple signatures
    pub fn batch_verify(
        verifications: &[(Vec<u8>, Vec<u8>, Box<dyn SignatureScheme>)] // (message, signature, public_key)
    ) -> SignatureResult<Vec<bool>> {
        let mut results = Vec::with_capacity(verifications.len());
        
        for (message, signature, public_key) in verifications {
            let result = public_key.verify(message, signature)?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// slay Check if an algorithm is available
    pub fn is_signature_algorithm_available(name: &str) -> bool {
        list_signature_algorithms().contains(&name.to_string())
    }
    
    /// slay Get recommended algorithms for security level
    pub fn recommended_signature_algorithms(min_security_level: u32) -> Vec<SignatureAlgorithmType> {
        SIGNATURE_REGISTRY.read()
            .map(|registry| registry.algorithms_by_security_level(min_security_level))
            .unwrap_or_default()
    }
    
    /// slay Get deterministic signature algorithms
    pub fn deterministic_signature_algorithms() -> Vec<SignatureAlgorithmType> {
        SIGNATURE_REGISTRY.read()
            .map(|registry| registry.deterministic_algorithms())
            .unwrap_or_default()
    }
    
    /// slay Validate signature format
    pub fn validate_signature_format(signature: &[u8], algorithm: SignatureAlgorithmType) -> SignatureResult<bool> {
        match algorithm {
            SignatureAlgorithmType::Ed25519 => Ok(signature.len() == 64),
            SignatureAlgorithmType::Ed448 => Ok(signature.len() == 114),
            SignatureAlgorithmType::EcdsaP256Sha256 => Ok(signature.len() >= 64 && signature.len() <= 72),
            SignatureAlgorithmType::EcdsaP384Sha384 => Ok(signature.len() >= 96 && signature.len() <= 104),
            SignatureAlgorithmType::EcdsaP521Sha512 => Ok(signature.len() >= 132 && signature.len() <= 140),
            _ => Ok(signature.len() >= 64), // Conservative check for RSA-PSS
        }
    }
}

/// fr fr Security configuration for signature operations
#[derive(Debug, Clone)]
pub struct SignatureSecurityConfig {
    pub minimum_security_level: u32,
    pub require_deterministic: bool,
    pub allow_weak_keys: bool,
    pub timestamp_required: bool,
    pub certificate_validation: bool,
    pub revocation_checking: bool,
    pub max_signature_age: Option<std::time::Duration>,
}

impl Default for SignatureSecurityConfig {
    fn default() -> Self {
        Self {
            minimum_security_level: 128,
            require_deterministic: false,
            allow_weak_keys: false,
            timestamp_required: true,
            certificate_validation: true,
            revocation_checking: true,
            max_signature_age: Some(std::time::Duration::from_secs(365 * 24 * 3600)), // 1 year
        }
    }
}

/// fr fr Initialize the crypto_signatures package
pub fn init_crypto_signatures() -> SignatureResult<()> {
    // Initialize signature algorithm registry (already done in new())
    let _registry = SIGNATURE_REGISTRY.read()
        .map_err(|_| SignatureError::Internal("Failed to initialize signature registry".to_string()))?;
    
    println!("✍️ crypto_signatures package initialized - digital signatures ready bestie!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signature_algorithm_type() {
        assert_eq!(SignatureAlgorithmType::Ed25519.name(), "Ed25519");
        assert_eq!(SignatureAlgorithmType::EcdsaP256Sha256.security_level(), 128);
        assert!(SignatureAlgorithmType::Ed25519.is_deterministic());
        assert!(!SignatureAlgorithmType::EcdsaP256Sha256.is_deterministic());
        assert!(!SignatureAlgorithmType::Ed25519.is_quantum_resistant());
    }
    
    #[test]
    fn test_signature_registry() {
        let registry = SignatureRegistry::new();
        assert!(registry.get_algorithm("ed25519").is_some());
        assert!(registry.get_algorithm("nonexistent").is_none());
        
        let algorithms = registry.list_algorithms();
        assert!(algorithms.contains(&"ed25519".to_string()));
        assert!(algorithms.contains(&"ecdsa-p256-sha256".to_string()));
        
        let deterministic = registry.deterministic_algorithms();
        assert!(deterministic.contains(&SignatureAlgorithmType::Ed25519));
        assert!(!deterministic.contains(&SignatureAlgorithmType::EcdsaP256Sha256));
    }
    
    #[test]
    fn test_signature_metadata() {
        let metadata = SignatureMetadata::default();
        assert_eq!(metadata.algorithm, SignatureAlgorithmType::Ed25519);
        assert!(metadata.timestamp.is_some());
        assert!(metadata.custom_attributes.is_empty());
    }
    
    #[test]
    fn test_signature_with_metadata() {
        let signature = vec![0u8; 64];
        let metadata = SignatureMetadata::default();
        let sig_with_meta = SignatureWithMetadata::new(signature.clone(), metadata);
        
        assert_eq!(sig_with_meta.signature, signature);
        assert!(!sig_with_meta.is_expired(std::time::Duration::from_secs(3600)));
    }
    
    #[test]
    fn test_init_crypto_signatures() {
        assert!(init_crypto_signatures().is_ok());
    }
    
    #[test]
    fn test_signature_error() {
        let error = SignatureError::InvalidSignature;
        assert_eq!(error.to_string(), "Invalid signature");
        
        let error = SignatureError::UnsupportedAlgorithm("test".to_string());
        assert_eq!(error.to_string(), "Unsupported signature algorithm: test");
    }
    
    #[test]
    fn test_security_config() {
        let config = SignatureSecurityConfig::default();
        assert_eq!(config.minimum_security_level, 128);
        assert!(!config.require_deterministic);
        assert!(!config.allow_weak_keys);
        assert!(config.timestamp_required);
        assert!(config.certificate_validation);
    }
    
    #[test]
    fn test_utils() {
        assert!(!utils::is_signature_algorithm_available("nonexistent"));
        
        let recommended = utils::recommended_signature_algorithms(128);
        assert!(!recommended.is_empty());
        
        let deterministic = utils::deterministic_signature_algorithms();
        assert!(deterministic.contains(&SignatureAlgorithmType::Ed25519));
        
        // Test signature format validation
        let ed25519_sig = vec![0u8; 64];
        assert!(utils::validate_signature_format(&ed25519_sig, SignatureAlgorithmType::Ed25519).unwrap());
        
        let invalid_sig = vec![0u8; 32];
        assert!(!utils::validate_signature_format(&invalid_sig, SignatureAlgorithmType::Ed25519).unwrap());
    }
}
