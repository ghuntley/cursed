/// Field definitions for struct-like constructs

use crate::ast::traits::Node;
use crate::ast::identifiers::Identifier;

#[derive(Debug, Clone)]
pub struct FieldStatement {
    pub token: String,
    pub name: Identifier,
    pub type_name: Identifier,
}

impl FieldStatement {
    pub fn new(token: String, name: Identifier, type_name: Identifier) -> Self {
        Self { token, name, type_name }
    }
}

impl Node for FieldStatement {
    fn string(&self) -> String {
        format!("{} {}", self.to_string().string(), self.type_name.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: String,
}

impl FieldDefinition {
    pub fn new(name: String, field_type: String) -> Self {
        Self { name, field_type }
    }
}

impl Node for FieldDefinition {
    fn string(&self) -> String {
        format!("{} {}", self.to_string(), self.field_type)
    }

    fn token_literal(&self) -> String {
        self.to_string().clone()
    }
}
