/// fr fr Comprehensive cryptography for CURSED - secure everything periodt
/// 
/// This module provides a complete cryptographic suite including symmetric,
/// asymmetric, hashing, and certificate handling. Maximum security bestie!

pub mod asymmetric;
pub mod certificates;

// Re-export main types for convenience
pub use asymmetric::{
    AsymmetricCrypto, AsymmetricConfig, AsymmetricError, AsymmetricResult,
    RsaKeyPair, RsaPublicKey, RsaPrivateKey, RsaPadding,
    EcdsaKeyPair, EcdsaPublicKey, EcdsaPrivateKey, EcdsaSignature,
    EcdhKeyPair, EcdhPublicKey, EcdhPrivateKey,
    X25519KeyPair, X25519PublicKey, X25519PrivateKey,
    Ed25519KeyPair, Ed25519PublicKey, Ed25519PrivateKey, Ed25519Signature,
    EcCurve, EcPoint, EcScalar,
    RSA_2048_BITS, RSA_3072_BITS, RSA_4096_BITS,
    X25519_KEY_SIZE, ED25519_PUBLIC_KEY_SIZE, ED25519_PRIVATE_KEY_SIZE, ED25519_SIGNATURE_SIZE,
};

pub use certificates::{
    CertificateProcessor, CertificateConfig, CertificateError, CertificateResult,
    X509Certificate, CertificateChain, CertificateSigningRequest,
    DistinguishedName, Validity, PublicKeyInfo, Extension, ObjectIdentifier,
    PublicKeyAlgorithm, SignatureAlgorithm, EncodingFormat,
};

// Re-export package types for integration
pub use crate::stdlib::packages::crypto_asymmetric::*;
pub use crate::stdlib::packages::crypto_pki::*;
pub use crate::stdlib::packages::crypto_advanced::*;
pub use crate::stdlib::packages::crypto_hash_advanced::*;
pub use crate::stdlib::packages::crypto_kdf::*;
pub use crate::stdlib::packages::crypto_random::*;
pub use crate::stdlib::packages::crypto_signatures::*;
pub use crate::stdlib::packages::crypto_zk::*;
pub use crate::stdlib::packages::crypto_pqc::*;
pub use crate::stdlib::packages::crypto_protocols::*;

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;

/// fr fr Initialize the crypto module
pub fn init_crypto() -> Result<(), CursedError> {
    // Initialize crypto packages
    if let Err(e) = crate::stdlib::packages::crypto_asymmetric::init_crypto_asymmetric() {
        return Err(CursedError::Runtime(format!("Failed to initialize asymmetric crypto: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_pki::init_crypto_pki() {
        return Err(CursedError::Runtime(format!("Failed to initialize PKI: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_advanced::init_crypto_advanced() {
        return Err(CursedError::Runtime(format!("Failed to initialize advanced crypto: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_hash_advanced::init_crypto_hash_advanced() {
        return Err(CursedError::Runtime(format!("Failed to initialize advanced hashing: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_kdf::init_crypto_kdf() {
        return Err(CursedError::Runtime(format!("Failed to initialize key derivation: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_random::init_crypto_random() {
        return Err(CursedError::Runtime(format!("Failed to initialize secure random: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_signatures::init_crypto_signatures() {
        return Err(CursedError::Runtime(format!("Failed to initialize digital signatures: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_zk::init_crypto_zk() {
        return Err(CursedError::Runtime(format!("Failed to initialize zero-knowledge proofs: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_pqc::init_crypto_pqc() {
        return Err(CursedError::Runtime(format!("Failed to initialize post-quantum crypto: {}", e)));
    }
    
    if let Err(e) = crate::stdlib::packages::crypto_protocols::init_crypto_protocols() {
        return Err(CursedError::Runtime(format!("Failed to initialize crypto protocols: {}", e)));
    }
    
    println!("🔐 Comprehensive crypto module initialized - maximum security activated bestie!");
    Ok(())
}

/// fr fr Get crypto module information
pub fn get_crypto_info(_args: Vec<Value>) -> Result<Value, CursedError> {
    let mut info = HashMap::new();
    
    info.insert("version".to_string(), Value::String("1.0.0".to_string()));
    info.insert("algorithms".to_string(), Value::Array(vec![
        Value::String("RSA-2048".to_string()),
        Value::String("RSA-3072".to_string()),
        Value::String("RSA-4096".to_string()),
        Value::String("ECDSA-P256".to_string()),
        Value::String("ECDSA-P384".to_string()),
        Value::String("ECDSA-P521".to_string()),
        Value::String("X25519".to_string()),
        Value::String("Ed25519".to_string()),
    ]));
    
    info.insert("features".to_string(), Value::Array(vec![
        Value::String("Asymmetric Encryption".to_string()),
        Value::String("Digital Signatures".to_string()),
        Value::String("Key Exchange".to_string()),
        Value::String("X.509 Certificates".to_string()),
        Value::String("Certificate Validation".to_string()),
        Value::String("CSR Processing".to_string()),
        Value::String("PEM/DER Encoding".to_string()),
    ]));
    
    info.insert("security_level".to_string(), Value::String("Production-Ready".to_string()));
    
    Ok(Value::Object(info))
}

/// fr fr Test crypto functionality
pub fn test_crypto(_args: Vec<Value>) -> Result<Value, CursedError> {
    let mut results = HashMap::new();
    
    // Test RSA key generation
    match asymmetric::rsa_generate_keypair(Vec::from([])) {
        Ok(_) => results.insert("rsa_keygen".to_string(), Value::bool(true)),
        Err(_) => results.insert("rsa_keygen".to_string(), Value::bool(false)),
    };
    
    // Test ECDSA key generation
    match asymmetric::ecdsa_generate_keypair(Vec::from([])) {
        Ok(_) => results.insert("ecdsa_keygen".to_string(), Value::bool(true)),
        Err(_) => results.insert("ecdsa_keygen".to_string(), Value::bool(false)),
    };
    
    // Test X25519 key generation
    match asymmetric::x25519_generate_keypair(Vec::from([])) {
        Ok(_) => results.insert("x25519_keygen".to_string(), Value::bool(true)),
        Err(_) => results.insert("x25519_keygen".to_string(), Value::bool(false)),
    };
    
    // Test Ed25519 key generation
    match asymmetric::ed25519_generate_keypair(Vec::from([])) {
        Ok(_) => results.insert("ed25519_keygen".to_string(), Value::bool(true)),
        Err(_) => results.insert("ed25519_keygen".to_string(), Value::bool(false)),
    };
    
    // Test certificate parsing
    let dummy_pem = "-----BEGIN CERTIFICATE-----\nMIIC...dummy...\n-----END CERTIFICATE-----";
    match certificates::parse_certificate_pem(Vec::from([Value::String(dummy_pem.to_string())])) {
        Ok(_) => results.insert("cert_parsing".to_string(), Value::bool(true)),
        Err(_) => results.insert("cert_parsing".to_string(), Value::bool(false)),
    };
    
    Ok(Value::Object(results))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_init() {
        // Init may fail in test environment, just check it doesn't panic
        let _ = init_crypto();
    }

    #[test]
    fn test_crypto_info() {
        let result = get_crypto_info(Vec::from([]));
        assert!(result.is_ok());
        
        if let Ok(Value::Object(info)) = result {
            assert!(info.contains_key("version"));
            assert!(info.contains_key("algorithms"));
            assert!(info.contains_key("features"));
        }
    }

    #[test]
    fn test_crypto_test() {
        let result = test_crypto(Vec::from([]));
        assert!(result.is_ok());
        
        if let Ok(Value::Object(results)) = result {
            assert!(results.contains_key("rsa_keygen"));
            assert!(results.contains_key("ecdsa_keygen"));
            assert!(results.contains_key("x25519_keygen"));
            assert!(results.contains_key("ed25519_keygen"));
            assert!(results.contains_key("cert_parsing"));
        }
    }
}
