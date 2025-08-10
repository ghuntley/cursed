// Runtime Value System for CURSED
//
// This module defines the value types used at runtime including:
// - Basic data types (null, bool, numbers, strings)
// - Composite types (arrays, objects)
// - Function values and closures
// - Type information and conversion

use std::collections::HashMap;
use std::fmt;

/// Runtime value representation for CURSED
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
    Binary(Vec<u8>),
    /// Function value (placeholder for now)
    Function {
        name: String,
        arity: usize,
    },
    /// Interface value (fat pointer with vtable)
    Interface {
        vtable_ptr: usize,
        data_ptr: usize,
        interface_name: String,
        concrete_type: String,
    },
}

impl Value {
    /// Create a null value
    pub fn null() -> Self {
        Value::Null
    }

    /// Create a boolean value
    pub fn bool(value: bool) -> Self {
        Value::Bool(value)
    }

    /// Create an integer value
    pub fn integer(value: i64) -> Self {
        Value::Integer(value)
    }

    /// Create a number value
    pub fn number(value: f64) -> Self {
        Value::Number(value)
    }

    /// Create a string value
    pub fn string<S: Into<String>>(value: S) -> Self {
        Value::String(value.into())
    }

    /// Create an array value
    pub fn array(values: Vec<Value>) -> Self {
        Value::Array(values)
    }

    /// Create an object value
    pub fn object(map: HashMap<String, Value>) -> Self {
        Value::Object(map)
    }

    /// Create binary data value
    pub fn binary(data: Vec<u8>) -> Self {
        Value::Binary(data)
    }

    /// Create a function value
    pub fn function(name: String, arity: usize) -> Self {
        Value::Function { name, arity }
    }
    
    /// Create an interface value
    pub fn interface(vtable_ptr: usize, data_ptr: usize, interface_name: String, concrete_type: String) -> Self {
        Value::Interface { vtable_ptr, data_ptr, interface_name, concrete_type }
    }

    /// Check if value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
            Value::Binary(b) => !b.is_empty(),
            Value::Function { .. } => true,
            Value::Interface { .. } => true,
        }
    }

    /// Get the type name of this value
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::Integer(_) => "integer",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Binary(_) => "binary",
            Value::Function { .. } => "function",
            Value::Interface { interface_name, .. } => "interface",
        }
    }

    /// Convert value to string representation
    pub fn to_string_repr(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_repr()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_string_repr()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Binary(data) => format!("<binary: {} bytes>", data.len()),
            Value::Function { name, arity } => format!("<function: {}({} args)>", name, arity),
            Value::Interface { interface_name, concrete_type, .. } => {
                format!("<interface: {} implemented by {}>", interface_name, concrete_type)
            },
        }
    }

    /// Try to convert value to boolean
    pub fn to_bool(&self) -> bool {
        self.is_truthy()
    }

    /// Try to convert value to integer
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            Value::Number(n) => Some(*n as i64),
            Value::Bool(true) => Some(1),
            Value::Bool(false) => Some(0),
            _ => None,
        }
    }

    /// Try to convert value to number
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Integer(i) => Some(*i as f64),
            Value::Bool(true) => Some(1.0),
            Value::Bool(false) => Some(0.0),
            _ => None,
        }
    }

    /// Try to convert value to string
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get array length if this is an array
    pub fn array_len(&self) -> Option<usize> {
        match self {
            Value::Array(arr) => Some(arr.len()),
            _ => None,
        }
    }

    /// Get object keys if this is an object
    pub fn object_keys(&self) -> Option<Vec<&String>> {
        match self {
            Value::Object(obj) => Some(obj.keys().collect()),
            _ => None,
        }
    }

    /// Check if two values are equal
    pub fn equals(&self, other: &Value) -> bool {
        self == other
    }

    /// Compare two values for ordering (for sorting)
    pub fn compare(&self, other: &Value) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (Value::String(a), Value::String(b)) => a.cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
            _ => Ordering::Equal, // Default for incomparable types
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

// Conversion traits for easier value creation
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

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer(value as i64)
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

/// Runtime errors for value operations
#[derive(Debug, Clone)]
pub enum ValueError {
    TypeMismatch { expected: String, found: String },
    IndexOutOfBounds { index: usize, length: usize },
    KeyNotFound { key: String },
    ConversionError { from: String, to: String },
    InvalidOperation { operation: String, value_type: String },
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            ValueError::IndexOutOfBounds { index, length } => {
                write!(f, "Index {} out of bounds for array of length {}", index, length)
            }
            ValueError::KeyNotFound { key } => {
                write!(f, "Key '{}' not found in object", key)
            }
            ValueError::ConversionError { from, to } => {
                write!(f, "Cannot convert from {} to {}", from, to)
            }
            ValueError::InvalidOperation { operation, value_type } => {
                write!(f, "Invalid operation '{}' for value of type {}", operation, value_type)
            }
        }
    }
}

impl std::error::Error for ValueError {}

/// Alias for the main Value type for backward compatibility
pub type CursedValue = Value;

/// Value manager for runtime value operations
pub struct ValueManager {
    /// Global value cache for optimization
    global_cache: std::sync::RwLock<HashMap<String, Value>>,
    /// Statistics
    stats: std::sync::Mutex<ValueManagerStats>,
}

/// Value manager statistics
#[derive(Debug, Default, Clone)]
pub struct ValueManagerStats {
    pub values_created: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub conversions_performed: usize,
}

impl ValueManager {
    /// Create a new value manager
    pub fn new() -> Self {
        Self {
            global_cache: std::sync::RwLock::new(HashMap::new()),
            stats: std::sync::Mutex::new(ValueManagerStats::default()),
        }
    }

    /// Create a new value and track statistics
    pub fn create_value(&self, value: Value) -> Value {
        {
            let mut stats = self.stats.lock().unwrap();
            stats.values_created += 1;
        }
        value
    }

    /// Get a cached value or create if not exists
    pub fn get_or_create(&self, key: &str, factory: impl FnOnce() -> Value) -> Value {
        // Check cache first
        {
            let cache = self.global_cache.read().unwrap();
            if let Some(value) = cache.get(key) {
                let mut stats = self.stats.lock().unwrap();
                stats.cache_hits += 1;
                return value.clone();
            }
        }

        // Create new value
        let value = factory();
        
        // Store in cache
        {
            let mut cache = self.global_cache.write().unwrap();
            cache.insert(key.to_string(), value.clone());
        }

        {
            let mut stats = self.stats.lock().unwrap();
            stats.cache_misses += 1;
            stats.values_created += 1;
        }

        value
    }

    /// Convert between value types with statistics tracking
    pub fn convert_value(&self, value: &Value, target_type: &str) -> Result<Value, ValueError> {
        let result = match target_type {
            "bool" => Ok(Value::Bool(value.to_bool())),
            "integer" => value.to_integer()
                .map(Value::Integer)
                .ok_or_else(|| ValueError::ConversionError {
                    from: value.type_name().to_string(),
                    to: target_type.to_string(),
                }),
            "number" => value.to_number()
                .map(Value::Number)
                .ok_or_else(|| ValueError::ConversionError {
                    from: value.type_name().to_string(),
                    to: target_type.to_string(),
                }),
            "string" => Ok(Value::String(value.to_string_repr())),
            _ => Err(ValueError::ConversionError {
                from: value.type_name().to_string(),
                to: target_type.to_string(),
            }),
        };

        if result.is_ok() {
            let mut stats = self.stats.lock().unwrap();
            stats.conversions_performed += 1;
        }

        result
    }

    /// Get statistics
    pub fn get_stats(&self) -> ValueManagerStats {
        self.stats.lock().unwrap().clone()
    }

    /// Clear the value cache
    pub fn clear_cache(&self) {
        let mut cache = self.global_cache.write().unwrap();
        cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        let cache = self.global_cache.read().unwrap();
        cache.len()
    }
}

impl Default for ValueManager {
    fn default() -> Self {
        Self::new()
    }
}
