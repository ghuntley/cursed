//! Certificate Authority Implementation
//! 
//! Comprehensive Certificate Authority functionality for issuing, managing, and revoking certificates.

use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use crate::stdlib::packages::crypto_pki::types::*;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};
use crate::stdlib::packages::crypto_asymmetric::*;

/// Certificate Authority with full PKI capabilities
#[derive(Debug, Clone)]
pub struct CertificateAuthority {
    /// CA configuration
    pub config: CaConfig,
    /// CA certificate
    pub ca_certificate: X509Certificate,
    /// CA private key (encrypted in real implementation)
    pub ca_private_key: Vec<u8>,
    /// Issued certificates registry
    pub issued_certificates: HashMap<SerialNumber, IssuedCertificate>,
    /// Certificate templates
    pub certificate_templates: HashMap<String, CertificateTemplate>,
    /// Revocation list
    pub revocation_list: CertificateRevocationList,
    /// Next serial number
    pub next_serial_number: u64,
    /// CA statistics
    pub statistics: CaStatistics,
}

/// Certificate Authority configuration
#[derive(Debug, Clone)]
pub struct CaConfig {
    /// CA name/identifier
    pub name: String,
    /// CA distinguished name
    pub distinguished_name: DistinguishedName,
    /// Default certificate validity period
    pub default_validity: Duration,
    /// Maximum certificate validity period
    pub max_validity: Duration,
    /// Signature algorithm for issued certificates
    pub signature_algorithm: SignatureAlgorithm,
    /// Key usage for CA certificate
    pub ca_key_usage: KeyUsage,
    /// Basic constraints for CA
    pub basic_constraints: BasicConstraints,
    /// Supported key algorithms
    pub supported_key_algorithms: Vec<PublicKeyAlgorithm>,
    /// Certificate policies
    pub certificate_policies: Vec<PolicyInformation>,
    /// CRL distribution points
    pub crl_distribution_points: Vec<String>,
    /// OCSP responder URLs
    pub ocsp_responders: Vec<String>,
    /// Authority information access
    pub authority_info_access: Vec<AccessDescription>,
    /// Certificate extensions to include
    pub default_extensions: Vec<DefaultExtension>,
}

/// Basic constraints configuration
#[derive(Debug, Clone)]
pub struct BasicConstraints {
    /// Whether this is a CA certificate
    pub is_ca: bool,
    /// Path length constraint (None for unlimited)
    pub path_length_constraint: Option<u32>,
    /// Whether basic constraints are critical
    pub critical: bool,
}

/// Default extension configuration
#[derive(Debug, Clone)]
pub struct DefaultExtension {
    /// Extension OID
    pub oid: String,
    /// Extension value
    pub value: Vec<u8>,
    /// Whether extension is critical
    pub critical: bool,
}

/// Issued certificate record
#[derive(Debug, Clone)]
pub struct IssuedCertificate {
    /// Certificate
    pub certificate: X509Certificate,
    /// Issue timestamp
    pub issued_at: SystemTime,
    /// Issuing CA
    pub issuer_name: String,
    /// Certificate status
    pub status: CertificateStatus,
    /// Subject distinguished name
    pub subject_dn: DistinguishedName,
    /// Certificate purpose
    pub purpose: CertificatePurpose,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Certificate status
#[derive(Debug, Clone, PartialEq)]
pub enum CertificateStatus {
    /// Certificate is valid and active
    Valid,
    /// Certificate has expired
    Expired,
    /// Certificate has been revoked
    Revoked {
        /// Revocation timestamp
        revoked_at: SystemTime,
        /// Revocation reason
        reason: RevocationReason,
    },
    /// Certificate is suspended (can be reactivated)
    Suspended {
        /// Suspension timestamp
        suspended_at: SystemTime,
        /// Suspension reason
        reason: String,
    },
}

/// Certificate purpose/usage
#[derive(Debug, Clone, PartialEq)]
pub enum CertificatePurpose {
    /// TLS server authentication
    ServerAuth,
    /// TLS client authentication
    ClientAuth,
    /// Code signing
    CodeSigning,
    /// Email protection
    EmailProtection,
    /// Time stamping
    TimeStamping,
    /// OCSP signing
    OcspSigning,
    /// General purpose
    GeneralPurpose,
    /// Custom purpose
    Custom(String),
}

/// Certificate template for standardized issuance
#[derive(Debug, Clone)]
pub struct CertificateTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Key usage settings
    pub key_usage: KeyUsage,
    /// Extended key usage
    pub extended_key_usage: ExtendedKeyUsage,
    /// Certificate validity period
    pub validity_period: Duration,
    /// Allowed key algorithms
    pub allowed_key_algorithms: Vec<PublicKeyAlgorithm>,
    /// Required subject fields
    pub required_subject_fields: Vec<String>,
    /// Additional extensions
    pub extensions: Vec<DefaultExtension>,
    /// Subject alternative name requirements
    pub san_requirements: SanRequirements,
}

/// Subject Alternative Name requirements
#[derive(Debug, Clone)]
pub struct SanRequirements {
    /// Require DNS names
    pub require_dns_names: bool,
    /// Require email addresses
    pub require_email_addresses: bool,
    /// Require IP addresses
    pub require_ip_addresses: bool,
    /// Allowed DNS patterns
    pub allowed_dns_patterns: Vec<String>,
    /// Allowed email domains
    pub allowed_email_domains: Vec<String>,
}

/// Certificate issuance request
#[derive(Debug, Clone)]
pub struct CertificateIssuanceRequest {
    /// Certificate signing request
    pub csr: CertificateSigningRequest,
    /// Requested validity period
    pub validity_period: Option<Duration>,
    /// Certificate template to use
    pub template_name: Option<String>,
    /// Additional subject alternative names
    pub additional_sans: Vec<GeneralName>,
    /// Custom extensions
    pub custom_extensions: Vec<X509Extension>,
    /// Certificate purpose
    pub purpose: CertificatePurpose,
    /// Requesting entity information
    pub requestor_info: RequestorInfo,
}

/// Information about the entity requesting the certificate
#[derive(Debug, Clone)]
pub struct RequestorInfo {
    /// Requestor name
    pub name: String,
    /// Contact email
    pub email: Option<String>,
    /// Organization
    pub organization: Option<String>,
    /// Request timestamp
    pub requested_at: SystemTime,
    /// Request metadata
    pub metadata: HashMap<String, String>,
}

/// Certificate revocation request
#[derive(Debug, Clone)]
pub struct CertificateRevocationRequest {
    /// Certificate serial number to revoke
    pub serial_number: SerialNumber,
    /// Revocation reason
    pub reason: RevocationReason,
    /// Revocation timestamp
    pub revocation_time: Option<SystemTime>,
    /// Requesting entity
    pub requestor: String,
    /// Additional context
    pub context: String,
}

/// CA operational statistics
#[derive(Debug, Clone, Default)]
pub struct CaStatistics {
    /// Total certificates issued
    pub total_issued: u64,
    /// Currently valid certificates
    pub currently_valid: u64,
    /// Expired certificates
    pub expired: u64,
    /// Revoked certificates
    pub revoked: u64,
    /// Suspended certificates
    pub suspended: u64,
    /// Certificates by purpose
    pub by_purpose: HashMap<String, u64>,
    /// Certificates by key algorithm
    pub by_key_algorithm: HashMap<String, u64>,
    /// Last CRL generation time
    pub last_crl_generated: Option<SystemTime>,
    /// CA creation time
    pub ca_created_at: SystemTime,
}

impl CertificateAuthority {
    /// Create a new Certificate Authority
    pub fn new(config: CaConfig, ca_cert: X509Certificate, ca_key: Vec<u8>) -> Self {
        Self {
            config,
            ca_certificate: ca_cert,
            ca_private_key: ca_key,
            issued_certificates: HashMap::new(),
            certificate_templates: HashMap::new(),
            revocation_list: CertificateRevocationList::new(),
            next_serial_number: 1,
            statistics: CaStatistics {
                ca_created_at: SystemTime::now(),
                ..Default::default()
            },
        }
    }
    
    /// Initialize CA with default templates
    pub fn initialize_with_defaults(&mut self) -> PkiResult<()> {
        // Add default certificate templates
        self.add_template(Self::create_server_auth_template())?;
        self.add_template(Self::create_client_auth_template())?;
        self.add_template(Self::create_code_signing_template())?;
        self.add_template(Self::create_email_protection_template())?;
        
        Ok(())
    }
    
    /// Issue a certificate based on a CSR
    pub fn issue_certificate(&mut self, request: CertificateIssuanceRequest) -> PkiResult<IssuedCertificate> {
        // Validate the CSR
        self.validate_csr(&request.csr)?;
        
        // Get certificate template
        let template = if let Some(template_name) = &request.template_name {
            self.certificate_templates.get(template_name)
                .ok_or_else(|| PkiError::ca_error(
                    format!("Template not found: {}", template_name),
                    "issue_certificate"
                ))?
                .clone()
        } else {
            self.get_default_template_for_purpose(&request.purpose)?
        };
        
        // Validate request against template
        self.validate_request_against_template(&request, &template)?;
        
        // Determine validity period
        let validity_period = request.validity_period
            .unwrap_or(template.validity_period)
            .min(self.config.max_validity);
        
        // Generate certificate
        let certificate = self.generate_certificate(
            &request.csr,
            &template,
            validity_period,
            &request.additional_sans,
            &request.custom_extensions,
        )?;
        
        // Create issued certificate record
        let issued_cert = IssuedCertificate {
            certificate: certificate.clone(),
            issued_at: SystemTime::now(),
            issuer_name: self.config.name.clone(),
            status: CertificateStatus::Valid,
            subject_dn: request.csr.subject.clone(),
            purpose: request.purpose.clone(),
            metadata: request.requestor_info.metadata.clone(),
        };
        
        // Store in registry
        self.issued_certificates.insert(
            certificate.serial_number.clone(),
            issued_cert.clone()
        );
        
        // Update statistics
        self.update_statistics_for_issuance(&request.purpose, &certificate);
        
        Ok(issued_cert)
    }
    
    /// Revoke a certificate
    pub fn revoke_certificate(&mut self, request: CertificateRevocationRequest) -> PkiResult<()> {
        // Find the certificate
        let mut issued_cert = self.issued_certificates.get_mut(&request.serial_number)
            .ok_or_else(|| PkiError::ca_error(
                format!("Certificate not found: {}", request.serial_number.to_hex_string()),
                "revoke_certificate"
            ))?;
        
        // Check if already revoked
        if matches!(issued_cert.status, CertificateStatus::Revoked { .. }) {
            return Err(PkiError::ca_error(
                "Certificate already revoked",
                "revoke_certificate"
            ));
        }
        
        // Update certificate status
        let revocation_time = request.revocation_time.unwrap_or_else(SystemTime::now);
        issued_cert.status = CertificateStatus::Revoked {
            revoked_at: revocation_time,
            reason: request.reason,
        };
        
        // Add to CRL
        self.revocation_list.revoked_certificates.push(RevokedCertificate {
            serial_number: request.serial_number.clone(),
            revocation_date: revocation_time,
            reason: Some(request.reason),
            extensions: Vec::new(),
        });
        
        // Update statistics
        self.statistics.revoked += 1;
        self.statistics.currently_valid = self.statistics.currently_valid.saturating_sub(1);
        
        Ok(())
    }
    
    /// Generate a new Certificate Revocation List (CRL)
    pub fn generate_crl(&mut self) -> PkiResult<CertificateRevocationList> {
        let now = SystemTime::now();
        
        // Collect all revoked certificates
        let mut revoked_certs = Vec::new();
        for (serial_number, issued_cert) in &self.issued_certificates {
            if let CertificateStatus::Revoked { revoked_at, reason } = &issued_cert.status {
                revoked_certs.push(RevokedCertificate {
                    serial_number: serial_number.clone(),
                    revocation_date: *revoked_at,
                    reason: Some(*reason),
                    extensions: Vec::new(),
                });
            }
        }
        
        // Sort by serial number
        revoked_certs.sort_by(|a, b| {
            a.serial_number.to_hex_string().cmp(&b.serial_number.to_hex_string())
        });
        
        // Create CRL
        let crl = CertificateRevocationList {
            version: Some(2), // Version 2 (v2)
            signature_algorithm: self.config.signature_algorithm.clone(),
            issuer: self.config.distinguished_name.clone(),
            this_update: now,
            next_update: Some(now + Duration::from_secs(7 * 24 * 3600)), // 7 days
            revoked_certificates: revoked_certs,
            extensions: self.generate_crl_extensions(),
            raw_data: Vec::new(), // Would be filled by DER encoder
        };
        
        self.statistics.last_crl_generated = Some(now);
        self.revocation_list = crl.clone();
        
        Ok(crl)
    }
    
    /// Validate a Certificate Signing Request
    fn validate_csr(&self, csr: &CertificateSigningRequest) -> PkiResult<()> {
        // Validate CSR structure
        if csr.subject.common_name.is_none() {
            return Err(PkiError::ca_error(
                "CSR must contain a Common Name",
                "validate_csr"
            ));
        }
        
        // Validate public key algorithm
        if !self.config.supported_key_algorithms.contains(&csr.subject_public_key_info.algorithm) {
            return Err(PkiError::ca_error(
                format!("Unsupported key algorithm: {:?}", csr.subject_public_key_info.algorithm),
                "validate_csr"
            ));
        }
        
        // Validate signature (simplified)
        self.validate_csr_signature(csr)?;
        
        Ok(())
    }
    
    /// Validate CSR signature
    fn validate_csr_signature(&self, _csr: &CertificateSigningRequest) -> PkiResult<()> {
        // TODO: Implement actual CSR signature validation
        // This would verify that the CSR is signed by the private key
        // corresponding to the public key in the CSR
        Ok(())
    }
    
    /// Validate request against template
    fn validate_request_against_template(
        &self,
        request: &CertificateIssuanceRequest,
        template: &CertificateTemplate,
    ) -> PkiResult<()> {
        // Check key algorithm
        if !template.allowed_key_algorithms.contains(&request.csr.subject_public_key_info.algorithm) {
            return Err(PkiError::ca_error(
                format!("Key algorithm not allowed for template: {:?}", 
                       request.csr.subject_public_key_info.algorithm),
                "validate_request_against_template"
            ));
        }
        
        // Check required subject fields
        for required_field in &template.required_subject_fields {
            match required_field.as_str() {
                "CN" => {
                    if request.csr.subject.common_name.is_none() {
                        return Err(PkiError::ca_error(
                            "Common Name is required",
                            "validate_request_against_template"
                        ));
                    }
                }
                "O" => {
                    if request.csr.subject.organization.is_none() {
                        return Err(PkiError::ca_error(
                            "Organization is required",
                            "validate_request_against_template"
                        ));
                    }
                }
                "C" => {
                    if request.csr.subject.country.is_none() {
                        return Err(PkiError::ca_error(
                            "Country is required",
                            "validate_request_against_template"
                        ));
                    }
                }
                _ => {}
            }
        }
        
        // Validate SAN requirements
        self.validate_san_requirements(&request.additional_sans, &template.san_requirements)?;
        
        Ok(())
    }
    
    /// Validate Subject Alternative Name requirements
    fn validate_san_requirements(
        &self,
        sans: &[GeneralName],
        requirements: &SanRequirements,
    ) -> PkiResult<()> {
        let mut has_dns = false;
        let mut has_email = false;
        let mut has_ip = false;
        
        for san in sans {
            match san {
                GeneralName::DnsName(dns) => {
                    has_dns = true;
                    if !requirements.allowed_dns_patterns.is_empty() {
                        let allowed = requirements.allowed_dns_patterns.iter()
                            .any(|pattern| self.matches_pattern(dns, pattern));
                        if !allowed {
                            return Err(PkiError::ca_error(
                                format!("DNS name not allowed: {}", dns),
                                "validate_san_requirements"
                            ));
                        }
                    }
                }
                GeneralName::Rfc822Name(email) => {
                    has_email = true;
                    if !requirements.allowed_email_domains.is_empty() {
                        let domain = email.split('@').nth(1).unwrap_or("");
                        if !requirements.allowed_email_domains.contains(&domain.to_string()) {
                            return Err(PkiError::ca_error(
                                format!("Email domain not allowed: {}", domain),
                                "validate_san_requirements"
                            ));
                        }
                    }
                }
                GeneralName::IpAddress(_) => {
                    has_ip = true;
                }
                _ => {}
            }
        }
        
        if requirements.require_dns_names && !has_dns {
            return Err(PkiError::ca_error(
                "DNS names are required",
                "validate_san_requirements"
            ));
        }
        
        if requirements.require_email_addresses && !has_email {
            return Err(PkiError::ca_error(
                "Email addresses are required",
                "validate_san_requirements"
            ));
        }
        
        if requirements.require_ip_addresses && !has_ip {
            return Err(PkiError::ca_error(
                "IP addresses are required",
                "validate_san_requirements"
            ));
        }
        
        Ok(())
    }
    
    /// Simple pattern matching for DNS names
    fn matches_pattern(&self, dns_name: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        
        if pattern.starts_with("*.") {
            let domain = &pattern[2..];
            dns_name.ends_with(domain) || dns_name == &domain[1..]
        } else {
            dns_name == pattern
        }
    }
    
    /// Generate a certificate from a CSR and template
    fn generate_certificate(
        &mut self,
        csr: &CertificateSigningRequest,
        template: &CertificateTemplate,
        validity_period: Duration,
        additional_sans: &[GeneralName],
        custom_extensions: &[X509Extension],
    ) -> PkiResult<X509Certificate> {
        let now = SystemTime::now();
        let serial_number = SerialNumber::from_big_int(self.next_serial_number);
        self.next_serial_number += 1;
        
        // Build extensions
        let mut extensions = Vec::new();
        
        // Key Usage extension
        extensions.push(X509Extension {
            oid: "2.5.29.15".to_string(), // Key Usage
            critical: true,
            value: self.encode_key_usage(&template.key_usage),
            parsed_data: Some(ExtensionData::KeyUsage(template.key_usage.clone())),
        });
        
        // Extended Key Usage extension
        if template.extended_key_usage.server_auth || 
           template.extended_key_usage.client_auth ||
           template.extended_key_usage.code_signing ||
           template.extended_key_usage.email_protection {
            extensions.push(X509Extension {
                oid: "2.5.29.37".to_string(), // Extended Key Usage
                critical: false,
                value: self.encode_extended_key_usage(&template.extended_key_usage),
                parsed_data: Some(ExtensionData::ExtendedKeyUsage(template.extended_key_usage.clone())),
            });
        }
        
        // Subject Alternative Names
        if !additional_sans.is_empty() {
            extensions.push(X509Extension {
                oid: "2.5.29.17".to_string(), // Subject Alternative Name
                critical: false,
                value: self.encode_subject_alternative_names(additional_sans),
                parsed_data: Some(ExtensionData::SubjectAlternativeName(additional_sans.to_vec())),
            });
        }
        
        // Authority Key Identifier
        extensions.push(X509Extension {
            oid: "2.5.29.35".to_string(), // Authority Key Identifier
            critical: false,
            value: self.encode_authority_key_identifier(),
            parsed_data: Some(ExtensionData::AuthorityKeyIdentifier {
                key_identifier: self.ca_certificate.fingerprint.clone(),
                authority_cert_issuer: None,
                authority_cert_serial_number: Some(self.ca_certificate.serial_number.clone()),
            }),
        });
        
        // Subject Key Identifier
        let subject_key_id = self.calculate_subject_key_identifier(&csr.subject_public_key_info.public_key);
        extensions.push(X509Extension {
            oid: "2.5.29.14".to_string(), // Subject Key Identifier
            critical: false,
            value: subject_key_id.clone(),
            parsed_data: Some(ExtensionData::SubjectKeyIdentifier(subject_key_id)),
        });
        
        // CRL Distribution Points
        if !self.config.crl_distribution_points.is_empty() {
            extensions.push(X509Extension {
                oid: "2.5.29.31".to_string(), // CRL Distribution Points
                critical: false,
                value: self.encode_crl_distribution_points(),
                parsed_data: Some(ExtensionData::CrlDistributionPoints(self.build_distribution_points())),
            });
        }
        
        // Authority Information Access
        if !self.config.ocsp_responders.is_empty() || !self.config.authority_info_access.is_empty() {
            extensions.push(X509Extension {
                oid: "1.3.6.1.5.5.7.1.1".to_string(), // Authority Information Access
                critical: false,
                value: self.encode_authority_info_access(),
                parsed_data: Some(ExtensionData::AuthorityInformationAccess(self.config.authority_info_access.clone())),
            });
        }
        
        // Add custom extensions
        extensions.extend(custom_extensions.iter().cloned());
        
        // Create certificate
        let certificate = X509Certificate {
            version: 3, // X.509 v3
            serial_number,
            signature_algorithm: self.config.signature_algorithm.clone(),
            issuer: self.config.distinguished_name.clone(),
            validity: Validity {
                not_before: now,
                not_after: now + validity_period,
            },
            subject: csr.subject.clone(),
            subject_public_key_info: csr.subject_public_key_info.clone(),
            extensions,
            raw_data: Vec::new(), // Would be filled by DER encoder
            fingerprint: None, // Would be calculated after DER encoding
            key_usage: template.key_usage.clone(),
            extended_key_usage: template.extended_key_usage.clone(),
        };
        
        Ok(certificate)
    }
    
    /// Get default template for certificate purpose
    fn get_default_template_for_purpose(&self, purpose: &CertificatePurpose) -> PkiResult<CertificateTemplate> {
        let template_name = match purpose {
            CertificatePurpose::ServerAuth => "server_auth",
            CertificatePurpose::ClientAuth => "client_auth",
            CertificatePurpose::CodeSigning => "code_signing",
            CertificatePurpose::EmailProtection => "email_protection",
            _ => "general_purpose",
        };
        
        self.certificate_templates.get(template_name)
            .cloned()
            .ok_or_else(|| PkiError::ca_error(
                format!("Default template not found: {}", template_name),
                "get_default_template_for_purpose"
            ))
    }
    
    /// Add a certificate template
    pub fn add_template(&mut self, template: CertificateTemplate) -> PkiResult<()> {
        self.certificate_templates.insert(template.name.clone(), template);
        Ok(())
    }
    
    /// Update statistics for certificate issuance
    fn update_statistics_for_issuance(&mut self, purpose: &CertificatePurpose, certificate: &X509Certificate) {
        self.statistics.total_issued += 1;
        self.statistics.currently_valid += 1;
        
        let purpose_key = format!("{:?}", purpose);
        *self.statistics.by_purpose.entry(purpose_key).or_insert(0) += 1;
        
        let key_algo_key = format!("{:?}", certificate.subject_public_key_info.algorithm);
        *self.statistics.by_key_algorithm.entry(key_algo_key).or_insert(0) += 1;
    }
    
    /// Create default certificate templates
    fn create_server_auth_template() -> CertificateTemplate {
        CertificateTemplate {
            name: "server_auth".to_string(),
            description: "TLS Server Authentication".to_string(),
            key_usage: KeyUsage {
                digital_signature: true,
                key_encipherment: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage {
                server_auth: true,
                ..Default::default()
            },
            validity_period: Duration::from_secs(365 * 24 * 3600), // 1 year
            allowed_key_algorithms: vec![
                PublicKeyAlgorithm::Rsa { key_size: 2048 },
                PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 },
                PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P384 },
            ],
            required_subject_fields: vec!["CN".to_string()],
            extensions: Vec::new(),
            san_requirements: SanRequirements {
                require_dns_names: true,
                require_email_addresses: false,
                require_ip_addresses: false,
                allowed_dns_patterns: vec!["*".to_string()],
                allowed_email_domains: Vec::new(),
            },
        }
    }
    
    fn create_client_auth_template() -> CertificateTemplate {
        CertificateTemplate {
            name: "client_auth".to_string(),
            description: "TLS Client Authentication".to_string(),
            key_usage: KeyUsage {
                digital_signature: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage {
                client_auth: true,
                ..Default::default()
            },
            validity_period: Duration::from_secs(365 * 24 * 3600), // 1 year
            allowed_key_algorithms: vec![
                PublicKeyAlgorithm::Rsa { key_size: 2048 },
                PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 },
                PublicKeyAlgorithm::Ed25519,
            ],
            required_subject_fields: vec!["CN".to_string()],
            extensions: Vec::new(),
            san_requirements: SanRequirements {
                require_dns_names: false,
                require_email_addresses: true,
                require_ip_addresses: false,
                allowed_dns_patterns: Vec::new(),
                allowed_email_domains: vec!["*".to_string()],
            },
        }
    }
    
    fn create_code_signing_template() -> CertificateTemplate {
        CertificateTemplate {
            name: "code_signing".to_string(),
            description: "Code Signing".to_string(),
            key_usage: KeyUsage {
                digital_signature: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage {
                code_signing: true,
                ..Default::default()
            },
            validity_period: Duration::from_secs(3 * 365 * 24 * 3600), // 3 years
            allowed_key_algorithms: vec![
                PublicKeyAlgorithm::Rsa { key_size: 2048 },
                PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 },
            ],
            required_subject_fields: vec!["CN".to_string(), "O".to_string()],
            extensions: Vec::new(),
            san_requirements: SanRequirements {
                require_dns_names: false,
                require_email_addresses: false,
                require_ip_addresses: false,
                allowed_dns_patterns: Vec::new(),
                allowed_email_domains: Vec::new(),
            },
        }
    }
    
    fn create_email_protection_template() -> CertificateTemplate {
        CertificateTemplate {
            name: "email_protection".to_string(),
            description: "Email Protection (S/MIME)".to_string(),
            key_usage: KeyUsage {
                digital_signature: true,
                key_encipherment: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage {
                email_protection: true,
                ..Default::default()
            },
            validity_period: Duration::from_secs(365 * 24 * 3600), // 1 year
            allowed_key_algorithms: vec![
                PublicKeyAlgorithm::Rsa { key_size: 2048 },
                PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 },
            ],
            required_subject_fields: vec!["CN".to_string()],
            extensions: Vec::new(),
            san_requirements: SanRequirements {
                require_dns_names: false,
                require_email_addresses: true,
                require_ip_addresses: false,
                allowed_dns_patterns: Vec::new(),
                allowed_email_domains: vec!["*".to_string()],
            },
        }
    }
    
    /// Extension encoding helpers (simplified implementations)
    fn encode_key_usage(&self, usage: &KeyUsage) -> Vec<u8> {
        // Simplified encoding - would use proper ASN.1 DER encoding
        let mut bits = 0u16;
        if usage.digital_signature { bits |= 0x8000; }
        if usage.non_repudiation { bits |= 0x4000; }
        if usage.key_encipherment { bits |= 0x2000; }
        if usage.data_encipherment { bits |= 0x1000; }
        if usage.key_agreement { bits |= 0x0800; }
        if usage.key_cert_sign { bits |= 0x0400; }
        if usage.crl_sign { bits |= 0x0200; }
        if usage.encipher_only { bits |= 0x0100; }
        if usage.decipher_only { bits |= 0x0080; }
        
        vec![0x03, 0x02, 0x00, (bits >> 8) as u8, bits as u8]
    }
    
    fn encode_extended_key_usage(&self, _ext_usage: &ExtendedKeyUsage) -> Vec<u8> {
        // Simplified encoding
        vec![0x30, 0x00] // Empty sequence for now
    }
    
    fn encode_subject_alternative_names(&self, _sans: &[GeneralName]) -> Vec<u8> {
        // Simplified encoding
        vec![0x30, 0x00] // Empty sequence for now
    }
    
    fn encode_authority_key_identifier(&self) -> Vec<u8> {
        // Simplified encoding
        vec![0x30, 0x00] // Empty sequence for now
    }
    
    fn encode_crl_distribution_points(&self) -> Vec<u8> {
        // Simplified encoding
        vec![0x30, 0x00] // Empty sequence for now
    }
    
    fn encode_authority_info_access(&self) -> Vec<u8> {
        // Simplified encoding
        vec![0x30, 0x00] // Empty sequence for now
    }
    
    fn calculate_subject_key_identifier(&self, public_key: &[u8]) -> Vec<u8> {
        // Simplified implementation - would use proper SHA-1 hash of public key
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        public_key.hash(&mut hasher);
        let hash = hasher.finish();
        hash.to_be_bytes().to_vec()
    }
    
    fn build_distribution_points(&self) -> Vec<DistributionPoint> {
        self.config.crl_distribution_points.iter().map(|url| {
            DistributionPoint {
                distribution_point: Some(DistributionPointName::FullName(vec![
                    GeneralName::UniformResourceIdentifier(url.clone())
                ])),
                reasons: None,
                crl_issuer: None,
            }
        }).collect()
    }
    
    fn generate_crl_extensions(&self) -> Vec<X509Extension> {
        vec![
            // CRL Number extension
            X509Extension {
                oid: "2.5.29.20".to_string(),
                critical: false,
                value: vec![0x02, 0x01, 0x01], // INTEGER 1
                parsed_data: None,
            }
        ]
    }
}

impl CertificateRevocationList {
    /// Create a new empty CRL
    pub fn new() -> Self {
        Self {
            version: Some(2),
            signature_algorithm: SignatureAlgorithm::RsaWithSha256,
            issuer: DistinguishedName::new(),
            this_update: SystemTime::now(),
            next_update: None,
            revoked_certificates: Vec::new(),
            extensions: Vec::new(),
            raw_data: Vec::new(),
        }
    }
}

impl Default for SanRequirements {
    fn default() -> Self {
        Self {
            require_dns_names: false,
            require_email_addresses: false,
            require_ip_addresses: false,
            allowed_dns_patterns: Vec::new(),
            allowed_email_domains: Vec::new(),
        }
    }
}

impl Default for BasicConstraints {
    fn default() -> Self {
        Self {
            is_ca: false,
            path_length_constraint: None,
            critical: true,
        }
    }
}
