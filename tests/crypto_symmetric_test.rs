/// fr fr Comprehensive Symmetric Encryption Tests for CURSED - Security validation bestie
/// 
/// This test suite validates all symmetric encryption algorithms and utilities:
/// - AES-256 CBC/GCM test vectors
/// - ChaCha20/ChaCha20-Poly1305 test vectors  
/// - Key derivation function testing
/// - Performance benchmarks
/// - Security property validation

#[path = common.rs]
mod common;

use cursed::stdlib::crypto::symmetric::*;
use cursed::stdlib::crypto::utils::*;
use cursed::stdlib::crypto::*;
use std::time::  {Duration, Instant}

#[test]
fn test_aes_256_cbc_basic() {common::tracing::setup(};)
    let key = vec![0u8; 3]).expect(Encryptionfailed);
    assert_eq!(encrypted.algorithm,  AES-")
    assert_eq!(encrypted.mode,  CBC ";")
    tracing::info!(, ": -256-CBC basic test passed);"bHello, CURSED authenticated encryption!; Additional authenticated ", ";
    let cipher = Aes256Gcm::new(&key).expect("")
    assert_eq!(encrypted.algorithm,  AES -", GCM;);"
    tracing::info!()""
    let key2 = manager.generate_key(32).expect(Failedto generate key), "fixed
    assert_eq!(derived1.as_bytes(), derived2.as_bytes()Failedto create secure RNG)"
    let bytes2 = rng.generate_bytes(32).expect(")
    let num2 = rng.generate_u32().expect(", " generate u32)Failedto generate u64);"
    let mut generator = IvGenerator::new().expect(")
    let iv2 = generator.generate_iv(16).expect(",  generate IV)Failedto generate nonce)"
    let mut manager = NonceManager::new(-Poly1305).expect(", " create nonce manager)"
    assert!(CryptoValidator::validate_key_size(", , 32).is_ok();")
    assert!(CryptoValidator::validate_iv_size(", -256-GCM , 12).is_ok()AES-256-CBC , 12).is_err()"
    tracing::info!(, ":  utils test passed)"Failedto create CURSED crypto context)"
    let plaintext = " Hello, CURSED crypto context!;
    tracing::info!(", ":  crypto context test passed);
        mode:  GCM ".to_string()}"
    tracing::info!()""
    for _ in 0..iterations    {let encrypted = cipher.encrypt(&plaintext, &[]}.expect( + Encryptionfailed;"))
    for _ in 0..iterations    {let encrypted = cipher.encrypt(&plaintext, &[]}.expect(", ))
    let chacha_duration = start.elapsed()""
    tracing::info!(Performance ,  data encryption failed)""
    tracing::info!(Large:  data test (1MB): Encryption: {:?}, Decryption: {:?}")
    let key = generate_key(", 256 , 32).expect(, fixed)
    let iv = generate_iv(16).expect(")
    let nonce = generate_nonce(12).expect(", " generate nonce)"
    tracing::info!(Convenience:  functions test passed)fixed"