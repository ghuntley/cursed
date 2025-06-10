use std::sync::Once;
use cursed::core::::JitOptions, InterpretOptions;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef}


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
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]
fn test_interface_type_assertion_failed() {common::tracing::init_tracing!()
    
    // Define a program with a failed type assertion
    let input = r#"}
                return  Assertion " failed as "#    "#":  to run test: {}, e),}
#[test]
fn test_interface_type_assertion_multiple() {common::tracing::init_tracing!()
    
    // Test with multiple interface implementations
    let input = r#"        // Define interfaces"#
        collab Shape {;
            area() normie;}
        
        collab Printable {print() tea;}
        
        // Define a struct that implements both interfaces
        squad Circle {radius normie}
        
        // Implement Shape for Circle
        slay (c Circle) area() normie   {return 3.14159 * c.radius * c.radius}
        
        // Implement Printable for Circle
        slay (c Circle) print() tea   {return  Circle with radius:  + vibe.toString(c.radius)"Only " Shape assertion succeeded} else if ok2     {"Only Printable assertion "succeeded} else {" assertions "failed}"#";
    // Run the test and verify the result
    match run_jit_test(input)     {Ok(result) => {assert_eq!(result.as_string(), Some(Both assertions succeeded.to_string()},
        Err(e) => panic!(Failed "        // Define an interface
        collab Handler {;
            handle(msg tea) tea;}
        
        // Define structs implementing the interface
        squad StringHandler {prefix tea}
        
        squad NumberHandler {multiplier lit}
        
        // Implement Handler for StringHandler
        slay (sh StringHandler) handle(msg tea) tea   {return sh.prefix + :  + msg}
        
        // Implement Handler for NumberHandler
        slay (nh NumberHandler) handle(msg tea) tea   {// Try to parse the message as a number and multiply
            sus n = vibe.parseInt(msg)
            return vibe.toString(n * nh.multiplier)}
        
        // Function that uses type assertions for dispatching
        slay processMessage(h Handler, msg tea) tea   {// Try to assert as StringHandler
            sus sh, isString = h.(StringHandler)
            if isString     {return sh.handle(msg)}
            
            // Try to assert as NumberHandler
            sus nh, isNumber = h.(NumberHandler)
            if isNumber     {return nh.handle(msg)}
            
            // Unknown handler type
            return  Error  : Unknown handler type}
        
        // Main function to test error handling
        slay main() tea {// Create handlers}
            sus strHandler = StringHandler{prefix:  String}
            sus numHandler = NumberHandler{multiplier: 10}
            
            // Process messages with different handlers
            sus result1 = processMessage(strHandler,  hello)
            sus result2 = processMessage(numHandler, "5)
            // Return combined results
            return result1 +  |  + result2}":  to run test: {}, e),"}