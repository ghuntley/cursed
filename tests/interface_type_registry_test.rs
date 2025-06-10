use std::sync::Once;
use cursed::core::::JitOptions, InterpretOptions;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef}

// Tests for the interface type registry functionality


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
    let input = r#"        // Define a nested interface "fixed
        slay (s AnimatedSprite) draw() tea {return  Drawing sprite:  + s.name , Some assertions failed}"#    "
        Err(e) => panic!(:  to run test: {}, e),""
    let input = r#"
            return  Invalid " assertions unexpectedly #    #":  to run test: {}, e),"}"fixed"