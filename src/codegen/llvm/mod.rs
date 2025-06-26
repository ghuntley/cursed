/// LLVM Code Generation Module
/// 
/// Provides comprehensive LLVM-based code generation, optimization,
/// and performance monitoring for the CURSED compiler.

// Main LLVM code generator
pub mod main;

// Core LLVM code generation modules
pub mod optimization;
pub mod optimization_engine;
pub mod optimization_integration;
pub mod optimization_passes;
pub mod optimization_pipeline;
pub mod performance_monitor;
pub mod types;

// Existing modules
pub mod async_await;
pub mod bool_conversions;
pub mod channels;
pub mod control_flow;
// pub mod database_integration;
pub mod debug;
pub mod debug_info;
pub mod debug_integration;
pub mod debug_metadata;
pub mod enhanced_codegen;
pub mod error_handling;
pub mod error_propagation;
pub mod error_propagation_enhanced;
pub mod expression_compiler;
pub mod function_compilation;
pub mod function_registry;
pub mod gc_integration;
pub mod goroutine;
pub mod ipc;
pub mod jit_compilation;
pub mod jit_engine;
pub mod lto_integration;
pub mod osr;
pub mod package_integration;
pub mod panic;
pub mod process;
pub mod process_execution;
pub mod process_execution_ffi;
pub mod process_stubs;
pub mod process_ipc_integration;
pub mod question_mark;
pub mod real_compilation;
pub mod real_optimization_integration;
pub mod result_types;
pub mod result_types_simple;
pub mod stdlib_registry;
pub mod symbol_table;
// pub mod template;  // Disabled for compilation
pub mod tiered_compilation;
pub mod type_switch;
pub mod type_system;
pub mod variable_management;
pub mod web_vibez_integration;
use crate::error::CursedError;

// Re-export from main module
pub use main::*;

// Re-export JIT engine types
pub use jit_engine::{
    CursedJitEngine, JitEngineConfig, JitEngineStats
};

// Re-export main optimization types
pub use optimization::{OptimizationConfig, OptimizationManager, LlvmOptimizer, OptimizationPreset};
pub use crate::common::OptimizationLevel;
pub use optimization_engine::{
    OptimizationEngine, OptimizationEngineConfig, OptimizationResult, EngineStatistics
};
pub use optimization_passes::{
    PassRegistry, PassConfiguration, OptimizationPass, PassResult, PassTimeCategory
};
pub use optimization_pipeline::{
    OptimizationPipeline, PipelineResult, StageResult, PipelineStatistics
};
pub use performance_monitor::{
    PerformanceMonitor, MonitoringConfig, CodeMetrics, BaselineMetrics, PerformanceReport
};
pub use types::{
    LlvmType, LlvmValue, LlvmFunction, LlvmModule, LlvmContext
};
pub use package_integration::{
    LlvmPackageConfig
};
