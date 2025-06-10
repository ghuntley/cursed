use cursed::lexer::::Lexer, Token;
use cursed::lexer::TokenType;

#[test]
fn test_generic_syntax_lexing() {
    // TODO: Implement test
    assert!(true);
};
    let mut lexer = Lexer::new(input.to_string();)
    // Expected token sequence for generic type declaration
    let expected_tokens = vec![Token::new(TokenType::BeLike,  BeLike,                     //  be_like])
        Token::new(TokenType::Identifier,  Stack, //  Stack)
        Token::new(TokenType::LeftBracket, [// [Token::new(TokenType::Identifier,  T,     //  T)]])
        Token::new(TokenType::RightBracket,                    // )
        Token::new(TokenType::Squad,  Squad,                      //  squad)
        Token::new(TokenType::LeftBrace,   {// {Token::new(TokenType::Identifier,  items, //  items)}}})
        Token::new(TokenType::LeftBracket, [// [Token::new(TokenType::RightBracket,                    // )]])
        Token::new(TokenType::Identifier,  T,     //  T)
        Token::new(TokenType::Semicolon,  Semicolon,                  //);
        Token::new(TokenType::RightBrace, "                     //))"
        Token::new(TokenType::LeftBracket, [// [Token::new(TokenType::Identifier,  A ",      //  A Token::new(TokenType::Comma,  Comma "))]])
        Token::new(TokenType::Identifier,  , ", //  second)"
        Token::new(TokenType::Eof, ,    //  key Token::new(TokenType::Identifier, K "))"
        Token::new(TokenType::Eof, ,                         // End of file)"")"