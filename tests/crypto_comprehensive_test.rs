/// fr fr Comprehensive test for all CURSED crypto packages
/// 
/// This test validates that all crypto modules are properly implemented
/// and working together as a complete cryptographic ecosystem.

use cursed::stdlib::packages::{
    crypto_random::*,
    crypto_hash_advanced::*,
    crypto_advanced::*,
};
use cursed::error::CursedError;

// Test crypto_random - foundation module
#[test]
fn test_crypto_random_functionality() {
    // Test basic random generation
    let random_bytes = generate_random_bytes(32).expect("Should generate random bytes");
    assert_eq!(random_bytes.len(), 32);

    // Test that it's not all zeros (our old stub behavior)
    assert!(random_bytes.iter().any(|&b| b != 0), "Random bytes should not be all zeros");

    // Test random u32
    let random_u32 = generate_random_u32().expect("Should generate random u32");
    println!("Generated random u32: {}", random_u32);

    // Test random u64
    let random_u64 = generate_random_u64().expect("Should generate random u64");
    println!("Generated random u64: {}", random_u64);

    // Test random boolean
    let random_bool = generate_random_bool().expect("Should generate random bool");
    println!("Generated random bool: {}", random_bool);

    // Test random range
    let random_range = generate_random_range(1, 100).expect("Should generate random in range");
    assert!(random_range >= 1 && random_range < 100, "Random range should be in bounds");

    // Test entropy quality
    let entropy_ok = test_entropy_quality().expect("Should test entropy quality");
    assert!(entropy_ok, "Entropy quality should be good");

    // Test RNG health
    verify_rng_health().expect("RNG should be healthy");

    // Test hex encoding
    let hex_string = generate_random_hex(16).expect("Should generate hex string");
    assert_eq!(hex_string.len(), 32); // 16 bytes = 32 hex chars

    // Test base64 encoding
    let base64_string = generate_random_base64(16).expect("Should generate base64 string");
    assert!(!base64_string.is_empty());

    println!("✅ crypto_random tests passed");
}

// Test crypto_hash_advanced - core hashing
#[test]
fn test_crypto_hash_advanced_functionality() {
    // Test SHA-3 variants
    let test_data = b"Hello, CURSED crypto world!";
    
    // SHA3-256
    let sha3_256_hash = sha3::Sha3Hasher::hash(sha3::Sha3Variant::Sha3_256, test_data);
    assert_eq!(sha3_256_hash.len(), 32);
    assert!(sha3_256_hash.iter().any(|&b| b != 0), "Hash should not be all zeros");

    // SHA3-512
    let sha3_512_hash = sha3::Sha3Hasher::hash(sha3::Sha3Variant::Sha3_512, test_data);
    assert_eq!(sha3_512_hash.len(), 64);

    // SHAKE128 with custom length
    let shake128_hash = sha3::Sha3Hasher::shake(sha3::Sha3Variant::Shake128, test_data, 20);
    assert_eq!(shake128_hash.len(), 20);

    // SHAKE256 with custom length
    let shake256_hash = sha3::Sha3Hasher::shake(sha3::Sha3Variant::Shake256, test_data, 50);
    assert_eq!(shake256_hash.len(), 50);

    // Test streaming SHA-3
    let mut hasher = sha3::Sha3Hasher::new(sha3::Sha3Variant::Sha3_256);
    hasher.update(b"Hello, ");
    hasher.update(b"CURSED ");
    hasher.update(b"crypto ");
    hasher.update(b"world!");
    let streaming_hash = hasher.finalize();
    assert_eq!(streaming_hash, sha3_256_hash);

    // Test BLAKE3
    let blake3_hash = blake3::Blake3Hasher::hash(test_data);
    assert_eq!(blake3_hash.len(), 32);
    assert!(blake3_hash.iter().any(|&b| b != 0), "BLAKE3 hash should not be all zeros");

    // Test BLAKE3 keyed hashing
    let key = blake3::Blake3Utils::generate_key();
    let keyed_hash = blake3::Blake3Hasher::keyed_hash(&key, test_data);
    assert_eq!(keyed_hash.len(), 32);
    assert_ne!(keyed_hash, blake3_hash); // Should be different from unkeyed

    // Test BLAKE3 key derivation
    let derived_key = blake3::Blake3Hasher::derive_key("test context", test_data, 64);
    assert_eq!(derived_key.len(), 64);

    // Test BLAKE3 streaming
    let mut blake3_hasher = blake3::Blake3Hasher::new();
    blake3_hasher.update(b"Hello, ");
    blake3_hasher.update(b"CURSED ");
    blake3_hasher.update(b"crypto ");
    blake3_hasher.update(b"world!");
    let blake3_streaming = blake3_hasher.finalize_fixed();
    assert_eq!(blake3_streaming, blake3_hash);

    println!("✅ crypto_hash_advanced tests passed");
}

// Test crypto_advanced - symmetric encryption
#[test]
fn test_crypto_advanced_functionality() {
    let test_plaintext = b"This is a secret message that needs encryption!";

    // Test AES-256-GCM
    let key256 = aes_gcm::AesGcm256::generate_key();
    let cipher256 = aes_gcm::AesGcm256::new(&key256).expect("Should create AES-256-GCM cipher");
    
    let ciphertext256 = cipher256.encrypt(test_plaintext).expect("Should encrypt with AES-256-GCM");
    assert!(ciphertext256.len() > test_plaintext.len()); // Should include nonce and tag
    
    let decrypted256 = cipher256.decrypt(&ciphertext256).expect("Should decrypt with AES-256-GCM");
    assert_eq!(decrypted256, test_plaintext);

    // Test AES-192-GCM
    let key192 = aes_gcm::AesGcm192::generate_key();
    let cipher192 = aes_gcm::AesGcm192::new(&key192).expect("Should create AES-192-GCM cipher");
    
    let ciphertext192 = cipher192.encrypt(test_plaintext).expect("Should encrypt with AES-192-GCM");
    let decrypted192 = cipher192.decrypt(&ciphertext192).expect("Should decrypt with AES-192-GCM");
    assert_eq!(decrypted192, test_plaintext);

    // Test AES-128-GCM
    let key128 = aes_gcm::AesGcm128::generate_key();
    let cipher128 = aes_gcm::AesGcm128::new(&key128).expect("Should create AES-128-GCM cipher");
    
    let ciphertext128 = cipher128.encrypt(test_plaintext).expect("Should encrypt with AES-128-GCM");
    let decrypted128 = cipher128.decrypt(&ciphertext128).expect("Should decrypt with AES-128-GCM");
    assert_eq!(decrypted128, test_plaintext);

    // Test AEAD with additional data
    let additional_data = b"This is public metadata";
    let encrypted_aead = cipher256.encrypt_aead(test_plaintext, Some(additional_data))
        .expect("Should encrypt with AEAD");
    let decrypted_aead = cipher256.decrypt_aead(&encrypted_aead)
        .expect("Should decrypt with AEAD");
    assert_eq!(decrypted_aead, test_plaintext);

    // Test key derivation from password
    let password = b"strong_password_123";
    let salt = b"random_salt_16_bytes_or_more";
    let derived_key = aes_gcm::AesGcm256::derive_key_from_password(password, salt, 10000)
        .expect("Should derive key from password");
    assert_eq!(derived_key.len(), 32);

    // Test utility functions
    let quick_encrypted = aes_gcm::utils::quick_encrypt_256(&key256, test_plaintext)
        .expect("Should quick encrypt");
    let quick_decrypted = aes_gcm::utils::quick_decrypt_256(&key256, &quick_encrypted)
        .expect("Should quick decrypt");
    assert_eq!(quick_decrypted, test_plaintext);

    // Test encryption with AAD utility
    let aad = b"Public metadata for utility test";
    let encrypted_with_aad = aes_gcm::utils::encrypt_with_aad(&key256, test_plaintext, aad)
        .expect("Should encrypt with AAD");
    let (decrypted_with_aad, recovered_aad) = aes_gcm::utils::decrypt_with_aad(&key256, &encrypted_with_aad)
        .expect("Should decrypt with AAD");
    assert_eq!(decrypted_with_aad, test_plaintext);
    assert_eq!(recovered_aad, aad);

    println!("✅ crypto_advanced tests passed");
}

// Test cross-module integration
#[test]
fn test_crypto_integration() {
    println!("Testing crypto module integration...");

    // Generate random key using crypto_random
    let random_key = generate_random_bytes(32).expect("Should generate random key");
    
    // Use random key for encryption
    let cipher = aes_gcm::AesGcm256::new(&random_key).expect("Should create cipher with random key");
    let test_message = b"Integration test message with random key";
    
    let encrypted = cipher.encrypt(test_message).expect("Should encrypt");
    let decrypted = cipher.decrypt(&encrypted).expect("Should decrypt");
    assert_eq!(decrypted, test_message);

    // Hash the encrypted data with SHA-3
    let hash = sha3::Sha3Hasher::hash(sha3::Sha3Variant::Sha3_256, &encrypted);
    assert_eq!(hash.len(), 32);

    // Hash the same data with BLAKE3
    let blake3_hash = blake3::Blake3Hasher::hash(&encrypted);
    assert_eq!(blake3_hash.len(), 32);
    assert_ne!(hash, blake3_hash); // Different algorithms should produce different results

    // Use BLAKE3 for key derivation
    let derived_key = blake3::Blake3Hasher::derive_key("integration test", &random_key, 32);
    assert_eq!(derived_key.len(), 32);
    assert_ne!(derived_key, random_key.as_slice()); // Should be different from input

    // Use derived key for another encryption
    let cipher2 = aes_gcm::AesGcm256::new(&derived_key).expect("Should create cipher with derived key");
    let encrypted2 = cipher2.encrypt(test_message).expect("Should encrypt with derived key");
    let decrypted2 = cipher2.decrypt(&encrypted2).expect("Should decrypt with derived key");
    assert_eq!(decrypted2, test_message);

    // Encrypted data should be different with different keys
    assert_ne!(encrypted, encrypted2);

    println!("✅ crypto integration tests passed");
}

// Test error conditions
#[test]
fn test_crypto_error_handling() {
    // Test invalid key sizes
    let invalid_key = vec![0u8; 15]; // Too short for AES-128
    let result = aes_gcm::AesGcm128::new(&invalid_key);
    assert!(result.is_err());

    let invalid_key256 = vec![0u8; 31]; // Too short for AES-256
    let result256 = aes_gcm::AesGcm256::new(&invalid_key256);
    assert!(result256.is_err());

    // Test invalid random range
    let invalid_range = generate_random_range(100, 50); // min > max
    assert!(invalid_range.is_err());

    // Test oversized random bytes
    let oversized = generate_random_bytes(2 * 1024 * 1024); // > 1MB limit
    assert!(oversized.is_err());

    // Test invalid SHAKE output length
    let invalid_shake = sha3::Sha3Hasher::shake(sha3::Sha3Variant::Shake128, b"test", 0);
    // This would return empty vector, but let's test a huge length
    let huge_shake = sha3::Sha3Hasher::shake(sha3::Sha3Variant::Shake128, b"test", 1024 * 1024);
    assert_eq!(huge_shake.len(), 1024 * 1024); // Should handle large outputs

    println!("✅ crypto error handling tests passed");
}

// Test performance characteristics
#[test]
fn test_crypto_performance() {
    use std::time::Instant;

    // Test encryption performance
    let key = aes_gcm::AesGcm256::generate_key();
    let cipher = aes_gcm::AesGcm256::new(&key).expect("Should create cipher");
    let test_data = vec![0u8; 1024]; // 1KB of test data

    let start = Instant::now();
    for _ in 0..100 {
        let encrypted = cipher.encrypt(&test_data).expect("Should encrypt");
        let _decrypted = cipher.decrypt(&encrypted).expect("Should decrypt");
    }
    let duration = start.elapsed();
    
    println!("100 encryption/decryption cycles took: {:?}", duration);
    assert!(duration.as_millis() < 5000, "Should complete in reasonable time");

    // Test hashing performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _hash = sha3::Sha3Hasher::hash(sha3::Sha3Variant::Sha3_256, &test_data);
    }
    let duration = start.elapsed();
    
    println!("1000 SHA-3 hashes took: {:?}", duration);
    assert!(duration.as_millis() < 5000, "Should complete in reasonable time");

    // Test BLAKE3 performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _hash = blake3::Blake3Hasher::hash(&test_data);
    }
    let duration = start.elapsed();
    
    println!("1000 BLAKE3 hashes took: {:?}", duration);
    assert!(duration.as_millis() < 2000, "BLAKE3 should be faster");

    println!("✅ crypto performance tests passed");
}

// Test initialization of all modules
#[test]
fn test_crypto_initialization() {
    // Test crypto_random initialization
    let random_init = crate::cursed::stdlib::packages::crypto_random::init_crypto_random();
    assert!(random_init.is_ok());

    // Test crypto_hash_advanced initialization
    let hash_init = crate::cursed::stdlib::packages::crypto_hash_advanced::init_crypto_hash_advanced();
    assert!(hash_init.is_ok());

    // Test crypto_advanced initialization
    let advanced_init = crate::cursed::stdlib::packages::crypto_advanced::init_crypto_advanced();
    assert!(advanced_init.is_ok());

    println!("✅ crypto initialization tests passed");
}

// Main comprehensive test runner
#[test]
fn test_all_crypto_functionality() {
    println!("🔐 Starting comprehensive CURSED crypto test suite...");
    
    test_crypto_random_functionality();
    test_crypto_hash_advanced_functionality();
    test_crypto_advanced_functionality();
    test_crypto_integration();
    test_crypto_error_handling();
    test_crypto_performance();
    
    println!("🎉 All CURSED crypto tests passed! The crypto ecosystem is functioning properly.");
}
