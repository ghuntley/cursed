//! Assertion functions for the CURSED testing framework

use crate::error::CursedError;
use super::core::VibeTest;

/// Result type for assertion operations
pub type AssertionResult = Result<(), CursedError>;

/// Assert that two values are equal
pub fn assert_equal<T>(actual: T, expected: T, message: &str) -> AssertionResult 
where
    T: PartialEq + std::fmt::Debug
{
    if actual == expected {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected: {:?}, got: {:?}", 
            message, expected, actual
        )))
    }
}

/// Assert that two values are equal (with test context)
pub fn assert_equal_with_test<T>(test: &mut VibeTest, actual: T, expected: T, message: &str) -> AssertionResult 
where
    T: PartialEq + std::fmt::Debug
{
    if actual == expected {
        Ok(())
    } else {
        let error_msg = format!(
            "Assertion failed: {} - expected: {:?}, got: {:?}", 
            message, expected, actual
        );
        test.fail(&error_msg);
        Err(CursedError::runtime_error(&error_msg))
    }
}

/// Assert that a value is true
pub fn assert_true(value: bool, message: &str) -> AssertionResult {
    if value {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected true, got false", 
            message
        )))
    }
}

/// Assert that a value is true (with test context)
pub fn assert_true_with_test(test: &mut VibeTest, value: bool, message: &str) -> AssertionResult {
    if value {
        Ok(())
    } else {
        let error_msg = format!(
            "Assertion failed: {} - expected true, got false", 
            message
        );
        test.fail(&error_msg);
        Err(CursedError::runtime_error(&error_msg))
    }
}

/// Assert that a value is false
pub fn assert_false(value: bool, message: &str) -> AssertionResult {
    if !value {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected false, got true", 
            message
        )))
    }
}

/// Assert that a value is false (with test context)
pub fn assert_false_with_test(test: &mut VibeTest, value: bool, message: &str) -> AssertionResult {
    if !value {
        Ok(())
    } else {
        let error_msg = format!(
            "Assertion failed: {} - expected false, got true", 
            message
        );
        test.fail(&error_msg);
        Err(CursedError::runtime_error(&error_msg))
    }
}

/// Assert that a string contains a substring
pub fn assert_contains_substr(text: &str, substr: &str, message: &str) -> AssertionResult {
    if text.contains(substr) {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - '{}' does not contain '{}'", 
            message, text, substr
        )))
    }
}

/// Assert that a string has a prefix
pub fn assert_has_prefix(text: &str, prefix: &str, message: &str) -> AssertionResult {
    if text.starts_with(prefix) {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - '{}' does not start with '{}'", 
            message, text, prefix
        )))
    }
}

/// Assert that a collection has a specific length
pub fn assert_len<T>(collection: &[T], expected_len: usize, message: &str) -> AssertionResult {
    if collection.len() == expected_len {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected length {}, got {}", 
            message, expected_len, collection.len()
        )))
    }
}

/// Assert that a collection contains a specific item
pub fn assert_contains<T>(collection: &[T], item: &T, message: &str) -> AssertionResult 
where
    T: PartialEq + std::fmt::Debug
{
    if collection.contains(item) {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - collection does not contain {:?}", 
            message, item
        )))
    }
}

/// Assert that a value is not None
pub fn assert_some<T>(value: Option<T>, message: &str) -> AssertionResult {
    if value.is_some() {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected Some, got None", 
            message
        )))
    }
}

/// Assert that a value is None
pub fn assert_none<T>(value: Option<T>, message: &str) -> AssertionResult {
    if value.is_none() {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected None, got Some", 
            message
        )))
    }
}

/// Assert that a result is Ok
pub fn assert_ok<T, E>(result: Result<T, E>, message: &str) -> AssertionResult 
where
    E: std::fmt::Debug,
    T: std::fmt::Debug
{
    if result.is_ok() {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected Ok, got Err({:?})", 
            message, result.unwrap_err()
        )))
    }
}

/// Assert that a result is Err
pub fn assert_err<T, E>(result: Result<T, E>, message: &str) -> AssertionResult 
where
    T: std::fmt::Debug,
    E: std::fmt::Debug
{
    if result.is_err() {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected Err, got Ok({:?})", 
            message, result.unwrap()
        )))
    }
}

/// Assert that a function panics
pub fn assert_shooks<F>(f: F, message: &str) -> AssertionResult
where
    F: FnOnce() + std::panic::UnwindSafe
{
    let result = std::panic::catch_unwind(f);
    if result.is_err() {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected panic but function completed normally", 
            message
        )))
    }
}

/// Assert that a function does not panic
pub fn assert_no_shook<F>(f: F, message: &str) -> AssertionResult
where
    F: FnOnce() + std::panic::UnwindSafe
{
    let result = std::panic::catch_unwind(f);
    if result.is_ok() {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected no panic but function panicked", 
            message
        )))
    }
}

// Re-export for compatibility
pub use assert_shooks as AssertShooks;
pub use assert_no_shook as AssertNoShook;

/// Assert that a function panics with a specific value
pub fn assert_shooks_with_value<F, T>(f: F, expected: T, message: &str) -> AssertionResult
where
    F: FnOnce() -> T + std::panic::UnwindSafe,
    T: PartialEq + std::fmt::Debug
{
    let result = std::panic::catch_unwind(f);
    if result.is_err() {
        Ok(())
    } else {
        Err(CursedError::runtime_error(&format!(
            "Assertion failed: {} - expected panic but function completed normally with value {:?}", 
            message, result.unwrap()
        )))
    }
}

pub use assert_shooks_with_value as AssertShooksWithValue;
