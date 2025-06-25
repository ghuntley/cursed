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
    policies: Arc<RwLock<Vec<SecurityPolicy>>>,
    algorithm_registry: Arc<RwLock<AlgorithmRegistry>>,
    performance_monitor: Arc<Mutex<PerformanceMonitor>>,
    threat_assessor: Arc<Mutex<ThreatAssessment>>,
    config: AgilityConfig,
}

/// Configuration for algorithm agility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgilityConfig {
    /// Enable automatic algorithm switching
    pub auto_switching_enabled: bool,
    /// Performance monitoring interval
    pub monitoring_interval: Duration,
    /// Threat assessment update frequency
    pub threat_update_frequency: Duration,
    /// Maximum allowed performance degradation (percentage)
    pub max_performance_degradation: f64,
    /// Enable algorithm deprecation warnings
    pub deprecation_warnings: bool,
    /// Compatibility mode for legacy systems
    pub legacy_compatibility: bool,
}

impl Default for AgilityConfig {
    fn default() -> Self {
        Self {
            auto_switching_enabled: true,
            monitoring_interval: Duration::from_secs(300), // 5 minutes
            threat_update_frequency: Duration::from_secs(3600), // 1 hour
            max_performance_degradation: 20.0, // 20%
            deprecation_warnings: true,
            legacy_compatibility: true,
        }
    }
}

/// Security policy for algorithm selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub priority: PolicyPriority,
    pub conditions: Vec<PolicyCondition>,
    pub actions: Vec<PolicyAction>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub enabled: bool,
}

/// Policy priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PolicyPriority {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
    Information = 0,
}

/// Policy condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyCondition {
    /// Security level requirement
    MinSecurityLevel(SecurityLevel),
    /// Maximum performance impact allowed
    MaxPerformanceImpact(f64),
    /// Threat level threshold
    ThreatLevel(ThreatLevel),
    /// Algorithm deprecation status
    AlgorithmStatus(StandardizationStatus),
    /// Time-based condition
    TimeRange { start: u64, end: u64 },
    /// Context-based condition
    ContextMatch(String),
}

/// Policy action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    /// Prefer specific algorithm
    PreferAlgorithm(AlgorithmType),
    /// Avoid specific algorithm
    AvoidAlgorithm(AlgorithmType),
    /// Require hybrid mode
    RequireHybrid,
    /// Set minimum security level
    EnforceSecurityLevel(SecurityLevel),
    /// Generate warning
    GenerateWarning(String),
    /// Force algorithm migration
    ForceMigration { from: AlgorithmType, to: AlgorithmType },
}

/// Algorithm registry for tracking available algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmRegistry {
    algorithms: HashMap<AlgorithmType, AlgorithmInfo>,
    last_updated: u64,
}

/// Information about a cryptographic algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmInfo {
    pub algorithm_type: AlgorithmType,
    pub family: AlgorithmFamily,
    pub standardization_status: StandardizationStatus,
    pub security_levels: Vec<SecurityLevel>,
    pub performance_characteristics: PerformanceCharacteristics,
    pub quantum_resistance: QuantumResistance,
    pub implementation_status: ImplementationStatus,
    pub deprecation_info: Option<DeprecationInfo>,
}

/// Performance characteristics of an algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCharacteristics {
    pub key_generation_time: Duration,
    pub operation_time: Duration, // Encrypt/sign time
    pub verification_time: Duration,
    pub key_size: usize,
    pub signature_size: Option<usize>,
    pub ciphertext_overhead: usize,
    pub memory_usage: usize,
}

/// Quantum resistance assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResistance {
    pub level: QuantumResistanceLevel,
    pub confidence: f64, // 0.0 to 1.0
    pub research_status: ResearchStatus,
    pub estimated_break_time: Option<u64>, // Years until quantum computer could break it
}

/// Quantum resistance levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantumResistanceLevel {
    None,       // Broken by quantum computers
    Weak,       // Some quantum resistance
    Moderate,   // Good quantum resistance
    Strong,     // Excellent quantum resistance
    Proven,     // Mathematically proven quantum resistance
}

/// Research and development status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchStatus {
    Experimental,
    InDevelopment,
    UnderReview,
    Standardized,
    Deprecated,
}

/// Implementation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplementationStatus {
    NotImplemented,
    Placeholder,
    PartialImplementation,
    FullImplementation,
    OptimizedImplementation,
}

/// Algorithm deprecation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecationInfo {
    pub reason: String,
    pub deprecated_at: u64,
    pub end_of_life: Option<u64>,
    pub replacement_algorithms: Vec<AlgorithmType>,
    pub migration_guidance: String,
}

/// Performance monitoring system
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    metrics: HashMap<AlgorithmType, PerformanceMetrics>,
    benchmarks: HashMap<AlgorithmType, BenchmarkResults>,
    last_benchmark: u64,
}

/// Performance metrics for an algorithm
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub operations_count: u64,
    pub total_time: Duration,
    pub average_time: Duration,
    pub success_rate: f64,
    pub error_count: u64,
    pub last_updated: Instant,
}

/// Benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub operations_per_second: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub benchmark_date: u64,
    pub test_conditions: String,
}

/// Threat assessment system
#[derive(Debug, Clone)]
pub struct ThreatAssessment {
    current_threat_level: ThreatLevel,
    threat_indicators: Vec<ThreatIndicator>,
    last_assessment: u64,
    quantum_computer_status: QuantumComputerStatus,
}

/// Threat levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low = 1,
    Moderate = 2,
    High = 3,
    Critical = 4,
    Emergency = 5,
}

/// Threat indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub indicator_type: ThreatIndicatorType,
    pub severity: ThreatLevel,
    pub description: String,
    pub detected_at: u64,
    pub expires_at: Option<u64>,
}

/// Types of threat indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatIndicatorType {
    QuantumAdvancement,
    AlgorithmBreak,
    StandardUpdate,
    SecurityVulnerability,
    PerformanceRegression,
    ComplianceChange,
}

/// Current status of quantum computer development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumComputerStatus {
    pub estimated_qubits_available: u32,
    pub error_correction_available: bool,
    pub estimated_years_to_cryptographic_relevance: u32,
    pub last_updated: u64,
}

/// Algorithm selection result
#[derive(Debug, Clone)]
pub struct AlgorithmSelection {
    pub selected_algorithm: AlgorithmType,
    pub selection_reason: String,
    pub confidence_score: f64,
    pub alternatives: Vec<AlgorithmType>,
    pub warnings: Vec<String>,
    pub hybrid_recommendation: Option<HybridRecommendation>,
}

/// Hybrid algorithm recommendation
#[derive(Debug, Clone)]
pub struct HybridRecommendation {
    pub classical_algorithm: ClassicalAlgorithm,
    pub pqc_algorithm: AlgorithmType,
    pub reason: String,
    pub migration_timeline: Option<Duration>,
}

impl AlgorithmAgilityManager {
    /// Create a new algorithm agility manager
    pub fn new() -> Self {
        Self::new_with_config(AgilityConfig::default())
    }

    /// Create a new algorithm agility manager with custom configuration
    #[instrument(skip(config))]
    pub fn new_with_config(config: AgilityConfig) -> Self {
        info!("Creating algorithm agility manager");

        let policies = Arc::new(RwLock::new(Self::default_policies()));
        let algorithm_registry = Arc::new(RwLock::new(Self::build_algorithm_registry()));
        let performance_monitor = Arc::new(Mutex::new(PerformanceMonitor {
            metrics: HashMap::new(),
            benchmarks: HashMap::new(),
            last_benchmark: 0,
        }));
        let threat_assessor = Arc::new(Mutex::new(ThreatAssessment {
            current_threat_level: ThreatLevel::Low,
            threat_indicators: Vec::new(),
            last_assessment: 0,
            quantum_computer_status: Self::default_quantum_status(),
        }));

        Self {
            policies,
            algorithm_registry,
            performance_monitor,
            threat_assessor,
            config,
        }
    }

    /// Select the best algorithm for a given context
    #[instrument(skip(self))]
    pub fn select_algorithm(
        &self,
        context: &SelectionContext,
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
                selected_algorithm: algorithm,
                selection_reason: format!("Selected based on policy evaluation (score: {:.2})", score),
                confidence_score: score,
                alternatives,
                warnings,
                hybrid_recommendation,
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
                indicator_type = ?indicator.indicator_type,
                severity = ?indicator.severity,
                "Adding threat indicator"
            );
            assessor.threat_indicators.push(indicator);
        }

        // Update overall threat level
        let max_severity = assessor.threat_indicators
            .iter()
            .map(|i| i.severity)
            .max()
            .unwrap_or(ThreatLevel::Low);

        if max_severity != assessor.current_threat_level {
            warn!(
                old_level = ?assessor.current_threat_level,
                new_level = ?max_severity,
                "Threat level updated"
            );
            assessor.current_threat_level = max_severity;
        }

        assessor.last_assessment = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(())
    }

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
    }

    /// Add a custom security policy
    pub fn add_policy(&self, policy: SecurityPolicy) -> PqcResult<()> {
        let mut policies = self.policies.write()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock policies: {}", e)))?;

        info!(policy_id = %policy.id, policy_name = %policy.name, "Adding security policy");
        policies.push(policy);
        policies.sort_by(|a, b| b.priority.cmp(&a.priority)); // Sort by priority descending

        Ok(())
    }

    /// Default security policies
    fn default_policies() -> Vec<SecurityPolicy> {
        vec![
            SecurityPolicy {
                id: "deprecated_algorithms".to_string(),
                name: "Avoid Deprecated Algorithms".to_string(),
                description: "Avoid using algorithms that have been deprecated or broken".to_string(),
                priority: PolicyPriority::Critical,
                conditions: vec![
                    PolicyCondition::AlgorithmStatus(StandardizationStatus::Deprecated),
                ],
                actions: vec![
                    PolicyAction::GenerateWarning("Algorithm is deprecated and should not be used".to_string()),
                ],
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                expires_at: None,
                enabled: true,
            },
            SecurityPolicy {
                id: "high_security_requirement".to_string(),
                name: "High Security Requirements".to_string(),
                description: "Use quantum-resistant algorithms for high security contexts".to_string(),
                priority: PolicyPriority::High,
                conditions: vec![
                    PolicyCondition::MinSecurityLevel(SecurityLevel::Level5),
                ],
                actions: vec![
                    PolicyAction::PreferAlgorithm(AlgorithmType::Kyber),
                    PolicyAction::RequireHybrid,
                ],
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                expires_at: None,
                enabled: true,
            },
            SecurityPolicy {
                id: "performance_sensitive".to_string(),
                name: "Performance Sensitive Applications".to_string(),
                description: "Balance security and performance for performance-critical applications".to_string(),
                priority: PolicyPriority::Medium,
                conditions: vec![
                    PolicyCondition::MaxPerformanceImpact(10.0),
                ],
                actions: vec![
                    PolicyAction::PreferAlgorithm(AlgorithmType::Kyber),
                    PolicyAction::AvoidAlgorithm(AlgorithmType::Sphincs), // Large signatures
                ],
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                expires_at: None,
                enabled: true,
            },
        ]
    }

    /// Build the algorithm registry with current information
    fn build_algorithm_registry() -> AlgorithmRegistry {
        let mut algorithms = HashMap::new();

        // Kyber
        algorithms.insert(AlgorithmType::Kyber, AlgorithmInfo {
            algorithm_type: AlgorithmType::Kyber,
            family: AlgorithmFamily::LatticeBased,
            standardization_status: StandardizationStatus::NistStandardized,
            security_levels: vec![SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5],
            performance_characteristics: PerformanceCharacteristics {
                key_generation_time: Duration::from_millis(1),
                operation_time: Duration::from_millis(1),
                verification_time: Duration::from_millis(1),
                key_size: 1568, // Kyber1024 public key
                signature_size: None, // KEM, not signature
                ciphertext_overhead: 1568, // Kyber1024 ciphertext
                memory_usage: 4096,
            },
            quantum_resistance: QuantumResistance {
                level: QuantumResistanceLevel::Strong,
                confidence: 0.9,
                research_status: ResearchStatus::Standardized,
                estimated_break_time: None,
            },
            implementation_status: ImplementationStatus::FullImplementation,
            deprecation_info: None,
        });

        // Dilithium
        algorithms.insert(AlgorithmType::Dilithium, AlgorithmInfo {
            algorithm_type: AlgorithmType::Dilithium,
            family: AlgorithmFamily::LatticeBased,
            standardization_status: StandardizationStatus::NistStandardized,
            security_levels: vec![SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5],
            performance_characteristics: PerformanceCharacteristics {
                key_generation_time: Duration::from_millis(2),
                operation_time: Duration::from_millis(3),
                verification_time: Duration::from_millis(1),
                key_size: 2592, // Dilithium5 public key
                signature_size: Some(4595), // Dilithium5 signature
                ciphertext_overhead: 0,
                memory_usage: 8192,
            },
            quantum_resistance: QuantumResistance {
                level: QuantumResistanceLevel::Strong,
                confidence: 0.9,
                research_status: ResearchStatus::Standardized,
                estimated_break_time: None,
            },
            implementation_status: ImplementationStatus::FullImplementation,
            deprecation_info: None,
        });

        // SPHINCS+
        algorithms.insert(AlgorithmType::Sphincs, AlgorithmInfo {
            algorithm_type: AlgorithmType::Sphincs,
            family: AlgorithmFamily::HashBased,
            standardization_status: StandardizationStatus::NistStandardized,
            security_levels: vec![SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5],
            performance_characteristics: PerformanceCharacteristics {
                key_generation_time: Duration::from_millis(10),
                operation_time: Duration::from_millis(50), // Slower signing
                verification_time: Duration::from_millis(5),
                key_size: 64,
                signature_size: Some(29792), // Large signatures
                ciphertext_overhead: 0,
                memory_usage: 16384,
            },
            quantum_resistance: QuantumResistance {
                level: QuantumResistanceLevel::Proven,
                confidence: 1.0,
                research_status: ResearchStatus::Standardized,
                estimated_break_time: None,
            },
            implementation_status: ImplementationStatus::Placeholder,
            deprecation_info: None,
        });

        // Add other algorithms...
        
        AlgorithmRegistry {
            algorithms,
            last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    /// Default quantum computer status
    fn default_quantum_status() -> QuantumComputerStatus {
        QuantumComputerStatus {
            estimated_qubits_available: 1000, // Current estimate
            error_correction_available: false,
            estimated_years_to_cryptographic_relevance: 10, // Conservative estimate
            last_updated: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
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
    }

    fn policy_applies(&self, policy: &SecurityPolicy, context: &SelectionContext) -> bool {
        for condition in &policy.conditions {
            match condition {
                PolicyCondition::MinSecurityLevel(level) => {
                    if let Some(required_level) = context.required_security_level {
                        if required_level < *level {
                            return false;
                        }
                    }
                },
                PolicyCondition::MaxPerformanceImpact(max_impact) => {
                    if context.performance_priority && context.max_performance_impact.unwrap_or(100.0) < *max_impact {
                        return false;
                    }
                },
                PolicyCondition::ThreatLevel(level) => {
                    if let Ok(assessor) = self.threat_assessor.lock() {
                        if assessor.current_threat_level < *level {
                            return false;
                        }
                    }
                },
                _ => {} // Other conditions not implemented yet
            }
        }
        true
    }

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
            }

            // Filter by standardization status if required
            if context.require_standardized && !info.standardization_status.is_production_ready() {
                continue;
            }

            candidates.push(*algorithm_type);
        }

        Ok(candidates)
    }

    fn score_algorithms(
        &self,
        candidates: &[AlgorithmType],
        policies: &[SecurityPolicy],
        context: &SelectionContext,
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
                        },
                        PolicyAction::AvoidAlgorithm(alg) if *alg == algorithm => {
                            score -= 0.5 * (policy.priority as u8 as f64 / 4.0);
                        },
                        _ => {}
                    }
                }
            }

            // Performance scoring
            if context.performance_priority {
                score += self.calculate_performance_score(algorithm)?;
            }

            // Security scoring
            score += self.calculate_security_score(algorithm, context)?;

            scored.push((algorithm, score));
        }

        // Sort by score descending
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        Ok(scored)
    }

    fn calculate_performance_score(&self, algorithm: AlgorithmType) -> PqcResult<f64> {
        // Simple performance scoring based on algorithm characteristics
        match algorithm {
            AlgorithmType::Kyber => Ok(0.2),           // Fast KEM
            AlgorithmType::Dilithium => Ok(0.15),      // Good signature performance
            AlgorithmType::Sphincs => Ok(-0.1),        // Slow signing, large signatures
            _ => Ok(0.0),
        }
    }

    fn calculate_security_score(&self, algorithm: AlgorithmType, context: &SelectionContext) -> PqcResult<f64> {
        let registry = self.algorithm_registry.read()
            .map_err(|e| PqcError::InternalError(format!("Failed to lock registry: {}", e)))?;

        if let Some(info) = registry.algorithms.get(&algorithm) {
            let mut score = match info.quantum_resistance.level {
                QuantumResistanceLevel::Proven => 0.3,
                QuantumResistanceLevel::Strong => 0.2,
                QuantumResistanceLevel::Moderate => 0.1,
                QuantumResistanceLevel::Weak => 0.0,
                QuantumResistanceLevel::None => -0.5,
            };

            // Boost score for standardized algorithms
            if info.standardization_status.is_production_ready() {
                score += 0.1;
            }

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
                    "Warning: Algorithm {:?} is deprecated. Reason: {}. Consider migrating to: {:?}",
                    algorithm, deprecation.reason, deprecation.replacement_algorithms
                )));
            }
        }

        Ok(None)
    }

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
        }

        Ok(alternatives)
    }

    fn generate_hybrid_recommendation(&self, algorithm: AlgorithmType, context: &SelectionContext) -> PqcResult<Option<HybridRecommendation>> {
        // For high security contexts, recommend hybrid approach
        if let Some(SecurityLevel::Level5) = context.required_security_level {
            return Ok(Some(HybridRecommendation {
                classical_algorithm: ClassicalAlgorithm::X25519,
                pqc_algorithm: algorithm,
                reason: "High security level requires hybrid approach for defense in depth".to_string(),
                migration_timeline: Some(Duration::from_secs(86400 * 30)), // 30 days
            }));
        }

        Ok(None)
    }
}

/// Context for algorithm selection
#[derive(Debug, Clone)]
pub struct SelectionContext {
    /// Required security level
    pub required_security_level: Option<SecurityLevel>,
    /// Whether performance is a priority
    pub performance_priority: bool,
    /// Maximum acceptable performance impact (percentage)
    pub max_performance_impact: Option<f64>,
    /// Whether algorithm must be standardized
    pub require_standardized: bool,
    /// Application context identifier
    pub application_context: Option<String>,
    /// Expected lifetime of data being protected
    pub data_lifetime: Option<Duration>,
    /// Whether hybrid mode is acceptable
    pub hybrid_acceptable: bool,
}

impl Default for SelectionContext {
    fn default() -> Self {
        Self {
            required_security_level: Some(SecurityLevel::Level3),
            performance_priority: false,
            max_performance_impact: None,
            require_standardized: true,
            application_context: None,
            data_lifetime: None,
            hybrid_acceptable: true,
        }
    }
}

impl Default for AlgorithmAgilityManager {
    fn default() -> Self {
        Self::new()
    }
}

