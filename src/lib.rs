/// CURSED Programming Language Library
/// 
/// A comprehensive programming language implementation with Gen Z slang syntax,
/// LLVM-based compilation, and advanced type system features.

// Core modules
pub mod error;
pub mod package_manager;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod core;
pub mod codegen;
pub mod memory;
pub mod runtime;

// Re-export enhanced debug information types
pub use runtime::{
    EnhancedDebugInfo, EnhancedStackFrame, EnhancedStackTrace, VariableInfo,
    StackTraceCapture, EnhancedStackTraceConfig, SymbolResolver, SymbolInfo,
    DebugManager, SourceFile, FunctionDebugInfo, DebugManagerConfig, DebugManagerStats
};

pub use error::debug_context::{
    DebugContext, DebugContextBuilder, DebugResult, IntoDebugContext, ErrorSeverity
};

// Re-export enhanced debugging system
pub use debug::{
    EnhancedDebugInfo as EnhancedDebugInfoNew, DebugInfoRegistry, SymbolMetadata, 
    TypeDebugInfo, SourceMap, SymbolType, TypeKind
};

pub use runtime::debug_runtime::{
    RuntimeDebugger, VariableInspection, RuntimeStackFrame, Breakpoint
};
pub mod stdlib;
pub mod profiling;
pub mod docs;
pub mod object;
pub mod debug;

// Build system
pub mod build_system;

// CLI utilities  
pub mod cli;

// REPL (Read-Eval-Print Loop)
pub mod repl;

// Language Server Protocol
pub mod lsp;

// Development tools
pub mod tools;

// Type system
pub mod type_system;
pub mod types;

// Re-export commonly used types for convenience
pub use error::{Error, SourceLocation};

/// Prelude module for common imports
pub mod prelude {
    pub use crate::error::{Error, SourceLocation};
    pub use crate::repl::CursedRepl;
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the CURSED runtime environment
pub fn init() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("cursed=info")
        .init();
}

/// Compile and execute CURSED source code
pub fn run(source: &str) -> Result<(), Error> {
    // This is a placeholder - implement actual compilation pipeline
    tracing::info!("Running CURSED source code");
    Ok(())
}

/// Compile CURSED source file
pub fn run_file(path: &str) -> Result<(), Error> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| Error::Io(e.into()))?;
    run(&source)
}

/// Compile CURSED source to LLVM IR
pub fn compile_to_ir(source: &str) -> Result<String, Error> {
    // Placeholder implementation
    tracing::info!("Compiling CURSED source to LLVM IR");
    Ok("define i32 @main() {\n  ret i32 0\n}".to_string())
}

/// Check CURSED source for errors without executing
pub fn check(source: &str) -> Result<(), Error> {
    // Placeholder implementation
    tracing::info!("Checking CURSED source for errors");
    Ok(())
}

/// Format CURSED source code
pub fn format(source: &str) -> Result<String, Error> {
    // Placeholder implementation
    tracing::info!("Formatting CURSED source code");
    Ok(source.to_string())
}

/// Execute CURSED code in REPL context
pub fn execute_repl_code(code: &str, session_manager: &mut repl::SessionManager) -> Result<String, Error> {
    use crate::repl::SessionManager;
    
    // This would integrate with the actual CURSED interpreter
    // For now, provide a basic evaluation
    tracing::info!("Executing REPL code: {}", code);
    
    let trimmed = code.trim();
    
    // Handle variable assignments
    if trimmed.contains('=') && !trimmed.contains("==") {
        return Ok("".to_string()); // Assignment doesn't return a value
    }
    
    // Handle simple expressions
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Ok(trimmed.to_string());
    }
    
    // Handle string literals
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        return Ok(trimmed.to_string());
    }
    
    // Handle boolean literals
    if trimmed == "true" || trimmed == "false" {
        return Ok(trimmed.to_string());
    }
    
    // Default evaluation result
    Ok("(result)".to_string())
}
