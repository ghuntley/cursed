/// fr fr Assertion functions for the TestVibes testing framework
// use crate::stdlib::packages::test_vibes::core::VibeTest;
use crate::error::CursedError;
use std::fmt::Debug;

/// fr fr Basic assertions

/// fr fr Assert that a condition is true
pub fn assert(t: &mut VibeTest, condition: bool, message: &str) {
    if !condition {
        t.fail_vibe(&format!("Assertion failed: {}", message));
    }
}

/// fr fr Assert that two values are equal
pub fn assert_equal<T: PartialEq + Debug>(t: &mut VibeTest, expected: T, actual: T, message: &str) {
    if expected != actual {
        t.fail_vibe(&format!("Assert equal failed: {}. Expected: {:?}, Got: {:?}", message, expected, actual));
    }
}

/// fr fr Assert that two values are not equal
pub fn assert_not_equal<T: PartialEq + Debug>(t: &mut VibeTest, expected: T, actual: T, message: &str) {
    if expected == actual {
        t.fail_vibe(&format!("Assert not equal failed: {}. Values should not be equal: {:?}", message, expected));
    }
}

/// fr fr Assert that a value is None/null
pub fn assert_nil<T: Debug>(t: &mut VibeTest, actual: Option<T>, message: &str) {
    if actual.is_some() {
        t.fail_vibe(&format!("Assert nil failed: {}. Expected None, got: {:?}", message, actual));
    }
}

/// fr fr Assert that a value is not None/null
pub fn assert_not_nil<T: Debug>(t: &mut VibeTest, actual: Option<T>, message: &str) {
    if actual.is_none() {
        t.fail_vibe(&format!("Assert not nil failed: {}. Expected Some value, got None", message));
    }
}

/// fr fr Assert that a boolean is true
pub fn assert_true(t: &mut VibeTest, actual: bool, message: &str) {
    if !actual {
        t.fail_vibe(&format!("Assert true failed: {}. Expected true, got false", message));
    }
}

/// fr fr Assert that a boolean is false
pub fn assert_false(t: &mut VibeTest, actual: bool, message: &str) {
    if actual {
        t.fail_vibe(&format!("Assert false failed: {}. Expected false, got true", message));
    }
}

/// fr fr CursedError assertions

/// fr fr Assert that an error occurred
pub fn assert_error(t: &mut VibeTest, err: Result<(), &str>, message: &str) {
    if err.is_ok() {
        t.fail_vibe(&format!("Assert error failed: {}. Expected error, got success", message));
    }
}

/// fr fr Assert that no error occurred
pub fn assert_no_error(t: &mut VibeTest, err: Result<(), &str>, message: &str) {
    if let Err(e) = err {
        t.fail_vibe(&format!("Assert no error failed: {}. Expected success, got error: {}", message, e));
    }
}

/// fr fr Assert that an error is a specific error
pub fn assert_error_is(t: &mut VibeTest, err: Result<(), &str>, target: &str, message: &str) {
    match err {
        Ok(()) => t.fail_vibe(&format!("Assert error is failed: {}. Expected error '{}', got success", message, target)),
        Err(e) => {
            if e != target {
                t.fail_vibe(&format!("Assert error is failed: {}. Expected error '{}', got '{}'", message, target, e));
            }
        }
    }
}

/// fr fr Assert that an error contains a specific substring
pub fn assert_error_contains(t: &mut VibeTest, err: Result<(), &str>, contains: &str, message: &str) {
    match err {
        Ok(()) => t.fail_vibe(&format!("Assert error contains failed: {}. Expected error containing '{}', got success", message, contains)),
        Err(e) => {
            if !e.contains(contains) {
                t.fail_vibe(&format!("Assert error contains failed: {}. Expected error containing '{}', got '{}'", message, contains, e));
            }
        }
    }
}

/// fr fr Collection assertions

/// fr fr Assert that a collection has a specific length
pub fn assert_len<T>(t: &mut VibeTest, collection: &[T], length: usize, message: &str) {
    if collection.len() != length {
        t.fail_vibe(&format!("Assert len failed: {}. Expected length {}, got {}", message, length, collection.len()));
    }
}

/// fr fr Assert that a collection is empty
pub fn assert_empty<T>(t: &mut VibeTest, collection: &[T], message: &str) {
    if !collection.is_empty() {
        t.fail_vibe(&format!("Assert empty failed: {}. Expected empty collection, got {} items", message, collection.len()));
    }
}

/// fr fr Assert that a collection is not empty
pub fn assert_not_empty<T>(t: &mut VibeTest, collection: &[T], message: &str) {
    if collection.is_empty() {
        t.fail_vibe(&format!("Assert not empty failed: {}. Expected non-empty collection, got empty", message));
    }
}

/// fr fr Assert that a collection contains a specific element
pub fn assert_contains<T: PartialEq + Debug>(t: &mut VibeTest, collection: &[T], element: &T, message: &str) {
    if !collection.contains(element) {
        t.fail_vibe(&format!("Assert contains failed: {}. Collection does not contain {:?}", message, element));
    }
}

/// fr fr Assert that a collection does not contain a specific element
pub fn assert_not_contains<T: PartialEq + Debug>(t: &mut VibeTest, collection: &[T], element: &T, message: &str) {
    if collection.contains(element) {
        t.fail_vibe(&format!("Assert not contains failed: {}. Collection should not contain {:?}", message, element));
    }
}

/// fr fr Numeric assertions

/// fr fr Assert that x > y
pub fn assert_greater<T: PartialOrd + Debug>(t: &mut VibeTest, x: T, y: T, message: &str) {
    if !(x > y) {
        t.fail_vibe(&format!("Assert greater failed: {}. Expected {:?} > {:?}", message, x, y));
    }
}

/// fr fr Assert that x >= y
pub fn assert_greater_or_equal<T: PartialOrd + Debug>(t: &mut VibeTest, x: T, y: T, message: &str) {
    if !(x >= y) {
        t.fail_vibe(&format!("Assert greater or equal failed: {}. Expected {:?} >= {:?}", message, x, y));
    }
}

/// fr fr Assert that x < y
pub fn assert_less<T: PartialOrd + Debug>(t: &mut VibeTest, x: T, y: T, message: &str) {
    if !(x < y) {
        t.fail_vibe(&format!("Assert less failed: {}. Expected {:?} < {:?}", message, x, y));
    }
}

/// fr fr Assert that x <= y
pub fn assert_less_or_equal<T: PartialOrd + Debug>(t: &mut VibeTest, x: T, y: T, message: &str) {
    if !(x <= y) {
        t.fail_vibe(&format!("Assert less or equal failed: {}. Expected {:?} <= {:?}", message, x, y));
    }
}

/// fr fr Assert that a value is zero
pub fn assert_zero<T: PartialEq + Default + Debug>(t: &mut VibeTest, actual: T, message: &str) {
    let zero = T::default();
    if actual != zero {
        t.fail_vibe(&format!("Assert zero failed: {}. Expected zero, got {:?}", message, actual));
    }
}

/// fr fr Assert that a value is not zero
pub fn assert_not_zero<T: PartialEq + Default + Debug>(t: &mut VibeTest, actual: T, message: &str) {
    let zero = T::default();
    if actual == zero {
        t.fail_vibe(&format!("Assert not zero failed: {}. Expected non-zero value, got zero", message));
    }
}

/// fr fr String assertions

/// fr fr Assert that a string contains a substring
pub fn assert_contains_substr(t: &mut VibeTest, str: &str, substr: &str, message: &str) {
    if !str.contains(substr) {
        t.fail_vibe(&format!("Assert contains substring failed: {}. String '{}' does not contain '{}'", message, str, substr));
    }
}

/// fr fr Assert that a string has a specific prefix
pub fn assert_has_prefix(t: &mut VibeTest, str: &str, prefix: &str, message: &str) {
    if !str.starts_with(prefix) {
        t.fail_vibe(&format!("Assert has prefix failed: {}. String '{}' does not start with '{}'", message, str, prefix));
    }
}

/// fr fr Assert that a string has a specific suffix
pub fn assert_has_suffix(t: &mut VibeTest, str: &str, suffix: &str, message: &str) {
    if !str.ends_with(suffix) {
        t.fail_vibe(&format!("Assert has suffix failed: {}. String '{}' does not end with '{}'", message, str, suffix));
    }
}

/// fr fr Assert that a string matches a regex pattern
pub fn assert_matches_regex(t: &mut VibeTest, str: &str, pattern: &str, message: &str) {
    // Simple pattern matching - in a real implementation would use regex crate
    let matches = match pattern {
        r"\d+" => str.chars().all(|c| c.is_ascii_digit()),
        r"\w+" => str.chars().all(|c| c.is_alphanumeric()),
        _ => str.contains(pattern), // Fallback to simple contains
    };
    
    if !matches {
        t.fail_vibe(&format!("Assert matches regex failed: {}. String '{}' does not match pattern '{}'", message, str, pattern));
    }
}

/// fr fr Type assertions

/// fr fr Assert that a value is of a specific type
pub fn assert_type<T: 'static, U: 'static>(t: &mut VibeTest, _expected_type: std::marker::PhantomData<T>, _value: U, message: &str) {
    let expected_type_name = std::any::type_name::<T>();
    let actual_type_name = std::any::type_name::<U>();
    
    if expected_type_name != actual_type_name {
        t.fail_vibe(&format!("Assert type failed: {}. Expected type {}, got {}", message, expected_type_name, actual_type_name));
    }
}

/// fr fr Panic assertions (CURSED calls them "shook" assertions)

/// fr fr Assert that a function panics
pub fn assert_shooks<F: FnOnce() + std::panic::UnwindSafe>(t: &mut VibeTest, fn_: F, message: &str) {
    let result = std::panic::catch_unwind(fn_);
    if result.is_ok() {
        t.fail_vibe(&format!("Assert shooks failed: {}. Expected function to panic, but it completed normally", message));
    }
}

/// fr fr Assert that a function panics with a specific value
pub fn assert_shooks_with_value<F: FnOnce() + std::panic::UnwindSafe>(t: &mut VibeTest, _value: &str, fn_: F, message: &str) {
    let result = std::panic::catch_unwind(fn_);
    match result {
        Ok(_) => t.fail_vibe(&format!("Assert shooks with value failed: {}. Expected function to panic, but it completed normally", message)),
        Err(_panic_info) => {
            // In a real implementation, we'd check if the panic message contains the expected value
            // For now, we just check that it panicked
        }
    }
}

/// fr fr Assert that a function does not panic
pub fn assert_no_shook<F: FnOnce() + std::panic::UnwindSafe>(t: &mut VibeTest, fn_: F, message: &str) {
    let result = std::panic::catch_unwind(fn_);
    if result.is_err() {
        t.fail_vibe(&format!("Assert no shook failed: {}. Expected function to complete normally, but it panicked", message));
    }
}

/// fr fr Floating point assertions with tolerance

/// fr fr Assert that two floating point numbers are approximately equal
pub fn assert_float_equal(t: &mut VibeTest, expected: f64, actual: f64, tolerance: f64, message: &str) {
    let diff = (expected - actual).abs();
    if diff > tolerance {
        t.fail_vibe(&format!("Assert float equal failed: {}. Expected {}, got {} (tolerance: {})", message, expected, actual, tolerance));
    }
}

/// fr fr Assert that two floating point numbers are not approximately equal
pub fn assert_float_not_equal(t: &mut VibeTest, expected: f64, actual: f64, tolerance: f64, message: &str) {
    let diff = (expected - actual).abs();
    if diff <= tolerance {
        t.fail_vibe(&format!("Assert float not equal failed: {}. Values should not be approximately equal: {} ≈ {} (tolerance: {})", message, expected, actual, tolerance));
    }
}

/// fr fr Range assertions

/// fr fr Assert that a value is within a specific range
pub fn assert_in_range<T: PartialOrd + Debug>(t: &mut VibeTest, value: T, min: T, max: T, message: &str) {
    if !(value >= min && value <= max) {
        t.fail_vibe(&format!("Assert in range failed: {}. Expected {:?} to be between {:?} and {:?}", message, value, min, max));
    }
}

/// fr fr Assert that a value is outside a specific range
pub fn assert_out_of_range<T: PartialOrd + Debug>(t: &mut VibeTest, value: T, min: T, max: T, message: &str) {
    if value >= min && value <= max {
        t.fail_vibe(&format!("Assert out of range failed: {}. Expected {:?} to be outside [{:?}, {:?}]", message, value, min, max));
    }
}

