use crate::error::CursedError;
// Benchmarking framework for performance testing and regression detection

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

// use crate::profiling::core::{ProfilerError, CursedProfiler, ProfilerConfig};

/// Benchmark suite coordinator
#[derive(Debug)]
pub struct BenchmarkSuite {
impl BenchmarkSuite {
    pub fn new(name: String, config: BenchmarkConfig) -> Self {
        Self {
        }
    }
    
    #[instrument(skip(self))]
    pub fn add_benchmark(&mut self, benchmark: Benchmark) {
        info!("Adding benchmark: {}", benchmark.name);
        self.benchmarks.push(benchmark);
    #[instrument(skip(self))]
    pub fn run_all(&mut self) -> crate::error::Result<()> {
        info!("Running benchmark suite: {} ({} benchmarks)", self.name, self.benchmarks.len());
        
        let mut results = BenchmarkResults::new(self.name.clone());
        
        // Create a separate loop to avoid borrowing issues
        let benchmark_count = self.benchmarks.len();
        for i in 0..benchmark_count {
            let benchmark_name = self.benchmarks[i].name.clone();
            info!("Running benchmark: {}", benchmark_name);
            
            let result = self.run_benchmark_by_index(i)?;
            results.add_result(benchmark_name, result);
        results.calculate_summary();
        
        // Check for regressions if baseline exists
        if let Some(baseline) = &self.baseline {
            results.regression_analysis = Some(self.analyze_regressions(baseline, &results));
        Ok(results)
    fn run_benchmark_by_index(&self, index: usize) -> crate::error::Result<()> {
        let benchmark = &self.benchmarks[index];
        let mut measurements = Vec::new();
        let mut profiler = CursedProfiler::new(ProfilerConfig::default());
        
        // Warmup runs
        for i in 0..self.config.warmup_iterations {
            debug!("Warmup iteration {}/{}", i + 1, self.config.warmup_iterations);
            (benchmark.function)();
        // Measurement runs
        for i in 0..self.config.measurement_iterations {
            debug!("Measurement iteration {}/{}", i + 1, self.config.measurement_iterations);
            
            // Start profiling if enabled
            if self.config.enable_profiling {
                profiler.start_session(format!("{}_iter_{}", benchmark.name, i))?;
            let start = Instant::now();
            (benchmark.function)();
            let duration = start.elapsed();
            
            // Stop profiling
            let profile_data = if self.config.enable_profiling {
                Some(profiler.stop_session()?)
            } else {
                None
            
            measurements.push(BenchmarkMeasurement {
            });
        Ok(BenchmarkResult::from_measurements(
        ))
    fn measure_memory_usage(&self) -> MemoryUsage {
        // In a real implementation, this would measure actual memory usage
        MemoryUsage {
        }
    }
    
    fn analyze_regressions(
    ) -> RegressionAnalysis {
        let mut regressions = Vec::new();
        let mut improvements = Vec::new();
        
        for (name, current_result) in &current.results {
            if let Some(baseline_result) = baseline.results.get(name) {
                let performance_change = Self::calculate_performance_change(
                );
                
                match performance_change {
                    PerformanceChange::Regression { percentage, .. } => {
                        if percentage > self.config.regression_threshold {
                            regressions.push(RegressionDetection {
                            });
                        }
                    }
                    PerformanceChange::Improvement { percentage, .. } => {
                        improvements.push(RegressionDetection {
                        });
                    }
                    PerformanceChange::NoChange => {}
                }
            }
        let total_benchmarks = current.results.len();
        let regression_count = regressions.len();
        
        RegressionAnalysis {
            regression_rate: regression_count as f64 / total_benchmarks as f64,
        }
    }
    
    fn calculate_performance_change(
    ) -> PerformanceChange {
        let baseline_mean = baseline.statistics.mean;
        let current_mean = current.statistics.mean;
        
        let percentage_change = ((current_mean.as_secs_f64() - baseline_mean.as_secs_f64()) 
            / baseline_mean.as_secs_f64()) * 100.0;
        
        const NOISE_THRESHOLD: f64 = 2.0; // 2% noise threshold
        
        if percentage_change > NOISE_THRESHOLD {
            PerformanceChange::Regression {
            }
        } else if percentage_change < -NOISE_THRESHOLD {
            PerformanceChange::Improvement {
            }
        } else {
            PerformanceChange::NoChange
        }
    }
    
    fn calculate_regression_severity(percentage: f64) -> RegressionSeverity {
        match percentage {
        }
    }
    
    pub fn set_baseline(&mut self, results: BenchmarkResults) {
        info!("Setting baseline for benchmark suite: {}", self.name);
        self.baseline = Some(results);
    pub fn load_baseline(&mut self, path: &str) -> crate::error::Result<()> {
        let baseline = BenchmarkResults::load_from_file(path)?;
        self.set_baseline(baseline);
        Ok(())
    }
}

/// Individual benchmark definition
pub struct Benchmark {
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
    {
        Self {
        }
    }
    
    pub fn with_expected_duration(mut self, duration: Duration) -> Self {
        self.expected_duration = Some(duration);
        self
    pub fn with_memory_limit(mut self, limit: usize) -> Self {
        self.memory_limit = Some(limit);
        self
    }
}

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub regression_threshold: f64, // Percentage threshold for regression detection
impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            regression_threshold: 10.0, // 10% regression threshold
        }
    }
/// Results from running all benchmarks in a suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
impl BenchmarkResults {
    pub fn new(suite_name: String) -> Self {
        Self {
        }
    }
    
    pub fn add_result(&mut self, name: String, result: BenchmarkResult) {
        self.results.insert(name, result);
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
            average_duration: if total_benchmarks > 0 {
                total_duration / total_benchmarks as u32
            } else {
                Duration::default()
    pub fn save_to_file(&self, path: &str) -> crate::error::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))?;
        
        std::fs::write(path, json)
            .map_err(ProfilerError::IoError)?;
        
        Ok(())
    pub fn load_from_file(path: &str) -> crate::error::Result<()> {
        let content = std::fs::read_to_string(path)
            .map_err(ProfilerError::IoError)?;
        
        serde_json::from_str(&content)
            .map_err(|e| ProfilerError::SerializationError(e.to_string()))
    }
}

/// Result from running a single benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
impl BenchmarkResult {
    pub fn from_measurements(name: String, measurements: Vec<BenchmarkMeasurement>) -> Self {
        let statistics = BenchmarkStatistics::calculate(&measurements);
        
        Self {
        }
    }
/// Individual benchmark measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMeasurement {
/// Memory usage information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryUsage {
/// Statistical analysis of benchmark measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStatistics {
impl BenchmarkStatistics {
    pub fn calculate(measurements: &[BenchmarkMeasurement]) -> Self {
        if measurements.is_empty() {
            return Self::default();
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
        
        Self {
        }
    }
impl Default for BenchmarkStatistics {
    fn default() -> Self {
        Self {
        }
    }
/// Summary of all benchmarks in a suite
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BenchmarkSummary {
/// Performance change analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceChange {
    Regression {
    Improvement {
impl fmt::Display for PerformanceChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regression { percentage, baseline_mean, current_mean } => {
                       percentage, baseline_mean, current_mean)
            }
            Self::Improvement { percentage, baseline_mean, current_mean } => {
                       percentage, baseline_mean, current_mean)
            }
        }
    }
/// Regression detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionDetection {
/// Severity levels for performance regressions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegressionSeverity {
/// Complete regression analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAnalysis {
impl RegressionAnalysis {
    pub fn has_critical_regressions(&self) -> bool {
        self.regressions
            .iter()
            .any(|r| r.severity == RegressionSeverity::Critical)
    pub fn summary(&self) -> String {
        format!(
            self.regression_rate * 100.0
        )
    }
}

/// Micro-benchmark builder for individual functions
pub struct MicroBenchmark;

impl MicroBenchmark {
    pub fn function<F>(name: &str, f: F) -> Benchmark
    where
    {
        Benchmark::new(format!("micro_{}", name), f)
    pub fn allocator<F>(name: &str, f: F) -> Benchmark
    where
    {
        Benchmark::new(format!("alloc_{}", name), f)
            .with_memory_limit(1024 * 1024) // 1MB limit
    pub fn computation<F>(name: &str, f: F) -> Benchmark
    where
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
    {
        Benchmark::new(format!("macro_{}", name), f)
            .with_expected_duration(Duration::from_secs(1))
    pub fn compilation<F>(name: &str, f: F) -> Benchmark
    where
    {
        Benchmark::new(format!("compile_{}", name), f)
            .with_expected_duration(Duration::from_secs(10))
    pub fn end_to_end<F>(name: &str, f: F) -> Benchmark
    where
    {
        Benchmark::new(format!("e2e_{}", name), f)
            .with_expected_duration(Duration::from_secs(30))
    }
}

