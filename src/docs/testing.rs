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
    /// Enable example verification
    /// Enable completeness analysis
    /// Enable accessibility testing
    /// Timeout for network requests (seconds)
    /// Maximum concurrent requests
    /// Retry attempts for failed requests
    /// Example execution timeout (seconds)
    /// Minimum documentation coverage required
/// Test result summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    /// Package being tested
    /// Version being tested
    /// Test execution timestamp
    /// Overall test result
    /// Individual test results
    /// Performance metrics
    /// Detailed issues found
    /// Suggestions for improvement
/// Category-specific test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResult {
    /// Category name
    /// Tests passed
    /// Tests performed
    /// Tests passed count
    /// Tests failed count
    /// Category-specific metrics
    /// Category issues
/// Performance metrics for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPerformanceMetrics {
    /// Total test time (milliseconds)
    /// Link checking time (milliseconds)
    /// Example verification time (milliseconds)
    /// Completeness analysis time (milliseconds)
    /// Number of links checked
    /// Number of examples tested
/// Test issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestIssue {
    /// Issue type
    /// Severity level
    /// Issue description
    /// Location information
    /// Fix suggestion
/// Test suggestion for improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuggestion {
    /// Suggestion type
    /// Priority level
    /// Suggestion description
    /// Implementation guidance
/// Issue type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
/// Issue severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
/// Suggestion priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionPriority {
/// Issue location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueLocation {
    /// File path
    /// Line number
    /// Column number
    /// Context information
/// Link checking result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCheckResult {
    /// URL checked
    /// Response status
    /// Response time (milliseconds)
    /// CursedError message (if any)
    /// Location where link was found
/// Link status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkStatus {
/// Example verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleResult {
    /// Example name
    /// Execution result
    /// Execution time (milliseconds)
    /// Expected output
    /// Actual output
    /// CursedError message (if any)
    /// Compilation success
/// Example execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExampleStatus {
/// Completeness analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletenessResult {
    /// Total public items
    /// Documented items
    /// Items with examples
    /// Coverage percentage
    /// Missing documentation
    /// Documentation quality scores
/// Missing documentation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDocumentation {
    /// Item name
    /// Item type
    /// Module path
    /// Missing elements
/// Accessibility check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityResult {
    /// Total checks performed
    /// Checks passed
    /// Accessibility score
    /// Violations found
    /// Recommendations
/// Accessibility violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityViolation {
    /// Rule violated
    /// Severity level
    /// Description
    /// Element selector
    /// Fix guidance
/// Documentation tester
pub struct DocumentationTester {
impl DocumentationTester {
    /// Create a new documentation tester
    pub fn new(config: TestingConfig, registry: DocumentationRegistry) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.request_timeout))
            .build()
            .map_err(|e| CursedError::Network(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
        })
    /// Test documentation for a package
    #[instrument(skip(self))]
    pub async fn test_package(&self, package: &Package, version: &str) -> Result<TestResults> {
        info!(package = %package.name, version = %version, "Testing package documentation");

        let start_time = SystemTime::now();
        let mut test_results = TestResults {
            performance: TestPerformanceMetrics {

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
        // Run completeness analysis
        if self.config.check_completeness {
            let completeness_start = SystemTime::now();
            let completeness_result = self.analyze_completeness(&package_doc, version).await?;
            test_results.performance.completeness_time_ms = completeness_start.elapsed().unwrap().as_millis() as u64;
            
            if !completeness_result.passed {
                test_results.passed = false;
            }
            test_results.test_categories.insert("completeness_analysis".to_string(), completeness_result);
        // Run accessibility checks
        if self.config.check_accessibility {
            let accessibility_result = self.check_accessibility(&package_doc, version).await?;
            if !accessibility_result.passed {
                test_results.passed = false;
            }
            test_results.test_categories.insert("accessibility".to_string(), accessibility_result);
        // Collect issues and suggestions
        self.collect_issues_and_suggestions(&mut test_results).await?;

        // Calculate total time
        test_results.performance.total_time_ms = start_time.elapsed().unwrap().as_millis() as u64;

        info!(
            "Documentation testing completed"
        );

        Ok(test_results)
    /// Check links in documentation
    #[instrument(skip(self))]
    async fn check_links(&self, package_doc: &PackageDocumentation, version: &str) -> Result<CategoryResult> {
        debug!(package = %package_doc.name, version = %version, "Checking documentation links");

        let mut result = CategoryResult {

        let version_doc = package_doc.versions.get(version).unwrap();
        let mut links_to_check = Vec::new();

        // Extract links from documentation
        for api_item in &version_doc.api_items {
            links_to_check.extend(self.extract_links_from_text(&api_item.documentation));
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
        result.tests_run = checked_links;
        result.metrics.insert("links_checked".to_string(), checked_links as f64);
        result.metrics.insert("broken_links".to_string(), broken_links as f64);
            if checked_links > 0 { result.tests_passed as f64 / checked_links as f64 * 100.0 } else { 100.0 });

        Ok(result)
    /// Verify examples in documentation
    #[instrument(skip(self))]
    async fn verify_examples(&self, package_doc: &PackageDocumentation, version: &str) -> Result<CategoryResult> {
        debug!(package = %package_doc.name, version = %version, "Verifying documentation examples");

        let mut result = CategoryResult {

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
        result.metrics.insert("examples_tested".to_string(), result.tests_run as f64);
        result.metrics.insert("failed_examples".to_string(), failed_examples as f64);
            if result.tests_run > 0 { result.tests_passed as f64 / result.tests_run as f64 * 100.0 } else { 100.0 });

        Ok(result)
    /// Analyze documentation completeness
    #[instrument(skip(self))]
    async fn analyze_completeness(&self, package_doc: &PackageDocumentation, version: &str) -> Result<CategoryResult> {
        debug!(package = %package_doc.name, version = %version, "Analyzing documentation completeness");

        let mut result = CategoryResult {

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

        let example_coverage = if total_items > 0 {
            items_with_examples as f64 / total_items as f64 * 100.0
        } else {
            100.0

        // Check if coverage meets minimum requirements
        if coverage_percentage < self.config.min_coverage_percentage {
            result.passed = false;
            result.tests_failed = 1;
            result.issues.push(format!(
                coverage_percentage, self.config.min_coverage_percentage
            ));
        } else {
            result.tests_passed = 1;
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
    /// Check accessibility of documentation
    #[instrument(skip(self))]
    async fn check_accessibility(&self, package_doc: &PackageDocumentation, _version: &str) -> Result<CategoryResult> {
        debug!(package = %package_doc.name, "Checking documentation accessibility");

        let mut result = CategoryResult {
            tests_run: 5, // Example: 5 accessibility checks
            tests_passed: 4, // Example: 4 passed
            tests_failed: 1, // Example: 1 failed

        // Simulate accessibility checks
        result.issues.push("Missing alt text for images".to_string());
        result.passed = false;

        result.metrics.insert("accessibility_score".to_string(), 80.0);
        result.metrics.insert("violations_found".to_string(), 1.0);

        Ok(result)
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
    /// Check a single link
    async fn check_single_link(&self, url: &str) -> LinkCheckResult {
        let start_time = SystemTime::now();
        
        let parsed_url = match Url::parse(url) {
            Err(_) => {
                return LinkCheckResult {
                    source_location: IssueLocation {
            }

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

                LinkCheckResult {
                    source_location: IssueLocation {
                }
            }
            Err(e) => {
                let status = if e.is_timeout() {
                    LinkStatus::Timeout
                } else {
                    LinkStatus::Broken

                LinkCheckResult {
                    source_location: IssueLocation {
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
                    }
                } else {
                    ExampleResult {
                    }
                }
            }
            Err(e) => ExampleResult {
        }
    }

    /// Test a standalone example
    async fn test_standalone_example(&self, example: &crate::docs::registry::ExampleDocumentation) -> ExampleResult {
        self.test_example(&example.source_code, &example.name).await
    /// Collect issues and suggestions from test results
    async fn collect_issues_and_suggestions(&self, test_results: &mut TestResults) -> Result<()> {
        // Collect issues from all categories
        for (category, result) in &test_results.test_categories {
            for issue_text in &result.issues {
                let issue = TestIssue {
                test_results.issues.push(issue);
            }
        }

        // Generate suggestions based on results
        test_results.suggestions = self.generate_suggestions(test_results).await;

        Ok(())
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
                    });
                }
            }
        if let Some(examples) = test_results.test_categories.get("example_verification") {
            if examples.tests_failed > 0 {
                suggestions.push(TestSuggestion {
                });
            }
        }

        suggestions
    /// Validate testing configuration
    pub fn validate_config(&self) -> Result<()> {
        if self.config.request_timeout == 0 {
            return Err(CursedError::Configuration(
                "Request timeout must be greater than 0".to_string()
            ));
        if self.config.max_concurrent_requests == 0 {
            return Err(CursedError::Configuration(
                "Max concurrent requests must be greater than 0".to_string()
            ));
        if self.config.min_coverage_percentage < 0.0 || self.config.min_coverage_percentage > 100.0 {
            return Err(CursedError::Configuration(
                "Minimum coverage percentage must be between 0 and 100".to_string()
            ));
        Ok(())
    }
}

impl Default for TestingConfig {
    fn default() -> Self {
        Self {
        }
    }
