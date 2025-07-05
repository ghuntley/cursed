//! Benchmarking module for CURSED optimization system

use crate::error::CursedError;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Results from benchmark execution
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    /// Name of the benchmark
    pub name: String,
    /// Total execution time
    pub execution_time: Duration,
    /// Number of iterations performed
    pub iterations: usize,
    /// Average time per iteration
    pub avg_time_per_iteration: Duration,
    /// Average time (alias for avg_time_per_iteration)
    pub average_time: Duration,
    /// Standard deviation of iteration times
    pub std_deviation: Duration,
    /// Minimum iteration time
    pub min_time: Duration,
    /// Maximum iteration time
    pub max_time: Duration,
    /// Throughput (operations per second)
    pub throughput: f64,
    /// Memory usage during benchmark
    pub memory_usage: usize,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
    /// Compilation metrics
    pub compilation_metrics: Option<CompilationMetrics>,
}

/// Compilation-specific metrics
#[derive(Debug, Clone)]
pub struct CompilationMetrics {
    /// Time spent in parsing
    pub parse_time: Duration,
    /// Time spent in semantic analysis
    pub semantic_time: Duration,
    /// Time spent in optimization
    pub optimization_time: Duration,
    /// Time spent in code generation
    pub codegen_time: Duration,
    /// Time spent in linking
    pub link_time: Duration,
    /// Number of functions compiled
    pub functions_compiled: usize,
    /// Lines of code compiled
    pub lines_compiled: usize,
    /// Size of generated code
    pub generated_code_size: usize,
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of iterations to run
    pub iterations: usize,
    /// Warmup iterations (not counted in results)
    pub warmup_iterations: usize,
    /// Timeout for entire benchmark
    pub timeout: Duration,
    /// Timeout for single iteration
    pub iteration_timeout: Duration,
    /// Whether to collect memory usage
    pub collect_memory: bool,
    /// Whether to collect CPU usage
    pub collect_cpu: bool,
    /// Custom configuration options
    pub custom_options: HashMap<String, String>,
}

/// Benchmark runner
#[derive(Debug)]
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

impl BenchmarkResults {
    /// Create new benchmark results
    pub fn new(name: String) -> Self {
        Self {
            name,
            execution_time: Duration::from_secs(0),
            iterations: 0,
            avg_time_per_iteration: Duration::from_secs(0),
            average_time: Duration::from_secs(0),
            std_deviation: Duration::from_secs(0),
            min_time: Duration::from_secs(0),
            max_time: Duration::from_secs(0),
            throughput: 0.0,
            memory_usage: 0,
            cpu_usage: 0.0,
            custom_metrics: HashMap::new(),
            compilation_metrics: None,
        }
    }

    /// Calculate statistics from iteration times
    pub fn calculate_stats(&mut self, iteration_times: &[Duration]) {
        if iteration_times.is_empty() {
            return;
        }

        self.iterations = iteration_times.len();
        self.execution_time = iteration_times.iter().sum();
        self.avg_time_per_iteration = self.execution_time / self.iterations as u32;
        self.average_time = self.avg_time_per_iteration;
        self.min_time = *iteration_times.iter().min().unwrap();
        self.max_time = *iteration_times.iter().max().unwrap();
        
        // Calculate standard deviation
        let avg_nanos = self.avg_time_per_iteration.as_nanos() as f64;
        let variance: f64 = iteration_times.iter()
            .map(|&time| {
                let diff = time.as_nanos() as f64 - avg_nanos;
                diff * diff
            })
            .sum::<f64>() / self.iterations as f64;
        
        self.std_deviation = Duration::from_nanos(variance.sqrt() as u64);
        
        // Calculate throughput (iterations per second)
        self.throughput = self.iterations as f64 / self.execution_time.as_secs_f64();
    }

    /// Add custom metric
    pub fn add_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
    }

    /// Get metric value
    pub fn get_metric(&self, name: &str) -> Option<f64> {
        self.custom_metrics.get(name).copied()
    }

    /// Set compilation metrics
    pub fn set_compilation_metrics(&mut self, metrics: CompilationMetrics) {
        self.compilation_metrics = Some(metrics);
    }

    /// Get efficiency score (higher is better)
    pub fn efficiency_score(&self) -> f64 {
        if self.execution_time.as_secs_f64() == 0.0 {
            return 0.0;
        }
        
        // Base score from throughput
        let mut score = self.throughput;
        
        // Adjust for consistency (lower std deviation is better)
        if self.std_deviation.as_secs_f64() > 0.0 {
            let consistency_factor = 1.0 / (1.0 + self.std_deviation.as_secs_f64());
            score *= consistency_factor;
        }
        
        // Adjust for memory efficiency (if available)
        if self.memory_usage > 0 {
            let memory_factor = 1.0 / (1.0 + (self.memory_usage as f64 / 1024.0 / 1024.0)); // MB
            score *= memory_factor;
        }
        
        score
    }

    /// Format results as string
    pub fn format_summary(&self) -> String {
        format!(
            "Benchmark: {}\n\
            Iterations: {}\n\
            Total Time: {:.3}s\n\
            Avg Time: {:.3}ms\n\
            Min Time: {:.3}ms\n\
            Max Time: {:.3}ms\n\
            Std Dev: {:.3}ms\n\
            Throughput: {:.2} ops/sec\n\
            Memory Usage: {:.2} MB\n\
            CPU Usage: {:.1}%\n\
            Efficiency Score: {:.2}",
            self.name,
            self.iterations,
            self.execution_time.as_secs_f64(),
            self.avg_time_per_iteration.as_secs_f64() * 1000.0,
            self.min_time.as_secs_f64() * 1000.0,
            self.max_time.as_secs_f64() * 1000.0,
            self.std_deviation.as_secs_f64() * 1000.0,
            self.throughput,
            self.memory_usage as f64 / 1024.0 / 1024.0,
            self.cpu_usage,
            self.efficiency_score()
        )
    }
}

impl CompilationMetrics {
    /// Create new compilation metrics
    pub fn new() -> Self {
        Self {
            parse_time: Duration::from_secs(0),
            semantic_time: Duration::from_secs(0),
            optimization_time: Duration::from_secs(0),
            codegen_time: Duration::from_secs(0),
            link_time: Duration::from_secs(0),
            functions_compiled: 0,
            lines_compiled: 0,
            generated_code_size: 0,
        }
    }

    /// Get total compilation time
    pub fn total_time(&self) -> Duration {
        self.parse_time + self.semantic_time + self.optimization_time + self.codegen_time + self.link_time
    }

    /// Get compilation rate (lines per second)
    pub fn compilation_rate(&self) -> f64 {
        if self.total_time().as_secs_f64() == 0.0 {
            return 0.0;
        }
        self.lines_compiled as f64 / self.total_time().as_secs_f64()
    }

    /// Get code generation efficiency (bytes per second)
    pub fn codegen_efficiency(&self) -> f64 {
        if self.codegen_time.as_secs_f64() == 0.0 {
            return 0.0;
        }
        self.generated_code_size as f64 / self.codegen_time.as_secs_f64()
    }
}

impl BenchmarkConfig {
    /// Create new benchmark configuration
    pub fn new() -> Self {
        Self {
            iterations: 100,
            warmup_iterations: 10,
            timeout: Duration::from_secs(300), // 5 minutes
            iteration_timeout: Duration::from_secs(10), // 10 seconds per iteration
            collect_memory: true,
            collect_cpu: true,
            custom_options: HashMap::new(),
        }
    }

    /// Set number of iterations
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    /// Set warmup iterations
    pub fn with_warmup(mut self, warmup_iterations: usize) -> Self {
        self.warmup_iterations = warmup_iterations;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Add custom option
    pub fn with_option(mut self, key: String, value: String) -> Self {
        self.custom_options.insert(key, value);
        self
    }

    /// Quick configuration for fast benchmarks
    pub fn quick() -> Self {
        Self::new()
            .with_iterations(10)
            .with_warmup(2)
            .with_timeout(Duration::from_secs(30))
    }

    /// Configuration for comprehensive benchmarks
    pub fn comprehensive() -> Self {
        Self::new()
            .with_iterations(1000)
            .with_warmup(100)
            .with_timeout(Duration::from_secs(1800)) // 30 minutes
    }
}

impl BenchmarkRunner {
    /// Create new benchmark runner
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// Run benchmark with given function
    pub fn run<F>(&self, name: &str, mut benchmark_fn: F) -> Result<BenchmarkResults, CursedError>
    where
        F: FnMut() -> Result<(), CursedError>,
    {
        let mut results = BenchmarkResults::new(name.to_string());
        let mut iteration_times = Vec::new();
        let total_iterations = self.config.warmup_iterations + self.config.iterations;

        let benchmark_start = Instant::now();

        for i in 0..total_iterations {
            // Check timeout
            if benchmark_start.elapsed() > self.config.timeout {
                return Err(CursedError::runtime_error("Benchmark timeout"));
            }

            let iteration_start = Instant::now();
            
            // Run benchmark function
            benchmark_fn().map_err(|e| CursedError::runtime_error(&format!("Benchmark failed: {}", e)))?;
            
            let iteration_time = iteration_start.elapsed();
            
            // Check iteration timeout
            if iteration_time > self.config.iteration_timeout {
                return Err(CursedError::runtime_error("Iteration timeout"));
            }

            // Only count non-warmup iterations
            if i >= self.config.warmup_iterations {
                iteration_times.push(iteration_time);
            }
        }

        // Calculate statistics
        results.calculate_stats(&iteration_times);

        // Mock resource usage (in real implementation, would collect actual metrics)
        if self.config.collect_memory {
            results.memory_usage = 64 * 1024 * 1024; // 64MB mock
        }
        if self.config.collect_cpu {
            results.cpu_usage = 45.0; // 45% mock
        }

        Ok(results)
    }

    /// Run compilation benchmark
    pub fn run_compilation_benchmark<F>(
        &self,
        name: &str,
        mut compile_fn: F,
    ) -> Result<BenchmarkResults, CursedError>
    where
        F: FnMut() -> Result<CompilationMetrics, CursedError>,
    {
        let mut results = BenchmarkResults::new(name.to_string());
        let mut iteration_times = Vec::new();
        let mut total_metrics = CompilationMetrics::new();

        let benchmark_start = Instant::now();

        for i in 0..(self.config.warmup_iterations + self.config.iterations) {
            // Check timeout
            if benchmark_start.elapsed() > self.config.timeout {
                return Err(CursedError::runtime_error("Benchmark timeout"));
            }

            let iteration_start = Instant::now();
            
            // Run compilation function
            let metrics = compile_fn().map_err(|e| CursedError::runtime_error(&format!("Compilation failed: {}", e)))?;
            
            let iteration_time = iteration_start.elapsed();
            
            // Only count non-warmup iterations
            if i >= self.config.warmup_iterations {
                iteration_times.push(iteration_time);
                
                // Accumulate metrics
                total_metrics.parse_time += metrics.parse_time;
                total_metrics.semantic_time += metrics.semantic_time;
                total_metrics.optimization_time += metrics.optimization_time;
                total_metrics.codegen_time += metrics.codegen_time;
                total_metrics.link_time += metrics.link_time;
                total_metrics.functions_compiled += metrics.functions_compiled;
                total_metrics.lines_compiled += metrics.lines_compiled;
                total_metrics.generated_code_size += metrics.generated_code_size;
            }
        }

        // Calculate statistics
        results.calculate_stats(&iteration_times);

        // Average the compilation metrics
        let actual_iterations = self.config.iterations as u32;
        if actual_iterations > 0 {
            total_metrics.parse_time /= actual_iterations;
            total_metrics.semantic_time /= actual_iterations;
            total_metrics.optimization_time /= actual_iterations;
            total_metrics.codegen_time /= actual_iterations;
            total_metrics.link_time /= actual_iterations;
            total_metrics.functions_compiled /= actual_iterations as usize;
            total_metrics.lines_compiled /= actual_iterations as usize;
            total_metrics.generated_code_size /= actual_iterations as usize;
        }

        results.set_compilation_metrics(total_metrics);

        // Add compilation-specific metrics
        if let Some(metrics) = results.compilation_metrics.clone() {
            results.add_metric("compilation_rate".to_string(), metrics.compilation_rate());
            results.add_metric("codegen_efficiency".to_string(), metrics.codegen_efficiency());
        }

        Ok(results)
    }

    /// Get configuration
    pub fn get_config(&self) -> &BenchmarkConfig {
        &self.config
    }
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CompilationMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Run quick benchmark
pub fn quick_benchmark<F>(name: &str, benchmark_fn: F) -> Result<BenchmarkResults, CursedError>
where
    F: FnMut() -> Result<(), CursedError>,
{
    let config = BenchmarkConfig::quick();
    let runner = BenchmarkRunner::new(config);
    runner.run(name, benchmark_fn)
}

/// Run comprehensive benchmark
pub fn comprehensive_benchmark<F>(name: &str, benchmark_fn: F) -> Result<BenchmarkResults, CursedError>
where
    F: FnMut() -> Result<(), CursedError>,
{
    let config = BenchmarkConfig::comprehensive();
    let runner = BenchmarkRunner::new(config);
    runner.run(name, benchmark_fn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_benchmark_results_creation() {
        let results = BenchmarkResults::new("test".to_string());
        assert_eq!(results.name, "test");
        assert_eq!(results.iterations, 0);
    }

    #[test]
    fn test_benchmark_config_creation() {
        let config = BenchmarkConfig::new();
        assert_eq!(config.iterations, 100);
        assert_eq!(config.warmup_iterations, 10);
    }

    #[test]
    fn test_benchmark_runner() {
        let config = BenchmarkConfig::new().with_iterations(5).with_warmup(1);
        let runner = BenchmarkRunner::new(config);
        
        let result = runner.run("test_benchmark", || {
            thread::sleep(Duration::from_millis(1));
            Ok(())
        });
        
        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.iterations, 5);
        assert!(results.execution_time.as_millis() >= 5);
    }

    #[test]
    fn test_quick_benchmark() {
        let result = quick_benchmark("quick_test", || {
            thread::sleep(Duration::from_millis(1));
            Ok(())
        });
        
        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.name, "quick_test");
    }

    #[test]
    fn test_compilation_metrics() {
        let mut metrics = CompilationMetrics::new();
        metrics.parse_time = Duration::from_millis(100);
        metrics.semantic_time = Duration::from_millis(200);
        metrics.optimization_time = Duration::from_millis(300);
        metrics.codegen_time = Duration::from_millis(400);
        metrics.link_time = Duration::from_millis(100);
        metrics.lines_compiled = 1000;
        metrics.generated_code_size = 50000;
        
        assert_eq!(metrics.total_time(), Duration::from_millis(1100));
        assert!(metrics.compilation_rate() > 0.0);
        assert!(metrics.codegen_efficiency() > 0.0);
    }
}
