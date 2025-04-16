use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::lexer::token::Token;
use crate::lexer::utils::{is_digit, is_hex_digit, is_letter, is_octal_digit, peek_sequence};

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
    #[tracing::instrument(skip(input), level = "debug")]
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
            // Use nth to avoid collecting the entire string into a vector
            self.ch = self.input.chars().nth(self.read_position);
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
            // Use nth to avoid collecting the entire string into a vector
            self.input.chars().nth(self.read_position)
        }
    }

    /// Check for a specific sequence in the input
    fn peek_sequence(&self, sequence: &str) -> bool {
        peek_sequence(self.input, self.position, self.read_position, sequence)
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
                        }
                        None => {
                            // Error: Unterminated block comment - We can't return Error here directly
                            // Mark as illegal state or handle in next_token maybe?
                            // For now, just break to avoid infinite loop on EOF
                            println!("Warning: Unterminated block comment"); // Temporary warning
                            break;
                        }
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
    #[tracing::instrument(skip(self), fields(position = self.position, line = self.line, column = self.column), level = "trace")]
    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();

        // Handle special case for floats that start with a decimal point (e.g., .5)
        if self.ch == Some('.') && self.peek_char().map_or(false, is_digit) {
            return self.read_float_starting_with_dot();
        }

        let token = match self.ch {
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some('+') => {
                if self.peek_char() == Some('+') {
                    self.read_char();
                    Token::Inc
                } else if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::PlusAssign
                } else {
                    Token::Plus
                }
            }
            Some('-') => {
                if self.peek_char() == Some('-') {
                    self.read_char();
                    Token::Dec
                } else if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::MinusAssign
                } else {
                    Token::Minus
                }
            }
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some('&') => {
                if self.peek_char() == Some('&') {
                    self.read_char();
                    Token::And
                } else if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::BitAndAssign
                } else {
                    Token::BitAnd
                }
            }
            Some('|') => {
                if self.peek_char() == Some('|') {
                    self.read_char();
                    Token::Or
                } else if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::BitOrAssign
                } else {
                    Token::BitOr
                }
            }
            Some('^') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::BitXorAssign
                } else {
                    Token::BitXor
                }
            }
            Some('~') => Token::BitCompl,
            Some('*') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::AsteriskAssign
                } else {
                    Token::Asterisk
                }
            }
            Some('/') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::SlashAssign
                } else {
                    Token::Slash
                }
            }
            Some('%') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::PercentAssign
                } else {
                    Token::Percent
                }
            }
            Some('@') => Token::At,
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::LtEq
                } else if self.peek_char() == Some('-') {
                    self.read_char();
                    Token::Arrow
                } else if self.peek_char() == Some('<') {
                    self.read_char();
                    Token::ShiftLeft
                } else {
                    Token::Lt
                }
            }
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::GtEq
                } else if self.peek_char() == Some('>') {
                    self.read_char();
                    Token::ShiftRight
                } else {
                    Token::Gt
                }
            }
            Some(':') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::DeclAssign
                } else {
                    Token::Colon
                }
            }
            Some('.') => {
                if self.peek_char() == Some('.')
                    && self.read_position + 1 < self.input.len()
                    && self.input.chars().nth(self.read_position + 1) == Some('.')
                {
                    self.read_char(); // Read the second dot
                    self.read_char(); // Read the third dot
                    Token::Ellipsis
                } else {
                    Token::Dot
                }
            }
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,
            Some('\"') | Some('`') => self.read_string()?,
            Some('\'') => self.read_rune()?,
            Some('b') if self.peek_char() == Some('\'') => {
                self.read_char(); // consume 'b'
                return self.read_byte();
            }
            Some(c) if is_letter(c) => {
                let identifier = self.read_identifier();
                return Ok(self.lookup_identifier(identifier));
            }
            Some(c) if is_digit(c) => {
                return self.read_number();
            }
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
}
