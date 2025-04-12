//! The json_tea package provides JSON encoding and decoding functionality.
//!
//! This module is equivalent to the encoding/json package in Go, providing functions
//! for converting between CURSED objects and JSON strings. It supports serialization
//! and deserialization of the basic data types including null, booleans, numbers,
//! strings, arrays, and hash tables (objects in JSON).
//!
//! # Features
//!
//! - JSON serialization via `marshal`
//! - JSON deserialization via `unmarshal`
//! - Support for all basic CURSED data types
//!
//! # Examples
//!
//! ```cursed
//! import "json_tea"
//!
//! // Creating a data structure
//! user := map[string]interface{}{
//!     "name": "Zoomer",
//!     "age": 21,
//!     "vibes": true,
//!     "skills": ["coding", "TikTok"],
//! }
//!
//! // Marshaling to JSON
//! jsonStr := json_tea.marshal(user)
//! vibez.println(jsonStr)
//!
//! // Unmarshaling from JSON
//! var newUser map[string]interface{}
//! json_tea.unmarshal(jsonStr, &newUser)
//! ```

use crate::error::Error;
use crate::object::Object;
use std::collections::HashMap;
use std::rc::Rc;

/// Serializes a CURSED object into a JSON string.
///
/// This function converts CURSED data structures into their JSON representation.
/// It supports the following object types:
/// - Null → null
/// - Integer → number
/// - Float → number
/// - Boolean → boolean
/// - String → string
/// - Array → array
/// - HashTable → object
///
/// # Arguments
///
/// * `args[0]` - The CURSED object to serialize
///
/// # Returns
///
/// A String Object containing the JSON representation
///
/// # Errors
///
/// Returns a Runtime error if:
/// - No argument is provided
/// - The object contains unsupported types for JSON serialization
pub fn marshal(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("marshal requires 1 argument".to_string()));
    }

    // Convert object to JSON string
    let json = match &*args[0] {
        Object::Null => "null".to_string(),
        Object::Integer(i) => i.to_string(),
        Object::Float(f) => f.to_string(),
        Object::Boolean(b) => b.to_string(),
        Object::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        Object::Array(arr) => {
            let mut json = String::from("[");
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    json.push_str(",");
                }
                // Recursive marshal
                let item_json = marshal(&[Rc::new(item.clone())])?;
                if let Object::String(s) = &*item_json {
                    json.push_str(s);
                }
            }
            json.push_str("]");
            json
        }
        Object::HashTable(map) => {
            let mut json = String::from("{");
            let mut first = true;

            for (key, value) in map {
                if !first {
                    json.push_str(",");
                }
                first = false;

                // Add key
                json.push_str(&format!("\"{}\": ", key));

                // Recursive marshal for value
                let value_json = marshal(&[Rc::new(value.clone())])?;
                if let Object::String(s) = &*value_json {
                    json.push_str(s);
                }
            }

            json.push_str("}");
            json
        }
        _ => {
            return Err(Error::Runtime(format!(
                "Cannot marshal type {} to JSON",
                args[0].type_name()
            )))
        }
    };

    Ok(Rc::new(Object::String(json)))
}

/// Parses a JSON string into a CURSED object.
///
/// This function converts JSON data into CURSED data structures.
/// It supports parsing all valid JSON constructs into their CURSED equivalents:
/// - null u2192 Null
/// - numbers u2192 Integer or Float
/// - booleans u2192 Boolean
/// - strings u2192 String
/// - arrays u2192 Array
/// - objects u2192 HashTable
///
/// Note: The current implementation is simplified and returns a placeholder object.
///
/// # Arguments
///
/// * `args[0]` - The JSON string to parse as a String Object
/// * `args[1]` - A reference to a CURSED object to store the parsed result
///
/// # Returns
///
/// Null to indicate successful unmarshaling
///
/// # Errors
///
/// Returns a Runtime error if fewer than 2 arguments are provided
pub fn unmarshal(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "unmarshal requires 2 arguments: JSON string and target object".to_string(),
        ));
    }

    // Simplified implementation - just return a placeholder object
    Ok(Rc::new(Object::Null))
}
