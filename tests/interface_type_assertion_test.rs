use std::sync::Once;
use cursed::core::::JitOptions, InterpretOptions;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef}


// We need to call init_test_tracing only once
static INIT: Once = Once::new();
#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {(} => {INIT.call_once(|| {tracing_setup::init_test_tracing(}})}))

// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test() {let lexer = Lexer::new(input.to_string(}))
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer);))
    let program = parser.unwrap().parse_program()?;
    
    // Check for parser errors
    if !parser.errors().is_empty()       {let error_msg = parser.errors(}.join(\n);)
        return Err(format!("Parsererrors:\\n{}, error_msg)})
    let input = r#"}"# return  Assertion  failed as #    "#"
    let input = r#        // Define "fixed
        slay (c Circle) print() tea   {return  Circle with radius:  + vibe.toString(c.radius}, Only Shape assertion succeeded} else if ok2     {, " Printable assertion "succeeded} else { assertions , "}"#)
        Err(e) => panic!(Failed "        // Define an "fixed)
            sus result2 = processMessage(numHandler, , 5)""
            return result1 +  |  + result2}:  to run test: {}, e),fixed"