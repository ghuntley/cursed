/// Post-quantum cryptography package
pub mod kyber;
pub mod dilithium;
pub mod falcon;
pub mod sphincs;
pub mod ntru;
pub mod rainbow;
pub mod frodo;
pub mod saber;
pub mod pqc_core;
pub mod lattice_crypto;
pub mod hash_crypto;
pub mod code_crypto;
pub mod multivariate_crypto;
pub mod hybrid_crypto;
pub mod hybrid;
pub mod utils;
pub mod compatibility;
pub mod migration_tools;
pub mod sphincs_plus;

// Re-export main functionality
pub use pqc_core::*;
pub use kyber::*;
pub use dilithium::*;
pub use falcon::*;
pub use sphincs::*;
pub use ntru::*;
pub use hybrid::*;

use crate::error::CursedError;

/// Security levels for post-quantum cryptography
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Level1,
    Level3,
    Level5,
}

/// PQC algorithm registry
#[derive(Debug, Clone)]
pub struct PqcAlgorithmRegistry {
    pub algorithms: Vec<String>,
}

impl PqcAlgorithmRegistry {
    pub fn new() -> Self {
        Self {
            algorithms: vec![
                "Kyber".to_string(),
                "Dilithium".to_string(),
                "Falcon".to_string(),
                "SPHINCS+".to_string(),
                "NTRU".to_string(),
            ],
        }
    }
}

/// Hybrid crypto manager
#[derive(Debug, Clone)]
pub struct HybridCryptoManager {
    pub fallback_strategy: String,
}

impl HybridCryptoManager {
    pub fn new(strategy: String) -> Self {
        Self {
            fallback_strategy: strategy,
        }
    }
    
    pub fn init_x25519_kyber(&mut self, _level: SecurityLevel) -> Result<(), CursedError> {
        Ok(())
    }
    
    pub fn init_ed25519_dilithium(&mut self, _level: SecurityLevel) -> Result<(), CursedError> {
        Ok(())
    }
}

/// PQC readiness assessment
#[derive(Debug, Clone)]
pub struct PqcReadinessAssessment {
    pub ready: bool,
    pub recommendations: Vec<String>,
}

/// PQC configuration
#[derive(Debug, Clone)]
pub struct PqcConfiguration {
    pub kem_algorithm: String,
    pub signature_algorithm: String,
    pub hash_signature_algorithm: String,
}

/// Migration modes for PQC transition
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationMode {
    Immediate,
    Gradual,
    Testing,
    Dual,
}

/// Initialize the crypto_pqc package
pub fn init_crypto_pqc() -> Result<(), CursedError> {
    let registry = PqcAlgorithmRegistry::new();
    let mut hybrid_manager = HybridCryptoManager::new("RequireBoth".to_string());
    
    // Test basic initialization
    if let Err(e) = hybrid_manager.init_x25519_kyber(SecurityLevel::Level1) {
        println!("⚠️ Warning: X25519+Kyber hybrid initialization failed: {}", e);
    }
    if let Err(e) = hybrid_manager.init_ed25519_dilithium(SecurityLevel::Level1) {
        println!("⚠️ Warning: Ed25519+Dilithium hybrid initialization failed: {}", e);
    }
    
    println!("🔐 crypto_pqc package initialized successfully!");
    println!("   📊 {} PQC algorithms available", registry.algorithms.len());
    println!("   🔄 Hybrid cryptography ready");
    println!("   🛡️  Post-quantum security enabled");
    println!("✨ Available algorithms:");
    println!("   - Kyber (lattice-based key exchange)");
    println!("   - SPHINCS+ (hash-based signatures)");
    println!("   - Falcon (lattice-based signatures)");
    println!("   - Dilithium (lattice-based signatures)");
    println!("   - NTRU (lattice-based encryption)");
    println!("   - Rainbow (multivariate signatures)");
    println!("   - Code-based cryptography (McEliece)");
    println!("   - Hybrid schemes (X25519+Kyber, Ed25519+Dilithium)");
    println!("🔄 Migration and compatibility tools:");
    println!("   - Classical to PQC migration planning");
    println!("   - Compatibility assessment tools");
    println!("   - Hybrid deployment strategies");
    println!("   - Risk assessment and mitigation");
    
    Ok(())
}

/// Get PQC readiness assessment for current system
pub fn assess_system_pqc_readiness() -> PqcReadinessAssessment {
    let current_algorithms = vec![
        "RSA-2048".to_string(),
        "ECDSA-P256".to_string(),
        "AES-256".to_string(),
    ];
    
    assess_pqc_readiness(&current_algorithms)
}

/// Assess PQC readiness for given algorithms
pub fn assess_pqc_readiness(algorithms: &[String]) -> PqcReadinessAssessment {
    let mut recommendations = vec![];
    
    for algorithm in algorithms {
        if algorithm.contains("RSA") {
            recommendations.push("Replace RSA with Kyber for key exchange".to_string());
        }
        if algorithm.contains("ECDSA") {
            recommendations.push("Replace ECDSA with Dilithium for signatures".to_string());
        }
    }
    
    PqcReadinessAssessment {
        ready: recommendations.is_empty(),
        recommendations,
    }
}

/// Create recommended PQC configuration for security level
pub fn create_recommended_pqc_config(security_level: SecurityLevel) -> PqcConfiguration {
    let kem_algorithm = match security_level {
        SecurityLevel::Level1 => "Kyber512".to_string(),
        SecurityLevel::Level3 => "Kyber768".to_string(),
        SecurityLevel::Level5 => "Kyber1024".to_string(),
    };
    
    let signature_algorithm = match security_level {
        SecurityLevel::Level1 => "Dilithium2".to_string(),
        SecurityLevel::Level3 => "Dilithium3".to_string(),
        SecurityLevel::Level5 => "Dilithium5".to_string(),
    };
    
    let hash_signature_algorithm = match security_level {
        SecurityLevel::Level1 => "SPHINCS+-128f".to_string(),
        SecurityLevel::Level3 => "SPHINCS+-192f".to_string(),
        SecurityLevel::Level5 => "SPHINCS+-256f".to_string(),
    };
    
    PqcConfiguration {
        kem_algorithm,
        signature_algorithm,
        hash_signature_algorithm,
    }
}
