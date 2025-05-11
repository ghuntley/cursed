//! Abstract Syntax Tree (AST) for the CURSED programming language
//!
//! This module defines the AST structures and interfaces used to represent
//! CURSED programs after parsing. The AST is a structured representation of
//! the source code that can be processed by later stages of compilation,
//! such as type checking and code generation.
//!
//! ## Key Components
//!
//! * `base`: Core AST structures like Program
//! * `expressions`: Expression types (literals, operations, function calls, etc.)
//! * `statements`: Statement types (assignments, declarations, etc.)
//! * `control_flow`: Control structures (if/else, loops, switches)
//! * `declarations`: Declarations (functions, types, variables, constants)
//! * `pointer`: Pointer-related operations and types
//! * `traits`: Common interfaces for AST nodes
//! * `type_assertion`: Interface type assertions and conversions

// Module declarations
pub mod base;
pub mod control_flow;
pub mod declarations;
pub mod expressions;
pub mod pointer;
pub mod statement_utils;
pub mod statements;
pub mod traits;
// Public re-exports
pub use base::Program;
pub use pointer::{PointerDereference, PointerType};
pub use statement_utils::StatementExtensions;
pub use traits::{Expression, Node, Statement};
pub use expressions::TypeAssertion;
// Directly re-export Parameter struct
pub use crate::ast::declarations::ParameterStatement as Parameter;
// Re-export TypeParameter from declarations module
pub use crate::ast::declarations::TypeParameter;
// Re-export BlockStatement as Block for backward compatibility
pub use crate::ast::statements::block::BlockStatement as Block;

// Re-export all AST types for easier imports
pub use control_flow::*;
pub use declarations::*;
pub use expressions::*;
pub use statements::*;
