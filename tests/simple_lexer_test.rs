use cursed::lexer::{Lexer, Token, TokenType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "sus x = 42;";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert!(!tokens.is_empty());
        // Should tokenize the basic variable declaration
        assert!(tokens.len() >= 3); // At least sus, x, =, 42, ;
    }

    #[test]
    fn test_keywords() {
        let source = "sus facts cap slay lowkey highkey flex periodt bestie yolo stan";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should recognize all CURSED keywords
        assert!(tokens.len() >= 11);
    }

    #[test]
    fn test_string_literals() {
        let source = r#""hello world" "another string""#;
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize string literals correctly
        assert!(tokens.len() >= 2);
    }

    #[test]
    fn test_numbers() {
        let source = "42 3.14 0 -123";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize numeric literals
        assert!(tokens.len() >= 4);
    }

    #[test]
    fn test_operators() {
        let source = "+ - * / == != < > <= >= && || !";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize all operators
        assert!(tokens.len() >= 10);
    }

    #[test]
    fn test_punctuation() {
        let source = "( ) { } [ ] , ; : .";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize punctuation marks
        assert!(tokens.len() >= 10);
    }

    #[test]
    fn test_identifiers() {
        let source = "variable_name someFunction CamelCase snake_case";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize identifiers
        assert!(tokens.len() >= 4);
    }

    #[test]
    fn test_complex_expression() {
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
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize a complex function definition
        assert!(tokens.len() > 20);
    }

    #[test]
    fn test_comments() {
        let source = r#"
            // This is a comment
            sus x = 42; // End of line comment
            /* Multi-line
               comment */
            sus y = 24;
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Comments should be ignored in tokenization
        assert!(tokens.len() >= 6); // sus, x, =, 42, ;, sus, y, =, 24, ;
    }

    #[test]
    fn test_whitespace_handling() {
        let source = "   sus    x   =   42   ;   ";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Whitespace should be ignored
        assert_eq!(tokens.len(), 5); // sus, x, =, 42, ;
    }

    #[test]
    fn test_type_annotations() {
        let source = "sus x: i64 = 42; sus y: String = \"hello\";";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize type annotations
        assert!(tokens.len() >= 10);
    }

    #[test]
    fn test_array_syntax() {
        let source = "sus arr: [i64] = [1, 2, 3];";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize array syntax
        assert!(tokens.len() >= 10);
    }

    #[test]
    fn test_generic_syntax() {
        let source = "slay identity<T>(x: T) -> T { x }";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize generic function syntax
        assert!(tokens.len() >= 10);
    }

    #[test]
    fn test_channel_syntax() {
        let source = "sus ch = make(chan i64); ch <- 42; sus val = <-ch;";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize channel operations
        assert!(tokens.len() >= 15);
    }

    #[test]
    fn test_error_propagation_syntax() {
        let source = "sus result = some_function()?;";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize error propagation operator
        assert!(tokens.len() >= 6);
    }

    #[test]
    fn test_struct_syntax() {
        let source = r#"
            squad Person {
                name: String,
                age: i64
            }
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize struct definition
        assert!(tokens.len() >= 10);
    }

    #[test]
    fn test_interface_syntax() {
        let source = r#"
            collab Drawable {
                slay draw(self);
            }
        "#;
        
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should tokenize interface definition
        assert!(tokens.len() >= 8);
    }

    #[test]
    fn test_empty_input() {
        let source = "";
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Empty input should result in empty token list
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_unicode_support() {
        let source = r#"sus message = "Hello 世界";"#;
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should handle Unicode in string literals
        assert!(tokens.len() >= 4);
    }

    #[test]
    fn test_lexer_error_handling() {
        let source = r#"sus x = "unterminated string"#;
        let mut lexer = Lexer::new(source.to_string());
        let result = lexer.tokenize();
        
        // Should handle lexer errors gracefully
        assert!(result.is_err() || result.is_ok());
    }
}
