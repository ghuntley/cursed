//! The json_tea package provides JSON encoding and decoding.
//! This is equivalent to the encoding/json package in Go.

use std::rc::Rc;
use std::collections::HashMap;
use crate::object::Object;
use crate::error::Error;

/// Marshal serializes a CURSED object into a JSON string
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
        },
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
        },
        _ => return Err(Error::Runtime(format!("Cannot marshal type {} to JSON", args[0].type_name()))),
    };
    
    Ok(Rc::new(Object::String(json)))
}

/// Unmarshal parses JSON into a CURSED object
pub fn unmarshal(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("unmarshal requires 2 arguments: JSON string and target object".to_string()));
    }
    
    // Simplified implementation - just return a placeholder object
    Ok(Rc::new(Object::Null))
}