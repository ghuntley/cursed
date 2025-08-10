//! Performance regression detection system
//! 
//! This module implements comprehensive performance regression detection
//! to identify and prevent performance degradations during optimization.

use crate::error::{CursedError, Result};
use crate::optimization::{OptimizationConfig, OptimizationLevel};
use crate::optimization::benchmarking::BenchmarkResults;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};
use serde::{Serialize, Deserialize};

/// Performance regression detection system
pub struct PerformanceRegressionDetector {
    config: RegressionDetectionConfig,
    baseline_database: BaselineDatabase,
    current_metrics: Option<PerformanceMetrics>,
    regression_history: VecDeque<RegressionEvent>,
    statistical_analyzer: StatisticalAnalyzer,
}

/// Configuration for regression detection
#[derive(Debug, Clone)]
pub struct RegressionDetectionConfig {
    pub regression_threshold_percent: f64,
    pub warning_threshold_percent: f64,
    pub minimum_samples_for_analysis: usize,
    pub baseline_retention_days: u32,
    pub enable_statistical_testing: bool,
    pub confidence_level: f64,
    pub enable_automated_rollback: bool,
    pub performance_variance_tolerance: f64,
}

/// Baseline performance database
pub struct BaselineDatabase {
    baselines: HashMap<String, PerformanceBaseline>,
    baseline_storage_path: PathBuf,
    retention_policy: BaselineRetentionPolicy,
}

/// Performance baseline for a specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub configuration_id: String,
    pub metrics: PerformanceMetrics,
    pub metadata: BaselineMetadata,
    pub statistical_summary: StatisticalSummary,
    pub created_at: SystemTime,
    pub last_updated: SystemTime,
}

/// Performance metrics for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub compilation_time: Duration,
    pub execution_time: Duration,
    pub memory_usage_peak: usize,
    pub memory_usage_average: usize,
    pub code_size: usize,
    pub optimization_time: Duration,
    pub throughput_ops_per_sec: f64,
    pub custom_metrics: HashMap<String, f64>,
}

/// Baseline metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetadata {
    pub version: String,
    pub optimization_level: String,
    pub target_platform: String,
    pub compiler_flags: Vec<String>,
    pub environment_info: EnvironmentInfo,
    pub test_configuration: TestConfiguration,
}

/// Environment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    pub os: String,
    pub arch: String,
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub memory_gb: f64,
    pub compiler_version: String,
}

/// Test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfiguration {
    pub test_suite: String,
    pub test_parameters: HashMap<String, String>,
    pub input_size: usize,
    pub iterations: usize,
}

/// Statistical summary of baseline performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalSummary {
    pub mean: f64,
    pub median: f64,
    pub standard_deviation: f64,
    pub variance: f64,
    pub confidence_interval_95: (f64, f64),
    pub sample_count: usize,
    pub outliers_removed: usize,
}

/// Baseline retention policy
#[derive(Debug, Clone)]
pub struct BaselineRetentionPolicy {
    pub max_baselines_per_config: usize,
    pub retention_days: u32,
    pub keep_milestone_versions: bool,
    pub auto_cleanup_enabled: bool,
}

/// Regression event record
#[derive(Debug, Clone)]
pub struct RegressionEvent {
    pub event_id: String,
    pub detected_at: SystemTime,
    pub severity: RegressionSeverity,
    pub affected_metrics: Vec<String>,
    pub regression_magnitude: f64,
    pub baseline_comparison: BaselineComparison,
    pub statistical_significance: StatisticalSignificance,
    pub recommended_actions: Vec<RecommendedAction>,
}

/// Regression severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum RegressionSeverity {
    Critical,    // > 50% degradation
    Major,       // 20-50% degradation
    Minor,       // 5-20% degradation
    Warning,     // 2-5% degradation
    Negligible,  // < 2% degradation
}

/// Baseline comparison result
#[derive(Debug, Clone)]
pub struct BaselineComparison {
    pub baseline_id: String,
    pub current_metrics: PerformanceMetrics,
    pub baseline_metrics: PerformanceMetrics,
    pub metric_comparisons: HashMap<String, MetricComparison>,
    pub overall_regression_score: f64,
}

/// Individual metric comparison
#[derive(Debug, Clone)]
pub struct MetricComparison {
    pub metric_name: String,
    pub current_value: f64,
    pub baseline_value: f64,
    pub change_percent: f64,
    pub is_regression: bool,
    pub severity: RegressionSeverity,
    pub statistical_significance: f64,
}

/// Statistical significance assessment
#[derive(Debug, Clone)]
pub struct StatisticalSignificance {
    pub p_value: f64,
    pub t_statistic: f64,
    pub confidence_level: f64,
    pub is_significant: bool,
    pub effect_size: f64,
}

/// Recommended actions for addressing regressions
#[derive(Debug, Clone)]
pub enum RecommendedAction {
    RollbackOptimization(String),
    AdjustOptimizationParameters(HashMap<String, String>),
    InvestigateSpecificPass(String),
    UpdateBaseline(String),
    IgnoreRegression(String),
    ScheduleDetailedAnalysis,
}

/// Statistical analyzer for regression detection
pub struct StatisticalAnalyzer {
    config: StatisticalAnalysisConfig,
    sample_buffer: VecDeque<PerformanceMetrics>,
}

/// Statistical analysis configuration
#[derive(Debug, Clone)]
pub struct StatisticalAnalysisConfig {
    pub significance_level: f64,
    pub minimum_effect_size: f64,
    pub outlier_detection_method: OutlierDetectionMethod,
    pub variance_stabilization: bool,
    pub multiple_comparison_correction: bool,
}

/// Outlier detection methods
#[derive(Debug, Clone)]
pub enum OutlierDetectionMethod {
    ZScore(f64),
    IQR(f64),
    ModifiedZScore(f64),
    None,
}

impl PerformanceRegressionDetector {
    /// Create a new regression detector
    pub fn new(config: RegressionDetectionConfig, baseline_storage_path: PathBuf) -> Result<Self> {
        let baseline_database = BaselineDatabase::new(baseline_storage_path.clone())?;
        let statistical_analyzer = StatisticalAnalyzer::new(StatisticalAnalysisConfig::default());
        
        Ok(Self {
            config,
            baseline_database,
            current_metrics: None,
            regression_history: VecDeque::new(),
            statistical_analyzer,
        })
    }
    
    /// Set current performance metrics
    pub fn set_current_metrics(&mut self, metrics: PerformanceMetrics) {
        self.current_metrics = Some(metrics);
    }
    
    /// Detect performance regressions
    pub fn detect_regressions(&mut self, configuration_id: &str) -> Result<RegressionDetectionResult> {
        let start_time = Instant::now();
        
        let current_metrics = self.current_metrics.as_ref()
            .ok_or_else(|| CursedError::runtime_error("No current metrics available"))?;
        
        // Get baseline for comparison
        let baseline = self.baseline_database.get_baseline(configuration_id)?;
        
        // Perform comparison
        let comparison = self.compare_with_baseline(current_metrics, &baseline)?;
        
        // Detect regressions
        let regressions = self.identify_regressions(&comparison)?;
        
        // Assess statistical significance
        let statistical_results = if self.config.enable_statistical_testing {
            Some(self.statistical_analyzer.analyze_significance(current_metrics, &baseline)?)
        } else {
            None
        };
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&regressions)?;
        
        // Record regression events
        for regression in &regressions {
            self.record_regression_event(regression.clone());
        }
        
        let result = RegressionDetectionResult {
            configuration_id: configuration_id.to_string(),
            baseline_comparison: comparison,
            detected_regressions: regressions,
            statistical_analysis: statistical_results,
            recommended_actions: recommendations,
            detection_time: start_time.elapsed(),
            overall_assessment: self.assess_overall_regression_severity(&regressions),
        };
        
        Ok(result)
    }
    
    /// Update baseline with new performance data
    pub fn update_baseline(&mut self, configuration_id: &str, metrics: PerformanceMetrics) -> Result<()> {
        let metadata = self.create_baseline_metadata(configuration_id)?;
        let statistical_summary = self.statistical_analyzer.compute_statistical_summary(&metrics)?;
        
        let baseline = PerformanceBaseline {
            configuration_id: configuration_id.to_string(),
            metrics,
            metadata,
            statistical_summary,
            created_at: SystemTime::now(),
            last_updated: SystemTime::now(),
        };
        
        self.baseline_database.store_baseline(baseline)?;
        Ok(())
    }
    
    /// Get regression history
    pub fn get_regression_history(&self) -> &VecDeque<RegressionEvent> {
        &self.regression_history
    }
    
    /// Check if automated rollback should be triggered
    pub fn should_trigger_rollback(&self, severity: &RegressionSeverity) -> bool {
        if !self.config.enable_automated_rollback {
            return false;
        }
        
        matches!(severity, RegressionSeverity::Critical | RegressionSeverity::Major)
    }
    
    /// Generate regression report
    pub fn generate_regression_report(&self) -> RegressionReport {
        let total_regressions = self.regression_history.len();
        let recent_regressions = self.regression_history.iter()
            .filter(|event| {
                event.detected_at.elapsed().unwrap_or(Duration::MAX) < Duration::from_secs(7 * 24 * 3600)
            })
            .count();
        
        let severity_distribution = self.calculate_severity_distribution();
        let most_affected_metrics = self.identify_most_affected_metrics();
        
        RegressionReport {
            total_regressions,
            recent_regressions,
            severity_distribution,
            most_affected_metrics,
            baseline_database_size: self.baseline_database.get_baseline_count(),
            detection_accuracy: self.calculate_detection_accuracy(),
        }
    }
    
    // Private implementation methods
    
    fn compare_with_baseline(&self, current: &PerformanceMetrics, baseline: &PerformanceBaseline) -> Result<BaselineComparison> {
        let mut metric_comparisons = HashMap::new();
        
        // Compare compilation time
        let comp_time_comparison = self.compare_metric(
            "compilation_time",
            current.compilation_time.as_secs_f64(),
            baseline.metrics.compilation_time.as_secs_f64(),
        );
        metric_comparisons.insert("compilation_time".to_string(), comp_time_comparison);
        
        // Compare execution time
        let exec_time_comparison = self.compare_metric(
            "execution_time",
            current.execution_time.as_secs_f64(),
            baseline.metrics.execution_time.as_secs_f64(),
        );
        metric_comparisons.insert("execution_time".to_string(), exec_time_comparison);
        
        // Compare memory usage
        let memory_comparison = self.compare_metric(
            "memory_usage_peak",
            current.memory_usage_peak as f64,
            baseline.metrics.memory_usage_peak as f64,
        );
        metric_comparisons.insert("memory_usage_peak".to_string(), memory_comparison);
        
        // Compare code size
        let code_size_comparison = self.compare_metric(
            "code_size",
            current.code_size as f64,
            baseline.metrics.code_size as f64,
        );
        metric_comparisons.insert("code_size".to_string(), code_size_comparison);
        
        // Compare throughput
        let throughput_comparison = self.compare_metric(
            "throughput_ops_per_sec",
            current.throughput_ops_per_sec,
            baseline.metrics.throughput_ops_per_sec,
        );
        metric_comparisons.insert("throughput_ops_per_sec".to_string(), throughput_comparison);
        
        // Compare custom metrics
        for (metric_name, current_value) in &current.custom_metrics {
            if let Some(baseline_value) = baseline.metrics.custom_metrics.get(metric_name) {
                let comparison = self.compare_metric(metric_name, *current_value, *baseline_value);
                metric_comparisons.insert(metric_name.clone(), comparison);
            }
        }
        
        // Calculate overall regression score
        let overall_regression_score = self.calculate_overall_regression_score(&metric_comparisons);
        
        Ok(BaselineComparison {
            baseline_id: baseline.configuration_id.clone(),
            current_metrics: current.clone(),
            baseline_metrics: baseline.metrics.clone(),
            metric_comparisons,
            overall_regression_score,
        })
    }
    
    fn compare_metric(&self, metric_name: &str, current_value: f64, baseline_value: f64) -> MetricComparison {
        let change_percent = if baseline_value != 0.0 {
            ((current_value - baseline_value) / baseline_value) * 100.0
        } else {
            0.0
        };
        
        // Determine if this is a regression based on metric type
        let is_regression = match metric_name {
            "compilation_time" | "execution_time" | "memory_usage_peak" | "memory_usage_average" | "code_size" => {
                change_percent > self.config.warning_threshold_percent
            }
            "throughput_ops_per_sec" => {
                change_percent < -self.config.warning_threshold_percent
            }
            _ => change_percent.abs() > self.config.warning_threshold_percent
        };
        
        let severity = self.determine_regression_severity(change_percent.abs());
        
        MetricComparison {
            metric_name: metric_name.to_string(),
            current_value,
            baseline_value,
            change_percent,
            is_regression,
            severity,
            statistical_significance: 0.95, // Would be calculated properly
        }
    }
    
    fn determine_regression_severity(&self, change_percent: f64) -> RegressionSeverity {
        if change_percent >= 50.0 {
            RegressionSeverity::Critical
        } else if change_percent >= 20.0 {
            RegressionSeverity::Major
        } else if change_percent >= 5.0 {
            RegressionSeverity::Minor
        } else if change_percent >= 2.0 {
            RegressionSeverity::Warning
        } else {
            RegressionSeverity::Negligible
        }
    }
    
    fn identify_regressions(&self, comparison: &BaselineComparison) -> Result<Vec<RegressionEvent>> {
        let mut regressions = Vec::new();
        
        for (metric_name, metric_comparison) in &comparison.metric_comparisons {
            if metric_comparison.is_regression && 
               metric_comparison.severity != RegressionSeverity::Negligible {
                
                let event_id = format!("{}_{}", metric_name, chrono::Utc::now().timestamp());
                
                let regression_event = RegressionEvent {
                    event_id,
                    detected_at: SystemTime::now(),
                    severity: metric_comparison.severity.clone(),
                    affected_metrics: vec![metric_name.clone()],
                    regression_magnitude: metric_comparison.change_percent.abs(),
                    baseline_comparison: comparison.clone(),
                    statistical_significance: StatisticalSignificance {
                        p_value: 0.05, // Would be calculated properly
                        t_statistic: 2.5, // Would be calculated properly
                        confidence_level: 0.95,
                        is_significant: true,
                        effect_size: metric_comparison.change_percent.abs() / 100.0,
                    },
                    recommended_actions: Vec::new(), // Will be filled later
                };
                
                regressions.push(regression_event);
            }
        }
        
        Ok(regressions)
    }
    
    fn generate_recommendations(&self, regressions: &[RegressionEvent]) -> Result<Vec<RecommendedAction>> {
        let mut recommendations = Vec::new();
        
        for regression in regressions {
            match regression.severity {
                RegressionSeverity::Critical => {
                    recommendations.push(RecommendedAction::RollbackOptimization(
                        "Critical regression detected - immediate rollback recommended".to_string()
                    ));
                }
                RegressionSeverity::Major => {
                    recommendations.push(RecommendedAction::InvestigateSpecificPass(
                        "Major regression - investigate optimization passes".to_string()
                    ));
                    recommendations.push(RecommendedAction::AdjustOptimizationParameters(
                        [("aggressiveness".to_string(), "reduce".to_string())].iter().cloned().collect()
                    ));
                }
                RegressionSeverity::Minor => {
                    recommendations.push(RecommendedAction::ScheduleDetailedAnalysis);
                }
                RegressionSeverity::Warning => {
                    recommendations.push(RecommendedAction::UpdateBaseline(
                        "Consider updating baseline if this is expected behavior".to_string()
                    ));
                }
                RegressionSeverity::Negligible => {
                    // No action needed
                }
            }
        }
        
        Ok(recommendations)
    }
    
    fn calculate_overall_regression_score(&self, comparisons: &HashMap<String, MetricComparison>) -> f64 {
        let total_weight = comparisons.len() as f64;
        if total_weight == 0.0 {
            return 0.0;
        }
        
        let weighted_sum: f64 = comparisons.values()
            .map(|comparison| {
                let weight = match comparison.metric_name.as_str() {
                    "execution_time" => 3.0,
                    "compilation_time" => 2.0,
                    "memory_usage_peak" => 2.0,
                    "throughput_ops_per_sec" => 3.0,
                    _ => 1.0,
                };
                comparison.change_percent.abs() * weight
            })
            .sum();
        
        let total_weights: f64 = comparisons.values()
            .map(|comparison| match comparison.metric_name.as_str() {
                "execution_time" => 3.0,
                "compilation_time" => 2.0,
                "memory_usage_peak" => 2.0,
                "throughput_ops_per_sec" => 3.0,
                _ => 1.0,
            })
            .sum();
        
        if total_weights > 0.0 {
            weighted_sum / total_weights
        } else {
            0.0
        }
    }
    
    fn assess_overall_regression_severity(&self, regressions: &[RegressionEvent]) -> RegressionSeverity {
        if regressions.is_empty() {
            return RegressionSeverity::Negligible;
        }
        
        let max_severity = regressions.iter()
            .map(|r| &r.severity)
            .max_by_key(|s| match s {
                RegressionSeverity::Critical => 5,
                RegressionSeverity::Major => 4,
                RegressionSeverity::Minor => 3,
                RegressionSeverity::Warning => 2,
                RegressionSeverity::Negligible => 1,
            })
            .unwrap_or(&RegressionSeverity::Negligible);
        
        max_severity.clone()
    }
    
    fn record_regression_event(&mut self, event: RegressionEvent) {
        self.regression_history.push_back(event);
        
        // Keep only recent events
        while self.regression_history.len() > 1000 {
            self.regression_history.pop_front();
        }
    }
    
    fn create_baseline_metadata(&self, configuration_id: &str) -> Result<BaselineMetadata> {
        Ok(BaselineMetadata {
            version: "1.0.0".to_string(), // Would be read from actual version
            optimization_level: "O2".to_string(), // Would be determined from config
            target_platform: "x86_64-linux".to_string(), // Would be detected
            compiler_flags: vec!["-O2".to_string()], // Would be read from config
            environment_info: EnvironmentInfo {
                os: "Linux".to_string(),
                arch: "x86_64".to_string(),
                cpu_model: "Unknown".to_string(),
                cpu_cores: 8,
                memory_gb: 16.0,
                compiler_version: "CURSED 1.0".to_string(),
            },
            test_configuration: TestConfiguration {
                test_suite: "standard".to_string(),
                test_parameters: HashMap::new(),
                input_size: 1000,
                iterations: 10,
            },
        })
    }
    
    fn calculate_severity_distribution(&self) -> HashMap<RegressionSeverity, usize> {
        let mut distribution = HashMap::new();
        
        for event in &self.regression_history {
            *distribution.entry(event.severity.clone()).or_insert(0) += 1;
        }
        
        distribution
    }
    
    fn identify_most_affected_metrics(&self) -> Vec<(String, usize)> {
        let mut metric_counts = HashMap::new();
        
        for event in &self.regression_history {
            for metric in &event.affected_metrics {
                *metric_counts.entry(metric.clone()).or_insert(0) += 1;
            }
        }
        
        let mut sorted_metrics: Vec<(String, usize)> = metric_counts.into_iter().collect();
        sorted_metrics.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_metrics.into_iter().take(10).collect()
    }
    
    fn calculate_detection_accuracy(&self) -> f64 {
        // This would be calculated based on false positive/negative rates
        // For now, return a placeholder value
        0.95
    }
}

/// Result of regression detection
#[derive(Debug, Clone)]
pub struct RegressionDetectionResult {
    pub configuration_id: String,
    pub baseline_comparison: BaselineComparison,
    pub detected_regressions: Vec<RegressionEvent>,
    pub statistical_analysis: Option<StatisticalAnalysisResult>,
    pub recommended_actions: Vec<RecommendedAction>,
    pub detection_time: Duration,
    pub overall_assessment: RegressionSeverity,
}

/// Statistical analysis result
#[derive(Debug, Clone)]
pub struct StatisticalAnalysisResult {
    pub overall_significance: StatisticalSignificance,
    pub metric_significance: HashMap<String, StatisticalSignificance>,
    pub outliers_detected: Vec<String>,
    pub variance_analysis: VarianceAnalysis,
}

/// Variance analysis
#[derive(Debug, Clone)]
pub struct VarianceAnalysis {
    pub baseline_variance: f64,
    pub current_variance: f64,
    pub variance_change_percent: f64,
    pub is_variance_stable: bool,
}

/// Regression report
#[derive(Debug, Clone)]
pub struct RegressionReport {
    pub total_regressions: usize,
    pub recent_regressions: usize,
    pub severity_distribution: HashMap<RegressionSeverity, usize>,
    pub most_affected_metrics: Vec<(String, usize)>,
    pub baseline_database_size: usize,
    pub detection_accuracy: f64,
}

impl BaselineDatabase {
    fn new(storage_path: PathBuf) -> Result<Self> {
        let retention_policy = BaselineRetentionPolicy {
            max_baselines_per_config: 50,
            retention_days: 90,
            keep_milestone_versions: true,
            auto_cleanup_enabled: true,
        };
        
        Ok(Self {
            baselines: HashMap::new(),
            baseline_storage_path: storage_path,
            retention_policy,
        })
    }
    
    fn get_baseline(&self, configuration_id: &str) -> Result<PerformanceBaseline> {
        self.baselines.get(configuration_id)
            .cloned()
            .ok_or_else(|| CursedError::runtime_error(&format!("No baseline found for configuration: {}", configuration_id)))
    }
    
    fn store_baseline(&mut self, baseline: PerformanceBaseline) -> Result<()> {
        self.baselines.insert(baseline.configuration_id.clone(), baseline);
        Ok(())
    }
    
    fn get_baseline_count(&self) -> usize {
        self.baselines.len()
    }
}

impl StatisticalAnalyzer {
    fn new(config: StatisticalAnalysisConfig) -> Self {
        Self {
            config,
            sample_buffer: VecDeque::new(),
        }
    }
    
    fn analyze_significance(&self, current: &PerformanceMetrics, baseline: &PerformanceBaseline) -> Result<StatisticalAnalysisResult> {
        // Simplified statistical analysis
        let overall_significance = StatisticalSignificance {
            p_value: 0.03,
            t_statistic: 2.5,
            confidence_level: 0.95,
            is_significant: true,
            effect_size: 0.3,
        };
        
        let mut metric_significance = HashMap::new();
        metric_significance.insert("execution_time".to_string(), overall_significance.clone());
        
        let variance_analysis = VarianceAnalysis {
            baseline_variance: 0.1,
            current_variance: 0.15,
            variance_change_percent: 50.0,
            is_variance_stable: false,
        };
        
        Ok(StatisticalAnalysisResult {
            overall_significance,
            metric_significance,
            outliers_detected: Vec::new(),
            variance_analysis,
        })
    }
    
    fn compute_statistical_summary(&self, metrics: &PerformanceMetrics) -> Result<StatisticalSummary> {
        // Simplified statistical summary computation
        Ok(StatisticalSummary {
            mean: metrics.execution_time.as_secs_f64(),
            median: metrics.execution_time.as_secs_f64(),
            standard_deviation: 0.1,
            variance: 0.01,
            confidence_interval_95: (metrics.execution_time.as_secs_f64() - 0.1, metrics.execution_time.as_secs_f64() + 0.1),
            sample_count: 10,
            outliers_removed: 0,
        })
    }
}

impl Default for RegressionDetectionConfig {
    fn default() -> Self {
        Self {
            regression_threshold_percent: 10.0,
            warning_threshold_percent: 5.0,
            minimum_samples_for_analysis: 5,
            baseline_retention_days: 30,
            enable_statistical_testing: true,
            confidence_level: 0.95,
            enable_automated_rollback: false,
            performance_variance_tolerance: 0.2,
        }
    }
}

impl Default for StatisticalAnalysisConfig {
    fn default() -> Self {
        Self {
            significance_level: 0.05,
            minimum_effect_size: 0.2,
            outlier_detection_method: OutlierDetectionMethod::ZScore(2.5),
            variance_stabilization: true,
            multiple_comparison_correction: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_regression_detection_config_default() {
        let config = RegressionDetectionConfig::default();
        assert_eq!(config.regression_threshold_percent, 10.0);
        assert!(config.enable_statistical_testing);
    }

    #[test]
    fn test_regression_severity_ordering() {
        assert!(RegressionSeverity::Critical != RegressionSeverity::Major);
        assert!(RegressionSeverity::Minor != RegressionSeverity::Warning);
    }

    #[test]
    fn test_performance_metrics_creation() {
        let metrics = PerformanceMetrics {
            compilation_time: Duration::from_secs(5),
            execution_time: Duration::from_secs(1),
            memory_usage_peak: 1024,
            memory_usage_average: 512,
            code_size: 2048,
            optimization_time: Duration::from_millis(500),
            throughput_ops_per_sec: 1000.0,
            custom_metrics: HashMap::new(),
        };
        
        assert_eq!(metrics.compilation_time, Duration::from_secs(5));
        assert_eq!(metrics.throughput_ops_per_sec, 1000.0);
    }

    #[test]
    fn test_baseline_database_creation() {
        let storage_path = PathBuf::from("/tmp/test_baselines");
        let database = BaselineDatabase::new(storage_path);
        assert!(database.is_ok());
    }
}
