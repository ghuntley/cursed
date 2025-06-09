//! AST node for slice literal expressions in the CURSED language.
//!
//! This module defines the AST representation for slice literal expressions,
//! which create slice values directly in code with a specified element type
//! and list of initial values.
//!
//! Slice literals have the syntax: `[]Type{element1, element2, ...}`
//! where Type is the element type and the elements are expressions.

use crate::ast::{Expression, Node};
use crate::lexer::Token;
use std::any::Any;

/// Represents a slice literal expression in the AST.
///
/// A slice literal creates a slice value directly in code by specifying
/// the element type and listing initial elements inside curly braces.
/// The slice type is indicated by the `[]Type` prefix before the braces.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// []normie{1, 2, 3}        // Slice of integers
/// []tea{"key": "value"}    // Slice of maps
/// []thicc{}                // Empty slice of int64s
/// []sip{'a', 'b', 'c'}     // Slice of characters
/// ```
///
/// The AST would have a `SliceLiteral` with:
/// - element_type: an expression representing the type (e.g., Identifier("normie"))
/// - elements: a vector of expressions for each initial element
pub struct SliceLiteral {
    /// The token that starts the slice literal (usually the '[' token)
    pub token: Token,
    /// The type of elements in the slice (e.g., "normie", "thicc", etc.)
    pub element_type: Box<dyn Expression>,
    /// The list of expressions that will be the initial elements of the slice
    pub elements: Vec<Box<dyn Expression>>,
}

impl std::fmt::Debug for SliceLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SliceLiteral")
            .field("token", &self.token.token_literal())
            .field("element_type", &self.element_type.string())
            .field("elements", &self.elements.iter().map(|e| e.string()).collect::<Vec<String>>())
            .finish()
    }
}

impl Node for SliceLiteral {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let elements = self
            .elements
            .iter()
            .map(|e| e.string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("[]{}{{{}}}", self.element_type.string(), elements)
    }
}

impl Expression for SliceLiteral {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(SliceLiteral {
            token: self.token.clone(),
            element_type: self.element_type.clone_box(),
            elements: self.elements.iter().map(|e| e.clone_box()).collect(),
        })
    }
}

impl SliceLiteral {
    /// Creates a new slice literal with the given element type and elements
    ///
    /// # Arguments
    ///
    /// * `token` - The token that starts the slice literal
    /// * `element_type` - An expression representing the type of elements
    /// * `elements` - A vector of expressions for the initial elements
    ///
    /// # Returns
    ///
    /// A new `SliceLiteral` instance
    pub fn new(
        token: Token,
        element_type: Box<dyn Expression>,
        elements: Vec<Box<dyn Expression>>,
    ) -> Self {
        SliceLiteral {
            token,
            element_type,
            elements,
        }
    }
    
    /// Returns true if this is an empty slice literal (no elements)
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    
    /// Returns the number of elements in the slice literal
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
