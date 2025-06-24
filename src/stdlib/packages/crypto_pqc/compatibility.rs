/// fr fr Post-quantum cryptography compatibility and transition tools
/// 
/// This module provides compatibility layers and transition tools for migrating
/// from classical cryptography to post-quantum cryptography, ensuring smooth
/// integration with existing systems while maintaining security.

use crate::error::CursedError;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::packages::crypto_asymmetric::{AsymmetricKey, AsymmetricKeyPair};
use crate::error::Error;
use super::pqc_core::{PqcKey, SecurityLevel, PqcKeyFormat};
use super::hybrid::{HybridKeyPair, HybridAlgorithmConfig, HybridSchemeType};
use std::collections::HashMap;
use std::fmt;

/// fr fr Compatibility mode for PQC transition
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityMode {
    /// Only classical algorithms (legacy mode)
    ClassicalOnly,
    /// Classical algorithms with PQC validation
    ClassicalWithPqcValidation,
    /// Hybrid mode (both classical and PQC)
    Hybrid,
    /// PQC with classical fallback
    PqcWithClassicalFallback,
    /// Only PQC algorithms (future mode)
    PqcOnly,
}

impl CompatibilityMode {
    pub fn name(&self) -> &'static str {
        match self {
            CompatibilityMode::ClassicalOnly => "Classical Only",
            CompatibilityMode::ClassicalWithPqcValidation => "Classical with PQC Validation",
            CompatibilityMode::Hybrid => "Hybrid Mode",
            CompatibilityMode::PqcWithClassicalFallback => "PQC with Classical Fallback",
            CompatibilityMode::PqcOnly => "PQC Only",
        }
    }
    
    pub fn supports_classical(&self) -> bool {
        matches!(self, 
            CompatibilityMode::ClassicalOnly | 
            CompatibilityMode::ClassicalWithPqcValidation |
            CompatibilityMode::Hybrid |
            CompatibilityMode::PqcWithClassicalFallback
        )
    }
    
    pub fn supports_pqc(&self) -> bool {
        matches!(self, 
            CompatibilityMode::ClassicalWithPqcValidation |
            CompatibilityMode::Hybrid |
            CompatibilityMode::PqcWithClassicalFallback |
            CompatibilityMode::PqcOnly
        )
    }
    
    pub fn requires_both(&self) -> bool {
        matches!(self, CompatibilityMode::Hybrid)
    }
}

/// fr fr Cryptographic algorithm mapping
#[derive(Debug, Clone)]
pub struct AlgorithmMapping {
    pub classical_algorithm: String,
    pub pqc_equivalent: String,
    pub hybrid_scheme: Option<String>,
    pub security_level: SecurityLevel,
    pub compatibility_notes: Vec<String>,
}

impl AlgorithmMapping {
    /// Create mapping for key exchange algorithms
    pub fn key_exchange_mapping() -> Vec<Self> {
        vec![
            AlgorithmMapping {
                classical_algorithm: "RSA-2048".to_string(),
                pqc_equivalent: "Kyber512".to_string(),
                hybrid_scheme: Some("RSA2048+Kyber512".to_string()),
                security_level: SecurityLevel::Level1,
                compatibility_notes: vec![
                    "RSA-2048 provides ~112-bit security".to_string(),
                    "Kyber512 provides 128-bit quantum security".to_string(),
                    "Hybrid mode recommended for transition".to_string(),
                ],
            },
            AlgorithmMapping {
                classical_algorithm: "RSA-3072".to_string(),
                pqc_equivalent: "Kyber768".to_string(),
                hybrid_scheme: Some("RSA3072+Kyber768".to_string()),
                security_level: SecurityLevel::Level3,
                compatibility_notes: vec![
                    "RSA-3072 provides ~128-bit security".to_string(),
                    "Kyber768 provides 192-bit quantum security".to_string(),
                ],
            },
            AlgorithmMapping {
                classical_algorithm: "RSA-4096".to_string(),
                pqc_equivalent: "Kyber1024".to_string(),
                hybrid_scheme: Some("RSA4096+Kyber1024".to_string()),
                security_level: SecurityLevel::Level5,
                compatibility_notes: vec![
                    "RSA-4096 provides ~152-bit security".to_string(),
                    "Kyber1024 provides 256-bit quantum security".to_string(),
                ],
            },
            AlgorithmMapping {
                classical_algorithm: "ECDH-P256".to_string(),
                pqc_equivalent: "Kyber512".to_string(),
                hybrid_scheme: Some("P256+Kyber512".to_string()),
                security_level: SecurityLevel::Level1,
                compatibility_notes: vec![
                    "ECDH-P256 provides ~128-bit security".to_string(),
                    "Direct replacement with Kyber512".to_string(),
                ],
            },
            AlgorithmMapping {
                classical_algorithm: "X25519".to_string(),
                pqc_equivalent: "Kyber768".to_string(),
                hybrid_scheme: Some("X25519+Kyber768".to_string()),
                security_level: SecurityLevel::Level3,
                compatibility_notes: vec![
                    "X25519 provides ~128-bit security".to_string(),
                    "Hybrid with Kyber768 recommended".to_string(),
                ],
            },
        ]
    }
    
    /// Create mapping for signature algorithms
    pub fn signature_mapping() -> Vec<Self> {
        vec![
            AlgorithmMapping {
                classical_algorithm: "RSA-2048".to_string(),
                pqc_equivalent: "Dilithium2".to_string(),
                hybrid_scheme: Some("RSA2048+Dilithium2".to_string()),
                security_level: SecurityLevel::Level1,
                compatibility_notes: vec![
                    "RSA-2048 signatures are large".to_string(),
                    "Dilithium2 provides smaller signatures".to_string(),
                ],
            },
            AlgorithmMapping {
                classical_algorithm: "ECDSA-P256".to_string(),
                pqc_equivalent: "Dilithium2".to_string(),
                hybrid_scheme: Some("P256+Dilithium2".to_string()),
                security_level: SecurityLevel::Level1,
                compatibility_notes: vec![
                    "ECDSA-P256 provides compact signatures".to_string(),
                    "Dilithium2 signatures are larger but quantum-safe".to_string(),
                ],
            },
            AlgorithmMapping {
                classical_algorithm: "Ed25519".to_string(),
                pqc_equivalent: "Dilithium3".to_string(),
                hybrid_scheme: Some("Ed25519+Dilithium3".to_string()),
                security_level: SecurityLevel::Level3,
                compatibility_notes: vec![
                    "Ed25519 has excellent performance".to_string(),
                    "Hybrid preserves performance while adding PQC".to_string(),
                ],
            },
            AlgorithmMapping {
                classical_algorithm: "RSA-PSS-3072".to_string(),
                pqc_equivalent: "SPHINCS+192s".to_string(),
                hybrid_scheme: Some("RSA3072+SPHINCS192s".to_string()),
                security_level: SecurityLevel::Level3,
                compatibility_notes: vec![
                    "RSA-PSS provides security proofs".to_string(),
                    "SPHINCS+ provides hash-based quantum security".to_string(),
                ],
            },
        ]
    }
}

/// fr fr Compatibility assessment result
#[derive(Debug, Clone)]
pub struct CompatibilityAssessment {
    pub current_algorithms: Vec<String>,
    pub compatibility_mode: CompatibilityMode,
    pub migration_recommendations: Vec<MigrationRecommendation>,
    pub security_analysis: SecurityAnalysis,
    pub performance_impact: PerformanceImpact,
    pub timeline_estimate: TimelineEstimate,
}

/// fr fr Migration recommendation
#[derive(Debug, Clone)]
pub struct MigrationRecommendation {
    pub priority: MigrationPriority,
    pub current_algorithm: String,
    pub recommended_replacement: String,
    pub migration_steps: Vec<String>,
    pub risks: Vec<String>,
    pub estimated_effort: String,
}

/// fr fr Migration priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MigrationPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl MigrationPriority {
    pub fn name(&self) -> &'static str {
        match self {
            MigrationPriority::Low => "Low",
            MigrationPriority::Medium => "Medium",
            MigrationPriority::High => "High",
            MigrationPriority::Critical => "Critical",
        }
    }
}

/// fr fr Security analysis for migration
#[derive(Debug, Clone)]
pub struct SecurityAnalysis {
    pub quantum_vulnerable_algorithms: Vec<String>,
    pub quantum_safe_algorithms: Vec<String>,
    pub overall_quantum_readiness: f64, // Percentage (0.0 to 100.0)
    pub critical_vulnerabilities: Vec<String>,
    pub security_recommendations: Vec<String>,
}

/// fr fr Performance impact assessment
#[derive(Debug, Clone)]
pub struct PerformanceImpact {
    pub key_generation_factor: f64,   // Multiplicative factor vs classical
    pub signature_size_factor: f64,   // Multiplicative factor vs classical
    pub verification_time_factor: f64, // Multiplicative factor vs classical
    pub bandwidth_impact: String,      // Description of bandwidth changes
    pub storage_impact: String,        // Description of storage changes
}

/// fr fr Timeline estimate for migration
#[derive(Debug, Clone)]
pub struct TimelineEstimate {
    pub planning_phase_weeks: u32,
    pub development_phase_weeks: u32,
    pub testing_phase_weeks: u32,
    pub deployment_phase_weeks: u32,
    pub total_weeks: u32,
    pub critical_milestones: Vec<String>,
}

/// fr fr Compatibility engine for PQC transition
#[derive(Debug)]
pub struct CompatibilityEngine {
    algorithm_mappings: HashMap<String, AlgorithmMapping>,
    compatibility_rules: Vec<CompatibilityRule>,
}

impl CompatibilityEngine {
    /// Create new compatibility engine
    pub fn new() -> Self {
        let mut engine = Self {
            algorithm_mappings: HashMap::new(),
            compatibility_rules: Vec::new(),
        };
        
        engine.initialize_default_mappings();
        engine.initialize_default_rules();
        engine
    }
    
    /// Initialize default algorithm mappings
    fn initialize_default_mappings(&mut self) {
        // Add key exchange mappings
        for mapping in AlgorithmMapping::key_exchange_mapping() {
            self.algorithm_mappings.insert(mapping.classical_algorithm.clone(), mapping);
        }
        
        // Add signature mappings
        for mapping in AlgorithmMapping::signature_mapping() {
            self.algorithm_mappings.insert(mapping.classical_algorithm.clone(), mapping);
        }
    }
    
    /// Initialize default compatibility rules
    fn initialize_default_rules(&mut self) {
        self.compatibility_rules = vec![
            CompatibilityRule {
                name: "Quantum Vulnerability".to_string(),
                description: "Classical algorithms vulnerable to quantum attacks".to_string(),
                applies_to: vec!["RSA".to_string(), "ECDSA".to_string(), "ECDH".to_string(), "DH".to_string()],
                severity: RuleSeverity::High,
                recommendation: "Migrate to quantum-safe alternatives immediately".to_string(),
            },
            CompatibilityRule {
                name: "Performance Impact".to_string(),
                description: "PQC algorithms may have different performance characteristics".to_string(),
                applies_to: vec!["Dilithium".to_string(), "SPHINCS+".to_string(), "Kyber".to_string()],
                severity: RuleSeverity::Medium,
                recommendation: "Evaluate performance impact in production environment".to_string(),
            },
            CompatibilityRule {
                name: "Signature Size".to_string(),
                description: "PQC signatures may be larger than classical signatures".to_string(),
                applies_to: vec!["Dilithium".to_string(), "SPHINCS+".to_string(), "Rainbow".to_string()],
                severity: RuleSeverity::Medium,
                recommendation: "Consider bandwidth and storage implications".to_string(),
            },
            CompatibilityRule {
                name: "Key Size".to_string(),
                description: "PQC keys may be larger than classical keys".to_string(),
                applies_to: vec!["Kyber".to_string(), "NTRU".to_string(), "Rainbow".to_string()],
                severity: RuleSeverity::Low,
                recommendation: "Evaluate key storage and transmission requirements".to_string(),
            },
        ];
    }
    
    /// Assess compatibility for given algorithms
    pub fn assess_compatibility(&self, current_algorithms: &[String]) -> AdvancedCryptoResult<CompatibilityAssessment> {
        let mut quantum_vulnerable = Vec::new();
        let mut quantum_safe = Vec::new();
        let mut migration_recommendations = Vec::new();
        
        for algorithm in current_algorithms {
            if self.is_quantum_vulnerable(algorithm) {
                quantum_vulnerable.push(algorithm.clone());
                
                if let Some(mapping) = self.algorithm_mappings.get(algorithm) {
                    let recommendation = MigrationRecommendation {
                        priority: self.determine_priority(algorithm),
                        current_algorithm: algorithm.clone(),
                        recommended_replacement: mapping.pqc_equivalent.clone(),
                        migration_steps: self.generate_migration_steps(algorithm, &mapping.pqc_equivalent),
                        risks: self.assess_migration_risks(algorithm),
                        estimated_effort: self.estimate_effort(algorithm),
                    };
                    migration_recommendations.push(recommendation);
                }
            } else {
                quantum_safe.push(algorithm.clone());
            }
        }
        
        let quantum_readiness = if current_algorithms.is_empty() {
            0.0
        } else {
            (quantum_safe.len() as f64 / current_algorithms.len() as f64) * 100.0
        };
        
        let security_analysis = SecurityAnalysis {
            quantum_vulnerable_algorithms: quantum_vulnerable,
            quantum_safe_algorithms: quantum_safe,
            overall_quantum_readiness: quantum_readiness,
            critical_vulnerabilities: self.identify_critical_vulnerabilities(current_algorithms),
            security_recommendations: self.generate_security_recommendations(current_algorithms),
        };
        
        let performance_impact = self.assess_performance_impact(current_algorithms);
        let timeline_estimate = self.estimate_timeline(&migration_recommendations);
        
        let compatibility_mode = if quantum_readiness >= 80.0 {
            CompatibilityMode::PqcOnly
        } else if quantum_readiness >= 50.0 {
            CompatibilityMode::Hybrid
        } else if quantum_readiness >= 20.0 {
            CompatibilityMode::PqcWithClassicalFallback
        } else if quantum_readiness > 0.0 {
            CompatibilityMode::ClassicalWithPqcValidation
        } else {
            CompatibilityMode::ClassicalOnly
        };
        
        Ok(CompatibilityAssessment {
            current_algorithms: current_algorithms.to_vec(),
            compatibility_mode,
            migration_recommendations,
            security_analysis,
            performance_impact,
            timeline_estimate,
        })
    }
    
    /// Check if algorithm is vulnerable to quantum attacks
    fn is_quantum_vulnerable(&self, algorithm: &str) -> bool {
        let vulnerable_patterns = ["RSA", "ECDSA", "ECDH", "DH", "DSA"];
        vulnerable_patterns.iter().any(|&pattern| algorithm.contains(pattern))
    }
    
    /// Determine migration priority for algorithm
    fn determine_priority(&self, algorithm: &str) -> MigrationPriority {
        if algorithm.contains("RSA-1024") || algorithm.contains("MD5") || algorithm.contains("SHA-1") {
            MigrationPriority::Critical
        } else if algorithm.contains("RSA-2048") || algorithm.contains("ECDSA") {
            MigrationPriority::High
        } else if algorithm.contains("RSA-3072") || algorithm.contains("ECDH") {
            MigrationPriority::Medium
        } else {
            MigrationPriority::Low
        }
    }
    
    /// Generate migration steps for algorithm transition
    fn generate_migration_steps(&self, current: &str, target: &str) -> Vec<String> {
        vec![
            format!("1. Evaluate {} implementation options", target),
            format!("2. Set up test environment with {}", target),
            format!("3. Implement hybrid mode ({} + {})", current, target),
            format!("4. Conduct performance testing"),
            format!("5. Gradually migrate traffic to {}", target),
            format!("6. Phase out {} when fully migrated", current),
        ]
    }
    
    /// Assess risks for migration
    fn assess_migration_risks(&self, algorithm: &str) -> Vec<String> {
        let mut risks = vec![
            "Performance degradation during transition".to_string(),
            "Compatibility issues with legacy systems".to_string(),
            "Increased bandwidth and storage requirements".to_string(),
        ];
        
        if algorithm.contains("RSA") {
            risks.push("Large signature size increase with PQC alternatives".to_string());
        }
        
        if algorithm.contains("ECDSA") || algorithm.contains("ECDH") {
            risks.push("Key size increase may affect protocols".to_string());
        }
        
        risks
    }
    
    /// Estimate effort for migration
    fn estimate_effort(&self, algorithm: &str) -> String {
        if algorithm.contains("RSA-1024") || algorithm.contains("MD5") {
            "High (6-12 months) - Critical security vulnerability".to_string()
        } else if algorithm.contains("RSA") {
            "Medium (3-6 months) - Requires careful planning".to_string()
        } else if algorithm.contains("ECDSA") || algorithm.contains("ECDH") {
            "Low-Medium (2-4 months) - More straightforward replacement".to_string()
        } else {
            "Low (1-2 months) - Minimal changes required".to_string()
        }
    }
    
    /// Identify critical vulnerabilities
    fn identify_critical_vulnerabilities(&self, algorithms: &[String]) -> Vec<String> {
        let mut vulnerabilities = Vec::new();
        
        for algorithm in algorithms {
            if algorithm.contains("RSA-1024") {
                vulnerabilities.push("RSA-1024 is cryptographically broken".to_string());
            }
            if algorithm.contains("MD5") {
                vulnerabilities.push("MD5 has known collision vulnerabilities".to_string());
            }
            if algorithm.contains("SHA-1") {
                vulnerabilities.push("SHA-1 has known collision vulnerabilities".to_string());
            }
            if algorithm.contains("DES") || algorithm.contains("3DES") {
                vulnerabilities.push("DES/3DES have insufficient key lengths".to_string());
            }
        }
        
        vulnerabilities
    }
    
    /// Generate security recommendations
    fn generate_security_recommendations(&self, algorithms: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if algorithms.iter().any(|alg| self.is_quantum_vulnerable(alg)) {
            recommendations.push("Implement post-quantum cryptography to prepare for quantum threats".to_string());
            recommendations.push("Use hybrid modes during transition period".to_string());
        }
        
        if algorithms.iter().any(|alg| alg.contains("RSA-2048")) {
            recommendations.push("Consider migrating RSA-2048 to RSA-3072 or PQC alternatives".to_string());
        }
        
        recommendations.push("Implement crypto-agility to enable future algorithm transitions".to_string());
        recommendations.push("Regular security audits of cryptographic implementations".to_string());
        recommendations.push("Monitor NIST post-quantum standardization progress".to_string());
        
        recommendations
    }
    
    /// Assess performance impact of migration
    fn assess_performance_impact(&self, algorithms: &[String]) -> PerformanceImpact {
        let has_signatures = algorithms.iter().any(|alg| 
            alg.contains("RSA") || alg.contains("ECDSA") || alg.contains("Ed25519")
        );
        
        let has_key_exchange = algorithms.iter().any(|alg| 
            alg.contains("ECDH") || alg.contains("DH") || alg.contains("X25519")
        );
        
        let key_gen_factor = if has_key_exchange { 2.0 } else { 1.5 };
        let sig_size_factor = if has_signatures { 5.0 } else { 1.0 };
        let verify_factor = if has_signatures { 1.2 } else { 1.0 };
        
        PerformanceImpact {
            key_generation_factor: key_gen_factor,
            signature_size_factor: sig_size_factor,
            verification_time_factor: verify_factor,
            bandwidth_impact: "Increased due to larger signatures and keys".to_string(),
            storage_impact: "Increased key and certificate storage requirements".to_string(),
        }
    }
    
    /// Estimate migration timeline
    fn estimate_timeline(&self, recommendations: &[MigrationRecommendation]) -> TimelineEstimate {
        let critical_count = recommendations.iter().filter(|r| r.priority == MigrationPriority::Critical).count();
        let high_count = recommendations.iter().filter(|r| r.priority == MigrationPriority::High).count();
        
        let planning_weeks = if critical_count > 0 { 2 } else if high_count > 0 { 4 } else { 6 };
        let development_weeks = critical_count as u32 * 4 + high_count as u32 * 6 + 8;
        let testing_weeks = if critical_count > 0 { 4 } else { 6 };
        let deployment_weeks = if critical_count > 0 { 2 } else { 4 };
        
        let total_weeks = planning_weeks + development_weeks + testing_weeks + deployment_weeks;
        
        let mut milestones = vec![
            "Complete compatibility assessment".to_string(),
            "Implement test environment".to_string(),
            "Deploy hybrid systems".to_string(),
            "Complete PQC migration".to_string(),
        ];
        
        if critical_count > 0 {
            milestones.insert(1, "Address critical vulnerabilities".to_string());
        }
        
        TimelineEstimate {
            planning_phase_weeks: planning_weeks,
            development_phase_weeks: development_weeks,
            testing_phase_weeks: testing_weeks,
            deployment_phase_weeks: deployment_weeks,
            total_weeks,
            critical_milestones: milestones,
        }
    }
    
    /// Convert classical key to PQC equivalent
    pub fn convert_to_pqc(&self, classical_key: &AsymmetricKey) -> AdvancedCryptoResult<PqcKey> {
        if let Some(mapping) = self.algorithm_mappings.get(&classical_key.algorithm) {
            // Create equivalent PQC key (simplified conversion)
            let pqc_key_data = self.generate_equivalent_pqc_key(&classical_key.key_data, &mapping.pqc_equivalent)?;
            
            Ok(PqcKey::new(
                mapping.pqc_equivalent.clone(),
                PqcKeyFormat::Raw,
                pqc_key_data,
                classical_key.is_private,
            ))
        } else {
            Err(CursedError::InvalidInput(format!("No PQC mapping for algorithm: {}", classical_key.algorithm)))
        }
    }
    
    /// Create hybrid key from classical key
    pub fn create_hybrid_key(&self, classical_keypair: &AsymmetricKeyPair) -> AdvancedCryptoResult<HybridKeyPair> {
        if let Some(mapping) = self.algorithm_mappings.get(&classical_keypair.public_key.algorithm) {
            // Create PQC component
            let pqc_key_data = self.generate_equivalent_pqc_key(&classical_keypair.public_key.key_data, &mapping.pqc_equivalent)?;
            let pqc_key = PqcKey::new(
                mapping.pqc_equivalent.clone(),
                PqcKeyFormat::Raw,
                pqc_key_data,
                false,
            );
            
            // Create hybrid configuration
            let scheme_type = if mapping.classical_algorithm.contains("RSA") || mapping.classical_algorithm.contains("ECDSA") {
                HybridSchemeType::Signature
            } else {
                HybridSchemeType::Kem
            };
            
            let config = HybridAlgorithmConfig {
                scheme_type,
                classical_algorithm: mapping.classical_algorithm.clone(),
                pqc_algorithm: mapping.pqc_equivalent.clone(),
                security_level: mapping.security_level,
                fallback_enabled: true,
                performance_priority: true,
            };
            
            Ok(HybridKeyPair::new(classical_keypair.clone(), pqc_key, config))
        } else {
            Err(CursedError::InvalidInput(format!("No hybrid mapping for algorithm: {}", classical_keypair.public_key.algorithm)))
        }
    }
    
    /// Generate equivalent PQC key (simplified)
    fn generate_equivalent_pqc_key(&self, classical_key_data: &[u8], pqc_algorithm: &str) -> AdvancedCryptoResult<Vec<u8>> {
        // This is a simplified key conversion for demonstration
        // In practice, this would involve proper key generation using the classical key as entropy
        
        let key_size = match pqc_algorithm {
            "Kyber512" => 800,
            "Kyber768" => 1184,
            "Kyber1024" => 1568,
            "Dilithium2" => 1312,
            "Dilithium3" => 1952,
            "Dilithium5" => 2592,
            "SPHINCS+128s" => 32,
            "SPHINCS+192s" => 48,
            "SPHINCS+256s" => 64,
            _ => 1024,
        };
        
        // Simple deterministic generation based on classical key
        let mut pqc_key = Vec::with_capacity(key_size);
        let mut seed = classical_key_data.iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64));
        
        for _ in 0..key_size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            pqc_key.push((seed >> 24) as u8);
        }
        
        Ok(pqc_key)
    }
    
    /// Get algorithm mapping
    pub fn get_mapping(&self, algorithm: &str) -> Option<&AlgorithmMapping> {
        self.algorithm_mappings.get(algorithm)
    }
    
    /// Get all mappings
    pub fn get_all_mappings(&self) -> &HashMap<String, AlgorithmMapping> {
        &self.algorithm_mappings
    }
}

impl Default for CompatibilityEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Compatibility rule for validation
#[derive(Debug, Clone)]
pub struct CompatibilityRule {
    pub name: String,
    pub description: String,
    pub applies_to: Vec<String>,
    pub severity: RuleSeverity,
    pub recommendation: String,
}

/// fr fr Rule severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl RuleSeverity {
    pub fn name(&self) -> &'static str {
        match self {
            RuleSeverity::Low => "Low",
            RuleSeverity::Medium => "Medium",
            RuleSeverity::High => "High",
            RuleSeverity::Critical => "Critical",
        }
    }
}

/// fr fr Compatibility errors
#[derive(Debug, Clone)]
pub enum CompatibilityError {
    UnsupportedAlgorithm(String),
    ConversionError(String),
    MappingError(String),
    ConfigurationError(String),
}

impl fmt::Display for CompatibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompatibilityError::UnsupportedAlgorithm(msg) => write!(f, "Unsupported algorithm: {}", msg),
            CompatibilityError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
            CompatibilityError::MappingError(msg) => write!(f, "Mapping error: {}", msg),
            CompatibilityError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for CompatibilityError {}

impl From<CompatibilityError> for CursedError {
    fn from(err: CompatibilityError) -> Self {
        CursedError::CryptoError(err.to_string())
    }
}

/// Initialize compatibility module
pub fn init_compatibility() -> AdvancedCryptoResult<()> {
    let engine = CompatibilityEngine::new();
    
    println!("🔄 PQC compatibility module initialized successfully!");
    println!("   📊 {} algorithm mappings loaded", engine.algorithm_mappings.len());
    println!("   📋 {} compatibility rules loaded", engine.compatibility_rules.len());
    println!("   🔄 Classical to PQC conversion ready");
    println!("   🌉 Hybrid key generation ready");
    println!("   📈 Migration assessment tools ready");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compatibility_mode() {
        assert!(CompatibilityMode::Hybrid.supports_classical());
        assert!(CompatibilityMode::Hybrid.supports_pqc());
        assert!(CompatibilityMode::Hybrid.requires_both());
        
        assert!(CompatibilityMode::ClassicalOnly.supports_classical());
        assert!(!CompatibilityMode::ClassicalOnly.supports_pqc());
        
        assert!(!CompatibilityMode::PqcOnly.supports_classical());
        assert!(CompatibilityMode::PqcOnly.supports_pqc());
    }
    
    #[test]
    fn test_algorithm_mappings() {
        let key_exchange_mappings = AlgorithmMapping::key_exchange_mapping();
        assert!(!key_exchange_mappings.is_empty());
        
        let rsa_mapping = key_exchange_mappings.iter()
            .find(|m| m.classical_algorithm == "RSA-2048")
            .unwrap();
        assert_eq!(rsa_mapping.pqc_equivalent, "Kyber512");
        assert_eq!(rsa_mapping.security_level, SecurityLevel::Level1);
        
        let signature_mappings = AlgorithmMapping::signature_mapping();
        assert!(!signature_mappings.is_empty());
        
        let ecdsa_mapping = signature_mappings.iter()
            .find(|m| m.classical_algorithm == "ECDSA-P256")
            .unwrap();
        assert_eq!(ecdsa_mapping.pqc_equivalent, "Dilithium2");
    }
    
    #[test]
    fn test_compatibility_engine() {
        let engine = CompatibilityEngine::new();
        assert!(!engine.algorithm_mappings.is_empty());
        assert!(!engine.compatibility_rules.is_empty());
        
        // Test quantum vulnerability detection
        assert!(engine.is_quantum_vulnerable("RSA-2048"));
        assert!(engine.is_quantum_vulnerable("ECDSA-P256"));
        assert!(!engine.is_quantum_vulnerable("Kyber512"));
        assert!(!engine.is_quantum_vulnerable("Dilithium2"));
    }
    
    #[test]
    fn test_compatibility_assessment() {
        let engine = CompatibilityEngine::new();
        let algorithms = vec![
            "RSA-2048".to_string(),
            "ECDSA-P256".to_string(),
            "AES-256".to_string(),
        ];
        
        let assessment = engine.assess_compatibility(&algorithms).unwrap();
        
        assert_eq!(assessment.current_algorithms.len(), 3);
        assert_eq!(assessment.security_analysis.quantum_vulnerable_algorithms.len(), 2);
        assert_eq!(assessment.security_analysis.quantum_safe_algorithms.len(), 1);
        assert!(assessment.security_analysis.overall_quantum_readiness < 100.0);
        assert!(!assessment.migration_recommendations.is_empty());
    }
    
    #[test]
    fn test_migration_priority() {
        let engine = CompatibilityEngine::new();
        
        assert_eq!(engine.determine_priority("RSA-1024"), MigrationPriority::Critical);
        assert_eq!(engine.determine_priority("RSA-2048"), MigrationPriority::High);
        assert_eq!(engine.determine_priority("RSA-3072"), MigrationPriority::Medium);
        assert_eq!(engine.determine_priority("AES-256"), MigrationPriority::Low);
    }
    
    #[test]
    fn test_migration_steps() {
        let engine = CompatibilityEngine::new();
        let steps = engine.generate_migration_steps("RSA-2048", "Kyber512");
        
        assert!(!steps.is_empty());
        assert!(steps.iter().any(|step| step.contains("Kyber512")));
        assert!(steps.iter().any(|step| step.contains("hybrid")));
    }
    
    #[test]
    fn test_performance_impact() {
        let engine = CompatibilityEngine::new();
        let algorithms = vec!["RSA-2048".to_string(), "ECDSA-P256".to_string()];
        let impact = engine.assess_performance_impact(&algorithms);
        
        assert!(impact.signature_size_factor > 1.0);
        assert!(impact.key_generation_factor >= 1.0);
        assert!(!impact.bandwidth_impact.is_empty());
    }
    
    #[test]
    fn test_timeline_estimation() {
        let recommendations = vec![
            MigrationRecommendation {
                priority: MigrationPriority::Critical,
                current_algorithm: "RSA-1024".to_string(),
                recommended_replacement: "Kyber512".to_string(),
                migration_steps: vec![],
                risks: vec![],
                estimated_effort: "High".to_string(),
            },
        ];
        
        let engine = CompatibilityEngine::new();
        let timeline = engine.estimate_timeline(&recommendations);
        
        assert!(timeline.total_weeks > 0);
        assert!(!timeline.critical_milestones.is_empty());
        assert_eq!(timeline.planning_phase_weeks, 2); // Critical items need fast planning
    }
    
    #[test]
    fn test_rule_severity() {
        assert_eq!(RuleSeverity::Critical.name(), "Critical");
        assert_eq!(RuleSeverity::High.name(), "High");
        assert_eq!(RuleSeverity::Medium.name(), "Medium");
        assert_eq!(RuleSeverity::Low.name(), "Low");
    }
}
