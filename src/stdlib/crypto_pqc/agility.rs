use crate::error::CursedError;
/// Algorithm Agility Framework for Post-Quantum Cryptography
/// 
/// This module provides a framework for algorithm agility, allowing systems to
/// dynamically switch between different cryptographic algorithms based on 
/// security requirements, performance constraints, and threat assessments.
/// 
/// # Key Features
/// 
/// - Dynamic algorithm selection based on security policies
/// - Performance-aware algorithm switching
/// - Threat-responsive algorithm updates
/// - Backward compatibility management
/// - Algorithm deprecation and migration support

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error, debug, instrument};

// use crate::stdlib::crypto_pqc::{PqcResult, PqcError, SecurityLevel, AlgorithmType, AlgorithmFamily, StandardizationStatus};
// use crate::stdlib::crypto_pqc::hybrid::{ClassicalAlgorithm, ClassicalSignatureAlgorithm, HybridConfig};

/// Algorithm agility manager for dynamic cryptographic algorithm selection
#[derive(Debug, Clone)]
pub struct AlgorithmAgilityManager {
/// Configuration for algorithm agility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgilityConfig {
    /// Enable automatic algorithm switching
    /// Performance monitoring interval
    /// Threat assessment update frequency
    /// Maximum allowed performance degradation (percentage)
    /// Enable algorithm deprecation warnings
    /// Compatibility mode for legacy systems
impl Default for AgilityConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: Duration::from_secs(300), // 5 minutes
            threat_update_frequency: Duration::from_secs(3600), // 1 hour
            max_performance_degradation: 20.0, // 20%
        }
    }
/// Security policy for algorithm selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
/// Policy priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PolicyPriority {
/// Policy condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyCondition {
    /// Security level requirement
    /// Maximum performance impact allowed
    /// Threat level threshold
    /// Algorithm deprecation status
    /// Time-based condition
    /// Context-based condition
/// Policy action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    /// Prefer specific algorithm
    /// Avoid specific algorithm
    /// Require hybrid mode
    /// Set minimum security level
    /// Generate warning
    /// Force algorithm migration
/// Algorithm registry for tracking available algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmRegistry {
/// Information about a cryptographic algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmInfo {
/// Performance characteristics of an algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCharacteristics {
    pub operation_time: Duration, // Encrypt/sign time
/// Quantum resistance assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResistance {
    pub confidence: f64, // 0.0 to 1.0
    pub estimated_break_time: Option<u64>, // Years until quantum computer could break it
/// Quantum resistance levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantumResistanceLevel {
    None,       // Broken by quantum computers
    Weak,       // Some quantum resistance
    Moderate,   // Good quantum resistance
    Strong,     // Excellent quantum resistance
    Proven,     // Mathematically proven quantum resistance
/// Research and development status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchStatus {
/// Implementation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplementationStatus {
/// Algorithm deprecation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecationInfo {
/// Performance monitoring system
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
/// Performance metrics for an algorithm
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
/// Benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
/// Threat assessment system
#[derive(Debug, Clone)]
pub struct ThreatAssessment {
/// Threat levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
/// Threat indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
/// Types of threat indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatIndicatorType {
/// Current status of quantum computer development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumComputerStatus {
/// Algorithm selection result
#[derive(Debug, Clone)]
pub struct AlgorithmSelection {
/// Hybrid algorithm recommendation
#[derive(Debug, Clone)]
pub struct HybridRecommendation {
impl AlgorithmAgilityManager {
    /// Create a new algorithm agility manager
    pub fn new() -> Self {
        Self::new_with_config(AgilityConfig::default())
    /// Create a new algorithm agility manager with custom configuration
    #[instrument(skip(config))]
    pub fn new_with_config(config: AgilityConfig) -> Self {
        info!("Creating algorithm agility manager");

        let policies = Arc::new(RwLock::new(Self::default_policies()));
        let algorithm_registry = Arc::new(RwLock::new(Self::build_algorithm_registry()));
        let performance_monitor = Arc::new(Mutex::new(PerformanceMonitor {
        }));
        let threat_assessor = Arc::new(Mutex::new(ThreatAssessment {
        }));

        Self {
        }
    }

    /// Select the best algorithm for a given context
    #[instrument(skip(self))]
    pub fn select_algorithm(
    ) -> PqcResult<AlgorithmSelection> {
        debug!(context = ?context, "Selecting algorithm for context");

        // Evaluate policies
        let applicable_policies = self.evaluate_policies(context)?;
        
        // Get algorithm candidates
        let candidates = self.get_algorithm_candidates(context)?;
        
        // Score algorithms based on policies and context
        let scored_algorithms = self.score_algorithms(&candidates, &applicable_policies, context)?;
        
        // Select best algorithm
        if let Some((algorithm, score)) = scored_algorithms.into_iter().next() {
            let mut warnings = Vec::new();
            let mut alternatives = Vec::new();

            // Check for deprecation warnings
            if self.config.deprecation_warnings {
                if let Some(warning) = self.check_deprecation_warning(algorithm)? {
                    warnings.push(warning);
                }
            }

            // Provide alternatives
            alternatives = self.get_alternatives(algorithm, context)?;

            // Generate hybrid recommendation if appropriate
            let hybrid_recommendation = self.generate_hybrid_recommendation(algorithm, context)?;

            Ok(AlgorithmSelection {
            })
        } else {
            Err(PqcError::AlgorithmNotAvailable("No suitable algorithm found for context".to_string()))
        }
    }

    /// Update threat assessment
    #[instrument(skip(self))]
    pub fn update_threat_assessment(&self, indicators: Vec<ThreatIndicator>) -> PqcResult<()> {
        let mut assessor = self.threat_assessor.lock()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock threat assessor: {}", e)))?;

        // Add new indicators
        for indicator in indicators {
            info!(
                "Adding threat indicator"
            );
            assessor.threat_indicators.push(indicator);
        // Update overall threat level
        let max_severity = assessor.threat_indicators
            .iter()
            .map(|i| i.severity)
            .max()
            .unwrap_or(ThreatLevel::Low);

        if max_severity != assessor.current_threat_level {
            warn!(
                "Threat level updated"
            );
            assessor.current_threat_level = max_severity;
        assessor.last_assessment = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(())
    /// Get current algorithm recommendations
    pub fn get_recommendations(&self, context: &SelectionContext) -> PqcResult<Vec<AlgorithmSelection>> {
        let mut recommendations = Vec::new();

        // Get recommendations for different security levels
        for security_level in &[SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let mut context_copy = context.clone();
            context_copy.required_security_level = Some(*security_level);
            
            if let Ok(selection) = self.select_algorithm(&context_copy) {
                recommendations.push(selection);
            }
        }

        Ok(recommendations)
    /// Add a custom security policy
    pub fn add_policy(&self, policy: SecurityPolicy) -> PqcResult<()> {
        let mut policies = self.policies.write()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock policies: {}", e)))?;

        info!(policy_id = %policy.id, policy_name = %policy.name, "Adding security policy");
        policies.push(policy);
        policies.sort_by(|a, b| b.priority.cmp(&a.priority)); // Sort by priority descending

        Ok(())
    /// Default security policies
    fn default_policies() -> Vec<SecurityPolicy> {
        vec![
            SecurityPolicy {
                conditions: vec![
                actions: vec![
            SecurityPolicy {
                conditions: vec![
                actions: vec![
            SecurityPolicy {
                conditions: vec![
                actions: vec![
                    PolicyAction::AvoidAlgorithm(AlgorithmType::Sphincs), // Large signatures
        ]
    /// Build the algorithm registry with current information
    fn build_algorithm_registry() -> AlgorithmRegistry {
        let mut algorithms = HashMap::new();

        // Kyber
        algorithms.insert(AlgorithmType::Kyber, AlgorithmInfo {
            performance_characteristics: PerformanceCharacteristics {
                key_size: 1568, // Kyber1024 public key
                signature_size: None, // KEM, not signature
                ciphertext_overhead: 1568, // Kyber1024 ciphertext
            quantum_resistance: QuantumResistance {
        });

        // Dilithium
        algorithms.insert(AlgorithmType::Dilithium, AlgorithmInfo {
            performance_characteristics: PerformanceCharacteristics {
                key_size: 2592, // Dilithium5 public key
                signature_size: Some(4595), // Dilithium5 signature
            quantum_resistance: QuantumResistance {
        });

        // SPHINCS+
        algorithms.insert(AlgorithmType::Sphincs, AlgorithmInfo {
            performance_characteristics: PerformanceCharacteristics {
                operation_time: Duration::from_millis(50), // Slower signing
                signature_size: Some(29792), // Large signatures
            quantum_resistance: QuantumResistance {
        });

        // Add other algorithms...
        
        AlgorithmRegistry {
        }
    }

    /// Default quantum computer status
    fn default_quantum_status() -> QuantumComputerStatus {
        QuantumComputerStatus {
            estimated_qubits_available: 1000, // Current estimate
            estimated_years_to_cryptographic_relevance: 10, // Conservative estimate
        }
    }

    // Helper methods for algorithm selection
    fn evaluate_policies(&self, context: &SelectionContext) -> PqcResult<Vec<SecurityPolicy>> {
        let policies = self.policies.read()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock policies: {}", e)))?;

        let mut applicable_policies = Vec::new();
        for policy in policies.iter() {
            if policy.enabled && self.policy_applies(policy, context) {
                applicable_policies.push(policy.clone());
            }
        }

        Ok(applicable_policies)
    fn policy_applies(&self, policy: &SecurityPolicy, context: &SelectionContext) -> bool {
        for condition in &policy.conditions {
            match condition {
                PolicyCondition::MinSecurityLevel(level) => {
                    if let Some(required_level) = context.required_security_level {
                        if required_level < *level {
                            return false;
                        }
                    }
                PolicyCondition::MaxPerformanceImpact(max_impact) => {
                    if context.performance_priority && context.max_performance_impact.unwrap_or(100.0) < *max_impact {
                        return false;
                    }
                PolicyCondition::ThreatLevel(level) => {
                    if let Ok(assessor) = self.threat_assessor.lock() {
                        if assessor.current_threat_level < *level {
                            return false;
                        }
                    }
                _ => {} // Other conditions not implemented yet
            }
        }
        true
    fn get_algorithm_candidates(&self, context: &SelectionContext) -> PqcResult<Vec<AlgorithmType>> {
        let registry = self.algorithm_registry.read()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock registry: {}", e)))?;

        let mut candidates = Vec::new();
        for (algorithm_type, info) in &registry.algorithms {
            // Filter by security level
            if let Some(required_level) = context.required_security_level {
                if !info.security_levels.contains(&required_level) {
                    continue;
                }
            }

            // Filter by implementation status
            if info.implementation_status == ImplementationStatus::NotImplemented {
                continue;
            // Filter by standardization status if required
            if context.require_standardized && !info.standardization_status.is_production_ready() {
                continue;
            candidates.push(*algorithm_type);
        Ok(candidates)
    fn score_algorithms(
    ) -> PqcResult<Vec<(AlgorithmType, f64)>> {
        let mut scored = Vec::new();

        for &algorithm in candidates {
            let mut score = 0.5; // Base score

            // Apply policy scores
            for policy in policies {
                for action in &policy.actions {
                    match action {
                        PolicyAction::PreferAlgorithm(alg) if *alg == algorithm => {
                            score += 0.3 * (policy.priority as u8 as f64 / 4.0);
                        PolicyAction::AvoidAlgorithm(alg) if *alg == algorithm => {
                            score -= 0.5 * (policy.priority as u8 as f64 / 4.0);
                        _ => {}
                    }
                }
            }

            // Performance scoring
            if context.performance_priority {
                score += self.calculate_performance_score(algorithm)?;
            // Security scoring
            score += self.calculate_security_score(algorithm, context)?;

            scored.push((algorithm, score));
        // Sort by score descending
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Ok(scored)
    fn calculate_performance_score(&self, algorithm: AlgorithmType) -> PqcResult<f64> {
        // Simple performance scoring based on algorithm characteristics
        match algorithm {
            AlgorithmType::Kyber => Ok(0.2),           // Fast KEM
            AlgorithmType::Dilithium => Ok(0.15),      // Good signature performance
            AlgorithmType::Sphincs => Ok(-0.1),        // Slow signing, large signatures
        }
    }

    fn calculate_security_score(&self, algorithm: AlgorithmType, context: &SelectionContext) -> PqcResult<f64> {
        let registry = self.algorithm_registry.read()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock registry: {}", e)))?;

        if let Some(info) = registry.algorithms.get(&algorithm) {
            let mut score = match info.quantum_resistance.level {

            // Boost score for standardized algorithms
            if info.standardization_status.is_production_ready() {
                score += 0.1;
            Ok(score)
        } else {
            Ok(0.0)
        }
    }

    fn check_deprecation_warning(&self, algorithm: AlgorithmType) -> PqcResult<Option<String>> {
        let registry = self.algorithm_registry.read()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock registry: {}", e)))?;

        if let Some(info) = registry.algorithms.get(&algorithm) {
            if let Some(deprecation) = &info.deprecation_info {
                return Ok(Some(format!(
                    algorithm, deprecation.reason, deprecation.replacement_algorithms
                )));
            }
        }

        Ok(None)
    fn get_alternatives(&self, algorithm: AlgorithmType, context: &SelectionContext) -> PqcResult<Vec<AlgorithmType>> {
        // Get algorithms from the same family
        let registry = self.algorithm_registry.read()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock registry: {}", e)))?;

        let mut alternatives = Vec::new();
        if let Some(info) = registry.algorithms.get(&algorithm) {
            for (other_alg, other_info) in &registry.algorithms {
                if *other_alg != algorithm && other_info.family == info.family {
                    alternatives.push(*other_alg);
                }
            }
        Ok(alternatives)
    fn generate_hybrid_recommendation(&self, algorithm: AlgorithmType, context: &SelectionContext) -> PqcResult<Option<HybridRecommendation>> {
        // For high security contexts, recommend hybrid approach
        if let Some(SecurityLevel::Level5) = context.required_security_level {
            return Ok(Some(HybridRecommendation {
                migration_timeline: Some(Duration::from_secs(86400 * 30)), // 30 days
            }));
        Ok(None)
    }
}

/// Context for algorithm selection
#[derive(Debug, Clone)]
pub struct SelectionContext {
    /// Required security level
    /// Whether performance is a priority
    /// Maximum acceptable performance impact (percentage)
    /// Whether algorithm must be standardized
    /// Application context identifier
    /// Expected lifetime of data being protected
    /// Whether hybrid mode is acceptable
impl Default for SelectionContext {
    fn default() -> Self {
        Self {
        }
    }
impl Default for AlgorithmAgilityManager {
    fn default() -> Self {
        Self::new()
    }
}

