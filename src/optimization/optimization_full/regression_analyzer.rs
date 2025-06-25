
// Comprehensive Regression Analysis System
//
// Provides advanced regression detection and analysis capabilities for
// performance monitoring and quality assurance.

use std::collections::HashMap;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, instrument};

use crate::error::{CursedError, Result};
use crate::optimization::benchmarks::{
// };

use crate::optimization::baseline_storage::{PerformanceBaseline, BaselineBenchmark};
use std::path::PathBuf;

/// Configuration for regression analysis
#[derive(Debug, Clone)]
pub struct RegressionAnalysisConfig {
    /// Performance thresholds for different metrics
    /// Statistical confidence level (0.0 to 1.0)
    /// Minimum sample size for statistical analysis
    /// Enable trend analysis
    /// Severity calculation mode
impl Default for RegressionAnalysisConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Mode for calculating regression severity
#[derive(Debug, Clone)]
pub enum SeverityCalculationMode {
    /// Fixed thresholds
    /// Adaptive based on historical data
    /// Percentile-based
/// Comprehensive regression analyzer
pub struct RegressionAnalyzer {
/// Historical performance data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    /// Timestamp of measurement
    /// Compilation time
    /// Runtime performance (if available)
    /// Binary size
    /// Memory usage
    /// Associated baseline ID
/// Statistical analysis result for a performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    /// Mean value
    /// Standard deviation
    /// Confidence interval (lower bound, upper bound)
    /// Number of samples
    /// Trend direction (1.0 = improving, -1.0 = degrading, 0.0 = stable)
    /// Statistical significance of any detected change
/// Detailed regression analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedRegressionAnalysis {
    /// Basic regressions detected
    /// Statistical analysis for each metric
    /// Baseline comparison results
    /// Trend analysis over time
    /// Overall regression status
    /// Recommendations for addressing regressions
/// Trend analysis over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// Performance trends for each benchmark
    /// Overall system trend
    /// Trend prediction for next measurements
/// Performance trend for a specific benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTrend {
    /// Benchmark name
    /// Trend for compilation time
    /// Trend for runtime performance
    /// Trend for binary size
    /// Trend for memory usage
/// Trend for a specific metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTrend {
    /// Trend direction (positive = worsening, negative = improving)
    /// Strength of trend (0.0 to 1.0)
    /// Whether trend is statistically significant
    /// Projected change over next period
/// Overall system performance trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTrend {
    /// Overall performance health score (0.0 to 1.0)
    /// Main areas of concern
    /// Areas showing improvement
    /// Stability assessment
/// Performance prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePrediction {
    /// Predicted value
    /// Confidence in prediction (0.0 to 1.0)
    /// Prediction interval (lower, upper)
    /// Time horizon for prediction
/// Recommendation for addressing a regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionRecommendation {
    /// Type of regression this addresses
    /// Priority level (1 = highest)
    /// Recommendation text
    /// Estimated effort to implement
    /// Expected impact
/// Effort level for implementing a recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
/// Expected impact level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
impl RegressionAnalyzer {
    /// Create a new regression analyzer
    pub fn new(config: RegressionAnalysisConfig) -> Self {
        Self {
        }
    }

    /// Perform comprehensive regression analysis
    #[instrument(skip(self, current_results, baseline))]
    pub fn analyze_regressions(
    ) -> Result<DetailedRegressionAnalysis> {
        info!("Starting comprehensive regression analysis for {} benchmarks", current_results.len());

        let mut regressions = Vec::new();
        let mut statistical_analysis = HashMap::new();
        let mut recommendations = Vec::new();

        // Update historical performance data
        self.update_historical_data(current_results);

        // Analyze each benchmark against baseline
        for result in current_results {
            if !result.success {
                continue;
            // Compile time regression analysis
            if let Some(compile_regression) = self.analyze_compile_time_regression(result, baseline)? {
                regressions.push(compile_regression);
            // Binary size regression analysis
            if let Some(size_regression) = self.analyze_binary_size_regression(result, baseline)? {
                regressions.push(size_regression);
            // Memory usage regression analysis
            if let Some(memory_regression) = self.analyze_memory_usage_regression(result, baseline)? {
                regressions.push(memory_regression);
            // Runtime performance regression analysis
            if let Some(runtime_regression) = self.analyze_runtime_regression(result, baseline)? {
                regressions.push(runtime_regression);
            // Statistical analysis for this benchmark
            if let Some(stats) = self.calculate_statistical_analysis(&result.name)? {
                statistical_analysis.insert(result.name.clone(), stats);
            }
        }

        // Generate baseline comparison
        let baseline_comparison = if let Some(baseline) = baseline {
            Some(self.create_baseline_comparison(current_results, baseline)?)
        } else {
            None

        // Generate trend analysis if enabled
        let trend_analysis = if self.config.enable_trend_analysis {
            Some(self.generate_trend_analysis(current_results)?)
        } else {
            None

        // Generate recommendations
        recommendations.extend(self.generate_recommendations(&regressions, &statistical_analysis)?);

        let has_critical_regressions = regressions.iter()
            .any(|r| matches!(r.severity, RegressionSeverity::Critical));

        let analysis = DetailedRegressionAnalysis {

        info!(
            "Regression analysis completed"
        );

        Ok(analysis)
    /// Analyze compilation time regression
    fn analyze_compile_time_regression(
    ) -> Result<Option<PerformanceRegression>> {
        let baseline_benchmark = baseline
            .and_then(|b| b.benchmarks.get(&result.name))?;

        if let Some(baseline_benchmark) = baseline_benchmark {
            let baseline_time = baseline_benchmark.compile_time_metrics.mean.as_secs_f64();
            let current_time = result.compile_time.as_secs_f64();
            
            if baseline_time > 0.0 {
                let increase_percent = ((current_time - baseline_time) / baseline_time) * 100.0;
                
                if increase_percent > self.config.thresholds.max_compile_time_increase {
                    let severity = self.calculate_severity(
                    );

                    return Ok(Some(PerformanceRegression {
                        description: format!(
                            self.config.thresholds.max_compile_time_increase
                    }));
                }
            }
        Ok(None)
    /// Analyze binary size regression
    fn analyze_binary_size_regression(
    ) -> Result<Option<PerformanceRegression>> {
        let baseline_benchmark = baseline
            .and_then(|b| b.benchmarks.get(&result.name))?;

        if let Some(baseline_benchmark) = baseline_benchmark {
            let baseline_size = baseline_benchmark.binary_size as f64;
            let current_size = result.binary_size as f64;
            
            if baseline_size > 0.0 {
                let increase_percent = ((current_size - baseline_size) / baseline_size) * 100.0;
                
                if increase_percent > self.config.thresholds.max_size_increase {
                    let severity = self.calculate_severity(
                    );

                    return Ok(Some(PerformanceRegression {
                        description: format!(
                            self.config.thresholds.max_size_increase
                    }));
                }
            }
        Ok(None)
    /// Analyze memory usage regression
    fn analyze_memory_usage_regression(
    ) -> Result<Option<PerformanceRegression>> {
        let baseline_benchmark = baseline
            .and_then(|b| b.benchmarks.get(&result.name))?;

        if let Some(baseline_benchmark) = baseline_benchmark {
            let baseline_memory = baseline_benchmark.peak_memory_usage as f64;
            let current_memory = result.peak_memory_usage as f64;
            
            if baseline_memory > 0.0 {
                let increase_percent = ((current_memory - baseline_memory) / baseline_memory) * 100.0;
                
                if increase_percent > self.config.thresholds.max_memory_increase {
                    let severity = self.calculate_severity(
                    );

                    return Ok(Some(PerformanceRegression {
                        description: format!(
                            self.config.thresholds.max_memory_increase
                    }));
                }
            }
        Ok(None)
    /// Analyze runtime performance regression
    fn analyze_runtime_regression(
    ) -> Result<Option<PerformanceRegression>> {
        let baseline_benchmark = baseline
            .and_then(|b| b.benchmarks.get(&result.name))?;

        if let (Some(current_runtime), Some(baseline_benchmark)) = (result.runtime_performance, baseline_benchmark) {
            if let Some(ref baseline_runtime_metrics) = baseline_benchmark.runtime_metrics {
                let baseline_time = baseline_runtime_metrics.mean.as_secs_f64();
                let current_time = current_runtime.as_secs_f64();
                
                if baseline_time > 0.0 {
                    let change_percent = ((current_time - baseline_time) / baseline_time) * 100.0;
                    
                    // For runtime, we expect improvement (negative change), so regression is positive change
                    // beyond the minimum expected improvement threshold
                    if change_percent > (self.config.thresholds.min_runtime_improvement * -1.0) {
                        let severity = self.calculate_severity(
                        );

                        return Ok(Some(PerformanceRegression {
                            description: format!(
                                self.config.thresholds.min_runtime_improvement
                        }));
                    }
                }
            }
        }

        Ok(None)
    /// Calculate regression severity based on the deviation from threshold
    fn calculate_severity(
    ) -> RegressionSeverity {
        let deviation = (actual_value - threshold).abs();
        let relative_deviation = deviation / threshold.abs();

        match self.config.severity_mode {
            SeverityCalculationMode::Fixed => {
                if relative_deviation > 2.0 {
                    RegressionSeverity::Critical
                } else if relative_deviation > 1.0 {
                    RegressionSeverity::Major
                } else if relative_deviation > 0.5 {
                    RegressionSeverity::Minor
                } else {
                    RegressionSeverity::Warning
                }
            }
            SeverityCalculationMode::Adaptive => {
                // Adjust severity based on regression type
                let multiplier = match regression_type {
                    RegressionType::RuntimePerformance => 1.5, // Runtime regressions are more serious

                let adjusted_deviation = relative_deviation * multiplier;
                
                if adjusted_deviation > 1.5 {
                    RegressionSeverity::Critical
                } else if adjusted_deviation > 1.0 {
                    RegressionSeverity::Major
                } else if adjusted_deviation > 0.3 {
                    RegressionSeverity::Minor
                } else {
                    RegressionSeverity::Warning
                }
            }
            SeverityCalculationMode::Percentile => {
                // Use historical data to determine percentile-based severity
                // This would require more sophisticated historical analysis
                // For now, fall back to fixed mode
                self.calculate_severity(actual_value, threshold, regression_type)
            }
        }
    /// Update historical performance data
    fn update_historical_data(&mut self, results: &[BenchmarkResult]) {
        let timestamp = chrono::Utc::now();
        
        for result in results {
            if !result.success {
                continue;
            let data_point = PerformanceDataPoint {
                baseline_id: None, // Could be set if we know the baseline

            self.historical_performance
                .entry(result.name.clone())
                .or_insert_with(Vec::new)
                .push(data_point);

            // Keep only recent data points to manage memory
            let history = self.historical_performance.get_mut(&result.name).unwrap();
            if history.len() > 100 {
                history.drain(0..50);
            }
        }
    /// Calculate statistical analysis for a benchmark
    fn calculate_statistical_analysis(&self, benchmark_name: &str) -> Result<Option<StatisticalAnalysis>> {
        let history = match self.historical_performance.get(benchmark_name) {

        // Calculate statistics for compile time
        let compile_times: Vec<f64> = history.iter()
            .map(|dp| dp.compile_time.as_secs_f64())
            .collect();

        let mean = compile_times.iter().sum::<f64>() / compile_times.len() as f64;
        let variance = compile_times.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (compile_times.len() - 1) as f64;
        let std_dev = variance.sqrt();

        // Calculate confidence interval
        let t_critical = 1.96; // For 95% confidence (approximation)
        let margin_of_error = t_critical * (std_dev / (compile_times.len() as f64).sqrt());
        let confidence_interval = (mean - margin_of_error, mean + margin_of_error);

        // Calculate trend direction using linear regression
        let trend_direction = self.calculate_trend_direction(&compile_times)?;

        // Calculate significance level (simplified)
        let significance_level = if trend_direction.abs() > 0.1 { 0.95 } else { 0.5 };

        Ok(Some(StatisticalAnalysis {
        }))
    /// Calculate trend direction using simple linear regression
    fn calculate_trend_direction(&self, values: &[f64]) -> Result<f64> {
        if values.len() < 2 {
            return Ok(0.0);
        let n = values.len() as f64;
        let x_mean = (n - 1.0) / 2.0; // x values are 0, 1, 2, ...
        let y_mean = values.iter().sum::<f64>() / n;

        let numerator: f64 = values.iter().enumerate()
            .map(|(i, &y)| (i as f64 - x_mean) * (y - y_mean))
            .sum();

        let denominator: f64 = (0..values.len())
            .map(|i| (i as f64 - x_mean).powi(2))
            .sum();

        if denominator == 0.0 {
            Ok(0.0)
        } else {
            Ok(numerator / denominator)
        }
    }

    /// Create baseline comparison
    fn create_baseline_comparison(
    ) -> Result<BaselineComparison> {
        let mut improvements = Vec::new();
        let mut total_improvement = 0.0;
        let mut improvement_count = 0;

        for result in current_results {
            if !result.success {
                continue;
            if let Some(baseline_benchmark) = baseline.benchmarks.get(&result.name) {
                // Calculate compilation time improvement
                let baseline_time = baseline_benchmark.compile_time_metrics.mean.as_secs_f64();
                let current_time = result.compile_time.as_secs_f64();
                
                if baseline_time > 0.0 {
                    let improvement_percent = ((baseline_time - current_time) / baseline_time) * 100.0;
                    
                    improvements.push(PerformanceImprovement {
                    });

                    total_improvement += improvement_percent;
                    improvement_count += 1;
                // Calculate binary size improvement
                let baseline_size = baseline_benchmark.binary_size as f64;
                let current_size = result.binary_size as f64;
                
                if baseline_size > 0.0 {
                    let size_improvement = ((baseline_size - current_size) / baseline_size) * 100.0;
                    
                    improvements.push(PerformanceImprovement {
                    });
                }
            }
        let overall_improvement = if improvement_count > 0 {
            total_improvement / improvement_count as f64
        } else {
            0.0

        Ok(BaselineComparison {
        })
    /// Generate trend analysis
    fn generate_trend_analysis(&self, _current_results: &[BenchmarkResult]) -> Result<TrendAnalysis> {
        // This is a simplified implementation
        // In a real system, this would do sophisticated trend analysis
        
        let mut benchmark_trends = HashMap::new();
        let overall_trend = SystemTrend {

        Ok(TrendAnalysis {
        })
    /// Generate recommendations for addressing regressions
    fn generate_recommendations(
    ) -> Result<Vec<RegressionRecommendation>> {
        let mut recommendations = Vec::new();

        for regression in regressions {
            let recommendation = match regression.regression_type {
                RegressionType::CompileTime => RegressionRecommendation {
                    priority: match regression.severity {
                RegressionType::BinarySize => RegressionRecommendation {
                RegressionType::MemoryUsage => RegressionRecommendation {
                RegressionType::RuntimePerformance => RegressionRecommendation {

            recommendations.push(recommendation);
        Ok(recommendations)
    }
}

