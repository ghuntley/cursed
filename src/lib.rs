#![warn(missing_docs)]
#![warn(clippy::all)]
#![recursion_limit = "512"]

// Just declare the modules, don't `use` them here if also declared below
// use inkwell::context::Context; // Context is used directly in functions
// use crate::lexer; // Declared below
// use crate::parser; // Declared below
// use crate::codegen; // Declared below
// use crate::error::{Error, SourceLocation}; // Error is used, SourceLocation isn't directly
// use crate::repl::start_repl; // Called directly below
use std::fs;
use std::io::{self, Read}; // Keep io import for run_stdin
// use std::path::PathBuf; // PathBuf is used directly in functions

/// The CURSED programming language implementation
/// 
/// This crate provides the main API for the CURSED language,
/// including lexer, parser, compiler, and LLVM code generation.

pub mod ast;
pub mod code;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod memory;
pub mod parser;
pub mod symbol;
pub mod prelude;
pub mod object;
pub mod repl;
pub mod helpers;
pub mod core;

// Re-export essential types
pub use core::CompiledFunction;
pub use core::symbol_table::SymbolTable;
pub use core::symbol_table::Symbol;
pub use core::symbol_table::SymbolScope;

// Re-export prelude
pub use prelude::*;

// Convenience re-exports at the crate level
pub use error::{Error, ErrorReporter, SourceLocation};
pub use ast::{Node, Statement, Expression, Program};
pub use lexer::Lexer;
pub use parser::Parser;

// Re-export repl
pub use repl::start_repl;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Run the CURSED Read-Eval-Print Loop (REPL).
///
pub fn run_repl() -> Result<(), Error> {
    println!("Welcome to the CURSED REPL! Type '.exit' or press Ctrl+D to exit.");
    crate::repl::start_repl()
}

/// Run a CURSED program from a file
pub fn run_file(filename: &str) -> Result<(), Error> {
    let input = fs::read_to_string(filename)
        .map_err(|e| Error::from_str(&format!("Failed to read file {}: {}", filename, e)))?;
    let file_path_buf = std::path::PathBuf::from(filename);
    run_program(&input, false, file_path_buf)
}

/// Run a CURSED program from standard input
/// 
/// Reads the entire standard input, parses it, and runs it.
/// 
/// # Errors
/// 
/// Returns an error if reading stdin fails, parsing fails, or execution fails.
pub fn run_stdin() -> Result<(), Error> {
    let mut input = String::new();
    // Use `?` which implicitly uses `From<io::Error>`
    io::stdin().read_to_string(&mut input)?;
    // Use a placeholder path for stdin
    let stdin_path = std::path::PathBuf::from("./stdin.csd"); 
    run_program(&input, false, stdin_path)
}

// Make internal helper public for now (consider a dedicated public fn later)
pub fn run_program(input: &str, _debug: bool, file_path: std::path::PathBuf) -> Result<(), Error> {
    println!("📝 Processing file: {:?}", file_path);
    println!("📦 Input size: {} bytes", input.len());
    
    println!("🔍 Lexical Analysis...");
    let mut lexer = lexer::Lexer::new(input);
    
    println!("🔨 Parsing...");
    let mut parser = parser::Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    if !parser.errors().is_empty() {
        println!("❌ Parser found {} errors", parser.errors().len());
        let errors_str = parser.errors().iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n");
        return Err(Error::from_str(&format!("Parser errors:\n{}", errors_str))); 
    }

    println!("✅ Successfully parsed program");
    println!("📊 Program structure:\n{}", program.string());
    
    // Create LLVM context and code generator
    println!("🏗️ Setting up LLVM code generation...");
    let context = inkwell::context::Context::create();
    let mut code_gen = codegen::llvm::LlvmCodeGenerator::new(&context, "main", file_path);
    
    // Compile the program
    println!("🔧 Compiling to LLVM IR...");
    let compile_result = code_gen.compile_program(&program);
    if let Err(ref e) = compile_result {
        println!("❌ Compilation failed: {}", e);
        return Err(Error::from_str(&format!("CodeGen error: {}", e)));
    }
    println!("✅ Compilation successful");
    
    // Print the generated LLVM IR for debugging
    println!("📄 Generated LLVM IR:");
    println!("--- Generated LLVM IR ---");
    let ir = code_gen.module().print_to_string().to_string();
    println!("{}", ir);
    println!("-------------------------");
    
    // JIT Execution
    println!("🚀 Executing code using JIT...");
    match code_gen.module().create_jit_execution_engine(inkwell::OptimizationLevel::Default) {
        Ok(execution_engine) => {
            // Get main function
            unsafe {
                match execution_engine.get_function::<unsafe extern "C" fn()>("main") {
                    Ok(main_fn) => {
                        println!("📌 Function 'main' found, executing...");
                        println!("--- Execution Output ---");
                        main_fn.call();
                        println!("------------------------");
                        println!("✅ Execution completed successfully");
                    },
                    Err(e) => {
                        println!("⚠️ Function 'main' not found in the module: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("❌ Failed to create execution engine: {}", e);
            return Err(Error::from_str(&format!("JIT execution error: {}", e)));
        }
    }
    
    Ok(())
}


