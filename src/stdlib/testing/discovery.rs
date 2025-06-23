/// Test discovery system for CURSED testing framework
/// 
/// Handles finding and cataloging test functions across CURSED source files
/// with support for various test patterns and filtering options.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::crate::stdlib::errors_simple::CursedError;
use super::{TestError, TestFrameworkResult};

/// Information about a discovered test
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestInfo {
    /// Test function name
    pub name: String,
    
    /// File path containing the test
    pub file_path: PathBuf,
    
    /// Line number where test is defined
    pub line_number: u32,
    
    /// Test metadata and attributes
    pub metadata: TestMetadata,
    
    /// Module or namespace the test belongs to
    pub module: String,
    
    /// Test discovery timestamp
    pub discovered_at: SystemTime,
}

impl TestInfo {
    pub fn new(name: String, file_path: PathBuf, line_number: u32) -> Self {
        Self {
            name,
            file_path,
            line_number,
            metadata: TestMetadata::default(),
            module: String::new(),
            discovered_at: SystemTime::now(),
        }
    }
    
    /// Get test identifier (module::name)
    pub fn identifier(&self) -> String {
        if self.module.is_empty() {
            self.name.clone()
        } else {
            format!("{}::{}", self.module, self.name)
        }
    }
    
    /// Check if test should be ignored
    pub fn should_ignore(&self) -> bool {
        self.metadata.ignore
    }
    
    /// Check if test is expected to panic
    pub fn should_panic(&self) -> bool {
        self.metadata.should_panic
    }
    
    /// Get test timeout
    pub fn timeout(&self) -> Option<std::time::Duration> {
        self.metadata.timeout
    }
}

/// Test metadata extracted from attributes and comments
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TestMetadata {
    /// Whether the test should be ignored
    pub ignore: bool,
    
    /// Whether the test is expected to panic
    pub should_panic: bool,
    
    /// Optional test timeout
    pub timeout: Option<std::time::Duration>,
    
    /// Test description or documentation
    pub description: Option<String>,
    
    /// Test tags or categories
    pub tags: Vec<String>,
    
    /// Setup function name (if any)
    pub setup: Option<String>,
    
    /// Teardown function name (if any)
    pub teardown: Option<String>,
    
    /// Custom attributes
    pub attributes: HashMap<String, String>,
}

/// Test filtering configuration
#[derive(Debug, Clone, Default)]
pub struct TestFilter {
    /// Include only tests matching these patterns
    pub include_patterns: Vec<String>,
    
    /// Exclude tests matching these patterns
    pub exclude_patterns: Vec<String>,
    
    /// Include only tests with these tags
    pub include_tags: Vec<String>,
    
    /// Exclude tests with these tags
    pub exclude_tags: Vec<String>,
    
    /// Include only tests in these modules
    pub include_modules: Vec<String>,
    
    /// Exclude tests in these modules
    pub exclude_modules: Vec<String>,
    
    /// Whether to include ignored tests
    pub include_ignored: bool,
}

impl TestFilter {
    /// Create a new empty filter (includes all tests)
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add include pattern
    pub fn include_pattern(mut self, pattern: String) -> Self {
        self.include_patterns.push(pattern);
        self
    }
    
    /// Add exclude pattern
    pub fn exclude_pattern(mut self, pattern: String) -> Self {
        self.exclude_patterns.push(pattern);
        self
    }
    
    /// Add include tag
    pub fn include_tag(mut self, tag: String) -> Self {
        self.include_tags.push(tag);
        self
    }
    
    /// Add exclude tag
    pub fn exclude_tag(mut self, tag: String) -> Self {
        self.exclude_tags.push(tag);
        self
    }
    
    /// Check if a test matches this filter
    pub fn matches(&self, test: &TestInfo) -> bool {
        // Check ignored tests
        if test.should_ignore() && !self.include_ignored {
            return false;
        }
        
        // Check include patterns
        if !self.include_patterns.is_empty() {
            let matches = self.include_patterns.iter()
                .any(|pattern| self.pattern_matches(pattern, &test.name));
            if !matches {
                return false;
            }
        }
        
        // Check exclude patterns
        if self.exclude_patterns.iter()
            .any(|pattern| self.pattern_matches(pattern, &test.name)) {
            return false;
        }
        
        // Check include tags
        if !self.include_tags.is_empty() {
            let has_included_tag = self.include_tags.iter()
                .any(|tag| test.metadata.tags.contains(tag));
            if !has_included_tag {
                return false;
            }
        }
        
        // Check exclude tags
        if self.exclude_tags.iter()
            .any(|tag| test.metadata.tags.contains(tag)) {
            return false;
        }
        
        // Check include modules
        if !self.include_modules.is_empty() {
            let matches = self.include_modules.iter()
                .any(|module| test.module.starts_with(module));
            if !matches {
                return false;
            }
        }
        
        // Check exclude modules
        if self.exclude_modules.iter()
            .any(|module| test.module.starts_with(module)) {
            return false;
        }
        
        true
    }
    
    /// Simple pattern matching (supports * wildcards)
    fn pattern_matches(&self, pattern: &str, text: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        
        if pattern.contains('*') {
            // Simple wildcard matching
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.is_empty() {
                return true;
            }
            
            let mut text_pos = 0;
            for (i, part) in parts.iter().enumerate() {
                if part.is_empty() {
                    continue;
                }
                
                if i == 0 {
                    // First part must match from the beginning
                    if !text[text_pos..].starts_with(part) {
                        return false;
                    }
                    text_pos += part.len();
                } else if i == parts.len() - 1 {
                    // Last part must match at the end
                    return text[text_pos..].ends_with(part);
                } else {
                    // Middle parts can match anywhere
                    if let Some(pos) = text[text_pos..].find(part) {
                        text_pos += pos + part.len();
                    } else {
                        return false;
                    }
                }
            }
            true
        } else {
            text.contains(pattern)
        }
    }
}

/// Configuration for test discovery
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Root directory to search for tests
    pub test_root: PathBuf,
    
    /// File patterns to match
    pub patterns: Vec<String>,
    
    /// Test filter to apply
    pub filter: TestFilter,
    
    /// Whether to search recursively
    pub recursive: bool,
    
    /// Whether to include ignored tests in discovery
    pub include_ignored: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            test_root: PathBuf::from("."),
            patterns: vec![
                "**/*test*.csd".to_string(),
                "**/test_*.csd".to_string(),
                "tests/**/*.csd".to_string(),
            ],
            filter: TestFilter::default(),
            recursive: true,
            include_ignored: false,
        }
    }
}

/// Test discovery implementation
pub struct TestDiscovery {
    config: DiscoveryConfig,
    filters: Vec<TestFilter>,
}

impl TestDiscovery {
    /// Create a new test discovery with default configuration
    pub fn new() -> Self {
        Self::with_config(DiscoveryConfig::default())
    }
    
    /// Create a new test discovery with custom configuration
    pub fn with_config(config: DiscoveryConfig) -> Self {
        Self {
            config,
            filters: Vec::new(),
        }
    }
    
    /// Add a custom filter
    pub fn add_filter(&mut self, filter: TestFilter) {
        self.filters.push(filter);
    }
    
    /// Discover all tests according to configuration
    pub fn discover_tests(&self) -> TestFrameworkResult<Vec<TestInfo>> {
        let mut tests = Vec::new();
        
        if !self.config.test_root.exists() {
            return Err(TestError::DiscoveryError(
                format!("Test root directory '{}' does not exist", self.config.test_root.display())
            ).into());
        }
        
        for pattern in &self.config.patterns {
            let pattern_tests = self.discover_tests_with_pattern(pattern)?;
            tests.extend(pattern_tests);
        }
        
        // Remove duplicates based on test identifier
        tests.sort_by(|a: &TestInfo, b: &TestInfo| a.identifier().cmp(&b.identifier()));
        tests.dedup_by(|a, b| a.identifier() == b.identifier());
        
        // Apply filters
        tests = self.apply_filters(tests);
        
        Ok(tests)
    }
    
    /// Discover tests matching a specific pattern
    fn discover_tests_with_pattern(&self, pattern: &str) -> TestFrameworkResult<Vec<TestInfo>> {
        let mut tests = Vec::new();
        
        if self.config.recursive {
            self.discover_tests_recursive(&self.config.test_root, pattern, &mut tests)?;
        } else {
            self.discover_tests_in_directory(&self.config.test_root, pattern, &mut tests)?;
        }
        
        Ok(tests)
    }
    
    /// Recursively discover tests in directories
    fn discover_tests_recursive(&self, dir: &Path, pattern: &str, tests: &mut Vec<TestInfo>) -> TestFrameworkResult<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        let entries = fs::read_dir(dir)
            .map_err(|e| TestError::DiscoveryError(format!("Failed to read directory '{}': {}", dir.display(), e)))?;
        
        for entry in entries {
            let entry = entry
                .map_err(|e| TestError::DiscoveryError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_dir() {
                self.discover_tests_recursive(&path, pattern, tests)?;
            } else if self.file_matches_pattern(&path, pattern) {
                let file_tests = self.parse_test_file(&path)?;
                tests.extend(file_tests);
            }
        }
        
        Ok(())
    }
    
    /// Discover tests in a single directory
    fn discover_tests_in_directory(&self, dir: &Path, pattern: &str, tests: &mut Vec<TestInfo>) -> TestFrameworkResult<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        let entries = fs::read_dir(dir)
            .map_err(|e| TestError::DiscoveryError(format!("Failed to read directory '{}': {}", dir.display(), e)))?;
        
        for entry in entries {
            let entry = entry
                .map_err(|e| TestError::DiscoveryError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_file() && self.file_matches_pattern(&path, pattern) {
                let file_tests = self.parse_test_file(&path)?;
                tests.extend(file_tests);
            }
        }
        
        Ok(())
    }
    
    /// Check if a file matches a given pattern
    fn file_matches_pattern(&self, path: &Path, pattern: &str) -> bool {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // Simple pattern matching for now
        if pattern.contains("**") {
            // Recursive pattern - match anywhere in path
            let pattern_part = pattern.replace("**/", "").replace("**", "");
            self.simple_pattern_match(&pattern_part, file_name)
        } else {
            self.simple_pattern_match(pattern, file_name)
        }
    }
    
    /// Simple pattern matching with * wildcards
    fn simple_pattern_match(&self, pattern: &str, text: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.is_empty() {
                return true;
            }
            
            let mut text_pos = 0;
            for (i, part) in parts.iter().enumerate() {
                if part.is_empty() {
                    continue;
                }
                
                if i == 0 {
                    if !text[text_pos..].starts_with(part) {
                        return false;
                    }
                    text_pos += part.len();
                } else if i == parts.len() - 1 {
                    return text[text_pos..].ends_with(part);
                } else {
                    if let Some(pos) = text[text_pos..].find(part) {
                        text_pos += pos + part.len();
                    } else {
                        return false;
                    }
                }
            }
            true
        } else {
            text == pattern
        }
    }
    
    /// Parse a CURSED test file to extract test functions
    fn parse_test_file(&self, file_path: &Path) -> TestFrameworkResult<Vec<TestInfo>> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| TestError::DiscoveryError(format!("Failed to read file '{}': {}", file_path.display(), e)))?;
        
        let mut tests = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        // Extract module name from file path
        let module = self.extract_module_name(file_path);
        
        for (line_number, line) in lines.iter().enumerate() {
            let line_num = (line_number + 1) as u32;
            
            // Look for test function declarations
            if let Some(test_info) = self.parse_test_function(line, file_path, line_num, &module, &lines, line_number) {
                tests.push(test_info);
            }
        }
        
        Ok(tests)
    }
    
    /// Extract module name from file path
    fn extract_module_name(&self, file_path: &Path) -> String {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }
    
    /// Parse a potential test function from a line
    fn parse_test_function(&self, line: &str, file_path: &Path, line_number: u32, module: &str, all_lines: &[&str], current_line: usize) -> Option<TestInfo> {
        let trimmed = line.trim();
        
        // Look for CURSED test function patterns
        // Examples:
        // slay test_something() {
        // facts test_result = test_value();
        // #[test] slay test_name() {
        
        if self.is_test_function(trimmed) {
            if let Some(test_name) = self.extract_test_name(trimmed) {
                let mut test_info = TestInfo::new(
                    test_name,
                    file_path.to_path_buf(),
                    line_number
                );
                test_info.module = module.to_string();
                
                // Parse metadata from preceding lines and current line
                test_info.metadata = self.parse_test_metadata(all_lines, current_line);
                
                return Some(test_info);
            }
        }
        
        None
    }
    
    /// Check if a line contains a test function declaration
    fn is_test_function(&self, line: &str) -> bool {
        // CURSED test function patterns
        line.contains("slay test_") ||
        line.contains("facts test_") ||
        line.contains("#[test]") ||
        line.starts_with("test_") ||
        (line.contains("fn test_") && line.contains("()"))
    }
    
    /// Extract test name from a function declaration line
    fn extract_test_name(&self, line: &str) -> Option<String> {
        // Remove common prefixes and suffixes to extract the test name
        let line = line.trim();
        
        // Handle different CURSED test patterns
        if let Some(pos) = line.find("test_") {
            let start = pos;
            let remaining = &line[start..];
            
            // Find the end of the function name
            if let Some(end_pos) = remaining.find('(') {
                let name = &remaining[..end_pos];
                return Some(name.trim().to_string());
            }
        }
        
        None
    }
    
    /// Parse test metadata from surrounding lines
    fn parse_test_metadata(&self, lines: &[&str], current_line: usize) -> TestMetadata {
        let mut metadata = TestMetadata::default();
        
        // Look at the current line and preceding lines for attributes
        let start = current_line.saturating_sub(5); // Look up to 5 lines back
        let end = (current_line + 1).min(lines.len());
        
        for i in start..end {
            let line = lines[i].trim();
            
            // Parse test attributes
            if line.contains("#[test]") {
                // Basic test attribute
            } else if line.contains("#[ignore]") {
                metadata.ignore = true;
            } else if line.contains("#[should_panic]") {
                metadata.should_panic = true;
            } else if line.contains("#[timeout") {
                // Parse timeout value
                if let Some(timeout) = self.parse_timeout_attribute(line) {
                    metadata.timeout = Some(timeout);
                }
            } else if line.starts_with("// ") || line.starts_with("/// ") {
                // Parse documentation comments
                let comment = line.trim_start_matches("//").trim_start_matches("/").trim();
                if metadata.description.is_none() {
                    metadata.description = Some(comment.to_string());
                } else if let Some(ref mut desc) = metadata.description {
                    desc.push('\n');
                    desc.push_str(comment);
                }
            } else if line.contains("#[tag(") {
                // Parse tags
                if let Some(tag) = self.parse_tag_attribute(line) {
                    metadata.tags.push(tag);
                }
            }
        }
        
        metadata
    }
    
    /// Parse timeout value from timeout attribute
    fn parse_timeout_attribute(&self, line: &str) -> Option<std::time::Duration> {
        // Look for patterns like #[timeout(5000)] or #[timeout = "5s"]
        if let Some(start) = line.find('(') {
            if let Some(end) = line.find(')') {
                let value_str = &line[start + 1..end].trim();
                
                // Try to parse as milliseconds
                if let Ok(millis) = value_str.parse::<u64>() {
                    return Some(std::time::Duration::from_millis(millis));
                }
                
                // Try to parse with unit suffix
                if value_str.ends_with("ms") {
                    if let Ok(millis) = value_str.trim_end_matches("ms").parse::<u64>() {
                        return Some(std::time::Duration::from_millis(millis));
                    }
                } else if value_str.ends_with('s') {
                    if let Ok(secs) = value_str.trim_end_matches('s').parse::<u64>() {
                        return Some(std::time::Duration::from_secs(secs));
                    }
                }
            }
        }
        
        None
    }
    
    /// Parse tag value from tag attribute
    fn parse_tag_attribute(&self, line: &str) -> Option<String> {
        // Look for patterns like #[tag("integration")] or #[tag(slow)]
        if let Some(start) = line.find('(') {
            if let Some(end) = line.find(')') {
                let tag_str = &line[start + 1..end].trim();
                let tag = tag_str.trim_matches('"').trim_matches('\'');
                return Some(tag.to_string());
            }
        }
        
        None
    }
    
    /// Apply all configured filters to the test list
    fn apply_filters(&self, tests: Vec<TestInfo>) -> Vec<TestInfo> {
        let mut filtered_tests = tests;
        
        // Apply main configuration filter
        filtered_tests.retain(|test| self.config.filter.matches(test));
        
        // Apply additional custom filters
        for filter in &self.filters {
            filtered_tests.retain(|test| filter.matches(test));
        }
        
        filtered_tests
    }
}

impl Default for TestDiscovery {
    fn default() -> Self {
        Self::new()
    }
}
