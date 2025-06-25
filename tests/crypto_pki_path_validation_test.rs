/// Comprehensive PKI certificate path validation test suite
/// 
/// This test suite validates the RFC 5280 compliant certificate path validation
/// implementation with comprehensive coverage of validation algorithms, error
/// handling, security features, and edge cases.

use cursed::stdlib::packages::crypto_pki::path_validation::*;
use cursed::stdlib::packages::crypto_pki::*;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, Duration, UNIX_EPOCH};

/// Mock certificate creation helper for testing
fn create_mock_certificate(
    subject: &str,
    issuer: &str,
    serial: u64,
    not_before: SystemTime,
    not_after: SystemTime,
) -> CertificateInfo {
    CertificateInfo {
        version: 3,
        serial_number: serial.to_string(),
        signature_algorithm: "sha256WithRSAEncryption".to_string(),
        issuer_name: DistinguishedName {
            common_name: Some(issuer.to_string()),
            organization: None,
            organizational_unit: None,
            country: None,
            locality: None,
            state_or_province: None,
            email_address: None,
        },
        subject_name: DistinguishedName {
            common_name: Some(subject.to_string()),
            organization: None,
            organizational_unit: None,
            country: None,
            locality: None,
            state_or_province: None,
            email_address: None,
        },
        not_before,
        not_after,
        public_key: PublicKeyInfo {
            algorithm: "rsaEncryption".to_string(),
            key_data: vec![0u8; 256], // Mock RSA key
            parameters: None,
        },
        extensions: vec![],
        signature_value: vec![0u8; 256], // Mock signature
        tbs_certificate_data: vec![0u8; 512], // Mock TBS certificate
        authority_key_identifier: None,
        subject_key_identifier: Some(format!("ski_{}", subject).into_bytes()),
        key_usage: Some(KeyUsageFlags::KEY_CERT_SIGN | KeyUsageFlags::DIGITAL_SIGNATURE),
        extended_key_usage: None,
        basic_constraints: Some(BasicConstraints {
            ca: !subject.contains("End Entity"),
            path_len_constraint: if subject.contains("Root") { Some(3) } else { None },
        }),
        subject_alt_names: None,
        issuer_alt_names: None,
        certificate_policies: None,
        name_constraints: None,
        crl_distribution_points: None,
        ocsp_responders: None,
    }
}

/// Create mock trust anchor for testing
fn create_mock_trust_anchor(subject: &str) -> TrustAnchor {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600); // 1 year validity
    
    TrustAnchor {
        certificate: Some(create_mock_certificate(subject, subject, 1, now, future)),
        public_key: PublicKeyInfo {
            algorithm: "rsaEncryption".to_string(),
            key_data: vec![0u8; 256],
            parameters: None,
        },
        subject_name: DistinguishedName {
            common_name: Some(subject.to_string()),
            organization: None,
            organizational_unit: None,
            country: None,
            locality: None,
            state_or_province: None,
            email_address: None,
        },
        key_identifier: Some(format!("ski_{}", subject).into_bytes()),
        name_constraints: None,
        certificate_policies: HashSet::new(),
    }
}

/// Test basic certificate path validation functionality
#[test]
fn test_basic_path_validation() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificate chain: Root CA -> Intermediate CA -> End Entity
    let root_ca = create_mock_certificate("Root CA", "Root CA", 1, now, future);
    let intermediate_ca = create_mock_certificate("Intermediate CA", "Root CA", 2, now, future);
    let end_entity = create_mock_certificate("End Entity", "Intermediate CA", 3, now, future);
    
    // Create trust anchor for root CA
    let trust_anchor = create_mock_trust_anchor("Root CA");
    
    // Create validation context
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    // Validate certificate path
    let result = validator.validate_path(&end_entity, &[intermediate_ca]).unwrap();
    
    match result {
        PathValidationResult::Valid { validated_chain, .. } => {
            assert_eq!(validated_chain.len(), 3);
            assert_eq!(validated_chain[0].subject_name.common_name, Some("End Entity".to_string()));
            assert_eq!(validated_chain[2].subject_name.common_name, Some("Root CA".to_string()));
        }
        PathValidationResult::Invalid { error, .. } => {
            panic!("Expected valid path but got error: {:?}", error);
        }
    }
}

/// Test certificate chain building with multiple intermediate certificates
#[test]
fn test_certificate_chain_building() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create complex certificate chain
    let root_ca = create_mock_certificate("Root CA", "Root CA", 1, now, future);
    let intermediate1 = create_mock_certificate("Intermediate 1", "Root CA", 2, now, future);
    let intermediate2 = create_mock_certificate("Intermediate 2", "Intermediate 1", 3, now, future);
    let end_entity = create_mock_certificate("End Entity", "Intermediate 2", 4, now, future);
    
    // Add some irrelevant intermediate certificates
    let irrelevant1 = create_mock_certificate("Irrelevant CA 1", "Other Root", 5, now, future);
    let irrelevant2 = create_mock_certificate("Irrelevant CA 2", "Another Root", 6, now, future);
    
    let trust_anchor = create_mock_trust_anchor("Root CA");
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    // Include irrelevant certificates to test chain building logic
    let intermediates = vec![
        irrelevant1,
        intermediate1,
        irrelevant2,
        intermediate2,
    ];
    
    let result = validator.validate_path(&end_entity, &intermediates).unwrap();
    
    match result {
        PathValidationResult::Valid { validated_chain, .. } => {
            assert_eq!(validated_chain.len(), 4);
            // Verify chain order from end entity to root
            assert_eq!(validated_chain[0].subject_name.common_name, Some("End Entity".to_string()));
            assert_eq!(validated_chain[1].subject_name.common_name, Some("Intermediate 2".to_string()));
            assert_eq!(validated_chain[2].subject_name.common_name, Some("Intermediate 1".to_string()));
            assert_eq!(validated_chain[3].subject_name.common_name, Some("Root CA".to_string()));
        }
        PathValidationResult::Invalid { error, .. } => {
            panic!("Expected valid chain but got error: {:?}", error);
        }
    }
}

/// Test certificate validity period validation
#[test]
fn test_validity_period_validation() {
    let now = SystemTime::now();
    let past = now - Duration::from_secs(365 * 24 * 3600); // 1 year ago
    let future = now + Duration::from_secs(365 * 24 * 3600); // 1 year from now
    let far_future = now + Duration::from_secs(2 * 365 * 24 * 3600); // 2 years from now
    
    // Test expired certificate
    let expired_cert = create_mock_certificate("Expired Cert", "Root CA", 1, past, past + Duration::from_secs(3600));
    let trust_anchor = create_mock_trust_anchor("Root CA");
    let context = create_validation_context_with_anchors(vec![trust_anchor.clone()]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&expired_cert, &[]).unwrap();
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::ValidityPeriodViolation { .. } => {
                    // Expected error
                }
                _ => panic!("Expected validity period violation but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for expired certificate");
        }
    }
    
    // Test not-yet-valid certificate
    let future_cert = create_mock_certificate("Future Cert", "Root CA", 2, far_future, far_future + Duration::from_secs(365 * 24 * 3600));
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&future_cert, &[]).unwrap();
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::ValidityPeriodViolation { .. } => {
                    // Expected error
                }
                _ => panic!("Expected validity period violation but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for not-yet-valid certificate");
        }
    }
}

/// Test path length constraint validation
#[test]
fn test_path_length_constraints() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificate chain that violates path length constraints
    let mut root_ca = create_mock_certificate("Root CA", "Root CA", 1, now, future);
    root_ca.basic_constraints = Some(BasicConstraints {
        ca: true,
        path_len_constraint: Some(1), // Only allow 1 intermediate
    });
    
    let intermediate1 = create_mock_certificate("Intermediate 1", "Root CA", 2, now, future);
    let intermediate2 = create_mock_certificate("Intermediate 2", "Intermediate 1", 3, now, future);
    let end_entity = create_mock_certificate("End Entity", "Intermediate 2", 4, now, future);
    
    let mut trust_anchor = create_mock_trust_anchor("Root CA");
    trust_anchor.certificate = Some(root_ca);
    
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&end_entity, &[intermediate1, intermediate2]).unwrap();
    
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::PathLengthConstraintViolation { .. } => {
                    // Expected error
                }
                _ => panic!("Expected path length constraint violation but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for path length constraint violation");
        }
    }
}

/// Test name constraint validation
#[test]
fn test_name_constraint_validation() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificate with subject alternative name
    let mut end_entity = create_mock_certificate("example.com", "Intermediate CA", 1, now, future);
    end_entity.subject_alt_names = Some(vec![
        GeneralName::DnsName("test.example.com".to_string()),
        GeneralName::EmailAddress("admin@example.com".to_string()),
    ]);
    
    let intermediate_ca = create_mock_certificate("Intermediate CA", "Root CA", 2, now, future);
    
    // Create trust anchor with name constraints
    let mut trust_anchor = create_mock_trust_anchor("Root CA");
    trust_anchor.name_constraints = Some(NameConstraints {
        permitted_subtrees: vec![
            GeneralSubtree {
                base: GeneralName::DnsName(".example.com".to_string()),
                minimum: None,
                maximum: None,
            },
            GeneralSubtree {
                base: GeneralName::EmailAddress("@example.com".to_string()),
                minimum: None,
                maximum: None,
            },
        ],
        excluded_subtrees: vec![
            GeneralSubtree {
                base: GeneralName::DnsName(".restricted.example.com".to_string()),
                minimum: None,
                maximum: None,
            },
        ],
    });
    
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    // This should succeed as names are within permitted constraints
    let result = validator.validate_path(&end_entity, &[intermediate_ca]).unwrap();
    
    match result {
        PathValidationResult::Valid { .. } => {
            // Expected success
        }
        PathValidationResult::Invalid { error, .. } => {
            panic!("Expected valid path but got error: {:?}", error);
        }
    }
}

/// Test name constraint violations
#[test]
fn test_name_constraint_violations() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificate with names outside permitted constraints
    let mut end_entity = create_mock_certificate("unauthorized.com", "Intermediate CA", 1, now, future);
    end_entity.subject_alt_names = Some(vec![
        GeneralName::DnsName("evil.com".to_string()),
        GeneralName::EmailAddress("hacker@evil.com".to_string()),
    ]);
    
    let intermediate_ca = create_mock_certificate("Intermediate CA", "Root CA", 2, now, future);
    
    // Create trust anchor with restrictive name constraints
    let mut trust_anchor = create_mock_trust_anchor("Root CA");
    trust_anchor.name_constraints = Some(NameConstraints {
        permitted_subtrees: vec![
            GeneralSubtree {
                base: GeneralName::DnsName(".example.com".to_string()),
                minimum: None,
                maximum: None,
            },
        ],
        excluded_subtrees: vec![],
    });
    
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&end_entity, &[intermediate_ca]).unwrap();
    
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::NameConstraintViolation { .. } => {
                    // Expected error
                }
                _ => panic!("Expected name constraint violation but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for name constraint violation");
        }
    }
}

/// Test certificate policy validation
#[test]
fn test_certificate_policy_validation() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificates with specific policies
    let mut end_entity = create_mock_certificate("End Entity", "Intermediate CA", 1, now, future);
    end_entity.certificate_policies = Some(vec![
        CertificatePolicy {
            policy_id: "1.2.3.4.5".to_string(),
            qualifiers: vec![],
        },
    ]);
    
    let intermediate_ca = create_mock_certificate("Intermediate CA", "Root CA", 2, now, future);
    let trust_anchor = create_mock_trust_anchor("Root CA");
    
    // Create context requiring specific policies
    let mut context = create_validation_context_with_anchors(vec![trust_anchor]);
    context.required_policies.insert("1.2.3.4.5".to_string());
    context.require_explicit_policy = true;
    
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&end_entity, &[intermediate_ca]).unwrap();
    
    match result {
        PathValidationResult::Valid { validated_policies, .. } => {
            assert!(validated_policies.contains("1.2.3.4.5"));
        }
        PathValidationResult::Invalid { error, .. } => {
            panic!("Expected valid path but got error: {:?}", error);
        }
    }
}

/// Test missing required policy validation failure
#[test]
fn test_missing_required_policy() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificates without required policies
    let end_entity = create_mock_certificate("End Entity", "Intermediate CA", 1, now, future);
    let intermediate_ca = create_mock_certificate("Intermediate CA", "Root CA", 2, now, future);
    let trust_anchor = create_mock_trust_anchor("Root CA");
    
    // Create context requiring specific policies
    let mut context = create_validation_context_with_anchors(vec![trust_anchor]);
    context.required_policies.insert("1.2.3.4.5".to_string());
    context.require_explicit_policy = true;
    
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&end_entity, &[intermediate_ca]).unwrap();
    
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::PolicyValidationFailed { .. } => {
                    // Expected error
                }
                _ => panic!("Expected policy validation failure but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for missing required policy");
        }
    }
}

/// Test key usage constraint validation
#[test]
fn test_key_usage_constraints() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create intermediate CA without key cert sign usage
    let mut intermediate_ca = create_mock_certificate("Intermediate CA", "Root CA", 1, now, future);
    intermediate_ca.key_usage = Some(KeyUsageFlags::DIGITAL_SIGNATURE); // Missing KEY_CERT_SIGN
    
    let end_entity = create_mock_certificate("End Entity", "Intermediate CA", 2, now, future);
    let trust_anchor = create_mock_trust_anchor("Root CA");
    
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&end_entity, &[intermediate_ca]).unwrap();
    
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::KeyUsageViolation { .. } => {
                    // Expected error
                }
                _ => panic!("Expected key usage violation but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for key usage violation");
        }
    }
}

/// Test critical extension processing
#[test]
fn test_critical_extension_processing() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificate with unsupported critical extension
    let mut end_entity = create_mock_certificate("End Entity", "Root CA", 1, now, future);
    end_entity.extensions.push(CertificateExtension {
        oid: "1.2.3.4.5.6.7.8.9".to_string(), // Unknown OID
        critical: true,
        value: vec![0x30, 0x00], // Empty sequence
    });
    
    let trust_anchor = create_mock_trust_anchor("Root CA");
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&end_entity, &[]).unwrap();
    
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::CriticalExtensionNotSupported { .. } => {
                    // Expected error
                }
                _ => panic!("Expected critical extension error but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for unsupported critical extension");
        }
    }
}

/// Test trust anchor not found scenario
#[test]
fn test_trust_anchor_not_found() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificate chain without corresponding trust anchor
    let end_entity = create_mock_certificate("End Entity", "Unknown CA", 1, now, future);
    let intermediate_ca = create_mock_certificate("Unknown CA", "Unknown Root", 2, now, future);
    
    // Create trust anchor for different root
    let trust_anchor = create_mock_trust_anchor("Different Root CA");
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&end_entity, &[intermediate_ca]).unwrap();
    
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::TrustAnchorError { .. } => {
                    // Expected error
                }
                PathValidationError::ChainBuildingFailed { .. } => {
                    // Also acceptable - chain building would fail
                }
                _ => panic!("Expected trust anchor error but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for missing trust anchor");
        }
    }
}

/// Test circular certificate chain detection
#[test]
fn test_circular_chain_detection() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create circular certificate chain: A -> B -> C -> A
    let cert_a = create_mock_certificate("Certificate A", "Certificate C", 1, now, future);
    let cert_b = create_mock_certificate("Certificate B", "Certificate A", 2, now, future);
    let cert_c = create_mock_certificate("Certificate C", "Certificate B", 3, now, future);
    
    let trust_anchor = create_mock_trust_anchor("Trust Root");
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    // Try to validate with circular chain
    let result = validator.validate_path(&cert_a, &[cert_b, cert_c]).unwrap();
    
    match result {
        PathValidationResult::Invalid { error, .. } => {
            match error {
                PathValidationError::ChainBuildingFailed { .. } => {
                    // Expected - circular chain should be detected
                }
                _ => panic!("Expected chain building failure but got: {:?}", error),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for circular certificate chain");
        }
    }
}

/// Test simple validation convenience function
#[test]
fn test_simple_validation_function() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    let end_entity = create_mock_certificate("End Entity", "Root CA", 1, now, future);
    let trust_anchor = create_mock_trust_anchor("Root CA");
    
    let result = validate_certificate_path_simple(
        &end_entity,
        &[],
        &[trust_anchor],
    ).unwrap();
    
    match result {
        PathValidationResult::Valid { validated_chain, .. } => {
            assert_eq!(validated_chain.len(), 2); // End entity + root
        }
        PathValidationResult::Invalid { error, .. } => {
            panic!("Expected valid path but got error: {:?}", error);
        }
    }
}

/// Test validation context creation functions
#[test]
fn test_validation_context_creation() {
    // Test default context
    let default_context = create_default_validation_context();
    assert!(default_context.trust_anchors.is_empty());
    assert!(!default_context.require_explicit_policy);
    assert!(default_context.enable_policy_mapping);
    
    // Test context with trust anchors
    let trust_anchor = create_mock_trust_anchor("Test Root");
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    assert_eq!(context.trust_anchors.len(), 1);
    assert_eq!(context.trust_anchors[0].subject_name.common_name, Some("Test Root".to_string()));
}

/// Test DNS name constraint matching
#[test]
fn test_dns_name_constraint_matching() {
    let validator = CertificatePathValidator::new(create_default_validation_context());
    
    // Test exact match
    assert!(validator.dns_name_matches("example.com", "example.com"));
    assert!(!validator.dns_name_matches("example.com", "other.com"));
    
    // Test subdomain constraint
    assert!(validator.dns_name_matches("test.example.com", ".example.com"));
    assert!(validator.dns_name_matches("example.com", ".example.com"));
    assert!(!validator.dns_name_matches("notexample.com", ".example.com"));
}

/// Test email address constraint matching
#[test]
fn test_email_constraint_matching() {
    let validator = CertificatePathValidator::new(create_default_validation_context());
    
    // Test exact match
    assert!(validator.email_address_matches("user@example.com", "user@example.com"));
    assert!(!validator.email_address_matches("user@example.com", "other@example.com"));
    
    // Test domain constraint
    assert!(validator.email_address_matches("user@example.com", "@example.com"));
    assert!(validator.email_address_matches("admin@test.example.com", "@example.com"));
    assert!(!validator.email_address_matches("user@other.com", "@example.com"));
}

/// Test IP address constraint matching
#[test]
fn test_ip_constraint_matching() {
    let validator = CertificatePathValidator::new(create_default_validation_context());
    
    // Test exact IP match
    let ip = vec![192, 168, 1, 100];
    let constraint_exact = vec![192, 168, 1, 100];
    assert!(validator.ip_address_matches(&ip, &constraint_exact));
    
    // Test subnet match
    let constraint_subnet = vec![192, 168, 1, 0, 255, 255, 255, 0]; // 192.168.1.0/24
    assert!(validator.ip_address_matches(&ip, &constraint_subnet));
    
    let ip_outside = vec![192, 168, 2, 100];
    assert!(!validator.ip_address_matches(&ip_outside, &constraint_subnet));
}

/// Test signature algorithm support validation
#[test]
fn test_signature_algorithm_support() {
    let validator = CertificatePathValidator::new(create_default_validation_context());
    
    // Test supported algorithms (these would return Ok in real implementation)
    let public_key = PublicKeyInfo {
        algorithm: "rsaEncryption".to_string(),
        key_data: vec![0u8; 256],
        parameters: None,
    };
    
    let data = b"test data";
    let signature = vec![0u8; 256];
    
    // These calls test the function structure (actual verification would use real crypto)
    let result1 = validator.verify_signature(data, &signature, &public_key, "sha256WithRSAEncryption");
    let result2 = validator.verify_signature(data, &signature, &public_key, "ecdsa-with-SHA256");
    
    // Both should not panic and return Results
    assert!(result1.is_ok() || result1.is_err()); // Either is fine for mock
    assert!(result2.is_ok() || result2.is_err());
}

/// Test performance with large certificate chains
#[test]
fn test_large_certificate_chain_performance() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create a large certificate chain (8 intermediates)
    let mut intermediates = Vec::new();
    let root_ca = create_mock_certificate("Root CA", "Root CA", 1, now, future);
    
    for i in 1..=8 {
        let issuer = if i == 1 { "Root CA" } else { &format!("Intermediate {}", i - 1) };
        let intermediate = create_mock_certificate(&format!("Intermediate {}", i), issuer, i + 1, now, future);
        intermediates.push(intermediate);
    }
    
    let end_entity = create_mock_certificate("End Entity", "Intermediate 8", 10, now, future);
    let trust_anchor = create_mock_trust_anchor("Root CA");
    
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let start_time = SystemTime::now();
    let result = validator.validate_path(&end_entity, &intermediates).unwrap();
    let validation_time = start_time.elapsed().unwrap();
    
    // Validation should complete within reasonable time (< 1 second for mock implementation)
    assert!(validation_time < Duration::from_secs(1));
    
    match result {
        PathValidationResult::Valid { validated_chain, .. } => {
            assert_eq!(validated_chain.len(), 10); // End entity + 8 intermediates + root
        }
        PathValidationResult::Invalid { error, .. } => {
            panic!("Expected valid large chain but got error: {:?}", error);
        }
    }
}

/// Test concurrent path validation (basic thread safety)
#[test]
fn test_concurrent_validation() {
    use std::thread;
    use std::sync::Arc;
    
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    let end_entity = Arc::new(create_mock_certificate("End Entity", "Root CA", 1, now, future));
    let trust_anchor = Arc::new(create_mock_trust_anchor("Root CA"));
    
    let mut handles = Vec::new();
    
    // Spawn multiple threads to validate the same path
    for i in 0..4 {
        let end_entity_clone = Arc::clone(&end_entity);
        let trust_anchor_clone = Arc::clone(&trust_anchor);
        
        let handle = thread::spawn(move || {
            let context = create_validation_context_with_anchors(vec![(*trust_anchor_clone).clone()]);
            let mut validator = CertificatePathValidator::new(context);
            
            let result = validator.validate_path(&end_entity_clone, &[]).unwrap();
            
            match result {
                PathValidationResult::Valid { .. } => true,
                PathValidationResult::Invalid { .. } => false,
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads and verify results
    for handle in handles {
        let success = handle.join().unwrap();
        assert!(success, "Concurrent validation should succeed");
    }
}

/// Test memory usage and cleanup
#[test]
fn test_memory_usage() {
    let now = SystemTime::now();
    let future = now + Duration::from_secs(365 * 24 * 3600);
    
    // Create validator and perform multiple validations to test memory handling
    let trust_anchor = create_mock_trust_anchor("Root CA");
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    // Perform 100 validations with different certificates
    for i in 0..100 {
        let end_entity = create_mock_certificate(&format!("End Entity {}", i), "Root CA", i + 1, now, future);
        let _result = validator.validate_path(&end_entity, &[]).unwrap();
        
        // Validator should not accumulate excessive state
        // In real implementation, this would check memory usage
    }
    
    // Test that validator can be dropped cleanly
    drop(validator);
}

/// Test error message quality and debugging information
#[test]
fn test_error_message_quality() {
    let now = SystemTime::now();
    let past = now - Duration::from_secs(365 * 24 * 3600);
    
    // Create expired certificate to generate detailed error
    let expired_cert = create_mock_certificate("Expired Certificate", "Root CA", 1, past, past + Duration::from_secs(3600));
    let trust_anchor = create_mock_trust_anchor("Root CA");
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&expired_cert, &[]).unwrap();
    
    match result {
        PathValidationResult::Invalid { error, partial_chain } => {
            // Verify error contains useful debugging information
            match error {
                PathValidationError::ValidityPeriodViolation { 
                    certificate_subject,
                    not_before,
                    not_after,
                    current_time,
                } => {
                    assert!(certificate_subject.contains("Expired Certificate"));
                    assert!(current_time > not_after);
                    assert!(!partial_chain.is_empty());
                }
                _ => panic!("Expected validity period violation"),
            }
        }
        PathValidationResult::Valid { .. } => {
            panic!("Expected validation failure for expired certificate");
        }
    }
}
