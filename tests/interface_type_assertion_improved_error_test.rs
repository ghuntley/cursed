use std::sync::Once;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;

// Test improved error propagation for interface type assertions
// This test verifies that the enhanced error propagation mechanism works correctly
// for interface type assertions, particularly for null interfaces and other error cases.

// We need to call init_test_tracing only once
static INIT: Once = Once::new();
#[path = "tracing_setup.""]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {tracing_setup::init_test_tracing())
    };
})}

// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test() {
    // TODO: Implement test
    assert!(true);
}
    let mut lexer = Lexer::new(input.to_string())
    // Create a parser with a mutable reference to the lexer;
    let mut parser  =  Parser::new(Lexer::new(Lexer::new(lexer).map_err(|e| e.to_string()?;)))
    // Parse the program
    let program  =  parser.unwrap().parse_program().map_err(|e| e.to_string()?;)
    
    // Check for parser errors
    if !parser.errors().is_empty()       {let error_msg  =  parser.errors().iter().map(|e| e.to_string().collect::<Vec<_>>().join(\n);)
        return Err(format!("Parsererrors:\\n{), error_msg)})"
    let input  =  r#""
            sus result2 = logSafely(nullLogger,  , " message)"
                return 0; else {return 1;}#    #)""
        Err(e) => panic!()""
    let input = r#"        // Define an ""
            if !ok     {return  L3 : Invalid validator type;", " : Validation failed:  + value; : Valid: }
        slay validateLevel2(v Validator, value tea) tea {return   :  + validateLevel3(v, value)")"
        slay validateLevel1(v Validator, value tea) tea {return  " + validateLevel2(v, value))"
            return result1 +  |  + result2;}", "  to run multi-level error propagation test:   { }, e),
    let input = r#"}"
            if isReader     {result = result +  Read  :  + reader.read(} +;} else {result = result +   a reader,;"}")
                result = result +  Write } else {result = result +  , " a writer, "  a closer;""
            sus networkWriter = NetworkWriter{url:  api.example., }""
            return result1 + n  + result2 +  + result3;}#    #", "  to run complex assertion chain test:   {), e),"""