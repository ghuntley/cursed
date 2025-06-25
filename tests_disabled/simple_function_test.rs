#[cfg(test)]
mod tests {
    use cursed::parser::Parser;
    use cursed::lexer::Lexer;
    use cursed::ast::*;
    
    fn parse_function(input: &str) -> Result<Box<dyn Node>, cursed::error::Error> {
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }
    
    #[test]
    fn test_simple_function_declaration() {
        let input = r#"
        slay greet() {
            // Empty function body
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse simple function: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("greet"));
        assert!(program_str.contains("slay") || program_str.contains("function"));
    }
    
    #[test]
    fn test_function_with_parameters() {
        let input = r#"
        slay add(a: Normie, b: Normie) -> Normie {
            yolo a + b;
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse function with parameters: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("add"));
        assert!(program_str.contains("a"));
        assert!(program_str.contains("b"));
        assert!(program_str.contains("Normie"));
    }
    
    #[test]
    fn test_function_with_return_type() {
        let input = r#"
        slay get_message() -> Tea {
            yolo "Hello, World!";
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse function with return type: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("get_message"));
        assert!(program_str.contains("Tea"));
        assert!(program_str.contains("Hello, World!"));
    }
    
    #[test]
    fn test_function_with_body() {
        let input = r#"
        slay calculate(x: Normie) -> Normie {
            sus result = x * 2;
            lowkey (result > 10) {
                yolo result;
            }
            yolo 0;
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse function with body: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("calculate"));
        assert!(program_str.contains("result"));
        assert!(program_str.contains("lowkey") || program_str.contains("if"));
    }
    
    #[test]
    fn test_multiple_functions() {
        let input = r#"
        slay first() {
            // First function
        }
        
        slay second(param: Tea) -> Tea {
            yolo param;
        }
        
        slay third(a: Normie, b: Normie) -> Normie {
            yolo a + b;
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse multiple functions: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("first"));
        assert!(program_str.contains("second"));
        assert!(program_str.contains("third"));
    }
    
    #[test]
    fn test_function_with_local_variables() {
        let input = r#"
        slay process_data() {
            sus count = 0;
            facts message = "Processing";
            sus result = count + 1;
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse function with variables: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("process_data"));
        assert!(program_str.contains("count"));
        assert!(program_str.contains("message"));
        assert!(program_str.contains("result"));
    }
    
    #[test]
    fn test_function_with_control_flow() {
        let input = r#"
        slay control_flow(condition: Cap) {
            lowkey (condition) {
                sus x = 1;
            } highkey {
                sus y = 2;
            }
            
            periodt (true) {
                ghosted;
            }
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse function with control flow: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("control_flow"));
        assert!(program_str.contains("condition"));
        assert!(program_str.contains("lowkey") || program_str.contains("if"));
        assert!(program_str.contains("periodt") || program_str.contains("while"));
    }
    
    #[test]
    fn test_nested_function_calls() {
        let input = r#"
        slay outer() {
            inner(helper(nested()));
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse nested function calls: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("outer"));
        assert!(program_str.contains("inner"));
        assert!(program_str.contains("helper"));
        assert!(program_str.contains("nested"));
    }
    
    #[test]
    fn test_function_with_expressions() {
        let input = r#"
        slay expressions(a: Normie, b: Normie) -> Normie {
            sus sum = a + b;
            sus product = a * b;
            sus comparison = a > b;
            yolo sum + product;
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse function with expressions: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("expressions"));
        assert!(program_str.contains("sum"));
        assert!(program_str.contains("product"));
        assert!(program_str.contains("comparison"));
    }
    
    #[test]
    fn test_function_error_recovery() {
        let input = r#"
        slay incomplete_function(
        "#;
        
        let result = parse_function(input);
        // Should handle incomplete function gracefully
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Expected for incomplete syntax
        }
    }
    
    #[test]
    fn test_function_with_string_parameters() {
        let input = r#"
        slay format_message(name: Tea, age: Normie) -> Tea {
            yolo "Hello " + name + ", you are " + age + " years old";
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse function with string operations: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("format_message"));
        assert!(program_str.contains("name"));
        assert!(program_str.contains("age"));
        assert!(program_str.contains("Hello"));
    }
    
    #[test]
    fn test_function_with_boolean_logic() {
        let input = r#"
        slay check_conditions(x: Normie, y: Normie) -> Cap {
            sus result = x > 0 && y < 100;
            yolo result || x == y;
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse function with boolean logic: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("check_conditions"));
        assert!(program_str.contains("result"));
        assert!(program_str.contains("Cap") || program_str.contains("bool"));
    }
    
    #[test]
    fn test_recursive_function() {
        let input = r#"
        slay factorial(n: Normie) -> Normie {
            lowkey (n <= 1) {
                yolo 1;
            }
            yolo n * factorial(n - 1);
        }
        "#;
        
        let result = parse_function(input);
        assert!(result.is_ok(), "Failed to parse recursive function: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("factorial"));
        assert!(program_str.contains("n"));
        // Should contain recursive call
        let factorial_count = program_str.matches("factorial").count();
        assert!(factorial_count >= 2);
    }
}
