/// Test suite for certificate validation functionality
/// Validates the enhanced certificate handling operations

use cursed::stdlib::crypto::certificates::*;
use cursed::stdlib::value::Value;
use cursed::error::CursedError;
use std::collections::HashMap;

#[test]
fn test_certificate_processor_creation() {
    let processor = CertificateProcessor::new();
    assert_eq!(processor.config.check_expiration, true);
    assert_eq!(processor.config.signature_verification, true);
    assert_eq!(processor.config.max_chain_length, 10);
}

#[test]
fn test_distinguished_name_creation() {
    let dn = DistinguishedName::new()
        .with_common_name("test.example.com")
        .with_organization("Test Corp")
        .with_country("US");
    
    assert_eq!(dn.common_name.as_ref().unwrap(), "test.example.com");
    assert_eq!(dn.organization.as_ref().unwrap(), "Test Corp");
    assert_eq!(dn.country.as_ref().unwrap(), "US");
    
    let dn_str = dn.to_string();
    assert!(dn_str.contains("CN=test.example.com"));
    assert!(dn_str.contains("O=Test Corp"));
    assert!(dn_str.contains("C=US"));
}

#[test]
fn test_object_identifier_parsing() {
    let oid = ObjectIdentifier::from_string("2.5.29.15").unwrap();
    assert_eq!(oid.components, vec![2, 5, 29, 15]);
    assert_eq!(oid.to_string(), "2.5.29.15");
    
    let invalid_oid = ObjectIdentifier::from_string("invalid.oid");
    assert!(invalid_oid.is_err());
}

#[test]
fn test_certificate_config() {
    let config = CertificateConfig::default();
    assert!(config.check_expiration);
    assert!(config.check_hostname);
    assert!(!config.check_revocation);
    assert!(!config.allow_self_signed);
    assert_eq!(config.max_chain_length, 10);
    
    let custom_config = CertificateConfig {
        check_expiration: false,
        allow_self_signed: true,
        max_chain_length: 5,
        ..Default::default()
    };
    
    let processor = CertificateProcessor::with_config(custom_config);
    assert!(!processor.config.check_expiration);
    assert!(processor.config.allow_self_signed);
    assert_eq!(processor.config.max_chain_length, 5);
}

#[test]
fn test_certificate_errors() {
    let error = CertificateError::Expired;
    assert_eq!(error.to_string(), "Certificate has expired");
    
    let error = CertificateError::HostnameMismatch("example.com".to_string());
    assert_eq!(error.to_string(), "Certificate hostname mismatch, expected: example.com");
    
    let error = CertificateError::InvalidSignature;
    assert_eq!(error.to_string(), "Invalid certificate signature");
    
    let error = CertificateError::UntrustedIssuer;
    assert_eq!(error.to_string(), "Certificate issued by untrusted authority");
}

#[test]
fn test_signature_algorithms() {
    assert_eq!(SignatureAlgorithm::Sha256WithRsaEncryption as u8, 0);
    assert_eq!(SignatureAlgorithm::EcdsaWithSha256 as u8, 3);
    assert_eq!(SignatureAlgorithm::Ed25519 as u8, 6);
}

#[test]
fn test_public_key_algorithms() {
    assert_eq!(PublicKeyAlgorithm::RsaEncryption as u8, 0);
    assert_eq!(PublicKeyAlgorithm::EcPublicKey as u8, 1);
    assert_eq!(PublicKeyAlgorithm::Ed25519 as u8, 2);
    assert_eq!(PublicKeyAlgorithm::X25519 as u8, 3);
}

#[test]
fn test_encoding_formats() {
    assert_eq!(EncodingFormat::Der as u8, 0);
    assert_eq!(EncodingFormat::Pem as u8, 1);
}

#[test]
fn test_base64_encoding_decoding() {
    let processor = CertificateProcessor::new();
    let data = b"hello world test data";
    let encoded = processor.base64_encode(data);
    let decoded = processor.base64_decode(&encoded).unwrap();
    assert_eq!(decoded, data);
    
    // Test with empty data
    let empty_encoded = processor.base64_encode(&[]);
    let empty_decoded = processor.base64_decode(&empty_encoded).unwrap();
    assert_eq!(empty_decoded, &[]);
}

#[test]
fn test_wildcard_hostname_matching() {
    let processor = CertificateProcessor::new();
    
    // Valid wildcard matches
    assert!(processor.wildcard_match("*.example.com", "www.example.com"));
    assert!(processor.wildcard_match("*.example.com", "api.example.com"));
    
    // Invalid wildcard matches
    assert!(!processor.wildcard_match("*.example.com", "sub.www.example.com"));
    assert!(!processor.wildcard_match("*.example.com", "different.com"));
    
    // Exact matches
    assert!(processor.wildcard_match("example.com", "example.com"));
    assert!(!processor.wildcard_match("example.com", "www.example.com"));
}

#[test]
fn test_der_length_parsing() {
    let processor = CertificateProcessor::new();
    
    // Short form length
    assert_eq!(processor.parse_der_length(&[0x05]).unwrap(), 5);
    assert_eq!(processor.parse_der_length(&[0x7F]).unwrap(), 127);
    
    // Long form length
    assert_eq!(processor.parse_der_length(&[0x81, 0xFF]).unwrap(), 255);
    assert_eq!(processor.parse_der_length(&[0x82, 0x01, 0x00]).unwrap(), 256);
    
    // Error cases
    assert!(processor.parse_der_length(&[]).is_err());
    assert!(processor.parse_der_length(&[0x80]).is_err()); // Indefinite length not allowed
}

#[test]
fn test_der_length_bytes_calculation() {
    let processor = CertificateProcessor::new();
    
    // Short form
    assert_eq!(processor.der_length_bytes(&[0x05]), 1);
    assert_eq!(processor.der_length_bytes(&[0x7F]), 1);
    
    // Long form
    assert_eq!(processor.der_length_bytes(&[0x81, 0xFF]), 2);
    assert_eq!(processor.der_length_bytes(&[0x82, 0x01, 0x00]), 3);
    
    // Empty data
    assert_eq!(processor.der_length_bytes(&[]), 0);
}

#[test]
fn test_hex_utility_functions() {
    // Test hex encoding
    let data = b"test data";
    let encoded = hex::encode(data);
    assert_eq!(encoded, "746573742064617461");
    
    // Test hex decoding
    let decoded = hex::decode("746573742064617461").unwrap();
    assert_eq!(decoded, b"test data");
    
    // Test invalid hex
    assert!(hex::decode("invalid_hex").is_err());
    assert!(hex::decode("abc").is_err()); // Odd length
}

#[test]
fn test_subject_alt_name_types() {
    let dns_name = SubjectAltName::DnsName("example.com".to_string());
    let ip_address = SubjectAltName::IpAddress("192.168.1.1".to_string());
    let email = SubjectAltName::Email("test@example.com".to_string());
    
    match dns_name {
        SubjectAltName::DnsName(name) => assert_eq!(name, "example.com"),
        _ => panic!("Expected DNS name"),
    }
    
    match ip_address {
        SubjectAltName::IpAddress(ip) => assert_eq!(ip, "192.168.1.1"),
        _ => panic!("Expected IP address"),
    }
    
    match email {
        SubjectAltName::Email(email_addr) => assert_eq!(email_addr, "test@example.com"),
        _ => panic!("Expected email"),
    }
}

#[test]
fn test_certificate_fingerprint_calculation() {
    let processor = CertificateProcessor::new();
    
    // Create a dummy certificate for testing
    let cert = X509Certificate {
        version: 3,
        serial_number: vec![0x01, 0x02, 0x03],
        signature_algorithm: SignatureAlgorithm::Sha256WithRsaEncryption,
        issuer: DistinguishedName::new().with_common_name("Test CA"),
        validity: Validity {
            not_before: std::time::SystemTime::now(),
            not_after: std::time::SystemTime::now() + std::time::Duration::from_secs(365 * 24 * 3600),
        },
        subject: DistinguishedName::new().with_common_name("test.example.com"),
        public_key: PublicKeyInfo {
            algorithm: PublicKeyAlgorithm::RsaEncryption,
            key_data: vec![0x30; 256],
            parameters: None,
        },
        extensions: vec![],
        signature: vec![0x42; 256],
        raw_der: vec![0x30, 0x82, 0x01, 0x00], // Dummy DER data
    };
    
    // Test SHA-256 fingerprint
    let sha256_fp = processor.get_fingerprint_with_algorithm(&cert, "sha256").unwrap();
    assert_eq!(sha256_fp.len(), 32); // SHA-256 is 32 bytes
    
    // Test SHA-1 fingerprint
    let sha1_fp = processor.get_fingerprint_with_algorithm(&cert, "sha1").unwrap();
    assert_eq!(sha1_fp.len(), 20); // SHA-1 is 20 bytes
    
    // Test SHA-512 fingerprint
    let sha512_fp = processor.get_fingerprint_with_algorithm(&cert, "sha512").unwrap();
    assert_eq!(sha512_fp.len(), 64); // SHA-512 is 64 bytes
    
    // Test unsupported algorithm
    assert!(processor.get_fingerprint_with_algorithm(&cert, "md5").is_err());
}

#[test]
fn test_system_cert_paths() {
    let processor = CertificateProcessor::new();
    let paths = processor.get_system_cert_paths();
    
    // Should have at least some paths defined
    assert!(!paths.is_empty());
    
    // Should include common Linux paths
    assert!(paths.iter().any(|p| p.to_string_lossy().contains("/etc/ssl")));
}

#[test]
fn test_certificate_metadata_extraction() {
    let processor = CertificateProcessor::new();
    
    // Create a test certificate
    let cert = X509Certificate {
        version: 3,
        serial_number: vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF],
        signature_algorithm: SignatureAlgorithm::Sha256WithRsaEncryption,
        issuer: DistinguishedName::new()
            .with_common_name("Test Root CA")
            .with_organization("Test Org")
            .with_country("US"),
        validity: Validity {
            not_before: std::time::SystemTime::now(),
            not_after: std::time::SystemTime::now() + std::time::Duration::from_secs(365 * 24 * 3600),
        },
        subject: DistinguishedName::new()
            .with_common_name("test.example.com")
            .with_organization("Example Corp")
            .with_country("US"),
        public_key: PublicKeyInfo {
            algorithm: PublicKeyAlgorithm::RsaEncryption,
            key_data: vec![0x30; 2048/8], // 2048-bit RSA key
            parameters: None,
        },
        extensions: vec![],
        signature: vec![0x42; 256],
        raw_der: vec![0x30, 0x82, 0x02, 0x00],
    };
    
    // Test serial number extraction
    let serial = processor.get_serial_number(&cert);
    assert_eq!(serial, vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]);
    
    // Test self-signed detection
    let self_signed_cert = X509Certificate {
        issuer: cert.subject.clone(),
        subject: cert.subject.clone(),
        ..cert.clone()
    };
    assert!(processor.is_self_signed(&self_signed_cert));
    assert!(!processor.is_self_signed(&cert));
    
    // Test validity period extraction
    let (not_before, not_after) = processor.get_validity_period(&cert);
    assert!(not_after > not_before);
    
    // Test public key extraction
    let pk_info = processor.extract_public_key(&cert).unwrap();
    assert_eq!(pk_info.algorithm, PublicKeyAlgorithm::RsaEncryption);
    assert_eq!(pk_info.key_data.len(), 256); // 2048 bits / 8 = 256 bytes
}

#[test] 
fn test_api_function_parse_certificate_pem() {
    // Test PEM parsing with valid base64 data
    let pem_data = "-----BEGIN CERTIFICATE-----\nTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUF\n-----END CERTIFICATE-----";
    let result = parse_certificate_pem(vec![Value::String(pem_data.to_string())]);
    
    // Should succeed with proper structure (even with dummy data)
    assert!(result.is_ok());
}

#[test]
fn test_api_function_error_handling() {
    // Test empty args
    let result = parse_certificate_pem(vec![]);
    assert!(result.is_err());
    
    // Test wrong type
    let result = parse_certificate_pem(vec![Value::Number(42.0)]);
    assert!(result.is_err());
    
    // Test empty fingerprint args
    let result = get_certificate_fingerprint(vec![]);
    assert!(result.is_err());
}

#[test]
fn test_certificate_chain_structure() {
    let leaf_cert = X509Certificate {
        version: 3,
        serial_number: vec![0x01],
        signature_algorithm: SignatureAlgorithm::Sha256WithRsaEncryption,
        issuer: DistinguishedName::new().with_common_name("Intermediate CA"),
        subject: DistinguishedName::new().with_common_name("leaf.example.com"),
        validity: Validity {
            not_before: std::time::SystemTime::now(),
            not_after: std::time::SystemTime::now() + std::time::Duration::from_secs(365 * 24 * 3600),
        },
        public_key: PublicKeyInfo {
            algorithm: PublicKeyAlgorithm::RsaEncryption,
            key_data: vec![0x30; 256],
            parameters: None,
        },
        extensions: vec![],
        signature: vec![0x42; 256],
        raw_der: vec![0x30, 0x82],
    };
    
    let intermediate_cert = X509Certificate {
        subject: DistinguishedName::new().with_common_name("Intermediate CA"),
        issuer: DistinguishedName::new().with_common_name("Root CA"),
        ..leaf_cert.clone()
    };
    
    let root_cert = X509Certificate {
        subject: DistinguishedName::new().with_common_name("Root CA"),
        issuer: DistinguishedName::new().with_common_name("Root CA"), // Self-signed
        ..leaf_cert.clone()
    };
    
    let chain = CertificateChain {
        certificates: vec![leaf_cert, intermediate_cert, root_cert.clone()],
        trusted_roots: vec![root_cert],
    };
    
    assert_eq!(chain.certificates.len(), 3);
    assert_eq!(chain.trusted_roots.len(), 1);
}
