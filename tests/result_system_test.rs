//! Comprehensive test suite for CURSED Result and Option system
//!
//! This test suite validates all aspects of the Result/Option error handling system
//! including type definitions, LLVM integration, parser support, and standard library
//! integration.

use cursed::types::result::{Result, Option, ResultTypeExpression, OptionTypeExpression, result_utils, error_patterns};
use cursed::error::{CursedError, SourceLocation};
use cursed::error::types::{
    CursedErrorTrait, ErrorCategory, ErrorSeverity, IoError, ParseError, RuntimeError,
    ErrorManager, ErrorManagerConfig, error_constructors
};
use cursed::stdlib::errors::{
    CursedResult, CursedOption, ErrorFormatter, std_errors, ErrorReporter, recovery
};
use cursed::parser::result_types::{
    ResultPattern, ResultMatchExpression, TryExpression, UnwrapExpression,
    ResultConstructorExpression, ResultConstructor, MatchArm
};
use cursed::ast::traits::{Node, Expression};
use cursed::ast::identifiers::Identifier;
use cursed::ast::literals::NumberLiteral;
use cursed::value::Value;

use std::collections::HashMap;

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

/// Test enhanced error hierarchy
#[test]
fn test_enhanced_error_hierarchy() {
    // Test IoError
    let io_error = IoError::file_not_found("/path/to/file.txt");
    assert_eq!(io_error.category(), ErrorCategory::Io);
    assert_eq!(io_error.severity(), ErrorSeverity::Error);
    assert_eq!(io_error.error_code(), "IO_FILE_NOT_FOUND");
    assert!(io_error.is_recoverable());
    assert!(!io_error.should_panic());

    // Test ParseError
    let parse_error = ParseError::syntax_error("Missing semicolon", 10, 5);
    assert_eq!(parse_error.category(), ErrorCategory::Parse);
    assert_eq!(parse_error.severity(), ErrorSeverity::Error);
    assert_eq!(parse_error.error_code(), "PARSE_SYNTAX_ERROR");
    
    let location = parse_error.source_location().unwrap();
    assert_eq!(location.line, 10);
    assert_eq!(location.column, 5);

    // Test RuntimeError
    let runtime_error = RuntimeError::division_by_zero(20, 15);
    assert_eq!(runtime_error.category(), ErrorCategory::Runtime);
    assert_eq!(runtime_error.severity(), ErrorSeverity::Error);
    assert_eq!(runtime_error.error_code(), "RUNTIME_DIVISION_BY_ZERO");

    // Test error display
    let display = format!("{}", io_error);
    assert!(display.contains("IO_FILE_NOT_FOUND"));
    assert!(display.contains("/path/to/file.txt"));
}

/// Test error manager
#[test]
fn test_error_manager() {
    let manager = ErrorManager::new();
    
    let error1 = error_constructors::io_error("TEST_IO", "I/O test", std::io::ErrorKind::NotFound);
    let error2 = error_constructors::parse_error("TEST_PARSE", "Parse test");
    
    manager.add_error(error1).unwrap();
    manager.add_error(error2).unwrap();
    
    let io_count = manager.get_error_count_by_category(ErrorCategory::Io).unwrap();
    let parse_count = manager.get_error_count_by_category(ErrorCategory::Parse).unwrap();
    
    assert_eq!(io_count, 1);
    assert_eq!(parse_count, 1);
    
    let stats = manager.get_statistics().unwrap();
    assert_eq!(stats.total_errors, 2);
    assert_eq!(stats.total_chains, 2);

    // Test error retrieval by category
    let io_errors = manager.get_errors_by_category(ErrorCategory::Io).unwrap();
    assert_eq!(io_errors.len(), 1);

    let parse_errors = manager.get_errors_by_category(ErrorCategory::Parse).unwrap();
    assert_eq!(parse_errors.len(), 1);

    // Test error retrieval by severity
    let error_level = manager.get_errors_by_severity(ErrorSeverity::Error).unwrap();
    assert_eq!(error_level.len(), 2);

    let critical_level = manager.get_errors_by_severity(ErrorSeverity::Critical).unwrap();
    assert_eq!(critical_level.len(), 0);

    // Test cleanup
    manager.clear_errors().unwrap();
    let stats_after = manager.get_statistics().unwrap();
    assert_eq!(stats_after.total_errors, 0);
}

/// Test standard library error utilities
#[test]
fn test_stdlib_error_utilities() {
    // Test standard error constructors
    let file_error = std_errors::file_not_found("/missing/file.txt");
    assert!(file_error.is_err());

    let div_error = std_errors::division_by_zero(10, 5);
    assert!(div_error.is_err());

    let type_error = std_errors::type_mismatch("String", "Number", 15, 10);
    assert!(type_error.is_err());

    // Test error formatter
    let formatter = ErrorFormatter::new();
    let error = CursedError::Runtime("Test error".to_string());
    let formatted = formatter.format_error(&error);
    assert!(formatted.contains("Test error"));

    // Test with colors disabled
    let formatter_no_color = ErrorFormatter::new().with_colors(false);
    let formatted_no_color = formatter_no_color.format_error(&error);
    assert!(formatted_no_color.contains("Test error"));

    // Test error list formatting
    let errors = vec![
        CursedError::Runtime("Error 1".to_string()),
        CursedError::Parse("Error 2".to_string()),
    ];
    let formatted_list = formatter.format_error_list(&errors);
    assert!(formatted_list.contains("1. "));
    assert!(formatted_list.contains("2. "));
    assert!(formatted_list.contains("Error 1"));
    assert!(formatted_list.contains("Error 2"));
}

/// Test recovery utilities
#[test]
fn test_recovery_utilities() {
    // Test retry with backoff
    let mut attempt_count = 0;
    let result = recovery::retry_with_backoff(
        || {
            attempt_count += 1;
            if attempt_count < 3 {
                error_patterns::runtime_error("Test error")
            } else {
                Ok(42)
            }
        },
        5,
        1
    );

    assert_eq!(result.unwrap(), 42);
    assert_eq!(attempt_count, 3);

    // Test try_or_none
    let option_result = recovery::try_or_none(|| error_patterns::runtime_error::<i32>("Test error"));
    assert!(option_result.is_none());

    let option_success = recovery::try_or_none(|| Ok(42));
    assert_eq!(option_success.unwrap(), 42);

    // Test try_or_default
    let default_result = recovery::try_or_default(|| error_patterns::runtime_error("Test error"), "default");
    assert_eq!(default_result, "default");

    let success_result = recovery::try_or_default(|| Ok("success"), "default");
    assert_eq!(success_result, "success");

    // Test try_alternatives
    let operations: Vec<Box<dyn FnOnce() -> CursedResult<i32>>> = vec![
        Box::new(|| error_patterns::runtime_error("Error 1")),
        Box::new(|| error_patterns::runtime_error("Error 2")),
        Box::new(|| Ok(42)),
    ];

    let result = recovery::try_alternatives(operations);
    assert_eq!(result.unwrap(), 42);
}

/// Test AST type expressions
#[test]
fn test_ast_type_expressions() {
    // Test ResultTypeExpression
    let ok_type = Box::new(Identifier::new("normie".to_string(), "normie".to_string()));
    let err_type = Box::new(Identifier::new("based".to_string(), "based".to_string()));
    
    let result_type = ResultTypeExpression::new(
        "Result".to_string(),
        ok_type,
        err_type,
    );

    assert_eq!(result_type.string(), "Result<normie, based>");
    assert_eq!(result_type.type_name(), "Result<normie, based>");
    assert!(result_type.is_generic());

    // Test OptionTypeExpression
    let inner_type = Box::new(Identifier::new("normie".to_string(), "normie".to_string()));
    
    let option_type = OptionTypeExpression::new(
        "Option".to_string(),
        inner_type,
    );

    assert_eq!(option_type.string(), "Option<normie>");
    assert_eq!(option_type.type_name(), "Option<normie>");
    assert!(option_type.is_generic());

    // Test cloning
    let cloned_result = result_type.clone_box();
    assert_eq!(result_type.string(), cloned_result.string());

    let cloned_option = option_type.clone_box();
    assert_eq!(option_type.string(), cloned_option.string());
}

/// Test parser AST nodes
#[test]
fn test_parser_ast_nodes() {
    // Test TryExpression
    let identifier = Box::new(Identifier::new("result".to_string(), "result".to_string()));
    let try_expr = TryExpression::new("?".to_string(), identifier);
    
    assert_eq!(try_expr.string(), "result?");
    assert_eq!(try_expr.token_literal(), "?");

    // Test UnwrapExpression
    let identifier = Box::new(Identifier::new("option".to_string(), "option".to_string()));
    let unwrap_expr = UnwrapExpression::new("unwrap".to_string(), identifier);
    
    assert_eq!(unwrap_expr.string(), "option.unwrap()");

    let identifier = Box::new(Identifier::new("option".to_string(), "option".to_string()));
    let default = Box::new(NumberLiteral::new("42".to_string(), 42.0));
    let unwrap_or_expr = UnwrapExpression::with_default("unwrap_or".to_string(), identifier, default);
    
    assert_eq!(unwrap_or_expr.string(), "option.unwrap_or(42)");

    // Test ResultConstructorExpression
    let number = Box::new(NumberLiteral::new("42".to_string(), 42.0));
    let ok_constructor = ResultConstructor::Ok(number);
    let ok_expr = ResultConstructorExpression::new("Ok".to_string(), ok_constructor);
    
    assert_eq!(ok_expr.string(), "Ok(42)");

    let none_constructor = ResultConstructor::None;
    let none_expr = ResultConstructorExpression::new("None".to_string(), none_constructor);
    
    assert_eq!(none_expr.string(), "None");

    // Test cloning
    let cloned_try = try_expr.clone_box();
    assert_eq!(try_expr.string(), cloned_try.string());

    let cloned_unwrap = unwrap_expr.clone_box();
    assert_eq!(unwrap_expr.string(), cloned_unwrap.string());

    let cloned_ok = ok_expr.clone_box();
    assert_eq!(ok_expr.string(), cloned_ok.string());
}

/// Test Result patterns
#[test]
fn test_result_patterns() {
    let identifier = Box::new(Identifier::new("x".to_string(), "x".to_string()));
    
    // Test Ok pattern
    let ok_pattern = ResultPattern::Ok(identifier.clone_box());
    assert_eq!(ok_pattern.string(), "Ok(x)");

    // Test Err pattern
    let err_pattern = ResultPattern::Err(identifier.clone_box());
    assert_eq!(err_pattern.string(), "Err(x)");

    // Test Some pattern
    let some_pattern = ResultPattern::Some(identifier.clone_box());
    assert_eq!(some_pattern.string(), "Some(x)");

    // Test None pattern
    let none_pattern = ResultPattern::None;
    assert_eq!(none_pattern.string(), "None");

    // Test Wildcard pattern
    let wildcard_pattern = ResultPattern::Wildcard;
    assert_eq!(wildcard_pattern.string(), "_");
}

/// Test match expressions
#[test]
fn test_match_expressions() {
    let value = Box::new(Identifier::new("result".to_string(), "result".to_string()));
    
    // Create match arms
    let ok_body = Box::new(NumberLiteral::new("42".to_string(), 42.0));
    let ok_pattern = ResultPattern::Ok(Box::new(Identifier::new("x".to_string(), "x".to_string())));
    let ok_arm = MatchArm {
        pattern: ok_pattern,
        body: ok_body,
        guard: None,
    };

    let err_body = Box::new(NumberLiteral::new("0".to_string(), 0.0));
    let err_pattern = ResultPattern::Err(Box::new(Identifier::new("e".to_string(), "e".to_string())));
    let err_arm = MatchArm {
        pattern: err_pattern,
        body: err_body,
        guard: None,
    };

    let arms = vec![ok_arm, err_arm];
    
    let match_expr = ResultMatchExpression::new("match".to_string(), value, arms);
    
    let match_string = match_expr.string();
    assert!(match_string.contains("match result"));
    assert!(match_string.contains("Ok(x) => 42"));
    assert!(match_string.contains("Err(e) => 0"));

    // Test cloning
    let cloned_match = match_expr.clone_box();
    assert_eq!(match_expr.string(), cloned_match.string());
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

    // Test safe_index function
    let array = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];
    let index_ok = safe_index(&array, 1);
    assert!(index_ok.is_ok());
    assert_eq!(index_ok.unwrap(), Value::Number(2.0));

    let index_err = safe_index(&array, 10);
    assert!(index_err.is_err());
}

/// Test error chain functionality
#[test]
fn test_error_chains() {
    use cursed::error::types::ErrorChain;

    let mut chain = ErrorChain::new();
    assert!(chain.is_empty());
    assert_eq!(chain.len(), 0);

    let error1 = error_constructors::runtime_error("TEST_ERROR_1", "First error");
    let error2 = error_constructors::parse_error("TEST_ERROR_2", "Second error");

    chain.add_error(error1);
    chain.add_error(error2);

    assert!(!chain.is_empty());
    assert_eq!(chain.len(), 2);
    
    let root = chain.root_cause().unwrap();
    assert_eq!(root.error_code(), "TEST_ERROR_1");
    
    let recent = chain.most_recent().unwrap();
    assert_eq!(recent.error_code(), "TEST_ERROR_2");

    let highest_severity = chain.highest_severity();
    assert_eq!(highest_severity, ErrorSeverity::Error);
}

/// Test error severity ordering
#[test]
fn test_error_severity_ordering() {
    let severities = vec![
        ErrorSeverity::Info,
        ErrorSeverity::Warning,
        ErrorSeverity::Error,
        ErrorSeverity::Critical,
        ErrorSeverity::Fatal,
    ];

    for i in 0..severities.len() {
        for j in i + 1..severities.len() {
            assert!(severities[j] > severities[i]);
        }
    }

    // Test logging and monitoring thresholds
    assert!(ErrorSeverity::Warning.should_log());
    assert!(ErrorSeverity::Error.should_log());
    assert!(!ErrorSeverity::Info.should_log());

    assert!(ErrorSeverity::Error.should_monitor());
    assert!(ErrorSeverity::Critical.should_monitor());
    assert!(!ErrorSeverity::Warning.should_monitor());
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

/// Helper functions for testing
fn safe_divide(a: f64, b: f64) -> CursedResult<f64> {
    if b == 0.0 {
        error_patterns::runtime_error("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn safe_index(array: &[Value], index: usize) -> CursedResult<Value> {
    if index >= array.len() {
        error_patterns::runtime_error(&format!(
            "Index {} out of bounds for array of length {}",
            index,
            array.len()
        ))
    } else {
        Ok(array[index].clone())
    }
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

fn complex_operation() -> CursedResult<i32> {
    let step1 = safe_divide(84.0, 2.0)?;
    let step2 = step1 as i32;
    
    if step2 > 0 {
        Ok(step2)
    } else {
        error_patterns::runtime_error("Invalid result")
    }
}

fn error_chain_operation() -> CursedResult<String> {
    let _number = safe_divide(10.0, 0.0)?; // This will propagate the error
    Ok("Should not reach here".to_string())
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

/// Test integration with existing error systems
#[test]
fn test_error_system_integration() {
    // Test with I/O operations
    let file_result = std::fs::read_to_string("/nonexistent/file.txt");
    let cursed_result: CursedResult<String> = file_result.map_err(|io_err| CursedError::Io(io_err));
    
    assert!(cursed_result.is_err());

    // Test with parsing operations
    let parse_result = "not_a_number".parse::<i32>();
    let cursed_parse_result: CursedResult<i32> = parse_result.map_err(|_| {
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

fn parse_and_process(input: &str) -> CursedResult<i32> {
    let number: i32 = input.parse()
        .map_err(|_| CursedError::Parse("Invalid number".to_string()))?;
    
    let doubled = safe_divide(number as f64 * 2.0, 1.0)?;
    Ok(doubled as i32)
}

/// Test documentation examples
#[test]
fn test_documentation_examples() {
    // Example from Result documentation
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Result::Err("Division by zero".to_string())
        } else {
            Result::Ok(a / b)
        }
    }

    let good_result = divide(10.0, 2.0);
    assert_eq!(good_result.unwrap(), 5.0);

    let bad_result = divide(10.0, 0.0);
    assert_eq!(bad_result.unwrap_err(), "Division by zero");

    // Example from Option documentation
    fn find_index(arr: &[i32], target: i32) -> Option<usize> {
        for (i, &item) in arr.iter().enumerate() {
            if item == target {
                return Option::Some(i);
            }
        }
        Option::None
    }

    let array = [1, 2, 3, 4, 5];
    let found = find_index(&array, 3);
    assert_eq!(found.unwrap(), 2);

    let not_found = find_index(&array, 6);
    assert!(not_found.is_none());
}
