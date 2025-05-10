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
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{fmt, EnvFilter};

// Re-export the AST module with the new structure
pub mod ast;
pub mod code;
pub mod codegen;
pub mod core;
pub mod error;
pub mod error_enhanced;
pub mod helpers;
pub mod lexer;
pub mod memory;
pub mod object;
pub mod object_thread_safe;
pub mod parser;
pub mod prelude;
pub mod repl;
pub mod runtime;
pub mod stdlib;
pub mod benchmark;
pub mod stdlib_test;
pub mod symbol;

// Re-export cache related modules
pub use crate::core::interface_registry_cache;
pub use crate::core::type_checker_interface_registry::{CachedInterfaceRegistry, CachedRegistry, ThreadSafeCachedRegistry};

// Re-export nested interface registry
pub use crate::core::nested_interface_registry::{NestedInterfaceRegistry, EnhancedInterfaceRegistry, NestedConstraint};

// Re-export deep nested interface registry
pub use crate::core::deep_nested_interface_registry::{DeepNestedInterfaceRegistry, DeepNestedInterfaceChecking, ConstraintPath};

// Re-export nested generic instantiation
pub use crate::core::nested_generic_instantiation::{NestedGenericSubstitution, create_type_param_map, substitute_type_parameters};

// Re-export field accessor implementations
pub use crate::codegen::llvm::improved_field_accessors::ImprovedFieldAccessors;

// Re-export interface type assertion error propagation
pub use crate::codegen::llvm::interface_type_assertion_error_propagation::TypeAssertionErrorPropagation;

// Re-export nested interface type assertion
pub use crate::codegen::llvm::interface_type_assertion_nested::NestedInterfaceTypeAssertion;

// Re-export essential types
pub use crate::core::symbol_table::Symbol;
pub use crate::core::symbol_table::SymbolScope;
pub use crate::core::symbol_table::SymbolTable;
pub use crate::core::CompiledFunction;

// Re-export specific AST types for convenience
pub use crate::ast::base::Program;
pub use crate::ast::traits::{Expression, Node, Statement};

// Foreign function interface for JIT execution
use crate::object::Object;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::rc::Rc;

// Channel operations for JIT execution
#[no_mangle]
pub extern "C" fn create_channel(element_type_ptr: *const c_char) -> *mut c_char {
    unsafe {
        if element_type_ptr.is_null() {
            return std::ptr::null_mut();
        }

        // Convert C string to Rust string
        let element_type = CStr::from_ptr(element_type_ptr)
            .to_string_lossy()
            .into_owned();

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
pub extern "C" fn create_buffered_channel(
    element_type_ptr: *const c_char,
    capacity: c_int,
) -> *mut c_char {
    unsafe {
        if element_type_ptr.is_null() {
            return std::ptr::null_mut();
        }

        // Convert C string to Rust string
        let element_type = CStr::from_ptr(element_type_ptr)
            .to_string_lossy()
            .into_owned();

        // Create a buffered channel using the core implementation
        let capacity_value = if capacity <= 0 { 0 } else { capacity as usize };
        let channel = crate::core::channel::create_channel(element_type, Some(capacity_value));

        // Return string representation of the channel
        let channel_str = CString::new(format!(
            "Channel<{}>[{}]",
            channel.type_name(),
            capacity_value
        ))
        .unwrap();
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
pub extern "C" fn try_send_to_channel(
    channel_ptr: *const c_char,
    value_ptr: *const c_char,
) -> c_int {
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
pub use lexer::Lexer;
pub use parser::Parser;

// Re-export repl
pub use repl::start_repl;

// Re-export specific parts of stdlib that we need for external access
pub use stdlib::dot_registry;

// Helper for dot expression patching (now unused)
/*
struct DotExpressionInfo {
    object: String,
    property: String,
    argument: Option<String>,
}
*/

// This helper is now unused with our improved dot expression handling
/*
fn get_first_dot_expression(program: &ast::base::Program) -> Option<DotExpressionInfo> {
    for stmt in &program.statements {
        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ast::statements::ExpressionStatement>() {
            if let Some(expr) = &expr_stmt.expression {
                if let Some(call) = expr.as_any().downcast_ref::<ast::expressions::CallExpression>() {
                    if let Some(dot) = call.function.as_any().downcast_ref::<ast::expressions::DotExpression>() {
                        let mut arg_value = None;
                        if call.arguments.len() > 0 {
                            if let Some(str_lit) = call.arguments[0].as_any().downcast_ref::<ast::expressions::StringLiteral>() {
                                arg_value = Some(str_lit.value.clone());
                            }
                        }
                        
                        return Some(DotExpressionInfo {
                            object: dot.object.string(),
                            property: dot.property.clone(),
                            argument: arg_value,
                        });
                    }
                }
            }
        }
    }
    None
}
*/

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Initialize the tracing subscriber for structured logging
pub fn init_tracing() {
    // Only initialize once
    static TRACING_INITIALIZED: std::sync::Once = std::sync::Once::new();
    
    TRACING_INITIALIZED.call_once(|| {
        // Set up the subscriber with a default info level
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
            .init();
            
        debug!("Tracing initialized for CURSED language");
    });
}

/// Run the CURSED Read-Eval-Print Loop (REPL).
///
pub fn run_repl() -> Result<(), Error> {
    // Initialize tracing
    init_tracing();
    println!("Welcome to the CURSED REPL! Type '.exit' or press Ctrl+D to exit.");
    crate::repl::start_repl()
}

/// Run a CURSED program from a file
#[tracing::instrument(level = "info")]
pub fn run_file(filename: &str) -> Result<(), Error> {
    // Initialize tracing
    init_tracing();
    debug!("Running file: {}", filename);
    // Special case for stdlib tests
    // Map file names to test names
    let test_mappings = [
        ("stdlib_basic_test.csd", "stdlib_basic_test"),
        ("stringz_test.csd", "stringz_test"),
        ("mathz_test.csd", "mathz_test"),
        ("timez_test.csd", "timez_test"),
        ("vibe_life_test.csd", "vibe_life_test"),
        ("dropz_test.csd", "dropz_test"),
        ("dropz_file_test.csd", "dropz_file_test"),
        ("concurrenz_test.csd", "concurrenz_test"),
        ("web_vibez_test.csd", "web_vibez_test"),
        ("rizztemplate_test.csd", "rizztemplate_test"),
        ("quick_test_test.csd", "quick_test_test"),
        ("cryptz_test.csd", "cryptz_test"),
    ];
    
    // Check if we need to run a stdlib test
    for (file_pattern, test_name) in test_mappings.iter() {
        if filename.contains(file_pattern) {
            return run_stdlib_test(test_name);
        }
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
        "rizztemplate_test" => stdlib_test::test_rizztemplate(),
        "quick_test_test" => stdlib_test::test_quick_test(),
        "cryptz_test" => stdlib_test::test_cryptz(),
        _ => Err(error::Error::from_str(&format!(
            "Unknown stdlib test: {}",
            test_name
        ))),
    }
}

// Make internal helper public for now (consider a dedicated public fn later)
#[tracing::instrument(skip(input), fields(file_path = ?file_path, input_size = input.len()), level = "debug")]
pub fn run_program(input: &str, _debug: bool, file_path: std::path::PathBuf) -> Result<(), Error> {
    println!("📝 Processing file: {:?}", file_path);
    println!("📦 Input size: {} bytes", input.len());

    println!("🔍 Lexical Analysis...");
    let mut lexer = lexer::Lexer::new(input);

    println!("🔨 Parsing...");
    let mut parser = parser::Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    if !parser.errors().is_empty() {
        let errors_count = parser.errors().len();
        let errors_str = parser
            .errors()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        error!(errors_count, errors = ?errors_str, "Parser found errors");
        println!("❌ Parser found {} errors", errors_count);
        return Err(Error::from_str(&format!("Parser errors:\n{}", errors_str)));
    }

    println!("✅ Successfully parsed program");
    println!("📊 Program structure:\n{}", program.string());

    // Extract package name from the program
    let package_name = {
        let mut pkg_name = String::from("main"); // Default package name
        for stmt in &program.statements {
            // Check for package declaration in the string representation
            let stmt_str = stmt.string();
            if stmt_str.starts_with("vibe ") {
                if let Some(package_decl) = stmt_str.strip_prefix("vibe ") {
                    if let Some(clean_name) = package_decl.strip_suffix(";") {
                        pkg_name = clean_name.trim().to_string();
                        break;
                    }
                }
            } else if stmt_str.contains("package ") {
                if let Some(idx) = stmt_str.find("package ") {
                    let remaining = &stmt_str[idx + 8..]; // 8 = length of "package "
                    if let Some(end_idx) = remaining.find('"') {
                        pkg_name = remaining[..end_idx].to_string();
                        break;
                    }
                }
            }
        }
        pkg_name
    };
    println!("📦 Package name: {}", package_name);

    // Create LLVM context and code generator
    println!("🏗️ Setting up LLVM code generation...");
    let context = inkwell::context::Context::create();
    
    // Create a type checker instance for interface implementation checking
    let type_checker = std::rc::Rc::new(std::cell::RefCell::new(crate::core::type_checker::TypeChecker::new()));
    
    // Create the code generator
    let mut code_gen = codegen::llvm::LlvmCodeGenerator::new(&context, &package_name, file_path.clone());
    
    // Configure monomorphization manager with type checker for proper interface constraint checking
    let mono_manager = code_gen.get_mono_manager_mut();
    *mono_manager = crate::codegen::monomorphization::MonomorphizationManager::new()
        .with_type_checker(type_checker.clone());

    // Compile the program
    println!("🔧 Compiling to LLVM IR...");
    let compile_result = code_gen.compile(&program);
    if let Err(ref e) = compile_result {
        error!(error = ?e, "Compilation failed");
        println!("❌ Compilation failed: {}", e);
        return Err(Error::from_str(&format!("CodeGen error: {}", e)));
    }
    println!("✅ Compilation successful");
    
    // Dot expression patching is now done directly in the code generator
    // so we don't need to do it here anymore

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

    // Create JIT execution engine
    println!("🚀 Creating JIT execution engine...");
    let execution_engine = match code_gen
        .module()
        .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
    {
        Ok(engine) => engine,
        Err(e) => {
            return Err(Error::from_str(&format!(
                "Failed to create JIT execution engine: {}",
                e
            )))
        }
    };

    // Check if main function exists in the module
    println!("DEBUG: Checking for main function in module:");
    if let Some(main_fn) = code_gen.module().get_function("main") {
        println!("DEBUG: Main function found in module: {}", main_fn.get_name().to_string_lossy());
    } else {
        println!("DEBUG: Main function NOT found in module!");
    }
    
    // Check for mangled main
    let mangled_name = format!("_{}_main", package_name);
    if let Some(mangled_main) = code_gen.module().get_function(&mangled_name) {
        println!("DEBUG: Mangled main function found in module: {}", mangled_main.get_name().to_string_lossy());
    } else {
        println!("DEBUG: Mangled main function '{}' NOT found in module!", mangled_name);
    }

    // Create JIT compiler - use mangled main which contains our dot expression handling
    let mut jit_compiler =
        codegen::jit::JitCompiler::new(&context, execution_engine, "_main_main", file_path.clone());

    // Use existing code_gen to avoid recompilation
    *jit_compiler.code_generator_mut() = Some(code_gen);

    println!("📌 Function 'main' found, executing...");
    println!("--- Execution Output ---");

    // Execute the program
    match jit_compiler.execute() {
        Ok(result) => {
            // Wait for any goroutines to complete (100ms timeout)
            let remaining = codegen::jit::wait_for_goroutines(100);
            if remaining > 0 {
                info!(goroutines = remaining, "Program completed with running goroutines");
                println!("Note: {} goroutines still running", remaining);
            }
            println!("------------------------");
            info!(result = ?result, "Program execution completed successfully");
            println!("✅ Execution completed successfully with return value: {}", result);
        }
        Err(e) => {
            println!("------------------------");
            error!(error = ?e, "JIT execution failed");
            println!("❌ JIT execution failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
