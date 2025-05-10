//! AST node for pointer dereference operations in the CURSED language.
//!
//! This module defines the AST representation for pointer dereference operations,
//! which access the value stored at a pointer's memory address. In CURSED, dereferencing
//! is done using the @ symbol, similar to how the * operator is used in C/C++/Go.
//!
//! Dereferencing allows reading from or writing to the memory location pointed to by a pointer.

use crate::ast::{Expression, Node};
use std::any::Any;

/// Represents a pointer dereference expression in the AST.
///
/// A pointer dereference expression accesses the value stored at the memory address
/// contained in a pointer. It allows reading from or writing to the referenced location.
/// In CURSED, this is denoted with the @ symbol in front of a pointer expression.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// // Given p is a pointer to an integer
/// value := @p       // Read the value at the address p points to
/// @p = 42          // Write 42 to the address p points to
/// func := @funcPtr // Call the function that funcPtr points to
/// ```
///
/// The AST would have a `PointerDereference` with the pointer expression (p, funcPtr)
pub struct PointerDereference {
    pub token: String,
    pub pointer: Box<dyn Expression>,
}

impl Node for PointerDereference {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("@{}", self.pointer.string())
    }
}

impl Expression for PointerDereference {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(PointerDereference {
            token: self.token.clone(),
            pointer: self.pointer.clone_box(),
        })
    }
}
