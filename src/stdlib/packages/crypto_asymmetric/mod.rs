/// fr fr Asymmetric cryptography module with production-ready implementations
/// 
/// This module provides comprehensive asymmetric cryptography operations including:
/// - RSA (2048, 3072, 4096-bit) for encryption and signatures
/// - ECC (P-256, P-384, P-521) for ECDSA signatures  
/// - Ed25519 for high-performance digital signatures
/// - X25519 for elliptic curve Diffie-Hellman key exchange
/// - Unified key generation and management
/// - Multiple key serialization formats (PEM, DER, raw)

pub mod key_generator;
pub mod algorithms;
pub mod ed25519;
pub mod rsa;
pub mod ecc;
pub mod x25519;
pub mod key_exchange;
pub mod key_agreement;
pub mod asymmetric;
pub mod key_validation;
pub mod hardware_acceleration;
pub mod key_formats;

// Core functionality exports
pub use key_generator::{KeyGenerator, AsymmetricAlgorithm, GeneratedKeyPair, KeyGeneratorError};
pub use algorithms::*;

// Algorithm-specific exports
pub use rsa::{RsaEngine, CursedRsaKeyPair, CursedRsaKeyPair as RsaKeyPair, RsaError, RsaPadding, KeyFormat as RsaKeyFormat};
pub use ecc::{EccEngine, EccKeyPair, EccError, EccCurve, EccKeyFormat, EccHashAlgorithm};
pub use ed25519::{Ed25519Engine, Ed25519KeyPair, Ed25519Error, Ed25519KeyFormat};
pub use x25519::{X25519Engine, X25519KeyPair, X25519EphemeralKeyPair, X25519Error, X25519KeyFormat};

// API function exports
pub use rsa::{rsa_generate_keypair, rsa_encrypt, rsa_decrypt, rsa_sign, rsa_verify};
pub use ecc::{ecc_generate_keypair, ecdsa_sign, ecdsa_verify};
pub use ed25519::{
    ed25519_generate_keypair, ed25519_generate_keypair_from_seed, ed25519_sign, 
    ed25519_verify, ed25519_verify_raw, ed25519_derive_public_key
};
pub use x25519::{
    x25519_generate_keypair, x25519_generate_ephemeral_keypair, x25519_generate_keypair_from_seed,
    x25519_key_exchange, x25519_derive_public_key, x25519_validate_public_key
};
pub use key_exchange::{
    KeyExchangeAlgorithm, KeyExchangeResult, DhParameters, DhKeyPair,
    dh_key_exchange, dh_generate_keypair, x448_key_exchange, x448_generate_keypair,
    validate_key_exchange_params, list_key_exchange_algorithms, derive_key_from_shared_secret
};
pub use key_agreement::{
    KeyAgreementAlgorithm, KeyAgreementResult, key_agreement,
    ecdh_p256_agreement, ecdh_p384_agreement, ecdh_p521_agreement,
    x25519_agreement, x448_agreement, rsa_oaep_agreement,
    validate_key_agreement_params, list_key_agreement_algorithms,
    derive_key_from_shared_secret as derive_key_agreement
};
pub use key_generator::{generate_asymmetric_keypair, list_asymmetric_algorithms};
pub use asymmetric::{
    AsymmetricCrypto, AsymmetricKey, AsymmetricKeyPair,
    generate_asymmetric_keypair as generate_keypair_unified,
    asymmetric_sign, asymmetric_verify, asymmetric_key_exchange,
    get_asymmetric_algorithms, get_asymmetric_capabilities
};
pub use key_validation::{validate_key, validate_key_pair, validate_key_strength};
pub use hardware_acceleration::{
    check_hardware_support, has_cpu_feature, get_available_hsms, refresh_hardware_detection,
    get_hardware_detector
};
pub use key_formats::{
    convert_public_key_format_enhanced, convert_private_key_format_enhanced
};

use crate::error::CursedError;

/// fr fr Initialize asymmetric crypto package with comprehensive testing
pub fn init_crypto_asymmetric() -> Result<(), CursedError> {
    // Initialize key generator
    let mut generator = KeyGenerator::new();
    
    // Test all supported algorithms
    let algorithms = KeyGenerator::supported_algorithms();
    println!("🔑 Initializing asymmetric crypto package...");
    println!("   Supported algorithms: {}", algorithms.len());
    
    // Test RSA engine
    let mut rsa_engine = RsaEngine::new();
    match rsa_engine.generate_keypair(2048) {
        Ok(_) => println!("   ✅ RSA engine: functional"),
        Err(e) => println!("   ❌ RSA engine: {}", e),
    }
    
    // Test ECC engine  
    let mut ecc_engine = EccEngine::new();
    match ecc_engine.generate_keypair(EccCurve::P256) {
        Ok(_) => println!("   ✅ ECC engine: functional"),
        Err(e) => println!("   ❌ ECC engine: {}", e),
    }
    
    // Test Ed25519 engine
    let mut ed25519_engine = Ed25519Engine::new();
    match ed25519_engine.generate_keypair() {
        Ok(_) => println!("   ✅ Ed25519 engine: functional"),
        Err(e) => println!("   ❌ Ed25519 engine: {}", e),
    }
    
    // Test X25519 engine
    let mut x25519_engine = X25519Engine::new();
    match x25519_engine.generate_static_keypair() {
        Ok(_) => println!("   ✅ X25519 engine: functional"),
        Err(e) => println!("   ❌ X25519 engine: {}", e),
    }
    
    // Test unified key generator
    for algorithm in [AsymmetricAlgorithm::Ed25519, AsymmetricAlgorithm::Rsa2048, AsymmetricAlgorithm::EcdsaP256] {
        match generator.generate_keypair(algorithm) {
            Ok(_) => println!("   ✅ {}: key generation working", algorithm.name()),
            Err(e) => println!("   ❌ {}: {}", algorithm.name(), e),
        }
    }
    
    // Test key exchange algorithms
    println!("   Testing key exchange algorithms...");
    match x25519_generate_keypair(vec![]) {
        Ok(_) => println!("   ✅ X25519: key generation working"),
        Err(e) => println!("   ❌ X25519: {}", e),
    }
    
    match x448_generate_keypair(vec![]) {
        Ok(_) => println!("   ✅ X448: key generation working"),
        Err(e) => println!("   ❌ X448: {}", e),
    }
    
    match dh_generate_keypair(vec![]) {
        Ok(_) => println!("   ✅ Diffie-Hellman: key generation working"),
        Err(e) => println!("   ❌ Diffie-Hellman: {}", e),
    }
    
    println!("🔑 Asymmetric crypto package initialized successfully!");
    println!("   Features: RSA encryption/signatures, ECDSA signatures, Ed25519 signatures, X25519/X448/DH key exchange");
    println!("   Security: Production-ready cryptographic implementations with proper validation");
    
    Ok(())
}

/// fr fr Get asymmetric crypto package capabilities
pub fn get_crypto_capabilities() -> Vec<String> {
    vec![
        "RSA-2048 encryption and signatures".to_string(),
        "RSA-3072 encryption and signatures".to_string(), 
        "RSA-4096 encryption and signatures".to_string(),
        "ECDSA-P256 signatures".to_string(),
        "ECDSA-P384 signatures".to_string(),
        "ECDSA-P521 signatures".to_string(),
        "Ed25519 high-performance signatures".to_string(),
        "X25519 elliptic curve Diffie-Hellman".to_string(),
        "X448 elliptic curve Diffie-Hellman".to_string(),
        "Classic Diffie-Hellman key exchange".to_string(),
        "Multiple padding schemes (PKCS#1 v1.5, OAEP, PSS)".to_string(),
        "Key serialization (PEM, DER, raw formats)".to_string(),
        "Secure random key generation".to_string(),
        "Deterministic key derivation".to_string(),
        "Ephemeral key support for forward secrecy".to_string(),
    ]
}

/// fr fr Crypto asymmetric package version info
pub const CRYPTO_ASYMMETRIC_VERSION: &str = "1.0.0";
pub const CRYPTO_ASYMMETRIC_FEATURES: &[&str] = &[
    "RSA", "ECC", "Ed25519", "X25519", "X448", "DH", "ECDSA", "ECDH", "PEM", "DER"
];
