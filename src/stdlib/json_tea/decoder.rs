/*!
 * JSON Decoder
 * 
 * JSON decoding functionality for CURSED
 */

use crate::error::CursedError;
use crate::runtime::value::Value;
// use crate::stdlib::json_tea::{JsonResult, JsonValue};
// use crate::stdlib::json_tea::value::unescape_json_string;
use std::collections::HashMap;
use std::io::Read;
use std::str::from_utf8;

/// JSON decoder for converting JSON to CURSED values
pub struct Decoder<'a> {
    /// Input JSON bytes
    /// Current position in input
    /// Current line number (for error reporting)
    /// Current column number (for error reporting)
impl<'a> Decoder<'a> {
    /// Create a new decoder
    pub fn new(input: &'a [u8]) -> Self {
        Self {
        }
    }
    
    /// Decode JSON to a CURSED Value
    pub fn decode(&mut self) -> JsonResult<Value> {
        self.skip_whitespace();
        let json_value = self.parse_value()?;
        self.skip_whitespace();
        
        if self.position < self.input.len() {
            return Err(CursedError::json_syntax_error(
            ));
        Ok(json_value.into())
    /// Validate JSON without fully parsing
    pub fn validate_only(&mut self) -> JsonResult<()> {
        self.skip_whitespace();
        self.validate_value()?;
        self.skip_whitespace();
        
        if self.position < self.input.len() {
            return Err(CursedError::json_syntax_error(
            ));
        Ok(())
    /// Parse a JSON value
    fn parse_value(&mut self) -> JsonResult<JsonValue> {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Err(CursedError::json_unexpected_eof());
        match self.current_byte()? {
            byte => Err(CursedError::json_syntax_error(
        }
    }
    
    /// Validate a JSON value without parsing
    fn validate_value(&mut self) -> JsonResult<()> {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Err(CursedError::json_unexpected_eof());
        match self.current_byte()? {
            byte => Err(CursedError::json_syntax_error(
        }
    }
    
    /// Parse null value
    fn parse_null(&mut self) -> JsonResult<JsonValue> {
        if self.consume_literal(b"null")? {
            Ok(JsonValue::Null)
        } else {
            Err(CursedError::json_syntax_error(
            ))
        }
    }
    
    /// Validate null value
    fn validate_null(&mut self) -> JsonResult<()> {
        if self.consume_literal(b"null")? {
            Ok(())
        } else {
            Err(CursedError::json_syntax_error(
            ))
        }
    }
    
    /// Parse boolean value
    fn parse_bool(&mut self) -> JsonResult<JsonValue> {
        if self.consume_literal(b"true")? {
            Ok(JsonValue::Bool(true))
        } else if self.consume_literal(b"false")? {
            Ok(JsonValue::Bool(false))
        } else {
            Err(CursedError::json_syntax_error(
            ))
        }
    }
    
    /// Validate boolean value
    fn validate_bool(&mut self) -> JsonResult<()> {
        if self.consume_literal(b"true")? || self.consume_literal(b"false")? {
            Ok(())
        } else {
            Err(CursedError::json_syntax_error(
            ))
        }
    }
    
    /// Parse string value
    fn parse_string(&mut self) -> JsonResult<JsonValue> {
        if self.current_byte()? != b'"' {
            return Err(CursedError::json_syntax_error(
            ));
        self.advance(); // Skip opening quote
        let start = self.position;
        let mut escaped = false;
        
        while self.position < self.input.len() {
            match self.current_byte()? {
                b'"' if !escaped => {
                    let string_content = &self.input[start..self.position];
                    let string_str = from_utf8(string_content)
                        .map_err(|e| CursedError::json_invalid_utf8(e.to_string()))?;
                    
                    self.advance(); // Skip closing quote
                    
                    let unescaped = unescape_json_string(string_str)?;
                    return Ok(JsonValue::String(unescaped));
                }
                b'\\' if !escaped => {
                    escaped = true;
                    self.advance();
                }
                b'\n' | b'\r' if !escaped => {
                    return Err(CursedError::json_syntax_error(
                    ));
                }
                _ => {
                    escaped = false;
                    self.advance();
                }
            }
        Err(CursedError::json_syntax_error(
        ))
    /// Validate string value
    fn validate_string(&mut self) -> JsonResult<()> {
        if self.current_byte()? != b'"' {
            return Err(CursedError::json_syntax_error(
            ));
        self.advance(); // Skip opening quote
        let mut escaped = false;
        
        while self.position < self.input.len() {
            match self.current_byte()? {
                b'"' if !escaped => {
                    self.advance(); // Skip closing quote
                    return Ok(());
                }
                b'\\' if !escaped => {
                    escaped = true;
                    self.advance();
                }
                b'\n' | b'\r' if !escaped => {
                    return Err(CursedError::json_syntax_error(
                    ));
                }
                _ => {
                    escaped = false;
                    self.advance();
                }
            }
        Err(CursedError::json_syntax_error(
        ))
    /// Parse number value
    fn parse_number(&mut self) -> JsonResult<JsonValue> {
        let start = self.position;
        
        // Handle negative sign
        if self.current_byte()? == b'-' {
            self.advance();
        // Parse integer part
        if self.position >= self.input.len() {
            return Err(CursedError::json_syntax_error(
            ));
        let first_digit = self.current_byte()?;
        if first_digit == b'0' {
            self.advance();
        } else if first_digit.is_ascii_digit() {
            while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                self.advance();
            }
        } else {
            return Err(CursedError::json_syntax_error(
            ));
        // Parse fractional part
        if self.position < self.input.len() && self.current_byte()? == b'.' {
            self.advance();
            
            if self.position >= self.input.len() || !self.current_byte()?.is_ascii_digit() {
                return Err(CursedError::json_syntax_error(
                ));
            while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                self.advance();
            }
        }
        
        // Parse exponent part
        if self.position < self.input.len() {
            let exp_char = self.current_byte()?;
            if exp_char == b'e' || exp_char == b'E' {
                self.advance();
                
                if self.position < self.input.len() {
                    let sign_char = self.current_byte()?;
                    if sign_char == b'+' || sign_char == b'-' {
                        self.advance();
                    }
                }
                
                if self.position >= self.input.len() || !self.current_byte()?.is_ascii_digit() {
                    return Err(CursedError::json_syntax_error(
                    ));
                while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                    self.advance();
                }
            }
        let number_str = from_utf8(&self.input[start..self.position])
            .map_err(|e| CursedError::json_invalid_utf8(e.to_string()))?;
        
        let number = number_str.parse::<f64>()
            .map_err(|_| CursedError::json_invalid_number(number_str.to_string()))?;
        
        Ok(JsonValue::Number(number))
    /// Validate number value
    fn validate_number(&mut self) -> JsonResult<()> {
        let start = self.position;
        
        // Handle negative sign
        if self.current_byte()? == b'-' {
            self.advance();
        // Parse integer part
        if self.position >= self.input.len() {
            return Err(CursedError::json_syntax_error(
            ));
        let first_digit = self.current_byte()?;
        if first_digit == b'0' {
            self.advance();
        } else if first_digit.is_ascii_digit() {
            while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                self.advance();
            }
        } else {
            return Err(CursedError::json_syntax_error(
            ));
        // Parse fractional part
        if self.position < self.input.len() && self.current_byte()? == b'.' {
            self.advance();
            
            if self.position >= self.input.len() || !self.current_byte()?.is_ascii_digit() {
                return Err(CursedError::json_syntax_error(
                ));
            while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                self.advance();
            }
        }
        
        // Parse exponent part
        if self.position < self.input.len() {
            let exp_char = self.current_byte()?;
            if exp_char == b'e' || exp_char == b'E' {
                self.advance();
                
                if self.position < self.input.len() {
                    let sign_char = self.current_byte()?;
                    if sign_char == b'+' || sign_char == b'-' {
                        self.advance();
                    }
                }
                
                if self.position >= self.input.len() || !self.current_byte()?.is_ascii_digit() {
                    return Err(CursedError::json_syntax_error(
                    ));
                while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                    self.advance();
                }
            }
        Ok(())
    /// Parse array value
    fn parse_array(&mut self) -> JsonResult<JsonValue> {
        if self.current_byte()? != b'[' {
            return Err(CursedError::json_syntax_error(
            ));
        self.advance(); // Skip opening bracket
        self.skip_whitespace();
        
        let mut elements = Vec::new();
        
        // Handle empty array
        if self.position < self.input.len() && self.current_byte()? == b']' {
            self.advance();
            return Ok(JsonValue::Array(elements));
        loop {
            elements.push(self.parse_value()?);
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                return Err(CursedError::json_syntax_error(
                ));
            match self.current_byte()? {
                b',' => {
                    self.advance();
                    self.skip_whitespace();
                }
                b']' => {
                    self.advance();
                    break;
                }
                byte => {
                    return Err(CursedError::json_syntax_error(
                    ));
                }
            }
        Ok(JsonValue::Array(elements))
    /// Validate array value
    fn validate_array(&mut self) -> JsonResult<()> {
        if self.current_byte()? != b'[' {
            return Err(CursedError::json_syntax_error(
            ));
        self.advance(); // Skip opening bracket
        self.skip_whitespace();
        
        // Handle empty array
        if self.position < self.input.len() && self.current_byte()? == b']' {
            self.advance();
            return Ok(());
        loop {
            self.validate_value()?;
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                return Err(CursedError::json_syntax_error(
                ));
            match self.current_byte()? {
                b',' => {
                    self.advance();
                    self.skip_whitespace();
                }
                b']' => {
                    self.advance();
                    break;
                }
                byte => {
                    return Err(CursedError::json_syntax_error(
                    ));
                }
            }
        Ok(())
    /// Parse object value
    fn parse_object(&mut self) -> JsonResult<JsonValue> {
        if self.current_byte()? != b'{' {
            return Err(CursedError::json_syntax_error(
            ));
        self.advance(); // Skip opening brace
        self.skip_whitespace();
        
        let mut object = HashMap::new();
        
        // Handle empty object
        if self.position < self.input.len() && self.current_byte()? == b'}' {
            self.advance();
            return Ok(JsonValue::Object(object));
        loop {
            // Parse key
            let key = match self.parse_string()? {
                _ => {
                    return Err(CursedError::json_syntax_error(
                    ));
                }
            
            self.skip_whitespace();
            
            // Expect colon
            if self.position >= self.input.len() || self.current_byte()? != b':' {
                return Err(CursedError::json_syntax_error(
                ));
            }
            self.advance();
            
            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);
            
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                return Err(CursedError::json_syntax_error(
                ));
            match self.current_byte()? {
                b',' => {
                    self.advance();
                    self.skip_whitespace();
                }
                b'}' => {
                    self.advance();
                    break;
                }
                byte => {
                    return Err(CursedError::json_syntax_error(
                    ));
                }
            }
        Ok(JsonValue::Object(object))
    /// Validate object value
    fn validate_object(&mut self) -> JsonResult<()> {
        if self.current_byte()? != b'{' {
            return Err(CursedError::json_syntax_error(
            ));
        self.advance(); // Skip opening brace
        self.skip_whitespace();
        
        // Handle empty object
        if self.position < self.input.len() && self.current_byte()? == b'}' {
            self.advance();
            return Ok(());
        loop {
            // Parse key
            self.validate_string()?;
            self.skip_whitespace();
            
            // Expect colon
            if self.position >= self.input.len() || self.current_byte()? != b':' {
                return Err(CursedError::json_syntax_error(
                ));
            }
            self.advance();
            
            // Parse value
            self.validate_value()?;
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                return Err(CursedError::json_syntax_error(
                ));
            match self.current_byte()? {
                b',' => {
                    self.advance();
                    self.skip_whitespace();
                }
                b'}' => {
                    self.advance();
                    break;
                }
                byte => {
                    return Err(CursedError::json_syntax_error(
                    ));
                }
            }
        Ok(())
    /// Helper methods
    fn current_byte(&self) -> JsonResult<u8> {
        if self.position < self.input.len() {
            Ok(self.input[self.position])
        } else {
            Err(CursedError::json_unexpected_eof())
        }
    }
    
    fn advance(&mut self) {
        if self.position < self.input.len() {
            if self.input[self.position] == b'\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            match self.input[self.position] {
            }
        }
    fn consume_literal(&mut self, literal: &[u8]) -> JsonResult<bool> {
        if self.position + literal.len() <= self.input.len() {
            let slice = &self.input[self.position..self.position + literal.len()];
            if slice == literal {
                for _ in 0..literal.len() {
                    self.advance();
                }
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// Streaming decoder for reading JSON from a stream
pub struct StreamingDecoder<R: Read> {
impl<R: Read> StreamingDecoder<R> {
    /// Create a new streaming decoder
    pub fn new(reader: R) -> Self {
        Self {
        }
    }
    
    /// Decode the next JSON value from the stream
    pub fn decode(&mut self) -> JsonResult<Value> {
        // Read more data if needed
        self.ensure_data()?;
        
        let mut decoder = Decoder::new(&self.buffer[self.position..]);
        let value = decoder.decode()?;
        
        // Update position in our buffer
        self.position += decoder.position;
        
        Ok(value)
    /// Check if there's more data to read
    pub fn has_more(&mut self) -> JsonResult<bool> {
        self.ensure_data()?;
        Ok(self.position < self.buffer.len())
    /// Ensure we have data in the buffer
    fn ensure_data(&mut self) -> JsonResult<()> {
        if self.position >= self.buffer.len() {
            // Read more data
            let mut temp_buf = [0u8; 8192];
            let bytes_read = self.reader.read(&mut temp_buf)?;
            
            if bytes_read > 0 {
                self.buffer.clear();
                self.buffer.extend_from_slice(&temp_buf[..bytes_read]);
                self.position = 0;
            }
        }
        Ok(())
    }
}

