/// fr fr KDF Factory Integration Tests
/// 
/// Comprehensive test suite to validate the KDF factory functions work correctly
/// with all supported algorithms and configurations.

use cursed::stdlib::packages::crypto_kdf::*;

/// Test basic factory function creation for all algorithms
#[test]
fn test_factory_basic_creation() {
    // Test PBKDF2 creation
    let pbkdf2 = KdfFactory::create_kdf("pbkdf2");
    assert!(pbkdf2.is_ok(), "PBKDF2 creation should succeed");
    
    let engine = pbkdf2.unwrap();
    assert_eq!(engine.algorithm_name(), "PBKDF2");
    assert_eq!(engine.algorithm_version(), "RFC2898");
    
    // Test Argon2 creation (default variant)
    let argon2 = KdfFactory::create_kdf("argon2");
    assert!(argon2.is_ok(), "Argon2 creation should succeed");
    
    let engine = argon2.unwrap();
    assert_eq!(engine.algorithm_name(), "Argon2");
    assert_eq!(engine.algorithm_version(), "v1.3");
    
    // Test Argon2id creation
    let argon2id = KdfFactory::create_kdf("argon2id");
    assert!(argon2id.is_ok(), "Argon2id creation should succeed");
    
    // Test Argon2i creation
    let argon2i = KdfFactory::create_kdf("argon2i");
    assert!(argon2i.is_ok(), "Argon2i creation should succeed");
    
    // Test Argon2d creation
    let argon2d = KdfFactory::create_kdf("argon2d");
    assert!(argon2d.is_ok(), "Argon2d creation should succeed");
    
    // Test Scrypt creation
    let scrypt = KdfFactory::create_kdf("scrypt");
    assert!(scrypt.is_ok(), "Scrypt creation should succeed");
    
    let engine = scrypt.unwrap();
    assert_eq!(engine.algorithm_name(), "scrypt");
    assert_eq!(engine.algorithm_version(), "RFC7914");
    
    // Test HKDF creation
    let hkdf = KdfFactory::create_kdf("hkdf");
    assert!(hkdf.is_ok(), "HKDF creation should succeed");
    
    let engine = hkdf.unwrap();
    assert_eq!(engine.algorithm_name(), "HKDF");
    assert_eq!(engine.algorithm_version(), "RFC5869");
}

/// Test factory creation with invalid algorithm names
#[test]
fn test_factory_invalid_algorithms() {
    // Test unknown algorithm
    let unknown = KdfFactory::create_kdf("unknown_algorithm");
    assert!(unknown.is_err(), "Unknown algorithm should fail");
    
    match unknown.unwrap_err() {
        KdfError::InvalidConfig(msg) => {
            assert!(msg.contains("Unknown algorithm"));
        }
        _ => panic!("Expected InvalidConfig error"),
    }
    
    // Test empty algorithm name
    let empty = KdfFactory::create_kdf("");
    assert!(empty.is_err(), "Empty algorithm name should fail");
    
    // Test case sensitivity (should work)
    let upper_case = KdfFactory::create_kdf("PBKDF2");
    assert!(upper_case.is_ok(), "Uppercase algorithm names should work");
    
    let mixed_case = KdfFactory::create_kdf("ScRyPt");
    assert!(mixed_case.is_ok(), "Mixed case algorithm names should work");
}

/// Test factory creation with configuration strings
#[test]
fn test_factory_with_config() {
    // Test PBKDF2 with custom configuration
    let pbkdf2_config = "iterations=50000,output_len=64,hash=sha512";
    let pbkdf2 = KdfFactory::create_kdf_with_config("pbkdf2", pbkdf2_config);
    assert!(pbkdf2.is_ok(), "PBKDF2 with config should succeed");
    
    // Test Argon2 with custom configuration
    let argon2_config = "memory=32768,time=2,parallelism=2,output_len=64";
    let argon2 = KdfFactory::create_kdf_with_config("argon2id", argon2_config);
    assert!(argon2.is_ok(), "Argon2 with config should succeed");
    
    // Test Scrypt with custom configuration
    let scrypt_config = "n=16384,r=4,p=2,output_len=64";
    let scrypt = KdfFactory::create_kdf_with_config("scrypt", scrypt_config);
    assert!(scrypt.is_ok(), "Scrypt with config should succeed");
    
    // Test HKDF (config ignored for HKDF)
    let hkdf = KdfFactory::create_kdf_with_config("hkdf", "any_config");
    assert!(hkdf.is_ok(), "HKDF with config should succeed");
}

/// Test factory with invalid configurations
#[test]
fn test_factory_invalid_configs() {
    // Test PBKDF2 with invalid iterations
    let bad_iterations = KdfFactory::create_kdf_with_config("pbkdf2", "iterations=invalid");
    assert!(bad_iterations.is_err(), "PBKDF2 with invalid iterations should fail");
    
    // Test PBKDF2 with too few iterations
    let low_iterations = KdfFactory::create_kdf_with_config("pbkdf2", "iterations=100");
    assert!(low_iterations.is_err(), "PBKDF2 with too few iterations should fail");
    
    // Test Argon2 with invalid memory
    let bad_memory = KdfFactory::create_kdf_with_config("argon2", "memory=invalid");
    assert!(bad_memory.is_err(), "Argon2 with invalid memory should fail");
    
    // Test Scrypt with invalid N
    let bad_n = KdfFactory::create_kdf_with_config("scrypt", "n=invalid");
    assert!(bad_n.is_err(), "Scrypt with invalid N should fail");
}

/// Test key derivation functionality through factory
#[test]
fn test_factory_key_derivation() {
    let password = b"test_password_123";
    let salt = b"test_salt_16bytes";
    let output_length = 32;
    
    // Test PBKDF2 key derivation
    let pbkdf2 = KdfFactory::create_kdf("pbkdf2").unwrap();
    let key1 = pbkdf2.derive_key(password, salt, output_length);
    assert!(key1.is_ok(), "PBKDF2 key derivation should succeed");
    
    let derived_key = key1.unwrap();
    assert_eq!(derived_key.len(), output_length);
    assert_ne!(derived_key.iter().sum::<u8>(), 0); // Should not be all zeros
    
    // Test Argon2 key derivation
    let argon2 = KdfFactory::create_kdf("argon2").unwrap();
    let key2 = argon2.derive_key(password, salt, output_length);
    assert!(key2.is_ok(), "Argon2 key derivation should succeed");
    
    let derived_key2 = key2.unwrap();
    assert_eq!(derived_key2.len(), output_length);
    assert_ne!(derived_key2.iter().sum::<u8>(), 0);
    
    // Test that different algorithms produce different keys
    assert_ne!(derived_key, derived_key2, "Different algorithms should produce different keys");
    
    // Test Scrypt key derivation
    let scrypt = KdfFactory::create_kdf("scrypt").unwrap();
    let key3 = scrypt.derive_key(password, salt, output_length);
    assert!(key3.is_ok(), "Scrypt key derivation should succeed");
    
    let derived_key3 = key3.unwrap();
    assert_eq!(derived_key3.len(), output_length);
    assert_ne!(derived_key3, derived_key);
    assert_ne!(derived_key3, derived_key2);
    
    // Test HKDF key derivation
    let hkdf = KdfFactory::create_kdf("hkdf").unwrap();
    let key4 = hkdf.derive_key(password, salt, output_length);
    assert!(key4.is_ok(), "HKDF key derivation should succeed");
    
    let derived_key4 = key4.unwrap();
    assert_eq!(derived_key4.len(), output_length);
}

/// Test key derivation with invalid inputs
#[test]
fn test_factory_invalid_inputs() {
    let kdf = KdfFactory::create_kdf("pbkdf2").unwrap();
    
    // Test empty password
    let empty_password = kdf.derive_key(b"", b"salt1234567890", 32);
    assert!(empty_password.is_err(), "Empty password should fail");
    
    // Test short salt
    let short_salt = kdf.derive_key(b"password", b"short", 32);
    assert!(short_salt.is_err(), "Short salt should fail");
    
    // Test zero output length
    let zero_length = kdf.derive_key(b"password", b"salt1234567890", 0);
    assert!(zero_length.is_err(), "Zero output length should fail");
    
    // Test excessive output length
    let huge_length = kdf.derive_key(b"password", b"salt1234567890", 2_000_000);
    assert!(huge_length.is_err(), "Excessive output length should fail");
}

/// Test security assessment functionality
#[test]
fn test_factory_security_assessment() {
    // Test PBKDF2 security
    let pbkdf2 = KdfFactory::create_kdf("pbkdf2").unwrap();
    assert_eq!(pbkdf2.security_level(), 128);
    assert!(pbkdf2.is_constant_time());
    
    // Test Argon2 security
    let argon2 = KdfFactory::create_kdf("argon2").unwrap();
    assert_eq!(argon2.security_level(), 256);
    assert!(!argon2.is_constant_time()); // Argon2d variant is not constant-time
    
    // Test Scrypt security
    let scrypt = KdfFactory::create_kdf("scrypt").unwrap();
    assert_eq!(scrypt.security_level(), 192);
    assert!(scrypt.is_constant_time());
    
    // Test HKDF security
    let hkdf = KdfFactory::create_kdf("hkdf").unwrap();
    assert_eq!(hkdf.security_level(), 128);
    assert!(hkdf.is_constant_time());
}

/// Test attack resistance assessment
#[test]
fn test_factory_attack_resistance() {
    // Test PBKDF2 attack resistance
    let pbkdf2 = KdfFactory::create_kdf("pbkdf2").unwrap();
    let resistance = pbkdf2.attack_resistance();
    assert!(resistance.brute_force >= 128);
    assert!(!resistance.side_channel); // Basic PBKDF2 doesn't have side-channel resistance
    
    // Test Argon2 attack resistance (high security)
    let argon2 = KdfFactory::create_kdf("argon2").unwrap();
    let resistance = argon2.attack_resistance();
    assert!(resistance.brute_force >= 256);
    assert!(resistance.side_channel); // Argon2 has side-channel resistance
    assert!(resistance.timing_attack);
    
    // Test Scrypt attack resistance
    let scrypt = KdfFactory::create_kdf("scrypt").unwrap();
    let resistance = scrypt.attack_resistance();
    assert!(resistance.gpu_attack >= 128);
    assert!(resistance.asic_attack >= 80);
}

/// Test factory utility functions
#[test]
fn test_factory_utilities() {
    // Test available algorithms
    let algorithms = KdfFactory::available_algorithms();
    assert!(algorithms.contains(&"pbkdf2"));
    assert!(algorithms.contains(&"argon2"));
    assert!(algorithms.contains(&"argon2i"));
    assert!(algorithms.contains(&"argon2d"));
    assert!(algorithms.contains(&"argon2id"));
    assert!(algorithms.contains(&"scrypt"));
    assert!(algorithms.contains(&"hkdf"));
    
    // Test algorithm recommendations
    assert_eq!(KdfFactory::recommend_algorithm("password"), "argon2id");
    assert_eq!(KdfFactory::recommend_algorithm("authentication"), "argon2id");
    assert_eq!(KdfFactory::recommend_algorithm("key_derivation"), "hkdf");
    assert_eq!(KdfFactory::recommend_algorithm("encryption"), "hkdf");
    assert_eq!(KdfFactory::recommend_algorithm("legacy"), "pbkdf2");
    assert_eq!(KdfFactory::recommend_algorithm("compatibility"), "pbkdf2");
    assert_eq!(KdfFactory::recommend_algorithm("memory_hard"), "scrypt");
    assert_eq!(KdfFactory::recommend_algorithm("slow"), "scrypt");
    assert_eq!(KdfFactory::recommend_algorithm("unknown"), "argon2id"); // Default
}

/// Test consistency across multiple factory creations
#[test]
fn test_factory_consistency() {
    let password = b"consistent_password";
    let salt = b"consistent_salt_16bytes";
    let output_length = 32;
    
    // Create multiple instances of the same algorithm
    let kdf1 = KdfFactory::create_kdf("pbkdf2").unwrap();
    let kdf2 = KdfFactory::create_kdf("pbkdf2").unwrap();
    
    let key1 = kdf1.derive_key(password, salt, output_length).unwrap();
    let key2 = kdf2.derive_key(password, salt, output_length).unwrap();
    
    // Keys should be identical for same algorithm and parameters
    assert_eq!(key1, key2, "Same algorithm should produce consistent keys");
}

/// Test configuration validation
#[test]
fn test_factory_config_validation() {
    // Test valid configuration formats
    let valid_configs = vec![
        "iterations=100000",
        "iterations=100000,output_len=32",
        "iterations=100000,output_len=32,hash=sha256",
        "memory=65536,time=3,parallelism=4",
        "n=32768,r=8,p=1,output_len=64",
    ];
    
    for config in valid_configs {
        // Should not panic or error during parsing
        let _result = KdfFactory::create_kdf_with_config("pbkdf2", config);
        // Note: We don't assert success here as some configs might be invalid for specific algorithms
    }
    
    // Test malformed configuration formats
    let malformed_configs = vec![
        "iterations", // Missing value
        "=100000", // Missing key
        "iterations=", // Missing value
        "invalid_format", // No key-value pairs
    ];
    
    for config in malformed_configs {
        // Should handle gracefully (either succeed with defaults or fail with proper error)
        let result = KdfFactory::create_kdf_with_config("pbkdf2", config);
        // We allow these to succeed if they fall back to defaults
        if result.is_err() {
            // If they fail, it should be with a proper error type
            match result.unwrap_err() {
                KdfError::InvalidConfig(_) | KdfError::InvalidInput(_) => {
                    // Expected error types
                }
                _ => panic!("Unexpected error type for malformed config"),
            }
        }
    }
}

/// Test memory safety and resource cleanup
#[test]
fn test_factory_memory_safety() {
    // Create many factory instances to test for memory leaks
    for _ in 0..100 {
        let _kdf = KdfFactory::create_kdf("pbkdf2").unwrap();
        let _kdf2 = KdfFactory::create_kdf("argon2").unwrap();
        let _kdf3 = KdfFactory::create_kdf("scrypt").unwrap();
        let _kdf4 = KdfFactory::create_kdf("hkdf").unwrap();
    }
    
    // Derive many keys to test memory handling
    let kdf = KdfFactory::create_kdf("pbkdf2").unwrap();
    for i in 0..50 {
        let password = format!("password_{}", i);
        let salt = format!("salt_{:016}", i);
        let _key = kdf.derive_key(password.as_bytes(), salt.as_bytes(), 32).unwrap();
    }
}

/// Test concurrent factory usage
#[test]
fn test_factory_concurrent_usage() {
    use std::sync::Arc;
    use std::thread;
    
    let handles: Vec<_> = (0..4).map(|thread_id| {
        thread::spawn(move || {
            for i in 0..25 {
                let algorithm = match i % 4 {
                    0 => "pbkdf2",
                    1 => "argon2",
                    2 => "scrypt",
                    3 => "hkdf",
                    _ => unreachable!(),
                };
                
                let kdf = KdfFactory::create_kdf(algorithm).unwrap();
                let password = format!("thread_{}_password_{}", thread_id, i);
                let salt = format!("thread_{}_salt_{:012}", thread_id, i);
                
                let _key = kdf.derive_key(password.as_bytes(), salt.as_bytes(), 32).unwrap();
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}
