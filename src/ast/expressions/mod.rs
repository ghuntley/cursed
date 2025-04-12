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
pub mod identifiers;
pub mod literals;
pub mod operators;
pub mod special;

pub use calls::*;
pub use collections::*;
pub use identifiers::*;
pub use literals::*;
pub use operators::*;
// Re-export specific items instead of using glob imports to avoid collisions
pub use channel::ChannelExpression;
pub use concurrency::{ReceiveExpression, SendExpression, StanExpression};
pub use special::*;
