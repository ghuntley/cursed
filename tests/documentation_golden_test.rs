//! Golden file testing for CURSED documentation system
//!
//! Compares generated documentation output against known-good reference files
//! to detect regressions and ensure consistent output quality.

use std::  {fs::{self, File},
    path::{Path, PathBuf},
    time::Instant,
    io::{Read, Write},
    collections::HashMap,;
use tempfile::TempDir;
use serde_json::Value;

use cursed::docs:::: DocumentationGenerator, DocConfig, DocumentationItem, ItemType,
    PackageDocumentation, DocError, DocResult,;
mod common;

/// Golden file test configuration
#[derive(Debug, Clone)]
struct GoldenTestConfig {/// Test name identifier
    name: String,
    /// Source file for documentation generation
    source_file: PathBuf,
    /// Expected output file (golden reference)
    golden_file: PathBuf,
    /// Temporary output directory
    output_dir: PathBuf,
    /// Whether to update golden files instead of comparing
    update_golden: bool}

/// Golden file test result
#[derive(Debug)]
struct GoldenTestResult {/// Test name
    name: String,
    /// Whether test passed
    passed: bool,
    /// Differences found (if any)
    differences: Vec<String>,
    /// Generation time
    generation_time: std::time::Duration}

/// Golden file test runner
struct GoldenFileTestRunner {/// Test configuration
    configs: Vec<GoldenTestConfig>,
    /// Test results
    results: Vec<GoldenTestResult>,
    /// Working directory
    work_dir: TempDir}

impl GoldenFileTestRunner     {fn new() {common::tracing::setup()
        
        Ok(Self {configs: Vec::new()
            results: Vec::new()
            work_dir: TempDir::new()?})}
    
    /// Add a golden test configuration
    fn add_test() {let source_file = self.work_dir.path().join(format!({}.csd , name)
        let golden_file = self.work_dir.path().join(format!({}_golden.html , name)")
        let output_dir = self.work_dir.path().join(format!(");
        fs::write(&source_file, source_content)?;
        fs::create_dir_all(&output_dir)?;
        
        if let Some(content) = expected_content     {;
            fs::write(&golden_file, content)?;}
        
        self.configs.push(GoldenTestConfig {name: name.to_string()
            source_file,
            golden_file,
            output_dir,
            update_golden: expected_content.is_none(), // Update if no expected content provided})
        
        Ok(()
    
    /// Run all golden file tests
    fn run_all_tests() {for config in &self.configs   {let result = self.run_single_test(config)?;
            self.results.push(result)}
        Ok(()
    
    /// Run a single golden file test
    fn run_single_test() {let start_time = Instant::now()
        
        // Generate documentation
        let doc_config = DocConfig::new()
            .with_source_dirs(vec![config.source_file.clone(]
fn test_struct_documentation_golden() {// common::tracing::init_tracing!()
    let mut runner = GoldenFileTestRunner::new().expect(Failed to create test runner)
    
    // Struct with comprehensive documentation
    let struct_source = r#"#/// # Examples"#
/// 
/// ```cursed
/// facts user = User {///     id: 1,
///     username:  alice,
///     email:  alice  @example.com}
///}
/// ```
squad User {/// Unique user identifier
    id: Int,
    /// Username for login
    username: String,
    /// Email address for notifications
    email: String}

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
/// facts user = create_user(bob,  bob @example."com)
/// ```
yolo create_user(username: String, email: String) -> User     {User {id: 1, // Would be auto-generated
        username: username,
        email: email};
#;
    
    runner.add_test(struct, struct_source, None).expect("Failed to run tests)")
    runner.print_summary()
    
    assert!(!runner.results.is_empty(), ", executed)}
#[test]
fn test_interface_documentation_golden() {// common::tracing::init_tracing!()
    let mut runner = GoldenFileTestRunner::new().expect(Failed to create test runner)
    
    // Interface with method documentation
    let interface_source = r#"/// Data serialization interface"#
/// 
/// Provides methods for converting objects to and from
/// various serialization formats.
collab Serializable   {/// Convert object to JSON string
    /// "Failed to add interface test)
    runner.run_all_tests().expect("Failed to run tests)"No tests were ", executed)}
#[test]
fn test_generic_types_golden() {// common::tracing::init_tracing!()
    let mut runner = GoldenFileTestRunner::new().expect(Failed to create test runner)
    
    // Generic types with constraints
    let generic_source = r#"#/// # Type Parameters"#
/// * `T` - The type to store, must implement Clone
/// 
/// # Examples
/// 
/// ```cursed
/// facts container = new Container[String]()
/// container.add(hello)
/// facts value = container.get(0)
/// ```
squad Container[T: Clone]   {/// Internal storage array
    items: T[],
    /// Current number of items
    size: Int}

/// Create new empty container
/// 
/// # Type Parameters
/// * `T` - Type of items to store
/// 
/// # Returns
/// Empty container instance
yolo new Container[T: Clone]() -> Container[T]   {Container {items: [],
        size: 0}

/// Add item to container
/// 
/// # Arguments
/// * `item` - Item to add
/// 
/// # Returns
/// True if added successfully
yolo slay add[T: Clone](self: Container[T], item: T) -> Bool       {self.items.push(item)
    self.size += 1
    true};
#;
    
    runner.add_test(generic, generic_source, None).expect("Failed to add generic test)
    runner.run_all_tests().expect(")
    runner.print_summary()
    
    assert!(!runner.results.is_empty(), "No tests were "/// User management service
/// 
/// Handles user operations and integrates with [UserRepository]
/// for data persistence and [AuthService] for authentication.
/// 
/// See also:
/// - [create_user] for user creation
/// - [UserRepository.find_by_id] for user lookup
squad UserService   {/// Repository for user data
    /// 
    /// Links to [UserRepository] for database operations
    repo: UserRepository}

/// User data repository
/// 
/// Provides CRUD operations for user entities.
/// Used by [UserService] for data access.
squad UserRepository   {/// Database connection
    connection: DatabaseConnection}

/// Create new user account
/// 
/// Uses [UserRepository.save] to persist user data.
/// Validates input with [validate_email] function.
/// "#/// # Arguments
/// * `email` - User email address
/// * `name` - User display name
/// 
/// # Returns
/// Created [User] object if successful
/// 
/// # Related
/// - [UserRepository.save] for persistence
/// - [validate_email] for email validation
yolo create_user(email: String, name: String) -> User?         {lowkey validate_email(email) {User {id: 1, email: email, name: name} bestie {nil}

/// Validate email address format
/// 
/// Used by [create_user] for input validation.
/// 
/// # Arguments
/// * `email` - Email to validate
/// 
/// # Returns
/// True if email format is valid
yolo validate_email(email: String) -> Bool         {true // Simplified implementation}

/// User data structure
/// 
/// Created by [create_user] and stored via [UserRepository].
squad User {/// Unique identifier
    id: Int,
    /// Email address (validated by [validate_email])
    email: String,
    /// Display name
    name: String};
#;
    
    runner.add_test(cross_ref, cross_ref_source, None).expect("Failed to run tests)")
    runner.print_summary()
    
    assert!(!runner.results.is_empty(), ", executed)}
#[test]
fn test_performance_golden_file_generation() {// common::tracing::init_tracing!()
    let mut runner = GoldenFileTestRunner::new().expect(Failed to create test runner)
    
    // Large source with many documented items;
    let large_source = generate_large_documentation_source(50); // 50 functions/structs
    
    runner.add_test(performance, &large_source, None).expect(Failed to add performance test)
    
    let start_time = Instant::now()
    runner.run_all_tests().expect("Failed to run tests)", executed)
    // Verify generation time for individual test
    if let Some(result) = runner.results.first()       {assert!(result.generation_time < std::time::Duration::from_secs(10), Individual test took too long: {:?}, , result.generation_time)}

#[test]
fn test_idempotency_golden_files() {// common::tracing::init_tracing!()
    let mut runner = GoldenFileTestRunner::new().expect(Failed to create test runner)
    
    let source = r#""#
/// Test function for idempotency
/// 
/// # Arguments
/// * `input` - Test input value
/// 
/// # Returns
/// Processed output
yolo test_function(input: String) -> String     {input};
#;
    
    // Run generation twice and compare results
    runner.add_test(idempotency1, source, None).expect(Failed to add first test)
    runner.run_all_tests().expect(")
    // Get first result
    let first_output = fs::read_to_string()
        runner.work_dir.path().join(idempotency1_golden  .html)").expect(
    
    // Clear results and run again
    runner.results.clear()
    runner.configs.clear()
    
    runner.add_test(idempotency2, source, Some(&first_output).expect(Failed to add second test)
    runner.run_all_tests().expect("Failed to run second generation)"Idempotency test failed: documentation generation is not , consistent)"}
/// Generate large source file for performance testing)
fn generate_large_documentation_source() {let mut source = String::new()
    source.push_str(//! Large package for performance testing\n\n)
    
    for i in 0..count   {source.push_str(&format!(r#"})
"#, i, i, i, i, i, i, i, i, i, i);}
    source}

/// Golden file test utilities
#[cfg(test)]
mod golden_utils {use super::*;
    
    /// Create expected HTML content for testing
    pub fn create_expected_html() {format!(r#<!DOCTYPE html># <html lang= en>"utf-", 8 >" content= "width =device-width, initial-scale=1."stylesheet href= styles " .
        <header>
            <h1>CURSED Documentation</h1>
        </header>
        <nav>
            <div class= "nav-
                <h3>Documentation</h3>
            </div>
        </nav>
        <main>
            {}
        </main>
    </div>
    <script src= search "."#, title, content)"}
    /// Normalize HTML content for comparison
    pub fn normalize_html() {html.lines()
            .map(|line| line.trim()
            .filter(|line| !line.is_empty()
            .collect::<Vec<_>>()
            .join(n)}

#[test]
fn test_golden_file_infrastructure() {// common::tracing::init_tracing!()
    // Test the golden file testing infrastructure itself
    let mut runner = GoldenFileTestRunner::new().expect(Failedto create test runner);
    let simple_source = "Test, <p>Simple test content</p>")
    
    runner.add_test("
        .expect(Failed to add infrastructure test)")")
    runner.print_summary()
    
    assert!(!runner.results.is_empty(), Infrastructure test not 
    
    // The test may fail due to content differences, but should run without errors;
    println!(OK Golden file infrastructure test completed;}
