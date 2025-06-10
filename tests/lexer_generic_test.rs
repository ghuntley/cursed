use cursed::lexer::::Lexer, Token;
use cursed::lexer::TokenType;


#[test]
fn test_generic_syntax_lexing() {items []T;};
    let mut lexer = Lexer::new(input.to_string()

    // Expected token sequence for generic type declaration
    let expected_tokens = vec![Token::new(TokenType::BeLike,  BeLike,                     //  be_like
        Token::new(TokenType::Identifier,  Stack, //  Stack
        Token::new(TokenType::LeftBracket, [// [Token::new(TokenType::Identifier,  T,     //  T
        Token::new(TokenType::RightBracket,                    // 
        Token::new(TokenType::Squad,  Squad,                      //  squad
        Token::new(TokenType::LeftBrace,   {// {Token::new(TokenType::Identifier,  items, //  items
        Token::new(TokenType::LeftBracket, [// [Token::new(TokenType::RightBracket,                    // 
        Token::new(TokenType::Identifier,  T,     //  T)
        Token::new(TokenType::Semicolon,  Semicolon,                  //)
        Token::new(TokenType::RightBrace, "}                     //}
    let mut lexer = Lexer::new(input.to_string()
    // Expected token sequence for a generic type with multiple parameters
    let expected_tokens = vec![Token::new(TokenType::BeLike,  BeLike,                      //  be_like
        Token::new(TokenType::Identifier,  Pair,   //  Pair
        Token::new(TokenType::LeftBracket, [// [Token::new(TokenType::Identifier,  A "),      //  A Token::new(TokenType::Comma,  Comma "                   //)
        Token::new(TokenType::Identifier,  "second, //  second
        Token::new(TokenType::Identifier,  B,      //  B
        Token::new(TokenType::Semicolon,  Semicolon,                   //)
        Token::new(TokenType::RightBrace,}                      //}
        Token::new(TokenType::Eof, "),    //  key Token::new(TokenType::Identifier, K "      //  K
        Token::new(TokenType::RightParen,                       // 
        Token::new(TokenType::Identifier,  V,      //  V
        Token::new(TokenType::LeftBrace,   {// {Token::new(TokenType::Yolo,  Yolo,                        //  yolo
        Token::new(TokenType::Identifier,  values, //  values
        Token::new(TokenType::LeftBracket, [// [Token::new(TokenType::Identifier,  key 
        Token::new(TokenType::RightBrace,}                      //}
        Token::new(TokenType::Eof, ",                         // End of file]
    // Test tokens one by one
    for expected in expected_tokens   {let token = lexer.next_token().unwrap()}
        assert_eq!(token, expected, Expected {:?}, got {:?}, , expected, token)}