/// fr fr Comprehensive test suite for foundational cryptographic packages
/// 
/// This test validates the core cryptographic implementations including
/// RSA, ECC, SHA-3, BLAKE3, HMAC, and PBKDF2.

use cursed::stdlib::packages::crypto_asymmetric::{
    rsa::{RsaEngine, RSA_2048_BITS, RsaPadding},
    ecc::{EccEngine, EcCurve},
};
use cursed::stdlib::packages::crypto_hash_advanced::{
    sha3::{Sha3Hasher, Sha3Variant, Sha3Utils},
    blake3::{Blake3Hasher, Blake3Utils},
    hmac::{HmacEngine, HmacAlgorithm, HmacUtils},
};
use cursed::stdlib::packages::crypto_kdf::{
    pbkdf2::{Pbkdf2Engine, Pbkdf2Config, Pbkdf2Utils},
};

#[test]
fn test_rsa_key_generation() {
    let mut engine = RsaEngine::new();
    let result = engine.generate_keypair(RSA_2048_BITS);
    
    assert!(result.is_ok(), "RSA key generation should succeed");
    
    let keypair = result.unwrap();
    assert_eq!(keypair.key_size, RSA_2048_BITS);
    assert!(!keypair.public_key.n.digits.iter().all(|&d| d == 0));
    assert!(!keypair.private_key.d.digits.iter().all(|&d| d == 0));
}

#[test]
fn test_rsa_encryption_structure() {
    let mut engine = RsaEngine::new();
    let keypair = engine.generate_keypair(RSA_2048_BITS).unwrap();
    
    let plaintext = b"Hello, RSA encryption!";
    let encrypted = engine.encrypt(&keypair.public_key, plaintext, RsaPadding::Pkcs1v15);
    
    assert!(encrypted.is_ok(), "RSA encryption should not fail structurally");
}

#[test]
fn test_rsa_signing_structure() {
    let mut engine = RsaEngine::new();
    let keypair = engine.generate_keypair(RSA_2048_BITS).unwrap();
    
    let message = b"Hello, RSA signatures!";
    let signature = engine.sign(&keypair.private_key, message, RsaPadding::Pkcs1v15);
    
    assert!(signature.is_ok(), "RSA signing should not fail structurally");
}

#[test]
fn test_ecdsa_key_generation() {
    let mut engine = EccEngine::new();
    let result = engine.generate_ecdsa_keypair(EcCurve::P256);
    
    assert!(result.is_ok(), "ECDSA key generation should succeed");
    
    let keypair = result.unwrap();
    assert_eq!(keypair.curve, EcCurve::P256);
    assert!(!keypair.public_key.point.is_infinity);
    assert!(!keypair.private_key.scalar.bytes.iter().all(|&b| b == 0));
}

#[test]
fn test_ecdsa_signing_structure() {
    let mut engine = EccEngine::new();
    let keypair = engine.generate_ecdsa_keypair(EcCurve::P256).unwrap();
    
    let message = b"Hello, ECDSA!";
    let signature = engine.ecdsa_sign(&keypair.private_key, message);
    
    assert!(signature.is_ok(), "ECDSA signing should not fail structurally");
    
    let sig = signature.unwrap();
    assert_eq!(sig.curve, EcCurve::P256);
    assert!(!sig.r.iter().all(|&b| b == 0));
    assert!(!sig.s.iter().all(|&b| b == 0));
}

#[test]
fn test_ecdh_key_exchange_structure() {
    let mut engine = EccEngine::new();
    let alice_keypair = engine.generate_ecdh_keypair(EcCurve::P256).unwrap();
    let bob_keypair = engine.generate_ecdh_keypair(EcCurve::P256).unwrap();
    
    let alice_shared = engine.ecdh_exchange(&alice_keypair.private_key, &bob_keypair.public_key);
    let bob_shared = engine.ecdh_exchange(&bob_keypair.private_key, &alice_keypair.public_key);
    
    assert!(alice_shared.is_ok(), "Alice's ECDH should succeed");
    assert!(bob_shared.is_ok(), "Bob's ECDH should succeed");
}

#[test]
fn test_ec_curve_properties() {
    assert_eq!(EcCurve::P256.name(), "P-256");
    assert_eq!(EcCurve::P256.key_size(), 32);
    assert_eq!(EcCurve::P256.security_level(), 128);
    
    assert_eq!(EcCurve::P384.name(), "P-384");
    assert_eq!(EcCurve::P384.key_size(), 48);
    assert_eq!(EcCurve::P384.security_level(), 192);
    
    assert_eq!(EcCurve::P521.name(), "P-521");
    assert_eq!(EcCurve::P521.key_size(), 66);
    assert_eq!(EcCurve::P521.security_level(), 256);
}

#[test]
fn test_sha3_basic_functionality() {
    // Test SHA3-256
    let hash = Sha3Hasher::hash(Sha3Variant::Sha3_256, b"test");
    assert_eq!(hash.len(), 32);
    assert_ne!(hash, vec![0u8; 32]); // Should not be all zeros
    
    // Test SHA3-512
    let hash = Sha3Hasher::hash(Sha3Variant::Sha3_512, b"test");
    assert_eq!(hash.len(), 64);
    
    // Test SHAKE128 with custom length
    let hash = Sha3Hasher::shake(Sha3Variant::Shake128, b"test", 16);
    assert_eq!(hash.len(), 16);
    
    // Test SHAKE256 with custom length
    let hash = Sha3Hasher::shake(Sha3Variant::Shake256, b"test", 100);
    assert_eq!(hash.len(), 100);
}

#[test]
fn test_sha3_streaming() {
    let mut hasher = Sha3Hasher::new(Sha3Variant::Sha3_256);
    hasher.update(b"hello");
    hasher.update(b" ");
    hasher.update(b"world");
    let hash1 = hasher.finalize();
    
    let hash2 = Sha3Hasher::hash(Sha3Variant::Sha3_256, b"hello world");
    assert_eq!(hash1, hash2, "Streaming and one-shot hashing should produce same result");
}

#[test]
fn test_sha3_variants() {
    assert_eq!(Sha3Variant::Sha3_256.name(), "SHA3-256");
    assert_eq!(Sha3Variant::Sha3_256.output_size(), Some(32));
    assert_eq!(Sha3Variant::Shake128.output_size(), None); // Extendable
    
    // Test rate and capacity
    assert_eq!(Sha3Variant::Sha3_256.rate(), 136);
    assert_eq!(Sha3Variant::Sha3_256.capacity(), 64);
}

#[test]
fn test_blake3_basic_functionality() {
    // Test basic hashing
    let hash = Blake3Hasher::hash(b"test");
    assert_eq!(hash.len(), 32);
    assert_ne!(hash, [0u8; 32]); // Should not be all zeros
    
    // Test streaming
    let mut hasher = Blake3Hasher::new();
    hasher.update(b"hello");
    hasher.update(b" ");
    hasher.update(b"world");
    let hash1 = hasher.finalize_fixed();
    
    let hash2 = Blake3Hasher::hash(b"hello world");
    assert_eq!(hash1, hash2, "Streaming and one-shot should produce same result");
}

#[test]
fn test_blake3_keyed_hashing() {
    let key = [1u8; 32];
    let data = b"test data";
    
    let hash1 = Blake3Hasher::keyed_hash(&key, data);
    let hash2 = Blake3Hasher::keyed_hash(&key, data);
    assert_eq!(hash1, hash2, "Keyed hashing should be deterministic");
    
    // Different key should produce different hash
    let key2 = [2u8; 32];
    let hash3 = Blake3Hasher::keyed_hash(&key2, data);
    assert_ne!(hash1, hash3, "Different keys should produce different hashes");
}

#[test]
fn test_blake3_key_derivation() {
    let context = "test context";
    let key_material = b"secret key material";
    
    let derived1 = Blake3Hasher::derive_key(context, key_material, 32);
    let derived2 = Blake3Hasher::derive_key(context, key_material, 32);
    assert_eq!(derived1, derived2, "Key derivation should be deterministic");
    
    // Different context should produce different key
    let derived3 = Blake3Hasher::derive_key("different context", key_material, 32);
    assert_ne!(derived1, derived3, "Different contexts should produce different keys");
    
    // Different length should work
    let derived4 = Blake3Hasher::derive_key(context, key_material, 64);
    assert_eq!(derived4.len(), 64);
}

#[test]
fn test_blake3_variable_output() {
    let data = b"test";
    let mut hasher = Blake3Hasher::new();
    hasher.update(data);
    
    let mut output1 = [0u8; 16];
    hasher.clone().finalize_variable(&mut output1);
    assert_eq!(output1.len(), 16);
    
    let mut output2 = [0u8; 100];
    hasher.finalize_variable(&mut output2);
    assert_eq!(output2.len(), 100);
    
    // First 16 bytes should match
    assert_eq!(output1, output2[..16]);
}

#[test]
fn test_hmac_sha256_basic() {
    let key = b"test_key";
    let message = b"test_message";
    
    let mac = HmacUtils::hmac_sha256(key, message).unwrap();
    assert_eq!(mac.len(), 32);
    assert_ne!(mac, vec![0u8; 32]); // Should not be all zeros
}

#[test]
fn test_hmac_verification() {
    let key = b"secret_key";
    let message = b"authenticated_message";
    
    let engine = HmacEngine::new(HmacAlgorithm::Sha256, key).unwrap();
    let mac = engine.compute(message);
    
    // Should verify correctly
    assert!(engine.verify(message, &mac), "HMAC should verify correctly");
    
    // Should fail with wrong message
    assert!(!engine.verify(b"wrong_message", &mac), "HMAC should fail with wrong message");
    
    // Should fail with wrong MAC
    let mut wrong_mac = mac.clone();
    wrong_mac[0] ^= 1;
    assert!(!engine.verify(message, &wrong_mac), "HMAC should fail with wrong MAC");
}

#[test]
fn test_hmac_streaming() {
    let key = b"streaming_key";
    let message = b"hello world";
    
    // Compute HMAC in one shot
    let engine = HmacEngine::new(HmacAlgorithm::Sha256, key).unwrap();
    let mac1 = engine.compute(message);
    
    // Compute HMAC with streaming
    let mut stream = engine.create_stream();
    stream.update(b"hello ");
    stream.update(b"world");
    let mac2 = stream.finalize();
    
    assert_eq!(mac1, mac2, "One-shot and streaming HMAC should produce same result");
}

#[test]
fn test_hmac_different_algorithms() {
    let key = b"test_key";
    let message = b"test_message";
    
    let mac_sha256 = HmacUtils::hmac_sha256(key, message).unwrap();
    let mac_sha512 = HmacUtils::hmac_sha512(key, message).unwrap();
    let mac_blake3 = HmacUtils::hmac_blake3(key, message).unwrap();
    
    assert_eq!(mac_sha256.len(), 32);
    assert_eq!(mac_sha512.len(), 64);
    assert_eq!(mac_blake3.len(), 32);
    
    // Different algorithms should produce different MACs
    assert_ne!(mac_sha256, mac_blake3);
    assert_ne!(mac_sha256, mac_sha512[..32]);
}

#[test]
fn test_hmac_algorithm_properties() {
    assert_eq!(HmacAlgorithm::Sha256.name(), "HMAC-SHA256");
    assert_eq!(HmacAlgorithm::Sha256.block_size(), 64);
    assert_eq!(HmacAlgorithm::Sha256.output_size(), 32);
    
    assert_eq!(HmacAlgorithm::Sha512.name(), "HMAC-SHA512");
    assert_eq!(HmacAlgorithm::Sha512.block_size(), 128);
    assert_eq!(HmacAlgorithm::Sha512.output_size(), 64);
    
    assert_eq!(HmacAlgorithm::Blake3.name(), "HMAC-BLAKE3");
    assert_eq!(HmacAlgorithm::Blake3.block_size(), 64);
    assert_eq!(HmacAlgorithm::Blake3.output_size(), 32);
}

#[test]
fn test_pbkdf2_basic_derivation() {
    let config = Pbkdf2Config::new();
    let engine = Pbkdf2Engine::new(config).unwrap();
    
    let password = b"test_password";
    let salt = b"test_salt_123456"; // At least 8 bytes
    
    let key1 = engine.derive_key(password, salt).unwrap();
    let key2 = engine.derive_key(password, salt).unwrap();
    
    assert_eq!(key1, key2, "PBKDF2 should be deterministic");
    assert_eq!(key1.len(), 32, "Default output length should be 32 bytes");
}

#[test]
fn test_pbkdf2_different_inputs() {
    let config = Pbkdf2Config::new();
    let engine = Pbkdf2Engine::new(config).unwrap();
    
    let salt = b"same_salt_123456";
    
    // Different passwords should produce different keys
    let key1 = engine.derive_key(b"password1", salt).unwrap();
    let key2 = engine.derive_key(b"password2", salt).unwrap();
    assert_ne!(key1, key2, "Different passwords should produce different keys");
    
    // Different salts should produce different keys
    let password = b"same_password";
    let key3 = engine.derive_key(password, b"salt1_123456").unwrap();
    let key4 = engine.derive_key(password, b"salt2_123456").unwrap();
    assert_ne!(key3, key4, "Different salts should produce different keys");
}

#[test]
fn test_pbkdf2_custom_parameters() {
    let config = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, 10_000, 32);
    let engine = Pbkdf2Engine::new(config).unwrap();
    
    let password = b"test_password";
    let salt = b"test_salt_123456";
    
    let key = engine.derive_key(password, salt).unwrap();
    assert_eq!(key.len(), 32);
}

#[test]
fn test_pbkdf2_different_algorithms() {
    let config_sha256 = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, 10_000, 32);
    let config_sha512 = Pbkdf2Config::with_params(HmacAlgorithm::Sha512, 10_000, 32);
    
    let engine_sha256 = Pbkdf2Engine::new(config_sha256).unwrap();
    let engine_sha512 = Pbkdf2Engine::new(config_sha512).unwrap();
    
    let password = b"test_password";
    let salt = b"test_salt_123456";
    
    let key_sha256 = engine_sha256.derive_key(password, salt).unwrap();
    let key_sha512 = engine_sha512.derive_key(password, salt).unwrap();
    
    assert_ne!(key_sha256, key_sha512, "Different algorithms should produce different keys");
}

#[test]
fn test_pbkdf2_password_hashing() {
    let config = Pbkdf2Config::new();
    let engine = Pbkdf2Engine::new(config).unwrap();
    
    let password = b"my_secure_password";
    let result = engine.hash_password(password).unwrap();
    
    assert_eq!(result.algorithm, HmacAlgorithm::Sha256);
    assert_eq!(result.iterations, 100_000);
    assert!(!result.salt.is_empty());
    assert!(!result.derived_key.is_empty());
}

#[test]
fn test_pbkdf2_password_verification() {
    let config = Pbkdf2Config::new();
    let engine = Pbkdf2Engine::new(config).unwrap();
    
    let password = b"correct_password";
    let result = engine.hash_password(password).unwrap();
    
    // Should verify with correct password
    assert!(engine.verify_password(password, &result.salt, &result.derived_key).unwrap());
    
    // Should fail with wrong password
    assert!(!engine.verify_password(b"wrong_password", &result.salt, &result.derived_key).unwrap());
}

#[test]
fn test_pbkdf2_salt_generation() {
    let salt1 = Pbkdf2Utils::generate_salt(16).unwrap();
    let salt2 = Pbkdf2Utils::generate_salt(16).unwrap();
    
    assert_eq!(salt1.len(), 16);
    assert_eq!(salt2.len(), 16);
    assert_ne!(salt1, salt2, "Should generate different random salts");
}

#[test]
fn test_pbkdf2_input_validation() {
    let config = Pbkdf2Config::new();
    let engine = Pbkdf2Engine::new(config).unwrap();
    
    // Empty password should fail
    assert!(engine.derive_key(b"", b"salt_123456").is_err());
    
    // Short salt should fail
    assert!(engine.derive_key(b"password", b"short").is_err());
}

#[test]
fn test_crypto_hex_encoding() {
    let data = b"hello world";
    
    // Test SHA-3 hex encoding
    let hex = Sha3Utils::sha3_256_string("test");
    assert!(!hex.is_empty());
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    
    // Test BLAKE3 hex encoding
    let hex = Blake3Utils::blake3_string("test");
    assert!(!hex.is_empty());
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    
    // Test HMAC hex encoding
    let hex = HmacUtils::hmac_sha256_string("key", "message").unwrap();
    assert!(!hex.is_empty());
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    
    // Test PBKDF2 hex encoding
    let salt = Pbkdf2Utils::generate_salt(16).unwrap();
    let hex = Pbkdf2Utils::to_hex(&salt);
    assert_eq!(hex.len(), 32); // 16 bytes = 32 hex chars
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_crypto_error_handling() {
    // Test HMAC with empty key
    let result = HmacEngine::new(HmacAlgorithm::Sha256, &[]);
    assert!(result.is_err());
    
    // Test PBKDF2 with invalid config
    let invalid_config = Pbkdf2Config::with_params(HmacAlgorithm::Sha256, 500, 32); // Too few iterations
    let result = Pbkdf2Engine::new(invalid_config);
    assert!(result.is_err());
}

#[test]
fn test_foundational_crypto_integration() {
    // This test ensures all crypto modules can be initialized together
    use cursed::stdlib::packages::crypto_asymmetric::init_crypto_asymmetric;
    use cursed::stdlib::packages::crypto_hash_advanced::init_crypto_hash_advanced;
    use cursed::stdlib::packages::crypto_kdf::init_crypto_kdf;
    
    assert!(init_crypto_asymmetric().is_ok());
    assert!(init_crypto_hash_advanced().is_ok());
    assert!(init_crypto_kdf().is_ok());
}
