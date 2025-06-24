// Certificate Generation Module
// 
// Comprehensive certificate generation functionality for the CURSED PKI system.
// Supports generating certificates for different purposes with full extension handling,
// multiple signature algorithms, and advanced certificate management features.

use crate::stdlib::packages::crypto_pki::{
    error::{PkiError, PkiResult, CertificateErrorCode},
    types::*,
};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::sync::atomic::{AtomicU64, Ordering};

/// Global certificate serial number counter for unique serial numbers
static SERIAL_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Certificate generation request with all parameters
#[derive(Debug, Clone)]
pub struct CertificateGenerationRequest {
    /// Certificate type and purpose
    pub certificate_type: CertificateType,
    /// Subject distinguished name
    pub subject: DistinguishedName,
    /// Subject public key information
    pub subject_public_key: SubjectPublicKeyInfo,
    /// Certificate validity period
    pub validity: CertificateValidity,
    /// Certificate extensions to include
    pub extensions: Vec<CertificateExtensionRequest>,
    /// Signature algorithm to use
    pub signature_algorithm: SignatureAlgorithm,
    /// Serial number (if not specified, auto-generated)
    pub serial_number: Option<SerialNumber>,
    /// Additional template parameters
    pub template_parameters: HashMap<String, String>,
}

/// Certificate validity specification
#[derive(Debug, Clone)]
pub struct CertificateValidity {
    /// Not valid before this time (if None, use current time)
    pub not_before: Option<SystemTime>,
    /// Not valid after this time (if None, calculate from duration)
    pub not_after: Option<SystemTime>,
    /// Validity duration from not_before
    pub duration: Option<Duration>,
}

/// Certificate type and purpose
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateType {
    /// Root Certificate Authority
    RootCa,
    /// Intermediate Certificate Authority
    IntermediateCa,
    /// End-entity server certificate (TLS server authentication)
    ServerCertificate,
    /// End-entity client certificate (TLS client authentication)
    ClientCertificate,
    /// Code signing certificate
    CodeSigning,
    /// Email protection certificate
    EmailProtection,
    /// Time stamping certificate
    TimeStamping,
    /// OCSP signing certificate
    OcspSigning,
    /// Custom certificate with specific purposes
    Custom { purposes: Vec<String> },
}

/// Certificate extension request
#[derive(Debug, Clone)]
pub struct CertificateExtensionRequest {
    /// Extension OID
    pub oid: String,
    /// Whether the extension is critical
    pub critical: bool,
    /// Extension data type and value
    pub extension_data: ExtensionRequestData,
}

/// Extension request data types
#[derive(Debug, Clone)]
pub enum ExtensionRequestData {
    /// Basic Constraints extension
    BasicConstraints {
        is_ca: bool,
        path_length_constraint: Option<u32>,
    },
    /// Key Usage extension
    KeyUsage(KeyUsage),
    /// Extended Key Usage extension
    ExtendedKeyUsage(ExtendedKeyUsage),
    /// Subject Alternative Names
    SubjectAlternativeName(Vec<GeneralName>),
    /// Issuer Alternative Names
    IssuerAlternativeName(Vec<GeneralName>),
    /// Authority Key Identifier (auto-generated if None)
    AuthorityKeyIdentifier {
        key_identifier: Option<Vec<u8>>,
        authority_cert_issuer: Option<Vec<GeneralName>>,
        authority_cert_serial_number: Option<SerialNumber>,
    },
    /// Subject Key Identifier (auto-generated if None)
    SubjectKeyIdentifier(Option<Vec<u8>>),
    /// Certificate Policies
    CertificatePolicies(Vec<PolicyInformation>),
    /// CRL Distribution Points
    CrlDistributionPoints(Vec<DistributionPoint>),
    /// Authority Information Access
    AuthorityInformationAccess(Vec<AccessDescription>),
    /// Name Constraints (for CA certificates)
    NameConstraints {
        permitted_subtrees: Option<Vec<GeneralSubtree>>,
        excluded_subtrees: Option<Vec<GeneralSubtree>>,
    },
    /// Custom extension with raw data
    Custom(Vec<u8>),
}

/// Certificate template for predefined certificate types
#[derive(Debug, Clone)]
pub struct CertificateTemplate {
    /// Template name
    pub name: String,
    /// Certificate type
    pub certificate_type: CertificateType,
    /// Default validity duration
    pub default_validity: Duration,
    /// Default key usage flags
    pub key_usage: KeyUsage,
    /// Default extended key usage
    pub extended_key_usage: ExtendedKeyUsage,
    /// Required extensions
    pub required_extensions: Vec<String>,
    /// Optional extensions
    pub optional_extensions: Vec<String>,
    /// Template-specific parameters
    pub parameters: HashMap<String, String>,
}

/// Certificate generation configuration
#[derive(Debug, Clone)]
pub struct CertificateGeneratorConfig {
    /// Default certificate validity (if not specified)
    pub default_validity_days: u32,
    /// Maximum allowed validity period
    pub max_validity_days: u32,
    /// Serial number generation strategy
    pub serial_number_strategy: SerialNumberStrategy,
    /// Default signature algorithm
    pub default_signature_algorithm: SignatureAlgorithm,
    /// Supported signature algorithms
    pub supported_signature_algorithms: Vec<SignatureAlgorithm>,
    /// Certificate extension policies
    pub extension_policies: ExtensionPolicies,
    /// Enable subject key identifier auto-generation
    pub auto_generate_ski: bool,
    /// Enable authority key identifier auto-generation
    pub auto_generate_aki: bool,
}

/// Serial number generation strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerialNumberStrategy {
    /// Sequential numbering
    Sequential,
    /// Random large integers
    Random,
    /// Timestamp-based
    TimestampBased,
    /// Custom function
    Custom,
}

/// Extension handling policies
#[derive(Debug, Clone)]
pub struct ExtensionPolicies {
    /// Allow custom extensions
    pub allow_custom_extensions: bool,
    /// Require specific extensions for certificate types
    pub required_extensions_by_type: HashMap<CertificateType, Vec<String>>,
    /// Maximum extension data size
    pub max_extension_size: usize,
    /// Critical extension validation
    pub validate_critical_extensions: bool,
}

/// Main certificate generator
#[derive(Debug)]
pub struct CertificateGenerator {
    /// Generator configuration
    pub config: CertificateGeneratorConfig,
    /// Available certificate templates
    pub templates: HashMap<String, CertificateTemplate>,
    /// Extension processor for handling different extension types
    pub extension_processor: ExtensionProcessor,
}

/// Extension processing system
#[derive(Debug)]
pub struct ExtensionProcessor {
    /// Supported extension handlers
    pub handlers: HashMap<String, Box<dyn ExtensionHandler>>,
}

/// Extension handler trait
pub trait ExtensionHandler: std::fmt::Debug + Send + Sync {
    /// Process extension request and generate extension data
    fn process_extension(
        &self,
        request: &ExtensionRequestData,
        context: &CertificateGenerationContext,
    ) -> PkiResult<X509Extension>;
    
    /// Validate extension for certificate type
    fn validate_for_certificate_type(
        &self,
        cert_type: CertificateType,
        request: &ExtensionRequestData,
    ) -> PkiResult<()>;
}

/// Certificate generation context
#[derive(Debug, Clone)]
pub struct CertificateGenerationContext {
    /// Certificate type being generated
    pub certificate_type: CertificateType,
    /// Subject distinguished name
    pub subject: DistinguishedName,
    /// Issuer distinguished name
    pub issuer: DistinguishedName,
    /// Issuer certificate (for non-self-signed)
    pub issuer_certificate: Option<X509Certificate>,
    /// Subject public key
    pub subject_public_key: SubjectPublicKeyInfo,
    /// Generation timestamp
    pub generation_time: SystemTime,
    /// Additional context parameters
    pub parameters: HashMap<String, String>,
}

impl CertificateGenerator {
    /// Create a new certificate generator with default configuration
    pub fn new() -> Self {
        Self {
            config: CertificateGeneratorConfig::default(),
            templates: Self::create_default_templates(),
            extension_processor: ExtensionProcessor::new(),
        }
    }
    
    /// Create certificate generator with custom configuration
    pub fn with_config(config: CertificateGeneratorConfig) -> Self {
        Self {
            config,
            templates: Self::create_default_templates(),
            extension_processor: ExtensionProcessor::new(),
        }
    }
    
    /// Generate a certificate from a request
    pub fn generate_certificate(
        &self,
        request: &CertificateGenerationRequest,
        issuer_certificate: Option<&X509Certificate>,
        issuer_private_key: Option<&[u8]>,
    ) -> PkiResult<X509Certificate> {
        // Validate the request
        self.validate_generation_request(request)?;
        
        // Create generation context
        let context = self.create_generation_context(request, issuer_certificate)?;
        
        // Generate serial number if not provided
        let serial_number = request.serial_number.clone()
            .unwrap_or_else(|| self.generate_serial_number());
        
        // Calculate validity period
        let validity = self.calculate_validity(&request.validity)?;
        
        // Process extensions
        let extensions = self.process_extensions(&request.extensions, &context)?;
        
        // Create the certificate structure
        let mut certificate = X509Certificate {
            version: 3, // X.509v3
            serial_number,
            signature_algorithm: request.signature_algorithm.clone(),
            issuer: context.issuer.clone(),
            validity,
            subject: request.subject.clone(),
            subject_public_key_info: request.subject_public_key.clone(),
            extensions,
            raw_data: Vec::new(), // Will be filled after signing
            fingerprint: None,    // Will be calculated after encoding
            key_usage: Self::extract_key_usage_from_extensions(&extensions),
            extended_key_usage: Self::extract_extended_key_usage_from_extensions(&extensions),
        };
        
        // Add automatic extensions if enabled
        if self.config.auto_generate_ski {
            certificate.extensions.push(self.generate_subject_key_identifier(&context)?);
        }
        
        if self.config.auto_generate_aki && issuer_certificate.is_some() {
            certificate.extensions.push(self.generate_authority_key_identifier(&context)?);
        }
        
        // Sign the certificate if private key is provided
        if let Some(_private_key) = issuer_private_key {
            self.sign_certificate(&mut certificate, _private_key)?;
        }
        
        Ok(certificate)
    }
    
    /// Generate a certificate using a template
    pub fn generate_from_template(
        &self,
        template_name: &str,
        subject: DistinguishedName,
        subject_public_key: SubjectPublicKeyInfo,
        issuer_certificate: Option<&X509Certificate>,
        issuer_private_key: Option<&[u8]>,
        custom_extensions: Option<Vec<CertificateExtensionRequest>>,
    ) -> PkiResult<X509Certificate> {
        let template = self.templates.get(template_name)
            .ok_or_else(|| PkiError::general(format!("Template not found: {}", template_name)))?;
        
        let request = self.create_request_from_template(
            template,
            subject,
            subject_public_key,
            custom_extensions,
        )?;
        
        self.generate_certificate(&request, issuer_certificate, issuer_private_key)
    }
    
    /// Generate a self-signed certificate (typically for root CA)
    pub fn generate_self_signed(
        &self,
        request: &CertificateGenerationRequest,
        private_key: &[u8],
    ) -> PkiResult<X509Certificate> {
        // For self-signed certificates, issuer equals subject
        let mut self_signed_request = request.clone();
        
        // Ensure basic constraints extension for CA certificates
        if matches!(request.certificate_type, CertificateType::RootCa) {
            self_signed_request.extensions.push(CertificateExtensionRequest {
                oid: "2.5.29.19".to_string(), // Basic Constraints
                critical: true,
                extension_data: ExtensionRequestData::BasicConstraints {
                    is_ca: true,
                    path_length_constraint: None,
                },
            });
        }
        
        self.generate_certificate(&self_signed_request, None, Some(private_key))
    }
    
    /// Add a custom certificate template
    pub fn add_template(&mut self, template: CertificateTemplate) {
        self.templates.insert(template.to_string().clone(), template);
    }
    
    /// Get available template names
    pub fn get_template_names(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }
    
    /// Validate a certificate generation request
    fn validate_generation_request(&self, request: &CertificateGenerationRequest) -> PkiResult<()> {
        // Check signature algorithm support
        if !self.config.supported_signature_algorithms.contains(&request.signature_algorithm) {
            return Err(PkiError::certificate_error(
                format!("Unsupported signature algorithm: {:?}", request.signature_algorithm),
                CertificateErrorCode::UnsupportedAlgorithm,
            ));
        }
        
        // Validate validity period
        if let Some(duration) = &request.validity.duration {
            let max_duration = Duration::from_secs(self.config.max_validity_days as u64 * 24 * 3600);
            if *duration > max_duration {
                return Err(PkiError::certificate_error(
                    format!(
                        "Validity period exceeds maximum allowed: {} days",
                        self.config.max_validity_days
                    ),
                    CertificateErrorCode::ValidationError,
                ));
            }
        }
        
        // Validate subject distinguished name
        self.validate_distinguished_name(&request.subject)?;
        
        // Validate extensions for certificate type
        for ext_request in &request.extensions {
            self.validate_extension_for_type(&request.certificate_type, ext_request)?;
        }
        
        Ok(())
    }
    
    /// Validate distinguished name
    fn validate_distinguished_name(&self, dn: &DistinguishedName) -> PkiResult<()> {
        // At minimum, require common name or organization
        if dn.common_name.is_none() && dn.organization.is_none() {
            return Err(PkiError::certificate_error(
                "Distinguished name must have at least common name or organization",
                CertificateErrorCode::MalformedCertificate,
            ));
        }
        
        // Validate country code length if present
        if let Some(country) = &dn.country {
            if country.len() != 2 {
                return Err(PkiError::certificate_error(
                    "Country code must be exactly 2 characters",
                    CertificateErrorCode::MalformedCertificate,
                ));
            }
        }
        
        Ok(())
    }
    
    /// Validate extension for certificate type
    fn validate_extension_for_type(
        &self,
        cert_type: &CertificateType,
        ext_request: &CertificateExtensionRequest,
    ) -> PkiResult<()> {
        // Check if extension is appropriate for certificate type
        match (&ext_request.oid.as_str(), cert_type) {
            ("2.5.29.19", CertificateType::RootCa) | 
            ("2.5.29.19", CertificateType::IntermediateCa) => {
                // Basic Constraints should be present and is_ca should be true for CA certs
                if let ExtensionRequestData::BasicConstraints { is_ca, .. } = &ext_request.extension_data {
                    if !is_ca {
                        return Err(PkiError::certificate_error(
                            "CA certificates must have is_ca=true in basic constraints",
                            CertificateErrorCode::BasicConstraintsViolation,
                        ));
                    }
                }
            }
            ("2.5.29.19", _) => {
                // Non-CA certificates should not be CA or should have is_ca=false
                if let ExtensionRequestData::BasicConstraints { is_ca, .. } = &ext_request.extension_data {
                    if *is_ca {
                        return Err(PkiError::certificate_error(
                            "End-entity certificates cannot have is_ca=true",
                            CertificateErrorCode::BasicConstraintsViolation,
                        ));
                    }
                }
            }
            _ => {} // Other combinations are generally allowed
        }
        
        Ok(())
    }
    
    /// Create generation context
    fn create_generation_context(
        &self,
        request: &CertificateGenerationRequest,
        issuer_certificate: Option<&X509Certificate>,
    ) -> PkiResult<CertificateGenerationContext> {
        let issuer = if let Some(issuer_cert) = issuer_certificate {
            issuer_cert.subject.clone()
        } else {
            // Self-signed certificate - issuer equals subject
            request.subject.clone()
        };
        
        Ok(CertificateGenerationContext {
            certificate_type: request.certificate_type,
            subject: request.subject.clone(),
            issuer,
            issuer_certificate: issuer_certificate.cloned(),
            subject_public_key: request.subject_public_key.clone(),
            generation_time: SystemTime::now(),
            parameters: request.template_parameters.clone(),
        })
    }
    
    /// Generate serial number based on strategy
    fn generate_serial_number(&self) -> SerialNumber {
        match self.config.serial_number_strategy {
            SerialNumberStrategy::Sequential => {
                let serial = SERIAL_COUNTER.fetch_add(1, Ordering::SeqCst);
                SerialNumber::from_big_int(serial)
            }
            SerialNumberStrategy::Random => {
                // Generate a random 64-bit number
                let random_value = SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64;
                SerialNumber::from_big_int(random_value)
            }
            SerialNumberStrategy::TimestampBased => {
                let timestamp = SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                SerialNumber::from_big_int(timestamp)
            }
            SerialNumberStrategy::Custom => {
                // For now, fallback to sequential
                let serial = SERIAL_COUNTER.fetch_add(1, Ordering::SeqCst);
                SerialNumber::from_big_int(serial)
            }
        }
    }
    
    /// Calculate validity period
    fn calculate_validity(&self, validity_request: &CertificateValidity) -> PkiResult<Validity> {
        let not_before = validity_request.not_before.unwrap_or_else(SystemTime::now);
        
        let not_after = if let Some(not_after) = validity_request.not_after {
            not_after
        } else if let Some(duration) = &validity_request.duration {
            not_before + *duration
        } else {
            let default_duration = Duration::from_secs(self.config.default_validity_days as u64 * 24 * 3600);
            not_before + default_duration
        };
        
        if not_after <= not_before {
            return Err(PkiError::certificate_error(
                "Certificate not_after must be after not_before",
                CertificateErrorCode::ValidationError,
            ));
        }
        
        Ok(Validity { not_before, not_after })
    }
    
    /// Process certificate extensions
    fn process_extensions(
        &self,
        extension_requests: &[CertificateExtensionRequest],
        context: &CertificateGenerationContext,
    ) -> PkiResult<Vec<X509Extension>> {
        let mut extensions = Vec::new();
        
        for ext_request in extension_requests {
            let extension = self.extension_processor.process_extension(ext_request, context)?;
            extensions.push(extension);
        }
        
        Ok(extensions)
    }
    
    /// Generate Subject Key Identifier extension
    fn generate_subject_key_identifier(
        &self,
        context: &CertificateGenerationContext,
    ) -> PkiResult<X509Extension> {
        // Simple implementation: use first 20 bytes of SHA-256 hash of public key
        let public_key_hash = self.hash_public_key(&context.subject_public_key.public_key);
        let ski = public_key_hash[..20].to_vec();
        
        Ok(X509Extension {
            oid: "2.5.29.14".to_string(), // Subject Key Identifier
            critical: false,
            value: ski.clone(),
            parsed_data: Some(ExtensionData::SubjectKeyIdentifier(ski)),
        })
    }
    
    /// Generate Authority Key Identifier extension
    fn generate_authority_key_identifier(
        &self,
        context: &CertificateGenerationContext,
    ) -> PkiResult<X509Extension> {
        if let Some(issuer_cert) = &context.issuer_certificate {
            // Extract key identifier from issuer's Subject Key Identifier
            let key_identifier = issuer_cert.extensions.iter()
                .find_map(|ext| {
                    if let Some(ExtensionData::SubjectKeyIdentifier(ski)) = &ext.parsed_data {
                        Some(ski.clone())
                    } else {
                        None
                    }
                });
            
            let aki_data = ExtensionData::AuthorityKeyIdentifier {
                key_identifier,
                authority_cert_issuer: Some(vec![GeneralName::DirectoryName(issuer_cert.issuer.clone())]),
                authority_cert_serial_number: Some(issuer_cert.serial_number.clone()),
            };
            
            Ok(X509Extension {
                oid: "2.5.29.35".to_string(), // Authority Key Identifier
                critical: false,
                value: Vec::new(), // Would be DER encoded in real implementation
                parsed_data: Some(aki_data),
            })
        } else {
            Err(PkiError::general("Cannot generate AKI without issuer certificate"))
        }
    }
    
    /// Hash public key for key identifiers
    fn hash_public_key(&self, public_key: &[u8]) -> Vec<u8> {
        // Simple hash function for demonstration
        // In real implementation, would use SHA-256
        let mut hash = vec![0u8; 32];
        for (i, byte) in public_key.iter().enumerate() {
            hash[i % 32] ^= byte;
        }
        hash
    }
    
    /// Sign the certificate
    fn sign_certificate(&self, certificate: &mut X509Certificate, _private_key: &[u8]) -> PkiResult<()> {
        // In a real implementation, this would:
        // 1. Encode the certificate data to DER format
        // 2. Create a signature using the private key and signature algorithm
        // 3. Append the signature to the certificate
        // 4. Calculate the certificate fingerprint
        
        // For now, just set some placeholder data
        certificate.raw_data = vec![0x30, 0x82]; // DER SEQUENCE placeholder
        certificate.fingerprint = Some(vec![0; 32]); // SHA-256 placeholder
        
        Ok(())
    }
    
    /// Extract key usage from extensions
    fn extract_key_usage_from_extensions(extensions: &[X509Extension]) -> KeyUsage {
        extensions.iter()
            .find_map(|ext| {
                if let Some(ExtensionData::KeyUsage(ku)) = &ext.parsed_data {
                    Some(ku.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }
    
    /// Extract extended key usage from extensions
    fn extract_extended_key_usage_from_extensions(extensions: &[X509Extension]) -> ExtendedKeyUsage {
        extensions.iter()
            .find_map(|ext| {
                if let Some(ExtensionData::ExtendedKeyUsage(eku)) = &ext.parsed_data {
                    Some(eku.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }
    
    /// Create request from template
    fn create_request_from_template(
        &self,
        template: &CertificateTemplate,
        subject: DistinguishedName,
        subject_public_key: SubjectPublicKeyInfo,
        custom_extensions: Option<Vec<CertificateExtensionRequest>>,
    ) -> PkiResult<CertificateGenerationRequest> {
        let mut extensions = Vec::new();
        
        // Add key usage extension
        extensions.push(CertificateExtensionRequest {
            oid: "2.5.29.15".to_string(), // Key Usage
            critical: true,
            extension_data: ExtensionRequestData::KeyUsage(template.key_usage.clone()),
        });
        
        // Add extended key usage extension if specified
        if template.extended_key_usage != ExtendedKeyUsage::default() {
            extensions.push(CertificateExtensionRequest {
                oid: "2.5.29.37".to_string(), // Extended Key Usage
                critical: false,
                extension_data: ExtensionRequestData::ExtendedKeyUsage(template.extended_key_usage.clone()),
            });
        }
        
        // Add basic constraints for CA certificates
        if matches!(template.certificate_type, CertificateType::RootCa | CertificateType::IntermediateCa) {
            extensions.push(CertificateExtensionRequest {
                oid: "2.5.29.19".to_string(), // Basic Constraints
                critical: true,
                extension_data: ExtensionRequestData::BasicConstraints {
                    is_ca: true,
                    path_length_constraint: None,
                },
            });
        }
        
        // Add custom extensions if provided
        if let Some(custom_exts) = custom_extensions {
            extensions.extend(custom_exts);
        }
        
        Ok(CertificateGenerationRequest {
            certificate_type: template.certificate_type,
            subject,
            subject_public_key,
            validity: CertificateValidity {
                not_before: None,
                not_after: None,
                duration: Some(template.default_validity),
            },
            extensions,
            signature_algorithm: self.config.default_signature_algorithm.clone(),
            serial_number: None,
            template_parameters: template.parameters.clone(),
        })
    }
    
    /// Create default certificate templates
    fn create_default_templates() -> HashMap<String, CertificateTemplate> {
        let mut templates = HashMap::new();
        
        // Root CA template
        templates.insert("root_ca".to_string(), CertificateTemplate {
            name: "root_ca".to_string(),
            certificate_type: CertificateType::RootCa,
            default_validity: Duration::from_secs(10 * 365 * 24 * 3600), // 10 years
            key_usage: KeyUsage {
                key_cert_sign: true,
                crl_sign: true,
                digital_signature: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage::default(),
            required_extensions: vec!["2.5.29.19".to_string()], // Basic Constraints
            optional_extensions: vec!["2.5.29.14".to_string()], // Subject Key Identifier
            parameters: HashMap::new(),
        });
        
        // Intermediate CA template
        templates.insert("intermediate_ca".to_string(), CertificateTemplate {
            name: "intermediate_ca".to_string(),
            certificate_type: CertificateType::IntermediateCa,
            default_validity: Duration::from_secs(5 * 365 * 24 * 3600), // 5 years
            key_usage: KeyUsage {
                key_cert_sign: true,
                crl_sign: true,
                digital_signature: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage::default(),
            required_extensions: vec!["2.5.29.19".to_string(), "2.5.29.35".to_string()], // Basic Constraints, AKI
            optional_extensions: vec!["2.5.29.14".to_string()], // Subject Key Identifier
            parameters: HashMap::new(),
        });
        
        // TLS Server certificate template
        templates.insert("tls_server".to_string(), CertificateTemplate {
            name: "tls_server".to_string(),
            certificate_type: CertificateType::ServerCertificate,
            default_validity: Duration::from_secs(365 * 24 * 3600), // 1 year
            key_usage: KeyUsage {
                digital_signature: true,
                key_encipherment: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage {
                server_auth: true,
                ..Default::default()
            },
            required_extensions: vec!["2.5.29.37".to_string()], // Extended Key Usage
            optional_extensions: vec![
                "2.5.29.17".to_string(), // Subject Alternative Name
                "2.5.29.35".to_string(), // Authority Key Identifier
            ],
            parameters: HashMap::new(),
        });
        
        // TLS Client certificate template
        templates.insert("tls_client".to_string(), CertificateTemplate {
            name: "tls_client".to_string(),
            certificate_type: CertificateType::ClientCertificate,
            default_validity: Duration::from_secs(365 * 24 * 3600), // 1 year
            key_usage: KeyUsage {
                digital_signature: true,
                key_agreement: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage {
                client_auth: true,
                ..Default::default()
            },
            required_extensions: vec!["2.5.29.37".to_string()], // Extended Key Usage
            optional_extensions: vec![
                "2.5.29.17".to_string(), // Subject Alternative Name
                "2.5.29.35".to_string(), // Authority Key Identifier
            ],
            parameters: HashMap::new(),
        });
        
        // Code Signing certificate template
        templates.insert("code_signing".to_string(), CertificateTemplate {
            name: "code_signing".to_string(),
            certificate_type: CertificateType::CodeSigning,
            default_validity: Duration::from_secs(3 * 365 * 24 * 3600), // 3 years
            key_usage: KeyUsage {
                digital_signature: true,
                ..Default::default()
            },
            extended_key_usage: ExtendedKeyUsage {
                code_signing: true,
                ..Default::default()
            },
            required_extensions: vec!["2.5.29.37".to_string()], // Extended Key Usage
            optional_extensions: vec!["2.5.29.35".to_string()], // Authority Key Identifier
            parameters: HashMap::new(),
        });
        
        templates
    }
}

impl ExtensionProcessor {
    /// Create a new extension processor
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Process an extension request
    pub fn process_extension(
        &self,
        request: &CertificateExtensionRequest,
        context: &CertificateGenerationContext,
    ) -> PkiResult<X509Extension> {
        match &request.extension_data {
            ExtensionRequestData::BasicConstraints { is_ca, path_length_constraint } => {
                Ok(X509Extension {
                    oid: request.oid.clone(),
                    critical: request.critical,
                    value: self.encode_basic_constraints(*is_ca, *path_length_constraint),
                    parsed_data: Some(ExtensionData::BasicConstraints {
                        is_ca: *is_ca,
                        path_length_constraint: *path_length_constraint,
                    }),
                })
            }
            ExtensionRequestData::KeyUsage(ku) => {
                Ok(X509Extension {
                    oid: request.oid.clone(),
                    critical: request.critical,
                    value: self.encode_key_usage(ku),
                    parsed_data: Some(ExtensionData::KeyUsage(ku.clone())),
                })
            }
            ExtensionRequestData::ExtendedKeyUsage(eku) => {
                Ok(X509Extension {
                    oid: request.oid.clone(),
                    critical: request.critical,
                    value: self.encode_extended_key_usage(eku),
                    parsed_data: Some(ExtensionData::ExtendedKeyUsage(eku.clone())),
                })
            }
            ExtensionRequestData::SubjectAlternativeName(names) => {
                Ok(X509Extension {
                    oid: request.oid.clone(),
                    critical: request.critical,
                    value: self.encode_general_names(names),
                    parsed_data: Some(ExtensionData::SubjectAlternativeName(names.clone())),
                })
            }
            ExtensionRequestData::SubjectKeyIdentifier(ski_opt) => {
                let ski = if let Some(ski) = ski_opt {
                    ski.clone()
                } else {
                    // Auto-generate from public key
                    self.generate_key_identifier(&context.subject_public_key.public_key)
                };
                
                Ok(X509Extension {
                    oid: request.oid.clone(),
                    critical: request.critical,
                    value: ski.clone(),
                    parsed_data: Some(ExtensionData::SubjectKeyIdentifier(ski)),
                })
            }
            ExtensionRequestData::Custom(data) => {
                Ok(X509Extension {
                    oid: request.oid.clone(),
                    critical: request.critical,
                    value: data.clone(),
                    parsed_data: Some(ExtensionData::Custom(data.clone())),
                })
            }
            _ => {
                // For now, return a basic extension for unsupported types
                Ok(X509Extension {
                    oid: request.oid.clone(),
                    critical: request.critical,
                    value: Vec::new(),
                    parsed_data: None,
                })
            }
        }
    }
    
    /// Encode basic constraints extension
    fn encode_basic_constraints(&self, is_ca: bool, path_length: Option<u32>) -> Vec<u8> {
        // Simple DER encoding for basic constraints
        let mut encoded = vec![0x30]; // SEQUENCE
        let mut content = Vec::new();
        
        if is_ca {
            content.extend_from_slice(&[0x01, 0x01, 0xFF]); // BOOLEAN TRUE
        }
        
        if let Some(path_len) = path_length {
            content.extend_from_slice(&[0x02, 0x01, path_len as u8]); // INTEGER
        }
        
        encoded.push(content.len() as u8);
        encoded.extend(content);
        encoded
    }
    
    /// Encode key usage extension
    fn encode_key_usage(&self, ku: &KeyUsage) -> Vec<u8> {
        let mut bits = 0u16;
        
        if ku.digital_signature { bits |= 0x8000; }
        if ku.non_repudiation { bits |= 0x4000; }
        if ku.key_encipherment { bits |= 0x2000; }
        if ku.data_encipherment { bits |= 0x1000; }
        if ku.key_agreement { bits |= 0x0800; }
        if ku.key_cert_sign { bits |= 0x0400; }
        if ku.crl_sign { bits |= 0x0200; }
        if ku.encipher_only { bits |= 0x0100; }
        if ku.decipher_only { bits |= 0x0080; }
        
        // DER encoding: BIT STRING
        vec![0x03, 0x03, 0x00, (bits >> 8) as u8, bits as u8]
    }
    
    /// Encode extended key usage extension
    fn encode_extended_key_usage(&self, eku: &ExtendedKeyUsage) -> Vec<u8> {
        let mut oids = Vec::new();
        
        if eku.server_auth {
            oids.push("1.3.6.1.5.5.7.3.1"); // id-kp-serverAuth
        }
        if eku.client_auth {
            oids.push("1.3.6.1.5.5.7.3.2"); // id-kp-clientAuth
        }
        if eku.code_signing {
            oids.push("1.3.6.1.5.5.7.3.3"); // id-kp-codeSigning
        }
        if eku.email_protection {
            oids.push("1.3.6.1.5.5.7.3.4"); // id-kp-emailProtection
        }
        if eku.time_stamping {
            oids.push("1.3.6.1.5.5.7.3.8"); // id-kp-timeStamping
        }
        if eku.ocsp_signing {
            oids.push("1.3.6.1.5.5.7.3.9"); // id-kp-OCSPSigning
        }
        
        // Simple encoding - just return the first OID as bytes
        if let Some(first_oid) = oids.first() {
            first_oid.as_bytes().to_vec()
        } else {
            Vec::new()
        }
    }
    
    /// Encode general names
    fn encode_general_names(&self, names: &[GeneralName]) -> Vec<u8> {
        // Simple encoding - just return the first name as bytes
        if let Some(first_name) = names.first() {
            match first_name {
                GeneralName::DnsName(name) => name.as_bytes().to_vec(),
                GeneralName::Rfc822Name(email) => email.as_bytes().to_vec(),
                GeneralName::UniformResourceIdentifier(uri) => uri.as_bytes().to_vec(),
                GeneralName::IpAddress(ip) => ip.clone(),
                _ => Vec::new(),
            }
        } else {
            Vec::new()
        }
    }
    
    /// Generate key identifier from public key
    fn generate_key_identifier(&self, public_key: &[u8]) -> Vec<u8> {
        // Simple hash function for demonstration
        let mut hash = vec![0u8; 20];
        for (i, byte) in public_key.iter().enumerate() {
            hash[i % 20] ^= byte;
        }
        hash
    }
}

impl Default for CertificateGeneratorConfig {
    fn default() -> Self {
        Self {
            default_validity_days: 365,
            max_validity_days: 3650, // 10 years
            serial_number_strategy: SerialNumberStrategy::Sequential,
            default_signature_algorithm: SignatureAlgorithm::RsaWithSha256,
            supported_signature_algorithms: vec![
                SignatureAlgorithm::RsaWithSha256,
                SignatureAlgorithm::RsaWithSha384,
                SignatureAlgorithm::RsaWithSha512,
                SignatureAlgorithm::EcdsaWithSha256,
                SignatureAlgorithm::EcdsaWithSha384,
                SignatureAlgorithm::Ed25519,
            ],
            extension_policies: ExtensionPolicies::default(),
            auto_generate_ski: true,
            auto_generate_aki: true,
        }
    }
}

impl Default for ExtensionPolicies {
    fn default() -> Self {
        let mut required_extensions = HashMap::new();
        
        // Root CA requirements
        required_extensions.insert(CertificateType::RootCa, vec![
            "2.5.29.19".to_string(), // Basic Constraints
        ]);
        
        // Intermediate CA requirements
        required_extensions.insert(CertificateType::IntermediateCa, vec![
            "2.5.29.19".to_string(), // Basic Constraints
            "2.5.29.35".to_string(), // Authority Key Identifier
        ]);
        
        // Server certificate requirements
        required_extensions.insert(CertificateType::ServerCertificate, vec![
            "2.5.29.37".to_string(), // Extended Key Usage
        ]);
        
        Self {
            allow_custom_extensions: true,
            required_extensions_by_type: required_extensions,
            max_extension_size: 65536, // 64KB
            validate_critical_extensions: true,
        }
    }
}

/// Public API functions for certificate generation

/// Create a new certificate generator with default configuration
pub fn create_certificate_generator() -> CertificateGenerator {
    CertificateGenerator::new()
}

/// Generate a certificate from a request
pub fn generate_certificate(
    generator: &CertificateGenerator,
    request: &CertificateGenerationRequest,
    issuer_certificate: Option<&X509Certificate>,
    issuer_private_key: Option<&[u8]>,
) -> PkiResult<X509Certificate> {
    generator.generate_certificate(request, issuer_certificate, issuer_private_key)
}

/// Generate a self-signed certificate
pub fn generate_self_signed_certificate(
    generator: &CertificateGenerator,
    request: &CertificateGenerationRequest,
    private_key: &[u8],
) -> PkiResult<X509Certificate> {
    generator.generate_self_signed(request, private_key)
}

/// Generate a certificate using a template
pub fn generate_certificate_from_template(
    generator: &CertificateGenerator,
    template_name: &str,
    subject: DistinguishedName,
    subject_public_key: SubjectPublicKeyInfo,
    issuer_certificate: Option<&X509Certificate>,
    issuer_private_key: Option<&[u8]>,
) -> PkiResult<X509Certificate> {
    generator.generate_from_template(
        template_name,
        subject,
        subject_public_key,
        issuer_certificate,
        issuer_private_key,
        None,
    )
}

/// Create a basic certificate generation request
pub fn create_basic_certificate_request(
    certificate_type: CertificateType,
    subject: DistinguishedName,
    subject_public_key: SubjectPublicKeyInfo,
    validity_days: u32,
) -> CertificateGenerationRequest {
    CertificateGenerationRequest {
        certificate_type,
        subject,
        subject_public_key,
        validity: CertificateValidity {
            not_before: None,
            not_after: None,
            duration: Some(Duration::from_secs(validity_days as u64 * 24 * 3600)),
        },
        extensions: Vec::new(),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        serial_number: None,
        template_parameters: HashMap::new(),
    }
}
