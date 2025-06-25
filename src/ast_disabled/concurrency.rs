/// Concurrency constructs for CURSED (stan keyword for goroutines)

use crate::ast::traits::{Node, Expression};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct StanExpression {
    pub token: String,
    pub call: Box<dyn Expression>,
}

impl StanExpression {
    pub fn new(token: String, call: Box<dyn Expression>) -> Self {
        Self { token, call }
    }
}

impl Node for StanExpression {
    fn string(&self) -> String {
        format!("stan {}", self.call.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for StanExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(StanExpression {
            token: self.token.clone(),
            call: self.call.clone_box(),
        })
    }
}
