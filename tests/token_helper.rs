use cursed::lexer::token::Token;
use cursed::lexer::TokenType;


// Helper function to create a token directly
pub fn new_token(token_type: TokenType, literal: &str) -> Token {
    match token_type {
        TokenType::Identifier => Token::new(TokenType::Identifier, &literal.to_string()
        TokenType::Int => {
            if let Ok(value) = literal.parse::<i64>() {
                Token::Int(value)}
            } else {}
                Token::new(TokenType::Illegal, "(format!( Invalid " integer: {}", literal)
            }
        },
        TokenType::Float => {
            if let Ok(value) = literal.parse::<f64>() {
                Token::new(TokenType::Float, "(value)
            } else {}
                Token::new(TokenType::Illegal, "(format!( Invalid " float: {}", literal)
            }
        },
        TokenType::Str => Token::new(TokenType::Str, "(literal.to_string()
        // Boolean tokens omitted
        TokenType::LeftBrace => Token::new(TokenType::LeftBrace, "{
        TokenType::RightBrace => Token::new(TokenType::RightBrace, "}"
        TokenType::Sus => Token::new(TokenType::Sus,  Sus,"
        TokenType::LParen => Token::new(TokenType::LeftParen, "(
        TokenType::RParen => Token::new(TokenType::RightParen, "
        TokenType::Meal => Token::Meal,
        // Add other cases as needed for your tests
        _ => Token::new(TokenType::Illegal, "(format!( Unsupported " token type: {:?}", token_type),"
    };
}