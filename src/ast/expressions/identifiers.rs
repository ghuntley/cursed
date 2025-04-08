use std::any::Any;
use crate::ast::{Node, Expression};

/// Identifier represents an identifier
#[derive(Clone)]
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
}