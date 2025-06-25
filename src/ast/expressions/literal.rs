use std::any::Any;
use crate::ast::traits::{Expression, Node};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LiteralValue,
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Self { value }
    }
    
    pub fn integer(value: i64) -> Self {
        Self::new(LiteralValue::Integer(value))
    }
    
    pub fn float(value: f64) -> Self {
        Self::new(LiteralValue::Float(value))
    }
    
    pub fn string(value: String) -> Self {
        Self::new(LiteralValue::String(value))
    }
    
    pub fn boolean(value: bool) -> Self {
        Self::new(LiteralValue::Boolean(value))
    }
    
    pub fn nil() -> Self {
        Self::new(LiteralValue::Nil)
    }
}

impl Node for Literal {
    fn string(&self) -> String {
        match &self.value {
            LiteralValue::Integer(i) => i.to_string(),
            LiteralValue::Float(f) => f.to_string(),
            LiteralValue::String(s) => format!("\"{}\"", s),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Nil => "no_cap".to_string(),
        }
    }
    
    fn token_literal(&self) -> String {
        self.string()
    }
}

impl Expression for Literal {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
