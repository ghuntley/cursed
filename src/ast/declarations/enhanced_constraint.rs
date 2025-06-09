//! Enhanced AST nodes for complex generic constraints.
//!
//! This module provides more sophisticated constraint expressions beyond simple
//! interface bounds, including multiple bounds, associated types, and complex relationships.

use crate::ast::{Node, Statement, Expression};
use crate::lexer::token::Token;
use std::any::Any;

/// Represents a single type bound in a constraint (e.g., Display, Clone, Into<String>)
pub struct TypeBound {
    pub token: Token,           // The bound token
    pub interface_name: String, // Name of the interface/trait
    pub type_args: Vec<Box<dyn Expression>>, // Optional type arguments for generic interfaces
}

impl std::fmt::Debug for TypeBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeBound")
            .field("token", &self.token)
            .field("interface_name", &self.interface_name)
            .field("type_args", &format!("[{} type args]", self.type_args.len()))
            .finish()
    }
}

impl Clone for TypeBound {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            interface_name: self.interface_name.clone(),
            // Note: Cannot clone Box<dyn Expression> directly, use empty vec as fallback
            type_args: Vec::new(),
        }
    }
}

impl TypeBound {
    /// Creates a new TypeBound with just an interface name
    pub fn simple(token: Token, interface_name: String) -> Self {
        Self {
            token,
            interface_name,
            type_args: Vec::new(),
        }
    }

    /// Creates a new TypeBound with type arguments (e.g., Into<String>)
    pub fn with_args(token: Token, interface_name: String, type_args: Vec<Box<dyn Expression>>) -> Self {
        Self {
            token,
            interface_name,
            type_args,
        }
    }

    /// Checks if this bound has type arguments
    pub fn has_type_args(&self) -> bool {
        !self.type_args.is_empty()
    }
}

impl Node for TypeBound {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        if self.type_args.is_empty() {
            self.interface_name.clone()
        } else {
            let args: Vec<String> = self.type_args
                .iter()
                .map(|arg| arg.string())
                .collect();
            format!("{}<{}>", self.interface_name, args.join(", "))
        }
    }
}

/// Represents an enhanced generic constraint with multiple bounds
/// 
/// Supports syntax like: `T: Display + Clone + Into<String>`
#[derive(Debug, Clone)]
pub struct EnhancedConstraint {
    pub token: Token,                 // The constraint token
    pub parameter_name: String,       // The type parameter being constrained
    pub bounds: Vec<TypeBound>,       // Multiple type bounds connected with +
    pub associated_types: Vec<AssociatedType>, // Associated type constraints
}

impl EnhancedConstraint {
    /// Creates a new EnhancedConstraint with a single bound
    pub fn single_bound(token: Token, parameter_name: String, bound: TypeBound) -> Self {
        Self {
            token,
            parameter_name,
            bounds: vec![bound],
            associated_types: Vec::new(),
        }
    }

    /// Creates a new EnhancedConstraint with multiple bounds
    pub fn multiple_bounds(token: Token, parameter_name: String, bounds: Vec<TypeBound>) -> Self {
        Self {
            token,
            parameter_name,
            bounds,
            associated_types: Vec::new(),
        }
    }

    /// Adds an associated type constraint to this enhanced constraint
    pub fn with_associated_type(mut self, assoc_type: AssociatedType) -> Self {
        self.associated_types.push(assoc_type);
        self
    }

    /// Checks if this constraint has multiple bounds
    pub fn has_multiple_bounds(&self) -> bool {
        self.bounds.len() > 1
    }

    /// Checks if this constraint has associated types
    pub fn has_associated_types(&self) -> bool {
        !self.associated_types.is_empty()
    }
}

impl Node for EnhancedConstraint {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        let bounds_str = if self.bounds.is_empty() {
            String::new()
        } else {
            let bound_strings: Vec<String> = self.bounds
                .iter()
                .map(|bound| bound.string())
                .collect();
            bound_strings.join(" + ")
        };

        let assoc_str = if self.associated_types.is_empty() {
            String::new()
        } else {
            let assoc_strings: Vec<String> = self.associated_types
                .iter()
                .map(|assoc| assoc.string())
                .collect();
            format!(", {}", assoc_strings.join(", "))
        };

        format!("{}: {}{}", self.parameter_name, bounds_str, assoc_str)
    }
}

impl Statement for EnhancedConstraint {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents an associated type constraint (e.g., Iterator::Item = String)
pub struct AssociatedType {
    pub token: Token,                    // The associated type token
    pub interface_name: String,          // The interface containing the associated type
    pub type_name: String,               // The name of the associated type
    pub constraint: Box<dyn Expression>, // The type constraint for the associated type
}

impl std::fmt::Debug for AssociatedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssociatedType")
            .field("token", &self.token)
            .field("interface_name", &self.interface_name)
            .field("type_name", &self.type_name)
            .field("constraint", &"<expr>")
            .finish()
    }
}

impl Clone for AssociatedType {
    fn clone(&self) -> Self {
        // Note: This is a simplified clone that doesn't preserve the constraint expression
        Self::new(
            self.token.clone(),
            self.interface_name.clone(),
            self.type_name.clone(),
            // Cannot clone Box<dyn Expression>, using placeholder
            Box::new(crate::ast::expressions::literals::StringLiteral::new("placeholder".to_string()))
        )
    }
}

impl AssociatedType {
    /// Creates a new AssociatedType constraint
    pub fn new(
        token: Token,
        interface_name: String,
        type_name: String,
        constraint: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            interface_name,
            type_name,
            constraint,
        }
    }
}

impl Node for AssociatedType {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }

    fn string(&self) -> String {
        format!("{}::{} = {}", self.interface_name, self.type_name, self.constraint.string())
    }
}

/// Constraint operator types for enhanced constraints
#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintOperator {
    Implements,     // : (implements interface)
    Plus,          // + (multiple bounds)
    Equals,        // = (associated type binding)
    Subtype,       // <: (subtype relationship)
}

impl ConstraintOperator {
    /// Returns the string representation of the operator
    pub fn as_str(&self) -> &'static str {
        match self {
            ConstraintOperator::Implements => ":",
            ConstraintOperator::Plus => "+",
            ConstraintOperator::Equals => "=",
            ConstraintOperator::Subtype => "<:",
        }
    }

    /// Parses a constraint operator from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            ":" => Some(ConstraintOperator::Implements),
            "+" => Some(ConstraintOperator::Plus),
            "=" => Some(ConstraintOperator::Equals),
            "<:" => Some(ConstraintOperator::Subtype),
            _ => None,
        }
    }
}
