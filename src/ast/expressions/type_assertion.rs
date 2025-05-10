use crate::ast::traits::{Expression, Node};
use std::any::Any;
use std::fmt;

/// Represents a type assertion expression of the form `expr.(Type)`
/// Used to convert an interface value to a concrete type
// Manually implement Debug since dyn Expression doesn't implement Debug
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

// Manual implementation of Debug for TypeAssertion
impl fmt::Debug for TypeAssertion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypeAssertion {{ token: {:?}, expression: <dyn Expression>, type_name: {:?} }}", 
               self.token, self.type_name)
    }
}

impl Expression for TypeAssertion {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    // Clone implementation for Expression trait
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeAssertion {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
            type_name: self.type_name.clone(),
        })
    }
    
    // Additional trait implementation for type reflection
    fn node_type(&self) -> &str {
        "TypeAssertion"
    }
}