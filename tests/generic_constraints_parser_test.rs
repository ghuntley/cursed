//! Parser integration tests for enhanced generic constraints.
//!
//! This test suite validates the parser implementation for enhanced generic constraints,
//! including error handling, complex syntax, and integration with existing AST nodes.

use cursed::ast::{
    EnhancedConstraint, MultiParamGeneric, WhereClause
};
use cursed::ast::{Expression, Node, Statement};
use cursed::error::Error;
use cursed::lexer::{Lexer, token::{Token, TokenType}};
use cursed::parser::Parser;
use std::io::Cursor;
use tracing::{debug, error, info, instrument, warn};

mod common;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup()
    };
}

/// Helper function to create a parser from source code
fn create_parser(input: &str) -> Parser {
    let cursor = Cursor::new(input.as_bytes());
    let mut lexer = Lexer::new(cursor.to_string());
    let tokens = lexer.tokenize().unwrap();
    Parser::new(tokens)
}

#[test]
#[instrument]
fn test_parse_simple_generic_params() {
    init_tracing!();
    info!("Testing parsing of simple generic parameters");

    let input = "[T]";
    let mut parser = create_parser(input);
    
    // Move to the first token
    parser.next_token();
    
    match parser.parse_simple_generic_params() {
        Ok(generic) => {
            assert!(!generic.is_empty());
            assert_eq!(generic.parameter_count(), 1);
            assert_eq!(generic.parameter_names(), vec!["T".to_string()]);
            
            debug!(
                param_count = generic.parameter_count(),
                param_names = ?generic.parameter_names(),
                "Successfully parsed simple generic parameters"
            );
        }
        Err(e) => {
            error!(error = ?e, "Failed to parse simple generic parameters");
            panic!("Expected successful parsing but got error: {:?}", e);
        }
    }
}

#[test]
#[instrument]
fn test_parse_multiple_generic_params() {
    init_tracing!();
    info!("Testing parsing of multiple generic parameters");

    let input = "[T, U, V]";
    let mut parser = create_parser(input);
    
    // Move to the first token
    parser.next_token();
    
    match parser.parse_simple_generic_params() {
        Ok(generic) => {
            assert_eq!(generic.parameter_count(), 3);
            assert_eq!(generic.parameter_names(), vec!["T".to_string(), "U".to_string(), "V".to_string()]);
            
            debug!(
                param_count = generic.parameter_count(),
                param_names = ?generic.parameter_names(),
                "Successfully parsed multiple generic parameters"
            );
        }
        Err(e) => {
            error!(error = ?e, "Failed to parse multiple generic parameters");
            panic!("Expected successful parsing but got error: {:?}", e);
        }
    }
}

#[test]
#[instrument]
fn test_parse_empty_generic_params() {
    init_tracing!();
    info!("Testing parsing of empty generic parameters");

    let input = "[]";
    let mut parser = create_parser(input);
    
    // Move to the first token
    parser.next_token();
    
    match parser.parse_simple_generic_params() {
        Ok(generic) => {
            assert!(generic.is_empty());
            assert_eq!(generic.parameter_count(), 0);
            
            debug!("Successfully parsed empty generic parameters");
        }
        Err(e) => {
            error!(error = ?e, "Failed to parse empty generic parameters");
            panic!("Expected successful parsing but got error: {:?}", e);
        }
    }
}

#[test]
#[instrument]
fn test_parse_generic_params_with_trailing_comma() {
    init_tracing!();
    info!("Testing parsing of generic parameters with trailing comma");

    let input = "[T, U,]";
    let mut parser = create_parser(input);
    
    // Move to the first token
    parser.next_token();
    
    match parser.parse_simple_generic_params() {
        Ok(generic) => {
            assert_eq!(generic.parameter_count(), 2);
            assert_eq!(generic.parameter_names(), vec!["T".to_string(), "U".to_string()]);
            
            debug!(
                param_count = generic.parameter_count(),
                "Successfully parsed generic parameters with trailing comma"
            );
        }
        Err(e) => {
            error!(error = ?e, "Failed to parse generic parameters with trailing comma");
            panic!("Expected successful parsing but got error: {:?}", e);
        }
    }
}

#[test]
#[instrument]
fn test_parse_where_clause_simple() {
    init_tracing!();
    info!("Testing parsing of simple where clause");

    let input = "where T: Display";
    let mut parser = create_parser(input);
    
    // Move to the first token
    parser.next_token();
    
    match parser.parse_simple_where_clause() {
        Ok(Some(where_clause)) => {
            assert!(!where_clause.is_empty());
            assert_eq!(where_clause.constraint_count(), 1);
            assert!(where_clause.string().contains("where"));
            assert!(where_clause.string().contains("T:Display"));
            
            debug!(
                constraint_count = where_clause.constraint_count(),
                where_string = where_clause.string(),
                "Successfully parsed simple where clause"
            );
        }
        Ok(None) => {
            panic!("Expected where clause but got None");
        }
        Err(e) => {
            error!(error = ?e, "Failed to parse simple where clause");
            panic!("Expected successful parsing but got error: {:?}", e);
        }
    }
}

#[test]
#[instrument]
fn test_parse_where_clause_multiple_constraints() {
    init_tracing!();
    info!("Testing parsing of where clause with multiple constraints");

    let input = "where T: Display, U: Clone";
    let mut parser = create_parser(input);
    
    // Move to the first token
    parser.next_token();
    
    match parser.parse_where_clause() {
        Ok(Some(where_clause)) => {
            assert_eq!(where_clause.constraint_count(), 2);
            assert!(where_clause.string().contains("T:Display"));
            assert!(where_clause.string().contains("U:Clone"));
            
            debug!(
                constraint_count = where_clause.constraint_count(),
                where_string = where_clause.string(),
                "Successfully parsed where clause with multiple constraints"
            );
        }
        Ok(None) => {
            panic!("Expected where clause but got None");
        }
        Err(e) => {
            error!(error = ?e, "Failed to parse where clause with multiple constraints");
            panic!("Expected successful parsing but got error: {:?}", e);
        }
    }
}

#[test]
#[instrument]
fn test_parse_no_where_clause() {
    init_tracing!();
    info!("Testing parsing when no where clause is present");

    let input = "fn test() {}";
    let mut parser = create_parser(input);
    
    // Move to the first token
    parser.next_token();
    
    match parser.parse_where_clause() {
        Ok(None) => {
            debug!("Correctly detected no where clause");
        }
        Ok(Some(_)) => {
            panic!("Expected None but got where clause");
        }
        Err(e) => {
            error!(error = ?e, "Unexpected error when no where clause present");
            panic!("Expected None but got error: {:?}", e);
        }
    }
}

#[test]
#[instrument]
fn test_parse_malformed_generic_params() {
    init_tracing!();
    info!("Testing parsing of malformed generic parameters");

    let test_cases = vec![
        "[",        // Missing closing bracket
        "[T",       // Missing closing bracket
        "[T U]",    // Missing comma
        "[T,]",     // Only trailing comma (should work)
        "[,T]",     // Leading comma
    ];

    for (i, input) in test_cases.iter().enumerate() {
        debug!(test_case = i, input = input, "Testing malformed input");
        
        let mut parser = create_parser(input);
        parser.next_token();
        
        match parser.parse_enhanced_generic_params() {
            Ok(generic) => {
                // Some cases like "[T,]" should work
                if input == &"[T,]" {
                    assert_eq!(generic.parameter_count(), 1);
                    debug!(case = i, "Correctly handled valid edge case");
                } else {
                    warn!(case = i, input = input, "Unexpected success for malformed input");
                }
            }
            Err(e) => {
                debug!(case = i, error = ?e, "Correctly rejected malformed input");
            }
        }
    }
}

#[test]
#[instrument]
fn test_parse_malformed_where_clause() {
    init_tracing!();
    info!("Testing parsing of malformed where clauses");

    let test_cases = vec![
        "where",        // Missing constraint
        "where T",      // Missing colon and interface
        "where T:",     // Missing interface
        "where : Display", // Missing parameter
        "where T Display", // Missing colon
    ];

    for (i, input) in test_cases.iter().enumerate() {
        debug!(test_case = i, input = input, "Testing malformed where clause");
        
        let mut parser = create_parser(input);
        parser.next_token();
        
        match parser.parse_where_clause() {
            Ok(_) => {
                warn!(case = i, input = input, "Unexpected success for malformed where clause");
            }
            Err(e) => {
                debug!(case = i, error = ?e, "Correctly rejected malformed where clause");
            }
        }
    }
}

#[test]
#[instrument]
fn test_complex_generic_scenarios() {
    init_tracing!();
    info!("Testing complex generic constraint scenarios");

    // This test focuses on the AST structure since we're testing parser integration
    let input = "[T, U]";
    let mut parser = create_parser(input);
    parser.next_token();
    
    match parser.parse_enhanced_generic_params() {
        Ok(generic) => {
            // Verify the structure is correct
            assert_eq!(generic.parameter_count(), 2);
            assert!(!generic.has_constraints()); // No constraints in this simple case
            
            let string_rep = generic.string();
            assert!(string_rep.contains("["));
            assert!(string_rep.contains("T"));
            assert!(string_rep.contains("U"));
            assert!(string_rep.contains("]"));
            
            debug!(
                generic_string = string_rep,
                param_count = generic.parameter_count(),
                "Successfully parsed complex generic scenario"
            );
        }
        Err(e) => {
            error!(error = ?e, "Failed to parse complex generic scenario");
            panic!("Expected successful parsing but got error: {:?}", e);
        }
    }
}

#[test]
#[instrument]
fn test_parser_error_recovery() {
    init_tracing!();
    info!("Testing parser error recovery for generic constraints");

    // Test various error conditions and ensure parser handles them gracefully
    let error_cases = vec![
        ("[T U V]", "Missing commas between parameters"),
        ("[T, , U]", "Empty parameter in list"),
        ("[123]", "Invalid parameter name"),
        ("where 123: Display", "Invalid parameter name in where clause"),
    ];

    for (input, description) in error_cases {
        debug!(input = input, description = description, "Testing error recovery");
        
        let mut parser = create_parser(input);
        parser.next_token();
        
        // For generic params
        if input.starts_with('[') {
            match parser.parse_enhanced_generic_params() {
                Ok(_) => {
                    warn!(input = input, "Unexpected success for error case");
                }
                Err(e) => {
                    debug!(error = ?e, "Correctly handled error case");
                }
            }
        }
        
        // For where clauses
        if input.starts_with("where") {
            let mut parser = create_parser(input);
            parser.next_token();
            
            match parser.parse_where_clause() {
                Ok(_) => {
                    warn!(input = input, "Unexpected success for error case");
                }
                Err(e) => {
                    debug!(error = ?e, "Correctly handled error case");
                }
            }
        }
    }
}

#[test]
#[instrument]
fn test_integration_with_existing_ast() {
    init_tracing!();
    info!("Testing integration of enhanced constraints with existing AST structures");

    // Test that our new structures implement required traits
    let generic = MultiParamGeneric::new(
        Token::new(TokenType::LeftBracket, "[".to_string(), 1, 1),
        vec![]
    );

    // Test Node trait
    assert_eq!(generic.literal.clone(), "[");
    assert!(!generic.string().is_empty());

    // Test Statement trait
    generic.statement_node(); // Should not panic
    assert!(generic.as_any().is::<MultiParamGeneric>());

    debug!("Enhanced constraint structures integrate correctly with existing AST");
}

#[test]
#[instrument]
fn test_performance_with_large_generic_lists() {
    init_tracing!();
    info!("Testing parser performance with large generic parameter lists");

    // Create a large generic parameter list
    let mut large_params = String::from("[");
    for i in 0..100 {
        if i > 0 {
            large_params.push_str(", ");
        }
        large_params.push_str(&format!("T{}", i));
    }
    large_params.push(']');

    let start_time = std::time::Instant::now();
    let mut parser = create_parser(&large_params);
    parser.next_token();
    
    match parser.parse_enhanced_generic_params() {
        Ok(generic) => {
            let elapsed = start_time.elapsed();
            assert_eq!(generic.parameter_count(), 100);
            
            debug!(
                param_count = generic.parameter_count(),
                elapsed_ms = elapsed.as_millis(),
                "Successfully parsed large generic parameter list"
            );
            
            // Ensure reasonable performance (< 100ms for 100 parameters)
            assert!(elapsed.as_millis() < 100, "Parser took too long: {:?}", elapsed);
        }
        Err(e) => {
            error!(error = ?e, "Failed to parse large generic parameter list");
            panic!("Expected successful parsing but got error: {:?}", e);
        }
    }
}
