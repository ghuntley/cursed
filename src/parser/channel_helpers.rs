//! Helper functions for channel-related parsing
//!
//! This module provides utility functions for parsing and creating
//! channel-related expressions and AST nodes.

use crate::ast::expressions::channel::*;
use crate::lexer::token::Token;
use crate::ast::traits::Expression;

/// Create a new channel expression
pub fn create_channel_expression(
    token: Token, 
    element_type: Box<dyn Expression>,
    capacity: Option<Box<dyn Expression>>
) -> ChannelExpression {
    // Extract a string representation for the element type
    let type_str = match element_type.string().as_str() {
        // Handle common types
        "int" => "int".to_string(),
        "string" => "string".to_string(),
        "bool" => "bool".to_string(),
        // Default case - just use the string representation
        _ => element_type.string(),
    };
    
    ChannelExpression {
        token,
        element_type: type_str,
        capacity,
    }
}

/// Create a send expression
pub fn create_send_expression(
    token: Token,
    channel: Box<dyn Expression>,
    value: Box<dyn Expression>
) -> SendExpression {
    SendExpression {
        token,
        channel,
        value,
    }
}

/// Create a receive expression
pub fn create_receive_expression(
    token: Token,
    channel: Box<dyn Expression>,
    element_type: String,
) -> ReceiveExpression {
    ReceiveExpression {
        token,
        channel,
        element_type,
    }
}