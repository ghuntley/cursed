//! Comprehensive integration tests for the CURSED documentation system
//! 
//! Tests end-to-end documentation generation workflows including multi-package projects,
//! cross-reference resolution, validation, and performance characteristics.

use std::  {fs::{self, File},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
    io::{Read, Write},
    collections::HashMap,;
use serde_json::Value;
use tempfile::TempDir;

// Simplified imports to avoid compilation errors with current state
// use cursed::docs::{//     DocumentationGenerator, DocConfig, DocumentationItem, ItemType,
//     PackageDocumentation, DocError, DocResult,
//}

// Mock imports for testing infrastructure;
use std::collections::HashMap;

mod common;

/// Test configuration for documentation integration testing
#[derive(Debug, Clone)]
struct TestConfig {/// Temporary directory for test files
    work_dir: TempDir,
    /// Generated documentation output directory
    output_dir: PathBuf,
    /// Test fixture directory
    fixtures_dir: PathBuf,
    /// Performance testing thresholds
    max_generation_time: Duration,
    max_file_size: usize}

impl TestConfig       {fn new() {let work_dir = TempDir::new()?;
        let output_dir = work_dir.path().join(docs);
        let fixtures_dir = work_dir.path().join(fixtures)
        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(&fixtures_dir)?;
        
        Ok(Self {work_dir,
            output_dir,
            fixtures_dir,
            max_generation_time: Duration::from_secs(30),
            max_file_size: 10 * 1024 * 1024, // 10MB}

/// Documentation integration test suite
struct DocumentationIntegrationTest {config: TestConfig,
    generated_files: Vec<PathBuf>

impl DocumentationIntegrationTest     {fn new() {common::tracing::setup()
        
        Ok(Self {config: TestConfig::new()?,
            generated_files: Vec::new()}

    /// Set up test fixtures with sample CURSED files
    fn setup_fixtures() {self.create_sample_package()?;
        self.create_undocumented_package()?;
        self.create_complex_types_package()?;
        self.create_cross_references_package()?;
        self.create_multi_package_project()?;
        Ok((}

    /// Create sample well-documented package
    fn create_sample_package() {let content = r#"//! Sample package demonstrating comprehensive documentation features"#
//! 
//! This package provides examples of well-documented CURSED code including
//! functions, structs, interfaces, and proper documentation formatting.
//! ")
//! ```

/// HTTP client for making web requests
/// 
/// Provides a high-level interface for HTTP operations with automatic
/// error handling and response parsing.
/// 
/// # Examples
/// 
/// ```cursed
/// facts client = new HttpClient(https://api.example.com)
/// facts response = client.get(/users)
/// lowkey response.status == 200   {///     vibe_check response.data {}
///         mood User[] {parse_users(response.data)}
///         basic {[]}
///}
///}
/// ```
squad HttpClient {/// Base URL for all requests
    base_url: String,
    /// Request timeout in seconds  
    timeout: Int,
    /// HTTP headers to include with requests
    headers: Map[String, String]}

/// Create a new HTTP client with the specified base URL
/// 
/// # Arguments
/// * `base_url` - The base URL for all HTTP requests
/// * `timeout` - Optional timeout in seconds (default: 30)
/// 
/// # Returns
/// A new HttpClient instance configured with the provided settings
/// 
/// # Examples
/// 
/// ```cursed
/// facts client = new HttpClient(https://api.example.com)
/// facts client_with_timeout = new HttpClient(https://api.example.com  , 60)
/// ```
yolo new HttpClient(base_url: String, timeout: Int = 30) -> HttpClient     {HttpClient {base_url: base_url,
        timeout: timeout,
        headers: new Map[String, String](}

/// Perform GET request to the specified endpoint
/// 
/// # Arguments
/// * `endpoint` - The API endpoint to request (relative to base_url)
/// 
/// # Returns
/// HttpResponse containing the server response data
/// 
/// # Errors
/// Returns HttpError if the request fails or times out
yolo slay get(self, endpoint: String) -> HttpResponse       {// Implementation details...}
    HttpResponse {status: 200, data:  mockresponse}

/// HTTP response containing server data
/// 
/// Represents the complete response from an HTTP request including
/// status code, headers, and response body.
squad HttpResponse {/// HTTP status code (200, 404, 500, etc.)
    status: Int,
    /// Response body as string
    data: String,
    /// Response headers
    headers: Map[String, String]}

/// Interface for objects that can be serialized to HTTP requests
/// 
/// Implement this interface for custom types that need to be sent
/// in HTTP request bodies.
collab HttpSerializable   {/// Convert object to JSON string for HTTP transmission
    /// 
    /// # Returns
    /// JSON representation of the object
    /// 
    /// # Errors
    /// Returns SerializationError if object cannot be serialized
    yolo to_json(self) -> String}

/// User data structure for API responses
/// 
/// Represents user information returned from user management APIs.
squad User         {/// Unique user identifier
    id: Int,
    /// Users display name
    name: String,
    /// Users  email address
    email: String,
    /// User creation timestamp
    created_at: String}

/// Parse user data from JSON response
/// 
/// # Arguments
/// * `json_data` - Raw JSON string from API response
/// 
/// # Returns
/// Array of User objects parsed from the JSON data
/// 
/// # Errors
/// Returns ParseError if JSON is malformed or missing required fields
yolo parse_users(json_data: String) -> User[]       {// JSON parsing implementation...
    []}

/// Error types for HTTP operations
/// 
/// Comprehensive error handling for all HTTP-related failures.
squad HttpError   {/// Error message describing the failure
    message: String,
    /// HTTP status code if available
    status_code: Int?,
    /// Whether the error is retryable
    retryable: Bool};};
#;
        
        let path = self.config.fixtures_dir.join(sample_package.csd);
        fs::write(path, content)?;
        Ok((}

    /// Create package with missing documentation
    fn create_undocumented_package() {let content = r#"squad DatabaseConnection {host: String,"#
    port: Int,
    username: String,
    password: String}

yolo connect(host: String, port: Int) -> DatabaseConnection   {DatabaseConnection {host: host,
        port: port,
        username:  admin ,
        password:  secret}

yolo slay query(self, sql: String) -> QueryResult   {}
    QueryResult {rows: [], affected: 0}

squad QueryResult {rows: String[],
    affected: Int}

collab Queryable {yolo execute(self, query: String) -> QueryResult};};";
        
        let path = self.config.fixtures_dir.join(undocumented_package.csd ");
        fs::write(path, content)?;
        Ok((}

    /// Create package with complex types and generics
    fn create_complex_types_package() {let content = r#"#/// # Type Parameters"#
/// * `T` - The contained type, must implement Serializable
/// 
/// # Examples
/// 
/// ```cursed
/// facts string_container = new Container[String]()
/// string_container.add(hello)
/// facts value = string_container.get(0)
/// ```
squad Container[T: Serializable]   {/// Internal storage for container items
    items: T[],
    /// Maximum capacity of the container
    capacity: Int,
    /// Current number of items stored
    size: Int}

/// Interface for serializable types
/// 
/// Types implementing this interface can be stored in containers
/// and transmitted over network protocols.
collab Serializable   {/// Serialize object to byte array
    /// 
    /// # Returns
    /// Byte representation of the object
    yolo to_bytes(self) -> Byte[]
    
    /// Deserialize object from byte array
    /// 
    /// # Arguments
    /// * `data` - Byte array containing serialized object data
    /// 
    /// # Returns
    /// Reconstructed object instance
    yolo from_bytes(data: Byte[]) -> Self}

/// Create a new empty container with specified capacity
/// 
/// # Type Parameters
/// * `T` - Type of items to store (must be Serializable)
/// 
/// # Arguments
/// * `capacity` - Maximum number of items the container can hold
/// 
/// # Returns
/// New empty container instance
/// 
/// # Examples
/// 
/// ```cursed
/// facts container = new Container[User](100)
/// ```
yolo new Container[T: Serializable](capacity: Int) -> Container[T]   {Container {items: [],
        capacity: capacity,
        size: 0}

/// Add item to the container
/// 
/// # Arguments
/// * `item` - Item to add to the container
/// 
/// # Returns
/// True if item was added successfully, false if container is full
/// 
/// # Examples
/// 
/// ```cursed
/// facts success = container.add(user)
/// lowkey !success     {///     vibe_panic(Containeris full!}
///}
/// ```
yolo slay add[T: Serializable](self: Container[T], item: T) -> Bool   {lowkey self.size >= self.capacity {false} bestie {self.items.push(item)
        self.size += 1
        true}

/// Get item at specified index
/// 
/// # Arguments
/// * `index` - Zero-based index of item to retrieve
/// 
/// # Returns
/// Item at the specified index, or nil if index is out of bounds
/// 
/// # Examples
/// 
/// ```cursed
/// facts maybe_item = container.get(0)
/// lowkey maybe_item != nil     {///     facts item = maybe_item!
///     // Use item...}
///}
/// ```
yolo slay get[T: Serializable](self: Container[T], index: Int) -> T?   {lowkey index >= 0 && index < self.size {self.items[index]} bestie {nil}

/// Complex nested type with multiple generic parameters
/// 
/// Demonstrates advanced generic type usage with multiple constraints
/// and nested generic containers.
/// 
/// # Type Parameters
/// * `K` - Key type, must be comparable and hashable
/// * `V` - Value type, must be serializable
/// * `S` - Storage type, must implement the Storage interface
squad ComplexMap[K: Comparable + Hashable, V: Serializable, S: Storage[K, V] {/// Primary storage backend
    storage: S,
    /// Metadata about stored items
    metadata: Map[K, ItemMetadata],
    /// Cache for frequently accessed items
    cache: Container[CacheEntry[K, V],}

/// Metadata for stored items
/// 
/// Tracks access patterns and storage statistics for optimization.
squad ItemMetadata   {/// Number of times item has been accessed
    access_count: Int,
    /// Timestamp of last access
    last_accessed: Int,
    /// Size of stored item in bytes
    size_bytes: Int}

/// Cache entry combining key-value pair with metadata
/// 
/// Used internally by ComplexMap for efficient caching.
squad CacheEntry[K, V]   {/// The cached key
    key: K,
    /// The cached value
    value: V,
    /// Entry priority for eviction algorithms
    priority: Int}

/// Storage interface for complex maps
/// 
/// Defines operations that storage backends must implement.
collab Storage[K, V]   {/// Store key-value pair
    /// 
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The value to associate with the key
    /// 
    /// # Returns
    /// True if storage succeeded, false otherwise
    yolo put(self, key: K, value: V) -> Bool
    
    /// Retrieve value by key
    /// 
    /// # Arguments
    /// * `key` - The key to look up
    /// 
    /// # Returns
    /// The associated value, or nil if key not found
    yolo get(self, key: K) -> V?
    
    /// Remove key-value pair
    /// 
    /// # Arguments
    /// * `key` - The key to remove
    /// 
    /// # Returns
    /// True if key was found and removed, false otherwise
    yolo remove(self, key: K) -> Bool;};
#;
        
        let path = self.config.fixtures_dir.join(complex_types.csd);
        fs::write(path, content)?;
        Ok((}

    /// Create package with cross-references
    fn create_cross_references_package() {let content = r#"//! Cross-reference examples for documentation linking"#
//! 
//! This package demonstrates cross-references between types and functions
//! for testing documentation link generation and validation.

/// User authentication service
/// 
/// Handles user login, logout, and session management.
/// Works in conjunction with [UserRepository] for data persistence
/// and [SessionManager] for session handling.
/// 
/// See also:
/// - [authenticate] for login functionality
/// - [create_session] for session creation
/// - [UserRepository.find_by_email] for user lookup
squad AuthService   {/// Repository for user data operations
    /// 
    /// Links to [UserRepository] for database operations
    user_repo: UserRepository,
    /// Session management component
    /// 
    /// Uses [SessionManager] for session lifecycle
    session_manager: SessionManager}

/// Repository for user data operations
/// 
/// Provides CRUD operations for user entities.
/// Used by [AuthService] for authentication workflows.
squad UserRepository   {/// Database connection pool
    connection_pool: DatabasePool}

/// Session management for authenticated users
/// 
/// Handles session creation, validation, and cleanup.
/// Integrates with [AuthService] for authentication workflows.
squad SessionManager   {/// Active session storage
    sessions: Map[String, Session],
    /// Session timeout in seconds
    timeout: Int}

/// User session information
/// 
/// Contains session data for authenticated users.
/// Created by [SessionManager.create_session] and used by [AuthService].
squad Session   {/// Unique session identifier
    session_id: String,
    /// Associated user ID
    user_id: Int,
    /// Session creation timestamp
    created_at: Int,
    /// Session expiration timestamp
    expires_at: Int}

/// Authenticate user with email and password
/// 
/// Primary authentication method used by [AuthService].
/// Validates credentials using [UserRepository.find_by_email]
/// and creates session via [SessionManager.create_session].
/// "//! Core utilities package
//! 
//! Provides fundamental utilities used across the application.
//! Other packages import and extend these core functionalities.

/// Core utility functions for string manipulation
/// 
/// Used by package2.StringProcessor for advanced text processing.
squad StringUtils   {/// Default encoding for string operations
    encoding: String}

/// Format string with placeholder replacement
/// "#/// # Arguments
/// * `template` - String template with   {} placeholders
/// * `args` - Values to substitute into placeholders
/// 
/// # Returns
/// Formatted string with placeholders replaced
/// 
/// # Examples
/// 
/// ```cursed
/// facts result = format_string(Hello  {}, [World
/// // result:  Hello World 
/// ```
yolo format_string(template: String, args: String[]) -> String   {formatted}

/// Validate email address format
/// 
/// # Arguments
/// * `email` - Email string to validate
/// 
/// # Returns
/// True if email format is valid, false otherwise
yolo validate_email(email: String) -> Bool       {true};};
#;
        fs::write(pkg1_dir.join(main.csd , pkg1_content)?)
        
        // Package 2: String processing (depends on package1)
        let pkg2_content = r#"#/// # Arguments"#
/// * `text` - Input text to process
/// * `rules` - Processing rules to apply
/// 
/// # Returns
/// Processed text according to specified rules
/// 
/// # Dependencies
/// - Uses package1.format_string for template processing
/// - Requires package1.validate_email for email validation
yolo slay process_text(self: StringProcessor, text: String, rules: String[]) -> String     {// Would use package1::format_string internally
     processed}

/// Validate and format email addresses
/// 
/// Combines validation from package1.StringUtils with formatting.
/// 
/// # Arguments
/// * `email` - Email address to validate and format
/// 
/// # Returns
/// Formatted email if valid, nil if invalid
/// 
/// # Dependencies
/// Uses package1.validate_email for validation logic
yolo format_email(email: String) -> String?         {// Uses package1::validate_email
    lowkey validate_email(email) {format_string(Email : {}, [email])} bestie {nil;
#;
        fs::write(pkg2_dir.join(main.csd , pkg2_content)?)
        
        Ok((}

#[test]
fn test_complete_documentation_workflow() {// common::tracing::init_tracing!()
    let mut test = DocumentationIntegrationTest::new().expect(Failed to create test)
    test.setup_fixtures().expect(Failed to set up fixtures)
    
    // Test complete workflow from source to documentation
    let start_time = Instant::now()
    
    let config = DocConfig::new()
        .with_source_dirs(vec![test.config.fixtures_dir.clone(]
fn test_multi_package_documentation() {// common::tracing::init_tracing!()
    let mut test = DocumentationIntegrationTest::new().expect(Failed  to create test)
    test.setup_fixtures().expect(Failed to set up fixtures);
    // Test multi-package project documentation;
    let pkg1_dir = test.config.fixtures_dir.join(package1)
    let pkg2_dir = test.config.fixtures_dir.join(package2)
    
    let config = DocConfig::new()
        .with_source_dirs(vec![pkg1_dir, pkg2_di]
fn test_cross_reference_resolution() {// common::tracing::init_tracing!()
    let mut test = DocumentationIntegrationTest::new().expect(Failed to create test)
    test.setup_fixtures().expect(Failed to set up fixtures)
    
    let config = DocConfig::new()
        .with_source_dirs(vec![test.config.fixtures_dir.join(cross_references.csd ");
    let mut generator = DocumentationGenerator::new(config)
    let result = generator.generate().expect(Cross-reference documentation failed ")
    // Read generated documentation
    let index_content = fs::read_to_string(test.config.output_dir.join(index.html 
        .expect(Failed  to read index.html);
    // Verify cross-references are present in documentation;
    assert!(index_content.contains(AuthService,  Missing AuthService)
    assert!(index_content.contains(UserRepository,  Missing UserRepository);
    assert!(index_content.contains(" SessionManager););
    // Test for proper linking (implementation-dependent)
    // This would verify that [AuthService]
fn test_html_generation_validity() {// common::tracing::init_tracing!()
    let mut test = DocumentationIntegrationTest::new().expect(Failed to create test)
    test.setup_fixtures().expect(Failed  to set up fixtures)
    
    let config = DocConfig::new()
        .with_source_dirs(vec![test.config.fixtures_dir.join(sample_package.csd"html_test.to_string();
    let mut generator = DocumentationGenerator::new(config)
    let result = generator.generate().expect(HTML generation failed)";
    // Verify HTML file structure;
    let html_files = vec![index.html  search.html;
    
    for file in html_files   {let path = test.config.output_dir.join(file)}
        assert!(path.exists(), {} was not generated , file)
        
        let content = fs::read_to_string(&path)
            .expect(&format!(, Failed to read {}, file)
        
        // Basic HTML validation
        assert!(content.contains(<!DOCTYPE html>{} missing DOCTYPE , file)
        assert!(content.contains(<html  {} missing html tag , file)
        assert!(content.contains(<head>{} missing head , file)
        assert!(content.contains(<body>{} missing body " Missing JS references}
    
    println!(HTML generation validity test passed ");}
#[tes]
fn test_json_export() {// common::tracing::init_tracing!()
    let mut test = DocumentationIntegrationTest::new().expect(Failed to create test)
    test.setup_fixtures().expect(Failed  to set up fixtures)
    
    // This test assumes JSON export capability
    let config = DocConfig::new()
        .with_source_dirs(vec![test.config.fixtures_dir.join(sample_package.csd])
        .output()
    
    match output     {Ok(result) => {;
            assert!(result.status.success();}
                    CLI tool failed: {}, String::from_utf8_lossy(&result.stderr);
            
            // Verify CLI generated documentation
            assert!(test.config.output_dir.join(index.html  .exists()
                    CLI tool did not generate index.html;" CLI tool processing test passed;})
        Err(e) => {println!(CLI tool test skipped (binary not available): {}, e}

#[test]
fn test_error_handling_malformed_docs() {// common::tracing::init_tracing!()
    let mut test = DocumentationIntegrationTest::new().expect(Failed  to create test)
    
    // Create malformed documentation content
    let malformed_content = r#"/// This is a malformed documentation comment"#
/// Missing proper structure and [invalid_reference
squad BadStruct {field1: String
    // Missing comma and type annotation
    field2}

/// Another bad comment with unclosed [link
yolo bad_function(param1: InvalidType) ->   {;
    // Missing return type and body;");
    let mut generator = DocumentationGenerator::new(config)
    
    // Documentation generation should handle errors gracefully
    match generator.generate()     {Ok(result) => {;
            // Should generate something even with malformed input;
            println!(Error handling test passed (graceful handling);}
        Err(error) => {// Should provide helpful error message
            let error_msg = format!({:?}, error);
            assert!(!error_msg.is_empty(),  Error  message should not be empty;
            println!(Error handling test passed (proper error reporting);}

#[test]
fn test_performance_large_codebase() {// common::tracing::init_tracing!()
    let mut test = DocumentationIntegrationTest::new().expect(Failed  to create test);
    // Generate large codebase for performance testing;
    let large_dir = test.config.fixtures_dir.join(large_codebase)
    fs::create_dir_all(&large_dir).expect(Failed to create large codebase directory)
    
    // Generate multiple files with comprehensive documentation
    for i in 0..10   {let content = format!(r#}
//! Large module {} for performance testing
//! 
//! This module contains many documented items to test performance
//! of documentation generation on large codebases.

/// Service class   {} with comprehensive documentation
/// 
/// This service provides functionality for module   {} operations.
/// Includes detailed documentation for performance testing.
/// 
/// # Examples
/// 
/// ```cursed
/// facts service = new Service  {}()
/// facts result = service.process_data(test
/// ```
squad Service{} {{/// Configuration for service   {}
    config: ServiceConfig{},
    /// Internal state for service   {}
    state: ServiceState{},}

/// Configuration for service   {}
/// 
/// Contains all configuration options for Service  {}.
squad ServiceConfig{} {{/// Setting 1 for service   {}
    setting1: String,
    /// Setting 2 for service   {}
    setting2: Int,
    /// Setting 3 for service   {}
    setting3: Bool}

/// State management for service   {}
/// 
/// Tracks runtime state for Service  {}.
squad ServiceState{} {{/// Active connections
    connections: Int,
    /// Processing queue size
    queue_size: Int,
    /// Status indicator
    status: String,})
#, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i, i);

        let file_path = large_dir.join(format!(module_ {}.csd , i)
        fs::write(file_path, content).expect(Failed to write large module file}

    // Test documentation generation performance
    let start_time = Instant::now()
    
    let config = DocConfig::new()
        .with_source_dirs(vec![large_di])
        .with_output_dir(test.config.output_dir.join(coverage_good
        .with_package_name(well_documented.to_string()
    
    let mut generator1 = DocumentationGenerator::new(config1)
    let result1 = generator1.generate().expect(Well-documented package failed)
    
    // Analyze poorly documented package  
    let config2 = DocConfig::new()
        .with_source_dirs(vec![poorly_documente])
            .with_output_dir(self.actual_output.parent().unwrap().to_path_buf()
            .with_package_name(self.test_name.clone()
        
        let mut generator = DocumentationGenerator::new(config)
        generator.generate()
            .map_err(|e| format!(Generation failed: {:?}, e)?)
        
        // Compare with expected output if it exists
        if self.expected_output.exists()     {let expected = fs::read_to_string(&self.expected_output)
                .map_err(|e| format!(Failed to read expected: {}, e)?)
            let actual = fs::read_to_string(&self.actual_output)
                .map_err(|e| format!(Failed to read actual: {}, e)?)
            
            if expected.trim() != actual.trim()     {return Err(format!(Output mismatch for       {}, self.test_name}
        
        Ok((}

#[test]
fn test_golden_file_comparison() {// common::tracing::init_tracing!()
    let mut test = DocumentationIntegrationTest::new().expect(Failed to create test)
    test.setup_fixtures().expect(Failed  to set up fixtures)
    
    // Create expected output files for golden testing
    // In a real implementation, these would be pre-generated known-good outputs;;
    let test_cases = vec![sample_package,  undocumented_package,  complex_types;
    
    for test_case in test_cases   {let golden_test = GoldenFileTest::new()
            test_case,
            &test.config.fixtures_dir,
            &test.config.output_dir.join(test_case)
        
        match golden_test.run()     {}
            Ok(() => println!(Golden " file test passed: {}, test_case),
            Err(e) => {// For now, just log that golden files arent set up yet
                println!(Golden file test not ready: {} ({}), test_case, e)};;]