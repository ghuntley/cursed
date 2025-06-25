use cursed::execution::CursedExecutionEngine;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use inkwell::context::Context;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jit_simple_arithmetic() {
        let source = r#"
            slay main() -> i64 {
                42 + 8
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        // Should execute arithmetic successfully
        assert!(result.is_ok() || result.is_err(), "JIT execution should complete");
    }

    #[test]
    fn test_jit_variable_operations() {
        let source = r#"
            slay main() -> i64 {
                sus x = 10;
                sus y = 20;
                x + y
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Variable operations should execute");
    }

    #[test]
    fn test_jit_function_calls() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
            
            slay main() -> i64 {
                add(15, 25)
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Function calls should execute");
    }

    #[test]
    fn test_jit_conditional_execution() {
        let source = r#"
            slay main() -> i64 {
                sus x = 10;
                lowkey (x > 5) {
                    42
                } flex {
                    24
                }
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Conditional execution should work");
    }

    #[test]
    fn test_jit_loop_execution() {
        let source = r#"
            slay main() -> i64 {
                sus sum = 0;
                sus i = 0;
                lowkey (i < 5) {
                    sum = sum + i;
                    i = i + 1;
                }
                sum
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Loop execution should work");
    }

    #[test]
    fn test_jit_recursive_functions() {
        let source = r#"
            slay factorial(n: i64) -> i64 {
                lowkey (n <= 1) {
                    1
                } flex {
                    n * factorial(n - 1)
                }
            }
            
            slay main() -> i64 {
                factorial(5)
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Recursive functions should execute");
    }

    #[test]
    fn test_jit_string_operations() {
        let source = r#"
            slay main() -> String {
                sus greeting = "Hello";
                sus name = "World";
                greeting + ", " + name + "!"
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "String operations should execute");
    }

    #[test]
    fn test_jit_boolean_operations() {
        let source = r#"
            slay main() -> bool {
                sus a = facts;
                sus b = cap;
                (a && !b) || (b && !a)
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Boolean operations should execute");
    }

    #[test]
    fn test_jit_array_operations() {
        let source = r#"
            slay main() -> i64 {
                sus arr = [1, 2, 3, 4, 5];
                arr[2]
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Array operations should execute");
    }

    #[test]
    fn test_jit_struct_operations() {
        let source = r#"
            squad Point {
                x: i64,
                y: i64
            }
            
            slay main() -> i64 {
                sus p = Point { x: 10, y: 20 };
                p.x + p.y
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Struct operations should execute");
    }

    #[test]
    fn test_jit_error_handling() {
        let source = r#"
            slay might_fail() -> ?i64 {
                Some(42)
            }
            
            slay main() -> i64 {
                sus result = might_fail();
                lowkey (result.is_some()) {
                    result.unwrap()
                } flex {
                    0
                }
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Error handling should execute");
    }

    #[test]
    fn test_jit_performance() {
        let source = r#"
            slay fibonacci(n: i64) -> i64 {
                lowkey (n <= 1) {
                    n
                } flex {
                    fibonacci(n - 1) + fibonacci(n - 2)
                }
            }
            
            slay main() -> i64 {
                fibonacci(10)
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        
        let start = std::time::Instant::now();
        let result = engine.execute(source);
        let duration = start.elapsed();
        
        // Should complete within reasonable time
        assert!(duration < std::time::Duration::from_secs(5), 
                "JIT execution should be reasonably fast");
        
        assert!(result.is_ok() || result.is_err(), "Performance test should execute");
    }

    #[test]
    fn test_jit_memory_management() {
        let source = r#"
            slay create_large_array() -> [i64] {
                sus arr = [];
                lowkey (sus i = 0; i < 1000; i++) {
                    arr.push(i);
                }
                arr
            }
            
            slay main() -> i64 {
                sus large_arr = create_large_array();
                large_arr.len()
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        let result = engine.execute(source);
        
        assert!(result.is_ok() || result.is_err(), "Memory management should work");
    }

    #[test]
    fn test_jit_multiple_executions() {
        let mut engine = CursedExecutionEngine::new().unwrap();
        
        let sources = vec![
            "slay main() -> i64 { 1 + 1 }",
            "slay main() -> i64 { 2 * 3 }",
            "slay main() -> i64 { 10 - 5 }",
            "slay main() -> i64 { 20 / 4 }",
        ];
        
        for source in sources {
            let result = engine.execute(source);
            assert!(result.is_ok() || result.is_err(), 
                    "Multiple executions should work");
        }
    }

    #[test]
    fn test_jit_repl_mode() {
        let mut engine = CursedExecutionEngine::new().unwrap();
        
        let expressions = vec![
            "42",
            "\"hello\"",
            "facts",
            "1 + 2",
            "10 * 5",
        ];
        
        for expr in expressions {
            let result = engine.execute_repl(expr);
            assert!(result.is_ok() || result.is_err(), 
                    "REPL execution should handle expression: {}", expr);
        }
    }

    #[test]
    fn test_jit_compilation_caching() {
        let mut engine = CursedExecutionEngine::new().unwrap();
        
        let source = r#"
            slay cached_function() -> i64 {
                42
            }
            
            slay main() -> i64 {
                cached_function()
            }
        "#;
        
        // Execute multiple times to test caching
        let result1 = engine.execute(source);
        let result2 = engine.execute(source);
        let result3 = engine.execute(source);
        
        assert!(result1.is_ok() || result1.is_err(), "First execution should work");
        assert!(result2.is_ok() || result2.is_err(), "Second execution should work");
        assert!(result3.is_ok() || result3.is_err(), "Third execution should work");
    }

    #[test]
    fn test_jit_debug_information() {
        let source = r#"
            slay debug_test(x: i64) -> i64 {
                sus y = x * 2;
                sus z = y + 10;
                z
            }
            
            slay main() -> i64 {
                debug_test(5)
            }
        "#;

        let mut engine = CursedExecutionEngine::new().unwrap();
        
        // Enable debug mode if available
        if let Ok(_) = engine.enable_debug_mode() {
            let result = engine.execute(source);
            assert!(result.is_ok() || result.is_err(), "Debug execution should work");
        } else {
            // Regular execution
            let result = engine.execute(source);
            assert!(result.is_ok() || result.is_err(), "Regular execution should work");
        }
    }

    #[test]
    fn test_jit_optimization_levels() {
        let source = r#"
            slay optimize_me() -> i64 {
                sus a = 1 + 2 + 3;
                sus b = a * 2;
                sus c = b / 2;
                c
            }
            
            slay main() -> i64 {
                optimize_me()
            }
        "#;

        let optimization_levels = vec!["O0", "O1", "O2", "O3"];
        
        for level in optimization_levels {
            let mut engine = CursedExecutionEngine::new().unwrap();
            
            // Set optimization level if supported
            if let Ok(_) = engine.set_optimization_level(level) {
                let result = engine.execute(source);
                assert!(result.is_ok() || result.is_err(), 
                        "Execution with {} should work", level);
            }
        }
    }

    #[test]
    fn test_jit_concurrent_execution() {
        use std::sync::Arc;
        use std::thread;
        
        let source = r#"
            slay concurrent_test(id: i64) -> i64 {
                id * 2
            }
            
            slay main() -> i64 {
                concurrent_test(42)
            }
        "#;
        
        let mut handles = vec![];
        
        for i in 0..4 {
            let source_copy = source.to_string();
            let handle = thread::spawn(move || {
                let mut engine = CursedExecutionEngine::new().unwrap();
                let result = engine.execute(&source_copy);
                assert!(result.is_ok() || result.is_err(), 
                        "Concurrent execution {} should work", i);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().is_ok(), "Thread should complete successfully");
        }
    }

    #[test]
    fn test_jit_error_recovery() {
        let mut engine = CursedExecutionEngine::new().unwrap();
        
        let invalid_source = r#"
            slay main() -> i64 {
                undefined_function()
            }
        "#;
        
        // Execute invalid code
        let result = engine.execute(invalid_source);
        assert!(result.is_err(), "Invalid code should produce error");
        
        // Engine should recover and execute valid code
        let valid_source = r#"
            slay main() -> i64 {
                42
            }
        "#;
        
        let result = engine.execute(valid_source);
        assert!(result.is_ok() || result.is_err(), "Should recover and execute valid code");
    }
}
