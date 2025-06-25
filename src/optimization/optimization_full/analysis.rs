// Performance analysis and bottleneck detection

use crate::error::{CursedError, Result};
use crate::optimization::benchmarking::{BenchmarkResults, BenchmarkStatistics};
use crate::optimization::metrics::{SystemStatistics, ResourceStatistics};

use std::time::Duration;
use std::collections::HashMap;
use tracing::{info, debug, warn, instrument};
use serde::{Deserialize, Serialize};

/// Performance analysis result
#[derive(Debug, Clone)]
pub struct PerformanceAnalysis {
/// Performance trend analysis
#[derive(Debug, Clone)]
pub struct PerformanceTrend {
/// Direction of performance trends
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrendDirection {
    Insufficient, // Not enough data
/// Performance bottleneck identification
#[derive(Debug, Clone)]
pub struct PerformanceBottleneck {
/// Severity levels for performance bottlenecks
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BottleneckSeverity {
/// Regression detection result
#[derive(Debug, Clone)]
pub struct RegressionAnalysis {
/// Severity of performance regressions
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegressionSeverity {
    Minor,      // < 5% degradation
    Moderate,   // 5-15% degradation  
    Significant, // 15-30% degradation
    Severe,     // > 30% degradation
/// Performance prediction result
#[derive(Debug, Clone)]
pub struct PerformancePrediction {
/// Configuration for performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Performance analysis engine
#[derive(Debug)]
pub struct PerformanceAnalyzer {
impl PerformanceAnalyzer {
    /// Create a new performance analyzer
    #[instrument]
    pub fn new(config: AnalysisConfig) -> Result<Self> {
        info!("Creating performance analyzer");
        
        Ok(Self {
        })
    /// Analyze performance trends over time
    #[instrument(skip(self))]
    pub fn analyze_trends(&self) -> Result<Vec<PerformanceTrend>> {
        if !self.config.enable_trend_analysis {
            return Ok(Vec::new());
        debug!("Analyzing performance trends");
        let mut trends = Vec::new();

        // Analyze compilation time trends
        if let Some(compilation_trend) = self.analyze_compilation_time_trend()? {
            trends.push(compilation_trend);
        // Analyze memory usage trends
        if let Some(memory_trend) = self.analyze_memory_usage_trend()? {
            trends.push(memory_trend);
        // Analyze throughput trends
        if let Some(throughput_trend) = self.analyze_throughput_trend()? {
            trends.push(throughput_trend);
        // Analyze cache performance trends
        if let Some(cache_trend) = self.analyze_cache_performance_trend()? {
            trends.push(cache_trend);
        info!("Analyzed {} performance trends", trends.len());
        Ok(trends)
    /// Detect performance bottlenecks
    #[instrument(skip(self))]
    pub fn detect_bottlenecks(&self) -> Result<Vec<PerformanceBottleneck>> {
        if !self.config.enable_bottleneck_detection {
            return Ok(Vec::new());
        debug!("Detecting performance bottlenecks");
        let mut bottlenecks = Vec::new();

        // Check for memory bottlenecks
        if let Some(memory_bottleneck) = self.detect_memory_bottleneck()? {
            bottlenecks.push(memory_bottleneck);
        // Check for CPU bottlenecks
        if let Some(cpu_bottleneck) = self.detect_cpu_bottleneck()? {
            bottlenecks.push(cpu_bottleneck);
        // Check for I/O bottlenecks
        if let Some(io_bottleneck) = self.detect_io_bottleneck()? {
            bottlenecks.push(io_bottleneck);
        // Check for compilation speed bottlenecks
        if let Some(compilation_bottleneck) = self.detect_compilation_bottleneck()? {
            bottlenecks.push(compilation_bottleneck);
        // Check for cache bottlenecks
        if let Some(cache_bottleneck) = self.detect_cache_bottleneck()? {
            bottlenecks.push(cache_bottleneck);
        // Sort by severity
        bottlenecks.sort_by(|a, b| b.severity.cmp(&a.severity));

        info!("Detected {} performance bottlenecks", bottlenecks.len());
        Ok(bottlenecks)
    /// Detect performance regressions
    #[instrument(skip(self))]
    pub fn detect_regressions(&self) -> Result<RegressionAnalysis> {
        if !self.config.enable_regression_detection {
            return Ok(RegressionAnalysis {
            });
        debug!("Detecting performance regressions");

        let mut affected_metrics = Vec::new();
        let mut max_regression_percentage = 0.0;

        // Check compilation time regressions
        if let Some(regression_pct) = self.check_compilation_time_regression()? {
            if regression_pct > self.config.regression_threshold_percentage {
                affected_metrics.push("compilation_time".to_string());
                max_regression_percentage = max_regression_percentage.max(regression_pct);
            }
        }

        // Check memory usage regressions
        if let Some(regression_pct) = self.check_memory_usage_regression()? {
            if regression_pct > self.config.regression_threshold_percentage {
                affected_metrics.push("memory_usage".to_string());
                max_regression_percentage = max_regression_percentage.max(regression_pct);
            }
        }

        // Check throughput regressions
        if let Some(regression_pct) = self.check_throughput_regression()? {
            if regression_pct > self.config.regression_threshold_percentage {
                affected_metrics.push("throughput".to_string());
                max_regression_percentage = max_regression_percentage.max(regression_pct);
            }
        }

        let regression_detected = !affected_metrics.is_empty();
        let severity = self.classify_regression_severity(max_regression_percentage);

        let probable_causes = if regression_detected {
            self.identify_probable_regression_causes(&affected_metrics)
        } else {
            Vec::new()

        let impact_description = if regression_detected {
            format!(
                max_regression_percentage
            )
        } else {
            "No significant performance regressions detected".to_string()

        Ok(RegressionAnalysis {
        })
    /// Generate performance predictions
    #[instrument(skip(self))]
    pub fn generate_predictions(&self) -> Result<Vec<PerformancePrediction>> {
        if !self.config.enable_prediction {
            return Ok(Vec::new());
        debug!("Generating performance predictions");
        let mut predictions = Vec::new();

        // Predict compilation time
        if let Some(compilation_prediction) = self.predict_compilation_time()? {
            predictions.push(compilation_prediction);
        // Predict memory usage
        if let Some(memory_prediction) = self.predict_memory_usage()? {
            predictions.push(memory_prediction);
        // Predict throughput
        if let Some(throughput_prediction) = self.predict_throughput()? {
            predictions.push(throughput_prediction);
        info!("Generated {} performance predictions", predictions.len());
        Ok(predictions)
    /// Add benchmark results to history for analysis
    pub fn add_benchmark_results(&mut self, benchmark_name: String, results: &BenchmarkResults) {
        self.benchmark_history
            .entry(benchmark_name)
            .or_insert_with(Vec::new)
            .push(results.statistics.clone());

        // Keep only recent results
        if let Some(history) = self.benchmark_history.get_mut(&benchmark_name) {
            if history.len() > 100 {
                history.drain(0..50);
            }
        }
    /// Add system metrics to history
    pub fn add_system_metrics(&mut self, metrics: SystemStatistics) {
        self.system_metrics_history.push(metrics);
        
        // Keep only recent metrics
        if self.system_metrics_history.len() > 1000 {
            self.system_metrics_history.drain(0..500);
        }
    }

    /// Add resource metrics to history
    pub fn add_resource_metrics(&mut self, metrics: ResourceStatistics) {
        self.resource_metrics_history.push(metrics);
        
        // Keep only recent metrics
        if self.resource_metrics_history.len() > 1000 {
            self.resource_metrics_history.drain(0..500);
        }
    }

    /// Set baseline metrics for comparison
    pub fn set_baseline_metrics(&mut self, metrics: HashMap<String, f64>) {
        info!("Setting baseline metrics for performance analysis");
        self.baseline_metrics = metrics;
    /// Generate comprehensive performance report
    #[instrument(skip(self))]
    pub fn generate_comprehensive_report(&self) -> Result<PerformanceReport> {
        info!("Generating comprehensive performance report");

        let trends = self.analyze_trends()?;
        let bottlenecks = self.detect_bottlenecks()?;
        let regression_analysis = self.detect_regressions()?;
        let predictions = self.generate_predictions()?;

        // Generate summary and recommendations
        let summary = self.generate_performance_summary(&trends, &bottlenecks, &regression_analysis);
        let recommendations = self.generate_recommendations(&trends, &bottlenecks, &regression_analysis);

        Ok(PerformanceReport {
        })
    // Helper methods for trend analysis
    fn analyze_compilation_time_trend(&self) -> Result<Option<PerformanceTrend>> {
        // Get recent compilation time data from system metrics
        if self.system_metrics_history.len() < self.config.minimum_samples_for_analysis {
            return Ok(None);
        let window_size = self.config.trend_window_size.min(self.system_metrics_history.len());
        let recent_metrics = &self.system_metrics_history[self.system_metrics_history.len() - window_size..];
        
        // Extract compilation times
        let times: Vec<f64> = recent_metrics.iter()
            .map(|m| m.average_compilation_time.as_millis() as f64)
            .filter(|&t| t > 0.0)
            .collect();
        
        if times.len() < 3 {
            return Ok(None);
        // Calculate trend using linear regression
        let (slope, confidence) = self.calculate_linear_trend(&times);
        
        let change_percentage = if times[0] > 0.0 {
            (slope / times[0]) * 100.0
        } else {
            0.0
        
        let trend_direction = if change_percentage.abs() < 1.0 {
            TrendDirection::Stable
        } else if change_percentage > 0.0 {
            TrendDirection::Declining // Higher compilation times are worse
        } else {
            TrendDirection::Improving
        
               slope, change_percentage, confidence);
        
        Ok(Some(PerformanceTrend {
        }))
    fn analyze_memory_usage_trend(&self) -> Result<Option<PerformanceTrend>> {
        if self.resource_metrics_history.len() < self.config.minimum_samples_for_analysis {
            return Ok(None);
        let window_size = self.config.trend_window_size.min(self.resource_metrics_history.len());
        let recent_metrics = &self.resource_metrics_history[self.resource_metrics_history.len() - window_size..];
        
        // Extract peak memory usage values
        let memory_values: Vec<f64> = recent_metrics.iter()
            .map(|m| m.peak_memory_mb)
            .filter(|&m| m > 0.0)
            .collect();
        
        if memory_values.len() < 3 {
            return Ok(None);
        let (slope, confidence) = self.calculate_linear_trend(&memory_values);
        
        let change_percentage = if memory_values[0] > 0.0 {
            (slope / memory_values[0]) * 100.0
        } else {
            0.0
        
        let trend_direction = if change_percentage.abs() < 2.0 {
            TrendDirection::Stable
        } else if change_percentage > 0.0 {
            TrendDirection::Declining // Higher memory usage is worse
        } else {
            TrendDirection::Improving
        
        Ok(Some(PerformanceTrend {
        }))
    fn analyze_throughput_trend(&self) -> Result<Option<PerformanceTrend>> {
        Ok(Some(PerformanceTrend {
        }))
    fn analyze_cache_performance_trend(&self) -> Result<Option<PerformanceTrend>> {
        Ok(Some(PerformanceTrend {
        }))
    // Helper methods for bottleneck detection
    fn detect_memory_bottleneck(&self) -> Result<Option<PerformanceBottleneck>> {
        // Simulated memory bottleneck detection
        if self.resource_metrics_history.len() > 5 {
            let avg_memory = self.resource_metrics_history.iter()
                .map(|m| m.peak_memory_mb)
                .sum::<f64>() / self.resource_metrics_history.len() as f64;

            if avg_memory > 400.0 {
                return Ok(Some(PerformanceBottleneck {
                    recommendations: vec![
                }));
            }
        }
        Ok(None)
    fn detect_cpu_bottleneck(&self) -> Result<Option<PerformanceBottleneck>> {
        Ok(None)
    fn detect_io_bottleneck(&self) -> Result<Option<PerformanceBottleneck>> {
        Ok(None)
    fn detect_compilation_bottleneck(&self) -> Result<Option<PerformanceBottleneck>> {
        Ok(None)
    fn detect_cache_bottleneck(&self) -> Result<Option<PerformanceBottleneck>> {
        Ok(None)
    // Helper methods for regression detection
    fn check_compilation_time_regression(&self) -> Result<Option<f64>> {
        if self.system_metrics_history.len() < 5 {
            return Ok(None);
        // Compare recent performance with baseline
        let recent_window = 3; // Last 3 builds
        let baseline_window = 5; // Previous 5 builds before recent
        
        if self.system_metrics_history.len() < recent_window + baseline_window {
            return Ok(None);
        let total_len = self.system_metrics_history.len();
        let recent_metrics = &self.system_metrics_history[total_len - recent_window..];
        let baseline_metrics = &self.system_metrics_history[total_len - recent_window - baseline_window..total_len - recent_window];
        
        // Calculate average compilation times
        let recent_avg = recent_metrics.iter()
            .map(|m| m.average_compilation_time.as_millis() as f64)
            .filter(|&t| t > 0.0)
            .sum::<f64>() / recent_window as f64;
        
        let baseline_avg = baseline_metrics.iter()
            .map(|m| m.average_compilation_time.as_millis() as f64)
            .filter(|&t| t > 0.0)
            .sum::<f64>() / baseline_window as f64;
        
        if baseline_avg <= 0.0 {
            return Ok(None);
        let regression_percentage = ((recent_avg - baseline_avg) / baseline_avg) * 100.0;
        
               recent_avg, baseline_avg, regression_percentage);
        
        // Only report positive regressions (performance getting worse)
        if regression_percentage > 0.0 {
            Ok(Some(regression_percentage))
        } else {
            Ok(None)
        }
    }

    fn check_memory_usage_regression(&self) -> Result<Option<f64>> {
        Ok(None)
    fn check_throughput_regression(&self) -> Result<Option<f64>> {
        Ok(None)
    fn classify_regression_severity(&self, percentage: f64) -> RegressionSeverity {
        if percentage < 5.0 {
            RegressionSeverity::Minor
        } else if percentage < 15.0 {
            RegressionSeverity::Moderate
        } else if percentage < 30.0 {
            RegressionSeverity::Significant
        } else {
            RegressionSeverity::Severe
        }
    }

    fn identify_probable_regression_causes(&self, _affected_metrics: &[String]) -> Vec<String> {
        vec![
        ]
    // Helper methods for predictions
    fn predict_compilation_time(&self) -> Result<Option<PerformancePrediction>> {
        Ok(Some(PerformancePrediction {
        }))
    fn predict_memory_usage(&self) -> Result<Option<PerformancePrediction>> {
        Ok(None)
    fn predict_throughput(&self) -> Result<Option<PerformancePrediction>> {
        Ok(None)
    fn generate_performance_summary(
    ) -> String {
        let improving_trends = trends.iter().filter(|t| t.trend_direction == TrendDirection::Improving).count();
        let declining_trends = trends.iter().filter(|t| t.trend_direction == TrendDirection::Declining).count();
        let critical_bottlenecks = bottlenecks.iter().filter(|b| b.severity == BottleneckSeverity::Critical).count();

        format!(
            if regression_analysis.regression_detected {
                format!("Regression detected with {} severity", format!("{:?}", regression_analysis.severity).to_lowercase())
            } else {
                "No regressions detected".to_string()
            }
        )
    fn generate_recommendations(
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Add bottleneck recommendations
        for bottleneck in bottlenecks.iter().take(3) { // Top 3 bottlenecks
            recommendations.extend(bottleneck.recommendations.clone());
        // Add regression recommendations
        if regression_analysis.regression_detected {
            recommendations.push("Investigate recent changes that may have caused performance regression".to_string());
            recommendations.push("Consider reverting recent optimization changes if regression is severe".to_string());
        // Add trend-based recommendations
        let declining_trends: Vec<_> = trends.iter()
            .filter(|t| t.trend_direction == TrendDirection::Declining)
            .collect();
        
        if !declining_trends.is_empty() {
            recommendations.push("Monitor declining performance trends and investigate root causes".to_string());
        recommendations
    /// Calculate linear trend using simple linear regression
    fn calculate_linear_trend(&self, values: &[f64]) -> (f64, f64) {
        if values.len() < 2 {
            return (0.0, 0.0);
        let n = values.len() as f64;
        let x_values: Vec<f64> = (0..values.len()).map(|i| i as f64).collect();
        
        // Calculate means
        let x_mean = x_values.iter().sum::<f64>() / n;
        let y_mean = values.iter().sum::<f64>() / n;
        
        // Calculate slope using least squares
        let numerator: f64 = x_values.iter().zip(values.iter())
            .map(|(x, y)| (x - x_mean) * (y - y_mean))
            .sum();
        
        let denominator: f64 = x_values.iter()
            .map(|x| (x - x_mean).powi(2))
            .sum();
        
        let slope = if denominator != 0.0 {
            numerator / denominator
        } else {
            0.0
        
        // Calculate R-squared for confidence
        let y_predicted: Vec<f64> = x_values.iter()
            .map(|x| (y_mean + slope * (x - x_mean)))
            .collect();
        
        let ss_tot: f64 = values.iter()
            .map(|y| (y - y_mean).powi(2))
            .sum();
        
        let ss_res: f64 = values.iter().zip(y_predicted.iter())
            .map(|(y, y_pred)| (y - y_pred).powi(2))
            .sum();
        
        let r_squared = if ss_tot != 0.0 {
            (1.0 - (ss_res / ss_tot)).max(0.0)
        } else {
            0.0
        
        (slope, r_squared)
    }
}

/// Comprehensive performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
