//! Testing functionality for assertions

use crate::error::CursedError;
use std::fmt::{Debug, Display};
use crate::stdlib::packages::CryptoError;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Standard assertion functions for testing
pub struct Assertions;

impl Assertions {
    /// Assert that two values are equal
    pub fn assert_eq<T: PartialEq + Debug>(left: T, right: T) -> TestResult<()> {
        if left == right {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: left == right\n  left: {:?}\n right: {:?}", left, right
            )))
        }
    }

    /// Assert that two values are not equal
    pub fn assert_ne<T: PartialEq + Debug>(left: T, right: T) -> TestResult<()> {
        if left != right {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: left != right\n  left: {:?}\n right: {:?}", left, right
            )))
        }
    }

    /// Assert that a condition is true
    pub fn assert(condition: bool) -> TestResult<()> {
        if condition {
            Ok(())
        } else {
            Err(CursedError::runtime_error("Assertion failed: condition is false"))
        }
    }

    /// Assert that a condition is true with custom message
    pub fn assert_with_msg(condition: bool, message: &str) -> TestResult<()> {
        if condition {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Assertion failed: {}", "placeholder")))
        }
    }

    /// Assert that a value is Some
    pub fn assert_some<T: Debug>(option: Option<T>) -> TestResult<T> {
        match option {
            Some(value) => Ok(value),
            None => Err(CursedError::runtime_error("Assertion failed: expected Some, got None")),
        }
    }

    /// Assert that a value is None
    pub fn assert_none<T: Debug>(option: Option<T>) -> TestResult<()> {
        match option {
            None => Ok(()),
            Some(value) => Err(CryptoError::Other(format!(
                "Assertion failed: expected None, got Some({:?})", value
            )).into()),
        }
    }

    /// Assert that a Result is Ok
    pub fn assert_ok<T: Debug, E: Debug>(result: Result<T, E>) -> TestResult<T> {
        match result {
            Ok(value) => Ok(value),
            Err(error) => Err(CryptoError::Other(format!(
                "Assertion failed: expected Ok, got Err({:?})", error
            )).into()),
        }
    }

    /// Assert that a Result is Err
    pub fn assert_err<T: Debug, E: Debug>(result: Result<T, E>) -> TestResult<E> {
        match result {
            Err(error) => Ok(error),
            Ok(value) => Err(CryptoError::Other(format!(
                "Assertion failed: expected Err, got Ok({:?})", value
            )).into()),
        }
    }

    /// Assert that a string contains a substring
    pub fn assert_contains(haystack: &str, needle: &str) -> TestResult<()> {
        if haystack.contains(needle) {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: '{}' does not contain '{}'", haystack, needle
            )))
        }
    }

    /// Assert that a string starts with a prefix
    pub fn assert_starts_with(string: &str, prefix: &str) -> TestResult<()> {
        if string.starts_with(prefix) {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: '{}' does not start with '{}'", string, prefix
            )))
        }
    }

    /// Assert that a string ends with a suffix
    pub fn assert_ends_with(string: &str, suffix: &str) -> TestResult<()> {
        if string.ends_with(suffix) {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: '{}' does not end with '{}'", string, suffix
            )))
        }
    }

    /// Assert that a value is greater than another
    pub fn assert_gt<T: PartialOrd + Debug>(left: T, right: T) -> TestResult<()> {
        if left > right {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: {:?} > {:?}", left, right
            )))
        }
    }

    /// Assert that a value is less than another
    pub fn assert_lt<T: PartialOrd + Debug>(left: T, right: T) -> TestResult<()> {
        if left < right {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: {:?} < {:?}", left, right
            )))
        }
    }

    /// Assert that a value is greater than or equal to another
    pub fn assert_ge<T: PartialOrd + Debug>(left: T, right: T) -> TestResult<()> {
        if left >= right {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: {:?} >= {:?}", left, right
            )))
        }
    }

    /// Assert that a value is less than or equal to another
    pub fn assert_le<T: PartialOrd + Debug>(left: T, right: T) -> TestResult<()> {
        if left <= right {
            Ok(())
        } else {
            Err(CursedError::runtime_error(&&format!(
                "Assertion failed: {:?} <= {:?}", left, right
            )))
        }
    }

    /// Assert that a collection is empty
    pub fn assert_empty<T>(collection: &[T]) -> TestResult<()> {
        if collection.is_empty() {
            Ok(())
        } else {
            Err(CryptoError::Other(format!(
                "Assertion failed: collection is not empty (length: {})", collection.len()
            )).into())
        }
    }

    /// Assert that a collection has a specific length
    pub fn assert_len<T>(collection: &[T], expected_len: usize) -> TestResult<()> {
        if collection.len() == expected_len {
            Ok(())
        } else {
            Err(CryptoError::Other(format!(
                "Assertion failed: expected length {}, got {}", expected_len, collection.len()
            )).into())
        }
    }
}

/// Convenience functions for common assertions
pub fn assert_eq<T: PartialEq + Debug>(left: T, right: T) -> TestResult<()> {
    Assertions::assert_eq(left, right)
}

pub fn assert_ne<T: PartialEq + Debug>(left: T, right: T) -> TestResult<()> {
    Assertions::assert_ne(left, right)
}

pub fn assert(condition: bool) -> TestResult<()> {
    Assertions::assert(condition)
}

pub fn assert_with_msg(condition: bool, message: &str) -> TestResult<()> {
    Assertions::assert_with_msg(condition, message)
}

/// Initialize assertion processing
pub fn init_assertions() -> TestResult<()> {
    // Test basic assertions
    assert_eq(2 + 2, 4)?;
    assert_ne(1, 2)?;
    assert(true)?;
    assert_with_msg(2 > 1, "two should be greater than one")?;
    
    println!("🧪 Test assertions initialized");
    Ok(())
}

/// Test assertion functionality
pub fn test_assertions() -> TestResult<()> {
    // Test various assertion types
    assert_eq("hello", "hello")?;
    assert_ne("hello", "world")?;
    assert(5 > 3)?;
    
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    Assertions::assert_some(some_value)?;
    Assertions::assert_none(none_value)?;
    
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");
    Assertions::assert_ok(ok_result)?;
    Assertions::assert_err(err_result)?;
    
    Assertions::assert_contains("hello world", "world")?;
    Assertions::assert_starts_with("hello world", "hello")?;
    Assertions::assert_ends_with("hello world", "world")?;
    
    Assertions::assert_gt(5, 3)?;
    Assertions::assert_lt(3, 5)?;
    Assertions::assert_ge(5, 5)?;
    Assertions::assert_le(3, 3)?;
    
    let empty_vec: Vec<i32> = vec![];
    let vec_with_items = vec![1, 2, 3];
    Assertions::assert_empty(&empty_vec)?;
    Assertions::assert_len(&vec_with_items, 3)?;
    
    Ok(())
}
