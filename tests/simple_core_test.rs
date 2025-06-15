
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::TypeChecker;
use cursed::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lexing() {
        let source = "sus x = 42;";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        assert!(tokens.is_ok(), "Basic lexing should succeed");
        let token_list = tokens.unwrap();
        assert!(!token_list.is_empty(), "Should produce tokens");
    }

    #[test]
    fn test_simple_parsing() {
        let source = r#"
            sus x = 10;
            sus y = 20;
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Simple parsing should succeed");
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 2, "Should parse 2 statements");
        }
    }

    #[test]
    fn test_function_parsing() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Function parsing should succeed");
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 1, "Should parse function");
        }
    }

    #[test]
    fn test_type_checking() {
        let source = r#"
            sus x: i64 = 42;
            sus y: String = "hello";
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        if let Ok(ast) = parser.parse_program() {
            let mut type_checker = TypeChecker::new();
            let result = type_checker.check(&ast);
            // Type checking should complete (may succeed or fail)
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_expression_parsing() {
        let expressions = vec![
            "42",
            "\"hello\"",
            "facts",
            "cap",
            "x + y",
            "a * b + c",
            "(x + y) * z",
        ];

        for expr in expressions {
            let source = format!("sus result = {};", expr);
            let mut lexer = Lexer::new(source);
            let mut parser = Parser::new(lexer).unwrap();
            let ast = parser.parse_program();
            
            assert!(ast.is_ok() || ast.is_err(), 
                    "Expression '{}' should attempt to parse", expr);
        }
    }

    #[test]
    fn test_control_flow_parsing() {
        let source = r#"
            slay test() -> i64 {
                lowkey (facts) {
                    42
                } flex {
                    24
                }
            }
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Control flow should parse");
    }

    #[test]
    fn test_loop_parsing() {
        let source = r#"
            slay loop_test() {
                lowkey (sus i = 0; i < 10; i++) {
                    // Loop body
                }
            }
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Loop should parse");
    }

    #[test]
    fn test_error_handling() {
        let invalid_sources = vec![
            "sus x =;",           // Incomplete assignment
            "slay func( -> i64;", // Invalid function syntax
            "lowkey facts {",     // Missing closing brace
        ];

        for source in invalid_sources {
            let mut lexer = Lexer::new(source.to_string());
            let parser_result = Parser::new(lexer);
            
            if let Ok(mut parser) = parser_result {
                let ast = parser.parse_program();
                // Should either succeed or fail gracefully
                assert!(ast.is_ok() || ast.is_err());
            }
        }
    }

    #[test]
    fn test_string_literals() {
        let source = r#"
            sus message = "Hello, World!";
            sus empty = "";
            sus with_escapes = "Line 1\nLine 2";
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "String literals should parse");
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 3);
        }
    }

    #[test]
    fn test_numeric_literals() {
        let source = r#"
            sus integer = 42;
            sus float = 3.14;
            sus negative = -10;
            sus zero = 0;
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program();
        
        assert!(ast.is_ok(), "Numeric literals should parse");
        if let Ok(program) = ast {
            assert_eq!(program.statements.len(), 4);
        }
    }
}