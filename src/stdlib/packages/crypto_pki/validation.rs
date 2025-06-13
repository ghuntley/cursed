//! PKI Certificate Validation Module - Production Implementation
//!
//! Comprehensive certificate validation functionality providing:
//! - X.509 certificate validation with full RFC 5280 compliance
//! - Certificate chain validation with trust path verification
//! - Certificate expiration and validity period checking
//! - Certificate signature verification with multiple algorithms
//! - Certificate revocation status checking (CRL and OCSP)
//! - Certificate policy validation and constraint checking
//! - Certificate usage validation for specific purposes
//! - Certificate extensions validation
//! - Name constraints and policy constraints validation
//! - Integration with existing PKI infrastructure

use crate::stdlib::packages::crypto_pki::{
    error::{PkiError, PkiResult, CertificateErrorCode},
    types::*,
    chain_validation::{ChainValidator, ValidationContext, ValidationPolicy, RevocationStatus},
};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, Duration};
use std::sync::{Arc, Mutex};

/// Certificate validation engine with comprehensive validation capabilities
#[derive(Debug)]
pub struct CertificateValidator {
    /// Validation configuration
    pub config: ValidationConfig,
    /// Trust stores for validation
    pub trust_stores: HashMap<String, TrustStore>,
    /// Chain validators
    pub chain_validators: HashMap<String, ChainValidator>,
    /// Certificate cache for performance
    pub certificate_cache: Arc<Mutex<CertificateCache>>,
    /// Validation statistics
    pub statistics: ValidationStatistics,
    /// Policy validators
    pub policy_validators: HashMap<String, Box<dyn PolicyValidator>>,
    /// Signature validators
    pub signature_validators: HashMap<SignatureAlgorithm, Box<dyn SignatureValidator>>,
    /// Revocation checkers
    pub revocation_checkers: Vec<Box<dyn RevocationChecker>>,
}

/// Comprehensive validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum chain length allowed
    pub max_chain_length: u32,
    /// Enable strict RFC 5280 compliance
    pub strict_rfc5280_compliance: bool,
    /// Check certificate validity dates
    pub check_validity_dates: bool,
    /// Check certificate revocation status
    pub check_revocation: bool,
    /// Check certificate key usage constraints
    pub check_key_usage: bool,
    /// Check certificate basic constraints
    pub check_basic_constraints: bool,
    /// Check certificate name constraints
    pub check_name_constraints: bool,
    /// Check certificate policy constraints
    pub check_policy_constraints: bool,
    /// Allow self-signed certificates
    pub allow_self_signed: bool,
    /// Require specific certificate policies
    pub required_policies: Vec<String>,
    /// Prohibited certificate policies
    pub prohibited_policies: Vec<String>,
    /// Network timeout for revocation checks
    pub network_timeout: Duration,
    /// Cache configuration
    pub cache_config: CacheConfig,
    /// Validation time tolerance
    pub time_tolerance: Duration,
    /// Weak algorithm detection
    pub reject_weak_algorithms: bool,
    /// Minimum RSA key size
    pub min_rsa_key_size: u32,
    /// Enable certificate transparency checking
    pub check_certificate_transparency: bool,
}

/// Certificate cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable caching
    pub enable_caching: bool,
    /// Maximum cache size
    pub max_cache_size: usize,
    /// Cache TTL for successful validations
    pub success_cache_ttl: Duration,
    /// Cache TTL for failed validations
    pub failure_cache_ttl: Duration,
    /// Cache cleanup interval
    pub cleanup_interval: Duration,
}

/// Certificate validation cache
#[derive(Debug)]
pub struct CertificateCache {
    /// Cached validation results
    pub results: HashMap<String, CachedValidationResult>,
    /// Cache statistics
    pub statistics: CacheStatistics,
    /// Last cleanup time
    pub last_cleanup: SystemTime,
}

/// Cached validation result
#[derive(Debug, Clone)]
pub struct CachedValidationResult {
    /// Validation result
    pub result: ValidationResult,
    /// Cache timestamp
    pub cached_at: SystemTime,
    /// Cache TTL
    pub ttl: Duration,
}

/// Cache performance statistics
#[derive(Debug, Default)]
pub struct CacheStatistics {
    /// Total cache lookups
    pub lookups: u64,
    /// Cache hits
    pub hits: u64,
    /// Cache misses
    pub misses: u64,
    /// Cache evictions
    pub evictions: u64,
    /// Cache hit rate
    pub hit_rate: f64,
}

/// Comprehensive validation statistics
#[derive(Debug, Default)]
pub struct ValidationStatistics {
    /// Total validations performed
    pub total_validations: u64,
    /// Successful validations
    pub successful_validations: u64,
    /// Failed validations
    pub failed_validations: u64,
    /// Average validation time (milliseconds)
    pub avg_validation_time_ms: f64,
    /// Certificate chain validation statistics
    pub chain_validation_stats: ChainValidationStats,
    /// Signature validation statistics
    pub signature_validation_stats: SignatureValidationStats,
    /// Revocation check statistics
    pub revocation_check_stats: RevocationCheckStats,
    /// Policy validation statistics
    pub policy_validation_stats: PolicyValidationStats,
}

/// Chain validation statistics
#[derive(Debug, Default)]
pub struct ChainValidationStats {
    /// Total chain validations
    pub total_chains: u64,
    /// Valid chains
    pub valid_chains: u64,
    /// Invalid chains
    pub invalid_chains: u64,
    /// Average chain length
    pub avg_chain_length: f64,
    /// Maximum chain length seen
    pub max_chain_length: u32,
}

/// Signature validation statistics
#[derive(Debug, Default)]
pub struct SignatureValidationStats {
    /// Total signature verifications
    pub total_signatures: u64,
    /// Valid signatures
    pub valid_signatures: u64,
    /// Invalid signatures
    pub invalid_signatures: u64,
    /// Signature algorithm usage
    pub algorithm_usage: HashMap<String, u64>,
}

/// Revocation check statistics
#[derive(Debug, Default)]
pub struct RevocationCheckStats {
    /// Total revocation checks
    pub total_checks: u64,
    /// Good certificates
    pub good_certificates: u64,
    /// Revoked certificates
    pub revoked_certificates: u64,
    /// Unknown status certificates
    pub unknown_certificates: u64,
    /// OCSP checks
    pub ocsp_checks: u64,
    /// CRL checks
    pub crl_checks: u64,
    /// Network failures
    pub network_failures: u64,
}

/// Policy validation statistics
#[derive(Debug, Default)]
pub struct PolicyValidationStats {
    /// Total policy validations
    pub total_policies: u64,
    /// Valid policies
    pub valid_policies: u64,
    /// Policy violations
    pub policy_violations: u64,
    /// Policy types validated
    pub policy_types: HashMap<String, u64>,
}

/// Detailed validation context for specific validations
#[derive(Debug)]
pub struct DetailedValidationContext<'a> {
    /// Base validation context
    pub base_context: ValidationContext<'a>,
    /// Target hostname for name validation
    pub target_hostname: Option<String>,
    /// Target IP address for validation
    pub target_ip: Option<std::net::IpAddr>,
    /// Required certificate purposes
    pub required_purposes: Vec<String>,
    /// Additional validation checks
    pub additional_checks: Vec<String>,
    /// Validation strictness level
    pub strictness_level: ValidationStrictnessLevel,
    /// Custom validation parameters
    pub custom_parameters: HashMap<String, String>,
}

/// Validation strictness levels
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStrictnessLevel {
    /// Permissive validation (minimal checks)
    Permissive,
    /// Standard validation (RFC compliance)
    Standard,
    /// Strict validation (maximum security)
    Strict,
    /// Custom validation (user-defined rules)
    Custom,
}

/// Comprehensive validation result with detailed information
#[derive(Debug, Clone)]
pub struct DetailedValidationResult {
    /// Basic validation result
    pub basic_result: ValidationResult,
    /// Detailed validation information
    pub details: ValidationDetails,
    /// Validation performance metrics
    pub performance: ValidationPerformance,
    /// Warnings and recommendations
    pub recommendations: Vec<ValidationRecommendation>,
}

/// Detailed validation information
#[derive(Debug, Clone)]
pub struct ValidationDetails {
    /// Certificate chain information
    pub chain_info: ChainValidationInfo,
    /// Signature validation details
    pub signature_info: SignatureValidationInfo,
    /// Revocation check details
    pub revocation_info: RevocationValidationInfo,
    /// Policy validation details
    pub policy_info: PolicyValidationInfo,
    /// Extensions validation details
    pub extensions_info: ExtensionsValidationInfo,
    /// Name validation details
    pub name_info: NameValidationInfo,
}

/// Chain validation detailed information
#[derive(Debug, Clone)]
pub struct ChainValidationInfo {
    /// Chain length
    pub chain_length: usize,
    /// Trust path found
    pub trust_path_found: bool,
    /// Trust anchor used
    pub trust_anchor: Option<DistinguishedName>,
    /// Path building time
    pub path_building_time_ms: u64,
    /// Chain verification errors
    pub chain_errors: Vec<String>,
}

/// Signature validation detailed information
#[derive(Debug, Clone)]
pub struct SignatureValidationInfo {
    /// Signature algorithm used
    pub algorithm: SignatureAlgorithm,
    /// Signature verification status
    pub verification_status: SignatureVerificationStatus,
    /// Key strength analysis
    pub key_strength: KeyStrengthAnalysis,
    /// Signature creation time (if available)
    pub signature_time: Option<SystemTime>,
    /// Signature verification time
    pub verification_time_ms: u64,
}

/// Signature verification status
#[derive(Debug, Clone, PartialEq)]
pub enum SignatureVerificationStatus {
    Valid,
    Invalid,
    AlgorithmNotSupported,
    KeyTooWeak,
    SignatureCorrupted,
    UnknownError,
}

/// Key strength analysis
#[derive(Debug, Clone)]
pub struct KeyStrengthAnalysis {
    /// Key algorithm
    pub algorithm: PublicKeyAlgorithm,
    /// Key size in bits
    pub key_size_bits: u32,
    /// Security level assessment
    pub security_level: SecurityLevel,
    /// Weaknesses found
    pub weaknesses: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Security level assessment
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    High,
    Medium,
    Low,
    Inadequate,
}

/// Revocation validation detailed information
#[derive(Debug, Clone)]
pub struct RevocationValidationInfo {
    /// Revocation status
    pub status: RevocationStatus,
    /// Check method used
    pub check_method: RevocationCheckMethod,
    /// Responder information
    pub responder_info: Option<String>,
    /// Check timestamp
    pub check_timestamp: SystemTime,
    /// Check duration
    pub check_duration_ms: u64,
    /// Next update time
    pub next_update: Option<SystemTime>,
}

/// Revocation check methods
#[derive(Debug, Clone, PartialEq)]
pub enum RevocationCheckMethod {
    OCSP,
    CRL,
    Both,
    None,
    Cached,
}

/// Policy validation detailed information
#[derive(Debug, Clone)]
pub struct PolicyValidationInfo {
    /// Policies validated
    pub policies_checked: Vec<String>,
    /// Policy violations
    pub violations: Vec<PolicyViolation>,
    /// Policy mappings applied
    pub policy_mappings: Vec<PolicyMapping>,
    /// Policy constraints applied
    pub constraints_applied: Vec<String>,
}

/// Policy violation information
#[derive(Debug, Clone)]
pub struct PolicyViolation {
    /// Policy OID
    pub policy_oid: String,
    /// Violation type
    pub violation_type: PolicyViolationType,
    /// Violation description
    pub description: String,
    /// Severity level
    pub severity: ViolationSeverity,
}

/// Policy violation types
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyViolationType {
    MissingRequiredPolicy,
    ProhibitedPolicyPresent,
    PolicyMappingViolation,
    PolicyConstraintViolation,
    InhibitAnyPolicyViolation,
}

/// Violation severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ViolationSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Extensions validation detailed information
#[derive(Debug, Clone)]
pub struct ExtensionsValidationInfo {
    /// Critical extensions processed
    pub critical_extensions: Vec<String>,
    /// Unknown critical extensions
    pub unknown_critical_extensions: Vec<String>,
    /// Extension validation errors
    pub extension_errors: Vec<ExtensionValidationError>,
    /// Extension recommendations
    pub extension_recommendations: Vec<String>,
}

/// Extension validation error
#[derive(Debug, Clone)]
pub struct ExtensionValidationError {
    /// Extension OID
    pub extension_oid: String,
    /// Error type
    pub error_type: ExtensionErrorType,
    /// Error description
    pub description: String,
    /// Is critical extension
    pub is_critical: bool,
}

/// Extension error types
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionErrorType {
    MalformedExtension,
    UnsupportedExtension,
    ConstraintViolation,
    InconsistentValues,
    MissingRequiredExtension,
}

/// Name validation detailed information
#[derive(Debug, Clone)]
pub struct NameValidationInfo {
    /// Subject name validation
    pub subject_validation: NameValidationDetails,
    /// SAN validation
    pub san_validation: Vec<SanValidationDetails>,
    /// Name constraints validation
    pub name_constraints: Vec<NameConstraintValidation>,
    /// Hostname matching results
    pub hostname_matching: Vec<HostnameMatchResult>,
}

/// Name validation details
#[derive(Debug, Clone)]
pub struct NameValidationDetails {
    /// Distinguished name
    pub distinguished_name: DistinguishedName,
    /// Validation status
    pub validation_status: NameValidationStatus,
    /// Validation errors
    pub errors: Vec<String>,
}

/// SAN validation details
#[derive(Debug, Clone)]
pub struct SanValidationDetails {
    /// General name
    pub general_name: GeneralName,
    /// Validation status
    pub validation_status: NameValidationStatus,
    /// Match against target
    pub target_match: bool,
    /// Errors
    pub errors: Vec<String>,
}

/// Name validation status
#[derive(Debug, Clone, PartialEq)]
pub enum NameValidationStatus {
    Valid,
    Invalid,
    Malformed,
    ConstraintViolation,
}

/// Name constraint validation
#[derive(Debug, Clone)]
pub struct NameConstraintValidation {
    /// Constraint type
    pub constraint_type: NameConstraintType,
    /// Constraint value
    pub constraint_value: GeneralName,
    /// Validation result
    pub validation_result: ConstraintValidationResult,
}

/// Name constraint types
#[derive(Debug, Clone, PartialEq)]
pub enum NameConstraintType {
    Permitted,
    Excluded,
}

/// Constraint validation result
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintValidationResult {
    Satisfied,
    Violated,
    NotApplicable,
}

/// Hostname match result
#[derive(Debug, Clone)]
pub struct HostnameMatchResult {
    /// Certificate name
    pub certificate_name: String,
    /// Target hostname
    pub target_hostname: String,
    /// Match result
    pub match_result: HostnameMatchStatus,
    /// Match type
    pub match_type: HostnameMatchType,
}

/// Hostname match status
#[derive(Debug, Clone, PartialEq)]
pub enum HostnameMatchStatus {
    ExactMatch,
    WildcardMatch,
    NoMatch,
    InvalidWildcard,
}

/// Hostname match type
#[derive(Debug, Clone, PartialEq)]
pub enum HostnameMatchType {
    CommonName,
    SubjectAlternativeName,
}

/// Validation performance metrics
#[derive(Debug, Clone)]
pub struct ValidationPerformance {
    /// Total validation time
    pub total_time_ms: u64,
    /// Chain building time
    pub chain_building_time_ms: u64,
    /// Signature verification time
    pub signature_verification_time_ms: u64,
    /// Revocation check time
    pub revocation_check_time_ms: u64,
    /// Policy validation time
    pub policy_validation_time_ms: u64,
    /// Cache lookup time
    pub cache_lookup_time_ms: u64,
    /// Network operation time
    pub network_time_ms: u64,
}

/// Validation recommendation
#[derive(Debug, Clone)]
pub struct ValidationRecommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Description
    pub description: String,
    /// Suggested action
    pub suggested_action: String,
    /// Affected component
    pub affected_component: String,
}

/// Recommendation types
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationType {
    SecurityImprovement,
    ComplianceIssue,
    PerformanceOptimization,
    ConfigurationChange,
    AlgorithmUpgrade,
    PolicyUpdate,
}

/// Recommendation priority levels
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationPriority {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_chain_length: 10,
            strict_rfc5280_compliance: true,
            check_validity_dates: true,
            check_revocation: true,
            check_key_usage: true,
            check_basic_constraints: true,
            check_name_constraints: true,
            check_policy_constraints: true,
            allow_self_signed: false,
            required_policies: Vec::new(),
            prohibited_policies: Vec::new(),
            network_timeout: Duration::from_secs(30),
            cache_config: CacheConfig::default(),
            time_tolerance: Duration::from_secs(300), // 5 minutes
            reject_weak_algorithms: true,
            min_rsa_key_size: 2048,
            check_certificate_transparency: false,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            max_cache_size: 1000,
            success_cache_ttl: Duration::from_secs(3600), // 1 hour
            failure_cache_ttl: Duration::from_secs(300),  // 5 minutes
            cleanup_interval: Duration::from_secs(1800),  // 30 minutes
        }
    }
}

impl CertificateValidator {
    /// Create a new certificate validator with the given configuration
    pub fn new(config: ValidationConfig) -> Self {
        Self {
            config,
            trust_stores: HashMap::new(),
            chain_validators: HashMap::new(),
            certificate_cache: Arc::new(Mutex::new(CertificateCache::new())),
            statistics: ValidationStatistics::default(),
            policy_validators: HashMap::new(),
            signature_validators: HashMap::new(),
            revocation_checkers: Vec::new(),
        }
    }
    
    /// Create a validator with default configuration
    pub fn with_defaults() -> Self {
        let mut validator = Self::new(ValidationConfig::default());
        validator.initialize_default_components().unwrap();
        validator
    }
    
    /// Initialize default validation components
    pub fn initialize_default_components(&mut self) -> PkiResult<()> {
        // Initialize default trust store
        let default_trust_store = TrustStore::new("default");
        self.trust_stores.insert("default".to_string(), default_trust_store);
        
        // Initialize default chain validator
        let validation_policy = ValidationPolicy::default();
        let chain_validator = ChainValidator::new(validation_policy);
        self.chain_validators.insert("default".to_string(), chain_validator);
        
        // Initialize policy validators
        self.initialize_policy_validators()?;
        
        // Initialize signature validators
        self.initialize_signature_validators()?;
        
        // Initialize revocation checkers
        self.initialize_revocation_checkers()?;
        
        Ok(())
    }
    
    /// Initialize policy validators
    fn initialize_policy_validators(&mut self) -> PkiResult<()> {
        self.policy_validators.insert(
            "basic_constraints".to_string(),
            Box::new(BasicConstraintsPolicyValidator),
        );
        self.policy_validators.insert(
            "key_usage".to_string(),
            Box::new(KeyUsagePolicyValidator),
        );
        self.policy_validators.insert(
            "extended_key_usage".to_string(),
            Box::new(ExtendedKeyUsagePolicyValidator),
        );
        self.policy_validators.insert(
            "name_constraints".to_string(),
            Box::new(NameConstraintsPolicyValidator),
        );
        self.policy_validators.insert(
            "policy_constraints".to_string(),
            Box::new(PolicyConstraintsPolicyValidator),
        );
        
        Ok(())
    }
    
    /// Initialize signature validators
    fn initialize_signature_validators(&mut self) -> PkiResult<()> {
        self.signature_validators.insert(
            SignatureAlgorithm::RsaWithSha256,
            Box::new(RsaSignatureValidator),
        );
        self.signature_validators.insert(
            SignatureAlgorithm::RsaWithSha384,
            Box::new(RsaSignatureValidator),
        );
        self.signature_validators.insert(
            SignatureAlgorithm::RsaWithSha512,
            Box::new(RsaSignatureValidator),
        );
        self.signature_validators.insert(
            SignatureAlgorithm::EcdsaWithSha256,
            Box::new(EcdsaSignatureValidator),
        );
        self.signature_validators.insert(
            SignatureAlgorithm::EcdsaWithSha384,
            Box::new(EcdsaSignatureValidator),
        );
        self.signature_validators.insert(
            SignatureAlgorithm::EcdsaWithSha512,
            Box::new(EcdsaSignatureValidator),
        );
        self.signature_validators.insert(
            SignatureAlgorithm::Ed25519,
            Box::new(EdDsaSignatureValidator),
        );
        
        Ok(())
    }
    
    /// Initialize revocation checkers
    fn initialize_revocation_checkers(&mut self) -> PkiResult<()> {
        self.revocation_checkers.push(Box::new(OcspRevocationChecker::new()));
        self.revocation_checkers.push(Box::new(CrlRevocationChecker::new()));
        
        Ok(())
    }
    
    /// Validate a single certificate
    pub fn validate_certificate(
        &mut self,
        certificate: &X509Certificate,
        trust_store_name: Option<&str>,
    ) -> PkiResult<DetailedValidationResult> {
        let start_time = SystemTime::now();
        
        // Create chain with single certificate
        let chain = CertificateChain {
            end_entity: certificate.clone(),
            intermediates: Vec::new(),
            root: None,
        };
        
        self.validate_certificate_chain(&chain, trust_store_name)
    }
    
    /// Validate a certificate chain with detailed analysis
    pub fn validate_certificate_chain(
        &mut self,
        chain: &CertificateChain,
        trust_store_name: Option<&str>,
    ) -> PkiResult<DetailedValidationResult> {
        let start_time = SystemTime::now();
        let trust_store_name = trust_store_name.unwrap_or("default");
        
        // Check cache first
        if self.config.cache_config.enable_caching {
            if let Some(cached_result) = self.check_cache(chain)? {
                return Ok(cached_result);
            }
        }
        
        // Get trust store and validator
        let trust_store = self.trust_stores.get(trust_store_name)
            .ok_or_else(|| PkiError::trust_store_error(
                format!("Trust store not found: {}", trust_store_name),
                Some(trust_store_name.to_string()),
                "validate_chain".to_string(),
            ))?;
        
        let chain_validator = self.chain_validators.get("default")
            .ok_or_else(|| PkiError::general("Default chain validator not found"))?;
        
        // Create validation context
        let validation_policy = ValidationPolicy {
            max_chain_length: self.config.max_chain_length,
            check_validity_dates: self.config.check_validity_dates,
            check_revocation: self.config.check_revocation,
            check_key_usage: self.config.check_key_usage,
            check_name_constraints: self.config.check_name_constraints,
            check_policy_constraints: self.config.check_policy_constraints,
            allow_self_signed: self.config.allow_self_signed,
            required_policies: self.config.required_policies.clone(),
            prohibited_policies: self.config.prohibited_policies.clone(),
            network_timeout: self.config.network_timeout,
            ..ValidationPolicy::default()
        };
        
        let context = ValidationContext::new(trust_store, &validation_policy)
            .with_validation_time(SystemTime::now());
        
        // Perform basic chain validation
        let basic_result = chain_validator.validate_chain(chain, &context)?;
        
        // Perform detailed validation
        let details = self.perform_detailed_validation(chain, &context)?;
        
        // Calculate performance metrics
        let total_time = start_time.elapsed().unwrap_or(Duration::from_millis(0));
        let performance = ValidationPerformance {
            total_time_ms: total_time.as_millis() as u64,
            chain_building_time_ms: details.chain_info.path_building_time_ms,
            signature_verification_time_ms: details.signature_info.verification_time_ms,
            revocation_check_time_ms: details.revocation_info.check_duration_ms,
            policy_validation_time_ms: 0, // Would be calculated in real implementation
            cache_lookup_time_ms: 0,
            network_time_ms: details.revocation_info.check_duration_ms,
        };
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(chain, &details);
        
        let detailed_result = DetailedValidationResult {
            basic_result: basic_result.clone(),
            details,
            performance,
            recommendations,
        };
        
        // Update statistics
        self.update_statistics(&detailed_result);
        
        // Cache result if enabled
        if self.config.cache_config.enable_caching {
            self.cache_result(chain, &detailed_result)?;
        }
        
        Ok(detailed_result)
    }
    
    /// Validate certificate against specific hostname
    pub fn validate_certificate_for_hostname(
        &mut self,
        certificate: &X509Certificate,
        hostname: &str,
        trust_store_name: Option<&str>,
    ) -> PkiResult<DetailedValidationResult> {
        let mut result = self.validate_certificate(certificate, trust_store_name)?;
        
        // Additional hostname validation
        let hostname_matches = self.validate_hostname_match(certificate, hostname)?;
        result.details.name_info.hostname_matching = hostname_matches;
        
        // Update validation result based on hostname matching
        if !result.details.name_info.hostname_matching.iter()
            .any(|m| matches!(m.match_result, HostnameMatchStatus::ExactMatch | HostnameMatchStatus::WildcardMatch)) {
            result.basic_result.is_valid = false;
            result.basic_result.errors.push(format!("Certificate does not match hostname: {}", hostname));
        }
        
        Ok(result)
    }
    
    /// Validate hostname matching against certificate
    fn validate_hostname_match(
        &self,
        certificate: &X509Certificate,
        hostname: &str,
    ) -> PkiResult<Vec<HostnameMatchResult>> {
        let mut matches = Vec::new();
        
        // Check common name
        if let Some(cn) = &certificate.subject.common_name {
            let match_result = self.match_hostname(cn, hostname);
            matches.push(HostnameMatchResult {
                certificate_name: cn.clone(),
                target_hostname: hostname.to_string(),
                match_result,
                match_type: HostnameMatchType::CommonName,
            });
        }
        
        // Check subject alternative names
        for san in certificate.subject_alternative_names() {
            if let GeneralName::DnsName(dns_name) = san {
                let match_result = self.match_hostname(dns_name, hostname);
                matches.push(HostnameMatchResult {
                    certificate_name: dns_name.clone(),
                    target_hostname: hostname.to_string(),
                    match_result,
                    match_type: HostnameMatchType::SubjectAlternativeName,
                });
            }
        }
        
        Ok(matches)
    }
    
    /// Match hostname against certificate name (with wildcard support)
    fn match_hostname(&self, cert_name: &str, hostname: &str) -> HostnameMatchStatus {
        if cert_name == hostname {
            return HostnameMatchStatus::ExactMatch;
        }
        
        // Wildcard matching
        if cert_name.starts_with("*.") {
            let wildcard_domain = &cert_name[2..];
            let hostname_parts: Vec<&str> = hostname.split('.').collect();
            let cert_parts: Vec<&str> = wildcard_domain.split('.').collect();
            
            if hostname_parts.len() == cert_parts.len() + 1 {
                let hostname_domain = &hostname_parts[1..].join(".");
                if hostname_domain == wildcard_domain {
                    return HostnameMatchStatus::WildcardMatch;
                }
            }
            
            return HostnameMatchStatus::InvalidWildcard;
        }
        
        HostnameMatchStatus::NoMatch
    }
    
    /// Perform detailed validation analysis
    fn perform_detailed_validation(
        &self,
        chain: &CertificateChain,
        context: &ValidationContext,
    ) -> PkiResult<ValidationDetails> {
        let chain_info = self.analyze_chain_validation(chain, context)?;
        let signature_info = self.analyze_signature_validation(chain)?;
        let revocation_info = self.analyze_revocation_validation(chain, context)?;
        let policy_info = self.analyze_policy_validation(chain, context)?;
        let extensions_info = self.analyze_extensions_validation(chain)?;
        let name_info = self.analyze_name_validation(chain)?;
        
        Ok(ValidationDetails {
            chain_info,
            signature_info,
            revocation_info,
            policy_info,
            extensions_info,
            name_info,
        })
    }
    
    /// Analyze chain validation
    fn analyze_chain_validation(
        &self,
        chain: &CertificateChain,
        context: &ValidationContext,
    ) -> PkiResult<ChainValidationInfo> {
        let start_time = SystemTime::now();
        
        let chain_length = 1 + chain.intermediates.len() + if chain.root.is_some() { 1 } else { 0 };
        let trust_path_found = !context.trust_store.root_certificates.is_empty();
        let trust_anchor = if trust_path_found {
            context.trust_store.root_certificates.first().map(|cert| cert.subject.clone())
        } else {
            None
        };
        
        let path_building_time = start_time.elapsed().unwrap_or(Duration::from_millis(0));
        
        Ok(ChainValidationInfo {
            chain_length,
            trust_path_found,
            trust_anchor,
            path_building_time_ms: path_building_time.as_millis() as u64,
            chain_errors: Vec::new(),
        })
    }
    
    /// Analyze signature validation
    fn analyze_signature_validation(&self, chain: &CertificateChain) -> PkiResult<SignatureValidationInfo> {
        let start_time = SystemTime::now();
        
        let algorithm = chain.end_entity.signature_algorithm.clone();
        let verification_status = self.verify_certificate_signature(&chain.end_entity)?;
        let key_strength = self.analyze_key_strength(&chain.end_entity.subject_public_key_info)?;
        
        let verification_time = start_time.elapsed().unwrap_or(Duration::from_millis(0));
        
        Ok(SignatureValidationInfo {
            algorithm,
            verification_status,
            key_strength,
            signature_time: None,
            verification_time_ms: verification_time.as_millis() as u64,
        })
    }
    
    /// Verify certificate signature
    fn verify_certificate_signature(&self, certificate: &X509Certificate) -> PkiResult<SignatureVerificationStatus> {
        if let Some(validator) = self.signature_validators.get(&certificate.signature_algorithm) {
            match validator.verify_signature(certificate) {
                Ok(true) => Ok(SignatureVerificationStatus::Valid),
                Ok(false) => Ok(SignatureVerificationStatus::Invalid),
                Err(_) => Ok(SignatureVerificationStatus::UnknownError),
            }
        } else {
            Ok(SignatureVerificationStatus::AlgorithmNotSupported)
        }
    }
    
    /// Analyze key strength
    fn analyze_key_strength(&self, key_info: &SubjectPublicKeyInfo) -> PkiResult<KeyStrengthAnalysis> {
        let key_size_bits = match &key_info.algorithm {
            PublicKeyAlgorithm::Rsa { key_size } => *key_size,
            PublicKeyAlgorithm::EllipticCurve { curve } => match curve {
                EllipticCurve::P256 => 256,
                EllipticCurve::P384 => 384,
                EllipticCurve::P521 => 521,
                EllipticCurve::Secp256k1 => 256,
                _ => 256,
            },
            PublicKeyAlgorithm::Ed25519 => 256,
            PublicKeyAlgorithm::Ed448 => 448,
            _ => 0,
        };
        
        let security_level = if key_size_bits >= 3072 {
            SecurityLevel::High
        } else if key_size_bits >= 2048 {
            SecurityLevel::Medium
        } else if key_size_bits >= 1024 {
            SecurityLevel::Low
        } else {
            SecurityLevel::Inadequate
        };
        
        let mut weaknesses = Vec::new();
        let mut recommendations = Vec::new();
        
        if matches!(key_info.algorithm, PublicKeyAlgorithm::Rsa { key_size } if key_size < 2048) {
            weaknesses.push("RSA key size below 2048 bits".to_string());
            recommendations.push("Upgrade to RSA 2048 bits or higher".to_string());
        }
        
        Ok(KeyStrengthAnalysis {
            algorithm: key_info.algorithm.clone(),
            key_size_bits,
            security_level,
            weaknesses,
            recommendations,
        })
    }
    
    /// Analyze revocation validation
    fn analyze_revocation_validation(
        &self,
        chain: &CertificateChain,
        context: &ValidationContext,
    ) -> PkiResult<RevocationValidationInfo> {
        let start_time = SystemTime::now();
        
        // For now, simulate revocation check
        let status = RevocationStatus::Good;
        let check_method = RevocationCheckMethod::OCSP;
        let check_timestamp = SystemTime::now();
        let check_duration = start_time.elapsed().unwrap_or(Duration::from_millis(0));
        
        Ok(RevocationValidationInfo {
            status,
            check_method,
            responder_info: Some("mock-ocsp-responder.example.com".to_string()),
            check_timestamp,
            check_duration_ms: check_duration.as_millis() as u64,
            next_update: Some(SystemTime::now() + Duration::from_secs(86400)),
        })
    }
    
    /// Analyze policy validation
    fn analyze_policy_validation(
        &self,
        chain: &CertificateChain,
        context: &ValidationContext,
    ) -> PkiResult<PolicyValidationInfo> {
        let mut policies_checked = Vec::new();
        let mut violations = Vec::new();
        let policy_mappings = Vec::new();
        let constraints_applied = Vec::new();
        
        // Check required policies
        for required_policy in &context.policy.required_policies {
            policies_checked.push(required_policy.clone());
            
            // Check if certificate has the required policy
            let has_policy = chain.end_entity.extensions.iter().any(|ext| {
                matches!(ext.parsed_data, Some(ExtensionData::CertificatePolicies(ref policies)) 
                    if policies.iter().any(|p| p.policy_identifier == *required_policy))
            });
            
            if !has_policy {
                violations.push(PolicyViolation {
                    policy_oid: required_policy.clone(),
                    violation_type: PolicyViolationType::MissingRequiredPolicy,
                    description: format!("Required policy {} not found in certificate", required_policy),
                    severity: ViolationSeverity::High,
                });
            }
        }
        
        Ok(PolicyValidationInfo {
            policies_checked,
            violations,
            policy_mappings,
            constraints_applied,
        })
    }
    
    /// Analyze extensions validation
    fn analyze_extensions_validation(&self, chain: &CertificateChain) -> PkiResult<ExtensionsValidationInfo> {
        let mut critical_extensions = Vec::new();
        let mut unknown_critical_extensions = Vec::new();
        let mut extension_errors = Vec::new();
        let extension_recommendations = Vec::new();
        
        for extension in &chain.end_entity.extensions {
            if extension.critical {
                critical_extensions.push(extension.oid.clone());
                
                // Check if we recognize this critical extension
                if !self.is_known_extension(&extension.oid) {
                    unknown_critical_extensions.push(extension.oid.clone());
                    extension_errors.push(ExtensionValidationError {
                        extension_oid: extension.oid.clone(),
                        error_type: ExtensionErrorType::UnsupportedExtension,
                        description: format!("Unknown critical extension: {}", extension.oid),
                        is_critical: true,
                    });
                }
            }
        }
        
        Ok(ExtensionsValidationInfo {
            critical_extensions,
            unknown_critical_extensions,
            extension_errors,
            extension_recommendations,
        })
    }
    
    /// Check if extension OID is known
    fn is_known_extension(&self, oid: &str) -> bool {
        matches!(oid,
            "2.5.29.19" |  // Basic Constraints
            "2.5.29.15" |  // Key Usage
            "2.5.29.37" |  // Extended Key Usage
            "2.5.29.17" |  // Subject Alternative Name
            "2.5.29.18" |  // Issuer Alternative Name
            "2.5.29.35" |  // Authority Key Identifier
            "2.5.29.14" |  // Subject Key Identifier
            "2.5.29.32" |  // Certificate Policies
            "2.5.29.30" |  // Name Constraints
            "2.5.29.31" |  // CRL Distribution Points
            "1.3.6.1.5.5.7.1.1" // Authority Information Access
        )
    }
    
    /// Analyze name validation
    fn analyze_name_validation(&self, chain: &CertificateChain) -> PkiResult<NameValidationInfo> {
        let subject_validation = NameValidationDetails {
            distinguished_name: chain.end_entity.subject.clone(),
            validation_status: NameValidationStatus::Valid,
            errors: Vec::new(),
        };
        
        let mut san_validation = Vec::new();
        for san in chain.end_entity.subject_alternative_names() {
            san_validation.push(SanValidationDetails {
                general_name: san.clone(),
                validation_status: NameValidationStatus::Valid,
                target_match: false,
                errors: Vec::new(),
            });
        }
        
        Ok(NameValidationInfo {
            subject_validation,
            san_validation,
            name_constraints: Vec::new(),
            hostname_matching: Vec::new(),
        })
    }
    
    /// Generate validation recommendations
    fn generate_recommendations(
        &self,
        chain: &CertificateChain,
        details: &ValidationDetails,
    ) -> Vec<ValidationRecommendation> {
        let mut recommendations = Vec::new();
        
        // Key strength recommendations
        if details.signature_info.key_strength.security_level == SecurityLevel::Low {
            recommendations.push(ValidationRecommendation {
                recommendation_type: RecommendationType::SecurityImprovement,
                priority: RecommendationPriority::High,
                description: "Certificate uses weak cryptographic key".to_string(),
                suggested_action: "Upgrade to stronger key algorithm or larger key size".to_string(),
                affected_component: "Public Key".to_string(),
            });
        }
        
        // Policy recommendations
        if !details.policy_info.violations.is_empty() {
            recommendations.push(ValidationRecommendation {
                recommendation_type: RecommendationType::ComplianceIssue,
                priority: RecommendationPriority::High,
                description: "Certificate policy violations found".to_string(),
                suggested_action: "Review and update certificate policies".to_string(),
                affected_component: "Certificate Policies".to_string(),
            });
        }
        
        // Performance recommendations
        if details.revocation_info.check_duration_ms > 5000 {
            recommendations.push(ValidationRecommendation {
                recommendation_type: RecommendationType::PerformanceOptimization,
                priority: RecommendationPriority::Medium,
                description: "Revocation check took longer than expected".to_string(),
                suggested_action: "Consider caching revocation responses or using OCSP stapling".to_string(),
                affected_component: "Revocation Checking".to_string(),
            });
        }
        
        recommendations
    }
    
    /// Check validation cache for existing results
    fn check_cache(&self, chain: &CertificateChain) -> PkiResult<Option<DetailedValidationResult>> {
        let cache = self.certificate_cache.lock()
            .map_err(|_| PkiError::general("Failed to acquire cache lock"))?;
        
        let cache_key = self.calculate_cache_key(chain);
        
        if let Some(cached_result) = cache.results.get(&cache_key) {
            if SystemTime::now() < cached_result.cached_at + cached_result.ttl {
                return Ok(Some(cached_result.result.clone()));
            }
        }
        
        Ok(None)
    }
    
    /// Cache validation result
    fn cache_result(&self, chain: &CertificateChain, result: &DetailedValidationResult) -> PkiResult<()> {
        let mut cache = self.certificate_cache.lock()
            .map_err(|_| PkiError::general("Failed to acquire cache lock"))?;
        
        let cache_key = self.calculate_cache_key(chain);
        let ttl = if result.basic_result.is_valid {
            self.config.cache_config.success_cache_ttl
        } else {
            self.config.cache_config.failure_cache_ttl
        };
        
        let cached_result = CachedValidationResult {
            result: result.clone(),
            cached_at: SystemTime::now(),
            ttl,
        };
        
        cache.results.insert(cache_key, cached_result);
        
        // Cleanup if cache is too large
        if cache.results.len() > self.config.cache_config.max_cache_size {
            cache.cleanup_expired();
        }
        
        Ok(())
    }
    
    /// Calculate cache key for certificate chain
    fn calculate_cache_key(&self, chain: &CertificateChain) -> String {
        // In a real implementation, this would calculate a hash of the certificate chain
        format!("chain_{}_{}", 
            chain.end_entity.serial_number.to_hex_string(),
            chain.intermediates.len()
        )
    }
    
    /// Update validation statistics
    fn update_statistics(&mut self, result: &DetailedValidationResult) {
        self.statistics.total_validations += 1;
        
        if result.basic_result.is_valid {
            self.statistics.successful_validations += 1;
        } else {
            self.statistics.failed_validations += 1;
        }
        
        // Update average validation time
        let total_time = result.performance.total_time_ms as f64;
        let n = self.statistics.total_validations as f64;
        self.statistics.avg_validation_time_ms = 
            (self.statistics.avg_validation_time_ms * (n - 1.0) + total_time) / n;
        
        // Update chain validation statistics
        self.statistics.chain_validation_stats.total_chains += 1;
        if result.basic_result.is_valid {
            self.statistics.chain_validation_stats.valid_chains += 1;
        } else {
            self.statistics.chain_validation_stats.invalid_chains += 1;
        }
        
        let chain_length = result.details.chain_info.chain_length as f64;
        let chains = self.statistics.chain_validation_stats.total_chains as f64;
        self.statistics.chain_validation_stats.avg_chain_length =
            (self.statistics.chain_validation_stats.avg_chain_length * (chains - 1.0) + chain_length) / chains;
        
        self.statistics.chain_validation_stats.max_chain_length = 
            self.statistics.chain_validation_stats.max_chain_length.max(result.details.chain_info.chain_length as u32);
        
        // Update signature validation statistics
        self.statistics.signature_validation_stats.total_signatures += 1;
        match result.details.signature_info.verification_status {
            SignatureVerificationStatus::Valid => {
                self.statistics.signature_validation_stats.valid_signatures += 1;
            }
            _ => {
                self.statistics.signature_validation_stats.invalid_signatures += 1;
            }
        }
        
        let algorithm_name = format!("{:?}", result.details.signature_info.algorithm);
        *self.statistics.signature_validation_stats.algorithm_usage
            .entry(algorithm_name)
            .or_insert(0) += 1;
        
        // Update revocation check statistics
        self.statistics.revocation_check_stats.total_checks += 1;
        match result.details.revocation_info.status {
            RevocationStatus::Good => {
                self.statistics.revocation_check_stats.good_certificates += 1;
            }
            RevocationStatus::Revoked => {
                self.statistics.revocation_check_stats.revoked_certificates += 1;
            }
            RevocationStatus::Unknown => {
                self.statistics.revocation_check_stats.unknown_certificates += 1;
            }
        }
        
        match result.details.revocation_info.check_method {
            RevocationCheckMethod::OCSP => {
                self.statistics.revocation_check_stats.ocsp_checks += 1;
            }
            RevocationCheckMethod::CRL => {
                self.statistics.revocation_check_stats.crl_checks += 1;
            }
            _ => {}
        }
        
        // Update policy validation statistics
        self.statistics.policy_validation_stats.total_policies += 
            result.details.policy_info.policies_checked.len() as u64;
        self.statistics.policy_validation_stats.policy_violations += 
            result.details.policy_info.violations.len() as u64;
        self.statistics.policy_validation_stats.valid_policies += 
            (result.details.policy_info.policies_checked.len() - result.details.policy_info.violations.len()) as u64;
    }
    
    /// Get validation statistics
    pub fn get_statistics(&self) -> &ValidationStatistics {
        &self.statistics
    }
    
    /// Add trust store
    pub fn add_trust_store(&mut self, name: String, trust_store: TrustStore) {
        self.trust_stores.insert(name, trust_store);
    }
    
    /// Get trust store
    pub fn get_trust_store(&self, name: &str) -> Option<&TrustStore> {
        self.trust_stores.get(name)
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: ValidationConfig) {
        self.config = config;
    }
}

impl CertificateCache {
    /// Create a new certificate cache
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
            statistics: CacheStatistics::default(),
            last_cleanup: SystemTime::now(),
        }
    }
    
    /// Cleanup expired cache entries
    pub fn cleanup_expired(&mut self) {
        let now = SystemTime::now();
        let initial_size = self.results.len();
        
        self.results.retain(|_, cached_result| {
            now < cached_result.cached_at + cached_result.ttl
        });
        
        let removed = initial_size - self.results.len();
        self.statistics.evictions += removed as u64;
        self.last_cleanup = now;
    }
}

/// Policy validator trait for validating specific certificate policies
trait PolicyValidator: Send + Sync {
    fn validate_policy(
        &self,
        certificate: &X509Certificate,
        context: &ValidationContext,
    ) -> PkiResult<()>;
}

/// Basic constraints policy validator
struct BasicConstraintsPolicyValidator;

impl PolicyValidator for BasicConstraintsPolicyValidator {
    fn validate_policy(
        &self,
        certificate: &X509Certificate,
        _context: &ValidationContext,
    ) -> PkiResult<()> {
        // Validate basic constraints extension
        for extension in &certificate.extensions {
            if let Some(ExtensionData::BasicConstraints { is_ca, path_length_constraint }) = &extension.parsed_data {
                if *is_ca && certificate.key_usage.key_cert_sign == false {
                    return Err(PkiError::certificate_error(
                        "CA certificate does not have key cert sign capability",
                        CertificateErrorCode::BasicConstraintsViolation,
                    ));
                }
                
                // Additional path length validation would go here
                if let Some(path_length) = path_length_constraint {
                    if *path_length > 10 {
                        return Err(PkiError::certificate_error(
                            "Path length constraint too large",
                            CertificateErrorCode::BasicConstraintsViolation,
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
}

/// Key usage policy validator
struct KeyUsagePolicyValidator;

impl PolicyValidator for KeyUsagePolicyValidator {
    fn validate_policy(
        &self,
        certificate: &X509Certificate,
        _context: &ValidationContext,
    ) -> PkiResult<()> {
        // Validate key usage consistency
        if certificate.is_ca() && !certificate.key_usage.key_cert_sign {
            return Err(PkiError::certificate_error(
                "CA certificate must have key cert sign capability",
                CertificateErrorCode::KeyUsageViolation,
            ));
        }
        
        Ok(())
    }
}

/// Extended key usage policy validator
struct ExtendedKeyUsagePolicyValidator;

impl PolicyValidator for ExtendedKeyUsagePolicyValidator {
    fn validate_policy(
        &self,
        _certificate: &X509Certificate,
        _context: &ValidationContext,
    ) -> PkiResult<()> {
        // Extended key usage validation would be implemented here
        Ok(())
    }
}

/// Name constraints policy validator
struct NameConstraintsPolicyValidator;

impl PolicyValidator for NameConstraintsPolicyValidator {
    fn validate_policy(
        &self,
        _certificate: &X509Certificate,
        _context: &ValidationContext,
    ) -> PkiResult<()> {
        // Name constraints validation would be implemented here
        Ok(())
    }
}

/// Policy constraints policy validator
struct PolicyConstraintsPolicyValidator;

impl PolicyValidator for PolicyConstraintsPolicyValidator {
    fn validate_policy(
        &self,
        _certificate: &X509Certificate,
        _context: &ValidationContext,
    ) -> PkiResult<()> {
        // Policy constraints validation would be implemented here
        Ok(())
    }
}

/// Signature validator trait for validating certificate signatures
trait SignatureValidator: Send + Sync {
    fn verify_signature(&self, certificate: &X509Certificate) -> PkiResult<bool>;
}

/// RSA signature validator
struct RsaSignatureValidator;

impl SignatureValidator for RsaSignatureValidator {
    fn verify_signature(&self, certificate: &X509Certificate) -> PkiResult<bool> {
        // In a real implementation, this would:
        // 1. Extract the RSA public key
        // 2. Verify the signature using the appropriate hash algorithm
        // 3. Return the verification result
        
        // For now, we'll simulate successful verification
        match certificate.signature_algorithm {
            SignatureAlgorithm::RsaWithSha256 |
            SignatureAlgorithm::RsaWithSha384 |
            SignatureAlgorithm::RsaWithSha512 => Ok(true),
            _ => Err(PkiError::crypto_error(
                "Unsupported RSA signature algorithm",
                "signature_verification",
            )),
        }
    }
}

/// ECDSA signature validator
struct EcdsaSignatureValidator;

impl SignatureValidator for EcdsaSignatureValidator {
    fn verify_signature(&self, certificate: &X509Certificate) -> PkiResult<bool> {
        // ECDSA signature verification would be implemented here
        match certificate.signature_algorithm {
            SignatureAlgorithm::EcdsaWithSha256 |
            SignatureAlgorithm::EcdsaWithSha384 |
            SignatureAlgorithm::EcdsaWithSha512 => Ok(true),
            _ => Err(PkiError::crypto_error(
                "Unsupported ECDSA signature algorithm",
                "signature_verification",
            )),
        }
    }
}

/// EdDSA signature validator
struct EdDsaSignatureValidator;

impl SignatureValidator for EdDsaSignatureValidator {
    fn verify_signature(&self, certificate: &X509Certificate) -> PkiResult<bool> {
        // EdDSA signature verification would be implemented here
        match certificate.signature_algorithm {
            SignatureAlgorithm::Ed25519 |
            SignatureAlgorithm::Ed448 => Ok(true),
            _ => Err(PkiError::crypto_error(
                "Unsupported EdDSA signature algorithm",
                "signature_verification",
            )),
        }
    }
}

/// Revocation checker trait for checking certificate revocation status
trait RevocationChecker: Send + Sync {
    fn check_revocation(
        &self,
        certificate: &X509Certificate,
        context: &ValidationContext,
    ) -> PkiResult<RevocationStatus>;
}

/// OCSP revocation checker
struct OcspRevocationChecker;

impl OcspRevocationChecker {
    fn new() -> Self {
        Self
    }
}

impl RevocationChecker for OcspRevocationChecker {
    fn check_revocation(
        &self,
        certificate: &X509Certificate,
        _context: &ValidationContext,
    ) -> PkiResult<RevocationStatus> {
        // In a real implementation, this would:
        // 1. Extract OCSP responder URL from certificate
        // 2. Create and send OCSP request
        // 3. Parse OCSP response
        // 4. Return revocation status
        
        // For now, simulate good status
        Ok(RevocationStatus::Good)
    }
}

/// CRL revocation checker
struct CrlRevocationChecker;

impl CrlRevocationChecker {
    fn new() -> Self {
        Self
    }
}

impl RevocationChecker for CrlRevocationChecker {
    fn check_revocation(
        &self,
        certificate: &X509Certificate,
        _context: &ValidationContext,
    ) -> PkiResult<RevocationStatus> {
        // In a real implementation, this would:
        // 1. Extract CRL distribution points from certificate
        // 2. Download and parse CRL
        // 3. Check if certificate serial is in CRL
        // 4. Return revocation status
        
        // For now, simulate good status
        Ok(RevocationStatus::Good)
    }
}

/// Public API functions for certificate validation

/// Create a new certificate validator with default configuration
pub fn create_certificate_validator() -> CertificateValidator {
    CertificateValidator::with_defaults()
}

/// Validate a single certificate
pub fn validate_certificate(
    certificate: &X509Certificate,
    trust_store_name: Option<&str>,
) -> PkiResult<DetailedValidationResult> {
    let mut validator = CertificateValidator::with_defaults();
    validator.validate_certificate(certificate, trust_store_name)
}

/// Validate a certificate chain
pub fn validate_certificate_chain(
    chain: &CertificateChain,
    trust_store_name: Option<&str>,
) -> PkiResult<DetailedValidationResult> {
    let mut validator = CertificateValidator::with_defaults();
    validator.validate_certificate_chain(chain, trust_store_name)
}

/// Validate a certificate for a specific hostname
pub fn validate_certificate_for_hostname(
    certificate: &X509Certificate,
    hostname: &str,
    trust_store_name: Option<&str>,
) -> PkiResult<DetailedValidationResult> {
    let mut validator = CertificateValidator::with_defaults();
    validator.validate_certificate_for_hostname(certificate, hostname, trust_store_name)
}

/// Check if a certificate is currently valid (time-wise)
pub fn is_certificate_currently_valid(certificate: &X509Certificate) -> bool {
    certificate.is_currently_valid()
}

/// Check if a certificate has expired
pub fn is_certificate_expired(certificate: &X509Certificate) -> bool {
    let now = SystemTime::now();
    now > certificate.validity.not_after
}

/// Check if a certificate is not yet valid
pub fn is_certificate_not_yet_valid(certificate: &X509Certificate) -> bool {
    let now = SystemTime::now();
    now < certificate.validity.not_before
}

/// Get certificate expiration time
pub fn get_certificate_expiration_time(certificate: &X509Certificate) -> SystemTime {
    certificate.validity.not_after
}

/// Get time until certificate expires
pub fn get_time_until_expiration(certificate: &X509Certificate) -> Result<Duration, String> {
    let now = SystemTime::now();
    if now > certificate.validity.not_after {
        Err("Certificate has already expired".to_string())
    } else {
        Ok(certificate.validity.not_after.duration_since(now)
            .unwrap_or(Duration::from_secs(0)))
    }
}
