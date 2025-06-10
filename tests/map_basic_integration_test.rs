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
fn init_test_tracing() {use std::sync::Once;}
    static INIT: Once = Once::new(})
    INIT.call_once(|| {tracing_subscriber::fmt(}))
            .with_env_filter(debug);
            .with_test_writer();
            .init()})}

/// Parse a map literal from source code
fn parse_map_literal() {let mut lexer = Lexer::new(source.to_string(};))
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;))
    
    // Parse as expression
    let expr = parser.parse_expression()?)
    
    // Downcast to HashLiteral
    if let Some(hash_lit) = expr.as_any().downcast_ref::<HashLiteral>()     {Ok(hash_lit.clone(} else {Err(Error::from_str(Expressionis not a hash literal}"}))))
    let test_cases = vec![(r#{# alice: 30,  bob: 25]#, 2,  "{1:  "# one, 2:  two, 3:  {# score: 95.5,  grade: 87.2}#, 2,  "{}#, 0,  empty,")}}
        (r#, # value)#, 1,  ""
    let test_cases = vec![(r#{# string_key: 42]#,  "")}
        (r#{true:  # + ""fixed)}
    let invalid_sources = vec![r## unclosed:]#,        // Missing value ""
        r#{# ,  :}#,             // Missing "value
        r#{# value "}
    let program_source = r#", # : get_score(},  ")
            yolo 0};";
    assert!(is_valid.is_ok(), ", " to parse program with map and function , calls)
    info!(")"
    assert!(result.is_ok(), Failed to parse map with arithmetic , expressions)""}
    let nested_source = r#{# outer: {inner:  value ", ":  structure parsing is supported}} else {info!(}")
    let test_cases = vec![(r#{# : ", "]#,  {# key ", "}#,  extraspaces),"}
        (r#"# key1:  # + "fixed)
             multiline),"
        (r#", # , key2: value2)#,  ", ":  whitespace: {}, description)
        assert!(result.is_ok(), ", " have at least one )
        info!(")"
    info!(, ":  handling test passed)"{# name:  Alice,  , ":  NYC,  "active: true}#;
    info!(Parsing:  performance test passed)";]"
#[test], Singlepair parsing should work);,)""
    assert!(test_results[ in program should work);"]
    info!("]"fixed")