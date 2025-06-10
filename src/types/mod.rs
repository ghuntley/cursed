//! Type system modules for CURSED
//!
//! This module contains the type system components for the CURSED programming language.

pub mod result;

// Re-export commonly used types
pub use result::{Result, Option, ResultTypeExpression, OptionTypeExpression};
