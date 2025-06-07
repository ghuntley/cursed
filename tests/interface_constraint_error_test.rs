use std::sync::Once;
use cursed::core::{JitOptions, InterpretOptions};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef};
use cursed::error_enhanced::CursedError;
use cursed::error_enhanced::ErrorKind;

//! Tests for detailed constraint error messages
//!
//! This module tests the enhanced error messages generated
//! when type constraints are not satisfied.


// Init tracing once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import required test utilities

// Helper function to test if code produces a constraint error
// Returns Some(error) if a constraint error occurred, None otherwise
fn test_constraint_error(input: &str) -> Option<CursedError> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => return Some(CursedError::from(e)),
    };
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().join("\n");
        return Some(CursedError::new(ErrorKind::Parser, error_msg));
    }
    
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]);
        
    match cursed::code::jit_compile_and_run(&program, options) {
        Ok(_) => None, // No error occurred
        Err(e) => {
            // Convert the error to a CursedError
            match e {
                cursed::error::Error::TypeAssertion(ce) => Some(ce),
                _ => Some(CursedError::from(e)),
            }
        }
    }
}

#[test]
fn test_constraint_error_message_contains_missing_methods() {
    init_tracing!();
    
    // Define a test with a constraint violation
    let input = r#"
        // Define an interface with methods
        collab Comparable {
            compare(other Comparable) normie;
        }
        
        // Define a struct that doesn't implement the interface
        squad Point {
            x normie,
            y normie
        }
        
        // Generic function requiring Comparable constraint
        slay sorted[T: Comparable](items []T) []T {
            return items  // Simplified for testing
        }
        
        // Main function that will trigger a constraint error
        slay main() tea {
            sus points = []Point{Point{x: 1, y: 2}, Point{x: 3, y: 4}}
            sus sorted_points = sorted(points)  // This will fail constraint check
            return "This should not execute"
        }
    "#;
    
    // Run the test and verify we get a constraint error
    if let Some(error) = test_constraint_error(input) {
        // Verify it's a type error
        assert_eq!(error.kind(), &ErrorKind::Type);
        
        // Verify it contains detailed information
        let error_msg = error.to_string());
        assert!(error_msg.contains("does not satisfy interface constraint"), 
                "Error message should mention constraint: {}", error_msg);
        
        // Verify it contains the function and type parameter name
        assert!(error_msg.contains("sorted"), 
                "Error message should mention function name: {}", error_msg);
        assert!(error_msg.contains("T"), 
                "Error message should mention type parameter: {}", error_msg);
                
        // Verify code and context are provided
        assert!(error_msg.contains("CNST"), 
                "Error message should contain error code: {}", error_msg);
                
        // Print for debugging
        println!("\nTest produced expected constraint error:\n{}", error_msg);
    } else {
        panic!("Expected a constraint error but none was produced");
    }
}

#[test]
fn test_direct_interface_implementation_error() {
    init_tracing!();
    
    // Define a test with a direct interface implementation error
    let input = r#"
        // Define an interface with methods
        collab Processor {
            process(data tea) tea;
            canProcess(data tea) lit;
        }
        
        // Define a struct with only one of the required methods
        squad BasicProcessor {
            name tea
        }
        
        // Implement only one method
        slay (bp BasicProcessor) process(data tea) tea {
            return "Processed: " + data
        }
        
        // Main function that will trigger a constraint error
        slay main() tea {
            sus bp = BasicProcessor{name: "Simple"}
            
            // This will fail because BasicProcessor doesn't implement all Processor methods
            sus p Processor = bp
            
            return "This should not execute"
        }
    "#;
    
    // Run the test and verify we get a constraint error
    if let Some(error) = test_constraint_error(input) {
        // Verify it's a type error
        assert!(error.is_kind(&ErrorKind::Type) || error.is_kind(&ErrorKind::TypeAssertion),
                "Error should be a type error, got: {:?}", error.kind());
        
        // Verify it contains detailed information
        let error_msg = error.to_string());
        assert!(error_msg.contains("does not implement"), 
                "Error message should mention implementation issue: {}", error_msg);
        
        // Verify it mentions the missing method
        assert!(error_msg.contains("canProcess"), 
                "Error message should mention missing method: {}", error_msg);
                
        // Print for debugging
        println!("\nTest produced expected interface implementation error:\n{}", error_msg);
    } else {
        panic!("Expected an interface implementation error but none was produced");
    }
}