/// fr fr Comprehensive tests for KDF engine creation and functionality
/// 
/// Tests the KDF engine creation from configuration strings and validates
/// all KDF implementations work correctly.

use cursed::stdlib::packages::crypto_kdf::kdf_traits::*;
use cursed::stdlib::packages::crypto_kdf::*;
use cursed::error::CursedError;

#[test]
fn test_kdf_engine_creation_from_config() {
    // Test PBKDF2 engine creation
    let pbkdf2_config = "pbkdf2:iterations=100000,output_len=32,hash=sha256";
    let pbkdf2_engine = KdfEngineWrapper::create_engine_from_config(&pbkdf2_config.to_string()).unwrap();
    assert_eq!(pbkdf2_engine.algorithm_name(), "PBKDF2");
    assert_eq!(pbkdf2_engine.algorithm_version(), "RFC2898");
    
    // Test Argon2 engine creation
    let argon2_config = "argon2id:memory=65536,time=3,parallelism=4,output_len=32";
    let argon2_engine = KdfEngineWrapper::create_engine_from_config(&argon2_config.to_string()).unwrap();
    assert_eq!(argon2_engine.algorithm_name(), "Argon2");
    assert_eq!(argon2_engine.algorithm_version(), "v1.3");
    
    // Test Scrypt engine creation
    let scrypt_config = "scrypt:n=32768,r=8,p=1,output_len=32";
    let scrypt_engine = KdfEngineWrapper::create_engine_from_config(&scrypt_config.to_string()).unwrap();
    assert_eq!(scrypt_engine.algorithm_name(), "scrypt");
    assert_eq!(scrypt_engine.algorithm_version(), "RFC7914");
    
    // Test HKDF engine creation
    let hkdf_config = "hkdf:";
    let hkdf_engine = KdfEngineWrapper::create_engine_from_config(&hkdf_config.to_string()).unwrap();
    assert_eq!(hkdf_engine.algorithm_name(), "HKDF");
    assert_eq!(hkdf_engine.algorithm_version(), "RFC5869");
}

#[test]
fn test_kdf_engine_config_validation() {
    // Test valid configurations
    let valid_configs = vec![
        "pbkdf2:iterations=100000",
        "argon2:memory=65536,time=3",
        "argon2i:time=5",
        "argon2d:parallelism=2",
        "argon2id:output_len=64",
        "scrypt:n=16384,r=8,p=1",
        "hkdf:",
        "hkdf",
    ];
    
    for config in valid_configs {
        assert!(KdfEngineWrapper::validate_config(&config.to_string()).is_ok(), 
                "Config should be valid: {}", config);
    }
    
    // Test invalid configurations
    let invalid_configs = vec![
        "",
        "unknown_algorithm:",
        "invalid",
        "pbkdf2", // Missing colon for params
        "xyz:param=value",
    ];
    
    for config in invalid_configs {
        assert!(KdfEngineWrapper::validate_config(&config.to_string()).is_err(),
                "Config should be invalid: {}", config);
    }
}

#[test]
fn test_kdf_factory_creation() {
    // Test creating KDF instances by algorithm name
    let pbkdf2_kdf = KdfFactory::create_kdf("pbkdf2").unwrap();
    assert_eq!(pbkdf2_kdf.algorithm_name(), "PBKDF2");
    
    let argon2_kdf = KdfFactory::create_kdf("argon2").unwrap();
    assert_eq!(argon2_kdf.algorithm_name(), "Argon2");
    
    let scrypt_kdf = KdfFactory::create_kdf("scrypt").unwrap();
    assert_eq!(scrypt_kdf.algorithm_name(), "scrypt");
    
    let hkdf_kdf = KdfFactory::create_kdf("hkdf").unwrap();
    assert_eq!(hkdf_kdf.algorithm_name(), "HKDF");
    
    // Test with configuration
    let configured_kdf = KdfFactory::create_kdf_with_config("pbkdf2", "iterations=50000,output_len=64").unwrap();
    assert_eq!(configured_kdf.algorithm_name(), "PBKDF2");
}

#[test]
fn test_kdf_factory_error_handling() {
    // Test unknown algorithm
    assert!(KdfFactory::create_kdf("unknown").is_err());
    assert!(KdfFactory::create_kdf("").is_err());
    
    // Test invalid configuration
    assert!(KdfFactory::create_kdf_with_config("pbkdf2", "invalid_param=value").is_ok()); // Should ignore unknown params
    
    // Test algorithm variants
    assert!(KdfFactory::create_kdf("argon2i").is_ok());
    assert!(KdfFactory::create_kdf("argon2d").is_ok());
    assert!(KdfFactory::create_kdf("argon2id").is_ok());
}

#[test]
fn test_kdf_available_algorithms() {
    let algorithms = KdfFactory::available_algorithms();
    
    assert!(algorithms.contains(&"pbkdf2"));
    assert!(algorithms.contains(&"argon2"));
    assert!(algorithms.contains(&"argon2i"));
    assert!(algorithms.contains(&"argon2d"));
    assert!(algorithms.contains(&"argon2id"));
    assert!(algorithms.contains(&"scrypt"));
    assert!(algorithms.contains(&"hkdf"));
    
    assert_eq!(algorithms.len(), 7);
}

#[test]
fn test_kdf_algorithm_recommendations() {
    let test_cases = vec![
        ("password", "argon2id"),
        ("authentication", "argon2id"),
        ("key_derivation", "hkdf"),
        ("encryption", "hkdf"),
        ("legacy", "pbkdf2"),
        ("compatibility", "pbkdf2"),
        ("memory_hard", "scrypt"),
        ("slow", "scrypt"),
        ("unknown_use_case", "argon2id"), // Default
    ];
    
    for (use_case, expected) in test_cases {
        let recommendation = KdfFactory::recommend_algorithm(use_case);
        assert_eq!(recommendation, expected, "Wrong recommendation for {}", use_case);
    }
}

#[test]
fn test_kdf_key_derivation_functionality() {
    let password = b"test_password_123";
    let salt = b"test_salt_456789";
    let output_length = 32;
    
    // Test PBKDF2
    let pbkdf2_engine = KdfFactory::create_kdf("pbkdf2").unwrap();
    let pbkdf2_key = pbkdf2_engine.derive_key(password, salt, output_length).unwrap();
    assert_eq!(pbkdf2_key.len(), output_length);
    
    // Test Argon2
    let argon2_engine = KdfFactory::create_kdf("argon2").unwrap();
    let argon2_key = argon2_engine.derive_key(password, salt, output_length).unwrap();
    assert_eq!(argon2_key.len(), output_length);
    
    // Test Scrypt
    let scrypt_engine = KdfFactory::create_kdf("scrypt").unwrap();
    let scrypt_key = scrypt_engine.derive_key(password, salt, output_length).unwrap();
    assert_eq!(scrypt_key.len(), output_length);
    
    // Test HKDF
    let hkdf_engine = KdfFactory::create_kdf("hkdf").unwrap();
    let hkdf_key = hkdf_engine.derive_key(password, salt, output_length).unwrap();
    assert_eq!(hkdf_key.len(), output_length);
    
    // All keys should be different
    assert_ne!(pbkdf2_key, argon2_key);
    assert_ne!(pbkdf2_key, scrypt_key);
    assert_ne!(pbkdf2_key, hkdf_key);
    assert_ne!(argon2_key, scrypt_key);
    assert_ne!(argon2_key, hkdf_key);
    assert_ne!(scrypt_key, hkdf_key);
}

#[test]
fn test_kdf_input_validation() {
    let engine = KdfFactory::create_kdf("pbkdf2").unwrap();
    
    // Test empty password
    assert!(engine.derive_key(b"", b"valid_salt", 32).is_err());
    
    // Test short salt
    assert!(engine.derive_key(b"password", b"short", 32).is_err());
    
    // Test zero output length
    assert!(engine.derive_key(b"password", b"valid_salt_123", 0).is_err());
    
    // Test excessive output length
    assert!(engine.derive_key(b"password", b"valid_salt_123", 1024 * 1024 + 1).is_err());
    
    // Test valid inputs
    assert!(engine.derive_key(b"password", b"valid_salt_123", 32).is_ok());
}

#[test]
fn test_kdf_security_assessment() {
    let engines = vec![
        KdfFactory::create_kdf("pbkdf2").unwrap(),
        KdfFactory::create_kdf("argon2").unwrap(),
        KdfFactory::create_kdf("scrypt").unwrap(),
        KdfFactory::create_kdf("hkdf").unwrap(),
    ];
    
    for engine in engines {
        let security_level = engine.security_level();
        assert!(security_level >= 128, "Security level should be at least 128 bits for {}", engine.algorithm_name());
        
        let resistance = engine.attack_resistance();
        assert!(resistance.brute_force >= 128, "Brute force resistance should be at least 128 bits");
        assert!(resistance.dictionary >= 80, "Dictionary attack resistance should be at least 80 bits");
        
        // Check minimum security requirements
        assert!(engine.meets_security_requirements(128));
        assert!(engine.meets_security_requirements(64));
    }
}

#[test]
fn test_kdf_constant_time_operations() {
    let engines = vec![
        KdfFactory::create_kdf("pbkdf2").unwrap(),
        KdfFactory::create_kdf("argon2").unwrap(),
        KdfFactory::create_kdf("scrypt").unwrap(),
        KdfFactory::create_kdf("hkdf").unwrap(),
    ];
    
    for engine in engines {
        // Test constant-time comparison
        let data1 = b"test_data_123";
        let data2 = b"test_data_123";
        let data3 = b"different_data";
        
        assert!(engine.constant_time_eq(data1, data2));
        assert!(!engine.constant_time_eq(data1, data3));
        
        // Check if algorithm claims to be constant-time
        let is_ct = engine.is_constant_time();
        match engine.algorithm_name() {
            "PBKDF2" | "scrypt" | "HKDF" => assert!(is_ct),
            "Argon2" => (), // Argon2d is not constant-time by design
            _ => (),
        }
    }
}

#[test]
fn test_kdf_configuration_parsing() {
    // Test PBKDF2 configuration parsing
    let pbkdf2_configs = vec![
        ("", true), // Empty should use defaults
        ("iterations=100000", true),
        ("iterations=50000,output_len=64", true),
        ("iterations=10000,output_len=32,hash=sha256", true),
        ("iterations=5000,output_len=16,hash=sha512", true),
        ("iterations=invalid", false), // Invalid iteration count
        ("output_len=0", false), // Invalid output length
        ("hash=md5", false), // Unsupported hash
    ];
    
    for (config, should_succeed) in pbkdf2_configs {
        let full_config = format!("pbkdf2:{}", config);
        let result = KdfEngineWrapper::create_engine_from_config(&full_config);
        
        if should_succeed {
            assert!(result.is_ok(), "Should parse config: {}", config);
        } else {
            assert!(result.is_err(), "Should reject config: {}", config);
        }
    }
    
    // Test Argon2 configuration parsing
    let argon2_configs = vec![
        ("", true), // Empty should use defaults
        ("memory=65536", true),
        ("memory=32768,time=3", true),
        ("memory=65536,time=5,parallelism=4", true),
        ("memory=32768,time=2,parallelism=2,output_len=64", true),
        ("memory=0", true), // Will be corrected by validation
        ("time=0", true), // Will be corrected by validation
    ];
    
    for (config, should_succeed) in argon2_configs {
        let full_config = format!("argon2:{}", config);
        let result = KdfEngineWrapper::create_engine_from_config(&full_config);
        
        if should_succeed {
            assert!(result.is_ok(), "Should parse Argon2 config: {}", config);
        } else {
            assert!(result.is_err(), "Should reject Argon2 config: {}", config);
        }
    }
    
    // Test Scrypt configuration parsing
    let scrypt_configs = vec![
        ("", true), // Empty should use defaults
        ("n=32768", true),
        ("n=16384,r=8", true),
        ("n=32768,r=8,p=1", true),
        ("n=65536,r=8,p=1,output_len=64", true),
    ];
    
    for (config, should_succeed) in scrypt_configs {
        let full_config = format!("scrypt:{}", config);
        let result = KdfEngineWrapper::create_engine_from_config(&full_config);
        
        if should_succeed {
            assert!(result.is_ok(), "Should parse Scrypt config: {}", config);
        } else {
            assert!(result.is_err(), "Should reject Scrypt config: {}", config);
        }
    }
}

#[test]
fn test_kdf_deterministic_output() {
    let password = b"consistent_password";
    let salt = b"consistent_salt_123";
    let output_length = 32;
    
    // Test that same inputs produce same outputs
    for algorithm in &["pbkdf2", "argon2", "scrypt", "hkdf"] {
        let engine1 = KdfFactory::create_kdf(algorithm).unwrap();
        let engine2 = KdfFactory::create_kdf(algorithm).unwrap();
        
        let key1 = engine1.derive_key(password, salt, output_length).unwrap();
        let key2 = engine2.derive_key(password, salt, output_length).unwrap();
        
        assert_eq!(key1, key2, "Keys should be deterministic for algorithm: {}", algorithm);
    }
}

#[test]
fn test_kdf_different_salts() {
    let password = b"same_password";
    let salt1 = b"salt_number_one_123";
    let salt2 = b"salt_number_two_456";
    let output_length = 32;
    
    // Test that different salts produce different outputs
    for algorithm in &["pbkdf2", "argon2", "scrypt", "hkdf"] {
        let engine = KdfFactory::create_kdf(algorithm).unwrap();
        
        let key1 = engine.derive_key(password, salt1, output_length).unwrap();
        let key2 = engine.derive_key(password, salt2, output_length).unwrap();
        
        assert_ne!(key1, key2, "Different salts should produce different keys for: {}", algorithm);
    }
}

#[test]
fn test_kdf_utils_functionality() {
    // Test salt generation
    let salt1 = KdfUtils::generate_salt(16).unwrap();
    let salt2 = KdfUtils::generate_salt(16).unwrap();
    
    assert_eq!(salt1.len(), 16);
    assert_eq!(salt2.len(), 16);
    assert_ne!(salt1, salt2); // Should be random
    
    // Test different lengths
    let salt_32 = KdfUtils::generate_salt(32).unwrap();
    assert_eq!(salt_32.len(), 32);
    
    // Test invalid lengths
    assert!(KdfUtils::generate_salt(0).is_err());
    assert!(KdfUtils::generate_salt(1025).is_err());
    
    // Test password strength validation
    let passwords = vec![
        (b"123".as_slice(), PasswordStrength::Weak),
        (b"password".as_slice(), PasswordStrength::Fair),
        (b"Password123".as_slice(), PasswordStrength::Good),
        (b"MyStr0ng!P@ssw0rd".as_slice(), PasswordStrength::Strong),
    ];
    
    for (password, expected_strength) in passwords {
        let strength = KdfUtils::validate_password_strength(password);
        assert_eq!(strength, expected_strength, "Wrong strength for password");
    }
    
    // Test entropy calculation
    let low_entropy = b"aaaaaaaaaaaa";
    let high_entropy = b"aB3$9mK2pX7Q";
    
    let low_ent = KdfUtils::calculate_entropy(low_entropy);
    let high_ent = KdfUtils::calculate_entropy(high_entropy);
    
    assert!(high_ent > low_ent, "High entropy string should have higher entropy");
    assert!(low_ent < 2.0, "Low entropy should be very low");
    assert!(high_ent > 3.0, "High entropy should be reasonably high");
    
    // Test secure comparison
    let data1 = b"test_data_for_comparison";
    let data2 = b"test_data_for_comparison";
    let data3 = b"different_data_entirely";
    
    assert!(KdfUtils::secure_compare(data1, data2));
    assert!(!KdfUtils::secure_compare(data1, data3));
    assert!(!KdfUtils::secure_compare(data1, b"short"));
}

#[test]
fn test_password_strength_scoring() {
    let strengths = vec![
        PasswordStrength::Weak,
        PasswordStrength::Fair,
        PasswordStrength::Good,
        PasswordStrength::Strong,
    ];
    
    for (i, strength) in strengths.iter().enumerate() {
        assert_eq!(strength.score(), (i + 1) as u32);
        assert!(!strength.description().is_empty());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_kdf_integration_with_crypto_ecosystem() {
        // Test that KDF engines work with the broader crypto ecosystem
        
        // Generate a derived key
        let engine = KdfFactory::create_kdf("argon2").unwrap();
        let derived_key = engine.derive_key(b"master_password", b"unique_salt_123", 32).unwrap();
        
        // Use derived key with symmetric crypto (mock test)
        assert_eq!(derived_key.len(), 32);
        assert_ne!(derived_key, vec![0u8; 32]); // Should not be all zeros
        
        // Test key derivation consistency
        let same_key = engine.derive_key(b"master_password", b"unique_salt_123", 32).unwrap();
        assert_eq!(derived_key, same_key);
        
        // Test different parameters produce different keys
        let different_key = engine.derive_key(b"master_password", b"different_salt", 32).unwrap();
        assert_ne!(derived_key, different_key);
    }
    
    #[test]
    fn test_all_kdf_algorithms_work() {
        let algorithms = KdfFactory::available_algorithms();
        let password = b"test_password_for_all_algorithms";
        let salt = b"test_salt_12345678";
        let output_length = 32;
        
        for algorithm in algorithms {
            let engine = KdfFactory::create_kdf(algorithm).unwrap();
            let key = engine.derive_key(password, salt, output_length).unwrap();
            
            assert_eq!(key.len(), output_length, "Algorithm {} failed", algorithm);
            assert_ne!(key, vec![0u8; output_length], "Algorithm {} produced all zeros", algorithm);
        }
    }
}
