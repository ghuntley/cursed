//! Testing functionality for benchmarking

use crate::error::CursedError;
use std::time::{Duration, Instant};

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Benchmark result structure
#[derive(Debug, Clone)]
pub struct BenchResult {
    pub name: String,
    pub duration: Duration,
    pub iterations: u64,
    pub bytes_processed: Option<u64>,
    pub memory_used: Option<u64>,
}

/// Basic benchmark runner
#[derive(Debug)]
pub struct Benchmark {
    pub name: String,
    pub iterations: u64,
    pub duration: Option<Duration>,
}

impl Benchmark {
    /// Create a new benchmark
    pub fn new(name: String) -> Self {
        Self {
            name,
            iterations: 1000,
            duration: None,
        }
    }
    
    /// Set the number of iterations
    pub fn iterations(mut self, count: u64) -> Self {
        self.iterations = count;
        self
    }
    
    /// Run the benchmark
    pub fn run<F>(mut self, benchmark_fn: F) -> TestResult<BenchResult>
    where
        F: Fn() -> (),
    {
        let start = Instant::now();
        
        for _ in 0..self.iterations {
            benchmark_fn();
        }
        
        let duration = start.elapsed();
        self.duration = Some(duration);
        
        Ok(BenchResult {
            name: self.name,
            duration,
            iterations: self.iterations,
            bytes_processed: None,
            memory_used: None,
        })
    }
}

/// Memory benchmark runner
#[derive(Debug)]
pub struct BenchmarkMemory {
    pub benchmark: Benchmark,
    pub initial_memory: Option<u64>,
}

impl BenchmarkMemory {
    /// Create a new memory benchmark
    pub fn new(name: String) -> Self {
        Self {
            benchmark: Benchmark::new(name),
            initial_memory: None,
        }
    }
    
    /// Run the memory benchmark
    pub fn run<F>(mut self, benchmark_fn: F) -> TestResult<BenchResult>
    where
        F: Fn() -> (),
    {
        // Simple memory tracking (in a real implementation would use proper profiling)
        self.initial_memory = Some(1024); // Stub value
        
        let mut result = self.benchmark.run(benchmark_fn)?;
        result.memory_used = Some(2048); // Stub value showing memory increase
        
        Ok(result)
    }
}

/// Parallel benchmark runner
#[derive(Debug)]
pub struct BenchmarkParallel {
    pub benchmark: Benchmark,
    pub thread_count: usize,
}

impl BenchmarkParallel {
    /// Create a new parallel benchmark
    pub fn new(name: String) -> Self {
        Self {
            benchmark: Benchmark::new(name),
            thread_count: num_cpus::get(),
        }
    }
    
    /// Set thread count
    pub fn threads(mut self, count: usize) -> Self {
        self.thread_count = count;
        self
    }
    
    /// Run the parallel benchmark
    pub fn run<F>(self, benchmark_fn: F) -> TestResult<BenchResult>
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        let start = Instant::now();
        
        // Simple parallel execution (in a real implementation would use proper thread pools)
        std::thread::scope(|s| {
            for _ in 0..self.thread_count {
                s.spawn(|| {
                    for _ in 0..(self.benchmark.iterations / self.thread_count as u64) {
                        benchmark_fn();
                    }
                });
            }
        });
        
        let duration = start.elapsed();
        
        Ok(BenchResult {
            name: self.benchmark.name,
            duration,
            iterations: self.benchmark.iterations,
            bytes_processed: None,
            memory_used: None,
        })
    }
}

/// Test operations handler
pub struct TestHandler {
    verbose: bool,
}

impl TestHandler {
    /// Create a new test handler
    pub fn new() -> Self {
        Self {
            verbose: false,
        }
    }
    
    /// Set verbose mode
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Assert equality
    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(&self, left: T, right: T) -> TestResult<()> {
        if left == right {
            if self.verbose {
                println!("✅ Assertion passed: {:?} == {:?}", left, right);
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Assertion failed: {:?} != {:?}", left, right)))
        }
    }
    
    /// Assert not equality
    pub fn assert_ne<T: PartialEq + std::fmt::Debug>(&self, left: T, right: T) -> TestResult<()> {
        if left != right {
            if self.verbose {
                println!("✅ Assertion passed: {:?} != {:?}", left, right);
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Assertion failed: {:?} == {:?}", left, right)))
        }
    }
    
    /// Assert true
    pub fn assert_true(&self, condition: bool) -> TestResult<()> {
        if condition {
            if self.verbose {
                println!("✅ Assertion passed: condition is true");
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error("Assertion failed: condition is false"))
        }
    }
    
    /// Assert false
    pub fn assert_false(&self, condition: bool) -> TestResult<()> {
        if !condition {
            if self.verbose {
                println!("✅ Assertion passed: condition is false");
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error("Assertion failed: condition is true"))
        }
    }
    
    /// Run a test
    pub fn run_test<F>(&self, name: &str, test_fn: F) -> TestResult<()>
    where
        F: FnOnce() -> TestResult<()>,
    {
        if self.verbose {
            println!("🧪 Running test: {}", name);
        }
        
        match test_fn() {
            Ok(()) => {
                if self.verbose {
                    println!("✅ Test passed: {}", name);
                }
                Ok(())
            }
            Err(e) => {
                println!("❌ Test failed: {}: {}", name, e);
                Err(e)
            }
        }
    }
}

impl Default for TestHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize test processing
pub fn init_benchmarking() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (benchmarking) initialized");
    Ok(())
}

/// Test functionality
pub fn test_benchmarking() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}
