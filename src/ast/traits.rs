//! Trait definitions for AST nodes
//!
//! This module defines the core traits that all AST nodes must implement.
//! These traits establish the interface for the various types of nodes
//! in the Abstract Syntax Tree, allowing for polymorphic operations on them.

use std::any::Any;
use crate::ast::expressions::Identifier;

/// Base trait for all nodes in the abstract syntax tree
///
/// Provides common functionality for all AST nodes, including string
/// representation and token information. This is implemented by all
/// statement and expression types.
pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

/// Represents a statement node in the Abstract Syntax Tree
///
/// Statements are top-level language constructs that don't produce values
/// but instead perform actions, such as variable declarations, assignments,
/// control flow structures, etc. Each statement implementation includes its
/// specific properties and behaviors.
pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

/// Represents an expression node in the Abstract Syntax Tree
///
/// Expressions are language constructs that produce values when evaluated.
/// This includes literals, variables, arithmetic operations, function calls,
/// and more complex expressions. The Expression trait provides methods for
/// inspecting the structure of expressions for code generation and analysis.
pub trait Expression: Node {
    fn expression_node(&self);
    
    /// Returns the node type as a string for compiler dispatch
    fn node_type(&self) -> &str {
        // Default implementation just returns a generic type name
        // Each expression type should override this
        "UnknownExpression"
    }
    
    fn as_any(&self) -> &dyn Any;

    /// Returns true if this expression is a prefix expression
    fn is_prefix_expression(&self) -> bool {
        false
    }

    /// Returns the operator and right expression if this is a prefix expression
    fn as_prefix_expression(&self) -> Option<(String, &dyn Expression)> {
        None
    }

    /// Returns true if this expression is an infix expression
    fn is_infix_expression(&self) -> bool {
        false
    }

    /// Returns the left, operator, and right expressions if this is an infix expression
    fn as_infix_expression(&self) -> Option<(&dyn Expression, String, &dyn Expression)> {
        None
    }

    /// Returns true if this expression is a call expression
    fn is_call_expression(&self) -> bool {
        false
    }

    /// Returns the function and arguments if this is a call expression
    fn as_call_expression(&self) -> Option<(&dyn Expression, Vec<&dyn Expression>)> {
        None
    }

    /// Returns true if this expression is an index expression
    fn is_index_expression(&self) -> bool {
        false
    }

    /// Returns the left and index expressions if this is an index expression
    fn as_index_expression(&self) -> Option<(&dyn Expression, &dyn Expression)> {
        None
    }
    
    /// Returns true if this expression is a property access expression
    fn is_property_expression(&self) -> bool {
        false
    }
    
    /// Returns the object and property if this is a property access expression
    fn as_property_expression(&self) -> Option<(&dyn Expression, &Identifier)> {
        None
    }
}