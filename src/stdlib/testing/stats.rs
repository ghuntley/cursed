/// Statistics and performance measurement for CURSED testing framework
/// 
/// Provides comprehensive metrics collection, timing analysis, and
/// performance tracking for test execution.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::crate::stdlib::errors_simple::CursedError;
use super::{TestError, TestFrameworkResult};

/// Test execution timing information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestTiming {
    /// Test name
    pub test_name: String,
    /// Execution start time
    #[serde(skip)]
    #[serde(default = "std::time::Instant::now")]
    pub start_time: Instant,
    /// Execution duration
    pub duration: Duration,
    /// Setup time (if applicable)
    pub setup_time: Option<Duration>,
    /// Teardown time (if applicable)
    pub teardown_time: Option<Duration>,
    /// Compilation time (if applicable)
    pub compilation_time: Option<Duration>,
}

impl TestTiming {
    /// Create a new test timing record
    pub fn new(test_name: String, start_time: Instant, duration: Duration) -> Self {
        Self {
            test_name,
            start_time,
            duration,
            setup_time: None,
            teardown_time: None,
            compilation_time: None,
        }
    }
    
    /// Add setup timing
    pub fn with_setup_time(mut self, setup_time: Duration) -> Self {
        self.setup_time = Some(setup_time);
        self
    }
    
    /// Add teardown timing
    pub fn with_teardown_time(mut self, teardown_time: Duration) -> Self {
        self.teardown_time = Some(teardown_time);
        self
    }
    
    /// Add compilation timing
    pub fn with_compilation_time(mut self, compilation_time: Duration) -> Self {
        self.compilation_time = Some(compilation_time);
        self
    }
    
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
    pub sample_count: usize,
    /// Minimum execution time
    pub min_time: Duration,
    /// Maximum execution time
    pub max_time: Duration,
    /// Average execution time
    pub average_time: Duration,
    /// Median execution time
    pub median_time: Duration,
    /// Standard deviation
    pub std_deviation: Duration,
    /// 95th percentile
    pub p95_time: Duration,
    /// 99th percentile
    pub p99_time: Duration,
}

impl PerformanceStats {
    /// Calculate performance statistics from timing data
    pub fn from_timings(timings: &[Duration]) -> Self {
        if timings.is_empty() {
            return Self::default();
        }
        
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
        };
        
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
            sample_count,
            min_time,
            max_time,
            average_time,
            median_time,
            std_deviation,
            p95_time,
            p99_time,
        }
    }
}

impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
            sample_count: 0,
            min_time: Duration::from_secs(0),
            max_time: Duration::from_secs(0),
            average_time: Duration::from_secs(0),
            median_time: Duration::from_secs(0),
            std_deviation: Duration::from_secs(0),
            p95_time: Duration::from_secs(0),
            p99_time: Duration::from_secs(0),
        }
    }
}

/// Test execution metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestMetrics {
    /// Test name
    pub test_name: String,
    /// Number of times test was executed
    pub execution_count: usize,
    /// Performance statistics
    pub performance: PerformanceStats,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Memory usage statistics
    pub memory_stats: Option<MemoryMetrics>,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

impl TestMetrics {
    /// Create new test metrics
    pub fn new(test_name: String) -> Self {
        Self {
            test_name,
            execution_count: 0,
            performance: PerformanceStats::default(),
            success_rate: 0.0,
            memory_stats: None,
            custom_metrics: HashMap::new(),
        }
    }
    
    /// Add a custom metric
    pub fn add_custom_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
    }
    
    /// Update performance statistics
    pub fn update_performance(&mut self, timings: &[Duration]) {
        self.performance = PerformanceStats::from_timings(timings);
    }
    
    /// Update success rate
    pub fn update_success_rate(&mut self, successes: usize, total: usize) {
        self.success_rate = if total > 0 {
            successes as f64 / total as f64
        } else {
            0.0
        };
        self.execution_count = total;
    }
}

/// Memory usage metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryMetrics {
    /// Peak memory usage in bytes
    pub peak_memory: u64,
    /// Average memory usage in bytes
    pub average_memory: u64,
    /// Memory allocated during test
    pub allocated_memory: u64,
    /// Memory deallocated during test
    pub deallocated_memory: u64,
    /// Memory leak amount (allocated - deallocated)
    pub leaked_memory: i64,
}

impl MemoryMetrics {
    /// Create new memory metrics
    pub fn new() -> Self {
        Self {
            peak_memory: 0,
            average_memory: 0,
            allocated_memory: 0,
            deallocated_memory: 0,
            leaked_memory: 0,
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
    pub tests_discovered: usize,
    /// Total number of tests executed
    pub tests_executed: usize,
    /// Total number of tests passed
    pub tests_passed: usize,
    /// Total number of tests failed
    pub tests_failed: usize,
    /// Total number of tests ignored
    pub tests_ignored: usize,
    /// Total execution time
    pub total_execution_time: Duration,
    /// Test discovery time
    pub discovery_time: Duration,
    /// Average test execution time
    pub average_test_time: Duration,
    /// Fastest test time
    pub fastest_test_time: Duration,
    /// Slowest test time
    pub slowest_test_time: Duration,
    /// Number of parallel threads used
    pub parallel_threads: usize,
    /// CPU utilization during test run
    pub cpu_utilization: Option<f64>,
    /// Memory utilization during test run
    pub memory_utilization: Option<MemoryMetrics>,
}

impl ExecutionStats {
    /// Create new execution statistics
    pub fn new() -> Self {
        Self {
            tests_discovered: 0,
            tests_executed: 0,
            tests_passed: 0,
            tests_failed: 0,
            tests_ignored: 0,
            total_execution_time: Duration::from_secs(0),
            discovery_time: Duration::from_secs(0),
            average_test_time: Duration::from_secs(0),
            fastest_test_time: Duration::from_secs(0),
            slowest_test_time: Duration::from_secs(0),
            parallel_threads: 1,
            cpu_utilization: None,
            memory_utilization: None,
        }
    }
    
    /// Update test counts
    pub fn update_test_counts(&mut self, discovered: usize, executed: usize, passed: usize, failed: usize, ignored: usize) {
        self.tests_discovered = discovered;
        self.tests_executed = executed;
        self.tests_passed = passed;
        self.tests_failed = failed;
        self.tests_ignored = ignored;
    }
    
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
    pub test_name: String,
    /// Benchmark iterations
    pub iterations: usize,
    /// Time per iteration
    pub time_per_iteration: Duration,
    /// Operations per second
    pub operations_per_second: f64,
    /// Benchmark confidence interval
    pub confidence_interval: Option<(Duration, Duration)>,
    /// Benchmark metadata
    pub metadata: HashMap<String, String>,
}

impl TestBenchmark {
    /// Create a new benchmark result
    pub fn new(test_name: String, iterations: usize, total_time: Duration) -> Self {
        let time_per_iteration = total_time / iterations as u32;
        let operations_per_second = if time_per_iteration.as_secs_f64() > 0.0 {
            1.0 / time_per_iteration.as_secs_f64()
        } else {
            0.0
        };
        
        Self {
            test_name,
            iterations,
            time_per_iteration,
            operations_per_second,
            confidence_interval: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to benchmark
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
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
    pub test_metrics: HashMap<String, TestMetrics>,
    /// Overall execution statistics
    pub execution_stats: ExecutionStats,
    /// Test benchmarks
    pub benchmarks: HashMap<String, TestBenchmark>,
    /// Test timing records
    pub timings: Vec<TestTiming>,
    /// Performance trends over time
    #[serde(skip)]
    pub performance_trends: HashMap<String, Vec<(Instant, Duration)>>,
}

impl TestStatistics {
    /// Create new test statistics
    pub fn new() -> Self {
        Self {
            test_metrics: HashMap::new(),
            execution_stats: ExecutionStats::new(),
            benchmarks: HashMap::new(),
            timings: Vec::new(),
            performance_trends: HashMap::new(),
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
    }
    
    /// Add a benchmark result
    pub fn add_benchmark(&mut self, benchmark: TestBenchmark) {
        self.benchmarks.insert(benchmark.test_name.clone(), benchmark);
    }
    
    /// Get metrics for a specific test
    pub fn get_test_metrics(&self, test_name: &str) -> Option<&TestMetrics> {
        self.test_metrics.get(test_name)
    }
    
    /// Get overall execution statistics
    pub fn get_execution_stats(&self) -> &ExecutionStats {
        &self.execution_stats
    }
    
    /// Get performance trend for a test
    pub fn get_performance_trend(&self, test_name: &str) -> Option<&Vec<(Instant, Duration)>> {
        self.performance_trends.get(test_name)
    }
    
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
        };
        
        let total_time: Duration = self.timings.iter()
            .map(|t| t.duration)
            .sum();
        
        let average_time = if total_executions > 0 {
            total_time / total_executions as u32
        } else {
            Duration::from_secs(0)
        };
        
        StatisticsSummary {
            total_tests,
            total_executions,
            average_success_rate,
            total_execution_time: total_time,
            average_execution_time: average_time,
            benchmarks_count: self.benchmarks.len(),
        }
    }
    
    /// Clear all statistics
    pub fn clear(&mut self) {
        self.test_metrics.clear();
        self.benchmarks.clear();
        self.timings.clear();
        self.performance_trends.clear();
        self.execution_stats = ExecutionStats::new();
    }
    
    /// Export statistics to JSON
    pub fn to_json(&self) -> TestFrameworkResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| TestError::ReportError(format!("Failed to serialize statistics: {}", e)).into())
    }
    
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
    pub total_tests: usize,
    /// Total number of test executions
    pub total_executions: usize,
    /// Average success rate across all tests
    pub average_success_rate: f64,
    /// Total execution time for all tests
    pub total_execution_time: Duration,
    /// Average execution time per test
    pub average_execution_time: Duration,
    /// Number of benchmarks recorded
    pub benchmarks_count: usize,
}

impl StatisticsSummary {
    /// Format summary as string
    pub fn format(&self) -> String {
        format!(
            "Summary: {} tests, {} executions, {:.1}% success rate, {:.3}s total time, {} benchmarks",
            self.total_tests,
            self.total_executions,
            self.average_success_rate * 100.0,
            self.total_execution_time.as_secs_f64(),
            self.benchmarks_count
        )
    }
}
