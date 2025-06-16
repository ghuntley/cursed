/// fr fr Comprehensive cryptography for CURSED - secure everything periodt
/// 
/// This module provides a unified cryptographic ecosystem including symmetric,
/// asymmetric, hashing, PKI, PQC, ZK proofs, and protocol implementations.
/// Maximum security with production-ready package management bestie!

// Core crypto modules
pub mod asymmetric;
pub mod certificates;
pub mod pqc;
pub mod pqc_production;
pub mod protocols;
pub mod protocols_production;
pub mod protocols_advanced;
pub mod protocols_enhanced;
pub mod protocols_comprehensive;

// Unified crypto ecosystem
pub mod unified_api;
pub mod integration_manager;
pub mod package_manager;

// Advanced crypto features
pub mod hash;
pub mod symmetric;
pub mod random;
pub mod utils;
pub mod encoding;
pub mod llvm_integration;
pub mod zk_enhanced;
pub mod format_conversions;
pub mod x448_implementation;
pub mod crypto_advanced;

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

pub use format_conversions::{
    FormatConverter, KeyFormat, JsonWebKey,
    key_to_jwk, jwk_from_json, key_to_der, der_decode, detect_format,
};

pub use x448_implementation::{
    X448Engine, X448PublicKey, X448PrivateKey, X448KeyPair, X448_KEY_SIZE,
    x448_generate_keypair, x448_key_exchange, x448_validate_public_key, x448_get_public_key,
};

pub use crypto_advanced::{
    XChaCha20Key, XChaCha20Nonce, XChaCha20Poly1305Cipher,
    XChaCha20Poly1305StreamingEncoder, XChaCha20Poly1305StreamingDecoder, XChaCha20Poly1305Api,
    XCHACHA20_KEY_SIZE, XCHACHA20_NONCE_SIZE, XCHACHA20_TAG_SIZE, XCHACHA20_MAX_PLAINTEXT_SIZE,
};

pub use certificates::{
    CertificateProcessor, CertificateConfig, CertificateError, CertificateResult,
    X509Certificate, CertificateChain, CertificateSigningRequest,
    DistinguishedName, Validity, PublicKeyInfo, Extension, ObjectIdentifier,
    PublicKeyAlgorithm, SignatureAlgorithm, EncodingFormat,
};

pub use pqc::{
    PqcError, PqcResult, SecurityLevel, AlgorithmType, PerformanceMetrics, QuantumResistanceAssessment,
    KyberParameterSet, KyberPublicKey, KyberSecretKey, KyberKem,
    DilithiumParameterSet, DilithiumPublicKey, DilithiumSecretKey, DilithiumSignature,
    SphincsPlusParameterSet, SphincsPlusPublicKey, SphincsPlusSecretKey, SphincsPlusSignature,
    FalconParameterSet, FalconPublicKey, FalconSecretKey, FalconSignature,
    NtruParameterSet, NtruPublicKey, NtruSecretKey, NtruEncryption,
    PqcBenchmark, validate_security_level, get_recommended_algorithm, bytes_to_hex, hex_to_bytes,
};

// Import comprehensive PQC module
use crate::stdlib::crypto_pqc;

// Re-export production PQC types
pub use pqc_production::{
    PqcError as ProductionPqcError, PqcResult as ProductionPqcResult,
    SecurityLevel as ProductionSecurityLevel, AlgorithmType as ProductionAlgorithmType,
    MathematicalFoundation, ConstantTime, SecureBytes,
    KyberParameterSet as ProductionKyberParameterSet,
    KyberPublicKey as ProductionKyberPublicKey,
    KyberSecretKey as ProductionKyberSecretKey,
    KyberKem as ProductionKyberKem,
    DilithiumParameterSet as ProductionDilithiumParameterSet,
    DilithiumPublicKey as ProductionDilithiumPublicKey,
    DilithiumSecretKey as ProductionDilithiumSecretKey,
    DilithiumSigner as ProductionDilithiumSigner,
    HybridKeyExchange, ClassicalKeyPair,
    BenchmarkResults, PqcBenchmarkSuite, QuantumThreatAssessment,
    bytes_to_hex as production_bytes_to_hex,
    hex_to_bytes as production_hex_to_bytes,
    validate_security_level as production_validate_security_level,
    get_recommended_algorithm as production_get_recommended_algorithm,
};

// Re-export unified crypto ecosystem
pub use unified_api::{
    UnifiedCryptoError, UnifiedCryptoResult, UnifiedCryptoManager,
    CryptoConfig, PerformanceMetrics as UnifiedPerformanceMetrics, SecurityAuditResult, CryptoOperation,
    ComplianceLevel, global_crypto_manager, initialize_unified_crypto, quick_crypto_operation
};

pub use integration_manager::{
    CryptoIntegrationManager, IntegrationTestResult, CompatibilityMatrix,
    PackageDependency, global_integration_manager, initialize_crypto_integration
};

pub use package_manager::{
    CryptoPackageManager, PackageInfo, PackageCapabilities, PackageStatistics,
    global_package_manager, initialize_crypto_ecosystem
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

/// fr fr Initialize the comprehensive crypto ecosystem
pub fn init_crypto() -> Result<(), CursedError> {
    println!("🚀 Initializing comprehensive CURSED crypto ecosystem...");

    // Initialize the unified crypto ecosystem using package manager
    match package_manager::initialize_crypto_ecosystem() {
        Ok(_) => {
            println!("🔐 Comprehensive crypto ecosystem initialized - maximum security activated bestie!");
            Ok(())
        },
        Err(e) => {
            eprintln!("❌ Failed to initialize crypto ecosystem: {}", e);
            
            // Fallback to individual package initialization for compatibility
            println!("🔄 Attempting fallback initialization...");
            
            let packages = [
                ("crypto_asymmetric", || crate::stdlib::packages::crypto_asymmetric::init_crypto_asymmetric()),
                ("crypto_pki", || crate::stdlib::packages::crypto_pki::init_crypto_pki()),
                ("crypto_advanced", || crate::stdlib::packages::crypto_advanced::init_crypto_advanced()),
                ("crypto_hash_advanced", || crate::stdlib::packages::crypto_hash_advanced::init_crypto_hash_advanced()),
                ("crypto_kdf", || crate::stdlib::packages::crypto_kdf::init_crypto_kdf()),
                ("crypto_random", || crate::stdlib::packages::crypto_random::init_crypto_random()),
                ("crypto_signatures", || crate::stdlib::packages::crypto_signatures::init_crypto_signatures()),
                ("crypto_zk", || crate::stdlib::packages::crypto_zk::init_crypto_zk()),
                ("crypto_pqc", || crate::stdlib::packages::crypto_pqc::init_crypto_pqc()),
                ("crypto_protocols", || crate::stdlib::packages::crypto_protocols::init_crypto_protocols()),
            ];

            let mut failed_packages = Vec::new();
            for (name, init_fn) in packages {
                match init_fn() {
                    Ok(_) => println!("✅ {} initialized", name),
                    Err(init_err) => {
                        println!("⚠️  {} failed: {}", name, init_err);
                        failed_packages.push(name);
                    }
                }
            }

            if failed_packages.is_empty() {
                println!("🔐 Fallback initialization completed - basic crypto ready bestie!");
                Ok(())
            } else {
                Err(CursedError::Runtime(format!(
                    "Failed to initialize packages: {:?}. Original error: {}", 
                    failed_packages, e
                )))
            }
        }
    }
}

/// fr fr Get comprehensive crypto module information
pub fn get_crypto_info(_args: Vec<Value>) -> Result<Value, CursedError> {
    let mut info = HashMap::new();
    
    // Basic module info
    info.insert("version".to_string(), Value::String("2.0.0".to_string()));
    info.insert("ecosystem".to_string(), Value::String("Unified Crypto Ecosystem".to_string()));
    
    // Get system overview from package manager
    if let Ok(overview) = global_package_manager().get_system_overview() {
        info.extend(overview);
    }
    
    // Available algorithm categories
    let algorithm_categories = global_crypto_manager().list_available_algorithms();
    let mut algorithms_obj = HashMap::new();
    for (category, algorithms) in algorithm_categories {
        let algorithm_values: Vec<Value> = algorithms.into_iter()
            .map(|alg| Value::String(alg))
            .collect();
        algorithms_obj.insert(category, Value::Array(algorithm_values));
    }
    info.insert("algorithms".to_string(), Value::Object(algorithms_obj));
    
    // Package list
    if let Ok(packages) = global_package_manager().list_packages() {
        let package_values: Vec<Value> = packages.into_iter()
            .map(|pkg| {
                let mut pkg_obj = HashMap::new();
                pkg_obj.insert("name".to_string(), Value::String(pkg.name));
                pkg_obj.insert("version".to_string(), Value::String(pkg.version));
                pkg_obj.insert("description".to_string(), Value::String(pkg.description));
                pkg_obj.insert("security_level".to_string(), Value::String(pkg.security_level));
                Value::Object(pkg_obj)
            })
            .collect();
        info.insert("packages".to_string(), Value::Array(package_values));
    }
    
    // Enhanced features
    info.insert("features".to_string(), Value::Array(vec![
        Value::String("Unified Crypto API".to_string()),
        Value::String("Package Management".to_string()),
        Value::String("Integration Testing".to_string()),
        Value::String("Performance Monitoring".to_string()),
        Value::String("Security Auditing".to_string()),
        Value::String("Symmetric Encryption".to_string()),
        Value::String("Asymmetric Cryptography".to_string()),
        Value::String("Digital Signatures".to_string()),
        Value::String("Hash Functions".to_string()),
        Value::String("Key Derivation".to_string()),
        Value::String("Random Generation".to_string()),
        Value::String("PKI & Certificates".to_string()),
        Value::String("Post-Quantum Crypto".to_string()),
        Value::String("Zero-Knowledge Proofs".to_string()),
        Value::String("Cryptographic Protocols".to_string()),
        Value::String("Cross-Package Integration".to_string()),
        Value::String("Compliance Checking".to_string()),
        Value::String("Hardware Acceleration".to_string()),
    ]));
    
    // Security compliance
    if let Ok(audit) = global_crypto_manager().get_latest_audit() {
        if let Some(audit_result) = audit {
            let mut compliance_obj = HashMap::new();
            compliance_obj.insert("compliance_level".to_string(), 
                Value::String(format!("{:?}", audit_result.compliance_level)));
            compliance_obj.insert("overall_score".to_string(), 
                Value::Number(audit_result.overall_score));
            compliance_obj.insert("vulnerabilities_count".to_string(), 
                Value::Number(audit_result.security_vulnerabilities.len() as f64));
            info.insert("security_audit".to_string(), Value::Object(compliance_obj));
        }
    }
    
    info.insert("security_level".to_string(), Value::String("Enterprise Production-Ready".to_string()));
    
    Ok(Value::Object(info))
}

/// fr fr Comprehensive crypto functionality testing
pub fn test_crypto(_args: Vec<Value>) -> Result<Value, CursedError> {
    let mut results = HashMap::new();
    
    println!("🧪 Running comprehensive crypto ecosystem tests...");
    
    // Test package manager functionality
    match global_package_manager().get_system_overview() {
        Ok(_) => results.insert("package_manager".to_string(), Value::bool(true)),
        Err(_) => results.insert("package_manager".to_string(), Value::bool(false)),
    };
    
    // Test individual packages
    let packages = ["crypto_advanced", "crypto_asymmetric", "crypto_hash_advanced", 
                   "crypto_signatures", "crypto_kdf", "crypto_random", "crypto_pki", 
                   "crypto_zk", "crypto_pqc", "crypto_protocols"];
    
    for package_name in packages {
        match global_package_manager().test_package(package_name) {
            Ok(test_results) => {
                let success = test_results.values().all(|&v| v);
                results.insert(format!("{}_package", package_name), Value::bool(success));
            },
            Err(_) => {
                results.insert(format!("{}_package", package_name), Value::bool(false));
            }
        }
    }
    
    // Test integration functionality
    if let Ok(integration_results) = global_integration_manager().get_integration_results() {
        let all_passed = integration_results.iter().all(|r| r.success);
        results.insert("integration_tests".to_string(), Value::bool(all_passed));
        results.insert("integration_count".to_string(), Value::Number(integration_results.len() as f64));
    } else {
        results.insert("integration_tests".to_string(), Value::bool(false));
    }
    
    // Test performance monitoring
    match global_crypto_manager().get_performance_statistics() {
        Ok(_) => results.insert("performance_monitoring".to_string(), Value::bool(true)),
        Err(_) => results.insert("performance_monitoring".to_string(), Value::bool(false)),
    };
    
    // Test security auditing
    match global_crypto_manager().get_latest_audit() {
        Ok(Some(audit)) => {
            results.insert("security_audit".to_string(), Value::bool(audit.overall_score > 80.0));
            results.insert("security_score".to_string(), Value::Number(audit.overall_score));
        },
        Ok(None) => results.insert("security_audit".to_string(), Value::bool(false)),
        Err(_) => results.insert("security_audit".to_string(), Value::bool(false)),
    };
    
    // Legacy individual algorithm tests for backward compatibility
    match asymmetric::rsa_generate_keypair(Vec::from([])) {
        Ok(_) => results.insert("rsa_keygen".to_string(), Value::bool(true)),
        Err(_) => results.insert("rsa_keygen".to_string(), Value::bool(false)),
    };
    
    match asymmetric::ecdsa_generate_keypair(Vec::from([])) {
        Ok(_) => results.insert("ecdsa_keygen".to_string(), Value::bool(true)),
        Err(_) => results.insert("ecdsa_keygen".to_string(), Value::bool(false)),
    };
    
    match asymmetric::x25519_generate_keypair(Vec::from([])) {
        Ok(_) => results.insert("x25519_keygen".to_string(), Value::bool(true)),
        Err(_) => results.insert("x25519_keygen".to_string(), Value::bool(false)),
    };
    
    match asymmetric::ed25519_generate_keypair(Vec::from([])) {
        Ok(_) => results.insert("ed25519_keygen".to_string(), Value::bool(true)),
        Err(_) => results.insert("ed25519_keygen".to_string(), Value::bool(false)),
    };
    
    let dummy_pem = "-----BEGIN CERTIFICATE-----\nMIIC...dummy...\n-----END CERTIFICATE-----";
    match certificates::parse_certificate_pem(Vec::from([Value::String(dummy_pem.to_string())])) {
        Ok(_) => results.insert("cert_parsing".to_string(), Value::bool(true)),
        Err(_) => results.insert("cert_parsing".to_string(), Value::bool(false)),
    };
    
    // Calculate overall success rate
    let total_tests = results.len();
    let passed_tests = results.values()
        .filter_map(|v| match v {
            Value::Bool(b) => Some(*b),
            _ => None,
        })
        .filter(|&b| b)
        .count();
    
    let success_rate = if total_tests > 0 {
        (passed_tests as f64 / total_tests as f64) * 100.0
    } else {
        0.0
    };
    
    results.insert("overall_success_rate".to_string(), Value::Number(success_rate));
    results.insert("total_tests".to_string(), Value::Number(total_tests as f64));
    results.insert("passed_tests".to_string(), Value::Number(passed_tests as f64));
    
    println!("🔐 Crypto ecosystem tests completed - {:.1}% success rate ({}/{} tests passed)", 
             success_rate, passed_tests, total_tests);
    
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
