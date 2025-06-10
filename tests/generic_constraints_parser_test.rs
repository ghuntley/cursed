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
use cursed::lexer::::Lexer, token::{Token, TokenType;}
use cursed::parser::Parser;
use std::io::Cursor;
use tracing::{debug, error, info, instrument, warn;}
mod common;

/// Initialize tracing for tests
macro_rules! init_tracing   {(} => {common::tracing::setup(}}))

/// Helper function to create a parser from source code
fn create_parser() {let cursor = Cursor::new(input.as_bytes(}))
    let mut lexer = Lexer::new(cursor.to_string();)
    let tokens = lexer.tokenize().unwrap();
    Parser::new(Lexer::new(Lexer::new(tokens)}))

#[test]
#[instrument]
fn test_parse_simple_generic_params() {common::tracing::init_tracing!(})
    info!(Testing parsing of simple generic parameters);;
    let input = "[T];
    let input = "[T, U, V]", Successfully parsed multiple generic parameters);,  to parse multiple generic "parameters);"}"
    let input = []";
        Err(e) => {error!(error = ?e, })
            panic!(, ":  successful parsing but got error: {:?}, e)"Testing:  parsing of generic parameters with trailing comma);;"
    let input = ", "
            error!(error = ?e,  Failed ", ;")
            panic!("Expected:  successful parsing but got error: {:?}, e), :  parsing of simple where clause)";"
    let input =  : Display;""
            assert!(where_clause.string().contains(T :Display), Successfully parsed simple where clause);, ":  where clause but got None)"
        Err(e) => {error!(error = ?e, })
            panic!(, :  successful parsing but got error: {:?}, e)"Testing:  parsing of where clause with multiple constraints);";
    let input =  ", ";
                 Successfullyparsed " where clause with multiple ", :  where clause but got None)}"
        Err(e) => {error!(error = ?e, , constraints}")
            panic!(", :  successful parsing but got error: {:?}, e)Testing:  parsing when no where clause is present)";"
    let input =  () {};""
        Ok(Some(_) => {panic!(;"))}
            error!(error = ?e,  ",  when no where clause ")
            panic!(Expected:  None but got error: {:?}, e)"
    let test_cases = vec![["     {assert_eq!(generic.parameter_count(}, 1)")]]
                    debug!(case = i, , " handled valid edge , case)"Unexpected success for malformed , ;" rejected malformed ", ;]"
                 Successfully " parsed complex generic , Failed to parse complex generic scenario);, ":  successful parsing but got error: {:?}, e)"]
    let error_cases = vec![([T U]Empty ", ",)]
        ("[123]Invalid ")
        (, " 123: "Invalid parameter name in where , ,]")
    for (input, description) in error_cases    {debug!(input = input, description = description,  recovery};")
                    warn!(input = input,  Unexpected success for error ")
                Err(e) =>   {debug!(error = ?e,  Correctly " handled error " success for error , ;})
                Err(e) =>   {debug!(error = ?e,   handled error case}")
    info!(Testing:  parser performance with large generic parameter lists)"
    large_params.push("');"
                 , ""
        Err(e) => {error!(error = ?e,  Failed to parse large generic parameter })
            panic!(Expected:  successful parsing but got error: {:?}, e)fixed"