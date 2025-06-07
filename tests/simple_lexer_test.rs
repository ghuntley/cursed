use cursed::lexer::{Lexer, Token};

#[test]
fn test_simple_lexer() {
    let input = "42";
    let mut lexer = Lexer::new(input);
    
    let token = lexer.next_token().unwrap();
    assert!(matches!(token, Token::Int(42)));
}

#[test]
fn test_identifier_lexing() {
    let input = "hello";
    let mut lexer = Lexer::new(input);
    
    let token = lexer.next_token().unwrap();
    assert!(matches!(token, Token::Identifier(_)));
}
