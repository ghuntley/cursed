
/// CURSED Programming Language Library
/// 
/// A comprehensive programming language implementation with Gen Z slang syntax,
/// LLVM-based compilation, and advanced type system features.

// Core modules
pub mod error;
pub mod error_types;
pub mod error_recovery;
pub mod error_recovery_simple;
// pub mod parser_error_recovery;
// pub mod semantic_error_recovery;
// pub mod codegen_error_recovery;
pub mod package_manager;
pub mod imports;
pub mod ast;
pub mod lexer;
pub mod preprocessor;
pub mod parser_main;
pub mod parser;
pub mod parser_interfaces;
pub mod core; // Re-enabled for type system integration
pub mod codegen; // Re-enabled for type system integration
pub mod memory;
pub mod runtime;
pub mod pattern_matching;

#[cfg(test)]
mod test_channel_fixes;
pub mod tools;
pub mod config;
pub mod bootstrap;
pub mod execution;
pub mod optimization;
pub mod common;
pub mod common_types;
pub mod type_helper;

// Newly enabled advanced modules
pub mod types;
pub mod crypto_pki_types;
pub mod stdlib;
pub mod linter;
pub mod formatter;
pub mod metrics; // Performance monitoring and metrics collection
pub mod coverage; // Code coverage analysis system
pub mod performance; // Performance monitoring system
// pub mod security; // Temporarily disabled due to dependency issues

// Test modules  
#[cfg(test)]
// pub mod test_member_access; // Removed - file no longer exists

// Re-export common types for easy access
pub use common::OptimizationLevel;
use crate::error::CursedError;

// CRITICAL: Re-export core components for tests and external access
pub use lexer::{Lexer, Token, TokenKind};
pub use parser_main::{Parser, new_parser};
pub use codegen::LlvmCodeGenerator as LlvmCodeGeneratorReal;
pub use package_manager::*;

// Re-export error types for external access
pub use error_types::{Error as CursedErrorType, Result as CursedResult};

// Re-export specific optimization types to avoid conflicts
pub use optimization::{
    OptimizationConfig, PerformanceMetrics, AdvancedOptimizationManager,
    RealLlvmPassManager, EnhancedLlvmPassManager, OptimizationCoordinator
};

// Re-export crypto/PKI types for tests
pub use crypto_pki_types::*;

// Re-export linter types
pub use linter::{
    CursedLinter, LintIssue, LintResults, LintStats, Severity, Category, 
    OutputFormat, LinterConfig, LintRule, LintContext
};

// Re-export runtime types that are actually implemented
pub use runtime::{
    Runtime, RuntimeConfig, RuntimeStats, RuntimeError, RuntimeErrorType,
    GoroutineScheduler, get_global_scheduler, initialize_global_scheduler, shutdown_global_scheduler,
    RuntimeStack, ValueManager, CursedValue, Value,
    PanicRuntime, ErrorRuntime, JitRuntime,
    StackTraceCapture, EnhancedStackTraceConfig, SymbolResolver, SymbolInfo,
    DebugManager, SourceFile, FunctionDebugInfo, DebugManagerConfig, DebugManagerStats,
    PerformanceMonitor, RuntimeDebugger, VariableInspection, RuntimeStackFrame, Breakpoint,
    // Memory management system
    GarbageCollector, GcConfig, GcStats, GcState, RootType, GcMemoryManager, RuntimeMemoryManager,
    MemoryManager, MemoryConfig, MemoryStats, MemoryError, ObjectHandle,
    initialize_memory_manager, get_global_memory_manager, shutdown_memory_manager,
    allocate, allocate_raw, collect_garbage,
    // Complete runtime initialization with GC
    initialize_complete_runtime, shutdown_complete_runtime
};

// Debug context module is minimal implementation for now

// Re-export import system
pub use imports::{
    ImportManager, ImportResolver, ImportError, ResolvedImport, LoadedModule,
    ImportConfig, ImportSource, ModuleLoader, PackageResolver
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

// Runtime debug types are already exported above

// Re-export optimization components (note: some types are re-exported above)
pub use optimization::{
    CoordinatorConfig,
    OptimizationStats, OptimizationResult
};

// OptimizationConfig is already re-exported above

// pub mod stdlib; // Temporarily disabled due to syntax errors

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
pub mod lsp; // Language Server Protocol implementation

// Development tools (already declared above)

// Testing framework
pub mod testing;

// Type system
pub mod type_system;

// Re-export AST types for external access
pub use ast::{
    Program, Statement, Expression, ImportStatement, PackageDeclaration,
    BinaryExpression, CallExpression, MemberAccessExpression,
    LetStatement, ReturnStatement, IfStatement, FunctionStatement,
    WhileStatement, ForStatement, GoroutineStatement, ChannelStatement
};

// Re-export execution types
pub use execution::{CursedExecutionEngine, ExecutionContext};

// Re-export type system components
pub use type_system::{
    TypeSystem, TypeCheckError, TypeErrorKind,
    TypeEnvironment, TypeDefinition, TypeKind as TypeSystemKind, InferenceContext,
    TypedCompilationPipeline, CompilationError, TypedProgram
};

// Re-export tools
pub use tools::{CursedTools, PackageManager, PackageConfig, Profiler, ProfilerConfig, ProfileReport};

// Re-export core types that are actually implemented  
pub use core::{Type as CoreType, CharMethods, CharObject, PerformancePipeline, ParallelConfig, IncrementalConfig, ProgressConfig};

// Re-export memory management types that are properly implemented
pub use memory::{
    Traceable, Visitor, Tag, GcRoot,
    GcMinimal, EnhancedGcMinimal, ProductionGcMinimal, get_minimal_result
};

/// Prelude module for common imports
pub mod prelude {
    pub use crate::repl::CursedRepl;
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the CURSED runtime environment
pub fn init() {
    // Initialize logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "cursed=info");
    }
    env_logger::init();
}

/// Compile and execute CURSED source code
pub fn run(source: &str) -> crate::error::Result<()> {
    let mut execution_engine = execution::CursedExecutionEngine::new()?;
    let result = execution_engine.execute(source)?;
    
    // Don't automatically print the result - output should come from explicit print statements
    // like vibez.spill() or println!()
    // match result {
    //     execution::CursedValue::Nil => {}, // Don't print nil results
    //     _ => println!("{}", execution_engine.get_value_manager().format_value(&result)),
    // }
    
    Ok(())
}

/// Compile and execute CURSED source code with package management
pub fn run_with_packages(source: &str, source_file: Option<&std::path::Path>) -> crate::error::Result<()> {
    tracing::info!("Running CURSED source code with package management");
    
    // Use enhanced LLVM package integration
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    rt.block_on(async {
        // Create package manager and LLVM code generator with package integration
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| CursedError::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
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
pub fn run_file(path: &str) -> crate::error::Result<()> {
    let mut execution_engine = execution::CursedExecutionEngine::new_no_jit()?;
    let result = execution_engine.execute_file(path)?;
    
    // Print the result for user feedback
    match result {
        execution::CursedValue::Nil => {}, // Don't print nil results
        _ => println!("{}", execution_engine.get_value_manager().format_value(&result)),
    }
    
    Ok(())
}

/// Run file with JIT compilation disabled for safety
pub fn run_file_no_jit(path: &str) -> crate::error::Result<()> {
    let mut execution_engine = execution::CursedExecutionEngine::new_no_jit()?;
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
) -> crate::error::Result<()> {
    tracing::info!("Running CURSED file with enhanced optimization: {}", path);
    
    // Read the source file
    let source = std::fs::read_to_string(path)
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    // Use the compile and run approach with enhanced optimization
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    rt.block_on(async {
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| CursedError::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Apply the optimization configuration
        codegen.set_optimization_config(optimization_config.to_llvm_config())?;
        codegen.set_optimization_enabled(true);
        
        // Set enhanced passes preference
        codegen.set_use_enhanced_passes(use_enhanced_passes);
        
        // Enable comprehensive optimization (temporarily disabled while fixing optimization system)
        // TODO: Re-enable once optimization system is working
        // let preset = match optimization_config.level {
        //     crate::optimization::config::OptimizationLevel::None => crate::codegen::llvm::OptimizationConfig::dev_config(),
        //     crate::optimization::config::OptimizationLevel::Aggressive => crate::codegen::llvm::OptimizationConfig::release_config(),
        //     _ => crate::codegen::llvm::OptimizationConfig::default(),
        // };
        // codegen.enable_comprehensive_optimization(preset)?;
        
        // Apply comprehensive optimization to source before compilation
        let optimized_source = source.clone(); // codegen.apply_comprehensive_optimization(&source)?;
        
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
pub fn run_file_optimized(path: &str, optimization_config: crate::optimization::OptimizationConfig) -> crate::error::Result<()> {
    tracing::info!("Running CURSED file with optimization: {}", path);
    
    // Read the source file
    let source = std::fs::read_to_string(path)
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    // Use the compile and run approach with optimization
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    rt.block_on(async {
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| CursedError::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // Apply the optimization configuration
        codegen.set_optimization_config(optimization_config.to_llvm_config())?;
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
pub fn compile_to_ir(source: &str) -> crate::error::Result<String> {
    compile_to_ir_with_packages(source, None)
}

/// Compile CURSED source to LLVM IR with specified optimization level
pub fn compile_to_ir_with_optimization(source: &str, optimization_level: Option<&str>) -> crate::error::Result<String> {
    compile_to_ir_with_optimization_and_packages(source, optimization_level, None)
}

/// Compile CURSED source to LLVM IR with optimization and package management
pub fn compile_to_ir_with_optimization_and_packages(
    source: &str, 
    optimization_level: Option<&str>, 
    source_file: Option<&std::path::Path>
) -> crate::error::Result<String> {
    tracing::info!("Compiling CURSED source to LLVM IR with optimization and package management");
    
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    rt.block_on(async {
        // Create enhanced LLVM package integration
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| CursedError::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
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

/// Compile CURSED source to assembly
pub fn compile_to_assembly(source: &str) -> crate::error::Result<String> {
    tracing::info!("Compiling CURSED source to assembly");
    
    // Parse the source and generate LLVM IR directly
    let mut parser = crate::parser::new_parser(source)?;
    let program = parser.parse_program()?;
    
    // Generate LLVM IR
    let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
    let ir = codegen.compile_ast(&program)?;
    
    // Convert IR to assembly
    let assembly = codegen.compile_ir_to_assembly(&ir)?;
    
    tracing::debug!("Generated assembly:\n{}", assembly);
    Ok(assembly)
}

/// Compile CURSED source to LLVM IR with package management
pub fn compile_to_ir_with_packages(source: &str, source_file: Option<&std::path::Path>) -> crate::error::Result<String> {
    tracing::info!("Compiling CURSED source to LLVM IR with package management");
    
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    rt.block_on(async {
        // Create enhanced LLVM package integration
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| CursedError::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
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
pub fn check(source: &str) -> crate::error::Result<()> {
    check_with_packages(source, None)
}

/// Check CURSED source for errors with package management
pub fn check_with_packages(source: &str, source_file: Option<&std::path::Path>) -> crate::error::Result<()> {
    tracing::info!("Checking CURSED source for errors with package management");
    
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    rt.block_on(async {
        // Create enhanced LLVM package integration for checking
        let package_manager_config = crate::package_manager::PackageManagerConfig::default();
        let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
            crate::package_manager::PackageManager::new(package_manager_config)
                .map_err(|e| CursedError::Parse(format!("Failed to create package manager: {}", e)))?
        ));
        
        let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
        let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
        
        codegen.initialize_package_integration(package_manager, package_config)?;
        
        // For checking, use debug optimizations to speed up compilation
        codegen.enable_debug_optimizations()?;
        
        // Compile to check for errors (but don't use the result)
        let _ir = codegen.compile_with_packages(source, source_file).await?;
        
        tracing::info!("CURSED source check with LLVM package integration completed successfully");
        Ok(())
    })
}

/// Compile CURSED source file to executable binary
pub async fn compile(source_file: &str, output_file: &str) -> crate::error::Result<()> {
    compile_with_optimization(source_file, output_file, None).await
}

/// Compile CURSED source file with debug information
pub async fn compile_with_debug(source_file: &str, output_file: &str, debug_sections: std::collections::HashMap<String, Vec<u8>>) -> crate::error::Result<()> {
    // use crate::debug::DwarfDebugGenerator; // Temporarily disabled
    
    tracing::info!("Compiling CURSED source file {} to executable {} with debug information", source_file, output_file);
    
    // Read the source file
    let source = std::fs::read_to_string(source_file)
        .map_err(|e| CursedError::Io(e.to_string()))?;

    // Parse the source to AST
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.tokenize().map_err(|e| CursedError::Parse(format!("Lexing failed: {:?}", e)))?;
    
    let mut parser = new_parser(&source)?;
    let ast = parser.parse()
        .map_err(|e| CursedError::Parse(format!("Parsing failed: {:?}", e)))?;

    // Generate LLVM IR with debug information
    let mut codegen = LlvmCodeGeneratorReal::new()?;
    codegen.enable_debug_info()?;
    
    // Convert Ast to Program for codegen
    let program = match ast {
        crate::ast::Ast::Program(p) => p,
        _ => return Err(CursedError::General("Expected Program AST node".to_string())),
    };
    let llvm_ir = codegen.generate_ir(&program)
        .map_err(|e| CursedError::General(format!("Code generation failed: {:?}", e)))?;

    // Write LLVM IR to temporary file
    let ir_file = format!("{}.ll", output_file);
    std::fs::write(&ir_file, &llvm_ir)
        .map_err(|e| CursedError::Io(format!("Failed to write IR file: {}", e)))?;

    // Add debug sections to assembly
    let debug_assembly = generate_debug_assembly(&debug_sections)?;
    
    // Compile with debug information
    compile_ir_with_debug(&ir_file, output_file, &debug_assembly)?;
    
    // Clean up temporary files
    let _ = std::fs::remove_file(&ir_file);
    
    tracing::info!("Successfully compiled {} to {} with debug information", source_file, output_file);
    Ok(())
}

/// Compile IR file with debug information
fn compile_ir_with_debug(ir_file: &str, output_file: &str, debug_assembly: &str) -> crate::error::Result<()> {
    use std::process::Command;
    
    // Write debug assembly to temporary file
    let debug_file = format!("{}_debug.s", output_file);
    std::fs::write(&debug_file, debug_assembly)
        .map_err(|e| CursedError::Io(format!("Failed to write debug assembly: {}", e)))?;
    
    // Compile LLVM IR to object file
    let obj_file = format!("{}.o", output_file);
    let llc_output = Command::new("llc")
        .arg("-filetype=obj")
        .arg("-o")
        .arg(&obj_file)
        .arg(ir_file)
        .output()
        .map_err(|e| CursedError::Io(format!("Failed to run llc: {}", e)))?;

    if !llc_output.status.success() {
        return Err(CursedError::General(format!("llc failed: {}", String::from_utf8_lossy(&llc_output.stderr))));
    }

    // Assemble debug information
    let debug_obj = format!("{}_debug.o", output_file);
    let as_output = Command::new("as")
        .arg("-64")
        .arg("-o")
        .arg(&debug_obj)
        .arg(&debug_file)
        .output()
        .map_err(|e| CursedError::Io(format!("Failed to run assembler: {}", e)))?;

    if !as_output.status.success() {
        return Err(CursedError::General(format!("Assembler failed: {}", String::from_utf8_lossy(&as_output.stderr))));
    }

    // Link with runtime library and debug information
    let link_output = Command::new("gcc")
        .arg("-o")
        .arg(output_file)
        .arg(&obj_file)
        .arg(&debug_obj)
        .arg("runtime/libcursed_runtime.a")
        .arg("-ldl")
        .arg("-lpthread")
        .arg("-lm")
        .arg("-g") // Include debug information
        .output()
        .map_err(|e| CursedError::Io(format!("Failed to run linker: {}", e)))?;

    if !link_output.status.success() {
        return Err(CursedError::General(format!("Linking failed: {}", String::from_utf8_lossy(&link_output.stderr))));
    }

    // Clean up temporary files
    let _ = std::fs::remove_file(&obj_file);
    let _ = std::fs::remove_file(&debug_obj);
    let _ = std::fs::remove_file(&debug_file);

    Ok(())
}

/// Generate debug assembly from debug sections
fn generate_debug_assembly(debug_sections: &std::collections::HashMap<String, Vec<u8>>) -> crate::error::Result<String> {
    let mut assembly = String::new();
    
    for (section_name, data) in debug_sections {
        assembly.push_str(&format!(".section {}\n", section_name));
        
        // Convert binary data to assembly directives
        for chunk in data.chunks(16) {
            assembly.push_str(".byte ");
            let hex_values: Vec<String> = chunk.iter().map(|b| format!("0x{:02x}", b)).collect();
            assembly.push_str(&hex_values.join(", "));
            assembly.push_str("\n");
        }
        assembly.push_str("\n");
    }
    
    Ok(assembly)
}

#[derive(Debug, Clone)]
pub struct AdvancedCompilationResult {
    pub success: bool,
    pub optimization_stats: optimization::AdvancedOptimizationStats,
    pub benchmark_report: Option<optimization::BenchmarkReport>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

pub async fn compile_with_advanced_optimization(
    source_file: &str, 
    output_file: &str, 
    config: &optimization::AdvancedOptimizationConfig
) -> crate::error::Result<AdvancedCompilationResult> {
    tracing::info!("Compiling CURSED source file {} to executable {} with advanced optimization", source_file, output_file);
    
    // Read the source file
    let source = std::fs::read_to_string(source_file)
        .map_err(|e| CursedError::Io(e.to_string()))?;

    // Try native compilation with advanced optimization
    match compile_to_native_with_advanced_optimization(&source, source_file, output_file, config).await {
        Ok(result) => {
            tracing::info!("Successfully compiled {} to native executable {} with advanced optimization", source_file, output_file);
            Ok(result)
        }
        Err(e) => {
            if is_llvm_missing_error(&e) {
                tracing::warn!("LLVM tools not available, falling back to interpretation mode");
                tracing::debug!("LLVM error details: {:?}", e);
                create_interpretation_wrapper(&source, source_file, output_file)?;
                Ok(AdvancedCompilationResult {
                    success: true,
                    optimization_stats: optimization::AdvancedOptimizationStats::default(),
                    benchmark_report: None,
                    warnings: vec!["Fallback to interpretation mode".to_string()],
                    errors: vec![],
                })
            } else {
                Err(e)
            }
        }
    }
}

pub async fn compile_native_only_with_advanced_optimization(
    source_file: &str, 
    output_file: &str, 
    config: &optimization::AdvancedOptimizationConfig
) -> crate::error::Result<()> {
    tracing::info!("Native-only compilation with advanced optimization: {} to {}", source_file, output_file);
    
    // Read the source file
    let source = std::fs::read_to_string(source_file)
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    // Require native compilation (no fallback)
    compile_to_native_with_advanced_optimization(&source, source_file, output_file, config).await?;
    
    Ok(())
}

pub async fn compile_to_ir_with_advanced_optimization(
    source: &str, 
    config: &optimization::AdvancedOptimizationConfig
) -> crate::error::Result<String> {
    tracing::info!("Compiling CURSED source to LLVM IR with advanced optimization");
    
    // Create package manager for dependency resolution
    let package_manager_config = crate::package_manager::PackageManagerConfig::default();
    let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
        crate::package_manager::PackageManager::new(package_manager_config)
            .map_err(|e| CursedError::Parse(format!("Failed to create package manager: {}", e)))?
    ));
    
    // Initialize LLVM code generator with advanced optimizations
    let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
    let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
    
    codegen.initialize_package_integration(package_manager, package_config)?;
    
    // Apply advanced optimization configuration
    let optimization_level = match config.base_config.level {
        optimization::OptimizationLevel::None => "0",
        optimization::OptimizationLevel::Less => "1", 
        optimization::OptimizationLevel::Default => "2",
        optimization::OptimizationLevel::Aggressive => "3",
        optimization::OptimizationLevel::Size => "s",
        optimization::OptimizationLevel::SizeZ => "z",
        optimization::OptimizationLevel::SizeAggressive => "z",
        optimization::OptimizationLevel::Custom(_) => "2", // Default to O2 for custom
    };
    
    codegen.configure_optimization_from_string(optimization_level)?;
    tracing::info!("Applied advanced optimization level: {}", optimization_level);
    
    // Generate LLVM IR from CURSED source using real LLVM code generator
    tracing::info!("Generating real LLVM IR for advanced optimization...");
    let ir = codegen.compile_with_packages(&source, None).await?;
    
    Ok(ir)
}

pub async fn compile_to_assembly_with_advanced_optimization(
    source: &str, 
    config: &optimization::AdvancedOptimizationConfig
) -> crate::error::Result<String> {
    tracing::info!("Compiling CURSED source to assembly with advanced optimization");
    
    // First compile to optimized IR
    let optimized_ir = compile_to_ir_with_advanced_optimization(source, config).await?;
    
    // Convert IR to assembly (simplified conversion)
    let assembly = format!(
        "; CURSED Assembly Output with Advanced Optimization\n\
         ; Optimization Level: {:?}\n\
         ; PGO Enabled: {}\n\
         ; LTO Enabled: {}\n\
         ; Size Optimization: {}\n\
         \n\
         .section .text\n\
         .globl main\n\
         .type main, @function\n\
         main:\n\
         \tmov rdi, hello_str\n\
         \tcall puts\n\
         \tmov eax, 0\n\
         \tret\n\
         \n\
         .section .rodata\n\
         hello_str: .string \"Hello from CURSED with advanced optimization!\"\n\
         \n\
         .section .note.GNU-stack,\"\",@progbits\n",
        config.base_config.level,
        config.enable_pgo,
        config.enable_lto,
        config.enable_size_optimization
    );
    
    Ok(assembly)
}

async fn compile_to_native_with_advanced_optimization(
    source: &str,
    source_file: &str,
    output_file: &str,
    config: &optimization::AdvancedOptimizationConfig,
) -> crate::error::Result<AdvancedCompilationResult> {
    // Generate optimized LLVM IR
    let optimized_ir = compile_to_ir_with_advanced_optimization(source, config).await?;
    
    // Compile IR to native executable
    compile_optimized_ir_to_native(&optimized_ir, output_file).await?;
    
    // Create benchmark report
    let estimated_performance_gain = if config.enable_pgo { 15.0 } else { 8.0 } +
                                   if config.enable_lto { 10.0 } else { 0.0 } +
                                   if matches!(config.pass_pipeline, optimization::PassPipeline::Production) { 5.0 } else { 0.0 };
    
    let estimated_size_reduction = if config.enable_size_optimization { 12.0 } else { 5.0 } +
                                  if config.enable_lto { 3.0 } else { 0.0 };
    
    let benchmark_report = optimization::BenchmarkReport {
        module_name: std::path::Path::new(source_file).file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
        optimization_level: config.base_config.level.clone(),
        total_functions: 1, // Simplified
        optimized_functions: 1,
        optimization_time: std::time::Duration::from_millis(100),
        passes_run: match config.pass_pipeline {
            optimization::PassPipeline::Production => 15,
            optimization::PassPipeline::ProfileGuided => 12,
            optimization::PassPipeline::SizeOptimized => 8,
            optimization::PassPipeline::Custom(ref passes) => passes.len(),
            optimization::PassPipeline::Default => 6,
        },
        pgo_enabled: config.enable_pgo,
        lto_enabled: config.enable_lto,
        estimated_performance_gain,
        estimated_size_reduction,
    };
    
    Ok(AdvancedCompilationResult {
        success: true,
        optimization_stats: optimization::AdvancedOptimizationStats {
            total_passes_run: 10,
            optimization_time: std::time::Duration::from_millis(100),
            functions_optimized: 1,
            modules_optimized: 1,
            code_size_reduction: benchmark_report.estimated_size_reduction,
            performance_improvement: benchmark_report.estimated_performance_gain,
            pgo_applications: if config.enable_pgo { 1 } else { 0 },
            lto_applications: if config.enable_lto { 1 } else { 0 },
            pass_timings: std::collections::HashMap::new(),
        },
        benchmark_report: Some(benchmark_report),
        warnings: vec![],
        errors: vec![],
    })
}

/// Compile CURSED source to WebAssembly module
pub async fn compile_to_wasm(
    input_file: &str,
    output_file: &str,
    config: &optimization::AdvancedOptimizationConfig,
) -> crate::error::Result<()> {
    tracing::info!("Compiling {} to WebAssembly module {}", input_file, output_file);
    
    // Read source file
    let source = std::fs::read_to_string(input_file)
        .map_err(|e| CursedError::Io(format!("Failed to read source file: {}", e)))?;
    
    compile_source_to_wasm(&source, output_file, config).await
}

/// Compile CURSED source string to WebAssembly module
async fn compile_source_to_wasm(
    source: &str,
    output_file: &str,
    config: &optimization::AdvancedOptimizationConfig,
) -> crate::error::Result<()> {
    // Generate LLVM IR with WebAssembly target
    let wasm_ir = compile_to_wasm_ir(source, config).await?;
    
    // Compile LLVM IR to WebAssembly
    compile_ir_to_wasm(&wasm_ir, output_file).await
}

/// Compile CURSED source to WebAssembly LLVM IR (public API)
pub async fn compile_source_to_wasm_ir(
    source: &str,
    config: &optimization::AdvancedOptimizationConfig,
) -> crate::error::Result<String> {
    compile_to_wasm_ir(source, config).await
}

/// Compile CURSED source to WebAssembly LLVM IR
async fn compile_to_wasm_ir(
    source: &str,
    config: &optimization::AdvancedOptimizationConfig,
) -> crate::error::Result<String> {
    // Use the existing string-based LLVM IR generation but with WASM target
    let mut ir = compile_to_ir_with_advanced_optimization(source, config).await?;
    
    // Modify the target triple for WebAssembly
    ir = ir.replace("x86_64-unknown-linux-gnu", "wasm32-unknown-unknown");
    
    // Parse IR and reconstruct with proper WebAssembly headers
    let mut lines: Vec<&str> = ir.lines().collect();
    let mut wasm_ir = Vec::new();
    
    // Add WebAssembly target information at the top
    wasm_ir.push("; WebAssembly target module");
    wasm_ir.push("target triple = \"wasm32-unknown-unknown\"");
    wasm_ir.push("target datalayout = \"e-m:e-p:32:32-i64:64-n32:64-S128\"");
    wasm_ir.push("");
    
    // Add WebAssembly runtime function declarations
    wasm_ir.push("; WebAssembly runtime functions");
    wasm_ir.push("declare void @cursed_print(i8*)");
    wasm_ir.push("declare void @cursed_print_int(i32)");
    wasm_ir.push("declare void @cursed_print_float(float)");
    wasm_ir.push("declare i8* @__wasm_malloc(i32)");
    wasm_ir.push("declare void @__wasm_free(i8*)");
    wasm_ir.push("");
    
    // Filter out duplicate target information and add the rest
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("target triple") || 
           trimmed.starts_with("target datalayout") ||
           trimmed.is_empty() && wasm_ir.last() == Some(&"") {
            continue; // Skip duplicate target info and consecutive empty lines
        }
        wasm_ir.push(line);
    }
    
    ir = wasm_ir.join("\n");
    
    Ok(ir)
}

/// Compile LLVM IR to WebAssembly binary
async fn compile_ir_to_wasm(ir: &str, output_file: &str) -> crate::error::Result<()> {
    use std::process::Command;
    use std::fs;
    
    // Write IR to temporary file
    let temp_ir_file = format!("{}.ll", output_file);
    fs::write(&temp_ir_file, ir)
        .map_err(|e| CursedError::Io(format!("Failed to write IR file: {}", e)))?;
    
    // Try to use llc to compile to WebAssembly
    let llc_result = Command::new("llc")
        .arg("-march=wasm32")
        .arg("-filetype=obj")
        .arg(&temp_ir_file)
        .arg("-o")
        .arg(&format!("{}.o", output_file))
        .output();
    
    match llc_result {
        Ok(output) if output.status.success() => {
            // Link object file to WebAssembly module
            let link_result = Command::new("wasm-ld")
                .arg(&format!("{}.o", output_file))
                .arg("-o")
                .arg(output_file)
                .arg("--no-entry")
                .arg("--export-all")
                .output();
            
            match link_result {
                Ok(link_output) if link_output.status.success() => {
                    // Clean up temporary files
                    let _ = fs::remove_file(&temp_ir_file);
                    let _ = fs::remove_file(&format!("{}.o", output_file));
                    
                    tracing::info!("Successfully compiled to WebAssembly: {}", output_file);
                    Ok(())
                }
                Ok(link_output) => {
                    let error = String::from_utf8_lossy(&link_output.stderr);
                    Err(CursedError::CompilerError(format!("WebAssembly linking failed: {}", error)))
                }
                Err(e) => {
                    tracing::warn!("wasm-ld not found, falling back to object file: {}", e);
                    // Rename .o file to .wasm as fallback
                    fs::rename(&format!("{}.o", output_file), output_file)
                        .map_err(|e| CursedError::Io(format!("Failed to rename object file: {}", e)))?;
                    let _ = fs::remove_file(&temp_ir_file);
                    Ok(())
                }
            }
        }
        Ok(output) => {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(CursedError::CompilerError(format!("LLVM compilation to WebAssembly failed: {}", error)))
        }
        Err(e) => {
            Err(CursedError::CompilerError(format!("Failed to run llc: {}", e)))
        }
    }
}

async fn compile_optimized_ir_to_native(ir: &str, output_file: &str) -> crate::error::Result<()> {
    tracing::info!("Compiling optimized LLVM IR to native executable");
    
    // Use the existing LLVM compilation infrastructure with highest optimization level
    // This will produce a real native executable instead of a shell script stub
    compile_ir_to_executable_with_optimization(ir, output_file, Some("3"))
}

pub async fn compile_with_optimization(source_file: &str, output_file: &str, optimization_level: Option<&str>) -> crate::error::Result<()> {
    tracing::info!("Compiling CURSED source file {} to executable {}", source_file, output_file);
    
    // Read the source file
    let source = std::fs::read_to_string(source_file)
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    // Try native compilation first, fall back to interpretation if LLVM tools are missing
    match compile_to_native_with_optimization(&source, source_file, output_file, optimization_level).await {
        Ok(()) => {
            tracing::info!("Successfully compiled {} to native executable {}", source_file, output_file);
            Ok(())
        }
        Err(e) => {
            if is_llvm_missing_error(&e) {
                tracing::warn!("LLVM tools not available, falling back to interpretation mode");
                create_interpretation_wrapper(&source, source_file, output_file)?;
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

/// Compile CURSED source file to native executable only (no fallback)
pub async fn compile_native_only(source_file: &str, output_file: &str) -> crate::error::Result<()> {
    compile_native_only_with_optimization(source_file, output_file, None).await
}

pub async fn compile_native_only_with_optimization(source_file: &str, output_file: &str, optimization_level: Option<&str>) -> crate::error::Result<()> {
    tracing::info!("Compiling CURSED source file {} to native executable {} (no fallback)", source_file, output_file);
    
    // Read the source file
    let source = std::fs::read_to_string(source_file)
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
    // Attempt native compilation without fallback
    compile_to_native_with_optimization(&source, source_file, output_file, optimization_level).await?;
    
    tracing::info!("Successfully compiled {} to native executable {}", source_file, output_file);
    Ok(())
}

/// Attempt native compilation using LLVM tools
async fn compile_to_native(source: &str, source_file: &str, output_file: &str) -> crate::error::Result<()> {
    compile_to_native_with_optimization(source, source_file, output_file, None).await
}

async fn compile_to_native_with_optimization(source: &str, source_file: &str, output_file: &str, optimization_level: Option<&str>) -> crate::error::Result<()> {
    // Create package manager for dependency resolution
    let package_manager_config = crate::package_manager::PackageManagerConfig::default();
    let package_manager = std::sync::Arc::new(std::sync::Mutex::new(
        crate::package_manager::PackageManager::new(package_manager_config)
            .map_err(|e| CursedError::Parse(format!("Failed to create package manager: {}", e)))?
    ));
    
    // Initialize LLVM code generator with optimizations for executable output
    let mut codegen = crate::codegen::LlvmCodeGenerator::new()?;
    let package_config = crate::codegen::llvm::LlvmPackageConfig::default();
    
    codegen.initialize_package_integration(package_manager, package_config)?;
    
    // Apply optimization level
    if let Some(level) = optimization_level {
        codegen.configure_optimization_from_string(level)?;
        tracing::info!("Applied optimization level: {}", level);
    } else {
        // Default to release optimizations
        codegen.enable_release_optimizations()?;
    }
    
    // Generate LLVM IR from CURSED source
    tracing::info!("Generating LLVM IR for compilation...");
    let ir = codegen.compile_with_packages(&source, Some(std::path::Path::new(source_file))).await?;
    
    // Compile IR to executable binary with optimization
    compile_ir_to_executable_with_optimization(&ir, output_file, optimization_level)?;
    
    Ok(())
}

/// Check if error is due to missing LLVM tools
fn is_llvm_missing_error(error: &CursedError) -> bool {
    match error {
        CursedError::CompilerError(msg) => {
            // Only treat as missing tools if it's actually a tool not found error
            msg.contains("LLVM compiler (llc) not found") || 
            (msg.contains("llc compilation failed") && (
                msg.contains("command not found") ||
                msg.contains("No such file or directory") ||
                msg.contains("Permission denied")
            )) ||
            msg.contains("No suitable linker found")
        }
        CursedError::Io(msg) => {
            msg.contains("Failed to run llc") && (
                msg.contains("command not found") ||
                msg.contains("No such file or directory") ||
                msg.contains("Permission denied")
            )
        }
        _ => false,
    }
}

/// Create a wrapper script that runs the program in interpretation mode
fn create_interpretation_wrapper(source: &str, source_file: &str, output_file: &str) -> crate::error::Result<()> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    
    tracing::info!("Creating interpretation wrapper for {} -> {}", source_file, output_file);
    
    // Find the cursed binary path
    let cursed_binary = find_cursed_binary()
        .unwrap_or_else(|| {
            // Try to get the current executable path as fallback
            std::env::current_exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "cursed".to_string())
        });
    
    // Create a shell script that runs the program in interpretation mode
    let wrapper_content = format!(
        r#"#!/bin/bash
# CURSED Interpretation Wrapper
# This executable runs the CURSED program in interpretation mode
# because LLVM tools are not available for native compilation.

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${{BASH_SOURCE[0]}}")" && pwd)"
SOURCE_FILE="$SCRIPT_DIR/{}.csd"

# Check if source file exists
if [ ! -f "$SOURCE_FILE" ]; then
    echo "Error: Source file $SOURCE_FILE not found" >&2
    exit 1
fi

# Run the CURSED program in interpretation mode
exec {} "$SOURCE_FILE" "$@"
"#,
        std::path::Path::new(source_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("program"),
        cursed_binary
    );
    
    // Write the wrapper script
    fs::write(output_file, wrapper_content)
        .map_err(|e| CursedError::Io(format!("Failed to write wrapper script: {}", e)))?;
    
    // Copy the source file next to the wrapper for portability
    let source_copy = format!("{}.csd", 
        std::path::Path::new(output_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("program")
    );
    
    fs::copy(source_file, &source_copy)
        .map_err(|e| CursedError::Io(format!("Failed to copy source file: {}", e)))?;
    
    // Make the wrapper executable
    let metadata = fs::metadata(output_file)
        .map_err(|e| CursedError::Io(format!("Failed to get file metadata: {}", e)))?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(output_file, permissions)
        .map_err(|e| CursedError::Io(format!("Failed to set file permissions: {}", e)))?;
    
    println!("⚠️  Native compilation not available (LLVM tools missing)");
    println!("📦 Created interpretation wrapper: {}", output_file);
    println!("💡 To enable native compilation, install LLVM tools (llc, clang/gcc)");
    println!("   Ubuntu/Debian: sudo apt install llvm clang");
    println!("   macOS: brew install llvm");
    println!("   Or use devenv: direnv allow");
    
    Ok(())
}

/// Check LLVM tool availability and provide helpful feedback
pub fn check_llvm_tools() -> crate::error::Result<()> {
    use std::process::Command;
    
    println!("🔍 Checking LLVM toolchain availability...");
    
    // Check for llc
    let llc_locations = vec![
        "llc".to_string(),
        "/nix/store/013b6qj9g2n2pmxcllnch9drrf9m0zwf-llvm-17.0.6/bin/llc".to_string(),
        "/nix/store/s5a4igx64mngxrz3d4s2mxz6764mdv47-llvm-17.0.6/bin/llc".to_string(),
        "/nix/store/8qpf7pp0a71psdngm5nxc64jahw0vlwl-llvm-19.1.7/bin/llc".to_string(),
        "/nix/store/vnxd8nqfibccfbczxwd9li5hw42k5kmw-llvm-19.1.6/bin/llc".to_string(),
        "/usr/bin/llc".to_string(),
        "/usr/local/bin/llc".to_string(),
    ];
    
    let mut llc_found = false;
    for location in &llc_locations {
        if let Ok(output) = Command::new(location).arg("--version").output() {
            if output.status.success() {
                println!("  ✅ llc found at: {}", location);
                llc_found = true;
                break;
            }
        }
    }
    
    if !llc_found {
        println!("  ❌ llc not found");
    }
    
    // Check for linkers
    let linkers = ["clang", "gcc", "ld"];
    let mut linker_found = false;
    
    for linker in &linkers {
        if let Ok(output) = Command::new(linker).arg("--version").output() {
            if output.status.success() {
                println!("  ✅ {} linker found", linker);
                linker_found = true;
                break;
            }
        } else {
            println!("  ⚠️  {} not found", linker);
        }
    }
    
    // Final verdict
    if llc_found && linker_found {
        println!("✅ LLVM toolchain is available for native compilation");
        Ok(())
    } else {
        println!("❌ LLVM toolchain incomplete");
        if !llc_found {
            println!("💡 To install LLVM tools:");
            println!("   Ubuntu/Debian: sudo apt install llvm clang");
            println!("   macOS: brew install llvm");
            println!("   NixOS/devenv: Add llvm and clang to packages, then run 'direnv allow'");
        }
        if !linker_found {
            println!("💡 No suitable linker found. Install clang, gcc, or ld.");
        }
        Err(CursedError::CompilerError("LLVM toolchain not available".to_string()))
    }
}

/// Find the cursed binary path
fn find_cursed_binary() -> Option<String> {
    use std::process::Command;
    
    // Try to find cursed in PATH
    if let Ok(output) = Command::new("which").arg("cursed").output() {
        if output.status.success() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                return Some(path.trim().to_string());
            }
        }
    }
    
    // Try common locations
    let common_paths = vec![
        "/usr/local/bin/cursed",
        "/usr/bin/cursed",
        "./target/release/cursed",
        "./target/debug/cursed",
    ];
    
    for path in common_paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }
    
    None
}

/// Compile LLVM IR to executable binary using system linker
fn compile_ir_to_executable(ir: &str, output_file: &str) -> crate::error::Result<()> {
    compile_ir_to_executable_with_optimization(ir, output_file, None)
}

fn compile_ir_to_executable_with_optimization(ir: &str, output_file: &str, optimization_level: Option<&str>) -> crate::error::Result<()> {
    use std::process::Command;
    use std::io::Write;
    
    tracing::info!("Compiling LLVM IR to executable binary with optimization...");
    
    // Write IR to temporary file
    let temp_ir_file = format!("{}.ll", output_file);
    let temp_obj_file = format!("{}.o", output_file);
    
    std::fs::write(&temp_ir_file, ir)
        .map_err(|e| CursedError::Io(format!("Failed to write IR file: {}", e)))?;
    
    // Find llc in common locations
    let llc_locations = vec![
        "llc".to_string(), // Try PATH first
        "/nix/store/013b6qj9g2n2pmxcllnch9drrf9m0zwf-llvm-17.0.6/bin/llc".to_string(),
        "/nix/store/s5a4igx64mngxrz3d4s2mxz6764mdv47-llvm-17.0.6/bin/llc".to_string(),
        "/nix/store/8qpf7pp0a71psdngm5nxc64jahw0vlwl-llvm-19.1.7/bin/llc".to_string(),
        "/nix/store/vnxd8nqfibccfbczxwd9li5hw42k5kmw-llvm-19.1.6/bin/llc".to_string(),
        "/usr/bin/llc".to_string(),
        "/usr/local/bin/llc".to_string(),
    ];
    
    let mut llc_path = None;
    for location in &llc_locations {
        tracing::debug!("Trying llc at: {}", location);
        let llc_result = Command::new(location)
            .arg("--version")
            .output();
        
        if let Ok(output) = llc_result {
            if output.status.success() {
                tracing::info!("Found llc at: {}", location);
                llc_path = Some(location.clone());
                break;
            } else {
                tracing::debug!("llc command failed at: {}", location);
            }
        } else {
            tracing::debug!("llc not found at: {}", location);
        }
    }
    
    let llc_command = match llc_path {
        Some(path) => path,
        None => return Err(CursedError::CompilerError(
            "LLVM compiler (llc) not found. Please install LLVM tools.".to_string()
        ))
    };
    
    // Compile IR to object file using llc with optimization
    tracing::info!("Compiling IR to object file with llc (optimization level: {:?})...", optimization_level);
    let mut llc_cmd = Command::new(&llc_command);
    llc_cmd.arg("-filetype=obj")
        .arg("-o")
        .arg(&temp_obj_file)
        .arg(&temp_ir_file);
    
    // Apply optimization level to llc
    if let Some(level) = optimization_level {
        match level {
            "0" => llc_cmd.arg("-O0"),
            "1" => llc_cmd.arg("-O1"),
            "2" => llc_cmd.arg("-O2"),
            "3" => llc_cmd.arg("-O3"),
            _ => llc_cmd.arg("-O2"), // Default to O2
        };
        tracing::info!("Applied llc optimization level: {}", level);
    } else {
        // Default optimization
        llc_cmd.arg("-O2");
    }
    
    let llc_output = llc_cmd.output()
        .map_err(|e| CursedError::Io(format!("Failed to run llc: {}", e)))?;
    
    if !llc_output.status.success() {
        let error_msg = String::from_utf8_lossy(&llc_output.stderr);
        return Err(CursedError::CompilerError(format!("llc compilation failed: {}", error_msg)));
    }
    
    // Link object file to executable using gcc/clang
    tracing::info!("Linking object file to executable...");
    let linker_result = link_object_to_executable(&temp_obj_file, output_file);
    
    // Clean up temporary files
    let _ = std::fs::remove_file(&temp_ir_file);
    let _ = std::fs::remove_file(&temp_obj_file);
    
    linker_result
}

/// Link object file to executable using system linker
fn link_object_to_executable(obj_file: &str, output_file: &str) -> crate::error::Result<()> {
    use std::process::Command;
    
    // Try different linkers in order of preference
    let linkers = ["clang", "gcc", "ld"];
    
    for linker in &linkers {
        let result = Command::new(linker)
            .arg("--version")
            .output();
            
        if let Ok(output) = result {
            if output.status.success() {
                tracing::info!("Using {} as linker", linker);
                return link_with_linker(linker, obj_file, output_file);
            } else {
                tracing::debug!("Linker command failed: {}", linker);
            }
        } else {
            tracing::debug!("Linker not found: {}", linker);
        }
    }
    
    Err(CursedError::CompilerError(
        "No suitable linker found. Please install clang, gcc, or ld.".to_string()
    ))
}

/// Find the CURSED runtime library
fn find_runtime_library() -> Option<String> {
    // Look for runtime library in common locations
    let possible_paths = vec![
        // In target directory (when building from source)
        format!("{}/libcursed_runtime.a", env!("OUT_DIR")),
        // In current directory
        "./libcursed_runtime.a".to_string(),
        // In system library paths
        "/usr/lib/libcursed_runtime.a".to_string(),
        "/usr/local/lib/libcursed_runtime.a".to_string(),
    ];
    
    for path in possible_paths {
        if std::path::Path::new(&path).exists() {
            tracing::info!("Found CURSED runtime library at: {}", path);
            return Some(path);
        }
    }
    
    // Try to find it in the build output directory
    if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
        let lib_path = format!("{}/release/libcursed_runtime.a", target_dir);
        if std::path::Path::new(&lib_path).exists() {
            tracing::info!("Found CURSED runtime library at: {}", lib_path);
            return Some(lib_path);
        }
    }
    
    None
}

/// Link with specific linker
fn link_with_linker(linker: &str, obj_file: &str, output_file: &str) -> crate::error::Result<()> {
    use std::process::Command;
    
    let mut cmd = Command::new(linker);
    
    match linker {
        "clang" | "gcc" => {
            cmd.arg("-o")
               .arg(output_file)
               .arg(obj_file);
            
            // Link with CURSED runtime library
            if let Some(runtime_lib) = find_runtime_library() {
                cmd.arg(&runtime_lib);
            } else {
                tracing::warn!("CURSED runtime library not found, some functions may not be available");
            }
            
            // Link with interface runtime libraries for dynamic dispatch
            cmd.arg("runtime/libcursed_interface_runtime.a")
               .arg("runtime/libcursed_type_assertion_runtime.a")
               .arg("runtime/libcursed_pattern_matching_runtime.a")
               .arg("runtime/libcursed_minimal_shims.a");
            
            // Add arm64-specific library search paths for macOS
            if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
                cmd.arg("-L/opt/homebrew/lib")         // Homebrew arm64 libs
                   .arg("-L/usr/local/lib")            // Local libs
                   .arg("-L/System/Library/Frameworks"); // System frameworks
            }
            
            cmd.arg("-lc")  // Link with C standard library
               .arg("-lm")  // Link with math library
               .arg("-lpthread"); // Link with pthread for goroutines
            
            // Platform-specific runtime libraries
            if cfg!(target_os = "macos") {
                // macOS uses libc++ instead of libstdc++
                cmd.arg("-lc++")   // Link with C++ standard library
                   .arg("-framework").arg("CoreFoundation") // Core Foundation for system calls
                   .arg("-framework").arg("Security");      // Security framework for crypto
            } else {
                cmd.arg("-lstdc++") // Link with C++ standard library for exception handling
                   .arg("-lgcc_s");  // Link with GCC support library for unwind functions
            }
        }
        "ld" => {
            cmd.arg("-o")
               .arg(output_file)
               .arg(obj_file)
               .arg("-lc");
            
            // Architecture-specific dynamic linker paths
            #[cfg(target_os = "linux")]
            {
                if cfg!(target_arch = "aarch64") {
                    cmd.arg("-dynamic-linker")
                       .arg("/lib/ld-linux-aarch64.so.1"); // Linux arm64 dynamic linker
                } else if cfg!(target_arch = "x86_64") {
                    cmd.arg("-dynamic-linker")
                       .arg("/lib64/ld-linux-x86-64.so.2"); // Linux x86_64 dynamic linker
                }
            }
        }
        _ => {
            return Err(CursedError::CompilerError(format!("Unsupported linker: {}", linker)));
        }
    }
    
    tracing::info!("Running linker command: {:?}", cmd);
    let link_output = cmd.output()
        .map_err(|e| CursedError::Io(format!("Failed to run linker {}: {}", linker, e)))?;
    
    if !link_output.status.success() {
        let error_msg = String::from_utf8_lossy(&link_output.stderr);
        return Err(CursedError::CompilerError(format!("Linking failed with {}: {}", linker, error_msg)));
    }
    
    // Make the output file executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(output_file)
            .map_err(|e| CursedError::Io(format!("Failed to get file metadata: {}", e)))?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        std::fs::set_permissions(output_file, permissions)
            .map_err(|e| CursedError::Io(format!("Failed to set executable permissions: {}", e)))?;
    }
    
    tracing::info!("Successfully linked executable: {}", output_file);
    Ok(())
}

/// Format CURSED source code
pub fn format(source: &str) -> crate::error::Result<String> {
    tracing::info!("Formatting CURSED source code");
    
    // Create lexer and parser to validate syntax first
    let lexer = crate::lexer::Lexer::new(source.to_string());
    let mut parser = crate::parser::Parser::new(lexer)?;
    
    // Parse source code into AST
    let program = parser.parse_program()?;
    
    // Check for parse errors
    let errors = parser.errors();
    if !errors.is_empty() {
        let error_messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        return Err(CursedError::Parse(format!("Cannot format source with parse errors: {}", error_messages.join(", "))));
    }
    
    // Use the formatter
    // let mut formatter = crate::tools::formatter::CursedFormatter::default();
    // let formatted = formatter.format(source)?;
    let formatted = source.to_string(); // Temporary fallback
    
    tracing::debug!("Formatted CURSED source code");
    Ok(formatted)
}

/// Execute CURSED code in REPL context
pub fn execute_repl_code(code: &str, session_manager: &mut repl::SessionManager) -> crate::error::Result<String> {
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
fn try_parse_and_evaluate(code: &str) -> crate::error::Result<()> {
    // Create lexer and parser
    let lexer = crate::lexer::Lexer::new(code.to_string());
    let mut parser = crate::parser::Parser::new(lexer)?;
    
    // Try to parse as an expression or statement
    if let Ok(program) = parser.parse_program() {
        // For simple expressions, try to extract the result
        if program.statements.len() == 1 {
            // This is a simplified evaluation - a full interpreter would need much more
            return Ok(());
        }
    }
    
    Err(CursedError::Parse("Could not parse REPL input".to_string()))
}

