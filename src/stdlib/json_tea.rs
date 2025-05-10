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
/// # Arguments
///
/// * `args[0]` - The JSON string to parse as a String Object
/// * `args[1]` - A reference to a CURSED object to store the parsed result (currently unused)
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

    // Get the JSON string from the first argument
    let json_str = match &*args[0] {
        Object::String(s) => s,
        _ => return Err(Error::Runtime("First argument to unmarshal must be a string".to_string())),
    };

    // Trim whitespace
    let json_str = json_str.trim();

    // If the string is empty, return null
    if json_str.is_empty() {
        return Ok(Rc::new(Object::Null));
    }

    // Parse the JSON content based on the first character
    let result = match json_str.chars().next().unwrap() {
        // Object
        '{' => parse_json_object(json_str),
        // Array
        '[' => parse_json_array(json_str),
        // String
        '"' => parse_json_string(json_str),
        // Null, Boolean, Number
        _ => {
            if json_str == "null" {
                Ok(Object::Null)
            } else if json_str == "true" {
                Ok(Object::Boolean(true))
            } else if json_str == "false" {
                Ok(Object::Boolean(false))
            } else {
                // Try to parse as number
                if let Ok(int_val) = json_str.parse::<i64>() {
                    Ok(Object::Integer(int_val))
                } else if let Ok(float_val) = json_str.parse::<f64>() {
                    Ok(Object::Float(float_val))
                } else {
                    Err(Error::Runtime(format!("Invalid JSON value: {}", json_str)))
                }
            }
        }
    }?;

    Ok(Rc::new(result))
}

/// Parse a JSON object from a string
fn parse_json_object(json_str: &str) -> Result<Object, Error> {
    // Ensure it starts and ends with braces
    if !json_str.starts_with('{') || !json_str.ends_with('}') {
        return Err(Error::Runtime(format!("Invalid JSON object: {}", json_str)));
    }

    // Extract the content between the braces
    let content = &json_str[1..json_str.len() - 1].trim();
    
    // If empty object, return empty map
    if content.is_empty() {
        return Ok(Object::HashTable(HashMap::new()));
    }

    let mut map = HashMap::new();
    
    // Split by commas, but respect nested structures
    let pairs = split_json_pairs(content)?;
    
    for pair in pairs {
        // Split the pair by colon
        let parts: Vec<&str> = pair.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(Error::Runtime(format!("Invalid JSON object pair: {}", pair)));
        }
        
        // Parse the key (must be a string)
        let key_str = parts[0].trim();
        if !key_str.starts_with('"') || !key_str.ends_with('"') {
            return Err(Error::Runtime(format!("JSON object key must be a string: {}", key_str)));
        }
        
        // Remove the quotes and parse the string
        let key = key_str[1..key_str.len() - 1].to_string();
        
        // Parse the value recursively
        let value_str = parts[1].trim();
        let value = match unmarshal(&[Rc::new(Object::String(value_str.to_string())), Rc::new(Object::Null)]) {
            Ok(parsed) => (*parsed).clone(),
            Err(e) => return Err(e),
        };
        
        map.insert(key, value);
    }
    
    Ok(Object::HashTable(map))
}

/// Parse a JSON array from a string
fn parse_json_array(json_str: &str) -> Result<Object, Error> {
    // Ensure it starts and ends with brackets
    if !json_str.starts_with('[') || !json_str.ends_with(']') {
        return Err(Error::Runtime(format!("Invalid JSON array: {}", json_str)));
    }
    
    // Extract the content between the brackets
    let content = &json_str[1..json_str.len() - 1].trim();
    
    // If empty array, return empty array
    if content.is_empty() {
        return Ok(Object::Array(Vec::new()));
    }
    
    let mut array = Vec::new();
    
    // Split by commas, but respect nested structures
    let elements = split_json_elements(content)?;
    
    for element in elements {
        // Parse each element recursively
        let elem_str = element.trim();
        let parsed_elem = match unmarshal(&[Rc::new(Object::String(elem_str.to_string())), Rc::new(Object::Null)]) {
            Ok(parsed) => (*parsed).clone(),
            Err(e) => return Err(e),
        };
        
        array.push(parsed_elem);
    }
    
    Ok(Object::Array(array))
}

/// Parse a JSON string from a string
fn parse_json_string(json_str: &str) -> Result<Object, Error> {
    // Ensure it starts and ends with quotes
    if !json_str.starts_with('"') || !json_str.ends_with('"') {
        return Err(Error::Runtime(format!("Invalid JSON string: {}", json_str)));
    }
    
    // Extract the content between the quotes
    let content = &json_str[1..json_str.len() - 1];
    
    // Handle escape sequences
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('/') => result.push('/'),
                Some('b') => result.push('\x08'), // Backspace
                Some('f') => result.push('\x0C'), // Form feed
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('u') => {
                    // Unicode escape sequence
                    let mut hex = String::new();
                    for _ in 0..4 {
                        if let Some(digit) = chars.next() {
                            hex.push(digit);
                        } else {
                            return Err(Error::Runtime("Incomplete Unicode escape sequence".to_string()));
                        }
                    }
                    
                    // Parse hex value and convert to char
                    if let Ok(hex_val) = u32::from_str_radix(&hex, 16) {
                        if let Some(unicode_char) = std::char::from_u32(hex_val) {
                            result.push(unicode_char);
                        } else {
                            return Err(Error::Runtime(format!("Invalid Unicode escape sequence: \\u{}", hex)));
                        }
                    } else {
                        return Err(Error::Runtime(format!("Invalid Unicode escape sequence: \\u{}", hex)));
                    }
                },
                Some(c) => return Err(Error::Runtime(format!("Invalid escape sequence: \\{}", c))),
                None => return Err(Error::Runtime("Incomplete escape sequence".to_string())),
            }
        } else {
            result.push(c);
        }
    }
    
    Ok(Object::String(result))
}

/// Split a JSON object's content into key-value pairs, respecting nested structures
fn split_json_pairs(content: &str) -> Result<Vec<String>, Error> {
    let mut pairs = Vec::new();
    let mut current_pair = String::new();
    let mut nesting_level = 0;
    let mut in_string = false;
    let mut escape_next = false;
    
    for c in content.chars() {
        if escape_next {
            current_pair.push(c);
            escape_next = false;
            continue;
        }
        
        match c {
            '\\' if in_string => {
                current_pair.push(c);
                escape_next = true;
            },
            '"' => {
                current_pair.push(c);
                in_string = !in_string;
            },
            '{' | '[' if !in_string => {
                current_pair.push(c);
                nesting_level += 1;
            },
            '}' | ']' if !in_string => {
                current_pair.push(c);
                nesting_level -= 1;
                if nesting_level < 0 {
                    return Err(Error::Runtime("Mismatched braces in JSON".to_string()));
                }
            },
            ',' if !in_string && nesting_level == 0 => {
                // End of a pair
                pairs.push(current_pair.trim().to_string());
                current_pair = String::new();
            },
            _ => {
                current_pair.push(c);
            }
        }
    }
    
    // Don't forget the last pair
    if !current_pair.trim().is_empty() {
        pairs.push(current_pair.trim().to_string());
    }
    
    if in_string {
        return Err(Error::Runtime("Unterminated string in JSON".to_string()));
    }
    
    if nesting_level != 0 {
        return Err(Error::Runtime("Mismatched braces in JSON".to_string()));
    }
    
    Ok(pairs)
}

/// Split a JSON array's content into elements, respecting nested structures
fn split_json_elements(content: &str) -> Result<Vec<String>, Error> {
    let mut elements = Vec::new();
    let mut current_element = String::new();
    let mut nesting_level = 0;
    let mut in_string = false;
    let mut escape_next = false;
    
    for c in content.chars() {
        if escape_next {
            current_element.push(c);
            escape_next = false;
            continue;
        }
        
        match c {
            '\\' if in_string => {
                current_element.push(c);
                escape_next = true;
            },
            '"' => {
                current_element.push(c);
                in_string = !in_string;
            },
            '{' | '[' if !in_string => {
                current_element.push(c);
                nesting_level += 1;
            },
            '}' | ']' if !in_string => {
                current_element.push(c);
                nesting_level -= 1;
                if nesting_level < 0 {
                    return Err(Error::Runtime("Mismatched braces in JSON".to_string()));
                }
            },
            ',' if !in_string && nesting_level == 0 => {
                // End of an element
                elements.push(current_element.trim().to_string());
                current_element = String::new();
            },
            _ => {
                current_element.push(c);
            }
        }
    }
    
    // Don't forget the last element
    if !current_element.trim().is_empty() {
        elements.push(current_element.trim().to_string());
    }
    
    if in_string {
        return Err(Error::Runtime("Unterminated string in JSON".to_string()));
    }
    
    if nesting_level != 0 {
        return Err(Error::Runtime("Mismatched braces in JSON".to_string()));
    }
    
    Ok(elements)
}
