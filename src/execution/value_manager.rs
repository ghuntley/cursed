/// Value Management for CURSED Execution
/// 
/// This module provides types and utilities for managing values during CURSED program execution,
/// including type conversions, value formatting, and memory management.

use std::collections::HashMap;
use std::fmt;

/// Represents a value in the CURSED runtime
#[derive(Debug, Clone, PartialEq)]
pub enum CursedValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
    Array(Vec<CursedValue>),
    Object(HashMap<String, CursedValue>),
}

/// Type information for CURSED values
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    Integer,
    Float,
    String,
    Boolean,
    Nil,
    Array(Box<ValueType>),
    Object,
}

/// Manages value creation, conversion, and formatting
pub struct ValueManager {
    stats: ValueManagerStats,
    type_cache: HashMap<String, ValueType>,
}

/// Statistics for value manager performance
#[derive(Debug, Default, Clone)]
pub struct ValueManagerStats {
    pub values_created: u64,
    pub conversions_performed: u64,
    pub format_operations: u64,
    pub type_checks: u64,
}

impl CursedValue {
    /// Get the type of this value
    pub fn get_type(&self) -> ValueType {
        match self {
            CursedValue::Integer(_) => ValueType::Integer,
            CursedValue::Float(_) => ValueType::Float,
            CursedValue::String(_) => ValueType::String,
            CursedValue::Boolean(_) => ValueType::Boolean,
            CursedValue::Nil => ValueType::Nil,
            CursedValue::Array(values) => {
                if let Some(first) = values.first() {
                    ValueType::Array(Box::new(first.get_type()))
                } else {
                    ValueType::Array(Box::new(ValueType::Nil))
                }
            },
            CursedValue::Object(_) => ValueType::Object,
        }
    }

    /// Check if this value is truthy (for conditional expressions)
    pub fn is_truthy(&self) -> bool {
        match self {
            CursedValue::Boolean(b) => *b,
            CursedValue::Integer(i) => *i != 0,
            CursedValue::Float(f) => *f != 0.0,
            CursedValue::String(s) => !s.is_empty(),
            CursedValue::Nil => false,
            CursedValue::Array(arr) => !arr.is_empty(),
            CursedValue::Object(obj) => !obj.is_empty(),
        }
    }

    /// Convert to integer if possible
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            CursedValue::Integer(i) => Some(*i),
            CursedValue::Float(f) => Some(*f as i64),
            CursedValue::Boolean(b) => Some(if *b { 1 } else { 0 }),
            CursedValue::String(s) => s.parse().ok(),
            CursedValue::Nil => Some(0),
            _ => None,
        }
    }

    /// Convert to float if possible
    pub fn to_float(&self) -> Option<f64> {
        match self {
            CursedValue::Float(f) => Some(*f),
            CursedValue::Integer(i) => Some(*i as f64),
            CursedValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            CursedValue::String(s) => s.parse().ok(),
            CursedValue::Nil => Some(0.0),
            _ => None,
        }
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        match self {
            CursedValue::String(s) => s.clone(),
            CursedValue::Integer(i) => i.to_string(),
            CursedValue::Float(f) => f.to_string(),
            CursedValue::Boolean(b) => b.to_string(),
            CursedValue::Nil => "nil".to_string(),
            CursedValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            },
            CursedValue::Object(obj) => {
                let pairs: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            },
        }
    }

    /// Convert to boolean
    pub fn to_boolean(&self) -> bool {
        self.is_truthy()
    }

    /// Perform arithmetic addition
    pub fn add(&self, other: &CursedValue) -> Option<CursedValue> {
        match (self, other) {
            (CursedValue::Integer(a), CursedValue::Integer(b)) => Some(CursedValue::Integer(a + b)),
            (CursedValue::Float(a), CursedValue::Float(b)) => Some(CursedValue::Float(a + b)),
            (CursedValue::Integer(a), CursedValue::Float(b)) => Some(CursedValue::Float(*a as f64 + b)),
            (CursedValue::Float(a), CursedValue::Integer(b)) => Some(CursedValue::Float(a + *b as f64)),
            (CursedValue::String(a), CursedValue::String(b)) => Some(CursedValue::String(format!("{}{}", a, b))),
            _ => None,
        }
    }

    /// Perform arithmetic subtraction
    pub fn subtract(&self, other: &CursedValue) -> Option<CursedValue> {
        match (self, other) {
            (CursedValue::Integer(a), CursedValue::Integer(b)) => Some(CursedValue::Integer(a - b)),
            (CursedValue::Float(a), CursedValue::Float(b)) => Some(CursedValue::Float(a - b)),
            (CursedValue::Integer(a), CursedValue::Float(b)) => Some(CursedValue::Float(*a as f64 - b)),
            (CursedValue::Float(a), CursedValue::Integer(b)) => Some(CursedValue::Float(a - *b as f64)),
            _ => None,
        }
    }

    /// Perform arithmetic multiplication
    pub fn multiply(&self, other: &CursedValue) -> Option<CursedValue> {
        match (self, other) {
            (CursedValue::Integer(a), CursedValue::Integer(b)) => Some(CursedValue::Integer(a * b)),
            (CursedValue::Float(a), CursedValue::Float(b)) => Some(CursedValue::Float(a * b)),
            (CursedValue::Integer(a), CursedValue::Float(b)) => Some(CursedValue::Float(*a as f64 * b)),
            (CursedValue::Float(a), CursedValue::Integer(b)) => Some(CursedValue::Float(a * *b as f64)),
            _ => None,
        }
    }

    /// Perform arithmetic division
    pub fn divide(&self, other: &CursedValue) -> Option<CursedValue> {
        match (self, other) {
            (CursedValue::Integer(a), CursedValue::Integer(b)) => {
                if *b != 0 {
                    Some(CursedValue::Integer(a / b))
                } else {
                    None // Division by zero
                }
            },
            (CursedValue::Float(a), CursedValue::Float(b)) => {
                if *b != 0.0 {
                    Some(CursedValue::Float(a / b))
                } else {
                    None // Division by zero
                }
            },
            (CursedValue::Integer(a), CursedValue::Float(b)) => {
                if *b != 0.0 {
                    Some(CursedValue::Float(*a as f64 / b))
                } else {
                    None
                }
            },
            (CursedValue::Float(a), CursedValue::Integer(b)) => {
                if *b != 0 {
                    Some(CursedValue::Float(a / *b as f64))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    /// Compare values for equality
    pub fn equals(&self, other: &CursedValue) -> bool {
        match (self, other) {
            (CursedValue::Integer(a), CursedValue::Integer(b)) => a == b,
            (CursedValue::Float(a), CursedValue::Float(b)) => (a - b).abs() < f64::EPSILON,
            (CursedValue::String(a), CursedValue::String(b)) => a == b,
            (CursedValue::Boolean(a), CursedValue::Boolean(b)) => a == b,
            (CursedValue::Nil, CursedValue::Nil) => true,
            // Type coercion comparisons
            (CursedValue::Integer(a), CursedValue::Float(b)) => (*a as f64 - b).abs() < f64::EPSILON,
            (CursedValue::Float(a), CursedValue::Integer(b)) => (a - *b as f64).abs() < f64::EPSILON,
            _ => false,
        }
    }

    /// Compare values (less than)
    pub fn less_than(&self, other: &CursedValue) -> Option<bool> {
        match (self, other) {
            (CursedValue::Integer(a), CursedValue::Integer(b)) => Some(a < b),
            (CursedValue::Float(a), CursedValue::Float(b)) => Some(a < b),
            (CursedValue::Integer(a), CursedValue::Float(b)) => Some((*a as f64) < *b),
            (CursedValue::Float(a), CursedValue::Integer(b)) => Some(*a < (*b as f64)),
            (CursedValue::String(a), CursedValue::String(b)) => Some(a < b),
            _ => None,
        }
    }
}

impl ValueManager {
    /// Create a new value manager
    pub fn new() -> Self {
        Self {
            stats: ValueManagerStats::default(),
            type_cache: HashMap::new(),
        }
    }

    /// Create a new integer value
    pub fn create_integer(&mut self, value: i64) -> CursedValue {
        self.stats.values_created += 1;
        CursedValue::Integer(value)
    }

    /// Create a new float value
    pub fn create_float(&mut self, value: f64) -> CursedValue {
        self.stats.values_created += 1;
        CursedValue::Float(value)
    }

    /// Create a new string value
    pub fn create_string(&mut self, value: String) -> CursedValue {
        self.stats.values_created += 1;
        CursedValue::String(value)
    }

    /// Create a new boolean value
    pub fn create_boolean(&mut self, value: bool) -> CursedValue {
        self.stats.values_created += 1;
        CursedValue::Boolean(value)
    }

    /// Create a nil value
    pub fn create_nil(&mut self) -> CursedValue {
        self.stats.values_created += 1;
        CursedValue::Nil
    }

    /// Create an array value
    pub fn create_array(&mut self, values: Vec<CursedValue>) -> CursedValue {
        self.stats.values_created += 1;
        CursedValue::Array(values)
    }

    /// Create an object value
    pub fn create_object(&mut self, values: HashMap<String, CursedValue>) -> CursedValue {
        self.stats.values_created += 1;
        CursedValue::Object(values)
    }

    /// Convert a value to a different type
    pub fn convert_value(&mut self, value: &CursedValue, target_type: &ValueType) -> Option<CursedValue> {
        self.stats.conversions_performed += 1;

        match target_type {
            ValueType::Integer => value.to_integer().map(CursedValue::Integer),
            ValueType::Float => value.to_float().map(CursedValue::Float),
            ValueType::String => Some(CursedValue::String(value.to_string())),
            ValueType::Boolean => Some(CursedValue::Boolean(value.to_boolean())),
            ValueType::Nil => Some(CursedValue::Nil),
            _ => None, // Complex types not supported for automatic conversion
        }
    }

    /// Format a value for display (REPL output)
    pub fn format_value(&mut self, value: &CursedValue) -> String {
        self.stats.format_operations += 1;

        match value {
            CursedValue::Integer(i) => i.to_string(),
            CursedValue::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{:.1}", f) // Show at least one decimal place
                } else {
                    f.to_string()
                }
            },
            CursedValue::String(s) => format!("\"{}\"", s),
            CursedValue::Boolean(b) => b.to_string(),
            CursedValue::Nil => "nil".to_string(),
            CursedValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| self.format_value(v)).collect();
                format!("[{}]", elements.join(", "))
            },
            CursedValue::Object(obj) => {
                let pairs: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, self.format_value(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            },
        }
    }

    /// Check if a value matches a type
    pub fn type_matches(&mut self, value: &CursedValue, expected_type: &ValueType) -> bool {
        self.stats.type_checks += 1;
        &value.get_type() == expected_type
    }

    /// Get type name as string
    pub fn type_name(&self, value_type: &ValueType) -> String {
        match value_type {
            ValueType::Integer => "int".to_string(),
            ValueType::Float => "float".to_string(),
            ValueType::String => "string".to_string(),
            ValueType::Boolean => "bool".to_string(),
            ValueType::Nil => "nil".to_string(),
            ValueType::Array(inner) => format!("array<{}>", self.type_name(inner)),
            ValueType::Object => "object".to_string(),
        }
    }

    /// Get statistics
    pub fn get_stats(&self) -> ValueManagerStats {
        self.stats.clone()
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = ValueManagerStats::default();
    }

    /// Parse a literal value from source code
    pub fn parse_literal(&mut self, literal_str: &str) -> Option<CursedValue> {
        self.stats.values_created += 1;

        // Try to parse as integer
        if let Ok(i) = literal_str.parse::<i64>() {
            return Some(CursedValue::Integer(i));
        }

        // Try to parse as float
        if let Ok(f) = literal_str.parse::<f64>() {
            return Some(CursedValue::Float(f));
        }

        // Try to parse as boolean
        match literal_str {
            "true" => return Some(CursedValue::Boolean(true)),
            "false" => return Some(CursedValue::Boolean(false)),
            "nil" => return Some(CursedValue::Nil),
            _ => {},
        }

        // Try to parse as string (remove quotes)
        if literal_str.starts_with('"') && literal_str.ends_with('"') && literal_str.len() >= 2 {
            let content = &literal_str[1..literal_str.len()-1];
            return Some(CursedValue::String(content.to_string()));
        }

        None
    }
}

impl fmt::Display for CursedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ValueType::Integer => "int",
            ValueType::Float => "float",
            ValueType::String => "string",
            ValueType::Boolean => "bool",
            ValueType::Nil => "nil",
            ValueType::Array(_) => "array",
            ValueType::Object => "object",
        };
        write!(f, "{}", name)
    }
}

impl Default for ValueManager {
    fn default() -> Self {
        Self::new()
    }
}

