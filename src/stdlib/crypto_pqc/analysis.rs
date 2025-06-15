//! Post-Quantum Cryptography Security Analysis
//! 
//! This module provides tools for analyzing the quantum resistance and security
//! properties of PQC algorithms.

use std::collections::HashMap;
use crate::stdlib::crypto_pqc::{PqcResult, SecurityLevel, AlgorithmType, AlgorithmFamily, StandardizationStatus};

/// Quantum threat assessment
#[derive(Debug, Clone)]
pub struct QuantumThreatAssessment {
    pub cryptographically_relevant_quantum_computer_timeline: QuantumTimeline,
    pub current_threat_level: ThreatLevel,
    pub affected_algorithms: Vec<ClassicalAlgorithm>,
    pub migration_urgency: MigrationUrgency,
}

/// Timeline estimates for quantum computers
#[derive(Debug, Clone)]
pub struct QuantumTimeline {
    pub conservative_estimate_years: u32,
    pub optimistic_estimate_years: u32,
    pub expert_consensus_years: u32,
    pub confidence_level: f64,
}

/// Current quantum threat level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatLevel {
    Negligible,
    Low,
    Moderate,
    High,
    Critical,
}

/// Classical algorithms vulnerable to quantum attacks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassicalAlgorithm {
    Rsa,
    Ecdsa,
    Ecdh,
    Dh,
    Dsa,
}

/// Migration urgency assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationUrgency {
    Immediate,
    HighPriority,
    Moderate,
    LowPriority,
    Planning,
}

/// Algorithm security analysis
#[derive(Debug, Clone)]
pub struct AlgorithmSecurityAnalysis {
    pub algorithm: AlgorithmType,
    pub family: AlgorithmFamily,
    pub standardization_status: StandardizationStatus,
    pub quantum_security_strength: QuantumSecurityStrength,
    pub attack_vectors: Vec<AttackVector>,
    pub implementation_considerations: Vec<ImplementationConsideration>,
    pub confidence_rating: ConfidenceRating,
}

/// Quantum security strength assessment
#[derive(Debug, Clone)]
pub struct QuantumSecurityStrength {
    pub bits_of_security: u32,
    pub classical_equivalent: SecurityLevel,
    pub estimated_break_time_years: Option<u64>,
    pub underlying_problem: String,
}

/// Known attack vectors
#[derive(Debug, Clone)]
pub struct AttackVector {
    pub name: String,
    pub attack_type: AttackType,
    pub complexity: AttackComplexity,
    pub practical_threat: bool,
    pub mitigation: Option<String>,
}

/// Types of cryptographic attacks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    Classical,
    Quantum,
    SideChannel,
    Implementation,
    Cryptanalytic,
}

/// Attack complexity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackComplexity {
    Trivial,
    Low,
    Moderate,
    High,
    Extreme,
}

/// Implementation considerations
#[derive(Debug, Clone)]
pub struct ImplementationConsideration {
    pub category: ImplementationCategory,
    pub description: String,
    pub severity: Severity,
    pub recommendation: String,
}

/// Categories of implementation considerations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplementationCategory {
    KeyManagement,
    Performance,
    MemoryUsage,
    SideChannelResistance,
    ParameterSelection,
    RandomnessRequirements,
}

/// Severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Confidence rating for analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceRating {
    VeryHigh,
    High,
    Medium,
    Low,
    VeryLow,
}

/// Security analyzer for PQC algorithms
pub struct PqcSecurityAnalyzer {
    analyses: HashMap<AlgorithmType, AlgorithmSecurityAnalysis>,
    quantum_threat: QuantumThreatAssessment,
}

impl PqcSecurityAnalyzer {
    /// Create a new security analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            analyses: HashMap::new(),
            quantum_threat: Self::default_quantum_threat_assessment(),
        };

        analyzer.initialize_algorithm_analyses();
        analyzer
    }

    /// Default quantum threat assessment
    fn default_quantum_threat_assessment() -> QuantumThreatAssessment {
        QuantumThreatAssessment {
            cryptographically_relevant_quantum_computer_timeline: QuantumTimeline {
                conservative_estimate_years: 20,
                optimistic_estimate_years: 5,
                expert_consensus_years: 15,
                confidence_level: 0.7,
            },
            current_threat_level: ThreatLevel::Moderate,
            affected_algorithms: vec![
                ClassicalAlgorithm::Rsa,
                ClassicalAlgorithm::Ecdsa,
                ClassicalAlgorithm::Ecdh,
                ClassicalAlgorithm::Dh,
                ClassicalAlgorithm::Dsa,
            ],
            migration_urgency: MigrationUrgency::HighPriority,
        }
    }

    /// Initialize analysis for all algorithms
    fn initialize_algorithm_analyses(&mut self) {
        let algorithms = vec![
            (AlgorithmType::Kyber, self.create_kyber_analysis()),
            (AlgorithmType::Dilithium, self.create_dilithium_analysis()),
            (AlgorithmType::Sphincs, self.create_sphincs_analysis()),
            (AlgorithmType::Ntru, self.create_ntru_analysis()),
            (AlgorithmType::FrodoKem, self.create_frodo_analysis()),
            (AlgorithmType::ClassicMcEliece, self.create_mceliece_analysis()),
            (AlgorithmType::Sike, self.create_sike_analysis()),
        ];

        for (algorithm, analysis) in algorithms {
            self.analyses.insert(algorithm, analysis);
        }
    }

    /// Create Kyber security analysis
    fn create_kyber_analysis(&self) -> AlgorithmSecurityAnalysis {
        AlgorithmSecurityAnalysis {
            algorithm: AlgorithmType::Kyber,
            family: AlgorithmFamily::LatticeBased,
            standardization_status: StandardizationStatus::NistStandardized,
            quantum_security_strength: QuantumSecurityStrength {
                bits_of_security: 128,
                classical_equivalent: SecurityLevel::Level1,
                estimated_break_time_years: None,
                underlying_problem: "Module Learning With Errors (Module-LWE)".to_string(),
            },
            attack_vectors: vec![
                AttackVector {
                    name: "Lattice reduction attacks".to_string(),
                    attack_type: AttackType::Cryptanalytic,
                    complexity: AttackComplexity::Extreme,
                    practical_threat: false,
                    mitigation: Some("Conservative parameter selection".to_string()),
                },
                AttackVector {
                    name: "Side-channel attacks".to_string(),
                    attack_type: AttackType::SideChannel,
                    complexity: AttackComplexity::Moderate,
                    practical_threat: true,
                    mitigation: Some("Constant-time implementation".to_string()),
                },
            ],
            implementation_considerations: vec![
                ImplementationConsideration {
                    category: ImplementationCategory::SideChannelResistance,
                    description: "Requires constant-time operations to prevent timing attacks".to_string(),
                    severity: Severity::High,
                    recommendation: "Use constant-time implementations for all operations".to_string(),
                },
                ImplementationConsideration {
                    category: ImplementationCategory::RandomnessRequirements,
                    description: "Requires high-quality randomness for key generation".to_string(),
                    severity: Severity::Critical,
                    recommendation: "Use cryptographically secure random number generator".to_string(),
                },
            ],
            confidence_rating: ConfidenceRating::VeryHigh,
        }
    }

    /// Create Dilithium security analysis
    fn create_dilithium_analysis(&self) -> AlgorithmSecurityAnalysis {
        AlgorithmSecurityAnalysis {
            algorithm: AlgorithmType::Dilithium,
            family: AlgorithmFamily::LatticeBased,
            standardization_status: StandardizationStatus::NistStandardized,
            quantum_security_strength: QuantumSecurityStrength {
                bits_of_security: 128,
                classical_equivalent: SecurityLevel::Level1,
                estimated_break_time_years: None,
                underlying_problem: "Module Learning With Errors (Module-LWE)".to_string(),
            },
            attack_vectors: vec![
                AttackVector {
                    name: "Lattice reduction attacks".to_string(),
                    attack_type: AttackType::Cryptanalytic,
                    complexity: AttackComplexity::Extreme,
                    practical_threat: false,
                    mitigation: Some("Conservative parameter selection".to_string()),
                },
                AttackVector {
                    name: "Fault attacks on signing".to_string(),
                    attack_type: AttackType::SideChannel,
                    complexity: AttackComplexity::High,
                    practical_threat: true,
                    mitigation: Some("Redundant computation and verification".to_string()),
                },
            ],
            implementation_considerations: vec![
                ImplementationConsideration {
                    category: ImplementationCategory::Performance,
                    description: "Fast signing and verification compared to other PQC signatures".to_string(),
                    severity: Severity::Low,
                    recommendation: "Take advantage of performance characteristics".to_string(),
                },
            ],
            confidence_rating: ConfidenceRating::VeryHigh,
        }
    }

    /// Create SPHINCS+ security analysis
    fn create_sphincs_analysis(&self) -> AlgorithmSecurityAnalysis {
        AlgorithmSecurityAnalysis {
            algorithm: AlgorithmType::Sphincs,
            family: AlgorithmFamily::HashBased,
            standardization_status: StandardizationStatus::NistStandardized,
            quantum_security_strength: QuantumSecurityStrength {
                bits_of_security: 128,
                classical_equivalent: SecurityLevel::Level1,
                estimated_break_time_years: None,
                underlying_problem: "One-way function security of hash functions".to_string(),
            },
            attack_vectors: vec![
                AttackVector {
                    name: "Hash function cryptanalysis".to_string(),
                    attack_type: AttackType::Cryptanalytic,
                    complexity: AttackComplexity::Extreme,
                    practical_threat: false,
                    mitigation: Some("Use well-analyzed hash functions".to_string()),
                },
            ],
            implementation_considerations: vec![
                ImplementationConsideration {
                    category: ImplementationCategory::Performance,
                    description: "Large signature sizes and slower signing".to_string(),
                    severity: Severity::Medium,
                    recommendation: "Consider fast variants for better performance".to_string(),
                },
                ImplementationConsideration {
                    category: ImplementationCategory::KeyManagement,
                    description: "Stateless signatures simplify key management".to_string(),
                    severity: Severity::Low,
                    recommendation: "Advantage over stateful hash-based signatures".to_string(),
                },
            ],
            confidence_rating: ConfidenceRating::VeryHigh,
        }
    }

    /// Create NTRU security analysis
    fn create_ntru_analysis(&self) -> AlgorithmSecurityAnalysis {
        AlgorithmSecurityAnalysis {
            algorithm: AlgorithmType::Ntru,
            family: AlgorithmFamily::LatticeBased,
            standardization_status: StandardizationStatus::NistFinalist,
            quantum_security_strength: QuantumSecurityStrength {
                bits_of_security: 128,
                classical_equivalent: SecurityLevel::Level1,
                estimated_break_time_years: None,
                underlying_problem: "NTRU lattice problem".to_string(),
            },
            attack_vectors: vec![
                AttackVector {
                    name: "Lattice reduction attacks".to_string(),
                    attack_type: AttackType::Cryptanalytic,
                    complexity: AttackComplexity::High,
                    practical_threat: false,
                    mitigation: Some("Conservative parameter selection".to_string()),
                },
            ],
            implementation_considerations: vec![
                ImplementationConsideration {
                    category: ImplementationCategory::Performance,
                    description: "Fast operations with moderate key sizes".to_string(),
                    severity: Severity::Low,
                    recommendation: "Good performance characteristics".to_string(),
                },
            ],
            confidence_rating: ConfidenceRating::High,
        }
    }

    /// Create FrodoKEM security analysis
    fn create_frodo_analysis(&self) -> AlgorithmSecurityAnalysis {
        AlgorithmSecurityAnalysis {
            algorithm: AlgorithmType::FrodoKem,
            family: AlgorithmFamily::LatticeBased,
            standardization_status: StandardizationStatus::Research,
            quantum_security_strength: QuantumSecurityStrength {
                bits_of_security: 128,
                classical_equivalent: SecurityLevel::Level1,
                estimated_break_time_years: None,
                underlying_problem: "Learning With Errors (LWE)".to_string(),
            },
            attack_vectors: vec![
                AttackVector {
                    name: "Lattice reduction attacks".to_string(),
                    attack_type: AttackType::Cryptanalytic,
                    complexity: AttackComplexity::Extreme,
                    practical_threat: false,
                    mitigation: Some("Conservative security analysis".to_string()),
                },
            ],
            implementation_considerations: vec![
                ImplementationConsideration {
                    category: ImplementationCategory::MemoryUsage,
                    description: "Large key sizes compared to other lattice-based schemes".to_string(),
                    severity: Severity::High,
                    recommendation: "Consider memory constraints in deployment".to_string(),
                },
            ],
            confidence_rating: ConfidenceRating::High,
        }
    }

    /// Create Classic McEliece security analysis
    fn create_mceliece_analysis(&self) -> AlgorithmSecurityAnalysis {
        AlgorithmSecurityAnalysis {
            algorithm: AlgorithmType::ClassicMcEliece,
            family: AlgorithmFamily::CodeBased,
            standardization_status: StandardizationStatus::NistAlternate,
            quantum_security_strength: QuantumSecurityStrength {
                bits_of_security: 128,
                classical_equivalent: SecurityLevel::Level1,
                estimated_break_time_years: None,
                underlying_problem: "Syndrome decoding of linear codes".to_string(),
            },
            attack_vectors: vec![
                AttackVector {
                    name: "Information set decoding".to_string(),
                    attack_type: AttackType::Cryptanalytic,
                    complexity: AttackComplexity::Extreme,
                    practical_threat: false,
                    mitigation: Some("Well-understood security reduction".to_string()),
                },
            ],
            implementation_considerations: vec![
                ImplementationConsideration {
                    category: ImplementationCategory::MemoryUsage,
                    description: "Extremely large public keys".to_string(),
                    severity: Severity::Critical,
                    recommendation: "Only suitable for specific use cases with sufficient storage".to_string(),
                },
            ],
            confidence_rating: ConfidenceRating::VeryHigh,
        }
    }

    /// Create SIKE security analysis (deprecated)
    fn create_sike_analysis(&self) -> AlgorithmSecurityAnalysis {
        AlgorithmSecurityAnalysis {
            algorithm: AlgorithmType::Sike,
            family: AlgorithmFamily::IsogenyBased,
            standardization_status: StandardizationStatus::Deprecated,
            quantum_security_strength: QuantumSecurityStrength {
                bits_of_security: 0,
                classical_equivalent: SecurityLevel::Level1,
                estimated_break_time_years: Some(0),
                underlying_problem: "Supersingular isogeny problem (BROKEN)".to_string(),
            },
            attack_vectors: vec![
                AttackVector {
                    name: "Castryck-Decru attack".to_string(),
                    attack_type: AttackType::Cryptanalytic,
                    complexity: AttackComplexity::Low,
                    practical_threat: true,
                    mitigation: None,
                },
            ],
            implementation_considerations: vec![
                ImplementationConsideration {
                    category: ImplementationCategory::KeyManagement,
                    description: "Algorithm is cryptographically broken".to_string(),
                    severity: Severity::Critical,
                    recommendation: "DO NOT USE - migrate immediately to secure alternatives".to_string(),
                },
            ],
            confidence_rating: ConfidenceRating::VeryHigh,
        }
    }

    /// Get security analysis for an algorithm
    pub fn get_analysis(&self, algorithm: AlgorithmType) -> Option<&AlgorithmSecurityAnalysis> {
        self.analyses.get(&algorithm)
    }

    /// Get quantum threat assessment
    pub fn get_quantum_threat_assessment(&self) -> &QuantumThreatAssessment {
        &self.quantum_threat
    }

    /// Get recommendations for a security level
    pub fn get_recommendations(&self, security_level: SecurityLevel) -> Vec<AlgorithmType> {
        self.analyses
            .values()
            .filter(|analysis| {
                analysis.quantum_security_strength.classical_equivalent >= security_level &&
                analysis.standardization_status.is_production_ready() &&
                analysis.confidence_rating >= ConfidenceRating::High
            })
            .map(|analysis| analysis.algorithm)
            .collect()
    }

    /// Generate security report
    pub fn generate_security_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("Post-Quantum Cryptography Security Analysis Report\n");
        report.push_str("=================================================\n\n");

        // Quantum threat assessment
        report.push_str("Quantum Threat Assessment\n");
        report.push_str("-------------------------\n");
        report.push_str(&format!("Current threat level: {:?}\n", self.quantum_threat.current_threat_level));
        report.push_str(&format!("Migration urgency: {:?}\n", self.quantum_threat.migration_urgency));
        report.push_str(&format!("Expert consensus timeline: {} years\n", 
                                 self.quantum_threat.cryptographically_relevant_quantum_computer_timeline.expert_consensus_years));
        report.push_str(&format!("Confidence level: {:.1}%\n\n", 
                                 self.quantum_threat.cryptographically_relevant_quantum_computer_timeline.confidence_level * 100.0));

        // Algorithm analyses
        for (algorithm, analysis) in &self.analyses {
            report.push_str(&format!("Algorithm: {:?}\n", algorithm));
            report.push_str(&format!("Family: {}\n", analysis.family.description()));
            report.push_str(&format!("Standardization: {}\n", analysis.standardization_status.description()));
            report.push_str(&format!("Confidence: {:?}\n", analysis.confidence_rating));
            report.push_str(&format!("Underlying problem: {}\n", analysis.quantum_security_strength.underlying_problem));
            
            if !analysis.attack_vectors.is_empty() {
                report.push_str("Attack vectors:\n");
                for attack in &analysis.attack_vectors {
                    report.push_str(&format!("  - {}: {:?} complexity, practical: {}\n", 
                                             attack.name, attack.complexity, attack.practical_threat));
                }
            }
            
            if !analysis.implementation_considerations.is_empty() {
                report.push_str("Implementation considerations:\n");
                for consideration in &analysis.implementation_considerations {
                    report.push_str(&format!("  - {:?}: {} ({})\n", 
                                             consideration.category, consideration.description, consideration.recommendation));
                }
            }
            
            report.push('\n');
        }

        // Recommendations by security level
        report.push_str("Recommendations by Security Level\n");
        report.push_str("---------------------------------\n");
        for level in [SecurityLevel::Level1, SecurityLevel::Level3, SecurityLevel::Level5] {
            let recommendations = self.get_recommendations(level);
            report.push_str(&format!("{:?}: {:?}\n", level, recommendations));
        }

        report
    }
}

impl Default for PqcSecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Global security analyzer
static mut GLOBAL_SECURITY_ANALYZER: Option<PqcSecurityAnalyzer> = None;
static INIT_SECURITY_ANALYZER: std::sync::Once = std::sync::Once::new();

/// Get the global security analyzer
pub fn global_security_analyzer() -> &'static PqcSecurityAnalyzer {
    unsafe {
        INIT_SECURITY_ANALYZER.call_once(|| {
            GLOBAL_SECURITY_ANALYZER = Some(PqcSecurityAnalyzer::new());
        });
        GLOBAL_SECURITY_ANALYZER.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_analyzer() {
        let analyzer = PqcSecurityAnalyzer::new();
        
        // Test getting analysis
        let kyber_analysis = analyzer.get_analysis(AlgorithmType::Kyber).unwrap();
        assert_eq!(kyber_analysis.algorithm, AlgorithmType::Kyber);
        assert_eq!(kyber_analysis.family, AlgorithmFamily::LatticeBased);
        assert_eq!(kyber_analysis.confidence_rating, ConfidenceRating::VeryHigh);
        
        // Test SIKE is marked as broken
        let sike_analysis = analyzer.get_analysis(AlgorithmType::Sike).unwrap();
        assert_eq!(sike_analysis.standardization_status, StandardizationStatus::Deprecated);
        assert_eq!(sike_analysis.quantum_security_strength.bits_of_security, 0);
        
        // Test quantum threat assessment
        let threat = analyzer.get_quantum_threat_assessment();
        assert_eq!(threat.current_threat_level, ThreatLevel::Moderate);
        assert!(!threat.affected_algorithms.is_empty());
        
        // Test recommendations
        let level1_recommendations = analyzer.get_recommendations(SecurityLevel::Level1);
        assert!(!level1_recommendations.is_empty());
        assert!(!level1_recommendations.contains(&AlgorithmType::Sike)); // Should not recommend broken algorithm
    }

    #[test]
    fn test_global_analyzer() {
        let analyzer = global_security_analyzer();
        assert!(analyzer.get_analysis(AlgorithmType::Kyber).is_some());
    }

    #[test]
    fn test_security_report_generation() {
        let analyzer = PqcSecurityAnalyzer::new();
        let report = analyzer.generate_security_report();
        
        assert!(report.contains("Post-Quantum Cryptography Security Analysis Report"));
        assert!(report.contains("Quantum Threat Assessment"));
        assert!(report.contains("Kyber"));
        assert!(report.contains("Dilithium"));
        assert!(report.contains("SPHINCS"));
    }
}
