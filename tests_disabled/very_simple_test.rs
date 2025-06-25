use cursed::lexer::Lexer;
use cursed::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        let source = "";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        // Empty source should produce empty token list or EOF token
        match tokens {
            Ok(token_list) => {
                // Should be empty or contain only EOF
                assert!(token_list.len() <= 1, "Empty source should produce minimal tokens");
            },
            Err(_) => {
                // This is also acceptable
            }
        }
    }

    #[test]
    fn test_single_number() {
        let source = "42";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        assert!(tokens.is_ok(), "Single number should tokenize");
        let token_list = tokens.unwrap();
        assert!(!token_list.is_empty(), "Should produce at least one token");
    }

    #[test]
    fn test_single_string() {
        let source = r#""hello""#;
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        assert!(tokens.is_ok(), "Single string should tokenize");
        let token_list = tokens.unwrap();
        assert!(!token_list.is_empty(), "Should produce at least one token");
    }

    #[test]
    fn test_boolean_literals() {
        let sources = vec!["facts", "cap"];
        
        for source in sources {
            let mut lexer = Lexer::new(source.to_string());
            let tokens = lexer.tokenize();
            
            assert!(tokens.is_ok(), "Boolean '{}' should tokenize", source);
            let token_list = tokens.unwrap();
            assert!(!token_list.is_empty(), "Should produce tokens for '{}'", source);
        }
    }

    #[test]
    fn test_basic_operators() {
        let operators = vec!["+", "-", "*", "/", "=", "==", "!=", "<", ">", "<=", ">="];
        
        for op in operators {
            let mut lexer = Lexer::new(op.to_string());
            let tokens = lexer.tokenize();
            
            assert!(tokens.is_ok(), "Operator '{}' should tokenize", op);
        }
    }

    #[test]
    fn test_keywords() {
        let keywords = vec!["sus", "slay", "lowkey", "highkey", "facts", "cap", "periodt"];
        
        for keyword in keywords {
            let mut lexer = Lexer::new(keyword.to_string());
            let tokens = lexer.tokenize();
            
            assert!(tokens.is_ok(), "Keyword '{}' should tokenize", keyword);
            let token_list = tokens.unwrap();
            assert!(!token_list.is_empty(), "Should produce tokens for '{}'", keyword);
        }
    }

    #[test]
    fn test_simple_expression() {
        let source = "1 + 2";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        assert!(tokens.is_ok(), "Simple expression should tokenize");
        let token_list = tokens.unwrap();
        assert!(token_list.len() >= 3, "Should have at least 3 tokens (1, +, 2)");
    }

    #[test]
    fn test_whitespace_handling() {
        let sources = vec![
            "1+2",      // No spaces
            "1 + 2",    // Normal spaces
            "1  +  2",  // Extra spaces
            " 1 + 2 ",  // Leading/trailing spaces
            "1\t+\t2",  // Tabs
            "1\n+\n2",  // Newlines
        ];
        
        for source in sources {
            let mut lexer = Lexer::new(source.to_string());
            let tokens = lexer.tokenize();
            
            assert!(tokens.is_ok(), "Source '{}' should tokenize", source.replace('\n', "\\n").replace('\t', "\\t"));
        }
    }

    #[test]
    fn test_comments() {
        let source = r#"
            // This is a comment
            42 // Another comment
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        
        // Should tokenize successfully, possibly filtering out comments
        assert!(tokens.is_ok(), "Source with comments should tokenize");
    }

    #[test]
    fn test_identifiers() {
        let identifiers = vec!["x", "variable", "camelCase", "snake_case", "x123", "_private"];
        
        for identifier in identifiers {
            let mut lexer = Lexer::new(identifier.to_string());
            let tokens = lexer.tokenize();
            
            assert!(tokens.is_ok(), "Identifier '{}' should tokenize", identifier);
        }
    }
}
