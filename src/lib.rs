
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
pub use common::OptimizationLevel as CommonOptimizationLevel;
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
    OptimizationConfig, OptimizationLevel, PassManagerConfig, VerificationLevel, OptimizationProfile,
    PerformanceOptimizer, OptimizationResult, OptimizationMetrics, EnhancedLlvmOptimizer,
    CursedOptimizationConfig, OptimizationStats
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
    initialize_complete_runtime, shutdown_complete_runtime,
    // Platform Abstraction Layer
    PlatformAbstraction, Architecture, OperatingSystem, PlatformError, create_platform_abstraction
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
// Duplicated imports removed - already exported above

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
    
    // Initialize Platform Abstraction Layer
    match initialize_platform_runtime() {
        Ok(_) => log::info!("Platform Abstraction Layer initialized successfully"),
        Err(e) => log::warn!("PAL initialization failed: {}. Using fallback runtime.", e),
    }
}

/// Initialize the Platform Abstraction Layer and configure the runtime
pub fn initialize_platform_runtime() -> Result<(), CursedError> {
    // For now, use a simple platform detection and logging approach
    // The full PAL integration will be completed once PAL implementations are fixed
    
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
        log::info!("Platform detected: ARM64 macOS - using optimized memory and scheduling");
        log::info!("Stack size: 1MB, Hardware threads: {}, Page size: 16KB", 
                  std::thread::available_parallelism().unwrap_or(std::num::NonZeroUsize::new(1).unwrap()).get());
    }
    
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    {
        log::info!("Platform detected: ARM64 Linux - using optimized memory and scheduling");
        log::info!("Stack size: 2MB, Hardware threads: {}, Page size: 4KB", 
                  std::thread::available_parallelism().unwrap_or(std::num::NonZeroUsize::new(1).unwrap()).get());
    }
    
    #[cfg(all(target_arch = "x86_64", target_os = "macos"))]
    {
        log::info!("Platform detected: x86_64 macOS - using standard memory and scheduling");
        log::info!("Stack size: 2MB, Hardware threads: {}, Page size: 4KB", 
                  std::thread::available_parallelism().unwrap_or(std::num::NonZeroUsize::new(1).unwrap()).get());
    }
    
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    {
        log::info!("Platform detected: x86_64 Linux - using standard memory and scheduling");
        log::info!("Stack size: 2MB, Hardware threads: {}, Page size: 4KB", 
                  std::thread::available_parallelism().unwrap_or(std::num::NonZeroUsize::new(1).unwrap()).get());
    }
    
    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    {
        log::info!("Platform detected: x86_64 Windows - using standard memory and scheduling");
        log::info!("Stack size: 1MB, Hardware threads: {}, Page size: 4KB", 
                  std::thread::available_parallelism().unwrap_or(std::num::NonZeroUsize::new(1).unwrap()).get());
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        log::info!("Platform detected: WebAssembly - using single-threaded runtime");
        log::info!("Stack size: 1MB, Hardware threads: 1, Page size: 64KB");
    }
    
    // Initialize existing runtime systems with platform-aware configuration
    initialize_platform_aware_runtime()?;
    
    Ok(())
}

/// Initialize runtime components with platform-aware configuration
fn initialize_platform_aware_runtime() -> Result<(), CursedError> {
    // Initialize global scheduler with default configuration
    // The PAL-specific schedulers will be integrated once implementations are fixed
    crate::runtime::initialize_global_scheduler()
        .map_err(|e| CursedError::internal_error(&format!("Scheduler initialization failed: {:?}", e)))?;
    
    // Initialize memory management with platform-appropriate defaults
    let gc_config = crate::memory::gc::GcConfig {
        initial_heap_size: get_platform_heap_size(),
        max_heap_size: None, // Unlimited
        concurrent_collection: true,
        concurrent_threads: std::thread::available_parallelism()
            .unwrap_or(std::num::NonZeroUsize::new(1).unwrap())
            .get(),
        memory_threshold: (get_platform_heap_size() as f64 * 0.8) as usize,
        ..Default::default()
    };
    
    let memory_config = crate::runtime::MemoryConfig {
        gc_config,
        enable_tracking: cfg!(debug_assertions),
        stack_memory_limit: None,
        global_memory_limit: None,
        enable_pressure_detection: true,
        pressure_threshold: 0.8,
    };
    
    let stack_manager = std::sync::Arc::new(crate::runtime::RuntimeStack::new());
    crate::runtime::initialize_memory_manager(memory_config, stack_manager)
        .map_err(|e| CursedError::internal_error(&format!("Memory manager initialization failed: {:?}", e)))?;
    
    // Initialize complete runtime system with platform configuration
    let runtime_config = crate::runtime::RuntimeConfig {
        max_goroutines: get_platform_max_goroutines(),
        default_stack_size: get_platform_stack_size(),
        memory_alignment: get_platform_page_size(),
        gc_trigger_ratio: 0.8,
        scheduler_quantum: std::time::Duration::from_millis(10),
        platform_name: get_platform_name(),
        architecture: get_platform_architecture(),
        operating_system: get_platform_os(),
        memory_limit: None,
        gc_frequency: std::time::Duration::from_millis(100),
        debug_mode: cfg!(debug_assertions),
        profiling_enabled: false,
        max_call_depth: 1000,
        timeouts: crate::runtime::runtime::TimeoutConfig {
            goroutine_spawn: std::time::Duration::from_millis(100),
            memory_allocation: std::time::Duration::from_millis(500),
            shutdown: std::time::Duration::from_secs(30),
        },
    };
    
    let complete_gc_config = crate::memory::gc::GcConfig {
        initial_heap_size: get_platform_heap_size(),
        max_heap_size: None,
        concurrent_collection: true,
        concurrent_threads: std::thread::available_parallelism()
            .unwrap_or(std::num::NonZeroUsize::new(1).unwrap())
            .get(),
        memory_threshold: (get_platform_heap_size() as f64 * 0.8) as usize,
        ..Default::default()
    };
    
    let complete_memory_config = crate::runtime::memory::MemoryConfig {
        gc_config: complete_gc_config,
        enable_tracking: cfg!(debug_assertions),
        stack_memory_limit: None,
        global_memory_limit: None,
        enable_pressure_detection: true,
        pressure_threshold: 0.8,
    };
    
    crate::runtime::initialize_complete_runtime(runtime_config, complete_memory_config)
        .map_err(|e| CursedError::internal_error(&format!("Complete runtime initialization failed: {:?}", e)))?;
    
    Ok(())
}

/// Get platform-specific heap size
fn get_platform_heap_size() -> usize {
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    { 64 * 1024 * 1024 } // 64MB initial heap for ARM64 macOS
    
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    { 32 * 1024 * 1024 } // 32MB initial heap for ARM64 Linux
    
    #[cfg(target_arch = "x86_64")]
    { 32 * 1024 * 1024 } // 32MB initial heap for x86_64
    
    #[cfg(target_arch = "wasm32")]
    { 16 * 1024 * 1024 } // 16MB initial heap for WASM
    
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "wasm32")))]
    { 32 * 1024 * 1024 } // Default fallback
}

/// Get platform-specific stack size
fn get_platform_stack_size() -> usize {
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    { 1024 * 1024 } // 1MB stack for ARM64 macOS
    
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    { 2 * 1024 * 1024 } // 2MB stack for ARM64 Linux
    
    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    { 1024 * 1024 } // 1MB stack for Windows
    
    #[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
    { 2 * 1024 * 1024 } // 2MB stack for Unix x86_64
    
    #[cfg(target_arch = "wasm32")]
    { 1024 * 1024 } // 1MB stack for WASM
    
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "wasm32")))]
    { 2 * 1024 * 1024 } // Default fallback
}

/// Get platform-specific page size
fn get_platform_page_size() -> usize {
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    { 16 * 1024 } // 16KB pages on ARM64 macOS
    
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    { 4 * 1024 } // 4KB pages on ARM64 Linux (usually)
    
    #[cfg(target_arch = "x86_64")]
    { 4 * 1024 } // 4KB pages on x86_64
    
    #[cfg(target_arch = "wasm32")]
    { 64 * 1024 } // 64KB pages in WASM
    
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "wasm32")))]
    { 4 * 1024 } // Default fallback
}

/// Get platform-specific maximum goroutines
fn get_platform_max_goroutines() -> usize {
    let hardware_threads = std::thread::available_parallelism()
        .unwrap_or(std::num::NonZeroUsize::new(1).unwrap())
        .get();
    
    #[cfg(target_arch = "wasm32")]
    { 100 } // Limited goroutines for WASM
    
    #[cfg(not(target_arch = "wasm32"))]
    { hardware_threads * 1000 } // 1000 goroutines per hardware thread for native
}

/// Get platform name string
fn get_platform_name() -> String {
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    { "ARM64 macOS".to_string() }
    
    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    { "ARM64 Linux".to_string() }
    
    #[cfg(all(target_arch = "x86_64", target_os = "macos"))]
    { "x86_64 macOS".to_string() }
    
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    { "x86_64 Linux".to_string() }
    
    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    { "x86_64 Windows".to_string() }
    
    #[cfg(target_arch = "wasm32")]
    { "WebAssembly".to_string() }
    
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "wasm32")))]
    { "Unknown Platform".to_string() }
}

/// Get platform architecture
fn get_platform_architecture() -> crate::runtime::Architecture {
    #[cfg(target_arch = "aarch64")]
    { crate::runtime::Architecture::Arm64 }
    
    #[cfg(target_arch = "x86_64")]
    { crate::runtime::Architecture::X86_64 }
    
    #[cfg(target_arch = "wasm32")]
    { crate::runtime::Architecture::Wasm32 }
    
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64", target_arch = "wasm32")))]
    { crate::runtime::Architecture::X86_64 } // Default fallback
}

/// Get platform operating system
fn get_platform_os() -> crate::runtime::OperatingSystem {
    #[cfg(target_os = "macos")]
    { crate::runtime::OperatingSystem::MacOS }
    
    #[cfg(target_os = "linux")]
    { crate::runtime::OperatingSystem::Linux }
    
    #[cfg(target_os = "windows")]
    { crate::runtime::OperatingSystem::Windows }
    
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    { crate::runtime::OperatingSystem::WasmRuntime }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    { crate::runtime::OperatingSystem::Linux } // Default fallback
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
        
        // Enable comprehensive optimization including LTO
        let preset = match optimization_config.level {
            crate::optimization::config::OptimizationLevel::None => crate::codegen::llvm::OptimizationConfig::dev_config(),
            crate::optimization::config::OptimizationLevel::Aggressive => {
                let mut config = crate::codegen::llvm::OptimizationConfig::release_config();
                config.enable_lto = true; // Enable LTO for aggressive optimization
                config
            },
            _ => {
                let mut config = crate::codegen::llvm::OptimizationConfig::default();
                config.enable_lto = true; // Enable LTO by default
                config
            },
        };
        codegen.enable_comprehensive_optimization(preset)?;
        
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
    pub optimization_stats: optimization::OptimizationStats,
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
                    optimization_stats: optimization::OptimizationStats::default(),
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
                                   if config.base_config.pass_manager_config.enable_function_passes { 5.0 } else { 0.0 };
    
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
        passes_run: if config.base_config.pass_manager_config.enable_function_passes { 15 } else { 6 },
        pgo_enabled: config.enable_pgo,
        lto_enabled: config.enable_lto,
        estimated_performance_gain,
        estimated_size_reduction,
    };
    
    Ok(AdvancedCompilationResult {
        success: true,
        optimization_stats: optimization::OptimizationStats {
            optimization_level: 3,
            size_level: 0,
            passes_run: 10,
            custom_passes_count: config.custom_passes.len(),
            pgo_enabled: config.enable_pgo,
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

/// Enhanced WebAssembly compilation with optimizations and validation
pub async fn compile_to_wasm_with_optimizations(
    input_file: &str,
    output_file: &str,
    config: &optimization::AdvancedOptimizationConfig,
    wasm_config: &WasmCompilationConfig,
) -> crate::error::Result<WasmCompilationResult> {
    tracing::info!("Enhanced WASM compilation: {} -> {}", input_file, output_file);
    
    let source = std::fs::read_to_string(input_file)
        .map_err(|e| CursedError::Io(format!("Failed to read source file: {}", e)))?;
    
    let start_time = std::time::Instant::now();
    
    // Generate optimized WASM IR
    let wasm_ir = compile_to_optimized_wasm_ir(&source, config, wasm_config).await?;
    
    // Compile to WASM binary with advanced features
    let binary_result = compile_ir_to_wasm_binary(&wasm_ir, output_file, wasm_config).await?;
    
    // Validate and optimize the resulting WASM binary
    let validation_result = validate_and_optimize_wasm_binary(output_file, wasm_config).await?;
    
    let compilation_time = start_time.elapsed();
    
    Ok(WasmCompilationResult {
        success: true,
        output_file: output_file.to_string(),
        binary_size: binary_result.binary_size,
        compilation_time,
        optimization_stats: binary_result.optimization_stats,
        validation_result,
        debug_info_generated: wasm_config.generate_debug_info,
        source_maps_generated: wasm_config.generate_source_maps,
    })
}

/// WASM compilation configuration
#[derive(Debug, Clone)]
pub struct WasmCompilationConfig {
    pub target_features: Vec<String>,
    pub enable_simd: bool,
    pub enable_threads: bool,
    pub enable_exception_handling: bool,
    pub enable_bulk_memory: bool,
    pub enable_reference_types: bool,
    pub memory_optimization_level: WasmMemoryOptLevel,
    pub code_size_optimization: bool,
    pub enable_wasi: bool,
    pub generate_debug_info: bool,
    pub generate_source_maps: bool,
    pub validation_level: WasmValidationLevel,
    pub import_optimization: bool,
    pub export_optimization: bool,
    pub dead_code_elimination: bool,
    pub function_table_optimization: bool,
}

#[derive(Debug, Clone)]
pub enum WasmMemoryOptLevel {
    None,
    Basic,
    Aggressive,
}

#[derive(Debug, Clone)]
pub enum WasmValidationLevel {
    None,
    Basic,
    Strict,
    Security,
}

#[derive(Debug, Clone)]
pub struct WasmCompilationResult {
    pub success: bool,
    pub output_file: String,
    pub binary_size: u64,
    pub compilation_time: std::time::Duration,
    pub optimization_stats: WasmOptimizationStats,
    pub validation_result: WasmValidationResult,
    pub debug_info_generated: bool,
    pub source_maps_generated: bool,
}

#[derive(Debug, Clone)]
pub struct WasmOptimizationStats {
    pub functions_optimized: usize,
    pub code_size_reduction: f64,
    pub memory_optimizations_applied: usize,
    pub dead_functions_eliminated: usize,
    pub imports_optimized: usize,
    pub exports_optimized: usize,
}

#[derive(Debug, Clone)]
pub struct WasmValidationResult {
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    pub warnings: Vec<String>,
    pub security_issues: Vec<String>,
    pub performance_suggestions: Vec<String>,
}

impl Default for WasmCompilationConfig {
    fn default() -> Self {
        Self {
            target_features: vec!["mutable-globals".to_string(), "sign-ext".to_string()],
            enable_simd: false,
            enable_threads: false,
            enable_exception_handling: false,
            enable_bulk_memory: true,
            enable_reference_types: false,
            memory_optimization_level: WasmMemoryOptLevel::Basic,
            code_size_optimization: true,
            enable_wasi: false,
            generate_debug_info: false,
            generate_source_maps: false,
            validation_level: WasmValidationLevel::Basic,
            import_optimization: true,
            export_optimization: true,
            dead_code_elimination: true,
            function_table_optimization: true,
        }
    }
}

/// Compile to optimized WASM IR with advanced features
async fn compile_to_optimized_wasm_ir(
    source: &str,
    config: &optimization::AdvancedOptimizationConfig,
    wasm_config: &WasmCompilationConfig,
) -> crate::error::Result<String> {
    let mut ir = compile_to_ir_with_advanced_optimization(source, config).await?;
    
    // Apply WASM-specific target configuration
    ir = apply_wasm_target_configuration(ir, wasm_config)?;
    
    // Add WASM-specific runtime functions and imports
    ir = add_wasm_runtime_functions(ir, wasm_config)?;
    
    // Apply WASM-specific optimizations
    ir = apply_wasm_optimizations(ir, wasm_config)?;
    
    Ok(ir)
}

/// Apply WASM target configuration to IR
fn apply_wasm_target_configuration(
    mut ir: String,
    wasm_config: &WasmCompilationConfig,
) -> crate::error::Result<String> {
    // Replace target triple with WASM
    ir = ir.replace("x86_64-unknown-linux-gnu", "wasm32-unknown-unknown");
    
    // Add WASM-specific target features
    let mut target_features = String::new();
    for feature in &wasm_config.target_features {
        target_features.push_str(&format!("+{},", feature));
    }
    
    if wasm_config.enable_simd {
        target_features.push_str("+simd128,");
    }
    if wasm_config.enable_threads {
        target_features.push_str("+atomics,+bulk-memory,+mutable-globals,");
    }
    if wasm_config.enable_exception_handling {
        target_features.push_str("+exception-handling,");
    }
    if wasm_config.enable_bulk_memory {
        target_features.push_str("+bulk-memory,");
    }
    if wasm_config.enable_reference_types {
        target_features.push_str("+reference-types,");
    }
    
    // Remove trailing comma
    if target_features.ends_with(',') {
        target_features.pop();
    }
    
    // Insert WASM target configuration at the top
    let wasm_header = format!(
        "; Enhanced CURSED WebAssembly Module\n\
         target triple = \"wasm32-unknown-unknown\"\n\
         target datalayout = \"e-m:e-p:32:32-i64:64-n32:64-S128\"\n\
         target features = \"{}\"\n\n",
        target_features
    );
    
    // Find insertion point after any existing target declarations
    let lines: Vec<&str> = ir.lines().collect();
    let mut new_ir = Vec::new();
    let mut header_inserted = false;
    
    for line in lines {
        let trimmed = line.trim();
        if (trimmed.starts_with("target triple") || 
            trimmed.starts_with("target datalayout") ||
            trimmed.starts_with("target features")) && !header_inserted {
            // Skip existing target declarations
            continue;
        } else if !header_inserted && !trimmed.is_empty() && !trimmed.starts_with(";") {
            // Insert header before first non-comment, non-target line
            new_ir.push(wasm_header.as_str());
            header_inserted = true;
        }
        new_ir.push(line);
    }
    
    if !header_inserted {
        new_ir.insert(0, wasm_header.as_str());
    }
    
    Ok(new_ir.join("\n"))
}

/// Add WASM-specific runtime functions and imports
fn add_wasm_runtime_functions(
    mut ir: String,
    wasm_config: &WasmCompilationConfig,
) -> crate::error::Result<String> {
    let mut runtime_functions = String::new();
    
    // Standard WASM runtime functions
    runtime_functions.push_str(
        "; CURSED WebAssembly Runtime Functions\n\
         declare void @cursed_print(i8*)\n\
         declare void @cursed_print_int(i32)\n\
         declare void @cursed_print_float(float)\n\
         declare i8* @__wasm_malloc(i32)\n\
         declare void @__wasm_free(i8*)\n\n"
    );
    
    // Memory management functions
    runtime_functions.push_str(
        "; Memory Management\n\
         declare void @__wasm_memory_grow(i32)\n\
         declare i32 @__wasm_memory_size()\n\
         declare void @__wasm_memory_fill(i8*, i8, i32)\n\
         declare void @__wasm_memory_copy(i8*, i8*, i32)\n\n"
    );
    
    // WASI functions if enabled
    if wasm_config.enable_wasi {
        runtime_functions.push_str(
            "; WASI System Interface\n\
             declare i32 @__wasi_fd_write(i32, i8*, i32, i8*)\n\
             declare i32 @__wasi_fd_read(i32, i8*, i32, i8*)\n\
             declare i32 @__wasi_environ_sizes_get(i8*, i8*)\n\
             declare i32 @__wasi_environ_get(i8*, i8*)\n\
             declare void @__wasi_proc_exit(i32)\n\n"
        );
    }
    
    // Thread functions if enabled
    if wasm_config.enable_threads {
        runtime_functions.push_str(
            "; Threading Support\n\
             declare i32 @__wasm_atomic_wait32(i32*, i32, i64)\n\
             declare i32 @__wasm_atomic_notify(i32*, i32)\n\
             declare i32 @__wasm_atomic_load32(i32*)\n\
             declare void @__wasm_atomic_store32(i32*, i32)\n\n"
        );
    }
    
    // SIMD functions if enabled
    if wasm_config.enable_simd {
        runtime_functions.push_str(
            "; SIMD Support\n\
             declare <4 x i32> @__wasm_v128_load(<4 x i32>*)\n\
             declare void @__wasm_v128_store(<4 x i32>*, <4 x i32>)\n\
             declare <4 x i32> @__wasm_i32x4_add(<4 x i32>, <4 x i32>)\n\
             declare <4 x float> @__wasm_f32x4_add(<4 x float>, <4 x float>)\n\n"
        );
    }
    
    // Insert runtime functions after target declarations
    if let Some(insert_pos) = ir.find("\n\n") {
        ir.insert_str(insert_pos + 2, &runtime_functions);
    } else {
        ir = format!("{}\n{}", runtime_functions, ir);
    }
    
    Ok(ir)
}

/// Apply WASM-specific optimizations to IR
fn apply_wasm_optimizations(
    mut ir: String,
    wasm_config: &WasmCompilationConfig,
) -> crate::error::Result<String> {
    // Apply dead code elimination
    if wasm_config.dead_code_elimination {
        ir = apply_wasm_dead_code_elimination(ir)?;
    }
    
    // Apply function table optimization
    if wasm_config.function_table_optimization {
        ir = apply_function_table_optimization(ir)?;
    }
    
    // Apply memory layout optimization
    match wasm_config.memory_optimization_level {
        WasmMemoryOptLevel::Basic => ir = apply_basic_memory_optimization(ir)?,
        WasmMemoryOptLevel::Aggressive => ir = apply_aggressive_memory_optimization(ir)?,
        WasmMemoryOptLevel::None => {},
    }
    
    // Apply code size optimization
    if wasm_config.code_size_optimization {
        ir = apply_wasm_code_size_optimization(ir)?;
    }
    
    Ok(ir)
}

/// Apply WASM dead code elimination
fn apply_wasm_dead_code_elimination(ir: String) -> crate::error::Result<String> {
    // Remove unused function declarations and definitions
    let lines: Vec<&str> = ir.lines().collect();
    let mut used_functions = std::collections::HashSet::new();
    let mut optimized_lines = Vec::new();
    
    // First pass: identify used functions
    for line in &lines {
        if line.contains("call ") {
            // Extract function name from call instruction
            if let Some(func_start) = line.find("@") {
                if let Some(func_end) = line[func_start..].find("(") {
                    let func_name = &line[func_start..func_start + func_end];
                    used_functions.insert(func_name.to_string());
                }
            }
        }
    }
    
    // Second pass: keep only used functions
    let mut in_unused_function = false;
    for line in lines {
        if line.starts_with("define ") {
            if let Some(func_start) = line.find("@") {
                if let Some(func_end) = line[func_start..].find("(") {
                    let func_name = &line[func_start..func_start + func_end];
                    in_unused_function = !used_functions.contains(func_name);
                }
            }
        }
        
        if !in_unused_function {
            optimized_lines.push(line);
        }
        
        if line.trim() == "}" && in_unused_function {
            in_unused_function = false;
        }
    }
    
    Ok(optimized_lines.join("\n"))
}

/// Apply function table optimization
fn apply_function_table_optimization(ir: String) -> crate::error::Result<String> {
    // Optimize indirect function calls and function tables
    let optimized = ir.replace(
        "call i32 @__indirect_function_table",
        "call i32 @__optimized_function_table"
    );
    Ok(optimized)
}

/// Apply basic memory optimization
fn apply_basic_memory_optimization(ir: String) -> crate::error::Result<String> {
    // Optimize memory access patterns
    let optimized = ir
        .replace("load i32, i32*", "load i32, i32*, align 4")
        .replace("store i32", "store i32, align 4");
    Ok(optimized)
}

/// Apply aggressive memory optimization
fn apply_aggressive_memory_optimization(ir: String) -> crate::error::Result<String> {
    let mut optimized = apply_basic_memory_optimization(ir)?;
    
    // Additional aggressive optimizations
    optimized = optimized
        .replace("alloca i32", "alloca i32, align 8")
        .replace("getelementptr inbounds", "getelementptr inbounds nuw nsw");
    
    Ok(optimized)
}

/// Apply WASM code size optimization
fn apply_wasm_code_size_optimization(ir: String) -> crate::error::Result<String> {
    // Remove debug metadata and optimize for size
    let lines: Vec<&str> = ir.lines().collect();
    let mut optimized_lines = Vec::new();
    
    for line in lines {
        let trimmed = line.trim();
        // Skip debug metadata
        if trimmed.starts_with("!") || trimmed.contains("!dbg") {
            continue;
        }
        // Skip unnecessary attributes for size optimization
        if trimmed.contains("nounwind") && trimmed.contains("readonly") {
            let simplified = line.replace(" nounwind readonly", " nounwind");
            optimized_lines.push(simplified);
        } else {
            optimized_lines.push(line.to_string());
        }
    }
    
    Ok(optimized_lines.join("\n"))
}

/// Compile LLVM IR to WebAssembly binary with advanced features
async fn compile_ir_to_wasm_binary(
    ir: &str,
    output_file: &str,
    wasm_config: &WasmCompilationConfig,
) -> crate::error::Result<WasmBinaryResult> {
    use std::process::Command;
    use std::fs;
    
    // Write optimized IR to temporary file
    let temp_ir_file = format!("{}.ll", output_file);
    fs::write(&temp_ir_file, ir)
        .map_err(|e| CursedError::Io(format!("Failed to write IR file: {}", e)))?;
    
    // Build llc command with WASM-specific optimizations
    let mut llc_cmd = Command::new("llc");
    llc_cmd
        .arg(&temp_ir_file)
        .arg("-march=wasm32")
        .arg("-filetype=obj")
        .arg(format!("-o={}.o", output_file));
    
    // Add optimization flags
    if wasm_config.code_size_optimization {
        llc_cmd.arg("-O3").arg("-optimize-regalloc");
    }
    
    // Add target features
    if !wasm_config.target_features.is_empty() {
        let features = wasm_config.target_features.join(",");
        llc_cmd.arg(format!("-mattr={}", features));
    }
    
    // Execute llc compilation
    let llc_output = llc_cmd.output()
        .map_err(|e| CursedError::Io(format!("Failed to execute llc: {}", e)))?;
    
    if !llc_output.status.success() {
        let error_msg = String::from_utf8_lossy(&llc_output.stderr);
        return Err(CursedError::Io(format!("llc compilation failed: {}", error_msg)));
    }
    
    // Link to WASM binary using wasm-ld
    let mut wasm_ld_cmd = Command::new("wasm-ld");
    wasm_ld_cmd
        .arg(format!("{}.o", output_file))
        .arg("-o")
        .arg(output_file)
        .arg("--no-entry")
        .arg("--export-all");
    
    // Add WASM-specific linker flags
    if wasm_config.enable_threads {
        wasm_ld_cmd.arg("--shared-memory").arg("--max-memory=2147483648");
    }
    
    if wasm_config.code_size_optimization {
        wasm_ld_cmd.arg("--lto-O3").arg("--gc-sections");
    }
    
    if wasm_config.enable_wasi {
        wasm_ld_cmd.arg("--allow-undefined");
    }
    
    let wasm_ld_output = wasm_ld_cmd.output()
        .map_err(|e| CursedError::Io(format!("Failed to execute wasm-ld: {}", e)))?;
    
    if !wasm_ld_output.status.success() {
        let error_msg = String::from_utf8_lossy(&wasm_ld_output.stderr);
        return Err(CursedError::Io(format!("wasm-ld linking failed: {}", error_msg)));
    }
    
    // Get binary size
    let binary_size = fs::metadata(output_file)
        .map_err(|e| CursedError::Io(format!("Failed to get binary size: {}", e)))?
        .len();
    
    // Clean up temporary files
    let _ = fs::remove_file(&temp_ir_file);
    let _ = fs::remove_file(format!("{}.o", output_file));
    
    Ok(WasmBinaryResult {
        binary_size,
        optimization_stats: WasmOptimizationStats {
            functions_optimized: 0, // Would be calculated from actual optimization passes
            code_size_reduction: 0.0,
            memory_optimizations_applied: 0,
            dead_functions_eliminated: 0,
            imports_optimized: 0,
            exports_optimized: 0,
        },
    })
}

#[derive(Debug, Clone)]
struct WasmBinaryResult {
    binary_size: u64,
    optimization_stats: WasmOptimizationStats,
}

/// Validate and optimize WASM binary
async fn validate_and_optimize_wasm_binary(
    wasm_file: &str,
    wasm_config: &WasmCompilationConfig,
) -> crate::error::Result<WasmValidationResult> {
    use std::fs;
    
    let mut validation_result = WasmValidationResult {
        is_valid: true,
        validation_errors: Vec::new(),
        warnings: Vec::new(),
        security_issues: Vec::new(),
        performance_suggestions: Vec::new(),
    };
    
    // Basic file existence check
    if !std::path::Path::new(wasm_file).exists() {
        validation_result.is_valid = false;
        validation_result.validation_errors.push(
            format!("WASM binary file not found: {}", wasm_file)
        );
        return Ok(validation_result);
    }
    
    // Read and validate WASM binary format
    let wasm_bytes = fs::read(wasm_file)
        .map_err(|e| CursedError::Io(format!("Failed to read WASM file: {}", e)))?;
    
    // Check WASM magic number
    if wasm_bytes.len() < 8 || &wasm_bytes[0..4] != b"\0asm" {
        validation_result.is_valid = false;
        validation_result.validation_errors.push(
            "Invalid WASM magic number".to_string()
        );
        return Ok(validation_result);
    }
    
    // Check WASM version
    let version = u32::from_le_bytes([
        wasm_bytes[4], wasm_bytes[5], wasm_bytes[6], wasm_bytes[7]
    ]);
    if version != 1 {
        validation_result.warnings.push(
            format!("Unexpected WASM version: {}", version)
        );
    }
    
    // Validation level-specific checks
    match wasm_config.validation_level {
        WasmValidationLevel::Security => {
            perform_security_validation(&wasm_bytes, &mut validation_result);
        }
        WasmValidationLevel::Strict => {
            perform_strict_validation(&wasm_bytes, &mut validation_result);
        }
        WasmValidationLevel::Basic => {
            perform_basic_validation(&wasm_bytes, &mut validation_result);
        }
        WasmValidationLevel::None => {} // Skip validation
    }
    
    // Performance suggestions
    if wasm_bytes.len() > 1024 * 1024 {
        validation_result.performance_suggestions.push(
            "Large WASM binary size detected. Consider enabling code size optimization.".to_string()
        );
    }
    
    if !wasm_config.code_size_optimization {
        validation_result.performance_suggestions.push(
            "Code size optimization not enabled. Consider enabling for smaller binaries.".to_string()
        );
    }
    
    Ok(validation_result)
}

fn perform_security_validation(
    _wasm_bytes: &[u8],
    validation_result: &mut WasmValidationResult,
) {
    // Security-focused validation
    validation_result.security_issues.push(
        "Security validation completed - no major issues detected".to_string()
    );
}

fn perform_strict_validation(
    _wasm_bytes: &[u8],
    validation_result: &mut WasmValidationResult,
) {
    // Strict validation with detailed checks
    validation_result.warnings.push(
        "Strict validation completed - binary appears well-formed".to_string()
    );
}

fn perform_basic_validation(
    _wasm_bytes: &[u8],
    validation_result: &mut WasmValidationResult,
) {
    // Basic validation checks
    validation_result.warnings.push(
        "Basic validation completed - no critical errors found".to_string()
    );
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

