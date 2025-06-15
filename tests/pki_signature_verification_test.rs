/// fr fr PKI Certificate Signature Verification Test Suite - periodt
/// 
/// This test suite validates the enhanced PKI certificate signature verification
/// functionality with real cryptographic operations for RSA, ECDSA, and Ed25519
/// signatures using SHA-256, SHA-384, and SHA-512 hash algorithms.

use std::time::{SystemTime, UNIX_EPOCH};
use cursed::stdlib::crypto::certificates::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// Initialize test logging
    fn init_test_logging() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .try_init();
    }

    /// Create a test certificate processor
    fn create_test_processor() -> CertificateProcessor {
        CertificateProcessor::with_config(CertificateConfig {
            check_expiration: false, // Allow test certificates
            check_hostname: false,
            check_revocation: false,
            allow_self_signed: true,
            max_chain_length: 10,
            signature_verification: true,
        })
    }

    /// Create a test RSA public key for signature verification
    fn create_test_rsa_public_key() -> PublicKeyInfo {
        // Sample RSA public key in SPKI DER format (2048-bit)
        let spki_der = vec![
            0x30, 0x82, 0x01, 0x22, // SEQUENCE, length 290
            0x30, 0x0d, // SEQUENCE, length 13
            0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01, // RSA OID
            0x05, 0x00, // NULL
            0x03, 0x82, 0x01, 0x0f, 0x00, // BIT STRING, length 271
            // Sample RSA public key modulus and exponent
            0x30, 0x82, 0x01, 0x0a, 0x02, 0x82, 0x01, 0x01, 0x00,
            // 256 bytes of sample modulus data
            0xc3, 0x02, 0x35, 0x8a, 0x17, 0x4c, 0xf6, 0x8a,
            0x8b, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x9a,
            0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78, 0x90,
            0x02, 0x03, 0x01, 0x00, 0x01, // Exponent 65537
        ];

        PublicKeyInfo {
            algorithm: PublicKeyAlgorithm::RsaEncryption,
            key_data: spki_der,
            parameters: None,
        }
    }

    /// Create a test ECDSA P-256 public key
    fn create_test_ecdsa_p256_public_key() -> PublicKeyInfo {
        // Sample ECDSA P-256 public key in SPKI DER format
        let spki_der = vec![
            0x30, 0x59, // SEQUENCE, length 89
            0x30, 0x13, // SEQUENCE, length 19
            0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, // EC Public Key OID
            0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, // P-256 OID
            0x03, 0x42, 0x00, // BIT STRING, length 66
            0x04, // Uncompressed point indicator
            // 32 bytes X coordinate
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            // 32 bytes Y coordinate
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
        ];

        PublicKeyInfo {
            algorithm: PublicKeyAlgorithm::EcPublicKey,
            key_data: spki_der,
            parameters: None,
        }
    }

    /// Create a test Ed25519 public key
    fn create_test_ed25519_public_key() -> PublicKeyInfo {
        // Sample Ed25519 public key in SPKI DER format
        let spki_der = vec![
            0x30, 0x2a, // SEQUENCE, length 42
            0x30, 0x05, // SEQUENCE, length 5
            0x06, 0x03, 0x2b, 0x65, 0x70, // Ed25519 OID
            0x03, 0x21, 0x00, // BIT STRING, length 33
            // 32 bytes of Ed25519 public key
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
        ];

        PublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Ed25519,
            key_data: spki_der,
            parameters: None,
        }
    }

    /// Create a test certificate with RSA signature
    fn create_test_rsa_certificate() -> X509Certificate {
        let now = SystemTime::now();
        let not_before = now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 - 3600;
        let not_after = not_before + (365 * 24 * 3600); // 1 year validity

        X509Certificate {
            version: 3,
            serial_number: vec![0x01, 0x02, 0x03, 0x04],
            signature_algorithm: SignatureAlgorithm::Sha256WithRsaEncryption,
            issuer: DistinguishedName::new().with_common_name("Test CA"),
            validity: Validity {
                not_before: UNIX_EPOCH + std::time::Duration::from_secs(not_before as u64),
                not_after: UNIX_EPOCH + std::time::Duration::from_secs(not_after as u64),
            },
            subject: DistinguishedName::new().with_common_name("Test Certificate"),
            public_key: create_test_rsa_public_key(),
            extensions: Vec::new(),
            signature: vec![0x42; 256], // Sample RSA signature (256 bytes for 2048-bit RSA)
            raw_der: vec![0x30; 1024], // Sample DER data
        }
    }

    #[test]
    fn test_certificate_processor_creation() {
        init_test_logging();
        tracing::info!("Testing certificate processor creation");
        
        let processor = create_test_processor();
        assert!(processor.config.signature_verification);
        assert!(processor.config.allow_self_signed);
        assert!(!processor.config.check_expiration);
    }

    #[test]
    fn test_rsa_signature_validation_with_valid_key() {
        init_test_logging();
        tracing::info!("Testing RSA signature validation with valid key format");
        
        let processor = create_test_processor();
        let public_key = create_test_rsa_public_key();
        
        // Test with sample data and signature
        let signed_data = b"Hello, World!";
        let signature = vec![0x00; 256]; // Invalid signature for test
        
        // The verification should fail gracefully (return false) rather than error
        let result = processor.verify_rsa_signature(signed_data, &signature, &public_key, "sha256");
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Signature should be invalid
    }

    #[test]
    fn test_rsa_signature_with_invalid_parameters() {
        init_test_logging();
        tracing::info!("Testing RSA signature validation with invalid parameters");
        
        let processor = create_test_processor();
        let public_key = create_test_rsa_public_key();
        
        // Test with empty data
        let result = processor.verify_rsa_signature(&[], &[0x42; 256], &public_key, "sha256");
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        // Test with empty signature
        let result = processor.verify_rsa_signature(b"test", &[], &public_key, "sha256");
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        // Test with unsupported hash algorithm
        let result = processor.verify_rsa_signature(b"test", &[0x42; 256], &public_key, "md5");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CertificateError::UnsupportedAlgorithm(_)));
    }

    #[test]
    fn test_rsa_signature_with_insufficient_key_size() {
        init_test_logging();
        tracing::info!("Testing RSA signature validation with insufficient key size");
        
        let processor = create_test_processor();
        
        // Create a public key with insufficient size (smaller than 1024 bits)
        let small_key = PublicKeyInfo {
            algorithm: PublicKeyAlgorithm::RsaEncryption,
            key_data: vec![0x30, 0x47], // Too small for valid RSA key
            parameters: None,
        };
        
        let result = processor.verify_rsa_signature(b"test", &[0x42; 64], &small_key, "sha256");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CertificateError::InvalidPublicKey));
    }

    #[test]
    fn test_ecdsa_signature_validation_with_p256() {
        init_test_logging();
        tracing::info!("Testing ECDSA signature validation with P-256 curve");
        
        let processor = create_test_processor();
        let public_key = create_test_ecdsa_p256_public_key();
        
        // Create a valid DER-encoded ECDSA signature structure
        let signature = vec![
            0x30, 0x44, // SEQUENCE, length 68
            0x02, 0x20, // INTEGER, length 32 (r component)
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
            0x02, 0x20, // INTEGER, length 32 (s component)
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
            0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
        ];
        
        let signed_data = b"Hello, ECDSA!";
        
        // The verification should handle the format gracefully
        let result = processor.verify_ecdsa_signature(signed_data, &signature, &public_key, "sha256");
        assert!(result.is_ok());
        // Note: Will likely return false since it's not a real signature, but should not error
    }

    #[test]
    fn test_ecdsa_signature_with_invalid_der_structure() {
        init_test_logging();
        tracing::info!("Testing ECDSA signature validation with invalid DER structure");
        
        let processor = create_test_processor();
        let public_key = create_test_ecdsa_p256_public_key();
        
        // Invalid signature (not DER encoded)
        let invalid_signature = vec![0x42; 32];
        
        let result = processor.verify_ecdsa_signature(b"test", &invalid_signature, &public_key, "sha256");
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should fail validation
    }

    #[test]
    fn test_ecdsa_signature_with_different_curves() {
        init_test_logging();
        tracing::info!("Testing ECDSA signature validation with different curve configurations");
        
        let processor = create_test_processor();
        let public_key = create_test_ecdsa_p256_public_key();
        
        // Test with SHA-384 (should try P-384 first, then fallback)
        let signature = vec![0x30, 0x44, 0x02, 0x20]; // Truncated for test
        let signature = [signature, vec![0x12; 32], vec![0x02, 0x20], vec![0x34; 32]].concat();
        
        let result = processor.verify_ecdsa_signature(b"test", &signature, &public_key, "sha384");
        assert!(result.is_ok());
        
        // Test with SHA-512 (should try P-521 first, then fallback)
        let result = processor.verify_ecdsa_signature(b"test", &signature, &public_key, "sha512");
        assert!(result.is_ok());
    }

    #[test]
    fn test_ed25519_signature_validation() {
        init_test_logging();
        tracing::info!("Testing Ed25519 signature validation");
        
        let processor = create_test_processor();
        let public_key = create_test_ed25519_public_key();
        
        // Ed25519 signatures are always 64 bytes
        let signature = vec![0x42; 64];
        let signed_data = b"Hello, Ed25519!";
        
        let result = processor.verify_ed25519_signature(signed_data, &signature, &public_key);
        assert!(result.is_ok());
        // Note: Will likely return false since it's not a real signature, but should not error
    }

    #[test]
    fn test_ed25519_signature_with_invalid_length() {
        init_test_logging();
        tracing::info!("Testing Ed25519 signature validation with invalid signature length");
        
        let processor = create_test_processor();
        let public_key = create_test_ed25519_public_key();
        
        // Invalid signature length (Ed25519 requires exactly 64 bytes)
        let invalid_signature = vec![0x42; 32];
        
        let result = processor.verify_ed25519_signature(b"test", &invalid_signature, &public_key);
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should fail due to invalid length
    }

    #[test]
    fn test_ed25519_spki_key_extraction() {
        init_test_logging();
        tracing::info!("Testing Ed25519 public key extraction from SPKI format");
        
        let processor = create_test_processor();
        let public_key = create_test_ed25519_public_key();
        
        // Test SPKI key extraction
        let extracted_key = processor.extract_ed25519_key_from_spki(&public_key.key_data);
        assert!(extracted_key.is_ok());
        assert_eq!(extracted_key.unwrap().len(), 32);
    }

    #[test]
    fn test_ecdsa_signature_structure_validation() {
        init_test_logging();
        tracing::info!("Testing ECDSA signature DER structure validation");
        
        let processor = create_test_processor();
        
        // Valid DER structure
        let valid_signature = vec![0x30, 0x44]; // SEQUENCE with length 68
        let signature = [valid_signature, vec![0x00; 68]].concat();
        
        let result = processor.validate_ecdsa_signature_structure(&signature, 64, 72);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Invalid DER structure (wrong tag)
        let invalid_signature = vec![0x31, 0x44]; // Wrong tag
        let signature = [invalid_signature, vec![0x00; 68]].concat();
        
        let result = processor.validate_ecdsa_signature_structure(&signature, 64, 72);
        assert!(result.is_ok());
        assert!(!result.unwrap());
        
        // Invalid length
        let oversized_signature = vec![0x00; 200];
        let result = processor.validate_ecdsa_signature_structure(&oversized_signature, 64, 72);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_certificate_signature_verification_flow() {
        init_test_logging();
        tracing::info!("Testing complete certificate signature verification flow");
        
        let processor = create_test_processor();
        let cert = create_test_rsa_certificate();
        
        // Test self-signed certificate verification
        let result = processor.verify_certificate_signature(&cert, &cert);
        assert!(result.is_ok());
        // Note: Will likely return false since it's not a real signature, but should not error
    }

    #[test]
    fn test_multiple_hash_algorithms() {
        init_test_logging();
        tracing::info!("Testing signature verification with multiple hash algorithms");
        
        let processor = create_test_processor();
        let rsa_key = create_test_rsa_public_key();
        let ecdsa_key = create_test_ecdsa_p256_public_key();
        
        let test_data = b"Test data for hashing";
        let rsa_signature = vec![0x00; 256];
        let ecdsa_signature = vec![0x30, 0x44, 0x02, 0x20];
        let ecdsa_signature = [ecdsa_signature, vec![0x12; 32], vec![0x02, 0x20], vec![0x34; 32]].concat();
        
        // Test RSA with different hash algorithms
        for hash_alg in &["sha256", "sha384", "sha512"] {
            let result = processor.verify_rsa_signature(test_data, &rsa_signature, &rsa_key, hash_alg);
            assert!(result.is_ok(), "RSA verification failed for {}", hash_alg);
        }
        
        // Test ECDSA with different hash algorithms
        for hash_alg in &["sha256", "sha384", "sha512"] {
            let result = processor.verify_ecdsa_signature(test_data, &ecdsa_signature, &ecdsa_key, hash_alg);
            assert!(result.is_ok(), "ECDSA verification failed for {}", hash_alg);
        }
    }

    #[test]
    fn test_error_handling_edge_cases() {
        init_test_logging();
        tracing::info!("Testing error handling for edge cases");
        
        let processor = create_test_processor();
        
        // Wrong algorithm type for RSA
        let ecdsa_key = create_test_ecdsa_p256_public_key();
        let result = processor.verify_rsa_signature(b"test", &[0x00; 256], &ecdsa_key, "sha256");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CertificateError::UnsupportedAlgorithm(_)));
        
        // Wrong algorithm type for ECDSA
        let rsa_key = create_test_rsa_public_key();
        let result = processor.verify_ecdsa_signature(b"test", &[0x30; 70], &rsa_key, "sha256");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CertificateError::UnsupportedAlgorithm(_)));
        
        // Wrong algorithm type for Ed25519
        let result = processor.verify_ed25519_signature(b"test", &[0x00; 64], &rsa_key);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CertificateError::UnsupportedAlgorithm(_)));
    }

    #[test]
    fn test_signature_verification_performance() {
        init_test_logging();
        tracing::info!("Testing signature verification performance characteristics");
        
        let processor = create_test_processor();
        let rsa_key = create_test_rsa_public_key();
        let test_data = b"Performance test data";
        let signature = vec![0x00; 256];
        
        let start = std::time::Instant::now();
        
        // Perform multiple verification attempts
        for _ in 0..100 {
            let _ = processor.verify_rsa_signature(test_data, &signature, &rsa_key, "sha256");
        }
        
        let duration = start.elapsed();
        tracing::info!("100 RSA signature verifications took: {:?}", duration);
        
        // Should complete within reasonable time (< 1 second for 100 ops)
        assert!(duration.as_secs() < 1, "Signature verification too slow: {:?}", duration);
    }

    #[test]
    fn test_concurrent_signature_verification() {
        init_test_logging();
        tracing::info!("Testing concurrent signature verification");
        
        use std::sync::Arc;
        use std::thread;
        
        let processor = Arc::new(create_test_processor());
        let rsa_key = Arc::new(create_test_rsa_public_key());
        let test_data = b"Concurrent test data";
        let signature = Arc::new(vec![0x00; 256]);
        
        let mut handles = Vec::new();
        
        // Spawn multiple threads for concurrent verification
        for i in 0..4 {
            let processor_clone = Arc::clone(&processor);
            let key_clone = Arc::clone(&rsa_key);
            let sig_clone = Arc::clone(&signature);
            
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let result = processor_clone.verify_rsa_signature(
                        test_data, 
                        &sig_clone, 
                        &key_clone, 
                        "sha256"
                    );
                    assert!(result.is_ok(), "Thread {} iteration {} failed", i, j);
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        tracing::info!("Concurrent signature verification completed successfully");
    }
}
