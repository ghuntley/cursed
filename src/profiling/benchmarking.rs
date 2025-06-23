// Benchmarking framework for performance testing and regression detection

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

use crate::profiling::core::{ProfilerError, CursedProfiler, ProfilerConfig};

/// Benchmark suite coordinator
#[derive(Debug)]
pub struct BenchmarkSuite {
    name: String,
    benchmarks: Vec<Benchmark>,
    config: BenchmarkConfig,
    baseline: Option<BenchmarkResults>,
}

impl BenchmarkSuite {
    pub fn new(name: String, config: BenchmarkConfig) -> Self {
        Self {
            name,
            benchmarks: Vec::new(),
            config,
            baseline: None,
        }
    }
    
    #[instrument(skip(self))]
    pub fn add_benchmark(&mut self, benchmark: Benchmark) {
        info!("Adding benchmark: {}", benchmark.name);
        self.benchmarks.push(benchmark);
    }
    
    #[instrument(skip(self))]
    pub fn run_all(&mut self) -> Result<(), Error> {
        info!("Running benchmark suite: {} ({} benchmarks)", self.name, self.benchmarks.len());
        
        let mut results = BenchmarkResults::new(self.name.clone());
        
        // Create a separate loop to avoid borrowing issues
        let benchmark_count = self.benchmarks.len();
        for i in 0..benchmark_count {
            let benchmark_name = self.benchmarks[i].name.clone();
            info!("Running benchmark: {}", benchmark_name);
            
            let result = self.run_benchmark_by_index(i)?;
            results.add_result(benchmark_name, result);
        }
        
        results.calculate_summary();
        
        // Check for regressions if baseline exists
        if let Some(baseline) = &self.baseline {
            results.regression_analysis = Some(self.analyze_regressions(baseline, &results));
        }
        
        Ok(results)
    }
    
    fn run_benchmark_by_index(&self, index: usize) -> Result<(), Error> {
        let benchmark = &self.benchmarks[index];
        let mut measurements = Vec::new();
        let mut profiler = CursedProfiler::new(ProfilerConfig::default());
        
        // Warmup runs
        for i in 0..self.config.warmup_iterations {
            debug!("Warmup iteration {}/{}", i + 1, self.config.warmup_iterations);
            (benchmark.function)();
        }
        
        // Measurement runs
        for i in 0..self.config.measurement_iterations {
            debug!("Measurement iteration {}/{}", i + 1, self.config.measurement_iterations);
            
            // Start profiling if enabled
            if self.config.enable_profiling {
                profiler.start_session(format!("{}_iter_{}", benchmark.name, i))?;
            }
            
            let start = Instant::now();
            (benchmark.function)();
            let duration = start.elapsed();
            
            // Stop profiling
            let profile_data = if self.config.enable_profiling {
                Some(profiler.stop_session()?)
            } else {
                None
            };
            
            measurements.push(BenchmarkMeasurement {
                iteration: i,
                duration,
                timestamp: SystemTime::now(),
                profile_data,
                memory_usage: self.measure_memory_usage(),
            });
        }
        
        Ok(BenchmarkResult::from_measurements(
            benchmark.name.clone(),
            measurements,
        ))
    }
    
    fn measure_memory_usage(&self) -> MemoryUsage {
        // In a real implementation, this would measure actual memory usage
        MemoryUsage {
            heap_used: 0,
            heap_allocated: 0,
            stack_size: 0,
        }
    }
    
    fn analyze_regressions(
        &self,
        baseline: &BenchmarkResults,
        current: &BenchmarkResults,
    ) -> RegressionAnalysis {
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();
        
        for (name, current_result) in &current.results {
            if let Some(baseline_result) = baseline.results.get(name) {
                let performance_change = Self::calculate_performance_change(
                    baseline_result,
                    current_result,
                );
                
                match performance_change {
                    PerformanceChange::Regression { percentage, .. } => {
                        if percentage > self.config.regression_threshold {
                            regressions.push(RegressionDetection {
                                benchmark_name: name.clone(),
                                change_type: performance_change,
                                severity: Self::calculate_regression_severity(percentage),
                            });
                        }
                    }
                    PerformanceChange::Improvement { percentage, .. } => {
                        improvements.push(RegressionDetection {
                            benchmark_name: name.clone(),
                            change_type: performance_change,
                            severity: RegressionSeverity::None,
                        });
                    }
                    PerformanceChange::NoChange => {}
                }
            }
        }
        
        let total_benchmarks = current.results.len();
        let regression_count = regressions.len();
        
        RegressionAnalysis {
            regressions,
            improvements,
            total_benchmarks,
            regression_rate: regression_count as f64 / total_benchmarks as f64,
        }
    }
    
    fn calculate_performance_change(
        baseline: &BenchmarkResult,
        current: &BenchmarkResult,
    ) -> PerformanceChange {
        let baseline_mean = baseline.statistics.mean;
        let current_mean = current.statistics.mean;
        
        let percentage_change = ((current_mean.as_secs_f64() - baseline_mean.as_secs_f64()) 
            / baseline_mean.as_secs_f64()) * 100.0;
        
        const NOISE_THRESHOLD: f64 = 2.0; // 2% noise threshold
        
        if percentage_change > NOISE_THRESHOLD {
            PerformanceChange::Regression {
                percentage: percentage_change,
                baseline_mean,
                current_mean,
            }
        } else if percentage_change < -NOISE_THRESHOLD {
            PerformanceChange::Improvement {
                percentage: -percentage_change,
                baseline_mean,
                current_mean,
            }
        } else {
            PerformanceChange::NoChange
        }
    }
    
    fn calculate_regression_severity(percentage: f64) -> RegressionSeverity {
        match percentage {
            p if p >= 50.0 => RegressionSeverity::Critical,
            p if p >= 20.0 => RegressionSeverity::High,
            p if p >= 10.0 => RegressionSeverity::Medium,
            _ => RegressionSeverity::Low,
        }
    }
    
    pub fn set_baseline(&mut self, results: BenchmarkResults) {
        info!("Setting baseline for benchmark suite: {}", self.name);
        self.baseline = Some(results);
    }
    
    pub fn load_baseline(&mut self, path: &str) -> Result<(), Error> {
        let baseline = BenchmarkResults::load_from_file(path)?;
        self.set_baseline(baseline);
        Ok(())
    }
}

/// Individual benchmark definition
pub struct Benchmark {
    pub name: String,
    pub function: Box<dyn Fn() + Send + Sync>,
    pub expected_duration: Option<Duration>,
    pub memory_limit: Option<usize>,
}

impl std::fmt::Debug for Benchmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Benchmark")
            .field("name", &self.name)
            .field("expected_duration", &self.expected_duration)
            .field("memory_limit", &self.memory_limit)
            .finish()
    }
}

impl Benchmark {
    pub fn new<F>(name: String, function: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            name,
            function: Box::new(function),
            expected_duration: None,
            memory_limit: None,
        }
    }
    
    pub fn with_expected_duration(mut self, duration: Duration) -> Self {
        self.expected_duration = Some(duration);
        self
    }
    
    pub fn with_memory_limit(mut self, limit: usize) -> Self {
        self.memory_limit = Some(limit);
        self
    }
}

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub warmup_iterations: usize,
    pub measurement_iterations: usize,
    pub enable_profiling: bool,
    pub regression_threshold: f64, // Percentage threshold for regression detection
    pub timeout: Duration,
    pub memory_limit: Option<usize>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 3,
            measurement_iterations: 10,
            enable_profiling: false,
            regression_threshold: 10.0, // 10% regression threshold
            timeout: Duration::from_secs(60),
            memory_limit: None,
        }
    }
}

/// Results from running all benchmarks in a suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    pub suite_name: String,
    pub timestamp: SystemTime,
    pub results: HashMap<String, BenchmarkResult>,
    pub summary: BenchmarkSummary,
    pub regression_analysis: Option<RegressionAnalysis>,
}

impl BenchmarkResults {
    pub fn new(suite_name: String) -> Self {
        Self {
            suite_name,
            timestamp: SystemTime::now(),
            results: HashMap::new(),
            summary: BenchmarkSummary::default(),
            regression_analysis: None,
        }
    }
    
    pub fn add_result(&mut self, name: String, result: BenchmarkResult) {
        self.results.insert(name, result);
    }
    
    pub fn calculate_summary(&mut self) {
        let total_benchmarks = self.results.len();
        let total_duration: Duration = self.results.values()
            .map(|r| r.statistics.mean)
            .sum();
        
        let fastest = self.results.values()
            .min_by_key(|r| r.statistics.mean)
            .map(|r| r.statistics.mean);
        
        let slowest = self.results.values()
            .max_by_key(|r| r.statistics.mean)
            .map(|r| r.statistics.mean);
        
        self.summary = BenchmarkSummary {
            total_benchmarks,
            total_duration,
            fastest_benchmark: fastest,
            slowest_benchmark: slowest,
            average_duration: if total_benchmarks > 0 {
                total_duration / total_benchmarks as u32
            } else {
                Duration::default()
            },
        };
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
        
        std::fs::write(path, json)
            .map_err(ProfilerError::IoError)?;
        
        Ok(())
    }
    
    pub fn load_from_file(path: &str) -> Result<(), Error> {
        let content = std::fs::read_to_string(path)
            .map_err(ProfilerError::IoError)?;
        
        serde_json::from_str(&content)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))
    }
}

/// Result from running a single benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub measurements: Vec<BenchmarkMeasurement>,
    pub statistics: BenchmarkStatistics,
    pub timestamp: SystemTime,
}

impl BenchmarkResult {
    pub fn from_measurements(name: String, measurements: Vec<BenchmarkMeasurement>) -> Self {
        let statistics = BenchmarkStatistics::calculate(&measurements);
        
        Self {
            name,
            measurements,
            statistics,
            timestamp: SystemTime::now(),
        }
    }
}

/// Individual benchmark measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMeasurement {
    pub iteration: usize,
    pub duration: Duration,
    pub timestamp: SystemTime,
    pub profile_data: Option<crate::profiling::core::ProfileData>,
    pub memory_usage: MemoryUsage,
}

/// Memory usage information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub heap_used: usize,
    pub heap_allocated: usize,
    pub stack_size: usize,
}

/// Statistical analysis of benchmark measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStatistics {
    pub mean: Duration,
    pub median: Duration,
    pub min: Duration,
    pub max: Duration,
    pub standard_deviation: Duration,
    pub variance: Duration,
    pub coefficient_of_variation: f64,
}

impl BenchmarkStatistics {
    pub fn calculate(measurements: &[BenchmarkMeasurement]) -> Self {
        if measurements.is_empty() {
            return Self::default();
        }
        
        let mut durations: Vec<Duration> = measurements
            .iter()
            .map(|m| m.duration)
            .collect();
        durations.sort();
        
        let min = durations[0];
        let max = durations[durations.len() - 1];
        let median = durations[durations.len() / 2];
        
        let total_nanos: u64 = durations
            .iter()
            .map(|d| d.as_nanos() as u64)
            .sum();
        let mean_nanos = total_nanos / durations.len() as u64;
        let mean = Duration::from_nanos(mean_nanos);
        
        // Calculate variance and standard deviation
        let variance_nanos: f64 = durations
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - mean_nanos as f64;
                diff * diff
            })
            .sum::<f64>() / durations.len() as f64;
        
        let std_dev_nanos = variance_nanos.sqrt();
        let standard_deviation = Duration::from_nanos(std_dev_nanos as u64);
        let variance = Duration::from_nanos(variance_nanos as u64);
        
        // Coefficient of variation (relative standard deviation)
        let coefficient_of_variation = if mean_nanos > 0 {
            std_dev_nanos / mean_nanos as f64
        } else {
            0.0
        };
        
        Self {
            mean,
            median,
            min,
            max,
            standard_deviation,
            variance,
            coefficient_of_variation,
        }
    }
}

impl Default for BenchmarkStatistics {
    fn default() -> Self {
        Self {
            mean: Duration::default(),
            median: Duration::default(),
            min: Duration::default(),
            max: Duration::default(),
            standard_deviation: Duration::default(),
            variance: Duration::default(),
            coefficient_of_variation: 0.0,
        }
    }
}

/// Summary of all benchmarks in a suite
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_benchmarks: usize,
    pub total_duration: Duration,
    pub fastest_benchmark: Option<Duration>,
    pub slowest_benchmark: Option<Duration>,
    pub average_duration: Duration,
}

/// Performance change analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceChange {
    Regression {
        percentage: f64,
        baseline_mean: Duration,
        current_mean: Duration,
    },
    Improvement {
        percentage: f64,
        baseline_mean: Duration,
        current_mean: Duration,
    },
    NoChange,
}

impl fmt::Display for PerformanceChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regression { percentage, baseline_mean, current_mean } => {
                write!(f, "Regression: {:.1}% slower ({:?} → {:?})", 
                       percentage, baseline_mean, current_mean)
            }
            Self::Improvement { percentage, baseline_mean, current_mean } => {
                write!(f, "Improvement: {:.1}% faster ({:?} → {:?})", 
                       percentage, baseline_mean, current_mean)
            }
            Self::NoChange => write!(f, "No significant change"),
        }
    }
}

/// Regression detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionDetection {
    pub benchmark_name: String,
    pub change_type: PerformanceChange,
    pub severity: RegressionSeverity,
}

/// Severity levels for performance regressions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegressionSeverity {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Complete regression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysis {
    pub regressions: Vec<RegressionDetection>,
    pub improvements: Vec<RegressionDetection>,
    pub total_benchmarks: usize,
    pub regression_rate: f64,
}

impl RegressionAnalysis {
    pub fn has_critical_regressions(&self) -> bool {
        self.regressions
            .iter()
            .any(|r| r.severity == RegressionSeverity::Critical)
    }
    
    pub fn summary(&self) -> String {
        format!(
            "Regressions: {}, Improvements: {}, Regression Rate: {:.1}%",
            self.regressions.len(),
            self.improvements.len(),
            self.regression_rate * 100.0
        )
    }
}

/// Micro-benchmark builder for individual functions
pub struct MicroBenchmark;

impl MicroBenchmark {
    pub fn function<F>(name: &str, f: F) -> Benchmark
    where
        F: Fn() + Send + Sync + 'static,
    {
        Benchmark::new(format!("micro_{}", name), f)
    }
    
    pub fn allocator<F>(name: &str, f: F) -> Benchmark
    where
        F: Fn() + Send + Sync + 'static,
    {
        Benchmark::new(format!("alloc_{}", name), f)
            .with_memory_limit(1024 * 1024) // 1MB limit
    }
    
    pub fn computation<F>(name: &str, f: F) -> Benchmark
    where
        F: Fn() + Send + Sync + 'static,
    {
        Benchmark::new(format!("compute_{}", name), f)
            .with_expected_duration(Duration::from_millis(100))
    }
}

/// Macro-benchmark builder for complete programs
pub struct MacroBenchmark;

impl MacroBenchmark {
    pub fn program<F>(name: &str, f: F) -> Benchmark
    where
        F: Fn() + Send + Sync + 'static,
    {
        Benchmark::new(format!("macro_{}", name), f)
            .with_expected_duration(Duration::from_secs(1))
    }
    
    pub fn compilation<F>(name: &str, f: F) -> Benchmark
    where
        F: Fn() + Send + Sync + 'static,
    {
        Benchmark::new(format!("compile_{}", name), f)
            .with_expected_duration(Duration::from_secs(10))
    }
    
    pub fn end_to_end<F>(name: &str, f: F) -> Benchmark
    where
        F: Fn() + Send + Sync + 'static,
    {
        Benchmark::new(format!("e2e_{}", name), f)
            .with_expected_duration(Duration::from_secs(30))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benchmark_creation() {
        let benchmark = Benchmark::new("test_bench".to_string(), || {
            std::thread::sleep(Duration::from_millis(1));
        });
        
        assert_eq!(benchmark.name, "test_bench");
    }
    
    #[test]
    fn test_benchmark_suite() {
        let config = BenchmarkConfig::default();
        let mut suite = BenchmarkSuite::new("test_suite".to_string(), config);
        
        let benchmark = Benchmark::new("simple_test".to_string(), || {
            // Simple computation
            let _ = (0..1000).sum::<i32>();
        });
        
        suite.add_benchmark(benchmark);
        assert_eq!(suite.benchmarks.len(), 1);
    }
    
    #[test]
    fn test_benchmark_statistics() {
        let measurements = vec![
            BenchmarkMeasurement {
                iteration: 0,
                duration: Duration::from_millis(100),
                timestamp: SystemTime::now(),
                profile_data: None,
                memory_usage: MemoryUsage::default(),
            },
            BenchmarkMeasurement {
                iteration: 1,
                duration: Duration::from_millis(200),
                timestamp: SystemTime::now(),
                profile_data: None,
                memory_usage: MemoryUsage::default(),
            },
        ];
        
        let stats = BenchmarkStatistics::calculate(&measurements);
        assert_eq!(stats.min, Duration::from_millis(100));
        assert_eq!(stats.max, Duration::from_millis(200));
        assert_eq!(stats.mean, Duration::from_millis(150));
    }
    
    #[test]
    fn test_performance_change_calculation() {
        let baseline = BenchmarkResult {
            name: "test".to_string(),
            measurements: Vec::from([]),
            statistics: BenchmarkStatistics {
                mean: Duration::from_millis(100),
                ..Default::default()
            },
            timestamp: SystemTime::now(),
        };
        
        let current = BenchmarkResult {
            name: "test".to_string(),
            measurements: Vec::from([]),
            statistics: BenchmarkStatistics {
                mean: Duration::from_millis(120),
                ..Default::default()
            },
            timestamp: SystemTime::now(),
        };
        
        let change = BenchmarkSuite::calculate_performance_change(&baseline, &current);
        
        match change {
            PerformanceChange::Regression { percentage, .. } => {
                assert!((percentage - 20.0).abs() < 0.01);
            }
            _ => panic!("Expected regression"),
        }
    }
    
    #[test]
    fn test_micro_benchmark_builder() {
        let benchmark = MicroBenchmark::function("test", || {
            let _ = 1 + 1;
        });
        
        assert_eq!(benchmark.name, "micro_test");
    }
    
    #[test]
    fn test_macro_benchmark_builder() {
        let benchmark = MacroBenchmark::program("test", || {
            // Simulate program execution
            std::thread::sleep(Duration::from_millis(1));
        });
        
        assert_eq!(benchmark.name, "macro_test");
        assert_eq!(benchmark.expected_duration, Some(Duration::from_secs(1)));
    }
}
