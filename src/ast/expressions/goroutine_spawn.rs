use std::any::Any;
use crate::ast::traits::{Expression, Node};

#[derive(Debug, Clone)]
pub struct GoroutineSpawn {
impl GoroutineSpawn {
    pub fn new(function_call: Box<dyn Expression>) -> Self {
        Self { function_call }
    }
impl Node for GoroutineSpawn {
    fn string(&self) -> String {
        format!("stan {}", self.function_call.string())
    fn token_literal(&self) -> String {
        "stan".to_string()
    }
}

impl Expression for GoroutineSpawn {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
