/// Comprehensive test suite for CURSED asymmetric cryptography implementation
/// 
/// This test suite validates all asymmetric cryptographic operations including
/// RSA, ECDSA, ECDH, X25519, and Ed25519 across various configurations,
/// security properties, and edge cases.

use cursed::stdlib::crypto::asymmetric::*;

/// Test RSA key generation with different key sizes
#[test]
fn test_rsa_key_generation_sizes() {
    let crypto = AsymmetricCrypto::new();
    
    // Test supported key sizes
    for &key_size in &[RSA_2048_BITS, RSA_3072_BITS, RSA_4096_BITS] {
        let result = crypto.rsa_generate_keypair(Some(key_size));
        assert!(result.is_ok(), "Failed to generate RSA key of size {}", key_size);
        
        let keypair = result.unwrap();
        assert_eq!(keypair.key_size, key_size);
        assert_eq!(keypair.public_key.key_size, key_size);
        assert_eq!(keypair.private_key.key_size, key_size);
    }
}

/// Test RSA key generation with invalid key sizes
#[test]
fn test_rsa_invalid_key_sizes() {
    let crypto = AsymmetricCrypto::new();
    
    // Test invalid key sizes
    for &invalid_size in &[1024, 1536, 5000, 8192] {
        let result = crypto.rsa_generate_keypair(Some(invalid_size));
        assert!(result.is_err(), "Should fail for invalid key size {}", invalid_size);
        
        if let Err(AsymmetricError::InvalidKeySize(size)) = result {
            assert_eq!(size, invalid_size);
        } else {
            panic!("Expected InvalidKeySize error for size {}", invalid_size);
        }
    }
}

/// Test RSA encryption and decryption with different padding schemes
#[test]
fn test_rsa_encryption_decryption_padding() {
    let crypto = AsymmetricCrypto::new();
    let keypair = crypto.rsa_generate_keypair(Some(RSA_2048_BITS)).unwrap();
    
    let test_data = b"Hello, RSA World! This is a test message.";
    
    // Test different padding schemes for encryption
    let padding_schemes = vec![
        RsaPadding::Pkcs1v15,
        RsaPadding::OaepSha256,
        RsaPadding::OaepSha384,
        RsaPadding::OaepSha512,
    ];
    
    for padding in padding_schemes {
        println!("Testing RSA with padding: {:?}", padding);
        
        // Encrypt
        let ciphertext = crypto.rsa_encrypt(&keypair.public_key, test_data, Some(padding));
        assert!(ciphertext.is_ok(), "Encryption failed for padding {:?}", padding);
        
        let encrypted_data = ciphertext.unwrap();
        assert_ne!(encrypted_data, test_data.to_vec());
        
        // Decrypt
        let plaintext = crypto.rsa_decrypt(&keypair.private_key, &encrypted_data, Some(padding));
        assert!(plaintext.is_ok(), "Decryption failed for padding {:?}", padding);
        
        let decrypted_data = plaintext.unwrap();
        assert_eq!(decrypted_data, test_data.to_vec());
    }
}

/// Test RSA signing and verification with different padding schemes
#[test]
fn test_rsa_signing_verification_padding() {
    let crypto = AsymmetricCrypto::new();
    let keypair = crypto.rsa_generate_keypair(Some(RSA_2048_BITS)).unwrap();
    
    let message = b"Important document to be signed";
    
    // Test different padding schemes for signing
    let padding_schemes = vec![
        RsaPadding::Pkcs1v15,
        RsaPadding::Pss,
    ];
    
    for padding in padding_schemes {
        println!("Testing RSA signing with padding: {:?}", padding);
        
        // Sign
        let signature = crypto.rsa_sign(&keypair.private_key, message, Some(padding));
        assert!(signature.is_ok(), "Signing failed for padding {:?}", padding);
        
        let sig_bytes = signature.unwrap();
        assert!(!sig_bytes.is_empty());
        
        // Verify
        let is_valid = crypto.rsa_verify(&keypair.public_key, message, &sig_bytes, Some(padding));
        assert!(is_valid.is_ok(), "Verification failed for padding {:?}", padding);
        assert!(is_valid.unwrap(), "Signature should be valid for padding {:?}", padding);
        
        // Test with tampered message
        let tampered_message = b"Tampered document content";
        let is_invalid = crypto.rsa_verify(&keypair.public_key, tampered_message, &sig_bytes, Some(padding));
        assert!(is_invalid.is_ok());
        assert!(!is_invalid.unwrap(), "Signature should be invalid for tampered message");
    }
}

/// Test ECDSA key generation for different curves
#[test]
fn test_ecdsa_key_generation_curves() {
    let crypto = AsymmetricCrypto::new();
    
    let supported_curves = vec![EcCurve::P256, EcCurve::Secp256k1];
    
    for curve in supported_curves {
        println!("Testing ECDSA key generation for curve: {:?}", curve);
        
        let result = crypto.ecdsa_generate_keypair(Some(curve));
        assert!(result.is_ok(), "Failed to generate ECDSA keypair for curve {:?}", curve);
        
        let keypair = result.unwrap();
        assert_eq!(keypair.curve, curve);
        assert_eq!(keypair.public_key.curve, curve);
        assert_eq!(keypair.private_key.curve, curve);
    }
}

/// Test ECDSA signing and verification
#[test]
fn test_ecdsa_signing_verification() {
    let crypto = AsymmetricCrypto::new();
    
    let curves = vec![EcCurve::P256, EcCurve::Secp256k1];
    
    for curve in curves {
        println!("Testing ECDSA operations for curve: {:?}", curve);
        
        let keypair = crypto.ecdsa_generate_keypair(Some(curve)).unwrap();
        let message = b"ECDSA test message for signature";
        
        // Sign message
        let signature = crypto.ecdsa_sign(&keypair.private_key, message);
        assert!(signature.is_ok(), "ECDSA signing failed for curve {:?}", curve);
        
        let sig = signature.unwrap();
        assert_eq!(sig.curve, curve);
        
        // Verify signature
        let is_valid = crypto.ecdsa_verify(&keypair.public_key, message, &sig);
        assert!(is_valid.is_ok(), "ECDSA verification failed for curve {:?}", curve);
        assert!(is_valid.unwrap(), "ECDSA signature should be valid for curve {:?}", curve);
        
        // Test with tampered message
        let tampered_message = b"Tampered ECDSA message";
        let is_invalid = crypto.ecdsa_verify(&keypair.public_key, tampered_message, &sig);
        assert!(is_invalid.is_ok());
        assert!(!is_invalid.unwrap(), "ECDSA signature should be invalid for tampered message");
    }
}

/// Test ECDSA cross-curve verification failure
#[test]
fn test_ecdsa_cross_curve_verification() {
    let crypto = AsymmetricCrypto::new();
    
    let p256_keypair = crypto.ecdsa_generate_keypair(Some(EcCurve::P256)).unwrap();
    let secp256k1_keypair = crypto.ecdsa_generate_keypair(Some(EcCurve::Secp256k1)).unwrap();
    
    let message = b"Cross-curve test message";
    
    // Sign with P256 key
    let p256_signature = crypto.ecdsa_sign(&p256_keypair.private_key, message).unwrap();
    
    // Try to verify with secp256k1 public key (should fail)
    let is_valid = crypto.ecdsa_verify(&secp256k1_keypair.public_key, message, &p256_signature);
    assert!(is_valid.is_ok());
    assert!(!is_valid.unwrap(), "Cross-curve verification should fail");
}

/// Test ECDH key exchange
#[test]
fn test_ecdh_key_exchange() {
    let crypto = AsymmetricCrypto::new();
    
    let curves = vec![EcCurve::P256, EcCurve::Secp256k1];
    
    for curve in curves {
        println!("Testing ECDH key exchange for curve: {:?}", curve);
        
        // Generate two keypairs (Alice and Bob)
        let alice_keypair = crypto.ecdh_generate_keypair(Some(curve)).unwrap();
        let bob_keypair = crypto.ecdh_generate_keypair(Some(curve)).unwrap();
        
        // Perform key exchange
        let alice_shared = crypto.ecdh_exchange(&alice_keypair.private_key, &bob_keypair.public_key);
        let bob_shared = crypto.ecdh_exchange(&bob_keypair.private_key, &alice_keypair.public_key);
        
        assert!(alice_shared.is_ok(), "Alice's ECDH exchange failed for curve {:?}", curve);
        assert!(bob_shared.is_ok(), "Bob's ECDH exchange failed for curve {:?}", curve);
        
        let alice_secret = alice_shared.unwrap();
        let bob_secret = bob_shared.unwrap();
        
        assert_eq!(alice_secret, bob_secret, "Shared secrets should match for curve {:?}", curve);
        assert!(!alice_secret.is_empty(), "Shared secret should not be empty");
    }
}

/// Test ECDH cross-curve exchange failure
#[test]
fn test_ecdh_cross_curve_exchange() {
    let crypto = AsymmetricCrypto::new();
    
    let alice_keypair = crypto.ecdh_generate_keypair(Some(EcCurve::P256)).unwrap();
    let bob_keypair = crypto.ecdh_generate_keypair(Some(EcCurve::Secp256k1)).unwrap();
    
    // Try to exchange keys with different curves (should fail)
    let result = crypto.ecdh_exchange(&alice_keypair.private_key, &bob_keypair.public_key);
    assert!(result.is_err(), "Cross-curve ECDH exchange should fail");
    
    if let Err(AsymmetricError::KeyExchangeFailed(msg)) = result {
        assert!(msg.contains("Curve mismatch"));
    } else {
        panic!("Expected KeyExchangeFailed error");
    }
}

/// Test X25519 key generation and exchange
#[test]
fn test_x25519_operations() {
    let crypto = AsymmetricCrypto::new();
    
    // Generate keypairs
    let alice_keypair = crypto.x25519_generate_keypair().unwrap();
    let bob_keypair = crypto.x25519_generate_keypair().unwrap();
    
    assert_eq!(alice_keypair.public_key.bytes.len(), X25519_KEY_SIZE);
    assert_eq!(bob_keypair.public_key.bytes.len(), X25519_KEY_SIZE);
    
    // Perform key exchange
    let alice_shared = crypto.x25519_exchange(&alice_keypair.private_key, &bob_keypair.public_key);
    let bob_shared = crypto.x25519_exchange(&bob_keypair.private_key, &alice_keypair.public_key);
    
    assert!(alice_shared.is_ok(), "Alice's X25519 exchange failed");
    assert!(bob_shared.is_ok(), "Bob's X25519 exchange failed");
    
    let alice_secret = alice_shared.unwrap();
    let bob_secret = bob_shared.unwrap();
    
    assert_eq!(alice_secret, bob_secret, "X25519 shared secrets should match");
    assert_eq!(alice_secret.len(), X25519_KEY_SIZE);
}

/// Test Ed25519 key generation, signing, and verification
#[test]
fn test_ed25519_operations() {
    let crypto = AsymmetricCrypto::new();
    
    // Generate keypair
    let keypair = crypto.ed25519_generate_keypair().unwrap();
    
    let test_message = b"Ed25519 signature test message";
    
    // Sign message
    let signature = crypto.ed25519_sign(&keypair.private_key, test_message);
    assert!(signature.is_ok(), "Ed25519 signing failed");
    
    let sig = signature.unwrap();
    
    // Verify signature
    let is_valid = crypto.ed25519_verify(&keypair.public_key, test_message, &sig);
    assert!(is_valid.is_ok(), "Ed25519 verification failed");
    assert!(is_valid.unwrap(), "Ed25519 signature should be valid");
    
    // Test with tampered message
    let tampered_message = b"Tampered Ed25519 message";
    let is_invalid = crypto.ed25519_verify(&keypair.public_key, tampered_message, &sig);
    assert!(is_invalid.is_ok());
    assert!(!is_invalid.unwrap(), "Ed25519 signature should be invalid for tampered message");
}

/// Test multiple Ed25519 signatures with same key
#[test]
fn test_ed25519_multiple_signatures() {
    let crypto = AsymmetricCrypto::new();
    let keypair = crypto.ed25519_generate_keypair().unwrap();
    
    let messages = vec![
        b"First message".as_slice(),
        b"Second message".as_slice(),
        b"Third message with different content".as_slice(),
    ];
    
    let mut signatures = Vec::new();
    
    // Sign multiple messages
    for (i, message) in messages.iter().enumerate() {
        let signature = crypto.ed25519_sign(&keypair.private_key, message).unwrap();
        signatures.push(signature);
        
        println!("Signed message {}: {:?}", i + 1, std::str::from_utf8(message).unwrap());
    }
    
    // Verify all signatures
    for (i, (message, signature)) in messages.iter().zip(signatures.iter()).enumerate() {
        let is_valid = crypto.ed25519_verify(&keypair.public_key, message, signature).unwrap();
        assert!(is_valid, "Message {} signature should be valid", i + 1);
    }
    
    // Cross-verify (should fail)
    let is_invalid = crypto.ed25519_verify(&keypair.public_key, messages[0], &signatures[1]).unwrap();
    assert!(!is_invalid, "Cross-verification should fail");
}

/// Test asymmetric crypto configuration
#[test]
fn test_asymmetric_config() {
    let default_config = AsymmetricConfig::default();
    assert_eq!(default_config.default_rsa_key_size, RSA_4096_BITS);
    assert_eq!(default_config.default_rsa_padding, RsaPadding::OaepSha256);
    assert_eq!(default_config.default_ec_curve, EcCurve::P256);
    assert!(default_config.hardware_acceleration);
    assert!(default_config.constant_time_operations);
    assert!(default_config.secure_key_generation);
    
    let custom_config = AsymmetricConfig {
        default_rsa_key_size: RSA_2048_BITS,
        default_rsa_padding: RsaPadding::Pss,
        default_ec_curve: EcCurve::Secp256k1,
        hardware_acceleration: false,
        constant_time_operations: true,
        secure_key_generation: true,
    };
    
    let crypto = AsymmetricCrypto::with_config(custom_config.clone());
    assert_eq!(crypto.config.default_rsa_key_size, RSA_2048_BITS);
    assert_eq!(crypto.config.default_ec_curve, EcCurve::Secp256k1);
}

/// Test elliptic curve properties
#[test]
fn test_ec_curve_properties() {
    let curves = vec![
        (EcCurve::P256, "P-256", 32, 128),
        (EcCurve::P384, "P-384", 48, 192),
        (EcCurve::P521, "P-521", 66, 256),
        (EcCurve::Secp256k1, "secp256k1", 32, 128),
    ];
    
    for (curve, expected_name, expected_key_size, expected_security_level) in curves {
        assert_eq!(curve.name(), expected_name);
        assert_eq!(curve.key_size(), expected_key_size);
        assert_eq!(curve.security_level(), expected_security_level);
    }
}

/// Test RSA padding scheme properties
#[test]
fn test_rsa_padding_properties() {
    let paddings = vec![
        (RsaPadding::Pkcs1v15, "PKCS1v15"),
        (RsaPadding::OaepSha256, "OAEP-SHA256"),
        (RsaPadding::OaepSha384, "OAEP-SHA384"),
        (RsaPadding::OaepSha512, "OAEP-SHA512"),
        (RsaPadding::Pss, "PSS"),
    ];
    
    for (padding, expected_name) in paddings {
        assert_eq!(padding.name(), expected_name);
    }
}

/// Test error types and messages
#[test]
fn test_error_types() {
    let errors = vec![
        (AsymmetricError::InvalidKeySize(1024), "Invalid key size: 1024"),
        (AsymmetricError::InvalidCurve("unknown".to_string()), "Invalid elliptic curve: unknown"),
        (AsymmetricError::InvalidPadding("bad".to_string()), "Invalid padding scheme: bad"),
        (AsymmetricError::KeyGenerationFailed("failed".to_string()), "Key generation failed: failed"),
        (AsymmetricError::EncryptionFailed("failed".to_string()), "Encryption failed: failed"),
        (AsymmetricError::DecryptionFailed("failed".to_string()), "Decryption failed: failed"),
        (AsymmetricError::SigningFailed("failed".to_string()), "Signing failed: failed"),
        (AsymmetricError::VerificationFailed("failed".to_string()), "Verification failed: failed"),
        (AsymmetricError::KeyExchangeFailed("failed".to_string()), "Key exchange failed: failed"),
        (AsymmetricError::InvalidSignature, "Invalid signature"),
        (AsymmetricError::InvalidPublicKey, "Invalid public key"),
        (AsymmetricError::InvalidPrivateKey, "Invalid private key"),
        (AsymmetricError::UnsupportedOperation("test".to_string()), "Unsupported operation: test"),
        (AsymmetricError::InsufficientEntropy, "Insufficient entropy for key generation"),
        (AsymmetricError::Internal("internal".to_string()), "Internal error: internal"),
    ];
    
    for (error, expected_message) in errors {
        assert_eq!(error.to_string(), expected_message);
    }
}

/// Test public API functions
#[test]
fn test_public_api_functions() {
    use cursed::stdlib::value::Value;
    
    // Test RSA key generation API
    let result = rsa_generate_keypair(vec![]);
    assert!(result.is_ok());
    
    let result = rsa_generate_keypair(vec![Value::Number(2048.0)]);
    assert!(result.is_ok());
    
    // Test ECDSA key generation API
    let result = ecdsa_generate_keypair(vec![]);
    assert!(result.is_ok());
    
    let result = ecdsa_generate_keypair(vec![Value::String("P-256".to_string())]);
    assert!(result.is_ok());
    
    let result = ecdsa_generate_keypair(vec![Value::String("secp256k1".to_string())]);
    assert!(result.is_ok());
    
    // Test X25519 key generation API
    let result = x25519_generate_keypair(vec![]);
    assert!(result.is_ok());
    
    // Test Ed25519 key generation API
    let result = ed25519_generate_keypair(vec![]);
    assert!(result.is_ok());
}

/// Test large message handling
#[test]
fn test_large_message_handling() {
    let crypto = AsymmetricCrypto::new();
    
    // Test Ed25519 with large message
    let keypair = crypto.ed25519_generate_keypair().unwrap();
    let large_message = vec![0x42u8; 10000]; // 10KB message
    
    let signature = crypto.ed25519_sign(&keypair.private_key, &large_message).unwrap();
    let is_valid = crypto.ed25519_verify(&keypair.public_key, &large_message, &signature).unwrap();
    
    assert!(is_valid, "Ed25519 should handle large messages");
    
    // Test ECDSA with large message
    let ecdsa_keypair = crypto.ecdsa_generate_keypair(Some(EcCurve::P256)).unwrap();
    let ecdsa_signature = crypto.ecdsa_sign(&ecdsa_keypair.private_key, &large_message).unwrap();
    let ecdsa_valid = crypto.ecdsa_verify(&ecdsa_keypair.public_key, &large_message, &ecdsa_signature).unwrap();
    
    assert!(ecdsa_valid, "ECDSA should handle large messages");
}

/// Test empty message handling
#[test]
fn test_empty_message_handling() {
    let crypto = AsymmetricCrypto::new();
    
    // Test Ed25519 with empty message
    let keypair = crypto.ed25519_generate_keypair().unwrap();
    let empty_message = b"";
    
    let signature = crypto.ed25519_sign(&keypair.private_key, empty_message).unwrap();
    let is_valid = crypto.ed25519_verify(&keypair.public_key, empty_message, &signature).unwrap();
    
    assert!(is_valid, "Ed25519 should handle empty messages");
    
    // Test ECDSA with empty message
    let ecdsa_keypair = crypto.ecdsa_generate_keypair(Some(EcCurve::P256)).unwrap();
    let ecdsa_signature = crypto.ecdsa_sign(&ecdsa_keypair.private_key, empty_message).unwrap();
    let ecdsa_valid = crypto.ecdsa_verify(&ecdsa_keypair.public_key, empty_message, &ecdsa_signature).unwrap();
    
    assert!(ecdsa_valid, "ECDSA should handle empty messages");
}

/// Performance benchmark for key generation
#[test]
fn test_key_generation_performance() {
    let crypto = AsymmetricCrypto::new();
    
    let start = std::time::Instant::now();
    
    // Generate multiple keys to test performance
    for _ in 0..10 {
        let _rsa_keypair = crypto.rsa_generate_keypair(Some(RSA_2048_BITS)).unwrap();
        let _ecdsa_keypair = crypto.ecdsa_generate_keypair(Some(EcCurve::P256)).unwrap();
        let _x25519_keypair = crypto.x25519_generate_keypair().unwrap();
        let _ed25519_keypair = crypto.ed25519_generate_keypair().unwrap();
    }
    
    let duration = start.elapsed();
    println!("Generated 40 keypairs in {:?}", duration);
    
    // Should complete within reasonable time (adjust as needed)
    assert!(duration.as_secs() < 30, "Key generation should be reasonably fast");
}
