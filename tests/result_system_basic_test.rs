//! Basic test suite for CURSED Result and Option system
//!
//! This test suite validates the core functionality of the Result/Option types
//! without complex integrations to avoid compilation issues.

use cursed::types::result::{Result, Option, result_utils, error_patterns};
use cursed::error::CursedError;

/// Test Result<T, E> basic functionality
#[test]
fn test_result_basic_operations() {
    // Test Ok variant
    let ok_result: Result<i32, &str> = Result::Ok(42);
    assert!(ok_result.is_ok());
    assert!(!ok_result.is_err());
    assert_eq!(ok_result.unwrap(), 42);

    // Test Err variant
    let err_result: Result<i32, &str> = Result::Err("error");
    assert!(!err_result.is_ok());
    assert!(err_result.is_err());
    assert_eq!(err_result.unwrap_or(0), 0);

    // Test map operation
    let mapped = Result::Ok(21).map(|x| x * 2);
    assert_eq!(mapped.unwrap(), 42);

    let mapped_err = Result::Err::<i32, &str>("error").map(|x| x * 2);
    assert!(mapped_err.is_err());

    // Test and_then operation
    let chained = Result::Ok(21).and_then(|x| Result::Ok(x * 2));
    assert_eq!(chained.unwrap(), 42);

    let chained_err = Result::Err::<i32, &str>("error").and_then(|x| Result::Ok(x * 2));
    assert!(chained_err.is_err());

    // Test map_err operation
    let mapped_err = Result::Err::<i32, &str>("error").map_err(|e| e.to_uppercase());
    assert_eq!(mapped_err.unwrap_err(), "ERROR");
}

/// Test Option<T> basic functionality
#[test]
fn test_option_basic_operations() {
    // Test Some variant
    let some_option: Option<i32> = Option::Some(42);
    assert!(some_option.is_some());
    assert!(!some_option.is_none());
    assert_eq!(some_option.unwrap(), 42);

    // Test None variant
    let none_option: Option<i32> = Option::None;
    assert!(!none_option.is_some());
    assert!(none_option.is_none());
    assert_eq!(none_option.unwrap_or(0), 0);

    // Test map operation
    let mapped = Option::Some(21).map(|x| x * 2);
    assert_eq!(mapped.unwrap(), 42);

    let mapped_none = Option::None::<i32>().map(|x| x * 2);
    assert!(mapped_none.is_none());

    // Test and_then operation
    let chained = Option::Some(21).and_then(|x| Option::Some(x * 2));
    assert_eq!(chained.unwrap(), 42);

    let chained_none = Option::None::<i32>().and_then(|x| Option::Some(x * 2));
    assert!(chained_none.is_none());

    // Test filter operation
    let filtered = Option::Some(42).filter(|&x| x > 30);
    assert_eq!(filtered.unwrap(), 42);

    let filtered_out = Option::Some(10).filter(|&x| x > 30);
    assert!(filtered_out.is_none());
}

/// Test Result/Option conversion
#[test]
fn test_result_option_conversion() {
    // Option to Result
    let some_option = Option::Some(42);
    let result = some_option.ok_or("error");
    assert_eq!(result.unwrap(), 42);

    let none_option: Option<i32> = Option::None;
    let result = none_option.ok_or("error");
    assert_eq!(result.unwrap_err(), "error");

    // Result to Option
    let ok_result = Result::Ok(42);
    let option = ok_result.ok();
    assert_eq!(option.unwrap(), 42);

    let err_result = Result::Err::<i32, &str>("error");
    let option = err_result.ok();
    assert!(option.is_none());

    // Result err to Option
    let err_result = Result::Err::<i32, &str>("error");
    let option = err_result.err();
    assert_eq!(option.unwrap(), "error");

    let ok_result = Result::Ok::<i32, &str>(42);
    let option = ok_result.err();
    assert!(option.is_none());
}

/// Test utility functions
#[test]
fn test_utility_functions() {
    // Test utility constructors
    let ok_val = result_utils::ok::<i32, &str>(42);
    assert_eq!(ok_val.unwrap(), 42);

    let err_val = result_utils::err::<i32, &str>("error");
    assert!(err_val.is_err());

    let some_val = result_utils::some(42);
    assert_eq!(some_val.unwrap(), 42);

    let none_val = result_utils::none::<i32>();
    assert!(none_val.is_none());

    // Test bool_to_option
    let condition_some = result_utils::bool_to_option(true, 42);
    assert_eq!(condition_some.unwrap(), 42);

    let condition_none = result_utils::bool_to_option(false, 42);
    assert!(condition_none.is_none());

    // Test transpose
    let result_some = Result::Ok(Option::Some(42));
    let transposed = result_utils::transpose(result_some);
    assert_eq!(transposed.unwrap().unwrap(), 42);

    let result_none = Result::Ok(Option::None::<i32>());
    let transposed = result_utils::transpose(result_none);
    assert!(transposed.is_none());

    let result_err = Result::Err::<Option<i32>, &str>("error");
    let transposed = result_utils::transpose(result_err);
    assert!(transposed.unwrap().is_err());
}

/// Test error patterns
#[test]
fn test_error_patterns() {
    let parse_err = error_patterns::parse_error::<i32>("syntax error", 10, 5);
    assert!(parse_err.is_err());
    match parse_err {
        Result::Err(CursedError::ParseError { message, line, column }) => {
            assert_eq!(message, "syntax error");
            assert_eq!(line, Some(10));
            assert_eq!(column, Some(5));
        }
        _ => panic!("Expected ParseError"),
    }

    let runtime_err = error_patterns::runtime_error::<i32>("division by zero");
    assert!(runtime_err.is_err());
    match runtime_err {
        Result::Err(CursedError::Runtime(msg)) => {
            assert_eq!(msg, "division by zero");
        }
        _ => panic!("Expected Runtime error"),
    }

    let type_err = error_patterns::type_error::<i32>("type mismatch");
    assert!(type_err.is_err());
    match type_err {
        Result::Err(CursedError::Type(msg)) => {
            assert_eq!(msg, "type mismatch");
        }
        _ => panic!("Expected Type error"),
    }

    let compilation_err = error_patterns::compilation_error::<i32>("syntax error");
    assert!(compilation_err.is_err());
    match compilation_err {
        Result::Err(CursedError::Compile(msg)) => {
            assert_eq!(msg, "syntax error");
        }
        _ => panic!("Expected Compile error"),
    }
}

/// Test Result type integration with Value system
#[test]
fn test_result_value_integration() {
    // Test safe_divide function
    let division_ok = safe_divide(10.0, 2.0);
    assert!(division_ok.is_ok());
    assert_eq!(division_ok.unwrap(), 5.0);

    let division_err = safe_divide(10.0, 0.0);
    assert!(division_err.is_err());
}

/// Test complex error scenarios
#[test]
fn test_complex_error_scenarios() {
    // Test nested error handling
    let result = complex_operation();
    match result {
        Result::Ok(value) => assert_eq!(value, 42),
        Result::Err(error) => {
            // Should not reach here in this test
            panic!("Unexpected error: {:?}", error);
        }
    }

    // Test error propagation chain
    let chain_result = error_chain_operation();
    assert!(chain_result.is_err());
}

/// Test error conversion between systems
#[test]
fn test_error_conversion() {
    // Test CursedError to Result conversion
    let cursed_error = CursedError::Runtime("Test error".to_string());
    let result_from_error: Result<(), CursedError> = Result::from(cursed_error);
    assert!(result_from_error.is_err());

    // Test Option to Result conversion  
    let some_option = Option::Some(42);
    let result_from_option: Result<i32, CursedError> = Result::from(some_option);
    assert_eq!(result_from_option.unwrap(), 42);

    let none_option: Option<i32> = Option::None;
    let result_from_none: Result<i32, CursedError> = Result::from(none_option);
    assert!(result_from_none.is_err());
}

/// Test memory management and performance
#[test]
fn test_result_performance() {
    // Test large number of Result operations
    let mut results = Vec::new();
    
    for i in 0..1000 {
        if i % 2 == 0 {
            results.push(Result::Ok(i));
        } else {
            results.push(Result::Err(format!("Error {}", i)));
        }
    }

    // Process all results
    let mut ok_count = 0;
    let mut err_count = 0;

    for result in results {
        match result {
            Result::Ok(_) => ok_count += 1,
            Result::Err(_) => err_count += 1,
        }
    }

    assert_eq!(ok_count, 500);
    assert_eq!(err_count, 500);

    // Test Option operations
    let mut options = Vec::new();
    
    for i in 0..1000 {
        if i % 3 == 0 {
            options.push(Option::Some(i));
        } else {
            options.push(Option::None);
        }
    }

    let some_count = options.iter().filter(|opt| opt.is_some()).count();
    let none_count = options.iter().filter(|opt| opt.is_none()).count();

    assert_eq!(some_count, 334); // 0, 3, 6, ..., 999 (334 numbers)
    assert_eq!(none_count, 666);
}

/// Helper functions for testing
fn safe_divide(a: f64, b: f64) -> Result<f64, CursedError> {
    if b == 0.0 {
        error_patterns::runtime_error("Division by zero")
    } else {
        Result::Ok(a / b)
    }
}

fn complex_operation() -> Result<i32, CursedError> {
    let step1 = safe_divide(84.0, 2.0)?;
    let step2 = step1 as i32;
    
    if step2 > 0 {
        Result::Ok(step2)
    } else {
        error_patterns::runtime_error("Invalid result")
    }
}

fn error_chain_operation() -> Result<String, CursedError> {
    let _number = safe_divide(10.0, 0.0)?; // This will propagate the error
    Result::Ok("Should not reach here".to_string())
}

/// Test integration with existing error systems
#[test]
fn test_error_system_integration() {
    // Test with I/O operations
    let file_result = std::fs::read_to_string("/nonexistent/file.txt");
    let cursed_result: Result<String, CursedError> = file_result.map_err(|io_err| CursedError::Io(io_err));
    
    assert!(cursed_result.is_err());

    // Test with parsing operations
    let parse_result = "not_a_number".parse::<i32>();
    let cursed_parse_result: Result<i32, CursedError> = parse_result.map_err(|_| {
        CursedError::Parse("Invalid number format".to_string())
    });
    
    assert!(cursed_parse_result.is_err());

    // Test error propagation through function chains
    let chain_result = parse_and_process("42");
    assert!(chain_result.is_ok());
    assert_eq!(chain_result.unwrap(), 84);

    let chain_error = parse_and_process("not_a_number");
    assert!(chain_error.is_err());
}

fn parse_and_process(input: &str) -> Result<i32, CursedError> {
    let number: i32 = input.parse()
        .map_err(|_| CursedError::Parse("Invalid number".to_string()))?;
    
    let doubled = safe_divide(number as f64 * 2.0, 1.0)?;
    Result::Ok(doubled as i32)
}
