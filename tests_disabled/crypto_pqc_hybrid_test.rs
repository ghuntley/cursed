//! Comprehensive Test Suite for Post-Quantum Cryptography Hybrid System
//! 
//! This test suite validates the hybrid cryptographic system that combines
//! classical and post-quantum algorithms for maximum security during the
//! post-quantum transition period.

use std::time::Duration;
use cursed::stdlib::crypto_pqc::hybrid::*;
use cursed::stdlib::crypto_pqc::{SecurityLevel, AlgorithmType, PqcResult};

/// Test basic hybrid KEM functionality
#[test]
fn test_hybrid_kem_basic_functionality() {
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level3,
    );

    // Test key generation
    let key_pair = hybrid_kem.keygen().expect("Key generation should succeed");
    
    // Verify key pair structure
    assert_eq!(key_pair.algorithm_info.classical, ClassicalAlgorithm::X25519);
    assert_eq!(key_pair.algorithm_info.pqc, AlgorithmType::Kyber);
    assert_eq!(key_pair.algorithm_info.security_level, SecurityLevel::Level3);
    assert!(!key_pair.classical_public.is_empty());
    assert!(!key_pair.classical_secret.is_empty());
    assert!(!key_pair.pqc_public.is_empty());
    assert!(!key_pair.pqc_secret.is_empty());

    // Test encapsulation/decapsulation round trip
    let (ciphertext, shared_secret1) = hybrid_kem.encaps(&key_pair)
        .expect("Encapsulation should succeed");
    let shared_secret2 = hybrid_kem.decaps(&key_pair, &ciphertext)
        .expect("Decapsulation should succeed");
    
    assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
}

/// Test hybrid KEM with different algorithm combinations
#[test]
fn test_hybrid_kem_algorithm_combinations() {
    let test_cases = vec![
        (ClassicalAlgorithm::EcdhP256, AlgorithmType::Kyber, SecurityLevel::Level1),
        (ClassicalAlgorithm::EcdhP384, AlgorithmType::Kyber, SecurityLevel::Level3),
        (ClassicalAlgorithm::EcdhP521, AlgorithmType::Kyber, SecurityLevel::Level5),
        (ClassicalAlgorithm::X25519, AlgorithmType::Dilithium, SecurityLevel::Level3),
        (ClassicalAlgorithm::Rsa2048, AlgorithmType::Kyber, SecurityLevel::Level1),
    ];

    for (classical, pqc, security_level) in test_cases {
        let hybrid_kem = HybridKem::new(classical, pqc, security_level);
        
        // Test key generation
        let key_pair = hybrid_kem.keygen()
            .expect(&format!("Key generation should succeed for {:?} + {:?}", classical, pqc));
        
        // Test round trip
        let (ciphertext, shared_secret1) = hybrid_kem.encaps(&key_pair)
            .expect("Encapsulation should succeed");
        let shared_secret2 = hybrid_kem.decaps(&key_pair, &ciphertext)
            .expect("Decapsulation should succeed");
        
        assert_eq!(shared_secret1, shared_secret2, 
            "Shared secrets should match for {:?} + {:?}", classical, pqc);
    }
}

/// Test hybrid KEM configuration options
#[test]
fn test_hybrid_kem_configuration() {
    let config = HybridConfig {
        enable_performance_caching: true,
        enable_security_logging: true,
        max_cached_operations: 100,
        key_derivation_iterations: 50_000,
        secure_memory_zeroing: true,
        timing_attack_resistance: true,
    };

    let hybrid_kem = HybridKem::new_with_config(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level3,
        config,
    );

    // Test with custom configuration
    let key_pair = hybrid_kem.keygen().expect("Key generation should succeed");
    let (ciphertext, shared_secret1) = hybrid_kem.encaps(&key_pair)
        .expect("Encapsulation should succeed");
    let shared_secret2 = hybrid_kem.decaps(&key_pair, &ciphertext)
        .expect("Decapsulation should succeed");
    
    assert_eq!(shared_secret1, shared_secret2);
}

/// Test key combination strategies
#[test]
fn test_key_combination_strategies() {
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level3,
    );

    let key_pair = hybrid_kem.keygen().expect("Key generation should succeed");
    
    // Test different key combination methods
    let classical_secret = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let pqc_secret = vec![8, 7, 6, 5, 4, 3, 2, 1];
    
    // Test concatenation
    let concat_result = hybrid_kem.combine_shared_secrets(
        classical_secret.clone(),
        pqc_secret.clone(),
        KeyCombinerType::Concatenation,
    ).expect("Concatenation should succeed");
    assert!(concat_result.len() >= classical_secret.len() + pqc_secret.len());
    
    // Test XOR (need same length)
    let classical_xor = vec![1, 2, 3, 4];
    let pqc_xor = vec![5, 6, 7, 8];
    let xor_result = hybrid_kem.combine_shared_secrets(
        classical_xor,
        pqc_xor,
        KeyCombinerType::Xor,
    ).expect("XOR should succeed");
    assert_eq!(xor_result, vec![4, 4, 4, 4]); // 1^5, 2^6, 3^7, 4^8
    
    // Test KDF combination
    let kdf_result = hybrid_kem.combine_shared_secrets(
        classical_secret.clone(),
        pqc_secret.clone(),
        KeyCombinerType::KdfCombination,
    ).expect("KDF combination should succeed");
    assert_eq!(kdf_result.len(), 32); // Standard derived key size
    
    // Test HKDF combination
    let hkdf_result = hybrid_kem.combine_shared_secrets(
        classical_secret,
        pqc_secret,
        KeyCombinerType::HkdfCombination,
    ).expect("HKDF combination should succeed");
    assert_eq!(hkdf_result.len(), 32); // Standard derived key size
}

/// Test hybrid signature system
#[test]
fn test_hybrid_signature_basic() {
    let hybrid_sig = HybridSignature::new(
        ClassicalSignatureAlgorithm::Ed25519,
        AlgorithmType::Dilithium,
        SecurityLevel::Level3,
    );

    // Generate keys
    let key_pair = hybrid_sig.keygen().expect("Key generation should succeed");
    
    // Verify key pair structure
    assert_eq!(key_pair.algorithm_info.classical, ClassicalSignatureAlgorithm::Ed25519);
    assert_eq!(key_pair.algorithm_info.pqc, AlgorithmType::Dilithium);
    assert_eq!(key_pair.algorithm_info.security_level, SecurityLevel::Level3);
    
    // Test signing and verification
    let message = b"Hello, hybrid post-quantum world!";
    let signature = hybrid_sig.sign(&key_pair, message)
        .expect("Signing should succeed");
    
    // Verify signature structure
    assert!(!signature.classical_signature.is_empty());
    assert!(!signature.pqc_signature.is_empty());
    assert!(!signature.combined_signature.is_empty());
    assert!(!signature.metadata.message_hash.is_empty());
    
    // Verify signature
    let is_valid = hybrid_sig.verify(&key_pair, message, &signature)
        .expect("Verification should succeed");
    assert!(is_valid, "Signature should be valid");
    
    // Test with modified message
    let modified_message = b"Hello, modified message!";
    let is_invalid = hybrid_sig.verify(&key_pair, modified_message, &signature)
        .expect("Verification should succeed");
    assert!(!is_invalid, "Signature should be invalid for modified message");
}

/// Test hybrid signature with different algorithm combinations
#[test]
fn test_hybrid_signature_algorithms() {
    let test_cases = vec![
        (ClassicalSignatureAlgorithm::EcdsaP256, AlgorithmType::Dilithium, SecurityLevel::Level1),
        (ClassicalSignatureAlgorithm::EcdsaP384, AlgorithmType::Dilithium, SecurityLevel::Level3),
        (ClassicalSignatureAlgorithm::Ed25519, AlgorithmType::Sphincs, SecurityLevel::Level3),
        (ClassicalSignatureAlgorithm::RsaPss2048, AlgorithmType::Dilithium, SecurityLevel::Level1),
    ];

    for (classical, pqc, security_level) in test_cases {
        let hybrid_sig = HybridSignature::new(classical, pqc, security_level);
        
        let key_pair = hybrid_sig.keygen()
            .expect(&format!("Key generation should succeed for {:?} + {:?}", classical, pqc));
        
        let message = b"Test message for hybrid signatures";
        let signature = hybrid_sig.sign(&key_pair, message)
            .expect("Signing should succeed");
        
        let is_valid = hybrid_sig.verify(&key_pair, message, &signature)
            .expect("Verification should succeed");
        assert!(is_valid, "Signature should be valid for {:?} + {:?}", classical, pqc);
    }
}

/// Test signature combination strategies
#[test]
fn test_signature_combination_strategies() {
    let hybrid_sig = HybridSignature::new(
        ClassicalSignatureAlgorithm::Ed25519,
        AlgorithmType::Dilithium,
        SecurityLevel::Level3,
    );

    let classical_sig = vec![1u8; 64]; // Ed25519 signature size
    let pqc_sig = vec![2u8; 3293]; // Dilithium3 signature size
    
    // Test concatenation
    let concat_result = hybrid_sig.combine_signatures(
        &classical_sig,
        &pqc_sig,
        SignatureCombinerType::Concatenation,
    ).expect("Concatenation should succeed");
    assert!(concat_result.len() >= classical_sig.len() + pqc_sig.len() + 8); // 8 bytes for length prefixes
    
    // Test structured format
    let structured_result = hybrid_sig.combine_signatures(
        &classical_sig,
        &pqc_sig,
        SignatureCombinerType::StructuredFormat,
    ).expect("Structured format should succeed");
    assert!(structured_result.len() >= classical_sig.len() + pqc_sig.len() + 17); // Header + length prefixes
    
    // Test composite scheme
    let composite_result = hybrid_sig.combine_signatures(
        &classical_sig,
        &pqc_sig,
        SignatureCombinerType::CompositeScheme,
    ).expect("Composite scheme should succeed");
    assert!(composite_result.len() >= classical_sig.len() + pqc_sig.len() + 40); // Hash + length prefixes
}

/// Test ciphertext combination and splitting
#[test]
fn test_ciphertext_operations() {
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level1,
    );

    let classical_ct = vec![1, 2, 3, 4, 5];
    let pqc_ct = vec![6, 7, 8, 9, 10, 11, 12];
    
    // Test combination
    let combined = hybrid_kem.combine_ciphertexts(classical_ct.clone(), pqc_ct.clone())
        .expect("Combination should succeed");
    
    // Test splitting
    let (split_classical, split_pqc) = hybrid_kem.split_ciphertext(&combined)
        .expect("Splitting should succeed");
    
    assert_eq!(split_classical, classical_ct);
    assert_eq!(split_pqc, pqc_ct);
}

/// Test migration strategy
#[test]
fn test_migration_strategy() {
    let mut strategy = HybridMigrationStrategy::standard();
    
    // Test initial state
    assert_eq!(strategy.current_phase, 0);
    let phase = strategy.current_phase().expect("Should have current phase");
    assert_eq!(phase.name, "Classical Only");
    assert_eq!(phase.classical_weight, 1.0);
    assert_eq!(phase.pqc_weight, 0.0);
    
    // Test advancement
    strategy.advance_phase().expect("Should advance phase");
    assert_eq!(strategy.current_phase, 1);
    let phase = strategy.current_phase().expect("Should have current phase");
    assert_eq!(phase.name, "Early Adoption");
    assert_eq!(phase.classical_weight, 0.8);
    assert_eq!(phase.pqc_weight, 0.2);
    
    // Test recommendations
    let recommendations = strategy.get_current_recommendations()
        .expect("Should have recommendations");
    assert!(!recommendations.is_empty());
    
    // Test advancement to final phase
    for _ in 0..3 {
        strategy.advance_phase().expect("Should advance phase");
    }
    assert_eq!(strategy.current_phase, 4);
    let phase = strategy.current_phase().expect("Should have current phase");
    assert_eq!(phase.name, "PQC Only");
    assert_eq!(phase.classical_weight, 0.0);
    assert_eq!(phase.pqc_weight, 1.0);
    
    // Test can't advance beyond final phase
    let result = strategy.advance_phase();
    assert!(result.is_err());
}

/// Test compatibility matrix
#[test]
fn test_compatibility_matrix() {
    let matrix = HybridCompatibilityMatrix::new();
    
    // Test known excellent combinations
    assert_eq!(
        matrix.get_rating(ClassicalAlgorithm::X25519, AlgorithmType::Kyber),
        CompatibilityRating::Excellent
    );
    assert_eq!(
        matrix.get_rating(ClassicalAlgorithm::EcdhP256, AlgorithmType::Kyber),
        CompatibilityRating::Excellent
    );
    
    // Test excellent combinations list
    let excellent = matrix.get_excellent_combinations();
    assert!(!excellent.is_empty());
    assert!(excellent.contains(&(ClassicalAlgorithm::X25519, AlgorithmType::Kyber)));
    
    // Test security level recommendations
    let level3_recommendations = matrix.get_recommended_for_security_level(SecurityLevel::Level3);
    assert!(!level3_recommendations.is_empty());
    
    let level1_recommendations = matrix.get_recommended_for_security_level(SecurityLevel::Level1);
    assert!(!level1_recommendations.is_empty());
}

/// Test performance and caching
#[test]
fn test_performance_caching() {
    let config = HybridConfig {
        enable_performance_caching: true,
        enable_security_logging: false,
        max_cached_operations: 10,
        key_derivation_iterations: 1000, // Reduced for testing
        secure_memory_zeroing: false, // Disabled for testing
        timing_attack_resistance: false,
    };

    let hybrid_kem = HybridKem::new_with_config(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level1,
        config,
    );

    // First key generation should be slower
    let start = std::time::Instant::now();
    let key_pair1 = hybrid_kem.keygen().expect("First keygen should succeed");
    let duration1 = start.elapsed();

    // Second key generation might use cache (timing is implementation dependent)
    let start = std::time::Instant::now();
    let key_pair2 = hybrid_kem.keygen().expect("Second keygen should succeed");
    let duration2 = start.elapsed();

    // Both should be valid
    let (ct1, ss1_1) = hybrid_kem.encaps(&key_pair1).expect("Encaps should succeed");
    let ss1_2 = hybrid_kem.decaps(&key_pair1, &ct1).expect("Decaps should succeed");
    assert_eq!(ss1_1, ss1_2);

    let (ct2, ss2_1) = hybrid_kem.encaps(&key_pair2).expect("Encaps should succeed");
    let ss2_2 = hybrid_kem.decaps(&key_pair2, &ct2).expect("Decaps should succeed");
    assert_eq!(ss2_1, ss2_2);
}

/// Test error conditions
#[test]
fn test_error_conditions() {
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level1,
    );

    // Test invalid ciphertext splitting
    let invalid_ciphertext = vec![1, 2, 3]; // Too short
    let result = hybrid_kem.split_ciphertext(&invalid_ciphertext);
    assert!(result.is_err());

    // Test XOR with mismatched lengths
    let classical = vec![1, 2, 3];
    let pqc = vec![4, 5]; // Different length
    let result = hybrid_kem.combine_shared_secrets(
        classical,
        pqc,
        KeyCombinerType::Xor,
    );
    assert!(result.is_err());
}

/// Test security properties
#[test]
fn test_security_properties() {
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level3,
    );

    // Generate multiple key pairs and ensure they're different
    let key_pair1 = hybrid_kem.keygen().expect("Keygen should succeed");
    let key_pair2 = hybrid_kem.keygen().expect("Keygen should succeed");
    
    assert_ne!(key_pair1.classical_public, key_pair2.classical_public);
    assert_ne!(key_pair1.classical_secret, key_pair2.classical_secret);
    assert_ne!(key_pair1.pqc_public, key_pair2.pqc_public);
    assert_ne!(key_pair1.pqc_secret, key_pair2.pqc_secret);

    // Test that encapsulation produces different results each time
    let (ct1, ss1) = hybrid_kem.encaps(&key_pair1).expect("Encaps should succeed");
    let (ct2, ss2) = hybrid_kem.encaps(&key_pair1).expect("Encaps should succeed");
    
    // Different ciphertexts but same shared secret when decapsulated
    assert_ne!(ct1, ct2); // Ciphertexts should be different due to randomness
    
    let ss1_check = hybrid_kem.decaps(&key_pair1, &ct1).expect("Decaps should succeed");
    let ss2_check = hybrid_kem.decaps(&key_pair1, &ct2).expect("Decaps should succeed");
    
    assert_eq!(ss1, ss1_check);
    assert_eq!(ss2, ss2_check);
}

/// Test interoperability between different security levels
#[test]
fn test_security_level_interoperability() {
    let levels = vec![SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5];
    
    for level in levels {
        let hybrid_kem = HybridKem::new(
            ClassicalAlgorithm::X25519,
            AlgorithmType::Kyber,
            level,
        );
        
        let key_pair = hybrid_kem.keygen()
            .expect(&format!("Keygen should succeed for level {:?}", level));
        
        let (ciphertext, shared_secret1) = hybrid_kem.encaps(&key_pair)
            .expect("Encaps should succeed");
        let shared_secret2 = hybrid_kem.decaps(&key_pair, &ciphertext)
            .expect("Decaps should succeed");
        
        assert_eq!(shared_secret1, shared_secret2);
        assert_eq!(shared_secret1.len(), 32); // Standard shared secret size
    }
}

/// Benchmark basic operations (for performance validation)
#[test]
#[ignore = "Performance test - run with cargo test --ignored"]
fn benchmark_hybrid_operations() {
    let hybrid_kem = HybridKem::new(
        ClassicalAlgorithm::X25519,
        AlgorithmType::Kyber,
        SecurityLevel::Level3,
    );

    // Benchmark key generation
    let start = std::time::Instant::now();
    let key_pair = hybrid_kem.keygen().expect("Keygen should succeed");
    let keygen_time = start.elapsed();
    println!("Key generation time: {:?}", keygen_time);
    
    // Benchmark encapsulation
    let start = std::time::Instant::now();
    let (ciphertext, _shared_secret) = hybrid_kem.encaps(&key_pair)
        .expect("Encaps should succeed");
    let encaps_time = start.elapsed();
    println!("Encapsulation time: {:?}", encaps_time);
    
    // Benchmark decapsulation
    let start = std::time::Instant::now();
    let _shared_secret = hybrid_kem.decaps(&key_pair, &ciphertext)
        .expect("Decaps should succeed");
    let decaps_time = start.elapsed();
    println!("Decapsulation time: {:?}", decaps_time);
    
    // Performance targets (these are rough guidelines)
    assert!(keygen_time < Duration::from_millis(100), "Key generation should be under 100ms");
    assert!(encaps_time < Duration::from_millis(50), "Encapsulation should be under 50ms");
    assert!(decaps_time < Duration::from_millis(50), "Decapsulation should be under 50ms");
}
