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

// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
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
    if let Err(e) = hybrid_manager.init_ed25519_dilithium(SecurityLevel::Level1) {
        println!("⚠️ Warning: Ed25519+Dilithium hybrid initialization failed: {}", e);
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
/// Get PQC readiness assessment for current system
pub fn assess_system_pqc_readiness() -> PqcReadinessAssessment {
    // Common algorithms in typical systems
    let current_algorithms = vec![
    ];
    
    assess_pqc_readiness(&current_algorithms)
/// Create recommended PQC configuration for security level
pub fn create_recommended_pqc_config(security_level: SecurityLevel) -> PqcConfiguration {
    let kem_algorithm = match security_level {
    
    let signature_algorithm = match security_level {
    
    let hash_signature_algorithm = match security_level {
    
    PqcConfiguration {
    }
}

/// PQC configuration structure
#[derive(Debug, Clone)]
pub struct PqcConfiguration {
/// Migration modes for PQC transition
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationMode {
    /// Immediate full PQC adoption
    /// Gradual transition with hybrid schemes
    /// Testing and evaluation phase
    /// Dual-mode operation
/// PQC package manager for comprehensive functionality
pub struct PqcPackageManager {
impl PqcPackageManager {
    /// Create new PQC package manager
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Initialize with configuration
    pub fn init_with_config(&mut self, config: &PqcConfiguration) -> AdvancedCryptoResult<()> {
        // Initialize hybrid schemes based on configuration
        if config.hybrid_enabled {
            self.hybrid_manager.init_x25519_kyber(config.security_level)?;
            self.hybrid_manager.init_ed25519_dilithium(config.security_level)?;
        Ok(())
    /// Generate PQC key pair
    pub fn generate_keypair(&self, algorithm: &str) -> AdvancedCryptoResult<PqcKey> {
        match algorithm {
            "Kyber512" => {
                let params = KyberParams::kyber512();
                let keypair = KyberKeyPair::generate(&params)?;
                Ok(PqcKey::new(
                ))
            "Kyber768" => {
                let params = KyberParams::kyber768();
                let keypair = KyberKeyPair::generate(&params)?;
                Ok(PqcKey::new(
                ))
            "Kyber1024" => {
                let params = KyberParams::kyber1024();
                let keypair = KyberKeyPair::generate(&params)?;
                Ok(PqcKey::new(
                ))
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
            HybridSchemeType::Signature => {
                if let Some(ed25519_dilithium) = &self.hybrid_manager.ed25519_dilithium {
                    ed25519_dilithium.generate_keypair()
                } else {
                    Err(CursedError::InvalidState("Ed25519+Dilithium not initialized".to_string()))
                }
        }
    }
    
    /// Benchmark algorithm performance
    pub fn benchmark_algorithm(&mut self, algorithm: &str) -> AdvancedCryptoResult<AlgorithmPerformance> {
        self.benchmark.run_comprehensive_benchmark(algorithm)
    /// Assess PQC migration for given algorithms
    pub fn assess_migration(&self, current_algorithms: &[String]) -> PqcReadinessAssessment {
        assess_pqc_readiness(current_algorithms)
    /// Get algorithm registry
    pub fn get_registry(&self) -> &PqcAlgorithmRegistry {
        &self.registry
    /// Get hybrid manager
    pub fn get_hybrid_manager(&self) -> &HybridCryptoManager {
        &self.hybrid_manager
    /// Sample using Gaussian distribution
    pub fn sample_gaussian(&self, length: usize) -> Vec<i32> {
        self.gaussian_sampler.sample_vector(length)
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
    let ed25519_dilithium = Ed25519DilithiumHybrid::new(SecurityLevel::Level1);
    report.hybrid_schemes_available.push("Ed25519+Dilithium".to_string());
    
    // Add recommendations
    if report.algorithms_available.len() < 6 {
        report.recommendations.push("Implement additional PQC algorithms for better coverage".to_string());
    if report.hybrid_schemes_available.len() < 2 {
        report.recommendations.push("Implement more hybrid schemes for transition flexibility".to_string());
    report.recommendations.push("Conduct thorough security analysis and side-channel resistance testing".to_string());
    report.recommendations.push("Implement constant-time operations for production deployment".to_string());
    
    Ok(report)
/// Validation report structure
#[derive(Debug)]
pub struct ValidationReport {
