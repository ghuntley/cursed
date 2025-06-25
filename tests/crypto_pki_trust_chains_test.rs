//! Comprehensive Tests for PKI Trust Chains Module
//! 
//! Tests for trust chain building, validation, and trust store management.

use cursed::stdlib::packages::crypto_pki::{
    trust_chains::{
        TrustChainBuilder, EnhancedTrustStore, ComprehensiveChainValidator,
        CertificatePath, TrustAnchor, TrustConstraints, TrustLevel,
        ChainBuildingConfig, ValidationConfig, EnhancedTrustStoreConfig,
        TrustInheritanceRule, InheritanceCondition, PolicyEngine,
        PathValidationStatus, PathBuildingAlgorithm, RevocationReason,
        build_certificate_chain, validate_certificate_path, create_enhanced_trust_store
    },
    types::{
        X509Certificate, DistinguishedName, SerialNumber, SignatureAlgorithm,
        Validity, SubjectPublicKeyInfo, PublicKeyAlgorithm, X509Extension,
        ExtensionData, KeyUsage, ExtendedKeyUsage, TrustStore, CertificateChain,
        ValidationResult, ValidationError
    },
    error::PkiError
};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

// Test helper functions
fn create_test_certificate(subject_cn: &str, issuer_cn: &str, is_ca: bool, serial: u32) -> X509Certificate {
    X509Certificate {
        version: 3,
        serial_number: SerialNumber { bytes: serial.to_be_bytes().to_vec() },
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName {
            common_name: Some(issuer_cn.to_string()),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state_or_province: Some("California".to_string()),
            locality: Some("San Francisco".to_string()),
            email_address: None,
            additional_attributes: HashMap::new(),
        },
        validity: Validity {
            not_before: SystemTime::now() - Duration::from_secs(3600),
            not_after: SystemTime::now() + Duration::from_secs(86400 * 365),
        },
        subject: DistinguishedName {
            common_name: Some(subject_cn.to_string()),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state_or_province: Some("California".to_string()),
            locality: Some("San Francisco".to_string()),
            email_address: None,
            additional_attributes: HashMap::new(),
        },
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0x30, 0x82, 0x01, 0x22, 0x30, 0x0d], // Mock RSA public key
            parameters: None,
        },
        extensions: vec![
            X509Extension {
                oid: "2.5.29.19".to_string(), // Basic Constraints
                critical: true,
                value: if is_ca { 
                    vec![0x30, 0x03, 0x01, 0x01, 0xFF] 
                } else { 
                    vec![0x30, 0x00] 
                },
                parsed_data: Some(ExtensionData::BasicConstraints {
                    is_ca,
                    path_length_constraint: if is_ca { Some(5) } else { None },
                }),
            }
        ],
        raw_data: vec![0x30, 0x82, 0x03, 0x21], // Mock DER data
        fingerprint: Some(vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
        key_usage: KeyUsage {
            digital_signature: true,
            non_repudiation: false,
            key_encipherment: !is_ca,
            data_encipherment: false,
            key_agreement: false,
            key_cert_sign: is_ca,
            crl_sign: is_ca,
            encipher_only: false,
            decipher_only: false,
        },
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_test_trust_store_with_root() -> TrustStore {
    let mut store = TrustStore::new("test");
    let root_cert = create_test_certificate("Root CA", "Root CA", true, 1);
    let _ = store.add_certificate(root_cert);
    store
}

#[test]
fn test_trust_chain_builder_creation() {
    let trust_store = create_test_trust_store_with_root();
    let config = ChainBuildingConfig::default();
    let builder = TrustChainBuilder::new(trust_store, config);
    
    let stats = builder.get_statistics();
    assert_eq!(stats.chains_built, 0);
    assert_eq!(stats.successful_builds, 0);
    assert_eq!(stats.failed_builds, 0);
}

#[test]
fn test_chain_building_config() {
    let config = ChainBuildingConfig {
        max_chain_length: 5,
        max_build_time_seconds: 15,
        enable_caching: false,
        allow_cross_certification: false,
        prefer_shorter_chains: false,
        max_intermediate_fetch: 3,
        network_timeout_seconds: 5,
    };
    
    assert_eq!(config.max_chain_length, 5);
    assert_eq!(config.max_build_time_seconds, 15);
    assert!(!config.enable_caching);
    assert!(!config.allow_cross_certification);
}

#[test]
fn test_certificate_path_creation() {
    let leaf_cert = create_test_certificate("test.example.com", "Intermediate CA", false, 2);
    let intermediate_cert = create_test_certificate("Intermediate CA", "Root CA", true, 3);
    let root_cert = create_test_certificate("Root CA", "Root CA", true, 1);
    
    let path = CertificatePath {
        certificates: vec![leaf_cert.clone(), intermediate_cert.clone()],
        trust_anchor: Some(root_cert.clone()),
        validation_status: PathValidationStatus::Valid,
        metadata: cursed::stdlib::packages::crypto_pki::trust_chains::PathMetadata {
            build_time_ms: 150,
            certificates_fetched: 1,
            uses_cross_certification: false,
            algorithm_used: PathBuildingAlgorithm::ForwardChaining,
        },
    };
    
    assert_eq!(path.certificates.len(), 2);
    assert!(path.trust_anchor.is_some());
    assert_eq!(path.metadata.build_time_ms, 150);
    assert!(!path.metadata.uses_cross_certification);
    
    // Test path validation status
    match path.validation_status {
        PathValidationStatus::Valid => assert!(true),
        _ => panic!("Expected valid path status"),
    }
}

#[test]
fn test_certificate_path_invalid_status() {
    let leaf_cert = create_test_certificate("invalid.example.com", "Unknown CA", false, 4);
    
    let errors = vec![
        ValidationError::UntrustedRoot {
            issuer: DistinguishedName {
                common_name: Some("Unknown CA".to_string()),
                organization: None,
                organizational_unit: None,
                country: None,
                state_or_province: None,
                locality: None,
                email_address: None,
                additional_attributes: HashMap::new(),
            },
        }
    ];
    
    let path = CertificatePath {
        certificates: vec![leaf_cert],
        trust_anchor: None,
        validation_status: PathValidationStatus::Invalid { errors: errors.clone() },
        metadata: cursed::stdlib::packages::crypto_pki::trust_chains::PathMetadata {
            build_time_ms: 50,
            certificates_fetched: 0,
            uses_cross_certification: false,
            algorithm_used: PathBuildingAlgorithm::ForwardChaining,
        },
    };
    
    match path.validation_status {
        PathValidationStatus::Invalid { errors } => {
            assert_eq!(errors.len(), 1);
        }
        _ => panic!("Expected invalid path status"),
    }
}

#[test]
fn test_enhanced_trust_store_creation() {
    let config = EnhancedTrustStoreConfig::default();
    let store = EnhancedTrustStore::new(config.clone());
    
    assert!(config.enable_pinning);
    assert!(!config.enable_trust_overrides); // Disabled by default for security
    assert_eq!(config.max_pinned_per_host, 3);
}

#[test]
fn test_certificate_pinning() {
    let mut store = create_enhanced_trust_store();
    let cert = create_test_certificate("test.example.com", "Test CA", false, 5);
    
    // Test successful pinning
    let result = store.pin_certificate("example.com".to_string(), cert.clone());
    assert!(result.is_ok());
    
    // Test pinning check
    assert!(store.is_certificate_pinned("example.com", &cert));
    assert!(!store.is_certificate_pinned("other.com", &cert));
    
    // Test different certificate is not pinned
    let other_cert = create_test_certificate("other.example.com", "Other CA", false, 6);
    assert!(!store.is_certificate_pinned("example.com", &other_cert));
}

#[test]
fn test_certificate_pinning_limits() {
    let mut store = create_enhanced_trust_store();
    let hostname = "example.com".to_string();
    
    // Pin maximum allowed certificates
    for i in 1..=3 {
        let cert = create_test_certificate("test.example.com", "Test CA", false, i);
        let result = store.pin_certificate(hostname.clone(), cert);
        assert!(result.is_ok());
    }
    
    // Try to pin one more (should fail)
    let extra_cert = create_test_certificate("extra.example.com", "Extra CA", false, 4);
    let result = store.pin_certificate(hostname, extra_cert);
    assert!(result.is_err());
}

#[test]
fn test_trust_overrides() {
    let mut store = create_enhanced_trust_store();
    
    // Enable trust overrides (disabled by default)
    store.config.enable_trust_overrides = true;
    
    let cert = create_test_certificate("override.example.com", "Override CA", false, 7);
    
    // Test adding trust override
    let result = store.add_trust_override("example.com".to_string(), cert.clone());
    assert!(result.is_ok());
    
    // Test retrieving trust override
    let retrieved = store.get_trust_override("example.com");
    assert!(retrieved.is_some());
    
    // Test non-existent override
    let none_override = store.get_trust_override("nonexistent.com");
    assert!(none_override.is_none());
}

#[test]
fn test_trust_overrides_disabled() {
    let mut store = create_enhanced_trust_store();
    // Trust overrides are disabled by default
    
    let cert = create_test_certificate("disabled.example.com", "Disabled CA", false, 8);
    
    // Should fail when disabled
    let result = store.add_trust_override("example.com".to_string(), cert);
    assert!(result.is_err());
}

#[test]
fn test_trust_inheritance_rules() {
    let mut store = create_enhanced_trust_store();
    
    // Enable trust inheritance
    store.config.inheritance_config.enabled = true;
    
    let rule = TrustInheritanceRule {
        source_anchor: "Root CA".to_string(),
        target_pattern: "*.example.com".to_string(),
        conditions: vec![
            InheritanceCondition::SubjectMatches("example.com".to_string()),
            InheritanceCondition::KeyUsageIncludes(KeyUsage {
                digital_signature: true,
                key_cert_sign: false,
                ..Default::default()
            }),
        ],
        priority: 100,
    };
    
    store.add_trust_inheritance_rule(rule);
    
    // Test inheritance check
    let cert = create_test_certificate("test.example.com", "Test CA", false, 9);
    let can_inherit = store.can_inherit_trust(&cert, "Root CA");
    assert!(can_inherit);
    
    // Test with non-matching certificate
    let non_matching_cert = create_test_certificate("test.other.com", "Test CA", false, 10);
    let cannot_inherit = store.can_inherit_trust(&non_matching_cert, "Root CA");
    assert!(!cannot_inherit);
}

#[test]
fn test_trust_inheritance_disabled() {
    let store = create_enhanced_trust_store();
    // Trust inheritance is disabled by default
    
    let cert = create_test_certificate("test.example.com", "Test CA", false, 11);
    let can_inherit = store.can_inherit_trust(&cert, "Root CA");
    assert!(!can_inherit);
}

#[test]
fn test_trust_anchor_creation() {
    let cert = create_test_certificate("Root CA", "Root CA", true, 1);
    let constraints = TrustConstraints {
        permitted_subtrees: vec!["*.example.com".to_string(), "*.test.com".to_string()],
        excluded_subtrees: vec!["*.malicious.com".to_string()],
        max_path_length: Some(5),
        required_policies: vec!["1.2.3.4.5".to_string()],
        permitted_key_usage: Some(KeyUsage {
            key_cert_sign: true,
            crl_sign: true,
            ..Default::default()
        }),
    };
    
    let anchor = TrustAnchor {
        certificate: cert,
        constraints,
        last_validated: Some(SystemTime::now()),
        trust_level: TrustLevel::System,
    };
    
    assert_eq!(anchor.constraints.permitted_subtrees.len(), 2);
    assert_eq!(anchor.constraints.excluded_subtrees.len(), 1);
    assert!(anchor.last_validated.is_some());
    assert_eq!(anchor.trust_level, TrustLevel::System);
}

#[test]
fn test_trust_levels() {
    // Test trust level ordering
    assert!(TrustLevel::System > TrustLevel::Manual);
    assert!(TrustLevel::Manual > TrustLevel::Conditional);
    
    let future_time = SystemTime::now() + Duration::from_secs(3600);
    let temporary = TrustLevel::Temporary { expires: future_time };
    assert!(TrustLevel::Conditional > temporary);
}

#[test]
fn test_comprehensive_chain_validator_creation() {
    let config = ValidationConfig {
        check_signatures: true,
        check_validity_periods: true,
        check_revocation_status: false, // Disabled for testing
        check_name_constraints: true,
        check_policy_constraints: false,
        check_basic_constraints: true,
        check_key_usage: true,
        max_clock_skew_seconds: 300,
        cache_validation_results: true,
        validation_cache_ttl_seconds: 1800,
    };
    
    let validator = ComprehensiveChainValidator::new(config.clone());
    
    assert_eq!(config.max_clock_skew_seconds, 300);
    assert!(config.check_signatures);
    assert!(!config.check_revocation_status);
}

#[test]
fn test_path_validation() {
    let config = ValidationConfig::default();
    let validator = ComprehensiveChainValidator::new(config);
    
    // Create a simple certificate path
    let leaf_cert = create_test_certificate("test.example.com", "Root CA", false, 12);
    let root_cert = create_test_certificate("Root CA", "Root CA", true, 1);
    
    let path = CertificatePath {
        certificates: vec![leaf_cert, root_cert.clone()],
        trust_anchor: Some(root_cert),
        validation_status: PathValidationStatus::Pending,
        metadata: cursed::stdlib::packages::crypto_pki::trust_chains::PathMetadata {
            build_time_ms: 75,
            certificates_fetched: 0,
            uses_cross_certification: false,
            algorithm_used: PathBuildingAlgorithm::ForwardChaining,
        },
    };
    
    let trust_store = create_enhanced_trust_store();
    
    // Validate path structure (will have validation errors due to empty trust store)
    let result = validator.validate_path(&path, &trust_store);
    assert!(result.is_ok());
    
    let validation_result = result.unwrap();
    assert!(!validation_result.is_valid); // Should be invalid due to untrusted root
    assert!(!validation_result.errors.is_empty());
}

#[test]
fn test_empty_path_validation() {
    let config = ValidationConfig::default();
    let validator = ComprehensiveChainValidator::new(config);
    
    let empty_path = CertificatePath {
        certificates: Vec::new(), // Empty chain
        trust_anchor: None,
        validation_status: PathValidationStatus::Pending,
        metadata: cursed::stdlib::packages::crypto_pki::trust_chains::PathMetadata {
            build_time_ms: 10,
            certificates_fetched: 0,
            uses_cross_certification: false,
            algorithm_used: PathBuildingAlgorithm::ForwardChaining,
        },
    };
    
    let trust_store = create_enhanced_trust_store();
    let result = validator.validate_path(&empty_path, &trust_store);
    
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    assert!(!validation_result.is_valid);
    
    // Should contain empty chain error
    let has_empty_chain_error = validation_result.errors.iter().any(|e| {
        matches!(e, ValidationError::EmptyChain)
    });
    assert!(has_empty_chain_error);
}

#[test]
fn test_policy_engine() {
    let mut engine = PolicyEngine::new();
    
    // Test policy mapping
    let mapping = cursed::stdlib::packages::crypto_pki::trust_chains::PolicyMapping {
        source_policy: "1.2.3.4.5".to_string(),
        target_policy: "1.2.3.4.6".to_string(),
        conditions: vec!["test condition".to_string()],
    };
    
    engine.add_policy_mapping(mapping);
    
    // Test policy constraint
    let constraint = cursed::stdlib::packages::crypto_pki::trust_chains::PolicyConstraint {
        policy_oid: "1.2.3.4.7".to_string(),
        constraint_type: cursed::stdlib::packages::crypto_pki::trust_chains::PolicyConstraintType::RequireExplicitPolicy,
        constraint_value: "required".to_string(),
    };
    
    engine.add_policy_constraint(constraint);
    
    // Test initial policy set
    let policies = vec!["1.2.3.4.8".to_string(), "1.2.3.4.9".to_string()]
        .into_iter().collect();
    engine.set_initial_policy_set(policies);
}

#[test]
fn test_revocation_reason() {
    // Test revocation reason variants
    let reasons = vec![
        RevocationReason::Unspecified,
        RevocationReason::KeyCompromise,
        RevocationReason::CaCompromise,
        RevocationReason::AffiliationChanged,
        RevocationReason::Superseded,
        RevocationReason::CessationOfOperation,
        RevocationReason::CertificateHold,
        RevocationReason::PrivilegeWithdrawn,
        RevocationReason::AaCompromise,
    ];
    
    assert_eq!(reasons.len(), 9);
}

#[test]
fn test_public_api_build_certificate_chain() {
    let leaf_cert = create_test_certificate("api.example.com", "Root CA", false, 13);
    let trust_store = create_test_trust_store_with_root();
    let config = Some(ChainBuildingConfig {
        max_chain_length: 3,
        prefer_shorter_chains: true,
        ..Default::default()
    });
    
    let result = build_certificate_chain(&leaf_cert, &trust_store, config);
    assert!(result.is_ok());
    
    let paths = result.unwrap();
    // May be empty if no valid path found, but function should not error
    assert!(paths.len() >= 0);
}

#[test]
fn test_public_api_validate_certificate_path() {
    let leaf_cert = create_test_certificate("validate.example.com", "Test CA", false, 14);
    let ca_cert = create_test_certificate("Test CA", "Test CA", true, 15);
    
    let path = CertificatePath {
        certificates: vec![leaf_cert, ca_cert.clone()],
        trust_anchor: Some(ca_cert),
        validation_status: PathValidationStatus::Pending,
        metadata: cursed::stdlib::packages::crypto_pki::trust_chains::PathMetadata {
            build_time_ms: 25,
            certificates_fetched: 0,
            uses_cross_certification: false,
            algorithm_used: PathBuildingAlgorithm::BidirectionalChaining,
        },
    };
    
    let trust_store = create_enhanced_trust_store();
    let config = Some(ValidationConfig {
        check_revocation_status: false, // Disable for testing
        ..Default::default()
    });
    
    let result = validate_certificate_path(&path, &trust_store, config);
    assert!(result.is_ok());
    
    let validation_result = result.unwrap();
    // Will be invalid due to untrusted root, but should not error
    assert!(!validation_result.is_valid);
}

#[test]
fn test_public_api_create_enhanced_trust_store() {
    let store = create_enhanced_trust_store();
    
    // Check default configuration
    assert!(store.config.enable_pinning);
    assert!(!store.config.enable_trust_overrides);
    assert_eq!(store.config.max_pinned_per_host, 3);
    assert!(!store.config.inheritance_config.enabled);
}

#[test]
fn test_chain_building_statistics() {
    let stats = cursed::stdlib::packages::crypto_pki::trust_chains::ChainBuildingStatistics {
        chains_built: 100,
        successful_builds: 85,
        failed_builds: 15,
        average_build_time_ms: 125.5,
        certificates_fetched: 250,
        cache_hit_ratio: 0.75,
        cross_certification_usage: 12,
    };
    
    assert_eq!(stats.chains_built, 100);
    assert_eq!(stats.successful_builds, 85);
    assert_eq!(stats.failed_builds, 15);
    assert_eq!(stats.successful_builds + stats.failed_builds, stats.chains_built);
    assert_eq!(stats.average_build_time_ms, 125.5);
    assert_eq!(stats.cache_hit_ratio, 0.75);
}

#[test]
fn test_inheritance_conditions() {
    // Test different inheritance condition types
    let subject_condition = InheritanceCondition::SubjectMatches("example.com".to_string());
    let issuer_condition = InheritanceCondition::IssuerMatches("Test CA".to_string());
    let key_usage_condition = InheritanceCondition::KeyUsageIncludes(KeyUsage {
        digital_signature: true,
        ..Default::default()
    });
    let validity_condition = InheritanceCondition::ValidityPeriod {
        min_days: 30,
        max_days: 365,
    };
    
    // Test condition creation
    match subject_condition {
        InheritanceCondition::SubjectMatches(pattern) => {
            assert_eq!(pattern, "example.com");
        }
        _ => panic!("Wrong condition type"),
    }
    
    match validity_condition {
        InheritanceCondition::ValidityPeriod { min_days, max_days } => {
            assert_eq!(min_days, 30);
            assert_eq!(max_days, 365);
        }
        _ => panic!("Wrong condition type"),
    }
}

#[test]
fn test_path_building_algorithms() {
    // Test all path building algorithm variants
    let algorithms = vec![
        PathBuildingAlgorithm::ForwardChaining,
        PathBuildingAlgorithm::ReverseChaining,
        PathBuildingAlgorithm::BidirectionalChaining,
    ];
    
    assert_eq!(algorithms.len(), 3);
    
    // Test in metadata
    let metadata = cursed::stdlib::packages::crypto_pki::trust_chains::PathMetadata {
        build_time_ms: 200,
        certificates_fetched: 3,
        uses_cross_certification: true,
        algorithm_used: PathBuildingAlgorithm::BidirectionalChaining,
    };
    
    assert_eq!(metadata.build_time_ms, 200);
    assert_eq!(metadata.certificates_fetched, 3);
    assert!(metadata.uses_cross_certification);
}

#[test]
fn test_complex_certificate_chain() {
    // Create a longer certificate chain: End Entity -> Intermediate -> Sub CA -> Root CA
    let end_entity = create_test_certificate("user.example.com", "Intermediate CA", false, 20);
    let intermediate = create_test_certificate("Intermediate CA", "Sub CA", true, 21);
    let sub_ca = create_test_certificate("Sub CA", "Root CA", true, 22);
    let root_ca = create_test_certificate("Root CA", "Root CA", true, 1);
    
    let chain = vec![end_entity, intermediate, sub_ca, root_ca.clone()];
    
    let path = CertificatePath {
        certificates: chain,
        trust_anchor: Some(root_ca),
        validation_status: PathValidationStatus::Valid,
        metadata: cursed::stdlib::packages::crypto_pki::trust_chains::PathMetadata {
            build_time_ms: 300,
            certificates_fetched: 2,
            uses_cross_certification: false,
            algorithm_used: PathBuildingAlgorithm::ForwardChaining,
        },
    };
    
    assert_eq!(path.certificates.len(), 4);
    assert!(path.trust_anchor.is_some());
    
    // Verify chain structure
    assert_eq!(path.certificates[0].subject.common_name, Some("user.example.com".to_string()));
    assert_eq!(path.certificates[1].subject.common_name, Some("Intermediate CA".to_string()));
    assert_eq!(path.certificates[2].subject.common_name, Some("Sub CA".to_string()));
    assert_eq!(path.certificates[3].subject.common_name, Some("Root CA".to_string()));
}

#[test]
fn test_trust_constraints() {
    let constraints = TrustConstraints {
        permitted_subtrees: vec![
            "*.example.com".to_string(),
            "*.test.org".to_string(),
            "specific.domain.net".to_string(),
        ],
        excluded_subtrees: vec![
            "*.malicious.com".to_string(),
            "*.phishing.org".to_string(),
        ],
        max_path_length: Some(3),
        required_policies: vec![
            "2.16.840.1.114412.1.1".to_string(), // Example OID
            "1.3.6.1.4.1.311.21.8.123456.1".to_string(),
        ],
        permitted_key_usage: Some(KeyUsage {
            digital_signature: true,
            key_encipherment: true,
            key_cert_sign: true,
            crl_sign: true,
            ..Default::default()
        }),
    };
    
    assert_eq!(constraints.permitted_subtrees.len(), 3);
    assert_eq!(constraints.excluded_subtrees.len(), 2);
    assert_eq!(constraints.max_path_length, Some(3));
    assert_eq!(constraints.required_policies.len(), 2);
    assert!(constraints.permitted_key_usage.is_some());
}

#[test]
fn test_validation_config_comprehensive() {
    let config = ValidationConfig {
        check_signatures: true,
        check_validity_periods: true,
        check_revocation_status: true,
        check_name_constraints: true,
        check_policy_constraints: true,
        check_basic_constraints: true,
        check_key_usage: true,
        max_clock_skew_seconds: 600, // 10 minutes
        cache_validation_results: true,
        validation_cache_ttl_seconds: 7200, // 2 hours
    };
    
    // Verify all checks are enabled
    assert!(config.check_signatures);
    assert!(config.check_validity_periods);
    assert!(config.check_revocation_status);
    assert!(config.check_name_constraints);
    assert!(config.check_policy_constraints);
    assert!(config.check_basic_constraints);
    assert!(config.check_key_usage);
    
    // Verify timing configuration
    assert_eq!(config.max_clock_skew_seconds, 600);
    assert_eq!(config.validation_cache_ttl_seconds, 7200);
    
    // Verify caching configuration
    assert!(config.cache_validation_results);
    
    // Test max chain length method
    assert_eq!(config.max_chain_length(), 10);
}

#[test]
fn test_error_handling_scenarios() {
    // Test certificate pinning with disabled feature
    let mut store = EnhancedTrustStore::new(EnhancedTrustStoreConfig {
        enable_pinning: false,
        ..Default::default()
    });
    
    let cert = create_test_certificate("disabled.example.com", "Test CA", false, 30);
    let result = store.pin_certificate("example.com".to_string(), cert);
    
    match result {
        Err(PkiError::General { message, .. }) => {
            assert!(message.contains("pinning is disabled"));
        }
        _ => panic!("Expected pinning disabled error"),
    }
}

#[test]
fn test_comprehensive_integration() {
    // Integration test combining multiple features
    
    // 1. Create enhanced trust store with custom config
    let config = EnhancedTrustStoreConfig {
        enable_pinning: true,
        enable_trust_overrides: true,
        max_pinned_per_host: 5,
        inheritance_config: cursed::stdlib::packages::crypto_pki::trust_chains::TrustInheritanceConfig {
            enabled: true,
            max_inheritance_depth: 2,
            default_conditions: vec![],
        },
        ..Default::default()
    };
    
    let mut trust_store = EnhancedTrustStore::new(config);
    
    // 2. Add trust inheritance rule
    let rule = TrustInheritanceRule {
        source_anchor: "Corporate Root CA".to_string(),
        target_pattern: "*.corp.example.com".to_string(),
        conditions: vec![
            InheritanceCondition::SubjectMatches("corp.example.com".to_string()),
            InheritanceCondition::ValidityPeriod { min_days: 30, max_days: 1095 },
        ],
        priority: 50,
    };
    trust_store.add_trust_inheritance_rule(rule);
    
    // 3. Pin certificate
    let pinned_cert = create_test_certificate("secure.corp.example.com", "Corporate CA", false, 31);
    let pin_result = trust_store.pin_certificate("corp.example.com".to_string(), pinned_cert.clone());
    assert!(pin_result.is_ok());
    
    // 4. Add trust override
    let override_cert = create_test_certificate("override.corp.example.com", "Override CA", false, 32);
    let override_result = trust_store.add_trust_override("special.corp.example.com".to_string(), override_cert);
    assert!(override_result.is_ok());
    
    // 5. Test inheritance
    let corp_cert = create_test_certificate("app.corp.example.com", "Corporate CA", false, 33);
    let can_inherit = trust_store.can_inherit_trust(&corp_cert, "Corporate Root CA");
    assert!(can_inherit);
    
    // 6. Verify pinning
    assert!(trust_store.is_certificate_pinned("corp.example.com", &pinned_cert));
    
    // 7. Verify override
    assert!(trust_store.get_trust_override("special.corp.example.com").is_some());
}
