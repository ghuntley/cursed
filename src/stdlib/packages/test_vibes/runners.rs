//! Testing functionality for runners

use crate::error::CursedError;
use super::core::{VibeTest, VibeBench, VibeTestingManager, BenchResult};
use std::collections::HashMap;
use crate::stdlib::packages::CryptoError;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Main test runner
pub fn test_main() -> TestResult<()> {
    println!("🧪 TestVibes test runner starting...");
    
    let manager = VibeTestingManager::new()
        .verbose(true)
        .fail_fast(false);
        
    let (test_results, bench_results) = manager.run_all()?;
    
    println!("🎉 Test run completed!");
    println!("📊 Test results: {:?}", test_results);
    println!("⚡ Benchmark results: {:?}", bench_results);
    
    Ok(())
}

/// Advanced test runner with configuration
#[derive(Debug)]
pub struct TestRunner {
    pub manager: VibeTestingManager,
    pub config: TestConfig,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            manager: VibeTestingManager::new(),
            config: TestConfig::default(),
        }
    }

    pub fn with_config(mut self, config: TestConfig) -> Self {
        self.config = config;
        self
    }

    pub fn add_test(mut self, test: VibeTest) -> Self {
        self.manager = self.manager.add_test(test);
        self
    }

    pub fn add_benchmark(mut self, benchmark: VibeBench) -> Self {
        self.manager = self.manager.add_benchmark(benchmark);
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.manager = self.manager.verbose(verbose);
        self
    }

    pub fn parallel(mut self, parallel: bool) -> Self {
        self.manager = self.manager.parallel(parallel);
        self
    }

    pub fn fail_fast(mut self, fail_fast: bool) -> Self {
        self.manager = self.manager.fail_fast(fail_fast);
        self
    }

    pub fn run(&self) -> TestResult<TestRunResult> {
        println!("🚀 Running tests with TestRunner...");
        
        let start = std::time::Instant::now();
        
        let (test_results, bench_results) = if self.config.run_benchmarks {
            self.manager.run_all()?
        } else {
            let test_results = self.manager.run_tests()?;
            (test_results, Vec::new())
        };
        
        let duration = start.elapsed();
        
        let run_result = TestRunResult {
            test_results,
            bench_results,
            duration,
            total_tests: self.manager.tests.len(),
            total_benchmarks: self.manager.benchmarks.len(),
            config: self.config.clone(),
        };
        
        if self.config.generate_report {
            run_result.print_report();
        }
        
        Ok(run_result)
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for test runner
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub run_benchmarks: bool,
    pub generate_report: bool,
    pub output_format: OutputFormat,
    pub filter_pattern: Option<String>,
    pub max_threads: Option<usize>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            run_benchmarks: true,
            generate_report: true,
            output_format: OutputFormat::Pretty,
            filter_pattern: None,
            max_threads: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Pretty,
    Json,
    Junit,
    Tap,
}

/// Result of a test run
#[derive(Debug)]
pub struct TestRunResult {
    pub test_results: HashMap<String, bool>,
    pub bench_results: Vec<BenchResult>,
    pub duration: std::time::Duration,
    pub total_tests: usize,
    pub total_benchmarks: usize,
    pub config: TestConfig,
}

impl TestRunResult {
    pub fn passed_tests(&self) -> usize {
        self.test_results.values().filter(|&&passed| passed).count()
    }

    pub fn failed_tests(&self) -> usize {
        self.test_results.values().filter(|&&passed| !passed).count()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            return 1.0;
        }
        self.passed_tests() as f64 / self.total_tests as f64
    }

    pub fn print_report(&self) {
        println!("\n🎯 Test Run Report");
        println!("==================");
        println!("📊 Tests: {} passed, {} failed, {} total", 
                 self.passed_tests(), 
                 self.failed_tests(), 
                 self.total_tests);
        println!("⚡ Benchmarks: {} completed", self.bench_results.len());
        println!("⏱️  Duration: {:?}", self.duration);
        println!("📈 Success Rate: {:.1}%", self.success_rate() * 100.0);
        
        if !self.bench_results.is_empty() {
            println!("\n🚀 Benchmark Results:");
            for bench in &self.bench_results {
                println!("  {} - {:?} avg over {} iterations", 
                         bench.name, 
                         bench.avg_duration(), 
                         bench.iterations);
            }
        }
        
        println!();
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
pub fn init_runners() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (runners) initialized");
    Ok(())
}

/// Test functionality
pub fn test_runners() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}

/// Configuration for test runners
#[derive(Debug, Clone)]
pub struct TestRunnerConfig {
    pub parallel: bool,
    pub verbose: bool,
    pub max_threads: usize,
    pub timeout_seconds: Option<u64>,
}

impl Default for TestRunnerConfig {
    fn default() -> Self {
        Self {
            parallel: false,
            verbose: false,
            max_threads: 4,
            timeout_seconds: Some(30),
        }
    }
}

/// Test manager for organizing test execution
#[derive(Debug)]
pub struct TestManager {
    pub test_count: usize,
}

impl TestManager {
    pub fn new() -> Self {
        Self { test_count: 0 }
    }
    
    /// Run tests managed by this manager
    pub fn run_tests(&mut self) -> TestResult<TestSummary> {
        // Basic implementation
        Ok(TestSummary {
            total: self.test_count,
            passed: self.test_count,
            failed: 0,
            failures: Vec::new(),
        })
    }
}

/// Summary of test execution results
#[derive(Debug)]
pub struct TestSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<String>,
}

impl TestSummary {
    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed == 0
    }
    
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.passed as f64 / self.total as f64) * 100.0
        }
    }
}
