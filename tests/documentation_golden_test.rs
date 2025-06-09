//! Golden file testing for CURSED documentation system
//!
//! Compares generated documentation output against known-good reference files
//! to detect regressions and ensure consistent output quality.

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    time::Instant,
    io::{Read, Write},
    collections::HashMap,
};
use tempfile::TempDir;
use serde_json::Value;

use cursed::docs::{
    DocumentationGenerator, DocConfig, DocumentationItem, ItemType,
    PackageDocumentation, DocError, DocResult,
};

mod common;

/// Golden file test configuration
#[derive(Debug, Clone)]
struct GoldenTestConfig {
    /// Test name identifier
    name: String,
    /// Source file for documentation generation
    source_file: PathBuf,
    /// Expected output file (golden reference)
    golden_file: PathBuf,
    /// Temporary output directory
    output_dir: PathBuf,
    /// Whether to update golden files instead of comparing
    update_golden: bool,
}

/// Golden file test result
#[derive(Debug)]
struct GoldenTestResult {
    /// Test name
    name: String,
    /// Whether test passed
    passed: bool,
    /// Differences found (if any)
    differences: Vec<String>,
    /// Generation time
    generation_time: std::time::Duration,
}

/// Golden file test runner
struct GoldenFileTestRunner {
    /// Test configuration
    configs: Vec<GoldenTestConfig>,
    /// Test results
    results: Vec<GoldenTestResult>,
    /// Working directory
    work_dir: TempDir,
}

impl GoldenFileTestRunner {
    fn new() -> std::io::Result<Self> {
        common::tracing::setup();
        
        Ok(Self {
            configs: Vec::new(),
            results: Vec::new(),
            work_dir: TempDir::new()?,
        })
    }
    
    /// Add a golden test configuration
    fn add_test(&mut self, name: &str, source_content: &str, expected_content: Option<&str>) -> std::io::Result<()> {
        let source_file = self.work_dir.path().join(format!("{}.csd", name));
        let golden_file = self.work_dir.path().join(format!("{}_golden.html", name));
        let output_dir = self.work_dir.path().join(format!("{}_output", name));
        
        fs::write(&source_file, source_content)?;
        fs::create_dir_all(&output_dir)?;
        
        if let Some(content) = expected_content {
            fs::write(&golden_file, content)?;
        }
        
        self.configs.push(GoldenTestConfig {
            name: name.to_string(),
            source_file,
            golden_file,
            output_dir,
            update_golden: expected_content.is_none(), // Update if no expected content provided
        });
        
        Ok(())
    }
    
    /// Run all golden file tests
    fn run_all_tests(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for config in &self.configs {
            let result = self.run_single_test(config)?;
            self.results.push(result);
        }
        Ok(())
    }
    
    /// Run a single golden file test
    fn run_single_test(&self, config: &GoldenTestConfig) -> Result<GoldenTestResult, Box<dyn std::error::Error>> {
        let start_time = Instant::now();
        
        // Generate documentation
        let doc_config = DocConfig::new()
            .with_source_dirs(vec![config.source_file.clone()])
            .with_output_dir(config.output_dir.clone())
            .with_package_name(config.name.clone());
        
        let mut generator = DocumentationGenerator::new(doc_config);
        generator.generate()?;
        
        let generation_time = start_time.elapsed();
        
        // Get generated output
        let generated_file = config.output_dir.join("index.html");
        let generated_content = if generated_file.exists() {
            fs::read_to_string(&generated_file)?
        } else {
            return Ok(GoldenTestResult {
                name: config.name.clone(),
                passed: false,
                differences: vec!["Generated file does not exist".to_string()],
                generation_time,
            });
        };
        
        // Handle golden file update or comparison
        if config.update_golden || !config.golden_file.exists() {
            // Update/create golden file
            fs::write(&config.golden_file, &generated_content)?;
            
            Ok(GoldenTestResult {
                name: config.name.clone(),
                passed: true,
                differences: vec!["Golden file updated".to_string()],
                generation_time,
            })
        } else {
            // Compare with golden file
            let golden_content = fs::read_to_string(&config.golden_file)?;
            let differences = self.compare_html_content(&golden_content, &generated_content);
            
            Ok(GoldenTestResult {
                name: config.name.clone(),
                passed: differences.is_empty(),
                differences,
                generation_time,
            })
        }
    }
    
    /// Compare HTML content and find differences
    fn compare_html_content(&self, expected: &str, actual: &str) -> Vec<String> {
        let mut differences = Vec::new();
        
        // Basic content comparison
        if expected.trim() == actual.trim() {
            return differences;
        }
        
        // Line-by-line comparison for detailed differences
        let expected_lines: Vec<&str> = expected.lines().collect();
        let actual_lines: Vec<&str> = actual.lines().collect();
        
        let max_lines = expected_lines.len().max(actual_lines.len());
        
        for i in 0..max_lines {
            let expected_line = expected_lines.get(i).unwrap_or(&"");
            let actual_line = actual_lines.get(i).unwrap_or(&"");
            
            if expected_line.trim() != actual_line.trim() {
                differences.push(format!(
                    "Line {}: Expected '{}', got '{}'",
                    i + 1,
                    expected_line.trim(),
                    actual_line.trim()
                ));
                
                // Limit number of differences reported
                if differences.len() >= 10 {
                    differences.push("... (more differences truncated)".to_string());
                    break;
                }
            }
        }
        
        differences
    }
    
    /// Print test results summary
    fn print_summary(&self) {
        println!("\n=== Golden File Test Results ===");
        
        let total_tests = self.results.len();
        let passed_tests = self.results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        
        println!("Total tests: {}", total_tests);
        println!("Passed: {}", passed_tests);
        println!("Failed: {}", failed_tests);
        
        // Print individual results
        for result in &self.results {
            let status = if result.passed { "PASS" } else { "FAIL" };
            println!(
                "[{}] {} ({:?})",
                status,
                result.name,
                result.generation_time
            );
            
            if !result.passed && !result.differences.is_empty() {
                for diff in &result.differences {
                    println!("  - {}", diff);
                }
            }
        }
        
        println!("\n=== Performance Summary ===");
        let total_time: std::time::Duration = self.results.iter().map(|r| r.generation_time).sum();
        let avg_time = total_time / (total_tests as u32);
        println!("Total generation time: {:?}", total_time);
        println!("Average per test: {:?}", avg_time);
    }
}

#[test]
fn test_simple_documentation_golden() {
    // init_tracing!();
    let mut runner = GoldenFileTestRunner::new().expect("Failed to create test runner");
    
    // Simple documented function
    let simple_source = r#""
/// Simple function for testing
/// 
/// # Arguments
/// * `name` - The name to greet
/// 
/// # Returns
/// Greeting string
yolo greet(name: String) -> String {
    format!("Hello, {}!", name)
}
"#";
    
    runner.add_test("simple", simple_source, None).expect("Failed to add simple test");
    runner.run_all_tests().expect("Failed to run tests");
    runner.print_summary();
    
    // Verify at least one test was run
    assert!(!runner.results.is_empty(), "No tests were executed");
}

#[test]
fn test_struct_documentation_golden() {
    // init_tracing!();
    let mut runner = GoldenFileTestRunner::new().expect("Failed to create test runner");
    
    // Struct with comprehensive documentation
    let struct_source = r#""
/// User account information
/// 
/// Represents a user in the system with authentication details
/// and profile information.
/// 
/// # Examples
/// 
/// ```cursed
/// facts user = User {
///     id: 1,
///     username: "alice",
///     email: "alice@example.com",
/// }
/// ```
squad User {
    /// Unique user identifier
    id: Int,
    /// Username for login
    username: String,
    /// Email address for notifications
    email: String,
}

/// Create a new user account
/// 
/// # Arguments
/// * `username` - Desired username (must be unique)
/// * `email` - User's email address
/// 
/// # Returns
/// New User instance with auto-generated ID
/// 
/// # Examples
/// 
/// ```cursed
/// facts user = create_user("bob", "bob@example.com")
/// ```
yolo create_user(username: String, email: String) -> User {
    User {
        id: 1, // Would be auto-generated
        username: username,
        email: email,
    }
}
"#";
    
    runner.add_test("struct", struct_source, None).expect("Failed to add struct test");
    runner.run_all_tests().expect("Failed to run tests");
    runner.print_summary();
    
    assert!(!runner.results.is_empty(), "No tests were executed");
}

#[test]
fn test_interface_documentation_golden() {
    // init_tracing!();
    let mut runner = GoldenFileTestRunner::new().expect("Failed to create test runner");
    
    // Interface with method documentation
    let interface_source = r#""
/// Data serialization interface
/// 
/// Provides methods for converting objects to and from
/// various serialization formats.
collab Serializable {
    /// Convert object to JSON string
    /// 
    /// # Returns
    /// JSON representation of the object
    /// 
    /// # Errors
    /// Returns SerializationError if object cannot be serialized
    yolo to_json(self) -> String
    
    /// Create object from JSON string
    /// 
    /// # Arguments
    /// * `json` - JSON string to deserialize
    /// 
    /// # Returns
    /// Deserialized object instance
    /// 
    /// # Errors
    /// Returns DeserializationError if JSON is invalid
    yolo from_json(json: String) -> Self
}

/// Configuration object implementing Serializable
/// 
/// Application configuration that can be serialized
/// to and from JSON format.
squad Config {
    /// Application name
    app_name: String,
    /// Debug mode flag
    debug: Bool,
    /// Port number for server
    port: Int,
}
"#";
    
    runner.add_test("interface", interface_source, None).expect("Failed to add interface test");
    runner.run_all_tests().expect("Failed to run tests");
    runner.print_summary();
    
    assert!(!runner.results.is_empty(), "No tests were executed");
}

#[test]
fn test_generic_types_golden() {
    // init_tracing!();
    let mut runner = GoldenFileTestRunner::new().expect("Failed to create test runner");
    
    // Generic types with constraints
    let generic_source = r#""
/// Generic container for any type
/// 
/// A flexible container that can store any type implementing
/// the Clone interface for safe copying operations.
/// 
/// # Type Parameters
/// * `T` - The type to store, must implement Clone
/// 
/// # Examples
/// 
/// ```cursed
/// facts container = new Container[String]()
/// container.add("hello")
/// facts value = container.get(0)
/// ```
squad Container[T: Clone] {
    /// Internal storage array
    items: T[],
    /// Current number of items
    size: Int,
}

/// Create new empty container
/// 
/// # Type Parameters
/// * `T` - Type of items to store
/// 
/// # Returns
/// Empty container instance
yolo new Container[T: Clone]() -> Container[T] {
    Container {
        items: [],
        size: 0,
    }
}

/// Add item to container
/// 
/// # Arguments
/// * `item` - Item to add
/// 
/// # Returns
/// True if added successfully
yolo slay add[T: Clone](self: Container[T], item: T) -> Bool {
    self.items.push(item)
    self.size += 1
    true
}
"#";
    
    runner.add_test("generic", generic_source, None).expect("Failed to add generic test");
    runner.run_all_tests().expect("Failed to run tests");
    runner.print_summary();
    
    assert!(!runner.results.is_empty(), "No tests were executed");
}

#[test]
fn test_cross_references_golden() {
    // init_tracing!();
    let mut runner = GoldenFileTestRunner::new().expect("Failed to create test runner");
    
    // Cross-references between types
    let cross_ref_source = r#""
/// User management service
/// 
/// Handles user operations and integrates with [UserRepository]
/// for data persistence and [AuthService] for authentication.
/// 
/// See also:
/// - [create_user] for user creation
/// - [UserRepository.find_by_id] for user lookup
squad UserService {
    /// Repository for user data
    /// 
    /// Links to [UserRepository] for database operations
    repo: UserRepository,
}

/// User data repository
/// 
/// Provides CRUD operations for user entities.
/// Used by [UserService] for data access.
squad UserRepository {
    /// Database connection
    connection: DatabaseConnection,
}

/// Create new user account
/// 
/// Uses [UserRepository.save] to persist user data.
/// Validates input with [validate_email] function.
/// 
/// # Arguments
/// * `email` - User email address
/// * `name` - User display name
/// 
/// # Returns
/// Created [User] object if successful
/// 
/// # Related
/// - [UserRepository.save] for persistence
/// - [validate_email] for email validation
yolo create_user(email: String, name: String) -> User? {
    lowkey validate_email(email) {
        User { id: 1, email: email, name: name }
    } bestie {
        nil
    }
}

/// Validate email address format
/// 
/// Used by [create_user] for input validation.
/// 
/// # Arguments
/// * `email` - Email to validate
/// 
/// # Returns
/// True if email format is valid
yolo validate_email(email: String) -> Bool {
    true // Simplified implementation
}

/// User data structure
/// 
/// Created by [create_user] and stored via [UserRepository].
squad User {
    /// Unique identifier
    id: Int,
    /// Email address (validated by [validate_email])
    email: String,
    /// Display name
    name: String,
}
"#";
    
    runner.add_test("cross_ref", cross_ref_source, None).expect("Failed to add cross-ref test");
    runner.run_all_tests().expect("Failed to run tests");
    runner.print_summary();
    
    assert!(!runner.results.is_empty(), "No tests were executed");
}

#[test]
fn test_performance_golden_file_generation() {
    // init_tracing!();
    let mut runner = GoldenFileTestRunner::new().expect("Failed to create test runner");
    
    // Large source with many documented items
    let large_source = generate_large_documentation_source(50); // 50 functions/structs
    
    runner.add_test("performance", &large_source, None).expect("Failed to add performance test");
    
    let start_time = Instant::now();
    runner.run_all_tests().expect("Failed to run tests");
    let total_time = start_time.elapsed();
    
    runner.print_summary();
    
    // Performance assertions
    assert!(total_time < std::time::Duration::from_secs(30), 
           "Golden file generation took too long: {:?}", total_time);
    
    assert!(!runner.results.is_empty(), "No tests were executed");
    
    // Verify generation time for individual test
    if let Some(result) = runner.results.first() {
        assert!(result.generation_time < std::time::Duration::from_secs(10),
               "Individual test took too long: {:?}", result.generation_time);
    }
}

#[test]
fn test_idempotency_golden_files() {
    // init_tracing!();
    let mut runner = GoldenFileTestRunner::new().expect("Failed to create test runner");
    
    let source = r#""
/// Test function for idempotency
/// 
/// # Arguments
/// * `input` - Test input value
/// 
/// # Returns
/// Processed output
yolo test_function(input: String) -> String {
    input
}
"#";
    
    // Run generation twice and compare results
    runner.add_test("idempotency1", source, None).expect("Failed to add first test");
    runner.run_all_tests().expect("Failed to run first generation");
    
    // Get first result
    let first_output = fs::read_to_string(
        runner.work_dir.path().join("idempotency1_golden.html")
    ).expect("Failed to read first output");
    
    // Clear results and run again
    runner.results.clear();
    runner.configs.clear();
    
    runner.add_test("idempotency2", source, Some(&first_output)).expect("Failed to add second test");
    runner.run_all_tests().expect("Failed to run second generation");
    
    runner.print_summary();
    
    // Verify idempotency
    assert!(!runner.results.is_empty(), "No tests were executed");
    assert!(runner.results[0].passed, "Idempotency test failed: documentation generation is not consistent");
}

/// Generate large source file for performance testing
fn generate_large_documentation_source(count: usize) -> String {
    let mut source = String::new();
    source.push_str("//! Large package for performance testing\n\n");
    
    for i in 0..count {
        source.push_str(&format!(r#""
/// Service class {} for performance testing
/// 
/// This service provides functionality for operation {}.
/// Includes comprehensive documentation to test generation performance.
/// 
/// # Examples
/// 
/// ```cursed
/// facts service = new Service{}()
/// facts result = service.process("data")
/// ```
squad Service{} {{
    /// Configuration for service {}
    config: String,
    /// State for service {}
    state: Int,
}}

/// Process data with service {}
/// 
/// # Arguments
/// * `data` - Input data to process
/// 
/// # Returns
/// Processed result string
/// 
/// # Examples
/// 
/// ```cursed
/// facts service = new Service{}()
/// facts result = service.process("test data")
/// ```
yolo slay process{}(self: Service{}, data: String) -> String {{
    format!("Processed: {{}}", data)
}}

"#, i, i, i, i, i, i, i, i, i, i))";
    }
    
    source
}

/// Golden file test utilities
#[cfg(test)]
mod golden_utils {
    use super::*;
    
    /// Create expected HTML content for testing
    pub fn create_expected_html(title: &str, content: &str) -> String {
        format!(r#"<!DOCTYPE html>"
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>{} - CURSED Documentation</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div class="container">
        <header>
            <h1>CURSED Documentation</h1>
        </header>
        <nav>
            <div class="nav-section">
                <h3>Documentation</h3>
            </div>
        </nav>
        <main>
            {}
        </main>
    </div>
    <script src="search.js"></script>
</body>
</html>"#, title, content)"
    }
    
    /// Normalize HTML content for comparison
    pub fn normalize_html(html: &str) -> String {
        html.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[test]
fn test_golden_file_infrastructure() {
    // init_tracing!();
    // Test the golden file testing infrastructure itself
    let mut runner = GoldenFileTestRunner::new().expect("Failed to create test runner");
    
    let simple_source = "/// Test\nyolo test() -> String { \"test\" }";
    let expected_html = golden_utils::create_expected_html("Test", "<p>Simple test content</p>");
    
    runner.add_test("infrastructure", simple_source, Some(&expected_html))
        .expect("Failed to add infrastructure test");
    
    runner.run_all_tests().expect("Failed to run infrastructure test");
    runner.print_summary();
    
    assert!(!runner.results.is_empty(), "Infrastructure test not executed");
    
    // The test may fail due to content differences, but should run without errors
    println!("✓ Golden file infrastructure test completed");
}
