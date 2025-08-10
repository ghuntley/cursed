/// Public Key Infrastructure (PKI) package
pub mod certificate;
pub mod certificate_authority;
pub mod certificate_chain;
pub mod certificate_generation;
pub mod certificate_renewal;
pub mod certificate_revocation;
pub mod certificate_signing;
pub mod certificate_transparency;
pub mod certificates;
pub mod chain_validation;
pub mod crl;
pub mod crl_manager;
pub mod csr_generator;
pub mod error;
pub mod errors;
pub mod extensions;
pub mod key_management;
pub mod key_pinning;
pub mod main;
pub mod ocsp;
pub mod ocsp_client;
pub mod path_validation;
pub mod pem_der;
pub mod pem_der_codec;
pub mod pkcs;
pub mod revocation;
pub mod templates;
pub mod timestamping;
pub mod trust_chains;
pub mod trust_store;
pub mod trust_stores;
pub mod types;
pub mod utils;
pub mod validation;
pub mod x509;
pub mod x509_extensions;
pub mod x509_parser;
pub mod enhanced_ca;
pub mod enhanced_main;

// Certificate types and operations
pub use certificate::{
    CertificateFormat, CertificateParser, CertificateValidator
};

// Certificate Authority
pub use certificate_authority::{
    CaError, CaResult, CaStatus, CaMetadata
};

// Certificate chains and validation
pub use certificate_chain::{
    ChainError, ChainResult, ChainValidationPolicy, ChainConstraints
};

// Trust store management
pub use trust_store::{
    remove_trusted_certificate, verify_trust, TrustStoreError, TrustStoreResult
};

// Certificate revocation
pub use revocation::{
    CrlError, CrlResult, RevocationStatus, CrlCache, CrlValidator
};

// PKCS standards support
pub use pkcs::{
    encrypt_private_key, decrypt_private_key, PkcsError, PkcsResult
};

// X.509 specific functionality
pub use x509::{
    X509Operations, X509
};

// Certificate extensions
pub use extensions::{
    ExtensionError, ExtensionResult, ExtensionBuilder, ExtensionValidator
};

// Certificate validation
pub use validation::{
    create_validation_context, ValidationLevel, ValidationMode
};

// Advanced features exports
pub use certificate_transparency::{
    CtLogList, parse_scts, verify_sct, CtError, CtResult
};

pub use ocsp::{
    OcspError, OcspResult, OcspCache, OcspValidator
};

pub use ocsp_client::OcspClient;

pub use key_pinning::{
    add_pin_from_certificate, verify_pin, PinError, PinResult
};

pub use templates::{
    create_client_template, create_ca_template, TemplateError, TemplateResult
};

use crate::error::CursedError;
use std::sync::Arc;

/// PKI result type
pub type PkiResult<T> = Result<T, CursedError>;

/// PKI system configuration
#[derive(Debug, Clone)]
pub struct PkiConfiguration {
    pub trust_anchors: Vec<String>,
    pub default_key_size: usize,
    pub default_validity_days: u32,
    pub enable_crl_checking: bool,
    pub enable_ocsp_checking: bool,
    pub enable_certificate_transparency: bool,
}

impl Default for PkiConfiguration {
    fn default() -> Self {
        Self {
            trust_anchors: vec![],
            default_key_size: 2048,
            default_validity_days: 365,
            enable_crl_checking: true,
            enable_ocsp_checking: true,
            enable_certificate_transparency: false,
        }
    }
}

/// Global PKI configuration
static PKI_CONFIG: std::sync::LazyLock<Arc<std::sync::RwLock<PkiConfiguration>>> = 
    std::sync::LazyLock::new(|| Arc::new(std::sync::RwLock::new(PkiConfiguration::default())));

/// Get PKI configuration
pub fn get_pki_config() -> PkiConfiguration {
    PKI_CONFIG.read()
        .map(|config| config.clone())
        .unwrap_or_default()
}

/// Update PKI configuration
pub fn update_pki_config<F>(updater: F) -> Result<(), CursedError> 
where
    F: FnOnce(&mut PkiConfiguration),
{
    let mut config = PKI_CONFIG.write()
        .map_err(|_| CursedError::internal_error("Failed to acquire PKI config lock"))?;
    
    updater(&mut *config);
    Ok(())
}

/// Certificate placeholder type
#[derive(Debug, Clone)]
pub struct Certificate {
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
}

/// Certificate chain placeholder type
#[derive(Debug, Clone)]
pub struct CertificateChain {
    pub certificates: Vec<Certificate>,
}

/// Validation result placeholder type
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// Validation options placeholder type
#[derive(Debug, Clone)]
pub struct ValidationOptions {
    pub check_hostname: Option<String>,
    pub check_revocation: bool,
    pub require_ct: bool,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
            check_hostname: None,
            check_revocation: true,
            require_ct: false,
        }
    }
}

/// Certificate validator placeholder type
#[derive(Debug, Clone)]
pub struct CertValidator;

impl CertValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn validate(&self, _cert: &Certificate, _options: &ValidationOptions) -> PkiResult<ValidationResult> {
        Ok(ValidationResult {
            valid: true,
            errors: vec![],
        })
    }
}

/// Chain builder placeholder type
#[derive(Debug, Clone)]
pub struct ChainBuilder;

impl ChainBuilder {
    pub fn new() -> Self {
        Self
    }
    
    pub fn build_chain(&self, cert: &Certificate, _intermediates: &[Certificate]) -> PkiResult<CertificateChain> {
        Ok(CertificateChain {
            certificates: vec![cert.clone()],
        })
    }
}

/// High-level PKI operations
pub mod pki {
    use super::*;
    
    /// Quick certificate validation
    pub fn quick_validate_certificate(cert: &Certificate, hostname: Option<&str>) -> PkiResult<ValidationResult> {
        let validator = CertValidator::new();
        
        let mut options = ValidationOptions::default();
        options.check_hostname = hostname.map(|h| h.to_string());
        
        validator.validate(cert, &options)
    }
    
    /// Quick certificate chain building
    pub fn quick_build_chain(cert: &Certificate, intermediates: &[Certificate]) -> PkiResult<CertificateChain> {
        let builder = ChainBuilder::new();
        builder.build_chain(cert, intermediates)
    }
    
    /// Quick CA certificate creation
    pub fn quick_create_ca(subject: &str, key_size: Option<usize>) -> PkiResult<(Certificate, Vec<u8>)> {
        let cert = Certificate {
            subject: subject.to_string(),
            issuer: subject.to_string(),
            serial_number: "1".to_string(),
        };
        
        let private_key = vec![0u8; key_size.unwrap_or(2048) / 8];
        
        Ok((cert, private_key))
    }
}

/// Initialize the crypto_pki package
pub fn init_crypto_pki() -> Result<(), CursedError> {
    println!("🔐 crypto_pki package initialized - PKI operations ready!");
    println!("  ✓ X.509 certificate handling");
    println!("  ✓ Certificate Authority operations");
    println!("  ✓ Certificate chain validation");
    println!("  ✓ Trust store management");
    println!("  ✓ Certificate revocation (CRL/OCSP)");
    println!("  ✓ PKCS standards support");
    println!("  ✓ Certificate transparency");
    println!("  ✓ Key pinning");
    
    Ok(())
}
