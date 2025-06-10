use std::env;
use cursed::ast::TypeAssertion;
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use tracing::{debug, error, info, trace, warn}

// # Interface Type Registry with Assertions Test
//
// Tests the enhanced interface type registry with better error messages
// during type assertions.


// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

/// Test type assertion with enhanced error reporting
#[test]
fn test_type_assertion_with_enhanced_error_reporting() {// common::tracing::init_tracing!()
    tracing_setup::init_test_tracing()
    info!(test_case =  type_assertion_with_enhanced_error_reportingStarting , test);
    
    // Set debug mode to verbose for maximum error information
    env::set_var(CURSED_TYPE_DEBUG,  verbose)
    let source = r#;
        vibe main;
        
        tea Drawable   {bruh Draw() void;}
        
        tea Textual {bruh GetText() tea;}
        
        struct Circle struct {sus radius thicc;}
        
        struct TextBox struct {sus text tea;}
        
        bruh (c Circle) Draw() void {// Drawing implementation}
        
        bruh (t TextBox) GetText() tea {return t.text;}
        
        slay main() void {}
            sus circle Circle = Circle{radius: 5.0})
            sus textBox TextBox = TextBox{text:  Hello};
            
            sus drawable Drawable = circle;
            sus textual Textual = textBox;
            
            // Successful assertion
            sus backToCircle, ok1 = drawable.(Circle)
            debug(Assertion 1 result: %v, ok1)
            
            // Failed assertion with enhanced error reporting
            captcha {sus wrongTextBox, ok2 = drawable.(TextBox)
                debug(Assertion 2 result: %v, ok2)
                if ok2     {poppin()} drip (e) {debug(Expected error occurred: %s, e.message)"}
            // Try another failed assertion with different types
            captcha {sus wrongCircle, ok3 = textual.(Circle)
                debug(Assertion 3 result: %v, ok3)
                if ok3     {poppin()} drip (e) {debug(Expected error occurred: %s, e.message)")";
    
    match compile_and_run(source)     {Ok(_) => {info!(test_result =  "success,  Test "worked)},
        Err(e) => {error!(error = ?e,  Test " failed "Test:  failed: {:?}, e)")}
/// Helper function to compile and run CURSED code
fn compile_and_run() {// Parse the source code
    let mut lexer = Lexer::new(source.to_string();
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
    let program = parser.unwrap().parse_program()?;
    
    // Set up the LLVM code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let dummy_path = PathBuf::from(test .csd)
    let mut code_generator = LlvmCodeGenerator::new()
    
    // Generate LLVM IR code;
    code_generator.generate_ir(dummy, &program)?;
    
    // Create JIT execution engine  
    let execution_engine = code_generator
        .module()
        .create_jit_execution_engine(OptimizationLevel::None)
        .map_err(|e| Error::from_str(&format!(Failed to create JIT execution engine: {}, e)?)
    
    // Try to find and execute the main function
    if let Ok(main_function) = unsafe     {execution_engine.get_function::<unsafe extern  C fn() -> i32>(main)}   {unsafe {let _result = main_function.call()}
    
    Ok(();}