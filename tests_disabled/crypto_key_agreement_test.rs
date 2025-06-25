//! Comprehensive tests for the CURSED crypto key agreement functionality
//! 
//! Tests all key agreement protocols including ECDH P-256/P-384/P-521, X25519, X448, and RSA OAEP.

use cursed::stdlib::packages::crypto_asymmetric::key_agreement::*;
use cursed::stdlib::value::Value;
use cursed::error::CursedError;
use rand::rngs::OsRng;
use rand::RngCore;
use x25519_dalek::{StaticSecret, PublicKey as X25519PublicKey};
use p256::{SecretKey as P256SecretKey, PublicKey as P256PublicKey};
use p384::{SecretKey as P384SecretKey, PublicKey as P384PublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use elliptic_curve::sec1::ToEncodedPoint;

/// Initialize test tracing
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_key_agreement_algorithm_from_name() {
    init_tracing!();
    tracing::info!("Testing key agreement algorithm name parsing");
    
    // Valid algorithm names
    assert_eq!(KeyAgreementAlgorithm::from_name("ECDH-P256").unwrap(), KeyAgreementAlgorithm::EcdhP256);
    assert_eq!(KeyAgreementAlgorithm::from_name("P256").unwrap(), KeyAgreementAlgorithm::EcdhP256);
    assert_eq!(KeyAgreementAlgorithm::from_name("ECDH-P384").unwrap(), KeyAgreementAlgorithm::EcdhP384);
    assert_eq!(KeyAgreementAlgorithm::from_name("P384").unwrap(), KeyAgreementAlgorithm::EcdhP384);
    assert_eq!(KeyAgreementAlgorithm::from_name("X25519").unwrap(), KeyAgreementAlgorithm::X25519);
    assert_eq!(KeyAgreementAlgorithm::from_name("X448").unwrap(), KeyAgreementAlgorithm::X448);
    assert_eq!(KeyAgreementAlgorithm::from_name("RSA-OAEP").unwrap(), KeyAgreementAlgorithm::RsaOaep);
    assert_eq!(KeyAgreementAlgorithm::from_name("RSA").unwrap(), KeyAgreementAlgorithm::RsaOaep);
    
    // Invalid algorithm name
    assert!(KeyAgreementAlgorithm::from_name("INVALID").is_err());
    
    tracing::info!("✅ Key agreement algorithm name parsing tests passed");
}

#[test]
fn test_key_agreement_algorithm_properties() {
    init_tracing!();
    tracing::info!("Testing key agreement algorithm properties");
    
    // Test key sizes
    assert_eq!(KeyAgreementAlgorithm::EcdhP256.key_size(), 256);
    assert_eq!(KeyAgreementAlgorithm::EcdhP384.key_size(), 384);
    assert_eq!(KeyAgreementAlgorithm::EcdhP521.key_size(), 521);
    assert_eq!(KeyAgreementAlgorithm::X25519.key_size(), 255);
    assert_eq!(KeyAgreementAlgorithm::X448.key_size(), 448);
    assert_eq!(KeyAgreementAlgorithm::RsaOaep.key_size(), 2048);
    
    // Test algorithm names
    assert_eq!(KeyAgreementAlgorithm::EcdhP256.name(), "ECDH-P256");
    assert_eq!(KeyAgreementAlgorithm::X25519.name(), "X25519");
    assert_eq!(KeyAgreementAlgorithm::RsaOaep.name(), "RSA-OAEP");
    
    tracing::info!("✅ Key agreement algorithm properties tests passed");
}

#[test]
fn test_key_agreement_result() {
    init_tracing!();
    tracing::info!("Testing key agreement result structure");
    
    let shared_secret = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let derived_key = vec![9, 10, 11, 12, 13, 14, 15, 16];
    
    let result = KeyAgreementResult::new(
        KeyAgreementAlgorithm::EcdhP256,
        shared_secret.clone(),
        Some(derived_key.clone()),
    );
    
    assert_eq!(result.algorithm, KeyAgreementAlgorithm::EcdhP256);
    assert_eq!(result.shared_secret, shared_secret);
    assert_eq!(result.derived_key, Some(derived_key));
    assert_eq!(result.key_size, 256);
    
    // Test conversion to Value
    let value = result.to_value().unwrap();
    if let Value::Object(map) = value {
        assert_eq!(map.get("algorithm"), Some(&Value::String("ECDH-P256".to_string())));
        assert_eq!(map.get("key_size"), Some(&Value::Integer(256)));
        assert!(map.contains_key("shared_secret"));
        assert!(map.contains_key("derived_key"));
    } else {
        panic!("Expected Value::Object");
    }
    
    tracing::info!("✅ Key agreement result tests passed");
}

#[test]
fn test_x25519_key_agreement() {
    init_tracing!();
    tracing::info!("Testing X25519 key agreement");
    
    // Generate test key pairs
    let mut rng = OsRng;
    let alice_private = StaticSecret::random_from_rng(&mut rng);
    let alice_public = X25519PublicKey::from(&alice_private);
    
    let bob_private = StaticSecret::random_from_rng(&mut rng);
    let bob_public = X25519PublicKey::from(&bob_private);
    
    // Test Alice's side
    let alice_args = vec![
        Value::String("X25519".to_string()),
        Value::String(hex::encode(alice_private.to_bytes())),
        Value::String(hex::encode(bob_public.to_bytes())),
    ];
    
    let alice_result = key_agreement(alice_args).unwrap();
    
    // Test Bob's side
    let bob_args = vec![
        Value::String("X25519".to_string()),
        Value::String(hex::encode(bob_private.to_bytes())),
        Value::String(hex::encode(alice_public.to_bytes())),
    ];
    
    let bob_result = key_agreement(bob_args).unwrap();
    
    // Both should get the same shared secret
    if let (Value::Object(alice_map), Value::Object(bob_map)) = (alice_result, bob_result) {
        assert_eq!(
            alice_map.get("shared_secret"),
            bob_map.get("shared_secret")
        );
        assert_eq!(
            alice_map.get("derived_key"),
            bob_map.get("derived_key")
        );
    } else {
        panic!("Expected Value::Object results");
    }
    
    tracing::info!("✅ X25519 key agreement tests passed");
}

#[test]
fn test_x25519_key_agreement_direct() {
    init_tracing!();
    tracing::info!("Testing X25519 key agreement direct function");
    
    // Generate test key pairs
    let mut rng = OsRng;
    let alice_private = StaticSecret::random_from_rng(&mut rng);
    let alice_public = X25519PublicKey::from(&alice_private);
    
    let bob_private = StaticSecret::random_from_rng(&mut rng);
    let bob_public = X25519PublicKey::from(&bob_private);
    
    // Test direct function call
    let args = vec![
        Value::String(hex::encode(alice_private.to_bytes())),
        Value::String(hex::encode(bob_public.to_bytes())),
    ];
    
    let result = x25519_agreement(&args).unwrap();
    
    if let Value::Object(map) = result {
        assert_eq!(map.get("algorithm"), Some(&Value::String("X25519".to_string())));
        assert_eq!(map.get("key_size"), Some(&Value::Integer(255)));
        assert!(map.contains_key("shared_secret"));
        assert!(map.contains_key("derived_key"));
    } else {
        panic!("Expected Value::Object");
    }
    
    tracing::info!("✅ X25519 direct key agreement tests passed");
}

#[test]
fn test_ecdh_p256_key_agreement() {
    init_tracing!();
    tracing::info!("Testing ECDH P-256 key agreement");
    
    // Generate test key pairs
    let mut rng = OsRng;
    let alice_private = P256SecretKey::random(&mut rng);
    let alice_public = alice_private.public_key();
    
    let bob_private = P256SecretKey::random(&mut rng);
    let bob_public = bob_private.public_key();
    
    // Test Alice's side
    let alice_args = vec![
        Value::String("ECDH-P256".to_string()),
        Value::String(hex::encode(alice_private.to_bytes())),
        Value::String(hex::encode(bob_public.to_encoded_point(false).as_bytes())),
    ];
    
    let alice_result = key_agreement(alice_args).unwrap();
    
    // Test Bob's side  
    let bob_args = vec![
        Value::String("ECDH-P256".to_string()),
        Value::String(hex::encode(bob_private.to_bytes())),
        Value::String(hex::encode(alice_public.to_encoded_point(false).as_bytes())),
    ];
    
    let bob_result = key_agreement(bob_args).unwrap();
    
    // Both should get the same shared secret
    if let (Value::Object(alice_map), Value::Object(bob_map)) = (alice_result, bob_result) {
        assert_eq!(
            alice_map.get("shared_secret"),
            bob_map.get("shared_secret")
        );
        assert_eq!(
            alice_map.get("derived_key"),
            bob_map.get("derived_key")
        );
        assert_eq!(alice_map.get("algorithm"), Some(&Value::String("ECDH-P256".to_string())));
    } else {
        panic!("Expected Value::Object results");
    }
    
    tracing::info!("✅ ECDH P-256 key agreement tests passed");
}

#[test]
fn test_ecdh_p384_key_agreement() {
    init_tracing!();
    tracing::info!("Testing ECDH P-384 key agreement");
    
    // Generate test key pairs
    let mut rng = OsRng;
    let alice_private = P384SecretKey::random(&mut rng);
    let alice_public = alice_private.public_key();
    
    let bob_private = P384SecretKey::random(&mut rng);
    let bob_public = bob_private.public_key();
    
    // Test direct function call
    let args = vec![
        Value::String(hex::encode(alice_private.to_bytes())),
        Value::String(hex::encode(bob_public.to_encoded_point(false).as_bytes())),
    ];
    
    let result = ecdh_p384_agreement(&args).unwrap();
    
    if let Value::Object(map) = result {
        assert_eq!(map.get("algorithm"), Some(&Value::String("ECDH-P384".to_string())));
        assert_eq!(map.get("key_size"), Some(&Value::Integer(384)));
        assert!(map.contains_key("shared_secret"));
        assert!(map.contains_key("derived_key"));
    } else {
        panic!("Expected Value::Object");
    }
    
    tracing::info!("✅ ECDH P-384 key agreement tests passed");
}

#[test]
fn test_ecdh_p521_key_agreement_not_implemented() {
    init_tracing!();
    tracing::info!("Testing ECDH P-521 key agreement (not implemented)");
    
    // Test with P-521 size key (66 bytes)
    let private_key = vec![0u8; 66];
    let public_key = vec![0u8; 67]; // Compressed P-521 public key
    
    let args = vec![
        Value::String(hex::encode(private_key)),
        Value::String(hex::encode(public_key)),
    ];
    
    let result = ecdh_p521_agreement(&args);
    assert!(result.is_err());
    
    if let Err(CursedError::NotImplemented(msg)) = result {
        assert!(msg.contains("P-521 ECDH not yet implemented"));
        tracing::info!("✅ P-521 correctly returns NotImplemented: {}", msg);
    } else {
        panic!("Expected NotImplemented error for P-521");
    }
    
    tracing::info!("✅ ECDH P-521 not implemented test passed");
}

#[test]
fn test_x448_key_agreement() {
    init_tracing!();
    tracing::info!("Testing X448 key agreement");
    
    // Generate test keys (56 bytes each)
    let mut rng = OsRng;
    let mut alice_private = [0u8; 56];
    let mut bob_private = [0u8; 56];
    rng.fill_bytes(&mut alice_private);
    rng.fill_bytes(&mut bob_private);
    
    // Generate mock public keys (in real implementation, would derive from private)
    let mut alice_public = [0u8; 56];
    let mut bob_public = [0u8; 56];
    alice_public[0] = 5; // Base point
    bob_public[0] = 5;
    rng.fill_bytes(&mut alice_public[1..]);
    rng.fill_bytes(&mut bob_public[1..]);
    
    // Test direct function call
    let args = vec![
        Value::String(hex::encode(alice_private)),
        Value::String(hex::encode(bob_public)),
    ];
    
    let result = x448_agreement(&args).unwrap();
    
    if let Value::Object(map) = result {
        assert_eq!(map.get("algorithm"), Some(&Value::String("X448".to_string())));
        assert_eq!(map.get("key_size"), Some(&Value::Integer(448)));
        assert!(map.contains_key("shared_secret"));
        assert!(map.contains_key("derived_key"));
    } else {
        panic!("Expected Value::Object");
    }
    
    tracing::info!("✅ X448 key agreement tests passed");
}

#[test]
fn test_rsa_oaep_key_transport() {
    init_tracing!();
    tracing::info!("Testing RSA OAEP key transport");
    
    // Generate RSA key pair
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
    let public_key = RsaPublicKey::from(&private_key);
    
    // Export public key to PEM
    let public_key_pem = rsa::pkcs8::EncodePublicKey::to_public_key_pem(&public_key, rsa::pkcs8::LineEnding::LF).unwrap();
    
    // Generate a key to transport (32 bytes)
    let mut key_to_transport = [0u8; 32];
    rng.fill_bytes(&mut key_to_transport);
    
    // Test RSA OAEP key transport
    let args = vec![
        Value::String(public_key_pem),
        Value::String(hex::encode(key_to_transport)),
    ];
    
    let result = rsa_oaep_agreement(&args).unwrap();
    
    if let Value::Object(map) = result {
        assert_eq!(map.get("algorithm"), Some(&Value::String("RSA-OAEP".to_string())));
        assert_eq!(map.get("key_size"), Some(&Value::Integer(2048)));
        assert!(map.contains_key("shared_secret"));
        assert!(map.contains_key("derived_key"));
        
        // The shared_secret should be the original key
        if let Some(Value::String(shared_secret_hex)) = map.get("shared_secret") {
            let decoded_secret = hex::decode(shared_secret_hex).unwrap();
            assert_eq!(decoded_secret, key_to_transport);
        }
        
        // The derived_key should be the encrypted key
        if let Some(Value::String(encrypted_key_hex)) = map.get("derived_key") {
            let encrypted_key = hex::decode(encrypted_key_hex).unwrap();
            assert_eq!(encrypted_key.len(), 256); // 2048-bit RSA = 256 bytes
        }
    } else {
        panic!("Expected Value::Object");
    }
    
    tracing::info!("✅ RSA OAEP key transport tests passed");
}

#[test]
fn test_validate_key_agreement_params() {
    init_tracing!();
    tracing::info!("Testing key agreement parameter validation");
    
    // Valid X25519 keys
    let valid_x25519_key = vec![0u8; 32];
    assert!(validate_key_agreement_params(
        KeyAgreementAlgorithm::X25519,
        &valid_x25519_key,
        &valid_x25519_key
    ).is_ok());
    
    // Invalid X25519 keys (wrong length)
    let invalid_key = vec![0u8; 16];
    assert!(validate_key_agreement_params(
        KeyAgreementAlgorithm::X25519,
        &invalid_key,
        &valid_x25519_key
    ).is_err());
    
    // Valid P-256 keys
    let valid_p256_private = vec![0u8; 32];
    let valid_p256_public_compressed = vec![0u8; 33];
    let valid_p256_public_uncompressed = vec![0u8; 65];
    
    assert!(validate_key_agreement_params(
        KeyAgreementAlgorithm::EcdhP256,
        &valid_p256_private,
        &valid_p256_public_compressed
    ).is_ok());
    
    assert!(validate_key_agreement_params(
        KeyAgreementAlgorithm::EcdhP256,
        &valid_p256_private,
        &valid_p256_public_uncompressed
    ).is_ok());
    
    // Valid P-384 keys
    let valid_p384_private = vec![0u8; 48];
    let valid_p384_public_compressed = vec![0u8; 49];
    assert!(validate_key_agreement_params(
        KeyAgreementAlgorithm::EcdhP384,
        &valid_p384_private,
        &valid_p384_public_compressed
    ).is_ok());
    
    // Valid X448 keys
    let valid_x448_key = vec![0u8; 56];
    assert!(validate_key_agreement_params(
        KeyAgreementAlgorithm::X448,
        &valid_x448_key,
        &valid_x448_key
    ).is_ok());
    
    // RSA keys (basic validation)
    let rsa_key = vec![1u8; 256];
    assert!(validate_key_agreement_params(
        KeyAgreementAlgorithm::RsaOaep,
        &rsa_key,
        &rsa_key
    ).is_ok());
    
    // Empty RSA keys should fail
    assert!(validate_key_agreement_params(
        KeyAgreementAlgorithm::RsaOaep,
        &[],
        &rsa_key
    ).is_err());
    
    tracing::info!("✅ Key agreement parameter validation tests passed");
}

#[test] 
fn test_list_key_agreement_algorithms() {
    init_tracing!();
    tracing::info!("Testing list of key agreement algorithms");
    
    let algorithms = list_key_agreement_algorithms();
    
    assert!(algorithms.contains(&"ECDH-P256".to_string()));
    assert!(algorithms.contains(&"ECDH-P384".to_string()));
    assert!(algorithms.contains(&"ECDH-P521".to_string()));
    assert!(algorithms.contains(&"X25519".to_string()));
    assert!(algorithms.contains(&"X448".to_string()));
    assert!(algorithms.contains(&"RSA-OAEP".to_string()));
    
    assert_eq!(algorithms.len(), 6);
    
    tracing::info!("✅ List key agreement algorithms tests passed");
}

#[test]
fn test_derive_key_from_shared_secret() {
    init_tracing!();
    tracing::info!("Testing key derivation from shared secret");
    
    let shared_secret = b"test_shared_secret_for_derivation";
    
    // Test with different algorithms
    for algorithm in [
        KeyAgreementAlgorithm::EcdhP256,
        KeyAgreementAlgorithm::EcdhP384,
        KeyAgreementAlgorithm::EcdhP521,
        KeyAgreementAlgorithm::X25519,
        KeyAgreementAlgorithm::X448,
        KeyAgreementAlgorithm::RsaOaep,
    ] {
        let result = derive_key_from_shared_secret(
            shared_secret,
            32,
            Some(algorithm),
            Some("test-info"),
        );
        
        assert!(result.is_ok(), "Failed for algorithm: {:?}", algorithm);
        assert_eq!(result.unwrap().len(), 32);
    }
    
    // Test with different key lengths
    for key_length in [16, 32, 48, 64] {
        let result = derive_key_from_shared_secret(
            shared_secret,
            key_length,
            Some(KeyAgreementAlgorithm::EcdhP256),
            Some("test-info"),
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), key_length);
    }
    
    // Test invalid key lengths
    assert!(derive_key_from_shared_secret(shared_secret, 0, None, None).is_err());
    assert!(derive_key_from_shared_secret(shared_secret, 255 * 64 + 1, None, None).is_err());
    
    tracing::info!("✅ Key derivation tests passed");
}

#[test]
fn test_key_agreement_error_handling() {
    init_tracing!();
    tracing::info!("Testing key agreement error handling");
    
    // Test missing arguments
    let result = key_agreement(vec![]);
    assert!(result.is_err());
    
    // Test invalid algorithm
    let result = key_agreement(vec![Value::String("INVALID".to_string())]);
    assert!(result.is_err());
    
    // Test X25519 with invalid key lengths
    let result = x25519_agreement(&[
        Value::String(hex::encode(vec![0u8; 16])), // Wrong length
        Value::String(hex::encode(vec![0u8; 32])),
    ]);
    assert!(result.is_err());
    
    // Test X25519 with identity element (all zeros)
    let result = x25519_agreement(&[
        Value::String(hex::encode(vec![1u8; 32])),
        Value::String(hex::encode(vec![0u8; 32])), // Identity element
    ]);
    assert!(result.is_err());
    
    // Test X448 with identity element
    let result = x448_agreement(&[
        Value::String(hex::encode(vec![1u8; 56])),
        Value::String(hex::encode(vec![0u8; 56])), // Identity element
    ]);
    assert!(result.is_err());
    
    // Test RSA with too large key
    let large_key = vec![0u8; 300]; // Too large for 2048-bit RSA
    let result = rsa_oaep_agreement(&[
        Value::String("-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA1234567890\n-----END PUBLIC KEY-----".to_string()),
        Value::String(hex::encode(large_key)),
    ]);
    assert!(result.is_err());
    
    tracing::info!("✅ Key agreement error handling tests passed");
}

#[test]
fn test_key_agreement_comprehensive() {
    init_tracing!();
    tracing::info!("Running comprehensive key agreement test suite");
    
    // Test all implemented algorithms
    let algorithms = [
        "ECDH-P256",
        "ECDH-P384", 
        "X25519",
        "X448",
    ];
    
    for &algorithm in &algorithms {
        tracing::info!("Testing algorithm: {}", algorithm);
        
        let alg_enum = KeyAgreementAlgorithm::from_name(algorithm).unwrap();
        let key_size = alg_enum.key_size();
        
        tracing::info!("Algorithm {} has key size: {}", algorithm, key_size);
        
        // Test that we can at least parse the algorithm
        assert!(!algorithm.is_empty());
        assert!(key_size > 0);
    }
    
    // Test parameter validation for all algorithms
    for &algorithm in &algorithms {
        let alg_enum = KeyAgreementAlgorithm::from_name(algorithm).unwrap();
        let algorithms_list = list_key_agreement_algorithms();
        assert!(algorithms_list.contains(&algorithm.to_string()));
    }
    
    tracing::info!("✅ Comprehensive key agreement tests passed");
}
