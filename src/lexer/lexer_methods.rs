use crate::error::{Error, ErrorReporter};
use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use crate::lexer::utils::{is_digit, is_hex_digit, is_letter, is_octal_digit};

impl<'a> Lexer<'a> {
    /// Read an identifier
    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        let mut allow_dot = false; // Initially don't allow dots

        while let Some(c) = self.ch {
            if is_letter(c) || is_digit(c) || c == '_' {
                allow_dot = true; // After a letter, digit, or underscore, a dot can follow
                self.read_char();
            } else if c == '.' && allow_dot {
                // If this is a dot and we've seen a character before,
                // include it only if the next character is a letter
                if self.peek_char().map_or(false, is_letter) {
                    self.read_char();
                    allow_dot = false; // Only allow one dot at a time
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
    pub fn read_number(&mut self) -> Result<Token, Error> {
        let position = self.position;
        let mut is_float = false;
        let mut has_exponent = false;

        // Check for prefix indicating non-decimal base
        if self.ch == Some('0') {
            // Peek at the next character to determine base
            match self.peek_char() {
                Some('x') | Some('X') => {
                    // Hexadecimal
                    self.read_char(); // consume '0'
                    self.read_char(); // consume 'x' or 'X'
                    return self.read_hex_number();
                }
                Some('o') | Some('O') => {
                    // Octal
                    self.read_char(); // consume '0'
                    self.read_char(); // consume 'o' or 'O'
                    return self.read_octal_number();
                }
                Some('b') | Some('B') => {
                    // Binary
                    self.read_char(); // consume '0'
                    self.read_char(); // consume 'b' or 'B'
                    return self.read_binary_number();
                }
                _ => {}
            }
        }

        // Regular decimal integer or float
        while let Some(c) = self.ch {
            if is_digit(c) {
                self.read_char();
            } else if c == '.' && !is_float && !has_exponent {
                is_float = true;
                self.read_char();
                // If the next character is not a digit, this is a trailing decimal point,
                // which is valid in Go/CURSED (e.g., "1.")
            } else if (c == 'e' || c == 'E') && !has_exponent {
                // Handle exponent notation (e.g., 1.0e10, 1e5)
                has_exponent = true;
                is_float = true; // If we have an exponent, it's a float
                self.read_char(); // consume 'e' or 'E'

                // Check for sign
                if self.ch == Some('+') || self.ch == Some('-') {
                    self.read_char(); // consume sign
                }

                // Must have at least one digit after exponent
                if !self.ch.map_or(false, is_digit) {
                    let location = self.location();
                    return Err(ErrorReporter::lexer_error(
                        location,
                        "Invalid float: exponent has no digits",
                    ));
                }

                // Read exponent digits
                while let Some(c) = self.ch {
                    if is_digit(c) {
                        self.read_char();
                    } else {
                        break;
                    }
                }
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
                    Err(ErrorReporter::lexer_error(
                        location,
                        &format!("Could not parse float: {}", number_str),
                    ))
                }
            }
        } else {
            match number_str.parse::<i64>() {
                Ok(value) => Ok(Token::Int(value)),
                Err(_) => {
                    let location = self.location();
                    Err(ErrorReporter::lexer_error(
                        location,
                        &format!("Could not parse integer: {}", number_str),
                    ))
                }
            }
        }
    }

    /// Read a string literal
    pub fn read_string(&mut self) -> Result<Token, Error> {
        // Check if it's a regular quoted string or backtick string
        let is_backtick = self.ch == Some('`');
        let quote_char = if is_backtick { '`' } else { '\"' };

        self.read_char(); // Skip the opening quote

        let position = self.position;
        while self.ch != Some(quote_char) && self.ch != None {
            // For regular strings, handle escape sequences
            if !is_backtick && self.ch == Some('\\') {
                self.read_char(); // Skip the backslash
                if self.ch == None {
                    let location = self.location();
                    return Err(ErrorReporter::lexer_error(
                        location,
                        "Unterminated string literal",
                    ));
                }
            }
            self.read_char();
        }

        if self.ch != Some(quote_char) {
            let location = self.location();
            return Err(ErrorReporter::lexer_error(
                location,
                "Unterminated string literal",
            ));
        }

        let str_content = self.input[position..self.position].to_string();

        // Process escape sequences for regular strings (not backtick strings)
        let final_string = if !is_backtick {
            // Replace escape sequences
            str_content
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\\\", "\\")
                .replace("\\\"", "\"")
                .replace("\\'", "'")
        } else {
            str_content
        };

        Ok(Token::String(final_string))
    }

    /// Read a rune literal (Unicode code point)
    pub fn read_rune(&mut self) -> Result<Token, Error> {
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
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated rune literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\n'));
                }
                Some('t') => {
                    self.read_char(); // Consume 't'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated rune literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\t'));
                }
                Some('r') => {
                    self.read_char(); // Consume 'r'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated rune literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\r'));
                }
                Some('\'') => {
                    self.read_char(); // Consume '\''
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated rune literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\''));
                }
                Some('\\') => {
                    self.read_char(); // Consume '\\'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated rune literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Rune('\\'));
                }
                _ => {
                    let location = self.location();
                    return Err(ErrorReporter::lexer_error(
                        location,
                        &format!("Unknown escape sequence: \\{:?}", escape_char),
                    ));
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
            return Err(ErrorReporter::lexer_error(
                location,
                "Unterminated rune literal",
            ));
        }

        self.read_char(); // Consume closing quote
        Ok(Token::Rune(ch.unwrap()))
    }

    /// Read a hexadecimal number (base 16)
    pub fn read_hex_number(&mut self) -> Result<Token, Error> {
        let position = self.position;
        let start_position = position - 2; // Include the "0x" prefix

        // Read hex digits
        while let Some(c) = self.ch {
            if is_hex_digit(c) {
                self.read_char();
            } else {
                break;
            }
        }

        if position == self.position {
            // No digits were read after 0x
            let location = self.location();
            return Err(ErrorReporter::lexer_error(
                location,
                "Invalid hexadecimal number: no digits after '0x'",
            ));
        }

        let hex_str = &self.input[start_position..self.position];

        // Parse as i64
        match i64::from_str_radix(&hex_str[2..], 16) {
            // Skip the "0x" prefix
            Ok(value) => Ok(Token::Int(value)),
            Err(_) => {
                let location = self.location();
                Err(ErrorReporter::lexer_error(
                    location,
                    &format!("Could not parse hexadecimal: {}", hex_str),
                ))
            }
        }
    }

    /// Read an octal number (base 8)
    pub fn read_octal_number(&mut self) -> Result<Token, Error> {
        let position = self.position;
        let start_position = position - 2; // Include the "0o" prefix

        // Read octal digits
        while let Some(c) = self.ch {
            if is_octal_digit(c) {
                self.read_char();
            } else {
                break;
            }
        }

        if position == self.position {
            // No digits were read after 0o
            let location = self.location();
            return Err(ErrorReporter::lexer_error(
                location,
                "Invalid octal number: no digits after '0o'",
            ));
        }

        let octal_str = &self.input[start_position..self.position];

        // Parse as i64
        match i64::from_str_radix(&octal_str[2..], 8) {
            // Skip the "0o" prefix
            Ok(value) => Ok(Token::Int(value)),
            Err(_) => {
                let location = self.location();
                Err(ErrorReporter::lexer_error(
                    location,
                    &format!("Could not parse octal: {}", octal_str),
                ))
            }
        }
    }

    /// Read a binary number (base 2)
    pub fn read_binary_number(&mut self) -> Result<Token, Error> {
        let position = self.position;
        let start_position = position - 2; // Include the "0b" prefix

        // Read binary digits
        while let Some(c) = self.ch {
            if c == '0' || c == '1' {
                self.read_char();
            } else {
                break;
            }
        }

        if position == self.position {
            // No digits were read after 0b
            let location = self.location();
            return Err(ErrorReporter::lexer_error(
                location,
                "Invalid binary number: no digits after '0b'",
            ));
        }

        let binary_str = &self.input[start_position..self.position];

        // Parse as i64
        match i64::from_str_radix(&binary_str[2..], 2) {
            // Skip the "0b" prefix
            Ok(value) => Ok(Token::Int(value)),
            Err(_) => {
                let location = self.location();
                Err(ErrorReporter::lexer_error(
                    location,
                    &format!("Could not parse binary: {}", binary_str),
                ))
            }
        }
    }

    /// Read a float that starts with a decimal point (e.g., .5)
    pub fn read_float_starting_with_dot(&mut self) -> Result<Token, Error> {
        let position = self.position;
        let mut has_exponent = false;

        // Consume the dot
        self.read_char();

        // Read digits after the decimal point
        while let Some(c) = self.ch {
            if is_digit(c) {
                self.read_char();
            } else if (c == 'e' || c == 'E') && !has_exponent {
                // Handle exponent notation
                has_exponent = true;
                self.read_char(); // consume 'e' or 'E'

                // Check for sign
                if self.ch == Some('+') || self.ch == Some('-') {
                    self.read_char(); // consume sign
                }

                // Must have at least one digit after exponent
                if !self.ch.map_or(false, is_digit) {
                    let location = self.location();
                    return Err(ErrorReporter::lexer_error(
                        location,
                        "Invalid float: exponent has no digits",
                    ));
                }

                // Read exponent digits
                while let Some(c) = self.ch {
                    if is_digit(c) {
                        self.read_char();
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        let float_str = &self.input[position..self.position];

        // Parse as f64
        match float_str.parse::<f64>() {
            Ok(value) => Ok(Token::Float(value)),
            Err(_) => {
                let location = self.location();
                Err(ErrorReporter::lexer_error(
                    location,
                    &format!("Could not parse float: {}", float_str),
                ))
            }
        }
    }

    /// Read a byte literal
    pub fn read_byte(&mut self) -> Result<Token, Error> {
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
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated byte literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\n'));
                }
                Some('t') => {
                    self.read_char(); // Consume 't'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated byte literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\t'));
                }
                Some('r') => {
                    self.read_char(); // Consume 'r'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated byte literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\r'));
                }
                Some('\'') => {
                    self.read_char(); // Consume '\''
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated byte literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\''));
                }
                Some('\\') => {
                    self.read_char(); // Consume '\\'
                    if self.ch != Some('\'') {
                        let location = self.location();
                        return Err(ErrorReporter::lexer_error(
                            location,
                            "Unterminated byte literal",
                        ));
                    }
                    self.read_char(); // Consume closing quote
                    return Ok(Token::Byte(b'\\'));
                }
                _ => {
                    let location = self.location();
                    return Err(ErrorReporter::lexer_error(
                        location,
                        &format!("Unknown escape sequence: \\{:?}", escape_char),
                    ));
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
            return Err(ErrorReporter::lexer_error(
                location,
                &format!(
                    "Byte literal must be ASCII (0-127), got: {:?} ({})",
                    ch, ch_val
                ),
            ));
        }

        self.read_char(); // Move past the character

        if self.ch != Some('\'') {
            let location = self.location();
            return Err(ErrorReporter::lexer_error(
                location,
                "Unterminated byte literal",
            ));
        }

        self.read_char(); // Consume closing quote
        Ok(Token::Byte(ch.unwrap() as u8))
    }

    /// Convert an identifier to a token
    pub fn lookup_identifier(&self, identifier: String) -> Token {
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
            "snack" => Token::Snack,
            "meal" => Token::Meal,
            "lit" => Token::Lit,
            "sip" => Token::Sip,
            "fr" => {
                // Check for "fr fr" comment
                if self.peek_char() == Some(' ')
                    && self.read_position + 1 < self.input.len()
                    && self.input.chars().nth(self.read_position + 1) == Some('f')
                    && self.read_position + 2 < self.input.len()
                    && self.input.chars().nth(self.read_position + 2) == Some('r')
                {
                    Token::LineComment
                } else {
                    Token::Identifier(identifier)
                }
            }
            "no" => {
                // Check for "no cap" block comment start
                if self.peek_char() == Some(' ')
                    && self.read_position + 1 < self.input.len()
                    && self.input.chars().nth(self.read_position + 1) == Some('c')
                    && self.read_position + 2 < self.input.len()
                    && self.input.chars().nth(self.read_position + 2) == Some('a')
                    && self.read_position + 3 < self.input.len()
                    && self.input.chars().nth(self.read_position + 3) == Some('p')
                {
                    Token::BlockCommentStart
                } else {
                    Token::Identifier(identifier)
                }
            }
            "on" => {
                // Check for "on god" block comment end
                if self.peek_char() == Some(' ')
                    && self.read_position + 1 < self.input.len()
                    && self.input.chars().nth(self.read_position + 1) == Some('g')
                    && self.read_position + 2 < self.input.len()
                    && self.input.chars().nth(self.read_position + 2) == Some('o')
                    && self.read_position + 3 < self.input.len()
                    && self.input.chars().nth(self.read_position + 3) == Some('d')
                {
                    Token::BlockCommentEnd
                } else {
                    Token::Identifier(identifier)
                }
            }
            _ => Token::Identifier(identifier),
        }
    }
}
