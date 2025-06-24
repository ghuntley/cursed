
// Comprehensive Regression Analysis System
//
// Provides advanced regression detection and analysis capabilities for
// performance monitoring and quality assurance.

use std::collections::HashMap;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, instrument};

use crate::error::{Error, Result};
use crate::optimization::benchmarks::{
    BenchmarkResult, PerformanceRegression, RegressionType, RegressionSeverity,
    BaselineComparison, PerformanceImprovement, ImprovementCategory,
    PerformanceThresholds,
};

use crate::optimization::baseline_storage::{PerformanceBaseline, BaselineBenchmark};
use std::path::PathBuf;

/// Configuration for regression analysis
#[derive(Debug, Clone)]
pub struct RegressionAnalysisConfig {
    /// Performance thresholds for different metrics
    pub thresholds: PerformanceThresholds,
    /// Statistical confidence level (0.0 to 1.0)
    pub confidence_level: f64,
    /// Minimum sample size for statistical analysis
    pub min_sample_size: usize,
    /// Enable trend analysis
    pub enable_trend_analysis: bool,
    /// Severity calculation mode
    pub severity_mode: SeverityCalculationMode,
}

impl Default for RegressionAnalysisConfig {
    fn default() -> Self {
        Self {
            thresholds: PerformanceThresholds::default(),
            confidence_level: 0.95,
            min_sample_size: 3,
            enable_trend_analysis: true,
            severity_mode: SeverityCalculationMode::Adaptive,
        }
    }
}

/// Mode for calculating regression severity
#[derive(Debug, Clone)]
pub enum SeverityCalculationMode {
    /// Fixed thresholds
    Fixed,
    /// Adaptive based on historical data
    Adaptive,
    /// Percentile-based
    Percentile,
}

/// Comprehensive regression analyzer
pub struct RegressionAnalyzer {
    config: RegressionAnalysisConfig,
    historical_performance: HashMap<String, Vec<PerformanceDataPoint>>,
}

/// Historical performance data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    /// Timestamp of measurement
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Compilation time
    pub compile_time: Duration,
    /// Runtime performance (if available)
    pub runtime_performance: Option<Duration>,
    /// Binary size
    pub binary_size: usize,
    /// Memory usage
    pub memory_usage: usize,
    /// Associated baseline ID
    pub baseline_id: Option<String>,
}

/// Statistical analysis result for a performance metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    /// Mean value
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Confidence interval (lower bound, upper bound)
    pub confidence_interval: (f64, f64),
    /// Number of samples
    pub sample_count: usize,
    /// Trend direction (1.0 = improving, -1.0 = degrading, 0.0 = stable)
    pub trend_direction: f64,
    /// Statistical significance of any detected change
    pub significance_level: f64,
}

/// Detailed regression analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedRegressionAnalysis {
    /// Basic regressions detected
    pub regressions: Vec<PerformanceRegression>,
    /// Statistical analysis for each metric
    pub statistical_analysis: HashMap<String, StatisticalAnalysis>,
    /// Baseline comparison results
    pub baseline_comparison: Option<BaselineComparison>,
    /// Trend analysis over time
    pub trend_analysis: Option<TrendAnalysis>,
    /// Overall regression status
    pub has_critical_regressions: bool,
    /// Recommendations for addressing regressions
    pub recommendations: Vec<RegressionRecommendation>,
}

/// Trend analysis over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// Performance trends for each benchmark
    pub benchmark_trends: HashMap<String, BenchmarkTrend>,
    /// Overall system trend
    pub overall_trend: SystemTrend,
    /// Trend prediction for next measurements
    pub predictions: HashMap<String, PerformancePrediction>,
}

/// Performance trend for a specific benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTrend {
    /// Benchmark name
    pub benchmark_name: String,
    /// Trend for compilation time
    pub compile_time_trend: MetricTrend,
    /// Trend for runtime performance
    pub runtime_trend: Option<MetricTrend>,
    /// Trend for binary size
    pub binary_size_trend: MetricTrend,
    /// Trend for memory usage
    pub memory_usage_trend: MetricTrend,
}

/// Trend for a specific metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTrend {
    /// Trend direction (positive = worsening, negative = improving)
    pub direction: f64,
    /// Strength of trend (0.0 to 1.0)
    pub strength: f64,
    /// Whether trend is statistically significant
    pub is_significant: bool,
    /// Projected change over next period
    pub projected_change_percent: f64,
}

/// Overall system performance trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTrend {
    /// Overall performance health score (0.0 to 1.0)
    pub health_score: f64,
    /// Main areas of concern
    pub concern_areas: Vec<String>,
    /// Areas showing improvement
    pub improvement_areas: Vec<String>,
    /// Stability assessment
    pub stability_score: f64,
}

/// Performance prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePrediction {
    /// Predicted value
    pub predicted_value: f64,
    /// Confidence in prediction (0.0 to 1.0)
    pub confidence: f64,
    /// Prediction interval (lower, upper)
    pub prediction_interval: (f64, f64),
    /// Time horizon for prediction
    pub horizon_days: u32,
}

/// Recommendation for addressing a regression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionRecommendation {
    /// Type of regression this addresses
    pub regression_type: RegressionType,
    /// Priority level (1 = highest)
    pub priority: u32,
    /// Recommendation text
    pub recommendation: String,
    /// Estimated effort to implement
    pub estimated_effort: EffortLevel,
    /// Expected impact
    pub expected_impact: ImpactLevel,
}

/// Effort level for implementing a recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Expected impact level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Minor,
    Moderate,
    Significant,
    Major,
}

impl RegressionAnalyzer {
    /// Create a new regression analyzer
    pub fn new(config: RegressionAnalysisConfig) -> Self {
        Self {
            config,
            historical_performance: HashMap::new(),
        }
    }

    /// Perform comprehensive regression analysis
    #[instrument(skip(self, current_results, baseline))]
    pub fn analyze_regressions(
        &mut self,
        current_results: &[BenchmarkResult],
        baseline: Option<&PerformanceBaseline>,
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
            }

            // Compile time regression analysis
            if let Some(compile_regression) = self.analyze_compile_time_regression(result, baseline)? {
                regressions.push(compile_regression);
            }

            // Binary size regression analysis
            if let Some(size_regression) = self.analyze_binary_size_regression(result, baseline)? {
                regressions.push(size_regression);
            }

            // Memory usage regression analysis
            if let Some(memory_regression) = self.analyze_memory_usage_regression(result, baseline)? {
                regressions.push(memory_regression);
            }

            // Runtime performance regression analysis
            if let Some(runtime_regression) = self.analyze_runtime_regression(result, baseline)? {
                regressions.push(runtime_regression);
            }

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
        };

        // Generate trend analysis if enabled
        let trend_analysis = if self.config.enable_trend_analysis {
            Some(self.generate_trend_analysis(current_results)?)
        } else {
            None
        };

        // Generate recommendations
        recommendations.extend(self.generate_recommendations(&regressions, &statistical_analysis)?);

        let has_critical_regressions = regressions.iter()
            .any(|r| matches!(r.severity, RegressionSeverity::Critical));

        let analysis = DetailedRegressionAnalysis {
            regressions,
            statistical_analysis,
            baseline_comparison,
            trend_analysis,
            has_critical_regressions,
            recommendations,
        };

        info!(
            regressions_found = analysis.regressions.len(),
            critical_regressions = analysis.has_critical_regressions,
            "Regression analysis completed"
        );

        Ok(analysis)
    }

    /// Analyze compilation time regression
    fn analyze_compile_time_regression(
        &self,
        result: &BenchmarkResult,
        baseline: Option<&PerformanceBaseline>,
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
                        increase_percent,
                        self.config.thresholds.max_compile_time_increase,
                        RegressionType::CompileTime,
                    );

                    return Ok(Some(PerformanceRegression {
                        benchmark_name: result.name.clone(),
                        regression_type: RegressionType::CompileTime,
                        severity,
                        actual_value: increase_percent,
                        expected_value: self.config.thresholds.max_compile_time_increase,
                        description: format!(
                            "Compilation time increased by {:.1}% (from {:.3}s to {:.3}s), exceeding threshold of {:.1}%",
                            increase_percent,
                            baseline_time,
                            current_time,
                            self.config.thresholds.max_compile_time_increase
                        ),
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Analyze binary size regression
    fn analyze_binary_size_regression(
        &self,
        result: &BenchmarkResult,
        baseline: Option<&PerformanceBaseline>,
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
                        increase_percent,
                        self.config.thresholds.max_size_increase,
                        RegressionType::BinarySize,
                    );

                    return Ok(Some(PerformanceRegression {
                        benchmark_name: result.name.clone(),
                        regression_type: RegressionType::BinarySize,
                        severity,
                        actual_value: increase_percent,
                        expected_value: self.config.thresholds.max_size_increase,
                        description: format!(
                            "Binary size increased by {:.1}% (from {} to {} bytes), exceeding threshold of {:.1}%",
                            increase_percent,
                            baseline_size as usize,
                            current_size as usize,
                            self.config.thresholds.max_size_increase
                        ),
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Analyze memory usage regression
    fn analyze_memory_usage_regression(
        &self,
        result: &BenchmarkResult,
        baseline: Option<&PerformanceBaseline>,
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
                        increase_percent,
                        self.config.thresholds.max_memory_increase,
                        RegressionType::MemoryUsage,
                    );

                    return Ok(Some(PerformanceRegression {
                        benchmark_name: result.name.clone(),
                        regression_type: RegressionType::MemoryUsage,
                        severity,
                        actual_value: increase_percent,
                        expected_value: self.config.thresholds.max_memory_increase,
                        description: format!(
                            "Memory usage increased by {:.1}% (from {} to {} bytes), exceeding threshold of {:.1}%",
                            increase_percent,
                            baseline_memory as usize,
                            current_memory as usize,
                            self.config.thresholds.max_memory_increase
                        ),
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Analyze runtime performance regression
    fn analyze_runtime_regression(
        &self,
        result: &BenchmarkResult,
        baseline: Option<&PerformanceBaseline>,
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
                            change_percent,
                            self.config.thresholds.min_runtime_improvement * -1.0,
                            RegressionType::RuntimePerformance,
                        );

                        return Ok(Some(PerformanceRegression {
                            benchmark_name: result.name.clone(),
                            regression_type: RegressionType::RuntimePerformance,
                            severity,
                            actual_value: change_percent,
                            expected_value: self.config.thresholds.min_runtime_improvement * -1.0,
                            description: format!(
                                "Runtime performance degraded by {:.1}% (from {:.3}s to {:.3}s), failing to meet improvement target of {:.1}%",
                                change_percent,
                                baseline_time,
                                current_time,
                                self.config.thresholds.min_runtime_improvement
                            ),
                        }));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Calculate regression severity based on the deviation from threshold
    fn calculate_severity(
        &self,
        actual_value: f64,
        threshold: f64,
        regression_type: RegressionType,
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
                    RegressionType::CompileTime => 1.0,
                    RegressionType::RuntimePerformance => 1.5, // Runtime regressions are more serious
                    RegressionType::BinarySize => 0.8,
                    RegressionType::MemoryUsage => 1.2,
                };

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
    }

    /// Update historical performance data
    fn update_historical_data(&mut self, results: &[BenchmarkResult]) {
        let timestamp = chrono::Utc::now();
        
        for result in results {
            if !result.success {
                continue;
            }

            let data_point = PerformanceDataPoint {
                timestamp,
                compile_time: result.compile_time,
                runtime_performance: result.runtime_performance,
                binary_size: result.binary_size,
                memory_usage: result.peak_memory_usage,
                baseline_id: None, // Could be set if we know the baseline
            };

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
    }

    /// Calculate statistical analysis for a benchmark
    fn calculate_statistical_analysis(&self, benchmark_name: &str) -> Result<Option<StatisticalAnalysis>> {
        let history = match self.historical_performance.get(benchmark_name) {
            Some(history) if history.len() >= self.config.min_sample_size => history,
            _ => return Ok(None),
        };

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
            mean,
            std_dev,
            confidence_interval,
            sample_count: compile_times.len(),
            trend_direction,
            significance_level,
        }))
    }

    /// Calculate trend direction using simple linear regression
    fn calculate_trend_direction(&self, values: &[f64]) -> Result<f64> {
        if values.len() < 2 {
            return Ok(0.0);
        }

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
        &self,
        current_results: &[BenchmarkResult],
        baseline: &PerformanceBaseline,
    ) -> Result<BaselineComparison> {
        let mut improvements = Vec::new();
        let mut total_improvement = 0.0;
        let mut improvement_count = 0;

        for result in current_results {
            if !result.success {
                continue;
            }

            if let Some(baseline_benchmark) = baseline.benchmarks.get(&result.name) {
                // Calculate compilation time improvement
                let baseline_time = baseline_benchmark.compile_time_metrics.mean.as_secs_f64();
                let current_time = result.compile_time.as_secs_f64();
                
                if baseline_time > 0.0 {
                    let improvement_percent = ((baseline_time - current_time) / baseline_time) * 100.0;
                    
                    improvements.push(PerformanceImprovement {
                        benchmark_name: result.name.clone(),
                        improvement_percentage: improvement_percent,
                        improvement_category: ImprovementCategory::CompileTime,
                    });

                    total_improvement += improvement_percent;
                    improvement_count += 1;
                }

                // Calculate binary size improvement
                let baseline_size = baseline_benchmark.binary_size as f64;
                let current_size = result.binary_size as f64;
                
                if baseline_size > 0.0 {
                    let size_improvement = ((baseline_size - current_size) / baseline_size) * 100.0;
                    
                    improvements.push(PerformanceImprovement {
                        benchmark_name: format!("{}_size", result.name),
                        improvement_percentage: size_improvement,
                        improvement_category: ImprovementCategory::BinarySize,
                    });
                }
            }
        }

        let overall_improvement = if improvement_count > 0 {
            total_improvement / improvement_count as f64
        } else {
            0.0
        };

        Ok(BaselineComparison {
            baseline_file: PathBuf::from(format!("{}.json", baseline.baseline_id)),
            improvements,
            overall_improvement,
        })
    }

    /// Generate trend analysis
    fn generate_trend_analysis(&self, _current_results: &[BenchmarkResult]) -> Result<TrendAnalysis> {
        // This is a simplified implementation
        // In a real system, this would do sophisticated trend analysis
        
        let mut benchmark_trends = HashMap::new();
        let overall_trend = SystemTrend {
            health_score: 0.8,
            concern_areas: vec!["compilation_time".to_string()],
            improvement_areas: vec!["binary_size".to_string()],
            stability_score: 0.9,
        };

        Ok(TrendAnalysis {
            benchmark_trends,
            overall_trend,
            predictions: HashMap::new(),
        })
    }

    /// Generate recommendations for addressing regressions
    fn generate_recommendations(
        &self,
        regressions: &[PerformanceRegression],
        _statistical_analysis: &HashMap<String, StatisticalAnalysis>,
    ) -> Result<Vec<RegressionRecommendation>> {
        let mut recommendations = Vec::new();

        for regression in regressions {
            let recommendation = match regression.regression_type {
                RegressionType::CompileTime => RegressionRecommendation {
                    regression_type: regression.regression_type.clone(),
                    priority: match regression.severity {
                        RegressionSeverity::Critical => 1,
                        RegressionSeverity::Major => 2,
                        RegressionSeverity::Minor => 3,
                        RegressionSeverity::Warning => 4,
                    },
                    recommendation: "Consider enabling parallel compilation, reviewing optimization levels, or improving dependency analysis".to_string(),
                    estimated_effort: EffortLevel::Medium,
                    expected_impact: ImpactLevel::Significant,
                },
                RegressionType::BinarySize => RegressionRecommendation {
                    regression_type: regression.regression_type.clone(),
                    priority: 3,
                    recommendation: "Review dead code elimination passes, enable link-time optimization, or check for debug symbol inclusion".to_string(),
                    estimated_effort: EffortLevel::Low,
                    expected_impact: ImpactLevel::Moderate,
                },
                RegressionType::MemoryUsage => RegressionRecommendation {
                    regression_type: regression.regression_type.clone(),
                    priority: 2,
                    recommendation: "Investigate memory leaks, optimize data structures, or reduce concurrent allocations".to_string(),
                    estimated_effort: EffortLevel::High,
                    expected_impact: ImpactLevel::Major,
                },
                RegressionType::RuntimePerformance => RegressionRecommendation {
                    regression_type: regression.regression_type.clone(),
                    priority: 1,
                    recommendation: "Review LLVM optimization passes, enable profile-guided optimization, or investigate algorithmic changes".to_string(),
                    estimated_effort: EffortLevel::High,
                    expected_impact: ImpactLevel::Major,
                },
            };

            recommendations.push(recommendation);
        }

        Ok(recommendations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimization::benchmarks::BenchmarkResult;
    use crate::optimization::baseline_storage::{PerformanceBaseline, BaselineBenchmark, TimeMetrics, BaselineType};
    use crate::common::optimization_level::OptimizationLevel;

    fn create_test_baseline() -> PerformanceBaseline {
        let mut benchmarks = HashMap::new();
        benchmarks.insert("test_benchmark".to_string(), BaselineBenchmark {
            name: "test_benchmark".to_string(),
            compile_time_metrics: TimeMetrics {
                mean: Duration::from_secs(2),
                std_dev: Duration::from_millis(100),
                min: Duration::from_millis(1800),
                max: Duration::from_millis(2200),
                sample_count: 10,
                percentile_95: Duration::from_millis(2100),
            },
            runtime_metrics: Some(TimeMetrics {
                mean: Duration::from_millis(500),
                std_dev: Duration::from_millis(50),
                min: Duration::from_millis(450),
                max: Duration::from_millis(550),
                sample_count: 10,
                percentile_95: Duration::from_millis(540),
            }),
            binary_size: 1024,
            peak_memory_usage: 8192,
            optimization_passes: 10,
            custom_metrics: HashMap::new(),
        });

        PerformanceBaseline {
            baseline_id: "test_baseline".to_string(),
            name: "Test Baseline".to_string(),
            baseline_type: BaselineType::Manual,
            created_at: chrono::Utc::now(),
            git_commit: None,
            version: None,
            benchmarks,
            metadata: HashMap::new(),
            confidence_level: 0.9,
        }
    }

    fn create_regression_result() -> BenchmarkResult {
        BenchmarkResult {
            name: "test_benchmark".to_string(),
            optimization_level: OptimizationLevel::O2,
            compile_time: Duration::from_secs(4), // 100% increase from baseline
            runtime_performance: Some(Duration::from_millis(600)), // 20% degradation
            binary_size: 1536, // 50% increase
            peak_memory_usage: 12288, // 50% increase
            optimization_passes: 10,
            success: true,
            error_message: None,
        }
    }

    #[test]
    fn test_regression_analyzer_creation() {
        let config = RegressionAnalysisConfig::default();
        let _analyzer = RegressionAnalyzer::new(config);
    }

    #[test]
    fn test_compile_time_regression_detection() {
        let config = RegressionAnalysisConfig::default();
        let analyzer = RegressionAnalyzer::new(config);
        let baseline = create_test_baseline();
        let result = create_regression_result();

        let regression = analyzer.analyze_compile_time_regression(&result, Some(&baseline)).unwrap();
        assert!(regression.is_some());
        
        let regression = regression.unwrap();
        assert_eq!(regression.regression_type, RegressionType::CompileTime);
        assert!(regression.actual_value > 50.0); // Should detect the 100% increase
    }

    #[test]
    fn test_binary_size_regression_detection() {
        let config = RegressionAnalysisConfig::default();
        let analyzer = RegressionAnalyzer::new(config);
        let baseline = create_test_baseline();
        let result = create_regression_result();

        let regression = analyzer.analyze_binary_size_regression(&result, Some(&baseline)).unwrap();
        assert!(regression.is_some());
        
        let regression = regression.unwrap();
        assert_eq!(regression.regression_type, RegressionType::BinarySize);
        assert!(regression.actual_value > 20.0); // Should detect the 50% increase
    }

    #[test]
    fn test_severity_calculation() {
        let config = RegressionAnalysisConfig::default();
        let analyzer = RegressionAnalyzer::new(config);

        // Test different severity levels
        let critical = analyzer.calculate_severity(200.0, 50.0, RegressionType::CompileTime);
        assert!(matches!(critical, RegressionSeverity::Critical));

        let major = analyzer.calculate_severity(100.0, 50.0, RegressionType::CompileTime);
        assert!(matches!(major, RegressionSeverity::Major));

        let minor = analyzer.calculate_severity(75.0, 50.0, RegressionType::CompileTime);
        assert!(matches!(minor, RegressionSeverity::Minor));
    }
}
