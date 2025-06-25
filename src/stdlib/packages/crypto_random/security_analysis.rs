/// Security analysis and quality assessment for cryptographic random number generation
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use super::randomness_tests::{RandomnessTestSuite, TestResult, TestSuiteConfig};
use super::entropy_monitoring::{EntropyMonitor, EntropyQualityMetrics, AlertLevel};

/// Security assessment level
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Weak,        // Poor randomness quality
    Moderate,    // Acceptable for non-critical use
    Strong,      // Good for most cryptographic uses
    Excellent,   // Highest quality randomness
    Unknown,     // Insufficient data for assessment
}

/// Threat model for random number generation
#[derive(Debug)]
pub enum ThreatModel {
    PassiveObserver,     // Attacker can observe outputs
    ActiveAdversary,     // Attacker can influence inputs
    SideChannelAttack,   // Timing/power analysis attacks
    StateCompromise,     // Internal state exposure
    PredictionAttack,    // Attempting to predict future outputs
}

/// Security analysis configuration
#[derive(Debug, Clone)]
pub struct SecurityAnalysisConfig {
    pub min_entropy_per_byte: f64,      // Minimum bits per byte
    pub max_autocorrelation: f64,       // Maximum autocorrelation
    pub min_test_pass_rate: f64,        // Minimum test pass rate
    pub analysis_window_size: usize,    // Size of analysis window
    pub threat_models: Vec<ThreatModel>, // Threat models to consider
    pub continuous_monitoring: bool,     // Enable continuous analysis
}

impl Default for SecurityAnalysisConfig {
    fn default() -> Self {
        Self {
            min_entropy_per_byte: 7.5,
            max_autocorrelation: 0.1,
            min_test_pass_rate: 0.9,
            analysis_window_size: 10000,
            threat_models: vec![
                ThreatModel::PassiveObserver,
                ThreatModel::PredictionAttack,
            ],
            continuous_monitoring: false,
        }
    }
}

/// Comprehensive security analysis results
#[derive(Debug, Clone)]
pub struct SecurityAnalysisResult {
    pub overall_security_level: SecurityLevel,
    pub entropy_analysis: EntropyAnalysisResult,
    pub statistical_analysis: StatisticalAnalysisResult,
    pub threat_assessment: ThreatAssessmentResult,
    pub recommendations: Vec<String>,
    pub confidence_score: f64,      // 0.0 to 1.0
    pub analysis_timestamp: SystemTime,
    pub sample_size: usize,
}

/// Entropy analysis results
#[derive(Debug, Clone)]
pub struct EntropyAnalysisResult {
    pub shannon_entropy: f64,
    pub min_entropy: f64,
    pub entropy_per_byte: f64,
    pub compression_ratio: f64,
    pub entropy_quality_score: f64,
    pub entropy_stability: f64,
    pub predictability_score: f64,
}

/// Statistical analysis results
#[derive(Debug, Clone)]
pub struct StatisticalAnalysisResult {
    pub test_results: Vec<TestResult>,
    pub pass_rate: f64,
    pub critical_failures: Vec<String>,
    pub statistical_quality_score: f64,
    pub distribution_uniformity: f64,
    pub independence_score: f64,
}

/// Threat assessment results
#[derive(Debug, Clone)]
pub struct ThreatAssessmentResult {
    pub vulnerable_to: Vec<ThreatModel>,
    pub resistant_to: Vec<ThreatModel>,
    pub risk_scores: HashMap<ThreatModel, f64>,
    pub attack_feasibility: HashMap<ThreatModel, String>,
    pub mitigation_suggestions: HashMap<ThreatModel, Vec<String>>,
}

/// Security analyzer for cryptographic random number generators
pub struct SecurityAnalyzer {
    config: SecurityAnalysisConfig,
    test_suite: RandomnessTestSuite,
    entropy_monitor: EntropyMonitor,
    analysis_history: Vec<SecurityAnalysisResult>,
}

impl SecurityAnalyzer {
    /// Create new security analyzer
    pub fn new() -> Self {
        Self::with_config(SecurityAnalysisConfig::default())
    }
    
    /// Create security analyzer with custom configuration
    pub fn with_config(config: SecurityAnalysisConfig) -> Self {
        let test_config = TestSuiteConfig {
            significance_level: 0.01,
            min_sample_size: config.analysis_window_size,
            enable_all_tests: true,
        };
        
        Self {
            config,
            test_suite: RandomnessTestSuite::with_config(test_config),
            entropy_monitor: EntropyMonitor::default(),
            analysis_history: Vec::new(),
        }
    }
    
    /// Perform comprehensive security analysis
    pub fn analyze(&mut self, data: &[u8]) -> AdvancedCryptoResult<SecurityAnalysisResult> {
        if data.len() < self.config.analysis_window_size {
            return Err(format!(
                "Insufficient data for analysis: {} bytes (minimum {})",
                data.len(),
                self.config.analysis_window_size
            ).into());
        }
        
        let analysis_start = SystemTime::now();
        
        // Perform entropy analysis
        let entropy_analysis = self.analyze_entropy(data)?;
        
        // Perform statistical analysis
        let statistical_analysis = self.analyze_statistics(data)?;
        
        // Perform threat assessment
        let threat_assessment = self.assess_threats(data, &entropy_analysis, &statistical_analysis)?;
        
        // Determine overall security level
        let overall_security_level = self.determine_security_level(
            &entropy_analysis,
            &statistical_analysis,
            &threat_assessment,
        );
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(
            &entropy_analysis,
            &statistical_analysis,
            &threat_assessment,
        );
        
        // Calculate confidence score
        let confidence_score = self.calculate_confidence_score(
            data.len(),
            &statistical_analysis,
            &entropy_analysis,
        );
        
        let result = SecurityAnalysisResult {
            overall_security_level,
            entropy_analysis,
            statistical_analysis,
            threat_assessment,
            recommendations,
            confidence_score,
            analysis_timestamp: analysis_start,
            sample_size: data.len(),
        };
        
        // Store in history
        self.analysis_history.push(result.clone());
        
        // Limit history size
        if self.analysis_history.len() > 100 {
            self.analysis_history.remove(0);
        }
        
        Ok(result)
    }
    
    /// Analyze entropy properties
    fn analyze_entropy(&self, data: &[u8]) -> AdvancedCryptoResult<EntropyAnalysisResult> {
        // Calculate Shannon entropy
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        let len = data.len() as f64;
        let mut shannon_entropy = 0.0;
        for &freq in &frequencies {
            if freq > 0 {
                let p = freq as f64 / len;
                shannon_entropy -= p * p.log2();
            }
        }
        
        // Calculate min-entropy (most conservative)
        let max_freq = frequencies.iter().max().unwrap_or(&0);
        let p_max = *max_freq as f64 / len;
        let min_entropy = if p_max > 0.0 { -p_max.log2() } else { 8.0 };
        
        // Calculate compression ratio
        let compressed_size = self.estimate_compression_ratio(data);
        
        // Calculate entropy per byte
        let entropy_per_byte = shannon_entropy;
        
        // Calculate entropy quality score
        let entropy_quality_score = self.calculate_entropy_quality_score(
            shannon_entropy,
            min_entropy,
            compressed_size,
        );
        
        // Calculate entropy stability (consistency across blocks)
        let entropy_stability = self.calculate_entropy_stability(data);
        
        // Calculate predictability score
        let predictability_score = self.calculate_predictability_score(data);
        
        Ok(EntropyAnalysisResult {
            shannon_entropy,
            min_entropy,
            entropy_per_byte,
            compression_ratio: compressed_size,
            entropy_quality_score,
            entropy_stability,
            predictability_score,
        })
    }
    
    /// Analyze statistical properties
    fn analyze_statistics(&self, data: &[u8]) -> AdvancedCryptoResult<StatisticalAnalysisResult> {
        // Run comprehensive randomness tests
        let test_results = self.test_suite.comprehensive_test(data)?;
        
        // Calculate pass rate
        let passed_tests = test_results.iter().filter(|r| r.passed).count();
        let pass_rate = passed_tests as f64 / test_results.len() as f64;
        
        // Identify critical failures
        let critical_failures = test_results.iter()
            .filter(|r| !r.passed && r.p_value < 0.001)
            .map(|r| r.test_name.clone())
            .collect();
        
        // Calculate statistical quality score
        let statistical_quality_score = self.calculate_statistical_quality_score(&test_results);
        
        // Calculate distribution uniformity
        let distribution_uniformity = self.calculate_distribution_uniformity(data);
        
        // Calculate independence score
        let independence_score = self.calculate_independence_score(data);
        
        Ok(StatisticalAnalysisResult {
            test_results,
            pass_rate,
            critical_failures,
            statistical_quality_score,
            distribution_uniformity,
            independence_score,
        })
    }
    
    /// Assess threats and vulnerabilities
    fn assess_threats(
        &self,
        data: &[u8],
        entropy_analysis: &EntropyAnalysisResult,
        statistical_analysis: &StatisticalAnalysisResult,
    ) -> AdvancedCryptoResult<ThreatAssessmentResult> {
        let mut vulnerable_to = Vec::new();
        let mut resistant_to = Vec::new();
        let mut risk_scores = HashMap::new();
        let mut attack_feasibility = HashMap::new();
        let mut mitigation_suggestions = HashMap::new();
        
        for threat in &self.config.threat_models {
            let (is_vulnerable, risk_score, feasibility, mitigations) = 
                self.assess_threat(threat, data, entropy_analysis, statistical_analysis);
            
            if is_vulnerable {
                vulnerable_to.push(threat.clone());
            } else {
                resistant_to.push(threat.clone());
            }
            
            risk_scores.insert(threat.clone(), risk_score);
            attack_feasibility.insert(threat.clone(), feasibility);
            mitigation_suggestions.insert(threat.clone(), mitigations);
        }
        
        Ok(ThreatAssessmentResult {
            vulnerable_to,
            resistant_to,
            risk_scores,
            attack_feasibility,
            mitigation_suggestions,
        })
    }
    
    /// Assess individual threat
    fn assess_threat(
        &self,
        threat: &ThreatModel,
        data: &[u8],
        entropy_analysis: &EntropyAnalysisResult,
        statistical_analysis: &StatisticalAnalysisResult,
    ) -> (bool, f64, String, Vec<String>) {
        match threat {
            ThreatModel::PassiveObserver => {
                let is_vulnerable = entropy_analysis.shannon_entropy < 7.0 || 
                                  statistical_analysis.pass_rate < 0.8;
                let risk_score = 1.0 - (entropy_analysis.shannon_entropy / 8.0).min(1.0);
                let feasibility = if is_vulnerable {
                    "High - patterns detectable in output".to_string()
                } else {
                    "Low - output appears random".to_string()
                };
                let mitigations = vec![
                    "Ensure high entropy sources".to_string(),
                    "Use cryptographic post-processing".to_string(),
                    "Regular entropy monitoring".to_string(),
                ];
                
                (is_vulnerable, risk_score, feasibility, mitigations)
            }
            
            ThreatModel::ActiveAdversary => {
                let is_vulnerable = entropy_analysis.predictability_score > 0.3;
                let risk_score = entropy_analysis.predictability_score;
                let feasibility = if is_vulnerable {
                    "Medium - may influence entropy sources".to_string()
                } else {
                    "Low - difficult to influence sources".to_string()
                };
                let mitigations = vec![
                    "Use multiple independent entropy sources".to_string(),
                    "Implement entropy source validation".to_string(),
                    "Use hardware-based entropy when available".to_string(),
                ];
                
                (is_vulnerable, risk_score, feasibility, mitigations)
            }
            
            ThreatModel::SideChannelAttack => {
                // Simplified assessment based on timing characteristics
                let timing_vulnerability = self.assess_timing_vulnerability(data);
                let is_vulnerable = timing_vulnerability > 0.5;
                let risk_score = timing_vulnerability;
                let feasibility = if is_vulnerable {
                    "Medium - timing patterns may leak information".to_string()
                } else {
                    "Low - constant-time operations".to_string()
                };
                let mitigations = vec![
                    "Implement constant-time algorithms".to_string(),
                    "Add noise to timing measurements".to_string(),
                    "Use side-channel resistant implementations".to_string(),
                ];
                
                (is_vulnerable, risk_score, feasibility, mitigations)
            }
            
            ThreatModel::StateCompromise => {
                let is_vulnerable = entropy_analysis.entropy_stability < 0.7;
                let risk_score = 1.0 - entropy_analysis.entropy_stability;
                let feasibility = if is_vulnerable {
                    "High - weak state recovery mechanisms".to_string()
                } else {
                    "Medium - forward security implemented".to_string()
                };
                let mitigations = vec![
                    "Implement forward security".to_string(),
                    "Regular reseeding from fresh entropy".to_string(),
                    "State erasure after use".to_string(),
                ];
                
                (is_vulnerable, risk_score, feasibility, mitigations)
            }
            
            ThreatModel::PredictionAttack => {
                let is_vulnerable = entropy_analysis.predictability_score > 0.2 ||
                                  statistical_analysis.independence_score < 0.8;
                let risk_score = entropy_analysis.predictability_score.max(
                    1.0 - statistical_analysis.independence_score
                );
                let feasibility = if is_vulnerable {
                    "Medium - patterns may allow prediction".to_string()
                } else {
                    "Low - high unpredictability".to_string()
                };
                let mitigations = vec![
                    "Use cryptographically secure PRNGs".to_string(),
                    "Ensure sufficient entropy accumulation".to_string(),
                    "Implement output whitening".to_string(),
                ];
                
                (is_vulnerable, risk_score, feasibility, mitigations)
            }
        }
    }
    
    /// Determine overall security level
    fn determine_security_level(
        &self,
        entropy_analysis: &EntropyAnalysisResult,
        statistical_analysis: &StatisticalAnalysisResult,
        threat_assessment: &ThreatAssessmentResult,
    ) -> SecurityLevel {
        // Calculate overall score based on multiple factors
        let entropy_score = entropy_analysis.entropy_quality_score;
        let statistical_score = statistical_analysis.statistical_quality_score;
        let threat_score = 1.0 - threat_assessment.risk_scores.values().sum::<f64>() / threat_assessment.risk_scores.len() as f64;
        
        let overall_score = (entropy_score + statistical_score + threat_score) / 3.0;
        
        match overall_score {
            s if s >= 0.9 => SecurityLevel::Excellent,
            s if s >= 0.8 => SecurityLevel::Strong,
            s if s >= 0.6 => SecurityLevel::Moderate,
            s if s >= 0.3 => SecurityLevel::Weak,
            _ => SecurityLevel::Weak,
        }
    }
    
    /// Generate security recommendations
    fn generate_recommendations(
        &self,
        entropy_analysis: &EntropyAnalysisResult,
        statistical_analysis: &StatisticalAnalysisResult,
        threat_assessment: &ThreatAssessmentResult,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Entropy-based recommendations
        if entropy_analysis.shannon_entropy < self.config.min_entropy_per_byte {
            recommendations.push("Improve entropy sources - Shannon entropy below threshold".to_string());
        }
        
        if entropy_analysis.min_entropy < 6.0 {
            recommendations.push("Address entropy concentration - min-entropy too low".to_string());
        }
        
        if entropy_analysis.compression_ratio < 0.9 {
            recommendations.push("Reduce predictable patterns - compression ratio indicates structure".to_string());
        }
        
        // Statistical recommendations
        if statistical_analysis.pass_rate < self.config.min_test_pass_rate {
            recommendations.push(format!(
                "Improve randomness quality - only {:.1}% tests passing",
                statistical_analysis.pass_rate * 100.0
            ));
        }
        
        if !statistical_analysis.critical_failures.is_empty() {
            recommendations.push(format!(
                "Address critical test failures: {}",
                statistical_analysis.critical_failures.join(", ")
            ));
        }
        
        // Threat-based recommendations
        for (threat, mitigations) in &threat_assessment.mitigation_suggestions {
            if threat_assessment.vulnerable_to.contains(threat) {
                for mitigation in mitigations {
                    recommendations.push(format!("For {:?}: {}", threat, mitigation));
                }
            }
        }
        
        // General recommendations
        if recommendations.is_empty() {
            recommendations.push("Quality appears good - continue monitoring".to_string());
        } else {
            recommendations.push("Consider implementing additional entropy sources".to_string());
            recommendations.push("Enable continuous monitoring for quality assurance".to_string());
        }
        
        recommendations
    }
    
    /// Calculate confidence score
    fn calculate_confidence_score(
        &self,
        sample_size: usize,
        statistical_analysis: &StatisticalAnalysisResult,
        entropy_analysis: &EntropyAnalysisResult,
    ) -> f64 {
        // Base confidence on sample size
        let size_factor = (sample_size as f64 / self.config.analysis_window_size as f64).min(1.0);
        
        // Adjust for test consistency
        let test_consistency = statistical_analysis.pass_rate;
        
        // Adjust for entropy stability
        let entropy_factor = entropy_analysis.entropy_stability;
        
        (size_factor * test_consistency * entropy_factor).min(1.0)
    }
    
    /// Helper functions for detailed analysis
    fn estimate_compression_ratio(&self, data: &[u8]) -> f64 {
        if data.is_empty() {
            return 1.0;
        }
        
        // Simple run-length encoding estimate
        let mut compressed_size = 0;
        let mut i = 0;
        
        while i < data.len() {
            let current = data[i];
            let mut run_length = 1;
            
            while i + run_length < data.len() && data[i + run_length] == current && run_length < 255 {
                run_length += 1;
            }
            
            compressed_size += 2; // value + length
            i += run_length;
        }
        
        compressed_size as f64 / data.len() as f64
    }
    
    fn calculate_entropy_quality_score(&self, shannon: f64, min_entropy: f64, compression: f64) -> f64 {
        let shannon_score = (shannon / 8.0).min(1.0);
        let min_entropy_score = (min_entropy / 8.0).min(1.0);
        let compression_score = (1.0 - compression).max(0.0);
        
        (shannon_score + min_entropy_score + compression_score) / 3.0
    }
    
    fn calculate_entropy_stability(&self, data: &[u8]) -> f64 {
        if data.len() < 1000 {
            return 0.5; // Insufficient data
        }
        
        let block_size = 100;
        let num_blocks = data.len() / block_size;
        let mut block_entropies = Vec::new();
        
        for i in 0..num_blocks {
            let start = i * block_size;
            let end = (start + block_size).min(data.len());
            let block = &data[start..end];
            
            let mut frequencies = [0u32; 256];
            for &byte in block {
                frequencies[byte as usize] += 1;
            }
            
            let len = block.len() as f64;
            let mut entropy = 0.0;
            for &freq in &frequencies {
                if freq > 0 {
                    let p = freq as f64 / len;
                    entropy -= p * p.log2();
                }
            }
            
            block_entropies.push(entropy);
        }
        
        if block_entropies.is_empty() {
            return 0.5;
        }
        
        let mean_entropy = block_entropies.iter().sum::<f64>() / block_entropies.len() as f64;
        let variance = block_entropies.iter()
            .map(|e| (e - mean_entropy).powi(2))
            .sum::<f64>() / block_entropies.len() as f64;
        
        let stability = 1.0 - (variance.sqrt() / 8.0).min(1.0);
        stability.max(0.0)
    }
    
    fn calculate_predictability_score(&self, data: &[u8]) -> f64 {
        if data.len() < 10 {
            return 1.0; // Highly predictable with insufficient data
        }
        
        // Simple predictability based on sequential patterns
        let mut correct_predictions = 0;
        let predictions = data.len() - 1;
        
        for i in 1..data.len() {
            // Simple predictor: next byte equals previous byte
            if data[i] == data[i - 1] {
                correct_predictions += 1;
            }
        }
        
        let prediction_rate = correct_predictions as f64 / predictions as f64;
        
        // Convert to predictability score (higher = more predictable)
        (prediction_rate - 1.0 / 256.0) / (1.0 - 1.0 / 256.0)
    }
    
    fn calculate_statistical_quality_score(&self, test_results: &[TestResult]) -> f64 {
        if test_results.is_empty() {
            return 0.0;
        }
        
        let passed = test_results.iter().filter(|r| r.passed).count();
        let total = test_results.len();
        
        // Weight by p-values for passed tests
        let p_value_sum: f64 = test_results.iter()
            .filter(|r| r.passed)
            .map(|r| r.p_value)
            .sum();
        
        let avg_p_value = if passed > 0 { p_value_sum / passed as f64 } else { 0.0 };
        
        let pass_rate = passed as f64 / total as f64;
        
        // Combine pass rate and average p-value
        (pass_rate + avg_p_value.min(1.0)) / 2.0
    }
    
    fn calculate_distribution_uniformity(&self, data: &[u8]) -> f64 {
        let mut frequencies = [0u32; 256];
        for &byte in data {
            frequencies[byte as usize] += 1;
        }
        
        let expected = data.len() as f64 / 256.0;
        let mut chi_squared = 0.0;
        
        for &freq in &frequencies {
            let diff = freq as f64 - expected;
            chi_squared += diff * diff / expected;
        }
        
        // Convert chi-squared to uniformity score
        let max_chi_squared = data.len() as f64; // Approximate maximum
        1.0 - (chi_squared / max_chi_squared).min(1.0)
    }
    
    fn calculate_independence_score(&self, data: &[u8]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        // Calculate autocorrelation at lag 1
        let n = data.len();
        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / n as f64;
        
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        
        for i in 0..(n - 1) {
            let x1 = data[i] as f64 - mean;
            let x2 = data[i + 1] as f64 - mean;
            numerator += x1 * x2;
        }
        
        for &byte in data {
            let x = byte as f64 - mean;
            denominator += x * x;
        }
        
        let autocorr = if denominator > 0.0 { numerator / denominator } else { 0.0 };
        
        // Convert to independence score (lower autocorrelation = higher independence)
        1.0 - autocorr.abs().min(1.0)
    }
    
    fn assess_timing_vulnerability(&self, _data: &[u8]) -> f64 {
        // Simplified timing vulnerability assessment
        // In a real implementation, this would analyze timing characteristics
        0.3 // Default moderate risk
    }
    
    /// Get analysis history
    pub fn get_analysis_history(&self) -> &[SecurityAnalysisResult] {
        &self.analysis_history
    }
    
    /// Generate security report
    pub fn generate_security_report(&self, result: &SecurityAnalysisResult) -> String {
        let mut report = String::new();
        
        report.push_str("=== CRYPTOGRAPHIC RANDOM NUMBER GENERATOR SECURITY ANALYSIS ===\n\n");
        report.push_str(&format!("Analysis Date: {:?}\n", result.analysis_timestamp));
        report.push_str(&format!("Sample Size: {} bytes\n", result.sample_size));
        report.push_str(&format!("Overall Security Level: {:?}\n", result.overall_security_level));
        report.push_str(&format!("Confidence Score: {:.2}%\n\n", result.confidence_score * 100.0));
        
        // Entropy Analysis
        report.push_str("--- ENTROPY ANALYSIS ---\n");
        report.push_str(&format!("Shannon Entropy: {:.3} bits/byte\n", result.entropy_analysis.shannon_entropy));
        report.push_str(&format!("Min Entropy: {:.3} bits/byte\n", result.entropy_analysis.min_entropy));
        report.push_str(&format!("Compression Ratio: {:.3}\n", result.entropy_analysis.compression_ratio));
        report.push_str(&format!("Quality Score: {:.3}\n", result.entropy_analysis.entropy_quality_score));
        report.push_str(&format!("Stability: {:.3}\n", result.entropy_analysis.entropy_stability));
        report.push_str(&format!("Predictability: {:.3}\n\n", result.entropy_analysis.predictability_score));
        
        // Statistical Analysis
        report.push_str("--- STATISTICAL ANALYSIS ---\n");
        report.push_str(&format!("Test Pass Rate: {:.1}%\n", result.statistical_analysis.pass_rate * 100.0));
        report.push_str(&format!("Quality Score: {:.3}\n", result.statistical_analysis.statistical_quality_score));
        report.push_str(&format!("Distribution Uniformity: {:.3}\n", result.statistical_analysis.distribution_uniformity));
        report.push_str(&format!("Independence Score: {:.3}\n", result.statistical_analysis.independence_score));
        
        if !result.statistical_analysis.critical_failures.is_empty() {
            report.push_str(&format!("Critical Failures: {}\n", result.statistical_analysis.critical_failures.join(", ")));
        }
        report.push('\n');
        
        // Threat Assessment
        report.push_str("--- THREAT ASSESSMENT ---\n");
        report.push_str(&format!("Vulnerable to: {:?}\n", result.threat_assessment.vulnerable_to));
        report.push_str(&format!("Resistant to: {:?}\n", result.threat_assessment.resistant_to));
        
        for (threat, risk) in &result.threat_assessment.risk_scores {
            report.push_str(&format!("{:?} Risk: {:.3}\n", threat, risk));
        }
        report.push('\n');
        
        // Recommendations
        report.push_str("--- RECOMMENDATIONS ---\n");
        for (i, recommendation) in result.recommendations.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, recommendation));
        }
        
        report
    }
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Clone implementations for threat models
impl Clone for ThreatModel {
    fn clone(&self) -> Self {
        match self {
            ThreatModel::PassiveObserver => ThreatModel::PassiveObserver,
            ThreatModel::ActiveAdversary => ThreatModel::ActiveAdversary,
            ThreatModel::SideChannelAttack => ThreatModel::SideChannelAttack,
            ThreatModel::StateCompromise => ThreatModel::StateCompromise,
            ThreatModel::PredictionAttack => ThreatModel::PredictionAttack,
        }
    }
}

impl PartialEq for ThreatModel {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for ThreatModel {}

impl std::hash::Hash for ThreatModel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
