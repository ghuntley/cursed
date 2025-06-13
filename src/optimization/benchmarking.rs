/// Optimization Benchmarking and Performance Regression Testing
/// 
/// This module provides comprehensive benchmarking infrastructure for measuring
/// optimization performance and detecting regressions.

use crate::error::{Error, Result};
use crate::optimization::OptimizationConfig;
use std::collections::{HashMap, BTreeMap};
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tracing::{debug, info, instrument, warn, error};

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of iterations for each benchmark
    pub iterations: usize,
    /// Warmup iterations before measuring
    pub warmup_iterations: usize,
    /// Maximum time to spend on a single benchmark
    pub max_time: Duration,
    /// Minimum time for stable measurements
    pub min_time: Duration,
    /// Statistical confidence level (0.0 to 1.0)
    pub confidence_level: f64,
    /// Maximum coefficient of variation for stable results
    pub max_cv: f64,
    /// Enable detailed profiling
    pub enable_profiling: bool,
    /// Output directory for benchmark results
    pub output_dir: PathBuf,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 10,
            warmup_iterations: 3,
            max_time: Duration::from_secs(60),
            min_time: Duration::from_millis(100),
            confidence_level: 0.95,
            max_cv: 0.1, // 10%
            enable_profiling: false,
            output_dir: PathBuf::from("benchmark_results"),
        }
    }
}

/// Individual benchmark measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMeasurement {
    /// Benchmark name
    pub name: String,
    /// Duration of the measurement
    pub duration: Duration,
    /// Memory usage during benchmark (bytes)
    pub memory_usage: Option<u64>,
    /// CPU usage percentage
    pub cpu_usage: Option<f64>,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
    /// Timestamp of measurement
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Statistical analysis of benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStatistics {
    /// Benchmark name
    pub name: String,
    /// Number of measurements
    pub count: usize,
    /// Mean execution time
    pub mean: Duration,
    /// Median execution time
    pub median: Duration,
    /// Standard deviation
    pub std_dev: Duration,
    /// Minimum time
    pub min: Duration,
    /// Maximum time
    pub max: Duration,
    /// Coefficient of variation
    pub coefficient_of_variation: f64,
    /// 95th percentile
    pub p95: Duration,
    /// 99th percentile
    pub p99: Duration,
    /// Confidence interval (lower, upper)
    pub confidence_interval: (Duration, Duration),
    /// Outliers detected
    pub outliers: Vec<usize>,
}

impl BenchmarkStatistics {
    /// Create statistics from measurements
    pub fn from_measurements(name: String, measurements: &[BenchmarkMeasurement]) -> Self {
        if measurements.is_empty() {
            return Self::default_with_name(name);
        }
        
        let mut durations: Vec<Duration> = measurements
            .iter()
            .map(|m| m.duration)
            .collect();
        durations.sort();
        
        let count = durations.len();
        let sum: Duration = durations.iter().sum();
        let mean = sum / count as u32;
        
        let median = if count % 2 == 0 {
            (durations[count / 2 - 1] + durations[count / 2]) / 2
        } else {
            durations[count / 2]
        };
        
        let variance = durations
            .iter()
            .map(|&d| {
                let diff = d.as_nanos() as f64 - mean.as_nanos() as f64;
                diff * diff
            })
            .sum::<f64>() / count as f64;
        
        let std_dev = Duration::from_nanos(variance.sqrt() as u64);
        
        let cv = if mean.as_nanos() > 0 {
            std_dev.as_nanos() as f64 / mean.as_nanos() as f64
        } else {
            0.0
        };
        
        let p95_idx = (count as f64 * 0.95) as usize;
        let p99_idx = (count as f64 * 0.99) as usize;
        
        let p95 = durations.get(p95_idx.min(count - 1)).copied().unwrap_or(Duration::default());
        let p99 = durations.get(p99_idx.min(count - 1)).copied().unwrap_or(Duration::default());
        
        // Simple confidence interval calculation (assumes normal distribution)
        let t_value = 1.96; // 95% confidence
        let margin = Duration::from_nanos((t_value * std_dev.as_nanos() as f64 / (count as f64).sqrt()) as u64);
        let confidence_interval = (
            mean.saturating_sub(margin),
            mean + margin
        );
        
        // Simple outlier detection using IQR method
        let q1_idx = count / 4;
        let q3_idx = 3 * count / 4;
        let q1 = durations[q1_idx];
        let q3 = durations[q3_idx];
        let iqr = q3 - q1;
        let lower_bound = q1.saturating_sub(iqr + iqr / 2);
        let upper_bound = q3 + iqr + iqr / 2;
        
        let outliers: Vec<usize> = durations
            .iter()
            .enumerate()
            .filter(|(_, &d)| d < lower_bound || d > upper_bound)
            .map(|(i, _)| i)
            .collect();
        
        Self {
            name,
            count,
            mean,
            median,
            std_dev,
            min: durations[0],
            max: durations[count - 1],
            coefficient_of_variation: cv,
            p95,
            p99,
            confidence_interval,
            outliers,
        }
    }
    
    fn default_with_name(name: String) -> Self {
        Self {
            name,
            count: 0,
            mean: Duration::default(),
            median: Duration::default(),
            std_dev: Duration::default(),
            min: Duration::default(),
            max: Duration::default(),
            coefficient_of_variation: 0.0,
            p95: Duration::default(),
            p99: Duration::default(),
            confidence_interval: (Duration::default(), Duration::default()),
            outliers: Vec::new(),
        }
    }
    
    /// Check if results are stable (low coefficient of variation)
    pub fn is_stable(&self, max_cv: f64) -> bool {
        self.coefficient_of_variation <= max_cv
    }
    
    /// Compare with another set of statistics for regression detection
    pub fn compare(&self, other: &BenchmarkStatistics) -> PerformanceComparison {
        if self.count == 0 || other.count == 0 {
            return PerformanceComparison::Inconclusive;
        }
        
        let self_mean = self.mean.as_nanos() as f64;
        let other_mean = other.mean.as_nanos() as f64;
        
        let change_ratio = (self_mean - other_mean) / other_mean;
        let change_percent = change_ratio * 100.0;
        
        // Simple significance test using confidence intervals
        let self_lower = self.confidence_interval.0.as_nanos() as f64;
        let self_upper = self.confidence_interval.1.as_nanos() as f64;
        let other_lower = other.confidence_interval.0.as_nanos() as f64;
        let other_upper = other.confidence_interval.1.as_nanos() as f64;
        
        let overlapping = !(self_upper < other_lower || other_upper < self_lower);
        
        if overlapping {
            PerformanceComparison::NoSignificantChange
        } else if change_percent < -5.0 {
            PerformanceComparison::Improvement(change_percent.abs())
        } else if change_percent > 5.0 {
            PerformanceComparison::Regression(change_percent)
        } else {
            PerformanceComparison::NoSignificantChange
        }
    }
}

/// Performance comparison result
#[derive(Debug, Clone)]
pub enum PerformanceComparison {
    /// Performance improved by the given percentage
    Improvement(f64),
    /// Performance regressed by the given percentage
    Regression(f64),
    /// No significant change detected
    NoSignificantChange,
    /// Not enough data for comparison
    Inconclusive,
}

/// Benchmark suite for running multiple benchmarks
#[derive(Debug)]
pub struct BenchmarkSuite {
    /// Configuration
    config: BenchmarkConfig,
    /// Registered benchmarks
    benchmarks: HashMap<String, Box<dyn Benchmark + Send + Sync>>,
    /// Historical results
    historical_results: Arc<Mutex<BTreeMap<String, Vec<BenchmarkStatistics>>>>,
    /// Current session results
    current_results: Arc<Mutex<HashMap<String, Vec<BenchmarkMeasurement>>>>,
}

/// Trait for individual benchmarks
pub trait Benchmark {
    /// Get the name of this benchmark
    fn name(&self) -> &str;
    
    /// Run the benchmark once and return the measurement
    fn run_once(&self) -> Result<BenchmarkMeasurement>;
    
    /// Setup before benchmark (optional)
    fn setup(&self) -> Result<()> {
        Ok(())
    }
    
    /// Cleanup after benchmark (optional)
    fn cleanup(&self) -> Result<()> {
        Ok(())
    }
    
    /// Get description of what this benchmark measures
    fn description(&self) -> &str {
        "No description provided"
    }
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(config: BenchmarkConfig) -> Result<Self> {
        std::fs::create_dir_all(&config.output_dir)
            .map_err(|e| Error::Io(e.into()))?;
        
        let mut suite = Self {
            config,
            benchmarks: HashMap::new(),
            historical_results: Arc::new(Mutex::new(BTreeMap::new())),
            current_results: Arc::new(Mutex::new(HashMap::new())),
        };
        
        suite.load_historical_results()?;
        Ok(suite)
    }
    
    /// Register a benchmark
    pub fn register_benchmark(&mut self, benchmark: Box<dyn Benchmark + Send + Sync>) {
        let name = benchmark.name().to_string();
        self.benchmarks.insert(name, benchmark);
    }
    
    /// Run all registered benchmarks
    #[instrument(skip(self))]
    pub fn run_all(&self) -> Result<BenchmarkSuiteResults> {
        let mut results = HashMap::new();
        let mut total_time = Duration::default();
        let start_time = Instant::now();
        
        info!("Running {} benchmarks", self.benchmarks.len());
        
        for (name, benchmark) in &self.benchmarks {
            info!("Running benchmark: {}", name);
            
            let benchmark_start = Instant::now();
            let stats = self.run_benchmark(benchmark.as_ref())?;
            let benchmark_time = benchmark_start.elapsed();
            
            total_time += benchmark_time;
            results.insert(name.clone(), stats);
            
            info!("Completed benchmark: {} in {:?}", name, benchmark_time);
        }
        
        let suite_results = BenchmarkSuiteResults {
            results,
            total_time,
            timestamp: chrono::Utc::now(),
        };
        
        // Save results
        self.save_results(&suite_results)?;
        
        // Check for regressions
        let regressions = self.detect_regressions(&suite_results)?;
        if !regressions.is_empty() {
            warn!("Performance regressions detected in {} benchmarks", regressions.len());
            for regression in &regressions {
                warn!("Regression in {}: {:?}", regression.benchmark_name, regression.comparison);
            }
        }
        
        info!("Benchmark suite completed in {:?}", start_time.elapsed());
        Ok(suite_results)
    }
    
    /// Run a specific benchmark
    pub fn run_benchmark(&self, benchmark: &dyn Benchmark) -> Result<BenchmarkStatistics> {
        let name = benchmark.name();
        debug!("Starting benchmark: {}", name);
        
        benchmark.setup()?;
        
        let mut measurements = Vec::new();
        let benchmark_start = Instant::now();
        
        // Warmup iterations
        for i in 0..self.config.warmup_iterations {
            debug!("Warmup iteration {} for {}", i + 1, name);
            let _ = benchmark.run_once()?;
        }
        
        // Actual measurements
        let mut stable_measurements = 0;
        let mut iteration = 0;
        
        while iteration < self.config.iterations || stable_measurements < self.config.iterations / 2 {
            if benchmark_start.elapsed() > self.config.max_time {
                warn!("Benchmark {} exceeded maximum time", name);
                break;
            }
            
            let measurement = benchmark.run_once()?;
            measurements.push(measurement);
            iteration += 1;
            
            // Check stability every few measurements
            if measurements.len() >= 5 {
                let recent_stats = BenchmarkStatistics::from_measurements(
                    name.to_string(),
                    &measurements[measurements.len().saturating_sub(5)..]
                );
                
                if recent_stats.is_stable(self.config.max_cv) {
                    stable_measurements += 1;
                } else {
                    stable_measurements = 0;
                }
            }
            
            if measurements.len() >= self.config.iterations && 
               benchmark_start.elapsed() >= self.config.min_time {
                break;
            }
        }
        
        benchmark.cleanup()?;
        
        let stats = BenchmarkStatistics::from_measurements(name.to_string(), &measurements);
        
        // Store current measurements
        {
            let mut current = self.current_results.lock().unwrap();
            current.insert(name.to_string(), measurements);
        }
        
        debug!("Completed benchmark: {} with {} measurements", name, stats.count);
        Ok(stats)
    }
    
    /// Detect performance regressions
    fn detect_regressions(&self, current: &BenchmarkSuiteResults) -> Result<Vec<RegressionDetection>> {
        let mut regressions = Vec::new();
        let historical = self.historical_results.lock().unwrap();
        
        for (benchmark_name, current_stats) in &current.results {
            if let Some(history) = historical.get(benchmark_name) {
                if let Some(last_stats) = history.last() {
                    let comparison = current_stats.compare(last_stats);
                    
                    match comparison {
                        PerformanceComparison::Regression(percent) => {
                            regressions.push(RegressionDetection {
                                benchmark_name: benchmark_name.clone(),
                                current_stats: current_stats.clone(),
                                baseline_stats: last_stats.clone(),
                                comparison,
                                severity: if percent > 20.0 {
                                    RegressionSeverity::Critical
                                } else if percent > 10.0 {
                                    RegressionSeverity::Major
                                } else {
                                    RegressionSeverity::Minor
                                },
                            });
                        },
                        _ => {}
                    }
                }
            }
        }
        
        Ok(regressions)
    }
    
    /// Save benchmark results
    fn save_results(&self, results: &BenchmarkSuiteResults) -> Result<()> {
        // Save current results
        let results_file = self.config.output_dir.join("latest_results.json");
        let json = serde_json::to_string_pretty(results)
            .map_err(|e| Error::Parse(format!("Failed to serialize results: {}", e)))?;
        
        std::fs::write(results_file, json)
            .map_err(|e| Error::Io(e.into()))?;
        
        // Update historical results
        {
            let mut historical = self.historical_results.lock().unwrap();
            for (name, stats) in &results.results {
                historical.entry(name.clone())
                    .or_default()
                    .push(stats.clone());
            }
        }
        
        // Save historical results
        self.save_historical_results()?;
        
        debug!("Saved benchmark results to {}", self.config.output_dir.display());
        Ok(())
    }
    
    /// Load historical results
    fn load_historical_results(&self) -> Result<()> {
        let history_file = self.config.output_dir.join("historical_results.json");
        
        if history_file.exists() {
            let content = std::fs::read_to_string(&history_file)
                .map_err(|e| Error::Io(e.into()))?;
            
            let historical: BTreeMap<String, Vec<BenchmarkStatistics>> = serde_json::from_str(&content)
                .map_err(|e| Error::Parse(format!("Failed to parse historical results: {}", e)))?;
            
            *self.historical_results.lock().unwrap() = historical;
            debug!("Loaded historical benchmark results");
        }
        
        Ok(())
    }
    
    /// Save historical results
    fn save_historical_results(&self) -> Result<()> {
        let history_file = self.config.output_dir.join("historical_results.json");
        let historical = self.historical_results.lock().unwrap();
        
        let json = serde_json::to_string_pretty(&*historical)
            .map_err(|e| Error::Parse(format!("Failed to serialize historical results: {}", e)))?;
        
        std::fs::write(history_file, json)
            .map_err(|e| Error::Io(e.into()))?;
        
        Ok(())
    }
    
    /// Generate benchmark report
    pub fn generate_report(&self, results: &BenchmarkSuiteResults) -> Result<String> {
        let mut report = String::new();
        report.push_str("# CURSED Compiler Benchmark Report\n\n");
        report.push_str(&format!("Generated: {}\n", results.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("Total Time: {:?}\n\n", results.total_time));
        
        // Summary table
        report.push_str("## Summary\n\n");
        report.push_str("| Benchmark | Mean | Median | Std Dev | Min | Max | CV% |\n");
        report.push_str("|-----------|------|--------|---------|-----|-----|----- |\n");
        
        for (name, stats) in &results.results {
            report.push_str(&format!(
                "| {} | {:?} | {:?} | {:?} | {:?} | {:?} | {:.2}% |\n",
                name,
                stats.mean,
                stats.median,
                stats.std_dev,
                stats.min,
                stats.max,
                stats.coefficient_of_variation * 100.0
            ));
        }
        
        // Detailed results
        report.push_str("\n## Detailed Results\n\n");
        for (name, stats) in &results.results {
            report.push_str(&format!("### {}\n\n", name));
            report.push_str(&format!("- **Measurements**: {}\n", stats.count));
            report.push_str(&format!("- **Mean**: {:?}\n", stats.mean));
            report.push_str(&format!("- **Median**: {:?}\n", stats.median));
            report.push_str(&format!("- **Standard Deviation**: {:?}\n", stats.std_dev));
            report.push_str(&format!("- **95th Percentile**: {:?}\n", stats.p95));
            report.push_str(&format!("- **99th Percentile**: {:?}\n", stats.p99));
            report.push_str(&format!("- **Coefficient of Variation**: {:.2}%\n", stats.coefficient_of_variation * 100.0));
            report.push_str(&format!("- **Outliers**: {} ({:.1}%)\n", 
                stats.outliers.len(), 
                stats.outliers.len() as f64 / stats.count as f64 * 100.0));
            
            if stats.is_stable(self.config.max_cv) {
                report.push_str("- **Status**: ✅ Stable\n");
            } else {
                report.push_str("- **Status**: ⚠️ Unstable (high variance)\n");
            }
            
            report.push_str("\n");
        }
        
        Ok(report)
    }
    
    /// Get benchmark configuration
    pub fn config(&self) -> &BenchmarkConfig {
        &self.config
    }
}

/// Results of running a benchmark suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuiteResults {
    /// Results for each benchmark
    pub results: HashMap<String, BenchmarkStatistics>,
    /// Total time for all benchmarks
    pub total_time: Duration,
    /// Timestamp of the benchmark run
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Regression detection result
#[derive(Debug, Clone)]
pub struct RegressionDetection {
    /// Name of the benchmark
    pub benchmark_name: String,
    /// Current statistics
    pub current_stats: BenchmarkStatistics,
    /// Baseline statistics for comparison
    pub baseline_stats: BenchmarkStatistics,
    /// Comparison result
    pub comparison: PerformanceComparison,
    /// Severity of the regression
    pub severity: RegressionSeverity,
}

/// Severity of performance regression
#[derive(Debug, Clone, PartialEq)]
pub enum RegressionSeverity {
    /// Minor regression (5-10% slower)
    Minor,
    /// Major regression (10-20% slower)
    Major,
    /// Critical regression (>20% slower)
    Critical,
}

/// Built-in optimization benchmarks
pub struct OptimizationBenchmarks {
    config: OptimizationConfig,
}

impl OptimizationBenchmarks {
    /// Create optimization benchmarks
    pub fn new(config: OptimizationConfig) -> Self {
        Self { config }
    }
    
    /// Create a benchmark suite with standard optimization benchmarks
    pub fn create_suite(&self, benchmark_config: BenchmarkConfig) -> Result<BenchmarkSuite> {
        let mut suite = BenchmarkSuite::new(benchmark_config)?;
        
        // Register standard benchmarks
        suite.register_benchmark(Box::new(CompilationSpeedBenchmark::new()));
        suite.register_benchmark(Box::new(OptimizationPassBenchmark::new()));
        suite.register_benchmark(Box::new(MemoryUsageBenchmark::new()));
        suite.register_benchmark(Box::new(CodeQualityBenchmark::new()));
        
        Ok(suite)
    }
}

/// Benchmark for compilation speed
pub struct CompilationSpeedBenchmark {
    test_files: Vec<PathBuf>,
}

impl CompilationSpeedBenchmark {
    pub fn new() -> Self {
        // In a real implementation, this would load test files
        Self {
            test_files: vec![
                PathBuf::from("test_data/small.csd"),
                PathBuf::from("test_data/medium.csd"),
                PathBuf::from("test_data/large.csd"),
            ],
        }
    }
}

impl Benchmark for CompilationSpeedBenchmark {
    fn name(&self) -> &str {
        "compilation_speed"
    }
    
    fn description(&self) -> &str {
        "Measures compilation speed for various code sizes"
    }
    
    fn run_once(&self) -> Result<BenchmarkMeasurement> {
        let start = Instant::now();
        
        // Simulate compilation
        std::thread::sleep(Duration::from_millis(50 + rand::random::<u64>() % 20));
        
        let duration = start.elapsed();
        
        Ok(BenchmarkMeasurement {
            name: self.name().to_string(),
            duration,
            memory_usage: Some(1024 * 1024), // Simulated
            cpu_usage: Some(75.0),
            custom_metrics: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Benchmark for optimization passes
pub struct OptimizationPassBenchmark;

impl OptimizationPassBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Benchmark for OptimizationPassBenchmark {
    fn name(&self) -> &str {
        "optimization_passes"
    }
    
    fn description(&self) -> &str {
        "Measures optimization pass execution time"
    }
    
    fn run_once(&self) -> Result<BenchmarkMeasurement> {
        let start = Instant::now();
        
        // Simulate optimization passes
        std::thread::sleep(Duration::from_millis(30 + rand::random::<u64>() % 15));
        
        let duration = start.elapsed();
        
        Ok(BenchmarkMeasurement {
            name: self.name().to_string(),
            duration,
            memory_usage: Some(512 * 1024),
            cpu_usage: Some(90.0),
            custom_metrics: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Benchmark for memory usage
pub struct MemoryUsageBenchmark;

impl MemoryUsageBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Benchmark for MemoryUsageBenchmark {
    fn name(&self) -> &str {
        "memory_usage"
    }
    
    fn description(&self) -> &str {
        "Measures peak memory usage during compilation"
    }
    
    fn run_once(&self) -> Result<BenchmarkMeasurement> {
        let start = Instant::now();
        
        // Simulate memory-intensive operation
        let _memory: Vec<u8> = vec![0; 1024 * 1024]; // 1MB allocation
        std::thread::sleep(Duration::from_millis(10));
        
        let duration = start.elapsed();
        
        Ok(BenchmarkMeasurement {
            name: self.name().to_string(),
            duration,
            memory_usage: Some(1024 * 1024),
            cpu_usage: Some(50.0),
            custom_metrics: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Benchmark for code quality metrics
pub struct CodeQualityBenchmark;

impl CodeQualityBenchmark {
    pub fn new() -> Self {
        Self
    }
}

impl Benchmark for CodeQualityBenchmark {
    fn name(&self) -> &str {
        "code_quality"
    }
    
    fn description(&self) -> &str {
        "Measures generated code quality metrics"
    }
    
    fn run_once(&self) -> Result<BenchmarkMeasurement> {
        let start = Instant::now();
        
        // Simulate code quality analysis
        std::thread::sleep(Duration::from_millis(25 + rand::random::<u64>() % 10));
        
        let duration = start.elapsed();
        
        let mut custom_metrics = HashMap::new();
        custom_metrics.insert("instructions_eliminated".to_string(), 150.0);
        custom_metrics.insert("functions_inlined".to_string(), 25.0);
        custom_metrics.insert("code_size_reduction".to_string(), 12.5);
        
        Ok(BenchmarkMeasurement {
            name: self.name().to_string(),
            duration,
            memory_usage: Some(256 * 1024),
            cpu_usage: Some(60.0),
            custom_metrics,
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Performance regression detector
pub struct PerformanceRegression {
    suite: BenchmarkSuite,
    alert_threshold: f64,
}

impl PerformanceRegression {
    /// Create a new regression detector
    pub fn new(suite: BenchmarkSuite, alert_threshold: f64) -> Self {
        Self {
            suite,
            alert_threshold,
        }
    }
    
    /// Run regression detection
    pub fn detect_regressions(&self) -> Result<Vec<RegressionDetection>> {
        let results = self.suite.run_all()?;
        self.suite.detect_regressions(&results)
    }
    
    /// Check if any critical regressions exist
    pub fn has_critical_regressions(&self) -> Result<bool> {
        let regressions = self.detect_regressions()?;
        Ok(regressions.iter().any(|r| r.severity == RegressionSeverity::Critical))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_benchmark_statistics() {
        let measurements = vec![
            BenchmarkMeasurement {
                name: "test".to_string(),
                duration: Duration::from_millis(100),
                memory_usage: None,
                cpu_usage: None,
                custom_metrics: HashMap::new(),
                timestamp: chrono::Utc::now(),
            },
            BenchmarkMeasurement {
                name: "test".to_string(),
                duration: Duration::from_millis(110),
                memory_usage: None,
                cpu_usage: None,
                custom_metrics: HashMap::new(),
                timestamp: chrono::Utc::now(),
            },
            BenchmarkMeasurement {
                name: "test".to_string(),
                duration: Duration::from_millis(90),
                memory_usage: None,
                cpu_usage: None,
                custom_metrics: HashMap::new(),
                timestamp: chrono::Utc::now(),
            },
        ];
        
        let stats = BenchmarkStatistics::from_measurements("test".to_string(), &measurements);
        
        assert_eq!(stats.count, 3);
        assert_eq!(stats.mean, Duration::from_millis(100));
        assert_eq!(stats.median, Duration::from_millis(100));
        assert_eq!(stats.min, Duration::from_millis(90));
        assert_eq!(stats.max, Duration::from_millis(110));
    }
    
    #[test]
    fn test_benchmark_suite() {
        let temp_dir = TempDir::new().unwrap();
        let config = BenchmarkConfig {
            output_dir: temp_dir.path().to_path_buf(),
            iterations: 3,
            warmup_iterations: 1,
            ..Default::default()
        };
        
        let mut suite = BenchmarkSuite::new(config).unwrap();
        suite.register_benchmark(Box::new(CompilationSpeedBenchmark::new()));
        
        let results = suite.run_all().unwrap();
        assert!(results.results.contains_key("compilation_speed"));
    }
    
    #[test]
    fn test_performance_comparison() {
        let stats1 = BenchmarkStatistics {
            name: "test".to_string(),
            count: 10,
            mean: Duration::from_millis(100),
            confidence_interval: (Duration::from_millis(95), Duration::from_millis(105)),
            ..BenchmarkStatistics::default_with_name("test".to_string())
        };
        
        let stats2 = BenchmarkStatistics {
            name: "test".to_string(),
            count: 10,
            mean: Duration::from_millis(120),
            confidence_interval: (Duration::from_millis(115), Duration::from_millis(125)),
            ..BenchmarkStatistics::default_with_name("test".to_string())
        };
        
        let comparison = stats2.compare(&stats1);
        match comparison {
            PerformanceComparison::Regression(percent) => {
                assert!(percent > 15.0 && percent < 25.0);
            },
            _ => panic!("Expected regression"),
        }
    }
}
