use crate::error::Error;
// PKI Error Handling
// 
// Comprehensive error types for Public Key Infrastructure operations.

use std::fmt;
use crate::error::Error as CursedError;

/// PKI-specific error types with detailed context
#[derive(Debug, Clone)]
pub enum PkiError {
    /// Certificate parsing or validation errors
    Certificate {
        message: String,
        certificate_id: Option<String>,
        error_code: CertificateErrorCode,
    },
    
    /// Certificate Authority operations errors  
    CertificateAuthority {
        message: String,
        ca_name: Option<String>,
        operation: String,
    },
    
    /// Certificate signing request errors
    CertificateSigningRequest {
        message: String,
        csr_id: Option<String>,
        validation_errors: Vec<String>,
    },
    
    /// Certificate chain validation errors
    ChainValidation {
        message: String,
        chain_length: Option<usize>,
        failed_certificate: Option<String>,
        validation_errors: Vec<String>,
    },
    
    /// Certificate revocation list errors
    RevocationList {
        message: String,
        crl_issuer: Option<String>,
        serial_numbers: Vec<String>,
    },
    
    /// OCSP (Online Certificate Status Protocol) errors
    Ocsp {
        message: String,
        responder_url: Option<String>,
        certificate_serial: Option<String>,
    },
    
    /// Key management errors
    KeyManagement {
        message: String,
        key_id: Option<String>,
        operation: String,
    },
    
    /// Trust store errors
    TrustStore {
        message: String,
        store_name: Option<String>,
        operation: String,
    },
    
    /// X.509 specific errors
    X509 {
        message: String,
        field: Option<String>,
        oid: Option<String>,
    },
    
    /// PEM/DER encoding/decoding errors
    Encoding {
        message: String,
        format: String,
        data_type: Option<String>,
    },
    
    /// PKCS format errors
    Pkcs {
        message: String,
        version: Option<u8>,
        format_type: String,
    },
    
    /// Cryptographic operation errors
    Cryptographic {
        message: String,
        algorithm: Option<String>,
        operation: String,
    },
    
    /// Policy validation errors
    Policy {
        message: String,
        policy_oid: Option<String>,
        constraint_violations: Vec<String>,
    },
    
    /// Configuration errors
    Configuration {
        message: String,
        config_key: Option<String>,
        invalid_value: Option<String>,
    },
    
    /// Network-related errors (OCSP, CRL distribution points)
    Network {
        message: String,
        url: Option<String>,
        status_code: Option<u16>,
    },
    
    /// General PKI error
    General(String),
}

/// Certificate-specific error codes for fine-grained error handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateErrorCode {
    /// Certificate has expired
    Expired,
    /// Certificate is not yet valid
    NotYetValid,
    /// Certificate signature is invalid
    InvalidSignature,
    /// Certificate issuer is not trusted
    UntrustedIssuer,
    /// Certificate has been revoked
    Revoked,
    /// Certificate purpose does not match usage
    InvalidPurpose,
    /// Certificate chain is incomplete
    IncompleteChain,
    /// Certificate contains invalid extensions
    InvalidExtensions,
    /// Certificate format is malformed
    MalformedCertificate,
    /// Certificate algorithm is not supported
    UnsupportedAlgorithm,
    /// Certificate key usage constraint violation
    KeyUsageViolation,
    /// Certificate basic constraints violation
    BasicConstraintsViolation,
    /// Certificate name constraints violation
    NameConstraintsViolation,
    /// Certificate policy constraints violation
    PolicyConstraintsViolation,
    /// General certificate validation error
    ValidationError,
}

impl fmt::Display for PkiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PkiError::Certificate { message, certificate_id, error_code } => {
                let cert_info = certificate_id
                    .as_ref()
                    .map(|id| format!(" (Certificate: {})", id))
                    .unwrap_or_default();
                write!(f, "Certificate Error [{}]: {}{}", error_code, message, cert_info)
            }
            PkiError::CertificateAuthority { message, ca_name, operation } => {
                let ca_info = ca_name
                    .as_ref()
                    .map(|name| format!(" (CA: {})", name))
                    .unwrap_or_default();
                write!(f, "Certificate Authority Error [{}]: {}{}", operation, message, ca_info)
            }
            PkiError::CertificateSigningRequest { message, csr_id, validation_errors } => {
                let csr_info = csr_id
                    .as_ref()
                    .map(|id| format!(" (CSR: {})", id))
                    .unwrap_or_default();
                let errors_info = if !validation_errors.is_empty() {
                    format!(" | Validation errors: {}", validation_errors.join(", "))
                } else {
                    String::new()
                };
                write!(f, "Certificate Signing Request Error: {}{}{}", message, csr_info, errors_info)
            }
            PkiError::ChainValidation { message, chain_length, failed_certificate, validation_errors } => {
                let chain_info = chain_length
                    .map(|len| format!(" (Chain length: {})", len))
                    .unwrap_or_default();
                let cert_info = failed_certificate
                    .as_ref()
                    .map(|cert| format!(" (Failed at: {})", cert))
                    .unwrap_or_default();
                let errors_info = if !validation_errors.is_empty() {
                    format!(" | Errors: {}", validation_errors.join(", "))
                } else {
                    String::new()
                };
                write!(f, "Chain Validation Error: {}{}{}{}", message, chain_info, cert_info, errors_info)
            }
            PkiError::RevocationList { message, crl_issuer, serial_numbers } => {
                let issuer_info = crl_issuer
                    .as_ref()
                    .map(|issuer| format!(" (Issuer: {})", issuer))
                    .unwrap_or_default();
                let serials_info = if !serial_numbers.is_empty() {
                    format!(" (Serials: {})", serial_numbers.join(", "))
                } else {
                    String::new()
                };
                write!(f, "CRL Error: {}{}{}", message, issuer_info, serials_info)
            }
            PkiError::Ocsp { message, responder_url, certificate_serial } => {
                let url_info = responder_url
                    .as_ref()
                    .map(|url| format!(" (Responder: {})", url))
                    .unwrap_or_default();
                let serial_info = certificate_serial
                    .as_ref()
                    .map(|serial| format!(" (Serial: {})", serial))
                    .unwrap_or_default();
                write!(f, "OCSP Error: {}{}{}", message, url_info, serial_info)
            }
            PkiError::KeyManagement { message, key_id, operation } => {
                let key_info = key_id
                    .as_ref()
                    .map(|id| format!(" (Key: {})", id))
                    .unwrap_or_default();
                write!(f, "Key Management Error [{}]: {}{}", operation, message, key_info)
            }
            PkiError::TrustStore { message, store_name, operation } => {
                let store_info = store_name
                    .as_ref()
                    .map(|name| format!(" (Store: {})", name))
                    .unwrap_or_default();
                write!(f, "Trust Store Error [{}]: {}{}", operation, message, store_info)
            }
            PkiError::X509 { message, field, oid } => {
                let field_info = field
                    .as_ref()
                    .map(|f| format!(" (Field: {})", f))
                    .unwrap_or_default();
                let oid_info = oid
                    .as_ref()
                    .map(|o| format!(" (OID: {})", o))
                    .unwrap_or_default();
                write!(f, "X.509 Error: {}{}{}", message, field_info, oid_info)
            }
            PkiError::Encoding { message, format, data_type } => {
                let type_info = data_type
                    .as_ref()
                    .map(|t| format!(" (Type: {})", t))
                    .unwrap_or_default();
                write!(f, "Encoding Error [{}]: {}{}", format, message, type_info)
            }
            PkiError::Pkcs { message, version, format_type } => {
                let version_info = version
                    .map(|v| format!(" (Version: {})", v))
                    .unwrap_or_default();
                write!(f, "PKCS Error [{}]: {}{}", format_type, message, version_info)
            }
            PkiError::Cryptographic { message, algorithm, operation } => {
                let algo_info = algorithm
                    .as_ref()
                    .map(|a| format!(" (Algorithm: {})", a))
                    .unwrap_or_default();
                write!(f, "Cryptographic Error [{}]: {}{}", operation, message, algo_info)
            }
            PkiError::Policy { message, policy_oid, constraint_violations } => {
                let oid_info = policy_oid
                    .as_ref()
                    .map(|oid| format!(" (Policy: {})", oid))
                    .unwrap_or_default();
                let violations_info = if !constraint_violations.is_empty() {
                    format!(" | Violations: {}", constraint_violations.join(", "))
                } else {
                    String::new()
                };
                write!(f, "Policy Error: {}{}{}", message, oid_info, violations_info)
            }
            PkiError::Configuration { message, config_key, invalid_value } => {
                let key_info = config_key
                    .as_ref()
                    .map(|key| format!(" (Key: {})", key))
                    .unwrap_or_default();
                let value_info = invalid_value
                    .as_ref()
                    .map(|value| format!(" (Value: {})", value))
                    .unwrap_or_default();
                write!(f, "Configuration Error: {}{}{}", message, key_info, value_info)
            }
            PkiError::Network { message, url, status_code } => {
                let url_info = url
                    .as_ref()
                    .map(|u| format!(" (URL: {})", u))
                    .unwrap_or_default();
                let status_info = status_code
                    .map(|code| format!(" (Status: {})", code))
                    .unwrap_or_default();
                write!(f, "Network Error: {}{}{}", message, url_info, status_info)
            }
            PkiError::General(message) => write!(f, "PKI Error: {}", message),
        }
    }
}

impl fmt::Display for CertificateErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CertificateErrorCode::Expired => write!(f, "EXPIRED"),
            CertificateErrorCode::NotYetValid => write!(f, "NOT_YET_VALID"),
            CertificateErrorCode::InvalidSignature => write!(f, "INVALID_SIGNATURE"),
            CertificateErrorCode::UntrustedIssuer => write!(f, "UNTRUSTED_ISSUER"),
            CertificateErrorCode::Revoked => write!(f, "REVOKED"),
            CertificateErrorCode::InvalidPurpose => write!(f, "INVALID_PURPOSE"),
            CertificateErrorCode::IncompleteChain => write!(f, "INCOMPLETE_CHAIN"),
            CertificateErrorCode::InvalidExtensions => write!(f, "INVALID_EXTENSIONS"),
            CertificateErrorCode::MalformedCertificate => write!(f, "MALFORMED_CERTIFICATE"),
            CertificateErrorCode::UnsupportedAlgorithm => write!(f, "UNSUPPORTED_ALGORITHM"),
            CertificateErrorCode::KeyUsageViolation => write!(f, "KEY_USAGE_VIOLATION"),
            CertificateErrorCode::BasicConstraintsViolation => write!(f, "BASIC_CONSTRAINTS_VIOLATION"),
            CertificateErrorCode::NameConstraintsViolation => write!(f, "NAME_CONSTRAINTS_VIOLATION"),
            CertificateErrorCode::PolicyConstraintsViolation => write!(f, "POLICY_CONSTRAINTS_VIOLATION"),
            CertificateErrorCode::ValidationError => write!(f, "VALIDATION_ERROR"),
        }
    }
}

impl std::error::Error for PkiError {}

/// Convert PkiError to CursedError for seamless integration
impl From<PkiError> for CursedError {
    fn from(err: PkiError) -> Self {
        CursedError::Runtime(format!("PKI Error: {}", err))
    }
}

/// Convenience type alias for PKI results
pub type PkiResult<T> = std::result::Result<T, PkiError>;

/// Helper functions for creating specific PKI errors
impl PkiError {
    /// Create a certificate error with specific error code
    pub fn certificate_error(message: impl Into<String>, error_code: CertificateErrorCode) -> Self {
        PkiError::Certificate {
            message: message.into(),
            certificate_id: None,
            error_code,
        }
    }
    
    /// Create a certificate error with certificate ID and error code
    pub fn certificate_error_with_id(
        message: impl Into<String>, 
        certificate_id: impl Into<String>,
        error_code: CertificateErrorCode
    ) -> Self {
        PkiError::Certificate {
            message: message.into(),
            certificate_id: Some(certificate_id.into()),
            error_code,
        }
    }
    
    /// Create a CA operation error
    pub fn ca_error(message: impl Into<String>, operation: impl Into<String>) -> Self {
        PkiError::CertificateAuthority {
            message: message.into(),
            ca_name: None,
            operation: operation.into(),
        }
    }
    
    /// Create a chain validation error
    pub fn chain_validation_error(message: impl Into<String>) -> Self {
        PkiError::ChainValidation {
            message: message.into(),
            chain_length: None,
            failed_certificate: None,
            validation_errors: Vec::new(),
        }
    }
    
    /// Create an encoding error
    pub fn encoding_error(message: impl Into<String>, format: impl Into<String>) -> Self {
        PkiError::Encoding {
            message: message.into(),
            format: format.into(),
            data_type: None,
        }
    }
    
    /// Create a cryptographic error
    pub fn crypto_error(message: impl Into<String>, operation: impl Into<String>) -> Self {
        PkiError::Cryptographic {
            message: message.into(),
            algorithm: None,
            operation: operation.into(),
        }
    }
    
    /// Create a general PKI error
    pub fn general(message: impl Into<String>) -> Self {
        PkiError::General(message.into())
    }
}
