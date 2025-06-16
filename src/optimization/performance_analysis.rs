/// Performance Analysis System
/// 
/// Provides comprehensive performance analysis and regression detection
/// for optimization effectiveness measurement.

use crate::error::{Error, Result};
use crate::optimization::config::OptimizationLevel;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn, instrument};

/// Performance analyzer for measuring optimization effectiveness
pub struct PerformanceAnalyzer {
    baseline_data: Option<PerformanceBaseline>,
    trending_data: VecDeque<PerformanceDataPoint>,
    regression_detector: RegressionDetector,
}

/// Performance baseline for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub timestamp: SystemTime,
    pub optimization_level: OptimizationLevel,
    pub compilation_time: Duration,
    pub runtime_performance: f64,
    pub memory_usage: u64,
    pub code_size: u64,
    pub metadata: HashMap<String, String>,
}

/// Performance data point for trending analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    pub timestamp: SystemTime,
    pub compilation_time: Duration,
    pub runtime_improvement: f64,
    pub size_reduction: f64,
    pub memory_reduction: f64,
    pub optimization_level: OptimizationLevel,
    pub confidence_score: f64,
}

/// Performance trends analysis
#[derive(Debug, Clone)]
pub struct PerformanceTrends {
    pub trend_direction: TrendDirection,
    pub average_improvement: f64,
    pub improvement_variance: f64,
    pub compilation_time_trend: TrendDirection,
    pub stability_score: f64,
    pub data_points: usize,
    pub confidence_interval: (f64, f64),
}

/// Trend direction for performance metrics
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
    Insufficient_Data,
}

/// Regression detection results
#[derive(Debug, Clone)]
pub struct RegressionDetectionResult {
    pub has_regression: bool,
    pub regression_type: RegressionType,
    pub affected_metrics: Vec<AffectedMetric>,
    pub severity: RegressionSeverity,
    pub confidence: f64,
    pub root_cause_analysis: RootCauseAnalysis,
}

/// Type of performance regression
#[derive(Debug, Clone, PartialEq)]
pub enum RegressionType {
    Compilation_Time,
    Runtime_Performance,
    Memory_Usage,
    Code_Size,
    Stability,
    Multiple,
}

/// Affected performance metric
#[derive(Debug, Clone)]
pub struct AffectedMetric {
    pub metric_name: String,
    pub baseline_value: f64,
    pub current_value: f64,
    pub percentage_change: f64,
    pub threshold_exceeded: bool,
}

/// Regression severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum RegressionSeverity {
    Critical,    // >20% degradation
    Major,       // 10-20% degradation  
    Minor,       // 5-10% degradation
    Negligible,  // <5% degradation
}

/// Root cause analysis for regressions
#[derive(Debug, Clone)]
pub struct RootCauseAnalysis {
    pub likely_causes: Vec<String>,
    pub optimization_changes: Vec<String>,
    pub environment_factors: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Regression detector using statistical analysis
pub struct RegressionDetector {
    sensitivity_threshold: f64,
    min_samples: usize,
    confidence_level: f64,
}

impl PerformanceAnalyzer {
    /// Create new performance analyzer
    pub fn new() -> Self {
        Self {
            baseline_data: None,
            trending_data: VecDeque::new(),
            regression_detector: RegressionDetector::new(),
        }
    }
    
    /// Set performance baseline
    #[instrument(skip(self))]
    pub fn set_baseline(&mut self, baseline: PerformanceBaseline) -> Result<()> {
        info!("Setting performance baseline for optimization level {:?}", baseline.optimization_level);
        self.baseline_data = Some(baseline);
        Ok(())
    }
    
    /// Record performance data point
    #[instrument(skip(self))]
    pub fn record_performance(&mut self, data_point: PerformanceDataPoint) -> Result<()> {
        debug!("Recording performance data point: {:.1}% improvement", data_point.runtime_improvement);
        
        self.trending_data.push_back(data_point);
        
        // Limit trending data history
        if self.trending_data.len() > 1000 {
            self.trending_data.pop_front();
        }
        
        Ok(())
    }
    
    /// Analyze performance trends
    #[instrument(skip(self))]
    pub fn analyze_trends(&self) -> Result<PerformanceTrends> {
        if self.trending_data.len() < 3 {
            return Ok(PerformanceTrends {
                trend_direction: TrendDirection::Insufficient_Data,
                average_improvement: 0.0,
                improvement_variance: 0.0,
                compilation_time_trend: TrendDirection::Insufficient_Data,
                stability_score: 0.0,
                data_points: self.trending_data.len(),
                confidence_interval: (0.0, 0.0),
            });
        }
        
        let improvements: Vec<f64> = self.trending_data
            .iter()
            .map(|dp| dp.runtime_improvement)
            .collect();
        
        let compilation_times: Vec<f64> = self.trending_data
            .iter()
            .map(|dp| dp.compilation_time.as_secs_f64())
            .collect();
        
        // Calculate trend statistics
        let average_improvement = improvements.iter().sum::<f64>() / improvements.len() as f64;
        let improvement_variance = self.calculate_variance(&improvements, average_improvement);
        
        let trend_direction = self.determine_trend_direction(&improvements);
        let compilation_time_trend = self.determine_compilation_time_trend(&compilation_times);
        let stability_score = self.calculate_stability_score(&improvements);
        let confidence_interval = self.calculate_confidence_interval(&improvements, 0.95);
        
        Ok(PerformanceTrends {
            trend_direction,
            average_improvement,
            improvement_variance,
            compilation_time_trend,
            stability_score,
            data_points: self.trending_data.len(),
            confidence_interval,
        })
    }
    
    /// Detect performance regressions
    #[instrument(skip(self))]
    pub fn detect_regressions(&self) -> Result<RegressionDetectionResult> {
        if let Some(baseline) = &self.baseline_data {
            if !self.trending_data.is_empty() {
                let latest = self.trending_data.back().unwrap();
                return self.regression_detector.detect_regression(baseline, latest);
            }
        }
        
        // No baseline or data - no regression detected
        Ok(RegressionDetectionResult {
            has_regression: false,
            regression_type: RegressionType::Runtime_Performance,
            affected_metrics: Vec::new(),
            severity: RegressionSeverity::Negligible,
            confidence: 0.0,
            root_cause_analysis: RootCauseAnalysis {
                likely_causes: vec!["Insufficient data for regression analysis".to_string()],
                optimization_changes: Vec::new(),
                environment_factors: Vec::new(),
                recommendations: vec!["Collect more performance data".to_string()],
            },
        })
    }
    
    /// Generate comprehensive performance report
    pub fn generate_performance_report(&self) -> Result<String> {
        let mut report = String::new();
        
        report.push_str("# Performance Analysis Report\n\n");
        
        // Baseline information
        if let Some(baseline) = &self.baseline_data {
            report.push_str("## Baseline Performance\n");
            report.push_str(&format!("- Timestamp: {:?}\n", baseline.timestamp));
            report.push_str(&format!("- Optimization Level: {:?}\n", baseline.optimization_level));
            report.push_str(&format!("- Compilation Time: {:?}\n", baseline.compilation_time));
            report.push_str(&format!("- Runtime Performance: {:.2}\n", baseline.runtime_performance));
            report.push_str(&format!("- Memory Usage: {} bytes\n", baseline.memory_usage));
            report.push_str(&format!("- Code Size: {} bytes\n\n", baseline.code_size));
        }
        
        // Trend analysis
        let trends = self.analyze_trends()?;
        report.push_str("## Performance Trends\n");
        report.push_str(&format!("- Trend Direction: {:?}\n", trends.trend_direction));
        report.push_str(&format!("- Average Improvement: {:.2}%\n", trends.average_improvement));
        report.push_str(&format!("- Improvement Variance: {:.4}\n", trends.improvement_variance));
        report.push_str(&format!("- Compilation Time Trend: {:?}\n", trends.compilation_time_trend));
        report.push_str(&format!("- Stability Score: {:.2}\n", trends.stability_score));
        report.push_str(&format!("- Data Points: {}\n", trends.data_points));
        report.push_str(&format!("- Confidence Interval: ({:.2}%, {:.2}%)\n\n", 
                               trends.confidence_interval.0, trends.confidence_interval.1));
        
        // Regression analysis
        let regression_result = self.detect_regressions()?;
        report.push_str("## Regression Analysis\n");
        report.push_str(&format!("- Has Regression: {}\n", regression_result.has_regression));
        if regression_result.has_regression {
            report.push_str(&format!("- Regression Type: {:?}\n", regression_result.regression_type));
            report.push_str(&format!("- Severity: {:?}\n", regression_result.severity));
            report.push_str(&format!("- Confidence: {:.1}%\n", regression_result.confidence * 100.0));
            
            if !regression_result.affected_metrics.is_empty() {
                report.push_str("\n### Affected Metrics\n");
                for metric in &regression_result.affected_metrics {
                    report.push_str(&format!("- {}: {:.2} → {:.2} ({:+.1}%)\n",
                                           metric.metric_name, metric.baseline_value, 
                                           metric.current_value, metric.percentage_change));
                }
            }
            
            if !regression_result.root_cause_analysis.likely_causes.is_empty() {
                report.push_str("\n### Likely Causes\n");
                for cause in &regression_result.root_cause_analysis.likely_causes {
                    report.push_str(&format!("- {}\n", cause));
                }
            }
            
            if !regression_result.root_cause_analysis.recommendations.is_empty() {
                report.push_str("\n### Recommendations\n");
                for rec in &regression_result.root_cause_analysis.recommendations {
                    report.push_str(&format!("- {}\n", rec));
                }
            }
        }
        
        // Recent performance data
        if !self.trending_data.is_empty() {
            report.push_str("\n## Recent Performance Data\n");
            let recent_data: Vec<_> = self.trending_data.iter().rev().take(5).collect();
            for (i, data) in recent_data.iter().enumerate() {
                report.push_str(&format!("{}. Runtime: {:.1}%, Size: {:.1}%, Memory: {:.1}% ({})\n",
                                       i + 1, data.runtime_improvement, data.size_reduction,
                                       data.memory_reduction, data.optimization_level.as_str()));
            }
        }
        
        Ok(report)
    }
    
    /// Get performance statistics summary
    pub fn get_performance_summary(&self) -> PerformanceSummary {
        let trends = self.analyze_trends().unwrap_or_else(|_| PerformanceTrends {
            trend_direction: TrendDirection::Insufficient_Data,
            average_improvement: 0.0,
            improvement_variance: 0.0,
            compilation_time_trend: TrendDirection::Insufficient_Data,
            stability_score: 0.0,
            data_points: 0,
            confidence_interval: (0.0, 0.0),
        });
        
        let regression_result = self.detect_regressions().unwrap_or_else(|_| RegressionDetectionResult {
            has_regression: false,
            regression_type: RegressionType::Runtime_Performance,
            affected_metrics: Vec::new(),
            severity: RegressionSeverity::Negligible,
            confidence: 0.0,
            root_cause_analysis: RootCauseAnalysis {
                likely_causes: Vec::new(),
                optimization_changes: Vec::new(),
                environment_factors: Vec::new(),
                recommendations: Vec::new(),
            },
        });
        
        PerformanceSummary {
            has_baseline: self.baseline_data.is_some(),
            data_points_count: self.trending_data.len(),
            average_improvement: trends.average_improvement,
            trend_direction: trends.trend_direction,
            stability_score: trends.stability_score,
            has_regressions: regression_result.has_regression,
            regression_severity: regression_result.severity,
        }
    }
    
    // Helper methods
    
    fn calculate_variance(&self, values: &[f64], mean: f64) -> f64 {
        if values.len() <= 1 {
            return 0.0;
        }
        
        let sum_squares: f64 = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum();
        
        sum_squares / (values.len() - 1) as f64
    }
    
    fn determine_trend_direction(&self, values: &[f64]) -> TrendDirection {
        if values.len() < 3 {
            return TrendDirection::Insufficient_Data;
        }
        
        // Simple linear regression approach
        let n = values.len() as f64;
        let x_values: Vec<f64> = (0..values.len()).map(|i| i as f64).collect();
        
        let x_mean = x_values.iter().sum::<f64>() / n;
        let y_mean = values.iter().sum::<f64>() / n;
        
        let numerator: f64 = x_values.iter().zip(values.iter())
            .map(|(x, y)| (x - x_mean) * (y - y_mean))
            .sum();
        
        let denominator: f64 = x_values.iter()
            .map(|x| (x - x_mean).powi(2))
            .sum();
        
        if denominator.abs() < f64::EPSILON {
            return TrendDirection::Stable;
        }
        
        let slope = numerator / denominator;
        
        if slope > 0.1 {
            TrendDirection::Improving
        } else if slope < -0.1 {
            TrendDirection::Degrading
        } else {
            TrendDirection::Stable
        }
    }
    
    fn determine_compilation_time_trend(&self, times: &[f64]) -> TrendDirection {
        // Similar to performance trend but inverted (lower is better)
        let trend = self.determine_trend_direction(times);
        match trend {
            TrendDirection::Improving => TrendDirection::Degrading, // Increasing time is bad
            TrendDirection::Degrading => TrendDirection::Improving, // Decreasing time is good
            other => other,
        }
    }
    
    fn calculate_stability_score(&self, values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = self.calculate_variance(values, mean);
        let coefficient_of_variation = if mean.abs() > f64::EPSILON {
            (variance.sqrt() / mean.abs()).min(1.0)
        } else {
            1.0
        };
        
        // Higher stability score means more stable (less variation)
        1.0 - coefficient_of_variation
    }
    
    fn calculate_confidence_interval(&self, values: &[f64], confidence_level: f64) -> (f64, f64) {
        if values.len() < 2 {
            return (0.0, 0.0);
        }
        
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = self.calculate_variance(values, mean);
        let std_error = variance.sqrt() / (values.len() as f64).sqrt();
        
        // Simplified confidence interval (using normal approximation)
        let z_score = if confidence_level >= 0.95 { 1.96 } else { 1.645 };
        let margin = z_score * std_error;
        
        (mean - margin, mean + margin)
    }
}

impl RegressionDetector {
    pub fn new() -> Self {
        Self {
            sensitivity_threshold: 0.05, // 5% threshold
            min_samples: 3,
            confidence_level: 0.8,
        }
    }
    
    pub fn detect_regression(
        &self, 
        baseline: &PerformanceBaseline, 
        current: &PerformanceDataPoint
    ) -> Result<RegressionDetectionResult> {
        let mut affected_metrics = Vec::new();
        let mut regression_types = Vec::new();
        
        // Check compilation time regression
        let baseline_compile_time = baseline.compilation_time.as_secs_f64();
        let current_compile_time = current.compilation_time.as_secs_f64();
        let compile_time_change = (current_compile_time - baseline_compile_time) / baseline_compile_time;
        
        if compile_time_change > self.sensitivity_threshold {
            affected_metrics.push(AffectedMetric {
                metric_name: "Compilation Time".to_string(),
                baseline_value: baseline_compile_time,
                current_value: current_compile_time,
                percentage_change: compile_time_change * 100.0,
                threshold_exceeded: true,
            });
            regression_types.push(RegressionType::Compilation_Time);
        }
        
        // Check runtime performance regression
        let runtime_change = -current.runtime_improvement / 100.0; // Convert improvement to change
        if runtime_change > self.sensitivity_threshold {
            affected_metrics.push(AffectedMetric {
                metric_name: "Runtime Performance".to_string(),
                baseline_value: baseline.runtime_performance,
                current_value: baseline.runtime_performance + runtime_change,
                percentage_change: runtime_change * 100.0,
                threshold_exceeded: true,
            });
            regression_types.push(RegressionType::Runtime_Performance);
        }
        
        // Determine overall regression result
        let has_regression = !affected_metrics.is_empty();
        let regression_type = if regression_types.len() > 1 {
            RegressionType::Multiple
        } else {
            regression_types.first().cloned().unwrap_or(RegressionType::Runtime_Performance)
        };
        
        let severity = self.determine_regression_severity(&affected_metrics);
        let confidence = current.confidence_score;
        
        let root_cause_analysis = self.analyze_root_causes(&regression_type, &affected_metrics);
        
        Ok(RegressionDetectionResult {
            has_regression,
            regression_type,
            affected_metrics,
            severity,
            confidence,
            root_cause_analysis,
        })
    }
    
    fn determine_regression_severity(&self, metrics: &[AffectedMetric]) -> RegressionSeverity {
        if metrics.is_empty() {
            return RegressionSeverity::Negligible;
        }
        
        let max_degradation = metrics.iter()
            .map(|m| m.percentage_change.abs())
            .fold(0.0, f64::max);
        
        if max_degradation > 20.0 {
            RegressionSeverity::Critical
        } else if max_degradation > 10.0 {
            RegressionSeverity::Major
        } else if max_degradation > 5.0 {
            RegressionSeverity::Minor
        } else {
            RegressionSeverity::Negligible
        }
    }
    
    fn analyze_root_causes(&self, regression_type: &RegressionType, _metrics: &[AffectedMetric]) -> RootCauseAnalysis {
        let mut likely_causes = Vec::new();
        let mut optimization_changes = Vec::new();
        let mut environment_factors = Vec::new();
        let mut recommendations = Vec::new();
        
        match regression_type {
            RegressionType::Compilation_Time => {
                likely_causes.push("Increased code complexity".to_string());
                likely_causes.push("Additional optimization passes".to_string());
                optimization_changes.push("More aggressive optimization settings".to_string());
                recommendations.push("Consider reducing optimization level for development builds".to_string());
            }
            RegressionType::Runtime_Performance => {
                likely_causes.push("Suboptimal optimization decisions".to_string());
                likely_causes.push("Changed inlining thresholds".to_string());
                optimization_changes.push("Modified function inlining criteria".to_string());
                recommendations.push("Review inlining profitability calculations".to_string());
            }
            RegressionType::Memory_Usage => {
                likely_causes.push("Increased memory allocations".to_string());
                likely_causes.push("Less effective dead code elimination".to_string());
                recommendations.push("Review memory optimization passes".to_string());
            }
            RegressionType::Code_Size => {
                likely_causes.push("Aggressive function inlining".to_string());
                likely_causes.push("Disabled size optimizations".to_string());
                recommendations.push("Enable size-focused optimization passes".to_string());
            }
            RegressionType::Multiple => {
                likely_causes.push("Systematic optimization changes".to_string());
                recommendations.push("Comprehensive optimization review needed".to_string());
            }
            _ => {}
        }
        
        environment_factors.push("Compiler version differences".to_string());
        environment_factors.push("Target platform changes".to_string());
        environment_factors.push("Build environment variations".to_string());
        
        RootCauseAnalysis {
            likely_causes,
            optimization_changes,
            environment_factors,
            recommendations,
        }
    }
}

/// Performance summary for quick overview
#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub has_baseline: bool,
    pub data_points_count: usize,
    pub average_improvement: f64,
    pub trend_direction: TrendDirection,
    pub stability_score: f64,
    pub has_regressions: bool,
    pub regression_severity: RegressionSeverity,
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_analyzer_creation() {
        let analyzer = PerformanceAnalyzer::new();
        let summary = analyzer.get_performance_summary();
        
        assert!(!summary.has_baseline);
        assert_eq!(summary.data_points_count, 0);
        assert_eq!(summary.trend_direction, TrendDirection::Insufficient_Data);
    }
    
    #[test]
    fn test_baseline_setting() {
        let mut analyzer = PerformanceAnalyzer::new();
        
        let baseline = PerformanceBaseline {
            timestamp: SystemTime::now(),
            optimization_level: OptimizationLevel::Default,
            compilation_time: Duration::from_secs(5),
            runtime_performance: 100.0,
            memory_usage: 1024 * 1024,
            code_size: 50000,
            metadata: HashMap::new(),
        };
        
        analyzer.set_baseline(baseline).unwrap();
        
        let summary = analyzer.get_performance_summary();
        assert!(summary.has_baseline);
    }
    
    #[test]
    fn test_performance_data_recording() {
        let mut analyzer = PerformanceAnalyzer::new();
        
        let data_point = PerformanceDataPoint {
            timestamp: SystemTime::now(),
            compilation_time: Duration::from_secs(4),
            runtime_improvement: 15.0,
            size_reduction: 10.0,
            memory_reduction: 5.0,
            optimization_level: OptimizationLevel::Default,
            confidence_score: 0.9,
        };
        
        analyzer.record_performance(data_point).unwrap();
        
        let summary = analyzer.get_performance_summary();
        assert_eq!(summary.data_points_count, 1);
    }
    
    #[test]
    fn test_trend_analysis() {
        let mut analyzer = PerformanceAnalyzer::new();
        
        // Add multiple data points with improving trend
        for i in 0..5 {
            let data_point = PerformanceDataPoint {
                timestamp: SystemTime::now(),
                compilation_time: Duration::from_secs(5 - i / 2), // Slightly improving
                runtime_improvement: 10.0 + i as f64 * 2.0, // Improving
                size_reduction: 5.0,
                memory_reduction: 3.0,
                optimization_level: OptimizationLevel::Default,
                confidence_score: 0.8,
            };
            analyzer.record_performance(data_point).unwrap();
        }
        
        let trends = analyzer.analyze_trends().unwrap();
        assert!(trends.average_improvement > 10.0);
        assert!(trends.data_points >= 5);
    }
    
    #[test]
    fn test_regression_detection() {
        let mut analyzer = PerformanceAnalyzer::new();
        
        // Set baseline
        let baseline = PerformanceBaseline {
            timestamp: SystemTime::now(),
            optimization_level: OptimizationLevel::Default,
            compilation_time: Duration::from_secs(3),
            runtime_performance: 100.0,
            memory_usage: 1024 * 1024,
            code_size: 50000,
            metadata: HashMap::new(),
        };
        analyzer.set_baseline(baseline).unwrap();
        
        // Add a data point showing regression
        let regression_point = PerformanceDataPoint {
            timestamp: SystemTime::now(),
            compilation_time: Duration::from_secs(6), // 2x slower compilation
            runtime_improvement: -10.0, // Performance regression
            size_reduction: 0.0,
            memory_reduction: 0.0,
            optimization_level: OptimizationLevel::Default,
            confidence_score: 0.9,
        };
        analyzer.record_performance(regression_point).unwrap();
        
        let regression_result = analyzer.detect_regressions().unwrap();
        assert!(regression_result.has_regression);
        assert!(!regression_result.affected_metrics.is_empty());
    }
    
    #[test]
    fn test_performance_report_generation() {
        let mut analyzer = PerformanceAnalyzer::new();
        
        // Add some test data
        let baseline = PerformanceBaseline {
            timestamp: SystemTime::now(),
            optimization_level: OptimizationLevel::Default,
            compilation_time: Duration::from_secs(4),
            runtime_performance: 100.0,
            memory_usage: 1024 * 1024,
            code_size: 50000,
            metadata: HashMap::new(),
        };
        analyzer.set_baseline(baseline).unwrap();
        
        let data_point = PerformanceDataPoint {
            timestamp: SystemTime::now(),
            compilation_time: Duration::from_secs(3),
            runtime_improvement: 20.0,
            size_reduction: 15.0,
            memory_reduction: 10.0,
            optimization_level: OptimizationLevel::Default,
            confidence_score: 0.95,
        };
        analyzer.record_performance(data_point).unwrap();
        
        let report = analyzer.generate_performance_report().unwrap();
        
        assert!(report.contains("Performance Analysis Report"));
        assert!(report.contains("Baseline Performance"));
        assert!(report.contains("Performance Trends"));
        assert!(report.contains("Regression Analysis"));
    }
}
