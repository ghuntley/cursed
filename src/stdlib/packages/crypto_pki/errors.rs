/// fr fr PKI error types and result definitions
use std::fmt;

/// Result type for PKI operations
pub type PkiResult<T> = Result<T, PkiError>;

/// Comprehensive PKI error types
#[derive(Debug, Clone)]
pub enum PkiError {
    // Certificate errors
    InvalidCertificate(String),
    CertificateParsingFailed(String),
    CertificateValidationFailed(String),
    CertificateExpired(String),
    CertificateNotYetValid(String),
    
    // CA errors
    CaOperationFailed(String),
    CaKeyMissing(String),
    CaConfigurationInvalid(String),
    CaHierarchyError(String),
    
    // Chain errors
    ChainBuildingFailed(String),
    ChainValidationFailed(String),
    ChainTooLong(String),
    ChainIncomplete(String),
    
    // Trust store errors
    TrustStoreError(String),
    UntrustedCertificate(String),
    TrustAnchorNotFound(String),
    
    // Revocation errors
    RevocationCheckFailed(String),
    CrlParsingFailed(String),
    CrlExpired(String),
    CertificateRevoked(String),
    
    // PKCS errors
    PkcsOperationFailed(String),
    InvalidPkcsFormat(String),
    UnsupportedPkcsVersion(String),
    
    // X.509 errors
    X509ParsingFailed(String),
    X509ValidationFailed(String),
    InvalidX509Format(String),
    UnsupportedX509Feature(String),
    
    // Extension errors
    ExtensionParsingFailed(String),
    ExtensionValidationFailed(String),
    CriticalExtensionNotSupported(String),
    
    // OCSP errors
    OcspRequestFailed(String),
    OcspResponseInvalid(String),
    OcspStatusUnknown(String),
    
    // Certificate Transparency errors
    CtValidationFailed(String),
    SctParsingFailed(String),
    CtLogNotTrusted(String),
    
    // Key pinning errors
    PinValidationFailed(String),
    PinMismatch(String),
    
    // Encoding/decoding errors
    EncodingFailed(String),
    DecodingFailed(String),
    InvalidFormat(String),
    UnsupportedFormat(String),
    
    // Cryptographic errors
    SignatureVerificationFailed(String),
    KeyGenerationFailed(String),
    InvalidKey(String),
    UnsupportedAlgorithm(String),
    
    // I/O errors
    IoError(String),
    FileNotFound(String),
    PermissionDenied(String),
    
    // Network errors
    NetworkError(String),
    TimeoutError(String),
    
    // Configuration errors
    ConfigurationError(String),
    InvalidParameter(String),
    MissingParameter(String),
    
    // Internal errors
    Internal(String),
    NotImplemented(String),
    OutOfMemory(String),
}

impl fmt::Display for PkiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Certificate errors
            PkiError::InvalidCertificate(msg) => write!(f, "Invalid certificate: {}", msg),
            PkiError::CertificateParsingFailed(msg) => write!(f, "Certificate parsing failed: {}", msg),
            PkiError::CertificateValidationFailed(msg) => write!(f, "Certificate validation failed: {}", msg),
            PkiError::CertificateExpired(msg) => write!(f, "Certificate expired: {}", msg),
            PkiError::CertificateNotYetValid(msg) => write!(f, "Certificate not yet valid: {}", msg),
            
            // CA errors
            PkiError::CaOperationFailed(msg) => write!(f, "CA operation failed: {}", msg),
            PkiError::CaKeyMissing(msg) => write!(f, "CA key missing: {}", msg),
            PkiError::CaConfigurationInvalid(msg) => write!(f, "CA configuration invalid: {}", msg),
            PkiError::CaHierarchyError(msg) => write!(f, "CA hierarchy error: {}", msg),
            
            // Chain errors
            PkiError::ChainBuildingFailed(msg) => write!(f, "Chain building failed: {}", msg),
            PkiError::ChainValidationFailed(msg) => write!(f, "Chain validation failed: {}", msg),
            PkiError::ChainTooLong(msg) => write!(f, "Chain too long: {}", msg),
            PkiError::ChainIncomplete(msg) => write!(f, "Chain incomplete: {}", msg),
            
            // Trust store errors
            PkiError::TrustStoreError(msg) => write!(f, "Trust store error: {}", msg),
            PkiError::UntrustedCertificate(msg) => write!(f, "Untrusted certificate: {}", msg),
            PkiError::TrustAnchorNotFound(msg) => write!(f, "Trust anchor not found: {}", msg),
            
            // Revocation errors
            PkiError::RevocationCheckFailed(msg) => write!(f, "Revocation check failed: {}", msg),
            PkiError::CrlParsingFailed(msg) => write!(f, "CRL parsing failed: {}", msg),
            PkiError::CrlExpired(msg) => write!(f, "CRL expired: {}", msg),
            PkiError::CertificateRevoked(msg) => write!(f, "Certificate revoked: {}", msg),
            
            // PKCS errors
            PkiError::PkcsOperationFailed(msg) => write!(f, "PKCS operation failed: {}", msg),
            PkiError::InvalidPkcsFormat(msg) => write!(f, "Invalid PKCS format: {}", msg),
            PkiError::UnsupportedPkcsVersion(msg) => write!(f, "Unsupported PKCS version: {}", msg),
            
            // X.509 errors
            PkiError::X509ParsingFailed(msg) => write!(f, "X.509 parsing failed: {}", msg),
            PkiError::X509ValidationFailed(msg) => write!(f, "X.509 validation failed: {}", msg),
            PkiError::InvalidX509Format(msg) => write!(f, "Invalid X.509 format: {}", msg),
            PkiError::UnsupportedX509Feature(msg) => write!(f, "Unsupported X.509 feature: {}", msg),
            
            // Extension errors
            PkiError::ExtensionParsingFailed(msg) => write!(f, "Extension parsing failed: {}", msg),
            PkiError::ExtensionValidationFailed(msg) => write!(f, "Extension validation failed: {}", msg),
            PkiError::CriticalExtensionNotSupported(msg) => write!(f, "Critical extension not supported: {}", msg),
            
            // OCSP errors
            PkiError::OcspRequestFailed(msg) => write!(f, "OCSP request failed: {}", msg),
            PkiError::OcspResponseInvalid(msg) => write!(f, "OCSP response invalid: {}", msg),
            PkiError::OcspStatusUnknown(msg) => write!(f, "OCSP status unknown: {}", msg),
            
            // Certificate Transparency errors
            PkiError::CtValidationFailed(msg) => write!(f, "CT validation failed: {}", msg),
            PkiError::SctParsingFailed(msg) => write!(f, "SCT parsing failed: {}", msg),
            PkiError::CtLogNotTrusted(msg) => write!(f, "CT log not trusted: {}", msg),
            
            // Key pinning errors
            PkiError::PinValidationFailed(msg) => write!(f, "Pin validation failed: {}", msg),
            PkiError::PinMismatch(msg) => write!(f, "Pin mismatch: {}", msg),
            
            // Encoding/decoding errors
            PkiError::EncodingFailed(msg) => write!(f, "Encoding failed: {}", msg),
            PkiError::DecodingFailed(msg) => write!(f, "Decoding failed: {}", msg),
            PkiError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            PkiError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
            
            // Cryptographic errors
            PkiError::SignatureVerificationFailed(msg) => write!(f, "Signature verification failed: {}", msg),
            PkiError::KeyGenerationFailed(msg) => write!(f, "Key generation failed: {}", msg),
            PkiError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            PkiError::UnsupportedAlgorithm(msg) => write!(f, "Unsupported algorithm: {}", msg),
            
            // I/O errors
            PkiError::IoError(msg) => write!(f, "I/O error: {}", msg),
            PkiError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            PkiError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            
            // Network errors
            PkiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            PkiError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            
            // Configuration errors
            PkiError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            PkiError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            PkiError::MissingParameter(msg) => write!(f, "Missing parameter: {}", msg),
            
            // Internal errors
            PkiError::Internal(msg) => write!(f, "Internal error: {}", msg),
            PkiError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            PkiError::OutOfMemory(msg) => write!(f, "Out of memory: {}", msg),
        }
    }
}

impl std::error::Error for PkiError {}

// Conversion from standard library errors
impl From<std::io::Error> for PkiError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => PkiError::FileNotFound(err.to_string()),
            std::io::ErrorKind::PermissionDenied => PkiError::PermissionDenied(err.to_string()),
            std::io::ErrorKind::TimedOut => PkiError::TimeoutError(err.to_string()),
            _ => PkiError::IoError(err.to_string()),
        }
    }
}

impl From<std::string::FromUtf8Error> for PkiError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        PkiError::DecodingFailed(format!("UTF-8 conversion error: {}", err))
    }
}

impl From<std::str::Utf8Error> for PkiError {
    fn from(err: std::str::Utf8Error) -> Self {
        PkiError::DecodingFailed(format!("UTF-8 error: {}", err))
    }
}

/// fr fr Helper functions for creating specific error types
impl PkiError {
    /// slay Create a certificate validation error
    pub fn certificate_validation(msg: &str) -> Self {
        PkiError::CertificateValidationFailed(msg.to_string())
    }
    
    /// slay Create a chain building error
    pub fn chain_building(msg: &str) -> Self {
        PkiError::ChainBuildingFailed(msg.to_string())
    }
    
    /// slay Create a trust store error
    pub fn trust_store(msg: &str) -> Self {
        PkiError::TrustStoreError(msg.to_string())
    }
    
    /// slay Create a CA operation error
    pub fn ca_operation(msg: &str) -> Self {
        PkiError::CaOperationFailed(msg.to_string())
    }
    
    /// slay Create a revocation check error
    pub fn revocation_check(msg: &str) -> Self {
        PkiError::RevocationCheckFailed(msg.to_string())
    }
    
    /// slay Create a signature verification error
    pub fn signature_verification(msg: &str) -> Self {
        PkiError::SignatureVerificationFailed(msg.to_string())
    }
    
    /// slay Create an encoding error
    pub fn encoding(msg: &str) -> Self {
        PkiError::EncodingFailed(msg.to_string())
    }
    
    /// slay Create a decoding error
    pub fn decoding(msg: &str) -> Self {
        PkiError::DecodingFailed(msg.to_string())
    }
    
    /// slay Create an internal error
    pub fn internal(msg: &str) -> Self {
        PkiError::Internal(msg.to_string())
    }
    
    /// slay Create a not implemented error
    pub fn not_implemented(feature: &str) -> Self {
        PkiError::NotImplemented(format!("Feature not implemented: {}", feature))
    }
    
    /// slay Create an invalid parameter error
    pub fn invalid_parameter(param: &str, value: &str) -> Self {
        PkiError::InvalidParameter(format!("Invalid parameter '{}': {}", param, value))
    }
    
    /// slay Create a missing parameter error
    pub fn missing_parameter(param: &str) -> Self {
        PkiError::MissingParameter(format!("Missing required parameter: {}", param))
    }
}

/// fr fr Specialized result types for different PKI components
pub type CertificateResult<T> = Result<T, PkiError>;
pub type CaResult<T> = Result<T, PkiError>;
pub type ChainResult<T> = Result<T, PkiError>;
pub type TrustStoreResult<T> = Result<T, PkiError>;
pub type CrlResult<T> = Result<T, PkiError>;
pub type PkcsResult<T> = Result<T, PkiError>;
pub type X509Result<T> = Result<T, PkiError>;
pub type ExtensionResult<T> = Result<T, PkiError>;
pub type OcspResult<T> = Result<T, PkiError>;
pub type CtResult<T> = Result<T, PkiError>;
pub type PinResult<T> = Result<T, PkiError>;
pub type TemplateResult<T> = Result<T, PkiError>;
pub type ValidationResult<T> = Result<T, PkiError>;
