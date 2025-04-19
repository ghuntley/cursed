use crate::ast::traits::{Expression, Node};
use std::any::Any;

/// Represents a type assertion expression of the form `expr.(Type)`
/// Used to convert an interface value to a concrete type
pub struct TypeAssertion {
    pub token: String,       // The token at which this expression starts
    pub expression: Box<dyn Expression>,  // The expression being asserted (interface value)
    pub type_name: String,   // The concrete type being asserted
}

impl Node for TypeAssertion {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{}.({})", self.expression.string(), self.type_name)
    }
}

impl Expression for TypeAssertion {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    // Additional trait implementation for type reflection
    fn node_type(&self) -> &str {
        "TypeAssertion"
    }
}