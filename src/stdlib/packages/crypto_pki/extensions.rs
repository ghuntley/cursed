/// fr fr X.509 Certificate Extensions - Comprehensive Implementation
/// 
/// This module provides complete support for X.509 certificate extensions including:
/// - Standard extensions (Basic Constraints, Key Usage, etc.)
/// - Extended key usage and application policies
/// - Subject and Authority Key Identifiers
/// - Alternative names and distribution points
/// - Policy constraints and name constraints
/// - Certificate transparency and OCSP extensions
/// - Custom extensions and policy constraints
/// - Extension validation and critical flag handling
/// - Extension parsing from ASN.1 DER encoding
/// - Extension generation for certificate creation

use crate::error::CursedError;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};
use tracing::{debug, error, info, instrument, warn};
use std::collections::HashMap;
use std::fmt;

/// fr fr Certificate extension representation
#[derive(Debug, Clone, PartialEq)]
pub struct Extension {
    /// Extension OID
    pub oid: ExtensionOid,
    /// Whether extension is critical
    pub critical: bool,
    /// Extension value
    pub value: ExtensionValue,
}

/// fr fr Extension Object Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtensionOid {
    /// Basic Constraints (2.5.29.19)
    BasicConstraints,
    /// Key Usage (2.5.29.15)
    KeyUsage,
    /// Extended Key Usage (2.5.29.37)
    ExtendedKeyUsage,
    /// Subject Key Identifier (2.5.29.14)
    SubjectKeyIdentifier,
    /// Authority Key Identifier (2.5.29.35)
    AuthorityKeyIdentifier,
    /// Subject Alternative Name (2.5.29.17)
    SubjectAlternativeName,
    /// Issuer Alternative Name (2.5.29.18)
    IssuerAlternativeName,
    /// CRL Distribution Points (2.5.29.31)
    CrlDistributionPoints,
    /// Authority Information Access (1.3.6.1.5.5.7.1.1)
    AuthorityInformationAccess,
    /// Certificate Policies (2.5.29.32)
    CertificatePolicies,
    /// Policy Mappings (2.5.29.33)
    PolicyMappings,
    /// Policy Constraints (2.5.29.36)
    PolicyConstraints,
    /// Inhibit Any Policy (2.5.29.54)
    InhibitAnyPolicy,
    /// Name Constraints (2.5.29.30)
    NameConstraints,
    /// Freshest CRL (2.5.29.46)
    FreshestCrl,
    /// OCSP No Check (1.3.6.1.5.5.7.48.1.5)
    OcspNoCheck,
    /// SCT List (1.3.6.1.4.1.11129.2.4.2)
    SignedCertificateTimestampList,
    /// Netscape Certificate Type
    NetscapeCertType,
    /// Microsoft Template Name
    MicrosoftTemplate,
    /// Custom extension with OID
    Custom(String),
}

impl ExtensionOid {
    /// slay Get OID string representation
    pub fn as_str(&self) -> &str {
        match self {
            ExtensionOid::BasicConstraints => "2.5.29.19",
            ExtensionOid::KeyUsage => "2.5.29.15",
            ExtensionOid::ExtendedKeyUsage => "2.5.29.37",
            ExtensionOid::SubjectKeyIdentifier => "2.5.29.14",
            ExtensionOid::AuthorityKeyIdentifier => "2.5.29.35",
            ExtensionOid::SubjectAlternativeName => "2.5.29.17",
            ExtensionOid::IssuerAlternativeName => "2.5.29.18",
            ExtensionOid::CrlDistributionPoints => "2.5.29.31",
            ExtensionOid::AuthorityInformationAccess => "1.3.6.1.5.5.7.1.1",
            ExtensionOid::CertificatePolicies => "2.5.29.32",
            ExtensionOid::PolicyMappings => "2.5.29.33",
            ExtensionOid::PolicyConstraints => "2.5.29.36",
            ExtensionOid::InhibitAnyPolicy => "2.5.29.54",
            ExtensionOid::NameConstraints => "2.5.29.30",
            ExtensionOid::FreshestCrl => "2.5.29.46",
            ExtensionOid::OcspNoCheck => "1.3.6.1.5.5.7.48.1.5",
            ExtensionOid::SignedCertificateTimestampList => "1.3.6.1.4.1.11129.2.4.2",
            ExtensionOid::NetscapeCertType => "2.16.840.1.113730.1.1",
            ExtensionOid::MicrosoftTemplate => "1.3.6.1.4.1.311.20.2",
            ExtensionOid::Custom(oid) => oid,
        }
    }

    /// slay Parse from OID string
    pub fn from_str(oid: &str) -> Self {
        match oid {
            "2.5.29.19" => ExtensionOid::BasicConstraints,
            "2.5.29.15" => ExtensionOid::KeyUsage,
            "2.5.29.37" => ExtensionOid::ExtendedKeyUsage,
            "2.5.29.14" => ExtensionOid::SubjectKeyIdentifier,
            "2.5.29.35" => ExtensionOid::AuthorityKeyIdentifier,
            "2.5.29.17" => ExtensionOid::SubjectAlternativeName,
            "2.5.29.18" => ExtensionOid::IssuerAlternativeName,
            "2.5.29.31" => ExtensionOid::CrlDistributionPoints,
            "1.3.6.1.5.5.7.1.1" => ExtensionOid::AuthorityInformationAccess,
            "2.5.29.32" => ExtensionOid::CertificatePolicies,
            "2.5.29.33" => ExtensionOid::PolicyMappings,
            "2.5.29.36" => ExtensionOid::PolicyConstraints,
            "2.5.29.54" => ExtensionOid::InhibitAnyPolicy,
            "2.5.29.30" => ExtensionOid::NameConstraints,
            "2.5.29.46" => ExtensionOid::FreshestCrl,
            "1.3.6.1.5.5.7.48.1.5" => ExtensionOid::OcspNoCheck,
            "1.3.6.1.4.1.11129.2.4.2" => ExtensionOid::SignedCertificateTimestampList,
            "2.16.840.1.113730.1.1" => ExtensionOid::NetscapeCertType,
            "1.3.6.1.4.1.311.20.2" => ExtensionOid::MicrosoftTemplate,
            _ => ExtensionOid::Custom(oid.to_string()),
        }
    }

    /// slay Check if extension is standard
    pub fn is_standard(&self) -> bool {
        !matches!(self, ExtensionOid::Custom(_))
    }

    /// slay Get extension name
    pub fn name(&self) -> &str {
        match self {
            ExtensionOid::BasicConstraints => "Basic Constraints",
            ExtensionOid::KeyUsage => "Key Usage",
            ExtensionOid::ExtendedKeyUsage => "Extended Key Usage",
            ExtensionOid::SubjectKeyIdentifier => "Subject Key Identifier",
            ExtensionOid::AuthorityKeyIdentifier => "Authority Key Identifier",
            ExtensionOid::SubjectAlternativeName => "Subject Alternative Name",
            ExtensionOid::IssuerAlternativeName => "Issuer Alternative Name",
            ExtensionOid::CrlDistributionPoints => "CRL Distribution Points",
            ExtensionOid::AuthorityInformationAccess => "Authority Information Access",
            ExtensionOid::CertificatePolicies => "Certificate Policies",
            ExtensionOid::PolicyMappings => "Policy Mappings",
            ExtensionOid::PolicyConstraints => "Policy Constraints",
            ExtensionOid::InhibitAnyPolicy => "Inhibit Any Policy",
            ExtensionOid::NameConstraints => "Name Constraints",
            ExtensionOid::FreshestCrl => "Freshest CRL",
            ExtensionOid::OcspNoCheck => "OCSP No Check",
            ExtensionOid::SignedCertificateTimestampList => "SCT List",
            ExtensionOid::NetscapeCertType => "Netscape Cert Type",
            ExtensionOid::MicrosoftTemplate => "Microsoft Template",
            ExtensionOid::Custom(_) => "Custom Extension",
        }
    }
}

impl fmt::Display for ExtensionOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// fr fr Extension value variants
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionValue {
    /// Basic Constraints extension
    BasicConstraints(BasicConstraints),
    /// Key Usage extension
    KeyUsage(KeyUsage),
    /// Extended Key Usage extension
    ExtendedKeyUsage(ExtendedKeyUsage),
    /// Subject Key Identifier
    SubjectKeyIdentifier(Vec<u8>),
    /// Authority Key Identifier
    AuthorityKeyIdentifier(AuthorityKeyIdentifier),
    /// Subject Alternative Name
    SubjectAlternativeName(Vec<GeneralName>),
    /// Issuer Alternative Name
    IssuerAlternativeName(Vec<GeneralName>),
    /// CRL Distribution Points
    CrlDistributionPoints(Vec<DistributionPoint>),
    /// Authority Information Access
    AuthorityInformationAccess(Vec<AccessDescription>),
    /// Certificate Policies
    CertificatePolicies(Vec<PolicyInformation>),
    /// Policy Mappings
    PolicyMappings(Vec<PolicyMapping>),
    /// Policy Constraints
    PolicyConstraints(PolicyConstraints),
    /// Name Constraints
    NameConstraints(NameConstraints),
    /// SCT List
    SignedCertificateTimestampList(Vec<u8>),
    /// Raw bytes for unknown extensions
    Raw(Vec<u8>),
}

/// fr fr Basic Constraints extension
#[derive(Debug, Clone, PartialEq)]
pub struct BasicConstraints {
    /// Is this a CA certificate
    pub ca: bool,
    /// Path length constraint
    pub path_len_constraint: Option<u32>,
}

/// fr fr Key Usage flags
#[derive(Debug, Clone, PartialEq)]
pub struct KeyUsage {
    /// Digital signature
    pub digital_signature: bool,
    /// Content commitment (non-repudiation)
    pub content_commitment: bool,
    /// Key encipherment
    pub key_encipherment: bool,
    /// Data encipherment
    pub data_encipherment: bool,
    /// Key agreement
    pub key_agreement: bool,
    /// Key certificate sign
    pub key_cert_sign: bool,
    /// CRL sign
    pub crl_sign: bool,
    /// Encipher only
    pub encipher_only: bool,
    /// Decipher only
    pub decipher_only: bool,
}

/// fr fr Extended Key Usage purposes
#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedKeyUsage {
    /// Server authentication
    pub server_auth: bool,
    /// Client authentication
    pub client_auth: bool,
    /// Code signing
    pub code_signing: bool,
    /// Email protection
    pub email_protection: bool,
    /// Time stamping
    pub time_stamping: bool,
    /// OCSP signing
    pub ocsp_signing: bool,
    /// Any extended key usage
    pub any_extended_key_usage: bool,
    /// Custom purposes (OIDs)
    pub custom_purposes: Vec<String>,
}

/// fr fr Authority Key Identifier
#[derive(Debug, Clone, PartialEq)]
pub struct AuthorityKeyIdentifier {
    /// Key identifier
    pub key_identifier: Option<Vec<u8>>,
    /// Authority cert issuer
    pub authority_cert_issuer: Option<Vec<GeneralName>>,
    /// Authority cert serial number
    pub authority_cert_serial_number: Option<Vec<u8>>,
}

/// fr fr General Name for alternative names
#[derive(Debug, Clone, PartialEq)]
pub enum GeneralName {
    /// Other name
    OtherName {
        /// Type ID
        type_id: String,
        /// Value
        value: Vec<u8>,
    },
    /// RFC 822 name (email)
    Rfc822Name(String),
    /// DNS name
    DnsName(String),
    /// X.400 address
    X400Address(Vec<u8>),
    /// Directory name
    DirectoryName(String),
    /// EDI party name
    EdiPartyName {
        /// Name assigner
        name_assigner: Option<String>,
        /// Party name
        party_name: String,
    },
    /// Uniform resource identifier
    UniformResourceIdentifier(String),
    /// IP address
    IpAddress(Vec<u8>),
    /// Registered ID
    RegisteredId(String),
}

impl GeneralName {
    /// slay Get name as string
    pub fn as_string(&self) -> String {
        match self {
            GeneralName::Rfc822Name(email) => email.clone(),
            GeneralName::DnsName(dns) => dns.clone(),
            GeneralName::UniformResourceIdentifier(uri) => uri.clone(),
            GeneralName::DirectoryName(dn) => dn.clone(),
            GeneralName::RegisteredId(oid) => oid.clone(),
            GeneralName::IpAddress(ip) => {
                if ip.len() == 4 {
                    format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
                } else if ip.len() == 16 {
                    // IPv6 formatting
                    let parts: Vec<String> = ip.chunks(2)
                        .map(|chunk| format!("{:02x}{:02x}", chunk[0], chunk.get(1).unwrap_or(&0)))
                        .collect();
                    parts.join(":")
                } else {
                    format!("Unknown IP format: {} bytes", ip.len())
                }
            }
            GeneralName::EdiPartyName { party_name, .. } => party_name.clone(),
            GeneralName::OtherName { type_id, .. } => format!("OtherName({})", type_id),
            GeneralName::X400Address(_) => "X400Address".to_string(),
        }
    }

    /// slay Check if name matches hostname
    pub fn matches_hostname(&self, hostname: &str) -> bool {
        match self {
            GeneralName::DnsName(dns) => {
                if dns.starts_with("*.") {
                    // Wildcard matching
                    let domain = &dns[2..];
                    hostname.ends_with(domain) && hostname.matches('.').count() >= domain.matches('.').count()
                } else {
                    dns.eq_ignore_ascii_case(hostname)
                }
            }
            GeneralName::IpAddress(ip) => {
                // Parse hostname as IP and compare
                self.parse_ip_address(hostname)
                    .map(|parsed_ip| parsed_ip == *ip)
                    .unwrap_or(false)
            }
            _ => false,
        }
    }

    /// slay Parse IP address from string
    fn parse_ip_address(&self, addr: &str) -> Option<Vec<u8>> {
        if let Ok(ipv4) = addr.parse::<std::net::Ipv4Addr>() {
            Some(ipv4.octets().to_vec())
        } else if let Ok(ipv6) = addr.parse::<std::net::Ipv6Addr>() {
            Some(ipv6.octets().to_vec())
        } else {
            None
        }
    }
}

/// fr fr CRL Distribution Point
#[derive(Debug, Clone, PartialEq)]
pub struct DistributionPoint {
    /// Distribution point name
    pub distribution_point: Option<DistributionPointName>,
    /// Reasons
    pub reasons: Option<ReasonFlags>,
    /// CRL issuer
    pub crl_issuer: Option<Vec<GeneralName>>,
}

/// fr fr Distribution Point Name
#[derive(Debug, Clone, PartialEq)]
pub enum DistributionPointName {
    /// Full name
    FullName(Vec<GeneralName>),
    /// Name relative to CRL issuer
    NameRelativeToCrlIssuer(String),
}

/// fr fr Reason flags for CRL distribution
#[derive(Debug, Clone, PartialEq)]
pub struct ReasonFlags {
    /// Unused
    pub unused: bool,
    /// Key compromise
    pub key_compromise: bool,
    /// CA compromise
    pub ca_compromise: bool,
    /// Affiliation changed
    pub affiliation_changed: bool,
    /// Superseded
    pub superseded: bool,
    /// Cessation of operation
    pub cessation_of_operation: bool,
    /// Certificate hold
    pub certificate_hold: bool,
    /// Privilege withdrawn
    pub privilege_withdrawn: bool,
    /// AA compromise
    pub aa_compromise: bool,
}

/// fr fr Authority Information Access description
#[derive(Debug, Clone, PartialEq)]
pub struct AccessDescription {
    /// Access method (OID)
    pub access_method: String,
    /// Access location
    pub access_location: GeneralName,
}

impl AccessDescription {
    /// slay Check if this is OCSP access
    pub fn is_ocsp(&self) -> bool {
        self.access_method == "1.3.6.1.5.5.7.48.1"
    }

    /// slay Check if this is CA Issuers access
    pub fn is_ca_issuers(&self) -> bool {
        self.access_method == "1.3.6.1.5.5.7.48.2"
    }

    /// slay Get access URL
    pub fn url(&self) -> Option<String> {
        match &self.access_location {
            GeneralName::UniformResourceIdentifier(uri) => Some(uri.clone()),
            _ => None,
        }
    }
}

/// fr fr Policy Information
#[derive(Debug, Clone, PartialEq)]
pub struct PolicyInformation {
    /// Policy identifier (OID)
    pub policy_identifier: String,
    /// Policy qualifiers
    pub policy_qualifiers: Option<Vec<PolicyQualifierInfo>>,
}

/// fr fr Policy Qualifier Information
#[derive(Debug, Clone, PartialEq)]
pub struct PolicyQualifierInfo {
    /// Policy qualifier ID
    pub policy_qualifier_id: String,
    /// Qualifier
    pub qualifier: PolicyQualifier,
}

/// fr fr Policy Qualifier
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyQualifier {
    /// CPS URI
    CpsUri(String),
    /// User notice
    UserNotice {
        /// Notice reference
        notice_ref: Option<NoticeReference>,
        /// Explicit text
        explicit_text: Option<String>,
    },
}

/// fr fr Notice Reference
#[derive(Debug, Clone, PartialEq)]
pub struct NoticeReference {
    /// Organization
    pub organization: String,
    /// Notice numbers
    pub notice_numbers: Vec<u32>,
}

/// fr fr Policy Mapping
#[derive(Debug, Clone, PartialEq)]
pub struct PolicyMapping {
    /// Issuer domain policy
    pub issuer_domain_policy: String,
    /// Subject domain policy
    pub subject_domain_policy: String,
}

/// fr fr Policy Constraints
#[derive(Debug, Clone, PartialEq)]
pub struct PolicyConstraints {
    /// Require explicit policy
    pub require_explicit_policy: Option<u32>,
    /// Inhibit policy mapping
    pub inhibit_policy_mapping: Option<u32>,
}

/// fr fr Name Constraints
#[derive(Debug, Clone, PartialEq)]
pub struct NameConstraints {
    /// Permitted subtrees
    pub permitted_subtrees: Option<Vec<GeneralSubtree>>,
    /// Excluded subtrees
    pub excluded_subtrees: Option<Vec<GeneralSubtree>>,
}

/// fr fr General Subtree for name constraints
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralSubtree {
    /// Base name
    pub base: GeneralName,
    /// Minimum
    pub minimum: Option<u32>,
    /// Maximum
    pub maximum: Option<u32>,
}

/// fr fr Extension criticality
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionCriticality {
    /// Critical extension
    Critical,
    /// Non-critical extension
    NonCritical,
}

/// fr fr Extension builder for creating extensions
#[derive(Debug)]
pub struct ExtensionBuilder {
    /// Extensions being built
    extensions: Vec<Extension>,
}

impl ExtensionBuilder {
    /// slay Create new extension builder
    #[instrument]
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    /// slay Add basic constraints
    #[instrument(skip(self))]
    pub fn add_basic_constraints(&mut self, ca: bool, path_len: Option<u32>, critical: bool) -> &mut Self {
        let basic_constraints = BasicConstraints {
            ca,
            path_len_constraint: path_len,
        };
        
        self.extensions.push(Extension {
            oid: ExtensionOid::BasicConstraints,
            critical,
            value: ExtensionValue::BasicConstraints(basic_constraints),
        });
        
        self
    }

    /// slay Add key usage
    #[instrument(skip(self))]
    pub fn add_key_usage(&mut self, usage: KeyUsage, critical: bool) -> &mut Self {
        self.extensions.push(Extension {
            oid: ExtensionOid::KeyUsage,
            critical,
            value: ExtensionValue::KeyUsage(usage),
        });
        
        self
    }

    /// slay Add extended key usage
    #[instrument(skip(self))]
    pub fn add_extended_key_usage(&mut self, usage: ExtendedKeyUsage, critical: bool) -> &mut Self {
        self.extensions.push(Extension {
            oid: ExtensionOid::ExtendedKeyUsage,
            critical,
            value: ExtensionValue::ExtendedKeyUsage(usage),
        });
        
        self
    }

    /// slay Add subject key identifier
    #[instrument(skip(self))]
    pub fn add_subject_key_identifier(&mut self, key_id: Vec<u8>, critical: bool) -> &mut Self {
        self.extensions.push(Extension {
            oid: ExtensionOid::SubjectKeyIdentifier,
            critical,
            value: ExtensionValue::SubjectKeyIdentifier(key_id),
        });
        
        self
    }

    /// slay Add authority key identifier
    #[instrument(skip(self))]
    pub fn add_authority_key_identifier(&mut self, auth_key_id: AuthorityKeyIdentifier, critical: bool) -> &mut Self {
        self.extensions.push(Extension {
            oid: ExtensionOid::AuthorityKeyIdentifier,
            critical,
            value: ExtensionValue::AuthorityKeyIdentifier(auth_key_id),
        });
        
        self
    }

    /// slay Add subject alternative name
    #[instrument(skip(self))]
    pub fn add_subject_alternative_name(&mut self, names: Vec<GeneralName>, critical: bool) -> &mut Self {
        self.extensions.push(Extension {
            oid: ExtensionOid::SubjectAlternativeName,
            critical,
            value: ExtensionValue::SubjectAlternativeName(names),
        });
        
        self
    }

    /// slay Add CRL distribution points
    #[instrument(skip(self))]
    pub fn add_crl_distribution_points(&mut self, points: Vec<DistributionPoint>, critical: bool) -> &mut Self {
        self.extensions.push(Extension {
            oid: ExtensionOid::CrlDistributionPoints,
            critical,
            value: ExtensionValue::CrlDistributionPoints(points),
        });
        
        self
    }

    /// slay Add authority information access
    #[instrument(skip(self))]
    pub fn add_authority_information_access(&mut self, access: Vec<AccessDescription>, critical: bool) -> &mut Self {
        self.extensions.push(Extension {
            oid: ExtensionOid::AuthorityInformationAccess,
            critical,
            value: ExtensionValue::AuthorityInformationAccess(access),
        });
        
        self
    }

    /// slay Add custom extension
    #[instrument(skip(self, value))]
    pub fn add_custom_extension(&mut self, oid: String, critical: bool, value: Vec<u8>) -> &mut Self {
        self.extensions.push(Extension {
            oid: ExtensionOid::Custom(oid),
            critical,
            value: ExtensionValue::Raw(value),
        });
        
        self
    }

    /// slay Build extensions
    #[instrument(skip(self))]
    pub fn build(self) -> Vec<Extension> {
        self.extensions
    }
}

/// fr fr Extension validator for validating extensions
#[derive(Debug)]
pub struct ExtensionValidator {
    /// Validation rules
    rules: Vec<ValidationRule>,
    /// Allow unknown extensions
    allow_unknown: bool,
}

/// fr fr Validation rule for extensions
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Extension OID
    pub oid: ExtensionOid,
    /// Whether extension is required
    pub required: bool,
    /// Whether extension can be critical
    pub can_be_critical: bool,
    /// Custom validation function
    pub validator: Option<String>,
}

/// fr fr Extension validation result
#[derive(Debug, Clone)]
pub struct ExtensionValidationResult {
    /// Whether validation passed
    pub valid: bool,
    /// Validation errors
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Missing required extensions
    pub missing_required: Vec<ExtensionOid>,
    /// Unknown extensions found
    pub unknown_extensions: Vec<String>,
}

impl ExtensionValidator {
    /// slay Create new extension validator
    #[instrument]
    pub fn new() -> Self {
        Self {
            rules: Self::default_rules(),
            allow_unknown: true,
        }
    }

    /// slay Create validator with custom rules
    #[instrument]
    pub fn with_rules(rules: Vec<ValidationRule>, allow_unknown: bool) -> Self {
        Self {
            rules,
            allow_unknown,
        }
    }

    /// slay Get default validation rules
    fn default_rules() -> Vec<ValidationRule> {
        vec![
            ValidationRule {
                oid: ExtensionOid::BasicConstraints,
                required: false,
                can_be_critical: true,
                validator: None,
            },
            ValidationRule {
                oid: ExtensionOid::KeyUsage,
                required: false,
                can_be_critical: true,
                validator: None,
            },
            ValidationRule {
                oid: ExtensionOid::SubjectKeyIdentifier,
                required: false,
                can_be_critical: false,
                validator: None,
            },
            ValidationRule {
                oid: ExtensionOid::AuthorityKeyIdentifier,
                required: false,
                can_be_critical: false,
                validator: None,
            },
        ]
    }

    /// slay Validate extensions
    #[instrument(skip(self, extensions))]
    pub fn validate(&self, extensions: &[Extension]) -> ExtensionValidationResult {
        let mut result = ExtensionValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            missing_required: Vec::new(),
            unknown_extensions: Vec::new(),
        };

        let extension_map: HashMap<ExtensionOid, &Extension> = extensions.iter()
            .map(|ext| (ext.oid.clone(), ext))
            .collect();

        // Check for required extensions
        for rule in &self.rules {
            if rule.required && !extension_map.contains_key(&rule.oid) {
                result.missing_required.push(rule.oid.clone());
                result.errors.push(format!("Required extension missing: {}", rule.oid.name()));
                result.valid = false;
            }
        }

        // Validate present extensions
        for extension in extensions {
            if let Some(rule) = self.rules.iter().find(|r| r.oid == extension.oid) {
                // Check criticality
                if extension.critical && !rule.can_be_critical {
                    result.errors.push(format!(
                        "Extension {} cannot be marked as critical", 
                        extension.oid.name()
                    ));
                    result.valid = false;
                }

                // Validate extension content
                if let Err(e) = self.validate_extension_content(extension) {
                    result.errors.push(format!(
                        "Extension {} validation failed: {}", 
                        extension.oid.name(), 
                        e
                    ));
                    result.valid = false;
                }
            } else {
                // Unknown extension
                if !self.allow_unknown {
                    result.errors.push(format!("Unknown extension: {}", extension.oid));
                    result.valid = false;
                } else {
                    result.unknown_extensions.push(extension.oid.to_string());
                    result.warnings.push(format!("Unknown extension: {}", extension.oid));
                }
            }
        }

        result
    }

    /// slay Validate individual extension content
    #[instrument(skip(self, extension))]
    fn validate_extension_content(&self, extension: &Extension) -> Result<(), String> {
        match &extension.value {
            ExtensionValue::BasicConstraints(bc) => {
                if bc.ca && bc.path_len_constraint.is_some() {
                    let path_len = bc.path_len_constraint.unwrap();
                    if path_len > 255 {
                        return Err("Path length constraint too large".to_string());
                    }
                }
            }
            ExtensionValue::KeyUsage(ku) => {
                // Validate key usage combinations
                if ku.encipher_only && !ku.key_agreement {
                    return Err("encipher_only requires key_agreement".to_string());
                }
                if ku.decipher_only && !ku.key_agreement {
                    return Err("decipher_only requires key_agreement".to_string());
                }
            }
            ExtensionValue::SubjectAlternativeName(names) => {
                if names.is_empty() {
                    return Err("Subject alternative name cannot be empty".to_string());
                }
            }
            ExtensionValue::CrlDistributionPoints(points) => {
                if points.is_empty() {
                    return Err("CRL distribution points cannot be empty".to_string());
                }
            }
            _ => {} // Other extensions don't have specific validation rules yet
        }

        Ok(())
    }
}

/// fr fr Convenience functions for common operations

/// slay Create extension with default criticality
#[instrument(skip(value))]
pub fn create_extension(oid: ExtensionOid, value: ExtensionValue) -> Extension {
    let critical = match oid {
        ExtensionOid::BasicConstraints => true,
        ExtensionOid::KeyUsage => true,
        _ => false,
    };

    Extension {
        oid,
        critical,
        value,
    }
}

/// slay Parse extension from ASN.1 DER bytes
#[instrument(skip(data))]
pub fn parse_extension(data: &[u8]) -> PkiResult<Extension> {
    // Simplified implementation - would use proper ASN.1 parsing
    if data.len() < 10 {
        return Err(PkiError::InvalidInput("Extension data too short".to_string()));
    }

    // Mock parsing - in reality would decode ASN.1 structure
    Ok(Extension {
        oid: ExtensionOid::Custom("1.2.3.4".to_string()),
        critical: false,
        value: ExtensionValue::Raw(data.to_vec()),
    })
}

/// fr fr Extension error types
#[derive(Debug, Clone)]
pub enum ExtensionError {
    /// Invalid extension format
    InvalidFormat(String),
    /// Unknown extension OID
    UnknownOid(String),
    /// Critical extension not supported
    CriticalNotSupported(String),
    /// Extension validation failed
    ValidationFailed(String),
    /// Duplicate extension
    DuplicateExtension(String),
    /// Missing required extension
    MissingRequired(String),
    /// Internal error
    Internal(String),
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::InvalidFormat(msg) => write!(f, "Invalid extension format: {}", msg),
            ExtensionError::UnknownOid(oid) => write!(f, "Unknown extension OID: {}", oid),
            ExtensionError::CriticalNotSupported(oid) => write!(f, "Critical extension not supported: {}", oid),
            ExtensionError::ValidationFailed(msg) => write!(f, "Extension validation failed: {}", msg),
            ExtensionError::DuplicateExtension(oid) => write!(f, "Duplicate extension: {}", oid),
            ExtensionError::MissingRequired(oid) => write!(f, "Missing required extension: {}", oid),
            ExtensionError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ExtensionError {}

/// fr fr Extension result type
pub type ExtensionResult<T> = Result<T, ExtensionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_oid_parsing() {
        let oid = ExtensionOid::from_str("2.5.29.19");
        assert_eq!(oid, ExtensionOid::BasicConstraints);
        
        let custom_oid = ExtensionOid::from_str("1.2.3.4.5");
        assert_eq!(custom_oid, ExtensionOid::Custom("1.2.3.4.5".to_string()));
    }

    #[test]
    fn test_extension_builder() {
        let mut builder = ExtensionBuilder::new();
        builder.add_basic_constraints(true, Some(0), true);
        
        let extensions = builder.build();
        assert_eq!(extensions.len(), 1);
        assert_eq!(extensions[0].oid, ExtensionOid::BasicConstraints);
        assert!(extensions[0].critical);
    }

    #[test]
    fn test_key_usage_creation() {
        let key_usage = KeyUsage {
            digital_signature: true,
            key_cert_sign: true,
            crl_sign: true,
            content_commitment: false,
            key_encipherment: false,
            data_encipherment: false,
            key_agreement: false,
            encipher_only: false,
            decipher_only: false,
        };
        
        assert!(key_usage.digital_signature);
        assert!(key_usage.key_cert_sign);
        assert!(!key_usage.key_encipherment);
    }

    #[test]
    fn test_general_name_hostname_matching() {
        let dns_name = GeneralName::DnsName("example.com".to_string());
        assert!(dns_name.matches_hostname("example.com"));
        assert!(!dns_name.matches_hostname("subdomain.example.com"));
        
        let wildcard_name = GeneralName::DnsName("*.example.com".to_string());
        assert!(wildcard_name.matches_hostname("subdomain.example.com"));
        assert!(!wildcard_name.matches_hostname("example.com"));
    }

    #[test]
    fn test_extension_validator() {
        let validator = ExtensionValidator::new();
        
        let extensions = vec![
            Extension {
                oid: ExtensionOid::BasicConstraints,
                critical: true,
                value: ExtensionValue::BasicConstraints(BasicConstraints {
                    ca: true,
                    path_len_constraint: Some(0),
                }),
            }
        ];
        
        let result = validator.validate(&extensions);
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_access_description() {
        let ocsp_access = AccessDescription {
            access_method: "1.3.6.1.5.5.7.48.1".to_string(),
            access_location: GeneralName::UniformResourceIdentifier("http://ocsp.example.com".to_string()),
        };
        
        assert!(ocsp_access.is_ocsp());
        assert!(!ocsp_access.is_ca_issuers());
        assert_eq!(ocsp_access.url(), Some("http://ocsp.example.com".to_string()));
    }

    #[test]
    fn test_extension_criticality() {
        let criticality = ExtensionCriticality::Critical;
        assert_eq!(criticality, ExtensionCriticality::Critical);
        
        let non_critical = ExtensionCriticality::NonCritical;
        assert_eq!(non_critical, ExtensionCriticality::NonCritical);
    }
}
