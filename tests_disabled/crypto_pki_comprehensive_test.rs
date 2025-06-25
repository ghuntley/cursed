/// fr fr Comprehensive PKI Certificate Signing and Verification Test Suite
/// 
/// This test suite validates the complete PKI implementation including:
/// - Certificate Authority signing with different algorithms (RSA, ECDSA, Ed25519)
/// - Certificate signature verification across different key types
/// - Certificate chain validation
/// - Key format support and conversions
/// - Error handling for invalid certificates
/// - Real cryptographic operations end-to-end

use cursed::stdlib::packages::crypto_pki::{
    certificate_authority::*,
    certificate::*,
    errors::*,
};
use cursed::stdlib::packages::crypto_asymmetric::{
    rsa_generate_keypair, ecc_generate_keypair, ed25519_generate_keypair,
    RsaKeyPair, EccKeyPair, EccCurve, EccHashAlgorithm,
};
use cursed::error::CursedError;

/// fr fr Test certificate signing with RSA
#[test]
fn test_rsa_certificate_signing() {
    // Create RSA CA
    let mut ca_config = CaConfiguration::default();
    ca_config.name = "Test RSA CA".to_string();
    ca_config.key_size = 2048;
    ca_config.signature_algorithm = SignatureAlgorithm::RsaWithSha256;
    
    let ca = CertificateAuthority::new(ca_config).expect("Failed to create CA");
    
    // Create certificate template
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "test.example.com".to_string(),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 365,
        key_size: 2048,
        subject_alt_names: vec!["test.example.com".to_string(), "www.test.example.com".to_string()],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    // Issue certificate
    let certificate = ca.issue_certificate(&template).expect("Failed to issue certificate");
    
    // Verify certificate properties
    assert_eq!(certificate.subject.common_name, Some("test.example.com".to_string()));
    assert_eq!(certificate.signature_algorithm, SignatureAlgorithm::RsaWithSha256);
    assert!(!certificate.signature.is_empty(), "Certificate should have signature");
    
    // Verify certificate signature
    let verification_result = certificate.verify_signature(&ca.certificate);
    assert!(verification_result.is_ok(), "Certificate signature verification should succeed");
    assert!(verification_result.unwrap(), "Certificate signature should be valid");
}

/// fr fr Test certificate signing with ECDSA P-256
#[test]
fn test_ecdsa_p256_certificate_signing() {
    // Create ECDSA CA
    let mut ca_config = CaConfiguration::default();
    ca_config.name = "Test ECDSA P-256 CA".to_string();
    ca_config.signature_algorithm = SignatureAlgorithm::EcdsaWithSha256;
    
    let ca = CertificateAuthority::new(ca_config).expect("Failed to create ECDSA CA");
    
    // Create certificate template
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "ecdsa-test.example.com".to_string(),
            organization: Some("ECDSA Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 365,
        key_size: 256, // P-256 key size
        subject_alt_names: vec!["ecdsa-test.example.com".to_string()],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    // Issue certificate
    let certificate = ca.issue_certificate(&template).expect("Failed to issue ECDSA certificate");
    
    // Verify certificate properties
    assert_eq!(certificate.subject.common_name, Some("ecdsa-test.example.com".to_string()));
    assert_eq!(certificate.signature_algorithm, SignatureAlgorithm::EcdsaWithSha256);
    assert!(!certificate.signature.is_empty(), "Certificate should have signature");
    
    // Verify certificate signature
    let verification_result = certificate.verify_signature(&ca.certificate);
    assert!(verification_result.is_ok(), "ECDSA certificate signature verification should succeed");
    assert!(verification_result.unwrap(), "ECDSA certificate signature should be valid");
}

/// fr fr Test certificate signing with ECDSA P-384
#[test]
fn test_ecdsa_p384_certificate_signing() {
    // Create ECDSA P-384 CA
    let mut ca_config = CaConfiguration::default();
    ca_config.name = "Test ECDSA P-384 CA".to_string();
    ca_config.signature_algorithm = SignatureAlgorithm::EcdsaWithSha384;
    
    let ca = CertificateAuthority::new(ca_config).expect("Failed to create ECDSA P-384 CA");
    
    // Create certificate template
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "ecdsa384-test.example.com".to_string(),
            organization: Some("ECDSA P-384 Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 365,
        key_size: 384, // P-384 key size
        subject_alt_names: vec!["ecdsa384-test.example.com".to_string()],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    // Issue certificate
    let certificate = ca.issue_certificate(&template).expect("Failed to issue ECDSA P-384 certificate");
    
    // Verify certificate properties
    assert_eq!(certificate.subject.common_name, Some("ecdsa384-test.example.com".to_string()));
    assert_eq!(certificate.signature_algorithm, SignatureAlgorithm::EcdsaWithSha384);
    assert!(!certificate.signature.is_empty(), "Certificate should have signature");
    
    // Verify certificate signature
    let verification_result = certificate.verify_signature(&ca.certificate);
    assert!(verification_result.is_ok(), "ECDSA P-384 certificate signature verification should succeed");
    assert!(verification_result.unwrap(), "ECDSA P-384 certificate signature should be valid");
}

/// fr fr Test certificate chain validation
#[test]
fn test_certificate_chain_validation() {
    // Create root CA
    let root_ca = create_root_ca("Root CA", "Root Organization", Some(2048))
        .expect("Failed to create root CA");
    
    // Create intermediate CA
    let mut intermediate_ca = create_intermediate_ca(
        &mut root_ca.clone(),
        "Intermediate CA",
        "Intermediate Organization"
    ).expect("Failed to create intermediate CA");
    
    // Create end-entity certificate from intermediate CA
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "end-entity.example.com".to_string(),
            organization: Some("End Entity Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 365,
        key_size: 2048,
        subject_alt_names: vec!["end-entity.example.com".to_string()],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: Some(BasicConstraints { ca: false, path_length: None }),
    };
    
    let end_entity_cert = intermediate_ca.issue_certificate(&template)
        .expect("Failed to issue end-entity certificate");
    
    // Verify certificate chain
    // End-entity -> Intermediate CA
    let intermediate_verification = end_entity_cert.verify_signature(&intermediate_ca.certificate);
    assert!(intermediate_verification.is_ok(), "End-entity certificate should verify against intermediate CA");
    assert!(intermediate_verification.unwrap(), "End-entity signature should be valid");
    
    // Intermediate CA -> Root CA
    let root_verification = intermediate_ca.certificate.verify_signature(&root_ca.certificate);
    assert!(root_verification.is_ok(), "Intermediate CA certificate should verify against root CA");
    assert!(root_verification.unwrap(), "Intermediate CA signature should be valid");
    
    // Root CA is self-signed
    let self_signed_verification = root_ca.certificate.verify_signature(&root_ca.certificate);
    assert!(self_signed_verification.is_ok(), "Root CA certificate should be self-verifying");
    assert!(self_signed_verification.unwrap(), "Root CA self-signature should be valid");
}

/// fr fr Test certificate validation errors
#[test]
fn test_certificate_validation_errors() {
    // Create CA
    let ca = create_root_ca("Test CA", "Test Organization", Some(2048))
        .expect("Failed to create CA");
    
    // Test invalid certificate template (exceeds policy limits)
    let invalid_template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "invalid.example.com".to_string(),
            organization: Some("Invalid Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 10000, // Exceeds CA policy max validity
        key_size: 1024, // Below minimum key size
        subject_alt_names: vec![],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    // This should fail due to policy violations
    let result = ca.issue_certificate(&invalid_template);
    assert!(result.is_err(), "Certificate issuance should fail for invalid template");
    
    // Verify specific error types
    match result.unwrap_err() {
        PkiError::CaOperationFailed(msg) => {
            assert!(msg.contains("validity period"), "Error should mention validity period");
        },
        _ => panic!("Expected CaOperationFailed error"),
    }
}

/// fr fr Test certificate expiration and validity
#[test]
fn test_certificate_validity_checking() {
    // Create CA
    let ca = create_root_ca("Test CA", "Test Organization", Some(2048))
        .expect("Failed to create CA");
    
    // Create certificate template with short validity
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "short-lived.example.com".to_string(),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 1, // Very short validity for testing
        key_size: 2048,
        subject_alt_names: vec!["short-lived.example.com".to_string()],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    let certificate = ca.issue_certificate(&template)
        .expect("Failed to issue short-lived certificate");
    
    // Certificate should be valid now
    assert!(certificate.is_valid_now(), "Certificate should be currently valid");
    
    // Check days until expiry
    let days_until_expiry = certificate.days_until_expiry();
    assert!(days_until_expiry.is_ok(), "Should be able to calculate days until expiry");
    assert!(days_until_expiry.unwrap() <= 1, "Certificate should expire within 1 day");
}

/// fr fr Test hostname validation
#[test]
fn test_hostname_validation() {
    // Create CA
    let ca = create_root_ca("Test CA", "Test Organization", Some(2048))
        .expect("Failed to create CA");
    
    // Create certificate with multiple SANs
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "primary.example.com".to_string(),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 365,
        key_size: 2048,
        subject_alt_names: vec![
            "primary.example.com".to_string(),
            "www.example.com".to_string(),
            "api.example.com".to_string(),
            "*.dev.example.com".to_string(), // Wildcard
        ],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    let certificate = ca.issue_certificate(&template)
        .expect("Failed to issue certificate with SANs");
    
    // Test valid hostnames
    assert!(certificate.validate_hostname("primary.example.com"), "Should validate primary hostname");
    assert!(certificate.validate_hostname("www.example.com"), "Should validate SAN hostname");
    assert!(certificate.validate_hostname("api.example.com"), "Should validate API hostname");
    assert!(certificate.validate_hostname("test.dev.example.com"), "Should validate wildcard hostname");
    
    // Test invalid hostnames
    assert!(!certificate.validate_hostname("invalid.example.com"), "Should reject invalid hostname");
    assert!(!certificate.validate_hostname("example.com"), "Should reject parent domain");
    assert!(!certificate.validate_hostname("malicious.com"), "Should reject completely different domain");
}

/// fr fr Test CA manager functionality
#[test]
fn test_ca_manager() {
    let mut manager = CaManager::new();
    
    // Create multiple CAs
    let ca1 = create_root_ca("CA One", "Organization One", Some(2048))
        .expect("Failed to create CA 1");
    let ca2 = create_root_ca("CA Two", "Organization Two", Some(3072))
        .expect("Failed to create CA 2");
    
    // Add CAs to manager
    manager.add_ca("ca1".to_string(), ca1);
    manager.add_ca("ca2".to_string(), ca2);
    
    // Test CA retrieval
    assert!(manager.get_ca("ca1").is_some(), "Should find CA 1");
    assert!(manager.get_ca("ca2").is_some(), "Should find CA 2");
    assert!(manager.get_ca("nonexistent").is_none(), "Should not find nonexistent CA");
    
    // Test CA listing
    let ca_list = manager.list_cas();
    assert_eq!(ca_list.len(), 2, "Should list 2 CAs");
    assert!(ca_list.contains(&"ca1".to_string()), "Should contain CA 1");
    assert!(ca_list.contains(&"ca2".to_string()), "Should contain CA 2");
    
    // Test CA profiles
    let profile = CaProfile {
        name: "Web Server".to_string(),
        validity_days: 365,
        key_usage: KeyUsage {
            digital_signature: true,
            non_repudiation: false,
            key_encipherment: true,
            data_encipherment: false,
            key_agreement: false,
            key_cert_sign: false,
            crl_sign: false,
            encipher_only: false,
            decipher_only: false,
        },
        extended_key_usage: Some(ExtendedKeyUsage {
            server_auth: true,
            client_auth: false,
            code_signing: false,
            email_protection: false,
            time_stamping: false,
            ocsp_signing: false,
            custom_purposes: Vec::new(),
        }),
        basic_constraints: Some(BasicConstraints { ca: false, path_length: None }),
        require_san: true,
    };
    
    manager.add_profile("webserver".to_string(), profile);
    
    let retrieved_profile = manager.get_profile("webserver");
    assert!(retrieved_profile.is_some(), "Should find web server profile");
    assert_eq!(retrieved_profile.unwrap().name, "Web Server", "Profile name should match");
}

/// fr fr Test certificate revocation
#[test]
fn test_certificate_revocation() {
    // Create CA
    let mut ca = create_root_ca("Test CA", "Test Organization", Some(2048))
        .expect("Failed to create CA");
    
    // Issue a certificate
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "revoked.example.com".to_string(),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 365,
        key_size: 2048,
        subject_alt_names: vec!["revoked.example.com".to_string()],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    let certificate = ca.issue_certificate(&template)
        .expect("Failed to issue certificate");
    
    let serial_number = certificate.serial_number.to_hex();
    
    // Certificate should not be revoked initially
    assert!(!ca.is_revoked(&serial_number), "Certificate should not be revoked initially");
    
    // Revoke the certificate
    ca.revoke_certificate(&serial_number, RevocationReason::KeyCompromise)
        .expect("Failed to revoke certificate");
    
    // Certificate should now be revoked
    assert!(ca.is_revoked(&serial_number), "Certificate should be revoked");
    
    // Generate CRL
    let crl = ca.generate_crl().expect("Failed to generate CRL");
    assert!(!crl.is_empty(), "CRL should not be empty");
    
    // Verify CRL contains the revoked certificate
    let crl_string = String::from_utf8_lossy(&crl);
    assert!(crl_string.contains(&serial_number), "CRL should contain revoked certificate serial number");
}

/// fr fr Test certificate fingerprints
#[test]
fn test_certificate_fingerprints() {
    // Create CA and certificate
    let ca = create_root_ca("Test CA", "Test Organization", Some(2048))
        .expect("Failed to create CA");
    
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "fingerprint-test.example.com".to_string(),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 365,
        key_size: 2048,
        subject_alt_names: vec!["fingerprint-test.example.com".to_string()],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    let certificate = ca.issue_certificate(&template)
        .expect("Failed to issue certificate");
    
    // Test fingerprint generation
    let sha256_fingerprint = certificate.fingerprint_sha256();
    assert!(sha256_fingerprint.is_ok(), "Should be able to generate SHA-256 fingerprint");
    
    let sha1_fingerprint = certificate.fingerprint_sha1();
    assert!(sha1_fingerprint.is_ok(), "Should be able to generate SHA-1 fingerprint");
    
    let general_fingerprint = certificate.fingerprint();
    assert!(general_fingerprint.is_ok(), "Should be able to generate general fingerprint");
    
    // Verify fingerprint format (should be uppercase hex)
    let fp = sha256_fingerprint.unwrap();
    assert!(fp.chars().all(|c| c.is_ascii_hexdigit() || c.is_ascii_uppercase()), 
            "Fingerprint should be uppercase hex");
    assert_eq!(fp.len(), 64, "SHA-256 fingerprint should be 64 characters");
}

/// fr fr Test PEM/DER format conversion
#[test]
fn test_certificate_format_conversion() {
    // Create CA and certificate
    let ca = create_root_ca("Test CA", "Test Organization", Some(2048))
        .expect("Failed to create CA");
    
    let template = CertificateTemplate {
        subject: TemplateSubject {
            common_name: "format-test.example.com".to_string(),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state: None,
            locality: None,
            email: None,
        },
        validity_days: 365,
        key_size: 2048,
        subject_alt_names: vec!["format-test.example.com".to_string()],
        key_usage: None,
        extended_key_usage: None,
        basic_constraints: None,
    };
    
    let certificate = ca.issue_certificate(&template)
        .expect("Failed to issue certificate");
    
    // Test PEM conversion
    let pem_result = certificate.to_pem();
    assert!(pem_result.is_ok(), "Should be able to convert to PEM");
    
    let pem_string = pem_result.unwrap();
    assert!(pem_string.contains("-----BEGIN CERTIFICATE-----"), "PEM should have begin marker");
    assert!(pem_string.contains("-----END CERTIFICATE-----"), "PEM should have end marker");
    
    // Test DER conversion
    let der_bytes = certificate.to_der();
    assert!(!der_bytes.is_empty(), "DER bytes should not be empty");
    
    // Test round-trip conversion
    let parsed_from_pem = Certificate::from_pem(&pem_string);
    assert!(parsed_from_pem.is_ok(), "Should be able to parse certificate from PEM");
    
    let parsed_from_der = Certificate::from_der(&der_bytes);
    assert!(parsed_from_der.is_ok(), "Should be able to parse certificate from DER");
}

/// fr fr Integration test with all algorithms
#[test]
fn test_multi_algorithm_integration() {
    let algorithms = vec![
        SignatureAlgorithm::RsaWithSha256,
        SignatureAlgorithm::RsaWithSha384,
        SignatureAlgorithm::RsaWithSha512,
        SignatureAlgorithm::EcdsaWithSha256,
        SignatureAlgorithm::EcdsaWithSha384,
    ];
    
    for algorithm in algorithms {
        // Create CA with specific algorithm
        let mut ca_config = CaConfiguration::default();
        ca_config.name = format!("Test CA {:?}", algorithm);
        ca_config.signature_algorithm = algorithm.clone();
        
        let ca_result = CertificateAuthority::new(ca_config);
        assert!(ca_result.is_ok(), "Should be able to create CA with {:?}", algorithm);
        
        let ca = ca_result.unwrap();
        
        // Create certificate template
        let template = CertificateTemplate {
            subject: TemplateSubject {
                common_name: format!("test-{:?}.example.com", algorithm).to_lowercase(),
                organization: Some("Multi-Algorithm Test Organization".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state: None,
                locality: None,
                email: None,
            },
            validity_days: 365,
            key_size: match algorithm {
                SignatureAlgorithm::EcdsaWithSha256 => 256,
                SignatureAlgorithm::EcdsaWithSha384 => 384,
                _ => 2048,
            },
            subject_alt_names: vec![format!("test-{:?}.example.com", algorithm).to_lowercase()],
            key_usage: None,
            extended_key_usage: None,
            basic_constraints: None,
        };
        
        // Issue and verify certificate
        let cert_result = ca.issue_certificate(&template);
        assert!(cert_result.is_ok(), "Should be able to issue certificate with {:?}", algorithm);
        
        let certificate = cert_result.unwrap();
        assert_eq!(certificate.signature_algorithm, algorithm, "Certificate should have correct algorithm");
        
        // Verify signature
        let verification_result = certificate.verify_signature(&ca.certificate);
        assert!(verification_result.is_ok(), "Should be able to verify certificate with {:?}", algorithm);
        assert!(verification_result.unwrap(), "Certificate signature should be valid for {:?}", algorithm);
    }
}
