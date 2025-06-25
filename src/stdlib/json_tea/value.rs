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
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// Check if this is a null value
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }
    
    /// Check if this is a boolean value
    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    }
    
    /// Check if this is a number value
    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    }
    
    /// Check if this is a string value
    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::String(_))
    }
    
    /// Check if this is an array value
    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }
    
    /// Check if this is an object value
    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }
    
    /// Get the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            JsonValue::Null => "null",
            JsonValue::Bool(_) => "boolean",
            JsonValue::Number(_) => "number",
            JsonValue::String(_) => "string",
            JsonValue::Array(_) => "array",
            JsonValue::Object(_) => "object",
        }
    }
    
    /// Try to get this value as a boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    
    /// Try to get this value as a number
    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }
    
    /// Try to get this value as a string
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    /// Try to get this value as an array
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }
    
    /// Try to get this value as an object
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }
    
    /// Try to get a mutable reference to this value as an array
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }
    
    /// Try to get a mutable reference to this value as an object
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }
    
    /// Get a value from an object by key
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(obj) => obj.get(key),
            _ => None,
        }
    }
    
    /// Get a value from an array by index
    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => None,
        }
    }
    
    /// Check if this value is considered "empty" for omitempty purposes
    pub fn is_empty(&self) -> bool {
        match self {
            JsonValue::Null => true,
            JsonValue::Bool(false) => true,
            JsonValue::Number(n) => *n == 0.0,
            JsonValue::String(s) => s.is_empty(),
            JsonValue::Array(arr) => arr.is_empty(),
            JsonValue::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            JsonValue::String(s) => write!(f, "\"{}\"", escape_json_string(s)),
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
            Value::Null => JsonValue::Null,
            Value::Boolean(b) => JsonValue::Bool(b),
            Value::Number(n) => JsonValue::Number(n),
            Value::String(s) => JsonValue::String(s),
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
            Value::Function(_) => JsonValue::String("<function>".to_string()),
            Value::Channel(_) => JsonValue::String("<channel>".to_string()),
            Value::Interface { .. } => JsonValue::String("<interface>".to_string()),
            Value::Reference(_) => JsonValue::String("<reference>".to_string()),
        }
    }
}

/// Convert from JsonValue to CURSED Value
impl From<JsonValue> for Value {
    fn from(json_value: JsonValue) -> Self {
        match json_value {
            JsonValue::Null => Value::Null,
            JsonValue::Bool(b) => Value::Boolean(b),
            JsonValue::Number(n) => Value::Number(n),
            JsonValue::String(s) => Value::String(s),
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
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\u{08}' => result.push_str("\\b"),
            '\u{0C}' => result.push_str("\\f"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    
    result
}

/// Unescape JSON string escape sequences
pub fn unescape_json_string(s: &str) -> crate::error::Result<()> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('/') => result.push('/'),
                Some('b') => result.push('\u{08}'),
                Some('f') => result.push('\u{0C}'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('u') => {
                    // Unicode escape sequence
                    let mut hex = String::new();
                    for _ in 0..4 {
                        match chars.next() {
                            Some(c) if c.is_ascii_hexdigit() => hex.push(c),
                            _ => return Err(CursedError::json_invalid_escape(format!("\\u{}", hex))),
                        }
                    }
                    
                    let code_point = u32::from_str_radix(&hex, 16)
                        .map_err(|_| CursedError::json_invalid_escape(format!("\\u{}", hex)))?;
                    
                    let unicode_char = char::from_u32(code_point)
                        .ok_or_else(|| CursedError::json_invalid_escape(format!("\\u{}", hex)))?;
                    
                    result.push(unicode_char);
                }
                Some(c) => return Err(CursedError::json_invalid_escape(format!("\\{}", c))),
                None => return Err(CursedError::json_invalid_escape("\\".to_string())),
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

