/*!
 * JSON Encoder
 * 
 * JSON encoding functionality for CURSED
 */

use crate::error::CursedError;
use crate::runtime::value::Value;
use crate::stdlib::json_tea::{JsonResult, JsonValue, tags::JsonTag};
use crate::stdlib::json_tea::value::escape_json_string;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::fmt::Write as FmtWrite;

/// JSON encoder for converting CURSED values to JSON
pub struct Encoder {
    /// Whether to pretty-print the output
    pretty: bool,
    /// Prefix for each line (for pretty printing)
    prefix: String,
    /// Indentation string (for pretty printing)
    indent: String,
    /// Current indentation level
    current_indent: usize,
    /// Track object references to detect circular references
    reference_stack: HashSet<usize>,
}

impl Encoder {
    /// Create a new encoder with compact output
    pub fn new() -> Self {
        Self {
            pretty: false,
            prefix: String::new(),
            indent: String::new(),
            current_indent: 0,
            reference_stack: HashSet::new(),
        }
    }
    
    /// Create a new encoder with pretty-printed output
    pub fn new_with_indent(prefix: String, indent: String) -> Self {
        Self {
            pretty: true,
            prefix,
            indent,
            current_indent: 0,
            reference_stack: HashSet::new(),
        }
    }
    
    /// Encode a Value to JSON bytes
    pub fn encode(&mut self, value: &Value) -> JsonResult<Vec<u8>> {
        let mut output = Vec::new();
        self.encode_value(value, &mut output)?;
        Ok(output)
    }
    
    /// Encode a Value to a writer
    pub fn encode_to_writer<W: Write>(&mut self, value: &Value, writer: &mut W) -> JsonResult<()> {
        self.encode_value(value, writer)
    }
    
    /// Internal method to encode a value
    fn encode_value<W: Write>(&mut self, value: &Value, writer: &mut W) -> JsonResult<()> {
        match value {
            Value::Null => writer.write_all(b"null")?,
            Value::Boolean(b) => {
                if *b {
                    writer.write_all(b"true")?;
                } else {
                    writer.write_all(b"false")?;
                }
            }
            Value::Number(n) => {
                if n.is_finite() {
                    if n.fract() == 0.0 && *n <= i64::MAX as f64 && *n >= i64::MIN as f64 {
                        write!(writer, "{}", *n as i64)?;
                    } else {
                        write!(writer, "{}", n)?;
                    }
                } else if n.is_nan() {
                    writer.write_all(b"null")?; // JSON doesn't support NaN
                } else if n.is_infinite() {
                    writer.write_all(b"null")?; // JSON doesn't support infinity
                }
            }
            Value::String(s) => {
                writer.write_all(b"\"")?;
                writer.write_all(escape_json_string(s).as_bytes())?;
                writer.write_all(b"\"")?;
            }
            Value::Array(arr) => {
                self.encode_array(arr, writer)?;
            }
            Value::Object(obj) => {
                self.encode_object(obj, writer)?;
            }
            Value::Function(_) => {
                return Err(CursedError::json_unsupported_type("function".to_string()));
            }
            Value::Channel(_) => {
                return Err(CursedError::json_unsupported_type("channel".to_string()));
            }
            Value::Interface { .. } => {
                return Err(CursedError::json_unsupported_type("interface".to_string()));
            }
            Value::Reference(_) => {
                return Err(CursedError::json_unsupported_type("reference".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Encode an array
    fn encode_array<W: Write>(&mut self, arr: &[Value], writer: &mut W) -> JsonResult<()> {
        writer.write_all(b"[")?;
        
        if !arr.is_empty() {
            if self.pretty {
                self.current_indent += 1;
            }
            
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    writer.write_all(b",")?;
                }
                
                if self.pretty {
                    writer.write_all(b"\n")?;
                    self.write_indent(writer)?;
                }
                
                self.encode_value(item, writer)?;
            }
            
            if self.pretty {
                self.current_indent -= 1;
                writer.write_all(b"\n")?;
                self.write_indent(writer)?;
            }
        }
        
        writer.write_all(b"]")?;
        Ok(())
    }
    
    /// Encode an object
    fn encode_object<W: Write>(&mut self, obj: &HashMap<String, Value>, writer: &mut W) -> JsonResult<()> {
        writer.write_all(b"{")?;
        
        if !obj.is_empty() {
            if self.pretty {
                self.current_indent += 1;
            }
            
            let mut first = true;
            for (key, value) in obj {
                if !first {
                    writer.write_all(b",")?;
                }
                
                if self.pretty {
                    writer.write_all(b"\n")?;
                    self.write_indent(writer)?;
                }
                
                // Write key
                writer.write_all(b"\"")?;
                writer.write_all(escape_json_string(key).as_bytes())?;
                writer.write_all(b"\"")?;
                writer.write_all(b":")?;
                
                if self.pretty {
                    writer.write_all(b" ")?;
                }
                
                // Write value
                self.encode_value(value, writer)?;
                
                first = false;
            }
            
            if self.pretty {
                self.current_indent -= 1;
                writer.write_all(b"\n")?;
                self.write_indent(writer)?;
            }
        }
        
        writer.write_all(b"}")?;
        Ok(())
    }
    
    /// Write current indentation
    fn write_indent<W: Write>(&self, writer: &mut W) -> JsonResult<()> {
        writer.write_all(self.prefix.as_bytes())?;
        for _ in 0..self.current_indent {
            writer.write_all(self.indent.as_bytes())?;
        }
        Ok(())
    }
}

/// Streaming encoder for writing JSON to a stream
pub struct StreamingEncoder<W: Write> {
    writer: W,
    encoder: Encoder,
    first_item: bool,
    in_array: bool,
    in_object: bool,
}

impl<W: Write> StreamingEncoder<W> {
    /// Create a new streaming encoder
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            encoder: Encoder::new(),
            first_item: true,
            in_array: false,
            in_object: false,
        }
    }
    
    /// Create a new streaming encoder with pretty printing
    pub fn new_with_indent(writer: W, prefix: String, indent: String) -> Self {
        Self {
            writer,
            encoder: Encoder::new_with_indent(prefix, indent),
            first_item: true,
            in_array: false,
            in_object: false,
        }
    }
    
    /// Start writing an array
    pub fn begin_array(&mut self) -> JsonResult<()> {
        if !self.first_item {
            self.writer.write_all(b",")?;
        }
        self.writer.write_all(b"[")?;
        self.in_array = true;
        self.first_item = true;
        Ok(())
    }
    
    /// End writing an array
    pub fn end_array(&mut self) -> JsonResult<()> {
        self.writer.write_all(b"]")?;
        self.in_array = false;
        self.first_item = false;
        Ok(())
    }
    
    /// Start writing an object
    pub fn begin_object(&mut self) -> JsonResult<()> {
        if !self.first_item {
            self.writer.write_all(b",")?;
        }
        self.writer.write_all(b"{")?;
        self.in_object = true;
        self.first_item = true;
        Ok(())
    }
    
    /// End writing an object
    pub fn end_object(&mut self) -> JsonResult<()> {
        self.writer.write_all(b"}")?;
        self.in_object = false;
        self.first_item = false;
        Ok(())
    }
    
    /// Write a key (only valid in object context)
    pub fn write_key(&mut self, key: &str) -> JsonResult<()> {
        if !self.in_object {
            return Err(CursedError::json_custom_error("Cannot write key outside of object".to_string()));
        }
        
        if !self.first_item {
            self.writer.write_all(b",")?;
        }
        
        self.writer.write_all(b"\"")?;
        self.writer.write_all(escape_json_string(key).as_bytes())?;
        self.writer.write_all(b"\":")?;
        
        self.first_item = false;
        Ok(())
    }
    
    /// Write a value
    pub fn write_value(&mut self, value: &Value) -> JsonResult<()> {
        if self.in_array && !self.first_item {
            self.writer.write_all(b",")?;
        }
        
        self.encoder.encode_value(value, &mut self.writer)?;
        
        if self.in_array {
            self.first_item = false;
        }
        
        Ok(())
    }
    
    /// Flush the writer
    pub fn flush(&mut self) -> JsonResult<()> {
        self.writer.flush()?;
        Ok(())
    }
    
    /// Finish writing and return the writer
    pub fn finish(mut self) -> JsonResult<W> {
        self.flush()?;
        Ok(self.writer)
    }
}

/// Helper functions for encoding specific data types
impl Encoder {
    /// Encode a struct with JSON tags
    pub fn encode_struct_with_tags<W: Write>(
        &mut self,
        fields: &HashMap<String, (Value, Option<String>)>,
        writer: &mut W,
    ) -> JsonResult<()> {
        writer.write_all(b"{")?;
        
        if self.pretty {
            self.current_indent += 1;
        }
        
        let mut first = true;
        
        for (field_name, (value, tag)) in fields {
            let json_tag = tag.as_ref().map(|t| JsonTag::parse(t)).unwrap_or_else(|| {
                JsonTag {
                    name: Some(field_name.clone()),
                    omit_empty: false,
                    skip: false,
                    string: false,
                }
            });
            
            // Skip fields marked with "-"
            if json_tag.skip {
                continue;
            }
            
            // Skip empty fields if omitempty is set
            if json_tag.omit_empty && self.is_empty_value(value) {
                continue;
            }
            
            let effective_name = json_tag.effective_name(field_name);
            if let Some(name) = effective_name {
                if !first {
                    writer.write_all(b",")?;
                }
                
                if self.pretty {
                    writer.write_all(b"\n")?;
                    self.write_indent(writer)?;
                }
                
                // Write key
                writer.write_all(b"\"")?;
                writer.write_all(escape_json_string(&name).as_bytes())?;
                writer.write_all(b"\"")?;
                writer.write_all(b":")?;
                
                if self.pretty {
                    writer.write_all(b" ")?;
                }
                
                // Handle string tag - encode as JSON string even if it's a number
                if json_tag.string {
                    let value_str = match value {
                        Value::Number(n) => n.to_string(),
                        Value::Boolean(b) => b.to_string(),
                        _ => {
                            // For other types, encode normally then convert to string
                            let mut temp_buf = Vec::new();
                            self.encode_value(value, &mut temp_buf)?;
                            String::from_utf8(temp_buf)
                                .map_err(|e| CursedError::json_invalid_utf8(e.to_string()))?
                        }
                    };
                    
                    writer.write_all(b"\"")?;
                    writer.write_all(escape_json_string(&value_str).as_bytes())?;
                    writer.write_all(b"\"")?;
                } else {
                    self.encode_value(value, writer)?;
                }
                
                first = false;
            }
        }
        
        if self.pretty && !first {
            self.current_indent -= 1;
            writer.write_all(b"\n")?;
            self.write_indent(writer)?;
        } else if self.pretty {
            self.current_indent -= 1;
        }
        
        writer.write_all(b"}")?;
        Ok(())
    }
    
    /// Check if a value is considered empty for omitempty purposes
    fn is_empty_value(&self, value: &Value) -> bool {
        match value {
            Value::Null => true,
            Value::Boolean(false) => true,
            Value::Number(n) => *n == 0.0,
            Value::String(s) => s.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert I/O errors to JSON errors
impl From<std::io::Error> for CursedError {
    fn from(err: std::io::Error) -> Self {
        CursedError::json_io_error(err.to_string())
    }
}

/// Convert fmt errors to JSON errors
impl From<std::fmt::Error> for CursedError {
    fn from(err: std::fmt::Error) -> Self {
        CursedError::json_io_error(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_encode_primitives() {
        let mut encoder = Encoder::new();
        
        assert_eq!(encoder.encode(&Value::Null).unwrap(), b"null");
        assert_eq!(encoder.encode(&Value::Boolean(true)).unwrap(), b"true");
        assert_eq!(encoder.encode(&Value::Boolean(false)).unwrap(), b"false");
        assert_eq!(encoder.encode(&Value::Number(42.0)).unwrap(), b"42");
        assert_eq!(encoder.encode(&Value::Number(3.14)).unwrap(), b"3.14");
        assert_eq!(encoder.encode(&Value::String("hello".to_string())).unwrap(), b"\"hello\"");
    }
    
    #[test]
    fn test_encode_array() {
        let mut encoder = Encoder::new();
        let arr = vec![
            Value::Number(1.0),
            Value::String("test".to_string()),
            Value::Boolean(true),
        ];
        let value = Value::Array(arr);
        
        let result = encoder.encode(&value).unwrap();
        let json_str = String::from_utf8(result).unwrap();
        
        assert_eq!(json_str, r#"[1,"test",true]"#);
    }
    
    #[test]
    fn test_encode_object() {
        let mut encoder = Encoder::new();
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::String("Alice".to_string()));
        obj.insert("age".to_string(), Value::Number(30.0));
        
        let value = Value::Object(obj);
        let result = encoder.encode(&value).unwrap();
        let json_str = String::from_utf8(result).unwrap();
        
        // Note: HashMap order is not guaranteed, so we check both possibilities
        assert!(json_str == r#"{"name":"Alice","age":30}"# || 
                json_str == r#"{"age":30,"name":"Alice"}"#);
    }
    
    #[test]
    fn test_encode_with_indent() {
        let mut encoder = Encoder::new_with_indent("".to_string(), "  ".to_string());
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::String("Alice".to_string()));
        
        let value = Value::Object(obj);
        let result = encoder.encode(&value).unwrap();
        let json_str = String::from_utf8(result).unwrap();
        
        assert!(json_str.contains("  \"name\""));
        assert!(json_str.contains("\n"));
    }
    
    #[test]
    fn test_streaming_encoder() {
        let mut output = Vec::new();
        let mut encoder = StreamingEncoder::new(&mut output);
        
        encoder.begin_object().unwrap();
        encoder.write_key("name").unwrap();
        encoder.write_value(&Value::String("Alice".to_string())).unwrap();
        encoder.write_key("age").unwrap();
        encoder.write_value(&Value::Number(30.0)).unwrap();
        encoder.end_object().unwrap();
        
        let json_str = String::from_utf8(output).unwrap();
        assert_eq!(json_str, r#"{"name":"Alice","age":30}"#);
    }
    
    #[test]
    fn test_streaming_encoder_array() {
        let mut output = Vec::new();
        let mut encoder = StreamingEncoder::new(&mut output);
        
        encoder.begin_array().unwrap();
        encoder.write_value(&Value::Number(1.0)).unwrap();
        encoder.write_value(&Value::Number(2.0)).unwrap();
        encoder.write_value(&Value::Number(3.0)).unwrap();
        encoder.end_array().unwrap();
        
        let json_str = String::from_utf8(output).unwrap();
        assert_eq!(json_str, "[1,2,3]");
    }
    
    #[test]
    fn test_unsupported_types() {
        let mut encoder = Encoder::new();
        
        // These should return errors
        assert!(encoder.encode(&Value::Function(std::sync::Arc::new(|_| Ok(Value::Null)))).is_err());
        assert!(encoder.encode(&Value::Channel(std::sync::Arc::new(std::sync::Mutex::new(
            crate::runtime::channel::Channel::new()
        )))).is_err());
    }
    
    #[test]
    fn test_special_numbers() {
        let mut encoder = Encoder::new();
        
        // NaN and infinity should encode as null
        assert_eq!(encoder.encode(&Value::Number(f64::NAN)).unwrap(), b"null");
        assert_eq!(encoder.encode(&Value::Number(f64::INFINITY)).unwrap(), b"null");
        assert_eq!(encoder.encode(&Value::Number(f64::NEG_INFINITY)).unwrap(), b"null");
    }
    
    #[test]
    fn test_string_escaping() {
        let mut encoder = Encoder::new();
        
        let test_string = "Hello \"world\"\nNew line\tTab\\Backslash";
        let value = Value::String(test_string.to_string());
        let result = encoder.encode(&value).unwrap();
        let json_str = String::from_utf8(result).unwrap();
        
        assert!(json_str.contains("\\\""));
        assert!(json_str.contains("\\n"));
        assert!(json_str.contains("\\t"));
        assert!(json_str.contains("\\\\"));
    }
}
