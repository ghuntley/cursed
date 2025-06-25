use crate::error::CursedError;
/// Core testing framework for CURSED
/// 
/// Provides the main TestFramework implementation that coordinates
/// test discovery, execution, and reporting.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use super::{
    TestError, TestFrameworkResult
// };

/// Configuration for the test framework
#[derive(Debug, Clone)]
pub struct TestFrameworkConfig {
    /// Root directory to search for tests
    
    /// Test file patterns to match
    
    /// Maximum number of parallel test threads
    
    /// Default timeout for individual tests
    
    /// Whether to capture test output
    
    /// Whether to stop on first failure
    
    /// Test filter configuration
    
    /// Report format preference
    
    /// Output directory for reports
    
    /// Whether to show verbose output
    
    /// Whether to show test timing information
    
    /// Whether to run ignored tests
impl Default for TestFrameworkConfig {
    fn default() -> Self {
        Self {
            test_patterns: vec![
                "**/*test*.csd".to_string(),
                "**/test_*.csd".to_string(),
                "tests/**/*.csd".to_string(),
        }
    }
/// Test execution mode
#[derive(Debug, Clone, PartialEq)]
pub enum TestExecutionMode {
    /// Run tests sequentially
    /// Run tests in parallel
    /// Adaptive based on test characteristics
/// Test filtering mode
#[derive(Debug, Clone, PartialEq)]
pub enum TestFilterMode {
    /// Run all tests
    /// Run only tests matching patterns
    /// Run all tests except those matching patterns
    /// Run tests with specific attributes
/// Main test framework implementation
pub struct TestFramework {
impl TestFramework {
    /// Create a new test framework with default configuration
    pub fn new() -> Self {
        Self::with_config(TestFrameworkConfig::default())
    /// Create a new test framework with custom configuration
    pub fn with_config(config: TestFrameworkConfig) -> Self {
        let discovery_config = DiscoveryConfig {
        
        let executor_config = TestExecutorConfig {
        
        let runner_config = TestRunnerConfig {
            execution_mode: if config.max_parallel_tests > 1 {
                TestExecutionMode::Parallel
            } else {
                TestExecutionMode::Sequential
        
        let discovery = TestDiscovery::with_config(discovery_config);
        let executor: Box<dyn TestExecutor> = if config.max_parallel_tests > 1 {
            Box::new(ParallelExecutor::with_config(executor_config))
        } else {
//             Box::new(crate::stdlib::testing::executor::SequentialExecutor::with_config(executor_config))
        let runner = TestRunner::with_config(runner_config);
        let reporter: Box<dyn TestReporter> = match config.report_format {
//             ReportFormat::Json => Box::new(crate::stdlib::testing::reporting::JsonReporter::new()),
//             ReportFormat::Xml => Box::new(crate::stdlib::testing::reporting::XmlReporter::new()),
//             ReportFormat::Html => Box::new(crate::stdlib::testing::reporting::HtmlReporter::new()),
        
        Self {
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
            });
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
//                 matches!(result.status, crate::stdlib::testing::executor::TestStatus::Passed)
            );
        // Phase 4: Reporting
        let report = TestFrameworkReport {
        
        self.reporter.report_execution_complete(&report)?;
        
        // Save report to file if configured
        if let Some(output_dir) = &self.config.report_output_dir {
            self.save_report(&report, output_dir)?;
        Ok(report)
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
//             matches!(result.status, crate::stdlib::testing::executor::TestStatus::Passed)
        );
        
        Ok(result)
    /// Get current test statistics
    pub fn get_statistics(&self) -> &TestStatistics {
        &self.statistics
    /// Update framework configuration
    pub fn update_config(&mut self, config: TestFrameworkConfig) {
        self.config = config;
        // TODO: Update dependent components with new config
    /// Add custom test filter
    pub fn add_filter(&mut self, filter: TestFilter) {
        self.discovery.add_filter(filter);
    /// Save test report to file
    fn save_report(&self, report: &TestFrameworkReport, output_dir: &PathBuf) -> TestFrameworkResult<()> {
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)
                .map_err(|e| TestError::ReportError(format!("Failed to create report directory: {}", e)))?;
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
    /// Generate XML report content
    fn generate_xml_report(&self, report: &TestFrameworkReport) -> TestFrameworkResult<String> {
        let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push('\n');
        xml.push_str(&format!(
            report.total_time.as_secs_f64()
        ));
        xml.push('\n');
        
        xml.push_str(&format!(
            report.total_time.as_secs_f64()
        ));
        xml.push('\n');
        
        for failure in &report.failures {
            xml.push_str(&format!(
                failure.execution_time.as_secs_f64()
            ));
            xml.push('\n');
            
//             if let crate::stdlib::testing::executor::TestStatus::Failed(ref error) = failure.status {
                xml.push_str(&format!(r#"      <failure message="{}">{}</failure>"#, 
                    error.replace('<', "&lt;").replace('>', "&gt;")
                ));
                xml.push('\n');
            xml.push_str("    </testcase>");
            xml.push('\n');
        xml.push_str("  </testsuite>");
        xml.push('\n');
        xml.push_str("</testsuites>");
        xml.push('\n');
        
        Ok(xml)
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
    
    /// Number of tests discovered
    
    /// Number of tests executed
    
    /// Number of tests that passed
    
    /// Number of tests that failed
    
    /// Number of tests that were ignored
    
    /// Number of tests that were skipped
    
    /// Details of failed tests
    
    /// Detailed statistics
impl TestFrameworkReport {
    /// Check if all tests passed
    pub fn is_success(&self) -> bool {
        self.tests_failed == 0
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
// Add num_cpus dependency for automatic CPU detection
mod num_cpus {
    use std::thread;
    
    pub fn get() -> usize {
        thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1)
    }
}
