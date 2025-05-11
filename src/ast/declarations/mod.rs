//! Declarations module for the CURSED AST
//!
//! This module contains AST nodes representing different kinds of declarations
//! including function, struct, interface, and parameter declarations.

// Submodules
pub mod fields;
pub mod function;
pub mod struct_interface;
pub mod type_parameter;

// Re-exports from fields module
pub use fields::Field;
pub use fields::Parameter;

// Re-exports from function module
pub use function::FunctionStatement;

// Re-exports from struct_interface module
pub use struct_interface::{CollabStatement, MethodSignature, SquadStatement};

// Re-exports from type_parameter module
pub use type_parameter::GenericConstraint;
pub use type_parameter::TypeParameter;