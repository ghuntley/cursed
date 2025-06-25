use std::any::Any;
use crate::ast::traits::{Expression, Node};

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
}

impl Parameter {
    pub fn new(name: String, param_type: String) -> Self {
        Self { name, param_type }
    }
}

impl Node for Parameter {
    fn string(&self) -> String {
        format!("{}: {}", self.name, self.param_type)
    }
    
    fn token_literal(&self) -> String {
        self.name.clone()
    }
}

impl Expression for Parameter {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
