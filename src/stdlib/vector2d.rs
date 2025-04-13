//! Vector2D Type Implementation
//! 
//! This module provides methods for the Vector2D user-defined type.

use crate::error::Error;
use serde_json::{Value, json};
use super::dot_registry::{DotRegistry, DOT_REGISTRY};

/// Calculates the length of a Vector2D
pub fn vector2d_length(object_json: String, _args: Vec<String>) -> Result<String, Error> {
    // Parse the object JSON
    if let Ok(value) = serde_json::from_str::<Value>(&object_json) {
        if let Some(x) = value.get("x").and_then(|v| v.as_f64()) {
            if let Some(y) = value.get("y").and_then(|v| v.as_f64()) {
                // Calculate length using Pythagorean theorem
                let length = (x * x + y * y).sqrt();
                return Ok(length.to_string());
            }
        }
    }
    
    Err(Error::from_str("Invalid Vector2D object or missing x/y fields"))
}

/// Adds values to Vector2D coordinates
pub fn vector2d_add(object_json: String, args: Vec<String>) -> Result<String, Error> {
    // Check if we have enough arguments
    if args.len() < 2 {
        return Err(Error::from_str("Vector2D.add requires two arguments: dx and dy"));
    }
    
    // Parse the object JSON
    if let Ok(mut value) = serde_json::from_str::<Value>(&object_json) {
        // Parse the arguments
        let dx = args[0].parse::<f64>().unwrap_or(0.0);
        let dy = args[1].parse::<f64>().unwrap_or(0.0);
        
        // Update the x and y values
        if let Some(x) = value.get("x").and_then(|v| v.as_f64()) {
            if let Some(y) = value.get("y").and_then(|v| v.as_f64()) {
                // Update the values
                if let Some(obj) = value.as_object_mut() {
                    obj.insert("x".to_string(), json!(x + dx));
                    obj.insert("y".to_string(), json!(y + dy));
                }
                
                return Ok(value.to_string());
            }
        }
    }
    
    Err(Error::from_str("Invalid Vector2D object or missing x/y fields"))
}

/// Returns a string representation of the Vector2D
pub fn vector2d_to_string(object_json: String, _args: Vec<String>) -> Result<String, Error> {
    // Parse the object JSON
    if let Ok(value) = serde_json::from_str::<Value>(&object_json) {
        if let Some(x) = value.get("x") {
            if let Some(y) = value.get("y") {
                return Ok(format!("Vector2D({}, {})", x, y));
            }
        }
    }
    
    Err(Error::from_str("Invalid Vector2D object or missing x/y fields"))
}

/// Generic handler for Vector2D.length
pub fn vector2d_length_generic(object: Value, _args: Vec<Value>) -> Result<Value, Error> {
    if let Some(x) = object.get("x").and_then(|v| v.as_f64()) {
        if let Some(y) = object.get("y").and_then(|v| v.as_f64()) {
            // Calculate length using Pythagorean theorem
            let length = (x * x + y * y).sqrt();
            return Ok(json!(length));
        }
    }
    
    Err(Error::from_str("Invalid Vector2D object or missing x/y fields"))
}

/// Generic handler for Vector2D.add
pub fn vector2d_add_generic(object: Value, args: Vec<Value>) -> Result<Value, Error> {
    // Check if we have enough arguments
    if args.len() < 2 {
        return Err(Error::from_str("Vector2D.add requires two arguments: dx and dy"));
    }
    
    // Parse the arguments
    let dx = args[0].as_f64().unwrap_or(0.0);
    let dy = args[1].as_f64().unwrap_or(0.0);
    
    // Clone the object to avoid borrowing issues
    let mut result = object.clone();
    
    // Update the x and y values
    if let Some(x) = object.get("x").and_then(|v| v.as_f64()) {
        if let Some(y) = object.get("y").and_then(|v| v.as_f64()) {
            // Update the values
            if let Some(obj) = result.as_object_mut() {
                obj.insert("x".to_string(), json!(x + dx));
                obj.insert("y".to_string(), json!(y + dy));
            }
            
            return Ok(result);
        }
    }
    
    Err(Error::from_str("Invalid Vector2D object or missing x/y fields"))
}

/// Generic handler for Vector2D.toString
pub fn vector2d_to_string_generic(object: Value, _args: Vec<Value>) -> Result<Value, Error> {
    if let Some(x) = object.get("x") {
        if let Some(y) = object.get("y") {
            return Ok(json!(format!("Vector2D({}, {})", x, y)));
        }
    }
    
    Err(Error::from_str("Invalid Vector2D object or missing x/y fields"))
}

/// Register Vector2D methods with the dot registry
pub fn register_vector2d_methods() {
    // Get the registry
    if let Ok(mut registry) = DOT_REGISTRY.lock() {
        // Register string-based methods
        registry.register_method("Vector2D", "length", vector2d_length);
        registry.register_method("Vector2D", "add", vector2d_add);
        registry.register_method("Vector2D", "toString", vector2d_to_string);
        
        // Register generic methods
        registry.register_generic_method("Vector2D", "length", vector2d_length_generic);
        registry.register_generic_method("Vector2D", "add", vector2d_add_generic);
        registry.register_generic_method("Vector2D", "toString", vector2d_to_string_generic);
    }
}