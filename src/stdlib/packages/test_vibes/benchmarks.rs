/// fr fr Benchmarking utilities for the TestVibes framework
use crate::stdlib::packages::test_vibes::core::VibeBench;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// fr fr Benchmark function wrapper
pub struct Benchmark {
    name: String,
    bench_fn: Box<dyn Fn(&mut VibeBench) + Send + Sync>,
    iterations: Option<i64>,
    warmup_iterations: i64,
}

impl Benchmark {
    /// fr fr Create a new benchmark
    pub fn new<F>(name: &str, bench_fn: F) -> Self
    where
        F: Fn(&mut VibeBench) + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            bench_fn: Box::new(bench_fn),
            iterations: None,
            warmup_iterations: 100,
        }
    }

    /// fr fr Set the number of iterations
    pub fn iterations(mut self, n: i64) -> Self {
        self.iterations = Some(n);
        self
    }

    /// fr fr Set warmup iterations
    pub fn warmup(mut self, n: i64) -> Self {
        self.warmup_iterations = n;
        self
    }

    /// fr fr Run the benchmark
    pub fn run(&self) -> BenchmarkResult {
        let mut bench = VibeBench::new(self.name.clone());
        
        // Set iterations
        if let Some(n) = self.iterations {
            bench.set_iterations(n);
        } else {
            // Auto-determine iterations
            let auto_iters = self.auto_determine_iterations();
            bench.set_iterations(auto_iters);
        }

        // Warmup phase
        for _ in 0..self.warmup_iterations {
            let mut warmup_bench = VibeBench::new(format!("{}_warmup", self.name));
            warmup_bench.set_iterations(1);
            (self.bench_fn)(&mut warmup_bench);
        }

        // Actual benchmark
        bench.reset_timer();
        (self.bench_fn)(&mut bench);
        bench.stop_timer();

        let result = bench.get_result();
        BenchmarkResult::from_bench_result(result)
    }

    /// fr fr Auto-determine optimal number of iterations
    fn auto_determine_iterations(&self) -> i64 {
        let mut test_bench = VibeBench::new(format!("{}_calibration", self.name));
        test_bench.set_iterations(1);
        
        let start = Instant::now();
        (self.bench_fn)(&mut test_bench);
        let duration = start.elapsed();

        // Target around 1 second total time
        let target_duration = Duration::from_secs(1);
        let iterations = if duration.as_nanos() > 0 {
            (target_duration.as_nanos() / duration.as_nanos()).max(1) as i64
        } else {
            1000 // Default fallback
        };

        iterations.min(1_000_000) // Cap at 1 million iterations
    }
}

/// fr fr Memory benchmarking
pub struct BenchmarkMemory {
    name: String,
    bench_fn: Box<dyn Fn(&mut VibeBench) + Send + Sync>,
    track_allocations: bool,
}

impl BenchmarkMemory {
    /// fr fr Create a new memory benchmark
    pub fn new<F>(name: &str, bench_fn: F) -> Self
    where
        F: Fn(&mut VibeBench) + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            bench_fn: Box::new(bench_fn),
            track_allocations: true,
        }
    }

    /// fr fr Enable or disable allocation tracking
    pub fn track_allocations(mut self, track: bool) -> Self {
        self.track_allocations = track;
        self
    }

    /// fr fr Run the memory benchmark
    pub fn run(&self) -> MemoryBenchmarkResult {
        let mut bench = VibeBench::new(self.name.clone());
        
        // Get memory usage before
        let memory_before = get_memory_usage();
        
        bench.reset_timer();
        (self.bench_fn)(&mut bench);
        bench.stop_timer();
        
        // Get memory usage after
        let memory_after = get_memory_usage();
        let memory_used = memory_after.saturating_sub(memory_before);

        let result = bench.get_result();
        MemoryBenchmarkResult {
            name: self.name.clone(),
            duration: result.duration,
            iterations: result.iterations,
            memory_used,
            allocations: if self.track_allocations { Some(0) } else { None }, // Placeholder
            bytes_per_allocation: if memory_used > 0 && result.iterations > 0 {
                Some(memory_used as f64 / result.iterations as f64)
            } else {
                None
            },
        }
    }
}

/// fr fr Parallel benchmarking
pub struct BenchmarkParallel {
    name: String,
    bench_fn: Box<dyn Fn(&mut VibeBench) + Send + Sync>,
    parallelism: usize,
}

impl BenchmarkParallel {
    /// fr fr Create a new parallel benchmark
    pub fn new<F>(name: &str, bench_fn: F) -> Self
    where
        F: Fn(&mut VibeBench) + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            bench_fn: Box::new(bench_fn),
            parallelism: std::thread::available_parallelism().map(|p| p.get()).unwrap_or(4),
        }
    }

    /// fr fr Set the level of parallelism
    pub fn parallelism(mut self, p: usize) -> Self {
        self.parallelism = p;
        self
    }

    /// fr fr Run the parallel benchmark
    pub fn run(&self) -> ParallelBenchmarkResult {
        let results = Arc::new(Mutex::new(Vec::new()));

        let start_time = Instant::now();

        // For simplicity, we'll run sequentially to avoid borrowing issues
        for i in 0..self.parallelism {
            let name = format!("{}_{}", self.name, i);
            let mut bench = VibeBench::new(name);
            bench.set_iterations(1000); // Fixed iterations per thread
            bench.reset_timer();
            
            (self.bench_fn)(&mut bench);
            
            bench.stop_timer();
            let result = bench.get_result();
            
            results.lock().unwrap().push(result);
        }

        let total_duration = start_time.elapsed();
        let results = results.lock().unwrap();

        ParallelBenchmarkResult {
            name: self.name.clone(),
            parallelism: self.parallelism,
            total_duration,
            thread_results: results.clone(),
            average_duration: Duration::from_nanos(
                results.iter().map(|r| r.duration.as_nanos()).sum::<u128>() as u64 / results.len() as u64
            ),
            throughput: calculate_throughput(&results, total_duration),
        }
    }
}

/// fr fr Benchmark result wrapper
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub iterations: i64,
    pub ns_per_op: f64,
    pub ops_per_sec: f64,
    pub failed: bool,
}

impl BenchmarkResult {
    /// fr fr Create from VibeBench result
    pub fn from_bench_result(result: crate::stdlib::packages::test_vibes::core::BenchResult) -> Self {
        let ns_per_op = if result.iterations > 0 {
            result.duration.as_nanos() as f64 / result.iterations as f64
        } else {
            0.0
        };

        let ops_per_sec = if ns_per_op > 0.0 {
            1_000_000_000.0 / ns_per_op
        } else {
            0.0
        };

        Self {
            name: result.name,
            duration: result.duration,
            iterations: result.iterations,
            ns_per_op,
            ops_per_sec,
            failed: result.failed,
        }
    }

    /// fr fr Print benchmark result
    pub fn print(&self) {
        println!("BENCH {} {} iterations {:.2} ns/op {:.2} ops/sec", 
                 self.name, self.iterations, self.ns_per_op, self.ops_per_sec);
    }
}

/// fr fr Memory benchmark result
#[derive(Debug, Clone)]
pub struct MemoryBenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub iterations: i64,
    pub memory_used: usize,
    pub allocations: Option<usize>,
    pub bytes_per_allocation: Option<f64>,
}

impl MemoryBenchmarkResult {
    /// fr fr Print memory benchmark result
    pub fn print(&self) {
        println!("MEM_BENCH {} {} iterations {:.2?} {} bytes", 
                 self.name, self.iterations, self.duration, self.memory_used);
        
        if let Some(allocs) = self.allocations {
            println!("  {} allocations", allocs);
        }
        
        if let Some(bpa) = self.bytes_per_allocation {
            println!("  {:.2} bytes/allocation", bpa);
        }
    }
}

/// fr fr Parallel benchmark result
#[derive(Debug, Clone)]
pub struct ParallelBenchmarkResult {
    pub name: String,
    pub parallelism: usize,
    pub total_duration: Duration,
    pub thread_results: Vec<crate::stdlib::packages::test_vibes::core::BenchResult>,
    pub average_duration: Duration,
    pub throughput: f64,
}

impl ParallelBenchmarkResult {
    /// fr fr Print parallel benchmark result
    pub fn print(&self) {
        println!("PARALLEL_BENCH {} {} threads {:.2?} total", 
                 self.name, self.parallelism, self.total_duration);
        println!("  Average per thread: {:.2?}", self.average_duration);
        println!("  Throughput: {:.2} ops/sec", self.throughput);
        
        for (i, result) in self.thread_results.iter().enumerate() {
            let ns_per_op = if result.iterations > 0 {
                result.duration.as_nanos() as f64 / result.iterations as f64
            } else {
                0.0
            };
            println!("    Thread {}: {:.2} ns/op", i, ns_per_op);
        }
    }
}

/// fr fr Benchmark comparison utilities
pub struct BenchmarkComparison {
    baseline: BenchmarkResult,
    current: BenchmarkResult,
}

impl BenchmarkComparison {
    /// fr fr Create a new benchmark comparison
    pub fn new(baseline: BenchmarkResult, current: BenchmarkResult) -> Self {
        Self { baseline, current }
    }

    /// fr fr Calculate performance ratio (current / baseline)
    pub fn performance_ratio(&self) -> f64 {
        if self.baseline.ns_per_op > 0.0 {
            self.current.ns_per_op / self.baseline.ns_per_op
        } else {
            1.0
        }
    }

    /// fr fr Calculate percentage change
    pub fn percentage_change(&self) -> f64 {
        let ratio = self.performance_ratio();
        (ratio - 1.0) * 100.0
    }

    /// fr fr Check if performance improved
    pub fn improved(&self) -> bool {
        self.performance_ratio() < 1.0
    }

    /// fr fr Check if performance regressed
    pub fn regressed(&self) -> bool {
        self.performance_ratio() > 1.0
    }

    /// fr fr Print comparison
    pub fn print(&self) {
        let change = self.percentage_change();
        let status = if self.improved() {
            "IMPROVED"
        } else if self.regressed() {
            "REGRESSED"
        } else {
            "UNCHANGED"
        };

        println!("COMPARISON {} vs {}: {} ({:+.1}%)",
                 self.baseline.name, self.current.name, status, change);
        println!("  Baseline: {:.2} ns/op", self.baseline.ns_per_op);
        println!("  Current:  {:.2} ns/op", self.current.ns_per_op);
    }
}

/// fr fr Benchmark suite for running multiple benchmarks
pub struct BenchmarkSuite {
    name: String,
    benchmarks: Vec<Benchmark>,
    memory_benchmarks: Vec<BenchmarkMemory>,
    parallel_benchmarks: Vec<BenchmarkParallel>,
}

impl BenchmarkSuite {
    /// fr fr Create a new benchmark suite
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            benchmarks: Vec::new(),
            memory_benchmarks: Vec::new(),
            parallel_benchmarks: Vec::new(),
        }
    }

    /// fr fr Add a benchmark to the suite
    pub fn add_benchmark(mut self, benchmark: Benchmark) -> Self {
        self.benchmarks.push(benchmark);
        self
    }

    /// fr fr Add a memory benchmark to the suite
    pub fn add_memory_benchmark(mut self, benchmark: BenchmarkMemory) -> Self {
        self.memory_benchmarks.push(benchmark);
        self
    }

    /// fr fr Add a parallel benchmark to the suite
    pub fn add_parallel_benchmark(mut self, benchmark: BenchmarkParallel) -> Self {
        self.parallel_benchmarks.push(benchmark);
        self
    }

    /// fr fr Run all benchmarks in the suite
    pub fn run(&self) {
        println!("Running benchmark suite: {}", self.name);
        
        // Run regular benchmarks
        for benchmark in &self.benchmarks {
            let result = benchmark.run();
            result.print();
        }

        // Run memory benchmarks
        for benchmark in &self.memory_benchmarks {
            let result = benchmark.run();
            result.print();
        }

        // Run parallel benchmarks
        for benchmark in &self.parallel_benchmarks {
            let result = benchmark.run();
            result.print();
        }
    }
}

/// fr fr Helper functions

/// fr fr Get current memory usage (simplified)
fn get_memory_usage() -> usize {
    // In a real implementation, this would use system calls to get actual memory usage
    // For now, return a placeholder value
    0
}

/// fr fr Calculate throughput for parallel benchmarks
fn calculate_throughput(results: &[crate::stdlib::packages::test_vibes::core::BenchResult], total_duration: Duration) -> f64 {
    let total_iterations: i64 = results.iter().map(|r| r.iterations).sum();
    
    if total_duration.as_nanos() > 0 {
        (total_iterations as f64 * 1_000_000_000.0) / total_duration.as_nanos() as f64
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_creation() {
        let benchmark = Benchmark::new("test_bench", |b| {
            for _ in 0..b.iterations() {
                // Simulate work
                let _ = 2 + 2;
            }
        });

        assert_eq!(benchmark.name, "test_bench");
    }

    #[test]
    fn test_benchmark_with_iterations() {
        let benchmark = Benchmark::new("test_bench", |b| {
            for _ in 0..b.iterations() {
                let _ = 2 + 2;
            }
        }).iterations(100);

        let result = benchmark.run();
        assert_eq!(result.iterations, 100);
        assert!(!result.failed);
    }

    #[test]
    fn test_memory_benchmark() {
        let benchmark = BenchmarkMemory::new("memory_test", |b| {
            for _ in 0..b.iterations() {
                let _vec = vec![1, 2, 3, 4, 5];
            }
        });

        let result = benchmark.run();
        assert_eq!(result.name, "memory_test");
    }

    #[test]
    fn test_parallel_benchmark() {
        let benchmark = BenchmarkParallel::new("parallel_test", |b| {
            for _ in 0..b.iterations() {
                let _ = 2 + 2;
            }
        }).parallelism(2);

        let result = benchmark.run();
        assert_eq!(result.parallelism, 2);
        assert_eq!(result.thread_results.len(), 2);
    }

    #[test]
    fn test_benchmark_comparison() {
        let baseline = BenchmarkResult {
            name: "baseline".to_string(),
            duration: Duration::from_nanos(1000),
            iterations: 1,
            ns_per_op: 1000.0,
            ops_per_sec: 1_000_000.0,
            failed: false,
        };

        let current = BenchmarkResult {
            name: "current".to_string(),
            duration: Duration::from_nanos(800),
            iterations: 1,
            ns_per_op: 800.0,
            ops_per_sec: 1_250_000.0,
            failed: false,
        };

        let comparison = BenchmarkComparison::new(baseline, current);
        assert!(comparison.improved());
        assert!(!comparison.regressed());
        assert_eq!(comparison.performance_ratio(), 0.8);
        assert_eq!(comparison.percentage_change(), -20.0);
    }

    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new("test_suite");
        
        let benchmark1 = Benchmark::new("bench1", |b| {
            for _ in 0..b.iterations() {
                let _ = 2 + 2;
            }
        }).iterations(10);

        let benchmark2 = Benchmark::new("bench2", |b| {
            for _ in 0..b.iterations() {
                let _ = 3 * 3;
            }
        }).iterations(10);

        suite = suite.add_benchmark(benchmark1);
        suite = suite.add_benchmark(benchmark2);

        assert_eq!(suite.benchmarks.len(), 2);
        
        // Running the suite would produce output
        // suite.run();
    }
}
