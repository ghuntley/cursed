use crate::error::CursedError;
pub type JsonError = crate::error::CursedError;

/// JSON Value Types
/// 
/// JSON value representation and conversion utilities for CURSED

use crate::runtime::value::Value;
use std::collections::HashMap;
use std::fmt;

/// JSON value type - represents any valid JSON value
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
impl JsonValue {
    /// Check if this is a null value
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    /// Check if this is a boolean value
    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    /// Check if this is a number value
    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    /// Check if this is a string value
    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::String(_))
    /// Check if this is an array value
    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    /// Check if this is an object value
    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    /// Get the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
        }
    }
    
    /// Try to get this value as a boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
        }
    }
    
    /// Try to get this value as a number
    pub fn as_number(&self) -> Option<f64> {
        match self {
        }
    }
    
    /// Try to get this value as a string
    pub fn as_str(&self) -> Option<&str> {
        match self {
        }
    }
    
    /// Try to get this value as an array
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
        }
    }
    
    /// Try to get this value as an object
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
        }
    }
    
    /// Try to get a mutable reference to this value as an array
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<JsonValue>> {
        match self {
        }
    }
    
    /// Try to get a mutable reference to this value as an object
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, JsonValue>> {
        match self {
        }
    }
    
    /// Get a value from an object by key
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
        }
    }
    
    /// Get a value from an array by index
    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
        }
    }
    
    /// Check if this value is considered "empty" for omitempty purposes
    pub fn is_empty(&self) -> bool {
        match self {
        }
    }
impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in obj {
                    if !first {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", escape_json_string(key), value)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

/// Convert from CURSED Value to JsonValue
impl From<Value> for JsonValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Array(arr) => {
                JsonValue::Array(arr.into_iter().map(JsonValue::from).collect())
            }
            Value::Object(obj) => {
                JsonValue::Object(
                    obj.into_iter()
                        .map(|(k, v)| (k, JsonValue::from(v)))
                        .collect()
                )
            }
            // Handle other Value types by converting to appropriate JSON representation
        }
    }
/// Convert from JsonValue to CURSED Value
impl From<JsonValue> for Value {
    fn from(json_value: JsonValue) -> Self {
        match json_value {
            JsonValue::Array(arr) => {
                Value::Array(arr.into_iter().map(Value::from).collect())
            }
            JsonValue::Object(obj) => {
                Value::Object(
                    obj.into_iter()
                        .map(|(k, v)| (k, Value::from(v)))
                        .collect()
                )
            }
        }
    }
}

/// Escape special characters in JSON strings
pub fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    
    for ch in s.chars() {
        match ch {
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
        }
    }
    
    result
/// Unescape JSON string escape sequences
pub fn unescape_json_string(s: &str) -> crate::error::Result<()> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('/') => result.push('/'),
                Some('u') => {
                    // Unicode escape sequence
                    let mut hex = String::new();
                    for _ in 0..4 {
                        match chars.next() {
                        }
                    }
                    
                    let code_point = u32::from_str_radix(&hex, 16)
                        .map_err(|_| CursedError::json_invalid_escape(format!("\\u{}", hex)))?;
                    
                    let unicode_char = char::from_u32(code_point)
                        .ok_or_else(|| CursedError::json_invalid_escape(format!("\\u{}", hex)))?;
                    
                    result.push(unicode_char);
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
