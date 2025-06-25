use crate::error::CursedError;
/// JSON Tea - JSON encoding and decoding for CURSED
/// 
/// This module provides JSON encoding and decoding functionality compatible with
/// the encode_mood interfaces and CURSED's type system.

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
/// JSON marshaling with indentation for pretty printing
pub fn marshal_indent(v: &Value, prefix: &str, indent: &str) -> JsonResult<Vec<u8>> {
    let mut encoder = Encoder::new_with_indent(prefix.to_string(), indent.to_string());
    encoder.encode(v)
/// Main JSON unmarshaling function - decode JSON bytes to a Value
pub fn unmarshal(data: &[u8], v: &mut Value) -> JsonResult<()> {
    let mut decoder = Decoder::new(data);
    *v = decoder.decode()?;
    Ok(())
/// Validate JSON data without parsing
pub fn valid(data: &[u8]) -> bool {
    match std::str::from_utf8(data) {
        Ok(s) => {
            let mut decoder = Decoder::new(data);
            decoder.validate_only().is_ok()
    }
}

/// Create a new streaming encoder
pub fn new_encoder<W: Write>(writer: W) -> StreamingEncoder<W> {
    StreamingEncoder::new(writer)
/// Create a new streaming decoder
pub fn new_decoder<R: Read>(reader: R) -> StreamingDecoder<R> {
    StreamingDecoder::new(reader)
/// Convenience function to marshal a Value to a JSON string
pub fn marshal_to_string(v: &Value) -> JsonResult<String> {
    let bytes = marshal(v)?;
    String::from_utf8(bytes).map_err(|e| {
        CursedError::json_error(format!("Invalid UTF-8 in JSON output: {}", e))
    })
/// Convenience function to unmarshal from a JSON string
pub fn unmarshal_from_string(s: &str, v: &mut Value) -> JsonResult<()> {
    unmarshal(s.as_bytes(), v)
/// JSON tag parsing utilities
pub mod tags {
    use std::collections::HashMap;
    
    #[derive(Debug, Clone, PartialEq)]
    pub struct JsonTag {
    impl JsonTag {
        pub fn parse(tag: &str) -> Self {
            let mut result = JsonTag {
            
            if tag == "-" {
                result.skip = true;
                return result;
            let parts: Vec<&str> = tag.split(',').collect();
            if !parts.is_empty() && !parts[0].is_empty() {
                result.name = Some(parts[0].to_string());
            for part in parts.iter().skip(1) {
                match part.trim() {
                    _ => {} // Ignore unknown options
                }
            }
            
            result
        pub fn effective_name(&self, field_name: &str) -> Option<String> {
            if self.skip {
                None
            } else {
                Some(self.name.as_ref().unwrap_or(&field_name.to_string()).clone())
            }
        }
    }
}

