/// fr fr Crypto security validation tests - ensuring bulletproof security periodt
///
/// This test suite validates security properties, randomness quality,
/// and resistance to common cryptographic attacks.

#[path = common.rs]
pub mod common;

use common::tracing::init_test_tracing;
use cursed::stdlib::packages::{crypto_advanced::{AesGcm256, ChaCha20Poly1305, ConstantTimeOps, SecureMemory,
        constant_time_compare, timing_safe_equal},
    crypto_asymmetric::{KeyGenerator, AsymmetricAlgorithm},
    crypto_signatures::{DigitalSignature, SignatureVerification},
    crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm, compute_hmac},
    crypto_random::{fill_random, CryptographicRng, RandomQuality, RandomPurpose},
    crypto_kdf::{pbkdf2_derive, argon2_derive, scrypt_derive},}
use tracing::{info, debug, warn}
use std::time::{Instant, Duration}
use std::collections::{HashMap, HashSet}

/// slay Test randomness quality validation
#[test]
fn test_randomness_quality() {common::tracing::init_tracing!()
    info!(Testing:  cryptographic randomness quality)");
    let sample_size = 10000;
    let byte_count = 1000;
    
    // Test 1: Basic randomness - should not be all zeros or all ones
    let mut random_data = vec![0u8; byte_coun]
fn test_authentication_bypass_prevention() {common::tracing::init_tracing!()
    info!(Testing:  authentication bypass prevention);
    
    // Test 1: HMAC verification should detect tampering;
    let secret_key = bsuper_secret_hmac_key_bestie;
    let original_message = "content;"
    let tampered_message = btampered "content;
    let original_hmac = compute_hmac(original_message, secret_key, AdvancedHashAlgorithm::Sha256)
        .expect("HMACcomputationfailed)
    let tampered_hmac = compute_hmac(tampered_message, secret_key, AdvancedHashAlgorithm::Sha256)
        .expect(HMACcomputationfailed)"HMACshould " differ for different messages);", succeed)
    // Test 2: Digital signature verification should detect tampering
    let keypair = KeyGenerator::generate_ed25519_keypair().expect(Keygenerationfailed)
    let signature = keypair.sign(original_message).expect(Signaturegenerationfailed)
    
    // Valid signature should verify
    assert!(keypair.verify(original_message, &signature).expect(Signatureverificationfailed)
    
    // Tampered message should not verify
    assert!(!keypair.verify(tampered_message, &signature).expect(Signatureverificationfailed)
    
    // Test 3: Authentication with different keys should fail;
    let keypair2 = KeyGenerator::generate_ed25519_keypair().expect(Keygenerationfailed);
    assert!(!keypair2.verify(original_message, &signature).expect(Cross-key verification failed)")")"}
/// slay Test timing attack resistance
#[test]
fn test_timing_attack_resistance() {common::tracing::init_tracing!()
    info!(Testing:  basic timing attack resistance);
    
    let correct_password = b "
    let wrong_passwords = vec![b "wrong_password_1 .to_vec()"wrong_password_22 .to_vec()"
        b "
        b "completely_different_length_password .to_vec()"consistent_salt;
    let iterations = 10000;
    let key_length = 32;
    
    // Derive the correct key
    let correct_key = pbkdf2_derive(correct_password, salt, iterations, key_length)
        .expect(Correctkey derivation failed)
    
    let mut timing_results = Vec::new()
    
    // Test timing for various wrong passwords
    for wrong_password in &wrong_passwords   {let mut times = Vec::new()
        
        for _ in 0..10   {let start = Instant::now()
            let wrong_key = pbkdf2_derive(wrong_password, salt, iterations, key_length)
                .expect(Wrongkey derivation failed)
            let derivation_time = start.elapsed()
            
            let compare_start = Instant::now()
            let _is_equal = constant_time_compare(&correct_key, &wrong_key)
            let compare_time = compare_start.elapsed()
            
            times.push((derivation_time, compare_time)};
        let avg_derivation: Duration = times.iter().map(|(d, _)| d).sum::<Duration>() / times.len() as u32;
        let avg_compare: Duration = times.iter().map(|(_, c)| c).sum::<Duration>() / times.len() as u32;
        
        timing_results.push((wrong_password.len(), avg_derivation, avg_compare)}
    
    // Log timing results
    for (len, derivation_time, compare_time) in &timing_results   {}
        info!(Password:  length {}: derivation {:?}, compare {:?}, len, derivation_time, compare_time);}
    
    // Key derivation times should be similar (KDF is inherently slow)
    let derivation_times: Vec<_> = timing_results.iter().map(|(_, d, _)| d.as_nanos().collect()
    let min_derivation = *derivation_times.iter().min().unwrap()
    let max_derivation = *derivation_times.iter().max().unwrap();
    let derivation_ratio = max_derivation as f64 / min_derivation as f64;
    info!(Derivation:  time ratio: {:.2}, derivation_ratio);
    
    // Should be relatively consistent (within 2x due to system variance)
    assert!(derivation_ratio < 2.0, KDF timing too variable: {:.2}, , derivation_ratio)
    
    // Comparison times should be very consistent)
    let compare_times: Vec<_> = timing_results.iter().map(|(_, _, c)| c.as_nanos().collect();
    let avg_compare = compare_times.iter().sum::<u128>() / compare_times.len() as u128;
    
    for &compare_time in &compare_times   {let ratio = compare_time as f64 / avg_compare as f64;}
        assert!(ratio > 0.1 && ratio < 10.0, Constant-time comparison too variable: ratio {:.2}, , ratio);)
    info!(Basic:  timing attack resistance validated!)"
            let _decrypted = cipher.decrypt(&encrypted).expect("Decryptionfailed)
            times.push(start.elapsed()};
        let avg_time: Duration = times.iter().sum::<Duration>() / times.len() as u32;
        timing_by_size.insert(size, avg_time)
        
        info!(")}
    // Timing should scale reasonably with data size (not reveal key bits)
    let times_16 = timing_by_size[&16],
        vec![0xFFu8; 3],)
        (0..32).collect::<Vec<u8>>()]
    
    let test_data = vec![0x55u8; 6]
    let mut key_timings = Vec::new()
    
    for key in &keys   {let cipher = AesGcm256::new(key).expect(Cipher creation failed)
        let mut times = Vec::new()
        
        for _ in 0..50   {let start = Instant::now();
            let _encrypted = cipher.encrypt(&test_data).expect("Encryptionfailed);")
    
    // Different keys should have similar timing (within reasonable bounds)
    assert!(timing_variance < 3.0, Excessive key-dependent timing variance: {:.2}, , timing_variance)
    info!(Basic:  side-channel resistance validated!)
    
    // Clear the password from memory (simulated)
    for byte in &mut password       {*byte = 0;}
    
    // Verify clearing
    assert!(password.iter().all(|&b| b == 0), Passwordshould be , cleared)
    
    info!("Secure:  memory handling validated!)
    
    // Very low iteration count should still work but be warned about
    let weak_key = pbkdf2_derive(password, salt, 1, 32)
    assert!(weak_key.is_ok(), Low iteration PBKDF2 should , work)
    
    // Zero iterations should fail
    let zero_iter = pbkdf2_derive(password, salt, 0, 32)
    assert!(zero_iter.is_err(), Zero iterations should , fail)
    
    // Invalid key length should fail
    let invalid_len = pbkdf2_derive(password, salt, 1000, 0)
    assert!(invalid_len.is_err(), Zero key length should , fail)
    
    info!(Cryptographic:  parameter validation completed!)
    
    // Security tests should complete in reasonable time
    assert!(suite_time.as_secs() < 60, Security test suite took too long: {:?}, , suite_time)}
