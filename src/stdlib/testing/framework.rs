use crate::error::CursedError;
/// Core testing framework for CURSED
/// 
/// Provides the main TestFramework implementation that coordinates
/// test discovery, execution, and reporting.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use super::{
    discovery::{TestDiscovery, TestFilter, DiscoveryConfig, TestInfo},
    executor::{TestExecutor, TestExecutorConfig, TestResult, ParallelExecutor},
    runner::{TestRunner, TestRunnerConfig, RunnerResult},
    reporting::{TestReporter, ReportFormat, ConsoleReporter},
    stats::{TestStatistics, TestTiming},
    TestError, TestFrameworkResult
};

/// Configuration for the test framework
#[derive(Debug, Clone)]
pub struct TestFrameworkConfig {
    /// Root directory to search for tests
    pub test_root: PathBuf,
    
    /// Test file patterns to match
    pub test_patterns: Vec<String>,
    
    /// Maximum number of parallel test threads
    pub max_parallel_tests: usize,
    
    /// Default timeout for individual tests
    pub default_timeout: Duration,
    
    /// Whether to capture test output
    pub capture_output: bool,
    
    /// Whether to stop on first failure
    pub fail_fast: bool,
    
    /// Test filter configuration
    pub filter: TestFilter,
    
    /// Report format preference
    pub report_format: ReportFormat,
    
    /// Output directory for reports
    pub report_output_dir: Option<PathBuf>,
    
    /// Whether to show verbose output
    pub verbose: bool,
    
    /// Whether to show test timing information
    pub show_timing: bool,
    
    /// Whether to run ignored tests
    pub run_ignored: bool,
}

impl Default for TestFrameworkConfig {
    fn default() -> Self {
        Self {
            test_root: PathBuf::from("."),
            test_patterns: vec![
                "**/*test*.csd".to_string(),
                "**/test_*.csd".to_string(),
                "tests/**/*.csd".to_string(),
            ],
            max_parallel_tests: num_cpus::get().max(1),
            default_timeout: Duration::from_secs(60),
            capture_output: true,
            fail_fast: false,
            filter: TestFilter::default(),
            report_format: ReportFormat::Console,
            report_output_dir: None,
            verbose: false,
            show_timing: true,
            run_ignored: false,
        }
    }
}

/// Test execution mode
#[derive(Debug, Clone, PartialEq)]
pub enum TestExecutionMode {
    /// Run tests sequentially
    Sequential,
    /// Run tests in parallel
    Parallel,
    /// Adaptive based on test characteristics
    Adaptive,
}

/// Test filtering mode
#[derive(Debug, Clone, PartialEq)]
pub enum TestFilterMode {
    /// Run all tests
    All,
    /// Run only tests matching patterns
    Include(Vec<String>),
    /// Run all tests except those matching patterns
    Exclude(Vec<String>),
    /// Run tests with specific attributes
    Attributes(Vec<String>),
}

/// Main test framework implementation
pub struct TestFramework {
    config: TestFrameworkConfig,
    discovery: TestDiscovery,
    executor: Box<dyn TestExecutor>,
    runner: TestRunner,
    reporter: Box<dyn TestReporter>,
    statistics: TestStatistics,
}

impl TestFramework {
    /// Create a new test framework with default configuration
    pub fn new() -> Self {
        Self::with_config(TestFrameworkConfig::default())
    }
    
    /// Create a new test framework with custom configuration
    pub fn with_config(config: TestFrameworkConfig) -> Self {
        let discovery_config = DiscoveryConfig {
            test_root: config.test_root.clone(),
            patterns: config.test_patterns.clone(),
            filter: config.filter.clone(),
            recursive: true,
            include_ignored: config.run_ignored,
        };
        
        let executor_config = TestExecutorConfig {
            default_timeout: config.default_timeout,
            capture_output: config.capture_output,
            max_parallel_tests: config.max_parallel_tests,
            fail_fast: config.fail_fast,
        };
        
        let runner_config = TestRunnerConfig {
            execution_mode: if config.max_parallel_tests > 1 {
                TestExecutionMode::Parallel
            } else {
                TestExecutionMode::Sequential
            },
            fail_fast: config.fail_fast,
            verbose: config.verbose,
            show_timing: config.show_timing,
        };
        
        let discovery = TestDiscovery::with_config(discovery_config);
        let executor: Box<dyn TestExecutor> = if config.max_parallel_tests > 1 {
            Box::new(ParallelExecutor::with_config(executor_config))
        } else {
//             Box::new(crate::stdlib::testing::executor::SequentialExecutor::with_config(executor_config))
        };
        let runner = TestRunner::with_config(runner_config);
        let reporter: Box<dyn TestReporter> = match config.report_format {
            ReportFormat::Console => Box::new(ConsoleReporter::new()),
//             ReportFormat::Json => Box::new(crate::stdlib::testing::reporting::JsonReporter::new()),
//             ReportFormat::Xml => Box::new(crate::stdlib::testing::reporting::XmlReporter::new()),
//             ReportFormat::Html => Box::new(crate::stdlib::testing::reporting::HtmlReporter::new()),
        };
        
        Self {
            config,
            discovery,
            executor,
            runner,
            reporter,
            statistics: TestStatistics::new(),
        }
    }
    
    /// Run all tests according to the framework configuration
    pub fn run_tests(&mut self) -> TestFrameworkResult<TestFrameworkReport> {
        let start_time = Instant::now();
        
        // Phase 1: Test Discovery
        self.reporter.report_discovery_start()?;
        let tests = self.discovery.discover_tests()
            .map_err(|e| TestError::DiscoveryError(e.to_string()))?;
        
        if tests.is_empty() {
            self.reporter.report_no_tests_found()?;
            return Ok(TestFrameworkReport {
                total_time: start_time.elapsed(),
                tests_discovered: 0,
                tests_executed: 0,
                tests_passed: 0,
                tests_failed: 0,
                tests_ignored: 0,
                tests_skipped: 0,
                failures: Vec::new(),
                statistics: self.statistics.clone(),
            });
        }
        
        self.reporter.report_discovery_complete(tests.len())?;
        
        // Phase 2: Test Execution
        self.reporter.report_execution_start(tests.len())?;
        let results = self.runner.run_tests(tests, self.executor.as_ref())
            .map_err(|e| TestError::ExecutionError(e.to_string()))?;
        
        // Phase 3: Statistics Collection
        let mut passed = 0;
        let mut failed = 0;
        let mut ignored = 0;
        let mut skipped = 0;
        let mut failures = Vec::new();
        
        for result in &results.test_results {
            match &result.status {
//                 crate::stdlib::testing::executor::TestStatus::Passed => passed += 1,
//                 crate::stdlib::testing::executor::TestStatus::Failed(_) => {
                    failed += 1;
                    failures.push(result.clone());
                }
//                 crate::stdlib::testing::executor::TestStatus::Ignored => ignored += 1,
//                 crate::stdlib::testing::executor::TestStatus::Skipped => skipped += 1,
//                 crate::stdlib::testing::executor::TestStatus::Timeout => {
                    failed += 1;
                    failures.push(result.clone());
                }
            }
            
            // Update statistics
            self.statistics.record_test_execution(
                &result.test_info.name,
                result.execution_time,
//                 matches!(result.status, crate::stdlib::testing::executor::TestStatus::Passed)
            );
        }
        
        // Phase 4: Reporting
        let report = TestFrameworkReport {
            total_time: start_time.elapsed(),
            tests_discovered: results.test_results.len(),
            tests_executed: passed + failed,
            tests_passed: passed,
            tests_failed: failed,
            tests_ignored: ignored,
            tests_skipped: skipped,
            failures,
            statistics: self.statistics.clone(),
        };
        
        self.reporter.report_execution_complete(&report)?;
        
        // Save report to file if configured
        if let Some(output_dir) = &self.config.report_output_dir {
            self.save_report(&report, output_dir)?;
        }
        
        Ok(report)
    }
    
    /// Run a specific test by name
    pub fn run_test(&mut self, test_name: &str) -> TestFrameworkResult<TestResult> {
        // Discover tests first
        let tests = self.discovery.discover_tests()
            .map_err(|e| TestError::DiscoveryError(e.to_string()))?;
        
        // Find the specific test
        let test_info = tests.into_iter()
            .find(|t| t.name == test_name)
            .ok_or_else(|| TestError::DiscoveryError(format!("Test '{}' not found", test_name)))?;
        
        // Execute the test
        let result = self.executor.execute_test(test_info)
            .map_err(|e| TestError::ExecutionError(e.to_string()))?;
        
        // Update statistics
        self.statistics.record_test_execution(
            &result.test_info.name,
            result.execution_time,
//             matches!(result.status, crate::stdlib::testing::executor::TestStatus::Passed)
        );
        
        Ok(result)
    }
    
    /// Get current test statistics
    pub fn get_statistics(&self) -> &TestStatistics {
        &self.statistics
    }
    
    /// Update framework configuration
    pub fn update_config(&mut self, config: TestFrameworkConfig) {
        self.config = config;
        // TODO: Update dependent components with new config
    }
    
    /// Add custom test filter
    pub fn add_filter(&mut self, filter: TestFilter) {
        self.discovery.add_filter(filter);
    }
    
    /// Save test report to file
    fn save_report(&self, report: &TestFrameworkReport, output_dir: &PathBuf) -> TestFrameworkResult<()> {
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)
                .map_err(|e| TestError::ReportError(format!("Failed to create report directory: {}", e)))?;
        }
        
        match self.config.report_format {
            ReportFormat::Json => {
                let file_path = output_dir.join("test_report.json");
                let json_content = serde_json::to_string_pretty(report)
                    .map_err(|e| TestError::ReportError(format!("Failed to serialize JSON report: {}", e)))?;
                std::fs::write(file_path, json_content)
                    .map_err(|e| TestError::ReportError(format!("Failed to write JSON report: {}", e)))?;
            }
            ReportFormat::Xml => {
                let file_path = output_dir.join("test_report.xml");
                let xml_content = self.generate_xml_report(report)?;
                std::fs::write(file_path, xml_content)
                    .map_err(|e| TestError::ReportError(format!("Failed to write XML report: {}", e)))?;
            }
            ReportFormat::Html => {
                let file_path = output_dir.join("test_report.html");
                let html_content = self.generate_html_report(report)?;
                std::fs::write(file_path, html_content)
                    .map_err(|e| TestError::ReportError(format!("Failed to write HTML report: {}", e)))?;
            }
            ReportFormat::Console => {
                // Console output doesn't need file saving
            }
        }
        
        Ok(())
    }
    
    /// Generate XML report content
    fn generate_xml_report(&self, report: &TestFrameworkReport) -> TestFrameworkResult<String> {
        let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push('\n');
        xml.push_str(&format!(
            r#"<testsuites tests="{}" failures="{}" errors="0" time="{:.3}">"#,
            report.tests_executed,
            report.tests_failed,
            report.total_time.as_secs_f64()
        ));
        xml.push('\n');
        
        xml.push_str(&format!(
            r#"  <testsuite name="CURSED Tests" tests="{}" failures="{}" errors="0" time="{:.3}">"#,
            report.tests_executed,
            report.tests_failed,
            report.total_time.as_secs_f64()
        ));
        xml.push('\n');
        
        for failure in &report.failures {
            xml.push_str(&format!(
                r#"    <testcase name="{}" time="{:.3}">"#,
                failure.test_info.name,
                failure.execution_time.as_secs_f64()
            ));
            xml.push('\n');
            
//             if let crate::stdlib::testing::executor::TestStatus::Failed(ref error) = failure.status {
                xml.push_str(&format!(r#"      <failure message="{}">{}</failure>"#, 
                    error.replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;"),
                    error.replace('<', "&lt;").replace('>', "&gt;")
                ));
                xml.push('\n');
            }
            
            xml.push_str("    </testcase>");
            xml.push('\n');
        }
        
        xml.push_str("  </testsuite>");
        xml.push('\n');
        xml.push_str("</testsuites>");
        xml.push('\n');
        
        Ok(xml)
    }
    
    /// Generate HTML report content
    fn generate_html_report(&self, report: &TestFrameworkReport) -> TestFrameworkResult<String> {
        let mut html = String::from(r#"<!DOCTYPE html>
<html>
<head>
    <title>CURSED Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .summary { background: #f5f5f5; padding: 15px; border-radius: 5px; margin-bottom: 20px; }
        .passed { color: green; }
        .failed { color: red; }
        .ignored { color: orange; }
        .failure { background: #ffebee; padding: 10px; margin: 10px 0; border-left: 4px solid red; }
        .failure-message { font-family: monospace; white-space: pre-wrap; }
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
    </style>
</head>
<body>
    <h1>CURSED Test Report</h1>
"#);
        
        // Summary section
        html.push_str("<div class=\"summary\">");
        html.push_str(&format!("<h2>Test Summary</h2>"));
        html.push_str(&format!("<p><strong>Total Time:</strong> {:.3}s</p>", report.total_time.as_secs_f64()));
        html.push_str(&format!("<p><strong>Tests Discovered:</strong> {}</p>", report.tests_discovered));
        html.push_str(&format!("<p><strong>Tests Executed:</strong> {}</p>", report.tests_executed));
        html.push_str(&format!("<p class=\"passed\"><strong>Passed:</strong> {}</p>", report.tests_passed));
        html.push_str(&format!("<p class=\"failed\"><strong>Failed:</strong> {}</p>", report.tests_failed));
        html.push_str(&format!("<p class=\"ignored\"><strong>Ignored:</strong> {}</p>", report.tests_ignored));
        html.push_str("</div>");
        
        // Failures section
        if !report.failures.is_empty() {
            html.push_str("<h2>Test Failures</h2>");
            for failure in &report.failures {
                html.push_str("<div class=\"failure\">");
                html.push_str(&format!("<h3>{}</h3>", failure.test_info.name));
                html.push_str(&format!("<p><strong>Execution Time:</strong> {:.3}s</p>", failure.execution_time.as_secs_f64()));
                
//                 if let crate::stdlib::testing::executor::TestStatus::Failed(ref error) = failure.status {
                    html.push_str("<div class=\"failure-message\">");
                    html.push_str(&error.replace('<', "&lt;").replace('>', "&gt;"));
                    html.push_str("</div>");
                }
                
                html.push_str("</div>");
            }
        }
        
        html.push_str("</body></html>");
        
        Ok(html)
    }
}

impl Default for TestFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive test framework report
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestFrameworkReport {
    /// Total execution time for all tests
    pub total_time: Duration,
    
    /// Number of tests discovered
    pub tests_discovered: usize,
    
    /// Number of tests executed
    pub tests_executed: usize,
    
    /// Number of tests that passed
    pub tests_passed: usize,
    
    /// Number of tests that failed
    pub tests_failed: usize,
    
    /// Number of tests that were ignored
    pub tests_ignored: usize,
    
    /// Number of tests that were skipped
    pub tests_skipped: usize,
    
    /// Details of failed tests
    pub failures: Vec<TestResult>,
    
    /// Detailed statistics
    pub statistics: TestStatistics,
}

impl TestFrameworkReport {
    /// Check if all tests passed
    pub fn is_success(&self) -> bool {
        self.tests_failed == 0
    }
    
    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.tests_executed == 0 {
            100.0
        } else {
            (self.tests_passed as f64 / self.tests_executed as f64) * 100.0
        }
    }
    
    /// Get average test execution time
    pub fn average_execution_time(&self) -> Duration {
        if self.tests_executed == 0 {
            Duration::from_secs(0)
        } else {
            self.total_time / self.tests_executed as u32
        }
    }
}

// Add num_cpus dependency for automatic CPU detection
mod num_cpus {
    use std::thread;
    
    pub fn get() -> usize {
        thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1)
    }
}
