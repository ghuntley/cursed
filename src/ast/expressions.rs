/// Additional expression types for the CURSED programming language
/// 
/// This module contains expression types that don't fit into other categories.

use crate::ast::traits::{Node, Expression};
use crate::lexer::Token;
use std::any::Any;

/// Parenthesized expression ((expression))
#[derive(Debug, Clone)]
pub struct ParenthesizedExpression {
    pub token: String,
    pub expression: Box<dyn Expression>,
}

impl ParenthesizedExpression {
    pub fn new(token: String, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl Node for ParenthesizedExpression {
    fn string(&self) -> String {
        format!("({})", self.expression.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ParenthesizedExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ParenthesizedExpression {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
        })
    }
}

/// Function literal/lambda expression
#[derive(Debug)]
pub struct FunctionLiteral {
    pub token: String,
    pub parameters: Vec<Parameter>,
    pub body: crate::ast::block::BlockStatement,
    pub return_type: Option<Box<dyn Expression>>,
}

impl FunctionLiteral {
    pub fn new(
        token: String,
        parameters: Vec<Parameter>,
        body: crate::ast::block::BlockStatement,
        return_type: Option<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            parameters,
            body,
            return_type,
        }
    }
}

impl Node for FunctionLiteral {
    fn string(&self) -> String {
        let params: Vec<String> = self.parameters.iter()
            .map(|p| p.string())
            .collect();
        
        let mut result = format!("slay({})", params.join(", "));
        
        if let Some(ret_type) = &self.return_type {
            result.push_str(&format!(" {}", ret_type.string()));
        }
        
        result.push(' ');
        result.push_str(&self.body.string());
        
        result
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for FunctionLiteral {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            parameters: self.parameters.clone(),
            body: self.body.clone(),
            return_type: self.return_type.as_ref().map(|t| t.clone_box()),
        }
    }
}

impl Expression for FunctionLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Parameter in function signatures
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
}

impl Parameter {
    pub fn new(name: String, param_type: String) -> Self {
        Self { name, param_type }
    }
    
    pub fn untyped(name: &str) -> Self {
        Self {
            name: name.to_string(),
            param_type: String::new(),
        }
    }
    
    pub fn typed(name: &str, param_type: &str) -> Self {
        Self {
            name: name.to_string(),
            param_type: param_type.to_string(),
        }
    }
}

impl Node for Parameter {
    fn string(&self) -> String {
        if !self.param_type.is_empty() {
            format!("{} {}", self.name, self.param_type)
        } else {
            self.name.clone()
        }
    }

    fn token_literal(&self) -> String {
        self.name.clone()
    }
}

/// Literal value enum for type-safe literal handling
#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
    Character(char),
}

/// Generic literal expression
#[derive(Debug, Clone)]
pub struct Literal {
    pub token: String,
    pub value: LiteralValue,
}

impl Literal {
    pub fn new(token: String, value: LiteralValue) -> Self {
        Self { token, value }
    }
    
    pub fn integer(value: i64) -> Self {
        Self {
            token: value.to_string(),
            value: LiteralValue::Integer(value),
        }
    }
    
    pub fn string(value: &str) -> Self {
        Self {
            token: format!("\"{}\"", value),
            value: LiteralValue::String(value.to_string()),
        }
    }
    
    pub fn boolean(value: bool) -> Self {
        Self {
            token: if value { "based".to_string() } else { "cap".to_string() },
            value: LiteralValue::Boolean(value),
        }
    }
    
    pub fn nil() -> Self {
        Self {
            token: "cap".to_string(),
            value: LiteralValue::Nil,
        }
    }
}

impl Node for Literal {
    fn string(&self) -> String {
        match &self.value {
            LiteralValue::Integer(i) => i.to_string(),
            LiteralValue::Float(f) => f.to_string(),
            LiteralValue::String(s) => format!("\"{}\"", s),
            LiteralValue::Boolean(b) => if *b { "based".to_string() } else { "cap".to_string() },
            LiteralValue::Nil => "cap".to_string(),
            LiteralValue::Character(c) => format!("'{}'", c),
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for Literal {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Error propagation expression using the ? operator
#[derive(Debug, Clone)]
pub struct ErrorPropagation {
    pub token: String,
    pub expression: Box<dyn Expression>,
}

impl ErrorPropagation {
    pub fn new(token: String, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl Node for ErrorPropagation {
    fn string(&self) -> String {
        format!("{}?", self.expression.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ErrorPropagation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ErrorPropagation {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
        })
    }
}

/// Type assertion expression (expr.(Type))
#[derive(Debug, Clone)]
pub struct TypeAssertion {
    pub token: String,
    pub expression: Box<dyn Expression>,
    pub assert_type: Box<dyn Expression>,
}

impl TypeAssertion {
    pub fn new(
        token: String,
        expression: Box<dyn Expression>,
        assert_type: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            expression,
            assert_type,
        }
    }
}

impl Node for TypeAssertion {
    fn string(&self) -> String {
        format!("{}.({}))", self.expression.string(), self.assert_type.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TypeAssertion {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeAssertion {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
            assert_type: self.assert_type.clone_box(),
        })
    }
}

/// Type assertion with question mark (expr.(Type)?)
#[derive(Debug, Clone)]
pub struct TypeAssertionQuestion {
    pub token: String,
    pub expression: Box<dyn Expression>,
    pub assert_type: Box<dyn Expression>,
}

impl TypeAssertionQuestion {
    pub fn new(
        token: String,
        expression: Box<dyn Expression>,
        assert_type: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            expression,
            assert_type,
        }
    }
}

impl Node for TypeAssertionQuestion {
    fn string(&self) -> String {
        format!("{}.({})?", self.expression.string(), self.assert_type.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TypeAssertionQuestion {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeAssertionQuestion {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
            assert_type: self.assert_type.clone_box(),
        })
    }
}

/// Channel send operation AST node (ch <- value)
/// 
/// Represents sending a value to a channel in CURSED language.
/// Syntax: `channel_expression <- value_expression`
#[derive(Debug)]
pub struct ChannelSend {
    pub token: String,
    pub channel: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl ChannelSend {
    pub fn new(token: String, channel: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self { token, channel, value }
    }
}

impl Node for ChannelSend {
    fn string(&self) -> String {
        format!("{} <- {}", self.channel.string(), self.value.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for ChannelSend {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
            value: self.value.clone_box(),
        }
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

/// Channel receive operation AST node (<-ch)
/// 
/// Represents receiving a value from a channel in CURSED language.
/// Syntax: `<-channel_expression` or `value := <-channel_expression`
#[derive(Debug)]
pub struct ChannelReceive {
    pub token: String,
    pub channel: Box<dyn Expression>,
}

impl ChannelReceive {
    pub fn new(token: String, channel: Box<dyn Expression>) -> Self {
        Self { token, channel }
    }
}

impl Node for ChannelReceive {
    fn string(&self) -> String {
        format!("<-{}", self.channel.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for ChannelReceive {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            channel: self.channel.clone_box(),
        }
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

/// Channel creation AST node
/// 
/// Represents creating a new channel in CURSED language.
/// Can be buffered or unbuffered channels.
#[derive(Debug)]
pub struct ChannelCreation {
    pub token: String,
    pub element_type: Box<dyn Expression>,
    pub buffer_size: Option<Box<dyn Expression>>,
}

impl ChannelCreation {
    pub fn new(token: String, element_type: Box<dyn Expression>) -> Self {
        Self { 
            token,
            element_type,
            buffer_size: None,
        }
    }
    
    pub fn with_buffer(token: String, element_type: Box<dyn Expression>, buffer_size: Box<dyn Expression>) -> Self {
        Self { 
            token,
            element_type,
            buffer_size: Some(buffer_size),
        }
    }
}

impl Node for ChannelCreation {
    fn string(&self) -> String {
        match &self.buffer_size {
            Some(size) => format!("make(dm {}, {})", self.element_type.string(), size.string()),
            None => format!("make(dm {})", self.element_type.string()),
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for ChannelCreation {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            element_type: self.element_type.clone_box(),
            buffer_size: self.buffer_size.as_ref().map(|s| s.clone_box()),
        }
    }
}

impl Expression for ChannelCreation {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Goroutine spawn AST node (stan function_call())
/// 
/// Represents spawning a goroutine in CURSED language.
/// Syntax: `stan function_call()` or `stan { block }`
#[derive(Debug)]
pub struct GoroutineSpawn {
    pub token: String,
    pub function_call: Box<dyn Expression>,
}

impl GoroutineSpawn {
    pub fn new(token: String, function_call: Box<dyn Expression>) -> Self {
        Self { token, function_call }
    }
}

impl Node for GoroutineSpawn {
    fn string(&self) -> String {
        format!("stan {}", self.function_call.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for GoroutineSpawn {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            function_call: self.function_call.clone_box(),
        }
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
