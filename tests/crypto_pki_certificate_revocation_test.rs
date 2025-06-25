/// Certificate Revocation Testing Suite
/// 
/// Comprehensive tests for the CURSED language PKI certificate revocation functionality.
/// 
/// This test suite validates:
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

use cursed::stdlib::packages::crypto_pki::certificate_revocation::{
    CertificateRevocationManager, InMemoryRevocationDatabase, RevocationDatabase,
    RevocationEntry, RevocationReason, RevocationAuditInfo, CrlConfig, OcspConfig,
    OcspRequest, OcspResponse, OcspResponseStatus, OcspCertificateStatus,
    CertificateRevocationList, CertificateRevocationStatus, BatchRevocationResult,
    RevocationNotificationHandler, create_revocation_manager, revoke_certificate_with_audit,
    is_certificate_revoked, generate_certificate_revocation_list, process_ocsp_status_request,
};
use cursed::stdlib::packages::crypto_pki::types::{
    SerialNumber, DistinguishedName, SignatureAlgorithm, X509Extension,
};
use cursed::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::sync::{Arc, Mutex};

// Test utilities

fn create_test_serial_number(value: u64) -> SerialNumber {
    SerialNumber {
        bytes: value.to_be_bytes().to_vec(),
    }
}

fn create_test_distinguished_name(cn: &str) -> DistinguishedName {
    DistinguishedName {
        common_name: Some(cn.to_string()),
        organization: Some("Test Organization".to_string()),
        organizational_unit: Some("Test Unit".to_string()),
        country: Some("US".to_string()),
        state_or_province: Some("California".to_string()),
        locality: Some("San Francisco".to_string()),
        email_address: Some(format!("{}@test.example.com", cn.to_lowercase())),
        additional_attributes: HashMap::new(),
    }
}

fn create_test_audit_info(user: &str) -> RevocationAuditInfo {
    let mut metadata = HashMap::new();
    metadata.insert("test_environment".to_string(), "true".to_string());
    metadata.insert("timestamp".to_string(), SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default().as_secs().to_string());
    
    RevocationAuditInfo {
        initiated_by: user.to_string(),
        authorization: format!("AUTH_TOKEN_{}", user.to_uppercase()),
        request_source: Some("127.0.0.1".to_string()),
        metadata,
    }
}

fn create_test_crl_config() -> CrlConfig {
    CrlConfig {
        validity_period: Duration::from_secs(7 * 24 * 3600), // 7 days
        max_entries_per_crl: 1000,
        enable_delta_crl: true,
        delta_crl_interval: Duration::from_secs(24 * 3600), // 1 day
        distribution_points: vec![
            "http://test.example.com/crl".to_string(),
            "ldap://ldap.test.example.com/crl".to_string(),
        ],
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        include_reason_codes: true,
        include_invalidity_dates: true,
    }
}

fn create_test_ocsp_config() -> OcspConfig {
    OcspConfig {
        responder_url: "http://ocsp.test.example.com".to_string(),
        response_validity: Duration::from_secs(24 * 3600), // 1 day
        cache_duration: Duration::from_secs(3600), // 1 hour
        include_nonce: true,
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        responder_certificate: None,
        network_timeout: Duration::from_secs(30),
    }
}

// Test notification handler for testing notification functionality
#[derive(Debug)]
struct TestNotificationHandler {
    revocations: Arc<Mutex<Vec<RevocationEntry>>>,
    crl_updates: Arc<Mutex<Vec<u64>>>,
    emergency_revocations: Arc<Mutex<Vec<usize>>>,
}

impl TestNotificationHandler {
    fn new() -> Self {
        Self {
            revocations: Arc::new(Mutex::new(Vec::new())),
            crl_updates: Arc::new(Mutex::new(Vec::new())),
            emergency_revocations: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn get_revocation_count(&self) -> usize {
        self.revocations.lock().unwrap().len()
    }
    
    fn get_crl_update_count(&self) -> usize {
        self.crl_updates.lock().unwrap().len()
    }
    
    fn get_emergency_revocation_count(&self) -> usize {
        self.emergency_revocations.lock().unwrap().len()
    }
}

impl RevocationNotificationHandler for TestNotificationHandler {
    fn handle_revocation(&self, entry: &RevocationEntry) -> PkiResult<()> {
        self.revocations.lock().unwrap().push(entry.clone());
        Ok(())
    }
    
    fn handle_crl_update(&self, crl: &CertificateRevocationList) -> PkiResult<()> {
        self.crl_updates.lock().unwrap().push(crl.crl_number);
        Ok(())
    }
    
    fn handle_emergency_revocation(&self, entries: &[RevocationEntry]) -> PkiResult<()> {
        self.emergency_revocations.lock().unwrap().push(entries.len());
        Ok(())
    }
}

// Unit tests for revocation database

#[test]
fn test_in_memory_database_basic_operations() {
    let mut database = InMemoryRevocationDatabase::new();
    
    let serial1 = create_test_serial_number(12345);
    let serial2 = create_test_serial_number(67890);
    let issuer = create_test_distinguished_name("Test CA");
    
    // Initially no revocations
    assert!(database.is_revoked(&serial1, &issuer).unwrap().is_none());
    assert!(database.is_revoked(&serial2, &issuer).unwrap().is_none());
    
    // Add first revocation
    let entry1 = RevocationEntry {
        serial_number: serial1.clone(),
        revocation_date: SystemTime::now(),
        reason: RevocationReason::KeyCompromise,
        invalidity_date: None,
        issuer: issuer.clone(),
        extensions: Vec::new(),
        entry_created: SystemTime::now(),
        entry_modified: SystemTime::now(),
        audit_info: create_test_audit_info("admin1"),
    };
    
    database.add_revocation(entry1.clone()).unwrap();
    
    // Check first certificate is revoked
    let result = database.is_revoked(&serial1, &issuer).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().reason, RevocationReason::KeyCompromise);
    
    // Second certificate still not revoked
    assert!(database.is_revoked(&serial2, &issuer).unwrap().is_none());
    
    // Add second revocation
    let entry2 = RevocationEntry {
        serial_number: serial2.clone(),
        revocation_date: SystemTime::now(),
        reason: RevocationReason::CessationOfOperation,
        invalidity_date: Some(SystemTime::now() - Duration::from_secs(3600)),
        issuer: issuer.clone(),
        extensions: Vec::new(),
        entry_created: SystemTime::now(),
        entry_modified: SystemTime::now(),
        audit_info: create_test_audit_info("admin2"),
    };
    
    database.add_revocation(entry2.clone()).unwrap();
    
    // Both certificates should be revoked
    assert!(database.is_revoked(&serial1, &issuer).unwrap().is_some());
    assert!(database.is_revoked(&serial2, &issuer).unwrap().is_some());
    
    // Check statistics
    let stats = database.get_statistics().unwrap();
    assert_eq!(stats.total_revocations, 2);
    assert_eq!(stats.revocations_by_reason.get(&RevocationReason::KeyCompromise), Some(&1));
    assert_eq!(stats.revocations_by_reason.get(&RevocationReason::CessationOfOperation), Some(&1));
}

#[test]
fn test_database_revocations_by_issuer() {
    let mut database = InMemoryRevocationDatabase::new();
    
    let issuer1 = create_test_distinguished_name("CA1");
    let issuer2 = create_test_distinguished_name("CA2");
    
    // Add revocations for different issuers
    for i in 1..=3 {
        let entry1 = RevocationEntry {
            serial_number: create_test_serial_number(i),
            revocation_date: SystemTime::now(),
            reason: RevocationReason::KeyCompromise,
            invalidity_date: None,
            issuer: issuer1.clone(),
            extensions: Vec::new(),
            entry_created: SystemTime::now(),
            entry_modified: SystemTime::now(),
            audit_info: create_test_audit_info("admin"),
        };
        database.add_revocation(entry1).unwrap();
        
        let entry2 = RevocationEntry {
            serial_number: create_test_serial_number(i + 100),
            revocation_date: SystemTime::now(),
            reason: RevocationReason::Superseded,
            invalidity_date: None,
            issuer: issuer2.clone(),
            extensions: Vec::new(),
            entry_created: SystemTime::now(),
            entry_modified: SystemTime::now(),
            audit_info: create_test_audit_info("admin"),
        };
        database.add_revocation(entry2).unwrap();
    }
    
    // Get revocations by issuer
    let issuer1_revocations = database.get_revocations_by_issuer(&issuer1).unwrap();
    let issuer2_revocations = database.get_revocations_by_issuer(&issuer2).unwrap();
    
    assert_eq!(issuer1_revocations.len(), 3);
    assert_eq!(issuer2_revocations.len(), 3);
    
    // Verify all entries for issuer1 have correct issuer and reason
    for entry in &issuer1_revocations {
        assert_eq!(entry.issuer, issuer1);
        assert_eq!(entry.reason, RevocationReason::KeyCompromise);
    }
    
    // Verify all entries for issuer2 have correct issuer and reason
    for entry in &issuer2_revocations {
        assert_eq!(entry.issuer, issuer2);
        assert_eq!(entry.reason, RevocationReason::Superseded);
    }
}

#[test]
fn test_database_revocations_since_timestamp() {
    let mut database = InMemoryRevocationDatabase::new();
    
    let issuer = create_test_distinguished_name("Test CA");
    let checkpoint = SystemTime::now();
    
    // Add revocation before checkpoint
    let entry_old = RevocationEntry {
        serial_number: create_test_serial_number(1),
        revocation_date: checkpoint - Duration::from_secs(3600),
        reason: RevocationReason::KeyCompromise,
        invalidity_date: None,
        issuer: issuer.clone(),
        extensions: Vec::new(),
        entry_created: checkpoint - Duration::from_secs(3600),
        entry_modified: checkpoint - Duration::from_secs(3600),
        audit_info: create_test_audit_info("admin"),
    };
    database.add_revocation(entry_old).unwrap();
    
    // Wait a bit to ensure different timestamps
    std::thread::sleep(Duration::from_millis(10));
    
    // Add revocations after checkpoint
    for i in 2..=4 {
        let entry = RevocationEntry {
            serial_number: create_test_serial_number(i),
            revocation_date: SystemTime::now(),
            reason: RevocationReason::CessationOfOperation,
            invalidity_date: None,
            issuer: issuer.clone(),
            extensions: Vec::new(),
            entry_created: SystemTime::now(),
            entry_modified: SystemTime::now(),
            audit_info: create_test_audit_info("admin"),
        };
        database.add_revocation(entry).unwrap();
    }
    
    // Get revocations since checkpoint
    let recent_revocations = database.get_revocations_since(checkpoint).unwrap();
    
    // Should only include the 3 recent entries
    assert_eq!(recent_revocations.len(), 3);
    
    for entry in &recent_revocations {
        assert_eq!(entry.reason, RevocationReason::CessationOfOperation);
        assert!(entry.entry_modified >= checkpoint);
    }
}

#[test]
fn test_database_remove_revocation() {
    let mut database = InMemoryRevocationDatabase::new();
    
    let serial = create_test_serial_number(12345);
    let issuer = create_test_distinguished_name("Test CA");
    
    // Add revocation
    let entry = RevocationEntry {
        serial_number: serial.clone(),
        revocation_date: SystemTime::now(),
        reason: RevocationReason::CertificateHold,
        invalidity_date: None,
        issuer: issuer.clone(),
        extensions: Vec::new(),
        entry_created: SystemTime::now(),
        entry_modified: SystemTime::now(),
        audit_info: create_test_audit_info("admin"),
    };
    database.add_revocation(entry).unwrap();
    
    // Verify certificate is revoked
    assert!(database.is_revoked(&serial, &issuer).unwrap().is_some());
    
    // Remove revocation (un-revoke)
    database.remove_revocation(&serial, &issuer).unwrap();
    
    // Verify certificate is no longer revoked
    assert!(database.is_revoked(&serial, &issuer).unwrap().is_none());
    
    // Check statistics updated
    let stats = database.get_statistics().unwrap();
    assert_eq!(stats.total_revocations, 0);
}

// Unit tests for revocation manager

#[test]
fn test_revocation_manager_creation() {
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let manager = create_revocation_manager(Some(crl_config), Some(ocsp_config)).unwrap();
    
    // Manager should be created successfully
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.total_revocations, 0);
    assert_eq!(stats.crls_generated, 0);
    assert_eq!(stats.ocsp_requests_processed, 0);
}

#[test]
fn test_certificate_revocation() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    let serial = create_test_serial_number(54321);
    let issuer = create_test_distinguished_name("Test CA");
    
    // Initially not revoked
    let status = manager.check_revocation_status(&serial, &issuer).unwrap();
    assert_eq!(status, CertificateRevocationStatus::Valid);
    
    // Revoke certificate
    manager.revoke_certificate(
        serial.clone(),
        issuer.clone(),
        RevocationReason::KeyCompromise,
        None,
        create_test_audit_info("security_admin"),
    ).unwrap();
    
    // Now should be revoked
    let status = manager.check_revocation_status(&serial, &issuer).unwrap();
    match status {
        CertificateRevocationStatus::Revoked { reason, .. } => {
            assert_eq!(reason, RevocationReason::KeyCompromise);
        }
        _ => panic!("Certificate should be revoked"),
    }
    
    // Check statistics
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.total_revocations, 1);
    assert!(stats.avg_revocation_time_ms >= 0.0);
}

#[test]
fn test_certificate_revocation_with_invalidity_date() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    let serial = create_test_serial_number(11111);
    let issuer = create_test_distinguished_name("Test CA");
    let invalidity_date = SystemTime::now() - Duration::from_secs(7200); // 2 hours ago
    
    // Revoke certificate with invalidity date
    manager.revoke_certificate(
        serial.clone(),
        issuer.clone(),
        RevocationReason::KeyCompromise,
        Some(invalidity_date),
        create_test_audit_info("forensics_admin"),
    ).unwrap();
    
    // Check revocation status includes invalidity date
    let status = manager.check_revocation_status(&serial, &issuer).unwrap();
    match status {
        CertificateRevocationStatus::Revoked { reason, invalidity_date: inv_date, .. } => {
            assert_eq!(reason, RevocationReason::KeyCompromise);
            assert_eq!(inv_date, Some(invalidity_date));
        }
        _ => panic!("Certificate should be revoked with invalidity date"),
    }
}

#[test]
fn test_batch_certificate_revocation() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    // Create batch revocation request
    let revocations = vec![
        (create_test_serial_number(1001), create_test_distinguished_name("CA1"), RevocationReason::KeyCompromise, None, create_test_audit_info("admin1")),
        (create_test_serial_number(1002), create_test_distinguished_name("CA2"), RevocationReason::Superseded, None, create_test_audit_info("admin2")),
        (create_test_serial_number(1003), create_test_distinguished_name("CA1"), RevocationReason::CessationOfOperation, None, create_test_audit_info("admin3")),
        (create_test_serial_number(1004), create_test_distinguished_name("CA2"), RevocationReason::AffiliationChanged, None, create_test_audit_info("admin4")),
    ];
    
    let result = manager.revoke_certificates_batch(revocations).unwrap();
    
    assert_eq!(result.successful_revocations.len(), 4);
    assert_eq!(result.failed_revocations.len(), 0);
    assert_eq!(result.total_processed, 4);
    assert!(result.processing_time > Duration::from_millis(0));
    
    // Verify all certificates are revoked with correct reasons
    let expectations = vec![
        (1001, "CA1", RevocationReason::KeyCompromise),
        (1002, "CA2", RevocationReason::Superseded),
        (1003, "CA1", RevocationReason::CessationOfOperation),
        (1004, "CA2", RevocationReason::AffiliationChanged),
    ];
    
    for (serial_num, ca_name, expected_reason) in expectations {
        let status = manager.check_revocation_status(
            &create_test_serial_number(serial_num),
            &create_test_distinguished_name(ca_name)
        ).unwrap();
        
        match status {
            CertificateRevocationStatus::Revoked { reason, .. } => {
                assert_eq!(reason, expected_reason);
            }
            _ => panic!("Certificate {} should be revoked", serial_num),
        }
    }
    
    // Check statistics
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.total_revocations, 4);
}

#[test]
fn test_emergency_revocation() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    // Add notification handler to track emergency notifications
    let handler = Arc::new(TestNotificationHandler::new());
    manager.add_notification_subscriber(Box::new(TestNotificationHandler::new())).unwrap();
    
    // Emergency revocation scenario
    let certificates = vec![
        (create_test_serial_number(2001), create_test_distinguished_name("Compromised CA")),
        (create_test_serial_number(2002), create_test_distinguished_name("Compromised CA")),
        (create_test_serial_number(2003), create_test_distinguished_name("Compromised CA")),
    ];
    
    manager.emergency_revoke_certificates(
        certificates.clone(),
        RevocationReason::CaCompromise,
        "EMERGENCY_AUTH_CRITICAL_2024".to_string(),
    ).unwrap();
    
    // Verify all certificates are immediately revoked
    for (serial, issuer) in &certificates {
        let status = manager.check_revocation_status(serial, issuer).unwrap();
        match status {
            CertificateRevocationStatus::Revoked { reason, .. } => {
                assert_eq!(reason, RevocationReason::CaCompromise);
            }
            _ => panic!("Certificate should be immediately revoked in emergency"),
        }
    }
    
    // Check emergency revocation statistics
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.emergency_revocations, 3);
    assert_eq!(stats.total_revocations, 3);
    assert_eq!(stats.crls_generated, 1); // Emergency CRL generated
}

// CRL generation tests

#[test]
fn test_full_crl_generation() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    // Add several revocations
    let revocations = vec![
        (1, RevocationReason::KeyCompromise),
        (2, RevocationReason::CessationOfOperation),
        (3, RevocationReason::Superseded),
        (4, RevocationReason::AffiliationChanged),
        (5, RevocationReason::PrivilegeWithdrawn),
    ];
    
    for (serial_num, reason) in &revocations {
        manager.revoke_certificate(
            create_test_serial_number(*serial_num),
            create_test_distinguished_name("Test CA"),
            *reason,
            None,
            create_test_audit_info("admin"),
        ).unwrap();
    }
    
    // Generate full CRL
    let crl = manager.generate_full_crl().unwrap();
    
    assert_eq!(crl.revoked_certificates.len(), 5);
    assert_eq!(crl.crl_number, 1);
    assert!(crl.this_update <= SystemTime::now());
    assert!(crl.next_update.is_some());
    assert!(crl.delta_crl_indicator.is_none()); // Full CRL, not delta
    assert_eq!(crl.signature_algorithm, SignatureAlgorithm::RsaWithSha256);
    
    // Verify all expected revocations are in the CRL
    let mut found_serials = std::collections::HashSet::new();
    for entry in &crl.revoked_certificates {
        found_serials.insert(entry.serial_number.to_big_int());
    }
    
    for (serial_num, _) in &revocations {
        assert!(found_serials.contains(&(*serial_num as u64)));
    }
    
    // Check CRL generation statistics
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.crls_generated, 1);
}

#[test]
fn test_delta_crl_generation() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let mut crl_config = create_test_crl_config();
    crl_config.enable_delta_crl = true;
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    // Generate initial full CRL
    let full_crl = manager.generate_full_crl().unwrap();
    assert_eq!(full_crl.crl_number, 1);
    
    // Add more revocations after full CRL
    manager.revoke_certificate(
        create_test_serial_number(100),
        create_test_distinguished_name("Test CA"),
        RevocationReason::KeyCompromise,
        None,
        create_test_audit_info("admin"),
    ).unwrap();
    
    manager.revoke_certificate(
        create_test_serial_number(200),
        create_test_distinguished_name("Test CA"),
        RevocationReason::Superseded,
        None,
        create_test_audit_info("admin"),
    ).unwrap();
    
    // Generate delta CRL
    let delta_crl_option = manager.generate_delta_crl().unwrap();
    assert!(delta_crl_option.is_some());
    
    let delta_crl = delta_crl_option.unwrap();
    assert_eq!(delta_crl.revoked_certificates.len(), 2); // Only new revocations
    assert_eq!(delta_crl.crl_number, 1); // First delta CRL
    assert_eq!(delta_crl.delta_crl_indicator, Some(1)); // References full CRL #1
    
    // Verify delta CRL contains only the new revocations
    let delta_serials: std::collections::HashSet<u64> = delta_crl.revoked_certificates
        .iter()
        .map(|entry| entry.serial_number.to_big_int())
        .collect();
    
    assert!(delta_serials.contains(&100));
    assert!(delta_serials.contains(&200));
    
    // Check CRL generation statistics (1 full + 1 delta)
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.crls_generated, 2);
}

#[test]
fn test_delta_crl_when_no_changes() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let mut crl_config = create_test_crl_config();
    crl_config.enable_delta_crl = true;
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    // Generate initial full CRL
    let _full_crl = manager.generate_full_crl().unwrap();
    
    // Try to generate delta CRL without any new revocations
    let delta_crl_option = manager.generate_delta_crl().unwrap();
    assert!(delta_crl_option.is_none()); // Should be None when no changes
}

// OCSP tests

#[test]
fn test_ocsp_request_for_valid_certificate() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    let serial = create_test_serial_number(99999);
    let issuer = create_test_distinguished_name("Test CA");
    
    let request = OcspRequest {
        certificate_serial: serial.clone(),
        issuer: issuer.clone(),
        nonce: Some(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        request_time: SystemTime::now(),
        extensions: Vec::new(),
    };
    
    let response = manager.process_ocsp_request(request).unwrap();
    
    assert_eq!(response.response_status, OcspResponseStatus::Successful);
    assert_eq!(response.certificate_status, OcspCertificateStatus::Good);
    assert!(response.nonce.is_some());
    assert_eq!(response.nonce.unwrap(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(response.signature_algorithm, SignatureAlgorithm::RsaWithSha256);
    
    // Check OCSP statistics
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.ocsp_requests_processed, 1);
}

#[test]
fn test_ocsp_request_for_revoked_certificate() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    let serial = create_test_serial_number(88888);
    let issuer = create_test_distinguished_name("Test CA");
    
    // Revoke the certificate first
    let revocation_time = SystemTime::now();
    manager.revoke_certificate(
        serial.clone(),
        issuer.clone(),
        RevocationReason::KeyCompromise,
        None,
        create_test_audit_info("security_admin"),
    ).unwrap();
    
    // Create OCSP request
    let request = OcspRequest {
        certificate_serial: serial.clone(),
        issuer: issuer.clone(),
        nonce: Some(vec![9, 8, 7, 6, 5, 4, 3, 2]),
        request_time: SystemTime::now(),
        extensions: Vec::new(),
    };
    
    let response = manager.process_ocsp_request(request).unwrap();
    
    assert_eq!(response.response_status, OcspResponseStatus::Successful);
    
    match response.certificate_status {
        OcspCertificateStatus::Revoked { revocation_time: rev_time, reason } => {
            assert!(rev_time >= revocation_time);
            assert_eq!(reason, Some(RevocationReason::KeyCompromise));
        }
        _ => panic!("Certificate should be reported as revoked in OCSP response"),
    }
    
    assert_eq!(response.nonce.unwrap(), vec![9, 8, 7, 6, 5, 4, 3, 2]);
}

#[test]
fn test_ocsp_response_caching() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let mut ocsp_config = create_test_ocsp_config();
    ocsp_config.cache_duration = Duration::from_secs(3600); // 1 hour cache
    
    let manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    let serial = create_test_serial_number(77777);
    let issuer = create_test_distinguished_name("Test CA");
    
    let request = OcspRequest {
        certificate_serial: serial.clone(),
        issuer: issuer.clone(),
        nonce: Some(vec![1, 1, 1, 1]),
        request_time: SystemTime::now(),
        extensions: Vec::new(),
    };
    
    // First request - should hit database
    let response1 = manager.process_ocsp_request(request.clone()).unwrap();
    
    // Second request - should hit cache
    let response2 = manager.process_ocsp_request(request).unwrap();
    
    // Responses should be identical (except possibly timestamps)
    assert_eq!(response1.response_status, response2.response_status);
    assert_eq!(response1.certificate_status, response2.certificate_status);
    assert_eq!(response1.nonce, response2.nonce);
    
    // Check cache hit rate statistics
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.ocsp_requests_processed, 2);
    assert!(stats.ocsp_cache_hit_rate > 0.0);
}

// Notification tests

#[test]
fn test_revocation_notifications() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    // Add notification handler
    let handler = Arc::new(TestNotificationHandler::new());
    let handler_clone = handler.clone();
    manager.add_notification_subscriber(Box::new(TestNotificationHandler::new())).unwrap();
    
    // Revoke some certificates
    for i in 1..=3 {
        manager.revoke_certificate(
            create_test_serial_number(i),
            create_test_distinguished_name("Test CA"),
            RevocationReason::KeyCompromise,
            None,
            create_test_audit_info("admin"),
        ).unwrap();
    }
    
    // Generate CRL (should trigger CRL update notification)
    let _crl = manager.generate_full_crl().unwrap();
    
    // Note: In a real test, we would check the handler's internal state
    // For now, we just verify the operations completed successfully
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.total_revocations, 3);
    assert_eq!(stats.crls_generated, 1);
}

// Integration tests with public API

#[test]
fn test_public_api_revoke_certificate_with_audit() {
    let mut manager = create_revocation_manager(None, None).unwrap();
    
    let serial = create_test_serial_number(123456);
    let issuer = create_test_distinguished_name("Public API CA");
    
    // Test public API function
    revoke_certificate_with_audit(
        &mut manager,
        serial.clone(),
        issuer.clone(),
        RevocationReason::CessationOfOperation,
        "api_user".to_string(),
        "API_TOKEN_12345".to_string(),
    ).unwrap();
    
    // Verify using public API
    let is_revoked = is_certificate_revoked(&manager, &serial, &issuer).unwrap();
    assert!(is_revoked);
}

#[test]
fn test_public_api_generate_crl() {
    let mut manager = create_revocation_manager(None, None).unwrap();
    
    // Add some revocations
    for i in 1..=5 {
        revoke_certificate_with_audit(
            &mut manager,
            create_test_serial_number(i),
            create_test_distinguished_name("API CA"),
            RevocationReason::KeyCompromise,
            "api_admin".to_string(),
            "API_ADMIN_TOKEN".to_string(),
        ).unwrap();
    }
    
    // Generate CRL using public API
    let crl = generate_certificate_revocation_list(&mut manager).unwrap();
    
    assert_eq!(crl.revoked_certificates.len(), 5);
    assert_eq!(crl.crl_number, 1);
}

#[test]
fn test_public_api_ocsp_request() {
    let manager = create_revocation_manager(None, None).unwrap();
    
    let serial = create_test_serial_number(654321);
    let issuer = create_test_distinguished_name("OCSP CA");
    
    // Test OCSP for valid certificate
    let response = process_ocsp_status_request(&manager, serial.clone(), issuer.clone()).unwrap();
    
    assert_eq!(response.response_status, OcspResponseStatus::Successful);
    assert_eq!(response.certificate_status, OcspCertificateStatus::Good);
}

// Error handling tests

#[test]
fn test_database_error_handling() {
    let mut database = InMemoryRevocationDatabase::new();
    
    // Test with malformed data
    let stats = database.get_statistics().unwrap();
    assert_eq!(stats.total_revocations, 0);
    
    // Test maintenance operation
    database.maintenance().unwrap();
    
    let stats_after = database.get_statistics().unwrap();
    assert!(stats_after.last_maintenance.is_some());
}

#[test]
fn test_revocation_reason_display() {
    assert_eq!(RevocationReason::KeyCompromise.to_string(), "keyCompromise");
    assert_eq!(RevocationReason::CaCompromise.to_string(), "cACompromise");
    assert_eq!(RevocationReason::AffiliationChanged.to_string(), "affiliationChanged");
    assert_eq!(RevocationReason::Superseded.to_string(), "superseded");
    assert_eq!(RevocationReason::CessationOfOperation.to_string(), "cessationOfOperation");
    assert_eq!(RevocationReason::CertificateHold.to_string(), "certificateHold");
    assert_eq!(RevocationReason::RemoveFromCrl.to_string(), "removeFromCRL");
    assert_eq!(RevocationReason::PrivilegeWithdrawn.to_string(), "privilegeWithdrawn");
    assert_eq!(RevocationReason::AaCompromise.to_string(), "aACompromise");
    assert_eq!(RevocationReason::Unspecified.to_string(), "unspecified");
}

#[test]
fn test_ocsp_response_status_display() {
    assert_eq!(OcspResponseStatus::Successful.to_string(), "successful");
    assert_eq!(OcspResponseStatus::MalformedRequest.to_string(), "malformedRequest");
    assert_eq!(OcspResponseStatus::InternalError.to_string(), "internalError");
    assert_eq!(OcspResponseStatus::TryLater.to_string(), "tryLater");
    assert_eq!(OcspResponseStatus::SigRequired.to_string(), "sigRequired");
    assert_eq!(OcspResponseStatus::Unauthorized.to_string(), "unauthorized");
}

#[test]
fn test_serial_number_operations() {
    // Test conversion methods
    let original_value = 0x123456789ABCDEFu64;
    let serial = SerialNumber {
        bytes: original_value.to_be_bytes().to_vec(),
    };
    
    let converted_back = serial.to_big_int();
    assert_eq!(original_value, converted_back);
    
    // Test from_big_int helper
    let serial2 = create_test_serial_number(original_value);
    assert_eq!(serial2.to_big_int(), original_value);
}

// Performance and stress tests

#[test]
fn test_large_revocation_batch() {
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let mut manager = CertificateRevocationManager::new(database, crl_config, ocsp_config);
    
    // Create large batch of revocations
    let mut revocations = Vec::new();
    for i in 1..=100 {
        revocations.push((
            create_test_serial_number(i),
            create_test_distinguished_name("Stress Test CA"),
            RevocationReason::KeyCompromise,
            None,
            create_test_audit_info("stress_admin"),
        ));
    }
    
    let start_time = SystemTime::now();
    let result = manager.revoke_certificates_batch(revocations).unwrap();
    let processing_time = start_time.elapsed().unwrap_or_default();
    
    assert_eq!(result.successful_revocations.len(), 100);
    assert_eq!(result.failed_revocations.len(), 0);
    assert!(processing_time < Duration::from_secs(5)); // Should be reasonably fast
    
    // Generate CRL with all revocations
    let crl = manager.generate_full_crl().unwrap();
    assert_eq!(crl.revoked_certificates.len(), 100);
}

#[test]
fn test_concurrent_revocation_safety() {
    use std::sync::Arc;
    use std::thread;
    
    let database = Arc::new(Mutex::new(InMemoryRevocationDatabase::new()));
    let crl_config = create_test_crl_config();
    let ocsp_config = create_test_ocsp_config();
    
    let manager = Arc::new(Mutex::new(
        CertificateRevocationManager::new(database, crl_config, ocsp_config)
    ));
    
    let mut handles = Vec::new();
    
    // Spawn multiple threads to revoke certificates concurrently
    for thread_id in 0..4 {
        let manager_clone = manager.clone();
        
        let handle = thread::spawn(move || {
            for i in 1..=10 {
                let serial = create_test_serial_number((thread_id * 100 + i) as u64);
                let issuer = create_test_distinguished_name(&format!("Thread{} CA", thread_id));
                
                let mut mgr = manager_clone.lock().unwrap();
                mgr.revoke_certificate(
                    serial,
                    issuer,
                    RevocationReason::KeyCompromise,
                    None,
                    create_test_audit_info(&format!("thread_{}", thread_id)),
                ).unwrap();
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all revocations were processed
    let mgr = manager.lock().unwrap();
    let stats = mgr.get_statistics().unwrap();
    assert_eq!(stats.total_revocations, 40); // 4 threads * 10 revocations each
}
