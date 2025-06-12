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
    Periodt,    // while
    Stan,       // goroutine (async execution)
    Bestie,     // for
    Flex,       // range
    Ghosted,    // break
    Simp,       // continue
    Squad,      // struct
    Collab,     // interface
    Vibe,       // package
    Yeet,       // import
    BeLike,     // type alias
    VibeCheck,  // switch
    Mood,       // case
    Basic,      // default
    YeetError,  // panic (throw error)
    Catch,      // catch/recover
    Normie,     // int
    Tea,        // string
    Cap,        // bool
    NoCap,      // nil/null
    MainCharacter, // main function
    
    // Additional tokens for type system and control flow
    Arrow,      // ->
    Match,      // match keyword (alternative to vibe_check)
    If,         // if keyword (alternative to lowkey)
    
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
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    Not,
    BitwiseNot,
    
    // Assignment operators
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    ShortVarDecl,  // :=
    
    // Channel operators
    LeftArrow,     // <-
    Dm,            // dm (channel type)
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Question,
    
    // Special
    Eof,
    Newline,
    Illegal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub location: SourceLocation,
}

impl TokenType {
    // Convenience aliases for common token types
    pub const LBracket: TokenType = TokenType::LeftBracket;
    pub const RBracket: TokenType = TokenType::RightBracket;
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
            location: SourceLocation { line: 0, column: 0, file: None },
        }
    }
    
    pub fn with_location(token_type: TokenType, literal: &str, location: SourceLocation) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
            location,
        }
    }
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
    
    /// Tokenize the entire input and return all tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = token.token_type == TokenType::Eof;
            tokens.push(token);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
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
            '[' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::LeftBracket,
                    literal: "[".to_string(),
                    location,
                })
            }
            ']' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::RightBracket,
                    literal: "]".to_string(),
                    location,
                })
            }
            ',' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Comma,
                    literal: ",".to_string(),
                    location,
                })
            }
            ';' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Semicolon,
                    literal: ";".to_string(),
                    location,
                })
            }
            '.' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Dot,
                    literal: ".".to_string(),
                    location,
                })
            }
            '?' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Question,
                    literal: "?".to_string(),
                    location,
                })
            }
            '+' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::PlusAssign,
                        literal: "+=".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Plus,
                        literal: "+".to_string(),
                        location,
                    })
                }
            }
            '-' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::MinusAssign,
                        literal: "-=".to_string(),
                        location,
                    })
                } else if self.peek_char() == '>' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Arrow,
                        literal: "->".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Minus,
                        literal: "-".to_string(),
                        location,
                    })
                }
            }
            '*' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::MultiplyAssign,
                        literal: "*=".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Multiply,
                        literal: "*".to_string(),
                        location,
                    })
                }
            }
            '/' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::DivideAssign,
                        literal: "/=".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Divide,
                        literal: "/".to_string(),
                        location,
                    })
                }
            }
            '%' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::ModuloAssign,
                        literal: "%=".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Modulo,
                        literal: "%".to_string(),
                        location,
                    })
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Equal,
                        literal: "==".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Assign,
                        literal: "=".to_string(),
                        location,
                    })
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::NotEqual,
                        literal: "!=".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Not,
                        literal: "!".to_string(),
                        location,
                    })
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LessThanEqual,
                        literal: "<=".to_string(),
                        location,
                    })
                } else if self.peek_char() == '<' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LeftShift,
                        literal: "<<".to_string(),
                        location,
                    })
                } else if self.peek_char() == '-' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LeftArrow,
                        literal: "<-".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LessThan,
                        literal: "<".to_string(),
                        location,
                    })
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::GreaterThanEqual,
                        literal: ">=".to_string(),
                        location,
                    })
                } else if self.peek_char() == '>' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::RightShift,
                        literal: ">>".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::GreaterThan,
                        literal: ">".to_string(),
                        location,
                    })
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LogicalAnd,
                        literal: "&&".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::BitwiseAnd,
                        literal: "&".to_string(),
                        location,
                    })
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::LogicalOr,
                        literal: "||".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::BitwiseOr,
                        literal: "|".to_string(),
                        location,
                    })
                }
            }
            '^' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::BitwiseXor,
                    literal: "^".to_string(),
                    location,
                })
            }
            '~' => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::BitwiseNot,
                    literal: "~".to_string(),
                    location,
                })
            }
            ':' => {
                if self.peek_char() == '=' {
                    self.advance();
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::ShortVarDecl,
                        literal: ":=".to_string(),
                        location,
                    })
                } else {
                    self.advance();
                    Ok(Token {
                        token_type: TokenType::Colon,
                        literal: ":".to_string(),
                        location,
                    })
                }
            }
            '"' => self.read_string(location),
            _ if ch.is_alphabetic() || ch == '_' => self.read_identifier(location),
            _ if ch.is_numeric() => self.read_number(location),
            _ => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Illegal,
                    literal: ch.to_string(),
                    location,
                })
            }
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
    
    fn peek_char(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.position + 1).unwrap_or('\0')
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
            "stan" => TokenType::Stan,
            "bestie" => TokenType::Bestie,
            "flex" => TokenType::Flex,
            "ghosted" => TokenType::Ghosted,
            "simp" => TokenType::Simp,
            "squad" => TokenType::Squad,
            "collab" => TokenType::Collab,
            "vibe" => TokenType::Vibe,
            "yeet" => TokenType::Yeet,
            "be_like" => TokenType::BeLike,
            "vibe_check" => TokenType::VibeCheck,
            "mood" => TokenType::Mood,
            "basic" => TokenType::Basic,
            "yeet_error" => TokenType::YeetError,
            "catch" => TokenType::Catch,
            "normie" => TokenType::Normie,
            "tea" => TokenType::Tea,
            "cap" => TokenType::Cap,
            "no_cap" => TokenType::NoCap,
            "main_character" => TokenType::MainCharacter,
            "dm" => TokenType::Dm,
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
    
    fn read_string(&mut self, start_location: SourceLocation) -> Result<Token, Error> {
        self.advance(); // Skip opening quote
        let start_pos = self.position;
        
        while self.position < self.input.len() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance(); // Skip escape character
                if self.position < self.input.len() {
                    self.advance(); // Skip escaped character
                }
            } else {
                self.advance();
            }
        }
        
        if self.position >= self.input.len() {
            return Err(Error::Parse("Unterminated string literal".to_string()));
        }
        
        let literal = self.input[start_pos..self.position].to_string();
        self.advance(); // Skip closing quote
        
        Ok(Token {
            token_type: TokenType::String,
            literal,
            location: start_location,
        })
    }
}
