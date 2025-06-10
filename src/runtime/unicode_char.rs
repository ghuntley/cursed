/// Unicode character operations for runtime
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

/// Check if a character is uppercase
#[no_mangle]
pub extern "C" fn cursed_unicode_is_uppercase(ch: u32) -> c_int {
    if let Some(c) = char::from_u32(ch) {
        if c.is_uppercase() { 1 } else { 0 }
    } else {
        0
    }
}

/// Check if a character is lowercase
#[no_mangle]
pub extern "C" fn cursed_unicode_is_lowercase(ch: u32) -> c_int {
    if let Some(c) = char::from_u32(ch) {
        if c.is_lowercase() { 1 } else { 0 }
    } else {
        0
    }
}

/// Check if a character is alphabetic
#[no_mangle]
pub extern "C" fn cursed_unicode_is_alphabetic(ch: u32) -> c_int {
    if let Some(c) = char::from_u32(ch) {
        if c.is_alphabetic() { 1 } else { 0 }
    } else {
        0
    }
}

/// Check if a character is numeric
#[no_mangle]
pub extern "C" fn cursed_unicode_is_numeric(ch: u32) -> c_int {
    if let Some(c) = char::from_u32(ch) {
        if c.is_numeric() { 1 } else { 0 }
    } else {
        0
    }
}

/// Check if a character is whitespace
#[no_mangle]
pub extern "C" fn cursed_unicode_is_whitespace(ch: u32) -> c_int {
    if let Some(c) = char::from_u32(ch) {
        if c.is_whitespace() { 1 } else { 0 }
    } else {
        0
    }
}

/// Convert character to uppercase
#[no_mangle]
pub extern "C" fn cursed_unicode_to_uppercase(ch: u32) -> u32 {
    if let Some(c) = char::from_u32(ch) {
        let upper_chars: Vec<char> = c.to_uppercase().collect();
        if upper_chars.len() == 1 {
            upper_chars[0] as u32
        } else {
            ch // Return original if multi-character result
        }
    } else {
        ch
    }
}

/// Convert character to lowercase
#[no_mangle]
pub extern "C" fn cursed_unicode_to_lowercase(ch: u32) -> u32 {
    if let Some(c) = char::from_u32(ch) {
        let lower_chars: Vec<char> = c.to_lowercase().collect();
        if lower_chars.len() == 1 {
            lower_chars[0] as u32
        } else {
            ch // Return original if multi-character result
        }
    } else {
        ch
    }
}

/// Convert character to string - returns C string pointer
#[no_mangle]
pub extern "C" fn cursed_unicode_to_string(ch: u32) -> *mut c_char {
    if let Some(c) = char::from_u32(ch) {
        if let Ok(c_string) = CString::new(c.to_string()) {
            c_string.into_raw()
        } else {
            std::ptr::null_mut()
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Free string allocated by cursed_unicode_to_string
#[no_mangle]
pub extern "C" fn cursed_unicode_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
