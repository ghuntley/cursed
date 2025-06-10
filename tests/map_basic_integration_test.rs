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

use std::any::Any;
use std::collections::HashMap;
use std::sync::Once;
use tracing::{debug, info};

/// Initialize tracing for tests
fn init_test_tracing() {
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
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer)?;
    
    let expression = parser.parse_expression()?;
    
    // Try to downcast to HashLiteral
    if let Some(hash_literal) = expression.as_any().downcast_ref::<HashLiteral>() {
        Ok(hash_literal.clone())
    } else {
        Err(Error::from_str("Expression is not a hash literal"))
    }
}

#[test]
fn test_basic_map_parsing() {
    init_test_tracing();
    info!("Starting basic map parsing test");
    
    let test_cases = vec![
        (r#"#{alice: 30, bob: 25}"#, 2, "simple map"),
        (r#"#{1: "one", 2: "two", 3: "three"}"#, 3, "numeric keys"),
        (r#"#{score: 95.5, grade: 87.2}"#, 2, "float values"),
        (r#"#{}"#, 0, "empty map"),
    ];
    
    for (source, expected_count, description) in test_cases {
        debug!("Testing: {}", description);
        let result = parse_map_literal(source);
        assert!(result.is_ok(), "Failed to parse {}: {}", description, source);
        
        let hash_literal = result.unwrap();
        assert_eq!(hash_literal.pairs.len(), expected_count, 
                  "Expected {} pairs for {}", expected_count, description);
        info!("✓ {} test passed", description);
    }
}

#[test]
fn test_invalid_map_syntax() {
    init_test_tracing();
    info!("Starting invalid map syntax test");
    
    let invalid_sources = vec![
        r#"#{unclosed: value"#,     // Missing closing brace
        r#"#{,key: value}"#,        // Missing key
        r#"#{key:}"#,              // Missing value
        r#"#{key value}"#,         // Missing colon
    ];
    
    for source in invalid_sources {
        debug!("Testing invalid syntax: {}", source);
        let result = parse_map_literal(source);
        assert!(result.is_err(), "Expected error for invalid syntax: {}", source);
        info!("✓ Invalid syntax correctly rejected: {}", source);
    }
}

#[test]
fn test_nested_map_parsing() {
    init_test_tracing();
    info!("Starting nested map parsing test");
    
    let nested_source = r#"#{outer: #{inner: "value"}, other: "test"}"#;
    let result = parse_map_literal(nested_source);
    assert!(result.is_ok(), "Failed to parse nested map structure");
    
    let hash_literal = result.unwrap();
    assert_eq!(hash_literal.pairs.len(), 2, "Expected 2 pairs in nested structure");
    info!("✓ Nested structure parsing test passed");
}

#[test]
fn test_map_with_whitespace() {
    init_test_tracing();
    info!("Starting whitespace handling test");
    
    let test_cases = vec![
        (r#"#{ key1: "value1" }"#, 1, "spaces around braces"),
        (r#"#{key1:"value1",key2:"value2"}"#, 2, "no spaces"),
        (r#"#{"
            key1: "value1",
            key2: "value2"
        }"#, 2, "multiline formatting"),"
    ];
    
    for (source, expected_count, description) in test_cases {
        debug!("Testing whitespace: {}", description);
        let result = parse_map_literal(source);
        assert!(result.is_ok(), "Failed to parse map with {}", description);
        
        let hash_literal = result.unwrap();
        assert_eq!(hash_literal.pairs.len(), expected_count,
                  "Expected {} pairs for {}", expected_count, description);
        info!("✓ {} test passed", description);
    }
}

#[test]
fn test_complex_map_parsing() {
    init_test_tracing();
    info!("Starting complex map parsing test");
    
    let complex_source = r#"#{name: "Alice", age: 30, city: "NYC", active: true}"#;
    let result = parse_map_literal(complex_source);
    assert!(result.is_ok(), "Failed to parse complex map");
    
    let hash_literal = result.unwrap();
    assert!(hash_literal.pairs.len() >= 4, "Expected at least 4 pairs in complex map");
    info!("✓ Complex map parsing test passed");
}
