/// Test reporting system for CURSED testing framework
/// 
/// Provides multiple report formats and comprehensive test result reporting
/// with support for console output, JSON, XML, and HTML formats.

use std::collections::HashMap;
use std::fmt::Write;
use std::time::Duration;
use crate::crate::stdlib::errors_simple::CursedError;
use super::{
    discovery::TestInfo,
    executor::{TestResult, TestStatus},
    runner::{RunnerResult, TestSuiteResult, RunSummary},
    stats::TestStatistics,
    TestError, TestFrameworkResult
};

/// Report format options
#[derive(Debug, Clone, PartialEq)]
pub enum ReportFormat {
    /// Console output with colors and formatting
    Console,
    /// JSON format for programmatic consumption
    Json,
    /// XML format for CI/CD integration
    Xml,
    /// HTML format for web viewing
    Html,
}

/// Report configuration
#[derive(Debug, Clone)]
pub struct ReportConfig {
    /// Report format
    pub format: ReportFormat,
    /// Whether to include verbose details
    pub verbose: bool,
    /// Whether to show timing information
    pub show_timing: bool,
    /// Whether to show stack traces
    pub show_stack_traces: bool,
    /// Whether to use colored output
    pub use_colors: bool,
    /// Maximum width for console output
    pub console_width: usize,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            format: ReportFormat::Console,
            verbose: false,
            show_timing: true,
            show_stack_traces: true,
            use_colors: true,
            console_width: 80,
        }
    }
}

/// Comprehensive test report
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestReport {
    /// Report metadata
    pub metadata: ReportMetadata,
    /// Overall test summary
    pub summary: RunSummary,
    /// Individual test results
    pub test_results: Vec<TestResult>,
    /// Suite results (if organized by suites)
    pub suite_results: Vec<TestSuiteResult>,
    /// Test statistics
    pub statistics: TestStatistics,
    /// Failure details
    pub failures: Vec<FailureDetail>,
}

/// Report metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportMetadata {
    /// Report generation timestamp
    pub generated_at: String,
    /// Test framework version
    pub framework_version: String,
    /// Test execution environment
    pub environment: HashMap<String, String>,
    /// Report format
    pub format: String,
}

/// Detailed failure information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FailureDetail {
    /// Test name
    pub test_name: String,
    /// Failure message
    pub message: String,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
    /// File and line information
    pub location: Option<String>,
    /// Test output
    pub output: Option<String>,
}

/// Summary report for quick overview
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SummaryReport {
    /// Basic test counts
    pub summary: RunSummary,
    /// Top failures
    pub top_failures: Vec<String>,
    /// Performance summary
    pub performance: PerformanceSummary,
}

/// Performance summary information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceSummary {
    /// Total execution time
    pub total_time: Duration,
    /// Average test time
    pub average_time: Duration,
    /// Slowest tests
    pub slowest_tests: Vec<(String, Duration)>,
    /// Fastest tests
    pub fastest_tests: Vec<(String, Duration)>,
}

/// Trait for test reporters
pub trait TestReporter: Send + Sync {
    /// Report test discovery start
    fn report_discovery_start(&self) -> TestFrameworkResult<()>;
    
    /// Report test discovery completion
    fn report_discovery_complete(&self, test_count: usize) -> TestFrameworkResult<()>;
    
    /// Report no tests found
    fn report_no_tests_found(&self) -> TestFrameworkResult<()>;
    
    /// Report test execution start
    fn report_execution_start(&self, test_count: usize) -> TestFrameworkResult<()>;
    
    /// Report individual test start
    fn report_test_start(&self, test_info: &TestInfo) -> TestFrameworkResult<()>;
    
    /// Report individual test completion
    fn report_test_complete(&self, result: &TestResult) -> TestFrameworkResult<()>;
    
    /// Report test execution completion
    fn report_execution_complete(&self, report: &super::framework::TestFrameworkReport) -> TestFrameworkResult<()>;
    
    /// Generate final report
    fn generate_report(&self, result: &RunnerResult) -> TestFrameworkResult<String>;
    
    /// Get reporter configuration
    fn get_config(&self) -> &ReportConfig;
}

/// Console reporter implementation
pub struct ConsoleReporter {
    config: ReportConfig,
}

impl ConsoleReporter {
    /// Create a new console reporter with default configuration
    pub fn new() -> Self {
        Self::with_config(ReportConfig {
            format: ReportFormat::Console,
            ..ReportConfig::default()
        })
    }
    
    /// Create a new console reporter with custom configuration
    pub fn with_config(config: ReportConfig) -> Self {
        Self { config }
    }
    
    /// Format text with color if enabled
    fn colorize(&self, text: &str, color: ConsoleColor) -> String {
        if self.config.use_colors {
            format!("{}{}{}", color.code(), text, ConsoleColor::Reset.code())
        } else {
            text.to_string()
        }
    }
    
    /// Print a separator line
    fn print_separator(&self) {
        println!("{}", "=".repeat(self.config.console_width));
    }
    
    /// Format duration for display
    fn format_duration(&self, duration: Duration) -> String {
        if duration < Duration::from_millis(1) {
            format!("{:.3}μs", duration.as_micros())
        } else if duration < Duration::from_secs(1) {
            format!("{:.3}ms", duration.as_millis())
        } else {
            format!("{:.3}s", duration.as_secs_f64())
        }
    }
}

impl TestReporter for ConsoleReporter {
    fn report_discovery_start(&self) -> TestFrameworkResult<()> {
        println!("{}", self.colorize("🔍 Discovering tests...", ConsoleColor::Blue));
        Ok(())
    }
    
    fn report_discovery_complete(&self, test_count: usize) -> TestFrameworkResult<()> {
        println!("{}", self.colorize(
            &format!("✓ Found {} test(s)", test_count),
            ConsoleColor::Green
        ));
        Ok(())
    }
    
    fn report_no_tests_found(&self) -> TestFrameworkResult<()> {
        println!("{}", self.colorize("⚠ No tests found", ConsoleColor::Yellow));
        Ok(())
    }
    
    fn report_execution_start(&self, test_count: usize) -> TestFrameworkResult<()> {
        self.print_separator();
        println!("{}", self.colorize(
            &format!("🚀 Running {} test(s)...", test_count),
            ConsoleColor::Blue
        ));
        Ok(())
    }
    
    fn report_test_start(&self, test_info: &TestInfo) -> TestFrameworkResult<()> {
        if self.config.verbose {
            print!("  {} ... ", test_info.name);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        Ok(())
    }
    
    fn report_test_complete(&self, result: &TestResult) -> TestFrameworkResult<()> {
        if self.config.verbose {
            let status_text = match &result.status {
                TestStatus::Passed => self.colorize("PASS", ConsoleColor::Green),
                TestStatus::Failed(_) => self.colorize("FAIL", ConsoleColor::Red),
                TestStatus::Ignored => self.colorize("IGNORE", ConsoleColor::Yellow),
                TestStatus::Skipped => self.colorize("SKIP", ConsoleColor::Yellow),
                TestStatus::Timeout => self.colorize("TIMEOUT", ConsoleColor::Red),
            };
            
            let timing = if self.config.show_timing {
                format!(" ({})", self.format_duration(result.execution_time))
            } else {
                String::new()
            };
            
            println!("{}{}", status_text, timing);
        } else {
            // Print a simple character indicator
            let indicator = match &result.status {
                TestStatus::Passed => self.colorize(".", ConsoleColor::Green),
                TestStatus::Failed(_) => self.colorize("F", ConsoleColor::Red),
                TestStatus::Ignored => self.colorize("I", ConsoleColor::Yellow),
                TestStatus::Skipped => self.colorize("S", ConsoleColor::Yellow),
                TestStatus::Timeout => self.colorize("T", ConsoleColor::Red),
            };
            print!("{}", indicator);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        Ok(())
    }
    
    fn report_execution_complete(&self, report: &super::framework::TestFrameworkReport) -> TestFrameworkResult<()> {
        if !self.config.verbose {
            println!(); // New line after indicators
        }
        
        self.print_separator();
        
        // Print summary
        let summary_color = if report.is_success() {
            ConsoleColor::Green
        } else {
            ConsoleColor::Red
        };
        
        println!("{}", self.colorize("📊 Test Results:", ConsoleColor::Bold));
        println!("  Total:   {}", report.tests_executed);
        println!("  {}", self.colorize(&format!("Passed:  {}", report.tests_passed), ConsoleColor::Green));
        println!("  {}", self.colorize(&format!("Failed:  {}", report.tests_failed), ConsoleColor::Red));
        println!("  {}", self.colorize(&format!("Ignored: {}", report.tests_ignored), ConsoleColor::Yellow));
        
        if self.config.show_timing {
            println!("  Time:    {}", self.format_duration(report.total_time));
            println!("  Success: {:.1}%", report.success_rate());
        }
        
        // Print failures
        if !report.failures.is_empty() {
            println!();
            println!("{}", self.colorize("❌ Failures:", ConsoleColor::Red));
            
            for (index, failure) in report.failures.iter().enumerate() {
                println!();
                println!("{}. {}", index + 1, self.colorize(&failure.test_info.name, ConsoleColor::Bold));
                
                if let TestStatus::Failed(ref message) = failure.status {
                    println!("   {}", message);
                }
                
                if self.config.show_stack_traces {
                    if let Some(ref output) = failure.output {
                        if !output.trim().is_empty() {
                            println!("   Output: {}", output.trim());
                        }
                    }
                }
                
                println!("   Location: {}:{}", 
                    failure.test_info.file_path.display(), 
                    failure.test_info.line_number
                );
            }
        }
        
        // Final status
        println!();
        let final_message = if report.is_success() {
            self.colorize("✅ All tests passed!", ConsoleColor::Green)
        } else {
            self.colorize(&format!("❌ {} test(s) failed", report.tests_failed), ConsoleColor::Red)
        };
        println!("{}", final_message);
        
        Ok(())
    }
    
    fn generate_report(&self, result: &RunnerResult) -> TestFrameworkResult<String> {
        let mut report = String::new();
        
        writeln!(report, "CURSED Test Report").unwrap();
        writeln!(report, "==================").unwrap();
        writeln!(report).unwrap();
        
        writeln!(report, "Summary:").unwrap();
        writeln!(report, "  {}", result.summary.format()).unwrap();
        writeln!(report).unwrap();
        
        if !result.test_results.is_empty() {
            writeln!(report, "Test Results:").unwrap();
            for test_result in &result.test_results {
                let status = match &test_result.status {
                    TestStatus::Passed => "PASS",
                    TestStatus::Failed(_) => "FAIL",
                    TestStatus::Ignored => "IGNORE",
                    TestStatus::Skipped => "SKIP",
                    TestStatus::Timeout => "TIMEOUT",
                };
                
                writeln!(report, "  {} - {} ({:.3}s)", 
                    status, 
                    test_result.test_info.name,
                    test_result.execution_time.as_secs_f64()
                ).unwrap();
            }
        }
        
        Ok(report)
    }
    
    fn get_config(&self) -> &ReportConfig {
        &self.config
    }
}

/// JSON reporter implementation
pub struct JsonReporter {
    config: ReportConfig,
}

impl JsonReporter {
    pub fn new() -> Self {
        Self::with_config(ReportConfig {
            format: ReportFormat::Json,
            ..ReportConfig::default()
        })
    }
    
    pub fn with_config(config: ReportConfig) -> Self {
        Self { config }
    }
}

impl TestReporter for JsonReporter {
    fn report_discovery_start(&self) -> TestFrameworkResult<()> {
        // JSON reporter doesn't output during execution
        Ok(())
    }
    
    fn report_discovery_complete(&self, _test_count: usize) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_no_tests_found(&self) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_execution_start(&self, _test_count: usize) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_test_start(&self, _test_info: &TestInfo) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_test_complete(&self, _result: &TestResult) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_execution_complete(&self, _report: &super::framework::TestFrameworkReport) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn generate_report(&self, result: &RunnerResult) -> TestFrameworkResult<String> {
        let report_data = serde_json::json!({
            "summary": {
                "total_tests": result.summary.total_tests,
                "passed": result.summary.passed,
                "failed": result.summary.failed,
                "ignored": result.summary.ignored,
                "success_rate": result.summary.success_rate,
                "total_time_secs": result.summary.total_time.as_secs_f64(),
                "average_time_secs": result.summary.average_time.as_secs_f64()
            },
            "test_results": result.test_results.iter().map(|test| {
                serde_json::json!({
                    "name": test.test_info.name,
                    "status": match &test.status {
                        TestStatus::Passed => "passed",
                        TestStatus::Failed(_) => "failed",
                        TestStatus::Ignored => "ignored",
                        TestStatus::Skipped => "skipped",
                        TestStatus::Timeout => "timeout"
                    },
                    "execution_time_secs": test.execution_time.as_secs_f64(),
                    "file_path": test.test_info.file_path,
                    "line_number": test.test_info.line_number,
                    "failure_message": test.status.failure_message()
                })
            }).collect::<Vec<_>>()
        });
        
        serde_json::to_string_pretty(&report_data)
            .map_err(|e| TestError::ReportError(format!("Failed to serialize JSON report: {}", e)).into())
    }
    
    fn get_config(&self) -> &ReportConfig {
        &self.config
    }
}

/// XML reporter implementation
pub struct XmlReporter {
    config: ReportConfig,
}

impl XmlReporter {
    pub fn new() -> Self {
        Self::with_config(ReportConfig {
            format: ReportFormat::Xml,
            ..ReportConfig::default()
        })
    }
    
    pub fn with_config(config: ReportConfig) -> Self {
        Self { config }
    }
}

impl TestReporter for XmlReporter {
    fn report_discovery_start(&self) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_discovery_complete(&self, _test_count: usize) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_no_tests_found(&self) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_execution_start(&self, _test_count: usize) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_test_start(&self, _test_info: &TestInfo) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_test_complete(&self, _result: &TestResult) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_execution_complete(&self, _report: &super::framework::TestFrameworkReport) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn generate_report(&self, result: &RunnerResult) -> TestFrameworkResult<String> {
        let mut xml = String::new();
        
        writeln!(xml, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
        writeln!(xml, r#"<testsuites tests="{}" failures="{}" time="{:.3}">"#,
            result.summary.total_tests,
            result.summary.failed,
            result.summary.total_time.as_secs_f64()
        ).unwrap();
        
        writeln!(xml, r#"  <testsuite name="CURSED Tests" tests="{}" failures="{}" time="{:.3}">"#,
            result.summary.total_tests,
            result.summary.failed,
            result.summary.total_time.as_secs_f64()
        ).unwrap();
        
        for test in &result.test_results {
            writeln!(xml, r#"    <testcase name="{}" time="{:.3}">"#,
                test.test_info.name,
                test.execution_time.as_secs_f64()
            ).unwrap();
            
            if let TestStatus::Failed(ref message) = test.status {
                writeln!(xml, r#"      <failure message="{}">{}</failure>"#,
                    message.replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;"),
                    message.replace('<', "&lt;").replace('>', "&gt;")
                ).unwrap();
            }
            
            writeln!(xml, "    </testcase>").unwrap();
        }
        
        writeln!(xml, "  </testsuite>").unwrap();
        writeln!(xml, "</testsuites>").unwrap();
        
        Ok(xml)
    }
    
    fn get_config(&self) -> &ReportConfig {
        &self.config
    }
}

/// HTML reporter implementation
pub struct HtmlReporter {
    config: ReportConfig,
}

impl HtmlReporter {
    pub fn new() -> Self {
        Self::with_config(ReportConfig {
            format: ReportFormat::Html,
            ..ReportConfig::default()
        })
    }
    
    pub fn with_config(config: ReportConfig) -> Self {
        Self { config }
    }
}

impl TestReporter for HtmlReporter {
    fn report_discovery_start(&self) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_discovery_complete(&self, _test_count: usize) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_no_tests_found(&self) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_execution_start(&self, _test_count: usize) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_test_start(&self, _test_info: &TestInfo) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_test_complete(&self, _result: &TestResult) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn report_execution_complete(&self, _report: &super::framework::TestFrameworkReport) -> TestFrameworkResult<()> {
        Ok(())
    }
    
    fn generate_report(&self, result: &RunnerResult) -> TestFrameworkResult<String> {
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
        table { border-collapse: collapse; width: 100%; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
    </style>
</head>
<body>
    <h1>CURSED Test Report</h1>
"#);
        
        writeln!(html, r#"    <div class="summary">"#).unwrap();
        writeln!(html, "        <h2>Summary</h2>").unwrap();
        writeln!(html, "        <p><strong>Total Tests:</strong> {}</p>", result.summary.total_tests).unwrap();
        writeln!(html, r#"        <p class="passed"><strong>Passed:</strong> {}</p>"#, result.summary.passed).unwrap();
        writeln!(html, r#"        <p class="failed"><strong>Failed:</strong> {}</p>"#, result.summary.failed).unwrap();
        writeln!(html, r#"        <p class="ignored"><strong>Ignored:</strong> {}</p>"#, result.summary.ignored).unwrap();
        writeln!(html, "        <p><strong>Success Rate:</strong> {:.1}%</p>", result.summary.success_rate).unwrap();
        writeln!(html, "        <p><strong>Total Time:</strong> {:.3}s</p>", result.summary.total_time.as_secs_f64()).unwrap();
        writeln!(html, "    </div>").unwrap();
        
        writeln!(html, "    <h2>Test Results</h2>").unwrap();
        writeln!(html, "    <table>").unwrap();
        writeln!(html, "        <tr><th>Test Name</th><th>Status</th><th>Time</th><th>File</th></tr>").unwrap();
        
        for test in &result.test_results {
            let status_class = match &test.status {
                TestStatus::Passed => "passed",
                TestStatus::Failed(_) => "failed",
                TestStatus::Ignored => "ignored",
                _ => "",
            };
            
            let status_text = match &test.status {
                TestStatus::Passed => "PASSED",
                TestStatus::Failed(_) => "FAILED",
                TestStatus::Ignored => "IGNORED",
                TestStatus::Skipped => "SKIPPED",
                TestStatus::Timeout => "TIMEOUT",
            };
            
            writeln!(html, r#"        <tr>"#).unwrap();
            writeln!(html, r#"            <td>{}</td>"#, test.test_info.name).unwrap();
            writeln!(html, r#"            <td class="{}">{}</td>"#, status_class, status_text).unwrap();
            writeln!(html, r#"            <td>{:.3}s</td>"#, test.execution_time.as_secs_f64()).unwrap();
            writeln!(html, r#"            <td>{}:{}</td>"#, test.test_info.file_path.display(), test.test_info.line_number).unwrap();
            writeln!(html, "        </tr>").unwrap();
        }
        
        writeln!(html, "    </table>").unwrap();
        writeln!(html, "</body>").unwrap();
        writeln!(html, "</html>").unwrap();
        
        Ok(html)
    }
    
    fn get_config(&self) -> &ReportConfig {
        &self.config
    }
}

/// Console color codes
#[derive(Debug, Clone)]
enum ConsoleColor {
    Red,
    Green,
    Yellow,
    Blue,
    Bold,
    Reset,
}

impl ConsoleColor {
    fn code(&self) -> &'static str {
        match self {
            ConsoleColor::Red => "\x1b[31m",
            ConsoleColor::Green => "\x1b[32m",
            ConsoleColor::Yellow => "\x1b[33m",
            ConsoleColor::Blue => "\x1b[34m",
            ConsoleColor::Bold => "\x1b[1m",
            ConsoleColor::Reset => "\x1b[0m",
        }
    }
}
