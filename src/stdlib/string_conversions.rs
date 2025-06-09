//! String conversion functions for the CURSED standard library
//!
//! This module provides high-level string conversion functions that integrate
//! with the CURSED object system and provide user-friendly APIs for converting
//! between strings and other types.

use crate::error::Error;
use crate::object::Object;
use crate::runtime::string_conversions::*;
use std::sync::Arc;
use tracing::{instrument, debug, warn};

/// Parse a string as an integer with comprehensive error handling
///
/// # Arguments
/// * `args[0]` - The string to parse
/// * `args[1]` - Optional: The base (2, 8, 10, 16). Defaults to 10.
///
/// # Returns
/// An integer object on success, or an error on failure
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn parse_int(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(Error::new(
            "ArgumentError",
            format!("parse_int takes 1 or 2 arguments, got {}", args.len()),
            None,
        ));
    }

    let string_value = match &*args[0] {
        Object::String(s) => s,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("parse_int requires a string, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Convert to CURSED string format
    let cursed_string = CursedString::from_str(string_value);
    
    // Parse with runtime function
    let result = cursed_string_to_int(cursed_string);
    
    if result.success {
        debug!(value = result.value, input = string_value, "Successfully parsed integer");
        Ok(Arc::new(Object::Integer(result.value)))
    } else {
        warn!(input = string_value, "Failed to parse integer");
        Err(Error::new(
            "ValueError",
            format!("Could not parse '{}' as an integer", string_value),
            None,
        ))
    }
}

/// Parse a string as a float with comprehensive error handling
///
/// # Arguments
/// * `args[0]` - The string to parse
///
/// # Returns
/// A float object on success, or an error on failure
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn parse_float(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("parse_float takes exactly 1 argument, got {}", args.len()),
            None,
        ));
    }

    let string_value = match &*args[0] {
        Object::String(s) => s,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("parse_float requires a string, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Convert to CURSED string format
    let cursed_string = CursedString::from_str(string_value);
    
    // Parse with runtime function
    let result = cursed_string_to_float(cursed_string);
    
    if result.success {
        debug!(value = result.value, input = string_value, "Successfully parsed float");
        Ok(Arc::new(Object::Float(result.value)))
    } else {
        warn!(input = string_value, "Failed to parse float");
        Err(Error::new(
            "ValueError",
            format!("Could not parse '{}' as a float", string_value),
            None,
        ))
    }
}

/// Parse a string as a boolean with multiple accepted formats
///
/// # Arguments
/// * `args[0]` - The string to parse
///
/// # Returns
/// A boolean object on success, or an error on failure
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn parse_bool(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("parse_bool takes exactly 1 argument, got {}", args.len()),
            None,
        ));
    }

    let string_value = match &*args[0] {
        Object::String(s) => s,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("parse_bool requires a string, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Convert to CURSED string format
    let cursed_string = CursedString::from_str(string_value);
    
    // Parse with runtime function
    let result = cursed_string_to_bool(cursed_string);
    
    if result.success {
        debug!(value = result.value, input = string_value, "Successfully parsed boolean");
        Ok(Arc::new(Object::Boolean(result.value)))
    } else {
        warn!(input = string_value, "Failed to parse boolean");
        Err(Error::new(
            "ValueError",
            format!("Could not parse '{}' as a boolean. Valid formats: true/false, yes/no, 1/0, on/off", string_value),
            None,
        ))
    }
}

/// Convert an integer to a string representation
///
/// # Arguments
/// * `args[0]` - The integer to convert
/// * `args[1]` - Optional: The base (2, 8, 10, 16). Defaults to 10.
///
/// # Returns
/// A string object containing the string representation
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn int_to_string(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(Error::new(
            "ArgumentError",
            format!("int_to_string takes 1 or 2 arguments, got {}", args.len()),
            None,
        ));
    }

    let int_value = match &*args[0] {
        Object::Integer(i) => *i,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("int_to_string requires an integer, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Get base if provided
    let base = if args.len() > 1 {
        match &*args[1] {
            Object::Integer(b) => *b as i32,
            _ => {
                return Err(Error::new(
                    "TypeError",
                    "Base must be an integer".to_string(),
                    None,
                ));
            }
        }
    } else {
        10 // Default to decimal
    };

    // Convert using runtime function
    let cursed_string = if base == 10 {
        cursed_int_to_string(int_value)
    } else {
        cursed_int_to_string_base(int_value, base)
    };
    
    let string_repr = cursed_string.as_str()
        .map_err(|e| Error::new("InternalError", format!("UTF-8 conversion error: {}", e), None))?;
    
    debug!(value = int_value, base = base, result = string_repr, "Converted integer to string");
    Ok(Arc::new(Object::String(string_repr.to_string())))
}

/// Convert a float to a string representation
///
/// # Arguments
/// * `args[0]` - The float to convert
/// * `args[1]` - Optional: The precision (number of decimal places). Defaults to automatic.
///
/// # Returns
/// A string object containing the string representation
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn float_to_string(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(Error::new(
            "ArgumentError",
            format!("float_to_string takes 1 or 2 arguments, got {}", args.len()),
            None,
        ));
    }

    let float_value = match &*args[0] {
        Object::Float(f) => *f,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("float_to_string requires a float, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Convert using runtime function
    let cursed_string = if args.len() > 1 {
        // Get precision if provided
        let precision = match &*args[1] {
            Object::Integer(p) => *p as i32,
            _ => {
                return Err(Error::new(
                    "TypeError",
                    "Precision must be an integer".to_string(),
                    None,
                ));
            }
        };
        cursed_float_to_string_precision(float_value, precision)
    } else {
        cursed_float_to_string(float_value)
    };
    
    let string_repr = cursed_string.as_str()
        .map_err(|e| Error::new("InternalError", format!("UTF-8 conversion error: {}", e), None))?;
    
    debug!(value = float_value, result = string_repr, "Converted float to string");
    Ok(Arc::new(Object::String(string_repr.to_string())))
}

/// Convert a boolean to a string representation
///
/// # Arguments
/// * `args[0]` - The boolean to convert
///
/// # Returns
/// A string object containing "true" or "false"
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn bool_to_string(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("bool_to_string takes exactly 1 argument, got {}", args.len()),
            None,
        ));
    }

    let bool_value = match &*args[0] {
        Object::Boolean(b) => *b,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("bool_to_string requires a boolean, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Convert using runtime function
    let cursed_string = cursed_bool_to_string(bool_value);
    
    let string_repr = cursed_string.as_str()
        .map_err(|e| Error::new("InternalError", format!("UTF-8 conversion error: {}", e), None))?;
    
    debug!(value = bool_value, result = string_repr, "Converted boolean to string");
    Ok(Arc::new(Object::String(string_repr.to_string())))
}

/// Check if a string contains valid UTF-8
///
/// # Arguments
/// * `args[0]` - The string to validate
///
/// # Returns
/// A boolean object indicating whether the string is valid UTF-8
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn is_valid_utf8(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("is_valid_utf8 takes exactly 1 argument, got {}", args.len()),
            None,
        ));
    }

    let string_value = match &*args[0] {
        Object::String(s) => s,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("is_valid_utf8 requires a string, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Convert to CURSED string format
    let cursed_string = CursedString::from_str(string_value);
    
    // Validate with runtime function
    let is_valid = cursed_string_is_valid_utf8(cursed_string);
    
    debug!(input = string_value, is_valid = is_valid, "Validated UTF-8");
    Ok(Arc::new(Object::Boolean(is_valid)))
}

/// Get the number of Unicode characters in a string
///
/// # Arguments
/// * `args[0]` - The string to measure
///
/// # Returns
/// An integer object containing the character count
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn utf8_char_count(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("utf8_char_count takes exactly 1 argument, got {}", args.len()),
            None,
        ));
    }

    let string_value = match &*args[0] {
        Object::String(s) => s,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("utf8_char_count requires a string, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Convert to CURSED string format
    let cursed_string = CursedString::from_str(string_value);
    
    // Count characters with runtime function
    let char_count = cursed_string_utf8_length(cursed_string);
    
    if char_count < 0 {
        return Err(Error::new(
            "ValueError",
            "Invalid UTF-8 string provided".to_string(),
            None,
        ));
    }
    
    debug!(input = string_value, char_count = char_count, "Counted UTF-8 characters");
    Ok(Arc::new(Object::Integer(char_count)))
}

/// Try to parse a string as any numeric type, returning the most appropriate type
///
/// # Arguments
/// * `args[0]` - The string to parse
///
/// # Returns
/// An integer, float, or error depending on the string content
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn parse_number(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("parse_number takes exactly 1 argument, got {}", args.len()),
            None,
        ));
    }

    let string_value = match &*args[0] {
        Object::String(s) => s,
        _ => {
            return Err(Error::new(
                "TypeError",
                format!("parse_number requires a string, got {}", args[0].type_name()),
                None,
            ));
        }
    };

    // Try integer first
    let cursed_string = CursedString::from_str(string_value);
    let int_result = cursed_string_to_int(cursed_string.clone());
    
    if int_result.success {
        debug!(value = int_result.value, input = string_value, "Parsed as integer");
        return Ok(Arc::new(Object::Integer(int_result.value)));
    }
    
    // Try float if integer parsing failed
    let float_result = cursed_string_to_float(cursed_string);
    
    if float_result.success {
        debug!(value = float_result.value, input = string_value, "Parsed as float");
        return Ok(Arc::new(Object::Float(float_result.value)));
    }
    
    // Neither worked
    warn!(input = string_value, "Failed to parse as number");
    Err(Error::new(
        "ValueError",
        format!("Could not parse '{}' as a number", string_value),
        None,
    ))
}

/// Convert any object to its string representation
///
/// # Arguments
/// * `args[0]` - The object to convert
///
/// # Returns
/// A string object containing the string representation
#[instrument(skip(args), fields(args_count = args.len()), level = "debug")]
pub fn to_string(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::new(
            "ArgumentError",
            format!("to_string takes exactly 1 argument, got {}", args.len()),
            None,
        ));
    }

    let result = match &*args[0] {
        Object::Integer(i) => {
            let cursed_string = cursed_int_to_string(*i);
            cursed_string.as_str()
                .map_err(|e| Error::new("InternalError", format!("UTF-8 conversion error: {}", e), None))?
                .to_string()
        },
        Object::Float(f) => {
            let cursed_string = cursed_float_to_string(*f);
            cursed_string.as_str()
                .map_err(|e| Error::new("InternalError", format!("UTF-8 conversion error: {}", e), None))?
                .to_string()
        },
        Object::Boolean(b) => {
            let cursed_string = cursed_bool_to_string(*b);
            cursed_string.as_str()
                .map_err(|e| Error::new("InternalError", format!("UTF-8 conversion error: {}", e), None))?
                .to_string()
        },
        Object::String(s) => s.clone(),
        Object::Null => "null".to_string(),
        Object::Array(arr) => {
            // Convert array to string representation
            let elements: Result<Vec<String>, Error> = arr.iter()
                .map(|elem| to_string(&[std::sync::Arc::new(elem.clone())]).map(|obj| {
                    if let Object::String(s) = &*obj {
                        s.clone()
                    } else {
                        "".to_string()
                    }
                }))
                .collect();
            format!("[{}]", elements?.join(", "))
        },
        other => {
            // For other types, use the default string representation
            other.to_string()
        }
    };
    
    debug!(result = result, "Converted object to string");
    Ok(Arc::new(Object::String(result)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_int_function() {
        // Test basic integer parsing
        let args = vec![Arc::new(Object::String("123".to_string()))];
        let result = parse_int(&args).unwrap();
        assert_eq!(*result, Object::Integer(123));
        
        // Test negative integer
        let args = vec![Arc::new(Object::String("-456".to_string()))];
        let result = parse_int(&args).unwrap();
        assert_eq!(*result, Object::Integer(-456));
        
        // Test hex
        let args = vec![Arc::new(Object::String("0xFF".to_string()))];
        let result = parse_int(&args).unwrap();
        assert_eq!(*result, Object::Integer(255));
        
        // Test error case
        let args = vec![Arc::new(Object::String("not_a_number".to_string()))];
        assert!(parse_int(&args).is_err());
    }
    
    #[test]
    fn test_parse_float_function() {
        // Test basic float parsing
        let args = vec![Arc::new(Object::String("123.456".to_string()))];
        let result = parse_float(&args).unwrap();
        if let Object::Float(f) = &*result {
            assert!((f - 123.456).abs() < f64::EPSILON);
        } else {
            panic!("Expected float result");
        }
        
        // Test scientific notation
        let args = vec![Arc::new(Object::String("1.23e10".to_string()))];
        let result = parse_float(&args).unwrap();
        if let Object::Float(f) = &*result {
            assert_eq!(*f, 1.23e10);
        } else {
            panic!("Expected float result");
        }
        
        // Test error case
        let args = vec![Arc::new(Object::String("not_a_float".to_string()))];
        assert!(parse_float(&args).is_err());
    }
    
    #[test]
    fn test_parse_bool_function() {
        // Test true values
        for true_str in &["true", "True", "yes", "1", "on"] {
            let args = vec![Arc::new(Object::String(true_str.to_string()))];
            let result = parse_bool(&args).unwrap();
            assert_eq!(*result, Object::Boolean(true));
        }
        
        // Test false values
        for false_str in &["false", "False", "no", "0", "off"] {
            let args = vec![Arc::new(Object::String(false_str.to_string()))];
            let result = parse_bool(&args).unwrap();
            assert_eq!(*result, Object::Boolean(false));
        }
        
        // Test error case
        let args = vec![Arc::new(Object::String("maybe".to_string()))];
        assert!(parse_bool(&args).is_err());
    }
    
    #[test]
    fn test_number_to_string_functions() {
        // Test int_to_string
        let args = vec![Arc::new(Object::Integer(123))];
        let result = int_to_string(&args).unwrap();
        assert_eq!(*result, Object::String("123".to_string()));
        
        // Test float_to_string
        let args = vec![Arc::new(Object::Float(123.456))];
        let result = float_to_string(&args).unwrap();
        if let Object::String(s) = &*result {
            assert!(s.contains("123"));
        } else {
            panic!("Expected string result");
        }
        
        // Test bool_to_string
        let args = vec![Arc::new(Object::Boolean(true))];
        let result = bool_to_string(&args).unwrap();
        assert_eq!(*result, Object::String("true".to_string()));
    }
    
    #[test]
    fn test_utf8_functions() {
        // Test UTF-8 validation
        let args = vec![Arc::new(Object::String("Hello, 世界!".to_string()))];
        let result = is_valid_utf8(&args).unwrap();
        assert_eq!(*result, Object::Boolean(true));
        
        // Test character counting
        let result = utf8_char_count(&args).unwrap();
        assert_eq!(*result, Object::Integer(9)); // "Hello, 世界!" has 9 characters
    }
    
    #[test]
    fn test_parse_number_function() {
        // Test integer parsing
        let args = vec![Arc::new(Object::String("123".to_string()))];
        let result = parse_number(&args).unwrap();
        assert_eq!(*result, Object::Integer(123));
        
        // Test float parsing
        let args = vec![Arc::new(Object::String("123.456".to_string()))];
        let result = parse_number(&args).unwrap();
        if let Object::Float(f) = &*result {
            assert!((f - 123.456).abs() < f64::EPSILON);
        } else {
            panic!("Expected float result");
        }
        
        // Test error case
        let args = vec![Arc::new(Object::String("not_a_number".to_string()))];
        assert!(parse_number(&args).is_err());
    }
    
    #[test]
    fn test_to_string_function() {
        // Test integer conversion
        let args = vec![Arc::new(Object::Integer(123))];
        let result = to_string(&args).unwrap();
        assert_eq!(*result, Object::String("123".to_string()));
        
        // Test boolean conversion
        let args = vec![Arc::new(Object::Boolean(true))];
        let result = to_string(&args).unwrap();
        assert_eq!(*result, Object::String("true".to_string()));
        
        // Test string passthrough
        let args = vec![Arc::new(Object::String("hello".to_string()))];
        let result = to_string(&args).unwrap();
        assert_eq!(*result, Object::String("hello".to_string()));
        
        // Test array conversion
        let args = vec![Arc::new(Object::Array(vec![
            Arc::new(Object::Integer(1)),
            Arc::new(Object::Integer(2)),
            Arc::new(Object::Integer(3)),
        ]))];
        let result = to_string(&args).unwrap();
        if let Object::String(s) = &*result {
            assert!(s.contains("1"));
            assert!(s.contains("2"));
            assert!(s.contains("3"));
        } else {
            panic!("Expected string result");
        }
    }
}
