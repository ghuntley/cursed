/// Access expressions for CURSED (indexing, calls, etc.)

use crate::ast::traits::{Node, Expression};
use crate::lexer::Token;
use std::any::Any;

/// Index expression array[index]
#[derive(Debug)]
pub struct IndexExpression {
    pub token: String,
    pub left: Box<dyn Expression>,
    pub index: Box<dyn Expression>,
}

impl IndexExpression {
    pub fn new(token: String, left: Box<dyn Expression>, index: Box<dyn Expression>) -> Self {
        Self { token, left, index }
    }
}

impl Node for IndexExpression {
    fn string(&self) -> String {
        format!("{}[{}]", self.left.string(), self.index.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for IndexExpression {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            left: self.left.clone_box(),
            index: self.index.clone_box(),
        }
    }
}

impl Expression for IndexExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Dot expression obj.field
#[derive(Debug, Clone)]
pub struct DotExpression {
    pub token: String,
    pub left: Box<dyn Expression>,
    pub property: String,
}

impl DotExpression {
    pub fn new(token: String, left: Box<dyn Expression>, property: String) -> Self {
        Self { token, left, property }
    }
}

impl Node for DotExpression {
    fn string(&self) -> String {
        format!("{}.{}", self.left.string(), self.property)
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for DotExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(DotExpression {
            token: self.token.clone(),
            left: self.left.clone_box(),
            property: self.property.clone(),
        })
    }
}
