//! Character classification functions for CURSED programming language
//! This module provides functions for classifying characters.

use crate::error::Error;
use crate::object::Object;
use std::rc::Rc;

/// Check if a character is uppercase
pub fn is_uppercase(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
            Ok(Rc::new(Object::Boolean(c.is_uppercase())))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            Ok(Rc::new(Object::Boolean(c.is_uppercase())))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is lowercase
pub fn is_lowercase(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
            Ok(Rc::new(Object::Boolean(c.is_lowercase())))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            Ok(Rc::new(Object::Boolean(c.is_lowercase())))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is a digit
pub fn is_digit(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
            Ok(Rc::new(Object::Boolean(c.is_digit(10))))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            Ok(Rc::new(Object::Boolean(c.is_digit(10))))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Check if a character is alphabetic
pub fn is_alpha(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
            Ok(Rc::new(Object::Boolean(c.is_alphabetic())))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            Ok(Rc::new(Object::Boolean(c.is_alphabetic())))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Convert a character to uppercase
pub fn to_uppercase(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
            Ok(Rc::new(Object::String(upper)))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            let upper = c.to_uppercase().collect::<String>();
            Ok(Rc::new(Object::String(upper)))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}

/// Convert a character to lowercase
pub fn to_lowercase(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
            Ok(Rc::new(Object::String(lower)))
        },
        Object::String(_) => Err(Error::new(
            "TypeError",
            "Expected a single character string".to_string(),
            None,
        )),
        Object::Char(c) => {
            // Handle character type if it exists
            let lower = c.to_lowercase().collect::<String>();
            Ok(Rc::new(Object::String(lower)))
        },
        _ => Err(Error::new(
            "TypeError",
            format!("Expected a character, got {}", args[0].type_name()),
            None,
        )),
    }
}