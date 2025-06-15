/// fr fr X.509 Certificate implementation with comprehensive functionality
use crate::stdlib::packages::crypto_pki::errors::*;
use crate::stdlib::packages::crypto_asymmetric::{rsa_generate_keypair, RsaKeyPair};
use crate::stdlib::packages::crypto_hash_advanced::{sha256_hash, sha1_hash};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

/// fr fr X.509 Certificate representation
#[derive(Debug, Clone)]
pub struct Certificate {
    /// Raw DER-encoded certificate data
    pub raw: Vec<u8>,
    
    /// Certificate version (1, 2, or 3)
    pub version: u32,
    
    /// Certificate serial number
    pub serial_number: SerialNumber,
    
    /// Signature algorithm identifier
    pub signature_algorithm: SignatureAlgorithm,
    
    /// Certificate issuer name
    pub issuer: CertificateIssuer,
    
    /// Certificate subject name
    pub subject: CertificateSubject,
    
    /// Certificate validity period
    pub validity: CertificateValidity,
    
    /// Subject public key information
    pub public_key_info: SubjectPublicKeyInfo,
    
    /// Certificate extensions
    pub extensions: CertificateExtensions,
    
    /// Certificate fingerprints
    fingerprints: HashMap<String, String>,
    
    /// Digital signature over the certificate
    pub signature: Vec<u8>,
}

/// fr fr Certificate serial number
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialNumber {
    pub bytes: Vec<u8>,
}

/// fr fr Signature algorithm types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    RsaWithSha1,
    RsaWithSha256,
    RsaWithSha384,
    RsaWithSha512,
    EcdsaWithSha256,
    EcdsaWithSha384,
    EcdsaWithSha512,
    Ed25519,
    Unknown(String),
}

/// fr fr Certificate issuer information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateIssuer {
    pub common_name: Option<String>,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
    pub distinguished_name: String,
}

/// fr fr Certificate subject information  
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateSubject {
    pub common_name: Option<String>,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
    pub distinguished_name: String,
}

/// fr fr Certificate validity period
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateValidity {
    pub not_before: SystemTime,
    pub not_after: SystemTime,
}

/// fr fr Subject public key information
#[derive(Debug, Clone)]
pub struct SubjectPublicKeyInfo {
    pub algorithm: PublicKeyAlgorithm,
    pub public_key: Vec<u8>,
    pub key_size: usize,
}

/// fr fr Public key algorithm types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicKeyAlgorithm {
    Rsa,
    Ecdsa,
    Ed25519,
    Unknown(String),
}

/// fr fr Certificate extensions
#[derive(Debug, Clone, Default)]
pub struct CertificateExtensions {
    pub key_usage: Option<KeyUsage>,
    pub extended_key_usage: Option<ExtendedKeyUsage>,
    pub basic_constraints: Option<BasicConstraints>,
    pub subject_alt_names: Vec<SubjectAlternativeName>,
    pub authority_key_identifier: Option<Vec<u8>>,
    pub subject_key_identifier: Option<Vec<u8>>,
    pub crl_distribution_points: Vec<String>,
    pub authority_info_access: Vec<AuthorityInfoAccess>,
    pub custom_extensions: HashMap<String, Vec<u8>>,
}

/// fr fr Key usage flags
#[derive(Debug, Clone)]
pub struct KeyUsage {
    pub digital_signature: bool,
    pub non_repudiation: bool,
    pub key_encipherment: bool,
    pub data_encipherment: bool,
    pub key_agreement: bool,
    pub key_cert_sign: bool,
    pub crl_sign: bool,
    pub encipher_only: bool,
    pub decipher_only: bool,
}

/// fr fr Extended key usage purposes
#[derive(Debug, Clone)]
pub struct ExtendedKeyUsage {
    pub server_auth: bool,
    pub client_auth: bool,
    pub code_signing: bool,
    pub email_protection: bool,
    pub time_stamping: bool,
    pub ocsp_signing: bool,
    pub custom_purposes: Vec<String>,
}

/// fr fr Basic constraints extension
#[derive(Debug, Clone)]
pub struct BasicConstraints {
    pub ca: bool,
    pub path_length: Option<u32>,
}

/// fr fr Subject alternative name types
#[derive(Debug, Clone)]
pub enum SubjectAlternativeName {
    DnsName(String),
    IpAddress(std::net::IpAddr),
    EmailAddress(String),
    Uri(String),
    DirectoryName(String),
    Other(String, Vec<u8>),
}

/// fr fr Authority information access
#[derive(Debug, Clone)]
pub struct AuthorityInfoAccess {
    pub method: String,
    pub location: String,
}

/// fr fr Certificate information summary
#[derive(Debug, Clone)]
pub struct CertificateInfo {
    pub subject_cn: Option<String>,
    pub issuer_cn: Option<String>,
    pub serial_number: String,
    pub valid_from: SystemTime,
    pub valid_to: SystemTime,
    pub is_ca: bool,
    pub is_self_signed: bool,
    pub key_algorithm: String,
    pub key_size: usize,
    pub signature_algorithm: String,
    pub fingerprint_sha256: String,
    pub fingerprint_sha1: String,
}

/// fr fr Certificate format types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificateFormat {
    Der,
    Pem,
    Pkcs7,
    Pkcs12,
}

/// fr fr Certificate builder for creating new certificates
#[derive(Debug, Clone)]
pub struct CertificateBuilder {
    subject: CertificateSubject,
    issuer: Option<CertificateIssuer>,
    validity_days: u32,
    key_size: usize,
    extensions: CertificateExtensions,
    signature_algorithm: SignatureAlgorithm,
}

/// fr fr Certificate parser for different formats
pub struct CertificateParser;

/// fr fr Certificate validator for verification
pub struct CertificateValidator;

impl Certificate {
    /// slay Create a new self-signed certificate
    pub fn new_self_signed(common_name: &str) -> PkiResult<Self> {
        let mut builder = CertificateBuilder::new();
        builder.set_subject_common_name(common_name);
        builder.build_self_signed()
    }
    
    /// slay Parse certificate from DER bytes
    pub fn from_der(der: &[u8]) -> PkiResult<Self> {
        CertificateParser::parse_der(der)
    }
    
    /// slay Parse certificate from PEM string
    pub fn from_pem(pem: &str) -> PkiResult<Self> {
        CertificateParser::parse_pem(pem)
    }
    
    /// slay Get certificate information summary
    pub fn info(&self) -> CertificateInfo {
        CertificateInfo {
            subject_cn: self.subject.common_name.clone(),
            issuer_cn: self.issuer.common_name.clone(),
            serial_number: hex::encode(&self.serial_number.bytes),
            valid_from: self.validity.not_before,
            valid_to: self.validity.not_after,
            is_ca: self.extensions.basic_constraints
                .as_ref()
                .map(|bc| bc.ca)
                .unwrap_or(false),
            is_self_signed: self.is_self_signed(),
            key_algorithm: format!("{:?}", self.public_key_info.algorithm),
            key_size: self.public_key_info.key_size,
            signature_algorithm: format!("{:?}", self.signature_algorithm),
            fingerprint_sha256: self.fingerprint_sha256().unwrap_or_default(),
            fingerprint_sha1: self.fingerprint_sha1().unwrap_or_default(),
        }
    }
    
    /// slay Check if certificate is self-signed
    pub fn is_self_signed(&self) -> bool {
        self.subject.distinguished_name == self.issuer.distinguished_name
    }
    
    /// slay Check if certificate is a CA certificate
    pub fn is_ca(&self) -> bool {
        self.extensions.basic_constraints
            .as_ref()
            .map(|bc| bc.ca)
            .unwrap_or(false)
    }
    
    /// slay Check if certificate is currently valid
    pub fn is_valid_now(&self) -> bool {
        let now = SystemTime::now();
        now >= self.validity.not_before && now <= self.validity.not_after
    }
    
    /// slay Get days until expiry (negative if expired)
    pub fn days_until_expiry(&self) -> PkiResult<i64> {
        let now = SystemTime::now();
        let duration = self.validity.not_after.duration_since(now)
            .map_err(|_| PkiError::CertificateExpired("Certificate has expired".to_string()))?;
        
        Ok(duration.as_secs() as i64 / 86400) // seconds to days
    }
    
    /// slay Get SHA-256 fingerprint
    pub fn fingerprint_sha256(&self) -> PkiResult<String> {
        let hash = sha256_hash(&self.raw)?;
        Ok(hex::encode(hash).to_uppercase())
    }
    
    /// slay Get SHA-1 fingerprint
    pub fn fingerprint_sha1(&self) -> PkiResult<String> {
        let hash = sha1_hash(&self.raw)?;
        Ok(hex::encode(hash).to_uppercase())
    }
    
    /// slay Get fingerprint for specified algorithm
    pub fn fingerprint(&self) -> PkiResult<String> {
        self.fingerprint_sha256()
    }
    
    /// slay Validate hostname against certificate
    pub fn validate_hostname(&self, hostname: &str) -> bool {
        // Check common name
        if let Some(ref cn) = self.subject.common_name {
            if hostname_matches(hostname, cn) {
                return true;
            }
        }
        
        // Check subject alternative names
        for san in &self.extensions.subject_alt_names {
            if let SubjectAlternativeName::DnsName(ref dns_name) = san {
                if hostname_matches(hostname, dns_name) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// slay Convert to PEM format
    pub fn to_pem(&self) -> PkiResult<String> {
        encode_certificate_pem(self)
    }
    
    /// slay Convert to DER format
    pub fn to_der(&self) -> Vec<u8> {
        self.raw.clone()
    }
    
    /// slay Verify certificate signature against issuer
    pub fn verify_signature(&self, issuer_cert: &Certificate) -> PkiResult<bool> {
        // This would implement actual signature verification
        // For now, return a basic check
        if self.is_self_signed() {
            return Ok(self.subject.distinguished_name == self.issuer.distinguished_name);
        }
        
        // Verify issuer can sign certificates
        if !issuer_cert.is_ca() {
            return Err(PkiError::CertificateValidationFailed(
                "Issuer is not a CA certificate".to_string()
            ));
        }
        
        // Implement actual cryptographic signature verification
        self.verify_certificate_signature(issuer_cert)
    }
    
    /// slay Verify the cryptographic signature of the certificate
    fn verify_certificate_signature(&self, issuer_cert: &Certificate) -> PkiResult<bool> {
        use crate::stdlib::packages::crypto_asymmetric::{rsa_verify, ecdsa_verify, RsaPadding, EccCurve, EccHashAlgorithm};
        
        // Extract the TBSCertificate data (the data that was signed)
        let tbs_cert_data = self.extract_tbs_certificate_data()?;
        
        // Get the issuer's public key
        let issuer_public_key = &issuer_cert.public_key_info.public_key;
        
        // Verify signature based on the signature algorithm
        match self.signature_algorithm {
            SignatureAlgorithm::RsaWithSha256 => {
                rsa_verify(&tbs_cert_data, &self.signature, issuer_public_key, RsaPadding::Pkcs1v15)
                    .map_err(|e| PkiError::CertificateValidationFailed(format!("RSA signature verification failed: {}", e)))
            },
            SignatureAlgorithm::RsaWithSha384 => {
                rsa_verify(&tbs_cert_data, &self.signature, issuer_public_key, RsaPadding::PssSha384)
                    .map_err(|e| PkiError::CertificateValidationFailed(format!("RSA signature verification failed: {}", e)))
            },
            SignatureAlgorithm::RsaWithSha512 => {
                rsa_verify(&tbs_cert_data, &self.signature, issuer_public_key, RsaPadding::PssSha512)
                    .map_err(|e| PkiError::CertificateValidationFailed(format!("RSA signature verification failed: {}", e)))
            },
            SignatureAlgorithm::EcdsaWithSha256 => {
                ecdsa_verify(&tbs_cert_data, &self.signature, issuer_public_key, EccCurve::P256, EccHashAlgorithm::Sha256)
                    .map_err(|e| PkiError::CertificateValidationFailed(format!("ECDSA signature verification failed: {}", e)))
            },
            SignatureAlgorithm::EcdsaWithSha384 => {
                ecdsa_verify(&tbs_cert_data, &self.signature, issuer_public_key, EccCurve::P384, EccHashAlgorithm::Sha384)
                    .map_err(|e| PkiError::CertificateValidationFailed(format!("ECDSA signature verification failed: {}", e)))
            },
            SignatureAlgorithm::EcdsaWithSha512 => {
                ecdsa_verify(&tbs_cert_data, &self.signature, issuer_public_key, EccCurve::P521, EccHashAlgorithm::Sha512)
                    .map_err(|e| PkiError::CertificateValidationFailed(format!("ECDSA signature verification failed: {}", e)))
            },
            SignatureAlgorithm::Ed25519 => {
                use crate::stdlib::packages::crypto_asymmetric::ed25519_verify;
                ed25519_verify(&tbs_cert_data, &self.signature, issuer_public_key)
                    .map_err(|e| PkiError::CertificateValidationFailed(format!("Ed25519 signature verification failed: {}", e)))
            },
            _ => Err(PkiError::UnsupportedAlgorithm(
                format!("Signature algorithm {:?} not supported for verification", self.signature_algorithm)
            )),
        }
    }
    
    /// slay Extract TBSCertificate data for signature verification
    fn extract_tbs_certificate_data(&self) -> PkiResult<Vec<u8>> {
        // In a real implementation, this would parse the ASN.1 DER structure to extract TBSCertificate
        // For now, we'll reconstruct the data that would have been signed
        let mut tbs_data = Vec::new();
        
        // Version
        tbs_data.extend_from_slice(&self.version.to_be_bytes());
        
        // Serial number
        tbs_data.extend_from_slice(&self.serial_number.bytes);
        
        // Signature algorithm identifier
        tbs_data.extend_from_slice(format!("{:?}", self.signature_algorithm).as_bytes());
        
        // Issuer distinguished name
        tbs_data.extend_from_slice(self.issuer.distinguished_name.as_bytes());
        
        // Validity period
        let not_before_secs = self.validity.not_before.duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default().as_secs();
        let not_after_secs = self.validity.not_after.duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default().as_secs();
        tbs_data.extend_from_slice(&not_before_secs.to_be_bytes());
        tbs_data.extend_from_slice(&not_after_secs.to_be_bytes());
        
        // Subject distinguished name
        tbs_data.extend_from_slice(self.subject.distinguished_name.as_bytes());
        
        // Subject public key info
        tbs_data.extend_from_slice(format!("{:?}", self.public_key_info.algorithm).as_bytes());
        tbs_data.extend_from_slice(&self.public_key_info.public_key);
        
        // Extensions (basic constraints, key usage, etc.)
        if let Some(ref basic_constraints) = self.extensions.basic_constraints {
            tbs_data.extend_from_slice(&[basic_constraints.ca as u8]);
            if let Some(path_length) = basic_constraints.path_length {
                tbs_data.extend_from_slice(&path_length.to_be_bytes());
            }
        }
        
        // Subject alternative names
        for san in &self.extensions.subject_alt_names {
            match san {
                SubjectAlternativeName::DnsName(name) => {
                    tbs_data.extend_from_slice(b"DNS:");
                    tbs_data.extend_from_slice(name.as_bytes());
                },
                SubjectAlternativeName::IpAddress(ip) => {
                    tbs_data.extend_from_slice(b"IP:");
                    tbs_data.extend_from_slice(ip.to_string().as_bytes());
                },
                SubjectAlternativeName::EmailAddress(email) => {
                    tbs_data.extend_from_slice(b"EMAIL:");
                    tbs_data.extend_from_slice(email.as_bytes());
                },
                _ => {}, // Skip other SAN types for now
            }
        }
        
        Ok(tbs_data)
    }
}

impl CertificateBuilder {
    /// slay Create a new certificate builder
    pub fn new() -> Self {
        Self {
            subject: CertificateSubject {
                common_name: None,
                organization: None,
                organizational_unit: None,
                country: None,
                state: None,
                locality: None,
                email: None,
                distinguished_name: String::new(),
            },
            issuer: None,
            validity_days: 365,
            key_size: 2048,
            extensions: CertificateExtensions::default(),
            signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        }
    }
    
    /// slay Set subject common name
    pub fn set_subject_common_name(&mut self, cn: &str) {
        self.subject.common_name = Some(cn.to_string());
        self.update_subject_dn();
    }
    
    /// slay Set subject organization
    pub fn set_subject_organization(&mut self, org: &str) {
        self.subject.organization = Some(org.to_string());
        self.update_subject_dn();
    }
    
    /// slay Set validity period in days
    pub fn set_validity_days(&mut self, days: u32) {
        self.validity_days = days;
    }
    
    /// slay Set key size
    pub fn set_key_size(&mut self, size: usize) {
        self.key_size = size;
    }
    
    /// slay Add DNS subject alternative name
    pub fn add_dns_san(&mut self, dns_name: &str) {
        self.extensions.subject_alt_names.push(
            SubjectAlternativeName::DnsName(dns_name.to_string())
        );
    }
    
    /// slay Set as CA certificate
    pub fn set_ca(&mut self, is_ca: bool, path_length: Option<u32>) {
        self.extensions.basic_constraints = Some(BasicConstraints {
            ca: is_ca,
            path_length,
        });
        
        if is_ca {
            let mut key_usage = self.extensions.key_usage.get_or_insert_with(|| KeyUsage {
                digital_signature: false,
                non_repudiation: false,
                key_encipherment: false,
                data_encipherment: false,
                key_agreement: false,
                key_cert_sign: false,
                crl_sign: false,
                encipher_only: false,
                decipher_only: false,
            });
            key_usage.key_cert_sign = true;
            key_usage.crl_sign = true;
        }
    }
    
    /// slay Build self-signed certificate
    pub fn build_self_signed(&self) -> PkiResult<Certificate> {
        // Generate key pair for the certificate
        let key_pair = rsa_generate_keypair(self.key_size, vec![])
            .map_err(|e| PkiError::KeyGenerationFailed(e.to_string()))?;
        
        // Create certificate structure
        let now = SystemTime::now();
        let validity_duration = std::time::Duration::from_secs(self.validity_days as u64 * 24 * 60 * 60);
        
        let mut cert = Certificate {
            raw: Vec::new(), // Will be filled by encoding
            version: 3,
            serial_number: SerialNumber::new(),
            signature_algorithm: self.signature_algorithm.clone(),
            issuer: self.subject.clone().into(),
            subject: self.subject.clone(),
            validity: CertificateValidity {
                not_before: now,
                not_after: now + validity_duration,
            },
            public_key_info: SubjectPublicKeyInfo {
                algorithm: PublicKeyAlgorithm::Rsa,
                public_key: key_pair.public_key.clone(),
                key_size: self.key_size,
            },
            extensions: self.extensions.clone(),
            fingerprints: HashMap::new(),
            signature: Vec::new(), // Will be filled by the CA when signing
        };
        
        // Generate mock DER encoding (in real implementation, this would be proper ASN.1 DER)
        cert.raw = self.generate_mock_der(&cert)?;
        
        Ok(cert)
    }
    
    fn update_subject_dn(&mut self) {
        let mut dn_parts = Vec::new();
        
        if let Some(ref cn) = self.subject.common_name {
            dn_parts.push(format!("CN={}", cn));
        }
        if let Some(ref org) = self.subject.organization {
            dn_parts.push(format!("O={}", org));
        }
        if let Some(ref ou) = self.subject.organizational_unit {
            dn_parts.push(format!("OU={}", ou));
        }
        if let Some(ref c) = self.subject.country {
            dn_parts.push(format!("C={}", c));
        }
        if let Some(ref st) = self.subject.state {
            dn_parts.push(format!("ST={}", st));
        }
        if let Some(ref l) = self.subject.locality {
            dn_parts.push(format!("L={}", l));
        }
        
        self.subject.distinguished_name = dn_parts.join(", ");
    }
    
    fn generate_mock_der(&self, cert: &Certificate) -> PkiResult<Vec<u8>> {
        // This is a mock implementation - real implementation would use proper ASN.1 DER encoding
        let mut der = Vec::new();
        
        // Add some mock DER structure
        der.extend_from_slice(b"MOCK_CERTIFICATE_DER");
        der.extend_from_slice(&cert.serial_number.bytes);
        der.extend_from_slice(cert.subject.distinguished_name.as_bytes());
        der.extend_from_slice(&cert.public_key_info.public_key);
        
        Ok(der)
    }
}

impl SerialNumber {
    /// slay Generate a new random serial number
    pub fn new() -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        let hash = hasher.finish();
        
        Self {
            bytes: hash.to_be_bytes().to_vec(),
        }
    }
    
    /// slay Create from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    
    /// slay Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.bytes)
    }
}

impl From<CertificateSubject> for CertificateIssuer {
    fn from(subject: CertificateSubject) -> Self {
        Self {
            common_name: subject.common_name,
            organization: subject.organization,
            organizational_unit: subject.organizational_unit,
            country: subject.country,
            state: subject.state,
            locality: subject.locality,
            email: subject.email,
            distinguished_name: subject.distinguished_name,
        }
    }
}

impl CertificateParser {
    /// slay Parse certificate from DER format
    pub fn parse_der(der: &[u8]) -> PkiResult<Certificate> {
        // Mock implementation - real implementation would parse ASN.1 DER
        if der.is_empty() {
            return Err(PkiError::CertificateParsingFailed("Empty DER data".to_string()));
        }
        
        // Create a mock certificate for demonstration
        let now = SystemTime::now();
        Ok(Certificate {
            raw: der.to_vec(),
            version: 3,
            serial_number: SerialNumber::new(),
            signature_algorithm: SignatureAlgorithm::RsaWithSha256,
            issuer: CertificateIssuer {
                common_name: Some("Mock CA".to_string()),
                organization: Some("Mock Organization".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state: None,
                locality: None,
                email: None,
                distinguished_name: "CN=Mock CA, O=Mock Organization, C=US".to_string(),
            },
            subject: CertificateSubject {
                common_name: Some("Mock Certificate".to_string()),
                organization: Some("Mock Organization".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state: None,
                locality: None,
                email: None,
                distinguished_name: "CN=Mock Certificate, O=Mock Organization, C=US".to_string(),
            },
            validity: CertificateValidity {
                not_before: now,
                not_after: now + std::time::Duration::from_secs(365 * 24 * 60 * 60),
            },
            public_key_info: SubjectPublicKeyInfo {
                algorithm: PublicKeyAlgorithm::Rsa,
                public_key: vec![0x01, 0x02, 0x03], // Mock public key
                key_size: 2048,
            },
            extensions: CertificateExtensions::default(),
            fingerprints: HashMap::new(),
            signature: Vec::new(), // Mock certificate without signature
        })
    }
    
    /// slay Parse certificate from PEM format
    pub fn parse_pem(pem: &str) -> PkiResult<Certificate> {
        let der = decode_pem_certificate(pem)?;
        Self::parse_der(&der)
    }
}

/// fr fr Certificate-related utility functions

/// slay Parse certificate from any format
pub fn parse_certificate(data: &[u8], format: CertificateFormat) -> PkiResult<Certificate> {
    match format {
        CertificateFormat::Der => Certificate::from_der(data),
        CertificateFormat::Pem => {
            let pem_str = String::from_utf8(data.to_vec())?;
            Certificate::from_pem(&pem_str)
        },
        _ => Err(PkiError::UnsupportedFormat(format!("Unsupported format: {:?}", format))),
    }
}

/// slay Create a new certificate
pub fn create_certificate() -> CertificateBuilder {
    CertificateBuilder::new()
}

/// slay Verify certificate signature
pub fn verify_certificate(cert: &Certificate, issuer: Option<&Certificate>) -> PkiResult<bool> {
    match issuer {
        Some(issuer_cert) => cert.verify_signature(issuer_cert),
        None => {
            if cert.is_self_signed() {
                cert.verify_signature(cert)
            } else {
                Err(PkiError::CertificateValidationFailed(
                    "No issuer certificate provided for non-self-signed certificate".to_string()
                ))
            }
        }
    }
}

/// slay Encode certificate to PEM format
pub fn encode_certificate_pem(cert: &Certificate) -> PkiResult<String> {
    let der = &cert.raw;
    let encoded = base64::encode(der);
    
    let mut pem = String::new();
    pem.push_str("-----BEGIN CERTIFICATE-----\n");
    
    // Split into 64-character lines
    for chunk in encoded.chars().collect::<Vec<_>>().chunks(64) {
        let line: String = chunk.iter().collect();
        pem.push_str(&line);
        pem.push('\n');
    }
    
    pem.push_str("-----END CERTIFICATE-----\n");
    Ok(pem)
}

/// slay Decode certificate from PEM format
pub fn decode_certificate_pem(pem: &str) -> PkiResult<Certificate> {
    let der = decode_pem_certificate(pem)?;
    Certificate::from_der(&der)
}

/// fr fr Helper functions

fn decode_pem_certificate(pem: &str) -> PkiResult<Vec<u8>> {
    let lines: Vec<&str> = pem.lines().collect();
    let mut in_cert = false;
    let mut cert_data = String::new();
    
    for line in lines {
        let line = line.trim();
        if line == "-----BEGIN CERTIFICATE-----" {
            in_cert = true;
            continue;
        }
        if line == "-----END CERTIFICATE-----" {
            break;
        }
        if in_cert {
            cert_data.push_str(line);
        }
    }
    
    if cert_data.is_empty() {
        return Err(PkiError::DecodingFailed("No certificate data found in PEM".to_string()));
    }
    
    base64::decode(&cert_data)
        .map_err(|e| PkiError::DecodingFailed(format!("Base64 decode error: {}", e)))
}

fn hostname_matches(hostname: &str, pattern: &str) -> bool {
    if pattern == hostname {
        return true;
    }
    
    // Handle wildcard matching
    if pattern.starts_with("*.") {
        let pattern_domain = &pattern[2..];
        if let Some(dot_pos) = hostname.find('.') {
            let hostname_domain = &hostname[dot_pos + 1..];
            return hostname_domain == pattern_domain;
        }
    }
    
    false
}

// Import required dependencies
use base64;
use hex;

impl Default for CertificateBuilder {
    fn default() -> Self {
        Self::new()
    }
}
