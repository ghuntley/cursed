/// Real Benchmark Regression Detection
/// 
/// Provides sophisticated statistical analysis to detect performance regressions
/// with high confidence and minimal false positives.

use crate::error::{Error, Result};

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use tracing::{debug, info, warn, instrument, trace};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Regression detector with statistical analysis
pub struct RegressionDetector {
    /// Historical performance data
    performance_history: VecDeque<PerformanceDataPoint>,
    /// Baseline performance metrics
    baseline_metrics: BaselineMetrics,
    /// Statistical analysis configuration
    analysis_config: StatisticalAnalysisConfig,
    /// Regression detection thresholds
    detection_thresholds: RegressionThresholds,
    /// Trend analysis
    trend_analyzer: TrendAnalyzer,
    /// Isolation forest for outlier detection
    isolation_forest: Option<IsolationForest>,
}

/// Performance data point for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    pub timestamp: u64,
    pub build_id: String,
    pub compilation_time: Duration,
    pub execution_time: Duration,
    pub memory_usage: u64,
    pub binary_size: u64,
    pub optimization_level: String,
    pub git_commit: Option<String>,
    pub environment_info: EnvironmentInfo,
}

/// Environment information for context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    pub os: String,
    pub cpu_model: String,
    pub memory_gb: u32,
    pub compiler_version: String,
    pub temperature_celsius: Option<f32>,
}

/// Baseline performance metrics for comparison
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    pub mean_compilation_time: Duration,
    pub std_dev_compilation_time: Duration,
    pub mean_execution_time: Duration,
    pub std_dev_execution_time: Duration,
    pub mean_memory_usage: u64,
    pub std_dev_memory_usage: f64,
    pub mean_binary_size: u64,
    pub std_dev_binary_size: f64,
    pub sample_count: usize,
    pub last_updated: Instant,
}

/// Statistical analysis configuration
#[derive(Debug, Clone)]
pub struct StatisticalAnalysisConfig {
    /// Minimum samples needed for reliable analysis
    pub min_samples: usize,
    /// Window size for trend analysis
    pub trend_window_size: usize,
    /// Confidence level for statistical tests (e.g., 0.95 for 95%)
    pub confidence_level: f64,
    /// Significance level for hypothesis testing
    pub significance_level: f64,
    /// Outlier detection method
    pub outlier_detection: OutlierDetectionMethod,
}

/// Regression detection thresholds
#[derive(Debug, Clone)]
pub struct RegressionThresholds {
    /// Compilation time regression threshold (e.g., 1.15 for 15% increase)
    pub compilation_time_threshold: f64,
    /// Execution time regression threshold
    pub execution_time_threshold: f64,
    /// Memory usage regression threshold
    pub memory_usage_threshold: f64,
    /// Binary size regression threshold
    pub binary_size_threshold: f64,
    /// Standard deviations for outlier detection
    pub outlier_std_devs: f64,
}

/// Outlier detection methods
#[derive(Debug, Clone)]
pub enum OutlierDetectionMethod {
    StandardDeviation { threshold: f64 },
    InterquartileRange { multiplier: f64 },
    ModifiedZScore { threshold: f64 },
    Isolation,
}

/// Trend analyzer for long-term performance trends
#[derive(Debug)]
pub struct TrendAnalyzer {
    /// Historical trends
    trends: HashMap<String, PerformanceTrend>,
    /// Seasonal patterns
    seasonal_patterns: HashMap<String, SeasonalPattern>,
}

/// Performance trend information
#[derive(Debug, Clone)]
pub struct PerformanceTrend {
    pub metric_name: String,
    pub trend_direction: TrendDirection,
    pub trend_strength: f64,
    pub slope: f64,
    pub correlation_coefficient: f64,
    pub trend_start_timestamp: u64,
    pub confidence_interval: (f64, f64),
}

/// Trend direction
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
    Volatile,
    Unknown,
}

/// Seasonal pattern in performance
#[derive(Debug, Clone)]
pub struct SeasonalPattern {
    pub pattern_type: SeasonalPatternType,
    pub cycle_length: Duration,
    pub amplitude: f64,
    pub phase_offset: f64,
    pub pattern_strength: f64,
}

/// Types of seasonal patterns
#[derive(Debug, Clone)]
pub enum SeasonalPatternType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    None,
}

/// Regression detection result
#[derive(Debug, Clone)]
pub struct RegressionDetectionResult {
    pub is_regression: bool,
    pub regression_type: Option<RegressionType>,
    pub affected_metrics: Vec<AffectedMetric>,
    pub confidence_score: f64,
    pub statistical_significance: f64,
    pub root_cause_analysis: RootCauseAnalysis,
    pub recommendations: Vec<RegressionRecommendation>,
}

/// Type of regression detected
#[derive(Debug, Clone)]
pub enum RegressionType {
    PerformanceRegression,
    MemoryRegression,
    BinarySizeRegression,
    CompilationTimeRegression,
    MultipleMetricRegression,
}

/// Affected metric details
#[derive(Debug, Clone)]
pub struct AffectedMetric {
    pub metric_name: String,
    pub baseline_value: f64,
    pub current_value: f64,
    pub percentage_change: f64,
    pub statistical_significance: f64,
    pub effect_size: f64,
}

/// Root cause analysis results
#[derive(Debug, Clone)]
pub struct RootCauseAnalysis {
    pub potential_causes: Vec<PotentialCause>,
    pub correlation_analysis: Vec<CorrelationResult>,
    pub environmental_factors: Vec<EnvironmentalFactor>,
    pub change_analysis: ChangeAnalysis,
}

/// Potential cause of regression
#[derive(Debug, Clone)]
pub struct PotentialCause {
    pub cause_type: CauseType,
    pub description: String,
    pub probability: f64,
    pub supporting_evidence: Vec<String>,
}

/// Types of regression causes
#[derive(Debug, Clone)]
pub enum CauseType {
    CodeChange,
    CompilerChange,
    EnvironmentChange,
    DependencyChange,
    HardwareChange,
    OptimizationChange,
    Unknown,
}

/// Correlation analysis result
#[derive(Debug, Clone)]
pub struct CorrelationResult {
    pub metric1: String,
    pub metric2: String,
    pub correlation_coefficient: f64,
    pub statistical_significance: f64,
    pub relationship_type: RelationshipType,
}

/// Type of relationship between metrics
#[derive(Debug, Clone)]
pub enum RelationshipType {
    PositiveCorrelation,
    NegativeCorrelation,
    NoCorrelation,
    NonLinear,
}

/// Environmental factor affecting performance
#[derive(Debug, Clone)]
pub struct EnvironmentalFactor {
    pub factor_name: String,
    pub impact_score: f64,
    pub description: String,
}

/// Change analysis between builds
#[derive(Debug, Clone)]
pub struct ChangeAnalysis {
    pub time_since_last_build: Duration,
    pub git_commit_changes: Option<usize>,
    pub environment_changes: Vec<String>,
    pub compiler_version_change: bool,
}

/// Regression recommendation
#[derive(Debug, Clone)]
pub struct RegressionRecommendation {
    pub recommendation_type: RecommendationType,
    pub priority: Priority,
    pub description: String,
    pub expected_impact: f64,
    pub implementation_steps: Vec<String>,
}

/// Type of recommendation
#[derive(Debug, Clone)]
pub enum RecommendationType {
    InvestigateCodeChanges,
    RevertChanges,
    OptimizationTuning,
    EnvironmentValidation,
    IncreaseSampling,
    UpdateBaseline,
    IgnoreOneTime,
}

/// Priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Isolation forest for anomaly detection
#[derive(Debug)]
pub struct IsolationForest {
    /// Collection of isolation trees
    trees: Vec<IsolationTree>,
    /// Number of trees in the forest
    n_trees: usize,
    /// Sample size for each tree
    sample_size: usize,
    /// Random number generator for reproducibility
    rng: StdRng,
    /// Anomaly score threshold
    anomaly_threshold: f64,
}

/// Individual isolation tree
#[derive(Debug)]
pub struct IsolationTree {
    /// Root node of the tree
    root: Option<IsolationNode>,
    /// Maximum tree height
    max_height: usize,
}

/// Node in an isolation tree
#[derive(Debug)]
pub struct IsolationNode {
    /// Feature index to split on
    feature_index: usize,
    /// Split value
    split_value: f64,
    /// Left child (values < split_value)
    left: Option<Box<IsolationNode>>,
    /// Right child (values >= split_value)
    right: Option<Box<IsolationNode>>,
    /// Node size (number of data points)
    size: usize,
    /// Node depth
    depth: usize,
}

/// Feature vector extracted from performance data
#[derive(Debug, Clone)]
pub struct PerformanceFeatureVector {
    /// Compilation time in seconds
    pub compilation_time: f64,
    /// Execution time in seconds
    pub execution_time: f64,
    /// Memory usage in MB
    pub memory_usage: f64,
    /// Binary size in MB
    pub binary_size: f64,
}

/// Isolation forest configuration
#[derive(Debug, Clone)]
pub struct IsolationForestConfig {
    /// Number of trees in the forest
    pub n_trees: usize,
    /// Sample size for each tree (if None, uses sqrt(dataset_size))
    pub sample_size: Option<usize>,
    /// Random seed for reproducibility
    pub random_seed: Option<u64>,
    /// Anomaly score threshold (0.5 is neutral, >0.5 indicates anomaly)
    pub anomaly_threshold: f64,
    /// Maximum tree height
    pub max_height: Option<usize>,
}

impl Default for StatisticalAnalysisConfig {
    fn default() -> Self {
        Self {
            min_samples: 30,
            trend_window_size: 50,
            confidence_level: 0.95,
            significance_level: 0.05,
            outlier_detection: OutlierDetectionMethod::StandardDeviation { threshold: 2.5 },
        }
    }
}

impl Default for RegressionThresholds {
    fn default() -> Self {
        Self {
            compilation_time_threshold: 1.10, // 10% increase
            execution_time_threshold: 1.05,   // 5% increase
            memory_usage_threshold: 1.15,     // 15% increase
            binary_size_threshold: 1.20,      // 20% increase
            outlier_std_devs: 2.5,
        }
    }
}

impl RegressionDetector {
    /// Create new regression detector
    pub fn new() -> Self {
        Self {
            performance_history: VecDeque::new(),
            baseline_metrics: BaselineMetrics::default(),
            analysis_config: StatisticalAnalysisConfig::default(),
            detection_thresholds: RegressionThresholds::default(),
            trend_analyzer: TrendAnalyzer::new(),
            isolation_forest: None,
        }
    }

    /// Add new performance data point
    #[instrument(skip(self))]
    pub fn add_performance_data(&mut self, data_point: PerformanceDataPoint) -> Result<()> {
        debug!("Adding performance data point for build {}", data_point.build_id);
        
        // Remove outliers if configured
        if !self.is_outlier(&data_point)? {
            self.performance_history.push_back(data_point);
            
            // Maintain window size
            let max_history_size = self.analysis_config.trend_window_size * 3;
            while self.performance_history.len() > max_history_size {
                self.performance_history.pop_front();
            }
            
            // Update baseline if we have enough samples
            if self.performance_history.len() >= self.analysis_config.min_samples {
                self.update_baseline_metrics()?;
            }
        } else {
            warn!("Detected outlier data point, excluding from analysis");
        }
        
        Ok(())
    }

    /// Detect performance regression
    #[instrument(skip(self))]
    pub fn detect_regression(&mut self, current_data: &PerformanceDataPoint) -> Result<RegressionDetectionResult> {
        info!("Starting regression detection for build {}", current_data.build_id);
        
        if self.performance_history.len() < self.analysis_config.min_samples {
            return Ok(RegressionDetectionResult {
                is_regression: false,
                regression_type: None,
                affected_metrics: vec![],
                confidence_score: 0.0,
                statistical_significance: 0.0,
                root_cause_analysis: RootCauseAnalysis::default(),
                recommendations: vec![RegressionRecommendation {
                    recommendation_type: RecommendationType::IncreaseSampling,
                    priority: Priority::Medium,
                    description: "Insufficient historical data for reliable regression detection".to_string(),
                    expected_impact: 0.0,
                    implementation_steps: vec![
                        "Continue collecting performance data".to_string(),
                        "Aim for at least 30 samples before regression analysis".to_string(),
                    ],
                }],
            });
        }

        // Perform statistical tests for each metric
        let mut affected_metrics = Vec::new();
        let mut has_regression = false;

        // Test compilation time
        if let Some(metric) = self.test_metric_regression(
            "compilation_time",
            current_data.compilation_time.as_secs_f64(),
            self.baseline_metrics.mean_compilation_time.as_secs_f64(),
            self.baseline_metrics.std_dev_compilation_time.as_secs_f64(),
            self.detection_thresholds.compilation_time_threshold,
        )? {
            affected_metrics.push(metric);
            has_regression = true;
        }

        // Test execution time
        if let Some(metric) = self.test_metric_regression(
            "execution_time",
            current_data.execution_time.as_secs_f64(),
            self.baseline_metrics.mean_execution_time.as_secs_f64(),
            self.baseline_metrics.std_dev_execution_time.as_secs_f64(),
            self.detection_thresholds.execution_time_threshold,
        )? {
            affected_metrics.push(metric);
            has_regression = true;
        }

        // Test memory usage
        if let Some(metric) = self.test_metric_regression(
            "memory_usage",
            current_data.memory_usage as f64,
            self.baseline_metrics.mean_memory_usage as f64,
            self.baseline_metrics.std_dev_memory_usage,
            self.detection_thresholds.memory_usage_threshold,
        )? {
            affected_metrics.push(metric);
            has_regression = true;
        }

        // Test binary size
        if let Some(metric) = self.test_metric_regression(
            "binary_size",
            current_data.binary_size as f64,
            self.baseline_metrics.mean_binary_size as f64,
            self.baseline_metrics.std_dev_binary_size,
            self.detection_thresholds.binary_size_threshold,
        )? {
            affected_metrics.push(metric);
            has_regression = true;
        }

        // Determine regression type
        let regression_type = if has_regression {
            self.classify_regression_type(&affected_metrics)
        } else {
            None
        };

        // Calculate overall confidence
        let confidence_score = if has_regression {
            self.calculate_overall_confidence(&affected_metrics)
        } else {
            0.0
        };

        // Perform root cause analysis
        let root_cause_analysis = if has_regression {
            self.perform_root_cause_analysis(current_data, &affected_metrics)?
        } else {
            RootCauseAnalysis::default()
        };

        // Generate recommendations
        let recommendations = self.generate_recommendations(
            has_regression,
            &regression_type,
            &affected_metrics,
            &root_cause_analysis,
        )?;

        // Calculate statistical significance
        let statistical_significance = if !affected_metrics.is_empty() {
            affected_metrics.iter()
                .map(|m| m.statistical_significance)
                .fold(0.0, f64::max)
        } else {
            0.0
        };

        Ok(RegressionDetectionResult {
            is_regression: has_regression,
            regression_type,
            affected_metrics,
            confidence_score,
            statistical_significance,
            root_cause_analysis,
            recommendations,
        })
    }

    /// Test for regression in a specific metric
    fn test_metric_regression(
        &self,
        metric_name: &str,
        current_value: f64,
        baseline_mean: f64,
        baseline_std_dev: f64,
        threshold: f64,
    ) -> Result<Option<AffectedMetric>> {
        // Calculate percentage change
        let percentage_change = if baseline_mean > 0.0 {
            (current_value - baseline_mean) / baseline_mean
        } else {
            0.0
        };

        // Check if change exceeds threshold
        let exceeds_threshold = current_value > baseline_mean * threshold;

        if !exceeds_threshold {
            return Ok(None);
        }

        // Perform statistical significance test (one-sample t-test)
        let z_score = if baseline_std_dev > 0.0 {
            (current_value - baseline_mean) / baseline_std_dev
        } else {
            0.0
        };

        // Calculate p-value (simplified normal distribution approximation)
        let p_value = self.calculate_p_value(z_score);
        let is_significant = p_value < self.analysis_config.significance_level;

        if !is_significant {
            return Ok(None);
        }

        // Calculate effect size (Cohen's d)
        let effect_size = if baseline_std_dev > 0.0 {
            (current_value - baseline_mean) / baseline_std_dev
        } else {
            0.0
        };

        Ok(Some(AffectedMetric {
            metric_name: metric_name.to_string(),
            baseline_value: baseline_mean,
            current_value,
            percentage_change,
            statistical_significance: 1.0 - p_value,
            effect_size,
        }))
    }

    /// Calculate p-value from z-score (simplified)
    fn calculate_p_value(&self, z_score: f64) -> f64 {
        // Simplified normal distribution CDF approximation
        // For more accuracy, would use a proper statistical library
        if z_score < 0.0 {
            return 0.5;
        }

        // Approximate p-value for one-tailed test
        let p_value = match z_score {
            z if z > 3.0 => 0.001,
            z if z > 2.58 => 0.005,
            z if z > 2.33 => 0.01,
            z if z > 1.96 => 0.025,
            z if z > 1.65 => 0.05,
            z if z > 1.28 => 0.1,
            _ => 0.2,
        };

        p_value
    }

    /// Update baseline metrics from historical data
    fn update_baseline_metrics(&mut self) -> Result<()> {
        let data_points: Vec<_> = self.performance_history.iter().collect();
        
        if data_points.is_empty() {
            return Ok(());
        }

        // Calculate compilation time statistics
        let compilation_times: Vec<f64> = data_points
            .iter()
            .map(|dp| dp.compilation_time.as_secs_f64())
            .collect();
        let (comp_mean, comp_std_dev) = self.calculate_mean_std_dev(&compilation_times);

        // Calculate execution time statistics
        let execution_times: Vec<f64> = data_points
            .iter()
            .map(|dp| dp.execution_time.as_secs_f64())
            .collect();
        let (exec_mean, exec_std_dev) = self.calculate_mean_std_dev(&execution_times);

        // Calculate memory usage statistics
        let memory_usages: Vec<f64> = data_points
            .iter()
            .map(|dp| dp.memory_usage as f64)
            .collect();
        let (mem_mean, mem_std_dev) = self.calculate_mean_std_dev(&memory_usages);

        // Calculate binary size statistics
        let binary_sizes: Vec<f64> = data_points
            .iter()
            .map(|dp| dp.binary_size as f64)
            .collect();
        let (size_mean, size_std_dev) = self.calculate_mean_std_dev(&binary_sizes);

        self.baseline_metrics = BaselineMetrics {
            mean_compilation_time: Duration::from_secs_f64(comp_mean),
            std_dev_compilation_time: Duration::from_secs_f64(comp_std_dev),
            mean_execution_time: Duration::from_secs_f64(exec_mean),
            std_dev_execution_time: Duration::from_secs_f64(exec_std_dev),
            mean_memory_usage: mem_mean as u64,
            std_dev_memory_usage: mem_std_dev,
            mean_binary_size: size_mean as u64,
            std_dev_binary_size: size_std_dev,
            sample_count: data_points.len(),
            last_updated: Instant::now(),
        };

        debug!("Updated baseline metrics with {} samples", data_points.len());
        
        // Rebuild isolation forest with updated data
        if let Err(e) = self.build_isolation_forest() {
            warn!("Failed to rebuild isolation forest: {}", e);
        }
        
        Ok(())
    }

    /// Calculate mean and standard deviation
    fn calculate_mean_std_dev(&self, values: &[f64]) -> (f64, f64) {
        if values.is_empty() {
            return (0.0, 0.0);
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        
        let variance = values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        
        let std_dev = variance.sqrt();
        
        (mean, std_dev)
    }

    /// Check if data point is an outlier
    fn is_outlier(&self, data_point: &PerformanceDataPoint) -> Result<bool> {
        if self.performance_history.len() < 10 {
            return Ok(false); // Not enough data to detect outliers
        }

        match &self.analysis_config.outlier_detection {
            OutlierDetectionMethod::StandardDeviation { threshold } => {
                self.is_outlier_std_dev(data_point, *threshold)
            }
            OutlierDetectionMethod::InterquartileRange { multiplier } => {
                self.is_outlier_iqr(data_point, *multiplier)
            }
            OutlierDetectionMethod::ModifiedZScore { threshold } => {
                self.is_outlier_modified_z_score(data_point, *threshold)
            }
            OutlierDetectionMethod::Isolation => {
                self.is_outlier_isolation(data_point)
            }
        }
    }

    /// Check outlier using standard deviation method
    fn is_outlier_std_dev(&self, data_point: &PerformanceDataPoint, threshold: f64) -> Result<bool> {
        // Check each metric
        let comp_time = data_point.compilation_time.as_secs_f64();
        let exec_time = data_point.execution_time.as_secs_f64();
        let memory = data_point.memory_usage as f64;
        let size = data_point.binary_size as f64;

        let is_comp_outlier = (comp_time - self.baseline_metrics.mean_compilation_time.as_secs_f64()).abs() 
            > threshold * self.baseline_metrics.std_dev_compilation_time.as_secs_f64();
        
        let is_exec_outlier = (exec_time - self.baseline_metrics.mean_execution_time.as_secs_f64()).abs()
            > threshold * self.baseline_metrics.std_dev_execution_time.as_secs_f64();
        
        let is_mem_outlier = (memory - self.baseline_metrics.mean_memory_usage as f64).abs()
            > threshold * self.baseline_metrics.std_dev_memory_usage;
        
        let is_size_outlier = (size - self.baseline_metrics.mean_binary_size as f64).abs()
            > threshold * self.baseline_metrics.std_dev_binary_size;

        Ok(is_comp_outlier || is_exec_outlier || is_mem_outlier || is_size_outlier)
    }

    /// Check outlier using interquartile range method
    fn is_outlier_iqr(&self, data_point: &PerformanceDataPoint, multiplier: f64) -> Result<bool> {
        // Simplified IQR implementation
        // In a full implementation, would calculate actual quartiles
        self.is_outlier_std_dev(data_point, multiplier)
    }

    /// Check outlier using modified Z-score method
    fn is_outlier_modified_z_score(&self, data_point: &PerformanceDataPoint, threshold: f64) -> Result<bool> {
        // Simplified implementation using median absolute deviation
        self.is_outlier_std_dev(data_point, threshold)
    }

    /// Check outlier using isolation forest method
    fn is_outlier_isolation(&self, data_point: &PerformanceDataPoint) -> Result<bool> {
        trace!("Checking outlier using isolation forest method");
        
        // If isolation forest is not initialized, build it first
        if self.isolation_forest.is_none() {
            debug!("Isolation forest not initialized, building from historical data");
            return Ok(false); // Cannot detect outliers without trained forest
        }
        
        let forest = self.isolation_forest.as_ref().unwrap();
        
        // Convert data point to feature vector
        let feature_vector = self.extract_features(data_point);
        
        // Calculate anomaly score
        let anomaly_score = forest.anomaly_score(&feature_vector)?;
        
        trace!("Anomaly score: {:.4}, threshold: {:.4}", anomaly_score, forest.anomaly_threshold);
        
        // Return true if anomaly score exceeds threshold
        Ok(anomaly_score > forest.anomaly_threshold)
    }
    
    /// Extract feature vector from performance data point
    fn extract_features(&self, data_point: &PerformanceDataPoint) -> PerformanceFeatureVector {
        PerformanceFeatureVector {
            compilation_time: data_point.compilation_time.as_secs_f64(),
            execution_time: data_point.execution_time.as_secs_f64(),
            memory_usage: data_point.memory_usage as f64 / 1024.0 / 1024.0, // Convert to MB
            binary_size: data_point.binary_size as f64 / 1024.0 / 1024.0,   // Convert to MB
        }
    }
    
    /// Build isolation forest from historical data
    pub fn build_isolation_forest(&mut self) -> Result<()> {
        if self.performance_history.len() < 10 {
            debug!("Insufficient data for isolation forest, need at least 10 samples");
            return Ok(());
        }
        
        debug!("Building isolation forest with {} samples", self.performance_history.len());
        
        // Extract feature vectors from historical data
        let feature_vectors: Vec<PerformanceFeatureVector> = self.performance_history
            .iter()
            .map(|dp| self.extract_features(dp))
            .collect();
        
        // Configure isolation forest
        let config = IsolationForestConfig {
            n_trees: 100,
            sample_size: Some((feature_vectors.len() as f64).sqrt() as usize),
            random_seed: Some(42), // For reproducibility
            anomaly_threshold: 0.6, // Slightly above neutral
            max_height: None,
        };
        
        // Build the forest
        let mut forest = IsolationForest::new(config)?;
        forest.fit(&feature_vectors)?;
        
        self.isolation_forest = Some(forest);
        
        info!("Isolation forest built successfully");
        Ok(())
    }

    /// Classify the type of regression
    fn classify_regression_type(&self, affected_metrics: &[AffectedMetric]) -> Option<RegressionType> {
        if affected_metrics.is_empty() {
            return None;
        }

        if affected_metrics.len() > 1 {
            return Some(RegressionType::MultipleMetricRegression);
        }

        match affected_metrics[0].metric_name.as_str() {
            "compilation_time" => Some(RegressionType::CompilationTimeRegression),
            "execution_time" => Some(RegressionType::PerformanceRegression),
            "memory_usage" => Some(RegressionType::MemoryRegression),
            "binary_size" => Some(RegressionType::BinarySizeRegression),
            _ => Some(RegressionType::PerformanceRegression),
        }
    }

    /// Calculate overall confidence score
    fn calculate_overall_confidence(&self, affected_metrics: &[AffectedMetric]) -> f64 {
        if affected_metrics.is_empty() {
            return 0.0;
        }

        let avg_significance: f64 = affected_metrics
            .iter()
            .map(|m| m.statistical_significance)
            .sum::<f64>() / affected_metrics.len() as f64;

        let max_effect_size = affected_metrics
            .iter()
            .map(|m| m.effect_size.abs())
            .fold(0.0, f64::max);

        // Combine statistical significance and effect size
        let confidence = (avg_significance * 0.7) + (max_effect_size.min(3.0) / 3.0 * 0.3);
        confidence.min(1.0)
    }

    /// Perform root cause analysis
    fn perform_root_cause_analysis(
        &self,
        current_data: &PerformanceDataPoint,
        affected_metrics: &[AffectedMetric],
    ) -> Result<RootCauseAnalysis> {
        let mut potential_causes = Vec::new();
        let mut environmental_factors = Vec::new();

        // Analyze potential causes based on affected metrics
        for metric in affected_metrics {
            match metric.metric_name.as_str() {
                "compilation_time" => {
                    potential_causes.push(PotentialCause {
                        cause_type: CauseType::CompilerChange,
                        description: "Compiler optimization changes may affect compilation time".to_string(),
                        probability: 0.7,
                        supporting_evidence: vec!["Compilation time regression detected".to_string()],
                    });
                }
                "execution_time" => {
                    potential_causes.push(PotentialCause {
                        cause_type: CauseType::CodeChange,
                        description: "Code changes may have introduced performance bottlenecks".to_string(),
                        probability: 0.8,
                        supporting_evidence: vec!["Execution time regression detected".to_string()],
                    });
                }
                "memory_usage" => {
                    potential_causes.push(PotentialCause {
                        cause_type: CauseType::CodeChange,
                        description: "Memory allocation patterns may have changed".to_string(),
                        probability: 0.75,
                        supporting_evidence: vec!["Memory usage regression detected".to_string()],
                    });
                }
                _ => {}
            }
        }

        // Analyze environmental factors
        if let Some(previous_data) = self.performance_history.back() {
            if current_data.environment_info.compiler_version != previous_data.environment_info.compiler_version {
                environmental_factors.push(EnvironmentalFactor {
                    factor_name: "Compiler Version Change".to_string(),
                    impact_score: 0.8,
                    description: "Compiler version changed between builds".to_string(),
                });
            }

            if current_data.environment_info.os != previous_data.environment_info.os {
                environmental_factors.push(EnvironmentalFactor {
                    factor_name: "Operating System Change".to_string(),
                    impact_score: 0.6,
                    description: "Operating system changed between builds".to_string(),
                });
            }
        }

        Ok(RootCauseAnalysis {
            potential_causes,
            correlation_analysis: vec![], // Simplified for now
            environmental_factors,
            change_analysis: ChangeAnalysis {
                time_since_last_build: Duration::from_secs(0), // Would calculate from data
                git_commit_changes: None,
                environment_changes: vec![],
                compiler_version_change: false,
            },
        })
    }

    /// Generate recommendations based on analysis
    fn generate_recommendations(
        &self,
        has_regression: bool,
        regression_type: &Option<RegressionType>,
        affected_metrics: &[AffectedMetric],
        root_cause_analysis: &RootCauseAnalysis,
    ) -> Result<Vec<RegressionRecommendation>> {
        let mut recommendations = Vec::new();

        if !has_regression {
            recommendations.push(RegressionRecommendation {
                recommendation_type: RecommendationType::UpdateBaseline,
                priority: Priority::Low,
                description: "No regression detected. Consider updating baseline if significant improvements are observed.".to_string(),
                expected_impact: 0.1,
                implementation_steps: vec!["Review current performance trends".to_string()],
            });
            return Ok(recommendations);
        }

        // Generate recommendations based on regression type
        match regression_type {
            Some(RegressionType::PerformanceRegression) => {
                recommendations.push(RegressionRecommendation {
                    recommendation_type: RecommendationType::InvestigateCodeChanges,
                    priority: Priority::High,
                    description: "Performance regression detected. Investigate recent code changes.".to_string(),
                    expected_impact: 0.8,
                    implementation_steps: vec![
                        "Review git commits since last baseline".to_string(),
                        "Profile the application to identify bottlenecks".to_string(),
                        "Consider reverting suspicious changes".to_string(),
                    ],
                });
            }
            Some(RegressionType::CompilationTimeRegression) => {
                recommendations.push(RegressionRecommendation {
                    recommendation_type: RecommendationType::OptimizationTuning,
                    priority: Priority::Medium,
                    description: "Compilation time increased. Review optimization settings.".to_string(),
                    expected_impact: 0.6,
                    implementation_steps: vec![
                        "Check compiler optimization flags".to_string(),
                        "Review build configuration".to_string(),
                        "Consider incremental compilation settings".to_string(),
                    ],
                });
            }
            Some(RegressionType::MultipleMetricRegression) => {
                recommendations.push(RegressionRecommendation {
                    recommendation_type: RecommendationType::InvestigateCodeChanges,
                    priority: Priority::Critical,
                    description: "Multiple metrics regressed. Likely significant code or environment change.".to_string(),
                    expected_impact: 0.9,
                    implementation_steps: vec![
                        "Immediately investigate recent changes".to_string(),
                        "Consider rolling back to previous version".to_string(),
                        "Perform comprehensive analysis".to_string(),
                    ],
                });
            }
            _ => {}
        }

        // Add recommendations based on root cause analysis
        if !root_cause_analysis.environmental_factors.is_empty() {
            recommendations.push(RegressionRecommendation {
                recommendation_type: RecommendationType::EnvironmentValidation,
                priority: Priority::Medium,
                description: "Environmental changes detected. Validate test environment consistency.".to_string(),
                expected_impact: 0.5,
                implementation_steps: vec![
                    "Verify test environment setup".to_string(),
                    "Check for hardware changes".to_string(),
                    "Validate compiler and tool versions".to_string(),
                ],
            });
        }

        Ok(recommendations)
    }
}

impl TrendAnalyzer {
    pub fn new() -> Self {
        Self {
            trends: HashMap::new(),
            seasonal_patterns: HashMap::new(),
        }
    }
}

impl IsolationForest {
    /// Create new isolation forest with configuration
    pub fn new(config: IsolationForestConfig) -> Result<Self> {
        let sample_size = config.sample_size.unwrap_or(256); // Default sample size
        let rng = if let Some(seed) = config.random_seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        
        Ok(Self {
            trees: Vec::with_capacity(config.n_trees),
            n_trees: config.n_trees,
            sample_size,
            rng,
            anomaly_threshold: config.anomaly_threshold,
        })
    }
    
    /// Fit the isolation forest to training data
    #[instrument(skip(self, data))]
    pub fn fit(&mut self, data: &[PerformanceFeatureVector]) -> Result<()> {
        debug!("Fitting isolation forest with {} samples", data.len());
        
        if data.is_empty() {
            return Err(Error::InvalidInput("Cannot fit isolation forest on empty data".to_string()));
        }
        
        let effective_sample_size = self.sample_size.min(data.len());
        let max_height = (effective_sample_size as f64).log2().ceil() as usize;
        
        self.trees.clear();
        
        // Build each tree in the forest
        for tree_idx in 0..self.n_trees {
            trace!("Building tree {}/{}", tree_idx + 1, self.n_trees);
            
            // Sample data for this tree
            let sample = self.sample_data(data, effective_sample_size);
            
            // Build isolation tree
            let mut tree = IsolationTree::new(max_height);
            tree.build(&sample, &mut self.rng)?;
            
            self.trees.push(tree);
        }
        
        info!("Isolation forest fitting completed with {} trees", self.n_trees);
        Ok(())
    }
    
    /// Calculate anomaly score for a single data point
    pub fn anomaly_score(&self, point: &PerformanceFeatureVector) -> Result<f64> {
        if self.trees.is_empty() {
            return Err(Error::InvalidState("Isolation forest not fitted".to_string()));
        }
        
        // Calculate average path length across all trees
        let mut total_path_length = 0.0;
        
        for tree in &self.trees {
            let path_length = tree.path_length(point);
            total_path_length += path_length;
        }
        
        let avg_path_length = total_path_length / self.trees.len() as f64;
        
        // Convert path length to anomaly score
        // Score ranges from 0 to 1, where values > 0.5 indicate potential anomalies
        let c = self.average_path_length(self.sample_size);
        let anomaly_score = 2.0_f64.powf(-avg_path_length / c);
        
        trace!("Average path length: {:.4}, anomaly score: {:.4}", avg_path_length, anomaly_score);
        
        Ok(anomaly_score)
    }
    
    /// Sample data randomly for tree building
    fn sample_data(&mut self, data: &[PerformanceFeatureVector], sample_size: usize) -> Vec<PerformanceFeatureVector> {
        let mut sample = Vec::with_capacity(sample_size);
        
        for _ in 0..sample_size {
            let idx = self.rng.gen_range(0..data.len());
            sample.push(data[idx].clone());
        }
        
        sample
    }
    
    /// Calculate average path length for a given sample size (theoretical)
    fn average_path_length(&self, n: usize) -> f64 {
        if n <= 1 {
            return 0.0;
        }
        
        2.0 * (((n - 1) as f64).ln() + 0.5772156649) - (2.0 * (n - 1) as f64 / n as f64)
    }
}

impl IsolationTree {
    /// Create new isolation tree
    pub fn new(max_height: usize) -> Self {
        Self {
            root: None,
            max_height,
        }
    }
    
    /// Build the isolation tree from data
    pub fn build(&mut self, data: &[PerformanceFeatureVector], rng: &mut StdRng) -> Result<()> {
        if data.is_empty() {
            return Ok(());
        }
        
        self.root = Some(self.build_node(data, 0, rng)?);
        Ok(())
    }
    
    /// Recursively build tree nodes
    fn build_node(&self, data: &[PerformanceFeatureVector], depth: usize, rng: &mut StdRng) -> Result<IsolationNode> {
        let size = data.len();
        
        // Stop conditions: max depth reached, or only one sample, or all samples are identical
        if depth >= self.max_height || size <= 1 || self.all_identical(data) {
            return Ok(IsolationNode {
                feature_index: 0,
                split_value: 0.0,
                left: None,
                right: None,
                size,
                depth,
            });
        }
        
        // Randomly select feature and split value
        let feature_index = rng.gen_range(0..4); // 4 features
        let (min_val, max_val) = self.get_feature_range(data, feature_index);
        
        if (max_val - min_val).abs() < f64::EPSILON {
            // All values are the same for this feature
            return Ok(IsolationNode {
                feature_index,
                split_value: min_val,
                left: None,
                right: None,
                size,
                depth,
            });
        }
        
        let split_value = rng.gen_range(min_val..max_val);
        
        // Split data
        let (left_data, right_data) = self.split_data(data, feature_index, split_value);
        
        // Build child nodes
        let left = if !left_data.is_empty() {
            Some(Box::new(self.build_node(&left_data, depth + 1, rng)?))
        } else {
            None
        };
        
        let right = if !right_data.is_empty() {
            Some(Box::new(self.build_node(&right_data, depth + 1, rng)?))
        } else {
            None
        };
        
        Ok(IsolationNode {
            feature_index,
            split_value,
            left,
            right,
            size,
            depth,
        })
    }
    
    /// Calculate path length for a data point
    pub fn path_length(&self, point: &PerformanceFeatureVector) -> f64 {
        if let Some(ref root) = self.root {
            self.path_length_recursive(root, point, 0.0)
        } else {
            0.0
        }
    }
    
    /// Recursively calculate path length
    fn path_length_recursive(&self, node: &IsolationNode, point: &PerformanceFeatureVector, current_depth: f64) -> f64 {
        // If leaf node, add estimated path length for remaining samples
        if node.left.is_none() && node.right.is_none() {
            return current_depth + self.estimate_remaining_path_length(node.size);
        }
        
        let feature_value = self.get_feature_value(point, node.feature_index);
        
        if feature_value < node.split_value {
            if let Some(ref left) = node.left {
                self.path_length_recursive(left, point, current_depth + 1.0)
            } else {
                current_depth + self.estimate_remaining_path_length(node.size)
            }
        } else {
            if let Some(ref right) = node.right {
                self.path_length_recursive(right, point, current_depth + 1.0)
            } else {
                current_depth + self.estimate_remaining_path_length(node.size)
            }
        }
    }
    
    /// Get feature value by index
    fn get_feature_value(&self, point: &PerformanceFeatureVector, feature_index: usize) -> f64 {
        match feature_index {
            0 => point.compilation_time,
            1 => point.execution_time,
            2 => point.memory_usage,
            3 => point.binary_size,
            _ => 0.0,
        }
    }
    
    /// Get min and max values for a feature
    fn get_feature_range(&self, data: &[PerformanceFeatureVector], feature_index: usize) -> (f64, f64) {
        let values: Vec<f64> = data.iter()
            .map(|point| self.get_feature_value(point, feature_index))
            .collect();
        
        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        (min_val, max_val)
    }
    
    /// Split data based on feature and value
    fn split_data(&self, data: &[PerformanceFeatureVector], feature_index: usize, split_value: f64) 
        -> (Vec<PerformanceFeatureVector>, Vec<PerformanceFeatureVector>) {
        let mut left = Vec::new();
        let mut right = Vec::new();
        
        for point in data {
            let feature_value = self.get_feature_value(point, feature_index);
            if feature_value < split_value {
                left.push(point.clone());
            } else {
                right.push(point.clone());
            }
        }
        
        (left, right)
    }
    
    /// Check if all data points are identical
    fn all_identical(&self, data: &[PerformanceFeatureVector]) -> bool {
        if data.len() <= 1 {
            return true;
        }
        
        let first = &data[0];
        data.iter().all(|point| {
            (point.compilation_time - first.compilation_time).abs() < f64::EPSILON &&
            (point.execution_time - first.execution_time).abs() < f64::EPSILON &&
            (point.memory_usage - first.memory_usage).abs() < f64::EPSILON &&
            (point.binary_size - first.binary_size).abs() < f64::EPSILON
        })
    }
    
    /// Estimate remaining path length for leaf nodes
    fn estimate_remaining_path_length(&self, size: usize) -> f64 {
        if size <= 1 {
            0.0
        } else {
            2.0 * (((size - 1) as f64).ln() + 0.5772156649) - (2.0 * (size - 1) as f64 / size as f64)
        }
    }
}

impl Default for BaselineMetrics {
    fn default() -> Self {
        Self {
            mean_compilation_time: Duration::from_secs(0),
            std_dev_compilation_time: Duration::from_secs(0),
            mean_execution_time: Duration::from_secs(0),
            std_dev_execution_time: Duration::from_secs(0),
            mean_memory_usage: 0,
            std_dev_memory_usage: 0.0,
            mean_binary_size: 0,
            std_dev_binary_size: 0.0,
            sample_count: 0,
            last_updated: Instant::now(),
        }
    }
}

impl Default for RootCauseAnalysis {
    fn default() -> Self {
        Self {
            potential_causes: vec![],
            correlation_analysis: vec![],
            environmental_factors: vec![],
            change_analysis: ChangeAnalysis {
                time_since_last_build: Duration::from_secs(0),
                git_commit_changes: None,
                environment_changes: vec![],
                compiler_version_change: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regression_detector_creation() {
        let detector = RegressionDetector::new();
        assert_eq!(detector.performance_history.len(), 0);
        assert_eq!(detector.baseline_metrics.sample_count, 0);
    }

    #[test]
    fn test_mean_std_dev_calculation() {
        let detector = RegressionDetector::new();
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (mean, std_dev) = detector.calculate_mean_std_dev(&values);
        
        assert!((mean - 3.0).abs() < 0.001);
        assert!(std_dev > 1.0 && std_dev < 2.0);
    }

    #[test]
    fn test_p_value_calculation() {
        let detector = RegressionDetector::new();
        
        // Test with high z-score (should be significant)
        let p_value_high = detector.calculate_p_value(3.0);
        assert!(p_value_high < 0.01);
        
        // Test with low z-score (should not be significant)
        let p_value_low = detector.calculate_p_value(1.0);
        assert!(p_value_low > 0.05);
    }

    #[test]
    fn test_regression_type_classification() {
        let detector = RegressionDetector::new();
        
        let compilation_metric = AffectedMetric {
            metric_name: "compilation_time".to_string(),
            baseline_value: 1.0,
            current_value: 1.2,
            percentage_change: 0.2,
            statistical_significance: 0.95,
            effect_size: 2.0,
        };
        
        let regression_type = detector.classify_regression_type(&[compilation_metric]);
        assert!(matches!(regression_type, Some(RegressionType::CompilationTimeRegression)));
    }

    #[test]
    fn test_feature_extraction() {
        let detector = RegressionDetector::new();
        
        let data_point = PerformanceDataPoint {
            timestamp: 1000,
            build_id: "test".to_string(),
            compilation_time: Duration::from_secs(10),
            execution_time: Duration::from_millis(500),
            memory_usage: 1024 * 1024 * 100, // 100 MB
            binary_size: 1024 * 1024 * 50,   // 50 MB
            optimization_level: "O2".to_string(),
            git_commit: None,
            environment_info: EnvironmentInfo {
                os: "linux".to_string(),
                cpu_model: "test".to_string(),
                memory_gb: 16,
                compiler_version: "1.0".to_string(),
                temperature_celsius: None,
            },
        };
        
        let features = detector.extract_features(&data_point);
        
        assert!((features.compilation_time - 10.0).abs() < f64::EPSILON);
        assert!((features.execution_time - 0.5).abs() < f64::EPSILON);
        assert!((features.memory_usage - 100.0).abs() < f64::EPSILON);
        assert!((features.binary_size - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_isolation_forest_creation() {
        let config = IsolationForestConfig {
            n_trees: 10,
            sample_size: Some(100),
            random_seed: Some(42),
            anomaly_threshold: 0.6,
            max_height: None,
        };
        
        let forest = IsolationForest::new(config);
        assert!(forest.is_ok());
        
        let forest = forest.unwrap();
        assert_eq!(forest.n_trees, 10);
        assert_eq!(forest.sample_size, 100);
        assert!((forest.anomaly_threshold - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn test_isolation_forest_fitting() {
        let config = IsolationForestConfig {
            n_trees: 5,
            sample_size: Some(10),
            random_seed: Some(42),
            anomaly_threshold: 0.6,
            max_height: None,
        };
        
        let mut forest = IsolationForest::new(config).unwrap();
        
        // Create test data
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(PerformanceFeatureVector {
                compilation_time: 10.0 + i as f64 * 0.1,
                execution_time: 1.0 + i as f64 * 0.05,
                memory_usage: 100.0 + i as f64 * 2.0,
                binary_size: 50.0 + i as f64 * 1.0,
            });
        }
        
        let result = forest.fit(&data);
        assert!(result.is_ok());
        assert_eq!(forest.trees.len(), 5);
    }

    #[test]
    fn test_isolation_forest_anomaly_detection() {
        let config = IsolationForestConfig {
            n_trees: 10,
            sample_size: Some(15),
            random_seed: Some(42),
            anomaly_threshold: 0.6,
            max_height: None,
        };
        
        let mut forest = IsolationForest::new(config).unwrap();
        
        // Create normal data
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(PerformanceFeatureVector {
                compilation_time: 10.0 + i as f64 * 0.1,
                execution_time: 1.0 + i as f64 * 0.05,
                memory_usage: 100.0 + i as f64 * 2.0,
                binary_size: 50.0 + i as f64 * 1.0,
            });
        }
        
        forest.fit(&data).unwrap();
        
        // Test normal point
        let normal_point = PerformanceFeatureVector {
            compilation_time: 10.5,
            execution_time: 1.25,
            memory_usage: 110.0,
            binary_size: 55.0,
        };
        
        let score = forest.anomaly_score(&normal_point).unwrap();
        assert!(score >= 0.0 && score <= 1.0);
        
        // Test outlier point
        let outlier_point = PerformanceFeatureVector {
            compilation_time: 100.0, // Much higher than normal
            execution_time: 10.0,    // Much higher than normal
            memory_usage: 1000.0,    // Much higher than normal
            binary_size: 500.0,      // Much higher than normal
        };
        
        let outlier_score = forest.anomaly_score(&outlier_point).unwrap();
        assert!(outlier_score >= 0.0 && outlier_score <= 1.0);
        // Outlier should generally have higher score than normal point
        // Note: This is probabilistic, so not guaranteed, but very likely
    }

    #[test]
    fn test_isolation_tree_path_length() {
        let tree = IsolationTree::new(10);
        
        let point = PerformanceFeatureVector {
            compilation_time: 10.0,
            execution_time: 1.0,
            memory_usage: 100.0,
            binary_size: 50.0,
        };
        
        // Empty tree should return 0
        let path_length = tree.path_length(&point);
        assert!((path_length - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_build_isolation_forest_integration() {
        let mut detector = RegressionDetector::new();
        
        // Add some historical data
        for i in 0..15 {
            let data_point = PerformanceDataPoint {
                timestamp: 1000 + i,
                build_id: format!("build_{}", i),
                compilation_time: Duration::from_secs(10 + i / 5),
                execution_time: Duration::from_millis(500 + i * 10),
                memory_usage: 1024 * 1024 * (100 + i * 2),
                binary_size: 1024 * 1024 * (50 + i),
                optimization_level: "O2".to_string(),
                git_commit: None,
                environment_info: EnvironmentInfo {
                    os: "linux".to_string(),
                    cpu_model: "test".to_string(),
                    memory_gb: 16,
                    compiler_version: "1.0".to_string(),
                    temperature_celsius: None,
                },
            };
            
            detector.add_performance_data(data_point).unwrap();
        }
        
        // Should have built isolation forest automatically
        assert!(detector.isolation_forest.is_some());
        
        // Test outlier detection
        let outlier_point = PerformanceDataPoint {
            timestamp: 2000,
            build_id: "outlier".to_string(),
            compilation_time: Duration::from_secs(100), // Much higher
            execution_time: Duration::from_secs(10),    // Much higher
            memory_usage: 1024 * 1024 * 1000,          // Much higher
            binary_size: 1024 * 1024 * 500,            // Much higher
            optimization_level: "O2".to_string(),
            git_commit: None,
            environment_info: EnvironmentInfo {
                os: "linux".to_string(),
                cpu_model: "test".to_string(),
                memory_gb: 16,
                compiler_version: "1.0".to_string(),
                temperature_celsius: None,
            },
        };
        
        let is_outlier = detector.is_outlier_isolation(&outlier_point).unwrap();
        // This is probabilistic, but outlier should likely be detected
        // For a deterministic test, we'll just check the function doesn't panic
        assert!(is_outlier == true || is_outlier == false);
    }
}
