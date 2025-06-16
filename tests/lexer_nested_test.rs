#[cfg(test)]
mod tests {
    use cursed::lexer::{Lexer, TokenType, Token};
    
    #[test]
    fn test_nested_structures() {
        let mut lexer = Lexer::new(r#"
        squad Person {
            sus name: Tea,
            sus age: Normie,
            sus address: squad {
                sus street: Tea,
                sus city: Tea,
                sus nested: squad {
                    sus deep: Normie
                }
            }
        }
        "#.to_string());
        
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        // Check for squad keyword
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Squad));
        // Check for identifier
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "Person"));
        // Check for nested braces
        let left_braces = tokens.iter().filter(|t| t.token_type == TokenType::LeftBrace).count();
        let right_braces = tokens.iter().filter(|t| t.token_type == TokenType::RightBrace).count();
        assert_eq!(left_braces, right_braces);
        assert!(left_braces >= 3); // At least 3 levels of nesting
    }
    
    #[test]
    fn test_nested_function_calls() {
        let mut lexer = Lexer::new("function(nested(inner(deep())))".to_string());
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        let left_parens = tokens.iter().filter(|t| t.token_type == TokenType::LeftParen).count();
        let right_parens = tokens.iter().filter(|t| t.token_type == TokenType::RightParen).count();
        assert_eq!(left_parens, right_parens);
        assert!(left_parens >= 4); // At least 4 levels of function calls
    }
    
    #[test]
    fn test_nested_comments() {
        let mut lexer = Lexer::new(r#"
        // Outer comment
        slay test() {
            // Inner comment
            sus x = 1; // End line comment
        }
        "#.to_string());
        
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        let comments = tokens.iter().filter(|t| t.token_type == TokenType::Comment).count();
        assert!(comments >= 2); // Should find multiple comments
    }
    
    #[test]
    fn test_nested_string_literals() {
        let mut lexer = Lexer::new(r#"sus message = "Outer \"nested \\\"deep\\\" nested\" string""#.to_string());
        let tokens = lexer.tokenize().expect("Failed to tokenize");
        
        // Should handle escaped quotes properly
        assert!(tokens.iter().any(|t| t.token_type == TokenType::String));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Sus));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "message"));
    }
    
    #[test]
    fn test_error_recovery_in_nested_structure() {
        let mut lexer = Lexer::new("{ { { unclosed".to_string());
        let tokens = lexer.tokenize().expect("Should tokenize even with unclosed braces");
        
        // Should tokenize what it can
        assert!(tokens.iter().any(|t| t.token_type == TokenType::LeftBrace));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Identifier && t.literal == "unclosed"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Eof));
    }
}
