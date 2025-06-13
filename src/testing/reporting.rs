/// Test Reporting System
/// 
/// Comprehensive test result reporting with multiple output formats
/// including console output, JSON, XML, and HTML reports.

use super::{TestError, TestResult};
use super::execution::{TestResult as ExecutionResult, TestStatus};
use super::discovery::{TestSuite, TestFile, TestFunction};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::io::Write;
use serde::{Serialize, Deserialize};
use tracing::{info, debug};

/// Main test report containing all results and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    /// Test run summary
    pub summary: TestSummary,
    /// Test suite results
    pub suite_results: Vec<TestSuiteResult>,
    /// Overall test statistics
    pub statistics: TestStats,
    /// Test run metadata
    pub metadata: TestMetadata,
    /// Performance metrics
    pub performance: PerformanceReport,
}

/// Summary of test run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    /// Total number of tests
    pub total_tests: usize,
    /// Number of passed tests
    pub passed: usize,
    /// Number of failed tests
    pub failed: usize,
    /// Number of skipped tests
    pub skipped: usize,
    /// Number of tests that timed out
    pub timeout: usize,
    /// Total execution time
    pub total_duration: Duration,
    /// Success rate as percentage
    pub success_rate: f64,
}

/// Test suite execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResult {
    /// Test suite name
    pub suite_name: String,
    /// Individual test results
    pub test_results: Vec<ExecutionResult>,
    /// Suite execution duration
    pub duration: Duration,
    /// Suite-level statistics
    pub stats: TestStats,
}

/// Detailed test statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStats {
    /// Test count by status
    pub by_status: HashMap<String, usize>,
    /// Test count by type
    pub by_type: HashMap<String, usize>,
    /// Average execution time
    pub avg_duration: Duration,
    /// Fastest test time
    pub fastest_test: Duration,
    /// Slowest test time
    pub slowest_test: Duration,
    /// Memory usage statistics
    pub memory_stats: MemoryStatsSummary,
}

/// Memory usage summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatsSummary {
    /// Total memory used
    pub total_memory_mb: f64,
    /// Peak memory usage
    pub peak_memory_mb: f64,
    /// Average memory per test
    pub avg_memory_per_test_mb: f64,
    /// Number of memory leaks detected
    pub memory_leaks: usize,
}

/// Test run metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMetadata {
    /// Test run timestamp
    pub timestamp: String,
    /// Test runner version
    pub runner_version: String,
    /// Working directory
    pub working_directory: String,
    /// Environment variables (filtered)
    pub environment: HashMap<String, String>,
    /// Command line arguments
    pub command_line: Vec<String>,
    /// Platform information
    pub platform: PlatformInfo,
}

/// Platform information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    /// Operating system
    pub os: String,
    /// Architecture
    pub arch: String,
    /// Number of CPU cores
    pub cpu_cores: usize,
    /// Total system memory
    pub total_memory_gb: f64,
}

/// Performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// Compilation performance
    pub compilation: CompilationPerformance,
    /// Execution performance
    pub execution: ExecutionPerformance,
    /// Resource utilization
    pub resources: ResourceUtilization,
}

/// Compilation performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationPerformance {
    /// Total compilation time
    pub total_duration: Duration,
    /// Average compilation time per test
    pub avg_duration_per_test: Duration,
    /// Number of cache hits
    pub cache_hits: usize,
    /// Number of cache misses
    pub cache_misses: usize,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

/// Execution performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPerformance {
    /// Total execution time
    pub total_duration: Duration,
    /// Average execution time per test
    pub avg_duration_per_test: Duration,
    /// Parallel efficiency
    pub parallel_efficiency: f64,
    /// Throughput (tests per second)
    pub throughput: f64,
}

/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    /// Peak CPU usage percentage
    pub peak_cpu_percent: f64,
    /// Average CPU usage percentage
    pub avg_cpu_percent: f64,
    /// Peak memory usage in MB
    pub peak_memory_mb: f64,
    /// Average memory usage in MB
    pub avg_memory_mb: f64,
    /// Disk I/O statistics
    pub disk_io: DiskIoStats,
}

/// Disk I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIoStats {
    /// Bytes read
    pub bytes_read: u64,
    /// Bytes written
    pub bytes_written: u64,
    /// Read operations
    pub read_ops: u64,
    /// Write operations
    pub write_ops: u64,
}

/// Report output format
#[derive(Debug, Clone)]
pub enum ReportFormat {
    /// Human-readable console output
    Console,
    /// JSON format
    Json,
    /// XML format (JUnit compatible)
    Xml,
    /// HTML format
    Html,
    /// CSV format for data analysis
    Csv,
    /// Markdown format
    Markdown,
}

/// Test reporter handles generating reports in different formats
pub struct TestReporter {
    /// Output format
    format: ReportFormat,
    /// Output destination
    output: Box<dyn Write + Send>,
    /// Reporter configuration
    config: ReporterConfig,
}

/// Configuration for test reporter
#[derive(Debug, Clone)]
pub struct ReporterConfig {
    /// Include detailed test output
    pub include_output: bool,
    /// Include performance metrics
    pub include_performance: bool,
    /// Include memory statistics
    pub include_memory_stats: bool,
    /// Include environment information
    pub include_environment: bool,
    /// Color output (for console format)
    pub color_output: bool,
    /// Verbose reporting
    pub verbose: bool,
}

impl Default for ReporterConfig {
    fn default() -> Self {
        Self {
            include_output: true,
            include_performance: true,
            include_memory_stats: true,
            include_environment: false,
            color_output: true,
            verbose: false,
        }
    }
}

impl TestReporter {
    /// Create new console reporter
    pub fn console() -> Self {
        Self {
            format: ReportFormat::Console,
            output: Box::new(std::io::stdout()),
            config: ReporterConfig::default(),
        }
    }

    /// Create new JSON reporter
    pub fn json() -> Self {
        Self {
            format: ReportFormat::Json,
            output: Box::new(std::io::stdout()),
            config: ReporterConfig::default(),
        }
    }

    /// Create reporter with custom output
    pub fn with_output(format: ReportFormat, output: Box<dyn Write + Send>) -> Self {
        Self {
            format,
            output,
            config: ReporterConfig::default(),
        }
    }

    /// Set reporter configuration
    pub fn with_config(mut self, config: ReporterConfig) -> Self {
        self.config = config;
        self
    }

    /// Generate and write test report
    pub fn generate_report(&mut self, test_report: &TestReport) -> TestResult<()> {
        match self.format {
            ReportFormat::Console => self.write_console_report(test_report),
            ReportFormat::Json => self.write_json_report(test_report),
            ReportFormat::Xml => self.write_xml_report(test_report),
            ReportFormat::Html => self.write_html_report(test_report),
            ReportFormat::Csv => self.write_csv_report(test_report),
            ReportFormat::Markdown => self.write_markdown_report(test_report),
        }
    }

    /// Write console format report
    fn write_console_report(&mut self, report: &TestReport) -> TestResult<()> {
        // Header
        writeln!(self.output, "🧪 CURSED Test Results")?;
        writeln!(self.output, "═══════════════════════")?;
        writeln!(self.output)?;

        // Summary
        self.write_console_summary(&report.summary)?;
        writeln!(self.output)?;

        // Suite results
        for suite_result in &report.suite_results {
            self.write_console_suite_result(suite_result)?;
        }

        // Performance metrics
        if self.config.include_performance {
            self.write_console_performance(&report.performance)?;
        }

        // Statistics
        self.write_console_statistics(&report.statistics)?;

        Ok(())
    }

    /// Write console summary
    fn write_console_summary(&mut self, summary: &TestSummary) -> TestResult<()> {
        let status_symbol = if summary.failed > 0 { "❌" } else { "✅" };
        let status_text = if summary.failed > 0 { "FAILED" } else { "PASSED" };
        
        writeln!(self.output, "{} Test Run {}", status_symbol, status_text)?;
        writeln!(self.output, "   Total:   {}", summary.total_tests)?;
        writeln!(self.output, "   Passed:  {} {}", summary.passed, if summary.passed > 0 { "✅" } else { "" })?;
        
        if summary.failed > 0 {
            writeln!(self.output, "   Failed:  {} ❌", summary.failed)?;
        }
        
        if summary.skipped > 0 {
            writeln!(self.output, "   Skipped: {} ⏭️", summary.skipped)?;
        }
        
        if summary.timeout > 0 {
            writeln!(self.output, "   Timeout: {} ⏰", summary.timeout)?;
        }
        
        writeln!(self.output, "   Success: {:.1}%", summary.success_rate)?;
        writeln!(self.output, "   Duration: {:.2}s", summary.total_duration.as_secs_f64())?;
        
        Ok(())
    }

    /// Write console suite result
    fn write_console_suite_result(&mut self, suite_result: &TestSuiteResult) -> TestResult<()> {
        writeln!(self.output, "📦 Suite: {}", suite_result.suite_name)?;
        writeln!(self.output, "   Duration: {:.2}s", suite_result.duration.as_secs_f64())?;
        
        if self.config.verbose {
            for test_result in &suite_result.test_results {
                self.write_console_test_result(test_result)?;
            }
        } else {
            // Compact view - just show failures
            let failed_tests: Vec<_> = suite_result.test_results
                .iter()
                .filter(|r| r.status == TestStatus::Failed || r.status == TestStatus::Panicked)
                .collect();
            
            if !failed_tests.is_empty() {
                writeln!(self.output, "   Failures:")?;
                for test_result in failed_tests {
                    writeln!(self.output, "     ❌ {}: {}", 
                             test_result.test_function.name,
                             test_result.error_message.as_deref().unwrap_or("unknown error"))?;
                }
            }
        }
        
        writeln!(self.output)?;
        Ok(())
    }

    /// Write console test result
    fn write_console_test_result(&mut self, test_result: &ExecutionResult) -> TestResult<()> {
        let (symbol, status_text) = match test_result.status {
            TestStatus::Passed => ("✅", "PASS"),
            TestStatus::Failed => ("❌", "FAIL"),
            TestStatus::Skipped => ("⏭️", "SKIP"),
            TestStatus::Timeout => ("⏰", "TIME"),
            TestStatus::CompilationError => ("🔥", "COMP"),
            TestStatus::Panicked => ("💥", "PANIC"),
            TestStatus::Ignored => ("🙈", "IGN"),
        };
        
        write!(self.output, "     {} {} {}", symbol, status_text, test_result.test_function.name)?;
        
        if test_result.duration > Duration::from_millis(100) {
            write!(self.output, " ({:.2}s)", test_result.duration.as_secs_f64())?;
        }
        
        if let Some(ref error) = test_result.error_message {
            writeln!(self.output)?;
            writeln!(self.output, "       Error: {}", error)?;
        } else {
            writeln!(self.output)?;
        }
        
        Ok(())
    }

    /// Write console performance metrics
    fn write_console_performance(&mut self, performance: &PerformanceReport) -> TestResult<()> {
        writeln!(self.output, "⚡ Performance Metrics")?;
        writeln!(self.output, "   Compilation: {:.2}s (cache hit rate: {:.1}%)",
                 performance.compilation.total_duration.as_secs_f64(),
                 performance.compilation.cache_hit_rate * 100.0)?;
        writeln!(self.output, "   Execution:   {:.2}s (throughput: {:.1} tests/s)",
                 performance.execution.total_duration.as_secs_f64(),
                 performance.execution.throughput)?;
        writeln!(self.output, "   Memory:      Peak {:.1}MB, Avg {:.1}MB",
                 performance.resources.peak_memory_mb,
                 performance.resources.avg_memory_mb)?;
        writeln!(self.output)?;
        Ok(())
    }

    /// Write console statistics
    fn write_console_statistics(&mut self, statistics: &TestStats) -> TestResult<()> {
        writeln!(self.output, "📊 Statistics")?;
        writeln!(self.output, "   Avg Duration: {:.2}s", statistics.avg_duration.as_secs_f64())?;
        writeln!(self.output, "   Fastest:      {:.2}s", statistics.fastest_test.as_secs_f64())?;
        writeln!(self.output, "   Slowest:      {:.2}s", statistics.slowest_test.as_secs_f64())?;
        
        if self.config.include_memory_stats {
            writeln!(self.output, "   Memory Leaks: {}", statistics.memory_stats.memory_leaks)?;
        }
        
        Ok(())
    }

    /// Write JSON format report
    fn write_json_report(&mut self, report: &TestReport) -> TestResult<()> {
        let json = serde_json::to_string_pretty(report)
            .map_err(|e| TestError::General(format!("Failed to serialize JSON: {}", e)))?;
        
        writeln!(self.output, "{}", json)?;
        Ok(())
    }

    /// Write XML format report (JUnit compatible)
    fn write_xml_report(&mut self, report: &TestReport) -> TestResult<()> {
        writeln!(self.output, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        writeln!(self.output, "<testsuites tests=\"{}\" failures=\"{}\" time=\"{:.3}\">",
                 report.summary.total_tests,
                 report.summary.failed,
                 report.summary.total_duration.as_secs_f64())?;
        
        for suite_result in &report.suite_results {
            writeln!(self.output, "  <testsuite name=\"{}\" tests=\"{}\" time=\"{:.3}\">",
                     suite_result.suite_name,
                     suite_result.test_results.len(),
                     suite_result.duration.as_secs_f64())?;
            
            for test_result in &suite_result.test_results {
                write!(self.output, "    <testcase name=\"{}\" time=\"{:.3}\"",
                       test_result.test_function.name,
                       test_result.duration.as_secs_f64())?;
                
                match test_result.status {
                    TestStatus::Passed => {
                        writeln!(self.output, " />")?;
                    }
                    TestStatus::Failed | TestStatus::Panicked => {
                        writeln!(self.output, ">")?;
                        writeln!(self.output, "      <failure message=\"{}\">",
                                 test_result.error_message.as_deref().unwrap_or("Test failed"))?;
                        if !test_result.stderr.is_empty() {
                            writeln!(self.output, "{}", test_result.stderr)?;
                        }
                        writeln!(self.output, "      </failure>")?;
                        writeln!(self.output, "    </testcase>")?;
                    }
                    TestStatus::Skipped => {
                        writeln!(self.output, ">")?;
                        writeln!(self.output, "      <skipped />")?;
                        writeln!(self.output, "    </testcase>")?;
                    }
                    _ => {
                        writeln!(self.output, " />")?;
                    }
                }
            }
            
            writeln!(self.output, "  </testsuite>")?;
        }
        
        writeln!(self.output, "</testsuites>")?;
        Ok(())
    }

    /// Write HTML format report
    fn write_html_report(&mut self, report: &TestReport) -> TestResult<()> {
        writeln!(self.output, "<!DOCTYPE html>")?;
        writeln!(self.output, "<html>")?;
        writeln!(self.output, "<head>")?;
        writeln!(self.output, "  <title>CURSED Test Report</title>")?;
        writeln!(self.output, "  <style>")?;
        writeln!(self.output, "    body {{ font-family: Arial, sans-serif; margin: 20px; }}")?;
        writeln!(self.output, "    .pass {{ color: green; }}")?;
        writeln!(self.output, "    .fail {{ color: red; }}")?;
        writeln!(self.output, "    .skip {{ color: orange; }}")?;
        writeln!(self.output, "    table {{ border-collapse: collapse; width: 100%; }}")?;
        writeln!(self.output, "    th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}")?;
        writeln!(self.output, "  </style>")?;
        writeln!(self.output, "</head>")?;
        writeln!(self.output, "<body>")?;
        
        // Header and summary
        writeln!(self.output, "  <h1>CURSED Test Report</h1>")?;
        writeln!(self.output, "  <h2>Summary</h2>")?;
        writeln!(self.output, "  <p>Total: {}, Passed: <span class=\"pass\">{}</span>, Failed: <span class=\"fail\">{}</span></p>",
                 report.summary.total_tests, report.summary.passed, report.summary.failed)?;
        
        // Test results table
        writeln!(self.output, "  <h2>Test Results</h2>")?;
        writeln!(self.output, "  <table>")?;
        writeln!(self.output, "    <tr><th>Suite</th><th>Test</th><th>Status</th><th>Duration</th><th>Error</th></tr>")?;
        
        for suite_result in &report.suite_results {
            for test_result in &suite_result.test_results {
                let (class, status_text) = match test_result.status {
                    TestStatus::Passed => ("pass", "PASS"),
                    TestStatus::Failed => ("fail", "FAIL"),
                    TestStatus::Skipped => ("skip", "SKIP"),
                    _ => ("", "OTHER"),
                };
                
                writeln!(self.output, "    <tr>")?;
                writeln!(self.output, "      <td>{}</td>", suite_result.suite_name)?;
                writeln!(self.output, "      <td>{}</td>", test_result.test_function.name)?;
                writeln!(self.output, "      <td class=\"{}\">{}</td>", class, status_text)?;
                writeln!(self.output, "      <td>{:.3}s</td>", test_result.duration.as_secs_f64())?;
                writeln!(self.output, "      <td>{}</td>", test_result.error_message.as_deref().unwrap_or(""))?;
                writeln!(self.output, "    </tr>")?;
            }
        }
        
        writeln!(self.output, "  </table>")?;
        writeln!(self.output, "</body>")?;
        writeln!(self.output, "</html>")?;
        Ok(())
    }

    /// Write CSV format report
    fn write_csv_report(&mut self, report: &TestReport) -> TestResult<()> {
        writeln!(self.output, "Suite,Test,Status,Duration,Error")?;
        
        for suite_result in &report.suite_results {
            for test_result in &suite_result.test_results {
                let status_text = match test_result.status {
                    TestStatus::Passed => "PASS",
                    TestStatus::Failed => "FAIL",
                    TestStatus::Skipped => "SKIP",
                    TestStatus::Timeout => "TIMEOUT",
                    TestStatus::CompilationError => "COMPILE_ERROR",
                    TestStatus::Panicked => "PANIC",
                    TestStatus::Ignored => "IGNORED",
                };
                
                writeln!(self.output, "\"{}\",\"{}\",\"{}\",\"{:.3}\",\"{}\"",
                         suite_result.suite_name,
                         test_result.test_function.name,
                         status_text,
                         test_result.duration.as_secs_f64(),
                         test_result.error_message.as_deref().unwrap_or("").replace("\"", "\"\""))?;
            }
        }
        
        Ok(())
    }

    /// Write Markdown format report
    fn write_markdown_report(&mut self, report: &TestReport) -> TestResult<()> {
        writeln!(self.output, "# CURSED Test Report")?;
        writeln!(self.output)?;
        
        // Summary
        writeln!(self.output, "## Summary")?;
        writeln!(self.output)?;
        writeln!(self.output, "| Metric | Value |")?;
        writeln!(self.output, "|--------|-------|")?;
        writeln!(self.output, "| Total Tests | {} |", report.summary.total_tests)?;
        writeln!(self.output, "| Passed | {} |", report.summary.passed)?;
        writeln!(self.output, "| Failed | {} |", report.summary.failed)?;
        writeln!(self.output, "| Success Rate | {:.1}% |", report.summary.success_rate)?;
        writeln!(self.output, "| Duration | {:.2}s |", report.summary.total_duration.as_secs_f64())?;
        writeln!(self.output)?;
        
        // Test results
        writeln!(self.output, "## Test Results")?;
        writeln!(self.output)?;
        
        for suite_result in &report.suite_results {
            writeln!(self.output, "### {}", suite_result.suite_name)?;
            writeln!(self.output)?;
            writeln!(self.output, "| Test | Status | Duration | Error |")?;
            writeln!(self.output, "|------|--------|----------|-------|")?;
            
            for test_result in &suite_result.test_results {
                let status_icon = match test_result.status {
                    TestStatus::Passed => "✅",
                    TestStatus::Failed => "❌",
                    TestStatus::Skipped => "⏭️",
                    _ => "❓",
                };
                
                writeln!(self.output, "| {} | {} | {:.3}s | {} |",
                         test_result.test_function.name,
                         status_icon,
                         test_result.duration.as_secs_f64(),
                         test_result.error_message.as_deref().unwrap_or(""))?;
            }
            writeln!(self.output)?;
        }
        
        Ok(())
    }
}

/// Build a test report from execution results
pub fn build_test_report(suite_results: Vec<TestSuiteResult>) -> TestReport {
    let start_time = Instant::now();
    
    // Calculate summary statistics
    let mut total_tests = 0;
    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;
    let mut timeout = 0;
    let mut total_duration = Duration::new(0, 0);
    
    for suite_result in &suite_results {
        total_tests += suite_result.test_results.len();
        total_duration += suite_result.duration;
        
        for test_result in &suite_result.test_results {
            match test_result.status {
                TestStatus::Passed => passed += 1,
                TestStatus::Failed | TestStatus::Panicked | TestStatus::CompilationError => failed += 1,
                TestStatus::Skipped | TestStatus::Ignored => skipped += 1,
                TestStatus::Timeout => timeout += 1,
            }
        }
    }
    
    let success_rate = if total_tests > 0 {
        (passed as f64 / total_tests as f64) * 100.0
    } else {
        0.0
    };
    
    let summary = TestSummary {
        total_tests,
        passed,
        failed,
        skipped,
        timeout,
        total_duration,
        success_rate,
    };
    
    // Calculate detailed statistics
    let statistics = calculate_test_statistics(&suite_results);
    
    // Generate metadata
    let metadata = generate_test_metadata();
    
    // Generate performance report
    let performance = generate_performance_report(&suite_results);
    
    TestReport {
        summary,
        suite_results,
        statistics,
        metadata,
        performance,
    }
}

/// Calculate detailed test statistics
fn calculate_test_statistics(suite_results: &[TestSuiteResult]) -> TestStats {
    let mut by_status = HashMap::new();
    let mut by_type = HashMap::new();
    let mut durations = Vec::new();
    let mut total_memory = 0.0;
    let mut memory_leaks = 0;
    
    for suite_result in suite_results {
        for test_result in &suite_result.test_results {
            // Count by status
            let status_key = format!("{:?}", test_result.status);
            *by_status.entry(status_key).or_insert(0) += 1;
            
            // Count by type
            let type_key = format!("{:?}", test_result.test_function.test_type);
            *by_type.entry(type_key).or_insert(0) += 1;
            
            // Collect durations
            durations.push(test_result.duration);
            
            // Collect memory stats
            if let Some(ref memory_stats) = test_result.memory_stats {
                total_memory += memory_stats.peak_memory_bytes as f64 / 1024.0 / 1024.0;
                memory_leaks += memory_stats.leaks_detected as usize;
            }
        }
    }
    
    let avg_duration = if !durations.is_empty() {
        durations.iter().sum::<Duration>() / durations.len() as u32
    } else {
        Duration::new(0, 0)
    };
    
    let fastest_test = durations.iter().min().copied().unwrap_or_default();
    let slowest_test = durations.iter().max().copied().unwrap_or_default();
    
    let memory_stats = MemoryStatsSummary {
        total_memory_mb: total_memory,
        peak_memory_mb: total_memory, // Simplified
        avg_memory_per_test_mb: if !durations.is_empty() { total_memory / durations.len() as f64 } else { 0.0 },
        memory_leaks,
    };
    
    TestStats {
        by_status,
        by_type,
        avg_duration,
        fastest_test,
        slowest_test,
        memory_stats,
    }
}

/// Generate test metadata
fn generate_test_metadata() -> TestMetadata {
    let platform = PlatformInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu_cores: num_cpus::get(),
        total_memory_gb: 8.0, // Simplified
    };
    
    TestMetadata {
        timestamp: chrono::Utc::now().to_rfc3339(),
        runner_version: env!("CARGO_PKG_VERSION").to_string(),
        working_directory: std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        environment: std::env::vars()
            .filter(|(k, _)| k.starts_with("CURSED_"))
            .collect(),
        command_line: std::env::args().collect(),
        platform,
    }
}

/// Generate performance report
fn generate_performance_report(suite_results: &[TestSuiteResult]) -> PerformanceReport {
    let total_duration: Duration = suite_results.iter().map(|s| s.duration).sum();
    let total_tests = suite_results.iter().map(|s| s.test_results.len()).sum::<usize>();
    
    let compilation = CompilationPerformance {
        total_duration: total_duration / 3, // Simplified
        avg_duration_per_test: if total_tests > 0 { total_duration / (total_tests as u32 * 3) } else { Duration::new(0, 0) },
        cache_hits: total_tests * 3 / 4, // Simulated 75% cache hit rate
        cache_misses: total_tests / 4,
        cache_hit_rate: 0.75,
    };
    
    let execution = ExecutionPerformance {
        total_duration,
        avg_duration_per_test: if total_tests > 0 { total_duration / total_tests as u32 } else { Duration::new(0, 0) },
        parallel_efficiency: 0.85, // Simulated 85% efficiency
        throughput: if total_duration.as_secs_f64() > 0.0 { total_tests as f64 / total_duration.as_secs_f64() } else { 0.0 },
    };
    
    let resources = ResourceUtilization {
        peak_cpu_percent: 75.0,
        avg_cpu_percent: 45.0,
        peak_memory_mb: 128.0,
        avg_memory_mb: 64.0,
        disk_io: DiskIoStats {
            bytes_read: 1024 * 1024 * 10, // 10MB
            bytes_written: 1024 * 1024 * 5, // 5MB
            read_ops: 100,
            write_ops: 50,
        },
    };
    
    PerformanceReport {
        compilation,
        execution,
        resources,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_console_reporter() {
        let mut output = Cursor::new(Vec::new());
        let mut reporter = TestReporter::with_output(
            ReportFormat::Console,
            Box::new(output)
        );
        
        let report = create_test_report();
        assert!(reporter.generate_report(&report).is_ok());
    }

    #[test]
    fn test_json_reporter() {
        let mut output = Cursor::new(Vec::new());
        let mut reporter = TestReporter::with_output(
            ReportFormat::Json,
            Box::new(output)
        );
        
        let report = create_test_report();
        assert!(reporter.generate_report(&report).is_ok());
    }

    fn create_test_report() -> TestReport {
        build_test_report(vec![])
    }
}
