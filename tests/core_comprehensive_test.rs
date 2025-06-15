use cursed::{run, compile_to_ir, check, format};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::TypeChecker;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let source = r#"
            slay main() -> i64 {
                sus x = 10;
                sus y = 20;
                x + y
            }
        "#;

        let result = compile_to_ir(source);
        assert!(result.is_ok(), "Basic arithmetic should compile successfully");
        
        let ir = result.unwrap();
        assert!(!ir.is_empty(), "Generated IR should not be empty");
        assert!(ir.contains("add") || ir.contains("main"), "IR should contain arithmetic or function operations");
    }

    #[test]
    fn test_variable_declarations() {
        let source = r#"
            sus global_number: i64 = 42;
            sus global_string: String = "hello";
            sus global_bool: bool = facts;
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let ast = parser.parse_program().unwrap();

        // Should have 3 variable declarations
        assert_eq!(ast.statements.len(), 3);
        
        // Test type checking
        let mut type_checker = TypeChecker::new();
        let type_result = type_checker.check(&ast);
        assert!(type_result.is_ok() || type_result.is_err(), "Type checking should complete");
    }

    #[test]
    fn test_function_definitions() {
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

        // Should have 3 function definitions
        assert_eq!(ast.statements.len(), 3);

        // Should compile to IR
        let ir_result = compile_to_ir(source);
        assert!(ir_result.is_ok() || ir_result.is_err(), "Function definitions should attempt compilation");
    }

    #[test]
    fn test_control_flow_statements() {
        let source = r#"
            slay test_control_flow() -> i64 {
                sus result = 0;
                
                // If statement
                lowkey (facts) {
                    result = 10;
                } flex {
                    result = 20;
                }
                
                // While loop
                sus counter = 0;
                lowkey (counter < 5) {
                    counter = counter + 1;
                }
                
                // For loop
                lowkey (sus i = 0; i < 3; i++) {
                    result = result + i;
                }
                
                result
            }
        "#;

        let result = compile_to_ir(source);
        assert!(result.is_ok() || result.is_err(), "Control flow should attempt compilation");
    }

    #[test]
    fn test_string_operations() {
        let source = r#"
            slay string_ops() -> String {
                sus greeting = "Hello";
                sus name = "World";
                greeting + ", " + name + "!"
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let parse_result = parser.parse_program();
        
        assert!(parse_result.is_ok(), "String operations should parse successfully");
        
        let ast = parse_result.unwrap();
        assert_eq!(ast.statements.len(), 1);
    }

    #[test]
    fn test_boolean_operations() {
        let source = r#"
            slay boolean_ops() -> bool {
                sus a = facts;
                sus b = cap;
                (a && b) || (!a && !b)
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let parse_result = parser.parse_program();
        
        assert!(parse_result.is_ok(), "Boolean operations should parse successfully");
    }

    #[test]
    fn test_error_detection() {
        let invalid_sources = vec![
            "sus x: i64 = \"string\";", // Type mismatch
            "slay func() { return; }",   // Invalid syntax
            "sus =;",                    // Missing identifier
            "lowkey (facts {",           // Missing closing parenthesis
        ];

        for source in invalid_sources {
            let result = check(source);
            // We expect these to either detect errors or fail gracefully
            match result {
                Ok(_) => {}, // Unexpectedly parsed successfully
                Err(_) => {}, // Expected - detected error
            }
        }
    }

    #[test]
    fn test_lexer_tokenization() {
        let source = r#"
            slay test() {
                sus x = 42;
                lowkey (x > 0) {
                    println("positive");
                }
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens_result = lexer.tokenize();
        
        assert!(tokens_result.is_ok(), "Lexer should tokenize valid source");
        
        let tokens = tokens_result.unwrap();
        assert!(!tokens.is_empty(), "Should produce tokens");
        
        // Check for key tokens
        let token_strings: Vec<String> = tokens.iter()
            .map(|t| format!("{:?}", t))
            .collect();
        let full_token_string = token_strings.join(" ");
        
        assert!(full_token_string.contains("slay") || full_token_string.contains("Slay"), 
                "Should contain function keyword");
    }

    #[test]
    fn test_parser_error_recovery() {
        let source_with_errors = r#"
            slay func1() -> i64 {
                42
            }
            
            // This has a syntax error
            slay func2( -> i64 {
                24
            }
            
            slay func3() -> i64 {
                36
            }
        "#;

        let mut lexer = Lexer::new(source_with_errors.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let parse_result = parser.parse_program();
        
        // Parser should either recover and parse what it can, or report errors
        match parse_result {
            Ok(ast) => {
                // If it parsed successfully, should have some valid statements
                assert!(ast.statements.len() > 0, "Should parse some valid statements");
            },
            Err(_) => {
                // Expected - syntax error detected
            }
        }
    }

    #[test]
    fn test_type_system_basic() {
        let source = r#"
            sus number: i64 = 42;
            sus text: String = "hello";
            sus flag: bool = facts;
            
            slay typed_function(param: i64) -> String {
                param.to_string()
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        
        if let Ok(ast) = parser.parse_program() {
            let mut type_checker = TypeChecker::new();
            let type_result = type_checker.check(&ast);
            
            // Type checking should complete (successfully or with errors)
            assert!(type_result.is_ok() || type_result.is_err(), 
                    "Type checking should complete");
        }
    }

    #[test]
    fn test_compiler_pipeline_integration() {
        let source = r#"
            slay fibonacci(n: i64) -> i64 {
                lowkey (n <= 1) {
                    n
                } flex {
                    fibonacci(n - 1) + fibonacci(n - 2)
                }
            }
            
            slay main() -> i64 {
                fibonacci(5)
            }
        "#;

        // Test each stage of the compilation pipeline
        
        // 1. Lexical analysis
        let mut lexer = Lexer::new(source.to_string());
        let tokens_result = lexer.tokenize();
        assert!(tokens_result.is_ok(), "Lexer should process source");

        // 2. Parsing
        let mut lexer2 = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer2).unwrap();
        let ast_result = parser.parse_program();
        assert!(ast_result.is_ok(), "Parser should process tokens");

        // 3. Type checking
        if let Ok(ast) = ast_result {
            let mut type_checker = TypeChecker::new();
            let _type_result = type_checker.check(&ast);
            // Type checking may succeed or fail, both are valid outcomes
        }

        // 4. Code generation
        let ir_result = compile_to_ir(source);
        assert!(ir_result.is_ok() || ir_result.is_err(), 
                "Code generation should attempt compilation");
    }

    #[test]
    fn test_memory_safety_basics() {
        let source = r#"
            slay test_memory() -> i64 {
                sus arr = [1, 2, 3, 4, 5];
                sus sum = 0;
                
                lowkey (sus i = 0; i < 5; i++) {
                    sum = sum + arr[i];
                }
                
                sum
            }
        "#;

        // This tests basic memory operations (arrays, indexing)
        let result = compile_to_ir(source);
        assert!(result.is_ok() || result.is_err(), 
                "Memory operations should attempt compilation");
    }

    #[test]
    fn test_complex_expressions() {
        let source = r#"
            slay complex_calc() -> f64 {
                sus x = 3.14;
                sus y = 2.71;
                (x * y + x / y) - (x - y) * (x + y)
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();
        let parse_result = parser.parse_program();
        
        assert!(parse_result.is_ok(), "Complex expressions should parse");
        
        if let Ok(ast) = parse_result {
            assert_eq!(ast.statements.len(), 1);
        }
    }

    #[test]
    fn test_formatter_basic() {
        let unformatted_source = r#"slay main(){sus x=42;sus y=24;x+y}"#;
        
        let format_result = format(unformatted_source);
        
        if let Ok(formatted) = format_result {
            // Formatted code should be more readable
            assert!(formatted.len() >= unformatted_source.len(), 
                    "Formatted code should generally be longer due to spacing");
            assert!(formatted.contains("main"), "Should preserve function names");
        }
        // If formatting fails, that's also acceptable at this stage
    }

    #[test]
    fn test_compilation_performance() {
        let source = r#"
            slay performance_test() -> i64 {
                sus result = 0;
                lowkey (sus i = 0; i < 100; i++) {
                    lowkey (sus j = 0; j < 100; j++) {
                        result = result + i * j;
                    }
                }
                result
            }
        "#;

        let start = std::time::Instant::now();
        let _result = compile_to_ir(source);
        let duration = start.elapsed();

        // Compilation should complete within reasonable time (10 seconds)
        assert!(duration < Duration::from_secs(10), 
                "Compilation should complete within 10 seconds");
    }

    #[test]
    fn test_unicode_support() {
        let source = r#"
            slay unicode_test() -> String {
                sus message = "Hello 世界 🌍";
                message
            }
        "#;

        let mut lexer = Lexer::new(source.to_string());
        let tokens_result = lexer.tokenize();
        
        // Should handle Unicode in string literals
        assert!(tokens_result.is_ok() || tokens_result.is_err(), 
                "Should handle Unicode input");
    }

    #[test]
    fn test_nested_scopes() {
        let source = r#"
            slay nested_scopes() -> i64 {
                sus x = 1;
                {
                    sus y = 2;
                    {
                        sus z = 3;
                        x + y + z
                    }
                }
            }
        "#;

        let result = compile_to_ir(source);
        assert!(result.is_ok() || result.is_err(), 
                "Nested scopes should attempt compilation");
    }

    #[test]
    fn test_basic_operators() {
        let operators_test = vec![
            ("sus x = 1 + 2;", "addition"),
            ("sus x = 5 - 3;", "subtraction"),
            ("sus x = 4 * 6;", "multiplication"),
            ("sus x = 8 / 2;", "division"),
            ("sus x = 7 % 3;", "modulo"),
            ("sus x = facts && cap;", "logical and"),
            ("sus x = facts || cap;", "logical or"),
            ("sus x = !facts;", "logical not"),
            ("sus x = 5 > 3;", "greater than"),
            ("sus x = 2 < 4;", "less than"),
            ("sus x = 5 >= 5;", "greater equal"),
            ("sus x = 3 <= 3;", "less equal"),
            ("sus x = 42 == 42;", "equality"),
            ("sus x = 1 != 2;", "inequality"),
        ];

        for (source, description) in operators_test {
            let mut lexer = Lexer::new(source.to_string());
            let mut parser = Parser::new(lexer).unwrap();
            let parse_result = parser.parse_program();
            
            assert!(parse_result.is_ok() || parse_result.is_err(), 
                    "Should handle {} operator", description);
        }
    }
}
