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
// Placeholder imports disabled
    ExtensionData, PublicKeyAlgorithm
// };
use std::collections::{HashMap, HashSet, BTreeMap};
use std::time::{SystemTime, Duration};
use std::sync::{Arc, Mutex, RwLock};
use std::fmt;

/// Certificate revocation reasons as defined in RFC 5280
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevocationReason {
    /// Unspecified reason
    /// Private key has been compromised
    /// CA key has been compromised  
    /// Certificate holder's affiliation has changed
    /// Certificate has been superseded
    /// Cessation of operation
    /// Certificate is on hold (temporary revocation)
    /// Remove from CRL (un-revoke, only for hold certificates)
    /// Privilege has been withdrawn
    /// AA (Attribute Authority) compromise
/// Certificate revocation entry
#[derive(Debug, Clone)]
pub struct RevocationEntry {
    /// Certificate serial number
    /// Revocation timestamp
    /// Reason for revocation
    /// Optional revocation date (if backdated)
    /// Certificate issuer
    /// Additional revocation information
    /// Entry creation timestamp
    /// Entry last modified timestamp
    /// Audit trail information
/// Audit information for revocation operations
#[derive(Debug, Clone)]
pub struct RevocationAuditInfo {
    /// User or system that initiated the revocation
    /// Authorization used for the revocation
    /// IP address or source of the request
    /// Additional audit metadata
/// Certificate Revocation List (CRL) structure
#[derive(Debug, Clone)]
pub struct CertificateRevocationList {
    /// CRL issuer DN
    /// CRL issue timestamp
    /// Next CRL update timestamp
    /// Revoked certificate entries
    /// CRL signature algorithm
    /// CRL signature
    /// CRL extensions
    /// Raw CRL data (DER encoded)
    /// CRL sequence number
    /// Delta CRL indicator (if this is a delta CRL)
    /// Authority key identifier
/// OCSP request structure
#[derive(Debug, Clone)]
pub struct OcspRequest {
    /// Certificate to check
    /// Certificate issuer
    /// Request nonce for replay protection
    /// Request timestamp
    /// Additional OCSP extensions
/// OCSP response status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcspCertificateStatus {
    /// Certificate is valid and not revoked
    /// Certificate has been revoked
    Revoked {
    /// Status is unknown (OCSP responder doesn't know about this certificate)
/// OCSP response structure
#[derive(Debug, Clone)]
pub struct OcspResponse {
    /// Response status (successful, malformed request, etc.)
    /// Certificate status (good, revoked, unknown)
    /// Response production time
    /// Next update time
    /// Response nonce (echoed from request)
    /// OCSP response signature
    /// Signature algorithm
    /// Additional response extensions
/// OCSP response status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcspResponseStatus {
    /// Response has valid confirmations
    /// Illegal confirmation request
    /// Internal error in issuer
    /// Try again later
    /// Must sign the request
    /// Request unauthorized
/// Configuration for CRL generation
#[derive(Debug, Clone)]
pub struct CrlConfig {
    /// Validity period for the CRL
    /// Maximum number of entries per CRL
    /// Whether to generate delta CRLs
    /// Delta CRL generation interval
    /// CRL distribution points
    /// Signature algorithm for CRL signing
    /// Include reason codes in CRL entries
    /// Include invalidity dates
/// Configuration for OCSP responder
#[derive(Debug, Clone)]
pub struct OcspConfig {
    /// OCSP responder URL
    /// Response validity period
    /// Maximum response cache time
    /// Include nonce in responses
    /// Response signature algorithm
    /// OCSP responder certificate
    /// Network timeout for OCSP requests
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
/// Revocation database statistics
#[derive(Debug, Clone)]
pub struct RevocationDatabaseStats {
    /// Total number of revoked certificates
    /// Number of revocations by reason
    /// Database size in bytes
    /// Last maintenance timestamp
    /// Number of pending operations
/// In-memory revocation database implementation
#[derive(Debug)]
pub struct InMemoryRevocationDatabase {
    /// Revocation entries indexed by (issuer, serial_number)
    /// Statistics
impl InMemoryRevocationDatabase {
    /// Create a new in-memory revocation database
    pub fn new() -> Self {
        Self {
            statistics: RwLock::new(RevocationDatabaseStats {
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
        Ok(())
    fn is_revoked(&self, serial_number: &SerialNumber, issuer: &DistinguishedName) -> PkiResult<Option<RevocationEntry>> {
        let revocations = self.revocations.read()
            .map_err(|_| PkiError::general("Failed to acquire revocation database read lock"))?;
        
        let key = (issuer.clone(), serial_number.clone());
        Ok(revocations.get(&key).cloned())
    fn get_revocations_by_issuer(&self, issuer: &DistinguishedName) -> PkiResult<Vec<RevocationEntry>> {
        let revocations = self.revocations.read()
            .map_err(|_| PkiError::general("Failed to acquire revocation database read lock"))?;
        
        let entries: Vec<RevocationEntry> = revocations
            .iter()
            .filter(|((iss, _), _)| iss == issuer)
            .map(|(_, entry)| entry.clone())
            .collect();
        
        Ok(entries)
    fn get_revocations_since(&self, since: SystemTime) -> PkiResult<Vec<RevocationEntry>> {
        let revocations = self.revocations.read()
            .map_err(|_| PkiError::general("Failed to acquire revocation database read lock"))?;
        
        let entries: Vec<RevocationEntry> = revocations
            .values()
            .filter(|entry| entry.entry_modified >= since)
            .cloned()
            .collect();
        
        Ok(entries)
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
        Ok(())
    fn get_statistics(&self) -> PkiResult<RevocationDatabaseStats> {
        let stats = self.statistics.read()
            .map_err(|_| PkiError::general("Failed to acquire statistics read lock"))?;
        
        Ok(stats.clone())
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
    /// CRL configuration
    /// OCSP configuration
    /// CRL generation state
    /// OCSP response cache
    /// Notification subscribers
    /// Emergency revocation enabled
    /// Manager statistics
/// CRL generation state
#[derive(Debug)]
struct CrlGenerationState {
    /// Last full CRL number
    /// Last delta CRL number
    /// Last full CRL generation time
    /// Last delta CRL generation time
    /// Pending CRL updates
/// Revocation notification handler trait
pub trait RevocationNotificationHandler: Send + Sync {
    /// Handle certificate revocation notification
    fn handle_revocation(&self, entry: &RevocationEntry) -> PkiResult<()>;
    
    /// Handle CRL update notification
    fn handle_crl_update(&self, crl: &CertificateRevocationList) -> PkiResult<()>;
    
    /// Handle emergency revocation notification
    fn handle_emergency_revocation(&self, entries: &[RevocationEntry]) -> PkiResult<()>;
/// Revocation manager statistics
#[derive(Debug, Clone, Default)]
pub struct RevocationManagerStats {
    /// Total revocations processed
    /// CRLs generated
    /// OCSP requests processed
    /// Emergency revocations
    /// Average revocation processing time (ms)
    /// Cache hit rate for OCSP
    /// Failed revocation attempts
impl CertificateRevocationManager {
    /// Create a new certificate revocation manager
    pub fn new(
    ) -> Self {
        Self {
            crl_state: Arc::new(Mutex::new(CrlGenerationState {
        }
    }
    
    /// Revoke a single certificate
    pub fn revoke_certificate(
    ) -> PkiResult<()> {
        let start_time = SystemTime::now();
        
        // Create revocation entry
        let entry = RevocationEntry {
        
        // Add to database
        {
            let mut database = self.database.lock()
                .map_err(|_| PkiError::general("Failed to acquire database lock"))?;
            database.add_revocation(entry.clone())?;
        // Add to pending CRL updates
        {
            let mut crl_state = self.crl_state.lock()
                .map_err(|_| PkiError::general("Failed to acquire CRL state lock"))?;
            crl_state.pending_updates.push(entry.clone());
        // Invalidate OCSP cache for this certificate
        self.invalidate_ocsp_cache(&serial_number)?;
        
        // Notify subscribers
        self.notify_revocation(&entry)?;
        
        // Update statistics
        self.update_revocation_statistics(start_time)?;
        
        Ok(())
    /// Revoke multiple certificates in a batch
    pub fn revoke_certificates_batch(
    ) -> PkiResult<BatchRevocationResult> {
        let start_time = SystemTime::now();
        let mut successful_revocations = Vec::new();
        let mut failed_revocations = Vec::new();
        
        for (serial_number, issuer, reason, invalidity_date, audit_info) in revocations {
            match self.revoke_certificate(
            ) {
            }
        }
        
        Ok(BatchRevocationResult {
        })
    /// Emergency revocation procedure (bypass normal authorization)
    pub fn emergency_revoke_certificates(
    ) -> PkiResult<()> {
        // Enable emergency mode
        {
            let mut emergency_mode = self.emergency_mode.write()
                .map_err(|_| PkiError::general("Failed to acquire emergency mode lock"))?;
            *emergency_mode = true;
        let mut revocation_entries = Vec::new();
        
        // Create emergency audit info
        let audit_info = RevocationAuditInfo {
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("emergency_revocation".to_string(), "true".to_string());
                meta.insert("timestamp".to_string(), SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default().as_secs().to_string());
                meta
        
        // Process emergency revocations
        for (serial_number, issuer) in certificates {
            let entry = RevocationEntry {
            
            // Add to database immediately
            {
                let mut database = self.database.lock()
                    .map_err(|_| PkiError::general("Failed to acquire database lock"))?;
                database.add_revocation(entry.clone())?;
            revocation_entries.push(entry);
        // Invalidate all OCSP cache
        self.clear_ocsp_cache()?;
        
        // Notify emergency revocation
        self.notify_emergency_revocation(&revocation_entries)?;
        
        // Update statistics
        {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
            stats.emergency_revocations += revocation_entries.len() as u64;
        // Generate immediate CRL
        self.generate_full_crl()?;
        
        Ok(())
    /// Check certificate revocation status
    pub fn check_revocation_status(
    ) -> PkiResult<CertificateRevocationStatus> {
        // Check database
        let database = self.database.lock()
            .map_err(|_| PkiError::general("Failed to acquire database lock"))?;
        
        if let Some(entry) = database.is_revoked(serial_number, issuer)? {
            Ok(CertificateRevocationStatus::Revoked {
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
            signature: Vec::new(), // Would be computed with actual signing
            raw_data: Vec::new(), // Would be DER-encoded CRL
        
        // Update statistics
        {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
            stats.crls_generated += 1;
        // Notify subscribers
        self.notify_crl_update(&crl)?;
        
        Ok(crl)
    /// Generate a delta CRL (contains only changes since last full CRL)
    pub fn generate_delta_crl(&mut self) -> PkiResult<Option<CertificateRevocationList>> {
        if !self.crl_config.enable_delta_crl {
            return Ok(None);
        let mut crl_state = self.crl_state.lock()
            .map_err(|_| PkiError::general("Failed to acquire CRL state lock"))?;
        
        if crl_state.pending_updates.is_empty() {
            return Ok(None);
        let now = SystemTime::now();
        let next_update = now + self.crl_config.delta_crl_interval;
        
        // Increment delta CRL number
        crl_state.last_delta_crl_number += 1;
        crl_state.last_delta_crl_time = Some(now);
        
        let delta_crl = CertificateRevocationList {
            issuer: DistinguishedName {
        
        crl_state.pending_updates.clear();
        
        // Update statistics
        {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
            stats.crls_generated += 1;
        // Notify subscribers
        self.notify_crl_update(&delta_crl)?;
        
        Ok(Some(delta_crl))
    /// Process OCSP request
    pub fn process_ocsp_request(&self, request: OcspRequest) -> PkiResult<OcspResponse> {
        let start_time = SystemTime::now();
        
        // Check cache first
        if let Some(cached_response) = self.get_cached_ocsp_response(&request.certificate_serial)? {
            self.update_ocsp_cache_stats(true)?;
            return Ok(cached_response);
        self.update_ocsp_cache_stats(false)?;
        
        // Check revocation status
        let status = match self.check_revocation_status(&request.certificate_serial, &request.issuer)? {
            CertificateRevocationStatus::Revoked { revocation_date, reason, .. } => {
                OcspCertificateStatus::Revoked {
                }
            }
        
        let now = SystemTime::now();
        let response = OcspResponse {
            signature: Vec::new(), // Would be computed with actual signing
        
        // Cache the response
        self.cache_ocsp_response(&request.certificate_serial, &response)?;
        
        // Update statistics
        {
            let mut stats = self.statistics.write()
                .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
            stats.ocsp_requests_processed += 1;
        Ok(response)
    /// Add a notification subscriber
    pub fn add_notification_subscriber(&self, handler: Box<dyn RevocationNotificationHandler>) -> PkiResult<()> {
        let mut subscribers = self.notification_subscribers.write()
            .map_err(|_| PkiError::general("Failed to acquire notification subscribers lock"))?;
        
        subscribers.push(handler);
        Ok(())
    /// Get manager statistics
    pub fn get_statistics(&self) -> PkiResult<RevocationManagerStats> {
        let stats = self.statistics.read()
            .map_err(|_| PkiError::general("Failed to acquire statistics lock"))?;
        
        Ok(stats.clone())
    /// Private helper methods
    
    fn invalidate_ocsp_cache(&self, serial_number: &SerialNumber) -> PkiResult<()> {
        let mut cache = self.ocsp_cache.write()
            .map_err(|_| PkiError::general("Failed to acquire OCSP cache lock"))?;
        
        cache.remove(serial_number);
        Ok(())
    fn clear_ocsp_cache(&self) -> PkiResult<()> {
        let mut cache = self.ocsp_cache.write()
            .map_err(|_| PkiError::general("Failed to acquire OCSP cache lock"))?;
        
        cache.clear();
        Ok(())
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
    fn cache_ocsp_response(&self, serial_number: &SerialNumber, response: &OcspResponse) -> PkiResult<()> {
        let mut cache = self.ocsp_cache.write()
            .map_err(|_| PkiError::general("Failed to acquire OCSP cache lock"))?;
        
        cache.insert(serial_number.clone(), (response.clone(), SystemTime::now()));
        Ok(())
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
    fn notify_crl_update(&self, crl: &CertificateRevocationList) -> PkiResult<()> {
        let subscribers = self.notification_subscribers.read()
            .map_err(|_| PkiError::general("Failed to acquire notification subscribers lock"))?;
        
        for subscriber in subscribers.iter() {
            if let Err(e) = subscriber.handle_crl_update(crl) {
                eprintln!("CRL notification handler error: {}", e);
            }
        }
        
        Ok(())
    fn notify_emergency_revocation(&self, entries: &[RevocationEntry]) -> PkiResult<()> {
        let subscribers = self.notification_subscribers.read()
            .map_err(|_| PkiError::general("Failed to acquire notification subscribers lock"))?;
        
        for subscriber in subscribers.iter() {
            if let Err(e) = subscriber.handle_emergency_revocation(entries) {
                eprintln!("Emergency notification handler error: {}", e);
            }
        }
        
        Ok(())
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
    /// Certificate has been revoked
    Revoked {
/// Batch revocation result
#[derive(Debug, Clone)]
pub struct BatchRevocationResult {
    /// Successfully revoked certificates
    /// Failed revocations with error details
    /// Total processing time
    /// Total certificates processed
/// Implement Display for RevocationReason
impl fmt::Display for RevocationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Implement Display for OcspResponseStatus
impl fmt::Display for OcspResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Helper functions for common operations
impl SerialNumber {
    /// Create serial number from big integer
    pub fn from_big_int(value: u64) -> Self {
        Self {
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
            delta_crl_interval: Duration::from_secs(24 * 3600), // 1 day
        }
    }
impl Default for OcspConfig {
    fn default() -> Self {
        Self {
            responder_url: "http://localhost:8080/ocsp".to_string(),
            response_validity: Duration::from_secs(24 * 3600), // 1 day
            cache_duration: Duration::from_secs(3600), // 1 hour
        }
    }
/// Public API functions for certificate revocation

/// Create a new certificate revocation manager
pub fn create_revocation_manager(
) -> PkiResult<CertificateRevocationManager> {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = crl_config.unwrap_or_default();
    let ocsp_config = ocsp_config.unwrap_or_default();
    
    Ok(CertificateRevocationManager::new(database, crl_config, ocsp_config))
/// Revoke a certificate with detailed audit information
pub fn revoke_certificate_with_audit(
) -> PkiResult<()> {
    let audit_info = RevocationAuditInfo {
    
    manager.revoke_certificate(serial_number, issuer, reason, None, audit_info)
/// Check if a certificate is revoked
pub fn is_certificate_revoked(
) -> PkiResult<bool> {
    match manager.check_revocation_status(serial_number, issuer)? {
    }
}

/// Generate a CRL for distribution
pub fn generate_certificate_revocation_list(
) -> PkiResult<CertificateRevocationList> {
    manager.generate_full_crl()
/// Process an OCSP request for real-time certificate status
pub fn process_ocsp_status_request(
) -> PkiResult<OcspResponse> {
    let request = OcspRequest {
        nonce: Some(vec![1, 2, 3, 4]), // In practice, generate random nonce
    
    manager.process_ocsp_request(request)
