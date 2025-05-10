use crate::ast::expressions::Identifier;
use crate::ast::traits::{Expression, Node};
use std::any::Any;

/// TypeConstraint represents a constraint on a generic type parameter
/// In CURSED syntax: `where T: Comparable`
pub struct TypeConstraint {
    pub token: String,       // The 'where' token
    pub type_param: Identifier, // The type parameter being constrained
    pub interface: Identifier,  // The interface that must be implemented
}

impl Node for TypeConstraint {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {}: {}", self.token, self.type_param.string(), self.interface.string())
    }
}

impl Expression for TypeConstraint {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeConstraint {
            token: self.token.clone(),
            type_param: self.type_param.clone(),
            interface: self.interface.clone(),
        })
    }
}