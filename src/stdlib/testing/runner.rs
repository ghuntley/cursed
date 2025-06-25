use crate::error::CursedError;
/// Test runner implementation for CURSED testing framework
/// 
/// Coordinates test discovery, execution, and reporting to provide
/// a complete test running experience with detailed progress tracking.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use super::{
    TestError, TestFrameworkResult
// };

/// Test runner configuration
#[derive(Debug, Clone)]
pub struct TestRunnerConfig {
    /// Test execution mode
    /// Whether to stop on first failure
    /// Whether to show verbose output
    /// Whether to show test timing information
impl Default for TestRunnerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Test suite information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestSuite {
    /// Suite name
    /// Tests in this suite
    /// Suite metadata
impl TestSuite {
    pub fn new(name: String, tests: Vec<TestInfo>) -> Self {
        Self {
        }
    }
    
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    /// Get number of tests in suite
    pub fn test_count(&self) -> usize {
        self.tests.len()
    /// Check if suite is empty
    pub fn is_empty(&self) -> bool {
        self.tests.is_empty()
    }
}

/// Test suite execution result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestSuiteResult {
    /// Suite information
    /// Individual test results
    /// Suite execution metrics
    /// Suite execution time
    /// Suite status summary
impl TestSuiteResult {
    pub fn new(suite: TestSuite, test_results: Vec<TestResult>, execution_time: Duration) -> Self {
        let metrics = ExecutionMetrics::from_results(&test_results);
        let status = if metrics.failed_tests > 0 {
            SuiteStatus::Failed
        } else if metrics.passed_tests > 0 {
            SuiteStatus::Passed
        } else {
            SuiteStatus::Empty
        
        Self {
        }
    }
    
    /// Check if suite passed (all tests passed)
    pub fn is_success(&self) -> bool {
        matches!(self.status, SuiteStatus::Passed)
    /// Get failure count
    pub fn failure_count(&self) -> usize {
        self.metrics.failed_tests
    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        self.metrics.success_rate()
    }
}

/// Test suite execution status
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SuiteStatus {
    /// All tests in suite passed
    /// At least one test in suite failed
    /// Suite was skipped
    /// Suite had no tests
/// Complete test run result
#[derive(Debug, Clone)]
pub struct RunnerResult {
    /// All test results
    /// Suite results (if organized by suites)
    /// Overall execution metrics
    /// Total execution time
    /// Run status
    /// Summary information
impl RunnerResult {
    pub fn new(test_results: Vec<TestResult>, total_time: Duration) -> Self {
        let overall_metrics = ExecutionMetrics::from_results(&test_results);
        let status = if overall_metrics.failed_tests > 0 {
            RunStatus::Failed
        } else if overall_metrics.passed_tests > 0 {
            RunStatus::Passed
        } else {
            RunStatus::NoTests
        
        let summary = RunSummary {
        
        Self {
        }
    }
    
    pub fn with_suites(mut self, suite_results: Vec<TestSuiteResult>) -> Self {
        self.suite_results = suite_results;
        self
    /// Check if all tests passed
    pub fn is_success(&self) -> bool {
        matches!(self.status, RunStatus::Passed)
    /// Get total number of failures
    pub fn failure_count(&self) -> usize {
        self.overall_metrics.failed_tests
    }
}

/// Overall test run status
#[derive(Debug, Clone, PartialEq)]
pub enum RunStatus {
    /// All tests passed
    /// At least one test failed
    /// No tests were found or executed
    /// Test run was cancelled
/// Test run summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RunSummary {
    /// Total number of tests
    /// Number of passed tests
    /// Number of failed tests
    /// Number of ignored tests
    /// Success rate percentage
    /// Total execution time
    /// Average execution time per test
impl RunSummary {
    /// Format summary as a string
    pub fn format(&self) -> String {
        format!(
            self.total_time.as_secs_f64()
        )
    }
}

/// Main test runner implementation
pub struct TestRunner {
impl TestRunner {
    /// Create a new test runner with default configuration
    pub fn new() -> Self {
        Self::with_config(TestRunnerConfig::default())
    /// Create a new test runner with custom configuration
    pub fn with_config(config: TestRunnerConfig) -> Self {
        Self { config }
    }
    
    /// Run tests using the provided executor
    pub fn run_tests<E: TestExecutor + ?Sized>(&self, tests: Vec<TestInfo>, executor: &E) -> TestFrameworkResult<RunnerResult> {
        let start_time = Instant::now();
        
        if tests.is_empty() {
            return Ok(RunnerResult::new(Vec::new(), Duration::from_secs(0)));
        // Execute tests based on configuration
        let test_results = match self.config.execution_mode {
            super::framework::TestExecutionMode::Sequential => {
                self.run_tests_sequential(tests, executor)?
            }
            super::framework::TestExecutionMode::Parallel => {
                self.run_tests_parallel(tests, executor)?
            }
            super::framework::TestExecutionMode::Adaptive => {
                self.run_tests_adaptive(tests, executor)?
            }
        
        let total_time = start_time.elapsed();
        let result = RunnerResult::new(test_results, total_time);
        
        Ok(result)
    /// Run tests organized into suites
    pub fn run_test_suites<E: TestExecutor>(&self, suites: Vec<TestSuite>, executor: &E) -> TestFrameworkResult<RunnerResult> {
        let start_time = Instant::now();
        let mut all_test_results = Vec::new();
        let mut suite_results = Vec::new();
        
        for suite in suites {
            let suite_start_time = Instant::now();
            
            if self.config.verbose {
                println!("Running test suite: {}", suite.name);
            let test_results = self.run_tests(suite.tests.clone(), executor)?;
            let suite_execution_time = suite_start_time.elapsed();
            
            let suite_result = TestSuiteResult::new(
                suite_execution_time
            );
            
            // Check fail fast before moving data
            let has_failures = test_results.failure_count() > 0;
            
            all_test_results.extend(test_results.test_results);
            suite_results.push(suite_result);
            
            if self.config.fail_fast && has_failures {
                break;
            }
        }
        
        let total_time = start_time.elapsed();
        let result = RunnerResult::new(all_test_results, total_time)
            .with_suites(suite_results);
        
        Ok(result)
    /// Run a single test
    pub fn run_single_test<E: TestExecutor>(&self, test: TestInfo, executor: &E) -> TestFrameworkResult<TestResult> {
        if self.config.verbose {
            println!("Running test: {}", test.name);
        let result = executor.execute_test(test)?;
        
        if self.config.show_timing {
            println!("Test completed in {:.3}s", result.execution_time.as_secs_f64());
        Ok(result)
    /// Run tests sequentially
    fn run_tests_sequential<E: TestExecutor + ?Sized>(&self, tests: Vec<TestInfo>, executor: &E) -> TestFrameworkResult<Vec<TestResult>> {
        let mut results = Vec::new();
        
        for (index, test) in tests.into_iter().enumerate() {
            if self.config.verbose {
                println!("Running test {}: {}", index + 1, test.name);
            let result = executor.execute_test(test)?;
            
            if self.config.show_timing {
                println!("  Completed in {:.3}s", result.execution_time.as_secs_f64());
            // Check for failure and fail fast option
            if self.config.fail_fast && result.status.is_failure() {
                results.push(result);
                break;
            results.push(result);
        Ok(results)
    /// Run tests in parallel
    fn run_tests_parallel<E: TestExecutor + ?Sized>(&self, tests: Vec<TestInfo>, executor: &E) -> TestFrameworkResult<Vec<TestResult>> {
        if self.config.verbose {
            println!("Running {} tests in parallel", tests.len());
        // Use the executor's parallel execution capability
        let results = executor.execute_tests(tests)?;
        
        Ok(results)
    /// Run tests adaptively (choose best execution mode)
    fn run_tests_adaptive<E: TestExecutor + ?Sized>(&self, tests: Vec<TestInfo>, executor: &E) -> TestFrameworkResult<Vec<TestResult>> {
        // Simple adaptive logic: use parallel for many tests, sequential for few
        if tests.len() > 10 && executor.get_config().max_parallel_tests > 1 {
            self.run_tests_parallel(tests, executor)
        } else {
            self.run_tests_sequential(tests, executor)
        }
    }
    
    /// Create test suites from a list of tests
    pub fn organize_into_suites(&self, tests: Vec<TestInfo>) -> Vec<TestSuite> {
        let mut suites: HashMap<String, Vec<TestInfo>> = HashMap::new();
        
        for test in tests {
            let suite_name = if test.module.is_empty() {
                "default".to_string()
            } else {
                test.module.clone()
            
            suites.entry(suite_name).or_insert_with(Vec::new).push(test);
        suites.into_iter()
            .map(|(name, tests)| TestSuite::new(name, tests))
            .collect()
    /// Filter tests based on patterns
    pub fn filter_tests(&self, tests: Vec<TestInfo>, include_patterns: &[String], exclude_patterns: &[String]) -> Vec<TestInfo> {
        tests.into_iter()
            .filter(|test| {
                // Check include patterns
                if !include_patterns.is_empty() {
                    let included = include_patterns.iter()
                        .any(|pattern| self.matches_pattern(&test.name, pattern));
                    if !included {
                        return false;
                    }
                }
                
                // Check exclude patterns
                if exclude_patterns.iter()
                    .any(|pattern| self.matches_pattern(&test.name, pattern)) {
                    return false;
                true
            })
            .collect()
    /// Simple pattern matching
    fn matches_pattern(&self, text: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        if pattern.contains('*') {
            // Simple wildcard matching
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.is_empty() {
                return true;
            let mut text_pos = 0;
            for (i, part) in parts.iter().enumerate() {
                if part.is_empty() {
                    continue;
                if i == 0 {
                    if !text[text_pos..].starts_with(part) {
                        return false;
                    }
                    text_pos += part.len();
                } else if i == parts.len() - 1 {
                    return text[text_pos..].ends_with(part);
                } else {
                    if let Some(pos) = text[text_pos..].find(part) {
                        text_pos += pos + part.len();
                    } else {
                        return false;
                    }
                }
            }
            true
        } else {
            text.contains(pattern)
        }
    }
    
    /// Get runner configuration
    pub fn get_config(&self) -> &TestRunnerConfig {
        &self.config
    /// Update runner configuration
    pub fn update_config(&mut self, config: TestRunnerConfig) {
        self.config = config;
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Test run progress tracking
#[derive(Debug, Clone)]
pub struct TestProgress {
    /// Current test being executed
    /// Number of tests completed
    /// Total number of tests
    /// Number of tests passed so far
    /// Number of tests failed so far
    /// Elapsed time since start
    /// Estimated remaining time
impl TestProgress {
    pub fn new(total: usize) -> Self {
        Self {
        }
    }
    
    /// Update progress with test result
    pub fn update_with_result(&mut self, result: &TestResult, elapsed: Duration) {
        self.completed += 1;
        self.elapsed_time = elapsed;
        
        if result.status.is_success() {
            self.passed += 1;
        } else if result.status.is_failure() {
            self.failed += 1;
        // Estimate remaining time
        if self.completed > 0 {
            let avg_time_per_test = elapsed.as_secs_f64() / self.completed as f64;
            let remaining_tests = self.total - self.completed;
            self.estimated_remaining = Some(Duration::from_secs_f64(
                avg_time_per_test * remaining_tests as f64
            ));
        }
    }
    
    /// Get completion percentage
    pub fn completion_percentage(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.completed as f64 / self.total as f64) * 100.0
        }
    }
    
    /// Format progress as string
    pub fn format(&self) -> String {
        let percentage = self.completion_percentage();
        format!(
            "Progress: {}/{} ({:.1}%) - {} passed, {} failed",
            self.failed
        )
    }
}
