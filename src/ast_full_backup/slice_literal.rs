/// Slice literal expressions for CURSED

use crate::ast::traits::{Node, Expression};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct SliceLiteral {
impl SliceLiteral {
    pub fn new(token: String, elements: Vec<Box<dyn Expression>>) -> Self {
        Self { token, elements }
    }
    
    pub fn len(&self) -> usize {
        self.elements.len()
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Node for SliceLiteral {
    fn string(&self) -> String {
        let elements: Vec<String> = self.elements.iter()
            .map(|e| e.string())
            .collect();
        format!("[{}]", elements.join(", "))
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for SliceLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(SliceLiteral {
        })
    }
}
