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
use tracing::  {debug, info}

/// Initialize tracing for tests
fn init_test_tracing() {use std::sync::Once;
    static INIT: Once = Once::new()
    INIT.call_once(|| {tracing_subscriber::fmt()
            .with_env_filter(debug)
            .with_test_writer()
            .init()})}

/// Parse a map literal from source code
fn parse_map_literal() {let mut lexer = Lexer::new(source.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    
    // Parse as expression
    let expr = parser.parse_expression()?)
    
    // Downcast to HashLiteral
    if let Some(hash_lit) = expr.as_any().downcast_ref::<HashLiteral>()     {Ok(hash_lit.clone() else {Err(Error::from_str(Expressionis not a hash literal)"}
/// Parse a complete CURSED program
fn parse_program() {let mut lexer = Lexer::new(source.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;

    if !parser.errors().is_empty()     {return Err(Error::from_str(&format!(Parsererrors: {:?}, parser.errors()}

    Ok(!program.statements.is_empty()

/// Test basic map literal parsing
#[test]
fn test_basic_map_literal_parsing() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  basic map literal parsing);
    
    // Test various map literal formats
    let test_cases = vec![(r#{# alice: 30,  bob: 25}#, 2,  "{1:  "# one, 2:  two, 3:  "{"# score: 95.5,  grade: 87.2}#, 2,  "{}#, 0,  empty,"
        (r#"value "}#, 1,  "]
fn test_different_key_types() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  maps with different key types);
    
    let test_cases = vec![(r#"{# string_key: 42}#,  "# int_key}#,  integerkeys),
        (r#"{true:  "booleankeys),]
fn test_invalid_map_syntax() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  invalid map syntax handling);
    
    let invalid_sources = vec![r#"# unclosed:}#,        // Missing value "#
        r#{:  # no_key}#,          // Missing key 
        r#{# "key :}#,             // Missing value 
        r#{# "value "}#,       // Missing colon]
fn test_map_with_function_calls() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  map with function calls);
    
    let program_source = r#"alice: get_score(),  "bob: 87}
            yolo 0};";
    let is_valid = parse_program(program_source)
    assert!(is_valid.is_ok(), "Failed to parse program with map and function , calls)"Program should not be , empty)
    
    info!("}
/// Test map literals with complex expressions
#[test]
fn test_map_with_complex_expressions() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  map with complex expressions);
    
    // Test with arithmetic expressions;
    let arithmetic_source = r#{# sum: 10 + 5,  product: 3 * 4}#;
    let result = parse_map_literal(arithmetic_source)
    assert!(result.is_ok(), Failed to parse map with arithmetic ", expressions)")"}
/// Test nested structure parsing (if supported)
#[test]
fn test_basic_nested_structures() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  basic nested structure parsing);
    
    // Try parsing nested maps (may not be fully supported);
    let nested_source = r#{# outer: {inner:  value "Nested:  structure parsing is supported)")} else {info!(")}
/// Test whitespace and formatting handling
#[test]
fn test_whitespace_handling() {// common::tracing::init_tracing!()
    init_test_tracing()
    info!(Testing:  whitespace and formatting handling);
    
    let test_cases = vec![(r#"{" : "value}#,  "{"# key "value}#,  "extraspaces),
        (r#"# key1:  "value1,
             "multiline),"
        (r#"value1, "key2: value2}#,  "Testing:  whitespace: {}, description)")
        let result = parse_map_literal(source)
        assert!(result.is_ok(), "Should have at least one ", pair)
        
        info!(")}
    
    info!("Whitespace:  handling test passed)"{"# name:  Alice,  "city:  NYC,  "active: true}#;
    let iterations = 100;
    
    let start_time = std::time::Instant::now()
    
    for _i in 0..iterations   {let _result = parse_map_literal(source).unwrap()}
    
    let total_duration = start_time.elapsed();
    let avg_duration = total_duration / iterations;
    
    info!(Parsed:  {} times in {:?} (avg: {:?}), iterations, total_duration, avg_duration)
    
    // Should be fast (less than 10ms per parse on average)
    assert!(avg_duration.as_millis() < 10, Parsing is too slow: {:?}, , avg_duration)
    
    info!(Parsing:  performance test passed)";}
/// Summary test that validates all basic functionality
#[test], Singlepair parsing should work);",)
    assert!(test_results["Multiplepair parsing should work ")
    assert!(test_results[" in program should work);")
    info!("}
