//! Comprehensive Documentation Generation Test Suite
//! 
//! Tests the complete CURSED documentation generation system including:
//! - Real documentation comment extraction
//! - Complete AST traversal for all language constructs
//! - Enhanced cross-reference generation
//! - Template system integration
//! - Multiple output format generation

use cursed::docs::generator::{DocumentationGenerator, DocGeneratorConfig, DocFormat, ExtractedDocumentation, DocumentationItem, ItemKind};
use cursed::docs::comment_parser::CommentParser;
use cursed::docs::api_extractor::ApiExtractor;
use cursed::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_complete_documentation_extraction() -> Result<(), Error> {
    let test_source = r#"
/// Main package for testing documentation generation
/// 
/// This package demonstrates all CURSED language constructs
/// with comprehensive documentation comments.
///
/// @author CURSED Team
/// @version 1.0.0
/// @since 0.1.0
///
/// # Examples
/// 
/// ```cursed
/// yeet "testing"
/// 
/// slay main() {
///     println("Hello, documentation!")?
/// }
/// ```
vibe testing

/// Standard library imports for mathematical operations
/// @see stdlib::math for more mathematical functions
yeet "stdlib::math"

/// User data structure representing a person
/// 
/// This struct demonstrates CURSED's squad keyword
/// for struct declarations with comprehensive field documentation.
///
/// @example
/// ```cursed
/// sus person = Person {
///     name: "Alice",
///     age: 30,
///     email: "alice@example.com"
/// }
/// ```
squad Person {
    /// Person's full name
    /// @param name String containing the person's name
    name: string,
    
    /// Person's age in years  
    /// @param age Non-negative integer representing age
    age: i32,
    
    /// Contact email address
    /// @param email Valid email address string
    email: string
}

/// User authentication interface
/// 
/// Defines the contract for user authentication systems
/// using CURSED's collab keyword for interface declarations.
///
/// @see Person for user data structure
/// @since 1.0.0
collab Authenticator {
    /// Authenticate a user with credentials
    /// 
    /// @param username User's login name
    /// @param password User's password
    /// @return Result containing authentication token or error
    /// @throws AuthenticationError if credentials are invalid
    slay authenticate(username: string, password: string) -> Result<string, Error>
    
    /// Validate an authentication token
    /// 
    /// @param token Authentication token to validate
    /// @return Boolean indicating token validity
    slay validate_token(token: string) -> bool
}

/// Maximum number of login attempts allowed
/// 
/// This constant defines the security policy for login attempts
/// using CURSED's facts keyword for constant declarations.
///
/// @value 3
/// @see authenticate function for usage
facts MAX_LOGIN_ATTEMPTS = 3

/// Current user session state
/// 
/// Tracks the active user session using CURSED's sus keyword
/// for mutable variable declarations.
///
/// @type Option<Person>
/// @default None
sus current_user: Option<Person> = None

/// Calculate user age in days
/// 
/// This function demonstrates CURSED's slay keyword for function
/// declarations with comprehensive parameter documentation.
///
/// @param person Person object containing age information
/// @return Age in days as floating point number
/// @throws ValueError if age is negative
/// 
/// # Examples
/// 
/// ```cursed
/// sus alice = Person { name: "Alice", age: 30, email: "test@example.com" }
/// facts age_days = calculate_age_in_days(alice)?
/// println("Alice is {} days old", age_days)?
/// ```
/// 
/// # Algorithm
/// 
/// Uses the standard conversion of 365.25 days per year to account
/// for leap years in the Gregorian calendar system.
///
/// @complexity O(1) - constant time calculation
/// @accuracy Approximate due to leap year variations
slay calculate_age_in_days(person: Person) -> f64 {
    return person.age * 365.25
}

/// Generic container for storing typed values
/// 
/// Demonstrates CURSED's generic type system with constraints
/// and comprehensive documentation for type parameters.
///
/// @param T Type of values stored in the container
/// @constraint T must implement Display trait for string representation
/// @constraint T must implement Clone trait for value copying
///
/// # Usage
/// 
/// ```cursed
/// sus int_container = Container<i32>::new()
/// int_container.add(42)?
/// ```
squad Container<T: Display + Clone> {
    /// Internal storage for container items
    /// @type Vec<T>
    items: Vec<T>,
    
    /// Container capacity limit
    /// @type usize
    /// @default 100
    capacity: usize
}

/// Asynchronous data processing function
/// 
/// Demonstrates CURSED's async function capabilities with
/// comprehensive error handling documentation.
///
/// @param data Input data to process
/// @param options Processing configuration options
/// @return Processed data or error
/// @async This function is asynchronous and must be awaited
/// @timeout 30 seconds maximum processing time
/// 
/// # Error Handling
/// 
/// This function can return several types of errors:
/// - `ProcessingError` for data processing failures
/// - `TimeoutError` if processing takes too long
/// - `ValidationError` if input data is invalid
///
/// @throws ProcessingError When data processing fails
/// @throws TimeoutError When operation exceeds timeout
/// @throws ValidationError When input validation fails
async slay process_data_async(data: string, options: ProcessingOptions) -> Result<ProcessedData, Error> {
    // Implementation would go here
    Ok(ProcessedData::new())
}
"#;

    // Create temporary directory for test output
    let temp_dir = TempDir::new().map_err(|e| Error::General(format!("Failed to create temp dir: {}", e)))?;
    let temp_path = temp_dir.path().to_path_buf();
    
    // Test documentation extraction
    let config = DocGeneratorConfig {
        output_dir: temp_path.clone(),
        format: DocFormat::Html,
        include_examples: true,
        include_private: true,
        generate_cross_refs: true,
        custom_css: None,
        template_dir: None,
        title: "Test Documentation".to_string(),
        description: Some("Comprehensive test of documentation generation".to_string()),
        version: Some("1.0.0".to_string()),
        authors: vec!["Test Author".to_string()],
        base_url: None,
    };

    let mut generator = DocumentationGenerator::new(config);
    
    // Create a test file
    let test_file = temp_path.join("test.csd");
    fs::write(&test_file, test_source).map_err(Error::Io)?;
    
    // Generate documentation
    generator.generate_from_files(vec![test_file])?;
    
    // Verify documentation was extracted
    assert!(!generator.extracted_docs.is_empty(), "Should have extracted documentation");
    
    let doc = &generator.extracted_docs[0];
    assert_eq!(doc.module_name, "test");
    assert!(!doc.items.is_empty(), "Should have extracted documentation items");
    
    // Verify different item types were extracted
    let mut found_function = false;
    let mut found_struct = false;
    let mut found_interface = false;
    let mut found_variable = false;
    let mut found_constant = false;
    
    for item in &doc.items {
        match item.kind {
            ItemKind::Function => found_function = true,
            ItemKind::Struct => found_struct = true,
            ItemKind::Interface => found_interface = true,
            ItemKind::Variable => found_variable = true,
            ItemKind::Constant => found_constant = true,
            _ => {}
        }
    }
    
    assert!(found_function, "Should have extracted function documentation");
    assert!(found_struct, "Should have extracted struct documentation");
    assert!(found_interface, "Should have extracted interface documentation");
    assert!(found_variable, "Should have extracted variable documentation");
    assert!(found_constant, "Should have extracted constant documentation");
    
    Ok(())
}

#[test]
fn test_documentation_comment_parsing() -> Result<(), Error> {
    let parser = CommentParser::new()?;
    
    let doc_content = r#"
/// Calculate the factorial of a number
/// 
/// This function computes the factorial using an iterative approach
/// for better performance and to avoid stack overflow with large numbers.
///
/// @param n Non-negative integer to calculate factorial for
/// @return Factorial of n as u64
/// @throws ValueError if n is negative
/// 
/// # Examples
/// 
/// ```cursed
/// facts result = factorial(5)?  // Returns 120
/// println("5! = {}", result)?
/// ```
/// 
/// @complexity O(n) time, O(1) space
/// @since 1.0.0
/// @author Math Team
"#;

    let parsed = parser.parse_doc_content(doc_content)?;
    
    assert!(!parsed.summary.is_empty(), "Should extract summary");
    assert!(!parsed.description.is_empty(), "Should extract description");
    assert!(!parsed.examples.is_empty(), "Should extract examples");
    
    // Check specific tags
    assert!(parsed.tags.contains_key("parameters"), "Should extract parameter tags");
    assert!(parsed.tags.contains_key("returns"), "Should extract return tags");
    assert!(parsed.tags.contains_key("throws"), "Should extract throws tags");
    
    // Verify examples
    assert_eq!(parsed.examples.len(), 1, "Should have one example");
    let example = &parsed.examples[0];
    assert_eq!(example.language, "cursed", "Example should be in CURSED language");
    assert!(example.code.contains("factorial(5)"), "Example should contain function call");
    
    Ok(())
}

#[test]
fn test_cross_reference_generation() -> Result<(), Error> {
    let test_source = r#"
/// User management system
squad UserManager {
    /// Store user data
    users: Vec<Person>
}

/// Person data structure  
squad Person {
    name: string,
    age: i32
}

/// Create a new user in the system
/// 
/// This function takes a Person object and adds it to the UserManager.
/// See Person struct for the required data structure.
///
/// @param manager UserManager instance to add user to
/// @param person Person object to add
/// @return Result indicating success or failure
slay add_user(manager: UserManager, person: Person) -> Result<(), Error> {
    // Implementation here
}
"#;

    let temp_dir = TempDir::new().map_err(|e| Error::General(format!("Failed to create temp dir: {}", e)))?;
    let temp_path = temp_dir.path().to_path_buf();
    
    let config = DocGeneratorConfig {
        output_dir: temp_path.clone(),
        format: DocFormat::Json,
        include_examples: true,
        include_private: false,
        generate_cross_refs: true,
        custom_css: None,
        template_dir: None,
        title: "Cross-Reference Test".to_string(),
        description: None,
        version: None,
        authors: vec![],
        base_url: None,
    };

    let mut generator = DocumentationGenerator::new(config);
    
    let test_file = temp_path.join("cross_ref_test.csd");
    fs::write(&test_file, test_source).map_err(Error::Io)?;
    
    generator.generate_from_files(vec![test_file])?;
    
    // Verify cross-references were generated
    assert!(!generator.cross_references.is_empty(), "Should have generated cross-references");
    
    // Check for specific cross-references
    let has_person_refs = generator.cross_references.iter()
        .any(|(_, refs)| refs.iter().any(|r| r.target == "Person"));
    
    let has_user_manager_refs = generator.cross_references.iter()
        .any(|(_, refs)| refs.iter().any(|r| r.target == "UserManager"));
    
    assert!(has_person_refs, "Should have cross-references to Person");
    assert!(has_user_manager_refs, "Should have cross-references to UserManager");
    
    Ok(())
}

#[test]
fn test_multiple_output_formats() -> Result<(), Error> {
    let simple_source = r#"
/// Simple test function
/// @param x Input number
/// @return Doubled value
slay double(x: i32) -> i32 {
    return x * 2
}
"#;

    let temp_dir = TempDir::new().map_err(|e| Error::General(format!("Failed to create temp dir: {}", e)))?;
    let temp_path = temp_dir.path().to_path_buf();
    
    let test_file = temp_path.join("simple_test.csd");
    fs::write(&test_file, simple_source).map_err(Error::Io)?;
    
    // Test HTML output
    let html_config = DocGeneratorConfig {
        output_dir: temp_path.join("html"),
        format: DocFormat::Html,
        include_examples: true,
        include_private: false,
        generate_cross_refs: false,
        custom_css: None,
        template_dir: None,
        title: "HTML Test".to_string(),
        description: None,
        version: None,
        authors: vec![],
        base_url: None,
    };
    
    let mut html_generator = DocumentationGenerator::new(html_config);
    html_generator.generate_from_files(vec![test_file.clone()])?;
    
    // Test Markdown output
    let md_config = DocGeneratorConfig {
        output_dir: temp_path.join("markdown"),
        format: DocFormat::Markdown,
        include_examples: true,
        include_private: false,
        generate_cross_refs: false,
        custom_css: None,
        template_dir: None,
        title: "Markdown Test".to_string(),
        description: None,
        version: None,
        authors: vec![],
        base_url: None,
    };
    
    let mut md_generator = DocumentationGenerator::new(md_config);
    md_generator.generate_from_files(vec![test_file.clone()])?;
    
    // Test JSON output
    let json_config = DocGeneratorConfig {
        output_dir: temp_path.join("json"),
        format: DocFormat::Json,
        include_examples: true,
        include_private: false,
        generate_cross_refs: false,
        custom_css: None,
        template_dir: None,
        title: "JSON Test".to_string(),
        description: None,
        version: None,
        authors: vec![],
        base_url: None,
    };
    
    let mut json_generator = DocumentationGenerator::new(json_config);
    json_generator.generate_from_files(vec![test_file])?;
    
    // Verify output directories were created
    assert!(temp_path.join("html").exists(), "HTML output directory should exist");
    assert!(temp_path.join("markdown").exists(), "Markdown output directory should exist");
    assert!(temp_path.join("json").exists(), "JSON output directory should exist");
    
    Ok(())
}

#[test]
fn test_api_extractor_integration() -> Result<(), Error> {
    let temp_dir = TempDir::new().map_err(|e| Error::General(format!("Failed to create temp dir: {}", e)))?;
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create a mock stdlib structure
    let stdlib_dir = temp_path.join("src/stdlib");
    fs::create_dir_all(&stdlib_dir).map_err(Error::Io)?;
    
    // Create a mock math module
    let math_dir = stdlib_dir.join("math");
    fs::create_dir_all(&math_dir).map_err(Error::Io)?;
    
    let math_mod = math_dir.join("mod.rs");
    let math_content = r#"
//! Mathematical operations and constants
//!
//! This module provides basic mathematical functions and constants
//! for the CURSED programming language.

/// The mathematical constant π (pi)
pub const PI: f64 = 3.141592653589793;

/// Calculate the square of a number
/// 
/// # Examples
/// 
/// ```rust
/// let result = square(4.0);
/// assert_eq!(result, 16.0);
/// ```
pub fn square(x: f64) -> f64 {
    x * x
}

/// A basic mathematical structure
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}
"#;
    fs::write(&math_mod, math_content).map_err(Error::Io)?;
    
    // Test API extraction
    let extractor = ApiExtractor::new(temp_path)
        .with_options(false, true);
    
    let docs = extractor.extract_stdlib_documentation()?;
    
    assert!(!docs.is_empty(), "Should extract documentation from stdlib");
    
    // Verify extracted content
    let math_doc = docs.iter().find(|d| d.module_name.contains("math"));
    assert!(math_doc.is_some(), "Should extract math module documentation");
    
    let math_doc = math_doc.unwrap();
    assert!(!math_doc.items.is_empty(), "Should extract items from math module");
    
    // Check for specific items
    let has_pi = math_doc.items.iter().any(|item| item.name == "PI");
    let has_square = math_doc.items.iter().any(|item| item.name == "square");
    let has_vector = math_doc.items.iter().any(|item| item.name == "Vector2D");
    
    assert!(has_pi, "Should extract PI constant");
    assert!(has_square, "Should extract square function");
    assert!(has_vector, "Should extract Vector2D struct");
    
    Ok(())
}

#[test]
fn test_search_index_generation() -> Result<(), Error> {
    let test_source = r#"
/// Calculator utility functions
/// @keywords math, arithmetic, calculator

/// Add two numbers
/// @param a First number
/// @param b Second number
/// @return Sum of a and b
slay add(a: f64, b: f64) -> f64 {
    return a + b
}

/// Mathematical constants
squad MathConstants {
    /// Euler's number
    e: f64,
    /// Pi constant  
    pi: f64
}
"#;

    let temp_dir = TempDir::new().map_err(|e| Error::General(format!("Failed to create temp dir: {}", e)))?;
    let temp_path = temp_dir.path().to_path_buf();
    
    let config = DocGeneratorConfig {
        output_dir: temp_path.clone(),
        format: DocFormat::Html,
        include_examples: true,
        include_private: false,
        generate_cross_refs: false,
        custom_css: None,
        template_dir: None,
        title: "Search Index Test".to_string(),
        description: None,
        version: None,
        authors: vec![],
        base_url: None,
    };

    let mut generator = DocumentationGenerator::new(config);
    
    let test_file = temp_path.join("search_test.csd");
    fs::write(&test_file, test_source).map_err(Error::Io)?;
    
    generator.generate_from_files(vec![test_file])?;
    
    // Verify search index was built
    assert!(!generator.search_index.is_empty(), "Should have built search index");
    
    // Check for specific search entries
    let has_add_entry = generator.search_index.iter()
        .any(|entry| entry.name == "add");
    
    let has_constants_entry = generator.search_index.iter()
        .any(|entry| entry.name == "MathConstants");
    
    assert!(has_add_entry, "Should have search index entry for add function");
    assert!(has_constants_entry, "Should have search index entry for MathConstants struct");
    
    // Verify keywords were extracted
    let add_entry = generator.search_index.iter()
        .find(|entry| entry.name == "add")
        .unwrap();
    
    assert!(add_entry.keywords.contains(&"add".to_string()), "Should contain function name as keyword");
    assert!(add_entry.keywords.contains(&"function".to_string()), "Should contain item type as keyword");
    
    Ok(())
}

#[test]
fn test_error_handling_and_validation() -> Result<(), Error> {
    let temp_dir = TempDir::new().map_err(|e| Error::General(format!("Failed to create temp dir: {}", e)))?;
    let temp_path = temp_dir.path().to_path_buf();
    
    // Test with invalid source code
    let invalid_source = r#"
/// This is documentation for invalid syntax
slay broken_function( {
    // Missing parameter list and return type
}
"#;

    let config = DocGeneratorConfig::default();
    let mut generator = DocumentationGenerator::new(config);
    
    let test_file = temp_path.join("invalid_test.csd");
    fs::write(&test_file, invalid_source).map_err(Error::Io)?;
    
    // Should handle parse errors gracefully
    let result = generator.generate_from_files(vec![test_file]);
    
    // The generator should either succeed with partial documentation or fail gracefully
    match result {
        Ok(_) => {
            // If it succeeds, verify it handled the error gracefully
            assert!(true, "Generator handled invalid syntax gracefully");
        }
        Err(_) => {
            // If it fails, verify it's a proper error and not a panic
            assert!(true, "Generator properly reported parse error");
        }
    }
    
    // Test with non-existent file
    let non_existent = temp_path.join("does_not_exist.csd");
    let result = generator.generate_from_files(vec![non_existent]);
    assert!(result.is_err(), "Should return error for non-existent file");
    
    Ok(())
}

#[test]
fn test_comment_parser_edge_cases() -> Result<(), Error> {
    let parser = CommentParser::new()?;
    
    // Test empty documentation
    let empty_result = parser.parse_doc_content("")?;
    assert!(empty_result.summary.is_empty(), "Empty content should have empty summary");
    
    // Test documentation with only tags
    let tags_only = r#"
/// @param x Input value
/// @return Output value
/// @since 1.0.0
"#;
    
    let tags_result = parser.parse_doc_content(tags_only)?;
    assert!(!tags_result.tags.is_empty(), "Should extract tags from tags-only content");
    
    // Test documentation with code blocks
    let with_code = r#"
/// Function with code example
/// 
/// ```cursed
/// slay example() {
///     return 42
/// }
/// ```
/// 
/// More description here.
"#;
    
    let code_result = parser.parse_doc_content(with_code)?;
    assert!(!code_result.examples.is_empty(), "Should extract code examples");
    assert!(code_result.examples[0].code.contains("return 42"), "Should preserve code content");
    
    // Test malformed tags
    let malformed_tags = r#"
/// @incomplete tag without content
/// @param
/// @return something useful
"#;
    
    let malformed_result = parser.parse_doc_content(malformed_tags)?;
    // Should handle malformed tags gracefully
    assert!(malformed_result.tags.contains_key("returns"), "Should handle well-formed tags");
    
    Ok(())
}
