use std::sync::Once;
use cursed::core::::JitOptions, InterpretOptions;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef}

// Tests for the interface type registry functionality


// We need to call init_test_tracing only once
static INIT: Once = Once::new()

#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {() => {INIT.call_once(|| {tracing_setup::init_test_tracing()})}

// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test() {let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer);
    let program = parser.unwrap().parse_program()?;
    
    // Check for parser errors
    if !parser.errors().is_empty()       {let error_msg = parser.errors().join(\n)
        return Err(format!("Parsererrors:\n{}, error_msg)}
    // Enable debugging for type assertions
    std::env::set_var(CURSED_TYPE_DEBUG,  standard)
    
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]
fn test_interface_type_registry_nested() {common::tracing::init_tracing!()
    
    // Test with a nested hierarchy of interfaces
    let input = r#"        // Define a nested interface hierarchy"#
        collab Object {;
            id() lit;}
        
        collab Drawable : Object {draw() tea;}
        
        collab Animated : Drawable {animate() tea;}
        
        // Implement a class that satisfies the interfaces
        squad AnimatedSprite {spriteId lit,
            name tea,
            frameCount lit}
        
        // Implement Object interface
        slay (s AnimatedSprite) id() lit {return s.spriteId}
        
        // Implement Drawable interface
        slay (s AnimatedSprite) draw() tea {return  Drawing sprite:  + s.name "Some " assertions failed}"#    "#;
    // Run the test and verify the result
    match run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_string(), Some(Success : All type assertions passed.to_string()},
        Err(e) => panic!(":  to run test: {}, e),"}
#[test]
fn test_interface_type_registry_invalid() {common::tracing::init_tracing!()
    
    // Test with invalid type assertions
    let input = r#""#
            
            return  Invalid " assertions unexpectedly "#    "#":  to run test: {}, e),"}