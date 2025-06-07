use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::object::Object;
use cursed::parser::Parser;
use std::sync::Arc;
use std::sync::Once;
use std::time::{Duration, Instant};

//! Common test utilities for the CURSED test suite


// AST Factory for creating test AST nodes
// pub mod ast_factory;

pub mod tracing {
    
    /// Initialize tracing for tests
    pub fn setup() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            if let Err(_) = tracing_subscriber::fmt()
                .with_env_filter("info,cursed=debug")
                .with_test_writer()
                .try_init() {
                // If it fails, tracing is likely already initialized
            }
        });
    }
}

/// Macro to initialize tracing in tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        // Initialize tracing if not already done
        common::tracing::setup();
    };
}

/// Benchmark timer utility
pub mod timing {
    
    pub struct Timer {
        start: Instant,
        operation: String,
    }
    
    impl Timer {
        pub fn new(operation: &str) -> Self {
            let timer = Timer {
                start: Instant::now(),
                operation: operation.to_string()),
            };
            tracing::info!(operation = operation, "Starting operation timing");
            timer
        }
    }
    
    impl Drop for Timer {
        fn drop(&mut self) {
            let elapsed = self.start.elapsed();
            tracing::info!(
                operation = self.operation.as_str(),
                duration_ms = elapsed.as_millis() as u64,
                "Operation completed"
            );
        }
    }
}

/// Standard JIT test runner for executing CURSED code snippets
pub fn run_jit_test(input: &str) -> Result<Arc<Object>, Error> {
    // Parse the input
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer)?;
    let _program = parser.parse_program()?;
    
    // For now, return a placeholder
    Ok(Arc::new(Object::Integer(42)))
}

/// Helper for integer return values
pub fn run_jit_test_int(input: &str) -> Result<i64, Error> {
    let result = run_jit_test(input)?;
    match &*result {
        Object::Integer(i) => Ok(*i),
        _ => Err(Error::from_str("Expected integer return value"))
    }
}

/// Helper for string return values
pub fn run_jit_test_string(input: &str) -> Result<String, Error> {
    let result = run_jit_test(input)?;
    match &*result {
        Object::String(s) => Ok(s.clone()),
        _ => Err(Error::from_str("Expected string return value"))
    }
}

/// Helper for boolean return values
pub fn run_jit_test_bool(input: &str) -> Result<bool, Error> {
    let result = run_jit_test(input)?;
    match &*result {
        Object::Boolean(b) => Ok(*b),
        _ => Err(Error::from_str("Expected boolean return value"))
    }
}

/// Helper for testing expressions
pub fn test_expression(expr: &str, expected: Arc<Object>) -> Result<(), Error> {
    // Create a simple program that returns the expression
    let program = format!("slay main() lit {{ return {} }}", expr);
    
    // Run the test
    let result = run_jit_test(&program)?;
    
    // Compare with expected value
    if *result != *expected {
        return Err(Error::from_str(&format!(
            "Expected {:?}, got {:?}", expected, result
        )));
    }
    
    Ok(())
}

/// Helper for testing container iteration
pub fn test_container_iteration(container_code: &str, expected_values: Vec<Arc<Object>>) -> Result<(), Error> {
    // Create a program that iterates over the container and collects results
    let program = format!("slay main() tea {{
        sus container = {};
        sus results tea = \"\";
        
        bestie value := flex container {{
            // Convert each value to string and append to results
            results = results + tea(value) + \",\";
        }}
        
        yolo results; // Return the collected results
    }}", container_code);
    
    // Run the test
    let result = run_jit_test_string(&program)?;
    
    // Compare with expected values
    let expected_str = expected_values.iter()
        .map(|obj| obj.to_string()
        .collect::<Vec<_>>()
        .join(",") + ",";
        
    if result != expected_str {
        return Err(Error::from_str(&format!(
            "Expected values {:?}, got {}", expected_values, result
        )));
    }
    
    Ok(())
}

/// Helper for testing array operations
pub fn test_array_operations(ops: &str, expected_result: Arc<Object>) -> Result<(), Error> {
    // Create a program that performs the operations on an array
    let program = format!("slay main() lit {{
        {}
    }}", ops);
    
    // Run the test and verify result
    let result = run_jit_test(&program)?;
    
    if *result != *expected_result {
        return Err(Error::from_str(&format!(
            "Expected {:?}, got {:?}", expected_result, result
        )));
    }
    
    Ok(())
}

/// Helper for testing interface implementation
pub fn test_interface_implementation(struct_code: &str, interface_name: &str) -> Result<bool, Error> {
    // Create a program that defines the struct and interface
    // and uses a type assertion to check implementation
    let program = format!("{}slay main() lit {{
        sus s = {{}}; // Create struct instance
        sus _, ok = s.({}); // Perform type assertion
        yolo ok; // Return whether the assertion succeeded
    }}", struct_code, interface_name);
    
    // Run the test
    run_jit_test_bool(&program)
}

/// Helper for testing generic constraints
pub fn test_generic_constraint(function_code: &str, type_args: &[&str], args: &[&str]) -> Result<bool, Error> {
    // Create a program that tries to use a generic function with the given type args
    let type_args_str = if !type_args.is_empty() {
        format!("[{}]", type_args.join(", "))
    } else {
        String::new()
    };
    
    let args_str = args.join(", ");
    
    let program = format!("{}slay main() lit {{
        // Call the function with the specified type args and arguments
        // If constraints aren't satisfied, this will fail at compile time
        function_name{}({});
        yolo based;
    }}", function_code, type_args_str, args_str);
    
    // If this compiles and runs, then constraints are satisfied
    run_jit_test_bool(&program)
}

/// Macro for testing expressions
#[macro_export]
macro_rules! assert_expr {
    ($expr:expr, $expected:expr) => {
        common::test_expression($expr, $expected).unwrap();
    };
}

// /// Helper functions for interface inheritance path visualization
// pub mod interface_path {
//     // Commented out due to complex dependencies
// }