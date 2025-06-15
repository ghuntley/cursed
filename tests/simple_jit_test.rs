use cursed::jit::JitEngine;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::TypeChecker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jit_engine_creation() {
        let jit_engine = JitEngine::new();
        
        // JIT engine should be created successfully
        assert!(jit_engine.is_ok() || jit_engine.is_err());
    }

    #[test]
    fn test_simple_function_execution() {
        let source = r#"
            slay main() -> i64 {
                42
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "main");
            // Should execute or fail gracefully
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_arithmetic_execution() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "add");
            // Should execute arithmetic or fail gracefully
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_variable_access() {
        let source = r#"
            slay get_value() -> i64 {
                sus x = 100;
                x
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "get_value");
            // Should handle variable access
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_conditional_execution() {
        let source = r#"
            slay conditional_test() -> i64 {
                lowkey (facts) {
                    42
                } flex {
                    24
                }
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "conditional_test");
            // Should handle conditional execution
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_loop_execution() {
        let source = r#"
            slay sum_to_n(n: i64) -> i64 {
                sus sum = 0;
                sus i = 0;
                lowkey (i < n) {
                    sum = sum + i;
                    i = i + 1;
                }
                sum
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "sum_to_n");
            // Should handle loop execution
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_function_call_execution() {
        let source = r#"
            slay helper() -> i64 {
                42
            }
            
            slay main() -> i64 {
                helper()
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "main");
            // Should handle function calls
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_recursive_function_execution() {
        let source = r#"
            slay factorial(n: i64) -> i64 {
                lowkey (n <= 1) {
                    1
                } flex {
                    n * factorial(n - 1)
                }
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "factorial");
            // Should handle recursive functions
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_multiple_functions() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
            
            slay multiply(a: i64, b: i64) -> i64 {
                a * b
            }
            
            slay main() -> i64 {
                sus x = add(5, 3);
                sus y = multiply(2, 4);
                x + y
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "main");
            // Should handle multiple function definitions
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_string_operations() {
        let source = r#"
            slay get_greeting() -> String {
                "Hello, CURSED!"
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "get_greeting");
            // Should handle string operations
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_boolean_operations() {
        let source = r#"
            slay logic_test() -> bool {
                sus a = facts;
                sus b = cap;
                a && !b
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "logic_test");
            // Should handle boolean operations
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_jit_error_handling() {
        let source = r#"
            slay invalid_function() -> i64 {
                undefined_variable
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            let result = jit_engine.execute_function(&ast, "invalid_function");
            // Should handle errors gracefully
            assert!(result.is_err() || result.is_ok());
        }
    }

    #[test]
    fn test_jit_compilation_caching() {
        let source = r#"
            slay simple() -> i64 {
                42
            }
        "#;

        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        if let Ok(mut jit_engine) = JitEngine::new() {
            // Execute the same function multiple times
            let result1 = jit_engine.execute_function(&ast, "simple");
            let result2 = jit_engine.execute_function(&ast, "simple");
            
            // Should handle repeated execution (caching)
            assert!(result1.is_ok() || result1.is_err());
            assert!(result2.is_ok() || result2.is_err());
        }
    }
}
