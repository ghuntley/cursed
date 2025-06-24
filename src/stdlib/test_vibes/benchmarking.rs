/// Benchmarking utilities for the TestVibes framework
/// 
/// Provides functions for performance testing and measurement

use crate::stdlib::value::Value;
use crate::error::Error;
use super::{VibeBench, TestVibesResult};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

/// Define a benchmark function
pub fn Benchmark<F>(name: &str, f: F) -> TestVibesResult<super::BenchmarkResult>
where
    F: Fn(&mut VibeBench) -> TestVibesResult<()>,
{
    // Determine optimal number of iterations
    let iterations = determine_iterations(name, &f)?;
    
    // Create benchmark instance
    let mut bench = VibeBench::new(name, iterations);
    bench.ResetTimer()?;
    
    // Run the benchmark
    bench.StartTimer()?;
    f(&mut bench)?;
    bench.StopTimer()?;
    
    Ok(bench.result())
}

/// Benchmark memory usage
pub fn BenchmarkMemory<F>(name: &str, f: F) -> TestVibesResult<super::BenchmarkResult>
where
    F: Fn(&mut VibeBench) -> TestVibesResult<()>,
{
    // Create benchmark with memory tracking
    let mut bench = VibeBench::new(name, 1000);
    
    // In a real implementation, would capture memory stats before/after
    let initial_memory = get_memory_usage();
    
    bench.ResetTimer()?;
    bench.StartTimer()?;
    f(&mut bench)?;
    bench.StopTimer()?;
    
    let final_memory = get_memory_usage();
    let memory_per_op = if bench.N > 0 {
        (final_memory - initial_memory) / bench.N
    } else {
        0
    };
    
    bench.ReportMetric(memory_per_op as f64, "bytes_per_op")?;
    
    let mut result = bench.result();
    result.bytes_per_op = Some(memory_per_op);
    
    Ok(result)
}

/// Parallel benchmark execution
pub fn BenchmarkParallel<F>(name: &str, parallelism: usize, f: F) -> TestVibesResult<super::BenchmarkResult>
where
    F: Fn(&mut VibeBench) -> TestVibesResult<()> + Send + Sync + Clone + 'static,
{
    let iterations_per_worker = 1000 / parallelism.max(1);
    let start_time = Instant::now();
    
    // Create worker threads
    let handles: Vec<_> = (0..parallelism).map(|i| {
        let f_clone = f.clone();
        let worker_name = format!("{}_{}", name, i);
        
        thread::spawn(move || {
            let mut bench = VibeBench::new(&worker_name, iterations_per_worker);
            bench.ResetTimer().unwrap();
            bench.StartTimer().unwrap();
            f_clone(&mut bench).unwrap();
            bench.StopTimer().unwrap();
            bench.result()
        })
    }).collect();
    
    // Collect results
    let mut total_iterations = 0;
    let mut total_duration = Duration::new(0, 0);
    let mut worker_results = Vec::new();
    
    for handle in handles {
        match handle.join() {
            Ok(result) => {
                total_iterations += result.iterations;
                total_duration += result.duration;
                worker_results.push(result);
            }
            Err(_) => {
                return Err(super::TestVibesError::BenchmarkFailed("Worker thread panicked".to_string()).into());
            }
        }
    }
    
    let total_elapsed = start_time.elapsed();
    let average_ns_per_op = if total_iterations > 0 {
        total_duration.as_nanos() as f64 / total_iterations as f64
    } else {
        0.0
    };
    
    Ok(super::BenchmarkResult {
        name: name.to_string(),
        state: super::VibeBenchState::Completed,
        iterations: total_iterations,
        duration: total_elapsed,
        ns_per_op: average_ns_per_op,
        bytes_per_op: None,
        allocations: None,
        custom_metrics: std::collections::HashMap::new(),
    })
}

/// Benchmark suite for running multiple benchmarks
pub struct BenchmarkSuite {
    name: String,
    benchmarks: Vec<Box<dyn Fn() -> TestVibesResult<super::BenchmarkResult> + Send + Sync>>,
    setup: Option<Box<dyn Fn() -> TestVibesResult<()> + Send + Sync>>,
    teardown: Option<Box<dyn Fn() -> TestVibesResult<()> + Send + Sync>>,
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            benchmarks: Vec::new(),
            setup: None,
            teardown: None,
        }
    }

    /// Add a benchmark to the suite
    pub fn add_benchmark<F>(mut self, name: &str, f: F) -> Self
    where
        F: Fn(&mut VibeBench) -> TestVibesResult<()> + Send + Sync + 'static,
    {
        let benchmark_name = name.to_string();
        self.benchmarks.push(Box::new(move || {
            Benchmark(&benchmark_name, &f)
        }));
        self
    }

    /// Add setup function
    pub fn with_setup<F>(mut self, setup: F) -> Self
    where
        F: Fn() -> TestVibesResult<()> + Send + Sync + 'static,
    {
        self.setup = Some(Box::new(setup));
        self
    }

    /// Add teardown function
    pub fn with_teardown<F>(mut self, teardown: F) -> Self
    where
        F: Fn() -> TestVibesResult<()> + Send + Sync + 'static,
    {
        self.teardown = Some(Box::new(teardown));
        self
    }

    /// Run all benchmarks in the suite
    pub fn run(self) -> TestVibesResult<BenchmarkSuiteResult> {
        println!("Running benchmark suite: {}", self.name);
        
        // Run setup
        if let Some(setup) = &self.setup {
            setup()?;
        }
        
        let mut results = Vec::new();
        let suite_start = Instant::now();
        
        // Run benchmarks
        for (i, benchmark) in self.benchmarks.iter().enumerate() {
            println!("Running benchmark {}/{}", i + 1, self.benchmarks.len());
            
            match benchmark() {
                Ok(result) => {
                    println!("  {}: {} iterations, {:.2} ns/op", 
                             result.name, result.iterations, result.ns_per_op);
                    results.push(result);
                }
                Err(e) => {
                    println!("  Benchmark failed: {}", e);
                    return Err(e);
                }
            }
        }
        
        let suite_duration = suite_start.elapsed();
        
        // Run teardown
        if let Some(teardown) = &self.teardown {
            teardown()?;
        }
        
        Ok(BenchmarkSuiteResult {
            suite_name: self.name,
            results,
            total_duration: suite_duration,
        })
    }
}

/// Results from running a benchmark suite
#[derive(Debug)]
pub struct BenchmarkSuiteResult {
    pub suite_name: String,
    pub results: Vec<super::BenchmarkResult>,
    pub total_duration: Duration,
}

impl BenchmarkSuiteResult {
    /// Print a formatted report
    pub fn print_report(&self) {
        println!("\n=== Benchmark Suite Results: {} ===", self.suite_name);
        println!("Total suite time: {:?}", self.total_duration);
        println!();
        
        // Print individual results
        for result in &self.results {
            println!("{:<30} {:>15} iterations {:>15.2} ns/op", 
                     result.name, 
                     result.iterations, 
                     result.ns_per_op);
            
            if let Some(bytes) = result.bytes_per_op {
                println!("{:<30} {:>15} bytes/op", "", bytes);
            }
            
            for (metric, value) in &result.custom_metrics {
                println!("{:<30} {:>15.2} {}", "", value, metric);
            }
        }
        
        // Print statistics
        if !self.results.is_empty() {
            let total_ops: usize = self.results.iter().map(|r| r.iterations).sum();
            let avg_ns_per_op: f64 = self.results.iter().map(|r| r.ns_per_op).sum::<f64>() / self.results.len() as f64;
            
            println!();
            println!("Suite Statistics:");
            println!("  Total operations: {}", total_ops);
            println!("  Average ns/op: {:.2}", avg_ns_per_op);
            println!("  Benchmarks run: {}", self.results.len());
        }
    }

    /// Get the fastest benchmark
    pub fn fastest(&self) -> Option<&super::BenchmarkResult> {
        self.results.iter().min_by(|a, b| a.ns_per_op.partial_cmp(&b.ns_per_op).unwrap())
    }

    /// Get the slowest benchmark
    pub fn slowest(&self) -> Option<&super::BenchmarkResult> {
        self.results.iter().max_by(|a, b| a.ns_per_op.partial_cmp(&b.ns_per_op).unwrap())
    }
}

/// Benchmark comparison utilities
pub struct BenchmarkComparison {
    baseline: super::BenchmarkResult,
    comparison: super::BenchmarkResult,
}

impl BenchmarkComparison {
    /// Create a new comparison
    pub fn new(baseline: super::BenchmarkResult, comparison: super::BenchmarkResult) -> Self {
        Self { baseline, comparison }
    }

    /// Calculate performance ratio (comparison / baseline)
    pub fn performance_ratio(&self) -> f64 {
        if self.baseline.ns_per_op > 0.0 {
            self.comparison.ns_per_op / self.baseline.ns_per_op
        } else {
            0.0
        }
    }

    /// Get percentage change
    pub fn percentage_change(&self) -> f64 {
        let ratio = self.performance_ratio();
        (ratio - 1.0) * 100.0
    }

    /// Check if comparison is faster
    pub fn is_faster(&self) -> bool {
        self.comparison.ns_per_op < self.baseline.ns_per_op
    }

    /// Print comparison report
    pub fn print_comparison(&self) {
        let ratio = self.performance_ratio();
        let percentage = self.percentage_change();
        
        println!("Benchmark Comparison:");
        println!("  Baseline: {} ({:.2} ns/op)", self.baseline.name, self.baseline.ns_per_op);
        println!("  Comparison: {} ({:.2} ns/op)", self.comparison.name, self.comparison.ns_per_op);
        println!("  Ratio: {:.2}x", ratio);
        
        if self.is_faster() {
            println!("  Result: {:.1}% faster", -percentage);
        } else {
            println!("  Result: {:.1}% slower", percentage);
        }
    }
}

// Helper functions

/// Determine optimal number of iterations for a benchmark
fn determine_iterations<F>(_name: &str, f: &F) -> TestVibesResult<usize>
where
    F: Fn(&mut VibeBench) -> TestVibesResult<()>,
{
    // Start with a small number to estimate timing
    let mut test_bench = VibeBench::new("calibration", 10);
    test_bench.ResetTimer()?;
    
    let start = Instant::now();
    f(&mut test_bench)?;
    let elapsed = start.elapsed();
    
    // Aim for benchmarks that take around 1 second total
    let target_duration = Duration::from_secs(1);
    let estimated_per_op = elapsed.as_nanos() as f64 / 10.0;
    
    if estimated_per_op > 0.0 {
        let target_ops = (target_duration.as_nanos() as f64 / estimated_per_op) as usize;
        Ok(target_ops.max(1).min(1_000_000)) // Clamp between 1 and 1M
    } else {
        Ok(1000) // Default fallback
    }
}

/// Get current memory usage (simplified)
fn get_memory_usage() -> usize {
    // In a real implementation, would use system APIs to get actual memory usage
    // For now, return a simulated value
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    
    // Simulate memory usage between 1KB and 10MB
    let base_memory = 1024 * 1024; // 1MB base
    let variation = (hasher.finish() % (9 * 1024 * 1024)) as usize; // 0-9MB variation
    
    base_memory + variation
}

// Common benchmark scenarios

/// Benchmark string operations
pub fn benchmark_string_operations() -> TestVibesResult<BenchmarkSuiteResult> {
    BenchmarkSuite::new("String Operations")
        .add_benchmark("string_creation", |b| {
            for _ in 0..b.N {
                let _s = String::from("hello world");
            }
            Ok(())
        })
        .add_benchmark("string_concatenation", |b| {
            for _ in 0..b.N {
                let _s = "hello".to_string() + " " + "world";
            }
            Ok(())
        })
        .add_benchmark("string_formatting", |b| {
            for _ in 0..b.N {
                let _s = format!("hello {}", "world");
            }
            Ok(())
        })
        .run()
}

/// Benchmark collection operations
pub fn benchmark_collection_operations() -> TestVibesResult<BenchmarkSuiteResult> {
    BenchmarkSuite::new("Collection Operations")
        .add_benchmark("vec_push", |b| {
            for _ in 0..b.N {
                let mut v = Vec::new();
                for i in 0..100 {
                    v.push(i);
                }
            }
            Ok(())
        })
        .add_benchmark("vec_clone", |b| {
            let original: Vec<i32> = (0..1000).collect();
            for _ in 0..b.N {
                let _cloned = original.clone();
            }
            Ok(())
        })
        .add_benchmark("hashmap_insert", |b| {
            use std::collections::HashMap;
            for _ in 0..b.N {
                let mut map = HashMap::new();
                for i in 0..100 {
                    map.insert(i, i * 2);
                }
            }
            Ok(())
        })
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_benchmark() {
        let result = Benchmark("test_benchmark", |b| {
            // Simple operation to benchmark
            for _ in 0..b.N {
                let _x = 2 + 2;
            }
            Ok(())
        });
        
        assert!(result.is_ok());
        let benchmark_result = result.unwrap();
        assert!(benchmark_result.iterations > 0);
        assert!(benchmark_result.ns_per_op > 0.0);
    }

    #[test]
    fn test_benchmark_suite() {
        let result = BenchmarkSuite::new("Test Suite")
            .add_benchmark("simple_math", |b| {
                for _ in 0..b.N {
                    let _x = 5 * 7;
                }
                Ok(())
            })
            .add_benchmark("string_length", |b| {
                let s = "hello world";
                for _ in 0..b.N {
                    let _len = s.len();
                }
                Ok(())
            })
            .run();
        
        assert!(result.is_ok());
        let suite_result = result.unwrap();
        assert_eq!(suite_result.results.len(), 2);
    }

    #[test]
    fn test_benchmark_comparison() {
        let baseline = super::super::BenchmarkResult {
            name: "baseline".to_string(),
            state: super::super::VibeBenchState::Completed,
            iterations: 1000,
            duration: Duration::from_nanos(1000),
            ns_per_op: 100.0,
            bytes_per_op: None,
            allocations: None,
            custom_metrics: std::collections::HashMap::new(),
        };
        
        let comparison = super::super::BenchmarkResult {
            name: "optimized".to_string(),
            state: super::super::VibeBenchState::Completed,
            iterations: 1000,
            duration: Duration::from_nanos(800),
            ns_per_op: 80.0,
            bytes_per_op: None,
            allocations: None,
            custom_metrics: std::collections::HashMap::new(),
        };
        
        let comp = BenchmarkComparison::new(baseline, comparison);
        assert!(comp.is_faster());
        assert!((comp.performance_ratio() - 0.8).abs() < 0.001);
    }
}
