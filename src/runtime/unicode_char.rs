//! Unicode character operations runtime implementation
//! Provides external functions for LLVM-generated code to call

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use tracing::{instrument, debug};

/// Check if a Unicode character is uppercase
/// Called from LLVM-generated code
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_is_uppercase(code_point: c_int) -> bool {
    if let Some(ch) = std::char::from_u32(code_point as u32) {
        let result = ch.is_uppercase();
        debug!(code_point = code_point, character = %ch, result = result, "Unicode is_uppercase check");
        result
    } else {
        debug!(invalid_code_point = code_point, "Invalid Unicode code point for is_uppercase");
        false
    }
}

/// Check if a Unicode character is lowercase
/// Called from LLVM-generated code
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_is_lowercase(code_point: c_int) -> bool {
    if let Some(ch) = std::char::from_u32(code_point as u32) {
        let result = ch.is_lowercase();
        debug!(code_point = code_point, character = %ch, result = result, "Unicode is_lowercase check");
        result
    } else {
        debug!(invalid_code_point = code_point, "Invalid Unicode code point for is_lowercase");
        false
    }
}

/// Check if a Unicode character is alphabetic
/// Called from LLVM-generated code
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_is_alphabetic(code_point: c_int) -> bool {
    if let Some(ch) = std::char::from_u32(code_point as u32) {
        let result = ch.is_alphabetic();
        debug!(code_point = code_point, character = %ch, result = result, "Unicode is_alphabetic check");
        result
    } else {
        debug!(invalid_code_point = code_point, "Invalid Unicode code point for is_alphabetic");
        false
    }
}

/// Check if a Unicode character is numeric
/// Called from LLVM-generated code
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_is_numeric(code_point: c_int) -> bool {
    if let Some(ch) = std::char::from_u32(code_point as u32) {
        let result = ch.is_numeric();
        debug!(code_point = code_point, character = %ch, result = result, "Unicode is_numeric check");
        result
    } else {
        debug!(invalid_code_point = code_point, "Invalid Unicode code point for is_numeric");
        false
    }
}

/// Check if a Unicode character is whitespace
/// Called from LLVM-generated code
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_is_whitespace(code_point: c_int) -> bool {
    if let Some(ch) = std::char::from_u32(code_point as u32) {
        let result = ch.is_whitespace();
        debug!(code_point = code_point, character = %ch, result = result, "Unicode is_whitespace check");
        result
    } else {
        debug!(invalid_code_point = code_point, "Invalid Unicode code point for is_whitespace");
        false
    }
}

/// Convert a Unicode character to uppercase
/// Called from LLVM-generated code
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_to_uppercase(code_point: c_int) -> c_int {
    if let Some(ch) = std::char::from_u32(code_point as u32) {
        let uppercase_ch = ch.to_uppercase().next().unwrap_or(ch);
        let result = uppercase_ch as c_int;
        debug!(
            code_point = code_point, 
            original_char = %ch, 
            uppercase_char = %uppercase_ch, 
            result = result, 
            "Unicode to_uppercase conversion"
        );
        result
    } else {
        debug!(invalid_code_point = code_point, "Invalid Unicode code point for to_uppercase");
        code_point // Return original if invalid
    }
}

/// Convert a Unicode character to lowercase
/// Called from LLVM-generated code
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_to_lowercase(code_point: c_int) -> c_int {
    if let Some(ch) = std::char::from_u32(code_point as u32) {
        let lowercase_ch = ch.to_lowercase().next().unwrap_or(ch);
        let result = lowercase_ch as c_int;
        debug!(
            code_point = code_point, 
            original_char = %ch, 
            lowercase_char = %lowercase_ch, 
            result = result, 
            "Unicode to_lowercase conversion"
        );
        result
    } else {
        debug!(invalid_code_point = code_point, "Invalid Unicode code point for to_lowercase");
        code_point // Return original if invalid
    }
}

/// Convert a Unicode character to string representation
/// Called from LLVM-generated code
/// Returns a null-terminated C string that must be freed by caller
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_to_string(code_point: c_int) -> *mut c_char {
    if let Some(ch) = std::char::from_u32(code_point as u32) {
        let char_string = ch.to_string();
        match CString::new(char_string.clone()) {
            Ok(c_string) => {
                let result = c_string.into_raw();
                debug!(
                    code_point = code_point, 
                    character = %ch, 
                    string = %char_string,
                    "Unicode to_string conversion"
                );
                result
            }
            Err(_) => {
                debug!(code_point = code_point, "Failed to create C string for character");
                std::ptr::null_mut()
            }
        }
    } else {
        debug!(invalid_code_point = code_point, "Invalid Unicode code point for to_string");
        std::ptr::null_mut()
    }
}

/// Free a string allocated by cursed_unicode_to_string
/// Called from LLVM-generated code for memory cleanup
#[no_mangle]
#[instrument]
pub extern "C" fn cursed_unicode_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
            debug!("Freed Unicode string memory");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_unicode_is_uppercase() {
        assert_eq!(cursed_unicode_is_uppercase('A' as c_int), true);
        assert_eq!(cursed_unicode_is_uppercase('a' as c_int), false);
        assert_eq!(cursed_unicode_is_uppercase('1' as c_int), false);
        assert_eq!(cursed_unicode_is_uppercase('Ä' as c_int), true); // Unicode
    }

    #[test]
    fn test_unicode_is_lowercase() {
        assert_eq!(cursed_unicode_is_lowercase('A' as c_int), false);
        assert_eq!(cursed_unicode_is_lowercase('a' as c_int), true);
        assert_eq!(cursed_unicode_is_lowercase('1' as c_int), false);
        assert_eq!(cursed_unicode_is_lowercase('ä' as c_int), true); // Unicode
    }

    #[test]
    fn test_unicode_is_alphabetic() {
        assert_eq!(cursed_unicode_is_alphabetic('A' as c_int), true);
        assert_eq!(cursed_unicode_is_alphabetic('a' as c_int), true);
        assert_eq!(cursed_unicode_is_alphabetic('1' as c_int), false);
        assert_eq!(cursed_unicode_is_alphabetic('α' as c_int), true); // Greek alpha
    }

    #[test]
    fn test_unicode_is_numeric() {
        assert_eq!(cursed_unicode_is_numeric('A' as c_int), false);
        assert_eq!(cursed_unicode_is_numeric('1' as c_int), true);
        assert_eq!(cursed_unicode_is_numeric('٠' as c_int), true); // Arabic-Indic digit zero
    }

    #[test]
    fn test_unicode_is_whitespace() {
        assert_eq!(cursed_unicode_is_whitespace(' ' as c_int), true);
        assert_eq!(cursed_unicode_is_whitespace('\t' as c_int), true);
        assert_eq!(cursed_unicode_is_whitespace('\n' as c_int), true);
        assert_eq!(cursed_unicode_is_whitespace('A' as c_int), false);
    }

    #[test]
    fn test_unicode_to_uppercase() {
        assert_eq!(cursed_unicode_to_uppercase('a' as c_int), 'A' as c_int);
        assert_eq!(cursed_unicode_to_uppercase('A' as c_int), 'A' as c_int);
        assert_eq!(cursed_unicode_to_uppercase('ä' as c_int), 'Ä' as c_int);
    }

    #[test]
    fn test_unicode_to_lowercase() {
        assert_eq!(cursed_unicode_to_lowercase('A' as c_int), 'a' as c_int);
        assert_eq!(cursed_unicode_to_lowercase('a' as c_int), 'a' as c_int);
        assert_eq!(cursed_unicode_to_lowercase('Ä' as c_int), 'ä' as c_int);
    }

    #[test]
    fn test_unicode_to_string() {
        let ptr = cursed_unicode_to_string('A' as c_int);
        assert!(!ptr.is_null());
        
        unsafe {
            let c_str = CStr::from_ptr(ptr);
            let rust_str = c_str.to_str().unwrap();
            assert_eq!(rust_str, "A");
        }
        
        cursed_unicode_free_string(ptr);
    }

    #[test]
    fn test_invalid_code_points() {
        // Test with invalid Unicode code point
        assert_eq!(cursed_unicode_is_uppercase(-1), false);
        assert_eq!(cursed_unicode_is_lowercase(0x110000), false); // Beyond Unicode range
        assert_eq!(cursed_unicode_to_uppercase(-1), -1);
        assert_eq!(cursed_unicode_to_string(-1), std::ptr::null_mut());
    }
}
