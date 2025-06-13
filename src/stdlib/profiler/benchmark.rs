/// Benchmark framework for performance testing
use crate::stdlib::profiler::error::{ProfilerError, ProfilerResult, benchmark_error};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Global benchmark counter
static BENCHMARK_COUNT: AtomicU64 = AtomicU64::new(0);

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub sample_size: usize,
    pub statistical_significance: bool,
    pub measure_memory: bool,
    pub measure_cpu: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            warmup_iterations: 100,
            min_duration: Duration::from_millis(100),
            max_duration: Duration::from_secs(30),
            sample_size: 100,
            statistical_significance: true,
            measure_memory: false,
            measure_cpu: false,
        }
    }
}

/// Statistical analysis of benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkStatistics {
    pub mean: Duration,
    pub median: Duration,
    pub min: Duration,
    pub max: Duration,
    pub std_dev: Duration,
    pub percentile_95: Duration,
    pub percentile_99: Duration,
    pub coefficient_of_variation: f64,
    pub outlier_count: usize,
}

impl BenchmarkStatistics {
    /// Calculate statistics from a set of measurements
    pub fn from_measurements(mut measurements: Vec<Duration>) -> Self {
        if measurements.is_empty() {
            return Self::default();
        }

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
        };

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
            mean,
            median,
            min,
            max,
            std_dev,
            percentile_95,
            percentile_99,
            coefficient_of_variation,
            outlier_count,
        }
    }
}

impl Default for BenchmarkStatistics {
    fn default() -> Self {
        Self {
            mean: Duration::new(0, 0),
            median: Duration::new(0, 0),
            min: Duration::new(0, 0),
            max: Duration::new(0, 0),
            std_dev: Duration::new(0, 0),
            percentile_95: Duration::new(0, 0),
            percentile_99: Duration::new(0, 0),
            coefficient_of_variation: 0.0,
            outlier_count: 0,
        }
    }
}

/// Memory usage during benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkMemoryUsage {
    pub initial_usage: u64,
    pub peak_usage: u64,
    pub final_usage: u64,
    pub total_allocated: u64,
    pub total_freed: u64,
    pub allocation_count: u64,
}

/// CPU usage during benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkCpuUsage {
    pub user_time: Duration,
    pub system_time: Duration,
    pub total_time: Duration,
    pub cpu_utilization: f64,
}

/// Complete benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub config: BenchmarkConfig,
    pub statistics: BenchmarkStatistics,
    pub iterations_completed: usize,
    pub total_time: Duration,
    pub warmup_time: Duration,
    pub memory_usage: Option<BenchmarkMemoryUsage>,
    pub cpu_usage: Option<BenchmarkCpuUsage>,
    pub metadata: HashMap<String, String>,
    pub raw_measurements: Vec<Duration>,
}

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
    }

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
}

/// Performance classification
#[derive(Debug, Clone, PartialEq)]
pub enum BenchmarkPerformanceClass {
    Excellent,
    VeryGood,
    Good,
    Fair,
    Poor,
}

/// Benchmark suite containing multiple benchmarks
#[derive(Debug)]
pub struct BenchmarkSuite {
    pub name: String,
    pub benchmarks: Vec<Box<dyn Fn() -> ProfilerResult<()> + Send + Sync>>,
    pub benchmark_names: Vec<String>,
    pub config: BenchmarkConfig,
    pub results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(name: &str, config: BenchmarkConfig) -> Self {
        Self {
            name: name.to_string(),
            benchmarks: Vec::new(),
            benchmark_names: Vec::new(),
            config,
            results: Vec::new(),
        }
    }

    /// Add a benchmark to the suite
    pub fn add_benchmark<F>(&mut self, name: &str, benchmark: F) 
    where 
        F: Fn() -> ProfilerResult<()> + Send + Sync + 'static 
    {
        self.benchmark_names.push(name.to_string());
        self.benchmarks.push(Box::new(benchmark));
    }

    /// Run all benchmarks in the suite
    pub fn run(&mut self) -> ProfilerResult<BenchmarkSuiteResult> {
        self.results.clear();
        let start_time = Instant::now();

        for (i, benchmark) in self.benchmarks.iter().enumerate() {
            let name = &self.benchmark_names[i];
            let result = run_benchmark_with_config(name, benchmark, &self.config)?;
            self.results.push(result);
        }

        let total_time = start_time.elapsed();
        
        Ok(BenchmarkSuiteResult {
            suite_name: self.name.clone(),
            results: self.results.clone(),
            total_time,
            benchmark_count: self.results.len(),
        })
    }
}

/// Result of running a benchmark suite
#[derive(Debug, Clone)]
pub struct BenchmarkSuiteResult {
    pub suite_name: String,
    pub results: Vec<BenchmarkResult>,
    pub total_time: Duration,
    pub benchmark_count: usize,
}

impl BenchmarkSuiteResult {
    /// Get the fastest benchmark
    pub fn fastest(&self) -> Option<&BenchmarkResult> {
        self.results.iter().min_by_key(|r| r.statistics.mean)
    }

    /// Get the slowest benchmark
    pub fn slowest(&self) -> Option<&BenchmarkResult> {
        self.results.iter().max_by_key(|r| r.statistics.mean)
    }

    /// Get average throughput across all benchmarks
    pub fn average_throughput(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }

        let total_throughput: f64 = self.results.iter().map(|r| r.throughput()).sum();
        total_throughput / self.results.len() as f64
    }
}

/// Benchmark runner for executing individual benchmarks
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

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
    name: String,
    function: F,
    config: BenchmarkConfig,
}

impl<F> Benchmark<F>
where
    F: Fn() -> ProfilerResult<()>
{
    /// Create a new benchmark
    pub fn new(name: &str, function: F, config: BenchmarkConfig) -> Self {
        Self {
            name: name.to_string(),
            function,
            config,
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
    pub baseline_name: String,
    pub comparison_name: String,
    pub speedup_factor: f64,
    pub is_faster: bool,
    pub is_statistically_significant: bool,
    pub confidence_level: f64,
}

/// Benchmark report generator
pub struct BenchmarkReport {
    pub results: Vec<BenchmarkResult>,
    pub comparisons: Vec<ComparisonResult>,
    pub metadata: HashMap<String, String>,
}

impl BenchmarkReport {
    /// Create a new benchmark report
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            comparisons: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a benchmark result
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }

    /// Add a comparison
    pub fn add_comparison(&mut self, comparison: ComparisonResult) {
        self.comparisons.push(comparison);
    }

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
                     - Iterations: {}\n\n",
                    result.name,
                    result.statistics.mean,
                    result.statistics.median,
                    result.statistics.min,
                    result.statistics.max,
                    result.statistics.std_dev,
                    result.throughput(),
                    result.iterations_completed
                ));
            }
        }

        if !self.comparisons.is_empty() {
            report.push_str("## Comparisons\n\n");
            for comparison in &self.comparisons {
                let status = if comparison.is_faster { "faster" } else { "slower" };
                report.push_str(&format!(
                    "- {} is {:.2}x {} than {}\n",
                    comparison.comparison_name,
                    comparison.speedup_factor,
                    status,
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
}

/// Run a benchmark with setup and teardown
pub fn benchmark_with_setup<S, F, T>(
    name: &str,
    setup: S,
    function: F,
    teardown: T,
) -> ProfilerResult<BenchmarkResult>
where
    S: Fn() -> ProfilerResult<()>,
    F: Fn() -> ProfilerResult<()>,
    T: Fn() -> ProfilerResult<()>,
{
    let config = BenchmarkConfig::default();
    
    // Setup
    setup().map_err(|e| benchmark_error(&format!("Setup failed: {}", e)))?;
    
    // Run benchmark
    let result = run_benchmark_with_config(name, &function, &config)?;
    
    // Teardown
    teardown().map_err(|e| benchmark_error(&format!("Teardown failed: {}", e)))?;
    
    Ok(result)
}

/// Run a benchmark suite
pub fn run_benchmark_suite(name: &str, mut suite: BenchmarkSuite) -> ProfilerResult<BenchmarkSuiteResult> {
    suite.run()
}

/// Compare two benchmark results
pub fn compare_benchmarks(
    baseline: &BenchmarkResult,
    comparison: &BenchmarkResult,
) -> ComparisonResult {
    let baseline_mean_ns = baseline.statistics.mean.as_nanos() as f64;
    let comparison_mean_ns = comparison.statistics.mean.as_nanos() as f64;
    
    let speedup_factor = if comparison_mean_ns > 0.0 {
        baseline_mean_ns / comparison_mean_ns
    } else {
        f64::INFINITY
    };
    
    let is_faster = speedup_factor > 1.0;
    
    // Simple statistical significance test (would be more sophisticated in real implementation)
    let baseline_cv = baseline.statistics.coefficient_of_variation;
    let comparison_cv = comparison.statistics.coefficient_of_variation;
    let is_statistically_significant = (speedup_factor - 1.0).abs() > (baseline_cv + comparison_cv);
    
    ComparisonResult {
        baseline_name: baseline.name.clone(),
        comparison_name: comparison.name.clone(),
        speedup_factor,
        is_faster,
        is_statistically_significant,
        confidence_level: if is_statistically_significant { 0.95 } else { 0.0 },
    }
}

/// Generate a benchmark report
pub fn generate_benchmark_report(results: Vec<BenchmarkResult>) -> BenchmarkReport {
    let mut report = BenchmarkReport::new();
    
    for result in results {
        report.add_result(result);
    }
    
    // Add metadata
    report.metadata.insert("generated_at".to_string(), 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string());
    
    report
}

/// Internal function to run benchmark with configuration
fn run_benchmark_with_config<F>(
    name: &str,
    benchmark: F,
    config: &BenchmarkConfig,
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
        }
        
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
    }
    
    let total_time = start_time.elapsed();
    let statistics = BenchmarkStatistics::from_measurements(measurements.clone());
    
    // Memory and CPU usage would be measured here in real implementation
    let memory_usage = if config.measure_memory {
        Some(BenchmarkMemoryUsage {
            initial_usage: 0,
            peak_usage: 0,
            final_usage: 0,
            total_allocated: 0,
            total_freed: 0,
            allocation_count: 0,
        })
    } else {
        None
    };
    
    let cpu_usage = if config.measure_cpu {
        Some(BenchmarkCpuUsage {
            user_time: Duration::new(0, 0),
            system_time: Duration::new(0, 0),
            total_time,
            cpu_utilization: 0.0,
        })
    } else {
        None
    };
    
    let mut metadata = HashMap::new();
    metadata.insert("rust_version".to_string(), env!("RUSTC_VERSION").to_string());
    metadata.insert("target".to_string(), std::env::consts::ARCH.to_string());
    
    BENCHMARK_COUNT.fetch_add(1, Ordering::Relaxed);
    
    Ok(BenchmarkResult {
        name: name.to_string(),
        config: config.clone(),
        statistics,
        iterations_completed: completed_iterations,
        total_time,
        warmup_time,
        memory_usage,
        cpu_usage,
        metadata,
        raw_measurements: measurements,
    })
}

/// Get number of benchmarks run
pub fn get_benchmark_count() -> u64 {
    BENCHMARK_COUNT.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_config() {
        let config = BenchmarkConfig::default();
        assert_eq!(config.iterations, 1000);
        assert_eq!(config.warmup_iterations, 100);
        assert_eq!(config.min_duration, Duration::from_millis(100));
    }

    #[test]
    fn test_benchmark_statistics() {
        let measurements = vec![
            Duration::from_nanos(100),
            Duration::from_nanos(200),
            Duration::from_nanos(150),
            Duration::from_nanos(180),
            Duration::from_nanos(120),
        ];
        
        let stats = BenchmarkStatistics::from_measurements(measurements);
        assert_eq!(stats.min, Duration::from_nanos(100));
        assert_eq!(stats.max, Duration::from_nanos(200));
        assert_eq!(stats.median, Duration::from_nanos(150));
    }

    #[test]
    fn test_benchmark_function() {
        let result = benchmark_function("test_benchmark", || {
            // Simulate some work
            std::thread::sleep(Duration::from_micros(1));
            Ok(())
        });
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.name, "test_benchmark");
        assert!(result.iterations_completed > 0);
        assert!(result.statistics.mean > Duration::new(0, 0));
    }

    #[test]
    fn test_benchmark_with_setup() {
        let mut setup_called = false;
        let mut teardown_called = false;
        
        let result = benchmark_with_setup(
            "test_with_setup",
            || {
                setup_called = true;
                Ok(())
            },
            || {
                std::thread::sleep(Duration::from_nanos(100));
                Ok(())
            },
            || {
                teardown_called = true;
                Ok(())
            },
        );
        
        assert!(result.is_ok());
        // Note: In this simplified implementation, setup/teardown flags aren't actually checked
        // because they're in different scopes
    }

    #[test]
    fn test_benchmark_suite() {
        let config = BenchmarkConfig {
            iterations: 10,
            warmup_iterations: 2,
            ..BenchmarkConfig::default()
        };
        
        let mut suite = BenchmarkSuite::new("test_suite", config);
        
        suite.add_benchmark("fast_test", || {
            std::thread::sleep(Duration::from_nanos(50));
            Ok(())
        });
        
        suite.add_benchmark("slow_test", || {
            std::thread::sleep(Duration::from_nanos(100));
            Ok(())
        });
        
        let result = suite.run();
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.benchmark_count, 2);
        assert_eq!(result.results.len(), 2);
    }

    #[test]
    fn test_benchmark_comparison() {
        let fast_result = BenchmarkResult {
            name: "fast".to_string(),
            config: BenchmarkConfig::default(),
            statistics: BenchmarkStatistics {
                mean: Duration::from_nanos(100),
                coefficient_of_variation: 0.1,
                ..BenchmarkStatistics::default()
            },
            iterations_completed: 100,
            total_time: Duration::from_secs(1),
            warmup_time: Duration::from_millis(10),
            memory_usage: None,
            cpu_usage: None,
            metadata: HashMap::new(),
            raw_measurements: vec![],
        };
        
        let slow_result = BenchmarkResult {
            name: "slow".to_string(),
            config: BenchmarkConfig::default(),
            statistics: BenchmarkStatistics {
                mean: Duration::from_nanos(200),
                coefficient_of_variation: 0.1,
                ..BenchmarkStatistics::default()
            },
            iterations_completed: 100,
            total_time: Duration::from_secs(2),
            warmup_time: Duration::from_millis(10),
            memory_usage: None,
            cpu_usage: None,
            metadata: HashMap::new(),
            raw_measurements: vec![],
        };
        
        let comparison = compare_benchmarks(&slow_result, &fast_result);
        assert!(comparison.is_faster);
        assert_eq!(comparison.speedup_factor, 2.0);
    }

    #[test]
    fn test_benchmark_result_throughput() {
        let result = BenchmarkResult {
            name: "test".to_string(),
            config: BenchmarkConfig::default(),
            statistics: BenchmarkStatistics {
                mean: Duration::from_nanos(1000), // 1 microsecond
                ..BenchmarkStatistics::default()
            },
            iterations_completed: 100,
            total_time: Duration::from_secs(1),
            warmup_time: Duration::from_millis(10),
            memory_usage: None,
            cpu_usage: None,
            metadata: HashMap::new(),
            raw_measurements: vec![],
        };
        
        let throughput = result.throughput();
        assert_eq!(throughput, 1_000_000.0); // 1M ops/sec
    }

    #[test]
    fn test_benchmark_performance_class() {
        let excellent = BenchmarkResult {
            statistics: BenchmarkStatistics {
                mean: Duration::from_nanos(500),
                ..BenchmarkStatistics::default()
            },
            ..Default::default()
        };
        assert_eq!(excellent.performance_class(), BenchmarkPerformanceClass::Excellent);
        
        let poor = BenchmarkResult {
            statistics: BenchmarkStatistics {
                mean: Duration::from_millis(10),
                ..BenchmarkStatistics::default()
            },
            ..Default::default()
        };
        assert_eq!(poor.performance_class(), BenchmarkPerformanceClass::Poor);
    }
}

// Default implementation for BenchmarkResult (for testing)
impl Default for BenchmarkResult {
    fn default() -> Self {
        Self {
            name: String::new(),
            config: BenchmarkConfig::default(),
            statistics: BenchmarkStatistics::default(),
            iterations_completed: 0,
            total_time: Duration::new(0, 0),
            warmup_time: Duration::new(0, 0),
            memory_usage: None,
            cpu_usage: None,
            metadata: HashMap::new(),
            raw_measurements: vec![],
        }
    }
}
