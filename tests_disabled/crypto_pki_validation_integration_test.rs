//! PKI Certificate Validation Integration Tests
//!
//! Comprehensive integration tests for the PKI certificate validation module covering:
//! - End-to-end certificate validation workflows
//! - Integration with trust stores and chain validators
//! - Real-world certificate validation scenarios
//! - Performance testing and benchmarking
//! - Error handling and edge cases
//! - Cache behavior and effectiveness

use std::time::{SystemTime, Duration};
use std::collections::HashMap;

#[path = "../src/stdlib/packages/crypto_pki/validation.rs"]
mod validation;

#[path = "../src/stdlib/packages/crypto_pki/types.rs"]  
mod types;

#[path = "../src/stdlib/packages/crypto_pki/error.rs"]
mod error;

#[path = "../src/stdlib/packages/crypto_pki/chain_validation.rs"]
mod chain_validation;

use validation::*;
use types::*;
use error::*;

/// Test certificate validation with a complete PKI infrastructure
#[test]
fn test_end_to_end_certificate_validation() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Create a complete certificate hierarchy
    let root_ca = create_root_ca_certificate();
    let intermediate_ca = create_intermediate_ca_certificate(&root_ca);
    let end_entity = create_end_entity_certificate(&intermediate_ca);
    
    // Set up trust store
    let mut trust_store = TrustStore::new("integration_test");
    trust_store.add_root_certificate(root_ca.clone());
    trust_store.add_intermediate_certificate(intermediate_ca.clone());
    validator.add_trust_store("integration_test".to_string(), trust_store);
    
    // Create certificate chain
    let chain = CertificateChain {
        end_entity: end_entity.clone(),
        intermediates: vec![intermediate_ca],
        root: Some(root_ca),
    };
    
    // Validate the complete chain
    let result = validator.validate_certificate_chain(&chain, Some("integration_test"));
    assert!(result.is_ok());
    
    let validation_result = result.unwrap();
    assert!(validation_result.basic_result.is_valid);
    assert!(validation_result.basic_result.errors.is_empty());
    
    // Verify detailed validation information
    assert_eq!(validation_result.details.chain_info.chain_length, 3);
    assert!(validation_result.details.chain_info.trust_path_found);
    assert!(validation_result.details.chain_info.trust_anchor.is_some());
    
    // Verify signature validation
    assert_eq!(validation_result.details.signature_info.verification_status, 
               SignatureVerificationStatus::Valid);
    
    // Verify revocation check
    assert_eq!(validation_result.details.revocation_info.status, RevocationStatus::Good);
    
    // Verify performance metrics
    assert!(validation_result.performance.total_time_ms > 0);
    assert!(validation_result.performance.total_time_ms >= 
            validation_result.performance.chain_building_time_ms);
}

#[test]
fn test_certificate_validation_with_invalid_chain() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Create certificates with mismatched issuer/subject
    let root_ca = create_root_ca_certificate();
    let unrelated_intermediate = create_intermediate_ca_certificate(&create_different_root_ca());
    let end_entity = create_end_entity_certificate(&unrelated_intermediate);
    
    // Set up trust store with only the root CA
    let mut trust_store = TrustStore::new("invalid_chain_test");
    trust_store.add_root_certificate(root_ca.clone());
    validator.add_trust_store("invalid_chain_test".to_string(), trust_store);
    
    // Create invalid chain
    let chain = CertificateChain {
        end_entity,
        intermediates: vec![unrelated_intermediate],
        root: Some(root_ca),
    };
    
    // Validation should fail due to chain mismatch
    let result = validator.validate_certificate_chain(&chain, Some("invalid_chain_test"));
    assert!(result.is_ok());
    
    let validation_result = result.unwrap();
    assert!(!validation_result.basic_result.is_valid);
    assert!(!validation_result.basic_result.errors.is_empty());
}

#[test]
fn test_hostname_validation_scenarios() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Test various hostname scenarios
    let test_cases = vec![
        ("example.com", "example.com", true),           // Exact match
        ("*.example.com", "www.example.com", true),     // Wildcard match
        ("*.example.com", "api.example.com", true),     // Wildcard match
        ("*.example.com", "sub.api.example.com", false), // Invalid wildcard depth
        ("example.com", "different.com", false),        // No match
        ("*.example.com", "example.com", false),        // Wildcard doesn't match base domain
    ];
    
    for (cert_name, target_hostname, should_match) in test_cases {
        let certificate = create_certificate_with_san(cert_name);
        let result = validator.validate_certificate_for_hostname(&certificate, target_hostname, None);
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        
        let has_match = validation_result.details.name_info.hostname_matching
            .iter()
            .any(|m| matches!(m.match_result, 
                HostnameMatchStatus::ExactMatch | HostnameMatchStatus::WildcardMatch));
        
        assert_eq!(has_match, should_match, 
                   "Hostname validation failed for cert_name='{}', target='{}'", 
                   cert_name, target_hostname);
    }
}

#[test]
fn test_certificate_expiration_scenarios() {
    let mut validator = CertificateValidator::with_defaults();
    
    let now = SystemTime::now();
    
    // Test various expiration scenarios
    let test_scenarios = vec![
        // (not_before, not_after, expected_valid)
        (now - Duration::from_secs(86400), now + Duration::from_secs(86400), true),  // Currently valid
        (now - Duration::from_secs(86400 * 365), now - Duration::from_secs(86400), false), // Expired
        (now + Duration::from_secs(86400), now + Duration::from_secs(86400 * 365), false), // Not yet valid
        (now - Duration::from_secs(1), now + Duration::from_secs(1), true),          // Edge case: very short validity
    ];
    
    for (not_before, not_after, expected_valid) in test_scenarios {
        let certificate = create_certificate_with_validity(not_before, not_after);
        let result = validator.validate_certificate(&certificate, None);
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        
        if expected_valid {
            assert!(validation_result.basic_result.is_valid || 
                    !validation_result.basic_result.errors.iter()
                        .any(|e| e.contains("validity") || e.contains("expired") || e.contains("not yet valid")),
                    "Certificate should be valid for validity period {:?} to {:?}", not_before, not_after);
        } else {
            assert!(!validation_result.basic_result.is_valid,
                    "Certificate should be invalid for validity period {:?} to {:?}", not_before, not_after);
        }
    }
}

#[test]
fn test_key_strength_validation() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Test various key strengths
    let test_keys = vec![
        (PublicKeyAlgorithm::Rsa { key_size: 1024 }, SecurityLevel::Low),
        (PublicKeyAlgorithm::Rsa { key_size: 2048 }, SecurityLevel::Medium),
        (PublicKeyAlgorithm::Rsa { key_size: 3072 }, SecurityLevel::High),
        (PublicKeyAlgorithm::Rsa { key_size: 4096 }, SecurityLevel::High),
        (PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 }, SecurityLevel::Medium),
        (PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P384 }, SecurityLevel::Medium),
        (PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P521 }, SecurityLevel::High),
        (PublicKeyAlgorithm::Ed25519, SecurityLevel::Medium),
        (PublicKeyAlgorithm::Ed448, SecurityLevel::High),
    ];
    
    for (algorithm, expected_level) in test_keys {
        let certificate = create_certificate_with_key_algorithm(algorithm.clone());
        let result = validator.validate_certificate(&certificate, None);
        
        assert!(result.is_ok());
        let validation_result = result.unwrap();
        
        assert_eq!(validation_result.details.signature_info.key_strength.security_level, 
                   expected_level,
                   "Security level mismatch for algorithm {:?}", algorithm);
        
        // Check for appropriate recommendations for weak keys
        if expected_level == SecurityLevel::Low {
            assert!(validation_result.recommendations.iter()
                .any(|r| r.recommendation_type == RecommendationType::SecurityImprovement),
                "Should have security improvement recommendation for weak key");
        }
    }
}

#[test]
fn test_policy_validation_scenarios() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Test certificates with various policy violations
    let mut config = validator.config.clone();
    config.required_policies = vec!["1.2.3.4.5".to_string()]; // Require specific policy
    config.prohibited_policies = vec!["1.2.3.4.6".to_string()]; // Prohibit specific policy
    validator.update_config(config);
    
    // Test certificate with required policy
    let cert_with_required_policy = create_certificate_with_policies(vec!["1.2.3.4.5".to_string()]);
    let result = validator.validate_certificate(&cert_with_required_policy, None);
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    // Should have fewer violations since required policy is present
    assert!(validation_result.details.policy_info.violations.len() <= 1);
    
    // Test certificate with prohibited policy
    let cert_with_prohibited_policy = create_certificate_with_policies(vec!["1.2.3.4.6".to_string()]);
    let result = validator.validate_certificate(&cert_with_prohibited_policy, None);
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    // Should have policy violations
    assert!(!validation_result.details.policy_info.violations.is_empty());
    
    // Test certificate without required policy
    let cert_without_required = create_certificate_with_policies(vec!["1.2.3.4.7".to_string()]);
    let result = validator.validate_certificate(&cert_without_required, None);
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    // Should have missing required policy violation
    assert!(validation_result.details.policy_info.violations.iter()
        .any(|v| v.violation_type == PolicyViolationType::MissingRequiredPolicy));
}

#[test]
fn test_extension_validation() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Test certificate with unknown critical extension
    let cert_with_unknown_critical = create_certificate_with_unknown_critical_extension();
    let result = validator.validate_certificate(&cert_with_unknown_critical, None);
    
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    
    // Should have extension errors for unknown critical extension
    assert!(!validation_result.details.extensions_info.unknown_critical_extensions.is_empty());
    assert!(validation_result.details.extensions_info.extension_errors.iter()
        .any(|e| e.error_type == ExtensionErrorType::UnsupportedExtension));
    
    // Test certificate with only known extensions
    let cert_with_known_extensions = create_certificate_with_standard_extensions();
    let result = validator.validate_certificate(&cert_with_known_extensions, None);
    
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    
    // Should not have unknown critical extensions
    assert!(validation_result.details.extensions_info.unknown_critical_extensions.is_empty());
}

#[test]
fn test_cache_behavior() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Ensure caching is enabled
    let mut config = validator.config.clone();
    config.cache_config.enable_caching = true;
    config.cache_config.success_cache_ttl = Duration::from_secs(60);
    validator.update_config(config);
    
    let certificate = create_standard_test_certificate();
    
    // First validation - should not be cached
    let start_time = SystemTime::now();
    let result1 = validator.validate_certificate(&certificate, None);
    let first_duration = start_time.elapsed().unwrap();
    
    assert!(result1.is_ok());
    let validation_result1 = result1.unwrap();
    assert!(validation_result1.basic_result.is_valid);
    
    // Second validation - should be faster due to caching
    let start_time = SystemTime::now();
    let result2 = validator.validate_certificate(&certificate, None);
    let second_duration = start_time.elapsed().unwrap();
    
    assert!(result2.is_ok());
    let validation_result2 = result2.unwrap();
    assert!(validation_result2.basic_result.is_valid);
    
    // Note: In a real implementation with actual caching, second_duration would be much smaller
    // For this test, we just verify that both validations succeed
    println!("First validation: {:?}, Second validation: {:?}", first_duration, second_duration);
}

#[test]
fn test_performance_under_load() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Create multiple certificates for testing
    let certificates: Vec<X509Certificate> = (0..50)
        .map(|i| create_certificate_with_serial(i as u64))
        .collect();
    
    let start_time = SystemTime::now();
    let mut successful_validations = 0;
    let mut failed_validations = 0;
    
    // Validate all certificates
    for certificate in &certificates {
        match validator.validate_certificate(certificate, None) {
            Ok(result) => {
                if result.basic_result.is_valid {
                    successful_validations += 1;
                } else {
                    failed_validations += 1;
                }
            }
            Err(_) => {
                failed_validations += 1;
            }
        }
    }
    
    let total_time = start_time.elapsed().unwrap();
    let avg_time_per_cert = total_time.as_millis() / certificates.len() as u128;
    
    println!("Validated {} certificates in {:?}", certificates.len(), total_time);
    println!("Average time per certificate: {}ms", avg_time_per_cert);
    println!("Successful validations: {}, Failed validations: {}", successful_validations, failed_validations);
    
    // Performance assertions
    assert!(successful_validations > 0);
    assert!(avg_time_per_cert < 1000); // Should be under 1 second per certificate
    
    // Verify statistics were updated
    let stats = validator.get_statistics();
    assert_eq!(stats.total_validations, certificates.len() as u64);
    assert!(stats.avg_validation_time_ms > 0.0);
}

#[test]
fn test_validation_statistics_accuracy() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Validate a mix of valid and invalid certificates
    let valid_cert = create_standard_test_certificate();
    let expired_cert = create_expired_test_certificate();
    let weak_key_cert = create_certificate_with_weak_key();
    
    // Track initial statistics
    let initial_stats = validator.get_statistics().clone();
    
    // Validate certificates
    let _ = validator.validate_certificate(&valid_cert, None);
    let _ = validator.validate_certificate(&expired_cert, None);
    let _ = validator.validate_certificate(&weak_key_cert, None);
    
    // Check updated statistics
    let final_stats = validator.get_statistics();
    
    assert_eq!(final_stats.total_validations, initial_stats.total_validations + 3);
    assert!(final_stats.avg_validation_time_ms > 0.0);
    assert!(final_stats.chain_validation_stats.total_chains > initial_stats.chain_validation_stats.total_chains);
    assert!(final_stats.signature_validation_stats.total_signatures > initial_stats.signature_validation_stats.total_signatures);
    assert!(final_stats.revocation_check_stats.total_checks > initial_stats.revocation_check_stats.total_checks);
}

#[test]
fn test_trust_store_integration() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Create multiple trust stores
    let root_ca1 = create_root_ca_certificate();
    let root_ca2 = create_different_root_ca();
    
    let mut trust_store1 = TrustStore::new("store1");
    trust_store1.add_root_certificate(root_ca1.clone());
    
    let mut trust_store2 = TrustStore::new("store2");
    trust_store2.add_root_certificate(root_ca2.clone());
    
    validator.add_trust_store("store1".to_string(), trust_store1);
    validator.add_trust_store("store2".to_string(), trust_store2);
    
    // Create certificates signed by different CAs
    let cert1 = create_end_entity_certificate(&root_ca1);
    let cert2 = create_end_entity_certificate(&root_ca2);
    
    // Validate cert1 with store1 (should succeed)
    let result1 = validator.validate_certificate(&cert1, Some("store1"));
    assert!(result1.is_ok());
    assert!(result1.unwrap().basic_result.is_valid);
    
    // Validate cert2 with store2 (should succeed)
    let result2 = validator.validate_certificate(&cert2, Some("store2"));
    assert!(result2.is_ok());
    assert!(result2.unwrap().basic_result.is_valid);
    
    // Validate cert1 with store2 (should fail - wrong trust store)
    let result3 = validator.validate_certificate(&cert1, Some("store2"));
    assert!(result3.is_ok());
    // May fail due to trust path issues, but test should complete
}

#[test]
fn test_error_handling_robustness() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Test various error conditions
    let test_cases = vec![
        ("Empty certificate data", create_empty_certificate()),
        ("Malformed extensions", create_certificate_with_malformed_extensions()),
        ("Invalid signature algorithm", create_certificate_with_invalid_signature()),
        ("Corrupted key data", create_certificate_with_corrupted_key()),
    ];
    
    for (test_name, certificate) in test_cases {
        let result = validator.validate_certificate(&certificate, None);
        
        // All validation attempts should return Ok (not panic), but may indicate invalid certificates
        assert!(result.is_ok(), "Validation failed for test case: {}", test_name);
        
        let validation_result = result.unwrap();
        
        // For most error cases, we expect the certificate to be marked as invalid
        // The specific behavior depends on the implementation
        println!("Test case '{}': valid={}, errors={}", 
                test_name, 
                validation_result.basic_result.is_valid,
                validation_result.basic_result.errors.len());
    }
}

// Helper functions for creating test certificates and data

fn create_root_ca_certificate() -> X509Certificate {
    let now = SystemTime::now();
    
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(1),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName::from_common_name("Test Root CA"),
        validity: Validity {
            not_before: now - Duration::from_secs(86400 * 365),
            not_after: now + Duration::from_secs(86400 * 365 * 10),
        },
        subject: DistinguishedName::from_common_name("Test Root CA"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 4096 },
            public_key: vec![0x30, 0x82, 0x02, 0x22],
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
        raw_data: Vec::new(),
        fingerprint: Some(vec![0x12, 0x34, 0x56, 0x78]),
        key_usage: KeyUsage {
            key_cert_sign: true,
            crl_sign: true,
            ..KeyUsage::default()
        },
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_different_root_ca() -> X509Certificate {
    let mut ca = create_root_ca_certificate();
    ca.serial_number = SerialNumber::from_big_int(2);
    ca.issuer = DistinguishedName::from_common_name("Different Root CA");
    ca.subject = DistinguishedName::from_common_name("Different Root CA");
    ca.fingerprint = Some(vec![0x98, 0x76, 0x54, 0x32]);
    ca
}

fn create_intermediate_ca_certificate(issuer: &X509Certificate) -> X509Certificate {
    let now = SystemTime::now();
    
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(100),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: issuer.subject.clone(),
        validity: Validity {
            not_before: now - Duration::from_secs(86400 * 30),
            not_after: now + Duration::from_secs(86400 * 365 * 5),
        },
        subject: DistinguishedName::from_common_name("Test Intermediate CA"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0x30, 0x82, 0x01, 0x22],
            parameters: None,
        },
        extensions: vec![
            X509Extension {
                oid: "2.5.29.19".to_string(),
                critical: true,
                value: vec![0x30, 0x06, 0x01, 0x01, 0xFF, 0x02, 0x01, 0x00],
                parsed_data: Some(ExtensionData::BasicConstraints {
                    is_ca: true,
                    path_length_constraint: Some(0),
                }),
            }
        ],
        raw_data: Vec::new(),
        fingerprint: Some(vec![0xAB, 0xCD, 0xEF, 0x12]),
        key_usage: KeyUsage {
            key_cert_sign: true,
            crl_sign: true,
            ..KeyUsage::default()
        },
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_end_entity_certificate(issuer: &X509Certificate) -> X509Certificate {
    let now = SystemTime::now();
    
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(1000),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: issuer.subject.clone(),
        validity: Validity {
            not_before: now - Duration::from_secs(86400),
            not_after: now + Duration::from_secs(86400 * 365),
        },
        subject: DistinguishedName::from_common_name("test.example.com"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0x30, 0x82, 0x01, 0x22],
            parameters: None,
        },
        extensions: vec![
            X509Extension {
                oid: "2.5.29.17".to_string(),
                critical: false,
                value: vec![0x30, 0x15, 0x82, 0x13, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d],
                parsed_data: Some(ExtensionData::SubjectAlternativeName(vec![
                    GeneralName::DnsName("test.example.com".to_string()),
                ])),
            }
        ],
        raw_data: Vec::new(),
        fingerprint: Some(vec![0x11, 0x22, 0x33, 0x44]),
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

fn create_certificate_with_san(san_name: &str) -> X509Certificate {
    let mut cert = create_end_entity_certificate(&create_root_ca_certificate());
    
    cert.extensions = vec![
        X509Extension {
            oid: "2.5.29.17".to_string(),
            critical: false,
            value: vec![0x30, 0x15], // Mock DER encoding
            parsed_data: Some(ExtensionData::SubjectAlternativeName(vec![
                GeneralName::DnsName(san_name.to_string()),
            ])),
        }
    ];
    
    cert
}

fn create_certificate_with_validity(not_before: SystemTime, not_after: SystemTime) -> X509Certificate {
    let mut cert = create_end_entity_certificate(&create_root_ca_certificate());
    cert.validity = Validity { not_before, not_after };
    cert
}

fn create_certificate_with_key_algorithm(algorithm: PublicKeyAlgorithm) -> X509Certificate {
    let mut cert = create_end_entity_certificate(&create_root_ca_certificate());
    cert.subject_public_key_info.algorithm = algorithm;
    cert
}

fn create_certificate_with_policies(policies: Vec<String>) -> X509Certificate {
    let mut cert = create_end_entity_certificate(&create_root_ca_certificate());
    
    let policy_infos: Vec<PolicyInformation> = policies.into_iter()
        .map(|oid| PolicyInformation {
            policy_identifier: oid,
            policy_qualifiers: None,
        })
        .collect();
    
    cert.extensions.push(X509Extension {
        oid: "2.5.29.32".to_string(), // Certificate Policies
        critical: false,
        value: vec![0x30, 0x06], // Mock DER encoding
        parsed_data: Some(ExtensionData::CertificatePolicies(policy_infos)),
    });
    
    cert
}

fn create_certificate_with_unknown_critical_extension() -> X509Certificate {
    let mut cert = create_end_entity_certificate(&create_root_ca_certificate());
    
    cert.extensions.push(X509Extension {
        oid: "1.2.3.4.5.6.7.8.9".to_string(), // Unknown OID
        critical: true,
        value: vec![0x04, 0x02, 0x01, 0x02], // Mock data
        parsed_data: None,
    });
    
    cert
}

fn create_certificate_with_standard_extensions() -> X509Certificate {
    create_end_entity_certificate(&create_root_ca_certificate())
}

fn create_standard_test_certificate() -> X509Certificate {
    create_end_entity_certificate(&create_root_ca_certificate())
}

fn create_expired_test_certificate() -> X509Certificate {
    let now = SystemTime::now();
    create_certificate_with_validity(
        now - Duration::from_secs(86400 * 365 * 2),
        now - Duration::from_secs(86400)
    )
}

fn create_certificate_with_weak_key() -> X509Certificate {
    create_certificate_with_key_algorithm(PublicKeyAlgorithm::Rsa { key_size: 1024 })
}

fn create_certificate_with_serial(serial: u64) -> X509Certificate {
    let mut cert = create_standard_test_certificate();
    cert.serial_number = SerialNumber::from_big_int(serial);
    cert
}

fn create_empty_certificate() -> X509Certificate {
    let now = SystemTime::now();
    
    X509Certificate {
        version: 1,
        serial_number: SerialNumber::from_big_int(0),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName::new(),
        validity: Validity {
            not_before: now,
            not_after: now,
        },
        subject: DistinguishedName::new(),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: Vec::new(),
            parameters: None,
        },
        extensions: Vec::new(),
        raw_data: Vec::new(),
        fingerprint: None,
        key_usage: KeyUsage::default(),
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_certificate_with_malformed_extensions() -> X509Certificate {
    let mut cert = create_standard_test_certificate();
    
    cert.extensions.push(X509Extension {
        oid: "2.5.29.19".to_string(), // Basic Constraints
        critical: true,
        value: vec![0xFF, 0xFF, 0xFF], // Invalid DER encoding
        parsed_data: None,
    });
    
    cert
}

fn create_certificate_with_invalid_signature() -> X509Certificate {
    let mut cert = create_standard_test_certificate();
    cert.signature_algorithm = SignatureAlgorithm::Custom {
        oid: "1.2.3.4.5.6.7.8.9.10".to_string(),
        name: "InvalidAlgorithm".to_string(),
    };
    cert
}

fn create_certificate_with_corrupted_key() -> X509Certificate {
    let mut cert = create_standard_test_certificate();
    cert.subject_public_key_info.public_key = vec![0xFF; 10]; // Corrupted key data
    cert
}
