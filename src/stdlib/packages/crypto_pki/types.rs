//! PKI Core Types and Data Structures
//! 
//! Comprehensive type definitions for Public Key Infrastructure operations.

use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};

/// Distinguished Name (DN) for certificate subjects and issuers
#[derive(Debug, Clone, PartialEq)]
pub struct DistinguishedName {
    /// Common Name (CN)
    pub common_name: Option<String>,
    /// Organization (O)
    pub organization: Option<String>,
    /// Organizational Unit (OU)  
    pub organizational_unit: Option<String>,
    /// Country (C)
    pub country: Option<String>,
    /// State or Province (ST)
    pub state_or_province: Option<String>,
    /// Locality (L)
    pub locality: Option<String>,
    /// Email Address
    pub email_address: Option<String>,
    /// Additional attributes as OID-value pairs
    pub additional_attributes: HashMap<String, String>,
}

/// Certificate serial number (can be large integers)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerialNumber {
    /// Serial number as bytes (big-endian)
    pub bytes: Vec<u8>,
}

/// X.509 certificate representation
#[derive(Debug, Clone)]
pub struct X509Certificate {
    /// Certificate version (typically 3 for X.509v3)
    pub version: u8,
    /// Certificate serial number
    pub serial_number: SerialNumber,
    /// Signature algorithm identifier
    pub signature_algorithm: SignatureAlgorithm,
    /// Certificate issuer DN
    pub issuer: DistinguishedName,
    /// Certificate validity period
    pub validity: Validity,
    /// Certificate subject DN
    pub subject: DistinguishedName,
    /// Subject public key information
    pub subject_public_key_info: SubjectPublicKeyInfo,
    /// Certificate extensions (for X.509v3)
    pub extensions: Vec<X509Extension>,
    /// Raw certificate bytes (DER encoded)
    pub raw_data: Vec<u8>,
    /// Certificate fingerprint (SHA-256)
    pub fingerprint: Option<Vec<u8>>,
    /// Certificate purpose/usage flags
    pub key_usage: KeyUsage,
    /// Extended key usage
    pub extended_key_usage: ExtendedKeyUsage,
}

/// Certificate validity period
#[derive(Debug, Clone)]
pub struct Validity {
    /// Certificate not valid before this time
    pub not_before: SystemTime,
    /// Certificate not valid after this time
    pub not_after: SystemTime,
}

/// Subject Public Key Information
#[derive(Debug, Clone)]
pub struct SubjectPublicKeyInfo {
    /// Public key algorithm
    pub algorithm: PublicKeyAlgorithm,
    /// Public key data
    pub public_key: Vec<u8>,
    /// Key parameters (algorithm-specific)
    pub parameters: Option<Vec<u8>>,
}

/// Supported signature algorithms
#[derive(Debug, Clone, PartialEq)]
pub enum SignatureAlgorithm {
    /// RSA with SHA-256
    RsaWithSha256,
    /// RSA with SHA-384
    RsaWithSha384,
    /// RSA with SHA-512
    RsaWithSha512,
    /// ECDSA with SHA-256
    EcdsaWithSha256,
    /// ECDSA with SHA-384
    EcdsaWithSha384,
    /// ECDSA with SHA-512
    EcdsaWithSha512,
    /// Ed25519 signature
    Ed25519,
    /// Ed448 signature
    Ed448,
    /// Custom algorithm with OID
    Custom { oid: String, name: String },
}

/// Supported public key algorithms
#[derive(Debug, Clone, PartialEq)]
pub enum PublicKeyAlgorithm {
    /// RSA public key
    Rsa { key_size: u32 },
    /// Elliptic Curve public key
    EllipticCurve { curve: EllipticCurve },
    /// Ed25519 public key
    Ed25519,
    /// Ed448 public key
    Ed448,
    /// Custom algorithm with OID
    Custom { oid: String, name: String },
}

/// Supported elliptic curves
#[derive(Debug, Clone, PartialEq)]
pub enum EllipticCurve {
    /// NIST P-256 (secp256r1)
    P256,
    /// NIST P-384 (secp384r1)
    P384,
    /// NIST P-521 (secp521r1)
    P521,
    /// secp256k1 (Bitcoin curve)
    Secp256k1,
    /// Custom curve with OID
    Custom { oid: String, name: String },
}

/// X.509 certificate extensions
#[derive(Debug, Clone)]
pub struct X509Extension {
    /// Extension OID
    pub oid: String,
    /// Whether extension is critical
    pub critical: bool,
    /// Extension value (DER encoded)
    pub value: Vec<u8>,
    /// Parsed extension data (if supported)
    pub parsed_data: Option<ExtensionData>,
}

/// Parsed extension data for supported extensions
#[derive(Debug, Clone)]
pub enum ExtensionData {
    /// Basic Constraints (CA:TRUE/FALSE, path length)
    BasicConstraints {
        is_ca: bool,
        path_length_constraint: Option<u32>,
    },
    /// Key Usage flags
    KeyUsage(KeyUsage),
    /// Extended Key Usage OIDs
    ExtendedKeyUsage(ExtendedKeyUsage),
    /// Subject Alternative Names
    SubjectAlternativeName(Vec<GeneralName>),
    /// Issuer Alternative Names
    IssuerAlternativeName(Vec<GeneralName>),
    /// Authority Key Identifier
    AuthorityKeyIdentifier {
        key_identifier: Option<Vec<u8>>,
        authority_cert_issuer: Option<Vec<GeneralName>>,
        authority_cert_serial_number: Option<SerialNumber>,
    },
    /// Subject Key Identifier
    SubjectKeyIdentifier(Vec<u8>),
    /// Certificate Policies
    CertificatePolicies(Vec<PolicyInformation>),
    /// Policy Mappings
    PolicyMappings(Vec<PolicyMapping>),
    /// Name Constraints
    NameConstraints {
        permitted_subtrees: Option<Vec<GeneralSubtree>>,
        excluded_subtrees: Option<Vec<GeneralSubtree>>,
    },
    /// Policy Constraints
    PolicyConstraints {
        require_explicit_policy: Option<u32>,
        inhibit_policy_mapping: Option<u32>,
    },
    /// CRL Distribution Points
    CrlDistributionPoints(Vec<DistributionPoint>),
    /// Authority Information Access
    AuthorityInformationAccess(Vec<AccessDescription>),
    /// Subject Information Access
    SubjectInformationAccess(Vec<AccessDescription>),
    /// Inhibit Any Policy
    InhibitAnyPolicy(u32),
    /// Custom extension data
    Custom(Vec<u8>),
}

/// Certificate key usage flags
#[derive(Debug, Clone, Default)]
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

/// Extended key usage purposes
#[derive(Debug, Clone, Default)]
pub struct ExtendedKeyUsage {
    /// TLS server authentication
    pub server_auth: bool,
    /// TLS client authentication
    pub client_auth: bool,
    /// Code signing
    pub code_signing: bool,
    /// Email protection
    pub email_protection: bool,
    /// Time stamping
    pub time_stamping: bool,
    /// OCSP signing
    pub ocsp_signing: bool,
    /// Custom purposes by OID
    pub custom_purposes: Vec<String>,
}

/// General name types for SAN/IAN extensions
#[derive(Debug, Clone)]
pub enum GeneralName {
    /// DNS name
    DnsName(String),
    /// RFC 822 email address
    Rfc822Name(String),
    /// Uniform Resource Identifier
    UniformResourceIdentifier(String),
    /// IP address (IPv4 or IPv6)
    IpAddress(Vec<u8>),
    /// Distinguished name
    DirectoryName(DistinguishedName),
    /// Registered ID (OID)
    RegisteredId(String),
    /// Other name
    OtherName { type_id: String, value: Vec<u8> },
}

/// Certificate policy information
#[derive(Debug, Clone)]
pub struct PolicyInformation {
    /// Policy identifier (OID)
    pub policy_identifier: String,
    /// Policy qualifiers
    pub policy_qualifiers: Option<Vec<PolicyQualifierInfo>>,
}

/// Policy qualifier information
#[derive(Debug, Clone)]
pub struct PolicyQualifierInfo {
    /// Qualifier ID (OID)
    pub policy_qualifier_id: String,
    /// Qualifier value
    pub qualifier: Vec<u8>,
}

/// Policy mapping
#[derive(Debug, Clone)]
pub struct PolicyMapping {
    /// Issuer domain policy
    pub issuer_domain_policy: String,
    /// Subject domain policy
    pub subject_domain_policy: String,
}

/// General subtree for name constraints
#[derive(Debug, Clone)]
pub struct GeneralSubtree {
    /// Base general name
    pub base: GeneralName,
    /// Minimum distance
    pub minimum: Option<u32>,
    /// Maximum distance
    pub maximum: Option<u32>,
}

/// CRL distribution point
#[derive(Debug, Clone)]
pub struct DistributionPoint {
    /// Distribution point name
    pub distribution_point: Option<DistributionPointName>,
    /// Reason flags
    pub reasons: Option<ReasonFlags>,
    /// CRL issuer
    pub crl_issuer: Option<Vec<GeneralName>>,
}

/// Distribution point name
#[derive(Debug, Clone)]
pub enum DistributionPointName {
    /// Full name
    FullName(Vec<GeneralName>),
    /// Name relative to CRL issuer
    NameRelativeToCrlIssuer(DistinguishedName),
}

/// CRL reason flags
#[derive(Debug, Clone, Default)]
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

/// Authority/Subject Information Access
#[derive(Debug, Clone)]
pub struct AccessDescription {
    /// Access method (OID)
    pub access_method: String,
    /// Access location
    pub access_location: GeneralName,
}

/// Certificate Signing Request (CSR)
#[derive(Debug, Clone)]
pub struct CertificateSigningRequest {
    /// CSR version
    pub version: u8,
    /// Subject distinguished name
    pub subject: DistinguishedName,
    /// Subject public key info
    pub subject_public_key_info: SubjectPublicKeyInfo,
    /// CSR attributes
    pub attributes: Vec<CsrAttribute>,
    /// Signature algorithm
    pub signature_algorithm: SignatureAlgorithm,
    /// Signature value
    pub signature: Vec<u8>,
    /// Raw CSR data (DER encoded)
    pub raw_data: Vec<u8>,
}

/// CSR attribute
#[derive(Debug, Clone)]
pub struct CsrAttribute {
    /// Attribute type (OID)
    pub attribute_type: String,
    /// Attribute values
    pub values: Vec<Vec<u8>>,
}

/// Certificate Revocation List (CRL)
#[derive(Debug, Clone)]
pub struct CertificateRevocationList {
    /// CRL version
    pub version: Option<u8>,
    /// Signature algorithm
    pub signature_algorithm: SignatureAlgorithm,
    /// CRL issuer
    pub issuer: DistinguishedName,
    /// This update time
    pub this_update: SystemTime,
    /// Next update time
    pub next_update: Option<SystemTime>,
    /// Revoked certificates
    pub revoked_certificates: Vec<RevokedCertificate>,
    /// CRL extensions
    pub extensions: Vec<X509Extension>,
    /// Raw CRL data (DER encoded)
    pub raw_data: Vec<u8>,
}

/// Revoked certificate entry in CRL
#[derive(Debug, Clone)]
pub struct RevokedCertificate {
    /// Certificate serial number
    pub serial_number: SerialNumber,
    /// Revocation date
    pub revocation_date: SystemTime,
    /// Revocation reason
    pub reason: Option<RevocationReason>,
    /// Entry extensions
    pub extensions: Vec<X509Extension>,
}

/// Certificate revocation reasons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevocationReason {
    Unspecified = 0,
    KeyCompromise = 1,
    CaCompromise = 2,
    AffiliationChanged = 3,
    Superseded = 4,
    CessationOfOperation = 5,
    CertificateHold = 6,
    // Note: 7 is not used
    RemoveFromCrl = 8,
    PrivilegeWithdrawn = 9,
    AaCompromise = 10,
}

/// OCSP request
#[derive(Debug, Clone)]
pub struct OcspRequest {
    /// OCSP request version
    pub version: u8,
    /// Single requests
    pub single_requests: Vec<SingleRequest>,
    /// Request extensions
    pub extensions: Vec<X509Extension>,
}

/// Single OCSP request
#[derive(Debug, Clone)]
pub struct SingleRequest {
    /// Certificate ID
    pub cert_id: CertId,
    /// Single request extensions
    pub extensions: Vec<X509Extension>,
}

/// Certificate identifier for OCSP
#[derive(Debug, Clone)]
pub struct CertId {
    /// Hash algorithm
    pub hash_algorithm: String,
    /// Issuer name hash
    pub issuer_name_hash: Vec<u8>,
    /// Issuer key hash
    pub issuer_key_hash: Vec<u8>,
    /// Certificate serial number
    pub serial_number: SerialNumber,
}

/// OCSP response
#[derive(Debug, Clone)]
pub struct OcspResponse {
    /// Response status
    pub response_status: OcspResponseStatus,
    /// Response bytes (if successful)
    pub response_bytes: Option<ResponseBytes>,
}

/// OCSP response status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcspResponseStatus {
    Successful = 0,
    MalformedRequest = 1,
    InternalError = 2,
    TryLater = 3,
    SigRequired = 5,
    Unauthorized = 6,
}

/// OCSP response bytes
#[derive(Debug, Clone)]
pub struct ResponseBytes {
    /// Response type
    pub response_type: String,
    /// Response data
    pub response: Vec<u8>,
}

/// Trust store for certificate validation
#[derive(Debug, Clone)]
pub struct TrustStore {
    /// Store name/identifier
    pub name: String,
    /// Trusted root CA certificates
    pub root_certificates: Vec<X509Certificate>,
    /// Intermediate CA certificates
    pub intermediate_certificates: Vec<X509Certificate>,
    /// Trusted certificate fingerprints
    pub trusted_fingerprints: HashMap<Vec<u8>, String>,
    /// Store configuration
    pub config: TrustStoreConfig,
}

/// Trust store configuration
#[derive(Debug, Clone)]
pub struct TrustStoreConfig {
    /// Allow self-signed certificates
    pub allow_self_signed: bool,
    /// Maximum chain length
    pub max_chain_length: u32,
    /// Check certificate validity dates
    pub check_validity_dates: bool,
    /// Check certificate revocation
    pub check_revocation: bool,
    /// OCSP responder URLs
    pub ocsp_responders: Vec<String>,
    /// CRL distribution points
    pub crl_distribution_points: Vec<String>,
    /// Network timeout for revocation checks
    pub network_timeout: Duration,
}

/// Certificate chain for validation
#[derive(Debug, Clone)]
pub struct CertificateChain {
    /// End entity certificate
    pub end_entity: X509Certificate,
    /// Intermediate certificates (ordered from end entity to root)
    pub intermediates: Vec<X509Certificate>,
    /// Root certificate (if included)
    pub root: Option<X509Certificate>,
}

/// Certificate validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the certificate is valid
    pub is_valid: bool,
    /// Validation errors (if any)
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Trust chain used for validation
    pub trust_chain: Option<CertificateChain>,
    /// Validation timestamp
    pub validated_at: SystemTime,
}

/// Implementation methods for core types
impl DistinguishedName {
    /// Create a new empty distinguished name
    pub fn new() -> Self {
        Self {
            common_name: None,
            organization: None,
            organizational_unit: None,
            country: None,
            state_or_province: None,
            locality: None,
            email_address: None,
            additional_attributes: HashMap::new(),
        }
    }
    
    /// Create a distinguished name from a common name
    pub fn from_common_name(cn: impl Into<String>) -> Self {
        let mut dn = Self::new();
        dn.common_name = Some(cn.into());
        dn
    }
    
    /// Convert to string representation (RFC 2253 format)
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(cn) = &self.common_name {
            parts.push(format!("CN={}", cn));
        }
        if let Some(ou) = &self.organizational_unit {
            parts.push(format!("OU={}", ou));
        }
        if let Some(o) = &self.organization {
            parts.push(format!("O={}", o));
        }
        if let Some(l) = &self.locality {
            parts.push(format!("L={}", l));
        }
        if let Some(st) = &self.state_or_province {
            parts.push(format!("ST={}", st));
        }
        if let Some(c) = &self.country {
            parts.push(format!("C={}", c));
        }
        if let Some(email) = &self.email_address {
            parts.push(format!("emailAddress={}", email));
        }
        
        for (oid, value) in &self.additional_attributes {
            parts.push(format!("{}={}", oid, value));
        }
        
        parts.join(", ")
    }
}

impl SerialNumber {
    /// Create a new serial number from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
    
    /// Create a serial number from a big integer
    pub fn from_big_int(value: u64) -> Self {
        Self {
            bytes: value.to_be_bytes().to_vec(),
        }
    }
    
    /// Convert to hexadecimal string
    pub fn to_hex_string(&self) -> String {
        self.bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(":")
    }
    
    /// Convert to big integer (if it fits in u64)
    pub fn to_u64(&self) -> Option<u64> {
        if self.bytes.len() <= 8 {
            let mut padded = vec![0u8; 8];
            let start = 8 - self.bytes.len();
            padded[start..].copy_from_slice(&self.bytes);
            Some(u64::from_be_bytes(padded.try_into().ok()?))
        } else {
            None
        }
    }
}

impl Default for DistinguishedName {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TrustStoreConfig {
    fn default() -> Self {
        Self {
            allow_self_signed: false,
            max_chain_length: 10,
            check_validity_dates: true,
            check_revocation: true,
            ocsp_responders: Vec::new(),
            crl_distribution_points: Vec::new(),
            network_timeout: Duration::from_secs(30),
        }
    }
}

impl X509Certificate {
    /// Check if the certificate is currently valid (time-wise)
    pub fn is_currently_valid(&self) -> bool {
        let now = SystemTime::now();
        now >= self.validity.not_before && now <= self.validity.not_after
    }
    
    /// Check if the certificate is a CA certificate
    pub fn is_ca(&self) -> bool {
        self.extensions.iter().any(|ext| {
            matches!(ext.parsed_data, Some(ExtensionData::BasicConstraints { is_ca: true, .. }))
        })
    }
    
    /// Get the certificate's common name
    pub fn common_name(&self) -> Option<&String> {
        self.subject.common_name.as_ref()
    }
    
    /// Get subject alternative names
    pub fn subject_alternative_names(&self) -> Vec<&GeneralName> {
        self.extensions.iter()
            .filter_map(|ext| {
                if let Some(ExtensionData::SubjectAlternativeName(names)) = &ext.parsed_data {
                    Some(names.iter().collect::<Vec<_>>())
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
}

impl TrustStore {
    /// Create a new trust store
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            root_certificates: Vec::new(),
            intermediate_certificates: Vec::new(),
            trusted_fingerprints: HashMap::new(),
            config: TrustStoreConfig::default(),
        }
    }
    
    /// Add a root CA certificate
    pub fn add_root_certificate(&mut self, cert: X509Certificate) {
        self.root_certificates.push(cert);
    }
    
    /// Add an intermediate CA certificate
    pub fn add_intermediate_certificate(&mut self, cert: X509Certificate) {
        self.intermediate_certificates.push(cert);
    }
    
    /// Check if a certificate is trusted
    pub fn is_trusted(&self, cert: &X509Certificate) -> bool {
        // Check if certificate is in root store
        if self.root_certificates.iter().any(|root| {
            root.serial_number == cert.serial_number && 
            root.issuer.to_string() == cert.issuer.to_string()
        }) {
            return true;
        }
        
        // Check fingerprint
        if let Some(fingerprint) = &cert.fingerprint {
            if self.trusted_fingerprints.contains_key(fingerprint) {
                return true;
            }
        }
        
        false
    }
}
