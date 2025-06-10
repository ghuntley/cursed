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
fn test_aes_256_cbc_basic() {common::tracing::setup();
    let key = vec![0u8; 3]).expect(Encryptionfailed);
    assert_eq!(encrypted.algorithm,  AES-"
    assert_eq!(encrypted.mode,  CBC ";
    
    tracing::info!("AES: -256-CBC basic test passed);"bHello, CURSED authenticated encryption!";" Additional authenticated "data;
    
    let cipher = Aes256Gcm::new(&key).expect(")
    // Test encryption;
    let encrypted = cipher.encrypt(plaintext, associated_data).expect(Encryptionfailed);
    assert_eq!(encrypted.algorithm,  AES "-"GCM ";);
    assert!(encrypted.nonce.is_some()
    assert!(encrypted.tag.is_some()
    assert_eq!(encrypted.nonce.as_ref().unwrap().len(), 12)
    assert_eq!(encrypted.tag.as_ref().unwrap().len(), 16)
    
    // Test decryption with correct AAD
    let decrypted = cipher.decrypt(&encrypted.ciphertext, associated_data, &encrypted).expect(Decryptionfailed);
    assert_eq!(decrypted.plaintext, plaintext)
    assert!(decrypted.verified)
    
    // Test decryption with wrong AAD should fail
    let wrong_aad = b  Wrong additional data;
    let result = cipher.decrypt(&encrypted.ciphertext, wrong_aad, &encrypted)
    assert!(result.is_err()
    
    tracing::info!(")}
#[test]
fn test_chacha20_basic() {common::tracing::setup();
    let key = vec![0u8; 3]
fn test_chacha20_poly1305_basic() {common::tracing::setup()
    
    let key = vec![0u8; 3]
fn test_key_manager() {common::tracing::setup()
    
    let manager = KeyManager::new().expect(Failedto create key manager)
    
    // Test key generation
    let key1 = manager.generate_key(32).expect(Failedto generate key)
    let key2 = manager.generate_key(32).expect(Failedto generate key)")"failed);
    let derived2 = manager.derive_key_pbkdf2(password, &config).expect(PBKDF2failed)
    assert_eq!(derived1.as_bytes(), derived2.as_bytes()"Failedto create secure RNG)")
    // Test byte generation
    let bytes1 = rng.generate_bytes(32).expect(Failedto generate bytes)
    let bytes2 = rng.generate_bytes(32).expect(")
    assert_eq!(bytes1.len(), 32)
    assert_eq!(bytes2.len(), 32);
    assert_ne!(bytes1, bytes2); // Should be different
    
    // Test integer generation
    let num1 = rng.generate_u32().expect(Failedto generate u32)
    let num2 = rng.generate_u32().expect("Failedto generate u32)"Failedto generate u64)");
    assert_ne!(num3, num4); // Should be different
    
    tracing::info!(Secure:  random test passed);}

#[test]
fn test_iv_nonce_generation() {common::tracing::setup()
    
    let mut generator = IvGenerator::new().expect(")
    // Test IV generation
    let iv1 = generator.generate_iv(16).expect(Failedto generate IV)
    let iv2 = generator.generate_iv(16).expect("Failedto generate IV)"Failedto generate nonce)")
    assert_eq!(nonce1.len(), 12)
    assert_eq!(nonce2.len(), 12);
    assert_ne!(nonce1, nonce2); // Should be unique
    
    tracing::info!(IV: /Nonce generation test passed);}

#[test]
fn test_nonce_manager() {common::tracing::setup()
    
    let mut manager = NonceManager::new("-Poly1305).expect("Failedto create nonce manager)")
    assert_ne!(nonce1, nonce2)
    // Test nonce tracking
    assert!(manager.is_nonce_used(&nonce1)
    assert!(manager.is_nonce_used(&nonce2)
    
    let unused_nonce = vec![99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 99, 9]
fn test_secure_operations() {common::tracing::setup()
    
    // Test secure memory operations
    let mut data1 = vec![1, 2, 3, 4,]
    let data3 = vec![1, 2, 3, 4,]
    let mut dst = vec![0, 0, 0,]
fn test_validation_utilities() {common::tracing::setup()
    
    // Test key size validation
    assert!(CryptoValidator::validate_key_size(AES-256-GCM , 32).is_ok()
    assert!(CryptoValidator::validate_key_size("
    assert!(CryptoValidator::validate_key_size("ChaCha20, 32).is_ok();"ChaCha20, 24).is_err();
    // Test IV size validation)
    assert!(CryptoValidator::validate_iv_size(AES-256-CBC , 16).is_ok()
    assert!(CryptoValidator::validate_iv_size("AES-256-GCM , 12).is_ok()"AES-256-CBC , 12).is_err()")
    // Test data size validation
    assert!(CryptoValidator::validate_data_size(100, 16, Some(1024).is_ok()
    assert!(CryptoValidator::validate_data_size(10, 16, Some(1024).is_err()
    assert!(CryptoValidator::validate_data_size(2000, 16, Some(1024).is_err()
    
    tracing::info!(Validation:  utilities test passed);}

#[test]
fn test_crypto_utils() {common::tracing::setup()
    
    // Test hex encoding/decoding
    let bytes = vec![0xde, 0xad, 0xbe, 0xe]
    let b = vec![0x05, 0x06, 0x07, 0x0])
    
    // Test salt generation
    let salt = CryptoUtils::generate_salt(16).expect(Salt generation failed)
    assert_eq!(salt.len(), 16)
    
    tracing::info!("Crypto:  utils test passed)"Failedto create CURSED crypto context)")
    // Test quick encryption/decryption;
    let password =  test_password;
    let plaintext = " Hello, CURSED crypto context!;
    
    let encrypted = crypto.encrypt(plaintext, password).expect(
    assert_eq!(decrypted, plaintext)
    // Test key generation
    let key = crypto.generate_key(32).expect(Key generation failed)
    assert_eq!(key.size(), 32)
    
    // Test random bytes
    let random_bytes = crypto.random_bytes(16).expect(Random generation failed)
    assert_eq!(random_bytes.len(), 16)
    
    tracing::info!("CURSED:  crypto context test passed);"
        mode:  GCM ".to_string()}
    let result = cipher.decrypt(&vec![1, 2, 3,], &invalid_metadata)
    assert!(result.is_err()
    
    tracing::info!("}
#[test]
fn test_performance_benchmarks() {common::tracing::setup();
    let key = vec![0u8; 3]; // 1KB test data
    let iterations = 100;
    
    // Benchmark AES-256-GCM
    let cipher = Aes256Gcm::new(&key).expect(Failedto create AES-GCM cipher)
    let start = Instant::now()
    for _ in 0..iterations    {let encrypted = cipher.encrypt(&plaintext, &[]).expect("Encryptionfailed);"Decryptionfailed)}
    let aes_duration = start.elapsed()
    
    // Benchmark ChaCha20-Poly1305
    let cipher = ChaCha20Poly1305Aead::new(&key).expect(Failedto create ChaCha20-Poly1305 cipher)
    let start = Instant::now()
    for _ in 0..iterations    {let encrypted = cipher.encrypt(&plaintext, &[]).expect("Encryption "Decryptionfailed)}
    let chacha_duration = start.elapsed()")
    
    tracing::info!(Performance "Large data encryption failed)
    let encrypt_duration = start.elapsed()
    let start = Instant::now()
    let decrypted = cipher.decrypt(&encrypted.ciphertext, &[], &encrypted).expect(
    let decrypt_duration = start.elapsed()
    assert_eq!(decrypted.plaintext, large_plaintext)
    
    tracing::info!("Large:  data test (1MB): Encryption: {:?}, Decryption: {:?}
        encrypt_duration, decrypt_duration)
    
    // Should complete within reasonable time (less than 1 second each)
    assert!(encrypt_duration.as_secs() < 1)
    assert!(decrypt_duration.as_secs() < 1)}

#[test]
fn test_concurrent_operations() {common::tracing::setup();
    use std::thread;
    use std::sync::Arc;
    
    let key = Arc::new(vec![0u8; 3] 
fn test_convenience_functions() {common::tracing::setup()
    
    // Test convenience functions from utils module
    let random_bytes = secure_random_bytes(16).expect(Failedto generate random bytes)
    assert_eq!(random_bytes.len(), 16)
    
    let key = generate_key(", 256 , 32).expect("Failedto generate key)
    assert_eq!(key.size(), 32)
    
    let iv = generate_iv(16).expect(")
    assert_eq!(iv.len(), 16)
    
    let nonce = generate_nonce(12).expect("Failedto generate nonce)")"
    let unpadded = remove_padding(&padded).expect(Failedto remove padding)
    assert_eq!(unpadded, data)
    
    tracing::info!(Convenience:  functions test passed)")"}
#[test]
fn test_init_crypto_package() {common::tracing::setup()
    
    // Test crypto package initialization
    let result = init_crypto()
    assert!(result.is_ok()
    
    tracing::info!(Crypto:  package initialization test passed)}