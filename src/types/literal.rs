// Literal types for CURSED
use std::fmt;

/// Literal value in CURSED source code
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Boolean literal (true/false)
    /// Integer literal
    /// Float literal
    /// String literal
    /// Character literal
    /// Null/nil literal
impl Literal {
    /// Create a boolean literal
    pub fn boolean(value: bool) -> Self {
        Literal::Boolean(value)
    /// Create an integer literal
    pub fn integer(value: i64) -> Self {
        Literal::Integer(value)
    /// Create a float literal
    pub fn float(value: f64) -> Self {
        Literal::Float(value)
    /// Create a string literal
    pub fn string<S: Into<String>>(value: S) -> Self {
        Literal::String(value.into())
    /// Create a character literal
    pub fn char(value: char) -> Self {
        Literal::Char(value)
    /// Create a null literal
    pub fn null() -> Self {
        Literal::Null
    /// Get the type name of this literal
    pub fn type_name(&self) -> &'static str {
        match self {
        }
    }
    
    /// Check if literal is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
        }
    }
    
    /// Convert to runtime value
    pub fn to_runtime_value(&self) -> crate::runtime::Value {
        match self {
        }
    }
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
