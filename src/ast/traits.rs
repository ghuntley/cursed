use std::any::Any;
use crate::ast::expressions::Identifier;

/// Node represents a node in the abstract syntax tree
pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

/// Statement represents a statement node in the AST
pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

/// Expression represents an expression node in the AST
pub trait Expression: Node {
    fn expression_node(&self);
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