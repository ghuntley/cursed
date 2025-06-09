//! Comprehensive tests for documentation comment parsing in CURSED language
//!
//! This test suite validates the documentation parsing capabilities including:
//! - Basic documentation comment parsing
//! - Multi-paragraph documentation
//! - Structured tags (@param, @return, @example)
//! - Code examples with syntax highlighting
//! - Error handling for malformed comments
//! - Integration with the main parser pipeline

use cursed::error::{Error, SourceLocation};
use cursed::lexer::Lexer;
use cursed::parser::{DocumentationComment, DocumentationParsing, DocumentationType, CodeExample, Parser};
use std::collections::HashMap;

#[path = "common/mod.rs"]
mod common;

/// Test basic documentation comment structure creation
#[test]
fn test_documentation_comment_creation() {
    
    let location = SourceLocation { 
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
        source_line: "".to_string(),
    };
    let doc = DocumentationComment::new(location);
    
    assert_eq!(doc.summary, "");
    assert_eq!(doc.description, "");
    assert!(doc.tags.is_empty());
    assert!(doc.examples.is_empty());
    assert_eq!(doc.location.line, 1);
    assert_eq!(doc.location.column, 1);
}

/// Test parsing simple single-line documentation
#[test]
#[traced_test]
fn test_parse_simple_documentation() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Simple documentation comment".to_string();
    
    doc.parse_content();
    
    assert_eq!(doc.summary, "Simple documentation comment");
    assert_eq!(doc.description, "");
}

/// Test parsing multi-line documentation with CURSED comment syntax
#[test]
#[traced_test]
fn test_parse_cursed_comment_syntax() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// fr fr This is a CURSED documentation comment\n/// fr fr with multiple lines".to_string();
    
    doc.parse_content();
    
    assert_eq!(doc.summary, "This is a CURSED documentation comment with multiple lines");
    assert_eq!(doc.description, "");
}

/// Test parsing documentation with summary and description
#[test]
#[traced_test]
fn test_parse_summary_and_description() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Brief summary of the function\n///\n/// This is a detailed description\n/// that spans multiple lines\n/// and provides comprehensive information".to_string();
    
    doc.parse_content();
    
    assert_eq!(doc.summary, "Brief summary of the function");
    assert_eq!(doc.description, "This is a detailed description\nthat spans multiple lines\nand provides comprehensive information");
}

/// Test parsing structured tags (@param, @return, etc.)
#[test]
#[traced_test]
fn test_parse_structured_tags() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Calculate the sum of two numbers\n/// @param x the first number\n/// @param y the second number\n/// @return the sum of x and y\n/// @example add(5, 3) returns 8".to_string();
    
    doc.parse_tags();
    
    let params = doc.tags.get("param").unwrap();
    assert_eq!(params.len(), 2);
    assert_eq!(params[0], "x the first number");
    assert_eq!(params[1], "y the second number");
    
    let returns = doc.tags.get("return").unwrap();
    assert_eq!(returns[0], "the sum of x and y");
    
    let examples = doc.tags.get("example").unwrap();
    assert_eq!(examples[0], "add(5, 3) returns 8");
}

/// Test parsing code examples with fenced code blocks
#[test]
#[traced_test]
fn test_parse_code_examples() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Example usage:\n/// ```cursed\n/// sus x = 42\n/// facts result = add(x, 8)\n/// ```\n/// Another example:\n/// ```text\n/// Output: 50\n/// ```".to_string();
    
    doc.parse_examples();
    
    assert_eq!(doc.examples.len(), 2);
    
    assert_eq!(doc.examples[0].language, "cursed");
    assert_eq!(doc.examples[0].code, "sus x = 42\nfacts result = add(x, 8)");
    
    assert_eq!(doc.examples[1].language, "text");
    assert_eq!(doc.examples[1].code, "Output: 50");
}

/// Test documentation validation functionality
#[test]
#[traced_test]
fn test_documentation_validation() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    
    // Valid documentation
    doc.tags.insert("param".to_string(), vec!["x the input value".to_string()]);
    doc.tags.insert("return".to_string(), vec!["the processed result".to_string()]);
    
    assert!(doc.validate().is_ok());
    
    // Invalid documentation with empty param
    doc.tags.insert("param".to_string(), vec!["".to_string()]);
    let result = doc.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains(&"Empty @param tag found".to_string()));
}

/// Test documentation helper methods
#[test]
#[traced_test]
fn test_documentation_helper_methods() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    
    doc.tags.insert("param".to_string(), vec!["x first value".to_string(), "y second value".to_string()]);
    doc.tags.insert("return".to_string(), vec!["computed result".to_string()]);
    
    let params = doc.get_parameters();
    assert_eq!(params.len(), 2);
    assert_eq!(params[0], "x first value");
    assert_eq!(params[1], "y second value");
    
    let return_doc = doc.get_return_documentation();
    assert_eq!(return_doc, Some("computed result"));
    
    let examples = doc.get_examples();
    assert_eq!(examples.len(), 0);
}

/// Test complete documentation parsing with all features
#[test]
#[traced_test]
fn test_comprehensive_documentation_parsing() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = r#"/// Calculate fibonacci number at given position
///
/// This function computes the nth fibonacci number using
/// an iterative approach for better performance.
///
/// @param n the position in fibonacci sequence
/// @return the fibonacci number at position n
/// @example fibonacci(5) returns 5
/// @example fibonacci(10) returns 55
///
/// ```cursed
/// slay fibonacci(n: int) -> int {
///     lowkey n <= 1 {
///         yolo n
///     }
///     sus a = 0
///     sus b = 1
///     bestie i in 2..=n {
///         facts temp = a + b
///         a = b
///         b = temp
///     }
///     yolo b
/// }
/// ```"#.to_string();
    
    doc.parse_content();
    doc.parse_tags();
    doc.parse_examples();
    
    // Verify summary and description
    assert_eq!(doc.summary, "Calculate fibonacci number at given position");
    assert!(doc.description.contains("This function computes the nth fibonacci number"));
    
    // Verify tags
    assert!(doc.tags.contains_key("param"));
    assert!(doc.tags.contains_key("return"));
    assert!(doc.tags.contains_key("example"));
    
    let examples_tag = doc.tags.get("example").unwrap();
    assert_eq!(examples_tag.len(), 2);
    
    // Verify code examples
    assert_eq!(doc.examples.len(), 1);
    assert_eq!(doc.examples[0].language, "cursed");
    assert!(doc.examples[0].code.contains("slay fibonacci"));
}

/// Test error handling for malformed documentation
#[test]
#[traced_test]
fn test_malformed_documentation_handling() {
    common::tracing::setup();
    
    let input = "/// Incomplete code example\n/// ```cursed\n/// sus x = 42\n// Missing closing backticks";
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    // Should handle unterminated code blocks gracefully
    let doc_result = parser.parse_documentation();
    assert!(doc_result.is_ok());
    
    if let Ok(Some(doc)) = doc_result {
        // Should still create an example even with unterminated block
        assert_eq!(doc.examples.len(), 1);
        assert_eq!(doc.examples[0].language, "cursed");
    }
}

/// Test integration with parser pipeline
#[test]
#[traced_test]
fn test_parser_integration() {
    common::tracing::setup();
    
    let input = r#"/// fr fr Add two numbers together
/// fr fr @param x first number
/// fr fr @param y second number  
/// fr fr @return sum of the numbers
slay add(x: int, y: int) -> int {
    yolo x + y
}"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    // Test documentation detection
    assert!(parser.is_documentation_comment());
    
    // Test documentation parsing
    let doc_result = parser.parse_documentation();
    assert!(doc_result.is_ok());
    
    if let Ok(Some(doc)) = doc_result {
        assert_eq!(doc.summary, "Add two numbers together");
        assert!(doc.tags.contains_key("param"));
        assert!(doc.tags.contains_key("return"));
    }
}

/// Test function documentation with complex parameters
#[test]
#[traced_test]
fn test_function_documentation_complex_params() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = r#"/// Process HTTP request with middleware
/// @param request the incoming HTTP request object containing headers and body
/// @param middleware array of middleware functions to apply
/// @param context request context with user authentication and session data
/// @return processed response with applied middleware transformations
/// @throws HTTPError when request processing fails
/// @since v1.2.0"#.to_string();
    
    doc.parse_tags();
    
    let params = doc.tags.get("param").unwrap();
    assert_eq!(params.len(), 3);
    assert!(params[0].contains("HTTP request object"));
    assert!(params[1].contains("middleware functions"));
    assert!(params[2].contains("authentication and session"));
    
    assert!(doc.tags.contains_key("return"));
    assert!(doc.tags.contains_key("throws"));
    assert!(doc.tags.contains_key("since"));
}

/// Test package-level documentation
#[test]
#[traced_test]
fn test_package_documentation() {
    common::tracing::setup();
    
    let input = r#"/// fr fr HTTP server implementation for CURSED language
/// fr fr
/// fr fr This package provides a fast and secure HTTP server
/// fr fr with support for middleware, routing, and WebSocket connections.
vibe http_server"#;
    
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer).unwrap();
    
    let doc_result = parser.parse_documentation();
    assert!(doc_result.is_ok());
    
    if let Ok(Some(doc)) = doc_result {
        assert_eq!(doc.summary, "HTTP server implementation for CURSED language");
        assert!(doc.description.contains("fast and secure HTTP server"));
    }
}

/// Test struct/type documentation
#[test]
#[traced_test]
fn test_struct_documentation() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = r#"/// User profile information
/// 
/// Contains all the necessary information about a user
/// including authentication credentials and preferences.
///
/// @field username unique identifier for the user
/// @field email contact email address
/// @field created_at timestamp when account was created"#.to_string();
    
    doc.parse_content();
    doc.parse_tags();
    
    assert_eq!(doc.summary, "User profile information");
    assert!(doc.description.contains("authentication credentials"));
    
    let fields = doc.tags.get("field").unwrap();
    assert_eq!(fields.len(), 3);
    assert!(fields[0].contains("unique identifier"));
}

/// Test nested code examples with different languages
#[test]
#[traced_test]
fn test_multiple_language_examples() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = r#"/// Multi-language example
/// ```cursed
/// sus server = HttpServer::new()
/// server.listen(8080)
/// ```
/// ```json
/// {
///   "port": 8080,
///   "host": "localhost"
/// }
/// ```
/// ```bash
/// curl http://localhost:8080/api/users
/// ```"#.to_string();
    
    doc.parse_examples();
    
    assert_eq!(doc.examples.len(), 3);
    assert_eq!(doc.examples[0].language, "cursed");
    assert_eq!(doc.examples[1].language, "json");
    assert_eq!(doc.examples[2].language, "bash");
    
    assert!(doc.examples[0].code.contains("HttpServer"));
    assert!(doc.examples[1].code.contains("port"));
    assert!(doc.examples[2].code.contains("curl"));
}

/// Test edge cases and error scenarios
#[test]
#[traced_test]
fn test_edge_cases() {
    common::tracing::setup();
    
    // Empty documentation
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "".to_string();
    doc.parse_content();
    assert_eq!(doc.summary, "");
    assert_eq!(doc.description, "");
    
    // Only comment markers
    doc.raw_content = "///\n///\n///".to_string();
    doc.parse_content();
    assert_eq!(doc.summary, "");
    
    // Malformed tags
    doc.raw_content = "/// @param\n/// @return".to_string();
    doc.parse_tags();
    assert!(doc.tags.contains_key("param"));
    assert!(doc.tags.contains_key("return"));
    
    // Mixed comment styles
    doc.raw_content = "/// Standard comment\n/// fr fr CURSED comment".to_string();
    doc.parse_content();
    assert!(doc.summary.contains("Standard comment"));
    assert!(doc.summary.contains("CURSED comment"));
}

/// Test performance with large documentation blocks
#[test]
#[traced_test]
fn test_large_documentation_performance() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    
    // Create a large documentation block
    let mut large_content = String::new();
    for i in 0..1000 {
        large_content.push_str(&format!("/// Line {} of documentation\n", i));
    }
    large_content.push_str("/// @param large_param a very detailed parameter description\n");
    for i in 0..100 {
        large_content.push_str(&format!("/// @example example_{} some example code\n", i));
    }
    
    doc.raw_content = large_content;
    
    use std::time::Instant;
    let start = Instant::now();
    
    doc.parse_content();
    doc.parse_tags();
    doc.parse_examples();
    
    let duration = start.elapsed();
    
    // Should process large documentation in reasonable time (< 100ms)
    assert!(duration.as_millis() < 100);
    
    // Verify parsing worked correctly
    assert!(!doc.summary.is_empty());
    assert!(doc.tags.contains_key("param"));
    assert!(doc.tags.contains_key("example"));
    assert_eq!(doc.tags.get("example").unwrap().len(), 100);
}

#[test] 
#[traced_test]
fn test_documentation_with_unicode() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1 };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Calculate π approximation using 🌟 method\n/// @param ε precision tolerance (ε > 0)\n/// @return approximated π value".to_string();
    
    doc.parse_content();
    doc.parse_tags();
    
    assert!(doc.summary.contains("π"));
    assert!(doc.summary.contains("🌟"));
    
    let params = doc.tags.get("param").unwrap();
    assert!(params[0].contains("ε"));
    
    let returns = doc.tags.get("return").unwrap();
    assert!(returns[0].contains("π"));
}
