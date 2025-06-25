use crate::error::CursedError;
/// Statistics and performance measurement for CURSED testing framework
/// 
/// Provides comprehensive metrics collection, timing analysis, and
/// performance tracking for test execution.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use super::{TestError, TestFrameworkResult};

/// Test execution timing information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestTiming {
    /// Test name
    /// Execution start time
    #[serde(skip)]
    #[serde(default = "std::time::Instant::now")]
    /// Execution duration
    /// Setup time (if applicable)
    /// Teardown time (if applicable)
    /// Compilation time (if applicable)
impl TestTiming {
    /// Create a new test timing record
    pub fn new(test_name: String, start_time: Instant, duration: Duration) -> Self {
        Self {
        }
    }
    
    /// Add setup timing
    pub fn with_setup_time(mut self, setup_time: Duration) -> Self {
        self.setup_time = Some(setup_time);
        self
    /// Add teardown timing
    pub fn with_teardown_time(mut self, teardown_time: Duration) -> Self {
        self.teardown_time = Some(teardown_time);
        self
    /// Add compilation timing
    pub fn with_compilation_time(mut self, compilation_time: Duration) -> Self {
        self.compilation_time = Some(compilation_time);
        self
    /// Get total time including setup/teardown
    pub fn total_time(&self) -> Duration {
        self.duration + 
        self.setup_time.unwrap_or(Duration::from_secs(0)) +
        self.teardown_time.unwrap_or(Duration::from_secs(0)) +
        self.compilation_time.unwrap_or(Duration::from_secs(0))
    }
}

/// Performance statistics for tests
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceStats {
    /// Total number of measurements
    /// Minimum execution time
    /// Maximum execution time
    /// Average execution time
    /// Median execution time
    /// Standard deviation
    /// 95th percentile
    /// 99th percentile
impl PerformanceStats {
    /// Calculate performance statistics from timing data
    pub fn from_timings(timings: &[Duration]) -> Self {
        if timings.is_empty() {
            return Self::default();
        let mut sorted_timings = timings.to_vec();
        sorted_timings.sort();
        
        let sample_count = timings.len();
        let min_time = sorted_timings[0];
        let max_time = sorted_timings[sample_count - 1];
        
        let total_nanos: u128 = timings.iter().map(|d| d.as_nanos()).sum();
        let average_nanos = total_nanos / sample_count as u128;
        let average_time = Duration::from_nanos(average_nanos as u64);
        
        let median_time = if sample_count % 2 == 0 {
            let mid1 = sorted_timings[sample_count / 2 - 1];
            let mid2 = sorted_timings[sample_count / 2];
            Duration::from_nanos((mid1.as_nanos() + mid2.as_nanos()) as u64 / 2)
        } else {
            sorted_timings[sample_count / 2]
        
        // Calculate standard deviation
        let variance: f64 = timings.iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - average_nanos as f64;
                diff * diff
            })
            .sum::<f64>() / sample_count as f64;
        let std_deviation = Duration::from_nanos(variance.sqrt() as u64);
        
        // Calculate percentiles
        let p95_index = ((sample_count as f64 * 0.95) as usize).min(sample_count - 1);
        let p99_index = ((sample_count as f64 * 0.99) as usize).min(sample_count - 1);
        let p95_time = sorted_timings[p95_index];
        let p99_time = sorted_timings[p99_index];
        
        Self {
        }
    }
impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
        }
    }
/// Test execution metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestMetrics {
    /// Test name
    /// Number of times test was executed
    /// Performance statistics
    /// Success rate (0.0 to 1.0)
    /// Memory usage statistics
    /// Custom metrics
impl TestMetrics {
    /// Create new test metrics
    pub fn new(test_name: String) -> Self {
        Self {
        }
    }
    
    /// Add a custom metric
    pub fn add_custom_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
    /// Update performance statistics
    pub fn update_performance(&mut self, timings: &[Duration]) {
        self.performance = PerformanceStats::from_timings(timings);
    /// Update success rate
    pub fn update_success_rate(&mut self, successes: usize, total: usize) {
        self.success_rate = if total > 0 {
            successes as f64 / total as f64
        } else {
            0.0
        self.execution_count = total;
    }
}

/// Memory usage metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryMetrics {
    /// Peak memory usage in bytes
    /// Average memory usage in bytes
    /// Memory allocated during test
    /// Memory deallocated during test
    /// Memory leak amount (allocated - deallocated)
impl MemoryMetrics {
    /// Create new memory metrics
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Update memory metrics
    pub fn update(&mut self, peak: u64, allocated: u64, deallocated: u64) {
        self.peak_memory = peak;
        self.allocated_memory = allocated;
        self.deallocated_memory = deallocated;
        self.leaked_memory = allocated as i64 - deallocated as i64;
    }
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution statistics for the entire test run
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionStats {
    /// Total number of tests discovered
    /// Total number of tests executed
    /// Total number of tests passed
    /// Total number of tests failed
    /// Total number of tests ignored
    /// Total execution time
    /// Test discovery time
    /// Average test execution time
    /// Fastest test time
    /// Slowest test time
    /// Number of parallel threads used
    /// CPU utilization during test run
    /// Memory utilization during test run
impl ExecutionStats {
    /// Create new execution statistics
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Update test counts
    pub fn update_test_counts(&mut self, discovered: usize, executed: usize, passed: usize, failed: usize, ignored: usize) {
        self.tests_discovered = discovered;
        self.tests_executed = executed;
        self.tests_passed = passed;
        self.tests_failed = failed;
        self.tests_ignored = ignored;
    /// Update timing information
    pub fn update_timing(&mut self, total_time: Duration, discovery_time: Duration, test_times: &[Duration]) {
        self.total_execution_time = total_time;
        self.discovery_time = discovery_time;
        
        if !test_times.is_empty() {
            let total_test_time: Duration = test_times.iter().sum();
            self.average_test_time = total_test_time / test_times.len() as u32;
            self.fastest_test_time = *test_times.iter().min().unwrap();
            self.slowest_test_time = *test_times.iter().max().unwrap();
        }
    }
    
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.tests_executed > 0 {
            self.tests_passed as f64 / self.tests_executed as f64
        } else {
            0.0
        }
    }
    
    /// Get failure rate
    pub fn failure_rate(&self) -> f64 {
        if self.tests_executed > 0 {
            self.tests_failed as f64 / self.tests_executed as f64
        } else {
            0.0
        }
    }
impl Default for ExecutionStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Test benchmark information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestBenchmark {
    /// Test name
    /// Benchmark iterations
    /// Time per iteration
    /// Operations per second
    /// Benchmark confidence interval
    /// Benchmark metadata
impl TestBenchmark {
    /// Create a new benchmark result
    pub fn new(test_name: String, iterations: usize, total_time: Duration) -> Self {
        let time_per_iteration = total_time / iterations as u32;
        let operations_per_second = if time_per_iteration.as_secs_f64() > 0.0 {
            1.0 / time_per_iteration.as_secs_f64()
        } else {
            0.0
        
        Self {
        }
    }
    
    /// Add metadata to benchmark
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    /// Set confidence interval
    pub fn with_confidence_interval(mut self, lower: Duration, upper: Duration) -> Self {
        self.confidence_interval = Some((lower, upper));
        self
    }
}

/// Comprehensive test statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestStatistics {
    /// Individual test metrics
    /// Overall execution statistics
    /// Test benchmarks
    /// Test timing records
    /// Performance trends over time
    #[serde(skip)]
impl TestStatistics {
    /// Create new test statistics
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Record a test execution
    pub fn record_test_execution(&mut self, test_name: &str, duration: Duration, success: bool) {
        let timing = TestTiming::new(test_name.to_string(), Instant::now(), duration);
        self.timings.push(timing);
        
        // Update performance trends
        self.performance_trends
            .entry(test_name.to_string())
            .or_insert_with(Vec::new)
            .push((Instant::now(), duration));
        
        // Update test metrics
        let metrics = self.test_metrics
            .entry(test_name.to_string())
            .or_insert_with(|| TestMetrics::new(test_name.to_string()));
        
        metrics.execution_count += 1;
        
        // Update success rate
        let test_timings: Vec<Duration> = self.timings.iter()
            .filter(|t| t.test_name == test_name)
            .map(|t| t.duration)
            .collect();
        
        metrics.update_performance(&test_timings);
        
        // Calculate success rate (simplified)
        let successes = if success { 1 } else { 0 };
        metrics.update_success_rate(successes, 1);
    /// Add a benchmark result
    pub fn add_benchmark(&mut self, benchmark: TestBenchmark) {
        self.benchmarks.insert(benchmark.test_name.clone(), benchmark);
    /// Get metrics for a specific test
    pub fn get_test_metrics(&self, test_name: &str) -> Option<&TestMetrics> {
        self.test_metrics.get(test_name)
    /// Get overall execution statistics
    pub fn get_execution_stats(&self) -> &ExecutionStats {
        &self.execution_stats
    /// Get performance trend for a test
    pub fn get_performance_trend(&self, test_name: &str) -> Option<&Vec<(Instant, Duration)>> {
        self.performance_trends.get(test_name)
    /// Get summary statistics
    pub fn get_summary(&self) -> StatisticsSummary {
        let total_tests = self.test_metrics.len();
        let total_executions: usize = self.test_metrics.values()
            .map(|m| m.execution_count)
            .sum();
        
        let average_success_rate = if total_tests > 0 {
            self.test_metrics.values()
                .map(|m| m.success_rate)
                .sum::<f64>() / total_tests as f64
        } else {
            0.0
        
        let total_time: Duration = self.timings.iter()
            .map(|t| t.duration)
            .sum();
        
        let average_time = if total_executions > 0 {
            total_time / total_executions as u32
        } else {
            Duration::from_secs(0)
        
        StatisticsSummary {
        }
    }
    
    /// Clear all statistics
    pub fn clear(&mut self) {
        self.test_metrics.clear();
        self.benchmarks.clear();
        self.timings.clear();
        self.performance_trends.clear();
        self.execution_stats = ExecutionStats::new();
    /// Export statistics to JSON
    pub fn to_json(&self) -> TestFrameworkResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| TestError::ReportError(format!("Failed to serialize statistics: {}", e)).into())
    /// Import statistics from JSON
    pub fn from_json(json: &str) -> TestFrameworkResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| TestError::ReportError(format!("Failed to deserialize statistics: {}", e)).into())
    }
}

impl Default for TestStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of test statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatisticsSummary {
    /// Total number of unique tests
    /// Total number of test executions
    /// Average success rate across all tests
    /// Total execution time for all tests
    /// Average execution time per test
    /// Number of benchmarks recorded
impl StatisticsSummary {
    /// Format summary as string
    pub fn format(&self) -> String {
        format!(
            self.benchmarks_count
        )
    }
}
