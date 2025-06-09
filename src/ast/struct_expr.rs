/// Struct expressions for the CURSED programming language

use crate::ast::traits::{Node, Expression};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct StructLiteral {
    pub token: String,
    pub name: Box<dyn Expression>,
    pub fields: Vec<KeyValuePair>,
}

impl StructLiteral {
    pub fn new(token: String, name: Box<dyn Expression>, fields: Vec<KeyValuePair>) -> Self {
        Self { token, name, fields }
    }
}

impl Node for StructLiteral {
    fn string(&self) -> String {
        let fields: Vec<String> = self.fields.iter()
            .map(|f| f.string())
            .collect();
        format!("{} {{ {} }}", self.name.string(), fields.join(", "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for StructLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(StructLiteral {
            token: self.token.clone(),
            name: self.name.clone_box(),
            fields: self.fields.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeyValuePair {
    pub key: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl KeyValuePair {
    pub fn new(key: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self { key, value }
    }
}

impl Node for KeyValuePair {
    fn string(&self) -> String {
        format!("{}: {}", self.key.string(), self.value.string())
    }

    fn token_literal(&self) -> String {
        self.key.token_literal()
    }
}
