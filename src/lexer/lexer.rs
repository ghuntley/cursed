use crate::error::{Error, ErrorReporter, SourceLocation};

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
    Byte(u8),    // byte literal (single byte value)
    Rune(char),  // rune literal (Unicode code point)
    
    // Operators
    Assign,      // =
    Plus,        // +
    Minus,       // -
    Bang,        // !
    Asterisk,    // *
    Slash,       // /
    Percent,     // %
    Lt,          // <
    Gt,          // >
    Eq,          // ==
    NotEq,       // !=
    LtEq,        // <=
    GtEq,        // >=
    And,         // &&
    Or,          // ||
    Arrow,       // <-
    At,          // @ (for pointers)
    
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
    Crew,        // array literal
    
    // Integer types
    Smol,        // int8 (smol)
    Mid,         // int16 (mid)
    Normie,      // int32 (normie)
    Thicc,       // int64 (thicc)
    
    // Comment tokens
    LineComment, // fr fr
    BlockCommentStart, // no cap
    BlockCommentEnd,   // on god
}

/// Lexer for the CURSED language
pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
    pub line: usize,
    pub column: usize,
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
    
    /// Skip whitespace characters and comments
    pub fn skip_whitespace(&mut self) {
        loop {
            // Skip standard whitespace
            while let Some(ch) = self.ch {
                if ch.is_whitespace() {
                    self.read_char();
                } else {
                    break;
                }
            }
            
            // Check for comments
            if self.ch == Some('f') && self.peek_sequence("fr fr") {
                // Skip 'f', 'r', ' ', 'f', 'r'
                self.read_char(); // f
                self.read_char(); // r
                self.read_char(); // space
                self.read_char(); // f
                self.read_char(); // r
                
                // Skip the rest of the line
                while let Some(ch) = self.ch {
                    if ch == '\n' {
                        self.read_char(); // Consume the newline
                        break; // Stop at the end of the line
                    }
                    self.read_char();
                }
                // Continue the outer loop to check for more whitespace/comments
                continue;
            } else if self.ch == Some('n') && self.peek_sequence("no cap") {
                // Skip 'n', 'o', ' ', 'c', 'a', 'p'
                self.read_char(); // n
                self.read_char(); // o
                self.read_char(); // space
                self.read_char(); // c
                self.read_char(); // a
                self.read_char(); // p
                
                // Skip until "on god"
                loop {
                    match self.ch {
                        Some('o') if self.peek_sequence("on god") => {
                            // Skip 'o', 'n', ' ', 'g', 'o', 'd'
                             self.read_char(); // o
                             self.read_char(); // n
                             self.read_char(); // space
                             self.read_char(); // g
                             self.read_char(); // o
                             self.read_char(); // d
                             break; // End of block comment
                        },
                        None => {
                            // Error: Unterminated block comment - We can't return Error here directly
                            // Mark as illegal state or handle in next_token maybe?
                            // For now, just break to avoid infinite loop on EOF
                             println!("Warning: Unterminated block comment"); // Temporary warning
                             break; 
                        },
                        _ => {
                            self.read_char(); // Consume character inside the comment
                        }
                    }
                }
                // Continue the outer loop to check for more whitespace/comments
                continue;
            }
            
            // If it's not whitespace and not a comment, break the loop
            break;
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
            Some('&') => {
                if self.peek_char() == Some('&') {
                    self.read_char();
                    Token::And
                } else {
                    let location = self.location();
                    let message = "Unexpected single '&', did you mean '&&'?";
                    return Err(ErrorReporter::lexer_error(location, message));
                }
            },
            Some('|') => {
                if self.peek_char() == Some('|') {
                    self.read_char();
                    Token::Or
                } else {
                    let location = self.location();
                    let message = "Unexpected single '|', did you mean '||'?";
                    return Err(ErrorReporter::lexer_error(location, message));
                }
            },
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('%') => Token::Percent,
            Some('@') => Token::At,
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::LtEq
                } else if self.peek_char() == Some('-') {
                    self.read_char();
                    Token::Arrow
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
            Some('\'') => self.read_rune()?,
            Some('b') if self.peek_char() == Some('\'') => {
                self.read_char(); // consume 'b'
                return self.read_byte();
            },
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
        let mut allow_dot = false;  // Initially don't allow dots
        
        while let Some(c) = self.ch {
            if Self::is_letter(c) || Self::is_digit(c) || c == '_' {
                allow_dot = true;  // After a letter, digit, or underscore, a dot can follow
                self.read_char();
            } else if c == '.' && allow_dot {
                // If this is a dot and we've seen a character before,
                // include it only if the next character is a letter
                if self.peek_char().map_or(false, Self::is_letter) {
                    self.read_char();
                    allow_dot = false;  // Only allow one dot at a time
                } else {
                    break;
                }
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
    
    /// Read a rune literal (Unicode code point)
    fn read_rune(&mut self) -> Result<Token, Error> {
        self.read_char(); // Skip the opening single quote
        
        let position = self.position;
        let ch = self.ch;
        
        // Check for escape sequences
        if ch == Some('\\') {
            self.read_char(); // Skip the backslash
            let escape_char = self.ch;
            
            match escape_char {
                Some('n') => {
                    self.read_char(); // Consume 'n'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated rune literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\n'));
                },
                Some('t') => {
                    self.read_char(); // Consume 't'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated rune literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\t'));
                },
                Some('r') => {
                    self.read_char(); // Consume 'r'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated rune literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\r'));
                },
                Some('\'') => {
                    self.read_char(); // Consume '\''
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated rune literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\''));
                },
                Some('\\') => {
                    self.read_char(); // Consume '\\'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated rune literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\\'));
                },
                _ => {
                    let location = self.location();
                    return Err(ErrorReporter::lexer_error(location, &format!("Unknown escape sequence: \\{:?}", escape_char)));
                }
            }
        }
        
        // Regular character
        self.read_char(); // Move past the character
        
        if ch.is_none() {
            let location = self.location();
            return Err(ErrorReporter::lexer_error(location, "Empty rune literal"));
        }
        
        if self.ch != Some('\'') {
            let location = self.location();
            return Err(ErrorReporter::lexer_error(location, "Unterminated rune literal"));
        }
        
        self.read_char(); // Consume closing quote
        Ok(Token::Rune(ch.unwrap()))
    }
    
    /// Read a byte literal
    fn read_byte(&mut self) -> Result<Token, Error> {
        self.read_char(); // Skip the opening single quote
        
        let ch = self.ch;
        
        // Check for escape sequences
        if ch == Some('\\') {
            self.read_char(); // Skip the backslash
            let escape_char = self.ch;
            
            match escape_char {
                Some('n') => {
                    self.read_char(); // Consume 'n'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated byte literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\n'));
                },
                Some('t') => {
                    self.read_char(); // Consume 't'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated byte literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\t'));
                },
                Some('r') => {
                    self.read_char(); // Consume 'r'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated byte literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\r'));
                },
                Some('\'') => {
                    self.read_char(); // Consume '\''
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated byte literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\''));
                },
                Some('\\') => {
                    self.read_char(); // Consume '\\'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(location, "Unterminated byte literal"));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\\'));
                },
                _ => {
                    let location = self.location();
                    return Err(ErrorReporter::lexer_error(location, &format!("Unknown escape sequence: \\{:?}", escape_char)));
                }
            }
        }
        
        // Regular character
        if ch.is_none() {
            let location = self.location();
            return Err(ErrorReporter::lexer_error(location, "Empty byte literal"));
        }
        
        // Check if character is within ASCII range (0-127)
        let ch_val = ch.unwrap() as u32;
        if ch_val > 127 {
            let location = self.location();
            return Err(ErrorReporter::lexer_error(location, &format!("Byte literal must be ASCII (0-127), got: {:?} ({})", ch, ch_val)));
        }
        
        self.read_char(); // Move past the character
        
        if self.ch != Some('\'') {
            let location = self.location();
            return Err(ErrorReporter::lexer_error(location, "Unterminated byte literal"));
        }
        
        self.read_char(); // Consume closing quote
        Ok(Token::Byte(ch.unwrap() as u8))
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
            "crew" => Token::Crew,
            "smol" => Token::Smol,
            "mid" => Token::Mid,
            "normie" => Token::Normie,
            "thicc" => Token::Thicc,
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
    
    /// Check if the current character is a digit
    pub fn is_current_digit(&self) -> bool {
        match self.ch {
            Some(ch) => Self::is_digit(ch),
            None => false,
        }
    }
    
    /// Check if the next characters match a specific sequence without consuming them
    fn peek_sequence(&self, sequence: &str) -> bool {
        self.input[self.position..].starts_with(sequence)
    }
}

impl Token {
    /// Get the literal value of the token as a string
    pub fn token_literal(&self) -> String {
        match self {
            Token::Identifier(s) => s.clone(),
            Token::String(s) => s.clone(),
            Token::Int(i) => i.to_string(),
            Token::Float(f) => f.to_string(),
            Token::Byte(b) => format!("b'{}'" , *b as char),
            Token::Rune(r) => format!("'{}'", *r),
            Token::Illegal(s) => s.clone(),
            // Default literals for non-literal tokens
            Token::Assign => "=".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Bang => "!".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Percent => "%".to_string(),
            Token::Lt => "<".to_string(),
            Token::Gt => ">".to_string(),
            Token::Eq => "==".to_string(),
            Token::NotEq => "!=".to_string(),
            Token::LtEq => "<=".to_string(),
            Token::GtEq => ">=".to_string(),
            Token::And => "&&".to_string(),
            Token::Or => "||".to_string(),
            Token::Arrow => "<-".to_string(),
            Token::At => "@".to_string(),
            Token::Comma => ",".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Colon => ":".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::LBracket => "[".to_string(),
            Token::RBracket => "]".to_string(),
            Token::Dot => ".".to_string(),
            Token::Vibe => "vibe".to_string(),
            Token::Yeet => "yeet".to_string(),
            Token::Slay => "slay".to_string(),
            Token::Sus => "sus".to_string(),
            Token::Facts => "facts".to_string(),
            Token::Lowkey => "lowkey".to_string(),
            Token::Highkey => "highkey".to_string(),
            Token::Bestie => "bestie".to_string(),
            Token::Periodt => "periodt".to_string(),
            Token::VibeCheck => "vibe_check".to_string(),
            Token::Mood => "mood".to_string(),
            Token::Basic => "basic".to_string(),
            Token::Ghosted => "ghosted".to_string(),
            Token::Simp => "simp".to_string(),
            Token::BeLike => "be_like".to_string(),
            Token::Squad => "squad".to_string(),
            Token::Collab => "collab".to_string(),
            Token::Tea => "tea".to_string(),
            Token::Dm => "dm".to_string(),
            Token::Stan => "stan".to_string(),
            Token::Flex => "flex".to_string(),
            Token::Later => "later".to_string(),
            Token::Yolo => "yolo".to_string(),
            Token::Based => "based".to_string(),
            Token::Cap => "cap".to_string(),
            Token::Crew => "crew".to_string(),
            Token::Smol => "smol".to_string(),
            Token::Mid => "mid".to_string(),
            Token::Normie => "normie".to_string(),
            Token::Thicc => "thicc".to_string(),
            Token::LineComment => "fr fr".to_string(),
            Token::BlockCommentStart => "no cap".to_string(),
            Token::BlockCommentEnd => "on god".to_string(),
            Token::Eof => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    // Helper function to tokenize a string and collect tokens
    fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token()?;
            tokens.push(token.clone());
            
            if let Token::Eof = token {
                break;
            }
        }
        
        Ok(tokens)
    }
    
    // Property: Tokenize and then convert back to string should preserve semantics
    proptest! {
        #[test]
        fn prop_symbols_tokenize_correctly(
            // Test individual symbols one at a time instead of sequences
            symbol in prop::sample::select(vec![
                "+", "-", "*", "/", "=", "!", "<", ">", ",", ".", ";", ":", "(", ")", "{", "}", "[", "]"
            ])
        ) {
            let result = tokenize(symbol);
            assert!(result.is_ok(), "Failed to tokenize symbol: {}", symbol);
            
            let tokens = result.unwrap();
            // Should be one token plus EOF
            assert_eq!(tokens.len(), 2, "Expected 2 tokens for symbol '{}', got {} - tokens: {:?}", symbol, tokens.len(), tokens);
            
            // Check that the symbol was tokenized correctly
            let expected = match symbol {
                "+" => Token::Plus,
                "-" => Token::Minus,
                "*" => Token::Asterisk,
                "/" => Token::Slash,
                "=" => Token::Assign,
                "!" => Token::Bang,
                "<" => Token::Lt,
                ">" => Token::Gt,
                "," => Token::Comma,
                "." => Token::Dot,
                ";" => Token::Semicolon,
                ":" => Token::Colon,
                "(" => Token::LParen,
                ")" => Token::RParen,
                "{" => Token::LBrace,
                "}" => Token::RBrace,
                "[" => Token::LBracket,
                "]" => Token::RBracket,
                _ => panic!("Unexpected symbol: {}", symbol),
            };
            
            assert_eq!(tokens[0], expected, "Symbol '{}' was not tokenized correctly", symbol);
        }
        
        #[test]
        fn prop_integer_tokenization(
            // Generate only positive integers to avoid the minus sign issue
            i in 0i64..9999999999i64
        ) {
            let input = i.to_string();
            
            let mut lexer = Lexer::new(&input);
            let mut tokens = Vec::new();
            
            loop {
                let token = lexer.next_token().unwrap();
                tokens.push(token.clone());
                
                if let Token::Eof = token {
                    break;
                }
            }
            
            // Should produce exactly one INT token and one EOF token
            assert_eq!(tokens.len(), 2, "Expected 2 tokens, got {} - tokens: {:?}", tokens.len(), tokens);
            
            match &tokens[0] {
                Token::Int(value) => assert_eq!(*value, i),
                _ => panic!("Expected INT token, got {:?}", tokens[0]),
            }
        }
        
        #[test]
        fn prop_float_tokenization(
            // Generate only positive floats to avoid the minus sign issue
            int_part in 0i64..999i64,
            decimal_part in 0u32..999u32
        ) {
            // Format as a float string like "123.456"
            let input = format!("{}.{}", int_part, decimal_part);
            // Parse it as a f64 to compare with the lexer's output
            let expected = input.parse::<f64>().unwrap();
            
            let mut lexer = Lexer::new(&input);
            let mut tokens = Vec::new();
            
            loop {
                let token = lexer.next_token().unwrap();
                tokens.push(token.clone());
                
                if let Token::Eof = token {
                    break;
                }
            }
            
            // Should produce exactly one FLOAT token and one EOF token
            assert_eq!(tokens.len(), 2, "Expected 2 tokens, got {} - tokens: {:?}", tokens.len(), tokens);
            
            match &tokens[0] {
                Token::Float(value) => {
                    // Use approximate equality for floating point
                    assert!((value - expected).abs() < f64::EPSILON, 
                        "Expected {}, got {}", expected, value);
                }
                _ => panic!("Expected FLOAT token, got {:?}", tokens[0]),
            }
        }
        
        #[test]
        fn prop_identifier_tokenization(
            // Generate random valid identifiers
            id in r"[a-zA-Z_][a-zA-Z0-9_]{0,19}"
        ) {
            // Skip testing keywords
            if ["vibe", "yeet", "slay", "sus", "facts", "lowkey", "highkey", 
                "bestie", "periodt", "vibe_check", "mood", "basic", "ghosted", 
                "simp", "be_like", "squad", "collab", "tea", "dm", "stan", 
                "flex", "later", "yolo", "based", "cap", "fr", "no", "on"]
                .contains(&id.as_str()) {
                return Ok(());
            }
            
            let result = tokenize(&id);
            assert!(result.is_ok(), "Failed to tokenize identifier: {}", id);
            
            let tokens = result.unwrap();
            assert_eq!(tokens.len(), 2, "Expected 2 tokens, got {}", tokens.len());
            
            match &tokens[0] {
                Token::Identifier(name) => assert_eq!(name, &id),
                _ => panic!("Expected IDENTIFIER token, got {:?}", tokens[0]),
            }
        }
        
        #[test]
        fn prop_string_tokenization(
            // Generate a random string content (no quotes inside)
            content in r"[a-zA-Z0-9 ]{0,50}"
        ) {
            let input = format!("\"{}\"", content);
            let result = tokenize(&input);
            
            assert!(result.is_ok(), "Failed to tokenize string: {}", input);
            let tokens = result.unwrap();
            
            assert_eq!(tokens.len(), 2, "Expected 2 tokens, got {}", tokens.len());
            
            match &tokens[0] {
                Token::String(s) => assert_eq!(s, &content),
                _ => panic!("Expected STRING token, got {:?}", tokens[0]),
            }
        }
        
        #[test]
        fn prop_compound_operators(
            // Test patterns with compound operators, one at a time
            op in prop::sample::select(vec![
                "==", "!=", "<=", ">=", "=", "+", "-", "*", "/", "%", "<", ">", "!", "&&", "||"
            ])
        ) {
            let result = tokenize(&op);
            assert!(result.is_ok(), "Failed to tokenize operator: {}", op);
            
            let tokens = result.unwrap();
            
            // The last token should be EOF
            assert_eq!(tokens.last().unwrap(), &Token::Eof);
            
            // Verify the token is correctly identified
            let expected_token = match op {
                "==" => Token::Eq,
                "!=" => Token::NotEq,
                "<=" => Token::LtEq,
                ">=" => Token::GtEq,
                "=" => Token::Assign,
                "+" => Token::Plus,
                "-" => Token::Minus,
                "*" => Token::Asterisk,
                "/" => Token::Slash,
                "%" => Token::Percent,
                "<" => Token::Lt,
                ">" => Token::Gt,
                "!" => Token::Bang,
                "&&" => Token::And,
                "||" => Token::Or,
                _ => panic!("Unexpected operator: {}", op),
            };
            
            assert_eq!(tokens[0], expected_token, "Token mismatch for operator {}", op);
        }
        
        #[test]
        fn prop_keywords_tokenization(
            keyword in prop::sample::select(vec![
                "vibe", "yeet", "slay", "sus", "facts", "lowkey", "highkey", 
                "bestie", "periodt", "vibe_check", "mood", "basic", "ghosted", 
                "simp", "be_like", "squad", "collab", "tea", "dm", "stan", 
                "flex", "later", "yolo", "based", "cap"
            ])
        ) {
            let result = tokenize(keyword);
            assert!(result.is_ok(), "Failed to tokenize keyword: {}", keyword);
            
            let tokens = result.unwrap();
            assert_eq!(tokens.len(), 2, "Expected 2 tokens, got {}", tokens.len());
            
            let expected_token = match keyword {
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
                _ => panic!("Unexpected keyword: {}", keyword),
            };
            
            assert_eq!(tokens[0], expected_token);
        }
        
        #[test]
        fn prop_comments_tokenization(
            // Test line comments and block comments
            has_line_comment in proptest::bool::ANY,
            has_block_comment in proptest::bool::ANY,
            content in r"[a-zA-Z0-9 ]{0,20}"
        ) {
            let mut input = String::new();
            let mut expected_tokens = Vec::new();
            
            if has_line_comment {
                input.push_str("fr fr ");
                input.push_str(&content);
                expected_tokens.push(Token::LineComment);
            }
            
            if has_block_comment {
                if !input.is_empty() {
                    input.push_str("\n");
                }
                input.push_str("no cap ");
                input.push_str(&content);
                input.push_str(" on god");
                expected_tokens.push(Token::BlockCommentStart);
                expected_tokens.push(Token::BlockCommentEnd);
            }
            
            // If both are false, just add some content
            if !has_line_comment && !has_block_comment {
                input.push_str(&content);
                if !content.is_empty() {
                    // This will be tokenized as identifiers or other tokens 
                    // depending on the content, so we don't predict specific tokens
                }
            }
            
            let result = tokenize(&input);
            assert!(result.is_ok(), "Failed to tokenize comments: {}", input);
        }
        
        #[test]
        fn prop_whitespace_handling(
            // Test handling of different whitespace characters
            tokens in prop::collection::vec(prop::sample::select(vec![
                "+", "-", "*", "/", "==", "!=", "<=", ">=", "vibe", "yeet", "123", "x"
            ]), 1..5),
            spaces in prop::collection::vec(prop::sample::select(vec![
                " ", "\t", "\n", "\r\n", "  ", "\t\t", "\n\n"
            ]), 0..10)
        ) {
            // Combine tokens with random whitespace
            let mut input = String::new();
            let mut i = 0;
            
            for token in &tokens {
                if i < spaces.len() {
                    input.push_str(spaces[i]);
                    i += 1;
                }
                input.push_str(token);
            }
            
            // Add whitespace at the end too
            if i < spaces.len() {
                input.push_str(spaces[i]);
            }
            
            let result = tokenize(&input);
            assert!(result.is_ok(), "Failed to tokenize with whitespace: {}", input);
            
            // The number of tokens should be the number of input tokens plus EOF
            let actual_tokens = result.unwrap();
            assert!(actual_tokens.len() > 0, "No tokens generated");
            
            // The last token should be EOF
            assert_eq!(actual_tokens.last().unwrap(), &Token::Eof);
        }
    }
    
    #[test]
    fn test_location_tracking() {
        let input = "1\n2\n3";
        let mut lexer = Lexer::new(input);
        
        // First token: "1" on line 1
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Int(1));
        assert_eq!(lexer.line, 2); // Already moved to the next line
        
        // Second token: "2" on line 2
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Int(2));
        assert_eq!(lexer.line, 3); // Moved to line 3
        
        // Third token: "3" on line 3
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Int(3));
        
        // Final token: EOF
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Eof);
    }
    
    #[test]
    fn test_periodt_statement_tokenization() {
        let input = "periodt x < 10 { x = x + 1; }";
        let result = tokenize(input);
        
        assert!(result.is_ok(), "Failed to tokenize periodt statement");
        let tokens = result.unwrap();
        
        // Expected tokens for "periodt x < 10 { x = x + 1; }"
        assert_eq!(tokens.len(), 13, "Expected 13 tokens, got {}", tokens.len());
        
        // Check each token in sequence
        assert_eq!(tokens[0], Token::Periodt);  // periodt
        assert!(matches!(tokens[1], Token::Identifier(ref s) if s == "x"));  // x
        assert_eq!(tokens[2], Token::Lt);  // <
        assert_eq!(tokens[3], Token::Int(10));  // 10
        assert_eq!(tokens[4], Token::LBrace);  // {
        assert!(matches!(tokens[5], Token::Identifier(ref s) if s == "x"));  // x
        assert_eq!(tokens[6], Token::Assign);  // =
        assert!(matches!(tokens[7], Token::Identifier(ref s) if s == "x"));  // x
        assert_eq!(tokens[8], Token::Plus);  // +
        assert_eq!(tokens[9], Token::Int(1));  // 1
        assert_eq!(tokens[10], Token::Semicolon);  // ;
        assert_eq!(tokens[11], Token::RBrace);  // }
        assert_eq!(tokens[12], Token::Eof);  // EOF
    }
    
    #[test]
    fn test_ghosted_keyword_tokenization() -> Result<(), Error> {
        let input = "ghosted";
        let expected_tokens = vec![Token::Ghosted, Token::Eof];
        let tokens = tokenize(input)?;
        assert_eq!(tokens, expected_tokens, "Failed to tokenize 'ghosted' keyword");
        Ok(())
    }
    
    #[test]
    fn test_simp_keyword_tokenization() -> Result<(), Error> {
        let input = "simp";
        let expected_tokens = vec![Token::Simp, Token::Eof];
        let tokens = tokenize(input)?;
        assert_eq!(tokens, expected_tokens, "Failed to tokenize 'simp' keyword");
        Ok(())
    }
    
    #[test]
    fn test_ghosted_in_loop_context() -> Result<(), Error> {
        let input = "bestie i := 0; i < 10; i++ { lowkey i == 5 { ghosted; } }";
        let result = tokenize(input);
        
        assert!(result.is_ok(), "Failed to tokenize loop with ghosted statement");
        let tokens = result.unwrap();
        
        // Verify that the 'ghosted' keyword is correctly tokenized in a loop context
        let ghosted_index = tokens.iter().position(|t| t == &Token::Ghosted);
        assert!(ghosted_index.is_some(), "'ghosted' token not found in the token stream");
        
        // Verify the sequence: lowkey -> identifier -> eq -> int -> lbrace -> ghosted -> semicolon -> rbrace
        let idx = tokens.iter().position(|t| t == &Token::Lowkey).unwrap();
        assert!(matches!(tokens[idx+1], Token::Identifier(ref s) if s == "i"), "Expected identifier 'i' after 'lowkey'");
        assert_eq!(tokens[idx+2], Token::Eq, "Expected '==' after identifier");
        assert_eq!(tokens[idx+3], Token::Int(5), "Expected integer 5 after '=='");
        assert_eq!(tokens[idx+4], Token::LBrace, "Expected '{{' after condition");
        assert_eq!(tokens[idx+5], Token::Ghosted, "Expected 'ghosted' inside the conditional block");
        assert_eq!(tokens[idx+6], Token::Semicolon, "Expected ';' after 'ghosted'");
        assert_eq!(tokens[idx+7], Token::RBrace, "Expected '}}' to close the conditional block");
        
        Ok(())
    }
    
    #[test]
    fn test_simp_in_loop_context() -> Result<(), Error> {
        let input = "periodt x > 0 { lowkey x % 2 == 0 { simp; } x = x - 1; }";
        let result = tokenize(input);
        
        assert!(result.is_ok(), "Failed to tokenize loop with simp statement");
        let tokens = result.unwrap();
        
        // Verify that the 'simp' keyword is correctly tokenized in a loop context
        let simp_index = tokens.iter().position(|t| t == &Token::Simp);
        assert!(simp_index.is_some(), "'simp' token not found in the token stream");
        
        // Verify the sequence: lowkey -> identifier -> % -> int -> eq -> int -> lbrace -> simp -> semicolon -> rbrace
        let idx = tokens.iter().position(|t| t == &Token::Lowkey).unwrap();
        assert!(matches!(tokens[idx+1], Token::Identifier(ref s) if s == "x"), "Expected identifier 'x' after 'lowkey'");
        // Check for modulo token followed by identifier
        assert_eq!(tokens[idx+2], Token::Percent, "Expected '%' after identifier 'x'");
        // Find the simp and check surrounding structure
        let simp_idx = simp_index.unwrap();
        assert_eq!(tokens[simp_idx-1], Token::LBrace, "Expected '{{' before 'simp'");
        assert_eq!(tokens[simp_idx+1], Token::Semicolon, "Expected ';' after 'simp'");
        assert_eq!(tokens[simp_idx+2], Token::RBrace, "Expected '}}' to close the conditional block");
        
        Ok(())
    }
    
    #[test]
    fn test_byte_literals() -> Result<(), Error> {
        // Test various byte literals
        let inputs = vec![
            ("b'a'", Token::Byte(b'a')),
            ("b'0'", Token::Byte(b'0')),
            ("b'\\n'", Token::Byte(b'\n')),
            ("b'\\t'", Token::Byte(b'\t')),
            ("b'\\r'", Token::Byte(b'\r')),
            ("b'\\\\'", Token::Byte(b'\\')),
            ("b'\\''", Token::Byte(b'\'')),
        ];
        
        for (input, expected_token) in inputs {
            let result = tokenize(input);
            assert!(result.is_ok(), "Failed to tokenize byte literal: {}", input);
            
            let tokens = result.unwrap();
            assert_eq!(tokens.len(), 2, "Expected 2 tokens for '{}', got {} - tokens: {:?}", input, tokens.len(), tokens);
            assert_eq!(tokens[0], expected_token, "Token mismatch for byte literal {}", input);
            assert_eq!(tokens[1], Token::Eof, "Second token should be EOF");
        }
        
        // Test invalid byte literals
        let invalid_inputs = vec![
            "b''",              // Empty byte literal
            "b'ab'",           // Too many characters
            "b'\\x'",          // Invalid escape sequence
            "b'🙂'",           // Non-ASCII character
        ];
        
        for input in invalid_inputs {
            let result = tokenize(input);
            assert!(result.is_err(), "Expected error for invalid byte literal: {}, but got: {:?}", input, result);
        }
        
        Ok(())
    }
    
    #[test]
    fn test_rune_literals() -> Result<(), Error> {
        // Test various rune literals
        let inputs = vec![
            ("'a'", Token::Rune('a')),
            ("'0'", Token::Rune('0')),
            ("'\\n'", Token::Rune('\n')),
            ("'\\t'", Token::Rune('\t')),
            ("'\\r'", Token::Rune('\r')),
            ("'\\\\'", Token::Rune('\\')),
            ("'\\''", Token::Rune('\'')),
            ("'🙂'", Token::Rune('🙂')),  // Unicode rune (emoji)
        ];
        
        for (input, expected_token) in inputs {
            let result = tokenize(input);
            assert!(result.is_ok(), "Failed to tokenize rune literal: {}", input);
            
            let tokens = result.unwrap();
            assert_eq!(tokens.len(), 2, "Expected 2 tokens for '{}', got {} - tokens: {:?}", input, tokens.len(), tokens);
            assert_eq!(tokens[0], expected_token, "Token mismatch for rune literal {}", input);
            assert_eq!(tokens[1], Token::Eof, "Second token should be EOF");
        }
        
        // Test invalid rune literals
        let invalid_inputs = vec![
            "''",              // Empty rune literal
            "'ab'",           // Too many characters
            "'\\x'",          // Invalid escape sequence
        ];
        
        for input in invalid_inputs {
            let result = tokenize(input);
            assert!(result.is_err(), "Expected error for invalid rune literal: {}, but got: {:?}", input, result);
        }
        
        Ok(())
    }
    
    #[test]
    fn test_nested_loops_with_ghosted() -> Result<(), Error> {
        let input = r#"
        bestie i := 0; i < 5; i++ {
            bestie j := 0; j < 5; j++ {
                lowkey i + j > 5 {
                    ghosted; fr fr break out of inner loop
                }
                lowkey i == 3 {
                    lowkey j == 3 {
                        ghosted; ghosted; fr fr double break, syntax error but should tokenize
                    }
                }
            }
        }
        "#;
        
        let result = tokenize(input);
        assert!(result.is_ok(), "Failed to tokenize nested loops with ghosted");
        let tokens = result.unwrap();
        
        // Count the number of 'ghosted' tokens
        let ghosted_count = tokens.iter().filter(|&t| t == &Token::Ghosted).count();
        assert_eq!(ghosted_count, 3, "Expected 3 'ghosted' tokens, found {}", ghosted_count);
        
        Ok(())
    }
    
    #[test]
    fn test_logical_operators() -> Result<(), Error> {
        let input = "lowkey x > 0 && y < 10 || z == 5 { ghosted; }";
        let result = tokenize(input);
        
        assert!(result.is_ok(), "Failed to tokenize statement with logical operators");
        let tokens = result.unwrap();
        
        // Verify the sequence with logical operators
        assert_eq!(tokens[0], Token::Lowkey);
        assert!(matches!(tokens[1], Token::Identifier(ref s) if s == "x"));
        assert_eq!(tokens[2], Token::Gt);
        assert_eq!(tokens[3], Token::Int(0));
        assert_eq!(tokens[4], Token::And);
        assert!(matches!(tokens[5], Token::Identifier(ref s) if s == "y"));
        assert_eq!(tokens[6], Token::Lt);
        assert_eq!(tokens[7], Token::Int(10));
        assert_eq!(tokens[8], Token::Or);
        assert!(matches!(tokens[9], Token::Identifier(ref s) if s == "z"));
        assert_eq!(tokens[10], Token::Eq);
        assert_eq!(tokens[11], Token::Int(5));
        assert_eq!(tokens[12], Token::LBrace);
        assert_eq!(tokens[13], Token::Ghosted);
        assert_eq!(tokens[14], Token::Semicolon);
        assert_eq!(tokens[15], Token::RBrace);
        
        Ok(())
    }
    
    #[test]
    fn test_nested_loops_with_simp() -> Result<(), Error> {
        let input = r#"
        bestie i := 0; i < 10; i++ {
            lowkey i % 3 == 0 {
                simp; fr fr skip multiples of 3
            }
            bestie j := 0; j < i; j++ {
                lowkey j % 2 == 0 {
                    simp; fr fr skip even j values
                }
                fr fr process only when i is not multiple of 3 and j is odd
            }
        }
        "#;
        
        let result = tokenize(input);
        assert!(result.is_ok(), "Failed to tokenize nested loops with simp");
        let tokens = result.unwrap();
        
        // Count the number of 'simp' tokens
        let simp_count = tokens.iter().filter(|&t| t == &Token::Simp).count();
        assert_eq!(simp_count, 2, "Expected 2 'simp' tokens, found {}", simp_count);
        
        Ok(())
    }
    
    #[test]
    fn test_mixed_ghosted_and_simp() -> Result<(), Error> {
        let input = r#"
        periodt x > 0 {
            lowkey x > 100 {
                ghosted; fr fr exit the loop if x > 100
            }
            lowkey x % 2 == 0 {
                x = x / 2;
                simp; fr fr continue to next iteration
            }
            x = x * 3 + 1;
        }
        "#;
        
        let result = tokenize(input);
        assert!(result.is_ok(), "Failed to tokenize with mixed ghosted and simp");
        let tokens = result.unwrap();
        
        // Verify we have both 'ghosted' and 'simp' tokens
        let ghosted_index = tokens.iter().position(|t| t == &Token::Ghosted);
        let simp_index = tokens.iter().position(|t| t == &Token::Simp);
        
        assert!(ghosted_index.is_some(), "'ghosted' token not found in the token stream");
        assert!(simp_index.is_some(), "'simp' token not found in the token stream");
        
        Ok(())
    }
    
    #[test]
    fn test_line_comments() -> Result<(), Error> {
        let input = r#"
        sus x = 5; fr fr this is a comment
        x = 10; fr fr another comment
        fr fr full line comment
        yolo x; fr fr comment after statement
        "#;
        
        let expected_tokens = vec![
            Token::Sus,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Yolo,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];
        
        let tokens = tokenize(input)?;
        assert_eq!(tokens, expected_tokens, "Line comments not skipped correctly");
        Ok(())
    }
    
    #[test]
    fn test_block_comments() -> Result<(), Error> {
        let input = r#"
        sus y = based;
        no cap this is a 
        multi-line
        block comment on god
        yolo y;
        no cap single line block comment on god sus z = cap;
        "#;
        
        let expected_tokens = vec![
            Token::Sus,
            Token::Identifier("y".to_string()),
            Token::Assign,
            Token::Based,
            Token::Semicolon,
            Token::Yolo,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
             Token::Sus, // This should be tokenized after the block comment
             Token::Identifier("z".to_string()),
             Token::Assign,
             Token::Cap,
             Token::Semicolon,
            Token::Eof,
        ];
        
        let tokens = tokenize(input)?;
        assert_eq!(tokens, expected_tokens, "Block comments not skipped correctly");
        Ok(())
    }
    
    #[test]
    fn test_mixed_comments() -> Result<(), Error> {
        let input = r#"
        fr fr Start with a line comment
        sus a = 1; no cap block comment on god fr fr line comment
        lowkey based { fr fr inside if
           no cap nested? no on god maybe not? on god
        } fr fr end
        "#;
        
        // This input should cause an error because the lexer encounters '?' 
        // after the first "on god" closes the block comment.
        let result = tokenize(input);
        assert!(result.is_err(), "Expected an error due to unexpected character after non-nested block comment, but got Ok({:?})", result.ok());
        
        // Optionally, check the specific error details if needed
        if let Err(e) = result {
            assert!(e.message().contains("Unexpected character: Some('?')"), "Expected error message about '?', got {}", e.message());
        }
        
        Ok(())
    }
    
     #[test]
     fn test_comment_within_string() -> Result<(), Error> {
         let input = r#"sus message = "hello fr fr world no cap on god";"#;
         let expected_tokens = vec![
             Token::Sus,
             Token::Identifier("message".to_string()),
             Token::Assign,
             Token::String("hello fr fr world no cap on god".to_string()),
             Token::Semicolon,
             Token::Eof,
         ];
         let tokens = tokenize(input)?;
         assert_eq!(tokens, expected_tokens, "Comment markers within strings incorrectly processed");
         Ok(())
     }

     // Note: Testing unterminated block comments requires adjustments
     // to how errors are handled or returned by the lexer/skip_whitespace.
     // The current implementation prints a warning and might lead to 
     // unexpected token sequences if not handled carefully in the parser.

} 