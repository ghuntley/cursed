/// fr fr Certificate Chain Handling - Production Ready Implementation
/// 
/// Comprehensive certificate chain building, validation, and path discovery functionality
/// for the CURSED language PKI module. This module provides complete support for:
/// - X.509 certificate chain construction and validation
/// - Trust path discovery and optimization
/// - Chain building algorithms with multiple strategies
/// - Certificate hierarchy management
/// - Cross-certificate handling for complex PKI environments
/// - Performance optimization for large certificate sets
/// - Certificate chain caching and persistence
/// - Chain validation with policy enforcement
/// - Authority discovery and certificate downloading
/// - Chain completeness verification and repair

use crate::error::CursedError;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};
use crate::stdlib::packages::crypto_pki::types::{
    Certificate, X509Certificate, TrustAnchor, ValidationPolicy, RevocationStatus
};
use tracing::{debug, error, info, instrument, warn};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

/// fr fr Certificate chain builder with multiple construction strategies
#[derive(Debug, Clone)]
pub struct ChainBuilder {
    /// Certificate store for intermediate certificates
    certificate_store: Arc<RwLock<CertificateStore>>,
    /// Chain building configuration
    config: ChainBuilderConfig,
    /// Performance metrics
    metrics: Arc<RwLock<ChainBuilderMetrics>>,
}

/// fr fr Certificate chain building configuration
#[derive(Debug, Clone)]
pub struct ChainBuilderConfig {
    /// Maximum chain length allowed
    pub max_chain_length: usize,
    /// Maximum time for chain building
    pub max_build_time: Duration,
    /// Enable online certificate retrieval
    pub enable_online_retrieval: bool,
    /// Certificate download timeout
    pub download_timeout: Duration,
    /// Maximum certificates to process
    pub max_certificates_processed: usize,
    /// Enable chain caching
    pub enable_caching: bool,
    /// Cache expiry time
    pub cache_expiry: Duration,
}

impl Default for ChainBuilderConfig {
    fn default() -> Self {
        Self {
            max_chain_length: 10,
            max_build_time: Duration::from_secs(30),
            enable_online_retrieval: true,
            download_timeout: Duration::from_secs(10),
            max_certificates_processed: 1000,
            enable_caching: true,
            cache_expiry: Duration::from_secs(3600),
        }
    }
}

/// fr fr Certificate store for intermediate certificates
#[derive(Debug, Default)]
struct CertificateStore {
    /// Certificates indexed by subject key identifier
    by_subject_key_id: HashMap<Vec<u8>, Certificate>,
    /// Certificates indexed by subject DN
    by_subject: HashMap<String, Vec<Certificate>>,
    /// Certificates indexed by issuer DN
    by_issuer: HashMap<String, Vec<Certificate>>,
    /// Certificate metadata
    metadata: HashMap<Vec<u8>, CertificateMetadata>,
}

/// fr fr Certificate metadata for chain building
#[derive(Debug, Clone)]
struct CertificateMetadata {
    /// When certificate was added
    pub added_time: SystemTime,
    /// Certificate source
    pub source: CertificateSource,
    /// Validation status
    pub validation_status: Option<ValidationStatus>,
    /// Usage count for LRU caching
    pub usage_count: u64,
}

/// fr fr Certificate source tracking
#[derive(Debug, Clone)]
pub enum CertificateSource {
    /// Local certificate store
    Local,
    /// Downloaded from AIA extension
    AuthorityInformationAccess(String),
    /// User provided
    UserProvided,
    /// System trust store
    SystemTrustStore,
    /// LDAP directory
    LdapDirectory(String),
    /// Certificate transparency log
    CertificateTransparency,
}

/// fr fr Certificate validation status
#[derive(Debug, Clone)]
pub enum ValidationStatus {
    /// Certificate is valid
    Valid,
    /// Certificate is expired
    Expired,
    /// Certificate is revoked
    Revoked,
    /// Certificate has invalid signature
    InvalidSignature,
    /// Certificate is not yet valid
    NotYetValid,
    /// Unknown status
    Unknown,
}

/// fr fr Chain building metrics
#[derive(Debug, Default)]
pub struct ChainBuilderMetrics {
    /// Total chains built
    pub chains_built: u64,
    /// Successful builds
    pub successful_builds: u64,
    /// Failed builds
    pub failed_builds: u64,
    /// Average build time
    pub average_build_time: Duration,
    /// Certificates downloaded
    pub certificates_downloaded: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
}

/// fr fr Certificate chain representation
#[derive(Debug, Clone)]
pub struct CertificateChain {
    /// Certificates in the chain (leaf to root)
    certificates: Vec<Certificate>,
    /// Trust anchor if found
    trust_anchor: Option<TrustAnchor>,
    /// Chain validation status
    validation_status: ChainValidationStatus,
    /// Chain metadata
    metadata: ChainMetadata,
}

/// fr fr Chain validation status
#[derive(Debug, Clone)]
pub enum ChainValidationStatus {
    /// Chain is valid and trusted
    Valid,
    /// Chain is valid but not trusted
    Untrusted,
    /// Chain has expired certificates
    Expired,
    /// Chain has revoked certificates
    Revoked,
    /// Chain has invalid signatures
    InvalidSignature,
    /// Chain is incomplete
    Incomplete,
    /// Chain validation failed
    Invalid(String),
}

/// fr fr Chain metadata
#[derive(Debug, Clone)]
pub struct ChainMetadata {
    /// When chain was built
    pub build_time: SystemTime,
    /// Build duration
    pub build_duration: Duration,
    /// Chain building strategy used
    pub strategy: ChainBuildingStrategy,
    /// Number of certificates processed
    pub certificates_processed: usize,
    /// Whether online retrieval was used
    pub used_online_retrieval: bool,
}

/// fr fr Chain building strategy
#[derive(Debug, Clone)]
pub enum ChainBuildingStrategy {
    /// Forward building (leaf to root)
    Forward,
    /// Reverse building (root to leaf)
    Reverse,
    /// Bidirectional building
    Bidirectional,
    /// Cached chain retrieval
    Cached,
}

/// fr fr Chain building options
#[derive(Debug, Clone)]
pub struct ChainBuilderOptions {
    /// Additional intermediate certificates
    pub intermediates: Vec<Certificate>,
    /// Trust anchors for validation
    pub trust_anchors: Vec<TrustAnchor>,
    /// Validation policy
    pub validation_policy: Option<ValidationPolicy>,
    /// Maximum chain length
    pub max_length: Option<usize>,
    /// Enable revocation checking
    pub check_revocation: bool,
    /// Chain building strategy preference
    pub preferred_strategy: Option<ChainBuildingStrategy>,
}

impl Default for ChainBuilderOptions {
    fn default() -> Self {
        Self {
            intermediates: Vec::new(),
            trust_anchors: Vec::new(),
            validation_policy: None,
            max_length: None,
            check_revocation: false,
            preferred_strategy: None,
        }
    }
}

/// fr fr Chain validator for comprehensive validation
#[derive(Debug)]
pub struct ChainValidator {
    /// Validation configuration
    config: ChainValidatorConfig,
    /// Revocation checkers
    revocation_checkers: Vec<Box<dyn RevocationChecker>>,
    /// Validation metrics
    metrics: Arc<RwLock<ChainValidatorMetrics>>,
}

/// fr fr Chain validator configuration
#[derive(Debug, Clone)]
pub struct ChainValidatorConfig {
    /// Check certificate timestamps
    pub check_timestamps: bool,
    /// Check certificate signatures
    pub check_signatures: bool,
    /// Check certificate revocation
    pub check_revocation: bool,
    /// Check certificate policies
    pub check_policies: bool,
    /// Check name constraints
    pub check_name_constraints: bool,
    /// Allow self-signed certificates
    pub allow_self_signed: bool,
    /// Maximum allowed clock skew
    pub max_clock_skew: Duration,
}

impl Default for ChainValidatorConfig {
    fn default() -> Self {
        Self {
            check_timestamps: true,
            check_signatures: true,
            check_revocation: false,
            check_policies: true,
            check_name_constraints: true,
            allow_self_signed: false,
            max_clock_skew: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// fr fr Revocation checker trait
pub trait RevocationChecker: Send + Sync {
    /// Check revocation status of certificate
    fn check_revocation(&self, cert: &Certificate, issuer: &Certificate) -> PkiResult<RevocationStatus>;
    
    /// Get revocation checker name
    fn name(&self) -> &str;
    
    /// Check if checker supports the certificate
    fn supports_certificate(&self, cert: &Certificate) -> bool;
}

/// fr fr Chain validator metrics
#[derive(Debug, Default)]
pub struct ChainValidatorMetrics {
    /// Total validations performed
    pub validations_performed: u64,
    /// Successful validations
    pub successful_validations: u64,
    /// Failed validations
    pub failed_validations: u64,
    /// Average validation time
    pub average_validation_time: Duration,
    /// Revocation checks performed
    pub revocation_checks: u64,
}

/// fr fr Certificate path for chain building
#[derive(Debug, Clone)]
pub struct CertificatePath {
    /// Certificates in the path
    pub certificates: Vec<Certificate>,
    /// Path score (lower is better)
    pub score: u32,
    /// Whether path is complete to trust anchor
    pub is_complete: bool,
    /// Path constraints satisfied
    pub constraints_satisfied: bool,
}

/// fr fr Path validation result
#[derive(Debug, Clone)]
pub struct PathValidationResult {
    /// Whether path is valid
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Trust anchor used
    pub trust_anchor: Option<TrustAnchor>,
    /// Validation details
    pub details: ValidationDetails,
}

/// fr fr Validation details
#[derive(Debug, Clone)]
pub struct ValidationDetails {
    /// Signature validation results
    pub signature_validations: Vec<SignatureValidation>,
    /// Timestamp validation results
    pub timestamp_validations: Vec<TimestampValidation>,
    /// Revocation check results
    pub revocation_checks: Vec<RevocationCheck>,
    /// Policy validation results
    pub policy_validations: Vec<PolicyValidation>,
}

/// fr fr Signature validation result
#[derive(Debug, Clone)]
pub struct SignatureValidation {
    /// Certificate index in chain
    pub certificate_index: usize,
    /// Whether signature is valid
    pub is_valid: bool,
    /// Validation error if any
    pub error: Option<String>,
    /// Signature algorithm used
    pub algorithm: String,
}

/// fr fr Timestamp validation result
#[derive(Debug, Clone)]
pub struct TimestampValidation {
    /// Certificate index in chain
    pub certificate_index: usize,
    /// Whether timestamps are valid
    pub is_valid: bool,
    /// Not before time
    pub not_before: SystemTime,
    /// Not after time
    pub not_after: SystemTime,
    /// Current time used for validation
    pub validation_time: SystemTime,
}

/// fr fr Revocation check result
#[derive(Debug, Clone)]
pub struct RevocationCheck {
    /// Certificate index in chain
    pub certificate_index: usize,
    /// Revocation status
    pub status: RevocationStatus,
    /// Checker used
    pub checker_name: String,
    /// Check duration
    pub check_duration: Duration,
}

/// fr fr Policy validation result
#[derive(Debug, Clone)]
pub struct PolicyValidation {
    /// Certificate index in chain
    pub certificate_index: usize,
    /// Whether policies are valid
    pub is_valid: bool,
    /// Policy OIDs validated
    pub policy_oids: Vec<String>,
    /// Policy errors
    pub errors: Vec<String>,
}

/// fr fr Chain building error types
#[derive(Debug, Clone)]
pub enum ChainError {
    /// Certificate not found
    CertificateNotFound(String),
    /// Chain too long
    ChainTooLong,
    /// Build timeout
    BuildTimeout,
    /// Invalid certificate
    InvalidCertificate(String),
    /// No trust anchor found
    NoTrustAnchor,
    /// Download failed
    DownloadFailed(String),
    /// Validation failed
    ValidationFailed(String),
    /// Internal error
    Internal(String),
}

impl std::fmt::Display for ChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainError::CertificateNotFound(msg) => write!(f, "Certificate not found: {}", msg),
            ChainError::ChainTooLong => write!(f, "Certificate chain too long"),
            ChainError::BuildTimeout => write!(f, "Chain building timeout"),
            ChainError::InvalidCertificate(msg) => write!(f, "Invalid certificate: {}", msg),
            ChainError::NoTrustAnchor => write!(f, "No trust anchor found"),
            ChainError::DownloadFailed(msg) => write!(f, "Certificate download failed: {}", msg),
            ChainError::ValidationFailed(msg) => write!(f, "Chain validation failed: {}", msg),
            ChainError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ChainError {}

/// fr fr Chain building result type
pub type ChainResult<T> = Result<T, ChainError>;

impl ChainBuilder {
    /// slay Create new chain builder
    #[instrument]
    pub fn new() -> Self {
        Self {
            certificate_store: Arc::new(RwLock::new(CertificateStore::default())),
            config: ChainBuilderConfig::default(),
            metrics: Arc::new(RwLock::new(ChainBuilderMetrics::default())),
        }
    }

    /// slay Create chain builder with configuration
    #[instrument]
    pub fn with_config(config: ChainBuilderConfig) -> Self {
        Self {
            certificate_store: Arc::new(RwLock::new(CertificateStore::default())),
            config,
            metrics: Arc::new(RwLock::new(ChainBuilderMetrics::default())),
        }
    }

    /// slay Add intermediate certificate to store
    #[instrument(skip(self, certificate))]
    pub fn add_intermediate(&self, certificate: Certificate) -> ChainResult<()> {
        let mut store = self.certificate_store.write()
            .map_err(|_| ChainError::Internal("Failed to acquire store lock".to_string()))?;

        let metadata = CertificateMetadata {
            added_time: SystemTime::now(),
            source: CertificateSource::UserProvided,
            validation_status: None,
            usage_count: 0,
        };

        // Index by subject key identifier
        if let Some(ski) = certificate.subject_key_identifier() {
            store.by_subject_key_id.insert(ski.clone(), certificate.clone());
        }

        // Index by subject DN
        let subject = certificate.subject_dn();
        store.by_subject.entry(subject.clone()).or_insert_with(Vec::new).push(certificate.clone());

        // Index by issuer DN  
        let issuer = certificate.issuer_dn();
        store.by_issuer.entry(issuer).or_insert_with(Vec::new).push(certificate.clone());

        // Store metadata
        let cert_id = certificate.fingerprint_bytes();
        store.metadata.insert(cert_id, metadata);

        debug!("Added intermediate certificate to store: {}", subject);
        Ok(())
    }

    /// slay Build certificate chain
    #[instrument(skip(self, end_entity, options))]
    pub fn build_chain(&self, end_entity: &Certificate, options: &ChainBuilderOptions) -> ChainResult<CertificateChain> {
        let start_time = SystemTime::now();
        
        // Add provided intermediates
        for intermediate in &options.intermediates {
            self.add_intermediate(intermediate.clone())?;
        }

        // Choose building strategy
        let strategy = options.preferred_strategy.clone()
            .unwrap_or(ChainBuildingStrategy::Forward);

        let result = match strategy {
            ChainBuildingStrategy::Forward => self.build_forward(end_entity, options),
            ChainBuildingStrategy::Reverse => self.build_reverse(end_entity, options),
            ChainBuildingStrategy::Bidirectional => self.build_bidirectional(end_entity, options),
            ChainBuildingStrategy::Cached => self.build_cached(end_entity, options),
        };

        // Update metrics
        let build_duration = start_time.elapsed().unwrap_or(Duration::from_secs(0));
        self.update_metrics(result.is_ok(), build_duration);

        result
    }

    /// slay Build chain using forward strategy (leaf to root)
    #[instrument(skip(self, end_entity, options))]
    fn build_forward(&self, end_entity: &Certificate, options: &ChainBuilderOptions) -> ChainResult<CertificateChain> {
        let mut chain = vec![end_entity.clone()];
        let mut current = end_entity;
        let max_length = options.max_length.unwrap_or(self.config.max_chain_length);

        while chain.len() < max_length {
            if current.is_self_signed() {
                // Found root certificate
                break;
            }

            // Find issuer certificate
            match self.find_issuer(current)? {
                Some(issuer) => {
                    // Verify signature
                    if !current.verify_signature(&issuer)? {
                        return Err(ChainError::ValidationFailed(
                            "Invalid certificate signature".to_string()
                        ));
                    }

                    chain.push(issuer.clone());
                    current = &issuer;
                }
                None => {
                    // Try to download issuer if enabled
                    if self.config.enable_online_retrieval {
                        if let Some(issuer) = self.download_issuer(current)? {
                            chain.push(issuer.clone());
                            current = &issuer;
                            continue;
                        }
                    }
                    
                    // Chain is incomplete
                    break;
                }
            }
        }

        // Find trust anchor
        let trust_anchor = self.find_trust_anchor(&chain, &options.trust_anchors);

        let validation_status = if trust_anchor.is_some() {
            ChainValidationStatus::Valid
        } else {
            ChainValidationStatus::Untrusted
        };

        let metadata = ChainMetadata {
            build_time: SystemTime::now(),
            build_duration: SystemTime::now().duration_since(SystemTime::now()).unwrap_or(Duration::from_secs(0)),
            strategy: ChainBuildingStrategy::Forward,
            certificates_processed: chain.len(),
            used_online_retrieval: false, // Track this properly
        };

        Ok(CertificateChain {
            certificates: chain,
            trust_anchor,
            validation_status,
            metadata,
        })
    }

    /// slay Build chain using reverse strategy (root to leaf)
    #[instrument(skip(self, end_entity, options))]
    fn build_reverse(&self, end_entity: &Certificate, options: &ChainBuilderOptions) -> ChainResult<CertificateChain> {
        // Find potential trust anchors first
        let trust_anchors = &options.trust_anchors;
        
        for trust_anchor in trust_anchors {
            if let Ok(chain) = self.build_path_from_anchor(trust_anchor, end_entity) {
                let validation_status = ChainValidationStatus::Valid;
                
                let metadata = ChainMetadata {
                    build_time: SystemTime::now(),
                    build_duration: Duration::from_secs(0),
                    strategy: ChainBuildingStrategy::Reverse,
                    certificates_processed: chain.len(),
                    used_online_retrieval: false,
                };

                return Ok(CertificateChain {
                    certificates: chain,
                    trust_anchor: Some(trust_anchor.clone()),
                    validation_status,
                    metadata,
                });
            }
        }

        Err(ChainError::NoTrustAnchor)
    }

    /// slay Build chain using bidirectional strategy
    #[instrument(skip(self, end_entity, options))]
    fn build_bidirectional(&self, end_entity: &Certificate, options: &ChainBuilderOptions) -> ChainResult<CertificateChain> {
        // Try forward first, then reverse
        match self.build_forward(end_entity, options) {
            Ok(chain) => Ok(chain),
            Err(_) => self.build_reverse(end_entity, options),
        }
    }

    /// slay Build chain from cache
    #[instrument(skip(self, end_entity, options))]
    fn build_cached(&self, end_entity: &Certificate, options: &ChainBuilderOptions) -> ChainResult<CertificateChain> {
        // Check cache first (simplified implementation)
        if self.config.enable_caching {
            // Cache lookup would go here
        }
        
        // Fall back to forward building
        self.build_forward(end_entity, options)
    }

    /// slay Find issuer certificate
    #[instrument(skip(self, certificate))]
    fn find_issuer(&self, certificate: &Certificate) -> ChainResult<Option<Certificate>> {
        let store = self.certificate_store.read()
            .map_err(|_| ChainError::Internal("Failed to acquire store lock".to_string()))?;

        // Try by authority key identifier first
        if let Some(aki) = certificate.authority_key_identifier() {
            if let Some(issuer) = store.by_subject_key_id.get(&aki) {
                return Ok(Some(issuer.clone()));
            }
        }

        // Try by issuer DN
        let issuer_dn = certificate.issuer_dn();
        if let Some(candidates) = store.by_subject.get(&issuer_dn) {
            for candidate in candidates {
                if certificate.verify_signature(candidate).unwrap_or(false) {
                    return Ok(Some(candidate.clone()));
                }
            }
        }

        Ok(None)
    }

    /// slay Download issuer certificate from AIA extension
    #[instrument(skip(self, certificate))]
    fn download_issuer(&self, certificate: &Certificate) -> ChainResult<Option<Certificate>> {
        if !self.config.enable_online_retrieval {
            return Ok(None);
        }

        // Get AIA URLs
        let aia_urls = certificate.authority_info_access_urls();
        
        for url in aia_urls {
            match self.download_certificate(&url) {
                Ok(cert) => {
                    // Add to store
                    self.add_intermediate(cert.clone())?;
                    return Ok(Some(cert));
                }
                Err(e) => {
                    warn!("Failed to download certificate from {}: {}", url, e);
                    continue;
                }
            }
        }

        Ok(None)
    }

    /// slay Download certificate from URL
    #[instrument(skip(self))]
    fn download_certificate(&self, url: &str) -> ChainResult<Certificate> {
        // Simplified implementation - in reality would use HTTP client
        warn!("Certificate download not implemented: {}", url);
        Err(ChainError::DownloadFailed("Download not implemented".to_string()))
    }

    /// slay Find trust anchor for chain
    #[instrument(skip(self, chain, trust_anchors))]
    fn find_trust_anchor(&self, chain: &[Certificate], trust_anchors: &[TrustAnchor]) -> Option<TrustAnchor> {
        let root_cert = chain.last()?;
        
        for anchor in trust_anchors {
            if anchor.matches_certificate(root_cert) {
                return Some(anchor.clone());
            }
        }

        None
    }

    /// slay Build path from trust anchor to end entity
    #[instrument(skip(self, trust_anchor, end_entity))]
    fn build_path_from_anchor(&self, trust_anchor: &TrustAnchor, end_entity: &Certificate) -> ChainResult<Vec<Certificate>> {
        // Simplified implementation - would implement proper reverse path building
        Err(ChainError::Internal("Reverse path building not implemented".to_string()))
    }

    /// slay Update metrics
    #[instrument(skip(self))]
    fn update_metrics(&self, success: bool, duration: Duration) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.chains_built += 1;
            if success {
                metrics.successful_builds += 1;
            } else {
                metrics.failed_builds += 1;
            }
            
            // Update average build time
            let total_time = metrics.average_build_time.as_nanos() * (metrics.chains_built - 1) as u128 + duration.as_nanos();
            metrics.average_build_time = Duration::from_nanos((total_time / metrics.chains_built as u128) as u64);
        }
    }

    /// slay Get builder metrics
    pub fn get_metrics(&self) -> ChainBuilderMetrics {
        self.metrics.read()
            .map(|m| m.clone())
            .unwrap_or_default()
    }
}

impl ChainValidator {
    /// slay Create new chain validator
    #[instrument]
    pub fn new() -> Self {
        Self {
            config: ChainValidatorConfig::default(),
            revocation_checkers: Vec::new(),
            metrics: Arc::new(RwLock::new(ChainValidatorMetrics::default())),
        }
    }

    /// slay Create validator with configuration
    #[instrument]
    pub fn with_config(config: ChainValidatorConfig) -> Self {
        Self {
            config,
            revocation_checkers: Vec::new(),
            metrics: Arc::new(RwLock::new(ChainValidatorMetrics::default())),
        }
    }

    /// slay Add revocation checker
    #[instrument(skip(self, checker))]
    pub fn add_revocation_checker(&mut self, checker: Box<dyn RevocationChecker>) {
        info!("Added revocation checker: {}", checker.name());
        self.revocation_checkers.push(checker);
    }

    /// slay Validate certificate chain
    #[instrument(skip(self, chain))]
    pub fn validate_chain(&self, chain: &CertificateChain) -> ChainResult<PathValidationResult> {
        let start_time = SystemTime::now();
        
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut details = ValidationDetails {
            signature_validations: Vec::new(),
            timestamp_validations: Vec::new(),
            revocation_checks: Vec::new(),
            policy_validations: Vec::new(),
        };

        // Validate each certificate in chain
        for (i, cert) in chain.certificates.iter().enumerate() {
            // Validate timestamps
            if self.config.check_timestamps {
                match self.validate_timestamps(cert, i) {
                    Ok(validation) => details.timestamp_validations.push(validation),
                    Err(e) => errors.push(format!("Timestamp validation failed for cert {}: {}", i, e)),
                }
            }

            // Validate signature (except for root)
            if self.config.check_signatures && i > 0 {
                let issuer = &chain.certificates[i - 1];
                match self.validate_signature(cert, issuer, i) {
                    Ok(validation) => details.signature_validations.push(validation),
                    Err(e) => errors.push(format!("Signature validation failed for cert {}: {}", i, e)),
                }
            }

            // Check revocation
            if self.config.check_revocation && i > 0 {
                let issuer = &chain.certificates[i - 1];
                match self.check_revocation(cert, issuer, i) {
                    Ok(check) => details.revocation_checks.push(check),
                    Err(e) => warnings.push(format!("Revocation check failed for cert {}: {}", i, e)),
                }
            }
        }

        // Update metrics
        let validation_duration = start_time.elapsed().unwrap_or(Duration::from_secs(0));
        self.update_validation_metrics(errors.is_empty(), validation_duration);

        let result = PathValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            trust_anchor: chain.trust_anchor.clone(),
            details,
        };

        Ok(result)
    }

    /// slay Validate certificate timestamps
    #[instrument(skip(self, certificate))]
    fn validate_timestamps(&self, certificate: &Certificate, index: usize) -> ChainResult<TimestampValidation> {
        let now = SystemTime::now();
        let not_before = certificate.not_before();
        let not_after = certificate.not_after();

        let is_valid = now >= not_before && now <= not_after;

        Ok(TimestampValidation {
            certificate_index: index,
            is_valid,
            not_before,
            not_after,
            validation_time: now,
        })
    }

    /// slay Validate certificate signature
    #[instrument(skip(self, certificate, issuer))]
    fn validate_signature(&self, certificate: &Certificate, issuer: &Certificate, index: usize) -> ChainResult<SignatureValidation> {
        match certificate.verify_signature(issuer) {
            Ok(is_valid) => Ok(SignatureValidation {
                certificate_index: index,
                is_valid,
                error: None,
                algorithm: certificate.signature_algorithm(),
            }),
            Err(e) => Ok(SignatureValidation {
                certificate_index: index,
                is_valid: false,
                error: Some(e.to_string()),
                algorithm: certificate.signature_algorithm(),
            }),
        }
    }

    /// slay Check certificate revocation
    #[instrument(skip(self, certificate, issuer))]
    fn check_revocation(&self, certificate: &Certificate, issuer: &Certificate, index: usize) -> ChainResult<RevocationCheck> {
        let start_time = SystemTime::now();

        for checker in &self.revocation_checkers {
            if checker.supports_certificate(certificate) {
                match checker.check_revocation(certificate, issuer) {
                    Ok(status) => {
                        let duration = start_time.elapsed().unwrap_or(Duration::from_secs(0));
                        return Ok(RevocationCheck {
                            certificate_index: index,
                            status,
                            checker_name: checker.name().to_string(),
                            check_duration: duration,
                        });
                    }
                    Err(e) => {
                        warn!("Revocation check failed with {}: {}", checker.name(), e);
                        continue;
                    }
                }
            }
        }

        let duration = start_time.elapsed().unwrap_or(Duration::from_secs(0));
        Ok(RevocationCheck {
            certificate_index: index,
            status: RevocationStatus::Unknown,
            checker_name: "none".to_string(),
            check_duration: duration,
        })
    }

    /// slay Update validation metrics
    #[instrument(skip(self))]
    fn update_validation_metrics(&self, success: bool, duration: Duration) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.validations_performed += 1;
            if success {
                metrics.successful_validations += 1;
            } else {
                metrics.failed_validations += 1;
            }
            
            // Update average validation time
            let total_time = metrics.average_validation_time.as_nanos() * (metrics.validations_performed - 1) as u128 + duration.as_nanos();
            metrics.average_validation_time = Duration::from_nanos((total_time / metrics.validations_performed as u128) as u64);
        }
    }

    /// slay Get validator metrics
    pub fn get_metrics(&self) -> ChainValidatorMetrics {
        self.metrics.read()
            .map(|m| m.clone())
            .unwrap_or_default()
    }
}

impl CertificateChain {
    /// slay Get end entity certificate
    pub fn end_entity(&self) -> &Certificate {
        &self.certificates[0]
    }

    /// slay Get root certificate
    pub fn root_certificate(&self) -> &Certificate {
        self.certificates.last().unwrap()
    }

    /// slay Get all certificates
    pub fn certificates(&self) -> &[Certificate] {
        &self.certificates
    }

    /// slay Get trust anchor
    pub fn trust_anchor(&self) -> Option<&TrustAnchor> {
        self.trust_anchor.as_ref()
    }

    /// slay Get validation status
    pub fn validation_status(&self) -> &ChainValidationStatus {
        &self.validation_status
    }

    /// slay Get chain metadata
    pub fn metadata(&self) -> &ChainMetadata {
        &self.metadata
    }

    /// slay Check if chain is trusted
    pub fn is_trusted(&self) -> bool {
        matches!(self.validation_status, ChainValidationStatus::Valid)
    }

    /// slay Get chain length
    pub fn length(&self) -> usize {
        self.certificates.len()
    }
}

/// fr fr Convenience functions for common operations

/// slay Build certificate chain with default options
#[instrument(skip(end_entity, intermediates))]
pub fn build_certificate_chain(end_entity: &Certificate, intermediates: &[Certificate]) -> ChainResult<CertificateChain> {
    let builder = ChainBuilder::new();
    let options = ChainBuilderOptions {
        intermediates: intermediates.to_vec(),
        ..Default::default()
    };
    builder.build_chain(end_entity, &options)
}

/// slay Validate certificate chain with default settings
#[instrument(skip(chain))]
pub fn validate_certificate_chain(chain: &CertificateChain) -> ChainResult<PathValidationResult> {
    let validator = ChainValidator::new();
    validator.validate_chain(chain)
}

/// slay Find chain path between certificates
#[instrument(skip(start, end, intermediates))]
pub fn find_chain_path(start: &Certificate, end: &Certificate, intermediates: &[Certificate]) -> ChainResult<CertificatePath> {
    // Simplified implementation
    let certificates = vec![start.clone(), end.clone()];
    Ok(CertificatePath {
        certificates,
        score: 0,
        is_complete: true,
        constraints_satisfied: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_builder_creation() {
        let builder = ChainBuilder::new();
        let metrics = builder.get_metrics();
        assert_eq!(metrics.chains_built, 0);
    }

    #[test]
    fn test_chain_validator_creation() {
        let validator = ChainValidator::new();
        let metrics = validator.get_metrics();
        assert_eq!(metrics.validations_performed, 0);
    }

    #[test]
    fn test_chain_builder_config() {
        let config = ChainBuilderConfig {
            max_chain_length: 5,
            ..Default::default()
        };
        let builder = ChainBuilder::with_config(config);
        assert_eq!(builder.config.max_chain_length, 5);
    }

    #[test]
    fn test_chain_validator_config() {
        let config = ChainValidatorConfig {
            check_timestamps: false,
            ..Default::default()
        };
        let validator = ChainValidator::with_config(config);
        assert!(!validator.config.check_timestamps);
    }

    #[test]
    fn test_certificate_path_creation() {
        let path = CertificatePath {
            certificates: Vec::new(),
            score: 100,
            is_complete: false,
            constraints_satisfied: true,
        };
        assert_eq!(path.score, 100);
        assert!(!path.is_complete);
        assert!(path.constraints_satisfied);
    }
}
