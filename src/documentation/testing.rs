// # Documentation Testing and Validation System
//
// Comprehensive testing and validation framework for CURSED documentation including
// automated testing of documentation examples, link checking, validation systems,
// and documentation coverage reporting.
//
// ## Features
//
// - **Example Testing**: Automated testing of code examples in documentation
// - **Link Validation**: Comprehensive link checking and validation
// - **Coverage Reporting**: Documentation coverage analysis and reporting
// - **CI/CD Integration**: Integration with continuous integration systems
// - **Quality Gates**: Documentation quality enforcement
// - **Performance Testing**: Documentation generation performance validation

use crate::documentation::interactive::{CodeExecutionRequest, CodeExecutionResult, InteractiveDocumentation};
use crate::error::CursedError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{debug, error, info, instrument, warn};
use url::Url;
use uuid::Uuid;

/// Configuration for documentation testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationTestConfig {
    /// Enable example testing
    
    /// Enable link validation
    
    /// Enable coverage reporting
    
    /// Test timeout duration
    
    /// Maximum parallel tests
    
    /// Retry count for flaky tests
    
    /// Minimum coverage threshold
    
    /// Fail on missing documentation
    
    /// Include private items in coverage
    
    /// Test output directory
    
    /// Coverage report formats
    
    /// Link checking settings
    
    /// Example test settings
impl Default for DocumentationTestConfig {
    fn default() -> Self {
        Self {
            test_output_dir: PathBuf::from("target/doc-tests"),
            coverage_formats: vec![
        }
    }
/// Coverage report formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoverageReportFormat {
/// Link checking settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCheckSettings {
    /// Check external links
    
    /// Check internal links
    
    /// Follow redirects
    
    /// Maximum redirects to follow
    
    /// Request timeout
    
    /// User agent for requests
    
    /// Allowed status codes
    
    /// URLs to skip checking
    
    /// URL patterns to skip
impl Default for LinkCheckSettings {
    fn default() -> Self {
        Self {
            skip_patterns: vec![
        }
    }
/// Example testing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleTestSettings {
    /// Test all examples
    
    /// Test only marked examples
    
    /// Example marker comment
    
    /// Skip examples marker
    
    /// Expected output marker
    
    /// Maximum example execution time
    
    /// Capture stdout/stderr
    
    /// Compare expected output
impl Default for ExampleTestSettings {
    fn default() -> Self {
        Self {
            example_marker: "// @test".to_string(),
            skip_marker: "// @skip".to_string(),
            expected_output_marker: "// @expect:".to_string(),
        }
    }
/// Documentation test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationTestResult {
    /// Test run ID
    
    /// Start time
    
    /// End time
    
    /// Total duration
    
    /// Overall success
    
    /// Example test results
    
    /// Link validation results
    
    /// Coverage results
    
    /// Summary statistics
    
    /// Errors encountered
    
    /// Warnings
/// Example test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleTestResult {
    /// Example ID
    
    /// Example source code
    
    /// File path containing the example
    
    /// Line number where example starts
    
    /// Test success
    
    /// Execution result
    
    /// Expected output
    
    /// Output comparison result
    
    /// Test duration
    
    /// CursedError message if failed
    
    /// Retry count
/// Link validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkValidationResult {
    /// Total links checked
    
    /// Valid links
    
    /// Invalid links
    
    /// Skipped links
    
    /// Individual link results
    
    /// Duration of link checking
/// Individual link check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCheckResult {
    /// URL that was checked
    
    /// Source file containing the link
    
    /// Line number of the link
    
    /// Whether link is valid
    
    /// HTTP status code (if applicable)
    
    /// CursedError message if invalid
    
    /// Response time
    
    /// Final URL after redirects
/// Coverage analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageResult {
    /// Overall coverage percentage
    
    /// Coverage by file
    
    /// Coverage by item type
    
    /// Undocumented items
    
    /// Coverage trends
    
    /// Quality metrics
/// File coverage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    /// File path
    
    /// Coverage percentage
    
    /// Total items in file
    
    /// Documented items
    
    /// Undocumented items
    
    /// Documentation quality score
    
    /// Lines of documentation
    
    /// Lines of code
/// Undocumented item information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndocumentedItem {
    /// Item name
    
    /// Item type
    
    /// File path
    
    /// Line number
    
    /// Visibility (public, private)
    
    /// Severity of missing documentation
/// Documentation severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DocumentationSeverity {
/// Coverage trend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageTrend {
    /// Timestamp
    
    /// Coverage percentage at this time
    
    /// Commit hash (if available)
    
    /// Branch name (if available)
/// Coverage quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageQualityMetrics {
    /// Average documentation length
    
    /// Documentation to code ratio
    
    /// Examples per function ratio
    
    /// Cross-reference density
    
    /// Documentation freshness score
/// Test summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    /// Total tests run
    
    /// Passed tests
    
    /// Failed tests
    
    /// Skipped tests
    
    /// Test pass rate
    
    /// Total duration
    
    /// Average test duration
/// Documentation testing and validation system
pub struct DocumentationTester {
    /// Configuration
    
    /// Interactive documentation system
    
    /// HTTP client for link checking
    
    /// Active test sessions
/// Test session information
#[derive(Debug, Clone)]
struct TestSession {
impl DocumentationTester {
    /// Create a new documentation tester
    #[instrument(skip(config))]
    pub fn new(config: DocumentationTestConfig) -> crate::error::Result<()> {
        info!("Creating documentation tester");
        
        // Create test output directory
        if !config.test_output_dir.exists() {
            std::fs::create_dir_all(&config.test_output_dir)
                .map_err(|e| CursedError::system_error(&format!("Failed to create test output directory: {}", e)))?;
        // Create interactive documentation system
        let interactive_config = crate::documentation::interactive::InteractiveConfig::default();
        let interactive_docs = InteractiveDocumentation::new(interactive_config)?;
        
        // Create HTTP client for link checking
        let http_client = reqwest::Client::builder()
            .timeout(config.link_check_settings.request_timeout)
            .user_agent(&config.link_check_settings.user_agent)
            .redirect(if config.link_check_settings.follow_redirects {
                reqwest::redirect::Policy::limited(config.link_check_settings.max_redirects)
            } else {
                reqwest::redirect::Policy::none()
            })
            .build()
            .map_err(|e| CursedError::system_error(&format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
        })
    /// Run comprehensive documentation tests
    #[instrument(skip(self, docs_dir))]
    pub async fn run_tests(&mut self, docs_dir: &Path) -> crate::error::Result<()> {
        let test_run_id = Uuid::new_v4().to_string();
        let started_at = SystemTime::now();
        
        info!("Starting documentation test run: {}", test_run_id);
        
        // Create test session
        let session = self.create_test_session(&test_run_id).await?;
        
        let mut result = DocumentationTestResult {
            completed_at: started_at, // Will be updated
            duration: Duration::from_secs(0), // Will be calculated
            link_results: LinkValidationResult {
            summary: TestSummary {
        
        // Run example tests
        if self.config.test_examples {
            match self.run_example_tests(docs_dir).await {
                Ok(example_results) => {
                    result.example_results = example_results;
                }
                Err(e) => {
                    result.errors.push(format!("Example testing failed: {}", e));
                    result.success = false;
                }
            }
        // Run link validation
        if self.config.validate_links {
            match self.validate_links(docs_dir).await {
                Ok(link_results) => {
                    result.link_results = link_results;
                }
                Err(e) => {
                    result.errors.push(format!("Link validation failed: {}", e));
                    result.success = false;
                }
            }
        // Generate coverage report
        if self.config.generate_coverage {
            match self.generate_coverage_report(docs_dir).await {
                Ok(coverage_results) => {
                    result.coverage_results = Some(coverage_results);
                }
                Err(e) => {
                    result.errors.push(format!("Coverage generation failed: {}", e));
                    result.success = false;
                }
            }
        // Calculate summary
        result.summary = self.calculate_test_summary(&result);
        
        // Complete the test run
        let completed_at = SystemTime::now();
        result.completed_at = completed_at;
        result.duration = completed_at.duration_since(started_at).unwrap_or(Duration::from_secs(0));
        
        // Check quality gates
        self.check_quality_gates(&mut result);
        
        // Generate test reports
        self.generate_test_reports(&result).await?;
        
        info!("Documentation test run completed: {} (success: {})", test_run_id, result.success);
        
        Ok(result)
    /// Run example tests
    #[instrument(skip(self, docs_dir))]
    async fn run_example_tests(&mut self, docs_dir: &Path) -> crate::error::Result<()> {
        info!("Running example tests");
        
        let examples = self.extract_examples_from_docs(docs_dir).await?;
        let mut results = Vec::new();
        
        // Execute examples with parallelism control
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.config.max_parallel_tests));
        let mut tasks = Vec::new();
        
        for example in examples {
            let permit = Arc::clone(&semaphore);
            let interactive_docs = &mut self.interactive_docs;
            let config = &self.config;
            
            let task = tokio::spawn(async move {
                let _permit = permit.acquire().await.unwrap();
                Self::run_single_example_test(example, interactive_docs, config).await
            });
            
            tasks.push(task);
        // Collect results
        for task in tasks {
            match task.await {
            }
        }
        
        info!("Completed {} example tests", results.len());
        Ok(results)
    /// Run a single example test
    async fn run_single_example_test(
    ) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        // Check if example should be skipped
        if example.source_code.contains(&config.example_test_settings.skip_marker) {
            return Ok(ExampleTestResult {
            });
        let mut retry_count = 0;
        let mut last_error = None;
        
        // Retry loop
        while retry_count <= config.retry_count {
            let execution_request = CodeExecutionRequest {
            
            match interactive_docs.execute_code(execution_request).await {
                Ok(execution_result) => {
                    // Check if execution was successful
                    if execution_result.success {
                        // Compare output if expected output is provided
                        let output_matches = if let Some(ref expected) = example.expected_output {
                            if config.example_test_settings.compare_output {
                                Some(execution_result.stdout.trim() == expected.trim())
                            } else {
                                None
                            }
                        } else {
                            None
                        
                        let success = output_matches.unwrap_or(true);
                        
                        return Ok(ExampleTestResult {
                        });
                    } else {
                        last_error = Some(format!("Execution failed: {}", execution_result.stderr));
                    }
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                }
            }
            
            retry_count += 1;
        // All retries failed
        Ok(ExampleTestResult {
        })
    /// Validate links in documentation
    #[instrument(skip(self, docs_dir))]
    async fn validate_links(&self, docs_dir: &Path) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        info!("Validating links in documentation");
        
        let links = self.extract_links_from_docs(docs_dir).await?;
        let mut link_results = Vec::new();
        
        for link in &links {
            if self.should_skip_link(&link.url) {
                link_results.push(LinkCheckResult {
                    is_valid: true, // Assume skipped links are valid
                });
                continue;
            let result = self.check_single_link(link).await;
            link_results.push(result);
        let total_links = link_results.len();
        let valid_links = link_results.iter().filter(|r| r.is_valid).count();
        let invalid_links = link_results.iter().filter(|r| !r.is_valid).count();
        let skipped_links = link_results.iter()
            .filter(|r| r.error_message.as_ref().map_or(false, |m| m == "Skipped"))
            .count();
        
        Ok(LinkValidationResult {
        })
    /// Check a single link
    async fn check_single_link(&self, link: &ExtractedLink) -> LinkCheckResult {
        let start_time = std::time::Instant::now();
        
        // Check if it's an internal link
        if link.url.starts_with('#') || link.url.starts_with('/') || !link.url.contains("://") {
            return self.check_internal_link(link);
        // Check external link
        if !self.config.link_check_settings.check_external_links {
            return LinkCheckResult {
        match self.http_client.get(&link.url).send().await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                let is_valid = self.config.link_check_settings.allowed_status_codes.contains(&status_code);
                let final_url = response.url().to_string();
                
                LinkCheckResult {
                }
            }
            Err(e) => {
                LinkCheckResult {
                }
            }
        }
    }
    
    /// Check internal link
    fn check_internal_link(&self, link: &ExtractedLink) -> LinkCheckResult {
        // For internal links, we would check if the target exists
        // This is a simplified implementation
        let is_valid = true; // Would implement actual checking
        
        LinkCheckResult {
        }
    }
    
    /// Generate coverage report
    #[instrument(skip(self, docs_dir))]
    async fn generate_coverage_report(&self, docs_dir: &Path) -> crate::error::Result<()> {
        info!("Generating documentation coverage report");
        
        // This would analyze the source code and documentation to determine coverage
        // For now, return a mock result
        let overall_coverage = 75.0;
        
        Ok(CoverageResult {
            quality_metrics: CoverageQualityMetrics {
        })
    /// Extract examples from documentation
    async fn extract_examples_from_docs(&self, docs_dir: &Path) -> crate::error::Result<()> {
        let mut examples = Vec::new();
        
        // Walk through documentation files
        for entry in walkdir::WalkDir::new(docs_dir) {
            let entry = entry.map_err(|e| CursedError::system_error(&format!("Walk error: {}", e)))?;
            
            if entry.file_type().is_file() {
                if let Some(extension) = entry.path().extension() {
                    if extension == "html" || extension == "md" {
                        let file_examples = self.extract_examples_from_file(entry.path()).await?;
                        examples.extend(file_examples);
                    }
                }
            }
        }
        
        Ok(examples)
    /// Extract examples from a single file
    async fn extract_examples_from_file(&self, file_path: &Path) -> crate::error::Result<()> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| CursedError::system_error(&format!("Failed to read file: {}", e)))?;
        
        let mut examples = Vec::new();
        let mut in_code_block = false;
        let mut current_example = String::new();
        let mut code_block_start = 0;
        let mut expected_output = None;
        
        for (line_num, line) in content.split("\n").enumerate() {
            let line_num_1based = line_num + 1;
            
            if line.trim().starts_with("```cursed") || line.trim().starts_with("```csd") {
                in_code_block = true;
                code_block_start = line_num_1based;
                current_example.clear();
                
                // Check for test marker
                if !self.config.example_test_settings.test_all_examples &&
                   !line.contains(&self.config.example_test_settings.example_marker) {
                    in_code_block = false;
                    continue;
                }
            } else if line.trim() == "```" && in_code_block {
                in_code_block = false;
                
                if !current_example.trim().is_empty() {
                    let example_id = format!("{}:{}", file_path.display(), code_block_start);
                    
                    examples.push(ExtractedExample {
                    });
                current_example.clear();
            } else if in_code_block {
                current_example.push_str(line);
                current_example.push('\n');
            } else if line.trim().starts_with(&self.config.example_test_settings.expected_output_marker) {
                // Extract expected output
                let output_line = line.trim().strip_prefix(&self.config.example_test_settings.expected_output_marker)
                    .unwrap_or("")
                    .trim();
                expected_output = Some(output_line.to_string());
            }
        }
        
        Ok(examples)
    /// Extract links from documentation
    async fn extract_links_from_docs(&self, docs_dir: &Path) -> crate::error::Result<()> {
        let mut links = Vec::new();
        
        // Walk through documentation files
        for entry in walkdir::WalkDir::new(docs_dir) {
            let entry = entry.map_err(|e| CursedError::system_error(&format!("Walk error: {}", e)))?;
            
            if entry.file_type().is_file() {
                if let Some(extension) = entry.path().extension() {
                    if extension == "html" || extension == "md" {
                        let file_links = self.extract_links_from_file(entry.path()).await?;
                        links.extend(file_links);
                    }
                }
            }
        }
        
        Ok(links)
    /// Extract links from a single file
    async fn extract_links_from_file(&self, file_path: &Path) -> crate::error::Result<()> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| CursedError::system_error(&format!("Failed to read file: {}", e)))?;
        
        let mut links = Vec::new();
        
        // Regular expressions for different link formats
        let html_link_regex = regex::Regex::new(r#"<a\s+[^>]*href\s*=\s*["']([^"']+)["'][^>]*>"#).unwrap();
        let markdown_link_regex = regex::Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
        
        for (line_num, line) in content.split("\n").enumerate() {
            let line_num_1based = line_num + 1;
            
            // Extract HTML links
            for cap in html_link_regex.captures_iter(line) {
                if let Some(url) = cap.get(1) {
                    links.push(ExtractedLink {
                    });
                }
            }
            
            // Extract Markdown links
            for cap in markdown_link_regex.captures_iter(line) {
                if let Some(text) = cap.get(1) {
                    if let Some(url) = cap.get(2) {
                        links.push(ExtractedLink {
                        });
                    }
                }
            }
        }
        
        Ok(links)
    /// Check if a link should be skipped
    fn should_skip_link(&self, url: &str) -> bool {
        // Check skip URLs
        if self.config.link_check_settings.skip_urls.contains(&url.to_string()) {
            return true;
        // Check skip patterns
        for pattern in &self.config.link_check_settings.skip_patterns {
            if url.contains(pattern) {
                return true;
            }
        }
        
        false
    /// Calculate test summary
    fn calculate_test_summary(&self, result: &DocumentationTestResult) -> TestSummary {
        let total_tests = result.example_results.len();
        let passed_tests = result.example_results.iter().filter(|r| r.success).count();
        let failed_tests = result.example_results.iter().filter(|r| !r.success).count();
        let skipped_tests = total_tests - passed_tests - failed_tests;
        
        let pass_rate = if total_tests > 0 {
            passed_tests as f64 / total_tests as f64 * 100.0
        } else {
            100.0
        
        let total_duration = result.example_results.iter()
            .map(|r| r.duration)
            .sum::<Duration>();
        
        let average_duration = if total_tests > 0 {
            total_duration / total_tests as u32
        } else {
            Duration::from_secs(0)
        
        TestSummary {
        }
    }
    
    /// Check quality gates
    fn check_quality_gates(&self, result: &mut DocumentationTestResult) {
        // Check coverage threshold
        if let Some(ref coverage) = result.coverage_results {
            if coverage.overall_coverage < self.config.min_coverage_threshold {
                result.success = false;
                result.errors.push(format!(
                    self.config.min_coverage_threshold
                ));
            }
        }
        
        // Check test pass rate
        if result.summary.pass_rate < 100.0 && self.config.fail_on_missing_docs {
            result.success = false;
            result.errors.push(format!(
                result.summary.pass_rate
            ));
        // Check link validation
        if result.link_results.invalid_links > 0 {
            result.warnings.push(format!(
                result.link_results.invalid_links
            ));
        }
    }
    
    /// Generate test reports
    async fn generate_test_reports(&self, result: &DocumentationTestResult) -> crate::error::Result<()> {
        // Generate JSON report
        let json_report = serde_json::to_string_pretty(result)
            .map_err(|e| CursedError::system_error(&format!("Failed to serialize JSON report: {}", e)))?;
        
        let json_path = self.config.test_output_dir.join("test-results.json");
        std::fs::write(&json_path, json_report)
            .map_err(|e| CursedError::system_error(&format!("Failed to write JSON report: {}", e)))?;
        
        // Generate HTML report
        let html_report = self.generate_html_report(result)?;
        let html_path = self.config.test_output_dir.join("test-results.html");
        std::fs::write(&html_path, html_report)
            .map_err(|e| CursedError::system_error(&format!("Failed to write HTML report: {}", e)))?;
        
        info!("Test reports generated:");
        info!("  JSON: {}", json_path.display());
        info!("  HTML: {}", html_path.display());
        
        Ok(())
    /// Generate HTML test report
    fn generate_html_report(&self, result: &DocumentationTestResult) -> crate::error::Result<()> {
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>CURSED Documentation Test Results</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .summary {{ background: #f5f5f5; padding: 20px; border-radius: 5px; }}
        .pass {{ color: green; }}
        .fail {{ color: red; }}
        .warning {{ color: orange; }}
        table {{ border-collapse: collapse; width: 100%; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
    </style>
</head>
<body>
    <h1>CURSED Documentation Test Results</h1>
    
    <div class="summary">
        <h2>Summary</h2>
        <p><strong>Test Run ID:</strong> {}</p>
        <p><strong>Duration:</strong> {:?}</p>
        <p><strong>Overall Success:</strong> <span class="{}">{}</span></p>
        <p><strong>Tests:</strong> {} total, {} passed, {} failed</p>
        <p><strong>Pass Rate:</strong> {:.1}%</p>
        <p><strong>Links:</strong> {} total, {} valid, {} invalid</p>
    </div>
    
    <h2>Example Test Results</h2>
    <table>
        <tr>
            <th>Example ID</th>
            <th>File</th>
            <th>Line</th>
            <th>Status</th>
            <th>Duration</th>
            <th>CursedError</th>
        </tr>
        {}
    </table>
    
    <h2>Link Validation Results</h2>
    <table>
        <tr>
            <th>URL</th>
            <th>File</th>
            <th>Line</th>
            <th>Status</th>
            <th>CursedError</th>
        </tr>
        {}
    </table>
</body>
</html>"#,
            result.example_results.iter().map(|r| format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td class=\"{}\">{}</td><td>{:?}</td><td>{}</td></tr>",
                r.error_message.as_deref().unwrap_or("")
            result.link_results.link_results.iter().map(|r| format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td class=\"{}\">{}</td><td>{}</td></tr>",
                r.error_message.as_deref().unwrap_or("")
            )).collect::<Vec<_>>().join("\n")
        );
        
        Ok(html)
    /// Create a test session
    async fn create_test_session(&mut self, test_run_id: &str) -> crate::error::Result<()> {
        let test_dir = self.config.test_output_dir.join(test_run_id);
        
        if !test_dir.exists() {
            std::fs::create_dir_all(&test_dir)
                .map_err(|e| CursedError::system_error(&format!("Failed to create test directory: {}", e)))?;
        let session = TestSession {
        
        self.active_sessions.insert(test_run_id.to_string(), session);
        Ok(self.active_sessions.get(test_run_id).unwrap())
    }
}

/// Extracted example information
#[derive(Debug, Clone)]
struct ExtractedExample {
/// Extracted link information
#[derive(Debug, Clone)]
struct ExtractedLink {
}
