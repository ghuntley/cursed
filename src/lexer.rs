/// Lexer for CURSED language
use crate::error::{Error, SourceLocation};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Gen Z slang keywords
    Slay,       // function
    Yolo,       // return
    Sus,        // variable declaration (mutable)
    Facts,      // variable declaration (immutable)
    Lowkey,     // if
    Highkey,    // else
    Periodt,    // end/semicolon equivalent
    
    // Identifiers and literals
    Identifier,
    Integer,
    Float,
    String,
    Boolean,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    
    // Special
    Eof,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub location: SourceLocation,
}

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn next_token(&mut self) -> Result<Token, Error> {
        // Skip whitespace
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
                continue;
            }
            break;
        }
        
        if self.position >= self.input.len() {
            return Ok(Token {
                token_type: TokenType::Eof,
                literal: String::new(),
                location: self.current_location(),
            });
        }
        
        let location = self.current_location();
        let ch = self.current_char();
        
        match ch {
            '\n' => {
                self.advance();
                self.line += 1;
                self.column = 1;
                Ok(Token {
                    token_type: TokenType::Newline,
                    literal: "\n".to_string(),
                    location,
                })
            }
            '(' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::LeftParen,
                    literal: "(".to_string(),
                    location,
                })
            }
            ')' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::RightParen,
                    literal: ")".to_string(),
                    location,
                })
            }
            '{' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::LeftBrace,
                    literal: "{".to_string(),
                    location,
                })
            }
            '}' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::RightBrace,
                    literal: "}".to_string(),
                    location,
                })
            }
            '+' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Plus,
                    literal: "+".to_string(),
                    location,
                })
            }
            '-' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Minus,
                    literal: "-".to_string(),
                    location,
                })
            }
            _ if ch.is_alphabetic() => self.read_identifier(location),
            _ if ch.is_numeric() => self.read_number(location),
            _ => Err(Error::Parse(format!("Unexpected character: {}", ch))),
        }
    }
    
    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }
    
    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
            self.column += 1;
        }
    }
    
    fn current_location(&self) -> SourceLocation {
        SourceLocation {
            file: None,
            line: self.line,
            column: self.column,
        }
    }
    
    fn read_identifier(&mut self, start_location: SourceLocation) -> Result<Token, Error> {
        let start_pos = self.position;
        
        while self.position < self.input.len() && 
              (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        
        let literal = self.input[start_pos..self.position].to_string();
        let token_type = match literal.as_str() {
            "slay" => TokenType::Slay,
            "yolo" => TokenType::Yolo,
            "sus" => TokenType::Sus,
            "facts" => TokenType::Facts,
            "lowkey" => TokenType::Lowkey,
            "highkey" => TokenType::Highkey,
            "periodt" => TokenType::Periodt,
            "true" | "false" => TokenType::Boolean,
            _ => TokenType::Identifier,
        };
        
        Ok(Token {
            token_type,
            literal,
            location: start_location,
        })
    }
    
    fn read_number(&mut self, start_location: SourceLocation) -> Result<Token, Error> {
        let start_pos = self.position;
        let mut is_float = false;
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_numeric() {
                self.advance();
            } else if ch == '.' && !is_float {
                is_float = true;
                self.advance();
            } else {
                break;
            }
        }
        
        let literal = self.input[start_pos..self.position].to_string();
        let token_type = if is_float {
            TokenType::Float
        } else {
            TokenType::Integer
        };
        
        Ok(Token {
            token_type,
            literal,
            location: start_location,
        })
    }
}
