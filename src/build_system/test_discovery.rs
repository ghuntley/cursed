use crate::error::CursedError;
// Test Discovery System
// 
// Discovers and analyzes test files in the project to identify test functions,
// test categories, and test metadata for execution.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn, instrument};

/// Represents a discovered test function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFunction {
    /// Name of the test function
    
    /// Test file path
    
    /// Line number where test is defined
    
    /// Test category (unit, integration, ignored, etc.)
    
    /// Whether test should be ignored by default
    
    /// Whether this is a benchmark test
    
    /// Test timeout in seconds (if specified)
    
    /// Test attributes and annotations
    
    /// Module path within the test file
/// Test categories for organization and filtering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestCategory {
    /// Unit tests (in lib.rs or mod tests)
    
    /// Integration tests (in tests/ directory)
    
    /// Documentation tests
    
    /// Benchmark tests
    
    /// Example tests
    
    /// Custom category
/// Test discovery configuration
#[derive(Debug, Clone)]
pub struct TestDiscoveryConfig {
    /// Root directory to search for tests
    
    /// Include unit tests in src/
    
    /// Include integration tests in tests/
    
    /// Include documentation tests
    
    /// Include benchmark tests
    
    /// Include example tests
    
    /// Custom test patterns to include
    
    /// Files to exclude from discovery
impl Default for TestDiscoveryConfig {
    fn default() -> Self {
        Self {
            exclude_patterns: vec![
                "target/**".to_string(),
                ".git/**".to_string(),
        }
    }
/// Test discovery results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDiscoveryResult {
    /// All discovered test functions
    
    /// Tests organized by category
    
    /// Tests organized by file
    
    /// Discovery statistics
/// Statistics about test discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDiscoveryStatistics {
    /// Total number of tests discovered
    
    /// Number of unit tests
    
    /// Number of integration tests
    
    /// Number of ignored tests
    
    /// Number of benchmark tests
    
    /// Number of files scanned
    
    /// Number of test files found
/// Main test discovery engine
pub struct TestDiscovery {
impl TestDiscovery {
    /// Create a new test discovery instance
    pub fn new(config: TestDiscoveryConfig) -> crate::error::Result<()> {
        let test_fn_regex = Regex::new(r"#\[test\]\s*(?:#\[.*?\])?\s*(?:async\s+)?fn\s+(\w+)")?;
        let benchmark_regex = Regex::new(r"#\[bench\]")?;
        let ignore_regex = Regex::new(r"#\[ignore\]")?;
        let timeout_regex = Regex::new(r"#\[timeout\((\d+)\)\]")?;
        
        Ok(TestDiscovery {
        })
    /// Discover all tests in the project
    #[instrument(skip(self))]
    pub fn discover_tests(&self) -> crate::error::Result<()> {
        info!("Starting test discovery in: {}", self.config.root_dir.display());
        
        let mut all_tests = Vec::new();
        let mut files_scanned = 0;
        let mut test_files_found = 0;
        
        // Discover integration tests
        if self.config.include_integration_tests {
            let tests_dir = self.config.root_dir.join("tests");
            if tests_dir.exists() {
                let (tests, scanned, found) = self.discover_integration_tests(&tests_dir)?;
                all_tests.extend(tests);
                files_scanned += scanned;
                test_files_found += found;
            }
        }
        
        // Discover unit tests
        if self.config.include_unit_tests {
            let src_dir = self.config.root_dir.join("src");
            if src_dir.exists() {
                let (tests, scanned, found) = self.discover_unit_tests(&src_dir)?;
                all_tests.extend(tests);
                files_scanned += scanned;
                test_files_found += found;
            }
        }
        
        // Discover benchmark tests
        if self.config.include_benchmarks {
            let benches_dir = self.config.root_dir.join("benches");
            if benches_dir.exists() {
                let (tests, scanned, found) = self.discover_benchmark_tests(&benches_dir)?;
                all_tests.extend(tests);
                files_scanned += scanned;
                test_files_found += found;
            }
        }
        
        // Discover example tests
        if self.config.include_examples {
            let examples_dir = self.config.root_dir.join("examples");
            if examples_dir.exists() {
                let (tests, scanned, found) = self.discover_example_tests(&examples_dir)?;
                all_tests.extend(tests);
                files_scanned += scanned;
                test_files_found += found;
            }
        }
        
        // Organize tests by category and file
        let mut tests_by_category: HashMap<TestCategory, Vec<TestFunction>> = HashMap::new();
        let mut tests_by_file: HashMap<PathBuf, Vec<TestFunction>> = HashMap::new();
        
        for test in &all_tests {
            tests_by_category.entry(test.category.clone()).or_default().push(test.clone());
            tests_by_file.entry(test.file_path.clone()).or_default().push(test.clone());
        // Calculate statistics
        let statistics = TestDiscoveryStatistics {
        
              statistics.total_tests, statistics.test_files_found);
        
        Ok(TestDiscoveryResult {
        })
    /// Discover integration tests in tests/ directory
    fn discover_integration_tests(&self, tests_dir: &Path) -> crate::error::Result<()> {
        let mut tests = Vec::new();
        let mut files_scanned = 0;
        let mut test_files_found = 0;
        
        for entry in fs::read_dir(tests_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                files_scanned += 1;
                
                if let Ok(file_tests) = self.parse_test_file(&path, TestCategory::Integration) {
                    if !file_tests.is_empty() {
                        test_files_found += 1;
                        tests.extend(file_tests);
                    }
                }
            } else if path.is_dir() && !self.should_exclude_path(&path) {
                let (sub_tests, sub_scanned, sub_found) = self.discover_integration_tests(&path)?;
                tests.extend(sub_tests);
                files_scanned += sub_scanned;
                test_files_found += sub_found;
            }
        }
        
        Ok((tests, files_scanned, test_files_found))
    /// Discover unit tests in src/ directory
    fn discover_unit_tests(&self, src_dir: &Path) -> crate::error::Result<()> {
        let mut tests = Vec::new();
        let mut files_scanned = 0;
        let mut test_files_found = 0;
        
        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                files_scanned += 1;
                
                if let Ok(file_tests) = self.parse_test_file(&path, TestCategory::Unit) {
                    if !file_tests.is_empty() {
                        test_files_found += 1;
                        tests.extend(file_tests);
                    }
                }
            } else if path.is_dir() && !self.should_exclude_path(&path) {
                let (sub_tests, sub_scanned, sub_found) = self.discover_unit_tests(&path)?;
                tests.extend(sub_tests);
                files_scanned += sub_scanned;
                test_files_found += sub_found;
            }
        }
        
        Ok((tests, files_scanned, test_files_found))
    /// Discover benchmark tests in benches/ directory
    fn discover_benchmark_tests(&self, benches_dir: &Path) -> crate::error::Result<()> {
        let mut tests = Vec::new();
        let mut files_scanned = 0;
        let mut test_files_found = 0;
        
        for entry in fs::read_dir(benches_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                files_scanned += 1;
                
                if let Ok(file_tests) = self.parse_test_file(&path, TestCategory::Benchmark) {
                    if !file_tests.is_empty() {
                        test_files_found += 1;
                        tests.extend(file_tests);
                    }
                }
            }
        }
        
        Ok((tests, files_scanned, test_files_found))
    /// Discover example tests in examples/ directory
    fn discover_example_tests(&self, examples_dir: &Path) -> crate::error::Result<()> {
        let mut tests = Vec::new();
        let mut files_scanned = 0;
        let mut test_files_found = 0;
        
        for entry in fs::read_dir(examples_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                files_scanned += 1;
                
                if let Ok(file_tests) = self.parse_test_file(&path, TestCategory::Example) {
                    if !file_tests.is_empty() {
                        test_files_found += 1;
                        tests.extend(file_tests);
                    }
                }
            }
        }
        
        Ok((tests, files_scanned, test_files_found))
    /// Parse a single test file to extract test functions
    fn parse_test_file(&self, file_path: &Path, default_category: TestCategory) -> crate::error::Result<()> {
        let content = fs::read_to_string(file_path)?;
        let mut tests = Vec::new();
        
        debug!("Parsing test file: {}", file_path.display());
        
        let lines: Vec<&str> = content.split("\n").collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Look for test attributes
            if line.starts_with("#[test]") || line.starts_with("#[bench]") {
                if let Some(test_fn) = self.parse_test_function(&lines, i, file_path, default_category.clone())? {
                    tests.push(test_fn);
                }
            }
            
            i += 1;
        debug!("Found {} tests in {}", tests.len(), file_path.display());
        Ok(tests)
    /// Parse a single test function from source lines
    fn parse_test_function(
    ) -> crate::error::Result<()> {
        let mut attributes = Vec::new();
        let mut ignored = false;
        let mut is_benchmark = false;
        let mut timeout = None;
        let mut i = start_index;
        
        // Parse attributes
        while i < lines.len() && lines[i].trim().starts_with('#') {
            let attr_line = lines[i].trim();
            attributes.push(attr_line.to_string());
            
            if attr_line.contains("#[ignore]") {
                ignored = true;
            if attr_line.contains("#[bench]") {
                is_benchmark = true;
            if let Some(captures) = self.timeout_regex.captures(attr_line) {
                if let Some(timeout_str) = captures.get(1) {
                    timeout = timeout_str.as_str().parse().ok();
                }
            }
            
            i += 1;
        // Parse function declaration
        if i < lines.len() {
            let fn_line = lines[i].trim();
            
            if let Some(captures) = self.test_fn_regex.captures(fn_line) {
                if let Some(fn_name) = captures.get(1) {
                    let test_function = TestFunction {
                    
                    return Ok(Some(test_function));
                }
            }
        Ok(None)
    /// Extract module path from file path
    fn extract_module_path(&self, file_path: &Path) -> String {
        // Convert file path to module path
        let relative_path = if let Ok(rel) = file_path.strip_prefix(&self.config.root_dir) {
            rel
        } else {
            file_path
        
        let path_str = relative_path.to_string_lossy();
        let module_path = path_str
            .replace('/', "::")
            .replace('\\', "::")
            .replace(".rs", "");
        
        // Remove common prefixes
        if module_path.starts_with("src::") {
            module_path[5..].to_string()
        } else if module_path.starts_with("tests::") {
            module_path[7..].to_string()
        } else {
            module_path
        }
    }
    
    /// Check if a path should be excluded from discovery
    fn should_exclude_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        for pattern in &self.config.exclude_patterns {
            if path_str.contains(pattern.trim_end_matches("**")) {
                return true;
            }
        }
        
        false
    /// Filter tests based on patterns
    pub fn filter_tests(&self, discovery_result: &TestDiscoveryResult, patterns: &[String]) -> Vec<TestFunction> {
        if patterns.is_empty() {
            return discovery_result.tests.clone();
        let mut filtered_tests = Vec::new();
        
        for pattern in patterns {
            for test in &discovery_result.tests {
                if self.matches_pattern(&test.name, pattern) || 
                   self.matches_pattern(&test.module_path, pattern) {
                    if !filtered_tests.iter().any(|t: &TestFunction| t.name == test.name && t.file_path == test.file_path) {
                        filtered_tests.push(test.clone());
                    }
                }
            }
        }
        
        filtered_tests
    /// Check if a test name matches a pattern
    fn matches_pattern(&self, name: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            // Simple glob-style matching
            let regex_pattern = pattern.replace('*', ".*");
            if let Ok(regex) = Regex::new(&regex_pattern) {
                return regex.is_match(name);
            }
        }
        
        name.contains(pattern)
    }
}

/// Test filter configuration
#[derive(Debug, Clone)]
pub struct TestFilter {
    /// Test name patterns to include
    
    /// Test name patterns to exclude
    
    /// Include ignored tests
    
    /// Include only ignored tests
    
    /// Include benchmark tests
    
    /// Test categories to include
impl Default for TestFilter {
    fn default() -> Self {
        Self {
        }
    }
impl TestFilter {
    /// Apply filter to test discovery results
    pub fn apply(&self, discovery_result: &TestDiscoveryResult) -> Vec<TestFunction> {
        let mut filtered_tests = discovery_result.tests.clone();
        
        // Filter by categories
        if !self.categories.is_empty() {
            filtered_tests.retain(|test| self.categories.contains(&test.category));
        // Filter by ignored status
        if self.only_ignored {
            filtered_tests.retain(|test| test.ignored);
        } else if !self.include_ignored {
            filtered_tests.retain(|test| !test.ignored);
        // Filter by benchmark status
        if !self.include_benchmarks {
            filtered_tests.retain(|test| !test.is_benchmark);
        // Apply include patterns
        if !self.include_patterns.is_empty() {
            filtered_tests.retain(|test| {
                self.include_patterns.iter().any(|pattern| {
                    test.name.contains(pattern) || test.module_path.contains(pattern)
                })
            });
        // Apply exclude patterns
        if !self.exclude_patterns.is_empty() {
            filtered_tests.retain(|test| {
                !self.exclude_patterns.iter().any(|pattern| {
                    test.name.contains(pattern) || test.module_path.contains(pattern)
                })
            });
        filtered_tests
    }
}

