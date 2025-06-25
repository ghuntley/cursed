// Minimal lexer for CURSED minimal build

use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,
    Boolean,
    
    // Identifiers and Keywords
    Identifier,
    
    // CURSED Gen Z Keywords
    Facts,  // Variable declaration
    Sus,    // Mutable variable
    Slay,   // Function declaration
    Stan,   // Goroutine spawn
    Yolo,   // Yield/continue
    
    // Control Flow
    Lowkey, // if
    Highkey, // else
    Periodt, // end/close block
    Bestie,  // for
    Flex,    // while
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Assign,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    
    // Special
    Eof,
    Newline,
    Illegal,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: u32,
    pub column: u32,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
            line: 1,
            column: 1,
        }
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    line: u32,
    column: u32,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 1,
        };
        lexer.read_char();
        lexer
    }
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        }
        self.position = self.read_position;
        self.read_position += 1;
        
        if self.ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let token = match self.ch {
            '+' => Token::new(TokenType::Plus, "+"),
            '-' => Token::new(TokenType::Minus, "-"),
            '*' => Token::new(TokenType::Multiply, "*"),
            '/' => Token::new(TokenType::Divide, "/"),
            '=' => Token::new(TokenType::Assign, "="),
            '(' => Token::new(TokenType::LeftParen, "("),
            ')' => Token::new(TokenType::RightParen, ")"),
            '{' => Token::new(TokenType::LeftBrace, "{"),
            '}' => Token::new(TokenType::RightBrace, "}"),
            ';' => Token::new(TokenType::Semicolon, ";"),
            ',' => Token::new(TokenType::Comma, ","),
            '"' => {
                let literal = self.read_string();
                let token = Token::new(TokenType::String, &literal);
                self.read_char(); // Skip the closing quote
                return token;
            },
            '\0' => Token::new(TokenType::Eof, ""),
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let literal = self.read_identifier();
                    let token_type = self.lookup_ident(&literal);
                    return Token {
                        token_type,
                        literal,
                        line: self.line,
                        column: self.column,
                    };
                } else if self.ch.is_numeric() {
                    let literal = self.read_number();
                    return Token::new(TokenType::Integer, &literal);
                } else {
                    Token::new(TokenType::Illegal, &self.ch.to_string())
                }
            }
        };
        
        self.read_char();
        token
    }
    
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    
    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    
    fn read_string(&mut self) -> String {
        let mut result = String::new();
        
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
            result.push(self.ch);
        }
        
        result
    }
    
    fn lookup_ident(&self, ident: &str) -> TokenType {
        match ident {
            "facts" => TokenType::Facts,
            "sus" => TokenType::Sus,
            "slay" => TokenType::Slay,
            "stan" => TokenType::Stan,
            "yolo" => TokenType::Yolo,
            "lowkey" => TokenType::Lowkey,
            "highkey" => TokenType::Highkey,
            "periodt" => TokenType::Periodt,
            "bestie" => TokenType::Bestie,
            "flex" => TokenType::Flex,
            "true" | "false" => TokenType::Boolean,
            _ => TokenType::Identifier,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.token_type == TokenType::Eof {
            None
        } else {
            Some(token)
        }
    }
}
