use crate::ast::{Expression, Node};
use crate::lexer::token::Token;
use std::any::Any;

/// ChannelExpression represents a channel creation expression
pub struct ChannelExpression {
    pub token: Token,
    pub element_type: Box<dyn Expression>,
    pub capacity: Option<Box<dyn Expression>>,
}

impl Node for ChannelExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("chan {}", self.element_type.string())
    }
}

impl Expression for ChannelExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// SendExpression represents sending a value to a channel
pub struct SendExpression {
    pub token: Token,
    pub channel: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl Node for SendExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("{} <- {}", self.channel.string(), self.value.string())
    }
}

impl Expression for SendExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// ReceiveExpression represents receiving a value from a channel
pub struct ReceiveExpression {
    pub token: Token,
    pub channel: Box<dyn Expression>,
}

impl Node for ReceiveExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("<-{}", self.channel.string())
    }
}

impl Expression for ReceiveExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// StanExpression represents a goroutine creation expression
pub struct StanExpression {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for StanExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("stan {}", self.expression.string())
    }
}

impl Expression for StanExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}
