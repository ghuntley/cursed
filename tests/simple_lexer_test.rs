use cursed::lexer::{Lexer, TokenType};

#[test]
fn test_simple_lexer() {
    let input = "test";
    let mut lexer = Lexer::new(input.to_string());
    let result = lexer.next_token();
    assert!(result.is_ok());
}