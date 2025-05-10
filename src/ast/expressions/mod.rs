//! Expression nodes for the CURSED Abstract Syntax Tree
//!
//! This module contains all the expression types that represent computations
//! that produce values in CURSED programs. Expressions include literals,
//! identifiers, operators, function calls, and other constructs that yield values.
//!
//! The module is organized into submodules by expression category:
//!
//! - `literals`: Constants like integers, strings, and booleans
//! - `identifiers`: Variable and function references
//! - `operators`: Binary, prefix, and infix operations
//! - `calls`: Function and method calls
//! - `collections`: Arrays, hash literals, indexing expressions
//! - `concurrency`: Goroutine and channel operations
//! - `special`: Special expression types (type assertions, etc.)
//! - `channel`: Channel-specific expressions

pub mod calls;
pub mod channel;
pub mod collections;
pub mod concurrency;
pub mod constraint;
pub mod dot_expression;
pub mod generics;
pub mod identifiers;
pub mod if_expression;
pub mod literals;
pub mod operators;
pub mod range_expression;
pub mod special;
pub mod struct_expr;
pub mod types;
pub mod type_assertion;
pub mod empty;

pub use calls::{CallExpression, GenericCallExpression};
pub use collections::{ArrayLiteral, HashLiteral, IndexExpression};
pub use identifiers::Identifier;
pub use literals::*;
pub use operators::*;
// Re-export specific items instead of using glob imports to avoid collisions
pub use channel::ChannelExpression;
pub use concurrency::StanExpression;
pub use channel::{ReceiveExpression, SendExpression};
pub use special::{AssignmentExpression, BeLikeExpression, DefaultCase};
pub use type_assertion::TypeAssertion;
pub use range_expression::RangeExpression;

// Specific imports from generics to avoid conflicts
pub use dot_expression::DotExpression;
pub use generics::TypeReference;
pub use if_expression::IfExpression;
pub use struct_expr::{StructLiteral, StructFieldAccess, KeyValuePair};
pub use types::TypeConversionExpression;
pub use empty::Empty;
