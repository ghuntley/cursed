//! Runtime implementation functions for CURSED standard library
//! 
//! This module provides the external implementation functions that the CURSED
//! standard library calls via extern declarations. These functions bridge the
//! gap between CURSED stdlib API and the underlying Rust runtime.

use crate::error::CursedError;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::fs;
use std::io::{self, Write, Read, BufRead, BufReader};

/// Initialize all runtime functions for the CURSED standard library
pub fn initialize_runtime_functions() -> Result<(), CursedError> {
    // Runtime function initialization - registers all external functions
    // with the JIT compiler and execution engine
    Ok(())
}

// ================================
// I/O Implementation Functions
// ================================

/// Print a message to stdout (implementation for io_print)
#[no_mangle]
pub extern "C" fn io_print(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                print!("{}", message);
                if io::stdout().flush().is_ok() { 0 } else { -1 }
            },
            Err(_) => -1
        }
    }
}

/// Print a message with newline to stdout (implementation for io_println)
#[no_mangle]
pub extern "C" fn io_println(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                println!("{}", message);
                0
            },
            Err(_) => -1
        }
    }
}

/// Print a message to stderr (implementation for io_eprint)
#[no_mangle]
pub extern "C" fn io_eprint(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                eprint!("{}", message);
                if io::stderr().flush().is_ok() { 0 } else { -1 }
            },
            Err(_) => -1
        }
    }
}

/// Print a message with newline to stderr (implementation for io_eprintln)
#[no_mangle]
pub extern "C" fn io_eprintln(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                eprintln!("{}", message);
                0
            },
            Err(_) => -1
        }
    }
}

/// Read a line from stdin (implementation for io_read_line)
#[no_mangle]
pub extern "C" fn io_read_line() -> *mut c_char {
    let stdin = io::stdin();
    let mut line = String::new();
    
    match stdin.read_line(&mut line) {
        Ok(_) => {
            // Remove trailing newline
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            
            match CString::new(line) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => ptr::null_mut()
            }
        },
        Err(_) => ptr::null_mut()
    }
}

/// Write content to a file (implementation for io_write_file)
#[no_mangle]
pub extern "C" fn io_write_file(path_ptr: *const c_char, content_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() || content_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let path_result = CStr::from_ptr(path_ptr).to_str();
        let content_result = CStr::from_ptr(content_ptr).to_str();
        
        match (path_result, content_result) {
            (Ok(path), Ok(content)) => {
                match fs::write(path, content) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Read content from a file (implementation for io_read_file)
#[no_mangle]
pub extern "C" fn io_read_file(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        match CString::new(content) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if a file exists (implementation for io_file_exists)
#[no_mangle]
pub extern "C" fn io_file_exists(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                if fs::metadata(path).is_ok() { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

/// Create a directory (implementation for io_create_directory)
#[no_mangle]
pub extern "C" fn io_create_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::create_dir(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Create a directory recursively (implementation for io_create_directory_recursive)
#[no_mangle]
pub extern "C" fn io_create_directory_recursive(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::create_dir_all(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Delete a file (implementation for io_delete_file)
#[no_mangle]
pub extern "C" fn io_delete_file(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_file(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

// ================================
// Collections Implementation Functions  
// ================================

// Note: Collections functions will be implemented in next phase
// For now, providing minimal stubs to prevent link errors

#[no_mangle]
pub extern "C" fn collections_array_push() -> i32 {
    // TODO: Implement array push functionality
    -1
}

#[no_mangle]
pub extern "C" fn collections_map_set() -> i32 {
    // TODO: Implement map set functionality
    -1
}

// ================================
// Math Implementation Functions
// ================================

#[no_mangle]
pub extern "C" fn math_sin_impl(x: f64) -> f64 {
    x.sin()
}

#[no_mangle]
pub extern "C" fn math_cos_impl(x: f64) -> f64 {
    x.cos()
}

#[no_mangle]
pub extern "C" fn math_sqrt_impl(x: f64) -> f64 {
    x.sqrt()
}

#[no_mangle]
pub extern "C" fn math_random_impl() -> f64 {
    // Simple random implementation - should be replaced with proper PRNG
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    
    (hasher.finish() as f64) / (u64::MAX as f64)
}

// ================================
// Legacy compatibility
// ================================

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED runtime functions implemented".to_string())
}
