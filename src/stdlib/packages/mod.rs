
// Common crypto result types
pub type CryptoResult<T> = std::result::Result<T, CryptoError>;
pub type IOResult<T> = std::io::Result<T>;
pub type ModuleResult<T> = std::result::Result<T, ModuleError>;
pub type PkiResult<T> = std::result::Result<T, PkiError>;

#[derive(Debug, Clone)]
pub enum CryptoError {
    InvalidInput,
    EncryptionFailed,
    DecryptionFailed,
    KeyGenerationFailed,
    SignatureFailed,
    VerificationFailed,
    Other(String),
}

#[derive(Debug, Clone)]
pub enum ModuleError {
    InitializationFailed,
    ProcessingFailed,
    InvalidConfiguration,
    Other(String),
}

#[derive(Debug, Clone)]  
pub enum PkiError {
    CertificateInvalid,
    KeyInvalid,
    SigningFailed,
    ValidationFailed,
    Other(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidInput => write!(f, "Invalid input"),
            CryptoError::EncryptionFailed => write!(f, "Encryption failed"),
            CryptoError::DecryptionFailed => write!(f, "Decryption failed"),
            CryptoError::KeyGenerationFailed => write!(f, "Key generation failed"),
            CryptoError::SignatureFailed => write!(f, "Signature failed"),
            CryptoError::VerificationFailed => write!(f, "Verification failed"),
            CryptoError::Other(msg) => write!(f, "Other: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::InitializationFailed => write!(f, "Initialization failed"),
            ModuleError::ProcessingFailed => write!(f, "Processing failed"),
            ModuleError::InvalidConfiguration => write!(f, "Invalid configuration"),
            ModuleError::Other(msg) => write!(f, "Other: {}", msg),
        }
    }
}

impl std::error::Error for ModuleError {}

impl std::fmt::Display for PkiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PkiError::CertificateInvalid => write!(f, "Certificate invalid"),
            PkiError::KeyInvalid => write!(f, "Key invalid"),
            PkiError::SigningFailed => write!(f, "Signing failed"),
            PkiError::ValidationFailed => write!(f, "Validation failed"),
            PkiError::Other(msg) => write!(f, "Other: {}", msg),
        }
    }
}

impl std::error::Error for PkiError {}

// Stub handler types
#[derive(Debug, Clone, Default)]
pub struct CryptoHandler;

#[derive(Debug, Clone, Default)]  
pub struct IOHandler;

#[derive(Debug, Clone, Default)]
pub struct ModuleHandler;

/// fr fr Packages module for CURSED stdlib - modular library organization

// Database packages - comprehensive database connectivity
// pub mod db_core; // Temporarily disabled due to syntax errors
pub mod db_sql;
pub mod db_nosql;
pub mod db_pool;
pub mod db_query;
pub mod db_orm;
pub mod db_migrate;

// Cryptography packages - comprehensive security suite
// TODO: Re-enable when all modules are properly implemented
pub mod crypto_advanced;
pub mod crypto_asymmetric;
pub mod crypto_signatures;
pub mod crypto_kdf;
pub mod crypto_hash_advanced;
pub mod crypto_random;
pub mod crypto_zk;
pub mod crypto_pqc;
pub mod crypto_pki;
pub mod crypto_protocols;

// Testing and quality assurance packages
pub mod quick_test;

// Existing packages
pub mod web_vibez;
pub mod sql_vibes;
pub mod test_vibes;

// Re-export database packages for convenience
// pub use db_core::*; // Temporarily disabled
pub use db_sql::*;
pub use db_nosql::*;

// Re-export cryptography packages for convenience
// TODO: Re-enable when all modules are properly implemented
pub use crypto_advanced::*;
pub use crypto_asymmetric::*;
pub use crypto_signatures::*;
pub use crypto_kdf::*;
pub use crypto_hash_advanced::*;
pub use crypto_random::*;
pub use crypto_zk::*;
pub use crypto_pqc::*;
pub use crypto_pki::*;
pub use crypto_protocols::*;

// Re-export testing packages for convenience
pub use quick_test::*;

// Re-export existing packages for convenience
pub use web_vibez::*;
pub use sql_vibes::*;
pub use test_vibes::*;
