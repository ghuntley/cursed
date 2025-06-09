/// Object system for CURSED
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Object>),
    Map(HashMap<String, Object>),
    Nil,
}

impl Object {
    pub fn type_name(&self) -> &'static str {
        match self {
            Object::Integer(_) => "integer",
            Object::Float(_) => "float",
            Object::String(_) => "string",
            Object::Boolean(_) => "boolean",
            Object::Array(_) => "array",
            Object::Map(_) => "map",
            Object::Nil => "nil",
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(b) => *b,
            Object::Nil => false,
            _ => true,
        }
    }
}
