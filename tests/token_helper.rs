use cursed::lexer::token::Token;
use cursed::lexer::TokenType;


// Helper function to create a token directly
pub fn new_token(token_type: TokenType, literal: &str) -> Token {
    match token_type {
        TokenType::Identifier => Token::Identifier(literal.to_string()),
        TokenType::Int => {
            if let Ok(value) = literal.parse::<i64>() {
                Token::Int(value)
            } else {
                Token::Illegal(format!("Invalid integer: {}", literal))
            }
        },
        TokenType::Float => {
            if let Ok(value) = literal.parse::<f64>() {
                Token::Float(value)
            } else {
                Token::Illegal(format!("Invalid float: {}", literal))
            }
        },
        TokenType::String => Token::String(literal.to_string()),
        // Boolean tokens omitted
        TokenType::LeftBrace => Token::LBrace,
        TokenType::RightBrace => Token::RBrace,
        TokenType::Sus => Token::Sus,
        TokenType::LParen => Token::LParen,
        TokenType::RParen => Token::RParen,
        TokenType::Meal => Token::Meal,
        // Add other cases as needed for your tests
        _ => Token::Illegal(format!("Unsupported token type: {:?}", token_type)),
    }
}