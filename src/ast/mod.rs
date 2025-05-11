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

// Public re-exports of core types
pub use base::Program;
pub use pointer::{PointerDereference, PointerType};
pub use statement_utils::StatementExtensions;
pub use traits::{Expression, Node, Statement};
pub use expressions::TypeAssertion;

// Explicitly re-export important types for easier use
pub use declarations::GenericConstraint;

// Re-export control flow types
pub use control_flow::*;

// Re-export declarations module types but exclude fields to avoid conflicts
pub use declarations::{FunctionStatement, SquadStatement, CollabStatement, ParameterStatement};
// Only import Parameter and TypeParameter once
pub use declarations::Parameter;
pub use declarations::TypeParameter;
// Explicitly import Field from declarations to avoid ambiguity
pub use declarations::fields::Field;

// Re-export expressions
pub use expressions::*;

// Re-export BlockStatement as Block for backward compatibility - only do it once
pub use statements::block::BlockStatement as Block;
// Explicitly import FieldStatement
pub use statements::fields::FieldStatement;
