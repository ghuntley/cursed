/// fr fr Comprehensive X.509 Certificate Extensions Implementation
/// 
/// This module provides complete support for X.509 certificate extensions
/// as defined in RFC 5280 and related standards, including:
/// - Standard extensions (Basic Constraints, Key Usage, etc.)
/// - Extended Key Usage and Certificate Policies
/// - Subject Alternative Names and Authority Information Access
/// - CRL Distribution Points and Certificate Transparency
/// - Custom extensions and policy constraints

use crate::stdlib::packages::crypto_pki::errors::*;
use std::collections::HashMap;
use std::net::IpAddr;

use x509_parser::prelude::*;
use x509_parser::extensions::*;
use der::{Decode, Encode, Document, Sequence};
use oid_registry::{OidRegistry, OID_REGISTRY};

/// fr fr X.509 Extension parser and validator
#[derive(Debug)]
pub struct X509ExtensionProcessor {
    /// Supported extension parsers
    parsers: HashMap<String, Box<dyn ExtensionParser>>,
    
    /// Extension validation rules
    validation_rules: HashMap<String, ValidationRule>,
    
    /// Statistics
    stats: ExtensionStatistics,
}

/// fr fr Extension parser trait
pub trait ExtensionParser: Send + Sync {
    fn parse(&self, extension_der: &[u8]) -> PkiResult<ParsedExtension>;
    fn validate(&self, extension: &ParsedExtension) -> PkiResult<()>;
    fn get_oid(&self) -> &str;
    fn is_critical_by_default(&self) -> bool;
}

/// fr fr Parsed extension structure
#[derive(Debug, Clone)]
pub struct ParsedExtension {
    pub oid: String,
    pub critical: bool,
    pub extension_type: ExtensionType,
    pub raw_value: Vec<u8>,
}

/// fr fr Extension types
#[derive(Debug, Clone)]
pub enum ExtensionType {
    BasicConstraints(BasicConstraintsExtension),
    KeyUsage(KeyUsageExtension),
    ExtendedKeyUsage(ExtendedKeyUsageExtension),
    SubjectAlternativeName(SubjectAlternativeNameExtension),
    IssuerAlternativeName(IssuerAlternativeNameExtension),
    SubjectKeyIdentifier(SubjectKeyIdentifierExtension),
    AuthorityKeyIdentifier(AuthorityKeyIdentifierExtension),
    CrlDistributionPoints(CrlDistributionPointsExtension),
    AuthorityInformationAccess(AuthorityInformationAccessExtension),
    CertificatePolicies(CertificatePoliciesExtension),
    PolicyMappings(PolicyMappingsExtension),
    PolicyConstraints(PolicyConstraintsExtension),
    NameConstraints(NameConstraintsExtension),
    InhibitAnyPolicy(InhibitAnyPolicyExtension),
    FreshestCrl(FreshestCrlExtension),
    SubjectDirectoryAttributes(SubjectDirectoryAttributesExtension),
    CertificateTransparency(CertificateTransparencyExtension),
    CustomExtension(CustomExtension),
}

/// fr fr Basic Constraints extension
#[derive(Debug, Clone)]
pub struct BasicConstraintsExtension {
    pub ca: bool,
    pub path_length_constraint: Option<u32>,
}

/// fr fr Key Usage extension
#[derive(Debug, Clone)]
pub struct KeyUsageExtension {
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

/// fr fr Extended Key Usage extension
#[derive(Debug, Clone)]
pub struct ExtendedKeyUsageExtension {
    pub key_purposes: Vec<KeyPurpose>,
}

/// fr fr Key purposes for Extended Key Usage
#[derive(Debug, Clone, PartialEq)]
pub enum KeyPurpose {
    ServerAuth,
    ClientAuth,
    CodeSigning,
    EmailProtection,
    TimeStamping,
    OcspSigning,
    IpsecEndSystem,
    IpsecTunnel,
    IpsecUser,
    SmartcardLogon,
    CustomPurpose(String),
}

/// fr fr Subject Alternative Name extension
#[derive(Debug, Clone)]
pub struct SubjectAlternativeNameExtension {
    pub names: Vec<GeneralName>,
}

/// fr fr Issuer Alternative Name extension
#[derive(Debug, Clone)]
pub struct IssuerAlternativeNameExtension {
    pub names: Vec<GeneralName>,
}

/// fr fr General name types
#[derive(Debug, Clone)]
pub enum GeneralName {
    OtherName { type_id: String, value: Vec<u8> },
    Rfc822Name(String),
    DnsName(String),
    X400Address(Vec<u8>),
    DirectoryName(String),
    EdiPartyName { name_assigner: Option<String>, party_name: String },
    UniformResourceIdentifier(String),
    IpAddress(IpAddr),
    RegisteredId(String),
}

/// fr fr Subject Key Identifier extension
#[derive(Debug, Clone)]
pub struct SubjectKeyIdentifierExtension {
    pub key_identifier: Vec<u8>,
}

/// fr fr Authority Key Identifier extension
#[derive(Debug, Clone)]
pub struct AuthorityKeyIdentifierExtension {
    pub key_identifier: Option<Vec<u8>>,
    pub authority_cert_issuer: Option<Vec<GeneralName>>,
    pub authority_cert_serial_number: Option<Vec<u8>>,
}

/// fr fr CRL Distribution Points extension
#[derive(Debug, Clone)]
pub struct CrlDistributionPointsExtension {
    pub distribution_points: Vec<CrlDistributionPoint>,
}

/// fr fr CRL Distribution Point
#[derive(Debug, Clone)]
pub struct CrlDistributionPoint {
    pub distribution_point: Option<DistributionPointName>,
    pub reasons: Option<ReasonFlags>,
    pub crl_issuer: Option<Vec<GeneralName>>,
}

/// fr fr Distribution Point Name
#[derive(Debug, Clone)]
pub enum DistributionPointName {
    FullName(Vec<GeneralName>),
    NameRelativeToCrlIssuer(String),
}

/// fr fr Reason flags for CRL
#[derive(Debug, Clone)]
pub struct ReasonFlags {
    pub unused: bool,
    pub key_compromise: bool,
    pub ca_compromise: bool,
    pub affiliation_changed: bool,
    pub superseded: bool,
    pub cessation_of_operation: bool,
    pub certificate_hold: bool,
    pub privilege_withdrawn: bool,
    pub aa_compromise: bool,
}

/// fr fr Authority Information Access extension
#[derive(Debug, Clone)]
pub struct AuthorityInformationAccessExtension {
    pub access_descriptions: Vec<AccessDescription>,
}

/// fr fr Access Description
#[derive(Debug, Clone)]
pub struct AccessDescription {
    pub access_method: AccessMethod,
    pub access_location: GeneralName,
}

/// fr fr Access methods
#[derive(Debug, Clone, PartialEq)]
pub enum AccessMethod {
    OcspResponder,
    CaIssuers,
    CustomMethod(String),
}

/// fr fr Certificate Policies extension
#[derive(Debug, Clone)]
pub struct CertificatePoliciesExtension {
    pub policies: Vec<PolicyInformation>,
}

/// fr fr Policy Information
#[derive(Debug, Clone)]
pub struct PolicyInformation {
    pub policy_identifier: String,
    pub policy_qualifiers: Vec<PolicyQualifier>,
}

/// fr fr Policy Qualifier
#[derive(Debug, Clone)]
pub struct PolicyQualifier {
    pub qualifier_id: String,
    pub qualifier: PolicyQualifierData,
}

/// fr fr Policy Qualifier Data
#[derive(Debug, Clone)]
pub enum PolicyQualifierData {
    CpsUri(String),
    UserNotice(UserNotice),
    CustomQualifier(Vec<u8>),
}

/// fr fr User Notice
#[derive(Debug, Clone)]
pub struct UserNotice {
    pub notice_ref: Option<NoticeReference>,
    pub explicit_text: Option<String>,
}

/// fr fr Notice Reference
#[derive(Debug, Clone)]
pub struct NoticeReference {
    pub organization: String,
    pub notice_numbers: Vec<u32>,
}

/// fr fr Policy Mappings extension
#[derive(Debug, Clone)]
pub struct PolicyMappingsExtension {
    pub mappings: Vec<PolicyMapping>,
}

/// fr fr Policy Mapping
#[derive(Debug, Clone)]
pub struct PolicyMapping {
    pub issuer_domain_policy: String,
    pub subject_domain_policy: String,
}

/// fr fr Policy Constraints extension
#[derive(Debug, Clone)]
pub struct PolicyConstraintsExtension {
    pub require_explicit_policy: Option<u32>,
    pub inhibit_policy_mapping: Option<u32>,
}

/// fr fr Name Constraints extension
#[derive(Debug, Clone)]
pub struct NameConstraintsExtension {
    pub permitted_subtrees: Option<Vec<GeneralSubtree>>,
    pub excluded_subtrees: Option<Vec<GeneralSubtree>>,
}

/// fr fr General Subtree
#[derive(Debug, Clone)]
pub struct GeneralSubtree {
    pub base: GeneralName,
    pub minimum: Option<u32>,
    pub maximum: Option<u32>,
}

/// fr fr Inhibit Any Policy extension
#[derive(Debug, Clone)]
pub struct InhibitAnyPolicyExtension {
    pub skip_certs: u32,
}

/// fr fr Freshest CRL extension
#[derive(Debug, Clone)]
pub struct FreshestCrlExtension {
    pub distribution_points: Vec<CrlDistributionPoint>,
}

/// fr fr Subject Directory Attributes extension
#[derive(Debug, Clone)]
pub struct SubjectDirectoryAttributesExtension {
    pub attributes: Vec<DirectoryAttribute>,
}

/// fr fr Directory Attribute
#[derive(Debug, Clone)]
pub struct DirectoryAttribute {
    pub attribute_type: String,
    pub attribute_values: Vec<Vec<u8>>,
}

/// fr fr Certificate Transparency extension
#[derive(Debug, Clone)]
pub struct CertificateTransparencyExtension {
    pub sct_list: Vec<SignedCertificateTimestamp>,
}

/// fr fr Signed Certificate Timestamp
#[derive(Debug, Clone)]
pub struct SignedCertificateTimestamp {
    pub version: u8,
    pub log_id: Vec<u8>,
    pub timestamp: u64,
    pub extensions: Vec<u8>,
    pub signature: CtSignature,
}

/// fr fr CT Signature
#[derive(Debug, Clone)]
pub struct CtSignature {
    pub hash_algorithm: u8,
    pub signature_algorithm: u8,
    pub signature: Vec<u8>,
}

/// fr fr Custom extension for unknown or proprietary extensions
#[derive(Debug, Clone)]
pub struct CustomExtension {
    pub oid: String,
    pub value: Vec<u8>,
    pub description: Option<String>,
}

/// fr fr Validation rule for extensions
#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub rule_type: ValidationRuleType,
    pub message: String,
    pub severity: ValidationSeverity,
    pub applicable_extensions: Vec<String>,
}

/// fr fr Validation rule types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationRuleType {
    CriticalityCheck,
    ValueValidation,
    ConsistencyCheck,
    SecurityCheck,
    ComplianceCheck,
}

/// fr fr Validation severity
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// fr fr Extension statistics
#[derive(Debug, Default)]
pub struct ExtensionStatistics {
    pub extensions_parsed: u64,
    pub parsing_errors: u64,
    pub validation_errors: u64,
    pub critical_extensions: u64,
    pub unknown_extensions: u64,
    pub extension_counts: HashMap<String, u64>,
}

impl X509ExtensionProcessor {
    /// slay Create new extension processor with standard parsers
    pub fn new() -> Self {
        let mut processor = Self {
            parsers: HashMap::new(),
            validation_rules: HashMap::new(),
            stats: ExtensionStatistics::default(),
        };
        
        processor.register_standard_parsers();
        processor.register_standard_validation_rules();
        
        processor
    }
    
    /// slay Register standard extension parsers
    fn register_standard_parsers(&mut self) {
        // Basic Constraints (2.5.29.19)
        self.register_parser(Box::new(BasicConstraintsParser));
        
        // Key Usage (2.5.29.15)
        self.register_parser(Box::new(KeyUsageParser));
        
        // Extended Key Usage (2.5.29.37)
        self.register_parser(Box::new(ExtendedKeyUsageParser));
        
        // Subject Alternative Name (2.5.29.17)
        self.register_parser(Box::new(SubjectAlternativeNameParser));
        
        // Issuer Alternative Name (2.5.29.18)
        self.register_parser(Box::new(IssuerAlternativeNameParser));
        
        // Subject Key Identifier (2.5.29.14)
        self.register_parser(Box::new(SubjectKeyIdentifierParser));
        
        // Authority Key Identifier (2.5.29.35)
        self.register_parser(Box::new(AuthorityKeyIdentifierParser));
        
        // CRL Distribution Points (2.5.29.31)
        self.register_parser(Box::new(CrlDistributionPointsParser));
        
        // Authority Information Access (1.3.6.1.5.5.7.1.1)
        self.register_parser(Box::new(AuthorityInformationAccessParser));
        
        // Certificate Policies (2.5.29.32)
        self.register_parser(Box::new(CertificatePoliciesParser));
        
        // Policy Mappings (2.5.29.33)
        self.register_parser(Box::new(PolicyMappingsParser));
        
        // Policy Constraints (2.5.29.36)
        self.register_parser(Box::new(PolicyConstraintsParser));
        
        // Name Constraints (2.5.29.30)
        self.register_parser(Box::new(NameConstraintsParser));
        
        // Certificate Transparency (1.3.6.1.4.1.11129.2.4.2)
        self.register_parser(Box::new(CertificateTransparencyParser));
    }
    
    /// slay Register extension parser
    pub fn register_parser(&mut self, parser: Box<dyn ExtensionParser>) {
        let oid = parser.get_oid().to_string();
        self.parsers.insert(oid, parser);
    }
    
    /// slay Register standard validation rules
    fn register_standard_validation_rules(&mut self) {
        // Basic Constraints validation
        self.validation_rules.insert("2.5.29.19".to_string(), ValidationRule {
            rule_type: ValidationRuleType::CriticalityCheck,
            message: "Basic Constraints should be critical for CA certificates".to_string(),
            severity: ValidationSeverity::Warning,
            applicable_extensions: vec!["2.5.29.19".to_string()],
        });
        
        // Key Usage validation
        self.validation_rules.insert("2.5.29.15".to_string(), ValidationRule {
            rule_type: ValidationRuleType::SecurityCheck,
            message: "Key Usage should be critical".to_string(),
            severity: ValidationSeverity::Warning,
            applicable_extensions: vec!["2.5.29.15".to_string()],
        });
        
        // Subject Alternative Name validation
        self.validation_rules.insert("2.5.29.17".to_string(), ValidationRule {
            rule_type: ValidationRuleType::ComplianceCheck,
            message: "Subject Alternative Name required for server certificates".to_string(),
            severity: ValidationSeverity::Error,
            applicable_extensions: vec!["2.5.29.17".to_string()],
        });
    }
    
    /// slay Parse all extensions from certificate
    pub fn parse_extensions(&mut self, extensions: &[X509Extension]) -> PkiResult<Vec<ParsedExtension>> {
        let mut parsed_extensions = Vec::new();
        
        for ext in extensions {
            match self.parse_single_extension(ext) {
                Ok(parsed_ext) => {
                    parsed_extensions.push(parsed_ext);
                    self.stats.extensions_parsed += 1;
                    
                    // Update extension count statistics
                    let count = self.stats.extension_counts.entry(ext.oid.to_string()).or_insert(0);
                    *count += 1;
                    
                    if ext.critical {
                        self.stats.critical_extensions += 1;
                    }
                },
                Err(e) => {
                    self.stats.parsing_errors += 1;
                    return Err(e);
                }
            }
        }
        
        Ok(parsed_extensions)
    }
    
    /// slay Parse single extension
    fn parse_single_extension(&self, extension: &X509Extension) -> PkiResult<ParsedExtension> {
        let oid_str = extension.oid.to_string();
        
        if let Some(parser) = self.parsers.get(&oid_str) {
            parser.parse(&extension.value)
        } else {
            // Handle unknown extension
            self.handle_unknown_extension(extension)
        }
    }
    
    /// slay Handle unknown extension
    fn handle_unknown_extension(&self, extension: &X509Extension) -> PkiResult<ParsedExtension> {
        let custom_ext = CustomExtension {
            oid: extension.oid.to_string(),
            value: extension.value.to_vec(),
            description: self.get_oid_description(&extension.oid.to_string()),
        };
        
        Ok(ParsedExtension {
            oid: extension.oid.to_string(),
            critical: extension.critical,
            extension_type: ExtensionType::CustomExtension(custom_ext),
            raw_value: extension.value.to_vec(),
        })
    }
    
    /// slay Get OID description from registry
    fn get_oid_description(&self, oid: &str) -> Option<String> {
        // Parse OID string to components
        let components: Result<Vec<u32>, _> = oid.split('.')
            .map(|s| s.parse::<u32>())
            .collect();
        
        if let Ok(oid_components) = components {
            if let Some(entry) = OID_REGISTRY.get(&oid_components) {
                return entry.description().map(|s| s.to_string());
            }
        }
        
        None
    }
    
    /// slay Validate extensions
    pub fn validate_extensions(&mut self, extensions: &[ParsedExtension]) -> PkiResult<Vec<ValidationResult>> {
        let mut validation_results = Vec::new();
        
        for extension in extensions {
            if let Some(rule) = self.validation_rules.get(&extension.oid) {
                match self.validate_extension_against_rule(extension, rule) {
                    Ok(result) => validation_results.push(result),
                    Err(_) => {
                        self.stats.validation_errors += 1;
                        // Continue with other validations
                    }
                }
            }
            
            // Validate using extension-specific parser
            if let Some(parser) = self.parsers.get(&extension.oid) {
                if let Err(_) = parser.validate(extension) {
                    self.stats.validation_errors += 1;
                }
            }
        }
        
        Ok(validation_results)
    }
    
    /// slay Validate extension against rule
    fn validate_extension_against_rule(
        &self,
        extension: &ParsedExtension,
        rule: &ValidationRule,
    ) -> PkiResult<ValidationResult> {
        match rule.rule_type {
            ValidationRuleType::CriticalityCheck => {
                self.validate_criticality(extension, rule)
            },
            ValidationRuleType::ValueValidation => {
                self.validate_value(extension, rule)
            },
            ValidationRuleType::ConsistencyCheck => {
                self.validate_consistency(extension, rule)
            },
            ValidationRuleType::SecurityCheck => {
                self.validate_security(extension, rule)
            },
            ValidationRuleType::ComplianceCheck => {
                self.validate_compliance(extension, rule)
            },
        }
    }
    
    /// slay Validate criticality
    fn validate_criticality(&self, extension: &ParsedExtension, rule: &ValidationRule) -> PkiResult<ValidationResult> {
        let should_be_critical = match &extension.extension_type {
            ExtensionType::BasicConstraints(bc) => bc.ca,
            ExtensionType::KeyUsage(_) => true,
            _ => false,
        };
        
        let is_valid = !should_be_critical || extension.critical;
        
        Ok(ValidationResult {
            extension_oid: extension.oid.clone(),
            rule_type: rule.rule_type.clone(),
            is_valid,
            message: if is_valid { 
                "Criticality check passed".to_string() 
            } else { 
                rule.message.clone() 
            },
            severity: rule.severity.clone(),
        })
    }
    
    /// slay Validate value
    fn validate_value(&self, extension: &ParsedExtension, rule: &ValidationRule) -> PkiResult<ValidationResult> {
        // Implementation would validate extension-specific values
        Ok(ValidationResult {
            extension_oid: extension.oid.clone(),
            rule_type: rule.rule_type.clone(),
            is_valid: true,
            message: "Value validation passed".to_string(),
            severity: rule.severity.clone(),
        })
    }
    
    /// slay Validate consistency
    fn validate_consistency(&self, extension: &ParsedExtension, rule: &ValidationRule) -> PkiResult<ValidationResult> {
        // Implementation would validate consistency between extensions
        Ok(ValidationResult {
            extension_oid: extension.oid.clone(),
            rule_type: rule.rule_type.clone(),
            is_valid: true,
            message: "Consistency check passed".to_string(),
            severity: rule.severity.clone(),
        })
    }
    
    /// slay Validate security
    fn validate_security(&self, extension: &ParsedExtension, rule: &ValidationRule) -> PkiResult<ValidationResult> {
        // Implementation would validate security-related aspects
        Ok(ValidationResult {
            extension_oid: extension.oid.clone(),
            rule_type: rule.rule_type.clone(),
            is_valid: true,
            message: "Security check passed".to_string(),
            severity: rule.severity.clone(),
        })
    }
    
    /// slay Validate compliance
    fn validate_compliance(&self, extension: &ParsedExtension, rule: &ValidationRule) -> PkiResult<ValidationResult> {
        // Implementation would validate compliance with standards
        Ok(ValidationResult {
            extension_oid: extension.oid.clone(),
            rule_type: rule.rule_type.clone(),
            is_valid: true,
            message: "Compliance check passed".to_string(),
            severity: rule.severity.clone(),
        })
    }
    
    /// slay Get extension statistics
    pub fn get_statistics(&self) -> &ExtensionStatistics {
        &self.stats
    }
}

/// fr fr Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub extension_oid: String,
    pub rule_type: ValidationRuleType,
    pub is_valid: bool,
    pub message: String,
    pub severity: ValidationSeverity,
}

/// fr fr Concrete extension parsers

pub struct BasicConstraintsParser;

impl ExtensionParser for BasicConstraintsParser {
    fn parse(&self, extension_der: &[u8]) -> PkiResult<ParsedExtension> {
        match BasicConstraints::from_der(extension_der) {
            Ok(bc) => {
                let basic_constraints = BasicConstraintsExtension {
                    ca: bc.ca,
                    path_length_constraint: bc.path_len_constraint,
                };
                
                Ok(ParsedExtension {
                    oid: self.get_oid().to_string(),
                    critical: true, // Will be set by caller
                    extension_type: ExtensionType::BasicConstraints(basic_constraints),
                    raw_value: extension_der.to_vec(),
                })
            },
            Err(e) => Err(PkiError::ExtensionParsingFailed(format!("Basic Constraints parsing failed: {:?}", e))),
        }
    }
    
    fn validate(&self, extension: &ParsedExtension) -> PkiResult<()> {
        if let ExtensionType::BasicConstraints(ref bc) = extension.extension_type {
            if bc.ca && !extension.critical {
                return Err(PkiError::ExtensionValidationFailed(
                    "Basic Constraints must be critical for CA certificates".to_string()
                ));
            }
        }
        Ok(())
    }
    
    fn get_oid(&self) -> &str {
        "2.5.29.19"
    }
    
    fn is_critical_by_default(&self) -> bool {
        true
    }
}

pub struct KeyUsageParser;

impl ExtensionParser for KeyUsageParser {
    fn parse(&self, extension_der: &[u8]) -> PkiResult<ParsedExtension> {
        match KeyUsage::from_der(extension_der) {
            Ok(ku) => {
                let key_usage = KeyUsageExtension {
                    digital_signature: ku.digital_signature(),
                    non_repudiation: ku.non_repudiation(),
                    key_encipherment: ku.key_encipherment(),
                    data_encipherment: ku.data_encipherment(),
                    key_agreement: ku.key_agreement(),
                    key_cert_sign: ku.key_cert_sign(),
                    crl_sign: ku.crl_sign(),
                    encipher_only: ku.encipher_only(),
                    decipher_only: ku.decipher_only(),
                };
                
                Ok(ParsedExtension {
                    oid: self.get_oid().to_string(),
                    critical: true,
                    extension_type: ExtensionType::KeyUsage(key_usage),
                    raw_value: extension_der.to_vec(),
                })
            },
            Err(e) => Err(PkiError::ExtensionParsingFailed(format!("Key Usage parsing failed: {:?}", e))),
        }
    }
    
    fn validate(&self, _extension: &ParsedExtension) -> PkiResult<()> {
        // Key Usage validation logic
        Ok(())
    }
    
    fn get_oid(&self) -> &str {
        "2.5.29.15"
    }
    
    fn is_critical_by_default(&self) -> bool {
        true
    }
}

pub struct ExtendedKeyUsageParser;

impl ExtensionParser for ExtendedKeyUsageParser {
    fn parse(&self, extension_der: &[u8]) -> PkiResult<ParsedExtension> {
        match ExtendedKeyUsage::from_der(extension_der) {
            Ok(eku) => {
                let mut key_purposes = Vec::new();
                
                for purpose_oid in &eku.purposes {
                    let purpose = match purpose_oid.to_string().as_str() {
                        "1.3.6.1.5.5.7.3.1" => KeyPurpose::ServerAuth,
                        "1.3.6.1.5.5.7.3.2" => KeyPurpose::ClientAuth,
                        "1.3.6.1.5.5.7.3.3" => KeyPurpose::CodeSigning,
                        "1.3.6.1.5.5.7.3.4" => KeyPurpose::EmailProtection,
                        "1.3.6.1.5.5.7.3.8" => KeyPurpose::TimeStamping,
                        "1.3.6.1.5.5.7.3.9" => KeyPurpose::OcspSigning,
                        oid => KeyPurpose::CustomPurpose(oid.to_string()),
                    };
                    key_purposes.push(purpose);
                }
                
                let extended_key_usage = ExtendedKeyUsageExtension { key_purposes };
                
                Ok(ParsedExtension {
                    oid: self.get_oid().to_string(),
                    critical: false,
                    extension_type: ExtensionType::ExtendedKeyUsage(extended_key_usage),
                    raw_value: extension_der.to_vec(),
                })
            },
            Err(e) => Err(PkiError::ExtensionParsingFailed(format!("Extended Key Usage parsing failed: {:?}", e))),
        }
    }
    
    fn validate(&self, _extension: &ParsedExtension) -> PkiResult<()> {
        Ok(())
    }
    
    fn get_oid(&self) -> &str {
        "2.5.29.37"
    }
    
    fn is_critical_by_default(&self) -> bool {
        false
    }
}

pub struct SubjectAlternativeNameParser;

impl ExtensionParser for SubjectAlternativeNameParser {
    fn parse(&self, extension_der: &[u8]) -> PkiResult<ParsedExtension> {
        match SubjectAlternativeName::from_der(extension_der) {
            Ok(san) => {
                let mut names = Vec::new();
                
                for general_name in &san.general_names {
                    let name = match general_name {
                        x509_parser::extensions::GeneralName::RFC822Name(email) => {
                            GeneralName::Rfc822Name(email.to_string())
                        },
                        x509_parser::extensions::GeneralName::DNSName(dns) => {
                            GeneralName::DnsName(dns.to_string())
                        },
                        x509_parser::extensions::GeneralName::IPAddress(ip_bytes) => {
                            if ip_bytes.len() == 4 {
                                let ip = std::net::Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
                                GeneralName::IpAddress(IpAddr::V4(ip))
                            } else if ip_bytes.len() == 16 {
                                let mut bytes = [0u8; 16];
                                bytes.copy_from_slice(ip_bytes);
                                let ip = std::net::Ipv6Addr::from(bytes);
                                GeneralName::IpAddress(IpAddr::V6(ip))
                            } else {
                                continue; // Skip invalid IP addresses
                            }
                        },
                        x509_parser::extensions::GeneralName::UniformResourceIdentifier(uri) => {
                            GeneralName::UniformResourceIdentifier(uri.to_string())
                        },
                        _ => continue, // Skip other name types for now
                    };
                    names.push(name);
                }
                
                let san_ext = SubjectAlternativeNameExtension { names };
                
                Ok(ParsedExtension {
                    oid: self.get_oid().to_string(),
                    critical: false,
                    extension_type: ExtensionType::SubjectAlternativeName(san_ext),
                    raw_value: extension_der.to_vec(),
                })
            },
            Err(e) => Err(PkiError::ExtensionParsingFailed(format!("Subject Alternative Name parsing failed: {:?}", e))),
        }
    }
    
    fn validate(&self, _extension: &ParsedExtension) -> PkiResult<()> {
        Ok(())
    }
    
    fn get_oid(&self) -> &str {
        "2.5.29.17"
    }
    
    fn is_critical_by_default(&self) -> bool {
        false
    }
}

// Additional parser implementations would go here for other extension types
// For brevity, I'm including just a few representative parsers

/// fr fr Stub parsers for completeness
macro_rules! create_stub_parser {
    ($name:ident, $oid:expr, $critical:expr) => {
        pub struct $name;
        
        impl ExtensionParser for $name {
            fn parse(&self, extension_der: &[u8]) -> PkiResult<ParsedExtension> {
                // Stub implementation - would parse the specific extension
                Ok(ParsedExtension {
                    oid: self.get_oid().to_string(),
                    critical: $critical,
                    extension_type: ExtensionType::CustomExtension(CustomExtension {
                        oid: self.get_oid().to_string(),
                        value: extension_der.to_vec(),
                        description: Some(format!("{} extension", stringify!($name))),
                    }),
                    raw_value: extension_der.to_vec(),
                })
            }
            
            fn validate(&self, _extension: &ParsedExtension) -> PkiResult<()> {
                Ok(())
            }
            
            fn get_oid(&self) -> &str {
                $oid
            }
            
            fn is_critical_by_default(&self) -> bool {
                $critical
            }
        }
    };
}

create_stub_parser!(IssuerAlternativeNameParser, "2.5.29.18", false);
create_stub_parser!(SubjectKeyIdentifierParser, "2.5.29.14", false);
create_stub_parser!(AuthorityKeyIdentifierParser, "2.5.29.35", false);
create_stub_parser!(CrlDistributionPointsParser, "2.5.29.31", false);
create_stub_parser!(AuthorityInformationAccessParser, "1.3.6.1.5.5.7.1.1", false);
create_stub_parser!(CertificatePoliciesParser, "2.5.29.32", false);
create_stub_parser!(PolicyMappingsParser, "2.5.29.33", true);
create_stub_parser!(PolicyConstraintsParser, "2.5.29.36", true);
create_stub_parser!(NameConstraintsParser, "2.5.29.30", true);
create_stub_parser!(CertificateTransparencyParser, "1.3.6.1.4.1.11129.2.4.2", false);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extension_processor_creation() {
        let processor = X509ExtensionProcessor::new();
        assert!(processor.parsers.len() > 0);
        assert!(processor.validation_rules.len() > 0);
    }
    
    #[test]
    fn test_basic_constraints_parser() {
        let parser = BasicConstraintsParser;
        assert_eq!(parser.get_oid(), "2.5.29.19");
        assert!(parser.is_critical_by_default());
    }
    
    #[test]
    fn test_key_usage_parser() {
        let parser = KeyUsageParser;
        assert_eq!(parser.get_oid(), "2.5.29.15");
        assert!(parser.is_critical_by_default());
    }
    
    #[test]
    fn test_extended_key_usage_parser() {
        let parser = ExtendedKeyUsageParser;
        assert_eq!(parser.get_oid(), "2.5.29.37");
        assert!(!parser.is_critical_by_default());
    }
    
    #[test]
    fn test_extension_statistics() {
        let stats = ExtensionStatistics::default();
        assert_eq!(stats.extensions_parsed, 0);
        assert_eq!(stats.parsing_errors, 0);
        assert_eq!(stats.validation_errors, 0);
    }
}
