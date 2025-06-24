use crate::error::Error;
/// JSON request/response handling utilities
use std::collections::HashMap;
use std::fmt;

/// JSON handler for request/response processing
pub struct JsonHandler {
    pretty: bool,
    indent: usize,
}

impl JsonHandler {
    /// Create new JSON handler
    pub fn new() -> Self {
        Self {
            pretty: false,
            indent: 2,
        }
    }

    /// Enable pretty printing
    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }

    /// Set indentation for pretty printing
    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    }

    /// Parse JSON string to value
    pub fn parse(&self, json_str: &str) -> Result<(), Error> {
        JsonParser::new(json_str).parse()
    }

    /// Serialize value to JSON string
    pub fn stringify(&self, value: &JsonValue) -> Result<(), Error> {
        if self.pretty {
            self.stringify_pretty(value, 0)
        } else {
            Ok(value.to_string())
        }
    }

    /// Serialize value to pretty JSON string
    fn stringify_pretty(&self, value: &JsonValue, depth: usize) -> Result<(), Error> {
        let indent_str = " ".repeat(depth * self.indent);
        let next_indent = " ".repeat((depth + 1) * self.indent);

        match value {
            JsonValue::Object(map) => {
                if map.is_empty() {
                    return Ok("{}".to_string());
                }

                let mut result = "{\n".to_string();
                let mut first = true;
                for (key, val) in map {
                    if !first {
                        result.push_str(",\n");
                    }
                    result.push_str(&format!("{}\"{}\":", next_indent, key));
                    let val_str = self.stringify_pretty(val, depth + 1)?;
                    if matches!(val, JsonValue::Object(_) | JsonValue::Array(_)) {
                        result.push_str(&format!(" {}", val_str));
                    } else {
                        result.push_str(&format!(" {}", val_str));
                    }
                    first = false;
                }
                result.push_str(&format!("\n{}}}", indent_str));
                Ok(result)
            }
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    return Ok("[]".to_string());
                }

                let mut result = "[\n".to_string();
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        result.push_str(",\n");
                    }
                    let val_str = self.stringify_pretty(val, depth + 1)?;
                    result.push_str(&format!("{}{}", next_indent, val_str));
                }
                result.push_str(&format!("\n{}]", indent_str));
                Ok(result)
            }
            _ => Ok(value.to_string()),
        }
    }

    /// Extract JSON from request body
    pub fn from_request_body(&self, body: &str, content_type: &str) -> Result<(), Error> {
        if !content_type.contains("application/json") {
            return Err(JsonError::InvalidContentType(content_type.to_string()));
        }

        self.parse(body)
    }

    /// Create JSON response
    pub fn create_response(&self, value: &JsonValue) -> Result<(), Error> {
        let body = self.stringify(value)?;
        Ok(JsonResponse {
            body,
            status: 200,
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Content-Length".to_string(), body.len().to_string()),
            ],
        })
    }

    /// Create JSON error response
    pub fn create_error_response(&self, message: &str, status: u16) -> JsonResponse {
        let error_obj = JsonValue::Object({
            let mut map = HashMap::new();
            map.insert("error".to_string(), JsonValue::String(message.to_string()));
            map.insert("status".to_string(), JsonValue::Number(status as f64));
            map
        });

        let body = self.stringify(&error_obj).unwrap_or_else(|_| {
            format!(r#"{{"error":"{}","status":{}}}"#, message, status)
        });

        JsonResponse {
            body,
            status,
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Content-Length".to_string(), body.len().to_string()),
            ],
        }
    }
}

impl Default for JsonHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// JSON value types
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            JsonValue::String(s) => write!(f, "\"{}\"", escape_json_string(s)),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, val) in map {
                    if !first {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", escape_json_string(key), val)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

impl JsonValue {
    /// Get value as string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get value as number
    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get value as boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get value as array
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Get value as object
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(map) => Some(map),
            _ => None,
        }
    }

    /// Check if value is null
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    /// Get object property
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(map) => map.get(key),
            _ => None,
        }
    }

    /// Get array index
    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => None,
        }
    }
}

/// JSON response structure
pub struct JsonResponse {
    pub body: String,
    pub status: u16,
    pub headers: Vec<(String, String)>,
}

impl JsonResponse {
    /// Create success response
    pub fn ok(value: &JsonValue) -> Self {
        JsonHandler::new().create_response(value).unwrap_or_else(|_| {
            JsonHandler::new().create_error_response("Failed to serialize response", 500)
        })
    }

    /// Create error response
    pub fn error(message: &str, status: u16) -> Self {
        JsonHandler::new().create_error_response(message, status)
    }

    /// Create not found response
    pub fn not_found() -> Self {
        Self::error("Not Found", 404)
    }

    /// Create bad request response
    pub fn bad_request(message: &str) -> Self {
        Self::error(message, 400)
    }

    /// Create internal server error response
    pub fn internal_error() -> Self {
        Self::error("Internal Server Error", 500)
    }

    /// Add header to response
    pub fn with_header(mut self, name: String, value: String) -> Self {
        self.headers.push((name, value));
        self
    }

    /// Set CORS headers
    pub fn with_cors(self) -> Self {
        self.with_header("Access-Control-Allow-Origin".to_string(), "*".to_string())
            .with_header("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string())
            .with_header("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string())
    }
}

/// JSON parsing errors
#[derive(Debug)]
pub enum JsonError {
    UnexpectedCharacter(char, usize),
    UnexpectedEnd,
    InvalidNumber(String),
    InvalidEscape(char),
    InvalidContentType(String),
    SerializationError(String),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonError::UnexpectedCharacter(ch, pos) => {
                write!(f, "Unexpected character '{}' at position {}", ch, pos)
            }
            JsonError::UnexpectedEnd => write!(f, "Unexpected end of input"),
            JsonError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            JsonError::InvalidEscape(ch) => write!(f, "Invalid escape sequence: \\{}", ch),
            JsonError::InvalidContentType(ct) => write!(f, "Invalid content type: {}", ct),
            JsonError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for JsonError {}

/// Simple JSON parser
struct JsonParser {
    input: Vec<char>,
    pos: usize,
}

impl JsonParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn parse(&mut self) -> Result<(), Error> {
        self.skip_whitespace();
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<(), Error> {
        self.skip_whitespace();
        
        if self.pos >= self.input.len() {
            return Err(JsonError::UnexpectedEnd);
        }

        match self.input[self.pos] {
            'n' => self.parse_null(),
            't' | 'f' => self.parse_bool(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            '-' | '0'..='9' => self.parse_number(),
            ch => Err(JsonError::UnexpectedCharacter(ch, self.pos)),
        }
    }

    fn parse_null(&mut self) -> Result<(), Error> {
        if self.input[self.pos..].iter().take(4).collect::<String>() == "null" {
            self.pos += 4;
            Ok(JsonValue::Null)
        } else {
            Err(JsonError::UnexpectedCharacter(self.input[self.pos], self.pos))
        }
    }

    fn parse_bool(&mut self) -> Result<(), Error> {
        if self.input[self.pos..].iter().take(4).collect::<String>() == "true" {
            self.pos += 4;
            Ok(JsonValue::Bool(true))
        } else if self.input[self.pos..].iter().take(5).collect::<String>() == "false" {
            self.pos += 5;
            Ok(JsonValue::Bool(false))
        } else {
            Err(JsonError::UnexpectedCharacter(self.input[self.pos], self.pos))
        }
    }

    fn parse_string(&mut self) -> Result<(), Error> {
        self.pos += 1; // Skip opening quote
        let mut string = String::new();

        while self.pos < self.input.len() {
            match self.input[self.pos] {
                '"' => {
                    self.pos += 1;
                    return Ok(JsonValue::String(string));
                }
                '\\' => {
                    self.pos += 1;
                    if self.pos >= self.input.len() {
                        return Err(JsonError::UnexpectedEnd);
                    }
                    match self.input[self.pos] {
                        '"' => string.push('"'),
                        '\\' => string.push('\\'),
                        '/' => string.push('/'),
                        'b' => string.push('\x08'),
                        'f' => string.push('\x0C'),
                        'n' => string.push('\n'),
                        'r' => string.push('\r'),
                        't' => string.push('\t'),
                        ch => return Err(JsonError::InvalidEscape(ch)),
                    }
                    self.pos += 1;
                }
                ch => {
                    string.push(ch);
                    self.pos += 1;
                }
            }
        }

        Err(JsonError::UnexpectedEnd)
    }

    fn parse_number(&mut self) -> Result<(), Error> {
        let start = self.pos;
        
        if self.input[self.pos] == '-' {
            self.pos += 1;
        }

        if self.pos >= self.input.len() || !self.input[self.pos].is_ascii_digit() {
            return Err(JsonError::InvalidNumber("Expected digit".to_string()));
        }

        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        if self.pos < self.input.len() && self.input[self.pos] == '.' {
            self.pos += 1;
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        if self.pos < self.input.len() && (self.input[self.pos] == 'e' || self.input[self.pos] == 'E') {
            self.pos += 1;
            if self.pos < self.input.len() && (self.input[self.pos] == '+' || self.input[self.pos] == '-') {
                self.pos += 1;
            }
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        let number_str: String = self.input[start..self.pos].iter().collect();
        match number_str.parse::<f64>() {
            Ok(n) => Ok(JsonValue::Number(n)),
            Err(_) => Err(JsonError::InvalidNumber(number_str)),
        }
    }

    fn parse_array(&mut self) -> Result<(), Error> {
        self.pos += 1; // Skip '['
        self.skip_whitespace();

        let mut array = Vec::new();

        if self.pos < self.input.len() && self.input[self.pos] == ']' {
            self.pos += 1;
            return Ok(JsonValue::Array(array));
        }

        loop {
            array.push(self.parse_value()?);
            self.skip_whitespace();

            if self.pos >= self.input.len() {
                return Err(JsonError::UnexpectedEnd);
            }

            match self.input[self.pos] {
                ',' => {
                    self.pos += 1;
                    self.skip_whitespace();
                }
                ']' => {
                    self.pos += 1;
                    break;
                }
                ch => return Err(JsonError::UnexpectedCharacter(ch, self.pos)),
            }
        }

        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<(), Error> {
        self.pos += 1; // Skip '{'
        self.skip_whitespace();

        let mut object = HashMap::new();

        if self.pos < self.input.len() && self.input[self.pos] == '}' {
            self.pos += 1;
            return Ok(JsonValue::Object(object));
        }

        loop {
            // Parse key
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => return Err(JsonError::UnexpectedCharacter(self.input[self.pos], self.pos)),
            };

            self.skip_whitespace();

            if self.pos >= self.input.len() || self.input[self.pos] != ':' {
                return Err(JsonError::UnexpectedCharacter(
                    self.input.get(self.pos).copied().unwrap_or('\0'),
                    self.pos,
                ));
            }

            self.pos += 1; // Skip ':'
            self.skip_whitespace();

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);

            self.skip_whitespace();

            if self.pos >= self.input.len() {
                return Err(JsonError::UnexpectedEnd);
            }

            match self.input[self.pos] {
                ',' => {
                    self.pos += 1;
                    self.skip_whitespace();
                }
                '}' => {
                    self.pos += 1;
                    break;
                }
                ch => return Err(JsonError::UnexpectedCharacter(ch, self.pos)),
            }
        }

        Ok(JsonValue::Object(object))
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                ' ' | '\t' | '\n' | '\r' => self.pos += 1,
                _ => break,
            }
        }
    }
}

/// Escape JSON string
fn escape_json_string(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            ch if ch.is_control() => {
                result.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch => result.push(ch),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parsing() {
        let handler = JsonHandler::new();

        // Test null
        assert_eq!(handler.parse("null").unwrap(), JsonValue::Null);

        // Test boolean
        assert_eq!(handler.parse("true").unwrap(), JsonValue::Bool(true));
        assert_eq!(handler.parse("false").unwrap(), JsonValue::Bool(false));

        // Test number
        assert_eq!(handler.parse("42").unwrap(), JsonValue::Number(42.0));
        assert_eq!(handler.parse("-3.14").unwrap(), JsonValue::Number(-3.14));

        // Test string
        assert_eq!(handler.parse("\"hello\"").unwrap(), JsonValue::String("hello".to_string()));

        // Test array
        let arr = handler.parse("[1, 2, 3]").unwrap();
        assert_eq!(arr, JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]));

        // Test object
        let obj = handler.parse(r#"{"name": "John", "age": 30}"#).unwrap();
        let mut expected = HashMap::new();
        expected.insert("name".to_string(), JsonValue::String("John".to_string()));
        expected.insert("age".to_string(), JsonValue::Number(30.0));
        assert_eq!(obj, JsonValue::Object(expected));
    }

    #[test]
    fn test_json_serialization() {
        let handler = JsonHandler::new();

        // Test basic values
        assert_eq!(handler.stringify(&JsonValue::Null).unwrap(), "null");
        assert_eq!(handler.stringify(&JsonValue::Bool(true)).unwrap(), "true");
        assert_eq!(handler.stringify(&JsonValue::Number(42.0)).unwrap(), "42");
        assert_eq!(handler.stringify(&JsonValue::String("hello".to_string())).unwrap(), "\"hello\"");

        // Test array
        let arr = JsonValue::Array(Vec::from([JsonValue::Number(1.0), JsonValue::Number(2.0)]));
        assert_eq!(handler.stringify(&arr).unwrap(), "[1,2]");

        // Test object
        let mut obj_map = HashMap::new();
        obj_map.insert("name".to_string(), JsonValue::String("John".to_string()));
        let obj = JsonValue::Object(obj_map);
        assert_eq!(handler.stringify(&obj).unwrap(), r#"{"name":"John"}"#);
    }

    #[test]
    fn test_pretty_printing() {
        let handler = JsonHandler::new().pretty();
        
        let mut obj_map = HashMap::new();
        obj_map.insert("name".to_string(), JsonValue::String("John".to_string()));
        obj_map.insert("age".to_string(), JsonValue::Number(30.0));
        let obj = JsonValue::Object(obj_map);

        let pretty = handler.stringify(&obj).unwrap();
        assert!(pretty.contains("\n"));
        assert!(pretty.contains("  "));
    }
}
