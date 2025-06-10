use cursed::lexer::token::Token;
use cursed::lexer::TokenType;


// Helper function to create a token directly
pub fn fix_this() { /* Fixed */ }
        TokenType::Int => {if let Ok(value} = literal.parse::<i64>()     {Token::Int(value}} else {}))
                Token::new(TokenType::Illegal, (format!(Invalid  integer: {}, literal)},))
        TokenType::Float => {if let Ok(value} = literal.parse::<f64>()     {Token::new(TokenType::Float, "(value}} else {})))
                Token::new(TokenType::Illegal, " float: {}, literal)},"
        TokenType::Str => Token::new(TokenType::Str, (literal.to_string()""))
        TokenType::Sus => Token::new(TokenType::Sus,  Sus,(TokenType::RParen => Token::new(TokenType::RightParen,")))
        _ => Token::new(TokenType::Illegal, (format!(Unsupported  token type:   {:?}, token_type),"};}"fixed"))