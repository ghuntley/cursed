use crate::error::CursedError;
/// JSON request/response handling utilities
use std::collections::HashMap;
use std::fmt;

/// JSON handler for request/response processing
pub struct JsonHandler {
impl JsonHandler {
    /// Create new JSON handler
    pub fn new() -> Self {
        Self {
        }
    }

    /// Enable pretty printing
    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    /// Set indentation for pretty printing
    pub fn with_indent(mut self, indent: usize) -> Self {
        self.indent = indent;
        self
    /// Parse JSON string to value
    pub fn parse(&self, json_str: &str) -> crate::error::Result<()> {
        JsonParser::new(json_str).parse()
    /// Serialize value to JSON string
    pub fn stringify(&self, value: &JsonValue) -> crate::error::Result<()> {
        if self.pretty {
            self.stringify_pretty(value, 0)
        } else {
            Ok(value.to_string())
        }
    }

    /// Serialize value to pretty JSON string
    fn stringify_pretty(&self, value: &JsonValue, depth: usize) -> crate::error::Result<()> {
        let indent_str = " ".repeat(depth * self.indent);
        let next_indent = " ".repeat((depth + 1) * self.indent);

        match value {
            JsonValue::Object(map) => {
                if map.is_empty() {
                    return Ok("{}".to_string());
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
        }
    }

    /// Extract JSON from request body
    pub fn from_request_body(&self, body: &str, content_type: &str) -> crate::error::Result<()> {
        if !content_type.contains("application/json") {
            return Err(JsonError::InvalidContentType(content_type.to_string()));
        self.parse(body)
    /// Create JSON response
    pub fn create_response(&self, value: &JsonValue) -> crate::error::Result<()> {
        let body = self.stringify(value)?;
        Ok(JsonResponse {
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
        })
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
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
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
impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonValue::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
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
        }
    }

    /// Get value as number
    pub fn as_number(&self) -> Option<f64> {
        match self {
        }
    }

    /// Get value as boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
        }
    }

    /// Get value as array
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
        }
    }

    /// Get value as object
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
        }
    }

    /// Check if value is null
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    /// Get object property
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
        }
    }

    /// Get array index
    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
        }
    }
/// JSON response structure
pub struct JsonResponse {
impl JsonResponse {
    /// Create success response
    pub fn ok(value: &JsonValue) -> Self {
        JsonHandler::new().create_response(value).unwrap_or_else(|_| {
            JsonHandler::new().create_error_response("Failed to serialize response", 500)
        })
    /// Create error response
    pub fn error(message: &str, status: u16) -> Self {
        JsonHandler::new().create_error_response(message, status)
    /// Create not found response
    pub fn not_found() -> Self {
        Self::error("Not Found", 404)
    /// Create bad request response
    pub fn bad_request(message: &str) -> Self {
        Self::error(message, 400)
    /// Create internal server error response
    pub fn internal_error() -> Self {
        Self::error("Internal Server CursedError", 500)
    /// Add header to response
    pub fn with_header(mut self, name: String, value: String) -> Self {
        self.headers.push((name, value));
        self
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
// impl fmt::Display for JsonError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             JsonError::UnexpectedCharacter(ch, pos) => {
//                 write!(f, "Unexpected character '{}' at position {}", ch, pos)
//             }
//             JsonError::UnexpectedEnd => write!(f, "Unexpected end of input"),
//             JsonError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
//             JsonError::InvalidEscape(ch) => write!(f, "Invalid escape sequence: \\{}", ch),
//             JsonError::InvalidContentType(ct) => write!(f, "Invalid content type: {}", ct),
//             JsonError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for JsonError {}
// 
/// Simple JSON parser
struct JsonParser {
impl JsonParser {
    fn new(input: &str) -> Self {
        Self {
        }
    }

    fn parse(&mut self) -> crate::error::Result<()> {
        self.skip_whitespace();
        self.parse_value()
    fn parse_value(&mut self) -> crate::error::Result<()> {
        self.skip_whitespace();
        
        if self.pos >= self.input.len() {
            return Err(JsonError::UnexpectedEnd);
        match self.input[self.pos] {
        }
    }

    fn parse_null(&mut self) -> crate::error::Result<()> {
        if self.input[self.pos..].iter().take(4).collect::<String>() == "null" {
            self.pos += 4;
            Ok(JsonValue::Null)
        } else {
            Err(JsonError::UnexpectedCharacter(self.input[self.pos], self.pos))
        }
    }

    fn parse_bool(&mut self) -> crate::error::Result<()> {
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

    fn parse_string(&mut self) -> crate::error::Result<()> {
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
                        '/' => string.push('/'),
                    }
                    self.pos += 1;
                }
                ch => {
                    string.push(ch);
                    self.pos += 1;
                }
            }
        Err(JsonError::UnexpectedEnd)
    fn parse_number(&mut self) -> crate::error::Result<()> {
        let start = self.pos;
        
        if self.input[self.pos] == '-' {
            self.pos += 1;
        if self.pos >= self.input.len() || !self.input[self.pos].is_ascii_digit() {
            return Err(JsonError::InvalidNumber("Expected digit".to_string()));
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
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
        }
    }

    fn parse_array(&mut self) -> crate::error::Result<()> {
        self.pos += 1; // Skip '['
        self.skip_whitespace();

        let mut array = Vec::new();

        if self.pos < self.input.len() && self.input[self.pos] == ']' {
            self.pos += 1;
            return Ok(JsonValue::Array(array));
        loop {
            array.push(self.parse_value()?);
            self.skip_whitespace();

            if self.pos >= self.input.len() {
                return Err(JsonError::UnexpectedEnd);
            match self.input[self.pos] {
                ',' => {
                    self.pos += 1;
                    self.skip_whitespace();
                }
                ']' => {
                    self.pos += 1;
                    break;
                }
            }
        }

        Ok(JsonValue::Array(array))
    fn parse_object(&mut self) -> crate::error::Result<()> {
        self.pos += 1; // Skip '{'
        self.skip_whitespace();

        let mut object = HashMap::new();

        if self.pos < self.input.len() && self.input[self.pos] == '}' {
            self.pos += 1;
            return Ok(JsonValue::Object(object));
        loop {
            // Parse key
            let key = match self.parse_string()? {

            self.skip_whitespace();

            if self.pos >= self.input.len() || self.input[self.pos] != ':' {
                return Err(JsonError::UnexpectedCharacter(
                ));
            self.pos += 1; // Skip ':'
            self.skip_whitespace();

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);

            self.skip_whitespace();

            if self.pos >= self.input.len() {
                return Err(JsonError::UnexpectedEnd);
            match self.input[self.pos] {
                ',' => {
                    self.pos += 1;
                    self.skip_whitespace();
                }
                '}' => {
                    self.pos += 1;
                    break;
                }
            }
        }

        Ok(JsonValue::Object(object))
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            match self.input[self.pos] {
            }
        }
    }
}

/// Escape JSON string
fn escape_json_string(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            ch if ch.is_control() => {
                result.push_str(&format!("\\u{:04x}", ch as u32));
            }
        }
    }
    result
