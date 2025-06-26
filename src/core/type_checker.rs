//! Type checking for CURSED

use crate::error_types::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Custom(String),
    Unknown,
}

pub struct TypeChecker {
    types: std::collections::HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            types: std::collections::HashMap::new(),
        }
    }

    pub fn check_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    pub fn add_type(&mut self, name: String, type_def: Type) {
        self.types.insert(name, type_def);
    }

    pub fn type_check(&self, expression: &str) -> Result<Type> {
        // Simple type checking logic
        match expression {
            "true" | "false" => Ok(Type::Bool),
            s if s.starts_with('"') && s.ends_with('"') => Ok(Type::String),
            s if s.parse::<i64>().is_ok() => Ok(Type::Int),
            s if s.parse::<f64>().is_ok() => Ok(Type::Float),
            _ => Ok(Type::Unknown),
        }
    }
}
