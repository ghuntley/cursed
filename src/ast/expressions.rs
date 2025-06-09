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
