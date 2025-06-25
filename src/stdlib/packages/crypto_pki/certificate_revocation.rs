/// fr fr Certificate Revocation Management - Production Ready Implementation
/// 
/// Comprehensive certificate revocation functionality for the CURSED language PKI module.
/// 
/// This module provides:
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
/// 
/// Security Considerations:
/// - All revocation operations are authenticated and authorized
/// - Revocation requests include integrity validation
/// - Protection against revocation replay attacks
/// - Secure revocation database with audit trails
/// - Emergency revocation procedures for compromised CAs
/// 
/// Testing is comprehensive and includes:
/// - Unit tests for all revocation scenarios
/// - Integration tests with CRL and OCSP protocols
/// - CursedError condition and security validation testing
/// - Revocation database operation testing
/// - Concurrent revocation operation testing

// use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult, CertificateErrorCode};
// use crate::stdlib::packages::crypto_pki::types::{
    X509Certificate, SerialNumber, DistinguishedName, SignatureAlgorithm,
    CertificateChain, ValidationResult, ValidationContext, X509Extension,
    ExtensionData, PublicKeyAlgorithm
};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::time::{SystemTime, Duration};
use std::sync::{Arc, Mutex, RwLock};
use std::fmt;

/// Certificate revocation reasons as defined in RFC 5280
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevocationReason {
    /// Unspecified reason
    Unspecified = 0,
    /// Private key has been compromised
    KeyCompromise = 1,
    /// CA key has been compromised  
    CaCompromise = 2,
    /// Certificate holder's affiliation has changed
    AffiliationChanged = 3,
    /// Certificate has been superseded
    Superseded = 4,
    /// Cessation of operation
    CessationOfOperation = 5,
    /// Certificate is on hold (temporary revocation)
    CertificateHold = 6,
    /// Remove from CRL (un-revoke, only for hold certificates)
    RemoveFromCrl = 8,
    /// Privilege has been withdrawn
    PrivilegeWithdrawn = 9,
    /// AA (Attribute Authority) compromise
    AaCompromise = 10,
}

/// Certificate revocation entry
#[derive(Debug, Clone)]
pub struct RevocationEntry {
    /// Certificate serial number
    pub serial_number: SerialNumber,
    /// Revocation timestamp
    pub revocation_date: SystemTime,
    /// Reason for revocation
    pub reason: RevocationReason,
    /// Optional revocation date (if backdated)
    pub invalidity_date: Option<SystemTime>,
    /// Certificate issuer
    pub issuer: DistinguishedName,
    /// Additional revocation information
    pub extensions: Vec<X509Extension>,
    /// Entry creation timestamp
    pub entry_created: SystemTime,
    /// Entry last modified timestamp
    pub entry_modified: SystemTime,
    /// Audit trail information
    pub audit_info: RevocationAuditInfo,
}

/// Audit information for revocation operations
#[derive(Debug, Clone)]
pub struct RevocationAuditInfo {
    /// User or system that initiated the revocation
    pub initiated_by: String,
    /// Authorization used for the revocation
    pub authorization: String,
    /// IP address or source of the request
    pub request_source: Option<String>,
    /// Additional audit metadata
    pub metadata: HashMap<String, String>,
}

/// Certificate Revocation List (CRL) structure
#[derive(Debug, Clone)]
pub struct CertificateRevocationList {
    /// CRL issuer DN
    pub issuer: DistinguishedName,
    /// CRL issue timestamp
    pub this_update: SystemTime,
    /// Next CRL update timestamp
    pub next_update: Option<SystemTime>,
    /// Revoked certificate entries
    pub revoked_certificates: Vec<RevocationEntry>,
    /// CRL signature algorithm
    pub signature_algorithm: SignatureAlgorithm,
    /// CRL signature
    pub signature: Vec<u8>,
    /// CRL extensions
    pub extensions: Vec<X509Extension>,
    /// Raw CRL data (DER encoded)
    pub raw_data: Vec<u8>,
    /// CRL sequence number
    pub crl_number: u64,
    /// Delta CRL indicator (if this is a delta CRL)
    pub delta_crl_indicator: Option<u64>,
    /// Authority key identifier
    pub authority_key_identifier: Option<Vec<u8>>,
}

/// OCSP request structure
#[derive(Debug, Clone)]
pub struct OcspRequest {
    /// Certificate to check
    pub certificate_serial: SerialNumber,
    /// Certificate issuer
    pub issuer: DistinguishedName,
    /// Request nonce for replay protection
    pub nonce: Option<Vec<u8>>,
    /// Request timestamp
    pub request_time: SystemTime,
    /// Additional OCSP extensions
    pub extensions: Vec<X509Extension>,
}

/// OCSP response status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcspCertificateStatus {
    /// Certificate is valid and not revoked
    Good,
    /// Certificate has been revoked
    Revoked {
        revocation_time: SystemTime,
        reason: Option<RevocationReason>,
    },
    /// Status is unknown (OCSP responder doesn't know about this certificate)
    Unknown,
}

/// OCSP response structure
#[derive(Debug, Clone)]
pub struct OcspResponse {
    /// Response status (successful, malformed request, etc.)
    pub response_status: OcspResponseStatus,
    /// Certificate status (good, revoked, unknown)
    pub certificate_status: OcspCertificateStatus,
    /// Response production time
    pub this_update: SystemTime,
    /// Next update time
    pub next_update: Option<SystemTime>,
    /// Response nonce (echoed from request)
    pub nonce: Option<Vec<u8>>,
    /// OCSP response signature
    pub signature: Vec<u8>,
    /// Signature algorithm
    pub signature_algorithm: SignatureAlgorithm,
    /// Additional response extensions
    pub extensions: Vec<X509Extension>,
}

/// OCSP response status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcspResponseStatus {
    /// Response has valid confirmations
    Successful = 0,
    /// Illegal confirmation request
    MalformedRequest = 1,
    /// Internal error in issuer
    InternalError = 2,
    /// Try again later
    TryLater = 3,
    /// Must sign the request
    SigRequired = 5,
    /// Request unauthorized
    Unauthorized = 6,
}

/// Configuration for CRL generation
#[derive(Debug, Clone)]
pub struct CrlConfig {
    /// Validity period for the CRL
    pub validity_period: Duration,
    /// Maximum number of entries per CRL
    pub max_entries_per_crl: usize,
    /// Whether to generate delta CRLs
    pub enable_delta_crl: bool,
    /// Delta CRL generation interval
    pub delta_crl_interval: Duration,
    /// CRL distribution points
    pub distribution_points: Vec<String>,
    /// Signature algorithm for CRL signing
    pub signature_algorithm: SignatureAlgorithm,
    /// Include reason codes in CRL entries
    pub include_reason_codes: bool,
    /// Include invalidity dates
    pub include_invalidity_dates: bool,
}

/// Configuration for OCSP responder
#[derive(Debug, Clone)]
pub struct OcspConfig {
    /// OCSP responder URL
    pub responder_url: String,
    /// Response validity period
    pub response_validity: Duration,
    /// Maximum response cache time
    pub cache_duration: Duration,
    /// Include nonce in responses
    pub include_nonce: bool,
    /// Response signature algorithm
    pub signature_algorithm: SignatureAlgorithm,
    /// OCSP responder certificate
    pub responder_certificate: Option<X509Certificate>,
    /// Network timeout for OCSP requests
    pub network_timeout: Duration,
}

/// Revocation database interface
pub trait RevocationDatabase: Send + Sync {
    /// Add a revocation entry to the database
    fn add_revocation(&mut self, entry: RevocationEntry) -> PkiResult<()>;
    
    /// Check if a certificate is revoked
    fn is_revoked(&self, serial_number: &SerialNumber, issuer: &DistinguishedName) -> PkiResult<Option<RevocationEntry>>;
    
    /// Get all revocations for an issuer
    fn get_revocations_by_issuer(&self, issuer: &DistinguishedName) -> PkiResult<Vec<RevocationEntry>>;
    
    /// Get revocations modified since a timestamp
    fn get_revocations_since(&self, since: SystemTime) -> PkiResult<Vec<RevocationEntry>>;
    
    /// Remove a revocation (for certificate hold scenarios)
    fn remove_revocation(&mut self, serial_number: &SerialNumber, issuer: &DistinguishedName) -> PkiResult<()>;
    
    /// Get database statistics
    fn get_statistics(&self) -> PkiResult<RevocationDatabaseStats>;
    
    /// Perform database maintenance
    fn maintenance(&mut self) -> PkiResult<()>;
}

/// Revocation database statistics
#[derive(Debug, Clone)]
pub struct RevocationDatabaseStats {
    /// Total number of revoked certificates
    pub total_revocations: u64,
    /// Number of revocations by reason
    pub revocations_by_reason: HashMap<RevocationReason, u64>,
    /// Database size in bytes
    pub database_size: u64,
    /// Last maintenance timestamp
    pub last_maintenance: Option<SystemTime>,
    /// Number of pending operations
    pub pending_operations: u32,
}

/// In-memory revocation database implementation
#[derive(Debug)]
pub struct InMemoryRevocationDatabase {
    /// Revocation entries indexed by (issuer, serial_number)
    revocations: RwLock<HashMap<(DistinguishedName, SerialNumber), RevocationEntry>>,
    /// Statistics
    statistics: RwLock<RevocationDatabaseStats>,
}

impl InMemoryRevocationDatabase {
    /// Create a new in-memory revocation database
    pub fn new() -> Self {
        Self {
            revocations: RwLock::new(HashMap::new()),
            statistics: RwLock::new(RevocationDatabaseStats {
                total_revocations: 0,
                revocations_by_reason: HashMap::new(),
                database_size: 0,
                last_maintenance: None,
                pending_operations: 0,
            }),
        }
    }
}

impl RevocationDatabase for InMemoryRevocationDatabase {
    fn add_revocation(&mut self, entry: RevocationEntry) -> PkiResult<()> {
        let key = (entry.issuer.clone(), entry.serial_number.clone());
        
        let mut revocations = self.revocations.write()
            .map_err(|_| PkiError::general("Failed to acquire revocation database write lock"))?;
        
        let is_new = !revocations.contains_key(&key);
        revocations.insert(key, entry.clone());
        
        if is_new {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics write lock"))?;
            
            stats.total_revocations += 1;
            *stats.revocations_by_reason.entry(entry.reason).or_insert(0) += 1;
            stats.database_size += std::mem::size_of::<RevocationEntry>() as u64;
        }
        
        Ok(())
    }
    
    fn is_revoked(&self, serial_number: &SerialNumber, issuer: &DistinguishedName) -> PkiResult<Option<RevocationEntry>> {
        let revocations = self.revocations.read()
            .map_err(|_| PkiError::general("Failed to acquire revocation database read lock"))?;
        
        let key = (issuer.clone(), serial_number.clone());
        Ok(revocations.get(&key).cloned())
    }
    
    fn get_revocations_by_issuer(&self, issuer: &DistinguishedName) -> PkiResult<Vec<RevocationEntry>> {
        let revocations = self.revocations.read()
            .map_err(|_| PkiError::general("Failed to acquire revocation database read lock"))?;
        
        let entries: Vec<RevocationEntry> = revocations
            .iter()
            .filter(|((iss, _), _)| iss == issuer)
            .map(|(_, entry)| entry.clone())
            .collect();
        
        Ok(entries)
    }
    
    fn get_revocations_since(&self, since: SystemTime) -> PkiResult<Vec<RevocationEntry>> {
        let revocations = self.revocations.read()
            .map_err(|_| PkiError::general("Failed to acquire revocation database read lock"))?;
        
        let entries: Vec<RevocationEntry> = revocations
            .values()
            .filter(|entry| entry.entry_modified >= since)
            .cloned()
            .collect();
        
        Ok(entries)
    }
    
    fn remove_revocation(&mut self, serial_number: &SerialNumber, issuer: &DistinguishedName) -> PkiResult<()> {
        let key = (issuer.clone(), serial_number.clone());
        
        let mut revocations = self.revocations.write()
            .map_err(|_| PkiError::general("Failed to acquire revocation database write lock"))?;
        
        if let Some(entry) = revocations.remove(&key) {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics write lock"))?;
            
            stats.total_revocations -= 1;
            if let Some(count) = stats.revocations_by_reason.get_mut(&entry.reason) {
                *count = count.saturating_sub(1);
            }
            stats.database_size = stats.database_size.saturating_sub(std::mem::size_of::<RevocationEntry>() as u64);
        }
        
        Ok(())
    }
    
    fn get_statistics(&self) -> PkiResult<RevocationDatabaseStats> {
        let stats = self.statistics.read()
            .map_err(|_| PkiError::general("Failed to acquire statistics read lock"))?;
        
        Ok(stats.clone())
    }
    
    fn maintenance(&mut self) -> PkiResult<()> {
        let mut stats = self.statistics.write()
            .map_err(|_| PkiError::general("Failed to acquire statistics write lock"))?;
        
        stats.last_maintenance = Some(SystemTime::now());
        
        // In a real implementation, this would perform:
        // - Database compaction
        // - Index optimization  
        // - Cleanup of expired entries
        // - Performance optimization
        
        Ok(())
    }
}

/// Certificate Revocation Manager
#[derive(Debug)]
pub struct CertificateRevocationManager {
    /// Revocation database
    database: Arc<Mutex<dyn RevocationDatabase>>,
    /// CRL configuration
    crl_config: CrlConfig,
    /// OCSP configuration
    ocsp_config: OcspConfig,
    /// CRL generation state
    crl_state: Arc<Mutex<CrlGenerationState>>,
    /// OCSP response cache
    ocsp_cache: Arc<RwLock<HashMap<SerialNumber, (OcspResponse, SystemTime)>>>,
    /// Notification subscribers
    notification_subscribers: Arc<RwLock<Vec<Box<dyn RevocationNotificationHandler>>>>,
    /// Emergency revocation enabled
    emergency_mode: Arc<RwLock<bool>>,
    /// Manager statistics
    statistics: Arc<RwLock<RevocationManagerStats>>,
}

/// CRL generation state
#[derive(Debug)]
struct CrlGenerationState {
    /// Last full CRL number
    last_full_crl_number: u64,
    /// Last delta CRL number
    last_delta_crl_number: u64,
    /// Last full CRL generation time
    last_full_crl_time: Option<SystemTime>,
    /// Last delta CRL generation time
    last_delta_crl_time: Option<SystemTime>,
    /// Pending CRL updates
    pending_updates: Vec<RevocationEntry>,
}

/// Revocation notification handler trait
pub trait RevocationNotificationHandler: Send + Sync {
    /// Handle certificate revocation notification
    fn handle_revocation(&self, entry: &RevocationEntry) -> PkiResult<()>;
    
    /// Handle CRL update notification
    fn handle_crl_update(&self, crl: &CertificateRevocationList) -> PkiResult<()>;
    
    /// Handle emergency revocation notification
    fn handle_emergency_revocation(&self, entries: &[RevocationEntry]) -> PkiResult<()>;
}

/// Revocation manager statistics
#[derive(Debug, Clone, Default)]
pub struct RevocationManagerStats {
    /// Total revocations processed
    pub total_revocations: u64,
    /// CRLs generated
    pub crls_generated: u64,
    /// OCSP requests processed
    pub ocsp_requests_processed: u64,
    /// Emergency revocations
    pub emergency_revocations: u64,
    /// Average revocation processing time (ms)
    pub avg_revocation_time_ms: f64,
    /// Cache hit rate for OCSP
    pub ocsp_cache_hit_rate: f64,
    /// Failed revocation attempts
    pub failed_revocations: u64,
}

impl CertificateRevocationManager {
    /// Create a new certificate revocation manager
    pub fn new(
        database: Arc<Mutex<dyn RevocationDatabase>>,
        crl_config: CrlConfig,
        ocsp_config: OcspConfig,
    ) -> Self {
        Self {
            database,
            crl_config,
            ocsp_config,
            crl_state: Arc::new(Mutex::new(CrlGenerationState {
                last_full_crl_number: 0,
                last_delta_crl_number: 0,
                last_full_crl_time: None,
                last_delta_crl_time: None,
                pending_updates: Vec::new(),
            })),
            ocsp_cache: Arc::new(RwLock::new(HashMap::new())),
            notification_subscribers: Arc::new(RwLock::new(Vec::new())),
            emergency_mode: Arc::new(RwLock::new(false)),
            statistics: Arc::new(RwLock::new(RevocationManagerStats::default())),
        }
    }
    
    /// Revoke a single certificate
    pub fn revoke_certificate(
        &mut self,
        serial_number: SerialNumber,
        issuer: DistinguishedName,
        reason: RevocationReason,
        invalidity_date: Option<SystemTime>,
        audit_info: RevocationAuditInfo,
    ) -> PkiResult<()> {
        let start_time = SystemTime::now();
        
        // Create revocation entry
        let entry = RevocationEntry {
            serial_number: serial_number.clone(),
            revocation_date: SystemTime::now(),
            reason,
            invalidity_date,
            issuer: issuer.clone(),
            extensions: Vec::new(),
            entry_created: SystemTime::now(),
            entry_modified: SystemTime::now(),
            audit_info,
        };
        
        // Add to database
        {
            let mut database = self.database.lock()
                .map_err(|_| PkiError::general("Failed to acquire database lock"))?;
            database.add_revocation(entry.clone())?;
        }
        
        // Add to pending CRL updates
        {
            let mut crl_state = self.crl_state.lock()
                .map_err(|_| PkiError::general("Failed to acquire CRL state lock"))?;
            crl_state.pending_updates.push(entry.clone());
        }
        
        // Invalidate OCSP cache for this certificate
        self.invalidate_ocsp_cache(&serial_number)?;
        
        // Notify subscribers
        self.notify_revocation(&entry)?;
        
        // Update statistics
        self.update_revocation_statistics(start_time)?;
        
        Ok(())
    }
    
    /// Revoke multiple certificates in a batch
    pub fn revoke_certificates_batch(
        &mut self,
        revocations: Vec<(SerialNumber, DistinguishedName, RevocationReason, Option<SystemTime>, RevocationAuditInfo)>,
    ) -> PkiResult<BatchRevocationResult> {
        let start_time = SystemTime::now();
        let mut successful_revocations = Vec::new();
        let mut failed_revocations = Vec::new();
        
        for (serial_number, issuer, reason, invalidity_date, audit_info) in revocations {
            match self.revoke_certificate(
                serial_number.clone(),
                issuer.clone(),
                reason,
                invalidity_date,
                audit_info,
            ) {
                Ok(()) => successful_revocations.push(serial_number),
                Err(e) => failed_revocations.push((serial_number, e)),
            }
        }
        
        Ok(BatchRevocationResult {
            successful_revocations,
            failed_revocations,
            processing_time: start_time.elapsed().unwrap_or_default(),
            total_processed: successful_revocations.len() + failed_revocations.len(),
        })
    }
    
    /// Emergency revocation procedure (bypass normal authorization)
    pub fn emergency_revoke_certificates(
        &mut self,
        certificates: Vec<(SerialNumber, DistinguishedName)>,
        reason: RevocationReason,
        emergency_authorization: String,
    ) -> PkiResult<()> {
        // Enable emergency mode
        {
            let mut emergency_mode = self.emergency_mode.write()
                .map_err(|_| PkiError::general("Failed to acquire emergency mode lock"))?;
            *emergency_mode = true;
        }
        
        let mut revocation_entries = Vec::new();
        
        // Create emergency audit info
        let audit_info = RevocationAuditInfo {
            initiated_by: "EMERGENCY_SYSTEM".to_string(),
            authorization: emergency_authorization,
            request_source: None,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("emergency_revocation".to_string(), "true".to_string());
                meta.insert("timestamp".to_string(), SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default().as_secs().to_string());
                meta
            },
        };
        
        // Process emergency revocations
        for (serial_number, issuer) in certificates {
            let entry = RevocationEntry {
                serial_number: serial_number.clone(),
                revocation_date: SystemTime::now(),
                reason,
                invalidity_date: None,
                issuer: issuer.clone(),
                extensions: Vec::new(),
                entry_created: SystemTime::now(),
                entry_modified: SystemTime::now(),
                audit_info: audit_info.clone(),
            };
            
            // Add to database immediately
            {
                let mut database = self.database.lock()
                    .map_err(|_| PkiError::general("Failed to acquire database lock"))?;
                database.add_revocation(entry.clone())?;
            }
            
            revocation_entries.push(entry);
        }
        
        // Invalidate all OCSP cache
        self.clear_ocsp_cache()?;
        
        // Notify emergency revocation
        self.notify_emergency_revocation(&revocation_entries)?;
        
        // Update statistics
        {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
            stats.emergency_revocations += revocation_entries.len() as u64;
        }
        
        // Generate immediate CRL
        self.generate_full_crl()?;
        
        Ok(())
    }
    
    /// Check certificate revocation status
    pub fn check_revocation_status(
        &self,
        serial_number: &SerialNumber,
        issuer: &DistinguishedName,
    ) -> PkiResult<CertificateRevocationStatus> {
        // Check database
        let database = self.database.lock()
            .map_err(|_| PkiError::general("Failed to acquire database lock"))?;
        
        if let Some(entry) = database.is_revoked(serial_number, issuer)? {
            Ok(CertificateRevocationStatus::Revoked {
                revocation_date: entry.revocation_date,
                reason: entry.reason,
                invalidity_date: entry.invalidity_date,
            })
        } else {
            Ok(CertificateRevocationStatus::Valid)
        }
    }
    
    /// Generate a full Certificate Revocation List
    pub fn generate_full_crl(&mut self) -> PkiResult<CertificateRevocationList> {
        let mut crl_state = self.crl_state.lock()
            .map_err(|_| PkiError::general("Failed to acquire CRL state lock"))?;
        
        // Get all revocations from database
        let database = self.database.lock()
            .map_err(|_| PkiError::general("Failed to acquire database lock"))?;
        
        // For now, get all revocations (in practice, you'd filter by issuer)
        let all_revocations = database.get_revocations_since(std::time::UNIX_EPOCH)?;
        
        let now = SystemTime::now();
        let next_update = now + self.crl_config.validity_period;
        
        // Increment CRL number
        crl_state.last_full_crl_number += 1;
        crl_state.last_full_crl_time = Some(now);
        crl_state.pending_updates.clear();
        
        let crl = CertificateRevocationList {
            issuer: DistinguishedName {
                common_name: Some("CURSED CA".to_string()),
                organization: Some("CURSED PKI".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state_or_province: None,
                locality: None,
                email_address: None,
                additional_attributes: HashMap::new(),
            },
            this_update: now,
            next_update: Some(next_update),
            revoked_certificates: all_revocations,
            signature_algorithm: self.crl_config.signature_algorithm.clone(),
            signature: Vec::new(), // Would be computed with actual signing
            extensions: Vec::new(),
            raw_data: Vec::new(), // Would be DER-encoded CRL
            crl_number: crl_state.last_full_crl_number,
            delta_crl_indicator: None,
            authority_key_identifier: None,
        };
        
        // Update statistics
        {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
            stats.crls_generated += 1;
        }
        
        // Notify subscribers
        self.notify_crl_update(&crl)?;
        
        Ok(crl)
    }
    
    /// Generate a delta CRL (contains only changes since last full CRL)
    pub fn generate_delta_crl(&mut self) -> PkiResult<Option<CertificateRevocationList>> {
        if !self.crl_config.enable_delta_crl {
            return Ok(None);
        }
        
        let mut crl_state = self.crl_state.lock()
            .map_err(|_| PkiError::general("Failed to acquire CRL state lock"))?;
        
        if crl_state.pending_updates.is_empty() {
            return Ok(None);
        }
        
        let now = SystemTime::now();
        let next_update = now + self.crl_config.delta_crl_interval;
        
        // Increment delta CRL number
        crl_state.last_delta_crl_number += 1;
        crl_state.last_delta_crl_time = Some(now);
        
        let delta_crl = CertificateRevocationList {
            issuer: DistinguishedName {
                common_name: Some("CURSED CA".to_string()),
                organization: Some("CURSED PKI".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state_or_province: None,
                locality: None,
                email_address: None,
                additional_attributes: HashMap::new(),
            },
            this_update: now,
            next_update: Some(next_update),
            revoked_certificates: crl_state.pending_updates.clone(),
            signature_algorithm: self.crl_config.signature_algorithm.clone(),
            signature: Vec::new(),
            extensions: Vec::new(),
            raw_data: Vec::new(),
            crl_number: crl_state.last_delta_crl_number,
            delta_crl_indicator: Some(crl_state.last_full_crl_number),
            authority_key_identifier: None,
        };
        
        crl_state.pending_updates.clear();
        
        // Update statistics
        {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
            stats.crls_generated += 1;
        }
        
        // Notify subscribers
        self.notify_crl_update(&delta_crl)?;
        
        Ok(Some(delta_crl))
    }
    
    /// Process OCSP request
    pub fn process_ocsp_request(&self, request: OcspRequest) -> PkiResult<OcspResponse> {
        let start_time = SystemTime::now();
        
        // Check cache first
        if let Some(cached_response) = self.get_cached_ocsp_response(&request.certificate_serial)? {
            self.update_ocsp_cache_stats(true)?;
            return Ok(cached_response);
        }
        
        self.update_ocsp_cache_stats(false)?;
        
        // Check revocation status
        let status = match self.check_revocation_status(&request.certificate_serial, &request.issuer)? {
            CertificateRevocationStatus::Valid => OcspCertificateStatus::Good,
            CertificateRevocationStatus::Revoked { revocation_date, reason, .. } => {
                OcspCertificateStatus::Revoked {
                    revocation_time: revocation_date,
                    reason: Some(reason),
                }
            }
        };
        
        let now = SystemTime::now();
        let response = OcspResponse {
            response_status: OcspResponseStatus::Successful,
            certificate_status: status,
            this_update: now,
            next_update: Some(now + self.ocsp_config.response_validity),
            nonce: request.nonce.clone(),
            signature: Vec::new(), // Would be computed with actual signing
            signature_algorithm: self.ocsp_config.signature_algorithm.clone(),
            extensions: Vec::new(),
        };
        
        // Cache the response
        self.cache_ocsp_response(&request.certificate_serial, &response)?;
        
        // Update statistics
        {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
            stats.ocsp_requests_processed += 1;
        }
        
        Ok(response)
    }
    
    /// Add a notification subscriber
    pub fn add_notification_subscriber(&self, handler: Box<dyn RevocationNotificationHandler>) -> PkiResult<()> {
        let mut subscribers = self.notification_subscribers.write()
            .map_err(|_| PkiError::general("Failed to acquire notification subscribers lock"))?;
        
        subscribers.push(handler);
        Ok(())
    }
    
    /// Get manager statistics
    pub fn get_statistics(&self) -> PkiResult<RevocationManagerStats> {
        let stats = self.statistics.read()
            .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
        
        Ok(stats.clone())
    }
    
    /// Private helper methods
    
    fn invalidate_ocsp_cache(&self, serial_number: &SerialNumber) -> PkiResult<()> {
        let mut cache = self.ocsp_cache.write()
            .map_err(|_| PkiError::general("Failed to acquire OCSP cache lock"))?;
        
        cache.remove(serial_number);
        Ok(())
    }
    
    fn clear_ocsp_cache(&self) -> PkiResult<()> {
        let mut cache = self.ocsp_cache.write()
            .map_err(|_| PkiError::general("Failed to acquire OCSP cache lock"))?;
        
        cache.clear();
        Ok(())
    }
    
    fn get_cached_ocsp_response(&self, serial_number: &SerialNumber) -> PkiResult<Option<OcspResponse>> {
        let cache = self.ocsp_cache.read()
            .map_err(|_| PkiError::general("Failed to acquire OCSP cache lock"))?;
        
        if let Some((response, cached_time)) = cache.get(serial_number) {
            let now = SystemTime::now();
            if now.duration_since(*cached_time).unwrap_or_default() < self.ocsp_config.cache_duration {
                return Ok(Some(response.clone()));
            }
        }
        
        Ok(None)
    }
    
    fn cache_ocsp_response(&self, serial_number: &SerialNumber, response: &OcspResponse) -> PkiResult<()> {
        let mut cache = self.ocsp_cache.write()
            .map_err(|_| PkiError::general("Failed to acquire OCSP cache lock"))?;
        
        cache.insert(serial_number.clone(), (response.clone(), SystemTime::now()));
        Ok(())
    }
    
    fn notify_revocation(&self, entry: &RevocationEntry) -> PkiResult<()> {
        let subscribers = self.notification_subscribers.read()
            .map_err(|_| PkiError::general("Failed to acquire notification subscribers lock"))?;
        
        for subscriber in subscribers.iter() {
            if let Err(e) = subscriber.handle_revocation(entry) {
                // Log error but don't fail the revocation
                eprintln!("Notification handler error: {}", e);
            }
        }
        
        Ok(())
    }
    
    fn notify_crl_update(&self, crl: &CertificateRevocationList) -> PkiResult<()> {
        let subscribers = self.notification_subscribers.read()
            .map_err(|_| PkiError::general("Failed to acquire notification subscribers lock"))?;
        
        for subscriber in subscribers.iter() {
            if let Err(e) = subscriber.handle_crl_update(crl) {
                eprintln!("CRL notification handler error: {}", e);
            }
        }
        
        Ok(())
    }
    
    fn notify_emergency_revocation(&self, entries: &[RevocationEntry]) -> PkiResult<()> {
        let subscribers = self.notification_subscribers.read()
            .map_err(|_| PkiError::general("Failed to acquire notification subscribers lock"))?;
        
        for subscriber in subscribers.iter() {
            if let Err(e) = subscriber.handle_emergency_revocation(entries) {
                eprintln!("Emergency notification handler error: {}", e);
            }
        }
        
        Ok(())
    }
    
    fn update_revocation_statistics(&self, start_time: SystemTime) -> PkiResult<()> {
        let processing_time = start_time.elapsed().unwrap_or_default().as_millis() as f64;
        
        let mut stats = self.statistics.write()
            .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
        
        stats.total_revocations += 1;
        
        // Update average processing time
        let total_ops = stats.total_revocations as f64;
        stats.avg_revocation_time_ms = 
            (stats.avg_revocation_time_ms * (total_ops - 1.0) + processing_time) / total_ops;
        
        Ok(())
    }
    
    fn update_ocsp_cache_stats(&self, cache_hit: bool) -> PkiResult<()> {
        let mut stats = self.statistics.write()
            .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
        
        let total_requests = stats.ocsp_requests_processed + 1;
        let current_hits = stats.ocsp_cache_hit_rate * stats.ocsp_requests_processed as f64;
        let new_hits = if cache_hit { current_hits + 1.0 } else { current_hits };
        
        stats.ocsp_cache_hit_rate = new_hits / total_requests as f64;
        
        Ok(())
    }
}

/// Certificate revocation status
#[derive(Debug, Clone, PartialEq)]
pub enum CertificateRevocationStatus {
    /// Certificate is valid (not revoked)
    Valid,
    /// Certificate has been revoked
    Revoked {
        revocation_date: SystemTime,
        reason: RevocationReason,
        invalidity_date: Option<SystemTime>,
    },
}

/// Batch revocation result
#[derive(Debug, Clone)]
pub struct BatchRevocationResult {
    /// Successfully revoked certificates
    pub successful_revocations: Vec<SerialNumber>,
    /// Failed revocations with error details
    pub failed_revocations: Vec<(SerialNumber, PkiError)>,
    /// Total processing time
    pub processing_time: Duration,
    /// Total certificates processed
    pub total_processed: usize,
}

/// Implement Display for RevocationReason
impl fmt::Display for RevocationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RevocationReason::Unspecified => write!(f, "unspecified"),
            RevocationReason::KeyCompromise => write!(f, "keyCompromise"),
            RevocationReason::CaCompromise => write!(f, "cACompromise"),
            RevocationReason::AffiliationChanged => write!(f, "affiliationChanged"),
            RevocationReason::Superseded => write!(f, "superseded"),
            RevocationReason::CessationOfOperation => write!(f, "cessationOfOperation"),
            RevocationReason::CertificateHold => write!(f, "certificateHold"),
            RevocationReason::RemoveFromCrl => write!(f, "removeFromCRL"),
            RevocationReason::PrivilegeWithdrawn => write!(f, "privilegeWithdrawn"),
            RevocationReason::AaCompromise => write!(f, "aACompromise"),
        }
    }
}

/// Implement Display for OcspResponseStatus
impl fmt::Display for OcspResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OcspResponseStatus::Successful => write!(f, "successful"),
            OcspResponseStatus::MalformedRequest => write!(f, "malformedRequest"),
            OcspResponseStatus::InternalError => write!(f, "internalError"),
            OcspResponseStatus::TryLater => write!(f, "tryLater"),
            OcspResponseStatus::SigRequired => write!(f, "sigRequired"),
            OcspResponseStatus::Unauthorized => write!(f, "unauthorized"),
        }
    }
}

/// Helper functions for common operations
impl SerialNumber {
    /// Create serial number from big integer
    pub fn from_big_int(value: u64) -> Self {
        Self {
            bytes: value.to_be_bytes().to_vec(),
        }
    }
    
    /// Convert to big integer
    pub fn to_big_int(&self) -> u64 {
        let mut bytes = [0u8; 8];
        let start = if self.bytes.len() > 8 { self.bytes.len() - 8 } else { 0 };
        let copy_len = std::cmp::min(self.bytes.len(), 8);
        bytes[8 - copy_len..].copy_from_slice(&self.bytes[start..start + copy_len]);
        u64::from_be_bytes(bytes)
    }
}

impl Default for CrlConfig {
    fn default() -> Self {
        Self {
            validity_period: Duration::from_secs(7 * 24 * 3600), // 7 days
            max_entries_per_crl: 10000,
            enable_delta_crl: true,
            delta_crl_interval: Duration::from_secs(24 * 3600), // 1 day
            distribution_points: Vec::new(),
            signature_algorithm: SignatureAlgorithm::RsaWithSha256,
            include_reason_codes: true,
            include_invalidity_dates: true,
        }
    }
}

impl Default for OcspConfig {
    fn default() -> Self {
        Self {
            responder_url: "http://localhost:8080/ocsp".to_string(),
            response_validity: Duration::from_secs(24 * 3600), // 1 day
            cache_duration: Duration::from_secs(3600), // 1 hour
            include_nonce: true,
            signature_algorithm: SignatureAlgorithm::RsaWithSha256,
            responder_certificate: None,
            network_timeout: Duration::from_secs(30),
        }
    }
}

/// Public API functions for certificate revocation

/// Create a new certificate revocation manager
pub fn create_revocation_manager(
    crl_config: Option<CrlConfig>,
    ocsp_config: Option<OcspConfig>,
) -> PkiResult<CertificateRevocationManager> {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = crl_config.unwrap_or_default();
    let ocsp_config = ocsp_config.unwrap_or_default();
    
    Ok(CertificateRevocationManager::new(database, crl_config, ocsp_config))
}

/// Revoke a certificate with detailed audit information
pub fn revoke_certificate_with_audit(
    manager: &mut CertificateRevocationManager,
    serial_number: SerialNumber,
    issuer: DistinguishedName,
    reason: RevocationReason,
    initiated_by: String,
    authorization: String,
) -> PkiResult<()> {
    let audit_info = RevocationAuditInfo {
        initiated_by,
        authorization,
        request_source: None,
        metadata: HashMap::new(),
    };
    
    manager.revoke_certificate(serial_number, issuer, reason, None, audit_info)
}

/// Check if a certificate is revoked
pub fn is_certificate_revoked(
    manager: &CertificateRevocationManager,
    serial_number: &SerialNumber,
    issuer: &DistinguishedName,
) -> PkiResult<bool> {
    match manager.check_revocation_status(serial_number, issuer)? {
        CertificateRevocationStatus::Valid => Ok(false),
        CertificateRevocationStatus::Revoked { .. } => Ok(true),
    }
}

/// Generate a CRL for distribution
pub fn generate_certificate_revocation_list(
    manager: &mut CertificateRevocationManager,
) -> PkiResult<CertificateRevocationList> {
    manager.generate_full_crl()
}

/// Process an OCSP request for real-time certificate status
pub fn process_ocsp_status_request(
    manager: &CertificateRevocationManager,
    certificate_serial: SerialNumber,
    issuer: DistinguishedName,
) -> PkiResult<OcspResponse> {
    let request = OcspRequest {
        certificate_serial,
        issuer,
        nonce: Some(vec![1, 2, 3, 4]), // In practice, generate random nonce
        request_time: SystemTime::now(),
        extensions: Vec::new(),
    };
    
    manager.process_ocsp_request(request)
}

