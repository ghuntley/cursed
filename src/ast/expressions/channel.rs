use crate::ast::{Expression, Node};
use std::any::Any;

/// ChannelExpression represents a channel creation expression
pub struct ChannelExpression {
    pub token: String,
    pub element_type: Box<dyn Expression>,
    pub capacity: Option<Box<dyn Expression>>,
}

impl Node for ChannelExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("dm ");
        out.push_str(&self.element_type.string());

        if let Some(cap) = &self.capacity {
            out.push('[');
            out.push_str(&cap.string());
            out.push(']');
        }

        out
    }
}

impl Expression for ChannelExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
