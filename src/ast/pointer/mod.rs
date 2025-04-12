//! AST nodes for pointer types and operations in the CURSED language.
//!
//! This module defines the AST representations for pointer-related constructs,
//! including pointer types and pointer dereference operations. Pointers in CURSED
//! allow direct memory manipulation and enable advanced patterns like data structures
//! with self-references and efficient access to large data objects.
//!
//! In CURSED, pointers are denoted with the @ symbol, similar to how Go uses * for pointers.

mod types;
mod operations;

pub use types::PointerType;
pub use operations::PointerDereference;

#[cfg(test)]
mod tests;