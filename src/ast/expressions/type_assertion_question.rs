use crate::ast::traits::{Expression, Node};
use std::any::Any;
use std::fmt;

/// Represents a type assertion expression with error propagation using `?` operator: `expr.(Type)?`
/// Used to convert an interface value to a concrete type and propagate any errors
pub struct TypeAssertionQuestion {
    pub token: String,       // The token at which this expression starts
    pub expression: Box<dyn Expression>,  // The expression being asserted (interface value)
    pub type_name: String,   // The concrete type being asserted
}

impl Node for TypeAssertionQuestion {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{}.({})?\n", self.expression.string(), self.type_name)
    }
}

// Manual implementation of Debug for TypeAssertionQuestion
impl fmt::Debug for TypeAssertionQuestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypeAssertionQuestion {{ token: {:?}, expression: <dyn Expression>, type_name: {:?} }}", 
               self.token, self.type_name)
    }
}

impl Expression for TypeAssertionQuestion {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    // Clone implementation for Expression trait
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeAssertionQuestion {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
            type_name: self.type_name.clone(),
        })
    }
    
    // Additional trait implementation for type reflection
    fn node_type(&self) -> &str {
        "TypeAssertionQuestion"
    }
}