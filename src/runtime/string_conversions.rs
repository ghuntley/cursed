//! Runtime implementation of string conversion functions
//!
//! This module provides the actual runtime implementations of string conversion
//! functions that are called from LLVM-generated code. These functions handle:
//! - String parsing with proper error handling
//! - Number to string formatting
//! - UTF-8 validation and character counting
//! - Memory management integration with the garbage collector

use std::ffi::{CStr, CString};
use std::ptr;
use std::slice;
use std::str;
use tracing::{instrument, debug, warn, error};

/// String conversion result for integer parsing
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StringToIntResult {
    pub value: i64,
    pub success: bool,
}

/// String conversion result for float parsing
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StringToFloatResult {
    pub value: f64,
    pub success: bool,
}

/// String conversion result for boolean parsing
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StringToBoolResult {
    pub value: bool,
    pub success: bool,
}

/// CURSED string struct layout (must match LLVM definition)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CursedString {
    pub length: i64,
    pub data: *const u8,
}

impl CursedString {
    /// Create a new CURSED string from Rust string data
    #[instrument(skip(data), level = "debug")]
    pub fn from_str(data: &str) -> Self {
        debug!(length = data.len(), "Creating CURSED string from str");
        Self {
            length: data.len() as i64,
            data: data.as_ptr(),
        }
    }
    
    /// Create a new CURSED string from owned data (for GC integration)
    #[instrument(skip(data), level = "debug")]
    pub fn from_owned(data: Vec<u8>) -> Self {
        let length = data.len() as i64;
        let ptr = data.as_ptr();
        
        // In a real implementation, we'd register this with the GC
        // For now, we'll leak the memory to avoid double-free issues
        std::mem::forget(data);
        
        debug!(length = length, "Created CURSED string from owned data");
        Self {
            length,
            data: ptr,
        }
    }
    
    /// Convert CURSED string to Rust str (with validation)
    #[instrument(skip(self), level = "debug")]
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        if self.data.is_null() || self.length < 0 {
            debug!("Invalid CURSED string: null pointer or negative length");
            return Ok("");
        }
        
        let slice = unsafe {
            slice::from_raw_parts(self.data, self.length as usize)
        };
        
        str::from_utf8(slice)
    }
    
    /// Get the raw bytes of the string
    #[instrument(skip(self), level = "trace")]
    pub fn as_bytes(&self) -> &[u8] {
        if self.data.is_null() || self.length < 0 {
            return &[];
        }
        
        unsafe {
            slice::from_raw_parts(self.data, self.length as usize)
        }
    }
}

/// String to integer conversion with comprehensive error handling
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_string_to_int(string: CursedString) -> StringToIntResult {
    debug!("Converting string to integer");
    
    let string_slice = match string.as_str() {
        Ok(s) => s,
        Err(e) => {
            warn!(error = ?e, "Invalid UTF-8 in string to int conversion");
            return StringToIntResult { value: 0, success: false };
        }
    };
    
    // Trim whitespace
    let trimmed = string_slice.trim();
    
    if trimmed.is_empty() {
        debug!("Empty string provided for integer conversion");
        return StringToIntResult { value: 0, success: false };
    }
    
    // Handle different integer formats
    match parse_integer(trimmed) {
        Ok(value) => {
            debug!(value = value, "Successfully parsed integer");
            StringToIntResult { value, success: true }
        }
        Err(e) => {
            warn!(input = trimmed, error = ?e, "Failed to parse integer");
            StringToIntResult { value: 0, success: false }
        }
    }
}

/// Parse integer with support for different bases and formats
#[instrument(level = "debug")]
fn parse_integer(s: &str) -> Result<i64, std::num::ParseIntError> {
    // Handle binary (0b prefix)
    if s.starts_with("0b") || s.starts_with("0B") {
        return i64::from_str_radix(&s[2..], 2);
    }
    
    // Handle octal (0o prefix)
    if s.starts_with("0o") || s.starts_with("0O") {
        return i64::from_str_radix(&s[2..], 8);
    }
    
    // Handle hexadecimal (0x prefix)
    if s.starts_with("0x") || s.starts_with("0X") {
        return i64::from_str_radix(&s[2..], 16);
    }
    
    // Default decimal parsing
    s.parse::<i64>()
}

/// String to float conversion with comprehensive error handling
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_string_to_float(string: CursedString) -> StringToFloatResult {
    debug!("Converting string to float");
    
    let string_slice = match string.as_str() {
        Ok(s) => s,
        Err(e) => {
            warn!(error = ?e, "Invalid UTF-8 in string to float conversion");
            return StringToFloatResult { value: 0.0, success: false };
        }
    };
    
    // Trim whitespace
    let trimmed = string_slice.trim();
    
    if trimmed.is_empty() {
        debug!("Empty string provided for float conversion");
        return StringToFloatResult { value: 0.0, success: false };
    }
    
    // Handle special float values
    match trimmed.to_lowercase().as_str() {
        "inf" | "infinity" | "+inf" | "+infinity" => {
            debug!("Parsed positive infinity");
            return StringToFloatResult { value: f64::INFINITY, success: true };
        }
        "-inf" | "-infinity" => {
            debug!("Parsed negative infinity");
            return StringToFloatResult { value: f64::NEG_INFINITY, success: true };
        }
        "nan" | "+nan" | "-nan" => {
            debug!("Parsed NaN");
            return StringToFloatResult { value: f64::NAN, success: true };
        }
        _ => {}
    }
    
    // Parse regular float
    match trimmed.parse::<f64>() {
        Ok(value) => {
            debug!(value = value, "Successfully parsed float");
            StringToFloatResult { value, success: true }
        }
        Err(e) => {
            warn!(input = trimmed, error = ?e, "Failed to parse float");
            StringToFloatResult { value: 0.0, success: false }
        }
    }
}

/// Integer to string conversion
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_int_to_string(value: i64) -> CursedString {
    debug!(value = value, "Converting integer to string");
    
    let string_repr = value.to_string();
    let bytes = string_repr.into_bytes();
    
    debug!(length = bytes.len(), "Created string representation");
    CursedString::from_owned(bytes)
}

/// Float to string conversion with configurable precision
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_float_to_string(value: f64) -> CursedString {
    debug!(value = value, "Converting float to string");
    
    let string_repr = if value.is_nan() {
        "NaN".to_string()
    } else if value.is_infinite() {
        if value.is_sign_positive() {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        }
    } else {
        // Use a reasonable default precision
        format!("{}", value)
    };
    
    debug!(representation = ?string_repr, "Creating float string representation");
    let bytes = string_repr.into_bytes();
    CursedString::from_owned(bytes)
}

/// Boolean to string conversion
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_bool_to_string(value: bool) -> CursedString {
    debug!(value = value, "Converting boolean to string");
    
    let string_repr = if value { "true" } else { "false" };
    let bytes = string_repr.as_bytes().to_vec();
    
    debug!(representation = string_repr, "Created boolean string representation");
    CursedString::from_owned(bytes)
}

/// String to boolean conversion with multiple accepted formats
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_string_to_bool(string: CursedString) -> StringToBoolResult {
    debug!("Converting string to boolean");
    
    let string_slice = match string.as_str() {
        Ok(s) => s,
        Err(e) => {
            warn!(error = ?e, "Invalid UTF-8 in string to bool conversion");
            return StringToBoolResult { value: false, success: false };
        }
    };
    
    // Trim and convert to lowercase for case-insensitive comparison
    let trimmed = string_slice.trim().to_lowercase();
    
    if trimmed.is_empty() {
        debug!("Empty string provided for boolean conversion");
        return StringToBoolResult { value: false, success: false };
    }
    
    // Check for various true/false representations
    let (value, success) = match trimmed.as_str() {
        "true" | "t" | "yes" | "y" | "1" | "on" => (true, true),
        "false" | "f" | "no" | "n" | "0" | "off" => (false, true),
        _ => {
            warn!(input = trimmed, "Unrecognized boolean string format");
            (false, false)
        }
    };
    
    debug!(value = value, success = success, input = trimmed, "Boolean conversion result");
    StringToBoolResult { value, success }
}

/// Check if a string contains valid UTF-8
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_string_is_valid_utf8(string: CursedString) -> bool {
    debug!("Checking UTF-8 validity");
    
    if string.data.is_null() || string.length < 0 {
        debug!("Invalid string pointer or length");
        return false;
    }
    
    let bytes = string.as_bytes();
    let is_valid = str::from_utf8(bytes).is_ok();
    
    debug!(is_valid = is_valid, length = bytes.len(), "UTF-8 validation result");
    is_valid
}

/// Get the number of Unicode characters in a UTF-8 string
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_string_utf8_length(string: CursedString) -> i64 {
    debug!("Calculating UTF-8 character length");
    
    let string_slice = match string.as_str() {
        Ok(s) => s,
        Err(e) => {
            warn!(error = ?e, "Invalid UTF-8 in length calculation");
            return -1; // Error indicator
        }
    };
    
    let char_count = string_slice.chars().count() as i64;
    debug!(char_count = char_count, byte_length = string.length, "UTF-8 length calculated");
    char_count
}

/// Memory management helpers for string conversion

/// Allocate a new string with GC integration
#[no_mangle]
#[instrument(skip(data), level = "debug")]
pub extern "C" fn cursed_string_alloc(length: i64, data: *const u8) -> CursedString {
    debug!(length = length, "Allocating new CURSED string");
    
    if length < 0 || data.is_null() {
        warn!("Invalid parameters for string allocation");
        return CursedString { length: 0, data: ptr::null() };
    }
    
    // Copy the data to GC-managed memory
    let source_slice = unsafe { slice::from_raw_parts(data, length as usize) };
    let owned_data = source_slice.to_vec();
    
    debug!(copied_length = owned_data.len(), "Copied data for GC management");
    CursedString::from_owned(owned_data)
}

/// Free a string (currently no-op due to GC)
#[no_mangle]
#[instrument(skip(string), level = "debug")]
pub extern "C" fn cursed_string_free(string: CursedString) {
    debug!(length = string.length, "Freeing CURSED string (GC managed)");
    // In a GC system, this would be a no-op or just mark for collection
    // The actual memory will be freed by the garbage collector
}

/// String conversion utilities for complex formatting

/// Format integer with specific base
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_int_to_string_base(value: i64, base: i32) -> CursedString {
    debug!(value = value, base = base, "Converting integer to string with base");
    
    let string_repr = match base {
        2 => format!("0b{:b}", value),
        8 => format!("0o{:o}", value),
        16 => format!("0x{:x}", value),
        10 | _ => value.to_string(), // Default to decimal
    };
    
    debug!(representation = ?string_repr, "Creating base-specific string representation");
    let bytes = string_repr.into_bytes();
    CursedString::from_owned(bytes)
}

/// Format float with specific precision
#[no_mangle]
#[instrument(level = "debug")]
pub extern "C" fn cursed_float_to_string_precision(value: f64, precision: i32) -> CursedString {
    debug!(value = value, precision = precision, "Converting float to string with precision");
    
    let precision = precision.max(0).min(20) as usize; // Clamp precision to reasonable range
    
    let string_repr = if value.is_nan() {
        "NaN".to_string()
    } else if value.is_infinite() {
        if value.is_sign_positive() {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        }
    } else {
        format!("{:.prec$}", value, prec = precision)
    };
    
    debug!(representation = ?string_repr, "Creating precision-formatted string representation");
    let bytes = string_repr.into_bytes();
    CursedString::from_owned(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_to_int_conversion() {
        // Test basic integer parsing
        let string = CursedString::from_str("123");
        let result = cursed_string_to_int(string);
        assert!(result.success);
        assert_eq!(result.value, 123);
        
        // Test negative integer
        let string = CursedString::from_str("-456");
        let result = cursed_string_to_int(string);
        assert!(result.success);
        assert_eq!(result.value, -456);
        
        // Test hexadecimal
        let string = CursedString::from_str("0xFF");
        let result = cursed_string_to_int(string);
        assert!(result.success);
        assert_eq!(result.value, 255);
        
        // Test binary
        let string = CursedString::from_str("0b1010");
        let result = cursed_string_to_int(string);
        assert!(result.success);
        assert_eq!(result.value, 10);
        
        // Test invalid input
        let string = CursedString::from_str("not_a_number");
        let result = cursed_string_to_int(string);
        assert!(!result.success);
    }
    
    #[test]
    fn test_string_to_float_conversion() {
        // Test basic float parsing
        let string = CursedString::from_str("123.456");
        let result = cursed_string_to_float(string);
        assert!(result.success);
        assert!((result.value - 123.456).abs() < f64::EPSILON);
        
        // Test scientific notation
        let string = CursedString::from_str("1.23e10");
        let result = cursed_string_to_float(string);
        assert!(result.success);
        assert_eq!(result.value, 1.23e10);
        
        // Test infinity
        let string = CursedString::from_str("inf");
        let result = cursed_string_to_float(string);
        assert!(result.success);
        assert!(result.value.is_infinite());
        
        // Test NaN
        let string = CursedString::from_str("nan");
        let result = cursed_string_to_float(string);
        assert!(result.success);
        assert!(result.value.is_nan());
        
        // Test invalid input
        let string = CursedString::from_str("not_a_float");
        let result = cursed_string_to_float(string);
        assert!(!result.success);
    }
    
    #[test]
    fn test_int_to_string_conversion() {
        let result = cursed_int_to_string(123);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "123");
        
        let result = cursed_int_to_string(-456);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "-456");
    }
    
    #[test]
    fn test_float_to_string_conversion() {
        let result = cursed_float_to_string(123.456);
        let string_repr = result.as_str().unwrap();
        assert!(string_repr.contains("123.456"));
        
        let result = cursed_float_to_string(f64::INFINITY);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "Infinity");
        
        let result = cursed_float_to_string(f64::NAN);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "NaN");
    }
    
    #[test]
    fn test_bool_conversions() {
        // Bool to string
        let result = cursed_bool_to_string(true);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "true");
        
        let result = cursed_bool_to_string(false);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "false");
        
        // String to bool - various true formats
        for true_str in &["true", "True", "TRUE", "t", "T", "yes", "YES", "y", "Y", "1", "on", "ON"] {
            let string = CursedString::from_str(true_str);
            let result = cursed_string_to_bool(string);
            assert!(result.success, "Failed to parse '{}' as true", true_str);
            assert!(result.value, "'{}' should parse as true", true_str);
        }
        
        // String to bool - various false formats
        for false_str in &["false", "False", "FALSE", "f", "F", "no", "NO", "n", "N", "0", "off", "OFF"] {
            let string = CursedString::from_str(false_str);
            let result = cursed_string_to_bool(string);
            assert!(result.success, "Failed to parse '{}' as false", false_str);
            assert!(!result.value, "'{}' should parse as false", false_str);
        }
        
        // Invalid boolean
        let string = CursedString::from_str("maybe");
        let result = cursed_string_to_bool(string);
        assert!(!result.success);
    }
    
    #[test]
    fn test_utf8_validation() {
        // Valid UTF-8
        let string = CursedString::from_str("Hello, 世界!");
        assert!(cursed_string_is_valid_utf8(string));
        
        // Get UTF-8 character count
        let char_count = cursed_string_utf8_length(string);
        assert_eq!(char_count, 9); // "Hello, 世界!" has 9 Unicode characters
        
        // Test with ASCII only
        let ascii_string = CursedString::from_str("Hello");
        assert!(cursed_string_is_valid_utf8(ascii_string));
        let char_count = cursed_string_utf8_length(ascii_string);
        assert_eq!(char_count, 5);
    }
    
    #[test]
    fn test_base_formatting() {
        // Binary
        let result = cursed_int_to_string_base(10, 2);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "0b1010");
        
        // Octal
        let result = cursed_int_to_string_base(64, 8);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "0o100");
        
        // Hexadecimal
        let result = cursed_int_to_string_base(255, 16);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "0xff");
    }
    
    #[test]
    fn test_precision_formatting() {
        let result = cursed_float_to_string_precision(3.141592653589793, 2);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "3.14");
        
        let result = cursed_float_to_string_precision(3.141592653589793, 5);
        let string_repr = result.as_str().unwrap();
        assert_eq!(string_repr, "3.14159");
    }
}
