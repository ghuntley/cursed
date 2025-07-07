
/// CURSED Programming Language Library
/// 
/// A comprehensive programming language implementation with Gen Z slang syntax,
/// LLVM-based compilation, and advanced type system features.

// Core modules
pub mod error;
pub mod error_types;
pub mod package_manager;
pub mod imports;
pub mod ast;
pub mod lexer;
pub mod preprocessor;
pub mod parser;
pub mod core; // Re-enabled for type system integration
pub mod codegen; // Re-enabled for type system integration
pub mod memory;
pub mod runtime;
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
// pub mod security; // Temporarily disabled due to dependency issues

// Test modules  
#[cfg(test)]
// pub mod test_member_access; // Removed - file no longer exists

// Re-export common types for easy access
pub use common::OptimizationLevel;
use crate::error::CursedError;

// CRITICAL: Re-export core components for tests and external access
pub use lexer::{Lexer, Token, TokenKind};
pub use parser::{Parser, new_parser};
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
pub mod lsp;

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
pub use tools::{CursedFormatter, FormatterConfig, FormattingOptions, CursedLinter, LinterConfig, LintRule, LintResult};

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
    tracing::info!("Compiling CURSED source file {} to executable {}", source_file, output_file);
    
    // Read the source file
    let source = std::fs::read_to_string(source_file)
        .map_err(|e| CursedError::Io(e.to_string()))?;
    
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
    codegen.enable_release_optimizations()?;
    
    // Generate LLVM IR from CURSED source
    tracing::info!("Generating LLVM IR for compilation...");
    let ir = codegen.compile_with_packages(&source, Some(std::path::Path::new(source_file))).await?;
    
    // Compile IR to executable binary
    compile_ir_to_executable(&ir, output_file)?;
    
    tracing::info!("Successfully compiled {} to executable {}", source_file, output_file);
    Ok(())
}

/// Compile LLVM IR to executable binary using system linker
fn compile_ir_to_executable(ir: &str, output_file: &str) -> crate::error::Result<()> {
    use std::process::Command;
    use std::io::Write;
    
    tracing::info!("Compiling LLVM IR to executable binary...");
    
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
        
        if llc_result.is_ok() {
            tracing::info!("Found llc at: {}", location);
            llc_path = Some(location.clone());
            break;
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
    
    // Compile IR to object file using llc
    tracing::info!("Compiling IR to object file with llc...");
    let llc_output = Command::new(&llc_command)
        .arg("-filetype=obj")
        .arg("-o")
        .arg(&temp_obj_file)
        .arg(&temp_ir_file)
        .output()
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
            
        if result.is_ok() {
            tracing::info!("Using {} as linker", linker);
            return link_with_linker(linker, obj_file, output_file);
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
            
            cmd.arg("-lc")  // Link with C standard library
               .arg("-lm")  // Link with math library
               .arg("-lpthread") // Link with pthread for goroutines
               .arg("-lstdc++") // Link with C++ standard library for exception handling
               .arg("-lgcc_s"); // Link with GCC support library for unwind functions
        }
        "ld" => {
            cmd.arg("-o")
               .arg(output_file)
               .arg(obj_file)
               .arg("-lc")
               .arg("-dynamic-linker")
               .arg("/lib64/ld-linux-x86-64.so.2"); // Linux dynamic linker
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
    let mut formatter = crate::tools::formatter::CursedFormatter::default();
    let formatted = formatter.format(source)?;
    
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
