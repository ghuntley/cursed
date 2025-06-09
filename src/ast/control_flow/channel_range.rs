//! Channel range operations for concurrent iteration over channels
//!
//! This module defines AST nodes for channel range operations, which allow
//! for-range loops to iterate over values received from a channel until the
//! channel is closed.

use crate::ast::{Expression, Node, Statement};
use crate::ast::statements::block::BlockStatement;
use crate::lexer::Token;
use std::any::Any;

/// Represents a channel range clause for iterating over channel values
///
/// Channel range clauses are used with for loops to iterate over values
/// received from a channel. The iteration continues until the channel is
/// closed, providing a natural way to process all values sent to a channel.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// bestie value := flex <-ch {
///     vibez.println(value)
/// }
/// 
/// bestie value, ok := flex <-ch {
///     if !ok {
///         // Channel is closed
///         break
///     }
///     processValue(value)
/// }
/// ```
///
/// The AST would contain a `ChannelRangeClause` representing the channel
/// iteration expression `<-ch`.
#[derive(Debug)]
pub struct ChannelRangeClause {
    pub token: Token,                      // Token::Flex
    pub channel: Box<dyn Expression>,      // Channel expression to iterate over
    pub with_ok: bool,                     // Whether to include the 'ok' closure status flag
}

impl Node for ChannelRangeClause {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("flex <-{}", self.channel.string())
    }
}

impl Expression for ChannelRangeClause {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ChannelRangeClause {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
            with_ok: self.with_ok,
        })
    }
}

/// Represents a channel range for statement in the AST
///
/// A channel range for loop iterates over values received from a channel
/// until the channel is closed. It can appear in two forms:
///
/// 1. Value-only iteration: `bestie value := flex <-ch { ... }`
///    - Receives values until channel is closed
///    - Loop terminates when channel is closed
///
/// 2. Value with ok flag: `bestie value, ok := flex <-ch { ... }`
///    - Receives values and channel status
///    - `ok` is true for valid values, false when channel is closed
///    - Allows manual handling of channel closure
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// // Simple channel iteration
/// bestie message := flex <-messageChannel {
///     handleMessage(message)
/// }
///
/// // Channel iteration with closure detection
/// bestie data, ok := flex <-dataChannel {
///     if !ok {
///         vibez.println("Channel closed")
///         break
///     }
///     processData(data)
/// }
/// ```
pub struct ChannelRangeForStatement {
    pub token: Token,                                 // Token::Bestie
    pub value_var: String,                           // Variable for received values
    pub ok_var: Option<String>,                      // Optional variable for channel status
    pub channel_range: Box<ChannelRangeClause>,     // Channel range expression
    pub body: Box<BlockStatement>,                   // Loop body
}

impl Node for ChannelRangeForStatement {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("bestie ");
        
        out.push_str(&self.value_var);
        
        if let Some(ok_var) = &self.ok_var {
            out.push_str(", ");
            out.push_str(ok_var);
        }
        
        out.push_str(" := ");
        out.push_str(&self.channel_range.string());
        out.push_str(" ");
        out.push_str(&self.body.string());
        
        out
    }
}

impl Statement for ChannelRangeForStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Represents channel closure detection in range operations
///
/// This expression provides runtime information about whether a channel
/// receive operation succeeded or if the channel was closed.
#[derive(Debug)]
pub struct ChannelClosureDetection {
    pub token: Token,                      // Token for the detection expression
    pub channel: Box<dyn Expression>,     // Channel being checked
}

impl Node for ChannelClosureDetection {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("closed({})", self.channel.string())
    }
}

impl Expression for ChannelClosureDetection {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ChannelClosureDetection {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
        })
    }
}
