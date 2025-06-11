/// Comprehensive assertion framework for CURSED unit testing
/// 
/// Provides a wide range of assertion functions for validating test expectations
/// including basic assertions, numeric comparisons, string operations, collection
/// validation, error handling, and advanced testing scenarios.

use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use crate::error::CursedError;
use crate::stdlib::value::Value;
use super::{TestError, assertion_error};

/// Result type for assertion operations
pub type AssertionResult = Result<(), TestError>;

/// Context information for assertion failures
#[derive(Debug, Clone)]
pub struct AssertionContext {
    pub file: String,
    pub line: u32,
    pub function: String,
    pub expression: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub message: Option<String>,
}

impl AssertionContext {
    pub fn new(file: &str, line: u32, function: &str, expression: &str) -> Self {
        Self {
            file: file.to_string(),
            line,
            function: function.to_string(),
            expression: expression.to_string(),
            expected: None,
            actual: None,
            message: None,
        }
    }

    pub fn with_values<T: Debug, U: Debug>(mut self, expected: T, actual: U) -> Self {
        self.expected = Some(format!("{:?}", expected));
        self.actual = Some(format!("{:?}", actual));
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}

/// Enhanced assertion error with context
#[derive(Debug, Clone)]
pub struct AssertionError {
    pub context: AssertionContext,
    pub details: String,
}

impl AssertionError {
    pub fn new(context: AssertionContext, details: String) -> Self {
        Self { context, details }
    }
}

impl std::fmt::Display for AssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Assertion failed in {}:{}", self.context.file, self.context.line)?;
        writeln!(f, "Function: {}", self.context.function)?;
        writeln!(f, "Expression: {}", self.context.expression)?;
        writeln!(f, "Details: {}", self.details)?;
        
        if let Some(expected) = &self.context.expected {
            writeln!(f, "Expected: {}", expected)?;
        }
        
        if let Some(actual) = &self.context.actual {
            writeln!(f, "Actual: {}", actual)?;
        }
        
        if let Some(message) = &self.context.message {
            writeln!(f, "Message: {}", message)?;
        }
        
        Ok(())
    }
}

// ============================================================================
// BASIC ASSERTIONS
// ============================================================================

/// Assert that a boolean value is true
pub fn assert_true(value: bool) -> AssertionResult {
    assert_true_with_message(value, None)
}

/// Assert that a boolean value is true with custom message
pub fn assert_true_with_message(value: bool, message: Option<String>) -> AssertionResult {
    if !value {
        let mut msg = "Expected true, got false".to_string();
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a boolean value is false
pub fn assert_false(value: bool) -> AssertionResult {
    assert_false_with_message(value, None)
}

/// Assert that a boolean value is false with custom message
pub fn assert_false_with_message(value: bool, message: Option<String>) -> AssertionResult {
    if value {
        let mut msg = "Expected false, got true".to_string();
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that two values are equal
pub fn assert_eq<T: PartialEq + Debug>(left: T, right: T) -> AssertionResult {
    assert_eq_with_message(left, right, None)
}

/// Assert that two values are equal with custom message
pub fn assert_eq_with_message<T: PartialEq + Debug>(left: T, right: T, message: Option<String>) -> AssertionResult {
    if left != right {
        let mut msg = format!("Values are not equal\nLeft: {:?}\nRight: {:?}", left, right);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that two values are not equal
pub fn assert_ne<T: PartialEq + Debug>(left: T, right: T) -> AssertionResult {
    assert_ne_with_message(left, right, None)
}

/// Assert that two values are not equal with custom message
pub fn assert_ne_with_message<T: PartialEq + Debug>(left: T, right: T, message: Option<String>) -> AssertionResult {
    if left == right {
        let mut msg = format!("Values should not be equal: {:?}", left);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a value is null/None
pub fn assert_null<T: Debug>(value: Option<T>) -> AssertionResult {
    assert_null_with_message(value, None)
}

/// Assert that a value is null/None with custom message
pub fn assert_null_with_message<T: Debug>(value: Option<T>, message: Option<String>) -> AssertionResult {
    if value.is_some() {
        let mut msg = format!("Expected None, got Some({:?})", value.unwrap());
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a value is not null/None
pub fn assert_not_null<T: Debug>(value: Option<T>) -> AssertionResult {
    assert_not_null_with_message(value, None)
}

/// Assert that a value is not null/None with custom message
pub fn assert_not_null_with_message<T: Debug>(value: Option<T>, message: Option<String>) -> AssertionResult {
    if value.is_none() {
        let mut msg = "Expected Some, got None".to_string();
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

// ============================================================================
// NUMERIC ASSERTIONS
// ============================================================================

/// Assert that a numeric value is greater than another
pub fn assert_greater<T: PartialOrd + Debug>(left: T, right: T) -> AssertionResult {
    assert_greater_with_message(left, right, None)
}

/// Assert that a numeric value is greater than another with custom message
pub fn assert_greater_with_message<T: PartialOrd + Debug>(left: T, right: T, message: Option<String>) -> AssertionResult {
    if !(left > right) {
        let mut msg = format!("{:?} is not greater than {:?}", left, right);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a numeric value is greater than or equal to another
pub fn assert_greater_equal<T: PartialOrd + Debug>(left: T, right: T) -> AssertionResult {
    assert_greater_equal_with_message(left, right, None)
}

/// Assert that a numeric value is greater than or equal to another with custom message
pub fn assert_greater_equal_with_message<T: PartialOrd + Debug>(left: T, right: T, message: Option<String>) -> AssertionResult {
    if !(left >= right) {
        let mut msg = format!("{:?} is not greater than or equal to {:?}", left, right);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a numeric value is less than another
pub fn assert_less<T: PartialOrd + Debug>(left: T, right: T) -> AssertionResult {
    assert_less_with_message(left, right, None)
}

/// Assert that a numeric value is less than another with custom message
pub fn assert_less_with_message<T: PartialOrd + Debug>(left: T, right: T, message: Option<String>) -> AssertionResult {
    if !(left < right) {
        let mut msg = format!("{:?} is not less than {:?}", left, right);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a numeric value is less than or equal to another
pub fn assert_less_equal<T: PartialOrd + Debug>(left: T, right: T) -> AssertionResult {
    assert_less_equal_with_message(left, right, None)
}

/// Assert that a numeric value is less than or equal to another with custom message
pub fn assert_less_equal_with_message<T: PartialOrd + Debug>(left: T, right: T, message: Option<String>) -> AssertionResult {
    if !(left <= right) {
        let mut msg = format!("{:?} is not less than or equal to {:?}", left, right);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a floating point value is close to another within epsilon
pub fn assert_close_to(left: f64, right: f64, epsilon: f64) -> AssertionResult {
    assert_close_to_with_message(left, right, epsilon, None)
}

/// Assert that a floating point value is close to another within epsilon with custom message
pub fn assert_close_to_with_message(left: f64, right: f64, epsilon: f64, message: Option<String>) -> AssertionResult {
    let diff = (left - right).abs();
    if diff > epsilon {
        let mut msg = format!("{} is not close to {} (difference: {}, epsilon: {})", left, right, diff, epsilon);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a numeric value is between two bounds (inclusive)
pub fn assert_between<T: PartialOrd + Debug>(value: T, min: T, max: T) -> AssertionResult {
    assert_between_with_message(value, min, max, None)
}

/// Assert that a numeric value is between two bounds (inclusive) with custom message
pub fn assert_between_with_message<T: PartialOrd + Debug>(value: T, min: T, max: T, message: Option<String>) -> AssertionResult {
    if !(value >= min && value <= max) {
        let mut msg = format!("{:?} is not between {:?} and {:?}", value, min, max);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a numeric value is positive
pub fn assert_positive<T: PartialOrd + Default + Debug>(value: T) -> AssertionResult {
    let zero = T::default();
    assert_greater_with_message(value, zero, Some("Value should be positive".to_string()))
}

/// Assert that a numeric value is negative
pub fn assert_negative<T: PartialOrd + Default + Debug>(value: T) -> AssertionResult {
    let zero = T::default();
    assert_less_with_message(value, zero, Some("Value should be negative".to_string()))
}

/// Assert that a numeric value is zero
pub fn assert_zero<T: PartialEq + Default + Debug>(value: T) -> AssertionResult {
    let zero = T::default();
    assert_eq_with_message(value, zero, Some("Value should be zero".to_string()))
}

// ============================================================================
// STRING ASSERTIONS
// ============================================================================

/// Assert that a string contains a substring
pub fn assert_contains(haystack: &str, needle: &str) -> AssertionResult {
    assert_contains_with_message(haystack, needle, None)
}

/// Assert that a string contains a substring with custom message
pub fn assert_contains_with_message(haystack: &str, needle: &str, message: Option<String>) -> AssertionResult {
    if !haystack.contains(needle) {
        let mut msg = format!("String '{}' does not contain '{}'", haystack, needle);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a string does not contain a substring
pub fn assert_not_contains(haystack: &str, needle: &str) -> AssertionResult {
    assert_not_contains_with_message(haystack, needle, None)
}

/// Assert that a string does not contain a substring with custom message
pub fn assert_not_contains_with_message(haystack: &str, needle: &str, message: Option<String>) -> AssertionResult {
    if haystack.contains(needle) {
        let mut msg = format!("String '{}' should not contain '{}'", haystack, needle);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a string starts with a prefix
pub fn assert_starts_with(string: &str, prefix: &str) -> AssertionResult {
    assert_starts_with_with_message(string, prefix, None)
}

/// Assert that a string starts with a prefix with custom message
pub fn assert_starts_with_with_message(string: &str, prefix: &str, message: Option<String>) -> AssertionResult {
    if !string.starts_with(prefix) {
        let mut msg = format!("String '{}' does not start with '{}'", string, prefix);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a string ends with a suffix
pub fn assert_ends_with(string: &str, suffix: &str) -> AssertionResult {
    assert_ends_with_with_message(string, suffix, None)
}

/// Assert that a string ends with a suffix with custom message
pub fn assert_ends_with_with_message(string: &str, suffix: &str, message: Option<String>) -> AssertionResult {
    if !string.ends_with(suffix) {
        let mut msg = format!("String '{}' does not end with '{}'", string, suffix);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a string matches a regular expression
pub fn assert_matches_regex(string: &str, pattern: &str) -> AssertionResult {
    assert_matches_regex_with_message(string, pattern, None)
}

/// Assert that a string matches a regular expression with custom message
pub fn assert_matches_regex_with_message(string: &str, pattern: &str, message: Option<String>) -> AssertionResult {
    // Simple pattern matching - in a real implementation, you'd use a regex library
    // For now, we'll do basic wildcard matching
    let matches = if pattern == "*" {
        true
    } else if pattern.starts_with('*') && pattern.ends_with('*') {
        let inner = &pattern[1..pattern.len()-1];
        string.contains(inner)
    } else if pattern.starts_with('*') {
        let suffix = &pattern[1..];
        string.ends_with(suffix)
    } else if pattern.ends_with('*') {
        let prefix = &pattern[..pattern.len()-1];
        string.starts_with(prefix)
    } else {
        string == pattern
    };

    if !matches {
        let mut msg = format!("String '{}' does not match pattern '{}'", string, pattern);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a string is empty
pub fn assert_empty_string(string: &str) -> AssertionResult {
    assert_empty_string_with_message(string, None)
}

/// Assert that a string is empty with custom message
pub fn assert_empty_string_with_message(string: &str, message: Option<String>) -> AssertionResult {
    if !string.is_empty() {
        let mut msg = format!("String should be empty, got '{}'", string);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a string has a specific length
pub fn assert_length(string: &str, expected_length: usize) -> AssertionResult {
    assert_length_with_message(string, expected_length, None)
}

/// Assert that a string has a specific length with custom message
pub fn assert_length_with_message(string: &str, expected_length: usize, message: Option<String>) -> AssertionResult {
    let actual_length = string.len();
    if actual_length != expected_length {
        let mut msg = format!("String length should be {}, got {} (string: '{}')", expected_length, actual_length, string);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

// ============================================================================
// COLLECTION ASSERTIONS
// ============================================================================

/// Assert that a collection is empty
pub fn assert_empty<T>(collection: &[T]) -> AssertionResult {
    assert_empty_with_message(collection, None)
}

/// Assert that a collection is empty with custom message
pub fn assert_empty_with_message<T>(collection: &[T], message: Option<String>) -> AssertionResult {
    if !collection.is_empty() {
        let mut msg = format!("Collection should be empty, got {} elements", collection.len());
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a collection is not empty
pub fn assert_not_empty<T>(collection: &[T]) -> AssertionResult {
    assert_not_empty_with_message(collection, None)
}

/// Assert that a collection is not empty with custom message
pub fn assert_not_empty_with_message<T>(collection: &[T], message: Option<String>) -> AssertionResult {
    if collection.is_empty() {
        let mut msg = "Collection should not be empty".to_string();
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a collection contains a specific element
pub fn assert_contains_element<T: PartialEq + Debug>(collection: &[T], element: &T) -> AssertionResult {
    assert_contains_element_with_message(collection, element, None)
}

/// Assert that a collection contains a specific element with custom message
pub fn assert_contains_element_with_message<T: PartialEq + Debug>(collection: &[T], element: &T, message: Option<String>) -> AssertionResult {
    if !collection.contains(element) {
        let mut msg = format!("Collection does not contain element {:?}", element);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a collection does not contain a specific element
pub fn assert_not_contains_element<T: PartialEq + Debug>(collection: &[T], element: &T) -> AssertionResult {
    assert_not_contains_element_with_message(collection, element, None)
}

/// Assert that a collection does not contain a specific element with custom message
pub fn assert_not_contains_element_with_message<T: PartialEq + Debug>(collection: &[T], element: &T, message: Option<String>) -> AssertionResult {
    if collection.contains(element) {
        let mut msg = format!("Collection should not contain element {:?}", element);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a collection has a specific length
pub fn assert_has_length<T>(collection: &[T], expected_length: usize) -> AssertionResult {
    assert_has_length_with_message(collection, expected_length, None)
}

/// Assert that a collection has a specific length with custom message
pub fn assert_has_length_with_message<T>(collection: &[T], expected_length: usize, message: Option<String>) -> AssertionResult {
    let actual_length = collection.len();
    if actual_length != expected_length {
        let mut msg = format!("Collection length should be {}, got {}", expected_length, actual_length);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that all elements in a boolean collection are true
pub fn assert_all_true(collection: &[bool]) -> AssertionResult {
    assert_all_true_with_message(collection, None)
}

/// Assert that all elements in a boolean collection are true with custom message
pub fn assert_all_true_with_message(collection: &[bool], message: Option<String>) -> AssertionResult {
    for (index, &value) in collection.iter().enumerate() {
        if !value {
            let mut msg = format!("Element at index {} is false, expected all true", index);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            return Err(assertion_error(&msg));
        }
    }
    Ok(())
}

/// Assert that at least one element in a boolean collection is true
pub fn assert_any_true(collection: &[bool]) -> AssertionResult {
    assert_any_true_with_message(collection, None)
}

/// Assert that at least one element in a boolean collection is true with custom message
pub fn assert_any_true_with_message(collection: &[bool], message: Option<String>) -> AssertionResult {
    if !collection.iter().any(|&x| x) {
        let mut msg = "No elements are true, expected at least one true".to_string();
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that no elements in a boolean collection are true
pub fn assert_none_true(collection: &[bool]) -> AssertionResult {
    assert_none_true_with_message(collection, None)
}

/// Assert that no elements in a boolean collection are true with custom message
pub fn assert_none_true_with_message(collection: &[bool], message: Option<String>) -> AssertionResult {
    for (index, &value) in collection.iter().enumerate() {
        if value {
            let mut msg = format!("Element at index {} is true, expected all false", index);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            return Err(assertion_error(&msg));
        }
    }
    Ok(())
}

// ============================================================================
// ERROR ASSERTIONS
// ============================================================================

/// Assert that a Result is an error
pub fn assert_error<T: Debug, E: Debug>(result: Result<T, E>) -> AssertionResult {
    assert_error_with_message(result, None)
}

/// Assert that a Result is an error with custom message
pub fn assert_error_with_message<T: Debug, E: Debug>(result: Result<T, E>, message: Option<String>) -> AssertionResult {
    match result {
        Ok(value) => {
            let mut msg = format!("Expected error, got Ok({:?})", value);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            Err(assertion_error(&msg))
        }
        Err(_) => Ok(())
    }
}

/// Assert that a Result is not an error
pub fn assert_no_error<T: Debug, E: Debug>(result: Result<T, E>) -> AssertionResult {
    assert_no_error_with_message(result, None)
}

/// Assert that a Result is not an error with custom message
pub fn assert_no_error_with_message<T: Debug, E: Debug>(result: Result<T, E>, message: Option<String>) -> AssertionResult {
    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            let mut msg = format!("Expected Ok, got Err({:?})", error);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            Err(assertion_error(&msg))
        }
    }
}

/// Assert that a Result contains a specific error type
pub fn assert_error_type<T: Debug, E: Debug + PartialEq>(result: Result<T, E>, expected_error: E) -> AssertionResult {
    assert_error_type_with_message(result, expected_error, None)
}

/// Assert that a Result contains a specific error type with custom message
pub fn assert_error_type_with_message<T: Debug, E: Debug + PartialEq>(result: Result<T, E>, expected_error: E, message: Option<String>) -> AssertionResult {
    match result {
        Ok(value) => {
            let mut msg = format!("Expected error {:?}, got Ok({:?})", expected_error, value);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            Err(assertion_error(&msg))
        }
        Err(actual_error) => {
            if actual_error != expected_error {
                let mut msg = format!("Expected error {:?}, got {:?}", expected_error, actual_error);
                if let Some(custom_msg) = message {
                    msg = format!("{}: {}", custom_msg, msg);
                }
                return Err(assertion_error(&msg));
            }
            Ok(())
        }
    }
}

/// Assert that a Result contains an error with a specific message
pub fn assert_error_message<T: Debug>(result: Result<T, String>, expected_message: &str) -> AssertionResult {
    assert_error_message_with_message(result, expected_message, None)
}

/// Assert that a Result contains an error with a specific message with custom message
pub fn assert_error_message_with_message<T: Debug>(result: Result<T, String>, expected_message: &str, message: Option<String>) -> AssertionResult {
    match result {
        Ok(value) => {
            let mut msg = format!("Expected error with message '{}', got Ok({:?})", expected_message, value);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            Err(assertion_error(&msg))
        }
        Err(actual_message) => {
            if actual_message != expected_message {
                let mut msg = format!("Expected error message '{}', got '{}'", expected_message, actual_message);
                if let Some(custom_msg) = message {
                    msg = format!("{}: {}", custom_msg, msg);
                }
                return Err(assertion_error(&msg));
            }
            Ok(())
        }
    }
}

/// Assert that a closure panics when executed
pub fn assert_panic<F: FnOnce() + std::panic::UnwindSafe>(f: F) -> AssertionResult {
    assert_panic_with_message(f, None)
}

/// Assert that a closure panics when executed with custom message
pub fn assert_panic_with_message<F: FnOnce() + std::panic::UnwindSafe>(f: F, message: Option<String>) -> AssertionResult {
    let result = std::panic::catch_unwind(f);
    match result {
        Ok(_) => {
            let mut msg = "Expected panic, but function completed normally".to_string();
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            Err(assertion_error(&msg))
        }
        Err(_) => Ok(())
    }
}

/// Assert that a closure does not panic when executed
pub fn assert_no_panic<F: FnOnce() -> T + std::panic::UnwindSafe, T>(f: F) -> AssertionResult {
    assert_no_panic_with_message(f, None)
}

/// Assert that a closure does not panic when executed with custom message
pub fn assert_no_panic_with_message<F: FnOnce() -> T + std::panic::UnwindSafe, T>(f: F, message: Option<String>) -> AssertionResult {
    let result = std::panic::catch_unwind(f);
    match result {
        Ok(_) => Ok(()),
        Err(panic_info) => {
            let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            
            let mut msg = format!("Expected no panic, but function panicked: {}", panic_msg);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            Err(assertion_error(&msg))
        }
    }
}

// ============================================================================
// ADVANCED ASSERTIONS
// ============================================================================

/// Assert that a condition becomes true within a timeout
pub fn assert_eventually<F: Fn() -> bool>(condition: F, timeout: Duration) -> AssertionResult {
    assert_eventually_with_message(condition, timeout, None)
}

/// Assert that a condition becomes true within a timeout with custom message
pub fn assert_eventually_with_message<F: Fn() -> bool>(condition: F, timeout: Duration, message: Option<String>) -> AssertionResult {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if condition() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    
    let mut msg = format!("Condition did not become true within {:?}", timeout);
    if let Some(custom_msg) = message {
        msg = format!("{}: {}", custom_msg, msg);
    }
    Err(assertion_error(&msg))
}

/// Assert that a closure completes within a timeout
pub fn assert_within_timeout<F: FnOnce() -> T + std::panic::UnwindSafe, T>(f: F, timeout: Duration) -> AssertionResult {
    assert_within_timeout_with_message(f, timeout, None)
}

/// Assert that a closure completes within a timeout with custom message
pub fn assert_within_timeout_with_message<F: FnOnce() -> T + std::panic::UnwindSafe, T>(f: F, timeout: Duration, message: Option<String>) -> AssertionResult {
    let start = Instant::now();
    
    // In a real implementation, you might use async/await or threading for proper timeout handling
    // For now, we'll just execute the function and check if it completes quickly
    let result = std::panic::catch_unwind(f);
    let elapsed = start.elapsed();
    
    if elapsed > timeout {
        let mut msg = format!("Function took {:?}, expected to complete within {:?}", elapsed, timeout);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    
    match result {
        Ok(_) => Ok(()),
        Err(panic_info) => {
            let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };
            
            let mut msg = format!("Function panicked within timeout: {}", panic_msg);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            Err(assertion_error(&msg))
        }
    }
}

/// Assert that a file exists
pub fn assert_file_exists(path: &str) -> AssertionResult {
    assert_file_exists_with_message(path, None)
}

/// Assert that a file exists with custom message
pub fn assert_file_exists_with_message(path: &str, message: Option<String>) -> AssertionResult {
    if !std::path::Path::new(path).exists() {
        let mut msg = format!("File '{}' does not exist", path);
        if let Some(custom_msg) = message {
            msg = format!("{}: {}", custom_msg, msg);
        }
        return Err(assertion_error(&msg));
    }
    Ok(())
}

/// Assert that a file contains specific content
pub fn assert_file_content(path: &str, expected_content: &str) -> AssertionResult {
    assert_file_content_with_message(path, expected_content, None)
}

/// Assert that a file contains specific content with custom message
pub fn assert_file_content_with_message(path: &str, expected_content: &str, message: Option<String>) -> AssertionResult {
    match std::fs::read_to_string(path) {
        Ok(actual_content) => {
            if actual_content != expected_content {
                let mut msg = format!("File '{}' content mismatch\nExpected: '{}'\nActual: '{}'", 
                                    path, expected_content, actual_content);
                if let Some(custom_msg) = message {
                    msg = format!("{}: {}", custom_msg, msg);
                }
                return Err(assertion_error(&msg));
            }
            Ok(())
        }
        Err(err) => {
            let mut msg = format!("Failed to read file '{}': {}", path, err);
            if let Some(custom_msg) = message {
                msg = format!("{}: {}", custom_msg, msg);
            }
            Err(assertion_error(&msg))
        }
    }
}
