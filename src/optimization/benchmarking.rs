//! Automated performance benchmarking system

use crate::error::{Result, CursedError};
use crate::optimization::{PerformanceConfig, OptimizationConfig};
use crate::optimization::metrics::CompilationUnit;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{info, debug, warn, instrument};
use serde::{Deserialize, Serialize};

/// Types of benchmarks that can be performed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BenchmarkType {
    /// Benchmark compilation speed
    CompilationSpeed,
    /// Benchmark memory usage during compilation
    MemoryUsage,
    /// Benchmark optimization effectiveness
    OptimizationEffectiveness,
    /// Benchmark cache performance
    CachePerformance,
    /// Benchmark parallel compilation scaling
    ParallelScaling,
}

/// Complexity levels for benchmark test data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplexityLevel {
    /// Simple test cases with minimal complexity
    Simple,
    /// Medium complexity test cases
    Medium,
    /// Complex test cases with high computational requirements
    Complex,
    /// Extreme complexity for stress testing
    Extreme,
}

/// Test data configuration for benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTestData {
    /// Number of compilation units to test with
    pub unit_count: usize,
    /// Complexity level of the test data
    pub complexity_level: ComplexityLevel,
    /// Size of test data in MB
    pub data_size_mb: f64,
}

/// Configuration for a benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Name of the benchmark
    pub name: String,
    /// Type of benchmark to perform
    pub benchmark_type: BenchmarkType,
    /// Number of iterations to run
    pub iterations: usize,
    /// Number of warmup iterations
    pub warmup_iterations: usize,
    /// Test data configuration
    pub test_data: BenchmarkTestData,
}

/// Statistical results from benchmark runs
#[derive(Debug, Clone)]
pub struct BenchmarkStatistics {
    pub mean_time_ms: f64,
    pub median_time_ms: f64,
    pub std_deviation_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub throughput_ops_per_sec: f64,
    pub mean_memory_delta_mb: f64,
    pub peak_memory_usage_mb: f64,
}

impl Default for BenchmarkStatistics {
    fn default() -> Self {
        Self {
            mean_time_ms: 0.0,
            median_time_ms: 0.0,
            std_deviation_ms: 0.0,
            min_time_ms: 0.0,
            max_time_ms: 0.0,
            throughput_ops_per_sec: 0.0,
            mean_memory_delta_mb: 0.0,
            peak_memory_usage_mb: 0.0,
        }
    }
}

/// Results from a complete benchmark run
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub name: String,
    pub benchmark_type: BenchmarkType,
    pub iterations_completed: usize,
    pub warmup_iterations_completed: usize,
    pub total_duration: Duration,
    pub statistics: BenchmarkStatistics,
    pub individual_times: Vec<Duration>,
    pub memory_samples: Vec<f64>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Individual benchmark run data
#[derive(Debug, Clone)]
struct BenchmarkRun {
    duration: Duration,
    memory_before_mb: f64,
    memory_after_mb: f64,
    peak_memory_mb: f64,
    operations_completed: usize,
    success: bool,
    error_message: Option<String>,
}

/// Benchmarking engine for automated performance testing
#[derive(Debug)]
pub struct BenchmarkingEngine {
    config: PerformanceConfig,
    benchmark_history: HashMap<String, Vec<BenchmarkResults>>,
    baseline_results: HashMap<BenchmarkType, BenchmarkStatistics>,
}

impl BenchmarkingEngine {
    /// Create a new benchmarking engine
    #[instrument]
    pub fn new(config: PerformanceConfig) -> Result<Self> {
        info!("Creating benchmarking engine");
        
        Ok(Self {
            config,
            benchmark_history: HashMap::new(),
            baseline_results: HashMap::new(),
        })
    }

    /// Run a benchmark with the given configuration
    #[instrument(skip(self))]
    pub fn run_benchmark(&mut self, config: BenchmarkConfig) -> Result<BenchmarkResults> {
        if !self.config.enable_benchmarking {
            return Err(CursedError::optimization_error(
                "Benchmarking is disabled in configuration"
            ));
        }

        info!("Starting benchmark: {} ({:?})", config.name, config.benchmark_type);

        let start_time = Instant::now();
        let mut individual_times = Vec::new();
        let mut memory_samples = Vec::new();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Run warmup iterations
        debug!("Running {} warmup iterations", config.warmup_iterations);
        for i in 0..config.warmup_iterations {
            match self.run_single_benchmark(&config) {
                Ok(run_data) => {
                    if !run_data.success {
                        warnings.push(format!("Warmup iteration {} failed", i));
                    }
                }
                Err(e) => {
                    warnings.push(format!("Warmup iteration {} error: {}", i, e));
                }
            }
        }

        let warmup_iterations_completed = config.warmup_iterations;

        // Run actual benchmark iterations
        debug!("Running {} benchmark iterations", config.iterations);
        let mut successful_runs = Vec::new();
        
        for i in 0..config.iterations {
            match self.run_single_benchmark(&config) {
                Ok(run_data) => {
                    if run_data.success {
                        individual_times.push(run_data.duration);
                        memory_samples.push(run_data.peak_memory_mb);
                        successful_runs.push(run_data);
                    } else {
                        errors.push(format!("Iteration {} failed: {:?}", i, run_data.error_message));
                    }
                }
                Err(e) => {
                    errors.push(format!("Iteration {} error: {}", i, e));
                }
            }

            // Check if we should stop due to too many failures
            if errors.len() > config.iterations / 2 {
                warn!("Stopping benchmark due to excessive failures");
                break;
            }
        }

        let iterations_completed = successful_runs.len();
        let total_duration = start_time.elapsed();

        // Calculate statistics
        let statistics = self.calculate_statistics(&successful_runs, &config)?;

        let results = BenchmarkResults {
            name: config.name.clone(),
            benchmark_type: config.benchmark_type.clone(),
            iterations_completed,
            warmup_iterations_completed,
            total_duration,
            statistics,
            individual_times,
            memory_samples,
            errors,
            warnings,
        };

        // Store results in history
        self.benchmark_history
            .entry(config.name.clone())
            .or_insert_with(Vec::new)
            .push(results.clone());

        // Update baseline if this is the first run of this type
        if !self.baseline_results.contains_key(&config.benchmark_type) {
            info!("Setting baseline for benchmark type: {:?}", config.benchmark_type);
            self.baseline_results.insert(config.benchmark_type, results.statistics.clone());
        }

        info!(
            "Benchmark completed: {} iterations in {:.2?}, mean time: {:.2}ms",
            iterations_completed,
            total_duration,
            results.statistics.mean_time_ms
        );

        Ok(results)
    }

    /// Run a single benchmark iteration
    #[instrument(skip(self, config))]
    fn run_single_benchmark(&self, config: &BenchmarkConfig) -> Result<BenchmarkRun> {
        let memory_before = self.get_memory_usage_mb();
        let start_time = Instant::now();
        let mut peak_memory = memory_before;
        let mut operations_completed = 0;
        let mut success = true;
        let mut error_message = None;

        // Perform benchmark based on type
        match config.benchmark_type {
            BenchmarkType::CompilationSpeed => {
                operations_completed = self.benchmark_compilation_speed(&config.test_data, &mut peak_memory)?;
            }
            BenchmarkType::MemoryUsage => {
                operations_completed = self.benchmark_memory_usage(&config.test_data, &mut peak_memory)?;
            }
            BenchmarkType::OptimizationEffectiveness => {
                operations_completed = self.benchmark_optimization_effectiveness(&config.test_data, &mut peak_memory)?;
            }
            BenchmarkType::CachePerformance => {
                operations_completed = self.benchmark_cache_performance(&config.test_data, &mut peak_memory)?;
            }
            BenchmarkType::ParallelScaling => {
                operations_completed = self.benchmark_parallel_scaling(&config.test_data, &mut peak_memory)?;
            }
        }

        let duration = start_time.elapsed();
        let memory_after = self.get_memory_usage_mb();

        // Check for anomalies
        if duration > Duration::from_secs(60) {
            success = false;
            error_message = Some("Benchmark exceeded maximum duration".to_string());
        }

        if peak_memory > 1000.0 {
            success = false;
            error_message = Some("Benchmark exceeded memory limit".to_string());
        }

        Ok(BenchmarkRun {
            duration,
            memory_before_mb: memory_before,
            memory_after_mb: memory_after,
            peak_memory_mb: peak_memory,
            operations_completed,
            success,
            error_message,
        })
    }

    /// Benchmark compilation speed
    fn benchmark_compilation_speed(&self, test_data: &BenchmarkTestData, peak_memory: &mut f64) -> Result<usize> {
        debug!("Benchmarking compilation speed with {} units", test_data.unit_count);

        let mut operations = 0;
        for i in 0..test_data.unit_count {
            // Simulate compilation work
            let work_amount = self.get_work_amount_for_complexity(&test_data.complexity_level);
            std::thread::sleep(Duration::from_millis(work_amount));
            
            // Simulate memory usage
            let memory_usage = self.get_memory_usage_mb() + (i as f64 * 2.0);
            *peak_memory = peak_memory.max(memory_usage);
            
            operations += 1;
        }

        Ok(operations)
    }

    /// Benchmark memory usage patterns
    fn benchmark_memory_usage(&self, test_data: &BenchmarkTestData, peak_memory: &mut f64) -> Result<usize> {
        debug!("Benchmarking memory usage with {:.1}MB data", test_data.data_size_mb);

        let mut operations = 0;
        let allocation_size_mb = test_data.data_size_mb / 10.0;

        for i in 0..10 {
            // Simulate memory allocation
            let current_memory = self.get_memory_usage_mb() + (allocation_size_mb * (i + 1) as f64);
            *peak_memory = peak_memory.max(current_memory);
            
            // Simulate processing work
            let work_amount = self.get_work_amount_for_complexity(&test_data.complexity_level);
            std::thread::sleep(Duration::from_millis(work_amount / 2));
            
            operations += 1;
        }

        Ok(operations)
    }

    /// Benchmark optimization effectiveness
    fn benchmark_optimization_effectiveness(&self, test_data: &BenchmarkTestData, peak_memory: &mut f64) -> Result<usize> {
        debug!("Benchmarking optimization effectiveness");

        let mut operations = 0;
        
        // Simulate multiple optimization passes
        for pass in 0..5 {
            let work_per_unit = self.get_work_amount_for_complexity(&test_data.complexity_level);
            
            for i in 0..test_data.unit_count {
                // Each pass does progressively less work (simulating optimization)
                let work_factor = (6 - pass) as f64 / 6.0;
                let actual_work = (work_per_unit as f64 * work_factor) as u64;
                
                std::thread::sleep(Duration::from_millis(actual_work));
                
                let memory_usage = self.get_memory_usage_mb() + (i as f64 * 1.5);
                *peak_memory = peak_memory.max(memory_usage);
                
                operations += 1;
            }
        }

        Ok(operations)
    }

    /// Benchmark cache performance
    fn benchmark_cache_performance(&self, test_data: &BenchmarkTestData, peak_memory: &mut f64) -> Result<usize> {
        debug!("Benchmarking cache performance");

        let mut operations = 0;
        let cache_size = test_data.unit_count / 4; // 25% cache hit rate simulation

        for i in 0..test_data.unit_count {
            let is_cache_hit = i % 4 == 0 && i < cache_size * 4;
            
            let work_amount = if is_cache_hit {
                // Cache hits are much faster
                self.get_work_amount_for_complexity(&test_data.complexity_level) / 10
            } else {
                // Cache misses require full work
                self.get_work_amount_for_complexity(&test_data.complexity_level)
            };

            std::thread::sleep(Duration::from_millis(work_amount));
            
            let memory_usage = self.get_memory_usage_mb() + if is_cache_hit { 0.5 } else { 2.0 };
            *peak_memory = peak_memory.max(memory_usage);
            
            operations += 1;
        }

        Ok(operations)
    }

    /// Benchmark parallel scaling
    fn benchmark_parallel_scaling(&self, test_data: &BenchmarkTestData, peak_memory: &mut f64) -> Result<usize> {
        debug!("Benchmarking parallel scaling");

        let thread_count = num_cpus::get().min(8);
        let work_per_thread = test_data.unit_count / thread_count;
        
        // Simulate parallel work with scaling efficiency
        let parallel_efficiency = 0.8; // 80% efficiency
        let sequential_work = self.get_work_amount_for_complexity(&test_data.complexity_level) * test_data.unit_count as u64;
        let parallel_work = (sequential_work as f64 / (thread_count as f64 * parallel_efficiency)) as u64;

        std::thread::sleep(Duration::from_millis(parallel_work));
        
        // Memory usage increases with thread count
        let memory_usage = self.get_memory_usage_mb() + (thread_count as f64 * 10.0);
        *peak_memory = peak_memory.max(memory_usage);

        Ok(test_data.unit_count)
    }

    /// Calculate benchmark statistics from run data
    fn calculate_statistics(&self, runs: &[BenchmarkRun], config: &BenchmarkConfig) -> Result<BenchmarkStatistics> {
        if runs.is_empty() {
            return Ok(BenchmarkStatistics::default());
        }

        let times_ms: Vec<f64> = runs.iter()
            .map(|run| run.duration.as_millis() as f64)
            .collect();

        let memory_deltas: Vec<f64> = runs.iter()
            .map(|run| run.memory_after_mb - run.memory_before_mb)
            .collect();

        let peak_memories: Vec<f64> = runs.iter()
            .map(|run| run.peak_memory_mb)
            .collect();

        // Calculate time statistics
        let mean_time_ms = times_ms.iter().sum::<f64>() / times_ms.len() as f64;
        let mut sorted_times = times_ms.clone();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median_time_ms = sorted_times[sorted_times.len() / 2];
        let min_time_ms = sorted_times[0];
        let max_time_ms = sorted_times[sorted_times.len() - 1];

        // Calculate standard deviation
        let variance = times_ms.iter()
            .map(|time| (time - mean_time_ms).powi(2))
            .sum::<f64>() / times_ms.len() as f64;
        let std_deviation_ms = variance.sqrt();

        // Calculate throughput (operations per second)
        let total_operations: usize = runs.iter().map(|run| run.operations_completed).sum();
        let total_time_secs = runs.iter().map(|run| run.duration.as_secs_f64()).sum::<f64>();
        let throughput_ops_per_sec = if total_time_secs > 0.0 {
            total_operations as f64 / total_time_secs
        } else {
            0.0
        };

        // Calculate memory statistics
        let mean_memory_delta_mb = memory_deltas.iter().sum::<f64>() / memory_deltas.len() as f64;
        let peak_memory_usage_mb = peak_memories.iter().fold(0.0, |acc, &x| acc.max(x));

        Ok(BenchmarkStatistics {
            mean_time_ms,
            median_time_ms,
            std_deviation_ms,
            min_time_ms,
            max_time_ms,
            throughput_ops_per_sec,
            mean_memory_delta_mb,
            peak_memory_usage_mb,
        })
    }

    /// Get work amount based on complexity level
    fn get_work_amount_for_complexity(&self, complexity: &ComplexityLevel) -> u64 {
        match complexity {
            ComplexityLevel::Simple => 10,
            ComplexityLevel::Medium => 25,
            ComplexityLevel::Complex => 50,
            ComplexityLevel::Extreme => 100,
        }
    }

    /// Get current memory usage (simulated)
    fn get_memory_usage_mb(&self) -> f64 {
        100.0 + (rand::random::<f64>() * 20.0)
    }

    /// Get benchmark history for a specific benchmark name
    pub fn get_benchmark_history(&self, name: &str) -> Option<&Vec<BenchmarkResults>> {
        self.benchmark_history.get(name)
    }

    /// Get baseline results for a benchmark type
    pub fn get_baseline_results(&self, benchmark_type: &BenchmarkType) -> Option<&BenchmarkStatistics> {
        self.baseline_results.get(benchmark_type)
    }

    /// Compare current results with baseline
    pub fn compare_with_baseline(&self, results: &BenchmarkResults) -> Option<f64> {
        self.baseline_results.get(&results.benchmark_type).map(|baseline| {
            if baseline.mean_time_ms > 0.0 {
                (results.statistics.mean_time_ms / baseline.mean_time_ms - 1.0) * 100.0
            } else {
                0.0
            }
        })
    }

    /// Clear benchmark history
    pub fn clear_history(&mut self) {
        info!("Clearing benchmark history");
        self.benchmark_history.clear();
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: PerformanceConfig) -> Result<()> {
        info!("Updating benchmarking engine configuration");
        self.config = new_config;
        Ok(())
    }
}

// Simple random number generation for simulation
mod rand {
    use std::cell::Cell;
    
    thread_local! {
        static RNG_STATE: Cell<u64> = Cell::new(1);
    }
    
    pub fn random<T>() -> T 
    where 
        T: From<u64>
    {
        RNG_STATE.with(|state| {
            let current = state.get();
            let next = current.wrapping_mul(1103515245).wrapping_add(12345);
            state.set(next);
            T::from(next)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmarking_engine_creation() {
        let config = PerformanceConfig::default();
        let engine = BenchmarkingEngine::new(config);
        assert!(engine.is_ok());
    }

    #[test]
    fn test_benchmark_config_creation() {
        let config = BenchmarkConfig {
            name: "test_benchmark".to_string(),
            benchmark_type: BenchmarkType::CompilationSpeed,
            iterations: 3,
            warmup_iterations: 1,
            test_data: BenchmarkTestData {
                unit_count: 10,
                complexity_level: ComplexityLevel::Simple,
                data_size_mb: 1.0,
            },
        };

        assert_eq!(config.name, "test_benchmark");
        assert_eq!(config.iterations, 3);
    }

    #[test]
    fn test_work_amount_calculation() {
        let config = PerformanceConfig::default();
        let engine = BenchmarkingEngine::new(config).unwrap();
        
        assert_eq!(engine.get_work_amount_for_complexity(&ComplexityLevel::Simple), 10);
        assert_eq!(engine.get_work_amount_for_complexity(&ComplexityLevel::Medium), 25);
        assert_eq!(engine.get_work_amount_for_complexity(&ComplexityLevel::Complex), 50);
        assert_eq!(engine.get_work_amount_for_complexity(&ComplexityLevel::Extreme), 100);
    }

    #[test]
    fn test_statistics_calculation() {
        let config = PerformanceConfig::default();
        let engine = BenchmarkingEngine::new(config).unwrap();
        
        let runs = vec![
            BenchmarkRun {
                duration: Duration::from_millis(100),
                memory_before_mb: 100.0,
                memory_after_mb: 110.0,
                peak_memory_mb: 115.0,
                operations_completed: 10,
                success: true,
                error_message: None,
            },
            BenchmarkRun {
                duration: Duration::from_millis(200),
                memory_before_mb: 100.0,
                memory_after_mb: 120.0,
                peak_memory_mb: 125.0,
                operations_completed: 10,
                success: true,
                error_message: None,
            },
        ];

        let benchmark_config = BenchmarkConfig {
            name: "test".to_string(),
            benchmark_type: BenchmarkType::CompilationSpeed,
            iterations: 2,
            warmup_iterations: 0,
            test_data: BenchmarkTestData {
                unit_count: 10,
                complexity_level: ComplexityLevel::Simple,
                data_size_mb: 1.0,
            },
        };

        let stats = engine.calculate_statistics(&runs, &benchmark_config).unwrap();
        assert_eq!(stats.mean_time_ms, 150.0);
        assert_eq!(stats.min_time_ms, 100.0);
        assert_eq!(stats.max_time_ms, 200.0);
    }
}
