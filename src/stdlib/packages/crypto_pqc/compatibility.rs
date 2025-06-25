/// fr fr Post-quantum cryptography compatibility and transition tools
/// 
/// This module provides compatibility layers and transition tools for migrating
/// from classical cryptography to post-quantum cryptography, ensuring smooth
/// integration with existing systems while maintaining security.

use crate::error::CursedError;
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
// use crate::stdlib::packages::crypto_asymmetric::{AsymmetricKey, AsymmetricKeyPair};
use super::pqc_core::{PqcKey, SecurityLevel, PqcKeyFormat};
use super::hybrid::{HybridKeyPair, HybridAlgorithmConfig, HybridSchemeType};
use std::collections::HashMap;
use std::fmt;

/// fr fr Compatibility mode for PQC transition
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityMode {
    /// Only classical algorithms (legacy mode)
    /// Classical algorithms with PQC validation
    /// Hybrid mode (both classical and PQC)
    /// PQC with classical fallback
    /// Only PQC algorithms (future mode)
impl CompatibilityMode {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
    
    pub fn supports_classical(&self) -> bool {
            CompatibilityMode::ClassicalOnly | 
            CompatibilityMode::ClassicalWithPqcValidation |
            CompatibilityMode::Hybrid |
            CompatibilityMode::PqcWithClassicalFallback
        )
    pub fn supports_pqc(&self) -> bool {
            CompatibilityMode::ClassicalWithPqcValidation |
            CompatibilityMode::Hybrid |
            CompatibilityMode::PqcWithClassicalFallback |
            CompatibilityMode::PqcOnly
        )
    pub fn requires_both(&self) -> bool {
        matches!(self, CompatibilityMode::Hybrid)
    }
}

/// fr fr Cryptographic algorithm mapping
#[derive(Debug, Clone)]
pub struct AlgorithmMapping {
impl AlgorithmMapping {
    /// Create mapping for key exchange algorithms
    pub fn key_exchange_mapping() -> Vec<Self> {
        vec![
            AlgorithmMapping {
                compatibility_notes: vec![
            AlgorithmMapping {
                compatibility_notes: vec![
            AlgorithmMapping {
                compatibility_notes: vec![
            AlgorithmMapping {
                compatibility_notes: vec![
            AlgorithmMapping {
                compatibility_notes: vec![
        ]
    /// Create mapping for signature algorithms
    pub fn signature_mapping() -> Vec<Self> {
        vec![
            AlgorithmMapping {
                compatibility_notes: vec![
            AlgorithmMapping {
                compatibility_notes: vec![
            AlgorithmMapping {
                compatibility_notes: vec![
            AlgorithmMapping {
                compatibility_notes: vec![
        ]
    }
}

/// fr fr Compatibility assessment result
#[derive(Debug, Clone)]
pub struct CompatibilityAssessment {
/// fr fr Migration recommendation
#[derive(Debug, Clone)]
pub struct MigrationRecommendation {
/// fr fr Migration priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MigrationPriority {
impl MigrationPriority {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
/// fr fr Security analysis for migration
#[derive(Debug, Clone)]
pub struct SecurityAnalysis {
    pub overall_quantum_readiness: f64, // Percentage (0.0 to 100.0)
/// fr fr Performance impact assessment
#[derive(Debug, Clone)]
pub struct PerformanceImpact {
    pub key_generation_factor: f64,   // Multiplicative factor vs classical
    pub signature_size_factor: f64,   // Multiplicative factor vs classical
    pub verification_time_factor: f64, // Multiplicative factor vs classical
    pub bandwidth_impact: String,      // Description of bandwidth changes
    pub storage_impact: String,        // Description of storage changes
/// fr fr Timeline estimate for migration
#[derive(Debug, Clone)]
pub struct TimelineEstimate {
/// fr fr Compatibility engine for PQC transition
#[derive(Debug)]
pub struct CompatibilityEngine {
impl CompatibilityEngine {
    /// Create new compatibility engine
    pub fn new() -> Self {
        let mut engine = Self {
        
        engine.initialize_default_mappings();
        engine.initialize_default_rules();
        engine
    /// Initialize default algorithm mappings
    fn initialize_default_mappings(&mut self) {
        // Add key exchange mappings
        for mapping in AlgorithmMapping::key_exchange_mapping() {
            self.algorithm_mappings.insert(mapping.classical_algorithm.clone(), mapping);
        // Add signature mappings
        for mapping in AlgorithmMapping::signature_mapping() {
            self.algorithm_mappings.insert(mapping.classical_algorithm.clone(), mapping);
        }
    }
    
    /// Initialize default compatibility rules
    fn initialize_default_rules(&mut self) {
        self.compatibility_rules = vec![
            CompatibilityRule {
            CompatibilityRule {
            CompatibilityRule {
            CompatibilityRule {
        ];
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
        
        let security_analysis = SecurityAnalysis {
        
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
        
        Ok(CompatibilityAssessment {
        })
    /// Check if algorithm is vulnerable to quantum attacks
    fn is_quantum_vulnerable(&self, algorithm: &str) -> bool {
        let vulnerable_patterns = ["RSA", "ECDSA", "ECDH", "DH", "DSA"];
        vulnerable_patterns.iter().any(|&pattern| algorithm.contains(pattern))
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
        ]
    /// Assess risks for migration
    fn assess_migration_risks(&self, algorithm: &str) -> Vec<String> {
        let mut risks = vec![
        ];
        
        if algorithm.contains("RSA") {
            risks.push("Large signature size increase with PQC alternatives".to_string());
        if algorithm.contains("ECDSA") || algorithm.contains("ECDH") {
            risks.push("Key size increase may affect protocols".to_string());
        risks
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
    /// Generate security recommendations
    fn generate_security_recommendations(&self, algorithms: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if algorithms.iter().any(|alg| self.is_quantum_vulnerable(alg)) {
            recommendations.push("Implement post-quantum cryptography to prepare for quantum threats".to_string());
            recommendations.push("Use hybrid modes during transition period".to_string());
        if algorithms.iter().any(|alg| alg.contains("RSA-2048")) {
            recommendations.push("Consider migrating RSA-2048 to RSA-3072 or PQC alternatives".to_string());
        recommendations.push("Implement crypto-agility to enable future algorithm transitions".to_string());
        recommendations.push("Regular security audits of cryptographic implementations".to_string());
        recommendations.push("Monitor NIST post-quantum standardization progress".to_string());
        
        recommendations
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
        ];
        
        if critical_count > 0 {
            milestones.insert(1, "Address critical vulnerabilities".to_string());
        TimelineEstimate {
        }
    }
    
    /// Convert classical key to PQC equivalent
    pub fn convert_to_pqc(&self, classical_key: &AsymmetricKey) -> AdvancedCryptoResult<PqcKey> {
        if let Some(mapping) = self.algorithm_mappings.get(&classical_key.algorithm) {
            // Create equivalent PQC key (simplified conversion)
            let pqc_key_data = self.generate_equivalent_pqc_key(&classical_key.key_data, &mapping.pqc_equivalent)?;
            
            Ok(PqcKey::new(
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
            );
            
            // Create hybrid configuration
            let scheme_type = if mapping.classical_algorithm.contains("RSA") || mapping.classical_algorithm.contains("ECDSA") {
                HybridSchemeType::Signature
            } else {
                HybridSchemeType::Kem
            
            let config = HybridAlgorithmConfig {
            
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
        
        // Simple deterministic generation based on classical key
        let mut pqc_key = Vec::with_capacity(key_size);
        let mut seed = classical_key_data.iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64));
        
        for _ in 0..key_size {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            pqc_key.push((seed >> 24) as u8);
        Ok(pqc_key)
    /// Get algorithm mapping
    pub fn get_mapping(&self, algorithm: &str) -> Option<&AlgorithmMapping> {
        self.algorithm_mappings.get(algorithm)
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
/// fr fr Rule severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleSeverity {
impl RuleSeverity {
    pub fn name(&self) -> &'static str {
        match self {
        }
    }
/// fr fr Compatibility errors
#[derive(Debug, Clone)]
pub enum CompatibilityError {
// impl fmt::Display for CompatibilityError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             CompatibilityError::UnsupportedAlgorithm(msg) => write!(f, "Unsupported algorithm: {}", msg),
//             CompatibilityError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
//             CompatibilityError::MappingError(msg) => write!(f, "Mapping error: {}", msg),
//             CompatibilityError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for CompatibilityError {}
// 
// impl From<CompatibilityError> for CursedError {
//     fn from(err: CompatibilityError) -> Self {
//         CursedError::CryptoError(err.to_string())
//     }
// }

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
