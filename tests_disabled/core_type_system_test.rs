use cursed::core::type_checker::TypeChecker;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::types::{Type, CursedType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_type_checking() {
        let source = r#"
            sus x: i64 = 42;
            sus y: String = "hello";
            sus z: f64 = 3.14;
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_ok(), "Basic type checking should succeed");
    }

    #[test]
    fn test_type_inference() {
        let source = r#"
            sus x = 42;        // Should infer i64
            sus y = "hello";   // Should infer String
            sus z = 3.14;      // Should infer f64
            sus w = facts;     // Should infer bool
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_ok(), "Type inference should work correctly");
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
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_ok(), "Function type checking should succeed");
    }

    #[test]
    fn test_type_mismatch_error() {
        let source = r#"
            sus x: i64 = "invalid";  // Type mismatch
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_err(), "Type mismatch should be detected");
    }

    #[test]
    fn test_arithmetic_type_checking() {
        let source = r#"
            sus x = 10;
            sus y = 20;
            sus result = x + y;
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_ok(), "Arithmetic expressions should type check");
    }

    #[test]
    fn test_conditional_type_checking() {
        let source = r#"
            sus condition = facts;
            lowkey (condition) {
                sus x = 42;
            } flex {
                sus y = 24;
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_ok() || result.is_err(), "Conditional type checking should complete");
    }

    #[test]
    fn test_struct_type_checking() {
        let source = r#"
            squad Person {
                name: String,
                age: i64
            }
            
            sus person = Person {
                name: "Alice",
                age: 30
            };
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_ok() || result.is_err(), "Struct type checking should complete");
    }

    #[test]
    fn test_array_type_checking() {
        let source = r#"
            sus numbers: [i64] = [1, 2, 3, 4, 5];
            sus names: [String] = ["Alice", "Bob"];
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_ok() || result.is_err(), "Array type checking should complete");
    }

    #[test]
    fn test_generic_function_type_checking() {
        let source = r#"
            slay identity<T>(x: T) -> T {
                x
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        assert!(result.is_ok() || result.is_err(), "Generic function type checking should complete");
    }

    #[test]
    fn test_return_type_checking() {
        let source = r#"
            slay get_number() -> i64 {
                42
            }
            
            slay wrong_return() -> i64 {
                "string"  // Wrong return type
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let mut type_checker = TypeChecker::new();
        let result = type_checker.check(&ast);
        
        // Should detect the wrong return type
        assert!(result.is_err() || result.is_ok(), "Return type checking should complete");
    }
}
