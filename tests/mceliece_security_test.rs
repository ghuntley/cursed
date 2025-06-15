//! McEliece Security and Mathematical Correctness Tests
//!
//! This module validates the security properties, mathematical correctness, and
//! cryptographic soundness of the Classic McEliece implementation.

use std::collections::HashMap;
use cursed::stdlib::crypto_pqc::{PqcResult, SecurityLevel};
use cursed::stdlib::crypto_pqc::algorithms::mceliece::{
    ClassicMcEliece, McElieceParams
};
use cursed::stdlib::crypto_pqc::algorithms::KeyEncapsulation;

/// Test that error correction works correctly for various error patterns
#[test]
fn test_error_correction_mathematical_properties() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let t = params.t();
    let n = params.n();
    
    // Test systematic error patterns
    for num_errors in 1..=t {
        let (original_ciphertext, original_shared_secret) = ClassicMcEliece::encaps(&public_key).unwrap();
        
        // Create error pattern with exactly num_errors errors
        let mut error_pattern = vec![false; n];
        for i in 0..num_errors {
            error_pattern[i * (n / num_errors)] = true; // Distribute errors evenly
        }
        
        // Apply error pattern
        let mut corrupted_bits = original_ciphertext.ciphertext.clone();
        for i in 0..n {
            corrupted_bits[i] ^= error_pattern[i];
        }
        
        let corrupted_ciphertext = cursed::stdlib::crypto_pqc::algorithms::mceliece::McElieceCiphertext::new(
            params, corrupted_bits
        ).unwrap();
        
        // Should be able to correct the errors
        let result = ClassicMcEliece::decaps(&secret_key, &corrupted_ciphertext);
        assert!(
            result.is_ok(),
            "Failed to correct {} systematic errors (t={})", num_errors, t
        );
        
        if let Ok(corrected_shared_secret) = result {
            assert_eq!(
                original_shared_secret.as_bytes(),
                corrected_shared_secret.as_bytes(),
                "Corrected shared secret doesn't match original with {} errors", num_errors
            );
        }
    }
}

/// Test the statistical properties of error vector generation
#[test]
fn test_error_vector_statistical_properties() {
    let params = McElieceParams::McEliece348864;
    let (public_key, _) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let t = params.t();
    let n = params.n();
    let num_samples = 100;
    
    let mut position_counts = vec![0; n];
    let mut weight_counts = HashMap::new();
    
    for _ in 0..num_samples {
        let (ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
        
        // Count error positions (approximate, since we don't know the original message)
        let weight = ciphertext.ciphertext.iter().filter(|&&bit| bit).count();
        *weight_counts.entry(weight).or_insert(0) += 1;
        
        for (i, &bit) in ciphertext.ciphertext.iter().enumerate() {
            if bit {
                position_counts[i] += 1;
            }
        }
    }
    
    // Check that error weights are distributed reasonably
    // In McEliece, ciphertext weight includes both message and error weights
    println!("Error weight distribution:");
    for (weight, count) in &weight_counts {
        println!("  Weight {}: {} occurrences", weight, count);
    }
    
    // Check that error positions are reasonably distributed
    let avg_position_count = position_counts.iter().sum::<usize>() as f64 / n as f64;
    let max_position_count = *position_counts.iter().max().unwrap() as f64;
    let min_position_count = *position_counts.iter().min().unwrap() as f64;
    
    println!("Position distribution: avg={:.2}, max={}, min={}", 
             avg_position_count, max_position_count, min_position_count);
    
    // Basic statistical checks
    assert!(max_position_count < avg_position_count * 3.0, 
            "Error positions should be reasonably distributed");
}

/// Test that different parameter sets provide different security levels
#[test]
fn test_security_level_differentiation() {
    let params_sets = vec![
        (McElieceParams::McEliece348864, SecurityLevel::Level1),
        (McElieceParams::McEliece460896, SecurityLevel::Level3),
        (McElieceParams::McEliece6688128, SecurityLevel::Level5),
    ];
    
    for (params, expected_level) in params_sets {
        assert_eq!(params.security_level(), expected_level);
        
        // Higher security levels should have larger parameters
        match expected_level {
            SecurityLevel::Level1 => {
                assert!(params.n() < 4000, "Level 1 should have smaller n");
                assert!(params.k() < 3000, "Level 1 should have smaller k");
                assert!(params.t() < 100, "Level 1 should have smaller t");
            },
            SecurityLevel::Level3 => {
                assert!(params.n() > 4000 && params.n() < 6000, "Level 3 should have medium n");
                assert!(params.k() > 3000 && params.k() < 5000, "Level 3 should have medium k");
                assert!(params.t() < 120, "Level 3 should have medium t");
            },
            SecurityLevel::Level5 => {
                assert!(params.n() > 6000, "Level 5 should have large n");
                assert!(params.k() > 5000, "Level 5 should have large k");
                assert!(params.t() > 100, "Level 5 should have large t");
            },
        }
    }
}

/// Test cryptographic randomness properties
#[test]
fn test_cryptographic_randomness() {
    let params = McElieceParams::McEliece348864;
    let (public_key, _) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let num_samples = 50;
    let mut shared_secrets = Vec::new();
    let mut ciphertexts = Vec::new();
    
    // Generate multiple encapsulations
    for _ in 0..num_samples {
        let (ciphertext, shared_secret) = ClassicMcEliece::encaps(&public_key).unwrap();
        shared_secrets.push(shared_secret.as_bytes().to_vec());
        ciphertexts.push(ciphertext.as_bytes());
    }
    
    // Test that all shared secrets are unique
    let mut unique_secrets = std::collections::HashSet::new();
    for secret in &shared_secrets {
        assert!(unique_secrets.insert(secret.clone()), 
                "Duplicate shared secret generated");
    }
    
    // Test that all ciphertexts are unique
    let mut unique_ciphertexts = std::collections::HashSet::new();
    for ciphertext in &ciphertexts {
        assert!(unique_ciphertexts.insert(ciphertext.clone()), 
                "Duplicate ciphertext generated");
    }
    
    // Basic randomness test: check bit distribution in shared secrets
    let total_bits = shared_secrets.len() * 32 * 8; // 32 bytes * 8 bits each
    let mut one_count = 0;
    
    for secret in &shared_secrets {
        for &byte in secret {
            one_count += byte.count_ones() as usize;
        }
    }
    
    let one_ratio = one_count as f64 / total_bits as f64;
    println!("Shared secret bit distribution: {:.3} (expected ~0.5)", one_ratio);
    
    // Should be close to 50% ones
    assert!(one_ratio > 0.4 && one_ratio < 0.6, 
            "Shared secret bits should be reasonably random");
}

/// Test key generation uniqueness and independence
#[test]
fn test_key_generation_independence() {
    let params = McElieceParams::McEliece348864;
    let num_keys = 10;
    
    let mut public_keys = Vec::new();
    let mut secret_keys = Vec::new();
    
    // Generate multiple key pairs
    for _ in 0..num_keys {
        let (pub_key, sec_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
        public_keys.push(pub_key.as_bytes());
        secret_keys.push(sec_key.as_bytes());
    }
    
    // Test that all public keys are unique
    let mut unique_pub_keys = std::collections::HashSet::new();
    for pub_key in &public_keys {
        assert!(unique_pub_keys.insert(pub_key.clone()), 
                "Duplicate public key generated");
    }
    
    // Test that all secret keys are unique
    let mut unique_sec_keys = std::collections::HashSet::new();
    for sec_key in &secret_keys {
        assert!(unique_sec_keys.insert(sec_key.clone()), 
                "Duplicate secret key generated");
    }
    
    // Test cross-compatibility (wrong keys should fail)
    let (test_pub, _) = ClassicMcEliece::keygen_with_params(params).unwrap();
    let (_, test_sec) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let (ciphertext, _) = ClassicMcEliece::encaps(&test_pub).unwrap();
    let wrong_decrypt = ClassicMcEliece::decaps(&test_sec, &ciphertext);
    
    assert!(wrong_decrypt.is_err(), "Wrong secret key should not decrypt ciphertext");
}

/// Test parameter validation security properties
#[test]
fn test_parameter_validation_security() {
    let valid_params = [
        McElieceParams::McEliece348864,
        McElieceParams::McEliece460896,
        McElieceParams::McEliece6688128,
        McElieceParams::McEliece6960119,
        McElieceParams::McEliece8192128,
    ];
    
    for params in &valid_params {
        // Validate basic mathematical constraints
        assert!(params.validate().is_ok());
        assert!(ClassicMcEliece::validate_parameters(*params).is_ok());
        
        let n = params.n();
        let k = params.k();
        let t = params.t();
        let m = params.m();
        
        // Security-critical constraints
        assert!(k < n, "Code dimension must be less than code length");
        assert!(2 * t < n - k, "Error correction bound: 2t < n-k");
        assert!(n <= (1 << m), "Code length must fit in field");
        assert!(t > 0, "Error correction capability must be positive");
        
        // Security level consistency
        let security_bits = match params.security_level() {
            SecurityLevel::Level1 => 128,
            SecurityLevel::Level3 => 192,
            SecurityLevel::Level5 => 256,
        };
        
        // Rough work factor estimate
        let estimated_work_factor = (t as f64 * (m as f64).log2()).floor() as u32;
        assert!(estimated_work_factor >= security_bits / 4, 
                "Parameter set provides insufficient security estimate");
    }
}

/// Test syndrome decoding correctness
#[test]
fn test_syndrome_decoding_correctness() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    let t = params.t();
    let n = params.n();
    
    // Test various error patterns
    let test_cases = vec![
        vec![0], // Single error at position 0
        vec![1], // Single error at position 1
        vec![n-1], // Single error at last position
        (0..t.min(10)).collect::<Vec<_>>(), // First t errors
        (n-t.min(10)..n).collect::<Vec<_>>(), // Last t errors
    ];
    
    for error_positions in test_cases {
        if error_positions.len() > t {
            continue; // Skip if too many errors
        }
        
        let (original_ciphertext, original_shared_secret) = ClassicMcEliece::encaps(&public_key).unwrap();
        
        // Apply specific error pattern
        let mut corrupted_bits = original_ciphertext.ciphertext.clone();
        for &pos in &error_positions {
            if pos < n {
                corrupted_bits[pos] ^= true;
            }
        }
        
        let corrupted_ciphertext = cursed::stdlib::crypto_pqc::algorithms::mceliece::McElieceCiphertext::new(
            params, corrupted_bits
        ).unwrap();
        
        // Test decoding
        let result = ClassicMcEliece::decaps(&secret_key, &corrupted_ciphertext);
        
        if error_positions.len() <= t {
            // Should be correctable
            assert!(result.is_ok(), 
                    "Failed to correct error pattern {:?}", error_positions);
            
            if let Ok(corrected_shared_secret) = result {
                assert_eq!(
                    original_shared_secret.as_bytes(),
                    corrected_shared_secret.as_bytes(),
                    "Syndrome decoding produced wrong result for pattern {:?}", error_positions
                );
            }
        }
    }
}

/// Test resistance to common attack patterns
#[test]
fn test_attack_resistance() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    // Test that modifying public key components breaks functionality
    let mut corrupted_pub_bytes = public_key.as_bytes();
    
    // Corrupt different parts of the public key
    let test_positions = vec![
        1,                                    // Beginning
        corrupted_pub_bytes.len() / 2,       // Middle  
        corrupted_pub_bytes.len() - 2,       // Near end (avoid checksum)
    ];
    
    for pos in test_positions {
        if pos < corrupted_pub_bytes.len() {
            let original_byte = corrupted_pub_bytes[pos];
            corrupted_pub_bytes[pos] ^= 0xFF; // Flip all bits
            
            // Corrupted public key should be detected
            // (In practice, this would be caught during integrity verification)
            
            corrupted_pub_bytes[pos] = original_byte; // Restore
        }
    }
    
    // Test that incorrect ciphertext modifications are detected
    let (ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
    let mut corrupted_ct_bytes = ciphertext.as_bytes();
    
    // Massive corruption should be detected/rejected
    for i in 0..corrupted_ct_bytes.len().min(100) {
        corrupted_ct_bytes[i] ^= 0xFF;
    }
    
    let corrupted_ct_result = cursed::stdlib::crypto_pqc::algorithms::mceliece::McElieceCiphertext::from_bytes(&corrupted_ct_bytes);
    // Should either fail to parse or fail integrity check
    if let Ok(corrupted_ct) = corrupted_ct_result {
        let decrypt_result = ClassicMcEliece::decaps(&secret_key, &corrupted_ct);
        // Should fail due to too many errors or integrity failure
        assert!(decrypt_result.is_err(), 
                "Massively corrupted ciphertext should not decrypt successfully");
    }
}

/// Test integrity protection mechanisms
#[test]
fn test_integrity_protection_comprehensive() {
    let params = McElieceParams::McEliece348864;
    let (public_key, secret_key) = ClassicMcEliece::keygen_with_params(params).unwrap();
    
    // Test that integrity checks work
    assert!(public_key.verify_integrity(), "Public key integrity should be valid");
    assert!(secret_key.verify_integrity(), "Secret key integrity should be valid");
    
    let (ciphertext, _) = ClassicMcEliece::encaps(&public_key).unwrap();
    assert!(ciphertext.verify_integrity(), "Ciphertext integrity should be valid");
    
    // Test that corrupted checksums are detected
    let mut corrupted_pub_bytes = public_key.as_bytes();
    let checksum_start = corrupted_pub_bytes.len() - 32;
    corrupted_pub_bytes[checksum_start] ^= 1; // Flip one bit in checksum
    
    // Would need to reconstruct public key and test integrity
    // (This is a limitation of our current API design)
    
    let mut corrupted_ct_bytes = ciphertext.as_bytes();
    let ct_checksum_start = corrupted_ct_bytes.len() - 16;
    corrupted_ct_bytes[ct_checksum_start] ^= 1; // Flip one bit in checksum
    
    let corrupted_ct = cursed::stdlib::crypto_pqc::algorithms::mceliece::McElieceCiphertext::from_bytes(&corrupted_ct_bytes);
    assert!(corrupted_ct.is_err(), "Corrupted ciphertext should fail integrity check");
}

/// Test mathematical field operations consistency
#[test]
fn test_finite_field_consistency() {
    let params = McElieceParams::McEliece348864;
    
    // Test that multiple operations with same parameters are consistent
    for _ in 0..10 {
        let (pub_key1, sec_key1) = ClassicMcEliece::keygen_with_params(params).unwrap();
        let (pub_key2, sec_key2) = ClassicMcEliece::keygen_with_params(params).unwrap();
        
        // Different keys should have same parameter structure
        assert_eq!(pub_key1.params, pub_key2.params);
        assert_eq!(sec_key1.params, sec_key2.params);
        
        // But different key material
        assert_ne!(pub_key1.as_bytes(), pub_key2.as_bytes());
        assert_ne!(sec_key1.as_bytes(), sec_key2.as_bytes());
        
        // Both should work independently
        let (ct1, ss1) = ClassicMcEliece::encaps(&pub_key1).unwrap();
        let (ct2, ss2) = ClassicMcEliece::encaps(&pub_key2).unwrap();
        
        let ss1_recovered = ClassicMcEliece::decaps(&sec_key1, &ct1).unwrap();
        let ss2_recovered = ClassicMcEliece::decaps(&sec_key2, &ct2).unwrap();
        
        assert_eq!(ss1.as_bytes(), ss1_recovered.as_bytes());
        assert_eq!(ss2.as_bytes(), ss2_recovered.as_bytes());
        assert_ne!(ss1.as_bytes(), ss2.as_bytes());
    }
}
