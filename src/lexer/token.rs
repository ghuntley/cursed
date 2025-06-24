use crate::error::Error;
/// Token types for the lexer
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Basic tokens
    Identifier,
    Number,
    String,
    // Keywords
    Function,
    Variable,
    If,
    Else,
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    // Special
    EOF,
    Error,
}
