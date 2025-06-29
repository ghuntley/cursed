//! Runtime value representation for CURSED language
//!
//! This module provides the core value types and operations used during
//! program execution, including type conversion, value comparison, and
//! runtime type checking.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::value::Value;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

/// Runtime value wrapper with metadata for CURSED execution
#[derive(Debug, Clone)]
pub struct RuntimeValue {
    /// The actual value
    pub value: Value,
    /// Type information for runtime checks
    pub type_info: TypeInfo,
    /// Reference count for memory management
    pub ref_count: Arc<Mutex<usize>>,
    /// Source location for debugging
    pub source_location: Option<SourceLocation>,
    /// Additional metadata
    pub metadata: ValueMetadata,
}

/// Type information for runtime values
#[derive(Debug, Clone, PartialEq)]
pub struct TypeInfo {
    /// Base type name
    pub type_name: String,
    /// Whether this type is mutable
    pub is_mutable: bool,
    /// Generic type parameters if any
    pub type_params: Vec<String>,
    /// Size in bytes for memory allocation
    pub size_hint: Option<usize>,
}

/// Source location information for debugging
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

/// Additional metadata for values
#[derive(Debug, Clone, Default)]
pub struct ValueMetadata {
    /// Whether this value is frozen (immutable)
    pub is_frozen: bool,
    /// Creation timestamp for GC
    pub created_at: Option<std::time::Instant>,
    /// Tags for debugging/profiling
    pub tags: Vec<String>,
}

impl RuntimeValue {
    /// Create a new runtime value
    pub fn new(value: Value) -> Self {
        let type_name = value.type_name().to_string();
        Self {
            value,
            type_info: TypeInfo {
                type_name,
                is_mutable: true,
                type_params: Vec::new(),
                size_hint: None,
            },
            ref_count: Arc::new(Mutex::new(1)),
            source_location: None,
            metadata: ValueMetadata::default(),
        }
    }

    /// Create a runtime value with type information
    pub fn with_type(value: Value, type_info: TypeInfo) -> Self {
        Self {
            value,
            type_info,
            ref_count: Arc::new(Mutex::new(1)),
            source_location: None,
            metadata: ValueMetadata::default(),
        }
    }

    /// Create a runtime value with source location
    pub fn with_location(value: Value, location: SourceLocation) -> Self {
        let type_name = value.type_name().to_string();
        Self {
            value,
            type_info: TypeInfo {
                type_name,
                is_mutable: true,
                type_params: Vec::new(),
                size_hint: None,
            },
            ref_count: Arc::new(Mutex::new(1)),
            source_location: Some(location),
            metadata: ValueMetadata::default(),
        }
    }

    /// Get the underlying value
    pub fn get_value(&self) -> &Value {
        &self.value
    }

    /// Get mutable reference to the value (if mutable)
    pub fn get_value_mut(&mut self) -> CursedResult<&mut Value> {
        if !self.type_info.is_mutable || self.metadata.is_frozen {
            return Err(Error::Runtime(
                "Cannot mutably access immutable or frozen value".to_string()
            ));
        }
        Ok(&mut self.value)
    }

    /// Check if this value is of a specific type
    pub fn is_type(&self, type_name: &str) -> bool {
        self.type_info.type_name == type_name
    }

    /// Get type name
    pub fn type_name(&self) -> &str {
        &self.type_info.type_name
    }

    /// Increment reference count
    pub fn add_ref(&self) {
        if let Ok(mut count) = self.ref_count.lock() {
            *count += 1;
        }
    }

    /// Decrement reference count
    pub fn release(&self) -> usize {
        if let Ok(mut count) = self.ref_count.lock() {
            if *count > 0 {
                *count -= 1;
            }
            *count
        } else {
            0
        }
    }

    /// Get current reference count
    pub fn ref_count(&self) -> usize {
        self.ref_count.lock().map(|c| *c).unwrap_or(0)
    }

    /// Freeze this value (make immutable)
    pub fn freeze(&mut self) {
        self.metadata.is_frozen = true;
        self.type_info.is_mutable = false;
    }

    /// Check if value is frozen
    pub fn is_frozen(&self) -> bool {
        self.metadata.is_frozen
    }

    /// Add a tag for debugging/profiling
    pub fn add_tag(&mut self, tag: String) {
        self.metadata.tags.push(tag);
    }

    /// Get all tags
    pub fn get_tags(&self) -> &[String] {
        &self.metadata.tags
    }

    /// Convert to string representation
    pub fn to_string_debug(&self) -> String {
        format!("RuntimeValue {{ value: {}, type: {}, refs: {}, frozen: {} }}",
            self.value.to_string_repr(),
            self.type_info.type_name,
            self.ref_count(),
            self.metadata.is_frozen
        )
    }

    /// Create null runtime value
    pub fn null() -> Self {
        Self::new(Value::null())
    }

    /// Create boolean runtime value
    pub fn bool(value: bool) -> Self {
        Self::new(Value::bool(value))
    }

    /// Create integer runtime value
    pub fn integer(value: i64) -> Self {
        Self::new(Value::integer(value))
    }

    /// Create number runtime value
    pub fn number(value: f64) -> Self {
        Self::new(Value::number(value))
    }

    /// Create string runtime value
    pub fn string<S: Into<String>>(value: S) -> Self {
        Self::new(Value::string(value))
    }

    /// Create array runtime value
    pub fn array(values: Vec<Value>) -> Self {
        Self::new(Value::array(values))
    }

    /// Create object runtime value
    pub fn object(map: HashMap<String, Value>) -> Self {
        Self::new(Value::object(map))
    }

    /// Check if value is truthy in CURSED semantics
    pub fn is_truthy(&self) -> bool {
        self.value.is_truthy()
    }

    /// Perform type conversion
    pub fn convert_to(&self, target_type: &str) -> CursedResult<RuntimeValue> {
        let converted_value = match target_type {
            "bool" => Value::bool(self.value.to_bool()),
            "integer" => {
                if let Some(i) = self.value.to_integer() {
                    Value::integer(i)
                } else {
                    return Err(Error::Runtime(format!(
                        "Cannot convert {} to integer", self.type_name()
                    )));
                }
            }
            "number" => {
                if let Some(n) = self.value.to_number() {
                    Value::number(n)
                } else {
                    return Err(Error::Runtime(format!(
                        "Cannot convert {} to number", self.type_name()
                    )));
                }
            }
            "string" => Value::string(self.value.to_string_repr()),
            _ => return Err(Error::Runtime(format!(
                "Unknown target type: {}", target_type
            ))),
        };

        let mut result = RuntimeValue::new(converted_value);
        result.type_info.type_name = target_type.to_string();
        
        // Copy source location if available
        result.source_location = self.source_location.clone();
        
        Ok(result)
    }

    /// Compare with another runtime value
    pub fn equals(&self, other: &RuntimeValue) -> bool {
        self.value.equals(&other.value)
    }

    /// Compare for ordering
    pub fn compare(&self, other: &RuntimeValue) -> std::cmp::Ordering {
        self.value.compare(&other.value)
    }
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.to_string_repr())
    }
}

impl Default for RuntimeValue {
    fn default() -> Self {
        Self::null()
    }
}

/// Runtime value manager for CURSED execution environment
pub struct RuntimeValueManager {
    /// Active values by ID
    active_values: Mutex<HashMap<usize, RuntimeValue>>,
    /// Next value ID
    next_id: Mutex<usize>,
    /// Statistics
    stats: Mutex<RuntimeValueStats>,
}

/// Statistics for runtime value operations
#[derive(Debug, Default, Clone)]
pub struct RuntimeValueStats {
    pub values_created: usize,
    pub values_destroyed: usize,
    pub type_conversions: usize,
    pub reference_increments: usize,
    pub reference_decrements: usize,
}

impl RuntimeValueManager {
    /// Create a new runtime value manager
    pub fn new() -> Self {
        Self {
            active_values: Mutex::new(HashMap::new()),
            next_id: Mutex::new(0),
            stats: Mutex::new(RuntimeValueStats::default()),
        }
    }

    /// Create and register a new runtime value
    pub fn create_value(&self, value: Value) -> usize {
        let runtime_value = RuntimeValue::new(value);
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };

        {
            let mut active_values = self.active_values.lock().unwrap();
            active_values.insert(id, runtime_value);
        }

        {
            let mut stats = self.stats.lock().unwrap();
            stats.values_created += 1;
        }

        id
    }

    /// Get a runtime value by ID
    pub fn get_value(&self, id: usize) -> Option<RuntimeValue> {
        let active_values = self.active_values.lock().unwrap();
        active_values.get(&id).cloned()
    }

    /// Remove a runtime value
    pub fn destroy_value(&self, id: usize) -> bool {
        let removed = {
            let mut active_values = self.active_values.lock().unwrap();
            active_values.remove(&id).is_some()
        };

        if removed {
            let mut stats = self.stats.lock().unwrap();
            stats.values_destroyed += 1;
        }

        removed
    }

    /// Get statistics
    pub fn get_stats(&self) -> RuntimeValueStats {
        self.stats.lock().unwrap().clone()
    }

    /// Get count of active values
    pub fn active_count(&self) -> usize {
        self.active_values.lock().unwrap().len()
    }

    /// Cleanup values with zero references
    pub fn cleanup_unused(&self) -> usize {
        let mut cleaned_up = 0;
        let mut to_remove = Vec::new();

        {
            let active_values = self.active_values.lock().unwrap();
            for (id, value) in active_values.iter() {
                if value.ref_count() == 0 {
                    to_remove.push(*id);
                }
            }
        }

        for id in to_remove {
            if self.destroy_value(id) {
                cleaned_up += 1;
            }
        }

        cleaned_up
    }
}

impl Default for RuntimeValueManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global runtime value manager instance
static GLOBAL_RUNTIME_VALUE_MANAGER: std::sync::LazyLock<RuntimeValueManager> = 
    std::sync::LazyLock::new(|| RuntimeValueManager::new());

/// Get the global runtime value manager
pub fn get_global_runtime_value_manager() -> &'static RuntimeValueManager {
    &GLOBAL_RUNTIME_VALUE_MANAGER
}

/// Utility functions for common runtime value operations
pub mod utils {
    use super::*;

    /// Create a runtime value from a literal
    pub fn from_literal(literal: &str) -> CursedResult<RuntimeValue> {
        if literal == "null" {
            Ok(RuntimeValue::null())
        } else if literal == "true" {
            Ok(RuntimeValue::bool(true))
        } else if literal == "false" {
            Ok(RuntimeValue::bool(false))
        } else if let Ok(i) = literal.parse::<i64>() {
            Ok(RuntimeValue::integer(i))
        } else if let Ok(f) = literal.parse::<f64>() {
            Ok(RuntimeValue::number(f))
        } else {
            // Treat as string literal (remove quotes if present)
            let str_value = if literal.starts_with('"') && literal.ends_with('"') {
                &literal[1..literal.len()-1]
            } else {
                literal
            };
            Ok(RuntimeValue::string(str_value))
        }
    }

    /// Perform binary operation on runtime values
    pub fn binary_op(left: &RuntimeValue, op: &str, right: &RuntimeValue) -> CursedResult<RuntimeValue> {
        match op {
            "+" => {
                match (&left.value, &right.value) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(RuntimeValue::integer(a + b)),
                    (Value::Number(a), Value::Number(b)) => Ok(RuntimeValue::number(a + b)),
                    (Value::Integer(a), Value::Number(b)) => Ok(RuntimeValue::number(*a as f64 + b)),
                    (Value::Number(a), Value::Integer(b)) => Ok(RuntimeValue::number(a + *b as f64)),
                    (Value::String(a), Value::String(b)) => Ok(RuntimeValue::string(format!("{}{}", a, b))),
                    _ => Err(Error::Runtime(format!(
                        "Cannot add {} and {}", left.type_name(), right.type_name()
                    ))),
                }
            }
            "-" => {
                match (&left.value, &right.value) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(RuntimeValue::integer(a - b)),
                    (Value::Number(a), Value::Number(b)) => Ok(RuntimeValue::number(a - b)),
                    (Value::Integer(a), Value::Number(b)) => Ok(RuntimeValue::number(*a as f64 - b)),
                    (Value::Number(a), Value::Integer(b)) => Ok(RuntimeValue::number(a - *b as f64)),
                    _ => Err(Error::Runtime(format!(
                        "Cannot subtract {} from {}", right.type_name(), left.type_name()
                    ))),
                }
            }
            "*" => {
                match (&left.value, &right.value) {
                    (Value::Integer(a), Value::Integer(b)) => Ok(RuntimeValue::integer(a * b)),
                    (Value::Number(a), Value::Number(b)) => Ok(RuntimeValue::number(a * b)),
                    (Value::Integer(a), Value::Number(b)) => Ok(RuntimeValue::number(*a as f64 * b)),
                    (Value::Number(a), Value::Integer(b)) => Ok(RuntimeValue::number(a * *b as f64)),
                    _ => Err(Error::Runtime(format!(
                        "Cannot multiply {} and {}", left.type_name(), right.type_name()
                    ))),
                }
            }
            "/" => {
                match (&left.value, &right.value) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if *b == 0 {
                            Err(Error::Runtime("Division by zero".to_string()))
                        } else {
                            Ok(RuntimeValue::integer(a / b))
                        }
                    }
                    (Value::Number(a), Value::Number(b)) => {
                        if *b == 0.0 {
                            Err(Error::Runtime("Division by zero".to_string()))
                        } else {
                            Ok(RuntimeValue::number(a / b))
                        }
                    }
                    (Value::Integer(a), Value::Number(b)) => {
                        if *b == 0.0 {
                            Err(Error::Runtime("Division by zero".to_string()))
                        } else {
                            Ok(RuntimeValue::number(*a as f64 / b))
                        }
                    }
                    (Value::Number(a), Value::Integer(b)) => {
                        if *b == 0 {
                            Err(Error::Runtime("Division by zero".to_string()))
                        } else {
                            Ok(RuntimeValue::number(a / *b as f64))
                        }
                    }
                    _ => Err(Error::Runtime(format!(
                        "Cannot divide {} by {}", left.type_name(), right.type_name()
                    ))),
                }
            }
            "==" => Ok(RuntimeValue::bool(left.equals(right))),
            "!=" => Ok(RuntimeValue::bool(!left.equals(right))),
            "<" => Ok(RuntimeValue::bool(left.compare(right) == std::cmp::Ordering::Less)),
            ">" => Ok(RuntimeValue::bool(left.compare(right) == std::cmp::Ordering::Greater)),
            "<=" => Ok(RuntimeValue::bool(matches!(left.compare(right), std::cmp::Ordering::Less | std::cmp::Ordering::Equal))),
            ">=" => Ok(RuntimeValue::bool(matches!(left.compare(right), std::cmp::Ordering::Greater | std::cmp::Ordering::Equal))),
            _ => Err(Error::Runtime(format!("Unknown binary operator: {}", op))),
        }
    }

    /// Perform unary operation on runtime value
    pub fn unary_op(op: &str, operand: &RuntimeValue) -> CursedResult<RuntimeValue> {
        match op {
            "-" => {
                match &operand.value {
                    Value::Integer(i) => Ok(RuntimeValue::integer(-i)),
                    Value::Number(n) => Ok(RuntimeValue::number(-n)),
                    _ => Err(Error::Runtime(format!(
                        "Cannot negate value of type {}", operand.type_name()
                    ))),
                }
            }
            "!" => Ok(RuntimeValue::bool(!operand.is_truthy())),
            _ => Err(Error::Runtime(format!("Unknown unary operator: {}", op))),
        }
    }
}

/// Public function that was likely being used by external code
pub fn get_minimal_result() -> CursedResult<String> {
    Ok("CURSED runtime value system initialized".to_string())
}
