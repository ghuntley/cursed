use crate::error::CursedError;

/// fr fr Comprehensive cryptography for CURSED - secure everything periodt
/// 
/// This module provides a unified cryptographic ecosystem including symmetric,
/// asymmetric, hashing, PKI, PQC, ZK proofs, and protocol implementations.
/// Maximum security with production-ready package management bestie!

// Core crypto modules
pub mod asymmetric;
pub mod certificates;
pub mod pqc;
pub mod nonce_generator;
pub mod security_analysis;
pub mod pqc_production;
pub mod protocols;
pub mod protocols_production;
pub mod protocols_advanced;
pub mod protocols_enhanced;
pub mod protocols_comprehensive;
pub mod types;

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
pub mod ed25519_keys;

// Re-export main types for convenience
pub use asymmetric::{
// };
pub use ed25519_keys::{
    // Note: Constants removed - they need to be defined in ed25519_keys module
// };

pub use format_conversions::{
// };

pub use x448_implementation::{
// };

pub use crypto_advanced::{
// };

pub use types::{
// };

pub use certificates::{
// };

pub use pqc::{
// };

// Import comprehensive PQC module
// use crate::stdlib::crypto_pqc;

// Re-export production PQC types
pub use pqc_production::{
// };

// Re-export protocol types that are being imported elsewhere
pub use protocols::{
// };

// Re-export random and encoding types
pub use random::{
// };

pub use encoding::{
// };

// Re-export CryptoPlatform if it exists
// Note: This may need to be created or found in appropriate module

// Re-export unified crypto ecosystem
pub use unified_api::{
    ComplianceLevel, global_crypto_manager, initialize_unified_crypto, quick_crypto_operation
// };

pub use integration_manager::{
    PackageDependency, global_integration_manager, initialize_crypto_integration
// };

pub use package_manager::{
    global_package_manager, initialize_crypto_ecosystem
// };

// Re-export package types for integration
// pub use crate::stdlib::packages::crypto_asymmetric::*;
// pub use crate::stdlib::packages::crypto_pki::*;
// pub use crate::stdlib::packages::crypto_advanced::*;
// pub use crate::stdlib::packages::crypto_hash_advanced::*;
// pub use crate::stdlib::packages::crypto_kdf::*;
// pub use crate::stdlib::packages::crypto_random::*;
// pub use crate::stdlib::packages::crypto_signatures::*;
// pub use crate::stdlib::packages::crypto_zk::*;
// pub use crate::stdlib::packages::crypto_pqc::*;
// pub use crate::stdlib::packages::crypto_protocols::*;

// use crate::stdlib::value::Value;
use std::collections::HashMap;

/// fr fr Initialize the comprehensive crypto ecosystem
pub fn init_crypto() -> std::result::crate::error::Result<()> {
    println!("🚀 Initializing comprehensive CURSED crypto ecosystem...");

    // Initialize the unified crypto ecosystem using package manager
    match package_manager::initialize_crypto_ecosystem() {
        Ok(_) => {
            println!("🔐 Comprehensive crypto ecosystem initialized - maximum security activated bestie!");
            Ok(())
        Err(e) => {
            eprintln!("❌ Failed to initialize crypto ecosystem: {}", e);
            
            // Fallback to individual package initialization for compatibility
            println!("🔄 Attempting fallback initialization...");
            
            let packages = [
//                 ("crypto_asymmetric", || crate::stdlib::packages::crypto_asymmetric::init_crypto_asymmetric()),
//                 ("crypto_pki", || crate::stdlib::packages::crypto_pki::init_crypto_pki()),
//                 ("crypto_advanced", || crate::stdlib::packages::crypto_advanced::init_crypto_advanced()),
//                 ("crypto_hash_advanced", || crate::stdlib::packages::crypto_hash_advanced::init_crypto_hash_advanced()),
//                 ("crypto_kdf", || crate::stdlib::packages::crypto_kdf::init_crypto_kdf()),
//                 ("crypto_random", || crate::stdlib::packages::crypto_random::init_crypto_random()),
//                 ("crypto_signatures", || crate::stdlib::packages::crypto_signatures::init_crypto_signatures()),
//                 ("crypto_zk", || crate::stdlib::packages::crypto_zk::init_crypto_zk()),
//                 ("crypto_pqc", || crate::stdlib::packages::crypto_pqc::init_crypto_pqc()),
//                 ("crypto_protocols", || crate::stdlib::packages::crypto_protocols::init_crypto_protocols()),
            ];

            let mut failed_packages = Vec::new();
            for (name, init_fn) in packages {
                match init_fn() {
                    Err(init_err) => {
                        println!("⚠️  {} failed: {}", name, init_err);
                        failed_packages.push(name);
                    }
                }
            if failed_packages.is_empty() {
                println!("🔐 Fallback initialization completed - basic crypto ready bestie!");
                Ok(())
            } else {
                Err(CursedError::Runtime(format!(
                    failed_packages, e
                )))
            }
        }
    }
}

/// fr fr Get comprehensive crypto module information
pub fn get_crypto_info(_args: Vec<Value>) -> std::result::crate::error::Result<()> {
    let mut info = HashMap::new();
    
    // Basic module info
    info.insert("version".to_string(), Value::String("2.0.0".to_string()));
    info.insert("ecosystem".to_string(), Value::String("Unified Crypto Ecosystem".to_string()));
    
    // Get system overview from package manager
    if let Ok(overview) = global_package_manager().get_system_overview() {
        info.extend(overview);
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
                pkg_obj.insert("name".to_string(), Value::String(pkg.to_string()));
                pkg_obj.insert("version".to_string(), Value::String(pkg.version));
                pkg_obj.insert("description".to_string(), Value::String(pkg.description));
                pkg_obj.insert("security_level".to_string(), Value::String(pkg.security_level));
                Value::Object(pkg_obj)
            })
            .collect();
        info.insert("packages".to_string(), Value::Array(package_values));
    // Enhanced features
    info.insert("features".to_string(), Value::Array(vec![
    ]));
    
    // Security compliance
    if let Ok(audit) = global_crypto_manager().get_latest_audit() {
        if let Some(audit_result) = audit {
            let mut compliance_obj = HashMap::new();
                Value::String(format!("{:?}", audit_result.compliance_level)));
                Value::Number(audit_result.overall_score));
                Value::Number(audit_result.security_vulnerabilities.len() as f64));
            info.insert("security_audit".to_string(), Value::Object(compliance_obj));
        }
    }
    
    info.insert("security_level".to_string(), Value::String("Enterprise Production-Ready".to_string()));
    
    Ok(Value::Object(info))
/// fr fr Comprehensive crypto functionality testing
pub fn test_crypto(_args: Vec<Value>) -> std::result::crate::error::Result<()> {
    let mut results = HashMap::new();
    
    println!("🧪 Running comprehensive crypto ecosystem tests...");
    
    // Test package manager functionality
    match global_package_manager().get_system_overview() {
    
    // Test individual packages
                   "crypto_zk", "crypto_pqc", "crypto_protocols"];
    
    for package_name in packages {
        match global_package_manager().test_package(package_name) {
            Ok(test_results) => {
                let success = test_results.values().all(|&v| v);
                results.insert(format!("{}_package", package_name), Value::bool(success));
            Err(_) => {
                results.insert(format!("{}_package", package_name), Value::bool(false));
            }
        }
    // Test integration functionality
    if let Ok(integration_results) = global_integration_manager().get_integration_results() {
        let all_passed = integration_results.iter().all(|r| r.success);
        results.insert("integration_tests".to_string(), Value::bool(all_passed));
        results.insert("integration_count".to_string(), Value::Number(integration_results.len() as f64));
    } else {
        results.insert("integration_tests".to_string(), Value::bool(false));
    // Test performance monitoring
    match global_crypto_manager().get_performance_statistics() {
    
    // Test security auditing
    match global_crypto_manager().get_latest_audit() {
        Ok(Some(audit)) => {
            results.insert("security_audit".to_string(), Value::bool(audit.overall_score > 80.0));
            results.insert("security_score".to_string(), Value::Number(audit.overall_score));
    
    // Legacy individual algorithm tests for backward compatibility
    match asymmetric::rsa_generate_keypair(Vec::from([])) {
    
    match asymmetric::ecdsa_generate_keypair(Vec::from([])) {
    
    match asymmetric::x25519_generate_keypair(Vec::from([])) {
    
    match asymmetric::ed25519_generate_keypair(Vec::from([])) {
    
    let dummy_pem = "-----BEGIN CERTIFICATE-----\nMIIC...dummy...\n-----END CERTIFICATE-----";
    match certificates::parse_certificate_pem(Vec::from([Value::String(dummy_pem.to_string())])) {
    
    // Calculate overall success rate
    let total_tests = results.len();
    let passed_tests = results.values()
        .filter_map(|v| match v {
        })
        .filter(|&b| b)
        .count();
    
    let success_rate = if total_tests > 0 {
        (passed_tests as f64 / total_tests as f64) * 100.0
    } else {
        0.0
    
    results.insert("overall_success_rate".to_string(), Value::Number(success_rate));
    results.insert("total_tests".to_string(), Value::Number(total_tests as f64));
    results.insert("passed_tests".to_string(), Value::Number(passed_tests as f64));
    
    println!("🔐 Crypto ecosystem tests completed - {:.1}% success rate ({}/{} tests passed)", 
             success_rate, passed_tests, total_tests);
    
    Ok(Value::Object(results))
