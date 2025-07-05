//! JIT Integration Tests for CURSED
//! Tests for the Just-In-Time execution capabilities

use cursed::execution::*;
use cursed::error::CursedError;

#[test]
fn test_println_string() {
    // Test basic println functionality through execution engine
    let code = r#"
        vibez.spill("Hello from JIT!");
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
        sus x = 10;
        sus y = 20;
        sus result = x + y;
        vibez.spill(result);
    "#;
    
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute(code);
    assert!(result.is_ok());
}

#[test]
fn test_function_call() {
    let code = r#"
        slay add(x normie, y normie) normie {
            yolo x + y;
        }
        
        sus result = add(5, 3);
        vibez.spill(result);
    "#;
    
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute(code);

    assert!(result.is_ok());
}

#[test]
fn test_control_flow() {
    let code = r#"
        sus x = 10;
        lowkey x > 5 {
            vibez.spill("x is greater than 5");
        } highkey {
            vibez.spill("x is not greater than 5");
        }
    "#;
    
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute(code);
    assert!(result.is_ok());
}

#[test]
fn test_loop_execution() {
    let code = r#"
        slay main() normie {
            bestie i in 0..3 {
                vibez.spill(i);
            }
            yolo 0;
        }
        
        sus result = main();
    "#;
    
    let mut engine = CursedExecutionEngine::new().unwrap();
    let result = engine.execute(code);

    assert!(result.is_ok());
}
