/// Test Assertions System
/// 
/// Provides comprehensive assertion functions for CURSED tests
/// with detailed error messages and support for different data types.

use super::{TestError, TestResult as TestingResult};
use super::framework::{AssertionResult as FrameworkAssertionResult, SourceLocation, TestContext};
use std::fmt::Debug;
use std::time::Instant;
use tracing::debug;

/// Main assertion interface
pub struct Assert;

/// Assertion error with detailed context
#[derive(Debug, Clone)]
pub struct AssertionError {
    /// Error message
    pub message: String,
    /// Expected value (if applicable)
    pub expected: Option<String>,
    /// Actual value (if applicable)
    pub actual: Option<String>,
    /// Source location
    pub location: SourceLocation,
    /// Additional context
    pub context: Option<String>,
}

/// Result type for assertions
pub type AssertionResult<T = ()> = Result<T, AssertionError>;

impl Assert {
    /// Assert that a condition is true
    pub fn assert_true(condition: bool, message: &str) -> AssertionResult {
        let start_time = Instant::now();
        
        if condition {
            debug!("Assertion passed: {}", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Expected true, but got false: {}", message),
                expected: Some("true".to_string()),
                actual: Some("false".to_string()),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a condition is false
    pub fn assert_false(condition: bool, message: &str) -> AssertionResult {
        if !condition {
            debug!("Assertion passed: {}", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Expected false, but got true: {}", message),
                expected: Some("false".to_string()),
                actual: Some("true".to_string()),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that two values are equal
    pub fn assert_equal<T: PartialEq + Debug>(expected: T, actual: T, message: &str) -> AssertionResult {
        if expected == actual {
            debug!("Assertion passed: {} - values are equal", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Values are not equal: {}", message),
                expected: Some(format!("{:?}", expected)),
                actual: Some(format!("{:?}", actual)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that two values are not equal
    pub fn assert_not_equal<T: PartialEq + Debug>(unexpected: T, actual: T, message: &str) -> AssertionResult {
        if unexpected != actual {
            debug!("Assertion passed: {} - values are not equal", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Values are equal when they should not be: {}", message),
                expected: Some(format!("not {:?}", unexpected)),
                actual: Some(format!("{:?}", actual)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a value is null/none
    pub fn assert_null<T>(value: Option<T>, message: &str) -> AssertionResult {
        if value.is_none() {
            debug!("Assertion passed: {} - value is null", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Expected null, but got a value: {}", message),
                expected: Some("null".to_string()),
                actual: Some("some value".to_string()),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a value is not null/none
    pub fn assert_not_null<T>(value: Option<T>, message: &str) -> AssertionResult {
        if value.is_some() {
            debug!("Assertion passed: {} - value is not null", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Expected a value, but got null: {}", message),
                expected: Some("some value".to_string()),
                actual: Some("null".to_string()),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a string contains a substring
    pub fn assert_contains(haystack: &str, needle: &str, message: &str) -> AssertionResult {
        if haystack.contains(needle) {
            debug!("Assertion passed: {} - string contains substring", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("String does not contain expected substring: {}", message),
                expected: Some(format!("string containing '{}'", needle)),
                actual: Some(format!("'{}'", haystack)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a string does not contain a substring
    pub fn assert_not_contains(haystack: &str, needle: &str, message: &str) -> AssertionResult {
        if !haystack.contains(needle) {
            debug!("Assertion passed: {} - string does not contain substring", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("String contains unexpected substring: {}", message),
                expected: Some(format!("string not containing '{}'", needle)),
                actual: Some(format!("'{}'", haystack)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a string starts with a prefix
    pub fn assert_starts_with(text: &str, prefix: &str, message: &str) -> AssertionResult {
        if text.starts_with(prefix) {
            debug!("Assertion passed: {} - string starts with prefix", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("String does not start with expected prefix: {}", message),
                expected: Some(format!("string starting with '{}'", prefix)),
                actual: Some(format!("'{}'", text)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a string ends with a suffix
    pub fn assert_ends_with(text: &str, suffix: &str, message: &str) -> AssertionResult {
        if text.ends_with(suffix) {
            debug!("Assertion passed: {} - string ends with suffix", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("String does not end with expected suffix: {}", message),
                expected: Some(format!("string ending with '{}'", suffix)),
                actual: Some(format!("'{}'", text)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a collection is empty
    pub fn assert_empty<T>(collection: &[T], message: &str) -> AssertionResult {
        if collection.is_empty() {
            debug!("Assertion passed: {} - collection is empty", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Expected empty collection, but got {} items: {}", collection.len(), message),
                expected: Some("empty collection".to_string()),
                actual: Some(format!("collection with {} items", collection.len())),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a collection is not empty
    pub fn assert_not_empty<T>(collection: &[T], message: &str) -> AssertionResult {
        if !collection.is_empty() {
            debug!("Assertion passed: {} - collection is not empty", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Expected non-empty collection: {}", message),
                expected: Some("non-empty collection".to_string()),
                actual: Some("empty collection".to_string()),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a collection has a specific length
    pub fn assert_length<T>(collection: &[T], expected_length: usize, message: &str) -> AssertionResult {
        let actual_length = collection.len();
        if actual_length == expected_length {
            debug!("Assertion passed: {} - collection has expected length", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Collection length mismatch: {}", message),
                expected: Some(format!("length {}", expected_length)),
                actual: Some(format!("length {}", actual_length)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that two floating point numbers are approximately equal
    pub fn assert_float_equal(expected: f64, actual: f64, epsilon: f64, message: &str) -> AssertionResult {
        let diff = (expected - actual).abs();
        if diff <= epsilon {
            debug!("Assertion passed: {} - floats are approximately equal", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Floating point numbers are not approximately equal: {}", message),
                expected: Some(format!("{} (±{})", expected, epsilon)),
                actual: Some(format!("{} (diff: {})", actual, diff)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: Some(format!("Difference {} exceeds epsilon {}", diff, epsilon)),
            })
        }
    }

    /// Assert that a value is within a range
    pub fn assert_in_range<T: PartialOrd + Debug>(value: T, min: T, max: T, message: &str) -> AssertionResult {
        if value >= min && value <= max {
            debug!("Assertion passed: {} - value is in range", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Value is not in expected range: {}", message),
                expected: Some(format!("value between {:?} and {:?}", min, max)),
                actual: Some(format!("{:?}", value)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a collection contains a specific item
    pub fn assert_contains_item<T: PartialEq + Debug>(collection: &[T], item: &T, message: &str) -> AssertionResult {
        if collection.contains(item) {
            debug!("Assertion passed: {} - collection contains item", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Collection does not contain expected item: {}", message),
                expected: Some(format!("collection containing {:?}", item)),
                actual: Some(format!("collection: {:?}", collection)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Assert that a result is Ok
    pub fn assert_ok<T, E: Debug>(result: Result<T, E>, message: &str) -> AssertionResult<T> {
        match result {
            Ok(value) => {
                debug!("Assertion passed: {} - result is Ok", message);
                Ok(value)
            }
            Err(error) => {
                Err(AssertionError {
                    message: format!("Expected Ok result, but got Err: {}", message),
                    expected: Some("Ok(_)".to_string()),
                    actual: Some(format!("Err({:?})", error)),
                    location: SourceLocation {
                        file: "unknown".to_string(),
                        line: 0,
                        column: 0,
                    },
                    context: None,
                })
            }
        }
    }

    /// Assert that a result is Err
    pub fn assert_err<T: Debug, E>(result: Result<T, E>, message: &str) -> AssertionResult<E> {
        match result {
            Err(error) => {
                debug!("Assertion passed: {} - result is Err", message);
                Ok(error)
            }
            Ok(value) => {
                Err(AssertionError {
                    message: format!("Expected Err result, but got Ok: {}", message),
                    expected: Some("Err(_)".to_string()),
                    actual: Some(format!("Ok({:?})", value)),
                    location: SourceLocation {
                        file: "unknown".to_string(),
                        line: 0,
                        column: 0,
                    },
                    context: None,
                })
            }
        }
    }

    /// Assert that code panics
    pub fn assert_panics<F: FnOnce() + std::panic::UnwindSafe>(f: F, message: &str) -> AssertionResult {
        let result = std::panic::catch_unwind(f);
        if result.is_err() {
            debug!("Assertion passed: {} - code panicked as expected", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Expected code to panic, but it completed normally: {}", message),
                expected: Some("panic".to_string()),
                actual: Some("normal completion".to_string()),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }

    /// Custom assertion with user-defined predicate
    pub fn assert_that<T, F>(value: T, predicate: F, message: &str) -> AssertionResult 
    where
        F: FnOnce(&T) -> bool,
        T: Debug,
    {
        if predicate(&value) {
            debug!("Assertion passed: {} - custom predicate satisfied", message);
            Ok(())
        } else {
            Err(AssertionError {
                message: format!("Custom assertion failed: {}", message),
                expected: Some("predicate to return true".to_string()),
                actual: Some(format!("predicate returned false for value: {:?}", value)),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: None,
            })
        }
    }
}

/// Macro helper for creating assertions with file/line information
#[macro_export]
macro_rules! assert_eq_loc {
    ($expected:expr, $actual:expr, $message:expr) => {
        {
            use $crate::testing::assertions::Assert;
            let result = Assert::assert_equal($expected, $actual, $message);
            if let Err(mut error) = result {
                error.location.file = file!().to_string();
                error.location.line = line!() as usize;
                error.location.column = column!() as usize;
                Err(error)
            } else {
                Ok(())
            }
        }
    };
}

/// Macro helper for creating true assertions with file/line information
#[macro_export]
macro_rules! assert_true_loc {
    ($condition:expr, $message:expr) => {
        {
            use $crate::testing::assertions::Assert;
            let result = Assert::assert_true($condition, $message);
            if let Err(mut error) = result {
                error.location.file = file!().to_string();
                error.location.line = line!() as usize;
                error.location.column = column!() as usize;
                Err(error)
            } else {
                Ok(())
            }
        }
    };
}

/// Assertion builder for fluent interface
pub struct AssertionBuilder<T> {
    value: T,
    context: Option<String>,
}

impl<T> AssertionBuilder<T> {
    /// Create new assertion builder
    pub fn new(value: T) -> Self {
        Self {
            value,
            context: None,
        }
    }

    /// Add context to the assertion
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    /// Get the wrapped value
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T: PartialEq + Debug> AssertionBuilder<T> {
    /// Assert equals using builder pattern
    pub fn equals(self, expected: T) -> AssertionResult {
        let message = self.context.as_deref().unwrap_or("assertion failed");
        Assert::assert_equal(expected, self.value, message)
    }

    /// Assert not equals using builder pattern
    pub fn not_equals(self, unexpected: T) -> AssertionResult {
        let message = self.context.as_deref().unwrap_or("assertion failed");
        Assert::assert_not_equal(unexpected, self.value, message)
    }
}

impl<T> AssertionBuilder<Option<T>> {
    /// Assert is some using builder pattern
    pub fn is_some(self) -> AssertionResult<T> {
        let message = self.context.as_deref().unwrap_or("assertion failed");
        match self.value {
            Some(value) => Ok(value),
            None => Err(AssertionError {
                message: format!("Expected Some, but got None: {}", message),
                expected: Some("Some(_)".to_string()),
                actual: Some("None".to_string()),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 0,
                    column: 0,
                },
                context: self.context,
            })
        }
    }

    /// Assert is none using builder pattern
    pub fn is_none(self) -> AssertionResult {
        let message = self.context.as_deref().unwrap_or("assertion failed");
        Assert::assert_null(self.value, message)
    }
}

/// Create assertion builder
pub fn assert_that<T>(value: T) -> AssertionBuilder<T> {
    AssertionBuilder::new(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_true() {
        assert!(Assert::assert_true(true, "should pass").is_ok());
        assert!(Assert::assert_true(false, "should fail").is_err());
    }

    #[test]
    fn test_assert_false() {
        assert!(Assert::assert_false(false, "should pass").is_ok());
        assert!(Assert::assert_false(true, "should fail").is_err());
    }

    #[test]
    fn test_assert_equal() {
        assert!(Assert::assert_equal(5, 5, "should pass").is_ok());
        assert!(Assert::assert_equal(5, 3, "should fail").is_err());
    }

    #[test]
    fn test_assert_contains() {
        assert!(Assert::assert_contains("hello world", "world", "should pass").is_ok());
        assert!(Assert::assert_contains("hello", "world", "should fail").is_err());
    }

    #[test]
    fn test_assert_empty() {
        let empty: Vec<i32> = vec![];
        let non_empty = vec![1, 2, 3];
        
        assert!(Assert::assert_empty(&empty, "should pass").is_ok());
        assert!(Assert::assert_empty(&non_empty, "should fail").is_err());
    }

    #[test]
    fn test_assert_float_equal() {
        assert!(Assert::assert_float_equal(1.0, 1.0001, 0.001, "should pass").is_ok());
        assert!(Assert::assert_float_equal(1.0, 1.1, 0.01, "should fail").is_err());
    }

    #[test]
    fn test_assertion_builder() {
        assert!(assert_that(5).equals(5).is_ok());
        assert!(assert_that(5).not_equals(3).is_ok());
        assert!(assert_that(Some(10)).is_some().is_ok());
        assert!(assert_that(None::<i32>).is_none().is_ok());
    }

    #[test]
    fn test_assert_panics() {
        assert!(Assert::assert_panics(|| panic!("test panic"), "should pass").is_ok());
        assert!(Assert::assert_panics(|| { /* no panic */ }, "should fail").is_err());
    }
}
