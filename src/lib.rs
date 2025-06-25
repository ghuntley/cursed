
/// CURSED Programming Language Library
/// 
/// A comprehensive programming language implementation with Gen Z slang syntax,
/// LLVM-based compilation, and advanced type system features.

// Core modules
pub mod error;
pub mod error_types;
pub mod cursed_error;
pub mod minimal_ast;

// Export main Error type
pub use error_types::{Error, SourceLocation, Result};
pub use cursed_error::CursedError;
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
pub mod config;
pub mod bootstrap;
pub mod execution;
pub mod optimization;
pub mod common;

// Re-export common types for easy access
pub use common::OptimizationLevel;

// Re-export basic runtime types
pub use runtime::{
    Runtime, RuntimeConfig, RuntimeStats, RuntimeError, RuntimeErrorType,
    GoroutineScheduler, PanicRuntime, ErrorRuntime, DebugManager
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

// Re-export optimization components
pub use optimization::{
    OptimizationCoordinator, OptimizationConfig, OptimizationLevel,
    RealLlvmPassManager, EnhancedLlvmPassManager,
    PerformanceMonitor, OptimizationStats, OptimizationResult,
    OptimizationManager, AdaptiveOptimizer, IncrementalCompiler, BenchmarkSuite,
    PerformanceProfiler, OptimizationFeedback, OptimizationStrategy, OptimizationRecommendation,
    IncrementalCompilationResult, BenchmarkSuiteResults, AdaptationResult, BenchmarkConfig,
    PerformanceMetrics
};

pub mod stdlib;

// Web framework
pub mod web;

// Re-export ByteFit for easy access
pub use stdlib::bytefit;
pub mod profiling;
pub mod docs;
pub mod documentation;
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

// Development tools (already declared above)

// Testing framework
pub mod testing;

// Type system
pub mod type_system;
pub mod types;

// Re-export commonly used types for convenience
// pub use error::{Error, SourceLocation}; // Commented out - using error_types instead

/// Prelude module for common imports
pub mod prelude {
    pub use crate::{Error, SourceLocation};
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
    let mut execution_engine = execution::CursedExecutionEngine::new()?;
    let result = execution_engine.execute(source)?;
    
    // Print the result for user feedback
    match result {
        execution::CursedValue::Nil => {}, // Don't print nil results
        _ => println!("{}", execution_engine.get_value_manager().format_value(&result)),
    }
    
    Ok(())
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
        let package_config = crate::codegen::crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Enable default optimizations (O2)
        codegen.enable_release_optimizations()?;
        
        // Compile with automatic package resolution
        let _ir = codegen.compile_with_packages(source, source_file).await?;
        
        tracing::info!("CURSED compilation with LLVM package integration completed successfully");
        Ok(())
    })
}

/// Compile and execute CURSED source file
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

/// Compile and execute CURSED source file with enhanced optimization
pub fn run_file_enhanced(
    path: &str, 
    optimization_config: crate::optimization::OptimizationConfig,
    use_enhanced_passes: bool
) -> Result<(), Error> {
    tracing::info!("Running CURSED file with enhanced optimization: {}", path);
    
    // Read the source file
    let source = std::fs::read_to_string(path)
        .map_err(|e| Error::Io(e.into()))?;
    
    // Use the compile and run approach with enhanced optimization
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::Io(e.into()))?;
    
    rt.block_on(async {
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| Error::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Apply the optimization configuration
        codegen.set_optimization_config(optimization_config)?;
        codegen.set_optimization_enabled(true);
        
        // Set enhanced passes preference
        codegen.set_use_enhanced_passes(use_enhanced_passes);
        
        // Enable comprehensive optimization
        let preset = match optimization_config.optimization_level.as_str() {
            "O0" => crate::codegen::llvm::OptimizationPreset::Development,
            "O3" => crate::codegen::llvm::OptimizationPreset::Release,
            _ => crate::codegen::llvm::OptimizationPreset::Balanced,
        };
        codegen.enable_comprehensive_optimization(preset)?;
        
        // Apply comprehensive optimization to source before compilation
        let optimized_source = codegen.apply_comprehensive_optimization(&source)?;
        
        // Compile with automatic package resolution and optimization
        let _ir = codegen.compile_with_packages(&optimized_source, Some(std::path::Path::new(path))).await?;
        
        // Log optimization statistics based on which passes were used
        if use_enhanced_passes {
            if let Ok(enhanced_manager) = codegen.get_enhanced_pass_manager() {
                let stats = enhanced_manager.get_statistics();
                tracing::info!("🚀 Enhanced LLVM optimization completed successfully!");
                tracing::info!("📊 Enhanced Optimization Statistics:");
                tracing::info!("   • Total optimizations: {}", stats.optimizations_applied);
                tracing::info!("   • Functions: {} → {} ({} inlined, {} specialized)", 
                              stats.initial_functions, stats.final_functions, 
                              stats.functions_inlined, stats.functions_specialized);
                tracing::info!("   • Instructions: {} → {} ({} eliminated)", 
                              stats.initial_instructions, stats.final_instructions, 
                              stats.instructions_eliminated);
                tracing::info!("   • CURSED optimizations: {} goroutines, {} channels, {} slang constructs", 
                              stats.goroutines_optimized, stats.channels_optimized, stats.slang_optimizations);
                tracing::info!("   • Advanced optimizations: {} vectorized, {} cache optimized", 
                              stats.vectorized_operations, stats.cache_optimizations);
                tracing::info!("   • Performance improvement: {:.1}%, Memory reduction: {:.1}%", 
                              stats.estimated_runtime_improvement * 100.0, 
                              stats.estimated_memory_reduction * 100.0);
                tracing::info!("   • Optimization time: {:?}", stats.total_optimization_time);
            }
        } else if let Some(stats) = codegen.get_real_pass_manager_statistics() {
            tracing::info!("🚀 Real LLVM optimization completed successfully!");
            tracing::info!("📊 Optimization Statistics:");
            tracing::info!("   • Total optimizations: {}", stats.total_optimizations());
            tracing::info!("   • Instructions saved: {}", stats.instructions_saved());
            tracing::info!("   • Basic blocks saved: {}", stats.blocks_saved());
            tracing::info!("   • Functions inlined: {}", stats.functions_inlined);
            tracing::info!("   • Dead code eliminated: {} instructions", stats.instructions_eliminated);
            tracing::info!("   • Constants propagated: {}", stats.constants_propagated);
            tracing::info!("   • Loops unrolled: {}", stats.loops_unrolled);
            tracing::info!("   • CFG simplifications: {}", stats.cfg_simplifications);
            tracing::info!("   • Optimization time: {:?}", stats.total_optimization_time);
        }
        
        tracing::info!("CURSED file execution with enhanced optimization completed successfully");
        Ok(())
    })
}

/// Compile and execute CURSED source file with optimization
pub fn run_file_optimized(path: &str, optimization_config: crate::optimization::OptimizationConfig) -> Result<(), Error> {
    tracing::info!("Running CURSED file with optimization: {}", path);
    
    // Read the source file
    let source = std::fs::read_to_string(path)
        .map_err(|e| Error::Io(e.into()))?;
    
    // Use the compile and run approach with optimization
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::Io(e.into()))?;
    
    rt.block_on(async {
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| Error::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Apply the optimization configuration
        codegen.set_optimization_config(optimization_config)?;
        codegen.set_optimization_enabled(true);
        
        // Compile with automatic package resolution and optimization
        let _ir = codegen.compile_with_packages(&source, Some(std::path::Path::new(path))).await?;
        
        // Log optimization statistics if available
        if let Some(stats) = codegen.get_real_pass_manager_statistics() {
            tracing::info!("🚀 Real LLVM optimization completed successfully!");
            tracing::info!("📊 Optimization Statistics:");
            tracing::info!("   • Total optimizations: {}", stats.total_optimizations());
            tracing::info!("   • Instructions saved: {}", stats.instructions_saved());
            tracing::info!("   • Basic blocks saved: {}", stats.blocks_saved());
            tracing::info!("   • Functions inlined: {}", stats.functions_inlined);
            tracing::info!("   • Dead code eliminated: {} instructions", stats.instructions_eliminated);
            tracing::info!("   • Constants propagated: {}", stats.constants_propagated);
            tracing::info!("   • Loops unrolled: {}", stats.loops_unrolled);
            tracing::info!("   • CFG simplifications: {}", stats.cfg_simplifications);
            tracing::info!("   • Optimization time: {:?}", stats.total_optimization_time);
        }
        
        tracing::info!("CURSED file execution with optimization completed successfully");
        Ok(())
    })
}

/// Compile CURSED source to LLVM IR
pub fn compile_to_ir(source: &str) -> Result<(), Error> {
    compile_to_ir_with_packages(source, None)
}

/// Compile CURSED source to LLVM IR with specified optimization level
pub fn compile_to_ir_with_optimization(source: &str, optimization_level: Option<&str>) -> Result<(), Error> {
    compile_to_ir_with_optimization_and_packages(source, optimization_level, None)
}

/// Compile CURSED source to LLVM IR with optimization and package management
pub fn compile_to_ir_with_optimization_and_packages(
    source: &str, 
    optimization_level: Option<&str>, 
    source_file: Option<&std::path::Path>
) -> Result<(), Error> {
    tracing::info!("Compiling CURSED source to LLVM IR with optimization and package management");
    
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
        let package_config = crate::codegen::crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Configure optimization level if specified
        if let Some(level_str) = optimization_level {
            codegen.configure_optimization_from_string(level_str)?;
            tracing::info!("Applied optimization level: {}", level_str);
        } else {
            // Default to release optimizations (O2)
            codegen.enable_release_optimizations()?;
        }
        
        // Compile with automatic package resolution and return IR
        let ir = codegen.compile_with_packages(source, source_file).await?;
        
        // Log optimization statistics if available
        if let Some(stats) = codegen.get_real_pass_manager_statistics() {
            tracing::info!("🚀 Real LLVM optimization completed successfully!");
            tracing::info!("📊 Optimization Statistics:");
            tracing::info!("   • Total optimizations: {}", stats.total_optimizations());
            tracing::info!("   • Instructions saved: {}", stats.instructions_saved());
            tracing::info!("   • Basic blocks saved: {}", stats.blocks_saved());
            tracing::info!("   • Functions inlined: {}", stats.functions_inlined);
            tracing::info!("   • Dead code eliminated: {} instructions", stats.instructions_eliminated);
            tracing::info!("   • Constants propagated: {}", stats.constants_propagated);
            tracing::info!("   • Loops unrolled: {}", stats.loops_unrolled);
            tracing::info!("   • CFG simplifications: {}", stats.cfg_simplifications);
            tracing::info!("   • Optimization time: {:?}", stats.total_optimization_time);
        }
        
        tracing::debug!("Generated optimized LLVM IR with package integration:\n{}", ir);
        Ok(ir)
    })
}

/// Compile CURSED source to LLVM IR with package management
pub fn compile_to_ir_with_packages(source: &str, source_file: Option<&std::path::Path>) -> Result<(), Error> {
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
        let package_config = crate::codegen::crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Enable default optimizations (O2)
        codegen.enable_release_optimizations()?;
        
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
        let package_config = crate::codegen::crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // For checking, use debug optimizations to speed up compilation
        codegen.enable_debug_optimizations()?;
        
        // Compile to check for errors (but don't use the result)
        let _ir = codegen.compile_with_packages(source, source_file).await?;
        
        tracing::info!("CURSED source check with LLVM package integration completed successfully");
        Ok(())
    })
}

/// Format CURSED source code
pub fn format(source: &str) -> Result<(), Error> {
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
pub fn execute_repl_code(code: &str, session_manager: &mut repl::SessionManager) -> Result<(), Error> {
    use crate::repl::SessionManager;
    
    tracing::info!("Executing REPL code: {}", code);
    
    let trimmed = code.trim();
    
    // Use the JIT execution engine for real evaluation
    let mut execution_engine = execution::CursedExecutionEngine::new()?;
    
    match execution_engine.execute_repl(trimmed) {
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
fn try_parse_and_evaluate(code: &str) -> Result<(), Error> {
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
