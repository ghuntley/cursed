//! AST node for identifiers in the CURSED language.
//!
//! This module defines the AST representation for identifiers, which are names that
//! refer to variables, functions, types, or other named entities in the program.
//! Identifiers are among the most fundamental elements in the AST.

use crate::ast::{Expression, Node};
use std::any::Any;

/// Represents an identifier node in the AST.
///
/// An identifier is a name that refers to a value, function, type, or other
/// named entity in the program. Identifiers are used in variable references,
/// function calls, type references, and many other contexts.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// x := 5
/// print(x)
/// ```
///
/// Both `x` and `print` would be represented as `Identifier` nodes in the AST.
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: String,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: self.token.clone(),
            value: self.value.clone(),
        })
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
