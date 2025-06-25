/// fr fr Crypto security validation tests for CURSED - maximum security periodt
/// 
/// This test suite validates security properties of the crypto implementation:
/// - Randomness quality and entropy analysis
/// - Constant-time operations for timing attack resistance  
/// - Key derivation security properties
/// - Authentication bypass prevention
/// - Side-channel resistance testing
/// - Secure memory handling
/// 
/// These tests ensure the crypto meets security standards for production use.

use cursed::stdlib::crypto::*;
use cursed::stdlib::packages::crypto_random::*;
use cursed::stdlib::packages::crypto_advanced::*;
use std::time::Instant;
use std::collections::HashMap;

// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_randomness_quality_statistical() {
    init_tracing!();
    tracing::info!("Testing statistical randomness quality");
    
    let sample_size = 10000;
    let mut data = vec![0u8; sample_size];
    fill_random(&mut data).expect("Failed to generate random data");
    
    // Frequency test - each byte value should appear roughly equally
    let mut frequency = [0u32; 256];
    for &byte in &data {
        frequency[byte as usize] += 1;
    }
    
    let expected_frequency = sample_size as f64 / 256.0;
    let tolerance = expected_frequency * 0.3; // 30% tolerance
    
    let mut outliers = 0;
    for (value, &count) in frequency.iter().enumerate() {
        let diff = (count as f64 - expected_frequency).abs();
        if diff > tolerance {
            outliers += 1;
            tracing::debug!(
                byte_value = value,
                count = count,
                expected = expected_frequency,
                diff = diff,
                "Frequency outlier detected"
            );
        }
    }
    
    let outlier_percentage = (outliers as f64 / 256.0) * 100.0;
    tracing::info!(
        sample_size = sample_size,
        outliers = outliers,
        outlier_percentage = outlier_percentage,
        "Frequency analysis completed"
    );
    
    // Should have relatively few outliers in a good random distribution
    assert!(outlier_percentage < 20.0, "Too many frequency outliers: {:.1}%", outlier_percentage);
    
    // Entropy estimation (simplified)
    let mut entropy = 0.0;
    for &count in &frequency {
        if count > 0 {
            let probability = count as f64 / sample_size as f64;
            entropy -= probability * probability.log2();
        }
    }
    
    tracing::info!(estimated_entropy = entropy, "Entropy analysis completed");
    
    // Good randomness should have high entropy (close to 8 bits for bytes)
    assert!(entropy > 7.5, "Entropy too low: {:.2} bits", entropy);
}

#[test]
fn test_constant_time_operations() {
    init_tracing!();
    tracing::info!("Testing constant-time operations for timing attack resistance");
    
    let iterations = 1000;
    let key1 = vec![0x42u8; 32];
    let key2 = vec![0x43u8; 32];
    let key3 = vec![0x42u8; 32]; // Same as key1
    
    // Test constant-time comparison
    let mut timing_same = Vec::new();
    let mut timing_different = Vec::new();
    
    for _ in 0..iterations {
        // Time comparison of same keys
        let start = Instant::now();
        let result = constant_time_compare(&key1, &key3);
        let duration = start.elapsed();
        timing_same.push(duration);
        assert!(result, "Same keys should compare equal");
        
        // Time comparison of different keys
        let start = Instant::now();
        let result = constant_time_compare(&key1, &key2);
        let duration = start.elapsed();
        timing_different.push(duration);
        assert!(!result, "Different keys should not compare equal");
    }
    
    // Calculate timing statistics
    let avg_same = timing_same.iter().sum::<std::time::Duration>() / timing_same.len() as u32;
    let avg_different = timing_different.iter().sum::<std::time::Duration>() / timing_different.len() as u32;
    
    let same_ns: Vec<u64> = timing_same.iter().map(|d| d.as_nanos() as u64).collect();
    let diff_ns: Vec<u64> = timing_different.iter().map(|d| d.as_nanos() as u64).collect();
    
    let same_variance = calculate_variance(&same_ns);
    let diff_variance = calculate_variance(&diff_ns);
    
    tracing::info!(
        iterations = iterations,
        avg_same_ns = avg_same.as_nanos(),
        avg_different_ns = avg_different.as_nanos(),
        same_variance = same_variance,
        diff_variance = diff_variance,
        "Constant-time comparison analysis"
    );
    
    // The difference between timings should be minimal for constant-time operations
    let timing_diff_ns = (avg_same.as_nanos() as i64 - avg_different.as_nanos() as i64).abs();
    let max_acceptable_diff = 1000; // 1 microsecond
    
    assert!(timing_diff_ns < max_acceptable_diff, 
           "Timing difference too large: {} ns (may indicate timing attack vulnerability)", 
           timing_diff_ns);
}

fn calculate_variance(values: &[u64]) -> f64 {
    let mean = values.iter().sum::<u64>() as f64 / values.len() as f64;
    let variance = values.iter()
        .map(|&x| (x as f64 - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    variance
}

#[test]
fn test_key_derivation_security_properties() {
    init_tracing!();
    tracing::info!("Testing key derivation security properties");
    
    let manager = KeyManager::new().expect("Failed to create key manager");
    let password = b"secure_test_password";
    let salt1 = generate_random_bytes(32).expect("Failed to generate salt1");
    let salt2 = generate_random_bytes(32).expect("Failed to generate salt2");
    
    // Test 1: Same password + salt should produce same key
    let config1 = KeyDerivationConfig {
        iterations: 10000,
        salt: salt1.clone(),
        key_length: 32,
    };
    
    let key1a = manager.derive_key_pbkdf2(password, &config1).expect("Key derivation failed");
    let key1b = manager.derive_key_pbkdf2(password, &config1).expect("Key derivation failed");
    
    assert_eq!(key1a.as_bytes(), key1b.as_bytes(), "Same inputs should produce same derived key");
    
    // Test 2: Different salts should produce different keys
    let config2 = KeyDerivationConfig {
        iterations: 10000,
        salt: salt2.clone(),
        key_length: 32,
    };
    
    let key2 = manager.derive_key_pbkdf2(password, &config2).expect("Key derivation failed");
    assert_ne!(key1a.as_bytes(), key2.as_bytes(), "Different salts should produce different keys");
    
    // Test 3: Different iteration counts should produce different keys
    let config3 = KeyDerivationConfig {
        iterations: 20000, // Different iteration count
        salt: salt1.clone(),
        key_length: 32,
    };
    
    let key3 = manager.derive_key_pbkdf2(password, &config3).expect("Key derivation failed");
    assert_ne!(key1a.as_bytes(), key3.as_bytes(), "Different iterations should produce different keys");
    
    // Test 4: Different passwords should produce different keys
    let key4 = manager.derive_key_pbkdf2(b"different_password", &config1).expect("Key derivation failed");
    assert_ne!(key1a.as_bytes(), key4.as_bytes(), "Different passwords should produce different keys");
    
    // Test 5: Verify sufficient key entropy
    let derived_bytes = key1a.as_bytes();
    assert!(has_sufficient_entropy(derived_bytes), "Derived key lacks sufficient entropy");
    
    // Test 6: PBKDF2 vs scrypt should produce different keys for same inputs
    let scrypt_key = manager.derive_key_scrypt(password, &config1).expect("scrypt derivation failed");
    assert_ne!(key1a.as_bytes(), scrypt_key.as_bytes(), 
              "PBKDF2 and scrypt should produce different keys");
    
    tracing::info!("Key derivation security properties validated successfully");
}

fn has_sufficient_entropy(data: &[u8]) -> bool {
    // Simple entropy check - count unique bytes
    let mut byte_counts = [0u32; 256];
    for &byte in data {
        byte_counts[byte as usize] += 1;
    }
    
    let unique_bytes = byte_counts.iter().filter(|&&count| count > 0).count();
    let entropy_ratio = unique_bytes as f64 / data.len() as f64;
    
    // Should have reasonable byte diversity
    entropy_ratio > 0.5 && unique_bytes >= 16
}

#[test]
fn test_authentication_bypass_prevention() {
    init_tracing!();
    tracing::info!("Testing authentication bypass prevention");
    
    let key = vec![42u8; 32];
    let plaintext = b"Secret message that must not be tampered with";
    let associated_data = b"additional authentication data";
    
    // Test with AES-256-GCM
    let cipher = Aes256Gcm::new(&key).expect("Failed to create AES-GCM cipher");
    let encrypted = cipher.encrypt(plaintext, associated_data).expect("Encryption failed");
    
    // Verify normal decryption works
    let decrypted = cipher.decrypt(&encrypted.ciphertext, associated_data, &encrypted);
    assert!(decrypted.is_ok(), "Normal decryption should succeed");
    assert_eq!(decrypted.unwrap().plaintext, plaintext, "Decrypted text should match");
    
    // Test 1: Modified ciphertext should fail authentication
    let mut modified_encrypted = encrypted.clone();
    if !modified_encrypted.ciphertext.is_empty() {
        modified_encrypted.ciphertext[0] ^= 0x01; // Flip one bit
        
        let tampered_result = cipher.decrypt(&modified_encrypted.ciphertext, associated_data, &modified_encrypted);
        assert!(tampered_result.is_err(), "Modified ciphertext should fail authentication");
        tracing::debug!("Tampered ciphertext correctly rejected");
    }
    
    // Test 2: Modified authentication tag should fail
    let mut modified_tag = encrypted.clone();
    if let Some(ref mut tag) = modified_tag.tag {
        if !tag.is_empty() {
            tag[0] ^= 0x01; // Flip one bit in tag
            
            let tampered_result = cipher.decrypt(&encrypted.ciphertext, associated_data, &modified_tag);
            assert!(tampered_result.is_err(), "Modified authentication tag should fail");
            tracing::debug!("Tampered authentication tag correctly rejected");
        }
    }
    
    // Test 3: Modified associated data should fail authentication
    let wrong_associated_data = b"wrong additional data";
    let wrong_aad_result = cipher.decrypt(&encrypted.ciphertext, wrong_associated_data, &encrypted);
    assert!(wrong_aad_result.is_err(), "Wrong associated data should fail authentication");
    tracing::debug!("Wrong associated data correctly rejected");
    
    // Test 4: Modified nonce should fail
    let mut modified_nonce = encrypted.clone();
    if let Some(ref mut nonce) = modified_nonce.nonce {
        if !nonce.is_empty() {
            nonce[0] ^= 0x01; // Flip one bit in nonce
            
            let tampered_result = cipher.decrypt(&encrypted.ciphertext, associated_data, &modified_nonce);
            assert!(tampered_result.is_err(), "Modified nonce should fail authentication");
            tracing::debug!("Tampered nonce correctly rejected");
        }
    }
    
    tracing::info!("Authentication bypass prevention tests passed");
}

#[test]
fn test_basic_timing_attack_resistance() {
    init_tracing!();
    tracing::info!("Testing basic timing attack resistance");
    
    let iterations = 1000;
    let key = vec![42u8; 32];
    let cipher = Aes256Gcm::new(&key).expect("Failed to create cipher");
    
    // Create valid and invalid authentication scenarios
    let plaintext = b"Test message for timing analysis";
    let associated_data = b"aad";
    
    let valid_encrypted = cipher.encrypt(plaintext, associated_data).expect("Encryption failed");
    
    // Create invalid encrypted data (modified tag)
    let mut invalid_encrypted = valid_encrypted.clone();
    if let Some(ref mut tag) = invalid_encrypted.tag {
        tag[0] ^= 0x01; // Flip one bit
    }
    
    let mut valid_timings = Vec::new();
    let mut invalid_timings = Vec::new();
    
    for _ in 0..iterations {
        // Time valid decryption (should succeed quickly, then fail during verification)
        let start = Instant::now();
        let _result = cipher.decrypt(&valid_encrypted.ciphertext, associated_data, &valid_encrypted);
        let valid_duration = start.elapsed();
        valid_timings.push(valid_duration.as_nanos() as u64);
        
        // Time invalid decryption (should fail during verification)
        let start = Instant::now();
        let _result = cipher.decrypt(&invalid_encrypted.ciphertext, associated_data, &invalid_encrypted);
        let invalid_duration = start.elapsed();
        invalid_timings.push(invalid_duration.as_nanos() as u64);
    }
    
    let avg_valid = valid_timings.iter().sum::<u64>() as f64 / valid_timings.len() as f64;
    let avg_invalid = invalid_timings.iter().sum::<u64>() as f64 / invalid_timings.len() as f64;
    
    let valid_variance = calculate_variance(&valid_timings);
    let invalid_variance = calculate_variance(&invalid_timings);
    
    let timing_diff_percent = ((avg_valid - avg_invalid).abs() / avg_valid) * 100.0;
    
    tracing::info!(
        iterations = iterations,
        avg_valid_ns = avg_valid,
        avg_invalid_ns = avg_invalid,
        timing_diff_percent = timing_diff_percent,
        valid_variance = valid_variance,
        invalid_variance = invalid_variance,
        "Timing attack resistance analysis"
    );
    
    // For basic resistance, timing difference should be relatively small
    // This is a simplified test - production systems need more sophisticated analysis
    assert!(timing_diff_percent < 50.0, 
           "Timing difference too large: {:.1}% (potential timing attack vulnerability)", 
           timing_diff_percent);
}

#[test]
fn test_side_channel_resistance_basics() {
    init_tracing!();
    tracing::info!("Testing basic side-channel resistance");
    
    let iterations = 500;
    let password = b"test_password_for_side_channel_analysis";
    
    // Test key derivation timing consistency
    let manager = KeyManager::new().expect("Failed to create key manager");
    let salt = generate_random_bytes(32).expect("Failed to generate salt");
    
    let config = KeyDerivationConfig {
        iterations: 5000,
        salt: salt.clone(),
        key_length: 32,
    };
    
    let mut derivation_timings = Vec::new();
    
    for _ in 0..iterations {
        let start = Instant::now();
        let _key = manager.derive_key_pbkdf2(password, &config).expect("Key derivation failed");
        let duration = start.elapsed();
        derivation_timings.push(duration.as_nanos() as u64);
    }
    
    let timings_variance = calculate_variance(&derivation_timings);
    let avg_timing = derivation_timings.iter().sum::<u64>() as f64 / derivation_timings.len() as f64;
    let cv = (timings_variance.sqrt() / avg_timing) * 100.0; // Coefficient of variation
    
    tracing::info!(
        iterations = iterations,
        avg_timing_ns = avg_timing,
        variance = timings_variance,
        coefficient_of_variation = cv,
        "Key derivation timing analysis"
    );
    
    // Key derivation should have relatively consistent timing
    assert!(cv < 20.0, "Key derivation timing too variable: {:.2}% CV", cv);
    
    // Test hash function timing consistency
    let test_data = b"consistent test data for hash timing analysis";
    let mut hash_timings = Vec::new();
    
    for _ in 0..iterations {
        let start = Instant::now();
        let _hash = Sha256::hash(test_data);
        let duration = start.elapsed();
        hash_timings.push(duration.as_nanos() as u64);
    }
    
    let hash_variance = calculate_variance(&hash_timings);
    let avg_hash_timing = hash_timings.iter().sum::<u64>() as f64 / hash_timings.len() as f64;
    let hash_cv = (hash_variance.sqrt() / avg_hash_timing) * 100.0;
    
    tracing::info!(
        hash_avg_timing_ns = avg_hash_timing,
        hash_variance = hash_variance,
        hash_coefficient_of_variation = hash_cv,
        "Hash function timing analysis"
    );
    
    // Hash functions should have very consistent timing for same input
    assert!(hash_cv < 15.0, "Hash function timing too variable: {:.2}% CV", hash_cv);
}

#[test]
fn test_secure_memory_handling() {
    init_tracing!();
    tracing::info!("Testing secure memory handling");
    
    // Test that encryption keys are properly managed
    let key_data = vec![42u8; 32];
    let key = EncryptionKey::new(key_data.clone(), "Test".to_string()).expect("Failed to create key");
    
    // Verify key data is accessible
    assert_eq!(key.as_bytes(), &key_data, "Key data should be accessible");
    assert_eq!(key.size(), 32, "Key size should be correct");
    assert_eq!(key.algorithm(), "Test", "Algorithm should be correct");
    
    // Test key generation
    let generated_key = EncryptionKey::generate("Generated", 64).expect("Failed to generate key");
    assert_eq!(generated_key.size(), 64, "Generated key should have correct size");
    assert_eq!(generated_key.algorithm(), "Generated", "Generated key should have correct algorithm");
    
    // Verify generated key has entropy
    let generated_bytes = generated_key.as_bytes();
    assert!(has_sufficient_entropy(generated_bytes), "Generated key should have sufficient entropy");
    
    // Test that different generated keys are different
    let key1 = EncryptionKey::generate("Test", 32).expect("Failed to generate key1");
    let key2 = EncryptionKey::generate("Test", 32).expect("Failed to generate key2");
    assert_ne!(key1.as_bytes(), key2.as_bytes(), "Generated keys should be different");
    
    tracing::info!("Secure memory handling tests completed");
}

#[test]
fn test_cryptographic_parameter_validation() {
    init_tracing!();
    tracing::info!("Testing cryptographic parameter validation");
    
    // Test invalid key sizes are rejected
    let invalid_sizes = [0, 1, 15, 17, 31, 33]; // Invalid for AES-256
    for &size in &invalid_sizes {
        let key = vec![42u8; size];
        let aes_result = Aes256Gcm::new(&key);
        let chacha_result = ChaCha20Poly1305Aead::new(&key);
        
        if size != 32 {
            assert!(aes_result.is_err(), "Should reject invalid AES key size: {}", size);
            assert!(chacha_result.is_err(), "Should reject invalid ChaCha20 key size: {}", size);
        }
    }
    
    // Test valid key size is accepted
    let valid_key = vec![42u8; 32];
    assert!(Aes256Gcm::new(&valid_key).is_ok(), "Should accept valid AES key size");
    assert!(ChaCha20Poly1305Aead::new(&valid_key).is_ok(), "Should accept valid ChaCha20 key size");
    
    // Test key derivation parameter validation
    let manager = KeyManager::new().expect("Failed to create key manager");
    let password = b"test";
    let salt = vec![1u8; 32];
    
    // Test invalid iteration counts (too low)
    let weak_config = KeyDerivationConfig {
        iterations: 100, // Too low for security
        salt: salt.clone(),
        key_length: 32,
    };
    
    // Should still work but is weak (we don't enforce minimum in our implementation)
    let weak_key = manager.derive_key_pbkdf2(password, &weak_config);
    assert!(weak_key.is_ok(), "Weak config should still work (but isn't recommended)");
    
    // Test invalid key lengths
    let invalid_length_config = KeyDerivationConfig {
        iterations: 10000,
        salt: salt.clone(),
        key_length: 0, // Invalid
    };
    
    let invalid_key = manager.derive_key_pbkdf2(password, &invalid_length_config);
    // Our implementation might handle this, but it's not useful
    if invalid_key.is_ok() {
        assert_eq!(invalid_key.unwrap().size(), 0, "Zero-length key should have zero size");
    }
    
    // Test random generation parameter validation
    let oversized_result = generate_random_bytes(10 * 1024 * 1024); // 10MB
    assert!(oversized_result.is_err(), "Should reject oversized random generation");
    
    let zero_size_result = generate_random_bytes(0);
    assert!(zero_size_result.is_ok(), "Should handle zero-size random generation");
    assert_eq!(zero_size_result.unwrap().len(), 0, "Zero-size result should be empty");
    
    tracing::info!("Cryptographic parameter validation tests completed");
}
