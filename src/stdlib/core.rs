/// Core - Fundamental types and functions for CURSED 🔥
/// 
/// This module provides the core builtin functions that are automatically included
/// in all CURSED programs. These functions handle type conversions, collection operations,
/// and panic/recovery mechanisms essential for CURSED programming.
/// 
/// # Why Core matters:
/// - Essential foundation for all CURSED programs
/// - Provides type safety with CURSED conventions
/// - Bridges runtime operations with language semantics
/// - Enables panic handling and memory management

use crate::stdlib::value::Value;
use crate::crate::stdlib::errors_simple::CursedError;
use std::collections::HashMap;
use std::any::Any;
use std::panic;

/// Error type for Core operations
pub type CoreError = CursedError;

/// Result type for Core operations
pub type CoreResult<(), Error>;

// ================================
// CURSED TYPE ALIASES
// ================================

/// `litean` - Boolean type (true/false vibes)
pub type Litean = bool;

/// `normie` - 32-bit integer (standard number vibes)
pub type Normie = i32;

/// `thicc` - 64-bit integer (big number vibes)
pub type Thicc = i64;

/// `snack` - 32-bit float (small decimal vibes)
pub type Snack = f32;

/// `meal` - 64-bit float (big decimal vibes)
pub type Meal = f64;

/// `tea` - String type (text vibes)
pub type Tea = String;

// ================================
// TYPE CONVERSION FUNCTIONS
// ================================

/// Convert value to litean (boolean vibes)
/// 
/// # Examples
/// ```cursed
/// facts truth = lit(1) // true
/// facts falsy = lit(0) // false
/// facts string_truth = lit("hello") // true
/// facts empty_falsy = lit("") // false
/// ```
pub fn lit(value: &Value) -> CoreResult<Litean> {
    match value {
        Value::Bool(b) => Ok(*b),
        Value::Integer(i) => Ok(*i != 0),
        Value::Number(f) => Ok(*f != 0.0),
        Value::String(s) => Ok(!s.is_empty()),
        Value::Array(arr) => Ok(!arr.is_empty()),
        Value::Object(obj) => Ok(!obj.is_empty()),
        Value::Null => Ok(false),
        Value::Bytes(bytes) => Ok(!bytes.is_empty()),
    }
}

/// Convert value to normie (32-bit integer vibes)
/// 
/// # Examples
/// ```cursed
/// facts num = normie(42.7) // 42
/// facts parsed = normie("123") // 123
/// facts bool_num = normie(true) // 1
/// ```
pub fn normie(value: &Value) -> CoreResult<Normie> {
    match value {
        Value::Integer(i) => {
            if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 {
                Ok(*i as i32)
            } else {
                Err(CursedError::parse_error(&format!("Value {} out of range for normie", i)))
            }
        },
        Value::Number(f) => Ok(*f as i32),
        Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
        Value::String(s) => {
            s.parse::<i32>()
                .map_err(|e| CursedError::parse_error(&format!("Cannot parse '{}' as normie: {}", s, e)))
        },
        Value::Null => Ok(0),
        _ => Err(CursedError::type_error(&format!("Cannot convert {:?} to normie", value))),
    }
}

/// Convert value to thicc (64-bit integer vibes)
/// 
/// # Examples
/// ```cursed
/// facts big_num = thicc(42.7) // 42
/// facts parsed = thicc("123456789") // 123456789
/// facts bool_num = thicc(true) // 1
/// ```
pub fn thicc(value: &Value) -> CoreResult<Thicc> {
    match value {
        Value::Integer(i) => Ok(*i),
        Value::Number(f) => Ok(*f as i64),
        Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
        Value::String(s) => {
            s.parse::<i64>()
                .map_err(|e| CursedError::parse_error(&format!("Cannot parse '{}' as thicc: {}", s, e)))
        },
        Value::Null => Ok(0),
        _ => Err(CursedError::type_error(&format!("Cannot convert {:?} to thicc", value))),
    }
}

/// Convert value to snack (32-bit float vibes)
/// 
/// # Examples
/// ```cursed
/// facts small_float = snack(42) // 42.0
/// facts parsed = snack("3.14") // 3.14
/// facts bool_float = snack(true) // 1.0
/// ```
pub fn snack(value: &Value) -> CoreResult<Snack> {
    match value {
        Value::Number(f) => Ok(*f as f32),
        Value::Integer(i) => Ok(*i as f32),
        Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
        Value::String(s) => {
            s.parse::<f32>()
                .map_err(|e| CursedError::parse_error(&format!("Cannot parse '{}' as snack: {}", s, e)))
        },
        Value::Null => Ok(0.0),
        _ => Err(CursedError::type_error(&format!("Cannot convert {:?} to snack", value))),
    }
}

/// Convert value to meal (64-bit float vibes)
/// 
/// # Examples
/// ```cursed
/// facts big_float = meal(42) // 42.0
/// facts parsed = meal("3.14159") // 3.14159
/// facts bool_float = meal(true) // 1.0
/// ```
pub fn meal(value: &Value) -> CoreResult<Meal> {
    match value {
        Value::Number(f) => Ok(*f),
        Value::Integer(i) => Ok(*i as f64),
        Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
        Value::String(s) => {
            s.parse::<f64>()
                .map_err(|e| CursedError::parse_error(&format!("Cannot parse '{}' as meal: {}", s, e)))
        },
        Value::Null => Ok(0.0),
        _ => Err(CursedError::type_error(&format!("Cannot convert {:?} to meal", value))),
    }
}

/// Convert value to tea (string vibes)
/// 
/// # Examples
/// ```cursed
/// facts text = tea(42) // "42"
/// facts bool_text = tea(true) // "true"
/// facts float_text = tea(3.14) // "3.14"
/// ```
pub fn tea(value: &Value) -> CoreResult<Tea> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Integer(i) => Ok(i.to_string()),
        Value::Number(f) => Ok(f.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Null => Ok("null".to_string()),
        Value::Array(arr) => Ok(format!("{:?}", arr)),
        Value::Object(obj) => Ok(format!("{:?}", obj)),
        Value::Bytes(bytes) => Ok(format!("{:?}", bytes)),
    }
}

// ================================
// COLLECTION OPERATIONS
// ================================

/// Append elements to slice (add to the vibes)
/// 
/// # Examples
/// ```cursed
/// facts slice = [1, 2, 3]
/// facts new_slice = append(slice, [4, 5]) // [1, 2, 3, 4, 5]
/// ```
pub fn append(slice: &Value, elements: &[Value]) -> CoreResult<Value> {
    match slice {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.extend_from_slice(elements);
            Ok(Value::Array(new_arr))
        },
        _ => Err(CursedError::type_error("append() requires an array as first argument")),
    }
}

/// Get capacity of slice, map, or channel (max vibes)
/// 
/// # Examples
/// ```cursed
/// facts slice_cap = cap(my_slice) // capacity of slice
/// facts map_cap = cap(my_map) // capacity of map
/// ```
pub fn cap(value: &Value) -> CoreResult<Normie> {
    match value {
        Value::Array(arr) => Ok(arr.capacity() as i32),
        Value::Object(obj) => Ok(obj.capacity() as i32),
        Value::String(s) => Ok(s.capacity() as i32),
        Value::Bytes(bytes) => Ok(bytes.capacity() as i32),
        _ => Err(CursedError::type_error("cap() requires a collection type")),
    }
}

/// Get length of tea, array, slice, map, or channel (count the vibes)
/// 
/// # Examples
/// ```cursed
/// facts array_len = len([1, 2, 3]) // 3
/// facts string_len = len("hello") // 5
/// facts map_len = len(my_map) // number of keys
/// ```
pub fn len(value: &Value) -> CoreResult<Normie> {
    match value {
        Value::String(s) => Ok(s.chars().count() as i32),
        Value::Array(arr) => Ok(arr.len() as i32),
        Value::Object(obj) => Ok(obj.len() as i32),
        Value::Bytes(bytes) => Ok(bytes.len() as i32),
        _ => Err(CursedError::type_error("len() requires a collection type")),
    }
}

/// Create slice, map, or channel (make the vibes)
/// 
/// # Examples
/// ```cursed
/// facts empty_slice = make("array") // []
/// facts sized_slice = make("array", 10) // array with capacity 10
/// facts empty_map = make("object") // {}
/// ```
pub fn make(type_name: &str, size: Option<Normie>) -> CoreResult<Value> {
    match type_name {
        "array" | "slice" => {
            let capacity = size.unwrap_or(0) as usize;
            Ok(Value::Array(Vec::with_capacity(capacity)))
        },
        "object" | "map" => {
            let capacity = size.unwrap_or(0) as usize;
            Ok(Value::Object(HashMap::with_capacity(capacity)))
        },
        "bytes" => {
            let capacity = size.unwrap_or(0) as usize;
            Ok(Value::Bytes(Vec::with_capacity(capacity)))
        },
        _ => Err(CursedError::type_error(&format!("Unknown type for make(): {}", type_name))),
    }
}

/// Create pointer to zero value of type (new vibes)
/// 
/// # Examples
/// ```cursed
/// facts new_int = new("normie") // 0
/// facts new_string = new("tea") // ""
/// facts new_bool = new("litean") // false
/// ```
pub fn new(type_name: &str) -> CoreResult<Value> {
    match type_name {
        "litean" | "bool" => Ok(Value::Bool(false)),
        "normie" | "int32" | "thicc" | "int64" | "integer" => Ok(Value::Integer(0)),
        "snack" | "float32" | "meal" | "float64" | "number" => Ok(Value::Number(0.0)),
        "tea" | "string" => Ok(Value::String(String::new())),
        "array" | "slice" => Ok(Value::Array(Vec::new())),
        "object" | "map" => Ok(Value::Object(HashMap::new())),
        "bytes" => Ok(Value::Bytes(Vec::new())),
        _ => Err(CursedError::type_error(&format!("Unknown type for new(): {}", type_name))),
    }
}

// ================================
// PANIC AND RECOVERY
// ================================

/// Cause panic with value (express those feelings)
/// 
/// # Examples
/// ```cursed
/// shook("Something went wrong!") // panics with message
/// shook(42) // panics with number
/// ```
pub fn shook(value: &Value) -> ! {
    let message = tea(value).unwrap_or_else(|_| "Unknown panic value".to_string());
    panic!("CURSED panic: {}", message);
}

/// Recover from panic (chill out vibes)
/// 
/// # Examples
/// ```cursed
/// facts recovered = unbothered() // None if no panic, Some(value) if recovered
/// ```
pub fn unbothered() -> Option<Value> {
    // Note: This is a simplified implementation. In a real implementation,
    // this would need to work with the CURSED runtime's panic handling system.
    // For now, we return None as there's no active panic to recover from.
    None
}

/// Execute function with panic recovery (safe vibes)
/// 
/// # Examples
/// ```cursed
/// facts result = try_unbothered(|| { risky_operation() })
/// ```
pub fn try_unbothered<F, T>(f: F) -> Result<T, Value>
where
    F: FnOnce() -> T + panic::UnwindSafe,
{
    match panic::catch_unwind(f) {
        Ok(result) => Ok(result),
        Err(panic_info) => {
            // Convert panic info to Value
            let panic_message = if let Some(s) = panic_info.downcast_ref::<&str>() {
                Value::String(s.to_string())
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                Value::String(s.clone())
            } else {
                Value::String("Unknown panic".to_string())
            };
            Err(panic_message)
        }
    }
}

// ================================
// UTILITY FUNCTIONS
// ================================

/// Get zero value for type (empty vibes)
/// 
/// # Examples
/// ```cursed
/// facts zero_int = zero_value("normie") // 0
/// facts zero_string = zero_value("tea") // ""
/// ```
pub fn zero_value(type_name: &str) -> CoreResult<Value> {
    new(type_name)
}

/// Check if value is zero/empty (check the emptiness)
/// 
/// # Examples
/// ```cursed
/// facts is_zero = is_zero_value(&Value::Int32(0)) // true
/// facts is_empty = is_zero_value(&Value::String("".to_string())) // true
/// ```
pub fn is_zero_value(value: &Value) -> bool {
    match value {
        Value::Bool(b) => !b,
        Value::Integer(i) => *i == 0,
        Value::Number(f) => *f == 0.0,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        Value::Bytes(bytes) => bytes.is_empty(),
        Value::Null => true,
    }
}

/// Get type name of value (what vibes is this)
/// 
/// # Examples
/// ```cursed
/// facts type_name = type_of(&Value::Int32(42)) // "normie"
/// facts type_name = type_of(&Value::String("hello".to_string())) // "tea"
/// ```
pub fn type_of(value: &Value) -> &'static str {
    match value {
        Value::Bool(_) => "litean",
        Value::Integer(_) => "thicc", // Use thicc as the primary integer type
        Value::Number(_) => "meal",   // Use meal as the primary float type
        Value::String(_) => "tea",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Bytes(_) => "bytes",
        Value::Null => "null",
    }
}

/// Clone a value (duplicate the vibes)
/// 
/// # Examples
/// ```cursed
/// facts original = Value::Int32(42)
/// facts copy = clone_value(&original) // same value, different instance
/// ```
pub fn clone_value(value: &Value) -> Value {
    value.clone()
}

/// Compare two values for equality (same vibes check)
/// 
/// # Examples
/// ```cursed
/// facts same = equal_values(&Value::Int32(42), &Value::Int32(42)) // true
/// facts different = equal_values(&Value::Int32(42), &Value::String("42".to_string())) // false
/// ```
pub fn equal_values(a: &Value, b: &Value) -> bool {
    a == b
}

/// Module initialization function
pub fn init_core() -> CoreResult<()> {
    // Initialize any global state for Core module
    Ok(())
}

/// Get module statistics and information
pub fn get_core_stats() -> HashMap<String, String> {
    let mut stats = HashMap::new();
    stats.insert("version".to_string(), "1.0.0".to_string());
    stats.insert("functions".to_string(), "20+".to_string());
    stats.insert("features".to_string(), "Type conversions, collections, panic handling".to_string());
    stats.insert("types".to_string(), "litean, normie, thicc, snack, meal, tea".to_string());
    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_conversions() {
        // Test lit() conversion
        assert_eq!(lit(&Value::Int32(1)).unwrap(), true);
        assert_eq!(lit(&Value::Int32(0)).unwrap(), false);
        assert_eq!(lit(&Value::String("hello".to_string())).unwrap(), true);
        assert_eq!(lit(&Value::String("".to_string())).unwrap(), false);

        // Test normie() conversion
        assert_eq!(normie(&Value::Int32(42)).unwrap(), 42);
        assert_eq!(normie(&Value::Float64(42.7)).unwrap(), 42);
        assert_eq!(normie(&Value::String("123".to_string())).unwrap(), 123);
        assert_eq!(normie(&Value::Bool(true)).unwrap(), 1);

        // Test thicc() conversion
        assert_eq!(thicc(&Value::Int32(42)).unwrap(), 42i64);
        assert_eq!(thicc(&Value::String("123456789".to_string())).unwrap(), 123456789i64);

        // Test snack() conversion
        assert_eq!(snack(&Value::Int32(42)).unwrap(), 42.0f32);
        assert_eq!(snack(&Value::String("3.14".to_string())).unwrap(), 3.14f32);

        // Test meal() conversion
        assert_eq!(meal(&Value::Int32(42)).unwrap(), 42.0f64);
        assert_eq!(meal(&Value::String("3.14159".to_string())).unwrap(), 3.14159f64);

        // Test tea() conversion
        assert_eq!(tea(&Value::Int32(42)).unwrap(), "42");
        assert_eq!(tea(&Value::Bool(true)).unwrap(), "true");
        assert_eq!(tea(&Value::String("hello".to_string())).unwrap(), "hello");
    }

    #[test]
    fn test_collection_operations() {
        // Test append()
        let slice = Value::Array(vec![Value::Int32(1), Value::Int32(2)]);
        let elements = vec![Value::Int32(3), Value::Int32(4)];
        let result = append(&slice, &elements).unwrap();
        if let Value::Array(arr) = result {
            assert_eq!(arr.len(), 4);
        } else {
            panic!("Expected array");
        }

        // Test len()
        let array = Value::Array(vec![Value::Int32(1), Value::Int32(2), Value::Int32(3)]);
        assert_eq!(len(&array).unwrap(), 3);
        
        let string = Value::String("hello".to_string());
        assert_eq!(len(&string).unwrap(), 5);

        // Test make()
        let empty_array = make("array", None).unwrap();
        assert!(matches!(empty_array, Value::Array(_)));
        
        let sized_array = make("array", Some(10)).unwrap();
        if let Value::Array(arr) = sized_array {
            assert!(arr.capacity() >= 10);
        }

        // Test new()
        assert_eq!(new("normie").unwrap(), Value::Int32(0));
        assert_eq!(new("tea").unwrap(), Value::String("".to_string()));
        assert_eq!(new("litean").unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_utility_functions() {
        // Test type_of()
        assert_eq!(type_of(&Value::Int32(42)), "normie");
        assert_eq!(type_of(&Value::String("hello".to_string())), "tea");
        assert_eq!(type_of(&Value::Bool(true)), "litean");

        // Test is_zero_value()
        assert!(is_zero_value(&Value::Int32(0)));
        assert!(!is_zero_value(&Value::Int32(42)));
        assert!(is_zero_value(&Value::String("".to_string())));
        assert!(!is_zero_value(&Value::String("hello".to_string())));

        // Test equal_values()
        assert!(equal_values(&Value::Int32(42), &Value::Int32(42)));
        assert!(!equal_values(&Value::Int32(42), &Value::Int32(43)));

        // Test clone_value()
        let original = Value::Int32(42);
        let cloned = clone_value(&original);
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_panic_recovery() {
        // Test try_unbothered() with success
        let result = try_unbothered(|| 42);
        assert_eq!(result.unwrap(), 42);

        // Test try_unbothered() with panic
        let result = try_unbothered(|| panic!("test panic"));
        assert!(result.is_err());
    }

    #[test]
    fn test_error_handling() {
        // Test invalid type conversions
        assert!(normie(&Value::Array(vec![])).is_err());
        assert!(snack(&Value::Object(HashMap::new())).is_err());
        
        // Test invalid collection operations
        assert!(len(&Value::Int32(42)).is_err());
        assert!(append(&Value::Int32(42), &[]).is_err());
        
        // Test invalid make() type
        assert!(make("invalid_type", None).is_err());
        assert!(new("invalid_type").is_err());
    }

    #[test]
    fn test_edge_cases() {
        // Test null conversions
        assert_eq!(lit(&Value::Null).unwrap(), false);
        assert_eq!(normie(&Value::Null).unwrap(), 0);
        assert_eq!(thicc(&Value::Null).unwrap(), 0);
        assert_eq!(snack(&Value::Null).unwrap(), 0.0);
        assert_eq!(meal(&Value::Null).unwrap(), 0.0);
        assert_eq!(tea(&Value::Null).unwrap(), "null");

        // Test range checking for normie()
        let big_int = Value::Int64(i64::MAX);
        assert!(normie(&big_int).is_err());

        // Test empty collections
        let empty_array = Value::Array(vec![]);
        assert_eq!(len(&empty_array).unwrap(), 0);
        assert!(is_zero_value(&empty_array));
    }

    #[test]
    fn test_module_functions() {
        // Test module initialization
        assert!(init_core().is_ok());

        // Test stats
        let stats = get_core_stats();
        assert!(stats.contains_key("version"));
        assert!(stats.contains_key("functions"));
    }
}
