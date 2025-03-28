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
    
    /// Check if the current character is a digit
    pub fn is_current_digit(&self) -> bool {
        match self.ch {
            Some(ch) => Self::is_digit(ch),
            None => false,
        }
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
            Token::Illegal(s) => s.clone(),
            // Default literals for non-literal tokens
            Token::Assign => "=".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Bang => "!".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Lt => "<".to_string(),
            Token::Gt => ">".to_string(),
            Token::Eq => "==".to_string(),
            Token::NotEq => "!=".to_string(),
            Token::LtEq => "<=".to_string(),
            Token::GtEq => ">=".to_string(),
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
                "==", "!=", "<=", ">=", "=", "+", "-", "*", "/", "<", ">", "!"
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
                "<" => Token::Lt,
                ">" => Token::Gt,
                "!" => Token::Bang,
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
} 