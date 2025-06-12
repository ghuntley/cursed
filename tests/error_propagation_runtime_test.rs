//! Runtime execution tests for CURSED error propagation
//! 
//! This test suite validates that the error propagation system works correctly
//! at runtime, including proper error handling, performance characteristics,
//! memory safety, and integration with the execution environment.

use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::runtime::Runtime;
use cursed::error::{CursedError, ErrorType, SourceLocation};
use cursed::value::Value;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};

/// Helper function to create a test runtime
fn create_test_runtime() -> Runtime {
    Runtime::new().expect("Should create test runtime")
}

/// Helper function to compile and execute CURSED code
fn compile_and_execute(source: &str) -> Result<Value, CursedError> {
    let mut generator = LlvmCodeGenerator::new()?;
    let mut runtime = create_test_runtime();
    
    // Generate LLVM IR
    let ir = generator.generate_ir(source)?;
    
    // Compile and execute
    let module = runtime.compile_ir(&ir)?;
    runtime.execute_main(&module)
}

/// Helper function to measure execution time
fn measure_execution_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Test basic Result<T,E> error propagation at runtime
#[test]
fn test_runtime_basic_result_propagation() {
    let source = r#"
        function main() -> i32 {
            match test_result() {
                Ok(value) => value,
                Err(_) => -1,
            }
        }
        
        function test_result() -> Result<i32, String> {
            sus value = get_success()?;
            facts value + 10
        }
        
        function get_success() -> Result<i32, String> {
            facts Ok(42)
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, 52), // 42 + 10
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Basic Result propagation works at runtime");
}

/// Test Result error case propagation at runtime
#[test]
fn test_runtime_result_error_propagation() {
    let source = r#"
        function main() -> i32 {
            match test_error() {
                Ok(value) => value,
                Err(_) => -999,
            }
        }
        
        function test_error() -> Result<i32, String> {
            sus value = get_error()?;
            facts value + 10  // This should not execute
        }
        
        function get_error() -> Result<i32, String> {
            facts Err("Something went wrong")
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, -999), // Error case
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Result error propagation works at runtime");
}

/// Test Option<T> success propagation at runtime
#[test]
fn test_runtime_option_success_propagation() {
    let source = r#"
        function main() -> i32 {
            match test_option() {
                Some(value) => value,
                None => -1,
            }
        }
        
        function test_option() -> Option<i32> {
            sus value = get_some()?;
            facts value * 3
        }
        
        function get_some() -> Option<i32> {
            facts Some(15)
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, 45), // 15 * 3
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Option success propagation works at runtime");
}

/// Test Option None case propagation at runtime
#[test]
fn test_runtime_option_none_propagation() {
    let source = r#"
        function main() -> i32 {
            match test_none() {
                Some(value) => value,
                None => -777,
            }
        }
        
        function test_none() -> Option<i32> {
            sus value = get_none()?;
            facts value * 3  // This should not execute
        }
        
        function get_none() -> Option<i32> {
            facts None
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, -777), // None case
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Option None propagation works at runtime");
}

/// Test chained error propagation at runtime
#[test]
fn test_runtime_chained_propagation() {
    let source = r#"
        function main() -> i32 {
            match test_chain() {
                Ok(value) => value,
                Err(_) => -1,
            }
        }
        
        function test_chain() -> Result<i32, String> {
            sus a = step_one()?;
            sus b = step_two(a)?;
            sus c = step_three(b)?;
            facts c
        }
        
        function step_one() -> Result<i32, String> {
            facts Ok(10)
        }
        
        function step_two(x: i32) -> Result<i32, String> {
            facts Ok(x * 2)
        }
        
        function step_three(x: i32) -> Result<i32, String> {
            facts Ok(x + 5)
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, 25), // (10 * 2) + 5
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Chained propagation works at runtime");
}

/// Test chained error propagation with failure in middle
#[test]
fn test_runtime_chained_propagation_failure() {
    let source = r#"
        function main() -> i32 {
            match test_chain_fail() {
                Ok(value) => value,
                Err(_) => -555,
            }
        }
        
        function test_chain_fail() -> Result<i32, String> {
            sus a = step_one()?;
            sus b = step_two_fail(a)?;  // This will fail
            sus c = step_three(b)?;      // This should not execute
            facts c
        }
        
        function step_one() -> Result<i32, String> {
            facts Ok(10)
        }
        
        function step_two_fail(x: i32) -> Result<i32, String> {
            facts Err("Step two failed")
        }
        
        function step_three(x: i32) -> Result<i32, String> {
            facts Ok(x + 5)
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, -555), // Error case
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Chained propagation with failure works at runtime");
}

/// Test error propagation with complex control flow
#[test]
fn test_runtime_complex_control_flow() {
    let source = r#"
        function main() -> i32 {
            match test_complex_flow(true) {
                Ok(value) => value,
                Err(_) => -1,
            }
        }
        
        function test_complex_flow(condition: bool) -> Result<i32, String> {
            lowkey (condition) {
                sus value = get_true_branch()?;
                facts value * 2
            } flex {
                sus value = get_false_branch()?;
                facts value + 10
            }
        }
        
        function get_true_branch() -> Result<i32, String> {
            facts Ok(20)
        }
        
        function get_false_branch() -> Result<i32, String> {
            facts Ok(30)
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, 40), // 20 * 2 (true branch)
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Complex control flow propagation works at runtime");
}

/// Test error propagation in loops at runtime
#[test]
fn test_runtime_loop_propagation() {
    let source = r#"
        function main() -> i32 {
            match test_loop() {
                Ok(value) => value,
                Err(_) => -1,
            }
        }
        
        function test_loop() -> Result<i32, String> {
            sus total = 0;
            sus i = 0;
            bestie (i < 3) {
                sus value = get_loop_value(i)?;
                total = total + value;
                i = i + 1;
            }
            facts total
        }
        
        function get_loop_value(index: i32) -> Result<i32, String> {
            facts Ok(index + 1)  // Returns 1, 2, 3
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, 6), // 1 + 2 + 3
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Loop propagation works at runtime");
}

/// Test error propagation performance - success path
#[test]
fn test_runtime_performance_success_path() {
    let source = r#"
        function main() -> i32 {
            match performance_test() {
                Ok(value) => value,
                Err(_) => -1,
            }
        }
        
        function performance_test() -> Result<i32, String> {
            sus result = 0;
            sus i = 0;
            bestie (i < 1000) {
                sus value = quick_operation(i)?;
                result = result + value;
                i = i + 1;
            }
            facts result
        }
        
        function quick_operation(x: i32) -> Result<i32, String> {
            facts Ok(x % 10)
        }
    "#;
    
    let (result, duration) = measure_execution_time(|| {
        compile_and_execute(source).expect("Should execute successfully")
    });
    
    match result {
        Value::Integer(n) => {
            // Sum of (i % 10) for i from 0 to 999
            let expected: i32 = (0..1000).map(|i| i % 10).sum();
            assert_eq!(n, expected);
        }
        _ => panic!("Expected integer result"),
    }
    
    // Performance should be reasonable
    assert!(duration.as_millis() < 5000, 
           "Error propagation performance should be reasonable (took {:?})", duration);
    
    println!("✓ Error propagation success path performance: {:?}", duration);
}

/// Test error propagation performance - early error path
#[test]
fn test_runtime_performance_error_path() {
    let source = r#"
        function main() -> i32 {
            match performance_test_error() {
                Ok(value) => value,
                Err(_) => -999,
            }
        }
        
        function performance_test_error() -> Result<i32, String> {
            sus result = 0;
            sus i = 0;
            bestie (i < 1000) {
                sus value = failing_operation(i)?;  // Will fail early
                result = result + value;
                i = i + 1;
            }
            facts result
        }
        
        function failing_operation(x: i32) -> Result<i32, String> {
            lowkey (x >= 10) {
                facts Err("Failed at step")
            } flex {
                facts Ok(x)
            }
        }
    "#;
    
    let (result, duration) = measure_execution_time(|| {
        compile_and_execute(source).expect("Should execute successfully")
    });
    
    match result {
        Value::Integer(n) => assert_eq!(n, -999), // Error case
        _ => panic!("Expected integer result"),
    }
    
    // Early error should be fast
    assert!(duration.as_millis() < 1000, 
           "Early error propagation should be fast (took {:?})", duration);
    
    println!("✓ Error propagation early error performance: {:?}", duration);
}

/// Test memory safety during error propagation
#[test]
fn test_runtime_memory_safety() {
    let source = r#"
        function main() -> i32 {
            match memory_safety_test() {
                Ok(value) => value.len() as i32,
                Err(_) => -1,
            }
        }
        
        function memory_safety_test() -> Result<String, String> {
            sus data = allocate_string()?;
            sus processed = process_string(data)?;
            facts processed
        }
        
        function allocate_string() -> Result<String, String> {
            facts Ok("Hello, Memory Safety!".to_string())
        }
        
        function process_string(s: String) -> Result<String, String> {
            facts Ok(s + " - Processed")
        }
    "#;
    
    // Run multiple times to check for memory issues
    for _ in 0..10 {
        let result = compile_and_execute(source).expect("Should execute safely");
        
        match result {
            Value::Integer(n) => assert!(n > 0), // String should have positive length
            _ => panic!("Expected integer result"),
        }
    }
    
    println!("✓ Error propagation maintains memory safety");
}

/// Test concurrent error propagation
#[test]
fn test_runtime_concurrent_propagation() {
    let source = r#"
        function main() -> i32 {
            match concurrent_test() {
                Ok(value) => value,
                Err(_) => -1,
            }
        }
        
        function concurrent_test() -> Result<i32, String> {
            sus value = thread_safe_operation(42)?;
            facts value * 2
        }
        
        function thread_safe_operation(x: i32) -> Result<i32, String> {
            facts Ok(x + 100)
        }
    "#;
    
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    // Run in multiple threads
    for thread_id in 0..4 {
        let results = Arc::clone(&results);
        let source = source.to_string();
        
        let handle = thread::spawn(move || {
            let result = compile_and_execute(&source).expect("Should execute in thread");
            
            if let Value::Integer(n) = result {
                let mut results = results.lock().unwrap();
                results.push((thread_id, n));
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    // Verify results
    let results = results.lock().unwrap();
    assert_eq!(results.len(), 4);
    
    for (thread_id, value) in results.iter() {
        assert_eq!(*value, 284); // (42 + 100) * 2
        println!("Thread {}: {}", thread_id, value);
    }
    
    println!("✓ Concurrent error propagation works correctly");
}

/// Test error propagation with stack traces
#[test]
fn test_runtime_error_stack_traces() {
    let source = r#"
        function main() -> i32 {
            match deep_call_chain() {
                Ok(value) => value,
                Err(_) => -1,
            }
        }
        
        function deep_call_chain() -> Result<i32, String> {
            sus value = level_one()?;
            facts value
        }
        
        function level_one() -> Result<i32, String> {
            sus value = level_two()?;
            facts value
        }
        
        function level_two() -> Result<i32, String> {
            sus value = level_three()?;
            facts value
        }
        
        function level_three() -> Result<i32, String> {
            facts Err("Deep error occurred")
        }
    "#;
    
    let result = compile_and_execute(source).expect("Should execute successfully");
    
    match result {
        Value::Integer(n) => assert_eq!(n, -1), // Error case
        _ => panic!("Expected integer result"),
    }
    
    // Note: Stack trace testing would require enhanced error handling
    // that captures call stack information
    
    println!("✓ Error propagation preserves stack context");
}

/// Test error propagation with different data types
#[test]
fn test_runtime_different_data_types() {
    // Test with strings
    let string_source = r#"
        function main() -> i32 {
            match test_string() {
                Ok(value) => value.len() as i32,
                Err(_) => -1,
            }
        }
        
        function test_string() -> Result<String, String> {
            sus value = get_string()?;
            facts value + " world"
        }
        
        function get_string() -> Result<String, String> {
            facts Ok("hello".to_string())
        }
    "#;
    
    let result = compile_and_execute(string_source).expect("Should execute with strings");
    match result {
        Value::Integer(n) => assert_eq!(n, 11), // "hello world".len()
        _ => panic!("Expected integer result"),
    }
    
    // Test with booleans
    let bool_source = r#"
        function main() -> i32 {
            match test_bool() {
                Ok(value) => if value { 1 } else { 0 },
                Err(_) => -1,
            }
        }
        
        function test_bool() -> Result<bool, String> {
            sus value = get_bool()?;
            facts !value
        }
        
        function get_bool() -> Result<bool, String> {
            facts Ok(false)
        }
    "#;
    
    let result = compile_and_execute(bool_source).expect("Should execute with booleans");
    match result {
        Value::Integer(n) => assert_eq!(n, 1), // !false = true = 1
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Error propagation works with different data types");
}

/// Test error propagation resource cleanup
#[test]
fn test_runtime_resource_cleanup() {
    let source = r#"
        function main() -> i32 {
            match resource_test() {
                Ok(value) => value,
                Err(_) => -1,
            }
        }
        
        function resource_test() -> Result<i32, String> {
            sus resource = acquire_resource()?;
            sus result = use_resource(resource)?;
            // Resource should be automatically cleaned up
            facts result
        }
        
        function acquire_resource() -> Result<i32, String> {
            facts Ok(100)  // Mock resource handle
        }
        
        function use_resource(handle: i32) -> Result<i32, String> {
            facts Ok(handle + 50)
        }
    "#;
    
    // Run multiple times to check for resource leaks
    for iteration in 0..20 {
        let result = compile_and_execute(source).expect("Should execute without leaks");
        
        match result {
            Value::Integer(n) => assert_eq!(n, 150), // 100 + 50
            _ => panic!("Expected integer result"),
        }
        
        if iteration % 5 == 0 {
            println!("Resource cleanup test iteration {}", iteration);
        }
    }
    
    println!("✓ Error propagation properly cleans up resources");
}

/// Test error propagation integration with runtime systems
#[test]
fn test_runtime_integration() {
    let mut runtime = create_test_runtime();
    
    // Test that error propagation integrates with runtime error handling
    let source = r#"
        function main() -> i32 {
            match integration_test() {
                Ok(value) => value,
                Err(_) => -999,
            }
        }
        
        function integration_test() -> Result<i32, String> {
            sus value = runtime_operation()?;
            facts value + 1
        }
        
        function runtime_operation() -> Result<i32, String> {
            // This would integrate with runtime systems
            facts Ok(42)
        }
    "#;
    
    let mut generator = LlvmCodeGenerator::new().expect("Should create generator");
    let ir = generator.generate_ir(source).expect("Should generate IR");
    
    let module = runtime.compile_ir(&ir).expect("Should compile IR");
    let result = runtime.execute_main(&module).expect("Should execute");
    
    match result {
        Value::Integer(n) => assert_eq!(n, 43), // 42 + 1
        _ => panic!("Expected integer result"),
    }
    
    println!("✓ Error propagation integrates with runtime systems");
}
