//! Runtime support for JIT execution in the CURSED language
//!
//! This module provides runtime support functions specifically for the JIT execution
//! engine, including printing capabilities, type conversion, and other runtime
//! operations necessary for JIT-compiled code.

use std::ffi::CStr;
use std::os::raw::c_char;
use tracing::{debug, info};

/// Print an integer to standard output
///
/// This function is mapped to the "puts" function in JIT execution context.
/// It prints the provided integer value followed by a newline character.
///
/// # Arguments
///
/// * `val` - The integer value to print
///
/// # Returns
///
/// 0 to indicate success
#[no_mangle]
pub extern "C" fn cursed_print_int(val: i32) -> i32 {
    println!("{}", val);
    debug!(value = val, "JIT printed integer value");
    0 // Return 0 for success
}

/// Print a string to standard output
///
/// This function is mapped to the "println" and "spill" functions in JIT execution context.
/// It takes a pointer to a null-terminated C string and prints it followed by a newline.
///
/// # Arguments
///
/// * `str_ptr` - Pointer to a null-terminated C string
///
/// # Returns
///
/// 0 to indicate success
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer. The caller must ensure
/// that the pointer is valid and points to a properly null-terminated string.
#[no_mangle]
pub extern "C" fn cursed_print_string(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        println!();
        debug!("JIT printed empty line");
        return 0;
    }
    
    unsafe {
        let c_str = CStr::from_ptr(str_ptr);
        match c_str.to_str() {
            Ok(s) => {
                println!("{}", s);
                debug!(string = s, "JIT printed string");
            },
            Err(_) => {
                println!("<invalid utf8 string>");
                debug!("JIT encountered invalid UTF-8 string");
            },
        }
    }
    
    0 // Return 0 for success
}

/// Print a floating-point value to standard output
///
/// This function is mapped to the floating-point print functions in JIT execution context.
/// It prints the provided float value followed by a newline character.
///
/// # Arguments
///
/// * `val` - The float value to print
///
/// # Returns
///
/// 0 to indicate success
#[no_mangle]
pub extern "C" fn cursed_print_float(val: f64) -> i32 {
    println!("{}", val);
    debug!(value = val, "JIT printed float value");
    0 // Return 0 for success
}

/// Print a boolean value to standard output
///
/// This function is mapped to the boolean print functions in JIT execution context.
/// It prints either "based" for true or "sus" for false, followed by a newline.
///
/// # Arguments
///
/// * `val` - The boolean value to print (0 for false, non-zero for true)
///
/// # Returns
///
/// 0 to indicate success
#[no_mangle]
pub extern "C" fn cursed_print_bool(val: i32) -> i32 {
    let bool_val = val != 0;
    let text = if bool_val { "based" } else { "sus" };
    println!("{}", text);
    debug!(value = bool_val, text = text, "JIT printed boolean value");
    0 // Return 0 for success
}

/// Print a character to standard output
///
/// This function is mapped to the character print functions in JIT execution context.
/// It prints the provided character followed by a newline.
///
/// # Arguments
///
/// * `val` - The character value to print as an i32 (Unicode code point)
///
/// # Returns
///
/// 0 to indicate success
#[no_mangle]
pub extern "C" fn cursed_print_char(val: i32) -> i32 {
    if let Some(c) = std::char::from_u32(val as u32) {
        println!("{}", c);
        debug!(char = c, code_point = val, "JIT printed character");
    } else {
        println!("<invalid character>");
        debug!(invalid_code_point = val, "JIT encountered invalid character code point");
    }
    
    0 // Return 0 for success
}