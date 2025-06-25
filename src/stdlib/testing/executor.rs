use crate::error::CursedError;
/// Test execution engine for CURSED testing framework
/// 
/// Handles the actual execution of discovered tests with support for
/// timeouts, parallel execution, output capture, and result reporting.

use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use super::{
    TestError, TestFrameworkResult
// };

/// Test execution status
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TestStatus {
    /// Test passed successfully
    /// Test failed with error message
    /// Test was ignored/skipped
    /// Test was skipped due to filter
    /// Test timed out
impl TestStatus {
    /// Check if the test status represents a success
    pub fn is_success(&self) -> bool {
        matches!(self, TestStatus::Passed)
    /// Check if the test status represents a failure
    pub fn is_failure(&self) -> bool {
        matches!(self, TestStatus::Failed(_) | TestStatus::Timeout)
    /// Get failure message if applicable
    pub fn failure_message(&self) -> Option<&str> {
        match self {
        }
    }
/// Test execution failure details
#[derive(Debug, Clone)]
pub struct TestFailure {
    /// CursedError message
    /// Stack trace (if available)
    /// Assertion details
    /// Standard output captured during test
    /// Standard error captured during test
impl TestFailure {
    pub fn new(message: String) -> Self {
        Self {
        }
    }
    
    pub fn with_stack_trace(mut self, stack_trace: String) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    pub fn with_assertion_details(mut self, details: String) -> Self {
        self.assertion_details = Some(details);
        self
    pub fn with_output(mut self, stdout: String, stderr: String) -> Self {
        self.stdout = Some(stdout);
        self.stderr = Some(stderr);
        self
    }
}

/// Test execution result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestResult {
    /// Test information
    /// Execution status
    /// Execution time
    /// Test output (if captured)
    /// CursedError output (if captured)
    /// Memory usage (if measured)
    /// Additional metadata
impl TestResult {
    pub fn new(test_info: TestInfo, status: TestStatus, execution_time: Duration) -> Self {
        Self {
        }
    }
    
    pub fn with_output(mut self, output: String) -> Self {
        self.output = Some(output);
        self
    pub fn with_error_output(mut self, error_output: String) -> Self {
        self.error_output = Some(error_output);
        self
    pub fn with_memory_usage(mut self, memory_usage: u64) -> Self {
        self.memory_usage = Some(memory_usage);
        self
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Test execution timeout configuration
#[derive(Debug, Clone)]
pub struct TestTimeout {
    /// Default timeout for all tests
    /// Per-test timeout overrides
    /// Maximum allowed timeout
impl Default for TestTimeout {
    fn default() -> Self {
        Self {
            maximum: Duration::from_secs(300), // 5 minutes
        }
    }
impl TestTimeout {
    /// Get timeout for a specific test
    pub fn get_timeout(&self, test_name: &str) -> Duration {
        self.overrides.get(test_name)
            .copied()
            .unwrap_or(self.default)
            .min(self.maximum)
    /// Set timeout for a specific test
    pub fn set_timeout(mut self, test_name: String, timeout: Duration) -> Self {
        self.overrides.insert(test_name, timeout.min(self.maximum));
        self
    }
}

/// Test execution context
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Working directory for test execution
    /// Environment variables
    /// Test execution timeout configuration
    /// Whether to capture test output
    /// Whether to measure memory usage
impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
        }
    }
/// Configuration for test executor
#[derive(Debug, Clone)]
pub struct TestExecutorConfig {
    /// Default timeout for individual tests
    /// Whether to capture test output
    /// Maximum number of parallel tests
    /// Whether to stop on first failure
impl Default for TestExecutorConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Trait for test execution implementations
pub trait TestExecutor: Send + Sync {
    /// Execute a single test
    fn execute_test(&self, test_info: TestInfo) -> TestFrameworkResult<TestResult>;
    
    /// Execute multiple tests
    fn execute_tests(&self, tests: Vec<TestInfo>) -> TestFrameworkResult<Vec<TestResult>> {
        let mut results = Vec::new();
        for test in tests {
            let result = self.execute_test(test)?;
            results.push(result);
        }
        Ok(results)
    /// Get executor configuration
    fn get_config(&self) -> &TestExecutorConfig;
/// Sequential test executor implementation
pub struct SequentialExecutor {
impl SequentialExecutor {
    /// Create a new sequential executor with default configuration
    pub fn new() -> Self {
        Self::with_config(TestExecutorConfig::default())
    /// Create a new sequential executor with custom configuration
    pub fn with_config(config: TestExecutorConfig) -> Self {
        Self {
        }
    }
    
    /// Set execution context
    pub fn with_context(mut self, context: ExecutionContext) -> Self {
        self.context = context;
        self
    }
}

impl TestExecutor for SequentialExecutor {
    fn execute_test(&self, test_info: TestInfo) -> TestFrameworkResult<TestResult> {
        let start_time = Instant::now();
        
        // Check if test should be ignored
        if test_info.should_ignore() {
            return Ok(TestResult::new(
                Duration::from_millis(0)
            ));
        // Get timeout for this test
        let timeout = test_info.timeout()
            .unwrap_or(self.context.timeout.get_timeout(&test_info.name));
        
        // Execute the test
        let status = match self.run_test_process(&test_info, timeout) {
            Ok((output, error_output)) => {
                if test_info.should_panic() {
                    // Test was expected to panic but didn't
                    TestStatus::Failed("Test was expected to panic but completed successfully".to_string())
                } else {
                    TestStatus::Passed
                }
            }
            Err(error) => {
                if test_info.should_panic() {
                    // Test panicked as expected
                    TestStatus::Passed
                } else {
                    TestStatus::Failed(error)
                }
            }
        
        let execution_time = start_time.elapsed();
        
        Ok(TestResult::new(test_info, status, execution_time))
    fn get_config(&self) -> &TestExecutorConfig {
        &self.config
    }
}

impl SequentialExecutor {
    /// Run a test as a separate process
    fn run_test_process(&self, test_info: &TestInfo, timeout: Duration) -> Result<(String, String), String> {
        // For now, simulate test execution
        // In a real implementation, this would compile and run the CURSED test
        
        // Simulate some execution time
        thread::sleep(Duration::from_millis(10));
        
        // Simple test result simulation based on test name
        if test_info.name.contains("fail") {
            Err("Simulated test failure".to_string())
        } else if test_info.name.contains("panic") {
            Err("Test panicked: Simulated panic".to_string())
        } else {
            Ok(("Test passed".to_string(), String::new()))
        }
    }
/// Parallel test executor implementation
pub struct ParallelExecutor {
impl ParallelExecutor {
    /// Create a new parallel executor with default configuration
    pub fn new() -> Self {
        Self::with_config(TestExecutorConfig::default())
    /// Create a new parallel executor with custom configuration
    pub fn with_config(config: TestExecutorConfig) -> Self {
        Self {
        }
    }
    
    /// Set execution context
    pub fn with_context(mut self, context: ExecutionContext) -> Self {
        self.context = context;
        self
    }
}

impl TestExecutor for ParallelExecutor {
    fn execute_test(&self, test_info: TestInfo) -> TestFrameworkResult<TestResult> {
        // For single test execution, just use sequential approach
        let sequential = SequentialExecutor::with_config(self.config.clone())
            .with_context(self.context.clone());
        sequential.execute_test(test_info)
    fn execute_tests(&self, tests: Vec<TestInfo>) -> TestFrameworkResult<Vec<TestResult>> {
        let max_threads = self.config.max_parallel_tests.max(1);
        let results = Arc::new(Mutex::new(Vec::new()));
        let tests = Arc::new(tests);
        let test_index = Arc::new(Mutex::new(0usize));
        
        let mut handles = Vec::new();
        
        for _ in 0..max_threads {
            let tests_clone = Arc::clone(&tests);
            let results_clone = Arc::clone(&results);
            let test_index_clone = Arc::clone(&test_index);
            let config = self.config.clone();
            let context = self.context.clone();
            
            let handle = thread::spawn(move || {
                let executor = SequentialExecutor::with_config(config)
                    .with_context(context);
                
                loop {
                    let test_to_run = {
                        let mut index = test_index_clone.lock().unwrap();
                        if *index >= tests_clone.len() {
                            break;
                        }
                        let test = tests_clone[*index].clone();
                        *index += 1;
                        test
                    
                    match executor.execute_test(test_to_run) {
                        Ok(result) => {
                            let mut results = results_clone.lock().unwrap();
                            results.push(result);
                        }
                        Err(error) => {
                            eprintln!("Test execution error: {:?}", error);
                        }
                    }
                }
            });
            
            handles.push(handle);
        // Wait for all threads to complete
        for handle in handles {
            handle.join().map_err(|_| TestError::ExecutionError("Thread join failed".to_string()))?;
        let results = Arc::try_unwrap(results)
            .map_err(|_| TestError::ExecutionError("Failed to unwrap results".to_string()))?
            .into_inner()
            .map_err(|_| TestError::ExecutionError("Failed to unlock results".to_string()))?;
        
        Ok(results)
    fn get_config(&self) -> &TestExecutorConfig {
        &self.config
    }
}

/// Test execution metrics and statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionMetrics {
    /// Total number of tests executed
    /// Number of tests that passed
    /// Number of tests that failed
    /// Number of tests that were ignored
    /// Total execution time
    /// Average execution time per test
    /// Fastest test execution time
    /// Slowest test execution time
    /// Memory usage statistics
impl ExecutionMetrics {
    /// Create metrics from test results
    pub fn from_results(results: &[TestResult]) -> Self {
        let total_tests = results.len();
        let passed_tests = results.iter()
            .filter(|r| r.status.is_success())
            .count();
        let failed_tests = results.iter()
            .filter(|r| r.status.is_failure())
            .count();
        let ignored_tests = results.iter()
            .filter(|r| matches!(r.status, TestStatus::Ignored))
            .count();
        
        let total_time: Duration = results.iter()
            .map(|r| r.execution_time)
            .sum();
        
        let average_time = if total_tests > 0 {
            total_time / total_tests as u32
        } else {
            Duration::from_secs(0)
        
        let fastest_time = results.iter()
            .map(|r| r.execution_time)
            .min()
            .unwrap_or(Duration::from_secs(0));
        
        let slowest_time = results.iter()
            .map(|r| r.execution_time)
            .max()
            .unwrap_or(Duration::from_secs(0));
        
        Self {
        }
    }
    
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            100.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }
/// Memory usage statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryStats {
    /// Peak memory usage in bytes
    /// Average memory usage in bytes
    /// Memory usage by test
impl Default for MemoryStats {
    fn default() -> Self {
        Self {
        }
    }
}
