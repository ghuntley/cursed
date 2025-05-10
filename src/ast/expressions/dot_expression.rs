//! AST node for dot (property/field access) expressions in the CURSED language.
//!
//! This module defines the AST representation for dot expressions, which are used to
//! access properties, fields, or methods on objects. Dot expressions are fundamental
//! for working with structs, packages, and other composite types.
//!
//! In CURSED, dot expressions appear in code like: `object.property` or `package.function`

use crate::ast::{Node, Expression};
use std::any::Any;

/// Represents a property or field access expression using dot notation in the AST.
///
/// A dot expression consists of an object expression followed by a dot and a property name.
/// It is used to access fields in structs, methods on objects, or functions in packages.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// user.name            // Accessing a struct field
/// math.sqrt(25)        // Accessing a package function
/// myString.length()    // Calling a method
/// ```
///
/// The AST would have a `DotExpression` with:
/// - object: the expression before the dot (e.g., `user`, `math`, `myString`)
/// - property: the identifier after the dot (e.g., `name`, `sqrt`, `length`)
pub struct DotExpression {
    pub token: String,
    pub object: Box<dyn Expression>,
    pub property: String,
}

impl Node for DotExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        format!("{}.{}", self.object.string(), self.property)
    }
}

impl Expression for DotExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(DotExpression {
            token: self.token.clone(),
            object: self.object.clone_box(),
            property: self.property.clone(),
        })
    }
}