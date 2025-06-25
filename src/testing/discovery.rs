/// Test Discovery System
/// 
/// Discovers CURSED test files and extracts test functions using the
/// lexer and parser to identify test functions with proper syntax.

use crate::error::CursedError;
use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use crate::ast::{Program, Statement, FunctionStatement};
use super::{TestError, TestResult as TestingResult};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use globwalk::{GlobWalkerBuilder, DirEntry};
use tracing::{info, debug, warn};

/// Represents a discovered test file
#[derive(Debug, Clone)]
pub struct TestFile {
    /// Path to the test file
    pub path: PathBuf,
    /// Test functions found in the file
    pub test_functions: Vec<TestFunction>,
    /// Package/module name if applicable
    pub package_name: Option<String>,
    /// File size in bytes
    pub size_bytes: u64,
    /// Last modified timestamp
    pub last_modified: std::time::SystemTime,
}

/// Represents a discovered test function
#[derive(Debug, Clone)]
pub struct TestFunction {
    /// Name of the test function
    pub name: String,
    /// Line number where function starts
    pub line_number: usize,
    /// Column number where function starts
    pub column_number: usize,
    /// Function source code
    pub source_code: String,
    /// Test type (unit, integration, benchmark, etc.)
    pub test_type: TestType,
    /// Expected to fail (marked with @should_fail or similar)
    pub should_fail: bool,
    /// Test timeout override
    pub timeout_override: Option<u64>,
    /// Test tags/categories
    pub tags: Vec<String>,
}

/// Test type classification
#[derive(Debug, Clone, PartialEq)]
pub enum TestType {
    /// Regular unit test
    Unit,
    /// Integration test
    Integration,
    /// Benchmark test
    Benchmark,
    /// Example/documentation test
    Example,
    /// Performance test
    Performance,
}

/// Represents a test suite (collection of test files)
#[derive(Debug, Clone)]
pub struct TestSuite {
    /// Name of the test suite
    pub name: String,
    /// Test files in the suite
    pub test_files: Vec<TestFile>,
    /// Total number of test functions
    pub total_tests: usize,
    /// Suite configuration
    pub config: TestSuiteConfig,
}

/// Configuration for a test suite
#[derive(Debug, Clone)]
pub struct TestSuiteConfig {
    /// Maximum parallel execution for this suite
    pub max_parallel: Option<usize>,
    /// Suite-specific timeout
    pub timeout: Option<u64>,
    /// Required setup/teardown
    pub requires_setup: bool,
    /// Environment requirements
    pub environment: HashMap<String, String>,
}

impl Default for TestSuiteConfig {
    fn default() -> Self {
        Self {
            max_parallel: None,
            timeout: None,
            requires_setup: false,
            environment: HashMap::new(),
        }
    }
}

/// Test discovery engine
pub struct TestDiscovery {
    /// File patterns to include
    include_patterns: Vec<String>,
    /// File patterns to exclude
    exclude_patterns: Vec<String>,
    /// Root directory for discovery
    root_directory: PathBuf,
    /// Discovered test suites
    discovered_suites: Vec<TestSuite>,
}

impl TestDiscovery {
    /// Create new test discovery instance
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        Self {
            include_patterns: vec!["**/*_test.csd".to_string(), "**/test_*.csd".to_string()],
            exclude_patterns: vec!["target/**".to_string(), ".git/**".to_string()],
            root_directory: root_dir.as_ref().to_path_buf(),
            discovered_suites: Vec::new(),
        }
    }

    /// Set include patterns for test file discovery
    pub fn with_include_patterns(mut self, patterns: Vec<String>) -> Self {
        self.include_patterns = patterns;
        self
    }

    /// Set exclude patterns for test file discovery
    pub fn with_exclude_patterns(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns = patterns;
        self
    }

    /// Filter test suites by pattern
    pub fn filter_tests(&self, pattern: &str) -> TestingResult<Vec<TestSuite>> {
        let filtered_suites = self.discovered_suites
            .iter()
            .filter(|suite| {
                suite.name.contains(pattern) || 
                suite.test_files.iter().any(|file| {
                    file.test_functions.iter().any(|func| func.name.contains(pattern))
                })
            })
            .cloned()
            .collect();
        Ok(filtered_suites)
    }

    /// Get discovered test suites
    pub fn get_discovered_suites(&self) -> &[TestSuite] {
        &self.discovered_suites
    }

    /// Discover all test files and functions
    pub async fn discover_tests(&mut self) -> TestingResult<Vec<TestSuite>> {
        info!("Starting test discovery in: {}", self.root_directory.display());
        
        // Create a simple default test suite for now
        let default_suite = TestSuite {
            name: "default".to_string(),
            test_files: Vec::new(),
            total_tests: 0,
            config: TestSuiteConfig::default(),
        };
        
        let suites = vec![default_suite];
        self.discovered_suites = suites.clone();
        
        info!("Discovery completed: {} test suites", suites.len());
        Ok(suites)
    }

    /// Find test files using glob patterns
    pub fn find_test_files(&self) -> TestingResult<Vec<PathBuf>> {
        let mut all_files = Vec::new();
        
        for pattern in &self.include_patterns {
            let walker = GlobWalkerBuilder::from_patterns(&self.root_directory, &[pattern])
                .follow_links(false)
                .build()
                .map_err(|e| TestError::Discovery(format!("Failed to create walker: {}", e)))?;
            
            for entry in walker {
                match entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.path().to_path_buf();
                        
                        // Check exclude patterns
                        let should_exclude = self.exclude_patterns.iter().any(|exclude_pattern| {
                            path.to_string_lossy().contains(exclude_pattern.trim_end_matches("**"))
                        });
                        
                        if !should_exclude && path.is_file() {
                            all_files.push(path);
                        }
                    }
                    Err(e) => {
                        warn!("CursedError walking directory: {}", e);
                    }
                }
            }
        }
        
        // Remove duplicates
        all_files.sort();
        all_files.dedup();
        
        Ok(all_files)
    }

    /// Process a single test file to extract test functions
    pub fn process_test_file(&self, file_path: &Path) -> TestingResult<TestFile> {
        debug!("Processing test file: {}", file_path.display());
        
        // Read file contents
        let source = std::fs::read_to_string(file_path)
            .map_err(|e| TestError::Io(format!("Failed to read {}: {}", file_path.display(), e)))?;
        
        // Get file metadata
        let metadata = std::fs::metadata(file_path)
            .map_err(|e| TestError::Io(format!("Failed to get metadata for {}: {}", file_path.display(), e)))?;
        
        // Parse the source code to find test functions
        let test_functions = self.extract_test_functions(&source)?;
        
        // Extract package name if present
        let package_name = self.extract_package_name(&source);
        
        Ok(TestFile {
            path: file_path.to_path_buf(),
            test_functions,
            package_name,
            size_bytes: metadata.len(),
            last_modified: metadata.modified().unwrap_or(std::time::UNIX_EPOCH),
        })
    }

    /// Extract test functions from source code using lexer and parser
    fn extract_test_functions(&self, source: &str) -> TestingResult<Vec<TestFunction>> {
        // Create lexer and parser
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer)
            .map_err(|e| TestError::Compilation(format!("Failed to create parser: {}", e)))?;
        
        // Parse the program
        let program = parser.parse_program()
            .map_err(|e| TestError::Compilation(format!("Failed to parse program: {}", e)))?;
        
        let mut test_functions = Vec::new();
        
        // Extract test functions from the AST
        for statement in &program.statements {
            // Try to downcast to FunctionStatement
            if let Some(func) = statement.as_any().downcast_ref::<FunctionStatement>() {
                if self.is_test_function(func) {
                    let test_func = self.create_test_function(func, source)?;
                    test_functions.push(test_func);
                }
            }
        }
        
        Ok(test_functions)
    }

    /// Check if a function is a test function
    fn is_test_function(&self, func: &FunctionStatement) -> bool {
        // Test functions start with "test_" or have specific attributes
        func.name.value.starts_with("test_") || 
        func.name.value.starts_with("Test") ||
        func.name.value.starts_with("bench_") ||
        self.has_test_attribute(func)
    }

    /// Check if function has test attributes/annotations
    fn has_test_attribute(&self, func: &FunctionStatement) -> bool {
        // For now, just check naming conventions
        // In the future, this could parse attributes or comments
        func.name.value.contains("test") || func.name.value.contains("Test")
    }

    /// Create TestFunction from AST FunctionStatement
    fn create_test_function(&self, func: &FunctionStatement, source: &str) -> TestingResult<TestFunction> {
        // Determine test type based on function name
        let test_type = if func.name.value.starts_with("bench_") {
            TestType::Benchmark
        } else if func.name.value.contains("integration") {
            TestType::Integration
        } else if func.name.value.contains("example") {
            TestType::Example
        } else if func.name.value.contains("perf") {
            TestType::Performance
        } else {
            TestType::Unit
        };

        // Extract function source code (simplified)
        let source_lines: Vec<&str> = source.split("\n").collect();
        // Since we don't have line numbers from AST, estimate based on function name search
        let start_line = source_lines.iter()
            .position(|line| line.contains(&func.name.value))
            .unwrap_or(0);
        let end_line = (start_line + 20).min(source_lines.len()); // Approximate function length
        let function_source = source_lines[start_line..end_line].join("\n");

        Ok(TestFunction {
            name: func.name.value.clone(),
            line_number: start_line + 1, // 1-indexed
            column_number: 1, // Default to column 1
            source_code: function_source,
            test_type,
            should_fail: func.name.value.contains("fail") || func.name.value.contains("error"),
            timeout_override: None,
            tags: self.extract_test_tags(&func.name.value),
        })
    }

    /// Extract test tags from function name
    fn extract_test_tags(&self, func_name: &str) -> Vec<String> {
        let mut tags = Vec::new();
        
        if func_name.contains("slow") {
            tags.push("slow".to_string());
        }
        if func_name.contains("integration") {
            tags.push("integration".to_string());
        }
        if func_name.contains("unit") {
            tags.push("unit".to_string());
        }
        if func_name.contains("benchmark") {
            tags.push("benchmark".to_string());
        }
        
        tags
    }

    /// Extract package name from source code
    fn extract_package_name(&self, source: &str) -> Option<String> {
        // Look for package declarations
        for line in source.split("\n") {
            let trimmed = line.trim();
            if trimmed.starts_with("package ") {
                return trimmed.strip_prefix("package ")
                    .map(|s| s.trim_end_matches(';').trim().to_string());
            }
        }
        None
    }

    /// Group test files into logical test suites
    pub fn group_into_suites(&self, test_files: Vec<TestFile>) -> TestingResult<Vec<TestSuite>> {
        let mut suites_map: HashMap<String, Vec<TestFile>> = HashMap::new();
        
        for test_file in test_files {
            // Group by directory or package name
            let suite_name = if let Some(package) = &test_file.package_name {
                package.clone()
            } else {
                // Use parent directory as suite name
                test_file.path
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("default")
                    .to_string()
            };
            
            suites_map.entry(suite_name).or_default().push(test_file);
        }
        
        let mut suites = Vec::new();
        for (name, files) in suites_map {
            let total_tests = files.iter().map(|f| f.test_functions.len()).sum();
            
            suites.push(TestSuite {
                name,
                test_files: files,
                total_tests,
                config: TestSuiteConfig::default(),
            });
        }
        
        // Sort suites by name for consistent output
        suites.sort_by(|a, b| a.name.cmp(&b.name));
        
        Ok(suites)
    }

    /// Get discovered test suites
    pub fn get_discovered_suites(&self) -> &[TestSuite] {
        &self.discovered_suites
    }

    /// Filter tests by pattern
    pub fn filter_tests(&self, pattern: &str) -> TestingResult<Vec<TestSuite>> {
        let mut filtered_suites = Vec::new();
        
        for suite in &self.discovered_suites {
            let mut filtered_files = Vec::new();
            
            for test_file in &suite.test_files {
                let mut filtered_functions = Vec::new();
                
                for test_func in &test_file.test_functions {
                    if test_func.name.contains(pattern) {
                        filtered_functions.push(test_func.clone());
                    }
                }
                
                if !filtered_functions.is_empty() {
                    let mut filtered_file = test_file.clone();
                    filtered_file.test_functions = filtered_functions;
                    filtered_files.push(filtered_file);
                }
            }
            
            if !filtered_files.is_empty() {
                let total_tests = filtered_files.iter().map(|f| f.test_functions.len()).sum();
                
                filtered_suites.push(TestSuite {
                    name: suite.name.clone(),
                    test_files: filtered_files,
                    total_tests,
                    config: suite.config.clone(),
                });
            }
        }
        
        Ok(filtered_suites)
    }
}

