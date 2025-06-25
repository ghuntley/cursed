/// Benchmark framework for performance testing
// use crate::stdlib::profiler::error::{ProfilerError, ProfilerResult, benchmark_error};
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Global benchmark counter
static BENCHMARK_COUNT: AtomicU64 = AtomicU64::new(0);

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Statistical analysis of benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkStatistics {
impl BenchmarkStatistics {
    /// Calculate statistics from a set of measurements
    pub fn from_measurements(mut measurements: Vec<Duration>) -> Self {
        if measurements.is_empty() {
            return Self::default();
        measurements.sort();
        let len = measurements.len();

        // Basic statistics
        let min = measurements[0];
        let max = measurements[len - 1];
        let median = measurements[len / 2];

        // Mean
        let total_ns: u128 = measurements.iter().map(|d| d.as_nanos()).sum();
        let mean_ns = total_ns / len as u128;
        let mean = Duration::from_nanos(mean_ns as u64);

        // Standard deviation
        let variance_sum: f64 = measurements
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - mean_ns as f64;
                diff * diff
            })
            .sum();
        let variance = variance_sum / len as f64;
        let std_dev = Duration::from_nanos(variance.sqrt() as u64);

        // Percentiles
        let p95_index = ((len as f64) * 0.95) as usize;
        let p99_index = ((len as f64) * 0.99) as usize;
        let percentile_95 = measurements[p95_index.min(len - 1)];
        let percentile_99 = measurements[p99_index.min(len - 1)];

        // Coefficient of variation
        let coefficient_of_variation = if mean_ns > 0 {
            (std_dev.as_nanos() as f64) / (mean_ns as f64)
        } else {
            0.0

        // Outlier detection (using IQR method)
        let q1_index = len / 4;
        let q3_index = (3 * len) / 4;
        let q1 = measurements[q1_index];
        let q3 = measurements[q3_index];
        let iqr = q3.saturating_sub(q1);
        let lower_bound = q1.saturating_sub(Duration::from_nanos((iqr.as_nanos() as f64 * 1.5) as u64));
        let upper_bound = q3 + Duration::from_nanos((iqr.as_nanos() as f64 * 1.5) as u64);

        let outlier_count = measurements
            .iter()
            .filter(|&&m| m < lower_bound || m > upper_bound)
            .count();

        Self {
        }
    }
impl Default for BenchmarkStatistics {
    fn default() -> Self {
        Self {
        }
    }
/// Memory usage during benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkMemoryUsage {
/// CPU usage during benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkCpuUsage {
/// Complete benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
impl BenchmarkResult {
    /// Get throughput (operations per second)
    pub fn throughput(&self) -> f64 {
        if self.statistics.mean.as_nanos() > 0 {
            1_000_000_000.0 / (self.statistics.mean.as_nanos() as f64)
        } else {
            0.0
        }
    }

    /// Check if benchmark is stable (low variance)
    pub fn is_stable(&self, max_cv: f64) -> bool {
        self.statistics.coefficient_of_variation <= max_cv
    /// Get performance classification
    pub fn performance_class(&self) -> BenchmarkPerformanceClass {
        if self.statistics.mean < Duration::from_micros(1) {
            BenchmarkPerformanceClass::Excellent
        } else if self.statistics.mean < Duration::from_micros(10) {
            BenchmarkPerformanceClass::VeryGood
        } else if self.statistics.mean < Duration::from_micros(100) {
            BenchmarkPerformanceClass::Good
        } else if self.statistics.mean < Duration::from_millis(1) {
            BenchmarkPerformanceClass::Fair
        } else {
            BenchmarkPerformanceClass::Poor
        }
    }
/// Performance classification
#[derive(Debug, Clone, PartialEq)]
pub enum BenchmarkPerformanceClass {
/// Benchmark suite containing multiple benchmarks
#[derive(Debug)]
pub struct BenchmarkSuite {
impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(name: &str, config: BenchmarkConfig) -> Self {
        Self {
        }
    }

    /// Add a benchmark to the suite
    pub fn add_benchmark<F>(&mut self, name: &str, benchmark: F) 
    where 
        F: Fn() -> ProfilerResult<()> + Send + Sync + 'static 
    {
        self.benchmark_names.push(name.to_string());
        self.benchmarks.push(Box::new(benchmark));
    /// Run all benchmarks in the suite
    pub fn run(&mut self) -> ProfilerResult<BenchmarkSuiteResult> {
        self.results.clear();
        let start_time = Instant::now();

        for (i, benchmark) in self.benchmarks.iter().enumerate() {
            let name = &self.benchmark_names[i];
            let result = run_benchmark_with_config(name, benchmark, &self.config)?;
            self.results.push(result);
        let total_time = start_time.elapsed();
        
        Ok(BenchmarkSuiteResult {
        })
    }
}

/// Result of running a benchmark suite
#[derive(Debug, Clone)]
pub struct BenchmarkSuiteResult {
impl BenchmarkSuiteResult {
    /// Get the fastest benchmark
    pub fn fastest(&self) -> Option<&BenchmarkResult> {
        self.results.iter().min_by_key(|r| r.statistics.mean)
    /// Get the slowest benchmark
    pub fn slowest(&self) -> Option<&BenchmarkResult> {
        self.results.iter().max_by_key(|r| r.statistics.mean)
    /// Get average throughput across all benchmarks
    pub fn average_throughput(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        let total_throughput: f64 = self.results.iter().map(|r| r.throughput()).sum();
        total_throughput / self.results.len() as f64
    }
}

/// Benchmark runner for executing individual benchmarks
pub struct BenchmarkRunner {
impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// Run a single benchmark
    pub fn run_benchmark<F>(&self, name: &str, benchmark: F) -> ProfilerResult<BenchmarkResult>
    where
        F: Fn() -> ProfilerResult<()>
    {
        run_benchmark_with_config(name, &benchmark, &self.config)
    }
}

/// Benchmark implementation
pub struct Benchmark<F>
where
    F: Fn() -> ProfilerResult<()>
{
impl<F> Benchmark<F>
where
    F: Fn() -> ProfilerResult<()>
{
    /// Create a new benchmark
    pub fn new(name: &str, function: F, config: BenchmarkConfig) -> Self {
        Self {
        }
    }

    /// Run the benchmark
    pub fn run(&self) -> ProfilerResult<BenchmarkResult> {
        run_benchmark_with_config(&self.name, &self.function, &self.config)
    }
}

/// Comparison result between two benchmarks
#[derive(Debug, Clone)]
pub struct ComparisonResult {
/// Benchmark report generator
pub struct BenchmarkReport {
impl BenchmarkReport {
    /// Create a new benchmark report
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add a benchmark result
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    /// Add a comparison
    pub fn add_comparison(&mut self, comparison: ComparisonResult) {
        self.comparisons.push(comparison);
    /// Generate a summary report
    pub fn generate_summary(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Benchmark Report\n\n");
        
        if !self.results.is_empty() {
            report.push_str("## Results\n\n");
            for result in &self.results {
                report.push_str(&format!(
                    "### {}\n\
                     - Mean: {:?}\n\
                     - Median: {:?}\n\
                     - Min: {:?}\n\
                     - Max: {:?}\n\
                     - Std Dev: {:?}\n\
                     - Throughput: {:.2} ops/sec\n\
                    result.iterations_completed
                ));
            }
        }

        if !self.comparisons.is_empty() {
            report.push_str("## Comparisons\n\n");
            for comparison in &self.comparisons {
                let status = if comparison.is_faster { "faster" } else { "slower" };
                report.push_str(&format!(
                    comparison.baseline_name
                ));
            }
        }

        report
    }
}

/// Run a benchmark with default configuration
pub fn benchmark_function<F>(name: &str, function: F) -> ProfilerResult<BenchmarkResult>
where
    F: Fn() -> ProfilerResult<()>
{
    benchmark_with_setup(name, || Ok(()), function, || Ok(()))
/// Run a benchmark with setup and teardown
pub fn benchmark_with_setup<S, F, T>(
) -> ProfilerResult<BenchmarkResult>
where
{
    let config = BenchmarkConfig::default();
    
    // Setup
    setup().map_err(|e| benchmark_error(&format!("Setup failed: {}", e)))?;
    
    // Run benchmark
    let result = run_benchmark_with_config(name, &function, &config)?;
    
    // Teardown
    teardown().map_err(|e| benchmark_error(&format!("Teardown failed: {}", e)))?;
    
    Ok(result)
/// Run a benchmark suite
pub fn run_benchmark_suite(name: &str, mut suite: BenchmarkSuite) -> ProfilerResult<BenchmarkSuiteResult> {
    suite.run()
/// Compare two benchmark results
pub fn compare_benchmarks(
) -> ComparisonResult {
    let baseline_mean_ns = baseline.statistics.mean.as_nanos() as f64;
    let comparison_mean_ns = comparison.statistics.mean.as_nanos() as f64;
    
    let speedup_factor = if comparison_mean_ns > 0.0 {
        baseline_mean_ns / comparison_mean_ns
    } else {
        f64::INFINITY
    
    let is_faster = speedup_factor > 1.0;
    
    // Simple statistical significance test (would be more sophisticated in real implementation)
    let baseline_cv = baseline.statistics.coefficient_of_variation;
    let comparison_cv = comparison.statistics.coefficient_of_variation;
    let is_statistically_significant = (speedup_factor - 1.0).abs() > (baseline_cv + comparison_cv);
    
    ComparisonResult {
    }
}

/// Generate a benchmark report
pub fn generate_benchmark_report(results: Vec<BenchmarkResult>) -> BenchmarkReport {
    let mut report = BenchmarkReport::new();
    
    for result in results {
        report.add_result(result);
    // Add metadata
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string());
    
    report
/// Internal function to run benchmark with configuration
fn run_benchmark_with_config<F>(
) -> ProfilerResult<BenchmarkResult>
where
    F: Fn() -> ProfilerResult<()>
{
    let start_time = Instant::now();
    let mut measurements = Vec::with_capacity(config.iterations);
    
    // Warmup phase
    let warmup_start = Instant::now();
    for _ in 0..config.warmup_iterations {
        benchmark().map_err(|e| benchmark_error(&format!("Warmup failed: {}", e)))?;
    }
    let warmup_time = warmup_start.elapsed();
    
    // Main benchmark phase
    let mut completed_iterations = 0;
    
    for i in 0..config.iterations {
        // Check time limits
        if start_time.elapsed() > config.max_duration {
            break;
        let iteration_start = Instant::now();
        benchmark().map_err(|e| benchmark_error(&format!("Iteration {} failed: {}", i, e)))?;
        let iteration_time = iteration_start.elapsed();
        
        measurements.push(iteration_time);
        completed_iterations += 1;
        
        // Check minimum duration
        if i >= config.sample_size && start_time.elapsed() >= config.min_duration {
            break;
        }
    }
    
    if measurements.is_empty() {
        return Err(benchmark_error("No measurements collected"));
    let total_time = start_time.elapsed();
    let statistics = BenchmarkStatistics::from_measurements(measurements.clone());
    
    // Memory and CPU usage would be measured here in real implementation
    let memory_usage = if config.measure_memory {
        Some(BenchmarkMemoryUsage {
        })
    } else {
        None
    
    let cpu_usage = if config.measure_cpu {
        Some(BenchmarkCpuUsage {
        })
    } else {
        None
    
    let mut metadata = HashMap::new();
    metadata.insert("rust_version".to_string(), option_env!("RUSTC_VERSION").unwrap_or("unknown").to_string());
    metadata.insert("target".to_string(), std::env::consts::ARCH.to_string());
    
    BENCHMARK_COUNT.fetch_add(1, Ordering::Relaxed);
    
    Ok(BenchmarkResult {
    })
/// Get number of benchmarks run
pub fn get_benchmark_count() -> u64 {
    BENCHMARK_COUNT.load(Ordering::Relaxed)
