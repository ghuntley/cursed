use cursed::lexer::{Lexer, TokenType};

#[test]
fn test_simple_lexer() {
    let input = "42";
    let mut lexer = Lexer::new(input.to_string());
    
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token_type, TokenType::Integer);
    assert_eq!(token.literal, "42");
}

#[test]
fn test_identifier_lexing() {
    let input = "hello";
    let mut lexer = Lexer::new(input.to_string());
    
    let token = lexer.next_token().unwrap();
    assert_eq!(token.token_type, TokenType::Identifier);
    assert_eq!(token.literal, "hello");
}
