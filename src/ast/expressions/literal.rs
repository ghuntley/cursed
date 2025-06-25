use std::any::Any;
use crate::ast::traits::{Expression, Node};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
#[derive(Debug, Clone)]
pub struct Literal {
impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Self { value }
    }
    
    pub fn integer(value: i64) -> Self {
        Self::new(LiteralValue::Integer(value))
    pub fn float(value: f64) -> Self {
        Self::new(LiteralValue::Float(value))
    pub fn string(value: String) -> Self {
        Self::new(LiteralValue::String(value))
    pub fn boolean(value: bool) -> Self {
        Self::new(LiteralValue::Boolean(value))
    pub fn nil() -> Self {
        Self::new(LiteralValue::Nil)
    }
}

impl Node for Literal {
    fn string(&self) -> String {
        match &self.value {
        }
    }
    
    fn token_literal(&self) -> String {
        self.string()
    }
}

impl Expression for Literal {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
