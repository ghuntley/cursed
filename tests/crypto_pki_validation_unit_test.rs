//! PKI Certificate Validation Unit Tests
//!
//! Comprehensive unit tests for the PKI certificate validation module covering:
//! - Certificate validation functions
//! - Chain validation logic
//! - Policy validation
//! - Signature verification
//! - Revocation checking
//! - Error handling
//! - Performance and caching

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

/// Create a test certificate for validation testing
fn create_test_certificate() -> X509Certificate {
    let now = SystemTime::now();
    
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(12345),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName::from_common_name("Test CA"),
        validity: Validity {
            not_before: now - Duration::from_secs(86400), // 1 day ago
            not_after: now + Duration::from_secs(86400 * 365), // 1 year from now
        },
        subject: DistinguishedName::from_common_name("test.example.com"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0x30, 0x82, 0x01, 0x22], // Mock RSA public key
            parameters: None,
        },
        extensions: vec![
            X509Extension {
                oid: "2.5.29.17".to_string(), // Subject Alternative Name
                critical: false,
                value: vec![0x30, 0x0c, 0x82, 0x0a, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x63, 0x6f, 0x6d], 
                parsed_data: Some(ExtensionData::SubjectAlternativeName(vec![
                    GeneralName::DnsName("test.example.com".to_string()),
                ])),
            }
        ],
        raw_data: Vec::new(),
        fingerprint: Some(vec![0x12, 0x34, 0x56, 0x78]),
        key_usage: KeyUsage {
            digital_signature: true,
            key_encipherment: true,
            ..KeyUsage::default()
        },
        extended_key_usage: ExtendedKeyUsage {
            server_auth: true,
            ..ExtendedKeyUsage::default()
        },
    }
}

/// Create a CA certificate for testing
fn create_ca_certificate() -> X509Certificate {
    let now = SystemTime::now();
    
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(1),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName::from_common_name("Test Root CA"),
        validity: Validity {
            not_before: now - Duration::from_secs(86400 * 365), // 1 year ago
            not_after: now + Duration::from_secs(86400 * 365 * 10), // 10 years from now
        },
        subject: DistinguishedName::from_common_name("Test Root CA"),
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 4096 },
            public_key: vec![0x30, 0x82, 0x02, 0x22], // Mock RSA public key
            parameters: None,
        },
        extensions: vec![
            X509Extension {
                oid: "2.5.29.19".to_string(), // Basic Constraints
                critical: true,
                value: vec![0x30, 0x03, 0x01, 0x01, 0xFF],
                parsed_data: Some(ExtensionData::BasicConstraints {
                    is_ca: true,
                    path_length_constraint: None,
                }),
            }
        ],
        raw_data: Vec::new(),
        fingerprint: Some(vec![0x98, 0x76, 0x54, 0x32]),
        key_usage: KeyUsage {
            key_cert_sign: true,
            crl_sign: true,
            ..KeyUsage::default()
        },
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

/// Create an expired certificate for testing
fn create_expired_certificate() -> X509Certificate {
    let now = SystemTime::now();
    
    let mut cert = create_test_certificate();
    cert.validity = Validity {
        not_before: now - Duration::from_secs(86400 * 365 * 2), // 2 years ago
        not_after: now - Duration::from_secs(86400), // 1 day ago (expired)
    };
    cert
}

/// Create a test certificate chain
fn create_test_chain() -> CertificateChain {
    CertificateChain {
        end_entity: create_test_certificate(),
        intermediates: Vec::new(),
        root: Some(create_ca_certificate()),
    }
}

/// Create a test trust store
fn create_test_trust_store() -> TrustStore {
    let mut trust_store = TrustStore::new("test");
    trust_store.add_root_certificate(create_ca_certificate());
    trust_store
}

#[test]
fn test_certificate_validator_creation() {
    let validator = CertificateValidator::with_defaults();
    
    assert!(!validator.trust_stores.is_empty());
    assert!(!validator.chain_validators.is_empty());
    assert!(!validator.policy_validators.is_empty());
    assert!(!validator.signature_validators.is_empty());
    assert!(!validator.revocation_checkers.is_empty());
}

#[test]
fn test_validation_config_default() {
    let config = ValidationConfig::default();
    
    assert_eq!(config.max_chain_length, 10);
    assert!(config.strict_rfc5280_compliance);
    assert!(config.check_validity_dates);
    assert!(config.check_revocation);
    assert!(config.check_key_usage);
    assert!(config.check_basic_constraints);
    assert!(!config.allow_self_signed);
    assert_eq!(config.min_rsa_key_size, 2048);
    assert_eq!(config.network_timeout, Duration::from_secs(30));
}

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default();
    
    assert!(config.enable_caching);
    assert_eq!(config.max_cache_size, 1000);
    assert_eq!(config.success_cache_ttl, Duration::from_secs(3600));
    assert_eq!(config.failure_cache_ttl, Duration::from_secs(300));
}

#[test]
fn test_certificate_cache_creation() {
    let cache = CertificateCache::new();
    
    assert!(cache.results.is_empty());
    assert_eq!(cache.statistics.lookups, 0);
    assert_eq!(cache.statistics.hits, 0);
    assert_eq!(cache.statistics.misses, 0);
}

#[test]
fn test_validate_single_certificate() {
    let mut validator = CertificateValidator::with_defaults();
    let certificate = create_test_certificate();
    
    let result = validator.validate_certificate(&certificate, None);
    assert!(result.is_ok());
    
    let validation_result = result.unwrap();
    assert!(validation_result.basic_result.is_valid);
    assert!(validation_result.basic_result.errors.is_empty());
}

#[test]
fn test_validate_expired_certificate() {
    let mut validator = CertificateValidator::with_defaults();
    let expired_cert = create_expired_certificate();
    
    let result = validator.validate_certificate(&expired_cert, None);
    assert!(result.is_ok());
    
    let validation_result = result.unwrap();
    // Should be invalid due to expiration
    assert!(!validation_result.basic_result.is_valid);
    assert!(!validation_result.basic_result.errors.is_empty());
}

#[test]
fn test_validate_certificate_chain() {
    let mut validator = CertificateValidator::with_defaults();
    validator.add_trust_store("test".to_string(), create_test_trust_store());
    
    let chain = create_test_chain();
    let result = validator.validate_certificate_chain(&chain, Some("test"));
    
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    assert!(validation_result.basic_result.is_valid);
}

#[test]
fn test_validate_certificate_for_hostname() {
    let mut validator = CertificateValidator::with_defaults();
    let certificate = create_test_certificate();
    
    // Test exact hostname match
    let result = validator.validate_certificate_for_hostname(&certificate, "test.example.com", None);
    assert!(result.is_ok());
    
    let validation_result = result.unwrap();
    assert!(!validation_result.details.name_info.hostname_matching.is_empty());
    
    let hostname_match = &validation_result.details.name_info.hostname_matching[0];
    assert_eq!(hostname_match.target_hostname, "test.example.com");
    assert!(matches!(hostname_match.match_result, 
        HostnameMatchStatus::ExactMatch | HostnameMatchStatus::WildcardMatch));
}

#[test]
fn test_validate_certificate_for_wrong_hostname() {
    let mut validator = CertificateValidator::with_defaults();
    let certificate = create_test_certificate();
    
    let result = validator.validate_certificate_for_hostname(&certificate, "wrong.example.com", None);
    assert!(result.is_ok());
    
    let validation_result = result.unwrap();
    // Should be invalid due to hostname mismatch
    assert!(!validation_result.basic_result.is_valid);
    assert!(validation_result.basic_result.errors.iter()
        .any(|e| e.contains("does not match hostname")));
}

#[test]
fn test_hostname_matching() {
    let validator = CertificateValidator::with_defaults();
    
    // Test exact match
    assert_eq!(
        validator.match_hostname("example.com", "example.com"),
        HostnameMatchStatus::ExactMatch
    );
    
    // Test wildcard match
    assert_eq!(
        validator.match_hostname("*.example.com", "test.example.com"),
        HostnameMatchStatus::WildcardMatch
    );
    
    // Test no match
    assert_eq!(
        validator.match_hostname("example.com", "different.com"),
        HostnameMatchStatus::NoMatch
    );
    
    // Test invalid wildcard
    assert_eq!(
        validator.match_hostname("*.example.com", "sub.test.example.com"),
        HostnameMatchStatus::InvalidWildcard
    );
}

#[test]
fn test_certificate_expiration_functions() {
    let valid_cert = create_test_certificate();
    let expired_cert = create_expired_certificate();
    
    // Test current validity
    assert!(is_certificate_currently_valid(&valid_cert));
    assert!(!is_certificate_currently_valid(&expired_cert));
    
    // Test expiration
    assert!(!is_certificate_expired(&valid_cert));
    assert!(is_certificate_expired(&expired_cert));
    
    // Test not yet valid
    assert!(!is_certificate_not_yet_valid(&valid_cert));
    assert!(!is_certificate_not_yet_valid(&expired_cert));
    
    // Test time until expiration
    let time_until = get_time_until_expiration(&valid_cert);
    assert!(time_until.is_ok());
    assert!(time_until.unwrap() > Duration::from_secs(0));
    
    let expired_time = get_time_until_expiration(&expired_cert);
    assert!(expired_time.is_err());
}

#[test]
fn test_key_strength_analysis() {
    let validator = CertificateValidator::with_defaults();
    
    // Test RSA 2048
    let rsa_2048 = SubjectPublicKeyInfo {
        algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
        public_key: vec![0x30, 0x82, 0x01, 0x22],
        parameters: None,
    };
    
    let analysis = validator.analyze_key_strength(&rsa_2048).unwrap();
    assert_eq!(analysis.key_size_bits, 2048);
    assert_eq!(analysis.security_level, SecurityLevel::Medium);
    
    // Test RSA 1024 (weak)
    let rsa_1024 = SubjectPublicKeyInfo {
        algorithm: PublicKeyAlgorithm::Rsa { key_size: 1024 },
        public_key: vec![0x30, 0x82, 0x01, 0x22],
        parameters: None,
    };
    
    let weak_analysis = validator.analyze_key_strength(&rsa_1024).unwrap();
    assert_eq!(weak_analysis.key_size_bits, 1024);
    assert_eq!(weak_analysis.security_level, SecurityLevel::Low);
    assert!(!weak_analysis.weaknesses.is_empty());
    
    // Test P-256 (strong)
    let p256 = SubjectPublicKeyInfo {
        algorithm: PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 },
        public_key: vec![0x30, 0x59, 0x30, 0x13],
        parameters: None,
    };
    
    let ec_analysis = validator.analyze_key_strength(&p256).unwrap();
    assert_eq!(ec_analysis.key_size_bits, 256);
    assert_eq!(ec_analysis.security_level, SecurityLevel::Medium);
}

#[test]
fn test_signature_verification_status() {
    let validator = CertificateValidator::with_defaults();
    
    // Test RSA certificate
    let rsa_cert = create_test_certificate();
    let status = validator.verify_certificate_signature(&rsa_cert).unwrap();
    assert_eq!(status, SignatureVerificationStatus::Valid);
    
    // Test unsupported algorithm
    let mut unsupported_cert = create_test_certificate();
    unsupported_cert.signature_algorithm = SignatureAlgorithm::Custom {
        oid: "1.2.3.4".to_string(),
        name: "UnsupportedAlgorithm".to_string(),
    };
    
    let unsupported_status = validator.verify_certificate_signature(&unsupported_cert).unwrap();
    assert_eq!(unsupported_status, SignatureVerificationStatus::AlgorithmNotSupported);
}

#[test]
fn test_policy_validation() {
    let validator = CertificateValidator::with_defaults();
    
    // Test basic constraints validation
    let ca_cert = create_ca_certificate();
    let basic_validator = validation::BasicConstraintsPolicyValidator;
    
    let result = basic_validator.validate_policy(&ca_cert, &create_test_validation_context());
    assert!(result.is_ok());
    
    // Test key usage validation
    let key_validator = validation::KeyUsagePolicyValidator;
    let result = key_validator.validate_policy(&ca_cert, &create_test_validation_context());
    assert!(result.is_ok());
}

#[test]
fn test_extension_validation() {
    let validator = CertificateValidator::with_defaults();
    
    // Test known extension
    assert!(validator.is_known_extension("2.5.29.19")); // Basic Constraints
    assert!(validator.is_known_extension("2.5.29.15")); // Key Usage
    assert!(validator.is_known_extension("2.5.29.17")); // Subject Alternative Name
    
    // Test unknown extension
    assert!(!validator.is_known_extension("1.2.3.4.5"));
}

#[test]
fn test_revocation_status_checking() {
    let validator = CertificateValidator::with_defaults();
    let certificate = create_test_certificate();
    let context = create_test_validation_context();
    
    // Test OCSP checker
    let ocsp_checker = validation::OcspRevocationChecker::new();
    let ocsp_result = ocsp_checker.check_revocation(&certificate, &context);
    assert!(ocsp_result.is_ok());
    assert_eq!(ocsp_result.unwrap(), RevocationStatus::Good);
    
    // Test CRL checker
    let crl_checker = validation::CrlRevocationChecker::new();
    let crl_result = crl_checker.check_revocation(&certificate, &context);
    assert!(crl_result.is_ok());
    assert_eq!(crl_result.unwrap(), RevocationStatus::Good);
}

#[test]
fn test_validation_statistics_update() {
    let mut validator = CertificateValidator::with_defaults();
    
    // Create a mock detailed validation result
    let basic_result = ValidationResult {
        is_valid: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        trust_chain: None,
        validated_at: SystemTime::now(),
    };
    
    let performance = ValidationPerformance {
        total_time_ms: 100,
        chain_building_time_ms: 20,
        signature_verification_time_ms: 30,
        revocation_check_time_ms: 25,
        policy_validation_time_ms: 15,
        cache_lookup_time_ms: 5,
        network_time_ms: 25,
    };
    
    let details = create_mock_validation_details();
    
    let result = DetailedValidationResult {
        basic_result,
        details,
        performance,
        recommendations: Vec::new(),
    };
    
    let initial_count = validator.statistics.total_validations;
    validator.update_statistics(&result);
    
    assert_eq!(validator.statistics.total_validations, initial_count + 1);
    assert_eq!(validator.statistics.successful_validations, 1);
    assert!(validator.statistics.avg_validation_time_ms > 0.0);
}

#[test]
fn test_cache_operations() {
    let validator = CertificateValidator::with_defaults();
    let chain = create_test_chain();
    
    // Test cache key calculation
    let cache_key = validator.calculate_cache_key(&chain);
    assert!(!cache_key.is_empty());
    assert!(cache_key.contains(&chain.end_entity.serial_number.to_hex_string()));
    
    // Test cache miss (empty cache)
    let cache_result = validator.check_cache(&chain);
    assert!(cache_result.is_ok());
    assert!(cache_result.unwrap().is_none());
}

#[test]
fn test_recommendation_generation() {
    let validator = CertificateValidator::with_defaults();
    let chain = create_test_chain();
    
    // Create mock details with weak key
    let mut details = create_mock_validation_details();
    details.signature_info.key_strength.security_level = SecurityLevel::Low;
    
    let recommendations = validator.generate_recommendations(&chain, &details);
    
    assert!(!recommendations.is_empty());
    assert!(recommendations.iter().any(|r| 
        r.recommendation_type == RecommendationType::SecurityImprovement));
}

#[test]
fn test_strictness_levels() {
    // Test different validation strictness levels
    assert_eq!(ValidationStrictnessLevel::Permissive, ValidationStrictnessLevel::Permissive);
    assert_eq!(ValidationStrictnessLevel::Standard, ValidationStrictnessLevel::Standard);
    assert_eq!(ValidationStrictnessLevel::Strict, ValidationStrictnessLevel::Strict);
    assert_eq!(ValidationStrictnessLevel::Custom, ValidationStrictnessLevel::Custom);
    
    assert_ne!(ValidationStrictnessLevel::Permissive, ValidationStrictnessLevel::Strict);
}

#[test]
fn test_violation_severity_levels() {
    // Test violation severity comparison
    assert_eq!(ViolationSeverity::Critical, ViolationSeverity::Critical);
    assert_eq!(ViolationSeverity::High, ViolationSeverity::High);
    assert_eq!(ViolationSeverity::Medium, ViolationSeverity::Medium);
    assert_eq!(ViolationSeverity::Low, ViolationSeverity::Low);
    assert_eq!(ViolationSeverity::Info, ViolationSeverity::Info);
    
    assert_ne!(ViolationSeverity::Critical, ViolationSeverity::Low);
}

#[test]
fn test_name_validation_status() {
    // Test name validation status types
    assert_eq!(NameValidationStatus::Valid, NameValidationStatus::Valid);
    assert_eq!(NameValidationStatus::Invalid, NameValidationStatus::Invalid);
    assert_eq!(NameValidationStatus::Malformed, NameValidationStatus::Malformed);
    assert_eq!(NameValidationStatus::ConstraintViolation, NameValidationStatus::ConstraintViolation);
    
    assert_ne!(NameValidationStatus::Valid, NameValidationStatus::Invalid);
}

#[test]
fn test_certificate_cache_cleanup() {
    let mut cache = CertificateCache::new();
    
    // Add some mock entries
    let now = SystemTime::now();
    let expired_result = CachedValidationResult {
        result: create_mock_detailed_validation_result(),
        cached_at: now - Duration::from_secs(7200), // 2 hours ago
        ttl: Duration::from_secs(3600), // 1 hour TTL (expired)
    };
    
    let valid_result = CachedValidationResult {
        result: create_mock_detailed_validation_result(),
        cached_at: now - Duration::from_secs(1800), // 30 minutes ago
        ttl: Duration::from_secs(3600), // 1 hour TTL (still valid)
    };
    
    cache.results.insert("expired".to_string(), expired_result);
    cache.results.insert("valid".to_string(), valid_result);
    
    assert_eq!(cache.results.len(), 2);
    
    cache.cleanup_expired();
    
    // Only valid entry should remain
    assert_eq!(cache.results.len(), 1);
    assert!(cache.results.contains_key("valid"));
    assert!(!cache.results.contains_key("expired"));
    assert_eq!(cache.statistics.evictions, 1);
}

#[test]
fn test_validation_config_update() {
    let mut validator = CertificateValidator::with_defaults();
    
    let mut new_config = ValidationConfig::default();
    new_config.max_chain_length = 5;
    new_config.check_revocation = false;
    new_config.min_rsa_key_size = 4096;
    
    validator.update_config(new_config.clone());
    
    assert_eq!(validator.config.max_chain_length, 5);
    assert!(!validator.config.check_revocation);
    assert_eq!(validator.config.min_rsa_key_size, 4096);
}

#[test]
fn test_trust_store_operations() {
    let mut validator = CertificateValidator::with_defaults();
    
    let trust_store = create_test_trust_store();
    validator.add_trust_store("test_store".to_string(), trust_store);
    
    assert!(validator.get_trust_store("test_store").is_some());
    assert!(validator.get_trust_store("nonexistent").is_none());
    
    let retrieved_store = validator.get_trust_store("test_store").unwrap();
    assert_eq!(retrieved_store.name, "test_store");
    assert!(!retrieved_store.root_certificates.is_empty());
}

// Helper functions for creating test data

fn create_test_validation_context() -> ValidationContext<'static> {
    let trust_store = Box::leak(Box::new(create_test_trust_store()));
    let policy = Box::leak(Box::new(ValidationPolicy::default()));
    ValidationContext::new(trust_store, policy)
}

fn create_mock_validation_details() -> ValidationDetails {
    ValidationDetails {
        chain_info: ChainValidationInfo {
            chain_length: 2,
            trust_path_found: true,
            trust_anchor: Some(DistinguishedName::from_common_name("Test Root CA")),
            path_building_time_ms: 20,
            chain_errors: Vec::new(),
        },
        signature_info: SignatureValidationInfo {
            algorithm: SignatureAlgorithm::RsaWithSha256,
            verification_status: SignatureVerificationStatus::Valid,
            key_strength: KeyStrengthAnalysis {
                algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
                key_size_bits: 2048,
                security_level: SecurityLevel::Medium,
                weaknesses: Vec::new(),
                recommendations: Vec::new(),
            },
            signature_time: None,
            verification_time_ms: 30,
        },
        revocation_info: RevocationValidationInfo {
            status: RevocationStatus::Good,
            check_method: RevocationCheckMethod::OCSP,
            responder_info: Some("ocsp.example.com".to_string()),
            check_timestamp: SystemTime::now(),
            check_duration_ms: 25,
            next_update: Some(SystemTime::now() + Duration::from_secs(86400)),
        },
        policy_info: PolicyValidationInfo {
            policies_checked: Vec::new(),
            violations: Vec::new(),
            policy_mappings: Vec::new(),
            constraints_applied: Vec::new(),
        },
        extensions_info: ExtensionsValidationInfo {
            critical_extensions: vec!["2.5.29.19".to_string()],
            unknown_critical_extensions: Vec::new(),
            extension_errors: Vec::new(),
            extension_recommendations: Vec::new(),
        },
        name_info: NameValidationInfo {
            subject_validation: NameValidationDetails {
                distinguished_name: DistinguishedName::from_common_name("test.example.com"),
                validation_status: NameValidationStatus::Valid,
                errors: Vec::new(),
            },
            san_validation: Vec::new(),
            name_constraints: Vec::new(),
            hostname_matching: Vec::new(),
        },
    }
}

fn create_mock_detailed_validation_result() -> DetailedValidationResult {
    DetailedValidationResult {
        basic_result: ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            trust_chain: None,
            validated_at: SystemTime::now(),
        },
        details: create_mock_validation_details(),
        performance: ValidationPerformance {
            total_time_ms: 100,
            chain_building_time_ms: 20,
            signature_verification_time_ms: 30,
            revocation_check_time_ms: 25,
            policy_validation_time_ms: 15,
            cache_lookup_time_ms: 5,
            network_time_ms: 25,
        },
        recommendations: Vec::new(),
    }
}
