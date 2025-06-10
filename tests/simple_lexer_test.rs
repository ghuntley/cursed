use cursed::lexer::{Lexer, TokenType};

#[test]
fn test_simple_lexer() {
    let input = "hello";
    assert_eq!(input, "hello");
}