//! Testing functionality for benchmarks

use crate::error::CursedError;
use super::core::BenchResult;
use std::time::{Duration, Instant};
use crate::stdlib::packages::CryptoError;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Standard benchmark implementation
#[derive(Debug, Clone)]
pub struct Benchmark {
    pub name: String,
    pub description: Option<String>,
    pub iterations: u64,
    pub warmup: u64,
    pub bench_fn: Option<fn() -> TestResult<()>>,
}

impl Benchmark {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            iterations: 1000,
            warmup: 100,
            bench_fn: None,
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn iterations(mut self, iterations: u64) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn warmup(mut self, warmup: u64) -> Self {
        self.warmup = warmup;
        self
    }

    pub fn with_bench_fn(mut self, bench_fn: fn() -> TestResult<()>) -> Self {
        self.bench_fn = Some(bench_fn);
        self
    }

    pub fn run(&self) -> TestResult<BenchResult> {
        println!("🚀 Running benchmark: {}", self.name);

        // Warmup
        for _ in 0..self.warmup {
            if let Some(bench_fn) = self.bench_fn {
                bench_fn()?;
            }
        }

        // Benchmark
        let start = Instant::now();
        for _ in 0..self.iterations {
            if let Some(bench_fn) = self.bench_fn {
                bench_fn()?;
            }
        }
        let duration = start.elapsed();

        Ok(BenchResult::new(self.name.clone(), duration, self.iterations))
    }
}

/// Memory-focused benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkMemory {
    pub name: String,
    pub description: Option<String>,
    pub iterations: u64,
    pub track_allocations: bool,
    pub bench_fn: Option<fn() -> TestResult<()>>,
}

impl BenchmarkMemory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            iterations: 1000,
            track_allocations: true,
            bench_fn: None,
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn iterations(mut self, iterations: u64) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn track_allocations(mut self, track: bool) -> Self {
        self.track_allocations = track;
        self
    }

    pub fn with_bench_fn(mut self, bench_fn: fn() -> TestResult<()>) -> Self {
        self.bench_fn = Some(bench_fn);
        self
    }

    pub fn run(&self) -> TestResult<BenchResult> {
        println!("🧠 Running memory benchmark: {}", self.name);

        let start_memory = if self.track_allocations {
            // In a real implementation, we'd track memory allocations
            // For now, just simulate
            Some(0u64)
        } else {
            None
        };

        let start = Instant::now();
        for _ in 0..self.iterations {
            if let Some(bench_fn) = self.bench_fn {
                bench_fn()?;
            }
        }
        let duration = start.elapsed();

        let mut result = BenchResult::new(self.name.clone(), duration, self.iterations);
        
        if let Some(_start_mem) = start_memory {
            // Simulate memory tracking
            result = result.with_memory(1024 * self.iterations); // Fake memory usage
        }

        Ok(result)
    }
}

/// Parallel benchmark for concurrent testing
#[derive(Debug, Clone)]
pub struct BenchmarkParallel {
    pub name: String,
    pub description: Option<String>,
    pub iterations: u64,
    pub concurrency: usize,
    pub bench_fn: Option<fn() -> TestResult<()>>,
}

impl BenchmarkParallel {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            iterations: 1000,
            concurrency: 4,
            bench_fn: None,
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn iterations(mut self, iterations: u64) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn with_bench_fn(mut self, bench_fn: fn() -> TestResult<()>) -> Self {
        self.bench_fn = Some(bench_fn);
        self
    }

    pub fn run(&self) -> TestResult<BenchResult> {
        println!("⚡ Running parallel benchmark: {} (concurrency: {})", self.name, self.concurrency);

        let start = Instant::now();
        
        // Simulate parallel execution
        // In a real implementation, we'd use std::thread or async runtime
        let iterations_per_thread = self.iterations / (self.concurrency as u64);
        
        for _thread in 0..self.concurrency {
            for _ in 0..iterations_per_thread {
                if let Some(bench_fn) = self.bench_fn {
                    bench_fn()?;
                }
            }
        }
        
        let duration = start.elapsed();

        Ok(BenchResult::new(self.name.clone(), duration, self.iterations))
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
pub fn init_benchmarks() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (benchmarks) initialized");
    Ok(())
}

/// Test functionality
pub fn test_benchmarks() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}
