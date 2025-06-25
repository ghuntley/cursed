/// fr fr Comprehensive tests for X448 key exchange implementation
/// 
/// Tests the complete X448 functionality including key generation, validation,
/// key exchange, and integration with the crypto ecosystem.

use cursed::stdlib::crypto::x448_implementation::*;
use cursed::stdlib::value::Value;
use cursed::error::CursedError;

#[test]
fn test_x448_key_generation() {
    let engine = X448Engine::new();
    let keypair = engine.generate_keypair().unwrap();
    
    // Verify key sizes
    assert_eq!(keypair.private_key.as_bytes().len(), X448_KEY_SIZE);
    assert_eq!(keypair.public_key.as_bytes().len(), X448_KEY_SIZE);
    
    // Ensure keys are not all zeros
    assert_ne!(keypair.private_key.as_bytes(), &[0u8; X448_KEY_SIZE]);
    assert_ne!(keypair.public_key.as_bytes(), &[0u8; X448_KEY_SIZE]);
    
    // Verify public key can be derived from private key
    let derived_public = keypair.private_key.to_public_key().unwrap();
    assert_eq!(derived_public.as_bytes(), keypair.public_key.as_bytes());
}

#[test]
fn test_x448_key_generation_randomness() {
    let engine = X448Engine::new();
    
    // Generate multiple key pairs and ensure they're different
    let mut keypairs = Vec::new();
    for _ in 0..10 {
        keypairs.push(engine.generate_keypair().unwrap());
    }
    
    // Check that all private keys are unique
    for i in 0..keypairs.len() {
        for j in (i + 1)..keypairs.len() {
            assert_ne!(keypairs[i].private_key.as_bytes(), keypairs[j].private_key.as_bytes());
            assert_ne!(keypairs[i].public_key.as_bytes(), keypairs[j].public_key.as_bytes());
        }
    }
}

#[test]
fn test_x448_key_exchange() {
    let engine = X448Engine::new();
    
    // Generate two key pairs
    let alice = engine.generate_keypair().unwrap();
    let bob = engine.generate_keypair().unwrap();
    
    // Perform key exchange from both sides
    let alice_shared = engine.key_exchange(&alice.private_key, &bob.public_key).unwrap();
    let bob_shared = engine.key_exchange(&bob.private_key, &alice.public_key).unwrap();
    
    // Shared secrets should match
    assert_eq!(alice_shared, bob_shared);
    
    // Shared secret should not be all zeros
    assert_ne!(alice_shared, [0u8; X448_KEY_SIZE]);
    
    // Different key pairs should produce different shared secrets
    let charlie = engine.generate_keypair().unwrap();
    let alice_charlie_shared = engine.key_exchange(&alice.private_key, &charlie.public_key).unwrap();
    assert_ne!(alice_shared, alice_charlie_shared);
}

#[test]
fn test_x448_key_exchange_symmetry() {
    let engine = X448Engine::new();
    
    // Test multiple rounds of key exchange symmetry
    for _ in 0..5 {
        let alice = engine.generate_keypair().unwrap();
        let bob = engine.generate_keypair().unwrap();
        
        let shared1 = engine.key_exchange(&alice.private_key, &bob.public_key).unwrap();
        let shared2 = engine.key_exchange(&bob.private_key, &alice.public_key).unwrap();
        
        assert_eq!(shared1, shared2);
    }
}

#[test]
fn test_x448_key_validation() {
    let engine = X448Engine::new();
    
    // Test valid key
    let keypair = engine.generate_keypair().unwrap();
    assert!(engine.validate_public_key(&keypair.public_key).is_ok());
    
    // Test invalid all-zero key
    let zero_key = X448PublicKey { bytes: [0u8; X448_KEY_SIZE] };
    assert!(engine.validate_public_key(&zero_key).is_err());
    
    // Test invalid all-one key
    let one_key = X448PublicKey { bytes: [0xFF; X448_KEY_SIZE] };
    assert!(engine.validate_public_key(&one_key).is_err());
    
    // Test that creation of invalid keys fails
    assert!(X448PublicKey::from_bytes([0u8; X448_KEY_SIZE]).is_err());
}

#[test]
fn test_x448_hex_conversion() {
    let engine = X448Engine::new();
    let keypair = engine.generate_keypair().unwrap();
    
    // Test private key hex conversion
    let private_hex = keypair.private_key.to_hex();
    assert_eq!(private_hex.len(), X448_KEY_SIZE * 2); // 2 hex chars per byte
    let private_restored = X448PrivateKey::from_hex(&private_hex).unwrap();
    assert_eq!(keypair.private_key.as_bytes(), private_restored.as_bytes());
    
    // Test public key hex conversion
    let public_hex = keypair.public_key.to_hex();
    assert_eq!(public_hex.len(), X448_KEY_SIZE * 2);
    let public_restored = X448PublicKey::from_hex(&public_hex).unwrap();
    assert_eq!(keypair.public_key.as_bytes(), public_restored.as_bytes());
    
    // Test round-trip conversion
    let original_private = keypair.private_key.as_bytes();
    let hex_private = hex::encode(original_private);
    let restored_private = X448PrivateKey::from_hex(&hex_private).unwrap();
    assert_eq!(original_private, restored_private.as_bytes());
}

#[test]
fn test_x448_key_exchange_error_cases() {
    let engine = X448Engine::new();
    let alice = engine.generate_keypair().unwrap();
    
    // Test with invalid public key (all zeros)
    let zero_key = X448PublicKey { bytes: [0u8; X448_KEY_SIZE] };
    assert!(engine.key_exchange(&alice.private_key, &zero_key).is_err());
}

#[test]
fn test_x448_key_derivation() {
    let engine = X448Engine::new();
    let shared_secret = [1u8; X448_KEY_SIZE]; // Mock shared secret
    
    // Test basic key derivation
    let derived_key = engine.derive_key(&shared_secret, b"test-info", 32).unwrap();
    assert_eq!(derived_key.len(), 32);
    
    // Different info should produce different keys
    let derived_key2 = engine.derive_key(&shared_secret, b"different-info", 32).unwrap();
    assert_ne!(derived_key, derived_key2);
    
    // Same info should produce same keys
    let derived_key3 = engine.derive_key(&shared_secret, b"test-info", 32).unwrap();
    assert_eq!(derived_key, derived_key3);
    
    // Different lengths should work
    let short_key = engine.derive_key(&shared_secret, b"test-info", 16).unwrap();
    let long_key = engine.derive_key(&shared_secret, b"test-info", 64).unwrap();
    assert_eq!(short_key.len(), 16);
    assert_eq!(long_key.len(), 64);
    
    // First 16 bytes of long key should not match short key (HKDF property)
    assert_ne!(short_key, &long_key[..16]);
}

#[test]
fn test_x448_api_functions() {
    // Test key generation API
    let gen_result = x448_generate_keypair(vec![]).unwrap();
    if let Value::Object(map) = gen_result {
        assert_eq!(map.get("algorithm"), Some(&Value::String("X448".to_string())));
        assert!(map.contains_key("private_key"));
        assert!(map.contains_key("public_key"));
        assert_eq!(map.get("key_size"), Some(&Value::Number(X448_KEY_SIZE as f64)));
        
        // Extract keys for further testing
        if let (Some(Value::String(private_hex)), Some(Value::String(public_hex))) = 
            (map.get("private_key"), map.get("public_key")) {
            
            // Verify they can be parsed
            assert!(X448PrivateKey::from_hex(private_hex).is_ok());
            assert!(X448PublicKey::from_hex(public_hex).is_ok());
            
            // Test key exchange API
            let exchange_args = vec![
                Value::String(private_hex.clone()),
                Value::String(public_hex.clone()),
            ];
            let exchange_result = x448_key_exchange(exchange_args).unwrap();
            
            if let Value::Object(exchange_map) = exchange_result {
                assert_eq!(exchange_map.get("algorithm"), Some(&Value::String("X448".to_string())));
                assert_eq!(exchange_map.get("success"), Some(&Value::Bool(true)));
                assert!(exchange_map.contains_key("shared_secret"));
                assert!(exchange_map.contains_key("derived_key"));
            } else {
                panic!("Expected object result from key exchange");
            }
            
            // Test public key derivation API
            let pubkey_args = vec![Value::String(private_hex.clone())];
            let pubkey_result = x448_get_public_key(pubkey_args).unwrap();
            
            if let Value::String(derived_public_hex) = pubkey_result {
                assert_eq!(&derived_public_hex, public_hex);
            } else {
                panic!("Expected string result from public key derivation");
            }
            
            // Test key validation API
            let validate_args = vec![Value::String(public_hex.clone())];
            let validate_result = x448_validate_public_key(validate_args).unwrap();
            assert_eq!(validate_result, Value::Bool(true));
        }
    } else {
        panic!("Expected object result from key generation");
    }
}

#[test]
fn test_x448_api_error_handling() {
    // Test insufficient arguments
    assert!(x448_key_exchange(vec![]).is_err());
    assert!(x448_key_exchange(vec![Value::String("test".to_string())]).is_err());
    
    assert!(x448_validate_public_key(vec![]).is_err());
    assert!(x448_get_public_key(vec![]).is_err());
    
    // Test wrong argument types
    assert!(x448_key_exchange(vec![
        Value::Number(123.0),
        Value::String("test".to_string()),
    ]).is_err());
    
    assert!(x448_validate_public_key(vec![Value::Number(123.0)]).is_err());
    assert!(x448_get_public_key(vec![Value::Number(123.0)]).is_err());
    
    // Test invalid hex strings
    assert!(x448_key_exchange(vec![
        Value::String("invalid_hex".to_string()),
        Value::String("another_invalid".to_string()),
    ]).is_err());
    
    assert!(x448_validate_public_key(vec![Value::String("invalid_hex".to_string())]).is_err());
    assert!(x448_get_public_key(vec![Value::String("invalid_hex".to_string())]).is_err());
    
    // Test wrong key sizes
    let short_hex = hex::encode(vec![0u8; 32]); // Too short
    let long_hex = hex::encode(vec![0u8; 64]); // Too long
    
    assert!(x448_key_exchange(vec![
        Value::String(short_hex.clone()),
        Value::String(long_hex.clone()),
    ]).is_err());
    
    assert!(x448_validate_public_key(vec![Value::String(short_hex)]).is_err());
    assert!(x448_get_public_key(vec![Value::String(long_hex)]).is_err());
}

#[test]
fn test_x448_key_sizes() {
    // Test that X448_KEY_SIZE is correct
    assert_eq!(X448_KEY_SIZE, 56);
    
    // Test key creation with correct size
    let valid_bytes = [0x01; X448_KEY_SIZE];
    let private_key = X448PrivateKey::from_bytes(valid_bytes);
    assert_eq!(private_key.as_bytes().len(), X448_KEY_SIZE);
    
    // Test key creation from slice
    let slice_key = X448PrivateKey::from_slice(&valid_bytes).unwrap();
    assert_eq!(slice_key.as_bytes(), &valid_bytes);
    
    // Test invalid sizes
    let short_slice = [0u8; 32];
    let long_slice = [0u8; 64];
    
    assert!(X448PrivateKey::from_slice(&short_slice).is_err());
    assert!(X448PrivateKey::from_slice(&long_slice).is_err());
    assert!(X448PublicKey::from_slice(&short_slice).is_err());
    assert!(X448PublicKey::from_slice(&long_slice).is_err());
}

#[test]
fn test_x448_private_key_clamping() {
    let engine = X448Engine::new();
    
    // Generate multiple keys and verify clamping
    for _ in 0..10 {
        let keypair = engine.generate_keypair().unwrap();
        let private_bytes = keypair.private_key.as_bytes();
        
        // Check X448 clamping rules
        // First byte: two least significant bits should be cleared
        assert_eq!(private_bytes[0] & 0x03, 0);
        
        // Last byte: most significant bit should be cleared, second MSB should be set
        assert_eq!(private_bytes[55] & 0x80, 0); // MSB cleared
        assert_eq!(private_bytes[55] & 0x40, 0x40); // Second MSB set
    }
}

#[test]
fn test_x448_public_key_validation_edge_cases() {
    // Test edge case values
    let test_cases = vec![
        [0x00; X448_KEY_SIZE], // All zeros (invalid)
        [0xFF; X448_KEY_SIZE], // All ones (invalid)
        {
            let mut bytes = [0x00; X448_KEY_SIZE];
            bytes[0] = 0x01;
            bytes
        }, // Valid: mostly zeros with one bit set
    ];
    
    for (i, bytes) in test_cases.iter().enumerate() {
        match i {
            0 | 1 => {
                // Should be invalid
                assert!(X448PublicKey::from_bytes(*bytes).is_err());
            }
            _ => {
                // Should be valid
                assert!(X448PublicKey::from_bytes(*bytes).is_ok());
            }
        }
    }
}

#[test]
fn test_x448_scalar_multiplication_basic() {
    // Test that scalar multiplication is implemented
    let private_key = [0x01; X448_KEY_SIZE];
    let public_key = [0x05; X448_KEY_SIZE]; // Base point is 5
    
    // This should not panic and should return a result
    let result = x448_scalar_mult(&private_key, &public_key);
    assert!(result.is_ok());
    
    let shared_secret = result.unwrap();
    assert_eq!(shared_secret.len(), X448_KEY_SIZE);
    
    // Result should not be all zeros (for these inputs)
    assert_ne!(shared_secret, [0u8; X448_KEY_SIZE]);
}

#[test]
fn test_x448_integration_with_crypto_module() {
    // Test that X448 is properly integrated with the main crypto module
    use cursed::stdlib::crypto::*;
    
    // These should be available from the main crypto module
    let engine = X448Engine::new();
    assert!(engine.generate_keypair().is_ok());
    
    // API functions should be available
    assert!(x448_generate_keypair(vec![]).is_ok());
}

#[test]
fn test_x448_performance_characteristics() {
    let engine = X448Engine::new();
    
    // Test that key generation is reasonably fast
    let start = std::time::Instant::now();
    for _ in 0..10 {
        let _ = engine.generate_keypair().unwrap();
    }
    let key_gen_time = start.elapsed();
    
    // Should generate 10 keys in reasonable time (< 1 second)
    assert!(key_gen_time.as_millis() < 1000);
    
    // Test that key exchange is reasonably fast
    let alice = engine.generate_keypair().unwrap();
    let bob = engine.generate_keypair().unwrap();
    
    let start = std::time::Instant::now();
    for _ in 0..10 {
        let _ = engine.key_exchange(&alice.private_key, &bob.public_key).unwrap();
    }
    let exchange_time = start.elapsed();
    
    // Should perform 10 exchanges in reasonable time (< 1 second)
    assert!(exchange_time.as_millis() < 1000);
}

#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[test]
    fn test_x448_weak_key_detection() {
        let engine = X448Engine::new();
        let alice = engine.generate_keypair().unwrap();
        
        // Test with known weak public keys
        let weak_keys = vec![
            [0x00; X448_KEY_SIZE], // All zeros
            [0xFF; X448_KEY_SIZE], // All ones
        ];
        
        for weak_key in weak_keys {
            let weak_public = X448PublicKey { bytes: weak_key };
            
            // Should fail validation
            assert!(engine.validate_public_key(&weak_public).is_err());
            
            // Should fail key exchange
            assert!(engine.key_exchange(&alice.private_key, &weak_public).is_err());
        }
    }
    
    #[test]
    fn test_x448_shared_secret_quality() {
        let engine = X448Engine::new();
        
        // Generate multiple key exchanges and check quality
        let mut shared_secrets = Vec::new();
        
        for _ in 0..20 {
            let alice = engine.generate_keypair().unwrap();
            let bob = engine.generate_keypair().unwrap();
            let shared = engine.key_exchange(&alice.private_key, &bob.public_key).unwrap();
            shared_secrets.push(shared);
        }
        
        // All shared secrets should be unique
        for i in 0..shared_secrets.len() {
            for j in (i + 1)..shared_secrets.len() {
                assert_ne!(shared_secrets[i], shared_secrets[j]);
            }
        }
        
        // Shared secrets should have good entropy (no obvious patterns)
        for secret in &shared_secrets {
            // Should not be all zeros
            assert_ne!(*secret, [0u8; X448_KEY_SIZE]);
            
            // Should not be all the same byte
            let first_byte = secret[0];
            assert!(!secret.iter().all(|&b| b == first_byte));
        }
    }
}
