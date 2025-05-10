use crate::ast::traits::{Expression, Node};
use std::any::Any;
use std::fmt;

/// Represents an empty expression, used primarily for testing and benchmarking
#[derive(Clone, Debug)]
pub struct Empty;

impl Node for Empty {
    fn token_literal(&self) -> String {
        "EMPTY".to_string()
    }

    fn string(&self) -> String {
        "EMPTY".to_string()
    }
}

impl Expression for Empty {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(Empty)
    }
    
    fn node_type(&self) -> &str {
        "Empty"
    }
}