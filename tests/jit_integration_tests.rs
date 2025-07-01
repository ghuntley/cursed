//! JIT Integration Tests for CURSED
//! Tests for the Just-In-Time execution capabilities

use cursed::execution::*;
use cursed::error::CursedError;

#[test]
fn test_println_string() {
    // Test basic println functionality through execution engine
    let code = r#"
        println("Hello from JIT!");
    "#;
    
    // Create execution engine
    let mut engine = CursedExecutionEngine::new().unwrap();
    
    // Execute code
    let result = engine.execute(code);
    assert!(result.is_ok());
}

#[test]
fn test_basic_arithmetic() {
    let code = r#"
        let x = 10;
        let y = 20;
        let result = x + y;
        println(result);
    "#;
    
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute(code);
    assert!(result.is_ok());
}

#[test]
fn test_function_call() {
    let code = r#"
        func add(x: int, y: int) -> int {
            return x + y;
        }
        
        let result = add(5, 3);
        println(result);
    "#;
    
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute(code);
    assert!(result.is_ok());
}

#[test]
fn test_control_flow() {
    let code = r#"
        let x = 10;
        if x > 5 {
            println("x is greater than 5");
        } else {
            println("x is not greater than 5");
        }
    "#;
    
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute(code);
    assert!(result.is_ok());
}

#[test]
fn test_loop_execution() {
    let code = r#"
        for i in 0..3 {
            println(i);
        }
    "#;
    
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute(code);
    assert!(result.is_ok());
}
