/// fr fr Comprehensive crypto integration tests - all modules working together bestie
///
/// This test suite validates the entire CURSED crypto package ecosystem,
/// ensuring all modules work together seamlessly for real-world scenarios.

#[path = "common.rs"]
pub mod common;

use common::init_tracing;
use cursed::stdlib::packages::{
    // Advanced symmetric crypto
    crypto_advanced::{
        AesGcm256, ChaCha20Poly1305, XChaCha20Poly1305, 
        register_cipher, get_cipher, SecurityLevel,
        init_crypto_advanced, ConstantTimeOps, SecureMemory
    },
    // Asymmetric crypto
    crypto_asymmetric::{
        AsymmetricAlgorithm, RsaKeyPair, EcKeyPair, Ed25519KeyPair,
        KeyGenerator, init_crypto_asymmetric
    },
    // Digital signatures
    crypto_signatures::{
        SignatureAlgorithm, DigitalSignature, SignatureVerification,
        init_crypto_signatures
    },
    // Key derivation
    crypto_kdf::{
        KdfAlgorithm, derive_key, pbkdf2_derive, scrypt_derive,
        argon2_derive, init_crypto_kdf
    },
    // Advanced hashing
    crypto_hash_advanced::{
        AdvancedHashAlgorithm, hash_with_algorithm, compute_hmac,
        init_crypto_hash_advanced
    },
    // Cryptographic random
    crypto_random::{
        RandomRequest, CryptographicRng, fill_random,
        RandomPurpose, RandomQuality
    },
    // Zero-knowledge proofs
    crypto_zk::{
        ZkProofSystem, ZkProof, ZkVerifier, init_crypto_zk
    },
    // Post-quantum crypto
    crypto_pqc::{
        PqcAlgorithm, QuantumThreatLevel, assess_quantum_threat,
        init_crypto_pqc
    },
    // PKI infrastructure
    crypto_pki::{
        Certificate, CertificateAuthority, TrustChain,
        init_crypto_pki
    },
    // Cryptographic protocols
    crypto_protocols::{
        CryptoProtocol, KeyExchangeProtocol, SecureChannel,
        HandshakeProtocol
    },
};
use tracing::{info, debug, error};
use std::time::Instant;
use std::collections::HashMap;

/// slay Initialize all crypto packages
fn setup_crypto_packages() -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up all crypto packages...");
    
    init_crypto_advanced()?;
    init_crypto_asymmetric()?;
    init_crypto_signatures()?;
    init_crypto_kdf()?;
    init_crypto_hash_advanced()?;
    init_crypto_zk()?;
    init_crypto_pqc()?;
    init_crypto_pki()?;
    
    info!("All crypto packages initialized successfully!");
    Ok(())
}

/// slay Test comprehensive end-to-end encryption workflow
#[test]
fn test_end_to_end_encryption_workflow() {
    init_tracing!();
    info!("Testing end-to-end encryption workflow");
    
    let start_time = Instant::now();
    
    // Setup
    setup_crypto_packages().expect("Failed to setup crypto packages");
    
    // Test data
    let plaintext = b"slay this is a secret message that needs maximum security periodt";
    let mut performance_metrics = HashMap::new();
    
    // 1. Generate cryptographically secure random keys
    let key_gen_start = Instant::now();
    let mut aes_key = vec![0u8; 32];
    fill_random(&mut aes_key).expect("Failed to generate AES key");
    
    let mut chacha_key = vec![0u8; 32];
    fill_random(&mut chacha_key).expect("Failed to generate ChaCha20 key");
    
    performance_metrics.insert("key_generation", key_gen_start.elapsed());
    
    // 2. Test symmetric encryption with multiple algorithms
    let sym_start = Instant::now();
    
    // AES-GCM-256 encryption
    let aes_cipher = AesGcm256::new(&aes_key).expect("Failed to create AES cipher");
    let aes_ciphertext = aes_cipher.encrypt(plaintext).expect("AES encryption failed");
    let aes_decrypted = aes_cipher.decrypt(&aes_ciphertext).expect("AES decryption failed");
    assert_eq!(plaintext, &aes_decrypted[..]);
    
    // ChaCha20-Poly1305 encryption
    let chacha_cipher = ChaCha20Poly1305::new(&chacha_key).expect("Failed to create ChaCha20 cipher");
    let chacha_ciphertext = chacha_cipher.encrypt(plaintext).expect("ChaCha20 encryption failed");
    let chacha_decrypted = chacha_cipher.decrypt(&chacha_ciphertext).expect("ChaCha20 decryption failed");
    assert_eq!(plaintext, &chacha_decrypted[..]);
    
    performance_metrics.insert("symmetric_encryption", sym_start.elapsed());
    
    // 3. Test asymmetric encryption
    let asym_start = Instant::now();
    
    // RSA key generation and encryption
    let rsa_keypair = KeyGenerator::generate_rsa_keypair(2048).expect("RSA key generation failed");
    let rsa_encrypted = rsa_keypair.public_key().encrypt(plaintext).expect("RSA encryption failed");
    let rsa_decrypted = rsa_keypair.private_key().decrypt(&rsa_encrypted).expect("RSA decryption failed");
    assert_eq!(plaintext, &rsa_decrypted[..]);
    
    // Elliptic Curve key generation
    let ec_keypair = KeyGenerator::generate_ec_keypair("P-256").expect("EC key generation failed");
    
    performance_metrics.insert("asymmetric_encryption", asym_start.elapsed());
    
    // 4. Test digital signatures
    let sig_start = Instant::now();
    
    // Ed25519 signatures
    let ed25519_keypair = KeyGenerator::generate_ed25519_keypair().expect("Ed25519 key generation failed");
    let signature = ed25519_keypair.sign(plaintext).expect("Signature generation failed");
    let is_valid = ed25519_keypair.verify(plaintext, &signature).expect("Signature verification failed");
    assert!(is_valid, "Signature should be valid");
    
    performance_metrics.insert("digital_signatures", sig_start.elapsed());
    
    // 5. Test key derivation
    let kdf_start = Instant::now();
    
    let password = b"bestie_secure_password_periodt";
    let salt = b"random_salt_123";
    
    // PBKDF2 key derivation
    let pbkdf2_key = pbkdf2_derive(password, salt, 100000, 32).expect("PBKDF2 derivation failed");
    assert_eq!(pbkdf2_key.len(), 32);
    
    // Argon2 key derivation
    let argon2_key = argon2_derive(password, salt, 32).expect("Argon2 derivation failed");
    assert_eq!(argon2_key.len(), 32);
    
    performance_metrics.insert("key_derivation", kdf_start.elapsed());
    
    // 6. Test advanced hashing
    let hash_start = Instant::now();
    
    // SHA-3 hashing
    let sha3_hash = hash_with_algorithm(plaintext, AdvancedHashAlgorithm::Sha3_256)
        .expect("SHA-3 hashing failed");
    assert_eq!(sha3_hash.len(), 32);
    
    // BLAKE3 hashing
    let blake3_hash = hash_with_algorithm(plaintext, AdvancedHashAlgorithm::Blake3)
        .expect("BLAKE3 hashing failed");
    assert_eq!(blake3_hash.len(), 32);
    
    // HMAC computation
    let hmac_key = b"hmac_secret_key";
    let hmac = compute_hmac(plaintext, hmac_key, AdvancedHashAlgorithm::Sha256)
        .expect("HMAC computation failed");
    assert_eq!(hmac.len(), 32);
    
    performance_metrics.insert("advanced_hashing", hash_start.elapsed());
    
    // 7. Test post-quantum readiness
    let pqc_start = Instant::now();
    
    let threat_level = assess_quantum_threat().expect("Quantum threat assessment failed");
    debug!("Current quantum threat level: {:?}", threat_level);
    
    performance_metrics.insert("post_quantum_assessment", pqc_start.elapsed());
    
    let total_time = start_time.elapsed();
    
    // Performance reporting
    info!("End-to-end encryption workflow completed successfully!");
    info!("Total execution time: {:?}", total_time);
    for (operation, time) in performance_metrics {
        info!("  {}: {:?}", operation, time);
    }
    
    // Performance assertions (reasonable bounds for CI)
    assert!(total_time.as_millis() < 5000, "Total time should be under 5 seconds");
}

/// slay Test cross-algorithm compatibility
#[test]
fn test_cross_algorithm_compatibility() {
    init_tracing!();
    info!("Testing cross-algorithm compatibility");
    
    setup_crypto_packages().expect("Failed to setup crypto packages");
    
    let message = b"compatibility test message bestie";
    
    // Test that different hash algorithms produce different but consistent results
    let sha256_hash = hash_with_algorithm(message, AdvancedHashAlgorithm::Sha256).unwrap();
    let sha3_hash = hash_with_algorithm(message, AdvancedHashAlgorithm::Sha3_256).unwrap();
    let blake3_hash = hash_with_algorithm(message, AdvancedHashAlgorithm::Blake3).unwrap();
    
    // Different algorithms should produce different hashes
    assert_ne!(sha256_hash, sha3_hash);
    assert_ne!(sha256_hash, blake3_hash);
    assert_ne!(sha3_hash, blake3_hash);
    
    // But same algorithm should be consistent
    let sha256_hash_2 = hash_with_algorithm(message, AdvancedHashAlgorithm::Sha256).unwrap();
    assert_eq!(sha256_hash, sha256_hash_2);
    
    // Test key derivation with different algorithms using same input
    let password = b"test_password";
    let salt = b"test_salt";
    
    let pbkdf2_key = pbkdf2_derive(password, salt, 10000, 32).unwrap();
    let scrypt_key = scrypt_derive(password, salt, 32).unwrap();
    let argon2_key = argon2_derive(password, salt, 32).unwrap();
    
    // Different KDF algorithms should produce different keys
    assert_ne!(pbkdf2_key, scrypt_key);
    assert_ne!(pbkdf2_key, argon2_key);
    assert_ne!(scrypt_key, argon2_key);
    
    info!("Cross-algorithm compatibility verified successfully!");
}

/// slay Test performance benchmarks
#[test]
fn test_performance_benchmarks() {
    init_tracing!();
    info!("Running crypto performance benchmarks");
    
    setup_crypto_packages().expect("Failed to setup crypto packages");
    
    let test_data_sizes = vec![1024, 4096, 16384, 65536]; // 1KB, 4KB, 16KB, 64KB
    let iterations = 100;
    
    for size in test_data_sizes {
        info!("Benchmarking with data size: {} bytes", size);
        
        let test_data = vec![0u8; size];
        
        // Benchmark symmetric encryption
        let aes_key = vec![0u8; 32];
        let aes_cipher = AesGcm256::new(&aes_key).unwrap();
        
        let sym_start = Instant::now();
        for _ in 0..iterations {
            let encrypted = aes_cipher.encrypt(&test_data).unwrap();
            let _decrypted = aes_cipher.decrypt(&encrypted).unwrap();
        }
        let sym_time = sym_start.elapsed();
        
        // Benchmark hashing
        let hash_start = Instant::now();
        for _ in 0..iterations {
            let _hash = hash_with_algorithm(&test_data, AdvancedHashAlgorithm::Sha256).unwrap();
        }
        let hash_time = hash_start.elapsed();
        
        info!("  Size {}: AES encryption: {:?}, SHA-256 hashing: {:?}", 
              size, sym_time, hash_time);
        
        // Performance assertions (reasonable bounds)
        let sym_per_iteration = sym_time.as_micros() / iterations as u128;
        let hash_per_iteration = hash_time.as_micros() / iterations as u128;
        
        // These are generous bounds to account for CI environment variance
        assert!(sym_per_iteration < 10000, "AES encryption too slow: {} μs", sym_per_iteration);
        assert!(hash_per_iteration < 5000, "SHA-256 hashing too slow: {} μs", hash_per_iteration);
    }
}

/// slay Test memory safety and security properties
#[test]
fn test_memory_safety_and_security() {
    init_tracing!();
    info!("Testing memory safety and security properties");
    
    setup_crypto_packages().expect("Failed to setup crypto packages");
    
    // Test secure memory handling
    let mut sensitive_data = vec![0x42u8; 1024];
    let secure_mem = SecureMemory::new(&mut sensitive_data).unwrap();
    
    // Test constant-time operations
    let a = vec![1, 2, 3, 4];
    let b = vec![1, 2, 3, 4];
    let c = vec![1, 2, 3, 5];
    
    assert!(ConstantTimeOps::constant_time_compare(&a, &b));
    assert!(!ConstantTimeOps::constant_time_compare(&a, &c));
    
    // Test cryptographic random quality
    let mut random_bytes_1 = vec![0u8; 256];
    let mut random_bytes_2 = vec![0u8; 256];
    
    fill_random(&mut random_bytes_1).unwrap();
    fill_random(&mut random_bytes_2).unwrap();
    
    // Random bytes should be different
    assert_ne!(random_bytes_1, random_bytes_2);
    
    // Should not be all zeros
    assert!(random_bytes_1.iter().any(|&b| b != 0));
    assert!(random_bytes_2.iter().any(|&b| b != 0));
    
    info!("Memory safety and security properties verified!");
}

/// slay Test error handling and edge cases
#[test]
fn test_error_handling_and_edge_cases() {
    init_tracing!();
    info!("Testing error handling and edge cases");
    
    setup_crypto_packages().expect("Failed to setup crypto packages");
    
    // Test invalid key sizes
    let invalid_key = vec![0u8; 15]; // Invalid AES key size
    assert!(AesGcm256::new(&invalid_key).is_err());
    
    // Test empty data encryption
    let valid_key = vec![0u8; 32];
    let cipher = AesGcm256::new(&valid_key).unwrap();
    let empty_data = b"";
    let encrypted_empty = cipher.encrypt(empty_data).unwrap();
    let decrypted_empty = cipher.decrypt(&encrypted_empty).unwrap();
    assert_eq!(empty_data, &decrypted_empty[..]);
    
    // Test invalid signature verification
    let ed25519_keypair = KeyGenerator::generate_ed25519_keypair().unwrap();
    let message = b"original message";
    let signature = ed25519_keypair.sign(message).unwrap();
    
    let tampered_message = b"tampered message";
    let is_valid = ed25519_keypair.verify(tampered_message, &signature).unwrap();
    assert!(!is_valid, "Signature should not be valid for tampered message");
    
    // Test key derivation with extreme parameters
    let password = b"test";
    let salt = b"salt";
    
    // Very low iteration count should still work
    let weak_key = pbkdf2_derive(password, salt, 1, 32).unwrap();
    assert_eq!(weak_key.len(), 32);
    
    info!("Error handling and edge cases verified!");
}

/// slay Test concurrent crypto operations
#[test]
fn test_concurrent_operations() {
    init_tracing!();
    info!("Testing concurrent crypto operations");
    
    setup_crypto_packages().expect("Failed to setup crypto packages");
    
    use std::thread;
    use std::sync::Arc;
    
    let test_data = Arc::new(vec![0x42u8; 1024]);
    let num_threads = 4;
    let operations_per_thread = 50;
    
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let data = test_data.clone();
        thread::spawn(move || {
            let mut thread_results = Vec::new();
            
            for i in 0..operations_per_thread {
                // Generate unique key for each operation
                let mut key = vec![0u8; 32];
                key[0] = thread_id as u8;
                key[1] = i as u8;
                fill_random(&mut key[2..]).unwrap();
                
                // Perform encryption
                let cipher = AesGcm256::new(&key).unwrap();
                let encrypted = cipher.encrypt(&data).unwrap();
                let decrypted = cipher.decrypt(&encrypted).unwrap();
                
                thread_results.push((thread_id, i, decrypted == **data));
            }
            
            thread_results
        })
    }).collect();
    
    // Collect results
    for handle in handles {
        let results = handle.join().unwrap();
        for (thread_id, op_id, success) in results {
            assert!(success, "Operation failed in thread {} operation {}", thread_id, op_id);
        }
    }
    
    info!("Concurrent operations completed successfully!");
}

/// slay Test crypto package integration
#[test]
fn test_package_integration() {
    init_tracing!();
    info!("Testing crypto package integration");
    
    setup_crypto_packages().expect("Failed to setup crypto packages");
    
    // Test cipher registry
    let aes_key = vec![0u8; 32];
    let aes_cipher = AesGcm256::new(&aes_key).unwrap();
    register_cipher("test-aes-256", aes_cipher).unwrap();
    
    let retrieved_cipher = get_cipher("test-aes-256").unwrap();
    assert!(retrieved_cipher.is_some());
    
    // Test that all expected algorithms are available
    let available_asymmetric = vec![
        AsymmetricAlgorithm::Rsa2048,
        AsymmetricAlgorithm::EcP256,
        AsymmetricAlgorithm::Ed25519,
    ];
    
    for algo in available_asymmetric {
        debug!("Testing availability of {:?}", algo);
        assert!(algo.name().len() > 0);
    }
    
    // Test hash algorithm availability
    let hash_algorithms = vec![
        AdvancedHashAlgorithm::Sha256,
        AdvancedHashAlgorithm::Sha3_256,
        AdvancedHashAlgorithm::Blake3,
    ];
    
    for algo in hash_algorithms {
        let test_data = b"integration test";
        let hash = hash_with_algorithm(test_data, algo).unwrap();
        assert!(!hash.is_empty());
    }
    
    info!("Package integration verified successfully!");
}

/// slay Comprehensive test runner
#[test]
fn test_comprehensive_crypto_suite() {
    init_tracing!();
    info!("Running comprehensive crypto test suite");
    
    let suite_start = Instant::now();
    
    // Run all test components
    test_end_to_end_encryption_workflow();
    test_cross_algorithm_compatibility();
    test_performance_benchmarks();
    test_memory_safety_and_security();
    test_error_handling_and_edge_cases();
    test_concurrent_operations();
    test_package_integration();
    
    let suite_time = suite_start.elapsed();
    
    info!("🎉 Comprehensive crypto test suite completed successfully!");
    info!("Total suite execution time: {:?}", suite_time);
    
    // Suite should complete in reasonable time
    assert!(suite_time.as_secs() < 30, "Test suite took too long: {:?}", suite_time);
}
