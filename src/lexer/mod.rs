//! Lexical analysis for CURSED

use crate::error::CursedError;

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    chars: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number,
    String,
    Boolean,
    
    // Identifiers
    Identifier,
    
    // Traditional Keywords (for compatibility)
    Let,
    Mut,
    Fn,
    If,
    Else,
    While,
    For,
    Return,
    
    // CURSED Gen Z Keywords
    Slay,        // function definition
    Yolo,        // return statement
    Sus,         // mutable variable
    Facts,       // immutable constant
    Lowkey,      // if statement
    Highkey,     // else statement
    Periodt,     // while loop
    Stan,        // goroutine
    Bestie,      // for loop
    Flex,        // while loop (alternative)
    Ghosted,     // break
    Simp,        // continue
    Squad,       // struct
    Collab,      // interface
    Vibe,        // package
    Yeet,        // import
    BeLike,      // assignment operator
    VibeCheck,   // switch statement
    Mood,        // case
    Basic,       // default case
    YeetError,   // throw error
    Catch,       // catch error
    Where,       // where clause for generics
    Normie,      // integer type
    Tea,         // string type
    Cap,         // null/nil
    NoCap,       // not null
    Truth,       // true
    Lies,        // false (NoTruth)
    MainCharacter, // main function
    Dm,          // channel type
    Select,      // select statement
    LeftArrow,   // <- channel operator
    
    // Visibility modifiers
    Spill,       // pub (public)
    Priv,        // private
    Crew,        // pkg (package)
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,        // %
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AmpAmp,         // &&
    PipePipe,       // ||
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftAngle,      // < (for generics)
    RightAngle,     // > (for generics)
    Comma,
    Semicolon,
    Colon,
    DoubleColon,    // :: (for paths and type annotations)
    Dot,
    
    // Special
    Newline,
    Eof,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        Self {
            input,
            chars,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, CursedError> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(Token {
                kind: TokenKind::Eof,
                lexeme: "".to_string(),
                line: self.line,
                column: self.column,
            });
        }
        
        let start_column = self.column;
        let c = self.advance();
        
        match c {
            '+' => Ok(self.make_token(TokenKind::Plus, "+".to_string(), start_column)),
            '-' => Ok(self.make_token(TokenKind::Minus, "-".to_string(), start_column)),
            '*' => Ok(self.make_token(TokenKind::Star, "*".to_string(), start_column)),
            '/' => {
                if self.match_char('/') {
                    // Line comment - skip until newline
                    self.skip_line_comment();
                    self.next_token() // Get next token after comment
                } else {
                    Ok(self.make_token(TokenKind::Slash, "/".to_string(), start_column))
                }
            },
            '%' => Ok(self.make_token(TokenKind::Percent, "%".to_string(), start_column)),
            '(' => Ok(self.make_token(TokenKind::LeftParen, "(".to_string(), start_column)),
            ')' => Ok(self.make_token(TokenKind::RightParen, ")".to_string(), start_column)),
            '{' => Ok(self.make_token(TokenKind::LeftBrace, "{".to_string(), start_column)),
            '}' => Ok(self.make_token(TokenKind::RightBrace, "}".to_string(), start_column)),
            '[' => Ok(self.make_token(TokenKind::LeftBracket, "[".to_string(), start_column)),
            ']' => Ok(self.make_token(TokenKind::RightBracket, "]".to_string(), start_column)),
            ',' => Ok(self.make_token(TokenKind::Comma, ",".to_string(), start_column)),
            ';' => Ok(self.make_token(TokenKind::Semicolon, ";".to_string(), start_column)),
            ':' => {
                if self.match_char(':') {
                    Ok(self.make_token(TokenKind::DoubleColon, "::".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Colon, ":".to_string(), start_column))
                }
            },
            '.' => Ok(self.make_token(TokenKind::Dot, ".".to_string(), start_column)),
            '=' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::EqualEqual, "==".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Equal, "=".to_string(), start_column))
                }
            },
            '!' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::BangEqual, "!=".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Bang, "!".to_string(), start_column))
                }
            },
            '<' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::LessEqual, "<=".to_string(), start_column))
                } else if self.match_char('-') {
                    Ok(self.make_token(TokenKind::LeftArrow, "<-".to_string(), start_column))
                } else {
                    // Check if this is a generic context (heuristic)
                    if self.is_generic_context() {
                        Ok(self.make_token(TokenKind::LeftAngle, "<".to_string(), start_column))
                    } else {
                        Ok(self.make_token(TokenKind::Less, "<".to_string(), start_column))
                    }
                }
            },
            '>' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::GreaterEqual, ">=".to_string(), start_column))
                } else {
                    // Check if this is a generic context (heuristic)
                    if self.is_generic_context() {
                        Ok(self.make_token(TokenKind::RightAngle, ">".to_string(), start_column))
                    } else {
                        Ok(self.make_token(TokenKind::Greater, ">".to_string(), start_column))
                    }
                }
            },
            '&' => {
                if self.match_char('&') {
                    Ok(self.make_token(TokenKind::AmpAmp, "&&".to_string(), start_column))
                } else {
                    Err(CursedError::syntax_error("Unexpected character: '&' (use '&&' for logical AND)"))
                }
            },
            '|' => {
                if self.match_char('|') {
                    Ok(self.make_token(TokenKind::PipePipe, "||".to_string(), start_column))
                } else {
                    Err(CursedError::syntax_error("Unexpected character: '|' (use '||' for logical OR)"))
                }
            },
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(self.make_token(TokenKind::Newline, "\n".to_string(), start_column))
            },
            '"' => self.string_literal(start_column),
            '\0' => Ok(Token {
                kind: TokenKind::Eof,
                lexeme: "".to_string(),
                line: self.line,
                column: start_column,
            }),
            _ if c.is_ascii_digit() => self.number_literal(start_column),
            _ if c.is_ascii_alphabetic() || c == '_' => self.identifier(start_column),
            _ => Err(CursedError::syntax_error(&format!("Unexpected character: {}", c))),
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                _ => break,
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn string_literal(&mut self, start_column: usize) -> Result<Token, CursedError> {
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            value.push(self.advance());
        }
        
        if self.is_at_end() {
            return Err(CursedError::syntax_error("Unterminated string"));
        }
        
        // Consume closing quote
        self.advance();
        
        Ok(self.make_token(TokenKind::String, value, start_column))
    }

    fn number_literal(&mut self, start_column: usize) -> Result<Token, CursedError> {
        let mut value = String::new();
        
        // Safe character access with bounds checking
        if self.position > 0 {
            if let Some(ch) = self.chars.get(self.position - 1).copied() {
                value.push(ch);
            } else {
                return Err(CursedError::syntax_error("Invalid character position during number parsing"));
            }
        } else {
            return Err(CursedError::syntax_error("Cannot parse number at start of input"));
        }
        
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }
        
        // Handle decimal
        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            value.push(self.advance()); // consume '.'
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }
        
        Ok(self.make_token(TokenKind::Number, value, start_column))
    }

    fn identifier(&mut self, start_column: usize) -> Result<Token, CursedError> {
        let mut value = String::new();
        
        // Safe character access with bounds checking
        if self.position > 0 {
            if let Some(ch) = self.input.chars().nth(self.position - 1) {
                value.push(ch);
            } else {
                return Err(CursedError::syntax_error("Invalid character position during identifier parsing"));
            }
        } else {
            return Err(CursedError::syntax_error("Cannot parse identifier at start of input"));
        }
        
        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == '_') {
            value.push(self.advance());
        }
        
        let kind = match value.as_str() {
            // Traditional keywords (for compatibility)
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "return" => TokenKind::Return,
            
            // CURSED Gen Z keywords
            "slay" => TokenKind::Slay,
            "yolo" => TokenKind::Yolo,
            "sus" => TokenKind::Sus,
            "facts" => TokenKind::Facts,
            "lowkey" => TokenKind::Lowkey,
            "highkey" => TokenKind::Highkey,
            "periodt" => TokenKind::Periodt,
            "stan" => TokenKind::Stan,
            "bestie" => TokenKind::Bestie,
            "flex" => TokenKind::Flex,
            "ghosted" => TokenKind::Ghosted,
            "simp" => TokenKind::Simp,
            "squad" => TokenKind::Squad,
            "collab" => TokenKind::Collab,
            "vibe" => TokenKind::Vibe,
            "yeet" => TokenKind::Yeet,
            "be_like" => TokenKind::BeLike,
            "vibe_check" => TokenKind::VibeCheck,
            "mood" => TokenKind::Mood,
            "basic" => TokenKind::Basic,
            "yeet_error" => TokenKind::YeetError,
            "catch" => TokenKind::Catch,
            "where" => TokenKind::Where,
            "normie" => TokenKind::Normie,
            "tea" => TokenKind::Tea,
            "cap" => TokenKind::Cap,
            "nocap" => TokenKind::NoCap,
            "main_character" => TokenKind::MainCharacter,
            "dm" => TokenKind::Dm,
            "select" => TokenKind::Select,
            
            // Visibility modifiers
            "spill" => TokenKind::Spill,
            "priv" => TokenKind::Priv,
            "crew" => TokenKind::Crew,
            
            // Boolean literals
            "true" | "based" => TokenKind::Boolean,
            "false" | "lies" => TokenKind::Boolean,
            
            _ => TokenKind::Identifier,
        };
        
        Ok(self.make_token(kind, value, start_column))
    }

    fn make_token(&self, kind: TokenKind, lexeme: String, column: usize) -> Token {
        Token {
            kind,
            lexeme,
            line: self.line,
            column,
        }
    }

    fn advance(&mut self) -> char {
        let c = self.chars.get(self.position).copied().unwrap_or('\0');
        self.position += 1;
        self.column += 1;
        c
    }

    fn peek(&self) -> char {
        self.chars.get(self.position).copied().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.chars.get(self.position + 1).copied().unwrap_or('\0')
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.chars.len()
    }

    /// Heuristic to determine if we're in a generic context
    /// This helps distinguish between `<` as a comparison operator vs generic delimiter
    fn is_generic_context(&self) -> bool {
        // Look back to see if we're after identifiers that might be generic
        if self.position < 2 {
            return false;
        }
        
        // Look for patterns like:
        // - identifier< (e.g., "Vec<")
        // - function_name< (e.g., "foo<")
        // - ": " (e.g., "x: Vec<")
        let mut check_pos = self.position;
        
        // Skip whitespace backwards
        while check_pos > 0 && self.chars.get(check_pos - 1).map_or(false, |c| c.is_whitespace()) {
            check_pos -= 1;
        }
        
        if check_pos == 0 {
            return false;
        }
        
        // Check the character before whitespace
        let prev_char = self.chars.get(check_pos - 1).copied().unwrap_or('\0');
        
        // Generic context if preceded by identifier characters, colon, or comma
        prev_char.is_ascii_alphanumeric() || prev_char == '_' || prev_char == ':' || prev_char == ','
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CursedError> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.kind, TokenKind::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut lexer = Lexer::new("+ - * /".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 5); // 4 operators + EOF
        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Star);
        assert_eq!(tokens[3].kind, TokenKind::Slash);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("\"hello world\"".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2); // string + EOF
        assert_eq!(tokens[0].kind, TokenKind::String);
        assert_eq!(tokens[0].lexeme, "hello world");
    }

    #[test]
    fn test_cursed_demo_keywords() {
        // Test that the demo program's key tokens are recognized
        let demo_snippet = r#"
vibe main
slay calculateArea(radius) {
    sus x = 42
    yolo result
}
lowkey x > 5 {
    vibez.spill("hello")
}
"#;
        
        let mut lexer = Lexer::new(demo_snippet.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        println!("🔍 Demo lexer test - found {} tokens", tokens.len());
        
        // Check that all key CURSED keywords are recognized
        let expected_keywords = ["vibe", "main", "slay", "calculateArea", "sus", "yolo", "lowkey", "vibez"];
        
        for keyword in &expected_keywords {
            let found = tokens.iter().any(|t| t.lexeme == *keyword);
            println!("  Keyword '{}': {}", keyword, if found { "✅ Found" } else { "❌ Missing" });
            assert!(found, "Expected keyword '{}' not found in tokens", keyword);
        }
        
        // Check token types for specific keywords
        let vibe_token = tokens.iter().find(|t| t.lexeme == "vibe");
        assert!(vibe_token.is_some());
        assert_eq!(vibe_token.unwrap().kind, TokenKind::Vibe);
        
        let slay_token = tokens.iter().find(|t| t.lexeme == "slay");
        assert!(slay_token.is_some());
        assert_eq!(slay_token.unwrap().kind, TokenKind::Slay);
        
        let sus_token = tokens.iter().find(|t| t.lexeme == "sus");
        assert!(sus_token.is_some());
        assert_eq!(sus_token.unwrap().kind, TokenKind::Sus);
        
        let yolo_token = tokens.iter().find(|t| t.lexeme == "yolo");
        assert!(yolo_token.is_some());
        assert_eq!(yolo_token.unwrap().kind, TokenKind::Yolo);
        
        let lowkey_token = tokens.iter().find(|t| t.lexeme == "lowkey");
        assert!(lowkey_token.is_some());
        assert_eq!(lowkey_token.unwrap().kind, TokenKind::Lowkey);
        
        println!("✅ All CURSED keywords recognized correctly!");
    }

    #[test]
    fn test_full_demo_tokenization() {
        // Test the actual demo program content
        let demo_content = r#"vibe main

fr fr This is a basic hello world demo in the CURSED language
fr fr showcasing Gen Z slang syntax and practical functionality

yeet "vibez"  fr fr import standard library

slay calculateArea(radius snack) snack {
    yolo 3.14159 * radius * radius
}

slay greetUser(name tea) {
    vibez.spill("Hello, " + name + "! Welcome to CURSED!")
}

slay demonstrateBasics() {
    fr fr Variable declarations
    sus radius snack = 5.0
    sus userName tea = "Developer"
    sus isAwesome lit = based  fr fr true
    
    fr fr Function calls
    sus area = calculateArea(radius)
    greetUser(userName)
    
    fr fr Output
    vibez.spill("Circle radius: " + radius)
    vibez.spill("Circle area: " + area)
    
    fr fr Conditionals
    lowkey isAwesome {
        vibez.spill("This language is based!")
    } highkey {
        vibez.spill("Something is sus...")
    }
}

slay main() {
    vibez.spill("=== CURSED Language Demo ===")
    demonstrateBasics()
    vibez.spill("=== Demo Complete ===")
}"#;
        
        let mut lexer = Lexer::new(demo_content.to_string());
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => panic!("❌ Failed to tokenize demo program: {}", e),
        };
        
        println!("🎉 Successfully tokenized full demo program!");
        println!("📊 Total tokens: {}", tokens.len());
        
        // Count different types of tokens
        let mut keyword_count = 0;
        let mut identifier_count = 0;
        let mut string_count = 0;
        let mut number_count = 0;
        
        for token in &tokens {
            match token.kind {
                TokenKind::Vibe | TokenKind::Slay | TokenKind::Sus | TokenKind::Yolo |
                TokenKind::Lowkey | TokenKind::Highkey | TokenKind::Yeet | TokenKind::Facts => {
                    keyword_count += 1;
                },
                TokenKind::Identifier => identifier_count += 1,
                TokenKind::String => string_count += 1,
                TokenKind::Number => number_count += 1,
                _ => {},
            }
        }
        
        println!("📋 Token breakdown:");
        println!("  CURSED Keywords: {}", keyword_count);
        println!("  Identifiers: {}", identifier_count);
        println!("  Strings: {}", string_count);
        println!("  Numbers: {}", number_count);
        
        // Verify we have a reasonable distribution
        assert!(keyword_count >= 15, "Expected at least 15 CURSED keywords, got {}", keyword_count);
        assert!(identifier_count >= 10, "Expected at least 10 identifiers, got {}", identifier_count);
        assert!(string_count >= 5, "Expected at least 5 strings, got {}", string_count);
        assert!(number_count >= 1, "Expected at least 1 number, got {}", number_count);
        
        println!("✅ Demo program tokenization test passed!");
    }

    #[test]
    fn test_number_literal() {
        let mut lexer = Lexer::new("123 45.67".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 3); // 2 numbers + EOF
        assert_eq!(tokens[0].kind, TokenKind::Number);
        assert_eq!(tokens[0].lexeme, "123");
        assert_eq!(tokens[1].kind, TokenKind::Number);
        assert_eq!(tokens[1].lexeme, "45.67");
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("let fn if else".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Fn);
        assert_eq!(tokens[2].kind, TokenKind::If);
        assert_eq!(tokens[3].kind, TokenKind::Else);
    }
}
