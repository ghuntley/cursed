//! Testing functionality for core

use crate::error::CursedError;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::stdlib::packages::CryptoError;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Result type for benchmark operations
#[derive(Debug, Clone)]
pub struct BenchResult {
    pub name: String,
    pub duration: Duration,
    pub iterations: u64,
    pub bytes_processed: Option<u64>,
    pub memory_used: Option<u64>,
}

impl BenchResult {
    pub fn new(name: String, duration: Duration, iterations: u64) -> Self {
        Self {
            name,
            duration,
            iterations,
            bytes_processed: None,
            memory_used: None,
        }
    }

    pub fn with_bytes(mut self, bytes: u64) -> Self {
        self.bytes_processed = Some(bytes);
        self
    }

    pub fn with_memory(mut self, memory: u64) -> Self {
        self.memory_used = Some(memory);
        self
    }

    pub fn avg_duration(&self) -> Duration {
        self.duration / (self.iterations as u32)
    }
}

/// Main test structure for individual tests
#[derive(Debug, Clone)]
pub struct VibeTest {
    pub name: String,
    pub description: Option<String>,
    pub timeout: Option<Duration>,
    pub should_panic: bool,
    pub ignore: bool,
    pub test_fn: Option<fn() -> TestResult<()>>,
}

impl VibeTest {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            timeout: None,
            should_panic: false,
            ignore: false,
            test_fn: None,
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn should_panic(mut self) -> Self {
        self.should_panic = true;
        self
    }

    pub fn ignore(mut self) -> Self {
        self.ignore = true;
        self
    }

    pub fn with_test_fn(mut self, test_fn: fn() -> TestResult<()>) -> Self {
        self.test_fn = Some(test_fn);
        self
    }

    pub fn run(&self) -> TestResult<()> {
        if self.ignore {
            println!("🦘 Ignoring test: {}", self.name);
            return Ok(());
        }

        println!("🧪 Running test: {}", self.name);
        let start = Instant::now();

        let result = if let Some(test_fn) = self.test_fn {
            test_fn()
        } else {
            Ok(())
        };

        let duration = start.elapsed();
        
        match result {
            Ok(()) => {
                if self.should_panic {
                    println!("❌ Test should have panicked but didn't: {}", self.name);
                    Err(CursedError::runtime_error(&"Test should have panicked".to_string()))
                } else {
                    println!("✅ Test passed: {} ({:?})", self.name, duration);
                    Ok(())
                }
            }
            Err(e) => {
                if self.should_panic {
                    println!("✅ Test panicked as expected: {} ({:?})", self.name, duration);
                    Ok(())
                } else {
                    println!("❌ Test failed: {}: {} ({:?})", self.name, e, duration);
                    Err(e)
                }
            }
        }
    }
}

/// Benchmark structure for performance testing
#[derive(Debug, Clone)]
pub struct VibeBench {
    pub name: String,
    pub description: Option<String>,
    pub iterations: u64,
    pub warmup_iterations: u64,
    pub bench_fn: Option<fn() -> TestResult<()>>,
}

impl VibeBench {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            iterations: 1000,
            warmup_iterations: 100,
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
        self.warmup_iterations = warmup;
        self
    }

    pub fn with_bench_fn(mut self, bench_fn: fn() -> TestResult<()>) -> Self {
        self.bench_fn = Some(bench_fn);
        self
    }

    pub fn run(&self) -> TestResult<BenchResult> {
        println!("🚀 Running benchmark: {}", self.name);

        // Warmup phase
        if self.warmup_iterations > 0 {
            println!("🔥 Warming up... ({} iterations)", self.warmup_iterations);
            for _ in 0..self.warmup_iterations {
                if let Some(bench_fn) = self.bench_fn {
                    bench_fn()?;
                }
            }
        }

        // Actual benchmark
        let start = Instant::now();
        for _ in 0..self.iterations {
            if let Some(bench_fn) = self.bench_fn {
                bench_fn()?;
            }
        }
        let duration = start.elapsed();

        let result = BenchResult::new(self.name.clone(), duration, self.iterations);
        
        println!("⚡ Benchmark completed: {} - {} iterations in {:?} (avg: {:?}/iter)", 
                 self.name, 
                 self.iterations, 
                 duration, 
                 result.avg_duration());

        Ok(result)
    }
}

/// Testing manager to coordinate multiple tests and benchmarks
#[derive(Debug)]
pub struct VibeTestingManager {
    pub tests: Vec<VibeTest>,
    pub benchmarks: Vec<VibeBench>,
    pub verbose: bool,
    pub parallel: bool,
    pub fail_fast: bool,
}

impl VibeTestingManager {
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            benchmarks: Vec::new(),
            verbose: false,
            parallel: false,
            fail_fast: false,
        }
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }

    pub fn fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }

    pub fn add_test(mut self, test: VibeTest) -> Self {
        self.tests.push(test);
        self
    }

    pub fn add_benchmark(mut self, benchmark: VibeBench) -> Self {
        self.benchmarks.push(benchmark);
        self
    }

    pub fn run_tests(&self) -> TestResult<HashMap<String, bool>> {
        let mut results = HashMap::new();
        let mut failed_count = 0;

        println!("🧪 Running {} tests...", self.tests.len());

        for test in &self.tests {
            let success = match test.run() {
                Ok(()) => true,
                Err(_) => {
                    failed_count += 1;
                    if self.fail_fast {
                        return Err(CursedError::runtime_error(&"Test failed and fail_fast is enabled".to_string()));
                    }
                    false
                }
            };
            results.insert(test.name.clone(), success);
        }

        if failed_count > 0 {
            println!("❌ {} test(s) failed out of {}", failed_count, self.tests.len());
            Err(CryptoError::Other(format!("{} test(s) failed", "placeholder")))
        } else {
            println!("✅ All {} tests passed!", self.tests.len());
            Ok(results)
        }
    }

    pub fn run_benchmarks(&self) -> TestResult<Vec<BenchResult>> {
        let mut results = Vec::new();

        println!("🚀 Running {} benchmarks...", self.benchmarks.len());

        for benchmark in &self.benchmarks {
            match benchmark.run() {
                Ok(result) => results.push(result),
                Err(e) => {
                    println!("❌ Benchmark failed: {}: {}", benchmark.name, e);
                    if self.fail_fast {
                        return Err(e);
                    }
                }
            }
        }

        println!("⚡ All benchmarks completed!");
        Ok(results)
    }

    pub fn run_all(&self) -> TestResult<(HashMap<String, bool>, Vec<BenchResult>)> {
        let test_results = self.run_tests()?;
        let bench_results = self.run_benchmarks()?;
        Ok((test_results, bench_results))
    }
}

impl Default for VibeTestingManager {
    fn default() -> Self {
        Self::new()
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
            Err(CursedError::runtime_error(&"Assertion failed: condition is false".to_string()))
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
            Err(CursedError::runtime_error(&"Assertion failed: condition is true".to_string()))
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
pub fn init_core() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (core) initialized");
    Ok(())
}

/// Test functionality
pub fn test_core() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}
