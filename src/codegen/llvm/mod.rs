//! LLVM code generation for CURSED programs
//!
//! This module handles the translation of CURSED AST to LLVM IR,
//! including type generation, expression evaluation, control flow,
//! and dynamic dispatch for interfaces.
//!
//! This module translates the CURSED Abstract Syntax Tree (AST) into LLVM
//! Intermediate Representation (IR), enabling optimization and native code
//! generation. It handles type translation, control flow, function calls,
//! memory management, and all other aspects of the CURSED runtime model.
//!
//! The code generator maintains mappings between CURSED language constructs
//! and their LLVM representations, including specialized versions of generic
//! functions and types created through monomorphization.
//!
//! ## Architecture
//!
//! The LLVM code generator is structured as a set of specialized modules:
//!
//! - **context.rs**: Main implementation of `LlvmCodeGenerator`
//! - **basic_expressions.rs**: Compilation of literal and operator expressions
//! - **pointer_ops.rs**: Address-of and dereference operations
//! - **variables.rs**: Variable declaration and reference handling
//! - **statement.rs**: Statement compilation
//! - **control_flow.rs**: If/while/for statement handling
//!
//! The implementation uses a standardized approach where:
//! 
//! 1. All modules import `LlvmCodeGenerator` from `context.rs`
//! 2. Functionality is added through trait implementations
//! 3. The main `mod.rs` re-exports the core types for external use
//!
//! ## Binary Compilation
//!
//! The `binary_compiler.rs` module provides Ahead-Of-Time (AOT) compilation to native
//! executable binaries, handling the entire process from LLVM IR generation to linking.
//!
//! ## Compatibility
//!
//! For backward compatibility, the `generator.rs` module re-exports 
//! from `context.rs` so code that previously imported from `generator.rs`
//! continues to work without changes.
//!
//! The legacy `pointer.rs` implementation is replaced by the standardized
//! `pointer_ops.rs` module, with compatibility wrappers provided.

// Re-export public types and functions
pub use self::context::LlvmCodeGenerator;

// Re-export extension traits
pub use self::llvm_code_generator_extensions::{
    SourceLocationExtensions, SymbolLookupExtensions, ErrorPathExtensions
};
pub use self::basic_value_extensions::{BasicValueExtensions, BasicTypeExtensions};

// Re-export type registry helpers
pub use self::interface_type_registry_helpers::TypeNameRegistry;

// Import type assertion implementation modules
use type_assertion_implementation::register_type_assertion_implementation;

// Re-export common utilities for interface type assertions
pub use self::interface_type_assertion_common::{get_result_type, get_source_location_type, build_struct_value, call_error_propagation_function, is_string_type_by_name, create_string_constant_from_codegen, find_inheritance_path, InterfaceRegistry, InterfacePathFinder, get_interface_registry, get_interface_path_finder, detect_diamond_inheritance, MutableInterfaceRegistry, get_interface_registry_mut};

// Re-export binary compiler
pub use self::binary_compiler::BinaryCompiler;

// Re-export binary compiler types
pub use self::binary_compiler::DebugInfoLevel;
// Interface implementation and dynamic dispatch
pub use self::dynamic_dispatch::{InterfaceManager, InterfaceStructure, VTable, VTableImpl};
pub use self::interface_implementation::InterfaceImplementation;
pub use self::enhanced_dynamic_dispatch::EnhancedDynamicDispatch;
pub use self::optimized_dynamic_dispatch::{OptimizedDynamicDispatch, OptimizedDynamicDispatchExtensions};
pub use self::integrated_interface_operations::IntegratedInterfaceOperations;
pub use self::auto_interface_dispatcher::{AutoInterfaceDispatcher, AutoInterfaceDispatchExtension};
pub use self::auto_interface_dispatcher_integration::AutoInterfaceDispatcherIntegration;
// pub use self::interface_type_assertion_error_handling::EnhancedTypeAssertionErrorHandling; // Commented out due to conflicts
// pub use self::interface_path_finder_enhanced::EnhancedInterfacePathFinder; // Commented out due to conflicts

// Re-export traits for module functionality
pub use self::container_layout::{ContainerLayout, ContainerLayoutExtension, ContainerLayoutManager};
pub use self::dot_expressions::DotExpressionCompilation;
pub use self::expression::ExpressionCompilation;
pub use self::hook_dot_expressions::patch_main_function; // Temporary dot expression patch
pub use self::memory_layout::{MemoryLayout, MemoryLayoutExtension, MemoryLayoutManager};
pub use self::monomorphization::{MonomorphizationManagerExtension, SpecializedFunctionBuilderExtension, SpecializedFunctionBuilder, MonomorphizationManager};
pub use self::statement::StatementCompilation;
pub use self::variables::VariableHandling;
pub use self::pointer_ops::PointerOperations;  // Updated to use the new standardized module
pub use self::basic_expressions::BasicExpressionOperations;
pub use self::function_monomorphization::FunctionMonomorphization;
pub use self::struct_monomorphization::StructMonomorphization;
pub use self::enhanced_monomorphization::EnhancedMonomorphization;
pub use self::integrated_monomorphization::IntegratedMonomorphization;
pub use self::improved_field_accessors::ImprovedFieldAccessors;
pub use self::lru_field_accessors::LruCachedFieldAccessors;
pub use self::property_access::PropertyAccessCompilation;
pub use self::assignment::AssignmentCompilation;
pub use self::break_statement::BreakStatementCompilation;
pub use self::continue_statement::ContinueStatementCompilation;
pub use self::import_statement::ImportStatementCompilation;
pub use self::later_statement::LaterStatementCompilation;
pub use self::switch_statement::SwitchStatementCompilation;
pub use self::if_expression::IfExpressionCompilation;
pub use self::struct_field_inference::StructFieldInference;
// Interface type assertion trait
pub use self::type_assertion::InterfaceTypeAssertion;
// Improved interface type assertions with additional runtime information
pub use self::interface_type_assertion::ImprovedTypeAssertion;
// Enhanced interface type assertions with optimized implementation
pub use self::enhanced_type_assertion::EnhancedTypeAssertion;
// Type assertion error handling
pub use self::interface_type_assertion_errors::TypeAssertionErrorHandler;
// Enhanced runtime debugging for interface type assertions
pub use self::interface_type_assertion_debugging::{RuntimeTypeAssertionDebugging, TypeAssertionDebugLevel};
// Type assertion integration with main compiler pipeline
pub use self::type_assertion_integration::TypeAssertionIntegration;
pub use self::type_assertion_implementation::IntegratedTypeAssertion;
// Improved type assertion integration with proper error propagation
pub use self::improved_type_assertion_integration::ImprovedTypeAssertionIntegration;
// Enhanced error propagation for interface type assertions
pub use self::interface_type_assertion_error_propagation::InterfaceTypeAssertionErrorPropagation;
// Integrated error propagation for the ? operator
pub use self::interface_type_assertion_error_propagation_integration::InterfaceTypeAssertionErrorIntegration;
// Improved error propagation system with consistent ? operator usage
pub use self::interface_type_assertion_error_propagation_improved::ImprovedErrorPropagation;
// Enhanced source location support for interface type assertion error propagation
pub use self::interface_type_assertion_error_propagation_source_location::EnhancedSourceLocationErrorPropagation;
// Enhanced source location support with file system integration
pub use self::interface_type_assertion_enhanced_source_location::EnhancedSourceLocationSupport;
// Filesystem integration for source location tracking
pub use self::interface_type_assertion_filesystem_integration::FilesystemSourceLocationIntegration;
// Enhanced error propagation with filesystem integration
pub use self::interface_type_assertion_error_propagation_filesystem::EnhancedErrorPropagationWithFilesystem;
// Comprehensive integration between error propagation and filesystem source location tracking
pub use self::interface_type_assertion_error_propagation_filesystem_integration::ComprehensiveErrorFilesystemIntegration as ComprehensiveErrorPropagationIntegration;
// Nesting level tracking for interface type assertions
pub use self::interface_type_assertion_nesting::{NestedTypeAssertion, TypeAssertionNestingContext};
// Interface type registry for runtime type information
pub use self::interface_type_registry::{InterfaceTypeRegistry, InterfaceTypeRegistryAccess};
// Interface registry extension checking for inheritance verification
pub use self::interface_path_finder_enhanced::InterfaceTypeRegistryExtensionChecking;
pub use self::interface_registry_extension_checking::{InterfaceTypeRegistryExtensionChecker, register_interface_type_registry_extension_checking};
// Enhanced interface type registry with full runtime type information
pub use self::interface_type_registry_enhanced::EnhancedTypeRegistry;
// Enhanced type assertions with rich type information
pub use self::interface_type_assertion_enhanced::EnhancedInterfaceTypeAssertion as EnhancedTypeAssertionWithRegistry;
// Enhanced interface type assertions with complex inheritance pattern support
pub use self::interface_type_assertion_enhanced_impl::EnhancedInterfaceTypeAssertion;
// Improved type registry with better runtime type information
pub use self::type_registry_improved::ImprovedTypeRegistry;
// Enhanced nested interface type assertions with proper error propagation
pub use self::interface_type_assertion_nested_enhanced::NestedInterfaceTypeAssertionEnhanced;
// Interface type assertion path visualization for debugging and developer tooling
pub use self::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
// Enhanced interface type assertion path visualization with improved error handling
pub use self::interface_type_assertion_path_visualization_enhanced::EnhancedInterfaceTypeAssertionPathVisualization;
// Adapter for proper method exposure between path visualization traits
pub use self::interface_type_assertion_path_visualization_adapter::InterfaceTypeAssertionPathVisualizationAdapter;
// Interface type assertion debug utilities for runtime debugging support
pub use self::interface_type_assertion_debug::{InterfaceTypeAssertionDebug, TypeAssertionDebugConfig};
// Simple path finding algorithms for interface inheritance relationships
pub use self::interface_path_finder_simple::*;
// Enhanced path finding algorithms for interface inheritance relationships with visualization
pub use self::interface_path_finder_enhanced::InterfaceInheritancePath;
pub use self::interface_path_finder_enhanced_fix::EnhancedInterfacePathFinder;
// Interface type assertion with registry integration for enhanced error diagnostics
pub use self::interface_type_assertion_with_registry::InterfaceTypeAssertionWithRegistry;
// Enhanced error handling for interface type assertions with proper propagation between systems
// pub use self::interface_type_assertion_error_handling::EnhancedTypeAssertionErrorHandling; // Commented out due to conflicts
// Range clause compilation traits
pub use self::range_clause_fixed::RangeClauseCompilationEnhanced as RangeClauseCompilation;
pub use self::range_clause_error_recovery::{RangeClauseErrorRecovery, RangeClauseCompilationWithRecovery};
pub use self::range_clause_fixed_extension::RangeClauseFixedMethodsExtension;
// Map iteration improvements
pub use self::map_iteration_improvements::MapIterationEnhancements;
pub use self::loop_context::*;
// Interface registry visualization integration
pub use self::interface_registry_visualization_integration::InterfaceRegistryVisualizationIntegration;
// Will be re-exported in a future PR
// pub use self::interface_type_integration::InterfaceTypeIntegration;


// Module declarations
mod context;         // Main LlvmCodeGenerator implementation
pub mod binary_compiler; // Binary (AOT) compiler implementation
mod runtime_linking; // Runtime library linking options
mod platform_optimizations; // Platform-specific code generation optimizations
mod debug_info;      // Debug information generation
mod cross_compilation; // Cross-compilation support
mod size_optimization; // Size optimization
mod optimize_module; // Module optimization passes
mod basic_expressions;
mod builder;
pub mod container_layout; // Container memory layout optimization
pub mod llvm_code_generator_extensions; // Extension traits for LlvmCodeGenerator
pub mod basic_value_extensions; // Extension traits for LLVM BasicValueEnum and types
mod dot_expressions;  // Dot expression compilation (module.function)
mod hook_dot_expressions; // Temporary patch for dot expressions
pub mod enhanced_monomorphization; // Enhanced monomorphization with constraint checking
pub mod integrated_monomorphization; // Integrated monomorphization system
pub mod improved_field_accessors; // Improved field accessors with proper error handling
pub mod lru_field_accessors; // LRU cached field accessors with optimized performance
pub mod interface_field_accessors_lru; // Interface field accessors with LRU caching
mod errors;
mod expression;
pub mod function_monomorphization;
pub mod memory_layout;   // Memory layout management
pub mod monomorphization;// Function monomorphization implementation
mod pointer_ops;     // Standardized pointer operations implementation
pub mod pointer;     // Keep for backward compatibility
pub mod interface_registry; // Interface registry for type assertions
mod statement;
mod string_switch;
mod string_utils;
mod path_utils;
pub mod struct_monomorphization;
mod types;
mod variables;
mod intrinsics;      // Standard library intrinsics
mod break_continue;  // Break and continue statement handling
mod control_flow;    // Control flow statements
// Range clause implementation with proper error handling
mod range_clause_fixed;
mod range_clause_error_recovery; // Error recovery for range clause operations
mod range_clause_fixed_extension; // Extension trait for range clause fixed methods
mod map_iteration_improvements; // Improved type determination for map iterations
mod loop_context;    // Loop context for break/continue management
mod concurrency;     // Goroutine and channel operations
mod property_access; // Property access expressions (obj.field)
mod assignment;      // Assignment expressions (a = b)
mod break_statement; // Break statement implementation
mod continue_statement; // Continue statement implementation
mod import_statement; // Import statement implementation
mod later_statement; // Later (defer) statement implementation
mod switch_statement; // Switch statement implementation
mod if_expression;   // If expression implementation
pub mod struct_field_inference; // Struct field type inference
// Dynamic dispatch for interfaces
pub mod dynamic_dispatch;
mod interface_implementation; // Interface implementation for code generator
mod interface_type_integration; // Integration of type checker with interface implementation
mod type_assertion; // Interface type assertion and conversion
mod interface_type_assertion; // Improved interface type assertions with additional runtime information
mod interface_type_assertion_errors; // Enhanced error handling for interface type assertions
mod type_assertion_implementation; // Integrated type assertion implementation
mod enhanced_type_assertion; // Optimized implementation of interface type assertions
mod type_assertion_integration; // Integration of type assertions with main compiler pipeline
mod enhanced_dynamic_dispatch; // Enhanced dynamic dispatch with improved error handling
mod optimized_dynamic_dispatch; // Optimized dynamic dispatch with inline caching and speculative dispatch
mod integrated_interface_operations; // Unified interface operations system
mod auto_interface_dispatcher; // Automatic code generation for interface method dispatching
mod auto_interface_dispatcher_integration; // Integration of auto interface dispatcher with the compiler
mod interface_field_accessors; // Integration of improved field accessors with interface system
mod interface_type_assertion_debugging; // Enhanced runtime debugging for interface type assertions
mod interface_type_assertion_nesting; // Nesting level tracking for interface type assertions
mod interface_type_registry; // Registry for storing type information at runtime
mod interface_type_registry_enhanced; // Enhanced registry for full runtime type information
pub mod interface_type_registry_common; // Common implementations to avoid duplicate definitions
mod pointer_type_extension; // Extension trait for pointer element type access
mod interface_type_assertion_enhanced; // Enhanced type assertions with rich error information
mod interface_type_assertion_enhanced_impl; // Enhanced interface type assertion implementation for complex inheritance patterns
mod type_registry_improved; // Improved type registry with better runtime type information
mod improved_type_assertion_integration; // Improved interface type assertion integration with proper error propagation
pub mod interface_type_assertion_error_propagation; // Enhanced error propagation for interface type assertions
pub mod interface_type_assertion_error_propagation_improved; // Improved error propagation system with consistent ? operator usage
pub mod interface_type_assertion_error_propagation_integration; // Integration layer for error propagation with ? operator support
pub mod interface_type_assertion_nested_enhanced; // Enhanced nested interface type assertions with proper error propagation
pub mod interface_type_assertion_path_visualization;
pub mod interface_type_assertion_path_visualization_adapter; // Adapter for ensuring proper method exposure
pub mod interface_type_assertion_debug; // Interface type assertion debug utilities
pub mod interface_registry_integration; // Visual debugging tools for interface type assertions
pub mod interface_type_assertion_path_visualization_enhanced; // Enhanced visual debugging tools with better error handling
pub mod interface_registry_visualization_integration; // Integration of interface registry visualization with code generator
pub mod interface_path_finder_simple; // Simple path finding algorithms for interface inheritance relationships
pub mod interface_path_finder_enhanced; // Enhanced path finding algorithms for interface inheritance relationships with visualization
pub mod interface_path_finder_enhanced_fix; // Fix for enhanced path finding with multi-path support for diamond inheritance detection
pub mod interface_type_assertion_with_registry; // Interface type assertion with registry integration
pub mod interface_registry_extension_checking; // Extension relationship checking for interface inheritance verification
mod interface_type_assertion_error_handling; // Enhanced error handling for interface type assertions with proper error propagation
mod interface_type_assertion_result_integration; // Result-based error propagation with ? operator integration
mod interface_type_assertion_result_implementation; // Comprehensive Result implementation with proper ? operator integration
mod interface_type_assertion_result; // Interface type assertion with Result type and ? operator integration
pub mod interface_type_assertion_common; // Common utilities for interface type assertions
pub mod interface_type_assertion_error_propagation_source_location; // Enhanced source location support for error propagation
pub mod interface_type_assertion_enhanced_source_location; // Improved source location support with file system integration
pub mod interface_type_assertion_error_propagation_filesystem; // Enhanced error propagation with filesystem integration
pub mod interface_type_assertion_filesystem_integration; // Filesystem integration for source location tracking
pub mod interface_type_assertion_error_propagation_filesystem_integration; // Comprehensive integration between error propagation and filesystem source location tracking
pub mod interface_type_assertion_error_visualization; // Enhanced visualization of interface type assertion errors with source context
pub mod interface_type_assertion_error_visualization_enhanced; // Enhanced error visualization with diamond pattern detection and rich formatting
pub mod interface_type_assertion_diamond_inheritance; // Specialized detection and visualization of diamond inheritance patterns
pub mod interface_type_assertion_diamond_inheritance_handler; // Enhanced handler for diamond inheritance pattern detection and visualization
pub mod interface_type_assertion_benchmarking; // Performance benchmarking for interface type assertions
// Module already declared above

/// Represents a loop context for tracking break/continue blocks in nested loops
/// 
/// When loops are nested, each loop creates its own context and pushes it onto
/// the context stack. Break/continue statements always operate on the innermost
/// loop context (the top of the stack). When a loop ends, its context is popped.
/// 
/// This ensures that break/continue statements in nested loops properly target
/// the correct loop level.
#[derive(Clone)]
pub struct LoopContext<'ctx> {
    /// The name of the loop
    pub name: String,
    /// Block to jump to for break statements
    pub break_block: inkwell::basic_block::BasicBlock<'ctx>,
    /// Block to jump to for continue statements
    pub continue_block: inkwell::basic_block::BasicBlock<'ctx>,
}

// Re-export ContainerKind for backward compatibility
pub use self::container_layout::ContainerKind;

// Re-export pointer type extension
pub use self::pointer_type_extension::{PointerTypeExtension, BasicTypeEnumExtension};

// Re-export string utilities extension
pub use self::string_utils::StringUtilsExtension;

// Re-export path utilities extension
pub use self::path_utils::PathStringRepresentation;

// Re-export Result integration for interface type assertions
pub use self::interface_type_assertion_result_integration::TypeAssertionResultIntegration;
pub use self::interface_type_assertion_result_implementation::{IntegratedResultTypeAssertion, TypeAssertionErrorInfo, register_result_implementation};
pub use self::interface_type_assertion_result::{InterfaceTypeAssertionResult, ResultPropagation};
// Re-export error propagation helpers for ? operator integration
pub use self::interface_type_assertion_error_propagation_integration::{is_type_mismatch_error, extract_type_info, register_error_propagation_integration};
// Re-export comprehensive error propagation filesystem integration
pub use self::interface_type_assertion_error_propagation_filesystem_integration::ComprehensiveErrorFilesystemIntegration;
pub use self::interface_type_assertion_error_propagation_filesystem_integration::register_comprehensive_error_filesystem_integration as register_comprehensive_error_propagation_integration;
pub use self::interface_type_assertion_error_visualization::ErrorVisualization;
pub use self::interface_type_assertion_error_visualization::register_error_visualization;
pub use self::interface_type_assertion_error_visualization_enhanced::EnhancedErrorVisualization;
pub use self::interface_type_assertion_error_visualization_enhanced::register_enhanced_error_visualization;
pub use self::interface_type_assertion_diamond_inheritance::{DiamondInheritanceDetection, DiamondInheritancePattern};
pub use self::interface_type_assertion_diamond_inheritance::register_diamond_inheritance_detection;
pub use self::interface_type_assertion_diamond_inheritance_handler::{DiamondInheritanceHandler, DiamondInheritanceInfo, InterfaceTypeRegistryExtensionCheckingAccess};
pub use self::interface_type_assertion_diamond_inheritance_handler::register_diamond_inheritance_handler;
// Re-export filesystem source location integration for interface type assertions
pub use self::interface_type_assertion_filesystem_integration::InterfaceTypeAssertionFilesystemIntegration;
pub use self::interface_type_assertion_filesystem_integration::register_filesystem_integration as register_filesystem_source_location_integration;
// Removed duplicate FilesystemSourceLocationIntegration declaration - already imported above
// Re-export benchmarking functionality for interface type assertions
pub use self::interface_type_assertion_benchmarking::{TypeAssertionBenchmarking, HierarchyPattern, BenchmarkStats, TypeAssertionBenchmark, TypeAssertionBenchmarkSuite};

// Import key modules and traits
pub use string_switch::*;

// Type registry helpers module
mod interface_type_registry_helpers;
pub use interface_field_accessors::InterfaceFieldAccessors;
// These are already re-exported through self:: in the re-exports section