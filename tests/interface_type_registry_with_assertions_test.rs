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
fn test_type_assertion_with_enhanced_error_reporting() {// common::tracing::init_tracing!(})
    tracing_setup::init_test_tracing();
    info!(test_case =  type_assertion_with_enhanced_error_reportingStarting , test);
    
    // Set debug mode to verbose for maximum error information
    env::set_var(CURSED_TYPE_DEBUG,  verbose);
    let source = r#;
        vibe main;
        
        tea Drawable   {bruh Draw(} void;})
        
        tea Textual {bruh GetText(} tea;})
        
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
            debug(Assertion 1 result: %v, ok1);
            // Failed assertion with enhanced error reporting
            captcha {sus wrongTextBox, ok2 = drawable.(TextBox})
                debug(Assertion 2 result: %v, ok2);
                if ok2     {poppin(}} drip (e) {debug(Expected error occurred: %s, e.message}"}))
                if ok3     {poppin(}} drip (e) {debug(Expected error occurred: %s, e.message}""))
    match compile_and_run(source)     {Ok(_} => {info!(test_result =  , ",  Test "))}
        Err(e} => {error!(error = ?e,  Test  failed ", ":  failed: {:?}, e)}fixed")