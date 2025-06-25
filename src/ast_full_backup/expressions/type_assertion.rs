/// Type assertion expression AST node for CURSED language
/// Represents expressions like `value.(Type)` for type assertions

use crate::ast::traits::{Expression, Node};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct TypeAssertion {
    pub expression_text: String,
    pub target_type_text: String,
}

impl TypeAssertion {
    pub fn new(expression_text: String, target_type_text: String) -> Self {
        Self {
            expression_text,
            target_type_text,
        }
    }
}

impl Node for TypeAssertion {
    fn string(&self) -> String {
        format!("{}.({})", self.expression_text, self.target_type_text)
    }
    
    fn token_literal(&self) -> String {
        self.string()
    }
}

impl Expression for TypeAssertion {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
