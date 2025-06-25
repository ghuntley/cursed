/// Real Benchmark Regression Detection
/// 
/// Provides sophisticated statistical analysis to detect performance regressions
/// with high confidence and minimal false positives.

use crate::error::{CursedError, Result};

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use tracing::{debug, info, warn, instrument, trace};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Regression detector with statistical analysis
pub struct RegressionDetector {
    /// Historical performance data
    /// Baseline performance metrics
    /// Statistical analysis configuration
    /// Regression detection thresholds
    /// Trend analysis
    /// Isolation forest for outlier detection
/// Performance data point for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
/// Environment information for context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
/// Baseline performance metrics for comparison
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
/// Statistical analysis configuration
#[derive(Debug, Clone)]
pub struct StatisticalAnalysisConfig {
    /// Minimum samples needed for reliable analysis
    /// Window size for trend analysis
    /// Confidence level for statistical tests (e.g., 0.95 for 95%)
    /// Significance level for hypothesis testing
    /// Outlier detection method
/// Regression detection thresholds
#[derive(Debug, Clone)]
pub struct RegressionThresholds {
    /// Compilation time regression threshold (e.g., 1.15 for 15% increase)
    /// Execution time regression threshold
    /// Memory usage regression threshold
    /// Binary size regression threshold
    /// Standard deviations for outlier detection
/// Outlier detection methods
#[derive(Debug, Clone)]
pub enum OutlierDetectionMethod {
/// Trend analyzer for long-term performance trends
#[derive(Debug)]
pub struct TrendAnalyzer {
    /// Historical trends
    /// Seasonal patterns
/// Performance trend information
#[derive(Debug, Clone)]
pub struct PerformanceTrend {
/// Trend direction
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
/// Seasonal pattern in performance
#[derive(Debug, Clone)]
pub struct SeasonalPattern {
/// Types of seasonal patterns
#[derive(Debug, Clone)]
pub enum SeasonalPatternType {
/// Regression detection result
#[derive(Debug, Clone)]
pub struct RegressionDetectionResult {
/// Type of regression detected
#[derive(Debug, Clone)]
pub enum RegressionType {
/// Affected metric details
#[derive(Debug, Clone)]
pub struct AffectedMetric {
/// Root cause analysis results
#[derive(Debug, Clone)]
pub struct RootCauseAnalysis {
/// Potential cause of regression
#[derive(Debug, Clone)]
pub struct PotentialCause {
/// Types of regression causes
#[derive(Debug, Clone)]
pub enum CauseType {
/// Correlation analysis result
#[derive(Debug, Clone)]
pub struct CorrelationResult {
/// Type of relationship between metrics
#[derive(Debug, Clone)]
pub enum RelationshipType {
/// Environmental factor affecting performance
#[derive(Debug, Clone)]
pub struct EnvironmentalFactor {
/// Change analysis between builds
#[derive(Debug, Clone)]
pub struct ChangeAnalysis {
/// Regression recommendation
#[derive(Debug, Clone)]
pub struct RegressionRecommendation {
/// Type of recommendation
#[derive(Debug, Clone)]
pub enum RecommendationType {
/// Priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
/// Isolation forest for anomaly detection
#[derive(Debug)]
pub struct IsolationForest {
    /// Collection of isolation trees
    /// Number of trees in the forest
    /// Sample size for each tree
    /// Random number generator for reproducibility
    /// Anomaly score threshold
/// Individual isolation tree
#[derive(Debug)]
pub struct IsolationTree {
    /// Root node of the tree
    /// Maximum tree height
/// Node in an isolation tree
#[derive(Debug)]
pub struct IsolationNode {
    /// Feature index to split on
    /// Split value
    /// Left child (values < split_value)
    /// Right child (values >= split_value)
    /// Node size (number of data points)
    /// Node depth
/// Feature vector extracted from performance data
#[derive(Debug, Clone)]
pub struct PerformanceFeatureVector {
    /// Compilation time in seconds
    /// Execution time in seconds
    /// Memory usage in MB
    /// Binary size in MB
/// Isolation forest configuration
#[derive(Debug, Clone)]
pub struct IsolationForestConfig {
    /// Number of trees in the forest
    /// Sample size for each tree (if None, uses sqrt(dataset_size))
    /// Random seed for reproducibility
    /// Anomaly score threshold (0.5 is neutral, >0.5 indicates anomaly)
    /// Maximum tree height
impl Default for StatisticalAnalysisConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for RegressionThresholds {
    fn default() -> Self {
        Self {
            compilation_time_threshold: 1.10, // 10% increase
            execution_time_threshold: 1.05,   // 5% increase
            memory_usage_threshold: 1.15,     // 15% increase
            binary_size_threshold: 1.20,      // 20% increase
        }
    }
impl RegressionDetector {
    /// Create new regression detector
    pub fn new() -> Self {
        Self {
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
            // Update baseline if we have enough samples
            if self.performance_history.len() >= self.analysis_config.min_samples {
                self.update_baseline_metrics()?;
            }
        } else {
            warn!("Detected outlier data point, excluding from analysis");
        Ok(())
    /// Detect performance regression
    #[instrument(skip(self))]
    pub fn detect_regression(&mut self, current_data: &PerformanceDataPoint) -> Result<RegressionDetectionResult> {
        info!("Starting regression detection for build {}", current_data.build_id);
        
        if self.performance_history.len() < self.analysis_config.min_samples {
            return Ok(RegressionDetectionResult {
                recommendations: vec![RegressionRecommendation {
                    implementation_steps: vec![
            });
        // Perform statistical tests for each metric
        let mut affected_metrics = Vec::new();
        let mut has_regression = false;

        // Test compilation time
        if let Some(metric) = self.test_metric_regression(
        )? {
            affected_metrics.push(metric);
            has_regression = true;
        // Test execution time
        if let Some(metric) = self.test_metric_regression(
        )? {
            affected_metrics.push(metric);
            has_regression = true;
        // Test memory usage
        if let Some(metric) = self.test_metric_regression(
        )? {
            affected_metrics.push(metric);
            has_regression = true;
        // Test binary size
        if let Some(metric) = self.test_metric_regression(
        )? {
            affected_metrics.push(metric);
            has_regression = true;
        // Determine regression type
        let regression_type = if has_regression {
            self.classify_regression_type(&affected_metrics)
        } else {
            None

        // Calculate overall confidence
        let confidence_score = if has_regression {
            self.calculate_overall_confidence(&affected_metrics)
        } else {
            0.0

        // Perform root cause analysis
        let root_cause_analysis = if has_regression {
            self.perform_root_cause_analysis(current_data, &affected_metrics)?
        } else {
            RootCauseAnalysis::default()

        // Generate recommendations
        let recommendations = self.generate_recommendations(
        )?;

        // Calculate statistical significance
        let statistical_significance = if !affected_metrics.is_empty() {
            affected_metrics.iter()
                .map(|m| m.statistical_significance)
                .fold(0.0, f64::max)
        } else {
            0.0

        Ok(RegressionDetectionResult {
        })
    /// Test for regression in a specific metric
    fn test_metric_regression(
    ) -> Result<Option<AffectedMetric>> {
        // Calculate percentage change
        let percentage_change = if baseline_mean > 0.0 {
            (current_value - baseline_mean) / baseline_mean
        } else {
            0.0

        // Check if change exceeds threshold
        let exceeds_threshold = current_value > baseline_mean * threshold;

        if !exceeds_threshold {
            return Ok(None);
        // Perform statistical significance test (one-sample t-test)
        let z_score = if baseline_std_dev > 0.0 {
            (current_value - baseline_mean) / baseline_std_dev
        } else {
            0.0

        // Calculate p-value (simplified normal distribution approximation)
        let p_value = self.calculate_p_value(z_score);
        let is_significant = p_value < self.analysis_config.significance_level;

        if !is_significant {
            return Ok(None);
        // Calculate effect size (Cohen's d)
        let effect_size = if baseline_std_dev > 0.0 {
            (current_value - baseline_mean) / baseline_std_dev
        } else {
            0.0

        Ok(Some(AffectedMetric {
        }))
    /// Calculate p-value from z-score (simplified)
    fn calculate_p_value(&self, z_score: f64) -> f64 {
        // Simplified normal distribution CDF approximation
        // For more accuracy, would use a proper statistical library
        if z_score < 0.0 {
            return 0.5;
        // Approximate p-value for one-tailed test
        let p_value = match z_score {

        p_value
    /// Update baseline metrics from historical data
    fn update_baseline_metrics(&mut self) -> Result<()> {
        let data_points: Vec<_> = self.performance_history.iter().collect();
        
        if data_points.is_empty() {
            return Ok(());
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

        debug!("Updated baseline metrics with {} samples", data_points.len());
        
        // Rebuild isolation forest with updated data
        if let Err(e) = self.build_isolation_forest() {
            warn!("Failed to rebuild isolation forest: {}", e);
        Ok(())
    /// Calculate mean and standard deviation
    fn calculate_mean_std_dev(&self, values: &[f64]) -> (f64, f64) {
        if values.is_empty() {
            return (0.0, 0.0);
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        
        let variance = values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        
        let std_dev = variance.sqrt();
        
        (mean, std_dev)
    /// Check if data point is an outlier
    fn is_outlier(&self, data_point: &PerformanceDataPoint) -> Result<bool> {
        if self.performance_history.len() < 10 {
            return Ok(false); // Not enough data to detect outliers
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
    /// Check outlier using interquartile range method
    fn is_outlier_iqr(&self, data_point: &PerformanceDataPoint, multiplier: f64) -> Result<bool> {
        // Simplified IQR implementation
        // In a full implementation, would calculate actual quartiles
        self.is_outlier_std_dev(data_point, multiplier)
    /// Check outlier using modified Z-score method
    fn is_outlier_modified_z_score(&self, data_point: &PerformanceDataPoint, threshold: f64) -> Result<bool> {
        // Simplified implementation using median absolute deviation
        self.is_outlier_std_dev(data_point, threshold)
    /// Check outlier using isolation forest method
    fn is_outlier_isolation(&self, data_point: &PerformanceDataPoint) -> Result<bool> {
        trace!("Checking outlier using isolation forest method");
        
        // If isolation forest is not initialized, build it first
        if self.isolation_forest.is_none() {
            debug!("Isolation forest not initialized, building from historical data");
            return Ok(false); // Cannot detect outliers without trained forest
        let forest = self.isolation_forest.as_ref().unwrap();
        
        // Convert data point to feature vector
        let feature_vector = self.extract_features(data_point);
        
        // Calculate anomaly score
        let anomaly_score = forest.anomaly_score(&feature_vector)?;
        
        trace!("Anomaly score: {:.4}, threshold: {:.4}", anomaly_score, forest.anomaly_threshold);
        
        // Return true if anomaly score exceeds threshold
        Ok(anomaly_score > forest.anomaly_threshold)
    /// Extract feature vector from performance data point
    fn extract_features(&self, data_point: &PerformanceDataPoint) -> PerformanceFeatureVector {
        PerformanceFeatureVector {
            memory_usage: data_point.memory_usage as f64 / 1024.0 / 1024.0, // Convert to MB
            binary_size: data_point.binary_size as f64 / 1024.0 / 1024.0,   // Convert to MB
        }
    }
    
    /// Build isolation forest from historical data
    pub fn build_isolation_forest(&mut self) -> Result<()> {
        if self.performance_history.len() < 10 {
            debug!("Insufficient data for isolation forest, need at least 10 samples");
            return Ok(());
        debug!("Building isolation forest with {} samples", self.performance_history.len());
        
        // Extract feature vectors from historical data
        let feature_vectors: Vec<PerformanceFeatureVector> = self.performance_history
            .iter()
            .map(|dp| self.extract_features(dp))
            .collect();
        
        // Configure isolation forest
        let config = IsolationForestConfig {
            random_seed: Some(42), // For reproducibility
            anomaly_threshold: 0.6, // Slightly above neutral
        
        // Build the forest
        let mut forest = IsolationForest::new(config)?;
        forest.fit(&feature_vectors)?;
        
        self.isolation_forest = Some(forest);
        
        info!("Isolation forest built successfully");
        Ok(())
    /// Classify the type of regression
    fn classify_regression_type(&self, affected_metrics: &[AffectedMetric]) -> Option<RegressionType> {
        if affected_metrics.is_empty() {
            return None;
        if affected_metrics.len() > 1 {
            return Some(RegressionType::MultipleMetricRegression);
        match affected_metrics[0].metric_name.as_str() {
        }
    }

    /// Calculate overall confidence score
    fn calculate_overall_confidence(&self, affected_metrics: &[AffectedMetric]) -> f64 {
        if affected_metrics.is_empty() {
            return 0.0;
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
    /// Perform root cause analysis
    fn perform_root_cause_analysis(
    ) -> Result<RootCauseAnalysis> {
        let mut potential_causes = Vec::new();
        let mut environmental_factors = Vec::new();

        // Analyze potential causes based on affected metrics
        for metric in affected_metrics {
            match metric.metric_name.as_str() {
                "compilation_time" => {
                    potential_causes.push(PotentialCause {
                    });
                }
                "execution_time" => {
                    potential_causes.push(PotentialCause {
                    });
                }
                "memory_usage" => {
                    potential_causes.push(PotentialCause {
                    });
                }
                _ => {}
            }
        }

        // Analyze environmental factors
        if let Some(previous_data) = self.performance_history.back() {
            if current_data.environment_info.compiler_version != previous_data.environment_info.compiler_version {
                environmental_factors.push(EnvironmentalFactor {
                });
            if current_data.environment_info.os != previous_data.environment_info.os {
                environmental_factors.push(EnvironmentalFactor {
                });
            }
        }

        Ok(RootCauseAnalysis {
            correlation_analysis: vec![], // Simplified for now
            change_analysis: ChangeAnalysis {
                time_since_last_build: Duration::from_secs(0), // Would calculate from data
        })
    /// Generate recommendations based on analysis
    fn generate_recommendations(
    ) -> Result<Vec<RegressionRecommendation>> {
        let mut recommendations = Vec::new();

        if !has_regression {
            recommendations.push(RegressionRecommendation {
            });
            return Ok(recommendations);
        // Generate recommendations based on regression type
        match regression_type {
            Some(RegressionType::PerformanceRegression) => {
                recommendations.push(RegressionRecommendation {
                    implementation_steps: vec![
                });
            }
            Some(RegressionType::CompilationTimeRegression) => {
                recommendations.push(RegressionRecommendation {
                    implementation_steps: vec![
                });
            }
            Some(RegressionType::MultipleMetricRegression) => {
                recommendations.push(RegressionRecommendation {
                    implementation_steps: vec![
                });
            }
            _ => {}
        // Add recommendations based on root cause analysis
        if !root_cause_analysis.environmental_factors.is_empty() {
            recommendations.push(RegressionRecommendation {
                implementation_steps: vec![
            });
        Ok(recommendations)
    }
}

impl TrendAnalyzer {
    pub fn new() -> Self {
        Self {
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
        
        Ok(Self {
        })
    /// Fit the isolation forest to training data
    #[instrument(skip(self, data))]
    pub fn fit(&mut self, data: &[PerformanceFeatureVector]) -> Result<()> {
        debug!("Fitting isolation forest with {} samples", data.len());
        
        if data.is_empty() {
            return Err(CursedError::InvalidInput("Cannot fit isolation forest on empty data".to_string()));
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
        info!("Isolation forest fitting completed with {} trees", self.n_trees);
        Ok(())
    /// Calculate anomaly score for a single data point
    pub fn anomaly_score(&self, point: &PerformanceFeatureVector) -> Result<f64> {
        if self.trees.is_empty() {
            return Err(CursedError::InvalidState("Isolation forest not fitted".to_string()));
        // Calculate average path length across all trees
        let mut total_path_length = 0.0;
        
        for tree in &self.trees {
            let path_length = tree.path_length(point);
            total_path_length += path_length;
        let avg_path_length = total_path_length / self.trees.len() as f64;
        
        // Convert path length to anomaly score
        // Score ranges from 0 to 1, where values > 0.5 indicate potential anomalies
        let c = self.average_path_length(self.sample_size);
        let anomaly_score = 2.0_f64.powf(-avg_path_length / c);
        
        trace!("Average path length: {:.4}, anomaly score: {:.4}", avg_path_length, anomaly_score);
        
        Ok(anomaly_score)
    /// Sample data randomly for tree building
    fn sample_data(&mut self, data: &[PerformanceFeatureVector], sample_size: usize) -> Vec<PerformanceFeatureVector> {
        let mut sample = Vec::with_capacity(sample_size);
        
        for _ in 0..sample_size {
            let idx = self.rng.gen_range(0..data.len());
            sample.push(data[idx].clone());
        sample
    /// Calculate average path length for a given sample size (theoretical)
    fn average_path_length(&self, n: usize) -> f64 {
        if n <= 1 {
            return 0.0;
        2.0 * (((n - 1) as f64).ln() + 0.5772156649) - (2.0 * (n - 1) as f64 / n as f64)
    }
}

impl IsolationTree {
    /// Create new isolation tree
    pub fn new(max_height: usize) -> Self {
        Self {
        }
    }
    
    /// Build the isolation tree from data
    pub fn build(&mut self, data: &[PerformanceFeatureVector], rng: &mut StdRng) -> Result<()> {
        if data.is_empty() {
            return Ok(());
        self.root = Some(self.build_node(data, 0, rng)?);
        Ok(())
    /// Recursively build tree nodes
    fn build_node(&self, data: &[PerformanceFeatureVector], depth: usize, rng: &mut StdRng) -> Result<IsolationNode> {
        let size = data.len();
        
        // Stop conditions: max depth reached, or only one sample, or all samples are identical
        if depth >= self.max_height || size <= 1 || self.all_identical(data) {
            return Ok(IsolationNode {
            });
        // Randomly select feature and split value
        let feature_index = rng.gen_range(0..4); // 4 features
        let (min_val, max_val) = self.get_feature_range(data, feature_index);
        
        if (max_val - min_val).abs() < f64::EPSILON {
            // All values are the same for this feature
            return Ok(IsolationNode {
            });
        let split_value = rng.gen_range(min_val..max_val);
        
        // Split data
        let (left_data, right_data) = self.split_data(data, feature_index, split_value);
        
        // Build child nodes
        let left = if !left_data.is_empty() {
            Some(Box::new(self.build_node(&left_data, depth + 1, rng)?))
        } else {
            None
        
        let right = if !right_data.is_empty() {
            Some(Box::new(self.build_node(&right_data, depth + 1, rng)?))
        } else {
            None
        
        Ok(IsolationNode {
        })
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
    /// Get feature value by index
    fn get_feature_value(&self, point: &PerformanceFeatureVector, feature_index: usize) -> f64 {
        match feature_index {
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
    /// Check if all data points are identical
    fn all_identical(&self, data: &[PerformanceFeatureVector]) -> bool {
        if data.len() <= 1 {
            return true;
        let first = &data[0];
        data.iter().all(|point| {
            (point.compilation_time - first.compilation_time).abs() < f64::EPSILON &&
            (point.execution_time - first.execution_time).abs() < f64::EPSILON &&
            (point.memory_usage - first.memory_usage).abs() < f64::EPSILON &&
            (point.binary_size - first.binary_size).abs() < f64::EPSILON
        })
    /// Estimate remaining path length for leaf nodes
    fn estimate_remaining_path_length(&self, size: usize) -> f64 {
        if size <= 1 {
            0.0
        } else {
            2.0 * (((size - 1) as f64).ln() + 0.5772156649) - (2.0 * (size - 1) as f64 / size as f64)
        }
    }
impl Default for BaselineMetrics {
    fn default() -> Self {
        Self {
        }
    }
impl Default for RootCauseAnalysis {
    fn default() -> Self {
        Self {
            change_analysis: ChangeAnalysis {
        }
    }
