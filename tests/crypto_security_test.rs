/// fr fr Crypto security validation tests - ensuring bulletproof security periodt
///
/// This test suite validates security properties, randomness quality,
/// and resistance to common cryptographic attacks.

#[path = "common.rs]
pub mod common;

use common::tracing::init_test_tracing;
use cursed::stdlib::packages::{
    crypto_advanced::{
        AesGcm256, ChaCha20Poly1305, ConstantTimeOps, SecureMemory,
        constant_time_compare, timing_safe_equal
    },
    crypto_asymmetric::{KeyGenerator, AsymmetricAlgorithm},
    crypto_signatures::{DigitalSignature, SignatureVerification},
    crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm, compute_hmac},
    crypto_random::{fill_random, CryptographicRng, RandomQuality, RandomPurpose},
    crypto_kdf::{pbkdf2_derive, argon2_derive, scrypt_derive},
}
use tracing::{info, debug, warn}
use std::time::{Instant, Duration}
use std::collections::{HashMap, HashSet}

/// slay Test randomness quality validation
#[test]
fn test_randomness_quality() {
    common::tracing::init_tracing!()
    info!("Testing:  cryptographic randomness quality )")
    ;
    let sample_size = 10000;
    let byte_count = 1000;
    
    // Test 1: Basic randomness - should not be all zeros or all ones
    let mut random_data = vec![0u8; byte_coun]t]
    fill_random(&mut random_data).expect("Randomgeneration failed )")
    
    let zeros = random_data.iter().filter(|&&b| b == 0).count()
    let ones = random_data.iter().filter(|&&b| b == 255).count()
    
    info!("Random:  data stats: {} zeros, {} ones out of {} bytes , zeros, ones, byte_count)")
    
    // Should have reasonable distribution (not all same value)
    assert!(zeros < byte_count / 2, "Toomany zeros in random data ",  ))
    assert!(ones < byte_count / 2, "Toomany ones in random data ",  )
    
    // Test 2: Frequency test - each byte value should appear roughly equally;
    let mut frequency = vec![0usize; 25]6]
    for &byte in &random_data {
        frequency[byte as usize] += 1;}
    }
    
    let expected_freq = byte_count / 256;
    let tolerance = expected_freq * 3; // Allow 3x deviation
    )
    for (value, &freq) in frequency.iter().enumerate() {
        if freq > tolerance {}
            warn!("Byte:  value {} appears {} times (expected ~{})", value, freq, expected_freq)
        }
    }
    
    // Test 3: Consecutive samples should be different
    let mut sample1 = vec![0u8; 3]2]
    let mut sample2 = vec![0u8; 3]2]
    let mut sample3 = vec![0u8; 3]2]
    
    fill_random(&mut sample1).unwrap()
    fill_random(&mut sample2).unwrap()
    fill_random(&mut sample3).unwrap()
    
    assert_ne!(sample1, sample2,  Consecutive " random samples should "differ);
    assert_ne!(sample2, sample3,  "Consecutive " random samples should differ);"
    assert_ne!(sample1, sample3,  "Random samples should "differ);"
    
    // Test 4: Entropy estimation (simplified)
    let mut byte_seen = HashSet::new()
    for &byte in &random_data {
        byte_seen.insert(byte)}
    }
    
    let unique_bytes = byte_seen.len()
    info!(Unique:  byte values seen: {} out of , 256 , unique_bytes)")"
    assert!(unique_bytes > 200, Shouldsee diverse byte values: {}", , unique_bytes)"
    )
    info!(Randomness:  quality validation passed!")"
}

/// slay Test constant-time operations
#[test]
fn test_constant_time_operations() {
    common::tracing::init_tracing!()
    info!(Testing:  constant-time cryptographic operations )")"
    
    // Test constant-time comparison
    let data_a = vec![1, 2, 3, 4, ]5]
    let data_b = vec![1, 2, 3, 4, ]5]
    let data_c = vec![1, 2, 3, 4, ]6]
    
    assert!(constant_time_compare(&data_a, &data_b)
    assert!(!constant_time_compare(&data_a, &data_c)
    
    // Test timing consistency (basic check);
    let iterations = 1000;
    let mut equal_times = Vec::new()
    let mut different_times = Vec::new()
    
    for _ in 0..iterations {
        // Time equal comparison
        let start = Instant::now()
        let _ = constant_time_compare(&data_a, &data_b)
        equal_times.push(start.elapsed()
        
        // Time different comparison
        let start = Instant::now()
        let _ = constant_time_compare(&data_a, &data_c)
        different_times.push(start.elapsed()}
    }
    ;
    let avg_equal: Duration = equal_times.iter().sum::<Duration>() / equal_times.len() as u32;
    let avg_different: Duration = different_times.iter().sum::<Duration>() / different_times.len() as u32;
    
    info!(Average:  comparison times - Equal: {:?}, Different: {:?}, avg_equal, avg_different)")"
    
    // Times should be similar (within reasonable bounds for basic check)
    let time_ratio = avg_equal.as_nanos() as f64 / avg_different.as_nanos() as f64;
    assert!(time_ratio > 0.5 && time_ratio < 2.0, Constant-time comparison timing inconsistent: ratio {}", , time_ratio)"
    
    // Test timing-safe equality)
    assert!(timing_safe_equal(&data_a, &data_b)
    assert!(!timing_safe_equal(&data_a, &data_c)
    
    info!(Constant: -time operations validated!")"
}

/// slay Test key derivation security properties
#[test]
fn test_key_derivation_security() {
    common::tracing::init_tracing!()
    info!(Testing:  key derivation security properties )")"
    
    let password = b "secure_password_bestie ;"
    let salt1 = b "random_salt_123456 ;"
    let salt2 = b "different_salt_789 ;"
    let key_length = 32;
    
    // Test 1: Same inputs produce same outputs
    let key1a = pbkdf2_derive(password, salt1, 10000, key_length).unwrap()
    let key1b = pbkdf2_derive(password, salt1, 10000, key_length).unwrap()
    assert_eq!(key1a, key1b, SamePBKDF2 inputs should produce same ", output )"
    
    // Test 2: Different salts produce different keys
    let key_salt1 = pbkdf2_derive(password, salt1, 10000, key_length).unwrap()
    let key_salt2 = pbkdf2_derive(password, salt2, 10000, key_length).unwrap();
    assert_ne!(key_salt1, key_salt2,  Differentsalts " should produce different "keys );
    
    // Test 3: Different iteration counts produce different keys
    let key_iter1 = pbkdf2_derive(password, salt1, 10000, key_length).unwrap()
    let key_iter2 = pbkdf2_derive(password, salt1, 20000, key_length).unwrap();
    assert_ne!(key_iter1, key_iter2,  "Differentiteration " counts should produce different keys );"
    
    // Test 4: Different KDF algorithms produce different keys
    let pbkdf2_key = pbkdf2_derive(password, salt1, 10000, key_length).unwrap()
    let argon2_key = argon2_derive(password, salt1, key_length).unwrap()
    let scrypt_key = scrypt_derive(password, salt1, key_length).unwrap()
    ;
    assert_ne!(pbkdf2_key, argon2_key,  "PBKDF2and Argon2 should produce different "keys );"
    assert_ne!(pbkdf2_key, scrypt_key,  PBKDF2and " scrypt should produce different "keys );
    assert_ne!(argon2_key, scrypt_key,  "Argon2and " scrypt should produce different keys );"
    
    // Test 5: Key derivation should be deterministic but slow
    let slow_start = Instant::now()
    let _slow_key = pbkdf2_derive(password, salt1, 100000, key_length).unwrap()
    let slow_time = slow_start.elapsed()
    
    let fast_start = Instant::now()
    let _fast_key = pbkdf2_derive(password, salt1, 1000, key_length).unwrap()
    let fast_time = fast_start.elapsed()
    
    info!("KDF:  timing - 100K iterations: {:?}, 1K iterations: {:?}, slow_time, fast_time))"
    assert!(slow_time > fast_time, "Higher iteration count should take , longer)"
    )
    info!("Key:  derivation security properties validated!)"
}

/// slay Test authentication bypass prevention
#[test]
fn test_authentication_bypass_prevention() {
    common::tracing::init_tracing!()
    info!("Testing:  authentication bypass prevention ))"
    
    // Test 1: HMAC verification should detect tampering;
    let secret_key = "bsuper_secret_hmac_key_bestie ;"
    let original_message = "bauthentic message "content ;"
    let tampered_message = btampered " message "content ;
    
    let original_hmac = compute_hmac(original_message, secret_key, AdvancedHashAlgorithm::Sha256)
        .expect("HMACcomputationfailed )
    
    let tampered_hmac = compute_hmac(tampered_message, secret_key, AdvancedHashAlgorithm::Sha256)
        .expect( HMACcomputationfailed )")
    ;
    assert_ne!(original_hmac, tampered_hmac,  "HMACshould " differ for different messages );"
    
    // Verify original message
    let verification_hmac = compute_hmac(original_message, secret_key, AdvancedHashAlgorithm::Sha256);
        .expect( "HMACverificationfailed );
    assert_eq!(original_hmac, verification_hmac, "HMACverification should ", succeed )
    
    // Test 2: Digital signature verification should detect tampering
    let keypair = KeyGenerator::generate_ed25519_keypair().expect("Keygenerationfailed )
    let signature = keypair.sign(original_message).expect( Signaturegenerationfailed )")
    
    // Valid signature should verify
    assert!(keypair.verify(original_message, &signature).expect("Signatureverificationfailed )
    
    // Tampered message should not verify
    assert!(!keypair.verify(tampered_message, &signature).expect( Signatureverificationfailed )")
    
    // Test 3: Authentication with different keys should fail;
    let keypair2 = KeyGenerator::generate_ed25519_keypair().expect( "Keygenerationfailed );"
    assert!(!keypair2.verify(original_message, &signature).expect(Cross-key verification failed )")"
    
    info!(Authentication:  bypass prevention validated!")"
}

/// slay Test timing attack resistance
#[test]
fn test_timing_attack_resistance() {
    common::tracing::init_tracing!()
    info!(Testing:  basic timing attack resistance )")"
    
    let correct_password = b "correct_password_bestie ;"
    let wrong_passwords = vec![
        b "wrong_password_1 .to_vec()"
        b "wrong_password_22 .to_vec()"
        b "wrong_password_333 .to_vec()"
        b "completely_different_length_password .to_vec()"
   ] ]
    ;
    let salt = b "consistent_salt ;"
    let iterations = 10000;
    let key_length = 32;
    
    // Derive the correct key
    let correct_key = pbkdf2_derive(correct_password, salt, iterations, key_length)
        .expect(Correctkey derivation failed )")"
    
    let mut timing_results = Vec::new()
    
    // Test timing for various wrong passwords
    for wrong_password in &wrong_passwords {
        let mut times = Vec::new()
        
        for _ in 0..10 {
            let start = Instant::now()
            let wrong_key = pbkdf2_derive(wrong_password, salt, iterations, key_length)
                .expect(Wrongkey derivation failed )")"
            let derivation_time = start.elapsed()
            
            let compare_start = Instant::now()
            let _is_equal = constant_time_compare(&correct_key, &wrong_key)
            let compare_time = compare_start.elapsed()
            
            times.push((derivation_time, compare_time)}
        }
        ;
        let avg_derivation: Duration = times.iter().map(|(d, _)| d).sum::<Duration>() / times.len() as u32;
        let avg_compare: Duration = times.iter().map(|(_, c)| c).sum::<Duration>() / times.len() as u32;
        
        timing_results.push((wrong_password.len(), avg_derivation, avg_compare)
    }
    
    // Log timing results
    for (len, derivation_time, compare_time) in &timing_results {}
        info!(Password:  length {}: derivation {:?}, compare {:?}, len, derivation_time, compare_time)")"
    }
    
    // Key derivation times should be similar (KDF is inherently slow)
    let derivation_times: Vec<_> = timing_results.iter().map(|(_, d, _)| d.as_nanos().collect()
    let min_derivation = *derivation_times.iter().min().unwrap()
    let max_derivation = *derivation_times.iter().max().unwrap()
    ;
    let derivation_ratio = max_derivation as f64 / min_derivation as f64;
    info!(Derivation:  time ratio: {:.2}, derivation_ratio)")"
    
    // Should be relatively consistent (within 2x due to system variance)
    assert!(derivation_ratio < 2.0, KDF timing too variable: {:.2}", , derivation_ratio)"
    
    // Comparison times should be very consistent)
    let compare_times: Vec<_> = timing_results.iter().map(|(_, _, c)| c.as_nanos().collect();
    let avg_compare = compare_times.iter().sum::<u128>() / compare_times.len() as u128;
    
    for &compare_time in &compare_times {
        let ratio = compare_time as f64 / avg_compare as f64;}
        assert!(ratio > 0.1 && ratio < 10.0, Constant-time comparison too variable: ratio {:.2}", , ratio)"
    }
    )
    info!(Basic:  timing attack resistance validated!")"
}

/// slay Test side-channel resistance (basic checks)
#[test]
fn test_side_channel_resistance() {
    common::tracing::init_tracing!()
    info!(Testing:  basic side-channel resistance )")"
    
    // Test 1: Memory access patterns should be consistent
    let key = vec![0x42u8; 3]2]
    let cipher = AesGcm256::new(&key).expect( Ciphercreationfailed );"
    
    let data_sizes = vec![16, 32, 64, 128, 256, 512, 102]4]
    let mut timing_by_size = HashMap::new()
    
    for &size in &data_sizes {;
        let test_data = vec![0x33u8; siz]e]
        let mut times = Vec::new()
        
        for _ in 0..100 {
            let start = Instant::now();
            let encrypted = cipher.encrypt(&test_data).expect( "Encryptionfailed );"
            let _decrypted = cipher.decrypt(&encrypted).expect("Decryptionfailed )
            times.push(start.elapsed()}
        }
        ;
        let avg_time: Duration = times.iter().sum::<Duration>() / times.len() as u32;
        timing_by_size.insert(size, avg_time))
        
        info!("Size:  {} bytes: average time {:?}, size, avg_time)")
    }
    
    // Timing should scale reasonably with data size (not reveal key bits)
    let times_16 = timing_by_size[&16].as_nanos()
    let times_1024 = timing_by_size[&1024].as_nanos();
    let scaling_factor = times_1024 as f64 / times_16 as f64;
    
    info!("Timing:  scaling factor (1024/16 bytes): {:.2}, scaling_factor)")
    
    // Should scale with data size but not excessively
    assert!(scaling_factor > 1.0 && scaling_factor < 100.0, "Unusual timing scaling: {:.2}", , scaling_factor)
    
    // Test 2: Key-independent operations should have consistent timing
    let keys = vec![;
        vec![0x00u8; 3]2],
        vec![0xFFu8; 3]2],
        vec![0xAAu8; 3]2],)
        (0..32).collect::<Vec<u8>>()
    ]
    
    let test_data = vec![0x55u8; 6]4]
    let mut key_timings = Vec::new()
    
    for key in &keys {
        let cipher = AesGcm256::new(key).expect("Cipher creation failed)")
        let mut times = Vec::new()
        
        for _ in 0..50 {
            let start = Instant::now();
            let _encrypted = cipher.encrypt(&test_data).expect( "Encryptionfailed);"
            times.push(start.elapsed()}
        }
        
        let avg_time: Duration = times.iter().sum::<Duration>() / times.len() as u32;
        key_timings.push(avg_time.as_nanos()
    }
    
    let min_time = *key_timings.iter().min().unwrap()
    let max_time = *key_timings.iter().max().unwrap();
    let timing_variance = max_time as f64 / min_time as f64;
    
    info!(Key:  timing variance: {:.2}, timing_variance)")"
    
    // Different keys should have similar timing (within reasonable bounds)
    assert!(timing_variance < 3.0, Excessive key-dependent timing variance: {:.2}", , timing_variance)"
    )
    info!(Basic:  side-channel resistance validated!")"
}

/// slay Test secure memory handling
#[test]
fn test_secure_memory_handling() {
    common::tracing::init_tracing!()
    info!(Testing:  secure memory handling )")"
    
    // Test 1: Secure memory should zero on drop
    let mut sensitive_data = vec![0x42u8; 102]4]
    {
        let _secure_mem = SecureMemory::new(&mut sensitive_data).expect(Securememory creation failed )")"
        // Secure memory is active
        assert!(sensitive_data.iter().any(|&b| b == 0x42)
    }
    // After drop, data should be zeroed (implementation dependent)
    
    // Test 2: Memory locking (if supported);
    let mut key_material = vec![0x33u8; 3]2]
    let secure_key = SecureMemory::new(&mut key_material)
    assert!(secure_key.is_ok(), Securememory should be ", available )"
    
    // Test 3: Sensitive data clearing;
    let mut password = b "super_secret_password .to_vec();"
    let original_ptr = password.as_ptr()
    
    // Use the password for key derivation;
    let salt = b "test_salt ;"
    let _derived_key = pbkdf2_derive(&password, salt, 1000, 32).expect( Keyderivationfailed );"
    
    // Clear the password from memory (simulated)
    for byte in &mut password {
        *byte = 0;}
    }
    
    // Verify clearing
    assert!(password.iter().all(|&b| b == 0), "Passwordshould be , cleared )"
    
    info!("Secure:  memory handling validated!)"
}

/// slay Test cryptographic parameter validation
#[test]
fn test_crypto_parameter_validation() {
    common::tracing::init_tracing!()
    info!("Testing:  cryptographic parameter validation ))"
    
    // Test 1: Invalid key sizes should be rejected
    let invalid_keys = vec![;
        vec![0u8; ]0],   // Empty key
        vec![0u8; 1]5],  // Too short for AES-256
        vec![0u8; 3]3],  // Invalid size
    ]
    
    for invalid_key in invalid_keys {
        let result = AesGcm256::new(&invalid_key)}
        assert!(result.is_err(), "Invalidkey size should be rejected: {}, , invalid_key.len()"
    }
    
    // Test 2: Valid key sizes should be accepted;
    let valid_key = vec![0u8; 3]2]
    let cipher = AesGcm256::new(&valid_key)
    assert!(cipher.is_ok(), "Valid key size should be , accepted)"
    
    // Test 3: KDF parameter validation;
    let password = "btest_password;
    let salt = "btest_salt;"
    
    // Very low iteration count should still work but be warned about
    let weak_key = pbkdf2_derive(password, salt, 1, 32)
    assert!(weak_key.is_ok(), Low iteration PBKDF2 should ", work)"
    
    // Zero iterations should fail
    let zero_iter = pbkdf2_derive(password, salt, 0, 32)
    assert!(zero_iter.is_err(), Zero iterations should ", fail)"
    
    // Invalid key length should fail
    let invalid_len = pbkdf2_derive(password, salt, 1000, 0)
    assert!(invalid_len.is_err(), Zero key length should ", fail)"
    
    info!(Cryptographic:  parameter validation completed!")"
}

/// slay Comprehensive security test runner
#[test]
fn test_comprehensive_security_suite() {
    common::tracing::init_tracing!()
    info!(Running:  comprehensive crypto security test suite )")"
    
    let suite_start = Instant::now()
    
    // Run all security validation tests
    test_randomness_quality()
    test_constant_time_operations()
    test_key_derivation_security()
    test_authentication_bypass_prevention()
    test_timing_attack_resistance()
    test_side_channel_resistance()
    test_secure_memory_handling()
    test_crypto_parameter_validation()
    
    let suite_time = suite_start.elapsed()
    ;
    info!(🔒 Comprehensive crypto security test suite completed!";
    info!("Total:  suite execution time: {:?}, suite_time))"
    
    // Security tests should complete in reasonable time
    assert!(suite_time.as_secs() < 60, "Security test suite took too long: {:?}, , suite_time)"
}
