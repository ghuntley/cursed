use std::any::Any;
use crate::ast::traits::{Expression, Node};
use crate::ast::block::BlockStatement;

#[derive(Debug, Clone)]
pub struct BlockExpression {
    pub block: BlockStatement,
}

impl BlockExpression {
    pub fn new(block: BlockStatement) -> Self {
        Self { block }
    }
}

impl Node for BlockExpression {
    fn string(&self) -> String {
        self.block.string()
    }
    
    fn token_literal(&self) -> String {
        self.block.token_literal()
    }
}

impl Expression for BlockExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
