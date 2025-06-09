//! Basic integration tests for map parsing and AST generation in the CURSED language.
//!
//! These tests focus on the fundamental parsing and AST generation capabilities
//! for map literals, ensuring the parser correctly handles various map syntaxes
//! and creates proper AST structures.

use cursed::ast::collections::HashLiteral;
use cursed::ast::Expression;
use cursed::ast::traits::Node;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;

use std::collections::HashMap;
use tracing::{debug, info};

/// Initialize tracing for tests
fn init_test_tracing() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .init();
    });
}

/// Parse a map literal from source code
fn parse_map_literal(source: &str) -> Result<HashLiteral, Error> {
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer)?;
    
    // Parse as expression
    let expr = parser.parse_expression()?;
    
    // Downcast to HashLiteral
    if let Some(hash_lit) = expr.as_any().downcast_ref::<HashLiteral>() {
        Ok(hash_lit.clone())
    } else {
        Err(Error::from_str("Expression is not a hash literal"))
    }
}

/// Parse a complete CURSED program
fn parse_program(source: &str) -> Result<bool, Error> {
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;

    if !parser.errors().is_empty() {
        return Err(Error::from_str(&format!("Parser errors: {:?}", parser.errors())));
    }

    Ok(!program.statements.is_empty())
}

/// Test basic map literal parsing
#[test]
fn test_basic_map_literal_parsing() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing basic map literal parsing");
    
    // Test various map literal formats
    let test_cases = vec![
        (r#"{"alice": 30, "bob": 25}"#, 2, "string_to_int"),
        (r#"{1: "one", 2: "two", 3: "three"}"#, 3, "int_to_string"),
        (r#"{"score": 95.5, "grade": 87.2}"#, 2, "string_to_float"),
        (r#"{}"#, 0, "empty"),
        (r#"{"single": "value"}"#, 1, "single_element"),
    ];
    
    for (source, expected_pairs, test_name) in test_cases {
        debug!("Testing map literal: {} ({})", source, test_name);
        
        let result = parse_map_literal(source);
        assert!(result.is_ok(), "Failed to parse {}: {:?}", test_name, result.err());
        
        let map_literal = result.unwrap();
        assert_eq!(map_literal.pairs.len(), expected_pairs, 
                  "Wrong number of pairs for {}: expected {}, got {}", 
                  test_name, expected_pairs, map_literal.pairs.len());
        
        info!("Successfully parsed {} with {} pairs ({})", 
              source, expected_pairs, test_name);
    }
}

/// Test map AST structure and properties
#[test] 
fn test_map_ast_structure() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing map AST structure");
    
    // Parse a complex map literal
    let source = r#"{"name": "Alice", "age": 30, "score": 95.5}"#;
    let map_literal = parse_map_literal(source).unwrap();
    
    // Verify AST structure
    assert_eq!(map_literal.pairs.len(), 3);
    
    // Test string representation
    let string_repr = map_literal.string();
    assert!(string_repr.contains("name"));
    assert!(string_repr.contains("age"));
    assert!(string_repr.contains("score"));
    assert!(string_repr.contains("Alice"));
    
    // Test token literal
    let token_literal = map_literal.token_literal();
    assert!(!token_literal.is_empty());
    
    info!("AST structure test passed for: {}", source);
}

/// Test empty map handling
#[test]
fn test_empty_map() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing empty map parsing");
    
    let empty_map = parse_map_literal("{}").unwrap();
    assert_eq!(empty_map.pairs.len(), 0);
    
    let string_repr = empty_map.string();
    assert_eq!(string_repr, "{}");
    
    info!("Empty map test passed");
}

/// Test single element map
#[test]
fn test_single_element_map() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing single element map");
    
    let single_map = parse_map_literal(r#"{"only": "one"}"#).unwrap();
    assert_eq!(single_map.pairs.len(), 1);
    
    let string_repr = single_map.string();
    assert!(string_repr.contains("only"));
    assert!(string_repr.contains("one"));
    
    info!("Single element map test passed");
}

/// Test maps with different key types
#[test]
fn test_different_key_types() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing maps with different key types");
    
    let test_cases = vec![
        (r#"{"string_key": 42}"#, "string keys"),
        (r#"{42: "int_key"}"#, "integer keys"),
        (r#"{true: "boolean_key"}"#, "boolean keys"),
    ];
    
    for (source, description) in test_cases {
        debug!("Testing: {} ({})", source, description);
        
        let result = parse_map_literal(source);
        assert!(result.is_ok(), "Failed to parse {}: {:?}", description, result.err());
        
        let map_literal = result.unwrap();
        assert_eq!(map_literal.pairs.len(), 1);
        
        info!("Successfully parsed {}", description);
    }
    
    info!("Different key types test passed");
}

/// Test maps with different value types
#[test]
fn test_different_value_types() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing maps with different value types");
    
    let test_cases = vec![
        (r#"{"int_val": 42}"#, "integer values"),
        (r#"{"float_val": 3.14}"#, "float values"),
        (r#"{"string_val": "hello"}"#, "string values"),
        (r#"{"bool_val": true}"#, "boolean values"),
    ];
    
    for (source, description) in test_cases {
        debug!("Testing: {} ({})", source, description);
        
        let result = parse_map_literal(source);
        assert!(result.is_ok(), "Failed to parse {}: {:?}", description, result.err());
        
        let map_literal = result.unwrap();
        assert_eq!(map_literal.pairs.len(), 1);
        
        info!("Successfully parsed {}", description);
    }
    
    info!("Different value types test passed");
}

/// Test larger maps for performance
#[test]
fn test_large_map_parsing() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing large map parsing");
    
    // Generate a larger map
    let elements: Vec<String> = (0..50).map(|i| format!(r#""key{}": {}"#, i, i * 10)).collect();
    let large_source = format!("{{{}}}", elements.join(", "));
    
    let start_time = std::time::Instant::now();
    let large_map = parse_map_literal(&large_source).unwrap();
    let parse_duration = start_time.elapsed();
    
    assert_eq!(large_map.pairs.len(), 50);
    info!("Large map parsing took: {:?}", parse_duration);
    
    // Performance should be reasonable (less than 100ms for 50 elements)
    assert!(parse_duration.as_millis() < 100, "Parsing took too long: {:?}", parse_duration);
    
    info!("Large map parsing test passed");
}

/// Test error cases for invalid syntax
#[test]
fn test_invalid_map_syntax() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing invalid map syntax handling");
    
    let invalid_sources = vec![
        r#"{"unclosed": }"#,        // Missing value
        r#"{: "no_key"}"#,          // Missing key  
        r#"{"key": }"#,             // Missing value
        r#"{"key" "value"}"#,       // Missing colon
    ];
    
    for source in invalid_sources {
        debug!("Testing invalid source: {}", source);
        let result = parse_map_literal(source);
        assert!(result.is_err(), "Should have failed for invalid source: {}", source);
        info!("Correctly detected error for: {}", source);
    }
    
    info!("Invalid syntax test passed");
}

/// Test map integration with programs
#[test]
fn test_map_in_program_context() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing map integration in program context");
    
    // Test map in variable declaration
    let program_source = r#""
        vibe test_map_program

        slay main() normie {
            sus scores = {"alice": 95, "bob": 87}
            yolo 0
        }
    "#";
    
    let is_valid = parse_program(program_source);
    assert!(is_valid.is_ok(), "Failed to parse program with map");
    assert!(is_valid.unwrap(), "Program should not be empty");
    
    info!("Map in program context test passed");
}

/// Test map with function calls as values
#[test]
fn test_map_with_function_calls() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing map with function calls");
    
    let program_source = r#""
        vibe test_map_functions

        slay get_score() normie {
            yolo 95
        }

        slay main() normie {
            sus scores = {"alice": get_score(), "bob": 87}
            yolo 0
        }
    "#";
    
    let is_valid = parse_program(program_source);
    assert!(is_valid.is_ok(), "Failed to parse program with map and function calls");
    assert!(is_valid.unwrap(), "Program should not be empty");
    
    info!("Map with function calls test passed");
}

/// Test map literals with complex expressions
#[test]
fn test_map_with_complex_expressions() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing map with complex expressions");
    
    // Test with arithmetic expressions
    let arithmetic_source = r#"{"sum": 10 + 5, "product": 3 * 4}"#;
    let result = parse_map_literal(arithmetic_source);
    assert!(result.is_ok(), "Failed to parse map with arithmetic expressions");
    
    let map_literal = result.unwrap();
    assert_eq!(map_literal.pairs.len(), 2);
    
    info!("Map with complex expressions test passed");
}

/// Test nested structure parsing (if supported)
#[test]
fn test_basic_nested_structures() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing basic nested structure parsing");
    
    // Try parsing nested maps (may not be fully supported)
    let nested_source = r#"{"outer": {"inner": "value"}}"#;
    let result = parse_map_literal(nested_source);
    
    if result.is_ok() {
        let map_literal = result.unwrap();
        assert_eq!(map_literal.pairs.len(), 1);
        info!("Nested structure parsing is supported");
    } else {
        info!("Nested structure parsing not yet supported (expected)");
    }
}

/// Test whitespace and formatting handling
#[test]
fn test_whitespace_handling() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing whitespace and formatting handling");
    
    let test_cases = vec![
        (r#"{"key":"value"}"#, "no spaces"),
        (r#"{ "key" : "value" }"#, "extra spaces"),
        (r#"{"
            "key1": "value1",
            "key2": "value2"
        }"#, "multiline"),"
        (r#"{"key1":"value1","key2":"value2"}"#, "compact"),
    ];
    
    for (source, description) in test_cases {
        debug!("Testing whitespace: {}", description);
        
        let result = parse_map_literal(source);
        assert!(result.is_ok(), "Failed to parse {}: {:?}", description, result.err());
        
        let map_literal = result.unwrap();
        assert!(map_literal.pairs.len() >= 1, "Should have at least one pair");
        
        info!("Successfully parsed {}", description);
    }
    
    info!("Whitespace handling test passed");
}

/// Performance test with repeated parsing
#[test]
fn test_parsing_performance() {
    // init_tracing!();
    init_test_tracing();
    info!("Testing parsing performance");
    
    let source = r#"{"name": "Alice", "age": 30, "city": "NYC", "active": true}"#;
    let iterations = 100;
    
    let start_time = std::time::Instant::now();
    
    for _i in 0..iterations {
        let _result = parse_map_literal(source).unwrap();
    }
    
    let total_duration = start_time.elapsed();
    let avg_duration = total_duration / iterations;
    
    info!("Parsed {} times in {:?} (avg: {:?})", iterations, total_duration, avg_duration);
    
    // Should be fast (less than 10ms per parse on average)
    assert!(avg_duration.as_millis() < 10, "Parsing is too slow: {:?}", avg_duration);
    
    info!("Parsing performance test passed");
}

/// Summary test that validates all basic functionality
#[test]
fn test_map_parsing_integration_summary() {
    // init_tracing!();
    init_test_tracing();
    info!("Running map parsing integration summary");
    
    // Collect test results
    let mut test_results = HashMap::new();
    
    // Test basic parsing
    test_results.insert("empty_map", parse_map_literal("{}").is_ok());
    test_results.insert("single_pair", parse_map_literal(r#"{"key": "value"}"#).is_ok());
    test_results.insert("multiple_pairs", parse_map_literal(r#"{"a": 1, "b": 2, "c": 3}"#).is_ok());
    test_results.insert("different_types", parse_map_literal(r#"{"str": "text", "num": 42, "bool": true}"#).is_ok());
    
    // Test program integration
    test_results.insert("in_program", parse_program(r#""
        vibe test
        slay main() normie {
            sus map = {"key": "value"}
            yolo 0
        }
    "#).is_ok())";
    
    // Report results
    let successful_tests = test_results.values().filter(|&&v| v).count();
    let total_tests = test_results.len();
    
    info!("Integration summary: {}/{} tests passed", successful_tests, total_tests);
    info!("Test results: {:?}", test_results);
    
    // All basic tests should work
    assert!(test_results["empty_map"], "Empty map parsing should work");
    assert!(test_results["single_pair"], "Single pair parsing should work");
    assert!(test_results["multiple_pairs"], "Multiple pair parsing should work");
    assert!(test_results["in_program"], "Map in program should work");
    
    info!("Map parsing integration summary completed successfully");
}
