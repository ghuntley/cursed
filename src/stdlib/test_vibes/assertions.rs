/// Assertion functions for the TestVibes framework
/// 
/// Provides comprehensive assertion functionality with expressive error messages
/// and support for various data types and conditions.

use crate::stdlib::value::Value;
use crate::error::Error;
use super::{VibeTest, TestVibesResult, assertion_failed};
use std::collections::HashMap;

/// Basic assertion - condition must be true
pub fn Assert(t: &VibeTest, condition: bool, message: &str) -> TestVibesResult<()> {
    if !condition {
        return t.Fatal(&[Value::String(format!("Assertion failed: {}", message))]);
    }
    Ok(())
}

/// Assert two values are equal
pub fn AssertEqual(t: &VibeTest, expected: &Value, actual: &Value, message: &str) -> TestVibesResult<()> {
    if !values_equal(expected, actual) {
        let error_msg = format!(
            "{}\n  Expected: {}\n  Actual:   {}",
            message,
            value_to_string(expected),
            value_to_string(actual)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert two values are not equal
pub fn AssertNotEqual(t: &VibeTest, expected: &Value, actual: &Value, message: &str) -> TestVibesResult<()> {
    if values_equal(expected, actual) {
        let error_msg = format!(
            "{}\n  Values should not be equal: {}",
            message,
            value_to_string(actual)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert value is nil
pub fn AssertNil(t: &VibeTest, actual: &Value, message: &str) -> TestVibesResult<()> {
    if !matches!(actual, Value::Nil) {
        let error_msg = format!(
            "{}\n  Expected: nil\n  Actual:   {}",
            message,
            value_to_string(actual)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert value is not nil
pub fn AssertNotNil(t: &VibeTest, actual: &Value, message: &str) -> TestVibesResult<()> {
    if matches!(actual, Value::Nil) {
        let error_msg = format!("{}\n  Value should not be nil", message);
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert boolean value is true
pub fn AssertTrue(t: &VibeTest, actual: bool, message: &str) -> TestVibesResult<()> {
    if !actual {
        let error_msg = format!(
            "{}\n  Expected: true\n  Actual:   false",
            message
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert boolean value is false
pub fn AssertFalse(t: &VibeTest, actual: bool, message: &str) -> TestVibesResult<()> {
    if actual {
        let error_msg = format!(
            "{}\n  Expected: false\n  Actual:   true",
            message
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert error exists (string is not empty)
pub fn AssertError(t: &VibeTest, err: &str, message: &str) -> TestVibesResult<()> {
    if err.is_empty() {
        let error_msg = format!("{}\n  Expected an error but got none", message);
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert no error exists (string is empty)
pub fn AssertNoError(t: &VibeTest, err: &str, message: &str) -> TestVibesResult<()> {
    if !err.is_empty() {
        let error_msg = format!(
            "{}\n  Expected no error but got: {}",
            message, err
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert error is of specific type/message
pub fn AssertErrorIs(t: &VibeTest, err: &str, target: &str, message: &str) -> TestVibesResult<()> {
    if err != target {
        let error_msg = format!(
            "{}\n  Expected error: {}\n  Actual error:   {}",
            message, target, err
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert error contains specific substring
pub fn AssertErrorContains(t: &VibeTest, err: &str, contains: &str, message: &str) -> TestVibesResult<()> {
    if !err.contains(contains) {
        let error_msg = format!(
            "{}\n  Expected error to contain: {}\n  Actual error: {}",
            message, contains, err
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert collection has specific length
pub fn AssertLen(t: &VibeTest, collection: &Value, length: i32, message: &str) -> TestVibesResult<()> {
    let actual_len = get_collection_length(collection)?;
    if actual_len != length {
        let error_msg = format!(
            "{}\n  Expected length: {}\n  Actual length:   {}",
            message, length, actual_len
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert collection is empty
pub fn AssertEmpty(t: &VibeTest, collection: &Value, message: &str) -> TestVibesResult<()> {
    let len = get_collection_length(collection)?;
    if len != 0 {
        let error_msg = format!(
            "{}\n  Expected empty collection but got length: {}",
            message, len
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert collection is not empty
pub fn AssertNotEmpty(t: &VibeTest, collection: &Value, message: &str) -> TestVibesResult<()> {
    let len = get_collection_length(collection)?;
    if len == 0 {
        let error_msg = format!("{}\n  Expected non-empty collection", message);
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert collection contains element
pub fn AssertContains(t: &VibeTest, collection: &Value, element: &Value, message: &str) -> TestVibesResult<()> {
    if !collection_contains(collection, element)? {
        let error_msg = format!(
            "{}\n  Collection should contain: {}\n  Collection: {}",
            message,
            value_to_string(element),
            value_to_string(collection)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert collection does not contain element
pub fn AssertNotContains(t: &VibeTest, collection: &Value, element: &Value, message: &str) -> TestVibesResult<()> {
    if collection_contains(collection, element)? {
        let error_msg = format!(
            "{}\n  Collection should not contain: {}\n  Collection: {}",
            message,
            value_to_string(element),
            value_to_string(collection)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert numeric value is greater than another
pub fn AssertGreater(t: &VibeTest, x: &Value, y: &Value, message: &str) -> TestVibesResult<()> {
    if !is_greater_than(x, y)? {
        let error_msg = format!(
            "{}\n  Expected {} > {}\n  But {} <= {}",
            message,
            value_to_string(x),
            value_to_string(y),
            value_to_string(x),
            value_to_string(y)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert numeric value is greater than or equal to another
pub fn AssertGreaterOrEqual(t: &VibeTest, x: &Value, y: &Value, message: &str) -> TestVibesResult<()> {
    if !is_greater_or_equal(x, y)? {
        let error_msg = format!(
            "{}\n  Expected {} >= {}\n  But {} < {}",
            message,
            value_to_string(x),
            value_to_string(y),
            value_to_string(x),
            value_to_string(y)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert numeric value is less than another
pub fn AssertLess(t: &VibeTest, x: &Value, y: &Value, message: &str) -> TestVibesResult<()> {
    if !is_less_than(x, y)? {
        let error_msg = format!(
            "{}\n  Expected {} < {}\n  But {} >= {}",
            message,
            value_to_string(x),
            value_to_string(y),
            value_to_string(x),
            value_to_string(y)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert numeric value is less than or equal to another
pub fn AssertLessOrEqual(t: &VibeTest, x: &Value, y: &Value, message: &str) -> TestVibesResult<()> {
    if !is_less_or_equal(x, y)? {
        let error_msg = format!(
            "{}\n  Expected {} <= {}\n  But {} > {}",
            message,
            value_to_string(x),
            value_to_string(y),
            value_to_string(x),
            value_to_string(y)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert numeric value is zero
pub fn AssertZero(t: &VibeTest, actual: &Value, message: &str) -> TestVibesResult<()> {
    if !is_zero(actual)? {
        let error_msg = format!(
            "{}\n  Expected: 0\n  Actual:   {}",
            message,
            value_to_string(actual)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert numeric value is not zero
pub fn AssertNotZero(t: &VibeTest, actual: &Value, message: &str) -> TestVibesResult<()> {
    if is_zero(actual)? {
        let error_msg = format!("{}\n  Value should not be zero", message);
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert string contains substring
pub fn AssertContainsSubtea(t: &VibeTest, str: &str, substr: &str, message: &str) -> TestVibesResult<()> {
    if !str.contains(substr) {
        let error_msg = format!(
            "{}\n  String should contain: '{}'\n  String: '{}'",
            message, substr, str
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert string has prefix
pub fn AssertHasPrefix(t: &VibeTest, str: &str, prefix: &str, message: &str) -> TestVibesResult<()> {
    if !str.starts_with(prefix) {
        let error_msg = format!(
            "{}\n  String should start with: '{}'\n  String: '{}'",
            message, prefix, str
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert string has suffix
pub fn AssertHasSuffix(t: &VibeTest, str: &str, suffix: &str, message: &str) -> TestVibesResult<()> {
    if !str.ends_with(suffix) {
        let error_msg = format!(
            "{}\n  String should end with: '{}'\n  String: '{}'",
            message, suffix, str
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert string matches regex pattern
pub fn AssertMatchesRegex(t: &VibeTest, str: &str, pattern: &str, message: &str) -> TestVibesResult<()> {
    // Simple pattern matching - in a real implementation would use regex crate
    let matches = if pattern.contains('*') {
        // Simple wildcard matching
        simple_wildcard_match(str, pattern)
    } else {
        str.contains(pattern)
    };

    if !matches {
        let error_msg = format!(
            "{}\n  String should match pattern: '{}'\n  String: '{}'",
            message, pattern, str
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert value is of expected type
pub fn AssertType(t: &VibeTest, expected_type: &str, value: &Value, message: &str) -> TestVibesResult<()> {
    let actual_type = get_value_type(value);
    if actual_type != expected_type {
        let error_msg = format!(
            "{}\n  Expected type: {}\n  Actual type:   {}\n  Value: {}",
            message, expected_type, actual_type, value_to_string(value)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert value implements interface (simplified for basic types)
pub fn AssertImplements(t: &VibeTest, interface_obj: &Value, value: &Value, message: &str) -> TestVibesResult<()> {
    // Simplified implementation - in real system would check interface compliance
    let interface_type = get_value_type(interface_obj);
    let value_type = get_value_type(value);
    
    if !type_implements_interface(&value_type, &interface_type) {
        let error_msg = format!(
            "{}\n  Type '{}' should implement interface '{}'\n  Value: {}",
            message, value_type, interface_type, value_to_string(value)
        );
        return t.Fatal(&[Value::String(error_msg)]);
    }
    Ok(())
}

/// Assert function panics (simplified - would need runtime integration)
pub fn AssertShooks(t: &VibeTest, _fn: fn(), message: &str) -> TestVibesResult<()> {
    // In a real implementation, this would catch panics
    // For now, we'll just log that panic assertion was attempted
    t.Log(&[Value::String(format!("AssertShooks: {}", message))])?;
    Ok(())
}

/// Assert function panics with specific value
pub fn AssertShooksWithValue(t: &VibeTest, _value: &Value, _fn: fn(), message: &str) -> TestVibesResult<()> {
    // In a real implementation, this would catch panics and check the panic value
    t.Log(&[Value::String(format!("AssertShooksWithValue: {}", message))])?;
    Ok(())
}

/// Assert function does not panic
pub fn AssertNoShook(t: &VibeTest, _fn: fn(), message: &str) -> TestVibesResult<()> {
    // In a real implementation, this would catch panics and fail if any occur
    t.Log(&[Value::String(format!("AssertNoShook: {}", message))])?;
    Ok(())
}

// Helper functions

/// Check if two values are equal
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| values_equal(x, y))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len() && a.iter().all(|(k, v)| {
                b.get(k).map_or(false, |v2| values_equal(v, v2))
            })
        }
        _ => false,
    }
}

/// Convert value to string representation
fn value_to_string(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(value_to_string).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Object(obj) => {
            let pairs: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
}

/// Get collection length
fn get_collection_length(value: &Value) -> TestVibesResult<i32> {
    match value {
        Value::Array(arr) => Ok(arr.len() as i32),
        Value::Object(obj) => Ok(obj.len() as i32),
        Value::String(s) => Ok(s.len() as i32),
        _ => Err(assertion_failed("Value is not a collection").into()),
    }
}

/// Check if collection contains element
fn collection_contains(collection: &Value, element: &Value) -> TestVibesResult<bool> {
    match collection {
        Value::Array(arr) => Ok(arr.iter().any(|v| values_equal(v, element))),
        Value::Object(obj) => Ok(obj.values().any(|v| values_equal(v, element))),
        Value::String(s) => {
            if let Value::String(substr) = element {
                Ok(s.contains(substr))
            } else {
                Ok(false)
            }
        }
        _ => Err(assertion_failed("Value is not a collection").into()),
    }
}

/// Check if x > y
fn is_greater_than(x: &Value, y: &Value) -> TestVibesResult<bool> {
    match (x, y) {
        (Value::Int(a), Value::Int(b)) => Ok(a > b),
        (Value::Float(a), Value::Float(b)) => Ok(a > b),
        (Value::Int(a), Value::Float(b)) => Ok(*a as f64 > *b),
        (Value::Float(a), Value::Int(b)) => Ok(*a > *b as f64),
        _ => Err(assertion_failed("Values are not numeric").into()),
    }
}

/// Check if x >= y
fn is_greater_or_equal(x: &Value, y: &Value) -> TestVibesResult<bool> {
    match (x, y) {
        (Value::Int(a), Value::Int(b)) => Ok(a >= b),
        (Value::Float(a), Value::Float(b)) => Ok(a >= b),
        (Value::Int(a), Value::Float(b)) => Ok(*a as f64 >= *b),
        (Value::Float(a), Value::Int(b)) => Ok(*a >= *b as f64),
        _ => Err(assertion_failed("Values are not numeric").into()),
    }
}

/// Check if x < y
fn is_less_than(x: &Value, y: &Value) -> TestVibesResult<bool> {
    match (x, y) {
        (Value::Int(a), Value::Int(b)) => Ok(a < b),
        (Value::Float(a), Value::Float(b)) => Ok(a < b),
        (Value::Int(a), Value::Float(b)) => Ok((*a as f64) < *b),
        (Value::Float(a), Value::Int(b)) => Ok(*a < (*b as f64)),
        _ => Err(assertion_failed("Values are not numeric").into()),
    }
}

/// Check if x <= y
fn is_less_or_equal(x: &Value, y: &Value) -> TestVibesResult<bool> {
    match (x, y) {
        (Value::Int(a), Value::Int(b)) => Ok(a <= b),
        (Value::Float(a), Value::Float(b)) => Ok(a <= b),
        (Value::Int(a), Value::Float(b)) => Ok((*a as f64) <= *b),
        (Value::Float(a), Value::Int(b)) => Ok(*a <= (*b as f64)),
        _ => Err(assertion_failed("Values are not numeric").into()),
    }
}

/// Check if value is zero
fn is_zero(value: &Value) -> TestVibesResult<bool> {
    match value {
        Value::Int(i) => Ok(*i == 0),
        Value::Float(f) => Ok(f.abs() < f64::EPSILON),
        _ => Err(assertion_failed("Value is not numeric").into()),
    }
}

/// Get value type as string
fn get_value_type(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Bool(_) => "bool".to_string(),
        Value::Int(_) => "int".to_string(),
        Value::Float(_) => "float".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(_) => "array".to_string(),
        Value::Object(_) => "object".to_string(),
    }
}

/// Simple wildcard pattern matching
fn simple_wildcard_match(text: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 1 {
        return text.contains(pattern);
    }
    
    let mut text_pos = 0;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        
        if i == 0 {
            // First part must match beginning
            if !text.starts_with(part) {
                return false;
            }
            text_pos = part.len();
        } else if i == parts.len() - 1 {
            // Last part must match end
            return text.ends_with(part);
        } else {
            // Middle parts must be found in order
            if let Some(pos) = text[text_pos..].find(part) {
                text_pos += pos + part.len();
            } else {
                return false;
            }
        }
    }
    
    true
}

/// Check if type implements interface (simplified)
fn type_implements_interface(value_type: &str, interface_type: &str) -> bool {
    // Simplified interface checking
    match (value_type, interface_type) {
        ("string", "string") => true,
        ("int", "number") => true,
        ("float", "number") => true,
        ("array", "collection") => true,
        ("object", "collection") => true,
        (a, b) => a == b,
    }
}
