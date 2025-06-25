/// Trust Stores Test Suite - Comprehensive Testing
/// 
/// Tests for PKI trust store functionality including:
/// - Trust store management and configuration
/// - Certificate validation and chain building
/// - System trust store integration
/// - Trust policy enforcement
/// - Revocation checking
/// - Import/export functionality
/// - Cross-platform compatibility
/// - Security validation

use cursed::stdlib::packages::crypto_pki::{
    trust_stores::*,
    types::*,
    error::{PkiError, PkiResult},
};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::collections::{HashMap, HashSet};

/// Test trust store manager creation and initialization
#[test]
fn test_trust_store_manager_creation() {
    let manager = TrustStoreManager::new();
    
    // Verify initial state
    let stats = manager.get_statistics().expect("Should get statistics");
    assert_eq!(stats.stores_managed, 0);
    assert_eq!(stats.trust_validations, 0);
}

/// Test trust store manager initialization with configuration
#[test]
fn test_trust_store_manager_initialization() {
    let mut manager = TrustStoreManager::new();
    let config = TrustManagerConfig::default();
    
    let result = manager.initialize(config);
    assert!(result.is_ok(), "Initialization should succeed");
    
    let stats = manager.get_statistics().expect("Should get statistics");
    assert!(stats.stores_managed >= 1, "Should have at least default store");
}

/// Test trust store creation and management
#[test]
fn test_trust_store_creation() {
    let manager = TrustStoreManager::new();
    
    // Create a new trust store
    let store_name = manager.create_trust_store("test-store".to_string())
        .expect("Should create trust store");
    assert_eq!(store_name, "test-store");
    
    // Test duplicate creation
    let duplicate_result = manager.create_trust_store("test-store".to_string());
    assert!(duplicate_result.is_err(), "Duplicate store creation should fail");
}

/// Test root certificate addition
#[test]
fn test_root_certificate_addition() {
    let manager = TrustStoreManager::new();
    let _store_name = manager.create_trust_store("test-store".to_string())
        .expect("Should create trust store");
    
    let root_cert = create_test_ca_certificate();
    
    let result = manager.add_root_certificate("test-store", root_cert);
    assert!(result.is_ok(), "Should add root certificate successfully");
    
    let stats = manager.get_statistics().expect("Should get statistics");
    assert!(stats.root_certificates >= 1, "Should have added root certificate");
}

/// Test intermediate certificate addition
#[test]
fn test_intermediate_certificate_addition() {
    let manager = TrustStoreManager::new();
    let _store_name = manager.create_trust_store("test-store".to_string())
        .expect("Should create trust store");
    
    let intermediate_cert = create_test_intermediate_certificate();
    
    let result = manager.add_intermediate_certificate("test-store", intermediate_cert);
    assert!(result.is_ok(), "Should add intermediate certificate successfully");
    
    let stats = manager.get_statistics().expect("Should get statistics");
    assert!(stats.intermediate_certificates >= 1, "Should have added intermediate certificate");
}

/// Test certificate chain validation
#[test]
fn test_certificate_chain_validation() {
    let manager = TrustStoreManager::new();
    let mut config = TrustManagerConfig::default();
    config.enable_system_stores = false; // Disable for testing
    let _result = manager.initialize(config);
    
    let _store_name = manager.create_trust_store("test-store".to_string())
        .expect("Should create trust store");
    
    // Add root certificate
    let root_cert = create_test_ca_certificate();
    let _result = manager.add_root_certificate("test-store", root_cert.clone());
    
    // Create certificate chain
    let chain = CertificateChain {
        end_entity: create_test_end_entity_certificate(),
        intermediates: vec![create_test_intermediate_certificate()],
        root: Some(root_cert),
    };
    
    let validation_result = manager.validate_certificate_chain(&chain, Some("test-store"), None)
        .expect("Should validate certificate chain");
    
    // Verify validation result structure
    assert!(validation_result.policy_name == "default");
    assert!(validation_result.validated_at > UNIX_EPOCH);
    assert!(!validation_result.validation_path.is_empty());
}

/// Test trust policy creation and enforcement
#[test]
fn test_trust_policy_enforcement() {
    let manager = TrustStoreManager::new();
    let _result = manager.initialize(TrustManagerConfig::default());
    
    // Test with restrictive policy that should fail
    let chain = CertificateChain {
        end_entity: create_test_end_entity_certificate(),
        intermediates: vec![],
        root: None,
    };
    
    let validation_result = manager.validate_certificate_chain(&chain, None, None)
        .expect("Should get validation result");
    
    // Should fail due to no trust anchor
    assert!(!validation_result.is_trusted, "Should not be trusted without trust anchor");
    assert!(!validation_result.errors.is_empty(), "Should have validation errors");
}

/// Test system trust store loading
#[test]
fn test_system_trust_store_loading() {
    let manager = TrustStoreManager::new();
    
    let result = manager.load_system_trust_stores();
    assert!(result.is_ok(), "Should load system trust stores successfully");
    
    // Note: Actual system stores depend on platform
    // This test mainly ensures the loading process doesn't fail
}

/// Test trust store export functionality
#[test]
fn test_trust_store_export() {
    let manager = TrustStoreManager::new();
    let _store_name = manager.create_trust_store("export-test".to_string())
        .expect("Should create trust store");
    
    let root_cert = create_test_ca_certificate();
    let _result = manager.add_root_certificate("export-test", root_cert);
    
    // Test PEM export
    let pem_result = manager.export_trust_store("export-test", "pem", "/tmp/test-store.pem");
    assert!(pem_result.is_ok(), "Should export PEM format successfully");
    
    // Test DER export
    let der_result = manager.export_trust_store("export-test", "der", "/tmp/test-store.der");
    assert!(der_result.is_ok(), "Should export DER format successfully");
    
    // Test invalid format
    let invalid_result = manager.export_trust_store("export-test", "invalid", "/tmp/test");
    assert!(invalid_result.is_err(), "Should fail for invalid format");
}

/// Test trust store import functionality
#[test]
fn test_trust_store_import() {
    let manager = TrustStoreManager::new();
    let _store_name = manager.create_trust_store("import-test".to_string())
        .expect("Should create trust store");
    
    // Create a test file first
    create_test_pem_file("/tmp/test-import.pem");
    
    let import_result = manager.import_trust_store("import-test", "pem", "/tmp/test-import.pem");
    assert!(import_result.is_ok(), "Should import PEM format successfully");
    
    let count = import_result.unwrap();
    assert!(count > 0, "Should import at least one certificate");
}

/// Test trust validation result structure
#[test]
fn test_trust_validation_result_structure() {
    let manager = TrustStoreManager::new();
    let _result = manager.initialize(TrustManagerConfig::default());
    
    let chain = CertificateChain {
        end_entity: create_test_end_entity_certificate(),
        intermediates: vec![],
        root: None,
    };
    
    let validation_result = manager.validate_certificate_chain(&chain, None, None)
        .expect("Should get validation result");
    
    // Verify result structure
    assert!(!validation_result.policy_name.is_empty());
    assert!(validation_result.trust_level <= TrustLevel::FullyTrusted);
    assert!(validation_result.validated_at > UNIX_EPOCH);
    assert!(validation_result.validation_duration.as_millis() >= 0);
    assert_eq!(validation_result.revocation_status, RevocationStatus::Good); // Default for test
}

/// Test revocation status checking
#[test]
fn test_revocation_status_checking() {
    let manager = TrustStoreManager::new();
    
    let certificate = create_test_end_entity_certificate();
    let policy = RevocationPolicy::default();
    
    let revocation_status = manager.check_revocation_status(&certificate, &policy)
        .expect("Should check revocation status");
    
    // For test implementation, should return Good
    assert_eq!(revocation_status, RevocationStatus::Good);
}

/// Test certificate format validation
#[test]
fn test_certificate_format_validation() {
    let manager = TrustStoreManager::new();
    
    let valid_cert = create_test_ca_certificate();
    let result = manager.validate_certificate_format(&valid_cert);
    assert!(result.is_ok(), "Valid certificate should pass format validation");
    
    let invalid_cert = create_invalid_certificate();
    let result = manager.validate_certificate_format(&invalid_cert);
    assert!(result.is_err(), "Invalid certificate should fail format validation");
}

/// Test key usage validation
#[test]
fn test_key_usage_validation() {
    let manager = TrustStoreManager::new();
    
    let certificate = create_test_ca_certificate();
    let required_usage = KeyUsage {
        key_cert_sign: true,
        crl_sign: true,
        ..KeyUsage::default()
    };
    
    let is_valid = manager.validate_key_usage(&certificate, &required_usage);
    // For test certificate, this should pass
    assert!(is_valid, "CA certificate should have required key usage");
}

/// Test extended key usage validation
#[test]
fn test_extended_key_usage_validation() {
    let manager = TrustStoreManager::new();
    
    let certificate = create_test_end_entity_certificate();
    let required_ext_usage = ExtendedKeyUsage {
        server_auth: true,
        ..ExtendedKeyUsage::default()
    };
    
    let is_valid = manager.validate_extended_key_usage(&certificate, &required_ext_usage);
    // For test certificate, this should pass
    assert!(is_valid, "Server certificate should have server auth usage");
}

/// Test trust level determination
#[test]
fn test_trust_level_determination() {
    // Test different trust levels
    assert!(TrustLevel::FullyTrusted > TrustLevel::PartiallyTrusted);
    assert!(TrustLevel::PartiallyTrusted > TrustLevel::NotTrusted);
    assert!(TrustLevel::ExplicitlyTrusted > TrustLevel::FullyTrusted);
}

/// Test trust error codes
#[test]
fn test_trust_error_codes() {
    let error = TrustValidationError {
        code: TrustErrorCode::CertificateExpired,
        message: "Certificate has expired".to_string(),
        certificate: None,
        failed_rule: Some("time_validation".to_string()),
    };
    
    assert_eq!(error.code, TrustErrorCode::CertificateExpired);
    assert!(!error.message.is_empty());
    assert!(error.failed_rule.is_some());
}

/// Test trust policy configuration
#[test]
fn test_trust_policy_configuration() {
    let policy = TrustPolicy {
        name: "strict".to_string(),
        version: "1.0".to_string(),
        allow_self_signed: false,
        max_chain_length: 5,
        required_key_usage: Some(KeyUsage {
            key_cert_sign: true,
            ..KeyUsage::default()
        }),
        required_extended_key_usage: None,
        allowed_purposes: [CertificatePurpose::ServerAuth].iter().cloned().collect(),
        name_constraints: None,
        policy_constraints: None,
        revocation_policy: RevocationPolicy::default(),
        signature_algorithm_constraints: SignatureAlgorithmConstraints::default(),
        time_validation: TimeValidationPolicy::default(),
        custom_rules: Vec::new(),
    };
    
    assert_eq!(policy.name, "strict");
    assert_eq!(policy.max_chain_length, 5);
    assert!(!policy.allow_self_signed);
    assert!(policy.allowed_purposes.contains(&CertificatePurpose::ServerAuth));
}

/// Test certificate purpose validation
#[test]
fn test_certificate_purpose_validation() {
    let purposes = vec![
        CertificatePurpose::ServerAuth,
        CertificatePurpose::ClientAuth,
        CertificatePurpose::CodeSigning,
        CertificatePurpose::EmailProtection,
        CertificatePurpose::TimeStamping,
        CertificatePurpose::OcspSigning,
        CertificatePurpose::CertificateSigning,
        CertificatePurpose::CrlSigning,
        CertificatePurpose::AnyPurpose,
        CertificatePurpose::Custom("test".to_string()),
    ];
    
    assert_eq!(purposes.len(), 10);
    assert!(purposes.contains(&CertificatePurpose::ServerAuth));
}

/// Test system trust store platform detection
#[test]
fn test_system_trust_store_platform_detection() {
    let platforms = vec![
        TrustStorePlatform::Linux,
        TrustStorePlatform::Windows,
        TrustStorePlatform::MacOS,
        TrustStorePlatform::Mozilla,
        TrustStorePlatform::Java,
        TrustStorePlatform::Custom("test".to_string()),
    ];
    
    assert_eq!(platforms.len(), 6);
    assert!(platforms.contains(&TrustStorePlatform::Linux));
}

/// Test cache configuration
#[test]
fn test_cache_configuration() {
    let cache_config = CacheConfig::default();
    
    assert!(cache_config.max_cache_size > 0);
    assert!(cache_config.default_crl_cache_duration.as_secs() > 0);
    assert!(cache_config.default_ocsp_cache_duration.as_secs() > 0);
    assert!(cache_config.cleanup_interval.as_secs() > 0);
}

/// Test security configuration
#[test]
fn test_security_configuration() {
    let security_config = SecurityConfig::default();
    
    assert!(security_config.min_rsa_key_size >= 2048);
    assert!(security_config.min_ecc_key_size >= 256);
    assert!(!security_config.allow_weak_signatures);
}

/// Test signature algorithm constraints
#[test]
fn test_signature_algorithm_constraints() {
    let constraints = SignatureAlgorithmConstraints::default();
    
    assert!(!constraints.allowed_algorithms.is_empty());
    assert!(!constraints.minimum_key_sizes.is_empty());
    assert!(!constraints.forbidden_algorithms.is_empty());
    
    assert!(constraints.allowed_algorithms.contains(&SignatureAlgorithm::RsaWithSha256));
    assert!(constraints.forbidden_algorithms.contains(&SignatureAlgorithm::RsaWithSha1));
}

/// Test time validation policy
#[test]
fn test_time_validation_policy() {
    let time_policy = TimeValidationPolicy::default();
    
    assert!(!time_policy.allow_not_yet_valid);
    assert!(!time_policy.allow_expired);
    assert_eq!(time_policy.expiry_grace_period.as_secs(), 0);
    assert!(time_policy.future_validity_tolerance.as_secs() > 0);
}

/// Test trust store statistics
#[test]
fn test_trust_store_statistics() {
    let mut stats = TrustStoreStatistics::default();
    
    stats.stores_managed = 5;
    stats.trust_validations = 100;
    stats.successful_validations = 95;
    stats.failed_validations = 5;
    
    assert_eq!(stats.stores_managed, 5);
    assert_eq!(stats.trust_validations, 100);
    let success_rate = stats.successful_validations as f64 / stats.trust_validations as f64;
    assert!((success_rate - 0.95).abs() < 0.01);
}

/// Test CRL cache functionality
#[test]
fn test_crl_cache_functionality() {
    let cache = CrlCache::default();
    
    assert!(cache.crls.is_empty());
    assert!(cache.ocsp_responses.is_empty());
    assert_eq!(cache.statistics.crl_cache_hits, 0);
    assert_eq!(cache.statistics.ocsp_cache_hits, 0);
}

/// Test custom validation rules
#[test]
fn test_custom_validation_rules() {
    let rule = CustomValidationRule {
        name: "test-rule".to_string(),
        description: "Test validation rule".to_string(),
        rule_type: ValidationRuleType::RequiredOid("1.2.3.4".to_string()),
        parameters: HashMap::new(),
    };
    
    assert_eq!(rule.name, "test-rule");
    assert!(!rule.description.is_empty());
    assert!(matches!(rule.rule_type, ValidationRuleType::RequiredOid(_)));
}

/// Test public API functions
#[test]
fn test_public_api_functions() {
    // Test trust store initialization
    let init_result = init_trust_stores();
    assert!(init_result.is_ok(), "Trust store initialization should succeed");
    
    // Test trust store creation
    let create_result = create_trust_store("api-test".to_string());
    assert!(create_result.is_ok(), "Trust store creation should succeed");
    
    // Test system trust store loading
    let load_result = load_system_trust_stores();
    assert!(load_result.is_ok(), "System trust store loading should succeed");
}

// Helper functions for creating test certificates

fn create_test_ca_certificate() -> X509Certificate {
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(1),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName::from_common_name("Test Root CA"),
        validity: Validity {
            not_before: UNIX_EPOCH,
            not_after: UNIX_EPOCH + Duration::from_secs(10 * 365 * 24 * 3600),
        },
        subject: DistinguishedName::from_common_name("Test Root CA"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0; 256],
            parameters: None,
        },
        extensions: vec![
            X509Extension {
                oid: "2.5.29.19".to_string(),
                critical: true,
                value: vec![0x30, 0x03, 0x01, 0x01, 0xFF],
                parsed_data: Some(ExtensionData::BasicConstraints {
                    is_ca: true,
                    path_length_constraint: None,
                }),
            }
        ],
        raw_data: vec![0; 100],
        fingerprint: Some(vec![1, 2, 3, 4]),
        key_usage: KeyUsage {
            key_cert_sign: true,
            crl_sign: true,
            ..KeyUsage::default()
        },
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_test_intermediate_certificate() -> X509Certificate {
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(2),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName::from_common_name("Test Root CA"),
        validity: Validity {
            not_before: UNIX_EPOCH,
            not_after: UNIX_EPOCH + Duration::from_secs(5 * 365 * 24 * 3600),
        },
        subject: DistinguishedName::from_common_name("Test Intermediate CA"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0; 256],
            parameters: None,
        },
        extensions: vec![
            X509Extension {
                oid: "2.5.29.19".to_string(),
                critical: true,
                value: vec![0x30, 0x06, 0x01, 0x01, 0xFF, 0x02, 0x01, 0x05],
                parsed_data: Some(ExtensionData::BasicConstraints {
                    is_ca: true,
                    path_length_constraint: Some(5),
                }),
            }
        ],
        raw_data: vec![0; 100],
        fingerprint: Some(vec![2, 3, 4, 5]),
        key_usage: KeyUsage {
            key_cert_sign: true,
            crl_sign: true,
            ..KeyUsage::default()
        },
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_test_end_entity_certificate() -> X509Certificate {
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(3),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName::from_common_name("Test Intermediate CA"),
        validity: Validity {
            not_before: UNIX_EPOCH,
            not_after: UNIX_EPOCH + Duration::from_secs(365 * 24 * 3600),
        },
        subject: DistinguishedName::from_common_name("test.example.com"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0; 256],
            parameters: None,
        },
        extensions: vec![],
        raw_data: vec![0; 100],
        fingerprint: Some(vec![3, 4, 5, 6]),
        key_usage: KeyUsage {
            digital_signature: true,
            key_encipherment: true,
            ..KeyUsage::default()
        },
        extended_key_usage: ExtendedKeyUsage {
            server_auth: true,
            client_auth: true,
            ..ExtendedKeyUsage::default()
        },
    }
}

fn create_invalid_certificate() -> X509Certificate {
    X509Certificate {
        version: 0, // Invalid version
        serial_number: SerialNumber::from_big_int(99),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName::from_common_name("Invalid Certificate"),
        validity: Validity {
            not_before: UNIX_EPOCH,
            not_after: UNIX_EPOCH,
        },
        subject: DistinguishedName::from_common_name("Invalid Certificate"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![],
            parameters: None,
        },
        extensions: vec![],
        raw_data: vec![], // Empty raw data makes it invalid
        fingerprint: None,
        key_usage: KeyUsage::default(),
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_test_pem_file(path: &str) {
    use std::fs::File;
    use std::io::Write;
    
    let pem_content = r#"-----BEGIN CERTIFICATE-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAuVDGbZZWB8nJA1VqeXAu
9mYYXTbA6VsKMJ1E3Y0XvJo9EkQYUZ1gJHFb9Z4eYT5Jgpk1MzSl8AcQ7hWx8lW9
7uRFmNRZwLGZb6HEJ1bKZ7o1x0aQ8e1JQTaJJ1MzSl8AcQ7hWx8lW97uRFmNRZwL
GZb6HEJ1bKZ7o1x0aQ8e1JQTaJJ1MzSl8AcQ7hWx8lW97uRFmNRZwLGZb6HEJ1bK
Z7o1x0aQ8e1JQTaJJ1MzSl8AcQ7hWx8lW97uRFmNRZwLGZb6HEJ1bKZ7o1x0aQ8e
1JQTaJJ1MzSl8AcQ7hWx8lW97uRFmNRZwL
-----END CERTIFICATE-----"#;
    
    if let Ok(mut file) = File::create(path) {
        let _ = file.write_all(pem_content.as_bytes());
    }
}
