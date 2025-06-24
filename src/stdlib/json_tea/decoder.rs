/*!
 * JSON Decoder
 * 
 * JSON decoding functionality for CURSED
 */

use crate::error::CursedError;
use crate::runtime::value::Value;
use crate::stdlib::json_tea::{JsonResult, JsonValue};
use crate::stdlib::json_tea::value::unescape_json_string;
use crate::error::Error;
use std::collections::HashMap;
use std::io::Read;
use std::str::from_utf8;

/// JSON decoder for converting JSON to CURSED values
pub struct Decoder<'a> {
    /// Input JSON bytes
    input: &'a [u8],
    /// Current position in input
    position: usize,
    /// Current line number (for error reporting)
    line: usize,
    /// Current column number (for error reporting)
    column: usize,
}

impl<'a> Decoder<'a> {
    /// Create a new decoder
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// Decode JSON to a CURSED Value
    pub fn decode(&mut self) -> JsonResult<Value> {
        self.skip_whitespace();
        let json_value = self.parse_value()?;
        self.skip_whitespace();
        
        if self.position < self.input.len() {
            return Err(CursedError::json_syntax_error(
                "Unexpected content after JSON value".to_string(),
                self.position,
            ));
        }
        
        Ok(json_value.into())
    }
    
    /// Validate JSON without fully parsing
    pub fn validate_only(&mut self) -> JsonResult<()> {
        self.skip_whitespace();
        self.validate_value()?;
        self.skip_whitespace();
        
        if self.position < self.input.len() {
            return Err(CursedError::json_syntax_error(
                "Unexpected content after JSON value".to_string(),
                self.position,
            ));
        }
        
        Ok(())
    }
    
    /// Parse a JSON value
    fn parse_value(&mut self) -> JsonResult<JsonValue> {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Err(CursedError::json_unexpected_eof());
        }
        
        match self.current_byte()? {
            b'n' => self.parse_null(),
            b't' | b'f' => self.parse_bool(),
            b'"' => self.parse_string(),
            b'[' => self.parse_array(),
            b'{' => self.parse_object(),
            b'-' | b'0'..=b'9' => self.parse_number(),
            byte => Err(CursedError::json_syntax_error(
                format!("Unexpected character '{}'", byte as char),
                self.position,
            )),
        }
    }
    
    /// Validate a JSON value without parsing
    fn validate_value(&mut self) -> JsonResult<()> {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Err(CursedError::json_unexpected_eof());
        }
        
        match self.current_byte()? {
            b'n' => self.validate_null(),
            b't' | b'f' => self.validate_bool(),
            b'"' => self.validate_string(),
            b'[' => self.validate_array(),
            b'{' => self.validate_object(),
            b'-' | b'0'..=b'9' => self.validate_number(),
            byte => Err(CursedError::json_syntax_error(
                format!("Unexpected character '{}'", byte as char),
                self.position,
            )),
        }
    }
    
    /// Parse null value
    fn parse_null(&mut self) -> JsonResult<JsonValue> {
        if self.consume_literal(b"null")? {
            Ok(JsonValue::Null)
        } else {
            Err(CursedError::json_syntax_error(
                "Invalid null literal".to_string(),
                self.position,
            ))
        }
    }
    
    /// Validate null value
    fn validate_null(&mut self) -> JsonResult<()> {
        if self.consume_literal(b"null")? {
            Ok(())
        } else {
            Err(CursedError::json_syntax_error(
                "Invalid null literal".to_string(),
                self.position,
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
                "Invalid boolean literal".to_string(),
                self.position,
            ))
        }
    }
    
    /// Validate boolean value
    fn validate_bool(&mut self) -> JsonResult<()> {
        if self.consume_literal(b"true")? || self.consume_literal(b"false")? {
            Ok(())
        } else {
            Err(CursedError::json_syntax_error(
                "Invalid boolean literal".to_string(),
                self.position,
            ))
        }
    }
    
    /// Parse string value
    fn parse_string(&mut self) -> JsonResult<JsonValue> {
        if self.current_byte()? != b'"' {
            return Err(CursedError::json_syntax_error(
                "Expected string".to_string(),
                self.position,
            ));
        }
        
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
                        "Unescaped newline in string".to_string(),
                        self.position,
                    ));
                }
                _ => {
                    escaped = false;
                    self.advance();
                }
            }
        }
        
        Err(CursedError::json_syntax_error(
            "Unterminated string".to_string(),
            self.position,
        ))
    }
    
    /// Validate string value
    fn validate_string(&mut self) -> JsonResult<()> {
        if self.current_byte()? != b'"' {
            return Err(CursedError::json_syntax_error(
                "Expected string".to_string(),
                self.position,
            ));
        }
        
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
                        "Unescaped newline in string".to_string(),
                        self.position,
                    ));
                }
                _ => {
                    escaped = false;
                    self.advance();
                }
            }
        }
        
        Err(CursedError::json_syntax_error(
            "Unterminated string".to_string(),
            self.position,
        ))
    }
    
    /// Parse number value
    fn parse_number(&mut self) -> JsonResult<JsonValue> {
        let start = self.position;
        
        // Handle negative sign
        if self.current_byte()? == b'-' {
            self.advance();
        }
        
        // Parse integer part
        if self.position >= self.input.len() {
            return Err(CursedError::json_syntax_error(
                "Incomplete number".to_string(),
                self.position,
            ));
        }
        
        let first_digit = self.current_byte()?;
        if first_digit == b'0' {
            self.advance();
        } else if first_digit.is_ascii_digit() {
            while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                self.advance();
            }
        } else {
            return Err(CursedError::json_syntax_error(
                "Invalid number format".to_string(),
                self.position,
            ));
        }
        
        // Parse fractional part
        if self.position < self.input.len() && self.current_byte()? == b'.' {
            self.advance();
            
            if self.position >= self.input.len() || !self.current_byte()?.is_ascii_digit() {
                return Err(CursedError::json_syntax_error(
                    "Invalid number: missing digits after decimal point".to_string(),
                    self.position,
                ));
            }
            
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
                        "Invalid number: missing digits in exponent".to_string(),
                        self.position,
                    ));
                }
                
                while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                    self.advance();
                }
            }
        }
        
        let number_str = from_utf8(&self.input[start..self.position])
            .map_err(|e| CursedError::json_invalid_utf8(e.to_string()))?;
        
        let number = number_str.parse::<f64>()
            .map_err(|_| CursedError::json_invalid_number(number_str.to_string()))?;
        
        Ok(JsonValue::Number(number))
    }
    
    /// Validate number value
    fn validate_number(&mut self) -> JsonResult<()> {
        let start = self.position;
        
        // Handle negative sign
        if self.current_byte()? == b'-' {
            self.advance();
        }
        
        // Parse integer part
        if self.position >= self.input.len() {
            return Err(CursedError::json_syntax_error(
                "Incomplete number".to_string(),
                self.position,
            ));
        }
        
        let first_digit = self.current_byte()?;
        if first_digit == b'0' {
            self.advance();
        } else if first_digit.is_ascii_digit() {
            while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                self.advance();
            }
        } else {
            return Err(CursedError::json_syntax_error(
                "Invalid number format".to_string(),
                self.position,
            ));
        }
        
        // Parse fractional part
        if self.position < self.input.len() && self.current_byte()? == b'.' {
            self.advance();
            
            if self.position >= self.input.len() || !self.current_byte()?.is_ascii_digit() {
                return Err(CursedError::json_syntax_error(
                    "Invalid number: missing digits after decimal point".to_string(),
                    self.position,
                ));
            }
            
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
                        "Invalid number: missing digits in exponent".to_string(),
                        self.position,
                    ));
                }
                
                while self.position < self.input.len() && self.current_byte()?.is_ascii_digit() {
                    self.advance();
                }
            }
        }
        
        Ok(())
    }
    
    /// Parse array value
    fn parse_array(&mut self) -> JsonResult<JsonValue> {
        if self.current_byte()? != b'[' {
            return Err(CursedError::json_syntax_error(
                "Expected array".to_string(),
                self.position,
            ));
        }
        
        self.advance(); // Skip opening bracket
        self.skip_whitespace();
        
        let mut elements = Vec::new();
        
        // Handle empty array
        if self.position < self.input.len() && self.current_byte()? == b']' {
            self.advance();
            return Ok(JsonValue::Array(elements));
        }
        
        loop {
            elements.push(self.parse_value()?);
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                return Err(CursedError::json_syntax_error(
                    "Unterminated array".to_string(),
                    self.position,
                ));
            }
            
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
                        format!("Expected ',' or ']' in array, found '{}'", byte as char),
                        self.position,
                    ));
                }
            }
        }
        
        Ok(JsonValue::Array(elements))
    }
    
    /// Validate array value
    fn validate_array(&mut self) -> JsonResult<()> {
        if self.current_byte()? != b'[' {
            return Err(CursedError::json_syntax_error(
                "Expected array".to_string(),
                self.position,
            ));
        }
        
        self.advance(); // Skip opening bracket
        self.skip_whitespace();
        
        // Handle empty array
        if self.position < self.input.len() && self.current_byte()? == b']' {
            self.advance();
            return Ok(());
        }
        
        loop {
            self.validate_value()?;
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                return Err(CursedError::json_syntax_error(
                    "Unterminated array".to_string(),
                    self.position,
                ));
            }
            
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
                        format!("Expected ',' or ']' in array, found '{}'", byte as char),
                        self.position,
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// Parse object value
    fn parse_object(&mut self) -> JsonResult<JsonValue> {
        if self.current_byte()? != b'{' {
            return Err(CursedError::json_syntax_error(
                "Expected object".to_string(),
                self.position,
            ));
        }
        
        self.advance(); // Skip opening brace
        self.skip_whitespace();
        
        let mut object = HashMap::new();
        
        // Handle empty object
        if self.position < self.input.len() && self.current_byte()? == b'}' {
            self.advance();
            return Ok(JsonValue::Object(object));
        }
        
        loop {
            // Parse key
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => {
                    return Err(CursedError::json_syntax_error(
                        "Object key must be a string".to_string(),
                        self.position,
                    ));
                }
            };
            
            self.skip_whitespace();
            
            // Expect colon
            if self.position >= self.input.len() || self.current_byte()? != b':' {
                return Err(CursedError::json_syntax_error(
                    "Expected ':' after object key".to_string(),
                    self.position,
                ));
            }
            self.advance();
            
            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);
            
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                return Err(CursedError::json_syntax_error(
                    "Unterminated object".to_string(),
                    self.position,
                ));
            }
            
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
                        format!("Expected ',' or '}}' in object, found '{}'", byte as char),
                        self.position,
                    ));
                }
            }
        }
        
        Ok(JsonValue::Object(object))
    }
    
    /// Validate object value
    fn validate_object(&mut self) -> JsonResult<()> {
        if self.current_byte()? != b'{' {
            return Err(CursedError::json_syntax_error(
                "Expected object".to_string(),
                self.position,
            ));
        }
        
        self.advance(); // Skip opening brace
        self.skip_whitespace();
        
        // Handle empty object
        if self.position < self.input.len() && self.current_byte()? == b'}' {
            self.advance();
            return Ok(());
        }
        
        loop {
            // Parse key
            self.validate_string()?;
            self.skip_whitespace();
            
            // Expect colon
            if self.position >= self.input.len() || self.current_byte()? != b':' {
                return Err(CursedError::json_syntax_error(
                    "Expected ':' after object key".to_string(),
                    self.position,
                ));
            }
            self.advance();
            
            // Parse value
            self.validate_value()?;
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                return Err(CursedError::json_syntax_error(
                    "Unterminated object".to_string(),
                    self.position,
                ));
            }
            
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
                        format!("Expected ',' or '}}' in object, found '{}'", byte as char),
                        self.position,
                    ));
                }
            }
        }
        
        Ok(())
    }
    
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
                b' ' | b'\t' | b'\n' | b'\r' => self.advance(),
                _ => break,
            }
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
    reader: R,
    buffer: Vec<u8>,
    position: usize,
    line: usize,
    column: usize,
}

impl<R: Read> StreamingDecoder<R> {
    /// Create a new streaming decoder
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
            position: 0,
            line: 1,
            column: 1,
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
    }
    
    /// Check if there's more data to read
    pub fn has_more(&mut self) -> JsonResult<bool> {
        self.ensure_data()?;
        Ok(self.position < self.buffer.len())
    }
    
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decode_primitives() {
        let mut decoder = Decoder::new(b"null");
        assert_eq!(decoder.decode().unwrap(), Value::Null);
        
        let mut decoder = Decoder::new(b"true");
        assert_eq!(decoder.decode().unwrap(), Value::Boolean(true));
        
        let mut decoder = Decoder::new(b"false");
        assert_eq!(decoder.decode().unwrap(), Value::Boolean(false));
        
        let mut decoder = Decoder::new(b"42");
        assert_eq!(decoder.decode().unwrap(), Value::Number(42.0));
        
        let mut decoder = Decoder::new(b"3.14");
        assert_eq!(decoder.decode().unwrap(), Value::Number(3.14));
        
        let mut decoder = Decoder::new(b"\"hello\"");
        assert_eq!(decoder.decode().unwrap(), Value::String("hello".to_string()));
    }
    
    #[test]
    fn test_decode_array() {
        let mut decoder = Decoder::new(br#"[1, "test", true, null]"#);
        let result = decoder.decode().unwrap();
        
        match result {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 4);
                assert_eq!(arr[0], Value::Number(1.0));
                assert_eq!(arr[1], Value::String("test".to_string()));
                assert_eq!(arr[2], Value::Boolean(true));
                assert_eq!(arr[3], Value::Null);
            }
            _ => panic!("Expected array"),
        }
    }
    
    #[test]
    fn test_decode_object() {
        let mut decoder = Decoder::new(br#"{"name": "Alice", "age": 30}"#);
        let result = decoder.decode().unwrap();
        
        match result {
            Value::Object(obj) => {
                assert_eq!(obj.len(), 2);
                assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
                assert_eq!(obj.get("age"), Some(&Value::Number(30.0)));
            }
            _ => panic!("Expected object"),
        }
    }
    
    #[test]
    fn test_decode_nested() {
        let json = br#"{"users": [{"name": "Alice", "active": true}, {"name": "Bob", "active": false}]}"#;
        let mut decoder = Decoder::new(json);
        let result = decoder.decode().unwrap();
        
        match result {
            Value::Object(obj) => {
                match obj.get("users") {
                    Some(Value::Array(users)) => {
                        assert_eq!(users.len(), 2);
                        // Further nested checks...
                    }
                    _ => panic!("Expected users array"),
                }
            }
            _ => panic!("Expected object"),
        }
    }
    
    #[test]
    fn test_decode_string_escapes() {
        let mut decoder = Decoder::new(br#""Hello \"world\"\nNew line""#);
        let result = decoder.decode().unwrap();
        
        match result {
            Value::String(s) => {
                assert_eq!(s, "Hello \"world\"\nNew line");
            }
            _ => panic!("Expected string"),
        }
    }
    
    #[test]
    fn test_decode_numbers() {
        let test_cases = vec![
            ("0", 0.0),
            ("42", 42.0),
            ("-17", -17.0),
            ("3.14", 3.14),
            ("-2.71", -2.71),
            ("1e10", 1e10),
            ("1.5e-3", 1.5e-3),
            ("-1.2E+4", -1.2E+4),
        ];
        
        for (json, expected) in test_cases {
            let mut decoder = Decoder::new(json.as_bytes());
            let result = decoder.decode().unwrap();
            
            match result {
                Value::Number(n) => {
                    assert!((n - expected).abs() < f64::EPSILON, "Expected {}, got {}", expected, n);
                }
                _ => panic!("Expected number for input '{}'", json),
            }
        }
    }
    
    #[test]
    fn test_validation() {
        let valid_cases = vec![
            b"null".as_slice(),
            b"true",
            b"false",
            b"42",
            b"\"hello\"",
            b"[]",
            b"{}",
            br#"{"key": "value"}"#,
            br#"[1, 2, 3]"#,
        ];
        
        for case in valid_cases {
            let mut decoder = Decoder::new(case);
            assert!(decoder.validate_only().is_ok(), "Should be valid: {:?}", std::str::from_utf8(case));
        }
        
        let invalid_cases = vec![
            b"nul".as_slice(),
            b"tru",
            b"fals",
            b"42.".as_slice(),
            b"\"unterminated",
            b"[1, 2,",
            b"{\"key\": }",
            b"{\"key\" \"value\"}",
        ];
        
        for case in invalid_cases {
            let mut decoder = Decoder::new(case);
            assert!(decoder.validate_only().is_err(), "Should be invalid: {:?}", std::str::from_utf8(case));
        }
    }
    
    #[test]
    fn test_streaming_decoder() {
        use std::io::Cursor;
        
        let json_data = br#"{"name": "Alice"}{"age": 30}"#;
        let cursor = Cursor::new(json_data);
        let mut decoder = StreamingDecoder::new(cursor);
        
        // This is a simplified test - in reality, streaming JSON parsing
        // is more complex and would need proper framing
        let result = decoder.decode();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_error_positions() {
        let mut decoder = Decoder::new(b"[1, 2, }");
        let result = decoder.decode();
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("position"));
    }
}
