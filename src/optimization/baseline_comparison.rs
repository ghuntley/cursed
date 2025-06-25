// Baseline comparison system for performance regression detection

use crate::error::{CursedError, Result};
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
    /// Version identifier for the baseline
    /// Benchmark results that form the baseline
    /// Metadata about the baseline
/// Baseline metadata for context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetadata {
    /// Commit hash or version when baseline was created
    /// Environment information
    /// Compiler configuration used
    /// Additional notes
/// Environment information for baseline context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    /// Operating system
    /// CPU architecture
    /// Number of CPU cores
    /// Available memory in MB
/// Baseline metrics for a specific benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineMetrics {
    /// Average execution time
    /// Standard deviation of execution time
    /// Minimum execution time observed
    /// Maximum execution time observed
    /// Memory usage metrics
    /// CPU utilization percentage
    /// Number of iterations in the baseline
/// Result of baseline comparison
#[derive(Debug, Clone)]
pub struct BaselineComparisonResult {
    /// Whether any regressions were detected
    /// Whether any improvements were detected
    /// Detailed comparison for each benchmark
    /// Overall performance change percentage
    /// Summary of findings
/// Comparison result for a single benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkComparison {
    /// Name of the benchmark
    /// Current performance metrics
    /// Baseline performance metrics
    /// Performance change (positive = improvement, negative = regression)
    /// Whether this is considered a regression
    /// Whether this is considered an improvement
    /// Confidence level of the comparison
    /// Detailed analysis
/// Configuration for baseline comparison
#[derive(Debug, Clone)]
pub struct BaselineComparisonConfig {
    /// Threshold for considering a change a regression (percentage)
    /// Threshold for considering a change an improvement (percentage)
    /// Minimum confidence level required for reporting changes
    /// Maximum age of baseline data to consider valid (in days)
    /// Whether to include statistical significance testing
impl Default for BaselineComparisonConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Baseline comparison engine
pub struct BaselineComparator {
impl BaselineComparator {
    /// Create a new baseline comparator
    pub fn new<P: AsRef<Path>>(storage_path: P, config: BaselineComparisonConfig) -> Self {
        Self {
        }
    }

    /// Create a new baseline from benchmark results
    #[instrument(skip(self, benchmark_results))]
    pub fn create_baseline(
    ) -> Result<BaselineData> {
        info!("Creating new baseline with version: {}", version);

        let mut baseline_metrics = HashMap::new();

        // Convert benchmark results to baseline metrics
        for result in &benchmark_results.results {
            let metrics = self.convert_to_baseline_metrics(result)?;
            baseline_metrics.insert(result.benchmark_name.clone(), metrics);
        let baseline = BaselineData {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()

        // Save baseline to storage
        self.save_baseline(&baseline)?;

        info!("Baseline created successfully with {} benchmarks", baseline_metrics.len());
        Ok(baseline)
    /// Load the most recent baseline
    #[instrument(skip(self))]
    pub fn load_latest_baseline(&self) -> Result<Option<BaselineData>> {
        debug!("Loading latest baseline from storage");

        let baseline_file = self.baseline_storage_path.join("latest_baseline.json");
        if !baseline_file.exists() {
            debug!("No baseline file found");
            return Ok(None);
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
                  baseline_age_seconds / (24 * 3600), 
                  self.config.max_baseline_age_days);
            return Ok(None);
        info!("Loaded baseline with version: {}", baseline.version);
        Ok(Some(baseline))
    /// Compare current results against baseline
    #[instrument(skip(self, current_results, baseline))]
    pub fn compare_against_baseline(
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
                )?;

                if comparison.is_regression {
                    has_regressions = true;
                }
                if comparison.is_improvement {
                    has_improvements = true;
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

        let summary = self.generate_comparison_summary(
        );

        let result = BaselineComparisonResult {

              overall_change_percent, has_regressions, has_improvements);

        Ok(result)
    /// Convert benchmark result to baseline metrics
    fn convert_to_baseline_metrics(&self, result: &BenchmarkResult) -> Result<BaselineMetrics> {
        // Calculate statistics from benchmark result
        let times: Vec<f64> = result.execution_times.iter().map(|d| d.as_secs_f64() * 1000.0).collect();
        
        if times.is_empty() {
            return Err(CursedError::optimization_error("No execution times available for baseline metrics"));
        let avg_time_ms = times.iter().sum::<f64>() / times.len() as f64;
        let min_time_ms = times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_time_ms = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Calculate standard deviation
        let variance = times.iter()
            .map(|&x| (x - avg_time_ms).powi(2))
            .sum::<f64>() / times.len() as f64;
        let std_dev_ms = variance.sqrt();

        Ok(BaselineMetrics {
        })
    /// Compare metrics between current and baseline
    fn compare_benchmark_metrics(
    ) -> Result<BenchmarkComparison> {
        // Calculate performance change percentage (negative = regression)
        let performance_change_percent = if baseline.avg_time_ms > 0.0 {
            ((baseline.avg_time_ms - current.avg_time_ms) / baseline.avg_time_ms) * 100.0
        } else {
            0.0

        // Determine if this is a regression or improvement
        let is_regression = performance_change_percent < -self.config.regression_threshold_percent;
        let is_improvement = performance_change_percent > self.config.improvement_threshold_percent;

        // Calculate confidence level using statistical significance
        let confidence_level = if self.config.use_statistical_testing {
            self.calculate_statistical_confidence(current, baseline)
        } else {
            1.0 // Assume high confidence if not using statistical testing

        // Generate detailed analysis
        let analysis = self.generate_benchmark_analysis(
        );

        Ok(BenchmarkComparison {
        })
    /// Calculate statistical confidence using t-test approximation
    fn calculate_statistical_confidence(&self, current: &BaselineMetrics, baseline: &BaselineMetrics) -> f64 {
        // Simplified t-test calculation for confidence
        let n1 = current.iterations as f64;
        let n2 = baseline.iterations as f64;
        
        if n1 < 2.0 || n2 < 2.0 {
            return 0.5; // Low confidence with insufficient data
        let mean_diff = (current.avg_time_ms - baseline.avg_time_ms).abs();
        let pooled_std = ((current.std_dev_ms.powi(2) + baseline.std_dev_ms.powi(2)) / 2.0).sqrt();
        
        if pooled_std == 0.0 {
            return 1.0; // Perfect confidence if no variation
        let standard_error = pooled_std * ((1.0 / n1) + (1.0 / n2)).sqrt();
        let t_stat = mean_diff / standard_error;
        
        // Simplified confidence calculation (approximation)
        let confidence = (t_stat / (t_stat + 2.0)).min(0.99).max(0.01);
        confidence
    /// Generate analysis text for a benchmark comparison
    fn generate_benchmark_analysis(
    ) -> String {
        let mut analysis = Vec::new();

        analysis.push(format!("Benchmark: {}", benchmark_name));
                             current.avg_time_ms, current.std_dev_ms));
                             baseline.avg_time_ms, baseline.std_dev_ms));
        analysis.push(format!("Performance change: {:.2}%", change_percent));
        analysis.push(format!("Confidence level: {:.1}%", confidence * 100.0));

        if change_percent < -self.config.regression_threshold_percent {
            analysis.push("⚠️  REGRESSION DETECTED: Performance has degraded significantly".to_string());
        } else if change_percent > self.config.improvement_threshold_percent {
            analysis.push("✅ IMPROVEMENT: Performance has improved significantly".to_string());
        } else {
            analysis.push("➡️  STABLE: Performance is within acceptable variance".to_string());
        // Memory usage analysis
        let memory_change = ((current.memory_usage_mb - baseline.memory_usage_mb) / baseline.memory_usage_mb.max(1.0)) * 100.0;
        if memory_change.abs() > 10.0 {
            analysis.push(format!("Memory usage change: {:.1}%", memory_change));
        analysis.join("\n")
    /// Generate overall comparison summary
    fn generate_comparison_summary(
    ) -> String {
        let mut summary = Vec::new();
        
        summary.push(format!("=== Baseline Comparison Summary ==="));
        summary.push(format!("Overall performance change: {:.2}%", overall_change));
        summary.push(format!("Benchmarks analyzed: {}", comparisons.len()));

        let regression_count = comparisons.values().filter(|c| c.is_regression).count();
        let improvement_count = comparisons.values().filter(|c| c.is_improvement).count();
        let stable_count = comparisons.len() - regression_count - improvement_count;

                           regression_count, improvement_count, stable_count));

        if has_regressions {
            summary.push("⚠️  WARNING: Performance regressions detected!".to_string());
            for (name, comparison) in comparisons {
                if comparison.is_regression {
                    summary.push(format!("  - {}: {:.2}% slower", name, -comparison.performance_change_percent));
                }
            }
        if has_improvements {
            summary.push("✅ Good news: Performance improvements detected!".to_string());
            for (name, comparison) in comparisons {
                if comparison.is_improvement {
                    summary.push(format!("  + {}: {:.2}% faster", name, comparison.performance_change_percent));
                }
            }
        if !has_regressions && !has_improvements {
            summary.push("➡️  Performance is stable within acceptable variance".to_string());
        summary.join("\n")
    /// Save baseline to storage
    fn save_baseline(&self, baseline: &BaselineData) -> Result<()> {
        // Ensure storage directory exists
        if let Some(parent) = self.baseline_storage_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CursedError::optimization_error(&format!("Failed to create baseline directory: {}", e)))?;
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
    /// Get current environment information
    pub fn get_current_environment() -> EnvironmentInfo {
        EnvironmentInfo {
        }
    }

    /// Get system memory in MB (best effort)
    fn get_system_memory_mb() -> u64 {
        // This is a simplified implementation
        // In a real implementation, you'd use system APIs
        8192 // Default to 8GB if we can't detect
    }
}

