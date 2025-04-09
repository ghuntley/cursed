use crate::ast::{Node, Expression};

/// Property access with dot notation
#[derive(Debug)]
pub struct DotExpression {
    pub token: String,
    pub object: Box<dyn Expression>,
    pub property: String,
}

impl Node for DotExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for DotExpression {
    fn expression_node(&self) {}
    
    fn as_node(&self) -> &dyn Node {
        self
    }
}