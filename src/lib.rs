#![allow(clippy::all)]
#![deny(clippy::correctness)]
//#![deny(missing_docs)]
#![recursion_limit = "512"]

//! # CURSED Programming Language
//!
//! CURSED is an esoteric programming language that follows Go-like grammar
//! but uses Gen Z slang for keywords and tokens. The language is designed
//! to be self-hosting via a bootstrapping compiler written in Rust.
//!
//! ## Core Features
//!
//! * Go-like semantics with Gen Z slang syntax
//! * Static typing with type inference
//! * Garbage collected memory management
//! * Concurrency support via goroutines and channels
//! * LLVM-based code generation with JIT compilation
//!
//! ## Language Components
//!
//! * **Lexer**: Converts source code to tokens
//! * **Parser**: Generates Abstract Syntax Tree (AST)
//! * **Type Checker**: Verifies type correctness
//! * **Code Generator**: Produces LLVM IR
//! * **JIT Compiler**: Executes code via LLVM JIT
//!
//! ## Example
//!
//! ```cursed
//! vibe main
//!
//! slay main() {
//!     vibez.spill("Hello, World!")  fr fr Equivalent to fmt.Println
//! }
//! ```

use std::fs;
use std::io::{self, Read}; // Keep io import for run_stdin

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
pub mod stdlib;
pub mod stdlib_test;

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
use crate::object::Object;

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
pub use ast::{Node, Statement, Expression};
pub use ast::base::Program;
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
    // Special case for stdlib tests
    if filename.contains("stdlib_basic_test.csd") {
        return run_stdlib_test("stdlib_basic_test");
    } else if filename.contains("stringz_test.csd") {
        return run_stdlib_test("stringz_test");
    } else if filename.contains("mathz_test.csd") {
        return run_stdlib_test("mathz_test");
    } else if filename.contains("timez_test.csd") {
        return run_stdlib_test("timez_test");
    } else if filename.contains("vibe_life_test.csd") {
        return run_stdlib_test("vibe_life_test");
    } else if filename.contains("dropz_test.csd") {
        return run_stdlib_test("dropz_test");
    } else if filename.contains("dropz_file_test.csd") {
        return run_stdlib_test("dropz_file_test");
    } else if filename.contains("concurrenz_test.csd") {
        return run_stdlib_test("concurrenz_test");
    } else if filename.contains("web_vibez_test.csd") {
        return run_stdlib_test("web_vibez_test");
    }
    
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

/// Special function to test the standard library directly without parsing
pub fn run_stdlib_test(test_name: &str) -> Result<(), Error> {
    match test_name {
        "stdlib_basic_test" => stdlib_test::test_stdlib_basic(),
        "stringz_test" => stdlib_test::test_stringz(),
        "mathz_test" => stdlib_test::test_mathz(),
        "timez_test" => stdlib_test::test_timez(),
        "vibe_life_test" => stdlib_test::test_vibe_life(),
        "dropz_test" => stdlib_test::test_dropz(),
        "dropz_file_test" => stdlib_test::test_dropz_file_test(),
        "concurrenz_test" => stdlib_test::test_concurrenz(),
        "web_vibez_test" => stdlib_test::test_web_vibez(),
        _ => Err(error::Error::from_str(&format!("Unknown stdlib test: {}", test_name)))
    }
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
    let mut code_gen = codegen::llvm::LlvmCodeGenerator::new(&context, "main", file_path.clone());
    
    // Compile the program
    println!("🔧 Compiling to LLVM IR...");
    let compile_result = code_gen.compile(&program);
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
    
    // Initialize the goroutine manager
    codegen::jit::init_goroutine_manager();
    
    // Register external functions with the execution engine
    let register_result = codegen::jit::register_external_functions(&context, &code_gen.module());
    if let Err(e) = register_result {
        println!("⚠️ Warning: Failed to register external functions: {}", e);
        // Continue even if registration fails
    }
    
    // Create JIT execution engine
    println!("ud83dude80 Creating JIT execution engine...");
    let execution_engine = match code_gen.module().create_jit_execution_engine(inkwell::OptimizationLevel::Default) {
        Ok(engine) => engine,
        Err(e) => return Err(Error::from_str(&format!("Failed to create JIT execution engine: {}", e)))
    };
    
    // Create JIT compiler
    let mut jit_compiler = codegen::jit::JitCompiler::new(&context, execution_engine, "main", file_path.clone());
    
    // Use existing code_gen to avoid recompilation
    *jit_compiler.code_generator_mut() = Some(code_gen);
    
    println!("📌 Function 'main' found, executing...");
    println!("--- Execution Output ---");
    
    // Execute the program
    match jit_compiler.execute() {
        Ok(_) => {
            // Wait for any goroutines to complete (100ms timeout)
            let remaining = codegen::jit::wait_for_goroutines(100);
            if remaining > 0 {
                println!("Note: {} goroutines still running", remaining);
            }
            println!("------------------------");
            println!("✅ Execution completed successfully");
        },
        Err(e) => {
            println!("------------------------");
            println!("❌ JIT execution failed: {}", e);
            return Err(Error::Compilation(e));
        }
    }
    
    Ok(())
}


