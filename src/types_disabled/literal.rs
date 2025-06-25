// Literal types for CURSED
use std::fmt;

/// Literal value in CURSED source code
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Boolean literal (true/false)
    Boolean(bool),
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Character literal
    Char(char),
    /// Null/nil literal
    Null,
}

impl Literal {
    /// Create a boolean literal
    pub fn boolean(value: bool) -> Self {
        Literal::Boolean(value)
    }
    
    /// Create an integer literal
    pub fn integer(value: i64) -> Self {
        Literal::Integer(value)
    }
    
    /// Create a float literal
    pub fn float(value: f64) -> Self {
        Literal::Float(value)
    }
    
    /// Create a string literal
    pub fn string<S: Into<String>>(value: S) -> Self {
        Literal::String(value.into())
    }
    
    /// Create a character literal
    pub fn char(value: char) -> Self {
        Literal::Char(value)
    }
    
    /// Create a null literal
    pub fn null() -> Self {
        Literal::Null
    }
    
    /// Get the type name of this literal
    pub fn type_name(&self) -> &'static str {
        match self {
            Literal::Boolean(_) => "bool",
            Literal::Integer(_) => "int",
            Literal::Float(_) => "float",
            Literal::String(_) => "string",
            Literal::Char(_) => "char",
            Literal::Null => "null",
        }
    }
    
    /// Check if literal is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Boolean(b) => *b,
            Literal::Integer(i) => *i != 0,
            Literal::Float(f) => *f != 0.0,
            Literal::String(s) => !s.is_empty(),
            Literal::Char(_) => true,
            Literal::Null => false,
        }
    }
    
    /// Convert to runtime value
    pub fn to_runtime_value(&self) -> crate::runtime::Value {
        match self {
            Literal::Boolean(b) => crate::runtime::Value::Bool(*b),
            Literal::Integer(i) => crate::runtime::Value::Integer(*i),
            Literal::Float(f) => crate::runtime::Value::Number(*f),
            Literal::String(s) => crate::runtime::Value::String(s.clone()),
            Literal::Char(c) => crate::runtime::Value::String(c.to_string()),
            Literal::Null => crate::runtime::Value::Null,
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Integer(i) => write!(f, "{}", i),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Char(c) => write!(f, "'{}'", c),
            Literal::Null => write!(f, "null"),
        }
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Literal::Boolean(value)
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Literal::Integer(value)
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Float(value)
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::String(value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal::String(value.to_string())
    }
}

impl From<char> for Literal {
    fn from(value: char) -> Self {
        Literal::Char(value)
    }
}
