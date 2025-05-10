//! Core runtime and language services for CURSED
//!
//! This module contains the central components of the CURSED language runtime
//! and compiler services. It includes the type system, symbol management,
//! concurrency primitives, and runtime type representations.
//!
//! Key components:
//! - Type checking and type system services
//! - Symbol table for name resolution and scope management
//! - Channel and goroutine implementations for concurrency
//! - Generic code instantiation services
//! - Runtime type information

pub mod channel;
pub mod char;
pub mod generic_instantiation;
pub mod goroutine;
pub mod nested_generic_instantiation;
pub mod symbol_table;
pub mod thread_safe_goroutine;
pub mod interface_type_checker;
pub mod interface_registry;
pub mod interface_registry_cache;
pub mod type_checker;
pub mod type_checker_interface_registry;
pub mod type_infer;
pub mod constraint_error;
pub mod async_constraint_checker;
pub mod constraint_recovery;
pub mod nested_interface_registry;
pub mod deep_nested_interface_registry;
pub mod interface_registry_extensions;
pub mod interface_registry_visualization;
pub mod interface_registry_visualization_enhanced;
pub mod interface_registry_visualization_reference;

/// Runtime representation of a compiled CURSED function
///
/// This structure holds the compiled bytecode, metadata, and runtime information
/// for a CURSED function. It serves as the interface between the compiler and
/// the runtime system, providing all the information needed to execute the function.
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    /// Name of the function
    pub name: String,
    /// Bytecode instructions
    pub bytecode: Vec<u8>,
    /// IR representation of the function
    pub ir_representation: String,
    /// Number of local variables
    pub num_locals: usize,
    /// Number of parameters
    pub num_parameters: usize,
    /// Names of free variables captured from outer scopes
    pub free_variables: Vec<String>,
    /// Whether this function accepts variadic arguments
    pub is_variadic: bool,
}
