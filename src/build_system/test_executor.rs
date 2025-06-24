//! Test Execution System
//! 
//! Handles compilation and execution of discovered tests with parallel execution,
//! result parsing, and performance metrics collection.

use crate::build_system::test_discovery::{TestFunction, TestCategory, TestFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn, instrument};
use regex::Regex;

/// Test execution configuration
#[derive(Debug, Clone)]
pub struct TestExecutionConfig {
    /// Number of parallel test threads
    pub parallel_threads: usize,
    
    /// Default timeout for test execution (seconds)
    pub default_timeout: u64,
    
    /// Maximum timeout for any test (seconds)
    pub max_timeout: u64,
    
    /// Whether to capture test output
    pub capture_output: bool,
    
    /// Whether to run tests in release mode
    pub release_mode: bool,
    
    /// Additional cargo test arguments
    pub cargo_args: Vec<String>,
    
    /// Environment variables for test execution
    pub env_vars: HashMap<String, String>,
    
    /// Working directory for test execution
    pub work_dir: PathBuf,
    
    /// Whether to use linking fix for Nix environment
    pub use_linking_fix: bool,
    
    /// Linking fix script path
    pub linking_fix_script: Option<PathBuf>,
}

impl Default for TestExecutionConfig {
    fn default() -> Self {
        Self {
            parallel_threads: num_cpus::get(),
            default_timeout: 60,
            max_timeout: 300,
            capture_output: true,
            release_mode: false,
            cargo_args: Vec::new(),
            env_vars: HashMap::new(),
            work_dir: PathBuf::from("."),
            use_linking_fix: true,
            linking_fix_script: Some(PathBuf::from("./fix_linking.sh")),
        }
    }
}

/// Result of a single test execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Test function information
    pub test_function: TestFunction,
    
    /// Test status
    pub status: TestStatus,
    
    /// Execution duration
    pub duration: Duration,
    
    /// Memory usage peak (if available)
    pub memory_usage: Option<usize>,
    
    /// Test output (stdout)
    pub output: String,
    
    /// Test error output (stderr)
    pub error_output: String,
    
    /// Exit code
    pub exit_code: Option<i32>,
    
    /// Failure reason (if failed)
    pub failure_reason: Option<String>,
    
    /// Additional metrics
    pub metrics: TestMetrics,
}

/// Test execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestStatus {
    /// Test passed successfully
    Passed,
    
    /// Test failed
    Failed,
    
    /// Test was ignored/skipped
    Ignored,
    
    /// Test timed out
    TimedOut,
    
    /// Test compilation failed
    CompilationFailed,
    
    /// Test execution was interrupted
    Interrupted,
    
    /// Test status is unknown
    Unknown,
}

/// Test execution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMetrics {
    /// Compilation time
    pub compilation_time: Option<Duration>,
    
    /// Test execution time
    pub execution_time: Duration,
    
    /// Peak memory usage (bytes)
    pub peak_memory: Option<usize>,
    
    /// Number of assertions (if parseable)
    pub assertion_count: Option<usize>,
    
    /// Custom metrics from test output
    pub custom_metrics: HashMap<String, f64>,
}

/// Overall test execution results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionResult {
    /// Individual test results
    pub test_results: Vec<TestResult>,
    
    /// Results organized by status
    pub results_by_status: HashMap<TestStatus, Vec<TestResult>>,
    
    /// Execution statistics
    pub statistics: TestExecutionStatistics,
    
    /// Total execution time
    pub total_duration: Duration,
    
    /// Test execution summary
    pub summary: TestExecutionSummary,
}

/// Test execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionStatistics {
    /// Total number of tests run
    pub total_tests: usize,
    
    /// Number of tests passed
    pub passed: usize,
    
    /// Number of tests failed
    pub failed: usize,
    
    /// Number of tests ignored
    pub ignored: usize,
    
    /// Number of tests timed out
    pub timed_out: usize,
    
    /// Number of compilation failures
    pub compilation_failed: usize,
    
    /// Average test execution time
    pub average_duration: Duration,
    
    /// Total memory usage
    pub total_memory_usage: Option<usize>,
    
    /// Success rate percentage
    pub success_rate: f64,
}

/// Test execution summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionSummary {
    /// Overall success status
    pub success: bool,
    
    /// Summary message
    pub message: String,
    
    /// Failed tests with details
    pub failed_tests: Vec<TestFailureDetails>,
    
    /// Performance insights
    pub performance_insights: Vec<String>,
    
    /// Recommendations for improvements
    pub recommendations: Vec<String>,
}

/// Details about a failed test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFailureDetails {
    /// Test name
    pub test_name: String,
    
    /// File path
    pub file_path: PathBuf,
    
    /// Failure reason
    pub reason: String,
    
    /// Relevant output excerpt
    pub output_excerpt: String,
}

/// Main test executor
pub struct TestExecutor {
    config: TestExecutionConfig,
    output_parser: TestOutputParser,
}

impl TestExecutor {
    /// Create a new test executor
    pub fn new(config: TestExecutionConfig) -> Self {
        Self {
            config,
            output_parser: TestOutputParser::new(),
        }
    }
    
    /// Execute a collection of tests
    #[instrument(skip(self, tests))]
    pub async fn execute_tests(&self, tests: Vec<TestFunction>) -> Result<(), Error> {
        info!("Starting execution of {} tests with {} threads", 
              tests.len(), self.config.parallel_threads);
        
        let start_time = Instant::now();
        let test_results = Arc::new(Mutex::new(Vec::new()));
        let tests_remaining = Arc::new(Mutex::new(tests.clone()));
        
        // Create worker threads for parallel execution
        let mut handles = Vec::new();
        
        for worker_id in 0..self.config.parallel_threads {
            let test_results_clone = Arc::clone(&test_results);
            let tests_remaining_clone = Arc::clone(&tests_remaining);
            let config = self.config.clone();
            let output_parser = self.output_parser.clone();
            
            let handle = thread::spawn(move || {
                Self::worker_thread(worker_id, test_results_clone, tests_remaining_clone, config, output_parser)
            });
            
            handles.push(handle);
        }
        
        // Wait for all workers to complete
        for handle in handles {
            if let Err(e) = handle.join() {
                error!("Worker thread error: {:?}", e);
            }
        }
        
        let total_duration = start_time.elapsed();
        let results = test_results.lock().unwrap().clone();
        
        // Process and organize results
        let execution_result = self.process_results(results, total_duration);
        
        info!("Test execution completed in {:?}. {} passed, {} failed", 
              total_duration, execution_result.statistics.passed, execution_result.statistics.failed);
        
        Ok(execution_result)
    }
    
    /// Worker thread for parallel test execution
    fn worker_thread(
        worker_id: usize,
        test_results: Arc<Mutex<Vec<TestResult>>>,
        tests_remaining: Arc<Mutex<Vec<TestFunction>>>,
        config: TestExecutionConfig,
        output_parser: TestOutputParser,
    ) {
        debug!("Worker {} started", worker_id);
        
        loop {
            // Get next test to execute
            let test = {
                let mut tests = tests_remaining.lock().unwrap();
                if tests.is_empty() {
                    break;
                }
                tests.pop().unwrap()
            };
            
            debug!("Worker {} executing test: {}", worker_id, test.name);
            
            // Execute the test
            let result = Self::execute_single_test(&test, &config, &output_parser);
            
            // Store result
            {
                let mut results = test_results.lock().unwrap();
                results.push(result);
            }
        }
        
        debug!("Worker {} finished", worker_id);
    }
    
    /// Execute a single test
    fn execute_single_test(
        test: &TestFunction,
        config: &TestExecutionConfig,
        output_parser: &TestOutputParser,
    ) -> TestResult {
        let start_time = Instant::now();
        
        // Build cargo test command
        let mut cmd = if config.use_linking_fix && config.linking_fix_script.as_ref().map_or(false, |p| p.exists()) {
            let mut fix_cmd = Command::new(config.linking_fix_script.as_ref().unwrap());
            fix_cmd.arg("cargo");
            fix_cmd
        } else {
            Command::new("cargo")
        };
        
        cmd.arg("test")
           .arg("--")
           .arg(&test.name);
        
        // Add release flag if needed
        if config.release_mode {
            cmd.arg("--release");
        }
        
        // Add cargo arguments
        for arg in &config.cargo_args {
            cmd.arg(arg);
        }
        
        // Set environment variables
        for (key, value) in &config.env_vars {
            cmd.env(key, value);
        }
        
        // Configure output capture
        if config.capture_output {
            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        }
        
        cmd.current_dir(&config.work_dir);
        
        // Determine timeout
        let timeout = Duration::from_secs(
            test.timeout.unwrap_or(config.default_timeout).min(config.max_timeout)
        );
        
        debug!("Executing command: {:?} with timeout {:?}", cmd, timeout);
        
        // Execute with timeout
        let execution_result = Self::execute_with_timeout(cmd, timeout);
        
        let duration = start_time.elapsed();
        
        // Parse results
        let (status, output, error_output, exit_code, failure_reason) = match execution_result {
            Ok((output, stderr, exit_code)) => {
                let status = if exit_code == 0 {
                    TestStatus::Passed
                } else {
                    TestStatus::Failed
                };
                
                let failure_reason = if status == TestStatus::Failed {
                    output_parser.extract_failure_reason(&output, &stderr)
                } else {
                    None
                };
                
                (status, output, stderr, Some(exit_code), failure_reason)
            }
            Err(e) => {
                let status = if e.to_string().contains("timeout") {
                    TestStatus::TimedOut
                } else {
                    TestStatus::Failed
                };
                
                (status, String::new(), e.to_string(), None, Some(e.to_string()))
            }
        };
        
        // Extract metrics
        let metrics = output_parser.extract_metrics(&output, duration);
        
        TestResult {
            test_function: test.clone(),
            status,
            duration,
            memory_usage: metrics.peak_memory,
            output,
            error_output,
            exit_code,
            failure_reason,
            metrics,
        }
    }
    
    /// Execute command with timeout
    fn execute_with_timeout(
        mut cmd: Command,
        timeout: Duration,
    ) -> Result<(), Error> {
        let start = Instant::now();
        
        let mut child = cmd.spawn()?;
        
        // Wait for completion or timeout
        loop {
            match child.try_wait()? {
                Some(status) => {
                    // Process completed
                    let output = child.wait_with_output()?;
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    let exit_code = status.code().unwrap_or(-1);
                    
                    return Ok((stdout, stderr, exit_code));
                }
                None => {
                    // Still running, check timeout
                    if start.elapsed() >= timeout {
                        let _ = child.kill();
                        let _ = child.wait();
                        return Err(format!("Test execution timed out after {:?}", timeout).into());
                    }
                    
                    // Sleep briefly before checking again
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
    
    /// Process raw test results into organized format
    fn process_results(&self, results: Vec<TestResult>, total_duration: Duration) -> TestExecutionResult {
        let mut results_by_status: HashMap<TestStatus, Vec<TestResult>> = HashMap::new();
        
        let mut passed = 0;
        let mut failed = 0;
        let mut ignored = 0;
        let mut timed_out = 0;
        let mut compilation_failed = 0;
        
        let mut total_duration_sum = Duration::new(0, 0);
        let mut total_memory_usage = 0;
        let mut memory_samples = 0;
        
        for result in &results {
            results_by_status.entry(result.status.clone()).or_default().push(result.clone());
            
            match result.status {
                TestStatus::Passed => passed += 1,
                TestStatus::Failed => failed += 1,
                TestStatus::Ignored => ignored += 1,
                TestStatus::TimedOut => timed_out += 1,
                TestStatus::CompilationFailed => compilation_failed += 1,
                _ => {}
            }
            
            total_duration_sum += result.duration;
            
            if let Some(memory) = result.memory_usage {
                total_memory_usage += memory;
                memory_samples += 1;
            }
        }
        
        let total_tests = results.len();
        let success_rate = if total_tests > 0 {
            (passed as f64 / total_tests as f64) * 100.0
        } else {
            0.0
        };
        
        let average_duration = if total_tests > 0 {
            total_duration_sum / total_tests as u32
        } else {
            Duration::new(0, 0)
        };
        
        let total_memory = if memory_samples > 0 {
            Some(total_memory_usage)
        } else {
            None
        };
        
        let statistics = TestExecutionStatistics {
            total_tests,
            passed,
            failed,
            ignored,
            timed_out,
            compilation_failed,
            average_duration,
            total_memory_usage: total_memory,
            success_rate,
        };
        
        // Generate summary
        let summary = self.generate_summary(&statistics, &results);
        
        TestExecutionResult {
            test_results: results,
            results_by_status,
            statistics,
            total_duration,
            summary,
        }
    }
    
    /// Generate execution summary with insights
    fn generate_summary(&self, statistics: &TestExecutionStatistics, results: &[TestResult]) -> TestExecutionSummary {
        let success = statistics.failed == 0 && statistics.timed_out == 0 && statistics.compilation_failed == 0;
        
        let message = if success {
            format!("All {} tests passed successfully! ({:.1}% success rate)", 
                   statistics.total_tests, statistics.success_rate)
        } else {
            format!("{} of {} tests failed. Success rate: {:.1}%", 
                   statistics.failed + statistics.timed_out + statistics.compilation_failed,
                   statistics.total_tests, statistics.success_rate)
        };
        
        // Extract failed test details
        let failed_tests = results.iter()
            .filter(|r| matches!(r.status, TestStatus::Failed | TestStatus::TimedOut | TestStatus::CompilationFailed))
            .map(|r| TestFailureDetails {
                test_name: r.test_function.name.clone(),
                file_path: r.test_function.file_path.clone(),
                reason: r.failure_reason.clone().unwrap_or_else(|| format!("{:?}", r.status)),
                output_excerpt: r.error_output.split("\n").take(3).collect::<Vec<_>>().join("\n"),
            })
            .collect();
        
        // Generate performance insights
        let mut performance_insights = Vec::new();
        
        if statistics.average_duration > Duration::from_secs(5) {
            performance_insights.push(format!(
                "Average test duration is {:.2}s, consider optimizing slow tests",
                statistics.average_duration.as_secs_f64()
            ));
        }
        
        let slow_tests = results.iter()
            .filter(|r| r.duration > Duration::from_secs(10))
            .count();
        
        if slow_tests > 0 {
            performance_insights.push(format!(
                "{} tests took longer than 10 seconds",
                slow_tests
            ));
        }
        
        // Generate recommendations
        let mut recommendations = Vec::new();
        
        if statistics.failed > 0 {
            recommendations.push("Review failed tests and fix underlying issues".to_string());
        }
        
        if statistics.timed_out > 0 {
            recommendations.push("Consider increasing timeout for long-running tests".to_string());
        }
        
        if statistics.success_rate < 90.0 {
            recommendations.push("Focus on improving test reliability".to_string());
        }
        
        TestExecutionSummary {
            success,
            message,
            failed_tests,
            performance_insights,
            recommendations,
        }
    }
}

/// Parser for test output to extract metrics and failure information
#[derive(Debug, Clone)]
pub struct TestOutputParser {
    assertion_regex: Regex,
    memory_regex: Regex,
    benchmark_regex: Regex,
    failure_regex: Regex,
}

impl TestOutputParser {
    /// Create a new output parser
    pub fn new() -> Self {
        Self {
            assertion_regex: Regex::new(r"assertion failed").unwrap(),
            memory_regex: Regex::new(r"memory usage: (\d+) bytes").unwrap(),
            benchmark_regex: Regex::new(r"bench: +(\d+(?:,\d+)*) ns/iter").unwrap(),
            failure_regex: Regex::new(r"thread '.*' panicked at '(.+)'").unwrap(),
        }
    }
    
    /// Extract failure reason from test output
    pub fn extract_failure_reason(&self, stdout: &str, stderr: &str) -> Option<String> {
        let combined_output = format!("{}\n{}", stdout, stderr);
        
        if let Some(captures) = self.failure_regex.captures(&combined_output) {
            if let Some(reason) = captures.get(1) {
                return Some(reason.as_str().to_string());
            }
        }
        
        // Look for other common failure patterns
        if combined_output.contains("assertion failed") {
            return Some("Assertion failed".to_string());
        }
        
        if combined_output.contains("expected") && combined_output.contains("found") {
            return Some("Value mismatch".to_string());
        }
        
        None
    }
    
    /// Extract metrics from test output
    pub fn extract_metrics(&self, output: &str, execution_time: Duration) -> TestMetrics {
        let mut custom_metrics = HashMap::new();
        
        // Extract memory usage
        let peak_memory = if let Some(captures) = self.memory_regex.captures(output) {
            captures.get(1).and_then(|m| m.as_str().replace(',', "").parse().ok())
        } else {
            None
        };
        
        // Extract benchmark results
        if let Some(captures) = self.benchmark_regex.captures(output) {
            if let Some(ns_str) = captures.get(1) {
                if let Ok(ns_value) = ns_str.as_str().replace(',', "").parse::<f64>() {
                    custom_metrics.insert("benchmark_ns_per_iter".to_string(), ns_value);
                }
            }
        }
        
        // Count assertions (rough estimate)
        let assertion_count = output.matches("assert").count();
        let assertion_count = if assertion_count > 0 { Some(assertion_count) } else { None };
        
        TestMetrics {
            compilation_time: self.extract_compilation_time(output),
            execution_time,
            peak_memory,
            assertion_count,
            custom_metrics,
        }
    }
    
    /// Extract compilation time from cargo output
    fn extract_compilation_time(&self, output: &str) -> Option<Duration> {
        use regex::Regex;
        
        // Look for compilation timing in cargo output
        let compile_regex = Regex::new(r"Compiling .+ \((.+)\)").ok()?;
        let finished_regex = Regex::new(r"Finished .+ target\(s\) in (.+)s").ok()?;
        
        // Try to extract from "Finished" line first (most accurate)
        for line in output.split("\n") {
            if let Some(captures) = finished_regex.captures(line) {
                if let Some(time_str) = captures.get(1) {
                    if let Ok(seconds) = time_str.as_str().parse::<f64>() {
                        return Some(Duration::from_secs_f64(seconds));
                    }
                }
            }
        }
        
        // Fallback: try to extract from individual compilation lines
        let mut total_time = 0.0;
        let mut compilation_count = 0;
        
        for line in output.split("\n") {
            if line.contains("Compiling") && line.contains("(") {
                // Extract time from compilation status
                if let Some(time_part) = line.split('(').nth(1) {
                    if let Some(time_str) = time_part.split(')').next() {
                        // Parse time formats like "1.2s", "345ms", etc.
                        if let Some(duration) = self.parse_duration_string(time_str) {
                            total_time += duration.as_secs_f64();
                            compilation_count += 1;
                        }
                    }
                }
            }
        }
        
        if compilation_count > 0 {
            Some(Duration::from_secs_f64(total_time))
        } else {
            None
        }
    }
    
    /// Parse duration string like "1.2s", "345ms", "1m 30s"
    fn parse_duration_string(&self, duration_str: &str) -> Option<Duration> {
        let trimmed = duration_str.trim();
        
        // Handle simple cases: "1.2s", "345ms"
        if trimmed.ends_with("ms") {
            let num_str = &trimmed[..trimmed.len() - 2];
            if let Ok(ms) = num_str.parse::<f64>() {
                return Some(Duration::from_millis(ms as u64));
            }
        } else if trimmed.ends_with('s') {
            let num_str = &trimmed[..trimmed.len() - 1];
            if let Ok(seconds) = num_str.parse::<f64>() {
                return Some(Duration::from_secs_f64(seconds));
            }
        } else if trimmed.ends_with('m') {
            let num_str = &trimmed[..trimmed.len() - 1];
            if let Ok(minutes) = num_str.parse::<f64>() {
                return Some(Duration::from_secs_f64(minutes * 60.0));
            }
        }
        
        // Handle complex cases: "1m 30s"
        if trimmed.contains('m') && trimmed.contains('s') {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let mut total_seconds = 0.0;
            
            for part in parts {
                if part.ends_with('m') {
                    if let Ok(minutes) = part[..part.len() - 1].parse::<f64>() {
                        total_seconds += minutes * 60.0;
                    }
                } else if part.ends_with('s') {
                    if let Ok(seconds) = part[..part.len() - 1].parse::<f64>() {
                        total_seconds += seconds;
                    }
                }
            }
            
            if total_seconds > 0.0 {
                return Some(Duration::from_secs_f64(total_seconds));
            }
        }
        
        None
    }
}

/// Test batch for organized execution
#[derive(Debug, Clone)]
pub struct TestBatch {
    /// Batch name
    pub name: String,
    
    /// Tests in this batch
    pub tests: Vec<TestFunction>,
    
    /// Batch-specific configuration
    pub config: Option<TestExecutionConfig>,
}

impl TestBatch {
    /// Create a new test batch
    pub fn new(name: String, tests: Vec<TestFunction>) -> Self {
        Self {
            name,
            tests,
            config: None,
        }
    }
    
    /// Create batches from tests organized by category
    pub fn from_tests_by_category(tests_by_category: &HashMap<TestCategory, Vec<TestFunction>>) -> Vec<TestBatch> {
        let mut batches = Vec::new();
        
        for (category, tests) in tests_by_category {
            let batch_name = match category {
                TestCategory::Unit => "unit_tests".to_string(),
                TestCategory::Integration => "integration_tests".to_string(),
                TestCategory::Benchmark => "benchmark_tests".to_string(),
                TestCategory::Doc => "doc_tests".to_string(),
                TestCategory::Example => "example_tests".to_string(),
                TestCategory::Custom(name) => format!("custom_{}", name),
            };
            
            batches.push(TestBatch::new(batch_name, tests.clone()));
        }
        
        batches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_execution_config_default() {
        let config = TestExecutionConfig::default();
        assert!(config.parallel_threads > 0);
        assert_eq!(config.default_timeout, 60);
        assert!(config.capture_output);
    }
    
    #[test]
    fn test_output_parser_failure_extraction() {
        let parser = TestOutputParser::new();
        
        let output = "thread 'test_something' panicked at 'assertion failed: `(left == right)`'";
        let reason = parser.extract_failure_reason(output, "");
        
        assert!(reason.is_some());
        assert!(reason.unwrap().contains("assertion failed"));
    }
    
    #[test]
    fn test_output_parser_metrics_extraction() {
        let parser = TestOutputParser::new();
        
        let output = "test bench_something ... bench: 1,234 ns/iter (+/- 100)";
        let metrics = parser.extract_metrics(output, Duration::from_millis(100));
        
        assert!(metrics.custom_metrics.contains_key("benchmark_ns_per_iter"));
        assert_eq!(metrics.custom_metrics["benchmark_ns_per_iter"], 1234.0);
    }
    
    #[test]
    fn test_test_batch_creation() {
        let tests = vec![
            TestFunction {
                name: "test1".to_string(),
                file_path: PathBuf::from("test.rs"),
                line_number: 1,
                category: TestCategory::Unit,
                ignored: false,
                is_benchmark: false,
                timeout: None,
                attributes: Vec::new(),
                module_path: "test".to_string(),
            }
        ];
        
        let batch = TestBatch::new("test_batch".to_string(), tests);
        assert_eq!(batch.name, "test_batch");
        assert_eq!(batch.tests.len(), 1);
    }
}
