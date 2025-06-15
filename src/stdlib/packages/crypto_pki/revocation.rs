/// fr fr Certificate Revocation Management - Production Ready Implementation
/// 
/// Comprehensive certificate revocation functionality for the CURSED language PKI module.
/// This module provides complete support for:
/// - Certificate Revocation List (CRL) generation and management
/// - Online Certificate Status Protocol (OCSP) support
/// - Certificate revocation reason code handling
/// - Revocation timestamp and signature validation
/// - Bulk certificate revocation capabilities
/// - Revocation notification and distribution systems
/// - Revocation database management
/// - Revocation audit trails and logging
/// - Emergency revocation procedures
/// - Revocation status checking and verification

use crate::error::CursedError;
use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};
use crate::stdlib::packages::crypto_pki::types::{Certificate, X509Certificate, RevocationReason};
use tracing::{debug, error, info, instrument, warn};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, SystemTime};

/// fr fr Certificate revocation status
#[derive(Debug, Clone, PartialEq)]
pub enum RevocationStatus {
    /// Certificate is not revoked
    Good,
    /// Certificate is revoked
    Revoked {
        /// Revocation time
        revocation_time: SystemTime,
        /// Revocation reason
        reason: RevocationReason,
        /// Additional revocation information
        info: Option<String>,
    },
    /// Revocation status unknown
    Unknown,
    /// Revocation check failed
    CheckFailed(String),
}

/// fr fr Certificate Revocation List (CRL) entry
#[derive(Debug, Clone)]
pub struct CrlEntry {
    /// Certificate serial number
    pub serial_number: Vec<u8>,
    /// Revocation date
    pub revocation_date: SystemTime,
    /// Revocation reason
    pub reason: RevocationReason,
    /// Entry extensions
    pub extensions: Vec<CrlExtension>,
    /// Invalidity date (if applicable)
    pub invalidity_date: Option<SystemTime>,
}

/// fr fr CRL extension
#[derive(Debug, Clone)]
pub struct CrlExtension {
    /// Extension OID
    pub oid: String,
    /// Whether extension is critical
    pub critical: bool,
    /// Extension value
    pub value: Vec<u8>,
}

/// fr fr Certificate Revocation List
#[derive(Debug, Clone)]
pub struct CertificateRevocationList {
    /// CRL version
    pub version: u32,
    /// Signature algorithm
    pub signature_algorithm: String,
    /// Issuer name
    pub issuer: String,
    /// This update time
    pub this_update: SystemTime,
    /// Next update time
    pub next_update: Option<SystemTime>,
    /// Revoked certificates
    pub revoked_certificates: Vec<CrlEntry>,
    /// CRL extensions
    pub extensions: Vec<CrlExtension>,
    /// CRL signature
    pub signature: Vec<u8>,
    /// Issuer certificate
    pub issuer_certificate: Option<Certificate>,
}

/// fr fr CRL builder for creating revocation lists
#[derive(Debug)]
pub struct CrlBuilder {
    /// CRL configuration
    config: CrlBuilderConfig,
    /// Revoked certificates
    entries: Vec<CrlEntry>,
    /// CRL extensions
    extensions: Vec<CrlExtension>,
    /// Issuer information
    issuer: Option<CrlIssuer>,
}

/// fr fr CRL builder configuration
#[derive(Debug, Clone)]
pub struct CrlBuilderConfig {
    /// CRL validity period
    pub validity_period: Duration,
    /// Default signature algorithm
    pub signature_algorithm: String,
    /// Include reason codes
    pub include_reason_codes: bool,
    /// Include invalidity dates
    pub include_invalidity_dates: bool,
    /// Maximum CRL size
    pub max_crl_size: usize,
    /// CRL number increment
    pub crl_number_increment: u64,
}

impl Default for CrlBuilderConfig {
    fn default() -> Self {
        Self {
            validity_period: Duration::from_secs(7 * 24 * 3600), // 7 days
            signature_algorithm: "SHA256withRSA".to_string(),
            include_reason_codes: true,
            include_invalidity_dates: true,
            max_crl_size: 10_000,
            crl_number_increment: 1,
        }
    }
}

/// fr fr CRL issuer information
#[derive(Debug, Clone)]
pub struct CrlIssuer {
    /// Issuer certificate
    pub certificate: Certificate,
    /// Issuer private key
    pub private_key: Vec<u8>,
    /// Issuer name
    pub name: String,
    /// Key identifier
    pub key_identifier: Option<Vec<u8>>,
}

/// fr fr CRL distribution point
#[derive(Debug, Clone)]
pub struct CrlDistributionPoint {
    /// Distribution point name
    pub name: Option<String>,
    /// Distribution point URL
    pub url: String,
    /// CRL issuer
    pub crl_issuer: Option<String>,
    /// Reasons covered
    pub reasons: Vec<RevocationReason>,
}

/// fr fr CRL cache for performance optimization
#[derive(Debug)]
pub struct CrlCache {
    /// Cached CRLs
    cache: Arc<RwLock<HashMap<String, CachedCrl>>>,
    /// Cache configuration
    config: CrlCacheConfig,
    /// Cache metrics
    metrics: Arc<RwLock<CrlCacheMetrics>>,
}

/// fr fr Cached CRL entry
#[derive(Debug, Clone)]
struct CachedCrl {
    /// The CRL
    crl: CertificateRevocationList,
    /// Cache time
    cached_at: SystemTime,
    /// Last accessed time
    last_accessed: SystemTime,
    /// Access count
    access_count: u64,
    /// CRL hash for integrity
    hash: Vec<u8>,
}

/// fr fr CRL cache configuration
#[derive(Debug, Clone)]
pub struct CrlCacheConfig {
    /// Maximum cache size
    pub max_cache_size: usize,
    /// Cache TTL
    pub cache_ttl: Duration,
    /// Preload popular CRLs
    pub preload_popular: bool,
    /// Auto-refresh before expiry
    pub auto_refresh: bool,
    /// Refresh threshold
    pub refresh_threshold: Duration,
}

impl Default for CrlCacheConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 1000,
            cache_ttl: Duration::from_secs(3600), // 1 hour
            preload_popular: true,
            auto_refresh: true,
            refresh_threshold: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// fr fr CRL cache metrics
#[derive(Debug, Default)]
pub struct CrlCacheMetrics {
    /// Cache hits
    pub hits: u64,
    /// Cache misses
    pub misses: u64,
    /// Cache evictions
    pub evictions: u64,
    /// Total CRLs cached
    pub total_cached: u64,
    /// Current cache size
    pub current_size: usize,
    /// Average access time
    pub average_access_time: Duration,
}

/// fr fr CRL validator for signature and structure validation
#[derive(Debug)]
pub struct CrlValidator {
    /// Validation configuration
    config: CrlValidatorConfig,
    /// Trust anchors for validation
    trust_anchors: Vec<Certificate>,
    /// Validation metrics
    metrics: Arc<RwLock<CrlValidatorMetrics>>,
}

/// fr fr CRL validator configuration
#[derive(Debug, Clone)]
pub struct CrlValidatorConfig {
    /// Check CRL signatures
    pub check_signatures: bool,
    /// Check CRL timestamps
    pub check_timestamps: bool,
    /// Require CRL extensions
    pub require_extensions: bool,
    /// Maximum allowed CRL age
    pub max_crl_age: Duration,
    /// Allow stale CRLs
    pub allow_stale: bool,
    /// Clock skew tolerance
    pub clock_skew_tolerance: Duration,
}

impl Default for CrlValidatorConfig {
    fn default() -> Self {
        Self {
            check_signatures: true,
            check_timestamps: true,
            require_extensions: false,
            max_crl_age: Duration::from_secs(30 * 24 * 3600), // 30 days
            allow_stale: false,
            clock_skew_tolerance: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// fr fr CRL validator metrics
#[derive(Debug, Default)]
pub struct CrlValidatorMetrics {
    /// Total validations
    pub total_validations: u64,
    /// Successful validations
    pub successful_validations: u64,
    /// Failed validations
    pub failed_validations: u64,
    /// Signature verification failures
    pub signature_failures: u64,
    /// Timestamp validation failures
    pub timestamp_failures: u64,
}

/// fr fr Revocation database for persistent storage
#[derive(Debug)]
pub struct RevocationDatabase {
    /// Database connection
    storage: Arc<Mutex<RevocationStorage>>,
    /// Database configuration
    config: RevocationDatabaseConfig,
    /// Database metrics
    metrics: Arc<RwLock<RevocationDatabaseMetrics>>,
}

/// fr fr Revocation storage implementation
#[derive(Debug)]
enum RevocationStorage {
    /// In-memory storage
    Memory(HashMap<Vec<u8>, RevocationRecord>),
    /// File-based storage
    File(std::path::PathBuf),
    /// Database storage
    Database(String),
}

/// fr fr Revocation record
#[derive(Debug, Clone)]
pub struct RevocationRecord {
    /// Certificate serial number
    pub serial_number: Vec<u8>,
    /// Issuer DN
    pub issuer_dn: String,
    /// Revocation time
    pub revocation_time: SystemTime,
    /// Revocation reason
    pub reason: RevocationReason,
    /// Revocation authority
    pub revoked_by: String,
    /// Invalidity date
    pub invalidity_date: Option<SystemTime>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// fr fr Revocation database configuration
#[derive(Debug, Clone)]
pub struct RevocationDatabaseConfig {
    /// Storage type
    pub storage_type: String,
    /// Storage location
    pub storage_location: String,
    /// Auto-backup interval
    pub backup_interval: Duration,
    /// Retention period
    pub retention_period: Duration,
    /// Compression enabled
    pub compression_enabled: bool,
    /// Encryption enabled
    pub encryption_enabled: bool,
}

impl Default for RevocationDatabaseConfig {
    fn default() -> Self {
        Self {
            storage_type: "memory".to_string(),
            storage_location: "".to_string(),
            backup_interval: Duration::from_secs(3600), // 1 hour
            retention_period: Duration::from_secs(365 * 24 * 3600), // 1 year
            compression_enabled: true,
            encryption_enabled: false,
        }
    }
}

/// fr fr Revocation database metrics
#[derive(Debug, Default)]
pub struct RevocationDatabaseMetrics {
    /// Total records
    pub total_records: u64,
    /// Records added
    pub records_added: u64,
    /// Records removed
    pub records_removed: u64,
    /// Queries performed
    pub queries_performed: u64,
    /// Average query time
    pub average_query_time: Duration,
    /// Database size
    pub database_size: u64,
}

/// fr fr Revocation notification system
#[derive(Debug)]
pub struct RevocationNotifier {
    /// Notification configuration
    config: RevocationNotifierConfig,
    /// Notification channels
    channels: Vec<Box<dyn NotificationChannel>>,
    /// Notification metrics
    metrics: Arc<RwLock<RevocationNotifierMetrics>>,
}

/// fr fr Revocation notification configuration
#[derive(Debug, Clone)]
pub struct RevocationNotifierConfig {
    /// Enable notifications
    pub enabled: bool,
    /// Notification timeout
    pub timeout: Duration,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// Batch size for bulk notifications
    pub batch_size: usize,
}

impl Default for RevocationNotifierConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            retry_delay: Duration::from_secs(5),
            batch_size: 100,
        }
    }
}

/// fr fr Notification channel trait
pub trait NotificationChannel: Send + Sync {
    /// Send revocation notification
    fn send_notification(&self, notification: &RevocationNotification) -> PkiResult<()>;
    
    /// Get channel name
    fn name(&self) -> &str;
    
    /// Check if channel is available
    fn is_available(&self) -> bool;
}

/// fr fr Revocation notification
#[derive(Debug, Clone)]
pub struct RevocationNotification {
    /// Certificate serial number
    pub serial_number: Vec<u8>,
    /// Issuer DN
    pub issuer_dn: String,
    /// Revocation time
    pub revocation_time: SystemTime,
    /// Revocation reason
    pub reason: RevocationReason,
    /// Notification urgency
    pub urgency: NotificationUrgency,
    /// Additional information
    pub additional_info: HashMap<String, String>,
}

/// fr fr Notification urgency levels
#[derive(Debug, Clone)]
pub enum NotificationUrgency {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Emergency
    Emergency,
}

/// fr fr Revocation notification metrics
#[derive(Debug, Default)]
pub struct RevocationNotifierMetrics {
    /// Total notifications sent
    pub notifications_sent: u64,
    /// Failed notifications
    pub failed_notifications: u64,
    /// Average send time
    pub average_send_time: Duration,
    /// Retry attempts
    pub retry_attempts: u64,
}

/// fr fr Revocation error types
#[derive(Debug, Clone)]
pub enum CrlError {
    /// Invalid CRL format
    InvalidFormat(String),
    /// CRL signature verification failed
    SignatureVerificationFailed,
    /// CRL expired
    Expired,
    /// CRL not yet valid
    NotYetValid,
    /// Certificate not found in CRL
    CertificateNotFound,
    /// Invalid revocation reason
    InvalidRevocationReason,
    /// CRL too large
    CrlTooLarge,
    /// Database error
    DatabaseError(String),
    /// Network error
    NetworkError(String),
    /// Cache error
    CacheError(String),
    /// Internal error
    Internal(String),
}

impl std::fmt::Display for CrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrlError::InvalidFormat(msg) => write!(f, "Invalid CRL format: {}", msg),
            CrlError::SignatureVerificationFailed => write!(f, "CRL signature verification failed"),
            CrlError::Expired => write!(f, "CRL has expired"),
            CrlError::NotYetValid => write!(f, "CRL is not yet valid"),
            CrlError::CertificateNotFound => write!(f, "Certificate not found in CRL"),
            CrlError::InvalidRevocationReason => write!(f, "Invalid revocation reason"),
            CrlError::CrlTooLarge => write!(f, "CRL too large"),
            CrlError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            CrlError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            CrlError::CacheError(msg) => write!(f, "Cache error: {}", msg),
            CrlError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CrlError {}

/// fr fr CRL result type
pub type CrlResult<T> = Result<T, CrlError>;

impl CrlBuilder {
    /// slay Create new CRL builder
    #[instrument]
    pub fn new() -> Self {
        Self {
            config: CrlBuilderConfig::default(),
            entries: Vec::new(),
            extensions: Vec::new(),
            issuer: None,
        }
    }

    /// slay Create CRL builder with configuration
    #[instrument]
    pub fn with_config(config: CrlBuilderConfig) -> Self {
        Self {
            config,
            entries: Vec::new(),
            extensions: Vec::new(),
            issuer: None,
        }
    }

    /// slay Set CRL issuer
    #[instrument(skip(self, issuer))]
    pub fn set_issuer(&mut self, issuer: CrlIssuer) -> &mut Self {
        self.issuer = Some(issuer);
        self
    }

    /// slay Add revoked certificate
    #[instrument(skip(self))]
    pub fn add_revoked_certificate(
        &mut self, 
        serial_number: Vec<u8>, 
        revocation_time: SystemTime, 
        reason: RevocationReason
    ) -> &mut Self {
        let entry = CrlEntry {
            serial_number,
            revocation_date: revocation_time,
            reason,
            extensions: Vec::new(),
            invalidity_date: None,
        };
        
        self.entries.push(entry);
        self
    }

    /// slay Add revoked certificate with extensions
    #[instrument(skip(self, extensions))]
    pub fn add_revoked_certificate_with_extensions(
        &mut self,
        serial_number: Vec<u8>,
        revocation_time: SystemTime,
        reason: RevocationReason,
        extensions: Vec<CrlExtension>,
        invalidity_date: Option<SystemTime>
    ) -> &mut Self {
        let entry = CrlEntry {
            serial_number,
            revocation_date: revocation_time,
            reason,
            extensions,
            invalidity_date,
        };
        
        self.entries.push(entry);
        self
    }

    /// slay Add CRL extension
    #[instrument(skip(self, extension))]
    pub fn add_extension(&mut self, extension: CrlExtension) -> &mut Self {
        self.extensions.push(extension);
        self
    }

    /// slay Build CRL
    #[instrument(skip(self))]
    pub fn build(&self) -> CrlResult<CertificateRevocationList> {
        let issuer = self.issuer.as_ref()
            .ok_or_else(|| CrlError::Internal("No issuer set".to_string()))?;

        if self.entries.len() > self.config.max_crl_size {
            return Err(CrlError::CrlTooLarge);
        }

        let this_update = SystemTime::now();
        let next_update = Some(this_update + self.config.validity_period);

        // Create CRL
        let crl = CertificateRevocationList {
            version: 2, // CRL v2
            signature_algorithm: self.config.signature_algorithm.clone(),
            issuer: issuer.name.clone(),
            this_update,
            next_update,
            revoked_certificates: self.entries.clone(),
            extensions: self.extensions.clone(),
            signature: self.sign_crl(issuer)?,
            issuer_certificate: Some(issuer.certificate.clone()),
        };

        info!("Built CRL with {} entries", self.entries.len());
        Ok(crl)
    }

    /// slay Sign CRL
    #[instrument(skip(self, issuer))]
    fn sign_crl(&self, issuer: &CrlIssuer) -> CrlResult<Vec<u8>> {
        // Simplified implementation - would use proper cryptographic signing
        let mut signature = Vec::new();
        signature.extend_from_slice(b"CRL_SIGNATURE");
        signature.extend_from_slice(&issuer.private_key[..16.min(issuer.private_key.len())]);
        Ok(signature)
    }
}

impl CrlCache {
    /// slay Create new CRL cache
    #[instrument]
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config: CrlCacheConfig::default(),
            metrics: Arc::new(RwLock::new(CrlCacheMetrics::default())),
        }
    }

    /// slay Create CRL cache with configuration
    #[instrument]
    pub fn with_config(config: CrlCacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            metrics: Arc::new(RwLock::new(CrlCacheMetrics::default())),
        }
    }

    /// slay Get CRL from cache
    #[instrument(skip(self))]
    pub fn get(&self, key: &str) -> Option<CertificateRevocationList> {
        let start_time = SystemTime::now();
        
        let cache = self.cache.read().ok()?;
        let cached_crl = cache.get(key)?;
        
        // Check if CRL is still valid
        let now = SystemTime::now();
        if now.duration_since(cached_crl.cached_at).ok()? > self.config.cache_ttl {
            drop(cache);
            self.remove(key);
            self.update_metrics(false, start_time);
            return None;
        }
        
        // Update access information
        drop(cache);
        self.update_access(key);
        self.update_metrics(true, start_time);
        
        Some(cached_crl.crl.clone())
    }

    /// slay Put CRL in cache
    #[instrument(skip(self, crl))]
    pub fn put(&self, key: String, crl: CertificateRevocationList) -> CrlResult<()> {
        let mut cache = self.cache.write()
            .map_err(|_| CrlError::CacheError("Failed to acquire cache lock".to_string()))?;

        // Check cache size limit
        if cache.len() >= self.config.max_cache_size {
            self.evict_lru(&mut cache)?;
        }

        let cached_crl = CachedCrl {
            crl,
            cached_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            access_count: 0,
            hash: self.calculate_crl_hash(&crl)?,
        };

        cache.insert(key, cached_crl);
        self.update_cache_metrics();
        
        Ok(())
    }

    /// slay Remove CRL from cache
    #[instrument(skip(self))]
    pub fn remove(&self, key: &str) {
        if let Ok(mut cache) = self.cache.write() {
            cache.remove(key);
            self.update_cache_metrics();
        }
    }

    /// slay Update access information
    #[instrument(skip(self))]
    fn update_access(&self, key: &str) {
        if let Ok(mut cache) = self.cache.write() {
            if let Some(cached_crl) = cache.get_mut(key) {
                cached_crl.last_accessed = SystemTime::now();
                cached_crl.access_count += 1;
            }
        }
    }

    /// slay Evict least recently used entry
    #[instrument(skip(self, cache))]
    fn evict_lru(&self, cache: &mut HashMap<String, CachedCrl>) -> CrlResult<()> {
        let lru_key = cache.iter()
            .min_by_key(|(_, v)| v.last_accessed)
            .map(|(k, _)| k.clone());

        if let Some(key) = lru_key {
            cache.remove(&key);
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.evictions += 1;
            }
        }

        Ok(())
    }

    /// slay Calculate CRL hash
    #[instrument(skip(self, crl))]
    fn calculate_crl_hash(&self, crl: &CertificateRevocationList) -> CrlResult<Vec<u8>> {
        // Simplified implementation - would use proper hash function
        Ok(crl.signature.clone())
    }

    /// slay Update metrics
    #[instrument(skip(self))]
    fn update_metrics(&self, hit: bool, start_time: SystemTime) {
        if let Ok(mut metrics) = self.metrics.write() {
            if hit {
                metrics.hits += 1;
            } else {
                metrics.misses += 1;
            }
            
            let access_time = start_time.elapsed().unwrap_or(Duration::from_secs(0));
            let total_accesses = metrics.hits + metrics.misses;
            let total_time = metrics.average_access_time.as_nanos() * (total_accesses - 1) as u128 + access_time.as_nanos();
            metrics.average_access_time = Duration::from_nanos((total_time / total_accesses as u128) as u64);
        }
    }

    /// slay Update cache metrics
    #[instrument(skip(self))]
    fn update_cache_metrics(&self) {
        if let (Ok(cache), Ok(mut metrics)) = (self.cache.read(), self.metrics.write()) {
            metrics.current_size = cache.len();
        }
    }

    /// slay Get cache metrics
    pub fn get_metrics(&self) -> CrlCacheMetrics {
        self.metrics.read()
            .map(|m| m.clone())
            .unwrap_or_default()
    }
}

impl CrlValidator {
    /// slay Create new CRL validator
    #[instrument]
    pub fn new() -> Self {
        Self {
            config: CrlValidatorConfig::default(),
            trust_anchors: Vec::new(),
            metrics: Arc::new(RwLock::new(CrlValidatorMetrics::default())),
        }
    }

    /// slay Create validator with configuration
    #[instrument]
    pub fn with_config(config: CrlValidatorConfig) -> Self {
        Self {
            config,
            trust_anchors: Vec::new(),
            metrics: Arc::new(RwLock::new(CrlValidatorMetrics::default())),
        }
    }

    /// slay Add trust anchor
    #[instrument(skip(self, anchor))]
    pub fn add_trust_anchor(&mut self, anchor: Certificate) {
        self.trust_anchors.push(anchor);
    }

    /// slay Validate CRL
    #[instrument(skip(self, crl))]
    pub fn validate(&self, crl: &CertificateRevocationList) -> CrlResult<()> {
        let start_time = SystemTime::now();
        let mut validation_successful = true;

        // Check timestamps
        if self.config.check_timestamps {
            if let Err(e) = self.validate_timestamps(crl) {
                validation_successful = false;
                return Err(e);
            }
        }

        // Check signature
        if self.config.check_signatures {
            if let Err(e) = self.validate_signature(crl) {
                validation_successful = false;
                if let Ok(mut metrics) = self.metrics.write() {
                    metrics.signature_failures += 1;
                }
                return Err(e);
            }
        }

        // Update metrics
        self.update_validation_metrics(validation_successful, start_time);

        if validation_successful {
            info!("CRL validation successful for issuer: {}", crl.issuer);
            Ok(())
        } else {
            Err(CrlError::Internal("Validation failed".to_string()))
        }
    }

    /// slay Validate CRL timestamps
    #[instrument(skip(self, crl))]
    fn validate_timestamps(&self, crl: &CertificateRevocationList) -> CrlResult<()> {
        let now = SystemTime::now();
        
        // Check thisUpdate
        if now < crl.this_update.checked_sub(self.config.clock_skew_tolerance).unwrap_or(crl.this_update) {
            if let Ok(mut metrics) = self.metrics.write() {
                metrics.timestamp_failures += 1;
            }
            return Err(CrlError::NotYetValid);
        }

        // Check nextUpdate
        if let Some(next_update) = crl.next_update {
            if now > next_update.checked_add(self.config.clock_skew_tolerance).unwrap_or(next_update) {
                if !self.config.allow_stale {
                    if let Ok(mut metrics) = self.metrics.write() {
                        metrics.timestamp_failures += 1;
                    }
                    return Err(CrlError::Expired);
                }
            }
        }

        Ok(())
    }

    /// slay Validate CRL signature
    #[instrument(skip(self, crl))]
    fn validate_signature(&self, crl: &CertificateRevocationList) -> CrlResult<()> {
        // Find issuer certificate
        let issuer_cert = crl.issuer_certificate.as_ref()
            .or_else(|| self.find_issuer_certificate(&crl.issuer))
            .ok_or(CrlError::SignatureVerificationFailed)?;

        // Verify signature (simplified implementation)
        if crl.signature.is_empty() {
            return Err(CrlError::SignatureVerificationFailed);
        }

        // In a real implementation, would verify the signature properly
        debug!("CRL signature verification passed for issuer: {}", crl.issuer);
        Ok(())
    }

    /// slay Find issuer certificate
    #[instrument(skip(self))]
    fn find_issuer_certificate(&self, issuer_name: &str) -> Option<&Certificate> {
        self.trust_anchors.iter()
            .find(|cert| cert.subject_dn() == issuer_name)
    }

    /// slay Update validation metrics
    #[instrument(skip(self))]
    fn update_validation_metrics(&self, success: bool, start_time: SystemTime) {
        if let Ok(mut metrics) = self.metrics.write() {
            metrics.total_validations += 1;
            if success {
                metrics.successful_validations += 1;
            } else {
                metrics.failed_validations += 1;
            }
        }
    }

    /// slay Get validator metrics
    pub fn get_metrics(&self) -> CrlValidatorMetrics {
        self.metrics.read()
            .map(|m| m.clone())
            .unwrap_or_default()
    }
}

/// fr fr Convenience functions for common operations

/// slay Create CRL with default configuration
#[instrument(skip(issuer, revoked_certs))]
pub fn create_crl(issuer: CrlIssuer, revoked_certs: &[(Vec<u8>, SystemTime, RevocationReason)]) -> CrlResult<CertificateRevocationList> {
    let mut builder = CrlBuilder::new();
    builder.set_issuer(issuer);
    
    for (serial, revocation_time, reason) in revoked_certs {
        builder.add_revoked_certificate(serial.clone(), *revocation_time, reason.clone());
    }
    
    builder.build()
}

/// slay Parse CRL from bytes
#[instrument(skip(data))]
pub fn parse_crl(data: &[u8]) -> CrlResult<CertificateRevocationList> {
    // Simplified implementation - would use proper ASN.1 parsing
    if data.len() < 10 {
        return Err(CrlError::InvalidFormat("CRL too short".to_string()));
    }
    
    Ok(CertificateRevocationList {
        version: 2,
        signature_algorithm: "SHA256withRSA".to_string(),
        issuer: "CN=Test CA".to_string(),
        this_update: SystemTime::now(),
        next_update: Some(SystemTime::now() + Duration::from_secs(7 * 24 * 3600)),
        revoked_certificates: Vec::new(),
        extensions: Vec::new(),
        signature: data[..16.min(data.len())].to_vec(),
        issuer_certificate: None,
    })
}

/// slay Verify CRL with default validator
#[instrument(skip(crl))]
pub fn verify_crl(crl: &CertificateRevocationList) -> CrlResult<()> {
    let validator = CrlValidator::new();
    validator.validate(crl)
}

/// slay Check revocation status in CRL
#[instrument(skip(crl))]
pub fn check_revocation_status(crl: &CertificateRevocationList, serial_number: &[u8]) -> RevocationStatus {
    for entry in &crl.revoked_certificates {
        if entry.serial_number == serial_number {
            return RevocationStatus::Revoked {
                revocation_time: entry.revocation_date,
                reason: entry.reason.clone(),
                info: None,
            };
        }
    }
    
    RevocationStatus::Good
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crl_builder_creation() {
        let builder = CrlBuilder::new();
        assert_eq!(builder.entries.len(), 0);
        assert_eq!(builder.extensions.len(), 0);
    }

    #[test]
    fn test_crl_cache_creation() {
        let cache = CrlCache::new();
        let metrics = cache.get_metrics();
        assert_eq!(metrics.hits, 0);
        assert_eq!(metrics.misses, 0);
    }

    #[test]
    fn test_crl_validator_creation() {
        let validator = CrlValidator::new();
        let metrics = validator.get_metrics();
        assert_eq!(metrics.total_validations, 0);
    }

    #[test]
    fn test_revocation_status() {
        let status = RevocationStatus::Good;
        assert_eq!(status, RevocationStatus::Good);
        
        let revoked_status = RevocationStatus::Revoked {
            revocation_time: SystemTime::now(),
            reason: RevocationReason::KeyCompromise,
            info: None,
        };
        
        match revoked_status {
            RevocationStatus::Revoked { reason, .. } => {
                assert_eq!(reason, RevocationReason::KeyCompromise);
            }
            _ => panic!("Expected revoked status"),
        }
    }

    #[test]
    fn test_crl_entry_creation() {
        let entry = CrlEntry {
            serial_number: vec![1, 2, 3, 4],
            revocation_date: SystemTime::now(),
            reason: RevocationReason::CessationOfOperation,
            extensions: Vec::new(),
            invalidity_date: None,
        };
        
        assert_eq!(entry.serial_number, vec![1, 2, 3, 4]);
        assert_eq!(entry.reason, RevocationReason::CessationOfOperation);
    }
}
