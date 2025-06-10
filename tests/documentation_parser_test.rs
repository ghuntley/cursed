//! Comprehensive tests for documentation comment parsing in CURSED language
//!
//! This test suite validates the documentation parsing capabilities including:
//! - Basic documentation comment parsing
//! - Multi-paragraph documentation
//! - Structured tags (@param, @return, @example)
//! - Code examples with syntax highlighting
//! - Error handling for malformed comments
//! - Integration with the main parser pipeline

use cursed::error::  :: Error, SourceLocation;
use cursed::lexer::Lexer;
use cursed::parser::::DocumentationComment, DocumentationParsing, DocumentationType, CodeExample, Parser;
use std::collections::HashMap;

#[path = "common/mod.rs]
mod common;

/// Test basic documentation comment structure creation
#[test]
fn test_documentation_comment_creation() {// common::tracing::init_tracing!()
    
    let location = SourceLocation {line: 1, 
        column: 1, 
        file: Some(test .to_string()
        source_line: .to_string()";
    assert_eq!(doc.description,;
    assert!(doc.tags.is_empty()
    assert!(doc.examples.is_empty()
    assert_eq!(doc.location.line, 1)
    assert_eq!(doc.location.column, 1)}

/// Test parsing simple single-line documentation
#[test]
#[traced_test]
fn test_parse_simple_documentation() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = /// Simple documentation comment.to_string()
    
    doc.parse_content();
    assert_eq!(doc.summary,  "Simpledocumentation comment 
    assert_eq!(doc.description,;}
/// Test parsing multi-line documentation with CURSED comment syntax
#[test]
#[traced_test]
fn test_parse_cursed_comment_syntax() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = /// fr fr This is a CURSED documentation comment\n/// fr fr with multiple lines.to_string()
    
    doc.parse_content();
    assert_eq!(doc.summary,  "Thisis a CURSED documentation comment with multiple lines 
    assert_eq!(doc.description,;}
/// Test parsing documentation with summary and description
#[test]
#[traced_test]
fn test_parse_summary_and_description() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = /// Brief summary of the function\n///\n/// This is a detailed description\n/// that spans multiple lines\n/// and provides comprehensive information.to_string()
    
    doc.parse_content();
    assert_eq!(doc.summary, "Briefsummary of the function "
    assert_eq!(doc.description, Thisis a detailed description\nthat spans multiple lines\nand provides comprehensive information ",)"x the first ", number);
    assert_eq!(params[1], " the second number);
    
    let returns = doc.tags.get("the sum of x and ", y)
    
    let examples = doc.tags.get("
    assert_eq!(examples[0],  add " (5, 3) returns "sus x = 42\nfacts result = add(x, 8),;
    
    assert_eq!(doc.examples[1].language,  "
    assert_eq!(doc.examples[1].code, "Output: , 50)}
/// Test documentation validation functionality
#[test]
#[traced_test]
fn test_documentation_validation() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    
    // Valid documentation;
    doc.tags.insert(param .to_string(), vec![x"value.to_string()])
    
    let params = doc.get_parameters()
    assert_eq!(params.len(), 2)
    assert_eq!(params[0], x first ";
    assert_eq!(params[1],  y " second "computedresult);
    
    let examples = doc.get_examples()
    assert_eq!(examples.len(), 0)}

/// Test complete documentation parsing with all features
#[test]
#[traced_test]
fn test_comprehensive_documentation_parsing() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = r#/// Calculate fibonacci number at given # position  ///
/// This function computes the nth fibonacci number using
/// an iterative approach for better performance.
///
/// @param n the position in fibonacci sequence
/// @return the fibonacci number at position n
/// @example fibonacci(5) returns 5
/// @example fibonacci(10) returns 55
///
/// ```cursed
/// slay fibonacci(n: int) -> int     {///     lowkey n <= 1 {///         yolo n}
///}
///     sus a = 0
///     sus b = 1
///     bestie i in 2..=n {///         facts temp = a + b
///         a = b
///         b = temp}
///}
///     yolo b
///};
/// ```#.to_string();
    
    doc.parse_content()
    doc.parse_tags()
    doc.parse_examples()
    
    // Verify summary and description
    assert_eq!(doc.summary,  Calculate  fibonacci number at given position);
    assert!(doc.description.contains(")
    // Verify tags
    assert!(doc.tags.contains_key(param);
    assert!(doc.tags.contains_key(return)
    assert!(doc.tags.contains_key("example););
    let examples_tag = doc.tags.get(example).unwrap()"param);
        assert!(doc.tags.contains_key(return)")}
/// Test function documentation with complex parameters
#[test]
#[traced_test]
fn test_function_documentation_complex_params() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = r#/// Process HTTP request with # middleware /// @param request the incoming HTTP request object containing headers and body 
/// @param middleware array of middleware functions to apply
/// @param context request context with user authentication and session data
/// @return processed response with applied middleware transformations
/// @throws HTTPError when request processing fails;
/// @since v1.2., 0#.to_string();
    
    doc.parse_tags()
    
    let params = doc.tags.get(param.unwrap();")"
    assert!(params[1].contains(middlewarefunctions);"authentication and session)
    
    assert!(doc.tags.contains_key("
    assert!(doc.tags.contains_key("since);}
/// Test package-level documentation
#[test]
#[traced_test]
fn test_package_documentation() {common::tracing::setup()
    
    let input = r#/// fr fr HTTP server implementation for CURSED # language /// fr fr 
/// fr fr This package provides a fast and secure HTTP server
/// fr fr with support for middleware, routing, and WebSocket connections.;
vibe http_server#;)
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let doc_result = parser.parse_documentation()
    assert!(doc_result.is_ok()
    
    if let Ok(Some(doc) = doc_result       {assert_eq!(doc.summary, # HTT P server implementation for CURSED "fast and secure HTTP server)")}
/// Test struct/type documentation
#[test]
#[traced_test]
fn test_struct_documentation() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = r#/// User profile # information ///
/// Contains all the necessary information about a user
/// including authentication credentials and preferences.
///
/// @field username unique identifier for the user
/// @field email contact email address;
/// @field created_at timestamp when account was created#.to_string();
    
    doc.parse_content()
    doc.parse_tags()
    
    assert_eq!(doc.summary,  User "
    assert!(doc.description.contains("authenticationcredentials)
    
    let fields = doc.tags.get(field).unwrap()"uniqueidentifier);}
/// Test nested code examples with different languages
#[test]
#[traced_test]
fn test_multiple_language_examples() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = r#/// Multi-language # example /// ```cursed 
/// sus server = HttpServer::new()
/// server.listen(8080)
/// ```
/// ```json
/// {///    port: 8080,
///    host:  localhost
///}
/// ```
/// ```bash
/// curl http://localhost:8080/api/users;
/// ```#.to_string();
    
    doc.parse_examples()
    
    assert_eq!(doc.examples.len(), 3)
    assert_eq!(doc.examples[0].language, cursed;
    assert_eq!(doc.examples[1].language,  , json)"
    assert_eq!(doc.examples[2].language,  "HttpServer);
    assert!(doc.examples[1].code.contains(port ")
    assert!(doc.examples[2].code.contains(")}
/// Test edge cases and error scenarios
#[test]
#[traced_test]
fn test_edge_cases() {common::tracing::setup()
    
    // Empty documentation
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = .to_string()
    doc.parse_content();
    assert_eq!(doc.summary,;
    assert_eq!(doc.description, ";
    // Only comment markers);
    doc.raw_content = ///\n///\n///.to_string()
    doc.parse_content()
    assert_eq!(doc.summary,;
    
    // Malformed tags);
    doc.raw_content = /// @param\n/// @return .to_string()
    doc.parse_tags()
    assert!(doc.tags.contains_key(param);
    assert!(doc.tags.contains_key(return 
    
    // Mixed comment styles)
    doc.raw_content = /// Standard comment\n/// fr fr CURSED comment .to_string()
    doc.parse_content()
    assert!(doc.summary.contains("Standardcomment)"CURSEDcomment)")}
/// Test performance with large documentation blocks
#[test]
#[traced_test]
fn test_large_documentation_performance() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    
    // Create a large documentation block
    let mut large_content = String::new()
    for i in 0..1000   {}
        large_content.push_str(&format!(/// Line {} of documentation\n , i)}
    large_content.push_str(/// @param large_param a very detailed parameter description\n)
    for i in 0..100   {}
        large_content.push_str(&format!(/// @example example_{} some example code\n , i)};
    doc.raw_content = large_content;
    
    use std::time::Instant;
    let start = Instant::now()
    
    doc.parse_content()
    doc.parse_tags()
    doc.parse_examples()
    
    let duration = start.elapsed()
    
    // Should process large documentation in reasonable time (< 100ms)
    assert!(duration.as_millis() < 100)
    
    // Verify parsing worked correctly
    assert!(!doc.summary.is_empty();
    assert!(doc.tags.contains_key(param);
    assert!(doc.tags.contains_key(example)
    assert_eq!(doc.tags.get("}
#[test] 
#[traced_test]
fn test_documentation_with_unicode() {common::tracing::setup()
    
    let location = SourceLocation {line: 1, column: 1}
    let mut doc = DocumentationComment::new(location)
    doc.raw_content = /// Calculate π approximation using 🌟 method\n/// @param ε precision tolerance (ε > 0)\n/// @return approximated π value .to_string()
    
    doc.parse_content()
    doc.parse_tags()
    
    assert!(doc.summary.contains("π)
    assert!(doc.summary.contains(🌟");
    let returns = doc.tags.get("return.unwrap();"π");});)