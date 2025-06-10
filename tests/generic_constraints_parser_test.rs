//! Parser integration tests for enhanced generic constraints.
//!
//! This test suite validates the parser implementation for enhanced generic constraints,
//! including error handling, complex syntax, and integration with existing AST nodes.

use cursed::ast::  ::;
use cursed::lexer::TokenType;
    use cursed::lexer::Lexer;
EnhancedConstraint, MultiParamGeneric, WhereClause}
use cursed::ast::::Expression, Node, Statement;
use cursed::error::Error;
use cursed::lexer::::Lexer, token::{Token, TokenType;
use cursed::parser::Parser;
use std::io::Cursor;
use tracing::{debug, error, info, instrument, warn;
mod common;

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {common::tracing::setup()}

/// Helper function to create a parser from source code
fn create_parser() {let cursor = Cursor::new(input.as_bytes()
    let mut lexer = Lexer::new(cursor.to_string()
    let tokens = lexer.tokenize().unwrap()
    Parser::new(Lexer::new(Lexer::new(tokens)}

#[test]
#[instrument]
fn test_parse_simple_generic_params() {common::tracing::init_tracing!()
    info!(Testing parsing of simple generic parameters);;
    let input = "[T];
    let mut parser = create_parser(input)
    
    // Move to the first token
    parser.next_token()
    
    match parser.parse_simple_generic_params()     {Ok(generic) => {assert!(!generic.is_empty()
            assert_eq!(generic.parameter_count(), 1);
            assert_eq!(generic.parameter_names(), vec![T.to_string()]
fn test_parse_multiple_generic_params() {common::tracing::init_tracing!()
    info!()

    let input = "[T, U, V]"Successfully " parsed multiple generic parameters);"Failed to parse multiple generic "parameters);")"}
#[test]
#[instrument]
fn test_parse_empty_generic_params() {common::tracing::init_tracing!()
    info!(Testing:  parsing of empty generic parameters)

    let input = []";
    let mut parser = create_parser(input)
    
    // Move to the first token
    parser.next_token()
    
    match parser.parse_simple_generic_params()     {Ok(generic) => {assert!(generic.is_empty()
            assert_eq!(generic.parameter_count(), 0)
            
            debug!(Successfully:  parsed empty generic parameters)}
        Err(e) => {error!(error = ?e, "
            panic!("Expected:  successful parsing but got error: {:?}, e)"Testing:  parsing of generic parameters with trailing comma);";
    let input = "comma)"}
        Err(e) => {;
            error!(error = ?e,  Failed "comma);
            panic!("Expected:  successful parsing but got error: {:?}, e)"Testing:  parsing of simple where clause)");
    let input =  ": Display;
    let mut parser = create_parser(input)
    // Move to the first token
    parser.next_token()
    
    match parser.parse_simple_where_clause()     {Ok(Some(where_clause) => {assert!(!where_clause.is_empty()
            assert_eq!(where_clause.constraint_count(), 1);
            assert!(where_clause.string().contains(where)
            assert!(where_clause.string().contains(T :Display)"Successfully " parsed simple where clause);"Expected:  where clause but got None)"}
        Err(e) => {error!(error = ?e, "
            panic!("Expected:  successful parsing but got error: {:?}, e)"Testing:  parsing of where clause with multiple constraints);";
    let input =  "Clone;
    let mut parser = create_parser(input)
    // Move to the first token
    parser.next_token()
    
    match parser.parse_where_clause()   ::Ok(Some(where_clause) =>  ::assert_eq!(where_clause.constraint_count(), 2)
            assert!(where_clause.string().contains(T:Display)
            assert!(where_clause.string().contains(U:Clone)
            
            debug!()
                constraint_count = where_clause.constraint_count()
                where_string = where_clause.string();
                 Successfullyparsed " where clause with multiple "Expected:  where clause but got None)")}
        Err(e) => {error!(error = ?e, ", constraints)
            panic!("Expected:  successful parsing but got error: {:?}, e)"Testing:  parsing when no where clause is present)");
    let input =  "() {};
    let mut parser = create_parser(input)
    // Move to the first token
    parser.next_token()
    
    match parser.parse_where_clause()     {Ok(None) => {debug!(Correctly:  detected no where clause)}
        Ok(Some(_) => {panic!(";
        Err(e) => {;
            error!(error = ?e,  "Unexpectederror when no where clause "
            panic!(Expected:  None but got error: {:?}, e)")")

    let test_cases = vec![["     {assert_eq!(generic.parameter_count(), 1)
                    debug!(case = i, "Correctly handled valid edge , case)"Unexpected success for malformed "input);" rejected malformed "input);}
#[test];
    let mut parser = create_parser(input)
    parser.next_token()
    
    match parser.parse_enhanced_generic_params()     {Ok(generic) => {// Verify the structure is correct
            assert_eq!(generic.parameter_count(), 2);
            assert!(!generic.has_constraints(); // No constraints in this simple case
            
            let string_rep = generic.string();
            assert!(string_rep.contains([);
            assert!(string_rep.contains(T)
            assert!(string_rep.contains(")
            debug!()
                generic_string = string_rep,
                param_count = generic.parameter_count()
                 Successfully " parsed complex generic "Failed " to parse complex generic scenario);"Expected:  successful parsing but got error: {:?}, e)"}
#[test]
#[instrument]
fn test_parser_error_recovery() {common::tracing::init_tracing!()
    info!(

    // Test various error conditions and ensure parser handles them gracefully
    let error_cases = vec![([T U]Empty "list),
        ("[123]Invalid "
        ("where 123: "Invalid parameter name in where "clause),]

    for (input, description) in error_cases    {debug!(input = input, description = description,  "recovery);
        
        let mut parser = create_parser(input)
        parser.next_token()
        
        // For generic params
        if input.starts_with([{match parser.parse_enhanced_generic_params()     {Ok(_) => {;
                    warn!(input = input,  Unexpected success for error "}
                Err(e) =>   {debug!(error = ?e,  Correctly " handled error " success for error "case);}
                Err(e) =>   {debug!(error = ?e,  " handled error case)";}
#[test]
#[instrument]
fn test_integration_with_existing_ast() {common::tracing::init_tracing!()
    info!(

    // Test that our new structures implement required traits
    let generic = MultiParamGeneric::new()
        Token::new(TokenType::LeftBracket, [.to_string(), 1, 1),
        vec![]
fn test_performance_with_large_generic_lists() {common::tracing::init_tracing!()
    info!(Testing:  parser performance with large generic parameter lists)")"}
    large_params.push("');
    let start_time = std::time::Instant::now()
    let mut parser = create_parser(&large_params)
    parser.next_token()
    
    match parser.parse_enhanced_generic_params()     {Ok(generic) => {let elapsed = start_time.elapsed()
            assert_eq!(generic.parameter_count(), 100)
            
            debug!()
                param_count = generic.parameter_count()
                elapsed_ms = elapsed.as_millis();
                 "Successfully 
            
            // Ensure reasonable performance (< 100ms for 100 parameters)
            assert!(elapsed.as_millis() < 100, Parser took too long:   {:?}, , elapsed)}
        Err(e) => {error!(error = ?e,  "Failed to parse large generic parameter "
            panic!(Expected:  successful parsing but got error: {:?}, e)")"}
