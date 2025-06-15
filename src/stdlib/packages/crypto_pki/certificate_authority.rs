/// fr fr Certificate Authority (CA) implementation with full lifecycle management
use crate::stdlib::packages::crypto_pki::errors::*;
use crate::stdlib::packages::crypto_pki::certificate::*;
use crate::stdlib::packages::crypto_asymmetric::{rsa_generate_keypair, RsaKeyPair, EccKeyPair};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

/// fr fr Certificate Authority main structure
#[derive(Debug, Clone)]
pub struct CertificateAuthority {
    /// CA configuration
    pub config: CaConfiguration,
    
    /// CA certificate
    pub certificate: Certificate,
    
    /// CA private key (encrypted)
    private_key: Vec<u8>,
    
    /// CA metadata
    pub metadata: CaMetadata,
    
    /// CA policy settings
    pub policy: CaPolicy,
    
    /// Issued certificates database
    issued_certificates: HashMap<String, Certificate>,
    
    /// Revoked certificates
    revoked_certificates: HashMap<String, RevocationEntry>,
}

/// fr fr CA configuration settings
#[derive(Debug, Clone)]
pub struct CaConfiguration {
    pub name: String,
    pub key_size: usize,
    pub signature_algorithm: SignatureAlgorithm,
    pub validity_days: u32,
    pub country: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub organization: String,
    pub organizational_unit: Option<String>,
    pub email: Option<String>,
    pub enable_crl: bool,
    pub crl_distribution_points: Vec<String>,
    pub ocsp_responder_url: Option<String>,
}

/// fr fr CA key pair structure
#[derive(Debug, Clone)]
pub struct CaKeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub algorithm: PublicKeyAlgorithm,
    pub key_size: usize,
}

/// fr fr CA policy settings
#[derive(Debug, Clone)]
pub struct CaPolicy {
    pub max_validity_days: u32,
    pub require_san: bool,
    pub allowed_key_sizes: Vec<usize>,
    pub allowed_signature_algorithms: Vec<SignatureAlgorithm>,
    pub path_length_constraint: Option<u32>,
    pub name_constraints: Vec<String>,
    pub policy_oids: Vec<String>,
}

/// fr fr CA profile for different certificate types
#[derive(Debug, Clone)]
pub struct CaProfile {
    pub name: String,
    pub validity_days: u32,
    pub key_usage: KeyUsage,
    pub extended_key_usage: Option<ExtendedKeyUsage>,
    pub basic_constraints: Option<BasicConstraints>,
    pub require_san: bool,
}

/// fr fr Root CA structure
#[derive(Debug, Clone)]
pub struct RootCa {
    pub ca: CertificateAuthority,
    pub subordinates: Vec<IntermediateCa>,
}

/// fr fr Intermediate CA structure
#[derive(Debug, Clone)]
pub struct IntermediateCa {
    pub ca: CertificateAuthority,
    pub parent: Box<Certificate>,
    pub subordinates: Vec<SubordinateCa>,
}

/// fr fr Subordinate CA structure
#[derive(Debug, Clone)]
pub struct SubordinateCa {
    pub ca: CertificateAuthority,
    pub parent: Box<Certificate>,
}

/// fr fr CA hierarchy management
#[derive(Debug, Clone)]
pub struct CaHierarchy {
    pub root: RootCa,
    pub depth: usize,
    pub trust_anchor: Certificate,
}

/// fr fr CA manager for multiple CAs
#[derive(Debug)]
pub struct CaManager {
    pub cas: HashMap<String, CertificateAuthority>,
    pub hierarchies: HashMap<String, CaHierarchy>,
    pub policies: HashMap<String, CaPolicy>,
    pub profiles: HashMap<String, CaProfile>,
}

/// fr fr CA metadata
#[derive(Debug, Clone)]
pub struct CaMetadata {
    pub ca_id: String,
    pub created_at: SystemTime,
    pub last_crl_update: Option<SystemTime>,
    pub next_crl_update: Option<SystemTime>,
    pub certificates_issued: u64,
    pub certificates_revoked: u64,
    pub status: CaStatus,
    pub version: String,
}

/// fr fr CA status enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaStatus {
    Active,
    Suspended,
    Revoked,
    Expired,
    Compromised,
}

/// fr fr Revocation entry
#[derive(Debug, Clone)]
pub struct RevocationEntry {
    pub serial_number: String,
    pub revocation_time: SystemTime,
    pub reason: RevocationReason,
    pub invalidity_date: Option<SystemTime>,
}

/// fr fr Revocation reasons
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevocationReason {
    Unspecified,
    KeyCompromise,
    CaCompromise,
    AffiliationChanged,
    Superseded,
    CessationOfOperation,
    CertificateHold,
    RemoveFromCrl,
    PrivilegeWithdrawn,
    AaCompromise,
}

impl Default for CaConfiguration {
    fn default() -> Self {
        Self {
            name: "Default CA".to_string(),
            key_size: 2048,
            signature_algorithm: SignatureAlgorithm::RsaWithSha256,
            validity_days: 3650, // 10 years for CA
            country: Some("US".to_string()),
            state: None,
            locality: None,
            organization: "Default Organization".to_string(),
            organizational_unit: None,
            email: None,
            enable_crl: true,
            crl_distribution_points: Vec::new(),
            ocsp_responder_url: None,
        }
    }
}

impl Default for CaPolicy {
    fn default() -> Self {
        Self {
            max_validity_days: 365,
            require_san: false,
            allowed_key_sizes: vec![2048, 3072, 4096],
            allowed_signature_algorithms: vec![
                SignatureAlgorithm::RsaWithSha256,
                SignatureAlgorithm::RsaWithSha384,
                SignatureAlgorithm::RsaWithSha512,
                SignatureAlgorithm::EcdsaWithSha256,
                SignatureAlgorithm::EcdsaWithSha384,
            ],
            path_length_constraint: None,
            name_constraints: Vec::new(),
            policy_oids: Vec::new(),
        }
    }
}

impl CertificateAuthority {
    /// slay Create a new Certificate Authority
    pub fn new(config: CaConfiguration) -> PkiResult<Self> {
        // Generate CA key pair
        let key_pair = rsa_generate_keypair(config.key_size, vec![])
            .map_err(|e| PkiError::CaOperationFailed(format!("Key generation failed: {}", e)))?;
        
        // Create CA certificate
        let mut cert_builder = CertificateBuilder::new();
        cert_builder.set_subject_common_name(&config.name);
        cert_builder.set_subject_organization(&config.organization);
        
        if let Some(ref country) = config.country {
            cert_builder.subject.country = Some(country.clone());
        }
        if let Some(ref state) = config.state {
            cert_builder.subject.state = Some(state.clone());
        }
        if let Some(ref locality) = config.locality {
            cert_builder.subject.locality = Some(locality.clone());
        }
        if let Some(ref ou) = config.organizational_unit {
            cert_builder.subject.organizational_unit = Some(ou.clone());
        }
        if let Some(ref email) = config.email {
            cert_builder.subject.email = Some(email.clone());
        }
        
        cert_builder.set_validity_days(config.validity_days);
        cert_builder.set_key_size(config.key_size);
        cert_builder.set_ca(true, None); // Root CA has no path length constraint
        
        let certificate = cert_builder.build_self_signed()?;
        
        let metadata = CaMetadata {
            ca_id: generate_ca_id(&config.name),
            created_at: SystemTime::now(),
            last_crl_update: None,
            next_crl_update: None,
            certificates_issued: 0,
            certificates_revoked: 0,
            status: CaStatus::Active,
            version: "1.0.0".to_string(),
        };
        
        Ok(Self {
            config,
            certificate,
            private_key: key_pair.private_key,
            metadata,
            policy: CaPolicy::default(),
            issued_certificates: HashMap::new(),
            revoked_certificates: HashMap::new(),
        })
    }
    
    /// slay Create CA from existing certificate and key
    pub fn from_certificate(cert: Certificate, private_key: Vec<u8>) -> PkiResult<Self> {
        if !cert.is_ca() {
            return Err(PkiError::CaConfigurationInvalid(
                "Certificate is not a CA certificate".to_string()
            ));
        }
        
        let config = CaConfiguration {
            name: cert.subject.common_name.clone().unwrap_or_default(),
            key_size: cert.public_key_info.key_size,
            signature_algorithm: cert.signature_algorithm.clone(),
            validity_days: 365, // Default for issued certificates
            country: cert.subject.country.clone(),
            state: cert.subject.state.clone(),
            locality: cert.subject.locality.clone(),
            organization: cert.subject.organization.clone().unwrap_or_default(),
            organizational_unit: cert.subject.organizational_unit.clone(),
            email: cert.subject.email.clone(),
            enable_crl: true,
            crl_distribution_points: Vec::new(),
            ocsp_responder_url: None,
        };
        
        let metadata = CaMetadata {
            ca_id: generate_ca_id(&config.name),
            created_at: cert.validity.not_before,
            last_crl_update: None,
            next_crl_update: None,
            certificates_issued: 0,
            certificates_revoked: 0,
            status: if cert.is_valid_now() { CaStatus::Active } else { CaStatus::Expired },
            version: "1.0.0".to_string(),
        };
        
        Ok(Self {
            config,
            certificate: cert,
            private_key,
            metadata,
            policy: CaPolicy::default(),
            issued_certificates: HashMap::new(),
            revoked_certificates: HashMap::new(),
        })
    }
    
    /// slay Create root CA
    pub fn create_root_ca(&self, subject: &str, key_size: usize) -> PkiResult<(Certificate, Vec<u8>)> {
        let mut config = self.config.clone();
        config.name = subject.to_string();
        config.key_size = key_size;
        
        let ca = Self::new(config)?;
        Ok((ca.certificate, ca.private_key))
    }
    
    /// slay Issue a certificate using a template
    pub fn issue_certificate(&self, template: &CertificateTemplate) -> PkiResult<Certificate> {
        // Validate request against CA policy
        self.validate_certificate_request(template)?;
        
        // Build certificate from template
        let mut cert_builder = CertificateBuilder::new();
        
        // Set subject from template
        cert_builder.set_subject_common_name(&template.subject.common_name);
        if let Some(ref org) = template.subject.organization {
            cert_builder.set_subject_organization(org);
        }
        
        // Set validity (constrained by CA policy)
        let validity_days = std::cmp::min(template.validity_days, self.policy.max_validity_days);
        cert_builder.set_validity_days(validity_days);
        
        // Set key size
        cert_builder.set_key_size(template.key_size);
        
        // Add subject alternative names
        for san in &template.subject_alt_names {
            cert_builder.add_dns_san(san);
        }
        
        // Set extensions from template
        if let Some(ref basic_constraints) = template.basic_constraints {
            cert_builder.set_ca(basic_constraints.ca, basic_constraints.path_length);
        }
        
        // Build and sign certificate
        let mut certificate = cert_builder.build_self_signed()?;
        
        // Override issuer with CA certificate subject
        certificate.issuer = self.certificate.subject.clone().into();
        
        // Sign certificate with CA private key
        self.sign_certificate(&mut certificate)?;
        
        Ok(certificate)
    }
    
    /// slay Revoke a certificate
    pub fn revoke_certificate(
        &mut self, 
        serial_number: &str, 
        reason: RevocationReason
    ) -> PkiResult<()> {
        let revocation_entry = RevocationEntry {
            serial_number: serial_number.to_string(),
            revocation_time: SystemTime::now(),
            reason,
            invalidity_date: None,
        };
        
        self.revoked_certificates.insert(serial_number.to_string(), revocation_entry);
        self.metadata.certificates_revoked += 1;
        
        Ok(())
    }
    
    /// slay Generate Certificate Revocation List (CRL)
    pub fn generate_crl(&mut self) -> PkiResult<Vec<u8>> {
        let now = SystemTime::now();
        let next_update = now + Duration::from_secs(7 * 24 * 60 * 60); // 7 days
        
        // Create CRL structure (mock implementation)
        let mut crl_data = Vec::new();
        crl_data.extend_from_slice(b"CRL_HEADER");
        
        // Add revoked certificates
        for (serial, entry) in &self.revoked_certificates {
            crl_data.extend_from_slice(serial.as_bytes());
            crl_data.extend_from_slice(&entry.revocation_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_be_bytes());
        }
        
        self.metadata.last_crl_update = Some(now);
        self.metadata.next_crl_update = Some(next_update);
        
        Ok(crl_data)
    }
    
    /// slay Check if certificate is revoked
    pub fn is_revoked(&self, serial_number: &str) -> bool {
        self.revoked_certificates.contains_key(serial_number)
    }
    
    /// slay Get CA certificate chain
    pub fn get_certificate_chain(&self) -> Vec<Certificate> {
        vec![self.certificate.clone()]
    }
    
    /// slay Validate certificate request against CA policy
    fn validate_certificate_request(&self, template: &CertificateTemplate) -> PkiResult<()> {
        // Check validity period
        if template.validity_days > self.policy.max_validity_days {
            return Err(PkiError::CaOperationFailed(
                format!("Requested validity period {} days exceeds maximum allowed {} days",
                    template.validity_days, self.policy.max_validity_days)
            ));
        }
        
        // Check key size
        if !self.policy.allowed_key_sizes.contains(&template.key_size) {
            return Err(PkiError::CaOperationFailed(
                format!("Key size {} not allowed by CA policy", template.key_size)
            ));
        }
        
        // Check if SAN is required
        if self.policy.require_san && template.subject_alt_names.is_empty() {
            return Err(PkiError::CaOperationFailed(
                "Subject Alternative Name is required by CA policy".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// slay Sign a certificate with the CA's private key
    fn sign_certificate(&self, certificate: &mut Certificate) -> PkiResult<()> {
        use crate::stdlib::packages::crypto_asymmetric::{rsa_sign, ecdsa_sign, RsaPadding, EccCurve, EccHashAlgorithm};
        use crate::stdlib::packages::crypto_hash_advanced::sha256_hash;
        
        // Prepare certificate data for signing (TBSCertificate)
        let tbs_cert_data = self.prepare_certificate_tbs_data(certificate)?;
        
        // Sign the certificate based on the signature algorithm
        let signature = match certificate.signature_algorithm {
            SignatureAlgorithm::RsaWithSha256 => {
                // Parse RSA private key from stored format
                let rsa_key_pair = self.parse_rsa_private_key()?;
                rsa_sign(&tbs_cert_data, &rsa_key_pair.private_key, RsaPadding::Pkcs1v15)
                    .map_err(|e| PkiError::CaOperationFailed(format!("RSA signing failed: {}", e)))?
            },
            SignatureAlgorithm::RsaWithSha384 => {
                let rsa_key_pair = self.parse_rsa_private_key()?;
                rsa_sign(&tbs_cert_data, &rsa_key_pair.private_key, RsaPadding::PssSha384)
                    .map_err(|e| PkiError::CaOperationFailed(format!("RSA signing failed: {}", e)))?
            },
            SignatureAlgorithm::RsaWithSha512 => {
                let rsa_key_pair = self.parse_rsa_private_key()?;
                rsa_sign(&tbs_cert_data, &rsa_key_pair.private_key, RsaPadding::PssSha512)
                    .map_err(|e| PkiError::CaOperationFailed(format!("RSA signing failed: {}", e)))?
            },
            SignatureAlgorithm::EcdsaWithSha256 => {
                let ecc_key_pair = self.parse_ecc_private_key(EccCurve::P256)?;
                ecdsa_sign(&tbs_cert_data, &ecc_key_pair.private_key_bytes, EccCurve::P256, EccHashAlgorithm::Sha256)
                    .map_err(|e| PkiError::CaOperationFailed(format!("ECDSA signing failed: {}", e)))?
            },
            SignatureAlgorithm::EcdsaWithSha384 => {
                let ecc_key_pair = self.parse_ecc_private_key(EccCurve::P384)?;
                ecdsa_sign(&tbs_cert_data, &ecc_key_pair.private_key_bytes, EccCurve::P384, EccHashAlgorithm::Sha384)
                    .map_err(|e| PkiError::CaOperationFailed(format!("ECDSA signing failed: {}", e)))?
            },
            SignatureAlgorithm::EcdsaWithSha512 => {
                let ecc_key_pair = self.parse_ecc_private_key(EccCurve::P521)?;
                ecdsa_sign(&tbs_cert_data, &ecc_key_pair.private_key_bytes, EccCurve::P521, EccHashAlgorithm::Sha512)
                    .map_err(|e| PkiError::CaOperationFailed(format!("ECDSA signing failed: {}", e)))?
            },
            _ => return Err(PkiError::UnsupportedAlgorithm(
                format!("Signature algorithm {:?} not supported for certificate signing", certificate.signature_algorithm)
            )),
        };
        
        // Store signature in certificate
        certificate.signature = signature;
        
        // Update raw DER data with signed certificate
        certificate.raw = self.encode_signed_certificate(certificate)?;
        
        Ok(())
    }
    
    /// slay Prepare certificate TBSCertificate data for signing
    fn prepare_certificate_tbs_data(&self, certificate: &Certificate) -> PkiResult<Vec<u8>> {
        // In a real implementation, this would create proper ASN.1 DER-encoded TBSCertificate
        // For now, we'll create a structured representation of the certificate data
        let mut tbs_data = Vec::new();
        
        // Version
        tbs_data.extend_from_slice(&certificate.version.to_be_bytes());
        
        // Serial number
        tbs_data.extend_from_slice(&certificate.serial_number.bytes);
        
        // Signature algorithm identifier
        tbs_data.extend_from_slice(format!("{:?}", certificate.signature_algorithm).as_bytes());
        
        // Issuer distinguished name
        tbs_data.extend_from_slice(certificate.issuer.distinguished_name.as_bytes());
        
        // Validity period
        let not_before_secs = certificate.validity.not_before.duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default().as_secs();
        let not_after_secs = certificate.validity.not_after.duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default().as_secs();
        tbs_data.extend_from_slice(&not_before_secs.to_be_bytes());
        tbs_data.extend_from_slice(&not_after_secs.to_be_bytes());
        
        // Subject distinguished name
        tbs_data.extend_from_slice(certificate.subject.distinguished_name.as_bytes());
        
        // Subject public key info
        tbs_data.extend_from_slice(format!("{:?}", certificate.public_key_info.algorithm).as_bytes());
        tbs_data.extend_from_slice(&certificate.public_key_info.public_key);
        
        // Extensions (basic constraints, key usage, etc.)
        if let Some(ref basic_constraints) = certificate.extensions.basic_constraints {
            tbs_data.extend_from_slice(&[basic_constraints.ca as u8]);
            if let Some(path_length) = basic_constraints.path_length {
                tbs_data.extend_from_slice(&path_length.to_be_bytes());
            }
        }
        
        // Subject alternative names
        for san in &certificate.extensions.subject_alt_names {
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
    
    /// slay Parse RSA private key from stored format
    fn parse_rsa_private_key(&self) -> PkiResult<RsaKeyPair> {
        // For now, assume the private key is already in the correct format
        // In a real implementation, this would parse PEM/DER encoded RSA keys
        Ok(RsaKeyPair {
            public_key: Vec::new(), // Not needed for signing
            private_key: self.private_key.clone(),
        })
    }
    
    /// slay Parse ECC private key from stored format
    fn parse_ecc_private_key(&self, curve: EccCurve) -> PkiResult<EccKeyPair> {
        use crate::stdlib::packages::crypto_asymmetric::EccKeyPair;
        
        // For now, assume the private key is already in the correct format
        // In a real implementation, this would parse PEM/DER encoded ECC keys
        Ok(EccKeyPair {
            public_key_bytes: Vec::new(), // Not needed for signing
            private_key_bytes: self.private_key.clone(),
            curve,
        })
    }
    
    /// slay Encode signed certificate to DER format
    fn encode_signed_certificate(&self, certificate: &Certificate) -> PkiResult<Vec<u8>> {
        // In a real implementation, this would create proper ASN.1 DER encoding
        // For now, create a mock signed certificate structure
        let mut der = Vec::new();
        
        // Certificate header
        der.extend_from_slice(b"SIGNED_CERTIFICATE_DER");
        
        // TBS Certificate data
        let tbs_data = self.prepare_certificate_tbs_data(certificate)?;
        der.extend_from_slice(&(tbs_data.len() as u32).to_be_bytes());
        der.extend_from_slice(&tbs_data);
        
        // Signature algorithm
        der.extend_from_slice(format!("{:?}", certificate.signature_algorithm).as_bytes());
        
        // Signature value
        der.extend_from_slice(&(certificate.signature.len() as u32).to_be_bytes());
        der.extend_from_slice(&certificate.signature);
        
        Ok(der)
    }
}

impl CaManager {
    /// slay Create a new CA manager
    pub fn new() -> Self {
        Self {
            cas: HashMap::new(),
            hierarchies: HashMap::new(),
            policies: HashMap::new(),
            profiles: HashMap::new(),
        }
    }
    
    /// slay Add a CA to the manager
    pub fn add_ca(&mut self, name: String, ca: CertificateAuthority) {
        self.cas.insert(name, ca);
    }
    
    /// slay Get a CA by name
    pub fn get_ca(&self, name: &str) -> Option<&CertificateAuthority> {
        self.cas.get(name)
    }
    
    /// slay Get a mutable CA by name
    pub fn get_ca_mut(&mut self, name: &str) -> Option<&mut CertificateAuthority> {
        self.cas.get_mut(name)
    }
    
    /// slay List all CAs
    pub fn list_cas(&self) -> Vec<String> {
        self.cas.keys().cloned().collect()
    }
    
    /// slay Add a CA profile
    pub fn add_profile(&mut self, name: String, profile: CaProfile) {
        self.profiles.insert(name, profile);
    }
    
    /// slay Get a CA profile
    pub fn get_profile(&self, name: &str) -> Option<&CaProfile> {
        self.profiles.get(name)
    }
}

/// fr fr High-level CA operations

/// slay Create a root CA
pub fn create_root_ca(name: &str, organization: &str, key_size: Option<usize>) -> PkiResult<CertificateAuthority> {
    let mut config = CaConfiguration::default();
    config.name = name.to_string();
    config.organization = organization.to_string();
    config.key_size = key_size.unwrap_or(2048);
    
    CertificateAuthority::new(config)
}

/// slay Create an intermediate CA
pub fn create_intermediate_ca(
    parent_ca: &mut CertificateAuthority,
    name: &str,
    organization: &str
) -> PkiResult<CertificateAuthority> {
    let mut config = CaConfiguration::default();
    config.name = name.to_string();
    config.organization = organization.to_string();
    config.validity_days = 1825; // 5 years for intermediate CA
    
    let mut ca = CertificateAuthority::new(config)?;
    
    // Set as intermediate CA (path length constraint)
    ca.certificate.extensions.basic_constraints = Some(BasicConstraints {
        ca: true,
        path_length: Some(0), // Can only issue end-entity certificates
    });
    
    // Override issuer with parent CA
    ca.certificate.issuer = parent_ca.certificate.subject.clone().into();
    
    Ok(ca)
}

/// slay Create a subordinate CA
pub fn create_subordinate_ca(
    parent_ca: &mut CertificateAuthority,
    name: &str,
    organization: &str
) -> PkiResult<CertificateAuthority> {
    create_intermediate_ca(parent_ca, name, organization)
}

/// slay Sign a certificate with CA
pub fn ca_sign_certificate(
    ca: &mut CertificateAuthority,
    template: &CertificateTemplate
) -> PkiResult<Certificate> {
    ca.issue_certificate(template)
}

/// slay Revoke a certificate with CA
pub fn ca_revoke_certificate(
    ca: &mut CertificateAuthority,
    serial_number: &str,
    reason: RevocationReason
) -> PkiResult<()> {
    ca.revoke_certificate(serial_number, reason)
}

/// slay Generate CRL with CA
pub fn ca_generate_crl(ca: &mut CertificateAuthority) -> PkiResult<Vec<u8>> {
    ca.generate_crl()
}

/// fr fr Utility functions

fn generate_ca_id(name: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    SystemTime::now().hash(&mut hasher);
    
    format!("ca_{:x}", hasher.finish())
}

/// fr fr Certificate template structure (imported from templates module)
#[derive(Debug, Clone)]
pub struct CertificateTemplate {
    pub subject: TemplateSubject,
    pub validity_days: u32,
    pub key_size: usize,
    pub subject_alt_names: Vec<String>,
    pub key_usage: Option<KeyUsage>,
    pub extended_key_usage: Option<ExtendedKeyUsage>,
    pub basic_constraints: Option<BasicConstraints>,
}

#[derive(Debug, Clone)]
pub struct TemplateSubject {
    pub common_name: String,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}

impl Default for CaManager {
    fn default() -> Self {
        Self::new()
    }
}
