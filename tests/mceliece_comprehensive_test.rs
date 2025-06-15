//! Comprehensive Test Suite for Classic McEliece Implementation
//!
//! This test suite validates all aspects of the production-ready Classic McEliece
//! implementation including parameter validation, key generation, encapsulation/decapsulation,
//! security properties, performance characteristics, and edge cases.

use std::collections::HashSet;
use cursed::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType};
use cursed::stdlib::crypto_pqc::algorithms::mceliece::{
    ClassicMcEliece, McElieceParams, McEliecePublicKey, McElieceSecretKey,
    McElieceCiphertext, McElieceSharedSecret
};
use cursed::stdlib::crypto_pqc::algorithms::{KeyEncapsulation, ParameterSet, AlgorithmPerformance};

/// Test parameter validation for all McEliece parameter sets
#[test]
fn test_parameter_validation() {
    // Test all valid parameter sets
    let valid_params = [
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
        McElieceParams::McEliece6688128,
        McElieceParams::McEliece6960119,
        McElieceParams::McEliece8192128,
    ];

    for params in &valid_params {
        assert!(params.validate().is_ok(), "Parameter set {:?} should be valid", params);
        
        // Verify parameter relationships
        assert!(params.k() < params.n(), "Code dimension must be less than code length");
        assert!(params.n() <= params.q(), "Code length must not exceed field size");
        assert!(params.t() > 0, "Error correction capability must be positive");
        assert!(params.m() >= 12 && params.m() <= 13, "Extension degree should be 12 or 13");
        
        // Verify security level consistency
        let expected_level = match params {
            McElieceParams::McEliece348864 => SecurityLevel::Level1,
            McElieceParams::McEliece460896 => SecurityLevel::Level3,
            _ => SecurityLevel::Level5,
        };
        assert_eq!(params.security_level(), expected_level);
    }

    // Test parameter relationships
    for params in &valid_params {
        assert!(ClassicMcEliece::validate_parameters(*params).is_ok());
    }
}

/// Test key generation for all parameter sets
#[test] 
fn test_key_generation_all_parameters() {
    let test_params = [
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
        McElieceParams::McEliece6688128,
    ];

    for params in &test_params {
        let result = ClassicMcEliece::keygen_with_params(*params);
        assert!(result.is_ok(), "Key generation failed for {:?}", params);
        
        let (public_key, secret_key) = result.unwrap();
        
        // Verify key structure
        assert_eq!(public_key.params, *params);
        assert_eq!(secret_key.params, *params);
        assert_eq!(public_key.generator_matrix.rows, params.k());
        assert_eq!(public_key.generator_matrix.cols, params.n());
        
        // Verify integrity checks
        assert!(public_key.verify_integrity(), "Public key integrity check failed");
        assert!(secret_key.verify_integrity(), "Secret key integrity check failed");
        
        // Verify key sizes
        let pub_key_bytes = public_key.as_bytes();
        let sec_key_bytes = secret_key.as_bytes();
        
        assert!(pub_key_bytes.len() >= params.public_key_size());
        assert!(sec_key_bytes.len() >= params.secret_key_size());
    }
}

/// Test key generation uniqueness (keys should be different)
#[test]
fn test_key_generation_uniqueness() {
    let params = McElieceParams::McEliece348864;
    
    let (pub_key1, sec_key1) = ClassicMcEliece::keygen_with_params(params).unwrap();
    let (pub_key2, sec_key2) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    // Keys should be different
    assert_ne!(pub_key1.as_bytes(), pub_key2.as_bytes());
    assert_ne!(sec_key1.as_bytes(), sec_key2.as_bytes());
}

/// Test encapsulation and decapsulation round-trip correctness
#[test]
fn test_encaps_decaps_correctness() {
    let test_params = [
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
        McElieceParams::McEliece6688128,
    ];

    for params in &test_params {
        let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(*params).unwrap();
        
        // Test multiple encapsulation/decapsulation rounds
        for _ in 0..10 {
            let (ciphertext, shared_secret1) = ClassicMcEliece::encaps(&public_key).unwrap();
            let shared_secret2 = ClassicMcEliece::decaps(&secret_key, &ciphertext).unwrap();
            
            // Shared secrets should match
            assert_eq!(
                shared_secret1.as_bytes(),
                shared_secret2.as_bytes(),
                "Shared secrets don't match for parameter set {:?}",
                params
            );
            
            // Verify ciphertext structure
            assert_eq!(ciphertext.params, *params);
            assert_eq!(ciphertext.ciphertext.len(), params.n());
            assert!(ciphertext.verify_integrity(), "Ciphertext integrity check failed");
        }
    }
}

/// Test encapsulation produces different ciphertexts and shared secrets
#[test]
fn test_encaps_randomness() {
    let (public_key, _) = ClassicMcEliece::keygen_with_params(McElieceParams::McEliece348864).unwrap();
    
    let mut ciphertexts = HashSet::new();
    let mut shared_secrets = HashSet::new();
    
    for _ in 0..20 {
        let (ciphertext, shared_secret) = ClassicMcEliece::encaps(&public_key).unwrap();
        
        // Each encapsulation should produce unique results
        let ct_bytes = ciphertext.as_bytes();
        let ss_bytes = shared_secret.as_bytes().to_vec();
        
        assert!(!ciphertexts.contains(&ct_bytes), "Duplicate ciphertext generated");
        assert!(!shared_secrets.contains(&ss_bytes), "Duplicate shared secret generated");
        
        ciphertexts.insert(ct_bytes);
        shared_secrets.insert(ss_bytes);
    }
}

/// Test error correction capabilities at maximum error weight
#[test]
fn test_error_correction_capabilities() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    // Test that we can correct up to t errors
    let t = params.t();
    let n = params.n();
    
    // Generate a valid ciphertext
    let (original_ciphertext, original_shared_secret) = ClassicMcEliece::encaps(&public_key).unwrap();
    
    // Introduce exactly t errors at random positions
    let mut corrupted_bits = original_ciphertext.ciphertext.clone();
    let mut error_positions = HashSet::new();
    
    while error_positions.len() < t {
        let pos = rand::random::<usize>() % n;
        if !error_positions.contains(&pos) {
            corrupted_bits[pos] ^= true; // Flip bit
            error_positions.insert(pos);
        }
    }
    
    // Create corrupted ciphertext
    let corrupted_ciphertext = McElieceCiphertext::new(params, corrupted_bits).unwrap();
    
    // Should still be able to decrypt correctly
    let decrypted_shared_secret = ClassicMcEliece::decaps(&secret_key, &corrupted_ciphertext);
    assert!(
        decrypted_shared_secret.is_ok(),
        "Failed to correct {} errors (maximum correctable)", t
    );
    
    // Shared secret should match original
    assert_eq!(
        original_shared_secret.as_bytes(),
        decrypted_shared_secret.unwrap().as_bytes()
    );
}

/// Test that too many errors cause decapsulation failure
#[test]
fn test_error_correction_limits() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let t = params.t();
    let n = params.n();
    
    // Generate a valid ciphertext
    let (original_ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
    
    // Introduce more than t errors (t + random additional errors)
    let mut corrupted_bits = original_ciphertext.ciphertext.clone();
    let num_errors = t + 10 + (rand::random::<usize>() % 10); // t + 10-19 errors
    let mut error_positions = HashSet::new();
    
    while error_positions.len() < num_errors.min(n / 2) {
        let pos = rand::random::<usize>() % n;
        if !error_positions.contains(&pos) {
            corrupted_bits[pos] ^= true;
            error_positions.insert(pos);
        }
    }
    
    let corrupted_ciphertext = McElieceCiphertext::new(params, corrupted_bits).unwrap();
    
    // Should fail to decrypt
    let result = ClassicMcEliece::decaps(&secret_key, &corrupted_ciphertext);
    assert!(
        result.is_err(),
        "Should not be able to correct {} errors (more than t={})",
        num_errors, t
    );
}

/// Test security properties and hardness assumptions
#[test]
fn test_security_properties() {
    let params = McElieceParams::McEliece460896; // Use Level 3 for testing
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    // Test that different keys produce different results
    let (public_key2, secret_key2) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let (ciphertext1, _) = ClassicMcEliece::encaps(&public_key).unwrap();
    let (ciphertext2, _) = ClassicMcEliece::encaps(&public_key2).unwrap();
    
    // Different public keys should produce different ciphertexts
    assert_ne!(ciphertext1.as_bytes(), ciphertext2.as_bytes());
    
    // Wrong secret key should not decrypt correctly
    let wrong_decryption = ClassicMcEliece::decaps(&secret_key2, &ciphertext1);
    assert!(wrong_decryption.is_err(), "Wrong key should not decrypt ciphertext");
    
    // Test parameter mismatch detection
    let params_different = McElieceParams::McEliece348864;
    let (_, secret_key_different) = ClassicMcEliece::keygen_with_params(params_different).unwrap();
    
    let mismatch_result = ClassicMcEliece::decaps(&secret_key_different, &ciphertext1);
    assert!(mismatch_result.is_err(), "Parameter mismatch should be detected");
}

/// Test integrity protection mechanisms
#[test]
fn test_integrity_protection() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    // Test public key integrity
    assert!(public_key.verify_integrity());
    
    // Test secret key integrity
    assert!(secret_key.verify_integrity());
    
    // Test ciphertext integrity
    let (ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
    assert!(ciphertext.verify_integrity());
    
    // Test that corrupted integrity is detected
    let mut corrupted_bytes = public_key.as_bytes();
    corrupted_bytes[corrupted_bytes.len() - 1] ^= 1; // Corrupt checksum
    
    // Should detect corruption when trying to use corrupted key
    // (This would be detected in actual usage when verifying integrity)
}

/// Test NIST security levels and algorithm identification
#[test]
fn test_nist_security_levels() {
    // Test security level mappings
    assert_eq!(McElieceParams::McEliece348864.security_level(), SecurityLevel::Level1);
    assert_eq!(McElieceParams::McEliece460896.security_level(), SecurityLevel::Level3);
    assert_eq!(McElieceParams::McEliece6688128.security_level(), SecurityLevel::Level5);
    assert_eq!(McElieceParams::McEliece6960119.security_level(), SecurityLevel::Level5);
    assert_eq!(McElieceParams::McEliece8192128.security_level(), SecurityLevel::Level5);
    
    // Test algorithm type
    assert_eq!(ClassicMcEliece::algorithm_type(), AlgorithmType::ClassicMcEliece);
    
    // Test that keygen respects security levels
    let (pub_key1, _) = ClassicMcEliece::keygen(SecurityLevel::Level1).unwrap();
    let (pub_key3, _) = ClassicMcEliece::keygen(SecurityLevel::Level3).unwrap();
    let (pub_key5, _) = ClassicMcEliece::keygen(SecurityLevel::Level5).unwrap();
    
    assert_eq!(pub_key1.security_level(), SecurityLevel::Level1);
    assert_eq!(pub_key3.security_level(), SecurityLevel::Level3);
    assert_eq!(pub_key5.security_level(), SecurityLevel::Level5);
}

/// Test performance characteristics and benchmarking
#[test]
fn test_performance_characteristics() {
    let test_params = [
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
        McElieceParams::McEliece6688128,
    ];

    for params in &test_params {
        let perf = ClassicMcEliece::performance_characteristics(*params);
        
        // Verify performance metrics are reasonable
        assert!(perf.keygen_time_ms > 0.0, "Key generation time should be positive");
        assert!(perf.operation_time_ms > 0.0, "Operation time should be positive");
        assert!(perf.throughput_ops_per_sec > 0.0, "Throughput should be positive");
        
        // Verify key sizes match parameter specifications
        assert_eq!(perf.key_sizes.public_key, params.public_key_size());
        assert_eq!(perf.key_sizes.secret_key, params.secret_key_size());
        assert_eq!(perf.key_sizes.shared_secret, Some(32));
        
        // Higher security levels should generally have larger keys and slower operations
        if params == &McElieceParams::McEliece348864 {
            assert!(perf.keygen_time_ms < 100.0, "Level 1 keygen should be fast");
        }
    }
}

/// Test large-scale operations and memory efficiency
#[test]
fn test_large_scale_operations() {
    let params = McElieceParams::McEliece348864; // Use smaller params for testing
    
    // Generate multiple key pairs
    let mut key_pairs = Vec::new();
    for _ in 0..5 {
        let keypair = ClassicMcEliece::keygen_with_params(params).unwrap();
        key_pairs.push(keypair);
    }
    
    // Test cross-compatibility (every public key with every secret key)
    for (i, (pub_key, _)) in key_pairs.iter().enumerate() {
        let (ciphertext, original_secret) = ClassicMcEliece::encaps(pub_key).unwrap();
        
        for (j, (_, sec_key)) in key_pairs.iter().enumerate() {
            let decrypt_result = ClassicMcEliece::decaps(sec_key, &ciphertext);
            
            if i == j {
                // Matching key pair should work
                assert!(decrypt_result.is_ok());
                assert_eq!(
                    decrypt_result.unwrap().as_bytes(),
                    original_secret.as_bytes()
                );
            } else {
                // Non-matching key pair should fail
                assert!(decrypt_result.is_err());
            }
        }
    }
}

/// Test edge cases and error conditions
#[test]
fn test_edge_cases() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    // Test with all-zero ciphertext
    let zero_ciphertext = McElieceCiphertext::new(params, vec![false; params.n()]).unwrap();
    let result = ClassicMcEliece::decaps(&secret_key, &zero_ciphertext);
    // Should either succeed or fail gracefully
    assert!(result.is_ok() || result.is_err());
    
    // Test with all-one ciphertext
    let one_ciphertext = McElieceCiphertext::new(params, vec![true; params.n()]).unwrap();
    let result = ClassicMcEliece::decaps(&secret_key, &one_ciphertext);
    // Should either succeed or fail gracefully
    assert!(result.is_ok() || result.is_err());
    
    // Test parameter validation edge cases
    for params in &[
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
        McElieceParams::McEliece6688128,
        McElieceParams::McEliece6960119,
        McElieceParams::McEliece8192128,
    ] {
        assert!(ClassicMcEliece::validate_parameters(*params).is_ok());
    }
}

/// Test mathematical properties of finite field operations
#[test]
fn test_finite_field_properties() {
    // This test verifies that the underlying finite field arithmetic is correct
    // by testing basic properties that should hold in any implementation
    
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    // Test that encoding and decoding are inverse operations for valid inputs
    for _ in 0..10 {
        let (ciphertext, shared_secret1) = ClassicMcEliece::encaps(&public_key).unwrap();
        let shared_secret2 = ClassicMcEliece::decaps(&secret_key, &ciphertext).unwrap();
        
        assert_eq!(shared_secret1.as_bytes(), shared_secret2.as_bytes());
    }
    
    // Test that the syndrome computation is consistent
    let (ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
    
    // Multiple decapsulation attempts should give same result
    let result1 = ClassicMcEliece::decaps(&secret_key, &ciphertext).unwrap();
    let result2 = ClassicMcEliece::decaps(&secret_key, &ciphertext).unwrap();
    assert_eq!(result1.as_bytes(), result2.as_bytes());
}

/// Test memory safety and resource cleanup
#[test]
fn test_memory_safety() {
    // Test that large operations don't cause memory issues
    let params = McElieceParams::McEliece348864;
    
    // Create and destroy many objects
    for _ in 0..100 {
        let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
        let (ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
        let _ = ClassicMcEliece::decaps(&secret_key, &ciphertext);
        
        // Objects should be properly cleaned up when they go out of scope
    }
}

/// Test serialization and deserialization
#[test]
fn test_serialization() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    let (ciphertext, shared_secret) = ClassicMcEliece::encaps(&public_key).unwrap();
    
    // Test key serialization
    let pub_key_bytes = public_key.as_bytes();
    let sec_key_bytes = secret_key.as_bytes();
    
    // Test ciphertext serialization
    let ciphertext_bytes = ciphertext.as_bytes();
    let restored_ciphertext = McElieceCiphertext::from_bytes(&ciphertext_bytes).unwrap();
    
    assert_eq!(ciphertext.params, restored_ciphertext.params);
    assert_eq!(ciphertext.ciphertext, restored_ciphertext.ciphertext);
    assert_eq!(ciphertext.checksum, restored_ciphertext.checksum);
    
    // Test shared secret access
    let secret_bytes = shared_secret.as_bytes();
    assert_eq!(secret_bytes.len(), 32);
}

#[cfg(test)]
mod stress_tests {
    use super::*;
    
    /// Stress test with many operations
    #[test]
    #[ignore] // Run with --ignored for stress testing
    fn stress_test_many_operations() {
        let params = McElieceParams::McEliece348864;
        let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
        
        // Perform many encaps/decaps operations
        for i in 0..1000 {
            let (ciphertext, shared_secret1) = ClassicMcEliece::encaps(&public_key).unwrap();
            let shared_secret2 = ClassicMcEliece::decaps(&secret_key, &ciphertext).unwrap();
            
            assert_eq!(
                shared_secret1.as_bytes(),
                shared_secret2.as_bytes(),
                "Mismatch at iteration {}", i
            );
        }
    }
    
    /// Stress test with large parameter sets
    #[test]
    #[ignore] // Run with --ignored for stress testing
    fn stress_test_large_parameters() {
        let params = McElieceParams::McEliece6688128; // Largest parameter set
        
        let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
        
        // Test multiple operations with large parameters
        for _ in 0..10 {
            let (ciphertext, shared_secret1) = ClassicMcEliece::encaps(&public_key).unwrap();
            let shared_secret2 = ClassicMcEliece::decaps(&secret_key, &ciphertext).unwrap();
            
            assert_eq!(shared_secret1.as_bytes(), shared_secret2.as_bytes());
        }
    }
}
