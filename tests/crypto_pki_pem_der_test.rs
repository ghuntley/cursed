/// Comprehensive test suite for PEM/DER format handling
/// 
/// This test suite validates the PKI PEM/DER module functionality including:
/// - Format detection and validation
/// - PEM parsing and encoding
/// - DER parsing and ASN.1 structure handling
/// - Format conversion between PEM and DER
/// - Certificate and private key parsing
/// - Encrypted PEM handling
/// - Certificate chain validation
/// - Error handling and edge cases

#[path = "common.rs"]
mod common;

use cursed::stdlib::packages::crypto_pki::pem_der::*;
use cursed::stdlib::packages::crypto_pki::*;

/// Test basic format detection
#[test]
fn test_format_detection_basic() {
    common::tracing::init_test_tracing();
    
    // Test PEM format detection
    let pem_cert = "-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAKZJ...";
    assert_eq!(format_detection::detect_format(pem_cert.as_bytes()), FormatType::Pem);
    
    // Test DER format detection (starts with SEQUENCE tag)
    let der_data = &[0x30, 0x82, 0x02, 0x5A]; // SEQUENCE with long form length
    assert_eq!(format_detection::detect_format(der_data), FormatType::Der);
    
    // Test unknown format
    let unknown_data = b"This is not a certificate or key";
    assert_eq!(format_detection::detect_format(unknown_data), FormatType::Unknown);
    
    // Test empty data
    let empty_data = &[];
    assert_eq!(format_detection::detect_format(empty_data), FormatType::Unknown);
}

/// Test certificate detection
#[test]
fn test_certificate_detection() {
    common::tracing::init_test_tracing();
    
    // Test PEM certificate detection
    let pem_cert = "-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAKZJ...";
    assert!(format_detection::is_certificate(pem_cert.as_bytes()));
    
    // Test PEM private key (should not be detected as certificate)
    let pem_key = "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkq...";
    assert!(!format_detection::is_certificate(pem_key.as_bytes()));
    
    // Test DER certificate detection
    let der_cert = &[0x30, 0x82, 0x02, 0x5A, 0x30, 0x82]; // Basic SEQUENCE structure
    assert!(format_detection::is_certificate(der_cert));
    
    // Test non-certificate data
    let text_data = b"Just some text";
    assert!(!format_detection::is_certificate(text_data));
}

/// Test private key detection
#[test]
fn test_private_key_detection() {
    common::tracing::init_test_tracing();
    
    // Test various private key formats
    let formats = [
        "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkq...",
        "-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQEA...",
        "-----BEGIN EC PRIVATE KEY-----\nMHcCAQEEIKGc+gBM...",
        "-----BEGIN ENCRYPTED PRIVATE KEY-----\nMIIFHDBOBgkqhkiG...",
    ];
    
    for format in &formats {
        assert!(format_detection::is_private_key(format.as_bytes()));
    }
    
    // Test non-private key data
    let cert_data = "-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAKZJ...";
    assert!(!format_detection::is_private_key(cert_data.as_bytes()));
}

/// Test PEM type handling
#[test]
fn test_pem_type_handling() {
    common::tracing::init_test_tracing();
    
    // Test header mapping
    assert_eq!(PemType::Certificate.header(), "CERTIFICATE");
    assert_eq!(PemType::PrivateKey.header(), "PRIVATE KEY");
    assert_eq!(PemType::RsaPrivateKey.header(), "RSA PRIVATE KEY");
    assert_eq!(PemType::EncryptedPrivateKey.header(), "ENCRYPTED PRIVATE KEY");
    
    // Test header parsing
    assert_eq!(PemType::from_header("CERTIFICATE"), PemType::Certificate);
    assert_eq!(PemType::from_header("PRIVATE KEY"), PemType::PrivateKey);
    
    // Test custom header
    let custom = PemType::from_header("CUSTOM HEADER");
    if let PemType::Custom(header) = custom {
        assert_eq!(header, "CUSTOM HEADER");
    } else {
        panic!("Expected Custom variant");
    }
}

/// Test PEM block creation and manipulation
#[test]
fn test_pem_block_creation() {
    common::tracing::init_test_tracing();
    
    let test_data = vec![0x30, 0x82, 0x01, 0x00, 0x01, 0x02, 0x03];
    let mut block = PemBlock::new(PemType::Certificate, test_data.clone());
    
    // Test basic properties
    assert_eq!(block.data, test_data);
    assert_eq!(block.pem_type, PemType::Certificate);
    assert!(!block.is_encrypted());
    
    // Test header manipulation
    block.add_header("Version".to_string(), "3".to_string());
    block.add_header("Subject".to_string(), "CN=Test".to_string());
    
    assert_eq!(block.get_header("Version"), Some(&"3".to_string()));
    assert_eq!(block.get_header("Subject"), Some(&"CN=Test".to_string()));
    assert_eq!(block.get_header("NonExistent"), None);
    
    // Test encryption detection
    let mut encrypted_block = PemBlock::new(PemType::EncryptedPrivateKey, vec![]);
    assert!(encrypted_block.is_encrypted());
    
    encrypted_block = PemBlock::new(PemType::PrivateKey, vec![]);
    encrypted_block.add_header("Proc-Type".to_string(), "4,ENCRYPTED".to_string());
    assert!(encrypted_block.is_encrypted());
    
    encrypted_block.add_header("DEK-Info".to_string(), "AES-256-CBC,1234567890ABCDEF".to_string());
    assert!(encrypted_block.is_encrypted());
}

/// Test PEM block encoding
#[test]
fn test_pem_block_encoding() {
    common::tracing::init_test_tracing();
    
    let test_data = b"Hello, World! This is test data for base64 encoding.";
    let mut block = PemBlock::new(PemType::Certificate, test_data.to_vec());
    
    // Add some headers
    block.add_header("Version".to_string(), "3".to_string());
    block.add_header("Serial".to_string(), "12345".to_string());
    
    let encoded = block.encode();
    
    // Verify PEM structure
    assert!(encoded.starts_with("-----BEGIN CERTIFICATE-----\n"));
    assert!(encoded.ends_with("-----END CERTIFICATE-----\n"));
    assert!(encoded.contains("Version: 3\n"));
    assert!(encoded.contains("Serial: 12345\n"));
    
    // Verify base64 content is present
    let lines: Vec<&str> = encoded.lines().collect();
    let mut base64_lines = Vec::new();
    let mut in_data = false;
    
    for line in lines {
        if line == "-----BEGIN CERTIFICATE-----" {
            in_data = true;
            continue;
        }
        if line == "-----END CERTIFICATE-----" {
            break;
        }
        if in_data && !line.contains(':') && !line.is_empty() {
            base64_lines.push(line);
        }
    }
    
    assert!(!base64_lines.is_empty());
}

/// Test ASN.1 tag creation and properties
#[test]
fn test_asn1_tag_creation() {
    common::tracing::init_test_tracing();
    
    // Test basic tag creation
    let integer_tag = Asn1Tag {
        class: 0,        // Universal
        constructed: false,
        tag_number: 2,   // INTEGER
    };
    
    let sequence_tag = Asn1Tag {
        class: 0,        // Universal
        constructed: true,
        tag_number: 16,  // SEQUENCE
    };
    
    // Test tag properties
    assert_eq!(integer_tag.class, 0);
    assert!(!integer_tag.constructed);
    assert_eq!(integer_tag.tag_number, 2);
    
    assert_eq!(sequence_tag.class, 0);
    assert!(sequence_tag.constructed);
    assert_eq!(sequence_tag.tag_number, 16);
}

/// Test ASN.1 element creation and data extraction
#[test]
fn test_asn1_element_creation() {
    common::tracing::init_test_tracing();
    
    // Test INTEGER element
    let integer_tag = Asn1Tag {
        class: 0,
        constructed: false,
        tag_number: 2,
    };
    let integer_element = Asn1Element::new(integer_tag, vec![0x01]); // INTEGER 1
    
    assert_eq!(integer_element.as_integer().unwrap(), 1);
    assert!(!integer_element.is_sequence());
    assert!(!integer_element.is_set());
    
    // Test larger integer
    let big_integer_element = Asn1Element::new(
        Asn1Tag { class: 0, constructed: false, tag_number: 2 },
        vec![0x01, 0x00] // INTEGER 256
    );
    assert_eq!(big_integer_element.as_integer().unwrap(), 256);
    
    // Test negative integer
    let negative_element = Asn1Element::new(
        Asn1Tag { class: 0, constructed: false, tag_number: 2 },
        vec![0xFF] // INTEGER -1
    );
    assert_eq!(negative_element.as_integer().unwrap(), -1);
    
    // Test SEQUENCE element
    let sequence_tag = Asn1Tag {
        class: 0,
        constructed: true,
        tag_number: 16,
    };
    let sequence_element = Asn1Element::new(sequence_tag, vec![]);
    
    assert!(sequence_element.is_sequence());
    assert!(!sequence_element.is_set());
    
    // Test SET element
    let set_tag = Asn1Tag {
        class: 0,
        constructed: true,
        tag_number: 17,
    };
    let set_element = Asn1Element::new(set_tag, vec![]);
    
    assert!(!set_element.is_sequence());
    assert!(set_element.is_set());
}

/// Test ASN.1 string handling
#[test]
fn test_asn1_string_handling() {
    common::tracing::init_test_tracing();
    
    // Test UTF8String
    let utf8_tag = Asn1Tag {
        class: 0,
        constructed: false,
        tag_number: 12,
    };
    let utf8_element = Asn1Element::new(utf8_tag, "Hello, World!".as_bytes().to_vec());
    assert_eq!(utf8_element.as_string().unwrap(), "Hello, World!");
    
    // Test PrintableString
    let printable_tag = Asn1Tag {
        class: 0,
        constructed: false,
        tag_number: 19,
    };
    let printable_element = Asn1Element::new(printable_tag, "Test123".as_bytes().to_vec());
    assert_eq!(printable_element.as_string().unwrap(), "Test123");
    
    // Test error for non-string type
    let integer_element = Asn1Element::new(
        Asn1Tag { class: 0, constructed: false, tag_number: 2 },
        vec![0x01]
    );
    assert!(integer_element.as_string().is_err());
}

/// Test ASN.1 boolean handling
#[test]
fn test_asn1_boolean_handling() {
    common::tracing::init_test_tracing();
    
    let boolean_tag = Asn1Tag {
        class: 0,
        constructed: false,
        tag_number: 1,
    };
    
    // Test TRUE
    let true_element = Asn1Element::new(boolean_tag.clone(), vec![0xFF]);
    assert_eq!(true_element.as_boolean().unwrap(), true);
    
    // Test FALSE
    let false_element = Asn1Element::new(boolean_tag.clone(), vec![0x00]);
    assert_eq!(false_element.as_boolean().unwrap(), false);
    
    // Test error for non-boolean type
    let integer_element = Asn1Element::new(
        Asn1Tag { class: 0, constructed: false, tag_number: 2 },
        vec![0x01]
    );
    assert!(integer_element.as_boolean().is_err());
    
    // Test error for invalid boolean length
    let invalid_boolean = Asn1Element::new(boolean_tag, vec![0x00, 0x01]);
    assert!(invalid_boolean.as_boolean().is_err());
}

/// Test simple PEM parsing
#[test]
fn test_simple_pem_parsing() {
    common::tracing::init_test_tracing();
    
    let simple_pem = "-----BEGIN CERTIFICATE-----\nSGVsbG8gV29ybGQ=\n-----END CERTIFICATE-----";
    
    let blocks = pem::parse_pem(simple_pem).expect("Failed to parse PEM");
    assert_eq!(blocks.len(), 1);
    
    let block = &blocks[0];
    assert_eq!(block.pem_type, PemType::Certificate);
    assert_eq!(block.data, b"Hello World");
    assert!(!block.is_encrypted());
}

/// Test multiple PEM block parsing
#[test]
fn test_multiple_pem_parsing() {
    common::tracing::init_test_tracing();
    
    let multiple_pem = r#"-----BEGIN CERTIFICATE-----
VGVzdCBDZXJ0aWZpY2F0ZSAx
-----END CERTIFICATE-----
-----BEGIN PRIVATE KEY-----
VGVzdCBQcml2YXRlIEtleQ==
-----END PRIVATE KEY-----"#;
    
    let blocks = pem::parse_pem(multiple_pem).expect("Failed to parse multiple PEM blocks");
    assert_eq!(blocks.len(), 2);
    
    assert_eq!(blocks[0].pem_type, PemType::Certificate);
    assert_eq!(blocks[0].data, b"Test Certificate 1");
    
    assert_eq!(blocks[1].pem_type, PemType::PrivateKey);
    assert_eq!(blocks[1].data, b"Test Private Key");
}

/// Test PEM parsing error cases
#[test]
fn test_pem_parsing_errors() {
    common::tracing::init_test_tracing();
    
    // Test mismatched headers
    let mismatched_pem = "-----BEGIN CERTIFICATE-----\nSGVsbG8=\n-----END PRIVATE KEY-----";
    assert!(pem::parse_pem(mismatched_pem).is_err());
    
    // Test invalid base64
    let invalid_base64_pem = "-----BEGIN CERTIFICATE-----\nInvalid@Base64!\n-----END CERTIFICATE-----";
    assert!(pem::parse_pem(invalid_base64_pem).is_err());
    
    // Test no PEM blocks
    let no_pem = "This is just regular text with no PEM blocks";
    assert!(pem::parse_pem(no_pem).is_err());
    
    // Test incomplete PEM block
    let incomplete_pem = "-----BEGIN CERTIFICATE-----\nSGVsbG8=";
    assert!(pem::parse_pem(incomplete_pem).is_err());
}

/// Test PEM encoding functionality
#[test]
fn test_pem_encoding() {
    common::tracing::init_test_tracing();
    
    let test_data = b"This is test data for PEM encoding";
    let encoded = pem::encode_pem(PemType::Certificate, test_data);
    
    // Verify structure
    assert!(encoded.starts_with("-----BEGIN CERTIFICATE-----\n"));
    assert!(encoded.ends_with("-----END CERTIFICATE-----\n"));
    
    // Verify round-trip
    let parsed = pem::parse_pem_single(&encoded).expect("Failed to parse encoded PEM");
    assert_eq!(parsed.data, test_data);
    assert_eq!(parsed.pem_type, PemType::Certificate);
}

/// Test basic DER element parsing
#[test]
fn test_der_element_parsing() {
    common::tracing::init_test_tracing();
    
    // Test simple INTEGER parsing (tag=02, length=01, value=05)
    let der_data = &[0x02, 0x01, 0x05];
    let (element, consumed) = der::parse_asn1_element(der_data).expect("Failed to parse DER element");
    
    assert_eq!(consumed, 3);
    assert_eq!(element.tag.class, 0);
    assert!(!element.tag.constructed);
    assert_eq!(element.tag.tag_number, 2);
    assert_eq!(element.data, vec![0x05]);
    assert_eq!(element.as_integer().unwrap(), 5);
}

/// Test DER long form length parsing
#[test]
fn test_der_long_form_length() {
    common::tracing::init_test_tracing();
    
    // Test long form length: tag=02, length=82 01 00 (256 bytes), followed by data
    let mut der_data = vec![0x02, 0x82, 0x01, 0x00]; // INTEGER with 256-byte length
    der_data.extend(vec![0x00; 256]); // 256 bytes of zeros
    
    let (element, consumed) = der::parse_asn1_element(&der_data).expect("Failed to parse long form DER");
    
    assert_eq!(consumed, 4 + 256);
    assert_eq!(element.data.len(), 256);
    assert_eq!(element.tag.tag_number, 2);
}

/// Test DER sequence parsing
#[test]
fn test_der_sequence_parsing() {
    common::tracing::init_test_tracing();
    
    // Create a simple SEQUENCE containing two INTEGERs
    let der_data = &[
        0x30, 0x06,  // SEQUENCE, length 6
        0x02, 0x01, 0x01,  // INTEGER 1
        0x02, 0x01, 0x02,  // INTEGER 2
    ];
    
    let (element, consumed) = der::parse_asn1_element(der_data).expect("Failed to parse SEQUENCE");
    
    assert_eq!(consumed, 8);
    assert!(element.is_sequence());
    assert_eq!(element.children.len(), 2);
    
    assert_eq!(element.children[0].as_integer().unwrap(), 1);
    assert_eq!(element.children[1].as_integer().unwrap(), 2);
}

/// Test DER parsing error cases
#[test]
fn test_der_parsing_errors() {
    common::tracing::init_test_tracing();
    
    // Test empty data
    assert!(der::parse_asn1_element(&[]).is_err());
    
    // Test truncated length
    let truncated_length = &[0x02, 0x82]; // Long form length without length bytes
    assert!(der::parse_asn1_element(truncated_length).is_err());
    
    // Test truncated value
    let truncated_value = &[0x02, 0x05, 0x01]; // Claims 5 bytes but only has 1
    assert!(der::parse_asn1_element(truncated_value).is_err());
    
    // Test indefinite length (not allowed in DER)
    let indefinite_length = &[0x02, 0x80]; // Length = 80 (indefinite)
    assert!(der::parse_asn1_element(indefinite_length).is_err());
}

/// Test DER element encoding
#[test]
fn test_der_element_encoding() {
    common::tracing::init_test_tracing();
    
    // Test simple INTEGER encoding
    let tag = Asn1Tag {
        class: 0,
        constructed: false,
        tag_number: 2,
    };
    let element = Asn1Element::new(tag, vec![0x05]);
    let encoded = der::encode_der_element(&element);
    
    assert_eq!(encoded, vec![0x02, 0x01, 0x05]);
    
    // Test round-trip
    let (parsed, _) = der::parse_asn1_element(&encoded).expect("Failed to parse encoded element");
    assert_eq!(parsed.as_integer().unwrap(), 5);
}

/// Test format conversion utilities
#[test]
fn test_format_conversion() {
    common::tracing::init_test_tracing();
    
    let test_data = b"Test data for conversion";
    
    // Test DER to PEM conversion
    let pem_result = conversion::der_to_pem(test_data, PemType::Certificate);
    assert!(pem_result.contains("-----BEGIN CERTIFICATE-----"));
    assert!(pem_result.contains("-----END CERTIFICATE-----"));
    
    // Test PEM to DER conversion
    let der_result = conversion::pem_to_der(&pem_result).expect("Failed to convert PEM to DER");
    assert_eq!(der_result, test_data);
    
    // Test auto-detect conversion to DER
    let auto_der = conversion::to_der(pem_result.as_bytes()).expect("Failed auto-conversion to DER");
    assert_eq!(auto_der, test_data);
    
    // Test auto-detect conversion to PEM
    let auto_pem = conversion::to_pem(test_data, PemType::Certificate).expect("Failed auto-conversion to PEM");
    assert!(auto_pem.contains("-----BEGIN CERTIFICATE-----"));
}

/// Test certificate validation functionality
#[test]
fn test_certificate_validation() {
    common::tracing::init_test_tracing();
    
    // Test empty chain validation
    let empty_chain = vec![];
    let result = validation::validate_certificate_chain(&empty_chain);
    assert!(!result.valid);
    assert!(!result.errors.is_empty());
    assert_eq!(result.chain_length, 0);
    
    // Test single certificate chain
    let single_cert = Certificate::new();
    let single_chain = vec![single_cert];
    let result = validation::validate_certificate_chain(&single_chain);
    assert!(result.valid); // Basic validation passes
    assert!(!result.warnings.is_empty()); // But has warnings about single cert
    assert_eq!(result.chain_length, 1);
    
    // Test format validation with valid PEM
    let valid_pem = "-----BEGIN CERTIFICATE-----\nSGVsbG8=\n-----END CERTIFICATE-----";
    let is_valid = validation::validate_certificate_format(valid_pem.as_bytes())
        .expect("Failed to validate format");
    assert!(is_valid);
    
    // Test format validation with invalid data
    let invalid_data = b"Not a certificate";
    let is_valid = validation::validate_certificate_format(invalid_data)
        .expect("Failed to validate format");
    assert!(!is_valid);
}

/// Test certificate metadata extraction
#[test]
fn test_certificate_metadata_extraction() {
    common::tracing::init_test_tracing();
    
    let mut cert = Certificate::new();
    cert.serial_number = Some("12345".to_string());
    cert.der_encoded = Some(vec![0x30, 0x82, 0x01, 0x00]); // Mock DER data
    
    let metadata = metadata::extract_certificate_metadata(&cert)
        .expect("Failed to extract metadata");
    
    assert_eq!(metadata.serial_number, "12345");
    assert_ne!(metadata.subject, "");
    assert_ne!(metadata.issuer, "");
    assert_ne!(metadata.public_key_algorithm, "");
    assert_ne!(metadata.signature_algorithm, "");
}

/// Test private key metadata extraction
#[test]
fn test_private_key_metadata_extraction() {
    common::tracing::init_test_tracing();
    
    let mut private_key = PrivateKey::new();
    private_key.der_encoded = Some(vec![0x30, 0x82, 0x01, 0x00]); // Mock DER data
    
    let metadata = metadata::extract_private_key_metadata(&private_key)
        .expect("Failed to extract private key metadata");
    
    assert_ne!(metadata.algorithm, "");
    assert!(metadata.key_size > 0);
    assert_ne!(metadata.format, "");
}

/// Test high-level convenience functions
#[test]
fn test_convenience_functions() {
    common::tracing::init_test_tracing();
    
    // Test format info function
    let pem_data = "-----BEGIN CERTIFICATE-----\nSGVsbG8=\n-----END CERTIFICATE-----";
    let (format, is_cert, is_key) = get_format_info(pem_data.as_bytes());
    
    assert_eq!(format, FormatType::Pem);
    assert!(is_cert);
    assert!(!is_key);
    
    // Test DER format info
    let der_data = &[0x30, 0x82, 0x01, 0x00];
    let (format, is_cert, is_key) = get_format_info(der_data);
    
    assert_eq!(format, FormatType::Der);
    assert!(is_cert);
    assert!(!is_key);
    
    // Test private key format info
    let key_pem = "-----BEGIN PRIVATE KEY-----\nVGVzdA==\n-----END PRIVATE KEY-----";
    let (format, is_cert, is_key) = get_format_info(key_pem.as_bytes());
    
    assert_eq!(format, FormatType::Pem);
    assert!(!is_cert);
    assert!(is_key);
}

/// Test certificate bundle handling
#[test]
fn test_certificate_bundle_handling() {
    common::tracing::init_test_tracing();
    
    let bundle_pem = r#"-----BEGIN CERTIFICATE-----
VGVzdCBDZXJ0aWZpY2F0ZSAx
-----END CERTIFICATE-----
-----BEGIN CERTIFICATE-----
VGVzdCBDZXJ0aWZpY2F0ZSAy
-----END CERTIFICATE-----"#;
    
    let certificates = bundle::parse_certificate_bundle(bundle_pem)
        .expect("Failed to parse certificate bundle");
    
    assert_eq!(certificates.len(), 2);
    
    // Test bundle creation (simplified test)
    let result = bundle::create_certificate_bundle(&certificates);
    // Since certificates don't have DER data in this test, expect error
    assert!(result.is_err());
}

/// Test error conversion and display
#[test]
fn test_error_handling() {
    common::tracing::init_test_tracing();
    
    // Test various error types
    let errors = vec![
        PemDerError::InvalidPemFormat("test".to_string()),
        PemDerError::InvalidDerEncoding("test".to_string()),
        PemDerError::Asn1ParseError("test".to_string()),
        PemDerError::UnsupportedFormat("test".to_string()),
        PemDerError::CertificateValidationError("test".to_string()),
        PemDerError::PrivateKeyError("test".to_string()),
        PemDerError::EncryptionError("test".to_string()),
        PemDerError::Base64Error("test".to_string()),
        PemDerError::ValidationError("test".to_string()),
        PemDerError::IoError("test".to_string()),
    ];
    
    for error in errors {
        let error_string = error.to_string();
        assert!(!error_string.is_empty());
        assert!(error_string.contains("test"));
        
        // Test conversion to CursedError
        let cursed_error: CursedError = error.into();
        assert!(cursed_error.to_string().contains("test"));
    }
}

/// Test encrypted PEM detection and handling
#[test]
fn test_encrypted_pem_handling() {
    common::tracing::init_test_tracing();
    
    // Test basic encrypted PEM error handling (since actual decryption is not implemented)
    let encrypted_pem = r#"-----BEGIN ENCRYPTED PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-256-CBC,1234567890ABCDEF

VGVzdCBFbmNyeXB0ZWQgS2V5
-----END ENCRYPTED PRIVATE KEY-----"#;
    
    let result = encrypted::decrypt_pem_private_key(encrypted_pem, "password");
    assert!(result.is_err()); // Should fail since decryption is not implemented
    
    // Test encryption (should also fail with not implemented)
    let private_key = PrivateKey::new();
    let result = encrypted::encrypt_private_key_pem(&private_key, "password", "AES-256-CBC");
    assert!(result.is_err());
}

/// Test PKCS#12 basic handling
#[test]
fn test_pkcs12_basic_handling() {
    common::tracing::init_test_tracing();
    
    let pkcs12_data = b"Mock PKCS#12 data";
    let result = bundle::parse_pkcs12_basic(pkcs12_data, "password");
    assert!(result.is_err()); // Should fail since PKCS#12 parsing is not implemented
}

/// Test comprehensive format conversion
#[test]
fn test_comprehensive_format_conversion() {
    common::tracing::init_test_tracing();
    
    let test_data = b"Test data for comprehensive conversion testing";
    
    // Test PEM to DER conversion
    let pem_str = pem::encode_pem(PemType::Certificate, test_data);
    let converted_der = convert_format(
        pem_str.as_bytes(),
        FormatType::Der,
        PemType::Certificate
    ).expect("Failed PEM to DER conversion");
    assert_eq!(converted_der, test_data);
    
    // Test DER to PEM conversion
    let converted_pem = convert_format(
        test_data,
        FormatType::Pem,
        PemType::Certificate
    ).expect("Failed DER to PEM conversion");
    assert!(String::from_utf8_lossy(&converted_pem).contains("-----BEGIN CERTIFICATE-----"));
    
    // Test same format conversion (should return identical data)
    let same_format = convert_format(
        test_data,
        FormatType::Der,
        PemType::Certificate
    ).expect("Failed same format conversion");
    assert_eq!(same_format, test_data);
}

/// Test ASN.1 element tree traversal
#[test]
fn test_asn1_element_tree() {
    common::tracing::init_test_tracing();
    
    // Create nested SEQUENCE structure
    let inner_sequence_data = &[
        0x30, 0x06,  // SEQUENCE, length 6
        0x02, 0x01, 0x01,  // INTEGER 1
        0x02, 0x01, 0x02,  // INTEGER 2
    ];
    
    let outer_sequence_data = &[
        0x30, 0x0A,  // SEQUENCE, length 10
        0x02, 0x01, 0x00,  // INTEGER 0
        0x30, 0x06,  // SEQUENCE, length 6
        0x02, 0x01, 0x01,  // INTEGER 1
        0x02, 0x01, 0x02,  // INTEGER 2
    ];
    
    let (outer_element, _) = der::parse_asn1_element(outer_sequence_data)
        .expect("Failed to parse nested SEQUENCE");
    
    assert!(outer_element.is_sequence());
    assert_eq!(outer_element.children.len(), 2);
    
    // Check first child (INTEGER 0)
    assert_eq!(outer_element.children[0].as_integer().unwrap(), 0);
    
    // Check second child (nested SEQUENCE)
    let inner_sequence = &outer_element.children[1];
    assert!(inner_sequence.is_sequence());
    assert_eq!(inner_sequence.children.len(), 2);
    assert_eq!(inner_sequence.children[0].as_integer().unwrap(), 1);
    assert_eq!(inner_sequence.children[1].as_integer().unwrap(), 2);
}

/// Test DER parsing with large tag numbers
#[test]
fn test_der_large_tag_numbers() {
    common::tracing::init_test_tracing();
    
    // Test tag number > 30 (long form)
    let long_tag_data = &[
        0x9F, 0x22,  // Tag: context-specific, primitive, tag number 34 (long form)
        0x01,        // Length: 1
        0x05,        // Value: 5
    ];
    
    let (element, consumed) = der::parse_asn1_element(long_tag_data)
        .expect("Failed to parse long form tag");
    
    assert_eq!(consumed, 4);
    assert_eq!(element.tag.class, 2); // Context-specific
    assert!(!element.tag.constructed);
    assert_eq!(element.tag.tag_number, 34);
    assert_eq!(element.data, vec![0x05]);
}

/// Test PEM parsing with headers
#[test]
fn test_pem_with_headers() {
    common::tracing::init_test_tracing();
    
    let pem_with_headers = r#"-----BEGIN CERTIFICATE-----
Version: 3
Serial: 12345

VGVzdCBDZXJ0aWZpY2F0ZQ==
-----END CERTIFICATE-----"#;
    
    // Note: Current implementation doesn't parse headers between BEGIN and data
    // This test documents the current behavior
    let result = pem::parse_pem(pem_with_headers);
    // Should fail due to invalid base64 (headers are included)
    assert!(result.is_err());
}

/// Test edge cases and boundary conditions
#[test]
fn test_edge_cases() {
    common::tracing::init_test_tracing();
    
    // Test zero-length data
    let zero_length_data = &[0x02, 0x00]; // INTEGER with zero length
    let (element, _) = der::parse_asn1_element(zero_length_data)
        .expect("Failed to parse zero-length element");
    assert_eq!(element.as_integer().unwrap(), 0);
    
    // Test very small PEM block
    let tiny_pem = "-----BEGIN CERTIFICATE-----\nQQ==\n-----END CERTIFICATE-----";
    let block = pem::parse_pem_single(tiny_pem).expect("Failed to parse tiny PEM");
    assert_eq!(block.data, b"A");
    
    // Test PEM with extra whitespace
    let whitespace_pem = "-----BEGIN CERTIFICATE-----\n  SG VsbG8g V29ybGQ=  \n-----END CERTIFICATE-----";
    let block = pem::parse_pem_single(whitespace_pem).expect("Failed to parse whitespace PEM");
    assert_eq!(block.data, b"Hello World");
}
