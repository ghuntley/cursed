//! Character classification and conversion functions for CURSED programming language
//! This module provides comprehensive Unicode-compliant character operations.

use crate::error::Error;
use crate::object::Object;
use crate::core::char::CharMethods;
use std::sync::Arc;
use tracing::instrument;

/// Check if a character is uppercase
#[instrument]
pub fn is_uppercase(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            // Get the first (and only) character
            let c = s.chars().next().unwrap();
            Ok(Arc::new(Object::Boolean(c.is_uppercase())))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            Ok(Arc::new(Object::Boolean(c.is_uppercase())))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is lowercase
pub fn is_lowercase(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            // Get the first (and only) character
            let c = s.chars().next().unwrap();
            Ok(Arc::new(Object::Boolean(c.is_lowercase())))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            Ok(Arc::new(Object::Boolean(c.is_lowercase())))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is a digit
pub fn is_digit(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            // Get the first (and only) character
            let c = s.chars().next().unwrap();
            Ok(Arc::new(Object::Boolean(c.is_digit(10))))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            Ok(Arc::new(Object::Boolean(c.is_digit(10))))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is alphabetic
pub fn is_alpha(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            // Get the first (and only) character
            let c = s.chars().next().unwrap();
            Ok(Arc::new(Object::Boolean(c.is_alphabetic())))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            Ok(Arc::new(Object::Boolean(c.is_alphabetic())))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Convert a character to uppercase
pub fn to_uppercase(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            // Get the first (and only) character
            let c = s.chars().next().unwrap();
            let upper = c.to_uppercase().collect::<String>();
            Ok(Arc::new(Object::String(upper)))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            let upper = c.to_uppercase().collect::<String>();
            Ok(Arc::new(Object::String(upper)))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Convert a character to lowercase
#[instrument]
pub fn to_lowercase(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            // Get the first (and only) character
            let c = s.chars().next().unwrap();
            let lower = c.to_lowercase().collect::<String>();
            Ok(Arc::new(Object::String(lower)))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            let lower = c.to_lowercase().collect::<String>();
            Ok(Arc::new(Object::String(lower)))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is alphabetic
#[instrument]
pub fn is_alphabetic(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Ok(Arc::new(Object::Boolean(CharMethods::is_alphabetic(c))))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            Ok(Arc::new(Object::Boolean(CharMethods::is_alphabetic(*c))))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is numeric (Unicode numeric character)
#[instrument]
pub fn is_numeric(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Ok(Arc::new(Object::Boolean(CharMethods::is_numeric(c))))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            Ok(Arc::new(Object::Boolean(CharMethods::is_numeric(*c))))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is whitespace
#[instrument]
pub fn is_whitespace(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Ok(Arc::new(Object::Boolean(CharMethods::is_whitespace(c))))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            Ok(Arc::new(Object::Boolean(CharMethods::is_whitespace(*c))))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Convert a character to string representation
#[instrument]
pub fn char_to_string(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("Expected 1 argument, got {}", args.len()),
            None,
        ));
    }
    
    match &*args[0] {
        Object::String(s) if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Ok(Arc::new(Object::String(CharMethods::to_string(c))))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            Ok(Arc::new(Object::String(CharMethods::to_string(*c))))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}