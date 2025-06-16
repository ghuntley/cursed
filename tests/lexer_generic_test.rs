#[cfg(test)]
mod tests {
    use cursed::lexer::{Lexer, TokenType, Token};
    
    #[test]
    fn test_generic_function_declaration() {
        let mut lexer = Lexer::new("slay generic_func<T>() -> T".to_string());
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        // Check for generic syntax
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Slay));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "generic_func"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::LessThan));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "T"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::GreaterThan));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Arrow));
    }
    
    #[test]
    fn test_generic_struct_declaration() {
        let mut lexer = Lexer::new(r#"
        squad Container<T> {
            sus value: T
        }
        "#.to_string());
        
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Squad));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "Container"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::LessThan));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "T"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::GreaterThan));
    }
    
    #[test]
    fn test_multiple_generic_parameters() {
        let mut lexer = Lexer::new("slay map_func<K, V>(key: K, value: V)".to_string());
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        // Count angle brackets
        let left_angles = tokens.iter().filter(|t| t.token_type == TokenType::LessThan).count();
        let right_angles = tokens.iter().filter(|t| t.token_type == TokenType::GreaterThan).count();
        assert_eq!(left_angles, right_angles);
        assert_eq!(left_angles, 1);
        
        // Check for generic parameters
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "K"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "V"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Comma));
    }
    
    #[test]
    fn test_nested_generics() {
        let mut lexer = Lexer::new("Container<Vec<T>>".to_string());
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        // Should handle nested angle brackets
        let left_angles = tokens.iter().filter(|t| t.token_type == TokenType::LessThan).count();
        let right_angles = tokens.iter().filter(|t| t.token_type == TokenType::GreaterThan).count();
        assert_eq!(left_angles, right_angles);
        assert_eq!(left_angles, 2); // Two levels of nesting
    }
    
    #[test]
    fn test_generic_constraints() {
        let mut lexer = Lexer::new("slay constrained<T: Display + Clone>()".to_string());
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Colon));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Plus));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "Display"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "Clone"));
    }
    
    #[test]
    fn test_generic_instantiation() {
        let mut lexer = Lexer::new("facts container = Container::<String>::new()".to_string());
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        // Check for turbofish operator
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Colon));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::LessThan));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "String"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::GreaterThan));
    }
    
    #[test]
    fn test_generic_edge_cases() {
        // Test right shift that looks like closing generics
        let mut lexer = Lexer::new("Container<Container<T>> result".to_string());
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        // Should properly parse >> as two separate > tokens in generic context
        let right_angles = tokens.iter().filter(|t| t.token_type == TokenType::GreaterThan).count();
        assert_eq!(right_angles, 2);
    }
    
    #[test]
    fn test_generic_error_recovery() {
        let mut lexer = Lexer::new("Container<T".to_string());
        let tokens = lexer.tokenize().expect("Should tokenize incomplete generics");
        
        // Should handle unclosed angle brackets gracefully
        assert!(tokens.iter().any(|t| t.token_type == TokenType::LessThan));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "T"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Eof));
    }
}
