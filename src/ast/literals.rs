/// Literal expressions for the CURSED programming language
/// 
/// This module contains AST nodes for all literal values including
/// integers, floats, strings, booleans, and nil values using Gen Z slang.

use crate::ast::traits::{Node, Expression};
use crate::lexer::Token;
use std::any::Any;

/// Integer literal expression (42, 0xFF, 0b1010, etc.)
#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: String, // The token literal as string
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: String, value: i64) -> Self {
        Self { token, value }
    }
}

impl Node for IntegerLiteral {
    fn string(&self) -> String {
        self.value.to_string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for IntegerLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Floating-point literal expression (3.14, 1.0e10, etc.)
#[derive(Debug, Clone)]
pub struct FloatLiteral {
    pub token: String,
    pub value: f64,
}

impl FloatLiteral {
    pub fn new(token: String, value: f64) -> Self {
        Self { token, value }
    }
}

impl Node for FloatLiteral {
    fn string(&self) -> String {
        self.value.to_string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for FloatLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// String literal expression ("hello world", `multiline string`)
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: String,
    pub value: String,
}

impl StringLiteral {
    pub fn new(token: String, value: String) -> Self {
        Self { token, value }
    }
}

impl Node for StringLiteral {
    fn string(&self) -> String {
        format!("\"{}\"", self.value)
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for StringLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Boolean literal expression (based = true, cap = false)
#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: String,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(token: String, value: bool) -> Self {
        Self { token, value }
    }
    
    pub fn based() -> Self {
        Self {
            token: "based".to_string(),
            value: true,
        }
    }
    
    pub fn cap() -> Self {
        Self {
            token: "cap".to_string(),
            value: false,
        }
    }
}

impl Node for BooleanLiteral {
    fn string(&self) -> String {
        if self.value { "based".to_string() } else { "cap".to_string() }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for BooleanLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Nil literal expression (cap = nil)
#[derive(Debug, Clone)]
pub struct NilLiteral {
    pub token: String,
}

impl NilLiteral {
    pub fn new() -> Self {
        Self {
            token: "cap".to_string(),
        }
    }
}

impl Node for NilLiteral {
    fn string(&self) -> String {
        "cap".to_string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for NilLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Default for NilLiteral {
    fn default() -> Self {
        Self::new()
    }
}

/// Character literal expression ('a', '\n', etc.)
#[derive(Debug, Clone)]
pub struct CharLiteral {
    pub token: String,
    pub value: char,
}

impl CharLiteral {
    pub fn new(token: String, value: char) -> Self {
        Self { token, value }
    }
}

impl Node for CharLiteral {
    fn string(&self) -> String {
        format!("'{}'", self.value)
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for CharLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Array literal expression ([1, 2, 3])
#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub token: String,
    pub elements: Vec<Box<dyn Expression>>,
}

impl ArrayLiteral {
    pub fn new(token: String, elements: Vec<Box<dyn Expression>>) -> Self {
        Self { token, elements }
    }
    
    pub fn empty() -> Self {
        Self {
            token: "[".to_string(),
            elements: Vec::new(),
        }
    }
}

impl Node for ArrayLiteral {
    fn string(&self) -> String {
        let elements: Vec<String> = self.elements.iter()
            .map(|e| e.string())
            .collect();
        format!("[{}]", elements.join(", "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for ArrayLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ArrayLiteral {
            token: self.token.clone(),
            elements: self.elements.iter().map(|e| e.clone_box()).collect(),
        })
    }
}

/// Hash/Map literal expression using "tea" keyword
/// tea[KeyType]ValueType{key1: value1, key2: value2}
#[derive(Debug, Clone)]
pub struct HashLiteral {
    pub token: String,
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
}

impl HashLiteral {
    pub fn new(token: String, pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>) -> Self {
        Self { token, pairs }
    }
    
    pub fn empty() -> Self {
        Self {
            token: "tea".to_string(),
            pairs: Vec::new(),
        }
    }
}

impl Node for HashLiteral {
    fn string(&self) -> String {
        let pairs: Vec<String> = self.pairs.iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect();
        format!("{{{}}}", pairs.join(", "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for HashLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(HashLiteral {
            token: self.token.clone(),
            pairs: self.pairs.iter()
                .map(|(k, v)| (k.clone_box(), v.clone_box()))
                .collect(),
        })
    }
}

/// Map literal expression (alias for HashLiteral)
#[derive(Debug, Clone)]
pub struct MapLiteral {
    pub token: String,
    pub key_type: Box<dyn Expression>,
    pub value_type: Box<dyn Expression>,
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
}

impl MapLiteral {
    pub fn new(
        token: crate::lexer::Token,
        key_type: Box<dyn Expression>,
        value_type: Box<dyn Expression>,
        pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
    ) -> Self {
        Self {
            token: token.literal,
            key_type,
            value_type,
            pairs,
        }
    }
    
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
    
    pub fn get_key_type(&self) -> &dyn Expression {
        &**self.key_type
    }
    
    pub fn get_value_type(&self) -> &dyn Expression {
        &**self.value_type
    }
    
    pub fn pairs_iter(&self) -> impl Iterator<Item = (&Box<dyn Expression>, &Box<dyn Expression>)> {
        self.pairs.iter().map(|(k, v)| (k, v))
    }
}

impl Node for MapLiteral {
    fn string(&self) -> String {
        let pairs: Vec<String> = self.pairs.iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect();
        format!("tea[{}]{}{{{}}}", 
            self.key_type.string(), 
            self.value_type.string(),
            pairs.join(", "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for MapLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(MapLiteral {
            token: self.token.clone(),
            key_type: self.key_type.clone_box(),
            value_type: self.value_type.clone_box(),
            pairs: self.pairs.iter()
                .map(|(k, v)| (k.clone_box(), v.clone_box()))
                .collect(),
        })
    }
}

/// Type alias for commonly used integer expression (normie)
pub type NormieExpression = IntegerLiteral;

/// Type alias for commonly used integer literal
pub type IntLiteral = IntegerLiteral;

/// Helper function to create integer literals
pub fn int_lit(value: i64) -> IntegerLiteral {
    IntegerLiteral::new(value.to_string(), value)
}

/// Helper function to create string literals  
pub fn string_lit(value: &str) -> StringLiteral {
    StringLiteral::new(format!("\"{}\"", value), value.to_string())
}

/// Helper function to create boolean literals
pub fn bool_lit(value: bool) -> BooleanLiteral {
    if value {
        BooleanLiteral::based()
    } else {
        BooleanLiteral::cap()
    }
}
