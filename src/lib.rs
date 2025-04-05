#![allow(clippy::all)]
#![allow(missing_docs)]
#![deny(clippy::correctness)]
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
pub mod object_thread_safe;
pub mod repl;
pub mod helpers;
pub mod core;

// Re-export essential types
pub use core::CompiledFunction;
pub use core::symbol_table::SymbolTable;
pub use core::symbol_table::Symbol;
pub use core::symbol_table::SymbolScope;

// Foreign function interface for JIT execution
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;
use std::cell::RefCell;

// Channel operations for JIT execution
#[no_mangle]
pub extern "C" fn create_channel(element_type_ptr: *const c_char) -> *mut c_char {
    unsafe {
        if element_type_ptr.is_null() {
            return std::ptr::null_mut();
        }
        
        // Convert C string to Rust string
        let element_type = CStr::from_ptr(element_type_ptr).to_string_lossy().into_owned();
        
        // Create a channel using the core implementation (unbuffered)
        let channel = crate::core::channel::create_channel(element_type, None);
        
        // For JIT, we'll just return a string representation of the channel
        // In a real implementation, we would need proper memory management
        let channel_str = CString::new(format!("Channel<{}>", channel.type_name())).unwrap();
        let result = channel_str.into_raw();
        result
    }
}

#[no_mangle]
pub extern "C" fn create_buffered_channel(element_type_ptr: *const c_char, capacity: c_int) -> *mut c_char {
    unsafe {
        if element_type_ptr.is_null() {
            return std::ptr::null_mut();
        }
        
        // Convert C string to Rust string
        let element_type = CStr::from_ptr(element_type_ptr).to_string_lossy().into_owned();
        
        // Create a buffered channel using the core implementation
        let capacity_value = if capacity <= 0 { 0 } else { capacity as usize };
        let channel = crate::core::channel::create_channel(element_type, Some(capacity_value));
        
        // Return string representation of the channel
        let channel_str = CString::new(format!("Channel<{}>[{}]", channel.type_name(), capacity_value)).unwrap();
        let result = channel_str.into_raw();
        result
    }
}

#[no_mangle]
pub extern "C" fn send_to_channel(channel_ptr: *const c_char, value_ptr: *const c_char) -> c_int {
    unsafe {
        if channel_ptr.is_null() || value_ptr.is_null() {
            return 1; // Error
        }
        
        // In a real implementation, we would extract the channel object from the pointer
        // and send the value, handling blocking as needed
        // For this implementation, we'll just return success
        0 // Success
    }
}

#[no_mangle]
pub extern "C" fn try_send_to_channel(channel_ptr: *const c_char, value_ptr: *const c_char) -> c_int {
    unsafe {
        if channel_ptr.is_null() || value_ptr.is_null() {
            return -1; // Error
        }
        
        // In a real implementation, we would:
        // 1. Extract the channel object from the pointer
        // 2. Try to send the value non-blocking
        // 3. Return: 0 = sent successfully, 1 = would block, -1 = error
        0 // Success (assume channel not full)
    }
}

#[no_mangle]
pub extern "C" fn receive_from_channel(channel_ptr: *const c_char) -> *mut c_char {
    unsafe {
        if channel_ptr.is_null() {
            return std::ptr::null_mut();
        }
        
        // In a real implementation, we would:
        // 1. Extract the channel object from the pointer
        // 2. Perform a blocking receive
        // 3. Convert the received value to a C string
        
        // For this implementation, we'll return a fixed value
        let value_str = CString::new("42").unwrap();
        value_str.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn try_receive_from_channel(channel_ptr: *const c_char) -> *mut c_char {
    unsafe {
        if channel_ptr.is_null() {
            return std::ptr::null_mut();
        }
        
        // In a real implementation, we would:
        // 1. Extract the channel object from the pointer
        // 2. Perform a non-blocking receive
        // 3. Return null for would-block, error string for error, or the value
        
        // For this implementation, we'll return a fixed value
        let value_str = CString::new("42").unwrap();
        value_str.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn close_channel(channel_ptr: *const c_char) -> c_int {
    unsafe {
        if channel_ptr.is_null() {
            return 1; // Error
        }
        
        // In a real implementation, we would:
        // 1. Extract the channel object from the pointer
        // 2. Close the channel
        
        // For this implementation, we'll just return success
        0 // Success
    }
}

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


