//! Channel expressions for concurrent communication.
//!
//! This module defines AST nodes for channel operations including
//! channel creation, sending to channels, and receiving from channels.

use crate::ast::traits::{Node, Expression};
use crate::lexer::Token;
use std::any::Any;

/// A channel expression (dm expression)
// No derives for now due to trait object issues
pub struct ChannelExpression {
    pub token: Token,
    pub element_type: String,
    pub capacity: Option<Box<dyn Expression>>,
}

impl Node for ChannelExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        if let Some(cap) = &self.capacity {
            format!("dm[{}]({})", self.element_type, cap.string())
        } else {
            format!("dm[{}]", self.element_type)
        }
    }
}

impl Expression for ChannelExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ChannelExpression {
            token: self.token.clone(),
            element_type: self.element_type.clone(),
            capacity: self.capacity.as_ref().map(|cap| cap.clone_box()),
        })
    }
}

/// A send expression (ch <- value)
// No derives for now due to trait object issues
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
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(SendExpression {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
            value: self.value.clone_box(),
        })
    }
}

/// A receive expression (<-ch)
// No derives for now due to trait object issues
pub struct ReceiveExpression {
    pub token: Token,
    pub channel: Box<dyn Expression>,
    pub element_type: String, // This field holds the type of elements in the channel
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
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ReceiveExpression {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
            element_type: self.element_type.clone(),
        })
    }
}