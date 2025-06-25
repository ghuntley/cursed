#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let source = r#"facts x = 42;"#;
        
        let tokens = tokenize(source).expect("Tokenization should succeed");
        
        // Should have: facts, x, =, 42, ;
        assert_eq!(tokens.len(), 5);
        
        assert_eq!(tokens[0].token_type, TokenType::Facts);
        assert_eq!(tokens[0].literal, "facts");
        
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].literal, "x");
        
        assert_eq!(tokens[2].token_type, TokenType::Assign);
        assert_eq!(tokens[2].literal, "=");
        
        assert_eq!(tokens[3].token_type, TokenType::Integer);
        assert_eq!(tokens[3].literal, "42");
        
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[4].literal, ";");
    }
    
    #[test]
    fn test_basic_parsing() {
        let source = r#"facts x = 42;"#;
        
        let program = parse(source).expect("Parsing should succeed");
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::Facts(name, expr) => {
                assert_eq!(name, "x");
                match expr {
                    Expression::Integer(val) => assert_eq!(*val, 42),
                    _ => panic!("Expected integer expression"),
                }
            }
            _ => panic!("Expected facts statement"),
        }
    }
    
    #[test]
    fn test_string_parsing() {
        let source = r#"facts name = "CURSED";"#;
        
        let program = parse(source).expect("Parsing should succeed");
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::Facts(name, expr) => {
                assert_eq!(name, "name");
                match expr {
                    Expression::String(val) => assert_eq!(val, "CURSED"),
                    _ => panic!("Expected string expression"),
                }
            }
            _ => panic!("Expected facts statement"),
        }
    }

    #[test]
    fn test_function_declaration() {
        let source = r#"slay greet(name) { facts x = 1; }"#;
        
        let program = parse(source).expect("Parsing should succeed");
        
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Statement::Slay(name, params, body) => {
                assert_eq!(name, "greet");
                assert_eq!(params.len(), 1);
                assert_eq!(params[0], "name");
                assert_eq!(body.len(), 1);
            }
            _ => panic!("Expected slay statement"),
        }
    }
}
