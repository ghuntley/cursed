//! Benchmark harness for running benchmarks and collecting results

use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn, instrument};

use super::metrics::{Metric, MetricType, MetricValue};

/// Configuration for a benchmark run
#[derive(Clone, Debug)]
pub struct BenchmarkConfig {
    /// Number of iterations to run
    pub iterations: usize,
    /// Warmup iterations (not included in results)
    pub warmup: usize,
    /// Whether to collect memory metrics
    pub collect_memory: bool,
    /// Whether to collect GC metrics
    pub collect_gc: bool,
    /// Whether to collect throughput metrics
    pub collect_throughput: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 10,
            warmup: 3,
            collect_memory: true,
            collect_gc: true,
            collect_throughput: false,
        }
    }
}

/// A benchmark function that can be executed
#[derive(Clone)]
pub struct Benchmark {
    /// Name of the benchmark
    pub name: String,
    /// Description of what the benchmark tests
    pub description: String,
    /// The function to run for the benchmark
    pub function: Arc<dyn Fn() -> Vec<Box<dyn Metric>> + Send + Sync>,
    /// Configuration for this benchmark
    pub config: BenchmarkConfig,
}

/// A collection of benchmarks that can be run together
pub struct BenchmarkSuite {
    /// Name of the benchmark suite
    pub name: String,
    /// Description of the benchmark suite
    pub description: String,
    /// Benchmarks to run
    pub benchmarks: Vec<Benchmark>,
}

/// Results from a single benchmark run
#[derive(Clone, Debug)]
pub struct BenchmarkResult {
    /// Name of the benchmark
    pub name: String,
    /// Total execution time
    pub total_time: Duration,
    /// Average execution time per iteration
    pub avg_time: Duration,
    /// Minimum execution time
    pub min_time: Duration,
    /// Maximum execution time
    pub max_time: Duration,
    /// All metrics collected during the benchmark
    pub metrics: Vec<Box<dyn Metric>>,
}

/// Results from a benchmark suite
#[derive(Clone, Debug)]
pub struct BenchmarkResults {
    /// Name of the benchmark suite
    pub suite_name: String,
    /// Results for each benchmark
    pub results: Vec<BenchmarkResult>,
    /// Time when the benchmarks were run
    pub timestamp: std::time::SystemTime,
}

impl Benchmark {
    /// Create a new benchmark
    pub fn new<F>(name: &str, description: &str, function: F) -> Self
    where
        F: Fn() -> Vec<Box<dyn Metric>> + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            function: Arc::new(function),
            config: BenchmarkConfig::default(),
        }
    }

    /// Set the configuration for this benchmark
    pub fn with_config(mut self, config: BenchmarkConfig) -> Self {
        self.config = config;
        self
    }

    /// Run the benchmark and return the results
    #[instrument(skip_all, fields(benchmark = %self.name))]
    pub fn run(&self) -> BenchmarkResult {
        info!(name = %self.name, "Running benchmark");

        // Run warmup iterations
        if self.config.warmup > 0 {
            debug!(warmup = self.config.warmup, "Running warmup iterations");
            for i in 0..self.config.warmup {
                trace!(iteration = i, "Warmup iteration");
                let _ = (self.function)();
            }
        }

        let mut iteration_times = Vec::with_capacity(self.config.iterations);
        let mut all_metrics = Vec::new();

        // Run benchmark iterations
        for i in 0..self.config.iterations {
            trace!(iteration = i, "Benchmark iteration");
            let start = Instant::now();
            let metrics = (self.function)();
            let elapsed = start.elapsed();

            iteration_times.push(elapsed);
            all_metrics.extend(metrics);

            trace!(iteration = i, elapsed_ms = ?elapsed.as_millis(), "Iteration complete");
        }

        // Calculate statistics
        let total_time: Duration = iteration_times.iter().sum();
        let avg_time = total_time / iteration_times.len() as u32;
        let min_time = *iteration_times.iter().min().unwrap_or(&Duration::ZERO);
        let max_time = *iteration_times.iter().max().unwrap_or(&Duration::ZERO);

        debug!(
            name = %self.name,
            iterations = self.config.iterations,
            avg_ms = ?avg_time.as_millis(),
            min_ms = ?min_time.as_millis(),
            max_ms = ?max_time.as_millis(),
            "Benchmark completed"
        );

        BenchmarkResult {
            name: self.name.clone(),
            total_time,
            avg_time,
            min_time,
            max_time,
            metrics: all_metrics,
        }
    }
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            benchmarks: Vec::new(),
        }
    }

    /// Add a benchmark to the suite
    pub fn add_benchmark(&mut self, benchmark: Benchmark) {
        self.benchmarks.push(benchmark);
    }

    /// Run all benchmarks in the suite
    #[instrument(skip_all, fields(suite = %self.name))]
    pub fn run(&self) -> BenchmarkResults {
        info!(name = %self.name, benchmark_count = self.benchmarks.len(), "Running benchmark suite");

        let mut results = Vec::with_capacity(self.benchmarks.len());

        for benchmark in &self.benchmarks {
            let result = benchmark.run();
            results.push(result);
        }

        BenchmarkResults {
            suite_name: self.name.clone(),
            results,
            timestamp: std::time::SystemTime::now(),
        }
    }
}