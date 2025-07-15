//! Lexical analysis for CURSED

use crate::error::{CursedError, StructuredError, ErrorCode};
use crate::error::structured::ErrorSourceLocation;

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
    Integer(String),    // For integer literals with value
    StringLiteral(String), // For string literals with value
    String,
    Boolean,
    Character,
    Based,              // For 'based' literal
    
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
    Struct,      // struct (alternative)
    Collab,      // interface
    Impl,        // implementation
    Extends,     // interface inheritance
    ForImpl,     // for (used in impl for)
    Vibe,        // package
    Yeet,        // import
    BeLike,      // assignment operator
    VibeCheck,   // switch statement
    Mood,        // case
    Basic,       // default case
    YeetError,   // throw error
    Catch,       // catch error
    Where,       // where clause for generics
    Normie,      // integer type (i32)
    Tea,         // string type
    Txt,         // string type (alias)
    Sip,         // character type
    Smol,        // small integer type (i8)
    Mid,         // medium integer type (i16)
    Thicc,       // large integer type (i64)
    Snack,       // small float type (f32)
    Meal,        // large float type (f64)
    Byte,        // unsigned 8-bit integer (u8)
    Rune,        // Unicode code point (i32 alias)
    Extra,       // complex number type
    Lit,         // boolean type
    Cap,         // null/nil
    NoCap,       // not null
    Truth,       // true
    Lies,        // false (NoTruth)
    MainCharacter, // main function
    Dm,          // channel type
    Select,      // select statement
    Ready,       // ready (for select statements)
    LeftArrow,   // <- channel operator
    Arrow,       // -> return type arrow
    Later,       // later (defer statement)
    In,          // in (for-in loops)
    
    // Error handling tokens
    Yikes,       // error type declarations
    Shook,       // error propagation operator / panic function
    Fam,         // panic recovery blocks
    Panic,       // panic function
    Recover,     // recover function
    
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
    PlusPlus,       // ++
    MinusMinus,     // --
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
    Pipe,           // |
    
    // Assignment operators
    Assign,         // = (for assignment context)
    PlusEqual,      // +=
    MinusEqual,     // -=
    StarEqual,      // *=
    SlashEqual,     // /=
    PercentEqual,   // %=
    ColonEqual,     // :=
    
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
    DotDot,         // .. (for range expressions)
    Question,       // ?
    
    // Special
    At,             // @ (for pointer types)
    Newline,
    Eof,
    
    // Comments
    LineComment,    // fr fr line comment
    BlockComment,   // no cap ... on god block comment
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
            '+' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::PlusEqual, "+=".to_string(), start_column))
                } else if self.match_char('+') {
                    Ok(self.make_token(TokenKind::PlusPlus, "++".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Plus, "+".to_string(), start_column))
                }
            },
            '-' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::MinusEqual, "-=".to_string(), start_column))
                } else if self.match_char('>') {
                    Ok(self.make_token(TokenKind::Arrow, "->".to_string(), start_column))
                } else if self.match_char('-') {
                    Ok(self.make_token(TokenKind::MinusMinus, "--".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Minus, "-".to_string(), start_column))
                }
            },
            '*' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::StarEqual, "*=".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Star, "*".to_string(), start_column))
                }
            },
            '/' => {
                if self.match_char('/') {
                    // Line comment - skip to end of line
                    self.skip_line_comment();
                    return self.next_token();
                } else if self.match_char('*') {
                    // Block comment - skip to */
                    self.skip_c_style_block_comment()?;
                    return self.next_token();
                } else if self.match_char('=') {
                    Ok(self.make_token(TokenKind::SlashEqual, "/=".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Slash, "/".to_string(), start_column))
                }
            },
            '%' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::PercentEqual, "%=".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Percent, "%".to_string(), start_column))
                }
            },
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
                } else if self.match_char('=') {
                    Ok(self.make_token(TokenKind::ColonEqual, ":=".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Colon, ":".to_string(), start_column))
                }
            },
            '.' => {
                if self.match_char('.') {
                    Ok(self.make_token(TokenKind::DotDot, "..".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Dot, ".".to_string(), start_column))
                }
            },
            '?' => Ok(self.make_token(TokenKind::Question, "?".to_string(), start_column)),
            '@' => Ok(self.make_token(TokenKind::At, "@".to_string(), start_column)),
            '=' => {
                if self.match_char('=') {
                    Ok(self.make_token(TokenKind::EqualEqual, "==".to_string(), start_column))
                } else {
                    // Use Equal for single = by default (parser will determine context)
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
                    let error = StructuredError::new(ErrorCode::E0001, "Unexpected character: '&'".to_string())
                        .with_location(ErrorSourceLocation {
                            file: "".to_string(),
                            line: self.line,
                            column: start_column,
                            length: 1,
                            source_line: None,
                        })
                        .with_suggestions(vec![
                            "Use '&&' for logical AND".to_string(),
                            "Use bitwise operations if intended".to_string(),
                        ]);
                    Err(CursedError::from(error))
                }
            },
            '|' => {
                if self.match_char('|') {
                    Ok(self.make_token(TokenKind::PipePipe, "||".to_string(), start_column))
                } else {
                    Ok(self.make_token(TokenKind::Pipe, "|".to_string(), start_column))
                }
            },
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(self.make_token(TokenKind::Newline, "\n".to_string(), start_column))
            },
            '"' => self.string_literal(start_column),
            '\'' => self.character_literal(start_column),
            '`' => self.raw_string_literal(start_column),
            '\0' => Ok(Token {
                kind: TokenKind::Eof,
                lexeme: "".to_string(),
                line: self.line,
                column: start_column,
            }),
            _ if c.is_ascii_digit() => self.number_literal(start_column),
            _ if c.is_ascii_alphabetic() || c == '_' => self.identifier(start_column),
            _ => {
                let error = StructuredError::new(ErrorCode::E0005, format!("Unexpected character: {}", c))
                    .with_location(ErrorSourceLocation {
                        file: "".to_string(),
                        line: self.line,
                        column: start_column,
                        length: 1,
                        source_line: None,
                    })
                    .with_suggestions(vec![
                        "Check for typos in the source code".to_string(),
                        "Ensure the character is valid CURSED syntax".to_string(),
                    ]);
                Err(CursedError::from(error))
            },
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
    
    fn match_whitespace_and_keyword(&mut self, keyword: &str) -> bool {
        // Skip whitespace
        while !self.is_at_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
        
        // Try to match the keyword
        let start_pos = self.position;
        let mut matched = true;
        
        for expected_char in keyword.chars() {
            if self.is_at_end() || self.peek() != expected_char {
                matched = false;
                break;
            }
            self.advance();
        }
        
        // Check that the keyword ends with a non-alphanumeric character
        if matched && !self.is_at_end() && self.peek().is_alphanumeric() {
            matched = false;
        }
        
        if !matched {
            // Reset position if not matched
            self.position = start_pos;
        }
        
        matched
    }
    
    fn skip_block_comment(&mut self) -> Result<(), CursedError> {
        // Skip until we find "on god"
        while !self.is_at_end() {
            if self.peek() == 'o' && self.match_keyword_sequence("on god") {
                return Ok(());
            }
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
        
        Err(CursedError::syntax_error("Unterminated block comment (missing 'on god')"))
    }
    
    fn skip_c_style_block_comment(&mut self) -> Result<(), CursedError> {
        // Skip until we find "*/"
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance(); // consume '*'
                self.advance(); // consume '/'
                return Ok(());
            }
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
        
        Err(CursedError::syntax_error("Unterminated block comment (missing '*/')"))
    }
    
    fn handle_line_comment(&mut self, start_column: usize) -> Result<Token, CursedError> {
        let mut comment_content = String::new();
        
        // Skip any remaining whitespace after "fr fr"
        while !self.is_at_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
        
        // Collect the comment content
        while !self.is_at_end() && self.peek() != '\n' {
            comment_content.push(self.advance());
        }
        
        // For now, skip comments and return the next token
        // TODO: Add option to preserve comments for documentation
        self.next_token()
    }
    
    fn handle_block_comment(&mut self, start_column: usize) -> Result<Token, CursedError> {
        let mut comment_content = String::new();
        let mut nesting_level = 1;
        
        // Skip any remaining whitespace after "no cap"
        while !self.is_at_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
        
        // Collect the comment content with nesting support
        while !self.is_at_end() && nesting_level > 0 {
            if self.peek() == 'o' && self.peek_ahead("on god") {
                // Found end of block comment
                nesting_level -= 1;
                if nesting_level == 0 {
                    self.advance(); // consume 'o'
                    self.advance(); // consume 'n'
                    self.advance(); // consume ' '
                    self.advance(); // consume 'g'
                    self.advance(); // consume 'o'
                    self.advance(); // consume 'd'
                    break;
                }
            } else if self.peek() == 'n' && self.peek_ahead("no cap") {
                // Found nested block comment
                nesting_level += 1;
                self.advance(); // consume 'n'
                self.advance(); // consume 'o'
                self.advance(); // consume ' '
                self.advance(); // consume 'c'
                self.advance(); // consume 'a'
                self.advance(); // consume 'p'
                comment_content.push_str("no cap");
                continue;
            }
            
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            comment_content.push(self.advance());
        }
        
        if nesting_level > 0 {
            return Err(CursedError::syntax_error("Unterminated block comment (missing 'on god')"));
        }
        
        // For now, skip comments and return the next token
        // TODO: Add option to preserve comments for documentation
        self.next_token()
    }
    
    fn match_keyword_sequence(&mut self, keyword: &str) -> bool {
        let start_pos = self.position;
        let mut matched = true;
        
        for expected_char in keyword.chars() {
            if self.is_at_end() || self.peek() != expected_char {
                matched = false;
                break;
            }
            self.advance();
        }
        
        if !matched {
            // Reset position if not matched
            self.position = start_pos;
        }
        
        matched
    }

    fn string_literal(&mut self, start_column: usize) -> Result<Token, CursedError> {
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
                value.push(self.advance());
            } else if self.peek() == '\\' {
                // Handle escape sequences
                self.advance(); // consume backslash
                if self.is_at_end() {
                    return Err(CursedError::syntax_error("Unterminated string escape"));
                }
                
                match self.advance() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    '0' => value.push('\0'),
                    c => {
                        let error = StructuredError::invalid_escape_sequence(&c.to_string(), self.line, self.column);
                        return Err(CursedError::from(error));
                    }
                }
            } else {
                value.push(self.advance());
            }
        }
        
        if self.is_at_end() {
            let error = StructuredError::unterminated_string(self.line, start_column);
            return Err(CursedError::from(error));
        }
        
        // Consume closing quote
        self.advance();
        
        Ok(self.make_token(TokenKind::String, value, start_column))
    }

    fn character_literal(&mut self, start_column: usize) -> Result<Token, CursedError> {
        if self.is_at_end() {
            return Err(CursedError::syntax_error("Unterminated character literal"));
        }

        let char_value = if self.peek() == '\\' {
            // Handle escape sequences
            self.advance(); // consume backslash
            if self.is_at_end() {
                return Err(CursedError::syntax_error("Unterminated character escape"));
            }
            
            match self.advance() {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '"' => '"',
                '0' => '\0',
                c => {
                    let error = StructuredError::invalid_escape_sequence(&c.to_string(), self.line, self.column);
                    return Err(CursedError::from(error));
                }
            }
        } else {
            self.advance()
        };

        if self.is_at_end() || self.peek() != '\'' {
            return Err(CursedError::syntax_error("Unterminated character literal"));
        }

        // Consume closing quote
        self.advance();

        Ok(self.make_token(TokenKind::Character, char_value.to_string(), start_column))
    }

    fn raw_string_literal(&mut self, start_column: usize) -> Result<Token, CursedError> {
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != '`' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            value.push(self.advance());
        }
        
        if self.is_at_end() {
            return Err(CursedError::syntax_error("Unterminated raw string"));
        }
        
        // Consume closing backtick
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
        
        // Check for binary (0b), octal (0o), or hexadecimal (0x) prefixes
        if value == "0" && !self.is_at_end() {
            match self.peek() {
                'b' | 'B' => {
                    // Binary number (0b...)
                    value.push(self.advance()); // consume 'b'
                    while !self.is_at_end() && matches!(self.peek(), '0' | '1') {
                        value.push(self.advance());
                    }
                    return Ok(self.make_token(TokenKind::Number, value, start_column));
                },
                'o' | 'O' => {
                    // Octal number (0o...)
                    value.push(self.advance()); // consume 'o'
                    while !self.is_at_end() && self.peek().is_ascii_digit() && self.peek() < '8' {
                        value.push(self.advance());
                    }
                    return Ok(self.make_token(TokenKind::Number, value, start_column));
                },
                'x' | 'X' => {
                    // Hexadecimal number (0x...)
                    value.push(self.advance()); // consume 'x'
                    while !self.is_at_end() && self.peek().is_ascii_hexdigit() {
                        value.push(self.advance());
                    }
                    return Ok(self.make_token(TokenKind::Number, value, start_column));
                },
                _ => {
                    // Continue with regular decimal parsing
                }
            }
        }
        
        // Regular decimal number parsing
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }
        
        // Handle decimal point
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
            "damn" => TokenKind::Yolo,  // alias for return statement
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
            "struct" => TokenKind::Struct,
            "collab" => TokenKind::Collab,
            "impl" => TokenKind::Impl,
            "extends" => TokenKind::Extends,
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
            "txt" => TokenKind::Txt,
            "sip" => TokenKind::Sip,
            "smol" => TokenKind::Smol,
            "mid" => TokenKind::Mid,
            "thicc" => TokenKind::Thicc,
            "snack" => TokenKind::Snack,
            "meal" => TokenKind::Meal,
            "byte" => TokenKind::Byte,
            "rune" => TokenKind::Rune,
            "extra" => TokenKind::Extra,
            "lit" => TokenKind::Lit,

            "nocap" => TokenKind::NoCap,
            "main_character" => TokenKind::MainCharacter,
            "dm" => TokenKind::Dm,
            "select" => TokenKind::Select,
            "ready" => TokenKind::Ready,
            "later" => TokenKind::Later,
            "in" => TokenKind::In,
            
            // Error handling keywords
            "yikes" => TokenKind::Yikes,
            "shook" => TokenKind::Shook,
            "fam" => TokenKind::Fam,
            "panic" => TokenKind::Panic,
            "recover" => TokenKind::Recover,
            
            // Visibility modifiers
            "spill" => TokenKind::Spill,
            "priv" => TokenKind::Priv,
            "crew" => TokenKind::Crew,
            
            // Boolean literals
            "based" => TokenKind::Truth,
            "cap" => TokenKind::Lies,
            
            // Nil literal
            "cringe" => TokenKind::Cap,
            
            // Comments - handle special keywords for comments
            "fr" => {
                // Check if this is "fr fr" (line comment)
                if self.match_whitespace_and_keyword("fr") {
                    self.skip_line_comment();
                    return self.next_token();
                } else {
                    TokenKind::Identifier
                }
            },
            "no" => {
                // Check if this is "no cap" (block comment start)
                if self.match_whitespace_and_keyword("cap") {
                    self.skip_block_comment().map_err(|e| e)?;
                    return self.next_token();
                } else {
                    TokenKind::Identifier
                }
            },
            
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
    
    fn peek_ahead(&self, text: &str) -> bool {
        let start_pos = self.position;
        for (i, expected_char) in text.chars().enumerate() {
            if let Some(actual_char) = self.chars.get(start_pos + i) {
                if *actual_char != expected_char {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
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
        assert!(keyword_count >= 13, "Expected at least 13 CURSED keywords, got {}", keyword_count);
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

    #[test]
    fn test_error_handling_tokens() {
        let mut lexer = Lexer::new("yikes shook fam".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 4); // 3 error tokens + EOF
        assert_eq!(tokens[0].kind, TokenKind::Yikes);
        assert_eq!(tokens[0].lexeme, "yikes");
        assert_eq!(tokens[1].kind, TokenKind::Shook);
        assert_eq!(tokens[1].lexeme, "shook");
        assert_eq!(tokens[2].kind, TokenKind::Fam);
        assert_eq!(tokens[2].lexeme, "fam");
        assert_eq!(tokens[3].kind, TokenKind::Eof);
    }

    #[test]
    fn test_cursed_comments() {
        // Test line comments
        let mut lexer = Lexer::new("fr fr This is a line comment\nslay hello()".to_string());
        let tokens = lexer.tokenize().unwrap();
        

        
        // Should have: newline, slay, hello, (, ), EOF (comments are skipped but newline remains)
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].kind, TokenKind::Newline);
        assert_eq!(tokens[1].kind, TokenKind::Slay);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].lexeme, "hello");
        
        // Test block comments
        let mut lexer = Lexer::new("slay test()\nno cap\nThis is a block comment\nthat spans multiple lines\non god\nyolo 42".to_string());
        let tokens = lexer.tokenize().unwrap();
        

        
        // Should have: slay, test, (, ), newline, newline, yolo, 42, EOF (block comment is skipped)
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].kind, TokenKind::Slay);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "test");
        assert_eq!(tokens[6].kind, TokenKind::Yolo);
        assert_eq!(tokens[7].kind, TokenKind::Number);
        assert_eq!(tokens[7].lexeme, "42");
    }

    #[test]
    fn test_comments_dont_interfere_with_keywords() {
        // Test that "fr" and "no" work as identifiers when not followed by comment syntax
        let mut lexer = Lexer::new("fr = 42\nno = 24".to_string());
        let tokens = lexer.tokenize().unwrap();
        

        
        // Should have: fr, =, 42, newline, no, =, 24, EOF
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, "fr");
        assert_eq!(tokens[4].kind, TokenKind::Identifier);
        assert_eq!(tokens[4].lexeme, "no");
    }

    #[test]
    fn test_c_style_comments() {
        // Test C-style line comments
        let mut lexer = Lexer::new("// This is a line comment\nslay hello()".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should have: newline, slay, hello, (, ), EOF (comments are skipped but newline remains)
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].kind, TokenKind::Newline);
        assert_eq!(tokens[1].kind, TokenKind::Slay);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].lexeme, "hello");
        
        // Test C-style block comments
        let mut lexer = Lexer::new("slay test()\n/* This is a block comment\nthat spans multiple lines */\nyolo 42".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should have: slay, test, (, ), newline, newline, yolo, 42, EOF (block comment is skipped)
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].kind, TokenKind::Slay);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "test");
        assert_eq!(tokens[6].kind, TokenKind::Yolo);
        assert_eq!(tokens[7].kind, TokenKind::Number);
        assert_eq!(tokens[7].lexeme, "42");
    }

    #[test]
    fn test_mixed_comment_styles() {
        // Test that both C-style and CURSED-style comments work together
        let code = r#"
// C-style line comment
slay test() {
    /* C-style block comment */
    sus x = 42;
    fr fr CURSED line comment
    no cap
    CURSED block comment
    on god
    yolo x;
}
"#;
        let mut lexer = Lexer::new(code.to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Verify key tokens are present and comments are properly skipped
        let token_lexemes: Vec<String> = tokens.iter().map(|t| t.lexeme.clone()).collect();
        assert!(token_lexemes.contains(&"slay".to_string()));
        assert!(token_lexemes.contains(&"test".to_string()));
        assert!(token_lexemes.contains(&"sus".to_string()));
        assert!(token_lexemes.contains(&"x".to_string()));
        assert!(token_lexemes.contains(&"42".to_string()));
        assert!(token_lexemes.contains(&"yolo".to_string()));
        
        // Verify comments are not present as tokens
        assert!(!token_lexemes.contains(&"C-style".to_string()));
        assert!(!token_lexemes.contains(&"CURSED".to_string()));
        assert!(!token_lexemes.contains(&"block".to_string()));
        assert!(!token_lexemes.contains(&"comment".to_string()));
    }

    #[test]
    fn test_slash_not_tokenized_as_separate_slashes() {
        // Test that // is properly handled as comment start, not as two slash tokens
        let mut lexer = Lexer::new("x = 5 // comment\ny = 10".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        // Should have: x, =, 5, newline, y, =, 10, EOF
        // No separate slash tokens should be present
        assert_eq!(tokens.len(), 8);
        
        let slash_tokens: Vec<&Token> = tokens.iter().filter(|t| t.kind == TokenKind::Slash).collect();
        assert_eq!(slash_tokens.len(), 0, "Found unexpected slash tokens: {:?}", slash_tokens);
        
        // Verify the expected tokens are there
        assert_eq!(tokens[0].lexeme, "x");
        assert_eq!(tokens[1].kind, TokenKind::Equal);
        assert_eq!(tokens[2].lexeme, "5");
        assert_eq!(tokens[3].kind, TokenKind::Newline);
        assert_eq!(tokens[4].lexeme, "y");
        assert_eq!(tokens[5].kind, TokenKind::Equal);
        assert_eq!(tokens[6].lexeme, "10");
        assert_eq!(tokens[7].kind, TokenKind::Eof);
    }

    #[test]
    fn test_for_in_tokens() {
        let mut lexer = Lexer::new("bestie person in people".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        println!("🔍 For-in test - found {} tokens", tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            println!("  #{} {:?} '{}'", i, token.kind, token.lexeme);
        }
        
        // Should have: Bestie, Identifier(person), In, Identifier(people), Eof
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].kind, TokenKind::Bestie);
        assert_eq!(tokens[0].lexeme, "bestie");
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "person");
        assert_eq!(tokens[2].kind, TokenKind::In);
        assert_eq!(tokens[2].lexeme, "in");
        assert_eq!(tokens[3].kind, TokenKind::Identifier);
        assert_eq!(tokens[3].lexeme, "people");
        assert_eq!(tokens[4].kind, TokenKind::Eof);
    }
}
