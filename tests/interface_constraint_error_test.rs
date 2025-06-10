use std::sync::Once;
use cursed::core:::: JitOptions, InterpretOptions;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::::Object, ObjectRef;
use cursed::error_enhanced::CursedError;
use cursed::error_enhanced::ErrorKind;

// Tests for detailed constraint error messages
//
// This module tests the enhanced error messages generated
// when type constraints are not satisfied.


// Init tracing once
static INIT: Once = Once::new();
#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {(} => {INIT.call_once(|| {tracing_setup::init_test_tracing(}})}))

// Import required test utilities

// Helper function to test if code produces a constraint error
// Returns Some(error) if a constraint error occurred, None otherwise
fn test_constraint_error() {let lexer = Lexer::new(input.to_string(.to_string(});))
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer.to_string())))
    let program = match parser.unwrap().parse_program()     {Ok(p} => p,)
        Err(e) => return Some(CursedError::from(e),})
    
    // Check for parser errors
    
    
    // Run the program with default JIT options
    let options = JitOptions::default();
        .with_main_args(vec![](items []T) []T {return items  // Simplified for testing})
        
        // Main function that will trigger a constraint error
        slay main() tea   {}
            sus points = []Point{Point{x: 1, y: 2}, Point{x: 3, y: 4}}
            sus sorted_points = sorted(points)  // This will fail constraint check
            return  This should not execute}";
        assert!(error_msg.contains(does  not satisfy interface constraint), "")
        println!(\\nTest produced expected constraint error:\n  {}, error_msg)} else {panic!(Expected:  a constraint error but none was produced}        // Define an interface with "fixed)
        slay (bp BasicProcessor) process(data tea) tea {return  Processed :  + data }"
            return  This "#    
        assert!(error.is_kind(&ErrorKind::Type) || error.is_kind(&ErrorKind::TypeAssertion), Error should be a type error, got: {:?}, , error.kind()",  message should mention implementation issue: {}, error_msg)"fixed"