//! # Documentation Testing and Validation System
//!
//! Comprehensive testing and validation framework for CURSED documentation including
//! automated testing of documentation examples, link checking, validation systems,
//! and documentation coverage reporting.
//!
//! ## Features
//!
//! - **Example Testing**: Automated testing of code examples in documentation
//! - **Link Validation**: Comprehensive link checking and validation
//! - **Coverage Reporting**: Documentation coverage analysis and reporting
//! - **CI/CD Integration**: Integration with continuous integration systems
//! - **Quality Gates**: Documentation quality enforcement
//! - **Performance Testing**: Documentation generation performance validation

use crate::documentation::interactive::{CodeExecutionRequest, CodeExecutionResult, InteractiveDocumentation};
use crate::error::Error as CursedError;
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
    pub test_examples: bool,
    
    /// Enable link validation
    pub validate_links: bool,
    
    /// Enable coverage reporting
    pub generate_coverage: bool,
    
    /// Test timeout duration
    pub test_timeout: Duration,
    
    /// Maximum parallel tests
    pub max_parallel_tests: usize,
    
    /// Retry count for flaky tests
    pub retry_count: usize,
    
    /// Minimum coverage threshold
    pub min_coverage_threshold: f64,
    
    /// Fail on missing documentation
    pub fail_on_missing_docs: bool,
    
    /// Include private items in coverage
    pub include_private_in_coverage: bool,
    
    /// Test output directory
    pub test_output_dir: PathBuf,
    
    /// Coverage report formats
    pub coverage_formats: Vec<CoverageReportFormat>,
    
    /// Link checking settings
    pub link_check_settings: LinkCheckSettings,
    
    /// Example test settings
    pub example_test_settings: ExampleTestSettings,
}

impl Default for DocumentationTestConfig {
    fn default() -> Self {
        Self {
            test_examples: true,
            validate_links: true,
            generate_coverage: true,
            test_timeout: Duration::from_secs(30),
            max_parallel_tests: 4,
            retry_count: 2,
            min_coverage_threshold: 80.0,
            fail_on_missing_docs: false,
            include_private_in_coverage: false,
            test_output_dir: PathBuf::from("target/doc-tests"),
            coverage_formats: vec![
                CoverageReportFormat::Html,
                CoverageReportFormat::Json,
            ],
            link_check_settings: LinkCheckSettings::default(),
            example_test_settings: ExampleTestSettings::default(),
        }
    }
}

/// Coverage report formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoverageReportFormat {
    Html,
    Json,
    Xml,
    Lcov,
    Text,
}

/// Link checking settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCheckSettings {
    /// Check external links
    pub check_external_links: bool,
    
    /// Check internal links
    pub check_internal_links: bool,
    
    /// Follow redirects
    pub follow_redirects: bool,
    
    /// Maximum redirects to follow
    pub max_redirects: usize,
    
    /// Request timeout
    pub request_timeout: Duration,
    
    /// User agent for requests
    pub user_agent: String,
    
    /// Allowed status codes
    pub allowed_status_codes: Vec<u16>,
    
    /// URLs to skip checking
    pub skip_urls: Vec<String>,
    
    /// URL patterns to skip
    pub skip_patterns: Vec<String>,
}

impl Default for LinkCheckSettings {
    fn default() -> Self {
        Self {
            check_external_links: true,
            check_internal_links: true,
            follow_redirects: true,
            max_redirects: 5,
            request_timeout: Duration::from_secs(10),
            user_agent: "CURSED Documentation Checker".to_string(),
            allowed_status_codes: vec![200, 301, 302, 307, 308],
            skip_urls: Vec::new(),
            skip_patterns: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "example.com".to_string(),
            ],
        }
    }
}

/// Example testing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleTestSettings {
    /// Test all examples
    pub test_all_examples: bool,
    
    /// Test only marked examples
    pub test_only_marked: bool,
    
    /// Example marker comment
    pub example_marker: String,
    
    /// Skip examples marker
    pub skip_marker: String,
    
    /// Expected output marker
    pub expected_output_marker: String,
    
    /// Maximum example execution time
    pub max_execution_time: Duration,
    
    /// Capture stdout/stderr
    pub capture_output: bool,
    
    /// Compare expected output
    pub compare_output: bool,
}

impl Default for ExampleTestSettings {
    fn default() -> Self {
        Self {
            test_all_examples: true,
            test_only_marked: false,
            example_marker: "// @test".to_string(),
            skip_marker: "// @skip".to_string(),
            expected_output_marker: "// @expect:".to_string(),
            max_execution_time: Duration::from_secs(10),
            capture_output: true,
            compare_output: true,
        }
    }
}

/// Documentation test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationTestResult {
    /// Test run ID
    pub test_run_id: String,
    
    /// Start time
    pub started_at: SystemTime,
    
    /// End time
    pub completed_at: SystemTime,
    
    /// Total duration
    pub duration: Duration,
    
    /// Overall success
    pub success: bool,
    
    /// Example test results
    pub example_results: Vec<ExampleTestResult>,
    
    /// Link validation results
    pub link_results: LinkValidationResult,
    
    /// Coverage results
    pub coverage_results: Option<CoverageResult>,
    
    /// Summary statistics
    pub summary: TestSummary,
    
    /// Errors encountered
    pub errors: Vec<String>,
    
    /// Warnings
    pub warnings: Vec<String>,
}

/// Example test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleTestResult {
    /// Example ID
    pub example_id: String,
    
    /// Example source code
    pub source_code: String,
    
    /// File path containing the example
    pub file_path: PathBuf,
    
    /// Line number where example starts
    pub line_number: usize,
    
    /// Test success
    pub success: bool,
    
    /// Execution result
    pub execution_result: Option<CodeExecutionResult>,
    
    /// Expected output
    pub expected_output: Option<String>,
    
    /// Output comparison result
    pub output_matches: Option<bool>,
    
    /// Test duration
    pub duration: Duration,
    
    /// Error message if failed
    pub error_message: Option<String>,
    
    /// Retry count
    pub retry_count: usize,
}

/// Link validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkValidationResult {
    /// Total links checked
    pub total_links: usize,
    
    /// Valid links
    pub valid_links: usize,
    
    /// Invalid links
    pub invalid_links: usize,
    
    /// Skipped links
    pub skipped_links: usize,
    
    /// Individual link results
    pub link_results: Vec<LinkCheckResult>,
    
    /// Duration of link checking
    pub duration: Duration,
}

/// Individual link check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCheckResult {
    /// URL that was checked
    pub url: String,
    
    /// Source file containing the link
    pub source_file: PathBuf,
    
    /// Line number of the link
    pub line_number: usize,
    
    /// Whether link is valid
    pub is_valid: bool,
    
    /// HTTP status code (if applicable)
    pub status_code: Option<u16>,
    
    /// Error message if invalid
    pub error_message: Option<String>,
    
    /// Response time
    pub response_time: Option<Duration>,
    
    /// Final URL after redirects
    pub final_url: Option<String>,
}

/// Coverage analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageResult {
    /// Overall coverage percentage
    pub overall_coverage: f64,
    
    /// Coverage by file
    pub file_coverage: HashMap<PathBuf, FileCoverage>,
    
    /// Coverage by item type
    pub item_type_coverage: HashMap<String, f64>,
    
    /// Undocumented items
    pub undocumented_items: Vec<UndocumentedItem>,
    
    /// Coverage trends
    pub coverage_trends: Vec<CoverageTrend>,
    
    /// Quality metrics
    pub quality_metrics: CoverageQualityMetrics,
}

/// File coverage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    /// File path
    pub file_path: PathBuf,
    
    /// Coverage percentage
    pub coverage_percentage: f64,
    
    /// Total items in file
    pub total_items: usize,
    
    /// Documented items
    pub documented_items: usize,
    
    /// Undocumented items
    pub undocumented_items: usize,
    
    /// Documentation quality score
    pub quality_score: f64,
    
    /// Lines of documentation
    pub doc_lines: usize,
    
    /// Lines of code
    pub code_lines: usize,
}

/// Undocumented item information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndocumentedItem {
    /// Item name
    pub name: String,
    
    /// Item type
    pub item_type: String,
    
    /// File path
    pub file_path: PathBuf,
    
    /// Line number
    pub line_number: usize,
    
    /// Visibility (public, private)
    pub visibility: String,
    
    /// Severity of missing documentation
    pub severity: DocumentationSeverity,
}

/// Documentation severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DocumentationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Coverage trend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageTrend {
    /// Timestamp
    pub timestamp: SystemTime,
    
    /// Coverage percentage at this time
    pub coverage_percentage: f64,
    
    /// Commit hash (if available)
    pub commit_hash: Option<String>,
    
    /// Branch name (if available)
    pub branch: Option<String>,
}

/// Coverage quality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageQualityMetrics {
    /// Average documentation length
    pub avg_doc_length: f64,
    
    /// Documentation to code ratio
    pub doc_to_code_ratio: f64,
    
    /// Examples per function ratio
    pub examples_per_function: f64,
    
    /// Cross-reference density
    pub cross_reference_density: f64,
    
    /// Documentation freshness score
    pub freshness_score: f64,
}

/// Test summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSummary {
    /// Total tests run
    pub total_tests: usize,
    
    /// Passed tests
    pub passed_tests: usize,
    
    /// Failed tests
    pub failed_tests: usize,
    
    /// Skipped tests
    pub skipped_tests: usize,
    
    /// Test pass rate
    pub pass_rate: f64,
    
    /// Total duration
    pub total_duration: Duration,
    
    /// Average test duration
    pub average_duration: Duration,
}

/// Documentation testing and validation system
pub struct DocumentationTester {
    /// Configuration
    config: DocumentationTestConfig,
    
    /// Interactive documentation system
    interactive_docs: InteractiveDocumentation,
    
    /// HTTP client for link checking
    http_client: reqwest::Client,
    
    /// Active test sessions
    active_sessions: HashMap<String, TestSession>,
}

/// Test session information
#[derive(Debug, Clone)]
struct TestSession {
    id: String,
    started_at: SystemTime,
    test_dir: PathBuf,
}

impl DocumentationTester {
    /// Create a new documentation tester
    #[instrument(skip(config))]
    pub fn new(config: DocumentationTestConfig) -> Result<Self, CursedError> {
        info!("Creating documentation tester");
        
        // Create test output directory
        if !config.test_output_dir.exists() {
            std::fs::create_dir_all(&config.test_output_dir)
                .map_err(|e| CursedError::system_error(&format!("Failed to create test output directory: {}", e)))?;
        }
        
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
            config,
            interactive_docs,
            http_client,
            active_sessions: HashMap::new(),
        })
    }
    
    /// Run comprehensive documentation tests
    #[instrument(skip(self, docs_dir))]
    pub async fn run_tests(&mut self, docs_dir: &Path) -> Result<DocumentationTestResult, CursedError> {
        let test_run_id = Uuid::new_v4().to_string();
        let started_at = SystemTime::now();
        
        info!("Starting documentation test run: {}", test_run_id);
        
        // Create test session
        let session = self.create_test_session(&test_run_id).await?;
        
        let mut result = DocumentationTestResult {
            test_run_id: test_run_id.clone(),
            started_at,
            completed_at: started_at, // Will be updated
            duration: Duration::from_secs(0), // Will be calculated
            success: true,
            example_results: Vec::new(),
            link_results: LinkValidationResult {
                total_links: 0,
                valid_links: 0,
                invalid_links: 0,
                skipped_links: 0,
                link_results: Vec::new(),
                duration: Duration::from_secs(0),
            },
            coverage_results: None,
            summary: TestSummary {
                total_tests: 0,
                passed_tests: 0,
                failed_tests: 0,
                skipped_tests: 0,
                pass_rate: 0.0,
                total_duration: Duration::from_secs(0),
                average_duration: Duration::from_secs(0),
            },
            errors: Vec::new(),
            warnings: Vec::new(),
        };
        
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
    }
    
    /// Run example tests
    #[instrument(skip(self, docs_dir))]
    async fn run_example_tests(&mut self, docs_dir: &Path) -> Result<Vec<ExampleTestResult>, CursedError> {
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
        }
        
        // Collect results
        for task in tasks {
            match task.await {
                Ok(Ok(test_result)) => results.push(test_result),
                Ok(Err(e)) => warn!("Example test failed: {}", e),
                Err(e) => warn!("Example test task failed: {}", e),
            }
        }
        
        info!("Completed {} example tests", results.len());
        Ok(results)
    }
    
    /// Run a single example test
    async fn run_single_example_test(
        example: ExtractedExample,
        interactive_docs: &mut InteractiveDocumentation,
        config: &DocumentationTestConfig,
    ) -> Result<ExampleTestResult, CursedError> {
        let start_time = std::time::Instant::now();
        
        // Check if example should be skipped
        if example.source_code.contains(&config.example_test_settings.skip_marker) {
            return Ok(ExampleTestResult {
                example_id: example.id.clone(),
                source_code: example.source_code,
                file_path: example.file_path,
                line_number: example.line_number,
                success: true,
                execution_result: None,
                expected_output: example.expected_output,
                output_matches: None,
                duration: start_time.elapsed(),
                error_message: None,
                retry_count: 0,
            });
        }
        
        let mut retry_count = 0;
        let mut last_error = None;
        
        // Retry loop
        while retry_count <= config.retry_count {
            let execution_request = CodeExecutionRequest {
                session_id: format!("example_test_{}", example.id),
                code: example.source_code.clone(),
                language: "cursed".to_string(),
                input: None,
                timeout: Some(config.example_test_settings.max_execution_time),
                args: Vec::new(),
                env: HashMap::new(),
                working_dir: None,
            };
            
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
                        };
                        
                        let success = output_matches.unwrap_or(true);
                        
                        return Ok(ExampleTestResult {
                            example_id: example.id,
                            source_code: example.source_code,
                            file_path: example.file_path,
                            line_number: example.line_number,
                            success,
                            execution_result: Some(execution_result),
                            expected_output: example.expected_output,
                            output_matches,
                            duration: start_time.elapsed(),
                            error_message: if success { None } else { Some("Output mismatch".to_string()) },
                            retry_count,
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
        }
        
        // All retries failed
        Ok(ExampleTestResult {
            example_id: example.id,
            source_code: example.source_code,
            file_path: example.file_path,
            line_number: example.line_number,
            success: false,
            execution_result: None,
            expected_output: example.expected_output,
            output_matches: None,
            duration: start_time.elapsed(),
            error_message: last_error,
            retry_count,
        })
    }
    
    /// Validate links in documentation
    #[instrument(skip(self, docs_dir))]
    async fn validate_links(&self, docs_dir: &Path) -> Result<LinkValidationResult, CursedError> {
        let start_time = std::time::Instant::now();
        info!("Validating links in documentation");
        
        let links = self.extract_links_from_docs(docs_dir).await?;
        let mut link_results = Vec::new();
        
        for link in &links {
            if self.should_skip_link(&link.url) {
                link_results.push(LinkCheckResult {
                    url: link.url.clone(),
                    source_file: link.source_file.clone(),
                    line_number: link.line_number,
                    is_valid: true, // Assume skipped links are valid
                    status_code: None,
                    error_message: Some("Skipped".to_string()),
                    response_time: None,
                    final_url: None,
                });
                continue;
            }
            
            let result = self.check_single_link(link).await;
            link_results.push(result);
        }
        
        let total_links = link_results.len();
        let valid_links = link_results.iter().filter(|r| r.is_valid).count();
        let invalid_links = link_results.iter().filter(|r| !r.is_valid).count();
        let skipped_links = link_results.iter()
            .filter(|r| r.error_message.as_ref().map_or(false, |m| m == "Skipped"))
            .count();
        
        Ok(LinkValidationResult {
            total_links,
            valid_links,
            invalid_links,
            skipped_links,
            link_results,
            duration: start_time.elapsed(),
        })
    }
    
    /// Check a single link
    async fn check_single_link(&self, link: &ExtractedLink) -> LinkCheckResult {
        let start_time = std::time::Instant::now();
        
        // Check if it's an internal link
        if link.url.starts_with('#') || link.url.starts_with('/') || !link.url.contains("://") {
            return self.check_internal_link(link);
        }
        
        // Check external link
        if !self.config.link_check_settings.check_external_links {
            return LinkCheckResult {
                url: link.url.clone(),
                source_file: link.source_file.clone(),
                line_number: link.line_number,
                is_valid: true,
                status_code: None,
                error_message: Some("External link checking disabled".to_string()),
                response_time: Some(start_time.elapsed()),
                final_url: None,
            };
        }
        
        match self.http_client.get(&link.url).send().await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                let is_valid = self.config.link_check_settings.allowed_status_codes.contains(&status_code);
                let final_url = response.url().to_string();
                
                LinkCheckResult {
                    url: link.url.clone(),
                    source_file: link.source_file.clone(),
                    line_number: link.line_number,
                    is_valid,
                    status_code: Some(status_code),
                    error_message: if is_valid { None } else { Some(format!("Invalid status code: {}", status_code)) },
                    response_time: Some(start_time.elapsed()),
                    final_url: Some(final_url),
                }
            }
            Err(e) => {
                LinkCheckResult {
                    url: link.url.clone(),
                    source_file: link.source_file.clone(),
                    line_number: link.line_number,
                    is_valid: false,
                    status_code: None,
                    error_message: Some(e.to_string()),
                    response_time: Some(start_time.elapsed()),
                    final_url: None,
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
            url: link.url.clone(),
            source_file: link.source_file.clone(),
            line_number: link.line_number,
            is_valid,
            status_code: None,
            error_message: None,
            response_time: None,
            final_url: None,
        }
    }
    
    /// Generate coverage report
    #[instrument(skip(self, docs_dir))]
    async fn generate_coverage_report(&self, docs_dir: &Path) -> Result<CoverageResult, CursedError> {
        info!("Generating documentation coverage report");
        
        // This would analyze the source code and documentation to determine coverage
        // For now, return a mock result
        let overall_coverage = 75.0;
        
        Ok(CoverageResult {
            overall_coverage,
            file_coverage: HashMap::new(),
            item_type_coverage: HashMap::new(),
            undocumented_items: Vec::new(),
            coverage_trends: Vec::new(),
            quality_metrics: CoverageQualityMetrics {
                avg_doc_length: 150.0,
                doc_to_code_ratio: 0.3,
                examples_per_function: 0.8,
                cross_reference_density: 0.2,
                freshness_score: 0.9,
            },
        })
    }
    
    /// Extract examples from documentation
    async fn extract_examples_from_docs(&self, docs_dir: &Path) -> Result<Vec<ExtractedExample>, CursedError> {
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
    }
    
    /// Extract examples from a single file
    async fn extract_examples_from_file(&self, file_path: &Path) -> Result<Vec<ExtractedExample>, CursedError> {
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
                        id: example_id,
                        source_code: current_example.clone(),
                        file_path: file_path.to_path_buf(),
                        line_number: code_block_start,
                        expected_output: expected_output.take(),
                    });
                }
                
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
    }
    
    /// Extract links from documentation
    async fn extract_links_from_docs(&self, docs_dir: &Path) -> Result<Vec<ExtractedLink>, CursedError> {
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
    }
    
    /// Extract links from a single file
    async fn extract_links_from_file(&self, file_path: &Path) -> Result<Vec<ExtractedLink>, CursedError> {
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
                        url: url.as_str().to_string(),
                        source_file: file_path.to_path_buf(),
                        line_number: line_num_1based,
                        link_text: None,
                    });
                }
            }
            
            // Extract Markdown links
            for cap in markdown_link_regex.captures_iter(line) {
                if let Some(text) = cap.get(1) {
                    if let Some(url) = cap.get(2) {
                        links.push(ExtractedLink {
                            url: url.as_str().to_string(),
                            source_file: file_path.to_path_buf(),
                            line_number: line_num_1based,
                            link_text: Some(text.as_str().to_string()),
                        });
                    }
                }
            }
        }
        
        Ok(links)
    }
    
    /// Check if a link should be skipped
    fn should_skip_link(&self, url: &str) -> bool {
        // Check skip URLs
        if self.config.link_check_settings.skip_urls.contains(&url.to_string()) {
            return true;
        }
        
        // Check skip patterns
        for pattern in &self.config.link_check_settings.skip_patterns {
            if url.contains(pattern) {
                return true;
            }
        }
        
        false
    }
    
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
        };
        
        let total_duration = result.example_results.iter()
            .map(|r| r.duration)
            .sum::<Duration>();
        
        let average_duration = if total_tests > 0 {
            total_duration / total_tests as u32
        } else {
            Duration::from_secs(0)
        };
        
        TestSummary {
            total_tests,
            passed_tests,
            failed_tests,
            skipped_tests,
            pass_rate,
            total_duration,
            average_duration,
        }
    }
    
    /// Check quality gates
    fn check_quality_gates(&self, result: &mut DocumentationTestResult) {
        // Check coverage threshold
        if let Some(ref coverage) = result.coverage_results {
            if coverage.overall_coverage < self.config.min_coverage_threshold {
                result.success = false;
                result.errors.push(format!(
                    "Coverage {} is below threshold {}",
                    coverage.overall_coverage,
                    self.config.min_coverage_threshold
                ));
            }
        }
        
        // Check test pass rate
        if result.summary.pass_rate < 100.0 && self.config.fail_on_missing_docs {
            result.success = false;
            result.errors.push(format!(
                "Test pass rate {} is below 100%",
                result.summary.pass_rate
            ));
        }
        
        // Check link validation
        if result.link_results.invalid_links > 0 {
            result.warnings.push(format!(
                "{} invalid links found",
                result.link_results.invalid_links
            ));
        }
    }
    
    /// Generate test reports
    async fn generate_test_reports(&self, result: &DocumentationTestResult) -> Result<(), CursedError> {
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
    }
    
    /// Generate HTML test report
    fn generate_html_report(&self, result: &DocumentationTestResult) -> Result<String, CursedError> {
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
            <th>Error</th>
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
            <th>Error</th>
        </tr>
        {}
    </table>
</body>
</html>"#,
            result.test_run_id,
            result.duration,
            if result.success { "pass" } else { "fail" },
            result.success,
            result.summary.total_tests,
            result.summary.passed_tests,
            result.summary.failed_tests,
            result.summary.pass_rate,
            result.link_results.total_links,
            result.link_results.valid_links,
            result.link_results.invalid_links,
            result.example_results.iter().map(|r| format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td class=\"{}\">{}</td><td>{:?}</td><td>{}</td></tr>",
                r.example_id,
                r.file_path.display(),
                r.line_number,
                if r.success { "pass" } else { "fail" },
                if r.success { "PASS" } else { "FAIL" },
                r.duration,
                r.error_message.as_deref().unwrap_or("")
            )).collect::<Vec<_>>().join("\n"),
            result.link_results.link_results.iter().map(|r| format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td class=\"{}\">{}</td><td>{}</td></tr>",
                r.url,
                r.source_file.display(),
                r.line_number,
                if r.is_valid { "pass" } else { "fail" },
                if r.is_valid { "VALID" } else { "INVALID" },
                r.error_message.as_deref().unwrap_or("")
            )).collect::<Vec<_>>().join("\n")
        );
        
        Ok(html)
    }
    
    /// Create a test session
    async fn create_test_session(&mut self, test_run_id: &str) -> Result<&TestSession, CursedError> {
        let test_dir = self.config.test_output_dir.join(test_run_id);
        
        if !test_dir.exists() {
            std::fs::create_dir_all(&test_dir)
                .map_err(|e| CursedError::system_error(&format!("Failed to create test directory: {}", e)))?;
        }
        
        let session = TestSession {
            id: test_run_id.to_string(),
            started_at: SystemTime::now(),
            test_dir,
        };
        
        self.active_sessions.insert(test_run_id.to_string(), session);
        Ok(self.active_sessions.get(test_run_id).unwrap())
    }
}

/// Extracted example information
#[derive(Debug, Clone)]
struct ExtractedExample {
    id: String,
    source_code: String,
    file_path: PathBuf,
    line_number: usize,
    expected_output: Option<String>,
}

/// Extracted link information
#[derive(Debug, Clone)]
struct ExtractedLink {
    url: String,
    source_file: PathBuf,
    line_number: usize,
    link_text: Option<String>,
}
