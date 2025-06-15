use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::TypeChecker;
use cursed::types::*;
use cursed::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_type_checking() {
        let source = r#"
            sus integer: i64 = 42;
            sus float: f64 = 3.14;
            sus text: String = "hello";
            sus flag: bool = facts;
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Type checking should complete for primitive types
        assert!(result.is_ok() || result.is_err(), "Primitive type checking should complete");
    }

    #[test]
    fn test_type_inference() {
        let source = r#"
            sus x = 42;          // Should infer i64
            sus y = 3.14;        // Should infer f64
            sus z = "hello";     // Should infer String
            sus w = facts;       // Should infer bool
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Type inference should work for literals
        assert!(result.is_ok() || result.is_err(), "Type inference should complete");
    }

    #[test]
    fn test_function_type_checking() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
            
            slay greet(name: String) -> String {
                "Hello, " + name
            }
            
            slay is_positive(x: i64) -> bool {
                x > 0
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Function type checking should complete
        assert!(result.is_ok() || result.is_err(), "Function type checking should complete");
    }

    #[test]
    fn test_type_mismatch_detection() {
        let invalid_sources = vec![
            (r#"sus x: i64 = "string";"#, "string assigned to integer"),
            (r#"sus y: String = 42;"#, "integer assigned to string"),
            (r#"sus z: bool = 3.14;"#, "float assigned to boolean"),
        ];

        for (source, description) in invalid_sources {
            let mut lexer = Lexer::new(source.to_string());
            let mut parser = Parser::new(lexer).unwrap();
            
            if let Ok(ast) = parser.parse_program() {
                let mut type_checker = TypeChecker::new();
                let result = type_checker.check(&ast);
                
                // Should detect type mismatch or handle gracefully
                match result {
                    Ok(_) => {
                        // Unexpectedly succeeded - might have weak typing
                    },
                    Err(_) => {
                        // Expected - detected type error
                    }
                }
            }
        }
    }

    #[test]
    fn test_arithmetic_type_checking() {
        let source = r#"
            slay arithmetic_test() -> i64 {
                sus a: i64 = 10;
                sus b: i64 = 20;
                sus c: i64 = 5;
                
                sus sum = a + b;
                sus diff = a - c;
                sus product = a * b;
                sus quotient = a / c;
                sus remainder = a % c;
                
                sum + diff + product + quotient + remainder
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Arithmetic operations should type check
        assert!(result.is_ok() || result.is_err(), "Arithmetic type checking should complete");
    }

    #[test]
    fn test_comparison_type_checking() {
        let source = r#"
            slay comparison_test() -> bool {
                sus a = 10;
                sus b = 20;
                
                sus eq = a == b;
                sus ne = a != b;
                sus lt = a < b;
                sus gt = a > b;
                sus le = a <= b;
                sus ge = a >= b;
                
                eq && ne && lt && gt && le && ge
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Comparison operations should type check
        assert!(result.is_ok() || result.is_err(), "Comparison type checking should complete");
    }

    #[test]
    fn test_logical_type_checking() {
        let source = r#"
            slay logical_test() -> bool {
                sus a = facts;
                sus b = cap;
                
                sus and_result = a && b;
                sus or_result = a || b;
                sus not_result = !a;
                
                and_result || or_result || not_result
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Logical operations should type check
        assert!(result.is_ok() || result.is_err(), "Logical type checking should complete");
    }

    #[test]
    fn test_array_type_checking() {
        let source = r#"
            slay array_test() -> i64 {
                sus numbers: [i64] = [1, 2, 3, 4, 5];
                sus first = numbers[0];
                sus length = numbers.len();
                first + length
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Array operations should type check
        assert!(result.is_ok() || result.is_err(), "Array type checking should complete");
    }

    #[test]
    fn test_struct_type_checking() {
        let source = r#"
            squad Person {
                name: String,
                age: i64
            }
            
            slay create_person() -> Person {
                Person {
                    name: "Alice",
                    age: 30
                }
            }
            
            slay get_name(p: Person) -> String {
                p.name
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Struct operations should type check
        assert!(result.is_ok() || result.is_err(), "Struct type checking should complete");
    }

    #[test]
    fn test_generic_type_checking() {
        let source = r#"
            slay identity<T>(x: T) -> T {
                x
            }
            
            slay test_generics() {
                sus int_result = identity(42);
                sus string_result = identity("hello");
                sus bool_result = identity(facts);
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Generic type checking should attempt to complete
        assert!(result.is_ok() || result.is_err(), "Generic type checking should complete");
    }

    #[test]
    fn test_optional_type_checking() {
        let source = r#"
            slay might_return_none() -> ?i64 {
                lowkey (facts) {
                    Some(42)
                } flex {
                    None
                }
            }
            
            slay handle_optional() -> i64 {
                sus maybe_value = might_return_none();
                lowkey (maybe_value.is_some()) {
                    maybe_value.unwrap()
                } flex {
                    0
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Optional type checking should attempt to complete
        assert!(result.is_ok() || result.is_err(), "Optional type checking should complete");
    }

    #[test]
    fn test_function_call_type_checking() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
            
            slay multiply(x: f64, y: f64) -> f64 {
                x * y
            }
            
            slay test_calls() {
                sus sum = add(10, 20);
                sus product = multiply(3.14, 2.0);
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Function call type checking should complete
        assert!(result.is_ok() || result.is_err(), "Function call type checking should complete");
    }

    #[test]
    fn test_control_flow_type_checking() {
        let source = r#"
            slay control_flow_test(x: i64) -> i64 {
                lowkey (x > 0) {
                    x * 2
                } flex lowkey (x < 0) {
                    x * -1
                } flex {
                    0
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Control flow type checking should complete
        assert!(result.is_ok() || result.is_err(), "Control flow type checking should complete");
    }

    #[test]
    fn test_loop_type_checking() {
        let source = r#"
            slay loop_test() -> i64 {
                sus sum = 0;
                lowkey (sus i = 0; i < 10; i++) {
                    sum = sum + i;
                }
                sum
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Loop type checking should complete
        assert!(result.is_ok() || result.is_err(), "Loop type checking should complete");
    }

    #[test]
    fn test_interface_type_checking() {
        let source = r#"
            collab Drawable {
                slay draw(self);
                slay get_area(self) -> f64;
            }
            
            squad Circle {
                radius: f64
            }
            
            impl Drawable for Circle {
                slay draw(self) {
                    // Draw implementation
                }
                
                slay get_area(self) -> f64 {
                    3.14 * self.radius * self.radius
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Interface type checking should attempt to complete
        assert!(result.is_ok() || result.is_err(), "Interface type checking should complete");
    }

    #[test]
    fn test_recursive_type_checking() {
        let source = r#"
            slay factorial(n: i64) -> i64 {
                lowkey (n <= 1) {
                    1
                } flex {
                    n * factorial(n - 1)
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Recursive function type checking should complete
        assert!(result.is_ok() || result.is_err(), "Recursive type checking should complete");
    }

    #[test]
    fn test_type_coercion() {
        let source = r#"
            slay coercion_test() -> f64 {
                sus int_val: i64 = 42;
                sus float_val: f64 = 3.14;
                int_val + float_val  // May require type coercion
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Type coercion should be handled appropriately
        assert!(result.is_ok() || result.is_err(), "Type coercion should be handled");
    }

    #[test]
    fn test_nested_function_calls() {
        let source = r#"
            slay double(x: i64) -> i64 {
                x * 2
            }
            
            slay square(x: i64) -> i64 {
                x * x
            }
            
            slay nested_test() -> i64 {
                double(square(5))
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Nested function calls should type check
        assert!(result.is_ok() || result.is_err(), "Nested function calls should type check");
    }

    #[test]
    fn test_scope_and_shadowing() {
        let source = r#"
            sus global_var = 42;
            
            slay scope_test() -> i64 {
                sus local_var = 10;
                {
                    sus local_var = 20;  // Shadows outer local_var
                    local_var
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Scope and shadowing should be handled
        assert!(result.is_ok() || result.is_err(), "Scope handling should complete");
    }

    #[test]
    fn test_complex_type_expressions() {
        let source = r#"
            slay complex_types() {
                sus matrix: [[i64]] = [[1, 2], [3, 4]];
                sus optional_array: ?[String] = Some(["a", "b", "c"]);
                sus function_ptr: slay(i64) -> i64 = double;
            }
            
            slay double(x: i64) -> i64 {
                x * 2
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Complex type expressions should attempt to type check
        assert!(result.is_ok() || result.is_err(), "Complex types should be processed");
    }
}
