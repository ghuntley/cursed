use crate::error::Error;
/// fr fr Common result types for crypto operations
use std::fmt;

/// Result type for crypto_advanced operations
pub type AdvancedCryptoResult<T> = std::result::Result<T, AdvancedCryptoError>;

/// Result type for cipher operations  
pub type CipherResult<T> = std::result::Result<T, AdvancedCryptoError>;

/// Errors for advanced crypto operations
#[derive(Debug, Clone)]
pub enum AdvancedCryptoError {
    InvalidKey(String),
    EncryptionFailed(String),
    DecryptionFailed(String),
    InvalidInput(String),
    Internal(String),
}

impl fmt::Display for AdvancedCryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvancedCryptoError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            AdvancedCryptoError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            AdvancedCryptoError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            AdvancedCryptoError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AdvancedCryptoError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AdvancedCryptoError {}

/// Errors for cipher operations
#[derive(Debug, Clone)]
pub enum CipherError {
    InvalidKey(String),
    UnsupportedCipher(String),
    OperationFailed(String),
    Internal(String),
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            CipherError::UnsupportedCipher(msg) => write!(f, "Unsupported cipher: {}", msg),
            CipherError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
            CipherError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CipherError {}

/// Security Analysis specific errors
#[derive(Debug, Clone)]
pub enum SecurityAnalysisError {
    TimingVulnerability(String),
    SideChannelLeak(String),
    EntropyFailure(String),
    ParameterError(String),
    VulnerabilityDetected(String),
    AnalysisTimeout(String),
    InsufficientData(String),
    Internal(String),
}

impl std::fmt::Display for SecurityAnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityAnalysisError::TimingVulnerability(msg) => write!(f, "Timing vulnerability: {}", msg),
            SecurityAnalysisError::SideChannelLeak(msg) => write!(f, "Side-channel leak: {}", msg),
            SecurityAnalysisError::EntropyFailure(msg) => write!(f, "Entropy failure: {}", msg),
            SecurityAnalysisError::ParameterError(msg) => write!(f, "Parameter error: {}", msg),
            SecurityAnalysisError::VulnerabilityDetected(msg) => write!(f, "Vulnerability detected: {}", msg),
            SecurityAnalysisError::AnalysisTimeout(msg) => write!(f, "Analysis timeout: {}", msg),
            SecurityAnalysisError::InsufficientData(msg) => write!(f, "Insufficient data: {}", msg),
            SecurityAnalysisError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for SecurityAnalysisError {}

/// Result type for security analysis operations
pub type SecurityAnalysisResult<T> = std::result::Result<T, AdvancedCryptoError>;
