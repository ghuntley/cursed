//! Common test utilities for the CURSED test suite

use cursed::code;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::object::ObjectRef;
use cursed::parser::Parser;
use cursed::prelude::JitOptions;

// AST Factory for creating test AST nodes
pub mod ast_factory;

pub mod tracing {
    use std::sync::Once;
    
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
    
    /// Macro to initialize tracing in tests
    #[macro_export]
    macro_rules! init_tracing {
        () => {
            // Initialize tracing if not already done
            $crate::common::tracing::setup();
        };
    }
}

/// Benchmark timer utility
pub mod timing {
    use std::time::{Duration, Instant};
    
    pub struct Timer {
        start: Instant,
        operation: String,
    }
    
    impl Timer {
        pub fn new(operation: &str) -> Self {
            let timer = Timer {
                start: Instant::now(),
                operation: operation.to_string(),
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
pub fn run_jit_test(input: &str) -> Result<ObjectRef, Error> {
    // Parse the input
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    // Set up JIT options
    let options = JitOptions::default().with_main_args(vec![]);
    
    // Compile and run
    code::jit_compile_and_run(&program, options)
}

/// Helper for integer return values
pub fn run_jit_test_int(input: &str) -> Result<i64, Error> {
    let result = run_jit_test(input)?;
    result.as_i64().ok_or_else(|| 
        Error::from_str("Expected integer return value")
    )
}

/// Helper for string return values
pub fn run_jit_test_string(input: &str) -> Result<String, Error> {
    let result = run_jit_test(input)?;
    result.as_string().ok_or_else(|| 
        Error::from_str("Expected string return value")
    )
}

/// Helper for boolean return values
pub fn run_jit_test_bool(input: &str) -> Result<bool, Error> {
    let result = run_jit_test(input)?;
    result.as_bool().ok_or_else(|| 
        Error::from_str("Expected boolean return value")
    )
}

/// Helper for testing expressions
pub fn test_expression(expr: &str, expected: impl Into<ObjectRef>) -> Result<(), Error> {
    // Create a simple program that returns the expression
    let program = format!("slay main() lit {{ return {} }}", expr);
    
    // Run the test
    let result = run_jit_test(&program)?;
    
    // Compare with expected value
    let expected = expected.into();
    if result != expected {
        return Err(Error::from_str(&format!(
            "Expected {:?}, got {:?}", expected, result
        )));
    }
    
    Ok(())
}

/// Helper for testing container iteration
pub fn test_container_iteration(container_code: &str, expected_values: Vec<ObjectRef>) -> Result<(), Error> {
    // Create a program that iterates over the container and collects results
    let program = format!("slay main() tea {{
        sus container = {};
        sus results tea = "";
        
        bestie value := flex container {{
            // Convert each value to string and append to results
            results = results + tea(value) + ",";
        }}
        
        yolo results; // Return the collected results
    }}", container_code);
    
    // Run the test
    let result = run_jit_test_string(&program)?;
    
    // Compare with expected values
    let expected_str = expected_values.iter()
        .map(|obj| obj.to_string())
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
pub fn test_array_operations(ops: &str, expected_result: impl Into<ObjectRef>) -> Result<(), Error> {
    // Create a program that performs the operations on an array
    let program = format!("slay main() lit {{
        {}
    }}", ops);
    
    // Run the test and verify result
    let result = run_jit_test(&program)?;
    let expected = expected_result.into();
    
    if result != expected {
        return Err(Error::from_str(&format!(
            "Expected {:?}, got {:?}", expected, result
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
        $crate::common::test_expression($expr, $expected).unwrap();
    };
}

/// Helper functions for interface inheritance path visualization
pub mod interface_path {
    use cursed::codegen::llvm::LlvmCodeGenerator;
    use cursed::codegen::llvm::interface_type_assertion_path_visualization::InterfaceTypeAssertionPathVisualization;
    use cursed::error::Error;
    use std::collections::{HashMap, HashSet};
    
    /// Create a test interface hierarchy and generate a path visualization
    pub fn create_visualization(
        interfaces: Vec<(String, Vec<String>)>, // (interface_name, extends)
        source: &str,
        target: &str
    ) -> Result<String, Error> {
        // Create code generator
        let mut code_generator = LlvmCodeGenerator::new_for_test()?;
        
        // Convert interfaces to the expected format
        let mut hierarchy = HashMap::new();
        for (name, extends) in interfaces {
            let extends_set: HashSet<String> = extends.into_iter().collect();
            hierarchy.insert(name, extends_set);
        }
        
        // Set up the mock hierarchy
        #[cfg(test)]
        code_generator.test_interface_hierarchy = Some(hierarchy.clone());
        #[cfg(test)]
        code_generator.test_all_interfaces = Some(hierarchy.keys().cloned().collect());
        
        // Find the path
        let path = code_generator.find_interface_inheritance_path(source, target)?;
        
        // Return the path as a string
        Ok(path.to_string())
    }
    
    /// Generate a DOT graph of an interface hierarchy
    pub fn generate_dot_graph(
        interfaces: Vec<(String, Vec<String>)>, // (interface_name, extends)
        root: Option<&str>
    ) -> Result<String, Error> {
        // Create code generator
        let mut code_generator = LlvmCodeGenerator::new_for_test()?;
        
        // Convert interfaces to the expected format
        let mut hierarchy = HashMap::new();
        for (name, extends) in interfaces {
            let extends_set: HashSet<String> = extends.into_iter().collect();
            hierarchy.insert(name, extends_set);
        }
        
        // Set up the mock hierarchy
        #[cfg(test)]
        code_generator.test_interface_hierarchy = Some(hierarchy.clone());
        #[cfg(test)]
        code_generator.test_all_interfaces = Some(hierarchy.keys().cloned().collect());
        
        // Generate the DOT graph
        code_generator.generate_dot_graph(root)
    }
}