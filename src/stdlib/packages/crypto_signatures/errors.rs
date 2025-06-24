use crate::error::Error;
/// fr fr Digital signatures error types for CURSED crypto
/// 
/// Comprehensive error handling for all signature algorithms and operations.

use std::fmt;

/// Result type for digital signature operations
pub type SignatureResult<T> = std::result::Result<T, SignatureError>;

/// Comprehensive error types for digital signature operations
#[derive(Debug, Clone)]
pub enum SignatureError {
    /// Invalid private key format or content
    InvalidPrivateKey(String),
    /// Invalid public key format or content
    InvalidPublicKey(String),
    /// Invalid signature format or content
    InvalidSignature(String),
    /// Message too large for signature algorithm
    MessageTooLarge(String),
    /// Signature verification failed
    VerificationFailed(String),
    /// Unsupported signature algorithm
    UnsupportedAlgorithm(String),
    /// Invalid key size for algorithm
    InvalidKeySize(String),
    /// Key generation failed
    KeyGenerationFailed(String),
    /// Invalid hash algorithm for signature
    InvalidHashAlgorithm(String),
    /// Invalid input parameter or data
    InvalidInput(String),
    /// Timestamp service error
    TimestampError(String),
    /// Multi-signature threshold not met
    ThresholdNotMet(String),
    /// Invalid multi-signature configuration
    InvalidMultiSigConfig(String),
    /// Cryptographic operation failed
    CryptographicError(String),
    /// Internal error in signature system
    Internal(String),
}

impl fmt::Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureError::InvalidPrivateKey(msg) => 
                write!(f, "Invalid private key: {}", msg),
            SignatureError::InvalidPublicKey(msg) => 
                write!(f, "Invalid public key: {}", msg),
            SignatureError::InvalidSignature(msg) => 
                write!(f, "Invalid signature: {}", msg),
            SignatureError::MessageTooLarge(msg) => 
                write!(f, "Message too large: {}", msg),
            SignatureError::VerificationFailed(msg) => 
                write!(f, "Signature verification failed: {}", msg),
            SignatureError::UnsupportedAlgorithm(msg) => 
                write!(f, "Unsupported algorithm: {}", msg),
            SignatureError::InvalidKeySize(msg) => 
                write!(f, "Invalid key size: {}", msg),
            SignatureError::KeyGenerationFailed(msg) => 
                write!(f, "Key generation failed: {}", msg),
            SignatureError::InvalidHashAlgorithm(msg) => 
                write!(f, "Invalid hash algorithm: {}", msg),
            SignatureError::InvalidInput(msg) => 
                write!(f, "Invalid input: {}", msg),
            SignatureError::TimestampError(msg) => 
                write!(f, "Timestamp error: {}", msg),
            SignatureError::ThresholdNotMet(msg) => 
                write!(f, "Multi-signature threshold not met: {}", msg),
            SignatureError::InvalidMultiSigConfig(msg) => 
                write!(f, "Invalid multi-signature configuration: {}", msg),
            SignatureError::CryptographicError(msg) => 
                write!(f, "Cryptographic error: {}", msg),
            SignatureError::Internal(msg) => 
                write!(f, "Internal signature error: {}", msg),
        }
    }
}

impl std::error::Error for SignatureError {}

/// Convert from various error types
impl From<std::io::Error> for SignatureError {
    fn from(err: std::io::Error) -> Self {
        SignatureError::Internal(format!("I/O error: {}", err))
    }
}

impl From<String> for SignatureError {
    fn from(msg: String) -> Self {
        SignatureError::Internal(msg)
    }
}

impl From<&str> for SignatureError {
    fn from(msg: &str) -> Self {
        SignatureError::Internal(msg.to_string())
    }
}

/// Helper functions for creating common error types
impl SignatureError {
    /// Create an invalid key error
    pub fn invalid_key(msg: &str) -> Self {
        SignatureError::InvalidPrivateKey(msg.to_string())
    }
    
    /// Create a verification failed error
    pub fn verification_failed(msg: &str) -> Self {
        SignatureError::VerificationFailed(msg.to_string())
    }
    
    /// Create an unsupported algorithm error
    pub fn unsupported_algorithm(algorithm: &str) -> Self {
        SignatureError::UnsupportedAlgorithm(format!("Algorithm '{}' is not supported", algorithm))
    }
    
    /// Create an internal error
    pub fn internal(msg: &str) -> Self {
        SignatureError::Internal(msg.to_string())
    }
}

/// Signature operation statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct SignatureStats {
    pub signatures_generated: u64,
    pub signatures_verified: u64,
    pub verification_failures: u64,
    pub key_generations: u64,
    pub total_operations: u64,
}

impl SignatureStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn increment_signatures(&mut self) {
        self.signatures_generated += 1;
        self.total_operations += 1;
    }
    
    pub fn increment_verifications(&mut self) {
        self.signatures_verified += 1;
        self.total_operations += 1;
    }
    
    pub fn increment_failures(&mut self) {
        self.verification_failures += 1;
        self.total_operations += 1;
    }
    
    pub fn increment_key_gen(&mut self) {
        self.key_generations += 1;
        self.total_operations += 1;
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.signatures_verified + self.verification_failures == 0 {
            0.0
        } else {
            self.signatures_verified as f64 / 
            (self.signatures_verified + self.verification_failures) as f64
        }
    }
}
