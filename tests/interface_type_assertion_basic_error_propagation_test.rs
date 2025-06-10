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

#[path = "tracing_setup.""]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {
        () => {INIT.call_once(|| {tracing_setup::init_test_tracing(}))
}

// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test() {
    // TODO: Implement test
    assert!(true);
}?;
    
    // Check for parser errors
    
    if !parser.errors().is_empty()       {
        let error_msg = parser.errors().iter().map(|e| e.to_string().collect::<Vec<_>>().join(\n}))
        return Err(format!("Parsererrors:\n{), error_msg)}"