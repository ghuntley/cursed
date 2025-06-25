// Channel-related statements for CURSED language

use std::any::Any;
use crate::ast::traits::{Node, Statement, Expression};
use crate::error::SourceLocation;

#[derive(Debug, Clone)]
pub struct ChannelReceiveStatement {
    pub target: Box<dyn Expression>,
    pub channel: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl ChannelReceiveStatement {
    pub fn new(target: Box<dyn Expression>, channel: Box<dyn Expression>) -> Self {
        Self {
            target,
            channel,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ChannelReceiveStatement {
    fn string(&self) -> String {
        format!("{} <- {};", self.target.string(), self.channel.string())
    }
    
    fn token_literal(&self) -> String {
        "<-".to_string()
    }
}

impl Statement for ChannelReceiveStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ChannelSendStatement {
    pub channel: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl ChannelSendStatement {
    pub fn new(channel: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self {
            channel,
            value,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ChannelSendStatement {
    fn string(&self) -> String {
        format!("{} <- {};", self.channel.string(), self.value.string())
    }
    
    fn token_literal(&self) -> String {
        "<-".to_string()
    }
}

impl Statement for ChannelSendStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct ChannelCloseStatement {
    pub channel: Box<dyn Expression>,
    pub location: SourceLocation,
}

impl ChannelCloseStatement {
    pub fn new(channel: Box<dyn Expression>) -> Self {
        Self {
            channel,
            location: SourceLocation::default(),
        }
    }
}

impl Node for ChannelCloseStatement {
    fn string(&self) -> String {
        format!("close({});", self.channel.string())
    }
    
    fn token_literal(&self) -> String {
        "close".to_string()
    }
}

impl Statement for ChannelCloseStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
