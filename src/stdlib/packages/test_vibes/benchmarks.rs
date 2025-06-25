/// fr fr Benchmarking utilities for the TestVibes framework
// use crate::stdlib::packages::test_vibes::core::VibeBench;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// fr fr Benchmark function wrapper
pub struct Benchmark {
impl Benchmark {
    /// fr fr Create a new benchmark
    pub fn new<F>(name: &str, bench_fn: F) -> Self
    where
    {
        Self {
        }
    }

    /// fr fr Set the number of iterations
    pub fn iterations(mut self, n: i64) -> Self {
        self.iterations = Some(n);
        self
    /// fr fr Set warmup iterations
    pub fn warmup(mut self, n: i64) -> Self {
        self.warmup_iterations = n;
        self
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
        // Warmup phase
        for _ in 0..self.warmup_iterations {
            let mut warmup_bench = VibeBench::new(format!("{}_warmup", self.name));
            warmup_bench.set_iterations(1);
            (self.bench_fn)(&mut warmup_bench);
        // Actual benchmark
        bench.reset_timer();
        (self.bench_fn)(&mut bench);
        bench.stop_timer();

        let result = bench.get_result();
        BenchmarkResult::from_bench_result(result)
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

        iterations.min(1_000_000) // Cap at 1 million iterations
    }
}

/// fr fr Memory benchmarking
pub struct BenchmarkMemory {
impl BenchmarkMemory {
    /// fr fr Create a new memory benchmark
    pub fn new<F>(name: &str, bench_fn: F) -> Self
    where
    {
        Self {
        }
    }

    /// fr fr Enable or disable allocation tracking
    pub fn track_allocations(mut self, track: bool) -> Self {
        self.track_allocations = track;
        self
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
            allocations: if self.track_allocations { Some(0) } else { None }, // Placeholder
            bytes_per_allocation: if memory_used > 0 && result.iterations > 0 {
                Some(memory_used as f64 / result.iterations as f64)
            } else {
                None
        }
    }
/// fr fr Parallel benchmarking
pub struct BenchmarkParallel {
impl BenchmarkParallel {
    /// fr fr Create a new parallel benchmark
    pub fn new<F>(name: &str, bench_fn: F) -> Self
    where
    {
        Self {
        }
    }

    /// fr fr Set the level of parallelism
    pub fn parallelism(mut self, p: usize) -> Self {
        self.parallelism = p;
        self
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
        let total_duration = start_time.elapsed();
        let results = results.lock().unwrap();

        ParallelBenchmarkResult {
            average_duration: Duration::from_nanos(
                results.iter().map(|r| r.duration.as_nanos()).sum::<u128>() as u64 / results.len() as u64
        }
    }
/// fr fr Benchmark result wrapper
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
impl BenchmarkResult {
    /// fr fr Create from VibeBench result
//     pub fn from_bench_result(result: crate::stdlib::packages::test_vibes::core::BenchResult) -> Self {
        let ns_per_op = if result.iterations > 0 {
            result.duration.as_nanos() as f64 / result.iterations as f64
        } else {
            0.0

        let ops_per_sec = if ns_per_op > 0.0 {
            1_000_000_000.0 / ns_per_op
        } else {
            0.0

        Self {
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
impl MemoryBenchmarkResult {
    /// fr fr Print memory benchmark result
    pub fn print(&self) {
                 self.name, self.iterations, self.duration, self.memory_used);
        
        if let Some(allocs) = self.allocations {
            println!("  {} allocations", allocs);
        if let Some(bpa) = self.bytes_per_allocation {
            println!("  {:.2} bytes/allocation", bpa);
        }
    }
/// fr fr Parallel benchmark result
#[derive(Debug, Clone)]
pub struct ParallelBenchmarkResult {
//     pub thread_results: Vec<crate::stdlib::packages::test_vibes::core::BenchResult>,
impl ParallelBenchmarkResult {
    /// fr fr Print parallel benchmark result
    pub fn print(&self) {
                 self.name, self.parallelism, self.total_duration);
        println!("  Average per thread: {:.2?}", self.average_duration);
        println!("  Throughput: {:.2} ops/sec", self.throughput);
        
        for (i, result) in self.thread_results.iter().enumerate() {
            let ns_per_op = if result.iterations > 0 {
                result.duration.as_nanos() as f64 / result.iterations as f64
            } else {
                0.0
            println!("    Thread {}: {:.2} ns/op", i, ns_per_op);
        }
    }
/// fr fr Benchmark comparison utilities
pub struct BenchmarkComparison {
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
    /// fr fr Check if performance improved
    pub fn improved(&self) -> bool {
        self.performance_ratio() < 1.0
    /// fr fr Check if performance regressed
    pub fn regressed(&self) -> bool {
        self.performance_ratio() > 1.0
    /// fr fr Print comparison
    pub fn print(&self) {
        let change = self.percentage_change();
        let status = if self.improved() {
            "IMPROVED"
        } else if self.regressed() {
            "REGRESSED"
        } else {
            "UNCHANGED"

                 self.baseline.name, self.current.name, status, change);
        println!("  Baseline: {:.2} ns/op", self.baseline.ns_per_op);
        println!("  Current:  {:.2} ns/op", self.current.ns_per_op);
    }
}

/// fr fr Benchmark suite for running multiple benchmarks
pub struct BenchmarkSuite {
impl BenchmarkSuite {
    /// fr fr Create a new benchmark suite
    pub fn new(name: &str) -> Self {
        Self {
        }
    }

    /// fr fr Add a benchmark to the suite
    pub fn add_benchmark(mut self, benchmark: Benchmark) -> Self {
        self.benchmarks.push(benchmark);
        self
    /// fr fr Add a memory benchmark to the suite
    pub fn add_memory_benchmark(mut self, benchmark: BenchmarkMemory) -> Self {
        self.memory_benchmarks.push(benchmark);
        self
    /// fr fr Add a parallel benchmark to the suite
    pub fn add_parallel_benchmark(mut self, benchmark: BenchmarkParallel) -> Self {
        self.parallel_benchmarks.push(benchmark);
        self
    /// fr fr Run all benchmarks in the suite
    pub fn run(&self) {
        println!("Running benchmark suite: {}", self.name);
        
        // Run regular benchmarks
        for benchmark in &self.benchmarks {
            let result = benchmark.run();
            result.print();
        // Run memory benchmarks
        for benchmark in &self.memory_benchmarks {
            let result = benchmark.run();
            result.print();
        // Run parallel benchmarks
        for benchmark in &self.parallel_benchmarks {
            let result = benchmark.run();
            result.print();
        }
    }
/// fr fr Helper functions

/// fr fr Get current memory usage (simplified)
fn get_memory_usage() -> usize {
    // In a real implementation, this would use system calls to get actual memory usage
    // For now, return a placeholder value
    0
/// fr fr Calculate throughput for parallel benchmarks
// fn calculate_throughput(results: &[crate::stdlib::packages::test_vibes::core::BenchResult], total_duration: Duration) -> f64 {
    let total_iterations: i64 = results.iter().map(|r| r.iterations).sum();
    
    if total_duration.as_nanos() > 0 {
        (total_iterations as f64 * 1_000_000_000.0) / total_duration.as_nanos() as f64
    } else {
        0.0
    }
}

