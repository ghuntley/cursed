/// fr fr PKI and certificate management for CURSED - trust infrastructure bestie
/// 
/// This module provides comprehensive Public Key Infrastructure including
/// X.509 certificates, certificate authorities, and trust management.

// Core PKI components
pub mod certificates;
pub mod certificate_authority;
pub mod trust_stores;
pub mod validation;

// Certificate formats and protocols
pub mod x509;
pub mod pkcs;
pub mod pem_der;
pub mod crl;

// PKI operations
pub mod certificate_generation;
pub mod certificate_signing;
pub mod certificate_revocation;
pub mod certificate_renewal;

// Trust and validation
pub mod trust_chains;
pub mod path_validation;
pub mod ocsp;
pub mod timestamping;

// Re-export main types
pub use certificates::{
    Certificate, CertificateBuilder, CertificateInfo, CertificateChain,
    CertificateError, CertificateResult
};
pub use certificate_authority::{
    CertificateAuthority, CaConfig, CaKeypair, IssuedCertificate,
    CaError, CaResult
};
pub use trust_stores::{
    TrustStore, TrustedCertificate, CertificateStore, TrustPolicy,
    TrustError, TrustResult
};
pub use validation::{
    CertificateValidator, ValidationResult, ValidationPolicy, ValidationError,
    ValidationFlags, ValidationContext
};

use std::time::SystemTime;
use std::collections::HashMap;

/// fr fr Certificate types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateType {
    RootCa,          // Root Certificate Authority
    IntermediateCa,  // Intermediate Certificate Authority
    EndEntity,       // End entity certificate
    CodeSigning,     // Code signing certificate
    TlsServer,       // TLS server certificate
    TlsClient,       // TLS client certificate
    Email,           // S/MIME email certificate
    TimeStamping,    // Time stamping certificate
}

/// fr fr Certificate formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateFormat {
    Der,        // DER binary format
    Pem,        // PEM text format
    Pkcs7,      // PKCS#7 format
    Pkcs12,     // PKCS#12 format
    Jks,        // Java KeyStore
    P7b,        // PKCS#7 bundle
}

/// fr fr Key usage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyUsage {
    pub digital_signature: bool,
    pub content_commitment: bool,
    pub key_encipherment: bool,
    pub data_encipherment: bool,
    pub key_agreement: bool,
    pub key_cert_sign: bool,
    pub crl_sign: bool,
    pub encipher_only: bool,
    pub decipher_only: bool,
}

impl Default for KeyUsage {
    fn default() -> Self {
        Self {
            digital_signature: false,
            content_commitment: false,
            key_encipherment: false,
            data_encipherment: false,
            key_agreement: false,
            key_cert_sign: false,
            crl_sign: false,
            encipher_only: false,
            decipher_only: false,
        }
    }
}

/// fr fr Extended key usage
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendedKeyUsage {
    ServerAuth,
    ClientAuth,
    CodeSigning,
    EmailProtection,
    TimeStamping,
    OcspSigning,
    IkeIntermediate,
    MsCodeInd,
    MsCodeCom,
    MsCTLSign,
    MsSGC,
    MsEFS,
    NsSGC,
    Custom(String),
}

/// fr fr PKI errors
#[derive(Debug, Clone, PartialEq)]
pub enum PkiError {
    InvalidCertificate,
    CertificateExpired,
    CertificateNotYetValid,
    InvalidSignature,
    UntrustedCertificate,
    CertificateRevoked,
    ChainValidationFailed,
    CaNotFound,
    InvalidKeyUsage,
    EncodingError(String),
    DecodingError(String),
    TrustStoreError(String),
    Internal(String),
}

impl std::fmt::Display for PkiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PkiError::InvalidCertificate => write!(f, "Invalid certificate"),
            PkiError::CertificateExpired => write!(f, "Certificate expired"),
            PkiError::CertificateNotYetValid => write!(f, "Certificate not yet valid"),
            PkiError::InvalidSignature => write!(f, "Invalid certificate signature"),
            PkiError::UntrustedCertificate => write!(f, "Untrusted certificate"),
            PkiError::CertificateRevoked => write!(f, "Certificate revoked"),
            PkiError::ChainValidationFailed => write!(f, "Certificate chain validation failed"),
            PkiError::CaNotFound => write!(f, "Certificate Authority not found"),
            PkiError::InvalidKeyUsage => write!(f, "Invalid key usage"),
            PkiError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            PkiError::DecodingError(msg) => write!(f, "Decoding error: {}", msg),
            PkiError::TrustStoreError(msg) => write!(f, "Trust store error: {}", msg),
            PkiError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for PkiError {}

/// fr fr PKI result type
pub type PkiResult<T> = Result<T, PkiError>;

/// fr fr Certificate subject/issuer information
#[derive(Debug, Clone)]
pub struct DistinguishedName {
    pub common_name: Option<String>,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state_or_province: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
    pub serial_number: Option<String>,
}

impl Default for DistinguishedName {
    fn default() -> Self {
        Self {
            common_name: None,
            organization: None,
            organizational_unit: None,
            country: None,
            state_or_province: None,
            locality: None,
            email: None,
            serial_number: None,
        }
    }
}

/// fr fr Certificate validity period
#[derive(Debug, Clone)]
pub struct ValidityPeriod {
    pub not_before: SystemTime,
    pub not_after: SystemTime,
}

impl ValidityPeriod {
    /// slay Create validity period for duration from now
    pub fn for_duration(duration: std::time::Duration) -> Self {
        let now = SystemTime::now();
        Self {
            not_before: now,
            not_after: now + duration,
        }
    }
    
    /// slay Check if currently valid
    pub fn is_valid_now(&self) -> bool {
        let now = SystemTime::now();
        now >= self.not_before && now <= self.not_after
    }
    
    /// slay Check if expired
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.not_after
    }
    
    /// slay Check if not yet valid
    pub fn is_not_yet_valid(&self) -> bool {
        SystemTime::now() < self.not_before
    }
}

/// fr fr Utilities and helper functions


pub mod utils {
    use super::*;
    
    /// slay Placeholder utility function
    pub fn placeholder() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

/// fr fr Initialize the crypto_pki package
pub fn init_crypto_pki() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 crypto_pki package initialized - ready bestie!");
    Ok(())
}
