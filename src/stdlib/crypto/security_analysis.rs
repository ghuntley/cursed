// Security analysis utilities for cryptographic operations

use std::time::Duration;

/// Timing analysis result
#[derive(Debug, Clone)]
pub struct TimingResult {
    pub duration: Duration,
    pub operations: usize,
    pub average_time: Duration,
}

/// Timing comparison result
#[derive(Debug, Clone)]
pub struct TimingComparison {
    pub baseline: TimingResult,
    pub test: TimingResult,
    pub difference: Duration,
    pub is_constant_time: bool,
}

/// Complexity analysis
#[derive(Debug, Clone)]
pub struct ComplexityAnalysis {
    pub algorithm: String,
    pub time_complexity: String,
    pub space_complexity: String,
    pub security_level: u32,
}

/// Information leak indicator
#[derive(Debug, Clone)]
pub struct LeakIndicator {
    pub leak_type: LeakType,
    pub risk_level: RiskLevel,
    pub description: String,
}

/// Types of information leaks
#[derive(Debug, Clone, PartialEq)]
pub enum LeakType {
    TimingLeak,
    PowerLeak,
    CacheLeak,
    SideChannel,
    DataLeak,
}

/// Risk assessment levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Entropy measurement result
#[derive(Debug, Clone)]
pub struct EntropyResult {
    pub bits_of_entropy: f64,
    pub quality: EntropyQuality,
    pub sources: Vec<String>,
}

/// Entropy quality assessment
#[derive(Debug, Clone, PartialEq)]
pub enum EntropyQuality {
    Poor,
    Fair,
    Good,
    Excellent,
}

/// Statistical test results
#[derive(Debug, Clone)]
pub struct StatisticalTestResults {
    pub chi_square: f64,
    pub entropy: f64,
    pub compression_ratio: f64,
    pub passed_tests: usize,
    pub total_tests: usize,
}

/// Parameter violation
#[derive(Debug, Clone)]
pub struct ParameterViolation {
    pub violation_type: ViolationType,
    pub parameter_name: String,
    pub expected_range: (f64, f64),
    pub actual_value: f64,
}

/// Types of parameter violations
#[derive(Debug, Clone, PartialEq)]
pub enum ViolationType {
    OutOfRange,
    InvalidType,
    SecurityWeakness,
    PerformanceIssue,
}

impl TimingResult {
    pub fn new(duration: Duration, operations: usize) -> Self {
        let average_time = if operations > 0 {
            duration / operations as u32
        } else {
            Duration::ZERO
        };

        Self {
            duration,
            operations,
            average_time,
        }
    }
}

impl TimingComparison {
    pub fn analyze(baseline: TimingResult, test: TimingResult) -> Self {
        let difference = if test.average_time > baseline.average_time {
            test.average_time - baseline.average_time
        } else {
            baseline.average_time - test.average_time
        };

        // Simple constant-time check (stub implementation)
        let is_constant_time = difference < Duration::from_nanos(1000);

        Self {
            baseline,
            test,
            difference,
            is_constant_time,
        }
    }
}

impl EntropyResult {
    pub fn new(bits_of_entropy: f64) -> Self {
        let quality = match bits_of_entropy {
            x if x >= 7.0 => EntropyQuality::Excellent,
            x if x >= 5.0 => EntropyQuality::Good,
            x if x >= 3.0 => EntropyQuality::Fair,
            _ => EntropyQuality::Poor,
        };

        Self {
            bits_of_entropy,
            quality,
            sources: vec!["system".to_string()],
        }
    }
}
