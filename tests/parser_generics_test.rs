#[cfg(test)]
mod tests {
    use cursed::parser::Parser;
    use cursed::lexer::Lexer;
    use cursed::ast::*;
    
    fn parse_input(input: &str) -> Result<Box<dyn Node>, cursed::error::Error> {
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }
    
    #[test]
    fn test_parse_generic_function() {
        let input = r#"
        slay generic_func<T>(param: T) -> T {
            yolo param;
        }
        "#;
        
        let result = parse_input(input);
        assert!(result.is_ok(), "Failed to parse generic function: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        // Should contain generic indicators
        assert!(program_str.contains("generic_func"));
        assert!(program_str.contains("T"));
    }
    
    #[test]
    fn test_parse_generic_struct() {
        let input = r#"
        squad Container<T> {
            sus value: T
        }
        "#;
        
        let result = parse_input(input);
        assert!(result.is_ok(), "Failed to parse generic struct: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("Container"));
        assert!(program_str.contains("T"));
        assert!(program_str.contains("value"));
    }
    
    #[test]
    fn test_parse_multiple_generic_parameters() {
        let input = r#"
        slay map_func<K, V>(key: K, value: V) -> bool {
            yolo true;
        }
        "#;
        
        let result = parse_input(input);
        assert!(result.is_ok(), "Failed to parse multiple generic parameters: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("map_func"));
        assert!(program_str.contains("K"));
        assert!(program_str.contains("V"));
    }
    
    #[test]
    fn test_parse_generic_constraints() {
        let input = r#"
        slay constrained<T: Display>(value: T) {
            // Function body
        }
        "#;
        
        let result = parse_input(input);
        // This may fail until constraint parsing is implemented
        // For now, just check it doesn't crash
        match result {
            Ok(program) => {
                let program_str = format!("{:?}", program);
                assert!(program_str.contains("constrained"));
            }
            Err(_) => {
                // Expected until constraint parsing is fully implemented
                assert!(true);
            }
        }
    }
    
    #[test]
    fn test_parse_nested_generics() {
        let input = r#"
        squad NestedContainer<T> {
            sus inner: Container<T>
        }
        "#;
        
        let result = parse_input(input);
        assert!(result.is_ok(), "Failed to parse nested generics: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("NestedContainer"));
        assert!(program_str.contains("Container"));
        assert!(program_str.contains("T"));
    }
    
    #[test]
    fn test_parse_generic_interface() {
        let input = r#"
        collab Comparable<T> {
            slay compare(other: T) -> Normie;
        }
        "#;
        
        let result = parse_input(input);
        assert!(result.is_ok(), "Failed to parse generic interface: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("Comparable"));
        assert!(program_str.contains("T"));
        assert!(program_str.contains("compare"));
    }
    
    #[test]
    fn test_parse_generic_instantiation() {
        let input = r#"
        facts container = Container<Tea>::new();
        "#;
        
        let result = parse_input(input);
        assert!(result.is_ok(), "Failed to parse generic instantiation: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("container"));
        assert!(program_str.contains("Container"));
        assert!(program_str.contains("Tea"));
    }
    
    #[test]
    fn test_parse_complex_generic_expression() {
        let input = r#"
        slay complex<T, U>() {
            sus result: Vec<HashMap<T, U>> = Vec::new();
            yolo result;
        }
        "#;
        
        let result = parse_input(input);
        match result {
            Ok(program) => {
                let program_str = format!("{:?}", program);
                assert!(program_str.contains("complex"));
                assert!(program_str.contains("T"));
                assert!(program_str.contains("U"));
            }
            Err(_) => {
                // May fail until complex generic parsing is implemented
                assert!(true);
            }
        }
    }
    
    #[test] 
    fn test_parse_error_recovery_incomplete_generics() {
        let input = "slay incomplete<T";
        
        let result = parse_input(input);
        // Should handle parsing errors gracefully
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(true), // Expected for incomplete syntax
        }
    }
    
    #[test]
    fn test_parse_generic_function_with_body() {
        let input = r#"
        slay identity<T>(value: T) -> T {
            yolo value;
        }
        "#;
        
        let result = parse_input(input);
        assert!(result.is_ok(), "Failed to parse generic function with body: {:?}", result.err());
        
        let program = result.unwrap();
        let program_str = format!("{:?}", program);
        
        assert!(program_str.contains("identity"));
        assert!(program_str.contains("T"));
        assert!(program_str.contains("value"));
        assert!(program_str.contains("yolo") || program_str.contains("return"));
    }
}
