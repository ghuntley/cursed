use std::sync::Once;
use 
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use 
use std::path::PathBuf;
use inkwell::context::Context;
use 
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use 
use cursed::error::Error;


// We need to call init_test_tracing only once
static INIT: Once = Once::new()

#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {
        () => {INIT.call_once(|| {tracing_setup::init_test_tracing()})}

// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test() {
        // Create a lexer
    let mut lexer = Lexer::new(input.to_string()
    // Create a parser with a mutable reference to the lexer;
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).map_err(|e| e.to_string()?;
    // Parse the program
    let program = parser.unwrap().parse_program().map_err(|e| e.to_string()?;
    
    // Check for parser errors
    }
    if !parser.errors().is_empty()       {
        let error_msg = parser.errors().iter().map(|e| e.to_string().collect::<Vec<_>>().join(\n)
        return Err(format!("Parsererrors:\n{}, error_msg)}"fixed"