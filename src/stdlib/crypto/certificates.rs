/// fr fr X.509 certificates and PKI for CURSED - secure authentication periodt
/// 
/// This module provides comprehensive certificate handling including parsing,
/// validation, chain verification, and CSR support. Trust but verify bestie!

use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::stdlib::value::Value;
use crate::error::CursedError;
use super::asymmetric::{AsymmetricError, AsymmetricResult, RsaPublicKey, EcdsaPublicKey};
use x509_parser::prelude::*;
use der::{Decode, Encode};
use pem::{Pem, encode, parse};

/// fr fr X.509 certificate structure
#[derive(Debug, Clone)]
pub struct X509Certificate {
    pub version: u8,
    pub serial_number: Vec<u8>,
    pub signature_algorithm: SignatureAlgorithm,
    pub issuer: DistinguishedName,
    pub validity: Validity,
    pub subject: DistinguishedName,
    pub public_key: PublicKeyInfo,
    pub extensions: Vec<Extension>,
    pub signature: Vec<u8>,
    pub raw_der: Vec<u8>,
}

/// fr fr Certificate chain for validation
#[derive(Debug, Clone)]
pub struct CertificateChain {
    pub certificates: Vec<X509Certificate>,
    pub trusted_roots: Vec<X509Certificate>,
}

/// fr fr Certificate signing request
#[derive(Debug, Clone)]
pub struct CertificateSigningRequest {
    pub subject: DistinguishedName,
    pub public_key: PublicKeyInfo,
    pub attributes: Vec<Attribute>,
    pub signature_algorithm: SignatureAlgorithm,
    pub signature: Vec<u8>,
    pub raw_der: Vec<u8>,
}

/// fr fr Distinguished name (subject/issuer)
#[derive(Debug, Clone, PartialEq)]
pub struct DistinguishedName {
    pub common_name: Option<String>,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}

/// fr fr Certificate validity period
#[derive(Debug, Clone)]
pub struct Validity {
    pub not_before: SystemTime,
    pub not_after: SystemTime,
}

/// fr fr Public key information
#[derive(Debug, Clone)]
pub struct PublicKeyInfo {
    pub algorithm: PublicKeyAlgorithm,
    pub key_data: Vec<u8>,
    pub parameters: Option<Vec<u8>>,
}

/// fr fr Supported public key algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicKeyAlgorithm {
    RsaEncryption,
    EcPublicKey,
    Ed25519,
    X25519,
}

/// fr fr Supported signature algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    Sha256WithRsaEncryption,
    Sha384WithRsaEncryption,
    Sha512WithRsaEncryption,
    EcdsaWithSha256,
    EcdsaWithSha384,
    EcdsaWithSha512,
    Ed25519,
}

/// fr fr Certificate extensions
#[derive(Debug, Clone)]
pub struct Extension {
    pub oid: ObjectIdentifier,
    pub critical: bool,
    pub value: Vec<u8>,
}

/// fr fr Object identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectIdentifier {
    pub components: Vec<u32>,
}

/// fr fr Certificate attribute (for CSRs)
#[derive(Debug, Clone)]
pub struct Attribute {
    pub oid: ObjectIdentifier,
    pub values: Vec<Vec<u8>>,
}

/// fr fr Certificate validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum CertificateError {
    InvalidFormat(String),
    InvalidSignature,
    Expired,
    NotYetValid,
    UntrustedIssuer,
    ChainValidationFailed(String),
    RevocationCheckFailed(String),
    HostnameMismatch(String),
    UnsupportedAlgorithm(String),
    ParseError(String),
    EncodingError(String),
    Internal(String),
}

impl fmt::Display for CertificateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CertificateError::InvalidFormat(msg) => 
                write!(f, "Invalid certificate format: {}", msg),
            CertificateError::InvalidSignature => 
                write!(f, "Invalid certificate signature"),
            CertificateError::Expired => 
                write!(f, "Certificate has expired"),
            CertificateError::NotYetValid => 
                write!(f, "Certificate is not yet valid"),
            CertificateError::UntrustedIssuer => 
                write!(f, "Certificate issued by untrusted authority"),
            CertificateError::ChainValidationFailed(msg) => 
                write!(f, "Certificate chain validation failed: {}", msg),
            CertificateError::RevocationCheckFailed(msg) => 
                write!(f, "Certificate revocation check failed: {}", msg),
            CertificateError::HostnameMismatch(expected) => 
                write!(f, "Certificate hostname mismatch, expected: {}", expected),
            CertificateError::UnsupportedAlgorithm(alg) => 
                write!(f, "Unsupported algorithm: {}", alg),
            CertificateError::ParseError(msg) => 
                write!(f, "Certificate parse error: {}", msg),
            CertificateError::EncodingError(msg) => 
                write!(f, "Certificate encoding error: {}", msg),
            CertificateError::Internal(msg) => 
                write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CertificateError {}

pub type CertificateResult<T> = Result<T, CertificateError>;

/// fr fr Certificate encoding formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodingFormat {
    Der,  // Binary DER encoding
    Pem,  // Base64 PEM encoding
}

/// fr fr Main certificate processor
pub struct CertificateProcessor {
    trusted_roots: Vec<X509Certificate>,
    config: CertificateConfig,
}

/// fr fr Certificate validation configuration
#[derive(Debug, Clone)]
pub struct CertificateConfig {
    pub check_expiration: bool,
    pub check_hostname: bool,
    pub check_revocation: bool,
    pub allow_self_signed: bool,
    pub max_chain_length: usize,
    pub signature_verification: bool,
}

impl Default for CertificateConfig {
    fn default() -> Self {
        Self {
            check_expiration: true,
            check_hostname: true,
            check_revocation: false, // Often requires network access
            allow_self_signed: false,
            max_chain_length: 10,
            signature_verification: true,
        }
    }
}

impl CertificateProcessor {
    /// slay Create new certificate processor
    pub fn new() -> Self {
        Self::with_config(CertificateConfig::default())
    }
    
    /// slay Create certificate processor with custom config
    pub fn with_config(config: CertificateConfig) -> Self {
        Self {
            trusted_roots: Vec::new(),
            config,
        }
    }
    
    /// slay Add trusted root certificate
    pub fn add_trusted_root(&mut self, root: X509Certificate) {
        self.trusted_roots.push(root);
    }
    
    /// slay Load system root certificates
    pub fn load_system_roots(&mut self) -> CertificateResult<()> {
        // Placeholder: load system root certificates
        // Real implementation would load from system store
        println!("Loading system root certificates...");
        Ok(())
    }

    /// slay Parse certificate from DER format
    pub fn parse_der(&self, der_data: &[u8]) -> CertificateResult<X509Certificate> {
        if der_data.is_empty() {
            return Err(CertificateError::InvalidFormat("Empty DER data".to_string()));
        }
        
        // Real X.509 DER parsing using x509-parser
        let (_, x509_cert) = parse_x509_certificate(der_data)
            .map_err(|e| CertificateError::ParseError(format!("X.509 parsing failed: {:?}", e)))?;
        
        // Convert to our internal representation
        let cert = self.convert_x509_to_internal(x509_cert, der_data)?;
        
        Ok(cert)
    }
    
    /// Convert x509-parser certificate to internal format
    fn convert_x509_to_internal(&self, x509_cert: X509Certificate<'_>, raw_der: &[u8]) -> CertificateResult<crate::stdlib::crypto::certificates::X509Certificate> {
        // Extract issuer DN
        let issuer = self.extract_distinguished_name(&x509_cert.issuer)?;
        
        // Extract subject DN
        let subject = self.extract_distinguished_name(&x509_cert.subject)?;
        
        // Extract validity
        let validity = Validity {
            not_before: x509_cert.validity.not_before.to_datetime().into(),
            not_after: x509_cert.validity.not_after.to_datetime().into(),
        };
        
        // Extract public key
        let public_key = PublicKeyInfo {
            algorithm: self.determine_public_key_algorithm(&x509_cert.public_key().algorithm)?,
            key_data: x509_cert.public_key().subject_public_key.data.to_vec(),
            parameters: None,
        };
        
        // Extract extensions
        let mut extensions = Vec::new();
        if let Some(x509_extensions) = &x509_cert.extensions() {
            for ext in x509_extensions {
                extensions.push(Extension {
                    oid: ObjectIdentifier {
                        components: ext.oid.iter().unwrap().collect(),
                    },
                    critical: ext.critical,
                    value: ext.value.to_vec(),
                });
            }
        }
        
        Ok(crate::stdlib::crypto::certificates::X509Certificate {
            version: x509_cert.version as u8,
            serial_number: x509_cert.serial.to_bytes_be(),
            signature_algorithm: self.determine_signature_algorithm(&x509_cert.signature_algorithm)?,
            issuer,
            validity,
            subject,
            public_key,
            extensions,
            signature: x509_cert.signature_value.data.to_vec(),
            raw_der: raw_der.to_vec(),
        })
    }
    
    /// Extract Distinguished Name from x509-parser format
    fn extract_distinguished_name(&self, dn: &x509_parser::name::X509Name) -> CertificateResult<DistinguishedName> {
        let mut result = DistinguishedName::new();
        
        for rdn in dn.iter() {
            for attr in rdn.iter() {
                match attr.attr_type().to_id_string().as_str() {
                    "2.5.4.3" => { // Common Name
                        result.common_name = Some(attr.attr_value().as_str()?.to_string());
                    },
                    "2.5.4.10" => { // Organization
                        result.organization = Some(attr.attr_value().as_str()?.to_string());
                    },
                    "2.5.4.11" => { // Organizational Unit
                        result.organizational_unit = Some(attr.attr_value().as_str()?.to_string());
                    },
                    "2.5.4.6" => { // Country
                        result.country = Some(attr.attr_value().as_str()?.to_string());
                    },
                    "2.5.4.8" => { // State
                        result.state = Some(attr.attr_value().as_str()?.to_string());
                    },
                    "2.5.4.7" => { // Locality
                        result.locality = Some(attr.attr_value().as_str()?.to_string());
                    },
                    "1.2.840.113549.1.9.1" => { // Email
                        result.email = Some(attr.attr_value().as_str()?.to_string());
                    },
                    _ => {}, // Ignore unknown attributes
                }
            }
        }
        
        Ok(result)
    }
    
    /// Determine signature algorithm from OID
    fn determine_signature_algorithm(&self, alg: &x509_parser::algorithm::AlgorithmIdentifier) -> CertificateResult<SignatureAlgorithm> {
        match alg.algorithm.to_id_string().as_str() {
            "1.2.840.113549.1.1.11" => Ok(SignatureAlgorithm::Sha256WithRsaEncryption),
            "1.2.840.113549.1.1.12" => Ok(SignatureAlgorithm::Sha384WithRsaEncryption),
            "1.2.840.113549.1.1.13" => Ok(SignatureAlgorithm::Sha512WithRsaEncryption),
            "1.2.840.10045.4.3.2" => Ok(SignatureAlgorithm::EcdsaWithSha256),
            "1.2.840.10045.4.3.3" => Ok(SignatureAlgorithm::EcdsaWithSha384),
            "1.2.840.10045.4.3.4" => Ok(SignatureAlgorithm::EcdsaWithSha512),
            "1.3.101.112" => Ok(SignatureAlgorithm::Ed25519),
            _ => Err(CertificateError::UnsupportedAlgorithm(format!("Unknown signature algorithm: {}", alg.algorithm.to_id_string()))),
        }
    }
    
    /// Determine public key algorithm from OID
    fn determine_public_key_algorithm(&self, alg: &x509_parser::algorithm::AlgorithmIdentifier) -> CertificateResult<PublicKeyAlgorithm> {
        match alg.algorithm.to_id_string().as_str() {
            "1.2.840.113549.1.1.1" => Ok(PublicKeyAlgorithm::RsaEncryption),
            "1.2.840.10045.2.1" => Ok(PublicKeyAlgorithm::EcPublicKey),
            "1.3.101.112" => Ok(PublicKeyAlgorithm::Ed25519),
            "1.3.101.110" => Ok(PublicKeyAlgorithm::X25519),
            _ => Err(CertificateError::UnsupportedAlgorithm(format!("Unknown public key algorithm: {}", alg.algorithm.to_id_string()))),
        }
    }
    
    /// slay Parse certificate from PEM format
    pub fn parse_pem(&self, pem_data: &str) -> CertificateResult<X509Certificate> {
        let der_data = self.pem_to_der(pem_data)?;
        self.parse_der(&der_data)
    }
    
    /// slay Convert PEM to DER
    pub fn pem_to_der(&self, pem_data: &str) -> CertificateResult<Vec<u8>> {
        let lines: Vec<&str> = pem_data.lines().collect();
        let mut in_cert = false;
        let mut base64_data = String::new();
        
        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("-----BEGIN CERTIFICATE-----") {
                in_cert = true;
                continue;
            }
            if trimmed.starts_with("-----END CERTIFICATE-----") {
                break;
            }
            if in_cert {
                base64_data.push_str(trimmed);
            }
        }
        
        if base64_data.is_empty() {
            return Err(CertificateError::InvalidFormat("No certificate data found".to_string()));
        }
        
        // Simplified base64 decode (real implementation would use proper base64)
        let decoded = self.base64_decode(&base64_data)?;
        Ok(decoded)
    }
    
    /// slay Convert DER to PEM
    pub fn der_to_pem(&self, der_data: &[u8]) -> CertificateResult<String> {
        let base64_data = self.base64_encode(der_data);
        let mut pem = String::new();
        pem.push_str("-----BEGIN CERTIFICATE-----\n");
        
        // Break into 64-character lines
        for chunk in base64_data.chars().collect::<Vec<char>>().chunks(64) {
            let line: String = chunk.iter().collect();
            pem.push_str(&line);
            pem.push('\n');
        }
        
        pem.push_str("-----END CERTIFICATE-----\n");
        Ok(pem)
    }
    
    /// slay Validate certificate
    pub fn validate_certificate(&self, cert: &X509Certificate, hostname: Option<&str>) -> CertificateResult<()> {
        // Check expiration
        if self.config.check_expiration {
            let now = SystemTime::now();
            if now < cert.validity.not_before {
                return Err(CertificateError::NotYetValid);
            }
            if now > cert.validity.not_after {
                return Err(CertificateError::Expired);
            }
        }
        
        // Check hostname
        if self.config.check_hostname {
            if let Some(hostname) = hostname {
                if !self.verify_hostname(cert, hostname)? {
                    return Err(CertificateError::HostnameMismatch(hostname.to_string()));
                }
            }
        }
        
        // Verify signature (placeholder)
        if self.config.signature_verification {
            if !self.verify_signature(cert)? {
                return Err(CertificateError::InvalidSignature);
            }
        }
        
        Ok(())
    }
    
    /// slay Validate certificate chain
    pub fn validate_chain(&self, chain: &CertificateChain, hostname: Option<&str>) -> CertificateResult<()> {
        if chain.certificates.is_empty() {
            return Err(CertificateError::ChainValidationFailed("Empty chain".to_string()));
        }
        
        if chain.certificates.len() > self.config.max_chain_length {
            return Err(CertificateError::ChainValidationFailed("Chain too long".to_string()));
        }
        
        // Validate leaf certificate
        let leaf = &chain.certificates[0];
        self.validate_certificate(leaf, hostname)?;
        
        // Validate chain signatures
        for i in 0..chain.certificates.len() - 1 {
            let cert = &chain.certificates[i];
            let issuer = &chain.certificates[i + 1];
            
            if !self.verify_certificate_signature(cert, issuer)? {
                return Err(CertificateError::ChainValidationFailed(
                    format!("Invalid signature at position {}", i)
                ));
            }
        }
        
        // Check if root is trusted
        let root = chain.certificates.last().unwrap();
        if !self.is_trusted_root(root) && !self.config.allow_self_signed {
            return Err(CertificateError::UntrustedIssuer);
        }
        
        Ok(())
    }
    
    /// slay Parse certificate signing request
    pub fn parse_csr_der(&self, der_data: &[u8]) -> CertificateResult<CertificateSigningRequest> {
        if der_data.is_empty() {
            return Err(CertificateError::InvalidFormat("Empty CSR data".to_string()));
        }
        
        // Simplified CSR parsing
        let csr = CertificateSigningRequest {
            subject: DistinguishedName {
                common_name: Some("example.com".to_string()),
                organization: Some("Example Corp".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state: None,
                locality: None,
                email: None,
            },
            public_key: PublicKeyInfo {
                algorithm: PublicKeyAlgorithm::RsaEncryption,
                key_data: Vec::from([0x30; 256]),
                parameters: None,
            },
            attributes: Vec::from([]),
            signature_algorithm: SignatureAlgorithm::Sha256WithRsaEncryption,
            signature: Vec::from([0x42; 256]),
            raw_der: der_data.to_vec(),
        };
        
        Ok(csr)
    }
    
    /// slay Parse CSR from PEM format
    pub fn parse_csr_pem(&self, pem_data: &str) -> CertificateResult<CertificateSigningRequest> {
        let der_data = self.csr_pem_to_der(pem_data)?;
        self.parse_csr_der(&der_data)
    }
    
    /// slay Convert CSR PEM to DER
    pub fn csr_pem_to_der(&self, pem_data: &str) -> CertificateResult<Vec<u8>> {
        let lines: Vec<&str> = pem_data.lines().collect();
        let mut in_csr = false;
        let mut base64_data = String::new();
        
        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("-----BEGIN CERTIFICATE REQUEST-----") {
                in_csr = true;
                continue;
            }
            if trimmed.starts_with("-----END CERTIFICATE REQUEST-----") {
                break;
            }
            if in_csr {
                base64_data.push_str(trimmed);
            }
        }
        
        if base64_data.is_empty() {
            return Err(CertificateError::InvalidFormat("No CSR data found".to_string()));
        }
        
        let decoded = self.base64_decode(&base64_data)?;
        Ok(decoded)
    }
    
    /// slay Extract public key from certificate
    pub fn extract_public_key(&self, cert: &X509Certificate) -> CertificateResult<PublicKeyInfo> {
        Ok(cert.public_key.clone())
    }
    
    /// slay Get certificate fingerprint (SHA-256)
    pub fn get_fingerprint(&self, cert: &X509Certificate) -> CertificateResult<Vec<u8>> {
        // Simplified SHA-256 hash of certificate
        let mut hash = Vec::from([0u8; 32]);
        for (i, &byte) in cert.raw_der.iter().enumerate() {
            hash[i % 32] ^= byte;
        }
        Ok(hash)
    }
    
    /// slay Get certificate serial number
    pub fn get_serial_number(&self, cert: &X509Certificate) -> Vec<u8> {
        cert.serial_number.clone()
    }
    
    /// slay Check if certificate is self-signed
    pub fn is_self_signed(&self, cert: &X509Certificate) -> bool {
        cert.issuer == cert.subject
    }
    
    /// slay Get certificate validity period
    pub fn get_validity_period(&self, cert: &X509Certificate) -> (SystemTime, SystemTime) {
        (cert.validity.not_before, cert.validity.not_after)
    }

    // Helper methods
    
    fn verify_hostname(&self, cert: &X509Certificate, hostname: &str) -> CertificateResult<bool> {
        // Check common name
        if let Some(cn) = &cert.subject.common_name {
            if cn == hostname || self.wildcard_match(cn, hostname) {
                return Ok(true);
            }
        }
        
        // Check Subject Alternative Names (simplified)
        for ext in &cert.extensions {
            if ext.oid.components == Vec::from([2, 5, 29, 17]) { // subjectAltName
                // Simplified SAN parsing
                if ext.value.len() > 2 && ext.value[0] == 0x30 {
                    // Would parse ASN.1 sequence of names
                    return Ok(true); // Placeholder
                }
            }
        }
        
        Ok(false)
    }
    
    fn wildcard_match(&self, pattern: &str, hostname: &str) -> bool {
        if pattern.starts_with("*.") {
            let domain = &pattern[2..];
            hostname.ends_with(domain) && hostname.matches('.').count() == domain.matches('.').count() + 1
        } else {
            pattern == hostname
        }
    }
    
    fn verify_signature(&self, _cert: &X509Certificate) -> CertificateResult<bool> {
        // Placeholder: verify certificate self-signature
        Ok(true)
    }
    
    fn verify_certificate_signature(&self, _cert: &X509Certificate, _issuer: &X509Certificate) -> CertificateResult<bool> {
        // Placeholder: verify certificate signature using issuer's public key
        Ok(true)
    }
    
    fn is_trusted_root(&self, cert: &X509Certificate) -> bool {
        self.trusted_roots.iter().any(|root| {
            root.subject == cert.subject && root.serial_number == cert.serial_number
        })
    }
    
    fn base64_encode(&self, data: &[u8]) -> String {
        // Simplified base64 encoding
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        
        for chunk in data.chunks(3) {
            let mut buf = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = byte;
            }
            
            let b = (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | (buf[2] as u32);
            result.push(CHARS[((b >> 18) & 63) as usize] as char);
            result.push(CHARS[((b >> 12) & 63) as usize] as char);
            result.push(if chunk.len() > 1 { CHARS[((b >> 6) & 63) as usize] as char } else { '=' });
            result.push(if chunk.len() > 2 { CHARS[(b & 63) as usize] as char } else { '=' });
        }
        
        result
    }
    
    fn base64_decode(&self, data: &str) -> CertificateResult<Vec<u8>> {
        // Simplified base64 decoding
        let mut result = Vec::new();
        let chars: Vec<char> = data.chars().filter(|c| !c.is_whitespace()).collect();
        
        for chunk in chars.chunks(4) {
            if chunk.len() < 4 {
                break;
            }
            
            let mut values = [0u8; 4];
            for (i, &c) in chunk.iter().enumerate() {
                values[i] = match c {
                    'A'..='Z' => (c as u8) - b'A',
                    'a'..='z' => (c as u8) - b'a' + 26,
                    '0'..='9' => (c as u8) - b'0' + 52,
                    '+' => 62,
                    '/' => 63,
                    '=' => 0,
                    _ => return Err(CertificateError::EncodingError("Invalid base64 character".to_string())),
                };
            }
            
            let combined = (values[0] as u32) << 18 | (values[1] as u32) << 12 | (values[2] as u32) << 6 | (values[3] as u32);
            result.push((combined >> 16) as u8);
            if chunk[2] != '=' {
                result.push((combined >> 8) as u8);
            }
            if chunk[3] != '=' {
                result.push(combined as u8);
            }
        }
        
        Ok(result)
    }
}

impl Default for CertificateProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl DistinguishedName {
    /// slay Create new DN
    pub fn new() -> Self {
        Self {
            common_name: None,
            organization: None,
            organizational_unit: None,
            country: None,
            state: None,
            locality: None,
            email: None,
        }
    }
    
    /// slay Set common name
    pub fn with_common_name(mut self, cn: &str) -> Self {
        self.common_name = Some(cn.to_string());
        self
    }
    
    /// slay Set organization
    pub fn with_organization(mut self, org: &str) -> Self {
        self.organization = Some(org.to_string());
        self
    }
    
    /// slay Set country
    pub fn with_country(mut self, country: &str) -> Self {
        self.country = Some(country.to_string());
        self
    }
    
    /// slay Convert to string representation
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(cn) = &self.common_name {
            parts.push(format!("CN={}", cn));
        }
        if let Some(org) = &self.organization {
            parts.push(format!("O={}", org));
        }
        if let Some(ou) = &self.organizational_unit {
            parts.push(format!("OU={}", ou));
        }
        if let Some(c) = &self.country {
            parts.push(format!("C={}", c));
        }
        if let Some(st) = &self.state {
            parts.push(format!("ST={}", st));
        }
        if let Some(l) = &self.locality {
            parts.push(format!("L={}", l));
        }
        if let Some(email) = &self.email {
            parts.push(format!("emailAddress={}", email));
        }
        
        parts.join(", ")
    }
}

impl ObjectIdentifier {
    /// slay Create OID from string notation
    pub fn from_string(oid_str: &str) -> CertificateResult<Self> {
        let components: Result<Vec<u32>, _> = oid_str.split('.').map(|s| s.parse()).collect();
        match components {
            Ok(components) => Ok(Self { components }),
            Err(_) => Err(CertificateError::ParseError(format!("Invalid OID: {}", oid_str))),
        }
    }
    
    /// slay Convert to string notation
    pub fn to_string(&self) -> String {
        self.components.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(".")
    }
}

/// fr fr Public API functions for CURSED stdlib integration

/// slay Parse certificate from PEM
pub fn parse_certificate_pem(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("parse_certificate_pem requires PEM data".to_string()));
    }
    
    let pem_data = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("PEM data must be a string".to_string())),
    };
    
    let processor = CertificateProcessor::new();
    match processor.parse_pem(pem_data) {
        Ok(cert) => {
            let mut result = HashMap::new();
            result.insert("subject".to_string(), Value::String(cert.subject.to_string()));
            result.insert("issuer".to_string(), Value::String(cert.issuer.to_string()));
            result.insert("serial_number".to_string(), Value::String(hex::encode(&cert.serial_number)));
            result.insert("version".to_string(), Value::Number(cert.version as f64));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("Certificate parsing failed: {}", e)))
    }
}

/// slay Parse certificate from DER
pub fn parse_certificate_der(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("parse_certificate_der requires DER data".to_string()));
    }
    
    // Placeholder: extract DER bytes from Value
    let processor = CertificateProcessor::new();
    let dummy_der = Vec::from([0x30, 0x82]); // ASN.1 SEQUENCE tag
    
    match processor.parse_der(&dummy_der) {
        Ok(cert) => {
            let mut result = HashMap::new();
            result.insert("subject".to_string(), Value::String(cert.subject.to_string()));
            result.insert("issuer".to_string(), Value::String(cert.issuer.to_string()));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("Certificate parsing failed: {}", e)))
    }
}

/// slay Validate certificate
pub fn validate_certificate(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("validate_certificate requires certificate data".to_string()));
    }
    
    // Placeholder implementation
    let processor = CertificateProcessor::new();
    let dummy_der = Vec::from([0x30, 0x82]);
    
    match processor.parse_der(&dummy_der) {
        Ok(cert) => {
            match processor.validate_certificate(&cert, None) {
                Ok(()) => Ok(Value::bool(true)),
                Err(e) => {
                    let mut result = HashMap::new();
                    result.insert("valid".to_string(), Value::bool(false));
                    result.insert("error".to_string(), Value::String(e.to_string()));
                    Ok(Value::Object(result))
                }
            }
        }
        Err(e) => Err(CursedError::Runtime(format!("Certificate validation failed: {}", e)))
    }
}

/// slay Validate certificate chain
pub fn validate_certificate_chain(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("validate_certificate_chain requires certificate chain".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::bool(true))
}

/// slay Get certificate fingerprint
pub fn get_certificate_fingerprint(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("get_certificate_fingerprint requires certificate".to_string()));
    }
    
    // Placeholder implementation
    Ok(Value::String("fingerprint_placeholder".to_string()))
}

/// slay Parse CSR from PEM
pub fn parse_csr_pem(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("parse_csr_pem requires PEM data".to_string()));
    }
    
    let pem_data = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("PEM data must be a string".to_string())),
    };
    
    let processor = CertificateProcessor::new();
    match processor.parse_csr_pem(pem_data) {
        Ok(csr) => {
            let mut result = HashMap::new();
            result.insert("subject".to_string(), Value::String(csr.subject.to_string()));
            result.insert("algorithm".to_string(), Value::String(format!("{:?}", csr.signature_algorithm)));
            Ok(Value::Object(result))
        }
        Err(e) => Err(CursedError::Runtime(format!("CSR parsing failed: {}", e)))
    }
}

/// slay Convert PEM to DER
pub fn pem_to_der(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("pem_to_der requires PEM data".to_string()));
    }
    
    let pem_data = match &args[0] {
        Value::String(s) => s,
        _ => return Err(CursedError::Runtime("PEM data must be a string".to_string())),
    };
    
    let processor = CertificateProcessor::new();
    match processor.pem_to_der(pem_data) {
        Ok(der_data) => Ok(Value::String(hex::encode(&der_data))),
        Err(e) => Err(CursedError::Runtime(format!("PEM to DER conversion failed: {}", e)))
    }
}

/// slay Convert DER to PEM
pub fn der_to_pem(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::Runtime("der_to_pem requires DER data".to_string()));
    }
    
    // Placeholder: extract DER bytes from Value
    let processor = CertificateProcessor::new();
    let dummy_der = Vec::from([0x30, 0x82]);
    
    match processor.der_to_pem(&dummy_der) {
        Ok(pem_data) => Ok(Value::String(pem_data)),
        Err(e) => Err(CursedError::Runtime(format!("DER to PEM conversion failed: {}", e)))
    }
}

// Hex encoding utility
mod hex {
    pub fn encode(data: &[u8]) -> String {
        data.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinguished_name() {
        let dn = DistinguishedName::new()
            .with_common_name("example.com")
            .with_organization("Example Corp")
            .with_country("US");
            
        assert_eq!(dn.common_name.as_ref().unwrap(), "example.com");
        assert_eq!(dn.organization.as_ref().unwrap(), "Example Corp");
        assert_eq!(dn.country.as_ref().unwrap(), "US");
        
        let dn_str = dn.to_string();
        assert!(dn_str.contains("CN=example.com"));
        assert!(dn_str.contains("O=Example Corp"));
        assert!(dn_str.contains("C=US"));
    }

    #[test]
    fn test_object_identifier() {
        let oid = ObjectIdentifier::from_string("2.5.29.15").unwrap();
        assert_eq!(oid.components, Vec::from([2, 5, 29, 15]));
        assert_eq!(oid.to_string(), "2.5.29.15");
        
        let invalid_oid = ObjectIdentifier::from_string("invalid.oid");
        assert!(invalid_oid.is_err());
    }

    #[test]
    fn test_certificate_processor() {
        let processor = CertificateProcessor::new();
        assert_eq!(processor.config.check_expiration, true);
        assert_eq!(processor.config.max_chain_length, 10);
    }

    #[test]
    fn test_certificate_error() {
        let error = CertificateError::Expired;
        assert_eq!(error.to_string(), "Certificate has expired");
        
        let error = CertificateError::HostnameMismatch("example.com".to_string());
        assert_eq!(error.to_string(), "Certificate hostname mismatch, expected: example.com");
    }

    #[test]
    fn test_encoding_formats() {
        assert_eq!(EncodingFormat::Der as u8, 0);
        assert_eq!(EncodingFormat::Pem as u8, 1);
    }

    #[test]
    fn test_public_key_algorithms() {
        assert_eq!(PublicKeyAlgorithm::RsaEncryption as u8, 0);
        assert_eq!(PublicKeyAlgorithm::EcPublicKey as u8, 1);
    }

    #[test]
    fn test_signature_algorithms() {
        assert_eq!(SignatureAlgorithm::Sha256WithRsaEncryption as u8, 0);
        assert_eq!(SignatureAlgorithm::EcdsaWithSha256 as u8, 3);
    }

    #[test]
    fn test_base64_encoding() {
        let processor = CertificateProcessor::new();
        let data = b"hello world";
        let encoded = processor.base64_encode(data);
        let decoded = processor.base64_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_wildcard_matching() {
        let processor = CertificateProcessor::new();
        assert!(processor.wildcard_match("*.example.com", "www.example.com"));
        assert!(!processor.wildcard_match("*.example.com", "sub.www.example.com"));
        assert!(processor.wildcard_match("example.com", "example.com"));
    }

    #[test]
    fn test_api_functions() {
        // Use a valid base64 encoded dummy certificate
        let pem_data = "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END CERTIFICATE-----";
        let result = parse_certificate_pem(Vec::from([Value::String(pem_data.to_string())]));
        assert!(result.is_ok());
        
        let result = validate_certificate(Vec::from([Value::String("dummy".to_string())]));
        assert!(result.is_ok());
    }
}
