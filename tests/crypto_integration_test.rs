/// fr fr Comprehensive crypto package integration tests for CURSED
/// 
/// This test suite validates the entire crypto ecosystem including:
/// - Cross-package integration and compatibility
/// - End-to-end encryption workflows  
/// - Performance characteristics and security properties
/// - Error handling and edge cases
/// 
/// These tests ensure the crypto infrastructure works correctly in production scenarios.

use cursed::stdlib::crypto::*;
use cursed::stdlib::packages::crypto_random::*;
use cursed::stdlib::packages::crypto_advanced::*;
use cursed::stdlib::packages::crypto_asymmetric::*;
use cursed::stdlib::packages::crypto_hash_advanced::*;
use cursed::stdlib::packages::crypto_kdf::*;
use cursed::stdlib::packages::crypto_signatures::*;
use cursed::stdlib::value::Value;
use std::time::Instant;
use std::collections::HashMap;

// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_crypto_package_initialization() {
    init_tracing!();
    tracing::info!("Testing crypto package initialization");
    
    // Test that all crypto packages can be initialized
    assert!(init_crypto().is_ok(), "Failed to initialize crypto ecosystem");
    
    // Test package manager functionality
    let package_manager = global_package_manager();
    let packages = package_manager.list_packages();
    assert!(packages.is_ok(), "Failed to list crypto packages");
    
    let package_list = packages.unwrap();
    assert!(!package_list.is_empty(), "No crypto packages found");
    
    // Verify key packages are present
    let package_names: Vec<String> = package_list.iter().map(|p| p.name.clone()).collect();
    let required_packages = ["crypto_random", "crypto_advanced", "crypto_asymmetric", 
                           "crypto_hash_advanced", "crypto_kdf", "crypto_signatures"];
    
    for required in required_packages {
        assert!(package_names.iter().any(|name| name.contains(required)), 
               "Required package {} not found", required);
    }
    
    tracing::info!(packages_found = package_list.len(), "Crypto packages initialized successfully");
}

#[test]
fn test_end_to_end_symmetric_encryption() {
    init_tracing!();
    tracing::info!("Testing end-to-end symmetric encryption workflow");
    
    let plaintext = b"Hello, CURSED crypto world! This is a test message for encryption.";
    let associated_data = b"additional authenticated data";
    
    // Test AES-256-GCM workflow
    let start_time = Instant::now();
    let aes_key = vec![42u8; 32]; // Test key
    let aes_cipher = Aes256Gcm::new(&aes_key).expect("Failed to create AES-256-GCM cipher");
    
    let encrypted = aes_cipher.encrypt(plaintext, associated_data)
        .expect("Failed to encrypt with AES-256-GCM");
    
    assert!(!encrypted.ciphertext.is_empty(), "Ciphertext should not be empty");
    assert!(encrypted.nonce.is_some(), "Nonce should be present");
    assert!(encrypted.tag.is_some(), "Authentication tag should be present");
    assert_eq!(encrypted.algorithm, "AES-256", "Algorithm should be AES-256");
    assert_eq!(encrypted.mode, "GCM", "Mode should be GCM");
    
    let decrypted = aes_cipher.decrypt(&encrypted.ciphertext, associated_data, &encrypted)
        .expect("Failed to decrypt with AES-256-GCM");
    
    assert_eq!(decrypted.plaintext, plaintext, "Decrypted text should match original");
    assert!(decrypted.verified, "Authentication should be verified");
    
    let aes_duration = start_time.elapsed();
    tracing::info!(duration_ms = aes_duration.as_millis(), "AES-256-GCM roundtrip completed");
    
    // Test ChaCha20-Poly1305 workflow
    let start_time = Instant::now();
    let chacha_key = vec![123u8; 32]; // Test key
    let chacha_cipher = ChaCha20Poly1305Aead::new(&chacha_key)
        .expect("Failed to create ChaCha20-Poly1305 cipher");
    
    let encrypted = chacha_cipher.encrypt(plaintext, associated_data)
        .expect("Failed to encrypt with ChaCha20-Poly1305");
    
    assert!(!encrypted.ciphertext.is_empty(), "Ciphertext should not be empty");
    assert!(encrypted.nonce.is_some(), "Nonce should be present");
    assert!(encrypted.tag.is_some(), "Authentication tag should be present");
    assert_eq!(encrypted.algorithm, "ChaCha20-Poly1305", "Algorithm should be ChaCha20-Poly1305");
    
    let decrypted = chacha_cipher.decrypt(&encrypted.ciphertext, associated_data, &encrypted)
        .expect("Failed to decrypt with ChaCha20-Poly1305");
    
    assert_eq!(decrypted.plaintext, plaintext, "Decrypted text should match original");
    assert!(decrypted.verified, "Authentication should be verified");
    
    let chacha_duration = start_time.elapsed();
    tracing::info!(duration_ms = chacha_duration.as_millis(), "ChaCha20-Poly1305 roundtrip completed");
    
    // Verify different algorithms produce different ciphertext
    let aes_encrypted = aes_cipher.encrypt(plaintext, associated_data).unwrap();
    let chacha_encrypted = chacha_cipher.encrypt(plaintext, associated_data).unwrap();
    assert_ne!(aes_encrypted.ciphertext, chacha_encrypted.ciphertext, 
              "Different algorithms should produce different ciphertext");
}

#[test]
fn test_asymmetric_cryptography_workflow() {
    init_tracing!();
    tracing::info!("Testing asymmetric cryptography workflow");
    
    // Test RSA key generation and encryption
    let start_time = Instant::now();
    let rsa_result = rsa_generate_keypair(vec![Value::Number(2048.0)]);
    assert!(rsa_result.is_ok(), "Failed to generate RSA keypair");
    
    let rsa_duration = start_time.elapsed();
    tracing::info!(duration_ms = rsa_duration.as_millis(), "RSA-2048 keypair generated");
    
    // Test ECDSA key generation and signing
    let start_time = Instant::now();
    let ecdsa_result = ecdsa_generate_keypair(vec![Value::String("P-256".to_string())]);
    assert!(ecdsa_result.is_ok(), "Failed to generate ECDSA keypair");
    
    let ecdsa_duration = start_time.elapsed();
    tracing::info!(duration_ms = ecdsa_duration.as_millis(), "ECDSA P-256 keypair generated");
    
    // Test Ed25519 key generation (fastest)
    let start_time = Instant::now();
    let ed25519_result = ed25519_generate_keypair(vec![]);
    assert!(ed25519_result.is_ok(), "Failed to generate Ed25519 keypair");
    
    let ed25519_duration = start_time.elapsed();
    tracing::info!(duration_ms = ed25519_duration.as_millis(), "Ed25519 keypair generated");
    
    // Test X25519 key exchange
    let start_time = Instant::now();
    let x25519_result = x25519_generate_keypair(vec![]);
    assert!(x25519_result.is_ok(), "Failed to generate X25519 keypair");
    
    let x25519_duration = start_time.elapsed();
    tracing::info!(duration_ms = x25519_duration.as_millis(), "X25519 keypair generated");
    
    // Performance comparison
    tracing::info!(
        rsa_ms = rsa_duration.as_millis(),
        ecdsa_ms = ecdsa_duration.as_millis(), 
        ed25519_ms = ed25519_duration.as_millis(),
        x25519_ms = x25519_duration.as_millis(),
        "Asymmetric crypto performance comparison"
    );
}

#[test]
fn test_hash_function_compatibility() {
    init_tracing!();
    tracing::info!("Testing hash function compatibility and performance");
    
    let test_data = b"The quick brown fox jumps over the lazy dog";
    
    // Test SHA-256
    let start_time = Instant::now();
    let sha256_hash = Sha256::hash(test_data);
    let sha256_duration = start_time.elapsed();
    assert_eq!(sha256_hash.len(), 32, "SHA-256 hash should be 32 bytes");
    
    // Test SHA-512
    let start_time = Instant::now();
    let sha512_hash = Sha512::hash(test_data);
    let sha512_duration = start_time.elapsed();
    assert_eq!(sha512_hash.len(), 64, "SHA-512 hash should be 64 bytes");
    
    // Test MD5 (for compatibility)
    let start_time = Instant::now();
    let md5_hash = Md5::hash(test_data);
    let md5_duration = start_time.elapsed();
    assert_eq!(md5_hash.len(), 16, "MD5 hash should be 16 bytes");
    
    // Verify known test vectors
    let sha256_hex = HashUtils::to_hex(&sha256_hash);
    assert_eq!(sha256_hex, "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592",
              "SHA-256 test vector should match");
    
    tracing::info!(
        sha256_us = sha256_duration.as_micros(),
        sha512_us = sha512_duration.as_micros(),
        md5_us = md5_duration.as_micros(),
        "Hash function performance comparison"
    );
    
    // Test constant-time comparison
    let hash1 = Sha256::hash(b"test1");
    let hash2 = Sha256::hash(b"test2");
    let hash3 = Sha256::hash(b"test1"); // Same as hash1
    
    assert!(!HashUtils::constant_time_compare(&hash1, &hash2), "Different hashes should not match");
    assert!(HashUtils::constant_time_compare(&hash1, &hash3), "Same hashes should match");
}

#[test]
fn test_random_number_generation_quality() {
    init_tracing!();
    tracing::info!("Testing random number generation quality and performance");
    
    // Test basic functionality
    let mut buffer = vec![0u8; 1024];
    let result = fill_random(&mut buffer);
    assert!(result.is_ok(), "Random fill should succeed");
    
    // Verify randomness quality (basic test)
    let zero_count = buffer.iter().filter(|&&b| b == 0).count();
    assert!(zero_count < buffer.len() / 2, "Too many zero bytes in random data");
    
    // Test different sizes
    let sizes = [16, 32, 64, 128, 256, 512, 1024];
    for size in sizes {
        let start_time = Instant::now();
        let random_bytes = generate_random_bytes(size);
        let duration = start_time.elapsed();
        
        assert!(random_bytes.is_ok(), "Failed to generate {} random bytes", size);
        assert_eq!(random_bytes.unwrap().len(), size, "Generated wrong number of bytes");
        
        tracing::debug!(size = size, duration_us = duration.as_micros(), 
                       "Random generation performance");
    }
    
    // Test entropy quality
    let entropy_result = test_entropy_quality();
    assert!(entropy_result.is_ok(), "Entropy quality test failed");
    assert!(entropy_result.unwrap(), "RNG failed entropy quality check");
    
    // Test RNG health verification
    let health_result = verify_rng_health();
    assert!(health_result.is_ok(), "RNG health verification failed: {:?}", health_result);
    
    tracing::info!("Random number generation quality tests passed");
}

#[test]
fn test_key_derivation_functions() {
    init_tracing!();
    tracing::info!("Testing key derivation functions");
    
    let password = b"secure_password_123";
    let salt = b"random_salt_data_456";
    
    // Test key manager creation
    let key_manager = KeyManager::new();
    assert!(key_manager.is_ok(), "Failed to create key manager");
    let manager = key_manager.unwrap();
    
    // Test PBKDF2 key derivation
    let start_time = Instant::now();
    let config = KeyDerivationConfig {
        iterations: 10000, // Reduced for testing
        salt: salt.to_vec(),
        key_length: 32,
    };
    
    let pbkdf2_key = manager.derive_key_pbkdf2(password, &config);
    let pbkdf2_duration = start_time.elapsed();
    assert!(pbkdf2_key.is_ok(), "PBKDF2 key derivation failed");
    
    let derived = pbkdf2_key.unwrap();
    assert_eq!(derived.size(), 32, "Derived key should be 32 bytes");
    assert_eq!(derived.algorithm(), "PBKDF2", "Algorithm should be PBKDF2");
    
    // Test scrypt key derivation
    let start_time = Instant::now();
    let scrypt_key = manager.derive_key_scrypt(password, &config);
    let scrypt_duration = start_time.elapsed();
    assert!(scrypt_key.is_ok(), "scrypt key derivation failed");
    
    let scrypt_derived = scrypt_key.unwrap();
    assert_eq!(scrypt_derived.size(), 32, "Derived key should be 32 bytes");
    assert_eq!(scrypt_derived.algorithm(), "scrypt", "Algorithm should be scrypt");
    
    // Verify different algorithms produce different keys
    assert_ne!(derived.as_bytes(), scrypt_derived.as_bytes(), 
              "Different KDF algorithms should produce different keys");
    
    tracing::info!(
        pbkdf2_ms = pbkdf2_duration.as_millis(),
        scrypt_ms = scrypt_duration.as_millis(),
        "Key derivation performance comparison"
    );
}

#[test] 
fn test_performance_benchmarks() {
    init_tracing!();
    tracing::info!("Running crypto performance benchmarks");
    
    let test_data = vec![0u8; 1024]; // 1KB test data
    let iterations = 100;
    
    // Benchmark symmetric encryption
    let aes_key = vec![42u8; 32];
    let aes_cipher = Aes256Gcm::new(&aes_key).unwrap();
    
    let start_time = Instant::now();
    for _ in 0..iterations {
        let encrypted = aes_cipher.encrypt(&test_data, b"").unwrap();
        let _decrypted = aes_cipher.decrypt(&encrypted.ciphertext, b"", &encrypted).unwrap();
    }
    let aes_total = start_time.elapsed();
    let aes_per_op = aes_total / iterations;
    
    // Benchmark hashing
    let start_time = Instant::now();
    for _ in 0..iterations {
        let _hash = Sha256::hash(&test_data);
    }
    let hash_total = start_time.elapsed();
    let hash_per_op = hash_total / iterations;
    
    // Benchmark random generation
    let start_time = Instant::now();
    for _ in 0..iterations {
        let _random = generate_random_bytes(32).unwrap();
    }
    let random_total = start_time.elapsed();
    let random_per_op = random_total / iterations;
    
    tracing::info!(
        aes_roundtrip_us = aes_per_op.as_micros(),
        hash_us = hash_per_op.as_micros(),
        random_gen_us = random_per_op.as_micros(),
        test_data_size = test_data.len(),
        iterations = iterations,
        "Crypto performance benchmarks completed"
    );
    
    // Performance assertions (should complete within reasonable time)
    assert!(aes_per_op.as_millis() < 10, "AES roundtrip should be under 10ms");
    assert!(hash_per_op.as_micros() < 1000, "SHA-256 should be under 1ms");
    assert!(random_per_op.as_micros() < 100, "Random generation should be under 100μs");
}

#[test]
fn test_error_handling_and_edge_cases() {
    init_tracing!();
    tracing::info!("Testing error handling and edge cases");
    
    // Test invalid key sizes
    let invalid_key = vec![0u8; 16]; // Wrong size for AES-256
    let aes_result = Aes256Gcm::new(&invalid_key);
    assert!(aes_result.is_err(), "Should reject invalid key size");
    
    let chacha_result = ChaCha20Poly1305Aead::new(&invalid_key);
    assert!(chacha_result.is_err(), "Should reject invalid key size");
    
    // Test empty data encryption
    let valid_key = vec![42u8; 32];
    let cipher = Aes256Gcm::new(&valid_key).unwrap();
    let empty_encrypted = cipher.encrypt(b"", b"");
    assert!(empty_encrypted.is_ok(), "Should handle empty plaintext");
    
    // Test large data generation limits
    let large_result = generate_random_bytes(2 * 1024 * 1024); // 2MB
    assert!(large_result.is_err(), "Should reject oversized random generation");
    
    // Test invalid ranges
    let range_result = generate_random_range(10, 5); // min > max
    assert!(range_result.is_err(), "Should reject invalid range");
    
    // Test hash algorithm security properties
    assert!(HashAlgorithm::Sha256.is_secure(), "SHA-256 should be secure");
    assert!(HashAlgorithm::Sha512.is_secure(), "SHA-512 should be secure");
    assert!(!HashAlgorithm::Md5.is_secure(), "MD5 should not be secure");
    
    tracing::info!("Error handling tests completed successfully");
}

#[test]
fn test_concurrent_crypto_operations() {
    use std::sync::Arc;
    use std::thread;
    
    init_tracing!();
    tracing::info!("Testing concurrent crypto operations");
    
    let num_threads = 8;
    let operations_per_thread = 10;
    
    let aes_key = Arc::new(vec![42u8; 32]);
    let test_data = Arc::new(b"Concurrent crypto test data".to_vec());
    
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let key = Arc::clone(&aes_key);
        let data = Arc::clone(&test_data);
        
        thread::spawn(move || {
            tracing::debug!(thread_id = thread_id, "Starting crypto operations");
            
            let cipher = Aes256Gcm::new(&key).expect("Failed to create cipher");
            
            for op_id in 0..operations_per_thread {
                // Encryption
                let encrypted = cipher.encrypt(&data, b"").expect("Encryption failed");
                
                // Decryption
                let decrypted = cipher.decrypt(&encrypted.ciphertext, b"", &encrypted)
                    .expect("Decryption failed");
                
                assert_eq!(decrypted.plaintext, data.as_slice(), "Concurrent decryption mismatch");
                
                // Random generation
                let _random = generate_random_bytes(32).expect("Random generation failed");
                
                // Hashing
                let _hash = Sha256::hash(&data);
                
                tracing::trace!(thread_id = thread_id, operation = op_id, "Operation completed");
            }
            
            tracing::debug!(thread_id = thread_id, "Thread completed successfully");
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    tracing::info!(
        threads = num_threads,
        operations_per_thread = operations_per_thread,
        total_operations = num_threads * operations_per_thread,
        "Concurrent crypto operations completed successfully"
    );
}

#[test]
fn test_package_integration() {
    init_tracing!();
    tracing::info!("Testing crypto package integration");
    
    // Test integration manager
    let integration_manager = global_integration_manager();
    let compatibility_matrix = integration_manager.get_compatibility_matrix();
    assert!(compatibility_matrix.is_ok(), "Failed to get compatibility matrix");
    
    // Test unified crypto manager
    let crypto_manager = global_crypto_manager();
    let algorithms = crypto_manager.list_available_algorithms();
    assert!(!algorithms.is_empty(), "No algorithms available from unified manager");
    
    // Verify key algorithm categories are present
    let expected_categories = ["symmetric", "asymmetric", "hash", "kdf", "signature"];
    for category in expected_categories {
        assert!(algorithms.contains_key(category), "Missing algorithm category: {}", category);
    }
    
    // Test performance statistics
    let stats_result = crypto_manager.get_performance_statistics();
    assert!(stats_result.is_ok(), "Failed to get performance statistics");
    
    // Test security audit
    let audit_result = crypto_manager.get_latest_audit();
    assert!(audit_result.is_ok(), "Failed to get security audit");
    
    tracing::info!("Package integration tests completed successfully");
}
