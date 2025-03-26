use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::prelude::StrExt;

/// Token type for the CURSED language
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Special tokens
    Illegal(String),
    Eof,
    
    // Identifiers and literals
    Identifier(String),
    String(String),
    Int(i64),
    Float(f64),
    
    // Operators
    Assign,      // =
    Plus,        // +
    Minus,       // -
    Bang,        // !
    Asterisk,    // *
    Slash,       // /
    Lt,          // <
    Gt,          // >
    Eq,          // ==
    NotEq,       // !=
    LtEq,        // <=
    GtEq,        // >=
    
    // Delimiters
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    Dot,         // .
    
    // Keywords - CURSED uses Gen Z slang for keywords
    Vibe,        // package (vibe)
    Yeet,        // import (yeet)
    Slay,        // func (slay)
    Sus,         // var (sus)
    Facts,       // const (facts)
    Lowkey,      // if (lowkey)
    Highkey,     // else (highkey)
    Bestie,      // for (bestie)
    Periodt,     // while (periodt)
    VibeCheck,   // switch (vibe_check)
    Mood,        // case (mood)
    Basic,       // default (basic)
    Ghosted,     // break (ghosted)
    Simp,        // continue (simp)
    BeLike,      // type (be_like)
    Squad,       // struct (squad)
    Collab,      // interface (collab)
    Tea,         // map (tea)
    Dm,          // chan (dm)
    Stan,        // go (stan)
    Flex,        // range (flex)
    Later,       // defer (later)
    Yolo,        // return (yolo)
    Based,       // true (based)
    Cap,         // nil (cap)
    
    // Comment tokens
    LineComment, // fr fr
    BlockCommentStart, // no cap
    BlockCommentEnd,   // on god
}

/// Lexer for the CURSED language
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the provided input
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
            line: 1,
            column: 1,
        };
        
        // Initialize by reading the first character
        lexer.read_char();
        
        lexer
    }
    
    /// Read the next character from the input
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            let chars: Vec<char> = self.input.chars().collect();
            self.ch = chars.get(self.read_position).copied();
        }
        self.position = self.read_position;
        self.read_position += 1;
        
        // Update line and column for error reporting
        if let Some('\n') = self.ch {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
    
    /// Peek at the next character without advancing
    pub fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            let chars: Vec<char> = self.input.chars().collect();
            chars.get(self.read_position).copied()
        }
    }
    
    /// Skip whitespace characters
    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
    
    /// Get the current source location
    pub fn location(&self) -> SourceLocation {
        SourceLocation::new(self.line, self.column)
    }
    
    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();
        
        let token = match self.ch {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            },
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            },
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            },
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            },
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some(':') => Token::Colon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,
            Some('.') => Token::Dot,
            Some('"') => self.read_string()?,
            Some(c) if Self::is_letter(c) => {
                let identifier = self.read_identifier();
                return Ok(self.lookup_identifier(identifier));
            },
            Some(c) if Self::is_digit(c) => {
                return self.read_number();
            },
            None => Token::Eof,
            _ => {
                let location = self.location();
                let message = format!("Unexpected character: {:?}", self.ch);
                return Err(ErrorReporter::lexer_error(location, &message));
            }
        };
        
        self.read_char();
        Ok(token)
    }
    
    /// Read an identifier
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while let Some(c) = self.ch {
            if Self::is_letter(c) || Self::is_digit(c) || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        
        self.input[position..self.position].to_string()
    }
    
    /// Read a number (integer or float)
    fn read_number(&mut self) -> Result<Token, Error> {
        let position = self.position;
        let mut is_float = false;
        
        while let Some(c) = self.ch {
            if Self::is_digit(c) {
                self.read_char();
            } else if c == '.' && !is_float && self.peek_char().map_or(false, Self::is_digit) {
                is_float = true;
                self.read_char();
            } else {
                break;
            }
        }
        
        let number_str = &self.input[position..self.position];
        
        if is_float {
            match number_str.parse::<f64>() {
                Ok(value) => Ok(Token::Float(value)),
                Err(_) => {
                    let location = self.location();
                    Err(ErrorReporter::lexer_error(location, &format!("Could not parse float: {}", number_str)))
                }
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(value) => Ok(Token::Int(value)),
                Err(_) => {
                    let location = self.location();
                    Err(ErrorReporter::lexer_error(location, &format!("Could not parse integer: {}", number_str)))
                }
            }
        }
    }
    
    /// Read a string literal
    fn read_string(&mut self) -> Result<Token, Error> {
        self.read_char(); // Skip the opening quote
        
        let position = self.position;
        while self.ch != Some('"') && self.ch != None {
            self.read_char();
        }
        
        if self.ch != Some('"') {
            let location = self.location();
            return Err(ErrorReporter::lexer_error(location, "Unterminated string literal"));
        }
        
        Ok(Token::String(self.input[position..self.position].to_string()))
    }
    
    /// Convert an identifier to a token
    fn lookup_identifier(&self, identifier: String) -> Token {
        match identifier.as_str() {
            "vibe" => Token::Vibe,
            "yeet" => Token::Yeet,
            "slay" => Token::Slay,
            "sus" => Token::Sus,
            "facts" => Token::Facts,
            "lowkey" => Token::Lowkey,
            "highkey" => Token::Highkey,
            "bestie" => Token::Bestie,
            "periodt" => Token::Periodt,
            "vibe_check" => Token::VibeCheck,
            "mood" => Token::Mood,
            "basic" => Token::Basic,
            "ghosted" => Token::Ghosted,
            "simp" => Token::Simp,
            "be_like" => Token::BeLike,
            "squad" => Token::Squad,
            "collab" => Token::Collab,
            "tea" => Token::Tea,
            "dm" => Token::Dm,
            "stan" => Token::Stan,
            "flex" => Token::Flex,
            "later" => Token::Later,
            "yolo" => Token::Yolo,
            "based" => Token::Based,
            "cap" => Token::Cap,
            "fr" => {
                // Check for "fr fr" comment
                if self.peek_char() == Some(' ') && 
                   self.read_position + 1 < self.input.len() && 
                   self.input.chars().nth(self.read_position + 1) == Some('f') && 
                   self.read_position + 2 < self.input.len() && 
                   self.input.chars().nth(self.read_position + 2) == Some('r') {
                    Token::LineComment
                } else {
                    Token::Identifier(identifier)
                }
            },
            "no" => {
                // Check for "no cap" block comment start
                if self.peek_char() == Some(' ') && 
                   self.read_position + 1 < self.input.len() && 
                   self.input.chars().nth(self.read_position + 1) == Some('c') && 
                   self.read_position + 2 < self.input.len() && 
                   self.input.chars().nth(self.read_position + 2) == Some('a') && 
                   self.read_position + 3 < self.input.len() && 
                   self.input.chars().nth(self.read_position + 3) == Some('p') {
                    Token::BlockCommentStart
                } else {
                    Token::Identifier(identifier)
                }
            },
            "on" => {
                // Check for "on god" block comment end
                if self.peek_char() == Some(' ') && 
                   self.read_position + 1 < self.input.len() && 
                   self.input.chars().nth(self.read_position + 1) == Some('g') && 
                   self.read_position + 2 < self.input.len() && 
                   self.input.chars().nth(self.read_position + 2) == Some('o') && 
                   self.read_position + 3 < self.input.len() && 
                   self.input.chars().nth(self.read_position + 3) == Some('d') {
                    Token::BlockCommentEnd
                } else {
                    Token::Identifier(identifier)
                }
            },
            _ => Token::Identifier(identifier),
        }
    }
    
    /// Check if a character is a letter
    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }
    
    /// Check if a character is a digit
    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }
}

#[cfg(test)]
mod tests; 