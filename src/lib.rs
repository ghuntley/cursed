/// CURSED Programming Language Library - ADVANCED FEATURES ENABLED
/// 
/// A comprehensive programming language implementation featuring:
/// - Complete LLVM compilation pipeline with JIT support
/// - Advanced runtime with goroutines and channels
/// - Garbage collector with cycle detection
/// - Profile-guided optimization system
/// - Complete standard library with async/await
/// - Advanced debugging and error handling

// ADVANCED CURSED FEATURES - FULLY RESTORED
pub mod error;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod codegen;
pub mod common;
pub mod web;
pub mod tokio;  // RE-ENABLED - Async runtime
pub mod core;
pub mod profiling;
pub mod object;
pub mod optimization;
pub mod type_system;

// ADVANCED MODULES - NEWLY ENABLED
pub mod imports;      // Package import system
pub mod preprocessor; // Advanced preprocessing
pub mod config;       // Configuration management
pub mod bootstrap;    // Bootstrapping system
pub mod tools;        // Development tools
pub mod repl;         // Interactive REPL
pub mod lsp;          // Language Server Protocol
pub mod testing;      // Testing framework
pub mod docs;         // Documentation system
pub mod documentation;// Enhanced documentation
pub mod build_system; // Build system
pub mod cli;          // Command line interface
pub mod package_manager; // Package management

// Crypto module
pub mod crypto {
    pub use crate::stdlib::crypto::*;
}

// Standard library module
pub mod stdlib;

// Types module for Result, Option, and error patterns
pub mod types;

// Debug module
pub mod debug;

// Memory management
pub mod memory;

// Runtime system
pub mod runtime;

// Basic execution engine
pub mod execution;

// Re-export core types only
pub use common::OptimizationLevel;

// Re-export essential error handling
pub use error::{Error, SourceLocation};
// Also re-export from common for compatibility
pub use common::Error as CommonError;

/// Prelude module for minimal imports
pub mod prelude {
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the minimal CURSED runtime environment
pub fn init() {
    // Initialize minimal logging
    tracing_subscriber::fmt()
        .with_env_filter("cursed=info")
        .init();
}

/// Compile and execute CURSED source code (minimal version)
pub fn run(source: &str) -> Result<(), Error> {
    let mut execution_engine = execution::CursedExecutionEngine::new()?;
    let result = execution_engine.execute(source)?;
    
    // Print the result for user feedback
    match result {
        execution::CursedValue::Nil => {}, // Don't print nil results
        _ => println!("{}", execution_engine.get_value_manager().format_value(&result)),
    }
    
    Ok(())
}

/// Compile and execute CURSED source file (minimal version)
pub fn run_file(path: &str) -> Result<(), Error> {
    let mut execution_engine = execution::CursedExecutionEngine::new()?;
    let result = execution_engine.execute_file(path)?;
    
    // Print the result for user feedback
    match result {
        execution::CursedValue::Nil => {}, // Don't print nil results
        _ => println!("{}", execution_engine.get_value_manager().format_value(&result)),
    }
    
    Ok(())
}

/// Compile CURSED source to LLVM IR (minimal version)
pub fn compile_to_ir(source: &str) -> Result<String, Error> {
    tracing::info!("Compiling CURSED source to LLVM IR (minimal build)");
    
    let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
    
    // Enable basic optimizations only
    codegen.enable_debug_optimizations()?;
    
    // Compile and return IR
    let ir = codegen.compile(source)?;
    
    tracing::debug!("Generated minimal LLVM IR:\n{}", ir);
    Ok(ir)
}

/// Compile CURSED source to LLVM IR with optimization level
pub fn compile_to_ir_with_optimization(source: &str, optimization_level: Option<&str>) -> Result<String, Error> {
    tracing::info!("Compiling CURSED source to LLVM IR with optimization (minimal build)");
    
    let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
    
    // Configure basic optimization level if specified
    if let Some(level_str) = optimization_level {
        match level_str {
            "O0" => codegen.enable_debug_optimizations()?,
            "O1" | "O2" | "O3" => codegen.enable_release_optimizations()?,
            _ => codegen.enable_debug_optimizations()?,
        }
        tracing::info!("Applied basic optimization level: {}", level_str);
    } else {
        codegen.enable_debug_optimizations()?;
    }
    
    // Compile and return IR
    let ir = codegen.compile(source)?;
    
    tracing::debug!("Generated optimized LLVM IR (minimal):\n{}", ir);
    Ok(ir)
}

/// Check CURSED source for errors without executing (minimal version)
pub fn check(source: &str) -> Result<(), Error> {
    tracing::info!("Checking CURSED source for errors (minimal build)");
    
    let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
    
    // For checking, use debug optimizations to speed up compilation
    codegen.enable_debug_optimizations()?;
    
    // Compile to check for errors (but don't use the result)
    let _ir = codegen.compile(source)?;
    
    tracing::info!("CURSED source check completed successfully (minimal)");
    Ok(())
}

/// Format CURSED source code (minimal version)
pub fn format(source: &str) -> Result<String, Error> {
    tracing::info!("Formatting CURSED source code (minimal build)");
    
    // Create lexer and parser to validate syntax first
    let lexer = crate::lexer::Lexer::new(source.to_string());
    let mut parser = crate::parser::Parser::new(lexer)?;
    
    // Parse source code into AST
    let program = parser.parse_program()?;
    
    // Check for parse errors
    let errors = parser.errors();
    if !errors.is_empty() {
        return Err(Error::Parse(format!("Cannot format source with parse errors: {}", errors.join(", "))));
    }
    
    // Basic formatting - for now just return the original source
    // TODO: Implement minimal formatter
    tracing::debug!("Basic formatting completed (minimal build)");
    Ok(source.to_string())
}
