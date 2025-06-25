use std::any::Any;
use crate::ast::traits::{Expression, Node};

#[derive(Debug, Clone)]
pub struct ChannelReceive {
    pub channel: Box<dyn Expression>,
}

impl ChannelReceive {
    pub fn new(channel: Box<dyn Expression>) -> Self {
        Self { channel }
    }
}

impl Node for ChannelReceive {
    fn string(&self) -> String {
        format!("<-{}", self.channel.string())
    }
    
    fn token_literal(&self) -> String {
        "<-".to_string()
    }
}

impl Expression for ChannelReceive {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ChannelSend {
    pub channel: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl ChannelSend {
    pub fn new(channel: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self { channel, value }
    }
}

impl Node for ChannelSend {
    fn string(&self) -> String {
        format!("{} <- {}", self.channel.string(), self.value.string())
    }
    
    fn token_literal(&self) -> String {
        "<-".to_string()
    }
}

impl Expression for ChannelSend {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
