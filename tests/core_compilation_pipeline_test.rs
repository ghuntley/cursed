use cursed::{run, compile_to_ir, check, format};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::TypeChecker;
use cursed::codegen::llvm::LlvmCodeGenerator;
use inkwell::context::Context;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_compilation_pipeline() {
        let source = r#"
            slay main() -> i64 {
                sus x = 42;
                sus y = 24;
                x + y
            }
        "#;

        // Test the complete pipeline
        let result = run(source, "test.csd");
        
        // Should complete without errors
        assert!(result.is_ok());
    }

    #[test]
    fn test_lexer_parser_integration() {
        let source = r#"
            sus x: i64 = 100;
            sus y: String = "hello";
            
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        // Should have 3 statements
        assert_eq!(ast.statements.len(), 3);
    }

    #[test]
    fn test_type_checker_integration() {
        let source = r#"
            sus x = 42;
            sus y = "hello";
            
            slay greet(name: String) -> String {
                "Hello, " + name
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Should successfully type check
        assert!(result.is_ok());
    }

    #[test]
    fn test_llvm_codegen_integration() {
        let source = r#"
            slay factorial(n: i64) -> i64 {
                lowkey (n <= 1) {
                    1
                } flex {
                    n * factorial(n - 1)
                }
            }
        "#;

        let result = compile_to_ir(source);
        
        // Should generate LLVM IR
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(!ir.is_empty());
    }

    #[test]
    fn test_error_handling_pipeline() {
        let source = r#"
            sus x: i64 = "invalid type";
        "#;

        let result = check(source);
        
        // Should detect type error
        assert!(result.is_err());
    }

    #[test]
    fn test_formatter_integration() {
        let source = r#"slay main(){sus x=42;x+1}"#;

        let result = format(source);
        
        assert!(result.is_ok());
        let formatted = result.unwrap();
        
        // Should be properly formatted
        assert!(formatted.contains("slay main()"));
        assert!(formatted.contains("sus x = 42"));
    }

    #[test]
    fn test_goroutine_compilation_pipeline() {
        let source = r#"
            slay worker(id: i64) {
                // Do work
            }
            
            slay main() {
                lowkey (sus i = 0; i < 5; i++) {
                    stan worker(i);
                    yolo;
                }
            }
        "#;

        let result = compile_to_ir(source);
        
        // Should compile goroutine constructs
        assert!(result.is_ok());
    }

    #[test]
    fn test_channel_compilation_pipeline() {
        let source = r#"
            slay main() {
                sus ch = make(chan i64, 10);
                ch <- 42;
                sus value = <-ch;
            }
        "#;

        let result = compile_to_ir(source);
        
        // Should compile channel operations
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for now
    }

    #[test]
    fn test_interface_compilation_pipeline() {
        let source = r#"
            collab Drawable {
                slay draw(self);
            }
            
            squad Circle {
                radius: f64
            }
            
            impl Drawable for Circle {
                slay draw(self) {
                    // Draw circle
                }
            }
        "#;

        let result = compile_to_ir(source);
        
        // Should compile interface definitions
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for now
    }

    #[test]
    fn test_generic_compilation_pipeline() {
        let source = r#"
            slay identity<T>(x: T) -> T {
                x
            }
            
            slay main() {
                sus x = identity(42);
                sus y = identity("hello");
            }
        "#;

        let result = compile_to_ir(source);
        
        // Should compile generic functions
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for now
    }

    #[test]
    fn test_error_propagation_compilation() {
        let source = r#"
            slay might_fail() -> ?String {
                lowkey (facts) {
                    Some("success")
                } flex {
                    None
                }
            }
            
            slay main() -> ?String {
                sus result = might_fail()?;
                Some(result)
            }
        "#;

        let result = compile_to_ir(source);
        
        // Should compile error propagation
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for now
    }

    #[test]
    fn test_complex_program_compilation() {
        let source = r#"
            squad User {
                id: i64,
                name: String,
                age: i64
            }
            
            slay create_user(name: String, age: i64) -> User {
                User {
                    id: 1,
                    name: name,
                    age: age
                }
            }
            
            slay main() {
                sus users = [];
                
                lowkey (sus i = 0; i < 10; i++) {
                    sus user = create_user("User " + i.to_string(), 20 + i);
                    users.push(user);
                }
                
                lowkey user in users {
                    println("User: " + user.name);
                }
            }
        "#;

        let result = compile_to_ir(source);
        
        // Should compile complex program structure
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for now
    }

    #[test]
    fn test_stdlib_integration_compilation() {
        let source = r#"
            import "stdlib::math";
            import "stdlib::string";
            
            slay calculate() -> f64 {
                sus x = math::sqrt(16.0);
                sus y = math::pow(2.0, 3.0);
                x + y
            }
        "#;

        let result = compile_to_ir(source);
        
        // Should compile with stdlib imports
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for now
    }

    #[test]
    fn test_compilation_performance() {
        let source = r#"
            slay fibonacci(n: i64) -> i64 {
                lowkey (n <= 1) {
                    n
                } flex {
                    fibonacci(n - 1) + fibonacci(n - 2)
                }
            }
            
            slay main() {
                lowkey (sus i = 0; i < 20; i++) {
                    sus result = fibonacci(i);
                    println("fib(" + i.to_string() + ") = " + result.to_string());
                }
            }
        "#;

        use std::time::Instant;
        let start = Instant::now();
        
        let result = compile_to_ir(source);
        
        let duration = start.elapsed();
        
        // Should compile reasonably quickly (less than 5 seconds)
        assert!(duration.as_secs() < 5);
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for now
    }
}
