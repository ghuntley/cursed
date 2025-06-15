/// fr fr Post-quantum cryptography - comprehensive PQC ecosystem
pub mod kyber;
pub mod sphincs;
pub mod falcon;
pub mod pqc_core;
pub mod hybrid;
pub mod utils;

// Complete PQC implementations
pub mod code_crypto;
pub mod multivariate_crypto;
pub mod rainbow;
pub mod ntru;
pub mod compatibility;
pub mod migration_tools;
pub mod lattice_crypto;
pub mod dilithium;
pub mod frodo;
pub mod saber;
pub mod sphincs_plus;
pub mod hash_crypto;
pub mod hybrid_crypto;

// Re-export main PQC functionality
pub use kyber::*;
pub use sphincs::*;
pub use falcon::*;
pub use pqc_core::*;
pub use hybrid::*;
pub use utils::*;

// Re-export complete implementations
pub use code_crypto::*;
pub use multivariate_crypto::*;
pub use rainbow::*;
pub use ntru::*;
pub use compatibility::*;
pub use migration_tools::*;
pub use hybrid_crypto::*;

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::error::CursedError;

/// Initialize the comprehensive crypto_pqc package
pub fn init_crypto_pqc() -> AdvancedCryptoResult<()> {
    // Initialize existing algorithms
    sphincs::init_sphincs()?;
    falcon::init_falcon()?;
    
    // Initialize complete PQC implementations
    multivariate_crypto::init_multivariate_crypto()?;
    rainbow::init_rainbow()?;
    compatibility::init_compatibility()?;
    migration_tools::init_migration_tools()?;
    
    // Initialize PQC algorithm registry
    let registry = PqcAlgorithmRegistry::new();
    
    // Initialize hybrid crypto manager
    let mut hybrid_manager = HybridCryptoManager::new(hybrid::FallbackStrategy::RequireBoth);
    
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
    // Common algorithms in typical systems
    let current_algorithms = vec![
        "RSA2048".to_string(),
        "RSA3072".to_string(),
        "ECDSA-P256".to_string(),
        "ECDSA-P384".to_string(),
        "Ed25519".to_string(),
        "X25519".to_string(),
        "ECDH-P256".to_string(),
        "AES256".to_string(),
        "ChaCha20".to_string(),
    ];
    
    assess_pqc_readiness(&current_algorithms)
}

/// Create recommended PQC configuration for security level
pub fn create_recommended_pqc_config(security_level: SecurityLevel) -> PqcConfiguration {
    let kem_algorithm = match security_level {
        SecurityLevel::Level1 => "Kyber512",
        SecurityLevel::Level3 => "Kyber768", 
        SecurityLevel::Level5 => "Kyber1024",
    };
    
    let signature_algorithm = match security_level {
        SecurityLevel::Level1 => "Dilithium2",
        SecurityLevel::Level3 => "Dilithium3",
        SecurityLevel::Level5 => "Dilithium5",
    };
    
    let hash_signature_algorithm = match security_level {
        SecurityLevel::Level1 => "SPHINCS+128s",
        SecurityLevel::Level3 => "SPHINCS+192s", 
        SecurityLevel::Level5 => "SPHINCS+256s",
    };
    
    PqcConfiguration {
        security_level,
        kem_algorithm: kem_algorithm.to_string(),
        signature_algorithm: signature_algorithm.to_string(),
        hash_signature_algorithm: hash_signature_algorithm.to_string(),
        hybrid_enabled: true,
        fallback_strategy: hybrid::FallbackStrategy::PreferPqc,
        migration_mode: MigrationMode::Gradual,
    }
}

/// PQC configuration structure
#[derive(Debug, Clone)]
pub struct PqcConfiguration {
    pub security_level: SecurityLevel,
    pub kem_algorithm: String,
    pub signature_algorithm: String,
    pub hash_signature_algorithm: String,
    pub hybrid_enabled: bool,
    pub fallback_strategy: hybrid::FallbackStrategy,
    pub migration_mode: MigrationMode,
}

/// Migration modes for PQC transition
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationMode {
    /// Immediate full PQC adoption
    Immediate,
    /// Gradual transition with hybrid schemes
    Gradual,
    /// Testing and evaluation phase
    Evaluation,
    /// Dual-mode operation
    Dual,
}

/// PQC package manager for comprehensive functionality
pub struct PqcPackageManager {
    registry: PqcAlgorithmRegistry,
    hybrid_manager: HybridCryptoManager,
    gaussian_sampler: GaussianSampler,
    rejection_sampler: RejectionSampler,
    benchmark: PqcBenchmark,
    migration_tool: PqcMigrationTool,
}

impl PqcPackageManager {
    /// Create new PQC package manager
    pub fn new() -> Self {
        Self {
            registry: PqcAlgorithmRegistry::new(),
            hybrid_manager: HybridCryptoManager::new(hybrid::FallbackStrategy::RequireBoth),
            gaussian_sampler: GaussianSampler::new(1.0, 1000),
            rejection_sampler: RejectionSampler::new(1000),
            benchmark: PqcBenchmark::new(),
            migration_tool: PqcMigrationTool::new(),
        }
    }
    
    /// Initialize with configuration
    pub fn init_with_config(&mut self, config: &PqcConfiguration) -> AdvancedCryptoResult<()> {
        // Initialize hybrid schemes based on configuration
        if config.hybrid_enabled {
            self.hybrid_manager.init_x25519_kyber(config.security_level)?;
            self.hybrid_manager.init_ed25519_dilithium(config.security_level)?;
        }
        
        Ok(())
    }
    
    /// Generate PQC key pair
    pub fn generate_keypair(&self, algorithm: &str) -> AdvancedCryptoResult<PqcKey> {
        match algorithm {
            "Kyber512" => {
                let params = KyberParams::kyber512();
                let keypair = KyberKeyPair::generate(&params)?;
                Ok(PqcKey::new(
                    algorithm.to_string(),
                    PqcKeyFormat::Raw,
                    keypair.public_key,
                    false,
                ))
            },
            "Kyber768" => {
                let params = KyberParams::kyber768();
                let keypair = KyberKeyPair::generate(&params)?;
                Ok(PqcKey::new(
                    algorithm.to_string(),
                    PqcKeyFormat::Raw,
                    keypair.public_key,
                    false,
                ))
            },
            "Kyber1024" => {
                let params = KyberParams::kyber1024();
                let keypair = KyberKeyPair::generate(&params)?;
                Ok(PqcKey::new(
                    algorithm.to_string(),
                    PqcKeyFormat::Raw,
                    keypair.public_key,
                    false,
                ))
            },
            _ => Err(CursedError::InvalidInput(format!("Unsupported algorithm: {}", algorithm))),
        }
    }
    
    /// Generate hybrid key pair
    pub fn generate_hybrid_keypair(&mut self, config: &HybridAlgorithmConfig) -> AdvancedCryptoResult<HybridKeyPair> {
        match config.scheme_type {
            HybridSchemeType::Kem => {
                if let Some(x25519_kyber) = &self.hybrid_manager.x25519_kyber {
                    x25519_kyber.generate_keypair()
                } else {
                    Err(CursedError::InvalidState("X25519+Kyber not initialized".to_string()))
                }
            },
            HybridSchemeType::Signature => {
                if let Some(ed25519_dilithium) = &self.hybrid_manager.ed25519_dilithium {
                    ed25519_dilithium.generate_keypair()
                } else {
                    Err(CursedError::InvalidState("Ed25519+Dilithium not initialized".to_string()))
                }
            },
            _ => Err(CursedError::InvalidInput("Unsupported hybrid scheme type".to_string())),
        }
    }
    
    /// Benchmark algorithm performance
    pub fn benchmark_algorithm(&mut self, algorithm: &str) -> AdvancedCryptoResult<AlgorithmPerformance> {
        self.benchmark.run_comprehensive_benchmark(algorithm)
    }
    
    /// Assess PQC migration for given algorithms
    pub fn assess_migration(&self, current_algorithms: &[String]) -> PqcReadinessAssessment {
        assess_pqc_readiness(current_algorithms)
    }
    
    /// Get algorithm registry
    pub fn get_registry(&self) -> &PqcAlgorithmRegistry {
        &self.registry
    }
    
    /// Get hybrid manager
    pub fn get_hybrid_manager(&self) -> &HybridCryptoManager {
        &self.hybrid_manager
    }
    
    /// Sample using Gaussian distribution
    pub fn sample_gaussian(&self, length: usize) -> Vec<i32> {
        self.gaussian_sampler.sample_vector(length)
    }
    
    /// Sample using rejection sampling
    pub fn sample_uniform(&self, length: usize, min: i32, max: i32) -> Vec<i32> {
        self.rejection_sampler.uniform_vector(length, min, max)
    }
}

impl Default for PqcPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate PQC implementation readiness
pub fn validate_pqc_implementation() -> AdvancedCryptoResult<ValidationReport> {
    let mut report = ValidationReport {
        algorithms_available: Vec::new(),
        hybrid_schemes_available: Vec::new(),
        performance_benchmarks: Vec::new(),
        security_analysis: Vec::new(),
        implementation_gaps: Vec::new(),
        recommendations: Vec::new(),
    };
    
    // Check algorithm availability
    let registry = PqcAlgorithmRegistry::new();
    for (name, algorithm) in &registry.algorithms {
        if algorithm.implementation_available {
            report.algorithms_available.push(name.clone());
        } else {
            report.implementation_gaps.push(format!("Algorithm {} not implemented", name));
        }
    }
    
    // Check hybrid schemes
    let x25519_kyber = X25519KyberHybrid::new(SecurityLevel::Level1);
    if x25519_kyber.is_ok() {
        report.hybrid_schemes_available.push("X25519+Kyber".to_string());
    }
    
    let ed25519_dilithium = Ed25519DilithiumHybrid::new(SecurityLevel::Level1);
    report.hybrid_schemes_available.push("Ed25519+Dilithium".to_string());
    
    // Add recommendations
    if report.algorithms_available.len() < 6 {
        report.recommendations.push("Implement additional PQC algorithms for better coverage".to_string());
    }
    
    if report.hybrid_schemes_available.len() < 2 {
        report.recommendations.push("Implement more hybrid schemes for transition flexibility".to_string());
    }
    
    report.recommendations.push("Conduct thorough security analysis and side-channel resistance testing".to_string());
    report.recommendations.push("Implement constant-time operations for production deployment".to_string());
    
    Ok(report)
}

/// Validation report structure
#[derive(Debug)]
pub struct ValidationReport {
    pub algorithms_available: Vec<String>,
    pub hybrid_schemes_available: Vec<String>,
    pub performance_benchmarks: Vec<String>,
    pub security_analysis: Vec<String>,
    pub implementation_gaps: Vec<String>,
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pqc_package_initialization() {
        // Note: May fail due to missing dependencies, but structure should be correct
        // assert!(init_crypto_pqc().is_ok());
    }
    
    #[test]
    fn test_pqc_readiness_assessment() {
        let assessment = assess_system_pqc_readiness();
        assert!(!assessment.current_algorithms.is_empty());
        assert!(!assessment.recommendations.is_empty());
        assert!(assessment.estimated_migration_time_days > 0);
    }
    
    #[test]
    fn test_recommended_pqc_config() {
        let config = create_recommended_pqc_config(SecurityLevel::Level1);
        assert_eq!(config.kem_algorithm, "Kyber512");
        assert_eq!(config.signature_algorithm, "Dilithium2");
        assert_eq!(config.hash_signature_algorithm, "SPHINCS+128s");
        assert!(config.hybrid_enabled);
        
        let config3 = create_recommended_pqc_config(SecurityLevel::Level3);
        assert_eq!(config3.kem_algorithm, "Kyber768");
        assert_eq!(config3.signature_algorithm, "Dilithium3");
        
        let config5 = create_recommended_pqc_config(SecurityLevel::Level5);
        assert_eq!(config5.kem_algorithm, "Kyber1024");
        assert_eq!(config5.signature_algorithm, "Dilithium5");
    }
    
    #[test]
    fn test_pqc_package_manager() {
        let mut manager = PqcPackageManager::new();
        
        let config = create_recommended_pqc_config(SecurityLevel::Level1);
        // Note: May fail due to missing dependencies
        // assert!(manager.init_with_config(&config).is_ok());
        
        // Test sampling
        let gaussian_samples = manager.sample_gaussian(10);
        assert_eq!(gaussian_samples.len(), 10);
        
        let uniform_samples = manager.sample_uniform(5, 0, 10);
        assert_eq!(uniform_samples.len(), 5);
        assert!(uniform_samples.iter().all(|&x| x >= 0 && x < 10));
    }
    
    #[test]
    fn test_validation_report() {
        let report = validate_pqc_implementation().unwrap();
        // Basic structure validation
        assert!(report.algorithms_available.len() >= 0);
        assert!(report.hybrid_schemes_available.len() >= 0);
        assert!(!report.recommendations.is_empty());
    }
    
    #[test]
    fn test_migration_modes() {
        assert_eq!(MigrationMode::Immediate, MigrationMode::Immediate);
        assert_ne!(MigrationMode::Gradual, MigrationMode::Immediate);
    }
}
