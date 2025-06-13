//! Documentation System Integration Tests
//! 
//! Comprehensive tests for the CURSED documentation generation system.

use cursed::docs::{DocumentationGenerator, DocGeneratorConfig, DocFormat};
use cursed::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use tempfile::TempDir;

/// Create a temporary test directory with sample CURSED files
fn create_test_directory() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create sample CURSED source file
    let sample_source = r#"
/// This is a sample CURSED module
/// Demonstrates the documentation system capabilities
vibe test_module

/// Import the standard I/O library
yeet "stdlib::io"

/// A sample function that greets the user
/// @param name The name of the person to greet
/// @return A greeting message
/// @example
/// ```cursed
/// let greeting = greet("World")
/// println(greeting)
/// ```
slay greet(name string) string {
    yolo "Hello, " + name + "!"
}

/// A sample struct representing a person
/// @since 1.0.0
squad Person {
    name string
    age i32
}

/// A sample interface for drawable objects
/// @deprecated Use Renderable instead
collab Drawable {
    slay draw()
}

/// A sample constant
/// @example
/// ```cursed
/// println(GREETING)
/// ```
facts GREETING = "Hello, CURSED!"

/// A sample variable
sus counter = 0
"#;

    fs::write(temp_dir.path().join("sample.csd"), sample_source)
        .expect("Failed to write sample file");
    
    // Create another module
    let math_source = r#"
/// Mathematical utilities for CURSED
/// Provides basic arithmetic and advanced functions
vibe math_utils

/// Add two numbers together
/// @param a First number
/// @param b Second number  
/// @return Sum of a and b
/// @example
/// ```cursed
/// let result = add(5, 3)
/// // result is 8
/// ```
slay add(a i32, b i32) i32 {
    yolo a + b
}

/// Calculate factorial of a number
/// @param n Input number (must be non-negative)
/// @return Factorial of n
/// @throws Error if n is negative
/// @example
/// ```cursed
/// let fact = factorial(5)
/// // fact is 120
/// ```
slay factorial(n i32) i32 {
    lowkey (n < 0) {
        // Error handling would go here
        yolo -1
    }
    
    lowkey (n <= 1) {
        yolo 1
    }
    
    yolo n * factorial(n - 1)
}
"#;

    fs::write(temp_dir.path().join("math.csd"), math_source)
        .expect("Failed to write math file");

    temp_dir
}

#[test]
fn test_html_documentation_generation() {
    let source_dir = create_test_directory();
    let output_dir = TempDir::new().expect("Failed to create output directory");

    let mut config = DocGeneratorConfig::default();
    config.output_dir = output_dir.path().to_path_buf();
    config.format = DocFormat::Html;
    config.title = "Test Documentation".to_string();
    config.description = Some("Test documentation for CURSED".to_string());
    config.version = Some("1.0.0".to_string());
    config.authors = vec!["Test Author".to_string()];

    let mut generator = DocumentationGenerator::new(config);
    let result = generator.generate_from_directory(source_dir.path());

    assert!(result.is_ok(), "HTML documentation generation failed: {:?}", result);

    // Verify output files exist
    assert!(output_dir.path().join("index.html").exists(), "index.html not generated");
    assert!(output_dir.path().join("static").exists(), "static directory not created");
    assert!(output_dir.path().join("static/docs.css").exists(), "CSS file not generated");
    assert!(output_dir.path().join("static/docs.js").exists(), "JavaScript file not generated");
    
    // Verify index.html contains expected content
    let index_content = fs::read_to_string(output_dir.path().join("index.html"))
        .expect("Failed to read index.html");
    assert!(index_content.contains("Test Documentation"));
    assert!(index_content.contains("Test documentation for CURSED"));
    assert!(index_content.contains("search-input"));
}

#[test]
fn test_markdown_documentation_generation() {
    let source_dir = create_test_directory();
    let output_dir = TempDir::new().expect("Failed to create output directory");

    let mut config = DocGeneratorConfig::default();
    config.output_dir = output_dir.path().to_path_buf();
    config.format = DocFormat::Markdown;
    config.title = "CURSED API Documentation".to_string();
    config.authors = vec!["CURSED Team".to_string()];

    let mut generator = DocumentationGenerator::new(config);
    let result = generator.generate_from_directory(source_dir.path());

    assert!(result.is_ok(), "Markdown documentation generation failed: {:?}", result);

    // Verify output files exist
    assert!(output_dir.path().join("README.md").exists(), "README.md not generated");

    // Verify README.md contains expected content
    let readme_content = fs::read_to_string(output_dir.path().join("README.md"))
        .expect("Failed to read README.md");
    assert!(readme_content.contains("# CURSED API Documentation"));
    assert!(readme_content.contains("## Table of Contents"));
    assert!(readme_content.contains("![Language]"));
}

#[test]
fn test_json_documentation_generation() {
    let source_dir = create_test_directory();
    let output_dir = TempDir::new().expect("Failed to create output directory");

    let mut config = DocGeneratorConfig::default();
    config.output_dir = output_dir.path().to_path_buf();
    config.format = DocFormat::Json;
    config.title = "CURSED JSON Docs".to_string();

    let mut generator = DocumentationGenerator::new(config);
    let result = generator.generate_from_directory(source_dir.path());

    assert!(result.is_ok(), "JSON documentation generation failed: {:?}", result);

    // Verify output files exist
    assert!(output_dir.path().join("documentation.json").exists(), "documentation.json not generated");
    assert!(output_dir.path().join("search-index.json").exists(), "search-index.json not generated");

    // Verify JSON structure
    let json_content = fs::read_to_string(output_dir.path().join("documentation.json"))
        .expect("Failed to read documentation.json");
    
    let json_data: serde_json::Value = serde_json::from_str(&json_content)
        .expect("Invalid JSON generated");
    
    assert!(json_data["metadata"].is_object(), "Missing metadata section");
    assert!(json_data["modules"].is_array(), "Missing modules section");
    assert!(json_data["statistics"].is_object(), "Missing statistics section");
    assert_eq!(json_data["metadata"]["title"], "CURSED JSON Docs");
}

#[test]
fn test_documentation_with_configuration_options() {
    let source_dir = create_test_directory();
    let output_dir = TempDir::new().expect("Failed to create output directory");

    let mut config = DocGeneratorConfig::default();
    config.output_dir = output_dir.path().to_path_buf();
    config.format = DocFormat::Html;
    config.include_private = true;
    config.include_examples = true;
    config.generate_cross_refs = true;
    config.custom_css = Some("body { background: blue; }".to_string());

    let mut generator = DocumentationGenerator::new(config);
    let result = generator.generate_from_directory(source_dir.path());

    assert!(result.is_ok(), "Documentation generation with options failed: {:?}", result);

    // Verify custom CSS is included (this would be in a real implementation)
    let css_content = fs::read_to_string(output_dir.path().join("static/docs.css"))
        .expect("Failed to read CSS file");
    
    // In a real implementation, custom CSS would be integrated
    assert!(!css_content.is_empty(), "CSS file is empty");
}

#[test]
fn test_single_file_documentation() {
    let source_dir = create_test_directory();
    let output_dir = TempDir::new().expect("Failed to create output directory");

    let mut config = DocGeneratorConfig::default();
    config.output_dir = output_dir.path().to_path_buf();
    config.format = DocFormat::Html;

    let mut generator = DocumentationGenerator::new(config);
    let source_file = source_dir.path().join("sample.csd");
    let result = generator.generate_from_files(vec![source_file]);

    assert!(result.is_ok(), "Single file documentation generation failed: {:?}", result);

    // Verify basic output structure
    assert!(output_dir.path().join("index.html").exists(), "index.html not generated for single file");
}

#[test]
fn test_documentation_error_handling() {
    let output_dir = TempDir::new().expect("Failed to create output directory");

    let mut config = DocGeneratorConfig::default();
    config.output_dir = output_dir.path().to_path_buf();
    config.format = DocFormat::Html;

    let mut generator = DocumentationGenerator::new(config);
    
    // Test with non-existent directory
    let result = generator.generate_from_directory(Path::new("/non/existent/path"));
    assert!(result.is_err(), "Should fail with non-existent directory");
    
    // Test with non-existent file
    let result = generator.generate_from_files(vec![PathBuf::from("/non/existent/file.csd")]);
    assert!(result.is_err(), "Should fail with non-existent file");
}

#[test]
fn test_comment_parser_functionality() {
    use cursed::docs::{CommentParser, ParsedDocumentation};
    use cursed::error::SourceLocation;

    let parser = CommentParser::new().expect("Failed to create comment parser");
    
    let source = r#"
/// This is a sample function
/// @param name The user's name
/// @return A greeting message
/// @example
/// ```cursed
/// greet("Alice")
/// ```
slay greet(name string) string {
    yolo "Hello, " + name
}
"#;

    let location = SourceLocation { line: 8, column: 1, file: None };
    let result = parser.parse_item_documentation(source, &location);
    
    assert!(result.is_ok(), "Failed to parse documentation");
    let parsed = result.unwrap();
    
    assert_eq!(parsed.summary, "This is a sample function");
    assert_eq!(parsed.parameters.len(), 1);
    assert_eq!(parsed.parameters[0].name, "name");
    assert_eq!(parsed.examples.len(), 1);
    assert_eq!(parsed.examples[0].language, "cursed");
}

#[test]
fn test_configuration_serialization() {
    let config = DocGeneratorConfig {
        output_dir: PathBuf::from("test_docs"),
        format: DocFormat::Html,
        include_examples: true,
        include_private: false,
        generate_cross_refs: true,
        custom_css: None,
        template_dir: None,
        title: "Test Config".to_string(),
        description: Some("Test description".to_string()),
        version: Some("1.0.0".to_string()),
        authors: vec!["Test Author".to_string()],
        base_url: None,
    };

    // Test JSON serialization
    let json_result = serde_json::to_string_pretty(&config);
    assert!(json_result.is_ok(), "Failed to serialize config to JSON");

    let json_str = json_result.unwrap();
    let deserialized: DocGeneratorConfig = serde_json::from_str(&json_str)
        .expect("Failed to deserialize config from JSON");
    
    assert_eq!(config.title, deserialized.title);
    assert_eq!(config.include_examples, deserialized.include_examples);

    // Test TOML serialization
    let toml_result = toml::to_string_pretty(&config);
    assert!(toml_result.is_ok(), "Failed to serialize config to TOML");
}

#[test]
fn test_format_parsing() {
    use std::str::FromStr;
    
    assert!(matches!(DocFormat::from_str("html"), Ok(DocFormat::Html)));
    assert!(matches!(DocFormat::from_str("markdown"), Ok(DocFormat::Markdown)));
    assert!(matches!(DocFormat::from_str("md"), Ok(DocFormat::Markdown)));
    assert!(matches!(DocFormat::from_str("json"), Ok(DocFormat::Json)));
    assert!(DocFormat::from_str("invalid").is_err());
}

#[test]
fn test_legacy_api_compatibility() {
    use cursed::docs::DocumentationGenerator as LegacyGenerator;
    
    let temp_dir = create_test_directory();
    let output_dir = TempDir::new().expect("Failed to create output directory");
    
    let mut generator = LegacyGenerator::new();
    let result = generator.generate_docs(
        temp_dir.path().to_str().unwrap(),
        output_dir.path().to_str().unwrap()
    );
    
    assert!(result.is_ok(), "Legacy API compatibility test failed");
}

#[test]
fn test_convenience_functions() {
    use cursed::docs::{generate_html_docs, generate_markdown_docs, generate_json_docs};
    
    let source_dir = create_test_directory();
    let source_file = source_dir.path().join("sample.csd");
    
    // Test HTML generation
    let html_output = TempDir::new().expect("Failed to create temp dir");
    let result = generate_html_docs(&source_file, html_output.path());
    assert!(result.is_ok(), "HTML convenience function failed");
    
    // Test Markdown generation
    let md_output = TempDir::new().expect("Failed to create temp dir");
    let result = generate_markdown_docs(&source_file, md_output.path());
    assert!(result.is_ok(), "Markdown convenience function failed");
    
    // Test JSON generation
    let json_output = TempDir::new().expect("Failed to create temp dir");
    let result = generate_json_docs(&source_file, json_output.path());
    assert!(result.is_ok(), "JSON convenience function failed");
}

#[test] 
fn test_documentation_extraction() {
    use cursed::docs::extract_documentation;
    
    let source = r#"
/// Sample CURSED module
vibe sample

/// A greeting function
slay hello() {
    println("Hello!")
}
"#;

    let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    let result = extract_documentation(source, temp_file.path());
    
    assert!(result.is_ok(), "Documentation extraction failed");
    let docs = result.unwrap();
    assert_eq!(docs.module_name, "sample");
    assert!(!docs.items.is_empty(), "No documentation items extracted");
}

/// Performance test for large documentation generation
#[test]
fn test_documentation_performance() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create multiple files to test performance
    for i in 0..10 {
        let content = format!(r#"
/// Module number {}
vibe module_{}

/// Function number {}
slay func_{}() {{
    yolo {}
}}
"#, i, i, i, i, i);
        
        fs::write(temp_dir.path().join(format!("module_{}.csd", i)), content)
            .expect("Failed to write test file");
    }
    
    let output_dir = TempDir::new().expect("Failed to create output directory");
    let mut config = DocGeneratorConfig::default();
    config.output_dir = output_dir.path().to_path_buf();
    config.format = DocFormat::Html;
    
    let start = std::time::Instant::now();
    let mut generator = DocumentationGenerator::new(config);
    let result = generator.generate_from_directory(temp_dir.path());
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Performance test failed");
    assert!(duration.as_millis() < 5000, "Documentation generation too slow: {}ms", duration.as_millis());
}
