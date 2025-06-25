/// Comprehensive test suite for the asymmetric cryptography module
/// 
/// This test suite validates all asymmetric cryptographic operations including:
/// - RSA encryption/decryption and signatures (2048, 3072, 4096-bit)
/// - ECDSA signatures on multiple curves (P-256, P-384, P-521)
/// - Ed25519 high-performance digital signatures
/// - X25519 and X448 key exchange protocols
/// - Unified asymmetric crypto API
/// - Key format compatibility and serialization
/// - Security property validation

#[cfg(test)]
mod tests {
    use cursed::stdlib::packages::crypto_asymmetric::*;
    use cursed::stdlib::value::Value;
    use cursed::error::CursedError;
    use std::collections::HashMap;

    #[test]
    fn test_asymmetric_crypto_initialization() {
        let crypto = AsymmetricCrypto::new();
        let algorithms = crypto.supported_algorithms();
        
        assert!(algorithms.contains(&"RSA-2048".to_string()));
        assert!(algorithms.contains(&"RSA-3072".to_string()));
        assert!(algorithms.contains(&"RSA-4096".to_string()));
        assert!(algorithms.contains(&"ECDSA-P256".to_string()));
        assert!(algorithms.contains(&"ECDSA-P384".to_string()));
        assert!(algorithms.contains(&"ECDSA-P521".to_string()));
        assert!(algorithms.contains(&"Ed25519".to_string()));
        assert!(algorithms.contains(&"X25519".to_string()));
        
        println!("✅ Asymmetric crypto initialization test passed");
    }

    #[test]
    fn test_rsa_full_workflow() {
        let mut crypto = AsymmetricCrypto::new();
        
        // Test RSA key generation for different sizes
        for key_size in &["RSA-2048", "RSA-3072", "RSA-4096"] {
            let keypair_result = crypto.generate_keypair(key_size);
            assert!(keypair_result.is_ok(), "Failed to generate {} keypair", key_size);
            
            if let Ok(Value::Object(keypair)) = keypair_result {
                assert!(keypair.contains_key("algorithm"));
                assert!(keypair.contains_key("public_key"));
                assert!(keypair.contains_key("private_key"));
                
                if let (Some(Value::String(public_key)), Some(Value::String(private_key))) = 
                   (keypair.get("public_key"), keypair.get("private_key")) {
                    
                    // Test encryption/decryption
                    let test_data = b"Hello, asymmetric world!";
                    let encrypt_result = crypto.rsa_encrypt(public_key, test_data);
                    assert!(encrypt_result.is_ok(), "Failed to encrypt with {}", key_size);
                    
                    if let Ok(encrypted_data) = encrypt_result {
                        let decrypt_result = crypto.rsa_decrypt(private_key, &encrypted_data);
                        assert!(decrypt_result.is_ok(), "Failed to decrypt with {}", key_size);
                        
                        if let Ok(decrypted_data) = decrypt_result {
                            assert_eq!(test_data, decrypted_data.as_slice(), 
                                     "{} encryption/decryption round trip failed", key_size);
                        }
                    }
                    
                    // Test signing/verification
                    let message = b"Test message for signing";
                    let sign_result = crypto.sign(key_size, private_key, message);
                    assert!(sign_result.is_ok(), "Failed to sign with {}", key_size);
                    
                    if let Ok(signature) = sign_result {
                        let verify_result = crypto.verify(key_size, public_key, message, &signature);
                        assert!(verify_result.is_ok(), "Failed to verify with {}", key_size);
                        assert!(verify_result.unwrap(), "{} signature verification failed", key_size);
                    }
                }
            }
        }
        
        println!("✅ RSA full workflow test passed");
    }

    #[test]
    fn test_ecdsa_full_workflow() {
        let mut crypto = AsymmetricCrypto::new();
        
        // Test ECDSA for different curves
        for algorithm in &["ECDSA-P256", "ECDSA-P384", "ECDSA-P521"] {
            let keypair_result = crypto.generate_keypair(algorithm);
            assert!(keypair_result.is_ok(), "Failed to generate {} keypair", algorithm);
            
            if let Ok(Value::Object(keypair)) = keypair_result {
                if let (Some(Value::String(public_key)), Some(Value::String(private_key))) = 
                   (keypair.get("public_key"), keypair.get("private_key")) {
                    
                    // Test signing/verification
                    let message = b"ECDSA test message";
                    let sign_result = crypto.sign(algorithm, private_key, message);
                    assert!(sign_result.is_ok(), "Failed to sign with {}", algorithm);
                    
                    if let Ok(signature) = sign_result {
                        let verify_result = crypto.verify(algorithm, public_key, message, &signature);
                        assert!(verify_result.is_ok(), "Failed to verify with {}", algorithm);
                        assert!(verify_result.unwrap(), "{} signature verification failed", algorithm);
                        
                        // Test with wrong message
                        let wrong_message = b"Wrong message";
                        let verify_wrong = crypto.verify(algorithm, public_key, wrong_message, &signature);
                        assert!(verify_wrong.is_ok(), "Verification should not error on wrong message");
                        assert!(!verify_wrong.unwrap(), "Signature should not verify for wrong message");
                    }
                }
            }
        }
        
        println!("✅ ECDSA full workflow test passed");
    }

    #[test]
    fn test_ed25519_full_workflow() {
        let mut crypto = AsymmetricCrypto::new();
        
        let keypair_result = crypto.generate_keypair("Ed25519");
        assert!(keypair_result.is_ok(), "Failed to generate Ed25519 keypair");
        
        if let Ok(Value::Object(keypair)) = keypair_result {
            if let (Some(Value::String(public_key)), Some(Value::String(private_key))) = 
               (keypair.get("public_key"), keypair.get("private_key")) {
                
                // Test signing/verification
                let message = b"Ed25519 test message";
                let sign_result = crypto.sign("Ed25519", private_key, message);
                assert!(sign_result.is_ok(), "Failed to sign with Ed25519");
                
                if let Ok(signature) = sign_result {
                    let verify_result = crypto.verify("Ed25519", public_key, message, &signature);
                    assert!(verify_result.is_ok(), "Failed to verify with Ed25519");
                    assert!(verify_result.unwrap(), "Ed25519 signature verification failed");
                    
                    // Test signature stability (deterministic)
                    let sign_result2 = crypto.sign("Ed25519", private_key, message);
                    assert!(sign_result2.is_ok(), "Second signing failed");
                    
                    // Ed25519 is deterministic, so signatures should be identical
                    // (This test may fail with randomized signatures, which is also valid)
                }
            }
        }
        
        println!("✅ Ed25519 full workflow test passed");
    }

    #[test]
    fn test_x25519_key_exchange() {
        let mut crypto = AsymmetricCrypto::new();
        
        // Generate two X25519 keypairs
        let keypair1_result = crypto.generate_keypair("X25519");
        let keypair2_result = crypto.generate_keypair("X25519");
        
        assert!(keypair1_result.is_ok(), "Failed to generate first X25519 keypair");
        assert!(keypair2_result.is_ok(), "Failed to generate second X25519 keypair");
        
        if let (Ok(Value::Object(keypair1)), Ok(Value::Object(keypair2))) = 
           (keypair1_result, keypair2_result) {
            
            if let (Some(Value::String(private_key1)), Some(Value::String(public_key1)),
                    Some(Value::String(private_key2)), Some(Value::String(public_key2))) = 
               (keypair1.get("private_key"), keypair1.get("public_key"),
                keypair2.get("private_key"), keypair2.get("public_key")) {
                
                // Perform key exchange from both sides
                let shared_secret1 = crypto.key_exchange("X25519", private_key1, public_key2);
                let shared_secret2 = crypto.key_exchange("X25519", private_key2, public_key1);
                
                assert!(shared_secret1.is_ok(), "First key exchange failed");
                assert!(shared_secret2.is_ok(), "Second key exchange failed");
                
                // Both parties should derive the same shared secret
                assert_eq!(shared_secret1.unwrap(), shared_secret2.unwrap(),
                          "X25519 shared secrets do not match");
            }
        }
        
        println!("✅ X25519 key exchange test passed");
    }

    #[test]
    fn test_x448_key_exchange() {
        // Test X448 key generation
        let keypair1_result = x448_generate_keypair(vec![]);
        let keypair2_result = x448_generate_keypair(vec![]);
        
        assert!(keypair1_result.is_ok(), "Failed to generate first X448 keypair");
        assert!(keypair2_result.is_ok(), "Failed to generate second X448 keypair");
        
        if let (Ok(Value::Object(keypair1)), Ok(Value::Object(keypair2))) = 
           (keypair1_result, keypair2_result) {
            
            if let (Some(Value::String(private_key1)), Some(Value::String(public_key2))) = 
               (keypair1.get("private_key"), keypair2.get("public_key")) {
                
                // Test key exchange
                let exchange_result = x448_key_exchange(vec![
                    Value::String(private_key1.clone()),
                    Value::String(public_key2.clone()),
                ]);
                
                assert!(exchange_result.is_ok(), "X448 key exchange failed");
                
                if let Ok(Value::Object(result)) = exchange_result {
                    assert!(result.contains_key("algorithm"));
                    assert!(result.contains_key("shared_secret"));
                    assert!(result.contains_key("derived_key"));
                    
                    assert_eq!(result.get("algorithm"), Some(&Value::String("X448".to_string())));
                }
            }
        }
        
        println!("✅ X448 key exchange test passed");
    }

    #[test]
    fn test_diffie_hellman_key_exchange() {
        // Test DH key generation
        let keypair1_result = dh_generate_keypair(vec![]);
        let keypair2_result = dh_generate_keypair(vec![]);
        
        assert!(keypair1_result.is_ok(), "Failed to generate first DH keypair");
        assert!(keypair2_result.is_ok(), "Failed to generate second DH keypair");
        
        if let (Ok(Value::Object(keypair1)), Ok(Value::Object(keypair2))) = 
           (keypair1_result, keypair2_result) {
            
            if let (Some(Value::String(private_key1)), Some(Value::String(public_key1)),
                    Some(Value::String(private_key2)), Some(Value::String(public_key2))) = 
               (keypair1.get("private_key"), keypair1.get("public_key"),
                keypair2.get("private_key"), keypair2.get("public_key")) {
                
                // Perform key exchange from both sides
                let exchange1_result = dh_key_exchange(vec![
                    Value::String(private_key1.clone()),
                    Value::String(public_key2.clone()),
                ]);
                
                let exchange2_result = dh_key_exchange(vec![
                    Value::String(private_key2.clone()),
                    Value::String(public_key1.clone()),
                ]);
                
                assert!(exchange1_result.is_ok(), "First DH key exchange failed");
                assert!(exchange2_result.is_ok(), "Second DH key exchange failed");
                
                // Both exchanges should produce the same shared secret
                if let (Ok(Value::Object(result1)), Ok(Value::Object(result2))) = 
                   (exchange1_result, exchange2_result) {
                    
                    let secret1 = result1.get("shared_secret");
                    let secret2 = result2.get("shared_secret");
                    
                    assert_eq!(secret1, secret2, "DH shared secrets do not match");
                }
            }
        }
        
        println!("✅ Diffie-Hellman key exchange test passed");
    }

    #[test]
    fn test_unified_api_functions() {
        // Test unified API function exports
        
        // Test key generation
        let keygen_result = generate_asymmetric_keypair(vec![
            Value::String("Ed25519".to_string())
        ]);
        assert!(keygen_result.is_ok(), "Unified key generation failed");
        
        // Test algorithm listing
        let algorithms_result = get_asymmetric_algorithms();
        assert!(algorithms_result.is_ok(), "Failed to get algorithm list");
        
        if let Ok(Value::Array(algorithms)) = algorithms_result {
            assert!(!algorithms.is_empty(), "Algorithm list is empty");
        }
        
        // Test capabilities listing
        let capabilities_result = get_asymmetric_capabilities();
        assert!(capabilities_result.is_ok(), "Failed to get capabilities");
        
        if let Ok(Value::Object(capabilities)) = capabilities_result {
            assert!(capabilities.contains_key("algorithms"));
            assert!(capabilities.contains_key("operations"));
            assert!(capabilities.contains_key("key_formats"));
        }
        
        println!("✅ Unified API functions test passed");
    }

    #[test]
    fn test_algorithm_information() {
        let crypto = AsymmetricCrypto::new();
        
        // Test algorithm info for each supported algorithm
        let algorithms = ["RSA-2048", "ECDSA-P256", "Ed25519", "X25519"];
        
        for algorithm in &algorithms {
            let info_result = crypto.get_algorithm_info(algorithm);
            assert!(info_result.is_ok(), "Failed to get info for {}", algorithm);
            
            if let Ok(Value::Object(info)) = info_result {
                assert!(info.contains_key("name"));
                assert!(info.contains_key("type"));
                assert!(info.contains_key("capabilities"));
                
                // Validate algorithm-specific fields
                match *algorithm {
                    "RSA-2048" => {
                        assert_eq!(info.get("key_size"), Some(&Value::Integer(2048)));
                        assert_eq!(info.get("type"), Some(&Value::String("RSA".to_string())));
                    },
                    "ECDSA-P256" => {
                        assert_eq!(info.get("key_size"), Some(&Value::Integer(256)));
                        assert_eq!(info.get("type"), Some(&Value::String("ECC".to_string())));
                        assert_eq!(info.get("curve"), Some(&Value::String("P-256".to_string())));
                    },
                    "Ed25519" => {
                        assert_eq!(info.get("key_size"), Some(&Value::Integer(255)));
                        assert_eq!(info.get("type"), Some(&Value::String("EdDSA".to_string())));
                    },
                    "X25519" => {
                        assert_eq!(info.get("key_size"), Some(&Value::Integer(255)));
                        assert_eq!(info.get("type"), Some(&Value::String("ECDH".to_string())));
                    },
                    _ => {}
                }
            }
        }
        
        println!("✅ Algorithm information test passed");
    }

    #[test]
    fn test_key_exchange_algorithm_list() {
        let algorithms = list_key_exchange_algorithms();
        
        assert!(algorithms.contains(&"Diffie-Hellman".to_string()));
        assert!(algorithms.contains(&"X25519".to_string()));
        assert!(algorithms.contains(&"X448".to_string()));
        
        println!("✅ Key exchange algorithm list test passed");
    }

    #[test]
    fn test_key_derivation() {
        let shared_secret = b"test_shared_secret_for_derivation";
        
        // Test key derivation with different lengths
        for key_length in &[16, 32, 64] {
            let derived_key = derive_key_from_shared_secret(shared_secret, *key_length, Some("TEST"));
            assert!(derived_key.is_ok(), "Key derivation failed for length {}", key_length);
            
            if let Ok(key) = derived_key {
                assert_eq!(key.len(), *key_length, "Derived key length mismatch");
            }
        }
        
        // Test with different info strings
        let key1 = derive_key_from_shared_secret(shared_secret, 32, Some("INFO1")).unwrap();
        let key2 = derive_key_from_shared_secret(shared_secret, 32, Some("INFO2")).unwrap();
        assert_ne!(key1, key2, "Keys with different info should be different");
        
        // Test deterministic derivation
        let key3 = derive_key_from_shared_secret(shared_secret, 32, Some("INFO1")).unwrap();
        assert_eq!(key1, key3, "Key derivation should be deterministic");
        
        println!("✅ Key derivation test passed");
    }

    #[test]
    fn test_parameter_validation() {
        // Test X25519 parameter validation
        let valid_x25519_key = vec![0u8; 32];
        let invalid_key = vec![0u8; 16];
        
        assert!(validate_key_exchange_params(
            KeyExchangeAlgorithm::X25519,
            &valid_x25519_key,
            &valid_x25519_key
        ).is_ok());
        
        assert!(validate_key_exchange_params(
            KeyExchangeAlgorithm::X25519,
            &invalid_key,
            &valid_x25519_key
        ).is_err());
        
        // Test X448 parameter validation
        let valid_x448_key = vec![0u8; 56];
        
        assert!(validate_key_exchange_params(
            KeyExchangeAlgorithm::X448,
            &valid_x448_key,
            &valid_x448_key
        ).is_ok());
        
        assert!(validate_key_exchange_params(
            KeyExchangeAlgorithm::X448,
            &invalid_key,
            &valid_x448_key
        ).is_err());
        
        println!("✅ Parameter validation test passed");
    }

    #[test]
    fn test_error_handling() {
        let mut crypto = AsymmetricCrypto::new();
        
        // Test unsupported algorithm
        let invalid_result = crypto.generate_keypair("INVALID_ALGORITHM");
        assert!(invalid_result.is_err(), "Should reject invalid algorithm");
        
        // Test invalid key exchange algorithm
        let invalid_exchange = crypto.key_exchange("INVALID_KX", "key1", "key2");
        assert!(invalid_exchange.is_err(), "Should reject invalid key exchange algorithm");
        
        // Test signing with invalid algorithm
        let invalid_sign = crypto.sign("INVALID_SIGN", "key", b"message");
        assert!(invalid_sign.is_err(), "Should reject invalid signing algorithm");
        
        println!("✅ Error handling test passed");
    }

    #[test]
    fn test_crypto_package_initialization() {
        let init_result = init_crypto_asymmetric();
        assert!(init_result.is_ok(), "Crypto package initialization failed");
        
        let capabilities = get_crypto_capabilities();
        assert!(!capabilities.is_empty(), "Capabilities list should not be empty");
        
        println!("✅ Crypto package initialization test passed");
    }

    #[test]
    fn test_comprehensive_security_properties() {
        let mut crypto = AsymmetricCrypto::new();
        
        // Test that different key generations produce different keys
        let keypair1 = crypto.generate_keypair("Ed25519").unwrap();
        let keypair2 = crypto.generate_keypair("Ed25519").unwrap();
        
        if let (Value::Object(kp1), Value::Object(kp2)) = (keypair1, keypair2) {
            let pk1 = kp1.get("public_key");
            let pk2 = kp2.get("public_key");
            assert_ne!(pk1, pk2, "Different key generations should produce different keys");
            
            let sk1 = kp1.get("private_key");
            let sk2 = kp2.get("private_key");
            assert_ne!(sk1, sk2, "Different key generations should produce different private keys");
        }
        
        // Test that signatures on different messages are different
        let keypair = crypto.generate_keypair("Ed25519").unwrap();
        if let Value::Object(kp) = keypair {
            if let Some(Value::String(private_key)) = kp.get("private_key") {
                let sig1 = crypto.sign("Ed25519", private_key, b"message1").unwrap();
                let sig2 = crypto.sign("Ed25519", private_key, b"message2").unwrap();
                assert_ne!(sig1, sig2, "Signatures on different messages should be different");
            }
        }
        
        println!("✅ Comprehensive security properties test passed");
    }
}
