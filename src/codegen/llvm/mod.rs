//! LLVM code generation for CURSED programs
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

// Re-export traits for module functionality
pub use self::container_layout::{ContainerLayout, ContainerLayoutExtension, ContainerLayoutManager};
pub use self::expression::ExpressionCompilation;
pub use self::memory_layout::{MemoryLayout, MemoryLayoutExtension, MemoryLayoutManager};
pub use self::monomorphization::{MonomorphizationManagerExtension, SpecializedFunctionBuilderExtension, SpecializedFunctionBuilder, MonomorphizationManager};
pub use self::statement::StatementCompilation;
pub use self::variables::VariableHandling;
pub use self::pointer_ops::PointerOperations;  // Updated to use the new standardized module
pub use self::basic_expressions::BasicExpressionOperations;
pub use self::function_monomorphization::FunctionMonomorphization;
pub use self::struct_monomorphization::StructMonomorphization;

// Module declarations
mod context;         // Main LlvmCodeGenerator implementation
mod basic_expressions;
mod builder;
pub mod container_layout; // Container memory layout optimization
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

/// Represents a loop context for tracking break/continue blocks
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
