/// Comprehensive Channel AST Nodes for CURSED Programming Language
///
/// This module implements complete AST node support for channel operations in CURSED,
/// enabling concurrent programming with goroutines and channels. Channel operations
/// are fundamental for concurrent programming patterns in CURSED.
///
/// # Why Channel AST Nodes are Critical for Concurrency
///
/// Channel AST nodes form the foundation of CURSED's concurrency model, which is
/// essential for modern applications requiring parallel processing, asynchronous
/// operations, and distributed system communication patterns.
///
/// ## 1. Concurrency Model Foundation
/// - **Communication**: Channels provide type-safe message passing between goroutines
/// - **Synchronization**: Unbuffered channels provide synchronization points
/// - **Buffering**: Buffered channels enable asynchronous communication patterns
/// - **Composability**: Channel operations can be composed into complex concurrent workflows
///
/// ## 2. Language Feature Support
/// - **Goroutine Spawning**: `stan` keyword integration for concurrent execution
/// - **Channel Send**: `ch <- value` syntax for sending values
/// - **Channel Receive**: `<-ch` syntax for receiving values  
/// - **Select Operations**: `vibe_check` statements for multi-channel coordination
/// - **Range Operations**: `flex <-ch` for iterating over channel values
/// - **Channel Types**: `dm<Type>` syntax for channel type declarations
///
/// ## 3. Performance and Safety
/// - **Type Safety**: Statically typed channels prevent runtime type errors
/// - **Memory Safety**: Proper channel lifecycle management prevents leaks
/// - **Performance**: Efficient compilation to underlying runtime primitives
/// - **Deadlock Prevention**: Static analysis opportunities for detecting channel misuse
///
/// ## 4. Integration Requirements
/// - **Parser Integration**: Channel syntax must be properly parsed into AST nodes
/// - **Type Checking**: Channel types must integrate with the type system
/// - **Code Generation**: AST nodes must compile to efficient LLVM IR
/// - **Runtime Support**: Integration with goroutine scheduler and GC system
///
/// ## 5. Concurrent Programming Patterns
/// Channel AST nodes enable critical concurrent programming patterns:
/// - Producer-consumer patterns with buffered channels
/// - Fan-in/fan-out patterns with multiple channel coordination
/// - Select-based multiplexing for handling multiple channel operations
/// - Pipeline patterns for data processing workflows
/// - Worker pool patterns for parallel task distribution
/// - Rate limiting and backpressure handling

use std::any::Any;
use crate::ast::traits::{Expression, Statement, Node, TypeNode};
use crate::lexer::Token;

/// Channel type declaration (dm<Type> or dm<Type>, capacity)
/// Represents channel types in variable declarations and function signatures
#[derive(Debug, Clone)]
pub struct ChannelType {
    pub token: Token,                    // The 'dm' token
    pub element_type: Box<dyn TypeNode>, // Type of elements in the channel
    pub is_buffered: bool,               // Whether this is a buffered channel
    pub buffer_capacity: Option<Box<dyn Expression>>, // Buffer size for buffered channels
}

impl ChannelType {
    pub fn new(token: Token, element_type: Box<dyn TypeNode>) -> Self {
        Self {
            token,
            element_type,
            is_buffered: false,
            buffer_capacity: None,
        }
    }
    
    pub fn new_buffered(token: Token, element_type: Box<dyn TypeNode>, capacity: Box<dyn Expression>) -> Self {
        Self {
            token,
            element_type,
            is_buffered: true,
            buffer_capacity: Some(capacity),
        }
    }
}

impl Node for ChannelType {
    fn string(&self) -> String {
        if self.is_buffered {
            if let Some(capacity) = &self.buffer_capacity {
                format!("dm<{}, {}>", self.element_type.string(), capacity.string())
            } else {
                format!("dm<{}>", self.element_type.string())
            }
        } else {
            format!("dm<{}>", self.element_type.string())
        }
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl TypeNode for ChannelType {
    fn type_name(&self) -> String {
        format!("channel<{}>", self.element_type.type_name())
    }
    
    fn is_generic(&self) -> bool {
        // Channel type is generic if its element type is generic
        self.element_type.is_generic()
    }
    
    fn size_hint(&self) -> Option<usize> {
        // Channels are reference types, so size is pointer size
        Some(std::mem::size_of::<*const u8>())
    }
}

/// Channel creation expression (make(dm<Type>) or make(dm<Type>, capacity))
/// Used for creating new channels with optional buffer capacity
#[derive(Debug, Clone)]
pub struct ChannelMake {
    pub token: Token,                                 // The 'make' token
    pub channel_type: ChannelType,                   // Type specification
    pub capacity: Option<Box<dyn Expression>>,       // Optional buffer capacity
}

impl ChannelMake {
    pub fn new(token: Token, channel_type: ChannelType) -> Self {
        Self {
            token,
            channel_type,
            capacity: None,
        }
    }
    
    pub fn new_with_capacity(token: Token, channel_type: ChannelType, capacity: Box<dyn Expression>) -> Self {
        Self {
            token,
            channel_type,
            capacity: Some(capacity),
        }
    }
}

impl Node for ChannelMake {
    fn string(&self) -> String {
        if let Some(capacity) = &self.capacity {
            format!("make({}, {})", self.channel_type.string(), capacity.string())
        } else {
            format!("make({})", self.channel_type.string())
        }
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for ChannelMake {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Channel send operation (channel <- value)
/// Represents sending a value to a channel
#[derive(Debug, Clone)]
pub struct ChannelSend {
    pub token: Token,                    // The '<-' token
    pub channel: Box<dyn Expression>,   // Channel expression
    pub value: Box<dyn Expression>,     // Value to send
}

impl ChannelSend {
    pub fn new(token: Token, channel: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self {
            token,
            channel,
            value,
        }
    }
}

impl Node for ChannelSend {
    fn string(&self) -> String {
        format!("{} <- {}", self.channel.string(), self.value.string())
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
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

impl Statement for ChannelSend {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Channel receive operation (<-channel)
/// Represents receiving a value from a channel
#[derive(Debug, Clone)]
pub struct ChannelReceive {
    pub token: Token,                    // The '<-' token
    pub channel: Box<dyn Expression>,   // Channel expression
}

impl ChannelReceive {
    pub fn new(token: Token, channel: Box<dyn Expression>) -> Self {
        Self {
            token,
            channel,
        }
    }
}

impl Node for ChannelReceive {
    fn string(&self) -> String {
        format!("<-{}", self.channel.string())
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
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

/// Channel receive with ok check (value, ok := <-channel)
/// Represents receiving a value with success/closed status
#[derive(Debug, Clone)]
pub struct ChannelReceiveOk {
    pub token: Token,                    // The '<-' token
    pub channel: Box<dyn Expression>,   // Channel expression
    pub value_name: String,              // Variable name for the value
    pub ok_name: String,                 // Variable name for the ok flag
}

impl ChannelReceiveOk {
    pub fn new(token: Token, channel: Box<dyn Expression>, value_name: String, ok_name: String) -> Self {
        Self {
            token,
            channel,
            value_name,
            ok_name,
        }
    }
}

impl Node for ChannelReceiveOk {
    fn string(&self) -> String {
        format!("{}, {} := <-{}", self.value_name, self.ok_name, self.channel.string())
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ChannelReceiveOk {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Channel close operation (close(channel))
/// Represents closing a channel to signal no more values
#[derive(Debug, Clone)]
pub struct ChannelClose {
    pub token: Token,                    // The 'close' token
    pub channel: Box<dyn Expression>,   // Channel expression
}

impl ChannelClose {
    pub fn new(token: Token, channel: Box<dyn Expression>) -> Self {
        Self {
            token,
            channel,
        }
    }
}

impl Node for ChannelClose {
    fn string(&self) -> String {
        format!("close({})", self.channel.string())
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for ChannelClose {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Statement for ChannelClose {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Channel length operation (len(channel))
/// Gets the number of elements currently in the channel buffer
#[derive(Debug, Clone)]
pub struct ChannelLen {
    pub token: Token,                    // The 'len' token
    pub channel: Box<dyn Expression>,   // Channel expression
}

impl ChannelLen {
    pub fn new(token: Token, channel: Box<dyn Expression>) -> Self {
        Self {
            token,
            channel,
        }
    }
}

impl Node for ChannelLen {
    fn string(&self) -> String {
        format!("len({})", self.channel.string())
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for ChannelLen {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Channel capacity operation (cap(channel))
/// Gets the buffer capacity of the channel
#[derive(Debug, Clone)]
pub struct ChannelCap {
    pub token: Token,                    // The 'cap' token
    pub channel: Box<dyn Expression>,   // Channel expression
}

impl ChannelCap {
    pub fn new(token: Token, channel: Box<dyn Expression>) -> Self {
        Self {
            token,
            channel,
        }
    }
}

impl Node for ChannelCap {
    fn string(&self) -> String {
        format!("cap({})", self.channel.string())
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for ChannelCap {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Select statement case for channel operations (mood <-ch: or mood ch <- value:)
/// Represents a single case in a select statement
#[derive(Debug, Clone)]
pub enum SelectCase {
    /// Receive case: mood value := <-channel:
    Receive {
        token: Token,
        channel: Box<dyn Expression>,
        variable: Option<String>,          // Optional variable name
    },
    /// Receive with ok case: mood value, ok := <-channel:
    ReceiveOk {
        token: Token,
        channel: Box<dyn Expression>,
        value_var: String,
        ok_var: String,
    },
    /// Send case: mood channel <- value:
    Send {
        token: Token,
        channel: Box<dyn Expression>,
        value: Box<dyn Expression>,
    },
    /// Default case: basic:
    Default {
        token: Token,
    },
}

impl SelectCase {
    pub fn new_receive(token: Token, channel: Box<dyn Expression>, variable: Option<String>) -> Self {
        SelectCase::Receive {
            token,
            channel,
            variable,
        }
    }
    
    pub fn new_receive_ok(token: Token, channel: Box<dyn Expression>, value_var: String, ok_var: String) -> Self {
        SelectCase::ReceiveOk {
            token,
            channel,
            value_var,
            ok_var,
        }
    }
    
    pub fn new_send(token: Token, channel: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        SelectCase::Send {
            token,
            channel,
            value,
        }
    }
    
    pub fn new_default(token: Token) -> Self {
        SelectCase::Default {
            token,
        }
    }
}

impl Node for SelectCase {
    fn string(&self) -> String {
        match self {
            SelectCase::Receive { channel, variable, .. } => {
                if let Some(var) = variable {
                    format!("mood {} := <-{}:", var, channel.string())
                } else {
                    format!("mood <-{}:", channel.string())
                }
            }
            SelectCase::ReceiveOk { channel, value_var, ok_var, .. } => {
                format!("mood {}, {} := <-{}:", value_var, ok_var, channel.string())
            }
            SelectCase::Send { channel, value, .. } => {
                format!("mood {} <- {}:", channel.string(), value.string())
            }
            SelectCase::Default { .. } => {
                "basic:".to_string()
            }
        }
    }
    
    fn token_literal(&self) -> String {
        match self {
            SelectCase::Receive { token, .. } |
            SelectCase::ReceiveOk { token, .. } |
            SelectCase::Send { token, .. } |
            SelectCase::Default { token } => token.literal.clone(),
        }
    }
}

/// Select statement (vibe_check { mood ... })
/// Represents a select statement for multi-channel operations
#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub token: Token,                              // The 'vibe_check' token
    pub cases: Vec<SelectCase>,                    // List of select cases
    pub case_bodies: Vec<Vec<Box<dyn Statement>>>, // Corresponding case bodies
}

impl SelectStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            cases: Vec::new(),
            case_bodies: Vec::new(),
        }
    }
    
    pub fn add_case(&mut self, case: SelectCase, body: Vec<Box<dyn Statement>>) {
        self.cases.push(case);
        self.case_bodies.push(body);
    }
}

impl Node for SelectStatement {
    fn string(&self) -> String {
        let mut result = String::from("vibe_check {\n");
        
        for (case, body) in self.cases.iter().zip(self.case_bodies.iter()) {
            result.push_str(&format!("    {}\n", case.string()));
            for stmt in body {
                result.push_str(&format!("        {}\n", stmt.string()));
            }
        }
        
        result.push('}');
        result
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for SelectStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Channel range operation (flex value := <-channel)
/// Represents iterating over values received from a channel
#[derive(Debug, Clone)]  
pub struct ChannelRange {
    pub token: Token,                    // The 'flex' token
    pub variable: String,                // Variable name for received values
    pub channel: Box<dyn Expression>,   // Channel expression
    pub body: Vec<Box<dyn Statement>>,   // Loop body statements
}

impl ChannelRange {
    pub fn new(token: Token, variable: String, channel: Box<dyn Expression>) -> Self {
        Self {
            token,
            variable,
            channel,
            body: Vec::new(),
        }
    }
    
    pub fn with_body(token: Token, variable: String, channel: Box<dyn Expression>, body: Vec<Box<dyn Statement>>) -> Self {
        Self {
            token,
            variable,
            channel,
            body,
        }
    }
}

impl Node for ChannelRange {
    fn string(&self) -> String {
        let mut result = format!("flex {} := <-{} {{\n", self.variable, self.channel.string());
        
        for stmt in &self.body {
            result.push_str(&format!("    {}\n", stmt.string()));
        }
        
        result.push('}');
        result
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for ChannelRange {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Goroutine spawn with channel communication (stan { ... })
/// Represents spawning a goroutine that typically involves channel operations
#[derive(Debug, Clone)]
pub struct GoroutineSpawn {
    pub token: Token,                         // The 'stan' token
    pub body: Vec<Box<dyn Statement>>,        // Goroutine body statements
    pub captures: Vec<String>,                // Variables captured by the goroutine
}

impl GoroutineSpawn {
    pub fn new(token: Token, body: Vec<Box<dyn Statement>>) -> Self {
        Self {
            token,
            body,
            captures: Vec::new(),
        }
    }
    
    pub fn with_captures(token: Token, body: Vec<Box<dyn Statement>>, captures: Vec<String>) -> Self {
        Self {
            token,
            body,
            captures,
        }
    }
}

impl Node for GoroutineSpawn {
    fn string(&self) -> String {
        let mut result = String::from("stan {\n");
        
        for stmt in &self.body {
            result.push_str(&format!("    {}\n", stmt.string()));
        }
        
        result.push('}');
        result
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for GoroutineSpawn {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Statement for GoroutineSpawn {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

/// Channel timeout operation using time.After
/// Represents creating a timeout channel for select operations
#[derive(Debug, Clone)]
pub struct ChannelTimeout {
    pub token: Token,                    // The function token (time.After)
    pub duration: Box<dyn Expression>,  // Duration expression
}

impl ChannelTimeout {
    pub fn new(token: Token, duration: Box<dyn Expression>) -> Self {
        Self {
            token,
            duration,
        }
    }
}

impl Node for ChannelTimeout {
    fn string(&self) -> String {
        format!("time.After({})", self.duration.string())
    }
    
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for ChannelTimeout {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;
    use crate::ast::crate::types::TypeExpression;
    use crate::ast::literals::IntegerLiteral;
    use crate::ast::identifiers::Identifier;
    use crate::ast::statements::ExpressionStatement;
    
    fn create_test_token(literal: &str) -> Token {
        Token {
            token_type: crate::lexer::TokenType::Identifier,
            literal: literal.to_string(),
            line: 1,
            column: 1,
        }
    }
    
    fn create_test_type() -> Box<dyn TypeNode> {
        Box::new(TypeExpression::new("tea".to_string(), "tea".to_string()))
    }
    
    fn create_test_expression() -> Box<dyn Expression> {
        Box::new(Identifier::new(create_test_token("test"), "test".to_string()))
    }
    
    #[test]
    fn test_channel_type_creation() {
        let token = create_test_token("dm");
        let element_type = create_test_type();
        let channel_type = ChannelType::new(token, element_type);
        
        assert_eq!(channel_type.string(), "dm<tea>");
        assert_eq!(channel_type.type_name(), "channel<tea>");
        assert!(!channel_type.is_buffered);
        assert!(channel_type.buffer_capacity.is_none());
    }
    
    #[test]
    fn test_buffered_channel_type() {
        let token = create_test_token("dm");
        let element_type = create_test_type();
        let capacity = Box::new(IntegerLiteral::new("10".to_string(), 10));
        let channel_type = ChannelType::new_buffered(token, element_type, capacity);
        
        assert_eq!(channel_type.string(), "dm<tea, 10>");
        assert!(channel_type.is_buffered);
        assert!(channel_type.buffer_capacity.is_some());
    }
    
    #[test]
    fn test_channel_make_expression() {
        let token = create_test_token("make");
        let channel_type = ChannelType::new(create_test_token("dm"), create_test_type());
        let make_expr = ChannelMake::new(token, channel_type);
        
        assert_eq!(make_expr.string(), "make(dm<tea>)");
        assert_eq!(make_expr.token_literal(), "make");
    }
    
    #[test]
    fn test_channel_send_operation() {
        let token = create_test_token("<-");
        let channel = create_test_expression();
        let value = create_test_expression();
        let send_op = ChannelSend::new(token, channel, value);
        
        assert_eq!(send_op.string(), "test <- test");
        assert_eq!(send_op.token_literal(), "<-");
    }
    
    #[test]
    fn test_channel_receive_operation() {
        let token = create_test_token("<-");
        let channel = create_test_expression();
        let receive_op = ChannelReceive::new(token, channel);
        
        assert_eq!(receive_op.string(), "<-test");
        assert_eq!(receive_op.token_literal(), "<-");
    }
    
    #[test]
    fn test_channel_receive_ok_operation() {
        let token = create_test_token("<-");
        let channel = create_test_expression();
        let receive_ok = ChannelReceiveOk::new(token, channel, "value".to_string(), "ok".to_string());
        
        assert_eq!(receive_ok.string(), "value, ok := <-test");
        assert_eq!(receive_ok.token_literal(), "<-");
    }
    
    #[test]
    fn test_channel_close_operation() {
        let token = create_test_token("close");
        let channel = create_test_expression();
        let close_op = ChannelClose::new(token, channel);
        
        assert_eq!(close_op.string(), "close(test)");
        assert_eq!(close_op.token_literal(), "close");
    }
    
    #[test]
    fn test_select_case_creation() {
        let token = create_test_token("mood");
        let channel = create_test_expression();
        
        // Test receive case
        let receive_case = SelectCase::new_receive(token.clone(), channel.clone(), Some("value".to_string()));
        assert_eq!(receive_case.string(), "mood value := <-test:");
        
        // Test send case
        let value = create_test_expression();
        let send_case = SelectCase::new_send(token.clone(), channel, value);
        assert_eq!(send_case.string(), "mood test <- test:");
        
        // Test default case
        let default_case = SelectCase::new_default(token);
        assert_eq!(default_case.string(), "basic:");
    }
    
    #[test]
    fn test_select_statement() {
        let token = create_test_token("vibe_check");
        let mut select_stmt = SelectStatement::new(token);
        
        let case = SelectCase::new_default(create_test_token("basic"));
        let body = vec![Box::new(ExpressionStatement::new(
            "test".to_string(),
            create_test_expression(),
        )) as Box<dyn Statement>];
        
        select_stmt.add_case(case, body);
        
        let result = select_stmt.string();
        assert!(result.contains("vibe_check {"));
        assert!(result.contains("basic:"));
        assert!(result.contains("}"));
    }
    
    #[test]
    fn test_channel_range_operation() {
        let token = create_test_token("flex");
        let channel = create_test_expression();
        let range_op = ChannelRange::new(token, "value".to_string(), channel);
        
        assert_eq!(range_op.string(), "flex value := <-test {\n}");
        assert_eq!(range_op.token_literal(), "flex");
    }
    
    #[test]
    fn test_goroutine_spawn() {
        let token = create_test_token("stan");
        let body = vec![Box::new(ExpressionStatement::new(
            "test".to_string(),
            create_test_expression(),
        )) as Box<dyn Statement>];
        
        let goroutine = GoroutineSpawn::new(token, body);
        
        let result = goroutine.string();
        assert!(result.contains("stan {"));
        assert!(result.contains("}"));
        assert_eq!(goroutine.token_literal(), "stan");
    }
    
    #[test]
    fn test_channel_timeout() {
        let token = create_test_token("time.After");
        let duration = Box::new(IntegerLiteral::new("1000".to_string(), 1000));
        let timeout = ChannelTimeout::new(token, duration);
        
        assert_eq!(timeout.string(), "time.After(1000)");
        assert_eq!(timeout.token_literal(), "time.After");
    }
    
    #[test]
    fn test_channel_len_and_cap() {
        let channel = create_test_expression();
        
        let len_op = ChannelLen::new(create_test_token("len"), channel.clone());
        assert_eq!(len_op.string(), "len(test)");
        
        let cap_op = ChannelCap::new(create_test_token("cap"), channel);
        assert_eq!(cap_op.string(), "cap(test)");
    }
}
