//! Baseline comparison system for performance regression detection

use crate::error::{Result, CursedError};
use crate::optimization::{BenchmarkResult, BenchmarkSuiteResult};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn, instrument};

/// Baseline performance data for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineData {
    /// When this baseline was created
    pub timestamp: u64,
    /// Version identifier for the baseline
    pub version: String,
    /// Benchmark results that form the baseline
    pub benchmark_results: HashMap<String, BaselineMetrics>,
    /// Metadata about the baseline
    pub metadata: BaselineMetadata,
}

/// Baseline metadata for context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetadata {
    /// Commit hash or version when baseline was created
    pub commit_hash: Option<String>,
    /// Environment information
    pub environment: EnvironmentInfo,
    /// Compiler configuration used
    pub compiler_config: String,
    /// Additional notes
    pub notes: Option<String>,
}

/// Environment information for baseline context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Operating system
    pub os: String,
    /// CPU architecture
    pub arch: String,
    /// Number of CPU cores
    pub cpu_cores: usize,
    /// Available memory in MB
    pub memory_mb: u64,
}

/// Baseline metrics for a specific benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetrics {
    /// Average execution time
    pub avg_time_ms: f64,
    /// Standard deviation of execution time
    pub std_dev_ms: f64,
    /// Minimum execution time observed
    pub min_time_ms: f64,
    /// Maximum execution time observed
    pub max_time_ms: f64,
    /// Memory usage metrics
    pub memory_usage_mb: f64,
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Number of iterations in the baseline
    pub iterations: usize,
}

/// Result of baseline comparison
#[derive(Debug, Clone)]
pub struct BaselineComparisonResult {
    /// Whether any regressions were detected
    pub has_regressions: bool,
    /// Whether any improvements were detected
    pub has_improvements: bool,
    /// Detailed comparison for each benchmark
    pub benchmark_comparisons: HashMap<String, BenchmarkComparison>,
    /// Overall performance change percentage
    pub overall_change_percent: f64,
    /// Summary of findings
    pub summary: String,
}

/// Comparison result for a single benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkComparison {
    /// Name of the benchmark
    pub benchmark_name: String,
    /// Current performance metrics
    pub current_metrics: BaselineMetrics,
    /// Baseline performance metrics
    pub baseline_metrics: BaselineMetrics,
    /// Performance change (positive = improvement, negative = regression)
    pub performance_change_percent: f64,
    /// Whether this is considered a regression
    pub is_regression: bool,
    /// Whether this is considered an improvement
    pub is_improvement: bool,
    /// Confidence level of the comparison
    pub confidence_level: f64,
    /// Detailed analysis
    pub analysis: String,
}

/// Configuration for baseline comparison
#[derive(Debug, Clone)]
pub struct BaselineComparisonConfig {
    /// Threshold for considering a change a regression (percentage)
    pub regression_threshold_percent: f64,
    /// Threshold for considering a change an improvement (percentage)
    pub improvement_threshold_percent: f64,
    /// Minimum confidence level required for reporting changes
    pub min_confidence_level: f64,
    /// Maximum age of baseline data to consider valid (in days)
    pub max_baseline_age_days: u64,
    /// Whether to include statistical significance testing
    pub use_statistical_testing: bool,
}

impl Default for BaselineComparisonConfig {
    fn default() -> Self {
        Self {
            regression_threshold_percent: 5.0,
            improvement_threshold_percent: 5.0,
            min_confidence_level: 0.8,
            max_baseline_age_days: 30,
            use_statistical_testing: true,
        }
    }
}

/// Baseline comparison engine
pub struct BaselineComparator {
    config: BaselineComparisonConfig,
    baseline_storage_path: PathBuf,
}

impl BaselineComparator {
    /// Create a new baseline comparator
    pub fn new<P: AsRef<Path>>(storage_path: P, config: BaselineComparisonConfig) -> Self {
        Self {
            config,
            baseline_storage_path: storage_path.as_ref().to_path_buf(),
        }
    }

    /// Create a new baseline from benchmark results
    #[instrument(skip(self, benchmark_results))]
    pub fn create_baseline(
        &self,
        benchmark_results: &BenchmarkSuiteResult,
        version: String,
        metadata: BaselineMetadata,
    ) -> Result<BaselineData> {
        info!("Creating new baseline with version: {}", version);

        let mut baseline_metrics = HashMap::new();

        // Convert benchmark results to baseline metrics
        for result in &benchmark_results.results {
            let metrics = self.convert_to_baseline_metrics(result)?;
            baseline_metrics.insert(result.benchmark_name.clone(), metrics);
        }

        let baseline = BaselineData {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            version,
            benchmark_results: baseline_metrics,
            metadata,
        };

        // Save baseline to storage
        self.save_baseline(&baseline)?;

        info!("Baseline created successfully with {} benchmarks", baseline_metrics.len());
        Ok(baseline)
    }

    /// Load the most recent baseline
    #[instrument(skip(self))]
    pub fn load_latest_baseline(&self) -> Result<Option<BaselineData>> {
        debug!("Loading latest baseline from storage");

        let baseline_file = self.baseline_storage_path.join("latest_baseline.json");
        if !baseline_file.exists() {
            debug!("No baseline file found");
            return Ok(None);
        }

        let baseline_data = std::fs::read_to_string(&baseline_file)
            .map_err(|e| CursedError::optimization_error(&format!("Failed to read baseline file: {}", e)))?;

        let baseline: BaselineData = serde_json::from_str(&baseline_data)
            .map_err(|e| CursedError::optimization_error(&format!("Failed to parse baseline data: {}", e)))?;

        // Check if baseline is not too old
        let baseline_age_seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() - baseline.timestamp;
        
        let max_age_seconds = self.config.max_baseline_age_days * 24 * 3600;
        
        if baseline_age_seconds > max_age_seconds {
            warn!("Baseline is {} days old, which exceeds maximum age of {} days", 
                  baseline_age_seconds / (24 * 3600), 
                  self.config.max_baseline_age_days);
            return Ok(None);
        }

        info!("Loaded baseline with version: {}", baseline.version);
        Ok(Some(baseline))
    }

    /// Compare current results against baseline
    #[instrument(skip(self, current_results, baseline))]
    pub fn compare_against_baseline(
        &self,
        current_results: &BenchmarkSuiteResult,
        baseline: &BaselineData,
    ) -> Result<BaselineComparisonResult> {
        info!("Comparing current results against baseline version: {}", baseline.version);

        let mut benchmark_comparisons = HashMap::new();
        let mut total_change_percent = 0.0;
        let mut comparison_count = 0;
        let mut has_regressions = false;
        let mut has_improvements = false;

        // Compare each benchmark
        for result in &current_results.results {
            if let Some(baseline_metrics) = baseline.benchmark_results.get(&result.benchmark_name) {
                let current_metrics = self.convert_to_baseline_metrics(result)?;
                let comparison = self.compare_benchmark_metrics(
                    &result.benchmark_name,
                    &current_metrics,
                    baseline_metrics,
                )?;

                if comparison.is_regression {
                    has_regressions = true;
                }
                if comparison.is_improvement {
                    has_improvements = true;
                }

                total_change_percent += comparison.performance_change_percent;
                comparison_count += 1;

                benchmark_comparisons.insert(result.benchmark_name.clone(), comparison);
            } else {
                warn!("No baseline data found for benchmark: {}", result.benchmark_name);
            }
        }

        let overall_change_percent = if comparison_count > 0 {
            total_change_percent / comparison_count as f64
        } else {
            0.0
        };

        let summary = self.generate_comparison_summary(
            &benchmark_comparisons,
            overall_change_percent,
            has_regressions,
            has_improvements,
        );

        let result = BaselineComparisonResult {
            has_regressions,
            has_improvements,
            benchmark_comparisons,
            overall_change_percent,
            summary,
        };

        info!("Baseline comparison completed. Overall change: {:.2}%, Regressions: {}, Improvements: {}", 
              overall_change_percent, has_regressions, has_improvements);

        Ok(result)
    }

    /// Convert benchmark result to baseline metrics
    fn convert_to_baseline_metrics(&self, result: &BenchmarkResult) -> Result<BaselineMetrics> {
        // Calculate statistics from benchmark result
        let times: Vec<f64> = result.execution_times.iter().map(|d| d.as_secs_f64() * 1000.0).collect();
        
        if times.is_empty() {
            return Err(CursedError::optimization_error("No execution times available for baseline metrics"));
        }

        let avg_time_ms = times.iter().sum::<f64>() / times.len() as f64;
        let min_time_ms = times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_time_ms = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Calculate standard deviation
        let variance = times.iter()
            .map(|&x| (x - avg_time_ms).powi(2))
            .sum::<f64>() / times.len() as f64;
        let std_dev_ms = variance.sqrt();

        Ok(BaselineMetrics {
            avg_time_ms,
            std_dev_ms,
            min_time_ms,
            max_time_ms,
            memory_usage_mb: result.memory_usage_mb.unwrap_or(0.0),
            cpu_utilization: result.cpu_utilization.unwrap_or(0.0),
            iterations: times.len(),
        })
    }

    /// Compare metrics between current and baseline
    fn compare_benchmark_metrics(
        &self,
        benchmark_name: &str,
        current: &BaselineMetrics,
        baseline: &BaselineMetrics,
    ) -> Result<BenchmarkComparison> {
        // Calculate performance change percentage (negative = regression)
        let performance_change_percent = if baseline.avg_time_ms > 0.0 {
            ((baseline.avg_time_ms - current.avg_time_ms) / baseline.avg_time_ms) * 100.0
        } else {
            0.0
        };

        // Determine if this is a regression or improvement
        let is_regression = performance_change_percent < -self.config.regression_threshold_percent;
        let is_improvement = performance_change_percent > self.config.improvement_threshold_percent;

        // Calculate confidence level using statistical significance
        let confidence_level = if self.config.use_statistical_testing {
            self.calculate_statistical_confidence(current, baseline)
        } else {
            1.0 // Assume high confidence if not using statistical testing
        };

        // Generate detailed analysis
        let analysis = self.generate_benchmark_analysis(
            benchmark_name,
            current,
            baseline,
            performance_change_percent,
            confidence_level,
        );

        Ok(BenchmarkComparison {
            benchmark_name: benchmark_name.to_string(),
            current_metrics: current.clone(),
            baseline_metrics: baseline.clone(),
            performance_change_percent,
            is_regression: is_regression && confidence_level >= self.config.min_confidence_level,
            is_improvement: is_improvement && confidence_level >= self.config.min_confidence_level,
            confidence_level,
            analysis,
        })
    }

    /// Calculate statistical confidence using t-test approximation
    fn calculate_statistical_confidence(&self, current: &BaselineMetrics, baseline: &BaselineMetrics) -> f64 {
        // Simplified t-test calculation for confidence
        let n1 = current.iterations as f64;
        let n2 = baseline.iterations as f64;
        
        if n1 < 2.0 || n2 < 2.0 {
            return 0.5; // Low confidence with insufficient data
        }

        let mean_diff = (current.avg_time_ms - baseline.avg_time_ms).abs();
        let pooled_std = ((current.std_dev_ms.powi(2) + baseline.std_dev_ms.powi(2)) / 2.0).sqrt();
        
        if pooled_std == 0.0 {
            return 1.0; // Perfect confidence if no variation
        }

        let standard_error = pooled_std * ((1.0 / n1) + (1.0 / n2)).sqrt();
        let t_stat = mean_diff / standard_error;
        
        // Simplified confidence calculation (approximation)
        let confidence = (t_stat / (t_stat + 2.0)).min(0.99).max(0.01);
        confidence
    }

    /// Generate analysis text for a benchmark comparison
    fn generate_benchmark_analysis(
        &self,
        benchmark_name: &str,
        current: &BaselineMetrics,
        baseline: &BaselineMetrics,
        change_percent: f64,
        confidence: f64,
    ) -> String {
        let mut analysis = Vec::new();

        analysis.push(format!("Benchmark: {}", benchmark_name));
        analysis.push(format!("Current avg time: {:.2}ms (std: {:.2}ms)", 
                             current.avg_time_ms, current.std_dev_ms));
        analysis.push(format!("Baseline avg time: {:.2}ms (std: {:.2}ms)", 
                             baseline.avg_time_ms, baseline.std_dev_ms));
        analysis.push(format!("Performance change: {:.2}%", change_percent));
        analysis.push(format!("Confidence level: {:.1}%", confidence * 100.0));

        if change_percent < -self.config.regression_threshold_percent {
            analysis.push("⚠️  REGRESSION DETECTED: Performance has degraded significantly".to_string());
        } else if change_percent > self.config.improvement_threshold_percent {
            analysis.push("✅ IMPROVEMENT: Performance has improved significantly".to_string());
        } else {
            analysis.push("➡️  STABLE: Performance is within acceptable variance".to_string());
        }

        // Memory usage analysis
        let memory_change = ((current.memory_usage_mb - baseline.memory_usage_mb) / baseline.memory_usage_mb.max(1.0)) * 100.0;
        if memory_change.abs() > 10.0 {
            analysis.push(format!("Memory usage change: {:.1}%", memory_change));
        }

        analysis.join("\n")
    }

    /// Generate overall comparison summary
    fn generate_comparison_summary(
        &self,
        comparisons: &HashMap<String, BenchmarkComparison>,
        overall_change: f64,
        has_regressions: bool,
        has_improvements: bool,
    ) -> String {
        let mut summary = Vec::new();
        
        summary.push(format!("=== Baseline Comparison Summary ==="));
        summary.push(format!("Overall performance change: {:.2}%", overall_change));
        summary.push(format!("Benchmarks analyzed: {}", comparisons.len()));

        let regression_count = comparisons.values().filter(|c| c.is_regression).count();
        let improvement_count = comparisons.values().filter(|c| c.is_improvement).count();
        let stable_count = comparisons.len() - regression_count - improvement_count;

        summary.push(format!("Regressions: {} | Improvements: {} | Stable: {}", 
                           regression_count, improvement_count, stable_count));

        if has_regressions {
            summary.push("⚠️  WARNING: Performance regressions detected!".to_string());
            for (name, comparison) in comparisons {
                if comparison.is_regression {
                    summary.push(format!("  - {}: {:.2}% slower", name, -comparison.performance_change_percent));
                }
            }
        }

        if has_improvements {
            summary.push("✅ Good news: Performance improvements detected!".to_string());
            for (name, comparison) in comparisons {
                if comparison.is_improvement {
                    summary.push(format!("  + {}: {:.2}% faster", name, comparison.performance_change_percent));
                }
            }
        }

        if !has_regressions && !has_improvements {
            summary.push("➡️  Performance is stable within acceptable variance".to_string());
        }

        summary.join("\n")
    }

    /// Save baseline to storage
    fn save_baseline(&self, baseline: &BaselineData) -> Result<()> {
        // Ensure storage directory exists
        if let Some(parent) = self.baseline_storage_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CursedError::optimization_error(&format!("Failed to create baseline directory: {}", e)))?;
        }

        let baseline_file = self.baseline_storage_path.join("latest_baseline.json");
        let serialized = serde_json::to_string_pretty(baseline)
            .map_err(|e| CursedError::optimization_error(&format!("Failed to serialize baseline: {}", e)))?;

        std::fs::write(&baseline_file, serialized)
            .map_err(|e| CursedError::optimization_error(&format!("Failed to write baseline file: {}", e)))?;

        // Also save a timestamped version for history
        let timestamped_file = self.baseline_storage_path.join(format!("baseline_{}.json", baseline.timestamp));
        std::fs::write(&timestamped_file, serde_json::to_string_pretty(baseline).unwrap_or_default())
            .map_err(|e| CursedError::optimization_error(&format!("Failed to write timestamped baseline: {}", e)))?;

        Ok(())
    }

    /// Get current environment information
    pub fn get_current_environment() -> EnvironmentInfo {
        EnvironmentInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cpu_cores: num_cpus::get(),
            memory_mb: Self::get_system_memory_mb(),
        }
    }

    /// Get system memory in MB (best effort)
    fn get_system_memory_mb() -> u64 {
        // This is a simplified implementation
        // In a real implementation, you'd use system APIs
        8192 // Default to 8GB if we can't detect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_baseline_comparator_creation() {
        let temp_dir = tempdir().unwrap();
        let config = BaselineComparisonConfig::default();
        let _comparator = BaselineComparator::new(temp_dir.path(), config);
    }

    #[test]
    fn test_baseline_metrics_conversion() {
        let temp_dir = tempdir().unwrap();
        let config = BaselineComparisonConfig::default();
        let comparator = BaselineComparator::new(temp_dir.path(), config);

        let benchmark_result = BenchmarkResult {
            benchmark_name: "test".to_string(),
            execution_times: vec![
                Duration::from_millis(100),
                Duration::from_millis(110),
                Duration::from_millis(90),
            ],
            memory_usage_mb: Some(256.0),
            cpu_utilization: Some(75.0),
            success: true,
            error_message: None,
        };

        let metrics = comparator.convert_to_baseline_metrics(&benchmark_result).unwrap();
        assert_eq!(metrics.iterations, 3);
        assert!((metrics.avg_time_ms - 100.0).abs() < 10.0);
    }

    #[test]
    fn test_performance_change_calculation() {
        let current = BaselineMetrics {
            avg_time_ms: 90.0,
            std_dev_ms: 5.0,
            min_time_ms: 85.0,
            max_time_ms: 95.0,
            memory_usage_mb: 100.0,
            cpu_utilization: 50.0,
            iterations: 10,
        };

        let baseline = BaselineMetrics {
            avg_time_ms: 100.0,
            std_dev_ms: 5.0,
            min_time_ms: 95.0,
            max_time_ms: 105.0,
            memory_usage_mb: 100.0,
            cpu_utilization: 50.0,
            iterations: 10,
        };

        let temp_dir = tempdir().unwrap();
        let config = BaselineComparisonConfig::default();
        let comparator = BaselineComparator::new(temp_dir.path(), config);

        let comparison = comparator.compare_benchmark_metrics("test", &current, &baseline).unwrap();
        assert!(comparison.performance_change_percent > 0.0); // Should be positive (improvement)
        assert!(comparison.is_improvement);
    }
}
