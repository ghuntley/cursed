// Documentation Testing Infrastructure
// 
// Automated validation and testing for CURSED documentation including
// link checking, example verification, and completeness analysis.

use crate::error::{CursedError, Result};
use crate::docs::registry::{DocumentationRegistry, PackageDocumentation};
use crate::package_manager::Package;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::process::Command;
use tracing::{debug, error, info, instrument, warn};
use url::Url;

/// Documentation testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingConfig {
    /// Enable link checking
    pub check_links: bool,
    /// Enable example verification
    pub verify_examples: bool,
    /// Enable completeness analysis
    pub check_completeness: bool,
    /// Enable accessibility testing
    pub check_accessibility: bool,
    /// Timeout for network requests (seconds)
    pub request_timeout: u64,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Retry attempts for failed requests
    pub retry_attempts: usize,
    /// Example execution timeout (seconds)
    pub example_timeout: u64,
    /// Minimum documentation coverage required
    pub min_coverage_percentage: f64,
}

/// Test result summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    /// Package being tested
    pub package: String,
    /// Version being tested
    pub version: String,
    /// Test execution timestamp
    pub timestamp: u64,
    /// Overall test result
    pub passed: bool,
    /// Individual test results
    pub test_categories: HashMap<String, CategoryResult>,
    /// Performance metrics
    pub performance: TestPerformanceMetrics,
    /// Detailed issues found
    pub issues: Vec<TestIssue>,
    /// Suggestions for improvement
    pub suggestions: Vec<TestSuggestion>,
}

/// Category-specific test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResult {
    /// Category name
    pub category: String,
    /// Tests passed
    pub passed: bool,
    /// Tests performed
    pub tests_run: usize,
    /// Tests passed count
    pub tests_passed: usize,
    /// Tests failed count
    pub tests_failed: usize,
    /// Category-specific metrics
    pub metrics: HashMap<String, f64>,
    /// Category issues
    pub issues: Vec<String>,
}

/// Performance metrics for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPerformanceMetrics {
    /// Total test time (milliseconds)
    pub total_time_ms: u64,
    /// Link checking time (milliseconds)
    pub link_check_time_ms: u64,
    /// Example verification time (milliseconds)
    pub example_verify_time_ms: u64,
    /// Completeness analysis time (milliseconds)
    pub completeness_time_ms: u64,
    /// Number of links checked
    pub links_checked: usize,
    /// Number of examples tested
    pub examples_tested: usize,
}

/// Test issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestIssue {
    /// Issue type
    pub issue_type: IssueType,
    /// Severity level
    pub severity: IssueSeverity,
    /// Issue description
    pub description: String,
    /// Location information
    pub location: Option<IssueLocation>,
    /// Fix suggestion
    pub fix_suggestion: Option<String>,
}

/// Test suggestion for improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuggestion {
    /// Suggestion type
    pub suggestion_type: String,
    /// Priority level
    pub priority: SuggestionPriority,
    /// Suggestion description
    pub description: String,
    /// Implementation guidance
    pub guidance: String,
}

/// Issue type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    BrokenLink,
    MissingDocumentation,
    ExampleFailure,
    AccessibilityViolation,
    PerformanceIssue,
    FormattingError,
    InconsistentStyle,
    SecurityConcern,
}

/// Issue severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Suggestion priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionPriority {
    High,
    Medium,
    Low,
}

/// Issue location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueLocation {
    /// File path
    pub file: String,
    /// Line number
    pub line: Option<usize>,
    /// Column number
    pub column: Option<usize>,
    /// Context information
    pub context: Option<String>,
}

/// Link checking result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCheckResult {
    /// URL checked
    pub url: String,
    /// Response status
    pub status: LinkStatus,
    /// Response time (milliseconds)
    pub response_time_ms: u64,
    /// CursedError message (if any)
    pub error: Option<String>,
    /// Location where link was found
    pub source_location: IssueLocation,
}

/// Link status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkStatus {
    Ok,
    Broken,
    Timeout,
    Redirect,
    Warning,
}

/// Example verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleResult {
    /// Example name
    pub name: String,
    /// Execution result
    pub status: ExampleStatus,
    /// Execution time (milliseconds)
    pub execution_time_ms: u64,
    /// Expected output
    pub expected_output: Option<String>,
    /// Actual output
    pub actual_output: Option<String>,
    /// CursedError message (if any)
    pub error: Option<String>,
    /// Compilation success
    pub compilation_success: bool,
}

/// Example execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExampleStatus {
    Passed,
    Failed,
    CompilationError,
    RuntimeError,
    Timeout,
    Skipped,
}

/// Completeness analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletenessResult {
    /// Total public items
    pub total_items: usize,
    /// Documented items
    pub documented_items: usize,
    /// Items with examples
    pub items_with_examples: usize,
    /// Coverage percentage
    pub coverage_percentage: f64,
    /// Missing documentation
    pub missing_docs: Vec<MissingDocumentation>,
    /// Documentation quality scores
    pub quality_scores: HashMap<String, f64>,
}

/// Missing documentation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDocumentation {
    /// Item name
    pub item_name: String,
    /// Item type
    pub item_type: String,
    /// Module path
    pub module: String,
    /// Missing elements
    pub missing_elements: Vec<String>,
}

/// Accessibility check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityResult {
    /// Total checks performed
    pub total_checks: usize,
    /// Checks passed
    pub checks_passed: usize,
    /// Accessibility score
    pub score: f64,
    /// Violations found
    pub violations: Vec<AccessibilityViolation>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Accessibility violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityViolation {
    /// Rule violated
    pub rule: String,
    /// Severity level
    pub severity: String,
    /// Description
    pub description: String,
    /// Element selector
    pub selector: Option<String>,
    /// Fix guidance
    pub fix_guidance: String,
}

/// Documentation tester
pub struct DocumentationTester {
    config: TestingConfig,
    registry: DocumentationRegistry,
    http_client: reqwest::Client,
}

impl DocumentationTester {
    /// Create a new documentation tester
    pub fn new(config: TestingConfig, registry: DocumentationRegistry) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.request_timeout))
            .build()
            .map_err(|e| CursedError::Network(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            registry,
            http_client,
        })
    }

    /// Test documentation for a package
    #[instrument(skip(self))]
    pub async fn test_package(&self, package: &Package, version: &str) -> Result<TestResults> {
        info!(package = %package.name, version = %version, "Testing package documentation");

        let start_time = SystemTime::now();
        let mut test_results = TestResults {
            package: package.name.clone(),
            version: version.to_string(),
            timestamp: start_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            passed: true,
            test_categories: HashMap::new(),
            performance: TestPerformanceMetrics {
                total_time_ms: 0,
                link_check_time_ms: 0,
                example_verify_time_ms: 0,
                completeness_time_ms: 0,
                links_checked: 0,
                examples_tested: 0,
            },
            issues: Vec::new(),
            suggestions: Vec::new(),
        };

        // Get package documentation
        let package_doc = self.registry.get_package(&package.name).await
            .ok_or_else(|| CursedError::NotFound(format!("Package {} not found in registry", package.name)))?;

        let version_doc = package_doc.versions.get(version)
            .ok_or_else(|| CursedError::NotFound(format!("Version {} not found for package {}", version, package.name)))?;

        // Run link checking
        if self.config.check_links {
            let link_start = SystemTime::now();
            let link_result = self.check_links(&package_doc, version).await?;
            test_results.performance.link_check_time_ms = link_start.elapsed().unwrap().as_millis() as u64;
            test_results.performance.links_checked = *link_result.metrics.get("links_checked").unwrap_or(&0.0) as usize;
            
            if !link_result.passed {
                test_results.passed = false;
            }
            test_results.test_categories.insert("link_checking".to_string(), link_result);
        }

        // Run example verification
        if self.config.verify_examples {
            let example_start = SystemTime::now();
            let example_result = self.verify_examples(&package_doc, version).await?;
            test_results.performance.example_verify_time_ms = example_start.elapsed().unwrap().as_millis() as u64;
            test_results.performance.examples_tested = *example_result.metrics.get("examples_tested").unwrap_or(&0.0) as usize;
            
            if !example_result.passed {
                test_results.passed = false;
            }
            test_results.test_categories.insert("example_verification".to_string(), example_result);
        }

        // Run completeness analysis
        if self.config.check_completeness {
            let completeness_start = SystemTime::now();
            let completeness_result = self.analyze_completeness(&package_doc, version).await?;
            test_results.performance.completeness_time_ms = completeness_start.elapsed().unwrap().as_millis() as u64;
            
            if !completeness_result.passed {
                test_results.passed = false;
            }
            test_results.test_categories.insert("completeness_analysis".to_string(), completeness_result);
        }

        // Run accessibility checks
        if self.config.check_accessibility {
            let accessibility_result = self.check_accessibility(&package_doc, version).await?;
            if !accessibility_result.passed {
                test_results.passed = false;
            }
            test_results.test_categories.insert("accessibility".to_string(), accessibility_result);
        }

        // Collect issues and suggestions
        self.collect_issues_and_suggestions(&mut test_results).await?;

        // Calculate total time
        test_results.performance.total_time_ms = start_time.elapsed().unwrap().as_millis() as u64;

        info!(
            package = %package.name,
            version = %version,
            passed = test_results.passed,
            total_time = test_results.performance.total_time_ms,
            "Documentation testing completed"
        );

        Ok(test_results)
    }

    /// Check links in documentation
    #[instrument(skip(self))]
    async fn check_links(&self, package_doc: &PackageDocumentation, version: &str) -> Result<CategoryResult> {
        debug!(package = %package_doc.name, version = %version, "Checking documentation links");

        let mut result = CategoryResult {
            category: "link_checking".to_string(),
            passed: true,
            tests_run: 0,
            tests_passed: 0,
            tests_failed: 0,
            metrics: HashMap::new(),
            issues: Vec::new(),
        };

        let version_doc = package_doc.versions.get(version).unwrap();
        let mut links_to_check = Vec::new();

        // Extract links from documentation
        for api_item in &version_doc.api_items {
            links_to_check.extend(self.extract_links_from_text(&api_item.documentation));
        }

        // Check each link
        let mut broken_links = 0;
        let mut checked_links = 0;

        for link in links_to_check {
            let link_result = self.check_single_link(&link).await;
            checked_links += 1;

            match link_result.status {
                LinkStatus::Ok | LinkStatus::Redirect => {
                    result.tests_passed += 1;
                }
                LinkStatus::Broken | LinkStatus::Timeout => {
                    result.tests_failed += 1;
                    broken_links += 1;
                    result.issues.push(format!("Broken link: {}", link));
                    result.passed = false;
                }
                LinkStatus::Warning => {
                    result.issues.push(format!("Link warning: {}", link));
                }
            }
        }

        result.tests_run = checked_links;
        result.metrics.insert("links_checked".to_string(), checked_links as f64);
        result.metrics.insert("broken_links".to_string(), broken_links as f64);
        result.metrics.insert("success_rate".to_string(), 
            if checked_links > 0 { result.tests_passed as f64 / checked_links as f64 * 100.0 } else { 100.0 });

        Ok(result)
    }

    /// Verify examples in documentation
    #[instrument(skip(self))]
    async fn verify_examples(&self, package_doc: &PackageDocumentation, version: &str) -> Result<CategoryResult> {
        debug!(package = %package_doc.name, version = %version, "Verifying documentation examples");

        let mut result = CategoryResult {
            category: "example_verification".to_string(),
            passed: true,
            tests_run: 0,
            tests_passed: 0,
            tests_failed: 0,
            metrics: HashMap::new(),
            issues: Vec::new(),
        };

        let version_doc = package_doc.versions.get(version).unwrap();
        let mut failed_examples = 0;

        // Test examples from API items
        for api_item in &version_doc.api_items {
            for example in &api_item.examples {
                let example_result = self.test_example(example, &api_item.name).await;
                result.tests_run += 1;

                match example_result.status {
                    ExampleStatus::Passed => {
                        result.tests_passed += 1;
                    }
                    ExampleStatus::Failed | ExampleStatus::CompilationError | ExampleStatus::RuntimeError => {
                        result.tests_failed += 1;
                        failed_examples += 1;
                        result.issues.push(format!("Example failed in {}: {}", api_item.name, 
                            example_result.error.unwrap_or_else(|| "Unknown error".to_string())));
                        result.passed = false;
                    }
                    ExampleStatus::Timeout => {
                        result.tests_failed += 1;
                        result.issues.push(format!("Example timeout in {}", api_item.name));
                        result.passed = false;
                    }
                    ExampleStatus::Skipped => {
                        // Don't count skipped examples as failures
                    }
                }
            }
        }

        // Test standalone examples
        for example in &version_doc.examples {
            let example_result = self.test_standalone_example(example).await;
            result.tests_run += 1;

            match example_result.status {
                ExampleStatus::Passed => {
                    result.tests_passed += 1;
                }
                _ => {
                    result.tests_failed += 1;
                    failed_examples += 1;
                    result.issues.push(format!("Standalone example '{}' failed", example.name));
                    result.passed = false;
                }
            }
        }

        result.metrics.insert("examples_tested".to_string(), result.tests_run as f64);
        result.metrics.insert("failed_examples".to_string(), failed_examples as f64);
        result.metrics.insert("success_rate".to_string(), 
            if result.tests_run > 0 { result.tests_passed as f64 / result.tests_run as f64 * 100.0 } else { 100.0 });

        Ok(result)
    }

    /// Analyze documentation completeness
    #[instrument(skip(self))]
    async fn analyze_completeness(&self, package_doc: &PackageDocumentation, version: &str) -> Result<CategoryResult> {
        debug!(package = %package_doc.name, version = %version, "Analyzing documentation completeness");

        let mut result = CategoryResult {
            category: "completeness_analysis".to_string(),
            passed: true,
            tests_run: 1,
            tests_passed: 0,
            tests_failed: 0,
            metrics: HashMap::new(),
            issues: Vec::new(),
        };

        let version_doc = package_doc.versions.get(version).unwrap();
        
        // Calculate coverage metrics
        let total_items = version_doc.api_items.len();
        let documented_items = version_doc.api_items.iter()
            .filter(|item| !item.documentation.is_empty())
            .count();
        let items_with_examples = version_doc.api_items.iter()
            .filter(|item| !item.examples.is_empty())
            .count();

        let coverage_percentage = if total_items > 0 {
            documented_items as f64 / total_items as f64 * 100.0
        } else {
            100.0
        };

        let example_coverage = if total_items > 0 {
            items_with_examples as f64 / total_items as f64 * 100.0
        } else {
            100.0
        };

        // Check if coverage meets minimum requirements
        if coverage_percentage < self.config.min_coverage_percentage {
            result.passed = false;
            result.tests_failed = 1;
            result.issues.push(format!(
                "Documentation coverage ({:.1}%) below minimum requirement ({:.1}%)",
                coverage_percentage, self.config.min_coverage_percentage
            ));
        } else {
            result.tests_passed = 1;
        }

        // Identify missing documentation
        for api_item in &version_doc.api_items {
            if api_item.documentation.is_empty() {
                result.issues.push(format!("Missing documentation for {}", api_item.name));
            }
            if api_item.examples.is_empty() {
                result.issues.push(format!("Missing examples for {}", api_item.name));
            }
        }

        result.metrics.insert("total_items".to_string(), total_items as f64);
        result.metrics.insert("documented_items".to_string(), documented_items as f64);
        result.metrics.insert("coverage_percentage".to_string(), coverage_percentage);
        result.metrics.insert("example_coverage".to_string(), example_coverage);

        Ok(result)
    }

    /// Check accessibility of documentation
    #[instrument(skip(self))]
    async fn check_accessibility(&self, package_doc: &PackageDocumentation, _version: &str) -> Result<CategoryResult> {
        debug!(package = %package_doc.name, "Checking documentation accessibility");

        let mut result = CategoryResult {
            category: "accessibility".to_string(),
            passed: true,
            tests_run: 5, // Example: 5 accessibility checks
            tests_passed: 4, // Example: 4 passed
            tests_failed: 1, // Example: 1 failed
            metrics: HashMap::new(),
            issues: Vec::new(),
        };

        // Simulate accessibility checks
        result.issues.push("Missing alt text for images".to_string());
        result.passed = false;

        result.metrics.insert("accessibility_score".to_string(), 80.0);
        result.metrics.insert("violations_found".to_string(), 1.0);

        Ok(result)
    }

    /// Extract links from text content
    fn extract_links_from_text(&self, text: &str) -> Vec<String> {
        let mut links = Vec::new();
        
        // Simple regex-based link extraction (would be more sophisticated in production)
        for line in text.split("\n") {
            if line.contains("http://") || line.contains("https://") {
                // Extract URLs using a simple heuristic
                let words: Vec<&str> = line.split_whitespace().collect();
                for word in words {
                    if word.starts_with("http://") || word.starts_with("https://") {
                        links.push(word.trim_end_matches(&['.', ',', ')', ']', '}'][..]).to_string());
                    }
                }
            }
        }
        
        links
    }

    /// Check a single link
    async fn check_single_link(&self, url: &str) -> LinkCheckResult {
        let start_time = SystemTime::now();
        
        let parsed_url = match Url::parse(url) {
            Ok(url) => url,
            Err(_) => {
                return LinkCheckResult {
                    url: url.to_string(),
                    status: LinkStatus::Broken,
                    response_time_ms: 0,
                    error: Some("Invalid URL format".to_string()),
                    source_location: IssueLocation {
                        file: "unknown".to_string(),
                        line: None,
                        column: None,
                        context: None,
                    },
                };
            }
        };

        let result = self.http_client.head(parsed_url).send().await;
        let response_time_ms = start_time.elapsed().unwrap().as_millis() as u64;

        match result {
            Ok(response) => {
                let status = if response.status().is_success() {
                    LinkStatus::Ok
                } else if response.status().is_redirection() {
                    LinkStatus::Redirect
                } else {
                    LinkStatus::Broken
                };

                LinkCheckResult {
                    url: url.to_string(),
                    status,
                    response_time_ms,
                    error: None,
                    source_location: IssueLocation {
                        file: "unknown".to_string(),
                        line: None,
                        column: None,
                        context: None,
                    },
                }
            }
            Err(e) => {
                let status = if e.is_timeout() {
                    LinkStatus::Timeout
                } else {
                    LinkStatus::Broken
                };

                LinkCheckResult {
                    url: url.to_string(),
                    status,
                    response_time_ms,
                    error: Some(e.to_string()),
                    source_location: IssueLocation {
                        file: "unknown".to_string(),
                        line: None,
                        column: None,
                        context: None,
                    },
                }
            }
        }
    }

    /// Test an example code snippet
    async fn test_example(&self, example_code: &str, context: &str) -> ExampleResult {
        let start_time = SystemTime::now();
        
        // Create temporary file for example
        let temp_file = format!("/tmp/cursed_example_{}.csd", 
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
        
        if let Err(e) = tokio::fs::write(&temp_file, example_code).await {
            return ExampleResult {
                name: context.to_string(),
                status: ExampleStatus::Failed,
                execution_time_ms: 0,
                expected_output: None,
                actual_output: None,
                error: Some(format!("Failed to write example file: {}", e)),
                compilation_success: false,
            };
        }

        // Try to compile the example
        let compile_result = Command::new("cursed")
            .args(&["compile", "--check", &temp_file])
            .output()
            .await;

        let execution_time_ms = start_time.elapsed().unwrap().as_millis() as u64;

        // Clean up temp file
        let _ = tokio::fs::remove_file(&temp_file).await;

        match compile_result {
            Ok(output) => {
                if output.status.success() {
                    ExampleResult {
                        name: context.to_string(),
                        status: ExampleStatus::Passed,
                        execution_time_ms,
                        expected_output: None,
                        actual_output: Some(String::from_utf8_lossy(&output.stdout).to_string()),
                        error: None,
                        compilation_success: true,
                    }
                } else {
                    ExampleResult {
                        name: context.to_string(),
                        status: ExampleStatus::CompilationError,
                        execution_time_ms,
                        expected_output: None,
                        actual_output: None,
                        error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                        compilation_success: false,
                    }
                }
            }
            Err(e) => ExampleResult {
                name: context.to_string(),
                status: ExampleStatus::Failed,
                execution_time_ms,
                expected_output: None,
                actual_output: None,
                error: Some(e.to_string()),
                compilation_success: false,
            },
        }
    }

    /// Test a standalone example
    async fn test_standalone_example(&self, example: &crate::docs::registry::ExampleDocumentation) -> ExampleResult {
        self.test_example(&example.source_code, &example.name).await
    }

    /// Collect issues and suggestions from test results
    async fn collect_issues_and_suggestions(&self, test_results: &mut TestResults) -> Result<()> {
        // Collect issues from all categories
        for (category, result) in &test_results.test_categories {
            for issue_text in &result.issues {
                let issue = TestIssue {
                    issue_type: self.categorize_issue(issue_text),
                    severity: self.determine_severity(issue_text),
                    description: issue_text.clone(),
                    location: None,
                    fix_suggestion: self.generate_fix_suggestion(issue_text),
                };
                test_results.issues.push(issue);
            }
        }

        // Generate suggestions based on results
        test_results.suggestions = self.generate_suggestions(test_results).await;

        Ok(())
    }

    /// Categorize an issue based on its text
    fn categorize_issue(&self, issue_text: &str) -> IssueType {
        if issue_text.contains("Broken link") {
            IssueType::BrokenLink
        } else if issue_text.contains("Missing documentation") {
            IssueType::MissingDocumentation
        } else if issue_text.contains("Example failed") {
            IssueType::ExampleFailure
        } else if issue_text.contains("accessibility") {
            IssueType::AccessibilityViolation
        } else {
            IssueType::FormattingError
        }
    }

    /// Determine issue severity
    fn determine_severity(&self, issue_text: &str) -> IssueSeverity {
        if issue_text.contains("Broken link") || issue_text.contains("Example failed") {
            IssueSeverity::High
        } else if issue_text.contains("Missing") {
            IssueSeverity::Medium
        } else {
            IssueSeverity::Low
        }
    }

    /// Generate fix suggestion for an issue
    fn generate_fix_suggestion(&self, issue_text: &str) -> Option<String> {
        if issue_text.contains("Broken link") {
            Some("Check and update the URL, or remove the broken link".to_string())
        } else if issue_text.contains("Missing documentation") {
            Some("Add comprehensive documentation for this item".to_string())
        } else if issue_text.contains("Example failed") {
            Some("Fix the example code or update dependencies".to_string())
        } else {
            None
        }
    }

    /// Generate improvement suggestions
    async fn generate_suggestions(&self, test_results: &TestResults) -> Vec<TestSuggestion> {
        let mut suggestions = Vec::new();

        // Analyze test results and generate suggestions
        if let Some(completeness) = test_results.test_categories.get("completeness_analysis") {
            if let Some(coverage) = completeness.metrics.get("coverage_percentage") {
                if *coverage < 80.0 {
                    suggestions.push(TestSuggestion {
                        suggestion_type: "coverage_improvement".to_string(),
                        priority: SuggestionPriority::High,
                        description: "Improve documentation coverage".to_string(),
                        guidance: "Add documentation for undocumented public items".to_string(),
                    });
                }
            }
        }

        if let Some(examples) = test_results.test_categories.get("example_verification") {
            if examples.tests_failed > 0 {
                suggestions.push(TestSuggestion {
                    suggestion_type: "example_improvement".to_string(),
                    priority: SuggestionPriority::High,
                    description: "Fix failing examples".to_string(),
                    guidance: "Review and update example code to ensure it compiles and runs correctly".to_string(),
                });
            }
        }

        suggestions
    }

    /// Validate testing configuration
    pub fn validate_config(&self) -> Result<()> {
        if self.config.request_timeout == 0 {
            return Err(CursedError::Configuration(
                "Request timeout must be greater than 0".to_string()
            ));
        }

        if self.config.max_concurrent_requests == 0 {
            return Err(CursedError::Configuration(
                "Max concurrent requests must be greater than 0".to_string()
            ));
        }

        if self.config.min_coverage_percentage < 0.0 || self.config.min_coverage_percentage > 100.0 {
            return Err(CursedError::Configuration(
                "Minimum coverage percentage must be between 0 and 100".to_string()
            ));
        }

        Ok(())
    }
}

impl Default for TestingConfig {
    fn default() -> Self {
        Self {
            check_links: true,
            verify_examples: true,
            check_completeness: true,
            check_accessibility: true,
            request_timeout: 30,
            max_concurrent_requests: 10,
            retry_attempts: 3,
            example_timeout: 60,
            min_coverage_percentage: 70.0,
        }
    }
}

