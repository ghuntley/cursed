/// fr fr Generic value type for CURSED runtime - the universal type periodt
///
/// This module provides a universal value type that can represent any CURSED
/// value at runtime. Think of it as the ultimate type flexibility bestie!

use std::collections::HashMap;
use std::fmt;

/// fr fr Universal value type for CURSED runtime
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Null/nil value
    Null,
    /// Boolean value
    Bool(bool),
    /// Integer value
    Integer(i64),
    /// Floating point number
    Number(f64),
    /// String value
    String(String),
    /// Array/list of values
    Array(Vec<Value>),
    /// Object/map of key-value pairs
    Object(HashMap<String, Value>),
    /// Binary data
    Bytes(Vec<u8>),
}

impl Value {
    /// slay Create a null value
    pub fn null() -> Self {
        Value::Null
    }

    /// slay Create a boolean value
    pub fn bool(value: bool) -> Self {
        Value::Bool(value)
    }

    /// slay Create an integer value
    pub fn integer(value: i64) -> Self {
        Value::Integer(value)
    }

    /// slay Create a number value
    pub fn number(value: f64) -> Self {
        Value::Number(value)
    }

    /// slay Create a string value
    pub fn string<S: Into<String>>(value: S) -> Self {
        Value::String(value.into())
    }

    /// slay Create an array value
    pub fn array(values: Vec<Value>) -> Self {
        Value::Array(values)
    }

    /// slay Create an object value
    pub fn object(map: HashMap<String, Value>) -> Self {
        Value::Object(map)
    }

    /// slay Create a bytes value
    pub fn bytes(data: Vec<u8>) -> Self {
        Value::Bytes(data)
    }

    /// slay Check if value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// slay Check if value is boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    /// slay Check if value is integer
    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    /// slay Check if value is number
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_) | Value::Integer(_))
    }

    /// slay Check if value is string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// slay Check if value is array
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// slay Check if value is object
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// slay Check if value is bytes
    pub fn is_bytes(&self) -> bool {
        matches!(self, Value::Bytes(_))
    }

    /// slay Get value as boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// slay Get value as integer
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            Value::Number(n) => Some(*n as i64),
            _ => None,
        }
    }

    /// slay Get value as number
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// slay Get value as string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// slay Get value as array
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// slay Get value as mutable array
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// slay Get value as object
    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// slay Get value as mutable object
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// slay Get value as bytes
    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        match self {
            Value::Bytes(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// slay Convert value to string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Bytes(bytes) => format!("bytes[{}]", bytes.len()),
        }
    }

    /// slay Get the type name of the value
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::Integer(_) => "int",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Bytes(_) => "bytes",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Array(value)
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Value::Object(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Bytes(value)
    }
}

