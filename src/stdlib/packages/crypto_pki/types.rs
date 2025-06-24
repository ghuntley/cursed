/// Core PKI Types - Production Implementation
/// 
/// This module provides fundamental types used throughout the PKI system

use crate::error::CursedError;
use crate::error::Error;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Result type for PKI operations
pub type PkiResult<T> = std::result::Result<T, PkiError>;

/// PKI-specific error types
#[derive(Debug, Clone, PartialEq)]
pub enum PkiError {
    /// Certificate parsing or validation error
    CertificateError(String),
    /// OCSP-related error
    OcspError(String),
    /// Network communication error
    NetworkError(String),
    /// ASN.1 parsing error
    Asn1Error(String),
    /// Signature verification error
    SignatureError(String),
    /// Certificate revocation error
    RevocationError(String),
    /// General internal error
    Internal(String),
}

impl std::fmt::Display for PkiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PkiError::CertificateError(msg) => write!(f, "Certificate error: {}", msg),
            PkiError::OcspError(msg) => write!(f, "OCSP error: {}", msg),
            PkiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            PkiError::Asn1Error(msg) => write!(f, "ASN.1 error: {}", msg),
            PkiError::SignatureError(msg) => write!(f, "Signature error: {}", msg),
            PkiError::RevocationError(msg) => write!(f, "Revocation error: {}", msg),
            PkiError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for PkiError {}

impl From<PkiError> for CursedError {
    fn from(err: PkiError) -> Self {
        CursedError::RuntimeError(err.to_string())
    }
}

/// X.509 Certificate representation
#[derive(Debug, Clone)]
pub struct X509Certificate {
    pub subject: String,
    pub issuer: String,
    pub serial_number: Vec<u8>,
    pub not_before: SystemTime,
    pub not_after: SystemTime,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub signature_algorithm: String,
    pub extensions: HashMap<String, Vec<u8>>,
    pub raw_data: Vec<u8>,
}

impl X509Certificate {
    /// Create a new X.509 certificate
    pub fn new(
        subject: String,
        issuer: String,
        serial_number: Vec<u8>,
        not_before: SystemTime,
        not_after: SystemTime,
        public_key: Vec<u8>,
        signature: Vec<u8>,
        signature_algorithm: String,
        raw_data: Vec<u8>,
    ) -> Self {
        Self {
            subject,
            issuer,
            serial_number,
            not_before,
            not_after,
            public_key,
            signature,
            signature_algorithm,
            extensions: HashMap::new(),
            raw_data,
        }
    }

    /// Get certificate fingerprint (SHA-256)
    pub fn fingerprint(&self) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&self.raw_data);
        hasher.finalize().to_vec()
    }

    /// Check if certificate is currently valid (time-wise)
    pub fn is_currently_valid(&self) -> bool {
        let now = SystemTime::now();
        now >= self.not_before && now <= self.not_after
    }

    /// Get certificate serial number as hex string
    pub fn serial_hex(&self) -> String {
        hex::encode(&self.serial_number)
    }
}

/// Certificate revocation status
#[derive(Debug, Clone, PartialEq)]
pub enum RevocationStatus {
    /// Certificate is valid and not revoked
    Good,
    /// Certificate has been revoked
    Revoked {
        reason: Option<RevocationReason>,
        revocation_time: SystemTime,
    },
    /// Revocation status is unknown
    Unknown,
}

/// Reasons for certificate revocation
#[derive(Debug, Clone, PartialEq)]
pub enum RevocationReason {
    Unspecified = 0,
    KeyCompromise = 1,
    CaCompromise = 2,
    AffiliationChanged = 3,
    Superseded = 4,
    CessationOfOperation = 5,
    CertificateHold = 6,
    RemoveFromCrl = 8,
    PrivilegeWithdrawn = 9,
    AaCompromise = 10,
}

impl From<u8> for RevocationReason {
    fn from(value: u8) -> Self {
        match value {
            1 => RevocationReason::KeyCompromise,
            2 => RevocationReason::CaCompromise,
            3 => RevocationReason::AffiliationChanged,
            4 => RevocationReason::Superseded,
            5 => RevocationReason::CessationOfOperation,
            6 => RevocationReason::CertificateHold,
            8 => RevocationReason::RemoveFromCrl,
            9 => RevocationReason::PrivilegeWithdrawn,
            10 => RevocationReason::AaCompromise,
            _ => RevocationReason::Unspecified,
        }
    }
}

/// Certificate identifier for OCSP requests
#[derive(Debug, Clone)]
pub struct CertId {
    pub hash_algorithm: String,
    pub issuer_name_hash: Vec<u8>,
    pub issuer_key_hash: Vec<u8>,
    pub serial_number: Vec<u8>,
}

impl CertId {
    /// Create a new certificate identifier
    pub fn new(cert: &X509Certificate, issuer: &X509Certificate) -> PkiResult<Self> {
        use sha1::{Sha1, Digest};
        
        // Use SHA-1 as default hash algorithm for OCSP (RFC 6960)
        let mut hasher = Sha1::new();
        
        // Hash issuer name
        hasher.update(issuer.subject.as_bytes());
        let issuer_name_hash = hasher.finalize_reset().to_vec();
        
        // Hash issuer public key
        hasher.update(&issuer.public_key);
        let issuer_key_hash = hasher.finalize().to_vec();
        
        Ok(Self {
            hash_algorithm: "SHA-1".to_string(),
            issuer_name_hash,
            issuer_key_hash,
            serial_number: cert.serial_number.clone(),
        })
    }
}

/// OCSP Request structure
#[derive(Debug, Clone)]
pub struct OcspRequestInfo {
    pub cert_id: CertId,
    pub single_request_extensions: Option<HashMap<String, Vec<u8>>>,
}

/// OCSP Response status
#[derive(Debug, Clone, PartialEq)]
pub enum OcspResponseStatus {
    Successful = 0,
    MalformedRequest = 1,
    InternalError = 2,
    TryLater = 3,
    SigRequired = 5,
    Unauthorized = 6,
}

impl From<u8> for OcspResponseStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => OcspResponseStatus::Successful,
            1 => OcspResponseStatus::MalformedRequest,
            2 => OcspResponseStatus::InternalError,
            3 => OcspResponseStatus::TryLater,
            5 => OcspResponseStatus::SigRequired,
            6 => OcspResponseStatus::Unauthorized,
            _ => OcspResponseStatus::InternalError,
        }
    }
}

/// OCSP Single Response
#[derive(Debug, Clone)]
pub struct SingleResponse {
    pub cert_id: CertId,
    pub cert_status: RevocationStatus,
    pub this_update: SystemTime,
    pub next_update: Option<SystemTime>,
    pub single_extensions: Option<HashMap<String, Vec<u8>>>,
}

/// OCSP Response Basic structure
#[derive(Debug, Clone)]
pub struct BasicOcspResponse {
    pub tbs_response_data: Vec<u8>,
    pub signature_algorithm: String,
    pub signature: Vec<u8>,
    pub certs: Option<Vec<X509Certificate>>,
    pub responses: Vec<SingleResponse>,
    pub responder_id: String,
    pub produced_at: SystemTime,
    pub response_extensions: Option<HashMap<String, Vec<u8>>>,
}

/// OCSP Configuration
#[derive(Debug, Clone)]
pub struct OcspConfig {
    pub timeout: Duration,
    pub max_response_size: usize,
    pub user_agent: String,
    pub verify_signature: bool,
    pub cache_responses: bool,
    pub nonce_extension: bool,
}

impl Default for OcspConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_response_size: 1024 * 1024, // 1MB
            user_agent: "CURSED-PKI-OCSP/1.0".to_string(),
            verify_signature: true,
            cache_responses: true,
            nonce_extension: true,
        }
    }
}

/// Certificate status information
#[derive(Debug, Clone)]
pub struct CertificateStatusInfo {
    pub status: RevocationStatus,
    pub this_update: SystemTime,
    pub next_update: Option<SystemTime>,
    pub produced_at: SystemTime,
    pub responder_id: String,
}
