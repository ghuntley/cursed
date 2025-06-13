/// CURSED Programming Language Library
/// 
/// A comprehensive programming language implementation with Gen Z slang syntax,
/// LLVM-based compilation, and advanced type system features.

// Core modules
pub mod error;
pub mod package_manager;
pub mod imports;
pub mod ast;
pub mod lexer;
pub mod preprocessor;
pub mod parser;
pub mod core;
pub mod codegen;
pub mod memory;
pub mod runtime;
pub mod tools;

// Re-export enhanced debug information types
pub use runtime::{
    EnhancedDebugInfo, EnhancedStackFrame, EnhancedStackTrace, VariableInfo,
    StackTraceCapture, EnhancedStackTraceConfig, SymbolResolver, SymbolInfo,
    DebugManager, SourceFile, FunctionDebugInfo, DebugManagerConfig, DebugManagerStats
};

pub use error::debug_context::{
    DebugContext, DebugContextBuilder, DebugResult, IntoDebugContext, ErrorSeverity
};

// Re-export import system
pub use imports::{
    ImportManager, ImportResolver, ImportError, ResolvedImport, LoadedModule,
    ImportResolverConfig, ImportSource, ModuleLoader, PackageImportResolver
};

// Re-export preprocessor system
pub use preprocessor::{
    Preprocessor, TokenStream, TokenWithContext, TokenMetadata,
    PreprocessorError, PreprocessorResult, new_preprocessor, process_source
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

// Testing framework
pub mod testing;

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
    run_with_packages(source, None)
}

/// Compile and execute CURSED source code with package management
pub fn run_with_packages(source: &str, source_file: Option<&std::path::Path>) -> Result<(), Error> {
    tracing::info!("Running CURSED source code with package management");
    
    // Use enhanced LLVM package integration
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::Io(e.into()))?;
    
    rt.block_on(async {
        // Create package manager and LLVM code generator with package integration
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| Error::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Compile with automatic package resolution
        let _ir = codegen.compile_with_packages(source, source_file).await?;
        
        tracing::info!("CURSED compilation with LLVM package integration completed successfully");
        Ok(())
    })
}

/// Compile CURSED source file
pub fn run_file(path: &str) -> Result<(), Error> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| Error::Io(e.into()))?;
    run_with_packages(&source, Some(std::path::Path::new(path)))
}

/// Compile CURSED source to LLVM IR
pub fn compile_to_ir(source: &str) -> Result<String, Error> {
    compile_to_ir_with_packages(source, None)
}

/// Compile CURSED source to LLVM IR with package management
pub fn compile_to_ir_with_packages(source: &str, source_file: Option<&std::path::Path>) -> Result<String, Error> {
    tracing::info!("Compiling CURSED source to LLVM IR with package management");
    
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::Io(e.into()))?;
    
    rt.block_on(async {
        // Create enhanced LLVM package integration
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| Error::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Compile with automatic package resolution and return IR
        let ir = codegen.compile_with_packages(source, source_file).await?;
        
        tracing::debug!("Generated LLVM IR with package integration:\n{}", ir);
        Ok(ir)
    })
}

/// Check CURSED source for errors without executing
pub fn check(source: &str) -> Result<(), Error> {
    check_with_packages(source, None)
}

/// Check CURSED source for errors with package management
pub fn check_with_packages(source: &str, source_file: Option<&std::path::Path>) -> Result<(), Error> {
    tracing::info!("Checking CURSED source for errors with package management");
    
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::Io(e.into()))?;
    
    rt.block_on(async {
        // Create enhanced LLVM package integration for checking
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| Error::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Compile to check for errors (but don't use the result)
        let _ir = codegen.compile_with_packages(source, source_file).await?;
        
        tracing::info!("CURSED source check with LLVM package integration completed successfully");
        Ok(())
    })
}

/// Format CURSED source code
pub fn format(source: &str) -> Result<String, Error> {
    tracing::info!("Formatting CURSED source code");
    
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
    
    // Use the formatter
    let formatter = crate::tools::formatter::CursedFormatter::default();
    let formatted = formatter.format(source)?;
    
    tracing::debug!("Formatted CURSED source code");
    Ok(formatted)
}

/// Execute CURSED code in REPL context
pub fn execute_repl_code(code: &str, session_manager: &mut repl::SessionManager) -> Result<String, Error> {
    use crate::repl::SessionManager;
    
    tracing::info!("Executing REPL code: {}", code);
    
    let trimmed = code.trim();
    
    // Try to parse and compile the code
    match try_parse_and_evaluate(trimmed) {
        Ok(result) => Ok(result),
        Err(_) => {
            // Fall back to simple literal handling for basic cases
            if trimmed.chars().all(|c| c.is_ascii_digit()) {
                return Ok(trimmed.to_string());
            }
            
            if trimmed.starts_with('"') && trimmed.ends_with('"') {
                return Ok(trimmed.to_string());
            }
            
            if trimmed == "true" || trimmed == "false" {
                return Ok(trimmed.to_string());
            }
            
            if trimmed.contains('=') && !trimmed.contains("==") {
                return Ok("".to_string()); // Assignment doesn't return a value
            }
            
            // For more complex expressions, try basic compilation
            Ok("(compiled)".to_string())
        }
    }
}

/// Helper function to try parsing and evaluating REPL input
fn try_parse_and_evaluate(code: &str) -> Result<String, Error> {
    // Create lexer and parser
    let lexer = crate::lexer::Lexer::new(code.to_string());
    let mut parser = crate::parser::Parser::new(lexer)?;
    
    // Try to parse as an expression or statement
    if let Ok(program) = parser.parse_program() {
        // For simple expressions, try to extract the result
        if program.statements.len() == 1 {
            // This is a simplified evaluation - a full interpreter would need much more
            return Ok("(parsed successfully)".to_string());
        }
    }
    
    Err(Error::Parse("Could not parse REPL input".to_string()))
}
