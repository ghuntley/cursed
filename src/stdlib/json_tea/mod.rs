use crate::error::Error;
/// JSON Tea - JSON encoding and decoding for CURSED
/// 
/// This module provides JSON encoding and decoding functionality compatible with
/// the encode_mood interfaces and CURSED's type system.

use crate::error::CursedError;
use crate::runtime::value::Value;
use std::collections::HashMap;
use std::io::{Read, Write};

// Re-export all public items
pub use encoder::*;
pub use decoder::*;
pub use error::*;
pub use value::*;

// Module declarations
pub mod encoder;
pub mod decoder;
pub mod error;
pub mod value;

// Type aliases for convenience
pub type JsonResult<T> = std::result::Result<T, JsonError>;
pub type JsonObject = HashMap<String, JsonValue>;
pub type JsonArray = Vec<JsonValue>;

/// Main JSON marshaling function - encode a Value to JSON bytes
pub fn marshal(v: &Value) -> JsonResult<Vec<u8>> {
    let mut encoder = Encoder::new();
    encoder.encode(v)
}

/// JSON marshaling with indentation for pretty printing
pub fn marshal_indent(v: &Value, prefix: &str, indent: &str) -> JsonResult<Vec<u8>> {
    let mut encoder = Encoder::new_with_indent(prefix.to_string(), indent.to_string());
    encoder.encode(v)
}

/// Main JSON unmarshaling function - decode JSON bytes to a Value
pub fn unmarshal(data: &[u8], v: &mut Value) -> JsonResult<()> {
    let mut decoder = Decoder::new(data);
    *v = decoder.decode()?;
    Ok(())
}

/// Validate JSON data without parsing
pub fn valid(data: &[u8]) -> bool {
    match std::str::from_utf8(data) {
        Ok(s) => {
            let mut decoder = Decoder::new(data);
            decoder.validate_only().is_ok()
        },
        Err(_) => false,
    }
}

/// Create a new streaming encoder
pub fn new_encoder<W: Write>(writer: W) -> StreamingEncoder<W> {
    StreamingEncoder::new(writer)
}

/// Create a new streaming decoder
pub fn new_decoder<R: Read>(reader: R) -> StreamingDecoder<R> {
    StreamingDecoder::new(reader)
}

/// Convenience function to marshal a Value to a JSON string
pub fn marshal_to_string(v: &Value) -> JsonResult<String> {
    let bytes = marshal(v)?;
    String::from_utf8(bytes).map_err(|e| {
        CursedError::json_error(format!("Invalid UTF-8 in JSON output: {}", e))
    })
}

/// Convenience function to unmarshal from a JSON string
pub fn unmarshal_from_string(s: &str, v: &mut Value) -> JsonResult<()> {
    unmarshal(s.as_bytes(), v)
}

/// JSON tag parsing utilities
pub mod tags {
    use std::collections::HashMap;
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct JsonTag {
        pub name: Option<String>,
        pub omit_empty: bool,
        pub skip: bool,
        pub string: bool,
    }
    
    impl JsonTag {
        pub fn parse(tag: &str) -> Self {
            let mut result = JsonTag {
                name: None,
                omit_empty: false,
                skip: false,
                string: false,
            };
            
            if tag == "-" {
                result.skip = true;
                return result;
            }
            
            let parts: Vec<&str> = tag.split(',').collect();
            if !parts.is_empty() && !parts[0].is_empty() {
                result.name = Some(parts[0].to_string());
            }
            
            for part in parts.iter().skip(1) {
                match part.trim() {
                    "omitempty" => result.omit_empty = true,
                    "string" => result.string = true,
                    _ => {} // Ignore unknown options
                }
            }
            
            result
        }
        
        pub fn effective_name(&self, field_name: &str) -> Option<String> {
            if self.skip {
                None
            } else {
                Some(self.name.as_ref().unwrap_or(&field_name.to_string()).clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_marshal_unmarshal_basic() {
        let original = Value::String("Hello, World!".to_string());
        let json_bytes = marshal(&original).expect("Marshal failed");
        
        let mut decoded = Value::Null;
        unmarshal(&json_bytes, &mut decoded).expect("Unmarshal failed");
        
        assert_eq!(original, decoded);
    }
    
    #[test]
    fn test_marshal_with_indent() {
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::String("Alice".to_string()));
        obj.insert("age".to_string(), Value::Number(30.0));
        
        let original = Value::Object(obj);
        let json_bytes = marshal_indent(&original, "", "  ").expect("Marshal indent failed");
        let json_str = String::from_utf8(json_bytes).expect("Invalid UTF-8");
        
        assert!(json_str.contains("  \"name\""));
        assert!(json_str.contains("  \"age\""));
    }
    
    #[test]
    fn test_valid_json() {
        let valid_json = br#"{"name": "Alice", "age": 30}"#;
        let invalid_json = br#"{"name": "Alice", "age":}"#;
        
        assert!(valid(valid_json));
        assert!(!valid(invalid_json));
    }
    
    #[test]
    fn test_json_tag_parsing() {
        let tag1 = tags::JsonTag::parse("name");
        assert_eq!(tag1.name, Some("name".to_string()));
        assert!(!tag1.omit_empty);
        
        let tag2 = tags::JsonTag::parse("name,omitempty");
        assert_eq!(tag2.name, Some("name".to_string()));
        assert!(tag2.omit_empty);
        
        let tag3 = tags::JsonTag::parse("-");
        assert!(tag3.skip);
        
        let tag4 = tags::JsonTag::parse(",omitempty,string");
        assert_eq!(tag4.name, None);
        assert!(tag4.omit_empty);
        assert!(tag4.string);
    }
    
    #[test]
    fn test_marshal_unmarshal_array() {
        let arr = vec![
            Value::String("hello".to_string()),
            Value::Number(42.0),
            Value::Boolean(true),
            Value::Null,
        ];
        let original = Value::Array(arr);
        
        let json_bytes = marshal(&original).expect("Marshal failed");
        let mut decoded = Value::Null;
        unmarshal(&json_bytes, &mut decoded).expect("Unmarshal failed");
        
        assert_eq!(original, decoded);
    }
    
    #[test]
    fn test_marshal_unmarshal_object() {
        let mut obj = HashMap::new();
        obj.insert("string".to_string(), Value::String("test".to_string()));
        obj.insert("number".to_string(), Value::Number(3.14));
        obj.insert("boolean".to_string(), Value::Boolean(false));
        obj.insert("null".to_string(), Value::Null);
        
        let original = Value::Object(obj);
        
        let json_bytes = marshal(&original).expect("Marshal failed");
        let mut decoded = Value::Null;
        unmarshal(&json_bytes, &mut decoded).expect("Unmarshal failed");
        
        assert_eq!(original, decoded);
    }
}
