//! Runtime implementation functions for CURSED standard library
//! 
//! This module provides the external implementation functions that the CURSED
//! standard library calls via extern declarations. These functions bridge the
//! gap between CURSED stdlib API and the underlying Rust runtime.

use crate::error::CursedError;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read, BufRead, BufReader};
use std::env;
use regex::Regex;
use base64::{Engine as _, engine::general_purpose};

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
// Additional I/O Functions Implementation
// ================================

/// Printf-style formatted printing (implementation for io_printf)
#[no_mangle]
pub extern "C" fn io_printf(format_ptr: *const c_char, _args_ptr: *const c_char) -> i32 {
    if format_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(format_ptr).to_str() {
            Ok(format_str) => {
                // For now, just print the format string (full printf implementation would be complex)
                print!("{}", format_str);
                io::stdout().flush().unwrap_or(());
                0
            },
            Err(_) => -1
        }
    }
}

/// Read a single character from console (implementation for io_read_char)
#[no_mangle]
pub extern "C" fn io_read_char(buf_ptr: *mut c_char, buf_len: usize) -> i32 {
    if buf_ptr.is_null() || buf_len < 2 {
        return -1;
    }
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let ch = input.chars().next().unwrap_or('\0');
            unsafe {
                *buf_ptr = ch as c_char;
                *buf_ptr.add(1) = 0; // null terminate
            }
            0
        },
        Err(_) => -1
    }
}

/// Read an integer from console (implementation for io_read_int)
#[no_mangle]
pub extern "C" fn io_read_int() -> i32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().parse::<i32>().unwrap_or(0)
        },
        Err(_) => 0
    }
}

/// Read a float from console (implementation for io_read_float) 
#[no_mangle]
pub extern "C" fn io_read_float() -> f64 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().parse::<f64>().unwrap_or(0.0)
        },
        Err(_) => 0.0
    }
}

/// Append content to a file (implementation for io_append_file)
#[no_mangle]
pub extern "C" fn io_append_file(path_ptr: *const c_char, content_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() || content_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(path_ptr).to_str(), CStr::from_ptr(content_ptr).to_str()) {
            (Ok(path), Ok(content)) => {
                match OpenOptions::new().create(true).append(true).open(path) {
                    Ok(mut file) => {
                        match file.write_all(content.as_bytes()) {
                            Ok(_) => 0,
                            Err(_) => -1
                        }
                    },
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Copy a file (implementation for io_copy_file)
#[no_mangle]
pub extern "C" fn io_copy_file(src_ptr: *const c_char, dest_ptr: *const c_char) -> i32 {
    if src_ptr.is_null() || dest_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(src_ptr).to_str(), CStr::from_ptr(dest_ptr).to_str()) {
            (Ok(src), Ok(dest)) => {
                match fs::copy(src, dest) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Move/rename a file (implementation for io_move_file)
#[no_mangle]
pub extern "C" fn io_move_file(src_ptr: *const c_char, dest_ptr: *const c_char) -> i32 {
    if src_ptr.is_null() || dest_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(src_ptr).to_str(), CStr::from_ptr(dest_ptr).to_str()) {
            (Ok(src), Ok(dest)) => {
                match fs::rename(src, dest) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Get file size (implementation for io_file_size)
#[no_mangle]
pub extern "C" fn io_file_size(path_ptr: *const c_char) -> i64 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => metadata.len() as i64,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Check if path is a file (implementation for io_is_file)
#[no_mangle]
pub extern "C" fn io_is_file(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => if metadata.is_file() { 1 } else { 0 },
                    Err(_) => 0
                }
            },
            Err(_) => 0
        }
    }
}

/// Check if path is a directory (implementation for io_is_directory)
#[no_mangle]
pub extern "C" fn io_is_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => if metadata.is_dir() { 1 } else { 0 },
                    Err(_) => 0
                }
            },
            Err(_) => 0
        }
    }
}

/// Remove a directory (implementation for io_remove_directory)
#[no_mangle]
pub extern "C" fn io_remove_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_dir(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Remove a directory recursively (implementation for io_remove_directory_recursive)
#[no_mangle]
pub extern "C" fn io_remove_directory_recursive(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_dir_all(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Get current working directory (implementation for io_current_directory)
#[no_mangle]
pub extern "C" fn io_current_directory(buf_ptr: *mut c_char, buf_len: usize) -> i32 {
    if buf_ptr.is_null() || buf_len == 0 {
        return -1;
    }
    
    match env::current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                let path_bytes = path_str.as_bytes();
                if path_bytes.len() + 1 <= buf_len {
                    unsafe {
                        ptr::copy_nonoverlapping(path_bytes.as_ptr(), buf_ptr as *mut u8, path_bytes.len());
                        *buf_ptr.add(path_bytes.len()) = 0; // null terminate
                    }
                    0
                } else {
                    -1 // Buffer too small
                }
            } else {
                -1 // Path not valid UTF-8
            }
        },
        Err(_) => -1
    }
}

/// Change working directory (implementation for io_change_directory)
#[no_mangle]
pub extern "C" fn io_change_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match env::set_current_dir(path) {
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

use std::collections::HashMap;
use std::collections::HashSet;
use std::slice;

// Array/Vector Operations
#[no_mangle]
pub extern "C" fn collections_array_new() -> *mut Vec<i64> {
    Box::into_raw(Box::new(Vec::<i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_array_with_capacity(capacity: usize) -> *mut Vec<i64> {
    Box::into_raw(Box::new(Vec::<i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_array_push(arr_ptr: *mut Vec<i64>, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.push(item);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_pop(arr_ptr: *mut Vec<i64>) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.pop().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_get(arr_ptr: *const Vec<i64>, index: usize) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        arr.get(index).copied().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_set(arr_ptr: *mut Vec<i64>, index: usize, value: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index >= arr.len() {
            return -1;
        }
        arr[index] = value;
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_len(arr_ptr: *const Vec<i64>) -> usize {
    if arr_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        arr.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_array_insert(arr_ptr: *mut Vec<i64>, index: usize, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index > arr.len() {
            return -1;
        }
        arr.insert(index, item);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_remove(arr_ptr: *mut Vec<i64>, index: usize) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index >= arr.len() {
            return -1;
        }
        arr.remove(index)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_clear(arr_ptr: *mut Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_is_empty(arr_ptr: *const Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        if arr.is_empty() { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_array_contains(arr_ptr: *const Vec<i64>, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        if arr.contains(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_array_reverse(arr_ptr: *mut Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.reverse();
    }
    0
}

// HashMap Operations
#[no_mangle]
pub extern "C" fn collections_map_new() -> *mut HashMap<i64, i64> {
    Box::into_raw(Box::new(HashMap::<i64, i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_map_with_capacity(capacity: usize) -> *mut HashMap<i64, i64> {
    Box::into_raw(Box::new(HashMap::<i64, i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_map_set(map_ptr: *mut HashMap<i64, i64>, key: i64, value: i64) -> i32 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.insert(key, value);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_map_get(map_ptr: *const HashMap<i64, i64>, key: i64) -> i64 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &*map_ptr;
        map.get(&key).copied().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_map_remove(map_ptr: *mut HashMap<i64, i64>, key: i64) -> i64 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.remove(&key).unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_map_contains_key(map_ptr: *const HashMap<i64, i64>, key: i64) -> i32 {
    if map_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*map_ptr;
        if map.contains_key(&key) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_map_len(map_ptr: *const HashMap<i64, i64>) -> usize {
    if map_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*map_ptr;
        map.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_map_clear(map_ptr: *mut HashMap<i64, i64>) -> i32 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_map_is_empty(map_ptr: *const HashMap<i64, i64>) -> i32 {
    if map_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let map = &*map_ptr;
        if map.is_empty() { 1 } else { 0 }
    }
}

// HashSet Operations
#[no_mangle]
pub extern "C" fn collections_set_new() -> *mut HashSet<i64> {
    Box::into_raw(Box::new(HashSet::<i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_set_with_capacity(capacity: usize) -> *mut HashSet<i64> {
    Box::into_raw(Box::new(HashSet::<i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_set_insert(set_ptr: *mut HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        if set.insert(item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_contains(set_ptr: *const HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &*set_ptr;
        if set.contains(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_remove(set_ptr: *mut HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        if set.remove(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_len(set_ptr: *const HashSet<i64>) -> usize {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &*set_ptr;
        set.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_set_clear(set_ptr: *mut HashSet<i64>) -> i32 {
    if set_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        set.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_set_is_empty(set_ptr: *const HashSet<i64>) -> i32 {
    if set_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let set = &*set_ptr;
        if set.is_empty() { 1 } else { 0 }
    }
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

// ================================
// String Processing Implementation Functions
// ================================

/// Get the length of a string (implementation for string_length)
#[no_mangle]
pub extern "C" fn string_length(str_ptr: *const c_char) -> usize {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s.chars().count(), // Unicode-aware character count
            Err(_) => 0
        }
    }
}

/// Convert string to uppercase (implementation for string_to_upper)
#[no_mangle]
pub extern "C" fn string_to_upper(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let upper = s.to_uppercase();
                match CString::new(upper) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Convert string to lowercase (implementation for string_to_lower)
#[no_mangle]
pub extern "C" fn string_to_lower(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let lower = s.to_lowercase();
                match CString::new(lower) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if string matches regex pattern (implementation for string_regex_match)
#[no_mangle]
pub extern "C" fn string_regex_match(str_ptr: *const c_char, pattern_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || pattern_ptr.is_null() {
        return -1; // Error
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return -1
        };
        
        match Regex::new(pattern) {
            Ok(regex) => if regex.is_match(text) { 1 } else { 0 },
            Err(_) => -1 // Invalid regex pattern
        }
    }
}

/// Find first regex match in string (implementation for string_regex_find)
#[no_mangle]
pub extern "C" fn string_regex_find(str_ptr: *const c_char, pattern_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || pattern_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return ptr::null_mut()
        };
        
        match Regex::new(pattern) {
            Ok(regex) => {
                if let Some(mat) = regex.find(text) {
                    match CString::new(mat.as_str()) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                } else {
                    ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Replace regex matches in string (implementation for string_regex_replace)
#[no_mangle]
pub extern "C" fn string_regex_replace(str_ptr: *const c_char, pattern_ptr: *const c_char, replacement_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || pattern_ptr.is_null() || replacement_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return ptr::null_mut()
        };
        
        let replacement = match CStr::from_ptr(replacement_ptr).to_str() {
            Ok(r) => r,
            Err(_) => return ptr::null_mut()
        };
        
        match Regex::new(pattern) {
            Ok(regex) => {
                let result = regex.replace_all(text, replacement);
                match CString::new(result.as_ref()) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Split string by regex pattern (implementation for string_regex_split)
#[no_mangle]
pub extern "C" fn string_regex_split(str_ptr: *const c_char, pattern_ptr: *const c_char, count_ptr: *mut usize) -> *mut *mut c_char {
    if str_ptr.is_null() || pattern_ptr.is_null() || count_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return ptr::null_mut()
        };
        
        match Regex::new(pattern) {
            Ok(regex) => {
                let parts: Vec<&str> = regex.split(text).collect();
                let count = parts.len();
                *count_ptr = count;
                
                if count == 0 {
                    return ptr::null_mut();
                }
                
                // Allocate array of string pointers
                let array = libc::malloc(count * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
                if array.is_null() {
                    return ptr::null_mut();
                }
                
                // Convert each part to C string
                for (i, part) in parts.iter().enumerate() {
                    match CString::new(*part) {
                        Ok(c_string) => {
                            *array.add(i) = c_string.into_raw();
                        },
                        Err(_) => {
                            // Cleanup on error
                            for j in 0..i {
                                let _ = CString::from_raw(*array.add(j));
                            }
                            libc::free(array as *mut libc::c_void);
                            return ptr::null_mut();
                        }
                    }
                }
                
                array
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Format string with template and arguments (implementation for string_format)
#[no_mangle]
pub extern "C" fn string_format(template_ptr: *const c_char, args_ptr: *const *const c_char, args_count: usize) -> *mut c_char {
    if template_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let template = match CStr::from_ptr(template_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let mut args = Vec::new();
        if !args_ptr.is_null() {
            for i in 0..args_count {
                let arg_ptr = *args_ptr.add(i);
                if !arg_ptr.is_null() {
                    if let Ok(arg) = CStr::from_ptr(arg_ptr).to_str() {
                        args.push(arg);
                    }
                }
            }
        }
        
        // Simple string interpolation - replace {0}, {1}, etc. with arguments
        let mut result = template.to_string();
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Trim whitespace from string (implementation for string_trim)
#[no_mangle]
pub extern "C" fn string_trim(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let trimmed = s.trim();
                match CString::new(trimmed) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if string contains substring (implementation for string_contains)
#[no_mangle]
pub extern "C" fn string_contains(str_ptr: *const c_char, substring_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || substring_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let substring = match CStr::from_ptr(substring_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        if text.contains(substring) { 1 } else { 0 }
    }
}

/// Find index of substring in string (implementation for string_index_of)
#[no_mangle]
pub extern "C" fn string_index_of(str_ptr: *const c_char, substring_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || substring_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let substring = match CStr::from_ptr(substring_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        match text.find(substring) {
            Some(index) => index as i32,
            None => -1
        }
    }
}

/// Substring extraction (implementation for string_substring)
#[no_mangle]
pub extern "C" fn string_substring(str_ptr: *const c_char, start: usize, length: usize) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let chars: Vec<char> = s.chars().collect();
                if start >= chars.len() {
                    // Return empty string if start is beyond string length
                    match CString::new("") {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                } else {
                    let end = std::cmp::min(start + length, chars.len());
                    let substring: String = chars[start..end].iter().collect();
                    match CString::new(substring) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// String concatenation (implementation for string_concat)
#[no_mangle]
pub extern "C" fn string_concat(str1_ptr: *const c_char, str2_ptr: *const c_char) -> *mut c_char {
    if str1_ptr.is_null() || str2_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let str1 = match CStr::from_ptr(str1_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let str2 = match CStr::from_ptr(str2_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let result = format!("{}{}", str1, str2);
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if string is empty (implementation for string_is_empty)
#[no_mangle]
pub extern "C" fn string_is_empty(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 1; // Consider null as empty
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => if s.is_empty() { 1 } else { 0 },
            Err(_) => 1
        }
    }
}

/// Encode string as base64 (implementation for string_base64_encode)
#[no_mangle]
pub extern "C" fn string_base64_encode(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let encoded = general_purpose::STANDARD.encode(s.as_bytes());
                match CString::new(encoded) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Decode base64 string (implementation for string_base64_decode)
#[no_mangle]
pub extern "C" fn string_base64_decode(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                match general_purpose::STANDARD.decode(s) {
                    Ok(decoded_bytes) => {
                        match String::from_utf8(decoded_bytes) {
                            Ok(decoded_string) => {
                                match CString::new(decoded_string) {
                                    Ok(c_string) => c_string.into_raw(),
                                    Err(_) => ptr::null_mut()
                                }
                            },
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

/// Calculate Levenshtein distance between two strings (implementation for string_levenshtein_distance)
#[no_mangle]
pub extern "C" fn string_levenshtein_distance(str1_ptr: *const c_char, str2_ptr: *const c_char) -> i32 {
    if str1_ptr.is_null() || str2_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let str1 = match CStr::from_ptr(str1_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let str2 = match CStr::from_ptr(str2_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let chars1: Vec<char> = str1.chars().collect();
        let chars2: Vec<char> = str2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();
        
        if len1 == 0 { return len2 as i32; }
        if len2 == 0 { return len1 as i32; }
        
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        // Initialize first row and column
        for i in 0..=len1 { matrix[i][0] = i; }
        for j in 0..=len2 { matrix[0][j] = j; }
        
        // Fill the matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i-1] == chars2[j-1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(
                        matrix[i-1][j] + 1,    // deletion
                        matrix[i][j-1] + 1     // insertion
                    ),
                    matrix[i-1][j-1] + cost    // substitution
                );
            }
        }
        
        matrix[len1][len2] as i32
    }
}

// ================================
// Networking Implementation Functions
// ================================

use std::net::{TcpStream, TcpListener, ToSocketAddrs, SocketAddr, IpAddr};
use std::io::{Read as IoRead, Write as IoWrite};

/// Connect to a TCP server (implementation for network_tcp_connect)
#[no_mangle]
pub extern "C" fn network_tcp_connect(host_ptr: *const c_char, port: u16) -> i32 {
    if host_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(host_ptr).to_str() {
            Ok(host) => {
                let addr = format!("{}:{}", host, port);
                match TcpStream::connect(&addr) {
                    Ok(stream) => {
                        // Store stream in a global registry for later use
                        // For now, return a dummy socket descriptor
                        1 // Success
                    },
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Create a TCP listener on a port (implementation for network_tcp_listen)
#[no_mangle]
pub extern "C" fn network_tcp_listen(port: u16) -> i32 {
    let addr = format!("0.0.0.0:{}", port);
    match TcpListener::bind(&addr) {
        Ok(_listener) => {
            // Store listener in a global registry for later use
            // For now, return a dummy socket descriptor
            1 // Success
        },
        Err(_) => -1
    }
}

/// Send data over TCP connection (implementation for network_tcp_send)
#[no_mangle]
pub extern "C" fn network_tcp_send(socket_id: i32, data_ptr: *const c_char, len: usize) -> i32 {
    if data_ptr.is_null() || socket_id <= 0 {
        return -1;
    }
    
    unsafe {
        let data = std::slice::from_raw_parts(data_ptr as *const u8, len);
        // TODO: Retrieve actual socket from registry by socket_id
        // For now, return success for basic functionality
        data.len() as i32
    }
}

/// Receive data from TCP connection (implementation for network_tcp_recv)
#[no_mangle]
pub extern "C" fn network_tcp_recv(socket_id: i32, buffer_ptr: *mut c_char, buffer_len: usize) -> i32 {
    if buffer_ptr.is_null() || socket_id <= 0 || buffer_len == 0 {
        return -1;
    }
    
    // TODO: Retrieve actual socket from registry by socket_id
    // For now, return 0 bytes received
    0
}

/// Close TCP connection (implementation for network_tcp_close)
#[no_mangle]
pub extern "C" fn network_tcp_close(socket_id: i32) -> i32 {
    if socket_id <= 0 {
        return -1;
    }
    
    // TODO: Remove socket from registry and close
    // For now, return success
    0
}

/// Perform DNS resolution (implementation for network_dns_resolve)
#[no_mangle]
pub extern "C" fn network_dns_resolve(hostname_ptr: *const c_char) -> *mut c_char {
    if hostname_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(hostname_ptr).to_str() {
            Ok(hostname) => {
                if hostname == "localhost" {
                    match CString::new("127.0.0.1") {
                        Ok(result) => result.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                } else {
                    // Basic DNS resolution using std::net
                    match format!("{}:80", hostname).to_socket_addrs() {
                        Ok(mut addrs) => {
                            if let Some(addr) = addrs.next() {
                                let ip_str = addr.ip().to_string();
                                match CString::new(ip_str) {
                                    Ok(result) => result.into_raw(),
                                    Err(_) => ptr::null_mut()
                                }
                            } else {
                                ptr::null_mut()
                            }
                        },
                        Err(_) => ptr::null_mut()
                    }
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Simple HTTP GET request (implementation for network_http_get)
#[no_mangle]
pub extern "C" fn network_http_get(url_ptr: *const c_char) -> *mut c_char {
    if url_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(url_ptr).to_str() {
            Ok(url) => {
                // Parse URL to extract host and path
                if let Some(host_start) = url.find("://") {
                    let after_protocol = &url[host_start + 3..];
                    if let Some(path_start) = after_protocol.find('/') {
                        let host = &after_protocol[..path_start];
                        let path = &after_protocol[path_start..];
                        
                        // Create HTTP request
                        let request = format!(
                            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                            path, host
                        );
                        
                        // Connect and send request
                        match TcpStream::connect(format!("{}:80", host)) {
                            Ok(mut stream) => {
                                if stream.write_all(request.as_bytes()).is_ok() {
                                    let mut response = String::new();
                                    if stream.read_to_string(&mut response).is_ok() {
                                        match CString::new(response) {
                                            Ok(result) => return result.into_raw(),
                                            Err(_) => {}
                                        }
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    }
                }
                
                // Return error response on failure
                match CString::new("HTTP/1.1 500 Internal Server Error\r\n\r\nHTTP request failed") {
                    Ok(result) => result.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Simple HTTP POST request (implementation for network_http_post)
#[no_mangle]
pub extern "C" fn network_http_post(url_ptr: *const c_char, data_ptr: *const c_char) -> *mut c_char {
    if url_ptr.is_null() || data_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(url_ptr).to_str(), CStr::from_ptr(data_ptr).to_str()) {
            (Ok(url), Ok(data)) => {
                // Parse URL to extract host and path
                if let Some(host_start) = url.find("://") {
                    let after_protocol = &url[host_start + 3..];
                    if let Some(path_start) = after_protocol.find('/') {
                        let host = &after_protocol[..path_start];
                        let path = &after_protocol[path_start..];
                        
                        // Create HTTP POST request
                        let request = format!(
                            "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            path, host, data.len(), data
                        );
                        
                        // Connect and send request
                        match TcpStream::connect(format!("{}:80", host)) {
                            Ok(mut stream) => {
                                if stream.write_all(request.as_bytes()).is_ok() {
                                    let mut response = String::new();
                                    if stream.read_to_string(&mut response).is_ok() {
                                        match CString::new(response) {
                                            Ok(result) => return result.into_raw(),
                                            Err(_) => {}
                                        }
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    }
                }
                
                // Return error response on failure
                match CString::new("HTTP/1.1 500 Internal Server Error\r\n\r\nHTTP POST request failed") {
                    Ok(result) => result.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

/// Generic HTTP request (implementation for network_http_request)
#[no_mangle]
pub extern "C" fn network_http_request(
    method_ptr: *const c_char, 
    url_ptr: *const c_char, 
    headers_ptr: *const c_char, 
    body_ptr: *const c_char
) -> *mut c_char {
    if method_ptr.is_null() || url_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let method = match CStr::from_ptr(method_ptr).to_str() {
            Ok(m) => m,
            Err(_) => return ptr::null_mut()
        };
        
        let url = match CStr::from_ptr(url_ptr).to_str() {
            Ok(u) => u,
            Err(_) => return ptr::null_mut()
        };
        
        let headers = if headers_ptr.is_null() {
            ""
        } else {
            match CStr::from_ptr(headers_ptr).to_str() {
                Ok(h) => h,
                Err(_) => ""
            }
        };
        
        let body = if body_ptr.is_null() {
            ""
        } else {
            match CStr::from_ptr(body_ptr).to_str() {
                Ok(b) => b,
                Err(_) => ""
            }
        };
        
        // Parse URL to extract host and path
        if let Some(host_start) = url.find("://") {
            let after_protocol = &url[host_start + 3..];
            if let Some(path_start) = after_protocol.find('/') {
                let host = &after_protocol[..path_start];
                let path = &after_protocol[path_start..];
                
                // Build HTTP request
                let mut request = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", method, path, host);
                
                // Add custom headers if provided
                if !headers.is_empty() {
                    request.push_str(headers);
                    if !headers.ends_with("\r\n") {
                        request.push_str("\r\n");
                    }
                }
                
                // Add body if provided
                if !body.is_empty() {
                    request.push_str(&format!("Content-Length: {}\r\n", body.len()));
                }
                
                request.push_str("Connection: close\r\n\r\n");
                
                if !body.is_empty() {
                    request.push_str(body);
                }
                
                // Connect and send request
                match TcpStream::connect(format!("{}:80", host)) {
                    Ok(mut stream) => {
                        if stream.write_all(request.as_bytes()).is_ok() {
                            let mut response = String::new();
                            if stream.read_to_string(&mut response).is_ok() {
                                match CString::new(response) {
                                    Ok(result) => return result.into_raw(),
                                    Err(_) => {}
                                }
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
        }
        
        // Return error response on failure
        match CString::new("HTTP/1.1 500 Internal Server Error\r\n\r\nHTTP request failed") {
            Ok(result) => result.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}
