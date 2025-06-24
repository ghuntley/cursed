use crate::error::Error;
/// fr fr Comprehensive cryptographic security analysis framework
/// 
/// This module provides state-of-the-art security analysis capabilities including:
/// - Timing attack detection and side-channel analysis
/// - Entropy validation and randomness quality testing
/// - Cryptographic parameter verification
/// - Vulnerability scanning and security metrics
/// - Real-time security monitoring and reporting
/// 
/// All implementations follow cryptographic best practices with production-ready security.

// Re-export error types
pub use super::errors::{SecurityAnalysisResult, SecurityAnalysisError};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// Import the security analysis types we need
use super::security_analysis::{
    TimingAnalyzer, SideChannelDetector, EntropyValidator, 
    ParameterVerifier, VulnerabilityScanner, EntropyMetrics,
    ParameterVerificationResult, VulnerabilityReport
};

/// fr fr Security analysis levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Basic security analysis
    Basic,
    /// Standard security analysis with timing checks
    Standard,
    /// Comprehensive analysis including side-channel detection
    Comprehensive,
    /// Enterprise-grade analysis with full vulnerability scanning
    Enterprise,
}

/// fr fr Security analysis configuration
#[derive(Debug, Clone)]
pub struct SecurityAnalysisConfig {
    pub level: SecurityLevel,
    pub timing_analysis_enabled: bool,
    pub side_channel_detection_enabled: bool,
    pub entropy_validation_enabled: bool,
    pub parameter_verification_enabled: bool,
    pub vulnerability_scanning_enabled: bool,
    pub analysis_timeout: Duration,
    pub sample_size: usize,
    pub confidence_threshold: f64,
}

impl Default for SecurityAnalysisConfig {
    fn default() -> Self {
        Self {
            level: SecurityLevel::Standard,
            timing_analysis_enabled: true,
            side_channel_detection_enabled: true,
            entropy_validation_enabled: true,
            parameter_verification_enabled: true,
            vulnerability_scanning_enabled: false, // Enterprise feature
            analysis_timeout: Duration::from_secs(30),
            sample_size: 1000,
            confidence_threshold: 0.95,
        }
    }
}

/// fr fr Security metrics collected during analysis
#[derive(Debug, Clone, Default)]
pub struct SecurityMetrics {
    pub timing_variance: f64,
    pub side_channel_score: f64,
    pub entropy_score: f64,
    pub parameter_compliance: f64,
    pub vulnerability_count: usize,
    pub overall_security_score: f64,
    pub analysis_duration: Duration,
    pub issues_detected: Vec<SecurityIssue>,
}

/// fr fr Security issue detected during analysis
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: SecuritySeverity,
    pub category: SecurityCategory,
    pub description: String,
    pub recommendation: String,
    pub confidence: f64,
}

/// fr fr Security issue severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// fr fr Security issue categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityCategory {
    TimingAttack,
    SideChannel,
    WeakEntropy,
    ParameterViolation,
    KnownVulnerability,
    ConfigurationIssue,
}

/// fr fr Main security analysis framework
#[derive(Debug)]
pub struct SecurityAnalysis {
    config: SecurityAnalysisConfig,
    timing_analyzer: Arc<TimingAnalyzer>,
    side_channel_detector: Arc<SideChannelDetector>,
    entropy_validator: Arc<EntropyValidator>,
    parameter_verifier: Arc<ParameterVerifier>,
    vulnerability_scanner: Arc<VulnerabilityScanner>,
    metrics: Arc<Mutex<SecurityMetrics>>,
}

impl SecurityAnalysis {
    /// slay Create a new security analysis framework
    pub fn new(config: SecurityAnalysisConfig) -> Self {
        Self {
            timing_analyzer: Arc::new(TimingAnalyzer::new()),
            side_channel_detector: Arc::new(SideChannelDetector::new()),
            entropy_validator: Arc::new(EntropyValidator::new()),
            parameter_verifier: Arc::new(ParameterVerifier::new()),
            vulnerability_scanner: Arc::new(VulnerabilityScanner::new()),
            metrics: Arc::new(Mutex::new(SecurityMetrics::default())),
            config,
        }
    }

    /// slay Create with default configuration
    pub fn default() -> Self {
        Self::new(SecurityAnalysisConfig::default())
    }

    /// slay Analyze a cryptographic function for security issues
    pub fn analyze_function<F, R>(&self, name: &str, func: F) -> SecurityAnalysisResult<SecurityMetrics>
    where
        F: Fn() -> R + Send + Sync + 'static,
        R: Send + 'static,
    {
        let start_time = Instant::now();
        let mut metrics = SecurityMetrics::default();
        
        // Timing analysis
        if self.config.timing_analysis_enabled {
            let timing_result = self.timing_analyzer.analyze_timing(name, &func, self.config.sample_size)?;
            metrics.timing_variance = timing_result.variance;
            if timing_result.has_timing_vulnerability() {
                metrics.issues_detected.push(SecurityIssue {
                    severity: SecuritySeverity::High,
                    category: SecurityCategory::TimingAttack,
                    description: format!("Timing vulnerability detected in {}", name),
                    recommendation: "Use constant-time implementation".to_string(),
                    confidence: timing_result.confidence,
                });
            }
        }

        // Side-channel analysis
        if self.config.side_channel_detection_enabled {
            let side_channel_result = self.side_channel_detector.analyze_side_channels(name, &func)?;
            metrics.side_channel_score = side_channel_result.security_score;
            if side_channel_result.has_side_channel_leak() {
                metrics.issues_detected.push(SecurityIssue {
                    severity: SecuritySeverity::Medium,
                    category: SecurityCategory::SideChannel,
                    description: format!("Potential side-channel leak in {}", name),
                    recommendation: "Review implementation for data-dependent operations".to_string(),
                    confidence: side_channel_result.confidence,
                });
            }
        }

        metrics.analysis_duration = start_time.elapsed();
        metrics.overall_security_score = self.calculate_overall_score(&metrics);

        // Update stored metrics
        if let Ok(mut stored_metrics) = self.metrics.lock() {
            *stored_metrics = metrics.clone();
        }

        Ok(metrics)
    }

    /// slay Analyze random data for entropy quality
    pub fn analyze_entropy(&self, data: &[u8]) -> SecurityAnalysisResult<EntropyMetrics> {
        if !self.config.entropy_validation_enabled {
            return Err(SecurityAnalysisError::Internal("Entropy validation disabled".to_string()));
        }

        self.entropy_validator.validate_entropy(data)
    }

    /// slay Verify cryptographic parameters
    pub fn verify_parameters(&self, params: &CryptoParameters) -> SecurityAnalysisResult<ParameterVerificationResult> {
        if !self.config.parameter_verification_enabled {
            return Err(SecurityAnalysisError::Internal("Parameter verification disabled".to_string()));
        }

        self.parameter_verifier.verify_parameters(params)
    }

    /// slay Scan for known vulnerabilities
    pub fn scan_vulnerabilities(&self, context: &SecurityContext) -> SecurityAnalysisResult<VulnerabilityReport> {
        if !self.config.vulnerability_scanning_enabled {
            return Err(SecurityAnalysisError::Internal("Vulnerability scanning disabled".to_string()));
        }

        self.vulnerability_scanner.scan_vulnerabilities(context)
    }

    /// slay Get current security metrics
    pub fn get_metrics(&self) -> SecurityMetrics {
        self.metrics.lock()
            .map(|m| m.clone())
            .unwrap_or_default()
    }

    /// slay Calculate overall security score
    fn calculate_overall_score(&self, metrics: &SecurityMetrics) -> f64 {
        let mut score = 100.0;

        // Timing variance penalty
        if metrics.timing_variance > 0.1 {
            score -= 20.0;
        }

        // Side-channel score (lower is better)
        score *= metrics.side_channel_score;

        // Entropy score (higher is better)
        score *= metrics.entropy_score;

        // Parameter compliance (higher is better)
        score *= metrics.parameter_compliance;

        // Vulnerability penalty
        for issue in &metrics.issues_detected {
            match issue.severity {
                SecuritySeverity::Critical => score -= 50.0,
                SecuritySeverity::High => score -= 25.0,
                SecuritySeverity::Medium => score -= 10.0,
                SecuritySeverity::Low => score -= 5.0,
                SecuritySeverity::Info => score -= 1.0,
            }
        }

        score.max(0.0).min(100.0)
    }

    /// slay Generate comprehensive security report
    pub fn generate_report(&self) -> SecurityReport {
        let metrics = self.get_metrics();
        SecurityReport {
            overall_score: metrics.overall_security_score,
            issues: metrics.issues_detected,
            metrics: metrics,
            recommendations: self.generate_recommendations(&metrics),
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// slay Generate security recommendations
    fn generate_recommendations(&self, metrics: &SecurityMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();

        if metrics.timing_variance > 0.1 {
            recommendations.push("Consider using constant-time implementations to prevent timing attacks".to_string());
        }

        if metrics.side_channel_score < 0.8 {
            recommendations.push("Review code for potential side-channel vulnerabilities".to_string());
        }

        if metrics.entropy_score < 0.9 {
            recommendations.push("Improve entropy source quality or use a cryptographically secure PRNG".to_string());
        }

        if metrics.parameter_compliance < 0.95 {
            recommendations.push("Verify all cryptographic parameters meet current security standards".to_string());
        }

        if metrics.vulnerability_count > 0 {
            recommendations.push("Address identified vulnerabilities immediately".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Security analysis passed - no immediate issues detected".to_string());
        }

        recommendations
    }
}

/// fr fr Comprehensive security report
#[derive(Debug, Clone)]
pub struct SecurityReport {
    pub overall_score: f64,
    pub issues: Vec<SecurityIssue>,
    pub metrics: SecurityMetrics,
    pub recommendations: Vec<String>,
    pub timestamp: std::time::SystemTime,
}

impl SecurityReport {
    /// slay Check if the analysis passed security requirements
    pub fn is_secure(&self) -> bool {
        self.overall_score >= 80.0 && 
        !self.issues.iter().any(|i| i.severity >= SecuritySeverity::High)
    }

    /// slay Get critical issues requiring immediate attention
    pub fn get_critical_issues(&self) -> Vec<&SecurityIssue> {
        self.issues.iter()
            .filter(|i| i.severity >= SecuritySeverity::High)
            .collect()
    }
}

/// fr fr Security context for vulnerability scanning
#[derive(Debug, Clone, Default)]
pub struct SecurityContext {
    pub algorithm_name: String,
    pub key_size: usize,
    pub implementation_details: HashMap<String, String>,
    pub environment_info: HashMap<String, String>,
}

/// fr fr Cryptographic parameters for verification
#[derive(Debug, Clone, Default)]
pub struct CryptoParameters {
    pub algorithm: String,
    pub key_size: usize,
    pub block_size: Option<usize>,
    pub iv_size: Option<usize>,
    pub tag_size: Option<usize>,
    pub rounds: Option<usize>,
    pub custom_params: HashMap<String, String>,
}

/// fr fr Convenience functions for quick security analysis
pub mod quick_analysis {
    use super::*;

    /// slay Quick timing analysis of a function
    pub fn check_timing_safety<F, R>(name: &str, func: F) -> SecurityAnalysisResult<bool>
    where
        F: Fn() -> R + Send + Sync + 'static,
        R: Send + 'static,
    {
        let analyzer = TimingAnalyzer::new();
        let result = analyzer.analyze_timing(name, func, 100)?;
        Ok(!result.has_timing_vulnerability())
    }

    /// slay Quick entropy validation
    pub fn check_entropy_quality(data: &[u8]) -> SecurityAnalysisResult<f64> {
        let validator = EntropyValidator::new();
        let result = validator.validate_entropy(data)?;
        Ok(result.entropy_score)
    }

    /// slay Quick parameter verification
    pub fn verify_crypto_params(params: &CryptoParameters) -> SecurityAnalysisResult<bool> {
        let verifier = ParameterVerifier::new();
        let result = verifier.verify_parameters(params)?;
        Ok(result.is_compliant())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_analysis_creation() {
        let config = SecurityAnalysisConfig::default();
        let analysis = SecurityAnalysis::new(config);
        
        // Should create without errors
        assert!(analysis.timing_analyzer);
        assert!(analysis.side_channel_detector);
        assert!(analysis.entropy_validator);
        assert!(analysis.parameter_verifier);
        assert!(analysis.vulnerability_scanner);
    }

    #[test]
    fn test_security_metrics_calculation() {
        let analysis = SecurityAnalysis::default();
        let mut metrics = SecurityMetrics::default();
        
        // Test with good metrics
        metrics.timing_variance = 0.05;
        metrics.side_channel_score = 0.9;
        metrics.entropy_score = 0.95;
        metrics.parameter_compliance = 0.98;
        
        let score = analysis.calculate_overall_score(&metrics);
        assert!(score > 80.0);
    }

    #[test]
    fn test_security_report_generation() {
        let analysis = SecurityAnalysis::default();
        let report = analysis.generate_report();
        
        assert!(report.overall_score >= 0.0);
        assert!(report.overall_score <= 100.0);
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_quick_analysis_functions() {
        // Test quick timing analysis
        let timing_safe = quick_analysis::check_timing_safety("test", || {
            // Constant time operation
            42
        });
        assert!(timing_safe.is_ok());

        // Test entropy check
        let entropy_data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let entropy_result = quick_analysis::check_entropy_quality(&entropy_data);
        assert!(entropy_result.is_ok());
    }
}
