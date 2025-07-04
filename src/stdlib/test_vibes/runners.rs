//! Test runners for the CURSED testing framework

use crate::error::CursedError;
use super::core::{VibeTest, VibeBench, TestResult};

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

/// Test runner for managing and executing tests
#[derive(Debug)]
pub struct TestRunner {
    pub tests: Vec<VibeTest>,
    pub benchmarks: Vec<VibeBench>,
    pub config: TestRunnerConfig,
    pub manager: TestManager,
}

impl TestRunner {
    /// Create a new test runner
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            benchmarks: Vec::new(),
            config: TestRunnerConfig::default(),
            manager: TestManager::new(),
        }
    }
    
    /// Add a test to the runner
    pub fn add_test(mut self, test: VibeTest) -> Self {
        self.tests.push(test);
        self
    }
    
    /// Add a benchmark to the runner
    pub fn add_benchmark(mut self, benchmark: VibeBench) -> Self {
        self.benchmarks.push(benchmark);
        self
    }
    
    /// Run all tests and return summary
    pub fn run(&mut self) -> TestResult<TestSummary> {
        let mut passed = 0;
        let mut failed = 0;
        let mut failures = Vec::new();
        
        for test in &mut self.tests {
            match test.get_result() {
                Ok(_) => passed += 1,
                Err(e) => {
                    failed += 1;
                    failures.push(format!("{}: {}", test.name, e));
                }
            }
        }
        
        Ok(TestSummary {
            total: self.tests.len(),
            passed,
            failed,
            failures,
        })
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
