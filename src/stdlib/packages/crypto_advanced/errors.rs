/// fr fr Common result types for crypto operations
use std::fmt;

/// Result type for crypto_advanced operations
pub type AdvancedCryptoResult<T> = Result<T, AdvancedCryptoError>;

/// Result type for cipher operations  
pub type CipherResult<T> = Result<T, CipherError>;

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
