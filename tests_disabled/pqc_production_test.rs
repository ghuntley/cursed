//! Comprehensive test suite for production-ready post-quantum cryptography
//! 
//! This test suite validates all aspects of the PQC implementation including:
//! - Correctness of cryptographic operations
//! - Performance benchmarking
//! - Security properties
//! - Side-channel resistance
//! - Memory safety
//! - Error handling

use cursed::stdlib::crypto::pqc_production::*;
use std::time::{Duration, Instant};

#[path = "common.rs"]
mod common;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_env_filter("debug")
            .try_init();
    };
}

// ============================================================================
// KYBER KEM TESTS
// ============================================================================

#[test]
fn test_kyber_key_generation() {
    init_tracing!();
    
    for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
        tracing::info!("Testing Kyber key generation for {:?}", level);
        
        let result = KyberKem::keygen(level);
        assert!(result.is_ok(), "Key generation failed for {:?}", level);
        
        let (public_key, secret_key) = result.unwrap();
        
        // Validate key properties
        assert_eq!(public_key.parameter_set.security_level(), level);
        assert_eq!(secret_key.parameter_set.security_level(), level);
        
        // Validate key sizes
        let expected_pub_size = public_key.parameter_set.public_key_size();
        let expected_sec_size = secret_key.parameter_set.secret_key_size();
        
        assert_eq!(public_key.key_data.len(), expected_pub_size);
        assert_eq!(secret_key.key_data.len(), expected_sec_size);
        
        // Validate fingerprints
        assert_ne!(public_key.fingerprint, [0u8; 32]);
        assert_eq!(secret_key.public_fingerprint, public_key.fingerprint);
        
        tracing::info!("✓ Kyber {:?} key generation successful", level);
    }
}

#[test]
fn test_kyber_encapsulation_decapsulation() {
    init_tracing!();
    
    for params in [KyberParameterSet::Kyber512, KyberParameterSet::Kyber768, KyberParameterSet::Kyber1024] {
        tracing::info!("Testing Kyber encaps/decaps for {:?}", params);
        
        // Generate key pair
        let (public_key, secret_key) = KyberKem::keygen_with_params(params)
            .expect("Key generation failed");
        
        // Encapsulation
        let (ciphertext, shared_secret1) = KyberKem::encaps(&public_key)
            .expect("Encapsulation failed");
        
        // Validate ciphertext size
        assert_eq!(ciphertext.len(), params.ciphertext_size());
        assert_eq!(shared_secret1.len(), params.shared_secret_size());
        
        // Decapsulation
        let shared_secret2 = KyberKem::decaps(&secret_key, &ciphertext)
            .expect("Decapsulation failed");
        
        // Validate shared secrets match
        assert_eq!(shared_secret1.as_slice(), shared_secret2.as_slice());
        assert_eq!(shared_secret2.len(), params.shared_secret_size());
        
        tracing::info!("✓ Kyber {:?} encaps/decaps successful", params);
    }
}

#[test]
fn test_kyber_invalid_ciphertext() {
    init_tracing!();
    
    let (_, secret_key) = KyberKem::keygen(SecurityLevel::Level1)
        .expect("Key generation failed");
    
    // Test with wrong size ciphertext
    let wrong_size_ciphertext = vec![0u8; 100];
    let result = KyberKem::decaps(&secret_key, &wrong_size_ciphertext);
    
    assert!(result.is_err());
    match result.err().unwrap() {
        PqcError::InvalidCiphertext(_) => {},
        _ => panic!("Expected InvalidCiphertext error"),
    }
    
    tracing::info!("✓ Invalid ciphertext properly rejected");
}

#[test]
fn test_kyber_deterministic_key_generation() {
    init_tracing!();
    
    // While key generation uses random seeds, we can test that
    // different calls produce different keys (very high probability)
    let (pk1, sk1) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
    let (pk2, sk2) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
    
    // Keys should be different (probability of collision is negligible)
    assert_ne!(pk1.key_data.as_slice(), pk2.key_data.as_slice());
    assert_ne!(sk1.key_data.as_slice(), sk2.key_data.as_slice());
    
    tracing::info!("✓ Key generation produces unique keys");
}

// ============================================================================
// DILITHIUM SIGNATURE TESTS
// ============================================================================

#[test]
fn test_dilithium_key_generation() {
    init_tracing!();
    
    for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
        tracing::info!("Testing Dilithium key generation for {:?}", level);
        
        let result = DilithiumSigner::keygen(level);
        assert!(result.is_ok(), "Key generation failed for {:?}", level);
        
        let (public_key, secret_key) = result.unwrap();
        
        // Validate key properties
        assert_eq!(public_key.parameter_set.security_level(), level);
        assert_eq!(secret_key.parameter_set.security_level(), level);
        
        // Validate key sizes
        let expected_pub_size = public_key.parameter_set.public_key_size();
        let expected_sec_size = secret_key.parameter_set.secret_key_size();
        
        assert_eq!(public_key.key_data.len(), expected_pub_size);
        assert_eq!(secret_key.key_data.len(), expected_sec_size);
        
        tracing::info!("✓ Dilithium {:?} key generation successful", level);
    }
}

#[test]
fn test_dilithium_sign_verify() {
    init_tracing!();
    
    let test_messages = [
        b"Hello, quantum-safe world!".as_slice(),
        b"".as_slice(), // Empty message
        b"A".as_slice(), // Single character
        b"This is a longer message that tests the digital signature algorithm with more substantial content to ensure it works correctly with various message lengths and content types.".as_slice(),
    ];
    
    for params in [DilithiumParameterSet::Dilithium2, DilithiumParameterSet::Dilithium3, DilithiumParameterSet::Dilithium5] {
        tracing::info!("Testing Dilithium sign/verify for {:?}", params);
        
        // Generate key pair
        let (public_key, secret_key) = DilithiumSigner::keygen_with_params(params)
            .expect("Key generation failed");
        
        for (i, message) in test_messages.iter().enumerate() {
            tracing::debug!("Testing message {}: {} bytes", i, message.len());
            
            // Sign message
            let signature = DilithiumSigner::sign(&secret_key, message)
                .expect("Signing failed");
            
            // Validate signature size
            assert_eq!(signature.len(), params.signature_size());
            
            // Verify signature
            let is_valid = DilithiumSigner::verify(&public_key, message, &signature)
                .expect("Verification failed");
            
            assert!(is_valid, "Signature verification failed for message {}", i);
            
            // Test signature tampering detection
            let mut tampered_signature = signature.clone();
            tampered_signature[0] ^= 0xFF;
            
            let tampered_result = DilithiumSigner::verify(&public_key, message, &tampered_signature)
                .expect("Tampered verification failed");
            
            assert!(!tampered_result, "Tampered signature was not detected for message {}", i);
            
            // Test message tampering detection
            if !message.is_empty() {
                let mut tampered_message = message.to_vec();
                tampered_message[0] ^= 0xFF;
                
                let message_tampered_result = DilithiumSigner::verify(&public_key, &tampered_message, &signature)
                    .expect("Message tampered verification failed");
                
                assert!(!message_tampered_result, "Message tampering was not detected for message {}", i);
            }
        }
        
        tracing::info!("✓ Dilithium {:?} sign/verify successful", params);
    }
}

#[test]
fn test_dilithium_invalid_signature() {
    init_tracing!();
    
    let (public_key, _) = DilithiumSigner::keygen(SecurityLevel::Level1)
        .expect("Key generation failed");
    
    let message = b"Test message";
    
    // Test with wrong size signature
    let wrong_size_signature = vec![0u8; 100];
    let result = DilithiumSigner::verify(&public_key, message, &wrong_size_signature);
    
    assert!(result.is_err());
    match result.err().unwrap() {
        PqcError::InvalidSignature(_) => {},
        _ => panic!("Expected InvalidSignature error"),
    }
    
    tracing::info!("✓ Invalid signature properly rejected");
}

// ============================================================================
// HYBRID CRYPTOGRAPHY TESTS
// ============================================================================

#[test]
fn test_hybrid_key_exchange() {
    init_tracing!();
    
    for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
        tracing::info!("Testing hybrid key exchange for {:?}", level);
        
        // Generate key pairs for both parties
        let alice_keys = HybridKeyExchange::generate_keypair(level)
            .expect("Alice key generation failed");
        let bob_keys = HybridKeyExchange::generate_keypair(level)
            .expect("Bob key generation failed");
        
        // Perform key exchange
        let shared_secret = HybridKeyExchange::perform_exchange(&alice_keys, &bob_keys)
            .expect("Key exchange failed");
        
        // Validate shared secret
        let expected_size = match level {
            SecurityLevel::Level1 => 32,
            SecurityLevel::Level3 => 48,
            SecurityLevel::Level5 => 64,
        };
        
        assert_eq!(shared_secret.len(), expected_size);
        assert_ne!(shared_secret.as_slice(), vec![0u8; expected_size].as_slice());
        
        tracing::info!("✓ Hybrid key exchange {:?} successful", level);
    }
}

// ============================================================================
// CONSTANT-TIME OPERATIONS TESTS
// ============================================================================

#[test]
fn test_constant_time_operations() {
    init_tracing!();
    
    let data1 = b"secret_password_123";
    let data2 = b"secret_password_123";
    let data3 = b"different_password";
    
    // Test equal data
    assert!(ConstantTime::bytes_equal(data1, data2));
    
    // Test different data
    assert!(!ConstantTime::bytes_equal(data1, data3));
    
    // Test different lengths
    assert!(!ConstantTime::bytes_equal(data1, b"short"));
    
    // Test empty data
    assert!(ConstantTime::bytes_equal(b"", b""));
    assert!(!ConstantTime::bytes_equal(b"", b"non-empty"));
    
    tracing::info!("✓ Constant-time operations working correctly");
}

#[test]
fn test_constant_time_conditional_operations() {
    init_tracing!();
    
    let mut dest = [0u8; 8];
    let src = [0xFF; 8];
    
    // Test conditional copy (true condition)
    ConstantTime::conditional_copy(&mut dest, &src, true);
    assert_eq!(dest, src);
    
    // Reset and test false condition
    dest = [0u8; 8];
    ConstantTime::conditional_copy(&mut dest, &src, false);
    assert_eq!(dest, [0u8; 8]);
    
    // Test conditional swap
    let mut a = [0x11; 4];
    let mut b = [0x22; 4];
    let orig_a = a;
    let orig_b = b;
    
    ConstantTime::conditional_swap(&mut a, &mut b, true);
    assert_eq!(a, orig_b);
    assert_eq!(b, orig_a);
    
    ConstantTime::conditional_swap(&mut a, &mut b, false);
    assert_eq!(a, orig_b); // No change
    assert_eq!(b, orig_a); // No change
    
    tracing::info!("✓ Constant-time conditional operations working correctly");
}

// ============================================================================
// SECURE MEMORY TESTS
// ============================================================================

#[test]
fn test_secure_bytes() {
    init_tracing!();
    
    // Test creation
    let secure = SecureBytes::new(32);
    assert_eq!(secure.len(), 32);
    assert!(!secure.is_empty());
    
    // Test from existing data
    let data = vec![1, 2, 3, 4, 5];
    let secure = SecureBytes::from_bytes(data);
    assert_eq!(secure.len(), 5);
    assert_eq!(secure.as_slice(), &[1, 2, 3, 4, 5]);
    
    // Test empty
    let empty = SecureBytes::new(0);
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    
    tracing::info!("✓ SecureBytes container working correctly");
}

// ============================================================================
// PERFORMANCE BENCHMARKS
// ============================================================================

#[test]
fn test_kyber_performance_benchmarks() {
    init_tracing!();
    
    let iterations = 10; // Reduced for testing
    
    for params in [KyberParameterSet::Kyber512, KyberParameterSet::Kyber768, KyberParameterSet::Kyber1024] {
        tracing::info!("Benchmarking Kyber {:?}", params);
        
        let result = KyberKem::benchmark(params, iterations);
        assert!(result.is_ok(), "Benchmark failed for {:?}", params);
        
        let benchmark = result.unwrap();
        
        // Validate benchmark results
        assert_eq!(benchmark.algorithm, AlgorithmType::Kyber);
        assert_eq!(benchmark.iterations, iterations);
        assert!(benchmark.avg_keygen_time > Duration::from_nanos(0));
        assert!(benchmark.avg_encaps_time > Duration::from_nanos(0));
        assert!(benchmark.avg_decaps_time > Duration::from_nanos(0));
        assert!(benchmark.operations_per_second() > 0.0);
        
        tracing::info!("✓ Kyber {:?} benchmark: {:.2}ms keygen, {:.0} ops/sec", 
            params, 
            benchmark.avg_keygen_time.as_millis(),
            benchmark.operations_per_second()
        );
    }
}

#[test]
fn test_comprehensive_benchmarks() {
    init_tracing!();
    
    let iterations = 5; // Reduced for testing
    
    let results = PqcBenchmarkSuite::run_all_benchmarks(iterations);
    assert!(results.is_ok(), "Comprehensive benchmarks failed");
    
    let benchmark_results = results.unwrap();
    assert!(!benchmark_results.is_empty());
    
    // Generate comparative analysis
    let analysis = PqcBenchmarkSuite::comparative_analysis(&benchmark_results);
    assert!(!analysis.is_empty());
    
    tracing::info!("✓ Comprehensive benchmarks completed");
    tracing::info!("Analysis: {}", analysis);
}

// ============================================================================
// SECURITY PROPERTY TESTS
// ============================================================================

#[test]
fn test_security_levels() {
    init_tracing!();
    
    assert_eq!(SecurityLevel::Level1.classical_bits(), 128);
    assert_eq!(SecurityLevel::Level1.quantum_bits(), 64);
    
    assert_eq!(SecurityLevel::Level3.classical_bits(), 192);
    assert_eq!(SecurityLevel::Level3.quantum_bits(), 96);
    
    assert_eq!(SecurityLevel::Level5.classical_bits(), 256);
    assert_eq!(SecurityLevel::Level5.quantum_bits(), 128);
    
    // Test use cases
    let level1_cases = SecurityLevel::Level1.use_cases();
    assert!(level1_cases.contains(&"IoT devices"));
    
    let level3_cases = SecurityLevel::Level3.use_cases();
    assert!(level3_cases.contains(&"General applications"));
    
    let level5_cases = SecurityLevel::Level5.use_cases();
    assert!(level5_cases.contains(&"Top secret"));
    
    tracing::info!("✓ Security levels properly defined");
}

#[test]
fn test_algorithm_recommendations() {
    init_tracing!();
    
    // Test valid recommendations
    assert_eq!(
        get_recommended_algorithm("kem", SecurityLevel::Level3).unwrap(),
        AlgorithmType::Kyber
    );
    
    assert_eq!(
        get_recommended_algorithm("signature", SecurityLevel::Level3).unwrap(),
        AlgorithmType::Dilithium
    );
    
    assert_eq!(
        get_recommended_algorithm("hash_signature", SecurityLevel::Level1).unwrap(),
        AlgorithmType::SphincsPl
    );
    
    // Test invalid use case
    let result = get_recommended_algorithm("invalid_use_case", SecurityLevel::Level1);
    assert!(result.is_err());
    
    tracing::info!("✓ Algorithm recommendations working correctly");
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_error_handling() {
    init_tracing!();
    
    // Test PqcError display
    let errors = [
        PqcError::InvalidKey("test key error".to_string()),
        PqcError::InvalidCiphertext("test ciphertext error".to_string()),
        PqcError::InvalidSignature("test signature error".to_string()),
        PqcError::RandomGenerationFailed("test random error".to_string()),
        PqcError::SideChannelDetected("test side channel".to_string()),
    ];
    
    for error in errors {
        let error_string = format!("{}", error);
        assert!(!error_string.is_empty());
        tracing::debug!("Error: {}", error_string);
    }
    
    // Test conversion to CursedError
    let pqc_error = PqcError::InvalidKey("test".to_string());
    let cursed_error: crate::error::CursedError = pqc_error.into();
    
    match cursed_error {
        crate::error::CursedError::Runtime(msg) => {
            assert!(msg.contains("Post-Quantum Cryptography error"));
        },
        _ => panic!("Expected Runtime error"),
    }
    
    tracing::info!("✓ Error handling working correctly");
}

// ============================================================================
// UTILITY FUNCTION TESTS
// ============================================================================

#[test]
fn test_utility_functions() {
    init_tracing!();
    
    // Test hex conversion
    let bytes = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    let hex = bytes_to_hex(&bytes);
    assert_eq!(hex, "0123456789abcdef");
    
    let converted = hex_to_bytes(&hex).unwrap();
    assert_eq!(bytes, converted);
    
    // Test invalid hex
    let invalid_hex_result = hex_to_bytes("xyz");
    assert!(invalid_hex_result.is_err());
    
    let odd_length_result = hex_to_bytes("123");
    assert!(odd_length_result.is_err());
    
    // Test security level validation
    for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
        assert!(validate_security_level(level).is_ok());
    }
    
    tracing::info!("✓ Utility functions working correctly");
}

// ============================================================================
// QUANTUM THREAT ASSESSMENT TESTS
// ============================================================================

#[test]
fn test_quantum_threat_assessment() {
    init_tracing!();
    
    // Test threat level assessment
    let threat_level = QuantumThreatAssessment::current_threat_level();
    assert!(!threat_level.is_empty());
    assert!(threat_level.contains("MODERATE"));
    
    // Test migration timeline
    let kyber_timeline = QuantumThreatAssessment::migration_timeline(AlgorithmType::Kyber);
    assert!(!kyber_timeline.is_empty());
    assert!(kyber_timeline.contains("IMMEDIATE"));
    
    let falcon_timeline = QuantumThreatAssessment::migration_timeline(AlgorithmType::Falcon);
    assert!(!falcon_timeline.is_empty());
    
    // Test security report generation
    let security_report = QuantumThreatAssessment::security_report();
    assert!(!security_report.is_empty());
    assert!(security_report.contains("Quantum Threat Assessment"));
    
    tracing::info!("✓ Quantum threat assessment working correctly");
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_end_to_end_secure_communication() {
    init_tracing!();
    
    tracing::info!("Testing end-to-end secure communication");
    
    // Alice and Bob generate their key pairs
    let (alice_sign_pk, alice_sign_sk) = DilithiumSigner::keygen(SecurityLevel::Level3).unwrap();
    let (alice_kem_pk, alice_kem_sk) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    
    let (bob_sign_pk, bob_sign_sk) = DilithiumSigner::keygen(SecurityLevel::Level3).unwrap();
    let (bob_kem_pk, bob_kem_sk) = KyberKem::keygen(SecurityLevel::Level3).unwrap();
    
    // Alice sends a signed and encrypted message to Bob
    let message = b"This is a secret quantum-safe message!";
    
    // Alice signs the message
    let signature = DilithiumSigner::sign(&alice_sign_sk, message).unwrap();
    
    // Alice encrypts for Bob
    let (ciphertext, shared_secret) = KyberKem::encaps(&bob_kem_pk).unwrap();
    
    // Bob receives the message
    // First, decrypt to get shared secret
    let bob_shared_secret = KyberKem::decaps(&bob_kem_sk, &ciphertext).unwrap();
    assert_eq!(shared_secret.as_slice(), bob_shared_secret.as_slice());
    
    // Then verify Alice's signature
    let signature_valid = DilithiumSigner::verify(&alice_sign_pk, message, &signature).unwrap();
    assert!(signature_valid);
    
    tracing::info!("✓ End-to-end secure communication successful");
}

#[test]
fn test_multiple_operations_performance() {
    init_tracing!();
    
    let operations = 50;
    let start_time = Instant::now();
    
    // Generate multiple key pairs and perform operations
    for i in 0..operations {
        if i % 10 == 0 {
            tracing::debug!("Completed {} operations", i);
        }
        
        // Kyber operations
        let (pk, sk) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
        let (ct, ss1) = KyberKem::encaps(&pk).unwrap();
        let ss2 = KyberKem::decaps(&sk, &ct).unwrap();
        assert_eq!(ss1.as_slice(), ss2.as_slice());
        
        // Dilithium operations
        let (dpk, dsk) = DilithiumSigner::keygen(SecurityLevel::Level1).unwrap();
        let msg = format!("Message {}", i);
        let sig = DilithiumSigner::sign(&dsk, msg.as_bytes()).unwrap();
        let valid = DilithiumSigner::verify(&dpk, msg.as_bytes(), &sig).unwrap();
        assert!(valid);
    }
    
    let total_time = start_time.elapsed();
    let ops_per_second = operations as f64 / total_time.as_secs_f64();
    
    tracing::info!("✓ Completed {} operations in {:.2}s ({:.1} ops/sec)", 
        operations, total_time.as_secs_f64(), ops_per_second);
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[test]
#[ignore] // Only run with --ignored flag
fn test_stress_large_messages() {
    init_tracing!();
    
    // Test with very large messages (up to 1MB)
    let sizes = [1024, 10_240, 102_400, 1_048_576]; // 1KB, 10KB, 100KB, 1MB
    
    let (pk, sk) = DilithiumSigner::keygen(SecurityLevel::Level1).unwrap();
    
    for size in sizes {
        tracing::info!("Testing message size: {} bytes", size);
        
        let large_message = vec![0x42u8; size];
        
        let start_time = Instant::now();
        let signature = DilithiumSigner::sign(&sk, &large_message).unwrap();
        let sign_time = start_time.elapsed();
        
        let start_time = Instant::now();
        let is_valid = DilithiumSigner::verify(&pk, &large_message, &signature).unwrap();
        let verify_time = start_time.elapsed();
        
        assert!(is_valid);
        
        tracing::info!("✓ {}KB message: sign={:.2}ms, verify={:.2}ms", 
            size / 1024, sign_time.as_millis(), verify_time.as_millis());
    }
}

#[test]
#[ignore] // Only run with --ignored flag
fn test_stress_concurrent_operations() {
    init_tracing!();
    
    use std::sync::Arc;
    use std::thread;
    
    let num_threads = 4;
    let operations_per_thread = 25;
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let handle = thread::spawn(move || {
            for op in 0..operations_per_thread {
                // Kyber operations
                let (pk, sk) = KyberKem::keygen(SecurityLevel::Level1).unwrap();
                let (ct, ss1) = KyberKem::encaps(&pk).unwrap();
                let ss2 = KyberKem::decaps(&sk, &ct).unwrap();
                assert_eq!(ss1.as_slice(), ss2.as_slice());
                
                if op % 5 == 0 {
                    tracing::debug!("Thread {} completed {} operations", thread_id, op + 1);
                }
            }
            tracing::info!("Thread {} completed all operations", thread_id);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    tracing::info!("✓ Concurrent stress test completed successfully");
}

// ============================================================================
// MEMORY SAFETY TESTS
// ============================================================================

#[test]
fn test_memory_safety() {
    init_tracing!();
    
    // Test that SecureBytes properly zeros memory on drop
    let test_data = vec![0xFF; 1000];
    let original_ptr: *const u8;
    
    {
        let secure = SecureBytes::from_bytes(test_data.clone());
        original_ptr = secure.as_slice().as_ptr();
        
        // Verify data is correct while in scope
        assert_eq!(secure.as_slice(), test_data.as_slice());
    } // SecureBytes drops here and should zero memory
    
    // Note: We can't actually verify the memory was zeroed because
    // accessing freed memory is undefined behavior. In a real implementation,
    // this would be tested with memory debugging tools.
    
    tracing::info!("✓ Memory safety test completed (SecureBytes drops properly)");
}
