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
// Re-export binary compiler
pub use self::binary_compiler::BinaryCompiler;

// Re-export binary compiler types
pub use self::binary_compiler::DebugInfoLevel;
// Interface implementation and dynamic dispatch
pub use self::dynamic_dispatch::{InterfaceManager, InterfaceStructure, VTable, VTableImpl};
pub use self::interface_implementation::InterfaceImplementation;

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
pub use self::property_access::PropertyAccessCompilation;
pub use self::assignment::AssignmentCompilation;
pub use self::break_statement::BreakStatementCompilation;
pub use self::continue_statement::ContinueStatementCompilation;
pub use self::import_statement::ImportStatementCompilation;
pub use self::later_statement::LaterStatementCompilation;
pub use self::switch_statement::SwitchStatementCompilation;
pub use self::if_expression::IfExpressionCompilation;
pub use self::struct_field_inference::StructFieldInference;
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
mod dot_expressions;  // Dot expression compilation (module.function)
mod hook_dot_expressions; // Temporary patch for dot expressions
pub mod enhanced_monomorphization; // Enhanced monomorphization with constraint checking
mod errors;
mod expression;
pub mod function_monomorphization;
pub mod memory_layout;   // Memory layout management
pub mod monomorphization;// Function monomorphization implementation
mod pointer_ops;     // Standardized pointer operations implementation
pub mod pointer;     // Keep for backward compatibility
mod statement;
mod string_switch;
pub mod struct_monomorphization;
mod types;
mod variables;
mod intrinsics;      // Standard library intrinsics
mod break_continue;  // Break and continue statement handling
mod control_flow;    // Control flow statements
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

// Import key modules and traits
pub use string_switch::*;
// These are already re-exported through self:: in the re-exports section