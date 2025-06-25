// Benchmarking system for optimization analysis
use crate::error::CursedError;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Results from running optimization benchmarks
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
impl BenchmarkResults {
    pub fn new(test_name: String, measurements: Vec<Duration>) -> Self {
        let iterations = measurements.len();
        let total_time = measurements.iter().sum();
        let average_time = if iterations > 0 {
            total_time / iterations as u32
        } else {
            Duration::from_secs(0)
        let min_time = measurements.iter().min().copied().unwrap_or_default();
        let max_time = measurements.iter().max().copied().unwrap_or_default();

        Self {
        }
    }

    pub fn calculate_stddev(&self) -> Duration {
        if self.measurements.len() < 2 {
            return Duration::from_secs(0);
        let avg_nanos = self.average_time.as_nanos() as f64;
        let variance: f64 = self.measurements
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - avg_nanos;
                diff * diff
            })
            .sum::<f64>() / (self.measurements.len() - 1) as f64;

        Duration::from_nanos(variance.sqrt() as u64)
    }
}

/// Statistical analysis of benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkStatistics {
impl BenchmarkStatistics {
    pub fn from_results(results: &BenchmarkResults) -> Self {
        let mut sorted_measurements = results.measurements.clone();
        sorted_measurements.sort();

        let median = if sorted_measurements.is_empty() {
            Duration::from_secs(0)
        } else {
            let mid = sorted_measurements.len() / 2;
            if sorted_measurements.len() % 2 == 0 {
                let sum = sorted_measurements[mid - 1] + sorted_measurements[mid];
                sum / 2
            } else {
                sorted_measurements[mid]
            }

        let percentile_95 = Self::calculate_percentile(&sorted_measurements, 0.95);
        let percentile_99 = Self::calculate_percentile(&sorted_measurements, 0.99);
        let stddev = results.calculate_stddev();
        
        let coefficient_of_variation = if results.average_time.as_nanos() > 0 {
            stddev.as_nanos() as f64 / results.average_time.as_nanos() as f64
        } else {
            0.0

        Self {
        }
    }

    fn calculate_percentile(sorted_measurements: &[Duration], percentile: f64) -> Duration {
        if sorted_measurements.is_empty() {
            return Duration::from_secs(0);
        let index = (percentile * (sorted_measurements.len() - 1) as f64) as usize;
        sorted_measurements[index.min(sorted_measurements.len() - 1)]
    }
}

/// Benchmark runner for optimization testing
#[derive(Debug)]
pub struct BenchmarkRunner {
impl BenchmarkRunner {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn with_iterations(mut self, warmup: usize, test: usize) -> Self {
        self.warmup_iterations = warmup;
        self.test_iterations = test;
        self
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    pub fn run_benchmark<F>(&self, name: &str, mut test_fn: F) -> crate::error::Result<BenchmarkResults>
    where
    {
        // Warmup phase
        for _ in 0..self.warmup_iterations {
            test_fn()?;
        // Measurement phase
        let mut measurements = Vec::new();
        let start_time = Instant::now();

        for _ in 0..self.test_iterations {
            if let Some(timeout) = self.timeout {
                if start_time.elapsed() > timeout {
                    break;
                }
            }

            let iteration_start = Instant::now();
            test_fn()?;
            let iteration_time = iteration_start.elapsed();
            measurements.push(iteration_time);
        Ok(BenchmarkResults::new(name.to_string(), measurements))
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Comparative benchmark analysis
#[derive(Debug)]
pub struct BenchmarkComparator {
impl BenchmarkComparator {
    pub fn new() -> Self {
        Self {
            comparison_threshold: 0.05, // 5% threshold
        }
    }

    pub fn set_baseline(&mut self, name: String, results: BenchmarkResults) {
        self.baseline_results.insert(name, results);
    pub fn compare_results(&self, name: &str, current: &BenchmarkResults) -> Option<BenchmarkComparison> {
        self.baseline_results.get(name).map(|baseline| {
            let improvement_ratio = if current.average_time.as_nanos() > 0 {
                baseline.average_time.as_nanos() as f64 / current.average_time.as_nanos() as f64
            } else {
                1.0

            let is_significant = (improvement_ratio - 1.0).abs() > self.comparison_threshold;

            BenchmarkComparison {
            }
        })
    }
}

impl Default for BenchmarkComparator {
    fn default() -> Self {
        Self::new()
    }
}

/// Comparison between baseline and current benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkComparison {
impl BenchmarkComparison {
    pub fn is_improvement(&self) -> bool {
        self.improvement_ratio > 1.0
    pub fn is_regression(&self) -> bool {
        self.improvement_ratio < 1.0
    pub fn percentage_change(&self) -> f64 {
        (self.improvement_ratio - 1.0) * 100.0
    }
}
